use std::cell::RefMut;
use std::convert::TryFrom;
use std::mem::take;
use log::{debug, info, Level, log_enabled};

use rand::Rng;
use strum::IntoEnumIterator;

use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, Effect, EffectPredicate, EntryHazardType, Move, MoveData, NonVolatileBattleAilment, Power, ScreenType, StatChangeTarget, Weather};
use glazed_data::species::Species;
use glazed_data::item::Item;
use glazed_data::types::{Effectiveness, PokemonType, Type};
use glazed_core::math;
use glazed_data::lookups::Lookup;

use crate::*;
use crate::effects::ProxyResult::{ContinueWith, DoNothing, Interrupt};

impl Battlefield {
    /// Called when a Pokemon enters battle (either at start, or when swapped out)
    pub fn enter_battle(&mut self, slot: SlotId) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let side = self.get_side_by_active_id(slot).borrow();
        let pkmn = self.get_active_pokemon_by_active_id(slot);
        info!("Pokemon enters battle in slot {}: {:?}", slot, pkmn);
        let pkmn_ability = pkmn.get_effective_ability();

        let grounded = || self.field.borrow().gravity > 0 || pkmn.borrow().is_holding(&Item::IronBall) || pkmn.data.borrow().rooted;
        let immune_to_spikes = pkmn_ability == Ability::MagicGuard ||
            ((pkmn.get_effective_type().has_type(&Type::Flying) ||  pkmn_ability == Ability::Levitate) && !grounded());

        // Spikes
        if side.spikes > 0 {
            if immune_to_spikes {
                debug!("Immune to spikes")
            } else {
                let max_hp = pkmn.borrow().hp.value;
                let damage = max_hp / get_spikes_damage(side.spikes);

                let (start_hp, end_hp) = pkmn.borrow_mut().subtract_hp(damage);
                debug!("Spike damage: {} ({} -> {})", end_hp - start_hp, start_hp, end_hp);
                effects.push(ActionSideEffects::BasicDamage {
                    damaged: slot,
                    start_hp,
                    end_hp,
                    cause: Cause::PokemonFieldState(FieldCause::Spikes)
                });

                if end_hp == 0 {
                    debug!("Pokemon fainted");
                    return effects;
                }
            }
        }

