//! A basic Markdown view (`markdown_view`'s look, first step): headings, paragraphs, emphasis,
//! lists, links, code, block quotes, rules and tables, in the serif reader voice at
//! `READER_MEASURE`. The Reader port (`pages/reader`) grows it into the full reader (dice links,
//! note links, wide tables, history); keep its signature.
//!
//! ```ignore
//! kit::Markdown::new(("entry", id), &entry.body)
//!     .compact()                       // a side panel: `ProseCompact`
//!     .on_link(cx.listener(|root, url: &str, window, cx| …))
//! ```
use crate::gui::theme::{
    HAIRLINE_WIDTH, L, M, RADIUS_CONTROL, READER_MEASURE, S, Type, TypeExt, XL, XS, colors,
};
use gpui::{
    AnyElement, App, Div, ElementId, FontStyle, FontWeight, HighlightStyle, InteractiveText,
    IntoElement, ParentElement, RenderOnce, SharedString, Styled, StyledText, UnderlineStyle,
    Window, div, prelude::*, px,
};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::ops::Range;
use std::rc::Rc;

type OnLink = Rc<dyn Fn(&str, &mut Window, &mut App)>;

/// Words with their styled runs and links.
#[derive(Default, Clone)]
struct Inline {
    text: String,
    runs: Vec<(Range<usize>, HighlightStyle)>,
    links: Vec<(Range<usize>, String)>,
}

impl Inline {
    fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }
}

/// One block of a document.
enum Block {
    Heading(HeadingLevel, Inline),
    Paragraph {
        inline: Inline,
        quote: usize,
    },
    Item {
        depth: usize,
        mark: String,
        inline: Inline,
        quote: usize,
    },
    Code(String),
    Rule,
    Table {
        rows: Vec<Vec<Inline>>,
    },
}

