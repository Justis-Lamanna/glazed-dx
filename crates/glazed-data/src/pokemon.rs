use std::collections::HashSet;
use rand::{Rng, RngCore};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use crate::abilities::{Ability, PokemonAbility};
use crate::attack::Move;
use crate::constants::Species;
use crate::core::Player;
use crate::item::{Item, Pokeball};
use crate::pokemon::MoveTemplate::NaturalMove;
use crate::types::PokemonType;

/// Represents the probability of a Pokemon being male or female (or neither)
#[derive(Debug)]
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

/// Represents an Egg Group, i.e. the Compatibility of two Pokemon
#[derive(Debug)]
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
#[derive(Debug)]
pub enum PokemonEggGroup {
    None,
    One(EggGroup),
    Two(EggGroup, EggGroup)
}

/// Represents how much EXP is required to advance levels
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Stats(pub Stat,pub Stat,pub Stat,pub Stat,pub Stat,pub Stat);

/// Represents data tied to a Stat for a Pokemon species as a whole
#[derive(Debug)]
pub struct Stat {
    base_stat: u8,
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
#[derive(Debug)]
pub struct PokemonData {
    pub _type: PokemonType,
    pub ability: PokemonAbility,
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
    pub level_up_moves: &'static[(u8, Move)],
    pub egg_moves: Option<&'static[Move]>
}

/// Represents an individual member of a Pokemon species
#[derive(Debug)]
pub struct Pokemon {
    pub species: Species,
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
    pub original_trainer_name: String,
    pub nickname: Option<String>,
    pub level: u8,
    pub markings: [bool; 6],
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
    pub fateful_encounter: bool
}

/// Represents the Contest Stats and Winnings of this Pokemon
#[derive(Debug, Default)]
pub struct PokemonContestStats {
    coolness: u8,
    beauty: u8,
    cuteness: u8,
    smartness: u8,
    toughness: u8,
    feel: u8,
    cool_ribbon: [bool; 4],
    beauty_ribbon: [bool; 4],
    cute_ribbon: [bool; 4],
    smart_ribbon: [bool; 4],
    tough_ribbon: [bool; 4],
    champion_ribbon: bool,
    winning_ribbon: bool,
    victory_ribbon: bool,
    artist_ribbon: bool,
    effort_ribbon: bool
}

/// Represents the status conditions of this Pokemon
#[derive(Debug, Default)]
pub struct PokemonStatusCondition {
    pub sleep: u8,
    pub poison: bool,
    pub bad_poison: bool,
    pub burn: bool,
    pub freeze: bool,
    pub paralysis: bool
}
impl PokemonStatusCondition {
    pub fn has_status_condition(&self) -> bool {
        self.sleep > 0 || self.poison || self.bad_poison || self.burn || self.freeze || self.paralysis
    }
}

/// Represents the values tied to a given moveslot
#[derive(Debug)]
pub struct MoveSlot {
    attack: Move,
    pp: u8,
    pp_bonus: u8
}
impl From<Move> for MoveSlot {
    fn from(m: Move) -> Self {
        MoveSlot {
            attack: m,
            pp: m.data().pp,
            pp_bonus: 0
        }
    }
}

/// Represents the values tied to a given stat (HP, Atk, etc.)
#[derive(Debug)]
pub struct StatSlot {
    pub value: u16,
    iv: u8,
    ev: u8
}
impl StatSlot {
    /// Create a StatSlot for the HP of Shedinja
    /// Even with the smallest base HP, the minimum HP value is 4. Shedinja is handled as a special
    /// case, to ensure it always has an HP of 1.
    fn hp_shedinja(iv: u8, ev: u8) -> StatSlot {
        StatSlot {
            value: 1,
            iv,
            ev
        }
    }

