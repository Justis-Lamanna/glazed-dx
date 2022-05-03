use std::fs::File;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use bevy::reflect::TypeUuid;

#[derive(Deserialize, Default, TypeUuid, Debug)]
#[uuid = "b2f760f1-ea7a-41c7-a293-f37304558f88"]
pub struct GlobalOptions {
    #[serde(default)]
    pub volume: Volume
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
        commands.insert_resource(go);
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