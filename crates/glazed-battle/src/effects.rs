use std::convert::TryFrom;
use fraction::{Fraction, ToPrimitive};
use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, DamageType, Effect, Move, MoveData, MultiHitFlavor, NonVolatileBattleAilment, Power, StatChangeTarget};
use glazed_data::constants::Species;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::{Pokemon};
use glazed_data::types::{Effectiveness, PokemonType, Type};
use crate::{ActionSideEffects, BattleData, Battlefield, Battler, BattleType, Cause, Field, FieldCause, Party, SemiInvulnerableLocation, Side, StatsCause, Weather, core};
use crate::core::MoveContext;

// Constants
pub const MAX_STAGE: i8 = 6;
pub const MIN_STAGE: i8 = -6;
pub const THAW_CHANCE: f64 = 0.2;
pub const FULL_PARALYSIS_CHANCE: f64 = 0.25;

pub(crate) type BattleBundle<'a> = (Battler, &'a Pokemon, &'a BattleData, Ability);

impl Battlefield {
    pub fn do_attack(&mut self, attacker: Battler, attack: Move, defender: Battler) -> Vec<ActionSideEffects> {
        let mut effects = Vec::new();
        let move_data = attack.data();

        // Check for reasons this Pokemon can't perform this move
        let attacker_pkmn = self.get_battle_bundle(&attacker).1;
        if attacker_pkmn.status.freeze {
            let thaw = attack.can_thaw_user() || rand::thread_rng().gen_bool(THAW_CHANCE);
            if thaw {
                let thaw = ActionSideEffects::Thawed(attacker);
                self.apply_side_effect(&thaw);
                effects.push(thaw);
            } else {
                let froze = ActionSideEffects::WasFrozen(attacker);
                self.apply_side_effect(&froze);
                effects.push(froze);
                return effects
            }
        }

        let attacker_pkmn = self.get_battle_bundle(&attacker).1;
        if attacker_pkmn.status.sleep > 0 {
            let turns_left = attacker_pkmn.status.sleep - 1;
            if turns_left == 0 {
                let woke = ActionSideEffects::WokeUp(attacker);
                self.apply_side_effect(&woke);
                effects.push(woke);
            } else if !attack.can_be_used_while_sleeping() {
                let sleep = ActionSideEffects::Sleep(attacker);
                self.apply_side_effect(&sleep);
                effects.push(sleep);
                return effects;
            }
        }

        let attacker_pkmn = self.get_battle_bundle(&attacker).1;
        if attacker_pkmn.status.paralysis {
            if rand::thread_rng().gen_bool(FULL_PARALYSIS_CHANCE) {
                effects.push(ActionSideEffects::IsFullyParalyzed(attacker));
                return effects
            }
        }

        core::copy_all(&mut effects, self.do_damage(attacker, attack, defender));

        for secondary_effect in move_data.effects {
            let secondary_effects = match secondary_effect {
                Effect::StatChange(stat, stages, probability, target) => {
                    let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
                    if triggers {
                        match target {
                            StatChangeTarget::User => self.change_self_stat(attacker, *stat, *stages),
                            StatChangeTarget::Target => self.change_opponent_stat(attacker, defender, *stat, *stages)
                        }
                    } else {
                        Vec::new()
                    }
                },
                Effect::NonVolatileStatus(ailment, probability) => {
                    let triggers = *probability == 0 || rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
                    if triggers {
                        self.inflict_non_volatile_status(defender, *ailment)
                    } else {
                        Vec::new()
                    }
                }
                _ => unimplemented!("Secondary effect not yet created")
            };
            self.apply_side_effects(&secondary_effects);
            core::copy_all(&mut effects, secondary_effects);
        }

        if effects.is_empty() {
            effects.push(ActionSideEffects::NothingHappened)
        }

        effects
    }

    //region Non-Damage
    fn _change_stat(&self, affected: BattleBundle, stat: BattleStat, stages: i8, cause: Cause) -> Vec<ActionSideEffects> {
        let (affected, _, data, _) = affected;

        let current = data.get_stage(stat);
        let next = current + stages;
        return if next > MAX_STAGE {
            vec![ActionSideEffects::NoEffect(Cause::StatsMaxed(StatsCause::TooHigh))]
        } else if next < MIN_STAGE {
            vec![ActionSideEffects::NoEffect(Cause::StatsMaxed(StatsCause::TooLow))]
        } else {
            vec![ActionSideEffects::StatChanged {
                stat,
                affected,
                cause,
                start: current,
                end: next
            }]
        }
    }

    fn change_self_stat(&self, affected: Battler, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
        let bundle = self.get_battle_bundle(&affected);
        let (affected, pkmn, data, ability) = bundle;

        let (ability_cause, stages) = match ability {
            Ability::Simple => (Cause::Ability(affected, Ability::Simple), stages * 2),
            Ability::Contrary => (Cause::Ability(affected, Ability::Contrary), -stages),
            _ => (Cause::Natural, stages)
        };

        self._change_stat(bundle, stat, stages, ability_cause)
    }

    fn change_opponent_stat(&self, affecter: Battler, affected: Battler, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
        let (affecter, _, _, affecting_ability) = self.get_battle_bundle(&affecter);
        let bundle = self.get_battle_bundle(&affected);
        let (affected, _, _, affected_ability) = bundle;
        let affected_side = self.get_side(&affected);

        if affected_side.mist > 0 {
            return vec![ActionSideEffects::NoEffectSecondary(Cause::Field(FieldCause::Mist))]
        }

        let (ability_cause, stages) = match affected_ability {
            Ability::Simple => (Cause::Ability(affected, Ability::Simple), stages * 2),
            Ability::Contrary  => {
                let cause = Cause::Ability(affected, Ability::Contrary);
                match affecting_ability {
                    oc if oc.is_ignore_ability_ability() => (cause.overwrite(Cause::Ability(affecter, oc)), -stages),
                    _ => (cause, stages)
                }
            },
            oc@ (Ability::ClearBody | Ability::WhiteSmoke) => {
                let cause = Cause::Ability(affected, oc);
                match affecting_ability {
                    oc if oc.is_ignore_ability_ability() => (cause.overwrite(Cause::Ability(affecter, oc)), stages),
                    _ => return vec![ActionSideEffects::NoEffectSecondary(Cause::Ability(affected, oc))]
                }
            }
            _ => (Cause::Natural, stages)
        };

        self._change_stat(bundle, stat, stages, ability_cause)
    }

    fn inflict_non_volatile_status(&self, affected: Battler, status: NonVolatileBattleAilment) -> Vec<ActionSideEffects> {
        vec![ActionSideEffects::NonVolatileStatusAilment {
            affected,
            status
        }]
    }

    //endregion
}