use std::collections::{BTreeMap, HashSet};

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize};

use crate::abilities::Ability;
use crate::attack::Move;
use crate::core::OneOrTwo;
use crate::item::{Item, Pokeball};
use crate::locations::Location;
use crate::species::Species;
use crate::types::Type;

pub const SHININESS_CHANCE: u16 = 16;

/// Represents the probability of a Pokemon being male or female (or neither)
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum GenderRatio {
    None,
    Proportion(u8, u8)
}
impl GenderRatio {
    pub const MALE_ONLY: GenderRatio = GenderRatio::Proportion(1, 0);
    pub const FEMALE_ONLY: GenderRatio = GenderRatio::Proportion(0, 1);
    pub const ONE_TO_ONE: GenderRatio = GenderRatio::Proportion(1, 1);
    pub const ONE_TO_THREE: GenderRatio = GenderRatio::Proportion(1, 3);
    pub const ONE_TO_SEVEN: GenderRatio = GenderRatio::Proportion(1, 7);
    pub const THREE_TO_ONE: GenderRatio = GenderRatio::Proportion(3, 1);
    pub const SEVEN_TO_ONE: GenderRatio = GenderRatio::Proportion(7, 1);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    None
}
impl Gender {
    /// Check if this gender is opposite another (M/F or F/M)
    pub fn can_infatuate(&self, other: Gender) -> bool {
        match (self, other) {
            (Gender::Male, Gender::Female) | (Gender::Female, Gender::Male) => true,
            _ => false
        }
    }
}

/// Represents an Egg Group, i.e. the Compatibility of two Pokemon
#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum EggGroup {
    Monster,
    Water1,
    Water2,
    Water3,
    Humanshape,
    Bug,
    Mineral,
    Flying,
    Indeterminate,
    Ground,
    Fairy,
    Ditto,
    Plant,
    Dragon
}

/// Represents the Egg Groups a Pokemon has
#[derive(Debug, Copy, Clone)]
pub enum PokemonEggGroup {
    None,
    One(EggGroup),
    Two(EggGroup, EggGroup)
}
impl<'de> Deserialize<'de> for PokemonEggGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let opt = Option::<Vec<EggGroup>>::deserialize(deserializer)?;
        match opt {
            None => Ok(Self::None),
            Some(mut v) => {
                if v.len() == 0 { Ok(Self::None) }
                else if v.len() == 1 { Ok(Self::One(v.pop().unwrap())) }
                else if v.len() == 2 {Ok(Self::Two(v.pop().unwrap(), v.pop().unwrap())) }
                else {
                    use serde::de::Error;
                    Err(D::Error::invalid_length(v.len(), &"Pokemon should have 0, 1, or 2 Egg Groups")) }
            }
        }
    }
}
impl PokemonEggGroup {
    pub fn as_set(&self) -> HashSet<EggGroup> {
        match self {
            PokemonEggGroup::None => HashSet::new(),
            PokemonEggGroup::One(a) => HashSet::from([*a]),
            PokemonEggGroup::Two(a, b) => HashSet::from([*a, *b]),
        }
    }
}