/// Parse `source` into blocks.
fn blocks(source: &str) -> Vec<Block> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut blocks = Vec::new();
    let mut inline = Inline::default();
    // The style of the words being read: strong, emphasis, code, strikethrough.
    let (mut strong, mut emphasis, mut code, mut struck) = (0usize, 0usize, 0usize, 0usize);
    let mut link: Option<(usize, String)> = None;
    // Each open list: the next number, or None for bullets.
    let mut lists: Vec<Option<u64>> = Vec::new();
    let mut item_mark: Option<String> = None;
    let mut quote = 0usize;
    let mut code_block: Option<String> = None;
    let mut table: Option<Vec<Vec<Inline>>> = None;
    let mut heading: Option<HeadingLevel> = None;

    let flush = |inline: &mut Inline,
                 blocks: &mut Vec<Block>,
                 item_mark: &mut Option<String>,
                 lists: &Vec<Option<u64>>,
                 quote: usize| {
        if inline.is_empty() && item_mark.is_none() {
            *inline = Inline::default();
            return;
        }
        let taken = std::mem::take(inline);
        match item_mark.take() {
            Some(mark) => blocks.push(Block::Item {
                depth: lists.len().saturating_sub(1),
                mark,
                inline: taken,
                quote,
            }),
            None if !lists.is_empty() => blocks.push(Block::Item {
                depth: lists.len().saturating_sub(1),
                mark: String::new(),
                inline: taken,
                quote,
            }),
            None => blocks.push(Block::Paragraph {
                inline: taken,
                quote,
            }),
        }
    };

    for event in Parser::new_ext(source, options) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => heading = Some(level),
            Event::End(TagEnd::Heading(level)) => {
                blocks.push(Block::Heading(level, std::mem::take(&mut inline)));
                heading = None;
            }
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => {
                if table.is_none() && heading.is_none() {
                    flush(&mut inline, &mut blocks, &mut item_mark, &lists, quote);
                }
            }
            Event::Start(Tag::BlockQuote(_)) => quote += 1,
            Event::End(TagEnd::BlockQuote(_)) => quote = quote.saturating_sub(1),
            Event::Start(Tag::List(start)) => {
                flush(&mut inline, &mut blocks, &mut item_mark, &lists, quote);
                lists.push(start);
            }
            Event::End(TagEnd::List(_)) => {
                flush(&mut inline, &mut blocks, &mut item_mark, &lists, quote);
                lists.pop();
            }
            Event::Start(Tag::Item) => {
                flush(&mut inline, &mut blocks, &mut item_mark, &lists, quote);
                let mark = match lists.last_mut() {
                    Some(Some(number)) => {
                        let mark = format!("{number}.");
                        *number += 1;
                        mark
                    }
                    _ => "•".to_owned(),
                };
                item_mark = Some(mark);
            }
            Event::End(TagEnd::Item) => {
                flush(&mut inline, &mut blocks, &mut item_mark, &lists, quote);
            }
            Event::TaskListMarker(done) => {
                item_mark = Some(if done { "☑" } else { "☐" }.to_owned());
            }
            Event::Start(Tag::CodeBlock(_)) => code_block = Some(String::new()),
            Event::End(TagEnd::CodeBlock) => {
                if let Some(code) = code_block.take() {
                    blocks.push(Block::Code(code.trim_end().to_owned()));
                }
            }
            Event::Start(Tag::Table(_)) => {
                flush(&mut inline, &mut blocks, &mut item_mark, &lists, quote);
                table = Some(Vec::new());
            }
            Event::End(TagEnd::Table) => {
                if let Some(rows) = table.take() {
                    blocks.push(Block::Table { rows });
                }
            }
            Event::Start(Tag::TableHead | Tag::TableRow) => {
                if let Some(rows) = &mut table {
                    rows.push(Vec::new());
                }
            }
            Event::End(TagEnd::TableCell) => {
                if let Some(row) = table.as_mut().and_then(|rows| rows.last_mut()) {
                    row.push(std::mem::take(&mut inline));
                }
            }
            Event::Start(Tag::Strong) => strong += 1,
            Event::End(TagEnd::Strong) => strong = strong.saturating_sub(1),
            Event::Start(Tag::Emphasis) => emphasis += 1,
            Event::End(TagEnd::Emphasis) => emphasis = emphasis.saturating_sub(1),
            Event::Start(Tag::Strikethrough) => struck += 1,
            Event::End(TagEnd::Strikethrough) => struck = struck.saturating_sub(1),
            Event::Start(Tag::Link { dest_url, .. }) => {
                link = Some((inline.text.len(), dest_url.to_string()));
            }
            Event::End(TagEnd::Link) => {
                if let Some((start, url)) = link.take() {
                    inline.links.push((start..inline.text.len(), url));
                }
            }
            Event::Code(words) => {
                code += 1;
                push(&mut inline, &words, (strong, emphasis, code, struck), &link);
                code -= 1;
            }
            Event::Text(words) => {
                if let Some(block) = &mut code_block {
                    block.push_str(&words);
                } else {
                    push(&mut inline, &words, (strong, emphasis, code, struck), &link);
                }
            }
            Event::SoftBreak => push(&mut inline, " ", (0, 0, 0, 0), &None),
            Event::HardBreak => push(&mut inline, "\n", (0, 0, 0, 0), &None),
            Event::Rule => blocks.push(Block::Rule),
            Event::Html(words) | Event::InlineHtml(words) => {
                push(&mut inline, &words, (0, 0, 0, 0), &None);
            }
            _ => {}
        }
    }
    flush(&mut inline, &mut blocks, &mut item_mark, &lists, quote);
    blocks
}

