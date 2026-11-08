use dioxus::prelude::*;

use non_empty_str::{NonEmptyStr, const_non_empty_str};

use crate::{components::heading::Heading, host::Host};

pub const NOT_AVAILABLE: &NonEmptyStr = const_non_empty_str!("n/a");

pub const SYSTEM: &str = "system";

#[component]
pub fn System(host: Host) -> Element {
    let name = host.display_name();
    let system = host.display_system();
    let cores = host.display_cores();
    let uptime = host.display_uptime();
    let load = host.display_load();

    rsx! {
        section { aria_labelledby: SYSTEM,
            Heading { id: SYSTEM, title: SYSTEM }

            dl { class: "grid grid-cols-[max-content_1fr] gap-x-[3ch] gap-y-1",
                dt { class: "text-light-accent dark:text-accent", "os" }
                dd { "{system}" }
                dt { class: "text-light-accent dark:text-accent", "name" }
                dd { "{name}" }
                dt { class: "text-light-accent dark:text-accent", "cores" }
                dd { "{cores}" }
                dt { class: "text-light-accent dark:text-accent", "uptime" }
                dd { "{uptime}" }
                dt { class: "text-light-accent dark:text-accent", "load" }
                dd { "{load}" }
            }
        }
    }
}
