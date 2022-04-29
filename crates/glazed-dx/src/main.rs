mod intro;
mod anim;
mod state;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy::input::system::exit_on_esc_system;
use bevy_tweening::TweeningPlugin;
use crate::anim::GlazedAnimator;
use crate::intro::{Title};

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
        .add_loopless_state(GameState::Loading)
        .add_startup_system(setup)
        .add_system(exit_on_esc_system)
        
        .add_enter_system(GameState::Loading, init_load)
        .add_system(tick_and_complete_load.run_in_state(GameState::Loading))
        .add_plugin(Title)
        .run()
}

/// Describes which portion of the game we are in!
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum GameState {
    /// Initial
    Loading,
    /// Title Screen
    Title,
    /// Subscreen where the player is asked if they want to start a new game,
    /// or continue an old game
    LoadGame,
    /// The famous Professor Lecture at the beginning of a new game
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

fn init_load(mut commands: Commands) {
    commands.spawn()
        .insert(LoadTimer(Timer::from_seconds(2.0, false)));
}

fn tick_and_complete_load(mut commands: Commands, time: Res<'_, Time>, mut query: Query<&mut LoadTimer>) {
    let timer = &mut query.single_mut().0;
    timer.tick(time.delta());
    if timer.just_finished() {
        commands.insert_resource(NextState(GameState::Title));
    }
}