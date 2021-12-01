mod core;
pub mod single;
pub mod double;
pub mod tag;

use std::option::Option::Some;
use either::Either;
use rand::Rng;
use glazed_data::abilities::{Ability, PokemonAbility};
use glazed_data::attack::{Accuracy, BattleStat, DamageType, Effect, Move, MoveData, NonVolatileBattleAilment, Power, StatChangeTarget};
use glazed_data::constants::Species;
use glazed_data::item::{EvolutionHeldItem, Incense, Item};
use glazed_data::pokemon::{AbilitySlot, MoveSlot, Pokemon, StatSlot};
use glazed_data::types::{Effectiveness, PokemonType, Type};

/// Helper structure that assists in retrieving a Pokemon on the field
#[derive(Debug)]
pub struct BattlePokemon<'a> {
    id: Battler,
    pokemon: &'a Pokemon,
    battle_data: &'a BattleData
}
impl <'a> BattlePokemon<'a> {
    /// Get the species of this Pokemon. Takes Transform into account
    fn get_effective_species(&self) -> &Species {
        match &self.battle_data.transformed {
            None => &self.pokemon.species,
            Some(t) => &t.species
        }
    }

    /// Get the effective attack of this Pokemon. Takes Transform, Power Trick, and Attack stage into account
    fn get_effective_attack(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.attack_stage);
        let raw = match &self.battle_data.transformed {
            None => f64::from(if self.battle_data.power_trick {self.pokemon.defense.value} else {self.pokemon.attack.value}),
            Some(t) => f64::from(if self.battle_data.power_trick {t.defense.value} else {t.attack.value})
        } * multiplier;
        raw as u16
    }

    /// Get the effective defense of this Pokemon. Takes Transform, Power Trick, and Defense stage into account
    fn get_effective_defense(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.defense_stage);
        let raw = match &self.battle_data.transformed {
            None => f64::from(if self.battle_data.power_trick {self.pokemon.attack.value} else {self.pokemon.defense.value}),
            Some(t) => f64::from(if self.battle_data.power_trick {t.attack.value} else {t.defense.value})
        } * multiplier;
        raw as u16
    }

    /// Get the effective special attack of this Pokemon. Takes Transform and Sp. Attack stage into account
    fn get_effective_special_attack(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.special_attack_stage);
        let raw = match &self.battle_data.transformed {
            None => f64::from(self.pokemon.special_attack.value),
            Some(t) => f64::from(t.special_attack.value)
        } * multiplier;
        raw as u16
    }

    /// Get the effective special defense of this Pokemon. Takes Transform and Sp. Defense stage into account
    fn get_effective_special_defense(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.special_defense_stage);
        let raw = match &self.battle_data.transformed {
            None => f64::from(self.pokemon.special_defense.value),
            Some(t) => f64::from(t.special_defense.value)
        } * multiplier;
        raw as u16
    }

    /// Get the effective speed of this Pokemon. Takes Transform and Speed stage into account
    fn get_effective_speed(&self) -> u16 {
        let multiplier = core::determine_stat_multiplier(self.battle_data.speed_stage);
        let raw = match &self.battle_data.transformed {
            None => f64::from(self.pokemon.speed.value),
            Some(t) => f64::from(t.speed.value)
        } * multiplier;
        raw as u16
    }

    /// Check if this Pokemon has a status condition (may be expanded to handle volatile status ailments)
    fn has_status_condition(&self) -> bool {
        self.pokemon.status.has_status_condition()
    }

    /// Get the effective ability of this Pokemon. Takes Transform and temporary Ability changes into effect
    fn get_effective_ability(&self) -> &Ability {
        self.battle_data.temp_ability.as_ref().unwrap_or_else(|| {
            match &self.battle_data.transformed {
                None => self.pokemon.get_ability(),
                Some(t) => t.get_ability()
            }
        })
    }

    /// If true, this Pokemon can ignore negative abilities
    fn is_mold_breaker(&self) -> bool {
        match *self.get_effective_ability() {
            Ability::MoldBreaker | Ability::Turboblaze | Ability::Teravolt => true,
            _ => false
        }
    }

    /// Get the effective type(s) of this Pokemon. Taks Transform and temporary Type changes into effect
    fn get_effective_type(&self) -> &PokemonType {
        self.battle_data.temp_type.as_ref().unwrap_or_else(|| {
            match &self.battle_data.transformed {
                None => &self.pokemon.species.data()._type,
                Some(t) => &t.species.data()._type
            }
        })
    }

    fn is_half_health_or_worse(&self) -> bool {
        let current = u32::from(self.pokemon.current_hp);
        let max = u32::from(self.pokemon.hp.value);
        current * 2u32 <= max
    }

    fn is_quarter_health_or_worse(&self) -> bool {
        let current = u32::from(self.pokemon.current_hp);
        let max = u32::from(self.pokemon.hp.value);
        current * 4u32 <= max
    }
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
    gravity: u8
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

