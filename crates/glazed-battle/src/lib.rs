use std::cell::{RefCell};
use std::convert::TryFrom;
use std::ops::{Deref, Index, IndexMut};
use std::option::Option::Some;
use std::rc::Rc;

use rand::Rng;

use glazed_data::abilities::{Ability, PokemonAbility};
use glazed_data::attack::{BattleStat, Move, NonVolatileBattleAilment, PoisonType, ScreenType, SemiInvulnerableLocation, Target, Weather};
use glazed_data::constants::Species;
use glazed_data::item::{EvolutionHeldItem, Item};
use glazed_data::pokemon::{AbilitySlot, Gender, MoveSlot, Pokemon, StatSlot};
use glazed_data::types::{Effectiveness, PokemonType, Type};

use crate::constants::{*};

mod effects;
mod core;
mod damage;
mod turn;
mod accuracy;
mod constants;

#[derive(Debug, Copy, Clone)]
pub enum SelectedTarget {
    Implied,
    One(SlotId)
}

/// Represents the entire battlefield
#[derive(Default, Debug)]
pub struct Field {
    weather: Option<WeatherCounter>,
    gravity: u8,
    magic_room: u8,
    coins_on_ground: u16
}
impl Field {
    /// Return if harsh sunlight is present on the field
    pub fn is_sunny(&self) -> bool {
        match self.weather {
            Some(WeatherCounter::Sun(_)) => true,
            _ => false
        }
    }

    /// Return if it is raining on the field
    pub fn is_rain(&self) -> bool {
        match self.weather {
            Some(WeatherCounter::Rain(_)) => true,
            _ => false
        }
    }

    /// Return if there is a sandstorm on the field
    pub fn is_sandstorm(&self) -> bool {
        match self.weather {
            Some(WeatherCounter::Sandstorm(_)) => true,
            _ => false
        }
    }

    /// Return if there is hail on the field
    pub fn is_hail(&self) -> bool {
        match self.weather {
            Some(WeatherCounter::Hail(_)) => true,
            _ => false
        }
    }

    pub fn is_foggy(&self) -> bool {
        match self.weather {
            Some(WeatherCounter::Fog) => true,
            _ => false
        }
    }

    pub fn is_clear(&self) -> bool {
        match self.weather {
            None => true,
            _ => false
        }
    }

    pub fn drop_coins(&mut self, coins: u16) {
        self.coins_on_ground = self.coins_on_ground.saturating_add(coins);
    }
}

