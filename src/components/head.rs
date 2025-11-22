use dioxus::prelude::*;

use crate::urls;

pub const STYLE: Asset = asset!(
    "/assets/tailwind.css",
    CssAssetOptions::new().with_preload(true)
);

pub const ICON_PNG: Asset = asset!("/assets/images/icon.png");
pub const ICON_SVG: Asset = asset!("/assets/images/icon.svg");

pub const ICONS: &str = "https://kit.fontawesome.com/c7493dda5d.js";

pub const NAME: &str = "nekitdev";
pub const LANG: &str = "en_US";
pub const TYPE: &str = "website";

#[component]
pub fn Head(title: String, description: String, url: String) -> Element {
    let icon = urls::asset(ICON_PNG);

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        document::Meta { charset: "utf-8" }

        document::Meta { property: "og:title", content: title.clone() }
        document::Meta { property: "og:type", content: TYPE }
        document::Meta { property: "og:url", content: url.clone() }
        document::Meta { property: "og:image", content: icon }
        document::Meta { property: "og:description", content: description.clone() }
        document::Meta { property: "og:site_name", content: NAME }
        document::Meta { property: "og:locale", content: LANG }

        document::Meta { name: "description", content: description }

        document::Title {
            { title }
        }

        document::Stylesheet { href: STYLE }

        document::Link { rel: "icon", href: ICON_PNG }
        document::Link { rel: "icon", href: ICON_SVG }
        document::Link { rel: "apple-touch-icon", href: ICON_PNG }

        document::Script {
            async: "true",
            crossorigin: "anonymous",
            src: ICONS,
        }
    }
}
