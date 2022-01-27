#![feature(hash_drain_filter)]

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::read;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::option::Option::Some;
use std::rc::Rc;

use rand::Rng;

use glazed_data::abilities::{Ability, PokemonAbility};
use glazed_data::attack::{BattleStat, Move, NonVolatileBattleAilment, PoisonType, ScreenType, SemiInvulnerableLocation, Target, Weather};
use glazed_data::attack::BattleStat::Defense;
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
mod hooks;

/// The target selected in a multi-battle
/// All targets for a move are well-defined, except in the case of moves that target one opponent
/// in a double-battle.
/// Implied should *always* provide at least one target, to the best of its ability.
#[derive(Debug, Copy, Clone)]
pub enum SelectedTarget {
    /// The engine will do its best to determine a valid target.
    Implied,
    /// One specific Pokemon is the target.
    One(usize)
}

/// Represents the effects of the entire battlefield
#[derive(Default, Debug)]
pub struct Field {
    weather: Option<WeatherCounter>,
    gravity: u8,
    magic_room: u8,
    coins_on_ground: u16,
    future_attacks: HashMap<SlotId, (PokemonId, Move, u8)>
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

    /// Returns if there is fog on the field
    pub fn is_foggy(&self) -> bool {
        match self.weather {
            Some(WeatherCounter::Fog) => true,
            _ => false
        }
    }

    /// Returns if no weather is present
    pub fn is_clear(&self) -> bool {
        match self.weather {
            None => true,
            _ => false
        }
    }

    /// Scatter coins on the field.
    pub fn drop_coins(&mut self, coins: u16) {
        self.coins_on_ground = self.coins_on_ground.saturating_add(coins);
    }

    /// Check if this Slot will receive a future attack eventually
    pub fn will_receive_future_attack(&self, slot: SlotId) -> bool {
        return self.future_attacks.contains_key(&slot)
    }

    /// Add a future attack to be executed eventually
    pub fn add_future_attack(&mut self, origin: PokemonId, attack: Move, turns: u8, recipient: SlotId) {
        self.future_attacks.insert(recipient, (origin, attack, turns));
    }

    /// Decrement future attack counters, and return any ready to go.
    pub fn decrement_future_attack_counters(&mut self) -> Vec<(PokemonId, Move, SlotId)> {
        self.future_attacks
            .drain_filter(|k, (_, _, turns)| {
                if *turns == 1 {
                    true
                } else {
                    *turns -= 1;
                    false
                }
            })
            .flat_map(|(k, (origin, attack, _))| {
                vec![(origin, attack, k)]
            })
            .collect()
    }
}

/// Represents one side of a battlefield
#[derive(Debug, Default)]
pub struct FieldSide {
    spikes: u8,
    toxic_spikes: u8,
    pointed_stones: bool,
    tailwind: u8,
    aurora_veil: u8,
    light_screen: u8,
    reflect: u8,
    mist: u8,
    safeguard: u8
}

/// A group of between 1 and 6 Pokemon, which a trainer owns
#[derive(Debug)]
pub struct Party {
    pub one: Option<RefCell<Pokemon>>,
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
            .take(6)
            .collect::<Vec<Pokemon>>();
        let size = party.len();
        let mut party = party.drain(..);
        Party {
            one: party.next().map(|i| RefCell::from(i)),
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
            one: Some(RefCell::from(one.into())),
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
impl Index<usize> for Party {
    type Output = Option<RefCell<Pokemon>>;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.one,
            1 => &self.two,
            2 => &self.three,
            3 => &self.four,
            4 => &self.five,
            5 => &self.six,
            _ => panic!("Unknown pokemon index")
        }
    }
}
impl IndexMut<usize> for Party {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.one,
            1 => &mut self.two,
            2 => &mut self.three,
            3 => &mut self.four,
            4 => &mut self.five,
            5 => &mut self.six,
            _ => panic!("Unknown pokemon index")
        }
    }
}

/// Represents an individual Pokemon somewhere in the battle.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PokemonId {
    pub party_id: PartyId,
    pub party_slot_id: PartyMemberId
}

