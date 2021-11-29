use glazed_battle::{Battlefield, Party, TurnAction};
use glazed_battle::single::SingleBattleSide;
use glazed_data::constants::Species;
use glazed_data::pokemon::PokemonTemplate;
use glazed_data::attack::Move;

fn main() {
    let me = Party::create_one(PokemonTemplate::pokemon(Species::Quilava, 20));
    let them = Party::create_one(PokemonTemplate::pokemon(Species::Ivysaur, 20));
    let mut battlefield = Battlefield::single_battle(me, them);
    // let turn = battlefield.perform_turn(
    //     TurnAction::Attack(Move::Tackle, SingleBattleSide::OPPONENT),
    //     TurnAction::Attack(Move::QuickAttack, SingleBattleSide::USER));
    // battlefield.finish_turn(turn);
    println!("{:#?}", battlefield);
    for _ in 1..40 {
        let effects = battlefield.do_attack(SingleBattleSide::USER, Move::Wrap, SingleBattleSide::OPPONENT, false);
        println!("{:?}", effects);
    }
}
