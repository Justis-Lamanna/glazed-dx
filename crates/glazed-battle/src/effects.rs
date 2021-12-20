use std::convert::TryFrom;
use fraction::{Fraction, ToPrimitive};
use rand::Rng;
use glazed_data::abilities::Ability;
use glazed_data::attack::{BattleStat, DamageType, Effect, Move, MoveData, MultiHitFlavor, Power, StatChangeTarget};
use glazed_data::constants::Species;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::{Pokemon};
use glazed_data::types::{Effectiveness, PokemonType, Type};
use crate::{ActionSideEffects, BattleData, Battlefield, Battler, BattleTypeTrait, Cause, Field, FieldCause, SemiInvulnerableLocation, StatsCause, Weather};

// Constants
pub const MAX_STAGE: i8 = 6;
pub const MIN_STAGE: i8 = -6;

/// Simple utility to drain a vector into another one
fn copy_all<T>(vec: &mut Vec<T>, vec_to_add: Vec<T>) {
    for elem in vec_to_add {
        vec.push(elem);
    }
}

/// Get the effective ability of this Pokemon. Takes Transform and temporary Ability changes into effect
fn get_effective_ability(pkmn: &Pokemon, battle_data: &BattleData) -> Ability {
    battle_data.temp_ability.unwrap_or_else(|| {
        match &battle_data.transformed {
            None => pkmn.get_ability(),
            Some(t) => *t.get_ability()
        }
    })
}

/// Get the effective type(s) of this Pokemon. Takes Transform and temporary Type changes into effect
fn get_effective_type(pkmn: &Pokemon, battle_data: &BattleData) -> PokemonType {
    battle_data.temp_type.unwrap_or_else(|| {
        match &battle_data.transformed {
            Some(t) => t.species.data()._type,
            None => pkmn.species.data()._type,
        }
    })
}

/// Get the effective species of this Pokemon. Takes Transform into effect
fn get_effective_species(pkmn: &Pokemon, battle_data: &BattleData) -> Species {
    match &battle_data.transformed {
        None => pkmn.species,
        Some(t) => t.species
    }
}

/// Get the effective stat of this Pokemon. Takes Transform and stages into effect
fn get_effective_stat(pkmn: &Pokemon, battle_data: &BattleData, stat: BattleStat) -> u16 {
    let base = match stat {
        BattleStat::Attack => battle_data.transformed.as_ref().map(|t| t.attack.value).unwrap_or(pkmn.attack.value),
        BattleStat::Defense => battle_data.transformed.as_ref().map(|t| t.defense.value).unwrap_or(pkmn.defense.value),
        BattleStat::SpecialAttack => battle_data.transformed.as_ref().map(|t| t.special_attack.value).unwrap_or(pkmn.special_attack.value),
        BattleStat::SpecialDefense => battle_data.transformed.as_ref().map(|t| t.special_defense.value).unwrap_or(pkmn.special_defense.value),
        BattleStat::Speed => battle_data.transformed.as_ref().map(|t| t.speed.value).unwrap_or(pkmn.speed.value),
        BattleStat::Accuracy => 1,
        BattleStat::Evasion => 1
    };

    let stage = match stat {
        BattleStat::Attack => battle_data.attack_stage,
        BattleStat::Defense => battle_data.defense_stage,
        BattleStat::SpecialAttack => battle_data.special_attack_stage,
        BattleStat::SpecialDefense => battle_data.special_defense_stage,
        BattleStat::Speed => battle_data.speed_stage,
        BattleStat::Accuracy => 0i8,
        BattleStat::Evasion => 0i8
    };

    match stage {
        i8::MIN..=MIN_STAGE => base * 2u16 / 8u16,
        -5 => base * 2u16 / 7u16,
        -4 => base * 2u16 / 6u16,
        -3 => base * 2u16 / 5u16,
        -2 => base * 2u16 / 4u16,
        -1 => base * 2u16 / 3u16,
        0 => base,
        1 => base * 3u16 / 2u16,
        2 => base * 2u16,
        3 => base * 5u16 / 2u16,
        4 => base * 3u16,
        5 => base * 7u16 / 2u16,
        MAX_STAGE..=i8::MAX => base * 4u16
    }
}

/// Get the raw critical hit stage of this Pokemon. Takes held item and ability into account
fn get_raw_critical_hit(pkmn: &Pokemon, species: Species, ability: Ability) -> u8 {
    let mut value = 0;
    value += match pkmn.held_item {
        Some(Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw)) => 1,
        Some(Item::ScopeLens) => 1,
        Some(Item::Leek) if species == Species::Farfetchd => 2,
        Some(Item::LuckyPunch) if species == Species::Chansey => 2,
        _ => 0
    };
    value += match ability {
        Ability::SuperLuck => 1,
        _ => 0
    };
    value
}

/// Get the type of the move used. Takes ability and type-changing moves into account
fn get_effective_move_type(attacker: BattleBundle, field: &Field, attack: Move, move_data: &MoveData) -> Type {
    let (_, attacker_pokemon, _, attacker_ability) = attacker;
    match attacker_ability {
        Ability::Normalize if attack != Move::HiddenPower ||
            attack != Move::WeatherBall ||
            attack != Move::NaturalGift ||
            attack != Move::Judgment ||
            attack != Move::TechnoBlast => return Type::Normal,
        _ => {}
    };

    match attack {
        Move::HiddenPower => attacker_pokemon.get_hidden_power_type(),
        Move::WeatherBall => match field.weather {
            Some(Weather::Sun(_)) => Type::Fire,
            Some(Weather::Rain(_)) => Type::Water,
            Some(Weather::Hail(_)) => Type::Ice,
            Some(Weather::Sandstorm(_)) => Type::Rock,
            Some(Weather::Fog) | None => Type::Normal
        },
        Move::Judgment => match attacker_pokemon.held_item {
            Some(Item::DracoPlate) => Type::Dragon,
            Some(Item::DreadPlate) => Type::Dark,
            Some(Item::EarthPlate) => Type::Ground,
            Some(Item::FistPlate) => Type::Fighting,
            Some(Item::FlamePlate) => Type::Fire,
            Some(Item::IciclePlate) => Type::Ice,
            Some(Item::InsectPlate) => Type::Bug,
            Some(Item::Iron) => Type::Steel,
            Some(Item::MeadowPlate) => Type::Grass,
            Some(Item::MindPlate) => Type::Psychic,
            Some(Item::PixiePlate) => Type::Fairy,
            Some(Item::SkyPlate) => Type::Flying,
            Some(Item::SplashPlate) => Type::Water,
            Some(Item::SpookyPlate) => Type::Ghost,
            Some(Item::StonePlate) => Type::Rock,
            Some(Item::ToxicPlate) => Type::Poison,
            Some(Item::ZapPlate) => Type::Electric,
            _ => Type::Normal
        },
        Move::TechnoBlast => match attacker_pokemon.held_item {
            Some(Item::DouseDrive) => Type::Water,
            Some(Item::ShockDrive) => Type::Electric,
            Some(Item::ChillDrive) => Type::Ice,
            Some(Item::BurnDrive) => Type::Fire,
            _ => Type::Normal
        },
        Move::NaturalGift => {
            if let Some(Item::Berry(a)) = &attacker_pokemon.held_item {
                a.get_natural_gift_type()
            } else {
                move_data._type
            }
        }
        _ => move_data._type
    }
}

