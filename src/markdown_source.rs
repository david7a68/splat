//! Markdown source, styled where it is typed, in the Reader's faces. Only the look changes:
//! every character stays, markup included, so the file holds exactly what was typed.
use crate::design::text_draft::DraftKey;
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

/// The looks of each drawn document, kept by its draft's key (layout PR 9b: the heading rules
/// and fold marks come from the kept parse, so a quiet frame hashes none of the words). egui
/// runs `update` at the end of each pass: a document not drawn in a pass loses its entry.
#[derive(Default)]
struct KeptStyles {
    kept: std::collections::HashMap<DraftKey, (Arc<Styled>, bool)>,
}

impl egui::cache::CacheTrait for KeptStyles {
    fn update(&mut self) {
        self.kept.retain(|_, (_, used)| std::mem::take(used));
    }

    fn len(&self) -> usize {
        self.kept.len()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// The looks of `text`, the words of the draft revision `key`: looked up by its words once per
/// revision.
fn kept_styles(ui: &egui::Ui, key: DraftKey, text: &str) -> Arc<Styled> {
    let hit = ui.ctx().memory_mut(|memory| {
        let (styled, used) = memory.caches.cache::<KeptStyles>().kept.get_mut(&key)?;
        *used = true;
        Some(styled.clone())
    });
    if let Some(styled) = hit {
        return styled;
    }
    let styled = styled_text(ui, text);
    ui.ctx().memory_mut(|memory| {
        let kept = &mut memory.caches.cache::<KeptStyles>().kept;
        kept.insert(key, (styled.clone(), true));
    });
    styled
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
    let line = |at: usize| {
        lines
            .partition_point(|&(char, _)| char <= at)
            .saturating_sub(1)
    };
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
                }
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
/// comment. A layouter cannot draw either. `words`: the draft's key, when the text is a
/// document's (`TextDraft`): its rules and marks are then placed once per laid-out text (keyed by
/// the draft's key and `pane::kept_layout_made`, which changes with the fold state too), and a
/// frame paints only those inside the clip (layout PR 9b-C: a quiet frame scans and allocates
/// nothing that grows with the note).
pub fn paint_section_rules(ui: &egui::Ui, output: &impl Laid, words: Option<DraftKey>) {
    let galley = output.galley();
    let galley_pos = output.galley_pos();
    let clip = output.text_clip_rect();
    let painter = ui.painter().with_clip_rect(clip);
    let field = output.response().id;
    let made = words.and_then(|_| design::pane::kept_layout_made(ui.ctx(), field));
    let plan = match (words, made) {
        (Some(key), Some(made)) => kept_plan(ui, field, (key, made), galley),
        _ => Arc::new(Plan::of(galley, &styled_text(ui, &galley.job.text))),
    };
    // What is in view, in the galley's own coordinates: rules and marks are in text order, so
    // in height order too.
    let (top, bottom) = (clip.top() - galley_pos.y, clip.bottom() - galley_pos.y);
    let right = galley_pos.x + galley.job.wrap.max_width;
    let first = plan.rules.partition_point(|y| *y + RULE_GAP < top);
    for y in plan.rules[first..]
        .iter()
        .take_while(|y| **y + RULE_GAP <= bottom)
    {
        painter.hline(
            galley_pos.x..=right,
            galley_pos.y + y + RULE_GAP,
            Stroke::new(design::HAIRLINE_WIDTH, colors().rule),
        );
    }
    let first = plan.marks.partition_point(|mark| mark.y < top - FOLD_ROOM);
    for mark in plan.marks[first..]
        .iter()
        .take_while(|mark| mark.y <= bottom + FOLD_ROOM)
    {
        painter.text(
            galley_pos + mark.to_vec2(),
            egui::Align2::CENTER_CENTER,
            FOLD_MARK,
            Type::Body.font(ui),
            colors().faint,
        );
    }
}

/// How far past the clip a fold mark's centre can be and still show part of its `⋯`.
const FOLD_ROOM: f32 = 2.0 * design::L;

/// Where a laid-out text's rules and fold marks go, in the galley's own coordinates: each
/// rule's row bottom, and each mark's centre, in text order.
struct Plan {
    rules: Vec<f32>,
    marks: Vec<egui::Pos2>,
}

impl Plan {
    fn of(galley: &egui::Galley, styled: &Styled) -> Self {
        let rules = styled
            .section_ends
            .iter()
            .map(|end| {
                galley
                    .pos_from_cursor(egui::text::CCursor::new(*end))
                    .bottom()
            })
            .collect();
        let marks = folded_marks(galley, styled)
            .into_iter()
            .map(|at| {
                let glyph = galley.pos_from_cursor(egui::text::CCursor::new(at.char));
                egui::pos2(glyph.left() - at.room / 2.0, glyph.center().y)
            })
            .collect();
        Self { rules, marks }
    }
}

/// Each drawn document's plan, by its field, with the (draft key, laid-out text) it was made
/// for. egui runs `update` at the end of each pass: a field not drawn in a pass loses its plan.
#[derive(Default)]
struct KeptPlans {
    kept: egui::IdMap<((DraftKey, u64), Arc<Plan>, bool)>,
}

impl egui::cache::CacheTrait for KeptPlans {
    fn update(&mut self) {
        self.kept.retain(|_, (_, _, used)| std::mem::take(used));
    }

    fn len(&self) -> usize {
        self.kept.len()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// The plan of field `field`'s galley, the laid-out text `key` names: made once per key.
fn kept_plan(
    ui: &egui::Ui,
    field: egui::Id,
    key: (DraftKey, u64),
    galley: &egui::Galley,
) -> Arc<Plan> {
    let hit = ui.ctx().memory_mut(|memory| {
        let (kept, plan, used) = memory.caches.cache::<KeptPlans>().kept.get_mut(&field)?;
        *used = true;
        (*kept == key).then(|| plan.clone())
    });
    if let Some(plan) = hit {
        return plan;
    }
    let plan = Arc::new(Plan::of(galley, &kept_styles(ui, key.0, &galley.job.text)));
    ui.ctx().memory_mut(|memory| {
        let kept = &mut memory.caches.cache::<KeptPlans>().kept;
        kept.insert(field, (key, plan.clone(), true));
    });
    plan
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
        }
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
