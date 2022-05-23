use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::item::Berry;
use crate::pokemon::{Nature, Pokemon, PokemonContestStats};

#[derive(Debug, Copy, Clone, Deserialize)]
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
impl PokemonContestStats {
    pub fn get_condition(&self, condition: &Condition) -> u8 {
        match condition {
            Condition::Cool => self.coolness,
            Condition::Beautiful => self.beauty,
            Condition::Cute => self.cuteness,
            Condition::Clever => self.smartness,
            Condition::Tough => self.toughness
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
pub enum FlavorPreference {
    Liked,
    Neutral,
    Disliked
}
impl FlavorPreference {
    /// Given a Pokemon's nature, how hard is it to raise the condition?
    pub fn for_nature_and_condition(nature: &Nature, condition: &Condition) -> FlavorPreference {
        match condition {
            Condition::Cool => match nature {
                Nature::Lonely | Nature::Brave | Nature::Adamant | Nature::Naughty => FlavorPreference::Liked,
                Nature::Bold | Nature::Timid | Nature::Modest | Nature::Calm => FlavorPreference::Disliked,
                _ => FlavorPreference::Neutral
            }
            Condition::Beautiful => match nature {
                Nature::Modest | Nature::Mild | Nature::Quiet | Nature::Rash => FlavorPreference::Liked,
                Nature::Adamant | Nature::Impish | Nature::Jolly | Nature::Careful => FlavorPreference::Disliked,
                _ => FlavorPreference::Neutral
            }
            Condition::Cute => match nature {
                Nature::Timid | Nature::Hasty | Nature::Jolly | Nature::Naive => FlavorPreference::Liked,
                Nature::Brave | Nature::Relaxed | Nature::Quiet | Nature::Sassy => FlavorPreference::Disliked,
                _ => FlavorPreference::Neutral
            }
            Condition::Clever => match nature {
                Nature::Calm | Nature::Gentle | Nature::Sassy | Nature::Careful => FlavorPreference::Liked,
                Nature::Naughty | Nature::Lax | Nature::Naive | Nature::Rash => FlavorPreference::Disliked,
                _ => FlavorPreference::Neutral
            }
            Condition::Tough => match nature {
                Nature::Bold | Nature::Relaxed | Nature::Impish | Nature::Lax => FlavorPreference::Liked,
                Nature::Lonely | Nature::Hasty | Nature::Mild | Nature::Gentle => FlavorPreference::Disliked,
                _ => FlavorPreference::Neutral
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
    /// Generate a black Pokeblock
    pub fn create_black_pokeblock() -> Pokeblock {
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

    /// Determine how happy a Pokemon of a given Nature will be while eating this Pokeblock.
    /// Returns LIKED if, for the Nature, the Liked flavor is greater than the Disliked flavor.
    /// Likewise, returns DISLIKED if, for the nature, the Disliked flavor is greater than the Liked flavor.
    /// Returns NEUTRAL if the nature has no flavor preferences.
    pub fn get_happiness_for_nature(&self, nature: &Nature) -> FlavorPreference {
        // Determine how happy the Pokemon is to eat this.
        // Algorithm: Sort increases, starting with highest. Find first Condition in the list that is non-neutral.
        const CONDITIONS: [Condition; 5] = [Condition::Cool, Condition::Beautiful, Condition::Cute, Condition::Clever, Condition::Tough];
        let increases: [u8; 5] = [self.spicy, self.dry, self.sweet, self.bitter, self.sour];


        let mut pairs = CONDITIONS.iter()
            .zip(increases.iter())
            .collect::<Vec<(&Condition, &u8)>>();
        pairs.sort_by_key(|(_, value)| **value);
        pairs.reverse();
        for (condition, _) in pairs {
            match FlavorPreference::for_nature_and_condition(nature, condition) {
                FlavorPreference::Liked => return FlavorPreference::Liked,
                FlavorPreference::Neutral => {}
                FlavorPreference::Disliked => return FlavorPreference::Disliked
            }
        }
        FlavorPreference::Neutral
    }
}
impl Pokemon {
    /// Check if this Pokemon can eat a Pokeblock
    pub fn can_eat_pokeblock(&self) -> bool {
        self.contest.feel == u8::MAX
    }

    /// Feed this Pokemon a Pokeblock
    /// Returns: How happy this Pokemon was to eat the Pokeblock, based on their nature and the
    /// Pokeblock's flavors.
    pub fn feed_pokeblock(&mut self, block: Pokeblock) -> FlavorPreference {
        let Pokeblock { spicy, dry, sweet, bitter, sour, feel, .. } = block;
        self.contest.feel = self.contest.feel.saturating_add(feel);
        self.contest.coolness = self.contest.coolness.saturating_add(self.get_increase_value(Condition::Cool, spicy));
        self.contest.beauty = self.contest.beauty.saturating_add(self.get_increase_value(Condition::Beautiful, dry));
        self.contest.cuteness = self.contest.cuteness.saturating_add(self.get_increase_value(Condition::Cute, sweet));
        self.contest.smartness = self.contest.smartness.saturating_add(self.get_increase_value(Condition::Clever, bitter));
        self.contest.toughness = self.contest.toughness.saturating_add(self.get_increase_value(Condition::Tough, sour));

        block.get_happiness_for_nature(&self.nature)
    }

    fn get_increase_value(&self, condition: Condition, value: u8) -> u8 {
        if value == 0 { return 0; }
        match FlavorPreference::for_nature_and_condition(&self.nature, &condition) {
            FlavorPreference::Liked => (f64::from(value) * 1.1f64).round() as u8,
            FlavorPreference::Neutral => value,
            FlavorPreference::Disliked => (f64::from(value) * 0.9f64).round() as u8,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BerryPokeblockData {
    pub id: Berry,
    pub spicy: i8,
    pub dry: i8,
    pub sweet: i8,
    pub bitter: i8,
    pub sour: i8,
    pub smoothness: u8
}


