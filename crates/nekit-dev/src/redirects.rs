use dioxus::{fullstack::Redirect, prelude::*};

use crate::urls::static_route;

pub const CHAT: &str = "https://discord.com/invite/KtJkbut";

pub const DISCORD: &str = "https://discord.com/users/292448864944783360";

pub const DOCS: &str = "https://docs.rs";

pub const GITHUB: &str = "https://github.com/nekitdev";
pub const BLUESKY: &str = "https://bsky.app/profile/nekit.dev";
pub const YOUTUBE: &str = "https://youtube.com/nekitdev";
pub const REDDIT: &str = "https://reddit.com/u/nekitdev";
pub const CHANNEL: &str = "https://t.me/nekitworks";
pub const TELEGRAM: &str = "https://t.me/nekitdev";

pub const FUNDING: &str = "https://boosty.to/nekitdev";

#[get("/keys/{name}")]
pub async fn redirect_keys(name: String) -> Result<Redirect> {
    let key = static_route(format!("/keys/{name}.key"));

    Ok(Redirect::to(key.as_str()))
}

#[get("/resume/{language}")]
pub async fn redirect_resume(language: String) -> Result<Redirect> {
    let resume = static_route(format!("/resume/{language}.pdf"));

    Ok(Redirect::to(resume.as_str()))
}

#[get("/docs/{name}")]
pub async fn redirect_docs(name: String) -> Result<Redirect> {
    let docs = format!("{DOCS}/{name}");

    Ok(Redirect::to(docs.as_str()))
}

#[get("/email/{name}")]
pub async fn redirect_email(name: String) -> Result<Redirect> {
    let email = format!("mailto:{name}@nekit.dev");

    Ok(Redirect::to(email.as_str()))
}

#[get("/chat")]
pub async fn redirect_chat() -> Result<Redirect> {
    Ok(Redirect::to(CHAT))
}

#[get("/discord")]
pub async fn redirect_discord() -> Result<Redirect> {
    Ok(Redirect::to(DISCORD))
}

#[get("/github")]
pub async fn redirect_github() -> Result<Redirect> {
    Ok(Redirect::to(GITHUB))
}

#[get("/bluesky")]
pub async fn redirect_bluesky() -> Result<Redirect> {
    Ok(Redirect::to(BLUESKY))
}

#[get("/youtube")]
pub async fn redirect_youtube() -> Result<Redirect> {
    Ok(Redirect::to(YOUTUBE))
}

#[get("/reddit")]
pub async fn redirect_reddit() -> Result<Redirect> {
    Ok(Redirect::to(REDDIT))
}

#[get("/channel")]
pub async fn redirect_channel() -> Result<Redirect> {
    Ok(Redirect::to(CHANNEL))
}

#[get("/telegram")]
pub async fn redirect_telegram() -> Result<Redirect> {
    Ok(Redirect::to(TELEGRAM))
}

#[get("/funding")]
pub async fn redirect_funding() -> Result<Redirect> {
    Ok(Redirect::to(FUNDING))
}
