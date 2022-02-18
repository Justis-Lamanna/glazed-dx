use std::time::Duration;
use bevy::prelude::*;
use crate::anim::{SSAnimation, SSAnimationBuilder};
use crate::GameState;

pub struct Intro;
impl Plugin for Intro {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Intro)
                .with_system(setup)
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>) {

    let animation = {
        SSAnimationBuilder::from_iter(vec![0, 1, 2, 1], Duration::from_millis(200))
            .loop_n_times(5)
            .then_from_iter(vec![0, 3, 4, 5, 6, 7, 8, 9], Duration::from_millis(100))
            .loop_forever()
            .build()
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(asset_server.load("MewLoop.png"), Vec2::new(48.0, 48.0), 10, 1)),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        })
        .insert(animation);
}