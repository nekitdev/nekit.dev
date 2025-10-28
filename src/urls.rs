use dioxus::prelude::*;

pub const ROOT: &str = "https://nekit.dev";

pub const SLASH: char = '/';
pub const EMPTY: &str = "";

pub fn route<R: AsRef<str>>(route: R) -> String {
    let mut output = ROOT.to_owned();

    let string = route.as_ref();

    if !string.starts_with(SLASH) {
        output.push(SLASH);
    }

    output.push_str(string);

    output
}

pub fn root() -> String {
    route(EMPTY)
}

pub fn asset(asset: Asset) -> String {
    route(asset.resolve().to_string_lossy())
}
