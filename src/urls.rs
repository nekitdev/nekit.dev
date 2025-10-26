use dioxus::prelude::*;
use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};

pub const ROOT: &NonEmptyStr = const_non_empty_str!("https://nekit.dev");

pub const SLASH: char = '/';
pub const EMPTY: &str = "";

pub fn route<R: AsRef<str>>(route: R) -> NonEmptyString {
    let mut output = ROOT.to_non_empty_string();

    let string = route.as_ref();

    if !string.starts_with(SLASH) {
        output.push(SLASH);
    }

    output.push_str(string);

    output
}

pub fn root() -> NonEmptyString {
    route(EMPTY)
}

pub fn asset(asset: Asset) -> NonEmptyString {
    route(asset.resolve().to_string_lossy())
}
