use rmcp::{
    Json, ServerHandler,
    handler::server::tool::ToolRouter,
    model::{Implementation, ServerCapabilities, ServerConfig},
    tool, tool_handler, tool_router,
};

use crate::{
    health::POISONED,
    time::{View, view},
    types::{Health, State as HealthState},
};

#[derive(Debug, Clone)]
pub struct Me {
    health: HealthState,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl Me {
    pub fn new(health: HealthState) -> Self {
        let tool_router = Self::tool_router();

        Self {
            health,
            tool_router,
        }
    }

    #[tool(
        name = "time",
        description = "Returns nekit's current date in time (UTC+3). \
            Use it when you need to know what time or day it is for him.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    pub async fn time(&self) -> Json<View> {
        Json(view())
    }

    #[tool(
        name = "health",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    pub async fn health(&self) -> Json<Health> {
        let guard = self.health.lock().expect(POISONED);

        let health = *guard;

        drop(guard);

        Json(health)
    }
}

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const INSTRUCTIONS: &str = "Tools about nekit: his local time and his public health stats.";

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Me {
    fn get_info(&self) -> ServerConfig {
        let capabilities = ServerCapabilities::builder().enable_tools().build();

        let implementation = Implementation::new(NAME, VERSION);

        ServerConfig::new(capabilities)
            .with_server_info(implementation)
            .with_instructions(INSTRUCTIONS)
    }
}
