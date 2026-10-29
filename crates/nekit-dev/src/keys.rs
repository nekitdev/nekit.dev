use std::num::NonZeroU32;

use dioxus::prelude::*;

use crate::routes::Tab;

pub const NEXT_PREFIXED: &str = "KeyJ";
pub const PREVIOUS_PREFIXED: &str = "KeyK";
pub const THEME_PREFIXED: &str = "KeyT";

pub const NEXT: &str = "j";
pub const PREVIOUS: &str = "k";
pub const THEME: &str = "t";

pub const HELP: &str = "?";
pub const HIDE: &str = "Escape";
pub const DIGIT: &str = "Digit";

pub const fn copy<T: Copy>(reference: &T) -> T {
    *reference
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Previous,
    Next,
    Tab(Tab),
    Help,
    Hide,
    Theme,
}

pub const SUB: usize = 1;

impl Key {
    pub fn parse_tab<S: AsRef<str>>(string: S) -> Option<Tab> {
        let one_based: usize = string.as_ref().strip_prefix(DIGIT)?.parse().ok()?;

        let index = one_based.checked_sub(SUB)?;

        let tab = Tab::ALL.get(index).copied()?;

        Some(tab)
    }

    pub fn parse<S: AsRef<str>, C: AsRef<str>>(string: S, code: C) -> Option<Self> {
        let option = match string.as_ref() {
            HELP => Some(Self::Help),
            HIDE => Some(Self::Hide),
            _ => None,
        };

        if option.is_some() {
            return option;
        }

        let parsed = match code.as_ref() {
            NEXT_PREFIXED => Self::Next,
            PREVIOUS_PREFIXED => Self::Previous,
            THEME_PREFIXED => Self::Theme,
            other => {
                let tab = Self::parse_tab(other)?;

                Self::Tab(tab)
            }
        };

        Some(parsed)
    }
}

pub const DEFAULT_HELP: bool = false;

#[derive(Debug, Clone, Copy)]
pub struct Help {
    pub signal: Signal<bool>,
}

impl Default for Help {
    fn default() -> Self {
        Self::new()
    }
}

impl Help {
    pub fn construct(signal: Signal<bool>) -> Self {
        Self { signal }
    }

    pub fn new() -> Self {
        Self::construct(Signal::new(DEFAULT_HELP))
    }

    pub fn get(&self) -> bool {
        (self.signal)()
    }

    pub fn set(&mut self, value: bool) {
        self.signal.set(value);
    }

    pub fn toggle(&mut self) {
        self.signal.toggle();
    }
}

pub fn provide_help() -> Help {
    use_context_provider(Help::new)
}

pub fn use_help() -> Help {
    use_context()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Backward,
    Forward,
}

use Direction::{Backward, Forward};

pub type Int = u32;
pub type NonZeroInt = NonZeroU32;

pub const ZERO: Int = 0;
pub const ONE: Int = 1;

pub type Cursor = Option<Int>;

impl Direction {
    pub const fn step(self, cursor: Cursor, count: Int) -> Cursor {
        let Some(non_zero) = NonZeroInt::new(count) else {
            return None;
        };

        let next = match (cursor, self) {
            (None, Forward) => ZERO, // 0
            (None, Backward) => {
                Self::sub_one(non_zero) // count - 1
            }
            (Some(index), Forward) => {
                Self::wrap(index.wrapping_add(ONE), non_zero) // (index + 1) % count
            }
            (Some(index), Backward) => Self::wrap(
                // ((index % count) + (count - 1)) % count
                Self::wrap(index, non_zero).wrapping_add(Self::sub_one(non_zero)),
                non_zero,
            ),
        };

        Some(next)
    }

    pub const fn sub_one(non_zero: NonZeroInt) -> Int {
        non_zero.get().strict_sub(ONE)
    }

    pub const fn wrap(index: Int, non_zero: NonZeroInt) -> Int {
        index % non_zero.get()
    }
}
