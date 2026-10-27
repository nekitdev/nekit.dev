use bon::Builder;
use non_empty_str::{NonEmptyStr, NonEmptyString};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use toml::de::Error as TomlError;

use crate::date::{self, Date};

#[derive(Debug, Error)]
#[error("meta error: {toml}")]
pub struct Error {
    #[from]
    toml: TomlError,
}

impl Error {
    pub const fn new(toml: TomlError) -> Self {
        Self { toml }
    }

    pub fn get(self) -> TomlError {
        self.toml
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub title: NonEmptyString,
    pub description: NonEmptyString,
    #[serde(deserialize_with = "date::required")]
    pub date: Date,
    #[serde(default, deserialize_with = "date::optional")]
    pub updated: Option<Date>,
    #[serde(default)]
    pub tags: Vec<NonEmptyString>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub preamble: Option<NonEmptyString>,
}

impl Meta {
    pub const fn title(&self) -> &NonEmptyStr {
        self.title.as_non_empty_str()
    }

    pub const fn description(&self) -> &NonEmptyStr {
        self.description.as_non_empty_str()
    }

    pub const fn date(&self) -> Date {
        self.date
    }

    pub const fn updated(&self) -> Option<Date> {
        self.updated
    }

    pub const fn is_draft(&self) -> bool {
        self.draft
    }

    pub fn preamble(&self) -> Option<&NonEmptyStr> {
        self.preamble.as_deref()
    }

    pub fn parse<M: AsRef<str>>(metadata: M) -> Result<Self, Error> {
        let meta = toml::from_str(metadata.as_ref())?;

        Ok(meta)
    }
}
