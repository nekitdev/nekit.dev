use dioxus::prelude::*;
use nekit_blog::slug::Slug;

use crate::{
    components::shell::Shell,
    urls,
    views::{
        blog::Blog, home::Home, host::Host, not_found::NotFound, post::Post, security::Security,
    },
};

pub type StaticStr = &'static str;

pub const SLASH: StaticStr = "/";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Info {
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tab {
    Home,
    Blog,
    Host,
    Security,
}

impl Tab {
    pub const COUNT: usize = 4;

    pub const ALL: [Self; Self::COUNT] = [Self::Home, Self::Blog, Self::Host, Self::Security];

    pub const fn name(self) -> StaticStr {
        match self {
            Self::Home => "nekitdev",
            Self::Blog => "blog",
            Self::Host => "host",
            Self::Security => "security",
        }
    }

    pub const fn route(self) -> Route {
        match self {
            Self::Home => Route::Home {},
            Self::Blog => Route::Blog {},
            Self::Host => Route::Host {},
            Self::Security => Route::Security {},
        }
    }
}

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Shell)]
        #[route("/")]
        Home {},
        #[route("/blog")]
        Blog {},
        #[route("/host")]
        Host {},
        #[route("/security")]
        Security {},
        #[route("/blog/:slug")]
        Post {
            slug: Slug,
        },
        #[route("/:..segments")]
        NotFound {
            segments: Vec<String>,
        }
}

impl Route {
    pub const fn tab(&self) -> Option<Tab> {
        match self {
            Self::Home {} => Some(Tab::Home),
            Self::Blog {} => Some(Tab::Blog),
            Self::Host {} => Some(Tab::Host),
            Self::Security {} => Some(Tab::Security),
            Self::Post { .. } => Some(Tab::Blog),
            Self::NotFound { .. } => None,
        }
    }

    pub const fn title_str(&self) -> StaticStr {
        match self {
            Self::Home {} => "Home",
            Self::Blog {} => "Blog",
            Self::Host {} => "Host",
            Self::Security {} => "Security Policy",
            Self::Post { .. } => "Blog Post",
            Self::NotFound { .. } => "Not Found",
        }
    }

    pub const fn description_str(&self) -> StaticStr {
        match self {
            Self::Home {} => "Building awesome software.",
            Self::Blog {} => "Personal blog.",
            Self::Host {} => "The machine behind the site.",
            Self::Security {} => "Reporting security vulnerabilities.",
            Self::Post { .. } => "Read more.",
            Self::NotFound { .. } => "Nothing here.",
        }
    }

    pub fn title(&self) -> String {
        self.title_str().to_owned()
    }

    pub fn description(&self) -> String {
        self.description_str().to_owned()
    }

    pub fn url(&self) -> String {
        match self {
            Self::Home {} => urls::root(),
            Self::Blog {} => urls::route("blog"),
            Self::Host {} => urls::route("host"),
            Self::Security {} => urls::route("security"),
            Self::Post { slug } => {
                let route = format!("blog/{slug}");

                urls::route(route)
            }
            Self::NotFound { segments } => {
                let route = segments.join(SLASH);

                urls::route(route)
            }
        }
    }

    pub fn info(&self) -> Info {
        let title = self.title();
        let description = self.description();
        let url = self.url();

        Info {
            title,
            description,
            url,
        }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
