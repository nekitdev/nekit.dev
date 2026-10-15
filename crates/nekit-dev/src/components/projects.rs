use dioxus::prelude::*;

use crate::data::Project;

pub const PROJECTS: &str = "projects";

#[component]
pub fn Projects() -> Element {
    rsx! {
        section { aria_labelledby: PROJECTS,
            div { class: "flex items-center gap-[2ch] mb-4 border-b-2 border-light-line dark:border-line",
                h2 {
                    class: "font-bold text-light-muted dark:text-muted",
                    id: PROJECTS,
                    {PROJECTS}
                }
            }
            div { class: "overflow-x-auto",
                table { class: "w-full min-w-[60ch] border-collapse",
                    thead {
                        tr { class: "text-left text-light-muted dark:text-muted",
                            th { class: "pb-2 pr-[3ch]", scope: "col", "name" }
                            th { class: "pb-2", scope: "col", "description" }
                        }
                    }
                    tbody {
                        for project in Project::all() {
                            tr {
                                td { class: "py-1 pr-[3ch] whitespace-nowrap",
                                    a {
                                        class: "text-light-accent dark:text-accent hover:underline",
                                        href: "{project.repository}",
                                        "{project.name}"
                                    }
                                }
                                td { class: "py-1",
                                    "["
                                    span {
                                        class: "text-light-accent dark:text-accent",
                                        title: "{project.status.label()}",
                                        "{project.status.symbol()}"
                                    }
                                    "]"
                                    " "
                                    "{project.description}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
