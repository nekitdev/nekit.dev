use pulldown_cmark::{Event, Options, Parser as MarkdownParser, html};
use thiserror::Error;

use crate::{
    by::By,
    context::Context,
    math::{self, Display, Error as MathError},
};

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Math(#[from] MathError),
}

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

pub fn render(string: &str, context: Context<'_>) -> Result<String, Error> {
    let result: Result<Vec<_>, _> = parse(string)
        .map(|event| match event {
            Event::InlineMath(math) => math::render(math, Display::Inline, context)
                .map(|output| Event::InlineHtml(output.into())),
            Event::DisplayMath(math) => math::render(math, Display::Block, context)
                .map(|output| Event::InlineHtml(output.into())),
            other => Ok(other),
        })
        .collect();

    let events = result?;

    let mut output = String::with_capacity(string.len()); // TODO: some heuristic?

    html::push_html(output.by_mut(), events.into_iter());

    Ok(output)
}