    /// Create a StatSlot for the HP of a Pokemon
    fn hp(base: u8, level: u8, iv: u8, ev: u8) -> StatSlot {
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
    fn recalculate_hp(&mut self, base: u8, level: u8) {
        let recalc = StatSlot::hp(base, level, self.iv, self.ev);
        self.value = recalc.value;
    }

    /// Create a StatSlot for this Pokemon
    fn stat(base: u8, level: u8, iv: u8, ev: u8, boost: &NatureBoost) -> StatSlot {
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
    fn recalculate(&mut self, base: u8, level: u8, boost: &NatureBoost) {
        let recalc = StatSlot::stat(base, level, self.iv, self.ev, boost);
        self.value = recalc.value;
    }
}

/// Represents which Ability the Pokemon has
#[derive(Debug)]
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
#[derive(Debug)]
pub enum PokemonPokerusStatus {
    None,
    Infected(u8),
    Cured
}

/// Represents one of the 25 Natures of a Pokemon
#[derive(Debug)]
pub enum Nature {
    Hardy, Docile, Serious, Bashful, Quirky, //Neutrals
    Lonely, Brave, Adamant, Naughty, //Attack Up
    Bold, Relaxed, Impish, Lax, //Defense Up
    Timid, Hasty, Jolly, Naive, //SPA Up
    Modest, Mild, Quiet, Rash, //SPD up
    Calm, Gentle, Sassy, Careful //Speed up
}

/// The result of checking if a stat is boosted by this nature
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
    fn determine_moves_by_level(species_data: &PokemonData, level: u8) -> Vec<MoveSlot> {
        let mut moveset = vec![];
        for (lvl, m) in species_data.level_up_moves {
            if *lvl > level {
                break
            }
            if !moveset.contains(m) {
                moveset.push(m.clone());
                if moveset.len() > 4 {
                    moveset.remove(0);
                }
            }
        }
        assert!(moveset.len() <= 4);

        let mut movepool = vec![];
        for ms in moveset {
            movepool.push(MoveSlot {
                attack: ms,
                pp: ms.data().pp,
                pp_bonus: 0
            })
        }
        movepool
    }

    /// Recalculate the level + stats of this Pokemon
    pub fn recalculate_stats(&mut self) {
        let species_data = self.species.data();
        let level = species_data.level_rate.level_for_experience(self.experience);
        match self.species {
            Species::Shedinja => self.hp.value = 1,
            _ => self.hp.recalculate_hp(species_data.stats.0.base_stat, level)
        }
        self.attack.recalculate(species_data.stats.1.base_stat, level, &self.nature.get_attack_boost());
        self.defense.recalculate(species_data.stats.2.base_stat, level, &self.nature.get_defense_boost());
        self.special_attack.recalculate(species_data.stats.3.base_stat, level, &self.nature.get_special_attack_boost());
        self.special_defense.recalculate(species_data.stats.4.base_stat, level, &self.nature.get_special_defense_boost());
        self.speed.recalculate(species_data.stats.5.base_stat, level, &self.nature.get_speed_boost());

        if self.current_hp > self.hp.value {
            self.current_hp = self.hp.value;
        }

        self.level = level;
    }

    /// Restore this Pokemon to its max HP
    pub fn heal(&mut self) {
        self.current_hp = self.hp.value;
    }

    /// Get the actual ability of this Pokemon
    /// Rules:
    /// If pokemon has SlotOne, return the first standard ability
    /// If pokemon has SlotTwo:
    ///     If pokemon has only one standard ability, return it
    ///     If pokemon has two standard abilities, returns the second standard ability
    /// If pokemon has Hidden:
    ///     If pokemon has a hidden ability, return it
    ///     If pokemon has no hidden ability, return the first standard ability
    pub fn get_ability(&self) -> &Ability {
        let data = self.species.data();
        match self.ability {
            AbilitySlot::SlotOne => {
                match &data.ability {
                    PokemonAbility::One(a) | PokemonAbility::Two(a, _) => a
                }
            },
            AbilitySlot::SlotTwo => {
                match &data.ability {
                    PokemonAbility::One(a) | PokemonAbility::Two(_, a) => a
                }
            },
            AbilitySlot::Hidden => match &data.hidden_ability {
                None => match &data.ability {
                    PokemonAbility::One(a) | PokemonAbility::Two(a, _) => a
                },
                Some(a) => a
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct PokemonTemplate {
    pub species: Species,
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
    pub original_trainer_id: Option<u16>,
    pub original_trainer_name: Option<String>,
    pub nickname: Option<String>,
    pub level: u8,
    pub markings: [bool; 6],
    pub status: Option<PokemonStatusCondition>,
    pub pokerus: Option<PokemonPokerusStatus>,
    pub current_hp: Option<u16>,
    pub ivs: IVTemplate,
    pub evs: EVTemplate,
    pub contest: Option<PokemonContestStats>,
    pub fateful_encounter: bool
}

#[derive(Debug)]
pub enum MoveTemplate {
    HardCoded(Move),
    NaturalMove,
    None
}
impl Default for MoveTemplate {
    fn default() -> Self {
        MoveTemplate::NaturalMove
    }
}

#[derive(Debug)]
pub enum IVTemplate {
    Random,
    HardCoded(u8, u8, u8, u8, u8, u8),
    All(u8),
    Rare
}
impl Default for IVTemplate {
    fn default() -> Self {
        IVTemplate::Random
    }
}

#[derive(Debug)]
pub enum EVTemplate {
    HardCoded(u8, u8, u8, u8, u8, u8),
    All(u8)
}
impl Default for EVTemplate {
    fn default() -> Self {
        EVTemplate::All(0)
    }
}

/// A template which can be used to create individual Pokemon
/// Use as few or as many fields as necessary. Any unused are given
/// suitable defaults, or are randomly generated.
/// At minimum, only a species is required (an egg will be generated).
impl PokemonTemplate {
    pub fn egg(species: Species) -> PokemonTemplate {
        PokemonTemplate {
            species,
            ..Default::default()
        }
    }

    pub fn pokemon(species: Species, level: u8) -> PokemonTemplate {
        PokemonTemplate {
            species,
            level,
            ..Default::default()
        }
    }

    fn create_stats(ivs: IVTemplate, evs: EVTemplate) -> (StatSlot, StatSlot, StatSlot, StatSlot, StatSlot, StatSlot) {
        let ivs = match ivs {
            IVTemplate::Random => [
                rand::thread_rng().gen_range(0..=31),
                rand::thread_rng().gen_range(0..=31),
                rand::thread_rng().gen_range(0..=31),
                rand::thread_rng().gen_range(0..=31),
                rand::thread_rng().gen_range(0..=31),
                rand::thread_rng().gen_range(0..=31)
            ],
            IVTemplate::HardCoded(hp, atk, def, spa, spd, spe) => [hp, atk, def, spa, spd, spe],
            IVTemplate::All(val) => [val; 6],
            IVTemplate::Rare => {
                let mut lucky_slots = [false; 6];
                let mut counter = 0;
                while counter < 3 {
                    let slot = rand::thread_rng().gen_range(0..6);
                    if !lucky_slots[slot] {
                        lucky_slots[slot] = true;
                        counter += 1;
                    }
                }
                let mut stats = [0u8; 6];
                for (idx, slot) in stats.iter_mut().enumerate() {
                    if lucky_slots[idx] {
                        *slot = 31u8;
                    } else {
                        *slot = rand::thread_rng().gen_range(0u8..=31u8);
                    }
                }
                stats
            }
        };
        let evs = match evs {
            EVTemplate::HardCoded(a, b, c, d, e, f) => [a, b, c, d, e, f],
            EVTemplate::All(v) => [v; 6]
        };
        (
            StatSlot {value: 0, iv: ivs[0], ev: evs[0] },
            StatSlot {value: 0, iv: ivs[1], ev: evs[1] },
            StatSlot {value: 0, iv: ivs[2], ev: evs[2] },
            StatSlot {value: 0, iv: ivs[3], ev: evs[3] },
            StatSlot {value: 0, iv: ivs[4], ev: evs[4] },
            StatSlot {value: 0, iv: ivs[5], ev: evs[5] }
        )
    }
}
impl From<PokemonTemplate> for Pokemon {
    fn from(template: PokemonTemplate) -> Self {
        let data = template.species.data();
        let (hp, atk, def, spa, spd, spe)
            = PokemonTemplate::create_stats(template.ivs, template.evs);
        let mut moves = Pokemon::determine_moves_by_level(data, template.level);
        let mut moves = moves.drain(..);
        let mut p = Pokemon {
            species: template.species,
            egg: template.level == 0,
            level_met: template.level_met.unwrap_or(template.level),
            nature: template.nature.unwrap_or_else(|| rand::thread_rng().gen()),
            ability: template.ability.unwrap_or_else(|| rand::thread_rng().gen()),
            poke_ball: template.poke_ball.unwrap_or(Pokeball::PokeBall),
            held_item: template.held_item,
            move_1: match template.move_1 {
                MoveTemplate::HardCoded(m) => Some(MoveSlot::from(m)),
                MoveTemplate::NaturalMove => moves.next(),
                MoveTemplate::None => None
            },
            move_2: match template.move_2 {
                MoveTemplate::HardCoded(m) => Some(MoveSlot::from(m)),
                MoveTemplate::NaturalMove => moves.next(),
                MoveTemplate::None => None
            },
            move_3: match template.move_3 {
                MoveTemplate::HardCoded(m) => Some(MoveSlot::from(m)),
                MoveTemplate::NaturalMove => moves.next(),
                MoveTemplate::None => None
            },
            move_4: match template.move_4 {
                MoveTemplate::HardCoded(m) => Some(MoveSlot::from(m)),
                MoveTemplate::NaturalMove => moves.next(),
                MoveTemplate::None => None
            },
            experience: data.level_rate.experience_for_level(template.level),
            personality: template.personality.unwrap_or_else(|| rand::thread_rng().gen()),
            friendship: template.friendship.unwrap_or(data.base_friendship),
            original_trainer_id: template.original_trainer_id.unwrap_or_else(|| rand::thread_rng().gen()),
            original_trainer_name: template.original_trainer_name.unwrap_or(String::from("Trainer")),
            nickname: template.nickname,
            level: template.level,
            markings: template.markings,
            status: template.status.unwrap_or_else(|| PokemonStatusCondition::default()),
            pokerus: template.pokerus.unwrap_or_else(|| PokemonPokerusStatus::None),
            contest: template.contest.unwrap_or_else(|| PokemonContestStats::default()),
            fateful_encounter: template.fateful_encounter,
            current_hp: 0,
            hp,
            attack: atk,
            defense: def,
            special_attack: spa,
            special_defense: spd,
            speed: spe
        };
        p.recalculate_stats();
        p.heal();
        p
    }
}