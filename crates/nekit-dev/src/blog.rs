use dioxus::prelude::*;

use nekit_blog::{blog::Summaries, post::Post, slug::Slug};

#[cfg(feature = "server")]
use crate::sync::blog;

#[get("/read")]
pub async fn read() -> Result<Summaries> {
    let summaries = blog().summaries();

    Ok(summaries)
}

#[get("/read/{slug}")]
pub async fn read_post(slug: Slug) -> Result<Option<Post>> {
    let option = blog().read(slug);

    Ok(option)
}
