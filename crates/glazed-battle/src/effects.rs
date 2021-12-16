use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{Move, MoveData};
use glazed_data::constants::Species;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::Pokemon;
use glazed_data::types::{PokemonType, Type};
use crate::{ActionSideEffects, BattleData, Battlefield, Battler, BattleTypeTrait, Cause, Weather};
use crate::TurnAction::Attack;

/// Simple utility to drain a vector into another one
fn copy_all<T>(vec: &mut Vec<T>, vec_to_add: Vec<T>) {
    for elem in vec_to_add {
        vec.push(elem);
    }
}

/// Get the effective ability of this Pokemon. Takes Transform and temporary Ability changes into effect
fn get_effective_ability(pkmn: &Pokemon, battle_data: &BattleData) -> Ability {
    battle_data.temp_ability.unwrap_or_else(|| {
        match &battle_data.transformed {
            None => pkmn.get_ability(),
            Some(t) => *t.get_ability()
        }
    })
}

/// Get the effective type(s) of this Pokemon. Takes Transform and temporary Type changes into effect
fn get_effective_type(pkmn: &Pokemon, battle_data: &BattleData) -> PokemonType {
    battle_data.temp_type.unwrap_or_else(|| {
        match &battle_data.transformed {
            Some(t) => t.species.data()._type,
            None => pkmn.species.data()._type,
        }
    })
}

/// Get the effective species of this Pokemon. Takes Transform into effect
fn get_effective_species(pkmn: &Pokemon, battle_data: &BattleData) -> Species {
    match &battle_data.transformed {
        None => pkmn.species,
        Some(t) => t.species
    }
}

fn get_raw_critical_hit(pkmn: &Pokemon, species: Species, ability: Ability) -> u8 {
    let mut value = 0;
    value += match pkmn.held_item {
        Some(Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw)) => 1,
        Some(Item::ScopeLens) => 1,
        Some(Item::Leek) if species == Species::Farfetchd => 2,
        Some(Item::LuckyPunch) if species == Species::Chansey => 2,
        _ => 0
    };
    value += match ability {
        Ability::SuperLuck => 1,
        _ => 0
    };
    value
}

impl <T> Battlefield<T> where T: BattleTypeTrait {
    pub fn do_damage_from_base_power(&mut self, attacker: Battler, attack: Move, defender: Battler) -> Vec<ActionSideEffects> {
        let attacker_pokemon = self.get_pokemon_by_id(&attacker).unwrap();
        let attacker_data = self.get_battle_data(&attacker);
        let attacker_ability = get_effective_ability(attacker_pokemon, attacker_data);
        let attacker_type = get_effective_type(attacker_pokemon, attacker_data);
        let attacker_species = get_effective_species(attacker_pokemon, attacker_data);

        let defender_pokemon = self.get_pokemon_by_id(&defender).unwrap();
        let defender_data = self.get_battle_data(&defender);
        let defender_ability = get_effective_ability(defender_pokemon, defender_data);
        let defender_type = get_effective_type(defender_pokemon, defender_data);
        let defender_species = get_effective_species(defender_pokemon, defender_data);

        let move_data = attack.data();

        // Effectiveness Calculation
        let effective_move_type = match attacker_ability {
            Ability::Normalize if attack != Move::HiddenPower ||
                attack != Move::WeatherBall ||
                attack != Move::NaturalGift ||
                attack != Move::Judgment ||
                attack != Move::TechnoBlast => Type::Normal,
            _ => {
                match attack {
                    Move::HiddenPower => attacker_pokemon.get_hidden_power_type(),
                    Move::WeatherBall => match self.field.weather {
                        Some(Weather::Sun(_)) => Type::Fire,
                        Some(Weather::Rain(_)) => Type::Water,
                        Some(Weather::Hail(_)) => Type::Ice,
                        Some(Weather::Sandstorm(_)) => Type::Rock,
                        Some(Weather::Fog) | None => Type::Normal
                    },
                    Move::Judgment => match attacker_pokemon.held_item {
                        Some(Item::DracoPlate) => Type::Dragon,
                        Some(Item::DreadPlate) => Type::Dark,
                        Some(Item::EarthPlate) => Type::Ground,
                        Some(Item::FistPlate) => Type::Fighting,
                        Some(Item::FlamePlate) => Type::Fire,
                        Some(Item::IciclePlate) => Type::Ice,
                        Some(Item::InsectPlate) => Type::Bug,
                        Some(Item::Iron) => Type::Steel,
                        Some(Item::MeadowPlate) => Type::Grass,
                        Some(Item::MindPlate) => Type::Psychic,
                        Some(Item::PixiePlate) => Type::Fairy,
                        Some(Item::SkyPlate) => Type::Flying,
                        Some(Item::SplashPlate) => Type::Water,
                        Some(Item::SpookyPlate) => Type::Ghost,
                        Some(Item::StonePlate) => Type::Rock,
                        Some(Item::ToxicPlate) => Type::Poison,
                        Some(Item::ZapPlate) => Type::Electric,
                        _ => Type::Normal
                    },
                    Move::TechnoBlast => match attacker_pokemon.held_item {
                        Some(Item::DouseDrive) => Type::Water,
                        Some(Item::ShockDrive) => Type::Electric,
                        Some(Item::ChillDrive) => Type::Ice,
                        Some(Item::BurnDrive) => Type::Fire,
                        _ => Type::Normal
                    },
                    Move::NaturalGift => {
                        if let Some(Item::Berry(a)) = &attacker_pokemon.held_item {
                            if attacker_ability != Ability::Klutz && self.field.magic_room == 0 && attacker_data.embargo == 0 {
                                a.get_natural_gift_type()
                            } else {
                                return vec![ActionSideEffects::Failed(Cause::Natural)]
                            }
                        } else {
                            return vec![ActionSideEffects::Failed(Cause::Natural)]
                        }
                    }
                    _ => move_data._type
                }
            }
        };
        let stab = match attacker_type {
            PokemonType::Single(a) => effective_move_type == a,
            PokemonType::Double(a, b) => effective_move_type == a || effective_move_type == b
        };

        // Critical Hit Calculation
        let crit_stages = get_raw_critical_hit(attacker_pokemon, attacker_species, attacker_ability) + move_data.crit_rate.unwrap_or(0);
        let crit = match crit_stages {
            0 => rand::thread_rng().gen_bool(0.0625),
            1 => rand::thread_rng().gen_bool(0.125),
            2 => rand::thread_rng().gen_bool(0.5),
            _ => true
        };

        vec![]
    }
}