/// Represents one of the parties in battle
/// (In most battles, Player is 0, Opponent is 1)
/// There may be more parties if the opponent is in a tag battle, a 1v2 battle, or a free-for-all.
pub type PartyId = usize;
/// Represents a Pokemon's ID within a party
/// 0 to 5
pub type PartyMemberId = usize;
/// A specific, targettable place in battle
/// (In a single battle, Player is 0, Opponent is 1.
/// In a double battle, Player is 0 and 1, Opponents are 2 and 3)
pub type SlotId = usize;
/// A specific side of the battle (Player is 0, Opponent is 1)
pub type BattleSideId = usize;

trait BaseSlot {
    fn is_not_active(&self) -> bool;
    fn id(&self) -> PokemonId {
        PokemonId {
            party_id: self.party_id(),
            party_slot_id: self.party_slot_id()
        }
    }
    fn party_id(&self) -> PartyId;
    fn party_slot_id(&self) -> PartyMemberId;
    fn pokemon(&self, idx: PartyMemberId) -> Option<&RefCell<Pokemon>>;
    fn active_pokemon(&self) -> &RefCell<Pokemon> {
        self.pokemon(self.party_slot_id()).unwrap()
    }
    fn data(&self) -> Ref<BattleData>;

    /// Return true if every member of the party has no HP
    fn is_everyone_koed(&self) -> bool {
        for idx in 0..6 {
            let pkmn = self.pokemon(idx);
            match pkmn {
                Some(p) if p.borrow().has_health() => {
                    return false;
                },
                _ => {}
            }
        }
        true
    }

    /// Return true if any member of the party, aside from the active Pokemon, has HP
    fn has_anyone_to_swap_to(&self) -> bool {
        for idx in 0..6 {
            if idx != self.party_slot_id() {
                let pkmn = &self.pokemon(idx);
                match pkmn {
                    Some(p) if p.borrow().has_health() => {
                        return true;
                    },
                    _ => {}
                }
            }
        }
        false
    }

    /// Get the effective species of this Pokemon. Takes Transform into effect
    fn get_effective_species(&self) -> Species {
        match &self.data().transformed {
            None => self.active_pokemon().borrow().species,
            Some(t) => t.species
        }
    }

    /// Get the effective ability of this Pokemon. Takes Transform and temporary Ability changes into effect
    fn get_effective_ability(&self) -> Ability {
        self.data().temp_ability.unwrap_or_else(|| {
            match &self.data().transformed {
                None => self.active_pokemon().borrow().get_ability(),
                Some(t) => *t.get_ability()
            }
        })
    }

    /// Get the effective type(s) of this Pokemon. Takes Transform and temporary Type changes into effect
    fn get_effective_type(&self) -> PokemonType {
        self.data().temp_type.unwrap_or_else(|| {
            match &self.data().transformed {
                Some(t) => t.species.data()._type,
                None => self.active_pokemon().borrow().species.data()._type,
            }
        })
    }

    /// Get the effective gender of this Pokemon. Takes Transform into effect
    fn get_effective_gender(&self) -> Gender {
        self.data().transformed
            .as_ref()
            .map(|t| t.gender)
            .unwrap_or_else(|| self.active_pokemon().borrow().gender)
    }

    /// Get the effective known moves of this Pokemon, taking Temporary moves and Transform into effect
    /// Priority is to temp moves, then transform, then regular known moves
    fn get_effective_known_moves(&self) -> Vec<Move> {
        let pkmn = self.active_pokemon().borrow();
        let data = self.data();
        let mut moves = Vec::new();
        if let Some(m) = data.temp_move_1
            .or_else(|| data.transformed.as_ref().and_then(|s| s.move_1))
            .or_else(|| pkmn.move_1) {
            moves.push(m.attack);
        }
        if let Some(m) = data.temp_move_2
            .or_else(|| data.transformed.as_ref().and_then(|s| s.move_2))
            .or_else(|| pkmn.move_2) {
            moves.push(m.attack);
        }
        if let Some(m) = data.temp_move_3
            .or_else(|| data.transformed.as_ref().and_then(|s| s.move_3))
            .or_else(|| pkmn.move_3) {
            moves.push(m.attack);
        }
        if let Some(m) = data.temp_move_4
            .or_else(|| data.transformed.as_ref().and_then(|s| s.move_4))
            .or_else(|| pkmn.move_4) {
            moves.push(m.attack);
        }

        moves
    }

