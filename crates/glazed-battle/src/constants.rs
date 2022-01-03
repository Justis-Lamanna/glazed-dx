use std::ops::RangeInclusive;
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;

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
/// Chance a confused Pokemon will hit itself
pub const CONFUSION_HIT_CHANCE: f64 = 0.5;
/// Power of a self-inflicted hit from confusion
pub const CONFUSION_POWER: u16 = 40;
/// Chance an infatuated Pokemon will do nothing
pub const INFATUATION_INACTION_CHANCE: f64 = 0.5;
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