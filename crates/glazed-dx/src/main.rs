mod intro;
mod anim;
mod state;
mod util;
mod audio;
mod text;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;
use bevy::input::system::exit_on_esc_system;
use bevy_kira_audio::AudioPlugin;
use bevy_tweening::TweeningPlugin;
use crate::anim::GlazedAnimator;
use crate::audio::Cry;
use crate::intro::Title;
use crate::text::TextPlugin;
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

// If this works I can die happy.
// use glyph_brush_layout::{ab_glyph::*, *};

// fn main() {
//     let font = FontRef::try_from_slice(include_bytes!("../assets/fonts/RobotoMono-Regular.ttf")).unwrap();
//     let fonts = &[font];

//     let glyphs = Layout::default().calculate_glyphs(fonts,
//          &SectionGeometry {
//             screen_position: (0.0, 0.0),
//             bounds: (f32::INFINITY, 256.0),
//         }, 
//         &[
//             SectionText {
//                 text: "Hey! This is a really really long string, and we have to find the line breaks in it.",
//                 scale: PxScale::from(16.0),
//                 font_id: FontId(0),
//             }
//         ]);

//     println!("{:?}", glyphs);
// }