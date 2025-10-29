use dioxus::{fullstack::*, prelude::*};

pub const PERSONAL: Asset = asset!("/assets/keys/personal.key");
pub const SECURITY: Asset = asset!("/assets/keys/security.key");

pub fn resolve(asset: &Asset) -> String {
    asset.resolve().to_string_lossy().into_owned()
}

#[get("/keys/security")]
pub async fn security() -> Result<Redirect> {
    Ok(Redirect::to(resolve(&SECURITY).as_str()))
}

#[get("/keys/personal")]
pub async fn personal() -> Result<Redirect> {
    Ok(Redirect::to(resolve(&PERSONAL).as_str()))
}
