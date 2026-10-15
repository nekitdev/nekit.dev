use dioxus::prelude::*;

#[component]
pub fn Heading(id: String, title: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-[2ch] mb-4 border-b-2 border-light-line dark:border-line",
            h2 { class: "font-bold text-light-muted dark:text-muted", id, {title} }
        }
    }
}
