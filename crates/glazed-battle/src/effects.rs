use std::cell::{Ref, RefCell, RefMut};
use std::convert::TryFrom;
use std::ops::{Deref, Index, IndexMut};
use std::pin::Pin;
use fraction::{Fraction, ToPrimitive};
use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{Accuracy, BattleStat, DamageType, Effect, Move, MoveData, MultiHitFlavor, NonVolatileBattleAilment, Power, StatChangeTarget, VolatileBattleAilment};
use glazed_data::constants::Species;
use glazed_data::constants::UnownForm::P;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::{Gender, Pokemon};
use glazed_data::types::{Effectiveness, PokemonType, Type};
use crate::{ActionSideEffects, BattleData, Battlefield, Battler, Cause, Field, FieldCause, Party, SemiInvulnerableLocation, Side, StatsCause, Weather, core, ActivePokemon, BattleSide, DoubleBattleSide, BattleParty};
use crate::core::{MoveContext};

// Constants
pub const MAX_STAGE: i8 = 6;
pub const MIN_STAGE: i8 = -6;
pub const THAW_CHANCE: f64 = 0.2;
pub const FULL_PARALYSIS_CHANCE: f64 = 0.25;
pub const CONFUSION_HIT_CHANCE: f64 = 0.5;
pub const INFATUATION_INACTION_CHANCE: f64 = 0.5;

impl Battlefield {
    pub fn do_attack(&mut self, attacker: Battler, attack: Move, defender: Battler) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let attacker = &self[attacker.side][attacker.individual];
        let defender = &self[defender.side][defender.individual];

        let move_data = attack.data();

        // Check for reasons this Pokemon can't perform this move

        //region Pre-attack
        if attacker.borrow().status.freeze {
            let thaw = attack.can_thaw_user() || rand::thread_rng().gen_bool(THAW_CHANCE);
            if thaw {
                attacker.borrow_mut().status.freeze = false;
                effects.push(ActionSideEffects::Thawed(attacker.id));
            } else {
                let froze = ActionSideEffects::WasFrozen(attacker.id);
                effects.push(froze);
                return effects;
            }
        }

        if attacker.deref().borrow().status.sleep > 0 {
            attacker.borrow_mut().status.sleep -= 1;
            if attacker.borrow().status.sleep == 0 {
                effects.push(ActionSideEffects::WokeUp(attacker.id));
            } else if !attack.can_be_used_while_sleeping() {
                effects.push(ActionSideEffects::Sleep(attacker.id));
                return effects;
            }
        }

        if attacker.borrow().status.paralysis {
            if rand::thread_rng().gen_bool(FULL_PARALYSIS_CHANCE) {
                effects.push(ActionSideEffects::IsFullyParalyzed(attacker.id));
                return effects
            }
        }

        if attacker.data.borrow().confused > 0 {
            attacker.data.borrow_mut().confused -= 1;
            if attacker.data.borrow().confused == 0 {
                // Snapped out of confusion
                effects.push(ActionSideEffects::SnappedOutOfConfusion(attacker.id));
            } else if rand::thread_rng().gen_bool(CONFUSION_HIT_CHANCE) {
                // Hit itself in confusion
                //let delta = self.calculate_confusion_damage(attacker);
                let delta = 20;
                let start_hp = attacker.borrow().current_hp;
                let end_hp = start_hp.saturating_sub(delta);

                attacker.borrow_mut().current_hp = end_hp;
                effects.push(ActionSideEffects::HurtInConfusion {
                    affected: attacker.id,
                    start: start_hp,
                    end: end_hp,
                    hang_on_cause: None
                });
                return effects;
            }
        }

        let hit = match move_data.accuracy {
            Accuracy::AlwaysHits => true,
            Accuracy::Percentage(p) => {
                //let evasion_accuracy = core::get_accuracy_factor(attacker, defender);
                let evasion_accuracy = 1.0;
                let move_accuracy = f64::from(p) / 100f64;
                rand::thread_rng().gen_bool(evasion_accuracy * move_accuracy)
            },
            Accuracy::Variable => unimplemented!("Unknown accuracy")
        };

