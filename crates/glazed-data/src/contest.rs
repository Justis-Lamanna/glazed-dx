use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::Add;
use rand::Rng;
use crate::item::Berry;
use crate::pokemon::Nature;

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    Cool,
    Beautiful,
    Cute,
    Clever,
    Tough
}
impl From<BerryFlavor> for Condition {
    fn from(b: BerryFlavor) -> Self {
        match b {
            BerryFlavor::Spicy => Condition::Cool,
            BerryFlavor::Dry => Condition::Beautiful,
            BerryFlavor::Sweet => Condition::Cute,
            BerryFlavor::Bitter => Condition::Clever,
            BerryFlavor::Sour => Condition::Tough
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BerryFlavor {
    Spicy,
    Dry,
    Sweet,
    Bitter,
    Sour
}

#[derive(Debug, Copy, Clone)]
pub enum ConditionRaiseStatus {
    Easy,
    Neutral,
    Hard
}
impl ConditionRaiseStatus {
    /// Given a Pokemon's nature, how hard is it to raise the condition?
    pub fn for_nature_and_condition(nature: &Nature, condition: &Condition) -> ConditionRaiseStatus {
        match condition {
            Condition::Cool => match nature {
                Nature::Lonely | Nature::Brave | Nature::Adamant | Nature::Naughty => ConditionRaiseStatus::Easy,
                Nature::Bold | Nature::Timid | Nature::Modest | Nature::Calm => ConditionRaiseStatus::Hard,
                _ => ConditionRaiseStatus::Neutral
            }
            Condition::Beautiful => match nature {
                Nature::Modest | Nature::Mild | Nature::Quiet | Nature::Rash => ConditionRaiseStatus::Easy,
                Nature::Adamant | Nature::Impish | Nature::Jolly | Nature::Careful => ConditionRaiseStatus::Hard,
                _ => ConditionRaiseStatus::Neutral
            }
            Condition::Cute => match nature {
                Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive => ConditionRaiseStatus::Easy,
                Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy => ConditionRaiseStatus::Hard,
                _ => ConditionRaiseStatus::Neutral
            }
            Condition::Clever => match nature {
                Nature::Calm | Nature::Gentle | Nature::Sassy | Nature::Careful => ConditionRaiseStatus::Easy,
                Nature::Naughty | Nature::Lax | Nature::Naive | Nature::Rash => ConditionRaiseStatus::Hard,
                _ => ConditionRaiseStatus::Neutral
            }
            Condition::Tough => match nature {
                Nature::Bold | Nature::Relaxed | Nature::Impish | Nature::Lax => ConditionRaiseStatus::Easy,
                Nature::Lonely | Nature::Hasty | Nature::Mild | Nature::Gentle => ConditionRaiseStatus::Hard,
                _ => ConditionRaiseStatus::Neutral
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PokeblockColor {
    Black, Red, Blue, Pink, Green, Yellow, Gold,
    Purple, Indigo, Brown, LightBlue, Olive, Gray, White
}

#[derive(Debug, Clone)]
pub struct Pokeblock {
    pub spicy: u8,
    pub dry: u8,
    pub sweet: u8,
    pub bitter: u8,
    pub sour: u8,
    /// Represents how "full" the Pokeblock makes a Pokemon. Added directly to a Pokemon's sheen when
    /// eaten. When sheen is at 255, no more Pokeblocks can be eaten.
    pub feel: u8,
    /// The maximum flavor value.
    pub level: u8,
    /// Represents the condition(s) that are raised.
    /// Black represents the weakest Pokeblock (All flavors are zero, or two of the same berries used)
    /// Red/Blue/Pink/Green/Yellow indicate one flavor of moderate power (specific color depends on the flavor)
    /// Purple/Indigo/Brown/Light Blue/Olive indicate two flavors of moderate power (specific color depends on the best flavor)
    /// Gold indicates one or two flavors of high power
    /// Gray indicates three flavors of any power
    /// White indicates four flavors of any power
    pub color: PokeblockColor
}
impl Pokeblock {
    fn check_black_pokeblock(berries: &Vec<Berry>) -> bool {
        let mut seen = HashSet::new();
        for berry in berries {
            if !seen.insert(berry) {
                return false
            }
        }
        true
    }

    fn create_black_pokeblock() -> Pokeblock {
        let mut block = Pokeblock {
            spicy: 0,
            dry: 0,
            sweet: 0,
            bitter: 0,
            sour: 0,
            feel: 0,
            level: 2,
            color: PokeblockColor::Black
        };
        let mut counter = 0;
        while counter < 3 {
            match rand::thread_rng().gen_range(0..5) {
                0 if block.spicy == 0 => { counter += 1; block.spicy = 2; }
                1 if block.dry == 0 => { counter += 1; block.dry = 2; }
                2 if block.sweet == 0 => { counter += 1; block.sweet = 2; }
                3 if block.bitter == 0 => { counter += 1; block.bitter = 2; }
                4 if block.sour == 0 => { counter += 1; block.sour = 2; }
                _ => {}
            }
        }
        block
    }

    /// Create a Pokeblock from Berry Blending
    pub fn blend(berries: Vec<Berry>, max_rpm: f64) -> Pokeblock {
        if Self::check_black_pokeblock(&berries) {
            return Self::create_black_pokeblock();
        }

        let mux = (max_rpm / 333f64) + 1f64;

        let data = berries
            .iter()
            .map(|item| BerryPokeblockData::from(*item))
            .collect::<Vec<BerryPokeblockData>>();

        let feel = data.iter()
            .map(|d| usize::from(d.smoothness))
            .sum::<usize>();
        let feel = ((feel / berries.len()) - berries.len()) as u8;

        // 1. Sum all flavors
        let (spicy, dry, sweet, bitter, sour) = data.iter()
            .map(|data| {
                let BerryPokeblockData { spicy, dry, sweet, bitter, sour, .. } = data;
                (*spicy, *dry, *sweet, *bitter, *sour)
            })
            .reduce(|a, b| {
                (a.0+b.0, a.1+b.1, a.2+b.2, a.3+b.3, a.4+b.4)
            })
            .unwrap_or((0, 0, 0, 0, 0));

        // 2. Multiply by 10,
        // subtract 1 to each flavor for every negative flavor,
        // set all negatives to 0,
        // multiply by mux.
        let mut negative_ctr = 0;
        if spicy < 0 { negative_ctr += 1}
        if dry < 0 { negative_ctr += 1}
        if sweet < 0 { negative_ctr += 1}
        if bitter < 0 { negative_ctr += 1}
        if sour < 0 { negative_ctr += 1}

        let (spicy, dry, sweet, bitter, sour) = (
            Self::normalize(spicy, negative_ctr, mux),
            Self::normalize(dry, negative_ctr, mux),
            Self::normalize(sweet, negative_ctr, mux),
            Self::normalize(bitter, negative_ctr, mux),
            Self::normalize(sour, negative_ctr, mux)
        );

        // Calculate Color and Level
        let (color, level) = Self::determine_color_and_level(spicy, dry, sweet, bitter, sour);

        if let PokeblockColor::Black = color {
            Self::create_black_pokeblock()
        } else {
            Pokeblock {
                spicy,
                dry,
                sweet,
                bitter,
                sour,
                level,
                feel,
                color
            }
        }
    }

    fn determine_color_and_level(spicy: u8, dry: u8, sweet: u8, bitter: u8, sour: u8) -> (PokeblockColor, u8) {
        // Color depends on the # of flavors, and the highest flavor.
        // Level == highest_strength_value
        let mut flavor_counter = 0;
        let mut highest_strength = None;
        let mut highest_strength_value = 0;

        if spicy > 0 {
            flavor_counter += 1;
            if spicy > highest_strength_value {
                highest_strength_value = spicy;
                highest_strength = Some(BerryFlavor::Spicy);
            }
        }

        if dry > 0 {
            flavor_counter += 1;
            if dry > highest_strength_value {
                highest_strength_value = dry;
                highest_strength = Some(BerryFlavor::Dry);
            }
        }

        if sweet > 0 {
            flavor_counter += 1;
            if sweet > highest_strength_value {
                highest_strength_value = sweet;
                highest_strength = Some(BerryFlavor::Sweet);
            }
        }

        if bitter > 0 {
            flavor_counter += 1;
            if bitter > highest_strength_value {
                highest_strength_value = bitter;
                highest_strength = Some(BerryFlavor::Bitter);
            }
        }

        if sour > 0 {
            flavor_counter += 1;
            if sour > highest_strength_value {
                highest_strength_value = sour;
                highest_strength = Some(BerryFlavor::Sour);
            }
        }

        let color = match flavor_counter {
            0 => PokeblockColor::Black,
            1 => {
                if highest_strength_value <= 50 {
                    match highest_strength.unwrap() {
                        BerryFlavor::Spicy => PokeblockColor::Red,
                        BerryFlavor::Dry => PokeblockColor::Blue,
                        BerryFlavor::Sweet => PokeblockColor::Pink,
                        BerryFlavor::Bitter => PokeblockColor::Green,
                        BerryFlavor::Sour => PokeblockColor::Yellow
                    }
                } else {
                    PokeblockColor::Gold
                }
            },
            2 => {
                if highest_strength_value <= 50 {
                    match highest_strength.unwrap() {
                        BerryFlavor::Spicy => PokeblockColor::Purple,
                        BerryFlavor::Dry => PokeblockColor::Indigo,
                        BerryFlavor::Sweet => PokeblockColor::Brown,
                        BerryFlavor::Bitter => PokeblockColor::LightBlue,
                        BerryFlavor::Sour => PokeblockColor::Olive
                    }
                } else {
                    PokeblockColor::Gold
                }
            },
            3 => PokeblockColor::Gray,
            _ => PokeblockColor::White
        };
        return (color, highest_strength_value)
    }

    fn normalize(value: i8, counter: i8, mux: f64) -> u8 {
        let value = (value * 10) - counter;
        if value < 0 {
            0
        } else {
            (mux * f64::from(value)) as u8
        }
    }
}

#[derive(Debug, Clone)]
pub struct BerryPokeblockData {
    pub spicy: i8,
    pub dry: i8,
    pub sweet: i8,
    pub bitter: i8,
    pub sour: i8,
    pub smoothness: u8
}

impl From<Berry> for BerryPokeblockData {
    fn from(_: Berry) -> Self {
        BerryPokeblockData {
            spicy: -1,
            dry: 0,
            sweet: 1,
            bitter: 0,
            sour: -1,
            smoothness: 30
        }
    }
}