/// Represents how much EXP is required to advance levels
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum LevelRate {
    Erratic, Fast, MediumFast, MediumSlow, Slow, Fluctuating
}
impl LevelRate {
    const ERRATIC_RATE: [u32; 101] = [
        0, 0, 15, 52, 122, 237, 406, 637, 942, 1326, 1800, 2369, 3041, 3822, 4719, 5737, 6881, 8155, 9564, 11111, 12800, 14632, 16610, 18737, 21012, 23437, 26012, 28737, 31610, 34632, 37800, 41111, 44564, 48155, 51881, 55737, 59719, 63822, 68041, 72369, 76800, 81326, 85942, 90637, 95406, 100237, 105122, 110052, 115015, 120001, 125000, 131324, 137795, 144410, 151165, 158056, 165079, 172229, 179503, 186894, 194400, 202013, 209728, 217540, 225443, 233431, 241496, 249633, 257834, 267406, 276458, 286328, 296358, 305767, 316074, 326531, 336255, 346965, 357812, 367807, 378880, 390077, 400293, 411686, 423190, 433572, 445239, 457001, 467489, 479378, 491346, 501878, 513934, 526049, 536557, 548720, 560922, 571333, 583539, 591882, 600000
    ];
    const SLOW_RATE: [u32; 101] = [
        0, 0, 10, 33, 80, 156, 270, 428, 640, 911, 1250, 1663, 2160, 2746, 3430, 4218, 5120, 6141, 7290, 8573, 10000, 11576, 13310, 15208, 17280, 19531, 21970, 24603, 27440, 30486, 33750, 37238, 40960, 44921, 49130, 53593, 58320, 63316, 68590, 74148, 80000, 86151, 92610, 99383, 106480, 113906, 121670, 129778, 138240, 147061, 156250, 165813, 175760, 186096, 196830, 207968, 219520, 231491, 243890, 256723, 270000, 283726, 297910, 312558, 327680, 343281, 359370, 375953, 393040, 410636, 428750, 447388, 466560, 486271, 506530, 527343, 548720, 570666, 593190, 616298, 640000, 664301, 689210, 714733, 740880, 767656, 795070, 823128, 851840, 881211, 911250, 941963, 973360, 1005446, 1038230, 1071718, 1105920, 1140841, 1176490, 1212873, 1250000
    ];
    const MEDIUM_FAST_RATE: [u32; 101] = [
        0, 0, 8, 27, 64, 125, 216, 343, 512, 729, 1000, 1331, 1728, 2197, 2744, 3375, 4096, 4913, 5832, 6859, 8000, 9261, 10648, 12167, 13824, 15625, 17576, 19683, 21952, 24389, 27000, 29791, 32768, 35937, 39304, 42875, 46656, 50653, 54872, 59319, 64000, 68921, 74088, 79507, 85184, 91125, 97336, 103823, 110592, 117649, 125000, 132651, 140608, 148877, 157464, 166375, 175616, 185193, 195112, 205379, 216000, 226981, 238328, 250047, 262144, 274625, 287496, 300763, 314432, 328509, 343000, 357911, 373248, 389017, 405224, 421875, 438976, 456533, 474552, 493039, 512000, 531441, 551368, 571787, 592704, 614125, 636056, 658503, 681472, 704969, 729000, 753571, 778688, 804357, 830584, 857375, 884736, 912673, 941192, 970299, 1000000
    ];
    const FAST_RATE: [u32; 101] = [
        0, 0, 6, 21, 51, 100, 172, 274, 409, 583, 800, 1064, 1382, 1757, 2195, 2700, 3276, 3930, 4665, 5487, 6400, 7408, 8518, 9733, 11059, 12500, 14060, 15746, 17561, 19511, 21600, 23832, 26214, 28749, 31443, 34300, 37324, 40522, 43897, 47455, 51200, 55136, 59270, 63605, 68147, 72900, 77868, 83058, 88473, 94119, 100000, 106120, 112486, 119101, 125971, 133100, 140492, 148154, 156089, 164303, 172800, 181584, 190662, 200037, 209715, 219700, 229996, 240610, 251545, 262807, 274400, 286328, 298598, 311213, 324179, 337500, 351180, 365226, 379641, 394431, 409600, 425152, 441094, 457429, 474163, 491300, 508844, 526802, 545177, 563975, 583200, 602856, 622950, 643485, 664467, 685900, 707788, 730138, 752953, 776239, 800000
    ];
    const MEDIUM_SLOW_RATE: [u32; 101] = [
        0, 0, 9, 57, 96, 135, 179, 236, 314, 419, 560, 742, 973, 1261, 1612, 2035, 2535, 3120, 3798, 4575, 5460, 6458, 7577, 8825, 10208, 11735, 13411, 15244, 17242, 19411, 21760, 24294, 27021, 29949, 33084, 36435, 40007, 43808, 47846, 52127, 56660, 61450, 66505, 71833, 77440, 83335, 89523, 96012, 102810, 109923, 117360, 125126, 133229, 141677, 150476, 159635, 169159, 179056, 189334, 199999, 211060, 222522, 234393, 246681, 259392, 272535, 286115, 300140, 314618, 329555, 344960, 360838, 377197, 394045, 411388, 429235, 447591, 466464, 485862, 505791, 526260, 547274, 568841, 590969, 613664, 636935, 660787, 685228, 710266, 735907, 762160, 789030, 816525, 844653, 873420, 902835, 932903, 963632, 995030, 1027103, 1059860
    ];
    const FLUCTUATING_RATE: [u32; 101] = [
        0, 0, 4, 13, 32, 65, 112, 178, 276, 393, 540, 745, 967, 1230, 1591, 1957, 2457, 3046, 3732, 4526, 5440, 6482, 7666, 9003, 10506, 12187, 14060, 16140, 18439, 20974, 23760, 26811, 30146, 33780, 37731, 42017, 46656, 50653, 55969, 60505, 66560, 71677, 78533, 84277, 91998, 98415, 107069, 114205, 123863, 131766, 142500, 151222, 163105, 172697, 185807, 196322, 210739, 222231, 238036, 250562, 267840, 281456, 300293, 315059, 335544, 351520, 373744, 390991, 415050, 433631, 459620, 479600, 507617, 529063, 559209, 582187, 614566, 639146, 673863, 700115, 737280, 765275, 804997, 834809, 877201, 908905, 954084, 987754, 1035837, 1071552, 1122660, 1160499, 1214753, 1254796, 1312322, 1354652, 1415577, 1460276, 1524731, 1571884, 1640000
    ];

