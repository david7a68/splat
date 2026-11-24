//! The Reader's Markdown view (egui: `markdown_view`): headings, paragraphs, emphasis, lists with
//! each mark in its own column, task boxes, block quotes, code, rules, tables (wide ones scroll
//! sideways under a fixed first column), roll tables, field rows, figures, and links that go
//! where the egui reader's go: a note link to Lookup, a bold name to Lookup, a web link to the
//! browser, a dice link to whoever rolls.
//!
//! ```ignore
//! kit::Markdown::new(("entry", id), &entry.body)
//!     .document(entry.id)              // its links resolve from this note
//!     .compact()                       // a side panel: `ProseCompact`
//!     .jump_in(&state.scroll)          // a typed jump to a heading scrolls this column
//! ```
//!
//! A pressed link with no handler of the caller's is left in the app's egui context for the
//! frame logic, as the egui reader leaves it: `markdown_view::take_note_link` (routed by
//! `Splat::route`), `markdown_view::take_lookup_link`, `rolls::take_ask`, `roll_table::take_roll`.
//! The context is the [`Host`] global, which `pages::reader::init` installs.
//!
//! What a document prepares (its parsed blocks, its tables' columns) is kept under its element id
//! and made again only when its words, its mode, its document, the link index, the art or its
//! width change: a quiet frame parses and measures nothing.
mod parse;
mod table;

use crate::gui::icons;
use crate::gui::theme::{
    HAIRLINE_WIDTH, L, LINE_GAP, M, READER_MEASURE, S, Type, TypeExt, XL, XS, colors,
};
pub use crate::markdown_view::LinkTarget;
use eframe::egui;
use gpui::{
    AnyElement, App, AppContext, Bounds, ElementId, Entity, FontStyle, FontWeight, InteractiveText,
    IntoElement, ObjectFit, ParentElement, Pixels, RenderOnce, ScrollHandle, SharedString,
    StrikethroughStyle, Styled, StyledImage, StyledText, TextRun, UnderlineStyle, Window, canvas,
    div, img, point, prelude::*, px,
};
use parse::{Block, Look, Mode, Paragraph, Words};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::sync::Arc;

type OnLink = Rc<dyn Fn(&str, &mut Window, &mut App)>;
type OnTarget = Rc<dyn Fn(&LinkTarget, &mut Window, &mut App)>;

/// The app's egui context (`root.bridge.ctx()`): where a reader finds its link index and the art,
/// and leaves a pressed link for the frame logic.
pub struct Host(pub egui::Context);

impl gpui::Global for Host {}

/// The host's context, when the app installed one (a kit test may not).
fn host(cx: &App) -> Option<egui::Context> {
    cx.try_global::<Host>().map(|host| host.0.clone())
}

/// The link index the app keeps for readers (`reader_link_index`), when a campaign is open.
fn link_index(ctx: &egui::Context) -> Option<Arc<crate::lookup::ContentIndex>> {
    ctx.data(|data| {
        data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))
    })
}

/// A Markdown document in the reader's voice.
#[derive(IntoElement)]
pub struct Markdown {
    id: ElementId,
    source: SharedString,
    mode: Mode,
    /// The prose type was chosen by the caller (`compact`), not by the width.
    chosen: bool,
    /// A Skills row: each link ends an entry and its target covers that entry.
    entries: bool,
    measure: Option<Pixels>,
    wide_tables: bool,
    document: Option<uuid::Uuid>,
    on_link: Option<OnLink>,
    on_target: Option<OnTarget>,
    on_note: Option<OnLink>,
    jump: Option<ScrollHandle>,
}

/// `markdown(id, source)`: a Markdown document (see [`Markdown`]).
pub fn markdown(id: impl Into<ElementId>, source: impl Into<SharedString>) -> Markdown {
    Markdown::new(id, source)
}

