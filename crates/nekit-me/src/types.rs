use std::{
    fmt,
    sync::{Arc, Mutex},
};

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Samples {
    value: u16,
}

impl fmt::Display for Samples {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value} samples", value = self.get())
    }
}

impl Samples {
    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    pub const fn get(self) -> u16 {
        self.value
    }

    pub const fn is_empty(self) -> bool {
        self.get() == 0
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Rate {
    value: u16,
}

impl fmt::Display for Rate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value} bpm", value = self.get())
    }
}

impl Rate {
    pub const fn new(value: u16) -> Self {
        Self { value }
    }

    pub const fn get(self) -> u16 {
        self.value
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Steps {
    value: u32,
}

impl fmt::Display for Steps {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value} steps", value = self.get())
    }
}

impl Steps {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn get(self) -> u32 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub struct Health {
    /// The number of heart rate samples.
    pub samples: Samples,
    /// The average heart rate over the last hour.
    ///
    /// Check the `samples` if this is zero, as the rate may be invalid.
    pub rate: Rate,
    /// The number of steps taken today.
    pub steps: Steps,
}

impl IntoResponse for Health {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub type State = Arc<Mutex<Health>>;

impl Health {
    pub fn into_state(self) -> State {
        Arc::new(Mutex::new(self))
    }

    pub const fn samples(self) -> Samples {
        self.samples
    }

    pub const fn rate(self) -> Rate {
        self.rate
    }

    pub const fn steps(self) -> Steps {
        self.steps
    }

    pub const fn samples_empty(&self) -> bool {
        self.samples().is_empty()
    }
}
