//! Markdown source, styled where it is typed, in the Reader's faces. Only the look changes:
//! every character stays, markup included, so the file holds exactly what was typed.
use crate::design::{self, Type, colors};
use eframe::egui::{
    self, Stroke, TextBuffer, TextFormat,
    text::LayoutJob,
    util::cache::{ComputerMut, FrameCache},
};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};
use std::{ops::Range, sync::Arc};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Role {
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
struct Look {
    role: Role,
    /// Syntax and not words, such as `##`, `**` or the target of a link.
    markup: bool,
    link: bool,
    struck: bool,
}

type Runs = Arc<Vec<(Range<usize>, Look)>>;

/// The look of each stretch of a text. Together they cover all of it.
fn looks(text: &str) -> Runs {
    let mut bytes = vec![Look::default(); text.len()];
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_WIKILINKS;
    for (event, range) in Parser::new_ext(text, options).into_offset_iter() {
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

/// A text's looks, and where each section heading ends, in characters.
struct Styled {
    runs: Runs,
    section_ends: Vec<usize>,
}

#[derive(Default)]
struct Styler;

impl ComputerMut<&str, Arc<Styled>> for Styler {
    fn compute(&mut self, text: &str) -> Arc<Styled> {
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
            .map(|(_, (range, _))| text[..range.end].chars().count())
            .collect();
        Arc::new(Styled { runs, section_ends })
    }
}

fn styled_text(ui: &egui::Ui, text: &str) -> Arc<Styled> {
    ui.ctx().memory_mut(|memory| {
        memory
            .caches
            .cache::<FrameCache<Arc<Styled>, Styler>>()
            .get(text)
    })
}

fn format(ui: &egui::Ui, prose: Type, look: Look) -> TextFormat {
    let role = match look.role {
        Role::Prose => prose,
        Role::Strong | Role::MinorHeading => Type::ProseStrong,
        Role::Italic => Type::ProseItalic,
        Role::InlineCode | Role::CodeBlock => Type::Mono,
        Role::Title => Type::Title,
        Role::Section => Type::Section,
    };
    let keeps_its_line = !matches!(look.role, Role::Prose | Role::Strong | Role::Italic);
    let mut font_id = role.font(ui);
    if matches!(look.role, Role::Strong | Role::Italic) {
        font_id.size = prose.size();
    }
    // Markup is in the plain face beside the words it marks. It keeps their size, so that
    // the two share a baseline.
    if look.markup {
        font_id.family = prose.font(ui).family;
    }
    let color = if look.markup {
        colors().faint
    } else if look.link {
        colors().accent
    } else {
        colors().text
    };
    TextFormat {
        font_id,
        // Mono's own line puts code on the baseline of the serif beside it.
        line_height: Some(if keeps_its_line {
            role.line()
        } else {
            prose.line()
        }),
        color,
        background: if look.role == Role::InlineCode && !look.markup {
            design::reader::inline_code_fill()
        } else {
            egui::Color32::TRANSPARENT
        },
        strikethrough: if look.struck && !look.markup {
            Stroke::new(1.0, color)
        } else {
            Stroke::NONE
        },
        ..Default::default()
    }
}

/// A `TextEdit` layouter. The text is parsed again only when it changes.
pub fn layouter(ui: &egui::Ui, text: &dyn TextBuffer, wrap_width: f32) -> Arc<egui::Galley> {
    let text = text.as_str();
    let styled = styled_text(ui, text);
    let prose = design::reader::prose_type(ui);
    let mut job = LayoutJob {
        text: text.to_owned(),
        sections: styled
            .runs
            .iter()
            .map(|(range, look)| egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: range.clone(),
                format: format(ui, prose, *look),
            })
            .collect(),
        ..Default::default()
    };
    // Lines break where the Reader breaks them, so a note keeps its shape between views.
    job.wrap.max_width = wrap_width.min(design::READER_MEASURE);
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

const RULE_GAP: f32 = 3.0;

/// The rule under a section heading, as in the Reader. A layouter cannot draw one.
pub fn paint_section_rules(ui: &egui::Ui, output: &egui::text_edit::TextEditOutput) {
    let galley = &output.galley;
    let painter = ui.painter().with_clip_rect(output.text_clip_rect);
    let left = output.galley_pos.x;
    for end in &styled_text(ui, &galley.job.text).section_ends {
        let row = galley.pos_from_cursor(egui::text::CCursor::new(*end));
        painter.hline(
            left..=left + galley.job.wrap.max_width,
            output.galley_pos.y + row.bottom() + RULE_GAP,
            Stroke::new(design::HAIRLINE_WIDTH, colors().rule),
        );
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

    #[test]
    fn only_section_headings_get_a_rule_and_it_ends_with_the_heading() {
        let ends = |text: &str| Styler.compute(text).section_ends.clone();
        assert_eq!(ends("# Title\n\nWords\n\n#### Minor\n"), [] as [usize; 0]);
        // Counted in characters, as a caret is.
        let text = "é\n\n## Über\n\n### Later ##\n";
        let [first, second] = ends(text)[..] else {
            panic!("two headings: {:?}", ends(text));
        };
        let chars: Vec<char> = text.chars().collect();
        assert_eq!(chars[..first].iter().collect::<String>(), "é\n\n## Über");
        assert_eq!(
            chars[first..second].iter().collect::<String>(),
            "\n\n### Later ##"
        );
    }

    #[test]
    fn the_layouter_shows_the_same_text_it_was_given() {
        egui::__run_test_ui(|ui| {
            for text in ["", "## Héading\n\n*wörds* [[T#^a|b]]\n"] {
                let text = text.to_owned();
                let galley = layouter(ui, &text, 300.0);
                assert_eq!(galley.text(), text);
                assert!(
                    galley.size().y > 0.0,
                    "{text:?} has no height for the caret"
                );
            }
        });
    }
}
