use std::cmp::Ordering;

use bon::Builder;
use jiff::civil;
use non_empty_str::{NonEmptyStr, NonEmptyString};
use serde::{Deserialize, Serialize};

use crate::slug::Slug;

pub type Date = civil::Date;

pub fn tag<S: AsRef<str>>(string: S) -> NonEmptyString {
    let new = format!("#{tag}", tag = string.as_ref());

    // SAFETY: there is `#` => `new` is non-empty
    unsafe { NonEmptyString::new_unchecked(new) }
}

pub type Tags = Vec<NonEmptyString>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
pub struct Summary {
    pub slug: Slug,
    pub title: NonEmptyString,
    pub description: NonEmptyString,
    pub date: Date,
    pub updated: Option<Date>,
    pub tags: Tags,
    pub draft: bool,
}

impl Summary {
    pub const fn slug(&self) -> &Slug {
        &self.slug
    }

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

    pub const fn is_published(&self) -> bool {
        !self.is_draft()
    }

    pub const fn date_ref(&self) -> &Date {
        &self.date
    }

    pub fn tags(&self) -> Tags {
        self.tags.iter().map(tag).collect()
    }

    pub fn newest_first(&self, other: &Self) -> Ordering {
        other
            .date_ref()
            .cmp(self.date_ref())
            .then_with(|| self.slug().cmp(other.slug()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Post {
    pub summary: Summary,
    pub content: String,
}

impl Post {
    pub const fn new(summary: Summary, content: String) -> Self {
        Self { summary, content }
    }

    pub const fn summary(&self) -> &Summary {
        &self.summary
    }

    pub const fn content(&self) -> &str {
        self.content.as_str()
    }

    pub const fn slug(&self) -> &Slug {
        self.summary().slug()
    }

    pub const fn is_published(&self) -> bool {
        self.summary().is_published()
    }
}
