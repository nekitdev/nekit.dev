use dioxus::prelude::*;

use crate::battery::get_battery;

pub const ERROR: &str = "error";
pub const LOADING: &str = "...";

#[component]
pub fn Battery() -> Element {
    let resource = use_resource(get_battery);

    rsx! {
        match &*resource.read_unchecked() {
            Some(Ok(battery)) => rsx! {
                div {
                    class: "flex items-center justify-center gap-2",
                    span {
                        class: "fa-solid {battery.name()} hover:text-melody-blue dark:hover:text-melody-blue h-4 w-4"
                    }
                    h3 {
                        class: "text-xl hover:text-melody-blue dark:hover:text-melody-blue",
                        "{battery}"
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
