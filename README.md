# Pokemon Glazed DX
A complete rewrite of Pokemon Glazed in the Rust language.

This project is a combination of two inner projects. More detail can be found in their respective crates:

* [glazed-data](crates/glazed-data) - Contains enums and data structures for Pokemon-related items, such as Species.
* [glazed-dx](crates/glazed-dx) - Contains the data and logic for running the game.

## Running the Game
To run the game, simply run `cargo run --package glazed_dx --bin glazed_dx` from the root.
No other files are required.

## Configuring the Game
Various aspects of the game can be configured via `options.yml`, including control schemes,
volume, and other miscellaneous options. All fields in options.yml are optional, with
suitable defaults if missing.