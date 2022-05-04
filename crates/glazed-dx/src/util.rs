use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::Animator;
use iyes_loopless::prelude::*;
use iyes_loopless::state::CurrentState;

pub fn despawn<T: Component>(mut commands: Commands, marked: Query<Entity, With<T>>) {
    marked.for_each(|e| commands.entity(e).despawn_recursive())
}

#[derive(Component)]
pub struct FadeMarker;

#[derive(Copy, Clone, Debug)]
pub struct TriggerFade {
    start: Color,
    end: Color,
    duration: Duration
}
impl TriggerFade {
    pub fn fade_in(color: Color, duration: Duration) -> TriggerFade {
        TriggerFade::fade(color, 0.0, 1.0, duration)
    }

    pub fn fade_out(color: Color, duration: Duration) -> TriggerFade {
        TriggerFade::fade(color, 1.0, 0.0, duration)
    }

    pub fn fade(color: Color, a_start: f32, a_end: f32, duration: Duration) -> TriggerFade {
        let mut start = color.clone();
        start.set_a(a_start);
        let mut end = color.clone();
        end.set_a(a_end);
        TriggerFade { start, end, duration }
    }
}

/// Special state that determines if we are in transition or not
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum TransitionState {
    In,
    Between,
    Out,
    None
}

/// The type of fade (either in or out)
#[derive(Copy, Clone, Debug)]
pub enum FadeType {
    None,
    Immediate(Color),
    Gentle(TriggerFade),
    Custom
}

/// Describes the total transition
#[derive(Copy, Clone, Debug)]
pub struct Transition {
    pub enter: FadeType,
    pub exit: FadeType
}
impl Transition {
    pub fn gentle(color: Color, duration: Duration) -> Transition {
        let enter = FadeType::Gentle(TriggerFade::fade_in(color, duration));
        let exit = FadeType::Gentle(TriggerFade::fade_out(color, duration));
        Transition { enter, exit }
    }

    pub fn fade_to_black() -> Transition {
        Transition::gentle(Color::BLACK, Duration::from_millis(600))
    }
}

pub struct TransitionPlugin;
impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(TransitionState::None)
            .add_event::<Transition>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TransitionState::None)
                    .with_system(monitor_transition_request)
                    .into()
            )
            .add_enter_system(TransitionState::In, init_fade_in)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TransitionState::In)
                    .run_if(should_monitor_fade_in)
                    .with_system(monitor_fade_in)
                    .into()
            )
            .add_enter_system(TransitionState::Out, init_fade_out)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(TransitionState::Out)
                    .run_if(should_monitor_fade_out)
                    .with_system(monitor_fade_out)
                    .into()
            )
        ;
    }
}

fn monitor_transition_request(mut cmds: Commands, mut reader: EventReader<Transition>) {
    if let Some(t) = reader.iter().last() {
        cmds.insert_resource(t.clone());
        cmds.insert_resource(NextState(TransitionState::In))
    }
}

fn init_fade_in(mut cmds: Commands, res: Option<Res<Transition>>) {
    match res.unwrap().enter {
        // Immediately transition to the next scene
        FadeType::None => cmds.insert_resource(NextState(TransitionState::Between)),
        // Do nothing. A third-party system will initialize
        FadeType::Custom => {},
        // Immediate plop the screen to another color
        FadeType::Immediate(color) => {
            cmds.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(256.0, 192.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 999.0),
                ..Default::default()
            }).insert(FadeMarker);
            cmds.insert_resource(NextState(TransitionState::Between))
        },
        // Gently fade to a certain color
        FadeType::Gentle(TriggerFade{start, end, duration}) => {
            use bevy_tweening::*;
            use bevy_tweening::lens::SpriteColorLens;
            let fade = Tween::new(
                EaseFunction::QuadraticInOut,
                bevy_tweening::TweeningType::Once,
                duration,
                SpriteColorLens { start, end });

            cmds.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: start,
                    custom_size: Some(Vec2::new(256.0, 192.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 999.0),
                ..Default::default()
            }).insert(FadeMarker).insert(Animator::new(fade));
        }
    }
}

fn should_monitor_fade_in(res: Option<Res<Transition>>) -> bool {
    if res.is_none() {
        return false;
    }
    if let FadeType::Gentle(_) = res.unwrap().enter {
        true
    } else {
        false
    }
}

fn monitor_fade_in(mut cmds: Commands, query: Query<&Animator<Sprite>, With<FadeMarker>>) {
    if let Some(a) = query.iter().last() {
        if a.progress() >= 1.0 {
            cmds.insert_resource(NextState(TransitionState::Between))
        }
    }
}

fn init_fade_out(mut cmds: Commands, res: Option<Res<Transition>>, mut existing: Query<(Entity, &mut Sprite), With<FadeMarker>>) {
    let (entity, mut sprite) = existing.single_mut();
    match res.unwrap().exit {
        // Immediately transition to the next scene
        FadeType::None => cmds.insert_resource(NextState(TransitionState::None)),
        // Do nothing. A third-party system will initialize
        FadeType::Custom => {},
        // Immediate plop the screen to another color
        FadeType::Immediate(color) => {
            sprite.color = color;
            cmds.insert_resource(NextState(TransitionState::None));
            if color.a() == 0.0 {
                cmds.entity(entity).despawn_recursive();
            }
        },
        // Gently fade to a certain color
        FadeType::Gentle(TriggerFade{start, end, duration}) => {
            use bevy_tweening::*;
            use bevy_tweening::lens::SpriteColorLens;
            let fade = Tween::new(
                EaseFunction::QuadraticInOut,
                bevy_tweening::TweeningType::Once,
                duration,
                SpriteColorLens { start, end });

            sprite.color = start;
            cmds.entity(entity).insert(Animator::new(fade));
        }
    }
}

fn should_monitor_fade_out(res: Option<Res<Transition>>) -> bool {
    if res.is_none() {
        return false;
    }
    if let FadeType::Gentle(_) = res.unwrap().exit {
        true
    } else {
        false
    }
}

fn monitor_fade_out(mut cmds: Commands, query: Query<(Entity, &Animator<Sprite>), With<FadeMarker>>) {
    if let Some((entity, a)) = query.iter().last() {
        if a.progress() >= 1.0 {
            cmds.insert_resource(NextState(TransitionState::None));
            cmds.entity(entity).despawn_recursive();
        }
    }
}

pub fn advance_through_continue(mut cmds: Commands) {
    cmds.insert_resource(NextState(TransitionState::Out))
}

pub fn in_transition(res: Res<CurrentState<TransitionState>>) -> bool {
    if let TransitionState::None = res.0 {
        false
    } else {
        true
    }
}