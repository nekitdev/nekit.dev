use dioxus::prelude::*;

use crate::{
    components::{head::Head, status::Status, tabs::Tabs},
    routes::Route,
};

#[component]
pub fn Shell() -> Element {
    let route = use_route::<Route>();

    let info = route.info();

    let active = route.tab();

    rsx! {
        Head { info }

        div { class: "min-h-screen flex flex-col",
            Tabs { active }

            Outlet::<Route> {}

            Status {}
        }
    }
}
