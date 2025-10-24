use dioxus::{fullstack::Redirect, prelude::*};
use non_empty_str::{NonEmptyStr, const_non_empty_str};

pub const CHAT: &NonEmptyStr = const_non_empty_str!("https://discord.com/invite/KtJkbut");

pub const DISCORD: &NonEmptyStr =
    const_non_empty_str!("https://discord.com/users/292448864944783360");

pub const GITHUB: &NonEmptyStr = const_non_empty_str!("https://github.com/nekitdev");
pub const BLUESKY: &NonEmptyStr = const_non_empty_str!("https://bsky.app/profile/nekit.dev");
pub const YOUTUBE: &NonEmptyStr = const_non_empty_str!("https://youtube.com/nekitdev");
pub const REDDIT: &NonEmptyStr = const_non_empty_str!("https://reddit.com/u/nekitdev");
pub const TELEGRAM: &NonEmptyStr = const_non_empty_str!("https://t.me/nekitdev");

pub const FUNDING: &NonEmptyStr = const_non_empty_str!("https://boosty.to/nekitdev");

#[get("/chat")]
pub async fn redirect_chat() -> Result<Redirect> {
    Ok(Redirect::to(CHAT.as_str()))
}

#[get("/discord")]
pub async fn redirect_discord() -> Result<Redirect> {
    Ok(Redirect::to(DISCORD.as_str()))
}

#[get("/github")]
pub async fn redirect_github() -> Result<Redirect> {
    Ok(Redirect::to(GITHUB.as_str()))
}

#[get("/bluesky")]
pub async fn redirect_bluesky() -> Result<Redirect> {
    Ok(Redirect::to(BLUESKY.as_str()))
}

#[get("/youtube")]
pub async fn redirect_youtube() -> Result<Redirect> {
    Ok(Redirect::to(YOUTUBE.as_str()))
}

#[get("/reddit")]
pub async fn redirect_reddit() -> Result<Redirect> {
    Ok(Redirect::to(REDDIT.as_str()))
}

#[get("/telegram")]
pub async fn redirect_telegram() -> Result<Redirect> {
    Ok(Redirect::to(TELEGRAM.as_str()))
}

#[get("/funding")]
pub async fn redirect_funding() -> Result<Redirect> {
    Ok(Redirect::to(FUNDING.as_str()))
}
