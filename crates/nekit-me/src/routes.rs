use std::sync::Arc;

use axum::{Router, extract::State, routing::get};
use rmcp::transport::{
    StreamableHttpServerConfig, StreamableHttpService,
    streamable_http_server::session::local::LocalSessionManager,
};

use crate::{
    health::{AppJson, AppState, Data, Error, Output},
    mcp::Me,
    types::{Health, State as HealthState},
};

pub async fn get_health(State(state): State<AppState>) -> Health {
    state.get_health()
}

pub async fn set_health(State(state): State<AppState>, AppJson(data): AppJson<Data>) -> Output {
    if data.key() != state.key() {
        return Err(Error::invalid_key());
    }

    let health = data.health();

    state.set_health(health);

    Ok(health)
}

pub const ME_HOST: &str = "me.nekit.dev";
pub const LOCAL_NAME: &str = "localhost";
pub const LOCAL_HOST: &str = "127.0.0.1";

pub const COUNT: usize = 3;

pub const ALLOWED_HOSTS: [&str; COUNT] = [ME_HOST, LOCAL_NAME, LOCAL_HOST];

pub type MeService = StreamableHttpService<Me, LocalSessionManager>;

pub fn mcp_service(health: HealthState) -> MeService {
    let config = StreamableHttpServerConfig::default().with_allowed_hosts(ALLOWED_HOSTS);

    let manager = LocalSessionManager::default();

    let wrapped = Arc::new(manager);

    let factory = move || {
        let me = Me::new(health.clone());

        Ok(me)
    };

    StreamableHttpService::new(factory, wrapped, config)
}

pub fn router<K: AsRef<str>>(key: K) -> Router {
    let state = AppState::new(key);

    let health = state.health.clone();

    let service = mcp_service(health);

    Router::new()
        .route("/health", get(get_health).put(set_health))
        .nest_service("/mcp", service)
        .with_state(state)
}
