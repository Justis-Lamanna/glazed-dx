use glazed_data::attack::Move;
use glazed_data::item::Item;
use crate::{BattleData, Battlefield, EntryHazard, Field, Party};

/// One side of battle in a double battle (one trainer, two pokemon)
#[derive(Debug)]
pub struct DoubleBattleSide {
    party: Party,
    current_out: (u8, u8),
    current_inflictions: (BattleData, BattleData),
    hazard: Option<EntryHazard>
}
impl DoubleBattleSide {
    pub fn start(party: Party) -> DoubleBattleSide {
        DoubleBattleSide {
            party,
            current_out: (0, 1),
            current_inflictions: (BattleData::default(), BattleData::default()),
            hazard: None
        }
    }
}
impl From<Party> for DoubleBattleSide {
    fn from(p: Party) -> Self {
        DoubleBattleSide::start(p)
    }
}

impl Battlefield<DoubleBattleSide> {
    pub fn double_battle<F>(user: F, opponent: F) -> Battlefield<DoubleBattleSide> where
        F: Into<DoubleBattleSide>
    {
        Battlefield {
            user: user.into(),
            opponent: opponent.into(),
            field: Field::default()
        }
    }

    pub fn do_turn(&self, user_left: DoubleTurnAction, user_right: DoubleTurnAction, opponent_left: DoubleTurnAction, opponent_right: DoubleTurnAction) -> () {

    }
}

#[derive(Debug)]
pub enum DoubleTarget {
    User,
    Ally,
    LeftOpponent,
    RightOpponent
}

#[derive(Debug)]
pub enum DoubleTurnAction {
    Attack(Move, Option<DoubleTarget>),
    Swap(u8),
    UseItem(Item),
    Flee
}
