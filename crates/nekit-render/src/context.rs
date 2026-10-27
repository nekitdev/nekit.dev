use std::path::Path;

use bon::Builder;
use non_empty_str::NonEmptyStr;

pub type Packages<'p> = &'p Path;
pub type Preamble<'p> = &'p NonEmptyStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Builder)]
pub struct Context<'c> {
    packages: Option<Packages<'c>>,
    preamble: Option<Preamble<'c>>,
}

impl<'c> Context<'c> {
    pub const fn packages(self) -> Option<Packages<'c>> {
        self.packages
    }

    pub const fn preamble(self) -> Option<Preamble<'c>> {
        self.preamble
    }
}
