//! Markdown source, styled where it is typed, in the Reader's faces. Only the look changes:
//! every character stays, markup included, so the file holds exactly what was typed.
use crate::design::{self, Type, colors};
use crate::markdown_view::LinkTarget;
use eframe::egui::{
    self, Stroke, TextBuffer, TextFormat,
    text::LayoutJob,
    util::cache::{ComputerMut, FrameCache},
};
use pulldown_cmark::{Event, HeadingLevel, LinkType, Options, Parser, Tag};
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

type Links = Arc<Vec<(Range<usize>, LinkTarget)>>;

/// Where each link that can be followed is in a text, markup included, and where it goes.
fn links(text: &str) -> Links {
    let found = Parser::new_ext(text, OPTIONS)
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
        .collect();
    Arc::new(found)
}

#[derive(Default)]
struct Linker;

impl ComputerMut<&str, Links> for Linker {
    fn compute(&mut self, text: &str) -> Links {
        links(text)
    }
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
    job.wrap.max_width = wrap_width;
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

/// The link under the pointer, if the pointer is on its characters and not only near them.
fn link_at(
    ui: &egui::Ui,
    output: &egui::text_edit::TextEditOutput,
    pointer: egui::Pos2,
) -> Option<LinkTarget> {
    if !output.text_clip_rect.contains(pointer) {
        return None;
    }
    let galley = &output.galley;
    let text = &galley.job.text;
    // Read first, so that the cache stays while the pointer crosses the gaps between words.
    let links = ui.ctx().memory_mut(|memory| {
        memory
            .caches
            .cache::<FrameCache<Links, Linker>>()
            .get(text.as_str())
    });
    let local = (pointer - output.galley_pos).to_pos2();
    let mut at = 0;
    let mut rows = galley.rows.iter();
    let row = loop {
        let row = rows.next()?;
        if row.rect().y_range().contains(local.y) {
            break row;
        }
        at += row.char_count_including_newline();
    };
    let x = local.x - row.pos.x;
    at += row
        .glyphs
        .iter()
        .position(|glyph| glyph.pos.x <= x && x < glyph.pos.x + glyph.advance_width)?;
    let byte = text.char_indices().nth(at)?.0;
    links
        .iter()
        .find(|(range, _)| range.contains(&byte))
        .map(|(_, target)| target.clone())
}

/// True for a note link that names nothing in the campaign. With no index, it cannot be known.
fn names_nothing(ui: &egui::Ui, note: &str, from: Option<uuid::Uuid>) -> bool {
    ui.ctx()
        .data(|data| {
            data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))
        })
        .is_some_and(|index| matches!(index.resolve_link(note, from), crate::world::Go::Missing(_)))
}

/// With Ctrl held, a link in the text can be pressed, as in the Reader. A plain click only
/// places the caret. A web link opens here. A note link goes back to the caller, which knows
/// where its notes open. `from` is the note being edited.
pub fn pressed_note_link(
    ui: &egui::Ui,
    output: &egui::text_edit::TextEditOutput,
    from: Option<uuid::Uuid>,
) -> Option<String> {
    let response = &output.response;
    // Ctrl+click is a right click on a Mac. There, Cmd is the key.
    if !response.sense.senses_click() || !ui.input(|input| input.modifiers.command) {
        return None;
    }
    let mut target = link_at(ui, output, response.hover_pos()?)?;
    if let LinkTarget::Note(note) = &target
        && names_nothing(ui, note, from)
    {
        target = LinkTarget::Missing;
    }
    match crate::markdown_view::press(ui, response, &target)? {
        LinkTarget::Note(note) => Some(note.clone()),
        _ => None,
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
    fn note_and_web_links_can_be_followed_but_images_and_code_cannot() {
        let text = "See [[Hobb|the miller]], [site](https://example.com) and [here](notes.md).\n\
                    ![[map.png]] `[[Not]]`\n";
        let found: Vec<_> = links(text)
            .iter()
            .map(|(range, target)| (&text[range.clone()], target.clone()))
            .collect();
        assert_eq!(
            found,
            [
                ("[[Hobb|the miller]]", LinkTarget::Note("Hobb".into())),
                (
                    "[site](https://example.com)",
                    LinkTarget::Url("https://example.com".into())
                ),
            ]
        );
    }

    /// Press on these words in an editor of `text`, with or without Ctrl.
    #[allow(clippy::disallowed_methods)]
    fn press_in_editor(text: &str, words: &str, ctrl: bool) -> (Option<String>, Vec<String>) {
        let context = egui::Context::default();
        design::theme::install(&context);
        let at = text[..text.find(words).unwrap()].chars().count();
        let mut body = text.to_owned();
        // The text does not move, so each frame finds the same place.
        let pointer = std::cell::Cell::new(egui::Pos2::ZERO);
        let mut noted = None;
        let mut draw = |ctx: &egui::Context| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut layouter = layouter;
                // One row, so that no word is split between two.
                let output = egui::TextEdit::multiline(&mut body)
                    .layouter(&mut layouter)
                    .desired_width(1000.0)
                    .show(ui);
                noted = noted.take().or(pressed_note_link(ui, &output, None));
                let cursor = |at| {
                    output
                        .galley
                        .pos_from_cursor(egui::text::CCursor::new(at))
                        .center()
                        .to_vec2()
                };
                pointer.set(output.galley_pos + (cursor(at) + cursor(at + 1)) / 2.0);
            });
        };
        let _ = context.run(egui::RawInput::default(), &mut draw);
        let modifiers = if ctrl {
            egui::Modifiers::COMMAND
        } else {
            egui::Modifiers::NONE
        };
        let pointer = pointer.get();
        let mut urls = Vec::new();
        for pressed in [true, false] {
            let events = vec![
                egui::Event::PointerMoved(pointer),
                egui::Event::PointerButton {
                    pos: pointer,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers,
                },
            ];
            let input = egui::RawInput {
                modifiers,
                events,
                ..Default::default()
            };
            let output = context.run(input, &mut draw);
            for command in output.platform_output.commands {
                if let egui::OutputCommand::OpenUrl(url) = command {
                    urls.push(url.url);
                }
            }
        }
        (noted, urls)
    }

    #[test]
    fn ctrl_click_follows_a_link_in_the_editor_and_a_plain_click_does_not() {
        let text = "Ask [[Hobb|the miller]] or read [the site](https://example.com).\n";
        assert_eq!(
            press_in_editor(text, "miller", true),
            (Some("Hobb".to_owned()), vec![])
        );
        assert_eq!(
            press_in_editor(text, "site", true),
            (None, vec!["https://example.com".to_owned()])
        );
        assert_eq!(press_in_editor(text, "miller", false), (None, vec![]));
        assert_eq!(press_in_editor(text, "Ask", true), (None, vec![]));
    }

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn the_first_letter_of_a_wrapped_row_is_part_of_its_link() {
        let text = format!("[[Hobb|{}]]\n", "the miller ".repeat(20));
        let context = egui::Context::default();
        design::theme::install(&context);
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut body = text.clone();
                let mut layouter = layouter;
                let output = egui::TextEdit::multiline(&mut body)
                    .layouter(&mut layouter)
                    .desired_width(200.0)
                    .show(ui);
                let row = &output.galley.rows[1];
                let glyph = &row.glyphs[0];
                let pointer = output.galley_pos
                    + row.pos.to_vec2()
                    + egui::vec2(glyph.pos.x + glyph.advance_width / 2.0, row.size.y / 2.0);
                assert_eq!(
                    link_at(ui, &output, pointer),
                    Some(LinkTarget::Note("Hobb".into()))
                );
            });
        });
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
