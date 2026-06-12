use std::fmt;

use reqwest::{Error, get};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use thiserror::Error;

pub const BATTERY: &str = "https://battery.nekit.dev/";

pub const FULL_NAME: &str = "fa-battery-full";
pub const THREE_QUARTERS_NAME: &str = "fa-battery-three-quarters";
pub const HALF_NAME: &str = "fa-battery-half";
pub const QUARTER_NAME: &str = "fa-battery-quarter";
pub const EMPTY_NAME: &str = "fa-battery-empty";

pub type StaticStr = &'static str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level {
    value: u8,
}

impl Serialize for Level {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Level {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = u8::deserialize(deserializer)?;

        let level = Self::new(value).map_err(D::Error::custom)?;

        Ok(level)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value}%", value = self.get())
    }
}

#[derive(Debug, Error)]
#[error("invalid battery level `{value}`")]
pub struct LevelError {
    value: u8,
}

impl LevelError {
    pub(crate) const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::FULL
    }
}

pub const EMPTY: u8 = 0;
pub const QUARTER: u8 = 25;
pub const HALF: u8 = 50;
pub const THREE_QUARTERS: u8 = 75;
pub const FULL: u8 = 100;

impl Level {
    pub const fn of(value: u8) -> Option<Self> {
        #[allow(clippy::absurd_extreme_comparisons)]
        if value < EMPTY || value > FULL {
            return None;
        }

        let level = Self { value };

        Some(level)
    }

    pub const fn new(value: u8) -> Result<Self, LevelError> {
        if let Some(level) = Self::of(value) {
            Ok(level)
        } else {
            Err(LevelError::new(value))
        }
    }

    pub const fn get(self) -> u8 {
        self.value
    }

    pub const fn name(self) -> StaticStr {
        match self.get() {
            EMPTY..QUARTER => EMPTY_NAME,
            QUARTER..HALF => QUARTER_NAME,
            HALF..THREE_QUARTERS => HALF_NAME,
            THREE_QUARTERS..FULL => THREE_QUARTERS_NAME,
            FULL => FULL_NAME,
            // NOTE: this should never happen since the type guarantees the `EMPTY..=FULL` range
            _ => unreachable!(),
        }
    }

    pub const EMPTY: Self = Self::of(EMPTY).unwrap();
    pub const QUARTER: Self = Self::of(QUARTER).unwrap();
    pub const HALF: Self = Self::of(HALF).unwrap();
    pub const THREE_QUARTERS: Self = Self::of(THREE_QUARTERS).unwrap();
    pub const FULL: Self = Self::of(FULL).unwrap();
}

pub async fn get_level() -> Result<Level, Error> {
    get(BATTERY).await?.json().await
}
