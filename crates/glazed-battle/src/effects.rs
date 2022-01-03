use std::cell::{Ref, RefCell, RefMut};
use std::convert::TryFrom;
use std::ops::{Deref, Index, IndexMut, RangeInclusive};
use std::pin::Pin;

use fraction::{Fraction, ToPrimitive};
use rand::Rng;

use glazed_data::abilities::Ability;
use glazed_data::attack::{Accuracy, BattleStat, DamageType, Effect, Move, MoveData, MultiHitFlavor, NonVolatileBattleAilment, Power, StatChangeTarget, Target, VolatileBattleAilment};
use glazed_data::constants::Species;
use glazed_data::constants::UnownForm::P;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::{Gender, Pokemon};
use glazed_data::types::{Effectiveness, PokemonType, Type};

use crate::*;
use crate::core::{ActionCheck, MoveContext};
use crate::core::CheckResult;

impl Battlefield {
    /// Perform a regular attack
    pub fn do_attack(&mut self, attacker_id: Battler, attack: Move, defender: SelectedTarget) -> Vec<ActionSideEffects> {
        let move_data = attack.data();

        if let Power::BaseWithCharge(_, place) = move_data.power {
            let attacker = &self[attacker_id.side][attacker_id.individual];
            let mut data = attacker.data.borrow_mut();

            if let Some(s) = place {
                data.invulnerable = Some(s);
            }

            data.charging = Some((defender, attack));

            vec![ActionSideEffects::Charging(attacker_id, attack)]
        } else {
            self._do_attack(attacker_id, attack, defender)
        }
    }

    /// Complete a charge attack
    pub fn complete_charge(&mut self, attacker_id: Battler) -> Vec<ActionSideEffects> {
        let attacker = &self[attacker_id.side][attacker_id.individual];
        let data = attacker.data.borrow();
        if let Some((defender, attack)) = data.charging {
            drop(data);
            self._do_attack(attacker_id, attack, defender)
        } else {
            vec![]
        }
    }

    /// Do all the end-of-turn things
    pub fn end_of_turn(&mut self) -> Vec<ActionSideEffects> {
        let everyone = self.get_everyone();
        let mut effects = Vec::new();
        for pokemon in everyone {
            effects.append(&mut turn::do_binding_damage(pokemon))
        }
        effects
    }

    /// Internal method to perform an attack
    fn _do_attack(&self, attacker_id: Battler, attack: Move, defender: SelectedTarget) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let attacker = &self[attacker_id.side][attacker_id.individual];

        let move_data = attack.data();

        // Check for reasons this Pokemon can't perform this move
        //region start-of-turn checks
        if turn::do_freeze_check(attacker, attack).add(&mut effects) { return effects; }
        if turn::do_sleep_check(attacker, attack).add(&mut effects) { return effects; }
        if turn::do_paralysis_check(attacker).add(&mut effects) { return effects; }
        if turn::do_flinch_check(attacker).add(&mut effects) { return effects; }
        if turn::do_confusion_check(attacker).add(&mut effects) { return effects; }
        //endregion

        // 1. Get the target(s)
        let targets = self.get_targets(attacker.id, attack, defender);
        if targets.is_empty() {
            effects.push(ActionSideEffects::NoTarget);
            return effects;
        }

        // 2. For each target, determine if the move hits
        //region Accuracy Check
        let mut targets_hit = Vec::new();
        for defender in targets {
            let hit = accuracy::do_accuracy_check(attacker, attack, defender);
            match hit {
                Ok(b) => {
                    if b {
                        targets_hit.push(defender);
                    } else {
                        effects.push(ActionSideEffects::Missed(defender.id, Cause::Natural));
                        if let Power::BaseWithCrash(_) = move_data.power {
                            effects.push(attacker.take_crash_damage());
                        }
                    }
                }
                Err(a) => {
                    effects.push(a);
                }
            }
        }

        if targets_hit.is_empty() {
            return effects;
        }
        //endregion

        let is_multi_target = targets_hit.len() > 1;

        // 3. For each hit target, perform damage
        // For non-damaging moves, "damaged" becomes everyone from Step 2.
        //region Primary Strike
        let targets_for_secondary_damage = if move_data.is_no_power_move() {
            targets_hit
        } else {
            let mut damaged = Vec::new();
            for defender in targets_hit {
                let attacker = &self[attacker_id.side][attacker_id.individual];
                let mut individual_effects = self.do_damage(attacker, attack, defender, is_multi_target);

                let hit_substitute = individual_effects.iter().any(|e| e.hit_substitute());
                let defender_fainted = defender.borrow().is_fainted();

                effects.append(&mut individual_effects);

                if !defender_fainted && !hit_substitute {
                    damaged.push(defender);
                }
            }
            damaged
        };
        //endregion

