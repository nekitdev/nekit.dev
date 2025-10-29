use dioxus::prelude::*;

use crate::{
    chrono,
    components::{
        head::{Head, NAME},
        skill::Skill,
    },
    routes::Route,
    urls,
};

pub const TITLE: &str = "Home";
pub const DESCRIPTION: &str = "Building awesome software.";

pub const NEKO: Asset = asset!("/assets/images/neko.png");

#[component]
pub fn Home() -> Element {
    let age = chrono::age();
    let year = chrono::year();

    rsx! {
        Head {
            title: TITLE,
            description: DESCRIPTION,
            url: urls::root(),
        }

        nav {
            aria_label: "Nagivagion",
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
                    { NAME }
                }
                div {
                    class: "hidden lg:flex lg:space-x-8 text-xl",
                    Link {
                        to: Route::Blog {},
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Blog"
                    },
                    Link {
                        to: Route::Security {},
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Security"
                    },
                    a {
                        href: "/funding",
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Funding"
                    }
                }
                div {
                    class: "relative ml-auto flex space-x-8",
                    div {
                        class: "hidden md:flex space-x-8",
                        a {
                            href: "/youtube",
                            aria_label: "YouTube",
                            span {
                                class: "fa-brands fa-youtube text-youtube text-4xl"
                            }
                        }
                        a {
                            href: "/reddit",
                            aria_label: "Reddit",
                            span {
                                class: "fa-brands fa-reddit-alien text-reddit text-4xl"
                            }
                        }
                        a {
                            href: "/telegram",
                            aria_label: "Telegram",
                            span {
                                class: "fa-brands fa-telegram text-telegram text-4xl"
                            }
                        }
                        a {
                            href: "/chat",
                            aria_label: "Discord",
                            span {
                                class: "fa-brands fa-discord text-discord text-4xl"
                            }
                        }
                        a {
                            href: "/bluesky",
                            aria_label: "Bluesky",
                            span {
                                class: "fa-brands fa-bluesky text-bluesky text-4xl"
                            }
                        }
                    }
                    a {
                        href: "/github",
                        aria_label: "GitHub",
                        span {
                            class: "fa-brands fa-github text-4xl"
                        }
                    }
                    Link {
                        to: Route::Projects {},
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
                        "Projects"
                    }
                }
            }
        }

        div {
            class: "
                mx-auto
                max-w-md sm:max-w-3xl lg:max-w-7xl
                px-4 sm:px-6 lg:px-8
                flex flex-col lg:flex-row
                justify-between
                gap-5
                pt-16 sm:pt-20 lg:pt-24
            ",
            section {
                class: "my-12 w-full lg:w-1/2",
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

                ul {
                    class: "mt-6 flex flex-row flex-wrap gap-5",
                    Skill {
                        display: "Rust",
                        color: "bg-rust/20",
                        dot: "bg-rust",
                    },
                    Skill {
                        display: "Python",
                        color: "bg-python/20",
                        dot: "bg-python",
                    },
                    Skill {
                        display: "Git",
                        color: "bg-git/20",
                        dot: "bg-git",
                    }
                    Skill {
                        display: "Web (HTML/CSS/TS)",
                        color: "bg-typescript/20",
                        dot: "bg-typescript",
                    },
                    Skill {
                        display: "SQL",
                        color: "bg-postgresql/20",
                        dot: "bg-postgresql",
                    },
                }
            }
            div {
                class: "mb-10 hidden lg:flex w-full lg:w-1/2",
                img {
                    class: "object-contain",
                    loading: "lazy",
                    src: NEKO,
                    alt: "Neko",
                }
            }
        }

        footer {
            class: "mx-auto max-w-md sm:max-w-3xl lg:max-w-7xl px-4 sm:px-6 lg:px-8 py-16",
            div {
                class: "grid grid-cols-2 gap-y-4 lg:flex lg:flex-row lg:justify-between mb-8 text-lg",
                div {
                    class: "ml-4 flex flex-col lg:ml-0",
                    h2 {
                        class: "mb-2 text-neutral-600 dark:text-neutral-400",
                        "Navigation"
                    }
                    ul {
                        class: "grid gap-2",
                        li {
                            Link {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                to: Route::Home {},
                                "Home",
                            }
                        }
                        li {
                            Link {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                to: Route::Blog {},
                                "Blog",
                            }
                        }
                        li {
                            Link {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                to: Route::Projects {},
                                "Projects"
                            }
                        }
                    }
                }
                div {
                    class: "ml-4 flex flex-col lg:ml-0",
                    h2 {
                        class: "mb-2 text-neutral-600 dark:text-neutral-400",
                        "Social"
                    }
                    ul {
                        class: "grid gap-2",
                        li {
                            a {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                href: "/github",
                                "GitHub"
                            }
                        }
                        li {
                            a {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                href: "/bluesky",
                                "Bluesky"
                            }
                        }
                        li {
                            a {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                href: "/reddit",
                                "Reddit"
                            }
                        }
                        li {
                            a {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                href: "/youtube",
                                "YouTube",
                            }
                        }
                    }
                }
                div {
                    class: "ml-4 flex flex-col lg:ml-0",
                    h2 {
                        class: "mb-2 text-neutral-600 dark:text-neutral-400",
                        "Contact"
                    }
                    ul {
                        class: "grid gap-2",
                        li {
                            a {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                href: "/discord",
                                "Discord"
                            }
                        }
                        li {
                            a {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                href: "/telegram",
                                "Telegram"
                            }
                        }
                        li {
                            a {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                href: "/email/nekit",
                                "Email"
                            }
                        }
                    }
                }
                div {
                    class: "ml-4 flex flex-col lg:ml-0",
                    h2 {
                        class: "mb-2 text-neutral-600 dark:text-neutral-400",
                        "Security"
                    }
                    ul {
                        class: "grid gap-2",
                        li {
                            Link {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                to: Route::Security {},
                                "Policy"
                            }
                        }
                        li {
                            a {
                                class: "hover:text-melody-blue dark:hover:text-melody-purple",
                                href: "/email/security",
                                "Report"
                            }
                        }
                    }
                }
                // div class="ml-4 flex flex-col lg:ml-0" {
                //     h4 class="mb-2 text-neutral-600 dark:text-neutral-400" { "Resources" }
                //     ul class="grid gap-2" {
                //         li {
                //             a href="/download" class="hover:text-melody-blue dark:hover:text-melody-purple" { "Download" }
                //         }
                //         li {
                //             a href="/support" class="hover:text-melody-blue dark:hover:text-melody-purple" { "Support" }
                //         }
                //         li {
                //             a href="/premium" class="hover:text-melody-blue dark:hover:text-melody-purple" { "Premium" }
                //         }
                //         li {
                //             a href="/dev" class="hover:text-melody-blue dark:hover:text-melody-purple" { "Developers" }
                //         }
                //     }
                // }
            }
            p {
                class: "min-w-full text-neutral-600 dark:text-neutral-400 text-center text-xl mt-8",
                span {
                    class: "fa-regular fa-copyright hover:text-melody-blue dark:hover:text-melody-purple mr-2"
                }
                "{year} {NAME}"
                span {
                    class: "fa-solid fa-heart hover:text-melody-blue dark:hover:text-melody-purple ml-2"
                }
            }
        }
    }
}
