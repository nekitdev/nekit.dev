use std::{
    fs::{read_to_string, write},
    io::Error as IoError,
    path::Path,
};

use thiserror::Error;

use crate::{
    context::{Context, Packages},
    markdown::{meta, render},
    math::Error as MathError,
    meta::Error as MetaError,
};

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Io(#[from] IoError),
    Math(#[from] MathError),
    Meta(#[from] MetaError),
}

pub fn render_from<P: AsRef<Path>>(
    path: P,
    packages: Option<Packages<'_>>,
) -> Result<String, Error> {
    let string = read_to_string(path)?;

    let option = meta(string.as_str())?;

    let preamble = if let Some(ref meta) = option {
        meta.preamble()
    } else {
        None
    };

    let context = Context::builder()
        .maybe_packages(packages)
        .maybe_preamble(preamble)
        .build();

    let output = render(string.as_str(), context)?;

    Ok(output)
}

pub fn render_print<P: AsRef<Path>>(path: P, packages: Option<Packages<'_>>) -> Result<(), Error> {
    let string = render_from(path, packages)?;

    println!("{string}");

    Ok(())
}

pub fn render_check<P: AsRef<Path>>(path: P, packages: Option<Packages<'_>>) -> Result<(), Error> {
    let _check = render_from(path, packages)?;

    Ok(())
}

pub fn render_from_to<P: AsRef<Path>, T: AsRef<Path>>(
    path: P,
    to: T,
    packages: Option<Packages<'_>>,
) -> Result<(), Error> {
    let string = render_from(path, packages)?;

    write(to, string)?;

    Ok(())
}
