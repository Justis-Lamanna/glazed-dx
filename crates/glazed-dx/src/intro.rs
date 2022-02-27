use std::time::Duration;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens, SpriteColorLens};
use bevy_tweening::{Tween, EaseFunction, Animator, Delay, Tracks, Sequence};
use crate::anim::{Timeline, Wait, SSAnimationBuilder, SpriteColorLensOpacityFix};
use crate::GameState;

const PRESENTS: &str = "Milo Marten\nPresents...";

fn create_frames_for_str(string: &str) -> Vec<u64> {
    let mut frames = Vec::new();
    for (idx, c) in string.chars().enumerate() {
        let time = match c {
            ' ' | '\n' => 300,
            '.' => 400,
            _ if idx == 0 => 1000,
            _ => 100
        };
        frames.push(time);
    }
    frames.push(2000);
    frames
}

pub struct Intro;
impl Plugin for Intro {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Intro)
                .with_system(init)
                .with_system(setup)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Intro)
                .with_system(run_milo_marten_presents_timeline)
                .with_system(init_camera_pan)
                .with_system(init_mew_zoom)
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("intro/TitlePan.png"),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..Default::default()
        });
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Assets<TextureAtlas>>, mut camera: Query<(Entity, &mut Transform), With<Camera>>) {
    let presents_timeline = Timeline::from_iter(create_frames_for_str(PRESENTS));
    let presents_timeline_duration = presents_timeline.total_time();

    let (entity, mut transform) = camera.single_mut();
    transform.translation.y = -336f32; 
    
    // Milo Marten Presents...
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
            transform: Transform::from_xyz(0.0, -336.0, 100.0),
            ..Default::default()
        })
        .insert(presents_timeline);

    // Camera pans after Presents text finishes
    commands
        .entity(entity)
        .insert(Wait(presents_timeline_duration));  

    // Mew zooms across the screen during the pan
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("intro/Mew.png"), 
                Vec2::new(48.0, 48.0), 
                10, 1)),
            transform: Transform::from_scale(Vec3::splat(2.0))
                .with_translation(Vec3::new(296.0, -180.0, 20.0)),
            sprite: TextureAtlasSprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Mew)
        .insert(Wait(presents_timeline_duration + Duration::from_millis(3000)));

    // Cliff at bottom of screen
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("intro/Cliff.png"),
            transform: Transform::from_scale(Vec3::splat(2.0))
                .with_translation(Vec3::new(-224.0, -476.0, 10.0)),
            ..Default::default()
        });

    // Quilava standing on cliff (ping pong animation)
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("intro/Quilava.png"), 
                Vec2::new(32.0, 32.0), 
                3, 1)),
            transform: Transform::from_scale(Vec3::splat(2.0))
                .with_translation(Vec3::new(-224.0, -476.0, 12.0)),
            ..Default::default()
        })
        .insert(SSAnimationBuilder::from_iter(vec![0, 1, 2, 1], Duration::from_millis(100))
            .loop_forever().build());

    // Pokemon Logo
    let pokemon_logo_fade = Delay::new(presents_timeline_duration + Duration::from_secs(5))
        .then(Tween::new(
            EaseFunction::QuadraticInOut, 
            bevy_tweening::TweeningType::Once, 
            Duration::from_secs(2), 
            SpriteColorLensOpacityFix { 
                start: Color::rgba(1.0, 1.0, 1.0, 0.0), 
                end: Color::rgba(1.0, 1.0, 1.0, 1.0) 
            }));

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                ..Default::default()
            },
            texture: asset_server.load("intro/Pokemon.png"),
            transform: Transform::from_scale(Vec3::splat(2.0))
                .with_translation(Vec3::new(0.0, 104.0, 200.0)),
            ..Default::default()
        })
        .insert(Animator::new(pokemon_logo_fade))
        ;
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

#[derive(Component)]
struct Done;

fn init_camera_pan(mut commands: Commands, camera: Query<(Entity, &Transform), (With<Camera>, Without<Wait>, Without<Done>)>) {
    for (entity, transform) in camera.iter(){
        let tween = Tween::new(
            EaseFunction::QuadraticInOut,
            bevy_tweening::TweeningType::Once,
            Duration::from_secs(4),
            TransformPositionLens {
                start: transform.translation,
                end: Vec3::new(0.0, 0.0, transform.translation.z),
            }
        );
        commands.entity(entity)
            .insert(Done)
            .insert(Animator::new(tween));
    }
}

fn init_mew_zoom(mut commands: Commands, camera: Query<(Entity, &Transform), (With<Mew>, Without<Wait>, Without<Done>)>) {
    for (entity, transform) in camera.iter(){
        let left_zoom_tween = Sequence::new(vec![Tween::new(
            EaseFunction::QuadraticInOut,
            bevy_tweening::TweeningType::Once,
            Duration::from_millis(1000),
            TransformPositionLens {
                start: transform.translation,
                end: Vec3::new(0.0, 80.0, transform.translation.z),
            }
        )]);
        let scale_tween = Delay::new(Duration::from_millis(250))
            .then(Tween::new(
                EaseFunction::QuadraticOut,
                bevy_tweening::TweeningType::Once,
                Duration::from_millis(650),
                TransformScaleLens {
                    start: Vec3::splat(2.0),
                    end: Vec3::splat(0.0),
                }
            ));
        commands.entity(entity)
            .insert(Done)
            .insert(Animator::new(Tracks::new(vec![left_zoom_tween, scale_tween])));
    }
}

#[derive(Component)]
pub struct Mew;