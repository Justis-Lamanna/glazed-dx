use std::time::Duration;

use bevy::prelude::*;
use bevy_sequential_actions::*;
use bevy_tweening::{Tween, EaseFunction, lens::{TransformPositionLens, SpriteColorLens, TransformRotateZLens}, Animator, Delay, EaseMethod, Tracks};
use iyes_loopless::prelude::*;
use rand::Rng as o;

use glazed_data::species::Species;

use crate::{App, GameState, Plugin, util::{despawn, Rng, in_transition}, LEFT_EDGE, TOP_EDGE, RIGHT_EDGE, BOTTOM_EDGE, text::TextBoxOptions, GlobalOptions};
use crate::actions::audio::PlayCry;
use crate::actions::delay::WaitAction;
use crate::actions::graphics::{ChangeFrame, ShowSprite, TweenTranslate};
use crate::pkmn::PokemonSprite;
use crate::actions::text::ShowTextAction;
use crate::locale::Fluent;

const INTRO_POKEMON: Species = Species::Buizel;
const GREETINGS: &'static str = "intro-a";
const GREETINGS_2: &'static str = "intro-b";
const GREETINGS_3: &'static str = "intro-c";

#[derive(Component)]
pub struct LectureAsset;

#[derive(Component)]
pub struct Professor;

#[derive(Component)]
pub struct DemoPokemon;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum LectureState {
    Fading,
    Introduction
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
            .add_loopless_state(LectureState::Fading)
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
                    .run_in_state(LectureState::Fading)
                    .run_if_not(in_transition)
                    .with_system(display_welcome_text)
                    .into()
            )
        ;
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut textures: ResMut<Assets<TextureAtlas>>,
         ps: PokemonSprite, options: Res<GlobalOptions>) {
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
        texture: ps.get_front_sprite(options.intro_pokemon.unwrap_or(INTRO_POKEMON)),
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

fn display_welcome_text(mut commands: Commands, opts: Res<GlobalOptions>, mut fluent: Fluent) {
    commands.insert_resource(NextState(LectureState::Introduction));

    let timeline = commands.spawn_bundle(ActionsBundle::default()).id();
    let intro_pokemon = opts.intro_pokemon.unwrap_or(INTRO_POKEMON);
    fluent.buffer_species_name("intro-pokemon", intro_pokemon);

    commands.action(timeline)
        .add(ShowTextAction(TextBoxOptions::new(GREETINGS).with_max_lines(2)))
        .add(WaitAction(Duration::from_millis(200)))
        .add(TweenTranslate::<Professor>::new(
            Vec3::new(-8.0, 16.0, 20.0),
            Vec3::new(-64.0, 16.0, 20.0),
            Duration::from_secs(1)))
        .add(WaitAction(Duration::from_millis(500)))
        .add(ChangeFrame::<Professor>::new(1))
        .add(WaitAction(Duration::from_secs(1)))
        .add(ShowSprite::<DemoPokemon>::new(true))
        .add(PlayCry(intro_pokemon))
        .add(WaitAction(Duration::from_millis(500)))
        .add(ShowTextAction(TextBoxOptions::new(GREETINGS_2).with_max_lines(2)))
        .add(WaitAction(Duration::from_millis(500)))
        .add(ShowSprite::<DemoPokemon>::new(false))
        .add(WaitAction(Duration::from_millis(500)))
        .add(ChangeFrame::<Professor>::new(0))
        .add(TweenTranslate::<Professor>::new(
            Vec3::new(-64.0, 16.0, 20.0),
            Vec3::new(-8.0, 16.0, 20.0),
            Duration::from_secs(1)))
        .add(ShowTextAction(TextBoxOptions::new(GREETINGS_3).with_max_lines(2)))
    ;
}