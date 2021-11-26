mod core;
pub mod single;
pub mod double;
pub mod tag;

use std::option::Option::Some;
use glazed_data::abilities::Ability;
use glazed_data::attack::{Accuracy, DamageType, Move};
use glazed_data::constants::Species;
use glazed_data::item::{Incense, Item};
use glazed_data::pokemon::Pokemon;

/// Helper structure that assists in retrieving a Pokemon on the field
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

    fn get_ability(&self) -> &Ability {
        self.battle_data.temp_ability.as_ref().unwrap_or_else(|| self.pokemon.get_ability())
    }
}

/// Helper structure that assists in retrieving a mutable Pokemon on the field
pub struct MutBattlePokemon<'a> {
    pokemon: &'a mut Pokemon,
    ailments: &'a mut BattleData
}

/// Represents one side of a battlefield
#[derive(Default, Debug)]
pub struct Side {
    hazard: Option<EntryHazard>,
    tailwind: u8
}

/// Represents the entire battlefield
#[derive(Default, Debug)]
pub struct Field {
    weather: Option<Weather>,
    gravity: bool
}
impl Field {
    /// Return if harsh sunlight is present on the field
    pub fn is_sunny(&self) -> bool {
        match self.weather {
            Some(Weather::Sun(_)) => true,
            _ => false
        }
    }

    /// Return if it is raining on the field
    pub fn is_rain(&self) -> bool {
        match self.weather {
            Some(Weather::Rain(_)) => true,
            _ => false
        }
    }

    /// Return if there is a sandstorm on the field
    pub fn is_sandstorm(&self) -> bool {
        match self.weather {
            Some(Weather::Sandstorm(_)) => true,
            _ => false
        }
    }

    /// Return if there is hail on the field
    pub fn is_hail(&self) -> bool {
        match self.weather {
            Some(Weather::Hail(_)) => true,
            _ => false
        }
    }

    pub fn is_foggy(&self) -> bool {
        match self.weather {
            Some(Weather::Fog) => true,
            _ => false
        }
    }
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

    pub fn any_match<F>(&self, func: F) -> bool
        where F: Fn(&Pokemon) -> bool
    {
        self.members.iter().any(|slot| {
            match slot {
                Some(p) => func(p),
                None => false
            }
        })
    }
}

/// Represents the current battlefield
#[derive(Debug)]
pub struct Battlefield<T> {
    user: T,
    opponent: T,
    field: Field
}
impl <T> Battlefield<T> {
    /// Get the current effective speed of a specific Pokemon on a specific side of the field
    /// This takes into account:
    /// * Raw speed stat of the Pokemon
    /// * Speed stage
    /// * Abilities, if applicable to the current state of the field
    /// * Items, if applicable to the current state of the field
    /// * Other statuses, such as paralysis or Tailwind
    fn get_effective_speed(&self, side: &Side, pokemon: &BattlePokemon) -> u16 {
        let speed = pokemon.get_effective_speed(); //Raw speed + stage multipliers

        // Ability modifiers
        let ability_multiplier = match pokemon.get_ability() {
            Ability::Chlorophyll if self.field.is_sunny() => 2.0,
            Ability::SandRush if self.field.is_sandstorm() => 2.0,
            Ability::SwiftSwim if self.field.is_rain() => 2.0,
            Ability::SlushRush if self.field.is_hail() => 2.0,
            Ability::QuickFeet if pokemon.has_status_condition() => 1.5,
            Ability::Unburden if pokemon.battle_data.lost_held_item => 2.0,
            Ability::SlowStart if pokemon.battle_data.turn_count < 5 => 0.5,
            _ => 1.0
        };

        let item_multiplier = match pokemon.pokemon.held_item {
            Some(Item::ChoiceScarf) => 1.5,
            Some(Item::QuickPowder) if pokemon.pokemon.species == Species::Ditto => 2.0,
            _ => 1.0
        };

        let mut eff_speed = f64::from(speed) * ability_multiplier * item_multiplier;
        if side.tailwind > 0 {
            eff_speed *= 2.0;
        }
        if pokemon.pokemon.status.paralysis {
            eff_speed *= 0.5;
        }

        eff_speed as u16
    }

    /// Gets the factor of accuracy for a user hitting the defender with the move. This is, essentially,
    /// the probability (out of 100) of a hit landing.
    /// Takes into account:
    /// * Move Accuracy
    /// * User Accuracy and Target Evasion
    /// * Abilities (User and Target)
    /// * Held Items (User and Target)
    /// Documentation is vague on what part of the equation the modifiers are applied to. Some moves
    /// affect the accuracy of the move, while others affect the accuracy of the Pokemon
    /// Does not take into account:
    /// * Allied Pokemon with Victory Star. Double and Tag battles need to account for this on their own.
    fn get_accuracy_factor(&self, user: &BattlePokemon, attack: Move, defender: &BattlePokemon) -> f64 {
        let move_data = attack.data();
        let move_hit_percent = match move_data.accuracy {
            Accuracy::AlwaysHits => return 100f64,
            Accuracy::Percentage(p) => f64::from(p)
        };

        let raw_accuracy = move_hit_percent *
            //Clamping is unnecessary, since it is handled in this method
            core::determine_accuracy_stat_multiplier(user.battle_data.accuracy_stage - defender.battle_data.evasion_stage);

        let mut field_modifier = match self.field.weather {
            Some(Weather::Fog) => 3f64 / 5f64,
            _ => 1.0
        };
        if self.field.gravity {
            field_modifier *= 5f64 / 3f64
        }

        let user_ability_modifier = match user.get_ability() {
            Ability::CompoundEyes => 1.3,
            Ability::VictoryStar => 1.1,
            Ability::Hustle if move_data.damage_type == DamageType::Physical => 0.8,
            _ => 1.0
        };

        let user_item_modifier = match user.pokemon.held_item {
            Some(Item::WideLens) => 1.1,
            Some(Item::ZoomLens) if defender.battle_data.used_move_this_turn => 1.2,
            _ => 1.0
        };

        let defender_ability_modifier = match defender.get_ability() {
            Ability::WonderSkin if move_data.damage_type == DamageType::Status => 0.5,
            Ability::SandVeil if self.field.is_sandstorm() => 4f64 / 5f64,
            Ability::SnowCloak if self.field.is_hail() => 4f64 / 5f64,
            Ability::TangledFeet if defender.battle_data.is_confused() => 0.5,
            _ => 1.0
        };

        let defender_item_modifier = match defender.pokemon.held_item {
            Some(Item::BrightPowder) | Some(Item::Incense(Incense::LaxIncense)) => 0.9,
            _ => 1.0
        };

        raw_accuracy * field_modifier * user_ability_modifier * user_item_modifier * defender_ability_modifier * defender_item_modifier
    }
}

