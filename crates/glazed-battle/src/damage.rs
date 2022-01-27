use std::convert::TryFrom;

use rand::{Rng};
use glazed_core::math;

use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, DamageType, Move, MoveData, MultiHitFlavor, Power};
use glazed_data::types::{Effectiveness, Type};

use crate::{ActionSideEffects, Slot, Battlefield, Cause, damage, ForcedAction, BaseSlot, SlotId};
use crate::core;
use crate::core::MoveContext;
use crate::constants::*;

/// Perform raw damage calculation
/// Nothing related to types (such as Weather, STAB, or Effectiveness) or ailments (Burn) is applied here.
/// Randomness is also not applied here. All additional multipliers should be handled externally.
pub fn calculate_raw_damage(attacker: &Slot, base_power: u16, damage_type: DamageType, defender: &Slot) -> u16 {
    let (ea, ed) = match damage_type {
        DamageType::Physical => (
            attacker.get_effective_stat(BattleStat::Attack),
            defender.get_effective_stat(BattleStat::Defense)
        ),
        DamageType::Special => (
            attacker.get_effective_stat(BattleStat::SpecialAttack),
            defender.get_effective_stat(BattleStat::SpecialDefense)
        ),
        DamageType::Status => (1, 1) // Shouldn't happen, but if it does, pokemon stats have no part in the calculation
    };

    let calc = ((2u16 * attacker.borrow().level as u16) / 5) + 2;
    let calc = u32::from(calc) * u32::from(base_power) * u32::from(ea);
    let calc = (calc / (u32::from(ed) * 50u32)) + 2u32;
    calc as u16
}

/// Calculate confusion damage
/// Confusion damage is equivalent to a typeless physical move of power 40.
pub fn calculate_confusion_damage(pkmn: &Slot) -> u16 {
    let raw = calculate_raw_damage(pkmn, CONFUSION_POWER, DamageType::Physical, pkmn);
    (f64::from(raw) * rand::thread_rng().gen_range(0.85..=1.0)) as u16
}

impl Battlefield { //region Damage
    fn determine_crit(attacker: &Slot, move_data: &MoveData) -> bool {
        match attacker.get_raw_critical_hit() + move_data.crit_rate.unwrap_or(0) {
            0 => rand::thread_rng().gen_bool(0.0625),
            1 => rand::thread_rng().gen_bool(0.125),
            2 => rand::thread_rng().gen_bool(0.5),
            _ => true
        }
    }

    fn lower_hp(attacker: &Slot, defender: &Slot, attack: Move, damage: u16, is_crit: bool, effectiveness: Effectiveness) -> Vec<ActionSideEffects> {
        if defender.is_behind_substitute() && !attack.bypasses_substitute() && attacker.get_effective_ability() != Ability::Infiltrator {
            let mut defender_data = defender.data.borrow_mut();
            let start_hp = defender_data.substituted;
            let end_hp = start_hp.saturating_sub(damage);

            defender_data.substituted = end_hp;

            vec![ActionSideEffects::DamagedSubstitute {
                damaged: defender.slot_id,
                start_hp,
                end_hp
            }]
        } else {
            let was_full_hp = defender.borrow().is_full_health();
            let start_hp = defender.borrow().current_hp;
            let mut end_hp = start_hp.saturating_sub(damage);

            let hang_on = if end_hp == 0 { Battlefield::do_hang_on_check(defender, was_full_hp) } else { None };
            let mut effects = if let Some(action) = hang_on {
                let mut pkmn = defender.borrow_mut();
                pkmn.current_hp = 1;
                let mut e = vec![ActionSideEffects::DirectDamage {
                    damaged: defender.slot_id,
                    damager: attacker.slot_id,
                    attack,
                    start_hp,
                    end_hp: 1,
                    critical_hit: is_crit,
                    effectiveness
                }];
                e.extend(action);
                e
            } else {
                let mut pkmn = defender.borrow_mut();
                pkmn.current_hp = end_hp;
                vec![ActionSideEffects::DirectDamage {
                    damaged: defender.slot_id,
                    damager: attacker.slot_id,
                    attack,
                    start_hp,
                    end_hp,
                    critical_hit: is_crit,
                    effectiveness
                }]
            };

            effects.append(&mut Battlefield::do_after_damage(attacker, defender, attack, damage));

            effects
        }
    }

