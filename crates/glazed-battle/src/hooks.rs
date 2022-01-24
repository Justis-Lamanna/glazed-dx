use glazed_data::attack::{Move, Power};
use crate::{ActionSideEffects, Battlefield, Slot};

impl Battlefield {
    /// Utility to allow for this common logic. Appends on_attack_interrupt effects with all the pre-existing effects
    pub(crate) fn run_on_attack_interrupt_hooks(&self, attacker: &Slot, attack: Move, mut effects: Vec<ActionSideEffects>) -> Vec<ActionSideEffects> {
        effects.append(&mut self.on_attack_interrupt(attacker, attack));
        effects
    }

    /// When an attempted attack is interrupted (either because of missing, immunity, sleeping, no target, etc), this code is run.
    fn on_attack_interrupt(&self, attacker: &Slot, attack: Move) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let move_data = attack.data();
        if let Power::BaseWithCrash(_) = move_data.power {
            effects.push(attacker.take_crash_damage());
        } else if let Power::MultiTurn(_, _) = move_data.power {
            attacker.data.borrow_mut().rolling = None;
        }

        if attack.is_protection_move() {
            attacker.data.borrow_mut().protection_counter = 0;
        }

        effects
    }

    /// When an attack succeeds (at least 1 Pokemon affected), this code is run.
    pub(crate) fn on_attack_succeed(&self, attacker: &Slot, attack: Move) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let mut data = attacker.data.borrow_mut();

        // Rage ends if the user is enraged, and they don't use a rage move
        if data.enraged && !attack.data().is_rage() {
            data.enraged = false;
            effects.push(ActionSideEffects::RageEnd(attacker.id))
        }

        // Increment the protection counter for any protection moves. Reset it if anything else is used.
        if attack.is_protection_move() {
            data.protection_counter = data.protection_counter.saturating_add(1);
        } else {
            data.protection_counter = 0;
        }

        effects
    }
}