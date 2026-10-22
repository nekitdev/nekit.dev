use dioxus::prelude::*;

use crate::{components::heading::Heading, data::Group};

pub const ELSEWHERE: &str = "elsewhere";

#[component]
pub fn Elsewhere() -> Element {
    rsx! {
        section { aria_labelledby: ELSEWHERE,
            Heading { id: ELSEWHERE, title: ELSEWHERE }

            div { class: "grid grid-cols-[repeat(auto-fit,minmax(18ch,1fr))] gap-x-[4ch] gap-y-6",
                for group in Group::all() {
                    div { class: "flex flex-col",
                        div { class: "text-light-muted dark:text-muted", "{group.name}/" }
                        for link in group.links {
                            div { class: "py-1",
                                a {
                                    class: "hover:underline",
                                    href: "{link.path}",
                                    "{link.name}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