pub trait BattleTypeTrait {
    fn get_by_id(&self, id: &Battler) -> Option<BattlePokemon>;
    fn do_by_id<F>(&mut self, _id: &Battler, _func: F) where
        F: Fn(&mut Pokemon, &mut BattleData) -> ()
    {
        unimplemented!()
    }
    fn get_side(&self) -> &Side;
}

/// Represents the current battlefield
#[derive(Debug)]
pub struct Battlefield<T> where T: BattleTypeTrait {
    user: T,
    opponent: T,
    field: Field,
    turn_record: Vec<Turn>
}
impl <T> Battlefield<T> where T: BattleTypeTrait {
    fn get_side_by_id(&self, id: &Battler) -> &Side {
        let Battler(side, _) = id;
        if *side {
            self.opponent.get_side()
        } else {
            self.user.get_side()
        }
    }

    fn get_by_id(&self, id: &Battler) -> Option<BattlePokemon> {
        let Battler(side, _) = id;
        if *side {
            self.opponent.get_by_id(id)
        } else {
            self.user.get_by_id(id)
        }
    }

    fn do_by_id<F>(&mut self, id: &Battler, func: F) -> () where
        F: Fn(&mut Pokemon, &mut BattleData) -> () {
        let Battler(side, _) = id;
        if *side {
            self.opponent.do_by_id(id, func)
        } else {
            self.user.do_by_id(id, func)
        }
    }

    fn get_ally(&self, id: &Battler) -> Option<BattlePokemon> {
        let Battler(side, id) = id;
        if *id == 0 {
            self.get_by_id(&Battler(*side, 1))
        } else {
            self.get_by_id(&Battler(*side, 0))
        }
    }

    /// Gets the effective current HP of a Pokemon on the field
    /// Returns the substitute's HP if the battler has one up, otherwise returns the battler's current HP
    fn get_effective_current_hp(&self, id: &Battler) -> u16 {
        let data = self.get_by_id(id).unwrap();
        if data.battle_data.substituted > 0 {
            data.battle_data.substituted
        } else {
            data.pokemon.current_hp
        }
    }

    /// Gets the effective attack stat of a Pokemon on the field
    /// This takes into account:
    /// * Raw attack stat (or defense stat, if afflicted with Power Trick)
    /// * Attack stage
    /// * Burn (+ Immunity to burn attack drop if ability is Guts
    /// * Abilities
    /// * Items
    /// * Transform
    fn get_effective_attack(&self, id: &Battler) -> f64 {
        let user = self.get_by_id(id).unwrap();
        let atk = f64::from(user.get_effective_attack()); //Raw attack + stage multipliers + Power Trick

        let ability_multiplier = match user.get_effective_ability() {
            Ability::FlowerGift if self.field.is_sunny() => 1.5,
            Ability::Guts if user.has_status_condition() => 1.5,
            Ability::HugePower => 2.0,
            Ability::Hustle => 1.5,
            Ability::PurePower => 2.0,
            Ability::Defeatist if user.is_half_health_or_worse() => 0.5,
            Ability::SlowStart if user.battle_data.turn_count < 5 => 0.5,
            _ => 1.0
        };

        let item_multiplier = match user.pokemon.held_item {
            Some(Item::ChoiceBand) => 1.5,
            Some(Item::LightBall) if *user.get_effective_species() == Species::Pikachu => 2.0,
            Some(Item::ThickClub) if *user.get_effective_species() == Species::Marowak => 2.0,
            _ => 1.0
        };

        atk * ability_multiplier * item_multiplier
    }

