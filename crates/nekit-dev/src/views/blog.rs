use dioxus::prelude::*;

use crate::components::shared::COLUMN;

pub const BLOG: &str = "blog";
pub const WIP: &str = "Work in progress... (._.')/";

#[component]
pub fn Blog() -> Element {
    rsx! {
        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-18",
            div { class: "flex flex-col gap-2",
                div { class: "text-light-muted dark:text-muted", {BLOG} }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight", {WIP} }
            }
        }
    }
}
