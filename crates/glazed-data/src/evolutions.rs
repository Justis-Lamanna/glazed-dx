#![allow(non_upper_case_globals)]

use serde::Deserialize;

use crate::attack::{Move, MoveData};
use crate::contest::Condition;
use crate::core::Player;
use crate::locations::GlazedLocation;
use crate::time::{GlazedTime, TimeOfDay};
use crate::item::{EvolutionHeldItem, EvolutionStone, Incense, Item};
use crate::lookups::Lookup;
use crate::pokemon::{Gender, Pokemon};
use crate::species::Species;
use crate::types::Type;

const MIN_CONDITION_TO_EVOLVE: u8 = 170;
const MIN_FRIENDSHIP_TO_EVOLVE: u8 = 220;
const HIDDEN_ABILITY_PASS_PROBABILITY: f64 = 0.6;
const REGULAR_ABILITY_PASS_PROBABILITY: f64 = 0.8;

/// Represents evolution + breeding data for a species
/// Breeding rules:
/// 1. if `baby`, and parent is holding `baby.incense`, offspring is `baby.species`
/// 2. else, offspring is `base`
#[derive(Debug, Deserialize)]
pub struct Evolution {
    pub id: Species,
    pub base: Species,
    #[serde(default)]
    pub baby: Option<IncenseBaby>,
    #[serde(default)]
    pub paths: Option<Vec<EvolutionPath>>
}

/// Represents a baby Pokemon that can be obtained by breeding while holding an incense
#[derive(Debug, Deserialize)]
pub struct IncenseBaby {
    pub species: Species,
    pub incense: Incense
}

/// Represents one possible evolution for this species
#[derive(Debug, Deserialize)]
pub struct EvolutionPath {
    pub to: Species,
    pub trigger: EvolutionTrigger
}

/// Represents all evolution triggers accepted by the game
#[derive(Debug, Deserialize)]
pub enum EvolutionTrigger {
    /// Evolves when a zero or more conditions are met at level up
    OnLevelUp(Vec<EvolutionCondition>),
    NincadaSpawn,

    /// Evolves when an evolutionary stone is used on it
    EvolutionStone {
        stone: EvolutionStone,
        #[serde(default)]
        conditions: Option<Vec<EvolutionCondition>>
    },

    /// Evolves after the Pokemon was traded and all conditions are met
    Trading {
        #[serde(default)]
        conditions: Option<Vec<EvolutionCondition>>
    },
    /// Evolves after the Pokemon was traded, if the opposite Pokemon was the specific species
    TradingForPokemon(Species)
}

/// Represents any number of sub-conditions that must be fulfilled on reaching a certain level
#[derive(Debug, Deserialize)]
pub enum EvolutionCondition {
    /// Is the Pokemon this level (or greater)?
    Level(u8),
    /// Is the Pokemon this gender?
    Gender(Gender),
    /// Is it current this time of day?
    TimeOfDay(TimeOfDay),
    /// Does the Pokemon have a high friendship? (>= 220)
    HighFriendship,
    /// Does the Pokemon have a high condition? (>= 170)
    HighCondition(Condition),
    /// Is the Pokemon holding this held item?
    HoldingItem(EvolutionHeldItem),
    /// Does the Pokemon know this move?
    KnowsMove(Move),
    /// Does the Pokemon know a move of this type?
    KnowsMoveOfType(Type),
    /// Does the party include a Pokemon of this species?
    WithPartyPokemon(Species),
    /// Is the Pokemon in a specific location?
    AtPlace(EvolutionTriggerLocation),
    /// Does the Pokemon have a low Personality Value?
    LowPersonality,
    /// Does the Pokemon have a high Personality Value?
    HighPersonality,
    /// Is the Pokemon's attack stat greater than its defense?
    HigherAttackThanDefense,
    /// Is the Pokemon's defense stat greater than its attack?
    HigherDefenseThanAttack,
    /// Are the Pokemon's attack and defense stats identical?
    EqualAttackAndDefense
}
impl EvolutionCondition {
    /// Check if a Pokemon meets the Evolution Condition
    pub fn meets_condition(&self, pkmn: &Pokemon) -> bool {
        match self {
            EvolutionCondition::Level(l) => 
                pkmn.level >= *l,
            EvolutionCondition::Gender(g) => 
                pkmn.gender == *g,
            EvolutionCondition::TimeOfDay(d) => 
                GlazedTime::get_time_of_day() == *d,
            EvolutionCondition::HighFriendship => 
                pkmn.friendship >= MIN_FRIENDSHIP_TO_EVOLVE,
            EvolutionCondition::HighCondition(c) => 
                pkmn.contest.get_condition(c) >= MIN_CONDITION_TO_EVOLVE,
            EvolutionCondition::HoldingItem(h) => 
                pkmn.is_holding(Item::from(*h)),
            EvolutionCondition::KnowsMove(m) => 
                pkmn.knows_move(*m),
            EvolutionCondition::KnowsMoveOfType(t) => 
                pkmn.get_moves().iter()
                    .map(|m| MoveData::lookup(m))
                    .any(|md| md._type == *t),
            EvolutionCondition::WithPartyPokemon(p) => false, // To be done later
            EvolutionCondition::AtPlace(p) => match p {
                EvolutionTriggerLocation::MossRock => GlazedLocation::is_moss_rock_nearby(),
                EvolutionTriggerLocation::IceRock => GlazedLocation::is_ice_rock_nearby(),
                EvolutionTriggerLocation::MagneticField => GlazedLocation::is_magnetic_field_nearby(),
            },
            EvolutionCondition::LowPersonality => 
                (pkmn.personality >> 16) & 10 < 5,
            EvolutionCondition::HighPersonality => 
                (pkmn.personality >> 16) & 10 >= 5,
            EvolutionCondition::HigherAttackThanDefense => 
                pkmn.attack.value > pkmn.defense.value,
            EvolutionCondition::HigherDefenseThanAttack => 
                pkmn.attack.value < pkmn.defense.value,
            EvolutionCondition::EqualAttackAndDefense => 
                pkmn.attack.value == pkmn.defense.value
        }
    }
}

