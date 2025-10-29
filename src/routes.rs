use dioxus::prelude::*;

use crate::views::{
    blog::Blog, home::Home, not_found::NotFound, projects::Projects, security::Security,
};

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/projects")]
    Projects {},
    #[route("/security")]
    Security {},
    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        main {
            Router::<Route> {}
        }
    }
}
