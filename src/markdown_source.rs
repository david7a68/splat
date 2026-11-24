//! Markdown source, styled where it is typed, in the Reader's faces. Only the look changes:
//! every character stays, markup included, so the file holds exactly what was typed.
use eframe::egui::{self};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};
use std::{ops::Range, sync::Arc};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Role {
    #[default]
    Prose,
    Strong,
    Italic,
    /// Code among words, on its own fill. A block's rows would make an uneven patch.
    InlineCode,
    CodeBlock,
    Title,
    Section,
    MinorHeading,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Look {
    pub role: Role,
    /// Syntax and not words, such as `##`, `**` or the target of a link.
    pub markup: bool,
    pub link: bool,
    pub struck: bool,
    /// Part of a `<!-- splat:… -->` mark that Splat wrote (a travel check). It folds to one muted
    /// mark unless the caret is on its line (closure FE9).
    pub folded: bool,
}

pub type Runs = Arc<Vec<(Range<usize>, Look)>>;

const OPTIONS: Options = Options::ENABLE_TABLES
    .union(Options::ENABLE_STRIKETHROUGH)
    .union(Options::ENABLE_WIKILINKS);

/// The look of each stretch of a text. Together they cover all of it.
fn looks(text: &str) -> Runs {
    let mut bytes = vec![Look::default(); text.len()];
    for (event, range) in Parser::new_ext(text, OPTIONS).into_offset_iter() {
        // A block's own line break stays plain, or the empty line after it takes its height.
        let end = range.start + text[range.clone()].trim_end_matches(['\n', '\r']).len();
        let range = range.start..end;
        // Words clear what their container set, so what stays marked is the syntax around them.
        let mark = |bytes: &mut [Look], change: &dyn Fn(&mut Look)| {
            bytes[range.clone()].iter_mut().for_each(change);
        };
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let role = match level {
                    HeadingLevel::H1 => Role::Title,
                    HeadingLevel::H2 | HeadingLevel::H3 => Role::Section,
                    _ => Role::MinorHeading,
                };
                mark(&mut bytes, &|look| {
                    look.role = role;
                    look.markup = true;
                });
            }
            Event::Start(Tag::Emphasis) => mark(&mut bytes, &|look| {
                if look.role == Role::Prose {
                    look.role = Role::Italic;
                }
                look.markup = true;
            }),
            // Bold wins over italic, as in the Reader.
            Event::Start(Tag::Strong) => mark(&mut bytes, &|look| {
                if matches!(look.role, Role::Prose | Role::Italic) {
                    look.role = Role::Strong;
                }
                look.markup = true;
            }),
            Event::Start(Tag::Strikethrough) => mark(&mut bytes, &|look| {
                look.struck = true;
                look.markup = true;
            }),
            Event::Start(Tag::Link { .. } | Tag::Image { .. }) => mark(&mut bytes, &|look| {
                look.link = true;
                look.markup = true;
            }),
            Event::Start(Tag::CodeBlock(_)) => mark(&mut bytes, &|look| {
                look.role = Role::CodeBlock;
                look.markup = true;
            }),
            Event::Start(Tag::Table(_)) | Event::Html(_) | Event::InlineHtml(_) | Event::Rule => {
                mark(&mut bytes, &|look| look.markup = true);
            }
            Event::Start(Tag::TableHead) => mark(&mut bytes, &|look| look.role = Role::Strong),
            Event::Start(Tag::Item) => {
                let marker = text[range.clone()]
                    .find(char::is_whitespace)
                    .unwrap_or(range.len());
                bytes[range.start..range.start + marker]
                    .iter_mut()
                    .for_each(|look| look.markup = true);
            }
            Event::Start(Tag::BlockQuote(_)) => {
                mark(&mut bytes, &|look| {
                    if look.role == Role::Prose {
                        look.role = Role::Italic;
                    }
                });
                let mut at = range.start;
                for line in text[range.clone()].split_inclusive('\n') {
                    let prefix = line.len() - line.trim_start_matches([' ', '>']).len();
                    for (offset, byte) in line.bytes().take(prefix).enumerate() {
                        if byte == b'>' {
                            bytes[at + offset].markup = true;
                        }
                    }
                    at += line.len();
                }
            }
            Event::Code(_) => {
                mark(&mut bytes, &|look| {
                    look.role = Role::InlineCode;
                    look.markup = true;
                });
                let ticks = text[range.clone()]
                    .bytes()
                    .take_while(|b| *b == b'`')
                    .count();
                bytes[range.start + ticks..range.end - ticks]
                    .iter_mut()
                    .for_each(|look| look.markup = false);
            }
            Event::Text(_) => mark(&mut bytes, &|look| look.markup = false),
            _ => {}
        }
    }
    let mut rest = 0;
    while let Some(start) = text[rest..].find("{{") {
        let start = rest + start;
        let Some(end) = text[start..].find("}}") else {
            break;
        };
        rest = start + end + 2;
        bytes[start..rest]
            .iter_mut()
            .for_each(|look| look.markup = true);
    }
    for fold in splat_marks(text) {
        bytes[fold].iter_mut().for_each(|look| look.folded = true);
    }
    let mut runs = Vec::new();
    let mut start = 0;
    for at in 1..=bytes.len() {
        if at == bytes.len() || bytes[at] != bytes[start] {
            runs.push((start..at, bytes[start]));
            start = at;
        }
    }
    // An empty note still has a font, or the caret has no height.
    if runs.is_empty() {
        runs.push((0..0, Look::default()));
    }
    Arc::new(runs)
}

