use std::path::Path;

use axum::Router;
use tower_http::services::ServeDir;

pub fn router<D: AsRef<Path>>(directory: D) -> Router {
    Router::new().nest_service("/static", ServeDir::new(directory))
}
