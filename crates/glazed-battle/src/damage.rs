use std::cell::{Ref, RefMut};
use std::convert::TryFrom;
use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{DamageType, Move, MultiHitFlavor, Power};
use glazed_data::item::Item;
use glazed_data::types::{Effectiveness, Type};
use fraction::{Fraction, ToPrimitive};
use glazed_data::pokemon::Pokemon;
use crate::{ActionSideEffects, ActivePokemon, Battlefield, Battler, Cause, SemiInvulnerableLocation};
use crate::core;
use crate::core::MoveContext;

impl Battlefield { //region Damage
    pub fn do_damage(&mut self, attacker: &mut ActivePokemon, attack: Move, defender: &mut ActivePokemon) -> Vec<ActionSideEffects> {
        let move_data = attack.data();
        let effects = match &move_data.power {
            Power::None => Vec::new(),
            Power::Base(_) | Power::BaseWithRecoil(_, _) | Power::BaseWithMercy(_) =>
                self.do_damage_from_base_power(attacker, attack, defender),
            Power::WeightBased => {
                let weight = attacker.get_effective_weight();
                let base = match weight {
                    0..=99 => 20,
                    100..=249 => 40,
                    250..=499 => 60,
                    500..=999 => 80,
                    1000..=1999 => 100,
                    2000..=u16::MAX => 120
                };
                self.do_damage_from_base_power(attacker, core::MoveContext {
                    attack,
                    data: move_data,
                    base_power: base
                }, defender)
            },
            Power::WeightRatioBased => {
                let attacker_weight = attacker.get_effective_weight() as f64;
                let defender_weight = defender.get_effective_weight() as f64;
                let ratio = attacker_weight / defender_weight;
                let base = if ratio > 0.5 { 40 }
                else if ratio > 0.3335 { 60 }
                else if ratio > 0.2501 { 80 }
                else if ratio > 0.2001 { 100 }
                else { 120 };

                self.do_damage_from_base_power(attacker, MoveContext {
                    attack,
                    data: move_data,
                    base_power: base
                }, defender)
            },
            Power::OneHitKnockout => self.do_ohko(attacker, attack, defender),
            Power::Exact(_) => self.do_exact_damage(attacker, attack, defender),
            Power::Variable => self.do_one_off_damage(attacker, attack, defender),
            Power::Percentage(_) => self.do_percent_damage(attacker, attack, defender),
            Power::Revenge(_, _) => self.do_revenge_damage(attacker, attack),
            Power::MultiHit(MultiHitFlavor::Variable(base)) => {
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

                    let turn_effects = self.do_damage_from_base_power(attacker, move_context, defender);

                    let end_turn = turn_effects.iter().any(|e| e.is_multi_hit_end());
                    let got_hit = turn_effects.iter().any(|e| match e {
                        ActionSideEffects::DirectDamage { .. } => true,
                        _ => false
                    });
                    effects.push(turn_effects);

                    if got_hit {
                        counter += 1;
                    }
                    if end_turn {
                        break;
                    }
                }

                vec![ActionSideEffects::MultiStrike {
                    actions: effects,
                    hits: counter
                }]
            },
            Power::MultiHit(MultiHitFlavor::Fixed(n, base)) => {
                let mut effects = Vec::new();
                let mut counter = 0;
                for _ in 0..*n {
                    let move_context = MoveContext {
                        attack,
                        data: move_data,
                        base_power: u16::from(*base)
                    };

                    let turn_effects = self.do_damage_from_base_power(attacker, move_context, defender);

                    let end_turn = turn_effects.iter().any(|e| e.is_multi_hit_end());
                    let got_hit = turn_effects.iter().any(|e| match e {
                        ActionSideEffects::DirectDamage { .. } => true,
                        _ => false
                    });
                    effects.push(turn_effects);

                    if got_hit {
                        counter += 1;
                    }
                    if end_turn {
                        break;
                    }
                }

                vec![ActionSideEffects::MultiStrike {
                    actions: effects,
                    hits: counter
                }]
            },
            Power::MultiHit(MultiHitFlavor::Accumulating(first, second, third)) => {
                let mut effects = Vec::new();
                let mut counter = 0;
                let strike_powers = vec![first, second, third];
                for base in strike_powers {
                    let move_context = MoveContext {
                        attack,
                        data: move_data,
                        base_power: u16::from(*base)
                    };

                    let turn_effects = self.do_damage_from_base_power(attacker, move_context, defender);

                    let end_turn = turn_effects.iter().any(|e| e.is_multi_hit_end());
                    let got_hit = turn_effects.iter().any(|e| match e {
                        ActionSideEffects::DirectDamage { .. } => true,
                        _ => false
                    });
                    effects.push(turn_effects);

                    if got_hit {
                        counter += 1;
                    }
                    if end_turn {
                        break;
                    }
                }

                vec![ActionSideEffects::MultiStrike {
                    actions: effects,
                    hits: counter
                }]
            },
            Power::MultiHit(MultiHitFlavor::BeatUp) => {
                let mut effects = Vec::new();
                let mut counter = 0;
                // let base_powers = attacker.get_party().members
                //     .iter()
                //     .map(|p| p.species.data().stats.1.base_stat)
                //     .collect::<Vec<u8>>();
                let base_powers = vec![1u8,2u8,3u8];

                for base in base_powers {
                    let move_context = MoveContext {
                        attack,
                        data: move_data,
                        base_power: u16::from(base)
                    };

                    let turn_effects = self.do_damage_from_base_power(attacker, move_context, defender);

                    let end_turn = turn_effects.iter().any(|e| e.is_multi_hit_end());
                    let got_hit = turn_effects.iter().any(|e| match e {
                        ActionSideEffects::DirectDamage { .. } => true,
                        _ => false
                    });
                    effects.push(turn_effects);

                    if got_hit {
                        counter += 1;
                    }
                    if end_turn {
                        break;
                    }
                }

                vec![ActionSideEffects::MultiStrike {
                    actions: effects,
                    hits: counter
                }]
            }
        };