/// Represents a type of location that can trigger Evolution
#[derive(Debug, Deserialize)]
pub enum EvolutionTriggerLocation {
    MossRock,
    IceRock,
    MagneticField
}

pub mod evolution {
    use crate::{species::Species, pokemon::Pokemon, lookups::Lookup, core::Player};

    use super::{Evolution, EvolutionTrigger};

    pub fn level_up(p: &Pokemon) -> Vec<Species> {
        let evolutions = Evolution::lookup(&p.species);
        match &evolutions.paths {
            None => vec![],
            Some(paths) => {
                let mut species = Vec::new();
                for path in paths {
                    if let EvolutionTrigger::OnLevelUp(conditions) = &path.trigger {
                        if conditions.iter().all(|c| c.meets_condition(p)) {
                            species.push(path.to);
                        }
                    } else if let EvolutionTrigger::NincadaSpawn = &path.trigger {
                        if p.level == 20 {
                            species.push(path.to);
                        }
                    }
                }
                species
            }
        }
    }
}

pub mod breeding {
    use rand::Rng;

    use crate::{pokemon::{Pokemon, SpeciesData, EggGroup, Gender, PokemonTemplate, Nature, AbilitySlot, IVTemplate}, species::Species, lookups::Lookup, attack::Move, item::{Item, Pokeball}};
    use super::{Evolution, HIDDEN_ABILITY_PASS_PROBABILITY, REGULAR_ABILITY_PASS_PROBABILITY};

    /// Two Pokemon's compatibility, or in other words, their likelihood of breeding
    pub enum Compatibility {
        /// Represents a high likelihood of producing an egg
        Great,
        /// Represents an average likelihood of producing an egg
        Ok,
        /// Represents a low likelihood of producing an egg
        Poor,
        /// Represents no chance of producing an egg
        None
    }

    /// Get how compatible two Pokemon are
    pub fn get_compatibility(p1: &Pokemon, p2: &Pokemon) -> Compatibility {
        if !are_egg_group_compatible(p1.species, p2.species) {
            return Compatibility::None;
        }
        if determine_offspring_species(p1, p2).is_none() {
            return Compatibility::None;
        }

        use crate::species::SpeciesDiscriminants;
        let same_species = SpeciesDiscriminants::from(p1.species) == SpeciesDiscriminants::from(p2.species);
        let same_trainer = p1.original_trainer_id == p2.original_trainer_id
            && p1.original_trainer_name == p2.original_trainer_name
            && p1.original_trainer_secret_id == p2.original_trainer_secret_id;

        if (same_species && same_trainer) || (!same_species && !same_trainer) { Compatibility::Ok }
        else if same_species && !same_trainer { Compatibility::Great }
        else { Compatibility::Poor }
    }

