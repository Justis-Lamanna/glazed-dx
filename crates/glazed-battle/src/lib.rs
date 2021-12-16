mod core;
pub mod single;
pub mod double;
pub mod tag;
mod effects;

use std::option::Option::Some;
use either::Either;
use rand::Rng;
use glazed_data::abilities::{Ability, PokemonAbility};
use glazed_data::attack::{Accuracy, BattleStat, DamageType, Effect, Move, MoveData, NonVolatileBattleAilment, Power, StatChangeTarget, Target};
use glazed_data::constants::Species;
use glazed_data::item::{Berry, EvolutionHeldItem, Incense, Item};
use glazed_data::pokemon::{AbilitySlot, MoveSlot, Pokemon, StatSlot};
use glazed_data::types::{Effectiveness, PokemonType, Type};

/// Represents one side of a battlefield
#[derive(Default, Debug)]
pub struct Side {
    hazard: Option<EntryHazard>,
    tailwind: u8,
    aurora_veil: u8,
    light_screen: u8,
    reflect: u8
}

/// Represents the entire battlefield
#[derive(Default, Debug)]
pub struct Field {
    weather: Option<Weather>,
    gravity: u8,
    magic_room: u8
}
impl Field {
    /// Return if harsh sunlight is present on the field
    pub fn is_sunny(&self) -> bool {
        match self.weather {
            Some(Weather::Sun(_)) => true,
            _ => false
        }
    }

    /// Return if it is raining on the field
    pub fn is_rain(&self) -> bool {
        match self.weather {
            Some(Weather::Rain(_)) => true,
            _ => false
        }
    }

    /// Return if there is a sandstorm on the field
    pub fn is_sandstorm(&self) -> bool {
        match self.weather {
            Some(Weather::Sandstorm(_)) => true,
            _ => false
        }
    }

    /// Return if there is hail on the field
    pub fn is_hail(&self) -> bool {
        match self.weather {
            Some(Weather::Hail(_)) => true,
            _ => false
        }
    }

    pub fn is_foggy(&self) -> bool {
        match self.weather {
            Some(Weather::Fog) => true,
            _ => false
        }
    }
}

/// A group of between 1 and 6 Pokemon, which a trainer owns
#[derive(Debug)]
pub struct Party {
    number_of_members: usize,
    members: [Option<Pokemon>; 6]
}
impl Party {
    pub fn create_one<T>(one: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 1,
            members: [Some(one.into()), None, None, None, None, None]
        }
    }

    pub fn create_two<T>(one: T, two: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 2,
            members: [Some(one.into()), Some(two.into()), None, None, None, None]
        }
    }

    pub fn create_three<T>(one: T, two: T, three: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 3,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), None, None, None]
        }
    }

    pub fn create_four<T>(one: T, two: T, three: T, four: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 4,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), None, None]
        }
    }

    pub fn create_five<T>(one: T, two: T, three: T, four: T, five: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 5,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), Some(five.into()), None]
        }
    }

    pub fn create_six<T>(one: T, two: T, three: T, four: T, five: T, six: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 6,
            members: [Some(one.into()), Some(two.into()), Some(three.into()), Some(four.into()), Some(five.into()), Some(six.into())]
        }
    }

    pub fn len(&self) -> usize {
        self.number_of_members
    }

    pub fn any_match<F>(&self, func: F) -> bool
        where F: Fn(&Pokemon) -> bool
    {
        self.members.iter().any(|slot| {
            match slot {
                Some(p) => func(p),
                None => false
            }
        })
    }
}

pub trait BattleTypeTrait {
    fn get_pokemon_by_id(&self, id: &Battler) -> Option<&Pokemon>;
    fn get_mut_pokemon_by_id(&mut self, id: &Battler) -> Option<&mut Pokemon>;
    fn get_battle_data(&self, id: &Battler) -> &BattleData;
    fn get_mut_battle_data(&mut self, id: &Battler) -> &mut BattleData;
    fn get_side(&self) -> &Side;
}

