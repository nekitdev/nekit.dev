use dioxus::{fullstack::*, prelude::*};

use crate::{components::head::Head, urls};

pub const TITLE: &str = "Not Found";
pub const DESCRIPTION: &str = "This page does not exist.";
pub const SLASH: &str = "/";

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);

    rsx! {
        Head {
            title: TITLE,
            description: DESCRIPTION,
            url: urls::route(route.join(SLASH)),
        }

        h1 {
            { TITLE }
        }

        p {
            { DESCRIPTION }
        }
    }
}
