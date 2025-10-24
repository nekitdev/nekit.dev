// use axum::response::Redirect;
use dioxus::prelude::*;

use crate::views::home::Home;

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
}

#[component]
pub fn App() -> Element {
    rsx! {
        body {
            class: "
                antialiased
                transition
                ease-in-out
                min-h-screen
                bg-neutral-50 dark:bg-neutral-900
                text-neutral-900 dark:text-neutral-50
                font-mono
            ",
            Router::<Route> {}
        }
    }
}
