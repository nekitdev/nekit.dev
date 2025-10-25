use dioxus::prelude::*;
use non_empty_str::{NonEmptyStr, const_non_empty_str};

use crate::{
    age::nekit,
    components::head::{Head, NAME},
    links::root,
    routes::Route,
};

pub const TITLE: &NonEmptyStr = const_non_empty_str!("Home");
pub const DESCRIPTION: &NonEmptyStr = const_non_empty_str!("Building awesome software.");

#[component]
pub fn Home() -> Element {
    let age = nekit();

    rsx! {
        Head {
            title: TITLE.to_non_empty_string(),
            description: DESCRIPTION.to_non_empty_string(),
            url: root(),
        }

        nav {
            class: "absolute flex w-full",
            div {
                class: "
                    mx-auto
                    max-w-md sm:max-w-3xl lg:max-w-7xl
                    px-4 sm:px-6 lg:px-8
                    flex items-center
                    w-full
                    py-4
                ",
                Link {
                    to: Route::Home {},
                    class: "mr-auto text-2xl lg:text-3xl",
                    { NAME.as_str() }
                }
                div {
                    class: "hidden lg:flex lg:space-x-8 text-xl",
                    Link {
                        to: Route::Blog {},
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Blog"
                    },
                    Link {
                        to: Route::Projects {},
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Projects"
                    }
                }
                div {
                    class: "relative ml-auto flex space-x-8",
                    div {
                        class: "hidden md:flex space-x-8",
                        a {
                            href: "/youtube",
                            aria_label: "YouTube",
                            i {
                                class: "fa-brands fa-youtube text-youtube text-4xl"
                            }
                        }
                        a {
                            href: "/reddit",
                            aria_label: "Reddit",
                            i {
                                class: "fa-brands fa-reddit-alien text-reddit text-4xl"
                            }
                        }
                        a {
                            href: "/telegram",
                            aria_label: "Telegram",
                            i {
                                class: "fa-brands fa-telegram text-telegram text-4xl"
                            }
                        }
                        a {
                            href: "/chat",
                            aria_label: "Discord",
                            i {
                                class: "fa-brands fa-discord text-discord text-4xl"
                            }
                        }
                        a {
                            href: "/bluesky",
                            aria_label: "Bluesky",
                            i {
                                class: "fa-brands fa-bluesky text-bluesky text-4xl"
                            }
                        }
                    }
                    a {
                        href: "/github",
                        aria_label: "GitHub",
                        i {
                            class: "fa-brands fa-github text-4xl"
                        }
                    }
                    a {
                        class: "
                            text-xl
                            focus:outline-none
                            border-neutral-900 dark:border-white
                            hover:border-melody-blue hover:text-melody-blue
                            dark:hover:border-melody-purple dark:hover:text-melody-purple
                            border-2
                            h-10 px-4
                            rounded-lg
                            w-full sm:w-auto
                            flex items-center justify-center
                        ",
                        href: "/funding",
                        "Funding"
                    }
                }
            }
        }

        section {
            class: "
                mx-auto
                max-w-md sm:max-w-3xl lg:max-w-7xl
                px-4 sm:px-6 lg:px-8
                flex flex-col lg:flex-row
                justify-between
                gap-5
                pt-16 sm:pt-20 lg:pt-24
            ",
            div {
                class: "my-12 lg:my-24 w-full lg:w-1/2",
                h1 {
                    class: "text-5xl lg:text-7xl",
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Building",
                    }
                    " "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "awesome",
                    }
                    " "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "software"
                    }
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "."
                    }
                }
                div {
                    class: "
                        mt-6
                        text-xl lg:text-2xl
                        text-neutral-700 dark:text-neutral-500
                    ",
                    b {
                        "Nikita Tikhonov"
                    }
                    ", "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        { age.to_string() }
                    } "-year-old software developer from Moscow, Russia."
                }
            }
        }
    }
}
