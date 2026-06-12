use dioxus::prelude::*;

use crate::battery::get_level;

pub const ERROR: &str = "error";
pub const LOADING: &str = "...";

#[component]
pub fn Battery() -> Element {
    let resource = use_resource(get_level);

    rsx! {
        match &*resource.read_unchecked() {
            Some(Ok(level)) => rsx! {
                div {
                    class: "flex items-center justify-center gap-2",
                    span {
                        class: "fa-solid {level.name()} hover:text-melody-blue dark:hover:text-melody-blue h-4 w-4"
                    }
                    h3 {
                        class: "text-xl hover:text-melody-blue dark:hover:text-melody-blue",
                        "{level}"
                    }
                }
            },
            Some(Err(_)) => rsx! {
                div {
                    class: "flex items-center justify-center gap-2",
                    span {
                        class: "fa-solid fa-exclamation hover:text-melody-blue dark:hover:text-melody-purple h-4 w-4"
                    }
                    h3 {
                        class: "text-xl hover:text-melody-blue dark:hover:text-melody-blue",
                        { ERROR }
                    }
                }
            },
            None => rsx! {
                div {
                    class: "flex items-center justify-center gap-2",
                    span {
                        class: "fa-solid fa-circle-notch hover:text-melody-blue dark:hover:text-melody-purple h-4 w-4"
                    }
                    h3 {
                        class: "text-xl hover:text-melody-blue dark:hover:text-melody-blue",
                        { LOADING }
                    }
                }
            },
        }
    }
}
