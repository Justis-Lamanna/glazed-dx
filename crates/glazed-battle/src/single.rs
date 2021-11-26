use glazed_data::abilities::Ability;
use glazed_data::attack::Move;
use glazed_data::item::Item;
use crate::core;
use crate::{Action, BattleData, Battlefield, BattlePokemon, EntryHazard, MutBattlePokemon, Party, Weather};

/// One side of battle in a single battle (one trainer, one pokemon)
#[derive(Debug)]
pub struct SingleBattleSide {
    party: Party,
    current_out: u8,
    current_inflictions: BattleData,
    hazard: Option<EntryHazard>
}
impl SingleBattleSide {
    pub fn start(party: Party) -> SingleBattleSide {
        SingleBattleSide {
            party,
            current_out: 0,
            current_inflictions: BattleData::default(),
            hazard: None
        }
    }

    pub fn current(&self) -> BattlePokemon {
        BattlePokemon {
            pokemon: self.party.members[usize::from(self.current_out)].as_ref().unwrap(),
            battle_data: &self.current_inflictions
        }
    }

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
    pub fn single_battle<F>(user: F, opponent: F) -> Battlefield<SingleBattleSide> where
        F: Into<SingleBattleSide>
    {
        Battlefield {
            user: user.into(),
            opponent: opponent.into(),
            weather: Option::None
        }
    }

    fn get_effective_speed(&self, pokemon: &BattlePokemon) {
        let speed = pokemon.get_effective_speed(); //Raw speed + stage multipliers

        // Ability modifiers
        let ability_multiplier = match pokemon.pokemon.get_ability() {
            Ability::Chlorophyll if self.is_sunny() => 2.0,
            Ability::SandRush if self.is_sandstorm() => 2.0,
            Ability::SwiftSwim if self.is_rain() => 2.0,
            Ability::SlushRush if self.is_hail() => 2.0,
            Ability::QuickFeet if pokemon.has_status_condition() => 1.5,
            Ability::Unburden if pokemon.battle_data.lost_held_item => 2.0,
            _ => 1.0
        };
    }

    pub fn do_turn(&mut self, user_action: SingleTurnAction, opponent_action: SingleTurnAction) -> () {
        let user_pokemon_speed = self.user.current().get_effective_speed();
        let opponent_pokemon_speed = self.opponent.current().get_effective_speed();
        let action_order = core::get_action_order(vec![(user_action, user_pokemon_speed), (opponent_action, opponent_pokemon_speed)]);
        println!("{:?}", action_order);

    }
}

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