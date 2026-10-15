use dioxus::prelude::*;

use crate::data::Group;

pub const ELSEWHERE: &str = "elsewhere";

#[component]
pub fn Elsewhere() -> Element {
    rsx! {
        section { aria_labelledby: ELSEWHERE,
            div { class: "flex items-center gap-[2ch] mb-4 border-b-2 border-light-line dark:border-line",
                h2 {
                    class: "font-bold text-light-muted dark:text-muted",
                    id: ELSEWHERE,
                    {ELSEWHERE}
                }
            }
            div { class: "grid grid-cols-[repeat(auto-fit,minmax(18ch,1fr))] gap-x-[4ch] gap-y-6",
                for group in Group::all() {
                    div { class: "flex flex-col",
                        div { class: "text-light-muted dark:text-muted", "{group.name}/" }
                        for link in group.links {
                            div { class: "py-1",
                                a { class: "hover:underline", href: "{link.path}", "{link.name}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
