use dioxus::prelude::*;

use nekit_blog::{post::Post as BlogPost, slug::Slug};
use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};

use crate::{
    blog::read_post,
    components::{
        meta::{Meta, RouteMeta},
        shared::COLUMN,
    },
    routes::{Info, Route},
    views::not_found::NotFound,
};

pub const BLOG: &str = "blog";

pub const UPDATED: &str = "updated";

pub const FAILED: &NonEmptyStr = const_non_empty_str!("Failed to fetch the post... ;~;");
pub const FETCHING: &NonEmptyStr = const_non_empty_str!("Fetching...");

pub const KIND: &NonEmptyStr = const_non_empty_str!("article");

pub fn segments(slug: Slug) -> Vec<String> {
    vec![BLOG.to_owned(), slug.get().into_string()]
}

#[component]
pub fn Post(slug: Slug) -> Element {
    let resource = use_server_future(use_reactive!(|slug| read_post(slug)))?;

    rsx! {
        match resource.read().cloned() {
            Some(Ok(Some(post))) => rsx! {
                Contents { post }
            },
            Some(Ok(None)) => rsx! {
                NotFound { segments: segments(slug.clone()) }
            },
            Some(Err(_)) => rsx! {
                Message { string: FAILED.to_owned() }
            },
            None => rsx! {
                Message { string: FETCHING.to_owned() }
            },
        }
    }
}

#[component]
pub fn Message(string: NonEmptyString) -> Element {
    let kind = KIND.to_owned();

    rsx! {
        RouteMeta { kind }

        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24",
            p { class: "text-light-muted dark:text-muted", "{string}" }
        }
    }
}

#[component]
pub fn Contents(post: BlogPost) -> Element {
    let route: Route = use_route();

    // summary information

    let summary = post.summary();

    let title = summary.title();

    let description = summary.description();

    let date = summary.date();

    let updated = summary.updated();

    let tags = summary.tags();

    let content = post.content();

    // for metadata

    let info = Info {
        title: title.to_owned(),
        description: description.to_owned(),
        url: route.url(),
    };

    let kind = KIND.to_owned();

    rsx! {
        Meta { info, kind }

        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-12",
            header { class: "flex flex-col gap-2",
                div { class: "flex flex-wrap gap-x-[2ch] text-light-muted dark:text-muted",
                    time { datetime: "{date}", "{date}" }

                    if let Some(update) = updated {
                        span {
                            "({UPDATED} "
                            time { datetime: "{update}", "{update}" }
                            ")"
                        }
                    }

                    for tag in tags {
                        span { class: "text-light-other dark:text-other", "{tag}" }
                    }
                }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight", "{title}" }
            }
            article { class: "post", dangerous_inner_html: content }
        }
    }
}
