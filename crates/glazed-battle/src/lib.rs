pub mod effects;
mod core;
mod damage;

use std::option::Option::Some;
use rand::Rng;
use glazed_data::abilities::{Ability, PokemonAbility};
use glazed_data::attack::{BattleStat, Move, NonVolatileBattleAilment, StatChangeTarget, Target};
use glazed_data::constants::Species;
use glazed_data::item::{Berry, EvolutionHeldItem, Incense, Item};
use glazed_data::pokemon::{AbilitySlot, MoveSlot, Pokemon, StatSlot};
use glazed_data::types::{Effectiveness, PokemonType, Type};
use crate::effects::{BattleBundle, MAX_STAGE, MIN_STAGE};

/// Represents one side of a battlefield
#[derive(Default, Debug)]
pub struct Side {
    hazard: Option<EntryHazard>,
    tailwind: u8,
    aurora_veil: u8,
    light_screen: u8,
    reflect: u8,
    mist: u8
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
    pub number_of_members: usize,
    pub members: [Option<Pokemon>; 6]
}
impl Party {
    pub fn create<PKMN, ITER>(party: ITER) -> Party
        where ITER: IntoIterator<Item=PKMN>, PKMN: Into<Pokemon>
    {
        let mut members = [None, None, None, None, None, None];
        let mut counter = 0;
        let mut party = party.into_iter();
        for idx in 0..6usize {
            match party.next() {
                None => { break; }
                Some(template) => {
                    members[idx] = Some(template.into());
                    counter += 1;
                }
            }
        }

        Party {
            number_of_members: counter,
            members
        }
    }

