//! Markdown made into the reader's blocks, as `markdown_view::prepare` makes them for egui: the
//! links resolved, the figures found, each paragraph's words styled. Nothing here lays out or
//! draws; `super` draws the blocks. Keep the two in step until the port's cleanup deletes egui's.
use crate::gui::icons;
use crate::gui::theme::{L, M, S, Type, XL, XS, colors};
use crate::markdown_view::{FILE_LINK, LinkTarget};
use gpui::{Hsla, Pixels};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::borrow::Cow;
use std::ops::Range;
use std::path::PathBuf;

/// How a stretch of words looks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Look {
    /// The face: its family, weight and style (its size is its paragraph's).
    pub role: Type,
    pub color: Hsla,
    pub underline: bool,
    pub struck: bool,
    /// The fill behind code among words.
    pub fill: Option<Hsla>,
    /// Slanted: emphasis in the UI face, which has no italic of its own.
    pub slant: bool,
    /// Set in the icon font (a task's box, a missing link's warning).
    pub icon: bool,
}

impl Look {
    fn plain(role: Type, color: Hsla) -> Self {
        Self {
            role,
            color,
            underline: false,
            struck: false,
            fill: None,
            slant: false,
            icon: false,
        }
    }
}

/// Styled words: their text and the look of each stretch, in order and covering all of it.
#[derive(Clone, Debug, Default)]
pub(crate) struct Words {
    pub text: String,
    pub looks: Vec<(Range<usize>, Look)>,
    /// The tallest line any stretch asked for.
    pub line: Pixels,
}

impl Words {
    fn append(&mut self, words: &str, look: Look, line: Pixels) {
        if words.is_empty() {
            return;
        }
        let start = self.text.len();
        self.text.push_str(words);
        self.looks.push((start..self.text.len(), look));
        self.line = self.line.max(line);
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// The words cut at byte `cut` (a stretch boundary) into the head and the rest.
    fn split(self, cut: usize) -> (Self, Self) {
        let part = |range: Range<usize>| Words {
            text: self.text[range.clone()].to_owned(),
            looks: self
                .looks
                .iter()
                .filter(|(stretch, _)| stretch.start < range.end && stretch.end > range.start)
                .map(|(stretch, look)| {
                    (
                        stretch.start.max(range.start) - range.start
                            ..stretch.end.min(range.end) - range.start,
                        *look,
                    )
                })
                .collect(),
            line: self.line,
        };
        (part(0..cut), part(cut..self.text.len()))
    }
}

/// One paragraph: a list item's words, a heading, a code block or prose.
#[derive(Clone, Debug)]
pub(crate) struct Paragraph {
    pub words: Words,
    /// The links in the words, by their bytes.
    pub links: Vec<(Range<usize>, LinkTarget)>,
    /// A list item's mark (a bullet, a number, a task's box) in its own column, or in a plan
    /// row's words a missing link's icon.
    pub lead: Option<Words>,
    /// A task's box: whether it is done.
    pub task: Option<bool>,
    /// How many lists it is in. Its words keep their item's edge.
    pub depth: usize,
    /// The space after it.
    pub after: Pixels,
    /// The type size of its words.
    pub size: Pixels,
}

/// A table's cells, its shown columns, and its roll marker.
#[derive(Clone, Debug)]
pub(crate) struct Table {
    pub rows: Vec<Vec<String>>,
    pub columns: Vec<usize>,
    /// An inventory table, shown by its item, quantity, equipment and attunement.
    pub compact: bool,
    pub rolls: Option<Result<crate::roll_table::Marker, String>>,
}

impl Table {
    /// A cell's words as the table shows them.
    pub(crate) fn display<'a>(&self, cell: &'a str) -> &'a str {
        if !self.compact {
            return cell;
        }
        match cell {
            "Link" => "Item",
            "true" => "Yes",
            "false" => "—",
            other => other,
        }
    }

    /// The cell at `row` in shown column `position`.
    pub(crate) fn cell(&self, row: usize, position: usize) -> &str {
        let column = self.columns[position];
        self.display(
            self.rows[row]
                .get(column)
                .map(String::as_str)
                .unwrap_or_default(),
        )
    }
}