/// Represents one side of a battlefield
#[derive(Debug)]
pub struct FieldSide {
    id: BattleSideId,
    spikes: u8,
    toxic_spikes: u8,
    pointed_stones: bool,
    tailwind: u8,
    aurora_veil: u8,
    light_screen: u8,
    reflect: u8,
    mist: u8
}
impl FieldSide {
    pub fn new(side: BattleSideId) -> FieldSide {
        FieldSide {
            id: side,
            spikes: 0,
            toxic_spikes: 0,
            pointed_stones: false,
            tailwind: 0,
            aurora_veil: 0,
            light_screen: 0,
            reflect: 0,
            mist: 0
        }
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
pub enum Side {
    Single (Slot),
    Double (Slot, Slot),
    Tag (Slot, Slot)
}
impl Side {
    pub fn left(&self) -> &Slot {
        match self {
            Side::Single(p) | Side::Double(p, _) | Side::Tag(p, _) => p
        }
    }
    pub fn right(&self) -> &Slot {
        match self {
            Side::Single(p) | Side::Double(_, p) | Side::Tag(_, p) => p
        }
    }
    pub fn both(&self) -> Vec<&Slot> {
        match self {
            Side::Single(p) => vec![p],
            Side::Double(a, b) | Side::Tag(a, b) => vec![a, b]
        }
    }
    pub fn ally_of(&self, me: &DoubleBattleSideId) -> &Slot {
        match me {
            DoubleBattleSideId::Left => self.right(),
            DoubleBattleSideId::Right => self.left()
        }
    }

    pub fn left_mut(&mut self) -> &mut Slot {
        match self {
            Side::Single(p) | Side::Double(p, _) | Side::Tag(p, _) => p
        }
    }
    pub fn right_mut(&mut self) -> &mut Slot {
        match self {
            Side::Single(p) | Side::Double(_, p) | Side::Tag(_, p) => p
        }
    }
}
impl Index<DoubleBattleSideId> for Side {
    type Output = Slot;

    fn index(&self, index: DoubleBattleSideId) -> &Self::Output {
        match index {
            DoubleBattleSideId::Left => self.left(),
            DoubleBattleSideId::Right => self.right()
        }
    }
}
impl IndexMut<DoubleBattleSideId> for Side {
    fn index_mut(&mut self, index: DoubleBattleSideId) -> &mut Self::Output {
        match index {
            DoubleBattleSideId::Left => self.left_mut(),
            DoubleBattleSideId::Right => self.right_mut()
        }
    }
}

/// Smart pointer that points to the current active Pokemon
#[derive(Debug)]
pub struct Slot {
    pub id: SlotId,
    party: Rc<Party>,
    pokemon: usize,
    pub data: RefCell<BattleData>,
}
impl Slot {
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
            BattleStat::Evasion => data.evasion_stage,
            BattleStat::CriticalHitRatio => i8::try_from(data.crit_stage).unwrap(),
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
            BattleStat::Accuracy => 0,
            BattleStat::Evasion => 0,
            BattleStat::CriticalHitRatio => 0
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
        let mut value = self.data.borrow().crit_stage;
        value += match self.borrow().held_item {
            Some(Item::EvolutionHeldItem(EvolutionHeldItem::RazorClaw)) => 1,
            Some(Item::ScopeLens) => 1,
            Some(Item::Leek) if self.get_effective_species() == Species::Farfetchd => 2,
            Some(Item::LuckyPunch) if self.get_effective_species() == Species::Chansey => 2,
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
            weight = weight / 2;
        }

        if let Some(Item::FloatStone) = self.borrow().held_item {
            weight = weight/ 2;
        }

        if weight == 0 { 1 } else { weight }
    }

    pub fn get_effective_move(&self, slot: usize) -> Option<MoveSlot> {
        let pkmn = self.borrow();
        let data = self.data.borrow();
        match slot {
            0 => data.temp_move_1.or(pkmn.move_1),
            1 => data.temp_move_2.or(pkmn.move_2),
            2 => data.temp_move_3.or(pkmn.move_3),
            _ => data.temp_move_4.or(pkmn.move_4)
        }
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
            cause: Cause::Natural
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

    pub fn take_poison_damage(&self) -> ActionSideEffects {
        let mut pkmn = self.borrow_mut();
        let data = self.data.borrow();
        if data.poison_counter > 0 {
            let mut d = pkmn.hp.value / 16;
            d *= data.poison_counter as u16;

            let start_hp = pkmn.current_hp;
            let end_hp = start_hp.saturating_sub(d);
            pkmn.current_hp = end_hp;

            ActionSideEffects::BasicDamage {
                damaged: self.id,
                start_hp,
                end_hp,
                cause: Cause::Ailment(NonVolatileBattleAilment::Poison(PoisonType::BadlyPoisoned))
            }
        } else {
            let d = pkmn.hp.value / 8;

            let start_hp = pkmn.current_hp;
            let end_hp = start_hp.saturating_sub(d);
            pkmn.current_hp = end_hp;

            ActionSideEffects::BasicDamage {
                damaged: self.id,
                start_hp,
                end_hp,
                cause: Cause::Ailment(NonVolatileBattleAilment::Poison(PoisonType::Poison))
            }
        }
    }

    pub fn consume_item(&self) -> Option<ActionSideEffects> {
        let mut pkmn = self.borrow_mut();
        if let Some(i) = &pkmn.held_item {
            let i = i.clone();
            pkmn.held_item = None;
            Some(ActionSideEffects::ConsumedItem(self.id, i))
        } else {
            None
        }
    }

    pub fn replace_mimic_with(&self, attack: Move) {
        let pkmn = self.borrow();
        let mut data = self.data.borrow_mut();

        if let Some(MoveSlot { attack: Move::Mimic, .. }) = pkmn.move_1 {
            data.temp_move_1 = Some(MoveSlot::from(attack));
        } else if let Some(MoveSlot { attack: Move::Mimic, .. }) = pkmn.move_2 {
            data.temp_move_2 = Some(MoveSlot::from(attack));
        } else if let Some(MoveSlot { attack: Move::Mimic, .. }) = pkmn.move_3 {
            data.temp_move_3 = Some(MoveSlot::from(attack));
        } else if let Some(MoveSlot { attack: Move::Mimic, .. }) = pkmn.move_4 {
            data.temp_move_4 = Some(MoveSlot::from(attack));
        }
    }

    pub fn replace_sketch_with(&self, attack: Move) {
        let mut data = self.data.borrow_mut();
        if let Some(transform) = &mut data.transformed {
            if let Some(MoveSlot { attack: Move::Sketch, .. }) = transform.move_1 {
                transform.move_1 = Some(MoveSlot::from(attack));
            } else if let Some(MoveSlot { attack: Move::Sketch, .. }) = transform.move_2 {
                transform.move_2 = Some(MoveSlot::from(attack));
            } else if let Some(MoveSlot { attack: Move::Sketch, .. }) = transform.move_3 {
                transform.move_3 = Some(MoveSlot::from(attack));
            } else if let Some(MoveSlot { attack: Move::Sketch, .. }) = transform.move_4 {
                transform.move_4 = Some(MoveSlot::from(attack));
            }
        } else {
            let mut pkmn = self.borrow_mut();
            if let Some(MoveSlot { attack: Move::Sketch, .. }) = pkmn.move_1 {
                pkmn.move_1 = Some(MoveSlot::from(attack));
            } else if let Some(MoveSlot { attack: Move::Sketch, .. }) = pkmn.move_2 {
                pkmn.move_2 = Some(MoveSlot::from(attack));
            } else if let Some(MoveSlot { attack: Move::Sketch, .. }) = pkmn.move_3 {
                pkmn.move_3 = Some(MoveSlot::from(attack));
            } else if let Some(MoveSlot { attack: Move::Sketch, .. }) = pkmn.move_4 {
                pkmn.move_4 = Some(MoveSlot::from(attack));
            }
        }
    }
}
impl Deref for Slot {
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

/// Represents the current battlefield
#[derive(Debug)]
pub struct Battlefield {
    user: Side,
    user_side: RefCell<FieldSide>,
    opponent: Side,
    opponent_side: RefCell<FieldSide>,
    field: RefCell<Field>,
    wild_battle: bool
}
impl Battlefield {
    pub fn one_v_one<F, G>(left: F, right: G) -> Battlefield
        where F: Into<Pokemon>, G: Into<Pokemon>
    {
        Battlefield::single_battle(Party::create_one(left), Party::create_one(right))
    }

    /// Standard 1v1 battle
    /// This should only be used for trainer battles
    pub fn single_battle(player: Party, opponent: Party) -> Battlefield {
        Battlefield {
            user: Side::Single(Slot {
                id: SlotId { side: BattleSideId::Forward, individual: DoubleBattleSideId::Left },
                party: Rc::new(player),
                pokemon: 0,
                data: Default::default()
            }),
            user_side: RefCell::from(FieldSide::new(BattleSideId::Forward)),
            opponent: Side::Single(Slot {
                id: SlotId { side: BattleSideId::Back, individual: DoubleBattleSideId::Left },
                party: Rc::new(opponent),
                pokemon: 0,
                data: Default::default()
            }),
            opponent_side: RefCell::from(FieldSide::new(BattleSideId::Back)),
            field: RefCell::from(Field::default()),
            wild_battle: false
        }
    }

    pub fn get_by_id(&self, id: &SlotId) -> &Slot {
        self.index(id.side).index(id.individual)
    }

    pub fn get_by_id_mut(&mut self, id: &SlotId) -> &mut Slot {
        self.index_mut(id.side).index_mut(id.individual)
    }

    fn opposite_of(&self, side: &BattleSideId) -> &Side {
        match side {
            BattleSideId::Forward => &self.opponent,
            BattleSideId::Back => &self.user
        }
    }

    pub fn get_targets(&self, attacker: SlotId, attack: Move, selected: SelectedTarget) -> Vec<&Slot> {
        let implied_opponent = || {
            let default_target = self.opposite_of(&attacker.side).index(attacker.individual);
            if default_target.borrow().is_fainted() {
                let default_target = self.opposite_of(&attacker.side).ally_of(&attacker.individual);
                if default_target.borrow().is_fainted() {
                    None
                } else {
                    Some(default_target)
                }
            } else {
                Some(default_target)
            }
        };
        let unimplied_opponent = |b: SlotId| {
            let default_target = &self[b.side][b.individual];
            if default_target.borrow().is_fainted() {
                let default_target = self.opposite_of(&attacker.side).ally_of(&attacker.individual);
                if default_target.borrow().is_fainted() {
                    None
                } else {
                    Some(default_target)
                }
            } else {
                Some(default_target)
            }
        };
        let user = || &self[attacker.side][attacker.individual];
        let ally = || {
            match self.user {
                Side::Single(_) => None,
                Side::Double(_, _) | Side::Tag(_, _) => {
                    let ally = self[attacker.side].ally_of(&attacker.individual);
                    if ally.borrow().is_fainted() {
                        Some(ally)
                    } else {
                        None
                    }
                }
            }
        };

        let move_data = attack.data();

        match move_data.target {
            Target::User | Target::Implicit => vec![user()],
            Target::Ally => {
                match ally() {
                    None => vec![],
                    Some(a) => vec![a]
                }
            },
            Target::UserAndAlly => {
                match ally() {
                    None => vec![user()],
                    Some(a) => vec![user(), a]
                }
            },
            Target::UserOrAlly => {
                match selected {
                    SelectedTarget::Implied => vec![user()],
                    SelectedTarget::One(b) => {
                        let pkmn = &self[b.side][b.individual];
                        if pkmn.borrow().is_fainted() { vec![] } else { vec![pkmn] }
                    }
                }
            },
            Target::Opponent => {
                match selected {
                    SelectedTarget::Implied => {
                        match implied_opponent() {
                            None => vec![],
                            Some(a) => vec![a]
                        }
                    },
                    SelectedTarget::One(b) => {
                        match unimplied_opponent(b) {
                            None => vec![],
                            Some(a) => vec![a]
                        }
                    }
                }
            },
            Target::Opponents => {
                self.opposite_of(&attacker.side).both()
                    .iter()
                    .map(|p| *p)
                    .filter(|p| p.borrow().has_health())
                    .collect()
            },
            Target::AllyOrOpponent => {
                match selected {
                    SelectedTarget::Implied => {
                        match implied_opponent() {
                            None => vec![],
                            Some(a) => vec![a]
                        }
                    }
                    SelectedTarget::One(b) => {
                        if attacker.is_ally(&b) {
                            match ally() {
                                None => vec![],
                                Some(a) => vec![a]
                            }
                        } else {
                            match unimplied_opponent(b) {
                                None => vec![],
                                Some(a) => vec![a]
                            }
                        }
                    }
                }
            },
            Target::RandomOpponent => {
                match self.opposite_of(&attacker.side) {
                    Side::Single(p) => {
                        if p.borrow().is_fainted() { vec![] } else { vec![p] }
                    },
                    Side::Double(a, b) |
                    Side::Tag(a, b) => {
                        let a_fainted = a.borrow().is_fainted();
                        let b_fainted = b.borrow().is_fainted();
                        if a_fainted && b_fainted {
                            vec![]
                        } else if a_fainted {
                            vec![b]
                        } else if b_fainted {
                            vec![a]
                        } else {
                            vec![if rand::thread_rng().gen_bool(0.5) { a } else { b }]
                        }
                    }
                }
            }
            Target::Any => {
                match selected {
                    SelectedTarget::Implied => {
                        match implied_opponent() {
                            None => vec![],
                            Some(a) => vec![a]
                        }
                    }
                    SelectedTarget::One(a) => {
                        if attacker == a {
                            vec![user()]
                        } else if attacker.is_ally(&a) {
                            match ally() {
                                None => vec![],
                                Some(a) => vec![a]
                            }
                        } else {
                            match unimplied_opponent(a) {
                                None => vec![],
                                Some(a) => vec![a]
                            }
                        }
                    }
                }
            },
            Target::AllExceptUser => {
                let mut targets: Vec<&Slot> = self.opposite_of(&attacker.side).both()
                    .iter()
                    .map(|p| *p)
                    .filter(|p| p.borrow().has_health())
                    .collect();
                match ally() {
                    None => {}
                    Some(a) => targets.push(a)
                }
                targets
            },
            Target::All => {
                let mut targets: Vec<&Slot> = self[attacker.side].both()
                    .iter()
                    .map(|p| *p)
                    .filter(|a| a.borrow().has_health())
                    .collect();
                let mut opponent_side: Vec<&Slot> = self.opposite_of(&attacker.side).both()
                    .iter()
                    .map(|p| *p)
                    .filter(|a| a.borrow().has_health())
                    .collect();
                targets.append(&mut opponent_side);
                targets
            }
            Target::LastAttacker(pred) => {
                let u = user();
                let data = u.data.borrow();
                let target = match pred {
                    None => data.damage_this_turn
                        .iter()
                        .filter(|(battler, _, _)| {
                            let potential_target = &self[battler.side][battler.individual];
                            potential_target.borrow().has_health()
                        })
                        .last()
                    ,
                    Some(dt) => {
                        data.damage_this_turn
                            .iter()
                            .filter(|(_, attack, _)| attack.data().damage_type == dt)
                            .filter(|(battler, _, _)| {
                                let potential_target = &self[battler.side][battler.individual];
                                potential_target.borrow().has_health()
                            })
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
    fn get_side(&self, id: &SlotId) -> &RefCell<FieldSide> {
        match id.side {
            BattleSideId::Forward => &self.user_side,
            BattleSideId::Back => &self.opponent_side
        }
    }

    fn get_everyone(&self) -> Vec<&Slot> {
        let mut active = self.user.both();
        active.append(&mut self.opponent.both());
        active
    }
}
impl Index<BattleSideId> for Battlefield {
    type Output = Side;

    fn index(&self, index: BattleSideId) -> &Self::Output {
        match index {
            BattleSideId::Forward => &self.user,
            BattleSideId::Back => &self.opponent
        }
    }
}
impl IndexMut<BattleSideId> for Battlefield {
    fn index_mut(&mut self, index: BattleSideId) -> &mut Self::Output {
        match index {
            BattleSideId::Forward => &mut self.user,
            BattleSideId::Back => &mut self.opponent
        }
    }
}

/// Really, anything that needs to be tracked during the course of the battle
/// When the pokemon is switched out, everything here is reset to defaults
#[derive(Default, Debug)]
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
    damage_this_turn: Vec<(SlotId, Move, u16)>,
    /// If present, the user is biding (turns left, damage accumulated)
    bide: Option<(u8, u16)>,
    /// Number of turns poisoned. 0 indicates not badly poison
    poison_counter: u8,
    /// The Pokemon that have targeted this Pokemon, in order. Index 0 == Most recent, For Mirror Move (not reset at end of round)
    last_targeted: Vec<(SlotId, Move)>,
    /// The last person who attacked the Pokemon (not reset at end of round)
    last_attacker: Option<(SlotId, Move)>,

    attack_stage: i8,
    defense_stage: i8,
    special_attack_stage: i8,
    special_defense_stage: i8,
    speed_stage: i8,
    accuracy_stage: i8,
    evasion_stage: i8,
    crit_stage: u8,

    /// Manipulated stats (Power Trick, Power Split, Guard Split)
    temp_attack: Option<u16>,
    temp_defense: Option<u16>,
    temp_special_attack: Option<u16>,
    temp_special_defense: Option<u16>,

    /// Manipulated moves (mimic)
    temp_move_1: Option<MoveSlot>,
    temp_move_2: Option<MoveSlot>,
    temp_move_3: Option<MoveSlot>,
    temp_move_4: Option<MoveSlot>,

    /// If present, the turns remaining bound, and whether a Binding Band was used
    bound: Option<(u8, bool)>,
    /// Turns remaining where this Pokemon is confused (0 == no confusion)
    confused: u8,
    /// If true, this Pokemon is infatuated
    infatuated: bool,
    /// If true, this Pokemon will flinch instead of attack this turn
    flinch: bool,
    /// If present, this Pokemon is charging, and will use Move on the next turn
    charging: Option<(SelectedTarget, Move)>,
    /// If true, this Pokemon is resting from a previously-used move
    recharge: bool,
    /// If present, this Pokemon is thrashing.
    thrashing: Option<(Move, u8)>,
    /// A list of disabled moves, and the amount of time left before they are enabled
    disabled: Vec<(Move, u8)>,
    /// If present, this Pokemon is seeded by the contained Pokemon
    seeded: Option<SlotId>,
    /// If true, this Pokemon's rage is building (attack increases if hit by attack)
    enraged: bool,
    /// If true, this Pokemon is minimized (certain moves hit for double damage + 100% accuracy)
    minimized: bool,
    /// If true, this Pokemon has curled (Rollout does more damage)
    curled: bool,
    /// If true, this Pokemon cannot escape
    trapped: bool,
    /// If present, the user is locked-on to that Battler for the specified number of turns
    locked_on: Option<(u8, SlotId)>,
    /// If true, this Pokemon is having a nightmare
    nightmare: bool,
    /// If true, this Pokemon has been cursed
    cursed: bool,
    /// If present, the form of Protection this Pokemon has
    protected: Option<Move>,
    /// The number of subsequent times Protect was used.
    /// Global counter for all protection moves; only resets when a non-protection move is used
    protection_counter: u8,
    /// If present, and the attacker is the Pokemon in SlotID, and the defender is this Pokemon, Evasion is ignored if >0
    foresight_by: Option<SlotId>,
    /// If true, this user is Destiny Bond'ed. If a Pokemon KOs this one, it will faint as well.
    destiny_bond: bool,
    /// If > 0, Perish Song is in effect. Death happens on 1 -> 0
    perish_song_counter: u8,
    /// If true, this user is rooted
    rooted: bool,
    /// If present, this user is using a multi-turn move (Rollout or Ice Ball). u8 is the # turns left
    rolling: Option<(Move, u8)>,
    /// If >0, levitating. Decrement after each turn
    levitating: u8,
    /// If present, the Pokemon is semi-invulnerable
    invulnerable: Option<SemiInvulnerableLocation>,
    /// The Pokemon has a substitute out, with this much HP remaining
    substituted: u16,
    /// This Pokemon is transformed into another.
    transformed: Option<TransformData>,
    /// If present, this Pokemon has this ability instead of its usual one
    temp_ability: Option<Ability>,
    /// If present, this Pokemon has this type instead of its usual one
    temp_type: Option<PokemonType>,
    /// If present, this Pokemon has this weight instead of its usual weight
    temp_weight: Option<u16>,
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
            BattleStat::Evasion => self.evasion_stage,
            BattleStat::CriticalHitRatio => i8::try_from(self.crit_stage).unwrap()
        }
    }

    pub fn add_stage(&mut self, stat: BattleStat, stages: i8) -> bool {
        let stage = match stat {
            BattleStat::Attack => &mut self.attack_stage,
            BattleStat::Defense => &mut self.defense_stage,
            BattleStat::SpecialAttack => &mut self.special_attack_stage,
            BattleStat::SpecialDefense => &mut self.special_defense_stage,
            BattleStat::Speed => &mut self.speed_stage,
            BattleStat::Accuracy => &mut self.accuracy_stage,
            BattleStat::Evasion => &mut self.evasion_stage,
            BattleStat::CriticalHitRatio => {
                let crit_stage = i8::try_from(self.crit_stage).unwrap();
                let new_stage = crit_stage + stages;
                if new_stage < 0 {
                    return false;
                } else {
                    self.crit_stage = u8::try_from(new_stage).unwrap();
                    return true;
                }
            }
        };

        let new_stage = *stage + stages;
        if new_stage < MIN_STAGE || new_stage > MAX_STAGE {
            false
        } else {
            *stage = new_stage;
            true
        }
    }

    pub fn set_last_used_move(&mut self, attack: Move) {
        if let Some(a) = self.last_move_used {
            if a == attack {
                self.last_move_used_counter = self.last_move_used_counter.saturating_add(1);
            } else {
                self.last_move_used = Some(attack);
                self.last_move_used_counter = 1;
            }
        } else {
            self.last_move_used = Some(attack);
            self.last_move_used_counter = 1;
        }
    }

    /// Check if a move is disabled
    pub fn is_disabled(&self, attack: Move) -> bool {
        self.disabled
            .iter()
            .any(|(a, _)| *a == attack)
    }

    /// Drop all disable counters by 1, removing and returning the ones that are no longer disabled
    pub fn lower_disable_counters(&mut self) -> Vec<Move> {
        let mut disabled = Vec::new();
        let mut undisabled = Vec::new();
        for (m, c) in self.disabled.iter() {
            if *c == 1 {
                undisabled.push(*m);
            } else {
                disabled.push((*m, *c - 1));
            }
        }
        self.disabled = disabled;
        undisabled
    }

    pub fn targeted_by(&mut self, pkmn: &Slot, attack: Move) {
        self.remove_from_targeted_by(pkmn);
        self.last_targeted.insert(0, (pkmn.id, attack));
    }

    fn remove_from_targeted_by(&mut self, pkmn: &Slot) {
        self.last_targeted.retain(|(e, _)| *e != pkmn.id);
    }

    pub fn get_last_targeted_attack(&self) -> Option<(SlotId, Move)> {
        self.last_targeted.first()
            .map(|(battler, attack)| (*battler, *attack))
    }

    pub fn is_locked_on_to(&self, pkmn: &Slot) -> bool {
        if let Some((_, battler)) = self.locked_on {
            pkmn.id == battler
        } else {
            false
        }
    }

    /// This hook is called when another Pokemon on the field swaps out
    pub fn other_swapped_out(&mut self, pkmn: &Slot) {
        self.remove_from_targeted_by(pkmn);
        if let Some(b) = self.foresight_by {
            if b == pkmn.id {
                self.foresight_by = None;
            }
        }
    }

    /// This hook is called just before this Pokemon's turn starts
    pub fn start_of_turn(&mut self) {
        self.destiny_bond = false;
    }

    /// This hook is called when the round ends
    pub fn end_of_round(&mut self) {
        self.turn_count += 1;
        self.damage_this_turn = Vec::new();
        self.has_acted_this_turn = false;
        self.has_landed_attack_this_turn = false;
        self.flinch = false;
        self.protected = None;
        if self.poison_counter > 0 {
            self.poison_counter = self.poison_counter.saturating_add(1);
        }

        if let Some((counter, battler)) = self.locked_on {
            let counter = counter - 1;
            if counter == 0 {
                self.locked_on = None;
            } else {
                self.locked_on = Some((counter, battler));
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BattleSideId {
    Forward,
    Back
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DoubleBattleSideId {
    Left, Right
}

/// Identifier of a member on the field (more specifically, a "place" on the battlefield)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SlotId {
    pub side: BattleSideId,
    pub individual: DoubleBattleSideId
}
impl SlotId {
    pub fn single(side: BattleSideId) -> SlotId {
        SlotId {
            side,
            individual: DoubleBattleSideId::Left
        }
    }

    pub fn double(side: BattleSideId, side2: DoubleBattleSideId) -> SlotId {
        SlotId {
            side,
            individual: side2
        }
    }

    /// Test if other battler and this battler are allies
    /// Note that this returns false if self == other!
    pub fn is_ally(&self, other: &SlotId) -> bool {
        self.side == other.side && self.individual != other.individual
    }

    /// Test if other battler and this battler are either the same, or an ally
    pub fn is_self_or_ally(&self, other: &SlotId) -> bool {
        self.side == other.side
    }
}

/// Temporary data for when transforming into a Pokemon.
#[derive(Debug)]
struct TransformData {
    species: Species,
    gender: Gender,
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
impl TransformData {
    fn from(p: &Slot) -> Self {
        let p = p.borrow();
        TransformData {
            species: p.species,
            gender: p.gender,
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
enum WeatherCounter {
    Rain(u8),
    Sun(u8),
    Hail(u8),
    Sandstorm(u8),
    Fog
}

/// The cause of some particular action's side effect
#[derive(Debug, Clone)]
pub enum Cause {
    /// This is just what normally happens
    Natural,
    /// A battler's ability caused the side effect
    Ability(SlotId, Ability),
    /// A used move caused the side effect (this is things that happen directly during the move)
    Move(SlotId, Move),
    /// A used move caused the side effect (this is things that happen later, such as Curse at the end of each turn)
    MoveSideEffect(Move),
    /// The cause is related to the Pokemon's type
    Type(Type),
    /// The side effect was the cause of a user's non-volatile ailment
    Ailment(NonVolatileBattleAilment),
    /// A battler's held item caused the side effect
    HeldItem(SlotId, Item),
    /// One cause was about to occur, but another one overwrote it
    Overwrite{
        initial: Box<Cause>,
        overwriter: Box<Cause>
    },
    /// Failed because of something related to the Pokemon's current battle state
    PokemonBattleState(SlotId, PokemonState),
    /// Failed because of something on the field
    PokemonFieldState(FieldCause)
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
pub enum PokemonState {
    Confused,
    StatsMaxed(StatsCause),
    Enraged,
    Substituted,
    TooWeak,
    HoldingItem, NotHoldingItem
}

#[derive(Debug, Clone)]
pub enum FieldCause {
    Mist,
    Spikes,
    ToxicSpikes,
    StealthRock,
    Weather(Weather)
}

/// Possible consequences of an Action
/// Plan is to use these to determine which text boxes to say.
#[derive(Debug)]
pub enum ActionSideEffects {
    DirectDamage {
        damaged: SlotId,
        damager: SlotId,
        attack: Move,
        start_hp: u16,
        end_hp: u16,
        critical_hit: bool,
        effectiveness: Effectiveness
    },
    Recoil {
        damaged: SlotId,
        start_hp: u16,
        end_hp: u16,
        cause: Cause
    },
    Crash {
        damaged: SlotId,
        start_hp: u16,
        end_hp: u16
    },
    BasicDamage {
        damaged: SlotId,
        start_hp: u16,
        end_hp: u16,
        cause: Cause
    },
    HungOn (SlotId, Cause),
    Healed {
        healed: SlotId,
        start_hp: u16,
        end_hp: u16,
        cause: Cause
    },
    Missed(SlotId, Cause),
    NoEffect(Cause),
    Failed(Cause),
    MultiStrike {
        actions: Vec<Vec<ActionSideEffects>>,
        hits: u8
    },
    CreatedSubstitute(SlotId), DamagedSubstitute {
        damaged: SlotId,
        start_hp: u16,
        end_hp: u16
    },
    ConsumedItem(SlotId, Item), GainedItem(SlotId, Item),
    PokemonLeft(SlotId, usize), PokemonEntered(SlotId, usize),
    LostPP(SlotId, Move, u8, u8),
    NoTarget,

    NoEffectSecondary(Cause),
    StatChanged {
        affected: SlotId,
        stat: BattleStat,
        cause: Cause,
        start: i8,
        end: i8
    },
    StatsReset(SlotId),
    StatMaxed(SlotId, BattleStat),
    NonVolatileStatusAilment {
        affected: SlotId,
        status: NonVolatileBattleAilment
    },
    WasFrozen(SlotId), Thawed(SlotId),
    Sleep(SlotId), WokeUp(SlotId),
    IsFullyParalyzed(SlotId),
    Confuse(SlotId), ConfusionTurn(SlotId),
    HurtInConfusion {
        affected: SlotId,
        start: u16,
        end: u16,
        hang_on_cause: Option<Cause>
    },
    SnappedOutOfConfusion(SlotId),
    Infatuated(SlotId), TooInfatuatedToAttack(SlotId),
    ForcePokemonSwap {
        must_leave: SlotId
    },
    StartWeather(glazed_data::attack::Weather), ContinueWeather(glazed_data::attack::Weather), EndWeather(glazed_data::attack::Weather),
    DroppedCoins,
    Charging(SlotId, Move),
    Recharging(Cause),
    Bound {
        binder: SlotId,
        bound: SlotId,
        turns: u8,
        attack: Move
    },
    HurtByBound {
        bound: SlotId,
        start_hp: u16,
        end_hp: u16
    },
    Unbound(SlotId),
    WillFlinch(SlotId), Flinched(SlotId),
    Disabled(SlotId, Move), NoLongerDisabled(SlotId, Move),
    MistStart(BattleSideId), MistEnd(BattleSideId, Cause),
    SeedStart {
        from: SlotId,
        to: SlotId
    },
    SeedLeech {
        from: SlotId,
        to: SlotId,
        damage: u8
    },
    RageStart(SlotId), RageContinue(SlotId), RageEnd(SlotId),
    Mimicked(SlotId, Move),
    ScreenStart(BattleSideId, ScreenType), ScreenEnd(BattleSideId, ScreenType),
    BideStart(SlotId), BideContinue(SlotId),
    Metronome(SlotId, Move),
    Transform {
        id: SlotId,
        species: Species,
        gender: Gender,
        shiny: bool
    },
    ChangeType(SlotId, Type), ChangeAbility(SlotId, Ability),
    Sketched {
        user: SlotId,
        target: SlotId,
        attack: Move
    },
    StoleItem {
        from: SlotId,
        to: SlotId,
        item: Item
    }, CouldntStealItem {
        from: SlotId,
        to: SlotId,
        cause: Cause
    },
    TrappedStart(SlotId),
    LockedOn {
        user: SlotId,
        target: SlotId
    },
    Foresighted {
        user: SlotId,
        target: SlotId
    },
    Nightmare(SlotId),
    Curse(SlotId),
    DestinyBond(SlotId),
    StartPerishSong, PerishCount(SlotId, u8),
    StartProtection(SlotId, Move), IsProtected(SlotId, Move),
    EntryHazard(BattleSideId, Move, u8),
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

    pub fn did_damage(&self) -> bool {
        match self {
            ActionSideEffects::BasicDamage {..} | ActionSideEffects::DirectDamage {..} => true,
            _ => false
        }
    }
}