/// Add words to the inline run with the open styles.
fn push(
    inline: &mut Inline,
    words: &str,
    (strong, emphasis, code, struck): (usize, usize, usize, usize),
    link: &Option<(usize, String)>,
) {
    let c = colors();
    let start = inline.text.len();
    inline.text.push_str(words);
    let range = start..inline.text.len();
    let mut style = HighlightStyle::default();
    if strong > 0 {
        style.font_weight = Some(FontWeight::SEMIBOLD);
    }
    if emphasis > 0 {
        style.font_style = Some(FontStyle::Italic);
    }
    if code > 0 {
        style.background_color = Some(c.raised);
    }
    if struck > 0 {
        style.strikethrough = Some(gpui::StrikethroughStyle {
            thickness: HAIRLINE_WIDTH,
            color: Some(c.muted),
        });
    }
    if link.is_some() {
        style.color = Some(c.accent);
        style.underline = Some(UnderlineStyle {
            thickness: HAIRLINE_WIDTH,
            color: Some(c.accent),
            wavy: false,
        });
    }
    if style != HighlightStyle::default() && !range.is_empty() {
        inline.runs.push((range, style));
    }
}

/// A Markdown document in the reader's voice.
#[derive(IntoElement)]
pub struct Markdown {
    id: ElementId,
    source: SharedString,
    compact: bool,
    measure: Option<gpui::Pixels>,
    on_link: Option<OnLink>,
}

/// `markdown(id, source)`: a Markdown document (see [`Markdown`]).
pub fn markdown(id: impl Into<ElementId>, source: impl Into<SharedString>) -> Markdown {
    Markdown::new(id, source)
}

impl Markdown {
    pub fn new(id: impl Into<ElementId>, source: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            source: source.into(),
            compact: false,
            measure: Some(READER_MEASURE),
            on_link: None,
        }
    }

    /// `ProseCompact` type, for a side panel narrower than `READER_MEASURE - 60`.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// Fill the parent's width instead of keeping `READER_MEASURE`.
    pub fn full_width(mut self) -> Self {
        self.measure = None;
        self
    }

    /// A press on a link, with its target.
    pub fn on_link(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_link = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Markdown {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let prose = if self.compact {
            Type::ProseCompact
        } else {
            Type::Prose
        };
        let mut count = 0usize;
        let mut next_id = |id: &ElementId| {
            count += 1;
            ElementId::NamedChild(
                std::sync::Arc::new(id.clone()),
                format!("block-{count}").into(),
            )
        };
        let on_link = self.on_link.clone();
        let mut document = div().flex().flex_col().gap(S).w_full();
        if let Some(measure) = self.measure {
            document = document.max_w(measure);
        }
        for block in blocks(&self.source) {
            let element: AnyElement = match block {
                Block::Heading(level, inline) => {
                    let ty = match level {
                        HeadingLevel::H1 => Type::Display,
                        HeadingLevel::H2 => Type::Section,
                        _ => Type::ProseStrong,
                    };
                    let top = if matches!(level, HeadingLevel::H1) {
                        px(0.0)
                    } else {
                        M
                    };
                    div()
                        .pt(top)
                        .ty(ty)
                        .text_color(colors().text)
                        .child(words(next_id(&self.id), inline, &on_link))
                        .into_any_element()
                }
                Block::Paragraph { inline, quote } => quoted(
                    quote,
                    div().ty(prose).text_color(colors().text).child(words(
                        next_id(&self.id),
                        inline,
                        &on_link,
                    )),
                ),
                Block::Item {
                    depth,
                    mark,
                    inline,
                    quote,
                } => quoted(
                    quote,
                    div()
                        .flex()
                        .flex_row()
                        .pl(XL * depth as f32)
                        .ty(prose)
                        .text_color(colors().text)
                        .child(
                            div()
                                .w(XL)
                                .flex_none()
                                .text_color(colors().muted)
                                .child(mark),
                        )
                        .child(div().flex_1().min_w_0().child(words(
                            next_id(&self.id),
                            inline,
                            &on_link,
                        ))),
                ),
                Block::Code(code) => div()
                    .p(S)
                    .rounded(RADIUS_CONTROL)
                    .bg(colors().well)
                    .ty(Type::Mono)
                    .text_color(colors().text)
                    .child(code)
                    .into_any_element(),
                Block::Rule => super::text::rule().my(S).into_any_element(),
                Block::Table { rows } => table(next_id(&self.id), rows, &on_link),
            };
            document = document.child(element);
        }
        document
    }
}

