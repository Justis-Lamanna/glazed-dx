use bevy::prelude::*;
use bevy_tweening::{Animator, component_animator_system, Lens};
use iyes_loopless::prelude::*;
use iyes_loopless::state::CurrentState;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use std::collections::hash_map::DefaultHasher;
use std::time::Duration;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use anyhow::Error;
use bevy::asset::{Asset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use crate::UI;

/// A system which despawns everything marked with a specific component.
/// To use, add the following as a system: `despawn::<{marker component}>`.
pub fn despawn<T: Component>(mut commands: Commands, marked: Query<Entity, With<T>>) {
    marked.for_each(|e| commands.entity(e).despawn_recursive())
}

/// An asset loader which loads yaml with different extensions.
pub struct YamlLoader<RAW, OUT: From<Vec<RAW>>> {
    extension: [&'static str; 1],
    _m1: PhantomData<RAW>,
    _m2: PhantomData<OUT>
}
impl<RAW, OUT: From<Vec<RAW>>> YamlLoader<RAW, OUT> {
    pub fn new(extension: &'static str) -> Self {
        Self {
            extension: [extension],
            _m1: PhantomData,
            _m2: PhantomData
        }
    }
}
impl<RAW, OUT> AssetLoader for YamlLoader<RAW, OUT> where
    RAW: DeserializeOwned + Send + Sync + 'static,
    OUT: Asset + From<Vec<RAW>>
{
    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<'a, anyhow::Result<(), Error>> {
        Box::pin(async move {
            let content: Vec<RAW> = serde_yaml::from_slice(bytes)?;
            let content = OUT::from(content);
            load_context.set_default_asset(LoadedAsset::new(content));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &self.extension
    }
}

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
    pub fn asymmetrical_fade_to_black(ind: Duration, outd: Duration) -> Transition {
        let enter = FadeType::Gentle(TriggerFade::fade_in(Color::BLACK, ind));
        let exit = FadeType::Gentle(TriggerFade::fade_out(Color::BLACK, outd));
        Transition { enter, exit }
    }
}

pub struct TransitionPlugin;
impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(TransitionState::None)
            .add_event::<Transition>()
            .add_system(component_animator_system::<UiColor>)
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
        info!("Got transition request {:?}", t);
        cmds.insert_resource(t.clone());
        cmds.insert_resource(NextState(TransitionState::In))
    }
}

fn init_fade_in(mut cmds: Commands, res: Option<Res<Transition>>, mut ui: Query<(Entity, &mut UiColor), With<UI>>) {
    let (entity, mut ui) = ui.single_mut();
    match res.unwrap().enter {
        // Immediately transition to the next scene
        FadeType::None => cmds.insert_resource(NextState(TransitionState::Between)),
        // Do nothing. A third-party system will initialize
        FadeType::Custom => {},
        // Immediate plop the screen to another color
        FadeType::Immediate(color) => {
            info!("Fade in to solid color: {:?}", color);
            *ui = UiColor(color);
            cmds.insert_resource(NextState(TransitionState::Between))
        },
        // Gently fade to a certain color
        FadeType::Gentle(TriggerFade{start, end, duration}) => {
            info!("Gentle fade in: {:?} => {:?} ({:?})", start, end, duration.as_millis());
            use bevy_tweening::*;
            use bevy_tweening::lens::SpriteColorLens;
            let fade = Tween::new(
                EaseFunction::QuadraticInOut,
                bevy_tweening::TweeningType::Once,
                duration,
                UiColorLens { start, end });

            cmds.entity(entity).insert(Animator::new(fade));
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

fn monitor_fade_in(mut cmds: Commands, query: Query<&Animator<UiColor>, With<UI>>) {
    if let Some(a) = query.iter().last() {
        if a.progress() >= 1.0 {
            cmds.insert_resource(NextState(TransitionState::Between))
        }
    }
}

fn init_fade_out(mut cmds: Commands, res: Option<Res<Transition>>, mut existing: Query<(Entity, &mut UiColor), With<UI>>) {
    let (entity, mut ui) = existing.single_mut();
    match res.unwrap().exit {
        // Immediately transition to the next scene
        FadeType::None => cmds.insert_resource(NextState(TransitionState::None)),
        // Do nothing. A third-party system will initialize
        FadeType::Custom => {},
        // Immediate plop the screen to another color
        FadeType::Immediate(color) => {
            info!("Fade out to solid color: {:?}", color);
            *ui = UiColor(color);
            cmds.insert_resource(NextState(TransitionState::None));
        },
        // Gently fade to a certain color
        FadeType::Gentle(TriggerFade{start, end, duration}) => {
            info!("Gentle fade out: {:?} => {:?} ({:?})", start, end, duration.as_millis());
            use bevy_tweening::*;
            use bevy_tweening::lens::SpriteColorLens;
            let fade = Tween::new(
                EaseFunction::QuadraticInOut,
                bevy_tweening::TweeningType::Once,
                duration,
                UiColorLens { start, end });

            *ui = UiColor(start);
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

fn monitor_fade_out(mut cmds: Commands, query: Query<(Entity, &Animator<UiColor>), With<UI>>) {
    if let Some((entity, a)) = query.iter().last() {
        if a.progress() >= 1.0 {
            cmds.insert_resource(NextState(TransitionState::None));
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UiColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}
impl Lens<UiColor> for UiColorLens {
    fn lerp(&mut self, target: &mut UiColor, ratio: f32) {
        debug!("Lerp: {}", ratio);
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        *target = UiColor(value.into());
    }
}

//Randomness. Inspired by bevy_random, which is no longer maintained.
#[derive(Debug, Clone, Default)]
pub struct RootRng {
    seed: Option<u64>
}
impl RootRng {
    /// Seed the RNG with anything that can be hashed.
    pub fn seed<T: Hash>(seed: T) -> RootRng {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        RootRng {
            seed: Some(hasher.finish())
        }
    }
}

/// A resource that generates random numbers.
/// If there is no global seed, the rng is seeded randomly. 
/// If there is a global seed, all instances of Rng are seeded with it.
/// This makes the game exceedingly predictable, but some people want it that way.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Rng(Xoshiro256StarStar);
impl FromWorld for Rng {
    fn from_world(world: &mut World) -> Self {
        let inner = world.get_resource::<RootRng>()
            .and_then(|rng| rng.seed)
            .map_or_else(
                || Xoshiro256StarStar::from_entropy(), 
                |s| Xoshiro256StarStar::seed_from_u64(s));
        Rng(inner)
    }
}