        effects
    }

    /// Called when swapping to a new Pokemon
    pub fn swap_pokemon(&mut self, slot: SlotId, member: PartyMemberId, baton_pass: bool) -> Vec<ActionSideEffects> {
        let s = self.get_active_pokemon_by_active_id(slot);
        info!("Pokemon leaving battle in slot {}: {:?}", slot, s);
        // Call hook for when a Pokemon swaps out
        for e_slot in self.get_everyone() {
            if e_slot.slot_id != slot {
                e_slot.data.borrow_mut().other_swapped_out(&s);
            }
        }

        let mut s = self.get_active_pokemon_by_active_id(slot);

        // Clear data. Baton pass is just a partial clear
        if baton_pass {
            s.data.replace_with(|t| t.baton_pass());
            debug!("Baton pass data: {:?}", s.data.borrow());
        } else {
            s.data.replace(BattleData::default());
            debug!("Clearing data")
        }

        // Call hook for start of turn action
        s.data.borrow_mut().start_of_turn();

        let old_member = s.id();
        s.party_slot_id = member;

        let mut effects = vec![ActionSideEffects::PokemonLeft(slot, old_member)];
        effects.append(&mut self.enter_battle(slot));
        effects
    }

    /// Perform a regular attack
    pub fn do_attack(&mut self, attacker_id: SlotId, attack: Move, defender: SelectedTarget) -> Vec<ActionSideEffects> {
        let attacker = self.get_active_pokemon_by_active_id(attacker_id);
        if let Some(_) = attacker.data.borrow().forced_action {
            return self.do_implicit_attack(attacker_id);
        }

        info!("{:?} (Slot {}) uses attack {:?} on target {:?}", attacker.borrow().species, attacker_id, attack, defender);
        attacker.data.borrow_mut().start_of_turn();
        attacker.data.borrow_mut().proxy_move = Some(attack);

        self._do_attack(attacker_id, attack, defender, false)
    }

    /// Do an attack a Pokemon is locked in to (example: charge attack selected the previous turn)
    pub fn do_implicit_attack(&mut self, attacker_id: SlotId) -> Vec<ActionSideEffects> {
        let attacker = self.get_active_pokemon_by_active_id(attacker_id);
        info!("{:?} (Slot {}) using implicit attack", attacker.borrow().species, attacker_id);
        attacker.data.borrow_mut().start_of_turn();

        let mut data = attacker.data.borrow_mut();
        match data.forced_action {
            Some(ForcedAction::DoNothing) => {
                debug!("Doing nothing");
                data.forced_action = None;
                vec![ActionSideEffects::Recharging(Cause::Natural)]
            },
            Some(ForcedAction::FinishAttack(attack, target)) => {
                debug!("Finishing attack {:?} on target {:?}", attack, target);
                data.invulnerable = None;
                data.forced_action = None;
                drop(data);
                let effects = self._do_attack(attacker_id, attack, target, true);
                effects
            },
            Some(ForcedAction::AttackWithWeakCounter(attack, counter)) => {
                debug!("Continuing attack {:?} (turns left: {})", attack, counter);
                drop(data);
                let mut effects = self._do_attack(attacker_id, attack, SelectedTarget::Implied, true);
                let damaged = ActionSideEffects::did_damage(&effects);

                if counter == 1 && attack == Move::Thrash {
                    effects.append(&mut inflict_confusion(&self.get_active_pokemon_by_active_id(attacker_id)));
                }

                let mut data = attacker.data.borrow_mut();
                data.set_last_used_move(attack);
                data.forced_action = if !damaged {
                    debug!("Ending attack (no damage done)");
                    None
                } else if counter == 1 {
                    debug!("Attack completed");
                    None
                } else {
                    Some(ForcedAction::AttackWithWeakCounter(attack, counter - 1))
                };
                effects
            },
            Some(ForcedAction::AttackWithCounter(attack, counter)) => {
                debug!("Continuing attack {:?} (turns left: {})", attack, counter);
                drop(data);
                let effects = self._do_attack(attacker_id, attack, SelectedTarget::Implied, true);

                let mut data = attacker.data.borrow_mut();
                data.set_last_used_move(attack);
                data.forced_action = if counter == 1 {
                    debug!("Attack completed");
                    None
                } else {
                    Some(ForcedAction::AttackWithWeakCounter(attack, counter - 1))
                };
                effects
            },
            Some(ForcedAction::Bide(damage, counter)) => {
                debug!("Biding (turns left: {}, total damage: {})", counter, damage);
                if counter == 1 {
                    let damage = damage.saturating_mul(2);
                    let effects = if let Some((defender, _)) = attacker.data.borrow().last_attacker {
                        debug!("Biding complete, inflicting damage {} on target {:?}", damage, defender);
                        attacker.data.borrow_mut().set_last_used_move(Move::Bide);
                        self.do_bide_damage(&attacker, damage, &self.get_active_pokemon_by_active_id(defender))
                    } else {
                        debug!("Biding complete, no target");
                        vec![ActionSideEffects::NoTarget]
                    };
                    attacker.data.borrow_mut().forced_action = None;
                    effects
                } else {
                    vec![ActionSideEffects::BideContinue(attacker_id)]
                }
            }
            None => {
                debug!("No implicit action, nothing happened.");
                vec![ActionSideEffects::NothingHappened]
            },
        }
    }

    /// Do all the end-of-turn things
    pub fn end_of_round(&mut self) -> Vec<ActionSideEffects> {
        info!("Running End-of-round checks for field: {:?}", self.field.borrow());
        let mut effects = Vec::new();

        let future_attacks = self.field.borrow_mut().decrement_future_attack_counters();
        for (origin, attack, recipient) in future_attacks {
            let PokemonId { party_id, party_slot_id } = origin;
            let party = self.get_party_by_party_id(party_id);
            let mock_attacker = Slot {
                slot_id: SlotId::MAX,
                party_id,
                party_slot_id,
                party,
                data: Rc::new(RefCell::default())
            };
            let defender = self.get_active_pokemon_by_active_id(recipient);
            if defender.borrow().has_health() {
                debug!("Executing future attack {:?} on {:?}", attack, recipient);
                effects.append(&mut self.do_damage(&mock_attacker, attack, &defender, false));
            } else {
                debug!("Future attack failed: target is fainted");
            }
        }

        let both_sides = self.get_sides_with_id();
        for (id, side) in both_sides {
            debug!("Running End-of-round checks for side {}: {:?}", id, side.borrow());
            effects.append(&mut turn::do_screen_countdown(id, side));
        }

        let everyone = self.get_everyone();
        for pokemon in everyone {
            debug!("Running End-of-round checks for pokemon: {:?}", pokemon);
            pokemon.data.borrow_mut().end_of_round();

            if pokemon.borrow().is_fainted() { continue; }
            effects.append(&mut turn::do_poison_damage(&pokemon));
            if pokemon.borrow().is_fainted() { continue; }
            effects.append(&mut turn::do_binding_damage(&pokemon));
            if pokemon.borrow().is_fainted() { continue; }
            effects.append(&mut turn::do_nightmare_damage(&pokemon));
            if pokemon.borrow().is_fainted() { continue; }
            effects.append(&mut turn::do_curse_damage(&pokemon));
            if pokemon.borrow().is_fainted() { continue; }

            let mut data = pokemon.data.borrow_mut();
            if data.perish_song_counter > 0 {
                data.perish_song_counter = data.perish_song_counter.saturating_sub(1);
                debug!("Decrement perish song counter to {}", data.perish_song_counter);
                effects.push(ActionSideEffects::PerishCount(pokemon.slot_id, data.perish_song_counter));
                if data.perish_song_counter == 0 {
                    effects.append(&mut Battlefield::faint(&pokemon, Cause::MoveSideEffect(Move::PerishSong)));
                    continue;
                }
            }
        }

        effects
    }

    /// Run weather effects
    pub fn do_weather(&self) -> Vec<ActionSideEffects> {
        debug!("Running weather checks");
        let mut effects = Vec::new();
        let mut field = self.field.borrow_mut();
        // Weather Effects
        if let Some(ctr) = &field.weather {
            match ctr {
                WeatherCounter::Sun(ctr) => {
                    let ctr = *ctr - 1;
                    if ctr == 0 {
                        debug!("Harsh sunlight ends");
                        effects.push(ActionSideEffects::EndWeather(Weather::HarshSun));
                        field.weather = None;
                    }
                    else {
                        debug!("Harsh sunlight continues ({} turns left)", ctr);
                        effects.push(ActionSideEffects::ContinueWeather(Weather::HarshSun));
                        field.weather = Some(WeatherCounter::Sun(ctr))
                    }
                },
                WeatherCounter::Rain(ctr) => {
                    let ctr = *ctr - 1;
                    if ctr == 0 {
                        debug!("Rain ends");
                        effects.push(ActionSideEffects::EndWeather(Weather::Rain));
                        field.weather = None;
                    }
                    else {
                        debug!("Rain continues ({} turns left)", ctr);
                        effects.push(ActionSideEffects::ContinueWeather(Weather::Rain));
                        field.weather = Some(WeatherCounter::Rain(ctr))
                    }
                },
                WeatherCounter::Fog => {
                    debug!("Fog continues");
                    effects.push(ActionSideEffects::ContinueWeather(Weather::Fog))
                },
                WeatherCounter::Hail(ctr) => {
                    let ctr = *ctr - 1;
                    if ctr == 0 {
                        debug!("Hail ends");
                        effects.push(ActionSideEffects::EndWeather(Weather::Hail));
                        field.weather = None;
                    }
                    else {
                        debug!("Hail continues ({} turns left)", ctr);
                        effects.push(ActionSideEffects::ContinueWeather(Weather::Hail));
                        field.weather = Some(WeatherCounter::Hail(ctr));
                        // Hail Damage
                        for pkmn in self.get_everyone() {
                            let immune_to_ice = pkmn.get_effective_type().has_type(&Type::Ice);
                            if !immune_to_ice {
                                let damage = pkmn.borrow().hp.value / 16;
                                let (start_hp, end_hp) = pkmn.borrow_mut().subtract_hp(damage);
                                debug!("Hail damage to {:?}: {} ({} -> {})", pkmn.slot_id, end_hp - start_hp, start_hp, end_hp);
                                effects.push(ActionSideEffects::BasicDamage {
                                    damaged: pkmn.slot_id,
                                    start_hp,
                                    end_hp,
                                    cause: Cause::PokemonFieldState(FieldCause::Weather(Weather::Hail))
                                })
                            } else {
                                debug!("{:?} is immune to Hail damage", pkmn.slot_id);
                            }
                        }
                    }
                },
                WeatherCounter::Sandstorm(ctr) => {
                    let ctr = *ctr - 1;
                    if ctr == 0 {
                        debug!("Sandstorm ends");
                        effects.push(ActionSideEffects::EndWeather(Weather::Sandstorm));
                        field.weather = None;
                    }
                    else {
                        debug!("Sandstorm continues ({} turns left)", ctr);
                        effects.push(ActionSideEffects::ContinueWeather(Weather::Sandstorm));
                        field.weather = Some(WeatherCounter::Sandstorm(ctr));
                        // Sandstorm Damage
                        for pkmn in self.get_everyone() {
                            let immune_to_standstorm =
                                pkmn.get_effective_type().has_type(&Type::Rock) ||
                                    pkmn.get_effective_type().has_type(&Type::Ground) ||
                                    pkmn.get_effective_type().has_type(&Type::Steel);
                            if !immune_to_standstorm {
                                let damage = pkmn.borrow().hp.value / 16;
                                let (start_hp, end_hp) = pkmn.borrow_mut().subtract_hp(damage);
                                debug!("Sandstorm damage to {:?}: {} ({} -> {})", pkmn.slot_id, end_hp - start_hp, start_hp, end_hp);
                                effects.push(ActionSideEffects::BasicDamage {
                                    damaged: pkmn.slot_id,
                                    start_hp,
                                    end_hp,
                                    cause: Cause::PokemonFieldState(FieldCause::Weather(Weather::Sandstorm))
                                })
                            } else {
                                debug!("{:?} is immune to Hail damage", pkmn.slot_id);
                            }
                        }
                    }
                }
            }
        }
        effects
    }

    /// Internal method to perform an attack
    fn _do_attack(&self, attacker_id: SlotId, attack: Move, defender: SelectedTarget, implicit: bool) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let attacker = self.get_active_pokemon_by_active_id(attacker_id);

        //region start-of-turn checks
        // Check for reasons this Pokemon can't perform this move
        // Follows same order as Gen IV.
        if turn::do_freeze_check(&attacker, attack).add(&mut effects) { return self.run_on_attack_interrupt_hooks(&attacker, attack, effects); }
        if turn::do_sleep_check(&attacker, attack).add(&mut effects) { return self.run_on_attack_interrupt_hooks(&attacker, attack, effects); }
        // Truant
        if turn::do_disable_check(&attacker, attack).add(&mut effects) { return self.run_on_attack_interrupt_hooks(&attacker, attack, effects); }
        // Imprison
        // Heal Block
        if turn::do_confusion_check(&attacker).add(&mut effects) { return self.run_on_attack_interrupt_hooks(&attacker, attack, effects); }
        if turn::do_flinch_check(&attacker).add(&mut effects) { return self.run_on_attack_interrupt_hooks(&attacker, attack, effects); }
        // Taunt
        // Gravity
        if turn::do_infatuation_check(&attacker).add(&mut effects) { return self.run_on_attack_interrupt_hooks(&attacker, attack, effects); }
        if turn::do_paralysis_check(&attacker).add(&mut effects) { return self.run_on_attack_interrupt_hooks(&attacker, attack, effects); }

        if attacker.borrow().is_fainted() { return self.run_on_attack_interrupt_hooks(&attacker, attack, effects); }
        //endregion

        // 0. Test if the attacker can use the move
        match accuracy::cannot_use_attack(&attacker, attack) {
            Ok(b) => {
                if !b {
                    debug!("Failed attack check, cannot use attack {:?}", attack);
                    effects.push(ActionSideEffects::Failed(Cause::Natural));
                    return self.run_on_attack_interrupt_hooks(&attacker, attack, effects);
                } else {
                    debug!("Passed attack check");
                }
            }
            Err(effect) => {
                debug!("{:?} failed, cannot use attack: {:?}", attack, effect);
                effects.push(effect);
                return self.run_on_attack_interrupt_hooks(&attacker, attack, effects);
            }
        }

        // If this move calls another move, we do the replacement here.
        let (attack, defender) = match proxy_move(&attacker, attack, &mut effects) {
            DoNothing => (attack, defender),
            ContinueWith(a) => {
                // We need to re-test the new move.
                match accuracy::cannot_use_attack(&attacker, a) {
                    Ok(b) => {
                        if !b {
                            debug!("Failed attack check, cannot use attack {:?}", attack);
                            effects.push(ActionSideEffects::Failed(Cause::Natural));
                            return self.run_on_attack_interrupt_hooks(&attacker, attack, effects);
                        } else {
                            debug!("Passed attack check");
                        }
                    }
                    Err(effect) => {
                        debug!("{:?} failed, cannot use attack: {:?}", attack, effect);
                        effects.push(effect);
                        return self.run_on_attack_interrupt_hooks(&attacker, attack, effects);
                    }
                }
                (a, SelectedTarget::Implied)
            },
            Interrupt => {
                return self.run_on_attack_interrupt_hooks(&attacker, attack, effects);
            }
        };

        let move_data = MoveData::lookup(&attack);

        // Specific types of moves that act on a later turn.
        // Naturally these shouldn't run when called on the later turn.
        if !implicit {
            if let Power::BaseWithCharge(_, place) = move_data.power {
                debug!("Attack {:?} charging (semi-unvulnerable: {:?}", attack, place);
                effects.push(ActionSideEffects::Charging(attacker_id, attack));

                // Skull Bash has a unique effect where it raises user defense on its charging turn
                if attack == Move::SkullBash {
                    let effect = change_self_stat(&attacker, BattleStat::Defense, 1);
                    if let ActionSideEffects::NoEffectSecondary(_) = effect {} else {
                        effects.push(effect);
                    }
                }

                let mut data = attacker.data.borrow_mut();

                if let Some(s) = place {
                    data.invulnerable = Some(s);
                }

                data.forced_action = Some(ForcedAction::FinishAttack(attack, defender));

                return effects;
            } else if let Power::BaseAfterNTurns(_, turns) = move_data.power {
                let mut field = self.field.borrow_mut();

                let defender = match defender {
                    SelectedTarget::Implied => {
                        let options = self.get_opposite_slots(attacker_id);
                        *options.first().unwrap()
                    },
                    SelectedTarget::One(a) => a
                };

                if field.will_receive_future_attack(defender) {
                    debug!("{:?} failed, {:?} already receiving an attack", attack, defender);
                    effects.push(ActionSideEffects::Failed(Cause::Natural))
                } else {
                    debug!("Adding future attack {:?} to {:?} in {} turns", attack, defender, turns);
                    field.add_future_attack(attacker.id(), attack, turns, defender);
                    effects.push(ActionSideEffects::FutureSight(attacker.slot_id))
                }
            }
        }

        // 1. Get the target(s)
        let targets = self.get_targets(attacker.slot_id, attack, defender);
        if targets.is_empty() {
            debug!("No valid targets");
            effects.push(ActionSideEffects::NoTarget);
            return self.run_on_attack_interrupt_hooks(&attacker, attack, effects);
        } else if log_enabled!(Level::Debug){
            debug!("Targets resolved: {:?}", targets.iter().map(|s| s.borrow()).collect::<Vec<_>>());
        }

        for t in targets.iter() {
            t.data.borrow_mut().targeted_by(&attacker, attack);
        }

        // 2. For each target, determine if the move hits
        //region Accuracy Check
        let mut targets_hit = Vec::new();
        for defender in targets {
            let hit = accuracy::do_accuracy_check(&self, &attacker, attack, &defender);
            match hit {
                Ok(b) => {
                    if b {
                        debug!("Accuracy check passed");
                        targets_hit.push(defender);
                    } else {
                        debug!("Accuracy check failed");
                        effects.push(ActionSideEffects::Missed(defender.slot_id, Cause::Natural));
                    }
                }
                Err(a) => {
                    debug!("Accuracy check failed: {:?}", a);
                    effects.push(a);
                }
            }
        }

        if targets_hit.is_empty() {
            debug!("Missed all targets");
            return self.run_on_attack_interrupt_hooks(&attacker, attack, effects);
        }
        //endregion

        effects.append(&mut self._do_damage_and_effects(&attacker, &targets_hit, attack, move_data));

        if effects.is_empty() {
            debug!("Nothing happened");
            effects.push(ActionSideEffects::NothingHappened)
        }

        //region end-of-turn checks
        let mut data = attacker.data.borrow_mut();

        effects.append(&mut data.lower_disable_counters()
            .iter()
            .map(|m| ActionSideEffects::NoLongerDisabled(attacker_id, *m))
            .collect());
        //endregion

        info!("Success! Final effects: {:?}", effects);
        effects
    }

    pub(crate) fn _do_damage_and_effects(&self, attacker: &Slot, targets_hit: &Vec<Slot>, attack: Move, move_data: &MoveData) -> Vec<ActionSideEffects> {
        let is_multi_target = targets_hit.len() > 1;
        let mut effects = Vec::new();

        // 3. For each hit target, perform damage
        // For non-damaging moves, all valid targets go on to Step 4.
        //region Primary Strike
        let targets_for_secondary_damage= {
            let mut damaged = Vec::new();
            for defender in targets_hit {
                match accuracy::cannot_use_attack_against(attacker, attack, defender) {
                    Ok(b) => {
                        if !b {
                            debug!("Failed attack check: cannot use {:?} on {:?}", attack, defender.borrow());
                            effects.push(ActionSideEffects::Failed(Cause::Natural));
                            continue;
                        } else {
                            debug!("Passed attack check: can use {:?} on {:?}", attack, defender.borrow())
                        }
                    }
                    Err(e) => {
                        debug!("Failed attack check: cannot use {:?} on {:?}: {:?}", attack, defender.borrow(), e);
                        effects.push(e);
                        continue;
                    }
                }

                if move_data.is_no_power_move() {
                    if defender.is_behind_substitute() && !attack.bypasses_substitute() {
                        debug!("Status attack failed: cannot use {:?} on {:?} (behind substitute)", attack, defender.borrow());
                        effects.push(ActionSideEffects::Failed(Cause::Natural));
                        continue;
                    } else {
                        damaged.push(defender);
                    }
                } else {
                    let mut individual_effects = self.do_damage(attacker, attack, &defender, is_multi_target);

                    let hit_substitute = individual_effects.iter().any(|e| e.hit_substitute());
                    let defender_fainted = defender.borrow().is_fainted();

                    effects.append(&mut individual_effects);

                    if !defender_fainted && !hit_substitute {
                        damaged.push(defender);
                    }
                }
            }
            damaged
        };

        if targets_for_secondary_damage.is_empty() {
            debug!("No targets to apply statuses to");
            return self.run_on_attack_interrupt_hooks(attacker, attack, effects);
        }
        //endregion

        // 4. Perform secondary effects.
        // Effects to the user happen regardless.
        // Effects to the target happen only to those hit in step 3.
        //region Secondary effects
        for secondary_effect in move_data.effects.iter() {
            let secondary_effect = if let Effect::Predicated(predicate, if_match, if_not_match) = secondary_effect {
                let matches = match predicate {
                    EffectPredicate::Sunny => self.field.borrow().is_sunny()
                };

                if matches {
                    debug!("Predicate effect passed, running effect {:?}", if_match);
                    if_match.as_ref()
                } else {
                    debug!("Predicate effect failed, running effect {:?}", if_match);
                    if_not_match.as_ref()
                }
            } else {
                secondary_effect
            };

            let mut secondary_effects = match secondary_effect {
                Effect::StatChange(stat, stages, probability, StatChangeTarget::User) => {
                    let triggers = self.activates_secondary_effect(attacker, *probability);
                    if triggers {
                        vec![change_self_stat(attacker, *stat, *stages)]
                    } else {
                        Vec::new()
                    }
                },
                Effect::StatChange(stat, stages, probability, StatChangeTarget::Target) => {
                    let do_stat_change = |attacker: &Slot, defender: &Slot|
                        vec![change_opponent_stat(&self, attacker, defender, *stat, *stages)];
                    self.do_probable_effect_on_all_targets(attacker, &targets_for_secondary_damage, *probability, do_stat_change)
                },
                Effect::NonVolatileStatus(ailment, probability, StatChangeTarget::User) => {
                    let triggers = self.activates_secondary_effect(attacker, *probability);
                    if triggers {
                        inflict_non_volatile_status(attacker, *ailment)
                    } else {
                        Vec::new()
                    }
                },
                Effect::NonVolatileStatus(ailment, probability, StatChangeTarget::Target) => {
                    targets_for_secondary_damage.iter()
                        .filter(|_| self.activates_secondary_effect(attacker, *probability))
                        .flat_map(|defender| {
                            if self.get_side_by_party_id(defender.party_id).borrow().safeguard > 0 {
                                vec![ActionSideEffects::NoEffectSecondary(Cause::PokemonFieldState(FieldCause::Safeguard))]
                            } else {
                                inflict_non_volatile_status(defender, *ailment)
                            }
                        })
                        .collect()
                },
                Effect::Confuse(probability, StatChangeTarget::User) => {
                    let triggers = self.activates_secondary_effect(attacker, *probability);
                    if triggers {
                        inflict_confusion(attacker)
                    } else {
                        Vec::new()
                    }
                },
                Effect::Confuse(probability, StatChangeTarget::Target) => {
                    targets_for_secondary_damage.iter()
                        .filter(|_| self.activates_secondary_effect(attacker, *probability))
                        .flat_map(|defender| {
                            if self.get_side_by_party_id(defender.party_id).borrow().safeguard > 0 {
                                vec![ActionSideEffects::NoEffectSecondary(Cause::PokemonFieldState(FieldCause::Safeguard))]
                            } else {
                                inflict_confusion(attacker)
                            }
                        })
                        .collect()
                },
                Effect::Infatuate(probability) =>
                    self.do_probable_effect_on_all_targets(attacker, &targets_for_secondary_damage, *probability, inflict_infatuation),
                Effect::ForceSwitch(StatChangeTarget::User) => {
                    vec![ActionSideEffects::ForcePokemonSwap { must_leave: attacker.id() }]
                },
                Effect::ForceSwitch(StatChangeTarget::Target) => {
                    targets_for_secondary_damage.iter()
                        .map(|defender| {
                            if defender.is_behind_substitute() {
                                ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(defender.slot_id, PokemonState::Substituted))
                            } else if defender.get_effective_ability() == Ability::SuctionCups {
                                ActionSideEffects::NoEffectSecondary(Cause::Ability(defender.id(), Ability::SuctionCups))
                            } else if defender.data.borrow().rooted {
                                ActionSideEffects::NoEffectSecondary(Cause::Move(defender.id(), Move::Ingrain))
                            } else {
                                ActionSideEffects::ForcePokemonSwap { must_leave: defender.id() }
                            }
                        })
                        .collect()
                },
                Effect::DropCoins => {
                    self.field.borrow_mut().drop_coins(attacker.borrow().level as u16 * 5);
                    vec![ActionSideEffects::DroppedCoins]
                },
                Effect::Bind => {
                    let has_grip_claw = if let Some(Item::GripClaw) = attacker.borrow().held_item { true } else { false };
                    let has_binding_band = if let Some(Item::BindingBand) = attacker.borrow().held_item { true } else { false };

                    targets_for_secondary_damage.iter()
                        .map(|defender| {
                            let mut data = defender.data.borrow_mut();
                            let turns_bound = if has_grip_claw {
                                BOUND_TURN_GRIP_CLAW
                            } else {
                                rand::thread_rng().gen_range(BOUND_TURN_RANGE)
                            };
                            data.bound = Some((turns_bound, has_binding_band));
                            ActionSideEffects::Bound {
                                binder: attacker.slot_id,
                                bound: defender.slot_id,
                                turns: turns_bound,
                                attack
                            }
                        })
                        .collect()
                }
                Effect::Flinch(probability) => self.do_probable_effect_on_all_targets(attacker, &targets_for_secondary_damage, *probability, do_flinch_effect),
                Effect::Thrash => {
                    let mut data = attacker.data.borrow_mut();
                    if data.forced_action.is_none() {
                        data.forced_action = Some(ForcedAction::AttackWithWeakCounter(attack, rand::thread_rng().gen_range(THRASH_RANGE)))
                    }
                    vec![]
                },
                Effect::Disable => {
                    targets_for_secondary_damage.iter()
                        .map(|defender| {
                            let mut data = defender.data.borrow_mut();
                            match data.last_move_used {
                                None => ActionSideEffects::Failed(Cause::Natural),
                                Some(m) => {
                                    data.disabled.push((m, DISABLE_TURN_COUNT));
                                    ActionSideEffects::Disabled(defender.slot_id, m)
                                }
                            }
                        })
                        .collect()
                },
                Effect::Mist => {
                    let mut side = self.get_side_by_party_id(attacker.party_id).borrow_mut();
                    if side.mist == 0 {
                        side.mist = MIST_TURN_COUNT;
                        vec![ActionSideEffects::MistStart(self.get_side_id_by_party_id(attacker.party_id))]
                    } else {
                        vec![]
                    }
                },
                Effect::Recharge => {
                    let mut data = attacker.data.borrow_mut();
                    data.forced_action = Some(ForcedAction::DoNothing);
                    vec![]
                },
                Effect::Heal(percentage) => do_heal_effect(attacker, (*percentage, 100)),
                Effect::WeatherHeal { no_weather, sun, other_weather } => {
                    let field = self.field.borrow();
                    if field.is_clear() {
                        do_heal_effect(attacker, *no_weather)
                    } else if field.is_sunny() {
                        do_heal_effect(attacker, *sun)
                    } else {
                        do_heal_effect(attacker, *other_weather)
                    }
                }
                Effect::Leech => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_leech_effect),
                Effect::Rage => {
                    let mut data = attacker.data.borrow_mut();
                    if data.enraged {
                        vec![ActionSideEffects::RageContinue(attacker.slot_id)]
                    } else {
                        data.enraged = true;
                        vec![ActionSideEffects::RageStart(attacker.slot_id)]
                    }
                },
                Effect::Mimic => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_mimic_effect),
                Effect::ChangeWeather(weather) => {
                    match weather {
                        Weather::HarshSun if self.field.borrow().is_sunny() => { return vec![ActionSideEffects::Failed(Cause::Natural)]},
                        Weather::Rain if self.field.borrow().is_rain() => { return vec![ActionSideEffects::Failed(Cause::Natural)]},
                        Weather::Sandstorm if self.field.borrow().is_sandstorm() => { return vec![ActionSideEffects::Failed(Cause::Natural)]},
                        Weather::Hail if self.field.borrow().is_hail() => { return vec![ActionSideEffects::Failed(Cause::Natural)]},
                        _ => {}
                    }

                    let w = match weather {
                        Weather::HarshSun => {
                            let turns =
                                if attacker.borrow().is_holding(&Item::HeatRock) { WEATHER_WITH_ROCK_TURN_COUNT }
                                else { WEATHER_TURN_COUNT };
                            WeatherCounter::Sun(turns)
                        },
                        Weather::Rain => {
                            let turns =
                                if attacker.borrow().is_holding(&Item::DampRock) { WEATHER_WITH_ROCK_TURN_COUNT }
                                else { WEATHER_TURN_COUNT };
                            WeatherCounter::Rain(turns)
                        },
                        Weather::Sandstorm => {
                            let turns =
                                if attacker.borrow().is_holding(&Item::SmoothRock) { WEATHER_WITH_ROCK_TURN_COUNT }
                                else { WEATHER_TURN_COUNT };
                            WeatherCounter::Sandstorm(turns)
                        }
                        Weather::Hail => {
                            let turns =
                                if attacker.borrow().is_holding(&Item::IcyRock) { WEATHER_WITH_ROCK_TURN_COUNT }
                                else { WEATHER_TURN_COUNT };
                            WeatherCounter::Hail(turns)
                        }
                        Weather::Fog => {
                            WeatherCounter::Fog
                        }
                    };
                    self.field.borrow_mut().weather = Some(w);
                    vec![ActionSideEffects::StartWeather(*weather)]
                },
                Effect::DispelWeather => unimplemented!("No Weather Yet"),
                Effect::Predicated(_, _, _) => panic!("Nested predicated not supported"),
                Effect::Custom => panic!("Consider changing from Custom to a concrete effect"),
                Effect::Minimize => {
                    let mut data = attacker.data.borrow_mut();
                    data.minimized = true;
                    Vec::new()
                },
                Effect::Curl => {
                    let mut data = attacker.data.borrow_mut();
                    data.curled = true;
                    Vec::new()
                },
                Effect::Screen(screen) => {
                    let mut side = self.get_side_by_party_id(attacker.party_id).borrow_mut();
                    let turn_count = if attacker.borrow().is_holding(&Item::LightClay) {
                        SCREEN_TURN_COUNT_LIGHT_CLAY
                    } else {
                        SCREEN_TURN_COUNT
                    };

                    do_screen_effect(self.get_side_id_by_party_id(attacker.party_id), &mut side, screen, turn_count)
                },
                Effect::StatReset => do_stat_reset_effect(&targets_for_secondary_damage),
                Effect::Bide => {
                    let mut data = attacker.data.borrow_mut();
                    data.forced_action = Some(ForcedAction::Bide(0, BIDE_TURN_COUNT));
                    vec![ActionSideEffects::BideStart(attacker.slot_id)]
                },
                Effect::Transform => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_transform_effect),
                Effect::Rest => do_rest_effect(attacker),
                Effect::Conversion => do_conversion_effect(attacker),
                Effect::Conversion2 => do_conversion_2_effect(attacker),
                Effect::TriAttack => self.do_probable_effect_on_all_targets(attacker, &targets_for_secondary_damage, 20, do_tri_attack_effect),
                Effect::Substitute => do_substitute_effect(attacker),
                Effect::Sketch => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_sketch_effect),
                Effect::StealItem => do_optional_effect_on_last_target(attacker, &targets_for_secondary_damage, do_steal_item_effect),
                Effect::Trap => do_effect_on_all_targets(attacker, &targets_for_secondary_damage, do_trap_effect),
                Effect::LockOn => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_lock_on_effect),
                Effect::Nightmare => do_effect_on_all_targets(attacker, &targets_for_secondary_damage, do_nightmare_effect),
                Effect::Curse => {
                    let is_ghost = attacker.get_effective_type().has_type(&Type::Ghost);
                    if is_ghost {
                        do_effect_on_all_targets(attacker, &targets_for_secondary_damage, do_ghost_curse_effect)
                    } else {
                        let mut effects = Vec::new();
                        effects.push(change_self_stat(attacker, BattleStat::Speed, -1));
                        effects.push(change_self_stat(attacker, BattleStat::Attack, 1));
                        effects.push(change_self_stat(attacker, BattleStat::Defense, 1));
                        effects
                    }
                }
                Effect::Spite => do_effect_on_all_targets(attacker, &targets_for_secondary_damage, do_spite_effect),
                Effect::Protect => do_protect_effect(attacker, attack),
                Effect::BellyDrum => do_belly_drum_effect(attacker),
                Effect::EntryHazard(eh) => {
                    match targets_for_secondary_damage.last() {
                        None => vec![ActionSideEffects::NoTarget],
                        Some(t) => do_hazard_effect(self.get_side_id_by_party_id(t.party_id), self.get_side_by_party_id(t.party_id), eh)
                    }
                }
                Effect::Foresight => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_foresight_effect),
                Effect::DestinyBond => {
                    attacker.data.borrow_mut().destiny_bond = true;
                    vec![ActionSideEffects::DestinyBond(attacker.slot_id)]
                },
                Effect::PerishSong => {
                    let affected_count = targets_for_secondary_damage.iter()
                        .filter(|e| e.data.borrow_mut().perish_song_counter == 0)
                        .map(|e| e.data.borrow_mut().perish_song_counter = 3)
                        .count();
                    if affected_count == 0 {
                        vec![ActionSideEffects::Failed(Cause::Natural)]
                    } else {
                        vec![ActionSideEffects::StartPerishSong]
                    }
                }
                Effect::Safeguard => do_safeguard_effect(self.get_side_id_by_party_id(attacker.party_id),self.get_side_by_party_id(attacker.party_id)),
                Effect::PainSplit => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_pain_split_effect),
                Effect::BatonPass => do_baton_pass_effect(attacker),
                Effect::Encore => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_encore_effect),
                Effect::ClearHazards => clear_hazards(attacker, self.get_side_by_party_id(attacker.party_id))
            };
            effects.append(&mut secondary_effects);
        }

        effects.append(&mut self.on_attack_succeed(attacker, attack));

        effects
    }

    /// Check if a secondary effect activates
    fn activates_secondary_effect(&self, attacker: &Slot, probability: u8) -> bool {
        let probability = if let Ability::SereneGrace = attacker.get_effective_ability() {
            probability * 2
        } else {
            probability
        };
        let activates = if probability == 0 || probability > 100 {
            true
        } else {
            rand::thread_rng().gen_bool(f64::from(probability) / 100f64)
        };

        if log_enabled!(Level::Debug) && activates {
            debug!("Secondary effect activates")
        }
        activates
    }

    /// Run an effect on all targets, doing probability check for each
    fn do_probable_effect_on_all_targets<F>(&self, attacker: &Slot, targets: &Vec<&Slot>, probability: u8, effect: F) -> Vec<ActionSideEffects>
        where F: Fn(&Slot, &Slot) -> Vec<ActionSideEffects>
    {
        targets.iter()
            .filter(|_| self.activates_secondary_effect(attacker, probability))
            .flat_map(|target| effect(attacker, target))
            .collect()
    }
}