    /// Get the amount of EXP needed to make it to a specific level
    pub fn experience_for_level(&self, level: u8) -> u32 {
        let rate = match self {
            LevelRate::Erratic => LevelRate::ERRATIC_RATE,
            LevelRate::Fast => LevelRate::FAST_RATE,
            LevelRate::MediumFast => LevelRate::MEDIUM_FAST_RATE,
            LevelRate::MediumSlow => LevelRate::MEDIUM_SLOW_RATE,
            LevelRate::Slow => LevelRate::SLOW_RATE,
            LevelRate::Fluctuating => LevelRate::FLUCTUATING_RATE
        };
        return rate[level as usize];
    }

    /// Get the level the Pokemon should be at with the given EXP
    pub fn level_for_experience(&self, experience: u32) -> u8 {
        let rate = match self {
            LevelRate::Erratic => LevelRate::ERRATIC_RATE,
            LevelRate::Fast => LevelRate::FAST_RATE,
            LevelRate::MediumFast => LevelRate::MEDIUM_FAST_RATE,
            LevelRate::MediumSlow => LevelRate::MEDIUM_SLOW_RATE,
            LevelRate::Slow => LevelRate::SLOW_RATE,
            LevelRate::Fluctuating => LevelRate::FLUCTUATING_RATE
        };
        for (level, xp) in rate.iter().enumerate() {
            if experience == *xp {
                return level as u8;
            } else if experience < *xp {
                return (level - 1) as u8;
            }
        }
        return 100;
    }
}

/// Represents the Pokedex color of a Pokemon
#[derive(Debug)]
pub enum Color {
    Red, Blue, Yellow, Green, Black, Brown, Purple, Gray, White, Pink
}

/// Represents the six members of stat data for a Pokemon species as a whole
#[derive(Debug, Clone, Deserialize)]
pub struct Stats(pub Stat,pub Stat,pub Stat,pub Stat,pub Stat,pub Stat);

/// Represents data tied to a Stat for a Pokemon species as a whole
#[derive(Debug, Clone, Deserialize)]
pub struct Stat {
    pub base_stat: u8,
    #[serde(default)]
    ev: u8
}
impl Stat {
    pub const fn new(base: u8, ev: u8) -> Stat {
        Stat {
            base_stat: base,
            ev
        }
    }
}

/// Represents data on a Pokemon species as a whole
#[derive(Debug, Deserialize)]
pub struct SpeciesData {
    #[serde(rename = "type")]
    pub _type: OneOrTwo<Type>,
    pub ability: OneOrTwo<Ability>,
    pub hidden_ability: Option<Ability>,
    pub gender_ratio: GenderRatio,
    pub catch_rate: u8,
    pub egg_group: PokemonEggGroup,
    pub egg_cycles: u8,
    pub height: u8,
    pub weight: u16,
    pub base_exp_yield: u16,
    pub level_rate: LevelRate,
    pub stats: Stats,
    pub base_friendship: u8,
    pub level_up_moves: BTreeMap<u8, Vec<Move>>,
    pub egg_moves: Option<Vec<Move>>
}
impl SpeciesData {
    pub fn get_all_knowable_moves(&self) -> HashSet<Move> {
        let mut set = HashSet::new();
        for (_, pool) in &self.level_up_moves {
            set.extend(pool);
        }
        set
    }

    /// Get a list of all moves this Pokemon can know at the specified level.
    /// The order of the list is from latest learned to earliest. As an example
    /// if this method was called with 40, a level 35 move would show up before a level
    /// 25 move.
    pub fn get_knowable_moves_for_level(&self, level: u8) -> Vec<Move> {
        (0..=level)
            .rev()
            .flat_map(|level| self.level_up_moves.get(&level))
            .flatten()
            .copied()
            .collect()
    }
}