impl Markdown {
    /// The Reader's prose at `READER_MEASURE`. Its type is `Prose`, or `ProseCompact` in a column
    /// narrower than `READER_MEASURE - 60` (as egui's `reader::prose_type`).
    pub fn new(id: impl Into<ElementId>, source: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            source: source.into(),
            mode: Mode::Reader,
            chosen: false,
            entries: false,
            measure: Some(READER_MEASURE),
            wide_tables: false,
            document: None,
            on_link: None,
            on_target: None,
            on_note: None,
            jump: None,
        }
    }

    /// `ProseCompact` type at any width, for a side panel.
    pub fn compact(mut self) -> Self {
        self.mode = Mode::Compact;
        self.chosen = true;
        self
    }

    /// Compact field rows in `Stat` type (egui `show_compact`): a `Field`/`Value` table becomes
    /// field rows, headings are `StatLabel`, and code is not set apart.
    pub fn fields(mut self) -> Self {
        self.mode = Mode::Dense;
        self.chosen = true;
        self
    }

    /// A stat block's Skills row (egui `show_skills`): field rows whose links are each one
    /// skill's bonus, their target covering the skill's name before them.
    pub fn skills(mut self) -> Self {
        self = self.fields();
        self.entries = true;
        self
    }

    /// A plan row's words (egui `show_row_words`): `Body` type on `XL` lines, marks kept in the
    /// line, no space after; `muted` for a done scene (its links keep their underline).
    pub fn row_words(mut self, muted: bool) -> Self {
        self.mode = Mode::Row { muted };
        self.chosen = true;
        self.measure = None;
        self
    }

    /// Fill the parent's width instead of keeping `READER_MEASURE`.
    pub fn full_width(mut self) -> Self {
        self.measure = None;
        self
    }

    /// Let tables take the parent's whole width while prose keeps its measure (egui
    /// `with_wide_tables`): a table wider than the measure scrolls only when the pane is too
    /// narrow for it.
    pub fn wide_tables(mut self) -> Self {
        self.wide_tables = true;
        self
    }

    /// The note or entry this document is, so its links resolve from it (a heading link, a
    /// relative name) and a followed note link says where it came from.
    pub fn document(mut self, id: impl Into<Option<uuid::Uuid>>) -> Self {
        self.document = id.into();
        self
    }

    /// A press on a note, web or Lookup link, with its target: a note link's destination
    /// (`splat-note:…`), a web link's URL, or a bold name's words. It replaces the reader's own
    /// routing of those links; a dice link still asks for its roll.
    pub fn on_link(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_link = Some(Rc::new(handler));
        self
    }

    /// A press on a note link, with the note it names (the words between `[[` and `]]`, as
    /// `markdown_view::take_note_link` gives them). Every other link keeps the reader's own
    /// routing: a web link opens the browser, a bold name goes to Lookup, a dice link asks for
    /// its roll. For a reader that follows its own note links (World's, the Library's).
    pub fn on_note(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_note = Some(Rc::new(handler));
        self
    }

    /// A press on any link, with where it goes. It replaces the reader's own routing.
    pub fn on_target(
        mut self,
        handler: impl Fn(&LinkTarget, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_target = Some(Rc::new(handler));
        self
    }

    /// The scroll column this document is in: a typed jump to one of its headings
    /// (`markdown_view::want_heading`) scrolls the column so the heading is at its top.
    pub fn jump_in(mut self, handle: &ScrollHandle) -> Self {
        self.jump = Some(handle.clone());
        self
    }
}

/// What a document keeps between frames.
#[derive(Default)]
struct Prepared {
    key: u64,
    blocks: Rc<Vec<Block>>,
    /// The document's width in the last layout: it chooses the prose type and the tables'
    /// columns.
    width: Option<Pixels>,
    /// Each table's columns and rows, for the width they were laid out at.
    tables: HashMap<usize, (Pixels, Rc<table::Laid>)>,
    /// Each roll table's typed roll.
    typed: HashMap<usize, String>,
}