#[derive(Default, Debug)]
/// Really, anything that needs to be tracked during the course of the battle
/// When the pokemon is switched out, everything here is reset to defaults
struct BattleData {
    /// The number of turns this Pokemon has been in battle
    turn_count: u8,
    /// Set to true when this Pokemon has used a turn this move
    used_move_this_turn: bool,
    /// The last move that this Pokemon used
    last_used_move: Option<Move>,

    attack_stage: i8,
    defense_stage: i8,
    special_attack_stage: i8,
    special_defense_stage: i8,
    speed_stage: i8,
    accuracy_stage: i8,
    evasion_stage: i8,

    /// Turns remaining bound (0 == not bound)
    bound: u8,
    /// If true, this Pokemon can't run away or switch out
    cant_flee: bool,
    /// If true, this Pokemon loses 25% HP at the end of the turn
    cursed: bool,
    /// If true, this Pokemon falls asleep at turn end
    drowsy: bool,
    /// Turns remaining where this Pokemon can't use items (0 == no embargo)
    embargo: u8,
    /// Turns remaining where this Pokemon has to use the last_used_move (0 == no encore)
    encore: u8,
    /// Turns remaining where this Pokemon is confused (0 == no confusion)
    confused: u8,
    /// If true, this Pokemon is infatuated
    infatuated: bool,
    /// If true, this Pokemon will flinch instead of attack this turn
    flinch: bool,
    /// Turns remaining where this Pokemon is unable to heal
    unable_to_heal: u8,
    /// If true, this Pokemon was identified and has evasion checks revoked
    identified: bool,
    /// Turns left before Perish Song KOs this Pokemon
    perish_song: u8,
    /// If true, this Pokemon is having a nightmare
    nightmare: bool,
    /// If true, this Pokemon cannot use Status moves
    taunted: u8,
    /// If true, this Pokemon is levitating
    levitating: u8,
    /// If true, this Pokemon can use any move except last_used_move
    tormented: bool,
    /// If true, this Pokemon gains 1/16 HP at the end of each turn
    aqua_ringed: bool,
    /// If present, this Pokemon will lose HP and the referred Battler will gain it
    leeched: Option<Battler>,
    /// Number of times braced. (0 == not tried)
    braced: u8,
    /// If present, this Pokemon is charging, and will use Move on the next turn
    charging: Option<Move>,
    /// If true, this Pokemon is the center of attention, and all moves target it
    center_of_attention: bool,
    /// If true, this Pokemon used Defense Curl. Certain moves are more powerful
    curled: bool,
    /// If true, this Pokemon is rooted to the ground. Gains HP at the end of each turn
    rooted: bool,
    /// If true, any status moves used against this Pokemon are rebounded
    magic_coated: bool,
    /// If true, the Pokemon is shrunk
    minimized: bool,
    /// Number of times protected. (0 == not tried)
    protected: u8,
    /// If true, the Pokemon is exhausted from its last attack, and cannot do anything
    recharging: bool,
    /// If present, the Pokemon is semi-invulnerable
    invulnerable: Option<SemiInvulnerableLocation>,
    /// If present, the Pokemon is carrying another (Sky Drop)
    carrying: Option<Battler>,
    /// The Pokemon has a substitute out, with this much HP remaining
    substituted: u16,
    /// This Pokemon is thrashing, and must continue using last_used_move.
    thrashing: u8,
    /// This Pokemon is transformed into another.
    transformed: bool, //TODO: Maintain a copy of everything transformed

    /// If true, this Pokemon had a held item + subsequently lost it
    lost_held_item: bool,
    /// If present, this Pokemon has this ability instead of its usual one
    temp_ability: Option<Ability>
}
impl BattleData {
    pub fn is_confused(&self) -> bool {
        self.confused > 0
    }
}

/// Methods common to all actions
pub trait Action {
    /// Get the priority of this move
    /// 0 is default. >0 means it will happen sooner, and <0 means it will happen later
    fn get_priority(&self) -> i8;
}

/// Identifier of a member on the field
#[derive(Debug)]
pub struct Battler(u8);

/// One of the possible weather conditions that can occur on the field
#[derive(Debug)]
enum Weather {
    Rain(u8),
    Sun(u8),
    Hail(u8),
    Sandstorm(u8),
    Fog
}

/// One of the possible entry hazards that can occur on one side of the field
#[derive(Debug)]
enum EntryHazard {
    Spikes(u8),
    ToxicSpikes(u8),
    StickyWeb,
    PointedStones
}

#[derive(Debug)]
enum SemiInvulnerableLocation {
    Underground,
    Underwater,
    InAir
}