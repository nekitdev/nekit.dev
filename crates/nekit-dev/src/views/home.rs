use dioxus::prelude::*;

use crate::components::{fetch::Fetch, projects::Projects, shared::COLUMN};

pub const SLOGAN: &str = "Building awesome software.";

#[component]
pub fn Home() -> Element {
    rsx! {
        main { class: "{COLUMN} grow px-[4ch] pt-18 pb-24 flex flex-col gap-18",
            h1 { class: "text-4xl leading-12 font-bold tracking-tight", {SLOGAN} }
            Fetch {}
            Projects {}
        }
    }
}
