mod intro;
mod anim;

use bevy::prelude::*;
use bevy::input::system::exit_on_esc_system;
use crate::anim::GlazedAnimator;
use crate::intro::Intro;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pokemon Glazed DX".to_string(),
            width: 256.0 * 2.0,
            height: 192.0 * 2.0,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(GlazedAnimator)
        .add_state(GameState::Intro)
        .add_plugin(Intro)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        .run()
}

/// Describes which portion of the game we are in!
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum GameState {
    /// Intro + Title Screen
    Intro,
    /// New Game / Continue / Options
    MainMenu,
    /// In the actual world
    Overworld,
    /// In a battle
    Battle
}

fn setup(mut commands: Commands) {
    // Spawns the camera
    let mut camera = OrthographicCameraBundle::new_2d();
    commands
        .spawn()
        .insert_bundle(camera)
        .insert(Transform::from_xyz(0f32, 0f32, 1000f32));
}
