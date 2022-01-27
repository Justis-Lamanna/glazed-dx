use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{Accuracy, Move};
use glazed_data::types::Type;
use crate::{ActionSideEffects, Slot, Battlefield, Cause, PROTECTION_CAP, BaseSlot};
use crate::core::{ActionCheck, MoveContext};

/// Get the probability of the attacker hitting the defender
fn get_accuracy_factor(attacker: &Slot, defender: &Slot) -> f64 {
    get_raw_factor(attacker.data.borrow().accuracy_stage, defender.data.borrow().evasion_stage)
}

/// Same as get_accuracy_factor, but ignores positive evasion changes.
fn get_accuracy_factor_foresight(attacker: &Slot, defender: &Slot) -> f64 {
    let evasion = defender.data.borrow().evasion_stage;
    let evasion = if evasion > 0 { 0 } else { evasion };
    get_raw_factor(attacker.data.borrow().accuracy_stage, evasion)
}

fn get_raw_factor(accuracy: i8, evasion: i8) -> f64 {
    let net_stages = accuracy - evasion;
    match net_stages {
        i8::MIN..=-6 => 3f64/9f64,
        -5 => 0.375,
        -4 => 3f64/7f64,
        -3 => 0.5,
        -2 => 0.6,
        -1 => 0.75,
        0 => 1f64,
        1 => 4f64/3f64,
        2 => 5f64/3f64,
        3 => 2f64,
        4 => 7f64/3f64,
        5 => 8f64/3f64,
        6..=i8::MAX => 3f64,
    }
}

/// Run the Accuracy formula, determining if an attack will miss. Type immunities shouldn't go here
pub fn do_accuracy_check<F>(field: &Battlefield, attacker: &Slot, attack: F, defender: &Slot) -> ActionCheck<bool>
    where F: Into<MoveContext>
{
    let percentage_formula = |p: u8| {
        let evasion_accuracy = match defender.data.borrow().foresight_by {
            Some(attack_id) if attack_id == attacker.slot_id => get_accuracy_factor_foresight(attacker, defender),
            _ => get_accuracy_factor(attacker, defender)
        };
        let move_accuracy = f64::from(p) / 100f64;
        let multiply = evasion_accuracy * move_accuracy;
        if multiply >= 1f64 {
            ActionCheck::Ok(true)
        } else {
            ActionCheck::Ok(rand::thread_rng().gen_bool(evasion_accuracy * move_accuracy))
        }
    };
    let MoveContext { attack, data: move_data, .. } = attack.into();

    // Bypass Accuracy Check
    if attacker.data.borrow().is_locked_on_to(defender) {
        return ActionCheck::Ok(true);
    } else if attack == Move::Toxic && attacker.get_effective_type().has_type(&Type::Poison) {
        return ActionCheck::Ok(true);
    } else if attack == Move::Thunder && field.field.borrow().is_rain() {
        return ActionCheck::Ok(true);
    } else if defender.data.borrow().minimized && attack.double_damage_on_minimized_target() {
        return ActionCheck::Ok(true);
    }

    // modified accuracy
    if attack == Move::Thunder && field.field.borrow().is_sunny() {
        return percentage_formula(50);
    }

    match move_data.accuracy {
        Accuracy::AlwaysHits => ActionCheck::Ok(true),
        Accuracy::Percentage(p) => percentage_formula(p),
        Accuracy::Variable => match attack {
            Move::Guillotine | Move::Fissure | Move::HornDrill | Move::SheerCold => {
                let level_user = attacker.borrow().level;
                let level_target = defender.borrow().level;
                if level_user < level_target {
                    ActionCheck::Err(ActionSideEffects::Failed(Cause::Natural))
                } else {
                    let acc = (level_user - level_target + 30) as f64 / 100f64;
                    ActionCheck::Ok(if acc > 1f64 { true } else {rand::thread_rng().gen_bool(acc)})
                }
            },
            Move::Protect | Move::Detect | Move::Endure | Move::QuickGuard | Move::WideGuard => {
                let counter = attacker.data.borrow().protection_counter;
                if counter == 0 {
                    ActionCheck::Ok(true)
                } else {
                    let counter = if counter < PROTECTION_CAP { counter } else { PROTECTION_CAP };
                    let accuracy = 1f64 / f64::from(1 << counter);
                    ActionCheck::Ok(rand::thread_rng().gen_bool(accuracy))
                }
            }
            _ => unimplemented!("Unknown accuracy for {:?}", attack)
        }
    }
}

/// Check whether the attacker can use the attack at all, regardless of the targets.
pub fn cannot_use_attack<F: Into<MoveContext>>(attacker: &Slot, attack: F) -> ActionCheck<bool> {
    let MoveContext { attack, .. } = attack.into();

    // Snore
    if attack.can_only_be_used_while_sleeping() && !attacker.borrow().status.is_asleep() {
        return ActionCheck::Ok(false);
    }

    ActionCheck::Ok(true)
}

/// Check whether the attacker can use the attack against a specific target.
/// "Type" and Ability immunities can go here.
pub fn cannot_use_attack_against<F>(attacker: &Slot, attack: F, defender: &Slot) -> ActionCheck<bool>
    where F: Into<MoveContext>
{
    let MoveContext { attack,.. } = attack.into();

    // Ability-based immunities
    match defender.get_effective_ability() {
        Ability::Soundproof if attack.is_sound_based() && !attacker.get_effective_ability().is_ignore_ability_ability() => {
            return ActionCheck::Err(ActionSideEffects::NoEffect(Cause::Ability(defender.id(), Ability::Soundproof)));
        },
        _ => {}
    }

    // Type-based immunities
    // note that regular type matchups are not calculated here, but instead when damage is applied
    let defender_type = defender.get_effective_type();
    if defender_type.has_type(&Type::Grass) && (attack.is_powder() || attack == Move::LeechSeed) {
        return ActionCheck::Err(ActionSideEffects::NoEffectSecondary(Cause::Type(Type::Grass)));
    } else if defender_type.has_type(&Type::Ghost) && attack.is_trapping() {
        return ActionCheck::Err(ActionSideEffects::NoEffectSecondary(Cause::Type(Type::Ghost)));
    }

    let using_against_self = attacker.id() == defender.id();

    if let Some(m) = defender.data.borrow().protected {
        match m {
            Move::Protect | Move::Detect if !using_against_self && !attack.bypasses_protect() => {
                return ActionCheck::Err(ActionSideEffects::IsProtected(defender.slot_id, m));
            }
            _ => {}
        }
    }

    if attack.can_only_be_used_on_sleeping_target() && !defender.borrow().status.is_asleep() {
        return ActionCheck::Ok(false);
    }
    ActionCheck::Ok(true)
}