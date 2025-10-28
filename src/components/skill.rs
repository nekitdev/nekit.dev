use dioxus::prelude::*;

#[component]
pub fn Skill(display: String, detail: String, name: String, color: String) -> Element {
    rsx! {
        li {
            class: "px-4 rounded-md {color}",

            div {
                class: "flex items-center justify-center gap-4",

                span {
                    class: "fa-brands fa-{name} text-4xl"
                }

                div {
                    class: "flex flex-col",

                    h2 {
                        class: "text-xl",
                        { display }
                    }

                    p {
                        { detail }
                    }
                }
            }
        }
    }
}
