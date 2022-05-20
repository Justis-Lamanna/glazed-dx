use std::ops::Range;
use serde::{Deserialize, Serialize};
use chrono::{Datelike, Local, Timelike, Weekday};

use strum_macros::IntoStaticStr;

/// Represents the season in the game
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, IntoStaticStr)]
pub enum Season {
    Spring, Summer, Autumn, Winter
}

/// Represents the time of day in the game
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Eq, Hash)]
pub enum TimeOfDay {
    Day,
    Night
}

/// Entry point for retrieving time information
pub struct GlazedTime;
impl GlazedTime {
    /// The span of hours considered Daytime during the Spring
    const DAYTIME_SPRING: Range<u8> = 5..20;
    /// The span of hours considered Daytime during the Summer
    const DAYTIME_SUMMER: Range<u8> = 4..21;
    /// The span of hours considered Daytime during the Autumn
    const DAYTIME_AUTUMN: Range<u8> = 6..20;
    /// The span of hours considered Daytime during the Winter
    const DAYTIME_WINTER: Range<u8> = 7..19;

    /// Get the current time of day in-game
    pub fn get_time_of_day() -> TimeOfDay {
        let daytime_range = match GlazedTime::get_season() {
            Season::Spring => GlazedTime::DAYTIME_SPRING,
            Season::Summer => GlazedTime::DAYTIME_SUMMER,
            Season::Autumn => GlazedTime::DAYTIME_AUTUMN,
            Season::Winter => GlazedTime::DAYTIME_WINTER,
        };
        let hour = Local::now().hour() as u8;
        if daytime_range.contains(&hour) {
            TimeOfDay::Day
        } else {
            TimeOfDay::Night
        }
    }

    /// Get the current day of the week in-game
    pub fn get_day_of_week() -> Weekday {
        Local::now().weekday()
    }

    /// Get the current season in-game
    pub fn get_season() -> Season {
        let month = Local::now().month0();
        match month % 4 {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Autumn,
            3 => Season::Winter,
            _ => unreachable!()
        }
    }
}