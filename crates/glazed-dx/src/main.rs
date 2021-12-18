use glazed_battle::{Battlefield, Party, TurnAction};
use glazed_battle::single::SingleBattleSide;
use glazed_data::constants::Species;
use glazed_data::pokemon::PokemonTemplate;
use glazed_data::attack::Move;
use glazed_data::item::Item;

fn main() {
    let me = Party::create_one(PokemonTemplate::pokemon(Species::Quilava, 20));
    let mut ivysaur = PokemonTemplate::pokemon(Species::Ivysaur, 20);
    let them = Party::create_one(ivysaur);
    let mut battlefield = Battlefield::single_battle(me, them);
    let fx = battlefield.do_damage(SingleBattleSide::USER, Move::TripleKick, SingleBattleSide::OPPONENT);
    println!("{:#?}", fx);
    //println!("{:#?}", battlefield);
}