    /// Gets the effective defense stat of a Pokemon on the field
    /// This takes into account:
    /// * Raw defense stat (or attack stat, if afflicted with Power Trick)
    /// * Defense stage
    /// * Abilities
    /// * Items
    /// * Transform
    fn get_effective_defense(&self, id: &Battler) -> f64 {
        let user = self.get_by_id(id).unwrap();
        let def = f64::from(user.get_effective_defense()); //Raw defense + stage multipliers + Power Trick

        let ability_multiplier = match user.get_effective_ability() {
            Ability::MarvelScale if user.has_status_condition() => 1.5,
            _ => 1.0
        };

        let item_multiplier = match user.pokemon.held_item {
            Some(Item::Eviolite) if !user.get_effective_species().is_fully_evolved() => 1.5,
            Some(Item::MetalPowder) if user.pokemon.species == Species::Ditto && user.battle_data.transformed.is_none() => 2.0,
            _ => 1.0
        };

        def * ability_multiplier * item_multiplier
    }

    /// Gets the effective special attack stat of a Pokemon on the field
    /// This takes into account:
    /// * Raw special attack stat
    /// * Special Attack stage
    /// * Abilities
    /// * Items
    /// * Transform
    /// * Plus or Minus, if the Ally also has Plus or Minus (no restrictions on two Plus or two Minus)
    fn get_effective_special_attack(&self, id: &Battler) -> f64 {
        let user = self.get_by_id(id).unwrap();
        let spa = f64::from(user.get_effective_special_attack()); //Raw SpA + stage multipliers

        let ability_multiplier = match user.get_effective_ability() {
            Ability::Plus | Ability::Minus => {
                match self.get_ally(&id) {
                    Some(p) if *p.get_effective_ability() == Ability::Plus
                        || *p.get_effective_ability() == Ability::Minus => 1.5,
                    _ => 1.0
                }
            }
            Ability::SolarPower if self.field.is_sunny() => 1.5,
            Ability::Defeatist if user.is_half_health_or_worse() => 0.5,
            _ => 1.0
        };

        let item_multiplier = match user.pokemon.held_item {
            Some(Item::ChoiceSpecs) => 1.5,
            Some(Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaTooth))
                if *user.get_effective_species() == Species::Clamperl => 2.0,
            Some(Item::LightBall) if *user.get_effective_species() == Species::Pikachu => 2.0,
            Some(Item::SoulDew) if *user.get_effective_species() == Species::Latias || *user.get_effective_species() == Species::Latios => 1.5,
            _ => 1.0
        };