        // 4. Perform secondary effects.
        // Effects to the user happen regardless.
        // Effects to the target happen only to those hit in step 3.
        //region Secondary effects
        for secondary_effect in move_data.effects {
            let mut secondary_effects = match secondary_effect {
                Effect::StatChange(stat, stages, probability, StatChangeTarget::User) => {
                    let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
                    if triggers {
                        self.change_self_stat(attacker, *stat, *stages)
                    } else {
                        Vec::new()
                    }
                },
                Effect::StatChange(stat, stages, probability, StatChangeTarget::Target) => {
                    let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
                    if triggers {
                        targets_for_secondary_damage.iter()
                            .flat_map(|defender| {
                                if defender.is_behind_substitute() {
                                    vec![ActionSideEffects::NoEffectSecondary(Cause::Substitute(defender.id))]
                                } else {
                                    self.change_opponent_stat(attacker, defender, *stat, *stages)
                                }
                            })
                            .collect()
                    } else {
                        Vec::new()
                    }
                },
                Effect::NonVolatileStatus(ailment, probability, StatChangeTarget::User) => {
                    let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
                    if triggers {
                        self.inflict_non_volatile_status(attacker, *ailment)
                    } else {
                        Vec::new()
                    }
                },
                Effect::NonVolatileStatus(ailment, probability, StatChangeTarget::Target) => {
                    let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
                    if triggers {
                        targets_for_secondary_damage.iter()
                            .flat_map(|defender| {
                                if defender.is_behind_substitute() {
                                    vec![ActionSideEffects::Failed(Cause::Substitute(defender.id))]
                                } else {
                                    self.inflict_non_volatile_status(defender, *ailment)
                                }
                            })
                            .collect()
                    } else {
                        Vec::new()
                    }
                },
                Effect::ForceSwitch(StatChangeTarget::User) => {
                    vec![ActionSideEffects::ForcePokemonSwap { must_leave: attacker_id }]
                },
                Effect::ForceSwitch(StatChangeTarget::Target) => {
                    targets_for_secondary_damage.iter()
                        .map(|defender| {
                            if defender.is_behind_substitute() {
                                ActionSideEffects::NoEffectSecondary(Cause::Substitute(defender.id))
                            } else if defender.get_effective_ability() == Ability::SuctionCups {
                                ActionSideEffects::NoEffectSecondary(Cause::Ability(defender.id, Ability::SuctionCups))
                            } else if defender.data.borrow().rooted {
                                ActionSideEffects::NoEffectSecondary(Cause::Move(defender.id, Move::Ingrain))
                            } else {
                                ActionSideEffects::ForcePokemonSwap { must_leave: defender.id }
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
                                binder: attacker_id,
                                bound: defender.id,
                                turns: turns_bound,
                                attack
                            }
                        })
                        .collect()
                }
                Effect::Flinch(probability) => {
                    targets_for_secondary_damage.iter()
                        .filter_map(|defender| {
                            if defender.is_behind_substitute() {
                                None
                            } else {
                                let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
                                if triggers {
                                    defender.data.borrow_mut().flinch = true;
                                    Some(ActionSideEffects::WillFlinch(defender.id))
                                } else {
                                    None
                                }
                            }
                        })
                        .collect()
                },
        //         Effect::VolatileStatus(ailment, probability, _) => {
        //             let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
        //             if triggers {
        //                 self.inflict_volatile_status(defender, attacker, *ailment)
        //             } else {
        //                 Vec::new()
        //             }
        //         }
                _ => unimplemented!("Secondary effect not yet created")
            };
            effects.append(&mut secondary_effects);
        }
        //endregion

        if effects.is_empty() {
            effects.push(ActionSideEffects::NothingHappened)
        }

