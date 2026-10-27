#[cfg(not(feature = "load"))]
compile_error!("expected `load` to be enabled");

use std::{path::Path, str::Utf8Error};

use non_empty_str::{EmptyStr, NonEmptyStr, NonEmptyString};
use thiserror::Error;

mod sealed {
    pub trait Sealed {}
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("path stem not found")]
    NotFound,

    #[error("empty path stem encountered")]
    Empty(#[from] EmptyStr),

    #[error("invalid utf-8 in path stem")]
    Utf8(#[from] Utf8Error),
}

pub trait PathStem: sealed::Sealed {
    fn path_stem(&self) -> Result<&NonEmptyStr, Error>;

    fn path_stem_owned(&self) -> Result<NonEmptyString, Error> {
        self.path_stem().map(ToOwned::to_owned)
    }
}

impl<P: AsRef<Path>> sealed::Sealed for P {}

impl<P: AsRef<Path>> PathStem for P {
    fn path_stem(&self) -> Result<&NonEmptyStr, Error> {
        let path = self.as_ref();

        let string: &str = path.file_stem().ok_or(Error::NotFound)?.try_into()?;

        let stem = string.try_into()?;

        Ok(stem)
    }
}