impl RenderOnce for Markdown {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let ctx = host(cx);
        let index = ctx.as_ref().and_then(link_index);
        let kept = window.use_keyed_state(self.id.clone(), cx, |_, _| Prepared::default());
        let width = kept.read(cx).width;
        // The prose type follows the column's width unless the caller chose it.
        let mode = match self.mode {
            Mode::Reader
                if !self.chosen
                    && width.is_some_and(|width| {
                        width.min(self.measure.unwrap_or(width)) < READER_MEASURE - px(60.0)
                    }) =>
            {
                Mode::Compact
            }
            mode => mode,
        };
        let key = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            self.source.hash(&mut hasher);
            mode.hash(&mut hasher);
            self.entries.hash(&mut hasher);
            self.document.hash(&mut hasher);
            index
                .as_ref()
                .map(|index| index.generation())
                .hash(&mut hasher);
            ctx.as_ref()
                .and_then(crate::art::generation)
                .hash(&mut hasher);
            hasher.finish()
        };
        if kept.read(cx).key != key || kept.read(cx).blocks.is_empty() {
            let art = |url: &str| ctx.as_ref().and_then(|ctx| crate::art::link_art(ctx, url));
            let mut blocks = parse::prepare(
                &self.source,
                &parse::Inputs {
                    mode,
                    index: index.as_deref(),
                    from: self.document,
                    art: &art,
                },
            );
            short_links(&mut blocks, self.entries, window);
            kept.update(cx, |kept, _| {
                kept.key = key;
                kept.blocks = Rc::new(blocks);
                kept.tables.clear();
            });
        }
        let blocks = kept.read(cx).blocks.clone();
        let draw = Draw {
            id: self.id.clone(),
            mode,
            document: self.document,
            on_link: self.on_link.clone(),
            on_target: self.on_target.clone().or_else(|| {
                let on_note = self.on_note.clone()?;
                let document = self.document;
                Some(Rc::new(
                    move |target: &LinkTarget, window: &mut Window, cx: &mut App| {
                        if let LinkTarget::Note(note) = target {
                            on_note(note, window, cx);
                            window.refresh();
                        } else {
                            let ctx = host(cx);
                            follow(target, document, (None, None, ctx.as_ref()), window, cx);
                        }
                    },
                ) as OnTarget)
            }),
            ctx: ctx.clone(),
            kept: kept.clone(),
            reader: egui::Id::new(("gpui_reader", format!("{:?}", self.id))),
        };
        // The width tables share: the whole document's in a wide reader, else the measure's.
        let table_width = width.map(|width| {
            if self.wide_tables {
                width
            } else {
                width.min(self.measure.unwrap_or(width))
            }
        });
        // The open quotes: each one's blocks, drawn inside its bar when it closes.
        let mut stack: Vec<Vec<AnyElement>> = vec![Vec::new()];
        let spacing = match mode {
            Mode::Dense => px(3.0),
            Mode::Row { .. } => px(0.0),
            _ => LINE_GAP,
        };
        let prose_measure = if self.wide_tables { self.measure } else { None };
        for (at, block) in blocks.iter().enumerate() {
            let element: Option<AnyElement> = match block {
                Block::Gap(gap) => Some(super::gap(*gap).into_any_element()),
                Block::Rule => Some(
                    div()
                        .w_full()
                        .when_some(prose_measure, |rule, measure| rule.max_w(measure))
                        .pb(spacing)
                        .child(super::rule())
                        .into_any_element(),
                ),
                Block::Heading(words) => self.jump.as_ref().and_then(|handle| {
                    let wanted = ctx
                        .as_ref()
                        .is_some_and(|ctx| crate::markdown_view::wanted_heading(ctx, words));
                    wanted.then(|| jump_here(handle.clone()).into_any_element())
                }),
                Block::QuoteStart => {
                    stack.push(Vec::new());
                    None
                }
                Block::QuoteEnd => {
                    let inner = stack.pop().unwrap_or_default();
                    Some(
                        div()
                            .flex()
                            .flex_col()
                            .border_l(px(2.0))
                            .border_color(colors().rule)
                            .pl(L - px(2.0))
                            .children(inner)
                            .into_any_element(),
                    )
                }
                Block::Paragraph(paragraph) => {
                    // A plan row's words, and field rows' last line, end with no space: the row
                    // around them owns the space after them (egui's `show_skills` in its row).
                    let row = matches!(mode, Mode::Row { .. })
                        || (mode == Mode::Dense && at + 1 == blocks.len());
                    Some(
                        div()
                            .flex()
                            .flex_col()
                            .when_some(prose_measure, |block, measure| block.max_w(measure))
                            .child(draw.paragraph(at, paragraph))
                            .when(!row, |block| {
                                block.pb(spacing).child(super::gap(paragraph.after))
                            })
                            .into_any_element(),
                    )
                }
                Block::Figures(paths, caption) => Some(figures(paths, caption)),
                Block::Table(table) => Some(match table_width {
                    Some(width) => {
                        let laid = draw.laid_table(at, table, width, window, cx);
                        draw.table(at, table, &laid, window, cx)
                    }
                    // The first frame has no width yet: the next one lays the table out.
                    None => div().h(Type::Stat.line()).into_any_element(),
                }),
                Block::Fields(rows) => Some(fields(rows, table_width, spacing)),
            };
            if let Some(element) = element
                && let Some(open) = stack.last_mut()
            {
                open.push(element);
            }
        }
        // An unclosed quote cannot happen in Markdown; keep its blocks if it does.
        while stack.len() > 1 {
            let inner = stack.pop().unwrap_or_default();
            if let Some(open) = stack.last_mut() {
                open.extend(inner);
            }
        }
        let measured = kept.clone();
        let mut document = div()
            .relative()
            .flex()
            .flex_col()
            .w_full()
            .children(stack.pop().unwrap_or_default())
            .child(
                // The document's width, for the next frame's prose type and table columns.
                canvas(
                    move |bounds: Bounds<Pixels>, window, cx| {
                        let width = bounds.size.width;
                        let changed = measured
                            .read(cx)
                            .width
                            .is_none_or(|old| (old - width).abs() > px(0.5));
                        if changed {
                            measured.update(cx, |kept, _| kept.width = Some(width));
                            window.on_next_frame(|window, _| window.refresh());
                        }
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top_0()
                .left_0()
                .size_full(),
            );
        if !self.wide_tables
            && let Some(measure) = self.measure
        {
            document = document.max_w(measure);
        }
        document
    }
}

