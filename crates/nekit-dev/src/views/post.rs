use dioxus::prelude::*;

use nekit_blog::{post::Post as BlogPost, slug::Slug};

use crate::{blog::read_post, components::shared::COLUMN, views::not_found::NotFound};

pub const BLOG: &str = "blog";

pub const FAILED: &str = "Failed to fetch the post... ;~;";

pub const FETCHING: &str = "Fetching...";

pub const UPDATED: &str = "updated";

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
                main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24",
                    p { class: "text-light-muted dark:text-muted", {FAILED} }
                }
            },
            None => rsx! {
                main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24",
                    p { class: "text-light-muted dark:text-muted", {FETCHING} }
                }
            },
        }
    }
}

#[component]
pub fn PostHead(title: String, description: String) -> Element {
    // override the title and description of the page
    rsx! {
        document::Title { {title.clone()} }

        document::Meta { name: "description", content: description.clone() }

        document::Meta { name: "og:title", content: title }
        document::Meta { name: "og:description", content: description }
    }
}

#[component]
pub fn Contents(post: BlogPost) -> Element {
    let summary = post.summary();

    let title = summary.title();

    let description = summary.description();

    let date = summary.date();

    let updated = summary.updated();

    let tags = summary.tags();

    let content = post.content();

    rsx! {
        PostHead { title: "{title}", description: "{description}" }

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
