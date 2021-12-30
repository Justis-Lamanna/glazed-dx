use std::cell::{Ref, RefCell, RefMut};
use std::convert::TryFrom;
use rand::{random, Rng};
use glazed_data::abilities::Ability;
use glazed_data::attack::{DamageType, Move, MoveData, MultiHitFlavor, Power};
use glazed_data::item::Item;
use glazed_data::types::{Effectiveness, Type};
use fraction::{Fraction, ToPrimitive};
use glazed_data::pokemon::Pokemon;
use crate::{ActionSideEffects, ActivePokemon, Battlefield, Battler, Cause};
use crate::core;
use crate::core::MoveContext;
use crate::effects::SelectedTarget;

impl Battlefield { //region Damage
    fn determine_crit(attacker: &ActivePokemon, move_data: &MoveData) -> bool {
        match attacker.get_raw_critical_hit() + move_data.crit_rate.unwrap_or(0) {
            0 => rand::thread_rng().gen_bool(0.0625),
            1 => rand::thread_rng().gen_bool(0.125),
            2 => rand::thread_rng().gen_bool(0.5),
            _ => true
        }
    }

    fn lower_hp(attacker: Battler, defender: &ActivePokemon, attack: Move, damage: u16, is_crit: bool, effectiveness: Effectiveness) -> Vec<ActionSideEffects> {
        if defender.is_behind_substitute() {
            let mut defender_data = defender.data.borrow_mut();
            let start_hp = defender_data.substituted;
            let end_hp = start_hp.saturating_sub(damage);

            defender_data.substituted = end_hp;

            vec![ActionSideEffects::DamagedSubstitute {
                damaged: defender.id,
                start_hp,
                end_hp
            }]
        } else {
            let defender_id = defender.id;
            let mut defender = defender.borrow_mut();
            let start_hp = defender.current_hp;
            let end_hp = start_hp.saturating_sub(damage);

            defender.current_hp = end_hp;

            vec![ActionSideEffects::DirectDamage {
                damaged: defender_id,
                damager: attacker,
                attack,
                start_hp,
                end_hp,
                hung_on_cause: None,
                critical_hit: is_crit,
                effectiveness
            }]
        }
    }

    fn lower_hp_basic(attacker: Battler, defender: &ActivePokemon, attack: Move, damage: u16, cause: Cause) -> Vec<ActionSideEffects> {
        if defender.is_behind_substitute() {
            let mut defender_data = defender.data.borrow_mut();
            let start_hp = defender_data.substituted;
            let end_hp = start_hp.saturating_sub(damage);

            defender_data.substituted = end_hp;

            vec![ActionSideEffects::DamagedSubstitute {
                damaged: defender.id,
                start_hp,
                end_hp
            }]
        } else {
            let defender_id = defender.id;
            let mut defender = defender.borrow_mut();
            let start_hp = defender.current_hp;
            let end_hp = start_hp.saturating_sub(damage);

            defender.current_hp = end_hp;

            vec![ActionSideEffects::BasicDamage {
                damaged: defender_id,
                start_hp,
                end_hp,
                cause,
                hung_on_cause: None
            }]
        }
    }

