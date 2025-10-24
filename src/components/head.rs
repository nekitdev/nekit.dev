use dioxus::prelude::*;
use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};

pub const STYLE: Asset = asset!(
    "/assets/tailwind.css",
    CssAssetOptions::new().with_preload(true)
);
pub const IMAGE_PNG: Asset = asset!("/assets/images/icon.png");
pub const IMAGE_SVG: Asset = asset!("/assets/images/icon.svg");
pub const MANIFEST: Asset = asset!("/assets/manifest.json");

pub const IMAGE: &NonEmptyStr = const_non_empty_str!("https://nekit.dev/assets/images/icon.png");

pub const NAME: &NonEmptyStr = const_non_empty_str!("nekitdev");
pub const LANG: &NonEmptyStr = const_non_empty_str!("en_US");
pub const TYPE: &NonEmptyStr = const_non_empty_str!("website");

#[component]
pub fn Head(title: NonEmptyString, description: NonEmptyString, url: NonEmptyString) -> Element {
    rsx! {
        document::Meta { charset: "utf-8" }

        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        document::Meta { property: "og:title", content: title.as_str() }
        document::Meta { property: "og:type", content: TYPE.as_str() }
        document::Meta { property: "og:url", content: url.as_str() }
        document::Meta { property: "og:image", content: IMAGE.as_str() }
        document::Meta { property: "og:description", content: description.as_str() }
        document::Meta { property: "og:site_name", content: NAME.as_str() }
        document::Meta { property: "og:locale", content: LANG.as_str() }

        document::Meta { name: "description", content: description.as_str() }

        document::Title {
            { title.as_str() }
        }

        document::Stylesheet { href: STYLE }

        document::Link { rel: "icon", href: IMAGE_PNG }
        document::Link { rel: "icon", href: IMAGE_SVG }
        document::Link { rel: "apple-touch-icon", href: IMAGE_PNG }

        document::Link { rel: "manifest", href: MANIFEST }
    }
}
