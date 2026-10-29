use dioxus::prelude::*;

use crate::theme::use_themes;

pub const THEME: &str = "theme";

#[component]
pub fn ThemeSwitch() -> Element {
    let mut themes = use_themes();

    let name = themes.get().name();

    rsx! {
        button {
            class: "cursor-pointer hover:underline",
            r#type: "button",
            aria_label: "{THEME}: {name}",
            onclick: move |_| themes.cycle(),
            span { class: "text-light-accent dark:text-accent", {name} }
        }
    }
}
