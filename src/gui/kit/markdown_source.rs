//! Markdown source styled where it is typed (egui `markdown_source::layouter` and
//! `paint_section_rules`), for `TextEditor::markdown_source`. Only the look changes: every
//! character stays, markup included, so the file holds exactly what was typed.
//!
//! The looks come from the egui parser (`markdown_source::source_looks`): strong words and minor
//! headings bold, italics and block quotes italic, inline code on its fill, struck words struck,
//! markup (`##`, `**`, a link's target, comments) faint, link words in the accent, and a rule under
//! each section heading.
//!
//! What GPUI's editor cannot do, and what it does in its place: one editor lays out one font size
//! and one family, so a title or section heading keeps the prose size and is bold (egui sets it
//! in the heading's size and face), and code keeps the prose face (egui sets `Mono`). A
//! `splat:` mark that egui folds to `⋯` while the caret is off its line stays in full, faint, as
//! all markup is: a folded run would change what the caret steps over.
use crate::gui::theme::{Type, colors};
use splat_content_ui::markdown_source::{Role, source_looks};
use gpui::{HighlightStyle, StrikethroughStyle, px};
use gpui_component::input::TextDecoration;

/// A source's decorations and the byte where each section heading ends.
pub(crate) struct SourceStyle {
    pub(crate) decorations: Vec<TextDecoration>,
    pub(crate) rules: Vec<usize>,
}

/// The decorations of `text`: one for each stretch with a look of its own.
pub(crate) fn style(text: &str) -> SourceStyle {
    let c = colors();
    let (runs, rules) = source_looks(text);
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
    SourceStyle { decorations, rules }
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
        assert_eq!(look_at(&style, text, "## ").color, Some(c.faint), "markup is faint");
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
        assert_eq!(look_at(&style, text, "code").background_color, Some(c.hover));
        assert!(look_at(&style, text, "gone").strikethrough.is_some());
        assert_eq!(look_at(&style, text, "Hobb").color, Some(c.accent), "a link's words");
        assert_eq!(look_at(&style, text, " words,"), HighlightStyle::default());
        // One rule, under `## Tonight` (a title has none, as in the Reader).
        assert_eq!(style.rules, vec![text.find("\n\nThe").unwrap()]);
    }
}
