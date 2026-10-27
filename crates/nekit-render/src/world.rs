use std::{fs::read as read_file, sync::LazyLock};

use bon::Builder;
use typst::{
    Library, LibraryExt, World,
    diag::{FileError, FileResult, PackageError},
    foundations::{Bytes, Datetime, Duration},
    syntax::{FileId, Source, VirtualRoot, package::PackageSpec},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_assets::fonts;

use crate::{by::By, context::Packages};

struct Fonts {
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
}

impl Fonts {
    pub fn construct(book: LazyHash<FontBook>, fonts: Vec<Font>) -> Self {
        Self { book, fonts }
    }

    pub fn new() -> Self {
        let fonts: Vec<_> = fonts().map(Bytes::new).flat_map(Font::iter).collect();

        let book = FontBook::from_fonts(fonts.iter());

        let lazy = LazyHash::new(book);

        Self::construct(lazy, fonts)
    }

    pub fn book(&self) -> &LazyHash<FontBook> {
        self.book.by_ref()
    }

    pub fn font(&self, index: usize) -> Option<&Font> {
        self.fonts.get(index)
    }
}

static FONTS: LazyLock<Fonts> = LazyLock::new(Fonts::new);

static LIBRARY: LazyLock<LazyHash<Library>> = LazyLock::new(library);

fn library() -> LazyHash<Library> {
    let default_library = Library::default();

    LazyHash::new(default_library)
}

#[derive(Builder)]
pub struct MathWorld<'m> {
    source: Source,
    packages: Option<Packages<'m>>,
}

pub type Data = Vec<u8>;

impl MathWorld<'_> {
    fn format_join(spec: &PackageSpec) -> String {
        format!(
            "{namespace}/{name}/{version}",
            namespace = spec.namespace,
            name = spec.name,
            version = spec.version
        )
    }

    fn source(&self) -> &Source {
        self.source.by_ref()
    }

    fn packages(&self) -> Option<Packages<'_>> {
        self.packages
    }

    fn read(&self, id: FileId) -> FileResult<Data> {
        // non-packages are inaccessible
        let VirtualRoot::Package(spec) = id.root() else {
            return Err(FileError::AccessDenied);
        };

        let not_found = || FileError::Package(PackageError::NotFound(spec.clone()));

        let join = Self::format_join(spec);

        let root = self.packages().ok_or_else(not_found)?.join(join);

        // non-directories => packages not found
        if !root.is_dir() {
            return Err(not_found());
        }

        // deny access for paths escaping package root
        let owned = id
            .vpath()
            .realize(root.as_path())
            .map_err(|_| FileError::AccessDenied)?;

        let path = owned.as_path();

        let data = read_file(path).map_err(|error| FileError::from_io(error, path))?;

        Ok(data)
    }

    fn read_string(&self, id: FileId) -> FileResult<String> {
        let data = self.read(id)?;

        let string = String::from_utf8(data).map_err(|_| FileError::InvalidUtf8)?;

        Ok(string)
    }
}

impl World for MathWorld<'_> {
    fn library(&self) -> &LazyHash<Library> {
        LIBRARY.by_ref()
    }

    fn book(&self) -> &LazyHash<FontBook> {
        FONTS.book()
    }

    fn font(&self, index: usize) -> Option<Font> {
        FONTS.font(index).cloned()
    }

    fn main(&self) -> FileId {
        self.source().id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        let main = self.source();

        if id == main.id() {
            Ok(main.clone())
        } else {
            let string = self.read_string(id)?;

            let source = Source::new(id, string);

            Ok(source)
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.read(id).map(Bytes::new)
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        None // date-time is not accessible
    }
}