/// Get the effective weight of this Pokemon. Takes Ability, held item, and Autotomize into account
fn get_effective_weight(pkmn: BattleBundle) -> u16 {
    let (_, species, data, ability) = pkmn;
    let mut weight = data.temp_weight.unwrap_or_else(|| species.species.data().weight);

    if ability == Ability::HeavyMetal {
        weight = weight.saturating_mul(2);
    } else if ability == Ability::LightMetal {
        weight = weight.saturating_div(2);
    }

    if let Some(Item::FloatStone) = species.held_item {
        weight = weight.saturating_div(2);
    }

    if weight == 0 { 1 } else { weight }
}

type BattleBundle<'a> = (Battler, &'a Pokemon, &'a BattleData, Ability);

struct MoveContext {
    attack: Move,
    data: &'static MoveData,
    base_power: u16
}
impl From<Move> for MoveContext {
    fn from(attack: Move) -> Self {
        let data = attack.data();
        let base_power = match data.power {
            Power::Base(base) | Power::BaseWithRecoil(base, _) | Power::BaseWithMercy(base) |
                Power::MultiHit(MultiHitFlavor::Variable(base)) | Power::MultiHit(MultiHitFlavor::Fixed(_, base)) => u16::from(base),
            _ => panic!("Can't derive base power, must be specified")
        };
        MoveContext {
            attack,
            data,
            base_power
        }
    }
}

impl <T> Battlefield<T> where T: BattleTypeTrait {
    fn get_battle_bundle(&self, id: &Battler) -> BattleBundle {
        let pokemon = self.get_pokemon_by_id(id).unwrap();
        let data = self.get_battle_data(id);
        let ability = get_effective_ability(pokemon, data);

        (*id, pokemon, data, ability)
    }

    fn apply_side_effects(&mut self, effects: &Vec<ActionSideEffects>) {
        for effect in effects {
            match effect {
                ActionSideEffects::BasicDamage {damaged, end_hp, cause, ..} => {
                    let mut damaged_pkmn = self.get_mut_pokemon_by_id(&damaged).unwrap();
                    let delta = damaged_pkmn.current_hp.saturating_sub(*end_hp);
                    damaged_pkmn.current_hp = *end_hp;

                    if let Cause::Move(battler, attack) = cause {
                        let data = self.get_mut_battle_data(&damaged);
                        data.damage_this_turn.push((*battler, *attack, delta));

                        if let Some((turns_left, damage_acc)) = data.bide {
                            data.bide = Some((turns_left, damage_acc.saturating_add(delta)));
                        }
                    }
                },
                ActionSideEffects::DirectDamage { damaged, end_hp, damager, attack, .. } => {
                    let damaged_pkmn = self.get_mut_pokemon_by_id(&damaged).unwrap();
                    let delta = damaged_pkmn.current_hp.saturating_sub(*end_hp);
                    damaged_pkmn.current_hp = *end_hp;

                    let data = self.get_mut_battle_data(&damaged);
                    data.damage_this_turn.push((*damager, *attack, delta));

                    if let Some((turns_left, damage_acc)) = data.bide {
                        data.bide = Some((turns_left, damage_acc.saturating_add(delta)));
                    }
                },
                ActionSideEffects::DamagedSubstitute { damaged, end_hp, ..} => {
                    let mut damaged = self.get_mut_battle_data(&damaged);
                    damaged.substituted = *end_hp;
                },
                ActionSideEffects::ConsumedItem(battler, _) => {
                    let mut battler = self.get_mut_pokemon_by_id(&battler);
                    battler.unwrap().held_item = None;
                },
                ActionSideEffects::Recoil {damaged, end_hp, .. } => {
                    let mut damaged = self.get_mut_pokemon_by_id(&damaged);
                    damaged.unwrap().current_hp = *end_hp;
                }
                ActionSideEffects::Missed(_) => {}
                ActionSideEffects::NoEffect(_) => {}
                ActionSideEffects::Failed(_) => {}
                ActionSideEffects::PainSplit { user, defender, split_hp } => {
                    let mut first = self.get_mut_pokemon_by_id(user).unwrap();
                    first.current_hp = (*split_hp).clamp(0, first.hp.value);

                    let mut second = self.get_mut_pokemon_by_id(defender).unwrap();
                    second.current_hp = (*split_hp).clamp(0, second.hp.value);
                }
                ActionSideEffects::NoTarget => {}
                ActionSideEffects::MultiStrike {..} => {}
                ActionSideEffects::StatChanged { affected, stat, end, .. } => {
                    let mut data = self.get_mut_battle_data(affected);
                    let stat_to_mod = match stat {
                        BattleStat::Attack => &mut data.attack_stage,
                        BattleStat::Defense => &mut data.defense_stage,
                        BattleStat::SpecialAttack => &mut data.special_attack_stage,
                        BattleStat::SpecialDefense => &mut data.special_defense_stage,
                        BattleStat::Speed => &mut data.speed_stage,
                        BattleStat::Accuracy => &mut data.accuracy_stage,
                        BattleStat::Evasion => &mut data.evasion_stage
                    };

                    *stat_to_mod = (*end).clamp(MIN_STAGE, MAX_STAGE);
                }
                ActionSideEffects::NoEffectSecondary(_) => {}
                ActionSideEffects::NothingHappened => {}
            }
        }
    }

    fn hangs_on(&self, defender: BattleBundle, attacker: BattleBundle) -> Option<Cause> {
        if defender.1.is_full_health() {
            match defender.3 {
                Ability::Sturdy => {
                    let cause = Cause::Ability(defender.0, defender.3);
                    if attacker.3 == Ability::MoldBreaker || attacker.3 == Ability::Teravolt || attacker.3 == Ability::Turboblaze {
                        Some(cause.overwrite(Cause::Ability(attacker.0, attacker.3)))
                    } else {
                        Some(cause)
                    }
                },
                _ => match defender.1.held_item {
                    Some(Item::FocusSash) => {
                        Some(Cause::HeldItem(defender.0, Item::FocusSash))
                    },
                    _ => None
                }
            }
        } else {
            None
        }
    }

