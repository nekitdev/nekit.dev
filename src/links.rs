use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};

pub const ROOT: &NonEmptyStr = const_non_empty_str!("https://nekit.dev/");

pub fn link<S: AsRef<str>>(string: S) -> NonEmptyString {
    let mut output = root();

    output.extend_from(string);

    output
}

pub fn root() -> NonEmptyString {
    ROOT.to_non_empty_string()
}
