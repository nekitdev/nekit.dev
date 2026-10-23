use std::time::Duration;

use dioxus::prelude::*;

#[cfg(feature = "web")]
use dioxus_sdk::time::sleep;

use non_empty_str::{NonEmptyStr, const_non_empty_str};

use crate::{
    components::{shared::COLUMN, system::System, usage::Usage},
    host,
};

pub const INTERVAL: Duration = Duration::from_secs(2);

pub const HOST: &str = "host";

pub const FETCHING: &str = "Fetching...";

pub const FALLBACK: &NonEmptyStr = const_non_empty_str!("The machine behind the site.");

pub const RECONNECT: Duration = Duration::from_secs(5);

#[component]
pub fn Host() -> Element {
    let initial = use_server_cached(host::optional);

    #[cfg_attr(feature = "server", allow(unused_mut))]
    let mut current = use_signal(|| initial);

    #[cfg(feature = "web")]
    use_future(move || async move {
        loop {
            if let Ok(mut events) = host::events().await {
                while let Some(Ok(host)) = events.recv().await {
                    current.set(Some(host));
                }
            }

            sleep(RECONNECT).await;
        }
    });

    let option = current();

    let name = option
        .as_ref()
        .and_then(|host| host.name())
        .unwrap_or(FALLBACK);

    rsx! {
        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-18",
            div { class: "flex flex-col gap-2",
                div { class: "text-light-muted dark:text-muted", {HOST} }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight", "{name}" }
            }
            match option {
                Some(host) => rsx! {
                    System { host: host.clone() }
                    Usage { host }
                },
                None => rsx! {
                    p { class: "text-light-muted dark:text-muted", {FETCHING} }
                },
            }
        }
    }
}
