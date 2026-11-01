use dioxus::prelude::*;

use crate::urls;

pub const THEME: Asset = asset!(
    "/assets/theme.js",
    JsAssetOptions::new().with_minify(true).with_preload(true)
);

pub const STYLE: Asset = asset!(
    "/assets/tailwind.css",
    CssAssetOptions::new().with_minify(true).with_preload(true)
);

pub const ICON_PNG: &str = "images/icon.png";
pub const ICON_SVG: &str = "images/icon.svg";

#[component]
pub fn Head() -> Element {
    let icon_png = urls::static_route(ICON_PNG);
    let icon_svg = urls::static_route(ICON_SVG);

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }

        document::Meta { charset: "utf-8" }

        document::Script { src: THEME }

        document::Stylesheet { href: STYLE }

        document::Link { rel: "apple-touch-icon", href: "{icon_png}" }

        document::Link { rel: "icon", href: "{icon_png}" }
        document::Link { rel: "icon", href: "{icon_svg}" }
    }
}