        effects
    }

    //region Non-Damage
    fn _change_stat(&self, affected: &ActivePokemon, stat: BattleStat, stages: i8, cause: Cause) -> Vec<ActionSideEffects> {
        let current = affected.get_stat_stage(stat);
        let next = current + stages;
        return if next > MAX_STAGE {
            vec![ActionSideEffects::NoEffectSecondary(Cause::StatsMaxed(StatsCause::TooHigh))]
        } else if next < MIN_STAGE {
            vec![ActionSideEffects::NoEffectSecondary(Cause::StatsMaxed(StatsCause::TooLow))]
        } else {
            let mut data = affected.data.borrow_mut();
            match stat {
                BattleStat::Attack => data.attack_stage = next,
                BattleStat::Defense => data.defense_stage = next,
                BattleStat::SpecialAttack => data.special_attack_stage = next,
                BattleStat::SpecialDefense => data.special_defense_stage = next,
                BattleStat::Speed => data.speed_stage = next,
                BattleStat::Accuracy => data.accuracy_stage = next,
                BattleStat::Evasion => data.evasion_stage = next,
            };
            vec![ActionSideEffects::StatChanged {
                stat,
                affected: affected.id,
                cause,
                start: current,
                end: next
            }]
        }
    }

    fn change_self_stat(&self, affected: &ActivePokemon, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
        let (ability_cause, stages) = match affected.get_effective_ability() {
            Ability::Simple => (Cause::Ability(affected.id, Ability::Simple), stages * 2),
            Ability::Contrary => (Cause::Ability(affected.id, Ability::Contrary), -stages),
            _ => (Cause::Natural, stages)
        };

        self._change_stat(affected, stat, stages, ability_cause)
    }

    fn change_opponent_stat(&self, affecter: &ActivePokemon, affected: &ActivePokemon, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
        let affected_side = self.get_side(&affected.id);

        if affected_side.mist > 0 {
            return vec![ActionSideEffects::NoEffectSecondary(Cause::Field(FieldCause::Mist))]
        }

        let (ability_cause, stages) = match affected.get_effective_ability() {
            Ability::Simple => (Cause::Ability(affected.id, Ability::Simple), stages * 2),
            Ability::Contrary  => {
                let cause = Cause::Ability(affected.id, Ability::Contrary);
                match affected.get_effective_ability() {
                    oc if oc.is_ignore_ability_ability() => (cause.overwrite(Cause::Ability(affecter.id, oc)), -stages),
                    _ => (cause, stages)
                }
            },
            oc@ (Ability::ClearBody | Ability::WhiteSmoke) => {
                let cause = Cause::Ability(affected.id, oc);
                match affected.get_effective_ability() {
                    oc if oc.is_ignore_ability_ability() => (cause.overwrite(Cause::Ability(affecter.id, oc)), stages),
                    _ => return vec![ActionSideEffects::NoEffectSecondary(Cause::Ability(affected.id, oc))]
                }
            }
            _ => (Cause::Natural, stages)
        };

        self._change_stat(affected, stat, stages, ability_cause)
    }

    fn inflict_non_volatile_status(&self, affected: &ActivePokemon, status: NonVolatileBattleAilment) -> Vec<ActionSideEffects> {
        {
            let mut affected = affected.borrow_mut();
            match status {
                NonVolatileBattleAilment::Paralysis => affected.status.paralysis = true,
                NonVolatileBattleAilment::Sleep => affected.status.sleep = rand::thread_rng().gen_range(SLEEP_TURNS_RANGE),
                NonVolatileBattleAilment::Freeze => affected.status.freeze = true,
                NonVolatileBattleAilment::Burn => affected.status.burn = true,
                NonVolatileBattleAilment::Poison(a) => affected.status.poison = Some(a)
            }
        }
        vec![ActionSideEffects::NonVolatileStatusAilment {
            affected: affected.id,
            status
        }]
    }

    fn inflict_volatile_status(&self, affected: &mut ActivePokemon, inflicter: &mut ActivePokemon, status: VolatileBattleAilment) -> Vec<ActionSideEffects> {
        match status {
            VolatileBattleAilment::Confusion => {
                if let Ability::OwnTempo = affected.get_effective_ability() {
                    vec![ActionSideEffects::NoEffectSecondary(Cause::Ability(affected.id, Ability::OwnTempo))]
                } else {
                    affected.data.borrow_mut().confused = rand::thread_rng().gen_range(2..=5);
                    vec![ActionSideEffects::Confuse(affected.id)]
                }
            }
            VolatileBattleAilment::Infatuation => {
                if affected.borrow().gender == Gender::None || affected.borrow().gender == inflicter.borrow().gender {
                    vec![ActionSideEffects::NoEffectSecondary(Cause::Natural)]
                } else if affected.get_effective_ability() == Ability::Oblivious {
                    vec![ActionSideEffects::NoEffectSecondary(Cause::Ability(affected.id, Ability::Oblivious))]
                } else {
                    affected.data.borrow_mut().infatuated = true;
                    vec![ActionSideEffects::Infatuated(affected.id)]
                }
            }
            VolatileBattleAilment::Levitation => {
                affected.data.borrow_mut().levitating = 5;
                vec![]
            }
        }
    }

    //endregion
}