//region oldstuff
// /// Gets the effective attack stat of a Pokemon on the field
// /// This takes into account:
// /// * Raw attack stat (or defense stat, if afflicted with Power Trick)
// /// * Attack stage
// /// * Burn (+ Immunity to burn attack drop if ability is Guts
// /// * Abilities
// /// * Items
// /// * Transform
// fn get_effective_attack(&self, id: &Battler) -> f64 {
//     let user = self.get_by_id(id).unwrap();
//     let atk = f64::from(user.get_effective_attack()); //Raw attack + stage multipliers + Power Trick
//
//     let ability_multiplier = match user.get_effective_ability() {
//         Ability::FlowerGift if self.field.is_sunny() => 1.5,
//         Ability::Guts if user.has_status_condition() => 1.5,
//         Ability::HugePower => 2.0,
//         Ability::Hustle => 1.5,
//         Ability::PurePower => 2.0,
//         Ability::Defeatist if user.is_half_health_or_worse() => 0.5,
//         Ability::SlowStart if user.battle_data.turn_count < 5 => 0.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match user.pokemon.held_item {
//         Some(Item::ChoiceBand) => 1.5,
//         Some(Item::LightBall) if *user.get_effective_species() == Species::Pikachu => 2.0,
//         Some(Item::ThickClub) if *user.get_effective_species() == Species::Marowak => 2.0,
//         _ => 1.0
//     };
//
//     atk * ability_multiplier * item_multiplier
// }
//
// /// Gets the effective defense stat of a Pokemon on the field
// /// This takes into account:
// /// * Raw defense stat (or attack stat, if afflicted with Power Trick)
// /// * Defense stage
// /// * Abilities
// /// * Items
// /// * Transform
// fn get_effective_defense(&self, id: &Battler) -> f64 {
//     let user = self.get_by_id(id).unwrap();
//     let def = f64::from(user.get_effective_defense()); //Raw defense + stage multipliers + Power Trick
//
//     let ability_multiplier = match user.get_effective_ability() {
//         Ability::MarvelScale if user.has_status_condition() => 1.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match user.pokemon.held_item {
//         Some(Item::Eviolite) if !user.get_effective_species().is_fully_evolved() => 1.5,
//         Some(Item::MetalPowder) if user.pokemon.species == Species::Ditto && user.battle_data.transformed.is_none() => 2.0,
//         _ => 1.0
//     };
//
//     def * ability_multiplier * item_multiplier
// }
//
// /// Gets the effective special attack stat of a Pokemon on the field
// /// This takes into account:
// /// * Raw special attack stat
// /// * Special Attack stage
// /// * Abilities
// /// * Items
// /// * Transform
// /// * Plus or Minus, if the Ally also has Plus or Minus (no restrictions on two Plus or two Minus)
// fn get_effective_special_attack(&self, id: &Battler) -> f64 {
//     let user = self.get_by_id(id).unwrap();
//     let spa = f64::from(user.get_effective_special_attack()); //Raw SpA + stage multipliers
//
//     let ability_multiplier = match user.get_effective_ability() {
//         Ability::Plus | Ability::Minus => {
//             match self.get_ally(&id) {
//                 Some(p) if *p.get_effective_ability() == Ability::Plus
//                     || *p.get_effective_ability() == Ability::Minus => 1.5,
//                 _ => 1.0
//             }
//         }
//         Ability::SolarPower if self.field.is_sunny() => 1.5,
//         Ability::Defeatist if user.is_half_health_or_worse() => 0.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match user.pokemon.held_item {
//         Some(Item::ChoiceSpecs) => 1.5,
//         Some(Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaTooth))
//         if *user.get_effective_species() == Species::Clamperl => 2.0,
//         Some(Item::LightBall) if *user.get_effective_species() == Species::Pikachu => 2.0,
//         Some(Item::SoulDew) if *user.get_effective_species() == Species::Latias || *user.get_effective_species() == Species::Latios => 1.5,
//         _ => 1.0
//     };
//
//     spa * ability_multiplier * item_multiplier
// }
//
// /// Gets the effective special defense stat of a Pokemon on the field
// /// This takes into account:
// /// * Raw special defense stat
// /// * Special Defense stage
// /// * Abilities
// /// * Items
// /// * Transform
// fn get_effective_special_defense(&self, id: &Battler) -> f64 {
//     let user = self.get_by_id(id).unwrap();
//     let spd = f64::from(user.get_effective_special_defense());
//
//     let ability_multiplier = match user.get_effective_ability() {
//         Ability::FlowerGift if self.field.is_sunny() => 1.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match user.pokemon.held_item {
//         Some(Item::AssaultVest) => 1.5,
//         Some(Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaScale))
//         if *user.get_effective_species() == Species::Clamperl => 2.0,
//         Some(Item::Eviolite) if !user.get_effective_species().is_fully_evolved() => 1.5,
//         Some(Item::MetalPowder) if user.pokemon.species == Species::Ditto => 1.5,
//         Some(Item::SoulDew) if *user.get_effective_species() == Species::Latias || *user.get_effective_species() == Species::Latios => 1.5,
//         _ => 1.0
//     };
//
//     spd * ability_multiplier * item_multiplier
// }
//
// /// Get the current effective speed of a specific Pokemon on a specific side of the field
// /// This takes into account:
// /// * Raw speed stat of the Pokemon
// /// * Speed stage
// /// * Abilities, if applicable to the current state of the field
// /// * Items, if applicable to the current state of the field
// /// * Other statuses, such as paralysis or Tailwind
// /// * Transform
// fn get_effective_speed(&self, id: &Battler) -> f64 {
//     let pokemon = self.get_by_id(id).unwrap();
//     let side = self.get_side_by_id(id);
//     let speed = pokemon.get_effective_speed(); //Raw speed + stage multipliers
//
//     // Ability modifiers
//     let ability_multiplier = match pokemon.get_effective_ability() {
//         Ability::Chlorophyll if self.field.is_sunny() => 2.0,
//         Ability::SandRush if self.field.is_sandstorm() => 2.0,
//         Ability::SwiftSwim if self.field.is_rain() => 2.0,
//         Ability::SlushRush if self.field.is_hail() => 2.0,
//         Ability::QuickFeet if pokemon.has_status_condition() => 1.5,
//         Ability::Unburden if pokemon.battle_data.lost_held_item => 2.0,
//         Ability::SlowStart if pokemon.battle_data.turn_count < 5 => 0.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match pokemon.pokemon.held_item {
//         Some(Item::ChoiceScarf) => 1.5,
//         Some(Item::QuickPowder) if pokemon.pokemon.species == Species::Ditto => 2.0,
//         _ => 1.0
//     };
//
//     let mut eff_speed = f64::from(speed) * ability_multiplier * item_multiplier;
//     if side.tailwind > 0 {
//         eff_speed *= 2.0;
//     }
//
//     if pokemon.pokemon.status.paralysis {
//         eff_speed *= 0.5;
//     }
//
//     eff_speed
// }
//
// fn get_effective_crit_rate(&self, id: &Battler, attack: &Move) -> u8 {
//     let user = self.get_by_id(id).unwrap();
//
//     let mut stage = attack.get_crit_rate().unwrap_or(0);
//
//     stage += match *user.get_effective_ability() {
//         Ability::SuperLuck => 1,
//         _ => 0
//     };
//
//     stage += match user.pokemon.held_item {
//         Some(Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw)) => 1,
//         Some(Item::Leek) if *user.get_effective_species() == Species::Farfetchd => 2,
//         Some(Item::LuckyPunch) if *user.get_effective_species() == Species::Chansey => 2,
//         _ => 0
//     };
//
//     if user.battle_data.focused {
//         stage += 2;
//     }
//
//     stage
// }
//
// /// Gets the factor of accuracy for a user hitting the defender with the move. This is, essentially,
// /// the probability (out of 100) of a hit landing.
// /// Takes into account:
// /// * User Accuracy and Target Evasion
// /// * Abilities (User and Target)
// /// * Held Items (User and Target)
// /// * Allied Pokemon with Victory Star (doesn't stack if you send out two Victini)
// /// Documentation is vague on what part of the equation the modifiers are applied to. Some moves
// /// affect the accuracy of the move, while others affect the accuracy of the Pokemon
// fn get_accuracy_factor(&self, user_id: &Battler, attack: &Move, defender_id: &Battler) -> f64 {
//     let user = self.get_by_id(user_id).unwrap();
//     let defender = self.get_by_id(defender_id).unwrap();
//     let move_data = attack.data();
//
//     let raw_accuracy =
//         //Clamping is unnecessary, since it is handled in this method
//         core::determine_accuracy_stat_multiplier(user.battle_data.accuracy_stage - defender.battle_data.evasion_stage);
//
//     let mut field_modifier = match self.field.weather {
//         Some(Weather::Fog) => 3f64 / 5f64,
//         _ => 1.0
//     };
//     if self.field.gravity > 0 {
//         field_modifier *= 5f64 / 3f64
//     }
//     match self.get_ally(&user_id) {
//         Some(b) if *b.get_effective_ability() == Ability::VictoryStar => field_modifier *= 1.1,
//         _ => {}
//     }
//
//     let user_ability_modifier = match user.get_effective_ability() {
//         Ability::CompoundEyes => 1.3,
//         Ability::VictoryStar => 1.1,
//         Ability::Hustle if move_data.damage_type == DamageType::Physical => 0.8,
//         _ => {
//             match self.get_ally(&user_id) {
//                 Some(b) if *b.get_effective_ability() == Ability::VictoryStar => 1.1,
//                 _ => 1.0
//             }
//         }
//     };
//
//     let user_item_modifier = match user.pokemon.held_item {
//         Some(Item::WideLens) => 1.1,
//         Some(Item::ZoomLens) if defender.battle_data.move_used_this_turn.is_some() => 1.2,
//         _ => 1.0
//     };
//
//     let defender_ability_modifier = match defender.get_effective_ability() {
//         Ability::WonderSkin if move_data.damage_type == DamageType::Status => 0.5,
//         Ability::SandVeil if self.field.is_sandstorm() => 4f64 / 5f64,
//         Ability::SnowCloak if self.field.is_hail() => 4f64 / 5f64,
//         Ability::TangledFeet if defender.battle_data.is_confused() => 0.5,
//         _ => 1.0
//     };
//
//     let defender_item_modifier = match defender.pokemon.held_item {
//         Some(Item::BrightPowder) | Some(Item::Incense(Incense::LaxIncense)) => 0.9,
//         _ => 1.0
//     };
//
//     raw_accuracy * field_modifier * user_ability_modifier * user_item_modifier * defender_ability_modifier * defender_item_modifier
// }
//
// /// Gets the effectiveness of am move, taking into account all things that could modify it
// /// This takes into account:
// /// * Potentially swapped or modified types
// /// TODO: Abilities + Items that modify effectiveness
// fn get_effective_move_effectiveness(&self, attack: &Move, defender_id: &Battler) -> Effectiveness {
//     let defender = self.get_by_id(defender_id).unwrap();
//     let move_data = attack.data();
//
//     let effective_type = defender.get_effective_type();
//
//     effective_type.defending_against(&move_data._type)
// }
//
// fn apply_damage(&mut self, to_hurt: &Battler, hp_drop: u16) -> (u16, u16, bool) {
//     let defender_data = self.get_by_id(to_hurt).unwrap();
//
//     if defender_data.battle_data.substituted > 0 {
//         let target_current_hp = defender_data.battle_data.substituted;
//         let target_end_hp = target_current_hp.saturating_sub(hp_drop);
//         self.do_by_id(to_hurt, |_, data| data.substituted = target_end_hp);
//
//         (target_current_hp, target_end_hp, true)
//     } else {
//         let target_current_hp = defender_data.pokemon.current_hp;
//         let target_end_hp = target_current_hp.saturating_sub(hp_drop);
//         self.do_by_id(to_hurt, |pkmn, _| pkmn.current_hp = target_end_hp);
//
//         (target_current_hp, target_end_hp, false)
//     }
// }
//endregion

