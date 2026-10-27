use dioxus::prelude::*;

use crate::{
    blog::read,
    components::{posts::Posts, shared::COLUMN},
};

pub const BLOG: &str = "blog";
pub const TITLE: &str = "Blog";
pub const EMPTY: &str = "Nothing here yet... (._.')/";
pub const FAILED: &str = "Failed to fetch posts... ;~;";
pub const FETCHING: &str = "Fetching...";

#[component]
pub fn Blog() -> Element {
    let resource = use_server_future(read)?;

    rsx! {
        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-18",
            div { class: "flex flex-col gap-2",
                div { class: "text-light-muted dark:text-muted", {BLOG} }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight", {TITLE} }
            }
            match resource.read().cloned() {
                Some(Ok(summaries)) => rsx! {
                    if summaries.is_empty() {
                        p { class: "text-light-muted dark:text-muted", {EMPTY} }
                    } else {
                        Posts { summaries }
                    }
                },
                Some(Err(_)) => rsx! {
                    p { class: "text-light-muted dark:text-muted", {FAILED} }
                },
                None => rsx! {
                    p { class: "text-light-muted dark:text-muted", {FETCHING} }
                }
            }
        }
    }
}
