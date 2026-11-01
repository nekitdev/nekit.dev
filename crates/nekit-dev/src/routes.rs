use dioxus::prelude::*;

use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};

use nekit_blog::slug::Slug;

use crate::{
    components::shell::Shell,
    statics::StaticNonEmptyStr,
    urls,
    views::{
        blog::Blog, home::Home, host::Host, not_found::NotFound, post::Post, security::Security,
    },
};

pub const SLASH: &str = "/";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Info {
    pub title: NonEmptyString,
    pub description: NonEmptyString,
    pub url: NonEmptyString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tab {
    Home,
    Blog,
    Host,
    Security,
}

pub const NEKITDEV: &NonEmptyStr = const_non_empty_str!("nekitdev");
pub const BLOG: &NonEmptyStr = const_non_empty_str!("blog");
pub const HOST: &NonEmptyStr = const_non_empty_str!("host");
pub const SECURITY: &NonEmptyStr = const_non_empty_str!("security");

impl Tab {
    pub const COUNT: usize = 4;

    pub const ALL: [Self; Self::COUNT] = [Self::Home, Self::Blog, Self::Host, Self::Security];

    pub const fn name(self) -> StaticNonEmptyStr {
        match self {
            Self::Home => NEKITDEV,
            Self::Blog => BLOG,
            Self::Host => HOST,
            Self::Security => SECURITY,
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
        #[route("/blog/:slug")]
        Post {
            slug: Slug,
        },
        #[route("/host")]
        Host {},
        #[route("/security")]
        Security {},
        #[route("/:..segments")]
        NotFound {
            segments: Vec<String>,
        }
}

pub const HOME_TITLE: &NonEmptyStr = const_non_empty_str!("Home");
pub const BLOG_TITLE: &NonEmptyStr = const_non_empty_str!("Blog");
pub const POST_TITLE: &NonEmptyStr = const_non_empty_str!("Post");
pub const HOST_TITLE: &NonEmptyStr = const_non_empty_str!("Host");
pub const SECURITY_TITLE: &NonEmptyStr = const_non_empty_str!("Security Policy");
pub const NOT_FOUND_TITLE: &NonEmptyStr = const_non_empty_str!("Not Found");

pub const HOME_DESCRIPTION: &NonEmptyStr = const_non_empty_str!("Building awesome software.");
pub const BLOG_DESCRIPTION: &NonEmptyStr = const_non_empty_str!("Personal blog.");
pub const POST_DESCRIPTION: &NonEmptyStr = const_non_empty_str!("Read the post.");
pub const HOST_DESCRIPTION: &NonEmptyStr = const_non_empty_str!("The machine behind the site.");
pub const SECURITY_DESCRIPTION: &NonEmptyStr =
    const_non_empty_str!("Reporting security vulnerabilities.");
pub const NOT_FOUND_DESCRIPTION: &NonEmptyStr = const_non_empty_str!("Nothing here.");

pub const BLOG_ROUTE: &NonEmptyStr = const_non_empty_str!("blog");
pub const HOST_ROUTE: &NonEmptyStr = const_non_empty_str!("host");
pub const SECURITY_ROUTE: &NonEmptyStr = const_non_empty_str!("security");

pub fn post_route<S: AsRef<NonEmptyStr>>(slug: S) -> NonEmptyString {
    let mut route = BLOG_ROUTE.to_owned();

    route.push_str(SLASH);

    route.push_str(slug.as_ref());

    route
}

impl Route {
    pub const fn tab(&self) -> Option<Tab> {
        match self {
            Self::Home {} => Some(Tab::Home),
            Self::Blog {} => Some(Tab::Blog),
            Self::Post { .. } => Some(Tab::Blog),
            Self::Host {} => Some(Tab::Host),
            Self::Security {} => Some(Tab::Security),
            Self::NotFound { .. } => None,
        }
    }

    pub const fn title(&self) -> StaticNonEmptyStr {
        match self {
            Self::Home {} => HOME_TITLE,
            Self::Blog {} => BLOG_TITLE,
            Self::Post { .. } => BLOG_TITLE,
            Self::Host {} => HOST_TITLE,
            Self::Security {} => SECURITY_TITLE,
            Self::NotFound { .. } => NOT_FOUND_TITLE,
        }
    }

    pub const fn description(&self) -> StaticNonEmptyStr {
        match self {
            Self::Home {} => HOME_DESCRIPTION,
            Self::Blog {} => BLOG_DESCRIPTION,
            Self::Post { .. } => POST_DESCRIPTION,
            Self::Host {} => HOST_DESCRIPTION,
            Self::Security {} => SECURITY_DESCRIPTION,
            Self::NotFound { .. } => NOT_FOUND_DESCRIPTION,
        }
    }

    pub fn url(&self) -> NonEmptyString {
        match self {
            Self::Home {} => urls::root(),
            Self::Blog {} => urls::route(BLOG_ROUTE),
            Self::Post { slug } => urls::route(post_route(slug)),
            Self::Host {} => urls::route(HOST_ROUTE),
            Self::Security {} => urls::route(SECURITY_ROUTE),
            Self::NotFound { segments } => urls::route(segments.join(SLASH)),
        }
    }

    pub fn info(&self) -> Info {
        let title = self.title().to_owned();
        let description = self.description().to_owned();
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
