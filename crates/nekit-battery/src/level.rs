use std::{
    fmt,
    sync::{Arc, Mutex},
};

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use refining::prelude::{Refinement, u8};
use serde::{Deserialize, Serialize};

pub const MIN: u8 = 0;
pub const MAX: u8 = 100;

pub type Range = u8::Closed<MIN, MAX>;
pub type Refined = Refinement<u8, Range>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct Level {
    refined: Refined,
}

impl Default for Level {
    fn default() -> Self {
        // SAFETY: the default value is within the valid range
        let refined = unsafe { Refined::unchecked(MIN) };

        Self::new(refined)
    }
}

impl fmt::Display for Level {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{refined}%", refined = self.refined())
    }
}

impl Level {
    pub const fn new(refined: Refined) -> Self {
        Self { refined }
    }

    pub const fn refined(&self) -> Refined {
        self.refined
    }
}

impl IntoResponse for Level {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub type State = Arc<Mutex<Level>>;

impl Level {
    pub fn into_state(self) -> State {
        Arc::new(Mutex::new(self))
    }
}
