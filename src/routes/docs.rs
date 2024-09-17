use axum::{extract::Path, response::Redirect, routing::get, Router};

async fn redirect_docs(Path(name): Path<String>) -> Redirect {
    let url = format!("https://nekitdev.github.io/{name}");

    Redirect::to(&url)
}

pub fn router() -> Router<()> {
    Router::new().route("/:name", get(redirect_docs))
}
