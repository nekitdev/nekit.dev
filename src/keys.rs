use dioxus::{fullstack::*, prelude::*};

use crate::urls;

pub const PERSONAL: Asset = asset!("/assets/keys/personal.key");
pub const SECURITY: Asset = asset!("/assets/keys/security.key");

#[get("/keys/security")]
pub async fn security() -> Result<Redirect> {
    let string = urls::resolve(SECURITY);

    Ok(Redirect::to(string.as_str()))
}

#[get("/keys/personal")]
pub async fn personal() -> Result<Redirect> {
    let string = urls::resolve(PERSONAL);

    Ok(Redirect::to(string.as_str()))
}
