use fraction::{Fraction, ToPrimitive};
use crate::{ActionSideEffects, ActivePokemon};
use crate::effects::{BOUND_HP, BOUND_HP_BINDING_BAND};

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