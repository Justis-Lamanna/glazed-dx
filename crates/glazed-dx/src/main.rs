use glazed_data::attack::DamageType::Special;
use glazed_data::constants::Species;
use glazed_data::pokemon::{Pokemon, PokemonTemplate};

fn main() {
    let mut template = PokemonTemplate::new(Species::Quilava);
    template.level = 5;
    println!("Hello, {:#?}", Pokemon::from(template));
}