    fn lower_hp_basic(attacker: &Slot, defender: &Slot, attack: Move, damage: u16, cause: Cause) -> Vec<ActionSideEffects> {
        if defender.is_behind_substitute() && !attack.bypasses_substitute() && attacker.get_effective_ability() != Ability::Infiltrator {
            let mut defender_data = defender.data.borrow_mut();
            let start_hp = defender_data.substituted;
            let end_hp = start_hp.saturating_sub(damage);

            defender_data.substituted = end_hp;

            vec![ActionSideEffects::DamagedSubstitute {
                damaged: defender.slot_id,
                start_hp,
                end_hp
            }]
        } else {
            let was_full_hp = defender.borrow().is_full_health();
            let start_hp = defender.borrow().current_hp;
            let mut end_hp = start_hp.saturating_sub(damage);

            let hang_on = if end_hp == 0 { Battlefield::do_hang_on_check(defender, was_full_hp) } else { None };
            let mut effects = if let Some(action) = hang_on {
                let mut pkmn = defender.borrow_mut();
                pkmn.current_hp = 1;
                let mut e = vec![ActionSideEffects::BasicDamage {
                    damaged: defender.slot_id,
                    start_hp,
                    end_hp: 1,
                    cause
                }];
                e.extend(action);
                e
            } else {
                let mut pkmn = defender.borrow_mut();
                pkmn.current_hp = end_hp;
                vec![ActionSideEffects::BasicDamage {
                    damaged: defender.slot_id,
                    start_hp,
                    end_hp,
                    cause
                }]
            };

            effects.append(&mut Battlefield::do_after_damage(attacker, defender, attack, damage));

            effects
        }
    }

    fn do_hang_on_check(damaged: &Slot, _was_full_hp: bool) -> Option<Vec<ActionSideEffects>> {
        let data = damaged.data.borrow();
        if let Some(Move::Endure) = data.protected {
            return Some(vec![ActionSideEffects::HungOn(damaged.slot_id, Cause::MoveSideEffect(Move::Endure))]);
        }
        None
    }

    fn do_after_damage(attacker: &Slot, defender: &Slot, attack: Move, damage: u16) -> Vec<ActionSideEffects> {
        let mut vec = Vec::new();
        let mut data = defender.data.borrow_mut();

        if defender.borrow().is_fainted() && data.destiny_bond && !attacker.is_not_active() {
            vec.append(&mut Battlefield::faint(attacker, Cause::MoveSideEffect(Move::DestinyBond)));
            return vec;
        }

        if !attacker.is_not_active() {
            data.damage_this_turn.push((attacker.slot_id, attack, damage));
            data.last_attacker = Some((attacker.slot_id, attack));
        }

        if let Some(ForcedAction::Bide(acc, counter)) = data.forced_action {
            data.forced_action = Some(ForcedAction::Bide(acc + damage, counter));
        }

        if data.enraged {
            vec.push(crate::effects::change_self_stat(defender, BattleStat::Attack, 1));
        }
        vec
    }

    pub fn faint(active: &Slot, cause: Cause) -> Vec<ActionSideEffects> {
        let mut pkmn = active.borrow_mut();
        let start_hp = pkmn.current_hp;
        let end_hp = 0;

        pkmn.current_hp = 0;

        vec![ActionSideEffects::BasicDamage {
            damaged: active.slot_id,
            start_hp,
            end_hp,
            cause
        }]
    }

