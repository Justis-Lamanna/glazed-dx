use glazed_battle::{Battlefield, Party};
use glazed_battle::single::SingleBattleSide;
use glazed_data::constants::Species;
use glazed_data::pokemon::PokemonTemplate;
use glazed_data::attack::Move;

fn main() {
    let me = Party::create_four(
        PokemonTemplate::pokemon(Species::Quilava, 20),
        PokemonTemplate::pokemon(Species::Eevee, 20),
        PokemonTemplate::pokemon(Species::Buizel, 20),
        PokemonTemplate::pokemon(Species::Furret, 20)
    );
    let them = Party::create_one(PokemonTemplate::pokemon(Species::Bastiodon, 20));

    let mut battlefield = Battlefield::single_battle(me, them);
    let fx = battlefield.do_attack(SingleBattleSide::USER, Move::SwordsDance, SingleBattleSide::OPPONENT);

    println!("{:#?}", fx);
    //println!("{:#?}", battlefield);
}
