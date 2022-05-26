mod anim;
mod state;
mod util;
mod pkmn;
mod text;
mod controls;
mod scenes;
mod actions;
mod locale;
mod player;

use bevy::prelude::*;
use glazed_data::pokemon::{PokemonTemplate, Pokemon};
use glazed_data::species::Species;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy::input::system::exit_on_esc_system;
use bevy_kira_audio::AudioPlugin;
use bevy_tweening::TweeningPlugin;
use locale::FluentData;
use player::{Player, PlayerService};
use crate::anim::GlazedAnimator;
use crate::pkmn::{Cry, PkmnPlugin, PokemonDataFiles};
use crate::controls::Actions;
use crate::scenes::intro::Title;
use crate::scenes::lecture::Lecture;
use crate::text::TextPlugin;
use crate::state::GlobalOptions;
use crate::util::TransitionPlugin;

pub const SCREEN_WIDTH: f32 = 400.0;
pub const SCREEN_HEIGHT: f32 = 224.0;
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
        .insert_resource(Player {
            name: "Milo Marten".into(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GlazedAnimator)
        .add_plugin(TweeningPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(InputManagerPlugin::<Actions>::default())
        .add_startup_system(setup)
        .add_startup_system(GlobalOptions::load)
        .add_system(exit_on_esc_system)

        // Random Plugins
        .add_plugin(PkmnPlugin)
        .add_plugin(TransitionPlugin)
        .add_plugin(TextPlugin)
        .add_plugin(actions::ActionsPlugin)
        .add_plugin(locale::TranslationPlugin::default())

        // Splash Screen
        .add_loopless_state(GameState::Splash)
        .add_plugin(
            ProgressPlugin::new(GameState::Splash)
            .continue_to(GameState::Title)
            .track_assets()
        )
        .add_enter_system(GameState::Splash, init_load)
        .add_enter_system(GameState::Title, test)

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

fn init_load(mut commands: Commands, ass: Res<AssetServer>, locales: Res<FluentData>, mut loading: ResMut<AssetsLoading>) {
    info!("Loading important assets");
    let font: Handle<Font> = ass.load(text::FONT);
    loading.add(font);

    for handle in locales.get_handles() {
        loading.add(handle);
    }

    let species_data = ass.load("pkmn/data.pkmn");
    loading.add(species_data.clone());

    commands.insert_resource(PokemonDataFiles {
        species_data
    });
}

fn test(player: PlayerService) {
    
}