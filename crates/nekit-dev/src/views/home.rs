use dioxus::prelude::*;

use crate::components::{elsewhere::Elsewhere, fetch::Fetch, projects::Projects, shared::COLUMN};

pub const NEKITDEV: &str = "nekitdev";
pub const SLOGAN: &str = "Building awesome software.";

#[component]
pub fn Home() -> Element {
    rsx! {
        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-18",
            div { class: "flex flex-col gap-2",
                div { class: "text-light-muted dark:text-muted", {NEKITDEV} }
                h1 { class: "text-4xl leading-12 font-bold tracking-tight", {SLOGAN} }
            }
            Fetch {}
            Projects {}
            Elsewhere {}
        }
    }
}
