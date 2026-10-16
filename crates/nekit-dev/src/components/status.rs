use dioxus::prelude::*;

use crate::{
    components::{battery::Battery, clock::Clock, keys::HelpToggle, shared::COLUMN},
    times,
};

#[component]
pub fn Status() -> Element {
    let year = times::year();

    rsx! {
        footer { class: "
            bg-light-bar dark:bg-bar border-light-line dark:border-line
            text-light-muted dark:text-muted border-t-2",
            div { class: "{COLUMN} px-[4ch] py-2 flex flex-wrap justify-between gap-x-[4ch] gap-y-1",
                div { class: "flex flex-wrap gap-x-[2ch]",
                    span {
                        "["
                        HelpToggle {}
                        "]"
                    }
                    span { class: "text-light-main dark:text-main", "nekit.dev" }
                    Clock {}
                    span {
                        "["
                        Battery {}
                        "]"
                    }
                    span { class: "text-light-accent dark:text-accent", "<3" }
                }
                span { "(c) {year} nekitdev" }
            }
        }
    }
}