    fn calculate_standard_basic_damage<F>(&self, defender: BattleBundle, attacker: BattleBundle, attack: Move, calc_end_hp: F) -> Vec<ActionSideEffects>
        where F: Fn(u16) -> u16 {
        if defender.2.substituted > 0 {
            let start_hp = defender.2.substituted;
            let end_hp = calc_end_hp(start_hp);
            vec![ActionSideEffects::DamagedSubstitute {
                damaged: defender.0,
                start_hp,
                end_hp
            }]
        } else {
            let start_hp = defender.1.current_hp;
            let end_hp = calc_end_hp(start_hp);

            let hang_on = if end_hp == 0 {
                self.hangs_on(defender, attacker)
            } else {
                None
            };

            let mut effects = Vec::new();
            match hang_on {
                Some(c) => {
                    if let Cause::HeldItem(battler, item) = c.clone() {
                        effects.push(ActionSideEffects::ConsumedItem(battler, item))
                    }
                    effects.push(
                        ActionSideEffects::BasicDamage {
                            damaged: defender.0,
                            start_hp,
                            end_hp: 1,
                            cause: Cause::Move(attacker.0, attack),
                            hung_on_cause: Some(c)
                        }
                    );
                },
                None => {
                    effects.push(ActionSideEffects::BasicDamage {
                        damaged: defender.0,
                        start_hp,
                        end_hp,
                        cause: Cause::Move(attacker.0, attack),
                        hung_on_cause: None
                    });
                }
            }
            effects
        }
    }

    // Run the raw damage calculations, without considering an exact move
    // Exact formula: base_damage * weather * crit * random * STAB * Type * Burn
    // Any other multipliers have to be done in addition.
    fn calculate_raw_damage_from_base(&self, defender_bundle: BattleBundle, attacker_bundle: BattleBundle, base: u16, _type: Option<Type>, damage_type: DamageType, crit_rate: u8) -> ActionSideEffects {
        let (attacker, attacker_pokemon, attacker_data, attacker_ability) = attacker_bundle;
        let attacker_type = get_effective_type(attacker_pokemon, attacker_data);
        let attacker_species = get_effective_species(attacker_pokemon, attacker_data);

        let (defender, defender_pokemon, defender_data, _) = defender_bundle;
        let defender_type = get_effective_type(defender_pokemon, defender_data);

        // Effectiveness Calculation
        let effective_move_type = _type;
        let effectiveness = match effective_move_type {
            Some(t) => defender_type.defending_against(&t),
            None => Effectiveness::NORMAL
        };

        if let Effectiveness::Immune = effectiveness {
            return ActionSideEffects::NoEffect(Cause::Natural)
        }

        let stab = match effective_move_type {
            Some(t) => attacker_type.is_stab(&t),
            None => false
        };

        // Critical Hit Calculation
        let crit_stages = get_raw_critical_hit(attacker_pokemon, attacker_species, attacker_ability) + crit_rate;
        let crit = match crit_stages {
            0 => rand::thread_rng().gen_bool(0.0625),
            1 => rand::thread_rng().gen_bool(0.125),
            2 => rand::thread_rng().gen_bool(0.5),
            _ => true
        };

        let (ea, ed) = match damage_type {
            DamageType::Physical => (
                get_effective_stat(attacker_pokemon, attacker_data, BattleStat::Attack),
                get_effective_stat(defender_pokemon, defender_data, BattleStat::Defense)),
            DamageType::Special => (
                get_effective_stat(attacker_pokemon, attacker_data, BattleStat::SpecialAttack),
                get_effective_stat(defender_pokemon, defender_data, BattleStat::SpecialDefense)),
            DamageType::Status => panic!("Status move should not do direct damage??")
        };

        let base_damage_num = (((2 * u32::from(attacker_pokemon.level)) / 5) + 2) * u32::from(base) * u32::from(ea);
        let base_damage_denom = u32::from(ed) * 50;
        let mut damage = base_damage_num / base_damage_denom + 2;

        // * weather
        if let Some(t) = effective_move_type {
            if t == Type::Water {
                if self.field.is_rain() {
                    damage += damage / 2;
                } else if self.field.is_sunny() {
                    damage /= 2;
                }
            } else if t == Type::Fire {
                if self.field.is_rain() {
                    damage /= 2;
                } else if self.field.is_sunny() {
                    damage += damage / 2;
                }
            }
        }

        // * crit
        if crit {
            damage += damage / 2;
        }

        // * random
        damage = (f64::from(damage) * rand::thread_rng().gen_range(0.85..=1.0)) as u32;

        // * stab
        if stab {
            if attacker_ability == Ability::Adaptability {
                damage *= 2;
            } else {
                // This is a fair approximation of 150%, right?
                damage += damage / 2;
            }
        }

        // * type
        if let Effectiveness::Effect(i) = effectiveness {
            if i < 0 {
                damage = damage >> -i;
            } else {
                damage = damage << i;
            }
        }

        // * burn
        if attacker_pokemon.status.burn &&
            attacker_ability != Ability::Guts &&
            damage_type == DamageType::Physical {
            damage /= 2;
        }

        ActionSideEffects::DirectDamage {
            damaged: defender,
            damager: attacker,
            attack: Move::Pound,
            start_hp: defender_pokemon.current_hp,
            end_hp: defender_pokemon.current_hp.saturating_sub(damage as u16),
            hung_on_cause: None,
            critical_hit: crit,
            effectiveness
        }
    }

    pub fn do_attack(&mut self, attacker: Battler, attack: Move, defender: Battler) -> Vec<ActionSideEffects> {
        let move_data = attack.data();
        let does_damage = if let Power::None = move_data.power { true } else { false };
        let mut effects = Vec::new();

        if does_damage {
            copy_all(&mut effects, self.do_damage(attacker, attack, defender));
        }

        for secondary_effect in move_data.effects {
            let secondary_effects = match secondary_effect {
                Effect::StatChange(stat, stages, probability, target) => {
                    let triggers = rand::thread_rng().gen_bool(f64::from(*probability) / 100f64);
                    if triggers {
                        match target {
                            StatChangeTarget::User => self.change_self_stat(attacker, *stat, *stages),
                            StatChangeTarget::Target => self.change_opponent_stat(attacker, defender, *stat, *stages)
                        }
                    } else {
                        Vec::new()
                    }
                }
                _ => {Vec::new()}
            };
            self.apply_side_effects(&secondary_effects);
            copy_all(&mut effects, secondary_effects);
        }

        if effects.is_empty() {
            effects.push(ActionSideEffects::NothingHappened)
        }

        effects
    }

