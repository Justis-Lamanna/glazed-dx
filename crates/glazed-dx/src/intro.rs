use std::time::Duration;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens, SpriteColorLens};
use bevy_tweening::{Tween, EaseFunction, Animator, Delay, Tracks, Sequence};
use crate::anim::{Timeline, SSAnimationBuilder, AnimationStep};
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
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("intro/TitlePan.png"),
            ..Default::default()
        });
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>, mut textures: ResMut<Assets<TextureAtlas>>, mut camera: Query<(Entity, &mut Transform), With<Camera>>) {
    let presents_timeline = Timeline::from_iter(create_frames_for_str(PRESENTS));
    let presents_timeline_duration = presents_timeline.total_time();

    let (entity, mut transform) = camera.single_mut();
    transform.translation.y = -168f32; 
    
    // Milo Marten Presents...
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/RobotoMono-Regular.ttf"),
                            font_size: 15.0,
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
                    width: 256.0,
                    height: 192.0
                }
            },
            transform: Transform::from_xyz(0.0, -168.0, 100.0),
            ..Default::default()
        })
        .insert(presents_timeline);

    // Camera pans after Presents text finishes
    let camera_pan_tween = Delay::new(presents_timeline_duration)
        .then(Tween::new(
            EaseFunction::QuadraticInOut,
            bevy_tweening::TweeningType::Once,
            Duration::from_secs(4),
            TransformPositionLens {
                start: transform.translation,
                end: Vec3::new(0.0, 0.0, transform.translation.z),
            }
        ));

    commands
        .entity(entity)
        .insert(Animator::new(camera_pan_tween));  

    // Mew zooms across the screen during the pan
    let left_zoom_tween = Sequence::from_single(Tween::new(
        EaseFunction::QuadraticInOut,
        bevy_tweening::TweeningType::Once,
        Duration::from_millis(1000),
        TransformPositionLens {
            start: Vec3::new(148.0, -90.0, 20.0),
            end: Vec3::new(0.0, 40.0, transform.translation.z),
        }
    ));
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

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("intro/Mew.png"), 
                Vec2::new(48.0, 48.0), 
                10, 1)),
            transform: Transform::from_xyz(148.0, -90.0, 20.0),
            sprite: TextureAtlasSprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Animator::new(Delay::new(presents_timeline_duration + Duration::from_millis(3000))
            .then(Tracks::new(vec![left_zoom_tween, scale_tween]))));

    // Cliff at bottom of screen
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("intro/Cliff.png"),
            transform: Transform::from_xyz(-112.0, -238.0, 10.0),
            ..Default::default()
        });

    // Quilava standing on cliff
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("intro/Quilava.png"), 
                Vec2::new(32.0, 32.0), 
                3, 1)),
            transform: Transform::from_xyz(-112.0, -238.0, 12.0),
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
            SpriteColorLens { 
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
            transform: Transform::from_xyz(0.0, 52.0, 200.0),
            ..Default::default()
        })
        .insert(Animator::new(pokemon_logo_fade));

    // Press Enter
    commands
        .spawn_bundle(SpriteSheetBundle {
            visibility: Visibility {
                is_visible: false
            },
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("intro/PressEnter.png"),
                Vec2::new(108.0, 16.0),
                1, 2)),
            transform: Transform::from_xyz(0.0, -64.0, 12.0),
            ..Default::default()
        })
        .insert(SSAnimationBuilder::from_vec(vec![
            AnimationStep::Wait(presents_timeline_duration + Duration::from_secs(8)),
            AnimationStep::Visible(true),
            AnimationStep::Point(0),
            AnimationStep::Wait(Duration::from_millis(500)),
            AnimationStep::AdvanceTo(1),
            AnimationStep::Wait(Duration::from_millis(500)),
            AnimationStep::AdvanceTo(0),
            AnimationStep::JumpToPoint(0)
        ]).build());
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