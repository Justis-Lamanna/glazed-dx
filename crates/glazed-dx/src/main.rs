use glazed_battle::{Battlefield, Party};
use glazed_battle::single::SingleTurnAction;
use glazed_data::attack::BattleStat::Attack;
use glazed_data::attack::DamageType::Special;
use glazed_data::constants::Species;
use glazed_data::pokemon::{Pokemon, PokemonTemplate};
use glazed_data::attack::Move;

use glazed_core::Id;

fn main() {
    let me = Party::create_one(PokemonTemplate::pokemon(Species::Quilava, 20));
    let them = Party::create_one(PokemonTemplate::pokemon(Species::Ivysaur, 20));
    let mut battlefield = Battlefield::single_battle(me, them);
    battlefield.do_turn(SingleTurnAction::Attack(Move::Tackle), SingleTurnAction::Attack(Move::QuickAttack));
}