/// Represents the current battlefield
#[derive(Debug)]
pub struct Battlefield<T> where T: BattleTypeTrait {
    user: T,
    opponent: T,
    field: Field,
    turn_record: Vec<Turn>
}
impl <T> Battlefield<T> where T: BattleTypeTrait {
    fn get_pokemon_by_id(&self, id: &Battler) -> Option<&Pokemon> {
        if id.0 { self.user.get_pokemon_by_id(id) } else { self.opponent.get_pokemon_by_id(id) }
    }
    fn get_mut_pokemon_by_id(&mut self, id: &Battler) -> Option<&mut Pokemon>{
        if id.0 { self.user.get_mut_pokemon_by_id(id) } else { self.opponent.get_mut_pokemon_by_id(id) }
    }
    fn get_battle_data(&self, id: &Battler) -> &BattleData{
        if id.0 { self.user.get_battle_data(id) } else { self.opponent.get_battle_data(id) }
    }
    fn get_mut_battle_data(&mut self, id: &Battler) -> &mut BattleData{
        if id.0 { self.user.get_mut_battle_data(id) } else { self.opponent.get_mut_battle_data(id) }
    }
    fn get_side(&self, id: &Battler) -> &Side{
        if id.0 { self.user.get_side() } else { self.opponent.get_side() }
    }
    /// Pushes this Turn to the Turn Record, to signal it is complete and permanent.
    /// Intentionally eats the passed-in turn...it belongs to the battlefield now
    pub fn finish_turn(&mut self, turn: Turn) {
        self.turn_record.push(turn);
    }
}

#[derive(Default, Debug)]
/// Really, anything that needs to be tracked during the course of the battle
/// When the pokemon is switched out, everything here is reset to defaults
pub struct BattleData {
    /// The number of turns this Pokemon has been in battle
    turn_count: u8,
    /// If true, the pokemon has acted this turn
    has_acted_this_turn: bool,
    /// If true, the pokemon managed to land a hit
    has_landed_attack_this_turn: bool,
    /// The last move that this pokemon used
    last_move_used: Option<Move>,
    /// The number of times the last move was used
    last_move_used_counter: u8,
    /// If present, contains the amount of damage this Pokemon encountered last
    last_damager: Option<(Battler, Move, u16)>,

    attack_stage: i8,
    defense_stage: i8,
    special_attack_stage: i8,
    special_defense_stage: i8,
    speed_stage: i8,
    accuracy_stage: i8,
    evasion_stage: i8,

    /// Turns remaining bound (0 == not bound)
    bound: u8,
    /// If true, this Pokemon can't run away or switch out
    cant_flee: bool,
    /// If true, this Pokemon loses 25% HP at the end of the turn
    cursed: bool,
    /// If true, this Pokemon falls asleep at turn end
    drowsy: bool,
    /// Turns remaining where this Pokemon can't use items (0 == no embargo)
    embargo: u8,
    /// Turns remaining where this Pokemon has to use the last_used_move (0 == no encore)
    encore: u8,
    /// Turns remaining where this Pokemon is confused (0 == no confusion)
    confused: u8,
    /// If true, this Pokemon is infatuated
    infatuated: bool,
    /// If true, this Pokemon will flinch instead of attack this turn
    flinch: bool,
    /// Turns remaining where this Pokemon is unable to heal
    unable_to_heal: u8,
    /// If true, this Pokemon was identified and has evasion checks revoked
    identified: bool,
    /// Turns left before Perish Song KOs this Pokemon
    perish_song: u8,
    /// If true, this Pokemon is having a nightmare
    nightmare: bool,
    /// If true, this Pokemon cannot use Status moves
    taunted: u8,
    /// If true, this Pokemon is levitating
    levitating: u8,
    /// If true, this Pokemon can use any move except last_used_move
    tormented: bool,
    /// If true, this Pokemon gains 1/16 HP at the end of each turn
    aqua_ringed: bool,
    /// If present, this Pokemon will lose HP and the referred Battler will gain it
    leeched: Option<Battler>,
    /// Number of times braced. (0 == not tried)
    braced: u8,
    /// If present, this Pokemon is charging, and will use Move on the next turn
    charging: Option<Move>,
    /// If true, this Pokemon is the center of attention, and all moves target it
    center_of_attention: bool,
    /// If true, this Pokemon used Defense Curl. Certain moves are more powerful
    curled: bool,
    /// If true, this Pokemon is rooted to the ground. Gains HP at the end of each turn
    rooted: bool,
    /// If true, any status moves used against this Pokemon are rebounded
    magic_coated: bool,
    /// If true, the Pokemon is shrunk
    minimized: bool,
    /// Number of times protected. (0 == not tried)
    protected: u8,
    /// If true, the Pokemon is exhausted from its last attack, and cannot do anything
    recharging: bool,
    /// If present, the Pokemon is semi-invulnerable
    invulnerable: Option<SemiInvulnerableLocation>,
    /// If present, the Pokemon is carrying another (Sky Drop)
    carrying: Option<Battler>,
    /// The Pokemon has a substitute out, with this much HP remaining
    substituted: u16,
    /// This Pokemon is thrashing, and must continue using last_used_move.
    thrashing: u8,
    /// This Pokemon is transformed into another.
    transformed: Option<TransformData>,
    /// This Pokemon is focused, increasing crit ratio
    focused: bool,

