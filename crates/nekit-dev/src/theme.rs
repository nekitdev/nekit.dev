use dioxus::prelude::*;

use non_empty_str::{NonEmptyStr, const_non_empty_str};

#[cfg(feature = "web")]
use crate::browser::{apply_theme, load_theme};

use crate::statics::StaticNonEmptyStr;

pub const SYSTEM: &NonEmptyStr = const_non_empty_str!("system");
pub const DARK: &NonEmptyStr = const_non_empty_str!("dark");
pub const LIGHT: &NonEmptyStr = const_non_empty_str!("light");

pub const MATCH_DARK: &str = DARK.as_str();
pub const MATCH_LIGHT: &str = LIGHT.as_str();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Theme {
    #[default]
    System,
    Dark,
    Light,
}

impl Theme {
    pub const fn next(self) -> Self {
        match self {
            Self::System => Self::Dark,
            Self::Dark => Self::Light,
            Self::Light => Self::System,
        }
    }

    pub const fn name(self) -> StaticNonEmptyStr {
        match self {
            Self::System => SYSTEM,
            Self::Dark => DARK,
            Self::Light => LIGHT,
        }
    }

    pub const fn attribute(self) -> Option<StaticNonEmptyStr> {
        match self {
            Self::System => None,
            Self::Dark => Some(DARK),
            Self::Light => Some(LIGHT),
        }
    }

    pub fn parse<S: AsRef<str>>(string: S) -> Self {
        match string.as_ref() {
            MATCH_DARK => Self::Dark,
            MATCH_LIGHT => Self::Light,
            _ => Self::System,
        }
    }
}

pub fn initial() -> Theme {
    cfg_select! {
        feature = "web" => {
            load_theme()
        }
        _ => {
            Theme::System
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Themes {
    pub signal: Signal<Theme>,
}

impl Default for Themes {
    fn default() -> Self {
        Self::new()
    }
}

impl Themes {
    pub fn construct(signal: Signal<Theme>) -> Self {
        Self { signal }
    }

    pub fn new() -> Self {
        let theme = initial();

        Self::construct(Signal::new(theme))
    }

    pub fn get(&self) -> Theme {
        (self.signal)()
    }

    pub fn set(&mut self, theme: Theme) {
        self.signal.set(theme);

        #[cfg(feature = "web")]
        let _ = apply_theme(theme);
    }

    pub fn cycle(&mut self) {
        let next = self.get().next();

        self.set(next);
    }
}

pub fn provide_themes() -> Themes {
    use_context_provider(Themes::new)
}

pub fn use_themes() -> Themes {
    use_context()
}