/// One block of a document.
#[derive(Clone, Debug)]
pub(crate) enum Block {
    Gap(Pixels),
    Rule,
    /// A heading's words: a typed jump to it lands at the next block.
    Heading(String),
    /// Images before the words of their block, which are their caption.
    Figures(Vec<PathBuf>, String),
    Paragraph(Box<Paragraph>),
    QuoteStart,
    QuoteEnd,
    Table(Box<Table>),
    /// A compact reader's `Field`/`Value` table, as field rows.
    Fields(Vec<Vec<String>>),
}

/// How a document's words are set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum Mode {
    /// The Reader's prose: `Prose`, or `ProseCompact` in a side panel.
    #[default]
    Reader,
    /// The Reader's prose one step smaller (`ProseCompact`).
    Compact,
    /// Compact field rows in `Stat` type (`show_compact`, `show_skills`).
    Dense,
    /// A plan row's words (`show_row_words`): `Body` on `XL` lines; `muted` for a done scene.
    Row { muted: bool },
}

/// What the preparation reads besides the Markdown.
pub(crate) struct Inputs<'a> {
    pub mode: Mode,
    /// The link index: notes' links resolve in it, and bold words that name one entry link to
    /// Lookup.
    pub index: Option<&'a crate::lookup::ContentIndex>,
    /// The document shown, for its links' resolution.
    pub from: Option<uuid::Uuid>,
    /// The local copy of an image a link names, which shows in place.
    pub art: &'a dyn Fn(&str) -> Option<PathBuf>,
}

/// What the words waiting to become a paragraph link to.
#[derive(Default)]
struct Targets {
    links: Vec<(Range<usize>, LinkTarget)>,
    task: Option<bool>,
    row: bool,
    /// The bytes of the mark that starts the words, in its own column.
    lead: Option<usize>,
    depth: usize,
}

/// The words waiting to be drawn become a paragraph block.
fn paragraph(blocks: &mut Vec<Block>, words: &mut Words, target: &mut Targets, size: Pixels) {
    if words.is_empty() {
        return;
    }
    let mut links = std::mem::take(&mut target.links);
    let task = target.task.take();
    let mut words = std::mem::take(words);
    let lead = target.lead.take().map(|cut| {
        let (mut mark, rest) = std::mem::take(&mut words).split(cut);
        for (range, _) in &mut links {
            *range = range.start.saturating_sub(cut)..range.end.saturating_sub(cut);
        }
        words = rest;
        if !links.is_empty() {
            // A link lifts its paragraph's lines to `XL`: the mark's line with them.
            mark.line = mark.line.max(XL);
        }
        mark
    });
    if !links.is_empty() {
        // A line with a link is a full target tall.
        words.line = words.line.max(XL);
    }
    blocks.push(Block::Paragraph(Box::new(Paragraph {
        words,
        links,
        lead,
        task,
        depth: target.depth,
        after: S,
        size,
    })));
}

/// The space after the block before, when that block is a list's words.
fn gap_after_listed(blocks: &mut [Block], gap: Pixels) {
    if let Some(Block::Paragraph(paragraph)) = blocks.last_mut()
        && paragraph.depth > 0
    {
        paragraph.after = gap;
    }
}

/// The images found in a block, before its words.
fn figures_block(blocks: &mut Vec<Block>, figures: &mut Vec<PathBuf>, caption: &Words) {
    if !figures.is_empty() {
        blocks.push(Block::Figures(
            std::mem::take(figures),
            caption.text.clone(),
        ));
    }
}