/// An empty mark where a typed jump lands: as it is laid out, its column scrolls it to its top.
/// A reader that draws a heading outside a document (an entry's section, a note's section) puts
/// one before it when `markdown_view::wanted_heading` says the jump is for that heading.
pub fn jump_here(handle: ScrollHandle) -> impl IntoElement {
    canvas(
        move |bounds: Bounds<Pixels>, window, _| {
            let column = handle.bounds();
            let offset = handle.offset();
            let below = bounds.top() - column.top();
            if below.abs() > px(0.5) {
                handle.set_offset(point(offset.x, offset.y - below));
                window.on_next_frame(|window, _| window.refresh());
            }
        },
        |_, _, _, _| {},
    )
    .w_full()
    .h(px(0.0))
}

/// Give each short link at least `XL` of width, as the egui reader does, so its press target
/// covers no other words. In prose the spare width is no-break space around the words (all of it
/// after a link that starts its paragraph or that punctuation follows, the punctuation then part
/// of the target); in a Skills row (`entries`) the target grows left over its own entry instead.
fn short_links(blocks: &mut [Block], entries: bool, window: &Window) {
    for block in blocks {
        let Block::Paragraph(paragraph) = block else {
            continue;
        };
        if paragraph.links.is_empty() {
            continue;
        }
        let size = paragraph.size;
        let width = |words: &Words, range: std::ops::Range<usize>| {
            let part = Words {
                text: words.text[range.clone()].to_owned(),
                looks: words
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
                line: words.line,
            };
            window
                .text_system()
                .shape_line(part.text.clone().into(), size, &runs(&part), None)
                .width
        };
        if entries {
            for index in 0..paragraph.links.len() {
                let range = paragraph.links[index].0.clone();
                if width(&paragraph.words, range.clone()) >= XL {
                    continue;
                }
                let floor = index
                    .checked_sub(1)
                    .map_or(0, |before| paragraph.links[before].0.end);
                let text = &paragraph.words.text[floor..range.start];
                // The entry starts after the last breaking blank before the link.
                let start = text
                    .char_indices()
                    .rev()
                    .find(|(_, c)| c.is_whitespace() && *c != '\u{a0}')
                    .map_or(floor, |(at, c)| floor + at + c.len_utf8());
                paragraph.links[index].0.start = start;
            }
            continue;
        }
        // From the last link back, so each insertion leaves the earlier ranges as they are.
        let space = {
            let look = paragraph.words.looks.first().map_or_else(
                || Look {
                    role: Type::Prose,
                    color: colors().text,
                    underline: false,
                    struck: false,
                    fill: None,
                    slant: false,
                    icon: false,
                },
                |(_, look)| *look,
            );
            let probe = Words {
                text: "\u{a0}".into(),
                looks: vec![(0..2, look)],
                line: paragraph.words.line,
            };
            window
                .text_system()
                .shape_line(probe.text.clone().into(), size, &runs(&probe), None)
                .width
                .max(px(1.0))
        };
        for index in (0..paragraph.links.len()).rev() {
            let range = paragraph.links[index].0.clone();
            if matches!(paragraph.links[index].1, LinkTarget::Missing) {
                continue;
            }
            let spare = XL - width(&paragraph.words, range.clone());
            if spare <= px(0.0) {
                continue;
            }
            let count = (spare / space).ceil() as usize;
            let text = &paragraph.words.text;
            // Punctuation right after the words stays with them; the space goes after it.
            let punctuation = text[range.end..]
                .char_indices()
                .take_while(|(_, c)| closes_phrase(*c))
                .last()
                .map_or(0, |(at, c)| at + c.len_utf8());
            let (before, after) = if range.start == 0 || punctuation > 0 {
                (0, count)
            } else {
                (count / 2, count - count / 2)
            };
            let end = range.end + punctuation;
            insert_space(paragraph, end, after);
            insert_space(paragraph, range.start, before);
            paragraph.links[index].0 = range.start..end + (before + after) * '\u{a0}'.len_utf8();
        }
    }
}

