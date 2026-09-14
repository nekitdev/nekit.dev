use std::sync::Arc;

use axum::{
    Json,
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::level::{Level, State as LevelState};

pub const INVALID_JSON: &str = "invalid json";
pub const INVALID_KEY: &str = "invalid key";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
#[error("{message}")]
pub struct Error {
    message: String,
}

impl Error {
    pub const fn new(message: String) -> Self {
        Self { message }
    }

    pub fn of<T: ToString>(value: T) -> Self {
        Self::new(value.to_string())
    }

    pub fn invalid_json() -> Self {
        Self::of(INVALID_JSON)
    }

    pub fn invalid_key() -> Self {
        Self::of(INVALID_KEY)
    }
}

impl From<JsonRejection> for Error {
    fn from(_json_rejection: JsonRejection) -> Self {
        Self::invalid_json()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

#[derive(FromRequest)]
#[from_request(via(Json), rejection(Error))]
pub struct AppJson<T>(pub T);

pub type Output = Result<Level, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Data {
    pub level: Level,
    pub key: String,
}

impl Data {
    pub const fn level(&self) -> Level {
        self.level
    }

    pub const fn key(&self) -> &str {
        self.key.as_str()
    }
}

pub type Key = Arc<str>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub level: LevelState,
    pub key: Key,
}

pub const POISONED: &str = "the mutex has been poisoned";

impl AppState {
    pub const fn construct(level: LevelState, key: Key) -> Self {
        Self { level, key }
    }

    pub fn new<K: AsRef<str>>(key: K) -> Self {
        let level = Level::default().into_state();

        let key = key.as_ref().into();

        Self::construct(level, key)
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    pub fn get_level(&self) -> Level {
        let guard = self.level.lock().expect(POISONED);

        *guard
    }

    pub fn set_level(&self, level: Level) {
        let mut guard = self.level.lock().expect(POISONED);

        *guard = level;
    }
}
