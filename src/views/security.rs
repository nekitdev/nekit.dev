use dioxus::prelude::*;

use crate::{
    components::head::{Head, NAME},
    routes::Route,
    urls,
};

pub const TITLE: &str = "Security Policy";
pub const DESCRIPTION: &str = "Reporting security vulnerabilities.";
pub const ROUTE: &str = "security";
pub const ID: &str = "6AF9DDF87B37BBE6E83F5DF2B8F5B86F98F12F5E";

#[component]
pub fn Security() -> Element {
    rsx! {
        Head {
            title: TITLE,
            description: DESCRIPTION,
            url: urls::route(ROUTE),
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
                    class: "relative ml-auto flex space-x-8",
                    a {
                        href: "/email/security",
                        span {
                            class: "fa-solid fa-envelope hover:text-melody-blue dark:hover:text-melody-purple text-4xl",
                        }
                    }
                    a {
                        href: "/keys/security",
                        span {
                            class: "fa-solid fa-key hover:text-melody-blue dark:hover:text-melody-purple text-4xl",
                        }
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
                class: "my-12 w-full",
                h1 {
                    class: "text-5xl lg:text-7xl",
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Security"
                    }
                    " "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple",
                        "Policy"
                    }
                }
                h2 {
                    class: "mt-6 text-2xl lg:text-4xl text-neutral-600 dark:text-neutral-400",
                    "Reporting"
                }
                p {
                    class: "mt-6",
                    "Thank you for taking the time to responsibly disclose any problems you find."
                }
                p {
                    class: "mt-6",
                    "All security vulnerabilities should be reported by email to "
                    a {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        href: "/email/security",
                        "security@nekit.dev"
                    }
                    "."
                }
                p {
                    class: "mt-6",
                    "Your report will be acknowledged within "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        "24 hours"
                    }
                    ", and you will receive a more detailed response within "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        "48 hours"
                    }
                    " indicating the next steps in handling your report."
                }
                p {
                    class: "mt-6",
                    "You can encrypt your report using our public key: "
                    a {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        href: "/keys/security",
                        { ID }
                    }
                    ". This key is also available on "
                    a {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        href: "https://pgp.mit.edu/pks/lookup?op=index&search=0x{ID}",
                        "MIT's Key Server"
                    }
                    "."
                }
            }
        }
    }
}
