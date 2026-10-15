use dioxus::prelude::*;

use crate::{data::Fact, times, urls};

pub const NEKO: &str = "/images/neko.png";

#[component]
pub fn Fetch() -> Element {
    let age = times::age();

    let neko = urls::static_route(NEKO);

    rsx! {
        section {
            class: "flex flex-wrap items-start, gap-x-[5ch], gap-y-8",
            aria_label: "About",
            img {
                class: "w-[22ch] aspect-square object-contain",
                src: neko,
                alt: "Neko",
            }
            div { class: "flex flex-col gap-1 min-w-0",
                div { class: "font-bold border-b-2 border-light-line dark:border-line",
                    span { class: "text-light-accent dark:text-accent", "nekit" }
                    "@dev"
                }
                dl { class: "grid grid-cols-[max-content_1fr] gap-x-[3ch]",
                    for fact in Fact::all(age) {
                        dt { class: "text-light-accent dark:text-accent", "{fact.title}" }
                        dd { "{fact.value}" }
                    }
                }
            }
        }
    }
}