/// Run an effect on all targets, with no probability check
fn do_effect_on_all_targets<F>(attacker: &Slot, to_effect: &Vec<&Slot>, effect: F) -> Vec<ActionSideEffects>
where F: Fn(&Slot, &Slot) -> Vec<ActionSideEffects>
{
    to_effect.iter()
        .flat_map(|pkmn| effect(attacker, pkmn))
        .collect()
}

/// Run an effect only on the last target
/// This should only be used when you are *CERTAIN* that only one (or zero) target(s) will ever exist.
/// If there are no targets, the return Vec contains NoTarget.
fn do_effect_on_last_target<F>(attacker: &Slot, to_effect: &Vec<&Slot>, effect: F) -> Vec<ActionSideEffects>
    where F: Fn(&Slot, &Slot) -> Vec<ActionSideEffects>
{
    match to_effect.last() {
        Some(t) => effect(attacker, t),
        None => vec![ActionSideEffects::NoTarget]
    }
}

/// Run an effect only on the last target
/// This should only be used when you are *CERTAIN* that only one (or zero) target(s) will ever exist.
/// If there are no targets, the return Vec is empty.
fn do_optional_effect_on_last_target<F>(attacker: &Slot, to_effect: &Vec<&Slot>, effect: F) -> Vec<ActionSideEffects>
    where F: Fn(&Slot, &Slot) -> Vec<ActionSideEffects>
{
    match to_effect.last() {
        Some(t) => effect(attacker, t),
        None => vec![]
    }
}

