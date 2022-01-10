use std::cell::{Ref, RefCell, RefMut};
use std::convert::TryFrom;
use std::ops::{Deref, Index, IndexMut, RangeInclusive};
use std::pin::Pin;

use fraction::{Fraction, ToPrimitive};
use rand::Rng;

use glazed_data::abilities::Ability;
use glazed_data::attack::{Accuracy, BattleStat, DamageType, Effect, EffectPredicate, Move, MoveData, MultiHitFlavor, NonVolatileBattleAilment, Power, StatChangeTarget, Target, VolatileBattleAilment};
use glazed_data::constants::Species;
use glazed_data::constants::UnownForm::P;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::{Gender, PoisonType, Pokemon};
use glazed_data::types::{Effectiveness, PokemonType, Type};
use glazed_core::math;

use crate::*;
use crate::core::{ActionCheck, MoveContext};
use crate::core::CheckResult;

pub fn inflict_confusion(afflicted: &ActivePokemon) -> Vec<ActionSideEffects> {
    if afflicted.get_effective_ability() == Ability::OwnTempo {
        return vec![ActionSideEffects::NoEffectSecondary(Cause::Ability(afflicted.id, Ability::OwnTempo))]
    }

    let mut data = afflicted.data.borrow_mut();
    data.confused = rand::thread_rng().gen_range(CONFUSION_TURN_RANGE);
    vec![ActionSideEffects::Confuse(afflicted.id)]
}

pub(crate) fn _change_stat(affected: &ActivePokemon, stat: BattleStat, stages: i8, cause: Cause) -> Vec<ActionSideEffects> {
    let current = affected.get_stat_stage(stat);
    let next = current + stages;
    return if next > MAX_STAGE {
        vec![ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(affected.id, PokemonState::StatsMaxed(StatsCause::TooHigh)))]
    } else if next < MIN_STAGE {
        vec![ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(affected.id, PokemonState::StatsMaxed(StatsCause::TooLow)))]
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

pub fn change_self_stat(affected: &ActivePokemon, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
    let (ability_cause, stages) = match affected.get_effective_ability() {
        Ability::Simple => (Cause::Ability(affected.id, Ability::Simple), stages * 2),
        Ability::Contrary => (Cause::Ability(affected.id, Ability::Contrary), -stages),
        _ => (Cause::Natural, stages)
    };

    _change_stat(affected, stat, stages, ability_cause)
}

