use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::pokemon::Pokemon;

/// Represents the player of the game
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub trainer_id: u16,
    pub secret_id: u16,
    pub money: u32,
    pub party: Vec<Pokemon>
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
            party: Vec::new()
        }
    }
}