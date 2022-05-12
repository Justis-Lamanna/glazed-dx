mod anim;
mod state;
mod util;
mod audio;
mod text;
mod controls;
mod scenes;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy::input::system::exit_on_esc_system;
use bevy_kira_audio::AudioPlugin;
use bevy_tweening::TweeningPlugin;
use util::RngPlugin;
use crate::anim::GlazedAnimator;
use crate::audio::Cry;
use crate::controls::Actions;
use crate::scenes::intro::Title;
use crate::scenes::lecture::Lecture;
use crate::text::TextPlugin;
use crate::state::GlobalOptions;
use crate::util::TransitionPlugin;

pub const SCREEN_WIDTH: f32 = 400.0;
pub const SCREEN_HEIGHT: f32 = 225.0;
pub const LEFT_EDGE: f32 = -(SCREEN_WIDTH / 2.0);
pub const RIGHT_EDGE: f32 = SCREEN_WIDTH / 2.0;
pub const TOP_EDGE: f32 = SCREEN_HEIGHT / 2.0;
pub const BOTTOM_EDGE: f32 = -(SCREEN_HEIGHT / 2.0);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pokemon Glazed DX".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(GlazedAnimator)
        .add_plugin(TweeningPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(InputManagerPlugin::<Actions>::default())
        .add_plugin(RngPlugin)
        .add_startup_system(setup)
        .add_startup_system(GlobalOptions::load)
        .add_system(exit_on_esc_system)

        // Random Plugins
        .add_plugin(TransitionPlugin)
        .add_plugin(Cry)
        .add_plugin(TextPlugin)

        // Splash Screen
        .add_loopless_state(GameState::Splash)
        .add_plugin(ProgressPlugin::new(GameState::Splash).continue_to(GameState::Title))
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Splash)
                .with_system(init_load.track_progress())
                .into()
        )

        // Other states go here
        .add_plugin(Title)
        .add_plugin(Lecture)
        .run();
}

/// Describes which portion of the game we are in!
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum GameState {
    /// Initial
    Splash,
    /// Title Screen
    Title,
    /// Welcome to the world of Pokemon
    ProfessorLecture
}

#[derive(Component)]
pub struct UI;

#[derive(Component)]
pub struct PlayerData;

fn setup(mut commands: Commands) {
    // Spawns the camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0f32, 0f32, 1000f32));

    commands
        .spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    })
    .insert(UI);
}

fn init_load() -> Progress {
    true.into()
}