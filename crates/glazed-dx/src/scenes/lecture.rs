use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{App, GameState, Plugin, util::despawn};

#[derive(Component)]
pub struct LectureAsset;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum LectureState {
    Opening
}

pub struct Lecture;
impl Plugin for Lecture {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(LectureState::Opening)
            .add_enter_system(GameState::ProfessorLecture, setup)
            .add_exit_system(GameState::ProfessorLecture, despawn::<LectureAsset>)
        ;
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("lecture/background.png"),
        ..default()
    }).insert(LectureAsset);
}