use log::{debug, info};
use glazed_data::attack::{Move, MoveData, Power};
use glazed_data::lookups::Lookup;
use crate::{ActionSideEffects, Battlefield, Slot};

impl Battlefield {
    /// Utility to allow for this common logic. Appends on_attack_interrupt effects with all the pre-existing effects
    pub(crate) fn run_on_attack_interrupt_hooks(&self, attacker: &Slot, attack: Move, mut effects: Vec<ActionSideEffects>) -> Vec<ActionSideEffects> {
        effects.append(&mut self.on_attack_interrupt(attacker, attack));
        info!("Failed! Final effects: {:?}", effects);
        effects
    }

    /// When an attempted attack is interrupted (either because of missing, immunity, sleeping, no target, etc), this code is run.
    fn on_attack_interrupt(&self, attacker: &Slot, attack: Move) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let move_data = MoveData::lookup(&attack);
        if let Power::BaseWithCrash(_) = move_data.power {
            effects.push(attacker.take_crash_damage());
        } else if let Power::MultiTurn(_, _) = move_data.power {
            debug!("Attack {:?} failed, cancelling forced action", attack);
            attacker.data.borrow_mut().forced_action = None;
        }

        let mut data = attacker.data.borrow_mut();
        data.last_move_used = None;
        data.last_move_used_counter = 0;
        if attack.is_protection_move() {
            debug!("Resetting Protect counter: Protect failed");
            data.protection_counter = 0;
        }

        effects
    }

    /// When an attack succeeds (at least 1 Pokemon affected), this code is run.
    pub(crate) fn on_attack_succeed(&self, attacker: &Slot, attack: Move) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let mut data = attacker.data.borrow_mut();

        if let Some(proxy) = data.proxy_move {
            data.set_last_used_move(proxy)
        } else {
            data.set_last_used_move(attack)
        }

        // Rage ends if the user is enraged, and they don't use a rage move
        if data.enraged && !MoveData::lookup(&attack).is_rage() {
            data.enraged = false;
            debug!("Rage ended");
            effects.push(ActionSideEffects::RageEnd(attacker.slot_id))
        }

        // Increment the protection counter for any protection moves. Reset it if anything else is used.
        if attack.is_protection_move() {
            data.protection_counter = data.protection_counter.saturating_add(1);
            debug!("Incrementing Protect Counter: {}", data.protection_counter);
        } else {
            debug!("Resetting Protect counter");
            data.protection_counter = 0;

        }

        effects
    }
}