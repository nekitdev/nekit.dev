use dioxus::prelude::*;

use crate::{
    components::shared::COLUMN,
    routes::{Route, Tab},
};

pub const CLASS: &str = "flex items-center min-h-10 px-[2ch]";

#[component]
pub fn Tabs(active: Option<Tab>) -> Element {
    rsx! {
        nav {
            class: "border-b-2 border-light-line dark:border-line",
            aria_label: "Tabs",
            div { class: "{COLUMN} px-[2ch] flex flex-wrap",
                for tab in Tab::ALL {
                    Link {
                        class: if Some(tab) == active { "{CLASS} bg-light-accent dark:bg-accent text-light-front dark:text-front" } else { "{CLASS} text-light-muted dark:text-muted" },
                        to: match tab {
                            Tab::Home => Route::Home {},
                            Tab::Blog => Route::Blog {},
                            Tab::Security => Route::Security {},
                        },
                        "{tab.name()}"
                    }
                }
            }
        }
    }
}
