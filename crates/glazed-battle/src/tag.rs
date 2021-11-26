use crate::{BattleData, Battlefield, EntryHazard, Field, Party};
use crate::double::DoubleTurnAction;

/// One side of battle in a tag battle (two trainers, one pokemon each)
#[derive(Debug)]
pub struct TagBattleSide {
    party: (Party, Party),
    current_out: (u8, u8),
    current_inflictions: (BattleData, BattleData),
    hazard: Option<EntryHazard>
}
impl TagBattleSide {
    pub fn start(party_left: Party, party_right: Party) -> TagBattleSide {
        TagBattleSide {
            party: (party_left, party_right),
            current_out: (0, 0),
            current_inflictions: (BattleData::default(), BattleData::default()),
            hazard: None
        }
    }
}
impl From<(Party, Party)> for TagBattleSide {
    fn from(pair: (Party, Party)) -> Self {
        TagBattleSide::start(pair.0, pair.1)
    }
}

impl Battlefield<TagBattleSide> {
    pub fn tag_battle<F>(user: F, opponent: F) -> Battlefield<TagBattleSide> where
        F: Into<TagBattleSide>
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