/// Notes write a captioned image as `*caption* ([image](url))`. Where the image will show in
/// place, the brackets around the link have nothing left to enclose.
pub(crate) fn local_figure_links(markdown: &str, local: impl Fn(&str) -> bool) -> Cow<'_, str> {
    const OPEN: &str = "([image](";
    if !markdown.contains(OPEN) {
        return Cow::Borrowed(markdown);
    }
    let mut output = String::with_capacity(markdown.len());
    let mut rest = markdown;
    while let Some(start) = rest.find(OPEN) {
        let after = &rest[start + OPEN.len()..];
        match after.find("))") {
            Some(end) if local(&after[..end]) => {
                output.push_str(&rest[..start]);
                output.push_str("[image](");
                output.push_str(&after[..=end]);
                rest = &after[end + 2..];
            }
            _ => {
                output.push_str(&rest[..start + OPEN.len()]);
                rest = after;
            }
        }
    }
    output.push_str(rest);
    Cow::Owned(output)
}

/// The blocks of `markdown` as the reader draws them.
pub(crate) fn prepare(markdown: &str, inputs: &Inputs<'_>) -> Vec<Block> {
    let c = colors();
    let dense = inputs.mode == Mode::Dense;
    let row = match inputs.mode {
        Mode::Row { muted } => Some(muted),
        _ => None,
    };
    let prose = match inputs.mode {
        Mode::Dense => Type::Stat,
        Mode::Row { .. } => Type::Body,
        Mode::Compact => Type::ProseCompact,
        Mode::Reader => Type::Prose,
    };
    let line = if row.is_some() { XL } else { prose.line() };
    // A done scene's words and links are muted; a link keeps its underline.
    let (text_color, link_color) = if row == Some(true) {
        (c.muted, c.muted)
    } else {
        (c.text, c.accent)
    };
    let mut blocks = Vec::new();
    let mut quote_depth = 0_usize;
    let mut words = Words::default();
    let mut heading: Option<HeadingLevel> = None;
    let mut inventory_table = false;
    let mut target = Targets {
        row: row.is_some(),
        ..Targets::default()
    };
    let mut link: Option<(usize, LinkTarget)> = None;
    let mut strong_link: Option<usize> = None;
    let mut rolls = None;
    let mut in_link = false;
    let mut code = false;
    let mut strong = 0_usize;
    let mut italic = false;
    let mut struck = false;
    let mut lists: Vec<Option<u64>> = Vec::new();
    // The Reader sets an item in by its level, its mark in its own column. A plan row's words
    // are one line of its row: a mark stays in that line, after two spaces for each level.
    let in_row = row.is_some();
    let depth = |lists: usize| if in_row { 0 } else { lists };
    let row_indent = |lists: usize| {
        if in_row {
            "  ".repeat(lists.saturating_sub(1))
        } else {
            String::new()
        }
    };
    let mut table: Option<Vec<Vec<String>>> = None;
    let mut figures: Vec<PathBuf> = Vec::new();
    let mut in_figure = false;
    // The size of the words of the paragraph being read: a heading's own, else the prose's.
    let mut size = prose.size();
    let linked: Cow<'_, str> = inputs.index.map_or(Cow::Borrowed(markdown), |index| {
        Cow::Owned(index.reader_markdown(markdown, inputs.from))
    });
    let source = local_figure_links(&linked, |url| (inputs.art)(url).is_some());
    let mark_look = Look::plain(prose, c.muted);
    for (event, range) in Parser::new_ext(
        &source,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS,
    )
    .into_offset_iter()
    {
        // A roll marker belongs to the table right after it, not to a later one.
        if matches!(
            event,
            Event::Start(
                Tag::Paragraph
                    | Tag::Heading { .. }
                    | Tag::List(_)
                    | Tag::CodeBlock(_)
                    | Tag::BlockQuote(_)
            )
        ) {
            rolls = None;
        }
        if matches!(event, Event::Code(_) | Event::Start(Tag::Image { .. })) {
            strong_link = None;
        }
        match event {
            Event::Html(ref text) if text.trim().starts_with("<!-- splat:inventory") => {
                inventory_table = true;
            }
            Event::Html(ref text) if crate::roll_table::rolls_marker(text).is_some() => {
                rolls = crate::roll_table::rolls_marker(text);
            }
            Event::Text(ref text) if in_figure && &**text == "image" => {}
            Event::Start(Tag::Link { dest_url, .. }) if table.is_none() => {
                // An explicit destination wins over the bold words around it.
                strong_link = None;
                in_link = true;
                // A link to an image with a local copy shows the image instead of opening a page.
                let local = (inputs.art)(&dest_url)
                    .or_else(|| dest_url.strip_prefix(FILE_LINK).map(Into::into));
                if let Some(path) = local {
                    figures.push(path);
                    in_figure = true;
                    continue;
                }
                if dest_url.starts_with("splat-missing:") {
                    let icon = Look {
                        icon: true,
                        ..Look::plain(prose, c.result.warning)
                    };
                    words.append(icons::WARN, icon, line);
                    words.append(" ", Look::plain(prose, c.result.warning), line);
                    if target.row && words.text.len() == icons::WARN.len() + 1 {
                        target.lead = Some(words.text.len());
                    }
                    link = Some((words.text.len(), LinkTarget::Missing));
                    continue;
                }
                if let Some(destination) = LinkTarget::from_destination(&dest_url) {
                    link = Some((words.text.len(), destination));
                }
            }
            Event::End(TagEnd::Link) => {
                in_link = false;
                in_figure = false;
                if let Some((start, destination)) = link.take() {
                    target.links.push((start..words.text.len(), destination));
                }
            }
            Event::Start(Tag::Table(_)) => {
                paragraph(&mut blocks, &mut words, &mut target, size);
                table = Some(Vec::new());
            }
            Event::Start(Tag::TableHead | Tag::TableRow) => {
                if let Some(rows) = &mut table {
                    rows.push(Vec::new());
                }
            }
            Event::Start(Tag::TableCell) => {
                if let Some(row) = table.as_mut().and_then(|rows| rows.last_mut()) {
                    row.push(String::new());
                }
            }
            Event::End(TagEnd::Table) => {
                if let Some(mut rows) = table.take() {
                    if dense && rows.first().is_some_and(|row| row == &["Field", "Value"]) {
                        rows.remove(0);
                        blocks.push(Block::Fields(rows));
                        inventory_table = false;
                        continue;
                    }
                    let compact = inventory_table
                        && rows
                            .first()
                            .is_some_and(|row| row.iter().any(|cell| cell == "Link"))
                        && rows.iter().skip(1).all(|row| {
                            rows[0]
                                .iter()
                                .position(|cell| cell == "Link")
                                .and_then(|i| row.get(i))
                                .is_some_and(|name| !name.is_empty())
                        });
                    let columns: Vec<_> = if compact {
                        ["Link", "Quantity", "Equipment", "Attuned"]
                            .iter()
                            .filter_map(|name| rows.first()?.iter().position(|cell| cell == name))
                            .collect()
                    } else {
                        (0..rows.first().map_or(0, Vec::len)).collect()
                    };
                    if let Some(Ok(crate::roll_table::Marker { dice, .. })) = &rolls
                        && let Some(header) = rows.first_mut().and_then(|row| row.first_mut())
                        && header.trim().is_empty()
                    {
                        *header = dice
                            .strip_prefix('1')
                            .filter(|value| value.starts_with('d'))
                            .unwrap_or(dice)
                            .to_owned();
                    }
                    blocks.push(Block::Table(Box::new(Table {
                        rows,
                        columns,
                        compact,
                        rolls: rolls.take(),
                    })));
                }
                inventory_table = false;
            }
            Event::Text(text) | Event::Code(text) if table.is_some() => {
                if let Some(cell) = table
                    .as_mut()
                    .and_then(|rows| rows.last_mut())
                    .and_then(|row| row.last_mut())
                {
                    cell.push_str(&text);
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                paragraph(&mut blocks, &mut words, &mut target, size);
                blocks.push(Block::Gap(if dense {
                    S
                } else {
                    match level {
                        HeadingLevel::H1 => S,
                        HeadingLevel::H2 => L,
                        _ => M,
                    }
                }));
                heading = Some(level);
                size = heading_type(level, dense).size();
            }
            Event::End(TagEnd::Heading(level)) => {
                // A linked section opens with its heading at the column's top.
                blocks.push(Block::Heading(words.text.clone()));
                figures_block(&mut blocks, &mut figures, &words);
                paragraph(&mut blocks, &mut words, &mut target, size);
                if level == HeadingLevel::H2 || (!dense && level == HeadingLevel::H3) {
                    blocks.push(Block::Rule);
                    blocks.push(Block::Gap(XS));
                }
                heading = None;
                size = prose.size();
            }
            Event::Start(Tag::CodeBlock(_)) => {
                paragraph(&mut blocks, &mut words, &mut target, size);
                code = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                paragraph(&mut blocks, &mut words, &mut target, Type::Mono.size());
                code = false;
            }
            Event::Start(Tag::Strong) => {
                strong += 1;
                if strong == 1 {
                    strong_link = (!in_link && heading.is_none() && table.is_none())
                        .then_some(words.text.len());
                }
            }
            Event::End(TagEnd::Strong) => {
                strong = strong.saturating_sub(1);
                if strong == 0
                    && let Some(start) = strong_link.take()
                {
                    let named = words.text[start..].trim();
                    let table_caption = words.text[..start].trim().is_empty()
                        && source[range.end..].trim_start().starts_with('|');
                    if !(named.is_empty() || named.ends_with(['.', ':']) || table_caption)
                        && inputs.index.is_some_and(|index| {
                            matches!(
                                index.resolve_link(named, inputs.from),
                                crate::world::Go::Item(..) | crate::world::Go::Many(_)
                            )
                        })
                    {
                        let named = named.to_owned();
                        for (stretch, look) in &mut words.looks {
                            if stretch.start >= start {
                                look.color = link_color;
                                look.underline = true;
                            }
                        }
                        target
                            .links
                            .push((start..words.text.len(), LinkTarget::Lookup(named)));
                    }
                }
            }
            Event::Start(Tag::Emphasis) => italic = true,
            Event::End(TagEnd::Emphasis) => italic = false,
            Event::Start(Tag::Strikethrough) => struck = true,
            Event::End(TagEnd::Strikethrough) => struck = false,
            Event::Start(Tag::BlockQuote(_)) => {
                paragraph(&mut blocks, &mut words, &mut target, size);
                blocks.push(Block::QuoteStart);
                quote_depth += 1;
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                figures_block(&mut blocks, &mut figures, &words);
                paragraph(&mut blocks, &mut words, &mut target, size);
                blocks.push(Block::QuoteEnd);
                quote_depth = quote_depth.saturating_sub(1);
            }
            Event::Start(Tag::List(start)) => {
                // An item's own words end before its inner list: they keep their own level.
                paragraph(&mut blocks, &mut words, &mut target, size);
                lists.push(start);
                target.depth = depth(lists.len());
            }
            Event::End(TagEnd::List(_)) => {
                lists.pop();
                target.depth = depth(lists.len());
                if lists.is_empty() && !dense {
                    gap_after_listed(&mut blocks, M);
                }
            }
            Event::Start(Tag::Item) => {
                paragraph(&mut blocks, &mut words, &mut target, size);
                if lists.len() > 1 {
                    gap_after_listed(&mut blocks, XS);
                }
                let indent = row_indent(lists.len());
                let marker = match lists.last_mut() {
                    Some(Some(number)) => {
                        let marker = format!("{number}. ");
                        *number += 1;
                        marker
                    }
                    _ => "• ".into(),
                };
                words.append(&format!("{indent}{marker}"), mark_look, line);
                target.lead = (target.depth > 0).then_some(words.text.len());
            }
            Event::TaskListMarker(done) => {
                // The marker follows Item's indentation and bullet, before any prose or links.
                let indent = row_indent(lists.len());
                // A bulleted item trades its bullet for the box; a numbered item keeps its
                // number and the box follows it.
                let bulleted = matches!(lists.last(), Some(None));
                let lead = if bulleted {
                    indent.clone()
                } else {
                    std::mem::take(&mut words.text)
                };
                words = Words::default();
                words.append(&lead, mark_look, line);
                let icon = if done {
                    icons::phosphor::CHECK_SQUARE
                } else {
                    icons::phosphor::SQUARE
                };
                words.append(
                    icon,
                    Look {
                        icon: true,
                        ..mark_look
                    },
                    line,
                );
                words.append(" ", mark_look, line);
                target.lead = (target.depth > 0).then_some(words.text.len());
                target.task = Some(done);
            }
            Event::End(TagEnd::Paragraph | TagEnd::Item) => {
                figures_block(&mut blocks, &mut figures, &words);
                paragraph(&mut blocks, &mut words, &mut target, size);
            }
            Event::HardBreak if table.is_none() => {
                words.append("\n", Look::plain(prose, text_color), line);
            }
            Event::SoftBreak if table.is_none() => {
                words.append(
                    if code { "\n" } else { " " },
                    Look::plain(if code { Type::Mono } else { prose }, text_color),
                    if code { Type::Mono.line() } else { line },
                );
            }
            Event::Code(text) if !dense && heading.is_none() => words.append(
                &text,
                Look {
                    fill: Some(c.hover),
                    ..Look::plain(Type::Mono, text_color)
                },
                // Mono's own line puts code on the baseline of the serif beside it.
                Type::Mono.line(),
            ),
            Event::Text(text) | Event::Code(text) => {
                let role = if code {
                    Type::Mono
                } else if let Some(level) = heading {
                    heading_type(level, dense)
                } else if strong > 0 {
                    if dense {
                        Type::StatLabel
                    } else if row.is_some() {
                        Type::Strong
                    } else {
                        Type::ProseStrong
                    }
                } else if (italic || quote_depth > 0) && !dense && row.is_none() {
                    Type::ProseItalic
                } else {
                    prose
                };
                let missing = matches!(link, Some((_, LinkTarget::Missing)));
                let color = if missing {
                    c.muted
                } else if link.is_some() {
                    link_color
                } else {
                    text_color
                };
                words.append(
                    &text,
                    Look {
                        underline: link.is_some() && !missing,
                        struck,
                        // The UI face has no italic of its own: a row's emphasis slants.
                        slant: row.is_some() && italic,
                        ..Look::plain(role, color)
                    },
                    if heading.is_some() || code {
                        role.line()
                    } else {
                        line
                    },
                );
            }
            Event::Rule => {
                paragraph(&mut blocks, &mut words, &mut target, size);
                blocks.push(Block::Rule);
            }
            _ => {}
        }
    }
    figures_block(&mut blocks, &mut figures, &words);
    paragraph(&mut blocks, &mut words, &mut target, size);
    blocks
}

