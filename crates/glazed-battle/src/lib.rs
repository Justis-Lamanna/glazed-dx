#![feature(get_mut_unchecked)]

pub mod effects;
mod core;
mod damage;
mod turn;

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::option::Option::Some;
use std::rc::Rc;
use fraction::Fraction;
use rand::Rng;
use glazed_data::abilities::{Ability, PokemonAbility};
use glazed_data::attack::{BattleStat, DamageType, Effect, Move, NonVolatileBattleAilment, SemiInvulnerableLocation, StatChangeTarget, Target, VolatileBattleAilment};
use glazed_data::constants::Species;
use glazed_data::item::{Berry, EvolutionHeldItem, Incense, Item};
use glazed_data::pokemon::{AbilitySlot, MoveSlot, Pokemon, StatSlot};
use glazed_data::types::{Effectiveness, PokemonType, Type};

use crate::effects::{MAX_STAGE, MIN_STAGE, SelectedTarget};

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
    magic_room: u8,
    coins_on_ground: u16
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

    pub fn drop_coins(&mut self, coins: u16) {
        self.coins_on_ground = self.coins_on_ground.saturating_add(coins);
    }
}

/// A group of between 1 and 6 Pokemon, which a trainer owns
#[derive(Debug)]
pub struct Party {
    pub one: RefCell<Pokemon>,
    pub two: Option<RefCell<Pokemon>>,
    pub three: Option<RefCell<Pokemon>>,
    pub four: Option<RefCell<Pokemon>>,
    pub five: Option<RefCell<Pokemon>>,
    pub six: Option<RefCell<Pokemon>>,
    pub size: usize
}
impl Party {
    pub fn create<PKMN, ITER>(party: ITER) -> Party
        where ITER: IntoIterator<Item=PKMN>, PKMN: Into<Pokemon>
    {
        let mut party = party.into_iter()
            .map(|f| f.into())
            .collect::<Vec<Pokemon>>();
        let size = party.len();
        let mut party = party.drain(..);
        Party {
            one: RefCell::from(party.next().unwrap()),
            two: party.next().map(|i| RefCell::from(i)),
            three: party.next().map(|i| RefCell::from(i)),
            four: party.next().map(|i| RefCell::from(i)),
            five: party.next().map(|i| RefCell::from(i)),
            six: party.next().map(|i| RefCell::from(i)),
            size
        }
    }