//region oldstuff2
// struct BattlePokemon<'a>(&'a Pokemon, &'a BattleData);
// impl <'a> BattlePokemon<'a> {
//     /// Get the species of this Pokemon. Takes Transform into account
//     fn get_effective_species(&self) -> &Species {
//         match &self.1.transformed {
//             None => &self.0.species,
//             Some(t) => &t.species
//         }
//     }
//
//     /// Get the effective attack of this Pokemon. Takes Transform, Power Trick, and Attack stage into account
//     fn get_effective_attack(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.attack_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(if self.1.power_trick {self.0.defense.value} else {self.0.attack.value}),
//             Some(t) => f64::from(if self.1.power_trick {t.defense.value} else {t.attack.value})
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Get the effective defense of this Pokemon. Takes Transform, Power Trick, and Defense stage into account
//     fn get_effective_defense(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.defense_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(if self.1.power_trick {self.0.attack.value} else {self.0.defense.value}),
//             Some(t) => f64::from(if self.1.power_trick {t.attack.value} else {t.defense.value})
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Get the effective special attack of this Pokemon. Takes Transform and Sp. Attack stage into account
//     fn get_effective_special_attack(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.special_attack_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(self.0.special_attack.value),
//             Some(t) => f64::from(t.special_attack.value)
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Get the effective special defense of this Pokemon. Takes Transform and Sp. Defense stage into account
//     fn get_effective_special_defense(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.special_defense_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(self.0.special_defense.value),
//             Some(t) => f64::from(t.special_defense.value)
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Get the effective speed of this Pokemon. Takes Transform and Speed stage into account
//     fn get_effective_speed(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.speed_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(self.0.speed.value),
//             Some(t) => f64::from(t.speed.value)
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Check if this Pokemon has a status condition (may be expanded to handle volatile status ailments)
//     fn has_status_condition(&self) -> bool {
//         self.0.status.has_status_condition()
//     }
//
//     /// Get the effective ability of this Pokemon. Takes Transform and temporary Ability changes into effect
//     fn get_effective_ability(&self) -> &Ability {
//         self.1.temp_ability.as_ref().unwrap_or_else(|| {
//             match &self.1.transformed {
//                 None => self.0.get_ability(),
//                 Some(t) => t.get_ability()
//             }
//         })
//     }
//
//     /// Get the effective type(s) of this Pokemon. Taks Transform and temporary Type changes into effect
//     fn get_effective_type(&self) -> &PokemonType {
//         self.1.temp_type.as_ref().unwrap_or_else(|| {
//             match &self.1.transformed {
//                 None => &self.0.species.data()._type,
//                 Some(t) => &t.species.data()._type
//             }
//         })
//     }
//
//     fn is_half_health_or_worse(&self) -> bool {
//         let current = u32::from(self.0.current_hp);
//         let max = u32::from(self.0.hp.value);
//         current * 2u32 <= max
//     }
//
//     fn is_quarter_health_or_worse(&self) -> bool {
//         let current = u32::from(self.0.current_hp);
//         let max = u32::from(self.0.hp.value);
//         current * 4u32 <= max
//     }
// }
//endregion