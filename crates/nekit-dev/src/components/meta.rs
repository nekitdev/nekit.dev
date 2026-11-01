use dioxus::prelude::*;

use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};

use crate::{
    components::head::ICON_PNG,
    routes::{Info, Route},
    urls,
};

pub const DEFAULT_KIND: &NonEmptyStr = const_non_empty_str!("website");

pub const NAME: &NonEmptyStr = const_non_empty_str!("nekitdev");

pub const LANGUAGE: &NonEmptyStr = const_non_empty_str!("en_US");

#[component]
pub fn RouteMeta(#[props(default = DEFAULT_KIND.to_owned())] kind: NonEmptyString) -> Element {
    let route: Route = use_route();

    let info = route.info();

    rsx! {
        Meta { info, kind }
    }
}

#[component]
pub fn Meta(
    info: Info,
    #[props(default = DEFAULT_KIND.to_owned())] kind: NonEmptyString,
) -> Element {
    let Info {
        title,
        description,
        url,
    } = info;

    let icon_png = urls::static_route(ICON_PNG);

    rsx! {
        document::Meta { property: "og:title", content: "{title}" }
        document::Meta { property: "og:type", content: "{kind}" }
        document::Meta { property: "og:url", content: "{url}" }
        document::Meta { property: "og:image", content: "{icon_png}" }
        document::Meta { property: "og:description", content: "{description}" }
        document::Meta { property: "og:site_name", content: "{NAME}" }
        document::Meta { property: "og:locale", content: "{LANGUAGE}" }

        document::Meta { name: "description", content: "{description}" }

        document::Link { rel: "canonical", href: "{url}" }

        document::Title { "{title}" }
    }
}
