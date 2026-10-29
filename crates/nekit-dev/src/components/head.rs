use dioxus::prelude::*;

use crate::{routes::Info, urls};

pub const THEME: Asset = asset!(
    "/assets/theme.js",
    JsAssetOptions::new().with_minify(true).with_preload(true)
);

pub const STYLE: Asset = asset!(
    "/assets/tailwind.css",
    CssAssetOptions::new().with_minify(true).with_preload(true)
);

pub const ICON_PNG: &str = "/images/icon.png";
pub const ICON_SVG: &str = "/images/icon.svg";

pub const NAME: &str = "nekitdev";
pub const LANG: &str = "en_US";
pub const TYPE: &str = "website";

#[component]
pub fn Head(info: Info) -> Element {
    let Info {
        title,
        description,
        url,
    } = info;

    let icon_png = urls::static_route(ICON_PNG);
    let icon_svg = urls::static_route(ICON_SVG);

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        document::Meta { charset: "utf-8" }

        document::Meta { property: "og:title", content: title.clone() }
        document::Meta { property: "og:type", content: TYPE }
        document::Meta { property: "og:url", content: url.clone() }
        document::Meta { property: "og:image", content: icon_png.clone() }
        document::Meta { property: "og:description", content: description.clone() }
        document::Meta { property: "og:site_name", content: NAME }
        document::Meta { property: "og:locale", content: LANG }

        document::Meta { name: "description", content: description }

        document::Title { {title} }

        document::Script { src: THEME }

        document::Stylesheet { href: STYLE }

        document::Link { rel: "apple-touch-icon", href: icon_png.clone() }

        document::Link { rel: "icon", href: icon_png }
        document::Link { rel: "icon", href: icon_svg }
    }
}
