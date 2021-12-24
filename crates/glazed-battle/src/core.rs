use std::cell::RefMut;
use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, DamageType, Move, MoveData, MultiHitFlavor, Power};
use glazed_data::constants::Species;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::Pokemon;
use glazed_data::types::{Effectiveness, PokemonType, Type};
use crate::{ActionSideEffects, ActivePokemon, BattleData, Cause, Field, Weather};
use crate::effects::{MAX_STAGE, MIN_STAGE};

/// Simple utility to drain a vector into another one
pub fn copy_all<T>(vec: &mut Vec<T>, vec_to_add: Vec<T>) {
    for elem in vec_to_add {
        vec.push(elem);
    }
}

pub struct MoveContext {
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
pub(crate) fn calculate_raw_damage_from_base(attacker: &ActivePokemon, defender: &ActivePokemon, base: u16, _type: Option<Type>, damage_type: DamageType, crit_rate: u8, field: &Field) -> ActionSideEffects {
    // Effectiveness Calculation
    let effectiveness = match _type {
        Some(t) => defender.get_effective_type().defending_against(&t),
        None => Effectiveness::NORMAL
    };

    if let Effectiveness::Immune = effectiveness {
        return ActionSideEffects::NoEffect(Cause::Natural)
    }

    let stab = match _type {
        Some(t) => attacker.get_effective_type().is_stab(&t),
        None => false
    };

    // Critical Hit Calculation
    let crit_stages = attacker.get_raw_critical_hit() + crit_rate;
    let crit = match crit_stages {
        0 => rand::thread_rng().gen_bool(0.0625),
        1 => rand::thread_rng().gen_bool(0.125),
        2 => rand::thread_rng().gen_bool(0.5),
        _ => true
    };

    let (ea, ed) = match damage_type {
        DamageType::Physical => (
            attacker.get_effective_stat(BattleStat::Attack),
            defender.get_effective_stat(BattleStat::Defense)),
        DamageType::Special => (
            attacker.get_effective_stat(BattleStat::SpecialAttack),
            defender.get_effective_stat(BattleStat::SpecialDefense)),
        DamageType::Status => panic!("Status move should not do direct damage??")
    };

    let base_damage_num = (((2 * u32::from(attacker.borrow().level)) / 5) + 2) * u32::from(base) * u32::from(ea);
    let base_damage_denom = u32::from(ed) * 50;
    let mut damage = base_damage_num / base_damage_denom + 2;

    // * weather
    if let Some(t) = _type {
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
        if attacker.get_effective_ability() == Ability::Adaptability {
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
    if attacker.borrow().status.burn &&
        attacker.get_effective_ability() != Ability::Guts &&
        damage_type == DamageType::Physical {
        damage /= 2;
    }

    ActionSideEffects::DirectDamage {
        damaged: defender.id,
        damager: attacker.id,
        attack: Move::Pound,
        start_hp: defender.borrow().current_hp,
        end_hp: defender.borrow().current_hp.saturating_sub(damage as u16),
        hung_on_cause: None,
        critical_hit: crit,
        effectiveness
    }
}

/// Get the type of the move used. Takes ability and type-changing moves into account
pub fn get_effective_move_type<F>(attacker: &ActivePokemon, field: &Field, attack: F) -> Type
    where F: Into<MoveContext>
{
    let MoveContext { attack, data, .. } = attack.into();
    match attacker.get_effective_ability() {
        Ability::Normalize if attack != Move::HiddenPower ||
            attack != Move::WeatherBall ||
            attack != Move::NaturalGift ||
            attack != Move::Judgment ||
            attack != Move::TechnoBlast => return Type::Normal,
        _ => {}
    };

    match attack {
        Move::HiddenPower => attacker.borrow().get_hidden_power_type(),
        Move::WeatherBall => match field.weather {
            Some(Weather::Sun(_)) => Type::Fire,
            Some(Weather::Rain(_)) => Type::Water,
            Some(Weather::Hail(_)) => Type::Ice,
            Some(Weather::Sandstorm(_)) => Type::Rock,
            Some(Weather::Fog) | None => Type::Normal
        },
        Move::Judgment => match attacker.borrow().held_item {
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
        Move::TechnoBlast => match attacker.borrow().held_item {
            Some(Item::DouseDrive) => Type::Water,
            Some(Item::ShockDrive) => Type::Electric,
            Some(Item::ChillDrive) => Type::Ice,
            Some(Item::BurnDrive) => Type::Fire,
            _ => Type::Normal
        },
        Move::NaturalGift => {
            if let Some(Item::Berry(a)) = &attacker.borrow().held_item {
                a.get_natural_gift_type()
            } else {
                data._type
            }
        }
        _ => data._type
    }
}

pub fn get_accuracy_factor(attacker: RefMut<ActivePokemon>, defender: RefMut<ActivePokemon>) -> f64 {
    let net_stages = attacker.data.borrow().accuracy_stage - defender.data.borrow().evasion_stage;
    1f64
}