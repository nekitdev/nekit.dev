use dioxus::{fullstack::Redirect, prelude::*};

#[allow(unused_imports)]
use crate::urls::static_route;

pub const DISCORD: &str = "https://discord.com/invite/KtJkbut";

pub const GITHUB: &str = "https://github.com/nekitdev";

// NOTE: `%23` is an encoded `#`, since fragments can't contain raw `#` per the URL spec

pub const SUPPORT: &str = "https://matrix.to/#/%23support:nekit.dev";
pub const SPACE: &str = "https://matrix.to/#/%23nekit:nekit.dev";

pub const MATRIX: &str = "https://matrix.to/#/@nekit:nekit.dev";

pub const TWITTER: &str = "https://twitter.com/nekitdev";

pub const BLUESKY: &str = "https://bsky.app/profile/nekit.dev";
pub const YOUTUBE: &str = "https://youtube.com/nekitdev";
pub const REDDIT: &str = "https://reddit.com/u/nekitdev";
pub const CHANNEL: &str = "https://t.me/nekitworks";
pub const TELEGRAM: &str = "https://t.me/nekitdev";

pub const LIZA: &str = "https://dualizzzm.design/";
pub const LYRICHAR: &str = "https://lyrichar.app/";
pub const JIYU: &str = "https://jiyu.sh/";

#[get("/keys/{name}")]
pub async fn redirect_keys(name: String) -> Result<Redirect> {
    let key = static_route(format!("keys/{name}.key"));

    Ok(Redirect::to(key.as_str()))
}

#[get("/resume/{language}")]
pub async fn redirect_resume(language: String) -> Result<Redirect> {
    let resume = static_route(format!("resume/{language}.pdf"));

    Ok(Redirect::to(resume.as_str()))
}

#[get("/email/{name}")]
pub async fn redirect_email(name: String) -> Result<Redirect> {
    let email = format!("mailto:{name}@nekit.dev");

    Ok(Redirect::to(email.as_str()))
}

#[get("/discord")]
pub async fn redirect_discord() -> Result<Redirect> {
    Ok(Redirect::to(DISCORD))
}

#[get("/github")]
pub async fn redirect_github() -> Result<Redirect> {
    Ok(Redirect::to(GITHUB))
}

#[get("/space")]
pub async fn redirect_space() -> Result<Redirect> {
    Ok(Redirect::to(SPACE))
}

#[get("/chat")]
pub async fn redirect_chat() -> Result<Redirect> {
    Ok(Redirect::to(SUPPORT)) // TODO: migrate already! >~>
}

#[get("/support")]
pub async fn redirect_support() -> Result<Redirect> {
    Ok(Redirect::to(SUPPORT))
}

#[get("/matrix")]
pub async fn redirect_matrix() -> Result<Redirect> {
    Ok(Redirect::to(MATRIX))
}

#[get("/bluesky")]
pub async fn redirect_bluesky() -> Result<Redirect> {
    Ok(Redirect::to(BLUESKY))
}

#[get("/twitter")]
pub async fn redirect_twitter() -> Result<Redirect> {
    Ok(Redirect::to(TWITTER))
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

#[get("/liza")]
pub async fn redirect_liza() -> Result<Redirect> {
    Ok(Redirect::to(LIZA))
}

#[get("/lyrichar")]
pub async fn redirect_lyrichar() -> Result<Redirect> {
    Ok(Redirect::to(LYRICHAR))
}

#[get("/jiyu")]
pub async fn redirect_jiyu() -> Result<Redirect> {
    Ok(Redirect::to(JIYU))
}