    /// Create the offspring of two Pokemon, following the regular breeding rules.
    /// Returns None if the Pokemon are incompatible for some reason,
    /// or Some with the new Pokemon Egg.
    pub fn create_offspring(p1: &Pokemon, p2: &Pokemon) -> Option<Pokemon> {
        if are_egg_group_compatible(p1.species, p2.species) {
            let baby_species = determine_offspring_species(p1, p2)?;
            let baby_moves = determine_offspring_moves(p1, p2, baby_species);
            let fake_masuda = 0; // To do: Pokemon from different languages reroll 5 shiny.
            let fake_shinycharm = 0; // To do: If Player has a Shiny Charm, reroll 2 shiny.

            Some(
                PokemonTemplate::egg(baby_species)
                .moves(baby_moves)
                .maybe_shiny(fake_masuda + fake_shinycharm)
                .custom(|t| {
                    t.nature = Some(determine_offspring_nature(p1, p2));
                    t.ability = Some(determine_offspring_ability(p1, p2));
                    t.poke_ball = Some(determine_offspring_pokeball(p1, p2));
                    t.ivs = determine_offspring_ivs(p1, p2);
                })
                .into()
            )
        } else {
            None
        }
    }

    fn are_egg_group_compatible(p1: Species, p2: Species) -> bool {
        let p1 = SpeciesData::lookup(&p1).egg_group.as_set();
        let p2 = SpeciesData::lookup(&p2).egg_group.as_set();

        if p1.contains(&EggGroup::Ditto) && p2.contains(&EggGroup::Ditto) {
            // Two Ditto cannot breed
            false
        } else if p1.contains(&EggGroup::Ditto) || p2.contains(&EggGroup::Ditto) {
            // One Ditto and one non-ditto can breed
            true
        } else {
            !p1.is_disjoint(&p2)
        }
    }

    fn determine_offspring_species(p1: &Pokemon, p2: &Pokemon) -> Option<Species> {
        let species_determiner = match (p1.gender, p2.gender) {
            (Gender::Male, Gender::Female) => p2,
            (Gender::Female, Gender::Male) => p1,
            _ => {
                if p1.species == Species::Ditto {
                    p2
                } else if p2.species == Species::Ditto {
                    p1
                } else {
                    return None;
                }
            }
        };

        let breed_data = Evolution::lookup(&species_determiner.species);
        let baby_species = match &breed_data.baby {
            None => {
                // Special cases: NidoranF/NidoranM, and Volbeat/Illumise have a 50/50 chance as to which baby selected
                if species_determiner.species == Species::NidoranF || species_determiner.species == Species::NidoranM {
                    if rand::thread_rng().gen_bool(0.5) { Species::NidoranF } else { Species::NidoranM }
                } else if species_determiner.species == Species::Volbeat || species_determiner.species == Species::Illumise {
                    if rand::thread_rng().gen_bool(0.5) { Species::Volbeat } else { Species::Illumise }
                } else {
                    breed_data.base
                }
            },
            Some(b) => {
                if p1.is_holding(b.incense.into()) || p2.is_holding(b.incense.into()) {
                    b.species
                } else {
                    breed_data.base
                }
            }
        };

        Some(baby_species)
    }

    // struct WeightedMove(u8, Move);

    fn determine_offspring_moves(p1: &Pokemon, p2: &Pokemon, baby: Species) -> Vec<Move> {
        let mut heap = Vec::new();
        let baby_data = SpeciesData::lookup(&baby);
        // 1. All Moves Baby learns at Level 1
        match baby_data.level_up_moves.get(&1) {
            Some(moves) => {
                for m in moves {
                    heap.push(*m);
                }
            },
            None => {},
        }
        // 2. Any moves that both parents know that the baby can learn via Level Up
        let baby_moves = baby_data.get_all_knowable_moves();
        for baby_move in baby_moves {
            if p1.knows_move(baby_move) && p2.knows_move(baby_move) {
                heap.push(baby_move)
            }
        }
        // 3. Any TM moves the father (or in the case of Genderless + Ditto, the non-ditto parent) knows.
        // We can comfortably assume that the pairing of p1 and p2 is legal, so the only options are
        // Male + Female
        // Any + Ditto
        // INCOMPLETE: We don't yet have TM/HM data for Pokemon yet.
        let father = if p1.gender == Gender::Male || p2.species == Species::Ditto {
            p1
        } else {
            p2
        };

        use strum::IntoEnumIterator;
        for tm in crate::item::TM::iter() {
            let m = tm.get_move();
            if father.knows_move(*m) /*&& baby_data.can_learn_tm(tm)*/{
                heap.push(*m)
            }
        }
        for hm in crate::item::HM::iter() {
            let m = hm.get_move();
            if father.knows_move(*m) /*&& baby_data.can_learn_hm(hm)*/{
                heap.push(*m)
            }
        }
        // 4. If either the father or the mother knows an egg move the baby has
        match &baby_data.egg_moves {
            None => {},
            Some(egg_moves) => {
                for egg in egg_moves {
                    if p1.knows_move(*egg) || p2.knows_move(*egg) {
                        heap.push(*egg)
                    }
                }
            }
        };
        // 5. If Baby is Pichu, and either parent has a Light Ball, the Pichu gets Volt Tackle
        if baby == Species::Pichu && 
            (p1.is_holding(Item::LightBall) || p2.is_holding(Item::LightBall)) {
                heap.push(Move::VoltTackle)
        }

        // Retrieve the top 4 moves from the pool. 
        let mut final_pool = Vec::new();
        while let Some(m) = heap.pop() {
            if !final_pool.contains(&m) {
                final_pool.push(m)
            }
        }
        final_pool
    }

