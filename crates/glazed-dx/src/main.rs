mod intro;
mod anim;
mod state;
mod util;
mod audio;

use std::time::Duration;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;
use bevy::input::system::exit_on_esc_system;
use bevy_kira_audio::AudioPlugin;
use bevy_tweening::{Animator, TweeningPlugin};
use crate::anim::GlazedAnimator;
use crate::audio::Cry;
use crate::intro::Title;
use crate::state::GlobalOptions;
use crate::util::TransitionPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pokemon Glazed DX".to_string(),
            width: 256.0,
            height: 192.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(GlazedAnimator)
        .add_plugin(TweeningPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(setup)
        .add_startup_system(GlobalOptions::load)
        .add_system(exit_on_esc_system)

        // Random Plugins
        .add_plugin(TransitionPlugin)
        .add_plugin(Cry)

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

fn setup(mut commands: Commands) {
    // Spawns the camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0f32, 0f32, 1000f32));
}

fn init_load() -> Progress {
    true.into()
}