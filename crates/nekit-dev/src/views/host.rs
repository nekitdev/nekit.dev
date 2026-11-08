use std::time::Duration;

use dioxus::prelude::*;

#[cfg(feature = "web")]
use dioxus_sdk::time::sleep;

use non_empty_str::{NonEmptyStr, const_non_empty_str};

use crate::{
    components::{meta::RouteMeta, shared::COLUMN, system::System, usage::Usage},
    host,
};

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

    rsx! {
        RouteMeta {}

        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-18",
            div { class: "flex flex-col gap-2",
                div { class: "text-light-muted dark:text-muted", {HOST} }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight",
                    if let Some(ref host) = option {
                        "{host.display_name()}"
                    } else {
                        "{FALLBACK}"
                    }
                }
            }
            if let Some(host) = option {
                System { host: host.clone() }
                Usage { host }
            } else {
                p { class: "text-light-muted dark:text-muted", {FETCHING} }
            }
        }
    }
}
