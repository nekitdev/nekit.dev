use std::fmt;

use dioxus::prelude::*;

use crate::components::shared::COLUMN;

pub const SECURITY_POLICY: &str = "Security Policy";

pub const EMAIL: &str = "security@nekit.dev";
pub const NAME: &str = "Nikita Tikhonov";
pub const COMMENT: &str = "security";

pub const UID: &str = "uid";
pub const FINGERPRINT: &str = "fingerprint";

pub const ID: &str = "EEBD6848A583A1B4F6B73B60D177918FC00BB74F";
pub const PUBLIC_KEY: &str = r"-----BEGIN PGP PUBLIC KEY BLOCK-----

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

pub const REPORTING: &str = "reporting";
pub const DISCLOSURE: &str = "disclosure";
pub const KEY: &str = "key";
pub const ATTRIBUTION: &str = "attribution";

pub fn uid<N: fmt::Display, C: fmt::Display, E: fmt::Display>(
    name: N,
    comment: C,
    email: E,
) -> String {
    format!("{name} ({comment}) <{email}>")
}

pub const SIZE: usize = 4;
pub const HALF: usize = 2;

pub const SPACE: &str = " ";

pub fn fingerprint<I: AsRef<str>>(id: I) -> (String, String) {
    let string = id.as_ref();

    let groups: Vec<_> = string
        .as_bytes()
        .chunks(SIZE)
        .filter_map(|chunk| str::from_utf8(chunk).ok())
        .collect();

    let half = groups.len() / HALF;

    let first = groups[..half].join(SPACE);
    let last = groups[half..].join(SPACE);

    (first, last)
}

#[component]
pub fn Heading(id: String, title: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-[2ch] border-b-2 border-light-line dark:border-line",
            h2 { class: "font-bold text-light-muted dark:text-muted", id, {title} }
        }
    }
}

#[component]
pub fn Security() -> Element {
    let uid = uid(NAME, COMMENT, EMAIL);
    let (finger, print) = fingerprint(ID);

    rsx! {
        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-12",
            div { class: "flex flex-col gap-2",
                div { class: "text-light-muted dark:text-muted", {COMMENT} }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight", {SECURITY_POLICY} }
            }

            section { class: "flex flex-col gap-4", aria_labelledby: REPORTING,
                Heading { id: REPORTING, title: REPORTING }
                p { "Thank you for taking the time to responsibly disclose any problems you find." }
                p {
                    "All security vulnerabilities should be reported by email to "
                    a {
                        class: "text-light-accent dark:text-accent hover:underline",
                        href: "/email/security",
                        {EMAIL}
                    }
                    "."
                }
                p {
                    "Your report will be acknowledged within "
                    span { class: "font-bold", "24 hours" }
                    ", and you will receive a more detailed response within "
                    span { class: "font-bold", "48 hours" }
                    " indicating the next steps in handling your report."
                }
                p {
                    "After the initial reply to your report, you will be kept informed of the "
                    "progress being made towards a fix and official announcement. "
                    "These updates will be sent at least every "
                    span { class: "font-bold", "5 days" }
                    "; in reality, this is more likely to be every "
                    span { class: "font-bold", "24-48 hours" }
                    "."
                }
            }

            section { class: "flex flex-col gap-4", aria_labelledby: DISCLOSURE,
                Heading { id: DISCLOSURE, title: DISCLOSURE }
                p { "Disclosure follows a 5-step process:" }
                ol { class: "flex flex-col gap-2",
                    li { class: "flex gap-[2ch]",
                        span { class: "text-light-muted dark:text-muted shrink-0", "1." }
                        span {
                            "The security report is received and is assigned a primary handler. "
                            "This person will coordinate the fix and release process."
                        }
                    }
                    li { class: "flex gap-[2ch]",
                        span { class: "text-light-muted dark:text-muted shrink-0", "2." }
                        span {
                            "The problem is confirmed and a list of all affected "
                            "versions is determined."
                        }
                    }
                    li { class: "flex gap-[2ch]",
                        span { class: "text-light-muted dark:text-muted shrink-0", "3." }
                        span { "Code is audited to find any potential similar problems." }
                    }
                    li { class: "flex gap-[2ch]",
                        span { class: "text-light-muted dark:text-muted shrink-0", "4." }
                        span {
                            "Fixes are prepared for all releases which are still under "
                            "maintenance. These fixes are not committed to the public repository "
                            "but rather held locally pending the announcement."
                        }
                    }
                    li { class: "flex gap-[2ch]",
                        span { class: "text-light-muted dark:text-muted shrink-0", "5." }
                        span {
                            "On the embargo date, the changes are pushed to the public repository "
                            "and new builds are deployed."
                        }
                    }
                }
                p {
                    "This process can take some time, especially when coordination is required "
                    "with maintainers of other projects. Every effort will be made to handle "
                    "the issue in as timely a manner as possible, however it is important to "
                    "follow the process above to ensure that disclosure is handled consistently."
                }
            }

            section { class: "flex flex-col gap-4", aria_labelledby: KEY,
                Heading { id: KEY, title: KEY }
                p {
                    "You can encrypt your report using the public key below. "
                    "It is also available on "
                    a {
                        class: "text-light-accent dark:text-accent hover:underline",
                        href: "https://pgp.mit.edu/pks/lookup?op=index&search=0x{ID}",
                        "MIT's key server"
                    }
                    " and as a "
                    a {
                        class: "text-light-accent dark:text-accent hover:underline",
                        href: "/keys/security",
                        "file"
                    }
                    "."
                }
                dl { class: "grid grid-cols-[max-content_1fr] gap-x-[3ch]",
                    dt { class: "text-light-accent dark:text-accent", {UID} }
                    dd { class: "wrap-anywhere", {uid} }
                    dt { class: "text-light-accent dark:text-accent", {FINGERPRINT} }
                    dd { class: "flex flex-wrap gap-x-[2ch]",
                        span { class: "wrap-anywhere", {finger} }
                        span { class: "wrap-anywhere", {print} }
                    }
                }
                pre {
                    class: "overflow-x-auto border-2 border-light-line dark:border-line px-[2ch] py-4",
                    aria_label: "Public Key",
                    {PUBLIC_KEY}
                }
            }

            section { class: "flex flex-col gap-4", aria_labelledby: ATTRIBUTION,
                Heading { id: ATTRIBUTION, title: ATTRIBUTION }
                p {
                    "This policy is adapted from "
                    a {
                        class: "text-light-accent dark:text-accent hover:underline",
                        href: "https://rust-lang.org/policies/security",
                        "Rust's Security Policy"
                    }
                    "."
                }
            }
        }
    }
}
