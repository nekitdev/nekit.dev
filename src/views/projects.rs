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
                title: "gd",
                description: "Geometry Dash API wrapper written in Rust.",
                repository: "https://github.com/gdpsapp/gd",
                documentation: "https://docs.rs/gd",
            }
            Project {
                title: "graphs",
                description: "Graph data structures and algorithms.",
                repository: "https://github.com/nekitdev/graphs",
                documentation: "https://docs.rs/graphs"
            }
            Project {
                title: "refinement-types",
                description: "Refinement types.",
                repository: "https://github.com/nekitdev/refinement-types",
                documentation: "https://docs.rs/refinement-types"
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
