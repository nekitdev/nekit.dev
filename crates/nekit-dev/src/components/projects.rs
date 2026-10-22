use dioxus::prelude::*;

use crate::{components::heading::Heading, data::Project};

pub const PROJECTS: &str = "projects";

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { aria_labelledby: PROJECTS,
            Heading { id: PROJECTS, title: PROJECTS }

            dl { class: "grid grid-cols-1 sm:grid-cols-[max-content_1fr] gap-x-[clamp(2ch,4vw,8ch)] gap-y-1",
                for project in Project::all() {
                    dt {
                        a {
                            class: "text-light-accent dark:text-accent hover:underline",
                            href: "{project.repository}",
                            "{project.name}"
                        }
                    }
                    dd { class: "flex gap-[1ch] mb-3 sm:mb-0",
                        span { class: "shrink-0",
                            "["
                            span {
                                class: "text-light-accent dark:text-accent",
                                title: "{project.status.label()}",
                                "{project.status.symbol()}"
                            }
                            "]"
                        }
                        span { "{project.description}" }
                    }
                }
            }
        }
    }
}
