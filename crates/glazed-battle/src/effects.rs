use std::cell::RefMut;
use std::convert::TryFrom;
use std::mem::take;

use rand::Rng;
use strum::IntoEnumIterator;

use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, Effect, EffectPredicate, EntryHazardType, Move, MoveData, NonVolatileBattleAilment, Power, ScreenType, StatChangeTarget, VolatileBattleAilment};
use glazed_data::constants::Species;
use glazed_data::item::Item;
use glazed_data::types::{Effectiveness, PokemonType, Type};
use glazed_core::math;

use crate::*;
use crate::core::ActionCheck;

pub fn inflict_confusion(afflicted: &ActivePokemon) -> Vec<ActionSideEffects> {
    if afflicted.get_effective_ability() == Ability::OwnTempo {
        return vec![ActionSideEffects::NoEffectSecondary(Cause::Ability(afflicted.id, Ability::OwnTempo))]
    }

    let mut data = afflicted.data.borrow_mut();
    data.confused = rand::thread_rng().gen_range(CONFUSION_TURN_RANGE);
    vec![ActionSideEffects::Confuse(afflicted.id)]
}

fn _change_stat(affected: &ActivePokemon, stat: BattleStat, stages: i8, cause: Cause) -> ActionSideEffects {
    let current = affected.get_stat_stage(stat);
    let next = current + stages;
    return if next > MAX_STAGE {
        ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(affected.id, PokemonState::StatsMaxed(StatsCause::TooHigh)))
    } else if next < MIN_STAGE {
        ActionSideEffects::NoEffectSecondary(Cause::PokemonBattleState(affected.id, PokemonState::StatsMaxed(StatsCause::TooLow)))
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
            BattleStat::CriticalHitRatio => data.crit_stage = if next < 0 { 0 } else { u8::try_from(next).unwrap() }
        };
        ActionSideEffects::StatChanged {
            stat,
            affected: affected.id,
            cause,
            start: current,
            end: next
        }
    }
}

pub fn change_self_stat(affected: &ActivePokemon, stat: BattleStat, stages: i8) -> ActionSideEffects {
    let (ability_cause, stages) = match affected.get_effective_ability() {
        Ability::Simple => (Cause::Ability(affected.id, Ability::Simple), stages * 2),
        Ability::Contrary => (Cause::Ability(affected.id, Ability::Contrary), -stages),
        _ => (Cause::Natural, stages)
    };

    _change_stat(affected, stat, stages, ability_cause)
}

pub fn change_opponent_stat(field: &Battlefield, affecter: &ActivePokemon, affected: &ActivePokemon, stat: BattleStat, stages: i8) -> ActionSideEffects {
    let affected_side = field.get_side(&affected.id);

    if affected_side.borrow().mist > 0 {
        return ActionSideEffects::NoEffectSecondary(Cause::PokemonFieldState(FieldCause::Mist))
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
                _ => return ActionSideEffects::NoEffectSecondary(Cause::Ability(affected.id, oc))
            }
        }
        _ => (Cause::Natural, stages)
    };

    _change_stat(affected, stat, stages, ability_cause)
}

