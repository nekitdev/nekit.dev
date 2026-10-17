use dioxus::prelude::*;

use futures_channel::mpsc::unbounded;
use futures_util::StreamExt;

#[cfg(feature = "web")]
use crate::browser::listen;

use crate::{
    keys::{HELP, Key, NEXT, PREVIOUS, use_help},
    routes::Tab,
};

pub const ESC: &str = "esc";
pub const ENTER: &str = "enter";

pub const SHORTCUTS: &str = "Shortcuts";

pub fn tabs() -> String {
    format!("1 - {count}", count = Tab::COUNT)
}

#[component]
pub fn HelpPanel() -> Element {
    let tabs = tabs();

    rsx! {
        aside {
            class: "
            fixed right-[2ch] bottom-14 z-10 px-[2ch] py-2
            bg-light-bar dark:bg-bar border-2 border-light-line dark:border-line
            ",
            aria_label: SHORTCUTS,
            dl { class: "grid grid-cols-[max-content_1fr] gap-x-[3ch]",
                dt { class: "text-light-accent dark:text-accent", "{NEXT} / {PREVIOUS}" }
                dd { "next / previous link" }
                dt { class: "text-light-accent dark:text-accent", {tabs} }
                dd { "switch tab" }
                dt { class: "text-light-accent dark:text-accent", {ENTER} }
                dd { "open" }
                dt { class: "text-light-accent dark:text-accent", {HELP} }
                dd { "toggle help" }
                dt { class: "text-light-accent dark:text-accent", {ESC} }
                dd { "hide help" }
            }
        }
    }
}

#[component]
pub fn HelpToggle() -> Element {
    let mut help = use_help();

    rsx! {
        button {
            class: "cursor-pointer hover:underline",
            r#type: "button",
            aria_label: SHORTCUTS,
            aria_expanded: "{help.get()}",
            onclick: move |_| help.toggle(),
            span { class: "text-light-accent dark:text-accent", {HELP} }
        }
    }
}

pub fn unbounded_keys() -> (UnboundedSender<Key>, UnboundedReceiver<Key>) {
    unbounded()
}

#[component]
pub fn Shortcuts() -> Element {
    let mut help = use_help();

    let navigator = use_navigator();

    use_future(move || async move {
        let (sender, mut keys) = unbounded_keys();

        cfg_select! {
            feature = "web" => {
                let _listener = listen(sender);
            }
            _ => {
                drop(sender);
            }
        }

        while let Some(key) = keys.next().await {
            match key {
                Key::Next | Key::Previous => {
                    // handled by the browser
                }
                Key::Tab(tab) => {
                    navigator.push(tab.route());
                }
                Key::Help => help.toggle(),
                Key::Hide => help.set(false),
            }
        }
    });

    rsx! {
        if help.get() {
            HelpPanel {}
        }
    }
}
