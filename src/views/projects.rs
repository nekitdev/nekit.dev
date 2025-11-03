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
                name: "gd",
            }
            Project {
                title: "graphs",
                description: "Graph data structures and algorithms.",
                name: "graphs",
            }
            Project {
                title: "refinement-types",
                description: "Refinement types.",
                name: "refinement-types",
            }
            Project {
                title: "changelogging",
                description: "Building changelogs from fragments.",
                name: "changelogging",
            }
        }
    }
}
