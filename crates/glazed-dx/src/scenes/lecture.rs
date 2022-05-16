use std::time::Duration;

use bevy::prelude::*;
use bevy_sequential_actions::*;
use bevy_tweening::{Tween, EaseFunction, lens::{TransformPositionLens, SpriteColorLens, TransformRotateZLens}, Animator, Sequence, Delay, EaseMethod, Tracks};
use iyes_loopless::prelude::*;
use rand::Rng as o;

use glazed_data::species::Species;

use crate::{App, GameState, Plugin, util::{despawn, Rng, TransitionState, in_transition}, LEFT_EDGE, TOP_EDGE, RIGHT_EDGE, BOTTOM_EDGE, text::TextBoxOptions, SCREEN_WIDTH};
use crate::actions::delay::WaitAction;
use crate::actions::graphics::ChangeFrame;
use crate::pkmn::{PokemonSprite, SpriteRequest};
use crate::text::{EndOfText, TextBoxSystem};
use crate::actions::text::ShowTextAction;

const GREETINGS: &'static str = "Greetings, and welcome to the world of Pokémon!\nMy name is Professor Willow. Some people happen to call me the Pokémon Professor. I study Pokémon for a living! With my research, we can learn all about these mysterious creatures.";

#[derive(Component)]
pub struct LectureAsset;

#[derive(Component)]
pub struct Professor;

#[derive(Component)]
pub struct DemoPokemon;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum LectureState {
    Opening,
    Introduction,
    ShowPokemon
}
#[derive(Component, Deref, DerefMut)]
pub struct PokeballTimer(Timer);
impl Default for PokeballTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(500), true))
    }
}

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
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::ProfessorLecture)
                    .run_in_state(LectureState::Opening)
                    .run_if_not(in_transition)
                    .with_system(display_welcome_text)
                    .into()
            )
        ;
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut textures: ResMut<Assets<TextureAtlas>>, ps: PokemonSprite) {
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("lecture/background.png"),
        ..default()
    }).insert(LectureAsset);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: textures.add(TextureAtlas::from_grid(
            assets.load("lecture/professor.png"),
            Vec2::new(128.0, 160.0), 2, 1
        )),
        transform: Transform::from_xyz(-8.0, 16.0, 20.0),
        ..default()
    })
        .insert(LectureAsset)
        .insert(Professor);

    commands.spawn_bundle(SpriteBundle {
        texture: ps.get_front_sprite(Species::Bulbasaur),
        transform: Transform::from_xyz(0.0, 0.0, 50.0),
        visibility: Visibility {
            is_visible: false
        },
        ..default()
    })
        .insert(LectureAsset)
        .insert(DemoPokemon);
}

// Cute background effect with Pokeballs. Makes the scene feel a bit dynamic
fn spawn_pokeballs(mut commands: Commands, assets: Res<AssetServer>, 
    time: Res<Time>, mut timer: Local<PokeballTimer>, mut rng: Local<Rng>) {
    
    timer.tick(time.delta());
    if timer.just_finished() {
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

fn display_welcome_text(mut commands: Commands) {
    commands.insert_resource(NextState(LectureState::Introduction));
    // text.show(TextBoxOptions::new(GREETINGS.into()).with_max_lines(2));
    let timeline = commands.spawn_bundle(ActionsBundle::default()).id();
    commands.action(timeline)
        .add(WaitAction(Duration::from_secs(2)))
        .add(ShowTextAction(GREETINGS.into()))
        .add(WaitAction(Duration::from_millis(200)))
        .add(ChangeFrame::<Professor>::new(1))
    ;
}