use dioxus::prelude::*;
use nekit_blog::{blog::Summaries, post::Summary};

use crate::{components::heading::Heading, routes::Route};

pub const POSTS: &str = "posts";

pub const UPDATED: &str = "updated";

#[component]
pub fn Posts(summaries: Summaries) -> Element {
    rsx! {
        section { aria_labelledby: POSTS,
            Heading { id: POSTS, title: POSTS }

            ol { class: "flex flex-col gap-6",
                for summary in summaries {
                    Entry { key: "{summary.slug()}", summary }
                }
            }
        }
    }
}

fn additional(updated: bool, empty: bool) -> bool {
    updated || !empty
}

#[component]
pub fn Entry(summary: Summary) -> Element {
    let slug = summary.slug().clone();

    let date = summary.date();

    let title = summary.title();

    let description = summary.description();

    let updated = summary.updated();

    let tags = summary.tags();

    rsx! {
        li { class: "grid grid-cols-1 sm:grid-cols-[max-content_1fr] gap-x-[clamp(2ch,4vw,8ch)] gap-y-1",
            time {
                class: "text-light-muted dark:text-muted",
                datetime: "{date}",
                "{date}"
            }
            article { class: "flex flex-col gap-1 min-w-0",
                h3 {
                    Link { class: "font-bold text-light-accent dark:text-accent hover:underline",
                        to: Route::Post { slug },
                        "{title}"
                    }
                }
                p { class: "wrap-break-word", "{description}" }
                if additional(updated.is_some(), tags.is_empty()) {
                    div { class: "flex flex-wrap gap-x-[2ch]",
                        if let Some(update) = updated {
                            span {
                                class: "text-light-muted dark:text-muted",
                                "{UPDATED}"
                                " "
                                time {
                                    datetime: "{update}",
                                    "{update}"
                                }
                            }
                        }
                        for tag in tags {
                            span { class: "text-light-other dark:text-other", "{tag}" }
                        }
                    }
                }
            }
        }
    }
}
