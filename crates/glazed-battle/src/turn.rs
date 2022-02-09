use std::cell::RefCell;
use rand::Rng;
use glazed_core::math;
use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, Move, ScreenType};

use crate::{ActionSideEffects, Slot, Cause, FieldSide, BaseSlot, BattleSideId};
use crate::damage::calculate_confusion_damage;
use crate::constants::{*};
use crate::core::CheckResult;
use crate::core::CheckResult::{Effect, EffectAndEnd, EffectsAndEnd, Nothing};

pub fn do_freeze_check(attacker: &Slot, attack: Move) -> CheckResult<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    if pkmn.status.freeze {
        let thaw = attack.can_thaw_user() || rand::thread_rng().gen_bool(THAW_CHANCE);
        if thaw {
            pkmn.status.freeze = false;
            Effect(ActionSideEffects::Thawed(attacker.slot_id))
        } else {
            EffectAndEnd(ActionSideEffects::WasFrozen(attacker.slot_id))
        }
    } else {
        Nothing
    }
}

pub fn do_sleep_check(attacker: &Slot, attack: Move) -> CheckResult<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    let attack = attacker.data.borrow().proxy_move.unwrap_or(attack);
    if pkmn.status.sleep > 0 {
        pkmn.status.sleep -= 1;
        if pkmn.status.sleep == 0 {
            attacker.data.borrow_mut().nightmare = false;
            Effect(ActionSideEffects::WokeUp(attacker.slot_id))
        } else if !attack.can_only_be_used_while_sleeping() {
            EffectAndEnd(ActionSideEffects::Sleep(attacker.slot_id))
        } else {
            Effect(ActionSideEffects::Sleep(attacker.slot_id))
        }
    } else {
        Nothing
    }
}

pub fn do_paralysis_check(attacker: &Slot) -> CheckResult<ActionSideEffects> {
    let pkmn = attacker.borrow();
    if pkmn.status.paralysis && rand::thread_rng().gen_bool(FULL_PARALYSIS_CHANCE) {
        EffectAndEnd(ActionSideEffects::IsFullyParalyzed(attacker.slot_id))
    } else {
        Nothing
    }
}

pub fn do_flinch_check(attacker: &Slot) -> CheckResult<ActionSideEffects> {
    let has_steadfast = attacker.get_effective_ability() == Ability::Steadfast;
    let mut data = attacker.data.borrow_mut();
    if data.flinch {
        data.flinch = false;
        let mut effects = vec![ActionSideEffects::Flinched(attacker.slot_id)];
        if has_steadfast && data.speed_stage < MAX_STAGE {
            let start = data.speed_stage;
            let end = start + 1;
            data.speed_stage = end;
            effects.push(ActionSideEffects::StatChanged {
                affected: attacker.slot_id,
                stat: BattleStat::Speed,
                cause: Cause::Ability(attacker.id(), Ability::Steadfast),
                start,
                end
            })
        }
        EffectsAndEnd(effects)
    } else {
        Nothing
    }
}