pub fn inflict_non_volatile_status(affected: &ActivePokemon, status: NonVolatileBattleAilment) -> Vec<ActionSideEffects> {
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

impl Battlefield {
    /// Perform a regular attack
    pub fn do_attack(&mut self, attacker_id: Battler, attack: Move, defender: SelectedTarget) -> Vec<ActionSideEffects> {
        let attacker = &self[attacker_id.side][attacker_id.individual];
        attacker.data.borrow_mut().set_last_used_move(attack);

        let move_data = attack.data();

        if attack == Move::Metronome {
            let rand_attack = Move::metronome();
            let mut effects = vec![ActionSideEffects::Metronome(attacker_id, rand_attack)];
            effects.append(&mut self._do_attack(attacker_id, rand_attack, SelectedTarget::Implied));
            effects
        } else if attack == Move::MirrorMove {
            let attacker = &self[attacker_id.side][attacker_id.individual];
            let data = attacker.data.borrow_mut();
            if let Some((_, attack)) = data.get_last_targeted_attack() {
                drop(data);
                self._do_attack(attacker_id, attack, SelectedTarget::Implied)
            } else {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            }
        } else if let Power::BaseWithCharge(_, place) = move_data.power {
            let attacker = &self[attacker_id.side][attacker_id.individual];

            let mut effects = vec![ActionSideEffects::Charging(attacker_id, attack)];

            // Skull Bash has a unique effect where it raises user defense on its charging turn
            if attack == Move::SkullBash {
                let effect = change_self_stat(attacker, BattleStat::Defense, 1);
                if let ActionSideEffects::NoEffectSecondary(_) = effect { }
                else {
                    effects.push(effect);
                }
            }

            let mut data = attacker.data.borrow_mut();

            if let Some(s) = place {
                data.invulnerable = Some(s);
            }

            data.charging = Some((defender, attack));

            effects
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
            let effects = self._do_attack(attacker_id, attack, defender);
            attacker.data.borrow_mut().invulnerable = None;
            effects
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
        } else if let Some((counter, damage)) = data.bide {
            let counter = counter - 1;
            if counter == 0 {
                let damage = damage.saturating_mul(2);
                if let Some((defender, _)) = attacker.data.borrow().last_attacker {
                    let defender = &self[defender.side][defender.individual];
                    self.do_bide_damage(attacker, damage, defender)
                } else {
                    vec![ActionSideEffects::NoTarget]
                }
            } else {
                vec![ActionSideEffects::BideContinue(attacker_id)]
            }
        } else {
            vec![]
        }
    }

    /// Do all the end-of-turn things
    pub fn end_of_round(&mut self) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let both_sides = vec![&self.user_side, &self.opponent_side];
        for side in both_sides {
            effects.append(&mut turn::do_screen_countdown(side));
        }

        let everyone = self.get_everyone();
        for pokemon in everyone {
            pokemon.data.borrow_mut().end_of_turn();

            if pokemon.borrow().is_fainted() { continue; }
            effects.append(&mut turn::do_poison_damage(pokemon));
            if pokemon.borrow().is_fainted() { continue; }
            effects.append(&mut turn::do_binding_damage(pokemon));
            if pokemon.borrow().is_fainted() { continue; }
            effects.append(&mut turn::do_nightmare_damage(pokemon));
            if pokemon.borrow().is_fainted() { continue; }
            effects.append(&mut turn::do_curse_damage(pokemon));
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

        // 0. Test if the attacker can use the move
        match accuracy::cannot_use_attack(attacker, attack) {
            Ok(b) => {
                if !b { effects.push(ActionSideEffects::Failed(Cause::Natural)); return effects; }
            }
            Err(effect) => {
                effects.push(effect); return effects
            }
        }

        // 1. Get the target(s)
        let targets = self.get_targets(attacker.id, attack, defender);
        if targets.is_empty() {
            effects.push(ActionSideEffects::NoTarget);
            return effects;
        }

        for t in targets.iter() {
            t.data.borrow_mut().targeted_by(attacker, attack);
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
                        if attack.is_protection_move() {
                            attacker.data.borrow_mut().protection_counter = 0;
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
        // For non-damaging moves, all valid targets go on to Step 4.
        //region Primary Strike
        let targets_for_secondary_damage= {
            let mut damaged = Vec::new();
            for defender in targets_hit {
                match accuracy::cannot_use_attack_against(attacker, attack, defender) {
                    Ok(b) => {
                        if !b {
                            effects.push(ActionSideEffects::Failed(Cause::Natural));
                            continue;
                        } else {}
                    }
                    Err(e) => {
                        effects.push(e);
                        continue;
                    }
                }

                if move_data.is_no_power_move() {
                    if defender.is_behind_substitute() && !attack.bypasses_substitute() {
                        effects.push(ActionSideEffects::Failed(Cause::Natural));
                        continue;
                    } else {
                        damaged.push(defender);
                    }
                } else {
                    let attacker = &self[attacker.id.side][attacker.id.individual];
                    let mut individual_effects = self.do_damage(attacker, attack, defender, is_multi_target);

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
                        vec![change_self_stat(attacker, *stat, *stages)]
                    } else {
                        Vec::new()
                    }
                },
                Effect::StatChange(stat, stages, probability, StatChangeTarget::Target) => {
                    let do_stat_change = |attacker: &ActivePokemon, defender: &ActivePokemon|
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
                            if defender.is_behind_substitute() {
                                vec![ActionSideEffects::Failed(Cause::PokemonBattleState(defender.id, PokemonState::Substituted))]
                            } else {
                                inflict_non_volatile_status(defender, *ailment)
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
                Effect::Flinch(probability) => self.do_probable_effect_on_all_targets(attacker, &targets_for_secondary_damage, *probability, do_flinch_effect),
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
                Effect::Heal(percentage) => do_heal_effect(attacker, *percentage),
                Effect::Leech => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_leech_effect),
                Effect::Rage => {
                    let mut data = attacker.data.borrow_mut();
                    if data.enraged {
                        vec![ActionSideEffects::RageContinue(attacker.id)]
                    } else {
                        data.enraged = true;
                        vec![ActionSideEffects::RageStart(attacker.id)]
                    }
                },
                Effect::Mimic => do_effect_on_last_target(attacker, &targets_for_secondary_damage, do_mimic_effect),
                Effect::ChangeWeather(_) => unimplemented!("No Weather Yet"),
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
                    let mut side = self.get_side(&attacker.id).borrow_mut();
                    let turn_count = if attacker.borrow().is_holding(Item::LightClay) {
                        SCREEN_TURN_COUNT_LIGHT_CLAY
                    } else {
                        SCREEN_TURN_COUNT
                    };

                    do_screen_effect(&mut side, screen, turn_count)
                },
                Effect::StatReset => do_stat_reset_effect(&targets_for_secondary_damage),
                Effect::Bide => {
                    let mut data = attacker.data.borrow_mut();
                    data.bide = Some((BIDE_TURN_COUNT, 0));
                    vec![ActionSideEffects::BideStart(attacker.id)]
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
                        Some(t) => do_hazard_effect(self.get_side(&t.id), eh)
                    }
                }
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

            // Increment the protection counter for any protection moves. Reset it if anything else is used.
            if attack.is_protection_move() {
                data.protection_counter = data.protection_counter.saturating_add(1);
            } else {
                data.protection_counter = 0;
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

    fn do_probable_effect_on_all_targets<F>(&self, attacker: &ActivePokemon, targets: &Vec<&ActivePokemon>, probability: u8, effect: F) -> Vec<ActionSideEffects>
        where F: Fn(&ActivePokemon, &ActivePokemon) -> Vec<ActionSideEffects>
    {
        targets.iter()
            .filter(|_| self.activates_secondary_effect(attacker, probability))
            .flat_map(|target| effect(attacker, *target))
            .collect()
    }
}

fn do_effect_on_all_targets<F>(attacker: &ActivePokemon, to_effect: &Vec<&ActivePokemon>, effect: F) -> Vec<ActionSideEffects>
where F: Fn(&ActivePokemon, &ActivePokemon) -> Vec<ActionSideEffects>
{
    to_effect.iter()
        .flat_map(|pkmn| effect(attacker, *pkmn))
        .collect()
}

fn do_effect_on_last_target<F>(attacker: &ActivePokemon, to_effect: &Vec<&ActivePokemon>, effect: F) -> Vec<ActionSideEffects>
    where F: Fn(&ActivePokemon, &ActivePokemon) -> Vec<ActionSideEffects>
{
    match to_effect.last() {
        Some(t) => effect(attacker, *t),
        None => vec![ActionSideEffects::NoTarget]
    }
}

fn do_optional_effect_on_last_target<F>(attacker: &ActivePokemon, to_effect: &Vec<&ActivePokemon>, effect: F) -> Vec<ActionSideEffects>
    where F: Fn(&ActivePokemon, &ActivePokemon) -> Vec<ActionSideEffects>
{
    match to_effect.last() {
        Some(t) => effect(attacker, *t),
        None => vec![]
    }
}

fn do_flinch_effect(_attacker: &ActivePokemon, defender: &ActivePokemon) -> Vec<ActionSideEffects> {
    if defender.is_behind_substitute() {
        vec![]
    } else {
        defender.data.borrow_mut().flinch = true;
        vec![ActionSideEffects::WillFlinch(defender.id)]
    }
}

fn do_heal_effect(attacker: &ActivePokemon, percentage: u8) -> Vec<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    let start_hp = pkmn.current_hp;
    let delta = math::ratio(start_hp, percentage, 100);
    let end_hp = start_hp.saturating_add(delta).clamp(0, pkmn.hp.value);
    pkmn.current_hp = end_hp;

    vec![ActionSideEffects::Healed {
        healed: attacker.id,
        start_hp,
        end_hp,
        cause: Cause::Natural
    }]
}

fn do_leech_effect(attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    let mut data = target.data.borrow_mut();
    let effect = if data.seeded.is_none() {
        data.seeded = Some(attacker.id);
        ActionSideEffects::SeedStart {
            from: target.id,
            to: attacker.id
        }
    } else {
        ActionSideEffects::Failed(Cause::Natural)
    };
    vec![effect]
}

fn do_mimic_effect(attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    let data = target.data.borrow();
    let effect = match data.last_move_used {
        None => ActionSideEffects::NoEffectSecondary(Cause::Natural),
        Some(m) => {
            if m.can_be_mimicked() && !attacker.borrow().knows_move(m) {
                attacker.replace_mimic_with(m);
                ActionSideEffects::Mimicked(attacker.id, m)
            } else {
                ActionSideEffects::Failed(Cause::Natural)
            }
        }
    };
    vec![effect]
}

fn do_screen_effect(side: &mut RefMut<Side>, screen: &ScreenType, turns: u8) -> Vec<ActionSideEffects> {
    let effect = match screen {
        ScreenType::LightScreen => {
            if side.light_screen > 0 {
                ActionSideEffects::NoEffectSecondary(Cause::Natural)
            } else {
                side.light_screen = turns;
                ActionSideEffects::ScreenStart(side.id, ScreenType::LightScreen)
            }
        }
        ScreenType::Reflect => {
            if side.reflect > 0 {
                ActionSideEffects::NoEffectSecondary(Cause::Natural)
            } else {
                side.reflect = turns;
                ActionSideEffects::ScreenStart(side.id, ScreenType::Reflect)
            }
        }
    };
    vec![effect]
}

fn do_stat_reset_effect(targets: &Vec<&ActivePokemon>) -> Vec<ActionSideEffects> {
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
        effects.push(ActionSideEffects::StatsReset(pkmn.id));
    }
    effects
}

fn do_rest_effect(attacker: &ActivePokemon) -> Vec<ActionSideEffects> {
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
        vec![ActionSideEffects::Sleep(attacker.id), ActionSideEffects::Healed {
            healed: attacker.id,
            start_hp,
            end_hp: pkmn.current_hp,
            cause: Cause::Natural
        }]
    }
}

fn do_conversion_effect(attacker: &ActivePokemon) -> Vec<ActionSideEffects> {
    let slot = loop {
        if let Some(slot) = attacker.get_effective_move(rand::thread_rng().gen_range(0..4)) {
            break slot;
        }
    };
    let new_type = slot.attack.data()._type;
    attacker.data.borrow_mut().temp_type = Some(PokemonType::Single(new_type));
    vec![ActionSideEffects::ChangeType(attacker.id, new_type)]
}

fn do_conversion_2_effect(attacker: &ActivePokemon) -> Vec<ActionSideEffects> {
    let mut data = attacker.data.borrow_mut();
    if let Some((_, attack)) = data.last_attacker {
        let attack_type = attack.data()._type;
        let candidates: Vec<Type> = Type::iter()
            .filter(|t: &Type| {
                match t.attacking(&attack_type) {
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
        vec![ActionSideEffects::ChangeType(attacker.id, *new_type)]
    } else {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

fn do_tri_attack_effect(_attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
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

fn do_substitute_effect(attacker: &ActivePokemon) -> Vec<ActionSideEffects> {
    let mut pkmn = attacker.borrow_mut();
    let substitute_hp = pkmn.hp.value / 4;
    if pkmn.current_hp <= substitute_hp {
        vec![ActionSideEffects::Failed(Cause::PokemonBattleState(attacker.id, PokemonState::TooWeak))]
    } else {
        let (start_hp, end_hp) = pkmn.subtract_hp(substitute_hp);
        let mut data = attacker.data.borrow_mut();
        data.substituted = substitute_hp + 1;
        vec![ActionSideEffects::BasicDamage {
            damaged: attacker.id,
            start_hp,
            end_hp,
            cause: Cause::PokemonBattleState(attacker.id, PokemonState::Substituted)
        }, ActionSideEffects::CreatedSubstitute(attacker.id)]
    }
}

fn do_sketch_effect(attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    let target_data = target.data.borrow();
    match target_data.last_move_used {
        Some(m) => {
            if attacker.borrow().knows_move(m) {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            } else {
                attacker.replace_sketch_with(m);
                vec![ActionSideEffects::Sketched {
                    user: attacker.id,
                    target: target.id,
                    attack: m
                }]
            }
        },
        None => vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

fn do_transform_effect(attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    let transform = TransformData::from(target);
    let target = target.borrow();
    attacker.data.borrow_mut().transformed = Some(transform);
    vec![ActionSideEffects::Transform {
        id: attacker.id,
        species: target.species,
        gender: target.gender,
        shiny: target.is_shiny()
    }]
}

fn do_steal_item_effect(attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    let mut attacking_pkmn = attacker.borrow_mut();
    if attacking_pkmn.held_item.is_some() {
        // We can't steal when holding a held item, so we do nothing.
        return vec![ActionSideEffects::CouldntStealItem {
            from: attacker.id,
            to: target.id,
            cause: Cause::PokemonBattleState(attacker.id, PokemonState::HoldingItem)
        }]
    }

    if target.get_effective_ability() == Ability::StickyHold {
        return vec![ActionSideEffects::CouldntStealItem {
            from: attacker.id,
            to: target.id,
            cause: Cause::Ability(attacker.id, Ability::StickyHold)
        }]
    }

    let mut target_pkmn = target.borrow_mut();
    let can_steal = match &target_pkmn.held_item {
        None => {
            return vec![ActionSideEffects::CouldntStealItem {
                from: attacker.id,
                to: target.id,
                cause: Cause::PokemonBattleState(target.id, PokemonState::NotHoldingItem)
            }]
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
            from: target.id,
            to: attacker.id,
            item: attacking_pkmn.held_item.as_ref().unwrap().clone()
        }]
    } else {
        vec![ActionSideEffects::CouldntStealItem {
            from: attacker.id,
            to: target.id,
            cause: Cause::Natural
        }]
    }
}

fn do_trap_effect(_attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    target.data.borrow_mut().trapped = true;
    vec![ActionSideEffects::TrappedStart(target.id)]
}

fn do_lock_on_effect(attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    target.data.borrow_mut().locked_on = Some((2, target.id));
    vec![ActionSideEffects::LockedOn { user: attacker.id, target: target.id }]
}

fn do_nightmare_effect(_attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    if target.borrow().status.sleep > 0 {
        target.data.borrow_mut().nightmare = true;
        vec![ActionSideEffects::Nightmare(target.id)]
    } else {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

fn do_ghost_curse_effect(_attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    target.data.borrow_mut().cursed = true;
    vec![ActionSideEffects::Curse(target.id)]
}

fn do_spite_effect(_attacker: &ActivePokemon, target: &ActivePokemon) -> Vec<ActionSideEffects> {
    if let Some(m) = target.data.borrow().last_move_used {
        let mut pkmn = target.borrow_mut();
        if let Some(slot) = pkmn.get_move_slot_mut(m) {
            let pp_start = slot.pp;
            if pp_start == 0 {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            } else {
                let pp_end = pp_start.saturating_sub(SPITE_PP_COUNT);
                slot.pp = pp_end;
                vec![ActionSideEffects::LostPP(target.id, slot.attack, pp_start, pp_end)]
            }
        } else {
            println!("Here");
            vec![ActionSideEffects::Failed(Cause::Natural)]
        }
    } else {
        vec![ActionSideEffects::Failed(Cause::Natural)]
    }
}

fn do_protect_effect(attacker: &ActivePokemon, attack: Move) -> Vec<ActionSideEffects> {
    let mut data = attacker.data.borrow_mut();
    data.protected = Some(attack);
    vec![ActionSideEffects::StartProtection(attacker.id, attack)]
}

fn do_belly_drum_effect(attacker: &ActivePokemon) -> Vec<ActionSideEffects> {
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
            damaged: attacker.id,
            start_hp,
            end_hp,
            cause: Cause::MoveSideEffect(Move::BellyDrum)
        }, ActionSideEffects::StatMaxed(attacker.id, BattleStat::Attack)]
    }
}

fn do_hazard_effect(side: &RefCell<Side>, hazard: &EntryHazardType) -> Vec<ActionSideEffects> {
    let mut side = side.borrow_mut();
    match hazard {
        EntryHazardType::Spikes => {
            if side.spikes < MAX_SPIKES {
                side.spikes = side.spikes + 1;
                vec![ActionSideEffects::EntryHazard(side.id, Move::Spikes, side.spikes)]
            } else {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            }
        }
        EntryHazardType::ToxicSpikes => {
            if side.toxic_spikes < MAX_TOXIC_SPIKES {
                side.toxic_spikes = side.toxic_spikes + 1;
                vec![ActionSideEffects::EntryHazard(side.id, Move::ToxicSpikes, side.toxic_spikes)]
            } else {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            }
        }
        EntryHazardType::StealthRock => {
            if side.pointed_stones {
                vec![ActionSideEffects::Failed(Cause::Natural)]
            } else {
                side.pointed_stones = true;
                vec![ActionSideEffects::EntryHazard(side.id, Move::StealthRock, 1)]
            }
        }
    }
}