    /// If true, this Pokemon had a held item + subsequently lost it
    lost_held_item: bool,
    /// If present, this Pokemon has this ability instead of its usual one
    temp_ability: Option<Ability>,
    /// If present, this Pokemon has this type instead of its usual one
    temp_type: Option<PokemonType>,
    /// If true, this Pokemon has been Power Tricked, and its Attack and Defense are swapped
    power_trick: bool
}
impl BattleData {
    pub fn is_confused(&self) -> bool {
        self.confused > 0
    }

    pub fn get_net_stages(&self, stat:BattleStat, stages: i8) -> i8 {
        let clamp = |v: i8| v.clamp(-6, 6);

        let var = match stat {
            BattleStat::Attack => &self.attack_stage,
            BattleStat::Defense => &self.defense_stage,
            BattleStat::SpecialAttack => &self.special_attack_stage,
            BattleStat::SpecialDefense => &self.special_defense_stage,
            BattleStat::Speed => &self.speed_stage,
            BattleStat::Accuracy => &self.accuracy_stage,
            BattleStat::Evasion => &self.evasion_stage
        };

        let before = *var;
        let after = clamp(before + stages);
        after - before
    }

    pub fn add_stages(&mut self, stat: BattleStat, stages: i8) {
        let clamp = |v: i8| v.clamp(-6, 6);

        let var = match stat {
            BattleStat::Attack => &mut self.attack_stage,
            BattleStat::Defense => &mut self.defense_stage,
            BattleStat::SpecialAttack => &mut self.special_attack_stage,
            BattleStat::SpecialDefense => &mut self.special_defense_stage,
            BattleStat::Speed => &mut self.speed_stage,
            BattleStat::Accuracy => &mut self.accuracy_stage,
            BattleStat::Evasion => &mut self.evasion_stage
        };

        *var = clamp(*var + stages);
    }

    pub fn end_of_turn(&mut self) {
        self.turn_count += 1;
        self.last_damager = None;
        self.has_acted_this_turn = false;
        self.has_landed_attack_this_turn = false;
    }
}

/// Identifier of a member on the field
#[derive(Debug, Clone, Copy)]
pub struct Battler(bool, u8);

#[derive(Debug)]
struct TransformData {
    species: Species,
    ability: AbilitySlot,
    attack: StatSlot,
    defense: StatSlot,
    special_attack: StatSlot,
    special_defense: StatSlot,
    speed: StatSlot,
    move_1: Option<MoveSlot>,
    move_2: Option<MoveSlot>,
    move_3: Option<MoveSlot>,
    move_4: Option<MoveSlot>
}
impl From<Pokemon> for TransformData {
    fn from(p: Pokemon) -> Self {
        TransformData {
            species: p.species,
            ability: p.ability,
            attack: p.attack,
            defense: p.defense,
            special_attack: p.special_attack,
            special_defense: p.special_defense,
            speed: p.speed,
            move_1: TransformData::transform_move(p.move_1),
            move_2: TransformData::transform_move(p.move_2),
            move_3: TransformData::transform_move(p.move_3),
            move_4: TransformData::transform_move(p.move_4)
        }
    }
}
impl TransformData {
    fn transform_move(slot: Option<MoveSlot>) -> Option<MoveSlot> {
        match slot {
            None => None,
            Some(m) => Some(m.copy_for_transform())
        }
    }