pub fn do_confusion_check(attacker: &Slot) -> CheckResult<ActionSideEffects> {
    let mut data = attacker.data.borrow_mut();
    if data.confused > 0 {
        data.confused -= 1;
        if data.confused == 0 {
            // Snapped out of confusion
            CheckResult::Effect(ActionSideEffects::SnappedOutOfConfusion(attacker.slot_id))
        } else if rand::thread_rng().gen_bool(CONFUSION_HIT_CHANCE) {
            // Hit itself in confusion
            let mut pkmn = attacker.borrow_mut();
            let delta = calculate_confusion_damage(attacker);
            let start_hp = pkmn.current_hp;
            let end_hp = start_hp.saturating_sub(delta);

            pkmn.current_hp = end_hp;
            CheckResult::EffectAndEnd(ActionSideEffects::HurtInConfusion {
                affected: attacker.slot_id,
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

pub fn do_disable_check(attacker: &Slot, attack: Move) -> CheckResult<ActionSideEffects> {
    let data = attacker.data.borrow_mut();
    if data.is_disabled(attack) {
        CheckResult::EffectAndEnd(ActionSideEffects::Disabled(attacker.slot_id, attack))
    } else {
        CheckResult::Nothing
    }
}

pub fn do_infatuation_check(attacker: &Slot) -> CheckResult<ActionSideEffects> {
    if attacker.data.borrow().infatuated {
        if  rand::thread_rng().gen_bool(INFATUATION_INACTION_CHANCE) {
            CheckResult::EffectsAndEnd(vec![
                ActionSideEffects::Infatuated(attacker.slot_id),
                ActionSideEffects::TooInfatuatedToAttack(attacker.slot_id)
            ])
        } else {
            CheckResult::Effect(ActionSideEffects::Infatuated(attacker.slot_id))
        }
    } else {
        CheckResult::Nothing
    }
}

pub fn do_screen_countdown(id: BattleSideId, side: &RefCell<FieldSide>) -> Vec<ActionSideEffects> {
    let mut effects = Vec::new();
    let mut side = side.borrow_mut();
    if side.light_screen > 0 {
        side.light_screen = side.light_screen.saturating_sub(1);
        if side.light_screen == 0 {
            effects.push(ActionSideEffects::ScreenEnd(id, ScreenType::LightScreen));
        }
    }
    if side.reflect > 0 {
        side.reflect = side.reflect.saturating_sub(1);
        if side.reflect == 0 {
            effects.push(ActionSideEffects::ScreenEnd(id, ScreenType::Reflect));
        }
    }
    if side.safeguard > 0 {
        side.safeguard = side.safeguard.saturating_sub(1);
    }
    effects
}

pub fn do_poison_damage(p: &Slot) -> Vec<ActionSideEffects> {
    if p.borrow().status.poison {
        vec![p.take_poison_damage()]
    } else {
        vec![]
    }
}

/// Perform binding damage, if applicable. Also decreases the turn amount, clearing the value if binding is complete.
pub fn do_binding_damage(attacker: &Slot) -> Vec<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    let mut data = attacker.data.borrow_mut();

    if let Some((turns, binding_band)) = data.bound {
        let fraction = if binding_band {BOUND_HP_BINDING_BAND } else { BOUND_HP };
        let damage = pkmn.hp.value / (fraction as u16);
        let start_hp = pkmn.current_hp;
        let end_hp = start_hp.saturating_sub(damage);
        pkmn.current_hp = end_hp;

        let mut effects = vec![ActionSideEffects::HurtByBound {
            bound: attacker.slot_id,
            start_hp,
            end_hp
        }];

        if turns == 1 {
            data.bound = None;
            effects.push(ActionSideEffects::Unbound(attacker.slot_id));
        } else {
            data.bound = Some((turns - 1, binding_band));
        }

        effects
    } else {
        vec![]
    }
}

pub fn do_nightmare_damage(affected: &Slot) -> Vec<ActionSideEffects> {
    if affected.data.borrow().nightmare {
        let mut pkmn = affected.borrow_mut();
        let damage = math::fraction(pkmn.hp.value, NIGHTMARE_MULTIPLIER);
        let (start_hp, end_hp) = pkmn.subtract_hp(damage);
        vec![ActionSideEffects::BasicDamage {
            damaged: affected.slot_id,
            start_hp,
            end_hp,
            cause: Cause::MoveSideEffect(Move::Nightmare)
        }]
    } else {
        vec![]
    }
}

pub fn do_curse_damage(affected: &Slot) -> Vec<ActionSideEffects> {
    if affected.data.borrow().cursed {
        let mut pkmn = affected.borrow_mut();
        let damage = math::fraction(pkmn.hp.value, CURSE_MULTIPLIER);
        let (start_hp, end_hp) = pkmn.subtract_hp(damage);
        vec![ActionSideEffects::BasicDamage {
            damaged: affected.slot_id,
            start_hp,
            end_hp,
            cause: Cause::MoveSideEffect(Move::Curse)
        }]
    } else {
        vec![]
    }
}