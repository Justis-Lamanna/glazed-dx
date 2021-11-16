use std::sync::{Mutex, RwLock};
use rand::RngCore;

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Season {
    Spring, Summer, Autumn, Winter
}