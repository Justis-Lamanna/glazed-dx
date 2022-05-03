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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawns the camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_xyz(0f32, 0f32, 1000f32));

    commands
        .spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|p| {
            p.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(256.0), Val::Px(48.0)),
                    ..Default::default()
                },
                color: Color::rgba(0.25, 0.25, 0.25, 0.75).into(),
                ..Default::default()
            }).with_children(|p| {
                   p.spawn_bundle(TextBundle {
                       style: Style {
                           margin: Rect {
                               left: Val::Px(8.0),
                               right: Val::Px(16.0),
                               top: Val::Px(8.0),
                               bottom: Val::Px(8.0)
                           },
                           max_size: Size::new(Val::Px(232.0), Val::Px(32.0)),
                           ..Default::default()
                       },
                       text: Text::with_section(
                           "Hello, World. This is a really really long message for you to render. Sorry.",
                           TextStyle {
                               font: asset_server.load("fonts/RobotoMono-Regular.ttf"),
                               font_size: 16.0,
                               color: Color::WHITE
                           },
                           Default::default()
                       ),
                       ..Default::default()
                   });
                });
            });
}

fn init_load() -> Progress {
    true.into()
}