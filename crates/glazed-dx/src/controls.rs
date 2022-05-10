use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::PlayerData;

pub type PlayerControls<'world, 'state, 't> = Query<'world, 'state, &'t ActionState<Actions>, With<PlayerData>>;

#[derive(Actionlike, Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Actions {
    Up, Down, Left, Right,
    Accept, Cancel
}