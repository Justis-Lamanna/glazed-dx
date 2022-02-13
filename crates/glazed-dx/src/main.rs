use log::LevelFilter;
use simple_logger::SimpleLogger;

use glazed_data::{species::Species, pokemon::{PokemonTemplate, Gender}};
use glazed_data::evolutions::breeding;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init().unwrap();

    let dad = PokemonTemplate::pokemon(Species::Quilava, 20)
    .gender(Gender::Male)
    .into();
    let mom = PokemonTemplate::pokemon(Species::Ditto, 20)
    // .gender(Gender::Female)
    .into();
    
    dbg!(breeding::create_offspring(&dad, &mom));
}