/// Represents an individual member of a Pokemon species
#[derive(Debug, Deserialize)]
pub struct Pokemon {
    pub species: Species,
    pub gender: Gender,
    pub egg: bool,
    pub level_met: u8,
    pub nature: Nature,
    pub ability: AbilitySlot,
    pub poke_ball: Pokeball,
    pub held_item: Option<Item>,
    pub move_1: Option<MoveSlot>,
    pub move_2: Option<MoveSlot>,
    pub move_3: Option<MoveSlot>,
    pub move_4: Option<MoveSlot>,
    pub experience: u32,
    pub personality: u32,
    pub friendship: u8,
    pub original_trainer_id: u16,
    pub original_trainer_secret_id: u16,
    pub original_trainer_name: String,
    pub nickname: Option<String>,
    pub level: u8,
    pub markings: Markings,
    pub status: PokemonStatusCondition,
    pub pokerus: PokemonPokerusStatus,
    pub current_hp: u16,
    pub hp: StatSlot,
    pub attack: StatSlot,
    pub defense: StatSlot,
    pub special_attack: StatSlot,
    pub special_defense: StatSlot,
    pub speed: StatSlot,
    pub contest: PokemonContestStats,
    pub fateful_encounter: bool,
    pub date_caught: i64,
    pub location_caught: Location
}

/// Represents the Contest Stats and Winnings of this Pokemon
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PokemonContestStats {
    pub coolness: u8,
    pub beauty: u8,
    pub cuteness: u8,
    pub smartness: u8,
    pub toughness: u8,
    pub feel: u8,
    pub ribbons: Ribbons
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Markings: u8 {
        const CIRCLE    = 0b00000001;
        const TRIANGLE  = 0b00000010;
        const SQUARE    = 0b00000100;
        const HEART     = 0b00001000;
        const STAR      = 0b00010000;
        const DIAMOND   = 0b00100000;
    }
}
impl Default for Markings {
    fn default() -> Self {
        Markings::empty()
    }
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Ribbons: u32 {
        const LEAGUE_RIBBON         = 0b00000000000000000000000000000001; // Winning the league
        const COOL_RIBBON           = 0b00000000000000000000000000000010; // Contest ribbons
        const COOL_SUPER_RIBBON     = 0b00000000000000000000000000000100;
        const COOL_HYPER_RIBBON     = 0b00000000000000000000000000001000;
        const COOL_MASTER_RIBBON    = 0b00000000000000000000000000010000;
        const BEAUTY_RIBBON         = 0b00000000000000000000000000100000;
        const BEAUTY_SUPER_RIBBON   = 0b00000000000000000000000001000000;
        const BEAUTY_HYPER_RIBBON   = 0b00000000000000000000000010000000;

        const BEAUTY_MASTER_RIBBON  = 0b00000000000000000000000100000000;
        const CUTE_RIBBON           = 0b00000000000000000000001000000000;
        const CUTE_SUPER_RIBBON     = 0b00000000000000000000010000000000;
        const CUTE_HYPER_RIBBON     = 0b00000000000000000000100000000000;
        const CUTE_MASTER_RIBBON    = 0b00000000000000000001000000000000;
        const SMART_RIBBON          = 0b00000000000000000010000000000000;
        const SMART_SUPER_RIBBON    = 0b00000000000000000100000000000000;
        const SMART_HYPER_RIBBON    = 0b00000000000000001000000000000000;

        const SMART_MASTER_RIBBON   = 0b00000000000000010000000000000000;
        const TOUGH_RIBBON          = 0b00000000000000100000000000000000;
        const TOUGH_SUPER_RIBBON    = 0b00000000000001000000000000000000;
        const TOUGH_HYPER_RIBBON    = 0b00000000000010000000000000000000;
        const TOUGH_MASTER_RIBBON   = 0b00000000000100000000000000000000;
        const WINNING_RIBBON        = 0b00000000001000000000000000000000; // Battle Tower, lv 50
        const VICTORY_RIBBON        = 0b00000000010000000000000000000000; // Battle Tower, lv 100
        const ARTIST_RIBBON         = 0b00000000100000000000000000000000; // Put Pokemon portrait in a museum

        const EFFORT_RIBBON         = 0b00000001000000000000000000000000; // 510 EVs
        const GORGEOUS_RIBBON       = 0b00000010000000000000000000000000; // $10,000
        const ROYAL_RIBBON          = 0b00000100000000000000000000000000; // $100,000
        const GORGEOUS_ROYAL_RIBBON = 0b00001000000000000000000000000000; // $999,999
        const FOOTPRINT_RIBBON      = 0b00010000000000000000000000000000; // Max Happiness
        const UNKNOWN_A_RIBBON      = 0b00100000000000000000000000000000; //
        const UNKNOWN_B_RIBBON      = 0b01000000000000000000000000000000; //
        const UNKNOWN_C_RIBBON      = 0b10000000000000000000000000000000; //
    }
}
impl Default for Ribbons {
    fn default() -> Self {
        Ribbons::empty()
    }
}

