use std::time::Duration;

use dioxus::prelude::*;
use dioxus_sdk::time::use_interval;
use non_empty_str::{NonEmptyStr, const_non_empty_str};

use crate::{
    components::{shared::COLUMN, system::System, usage::Usage},
    host::status,
};

pub const INTERVAL: Duration = Duration::from_secs(2);

pub const HOST: &str = "host";

pub const UNREACHABLE: &str = "Unfortunately, the host is unreachable at the moment...";

pub const FALLBACK: &NonEmptyStr = const_non_empty_str!("The machine behind the site.");

pub fn schrodinger<T: Clone, E>(resource: Resource<Result<T, E>>) -> Option<T> {
    resource.read().as_ref()?.as_ref().ok().cloned() // loading or failed? does not exist! ~ nekit
}

#[component]
pub fn Host() -> Element {
    let resource = use_server_future(status)?;

    let mut latest = use_signal(|| None);

    use_interval(INTERVAL, move |()| async move {
        if let Ok(host) = status().await {
            latest.set(Some(host));
        }
    });

    let current = latest().or_else(|| schrodinger(resource));

    let name = current
        .as_ref()
        .and_then(|host| host.name())
        .unwrap_or(FALLBACK);

    rsx! {
        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-18",
            div { class: "flex flex-col gap-2",
                div { class: "text-light-muted dark:text-muted", {HOST} }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight", "{name}" }
            }
            match current {
                Some(host) => rsx! {
                    System { host: host.clone() }
                    Usage { host }
                },
                None => rsx! {
                    p { class: "text-light-muted dark:text-muted", {UNREACHABLE} }
                },
            }
        }
    }
}
