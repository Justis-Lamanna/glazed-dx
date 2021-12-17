use glazed_battle::{Battlefield, Party, TurnAction};
use glazed_battle::single::SingleBattleSide;
use glazed_data::constants::Species;
use glazed_data::pokemon::PokemonTemplate;
use glazed_data::attack::Move;
use glazed_data::item::Item;

fn main() {
    let me = Party::create_one(PokemonTemplate::pokemon(Species::Quilava, 20));
    let mut ivysaur = PokemonTemplate::pokemon(Species::Ivysaur, 20);
    ivysaur.held_item = Some(Item::FocusSash);
    let them = Party::create_one(ivysaur);
    let mut battlefield = Battlefield::single_battle(me, them);
    let fx = battlefield.do_damage(SingleBattleSide::USER, Move::Tackle, SingleBattleSide::OPPONENT);
    // let turn = battlefield.perform_turn(
    //     TurnAction::Attack(Move::Tackle, SingleBattleSide::OPPONENT),
    //     TurnAction::Attack(Move::QuickAttack, SingleBattleSide::USER));
    // battlefield.finish_turn(turn);
    println!("{:#?}", fx);
    println!("{:#?}", battlefield);
    // for _ in 1..40 {
    //     let effects = battlefield.do_attack(SingleBattleSide::USER, Move::Wrap, SingleBattleSide::OPPONENT, false);
    //     println!("{:?}", effects);
    // }
}
