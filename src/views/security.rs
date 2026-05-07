use dioxus::prelude::*;

use crate::{
    components::head::{Head, NAME},
    routes::Route,
    urls,
};

pub const TITLE: &str = "Security Policy";
pub const DESCRIPTION: &str = "Reporting security vulnerabilities.";
pub const ROUTE: &str = "security";
pub const ID: &str = "EEBD6848A583A1B4F6B73B60D177918FC00BB74F";
pub const KEY: &str = r"-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEac+/LRYJKwYBBAHaRw8BAQdAxZdnBe5EWI4ZV33a9MpRdNtkB3c02mMriiV9
Cv88tQa0L05pa2l0YSBUaWtob25vdiAoc2VjdXJpdHkpIDxzZWN1cml0eUBuZWtp
dC5kZXY+iJAEExYKADgWIQTuvWhIpYOhtPa3O2DRd5GPwAu3TwUCac+/LQIbAwUL
CQgHAgYVCgkICwIEFgIDAQIeAQIXgAAKCRDRd5GPwAu3T6iHAQDe0Q+0rjKMg2rf
TZ1lHXeKmIg1EW8ZB+W/z05ahnHpGwD5AbR0fCxiM5IMqHaEdwJl6fxE9TYDvM7g
NWoRdbiHIAe4OARpz78tEgorBgEEAZdVAQUBAQdAfGJto7sXSt0YbBUDkK901tpA
m08yaiqDJ0MgsytMFkgDAQgHiHgEGBYKACAWIQTuvWhIpYOhtPa3O2DRd5GPwAu3
TwUCac+/LQIbDAAKCRDRd5GPwAu3T4QBAP9v6yLLf9FVAnD3XNGzz9juTk/z+sSL
F9XiwmRXQJ31SQEA/QmBRPc35EFA+zA+wwx8beeeg6A8iYABIpXde15Xgw8=
=fKBT
-----END PGP PUBLIC KEY BLOCK-----";

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
                        aria_label: "Security Report",
                        span {
                            class: "fa-solid fa-envelope hover:text-melody-blue dark:hover:text-melody-purple text-4xl",
                        }
                    }
                    a {
                        href: "/keys/security",
                        aria_label: "Security Key",
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
                        { KEY }
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