    fn determine_offspring_nature(p1: &Pokemon, p2: &Pokemon) -> Nature {
        let p1_everstone = p1.is_holding(Item::Everstone);
        let p2_everstone = p2.is_holding(Item::Everstone);

        if p1_everstone && p2_everstone {
            if rand::thread_rng().gen_bool(0.5) {
                p1.nature
            } else {
                p2.nature
            }
        } else if p1_everstone { p1.nature }
        else if p2_everstone { p2.nature }
        else { rand::thread_rng().gen() }
    }

    fn determine_offspring_ability(p1: &Pokemon, p2: &Pokemon) -> AbilitySlot {
        // The female (or non-ditto, if Male/Ditto pair) determines the ability
        let ability_determiner = if p1.gender == Gender::Female || p2.species == Species::Ditto {
            p1
        } else {
            p2
        };

        let pass_probability = match ability_determiner.ability {
            AbilitySlot::Hidden => HIDDEN_ABILITY_PASS_PROBABILITY,
            _ => REGULAR_ABILITY_PASS_PROBABILITY
        };

        if rand::thread_rng().gen_bool(pass_probability) {
            ability_determiner.ability
        } else if rand::thread_rng().gen_bool(0.5) {
            AbilitySlot::SlotOne
        } else {
            AbilitySlot::SlotTwo
        }
    }

    fn determine_offspring_pokeball(p1: &Pokemon, p2: &Pokemon) -> Pokeball {
        // The female determines the Pokeball exclusively. Male/Ditto pairs results in Pokeball.
        let pokeball_determiner = if p1.gender == Gender::Female {
            p1
        } else if p2.gender == Gender::Female {
            p2
        } else {
            return Pokeball::PokeBall;
        };

        match pokeball_determiner.poke_ball {
            // Master and Cherish Balls cannot be passed :(
            Pokeball::MasterBall | Pokeball::CherishBall => Pokeball::PokeBall,
            a => a
        }
    }

    fn determine_offspring_ivs(p1: &Pokemon, p2: &Pokemon) -> IVTemplate {
        let random_parent = || if rand::thread_rng().gen_bool(0.5) { p1 } else { p2 };
        let random_iv = || rand::thread_rng().gen_range(0u8..=31u8);

        let mut inherit_from_parents_ctr = if p1.is_holding(Item::DestinyKnot) || p2.is_holding(Item::DestinyKnot) {
            5
        } else {
            3
        };

        let mut hp = None;
        let mut atk = None;
        let mut def = None;
        let mut spa = None;
        let mut spd = None;
        let mut  spe = None;

        while inherit_from_parents_ctr > 0 {
            match rand::thread_rng().gen_range(0..6) {
                0 if hp.is_none() => {
                    hp = Some(random_parent().hp.iv);
                    inherit_from_parents_ctr -= 1;
                },
                1 if atk.is_none() => {
                    atk = Some(random_parent().attack.iv);
                    inherit_from_parents_ctr -= 1;
                },
                2 if def.is_none() => {
                    def = Some(random_parent().defense.iv);
                    inherit_from_parents_ctr -= 1;
                },
                3 if spa.is_none() => {
                    spa = Some(random_parent().special_attack.iv);
                    inherit_from_parents_ctr -= 1;
                },
                4 if spd.is_none() => {
                    spd = Some(random_parent().special_defense.iv);
                    inherit_from_parents_ctr -= 1;
                },
                5 if spe.is_none() => {
                    spe = Some(random_parent().speed.iv);
                    inherit_from_parents_ctr -= 1;
                },
                _ => {}
            }
        }

        IVTemplate::HardCoded(
            hp.unwrap_or_else(random_iv),
            atk.unwrap_or_else(random_iv),
            def.unwrap_or_else(random_iv),
            spa.unwrap_or_else(random_iv),
            spd.unwrap_or_else(random_iv),
            spe.unwrap_or_else(random_iv)
        )
    }
}