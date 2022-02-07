use rand::RngCore;

use serde::{Serialize, Deserialize};

/// Represents the player of the game
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub trainer_id: u32,
    pub money: u32
}
impl Player {
    pub fn create_player<T>(name: T) -> Player
        where T: Into<String>
    {
        Player {
            name: name.into(),
            trainer_id: rand::thread_rng().next_u32(),
            money: 3000
        }
    }
}

/// Represents the season in the game
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum Season {
    Spring, Summer, Autumn, Winter
}