use dioxus::{fullstack::Redirect, prelude::*};

pub const CHAT: &str = "https://discord.com/invite/KtJkbut";

pub const DISCORD: &str = "https://discord.com/users/292448864944783360";

pub const GITHUB: &str = "https://github.com/nekitdev";
pub const BLUESKY: &str = "https://bsky.app/profile/nekit.dev";
pub const YOUTUBE: &str = "https://youtube.com/nekitdev";
pub const REDDIT: &str = "https://reddit.com/u/nekitdev";
pub const TELEGRAM: &str = "https://t.me/nekitdev";

pub const FUNDING: &str = "https://boosty.to/nekitdev";

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

#[get("/telegram")]
pub async fn redirect_telegram() -> Result<Redirect> {
    Ok(Redirect::to(TELEGRAM))
}

#[get("/funding")]
pub async fn redirect_funding() -> Result<Redirect> {
    Ok(Redirect::to(FUNDING))
}
