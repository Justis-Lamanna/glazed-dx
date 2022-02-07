use lazy_static::lazy_static;
use resource::resource_str;

use std::collections::HashMap;

use crate::item::Berry;
use crate::attack::{Move, MoveData};
use crate::contest::BerryPokeblockData;

/// Represents this object has associated data
pub trait Lookup<Input> {
    fn lookup(i: &Input) -> &Self;
}

use serde::{Serialize, Deserialize};

lazy_static! {
    pub static ref MOVE_DATA: HashMap<Move, MoveData> = {
        resource_str!("resources/movedata.yml", |yml: &str| {
            let data: Vec<MoveData> = serde_yaml::from_str(yml).unwrap();
            data.into_iter()
                .map(|d| (d.id, d))
                .collect::<HashMap<_, _>>()
        })
    };

    pub static ref BERRY_POKEBLOCK_DATA: HashMap<Berry, BerryPokeblockData> = {
        resource_str!("resources/berrypokeblockdata.yml", |yml: &str| {
            let data: Vec<BerryPokeblockData> = serde_yaml::from_str(yml).unwrap();
            data.into_iter()
                .map(|d| (d.id, d))
                .collect::<HashMap<_, _>>()
        })
    };
}

impl Lookup<Berry> for BerryPokeblockData {
    fn lookup(i: &Berry) -> &Self {
        BERRY_POKEBLOCK_DATA.get(i).unwrap()
    }
}

impl Lookup<Move> for MoveData {
    fn lookup(i: &Move) -> &Self {
        MOVE_DATA.get(i).unwrap()
    }
}