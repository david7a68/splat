//! Markdown source styled where it is typed, for `TextEditor::markdown_source`. Only the look changes: every
//! character stays, markup included, so the file holds exactly what was typed.
//!
//! The looks come from the source parser (`markdown_source::source_looks`): strong words and minor
//! headings bold, italics and block quotes italic, inline code on its fill, struck words struck,
//! markup (`##`, `**`, a link's target, comments) faint, link words in the accent, and a rule under
//! each section heading.
//!
//! A `splat:` mark that Splat wrote (a travel check) folds to a faint `⋯` while the caret and the
//! selection are off its line, as egui's did (closure FE9): the mark is one inline token of the
//! editor, so the caret steps over it as one unit and its letters stay in the text. On the
//! caret's line it shows in full, faint, as all markup is.
//!
//! What GPUI's editor cannot do, and what it does in its place: one editor lays out one font size
//! and one family (a decoration sets only colour, weight, slant, fill and lines), so a title or
//! section heading keeps the prose size and is bold, and code keeps the prose face.
use crate::gui::theme::{Type, colors};
use crate::markdown_view::LinkTarget;
use gpui::{HighlightStyle, StrikethroughStyle, px};
use gpui_component::input::TextDecoration;
use pulldown_cmark::{Event, LinkType, Options, Parser, Tag};
use splat_content_ui::markdown_source::{Role, source_looks};
use std::ops::Range;

/// A source's decorations, the byte where each section heading ends, and the `splat:` marks
/// that fold.
pub(crate) struct SourceStyle {
    pub(crate) decorations: Vec<TextDecoration>,
    pub(crate) rules: Vec<usize>,
    pub(crate) folds: Vec<Range<usize>>,
}

/// The glyph a folded `splat:` mark shows, in the icon face (egui's `FOLD_MARK`).
pub(crate) const FOLD_MARK: &str = crate::gui::icons::EXPAND;

/// The id of a folded mark's inline token.
pub(crate) const FOLD_TOKEN: &str = "splat-mark";

/// The bytes of the lines that hold `selection` in `text`: from the start of the first line to
/// the end of the last, its line break left out. A mark there stays open (egui's `caret_lines`).
pub(crate) fn caret_lines(text: &str, selection: Range<usize>) -> Range<usize> {
    let (first, last) = (
        selection.start.min(text.len()),
        selection.end.min(text.len()),
    );
    let start = text
        .get(..first)
        .and_then(|before| before.rfind('\n'))
        .map_or(0, |at| at + 1);
    let end = text
        .get(last..)
        .and_then(|after| after.find('\n'))
        .map_or(text.len(), |at| last + at);
    start..end
}

/// The marks of `folds` that are shut while `open` (the caret's lines, if the editor has the
/// keyboard) is where it is: every mark off those lines.
pub(crate) fn shut(folds: &[Range<usize>], open: Option<&Range<usize>>) -> Vec<Range<usize>> {
    folds
        .iter()
        .filter(|fold| open.is_none_or(|open| fold.end < open.start || fold.start > open.end))
        .cloned()
        .collect()
}

const OPTIONS: Options = Options::ENABLE_TABLES
    .union(Options::ENABLE_STRIKETHROUGH)
    .union(Options::ENABLE_WIKILINKS);

/// Where each link that can be followed is in a text, markup included, and where it goes
/// (egui's `markdown_source::links`).
pub(crate) fn links(text: &str) -> Vec<(Range<usize>, LinkTarget)> {
    Parser::new_ext(text, OPTIONS)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            let Event::Start(Tag::Link {
                link_type,
                dest_url,
                ..
            }) = event
            else {
                return None;
            };
            let target = if matches!(link_type, LinkType::WikiLink { .. }) {
                // What the link names, read as the Reader reads it.
                let (_, _, note, _) = crate::world::each_link(&text[range.clone()]).pop()?;
                LinkTarget::Note(note)
            } else {
                LinkTarget::from_destination(&dest_url)?
            };
            Some((range, target))
        })
        .collect()
}