/// A heading's type: in field rows `StatLabel`; else `Title`, `Section`, or `ProseStrong`.
fn heading_type(level: HeadingLevel, dense: bool) -> Type {
    match level {
        _ if dense => Type::StatLabel,
        HeadingLevel::H1 => Type::Title,
        HeadingLevel::H2 | HeadingLevel::H3 => Type::Section,
        _ => Type::ProseStrong,
    }
}

/// A field row's value: a list of words joined, a yes or no in words.
pub(crate) fn display_field_value(value: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(value) {
        Ok(serde_json::Value::Array(values)) if values.iter().all(|v| v.is_string()) => values
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        Ok(serde_json::Value::Bool(value)) => if value { "Yes" } else { "No" }.into(),
        _ => value.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(source: &str, mode: Mode) -> Vec<Block> {
        prepare(
            source,
            &Inputs {
                mode,
                index: None,
                from: None,
                art: &|_| None,
            },
        )
    }

    fn kinds(blocks: &[Block]) -> Vec<String> {
        blocks
            .iter()
            .map(|block| match block {
                Block::Gap(_) => "gap".to_owned(),
                Block::Rule => "rule".to_owned(),
                Block::Heading(words) => format!("heading {words}"),
                Block::Figures(..) => "figures".to_owned(),
                Block::Paragraph(paragraph) => format!(
                    "{}{}",
                    paragraph
                        .lead
                        .as_ref()
                        .map_or(String::new(), |lead| format!("[{}]", lead.text)),
                    paragraph.words.text
                ),
                Block::QuoteStart => "quote".to_owned(),
                Block::QuoteEnd => "end quote".to_owned(),
                Block::Table(_) => "table".to_owned(),
                Block::Fields(_) => "fields".to_owned(),
            })
            .collect()
    }

    #[test]
    fn blocks_follow_the_egui_reader() {
        let source = "# Title\n\nSome **strong** and *soft* words with [a link](https://a.example).\n\n## Part\n\n- one\n- two\n  1. inner\n\n> quoted\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n```\ncode\n```\n";
        let blocks = read(source, Mode::Reader);
        assert_eq!(
            kinds(&blocks),
            [
                "gap",
                "heading Title",
                "Title",
                "Some strong and soft words with a link.",
                "gap",
                "heading Part",
                "Part",
                "rule",
                "gap",
                "[• ]one",
                "[• ]two",
                "[1. ]inner",
                "quote",
                "quoted",
                "end quote",
                "table",
                "code",
            ]
        );
        let Block::Paragraph(prose) = &blocks[3] else {
            unreachable!()
        };
        assert_eq!(
            prose.links,
            [(32..38, LinkTarget::Url("https://a.example".into()))]
        );
        assert!(prose.words.looks.iter().any(|(range, look)| {
            &prose.words.text[range.clone()] == "strong" && look.role == Type::ProseStrong
        }));
        assert_eq!(prose.words.line, XL, "a linked line is a full target tall");
        // An inner list's item follows `XS` after the item before it; a whole list, `M`.
        let Block::Paragraph(two) = &blocks[10] else {
            unreachable!()
        };
        let Block::Paragraph(inner) = &blocks[11] else {
            unreachable!()
        };
        assert_eq!((two.depth, two.after), (1, XS));
        assert_eq!((inner.depth, inner.after), (2, M));
    }

    #[test]
    fn task_boxes_stand_in_the_mark_column() {
        let blocks = read("- [x] Done\n1. [ ] Open\n", Mode::Reader);
        let leads: Vec<_> = blocks
            .iter()
            .filter_map(|block| match block {
                Block::Paragraph(paragraph) => {
                    Some((paragraph.lead.clone()?.text, paragraph.task, paragraph.words.text.clone()))
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            leads,
            [
                (format!("{} ", icons::phosphor::CHECK_SQUARE), Some(true), "Done".to_owned()),
                (format!("1. {} ", icons::phosphor::SQUARE), Some(false), "Open".to_owned()),
            ]
        );
    }

    #[test]
    fn dense_field_tables_become_field_rows_and_rows_keep_marks_in_line() {
        let blocks = read("| Field | Value |\n|---|---|\n| hit_points | 22 |\n", Mode::Dense);
        assert!(matches!(&blocks[..], [Block::Fields(rows)] if rows[0] == ["hit_points", "22"]));
        let blocks = read("- one\n  - inner\n", Mode::Row { muted: false });
        assert_eq!(kinds(&blocks), ["• one", "  • inner"]);
        assert_eq!(display_field_value(r#"["a","b"]"#), "a, b");
        assert_eq!(display_field_value("true"), "Yes");
    }

    #[test]
    fn dice_and_missing_links_keep_their_targets() {
        let link = crate::content::dice_text::RollLink::d20(
            crate::content::dice_text::D20::Check,
            6,
            "Strength",
        );
        let source = format!(
            "Roll {} or see [Gone](splat-missing:Gone).",
            link.markdown("+6")
        );
        let blocks = read(&source, Mode::Reader);
        let Block::Paragraph(paragraph) = &blocks[0] else {
            unreachable!()
        };
        let targets: Vec<_> = paragraph
            .links
            .iter()
            .map(|(range, target)| (&paragraph.words.text[range.clone()], target.clone()))
            .collect();
        assert_eq!(
            targets,
            [("+6", LinkTarget::Roll(link)), ("Gone", LinkTarget::Missing)]
        );
        assert!(paragraph.words.text.contains(icons::WARN), "a missing link warns");
    }
}
