pub const ROOT: &str = "https://nekit.dev";
pub const STATIC: &str = "https://serve.nekit.dev/static";

pub const SLASH: char = '/';
pub const EMPTY: &str = "";

pub fn route_with<R: AsRef<str>>(mut output: String, route: R) -> String {
    let string = route.as_ref();

    if !string.starts_with(SLASH) {
        output.push(SLASH);
    }

    output.push_str(string);

    output
}

pub fn route<R: AsRef<str>>(route: R) -> String {
    route_with(ROOT.to_owned(), route)
}

pub fn static_route<R: AsRef<str>>(route: R) -> String {
    route_with(STATIC.to_owned(), route)
}

pub fn root() -> String {
    route(EMPTY)
}
