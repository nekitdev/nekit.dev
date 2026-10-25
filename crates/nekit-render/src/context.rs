use std::path::Path;

use bon::Builder;
use non_empty_str::NonEmptyStr;

pub type PathRef<'p> = &'p Path;
pub type Str<'s> = &'s NonEmptyStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Builder)]
pub struct Context<'c> {
    pub packages: Option<PathRef<'c>>,
    pub preamble: Option<Str<'c>>,
}

impl<'c> Context<'c> {
    pub const fn packages(self) -> Option<PathRef<'c>> {
        self.packages
    }

    pub const fn preamble(self) -> Option<Str<'c>> {
        self.preamble
    }
}