/// Represents the status conditions of this Pokemon
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PokemonStatusCondition {
    pub sleep: u8,
    pub poison: bool,
    pub burn: bool,
    pub freeze: bool,
    pub paralysis: bool
}
impl PokemonStatusCondition {
    pub fn asleep() -> PokemonStatusCondition {
        PokemonStatusCondition {
            sleep: 5,
            ..Default::default()
        }
    }

    pub fn has_status_condition(&self) -> bool {
        self.sleep > 0 || self.poison || self.burn || self.freeze || self.paralysis
    }

    pub fn is_asleep(&self) -> bool {
        self.sleep > 0
    }
}

/// Represents the values tied to a given moveslot
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MoveSlot {
    pub attack: Move,
    pub pp: u8,
    pub pp_bonus: u8
}
impl MoveSlot {
    pub fn copy_for_transform(&self) -> Self {
        MoveSlot {
            attack: self.attack,
            pp: if self.pp > 5 {
                5
            } else {
                self.pp
            },
            pp_bonus: 0
        }
    }
}

/// Represents the values tied to a given stat (HP, Atk, etc.)
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct StatSlot {
    pub value: u16,
    pub iv: u8,
    ev: u8
}
impl StatSlot {
    /// Create a StatSlot for the HP of a Pokemon
    pub fn hp(base: u8, level: u8, iv: u8, ev: u8) -> StatSlot {
        let calculation = (2u32 * u32::from(base)) + u32::from(iv) + (u32::from(ev) / 4u32);
        let calculation = calculation * u32::from(level);
        let calculation = calculation / 100;
        let calculation = calculation + u32::from(level) + 10;
        StatSlot {
            value: calculation as u16,
            iv,
            ev
        }
    }

    /// Recalculate (in-place) the HP of this Pokemon
    pub fn recalculate_hp(&mut self, base: u8, level: u8) {
        let recalc = StatSlot::hp(base, level, self.iv, self.ev);
        self.value = recalc.value;
    }

    /// Create a StatSlot for this Pokemon
    pub fn stat(base: u8, level: u8, iv: u8, ev: u8, boost: NatureBoost) -> StatSlot {
        let calculation = (2u32 * u32::from(base)) + u32::from(iv) + (u32::from(ev) / 4u32);
        let calculation = calculation * u32::from(level);
        let calculation = calculation / 100;
        let calculation = calculation + 5;
        let calculation = match boost {
            NatureBoost::Increased => (calculation * 11) / 10,
            NatureBoost::Decreased => (calculation * 9) / 10,
            NatureBoost::Neutral => calculation
        };
        StatSlot {
            value: calculation as u16,
            iv,
            ev
        }
    }

    /// Recalculate (in-place) the stat of this Pokemon
    fn recalculate(&mut self, base: u8, level: u8, boost: NatureBoost) {
        let recalc = StatSlot::stat(base, level, self.iv, self.ev, boost);
        self.value = recalc.value;
    }
}

/// Represents which Ability the Pokemon has
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AbilitySlot {
    SlotOne,
    SlotTwo,
    Hidden
}
impl Distribution<AbilitySlot> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AbilitySlot {
        if rng.gen() {
            AbilitySlot::SlotOne
        } else {
            AbilitySlot::SlotTwo
        }
    }
}

/// Represents the stage of Pokerus the Pokemon is at
#[derive(Debug, Serialize, Deserialize)]
pub enum PokemonPokerusStatus {
    None,
    Infected(u8),
    Cured
}

/// Represents one of the 25 Natures of a Pokemon
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Nature {
    Hardy, Docile, Serious, Bashful, Quirky, //Neutrals
    Lonely, Brave, Adamant, Naughty, //Attack Up
    Bold, Relaxed, Impish, Lax, //Defense Up
    Timid, Hasty, Jolly, Naive, //SPA Up
    Modest, Mild, Quiet, Rash, //SPD up
    Calm, Gentle, Sassy, Careful //Speed up
}

/// The result of checking if a stat is boosted by this nature
#[derive(Copy, Clone, Debug)]
pub enum NatureBoost {
    Increased,
    Decreased,
    Neutral
}
impl Nature {
    /// Get whether the boost, if any, this nature gives the attack stat
    pub fn get_attack_boost(&self) -> NatureBoost {
        match self {
            Nature::Lonely | Nature::Brave | Nature::Adamant | Nature::Naughty => NatureBoost::Increased,
            Nature::Bold | Nature::Timid | Nature::Modest | Nature::Calm => NatureBoost::Decreased,
            _ => NatureBoost::Neutral
        }
    }

