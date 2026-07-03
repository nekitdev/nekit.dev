use dioxus::{fullstack::*, prelude::*};

use crate::urls;

pub const EN: Asset = asset!("assets/resume/en.pdf");
pub const RU: Asset = asset!("assets/resume/ru.pdf");

#[get("/resume/en")]
pub async fn redirect_resume_en() -> Result<Redirect> {
    let string = urls::resolve(EN);

    Ok(Redirect::to(string.as_str()))
}

#[get("/resume/ru")]
pub async fn redirect_resume_ru() -> Result<Redirect> {
    let string = urls::resolve(RU);

    Ok(Redirect::to(string.as_str()))
}
