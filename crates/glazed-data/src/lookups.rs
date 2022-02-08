use lazy_static::lazy_static;
use resource::resource_str;
use serde::Deserialize;
use serde::Serialize;

use std::collections::{BTreeMap, HashMap};

use crate::item::Berry;
use crate::attack::{Move, MoveData};
use crate::contest::BerryPokeblockData;
use crate::evolutions::EvolutionTrigger;
use crate::pokemon::SpeciesData;
use crate::species::Species;

/// Represents this object has associated data
pub trait Lookup<Input> {
    fn lookup(i: &Input) -> &Self;
}

/// Represents a more YAML-friendly structure for complex data
/// This allows for re-use of the data attribute between members by leveraging
/// YAML's ability to have anchors/aliases.
#[derive(Deserialize)]
struct YamlFriendlySpeciesData<DATA> {
    id: Species,
    data: DATA
}

pub fn test() {

}

lazy_static! {
    pub static ref SPECIES_DATA: HashMap<Species, SpeciesData> = {
        resource_str!("resources/pokemon/speciesdata.yml", |yml: &str| {
            let data: Vec<YamlFriendlySpeciesData<SpeciesData>> = serde_yaml::from_str(yml).unwrap();
            data.into_iter()
                .map(|d| (d.id, d.data))
                .collect::<HashMap<_, _>>()
        })
    };

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

impl Lookup<Species> for SpeciesData {
    fn lookup(i: &Species) -> &Self {
        SPECIES_DATA.get(i).unwrap()
    }
}