    pub fn create_one<T>(one: T) -> Party
        where T: Into<Pokemon> {
        Party {
            number_of_members: 1,
            members: [Some(one.into()), None, None, None, None, None]
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

    pub fn single(self) -> BattleType {
        BattleType::Single(self, 0, BattleData::default())
    }

    pub fn double(self) -> BattleType {
        BattleType::Double(self, (0, 1), (BattleData::default(), BattleData::default()))
    }

    pub fn paired_with(self, other: Party) -> BattleType {
        BattleType::Tag(self, 0, BattleData::default(), other, 0, BattleData::default())
    }
}

#[derive(Debug)]
pub enum BattleType {
    Single(Party, usize, BattleData),
    Double(Party, (usize, usize), (BattleData, BattleData)),
    Tag(Party, usize, BattleData, Party, usize, BattleData)
}
impl From<Party> for BattleType {
    fn from(p: Party) -> Self {
        BattleType::Single(p, 0, BattleData::default())
    }
}

/// Represents the current battlefield
#[derive(Debug)]
pub struct Battlefield {
    user: BattleType,
    user_side: Side,
    opponent: BattleType,
    opponent_side: Side,
    field: Field,
    turn_record: Vec<Turn>
}
impl Battlefield {
    pub const USER: Battler = Battler(true, 0);
    pub const ALLY: Battler = Battler(true, 1);
    pub const OPPONENT: Battler = Battler(false, 0);
    pub const OPPONENT_TWO: Battler = Battler(false, 1);

    /// Standard 1v1 battle
    pub fn single_battle(player: Party, opponent: Party) -> Battlefield {
        Battlefield {
            user: player.single(),
            user_side: Side::default(),
            opponent: opponent.single(),
            opponent_side: Side::default(),
            field: Field::default(),
            turn_record: Vec::new()
        }
    }

    /// Battlefield for 1v1, but each fighter uses 2 pokemon
    /// Example: fighting pairs (Twins, Young Couple, etc)
    pub fn double_battle(player: Party, opponent: Party) -> Battlefield {
        Battlefield {
            user: player.double(),
            user_side: Side::default(),
            opponent: opponent.double(),
            opponent_side: Side::default(),
            field: Field::default(),
            turn_record: Vec::new()
        }
    }

    /// Battlefield for 1v2
    /// Example: two individual trainers see you at the same time
    pub fn single_vs_double(player: Party, opponent_one: Party, opponent_two: Party) -> Battlefield {
        Battlefield {
            user: player.double(),
            user_side: Side::default(),
            opponent: opponent_one.paired_with(opponent_two),
            opponent_side: Side::default(),
            field: Field::default(),
            turn_record: Vec::new()
        }
    }

    /// Battle for 2v2,
    /// Example: when Steven fights alongside you against Team Magma
    pub fn tag(player: Party, ally: Party, opponent_one: Party, opponent_two: Party) -> Battlefield {
        Battlefield {
            user: player.paired_with(ally),
            user_side: Side::default(),
            opponent: opponent_one.paired_with(opponent_two),
            opponent_side: Side::default(),
            field: Field::default(),
            turn_record: Vec::new()
        }
    }

    fn get_battle_bundle(&self, id: &Battler) -> BattleBundle {
        let Battler(side, idx) = id;
        let (party, current, data) = match if *side { &self.user } else { &self.opponent } {
            BattleType::Single(party, current, data) => (party, current, data),
            BattleType::Double(party, (one, two), (dataOne, dataTwo)) => {
                let (current, data) = if *idx == 0 { (one, dataOne) } else { (two, dataTwo) };
                (party, current, data)
            }
            BattleType::Tag(partyOne, outOne, dataOne, partyTwo, outTwo, dataTwo) => {
                if *idx == 0 { (partyOne, outOne, dataOne) } else { (partyTwo, outTwo, dataTwo) }
            }
        };
        let pokemon = party.members[*current].as_ref().unwrap();
        let ability = core::get_effective_ability(pokemon, data);

        (*id, pokemon, data, ability)
    }

    fn apply_side_effect(&mut self, effect: &ActionSideEffects) {
        match effect {
            ActionSideEffects::BasicDamage {damaged, end_hp, cause, ..} => {
                let mut damaged_pkmn = self.get_mut_pokemon_by_id(&damaged);
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
                let damaged_pkmn = self.get_mut_pokemon_by_id(&damaged);
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
                battler.held_item = None;
            },
            ActionSideEffects::Recoil {damaged, end_hp, .. } => {
                let mut damaged = self.get_mut_pokemon_by_id(&damaged);
                damaged.current_hp = *end_hp;
            }
            ActionSideEffects::Missed(_) => {}
            ActionSideEffects::NoEffect(_) => {}
            ActionSideEffects::Failed(_) => {}
            ActionSideEffects::PainSplit { user, defender, split_hp } => {
                let mut first = self.get_mut_pokemon_by_id(user);
                first.current_hp = (*split_hp).clamp(0, first.hp.value);

                let mut second = self.get_mut_pokemon_by_id(defender);
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
            ActionSideEffects::NonVolatileStatusAilment { affected, status } => {
                let mut pkmn = self.get_mut_pokemon_by_id(affected);
                match status {
                    NonVolatileBattleAilment::Paralysis => {
                        pkmn.status.paralysis = true;
                    }
                    NonVolatileBattleAilment::Sleep => {
                        pkmn.status.sleep = rand::thread_rng().gen_range(1..=3);
                    }
                    NonVolatileBattleAilment::Freeze => {
                        pkmn.status.freeze = true;
                    }
                    NonVolatileBattleAilment::Burn => {
                        pkmn.status.burn = true;
                    }
                    NonVolatileBattleAilment::Poison(poison_type) => {
                        pkmn.status.poison = Some(*poison_type);
                    }
                }
            }
            ActionSideEffects::Thawed(affected) => {
                let mut pkmn = self.get_mut_pokemon_by_id(affected);
                pkmn.status.freeze = false;
            }
            ActionSideEffects::WasFrozen(_) => {}
            ActionSideEffects::Sleep(affected) => {
                let mut pkmn = self.get_mut_pokemon_by_id(affected);
                pkmn.status.sleep = pkmn.status.sleep - 1;
            }
            ActionSideEffects::WokeUp(affected) => {
                let mut pkmn = self.get_mut_pokemon_by_id(affected);
                pkmn.status.sleep = 0;
            }
            ActionSideEffects::IsFullyParalyzed(_) => {}
        }
    }

    fn apply_side_effects(&mut self, effects: &Vec<ActionSideEffects>) {
        for effect in effects {
            self.apply_side_effect(effect);
        }
    }

    fn get_mut_pokemon_by_id(&mut self, id: &Battler) -> &mut Pokemon {
        let Battler(side, idx) = id;
        let (party, current) = match if *side { &mut self.user } else { &mut self.opponent } {
            BattleType::Single(party, current, _) => (party, current),
            BattleType::Double(party, (one, two), _) => {
                let current = if *idx == 0 { one } else { two };
                (party, current)
            }
            BattleType::Tag(partyOne, outOne, _, partyTwo, outTwo, _) => {
                if *idx == 0 { (partyOne, outOne) } else { (partyTwo, outTwo) }
            }
        };
        party.members[*current].as_mut().unwrap()
    }

    fn get_mut_battle_data(&mut self, id: &Battler) -> &mut BattleData {
        let Battler(side, idx) = id;
        match if *side { &mut self.user } else { &mut self.opponent } {
            BattleType::Single(_, _, data) => data,
            BattleType::Double(_, _, (one, two)) => {
                if *idx == 0 { one } else { two }
            }
            BattleType::Tag(_, _, one, _, _, two) => {
                if *idx == 0 { one } else { two }
            }
        }
    }

    fn get_party(&self, id: &Battler) -> &Party {
        let Battler(side, idx) = id;
        match if *side { &self.user } else { &self.opponent } {
            BattleType::Single(party, _, _) => party,
            BattleType::Double(party, _, _) => party,
            BattleType::Tag(one, _, _, two, _, _) => {
                if *idx == 0 { one } else { two }
            }
        }
    }

    fn get_side(&self, id: &Battler) -> &Side {
        let Battler(side, idx) = id;
        if *side {
            &self.user_side
        } else {
            &self.opponent_side
        }
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
    damage_this_turn: Vec<(Battler, Move, u16)>,
    /// If present, the user is biding (turns left, damage accumulated)
    bide: Option<(u8, u16)>,

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
    /// If present, this Pokemon has this weight instead of its usual weight
    temp_weight: Option<u16>,
    /// If true, this Pokemon has been Power Tricked, and its Attack and Defense are swapped
    power_trick: bool
}
impl BattleData {
    pub fn is_confused(&self) -> bool {
        self.confused > 0
    }

    pub fn get_stage(&self, stat: BattleStat) -> i8 {
        match stat {
            BattleStat::Attack => self.attack_stage,
            BattleStat::Defense => self.defense_stage,
            BattleStat::SpecialAttack => self.special_attack_stage,
            BattleStat::SpecialDefense => self.special_defense_stage,
            BattleStat::Speed => self.speed_stage,
            BattleStat::Accuracy => self.accuracy_stage,
            BattleStat::Evasion => self.evasion_stage
        }
    }

    pub fn end_of_turn(&mut self) {
        self.turn_count += 1;
        self.damage_this_turn = Vec::new();
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
    },
    /// Failed because the stats were maxed out
    StatsMaxed(StatsCause),
    /// Failed because of something on the field
    Field(FieldCause)
}
impl Cause {
    pub fn overwrite(self, cause: Cause) -> Cause {
        Cause::Overwrite {
            initial: Box::from(self),
            overwriter: Box::from(cause.clone())
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatsCause {
    TooHigh, TooLow
}

#[derive(Debug, Clone)]
pub enum FieldCause {
    Mist
}

/// Possible consequences of an Action
/// Plan is to use these to determine which text boxes to say.
#[derive(Debug)]
pub enum ActionSideEffects {
    DirectDamage {
        damaged: Battler,
        damager: Battler,
        attack: Move,
        start_hp: u16,
        end_hp: u16,
        hung_on_cause: Option<Cause>,
        critical_hit: bool,
        effectiveness: Effectiveness
    },
    Recoil {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16,
        cause: Cause
    },
    BasicDamage {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16,
        cause: Cause,
        hung_on_cause: Option<Cause>
    },
    Missed(Cause),
    NoEffect(Cause),
    Failed(Cause),
    MultiStrike {
        actions: Vec<Vec<ActionSideEffects>>,
        hits: u8
    },
    DamagedSubstitute {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16
    },
    ConsumedItem(Battler, Item),
    PainSplit {
        user: Battler,
        defender: Battler,
        split_hp: u16
    },
    NoTarget,

    NoEffectSecondary(Cause),
    StatChanged {
        affected: Battler,
        stat: BattleStat,
        cause: Cause,
        start: i8,
        end: i8
    },
    NonVolatileStatusAilment {
        affected: Battler,
        status: NonVolatileBattleAilment
    },
    Thawed(Battler),
    WasFrozen(Battler),
    Sleep(Battler),
    WokeUp(Battler),
    IsFullyParalyzed(Battler),

    NothingHappened
}
impl ActionSideEffects {
    pub fn is_multi_hit_end(&self) -> bool {
        match self {
            ActionSideEffects::Missed(_) | ActionSideEffects::NoEffect(_) | ActionSideEffects::Failed(_) | ActionSideEffects::NoTarget => true,
            ActionSideEffects::DirectDamage {end_hp, ..} | ActionSideEffects::BasicDamage {end_hp, ..} if *end_hp == 0 => true,
            _ => false
        }
    }
}