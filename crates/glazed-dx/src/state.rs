use std::fs::File;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Deserializer};
use bevy::reflect::TypeUuid;
use crate::controls::Actions;
use crate::PlayerData;

#[derive(Deserialize, Default, TypeUuid, Debug)]
#[uuid = "b2f760f1-ea7a-41c7-a293-f37304558f88"]
pub struct GlobalOptions {
    #[serde(default)]
    pub volume: Volume,
    #[serde(default)]
    pub keyboard_controls: Controls<KeyCode>,
    #[serde(default)]
    pub gamepad_controls: Controls<GamepadButtonType>
}
impl GlobalOptions {
    pub fn load(mut commands: Commands) {
        let go = match File::open("options.yml") {
            Ok(f) => {
                match serde_yaml::from_reader(f) {
                    Ok(r) => {
                        info!("Using options {:?}", r);
                        r
                    },
                    Err(e) => {
                        warn!("Error parsing options.yml: {}. Using defaults", e);
                        GlobalOptions::default()
                    }
                }
            },
            Err(e) => {
                warn!("Error opening options.yml: {}. Using defaults", e);
                GlobalOptions::default()
            }
        };

        let mut m = InputMap::default();
        Self::add_to_input_map(&mut m, Actions::Up, &go.keyboard_controls.up, &go.gamepad_controls.up);
        Self::add_to_input_map(&mut m, Actions::Down, &go.keyboard_controls.down, &go.gamepad_controls.down);
        Self::add_to_input_map(&mut m, Actions::Left, &go.keyboard_controls.left, &go.gamepad_controls.left);
        Self::add_to_input_map(&mut m, Actions::Right, &go.keyboard_controls.right, &go.gamepad_controls.right);
        Self::add_to_input_map(&mut m, Actions::Accept, &go.keyboard_controls.accept, &go.gamepad_controls.accept);
        Self::add_to_input_map(&mut m, Actions::Cancel, &go.keyboard_controls.cancel, &go.gamepad_controls.cancel);

        // A place for Player data to live
        commands.spawn()
            .insert(PlayerData)
            .insert_bundle(InputManagerBundle::<Actions> {
                action_state: ActionState::default(),
                input_map: m
            });

        commands.insert_resource(go);
    }

    fn add_to_input_map(map: &mut InputMap<Actions>, action: Actions, codes: &Vec<KeyCode>, codes2: &Vec<GamepadButtonType>) {
        map.insert_multiple(codes.iter().map(|i| (action, *i)));
        map.insert_multiple(codes2.iter().map(|i| (action, *i)));
    }
}

#[derive(Deserialize, Debug)]
pub struct Volume {
    #[serde(default = "half")]
    master: f32,
    #[serde(default = "one")]
    music: f32,
    #[serde(default = "one")]
    sfx: f32
}
impl Volume {
    pub fn get_raw_music_volume(&self) -> f32 { self.music }
    pub fn get_raw_sfx_volume(&self) -> f32 { self.sfx }
    pub fn set_master_volume(&mut self, val: f32) { self.master = val }
    pub fn set_raw_music_volume(&mut self, val: f32) { self.music = val }
    pub fn set_raw_sfx_volume(&mut self, val: f32) { self.sfx = val }

    pub fn get_master_volume(&self) -> f32 { self.master.clamp(0.0, 1.0) }
    pub fn get_music_volume(&self) -> f32 { (self.master * self.music).clamp(0.0, 1.0) }
    pub fn get_sfx_volume(&self) -> f32 { (self.master * self.sfx).clamp(0.0, 1.0) }
}
impl Default for Volume {
    fn default() -> Self {
        Volume {
            master: 0.5,
            music: 1.0,
            sfx: 1.0
        }
    }
}
fn half() -> f32 { 0.5 }
fn one() -> f32 { 1.0 }

#[derive(Deserialize, Debug)]
pub struct Controls<T> {
    pub up: Vec<T>,
    pub down: Vec<T>,
    pub left: Vec<T>,
    pub right: Vec<T>,
    pub accept: Vec<T>,
    pub cancel: Vec<T>
}
impl Default for Controls<KeyCode> {
    fn default() -> Self {
        Controls {
            up: vec![KeyCode::Up, KeyCode::W],
            down: vec![KeyCode::Down, KeyCode::S],
            left: vec![KeyCode::Left, KeyCode::A],
            right: vec![KeyCode::Right, KeyCode::D],
            accept: vec![KeyCode::Return, KeyCode::Z],
            cancel: vec![KeyCode::Back, KeyCode::X]
        }
    }
}
impl Default for Controls<GamepadButtonType> {
    fn default() -> Self {
        Controls {
            up: vec![GamepadButtonType::DPadUp],
            down: vec![GamepadButtonType::DPadDown],
            left: vec![GamepadButtonType::DPadLeft],
            right: vec![GamepadButtonType::DPadRight],
            accept: vec![GamepadButtonType::South],
            cancel: vec![GamepadButtonType::East]
        }
    }
}

pub enum SaveGameState {
    NewGame,
    Save(String)
}

pub struct Save;
impl Save {
    pub fn check_for_saves() -> Result<bool, ()> {
        use std::fs;
        use std::path::Path;

        if Path::new("./saves").is_dir() {
            match fs::read_dir("./saves") {
                Ok(mut a) => {
                    if a.any(|_| true) {
                        Result::Ok(true)
                    } else {
                        Result::Ok(false)
                    }
                },
                Err(_) => Result::Err(())
            }
        } else {
            match fs::create_dir("./saves") {
                Ok(_) => Result::Ok(false),
                Err(_) => Result::Err(())
            }
        }
    }
}