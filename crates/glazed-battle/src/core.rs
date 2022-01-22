use std::cell::RefCell;

use glazed_data::abilities::Ability;
use glazed_data::attack::{Move, MoveData, MultiHitFlavor, Power};
use glazed_data::item::{Item};
use glazed_data::types::{Effectiveness, Type};

use crate::{ActionSideEffects, Slot, Battlefield, Cause, Field, Weather};

pub type ActionCheck<T> = Result<T, ActionSideEffects>;

pub enum CheckResult<T> {
    Nothing,
    Effect(T),
    EffectAndEnd(T),
    EffectsAndEnd(Vec<T>)
}
impl<T> CheckResult<T> {
    pub fn add(self, list: &mut Vec<T>) -> bool {
        match self {
            CheckResult::Nothing => false,
            CheckResult::Effect(a) => {
                list.push(a);
                false
            }
            CheckResult::EffectAndEnd(a) => {
                list.push(a);
                true
            }
            CheckResult::EffectsAndEnd(a) => {
                list.extend(a);
                true
            }
        }
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
            Power::MultiHit(MultiHitFlavor::Variable(base)) | Power::MultiHit(MultiHitFlavor::Fixed(_, base)) |
            Power::BaseWithCharge(base, _) | Power::BaseWithCrash(base) | Power::BaseWithFaint(base) |
            Power::BaseWithDrain(base) => u16::from(base),
            _ => 0
        };
        MoveContext {
            attack,
            data,
            base_power
        }
    }
}

/// Get the type of the move used. Takes ability and type-changing moves into account
pub fn get_effective_move_type<F>(attacker: &Slot, field: &RefCell<Field>, attack: F) -> Type
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

pub fn get_type_effectiveness(field: &Battlefield, attacker: &Slot, attack: Move, defender: &Slot) -> (Effectiveness, Cause) {
    let move_type = get_effective_move_type(attacker, &field.field, attack);
    let attacker_ability = attacker.get_effective_ability();
    let defender_ability = defender.get_effective_ability();

    let raw_effectiveness = match defender.data.borrow().foresight_by {
        Some(attack_id) if attack_id == attacker.id => defender.get_effective_type().defending_against_ignore_immunities(&move_type),
        _ => defender.get_effective_type().defending_against(&move_type)
    };

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