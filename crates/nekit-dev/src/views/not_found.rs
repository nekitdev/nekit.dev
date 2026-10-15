use dioxus::{fullstack::*, prelude::*};

use crate::{
    components::shared::COLUMN,
    routes::{Route, SLASH},
    urls,
};

pub const NOT_FOUND: &str = "/images/not-found.png";

pub const MESSAGE: &str = "Not Found";

pub const NOTHING: &str = "Nothing here.";

pub const CODE: u16 = 404;

pub fn use_not_found_status() {
    use_hook(|| {
        if let Some(mut context) = FullstackContext::current() {
            context.set_current_http_status(HttpError::new(StatusCode::NOT_FOUND, MESSAGE));
        }
    })
}

pub fn diagnostic_code(code: u16) -> String {
    format!("error[E{code:04}]")
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    use_not_found_status();

    let route = segments.join(SLASH);

    let not_found = urls::static_route(NOT_FOUND);

    rsx! {
        main { class: "{COLUMN} grow px-[4ch] py-24 flex flex-col gap-12",
            div { class: "flex flex-col gap-2",
                div { class: "text-light-muted dark:text-muted", "{CODE}" }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight", {NOTHING} }
            }
            div { class: "flex flex-wrap items-center gap-x-[5ch] gap-y-8",
                div { class: "flex-1 basis-[22ch] min-w-0",
                    pre {
                        class: "whitespace-pre-wrap wrap-anywhere",
                        aria_label: "Diagnostic",
                        span { class: "block",
                            span { class: "text-light-error dark:text-error font-bold",
                                {diagnostic_code(CODE)}
                            }
                            span { class: "font-bold", ": resource not found" }
                        }
                        span { class: "block pl-[6ch] -indent-[6ch]",
                            span { class: "text-light-muted dark:text-muted", "  --> " }
                            "{SLASH}{route}"
                        }
                        span { class: "block text-light-muted dark:text-muted", "   |" }
                        span { class: "block pl-[5ch] -indent-[5ch]",
                            span { class: "text-light-muted dark:text-muted", "   = " }
                            span { class: "font-bold", "note" }
                            ": it may have been moved, or it never existed at all"
                        }
                        span { class: "block pl-[5ch] -indent-[5ch]",
                            span { class: "text-light-muted dark:text-muted", "   = " }
                            span { class: "font-bold", "help" }
                            ": try "
                            Link {
                                class: "text-light-accent dark:text-accent hover:underline",
                                to: Route::Home {},
                                "home"
                            }
                            " or "
                            Link {
                                class: "text-light-accent dark:text-accent hover:underline",
                                to: Route::Blog {},
                                "blog"
                            }
                            " instead"
                        }
                    }
                }
                img {
                    class: "w-[22ch] shrink-0 aspect-square object-contain",
                    src: not_found,
                    alt: "Confused Neko",
                }
            }
        }
    }
}