    /// Get whether the boost, if any, this nature gives the defense stat
    pub fn get_defense_boost(&self) -> NatureBoost {
        match self {
            Nature::Bold | Nature::Impish | Nature::Lax | Nature::Relaxed => NatureBoost::Increased,
            Nature::Lonely | Nature::Mild | Nature::Gentle | Nature::Hasty => NatureBoost::Decreased,
            _ => NatureBoost::Neutral
        }
    }

    /// Get whether the boost, if any, this nature gives the special attack stat
    pub fn get_special_attack_boost(&self) -> NatureBoost {
        match self {
            Nature::Modest | Nature::Mild | Nature::Rash | Nature::Quiet => NatureBoost::Increased,
            Nature::Adamant | Nature::Impish | Nature::Careful | Nature::Jolly => NatureBoost::Decreased,
            _ => NatureBoost::Neutral
        }
    }

    /// Get whether the boost, if any, this nature gives the special defense stat
    pub fn get_special_defense_boost(&self) -> NatureBoost {
        match self {
            Nature::Calm | Nature::Gentle | Nature::Careful | Nature::Sassy => NatureBoost::Increased,
            Nature::Naughty | Nature::Lax | Nature::Rash | Nature::Naive => NatureBoost::Decreased,
            _ => NatureBoost::Neutral
        }
    }

    /// Get whether the boost, if any, this nature gives the speed stat
    pub fn get_speed_boost(&self) -> NatureBoost {
        match self {
            Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive => NatureBoost::Increased,
            Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy => NatureBoost::Decreased,
            _ => NatureBoost::Neutral
        }
    }
}
/// Allows for random generation of a Nature
impl Distribution<Nature> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Nature {
        match rng.gen_range(0..=24) {
            0 => Nature::Hardy,
            1 => Nature::Lonely,
            2 => Nature::Brave,
            3 => Nature::Adamant,
            4 => Nature::Naughty,
            5 => Nature::Bold,
            6 => Nature::Docile,
            7 => Nature::Relaxed,
            8 => Nature::Impish,
            9 => Nature::Lax,
            10 => Nature::Timid,
            11 => Nature::Hasty,
            12 => Nature::Serious,
            13 => Nature::Jolly,
            14 => Nature::Naive,
            15 => Nature::Modest,
            16 => Nature::Mild,
            17 => Nature::Quiet,
            18 => Nature::Bashful,
            19 => Nature::Rash,
            20 => Nature::Calm,
            21 => Nature::Gentle,
            22 => Nature::Sassy,
            23 => Nature::Careful,
            24 => Nature::Quirky,
            _ => panic!("Expected number in range 0 - 24")
        }
    }
}

impl Pokemon {
    /// Restore this Pokemon to its max HP
    pub fn heal(&mut self) {
        self.current_hp = self.hp.value;
    }

    pub fn get_hidden_power_type(&self) -> Type {
        let bit = (self.hp.iv & 1) |
            (self.attack.iv & 1) << 1 |
            (self.defense.iv & 1) << 2 |
            (self.speed.iv & 1) << 3 |
            (self.special_attack.iv & 1) << 4 |
            (self.special_defense.iv & 1) << 5;
        let number = u16::from(bit) * 5u16 / 21u16;
        match number {
            0 => Type::Fighting,
            1 => Type::Flying,
            2 => Type::Poison,
            3 => Type::Ground,
            4 => Type::Rock,
            5 => Type::Bug,
            6 => Type::Ghost,
            7 => Type::Steel,
            8 => Type::Fire,
            9 => Type::Water,
            10 => Type::Grass,
            11 => Type::Electric,
            12 => Type::Psychic,
            13 => Type::Ice,
            14 => Type::Dragon,
            15 => Type::Dark,
            _ => panic!("Hidden Value calculation error: >15")
        }
    }

    pub fn get_hidden_power_power(&self) -> u16 {
        let bit = (self.hp.iv & 2) >> 1 |
            (self.attack.iv & 2) |
            (self.defense.iv & 2) << 1 |
            (self.speed.iv & 2) << 2 |
            (self.special_attack.iv & 2) << 3 |
            (self.special_defense.iv & 2) << 4;
        let number = ((u32::from(bit) * 40u32) / 63u32) + 30u32;
        number as u16
    }

    pub fn is_full_health(&self) -> bool {
        self.current_hp == self.hp.value
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }

    pub fn has_health(&self) -> bool {
        self.current_hp > 0
    }

