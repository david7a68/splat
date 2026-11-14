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
    /// Part of a `<!-- splat:… -->` mark that Splat wrote (a travel check). It folds to one muted
    /// mark unless the caret is on its line (closure FE9).
    folded: bool,
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

/// The marks Splat writes itself, and only those, fold (closure FE9; P2 of the PR 331 review): a
/// closed list, each kind by its command word and the exact shape its writer gives. A mark the DM
/// types (`splat:rolls`), an unknown command and a malformed mark stay in view.
const GENERATED_MARKS: &[(&str, fn(&[&str]) -> bool)] = &[(
    // `timeline::checks::RoadMark::comment`: `journey=<id> at=<offset> sum=<hex>`.
    "check",
    |terms| match terms {
        [journey, at, sum] => {
            journey.strip_prefix("journey=").is_some_and(|id| !id.is_empty())
                && at
                    .strip_prefix("at=")
                    .is_some_and(|at| !at.is_empty() && at.bytes().all(|b| b.is_ascii_digit()))
                && sum
                    .strip_prefix("sum=")
                    .is_some_and(|sum| !sum.is_empty() && sum.bytes().all(|b| b.is_ascii_hexdigit()))
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
    /// The `splat:` marks, in bytes, each with the character it starts at, in order.
    folds: Vec<(Range<usize>, usize)>,
    /// Each line's first character and first byte, for a text with marks (else empty): the caret's
    /// line is found by a search, not by a walk of the text.
    lines: Vec<(usize, usize)>,
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
        let marks = splat_marks(text);
        let mut lines = Vec::new();
        let mut folds = Vec::with_capacity(marks.len());
        if !marks.is_empty() {
            // One walk of the text, when it changes: line starts and each mark's character.
            lines.push((0, 0));
            let mut next = marks.iter().peekable();
            for (char, (byte, letter)) in text.char_indices().enumerate() {
                if next.peek().is_some_and(|mark| mark.start == byte) {
                    folds.push((next.next().unwrap().clone(), char));
                }
                if letter == '\n' {
                    lines.push((char + 1, byte + 1));
                }
            }
        }
        Arc::new(Styled {
            runs,
            section_ends,
            folds,
            lines,
        })
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

#[cfg(test)]
thread_local! {
    /// The layouter's and the paint's steps over runs, sections and folds (a test reads it).
    static FOLD_STEPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn count_fold_step() {
    #[cfg(test)]
    FOLD_STEPS.with(|steps| steps.set(steps.get() + 1));
}

/// A folded mark's letters: this small, and clear. They stay in the text, so the file keeps every
/// byte and the caret can still pass through them.
const FOLDED_SIZE: f32 = 0.05;

/// The mark that stands for a folded `splat:` comment, in the body face (it holds the icons).
const FOLD_MARK: &str = design::icons::EXPAND;

/// The lines that hold the caret or the selection of the editor being laid out, in bytes. The
/// editor is the focused field whose rect (from the last frame) holds this layout's place: egui
/// lays the text out at the `Ui`'s cursor, before it gives the field its rect.
fn caret_lines(ui: &egui::Ui, text: &str, lines: &[(usize, usize)]) -> Option<Range<usize>> {
    let focused = ui.memory(|memory| memory.focused())?;
    let field = ui.ctx().read_response(focused)?;
    if !field.rect.contains(ui.cursor().min) {
        return None;
    }
    let range = egui::TextEdit::load_state(ui.ctx(), focused)?
        .cursor
        .char_range()?;
    let (first, last) = (range.primary.index, range.secondary.index);
    // The line that holds a character: the last line that starts at or before it.
    let line = |at: usize| lines.partition_point(|&(char, _)| char <= at).saturating_sub(1);
    let start = lines.get(line(first.min(last)))?.1;
    let end = lines
        .get(line(first.max(last)) + 1)
        .map_or(text.len(), |&(_, byte)| byte - 1);
    Some(start..end)
}

/// A `TextEdit` layouter. The text is parsed again only when it changes. A `splat:` mark folds
/// to one muted mark (`⋯`, painted by [`paint_section_rules`]) unless the caret or the selection
/// is on its line; its letters stay in the text, so the file keeps every byte (closure FE9).
pub fn layouter(ui: &egui::Ui, text: &dyn TextBuffer, wrap_width: f32) -> Arc<egui::Galley> {
    let text = text.as_str();
    let styled = styled_text(ui, text);
    let prose = design::reader::prose_type(ui);
    let open = (!styled.folds.is_empty())
        .then(|| caret_lines(ui, text, &styled.lines))
        .flatten();
    let mark_font = Type::Body.font(ui);
    let mark_room = FOLD_MARK
        .chars()
        .map(|mark| ui.fonts_mut(|fonts| fonts.glyph_width(&mark_font, mark)))
        .sum::<f32>()
        + design::XS;
    // Runs and folds are both in text order: one moving index finds a run's fold.
    let mut next_fold = 0;
    let mut job = LayoutJob {
        text: text.to_owned(),
        sections: styled
            .runs
            .iter()
            .map(|(range, look)| {
                while styled
                    .folds
                    .get(next_fold)
                    .is_some_and(|(fold, _)| fold.start < range.start)
                {
                    next_fold += 1;
                    count_fold_step();
                }
                count_fold_step();
                let shut = look.folded
                    && open
                        .as_ref()
                        .is_none_or(|open| range.end < open.start || range.start > open.end);
                let mut format = format(ui, prose, *look);
                if shut {
                    format.font_id.size = FOLDED_SIZE;
                    format.color = egui::Color32::TRANSPARENT;
                    format.background = egui::Color32::TRANSPARENT;
                    format.strikethrough = Stroke::NONE;
                }
                egui::text::LayoutSection {
                    leading_space: if shut
                        && styled
                            .folds
                            .get(next_fold)
                            .is_some_and(|(fold, _)| fold.start == range.start)
                    {
                        mark_room
                    } else {
                        0.0
                    },
                    byte_range: range.clone(),
                    format,
                }
            })
            .collect(),
        ..Default::default()
    };
    job.wrap.max_width = wrap_width;
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

const RULE_GAP: f32 = 3.0;

mod sealed {
    /// Only this file implements [`super::Laid`].
    pub trait Sealed {}
    impl Sealed for eframe::egui::text_edit::TextEditOutput {}
    impl Sealed for crate::design::Editor {}
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

impl Laid for design::Editor {
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

/// The rule under a section heading, as in the Reader, and the mark of each folded `splat:`
/// comment. A layouter cannot draw either.
pub fn paint_section_rules(ui: &egui::Ui, output: &impl Laid) {
    let galley = output.galley();
    let galley_pos = output.galley_pos();
    let painter = ui.painter().with_clip_rect(output.text_clip_rect());
    let left = galley_pos.x;
    let styled = styled_text(ui, &galley.job.text);
    for end in &styled.section_ends {
        let row = galley.pos_from_cursor(egui::text::CCursor::new(*end));
        painter.hline(
            left..=left + galley.job.wrap.max_width,
            galley_pos.y + row.bottom() + RULE_GAP,
            Stroke::new(design::HAIRLINE_WIDTH, colors().rule),
        );
    }
    for at in folded_marks(galley, &styled) {
        let glyph = galley
            .pos_from_cursor(egui::text::CCursor::new(at.char))
            .translate(galley_pos.to_vec2());
        painter.text(
            egui::pos2(glyph.left() - at.room / 2.0, glyph.center().y),
            egui::Align2::CENTER_CENTER,
            FOLD_MARK,
            Type::Body.font(ui),
            colors().faint,
        );
    }
}

/// A folded mark in a laid-out text: the character it starts at and the room left for its `⋯`.
struct Folded {
    char: usize,
    room: f32,
}

/// The folded marks of a galley laid out by [`layouter`]: no scan of the text, which can be long,
/// and one pass over the sections and the folds, both in text order.
fn folded_marks(galley: &egui::Galley, styled: &Styled) -> Vec<Folded> {
    let mut sections = galley.job.sections.iter().peekable();
    let mut marks = Vec::new();
    for (fold, char) in &styled.folds {
        while sections
            .peek()
            .is_some_and(|section| section.byte_range.start < fold.start)
        {
            sections.next();
            count_fold_step();
        }
        count_fold_step();
        if let Some(section) = sections
            .peek()
            .filter(|section| section.byte_range.start == fold.start)
            && section.format.font_id.size == FOLDED_SIZE
        {
            marks.push(Folded {
                char: *char,
                room: section.leading_space,
            });
        }
    }
    marks
}

/// The link under the pointer, if the pointer is on its characters and not only near them.
fn link_at(ui: &egui::Ui, output: &impl Laid, pointer: egui::Pos2) -> Option<LinkTarget> {
    if !output.text_clip_rect().contains(pointer) {
        return None;
    }
    let galley = output.galley();
    let text = &galley.job.text;
    // Read first, so that the cache stays while the pointer crosses the gaps between words.
    let links = ui.ctx().memory_mut(|memory| {
        memory
            .caches
            .cache::<FrameCache<Links, Linker>>()
            .get(text.as_str())
    });
    let local = (pointer - output.galley_pos()).to_pos2();
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
    output: &impl Laid,
    from: Option<uuid::Uuid>,
) -> Option<String> {
    let response = output.response();
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

    const TEST_NOTE: design::FieldName = crate::field_id!("markdown_source_test");

    /// A styled Markdown editor of `body`, through the field part, as the app's editors draw it.
    fn styled_editor(ui: &mut egui::Ui, body: &mut String, width: f32) -> design::Editor {
        TEST_NOTE
            .once(design::Once::Example)
            .editor(ui, body, |edit| {
                // The layouter is a function item: leaking its box allocates nothing.
                edit.layouter(Box::leak(Box::new(layouter)))
                    .desired_width(width)
            })
    }

    /// Press on these words in an editor of `text`, with or without Ctrl.
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
                // One row, so that no word is split between two.
                let output = styled_editor(ui, &mut body, 1000.0);
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
    fn the_first_letter_of_a_wrapped_row_is_part_of_its_link() {
        let text = format!("[[Hobb|{}]]\n", "the miller ".repeat(20));
        let context = egui::Context::default();
        design::theme::install(&context);
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut body = text.clone();
                let output = styled_editor(ui, &mut body, 200.0);
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

    /// Closure FE9 (rows 118, 119a): a `splat:` mark that Splat wrote folds to one muted `⋯`
    /// unless the caret is on its line; the text keeps every byte. Must stay true: a comment the
    /// DM wrote, and a mark the DM is still typing (no `-->`), stay in view.
    #[test]
    fn a_splat_mark_folds_unless_the_caret_is_on_its_line() {
        let text = "## Plan for tonight\n- [[Bridge Ambush]] <!-- splat:check journey=t-road at=24 sum=12345678 --> Bring silver.\n- Wolves <!-- my note -->\n- <!-- splat:check journey=t\n<!-- splat:rolls key=weather dice=1d6 per=watch -->\n- <!-- splat:hello -->\n- <!-- splat:check at=x -->\n- <!-- splat:check journey=t at=24 sum=zz -->\n";
        assert_eq!(
            splat_marks(text)
                .into_iter()
                .map(|mark| &text[mark])
                .collect::<Vec<_>>(),
            vec!["<!-- splat:check journey=t-road at=24 sum=12345678 -->"]
        );
        let mark = text.find("<!-- splat").unwrap();
        let char_of = |words: &str| text[..text.find(words).unwrap()].chars().count();
        let context = egui::Context::default();
        design::theme::install(&context);
        let id = TEST_NOTE.once(design::Once::Example).id();
        let mut body = text.to_owned();
        // Whether the mark is folded, how many `⋯` were painted, and whether a repaint follows.
        let mut frame = |events: Vec<egui::Event>| {
            let mut folded = None;
            let output = context.run(
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let output = styled_editor(ui, &mut body, 1000.0);
                        paint_section_rules(ui, &output);
                        let section = |at: usize| {
                            output
                                .galley
                                .job
                                .sections
                                .iter()
                                .find(|section| section.byte_range.contains(&at))
                                .map(|section| section.format.font_id.size)
                        };
                        folded = Some(section(mark) == Some(FOLDED_SIZE));
                        // P2: only Splat's own generated mark folds. The DM's comment, a roll
                        // mark the DM typed, an unknown command and malformed marks stay.
                        for words in [
                            "my note",
                            "splat:rolls",
                            "splat:hello",
                            "splat:check at=x",
                            "sum=zz",
                        ] {
                            assert_ne!(
                                section(text.find(words).unwrap()),
                                Some(FOLDED_SIZE),
                                "fe9-own-comment: {words} stays in view"
                            );
                        }
                    });
                },
            );
            let marks = output
                .shapes
                .iter()
                .filter(|shape| {
                    matches!(&shape.shape, egui::Shape::Text(text)
                        if text.galley.job.text == FOLD_MARK)
                })
                .count();
            let repaint = output
                .viewport_output
                .values()
                .any(|viewport| viewport.repaint_delay == std::time::Duration::ZERO);
            (folded.unwrap(), marks, repaint)
        };
        let place_caret = |at: usize| {
            let mut state = egui::TextEdit::load_state(&context, id).unwrap_or_default();
            state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::one(
                    egui::text::CCursor::new(at),
                )));
            state.store(&context, id);
            context.memory_mut(|memory| memory.request_focus(id));
        };
        let (folded, marks, _) = frame(Vec::new());
        assert!(
            folded && marks == 1,
            "fe9-fold: no focus, one mark ({marks})"
        );
        place_caret(char_of("Bridge"));
        frame(Vec::new());
        let (folded, marks, _) = frame(Vec::new());
        assert!(
            !folded && marks == 0,
            "fe9-open: the caret's line shows the whole mark"
        );
        place_caret(char_of("Wolves"));
        frame(Vec::new());
        let (folded, marks, _) = frame(Vec::new());
        assert!(
            folded && marks == 1,
            "fe9-other-line: the caret on another line folds it"
        );
        // The keyboard moves the caret after the text is laid out; egui's next frame (it asks for
        // one after a key press) opens it. Must stay true: the fold never waits for other input.
        let (_, _, repaint) = frame(vec![egui::Event::Key {
            key: egui::Key::ArrowUp,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]);
        assert!(
            repaint,
            "fe9-key: egui asks for the frame that opens the mark"
        );
        let (folded, _, _) = frame(Vec::new());
        assert!(!folded, "fe9-key: the caret moved up onto the mark's line");
        assert_eq!(body, text, "fe9-bytes: the text keeps every byte");
    }

    /// P2 of the PR 331 review: on a line with a folded mark, every editing gesture gives the
    /// bytes a plain editor gives (the fold changes only the look): a selection across the mark
    /// and typing, Backspace and Delete at its two edges, cut and paste over it, copy (the
    /// clipboard holds the real bytes), Undo and Redo.
    #[test]
    fn editing_across_a_folded_mark_gives_the_bytes_of_a_plain_editor() {
        let text = "Plan\n- [[Bridge Ambush]] <!-- splat:check journey=t-road at=24 sum=12345678 --> Bring silver.\nWolves\n";
        let mark = "<!-- splat:check journey=t-road at=24 sum=12345678 -->";
        let at = |words: &str| text[..text.find(words).unwrap()].chars().count();
        let (start, end) = (at(mark), at(mark) + mark.chars().count());
        let command = egui::Modifiers::COMMAND;
        let key = |key: egui::Key, modifiers: egui::Modifiers| egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        };
        // Each gesture: where the caret or selection is, and the events.
        let gestures: Vec<(&str, (usize, usize), Vec<egui::Event>)> = vec![
            ("copy across the mark", (at("[[Bridge"), end + 1), vec![egui::Event::Copy]),
            ("backspace at its start", (start, start), vec![key(egui::Key::Backspace, egui::Modifiers::NONE)]),
            ("delete at its end", (end - 1, end - 1), vec![key(egui::Key::Delete, egui::Modifiers::NONE)]),
            ("delete at its start", (start - 1, start - 1), vec![key(egui::Key::Delete, egui::Modifiers::NONE)]),
            ("backspace at its end", (end - 2, end - 2), vec![key(egui::Key::Backspace, egui::Modifiers::NONE)]),
            ("select across and type", (at("Bridge"), end), vec![egui::Event::Text("x".into())]),
            ("cut", (2, 12), vec![egui::Event::Cut]),
            ("paste", (3, 3), vec![egui::Event::Paste("<!-- splat:check journey=a at=1 sum=ff -->".into())]),
            ("undo", (3, 3), vec![key(egui::Key::Z, command)]),
            ("redo", (3, 3), vec![key(egui::Key::Z, command | egui::Modifiers::SHIFT)]),
        ];
        // The buffer and the clipboard after each gesture, in an editor with or without the fold.
        let run = |styled: bool| {
            let context = egui::Context::default();
            design::theme::install(&context);
            let id = TEST_NOTE.once(design::Once::Example).id();
            let mut body = text.to_owned();
            let mut after = Vec::new();
            let mut time = 0.0;
            let mut frame = |body: &mut String, events: Vec<egui::Event>, time: f64| {
                let output = context.run(
                    egui::RawInput {
                        events,
                        time: Some(time),
                        ..Default::default()
                    },
                    |ctx| {
                        ctx.memory_mut(|memory| memory.request_focus(id));
                        egui::CentralPanel::default().show(ctx, |ui| {
                            if styled {
                                let _ = styled_editor(ui, body, 1000.0);
                            } else {
                                let _ = TEST_NOTE
                                    .once(design::Once::Example)
                                    .editor(ui, body, |edit| edit.desired_width(1000.0));
                            }
                        });
                    },
                );
                output.platform_output.commands.into_iter().find_map(|command| match command {
                    egui::OutputCommand::CopyText(text) => Some(text),
                    _ => None,
                })
            };
            frame(&mut body, Vec::new(), time);
            for (name, (from, to), events) in &gestures {
                let mut state = egui::TextEdit::load_state(&context, id).unwrap_or_default();
                state.cursor.set_char_range(Some(egui::text::CCursorRange::two(
                    egui::text::CCursor::new(*from),
                    egui::text::CCursor::new(*to),
                )));
                state.store(&context, id);
                time += 2.0;
                frame(&mut body, Vec::new(), time);
                time += 2.0;
                let copied = frame(&mut body, events.clone(), time);
                time += 2.0;
                frame(&mut body, Vec::new(), time);
                after.push((*name, body.clone(), copied));
            }
            after
        };
        let plain = run(false);
        let folded = run(true);
        for (plain, folded) in plain.iter().zip(&folded) {
            assert_eq!(plain, folded, "fe9-edit: {}", plain.0);
        }
        // The gestures did something: the copy holds real bytes, and the text changed.
        let copied = &folded[0].2;
        let whole = format!("[[Bridge Ambush]] {mark} ");
        assert_eq!(copied.as_deref(), Some(whole.as_str()), "fe9-copy: the real bytes");
        assert_ne!(folded[5].1, folded[4].1, "fe9-edit: typing over the selection changed the text");
    }

    /// P2 cost: a note of 100 000 characters with 500 generated marks. One layout and one paint
    /// step over each run, section and fold, never one per pair.
    #[test]
    fn a_long_note_with_many_marks_folds_in_one_pass() {
        let line = "- [[Wolves]] <!-- splat:check journey=t-road at=24 sum=12345678 --> Keep watch by the old road and its stones.\n";
        let mut text = String::new();
        let mut marks = 0;
        while text.len() < 100_000 {
            if marks < 500 {
                text.push_str(line);
                marks += 1;
            } else {
                text.push_str("Plain words of the note go on and on here.\n");
            }
        }
        assert_eq!(splat_marks(&text).len(), 500);
        let context = egui::Context::default();
        design::theme::install(&context);
        let mut body = text.clone();
        FOLD_STEPS.with(|steps| steps.set(0));
        let mut runs = 0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let output = styled_editor(ui, &mut body, 1000.0);
                paint_section_rules(ui, &output);
                runs = styled_text(ui, &text).runs.len();
            });
        });
        let steps = FOLD_STEPS.with(std::cell::Cell::get);
        // Layout: each run once, plus each fold passed once; paint: each section passed once,
        // plus each fold once. Linear: well under the 500 × runs a search per run would take.
        assert!(
            steps <= 2 * (runs + 500) + 10,
            "fe9-cost: {steps} steps for {runs} runs and 500 marks"
        );
        assert!(steps > 0, "fe9-cost: the counter counts");
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