/// Characters that end a phrase and sit right after a link's words.
fn closes_phrase(character: char) -> bool {
    matches!(
        character,
        '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}' | '\'' | '"' | '’' | '”' | '…'
    )
}

/// `count` no-break spaces into a paragraph's words at byte `at`, in the look of the words
/// before them without an underline. Ranges at or after `at` move; a range across it grows.
fn insert_space(paragraph: &mut Paragraph, at: usize, count: usize) {
    if count == 0 {
        return;
    }
    let space = "\u{a0}".repeat(count);
    let added = space.len();
    let words = &mut paragraph.words;
    let look = words
        .looks
        .iter()
        .find(|(stretch, _)| stretch.start < at && stretch.end >= at)
        .or_else(|| words.looks.first())
        .map(|(_, look)| Look {
            underline: false,
            struck: false,
            fill: None,
            icon: false,
            ..*look
        });
    words.text.insert_str(at, &space);
    let mut looks = Vec::with_capacity(words.looks.len() + 2);
    for (stretch, old) in words.looks.drain(..) {
        if stretch.end <= at {
            looks.push((stretch, old));
        } else if stretch.start >= at {
            looks.push((stretch.start + added..stretch.end + added, old));
        } else {
            looks.push((stretch.start..at, old));
            looks.push((at + added..stretch.end + added, old));
        }
    }
    if let Some(look) = look {
        looks.push((at..at + added, look));
    }
    looks.sort_by_key(|(stretch, _)| stretch.start);
    words.looks = looks;
    for (range, _) in &mut paragraph.links {
        if range.start >= at {
            range.start += added;
            range.end += added;
        } else if range.end > at {
            range.end += added;
        }
    }
}

/// The text runs of styled words.
pub(crate) fn runs(words: &Words) -> Vec<TextRun> {
    words
        .looks
        .iter()
        .map(|(range, look)| {
            let mut font = gpui::font(if look.icon {
                icons::FAMILY
            } else {
                look.role.family()
            });
            font.weight = if look.icon {
                FontWeight::NORMAL
            } else {
                look.role.weight()
            };
            font.style = if look.slant || look.role.style() == FontStyle::Italic {
                FontStyle::Italic
            } else {
                FontStyle::Normal
            };
            TextRun {
                len: range.len(),
                font,
                color: look.color,
                background_color: look.fill,
                underline: look.underline.then_some(UnderlineStyle {
                    thickness: HAIRLINE_WIDTH,
                    color: Some(look.color),
                    wavy: false,
                }),
                strikethrough: look.struck.then_some(StrikethroughStyle {
                    thickness: px(1.0),
                    color: Some(look.color),
                }),
            }
        })
        .collect()
}

/// What drawing a document's blocks needs.
struct Draw {
    id: ElementId,
    mode: Mode,
    document: Option<uuid::Uuid>,
    on_link: Option<OnLink>,
    on_target: Option<OnTarget>,
    ctx: Option<egui::Context>,
    kept: Entity<Prepared>,
    /// The reader's id in the egui context, where a roll table keeps its state.
    reader: egui::Id,
}

