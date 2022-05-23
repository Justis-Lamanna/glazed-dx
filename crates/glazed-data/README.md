# Pokemon Glazed DX
A complete rewrite of Pokémon Glazed in the Rust language.

This crate contains a number of enums and structs representing the core concepts of Pokémon.
These concepts don't depend on Bevy, which is why they remain in a separate crate.

This code base also formerly included lookups, which allow for looking up of data related
to an enum. One example would be retrieving the base stats, given a Pokémon species. In the
future, these lookups will be handled by the Bevy asset system, to allow for hot-swapping.