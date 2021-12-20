use glazed_battle::{Battlefield, Party};
use glazed_data::constants::Species;
use glazed_data::pokemon::{PokemonStatusCondition, PokemonTemplate};
use glazed_data::attack::Move;

fn main() {
    let me = Party::create(vec![
        PokemonTemplate::pokemon(Species::Quilava, 20),
        PokemonTemplate::pokemon(Species::Eevee, 20),
        PokemonTemplate::pokemon(Species::Buizel, 20),
        PokemonTemplate::pokemon(Species::Furret, 20)]
    );
    let them = Party::create_one(PokemonTemplate::pokemon(Species::Ivysaur, 20));

    let mut battlefield = Battlefield::single_battle(me, them);
    // for _ in 1..=10 {
        let fx = battlefield.do_attack(Battlefield::USER, Move::Tackle, Battlefield::OPPONENT);

        println!("{:#?}", fx);
    // }
    //println!("{:#?}", battlefield);
}
