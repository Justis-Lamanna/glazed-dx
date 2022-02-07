use log::LevelFilter;
use simple_logger::SimpleLogger;

use glazed_battle::{Battlefield, Party};
use glazed_battle::SelectedTarget;

use glazed_data::attack::{Move, MoveData};
use glazed_data::species::{GenesectForm, Species, UnownForm};
use glazed_data::contest::BerryPokeblockData;
use glazed_data::item::Berry;
use glazed_data::lookups::Lookup;
use glazed_data::pokemon::{PokemonTemplate, SpeciesData};

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init().unwrap();

    let b = SpeciesData::lookup(&Species::Genesect(GenesectForm::Chill));
    dbg!(b);

    // for _ in 1..=40 {
    let me = Party::create(vec![
        PokemonTemplate::pokemon(Species::Quilava, 100).shiny(),
        PokemonTemplate::pokemon(Species::Eevee, 20),
        PokemonTemplate::pokemon(Species::Buizel, 20),
        PokemonTemplate::pokemon(Species::Furret, 20)]
    );
    let them = Party::create_one(PokemonTemplate::pokemon(Species::Ivysaur, 20).holding(Berry::SitrusBerry));

    let mut battlefield = Battlefield::single_battle(me, them);
    battlefield.do_attack(0, Move::SuperFang, SelectedTarget::Implied);
    battlefield.do_attack(0, Move::SuperFang, SelectedTarget::Implied);
    battlefield.do_attack(0, Move::SuperFang, SelectedTarget::Implied);
    //battlefield.do_implicit_attack(0);
    // }
}
