use dioxus::prelude::*;
use non_empty_str::{NonEmptyStr, const_non_empty_str};

use crate::{components::head::Head, urls};

pub const TITLE: &NonEmptyStr = const_non_empty_str!("Not Found");
pub const DESCRIPTION: &NonEmptyStr = const_non_empty_str!("This page does not exist.");
pub const SLASH: &NonEmptyStr = const_non_empty_str!("/");

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        Head {
            title: TITLE.to_non_empty_string(),
            description: DESCRIPTION.to_non_empty_string(),
            url: urls::route(route.join(SLASH)),
        }

        h1 {
            "{TITLE}"
        }

        p {
            "{DESCRIPTION}"
        }
    }
}