pub fn change_opponent_stat(field: &Battlefield, affecter: &ActivePokemon, affected: &ActivePokemon, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
    let affected_side = field.get_side(&affected.id);

    if affected_side.borrow().mist > 0 {
        return vec![ActionSideEffects::NoEffectSecondary(Cause::PokemonFieldState(FieldCause::Mist))]
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

    _change_stat(affected, stat, stages, ability_cause)
}

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

    /// Do an attack a Pokemon is locked in to (example: charge attack selected the previous turn)
    pub fn do_implicit_attack(&mut self, attacker_id: Battler) -> Vec<ActionSideEffects> {
        let attacker = &self[attacker_id.side][attacker_id.individual];
        let data = attacker.data.borrow();
        if data.recharge {
            drop(data);
            let mut data = attacker.data.borrow_mut();
            data.recharge = false;
            vec![ActionSideEffects::Recharging(Cause::Natural)]
        } else if let Some((defender, attack)) = data.charging {
            drop(data);
            self._do_attack(attacker_id, attack, defender)
        } else if let Some((attack, counter)) = data.thrashing {
            drop(data);
            let mut effects = self._do_attack(attacker_id, attack, SelectedTarget::Implied);
            let counter = counter - 1;
            let damaged = effects.iter().any(|e| e.did_damage());
            if counter == 0 {
                effects.append(&mut inflict_confusion(&self[attacker_id.side][attacker_id.individual]));
            }

            let mut data = attacker.data.borrow_mut();
            if !damaged {
                data.thrashing = None
            } else {
                data.thrashing = Some((attack, counter))
            }
            effects
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
        if turn::do_disable_check(attacker, attack).add(&mut effects) { return effects; }
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
            // Ability-based immunities
            match defender.get_effective_ability() {
                Ability::Soundproof if attack.is_sound_based() && !attacker.get_effective_ability().is_ignore_ability_ability() => {
                    effects.push(ActionSideEffects::NoEffect(Cause::Ability(defender.id, Ability::Soundproof)));
                    continue;
                },
                _ => {}
            }

            // Type-based immunities
            // note that regular type matchups are not calculated here, but instead when damage is applied
            let defender_type = defender.get_effective_type();
            if defender_type.has_type(&Type::Grass) && (attack.is_powder() || attack == Move::LeechSeed) {
                effects.push(ActionSideEffects::NoEffectSecondary(Cause::Type(Type::Grass)))
            } else if defender_type.has_type(&Type::Ghost) && attack.is_trapping() {
                effects.push(ActionSideEffects::NoEffectSecondary(Cause::Type(Type::Ghost)))
            }

            let hit = accuracy::do_accuracy_check(&self, attacker, attack, defender);
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

        effects.append(&mut self._do_damage_and_effects(attacker, targets_hit, attack, move_data));

        if effects.is_empty() {
            effects.push(ActionSideEffects::NothingHappened)
        }

        //region end-of-turn checks
        let mut data = attacker.data.borrow_mut();

        data.set_last_used_move(attack);

        effects.append(&mut data.lower_disable_counters()
            .iter()
            .map(|m| ActionSideEffects::NoLongerDisabled(attacker_id, *m))
            .collect());
        //endregion

        effects
    }

    pub(crate) fn _do_damage_and_effects(&self, attacker: &ActivePokemon, targets_hit: Vec<&ActivePokemon>, attack: Move, move_data: &MoveData) -> Vec<ActionSideEffects> {
        let is_multi_target = targets_hit.len() > 1;
        let mut effects = Vec::new();

        // 3. For each hit target, perform damage
        // For non-damaging moves, "damaged" becomes everyone from Step 2.
        //region Primary Strike
        let targets_for_secondary_damage = if move_data.is_no_power_move() {
            targets_hit
        } else {
            let mut damaged = Vec::new();
            for defender in targets_hit {
                let attacker = &self[attacker.id.side][attacker.id.individual];
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
            let secondary_effect = if let Effect::Predicated(predicate, if_match, if_not_match) = secondary_effect {
                let matches = match predicate {
                    EffectPredicate::Sunny => self.field.borrow().is_sunny()
                };

                if matches {
                    *if_match
                } else {
                    *if_not_match
                }
            } else {
                secondary_effect
            };

            let mut secondary_effects = match secondary_effect {
                Effect::StatChange(stat, stages, probability, StatChangeTarget::User) => {
                    let triggers = self.activates_secondary_effect(attacker, *probability);
                    if triggers {
                        change_self_stat(attacker, *stat, *stages)
                    } else {
                        Vec::new()
                    }
                },
                Effect::StatChange(stat, stages, probability, StatChangeTarget::Target) => {
                    targets_for_secondary_damage.iter()
                        .filter(|_| self.activates_secondary_effect(attacker, *probability))
                        .flat_map(|defender| {
                            if defender.is_behind_substitute() {
                                vec![ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(defender.id, PokemonState::Substituted))]
                            } else {
                                change_opponent_stat(&self, attacker, defender, *stat, *stages)
                            }
                        })
                        .collect()
                },
                Effect::NonVolatileStatus(ailment, probability, StatChangeTarget::User) => {
                    let triggers = self.activates_secondary_effect(attacker, *probability);
                    if triggers {
                        self.inflict_non_volatile_status(attacker, *ailment)
                    } else {
                        Vec::new()
                    }
                },
                Effect::NonVolatileStatus(ailment, probability, StatChangeTarget::Target) => {
                    targets_for_secondary_damage.iter()
                        .filter(|_| self.activates_secondary_effect(attacker, *probability))
                        .flat_map(|defender| {
                            if defender.is_behind_substitute() {
                                vec![ActionSideEffects::Failed(Cause::PokemonBattleState(defender.id, PokemonState::Substituted))]
                            } else {
                                self.inflict_non_volatile_status(defender, *ailment)
                            }
                        })
                        .collect()
                },
                Effect::VolatileStatus(VolatileBattleAilment::Confusion, probability, StatChangeTarget::User) => {
                    let triggers = self.activates_secondary_effect(attacker, *probability);
                    if triggers {
                        inflict_confusion(attacker)
                    } else {
                        Vec::new()
                    }
                },
                Effect::VolatileStatus(VolatileBattleAilment::Confusion, probability, StatChangeTarget::Target) => {
                    targets_for_secondary_damage.iter()
                        .filter(|_| self.activates_secondary_effect(attacker, *probability))
                        .flat_map(|defender| {
                            if defender.is_behind_substitute() {
                                vec![ActionSideEffects::Failed(Cause::PokemonBattleState(defender.id, PokemonState::Substituted))]
                            } else {
                                inflict_confusion(attacker)
                            }
                        })
                        .collect()
                },
                Effect::VolatileStatus(VolatileBattleAilment::Infatuation, _, _) => unimplemented!("No infatuation yet"),
                Effect::ForceSwitch(StatChangeTarget::User) => {
                    vec![ActionSideEffects::ForcePokemonSwap { must_leave: attacker.id }]
                },
                Effect::ForceSwitch(StatChangeTarget::Target) => {
                    targets_for_secondary_damage.iter()
                        .map(|defender| {
                            if defender.is_behind_substitute() {
                                ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(defender.id, PokemonState::Substituted))
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
                                binder: attacker.id,
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
                Effect::Thrash => {
                    let mut data = attacker.data.borrow_mut();
                    if data.thrashing.is_none() {
                        data.thrashing = Some((attack, rand::thread_rng().gen_range(THRASH_RANGE)))
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
                                    ActionSideEffects::Disabled(defender.id, m)
                                }
                            }
                        })
                        .collect()
                },
                Effect::Mist => {
                    let mut side = self.get_side(&attacker.id).borrow_mut();
                    if side.mist == 0 {
                        side.mist = MIST_TURN_COUNT;
                        vec![ActionSideEffects::MistStart(attacker.id.side)]
                    } else {
                        vec![]
                    }
                },
                Effect::Recharge => {
                    let mut data = attacker.data.borrow_mut();
                    data.recharge = true;
                    vec![]
                },
                Effect::Heal(percentage) => {
                    let mut pkmn = attacker.borrow_mut();
                    let start_hp = pkmn.current_hp;
                    let delta = math::ratio(start_hp, *percentage, 100);
                    let end_hp = start_hp.saturating_add(delta).clamp(0, pkmn.hp.value);
                    pkmn.current_hp = end_hp;

                    vec![ActionSideEffects::Healed {
                        healed: attacker.id,
                        start_hp,
                        end_hp,
                        cause: Cause::Natural
                    }]
                },
                Effect::Drain(percentage) => {
                    let mut pkmn = attacker.borrow_mut();
                    let mut effects = Vec::new();
                    for defender in targets_for_secondary_damage.iter() {
                        if let Some((_, _, damage)) = defender.data.borrow().damage_this_turn.last() {
                            let delta =
                                if *damage == 1 {1}
                                else {math::ratio(*damage, *percentage, 100)};
                            let start_hp = pkmn.current_hp;

                            if defender.get_effective_ability() == Ability::LiquidOoze {
                                let end_hp = start_hp.saturating_sub(delta);
                                pkmn.current_hp = end_hp;
                                effects.push(ActionSideEffects::BasicDamage {
                                    damaged: attacker.id,
                                    start_hp,
                                    end_hp,
                                    cause: Cause::Ability(defender.id, Ability::LiquidOoze)
                                })
                            } else {
                                let end_hp = start_hp.saturating_add(delta).clamp(0, pkmn.hp.value);
                                pkmn.current_hp = end_hp;
                                effects.push(ActionSideEffects::Healed {
                                    healed: attacker.id,
                                    start_hp,
                                    end_hp,
                                    cause: Cause::Move(attacker.id, attack)
                                })
                            }
                        }
                    }
                    effects
                },
                Effect::Leech => {
                    targets_for_secondary_damage.iter()
                        .map(|defender| {
                            let mut data = defender.data.borrow_mut();
                            if data.seeded.is_none() {
                                data.seeded = Some(attacker.id);
                                ActionSideEffects::SeedStart {
                                    from: defender.id,
                                    to: attacker.id
                                }
                            } else {
                                ActionSideEffects::Failed(Cause::Natural)
                            }
                        })
                        .collect()
                },
                Effect::Rage => {
                    let mut data = attacker.data.borrow_mut();
                    if data.enraged {
                        vec![ActionSideEffects::RageContinue(attacker.id)]
                    } else {
                        data.enraged = true;
                        vec![ActionSideEffects::RageStart(attacker.id)]
                    }
                },
                Effect::Mimic => {
                    let target = targets_for_secondary_damage.first();
                    let effect = match target {
                        None => ActionSideEffects::NoTarget,
                        Some(defender) => {
                            let data = defender.data.borrow();
                            match data.last_move_used {
                                None => ActionSideEffects::NoEffectSecondary(Cause::Natural),
                                Some(m) => {
                                    if m.can_be_mimicked() && !attacker.borrow().knows_move(m) {
                                        attacker.replace_mimic_with(m);
                                        ActionSideEffects::Mimicked(attacker.id, m)
                                    } else {
                                        ActionSideEffects::Failed(Cause::Natural)
                                    }
                                }
                            }
                        }
                    };
                    vec![effect]
                },
                Effect::ChangeWeather(_) => unimplemented!("No Weather Yet"),
                Effect::DispelWeather => unimplemented!("No Weather Yet"),
                Effect::Predicated(_, _, _) => panic!("Nested predicated not supported"),
                Effect::Custom => panic!("Consider changing from Custom to a concrete effect"),
            };
            effects.append(&mut secondary_effects);
        }

        {
            let mut data = attacker.data.borrow_mut();

            // Rage ends if the user is enraged, and they don't use a rage move
            if data.enraged && !move_data.is_rage() {
                data.enraged = false;
                effects.push(ActionSideEffects::RageEnd(attacker.id))
            }
        }
        //endregion

        effects
    }

    fn activates_secondary_effect(&self, attacker: &ActivePokemon, probability: u8) -> bool {
        let probability = if let Ability::SereneGrace = attacker.get_effective_ability() {
            probability * 2
        } else {
            probability
        };
        if probability == 0 || probability > 100 {
            true
        } else {
            rand::thread_rng().gen_bool(f64::from(probability) / 100f64)
        }
    }

    //region Non-Damage

    fn inflict_non_volatile_status(&self, affected: &ActivePokemon, status: NonVolatileBattleAilment) -> Vec<ActionSideEffects> {
        {
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
            affected: affected.id,
            status
        }]
    }
    //endregion
}