    pub fn do_damage(&self, attacker: &Slot, attack: Move, defender: &Slot, is_multi_target: bool) -> Vec<ActionSideEffects> {
        let move_data = attack.data();
        let crit = || Battlefield::determine_crit(attacker, move_data);
        let effectiveness = || core::get_type_effectiveness(&self, attacker, attack, defender);
        match &move_data.power {
            Power::None => Vec::new(),
            Power::Base(_) | Power::BaseAfterNTurns(_, _) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, attack, defender, is_multi_target, is_crit, effectiveness);
                    Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                }
            },
            Power::BaseWithCharge(_, _) => {
                let (effectiveness, cause) = effectiveness();
                let effects = if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, attack, defender, is_multi_target, is_crit, effectiveness);
                    Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                };

                // Clear charging data
                let mut data = attacker.data.borrow_mut();
                data.forced_action = None;
                data.invulnerable = None;
                effects
            },
            Power::BaseWithRecoil(_, recoil) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, attack, defender, is_multi_target, is_crit, effectiveness);
                    let mut effects = Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness);

                    {
                        let attacker_ability = attacker.get_effective_ability();
                        if attacker_ability != Ability::RockHead && attacker_ability != Ability::MagicGuard {
                            let recoil_damage = math::fraction(damage, *recoil);

                            let mut attacker_pkmn = attacker.borrow_mut();
                            let start_hp = attacker_pkmn.current_hp;
                            let end_hp = start_hp.saturating_sub(recoil_damage);
                            attacker_pkmn.current_hp = end_hp;
                            effects.push(ActionSideEffects::Recoil {
                                damaged: attacker.slot_id,
                                start_hp,
                                end_hp,
                                cause: Cause::Move(attacker.id(), attack)
                            });
                        }
                    }

                    effects
                }
            },
            Power::BaseWithCrash(_) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause), attacker.take_crash_damage()]
                } else {
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, attack, defender, is_multi_target, is_crit, effectiveness);
                    Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                }
            },
            Power::BaseWithFaint(_) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, attack, defender, is_multi_target, is_crit, effectiveness);
                    let mut effects = Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness);
                    effects.append(&mut Battlefield::faint(attacker, Cause::Move(attacker.id(), attack)));
                    effects
                }
            },
            Power::BaseWithDrain(_) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, attack, defender, is_multi_target, is_crit, effectiveness);
                    let mut effects = Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness);

                    let heal_amount = if damage == 1 { 1 } else { damage / 2 };
                    let mut pkmn = attacker.borrow_mut();
                    if defender.get_effective_ability() == Ability::LiquidOoze {
                        let (start_hp, end_hp) = pkmn.subtract_hp(heal_amount);
                        effects.push(ActionSideEffects::BasicDamage {
                            damaged: attacker.slot_id,
                            start_hp,
                            end_hp,
                            cause: Cause::Ability(defender.id(), Ability::LiquidOoze)
                        })
                    } else {
                        let (start_hp, end_hp) = pkmn.add_hp(heal_amount);
                        effects.push(ActionSideEffects::Healed {
                            healed: attacker.slot_id,
                            start_hp,
                            end_hp,
                            cause: Cause::Move(attacker.id(), attack)
                        })
                    }
                    effects
                }
            },
            Power::BaseWithTurnMultiplier(b) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let power_of_two = attacker.data.borrow().last_move_used_counter.clamp(0, FURY_CUTTER_CAP);
                    let context = MoveContext {
                        attack,
                        data: attack.data(),
                        base_power: u16::from(*b) * (1 << power_of_two)
                    };
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, context, defender, is_multi_target, is_crit, effectiveness);
                    let mut effects = Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness);
                    effects.append(&mut Battlefield::faint(attacker, Cause::Move(attacker.id(), attack)));
                    effects
                }
            },
            Power::WeightBased => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let move_context = MoveContext {
                        attack,
                        data: move_data,
                        base_power: weight_to_power_map(defender.get_effective_weight())
                    };
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, move_context, defender, is_multi_target, is_crit, effectiveness);
                    Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                }
            },
        //     Power::WeightRatioBased => {
        //         let attacker_weight = attacker.get_effective_weight() as f64;
        //         let defender_weight = defender.get_effective_weight() as f64;
        //         let ratio = attacker_weight / defender_weight;
        //         let base = if ratio > 0.5 { 40 }
        //         else if ratio > 0.3335 { 60 }
        //         else if ratio > 0.2501 { 80 }
        //         else if ratio > 0.2001 { 100 }
        //         else { 120 };
        //
        //         self.do_damage_from_base_power(attacker, MoveContext {
        //             attack,
        //             data: move_data,
        //             base_power: base
        //         }, defender)
        //     },
            Power::OneHitKnockout => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    Battlefield::lower_hp_basic(attacker, defender, attack, u16::MAX, Cause::Move(attacker.id(), attack))
                }
            },
            Power::Exact(damage) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    Battlefield::lower_hp_basic(attacker, defender, attack, *damage as u16, Cause::Move(attacker.id(), attack))
                }
            }
        //     Power::Percentage(_) => self.do_percent_damage(attacker, attack, defender),
            Power::Revenge(a, damage) => {
                let data = attacker.data.borrow();
                let candidates = data.damage_this_turn.iter()
                    .filter_map(|(attacker, attack, damage)| {
                        let pkmn = self.get_active_pokemon_by_active_id(*attacker);
                        if pkmn.borrow().has_health() {
                            Some((pkmn, *attack, *damage))
                        } else {
                            None
                        }
                    });
                let first_damage = match damage {
                    None => candidates
                            .map(|(pkmn, _, damage)| (pkmn, damage))
                            .last(),
                    Some(t) => candidates
                            .filter_map(|(pkmn, attack, damage)| {
                                if attack.data().damage_type == *t {
                                    Some((pkmn, damage))
                                } else {
                                    None
                                }
                            })
                            .last()
                };
                match first_damage {
                    None => vec![ActionSideEffects::Failed(Cause::Natural)],
                    Some((defender, damage)) => {
                        let (numerator, denominator) = *a;
                        let return_damage = u32::from(damage) * u32::from(numerator) / u32::from(denominator);
                        Battlefield::lower_hp_basic(attacker, &defender, attack, return_damage as u16, Cause::Natural)
                    }
                }
            },
            Power::MultiTurn(base, turns_to_do) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let curl_multiplier = if attacker.data.borrow().curled { 2 } else { 1 };
                    let context = if let Some(ForcedAction::AttackWithWeakCounter(_, turn_ctr)) = attacker.data.borrow().forced_action {
                        MoveContext {
                            attack,
                            data: attack.data(),
                            base_power: u16::from(*base).saturating_mul(1 << (*turns_to_do - turn_ctr)).saturating_mul(curl_multiplier)
                        }
                    } else {
                        let mut ctx = MoveContext::from(attack);
                        ctx.base_power = ctx.base_power.saturating_mul(curl_multiplier);
                        ctx
                    };

                    {
                        let mut data = attacker.data.borrow_mut();
                        if data.forced_action.is_none() {
                            data.forced_action = Some(ForcedAction::AttackWithWeakCounter(attack, 4));
                        }
                    }

                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, context, defender, is_multi_target, is_crit, effectiveness);
                    Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                }
            }
            Power::MultiHit(MultiHitFlavor::Variable(_)) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    return vec![ActionSideEffects::NoEffect(cause)]
                }

                let n = if attacker.get_effective_ability() == Ability::SkillLink {
                    MULTI_HIT_SKILL_LINK
                } else {
                    rand::thread_rng().sample(MultiHitDistribution)
                };

                let mut effects = Vec::new();
                let mut counter = 0;
                for _ in 0..n {
                    let damage_action = self._do_damage_and_effects(attacker, &vec![defender.clone()], attack, move_data);

                    effects.push(damage_action);
                    counter += 1;
                    if defender.borrow().is_fainted() || attacker.borrow().is_fainted() {
                        break;
                    }
                }

                vec![ActionSideEffects::MultiStrike {
                    actions: effects,
                    hits: counter
                }]
            },
            Power::MultiHit(MultiHitFlavor::Fixed(n, base)) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    return vec![ActionSideEffects::NoEffect(cause)]
                }

                let mut effects = Vec::new();
                let mut counter = 0;
                for _ in 0..*n {
                    let move_context = MoveContext {
                        attack,
                        data: move_data,
                        base_power: u16::from(*base)
                    };

                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, move_context, defender, is_multi_target, is_crit, effectiveness);
                    let damage_action = Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness);

                    if damage_action.iter().any(|e| e.did_damage()) {

                    }

                    effects.push(damage_action);
                    counter += 1;
                    if defender.borrow().is_fainted() || attacker.borrow().is_fainted() {
                        break;
                    }
                }

                vec![ActionSideEffects::MultiStrike {
                    actions: effects,
                    hits: counter
                }]
            },
        //     Power::MultiHit(MultiHitFlavor::Accumulating(first, second, third)) => {
        //         let mut effects = Vec::new();
        //         let mut counter = 0;
        //         let strike_powers = vec![first, second, third];
        //         for base in strike_powers {
        //             let move_context = MoveContext {
        //                 attack,
        //                 data: move_data,
        //                 base_power: u16::from(*base)
        //             };
        //
        //             let turn_effects = self.do_damage_from_base_power(attacker, move_context, defender);
        //
        //             let end_turn = turn_effects.iter().any(|e| e.is_multi_hit_end());
        //             let got_hit = turn_effects.iter().any(|e| match e {
        //                 ActionSideEffects::DirectDamage { .. } => true,
        //                 _ => false
        //             });
        //             effects.push(turn_effects);
        //
        //             if got_hit {
        //                 counter += 1;
        //             }
        //             if end_turn {
        //                 break;
        //             }
        //         }
        //
        //         vec![ActionSideEffects::MultiStrike {
        //             actions: effects,
        //             hits: counter
        //         }]
        //     },
        //     Power::MultiHit(MultiHitFlavor::BeatUp) => {
        //         let mut effects = Vec::new();
        //         let mut counter = 0;
        //         // let base_powers = attacker.get_party().members
        //         //     .iter()
        //         //     .map(|p| p.species.data().stats.1.base_stat)
        //         //     .collect::<Vec<u8>>();
        //         let base_powers = vec![1u8,2u8,3u8];
        //
        //         for base in base_powers {
        //             let move_context = MoveContext {
        //                 attack,
        //                 data: move_data,
        //                 base_power: u16::from(base)
        //             };
        //
        //             let turn_effects = self.do_damage_from_base_power(attacker, move_context, defender);
        //
        //             let end_turn = turn_effects.iter().any(|e| e.is_multi_hit_end());
        //             let got_hit = turn_effects.iter().any(|e| match e {
        //                 ActionSideEffects::DirectDamage { .. } => true,
        //                 _ => false
        //             });
        //             effects.push(turn_effects);
        //
        //             if got_hit {
        //                 counter += 1;
        //             }
        //             if end_turn {
        //                 break;
        //             }
        //         }
        //
        //         vec![ActionSideEffects::MultiStrike {
        //             actions: effects,
        //             hits: counter
        //         }]
        //     }
            Power::Variable => {
                match attack {
                    Move::SeismicToss | Move::NightShade => {
                        let (effectiveness, cause) = effectiveness();
                        if let Effectiveness::Immune = effectiveness {
                            return vec![ActionSideEffects::NoEffect(cause)]
                        }

                        let damage = attacker.borrow().level as u16;
                        Battlefield::lower_hp_basic(attacker, defender, attack, damage, Cause::Move(attacker.id(), attack))
                    },
                    Move::Psywave => {
                        let (effectiveness, cause) = effectiveness();
                        if let Effectiveness::Immune = effectiveness {
                            return vec![ActionSideEffects::NoEffect(cause)]
                        }

                        let r = rand::thread_rng().gen_range(0..=100);
                        let mut damage = (attacker.borrow().level as u16 * (r + 50)) / 100;
                        if damage == 0 { damage = 1; }
                        Battlefield::lower_hp_basic(attacker, defender, attack, damage, Cause::Move(attacker.id(), attack))
                    },
                    Move::Flail | Move::Reversal => {
                        let (effectiveness, cause) = effectiveness();
                        if let Effectiveness::Immune = effectiveness {
                            vec![ActionSideEffects::NoEffect(cause)]
                        } else {
                            let move_context = MoveContext {
                                attack,
                                data: move_data,
                                base_power: hp_to_power_map(defender.borrow().current_hp, defender.borrow().hp.value)
                            };
                            let is_crit = crit();
                            let damage = self.calculate_full_damage(attacker, move_context, defender, is_multi_target, is_crit, effectiveness);
                            Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                        }
                    },
                    Move::Return => {
                        let (effectiveness, cause) = effectiveness();
                        if let Effectiveness::Immune = effectiveness {
                            vec![ActionSideEffects::NoEffect(cause)]
                        } else {
                            // Base Power of Return == Friendship / 2.5 == Friendship * 2 / 5
                            let move_context = MoveContext {
                                attack,
                                data: move_data,
                                base_power: math::cap_min(math::ratio(u16::from(attacker.borrow().friendship), 2u16, 5u16), 1)
                            };
                            let is_crit = crit();
                            let damage = self.calculate_full_damage(attacker, move_context, defender, is_multi_target, is_crit, effectiveness);
                            Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                        }
                    },
                    Move::Frustration => {
                        let (effectiveness, cause) = effectiveness();
                        if let Effectiveness::Immune = effectiveness {
                            vec![ActionSideEffects::NoEffect(cause)]
                        } else {
                            // Base Power of Frustration == (255 - Friendship) / 2.5 == (255 - Friendship) * 2 / 5
                            let move_context = MoveContext {
                                attack,
                                data: move_data,
                                base_power: math::cap_min(math::ratio(u16::from(u8::MAX - attacker.borrow().friendship), 2u16, 5u16), 1)
                            };
                            let is_crit = crit();
                            let damage = self.calculate_full_damage(attacker, move_context, defender, is_multi_target, is_crit, effectiveness);
                            Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                        }
                    },
                    Move::Present => {
                        let (effectiveness, cause) = effectiveness();
                        if let Effectiveness::Immune = effectiveness {
                            vec![ActionSideEffects::NoEffect(cause)]
                        } else {
                            let present = rand::thread_rng().sample(PresentDistribution);
                            match present {
                                Present::Damage(base) => {
                                    let move_context = MoveContext {
                                        attack,
                                        data: move_data,
                                        base_power: base
                                    };
                                    let is_crit = crit();
                                    let damage = self.calculate_full_damage(attacker, move_context, defender, is_multi_target, is_crit, effectiveness);
                                    Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                                },
                                Present::Heal => {
                                    let mut pkmn = defender.borrow_mut();
                                    if pkmn.is_full_health() {
                                        vec![ActionSideEffects::Failed(Cause::Natural)]
                                    } else {
                                        let to_heal = pkmn.hp.value / 4;
                                        let (start_hp, end_hp) = pkmn.add_hp(to_heal);
                                        vec![ActionSideEffects::Healed {
                                            healed: attacker.slot_id,
                                            start_hp,
                                            end_hp,
                                            cause: Cause::Move(attacker.id(), Move::Present)
                                        }]
                                    }
                                }
                            }
                        }
                    },
                    Move::Magnitude => {
                        let (effectiveness, cause) = effectiveness();
                        if let Effectiveness::Immune = effectiveness {
                            vec![ActionSideEffects::NoEffect(cause)]
                        } else {
                            let (magnitude, power) = rand::thread_rng().sample(MagnitudeDistribution);
                            let context = MoveContext {
                                attack,
                                data: move_data,
                                base_power: power
                            };
                            let mut effects = vec![ActionSideEffects::Magnitude(magnitude)];
                            let is_crit = crit();
                            let damage = self.calculate_full_damage(attacker, context, defender, is_multi_target, is_crit, effectiveness);
                            effects.append(&mut Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness));
                            effects
                        }
                    },
                    Move::HiddenPower => {
                        let (effectiveness, cause) = effectiveness();
                        if let Effectiveness::Immune = effectiveness {
                            vec![ActionSideEffects::NoEffect(cause)]
                        } else {
                            let is_crit = crit();
                            let context = MoveContext {
                                attack,
                                data: move_data,
                                base_power: attacker.borrow().get_hidden_power_power()
                            };
                            let damage = self.calculate_full_damage(attacker, context, defender, is_multi_target, is_crit, effectiveness);
                            Battlefield::lower_hp(attacker, defender, attack, damage, is_crit, effectiveness)
                        }
                    }
                    a => panic!("Move {:?} has variable power, yet no implementation specified", a)
                }
            }
            _ => unimplemented!("Unknown Power Type")
        }
    }

    pub fn calculate_full_damage<F>(&self, attacker: &Slot, attack: F, defender: &Slot, is_multi_target: bool, is_crit: bool, effectiveness: Effectiveness) -> u16
        where F: Into<MoveContext>
    {
        let MoveContext{ attack, data, base_power} = attack.into();
        let attack_type = core::get_effective_move_type(attacker, &self.field, attack);

        let mut calc = damage::calculate_raw_damage(attacker, base_power, data.damage_type, defender) as u32;

        // *targets
        if is_multi_target {
            calc = math::fraction(calc, MULTI_TARGET_MULTIPLIER);
        }

        // *weather
        {
            let field = self.field.borrow();
            match attack_type {
                Type::Water if field.is_rain() => calc = math::fraction(calc, GOOD_WEATHER_MULTIPLIER),
                Type::Water if field.is_sunny() => calc = math::fraction(calc, BAD_WEATHER_MULTIPLIER),
                Type::Fire if field.is_rain() => calc = math::fraction(calc, BAD_WEATHER_MULTIPLIER),
                Type::Fire if field.is_sunny() => calc = math::fraction(calc, GOOD_WEATHER_MULTIPLIER),
                _ => {}
            }
        }

        // *critical
        if is_crit {
            calc = math::fraction(calc, CRIT_MULTIPLIER);
        }

        // *random
        let random = rand::thread_rng().gen_range(DAMAGE_VARIABILITY);
        calc = ((calc as f64) * random) as u32;

        // *STAB
        if attacker.get_effective_type().is_stab(&attack_type) {
            calc = math::fraction(calc, STAB_MULTIPLIER);
        }

        // *Type
        if let Effectiveness::Effect(i) = effectiveness {
            if i < 0 {
                calc = calc >> -i
            } else {
                calc = calc << i
            }
        }

        // *burn
        if attacker.borrow().status.burn {
            calc = math::fraction(calc, BURN_MULTIPLIER);
        }

        if defender.data.borrow().minimized && attack.double_damage_on_minimized_target() {
            calc = math::fraction(calc, MINIMIZE_MULTIPLIER);
        }

        u16::try_from(calc).unwrap_or(u16::MAX)
    }

    pub fn do_bide_damage(&self, attacker: &Slot, damage: u16, defender: &Slot) -> Vec<ActionSideEffects> {
        let (effectiveness, cause) = core::get_type_effectiveness(&self, attacker, Move::Bide, defender);
        if let Effectiveness::Immune = effectiveness {
            vec![ActionSideEffects::NoEffect(cause)]
        } else {
            Battlefield::lower_hp(attacker, defender, Move::Bide, damage, false, effectiveness)
        }
    }
    // endregion
}