// Move Effects

pub fn inflict_confusion(afflicted: &Slot) -> Vec<ActionSideEffects> {
    if afflicted.get_effective_ability() == Ability::OwnTempo {
        debug!("Failed to inflict confusion, {:?} has Own Tempo", afflicted.id());
        return vec![ActionSideEffects::NoEffectSecondary(Cause::Ability(afflicted.id(), Ability::OwnTempo))]
    }

    let mut data = afflicted.data.borrow_mut();
    if data.confused > 0 {
        debug!("Failed to inflict confusion, Pokemon is already confused");
        vec![ActionSideEffects::NoEffectSecondary(Cause::Natural)]
    } else {
        data.confused = rand::thread_rng().gen_range(CONFUSION_TURN_RANGE);
        debug!("Inflicted confusion for {} turns", data.confused);
        vec![ActionSideEffects::Confuse(afflicted.slot_id)]
    }
}

pub fn inflict_infatuation(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    if attacker.get_effective_gender().can_infatuate(target.get_effective_gender()) {
        if target.get_effective_ability() == Ability::Oblivious {
            debug!("Failed to inflict infatuation, {:?} has Oblivious", target.id());
            return vec![ActionSideEffects::NoEffectSecondary(Cause::Ability(target.id(), Ability::Oblivious))]
        }

        let mut data = target.data.borrow_mut();
        if data.infatuated {
            debug!("Failed to inflict infatuation, {:?} is already infatuated", target.id());
            vec![ActionSideEffects::NoEffectSecondary(Cause::Natural)]
        } else {
            debug!("Inflicted infatuation on {:?}", target.id());
            data.infatuated = true;
            vec![ActionSideEffects::Infatuated(target.slot_id)]
        }
    } else {
        debug!("Failed to inflict infatuation, {:?} and {:?} are incompatible", attacker.borrow().species, target.borrow().species);
        vec![ActionSideEffects::NoEffectSecondary(Cause::Natural)]
    }
}

