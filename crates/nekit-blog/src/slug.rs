use std::{borrow::Borrow, fmt, str::FromStr};

use non_empty_str::{EmptyStr, NonEmptyStr, NonEmptyString};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use thiserror::Error;

pub const SEPARATOR: char = '-';

#[derive(Debug, Error)]
#[error("invalid slug `{string}`; expected `[a-z]`, `[0-9]` and inner `{SEPARATOR}`)")]
pub struct Error {
    string: NonEmptyString,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ParseError {
    Empty(#[from] EmptyStr),
    Slug(#[from] Error),
}

impl Error {
    const fn new(string: NonEmptyString) -> Self {
        Self { string }
    }

    pub fn get(self) -> NonEmptyString {
        self.string
    }
}

fn is_valid_char(character: char) -> bool {
    character.is_ascii_lowercase() || character.is_ascii_digit() || character == SEPARATOR
}

fn is_valid<S: AsRef<NonEmptyStr>>(source: S) -> bool {
    let string = source.as_ref();

    string.chars().into_iter().all(is_valid_char)
        && !string.starts_with(SEPARATOR)
        && !string.ends_with(SEPARATOR)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slug {
    string: NonEmptyString,
}

impl Serialize for Slug {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_non_empty_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Slug {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = NonEmptyString::deserialize(deserializer)?;

        let slug = Slug::new(string).map_err(D::Error::custom)?;

        Ok(slug)
    }
}

impl Borrow<NonEmptyStr> for Slug {
    fn borrow(&self) -> &NonEmptyStr {
        self.as_non_empty_str()
    }
}

impl AsRef<NonEmptyStr> for Slug {
    fn as_ref(&self) -> &NonEmptyStr {
        self.as_non_empty_str()
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_non_empty_str().fmt(formatter)
    }
}

impl FromStr for Slug {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let non_empty = NonEmptyStr::try_from_str(string)?;

        let string = non_empty.to_owned();

        let slug = Self::new(string)?;

        Ok(slug)
    }
}

impl Slug {
    pub fn new(string: NonEmptyString) -> Result<Self, Error> {
        if is_valid(string.as_non_empty_str()) {
            // SAFETY: just checked that the string is valid
            let slug = unsafe { Self::new_unchecked(string) };

            Ok(slug)
        } else {
            let error = Error::new(string);

            Err(error)
        }
    }

    pub const unsafe fn new_unchecked(string: NonEmptyString) -> Self {
        Self { string }
    }

    pub const fn as_non_empty_str(&self) -> &NonEmptyStr {
        self.string.as_non_empty_str()
    }

    pub const fn as_str(&self) -> &str {
        self.string.as_str()
    }

    pub fn get(self) -> NonEmptyString {
        self.string
    }

    pub fn is_valid<S: AsRef<NonEmptyStr>>(string: S) -> bool {
        is_valid(string)
    }
}
