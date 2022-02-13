use std::{rc::Rc, sync::RwLock, cell::RefCell};

use lazy_static::lazy_static;

use rand::Rng;
use serde::{Deserialize, Serialize};

/// Represents the player of the game
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub trainer_id: u16,
    pub secret_id: u16,
    pub money: u32,
    pub badges: Badges
}
impl Player {
    pub fn create_player<T>(name: T) -> Player
        where T: Into<String>
    {
        Player {
            name: name.into(),
            trainer_id: rand::thread_rng().gen(),
            secret_id: rand::thread_rng().gen(),
            money: 3000,
            badges: Badges::empty()
        }
    }
}

bitflags::bitflags! {
    /// Represents a badge a Player can have
    #[derive(Serialize, Deserialize)]
    pub struct Badges: u16 {
        const BADGE_1   = 0b0000000000000001;
        const BADGE_2   = 0b0000000000000010;
        const BADGE_3   = 0b0000000000000100;
        const BADGE_4   = 0b0000000000001000;
        const BADGE_5   = 0b0000000000010000;
        const BADGE_6   = 0b0000000000100000;
        const BADGE_7   = 0b0000000001000000;
        const BADGE_8   = 0b0000000010000000;

        const BADGE_9   = 0b0000000100000000;
        const BADGE_10  = 0b0000001000000000;
        const BADGE_11  = 0b0000010000000000;
        const BADGE_12  = 0b0000100000000000;
        const BADGE_13  = 0b0001000000000000;
        const BADGE_14  = 0b0010000000000000;
        const BADGE_15  = 0b0100000000000000;
        const BADGE_16  = 0b1000000000000000;
    }
}
impl Default for Badges {
    fn default() -> Self {
        Badges::empty()
    }
}