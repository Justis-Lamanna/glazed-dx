use glazed_data::attack::Move;
use glazed_data::item::Item;
use glazed_data::pokemon::Pokemon;
use crate::{Battler, BattleTypeTrait, core, Field, Side, Turn, TurnAction};
use crate::{BattleData, Battlefield, BattlePokemon, Party};

/// One side of battle in a single battle (one trainer, one pokemon)
#[derive(Debug)]
pub struct SingleBattleSide {
    party: Party,
    current_out: u8,
    current_inflictions: BattleData,
    side: Side
}
impl SingleBattleSide {
    pub const USER: Battler = Battler(false, 0);
    pub const OPPONENT: Battler = Battler(true, 0);

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
    fn get_by_id(&self, id: &Battler) -> Option<BattlePokemon> {
        let member = &self.party.members[usize::from(self.current_out)];
        match member {
            Some(p) => Some(BattlePokemon {
                id: id.clone(),
                pokemon: p,
                battle_data: &self.current_inflictions
            }),
            None => None
        }
    }

    fn do_by_id<F>(&mut self, _id: &Battler, func: F) where F: Fn(&mut Pokemon, &mut BattleData) -> () {
        let member = self.party.members[usize::from(self.current_out)].as_mut();
        match member {
            Some(p) => {
                func(p, &mut self.current_inflictions)
            },
            None => {}
        }
    }

    fn get_side(&self) -> &Side {
        &self.side
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
        let mut turn = Turn::new();

        let user_pokemon_speed = self.get_effective_speed(&SingleBattleSide::USER);
        let opponent_pokemon_speed = self.get_effective_speed(&SingleBattleSide::OPPONENT);
        let action_order = core::get_action_order(vec![(user_action, user_pokemon_speed), (opponent_action, opponent_pokemon_speed)]);
        println!("{:?}", action_order);

        turn
    }
}