    fn get_ability(&self) -> &Ability {
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

/// One of the possible weather conditions that can occur on the field
#[derive(Debug)]
enum Weather {
    Rain(u8),
    Sun(u8),
    Hail(u8),
    Sandstorm(u8),
    Fog
}

/// One of the possible entry hazards that can occur on one side of the field
#[derive(Debug)]
enum EntryHazard {
    Spikes(u8),
    ToxicSpikes(u8),
    StickyWeb,
    PointedStones
}

#[derive(Debug)]
enum SemiInvulnerableLocation {
    Underground,
    Underwater,
    InAir
}

// Turn Recording (for use by UIs)
/// One of the usable actions that can be taken in a turn
#[derive(Debug)]
pub enum TurnAction {
    Attack(Move, Battler),
    Swap(u8),
    UseItem(Item),
    Flee
}
impl TurnAction{
    /// Get the priority of this move
    /// 0 is default. >0 means it will happen sooner, and <0 means it will happen later
    fn get_priority(&self) -> i8 {
        match self {
            TurnAction::Attack(m, _) => m.data().priority,
            TurnAction::Swap(_) => 10,
            TurnAction::UseItem(_) => 10,
            TurnAction::Flee => 10
        }
    }
}

/// Represents the actions taken during one turn of battle
#[derive(Debug)]
pub struct Turn {
    /// All side effects that occur at the start of the turn
    pub start_of_turn: Vec<ActionSideEffects>,
    /// All actions that take place during the turn, in order
    pub actions: Vec<ActionRecord>,
    /// All side effects that occur at the end of the turn
    pub end_of_turn: Vec<ActionSideEffects>
}
impl Turn {
    pub fn new() -> Turn {
        Turn {
            start_of_turn: Vec::new(),
            actions: Vec::new(),
            end_of_turn: Vec::new()
        }
    }
}

/// An action, and all consequences that occurred because of it
#[derive(Debug)]
pub struct ActionRecord {
    pub user: Battler,
    pub action: TurnAction,
    pub side_effects: Vec<ActionSideEffects>
}

/// The cause of some particular action's side effect
#[derive(Debug, Clone)]
pub enum Cause {
    /// This is just what normally happens
    Natural,
    /// A battler's ability caused the side effect
    Ability(Battler, Ability),
    /// A previously used move caused the side effect
    Move(Battler, Move),
    /// The side effect was the cause of a user's ailment
    Ailment(NonVolatileBattleAilment),
    /// A battler's held item caused the side effect
    HeldItem(Battler, Item),
    /// One cause was about to occur, but another one overwrote it
    Overwrite{
        initial: Box<Cause>,
        overwriter: Box<Cause>
    }
}
impl Cause {
    pub fn of_ability(battler: &Battler, ability: Ability) -> Cause {
        Cause::Ability(battler.clone(), ability)
    }

    pub fn overwrite(self, cause: Cause) -> Cause {
        Cause::Overwrite {
            initial: Box::from(self),
            overwriter: Box::from(cause.clone())
        }
    }
}

/// Possible consequences of an Action
/// Plan is to use these to determine which text boxes to say.
#[derive(Debug)]
pub enum ActionSideEffects {
    DirectDamage {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16,
        critical_hit: bool,
        effectiveness: Effectiveness
    },
    Recoil {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16
    },
    BasicDamage {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16,
        cause: Cause
    },
    Missed,
    NoEffect(Cause),
    Failed(Cause),
    DamagedSubstitute {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16
    },
    ConsumedItem(Battler, Item),
    NoTarget
}

impl <T> Battlefield<T> where T: BattleTypeTrait {
    // pub fn do_attack(&mut self, user: Battler, attack: Move, defender: Battler, is_multi: bool) -> Vec<ActionSideEffects> {
    //     let attacker_data = self.get_by_id(&user).unwrap();
    //     let attack_data = attack.data();
    //
    //     // Step 1: Ensure the target exists
    //     let defender_data = self.get_by_id(&defender);
    //     if defender_data.is_none() {
    //         return vec![ActionSideEffects::NoTarget];
    //     }
    //     let defender_data = defender_data.unwrap();
    //     let effective_attack_type = if *attacker_data.get_effective_ability() == Ability::Normalize {
    //         Type::Normal
    //     } else {
    //         attack_data._type
    //     };
    //
    //     //region Suppression
    //     let (suppressed, supression_cause) = match (*attacker_data.get_effective_ability(), *defender_data.get_effective_ability()) {
    //         (Ability::Damp, _) if attack == Move::Explosion || attack == Move::SelfDestruct => (true, Cause::of_ability(&user, Ability::Damp)),
    //         (_, Ability::Damp) if attack == Move::Explosion || attack == Move::SelfDestruct => (true, Cause::of_ability(&defender, Ability::Damp)),
    //         _ => (false, Cause::Natural)
    //     };
    //
    //     if suppressed {
    //         return vec![ActionSideEffects::Failed(supression_cause)]
    //     }
    //     //endregion
    //
    //     //region Accuracy
    //     let accuracy_succeeds =
    //         match attack_data.accuracy {
    //             Accuracy::AlwaysHits => true,
    //             Accuracy::Percentage(bp) => {
    //                 if *attacker_data.get_effective_ability() == Ability::NoGuard || *defender_data.get_effective_ability() == Ability::NoGuard {
    //                     true
    //                 } else if *defender_data.get_effective_ability() == Ability::LightningRod && effective_attack_type == Type::Electric {
    //                     true
    //                 } else if *defender_data.get_effective_ability() == Ability::StormDrain && effective_attack_type == Type::Water {
    //                     true
    //                 } else {
    //                     let accuracy_factor = self.get_accuracy_factor(&user, &attack, &defender) * f64::from(bp);
    //                     rand::thread_rng().gen_range(0f64..=100f64) < accuracy_factor
    //                 }
    //             }
    //             Accuracy::Variable => {
    //                 match attack {
    //                     Move::Fissure | Move::Guillotine | Move::HornDrill | Move::SheerCold => {
    //                         if *attacker_data.get_effective_ability() == Ability::NoGuard || *defender_data.get_effective_ability() == Ability::NoGuard {
    //                             true
    //                         } else {
    //                             let accuracy_factor = 30 + (attacker_data.pokemon.level - defender_data.pokemon.level);
    //                             rand::thread_rng().gen_range(0u8..=100u8) < accuracy_factor
    //                         }
    //                     },
    //                     _ => panic!("Unknown Move with Variable accuracy.")
    //                 }
    //             }
    //         };
    //
    //     if !accuracy_succeeds {
    //         return vec![ActionSideEffects::Missed]
    //     }
    //     //endregion
    //
    //     //region Effectiveness
    //     let default_effectiveness = || defender_data.get_effective_type().defending_against(&effective_attack_type);
    //     let effectiveness = default_effectiveness();
    //     // 3a: Defender Ability + Held Items
    //     let (effectiveness, defender_cause) =
    //         match *defender_data.get_effective_ability() {
    //             Ability::Levitate if attack_data._type == Type::Ground => (Effectiveness::Immune, Cause::of_ability(&defender, Ability::Levitate)),
    //             Ability::WonderGuard => {
    //                 match effectiveness {
    //                     Effectiveness::Effect(e) if e > 0 => (Effectiveness::Effect(e), Cause::Natural),
    //                     _ => (Effectiveness::Immune, Cause::of_ability(&defender, Ability::WonderGuard))
    //                 }
    //             },
    //             Ability::Soundproof if attack.is_sound_based() => (Effectiveness::Immune, Cause::of_ability(&defender, Ability::Soundproof)),
    //             Ability::Overcoat if attack.is_powder() => (Effectiveness::Immune, Cause::of_ability(&defender, Ability::Overcoat)),
    //             _ => (effectiveness, Cause::Natural)
    //         };
    //
    //     // 3b: Attacker Ability + Held Items
    //     let (effectiveness, attacker_cause) = match *attacker_data.get_effective_ability() {
    //         Ability::Scrappy if effective_attack_type == Type::Normal || effective_attack_type == Type::Fighting => {
    //             match effectiveness {
    //                 Effectiveness::Immune => (Effectiveness::NORMAL, Cause::of_ability(&user, Ability::Scrappy)),
    //                 e => (e, Cause::Natural)
    //             }
    //         },
    //         Ability::TintedLens => {
    //             match effectiveness {
    //                 Effectiveness::Effect(a) if a < 0 => (Effectiveness::Effect(a + 1), Cause::of_ability(&user, Ability::TintedLens)),
    //                 a => (a, Cause::Natural)
    //             }
    //         },
    //         Ability::MoldBreaker | Ability::Turboblaze | Ability::Teravolt => {
    //             match (&effectiveness, &defender_cause) {
    //                 (Effectiveness::Immune, Cause::Ability(_, _)) => (default_effectiveness(), Cause::of_ability(&user, *attacker_data.get_effective_ability())),
    //                 _ => (effectiveness, Cause::Natural)
    //             }
    //         }
    //         _ => (effectiveness, Cause::Natural)
    //     };
    //
    //     if let Effectiveness::Immune = effectiveness {
    //         let final_cause = match (attacker_cause, defender_cause) {
    //             (Cause::Natural, Cause::Natural) => Cause::Natural,
    //             (Cause::Natural, defender) => defender,
    //             (attacker, Cause::Natural) => attacker,
    //             (attacker, defender) => attacker.overwrite(defender)
    //         };
    //         return vec![ActionSideEffects::NoEffect(final_cause)]
    //     }
    //     //endregion
    //
    //     // Beyond this point, we *will* hit the attacker. We just need to figure out how and why
    //
    //     vec![]
    // }
}