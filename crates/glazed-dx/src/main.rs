use glazed_data::attack::BattleStat::Attack;
use glazed_data::attack::DamageType::Special;
use glazed_data::constants::Species;
use glazed_data::pokemon::{Pokemon, PokemonTemplate};
use glazed_data::attack::Move;

use glazed_core::Id;

fn main() {
    let mut template = PokemonTemplate::new(Species::Quilava);
    template.level = 5;
    println!("Attack: {}", Move::DoubleSlap.get_id());
    println!("Hello, {:#?}", Pokemon::from(template));
}
