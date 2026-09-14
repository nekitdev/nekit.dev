use axum::{Router, extract::State, routing::get};

use crate::{
    battery::{AppJson, AppState, Data, Error, Output},
    level::Level,
};

pub async fn get_level(State(state): State<AppState>) -> Level {
    state.get_level()
}

pub async fn set_level(State(state): State<AppState>, AppJson(data): AppJson<Data>) -> Output {
    if data.key() != state.key() {
        return Err(Error::invalid_key());
    }

    let level = data.level();

    state.set_level(level);

    Ok(level)
}

pub fn router<K: AsRef<str>>(key: K) -> Router {
    let state = AppState::new(key);

    Router::new()
        .route("/", get(get_level).patch(set_level))
        .with_state(state)
}
