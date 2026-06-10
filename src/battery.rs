use std::fmt;

use reqwest::{Error, get};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use thiserror::Error;

pub const BATTERY: &str = "https://battery.nekit.dev/";

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
        Self::MAX
    }
}

pub const MIN: u8 = 0;
pub const MAX: u8 = 100;

impl Level {
    pub const fn of(value: u8) -> Option<Self> {
        #[allow(clippy::absurd_extreme_comparisons)]
        if value < MIN || value > MAX {
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

    pub const MIN: Self = Self::of(MIN).unwrap();
    pub const MAX: Self = Self::of(MAX).unwrap();
}

pub async fn get_level() -> Result<Level, Error> {
    get(BATTERY).await?.json().await
}