impl Draw {
    fn child_id(&self, name: String) -> ElementId {
        ElementId::NamedChild(Arc::new(self.id.clone()), name.into())
    }

    /// One paragraph: its words, its links, a list item's mark or a task's box.
    fn paragraph(&self, at: usize, paragraph: &Paragraph) -> AnyElement {
        let words = self.words(at, paragraph);
        let row = matches!(self.mode, Mode::Row { .. });
        if paragraph.lead.is_none() && paragraph.depth == 0 {
            return words;
        }
        // The mark in its own column, ending where the words start; the words wrap at their own
        // edge. In a plan row (depth 0) the mark takes its own width.
        let room = XL * paragraph.depth as f32;
        let mark = paragraph.lead.as_ref().map(|lead| {
            div()
                .flex_none()
                .whitespace_nowrap()
                .text_size(paragraph.size)
                .line_height(lead.line)
                .child(StyledText::new(lead.text.clone()).with_runs(runs(lead)))
        });
        div()
            .flex()
            .flex_row()
            .child(
                div()
                    .flex_none()
                    .when(!row, |column| column.min_w(room))
                    .flex()
                    .flex_row()
                    .justify_end()
                    .children(mark),
            )
            .child(div().flex_1().min_w_0().child(words))
            .into_any_element()
    }

    /// A paragraph's styled words, its links pressable.
    fn words(&self, at: usize, paragraph: &Paragraph) -> AnyElement {
        let words = &paragraph.words;
        let text = StyledText::new(words.text.clone()).with_runs(runs(words));
        let block = div()
            .w_full()
            .text_size(paragraph.size)
            .line_height(words.line);
        if paragraph.links.is_empty() {
            return block.child(text).into_any_element();
        }
        let links: Rc<Vec<_>> = Rc::new(paragraph.links.clone());
        let pressable: Vec<_> = links
            .iter()
            .enumerate()
            .filter(|(_, (_, target))| !matches!(target, LinkTarget::Missing))
            .map(|(index, (range, _))| (index, range.clone()))
            .collect();
        let (which, ranges): (Vec<usize>, Vec<_>) = pressable.into_iter().unzip();
        let follow = {
            let links = links.clone();
            let on_link = self.on_link.clone();
            let on_target = self.on_target.clone();
            let ctx = self.ctx.clone();
            let from = self.document;
            move |pressed: usize, window: &mut Window, cx: &mut App| {
                let Some((_, target)) = which.get(pressed).and_then(|index| links.get(*index))
                else {
                    return;
                };
                follow(
                    target,
                    from,
                    (on_link.as_ref(), on_target.as_ref(), ctx.as_ref()),
                    window,
                    cx,
                );
            }
        };
        let tips = {
            let links = links.clone();
            let ctx = self.ctx.clone();
            let from = self.document;
            move |index: usize, window: &mut Window, cx: &mut App| -> Option<gpui::AnyView> {
                let (_, target) = links.iter().find(|(range, _)| range.contains(&index))?;
                let words = hint(target, ctx.as_ref(), from)?;
                Some(super::tooltip::tip(words.into())(window, cx))
            }
        };
        // The harness finds a linked paragraph by its words.
        let name = words.text.clone();
        super::target(
            block.child(
                InteractiveText::new(self.child_id(format!("words-{at}")), text)
                    .on_click(ranges, follow)
                    .tooltip(tips),
            ),
            name,
        )
        .into_any_element()
    }

    /// A table's columns and rows, kept for the width they were laid out at.
    fn laid_table(
        &self,
        at: usize,
        table: &parse::Table,
        width: Pixels,
        window: &mut Window,
        cx: &mut App,
    ) -> Rc<table::Laid> {
        if let Some((kept_width, laid)) = self.kept.read(cx).tables.get(&at)
            && (*kept_width - width).abs() <= px(0.5)
        {
            return laid.clone();
        }
        let laid = Rc::new(table::lay_out(table, width, window));
        self.kept.update(cx, |kept, _| {
            kept.tables.insert(at, (width, laid.clone()));
        });
        laid
    }

