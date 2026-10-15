use std::{env, fmt};

use dioxus::prelude::*;
use refining::prelude::{Refinement, u8};
use reqwest::{Error as ReqwestError, get};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const BATTERY: &str = "https://battery.nekit.dev/";
pub const NEKIT_BATTERY_URL: &str = "NEKIT_BATTERY_URL";

pub fn battery_url() -> String {
    env::var(NEKIT_BATTERY_URL).unwrap_or_else(|_| BATTERY.to_owned())
}

pub type Level = Refinement<u8, u8::Closed<0, 100>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("failed to get battery")]
pub struct Error;

impl From<ReqwestError> for Error {
    fn from(_error: ReqwestError) -> Self {
        Self
    }
}

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

pub async fn get_battery() -> Result<Battery, Error> {
    let battery = get(battery_url()).await?.json().await?;

    Ok(battery)
}

#[get("/battery")]
pub async fn battery() -> Result<Battery> {
    let battery = get_battery().await?;

    Ok(battery)
}