        if !hit {
            effects.push(ActionSideEffects::Missed(Cause::Natural));
            return effects
        }
        //endregion

        //region Primary damage (if applicable)
        // core::copy_all(&mut effects, self.do_damage(attacker, attack, defender));

        if effects.iter().any(|e| e.is_primary_failure()) {
            // If the move failed, don't execute the secondary effects
            return effects;
        }
        let primary_damage_hit = effects.len() > 0;
        //endregion

        //region Secondary effects
        // for secondary_effect in move_data.effects {
        //     let secondary_effects = match secondary_effect {
        //         Effect::StatChange(stat, stages, probability, target) => {
        //             let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
        //             if triggers {
        //                 match target {
        //                     StatChangeTarget::User => self.change_self_stat(attacker, *stat, *stages),
        //                     StatChangeTarget::Target => self.change_opponent_stat(attacker, defender, *stat, *stages)
        //                 }
        //             } else {
        //                 Vec::new()
        //             }
        //         },
        //         Effect::NonVolatileStatus(ailment, probability, target) => {
        //             let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
        //             if triggers {
        //                 match target {
        //                     StatChangeTarget::User => self.inflict_non_volatile_status(attacker, *ailment),
        //                     StatChangeTarget::Target => self.inflict_non_volatile_status(defender, *ailment)
        //                 }
        //             } else {
        //                 Vec::new()
        //             }
        //         },
        //         Effect::VolatileStatus(ailment, probability, _) => {
        //             let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
        //             if triggers {
        //                 self.inflict_volatile_status(defender, attacker, *ailment)
        //             } else {
        //                 Vec::new()
        //             }
        //         }
        //         _ => unimplemented!("Secondary effect not yet created")
        //     };
        //     core::copy_all(&mut effects, secondary_effects);
        // }
        //endregion

        if effects.is_empty() {
            effects.push(ActionSideEffects::NothingHappened)
        }

        effects
    }

    //region Non-Damage
    fn _change_stat(&self, affected: &mut ActivePokemon, stat: BattleStat, stages: i8, cause: Cause) -> Vec<ActionSideEffects> {
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

    fn change_self_stat(&self, affected: &mut ActivePokemon, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
        let (ability_cause, stages) = match affected.get_effective_ability() {
            Ability::Simple => (Cause::Ability(affected.id, Ability::Simple), stages * 2),
            Ability::Contrary => (Cause::Ability(affected.id, Ability::Contrary), -stages),
            _ => (Cause::Natural, stages)
        };

        self._change_stat(affected, stat, stages, ability_cause)
    }

    fn change_opponent_stat(&self, affecter: &mut ActivePokemon, affected: &mut ActivePokemon, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
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

    fn inflict_non_volatile_status(&self, affected: &mut ActivePokemon, status: NonVolatileBattleAilment) -> Vec<ActionSideEffects> {
        match status {
            NonVolatileBattleAilment::Paralysis => affected.borrow_mut().status.paralysis = true,
            NonVolatileBattleAilment::Sleep => affected.borrow_mut().status.sleep = rand::thread_rng().gen_range(1..=3),
            NonVolatileBattleAilment::Freeze => affected.borrow_mut().status.freeze = true,
            NonVolatileBattleAilment::Burn => affected.borrow_mut().status.burn = true,
            NonVolatileBattleAilment::Poison(a) => affected.borrow_mut().status.poison = Some(a)
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
        }
    }

    //endregion
}
impl Index<BattleSide> for Battlefield {
    type Output = BattleParty;

    fn index(&self, index: BattleSide) -> &Self::Output {
        match index {
            BattleSide::Forward => &self.user,
            BattleSide::Back => &self.opponent
        }
    }
}