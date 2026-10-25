use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{LazyLock, Mutex, PoisonError},
};

use bon::Builder;
use non_empty_str::{NonEmptyStr, NonEmptyString, const_non_empty_str};
use thiserror::Error;
use typst::{
    comemo::Track,
    layout::{Abs, FrameItem},
    model::LateLinkResolver,
    syntax::Source,
};
use typst_layout::PagedDocument;
use typst_svg::svg_in_html;

use crate::{by::By, context::Context, world::MathWorld};

pub const DISPLAY_BLOCK: &str = "display: block";
pub const MARGIN_INLINE_AUTO: &str = "margin-inline: auto";

pub const DELIMITER: char = '$';
pub const NEW_LINE: char = '\n';
pub const SPACE: char = ' ';

pub const PREAMBLE: &str = include_str!("preamble.typ");

pub const NON_EMPTY_PREAMBLE: &NonEmptyStr = const_non_empty_str!(PREAMBLE);

pub const SIZE: f64 = 16.0;

pub const DEFAULT_COLOR: &str = "#000000";
pub const CURRENT_COLOR: &str = "currentColor";

fn text_size_with(size: f64) -> String {
    format!("#set text(size: {size}pt)")
}

pub fn text_size() -> String {
    text_size_with(SIZE)
}

#[derive(Debug, Error)]
#[error("failed to render")]
pub struct Error {
    string: String,
}

impl Error {
    pub const fn new(string: String) -> Self {
        Self { string }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Display {
    Inline,
    Block,
}

impl Display {
    pub const fn is_inline(self) -> bool {
        matches!(self, Self::Inline)
    }

    pub const fn is_block(self) -> bool {
        matches!(self, Self::Block)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder)]
struct Key {
    display: Display,
    packages: Option<PathBuf>,
    preamble: Option<NonEmptyString>,
    source: String,
}

pub type Output = String;

type CacheMap = HashMap<Key, Output>;

fn extract<T>(poison_error: PoisonError<T>) -> T {
    poison_error.into_inner()
}

fn setup_cache() -> Mutex<CacheMap> {
    Mutex::new(CacheMap::new())
}

fn try_cached(key: &Key) -> Option<Output> {
    CACHE.lock().unwrap_or_else(extract).get(key).cloned()
}

fn set_cached(key: Key, output: Output) {
    CACHE.lock().unwrap_or_else(extract).insert(key, output);
}

static CACHE: LazyLock<Mutex<CacheMap>> = LazyLock::new(setup_cache);

pub fn render<S: AsRef<str>>(
    source: S,
    display: Display,
    context: Context<'_>,
) -> Result<Output, Error> {
    let string = source.as_ref();

    let key = Key::builder()
        .display(display)
        .maybe_packages(context.packages().map(ToOwned::to_owned))
        .maybe_preamble(context.preamble().map(ToOwned::to_owned))
        .source(string.to_owned())
        .build();

    if let Some(cached) = try_cached(key.by_ref()) {
        return Ok(cached);
    }

    let compiled = compile(source, display, context)?;

    set_cached(key, compiled.clone());

    Ok(compiled)
}

fn push_math<M: AsRef<str>>(math: M, display: Display, string: &mut String) {
    string.push(NEW_LINE);

    string.push(DELIMITER); // $

    if display.is_block() {
        string.push(SPACE);
    }

    string.push_str(math.as_ref());

    if display.is_block() {
        string.push(SPACE);
    }

    string.push(DELIMITER); // $
}

fn world<S: AsRef<str>>(source: S, display: Display, context: Context<'_>) -> MathWorld<'_> {
    let trimmed = source.as_ref().trim();

    let mut string = PREAMBLE.to_owned();

    string.push_str(text_size().as_str());

    if let Some(preamble) = context.preamble() {
        string.push_str(preamble.as_str());
    };

    push_math(trimmed, display, string.by_mut());

    let packages = context.packages();

    let detached = Source::detached(string);

    MathWorld::builder()
        .source(detached)
        .maybe_packages(packages)
        .build()
}

// TODO: refactor!
pub fn compile<S: AsRef<str>>(
    source: S,
    display: Display,
    context: Context<'_>,
) -> Result<Output, Error> {
    let string = source.as_ref();

    let world = world(string, display, context);

    let document: PagedDocument = typst::compile(world.by_ref())
        .output
        .map_err(|errors| Error::new(string.to_owned()))?; // TODO: report errors

    let Some(page) = document.pages().first() else {
        return Ok(Output::new()); // no pages, return empty output
    };

    let frame = page.frame.by_ref();

    let y = frame.size().y;

    let baseline = frame
        .items()
        .find_map(|(point, item)| match item {
            FrameItem::Group(group) => Some(point.y + group.frame.baseline()),
            FrameItem::Text(_) => Some(point.y),
            _ => None,
        })
        .unwrap_or(y);

    let descent = em(y - baseline);

    let text_size = Abs::pt(SIZE);

    let pretty = false; // no pretty printing

    let id = None; // do not set any id

    let style = match display {
        Display::Inline => DISPLAY_BLOCK.to_owned(),
        Display::Block => format!("{DISPLAY_BLOCK}; {MARGIN_INLINE_AUTO}"),
    };

    let styles = style.as_str();

    let no_anchors = [];

    let anchors = no_anchors.as_slice();

    let introspector = document.introspector().as_ref();

    let resolver = LateLinkResolver::new(None, introspector);

    let typst_svg = svg_in_html(
        frame,
        text_size,
        pretty,
        id,
        styles,
        anchors,
        resolver.track(),
    );

    let svg = typst_svg.replace(DEFAULT_COLOR, CURRENT_COLOR);

    let html = match display {
        Display::Inline => {
            let style = inline_style(descent);

            format!("{OPEN_INLINE}{style}{CONTINUE_INLINE}{svg}{CLOSE_INLINE}")
        }
        Display::Block => format!("{OPEN_BLOCK}{svg}{CLOSE_BLOCK}"),
    };

    Ok(html)
}

const PRECISION: usize = 3;

fn inline_style(descent: f64) -> String {
    format!("display: inline-block; vertical-align: -{descent:.PRECISION$}em")
}

// TODO: aria-label?
const OPEN_INLINE: &str = r#"<span class="math math-inline" role="img" style=""#;
const CONTINUE_INLINE: &str = r#"">"#;
const CLOSE_INLINE: &str = "</span>";

// TODO: aria-label?
const OPEN_BLOCK: &str = r#"<span class="math math-block" role="img" style="
  display: block; text-align: center; margin: 1em 0; overflow-x: auto
">"#;

const CLOSE_BLOCK: &str = "</span>";

fn em_with(length: Abs, size: f64) -> f64 {
    length.to_pt() / size
}

fn em(length: Abs) -> f64 {
    em_with(length, SIZE)
}
