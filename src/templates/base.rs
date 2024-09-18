use std::borrow::Cow;

use maud::{html, Markup, DOCTYPE};
use serde::{Deserialize, Serialize};

pub const ICONS: &str = "https://use.fontawesome.com/releases/v6.6.0/js/all.js";

pub const IMAGE: &str = "https://nekit.dev/static/images/icon.png";

pub const IMAGE_PNG: &str = "/static/images/icon.png";
pub const IMAGE_SVG: &str = "/static/images/icon.svg";
pub const MANIFEST: &str = "/static/manifest.json";

pub const STYLE: &str = "/static/css/output.css";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeadContext<'c> {
    pub title: Cow<'c, str>,
    pub description: Cow<'c, str>,
}

impl<'c> HeadContext<'c> {
    pub fn new(title: Cow<'c, str>, description: Cow<'c, str>) -> Self {
        Self { title, description }
    }
}

pub fn head(context: &HeadContext<'_>) -> Markup {
    html! {
        head {
            meta charset="utf-8";

            meta name="viewport" content="width=device-width, initial-scale=1";

            meta name="description" content=(context.description);

            meta property="og:site_name" content="nekitdev";
            meta property="og:title" content=(context.title);
            meta property="og:description" content=(context.description);
            meta property="og:image" content=(IMAGE);

            link rel="icon" type="image/svg+xml" href=(IMAGE_SVG);
            link rel="icon" type="image/png" href=(IMAGE_PNG);

            link rel="apple-touch-icon" href=(IMAGE_PNG);

            link rel="manifest" href=(MANIFEST);

            script defer src=(ICONS) {}

            link rel="preload" href=(STYLE) as="style";
            link rel="stylesheet" href=(STYLE);


            title { (context.title) }
        }
    }
}

pub fn wrap(body: Markup) -> Markup {
    html! {
        body class="
            antialiased
            transition
            ease-in-out
            bg-neutral-50
            dark:bg-neutral-900
            text-neutral-900
            dark:text-neutral-50
            font-mono
        " {
            (body)
        }
    }
}

pub fn base(head: Markup, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head)
            (wrap(body))
        }
    }
}