    fn get_stat_stage(&self, stat: BattleStat) -> i8 {
        let data = self.data();
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
    fn get_effective_stat(&self, stat: BattleStat) -> u16 {
        let pkmn = self.active_pokemon().borrow();
        let data = self.data();
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
    fn get_raw_critical_hit(&self) -> u8 {
        let mut value = self.data().crit_stage;
        value += match self.active_pokemon().borrow().held_item {
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
    fn get_effective_weight(&self) -> u16 {
        let mut weight = self.data().temp_weight.unwrap_or_else(|| self.get_effective_species().data().weight);
        let ability = self.get_effective_ability();

        if ability == Ability::HeavyMetal {
            weight = weight.saturating_mul(2);
        } else if ability == Ability::LightMetal {
            weight = weight / 2;
        }

        if let Some(Item::FloatStone) = self.active_pokemon().borrow().held_item {
            weight = weight/ 2;
        }

        if weight == 0 { 1 } else { weight }
    }

    fn get_effective_move(&self, slot: usize) -> Option<MoveSlot> {
        let pkmn = self.active_pokemon().borrow();
        let data = self.data();
        match slot {
            0 => data.temp_move_1.or(pkmn.move_1),
            1 => data.temp_move_2.or(pkmn.move_2),
            2 => data.temp_move_3.or(pkmn.move_3),
            _ => data.temp_move_4.or(pkmn.move_4)
        }
    }

    fn is_grounded(&self) -> (bool, Cause) {
        if let Some(Item::IronBall) = self.active_pokemon().borrow().held_item {
            return (true, Cause::HeldItem(self.id(), Item::IronBall))
        }
        (false, Cause::Natural)
    }

    fn is_behind_substitute(&self) -> bool {
        self.data().substituted > 0
    }
}

/// Smart pointer that points to the current active Pokemon
#[derive(Debug, Clone)]
pub struct Slot {
    pub slot_id: SlotId,
    pub party_id: PartyId,
    pub party_slot_id: PartyMemberId,
    party: Rc<Party>,
    pub data: Rc<RefCell<BattleData>>,
}
impl BaseSlot for Slot {
    fn is_not_active(&self) -> bool {
        self.slot_id == SlotId::MAX
    }

    fn party_id(&self) -> PartyId {
        self.party_id
    }

    fn party_slot_id(&self) -> PartyMemberId {
        self.party_slot_id
    }

    fn pokemon(&self, idx: PartyMemberId) -> Option<&RefCell<Pokemon>> {
        self.party.index(idx).as_ref()
    }

    fn data(&self) -> Ref<BattleData> {
        self.data.borrow()
    }
}
impl Deref for Slot {
    type Target = RefCell<Pokemon>;

    fn deref(&self) -> &Self::Target {
        self.party[self.party_slot_id].as_ref().unwrap()
    }
}

impl Slot {
    pub fn lower_hp(&mut self, amount: u16) -> Vec<ActionSideEffects> {
        let start_hp = self.borrow().current_hp;
        let end_hp = start_hp.saturating_sub(amount);
        self.borrow_mut().current_hp = end_hp;
        vec![ActionSideEffects::BasicDamage {
            damaged: self.slot_id,
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
            damaged: self.slot_id,
            start_hp,
            end_hp
        }
    }

    pub fn take_poison_damage(&self) -> ActionSideEffects {
        let mut pkmn = self.borrow_mut();
        let data = self.data.borrow_mut();
        if data.poison_counter > 0 {
            let mut d = pkmn.hp.value / 16;
            d *= data.poison_counter as u16;

            let start_hp = pkmn.current_hp;
            let end_hp = start_hp.saturating_sub(d);
            pkmn.current_hp = end_hp;

            ActionSideEffects::BasicDamage {
                damaged: self.slot_id,
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
                damaged: self.slot_id,
                start_hp,
                end_hp,
                cause: Cause::Ailment(NonVolatileBattleAilment::Poison(PoisonType::Poison))
            }
        }
    }

    pub fn consume_item(&mut self) -> Option<ActionSideEffects> {
        let mut pkmn = self.borrow_mut();
        if let Some(i) = &pkmn.held_item {
            let i = i.clone();
            pkmn.held_item = None;
            Some(ActionSideEffects::ConsumedItem(self.slot_id, i))
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

/// Represents the current battlefield
#[derive(Debug)]
pub struct Battlefield {
    sides: Vec<RefCell<FieldSide>>,
    parties: Vec<(BattleSideId, Rc<Party>)>, //(side_index, party)
    active: Vec<(PartyId, PartyMemberId, Rc<RefCell<BattleData>>)>, //(party_index, active slot, data)
    field: RefCell<Field>,
    wild_battle: bool
}
impl Battlefield {
    // Hierarchy:
    // Each battle has X sides
    // Each Side has Y Parties
    // Each Party has Z Active Pokemon
    // Typical Values:
    // Single Battle:   X=2, Y=1, Z=1
    // Double Battle:   X=2, Y=1, Z=2
    // Triple Battle:   X=2, Y=1, Z=3
    // Tag Battle:      X=2, Y=2, Z=1
    // 1-on-2:          X=2, Y=1/2, Z=2/1
    // Free For All:    X=4, Y=1, Z=1

    // Side Lookups
    /// Get all sides
    pub fn get_sides(&self) -> &Vec<RefCell<FieldSide>> {
        &self.sides
    }

    pub fn get_sides_with_id(&self) -> Vec<(BattleSideId, &RefCell<FieldSide>)> {
        self.sides
            .iter()
            .enumerate()
            .collect()
    }

    /// Given a Side ID, return the Side
    pub fn get_side_by_side_id(&self, side: BattleSideId) -> &RefCell<FieldSide> { self.sides.get(side).unwrap() }

    /// Given a Party ID, return the Side
    pub fn get_side_by_party_id(&self, id: PartyId) -> &RefCell<FieldSide> {
        let (side_id, _) = self.parties.get(id).unwrap();
        self.get_side_by_side_id(*side_id)
    }

    /// Given a Party ID, return the Side
    pub fn get_side_id_by_party_id(&self, id: PartyId) -> BattleSideId {
        let (side_id, _) = self.parties.get(id).unwrap();
        *side_id
    }

    /// Given an Active ID, return the Side
    pub fn get_side_by_active_id(&self, id: SlotId) -> &RefCell<FieldSide> {
        let (party_id, _, _) = self.active.get(id).unwrap();
        self.get_side_by_party_id(*party_id)
    }

    // Party Lookups
    /// Given a Side ID, return all Partys on that side
    pub fn get_parties_by_side_id(&self, side: BattleSideId) -> Vec<Rc<Party>> {
        self.parties.iter()
            .filter_map(|(side_id, party)| {
                if side == *side_id {
                    Some(party.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Given a Party ID, return the Party
    pub fn get_party_by_party_id(&self, party: PartyId) -> Rc<Party> {
        let (_, p) = self.parties.get(party).unwrap();
        p.clone()
    }

    /// Given an Active ID, return the Party
    pub fn get_party_by_active_id(&self, id: SlotId) -> Rc<Party> {
        let (party_id, _, _) = self.active.get(id).unwrap();
        self.get_party_by_party_id(*party_id)
    }

    // Active Pokemon lookups
    /// Given a Side ID, return all Active Pokemon on that side
    pub fn get_active_pokemon_by_side_id(&self, id: BattleSideId) -> Vec<Slot> {
        self.parties.iter()
            .enumerate()
            .filter_map(|(party_idx, (side_idx, party))| {
                if *side_idx == id {
                    Some(party_idx)
                } else {
                    None
                }
            })
            .flat_map(|party_idx| {
                self.get_active_pokemon_by_party_id(party_idx)
            })
            .collect()
    }

    /// Given a Party ID, return all Active Pokemon in the party
    pub fn get_active_pokemon_by_party_id(&self, id: PartyId) -> Vec<Slot> {
        let party = self.get_party_by_party_id(id);
        self.active.iter()
            .enumerate()
            .filter_map(|(active_id, (party_id, slot_id, data))| {
                if *party_id == id {
                    Some(Slot {
                        slot_id: active_id,
                        party_id: *party_id,
                        party_slot_id: *slot_id,
                        party: party.clone(),
                        data: data.clone()
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Given a Active ID, return the Active Pokemon on that side
    pub fn get_active_pokemon_by_active_id(&self, id: SlotId) -> Slot {
        let (party_id, party_slot_id, data) = self.active.get(id).unwrap();
        let party = self.get_party_by_party_id(*party_id);
        Slot {
            slot_id: id,
            party_id: *party_id,
            party_slot_id: *party_slot_id,
            party: party.clone(),
            data: data.clone()
        }
    }

    /// Given an Active Pokemon ID, return the Active Pokemon directly opposite
    pub fn get_opposite_slots(&self, id: SlotId) -> Vec<SlotId> {
        if self.active.len() == 2 {
            match id {
                0 => vec![1],
                1 => vec![0],
                _ => panic!("Unknown slot {:}", id)
            }
        } else {
            match id {
                0 => vec![2, 3],
                1 => vec![3, 2],
                2 => vec![0, 1],
                3 => vec![1, 0],
                _ => panic!("Unknown slot {:}", id)
            }
        }
    }

    /// Given an Active Pokemon ID, return the Active Pokemon on the opposite side, starting with first_choice
    pub fn get_opposite_slots_starting_with(&self, id: SlotId, first_choice: SlotId) -> Vec<SlotId> {
        if self.active.len() == 2 {
            match id {
                0 => vec![1],
                1 => vec![0],
                _ => panic!("Unknown slot {:}", id)
            }
        } else {
            match id {
                0 | 1 => if first_choice == 2 { vec![2, 3] } else { vec![3, 2] },
                2 | 3 => if first_choice == 0 { vec![0, 1] } else { vec![1, 0] },
                _ => panic!("Unknown slot {:}", id)
            }
        }
    }

    /// Check if two slots are allies (not on opposite sides, and not the same Pokemon)
    pub fn is_ally(&self, id: SlotId, id2: SlotId) -> bool {
        if self.active.len() == 2 {
            false // No allies for a single battle
        } else {
            match (id, id2) {
                (0, 1) | (1, 0) | (2, 3) | (3, 2) => true,
                _ => false
            }
        }
    }

    /// Given an Active Pokemon ID, return all Active Pokemon on the other sides
    pub fn get_active_opponents_by_active_id(&self, id: SlotId) -> Vec<Slot> {
        let (party_id, _, _) = self.active.get(id).unwrap(); // Get the party you're on
        let (side_id, _) = self.parties.get(*party_id).unwrap(); // Get the side you're on
        self.active.iter()
            .enumerate()
            .filter_map(|(idx, (candidate_party_id, candidate_party_slot, candidate_data))| {
                let (candidate_side_id, candidate_party) = self.parties.get(*candidate_party_id).unwrap();
                if *candidate_side_id != *side_id {
                    Some(Slot {
                        slot_id: idx,
                        party_id: *candidate_party_id,
                        party_slot_id: *candidate_party_slot,
                        party: candidate_party.clone(),
                        data: candidate_data.clone()
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Given an Active Pokemon ID, return all Active Pokemon on this side (including the user)
    pub fn get_active_pokemon_on_side_by_active_id(&self, id: SlotId) -> Vec<Slot> {
        let (party_id, _, _) = self.active.get(id).unwrap(); // Get the party you're on
        let (side_id, _) = self.parties.get(*party_id).unwrap(); // Get the side you're on
        self.active.iter()
            .enumerate()
            .filter_map(|(idx, (candidate_party_id, candidate_party_slot, candidate_data))| {
                let (candidate_side_id, candidate_party) = self.parties.get(*candidate_party_id).unwrap();
                if *candidate_side_id == *side_id {
                    Some(Slot {
                        slot_id: idx,
                        party_id: *candidate_party_id,
                        party_slot_id: *candidate_party_slot,
                        party: candidate_party.clone(),
                        data: candidate_data.clone()
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Given an Active Pokemon ID, return all other Active Pokemon on this side
    pub fn get_active_allies_by_active_id(&self, id: SlotId) -> Vec<Slot> {
        let (party_id, _, _) = self.active.get(id).unwrap(); // Get the party you're on
        let (side_id, _) = self.parties.get(*party_id).unwrap(); // Get the side you're on
        self.active.iter()
            .enumerate()
            .filter_map(|(candidate_idx, (candidate_party_id, candidate_party_slot, candidate_data))| {
                let (candidate_side_id, candidate_party) = self.parties.get(*candidate_party_id).unwrap();
                if *candidate_side_id == *side_id && candidate_idx != id {
                    Some(Slot {
                        slot_id: candidate_idx,
                        party_id: *candidate_party_id,
                        party_slot_id: *candidate_party_slot,
                        party: candidate_party.clone(),
                        data: candidate_data.clone()
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_everyone(&self) -> Vec<Slot> {
        self.active.iter()
            .enumerate()
            .map(|(idx, (party_id, active_id, data))| {
                Slot {
                    slot_id: idx,
                    party_id: *party_id,
                    party_slot_id: *active_id,
                    party: self.get_party_by_party_id(*party_id).clone(),
                    data: data.clone()
                }
            })
            .collect()
    }

    pub fn one_v_one<F, G>(left: F, right: G) -> Battlefield
        where F: Into<Pokemon>, G: Into<Pokemon>
    {
        Battlefield::single_battle(Party::create_one(left), Party::create_one(right))
    }

    /// Standard 1v1 battle
    /// This should only be used for trainer battles
    pub fn single_battle(player: Party, opponent: Party) -> Battlefield {
        Battlefield {
            sides: vec![RefCell::from(FieldSide::default()), RefCell::from(FieldSide::default())],
            parties: vec![(0, Rc::new(player)), (1, Rc::new(opponent))],
            active: vec![(0, 0, Rc::new(RefCell::default())), (1, 0, Rc::new(RefCell::default()))],
            field: RefCell::from(Field::default()),
            wild_battle: false,
        }
    }

    pub fn get_targets(&self, attacker: SlotId, attack: Move, selected: SelectedTarget) -> Vec<Slot> {
        let implied_opponent = || {
            self.get_opposite_slots(attacker)
                .iter()
                .map(|s| self.get_active_pokemon_by_active_id(*s))
                .filter(|s| s.borrow().has_health())
                .nth(0)
        };
        let unimplied_opponent = |b: SlotId| {
            self.get_opposite_slots_starting_with(attacker, b)
                .iter()
                .map(|s| self.get_active_pokemon_by_active_id(*s))
                .filter(|s| s.borrow().has_health())
                .nth(0)
        };
        let user = || self.get_active_pokemon_by_active_id(attacker);
        let ally = || self.get_active_allies_by_active_id(attacker)
            .iter()
            .filter(|s| s.borrow().has_health())
            .map(|s| s.clone())
            .collect::<Vec<Slot>>();

        let move_data = attack.data();

        match move_data.target {
            Target::User | Target::Implicit => vec![user()],
            Target::Ally => ally(),
            Target::UserAndAlly => {
                let mut v = vec![user()];
                v.append(&mut ally());
                v
            },
            Target::UserOrAlly => {
                match selected {
                    SelectedTarget::Implied => {
                        vec![user()]
                    }
                    SelectedTarget::One(a) => {
                        let pkmn = self.get_active_pokemon_by_active_id(a);
                        if pkmn.borrow().has_health() {
                            vec![pkmn]
                        } else {
                            vec![]
                        }
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
                self.get_active_opponents_by_active_id(attacker)
                    .iter()
                    .filter(|p| p.borrow().has_health())
                    .map(|s| s.clone())
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
                        if self.is_ally(attacker, b) {
                            ally()
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
                let e = self.get_active_opponents_by_active_id(attacker)
                    .iter()
                    .filter(|s| s.borrow().has_health())
                    .map(|s| s.clone())
                    .collect::<Vec<Slot>>();
                let opt = if e.len() > 1 {
                    e.get(rand::thread_rng().gen_range(0..e.len()))
                } else {
                    e.get(0)
                };
                match opt {
                    Some(a) => vec![a.clone()],
                    None => vec![]
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
                        } else if self.is_ally(attacker, a) {
                            ally()
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
                let mut targets = self.get_active_opponents_by_active_id(attacker);
                targets.append(&mut self.get_active_allies_by_active_id(attacker));
                targets
                    .iter()
                    .map(|p| p.clone())
                    .filter(|p| p.borrow().has_health())
                    .collect()
            },
            Target::All => {
                let mut targets: Vec<Slot> = self.get_active_opponents_by_active_id(attacker)
                    .iter()
                    .map(|p| p.clone())
                    .filter(|a| a.borrow().has_health())
                    .collect();
                let mut opponent_side: Vec<Slot> = self.get_active_pokemon_on_side_by_active_id(attacker)
                    .iter()
                    .map(|p| p.clone())
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
                            let potential_target = self.get_active_pokemon_by_active_id(*battler);
                            let potential_target = potential_target.borrow();
                            potential_target.has_health()
                        })
                        .last()
                    ,
                    Some(dt) => {
                        data.damage_this_turn
                            .iter()
                            .filter(|(_, attack, _)| attack.data().damage_type == dt)
                            .filter(|(battler, _, _)| {
                                let potential_target = self.get_active_pokemon_by_active_id(*battler);
                                let potential_target = potential_target.borrow();
                                potential_target.has_health()
                            })
                            .last()
                    }
                };
                match target {
                    None => vec![],
                    Some((battler, _, _)) => vec![self.get_active_pokemon_by_active_id(*battler)]
                }
            }
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
    /// If present, this move is the actual one that was used (e.g. Metronome)
    proxy_move: Option<Move>,
    /// The last move that this pokemon used
    last_move_used: Option<Move>,
    /// The number of times the last move was used
    last_move_used_counter: u8,
    /// If present, contains the amount of damage this Pokemon encountered last
    damage_this_turn: Vec<(SlotId, Move, u16)>,
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

    /// An action this Pokemon must do on the next turn(s).
    forced_action: Option<ForcedAction>,

    /// If present, the turns remaining bound, and whether a Binding Band was used
    bound: Option<(u8, bool)>,
    /// Turns remaining where this Pokemon is confused (0 == no confusion)
    confused: u8,
    /// If true, this Pokemon is infatuated
    infatuated: bool,
    /// If true, this Pokemon will flinch instead of attack this turn
    flinch: bool,
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
    pub fn baton_pass(&self) -> BattleData {
        BattleData {
            attack_stage: self.attack_stage,
            defense_stage: self.defense_stage,
            special_attack_stage: self.special_attack_stage,
            special_defense_stage: self.special_defense_stage,
            speed_stage: self.speed_stage,
            accuracy_stage: self.accuracy_stage,
            evasion_stage: self.evasion_stage,
            crit_stage: self.crit_stage,
            confused: self.confused,
            locked_on: self.locked_on,
            trapped: self.trapped,
            seeded: self.seeded,
            cursed: self.cursed,
            substituted: self.substituted,
            rooted: self.rooted,
            perish_song_counter: self.perish_song_counter,
            levitating: self.levitating,
            ..Default::default()
        }
    }

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
        self.last_targeted.insert(0, (pkmn.slot_id, attack));
    }

    fn remove_from_targeted_by(&mut self, pkmn: &Slot) {
        self.last_targeted.retain(|(e, _)| *e != pkmn.slot_id);
    }

    pub fn get_last_targeted_attack(&self) -> Option<(SlotId, Move)> {
        self.last_targeted.first()
            .map(|(battler, attack)| (*battler, *attack))
    }

    pub fn is_locked_on_to(&self, pkmn: &Slot) -> bool {
        if let Some((_, battler)) = self.locked_on {
            pkmn.slot_id == battler
        } else {
            false
        }
    }

    /// This hook is called when another Pokemon on the field swaps out
    pub fn other_swapped_out(&mut self, pkmn: &Slot) {
        self.remove_from_targeted_by(pkmn);
        if let Some(b) = self.foresight_by {
            if b == pkmn.slot_id {
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

/// A type of action the Pokemon must do on the next turn
#[derive(Debug, Copy, Clone)]
pub enum ForcedAction {
    /// Do nothing (recharge)
    DoNothing,
    /// Finish attach (charge attack)
    FinishAttack(Move, SelectedTarget),
    /// Attack with this move for the next x turns. If the attack fails, the forced action ends
    AttackWithWeakCounter(Move, u8),
    /// Attack with this move for the next x turns
    AttackWithCounter(Move, u8),
    /// Do nothing, but keep track of damage. (Damage accumulated, turns left)
    Bide(u16, u8)
}
//
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum BattleSideId {
//     Forward,
//     Back
// }
//
// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum DoubleBattleSideId {
//     Left, Right
// }
//
// /// Identifier of a member on the field (more specifically, a "place" on the battlefield)
// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// pub struct SlotId {
//     pub side: BattleSideId,
//     pub individual: DoubleBattleSideId
// }
// impl SlotId {
//     pub fn single(side: BattleSideId) -> SlotId {
//         SlotId {
//             side,
//             individual: DoubleBattleSideId::Left
//         }
//     }
//
//     pub fn double(side: BattleSideId, side2: DoubleBattleSideId) -> SlotId {
//         SlotId {
//             side,
//             individual: side2
//         }
//     }
//
//     /// Test if other battler and this battler are allies
//     /// Note that this returns false if self == other!
//     pub fn is_ally(&self, other: &SlotId) -> bool {
//         self.side == other.side && self.individual != other.individual
//     }
//
//     /// Test if other battler and this battler are either the same, or an ally
//     pub fn is_self_or_ally(&self, other: &SlotId) -> bool {
//         self.side == other.side
//     }
// }

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
    Ability(PokemonId, Ability),
    /// A used move caused the side effect (this is things that happen directly during the move)
    Move(PokemonId, Move),
    /// A used move caused the side effect (this is things that happen later, such as Curse at the end of each turn)
    MoveSideEffect(Move),
    /// The cause is related to the Pokemon's type
    Type(Type),
    /// The side effect was the cause of a user's non-volatile ailment
    Ailment(NonVolatileBattleAilment),
    /// A battler's held item caused the side effect
    HeldItem(PokemonId, Item),
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
    Weather(Weather),
    Safeguard
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
    PokemonLeft(SlotId, PokemonId), PokemonEntered(SlotId, PokemonId), PokemonLeftBatonPass(SlotId, PokemonId),
    LostPP(SlotId, Move, u8, u8),
    FutureSight(SlotId),
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
        must_leave: PokemonId
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
    SleepTalk(SlotId, Move),
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
    Safeguard(BattleSideId),
    Magnitude(u8),
    Encore(SlotId, Move),
    ClearedHazards,
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

    pub fn succeeded(list: Vec<ActionSideEffects>) -> bool {
        list.iter()
            .any(|e| match e {
                ActionSideEffects::NoEffect(_) | ActionSideEffects::NoEffectSecondary(_) |
                ActionSideEffects::Failed(_) | ActionSideEffects::NoTarget | ActionSideEffects::NothingHappened => false,
                _ => true
            })
    }
}