    pub fn do_damage(&self, attacker: &ActivePokemon, attack: Move, defender: &ActivePokemon, is_multi_target: bool) -> Vec<ActionSideEffects> {
        let move_data = attack.data();
        let crit = || Battlefield::determine_crit(attacker, move_data);
        let effectiveness = || core::get_type_effectiveness(&self, attacker, attack, defender);
        match &move_data.power {
            Power::None => Vec::new(),
            Power::Base(_) | Power::BaseWithCharge(_, _) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, attack, defender, is_multi_target, is_crit, effectiveness);
                    Battlefield::lower_hp(attacker.id, defender, attack, damage, is_crit, effectiveness)
                }
            },
            Power::BaseWithCharge(_, _) => {
                let (effectiveness, cause) = effectiveness();
                let effects = if let Effectiveness::Immune = effectiveness {
                    vec![ActionSideEffects::NoEffect(cause)]
                } else {
                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, attack, defender, is_multi_target, is_crit, effectiveness);
                    Battlefield::lower_hp(attacker.id, defender, attack, damage, is_crit, effectiveness)
                };

                // Clear charging data
                let mut data = attacker.data.borrow_mut();
                data.charging = None;
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
                    let mut effects = Battlefield::lower_hp(attacker.id, defender, attack, damage, is_crit, effectiveness);

                    {
                        let attacker_ability = attacker.get_effective_ability();
                        if attacker_ability != Ability::RockHead && attacker_ability != Ability::MagicGuard {
                            let recoil_damage = (Fraction::from(*recoil) * Fraction::from(damage)).to_u16().unwrap();

                            let mut attacker_pkmn = attacker.borrow_mut();
                            let start_hp = attacker_pkmn.current_hp;
                            let end_hp = start_hp.saturating_sub(recoil_damage);
                            attacker_pkmn.current_hp = end_hp;
                            effects.push(ActionSideEffects::Recoil {
                                damaged: attacker.id,
                                start_hp,
                                end_hp,
                                cause: Cause::Move(attacker.id, attack)
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
                    Battlefield::lower_hp(attacker.id, defender, attack, damage, is_crit, effectiveness)
                }
            }
        //     Power::WeightBased => {
        //         let weight = attacker.get_effective_weight();
        //         let base = match weight {
        //             0..=99 => 20,
        //             100..=249 => 40,
        //             250..=499 => 60,
        //             500..=999 => 80,
        //             1000..=1999 => 100,
        //             2000..=u16::MAX => 120
        //         };
        //         self.do_damage_from_base_power(attacker, core::MoveContext {
        //             attack,
        //             data: move_data,
        //             base_power: base
        //         }, defender)
        //     },
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
                    Battlefield::lower_hp_basic(attacker.id, defender, attack, u16::MAX, Cause::Move(attacker.id, attack))
                }
            },
        //     Power::Exact(_) => self.do_exact_damage(attacker, attack, defender),
        //     Power::Variable => self.do_one_off_damage(attacker, attack, defender),
        //     Power::Percentage(_) => self.do_percent_damage(attacker, attack, defender),
        //     Power::Revenge(_) => self.do_revenge_damage(attacker, attack),
            Power::MultiHit(MultiHitFlavor::Variable(base)) => {
                let (effectiveness, cause) = effectiveness();
                if let Effectiveness::Immune = effectiveness {
                    return vec![ActionSideEffects::NoEffect(cause)]
                }

                let n = if attacker.get_effective_ability() == Ability::SkillLink {
                    5
                } else {
                    let n = rand::thread_rng().gen_range(0u8..100u8);
                    if n < 35 { 2 } else if n < 70 { 3 } else if n < 85 { 4 } else { 5 }
                };

                let mut effects = Vec::new();
                let mut counter = 0;
                for _ in 0..n {
                    let move_context = MoveContext {
                        attack,
                        data: move_data,
                        base_power: u16::from(*base)
                    };

                    let is_crit = crit();
                    let damage = self.calculate_full_damage(attacker, move_context, defender, is_multi_target, is_crit, effectiveness);
                    let damage_action = Battlefield::lower_hp(attacker.id, defender, attack, damage, is_crit, effectiveness);

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
                    let damage_action = Battlefield::lower_hp(attacker.id, defender, attack, damage, is_crit, effectiveness);

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
            _ => vec![]
        }
    }

    pub fn calculate_full_damage<F>(&self, attacker: &ActivePokemon, attack: F, defender: &ActivePokemon, is_multi_target: bool, is_crit: bool, effectiveness: Effectiveness) -> u16
        where F: Into<MoveContext>
    {
        let MoveContext{ attack, data, base_power} = attack.into();
        let attack_type = core::get_effective_move_type(attacker, &self.field, attack);

        let mut calc = core::calculate_raw_damage(attacker, base_power, data.damage_type, defender) as u32;

        // *targets
        if is_multi_target {
            calc = calc * 3 / 4;
        }

        // *weather
        {
            let field = self.field.borrow();
            match attack_type {
                Type::Water if field.is_rain() => calc = calc * 3 / 2,
                Type::Water if field.is_sunny() => calc = calc / 2,
                Type::Fire if field.is_rain() => calc = calc / 2,
                Type::Fire if field.is_sunny() => calc = calc * 3 / 2,
                _ => {}
            }
        }

        // *critical
        if is_crit {
            calc = calc * 2
        }

        // *random
        let random = rand::thread_rng().gen_range(0.85..=1.0);
        calc = ((calc as f64) * random) as u32;

        // *STAB
        if attacker.get_effective_type().is_stab(&attack_type) {
            calc = calc * 3 / 2
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
            calc = calc / 2
        }

        u16::try_from(calc).unwrap_or(u16::MAX)
    }
    // endregion
}