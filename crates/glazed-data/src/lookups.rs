use lazy_static::lazy_static;
use resource::resource_str;

use std::collections::HashMap;

use crate::item::Berry;
use crate::contest::BerryPokeblockData;

/// Represents this object has associated data
pub trait Lookup<Input> {
    fn lookup(i: &Input) -> &Self;
}

lazy_static! {
    pub static ref BERRY_POKEBLOCK_DATA: HashMap<Berry, BerryPokeblockData> = {
        resource_str!("resources/berry-berrypokeblockdata.yml", |yml: &str| {
            let data: Vec<BerryPokeblockData> = serde_yaml::from_str(yml).unwrap();
            data.into_iter()
                .map(|d| (d.id, d))
                .collect::<HashMap<_, _>>()
        })
    };
}

impl Lookup<Berry> for BerryPokeblockData {
    fn lookup(i: &Berry) -> &Self {
        crate::lookups::BERRY_POKEBLOCK_DATA.get(i).unwrap()
    }
}