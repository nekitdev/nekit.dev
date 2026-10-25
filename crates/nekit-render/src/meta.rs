use jiff::civil;
use non_empty_str::NonEmptyString;
use serde::{Deserialize, Serialize};

pub type Date = civil::Date;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    pub title: NonEmptyString,
    pub description: NonEmptyString,
    pub date: Date,
    #[serde(default)]
    pub updated: Option<Date>,
    #[serde(default)]
    pub tags: Vec<NonEmptyString>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub preamble: Option<NonEmptyString>,
}

impl Meta {
    pub const fn is_draft(&self) -> bool {
        self.draft
    }
}
