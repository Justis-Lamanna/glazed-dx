use fraction::{Fraction, ToPrimitive};
use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, Move};

use crate::{ActionSideEffects, ActivePokemon, Cause};
use crate::damage::calculate_confusion_damage;
use crate::constants::{*};
use crate::core::CheckResult;
use crate::core::CheckResult::{Effect, EffectAndEnd, EffectsAndEnd, Nothing};

pub fn do_freeze_check(attacker: &ActivePokemon, attack: Move) -> CheckResult<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    if pkmn.status.freeze {
        let thaw = attack.can_thaw_user() || rand::thread_rng().gen_bool(THAW_CHANCE);
        if thaw {
            pkmn.status.freeze = false;
            Effect(ActionSideEffects::Thawed(attacker.id))
        } else {
            EffectAndEnd(ActionSideEffects::WasFrozen(attacker.id))
        }
    } else {
        Nothing
    }
}

pub fn do_sleep_check(attacker: &ActivePokemon, attack: Move) -> CheckResult<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    if pkmn.status.sleep > 0 {
        pkmn.status.sleep -= 1;
        if pkmn.status.sleep == 0 {
            Effect(ActionSideEffects::WokeUp(attacker.id))
        } else if !attack.can_be_used_while_sleeping() {
            EffectAndEnd(ActionSideEffects::Sleep(attacker.id))
        } else {
            Effect(ActionSideEffects::Sleep(attacker.id))
        }
    } else {
        Nothing
    }
}

pub fn do_paralysis_check(attacker: &ActivePokemon) -> CheckResult<ActionSideEffects> {
    let pkmn = attacker.borrow();
    if pkmn.status.paralysis && rand::thread_rng().gen_bool(FULL_PARALYSIS_CHANCE) {
        EffectAndEnd(ActionSideEffects::IsFullyParalyzed(attacker.id))
    } else {
        Nothing
    }
}

pub fn do_flinch_check(attacker: &ActivePokemon) -> CheckResult<ActionSideEffects> {
    let has_steadfast = attacker.get_effective_ability() == Ability::Steadfast;
    let mut data = attacker.data.borrow_mut();
    if data.flinch {
        data.flinch = false;
        let mut effects = vec![ActionSideEffects::Flinched(attacker.id)];
        if has_steadfast && data.speed_stage < MAX_STAGE {
            let start = data.speed_stage;
            let end = start + 1;
            data.speed_stage = end;
            effects.push(ActionSideEffects::StatChanged {
                affected: attacker.id,
                stat: BattleStat::Speed,
                cause: Cause::Ability(attacker.id, Ability::Steadfast),
                start,
                end
            })
        }
        EffectsAndEnd(effects)
    } else {
        Nothing
    }
}

pub fn do_confusion_check(attacker: &ActivePokemon) -> CheckResult<ActionSideEffects> {
    let mut data = attacker.data.borrow_mut();
    if data.confused > 0 {
        data.confused -= 1;
        if data.confused == 0 {
            // Snapped out of confusion
            CheckResult::Effect(ActionSideEffects::SnappedOutOfConfusion(attacker.id))
        } else if rand::thread_rng().gen_bool(CONFUSION_HIT_CHANCE) {
            // Hit itself in confusion
            let mut pkmn = attacker.borrow_mut();
            let delta = calculate_confusion_damage(attacker);
            let start_hp = pkmn.current_hp;
            let end_hp = start_hp.saturating_sub(delta);

            pkmn.current_hp = end_hp;
            CheckResult::EffectAndEnd(ActionSideEffects::HurtInConfusion {
                affected: attacker.id,
                start: start_hp,
                end: end_hp,
                hang_on_cause: None
            })
        } else {
            CheckResult::Nothing
        }
    } else {
        CheckResult::Nothing
    }
}

/// Perform binding damage, if applicable. Also decreases the turn amount, clearing the value if binding is complete.
pub fn do_binding_damage(attacker: &ActivePokemon) -> Vec<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    let mut data = attacker.data.borrow_mut();

    if let Some((turns, binding_band)) = data.bound {
        let fraction = if binding_band {BOUND_HP_BINDING_BAND } else { BOUND_HP };
        let damage = pkmn.hp.value / (fraction as u16);
        let start_hp = pkmn.current_hp;
        let end_hp = start_hp.saturating_sub(damage);
        pkmn.current_hp = end_hp;

        let mut effects = vec![ActionSideEffects::HurtByBound {
            bound: attacker.id,
            start_hp,
            end_hp
        }];

        if turns == 1 {
            data.bound = None;
            effects.push(ActionSideEffects::Unbound(attacker.id));
        } else {
            data.bound = Some((turns - 1, binding_band));
        }

        effects
    } else {
        vec![]
    }
}