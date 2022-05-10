use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{App, GameState, Plugin};

pub struct Lecture;
impl Plugin for Lecture {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::ProfessorLecture, setup);
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("lecture/background.png"),
        ..default()
    });
}