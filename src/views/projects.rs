use dioxus::prelude::*;

use crate::{
    components::{head::Head, project::Project},
    urls,
};

pub const TITLE: &str = "Projects";
pub const DESCRIPTION: &str = "All projects.";
pub const ROUTE: &str = "projects";

#[component]
pub fn Projects() -> Element {
    rsx! {
        Head {
            title: TITLE,
            description: DESCRIPTION,
            url: urls::route(ROUTE),
        }
        ul {
            class: "flex flex-row flex-wrap gap-5",
            Project {
                title: "lyrichar",
                description: "Lyrics at the speed of thought.",
                repository: "https://github.com/lyrichar/lyrichar",
                documentation: "https://docs.rs/lyrichar",
            }
            Project {
                title: "gd",
                description: "Geometry Dash API wrapper written in Rust.",
                repository: "https://github.com/gdpsapp/gd",
                documentation: "https://docs.rs/gd",
            }
            Project {
                title: "nixos-config",
                description: "NixOS configuration.",
                repository: "https://github.com/nekitdev/nixos-config",
            }
            Project {
                title: "graphs",
                description: "Graph data structures and algorithms.",
                repository: "https://github.com/nekitdev/graphs",
                documentation: "https://docs.rs/graphs"
            }
            Project {
                title: "refining",
                description: "Refinement types.",
                repository: "https://github.com/nekitdev/refining",
                documentation: "https://docs.rs/refining"
            }
            Project {
                title: "ownership",
                description: "Obtaining ownership.",
                repository: "https://github.com/nekitdev/ownership",
                documentation: "https://docs.rs/ownership"
            }
            Project {
                title: "trait-aliases",
                description: "Trait aliases.",
                repository: "https://github.com/nekitdev/trait-aliases",
                documentation: "https://docs.rs/trait-aliases"
            }
            Project {
                title: "changelogging",
                description: "Building changelogs from fragments.",
                repository: "https://github.com/nekitdev/changelogging",
                documentation: "https://docs.rs/changelogging"
            }
        }
    }
}
