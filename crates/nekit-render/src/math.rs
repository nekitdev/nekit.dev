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
    diag::SourceDiagnostic,
    layout::{Abs, Frame, FrameItem, Point},
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

pub const SEPARATOR: &str = "; ";

pub const SIZE: f64 = 13.0;

pub const DEFAULT_COLOR: &str = "#000000";
pub const CURRENT_COLOR: &str = "currentColor";

fn text_size_with(size: f64) -> String {
    format!("#set text(size: {size}pt)")
}

pub fn text_size() -> String {
    text_size_with(SIZE)
}

#[derive(Debug, Error)]
#[error("{message}", message = self.message())]
pub struct Error {
    string: String,
    messages: Vec<String>,
}

impl Error {
    pub fn message(&self) -> String {
        self.messages.join(SEPARATOR)
    }
}

impl Error {
    pub const fn new(string: String, messages: Vec<String>) -> Self {
        Self { string, messages }
    }

    pub const fn source(&self) -> &str {
        self.string.as_str()
    }

    pub fn into_source(self) -> String {
        self.string
    }

    pub fn get(self) -> (String, Vec<String>) {
        (self.string, self.messages)
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
        string.push(NEW_LINE);

        string.push_str(preamble.as_str());
    }

    push_math(trimmed, display, string.by_mut());

    let detached = Source::detached(string);

    MathWorld::builder()
        .source(detached)
        .maybe_packages(context.packages())
        .build()
}

pub fn diagnostic_message<D: IntoIterator<Item = SourceDiagnostic>>(diagnostics: D) -> Vec<String> {
    diagnostics
        .into_iter()
        .map(|diagnostic| diagnostic.message.to_string())
        .collect()
}

pub fn compile<S: AsRef<str>>(
    source: S,
    display: Display,
    context: Context<'_>,
) -> Result<Output, Error> {
    let string = source.as_ref();

    let world = world(string, display, context);

    let document: PagedDocument = typst::compile(world.by_ref())
        .output
        .map_err(|diagnostics| Error::new(string.to_owned(), diagnostic_message(diagnostics)))?;

    let Some(page) = document.pages().first() else {
        return Ok(Output::new()); // no pages, return empty output
    };

    let frame = page.frame.by_ref();

    let svg = render_svg(document.by_ref(), frame, display);

    let html = match display {
        Display::Inline => {
            let descent = compute_descent(frame);

            inline_html(svg, descent)
        }
        Display::Block => block_html(svg),
    };

    Ok(html)
}

fn compute_descent(frame: &Frame) -> f64 {
    let y = frame.size().y;

    let baseline = frame
        .items()
        .find_map(|(point, item)| try_baseline(point, item))
        .unwrap_or(y);

    em(y - baseline)
}

fn try_baseline(point: &Point, item: &FrameItem) -> Option<Abs> {
    match item {
        FrameItem::Group(group) => Some(point.y + group.frame.baseline()),
        FrameItem::Text(_) => Some(point.y),
        _ => None,
    }
}

fn inline_html<S: AsRef<str>>(svg: S, descent: f64) -> String {
    let mut html = String::new();

    html.push_str(OPEN_INLINE);

    let inline = inline_style(descent);

    html.push_str(inline.as_str());

    html.push_str(CONTINUE_INLINE);

    html.push_str(svg.as_ref());

    html.push_str(CLOSE_INLINE);

    html
}

fn block_html<S: AsRef<str>>(svg: S) -> String {
    let mut string = String::new();

    string.push_str(OPEN_BLOCK);

    string.push_str(svg.as_ref());

    string.push_str(CLOSE_BLOCK);

    string
}

fn render_svg(document: &PagedDocument, frame: &Frame, display: Display) -> String {
    render_svg_with(document, frame, display, SIZE)
}

fn render_svg_with(document: &PagedDocument, frame: &Frame, display: Display, size: f64) -> String {
    let text_size = Abs::pt(size);

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

    let svg = svg_in_html(
        frame,
        text_size,
        pretty,
        id,
        styles,
        anchors,
        resolver.track(),
    );

    svg.replace(DEFAULT_COLOR, CURRENT_COLOR)
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
