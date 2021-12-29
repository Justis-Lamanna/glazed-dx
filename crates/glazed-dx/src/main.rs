use glazed_battle::{Battlefield, Battler, BattleSide, Party};
use glazed_battle::effects::SelectedTarget;
use glazed_data::constants::Species;
use glazed_data::pokemon::{Gender, PokemonTemplate};
use glazed_data::attack::Move;

fn main() {
    let me = Party::create(vec![
        PokemonTemplate::pokemon(Species::Quilava, 20).shiny(),
        PokemonTemplate::pokemon(Species::Eevee, 20),
        PokemonTemplate::pokemon(Species::Buizel, 20),
        PokemonTemplate::pokemon(Species::Furret, 20)]
    );
    let them = Party::create_one(PokemonTemplate::pokemon(Species::Ivysaur, 20));

    let mut battlefield = Battlefield::single_battle(me, them);
    //for _ in 1..=40 {
        let fx = battlefield.do_attack(Battler::single(BattleSide::Forward), Move::PayDay, SelectedTarget::Implied);

        println!("{:#?}", fx);
    //}
    //println!("{:#?}", battlefield);
}