    pub fn create_one<T>(one: T) -> Party
        where T: Into<Pokemon> {
        Party {
            one: RefCell::from(one.into()),
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            size: 1
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub enum BattleParty {
    Single (ActivePokemon),
    Double (ActivePokemon, ActivePokemon),
    Tag (ActivePokemon, ActivePokemon)
}
impl BattleParty {
    pub fn left(&self) -> &ActivePokemon {
        match self {
            BattleParty::Single(p) | BattleParty::Double(p, _) | BattleParty::Tag(p, _) => p
        }
    }
    pub fn right(&self) -> &ActivePokemon {
        match self {
            BattleParty::Single(p) | BattleParty::Double(_, p) | BattleParty::Tag(_, p) => p
        }
    }
    pub fn both(&self) -> Vec<&ActivePokemon> {
        match self {
            BattleParty::Single(p) => vec![p],
            BattleParty::Double(a, b) | BattleParty::Tag(a, b) => vec![a, b]
        }
    }
    pub fn ally_of(&self, me: &DoubleBattleSide) -> &ActivePokemon {
        match me {
            DoubleBattleSide::Left => self.right(),
            DoubleBattleSide::Right => self.left()
        }
    }

    pub fn left_mut(&mut self) -> &mut ActivePokemon {
        match self {
            BattleParty::Single(p) | BattleParty::Double(p, _) | BattleParty::Tag(p, _) => p
        }
    }
    pub fn right_mut(&mut self) -> &mut ActivePokemon {
        match self {
            BattleParty::Single(p) | BattleParty::Double(_, p) | BattleParty::Tag(_, p) => p
        }
    }
}
impl Index<DoubleBattleSide> for BattleParty {
    type Output = ActivePokemon;

    fn index(&self, index: DoubleBattleSide) -> &Self::Output {
        match index {
            DoubleBattleSide::Left => self.left(),
            DoubleBattleSide::Right => self.right()
        }
    }
}
impl IndexMut<DoubleBattleSide> for BattleParty {
    fn index_mut(&mut self, index: DoubleBattleSide) -> &mut Self::Output {
        match index {
            DoubleBattleSide::Left => self.left_mut(),
            DoubleBattleSide::Right => self.right_mut()
        }
    }
}

/// Smart pointer that points to the current active Pokemon
#[derive(Debug)]
pub struct ActivePokemon {
    id: Battler,
    party: Rc<Party>,
    pokemon: usize,
    data: RefCell<BattleData>,
}
impl ActivePokemon {
    /// Get the effective species of this Pokemon. Takes Transform into effect
    pub fn get_effective_species(&self) -> Species {
        match &self.data.borrow().transformed {
            None => self.borrow().species,
            Some(t) => t.species
        }
    }

    /// Get the effective ability of this Pokemon. Takes Transform and temporary Ability changes into effect
    pub fn get_effective_ability(&self) -> Ability {
        self.data.borrow().temp_ability.unwrap_or_else(|| {
            match &self.data.borrow().transformed {
                None => self.borrow().get_ability(),
                Some(t) => *t.get_ability()
            }
        })
    }

    /// Get the effective type(s) of this Pokemon. Takes Transform and temporary Type changes into effect
    pub fn get_effective_type(&self) -> PokemonType {
        self.data.borrow().temp_type.unwrap_or_else(|| {
            match &self.data.borrow().transformed {
                Some(t) => t.species.data()._type,
                None => self.borrow().species.data()._type,
            }
        })
    }

    pub fn get_stat_stage(&self, stat: BattleStat) -> i8 {
        let data = self.data.borrow();
        match stat {
            BattleStat::Attack => data.attack_stage,
            BattleStat::Defense => data.defense_stage,
            BattleStat::SpecialAttack => data.special_attack_stage,
            BattleStat::SpecialDefense => data.special_defense_stage,
            BattleStat::Speed => data.speed_stage,
            BattleStat::Accuracy => data.accuracy_stage,
            BattleStat::Evasion => data.evasion_stage
        }
    }

    /// Get the effective stat of this Pokemon. Takes Transform and stages into effect
    pub fn get_effective_stat(&self, stat: BattleStat) -> u16 {
        let pkmn = self.borrow();
        let data = self.data.borrow();
        let base = match stat {
            BattleStat::Attack => data.transformed.as_ref().map(|t| t.attack.value).or(data.temp_attack).unwrap_or(pkmn.attack.value),
            BattleStat::Defense => data.transformed.as_ref().map(|t| t.defense.value).or(data.temp_defense).unwrap_or(pkmn.defense.value),
            BattleStat::SpecialAttack => data.transformed.as_ref().map(|t| t.special_attack.value).or(data.temp_special_attack).unwrap_or(pkmn.special_attack.value),
            BattleStat::SpecialDefense => data.transformed.as_ref().map(|t| t.special_defense.value).or(data.temp_special_defense).unwrap_or(pkmn.special_defense.value),
            BattleStat::Speed => data.transformed.as_ref().map(|t| t.speed.value).unwrap_or(pkmn.speed.value),
            BattleStat::Accuracy => 1,
            BattleStat::Evasion => 1
        };

        let stage = self.get_stat_stage(stat);

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
    pub fn get_raw_critical_hit(&self) -> u8 {
        let mut value = 0;
        value += match self.borrow().held_item {
            Some(Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw)) => 1,
            Some(Item::ScopeLens) => 1,
            Some(Item::Leek) if self.borrow().species == Species::Farfetchd => 2,
            Some(Item::LuckyPunch) if self.borrow().species == Species::Chansey => 2,
            _ => 0
        };
        value += match self.get_effective_ability() {
            Ability::SuperLuck => 1,
            _ => 0
        };
        value
    }

    /// Get the effective weight of this Pokemon. Takes Ability, held item, and Autotomize into account
    pub fn get_effective_weight(&self) -> u16 {
        let mut weight = self.data.borrow().temp_weight.unwrap_or_else(|| self.get_effective_species().data().weight);
        let ability = self.get_effective_ability();

        if ability == Ability::HeavyMetal {
            weight = weight.saturating_mul(2);
        } else if ability == Ability::LightMetal {
            weight = weight.saturating_div(2);
        }

        if let Some(Item::FloatStone) = self.borrow().held_item {
            weight = weight.saturating_div(2);
        }

        if weight == 0 { 1 } else { weight }
    }

    pub fn is_grounded(&self) -> (bool, Cause) {
        if let Some(Item::IronBall) = self.borrow().held_item {
            return (true, Cause::HeldItem(self.id, Item::IronBall))
        }
        (false, Cause::Natural)
    }

    pub fn is_behind_substitute(&self) -> bool {
        self.data.borrow().substituted > 0
    }

    pub fn lower_hp(&mut self, amount: u16) -> Vec<ActionSideEffects> {
        let start_hp = self.borrow().current_hp;
        let end_hp = start_hp.saturating_sub(amount);
        self.borrow_mut().current_hp = end_hp;
        vec![ActionSideEffects::BasicDamage {
            damaged: self.id,
            start_hp,
            end_hp,
            cause: Cause::Natural,
            hung_on_cause: None
        }]
    }

    pub fn take_crash_damage(&self) -> ActionSideEffects {
        let mut pkmn = self.borrow_mut();
        let crash = pkmn.hp.value / 2;
        let start_hp = pkmn.current_hp;
        let end_hp = start_hp.saturating_sub(crash);
        pkmn.current_hp = end_hp;

        ActionSideEffects::Crash {
            damaged: self.id,
            start_hp,
            end_hp
        }
    }
}
impl Deref for ActivePokemon {
    type Target = RefCell<Pokemon>;

    fn deref(&self) -> &Self::Target {
        match self.pokemon {
            0 => &self.party.one,
            1 => &self.party.two.as_ref().unwrap(),
            2 => &self.party.three.as_ref().unwrap(),
            3 => &self.party.four.as_ref().unwrap(),
            4 => &self.party.five.as_ref().unwrap(),
            5 => &self.party.six.as_ref().unwrap(),
            _ => panic!("Unknown pokemon index")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BattleSide {
    Forward,
    Back
}

#[derive(Debug, Copy, Clone)]
pub enum DoubleBattleSide {
    Left, Right
}

/// Represents the current battlefield
#[derive(Debug)]
pub struct Battlefield {
    user: BattleParty,
    user_side: Side,
    opponent: BattleParty,
    opponent_side: Side,
    field: RefCell<Field>,
    wild_battle: bool,
    turn_record: Vec<Turn>
}
impl Battlefield {
    /// Standard 1v1 battle
    pub fn single_battle(player: Party, opponent: Party) -> Battlefield {
        Battlefield {
            user: BattleParty::Single(ActivePokemon {
                id: Battler { side: BattleSide::Forward, individual: DoubleBattleSide::Left },
                party: Rc::new(player),
                pokemon: 0,
                data: Default::default()
            }),
            user_side: Side::default(),
            opponent: BattleParty::Single(ActivePokemon {
                id: Battler { side: BattleSide::Back, individual: DoubleBattleSide::Left },
                party: Rc::new(opponent),
                pokemon: 0,
                data: Default::default()
            }),
            opponent_side: Side::default(),
            field: RefCell::from(Field::default()),
            wild_battle: false,
            turn_record: Vec::new()
        }
    }

    fn opposite_of(&self, side: &BattleSide) -> &BattleParty {
        match side {
            BattleSide::Forward => &self.opponent,
            BattleSide::Back => &self.user
        }
    }

    pub fn get_targets(&self, attacker: Battler, attack: Move, selected: SelectedTarget) -> Vec<&ActivePokemon> {
        let selected_default_opponent = || match selected {
            SelectedTarget::Implied => self.opposite_of(&attacker.side).left(), // Default is opponent on the left, although this shouldn't be called normally
            SelectedTarget::One(b) => &self[b.side][b.individual]
        };
        let user = || &self[attacker.side][attacker.individual];
        let ally = || self[attacker.side].ally_of(&attacker.individual);

        let move_data = attack.data();

        match move_data.target {
            Target::User => vec![user()],
            Target::Ally => vec![ally()],
            Target::UserAndAlly => vec![user(), ally()],
            Target::UserOrAlly => {
                match selected {
                    SelectedTarget::Implied => vec![user()],
                    SelectedTarget::One(b) => vec![&self[b.side][b.individual]]
                }
            },
            Target::Opponent => vec![selected_default_opponent()],
            Target::Opponents => self.opposite_of(&attacker.side).both(),
            Target::AllyOrOpponent => vec![selected_default_opponent()],
            Target::RandomOpponent => {
                let p = match self.opposite_of(&attacker.side) {
                    BattleParty::Single(p) => p,
                    BattleParty::Double(a, b) |
                    BattleParty::Tag(a, b) => if rand::thread_rng().gen_bool(0.5) { a } else { b }
                };
                vec![p]
            }
            Target::Any => vec![selected_default_opponent()],
            Target::AllExceptUser => vec![selected_default_opponent()],
            Target::All => {
                let mut vec = vec![user(), ally()];
                vec.append(&mut self.opposite_of(&attacker.side).both());
                vec
            }
            Target::LastAttacker(pred) => {
                let u = user();
                let data = u.data.borrow();
                let target = match pred {
                    None => data.damage_this_turn.last(),
                    Some(dt) => {
                        data.damage_this_turn
                            .iter()
                            .filter(|(_, attack, _)| attack.data().damage_type == dt)
                            .last()
                    }
                };
                match target {
                    None => vec![],
                    Some((battler, _, _)) => vec![&self[battler.side][battler.individual]]
                }
            }
        }
    }

    /// Get the side associated with a battler
    fn get_side(&self, id: &Battler) -> &Side {
        match id.side {
            BattleSide::Forward => &self.user_side,
            BattleSide::Back => &self.opponent_side
        }
    }

    fn get_everyone(&self) -> Vec<&ActivePokemon> {
        let mut active = self.user.both();
        active.append(&mut self.opponent.both());
        active
    }

    /// Pushes this Turn to the Turn Record, to signal it is complete and permanent.
    /// Intentionally eats the passed-in turn...it belongs to the battlefield now
    pub fn finish_turn(&mut self, turn: Turn) {
        self.turn_record.push(turn);
    }
}
impl Index<BattleSide> for Battlefield {
    type Output = BattleParty;

    fn index(&self, index: BattleSide) -> &Self::Output {
        match index {
            BattleSide::Forward => &self.user,
            BattleSide::Back => &self.opponent
        }
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

    /// Manipulated stats (Power Trick, Power Split, Guard Split)
    temp_attack: Option<u16>,
    temp_defense: Option<u16>,
    temp_special_attack: Option<u16>,
    temp_special_defense: Option<u16>,

    /// If present, the turns remaining bound, and whether a Binding Band was used
    bound: Option<(u8, bool)>,
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
    charging: Option<(SelectedTarget, Move)>,
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
pub struct Battler {
    side: BattleSide,
    individual: DoubleBattleSide
}
impl Battler {
    pub fn single(side: BattleSide) -> Battler {
        Battler {
            side,
            individual: DoubleBattleSide::Left
        }
    }

    pub fn double(side: BattleSide, side2: DoubleBattleSide) -> Battler {
        Battler {
            side,
            individual: side2
        }
    }
}

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
    /// The side effect was the cause of a user's non-volatile ailment
    Ailment(NonVolatileBattleAilment),
    /// The side effect was the cause of a user's volatile ailment
    Ailment2(VolatileBattleAilment),
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
    Field(FieldCause),
    /// Failed because the user is behind a substitute
    Substitute(Battler)
}
impl Cause {
    pub fn overwrite(self, cause: Cause) -> Cause {
        match self {
            Cause::Natural => cause,
            _ => Cause::Overwrite {
                initial: Box::from(self),
                overwriter: Box::from(cause.clone())
            }
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
    Crash {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16
    },
    BasicDamage {
        damaged: Battler,
        start_hp: u16,
        end_hp: u16,
        cause: Cause,
        hung_on_cause: Option<Cause>
    },
    Missed(Battler, Cause),
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
    Confuse(Battler),
    ConfusionTurn(Battler),
    HurtInConfusion {
        affected: Battler,
        start: u16,
        end: u16,
        hang_on_cause: Option<Cause>
    },
    SnappedOutOfConfusion(Battler),
    Infatuated(Battler),
    TooInfatuatedToAttack(Battler),
    ForcePokemonSwap {
        must_leave: Battler
    },
    DroppedCoins,
    Charging(Battler, Move),
    Bound {
        binder: Battler,
        bound: Battler,
        turns: u8,
        attack: Move
    },
    HurtByBound {
        bound: Battler,
        start_hp: u16,
        end_hp: u16
    },
    Unbound(Battler),
    WillFlinch(Battler),
    Flinched(Battler),
    NothingHappened
}
impl ActionSideEffects {
    pub fn hit_substitute(&self) -> bool {
        if let ActionSideEffects::DamagedSubstitute {..} = self {
            true
        } else {
            false
        }
    }
}