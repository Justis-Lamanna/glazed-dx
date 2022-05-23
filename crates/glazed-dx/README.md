# Pokemon Glazed DX
A complete rewrite of Pokémon Glazed in the Rust language.

This crate contains all the game logic, and encapsulates the bulk of the game. It is built
atop the [Bevy](https://bevyengine.org/) engine, which performs all the logic for displaying
graphics, playing audio, and managing game state.

## File Structure
In the `src/` directory, the following Rust files are present. I have attempted to encapsulate
each concept in its own file, but I am not the best at this:
* `main.rs` - Contains the code that builds the game and loads important assets.
* `actions.rs` - Contains a number of `Action`s for common use cases. 
Actions encapsulate a piece of code that can span several frames, such as waiting for a
specific duration or player input. Built atop [bevy-sequential-actions](https://crates.io/crates/bevy-sequential-actions)
* `anim.rs` - Contains code for basic frame-based animations, which Bevy Tween does not support.
* `controls.rs` - Contains code related to mapping controls to different in-game actions.
Multiple groups of actions are supported, but currently only global actions are present.
* `locale.rs` - Multi-language functionality, built on [Project Fluent](https://projectfluent.org/).
More locale information is provided below.
* `pkmn.rs` - Contains orchestration for common Pokemon-related processes, such as retrieving
a sprite or cry for a specific Pokémon.
* `player.rs` - Contains orchestration for Player-related processes, such as awarding a Pokémon.
* `state.rs` - Contains orchestration for saving and loading game state, such as flags.
* `text.rs` - Contains orchestration for managing text boxes.
* `util.rs` - Contains various convenient methods. Also contains transition logic, which
should be moved to another module, or into another action.
* `scenes/` - This directory contains modules for each "scene" in the game.
    * `intro.rs` - Title screen
    * `lecture.rs` - Introductory Professor Lecture

## Asset Structure
The `assets/` directory contains all assets for the game, such as fonts, sounds, and graphics.

The `assets_aseprite/` directory is a pseudo-mirror of the `assets/` directory, but contains
aseprite files, rather than png. Not all assets are stored here, only animations and other
complex sprites.

## Locales
Localization support is done via Project Fluent, through `.ftl` files. These files are stored
in `assets/locales/<locale id>/<filename>.ftl`. Each `.ftl` file in a specific directory are
combined at runtime, so can be split arbitrarily depending on organization. 

Other languages can be supported by creating another directory in `assets/locales`, with the
directory name being the language code. The locale system will intelligently fallback if a code
does not exist in the preferred language. As an example, suppose the following structure:
* `pt-BR/`
* `pt/`
* `en-US/`

If the preferred language is `pt-BR`, the locale system will first check language files there.
If the specific key does not exist there, `pt` is checked. If the key also does not exist
in `pt`, `en-US` is returned (behavior is undefined if it does not exist in `en-US`).

In the future, locale-specific assets may be supported as well.

Localization files can be hot-swapped, for convenience. However, they won't update mid-sentence;
you will need to re-open the text in-game to view the changes.

### Placeables
The following is a list of dynamic values that can be inserted into any string:
* `player` - Replaced with player name.

Other placeables may be present on a message-by-message basis, and must be set in code
via the `buffer_*` methods. The key provided to these methods can be whatever seems
most readable to you. After buffering a value, the same key can be used in the `.ftl` files.

Buffered values last until cleared manually. Make sure to clear the buffer after use.

Buffer methods:
* `buffer_string` - Allows you to buffer any arbitrary value that inherits `ToString`. 
* `buffer_species_name` - Allows you buffer the species name of any Pokemon.