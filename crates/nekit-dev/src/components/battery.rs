use dioxus::prelude::*;

use crate::battery::battery;

pub const NOT_AVAILABLE: &str = "n/a";
pub const LOADING: &str = "...";

#[component]
pub fn Battery() -> Element {
    let resource = use_server_future(battery)?;

    rsx! {
        match resource.read().cloned() {
            Some(Ok(battery)) => rsx! {
                span { class: "battery-light dark:battery", style: "--level: {battery}", "{battery}" }
            },
            Some(Err(_)) => rsx! {
                span { class: "text-light-accent dark:text-accent", {NOT_AVAILABLE} }
            },
            None => rsx! {
                span { class: "text-light-muted dark:text-muted", {LOADING} }
            },
        }
    }
}
