use std::time::Duration;

use dioxus::prelude::*;
use dioxus_sdk::time::use_interval;

use crate::times::clock;

pub const INTERVAL: Duration = Duration::from_secs(1);

#[component]
pub fn Clock() -> Element {
    let initial = use_server_cached(clock);

    let mut current = use_signal(|| initial);

    use_interval(INTERVAL, move |()| {
        current.set(clock());
    });

    rsx! {
        time { "{current}" }
    }
}
