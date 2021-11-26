mod core;
pub mod single;
pub mod double;
pub mod tag;

use std::cmp::Ordering;
use std::ops::{Add, Index};
use std::option::Option::Some;
use std::panic::set_hook;
use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{Accuracy, Move};
use glazed_data::item::Item;
use glazed_data::pokemon::Pokemon;

#[derive(Debug)]
pub struct BattlePokemon<'a> {
    pokemon: &'a Pokemon,
    battle_data: &'a BattleData
}
impl <'a> BattlePokemon<'a> {
    fn get_effective_attack(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.attack_stage);
        let raw = f64::from(self.pokemon.attack.value) * multiplier;
        raw as u16
    }

    fn get_effective_defense(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.defense_stage);
        let raw = f64::from(self.pokemon.defense.value) * multiplier;
        raw as u16
    }

    fn get_effective_special_attack(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.special_attack_stage);
        let raw = f64::from(self.pokemon.special_attack.value) * multiplier;
        raw as u16
    }

    fn get_effective_special_defense(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.special_defense_stage);
        let raw = f64::from(self.pokemon.special_defense.value) * multiplier;
        raw as u16
    }

    fn get_effective_speed(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.speed_stage);
        let raw = f64::from(self.pokemon.speed.value) * multiplier;
        raw as u16
    }

    fn get_effective_accuracy(&self) -> f64 {
        core::determine_accuracy_stat_multiplier(self.battle_data.accuracy_stage)
    }

    fn get_effective_evasion(&self) -> f64 {
        core::determine_accuracy_stat_multiplier(self.battle_data.evasion_stage)
    }

    fn has_status_condition(&self) -> bool {
        self.pokemon.status.has_status_condition()
    }
}

pub struct MutBattlePokemon<'a> {
    pokemon: &'a mut Pokemon,
    ailments: &'a mut BattleData
}

/// A group of between 1 and 6 Pokemon, which a trainer owns
#[derive(Debug)]
pub struct Party {
    number_of_members: usize,
    members: [Option<Pokemon>; 6]
}
impl Party {
    pub fn create_one<T>(one: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 1,
            members: [Some(one.into()), None, None, None, None, None]
        }
    }

    pub fn create_two<T>(one: T, two: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 2,
            members: [Some(one.into()), Some(two.into()), None, None, None, None]
        }
    }

    pub fn create_three<T>(one: T, two: T, three: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 3,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), None, None, None]
        }
    }

    pub fn create_four<T>(one: T, two: T, three: T, four: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 4,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), None, None]
        }
    }

    pub fn create_five<T>(one: T, two: T, three: T, four: T, five: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 5,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), Some(five.into()), None]
        }
    }

    pub fn create_six<T>(one: T, two: T, three: T, four: T, five: T, six: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 6,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), Some(five.into()), Some(six.into())]
        }
    }

    pub fn len(&self) -> usize {
        self.number_of_members
    }
}

#[derive(Debug)]
pub struct Battlefield<T> {
    user: T,
    opponent: T,
    weather: Option<Weather>
}
impl <T> Battlefield<T> {
    pub fn is_sunny(&self) -> bool {
        match self.weather {
            Some(Weather::Sun(_)) => true,
            _ => false
        }
    }

    pub fn is_rain(&self) -> bool {
        match self.weather {
            Some(Weather::Rain(_)) => true,
            _ => false
        }
    }

    pub fn is_sandstorm(&self) -> bool {
        match self.weather {
            Some(Weather::Sandstorm(_)) => true,
            _ => false
        }
    }

    pub fn is_hail(&self) -> bool {
        match self.weather {
            Some(Weather::Hail(_)) => true,
            _ => false
        }
    }
}

#[derive(Default, Debug)]
/// Really, anything that needs to be tracked during the course of the battle
struct BattleData {
    attack_stage: i8,
    defense_stage: i8,
    special_attack_stage: i8,
    special_defense_stage: i8,
    speed_stage: i8,
    accuracy_stage: i8,
    evasion_stage: i8,

    lost_held_item: bool,
    turn_count: u8
}

pub trait Action {
    fn get_priority(&self) -> i8;
}

#[derive(Debug)]
enum Weather {
    Rain(u8),
    Sun(u8),
    Hail(u8),
    Sandstorm(u8)
}

#[derive(Debug)]
enum EntryHazard {
    Spikes(u8),
    ToxicSpikes(u8),
    StickyWeb,
    PointedStones
}