    fn table(
        &self,
        at: usize,
        table: &parse::Table,
        laid: &table::Laid,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let matched = table.rolls.as_ref().and_then(|marker| {
            self.ctx.as_ref().and_then(|ctx| {
                crate::roll_table::matched_in(ctx, self.roll_reader(at), marker, &table.rows)
            })
        });
        let mut block = div().flex().flex_col().w_full().child(table::draw(
            self.child_id(format!("table-{at}")),
            table,
            laid,
            matched,
        ));
        if let Some(marker) = &table.rolls {
            block = block.child(self.roll_control(at, table, marker, window, cx));
        }
        block.child(super::gap(S)).into_any_element()
    }

    /// The id a roll table's state has in the egui context: this reader's and the table's.
    fn roll_reader(&self, at: usize) -> egui::Id {
        self.reader.with(at)
    }

    /// **Roll** and a box for a typed result under a marked table, its result line, and its
    /// warnings (egui `roll_table::roll_control`).
    fn roll_control(
        &self,
        at: usize,
        table: &parse::Table,
        marker: &Result<crate::roll_table::Marker, String>,
        _: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let marker = match marker {
            Ok(marker) => marker,
            Err(error) => return warning(error),
        };
        if let Err(error) = crate::roll_table::table_rows(&marker.dice, &table.rows) {
            return warning(&error);
        }
        let reader = self.roll_reader(at);
        let ask = {
            let ctx = self.ctx.clone();
            let marker = Ok(marker.clone());
            let rows = table.rows.clone();
            move |typed: Option<i64>, window: &mut Window| {
                if let Some(ctx) = &ctx {
                    let _ = crate::roll_table::ask_in(ctx, reader, &marker, &rows, typed);
                    window.refresh();
                }
            }
        };
        let ask = Rc::new(ask);
        let typed = self
            .kept
            .read(cx)
            .typed
            .get(&at)
            .cloned()
            .unwrap_or_default();
        let on_change = {
            let kept = self.kept.clone();
            move |words: &str, _: &mut Window, cx: &mut App| {
                kept.update(cx, |kept, _| {
                    kept.typed.insert(at, words.to_owned());
                });
            }
        };
        let on_enter = {
            let kept = self.kept.clone();
            let ask = ask.clone();
            let ctx = self.ctx.clone();
            let key = marker.key.clone();
            move |window: &mut Window, cx: &mut App| {
                let typed = kept.update(cx, |kept, _| kept.typed.remove(&at).unwrap_or_default());
                match crate::design::parse_signed(&typed) {
                    Some(value) => ask(Some(value), window),
                    None if typed.trim().is_empty() => {}
                    None => {
                        if let Some(ctx) = &ctx {
                            crate::roll_table::typed_not_a_number_in(ctx, reader, &key);
                        }
                        window.refresh();
                    }
                }
            }
        };
        let result = self
            .ctx
            .as_ref()
            .and_then(|ctx| crate::roll_table::result_in(ctx, reader, &marker.key));
        let c = colors();
        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(S)
                    .child(
                        super::Button::secondary(
                            self.child_id(format!("roll-{at}")),
                            format!("Roll {}", marker.dice),
                        )
                        .icon(icons::DICE)
                        .density(crate::gui::theme::Density::Read)
                        .on_click(move |_, window, _| ask(None, window)),
                    )
                    .child(
                        super::TextField::new(self.child_id(format!("typed-{at}")), typed)
                            .placeholder("Typed roll")
                            .name(format!(
                                "Typed roll for {}",
                                crate::design::reader::human_label(&marker.key)
                            ))
                            .width(crate::gui::theme::XXL * 3.0)
                            .on_change(on_change)
                            .on_enter(on_enter),
                    ),
            )
            // One line under the Roll row is kept for the result, so a roll moves nothing.
            .child(
                div()
                    .h(Type::Body.line())
                    .w_full()
                    .overflow_hidden()
                    .children(result.map(|result| {
                        super::target(
                            super::text::one_line(Type::Body, c.text, result),
                            ROLL_RESULT,
                        )
                    })),
            )
            .children(marker.per.warning().map(|words| warning(&words)))
            .into_any_element()
    }
}

/// The harness's name for a roll table's result line.
pub(crate) const ROLL_RESULT: &str = "Roll result";

/// A warning under a table, in its parser's words.
fn warning(words: &str) -> AnyElement {
    super::Message::new(super::Tone::Warning, words.to_owned())
        .small()
        .into_any_element()
}

