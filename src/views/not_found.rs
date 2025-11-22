use dioxus::{fullstack::*, prelude::*};

use crate::{components::head::Head, urls};

pub const NOT_FOUND: Asset = asset!("/assets/images/not-found.png");

pub const TITLE: &str = "Not Found";
pub const DESCRIPTION: &str = "This route was not found.";
pub const SLASH: &str = "/";

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route = route.join(SLASH);

    FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);

    rsx! {
        Head {
            title: TITLE,
            description: DESCRIPTION,
            url: urls::route(&route),
        }

        div {
            class: "
                mx-auto
                max-w-md sm:max-w-3xl lg:max-w-5xl
                px-4 sm:px-6 lg:px-8
                flex flex-col lg:flex-row
                justify-between
                gap-5
                py-16 sm:pt-20 lg:pt-24
            ",
            section {
                class: "w-full lg:w-1/2",
                h1 {
                    class: "text-5xl lg:text-7xl",
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Not"
                    }
                    " "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Found"
                    }
                }
                p {
                    class: "text-xl",
                    "Route "
                    span {
                        class: "wrap-break-word text-transparent bg-clip-text bg-gradient-to-r from-melody-purple to-melody-blue",
                        "/{route}",
                    }
                    " was not found."
                }
            }

            img {
                class: "w-full lg:w-1/2 object-contain",
                loading: "lazy",
                width: "448",
                height: "448",
                src: NOT_FOUND,
                alt: "Not Found",
            }
        }
    }
}
