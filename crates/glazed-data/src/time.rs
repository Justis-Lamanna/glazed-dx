use serde::Deserialize;

use strum_macros::IntoStaticStr;

/// Represents the season in the game
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Eq, Hash, IntoStaticStr)]
pub enum Season {
    Spring, Summer, Autumn, Winter
}

/// Represents the time of day in the game
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Eq, Hash)]
pub enum TimeOfDay {
    Day,
    Night
}