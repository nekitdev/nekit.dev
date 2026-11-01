use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};

pub const ROOT: &NonEmptyStr = const_non_empty_str!("https://nekit.dev");
pub const STATIC: &NonEmptyStr = const_non_empty_str!("https://serve.nekit.dev/static");

pub const SLASH: char = '/';
pub const EMPTY: &str = "";

pub fn route_with<R: AsRef<str>>(mut output: NonEmptyString, route: R) -> NonEmptyString {
    let string = route.as_ref();

    if !string.starts_with(SLASH) {
        output.push(SLASH);
    }

    output.push_str(string);

    output
}

pub fn route<R: AsRef<str>>(route: R) -> NonEmptyString {
    route_with(ROOT.to_owned(), route)
}

pub fn static_route<R: AsRef<str>>(route: R) -> NonEmptyString {
    route_with(STATIC.to_owned(), route)
}

pub fn root() -> NonEmptyString {
    route(EMPTY)
}