        effects
    }

    fn do_damage_from_base_power<F>(&mut self, attacker: &mut ActivePokemon, attack: F, defender: &mut ActivePokemon) -> Vec<ActionSideEffects>
        where F: Into<MoveContext>
    {
        let defender_side = self.get_side(&defender.id);

        let MoveContext{ attack, data, base_power } = attack.into();
        let move_data = data;
        let mut effects = Vec::new();

        let base = base_power;

        let damage = core::calculate_raw_damage_from_base(attacker, defender, base,
                                                          Some(move_data._type), move_data.damage_type, move_data.crit_rate.unwrap_or(0),
                                                          &self.field);

        if let ActionSideEffects::NoEffect(_) = damage {
            return vec![damage]
        }

        if let ActionSideEffects::DirectDamage {
            start_hp, end_hp, critical_hit, effectiveness, ..
        } = damage {
            let mut damage = start_hp - end_hp;

            // Do: Apply additional multipliers
            // Specific attacks, interacting with defender state
            match attack {
                Move::BodySlam | Move::Stomp | Move::Astonish | Move::Extrasensory |
                Move::NeedleArm | Move::DragonRush | Move::ShadowForce |
                Move::Steamroller | Move::HeatCrash | Move::HeavySlam if defender.data.borrow().minimized => {
                    damage *= 2;
                },
                Move::Magnitude | Move::Earthquake => {
                    if let Some(SemiInvulnerableLocation::Underground) = defender.data.borrow().invulnerable {
                        damage *= 2;
                    }
                },
                Move::Surf | Move::Whirlpool => {
                    if let Some(SemiInvulnerableLocation::Underwater) = defender.data.borrow().invulnerable {
                        damage *= 2;
                    }
                },
                _ => {}
            }
            // Broad attack categories interacting with defender state
            if defender_side.aurora_veil > 0 && !critical_hit && attacker.get_effective_ability() != Ability::Infiltrator {
                damage /= 2;
            } else {
                if defender_side.light_screen > 0 &&
                    move_data.damage_type == DamageType::Special &&
                    !critical_hit &&
                    attacker.get_effective_ability() != Ability::Infiltrator {
                    damage /= 2;
                } else if defender_side.reflect > 0 &&
                    move_data.damage_type == DamageType::Physical &&
                    !critical_hit &&
                    attacker.get_effective_ability() != Ability::Infiltrator {
                    damage /= 2;
                }
            }
            // Attacker/Defender ability flavors
            match attacker.get_effective_ability() {
                Ability::Sniper if critical_hit => {
                    damage += damage / 2;
                },
                Ability::TintedLens if effectiveness.is_super_effective() => {
                    damage *= 2;
                },
                _ => {}
            }
            match defender.get_effective_ability() {
                Ability::Filter | Ability::SolidRock if effectiveness.is_super_effective() => {
                    damage = damage * 3 / 4;
                },
                Ability::Multiscale if defender.borrow().is_full_health() => {
                    damage /= 2;
                },
                _ => {}
            }
            // Attacker/Defender held item flavors
            match attacker.borrow().held_item {
                Some(Item::ExpertBelt) if effectiveness.is_super_effective() => {
                    damage += damage * 2 / 10;
                },
                Some(Item::LifeOrb) => {
                    damage += damage * 3 / 10;
                },
                Some(Item::Metronome) => {
                    match attacker.data.borrow().last_move_used {
                        Some(m) if m == attack => {
                            let multiplier = 0.2 * f64::from(attacker.data.borrow().last_move_used_counter);
                            let multiplier = multiplier.clamp(0.0, 1.0);
                            damage = (f64::from(damage) * multiplier) as u16;
                        },
                        None | Some(_) => {}
                    }
                }
                _ => {}
            }
            match defender.borrow().held_item {
                Some(Item::Berry(b)) => {
                    let effective_move_type = core::get_effective_move_type(attacker, &self.field, attack);
                    match b.get_resistance_berry_type() {
                        Some(Type::Normal) if Type::Normal == effective_move_type => {
                            damage /= 2;
                            defender.borrow_mut().held_item = None;
                            effects.push(ActionSideEffects::ConsumedItem(defender.id, Item::Berry(b)));
                        },
                        Some(t) if t == effective_move_type && effectiveness.is_super_effective() => {
                            damage /= 2;
                            defender.borrow_mut().held_item = None;
                            effects.push(ActionSideEffects::ConsumedItem(defender.id, Item::Berry(b)));
                        },
                        None | Some(_) => {}
                    }
                }
                _ => {}
            }

            let damage = u16::try_from(damage).unwrap_or(u16::MAX); //No rollover today

            if defender.data.borrow().substituted > 0 {
                let start_hp = defender.data.borrow().substituted;
                let mut end_hp = start_hp.saturating_sub(damage);
                if let Power::BaseWithMercy(_) = move_data.power {
                    end_hp = end_hp.clamp(1, u16::MAX);
                }
                effects.push(ActionSideEffects::DamagedSubstitute {
                    damaged: defender.id,
                    start_hp,
                    end_hp
                });
            } else {
                let start_hp = defender.borrow().current_hp;
                let mut end_hp = start_hp.saturating_sub(damage);
                if let Power::BaseWithMercy(_) = move_data.power {
                    end_hp = end_hp.clamp(1, u16::MAX);
                }
                effects.push(ActionSideEffects::DirectDamage {
                    damaged: defender.id,
                    damager: attacker.id,
                    attack,
                    start_hp,
                    end_hp,
                    critical_hit,
                    effectiveness,
                    hung_on_cause: None
                });
                // Additional ability effects can go here
            }

            if let Power::BaseWithRecoil(_, recoil) = &move_data.power {
                let attacker_ability = attacker.get_effective_ability();
                if attacker_ability != Ability::RockHead && attacker_ability != Ability::MagicGuard {
                    let recoil_damage = Fraction::from(damage) * Fraction::from(*recoil);
                    let recoil_damage = recoil_damage.to_u16().unwrap_or(1);
                    let start_hp = attacker.borrow().current_hp;
                    let end_hp = start_hp.saturating_sub(recoil_damage);
                    attacker.borrow_mut().current_hp = end_hp;
                    effects.push(ActionSideEffects::Recoil {
                        damaged: attacker.id,
                        start_hp,
                        end_hp,
                        cause: Cause::Natural
                    });
                }
            }
        }
        effects
    }

    fn do_ohko(&mut self, attacker: &mut ActivePokemon, attack: Move, defender: &mut ActivePokemon) -> Vec<ActionSideEffects> {
        // Effectiveness Calculation
        let effective_move_type = core::get_effective_move_type(attacker, &self.field, attack);
        let effectiveness = defender.get_effective_type().defending_against(&effective_move_type);

        if let Effectiveness::Immune = effectiveness {
            vec![ActionSideEffects::NoEffect(Cause::Natural)]
        } else {
            defender.lower_hp(u16::MAX)
        }
    }

    fn do_exact_damage(&mut self, attacker: &mut ActivePokemon, attack: Move, defender: &mut ActivePokemon) -> Vec<ActionSideEffects> {
        let move_data = attack.data();

        // Effectiveness Calculation
        let effective_move_type = core::get_effective_move_type(attacker, &self.field, attack);
        let effectiveness = defender.get_effective_type().defending_against(&effective_move_type);

        if let Effectiveness::Immune = effectiveness {
            vec![ActionSideEffects::NoEffect(Cause::Natural)]
        } else if let Power::Exact(delta) = move_data.power {
            defender.lower_hp(u16::from(delta))
        } else {
            panic!("do_exact_damage called with non-exact damage move")
        }
    }

    fn do_percent_damage(&mut self, attacker: &mut ActivePokemon, attack: Move, defender: &mut ActivePokemon) -> Vec<ActionSideEffects> {
        let move_data = attack.data();

        // Effectiveness Calculation
        let effective_move_type = core::get_effective_move_type(attacker, &self.field, attack);
        let effectiveness = defender.get_effective_type().defending_against(&effective_move_type);

        if let Effectiveness::Immune = effectiveness {
            return vec![ActionSideEffects::NoEffect(Cause::Natural)]
        }

        if let Power::Percentage(fraction) = move_data.power {
            let damage = Fraction::from(defender.borrow().current_hp) * Fraction::from(fraction);
            let mut damage = damage.to_u16().unwrap_or(u16::MAX);
            if damage == 0 {
                damage = 1;
            }
            defender.lower_hp(damage)
        } else {
            panic!("Called do_percent_damage with incorrect power type")
        }
    }

    fn do_one_off_damage(&mut self, attacker: &mut ActivePokemon, attack: Move, defender: &mut ActivePokemon) -> Vec<ActionSideEffects> {
        // Effectiveness Calculation
        let effective_move_type = core::get_effective_move_type(attacker, &self.field, attack);
        let effectiveness = defender.get_effective_type().defending_against(&effective_move_type);

        if let Effectiveness::Immune = effectiveness {
            return vec![ActionSideEffects::NoEffect(Cause::Natural)]
        }

        match attack {
            Move::Endeavor => {
                let user_hp = attacker.borrow().current_hp;
                let target_hp = if defender.data.borrow().substituted > 0 {
                    defender.data.borrow().substituted
                } else {
                    defender.borrow().current_hp
                };

                if user_hp >= target_hp {
                    vec![ActionSideEffects::Failed(Cause::Natural)]
                } else {
                    let start_hp = defender.borrow().current_hp;
                    defender.borrow_mut().current_hp = user_hp;
                    vec![ActionSideEffects::BasicDamage {
                        damaged: defender.id,
                        start_hp,
                        end_hp: user_hp,
                        cause: Cause::Natural,
                        hung_on_cause: None
                    }]
                }
            },
            Move::FinalGambit => {
                let damage = attacker.borrow().current_hp;

                // User Damage technically happens first, so the user would lose in a 1 on 1 if dealing lethal damage
                attacker.borrow_mut().current_hp = 0;
                let mut effects = vec![ActionSideEffects::BasicDamage {
                    damaged: attacker.id,
                    start_hp: damage,
                    end_hp: 0,
                    cause: Cause::Natural,
                    hung_on_cause: None
                }];

                core::copy_all(&mut effects, defender.lower_hp(damage));

                effects
            },
            Move::NightShade | Move::SeismicToss => {
                let damage = u16::from(attacker.borrow().level);
                defender.lower_hp(damage)
            },
            Move::Psywave => {
                let damage = u32::from(attacker.borrow().level) * (rand::thread_rng().gen_range(0..=100) + 50) / 100;
                let mut damage = u16::try_from(damage).unwrap_or(u16::MAX);
                if damage == 0 {
                    damage = 1;
                }
                defender.lower_hp(damage)
            },
            Move::PainSplit => {
                if defender.data.borrow().substituted > 0 {
                    return vec![ActionSideEffects::Failed(Cause::Natural)]
                }
                let calc = u32::from(attacker.borrow().current_hp) + u32::from(defender.borrow().current_hp);
                let hp = u16::try_from(calc / 2).unwrap(); // Per the existence of math, we can guarantee this will work fine.
                attacker.borrow_mut().current_hp = hp;
                defender.borrow_mut().current_hp = hp;
                vec![ActionSideEffects::PainSplit {
                    user: attacker.id,
                    defender: defender.id,
                    split_hp: hp
                }]
            }
            a => panic!("{:?} does not have one-off damage associated with it", a)
        }
    }

    fn do_revenge_damage(&mut self, attacker: &mut ActivePokemon, attack: Move) -> Vec<ActionSideEffects> {
        let move_data = attack.data();
        let data = attacker.data.borrow();

        if let Power::Revenge(t, fraction) = &move_data.power {
            let to_hit = match t {
                Some(dt) => {
                    // Find the most recent attack of the correct type that damaged the user
                    data.damage_this_turn.iter()
                        .filter(move |(battler, attack, damage)| attack.data().damage_type == *dt)
                        .last()
                },
                None => {
                    // (Metal Burst) select the last damager
                    data.damage_this_turn.last()
                }
            };
            match to_hit {
                Some((target, _, damage)) => {
                    let payback_damage = Fraction::from(*damage) * Fraction::from(*fraction);
                    let payback_damage = payback_damage.to_u16().unwrap_or(0);
                    //self.get_mut_active_pokemon(target).lower_hp(payback_damage)
                    vec![]
                },
                None => vec![ActionSideEffects::Failed(Cause::Natural)]
            }
        } else {
            panic!("Called do_revenge_damage for move without Revenge power type")
        }
    }

    pub fn calculate_confusion_damage(&self, pkmn: RefMut<ActivePokemon>) -> u16 {
        let atk = pkmn.borrow().attack.value;
        let def = pkmn.borrow().defense.value;
        let numerator = (((2 * u16::from(pkmn.borrow().level)) / 5) + 2) * 40 * atk;
        let denominator = 50 * def;
        let raw = ((numerator / denominator) + 2);
        (f64::from(raw) * rand::thread_rng().gen_range(0.85..=1.0)) as u16
    }
    // endregion
}