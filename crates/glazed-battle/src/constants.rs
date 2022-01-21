use std::ops::RangeInclusive;
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;

// Damage-related
/// If a move hits multiple targets, the damage is multiplied by this
pub const MULTI_TARGET_MULTIPLIER: (u8, u8) = (3, 4);
/// If the move type matches weather (fire/sun or water/rain), the damage is multiplied by this
pub const GOOD_WEATHER_MULTIPLIER: (u8, u8) = (3, 2);
/// If the move type matches the wrong weather (fire/rain or water/sun), the damage is multiplied by this
pub const BAD_WEATHER_MULTIPLIER: (u8, u8) = (1, 2);
/// If the move crits, damage is multiplied by this
pub const CRIT_MULTIPLIER: (u8, u8) = (2, 1);
/// Each move strike is varied by multiplying by a random number in this range
pub const DAMAGE_VARIABILITY: RangeInclusive<f64> = 0.85..=1.0;
/// If the move type matches the attacker's type (STAB), the damage is multiplied by this
pub const STAB_MULTIPLIER: (u8, u8) = (3, 2);
/// If the attacker is burned + using a physical attack, the damage is multiplied by this
pub const BURN_MULTIPLIER: (u8, u8) = (1, 2);
/// If the defender is minimized, and the attacker
pub const MINIMIZE_MULTIPLIER: (u8, u8) = (2, 1);
/// The fraction of max HP that Nightmare takes each turn
pub const NIGHTMARE_MULTIPLIER: (u8, u8) = (1, 4);
/// The fraction of max HP that Curse takes each turn
pub const CURSE_MULTIPLIER: (u8, u8) = (1, 4);

/// Max # of stages a stat can be at
pub const MAX_STAGE: i8 = 6;
/// Min # of stages a stat can be at
pub const MIN_STAGE: i8 = -6;
/// Chance of a frozen Pokemon thawing at the start of a turn
pub const THAW_CHANCE: f64 = 0.2;
/// Chance of a paralyzed Pokemon being unable to attack
pub const FULL_PARALYSIS_CHANCE: f64 = 0.25;
/// Number of turns a Pokemon can sleep for, equally distributed
pub const SLEEP_TURNS_RANGE: RangeInclusive<u8> = 1..=3;
/// Number of turns a Pokemon can be confused for
pub const CONFUSION_TURN_RANGE: RangeInclusive<u8> = 2..=5;
/// Chance a confused Pokemon will hit itself
pub const CONFUSION_HIT_CHANCE: f64 = 0.5;
/// Power of a self-inflicted hit from confusion
pub const CONFUSION_POWER: u16 = 40;
/// Chance an infatuated Pokemon will do nothing
// pub const INFATUATION_INACTION_CHANCE: f64 = 0.5;
/// Number of turns a Pokemon may be bound for
pub const BOUND_TURN_RANGE: RangeInclusive<u8> = 4..=5;
/// Number of turns a Pokemon will be bound for, if the attacker is holding the Grip Claw
pub const BOUND_TURN_GRIP_CLAW: u8 = 7;
/// Reciprocal of the relative damage a bound Pokemon takes at the end of a round. 1/8 of max HP
pub const BOUND_HP: u8 = 8;
/// Reciprocal of the relative damage a bound Pokemon takes at the end of a round, if the binder held a Binding Band. 1/6 of max HP
pub const BOUND_HP_BINDING_BAND: u8 = 6;
/// Weights for the number of turns for a multi-strike move. Can be any Distribution
pub const MULTI_HIT_RANGE: MultiHitDistribution = MultiHitDistribution;
/// Number of turns for a multi-strike move, if the user has Skill Link
pub const MULTI_HIT_SKILL_LINK: u8 = 5;
/// Number of turns a Pokemon may thrash for
pub const THRASH_RANGE: RangeInclusive<u8> = 2..=3;
/// Number of turns a move is disabled for
pub const DISABLE_TURN_COUNT: u8 = 4;
/// Number of turns mist is active on a field
pub const MIST_TURN_COUNT: u8 = 5;
/// Number of turns bide lasts before an attack
pub const BIDE_TURN_COUNT: u8 = 2;
/// Number of turns a resting Pokemon sleeps
pub const REST_SLEEP_TURN_COUNT: u8 = 3;
/// Number of PP deducted from Spite
pub const SPITE_PP_COUNT: u8 = 4;
/// The number of subsequent protection moves that can be used before the accuracy stops dropping
pub const PROTECTION_CAP: u8 = 4;

pub const SCREEN_TURN_COUNT: u8 = 5;

pub const SCREEN_TURN_COUNT_LIGHT_CLAY: u8 = 8;

/// Table used to convert weight to base power for weight-based moves (Low Kick, Grass Knot)
pub fn weight_to_power_map(weight: u16) -> u16 {
    match weight {
        0..=99 => 20,
        100..=249 => 40,
        250..=499 => 60,
        500..=999 => 80,
        1000..=1999 => 100,
        2000..=u16::MAX => 120
    }
}

/// Table used to convert weight ratio to base power for weight-ratio-based moves (Heavy Slam, Heat Crash)
pub fn weight_ratio_to_power_map(attacker_weight: u16, defender_weight: u16) -> u16 {
    let ratio = (attacker_weight as f64) / (defender_weight as f64);
    if ratio > 0.5 { 40 }
    else if ratio > 0.3335 { 60 }
    else if ratio > 0.2501 { 80 }
    else if ratio > 0.2001 { 100 }
    else { 120 }
}

/// Table to convert HP Ratio to base power for Flail
pub fn hp_to_power_map(current_hp: u16, max_hp: u16) -> u16 {
    let ratio = f64::from(current_hp) / f64::from(max_hp);
    if ratio > 0.6875 { 20 }
    else if ratio > 0.3542 { 40 }
    else if ratio > 0.2083 { 80 }
    else if ratio > 0.1042 { 100 }
    else if ratio > 0.0417 { 150 }
    else { 200 }
}

/// One-off structure to allow configuration of the Multi-Hit Distribution
pub struct MultiHitDistribution;
impl Distribution<u8> for MultiHitDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const CHOICES: [u8; 4] = [2, 3, 4, 5]; // Possible turn amounts.
        const WEIGHTS: [u8; 4] = [35, 35, 15, 15]; // Probability of each choice. Weighted, so doesn't need to equal 100.
        let dist = WeightedIndex::new(&WEIGHTS).unwrap();
        CHOICES[dist.sample(rng)]
    }
}