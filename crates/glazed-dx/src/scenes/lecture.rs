use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{Tween, EaseFunction, lens::{TransformPositionLens, SpriteColorLens, TransformRotateZLens}, Animator, Sequence, Delay, EaseMethod, Tracks};
use iyes_loopless::prelude::*;
use rand::Rng as o;

use crate::{App, GameState, Plugin, util::{despawn, Rng}, LEFT_EDGE, TOP_EDGE, RIGHT_EDGE, BOTTOM_EDGE};

#[derive(Component)]
pub struct LectureAsset;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum LectureState {
    Opening
}
#[derive(Component)]
pub struct Wait(Timer);

pub struct Lecture;
impl Plugin for Lecture {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(LectureState::Opening)
            .add_enter_system(GameState::ProfessorLecture, setup)
            .add_exit_system(GameState::ProfessorLecture, despawn::<LectureAsset>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::ProfessorLecture)
                    .with_system(spawn_pokeballs)
                    .with_system(despawn_complete_pokeballs)
                    .into()
            )
        ;
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("lecture/background.png"),
        ..default()
    }).insert(LectureAsset);

    commands.spawn().insert(Wait(Timer::new(Duration::from_millis(500), true)));
}

// Cute background effect with Pokeballs. Makes the scene feel a bit dynamic
fn spawn_pokeballs(mut commands: Commands, assets: Res<AssetServer>, 
    time: Res<Time>, mut query: Query<&mut Wait>, mut rng: Local<Rng>) {
    
    let mut timer = match query.iter_mut().next() {
        Some(t) => t,
        None => return
    };
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        for _ in 0..2 {
            // Every 500 ms, generate two random Pokeballs.
            // Each Pokeball starts just above the top of the screen, and slides down
            // slowly to between 16 and 64 pixels below the edge of the screen.
            // Each pokeball spins slowly as well.
            // Each slide takes five seconds, fading half a second before completion
            let random_x = rng.gen_range(LEFT_EDGE..=RIGHT_EDGE);
            let random_end_y = rng.gen_range(16..=64) as f32;

            let slide = Tween::new(
                EaseFunction::QuadraticIn,
                bevy_tweening::TweeningType::Once,
                Duration::from_secs(5),
                TransformPositionLens {
                    start: Vec3::new(random_x, TOP_EDGE + 8.0, 10.0),
                    end: Vec3::new(random_x, BOTTOM_EDGE + random_end_y, 10.0)
                }
            );

            let rotate = Tween::new(
                EaseMethod::Linear,
                bevy_tweening::TweeningType::Loop,
                Duration::from_secs(4),
                TransformRotateZLens {
                    start: 0.0,
                    end: 360f32.to_radians(),
                }
            );

            let fade_out = Delay::new(Duration::from_millis(4500))
            .then(Tween::new (
                EaseFunction::QuadraticOut,
                bevy_tweening::TweeningType::Once,
                Duration::from_millis(500),
                SpriteColorLens {
                    start: Color::rgba(1.0, 1.0, 1.0, 0.2),
                    end: Color::rgba(1.0, 1.0, 1.0, 0.0),
                }
            ));

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                    ..default()
                },
                texture: assets.load("lecture/pokeball.png"),
                transform: Transform::from_xyz(random_x, TOP_EDGE + 8.0, 10.0),
                ..default()
            })
            .insert(LectureAsset)
            .insert(Animator::new(Tracks::new([slide, rotate])))
            .insert(Animator::new(fade_out));
        }
    }
}

// Clean up Pokeballs no longer visible
fn despawn_complete_pokeballs(mut commands: Commands, query: Query<(Entity, &Animator<Sprite>)>) {
    for (entity, a) in query.iter() {
        if a.progress() == 1.0 {
            commands.entity(entity).despawn();
        }
    }
}