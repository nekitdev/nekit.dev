use dioxus::prelude::*;

use non_empty_str::{NonEmptyStr, const_non_empty_str};

use crate::{components::heading::Heading, host::Host};

pub const NOT_AVAILABLE: &NonEmptyStr = const_non_empty_str!("n/a");

pub const SYSTEM: &str = "system";

#[component]
pub fn System(host: Host) -> Element {
    let name = host.name().unwrap_or(NOT_AVAILABLE);
    let system = host.system().unwrap_or(NOT_AVAILABLE);

    rsx! {
        section { aria_labelledby: SYSTEM,
            Heading { id: SYSTEM, title: SYSTEM }

            dl { class: "grid grid-cols-[max-content_1fr] gap-x-[3ch] gap-y-1",
                dt { class: "text-light-accent dark:text-accent", "os" }
                dd { "{system}" }
                dt { class: "text-light-accent dark:text-accent", "name" }
                dd { "{name}" }
                dt { class: "text-light-accent dark:text-accent", "cores" }
                dd { "{host.cores()}" }
                dt { class: "text-light-accent dark:text-accent", "uptime" }
                dd { "{host.uptime()}" }
                dt { class: "text-light-accent dark:text-accent", "load" }
                dd { "{host.load()}" }
            }
        }
    }
}
