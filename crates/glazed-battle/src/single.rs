use glazed_data::attack::Move;
use glazed_data::item::Item;
use glazed_data::pokemon::Pokemon;
use crate::{Battler, BattleTypeTrait, core, Field, Side};
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
}
impl From<Party> for SingleBattleSide {
    fn from(p: Party) -> Self {
        SingleBattleSide::start(p)
    }
}
impl BattleTypeTrait for SingleBattleSide {
    fn get_by_id(&self, _id: u8) -> Option<BattlePokemon> {
        let member = &self.party.members[usize::from(self.current_out)];
        match member {
            Some(p) => Some(BattlePokemon {
                pokemon: p,
                battle_data: &self.current_inflictions
            }),
            None => None
        }
    }

    fn get_side(&self) -> &Side {
        &self.side
    }
}

impl Battlefield<SingleBattleSide> {
    const USER: Battler = Battler(false, 0);
    const OPPONENT: Battler = Battler(true, 0);
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
        let user_pokemon_speed = self.get_effective_speed(Battlefield::USER);
        let opponent_pokemon_speed = self.get_effective_speed(Battlefield::OPPONENT);
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