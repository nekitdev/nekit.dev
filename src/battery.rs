use std::fmt;

use refining::prelude::*;
use reqwest::{Error, get};
use serde::{Deserialize, Serialize};

pub const BATTERY: &str = "https://battery.nekit.dev/";

pub const FULL_NAME: &str = "fa-battery-full";
pub const THREE_QUARTERS_NAME: &str = "fa-battery-three-quarters";
pub const HALF_NAME: &str = "fa-battery-half";
pub const QUARTER_NAME: &str = "fa-battery-quarter";
pub const EMPTY_NAME: &str = "fa-battery-empty";

pub type Level = Refinement<u8, u8::Closed<0, 100>>;

pub type StaticStr = &'static str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Battery {
    level: Level,
}

impl Battery {
    pub const fn new(level: Level) -> Self {
        Self { level }
    }

    pub const fn level(self) -> Level {
        self.level
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{level}%", level = self.level())
    }
}

pub const EMPTY: u8 = 0;
pub const QUARTER: u8 = 25;
pub const HALF: u8 = 50;
pub const THREE_QUARTERS: u8 = 75;
pub const FULL: u8 = 100;

impl Battery {
    pub fn name(self) -> StaticStr {
        match self.level().get() {
            EMPTY..QUARTER => EMPTY_NAME,
            QUARTER..HALF => QUARTER_NAME,
            HALF..THREE_QUARTERS => HALF_NAME,
            THREE_QUARTERS..FULL => THREE_QUARTERS_NAME,
            FULL => FULL_NAME,
            // NOTE: this should never happen since the type guarantees the `EMPTY..=FULL` range
            _ => unreachable!(),
        }
    }
}

pub async fn get_battery() -> Result<Battery, Error> {
    get(BATTERY).await?.json().await
}
