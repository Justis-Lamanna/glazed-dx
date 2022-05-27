use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use glazed_data::pokemon::*;
use rand::Rng as o;

use crate::{locale::Fluent, pkmn::PokemonLookupService, util::Rng};

/// The maximum number of Pokemon allowed in the party
pub const MAX_POKEMON_IN_PARTY: usize = 6;
/// The maximum number of Pokemon in one box
pub const MAX_POKEMON_IN_BOX: usize = 30;
/// The number of boxes the player starts with.
/// Also the number of boxes that are added if there are no more boxes
pub const STARTING_BOX_COUNT: usize = 8;

/// Represents the player
#[derive(Default, Debug)]
pub struct Player {
    /// The player's name
    pub name: String,
    /// The player's trainer ID
    pub trainer_id: u16,
    /// The player's secret ID
    pub secret_id: u16,
    /// The player's current party
    pub party: Party
}

/// Represents a party of Pokemon
#[derive(Default, Debug)]
pub struct Party {
    slots: Vec<Pokemon>
}
impl Party {
    /// Add a Pokemon to the party. 
    /// If the party is full, this acts as a no-op.
    fn add_pokemon<T: Into<Pokemon>>(&mut self, pkmn: T) {
        if self.slots.len() < MAX_POKEMON_IN_PARTY {
            self.slots.push(pkmn.into());
        }
    }
}

/// Represents boxes of Pokemon the player does not currently use.
/// This structure maintains a cursor, which represents the active box.
/// The active box will be where caught Pokemon are placed.
#[derive(Default, Debug)]
pub struct Boxes {
    cursor: usize,
    boxes: Vec<Box>
}
impl Boxes {
    /// Check if the player has no boxes at all
    pub fn has_no_boxes(&self) -> bool {
        self.boxes.is_empty()
    }

    /// Get the current box
    fn get_active_box(&mut self) -> &mut Box {
        if self.cursor >= self.boxes.len() {
            self.cursor = 0;
        }
        &mut self.boxes[self.cursor]
    }
    
    /// Advance to the next box, if possible.
    /// Only boxes after the current one are searched. This may be changed in the future,
    /// but as far as I know, this is the typical behavior for Pokemon.
    /// If None is returned, there are no boxes after the current one, and more need to be created.
    fn advance_to_next_best_box(&mut self) -> Option<(usize, &mut Box)> {
        let boxes = self.boxes.iter_mut()
            .enumerate()
            .skip(self.cursor + 1);
        for (idx, b) in boxes {
            if !b.is_full() {
                self.cursor = idx;
                return Some((idx, b));
            }
        }
        None
    }
}

/// Represents a single box of Pokemon.
#[derive(Default, Debug)]
pub struct Box {
    name: String,
    slots: [Option<Pokemon>; MAX_POKEMON_IN_BOX]
}
impl Box {
    /// Check if this box is full
    pub fn is_full(&self) -> bool {
        self.slots.iter()
            .find(|i| i.is_none())
            .is_none()
    }

    /// Add a Pokemon to this box.
    /// If the box is full, this acts as a no-op.
    fn add_pokemon<T: Into<Pokemon>>(&mut self, pkmn: T) {
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                *slot = Some(pkmn.into());
                return;
            }
        }
    }
}

/// Represents the result of giving the player a Pokemon
pub enum AddPokemonResult {
    /// The Pokemon was added into the party
    InParty,
    /// The Pokemon was added in the active box
    InBox(usize),
    /// The active box was changed to a new box, and placed there
    BoxFullNewBox {
        old_box: usize,
        new_box: usize
    }
}

/// The orchestrator for Player data.
#[derive(SystemParam)]
pub struct PlayerService<'w, 's> {
    pub player: ResMut<'w, Player>,
    pub boxes: ResMut<'w, Boxes>,
    fluent: Fluent<'w, 's>,
    pub pkmn_lookup: PokemonLookupService<'w, 's>,
    rng: Local<'s, Rng>
}
impl<'w, 's> PlayerService<'w, 's> {
    pub fn init_player(&mut self, name: String) {
        self.player.name = name;
        self.player.trainer_id = self.rng.gen();
        self.player.secret_id = self.rng.gen();
        self.player.party = Party::default();
        *self.boxes = Boxes::default();
        // Create initial boxes
        self.create_more_boxes();
    }

    /// Intelligently give the player a Pokemon.
    /// The following steps are followed to add the Pokemon:
    /// 1. Add the Pokemon to the party
    /// 2. If the party is full, add the Pokemon to the active box
    /// 3. If the active box is full, advance to the next non-full box
    /// 4. If no non-full boxes are found, create a new set of boxes
    /// Note that we need to also do Pokedex information here I think. 
    pub fn add_pokemon<T: Into<Pokemon>>(&mut self, pkmn: T) -> AddPokemonResult {
        if MAX_POKEMON_IN_PARTY < self.player.party.slots.len() {
            self.player.party.add_pokemon(pkmn);
            AddPokemonResult::InParty
        } else {
            // If the player has no boxes for some reason, we create them.
            if self.boxes.has_no_boxes() {
                self.create_more_boxes();
            }

            let current_box = self.boxes.get_active_box();
            if current_box.is_full() {
                // If the current box is full, advance to the next box until you run out of boxes.
                let old_box = self.boxes.cursor;
                if let Some((new_box, b)) = self.boxes.advance_to_next_best_box() {
                    b.add_pokemon(pkmn);
                    AddPokemonResult::BoxFullNewBox { old_box, new_box }
                } else {
                    // If there are no good box candidates, create more boxes!
                    self.create_more_boxes();
                    let new_box = self.boxes.get_active_box();
                    new_box.add_pokemon(pkmn);

                    let new_box = self.boxes.cursor;
                    AddPokemonResult::BoxFullNewBox { old_box, new_box }
                }
            } else {
                // Add Pokemon to box
                current_box.add_pokemon(pkmn);
                AddPokemonResult::InBox(self.boxes.cursor)
            }
        }
    }

    /// Create STARTING_BOX_COUNT boxes, and move the cursor to the first one
    fn create_more_boxes(&mut self) {
        self.boxes.cursor = self.boxes.boxes.len();
        for _ in 0..STARTING_BOX_COUNT {
            self.add_box();
        }
    }

    /// Create a box with the appropriate default name
    fn add_box(&mut self) {
        let next_box_number = self.boxes.boxes.len();
        let box_name = self.fluent.translate_with_arg(
            "box-default-name", 
            "box-number", next_box_number)
            .unwrap_or_else(|| format!("Box {}", next_box_number));
        let b = Box {
            name: box_name,
            ..default()
        };
        self.boxes.boxes.push(b);
    }
}