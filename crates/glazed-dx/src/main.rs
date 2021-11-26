use glazed_battle::{Battlefield, Party};
use glazed_battle::single::SingleTurnAction;
use glazed_data::constants::Species;
use glazed_data::pokemon::PokemonTemplate;
use glazed_data::attack::Move;

fn main() {
    let me = Party::create_one(PokemonTemplate::pokemon(Species::Quilava, 20));
    let them = Party::create_one(PokemonTemplate::pokemon(Species::Ivysaur, 20));
    let mut battlefield = Battlefield::single_battle(me, them);
    battlefield.do_turn(SingleTurnAction::Attack(Move::Tackle), SingleTurnAction::Attack(Move::QuickAttack));
}