/// A block set in by its quote level, with a rule at its left.
fn quoted(level: usize, block: Div) -> AnyElement {
    if level == 0 {
        return block.into_any_element();
    }
    div()
        .ml(L * (level - 1) as f32)
        .pl(M)
        .border_l(px(2.0))
        .border_color(colors().rule)
        .text_color(colors().muted)
        .child(block)
        .into_any_element()
}

/// Styled words with their links pressable.
fn words(id: ElementId, inline: Inline, on_link: &Option<OnLink>) -> AnyElement {
    let text = StyledText::new(inline.text).with_highlights(inline.runs);
    if inline.links.is_empty() {
        return text.into_any_element();
    }
    let (ranges, urls): (Vec<_>, Vec<_>) = inline.links.into_iter().unzip();
    let on_link = on_link.clone();
    InteractiveText::new(id, text)
        .on_click(ranges, move |index, window, cx| {
            if let (Some(on_link), Some(url)) = (&on_link, urls.get(index)) {
                on_link(url, window, cx);
            }
        })
        .into_any_element()
}

/// A table: each column as wide as its widest cell (a grid of max-content columns), a rule under
/// the header row, no stripes; a table wider than its column scrolls sideways.
fn table(id: ElementId, rows: Vec<Vec<Inline>>, on_link: &Option<OnLink>) -> AnyElement {
    let c = colors();
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0).max(1);
    let mut grid = div()
        .grid()
        .grid_cols_max_content(u16::try_from(columns).unwrap_or(u16::MAX))
        .ty(Type::Stat)
        .text_color(c.text);
    for (row_index, mut row) in rows.into_iter().enumerate() {
        row.resize_with(columns, Inline::default);
        for (column, cell) in row.into_iter().enumerate() {
            let cell_id = ElementId::NamedChild(
                std::sync::Arc::new(id.clone()),
                format!("{row_index}-{column}").into(),
            );
            grid = grid.child(
                div()
                    .py(XS)
                    .pr(M)
                    .when(row_index == 0, |cell| {
                        cell.ty(Type::StatLabel)
                            .border_b(HAIRLINE_WIDTH)
                            .border_color(c.rule)
                    })
                    .child(words(cell_id, cell, on_link)),
            );
        }
    }
    super::scroll::sideways(id, grid).into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_reads_blocks_and_styles() {
        let source = "# Title\n\nSome **strong** and *soft* words with [a link](note.md).\n\n- one\n- two\n  1. inner\n\n> quoted\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n```\ncode\n```\n";
        let blocks = blocks(source);
        let kinds: Vec<&str> = blocks
            .iter()
            .map(|block| match block {
                Block::Heading(..) => "heading",
                Block::Paragraph { quote: 0, .. } => "paragraph",
                Block::Paragraph { .. } => "quote",
                Block::Item { .. } => "item",
                Block::Code(_) => "code",
                Block::Rule => "rule",
                Block::Table { .. } => "table",
            })
            .collect();
        assert_eq!(
            kinds,
            [
                "heading",
                "paragraph",
                "item",
                "item",
                "item",
                "quote",
                "table",
                "code"
            ]
        );
        let Block::Paragraph { inline, .. } = &blocks[1] else {
            unreachable!()
        };
        assert_eq!(inline.links, [(32..38, "note.md".to_owned())]);
        assert_eq!(&inline.text[32..38], "a link");
        assert!(inline.runs.iter().any(|(range, style)| {
            &inline.text[range.clone()] == "strong"
                && style.font_weight == Some(FontWeight::SEMIBOLD)
        }));
        let Block::Item { depth, mark, .. } = &blocks[4] else {
            unreachable!()
        };
        assert_eq!((*depth, mark.as_str()), (1, "1."));
    }
}