/// The link whose characters hold byte `at` of `text`.
pub(crate) fn link_at(text: &str, at: usize) -> Option<LinkTarget> {
    links(text)
        .into_iter()
        .find(|(range, _)| range.contains(&at))
        .map(|(_, target)| target)
}

/// The decorations of `text`: one for each stretch with a look of its own.
pub(crate) fn style(text: &str) -> SourceStyle {
    let c = colors();
    let (runs, rules) = source_looks(text);
    // A mark's folded runs, joined: one token each.
    let mut folds: Vec<Range<usize>> = Vec::new();
    for (range, _) in runs
        .iter()
        .filter(|(range, look)| look.folded && !range.is_empty())
    {
        match folds.last_mut() {
            Some(last) if last.end == range.start => last.end = range.end,
            _ => folds.push(range.clone()),
        }
    }
    let decorations = runs
        .iter()
        .filter(|(range, _)| !range.is_empty())
        .filter_map(|(range, look)| {
            let face = match look.role {
                Role::Prose => None,
                Role::Strong | Role::MinorHeading => Some(Type::ProseStrong),
                Role::Italic => Some(Type::ProseItalic),
                Role::InlineCode | Role::CodeBlock => None,
                Role::Title => Some(Type::Title),
                Role::Section => Some(Type::Section),
            };
            let color = if look.markup || look.folded {
                Some(c.faint)
            } else if look.link {
                Some(c.accent)
            } else {
                None
            };
            let style = HighlightStyle {
                color,
                font_weight: face.map(|face| face.weight()),
                font_style: face
                    .filter(|face| *face == Type::ProseItalic)
                    .map(|face| face.style()),
                background_color: (look.role == Role::InlineCode && !look.markup)
                    .then_some(c.hover),
                strikethrough: (look.struck && !look.markup).then(|| StrikethroughStyle {
                    thickness: px(1.0),
                    color: Some(c.text),
                }),
                ..HighlightStyle::default()
            };
            (style != HighlightStyle::default()).then(|| TextDecoration::new(range.clone(), style))
        })
        .collect();
    SourceStyle {
        decorations,
        rules,
        folds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn look_at(style: &SourceStyle, text: &str, words: &str) -> HighlightStyle {
        let at = text.find(words).expect("the words");
        style
            .decorations
            .iter()
            .find(|decoration| decoration.range.contains(&at))
            .map(|decoration| decoration.style)
            .unwrap_or_default()
    }

    #[test]
    fn source_marks_markup_words_and_section_rules() {
        let c = colors();
        let text = "# Rook\n\n## Tonight\n\nThe **bold** and *slanted* words, `code`, ~~gone~~ and [[Hobb]].\n";
        let style = style(text);
        assert_eq!(
            look_at(&style, text, "## ").color,
            Some(c.faint),
            "markup is faint"
        );
        assert_eq!(
            look_at(&style, text, "bold").font_weight,
            Some(Type::ProseStrong.weight()),
            "strong words are bold"
        );
        assert_eq!(look_at(&style, text, "**").color, Some(c.faint));
        assert_eq!(
            look_at(&style, text, "slanted").font_style,
            Some(Type::ProseItalic.style())
        );
        assert_eq!(
            look_at(&style, text, "code").background_color,
            Some(c.hover)
        );
        assert!(look_at(&style, text, "gone").strikethrough.is_some());
        assert_eq!(
            look_at(&style, text, "Hobb").color,
            Some(c.accent),
            "a link's words"
        );
        assert_eq!(look_at(&style, text, " words,"), HighlightStyle::default());
        // One rule, under `## Tonight` (a title has none, as in the Reader).
        assert_eq!(style.rules, vec![text.find("\n\nThe").unwrap()]);
    }
}