/// A text's looks for an editor that is not egui's (the GPUI note editor, `gui::kit::field`): each
/// stretch's look, as [`layouter`] styles it, and the byte where each section heading ends (its
/// rule goes under that line, as [`paint_section_rules`] draws it). Toolkit-free.
pub fn source_looks(text: &str) -> (Runs, Vec<usize>) {
    let runs = looks(text);
    let section_ends = runs
        .iter()
        .enumerate()
        .filter(|(index, (_, look))| {
            look.role == Role::Section
                && runs
                    .get(index + 1)
                    .is_none_or(|(_, next)| next.role != Role::Section)
        })
        .map(|(_, (range, _))| range.end)
        .collect();
    (runs, section_ends)
}

/// The marks Splat writes itself, and only those, fold (closure FE9; P2 of the PR 331 review): a
/// closed list, each kind by its command word and the exact shape its writer gives. A mark the DM
/// types (`splat:rolls`), an unknown command and a malformed mark stay in view.
/// A generated mark's command word, and whether its terms have the shape its writer gives.
type MarkKind = (&'static str, fn(&[&str]) -> bool);

const GENERATED_MARKS: &[MarkKind] = &[(
    // `timeline::checks::RoadMark::comment`: `journey=<id> at=<offset> sum=<hex>`.
    "check",
    |terms| match terms {
        [journey, at, sum] => {
            journey
                .strip_prefix("journey=")
                .is_some_and(|id| !id.is_empty())
                && at
                    .strip_prefix("at=")
                    .is_some_and(|at| !at.is_empty() && at.bytes().all(|b| b.is_ascii_digit()))
                && sum.strip_prefix("sum=").is_some_and(|sum| {
                    !sum.is_empty() && sum.bytes().all(|b| b.is_ascii_hexdigit())
                })
        }
        _ => false,
    },
)];

/// Where each mark that Splat generated is, in bytes, in order. One pass over the text.
fn splat_marks(text: &str) -> Vec<Range<usize>> {
    const OPEN: &str = "<!-- splat:";
    let mut marks = Vec::new();
    let mut rest = 0;
    while let Some(start) = text[rest..].find(OPEN) {
        let start = rest + start;
        let line_end = text[start..]
            .find('\n')
            .map_or(text.len(), |end| start + end);
        let Some(end) = text[start..line_end].find("-->") else {
            rest = line_end;
            continue;
        };
        rest = start + end + 3;
        let inner = &text[start + OPEN.len()..start + end];
        let mut words = inner.split_whitespace();
        let command = words.next().unwrap_or_default();
        let terms: Vec<&str> = words.collect();
        if GENERATED_MARKS
            .iter()
            .any(|(word, shape)| *word == command && shape(&terms))
        {
            marks.push(start..rest);
        }
    }
    marks
}

mod sealed {
    /// Only this file implements [`super::Laid`].
    pub trait Sealed {}
    impl Sealed for eframe::egui::text_edit::TextEditOutput {}
}

/// An editor's laid-out text: egui's `TextEditOutput` (a staged raw editor) or the field part's
/// [`Editor`](design::Editor). Sealed: no caller adds a third with other offsets.
pub trait Laid: sealed::Sealed {
    fn response(&self) -> &egui::Response;
    fn galley(&self) -> &Arc<egui::Galley>;
    fn galley_pos(&self) -> egui::Pos2;
    fn text_clip_rect(&self) -> egui::Rect;
}

impl Laid for egui::text_edit::TextEditOutput {
    fn response(&self) -> &egui::Response {
        &self.response
    }
    fn galley(&self) -> &Arc<egui::Galley> {
        &self.galley
    }
    fn galley_pos(&self) -> egui::Pos2 {
        self.galley_pos
    }
    fn text_clip_rect(&self) -> egui::Rect {
        self.text_clip_rect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each stretch's words and look.
    fn styled(text: &str) -> Vec<(&str, Look)> {
        looks(text)
            .iter()
            .map(|(range, look)| (&text[range.clone()], *look))
            .collect()
    }

    /// The words a reader would see, without the syntax around them.
    fn words(text: &str) -> Vec<&str> {
        styled(text)
            .into_iter()
            .filter(|(words, look)| !look.markup && !words.trim().is_empty())
            .map(|(words, _)| words.trim())
            .collect()
    }

    fn look_of(text: &str, part: &str) -> Look {
        styled(text)
            .into_iter()
            .find(|(words, _)| *words == part)
            .unwrap_or_else(|| panic!("no stretch {part:?} in {:?}", styled(text)))
            .1
    }

    #[test]
    fn every_character_stays_in_order_whatever_the_markup() {
        for text in [
            "",
            "\n",
            "plain",
            "# Head\n\nbody\n",
            "**unfinished *markup `here\n\n```\nunclosed fence",
            "Ünïcödé *ïtalic* — **bøld** [[Tïmeline#^t-1|wörds]] `cödé` ⅔\n",
            "- \n- \n- a\n\n1. one\n   - nested\n",
            "> quote\n> > deeper\n>\n",
            "| a | b |\n|---|---|\n| 1 | *2* |\n",
            "line\r\n\r\n## Head\r\n\r\n- item\r\n",
            "<!-- comment -->\n\n---\n\n![alt](a.png) <b>x</b> ~~old~~\n",
            "***both*** [a](http://b) <https://c> [[Plain]]",
        ] {
            let runs = looks(text);
            let mut at = 0;
            for (range, _) in runs.iter() {
                assert_eq!(range.start, at, "{text:?}");
                at = range.end;
            }
            assert_eq!(at, text.len(), "{text:?}");
        }
    }

    #[test]
    fn a_heading_takes_the_readers_face_and_its_marks_are_quiet() {
        let text = "# Session 4\n\n## What happened\n\n#### Later\n\nBody";
        assert_eq!(look_of(text, "Session 4").role, Role::Title);
        assert_eq!(look_of(text, "What happened").role, Role::Section);
        assert_eq!(look_of(text, "Later").role, Role::MinorHeading);
        assert!(look_of(text, "## ").markup);
        assert_eq!(look_of(text, "\n\nBody").role, Role::Prose);
        assert_eq!(words(text), ["Session 4", "What happened", "Later", "Body"]);
    }

    #[test]
    fn the_line_after_a_heading_is_plain_so_the_caret_is_its_usual_height() {
        let runs = looks("## What happened\n");
        let (_, last) = runs.last().unwrap();
        assert_eq!(*last, Look::default());
    }

    #[test]
    fn emphasis_shows_as_itself_and_keeps_its_marks() {
        let text = "*Who is playing?* and **who** and ***both*** and ~~old~~";
        assert_eq!(look_of(text, "Who is playing?").role, Role::Italic);
        assert_eq!(look_of(text, "who").role, Role::Strong);
        assert_eq!(look_of(text, "both").role, Role::Strong);
        assert!(look_of(text, "old").struck);
        assert!(look_of(text, "**").markup);
    }

    #[test]
    fn markup_that_is_not_closed_is_only_words() {
        let text = "**not bold yet";
        assert_eq!(styled(text), [(text, Look::default())]);
    }

    #[test]
    fn a_timeline_reference_reads_as_its_words() {
        let text = "We fought at [[Timeline#^t-1a2b|Bridge Ambush]].";
        let link = look_of(text, "Bridge Ambush");
        assert!(link.link && !link.markup);
        assert!(look_of(text, "[[Timeline#^t-1a2b|").markup);
        assert!(look_of(text, "]]").markup);
        assert_eq!(words(text), ["We fought at", "Bridge Ambush", "."]);
    }

    #[test]
    fn a_web_link_reads_as_its_words() {
        let text = "See [the map](https://example.com) now";
        assert!(look_of(text, "the map").link);
        assert!(look_of(text, "](https://example.com)").markup);
    }

    #[test]
    fn code_keeps_its_words_as_typed() {
        let inline = "Roll `1d20 *2*` now";
        assert_eq!(look_of(inline, "1d20 *2*").role, Role::InlineCode);
        assert!(look_of(inline, "`").markup);

        let fence = "```\n# not a heading\n```\n";
        let code = look_of(fence, "# not a heading");
        assert_eq!(code.role, Role::CodeBlock);
        assert!(!code.markup);
    }

    #[test]
    fn list_and_quote_marks_are_quiet() {
        let list = "- first\n- second\n";
        assert!(look_of(list, "-").markup);
        assert_eq!(words(list), ["first", "second"]);

        let quote = "> said\n> again\n";
        assert_eq!(words(quote), ["said", "again"]);
        assert!(
            styled(quote)
                .iter()
                .any(|(part, look)| *part == ">" && look.markup)
        );
    }

    #[test]
    fn a_quote_reads_as_italic_with_its_mark_quiet() {
        let text = "> *a* said\n";
        assert_eq!(look_of(text, " ").role, Role::Italic);
        assert_eq!(look_of(text, " said").role, Role::Italic);
        assert!(look_of(text, ">").markup);
    }

    /// Closure FE9 (rows 118, 119a): only a `splat:` mark that Splat wrote is a fold. A comment
    /// the DM wrote, a mark the DM is still typing (no `-->`), a roll mark, an unknown command and
    /// malformed marks are not.
    #[test]
    fn only_splats_own_generated_mark_folds() {
        let text = "## Plan for tonight\n- [[Bridge Ambush]] <!-- splat:check journey=t-road at=24 sum=12345678 --> Bring silver.\n- Wolves <!-- my note -->\n- <!-- splat:check journey=t\n<!-- splat:rolls key=weather dice=1d6 per=watch -->\n- <!-- splat:hello -->\n- <!-- splat:check at=x -->\n- <!-- splat:check journey=t at=24 sum=zz -->\n";
        let mark = "<!-- splat:check journey=t-road at=24 sum=12345678 -->";
        assert_eq!(
            splat_marks(text)
                .into_iter()
                .map(|mark| &text[mark])
                .collect::<Vec<_>>(),
            vec![mark]
        );
        // The looks GPUI's editor reads mark the same bytes, and only them, as folded.
        let folded: Vec<_> = looks(text)
            .iter()
            .filter(|(_, look)| look.folded)
            .map(|(range, _)| &text[range.clone()])
            .collect();
        assert_eq!(folded.concat(), mark);
    }
}
