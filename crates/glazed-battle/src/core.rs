use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, DamageType, Move, MoveData, MultiHitFlavor, Power};
use glazed_data::constants::Species;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::Pokemon;
use glazed_data::types::{Effectiveness, PokemonType, Type};
use crate::{ActionSideEffects, BattleData, Cause, Field, Weather};
use crate::effects::{BattleBundle, MAX_STAGE, MIN_STAGE};

/// Simple utility to drain a vector into another one
pub fn copy_all<T>(vec: &mut Vec<T>, vec_to_add: Vec<T>) {
    for elem in vec_to_add {
        vec.push(elem);
    }
}

pub(crate) struct MoveContext {
    pub attack: Move,
    pub data: &'static MoveData,
    pub base_power: u16
}
impl From<Move> for MoveContext {
    fn from(attack: Move) -> Self {
        let data = attack.data();
        let base_power = match data.power {
            Power::Base(base) | Power::BaseWithRecoil(base, _) | Power::BaseWithMercy(base) |
            Power::MultiHit(MultiHitFlavor::Variable(base)) | Power::MultiHit(MultiHitFlavor::Fixed(_, base)) => u16::from(base),
            _ => panic!("Can't derive base power, must be specified")
        };
        MoveContext {
            attack,
            data,
            base_power
        }
    }
}
// Run the raw damage calculations, without considering an exact move
// Exact formula: base_damage * weather * crit * random * STAB * Type * Burn
// Any other multipliers have to be done in addition.
pub(crate) fn calculate_raw_damage_from_base(defender_bundle: BattleBundle, attacker_bundle: BattleBundle, base: u16, _type: Option<Type>, damage_type: DamageType, crit_rate: u8, field: &Field) -> ActionSideEffects {
    let (attacker, attacker_pokemon, attacker_data, attacker_ability) = attacker_bundle;
    let attacker_type = get_effective_type(attacker_pokemon, attacker_data);
    let attacker_species = get_effective_species(attacker_pokemon, attacker_data);

    let (defender, defender_pokemon, defender_data, _) = defender_bundle;
    let defender_type = get_effective_type(defender_pokemon, defender_data);

    // Effectiveness Calculation
    let effective_move_type = _type;
    let effectiveness = match effective_move_type {
        Some(t) => defender_type.defending_against(&t),
        None => Effectiveness::NORMAL
    };

    if let Effectiveness::Immune = effectiveness {
        return ActionSideEffects::NoEffect(Cause::Natural)
    }

    let stab = match effective_move_type {
        Some(t) => attacker_type.is_stab(&t),
        None => false
    };

    // Critical Hit Calculation
    let crit_stages = get_raw_critical_hit(attacker_pokemon, attacker_species, attacker_ability) + crit_rate;
    let crit = match crit_stages {
        0 => rand::thread_rng().gen_bool(0.0625),
        1 => rand::thread_rng().gen_bool(0.125),
        2 => rand::thread_rng().gen_bool(0.5),
        _ => true
    };

    let (ea, ed) = match damage_type {
        DamageType::Physical => (
            get_effective_stat(attacker_pokemon, attacker_data, BattleStat::Attack),
            get_effective_stat(defender_pokemon, defender_data, BattleStat::Defense)),
        DamageType::Special => (
            get_effective_stat(attacker_pokemon, attacker_data, BattleStat::SpecialAttack),
            get_effective_stat(defender_pokemon, defender_data, BattleStat::SpecialDefense)),
        DamageType::Status => panic!("Status move should not do direct damage??")
    };

    let base_damage_num = (((2 * u32::from(attacker_pokemon.level)) / 5) + 2) * u32::from(base) * u32::from(ea);
    let base_damage_denom = u32::from(ed) * 50;
    let mut damage = base_damage_num / base_damage_denom + 2;

    // * weather
    if let Some(t) = effective_move_type {
        if t == Type::Water {
            if field.is_rain() {
                damage += damage / 2;
            } else if field.is_sunny() {
                damage /= 2;
            }
        } else if t == Type::Fire {
            if field.is_rain() {
                damage /= 2;
            } else if field.is_sunny() {
                damage += damage / 2;
            }
        }
    }

    // * crit
    if crit {
        damage += damage / 2;
    }

    // * random
    damage = (f64::from(damage) * rand::thread_rng().gen_range(0.85..=1.0)) as u32;

    // * stab
    if stab {
        if attacker_ability == Ability::Adaptability {
            damage *= 2;
        } else {
            // This is a fair approximation of 150%, right?
            damage += damage / 2;
        }
    }

    // * type
    if let Effectiveness::Effect(i) = effectiveness {
        if i < 0 {
            damage = damage >> -i;
        } else {
            damage = damage << i;
        }
    }

    // * burn
    if attacker_pokemon.status.burn &&
        attacker_ability != Ability::Guts &&
        damage_type == DamageType::Physical {
        damage /= 2;
    }

    ActionSideEffects::DirectDamage {
        damaged: defender,
        damager: attacker,
        attack: Move::Pound,
        start_hp: defender_pokemon.current_hp,
        end_hp: defender_pokemon.current_hp.saturating_sub(damage as u16),
        hung_on_cause: None,
        critical_hit: crit,
        effectiveness
    }
}