    pub fn is_shiny(&self) -> bool {
        let hb = (self.personality >> 16) as u16;
        let lb = self.personality as u16;
        self.original_trainer_id ^ self.original_trainer_secret_id ^ hb ^ lb < SHININESS_CHANCE
    }

    pub fn is_holding(&self, item: &Item) -> bool {
        if let Some(i) = &self.held_item {
            i == item
        } else {
            false
        }
    }

    pub fn get_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        if let Some(m) = self.move_1 { moves.push(m.attack); }
        if let Some(m) = self.move_2 { moves.push(m.attack); }
        if let Some(m) = self.move_3 { moves.push(m.attack); }
        if let Some(m) = self.move_4 { moves.push(m.attack); }
        moves
    }

    pub fn knows_move(&self, check_attack: Move) -> bool {
        self.get_moves().iter()
            .any(|m| *m == check_attack)
    }

    pub fn get_move_slot(&self, check_attack: Move) -> Option<&MoveSlot> {
        if let Some(m) = &self.move_1 {
            if m.attack == check_attack { return Some(m); }
        }

        if let Some(m) = &self.move_2 {
            if m.attack == check_attack { return Some(m); }
        }

        if let Some(m) = &self.move_3 {
            if m.attack == check_attack { return Some(m); }
        }

        if let Some(m) = &self.move_4 {
            if m.attack == check_attack { return Some(m); }
        }

        None
    }

    pub fn get_move_slot_mut(&mut self, check_attack: Move) -> Option<&mut MoveSlot> {
        if let Some(m) = &mut self.move_1 {
            if m.attack == check_attack { return Some(m); }
        }

        if let Some(m) = &mut self.move_2 {
            if m.attack == check_attack { return Some(m); }
        }

        if let Some(m) = &mut self.move_3 {
            if m.attack == check_attack { return Some(m); }
        }

        if let Some(m) = &mut self.move_4 {
            if m.attack == check_attack { return Some(m); }
        }

        None
    }

    pub fn subtract_hp(&mut self, lose: u16) -> (u16, u16) {
        let start_hp = self.current_hp;
        let end_hp = start_hp.saturating_sub(lose);
        self.current_hp = end_hp;
        (start_hp, end_hp)
    }

    pub fn add_hp(&mut self, gain: u16) -> (u16, u16) {
        let start_hp = self.current_hp;
        let end_hp = start_hp.saturating_add(gain).clamp(0, self.hp.value);
        self.current_hp = end_hp;
        (start_hp, end_hp)
    }
}

/// Represents the three important pieces of trainer data in a Pokemon.
/// Typically all three of these are set, or all unset, so it is logical to
/// have them in a separate class.
#[derive(Debug)]
pub struct TemplateTrainer {
    pub trainer_id: u16,
    pub secret_id: u16,
    pub name: String
}

/// A template which allows for generating Pokemon with suitable defaults.
/// All fields are optional; the default is a Bulbasaur egg. Typically, a species
/// will be specified, as well as a level for non-eggs. 
/// 
/// In addition, some fields allow for more dynamic or user-friendly generation.
#[derive(Debug, Default)]
pub struct PokemonTemplate {
    pub species: SpeciesTemplate,
    pub gender: Option<Gender>,
    pub level_met: Option<u8>,
    pub nature: Option<Nature>,
    pub ability: Option<AbilitySlot>,
    pub poke_ball: Option<Pokeball>,
    pub held_item: Option<Item>,
    pub move_1: MoveTemplate,
    pub move_2: MoveTemplate,
    pub move_3: MoveTemplate,
    pub move_4: MoveTemplate,
    pub personality: Option<u32>,
    pub friendship: Option<u8>,
    pub original_trainer: Option<TemplateTrainer>,
    pub nickname: Option<String>,
    pub level: u8,
    pub markings: Markings,
    pub status: Option<PokemonStatusCondition>,
    pub pokerus: Option<PokemonPokerusStatus>,
    pub current_hp: Option<u16>,
    pub ivs: IVTemplate,
    pub evs: EVTemplate,
    pub contest: Option<PokemonContestStats>,
    pub fateful_encounter: bool,
    pub date_caught: Option<i64>,
    pub location_caught: Option<Location>,
    pub force_shiny: bool
}

/// A template which allows for specifying moves
#[derive(Debug)]
pub enum MoveTemplate {
    /// Forces this move to be present in this slot.
    HardCoded(Move),
    /// Retrieves the next move in the move pool this Pokemon can learn naturally, and use it.
    NaturalMove,
    /// Force no move in this slot.
    None
}
impl Default for MoveTemplate {
    fn default() -> Self {
        MoveTemplate::NaturalMove
    }
}

