#![allow(non_upper_case_globals)]

use serde::Deserialize;

use crate::attack::Move;
use crate::contest::Condition;
use crate::time::TimeOfDay;
use crate::item::{EvolutionStone, Item};
use crate::pokemon::Gender;
use crate::species::Species;
use crate::types::Type;

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
    pub incense: Item
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
    HoldingItem(Item),
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

/// Represents a type of location that can trigger Evolution
#[derive(Debug, Deserialize)]
pub enum EvolutionTriggerLocation {
    MossRock,
    IceRock,
    MagneticField
}