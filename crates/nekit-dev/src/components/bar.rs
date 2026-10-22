use std::iter::repeat_n;

use dioxus::prelude::*;

use crate::host::Percent;

pub const PERCENT: f64 = 100.0;

pub const FULL: char = '|';
pub const EMPTY: char = ' ';

pub fn float(int: usize) -> f64 {
    int as f64
}

pub fn int(float: f64) -> usize {
    float.round() as usize
}

pub fn normalize(percent: Percent) -> f64 {
    percent.get() / PERCENT
}

pub fn fill(percent: Percent, cells: usize) -> usize {
    int(normalize(percent) * float(cells)).min(cells) // pythonic rust?! ~ nekit
}

pub fn repeat(character: char, count: usize) -> String {
    repeat_n(character, count).collect()
}

#[component]
pub fn Bar(percent: Percent, cells: usize, content: String) -> Element {
    let filled = fill(percent, cells);
    let emptied = cells - filled;

    let level = percent.reverse();

    let full = repeat(FULL, filled);
    let empty = repeat(EMPTY, emptied);

    let label = content.clone();

    rsx! {
        div { class: "flex flex-wrap gap-x-[1ch]",
            span {
                class: "whitespace-pre",
                role: "meter",
                aria_valuemin: "0",
                aria_valuemax: "100",
                aria_valuenow: "{percent}",
                aria_label: label,
                span { class: "text-light-muted dark:text-muted", "[" }
                span {
                    class: "battery-light dark:battery",
                    style: "--level: {level:#}",
                    "{full}"
                }
                {empty}
                span { class: "text-light-muted dark:text-muted", "]" }
            }
            span { {content} }
        }
    }
}
