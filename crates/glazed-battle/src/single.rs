use glazed_data::pokemon::Pokemon;
use crate::{Battler, BattleTypeTrait, Field, Side, Turn, TurnAction};
use crate::{BattleData, Battlefield, Party};

/// One side of battle in a single battle (one trainer, one pokemon)
#[derive(Debug)]
pub struct SingleBattleSide {
    party: Party,
    current_out_slot: usize,
    battle_data: BattleData,
    side: Side
}
impl SingleBattleSide {
    pub const USER: Battler = Battler(true, 0);
    pub const OPPONENT: Battler = Battler(false, 0);

    pub fn start(party: Party) -> SingleBattleSide {
        SingleBattleSide {
            party,
            current_out_slot: 0,
            battle_data: BattleData::default(),
            side: Side::default()
        }
    }

    pub fn swap(&mut self, slot: usize) {
        self.current_out_slot = slot;
    }
}
impl From<Party> for SingleBattleSide {
    fn from(p: Party) -> Self {
        SingleBattleSide::start(p)
    }
}
impl BattleTypeTrait for SingleBattleSide {
    fn get_pokemon_by_id(&self, _id: &Battler) -> Option<&Pokemon> {
        return self.party.members[self.current_out_slot].as_ref();
    }

    fn get_mut_pokemon_by_id(&mut self, _id: &Battler) -> Option<&mut Pokemon> {
        return self.party.members[self.current_out_slot].as_mut();
    }

    fn get_battle_data(&self, _id: &Battler) -> &BattleData {
        return &self.battle_data;
    }

    fn get_mut_battle_data(&mut self, _id: &Battler) -> &mut BattleData {
        return &mut self.battle_data;
    }

    fn get_side(&self) -> &Side {
        return &self.side;
    }

    fn get_party(&self, _id: &Battler) -> &Party {
        return &self.party
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
            field: Field::default(),
            turn_record: Vec::new()
        }
    }

    /// Perform a turn, given everyone's attempted action
    pub fn perform_turn(&mut self, user_action: TurnAction, opponent_action: TurnAction) -> Turn {
        let turn = Turn::new();

        // let user_pokemon_speed = self.get_effective_speed(&SingleBattleSide::USER);
        // let opponent_pokemon_speed = self.get_effective_speed(&SingleBattleSide::OPPONENT);
        let action_order = vec![user_action, opponent_action];
        println!("{:?}", action_order);

        turn
    }
}