/// A template which allows for specifying IVs
#[derive(Debug)]
pub enum IVTemplate {
    /// IVs are randomly generated
    Random,
    /// Set all IVs to specific values
    HardCoded(u8, u8, u8, u8, u8, u8),
    /// Set all IVs to one specific value
    All(u8),
    /// Selects three stats at random, and assigns max IVs to those. Remainder are random.
    Rare
}
impl Default for IVTemplate {
    fn default() -> Self {
        IVTemplate::Random
    }
}

/// A template which allows for specifying EVs
#[derive(Debug)]
pub enum EVTemplate {
    /// Set all stats individually to a specific value
    HardCoded(u8, u8, u8, u8, u8, u8),
    /// Set all stats to the same value
    All(u8)
}
impl Default for EVTemplate {
    fn default() -> Self {
        EVTemplate::All(0)
    }
}

/// A template which allows for dynamic species generation
#[derive(Debug)]
pub enum SpeciesTemplate {
    /// No dynamic generation
    HardCoded(Species),
    /// Generates a random Unown, evenly distributed between all varieties
    RandomUnown
}
impl Default for SpeciesTemplate {
    fn default() -> Self {
        Self::HardCoded(Species::default())
    }
}
impl From<Species> for SpeciesTemplate {
    fn from(s: Species) -> Self {
        Self::HardCoded(s)
    }
}

/// A template which can be used to create individual Pokemon
/// Use as few or as many fields as necessary. Any unused are given
/// suitable defaults, or are randomly generated.
/// At minimum, only a species is required (an egg will be generated).
impl PokemonTemplate {
    pub fn egg<S: Into<SpeciesTemplate>>(species: S) -> PokemonTemplate {
        PokemonTemplate {
            species: species.into(),
            ..Default::default()
        }
    }

    pub fn pokemon<S: Into<SpeciesTemplate>>(species: S, level: u8) -> PokemonTemplate {
        PokemonTemplate {
            species: species.into(),
            level,
            ..Default::default()
        }
    }

    /// Force this Pokemon to be shiny.
    pub fn shiny(mut self) -> PokemonTemplate {
        self.force_shiny = true;
        self
    }

    /// Increase the odds of a Shiny being created by "rerolling" the Personality Value
    /// Rerolls Personality `rolls` times, stopping only when running out of rolls, or a shiny
    /// is generated.
    /// Masuda Method rerolls 5 times. Shiny Charm rerolls 2 times. If both are present, 7 rerolls
    /// are done.
    /// If an original trainer has not already been created, the player is used.
    /// Changing the original trainer after running this method will lose the potential shiniess.
    pub fn maybe_shiny(mut self, rolls: u8) -> Self {
        if rolls == 0 { return self; }

        let (trainer_id, secret_id) = match self.original_trainer {
            Some(TemplateTrainer { trainer_id, secret_id, ..}) => (trainer_id, secret_id),
            None => panic!()
        };

        let is_shiny = |t: &Self| {
            match &t.personality {
                None => false,
                Some(a) => {
                    let hb = (*a >> 16) as u16;
                    let lb = *a as u16;
                    trainer_id ^ secret_id ^ hb ^ lb < SHININESS_CHANCE
                }
            }
        };
        if is_shiny (&self) {
            return self;
        }

        for _ in 0..rolls {
            self.personality = rand::thread_rng().gen();
            if is_shiny (&self) {
                return self;
            }
        }
        self
    }

    pub fn moves(mut self, moves: Vec<Move>) -> Self {
        let mut iter = moves.into_iter();
        self.move_1 = match iter.next() {
            Some(m) => MoveTemplate::HardCoded(m),
            None => MoveTemplate::None,
        };
        self.move_2 = match iter.next() {
            Some(m) => MoveTemplate::HardCoded(m),
            None => MoveTemplate::None,
        };
        self.move_3 = match iter.next() {
            Some(m) => MoveTemplate::HardCoded(m),
            None => MoveTemplate::None,
        };
        self.move_4 = match iter.next() {
            Some(m) => MoveTemplate::HardCoded(m),
            None => MoveTemplate::None,
        };
        self
    }

    /// Give this Pokemon an item to hold
    pub fn holding<B: Into<Item>>(mut self, item: B) -> Self {
        self.held_item = Some(item.into());

        self
    }

    /// Force this Pokemon's gender
    pub fn gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);

        self
    }

    /// Customize this Pokemon in a builder-friendly way
    pub fn custom<F: Fn(&mut PokemonTemplate)>(mut self, func: F) -> Self {
        func(&mut self);
        self
    }
}