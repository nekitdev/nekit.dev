use dioxus::prelude::*;

#[component]
pub fn Skill(display: String, color: String, dot: String) -> Element {
    rsx! {
        li {
            class: "w-auto px-4 rounded-md {color}",

            div {
                class: "flex items-center justify-center gap-4",

                span {
                    class: "h-4 w-4 rounded-full {dot}"
                }

                h2 {
                    class: "text-xl",
                    { display }
                }
            }
        }
    }
}
