use log::LevelFilter;
use simple_logger::SimpleLogger;

use glazed_data::{species::Species, evolutions::Evolution, lookups::Lookup};

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init().unwrap();

    dbg!(Evolution::lookup(&Species::Marill));
}
