use std::cell::{Ref, RefCell, RefMut};
use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, DamageType, Effect, Move, MoveData, MultiHitFlavor, Power, VolatileBattleAilment};
use glazed_data::constants::Species;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::Pokemon;
use glazed_data::types::{Effectiveness, PokemonType, Type};
use crate::{ActionSideEffects, ActivePokemon, BattleData, Battlefield, Cause, Field, Weather};
use crate::effects::{MAX_STAGE, MIN_STAGE, CONFUSION_POWER};

pub type ActionResult<T> = Result<T, Cause>;

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
            Power::MultiHit(MultiHitFlavor::Variable(base)) | Power::MultiHit(MultiHitFlavor::Fixed(_, base)) |
            Power::BaseWithCharge(base, _) | Power::BaseWithCrash(base) => u16::from(base),
            _ => 0
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
// pub(crate) fn calculate_raw_damage_from_base(attacker: &ActivePokemon, defender: &ActivePokemon, base: u16, _type: Option<Type>, damage_type: DamageType, crit_rate: u8, field: &Field) -> ActionSideEffects {
//     // Effectiveness Calculation
//     let effectiveness = match _type {
//         Some(t) => defender.get_effective_type().defending_against(&t),
//         None => Effectiveness::NORMAL
//     };
//
//     if let Effectiveness::Immune = effectiveness {
//         return ActionSideEffects::NoEffect(Cause::Natural)
//     }
//
//     let stab = match _type {
//         Some(t) => attacker.get_effective_type().is_stab(&t),
//         None => false
//     };
//
//     // Critical Hit Calculation
//     let crit_stages = attacker.get_raw_critical_hit() + crit_rate;
//     let crit = match crit_stages {
//         0 => rand::thread_rng().gen_bool(0.0625),
//         1 => rand::thread_rng().gen_bool(0.125),
//         2 => rand::thread_rng().gen_bool(0.5),
//         _ => true
//     };
//
//     let (ea, ed) = match damage_type {
//         DamageType::Physical => (
//             attacker.get_effective_stat(BattleStat::Attack),
//             defender.get_effective_stat(BattleStat::Defense)),
//         DamageType::Special => (
//             attacker.get_effective_stat(BattleStat::SpecialAttack),
//             defender.get_effective_stat(BattleStat::SpecialDefense)),
//         DamageType::Status => panic!("Status move should not do direct damage??")
//     };
//
//     let base_damage_num = (((2 * u32::from(attacker.borrow().level)) / 5) + 2) * u32::from(base) * u32::from(ea);
//     let base_damage_denom = u32::from(ed) * 50;
//     let mut damage = base_damage_num / base_damage_denom + 2;
//
//     // * weather
//     if let Some(t) = _type {
//         if t == Type::Water {
//             if field.is_rain() {
//                 damage += damage / 2;
//             } else if field.is_sunny() {
//                 damage /= 2;
//             }
//         } else if t == Type::Fire {
//             if field.is_rain() {
//                 damage /= 2;
//             } else if field.is_sunny() {
//                 damage += damage / 2;
//             }
//         }
//     }
//
//     // * crit
//     if crit {
//         damage += damage / 2;
//     }
//
//     // * random
//     damage = (f64::from(damage) * rand::thread_rng().gen_range(0.85..=1.0)) as u32;
//
//     // * stab
//     if stab {
//         if attacker.get_effective_ability() == Ability::Adaptability {
//             damage *= 2;
//         } else {
//             // This is a fair approximation of 150%, right?
//             damage += damage / 2;
//         }
//     }
//
//     // * type
//     if let Effectiveness::Effect(i) = effectiveness {
//         if i < 0 {
//             damage = damage >> -i;
//         } else {
//             damage = damage << i;
//         }
//     }
//
//     // * burn
//     if attacker.borrow().status.burn &&
//         attacker.get_effective_ability() != Ability::Guts &&
//         damage_type == DamageType::Physical {
//         damage /= 2;
//     }
//
//     ActionSideEffects::DirectDamage {
//         damaged: defender.id,
//         damager: attacker.id,
//         attack: Move::Pound,
//         start_hp: defender.borrow().current_hp,
//         end_hp: defender.borrow().current_hp.saturating_sub(damage as u16),
//         hung_on_cause: None,
//         critical_hit: crit,
//         effectiveness
//     }
// }

/// Get the type of the move used. Takes ability and type-changing moves into account
pub fn get_effective_move_type<F>(attacker: &ActivePokemon, field: &RefCell<Field>, attack: F) -> Type
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
        Move::WeatherBall => match field.borrow().weather {
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

pub fn get_accuracy_factor(attacker: &ActivePokemon, defender: &ActivePokemon) -> f64 {
    let net_stages = attacker.data.borrow().accuracy_stage - defender.data.borrow().evasion_stage;
    1f64
}

pub fn get_type_effectiveness(field: &Battlefield, attacker: &ActivePokemon, attack: Move, defender: &ActivePokemon) -> (Effectiveness, Cause) {
    let move_type = get_effective_move_type(attacker, &field.field, attack);
    let attacker_ability = attacker.get_effective_ability();
    let defender_ability = defender.get_effective_ability();

    let raw_effectiveness = defender.get_effective_type().defending_against(&move_type);

    let (effectiveness, cause) =
        if defender_ability == Ability::WonderGuard {
            match raw_effectiveness {
                Effectiveness::Effect(i) if i > 0 => (Effectiveness::NORMAL, Cause::Natural),
                _ => {
                    let cause = Cause::Ability(defender.id, Ability::WonderGuard);
                    return if attacker_ability.is_ignore_ability_ability() {
                        let cause = cause.overwrite(Cause::Ability(attacker.id, attacker_ability));
                        (raw_effectiveness, cause)
                    } else {
                        (Effectiveness::Immune, cause)
                    }
                }
            }
        } else {
            (raw_effectiveness, Cause::Natural)
        };

    (effectiveness, cause)
}

/// Perform raw damage calculation
/// Nothing related to types (such as Weather, STAB, or Effectiveness) or ailments (Burn) is applied here.
/// Randomness is also not applied here. All additional multipliers should be handled externally.
pub fn calculate_raw_damage(attacker: &ActivePokemon, base_power: u16, damage_type: DamageType, defender: &ActivePokemon) -> u16 {
    let (ea, ed) = match damage_type {
        DamageType::Physical => (
            attacker.get_effective_stat(BattleStat::Attack),
            defender.get_effective_stat(BattleStat::Defense)
        ),
        DamageType::Special => (
            attacker.get_effective_stat(BattleStat::SpecialAttack),
            defender.get_effective_stat(BattleStat::SpecialDefense)
        ),
        DamageType::Status => (1, 1) // Shouldn't happen, but if it does, pokemon stats have no part in the calculation
    };

    let calc = ((2u16 * attacker.borrow().level as u16) / 5) + 2;
    let calc = calc * base_power * ea;
    (calc / (ed * 50)) + 2
}

/// Calculate confusion damage
/// Confusion damage is equivalent to a typeless physical move of power 40.
pub fn calculate_confusion_damage(pkmn: &ActivePokemon) -> u16 {
    let raw = calculate_raw_damage(pkmn, CONFUSION_POWER, DamageType::Physical, pkmn);
    (f64::from(raw) * rand::thread_rng().gen_range(0.85..=1.0)) as u16
}