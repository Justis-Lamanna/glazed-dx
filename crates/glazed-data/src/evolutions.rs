#![allow(non_upper_case_globals)]

use serde::Deserialize;

use crate::attack::{Move, MoveData};
use crate::contest::Condition;
use crate::core::Player;
use crate::locations::GlazedLocation;
use crate::time::{GlazedTime, TimeOfDay};
use crate::item::{EvolutionHeldItem, EvolutionStone, Incense, Item};
use crate::lookups::Lookup;
use crate::pokemon::{Gender, Pokemon};
use crate::species::Species;
use crate::types::Type;

const MIN_CONDITION_TO_EVOLVE: u8 = 170;
const MIN_FRIENDSHIP_TO_EVOLVE: u8 = 220;

/// Represents evolution + breeding data for a species
/// Breeding rules:
/// 1. if `baby`, and parent is holding `baby.incense`, offspring is `baby.species`
/// 2. else, offspring is `base`
#[derive(Debug, Deserialize)]
pub struct Evolution {
    pub id: Species,
    pub base: Species,
    #[serde(default)]
    pub baby: Option<IncenseBaby>,
    #[serde(default)]
    pub paths: Option<Vec<EvolutionPath>>
}

/// Represents a baby Pokemon that can be obtained by breeding while holding an incense
#[derive(Debug, Deserialize)]
pub struct IncenseBaby {
    pub species: Species,
    pub incense: Incense
}

/// Represents one possible evolution for this species
#[derive(Debug, Deserialize)]
pub struct EvolutionPath {
    pub to: Species,
    pub trigger: EvolutionTrigger
}

/// Represents all evolution triggers accepted by the game
#[derive(Debug, Deserialize)]
pub enum EvolutionTrigger {
    /// Evolves when a zero or more conditions are met at level up
    OnLevelUp(Vec<EvolutionCondition>),
    NincadaSpawn,

    /// Evolves when an evolutionary stone is used on it
    EvolutionStone {
        stone: EvolutionStone,
        #[serde(default)]
        conditions: Option<Vec<EvolutionCondition>>
    },

    /// Evolves after the Pokemon was traded and all conditions are met
    Trading {
        #[serde(default)]
        conditions: Option<Vec<EvolutionCondition>>
    },
    /// Evolves after the Pokemon was traded, if the opposite Pokemon was the specific species
    TradingForPokemon(Species)
}

/// Represents any number of sub-conditions that must be fulfilled on reaching a certain level
#[derive(Debug, Deserialize)]
pub enum EvolutionCondition {
    /// Is the Pokemon this level (or greater)?
    Level(u8),
    /// Is the Pokemon this gender?
    Gender(Gender),
    /// Is it current this time of day?
    TimeOfDay(TimeOfDay),
    /// Does the Pokemon have a high friendship? (>= 220)
    HighFriendship,
    /// Does the Pokemon have a high condition? (>= 170)
    HighCondition(Condition),
    /// Is the Pokemon holding this held item?
    HoldingItem(EvolutionHeldItem),
    /// Does the Pokemon know this move?
    KnowsMove(Move),
    /// Does the Pokemon know a move of this type?
    KnowsMoveOfType(Type),
    /// Does the party include a Pokemon of this species?
    WithPartyPokemon(Species),
    /// Is the Pokemon in a specific location?
    AtPlace(EvolutionTriggerLocation),
    /// Does the Pokemon have a low Personality Value?
    LowPersonality,
    /// Does the Pokemon have a high Personality Value?
    HighPersonality,
    /// Is the Pokemon's attack stat greater than its defense?
    HigherAttackThanDefense,
    /// Is the Pokemon's defense stat greater than its attack?
    HigherDefenseThanAttack,
    /// Are the Pokemon's attack and defense stats identical?
    EqualAttackAndDefense
}
impl EvolutionCondition {
    /// Check if a Pokemon meets the Evolution Condition
    pub fn meets_condition(&self, pkmn: &Pokemon, player: &Player) -> bool {
        match self {
            EvolutionCondition::Level(l) => 
                pkmn.level >= *l,
            EvolutionCondition::Gender(g) => 
                pkmn.gender == *g,
            EvolutionCondition::TimeOfDay(d) => 
                GlazedTime::get_time_of_day() == *d,
            EvolutionCondition::HighFriendship => 
                pkmn.friendship >= MIN_FRIENDSHIP_TO_EVOLVE,
            EvolutionCondition::HighCondition(c) => 
                pkmn.contest.get_condition(c) >= MIN_CONDITION_TO_EVOLVE,
            EvolutionCondition::HoldingItem(h) => 
                pkmn.is_holding(Item::from(*h)),
            EvolutionCondition::KnowsMove(m) => 
                pkmn.knows_move(*m),
            EvolutionCondition::KnowsMoveOfType(t) => 
                pkmn.get_moves().iter()
                    .map(|m| MoveData::lookup(m))
                    .any(|md| md._type == *t),
            EvolutionCondition::WithPartyPokemon(p) => 
                player.party.iter().any(|pm| pm.species == *p),
            EvolutionCondition::AtPlace(p) => match p {
                EvolutionTriggerLocation::MossRock => GlazedLocation::is_moss_rock_nearby(),
                EvolutionTriggerLocation::IceRock => GlazedLocation::is_ice_rock_nearby(),
                EvolutionTriggerLocation::MagneticField => GlazedLocation::is_magnetic_field_nearby(),
            },
            EvolutionCondition::LowPersonality => 
                (pkmn.personality >> 16) & 10 < 5,
            EvolutionCondition::HighPersonality => 
                (pkmn.personality >> 16) & 10 >= 5,
            EvolutionCondition::HigherAttackThanDefense => 
                pkmn.attack.value > pkmn.defense.value,
            EvolutionCondition::HigherDefenseThanAttack => 
                pkmn.attack.value < pkmn.defense.value,
            EvolutionCondition::EqualAttackAndDefense => 
                pkmn.attack.value == pkmn.defense.value
        }
    }
}

/// Represents a type of location that can trigger Evolution
#[derive(Debug, Deserialize)]
pub enum EvolutionTriggerLocation {
    MossRock,
    IceRock,
    MagneticField
}