fn _change_stat(affected: &Slot, stat: BattleStat, stages: i8, cause: Cause) -> ActionSideEffects {
    let current = affected.get_stat_stage(stat);
    let next = current + stages;
    return if next > MAX_STAGE {
        debug!("Can't raise {:?} {:?} stat, already at {}", affected.id(), stat, MAX_STAGE);
        ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(affected.slot_id, PokemonState::StatsMaxed(StatsCause::TooHigh)))
    } else if next < MIN_STAGE {
        debug!("Can't lower {:?} {:?} stat, already at {}", affected.id(), stat, MIN_STAGE);
        ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(affected.slot_id, PokemonState::StatsMaxed(StatsCause::TooLow)))
    } else {
        debug!("Increased {:?} {:?} stat ({} -> {})", affected.id(), stat, current, next);
        let mut data = affected.data.borrow_mut();
        match stat {
            BattleStat::Attack => data.attack_stage = next,
            BattleStat::Defense => data.defense_stage = next,
            BattleStat::SpecialAttack => data.special_attack_stage = next,
            BattleStat::SpecialDefense => data.special_defense_stage = next,
            BattleStat::Speed => data.speed_stage = next,
            BattleStat::Accuracy => data.accuracy_stage = next,
            BattleStat::Evasion => data.evasion_stage = next,
            BattleStat::CriticalHitRatio => data.crit_stage = if next < 0 { 0 } else { u8::try_from(next).unwrap() }
        };
        ActionSideEffects::StatChanged {
            stat,
            affected: affected.slot_id,
            cause,
            start: current,
            end: next
        }
    }
}

