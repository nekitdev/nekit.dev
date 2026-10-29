#[cfg(not(feature = "web"))]
compile_error!("expected `web` feature to be enabled");

use futures_channel::mpsc::UnboundedSender;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Event, EventTarget, HtmlElement, KeyboardEvent, window};

use crate::{
    keys::{
        Direction::{self, Backward, Forward},
        Int, Key, copy,
    },
    theme::Theme,
};

// typing in those never trigger shortcuts
pub const EDITABLE: &str = "input, textarea, select, [contenteditable]";

// reachable links, only in the `main`
pub const LINKS: &str = "main a[href]";

pub const KEY_DOWN: &str = "keydown";

pub const ZERO: Int = 0;

// local storage key
pub const THEME: &str = "theme";

pub const DATA_THEME: &str = "data-theme";

pub fn try_keyboard_event(event: &Event) -> Option<&KeyboardEvent> {
    event.dyn_ref()
}

pub fn try_into_element(target: EventTarget) -> Option<Element> {
    target.dyn_into().ok()
}

pub fn is_browser_shortcut_or_composition(event: &KeyboardEvent) -> bool {
    event.ctrl_key() || event.meta_key() || event.alt_key() || event.is_composing()
}

pub fn editable(event: &KeyboardEvent) -> Option<Element> {
    let element = event.target().and_then(try_into_element)?;

    element.closest(EDITABLE).ok().flatten()
}

pub fn is_editable(event: &KeyboardEvent) -> bool {
    editable(event).is_some()
}

pub fn listen(sender: UnboundedSender<Key>) -> Option<EventListener> {
    let document = window()?.document()?;

    let target = document.clone();

    let listener = EventListener::new(&target, KEY_DOWN, move |event| {
        let Some(event) = try_keyboard_event(event) else {
            return;
        };

        if is_browser_shortcut_or_composition(event) {
            return;
        }

        if is_editable(event) {
            return;
        }

        let string = event.key();
        let code = event.code();

        let Some(key) = Key::parse(string, code) else {
            return;
        };

        event.prevent_default();

        match key {
            Key::Next => {
                let _ = move_focus(&document, Forward);
            }
            Key::Previous => {
                let _ = move_focus(&document, Backward);
            }
            other => {
                // handled by the shortcuts component
                let _ = sender.unbounded_send(other);
            }
        }
    });

    Some(listener)
}

pub fn move_focus(document: &Document, direction: Direction) -> Option<()> {
    let links = document.query_selector_all(LINKS).ok()?;

    let count = links.length();

    let cursor = document.active_element().and_then(|active| {
        (ZERO..count).find(|reference| {
            links
                .item(copy(reference))
                .is_some_and(|link| link.is_same_node(Some(active.as_ref())))
        })
    });

    let next = direction.step(cursor, count)?;

    let link: HtmlElement = links.item(next)?.dyn_into().ok()?;

    link.focus().ok()
}

pub fn load_theme_string() -> Option<String> {
    // uh... yeah ~ nekit
    let theme = window()?
        .local_storage()
        .ok()
        .flatten()?
        .get_item(THEME)
        .ok()
        .flatten()?;

    Some(theme)
}

pub fn load_theme() -> Theme {
    load_theme_string().map(Theme::parse).unwrap_or_default()
}

pub fn apply_theme(theme: Theme) -> Option<()> {
    let window = window()?;

    let root = window.document()?.document_element()?;

    match theme.attribute() {
        Some(value) => {
            root.set_attribute(DATA_THEME, value).ok()?;

            window
                .local_storage()
                .ok()
                .flatten()?
                .set_item(THEME, value)
                .ok()
        }
        None => {
            root.remove_attribute(DATA_THEME).ok()?;

            window
                .local_storage()
                .ok()
                .flatten()?
                .remove_item(THEME)
                .ok()
        }
    }
}
