mod intro;
mod anim;
mod state;
mod util;

use std::time::Duration;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;
use bevy::input::system::exit_on_esc_system;
use bevy_tweening::{Animator, TweeningPlugin};
use crate::anim::GlazedAnimator;
use crate::intro::Title;
use crate::util::Fade;

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
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)

        // Fading Systems
        .add_plugin(Fade)

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
    let mut camera = OrthographicCameraBundle::new_2d();
    commands
        .spawn()
        .insert_bundle(camera)
        .insert(Transform::from_xyz(0f32, 0f32, 1000f32));
}

#[derive(Component)]
struct LoadTimer(Timer);

fn init_load() -> Progress {
    true.into()
}