pub fn change_self_stat(affected: &Slot, stat: BattleStat, stages: i8) -> ActionSideEffects {
    let (ability_cause, stages) = match affected.get_effective_ability() {
        Ability::Simple => (Cause::Ability(affected.id(), Ability::Simple), stages * 2),
        Ability::Contrary => (Cause::Ability(affected.id(), Ability::Contrary), -stages),
        _ => (Cause::Natural, stages)
    };

    debug!("Changing {:?} {:?} stat by {} (affected by {:?})", affected.id(), stat, stages, ability_cause);
    _change_stat(affected, stat, stages, Cause::Natural)
}

pub fn change_opponent_stat(field: &Battlefield, affecter: &Slot, affected: &Slot, stat: BattleStat, stages: i8) -> ActionSideEffects {
    let affected_side = field.get_side_by_party_id(affected.party_id);

    if affected_side.borrow().mist > 0 {
        debug!("Cannot change stat, Mist in effect");
        return ActionSideEffects::NoEffectSecondary(Cause::PokemonFieldState(FieldCause::Mist))
    }

    let (ability_cause, stages) = match affected.get_effective_ability() {
        Ability::Simple => (Cause::Ability(affected.id(), Ability::Simple), stages * 2),
        Ability::Contrary  => {
            let cause = Cause::Ability(affected.id(), Ability::Contrary);
            match affected.get_effective_ability() {
                oc if oc.is_ignore_ability_ability() => (cause.overwrite(Cause::Ability(affecter.id(), oc)), -stages),
                _ => (cause, stages)
            }
        },
        oc@ (Ability::ClearBody | Ability::WhiteSmoke) => {
            let cause = Cause::Ability(affected.id(), oc);
            match affected.get_effective_ability() {
                oc if oc.is_ignore_ability_ability() => (cause.overwrite(Cause::Ability(affecter.id(), oc)), stages),
                _ => return ActionSideEffects::NoEffectSecondary(Cause::Ability(affected.id(), oc))
            }
        }
        _ => (Cause::Natural, stages)
    };

    debug!("Changing {:?} {:?} stat by {} (affected by {:?})", affected.id(), stat, stages, ability_cause);
    _change_stat(affected, stat, stages, ability_cause)
}

pub fn inflict_non_volatile_status(affected: &Slot, status: NonVolatileBattleAilment) -> Vec<ActionSideEffects> {
    {
        debug!("Inflicting status {:?} to {:?}", status, affected.id());
        let mut data = affected.data.borrow_mut();
        let mut affected = affected.borrow_mut();
        match status {
            NonVolatileBattleAilment::Paralysis => affected.status.paralysis = true,
            NonVolatileBattleAilment::Sleep => affected.status.sleep = rand::thread_rng().gen_range(SLEEP_TURNS_RANGE),
            NonVolatileBattleAilment::Freeze => affected.status.freeze = true,
            NonVolatileBattleAilment::Burn => affected.status.burn = true,
            NonVolatileBattleAilment::Poison(a) => {
                affected.status.poison = true;
                if let PoisonType::BadlyPoisoned = a {
                    data.poison_counter = 1;
                }
            }
        }
    }
    vec![ActionSideEffects::NonVolatileStatusAilment {
        affected: affected.slot_id,
        status
    }]
}

/// Sets the flinch flag on the defender.
/// Note that this doesn't make the target flinch *yet*. The target won't flinch until it is their turn.
fn do_flinch_effect(_attacker: &Slot, defender: &Slot) -> Vec<ActionSideEffects> {
    defender.data.borrow_mut().flinch = true;
    vec![ActionSideEffects::WillFlinch(defender.slot_id)]
}

/// Heal a percentage of the attacker's max HP
fn do_heal_effect(attacker: &Slot, percentage: (u8, u8)) -> Vec<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    let start_hp = pkmn.current_hp;
    let delta = math::fraction(pkmn.hp.value, percentage);
    let end_hp = start_hp.saturating_add(delta).clamp(0, pkmn.hp.value);
    pkmn.current_hp = end_hp;

    vec![ActionSideEffects::Healed {
        healed: attacker.slot_id,
        start_hp,
        end_hp,
        cause: Cause::Natural
    }]
}

/// Seed the target, if not already seeded.
fn do_leech_effect(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    let mut data = target.data.borrow_mut();
    let effect = if data.seeded.is_none() {
        data.seeded = Some(attacker.slot_id);
        ActionSideEffects::SeedStart {
            from: target.slot_id,
            to: attacker.slot_id
        }
    } else {
        ActionSideEffects::Failed(Cause::Natural)
    };
    vec![effect]
}

/// Get the target's last move used, and replace the attacker's Mimic with it for the rest of battle.
/// If the target hasn't used a move, or the move is not Mimic-able, fails.
fn do_mimic_effect(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    let data = target.data.borrow();
    let effect = match data.last_move_used {
        None => ActionSideEffects::NoEffectSecondary(Cause::Natural),
        Some(m) => {
            if m.can_be_mimicked() && !attacker.borrow().knows_move(m) {
                attacker.replace_mimic_with(m);
                ActionSideEffects::Mimicked(attacker.slot_id, m)
            } else {
                ActionSideEffects::Failed(Cause::Natural)
            }
        }
    };
    vec![effect]
}

/// Set up screen (Light Screen or Reflect) for some number of turns
fn do_screen_effect(id: BattleSideId, side: &mut RefMut<FieldSide>, screen: &ScreenType, turns: u8) -> Vec<ActionSideEffects> {
    let effect = match screen {
        ScreenType::LightScreen => {
            if side.light_screen > 0 {
                ActionSideEffects::NoEffectSecondary(Cause::Natural)
            } else {
                side.light_screen = turns;
                ActionSideEffects::ScreenStart(id, ScreenType::LightScreen)
            }
        }
        ScreenType::Reflect => {
            if side.reflect > 0 {
                ActionSideEffects::NoEffectSecondary(Cause::Natural)
            } else {
                side.reflect = turns;
                ActionSideEffects::ScreenStart(id, ScreenType::Reflect)
            }
        }
    };
    vec![effect]
}

/// Reset everyone's stat changes to 0.
fn do_stat_reset_effect(targets: &Vec<&Slot>) -> Vec<ActionSideEffects> {
    let mut effects = Vec::new();
    for pkmn in targets.iter() {
        let mut data = pkmn.data.borrow_mut();
        data.attack_stage = 0;
        data.defense_stage = 0;
        data.special_attack_stage = 0;
        data.special_defense_stage = 0;
        data.speed_stage = 0;
        data.accuracy_stage = 0;
        data.evasion_stage = 0;
        data.crit_stage = 0;
        effects.push(ActionSideEffects::StatsReset(pkmn.slot_id));
    }
    effects
}

/// Heal all non-volatile statues and HP and sleep. Fail if full health.
fn do_rest_effect(attacker: &Slot) -> Vec<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    if pkmn.is_full_health() {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    } else {
        let start_hp = pkmn.current_hp;
        pkmn.status.sleep = REST_SLEEP_TURN_COUNT;
        pkmn.status.poison = false;
        pkmn.status.freeze = false;
        pkmn.status.burn = false;
        pkmn.status.paralysis = false;
        pkmn.heal();
        vec![ActionSideEffects::Sleep(attacker.slot_id), ActionSideEffects::Healed {
            healed: attacker.slot_id,
            start_hp,
            end_hp: pkmn.current_hp,
            cause: Cause::Natural
        }]
    }
}

