use dioxus::prelude::*;

use crate::{
    components::{bar::Bar, heading::Heading},
    host::{Host, Temperature},
};

pub const USAGE: &str = "usage";

pub const CELLS: usize = 24;

pub const NOT_AVAILABLE: &str = "n/a";

pub fn not_available() -> String {
    NOT_AVAILABLE.to_owned()
}

pub fn format_temperature(temperature: Temperature) -> String {
    format!("{temperature:#}")
}

#[component]
pub fn Usage(host: Host) -> Element {
    let cpu = host.cpu();
    let memory = host.memory();
    let swap = host.swap();

    let temperature = host
        .temperature()
        .map_or_else(not_available, format_temperature);

    rsx! {
        section { aria_labelledby: USAGE,
            Heading { id: USAGE, title: USAGE }
            dl { class: "grid grid-cols-[max-content_1fr] gap-x-[3ch] gap-y-1",
                dt { class: "text-light-accent dark:text-accent", "cpu" }
                dd {
                    Bar { percent: cpu, cells: CELLS, content: "{cpu:#}" }
                }
                dt { class: "text-light-accent dark:text-accent", "memory" }
                dd {
                    Bar {
                        percent: memory.percent(),
                        cells: CELLS,
                        content: "{memory}",
                    }
                }
                dt { class: "text-light-accent dark:text-accent", "swap" }
                dd {
                    Bar {
                        percent: swap.percent(),
                        cells: CELLS,
                        content: "{swap}",
                    }
                }
                dt { class: "text-light-accent dark:text-accent", "temperature" }
                dd { {temperature} }
            }
        }
    }
}