        spa * ability_multiplier * item_multiplier
    }

    /// Gets the effective special defense stat of a Pokemon on the field
    /// This takes into account:
    /// * Raw special defense stat
    /// * Special Defense stage
    /// * Abilities
    /// * Items
    /// * Transform
    fn get_effective_special_defense(&self, id: &Battler) -> f64 {
        let user = self.get_by_id(id).unwrap();
        let spd = f64::from(user.get_effective_special_defense());

        let ability_multiplier = match user.get_effective_ability() {
            Ability::FlowerGift if self.field.is_sunny() => 1.5,
            _ => 1.0
        };

        let item_multiplier = match user.pokemon.held_item {
            Some(Item::AssaultVest) => 1.5,
            Some(Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaScale))
                if *user.get_effective_species() == Species::Clamperl => 2.0,
            Some(Item::Eviolite) if !user.get_effective_species().is_fully_evolved() => 1.5,
            Some(Item::MetalPowder) if user.pokemon.species == Species::Ditto => 1.5,
            Some(Item::SoulDew) if *user.get_effective_species() == Species::Latias || *user.get_effective_species() == Species::Latios => 1.5,
            _ => 1.0
        };

        spd * ability_multiplier * item_multiplier
    }

    /// Get the current effective speed of a specific Pokemon on a specific side of the field
    /// This takes into account:
    /// * Raw speed stat of the Pokemon
    /// * Speed stage
    /// * Abilities, if applicable to the current state of the field
    /// * Items, if applicable to the current state of the field
    /// * Other statuses, such as paralysis or Tailwind
    /// * Transform
    fn get_effective_speed(&self, id: &Battler) -> f64 {
        let pokemon = self.get_by_id(id).unwrap();
        let side = self.get_side_by_id(id);
        let speed = pokemon.get_effective_speed(); //Raw speed + stage multipliers

        // Ability modifiers
        let ability_multiplier = match pokemon.get_effective_ability() {
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

        eff_speed
    }

    fn get_effective_crit_rate(&self, id: &Battler, attack: &Move) -> u8 {
        let user = self.get_by_id(id).unwrap();

        let mut stage = attack.get_crit_rate().unwrap_or(0);

        stage += match *user.get_effective_ability() {
            Ability::SuperLuck => 1,
            _ => 0
        };

        stage += match user.pokemon.held_item {
            Some(Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw)) => 1,
            Some(Item::Leek) if *user.get_effective_species() == Species::Farfetchd => 2,
            Some(Item::LuckyPunch) if *user.get_effective_species() == Species::Chansey => 2,
            _ => 0
        };

        if user.battle_data.focused {
            stage += 2;
        }

        stage
    }

    /// Gets the factor of accuracy for a user hitting the defender with the move. This is, essentially,
    /// the probability (out of 100) of a hit landing.
    /// Takes into account:
    /// * User Accuracy and Target Evasion
    /// * Abilities (User and Target)
    /// * Held Items (User and Target)
    /// * Allied Pokemon with Victory Star (doesn't stack if you send out two Victini)
    /// Documentation is vague on what part of the equation the modifiers are applied to. Some moves
    /// affect the accuracy of the move, while others affect the accuracy of the Pokemon
    fn get_accuracy_factor(&self, user_id: &Battler, attack: &Move, defender_id: &Battler) -> f64 {
        let user = self.get_by_id(user_id).unwrap();
        let defender = self.get_by_id(defender_id).unwrap();
        let move_data = attack.data();

        let raw_accuracy =
            //Clamping is unnecessary, since it is handled in this method
            core::determine_accuracy_stat_multiplier(user.battle_data.accuracy_stage - defender.battle_data.evasion_stage);

        let mut field_modifier = match self.field.weather {
            Some(Weather::Fog) => 3f64 / 5f64,
            _ => 1.0
        };
        if self.field.gravity > 0 {
            field_modifier *= 5f64 / 3f64
        }
        match self.get_ally(&user_id) {
            Some(b) if *b.get_effective_ability() == Ability::VictoryStar => field_modifier *= 1.1,
            _ => {}
        }

        let user_ability_modifier = match user.get_effective_ability() {
            Ability::CompoundEyes => 1.3,
            Ability::VictoryStar => 1.1,
            Ability::Hustle if move_data.damage_type == DamageType::Physical => 0.8,
            _ => {
                match self.get_ally(&user_id) {
                    Some(b) if *b.get_effective_ability() == Ability::VictoryStar => 1.1,
                    _ => 1.0
                }
            }
        };

        let user_item_modifier = match user.pokemon.held_item {
            Some(Item::WideLens) => 1.1,
            Some(Item::ZoomLens) if defender.battle_data.move_used_this_turn.is_some() => 1.2,
            _ => 1.0
        };

        let defender_ability_modifier = match defender.get_effective_ability() {
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

    /// Gets the effectiveness of am move, taking into account all things that could modify it
    /// This takes into account:
    /// * Potentially swapped or modified types
    /// TODO: Abilities + Items that modify effectiveness
    fn get_effective_move_effectiveness(&self, attack: &Move, defender_id: &Battler) -> Effectiveness {
        let defender = self.get_by_id(defender_id).unwrap();
        let move_data = attack.data();

        let effective_type = defender.get_effective_type();

        effective_type.defending_against(&move_data._type)
    }

    fn apply_damage(&mut self, to_hurt: &Battler, hp_drop: u16) -> (u16, u16, bool) {
        let defender_data = self.get_by_id(to_hurt).unwrap();

        if defender_data.battle_data.substituted > 0 {
            let target_current_hp = defender_data.battle_data.substituted;
            let target_end_hp = target_current_hp.saturating_sub(hp_drop);
            self.do_by_id(to_hurt, |_, data| data.substituted = target_end_hp);

            (target_current_hp, target_end_hp, true)
        } else {
            let target_current_hp = defender_data.pokemon.current_hp;
            let target_end_hp = target_current_hp.saturating_sub(hp_drop);
            self.do_by_id(to_hurt, |pkmn, _| pkmn.current_hp = target_end_hp);

            (target_current_hp, target_end_hp, false)
        }
    }

    /// Pushes this Turn to the Turn Record, to signal it is complete and permanent.
    /// Intentionally eats the passed-in turn...it belongs to the battlefield now
    pub fn finish_turn(&mut self, turn: Turn) {
        self.turn_record.push(turn);
    }
}

#[derive(Default, Debug)]
/// Really, anything that needs to be tracked during the course of the battle
/// When the pokemon is switched out, everything here is reset to defaults
pub struct BattleData {
    /// The number of turns this Pokemon has been in battle
    turn_count: u8,
    /// The last move that this Pokemon used
    move_used_this_turn: Option<Move>,
    /// If present, contains the amount of damage this Pokemon encountered last
    last_damager: Option<(Battler, Move, u16)>,

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
    transformed: Option<TransformData>,
    /// This Pokemon is focused, increasing crit ratio
    focused: bool,

    /// If true, this Pokemon had a held item + subsequently lost it
    lost_held_item: bool,
    /// If present, this Pokemon has this ability instead of its usual one
    temp_ability: Option<Ability>,
    /// If present, this Pokemon has this type instead of its usual one
    temp_type: Option<PokemonType>,
    /// If true, this Pokemon has been Power Tricked, and its Attack and Defense are swapped
    power_trick: bool
}
impl BattleData {
    pub fn is_confused(&self) -> bool {
        self.confused > 0
    }

    pub fn get_net_stages(&self, stat:BattleStat, stages: i8) -> i8 {
        let clamp = |v: i8| v.clamp(-6, 6);

        let var = match stat {
            BattleStat::Attack => &self.attack_stage,
            BattleStat::Defense => &self.defense_stage,
            BattleStat::SpecialAttack => &self.special_attack_stage,
            BattleStat::SpecialDefense => &self.special_defense_stage,
            BattleStat::Speed => &self.speed_stage,
            BattleStat::Accuracy => &self.accuracy_stage,
            BattleStat::Evasion => &self.evasion_stage
        };

        let before = *var;
        let after = clamp(before + stages);
        after - before
    }

    pub fn add_stages(&mut self, stat: BattleStat, stages: i8) {
        let clamp = |v: i8| v.clamp(-6, 6);

        let var = match stat {
            BattleStat::Attack => &mut self.attack_stage,
            BattleStat::Defense => &mut self.defense_stage,
            BattleStat::SpecialAttack => &mut self.special_attack_stage,
            BattleStat::SpecialDefense => &mut self.special_defense_stage,
            BattleStat::Speed => &mut self.speed_stage,
            BattleStat::Accuracy => &mut self.accuracy_stage,
            BattleStat::Evasion => &mut self.evasion_stage
        };

        *var = clamp(*var + stages);
    }

    pub fn end_of_turn(&mut self) {
        self.turn_count += 1;
        self.last_damager = None;
        self.move_used_this_turn = None;
    }
}

/// Identifier of a member on the field
#[derive(Debug, Clone)]
pub struct Battler(bool, u8);

#[derive(Debug)]
struct TransformData {
    species: Species,
    ability: AbilitySlot,
    attack: StatSlot,
    defense: StatSlot,
    special_attack: StatSlot,
    special_defense: StatSlot,
    speed: StatSlot,
    move_1: Option<MoveSlot>,
    move_2: Option<MoveSlot>,
    move_3: Option<MoveSlot>,
    move_4: Option<MoveSlot>
}
impl From<Pokemon> for TransformData {
    fn from(p: Pokemon) -> Self {
        TransformData {
            species: p.species,
            ability: p.ability,
            attack: p.attack,
            defense: p.defense,
            special_attack: p.special_attack,
            special_defense: p.special_defense,
            speed: p.speed,
            move_1: TransformData::transform_move(p.move_1),
            move_2: TransformData::transform_move(p.move_2),
            move_3: TransformData::transform_move(p.move_3),
            move_4: TransformData::transform_move(p.move_4)
        }
    }
}
impl TransformData {
    fn transform_move(slot: Option<MoveSlot>) -> Option<MoveSlot> {
        match slot {
            None => None,
            Some(m) => Some(m.copy_for_transform())
        }
    }

    fn get_ability(&self) -> &Ability {
        let data = self.species.data();
        match self.ability {
            AbilitySlot::SlotOne => {
                match &data.ability {
                    PokemonAbility::One(a) | PokemonAbility::Two(a, _) => a
                }
            },
            AbilitySlot::SlotTwo => {
                match &data.ability {
                    PokemonAbility::One(a) | PokemonAbility::Two(_, a) => a
                }
            },
            AbilitySlot::Hidden => match &data.hidden_ability {
                None => match &data.ability {
                    PokemonAbility::One(a) | PokemonAbility::Two(a, _) => a
                },
                Some(a) => a
            }
        }
    }
}

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

// Turn Recording (for use by UIs)
/// One of the usable actions that can be taken in a turn
#[derive(Debug)]
pub enum TurnAction {
    Attack(Move, Battler),
    Swap(u8),
    UseItem(Item),
    Flee
}
impl TurnAction{
    /// Get the priority of this move
    /// 0 is default. >0 means it will happen sooner, and <0 means it will happen later
    fn get_priority(&self) -> i8 {
        match self {
            TurnAction::Attack(m, _) => m.data().priority,
            TurnAction::Swap(_) => 10,
            TurnAction::UseItem(_) => 10,
            TurnAction::Flee => 10
        }
    }
}

/// Represents the actions taken during one turn of battle
#[derive(Debug)]
pub struct Turn {
    /// All side effects that occur at the start of the turn
    pub start_of_turn: Vec<ActionSideEffects>,
    /// All actions that take place during the turn, in order
    pub actions: Vec<ActionRecord>,
    /// All side effects that occur at the end of the turn
    pub end_of_turn: Vec<ActionSideEffects>
}
impl Turn {
    pub fn new() -> Turn {
        Turn {
            start_of_turn: Vec::new(),
            actions: Vec::new(),
            end_of_turn: Vec::new()
        }
    }
}

/// An action, and all consequences that occurred because of it
#[derive(Debug)]
pub struct ActionRecord {
    pub user: Battler,
    pub action: TurnAction,
    pub side_effects: Vec<ActionSideEffects>
}

/// The cause of some particular action's side effect
#[derive(Debug)]
pub enum Cause {
    /// This is just what normally happens
    Natural,
    /// A battler's ability caused the side effect
    Ability(Battler),
    /// A previously used move caused the side effect
    Move(Battler, Move),
    /// The side effect was the cause of a user's ailment
    Ailment(NonVolatileBattleAilment),
    /// A battler's held item caused the side effect
    HeldItem(Battler, Item),
    /// One cause was about to occur, but another one overwrote it
    Overwrite{
        initial: Box<Cause>,
        overwriter: Box<Cause>
    }
}
impl Cause {
    pub fn of_ability(battler: &Battler) -> Cause {
        Cause::Ability(battler.clone())
    }

    pub fn overwrite(self, cause: Cause) -> Cause {
        Cause::Overwrite {
            initial: Box::from(self),
            overwriter: Box::from(cause)
        }
    }
}

/// Possible consequences of an Action
/// Plan is to use these to determine which text boxes to say.
#[derive(Debug)]
pub enum ActionSideEffects {
    DirectDamage {
        start_hp: u16,
        end_hp: u16,
        critical_hit: bool,
        effectiveness: Effectiveness
    },
    Missed,
    OneHitKnockout,
    NoEffect,
    ReceivedNonVolatileStatus {
        status: NonVolatileBattleAilment,
        cause: Cause
    },
    IndirectDamage {
        end_hp: u8,
        cause: Cause
    },
    NoTarget
}

impl <T> Battlefield<T> where T: BattleTypeTrait {
    pub fn do_attack(&mut self, user: Battler, attack: Move, defender: Battler, is_multi: bool) -> Vec<ActionSideEffects> {
        let attacker_data = self.get_by_id(&user).unwrap();
        let attack_data = attack.data();

        // Step 1: Ensure the target exists
        let defender_data = self.get_by_id(&defender);
        if defender_data.is_none() {
            return vec![ActionSideEffects::NoTarget];
        }
        let defender_data = defender_data.unwrap();
        let effective_attack_type = if *attacker_data.get_effective_ability() == Ability::Normalize {
            Type::Normal
        } else {
            attack_data._type
        };

        // Step 2: Accuracy Check
        let accuracy_succeeds =
            match attack_data.accuracy {
                Accuracy::AlwaysHits => true,
                Accuracy::Percentage(bp) => {
                    if *attacker_data.get_effective_ability() == Ability::NoGuard || *defender_data.get_effective_ability() == Ability::NoGuard {
                        true
                    } else if *defender_data.get_effective_ability() == Ability::LightningRod && effective_attack_type == Type::Electric {
                        true
                    } else if *defender_data.get_effective_ability() == Ability::StormDrain && effective_attack_type == Type::Water {
                        true
                    } else {
                        let accuracy_factor = self.get_accuracy_factor(&user, &attack, &defender) * f64::from(bp);
                        rand::thread_rng().gen_range(0f64..=100f64) < accuracy_factor
                    }
                }
                Accuracy::Variable => {
                    match attack {
                        Move::Fissure | Move::Guillotine | Move::HornDrill | Move::SheerCold => {
                            if attacker_data.pokemon.level < defender_data.pokemon.level {
                                return vec![ActionSideEffects::NoEffect]
                            } else if *attacker_data.get_effective_ability() == Ability::NoGuard || *defender_data.get_effective_ability() == Ability::NoGuard {
                                true
                            } else {
                                let accuracy_factor = 30 + (attacker_data.pokemon.level - defender_data.pokemon.level);
                                rand::thread_rng().gen_range(0u8..=100u8) < accuracy_factor
                            }
                        },
                        _ => panic!("Unknown Move with Variable accuracy.")
                    }
                }
            };

        // Step 3: Effectiveness check
        let mold_breaker = attacker_data.is_mold_breaker();
        let effectiveness = defender_data.get_effective_type().defending_against(&effective_attack_type);
        // 3a: Defender Ability + Held Items
        let (effectiveness, defender_cause) =
            match *defender_data.get_effective_ability() {
                Ability::Levitate if attack_data._type == Type::Ground => {
                    let cause = Cause::of_ability(&defender);
                    if mold_breaker {
                        (effectiveness, cause.overwrite(Cause::of_ability(&user)))
                    } else {
                        (Effectiveness::Immune, cause)
                    }
                },
                Ability::WonderGuard => {
                    match effectiveness {
                        Effectiveness::Effect(e) if e > 0 => (effectiveness, Cause::Natural),
                        _ => {
                            if mold_breaker {
                                (effectiveness, Cause::of_ability(&defender).overwrite(Cause::of_ability(&user)))
                            } else {
                                (Effectiveness::Immune, Cause::of_ability(&defender))
                            }
                        }
                    }
                },
                Ability::Soundproof if attack.is_sound_based() => {
                    let cause = Cause::of_ability(&defender);
                    if mold_breaker {
                        (effectiveness, cause.overwrite(Cause::of_ability(&user)))
                    } else {
                        (Effectiveness::Immune, cause)
                    }
                },
                Ability::Overcoat if attack.is_powder() => {
                    let cause = Cause::of_ability(&defender);
                    if mold_breaker {
                        (effectiveness, cause.overwrite(Cause::of_ability(&user)))
                    } else {
                        (Effectiveness::Immune, cause)
                    }
                }
                _ => (effectiveness, Cause::Natural)
            };

        // 3b: Attacker Ability + Held Items
        let (effectiveness, attacker_cause) = match *attacker_data.get_effective_ability() {
            Ability::Scrappy if effective_attack_type == Type::Normal || effective_attack_type == Type::Fighting => {
                match effectiveness {
                    Effectiveness::Immune => (Effectiveness::NORMAL, Cause::of_ability(&user)),
                    e => (e, Cause::Natural)
                }
            },
            Ability::TintedLens => {
                match effectiveness {
                    Effectiveness::Effect(a) if a < 0 => (Effectiveness::Effect(a + 1), Cause::of_ability(&user)),
                    a => (a, Cause::Natural)
                }
            },
            _ => (effectiveness, Cause::Natural)
        };

        // Step 4: Suppression check

        vec![]
    }

    fn get_effectiveness_and_cause_defender(&self) -> (Effectiveness, Cause) {

    }
}