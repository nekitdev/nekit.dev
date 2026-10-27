use pulldown_cmark::{
    Event, MetadataBlockKind, Options, Parser as MarkdownParser, Tag, TagEnd, html,
};

use crate::{
    by::By,
    context::Context,
    math::{self, Display, Error as MathError},
    meta::{Error as MetaError, Meta},
};

pub fn default_options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION
        | Options::ENABLE_HEADING_ATTRIBUTES
        | Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
        | Options::ENABLE_MATH
        | Options::ENABLE_DEFINITION_LIST
        | Options::ENABLE_SUPERSCRIPT
        | Options::ENABLE_SUBSCRIPT
}

pub type Parser<'p> = MarkdownParser<'p>;

pub fn parse(string: &str) -> Parser<'_> {
    let options = default_options();

    let broken_link_callback = None;

    Parser::new_with_broken_link_callback(string, options, broken_link_callback)
}

fn is_metadata(event: Event<'_>) -> bool {
    matches!(
        event,
        Event::Start(Tag::MetadataBlock(MetadataBlockKind::PlusesStyle))
    )
}

fn is_metadata_end(end: TagEnd) -> bool {
    matches!(end, TagEnd::MetadataBlock(MetadataBlockKind::PlusesStyle))
}

pub fn metadata<S: AsRef<str>>(source: S) -> Option<String> {
    let mut events = parse(source.as_ref());

    let first = events.next()?;

    if !is_metadata(first) {
        return None;
    }

    let mut metadata = String::new();

    for event in events {
        match event {
            Event::Text(text) => metadata.push_str(text.as_ref()),
            Event::End(end) if is_metadata_end(end) => break,
            _ => {}
        }
    }

    Some(metadata)
}

pub fn meta<S: AsRef<str>>(source: S) -> Result<Option<Meta>, MetaError> {
    let Some(metadata) = metadata(source) else {
        return Ok(None);
    };

    let meta = Meta::parse(metadata)?;

    Ok(Some(meta))
}

pub fn handle_math<'e>(event: Event<'e>, context: Context<'_>) -> Result<Event<'e>, MathError> {
    match event {
        Event::InlineMath(math) => math::render(math, Display::Inline, context)
            .map(|output| Event::InlineHtml(output.into())),
        Event::DisplayMath(math) => math::render(math, Display::Block, context)
            .map(|output| Event::InlineHtml(output.into())),
        other => Ok(other),
    }
}

pub fn heuristic(length: usize) -> usize {
    length // TODO: improve!
}

pub fn render<S: AsRef<str>>(source: S, context: Context<'_>) -> Result<String, MathError> {
    let string = source.as_ref();

    let result: Result<Vec<_>, _> = parse(string)
        .map(|event| handle_math(event, context))
        .collect();

    let events = result?;

    let capacity = heuristic(string.len());

    let mut output = String::with_capacity(capacity);

    html::push_html(output.by_mut(), events.into_iter());

    Ok(output)
}
