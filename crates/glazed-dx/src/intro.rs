use std::time::Duration;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use crate::anim::{SSAnimationBuilder, Timeline};
use crate::GameState;

const PRESENTS: &str = "Milo Marten Presents...";

fn create_frames_for_str(string: &str) -> Vec<u64> {
    let mut frames = Vec::new();
    for (idx, c) in string.chars().enumerate() {
        let time = match c {
            ' ' => 300,
            '.' => 400,
            _ if idx == 0 => 1000,
            _ => 100
        };
        frames.push(time);
    }
    frames
}

pub struct Intro;
impl Plugin for Intro {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Intro)
                .with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Intro)
                .with_system(run_milo_marten_presents_timeline)
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>) {


    let animation =
        SSAnimationBuilder::from_iter(vec![0, 1, 2, 1], Duration::from_millis(200))
            .loop_n_times(5)
            .then_from_iter(vec![0, 3, 4, 5, 6, 7, 8, 9], Duration::from_millis(100))
            .loop_forever()
            .build();

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(asset_server.load("MewLoop.png"), Vec2::new(48.0, 48.0), 10, 1)),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            visibility: Visibility {
                is_visible: false,
            },
            ..Default::default()
        })
        .insert(animation);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/RobotoMono-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE
                        }
                    }
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
                ..Default::default()
            },
            text_2d_size: Text2dSize {
                size: Size {
                    width: 512.0,
                    height: 384.0
                }
            },
            ..Default::default()
        })
        .insert(Timeline::from_iter(create_frames_for_str(PRESENTS)));
}

fn run_milo_marten_presents_timeline(time: Res<'_, Time>, mut commands: Commands, 
        mut animations: Query<(Entity, &mut Timeline, &mut Text)>) {
    for (entity, mut animation, mut text) in animations.iter_mut() {
        if animation.update(time.delta()) {
            let frame = animation.advance();
            if frame < PRESENTS.len() {
                text.sections[0].value = PRESENTS[0..=frame].to_string();
            }

            if animation.is_complete() {
                commands.entity(entity)
                    .remove::<Timeline>();
            }
        }
    }
}