pub fn hangs_on(defender: BattleBundle, attacker: BattleBundle) -> Option<Cause> {
    if defender.1.is_full_health() {
        match defender.3 {
            Ability::Sturdy => {
                let cause = Cause::Ability(defender.0, defender.3);
                if attacker.3 == Ability::MoldBreaker || attacker.3 == Ability::Teravolt || attacker.3 == Ability::Turboblaze {
                    Some(cause.overwrite(Cause::Ability(attacker.0, attacker.3)))
                } else {
                    Some(cause)
                }
            },
            _ => match defender.1.held_item {
                Some(Item::FocusSash) => {
                    Some(Cause::HeldItem(defender.0, Item::FocusSash))
                },
                _ => None
            }
        }
    } else {
        None
    }
}

/// Get the effective ability of this Pokemon. Takes Transform and temporary Ability changes into effect
pub fn get_effective_ability(pkmn: &Pokemon, battle_data: &BattleData) -> Ability {
    battle_data.temp_ability.unwrap_or_else(|| {
        match &battle_data.transformed {
            None => pkmn.get_ability(),
            Some(t) => *t.get_ability()
        }
    })
}

/// Get the effective type(s) of this Pokemon. Takes Transform and temporary Type changes into effect
pub fn get_effective_type(pkmn: &Pokemon, battle_data: &BattleData) -> PokemonType {
    battle_data.temp_type.unwrap_or_else(|| {
        match &battle_data.transformed {
            Some(t) => t.species.data()._type,
            None => pkmn.species.data()._type,
        }
    })
}

/// Get the effective species of this Pokemon. Takes Transform into effect
pub fn get_effective_species(pkmn: &Pokemon, battle_data: &BattleData) -> Species {
    match &battle_data.transformed {
        None => pkmn.species,
        Some(t) => t.species
    }
}

/// Get the effective stat of this Pokemon. Takes Transform and stages into effect
pub fn get_effective_stat(pkmn: &Pokemon, battle_data: &BattleData, stat: BattleStat) -> u16 {
    let base = match stat {
        BattleStat::Attack => battle_data.transformed.as_ref().map(|t| t.attack.value).unwrap_or(pkmn.attack.value),
        BattleStat::Defense => battle_data.transformed.as_ref().map(|t| t.defense.value).unwrap_or(pkmn.defense.value),
        BattleStat::SpecialAttack => battle_data.transformed.as_ref().map(|t| t.special_attack.value).unwrap_or(pkmn.special_attack.value),
        BattleStat::SpecialDefense => battle_data.transformed.as_ref().map(|t| t.special_defense.value).unwrap_or(pkmn.special_defense.value),
        BattleStat::Speed => battle_data.transformed.as_ref().map(|t| t.speed.value).unwrap_or(pkmn.speed.value),
        BattleStat::Accuracy => 1,
        BattleStat::Evasion => 1
    };

    let stage = match stat {
        BattleStat::Attack => battle_data.attack_stage,
        BattleStat::Defense => battle_data.defense_stage,
        BattleStat::SpecialAttack => battle_data.special_attack_stage,
        BattleStat::SpecialDefense => battle_data.special_defense_stage,
        BattleStat::Speed => battle_data.speed_stage,
        BattleStat::Accuracy => 0i8,
        BattleStat::Evasion => 0i8
    };

    match stage {
        i8::MIN..=MIN_STAGE => base * 2u16 / 8u16,
        -5 => base * 2u16 / 7u16,
        -4 => base * 2u16 / 6u16,
        -3 => base * 2u16 / 5u16,
        -2 => base * 2u16 / 4u16,
        -1 => base * 2u16 / 3u16,
        0 => base,
        1 => base * 3u16 / 2u16,
        2 => base * 2u16,
        3 => base * 5u16 / 2u16,
        4 => base * 3u16,
        5 => base * 7u16 / 2u16,
        MAX_STAGE..=i8::MAX => base * 4u16
    }
}

/// Get the raw critical hit stage of this Pokemon. Takes held item and ability into account
pub fn get_raw_critical_hit(pkmn: &Pokemon, species: Species, ability: Ability) -> u8 {
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

/// Get the type of the move used. Takes ability and type-changing moves into account
pub fn get_effective_move_type(attacker: BattleBundle, field: &Field, attack: Move, move_data: &MoveData) -> Type {
    let (_, attacker_pokemon, _, attacker_ability) = attacker;
    match attacker_ability {
        Ability::Normalize if attack != Move::HiddenPower ||
            attack != Move::WeatherBall ||
            attack != Move::NaturalGift ||
            attack != Move::Judgment ||
            attack != Move::TechnoBlast => return Type::Normal,
        _ => {}
    };

    match attack {
        Move::HiddenPower => attacker_pokemon.get_hidden_power_type(),
        Move::WeatherBall => match field.weather {
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
                a.get_natural_gift_type()
            } else {
                move_data._type
            }
        }
        _ => move_data._type
    }
}

/// Get the effective weight of this Pokemon. Takes Ability, held item, and Autotomize into account
pub fn get_effective_weight(pkmn: BattleBundle) -> u16 {
    let (_, species, data, ability) = pkmn;
    let mut weight = data.temp_weight.unwrap_or_else(|| species.species.data().weight);

    if ability == Ability::HeavyMetal {
        weight = weight.saturating_mul(2);
    } else if ability == Ability::LightMetal {
        weight = weight.saturating_div(2);
    }

    if let Some(Item::FloatStone) = species.held_item {
        weight = weight.saturating_div(2);
    }

    if weight == 0 { 1 } else { weight }
}