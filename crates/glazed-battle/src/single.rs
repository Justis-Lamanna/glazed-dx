use glazed_data::abilities::Ability;
use glazed_data::attack::Move;
use glazed_data::constants::Species;
use glazed_data::item::Item;
use crate::{core, Field, Side};
use crate::{Action, BattleData, Battlefield, BattlePokemon, EntryHazard, MutBattlePokemon, Party, Weather};

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

    /// Get the current effective speed of a specific Pokemon on a specific side of the field
    /// This takes into account:
    /// * Raw speed stat of the Pokemon
    /// * Speed stage
    /// * Abilities, if applicable to the current state of the field
    /// * Items, if applicable to the current state of the field
    /// * Other statuses, such as paralysis or Tailwind
    fn get_effective_speed(&self, side: &Side, pokemon: &BattlePokemon) -> u16 {
        let speed = pokemon.get_effective_speed(); //Raw speed + stage multipliers

        // Ability modifiers
        let ability_multiplier = match pokemon.get_ability() {
            Ability::Chlorophyll if self.field.is_sunny() => 2.0,
            Ability::SandRush if self.field.is_sandstorm() => 2.0,
            Ability::SwiftSwim if self.field.is_rain() => 2.0,
            Ability::SlushRush if self.field.is_hail() => 2.0,
            Ability::QuickFeet if pokemon.has_status_condition() => 1.5,
            Ability::Unburden if pokemon.battle_data.lost_held_item => 2.0,
            Ability::SlowStart if pokemon.battle_data.turn_count < 5 => 0.5,
            _ => 1.0
        };

        let item_multiplier = match pokemon.pokemon.held_item {
            Some(Item::ChoiceScarf) => 1.5,
            Some(Item::QuickPowder) if pokemon.pokemon.species == Species::Ditto => 2.0,
            _ => 1.0
        };

        let mut eff_speed = f64::from(speed) * ability_multiplier * item_multiplier;
        if side.tailwind > 0 {
            eff_speed *= 2.0;
        }
        if pokemon.pokemon.status.paralysis {
            eff_speed *= 0.5;
        }

        eff_speed as u16
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