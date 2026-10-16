use dioxus::prelude::*;

use crate::{
    components::{head::Head, keys::Shortcuts, status::Status, tabs::Tabs},
    keys::provide_help,
    routes::Route,
};

#[component]
pub fn Shell() -> Element {
    provide_help();

    let route = use_route::<Route>();

    let info = route.info();

    let active = route.tab();

    rsx! {
        Head { info }

        div { class: "min-h-screen flex flex-col",
            Tabs { active }

            Outlet::<Route> {}

            Status {}

            Shortcuts {}
        }
    }
}
