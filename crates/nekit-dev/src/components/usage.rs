use dioxus::prelude::*;

use crate::{
    components::{bar::Bar, heading::Heading},
    host::{Host, Temperature},
};

pub const USAGE: &str = "usage";

pub const CELLS: usize = 20;

pub fn format_temperature(temperature: Temperature) -> String {
    format!("{temperature:#}")
}

#[component]
pub fn Usage(host: Host) -> Element {
    let cpu = host.display_cpu();
    let memory = host.display_memory();
    let swap = host.display_swap();

    let temperature = host.display_temperature();

    let cpu_percent = host.cpu_percent().unwrap_or_default();
    let memory_percent = host.memory_percent().unwrap_or_default();
    let swap_percent = host.swap_percent().unwrap_or_default();

    rsx! {
        section { aria_labelledby: USAGE,
            Heading { id: USAGE, title: USAGE }
            dl { class: "grid grid-cols-[max-content_1fr] gap-x-[3ch] gap-y-1",
                dt { class: "text-light-accent dark:text-accent", "cpu" }
                dd {
                    Bar { percent: cpu_percent, cells: CELLS, content: "{cpu}" }
                }
                dt { class: "text-light-accent dark:text-accent", "memory" }
                dd {
                    Bar {
                        percent: memory_percent,
                        cells: CELLS,
                        content: "{memory}",
                    }
                }
                dt { class: "text-light-accent dark:text-accent", "swap" }
                dd {
                    Bar {
                        percent: swap_percent,
                        cells: CELLS,
                        content: "{swap}",
                    }
                }
                dt { class: "text-light-accent dark:text-accent", "temp" }
                dd { "{temperature}" }
            }
        }
    }
}
