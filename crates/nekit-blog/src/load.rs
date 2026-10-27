#[cfg(not(feature = "load"))]
compile_error!("expected `load` to be enabled");

use std::{
    fs::{read_dir, read_to_string},
    io::Error as IoError,
    path::{Path, PathBuf},
};

use nekit_render::{
    context::{Context, Packages},
    markdown::{meta, render},
    math::Error as MathError,
    meta::{Error as MetaError, Meta},
};
use thiserror::Error;

use crate::{
    blog::Blog,
    post::{Post, Summary},
    slug::{Error as ParseSlugError, Slug},
    stem::{Error as StemError, PathStem},
};

pub const POSTS: &str = "posts";
pub const PACKAGES: &str = "packages";
pub const EXTENSION: &str = "md";

#[derive(Debug, Error)]
#[error("missing metadata")]
pub struct MissingMetaError;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum SlugError {
    Stem(#[from] StemError),
    Parse(#[from] ParseSlugError),
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum PostError {
    Slug(#[from] SlugError),
    Io(#[from] IoError),
    Meta(#[from] MetaError),
    MissingMeta(#[from] MissingMetaError),
    Math(#[from] MathError),
}

#[derive(Debug, Error)]
#[error("failed to load `{path}`: {error}")]
pub struct Failure {
    path: PathBuf,
    #[source]
    error: PostError,
}

impl Failure {
    pub const fn new(path: PathBuf, error: PostError) -> Self {
        Self { path, error }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn into_error(self) -> PostError {
        self.error
    }

    pub fn get(self) -> (PathBuf, PostError) {
        (self.path, self.error)
    }
}

#[derive(Debug, Error)]
#[error("failed to read posts from `{path}`: {error}")]
pub struct Error {
    path: PathBuf,
    #[source]
    error: IoError,
}

impl Error {
    pub const fn new(path: PathBuf, error: IoError) -> Self {
        Self { path, error }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn into_error(self) -> IoError {
        self.error
    }

    pub fn get(self) -> (PathBuf, IoError) {
        (self.path, self.error)
    }
}

pub type Failures = Vec<Failure>;

#[derive(Debug)]
pub struct Load {
    pub blog: Blog,
    pub failures: Failures,
}

impl Default for Load {
    fn default() -> Self {
        Self::new()
    }
}

impl Load {
    pub const fn new() -> Self {
        Self::with(Blog::new(), Failures::new())
    }

    pub const fn with(blog: Blog, failures: Failures) -> Self {
        Self { blog, failures }
    }

    pub const fn is_clean(&self) -> bool {
        self.failures.is_empty()
    }

    pub fn post(&mut self, post: Post) {
        self.blog.insert(post);
    }

    pub fn failure(&mut self, failure: Failure) {
        self.failures.push(failure);
    }

    pub fn split(self) -> (Blog, Failures) {
        (self.blog, self.failures)
    }
}

pub fn is_post<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();

    path.is_file()
        && path
            .extension()
            .is_some_and(|extension| extension == EXTENSION)
}

pub fn slug<P: AsRef<Path>>(path: P) -> Result<Slug, SlugError> {
    let stem = path.path_stem_owned()?;

    let slug = Slug::new(stem)?;

    Ok(slug)
}

pub fn combine(slug: Slug, meta: Meta) -> Summary {
    Summary::builder()
        .slug(slug)
        .title(meta.title)
        .description(meta.description)
        .date(meta.date)
        .maybe_updated(meta.updated)
        .tags(meta.tags)
        .draft(meta.draft)
        .build()
}

pub fn post<P: AsRef<Path>>(path: P, packages: Option<Packages<'_>>) -> Result<Post, PostError> {
    let path = path.as_ref();

    let slug = slug(path)?;

    let source = read_to_string(path)?;

    let meta = meta(source.as_str())?.ok_or(MissingMetaError)?;

    let context = Context::builder()
        .maybe_packages(packages)
        .maybe_preamble(meta.preamble())
        .build();

    let content = render(source, context)?;

    let summary = combine(slug, meta);

    let post = Post::new(summary, content);

    Ok(post)
}

pub type Paths = Vec<PathBuf>;

fn post_paths<D: AsRef<Path>>(directory: D) -> Result<Paths, IoError> {
    let mut paths = Paths::new();

    let reader = read_dir(directory)?;

    for result in reader {
        let entry = result?;

        let path = entry.path();

        if is_post(path.as_path()) {
            paths.push(path);
        }
    }

    paths.sort();

    Ok(paths)
}

pub fn load<R: AsRef<Path>>(root: R) -> Result<Load, Error> {
    let root = root.as_ref();

    let directory = root.join(POSTS);

    let paths = post_paths(directory.as_path()).map_err(|error| Error::new(directory, error))?;

    let directory = root.join(PACKAGES);

    let packages = directory.is_dir().then_some(directory.as_path());

    let mut load = Load::new();

    paths
        .into_iter()
        .for_each(|path| match post(path.as_path(), packages) {
            Ok(post) => {
                load.post(post);
            }
            Err(error) => load.failure(Failure::new(path, error)),
        });

    Ok(load)
}
