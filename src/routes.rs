use axum::{response::Redirect, routing::get, Router};
use maud::Markup;

use crate::templates::index::index;

async fn get_index() -> Markup {
    index()
}

pub const CHAT: &str = "https://discord.com/invite/KtJkbut";

pub const DISCORD: &str = "https://discord.com/users/292448864944783360";

pub const GITHUB: &str = "https://github.com/nekitdev";
pub const BLUESKY: &str = "https://bsky.app/profile/nekit.dev";
pub const YOUTUBE: &str = "https://youtube.com/nekitdev";
pub const REDDIT: &str = "https://reddit.com/u/nekitdev";
pub const TELEGRAM: &str = "https://t.me/nekitdev";

pub const FUNDING: &str = "https://boosty.to/nekitdev";

async fn redirect_chat() -> Redirect {
    Redirect::to(CHAT)
}

async fn redirect_discord() -> Redirect {
    Redirect::to(DISCORD)
}

async fn redirect_github() -> Redirect {
    Redirect::to(GITHUB)
}

async fn redirect_bluesky() -> Redirect {
    Redirect::to(BLUESKY)
}

async fn redirect_youtube() -> Redirect {
    Redirect::to(YOUTUBE)
}

async fn redirect_reddit() -> Redirect {
    Redirect::to(REDDIT)
}

async fn redirect_telegram() -> Redirect {
    Redirect::to(TELEGRAM)
}

async fn redirect_funding() -> Redirect {
    Redirect::to(FUNDING)
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/", get(get_index))
        .route("/chat", get(redirect_chat))
        .route("/discord", get(redirect_discord))
        .route("/github", get(redirect_github))
        .route("/bluesky", get(redirect_bluesky))
        .route("/youtube", get(redirect_youtube))
        .route("/reddit", get(redirect_reddit))
        .route("/telegram", get(redirect_telegram))
        .route("/funding", get(redirect_funding))
}
