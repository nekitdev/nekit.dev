use dioxus::prelude::*;

pub const SPINNER: &str = "|/-\\";

#[component]
pub fn Spinner() -> Element {
    rsx! {
        span {
            class: "inline-block w-[1ch] overflow-hidden align-bottom",
            role: "status",
            aria_label: "loading",
            span {
                class: "inline-block whitespace-pre animate-spinner motion-reduce:animate-none",
                aria_hidden: "true",
                {SPINNER}
            }
        }
    }
}
