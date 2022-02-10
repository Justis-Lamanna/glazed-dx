use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Location {
    // To be filled as-needed
    FarawayPlace
}
impl Default for Location {
    fn default() -> Self {
        Location::FarawayPlace
    }
}

/// Entry Point for retriving Location information
pub struct GlazedLocation;
impl GlazedLocation {
    pub fn current_location() -> Location {
        Location::FarawayPlace
    }

    pub fn is_moss_rock_nearby() -> bool {
        false
    }

    pub fn is_ice_rock_nearby() -> bool {
        false
    }

    pub fn is_magnetic_field_nearby() -> bool {
        false
    }
}