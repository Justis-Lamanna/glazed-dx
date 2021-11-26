use glazed_data::attack::Move;
use glazed_data::item::Item;
use crate::{core, Field, Side};
use crate::{Action, BattleData, Battlefield, BattlePokemon, MutBattlePokemon, Party};

/// One side of battle in a single battle (one trainer, one pokemon)
#[derive(Debug)]
pub struct SingleBattleSide {
    party: Party,
    current_out: u8,
    current_inflictions: BattleData,
    side: Side
}
impl SingleBattleSide {
    pub fn start(party: Party) -> SingleBattleSide {
        SingleBattleSide {
            party,
            current_out: 0,
            current_inflictions: BattleData::default(),
            side: Side::default()
        }
    }

    /// Get an immutable instance of the current Pokemon in battle
    pub fn current(&self) -> BattlePokemon {
        BattlePokemon {
            pokemon: self.party.members[usize::from(self.current_out)].as_ref().unwrap(),
            battle_data: &self.current_inflictions
        }
    }

    /// Get a mutable instance of the current Pokemon in battle
    pub fn current_mut(&mut self) -> MutBattlePokemon {
        MutBattlePokemon {
            pokemon: self.party.members[usize::from(self.current_out)].as_mut().unwrap(),
            ailments: &mut self.current_inflictions
        }
    }
}
impl From<Party> for SingleBattleSide {
    fn from(p: Party) -> Self {
        SingleBattleSide::start(p)
    }
}

impl Battlefield<SingleBattleSide> {
    /// Create a Single Battle with a user and an opponent
    pub fn single_battle<F>(user: F, opponent: F) -> Battlefield<SingleBattleSide> where
        F: Into<SingleBattleSide>
    {
        Battlefield {
            user: user.into(),
            opponent: opponent.into(),
            field: Field::default()
        }
    }

    /// Perform a turn, given everyone's attempted action
    pub fn do_turn(&mut self, user_action: SingleTurnAction, opponent_action: SingleTurnAction) -> () {
        let user_pokemon_speed = self.get_effective_speed(&self.user.side, &self.user.current());
        let opponent_pokemon_speed = self.get_effective_speed(&self.opponent.side, &self.opponent.current());
        let action_order = core::get_action_order(vec![(user_action, user_pokemon_speed), (opponent_action, opponent_pokemon_speed)]);
        println!("{:?}", action_order);
    }
}

/// One of the usable actions that can be taken in a turn
#[derive(Debug)]
pub enum SingleTurnAction {
    Attack(Move),
    Swap(u8),
    UseItem(Item),
    Flee
}
impl Action for SingleTurnAction {
    fn get_priority(&self) -> i8 {
        match self {
            SingleTurnAction::Attack(a) => a.data().priority,
            SingleTurnAction::Swap(_) => 7,
            SingleTurnAction::UseItem(_) => 7,
            SingleTurnAction::Flee => -7
        }
    }
}