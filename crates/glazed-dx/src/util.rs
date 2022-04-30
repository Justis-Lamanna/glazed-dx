use std::time::Duration;
use bevy::prelude::*;
use bevy_tweening::Animator;
use iyes_loopless::prelude::NextState;
use iyes_progress::Progress;
use crate::GameState;

pub fn despawn<T: Component>(mut commands: Commands, marked: Query<Entity, With<T>>) {
    marked.for_each(|e| commands.entity(e).despawn_recursive())
}

pub struct Fade;
impl Plugin for Fade {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TriggerFade>()
            .add_system(listen_for_fade_trigger);
    }
}

#[derive(Component)]
pub struct FadeMarker;

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

    pub fn reverse(&self) -> TriggerFade {
        TriggerFade {
            start: self.end,
            end: self.start,
            duration: self.duration
        }
    }
}

fn listen_for_fade_trigger(mut commands: Commands, mut listener: EventReader<TriggerFade>) {
    if let Some(TriggerFade { start, end, duration }) = listener.iter().last() {
        use bevy_tweening::*;
        use bevy_tweening::lens::SpriteColorLens;
        let fade = Tween::new(
            EaseFunction::QuadraticInOut,
            bevy_tweening::TweeningType::Once,
            *duration,
            SpriteColorLens {
                start: *start, end: *end
            });

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(0,0,0,0),
                custom_size: Some(Vec2::new(256.0, 192.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 999.0),
            ..Default::default()
        }).insert(FadeMarker).insert(Animator::new(fade));
    }
}

pub fn get_fade_progress(mut query: Query<&Animator<Sprite>, With<FadeMarker>>) -> Progress {
    is_fade_complete(query).into()
}

pub fn fade_to_black(mut writer: EventWriter<TriggerFade>) {
    writer.send(TriggerFade::fade_in(Color::BLACK, Duration::from_millis(500)))
}