/// Pick a random Move the user has, and change the user's type to match it.
fn do_conversion_effect(attacker: &Slot) -> Vec<ActionSideEffects> {
    let slot = loop {
        if let Some(slot) = attacker.get_effective_move(rand::thread_rng().gen_range(0..4)) {
            break slot;
        }
    };
    let new_type = MoveData::lookup(&slot.attack)._type;
    attacker.data.borrow_mut().temp_type = Some(PokemonType::Single(new_type));
    vec![ActionSideEffects::ChangeType(attacker.slot_id, new_type)]
}

/// Pick a random type that is immune or not very effective to the target's last used move.
/// Fails if the target hasn't used a move yet.
fn do_conversion_2_effect(attacker: &Slot) -> Vec<ActionSideEffects> {
    let mut data = attacker.data.borrow_mut();
    if let Some((_, attack)) = data.last_attacker {
        let attack_type = MoveData::lookup(&attack)._type;
        let candidates: Vec<Type> = Type::iter()
            .filter(|t: &Type| {
                match attack_type.attacking(t) {
                    Effectiveness::Immune => true,
                    Effectiveness::Effect(i) => i < 0,
                }
            })
            .collect();
        let new_type = if candidates.len() == 1 {
            candidates.get(0).unwrap()
        } else {
            let idx = rand::thread_rng().gen_range(0..candidates.len());
            candidates.get(idx).unwrap()
        };
        data.temp_type = Some(PokemonType::Single(*new_type));
        vec![ActionSideEffects::ChangeType(attacker.slot_id, *new_type)]
    } else {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

/// Randomly either paralyze, burn, or freeze the target (1/3 chance of each)
fn do_tri_attack_effect(_attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    if target.is_behind_substitute() {
        vec![]
    } else {
        match rand::thread_rng().gen_range(0..3) {
            0 => inflict_non_volatile_status(target, NonVolatileBattleAilment::Paralysis),
            1 => inflict_non_volatile_status(target, NonVolatileBattleAilment::Burn),
            _ => inflict_non_volatile_status(target, NonVolatileBattleAilment::Freeze)
        }
    }
}

/// Take 1/4 max damage, and create a substitute with that much HP + 1.
/// Fails if not enough HP.
fn do_substitute_effect(attacker: &Slot) -> Vec<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    let substitute_hp = pkmn.hp.value / 4;
    if pkmn.current_hp <= substitute_hp {
        vec![ActionSideEffects::Failed(Cause::PokemonBattleState(attacker.slot_id, PokemonState::TooWeak))]
    } else {
        let (start_hp, end_hp) = pkmn.subtract_hp(substitute_hp);
        let mut data = attacker.data.borrow_mut();
        data.substituted = substitute_hp + 1;
        vec![ActionSideEffects::BasicDamage {
            damaged: attacker.slot_id,
            start_hp,
            end_hp,
            cause: Cause::PokemonBattleState(attacker.slot_id, PokemonState::Substituted)
        }, ActionSideEffects::CreatedSubstitute(attacker.slot_id)]
    }
}

/// Replace Sketch with the target's last used move, permanently.
/// Fails if the target hasn't moved.
fn do_sketch_effect(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    let target_data = target.data.borrow();
    match target_data.last_move_used {
        Some(m) => {
            if attacker.borrow().knows_move(m) {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            } else {
                attacker.replace_sketch_with(m);
                vec![ActionSideEffects::Sketched {
                    user: attacker.slot_id,
                    target: target.slot_id,
                    attack: m
                }]
            }
        },
        None => vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

/// Transform into the target. Species, gender, ability, moves, and all stats (except HP) are copied.
fn do_transform_effect(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    let transform = TransformData::from(target);
    let target = target.borrow();
    attacker.data.borrow_mut().transformed = Some(transform);
    vec![ActionSideEffects::Transform {
        id: attacker.slot_id,
        species: target.species,
        gender: target.gender,
        shiny: target.is_shiny()
    }]
}

/// Take the target's held item and give it to the user.
/// Fails if the target has no item, or the user has an item already.
/// Fails if the target is holding Mail.
/// Fails if the target has Sticky Hold.
/// Fails if either Pokemon are:
///     Arceus, and the items swapping is a plate
///     Giratina, and the items swapping is a Griseous Orb
///     Genesect, and the items swapping is a Drive
fn do_steal_item_effect(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    let mut attacking_pkmn = attacker.borrow_mut();
    if attacking_pkmn.held_item.is_some() {
        // We can't steal when holding a held item, so we do nothing.
        return vec![ActionSideEffects::Failed(Cause::PokemonBattleState(attacker.slot_id, PokemonState::HoldingItem))];
    }

    if target.get_effective_ability() == Ability::StickyHold {
        return vec![ActionSideEffects::Failed(Cause::Ability(attacker.id(), Ability::StickyHold))];
    }

    let mut target_pkmn = target.borrow_mut();
    let can_steal = match &target_pkmn.held_item {
        None => {
            return vec![ActionSideEffects::Failed(Cause::PokemonBattleState(target.slot_id, PokemonState::NotHoldingItem))];
        },
        Some(i) if i.is_mail() => false,
        Some(Item::GriseousOrb) => {
            match (attacking_pkmn.species, target_pkmn.species) {
                (Species::Giratina(_), _) | (_, Species::Giratina(_)) => false,
                _ => true
            }
        },
        Some(i) if i.is_plate() => {
            match (attacking_pkmn.species, target_pkmn.species) {
                (Species::Arceus(_), _) | (_, Species::Arceus(_)) => false,
                _ => true
            }
        },
        Some(i) if i.is_drive() => {
            match (attacking_pkmn.species, target_pkmn.species) {
                (Species::Genesect(_), _) | (_, Species::Genesect(_)) => false,
                _ => true
            }
        },
        _ => true
    };

    if can_steal {
        attacking_pkmn.held_item = take(&mut target_pkmn.held_item);
        vec![ActionSideEffects::StoleItem {
            from: target.slot_id,
            to: attacker.slot_id,
            item: attacking_pkmn.held_item.as_ref().unwrap().clone()
        }]
    } else {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

/// Prevent the Pokemon from leaving battle
fn do_trap_effect(_attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    target.data.borrow_mut().trapped = true;
    vec![ActionSideEffects::TrappedStart(target.slot_id)]
}

/// Attacks by the user to the target will not miss
fn do_lock_on_effect(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    attacker.data.borrow_mut().locked_on = Some((2, target.slot_id));
    vec![ActionSideEffects::LockedOn { user: attacker.slot_id, target: target.slot_id }]
}

/// Gives this Pokemon a nightmare. Nightmare's effect done at end of turn
/// Fails if the Pokemon isn't asleep.
fn do_nightmare_effect(_attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    if target.borrow().status.sleep > 0 {
        target.data.borrow_mut().nightmare = true;
        vec![ActionSideEffects::Nightmare(target.slot_id)]
    } else {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

/// Curse the target. Curse effect done at the end of turn
fn do_ghost_curse_effect(_attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    target.data.borrow_mut().cursed = true;
    vec![ActionSideEffects::Curse(target.slot_id)]
}

/// Reduces the PP of the target's last used move by 4.
/// Fails if the target hasn't moved, the target doesn't know the move anymore, or PP is already 0.
fn do_spite_effect(_attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    if let Some(m) = target.data.borrow().last_move_used {
        let mut pkmn = target.borrow_mut();
        if let Some(slot) = pkmn.get_move_slot_mut(m) {
            let pp_start = slot.pp;
            if pp_start == 0 {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            } else {
                let pp_end = pp_start.saturating_sub(SPITE_PP_COUNT);
                slot.pp = pp_end;
                vec![ActionSideEffects::LostPP(target.slot_id, slot.attack, pp_start, pp_end)]
            }
        } else {
            vec![ActionSideEffects::Failed(Cause::Natural)]
        }
    } else {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

/// Mark the user as protected (also used for Endure, Wide Guard, and Quick Guard).
fn do_protect_effect(attacker: &Slot, attack: Move) -> Vec<ActionSideEffects> {
    let mut data = attacker.data.borrow_mut();
    data.protected = Some(attack);
    vec![ActionSideEffects::StartProtection(attacker.slot_id, attack)]
}

/// Reduce HP by 50%, and maxes Pokemon's Attack to +6.
/// If the user has Contrary, Attack is reduced to -6.
/// Fails if not enough HP.
fn do_belly_drum_effect(attacker: &Slot) -> Vec<ActionSideEffects> {
    let has_contrary = attacker.get_effective_ability() == Ability::Contrary;

    let mut pkmn = attacker.borrow_mut();
    let delta_hp = pkmn.hp.value / 2;
    if pkmn.current_hp < delta_hp {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    } else {
        let (start_hp, end_hp) = pkmn.subtract_hp(delta_hp);
        let mut data = attacker.data.borrow_mut();
        if has_contrary {
            data.attack_stage = MIN_STAGE;
        } else {
            data.attack_stage = MAX_STAGE;
        }
        vec![ActionSideEffects::BasicDamage {
            damaged: attacker.slot_id,
            start_hp,
            end_hp,
            cause: Cause::MoveSideEffect(Move::BellyDrum)
        }, ActionSideEffects::StatMaxed(attacker.slot_id, BattleStat::Attack)]
    }
}

/// Sets up an Entry Hazard.
/// Fails if attempting to use Spikes for a fourth time, Toxic Spikes for a third time, and Stealth Rock for a second time.
fn do_hazard_effect(id: BattleSideId, side: &RefCell<FieldSide>, hazard: &EntryHazardType) -> Vec<ActionSideEffects> {
    let mut side = side.borrow_mut();
    match hazard {
        EntryHazardType::Spikes => {
            if side.spikes < MAX_SPIKES {
                side.spikes = side.spikes + 1;
                vec![ActionSideEffects::EntryHazard(id, Move::Spikes, side.spikes)]
            } else {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            }
        }
        EntryHazardType::ToxicSpikes => {
            if side.toxic_spikes < MAX_TOXIC_SPIKES {
                side.toxic_spikes = side.toxic_spikes + 1;
                vec![ActionSideEffects::EntryHazard(id, Move::ToxicSpikes, side.toxic_spikes)]
            } else {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            }
        }
        EntryHazardType::StealthRock => {
            if side.pointed_stones {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            } else {
                side.pointed_stones = true;
                vec![ActionSideEffects::EntryHazard(id, Move::StealthRock, 1)]
            }
        }
    }
}

/// Attacks from the attacker to the target ignore positive evasion changes.
fn do_foresight_effect(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    let mut target_data = target.data.borrow_mut();
    match target_data.foresight_by {
        Some(_) => vec![ActionSideEffects::Failed(Cause::Natural)],
        None => {
            target_data.foresight_by = Some(attacker.slot_id);
            vec![ActionSideEffects::Foresighted { user: attacker.slot_id, target: target.slot_id }]
        }
    }
}

fn do_safeguard_effect(id: BattleSideId, side: &RefCell<FieldSide>) -> Vec<ActionSideEffects> {
    let mut side = side.borrow_mut();
    if side.safeguard > 0 {
        vec![ActionSideEffects::Failed(Cause::PokemonFieldState(FieldCause::Safeguard))]
    } else {
        side.safeguard = SAFEGUARD_TURN_COUNT;
        vec![ActionSideEffects::Safeguard(id)]
    }
}

fn do_pain_split_effect(attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    let mut attacker_pkmn = attacker.borrow_mut();
    let attacker_start_hp = attacker_pkmn.current_hp;
    let mut defender_pkmn = target.borrow_mut();
    let defender_start_hp = defender_pkmn.current_hp;
    let end_hp = (attacker_start_hp + defender_start_hp) / 2;

    attacker_pkmn.current_hp = math::cap_max(end_hp, attacker_pkmn.hp.value);
    defender_pkmn.current_hp = math::cap_max(end_hp, defender_pkmn.hp.value);

    let attack_fx = if end_hp < attacker_start_hp {
        ActionSideEffects::BasicDamage {
            damaged: attacker.slot_id,
            start_hp: attacker_start_hp,
            end_hp: attacker_pkmn.current_hp,
            cause: Cause::Natural
        }
    } else {
        ActionSideEffects::Healed {
            healed: attacker.slot_id,
            start_hp: attacker_start_hp,
            end_hp: attacker_pkmn.current_hp,
            cause: Cause::Natural
        }
    };

    let defender_fx = if end_hp < defender_start_hp {
        ActionSideEffects::BasicDamage {
            damaged: target.slot_id,
            start_hp: defender_start_hp,
            end_hp: defender_pkmn.current_hp,
            cause: Cause::Natural
        }
    } else {
        ActionSideEffects::Healed {
            healed: target.slot_id,
            start_hp: defender_start_hp,
            end_hp: defender_pkmn.current_hp,
            cause: Cause::Natural
        }
    };

    vec![attack_fx, defender_fx]
}

fn do_baton_pass_effect(attacker: &Slot) -> Vec<ActionSideEffects> {
    if attacker.has_anyone_to_swap_to() {
        vec![ActionSideEffects::PokemonLeftBatonPass(attacker.slot_id, attacker.id())]
    } else {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

fn do_encore_effect(_attacker: &Slot, target: &Slot) -> Vec<ActionSideEffects> {
    let mut target_data = target.data.borrow_mut();
    println!("{:#?}", target_data);
    if target_data.forced_action.is_some() {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    } else {
        match target_data.last_move_used {
            Some(m) if m.can_be_encored() => {
                let pp = target.borrow().get_move_slot(m)
                    .map(|s| s.pp)
                    .unwrap_or(0);
                if pp == 0 {
                    vec![ActionSideEffects::Failed(Cause::Natural)]
                } else {
                    target_data.forced_action = Some(ForcedAction::AttackWithCounter(m, ENCORE_TURN_COUNT));
                    vec![ActionSideEffects::Encore(target.slot_id, m)]
                }
            },
            _ => vec![ActionSideEffects::Failed(Cause::Natural)]
        }
    }
}

fn clear_hazards(attacker: &Slot, side: &RefCell<FieldSide>) -> Vec<ActionSideEffects> {
    let mut removed_anything = false;
    let mut data = attacker.data.borrow_mut();
    let mut side = side.borrow_mut();

    if data.bound.is_some() {
        removed_anything = true; data.bound = None;
    }

    if data.seeded.is_some() {
        removed_anything = true; data.bound = None;
    }

    if side.pointed_stones {
        removed_anything = true; side.pointed_stones = false;
    }

    if side.toxic_spikes > 0 {
        removed_anything = true; side.toxic_spikes = 0;
    }

    if side.spikes > 0 {
        removed_anything = true; side.spikes = 0;
    }

    if removed_anything {
        vec![ActionSideEffects::ClearedHazards]
    } else {
        vec![]
    }
}

pub enum ProxyResult<T> {
    DoNothing,
    ContinueWith(T),
    Interrupt
}

fn proxy_move(attacker: &Slot, attack: Move, effects: &mut Vec<ActionSideEffects>) -> ProxyResult<Move> {
    match attack {
        Move::Metronome => {
            let rand_attack = Move::metronome();
            debug!("{:?} turned into {:?}", attack, rand_attack);
            effects.push(ActionSideEffects::Metronome(attacker.slot_id, rand_attack));
            ContinueWith(rand_attack)
        },
        Move::MirrorMove => {
            let data = attacker.data.borrow_mut();
            if let Some((_, attack)) = data.get_last_targeted_attack() {
                debug!("Mirror Move turned into {:?}", attack);
                ContinueWith(attack)
            } else {
                debug!("Mirror Move failed: no last targeted attack");
                effects.push(ActionSideEffects::Failed(Cause::Natural));
                Interrupt
            }
        },
        Move::SleepTalk => {
            let candidates = attacker.get_effective_known_moves()
                .iter()
                .filter(|m| m.can_be_sleep_talked())
                .map(|m| *m)
                .collect::<Vec<Move>>();
            if candidates.is_empty() {
                debug!("Sleep Talk failed: no sleep-talkable moves known");
                effects.push(ActionSideEffects::Failed(Cause::Natural));
                Interrupt
            } else {
                let selected = if candidates.len() == 1 {
                    candidates.get(0).unwrap()
                } else {
                    candidates.get(rand::thread_rng().gen_range(0..candidates.len())).unwrap()
                };
                debug!("Sleep Talk turned into {:?}", selected);
                effects.push(ActionSideEffects::SleepTalk(attacker.slot_id, *selected));
                ContinueWith(*selected)
            }
        },
        _ => DoNothing
    }
}