/// Follow a pressed link: the caller's handler, else the reader's routing, as egui's.
fn follow(
    target: &LinkTarget,
    from: Option<uuid::Uuid>,
    (on_link, on_target, ctx): (Option<&OnLink>, Option<&OnTarget>, Option<&egui::Context>),
    window: &mut Window,
    cx: &mut App,
) {
    if let Some(on_target) = on_target {
        on_target(target, window, cx);
        return;
    }
    if let Some(on_link) = on_link {
        let destination = match target {
            LinkTarget::Note(note) => Some(format!("{}{note}", crate::markdown_view::NOTE_LINK)),
            LinkTarget::Url(url) => Some(url.clone()),
            LinkTarget::Lookup(words) => Some(words.clone()),
            LinkTarget::Roll(_) | LinkTarget::Missing => None,
        };
        if let Some(destination) = destination {
            on_link(&destination, window, cx);
            return;
        }
    }
    match target {
        LinkTarget::Missing => return,
        LinkTarget::Url(url) => {
            cx.open_url(url);
            return;
        }
        LinkTarget::Note(note) => {
            if let Some(ctx) = ctx {
                crate::markdown_view::send_note_link(ctx, note.clone(), from);
            }
        }
        LinkTarget::Lookup(words) => {
            if let Some(ctx) = ctx {
                crate::markdown_view::send_lookup_link(ctx, words.clone());
            }
        }
        LinkTarget::Roll(link) => {
            if let Some(ctx) = ctx {
                let keys = window.modifiers();
                crate::rolls::ask_pressed(ctx, link, keys.shift, keys.control || keys.platform);
            }
        }
    }
    // The next frame's logic routes what the press left (`Splat::route`).
    window.refresh();
}

/// A link's hover words: where it goes, or why it goes nowhere.
fn hint(
    target: &LinkTarget,
    ctx: Option<&egui::Context>,
    from: Option<uuid::Uuid>,
) -> Option<String> {
    match target {
        LinkTarget::Missing => Some("Not found in this campaign".to_owned()),
        LinkTarget::Url(url) => Some(url.clone()),
        LinkTarget::Roll(link) => Some(crate::rolls::hint(link)),
        LinkTarget::Lookup(_) => None,
        // A linked world page says what it is before the DM opens it.
        LinkTarget::Note(note) => {
            let index = link_index(ctx?)?;
            match index.resolve_link(note, from) {
                crate::world::Go::Item(at, _) if !index.items[at].summary().is_empty() => {
                    Some(index.items[at].summary().to_owned())
                }
                _ => None,
            }
        }
    }
}

/// Images in the flow of a note, as wide as the text and at most 520 px tall.
fn figures(paths: &[std::path::PathBuf], caption: &str) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(XS)
        .children(paths.iter().map(|path| {
            let name = match caption.trim() {
                "" => "Image".to_owned(),
                caption => caption.to_owned(),
            };
            super::target(
                div().max_w_full().child(
                    img(path.clone())
                        .max_w_full()
                        .max_h(px(520.0))
                        .object_fit(ObjectFit::Contain),
                ),
                name,
            )
        }))
        .into_any_element()
}

/// A compact reader's `Field`/`Value` rows: the field's name in `StatLabel`, its value in `Stat`.
fn fields(rows: &[Vec<String>], width: Option<Pixels>, spacing: Pixels) -> AnyElement {
    let c = colors();
    let available = width.unwrap_or(READER_MEASURE);
    let label = (available * 0.25).clamp(px(100.0), px(145.0));
    let value = (available - label - L).max(px(70.0));
    div()
        .flex()
        .flex_col()
        .gap(spacing)
        .children(rows.iter().map(|row| {
            let field = row
                .first()
                .map_or("", String::as_str)
                .trim()
                .replace('_', " ");
            let shown = parse::display_field_value(row.get(1).map_or("", String::as_str).trim());
            div()
                .flex()
                .flex_row()
                .gap(M)
                .child(div().w(label).flex_none().child(super::text(
                    Type::StatLabel,
                    c.text,
                    field,
                )))
                .child(
                    div()
                        .w(value)
                        .min_w_0()
                        .child(super::text(Type::Stat, c.text, shown)),
                )
        }))
        .into_any_element()
}