    //region Damage
    fn do_damage(&mut self, attacker: Battler, attack: Move, defender:Battler) -> Vec<ActionSideEffects> {
        let move_data = attack.data();
        let effects = match &move_data.power {
            Power::None => Vec::new(),
            Power::Base(_) | Power::BaseWithRecoil(_, _) | Power::BaseWithMercy(_) =>
                self.do_damage_from_base_power(attacker, attack, defender),
            Power::WeightBased => {
                let defender_bundle= self.get_battle_bundle(&defender);
                let weight = get_effective_weight(defender_bundle);
                let base = match weight {
                    0..=99 => 20,
                    100..=249 => 40,
                    250..=499 => 60,
                    500..=999 => 80,
                    1000..=1999 => 100,
                    2000..=u16::MAX => 120
                };
                self.do_damage_from_base_power(attacker, MoveContext {
                    attack,
                    data: move_data,
                    base_power: base
                }, defender)
            },
            Power::WeightRatioBased => {
                let attacker_bundle = self.get_battle_bundle(&attacker);
                let defender_bundle = self.get_battle_bundle(&defender);
                let attacker_weight = get_effective_weight(attacker_bundle) as f64;
                let defender_weight = get_effective_weight(defender_bundle) as f64;
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
                let attacker_bundle = self.get_battle_bundle(&attacker);
                let n = if attacker_bundle.3 == Ability::SkillLink {
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
                    self.apply_side_effects(&turn_effects);

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
                    self.apply_side_effects(&turn_effects);

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
                    self.apply_side_effects(&turn_effects);

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
                let base_powers = self.get_party(&attacker).members
                    .iter()
                    .filter_map(|p| {
                        match p {
                            Some(p) if !p.is_fainted() && !p.status.has_status_condition() && !p.egg => {
                                Some(p.species.data().stats.1.base_stat)
                            },
                            _ => None
                        }
                    })
                    .collect::<Vec<u8>>();

                for base in base_powers {
                    let move_context = MoveContext {
                        attack,
                        data: move_data,
                        base_power: u16::from(base)
                    };

                    let turn_effects = self.do_damage_from_base_power(attacker, move_context, defender);
                    self.apply_side_effects(&turn_effects);

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

        self.apply_side_effects(&effects);
        effects
    }

    fn do_damage_from_base_power<F>(&mut self, attacker: Battler, attack: F, defender: Battler) -> Vec<ActionSideEffects>
        where F: Into<MoveContext>
    {
        let attacker_bundle = self.get_battle_bundle(&attacker);
        let (attacker, attacker_pokemon, attacker_data, attacker_ability) = attacker_bundle;

        let defender_bundle = self.get_battle_bundle(&defender);
        let (defender, defender_pokemon, defender_data, defender_ability) = defender_bundle;
        let defender_side = self.get_side(&defender);

        let MoveContext{ attack, data, base_power } = attack.into();
        let move_data = data;
        let mut effects = Vec::new();

        let base = base_power;

        let damage = self.calculate_raw_damage_from_base(defender_bundle, attacker_bundle, base,
                                                         Some(move_data._type), move_data.damage_type, move_data.crit_rate.unwrap_or(0));

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
                Move::Steamroller | Move::HeatCrash | Move::HeavySlam if defender_data.minimized => {
                    damage *= 2;
                },
                Move::Magnitude | Move::Earthquake => {
                    if let Some(SemiInvulnerableLocation::Underground) = &defender_data.invulnerable {
                        damage *= 2;
                    }
                },
                Move::Surf | Move::Whirlpool => {
                    if let Some(SemiInvulnerableLocation::Underwater) = &defender_data.invulnerable {
                        damage *= 2;
                    }
                },
                _ => {}
            }
            // Broad attack categories interacting with defender state
            if defender_side.aurora_veil > 0 && !critical_hit && attacker_ability != Ability::Infiltrator {
                damage /= 2;
            } else {
                if defender_side.light_screen > 0 && move_data.damage_type == DamageType::Special && !critical_hit && attacker_ability != Ability::Infiltrator {
                    damage /= 2;
                } else if defender_side.reflect > 0 && move_data.damage_type == DamageType::Physical && !critical_hit && attacker_ability != Ability::Infiltrator {
                    damage /= 2;
                }
            }
            // Attacker/Defender ability flavors
            match attacker_ability {
                Ability::Sniper if critical_hit => {
                    damage += damage / 2;
                },
                Ability::TintedLens if effectiveness.is_super_effective() => {
                    damage *= 2;
                },
                _ => {}
            }
            match defender_ability {
                Ability::Filter | Ability::SolidRock if effectiveness.is_super_effective() => {
                    damage = damage * 3 / 4;
                },
                Ability::Multiscale if defender_pokemon.is_full_health() => {
                    damage /= 2;
                },
                _ => {}
            }
            // Attacker/Defender held item flavors
            match attacker_pokemon.held_item {
                Some(Item::ExpertBelt) if effectiveness.is_super_effective() => {
                    damage += damage * 2 / 10;
                },
                Some(Item::LifeOrb) => {
                    damage += damage * 3 / 10;
                },
                Some(Item::Metronome) => {
                    match attacker_data.last_move_used {
                        Some(m) if m == attack => {
                            let multiplier = 0.2 * f64::from(attacker_data.last_move_used_counter);
                            let multiplier = multiplier.clamp(0.0, 1.0);
                            damage = (f64::from(damage) * multiplier) as u16;
                        },
                        None | Some(_) => {}
                    }
                }
                _ => {}
            }
            match &defender_pokemon.held_item {
                Some(Item::Berry(b)) => {
                    let effective_move_type = get_effective_move_type(attacker_bundle, &self.field, attack, move_data);
                    match b.get_resistance_berry_type() {
                        Some(Type::Normal) if Type::Normal == effective_move_type => {
                            damage /= 2;
                            effects.push(ActionSideEffects::ConsumedItem(defender, Item::Berry(*b)));
                        },
                        Some(t) if t == effective_move_type && effectiveness.is_super_effective() => {
                            damage /= 2;
                            effects.push(ActionSideEffects::ConsumedItem(defender, Item::from(*b)));
                        },
                        None | Some(_) => {}
                    }
                }
                _ => {}
            }

            let damage = u16::try_from(damage).unwrap_or(u16::MAX); //No rollover today

            if defender_data.substituted > 0 {
                let start_hp = defender_data.substituted;
                let mut end_hp = start_hp.saturating_sub(damage);
                if let Power::BaseWithMercy(_) = move_data.power {
                    end_hp = end_hp.clamp(1, u16::MAX);
                }
                effects.push(ActionSideEffects::DamagedSubstitute {
                    damaged: defender,
                    start_hp,
                    end_hp
                });
            } else {
                let start_hp = defender_pokemon.current_hp;
                let mut end_hp = start_hp.saturating_sub(damage);
                if let Power::BaseWithMercy(_) = move_data.power {
                    end_hp = end_hp.clamp(1, u16::MAX);
                }
                let hang_on = if end_hp == 0 {
                    self.hangs_on(defender_bundle, attacker_bundle)
                } else {
                    None
                };
                match hang_on {
                    None => {
                        effects.push(ActionSideEffects::DirectDamage {
                            damaged: defender,
                            damager: attacker,
                            attack,
                            start_hp,
                            end_hp,
                            critical_hit,
                            effectiveness,
                            hung_on_cause: None
                        });
                    },
                    Some(c) => {
                        effects.push(ActionSideEffects::DirectDamage {
                            damaged: defender,
                            damager: attacker,
                            attack,
                            start_hp,
                            end_hp: 1,
                            critical_hit,
                            effectiveness,
                            hung_on_cause: Some(c)
                        });
                    }
                }

                // Additional ability effects can go here
            }

            if let Power::BaseWithRecoil(_, recoil) = &move_data.power {
                if attacker_ability != Ability::RockHead && attacker_ability != Ability::MagicGuard {
                    let recoil_damage = Fraction::from(damage) * Fraction::from(*recoil);
                    let recoil_damage = recoil_damage.to_u16().unwrap_or(1);
                    let start_hp = attacker_pokemon.current_hp;
                    effects.push(ActionSideEffects::Recoil {
                        damaged: attacker,
                        start_hp,
                        end_hp: start_hp.saturating_sub(recoil_damage),
                        cause: Cause::Move(attacker, attack)
                    });
                }
            }
        }
        effects
    }

    fn do_ohko(&mut self, attacker: Battler, attack: Move, defender: Battler) -> Vec<ActionSideEffects> {
        let attacker_bundle = self.get_battle_bundle(&attacker);

        let defender_bundle = self.get_battle_bundle(&defender);
        let (_, defender_pokemon, defender_data, _) = defender_bundle;
        let defender_type = get_effective_type(defender_pokemon, defender_data);

        let move_data = attack.data();

        // Effectiveness Calculation
        let effective_move_type = get_effective_move_type(attacker_bundle, &self.field, attack, move_data);
        let effectiveness = defender_type.defending_against(&effective_move_type);

        if let Effectiveness::Immune = effectiveness {
            vec![ActionSideEffects::NoEffect(Cause::Natural)]
        } else {
            self.calculate_standard_basic_damage(defender_bundle, attacker_bundle, attack, |start_hp| 0)
        }
    }

    fn do_exact_damage(&mut self, attacker: Battler, attack: Move, defender: Battler) -> Vec<ActionSideEffects> {
        let attacker_bundle: BattleBundle = self.get_battle_bundle(&attacker);

        let defender_bundle = self.get_battle_bundle(&defender);
        let (_, defender_pokemon, defender_data, _) = defender_bundle;
        let defender_type = get_effective_type(defender_pokemon, defender_data);

        let move_data = attack.data();

        // Effectiveness Calculation
        let effective_move_type = get_effective_move_type(attacker_bundle, &self.field, attack, move_data);
        let effectiveness = defender_type.defending_against(&effective_move_type);

        if let Effectiveness::Immune = effectiveness {
            vec![ActionSideEffects::NoEffect(Cause::Natural)]
        } else if let Power::Exact(delta) = move_data.power {
            self.calculate_standard_basic_damage(defender_bundle, attacker_bundle,
                                                 attack,
                                                 |start_hp| start_hp.saturating_sub(u16::from(delta)))
        } else {
            panic!("do_exact_damage called with non-exact damage move")
        }
    }

    fn do_percent_damage(&mut self, attacker: Battler, attack: Move, defender: Battler) -> Vec<ActionSideEffects> {
        let attacker_bundle = self.get_battle_bundle(&attacker);

        let defender_bundle = self.get_battle_bundle(&defender);
        let (_, defender_pokemon, defender_data, _) = defender_bundle;
        let defender_type = get_effective_type(defender_pokemon, defender_data);

        let move_data = attack.data();

        // Effectiveness Calculation
        let effective_move_type = get_effective_move_type(attacker_bundle, &self.field, attack, move_data);
        let effectiveness = defender_type.defending_against(&effective_move_type);

        if let Effectiveness::Immune = effectiveness {
            return vec![ActionSideEffects::NoEffect(Cause::Natural)]
        }

        if let Power::Percentage(fraction) = move_data.power {
            let damage = Fraction::from(defender_pokemon.current_hp) * Fraction::from(fraction);
            let mut damage = damage.to_u16().unwrap_or(u16::MAX);
            if damage == 0 {
                damage = 1;
            }
            self.calculate_standard_basic_damage(defender_bundle, attacker_bundle, attack, |start_hp| start_hp.saturating_sub(damage))
        } else {
            panic!("Called do_percent_damage with incorrect power type")
        }
    }

    fn do_one_off_damage(&mut self, attacker: Battler, attack: Move, defender: Battler) -> Vec<ActionSideEffects> {
        let attacker_bundle = self.get_battle_bundle(&attacker);
        let (attacker, attacker_pokemon, _, _) = attacker_bundle;

        let defender_bundle = self.get_battle_bundle(&defender);
        let (_, defender_pokemon, defender_data, _) = defender_bundle;
        let defender_type = get_effective_type(defender_pokemon, defender_data);

        let move_data = attack.data();

        // Effectiveness Calculation
        let effective_move_type = get_effective_move_type(attacker_bundle, &self.field, attack, move_data);
        let effectiveness = defender_type.defending_against(&effective_move_type);

        if let Effectiveness::Immune = effectiveness {
            return vec![ActionSideEffects::NoEffect(Cause::Natural)]
        }

        match attack {
            Move::Endeavor => {
                let user_hp = attacker_pokemon.current_hp;
                let target_hp = if defender_data.substituted > 0 {
                    defender_data.substituted
                } else {
                    defender_pokemon.current_hp
                };
                if user_hp >= target_hp {
                    vec![ActionSideEffects::Failed(Cause::Natural)]
                } else {
                    self.calculate_standard_basic_damage(defender_bundle, attacker_bundle, attack, |start_hp| user_hp)
                }
            },
            Move::FinalGambit => {
                let damage = attacker_pokemon.current_hp;

                // User Damage technically happens first, so the user would lose in a 1 on 1 if dealing lethal damage
                let mut effects = vec![ActionSideEffects::BasicDamage {
                    damaged: attacker,
                    start_hp: damage,
                    end_hp: 0,
                    cause: Cause::Natural,
                    hung_on_cause: None
                }];

                copy_all(&mut effects,
                         self.calculate_standard_basic_damage(defender_bundle, attacker_bundle, attack, |start_hp| start_hp.saturating_sub(damage)));

                effects
            },
            Move::NightShade | Move::SeismicToss => {
                let damage = u16::from(attacker_pokemon.level);
                self.calculate_standard_basic_damage(defender_bundle, attacker_bundle, attack, |start_hp| start_hp.saturating_sub(damage))
            },
            Move::Psywave => {
                let damage = u32::from(attacker_pokemon.level) * (rand::thread_rng().gen_range(0..=100) + 50) / 100;
                let mut damage = u16::try_from(damage).unwrap_or(u16::MAX);
                if damage == 0 {
                    damage = 1;
                }
                self.calculate_standard_basic_damage(defender_bundle, attacker_bundle, attack, |start_hp| start_hp.saturating_sub(damage))
            },
            Move::PainSplit => {
                if defender_data.substituted > 0 {
                    return vec![ActionSideEffects::Failed(Cause::Natural)]
                }
                let calc = u32::from(attacker_pokemon.current_hp) + u32::from(defender_pokemon.current_hp);
                let hp = u16::try_from(calc / 2).unwrap(); // Per the existence of math, we can guarantee this will work fine.
                vec![ActionSideEffects::PainSplit {
                    user: attacker,
                    defender,
                    split_hp: hp
                }]
            }
            a => panic!("{:?} does not have one-off damage associated with it", a)
        }
    }

    fn do_revenge_damage(&mut self, attacker: Battler, attack: Move) -> Vec<ActionSideEffects> {
        let attacker_bundle = self.get_battle_bundle(&attacker);
        let (_, _, attacker_data, _) = attacker_bundle;

        let move_data = attack.data();

        if let Power::Revenge(t, fraction) = &move_data.power {
            let to_hit = match t {
                Some(dt) => {
                    // Find the most recent attack of the correct type that damaged the user
                    attacker_data.damage_this_turn.iter()
                        .filter(|(battler, attack, damage)| attack.data().damage_type == *dt)
                        .last()
                },
                None => {
                    // (Metal Burst) select the last damager
                    attacker_data.damage_this_turn.last()
                }
            };
            match to_hit {
                Some((target, _, damage)) => {
                    let payback_damage = Fraction::from(*damage) * Fraction::from(*fraction);
                    let payback_damage = payback_damage.to_u16().unwrap_or(0);
                    self.calculate_standard_basic_damage(self.get_battle_bundle(target), attacker_bundle, attack,
                                                         |start_hp| start_hp.saturating_sub(payback_damage))
                },
                None => vec![ActionSideEffects::Failed(Cause::Natural)]
            }
        } else {
            panic!("Called do_revenge_damage for move without Revenge power type")
        }
    }

    // endregion

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

    pub fn change_self_stat(&mut self, affected: Battler, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
        let bundle = self.get_battle_bundle(&affected);
        let (affected, pkmn, data, ability) = bundle;

        let (ability_cause, stages) = match ability {
            Ability::Simple => (Cause::Ability(affected, Ability::Simple), stages * 2),
            Ability::Contrary => (Cause::Ability(affected, Ability::Contrary), -stages),
            _ => (Cause::Natural, stages)
        };

        self._change_stat(bundle, stat, stages, ability_cause)
    }

    pub fn change_opponent_stat(&mut self, affecter: Battler, affected: Battler, stat: BattleStat, stages: i8) -> Vec<ActionSideEffects> {
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

    //endregion
}

//region oldstuff
// /// Gets the effective attack stat of a Pokemon on the field
// /// This takes into account:
// /// * Raw attack stat (or defense stat, if afflicted with Power Trick)
// /// * Attack stage
// /// * Burn (+ Immunity to burn attack drop if ability is Guts
// /// * Abilities
// /// * Items
// /// * Transform
// fn get_effective_attack(&self, id: &Battler) -> f64 {
//     let user = self.get_by_id(id).unwrap();
//     let atk = f64::from(user.get_effective_attack()); //Raw attack + stage multipliers + Power Trick
//
//     let ability_multiplier = match user.get_effective_ability() {
//         Ability::FlowerGift if self.field.is_sunny() => 1.5,
//         Ability::Guts if user.has_status_condition() => 1.5,
//         Ability::HugePower => 2.0,
//         Ability::Hustle => 1.5,
//         Ability::PurePower => 2.0,
//         Ability::Defeatist if user.is_half_health_or_worse() => 0.5,
//         Ability::SlowStart if user.battle_data.turn_count < 5 => 0.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match user.pokemon.held_item {
//         Some(Item::ChoiceBand) => 1.5,
//         Some(Item::LightBall) if *user.get_effective_species() == Species::Pikachu => 2.0,
//         Some(Item::ThickClub) if *user.get_effective_species() == Species::Marowak => 2.0,
//         _ => 1.0
//     };
//
//     atk * ability_multiplier * item_multiplier
// }
//
// /// Gets the effective defense stat of a Pokemon on the field
// /// This takes into account:
// /// * Raw defense stat (or attack stat, if afflicted with Power Trick)
// /// * Defense stage
// /// * Abilities
// /// * Items
// /// * Transform
// fn get_effective_defense(&self, id: &Battler) -> f64 {
//     let user = self.get_by_id(id).unwrap();
//     let def = f64::from(user.get_effective_defense()); //Raw defense + stage multipliers + Power Trick
//
//     let ability_multiplier = match user.get_effective_ability() {
//         Ability::MarvelScale if user.has_status_condition() => 1.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match user.pokemon.held_item {
//         Some(Item::Eviolite) if !user.get_effective_species().is_fully_evolved() => 1.5,
//         Some(Item::MetalPowder) if user.pokemon.species == Species::Ditto && user.battle_data.transformed.is_none() => 2.0,
//         _ => 1.0
//     };
//
//     def * ability_multiplier * item_multiplier
// }
//
// /// Gets the effective special attack stat of a Pokemon on the field
// /// This takes into account:
// /// * Raw special attack stat
// /// * Special Attack stage
// /// * Abilities
// /// * Items
// /// * Transform
// /// * Plus or Minus, if the Ally also has Plus or Minus (no restrictions on two Plus or two Minus)
// fn get_effective_special_attack(&self, id: &Battler) -> f64 {
//     let user = self.get_by_id(id).unwrap();
//     let spa = f64::from(user.get_effective_special_attack()); //Raw SpA + stage multipliers
//
//     let ability_multiplier = match user.get_effective_ability() {
//         Ability::Plus | Ability::Minus => {
//             match self.get_ally(&id) {
//                 Some(p) if *p.get_effective_ability() == Ability::Plus
//                     || *p.get_effective_ability() == Ability::Minus => 1.5,
//                 _ => 1.0
//             }
//         }
//         Ability::SolarPower if self.field.is_sunny() => 1.5,
//         Ability::Defeatist if user.is_half_health_or_worse() => 0.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match user.pokemon.held_item {
//         Some(Item::ChoiceSpecs) => 1.5,
//         Some(Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaTooth))
//         if *user.get_effective_species() == Species::Clamperl => 2.0,
//         Some(Item::LightBall) if *user.get_effective_species() == Species::Pikachu => 2.0,
//         Some(Item::SoulDew) if *user.get_effective_species() == Species::Latias || *user.get_effective_species() == Species::Latios => 1.5,
//         _ => 1.0
//     };
//
//     spa * ability_multiplier * item_multiplier
// }
//
// /// Gets the effective special defense stat of a Pokemon on the field
// /// This takes into account:
// /// * Raw special defense stat
// /// * Special Defense stage
// /// * Abilities
// /// * Items
// /// * Transform
// fn get_effective_special_defense(&self, id: &Battler) -> f64 {
//     let user = self.get_by_id(id).unwrap();
//     let spd = f64::from(user.get_effective_special_defense());
//
//     let ability_multiplier = match user.get_effective_ability() {
//         Ability::FlowerGift if self.field.is_sunny() => 1.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match user.pokemon.held_item {
//         Some(Item::AssaultVest) => 1.5,
//         Some(Item::EvolutionHeldItem(EvolutionHeldItem::DeepSeaScale))
//         if *user.get_effective_species() == Species::Clamperl => 2.0,
//         Some(Item::Eviolite) if !user.get_effective_species().is_fully_evolved() => 1.5,
//         Some(Item::MetalPowder) if user.pokemon.species == Species::Ditto => 1.5,
//         Some(Item::SoulDew) if *user.get_effective_species() == Species::Latias || *user.get_effective_species() == Species::Latios => 1.5,
//         _ => 1.0
//     };
//
//     spd * ability_multiplier * item_multiplier
// }
//
// /// Get the current effective speed of a specific Pokemon on a specific side of the field
// /// This takes into account:
// /// * Raw speed stat of the Pokemon
// /// * Speed stage
// /// * Abilities, if applicable to the current state of the field
// /// * Items, if applicable to the current state of the field
// /// * Other statuses, such as paralysis or Tailwind
// /// * Transform
// fn get_effective_speed(&self, id: &Battler) -> f64 {
//     let pokemon = self.get_by_id(id).unwrap();
//     let side = self.get_side_by_id(id);
//     let speed = pokemon.get_effective_speed(); //Raw speed + stage multipliers
//
//     // Ability modifiers
//     let ability_multiplier = match pokemon.get_effective_ability() {
//         Ability::Chlorophyll if self.field.is_sunny() => 2.0,
//         Ability::SandRush if self.field.is_sandstorm() => 2.0,
//         Ability::SwiftSwim if self.field.is_rain() => 2.0,
//         Ability::SlushRush if self.field.is_hail() => 2.0,
//         Ability::QuickFeet if pokemon.has_status_condition() => 1.5,
//         Ability::Unburden if pokemon.battle_data.lost_held_item => 2.0,
//         Ability::SlowStart if pokemon.battle_data.turn_count < 5 => 0.5,
//         _ => 1.0
//     };
//
//     let item_multiplier = match pokemon.pokemon.held_item {
//         Some(Item::ChoiceScarf) => 1.5,
//         Some(Item::QuickPowder) if pokemon.pokemon.species == Species::Ditto => 2.0,
//         _ => 1.0
//     };
//
//     let mut eff_speed = f64::from(speed) * ability_multiplier * item_multiplier;
//     if side.tailwind > 0 {
//         eff_speed *= 2.0;
//     }
//
//     if pokemon.pokemon.status.paralysis {
//         eff_speed *= 0.5;
//     }
//
//     eff_speed
// }
//
// fn get_effective_crit_rate(&self, id: &Battler, attack: &Move) -> u8 {
//     let user = self.get_by_id(id).unwrap();
//
//     let mut stage = attack.get_crit_rate().unwrap_or(0);
//
//     stage += match *user.get_effective_ability() {
//         Ability::SuperLuck => 1,
//         _ => 0
//     };
//
//     stage += match user.pokemon.held_item {
//         Some(Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw)) => 1,
//         Some(Item::Leek) if *user.get_effective_species() == Species::Farfetchd => 2,
//         Some(Item::LuckyPunch) if *user.get_effective_species() == Species::Chansey => 2,
//         _ => 0
//     };
//
//     if user.battle_data.focused {
//         stage += 2;
//     }
//
//     stage
// }
//
// /// Gets the factor of accuracy for a user hitting the defender with the move. This is, essentially,
// /// the probability (out of 100) of a hit landing.
// /// Takes into account:
// /// * User Accuracy and Target Evasion
// /// * Abilities (User and Target)
// /// * Held Items (User and Target)
// /// * Allied Pokemon with Victory Star (doesn't stack if you send out two Victini)
// /// Documentation is vague on what part of the equation the modifiers are applied to. Some moves
// /// affect the accuracy of the move, while others affect the accuracy of the Pokemon
// fn get_accuracy_factor(&self, user_id: &Battler, attack: &Move, defender_id: &Battler) -> f64 {
//     let user = self.get_by_id(user_id).unwrap();
//     let defender = self.get_by_id(defender_id).unwrap();
//     let move_data = attack.data();
//
//     let raw_accuracy =
//         //Clamping is unnecessary, since it is handled in this method
//         core::determine_accuracy_stat_multiplier(user.battle_data.accuracy_stage - defender.battle_data.evasion_stage);
//
//     let mut field_modifier = match self.field.weather {
//         Some(Weather::Fog) => 3f64 / 5f64,
//         _ => 1.0
//     };
//     if self.field.gravity > 0 {
//         field_modifier *= 5f64 / 3f64
//     }
//     match self.get_ally(&user_id) {
//         Some(b) if *b.get_effective_ability() == Ability::VictoryStar => field_modifier *= 1.1,
//         _ => {}
//     }
//
//     let user_ability_modifier = match user.get_effective_ability() {
//         Ability::CompoundEyes => 1.3,
//         Ability::VictoryStar => 1.1,
//         Ability::Hustle if move_data.damage_type == DamageType::Physical => 0.8,
//         _ => {
//             match self.get_ally(&user_id) {
//                 Some(b) if *b.get_effective_ability() == Ability::VictoryStar => 1.1,
//                 _ => 1.0
//             }
//         }
//     };
//
//     let user_item_modifier = match user.pokemon.held_item {
//         Some(Item::WideLens) => 1.1,
//         Some(Item::ZoomLens) if defender.battle_data.move_used_this_turn.is_some() => 1.2,
//         _ => 1.0
//     };
//
//     let defender_ability_modifier = match defender.get_effective_ability() {
//         Ability::WonderSkin if move_data.damage_type == DamageType::Status => 0.5,
//         Ability::SandVeil if self.field.is_sandstorm() => 4f64 / 5f64,
//         Ability::SnowCloak if self.field.is_hail() => 4f64 / 5f64,
//         Ability::TangledFeet if defender.battle_data.is_confused() => 0.5,
//         _ => 1.0
//     };
//
//     let defender_item_modifier = match defender.pokemon.held_item {
//         Some(Item::BrightPowder) | Some(Item::Incense(Incense::LaxIncense)) => 0.9,
//         _ => 1.0
//     };
//
//     raw_accuracy * field_modifier * user_ability_modifier * user_item_modifier * defender_ability_modifier * defender_item_modifier
// }
//
// /// Gets the effectiveness of am move, taking into account all things that could modify it
// /// This takes into account:
// /// * Potentially swapped or modified types
// /// TODO: Abilities + Items that modify effectiveness
// fn get_effective_move_effectiveness(&self, attack: &Move, defender_id: &Battler) -> Effectiveness {
//     let defender = self.get_by_id(defender_id).unwrap();
//     let move_data = attack.data();
//
//     let effective_type = defender.get_effective_type();
//
//     effective_type.defending_against(&move_data._type)
// }
//
// fn apply_damage(&mut self, to_hurt: &Battler, hp_drop: u16) -> (u16, u16, bool) {
//     let defender_data = self.get_by_id(to_hurt).unwrap();
//
//     if defender_data.battle_data.substituted > 0 {
//         let target_current_hp = defender_data.battle_data.substituted;
//         let target_end_hp = target_current_hp.saturating_sub(hp_drop);
//         self.do_by_id(to_hurt, |_, data| data.substituted = target_end_hp);
//
//         (target_current_hp, target_end_hp, true)
//     } else {
//         let target_current_hp = defender_data.pokemon.current_hp;
//         let target_end_hp = target_current_hp.saturating_sub(hp_drop);
//         self.do_by_id(to_hurt, |pkmn, _| pkmn.current_hp = target_end_hp);
//
//         (target_current_hp, target_end_hp, false)
//     }
// }
//endregion

//region oldstuff2
// struct BattlePokemon<'a>(&'a Pokemon, &'a BattleData);
// impl <'a> BattlePokemon<'a> {
//     /// Get the species of this Pokemon. Takes Transform into account
//     fn get_effective_species(&self) -> &Species {
//         match &self.1.transformed {
//             None => &self.0.species,
//             Some(t) => &t.species
//         }
//     }
//
//     /// Get the effective attack of this Pokemon. Takes Transform, Power Trick, and Attack stage into account
//     fn get_effective_attack(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.attack_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(if self.1.power_trick {self.0.defense.value} else {self.0.attack.value}),
//             Some(t) => f64::from(if self.1.power_trick {t.defense.value} else {t.attack.value})
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Get the effective defense of this Pokemon. Takes Transform, Power Trick, and Defense stage into account
//     fn get_effective_defense(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.defense_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(if self.1.power_trick {self.0.attack.value} else {self.0.defense.value}),
//             Some(t) => f64::from(if self.1.power_trick {t.attack.value} else {t.defense.value})
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Get the effective special attack of this Pokemon. Takes Transform and Sp. Attack stage into account
//     fn get_effective_special_attack(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.special_attack_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(self.0.special_attack.value),
//             Some(t) => f64::from(t.special_attack.value)
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Get the effective special defense of this Pokemon. Takes Transform and Sp. Defense stage into account
//     fn get_effective_special_defense(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.special_defense_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(self.0.special_defense.value),
//             Some(t) => f64::from(t.special_defense.value)
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Get the effective speed of this Pokemon. Takes Transform and Speed stage into account
//     fn get_effective_speed(&self) -> u16 {
//         let multiplier = core::determine_stat_multiplier(self.1.speed_stage);
//         let raw = match &self.1.transformed {
//             None => f64::from(self.0.speed.value),
//             Some(t) => f64::from(t.speed.value)
//         } * multiplier;
//         raw as u16
//     }
//
//     /// Check if this Pokemon has a status condition (may be expanded to handle volatile status ailments)
//     fn has_status_condition(&self) -> bool {
//         self.0.status.has_status_condition()
//     }
//
//     /// Get the effective ability of this Pokemon. Takes Transform and temporary Ability changes into effect
//     fn get_effective_ability(&self) -> &Ability {
//         self.1.temp_ability.as_ref().unwrap_or_else(|| {
//             match &self.1.transformed {
//                 None => self.0.get_ability(),
//                 Some(t) => t.get_ability()
//             }
//         })
//     }
//
//     /// Get the effective type(s) of this Pokemon. Taks Transform and temporary Type changes into effect
//     fn get_effective_type(&self) -> &PokemonType {
//         self.1.temp_type.as_ref().unwrap_or_else(|| {
//             match &self.1.transformed {
//                 None => &self.0.species.data()._type,
//                 Some(t) => &t.species.data()._type
//             }
//         })
//     }
//
//     fn is_half_health_or_worse(&self) -> bool {
//         let current = u32::from(self.0.current_hp);
//         let max = u32::from(self.0.hp.value);
//         current * 2u32 <= max
//     }
//
//     fn is_quarter_health_or_worse(&self) -> bool {
//         let current = u32::from(self.0.current_hp);
//         let max = u32::from(self.0.hp.value);
//         current * 4u32 <= max
//     }
// }
//endregion