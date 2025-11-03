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
                    " and reproduced "
                    a {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        href: "#key",
                        "below"
                    }
                    "."
                }
                p {
                    class: "mt-6",
                    "After the initial reply to your report, the core team will try to keep you "
                    "informed of the progress being made towards a fix and official announcement. "
                    "These updates will be sent at least every "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        "5 days"
                    }
                    ". "
                    "In reality, this is more likely to be every "
                    span {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        "24-48 hours"
                    }
                    "."
                }
                h2 {
                    class: "mt-6 text-2xl lg:text-4xl text-neutral-600 dark:text-neutral-400",
                    "Disclosure"
                }
                p {
                    class: "mt-6",
                    "Software has a 5-step disclosure process:"
                    ol {
                        class: "mt-6 list-decimal list-inside",
                        li {
                            "The security report is received and is assigned a primary handler. "
                            "This person will coordinate the fix and release process."
                        }
                        li {
                            "The problem is confirmed and a list of all affected versions is determined."
                        }
                        li {
                            "Code is audited to find any potential similar problems."
                        }
                        li {
                            "Fixes are prepared for all releases which are still under maintenance. "
                            "These fixes are not committed to the public repository but rather "
                            "held locally pending the announcement."
                        }
                        li {
                            "On the embargo date, the changes are pushed to the public repository "
                            "and new builds are deployed."
                        }
                    }
                }
                p {
                    class: "mt-6",
                    "This process can take some time, especially when coordination is required "
                    "with maintainers of other projects. Every effort will be made to handle "
                    "the issue in as timely a manner as possible, however it is important that "
                    "we follow the release process above to ensure that the disclosure is handled "
                    "in a consistent manner."
                }
                h2 {
                    id: "key",
                    class: "mt-6 text-2xl lg:text-4xl text-neutral-600 dark:text-neutral-400",
                    "Key"
                }
                div {
                    class: "mt-6 p-1 bg-gradient-to-b from-melody-purple to-melody-blue rounded-lg",
                    pre {
                        class: "p-4 overflow-auto rounded-lg bg-neutral-50 dark:bg-neutral-900",
                        r"-----BEGIN PGP PUBLIC KEY BLOCK-----

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
pbmAGLbBB2TBok4=
=Ir8m
-----END PGP PUBLIC KEY BLOCK-----"
                    }
                }
                h2 {
                    class: "mt-6 text-2xl lg:text-4xl text-neutral-600 dark:text-neutral-400",
                    "Attribution"
                }
                p {
                    class: "mt-6",
                    "This Security Policy is adapted from "
                    a {
                        class: "hover:text-melody-blue dark:hover:text-melody-purple underline",
                        href: "https://rust-lang.org/policies/security",
                        "Rust's Security Policy"
                    }
                    "."
                }
            }
        }
    }
}
