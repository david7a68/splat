//! A line of a list whose rows vary in height, as a recipe (layout 11c, issue #683): a name in a
//! column, a value in a column, and actions that wrap. Every part is measured with the fonts
//! before anything paints, so the closed plan holds the line as a measured block (its height,
//! its head and its place known before paint), and a list of uneven rows (the Trackers panel's
//! lines) leaves the panel's own row registration for the plan.
//!
//! The geometry is the Trackers panel's line (`tracker_lines`), decided before paint:
//!
//! - **A heading** (optional): a list group's heading bound to its first line (SH9), its words
//!   with `Heading::above` over them and `Heading::below` under them, as `Block::Section`
//!   measures a heading. It is the block's head (L6).
//! - **A title** (optional): Body words over the line (a tracker's own name over its instances'
//!   lines), then the column's gap.
//! - **The line**: from `indent`, the name cell (`columns.name - indent` wide, one control tall,
//!   the caller's words centred in its height), `S`, the value cell (`columns.value` wide, the
//!   Numeral words at its right edge), then the actions, each as wide as the caller measured it,
//!   `S` apart. `beside`: after the value, wrapping to the first action's edge; else on the rows
//!   under the name, from the indent. Wrapped rows stand `XS` apart, a control tall each: a
//!   wrapped action reads with its own line (FA2).
//! - **Below** (optional): parts under the line that the host draws in slots of their own
//!   measured height (a repeated field's row fault), each after the column's gap.
//! - **The tail**: the gap the column leaves after an item, where the block stands in a region
//!   that stacks blocks with none (as `PlanRow::tail`). Zero in place.
use egui::{Id, Rect, Ui, pos2, vec2};

use super::laid::{Laid, LaidText};
use crate::design::components::group_heading_text;
use crate::design::extras::Heading;
use crate::design::theme::LINE_GAP;
use crate::design::tokens::{Density, S, Type, XS, colors};

/// The list's shared columns: the name column's right edge from the line's start, and the
/// value column's width. The list measures them (the Trackers panel's `columns`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LineColumns {
    pub name: f32,
    pub value: f32,
}

/// A line of a list whose rows vary in height (issue #683).
#[derive(Clone, Copy)]
pub struct ActionLine<'a> {
    /// The line's stable key (a tracker and its instance).
    pub key: Id,
    /// A list group's heading bound to this line (its title and count), or none.
    pub heading: Option<(&'a str, usize)>,
    /// Body words over the line (a tracker's own name over its instances' lines), or none.
    pub title: Option<&'a str>,
    /// Where the name cell starts: an instance's line is set in under its tracker's name.
    pub indent: f32,
    /// The name's words as the caller fitted them to its cell (one line).
    pub name: &'a str,
    /// The value's words (Numeral), at the value cell's right edge.
    pub value: &'a str,
    pub columns: LineColumns,
    /// Each action, in order: its own key (its slot's `Ui` and its control's id follow it, so an
    /// action hidden before it moves no focus) and its width as it measures
    /// (`Action::natural_width`).
    pub actions: &'a [LineAction],
    /// The actions stand after the value; else on the rows under the name.
    pub beside: bool,
    /// Each part under the line, by its own measured height.
    pub below: &'a [f32],
    /// The gap the column leaves after the block (in a region); zero in place.
    pub tail: f32,
}

/// One action of a line: its own key and its measured width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineAction {
    pub key: Id,
    pub width: f32,
}

/// The slots of a laid line, in order: the heading, the name, the value, then one per action
/// (`FIRST_ACTION + at`), then one per part below (`ActionLine::below_slot`).
pub mod slot {
    pub const HEADING: usize = 0;
    pub const NAME: usize = 1;
    pub const VALUE: usize = 2;
    pub const FIRST_ACTION: usize = 3;
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many lines were laid out (the quiet-frame proof: a host keeps its layouts).
    pub static LINES_LAID: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl ActionLine<'_> {
    /// The slot of part `at` below the line.
    pub fn below_slot(&self, at: usize) -> usize {
        slot::FIRST_ACTION + self.actions.len() + at
    }

    /// Where each action stands from the line's top `top`, in a row `width` wide, and the line's
    /// bottom. An action that would pass the row's edge starts the next row. An action wider than
    /// a whole row takes a row of its own, cut to the row's room: its slot ends at the row's edge,
    /// and the action cuts its words there by its own one-line rule (review of PR 876). Every
    /// action stays in the block.
    fn flow(&self, width: f32, top: f32) -> (Vec<Rect>, f32) {
        let control = Density::Read.control();
        if self.actions.is_empty() {
            return (Vec::new(), top + control);
        }
        let (start, top) = if self.beside {
            (self.columns.name + S + self.columns.value + S, top)
        } else {
            (self.indent, top + control + XS)
        };
        let widths: Vec<f32> = self.actions.iter().map(|action| action.width).collect();
        wrap_actions(&widths, (start, width), top, XS)
    }

    /// Each action's width in a row `width` wide, as the line lays it: its own, or a whole row's
    /// room when it is wider (`wrap_actions`). A host that lays an action's words before paint
    /// (`Action::laid_words`) lays them in this width.
    pub fn action_widths(&self, width: f32) -> Vec<f32> {
        self.flow(width, 0.0)
            .0
            .iter()
            .map(|rect| rect.width())
            .collect()
    }

    /// The line laid out in a row `width` wide.
    pub fn lay(&self, ui: &Ui, width: f32) -> Laid {
        #[cfg(any(test, feature = "test-support"))]
        LINES_LAID.with(|laid| laid.set(laid.get() + 1));
        let control = Density::Read.control();
        let mut texts = Vec::new();
        // A part the line does not have (no heading) has an empty slot at the block's start.
        let mut slots =
            vec![Rect::from_min_size(pos2(0.0, 0.0), vec2(0.0, 0.0)); slot::FIRST_ACTION];
        let mut slot_texts = vec![None; slot::FIRST_ACTION];
        let mut y = 0.0;
        let mut head = None;
        if let Some((title, count)) = self.heading {
            let group = Heading::Group { title, count };
            let galley =
                ui.fonts_mut(|fonts| fonts.layout_job(group_heading_text(ui, title, count)));
            let height = group.above() + galley.size().y + group.below();
            texts.push(LaidText {
                at: pos2(0.0, group.above()),
                galley,
            });
            slots[slot::HEADING] = Rect::from_min_size(pos2(0.0, 0.0), vec2(width, height));
            y = height;
            head = Some(height);
        }
        if let Some(title) = self.title {
            let galley = ui.fonts_mut(|fonts| {
                fonts.layout(title.to_owned(), Type::Body.font(ui), colors().text, width)
            });
            let height = galley.size().y;
            texts.push(LaidText {
                at: pos2(0.0, y),
                galley,
            });
            y += height + LINE_GAP;
        }
        // The name, in its cell from the indent, the caller's words centred in its height.
        let name = ui.fonts_mut(|fonts| {
            fonts.layout_no_wrap(self.name.to_owned(), Type::Body.font(ui), colors().text)
        });
        slots[slot::NAME] = Rect::from_min_size(
            pos2(self.indent, y),
            vec2((self.columns.name - self.indent).max(0.0), control),
        );
        slot_texts[slot::NAME] = Some(name);
        // The value, at its cell's right edge.
        let value = ui.fonts_mut(|fonts| {
            fonts.layout_no_wrap(self.value.to_owned(), Type::Numeral.font(ui), colors().text)
        });
        let right = self.columns.name + S + self.columns.value;
        slots[slot::VALUE] = Rect::from_min_size(
            pos2(right - value.size().x, y),
            vec2(value.size().x, control),
        );
        slot_texts[slot::VALUE] = Some(value);
        let (actions, bottom) = self.flow(width, y);
        slot_texts.extend(actions.iter().map(|_| None));
        slots.extend(actions);
        y = bottom;
        for &part in self.below {
            y += LINE_GAP;
            slots.push(Rect::from_min_size(pos2(0.0, y), vec2(width, part)));
            slot_texts.push(None);
            y += part;
        }
        // Each action's slot is keyed by its action, so its control keeps its id and its focus
        // when an action before it is hidden.
        let mut slot_keys = vec![None; slot::FIRST_ACTION];
        slot_keys.extend(self.actions.iter().map(|action| Some(action.key)));
        slot_keys.resize(slot_texts.len(), None);
        Laid::new(self.key, vec2(width, y + self.tail), head, texts, slots)
            .with_slot_texts(slot_texts)
            .with_slot_keys(slot_keys)
    }
}

/// The wrapping rule of a row of actions (`ActionLine`, and the Trackers page's Due rows, L7):
/// each action as wide as `widths` says, `S` apart from `start`; an action that would pass
/// `width` starts a new row unless it is a row's first; rows `row_gap` apart, a control tall
/// each; an action wider than a whole row (`width - start`) takes a row of its own, cut to it.
/// Returns each action's rect and the bottom of its last row (`top` with no actions).
pub(in crate::design) fn wrap_actions(
    widths: &[f32],
    (start, width): (f32, f32),
    top: f32,
    row_gap: f32,
) -> (Vec<Rect>, f32) {
    let control = Density::Read.control();
    if widths.is_empty() {
        return (Vec::new(), top);
    }
    let room = (width - start).max(0.0);
    let (mut x, mut y) = (start, top);
    let mut rects = Vec::with_capacity(widths.len());
    for &action in widths {
        if x > start + 0.5 && x + action > width + 0.5 {
            x = start;
            y += control + row_gap;
        }
        let shown = action.min(room);
        rects.push(Rect::from_min_size(pos2(x, y), vec2(shown, control)));
        x += shown + S;
    }
    (rects, y + control)
}

/// For a caller's tests (the Trackers page's, L7): the narrowest row width at which `line`'s
/// actions take one row. One point narrower, the last of them wraps.
#[cfg(any(test, feature = "test-support"))]
pub fn one_row_width(line: &ActionLine<'_>) -> f32 {
    let start = if line.beside {
        line.columns.name + S + line.columns.value + S
    } else {
        line.indent
    };
    let widths: f32 = line.actions.iter().map(|action| action.width).sum();
    start + widths + S * line.actions.len().saturating_sub(1) as f32
}

/// For a caller's tests: whether a line drew inside its measured `block` (its rect, as
/// `draw_laid` or a region placed it): every rect it painted lies in the block, and the lowest
/// painted bottom with the block's `tail` is the block's end. Each painted rect is read from what
/// was drawn (an action's response, a part's painted words), never from its slot.
#[cfg(any(test, feature = "test-support"))]
pub fn painted_in_block(block: Rect, tail: f32, painted: &[Rect]) -> Result<(), String> {
    if let Some(out) = painted
        .iter()
        .find(|rect| !block.expand(0.5).contains_rect(**rect))
    {
        return Err(format!("{out:?} is painted outside its block {block:?}"));
    }
    let lowest = painted
        .iter()
        .map(|rect| rect.bottom())
        .fold(block.top(), f32::max);
    if (lowest + tail - block.bottom()).abs() > 0.5 {
        return Err(format!(
            "the lowest painted bottom {lowest} with the tail {tail} does not end the block \
             {block:?}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::components::{ACTION_WORDS_LAID, Action};
    use crate::design::memo::{Changed, Revised, Revision};
    use crate::design::outcome::{Messages, NoPress, PaneFrame};
    use crate::design::pane::{Block, BlockRegion, Body, RegionState, draw_laid};
    use crate::design::test_harness::Themed;
    use egui::Galley;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use std::sync::Arc;

    thread_local! {
        /// Every measure of text the test host makes itself: each action's width and words, each
        /// part's words, its own count of each line.
        static HOST_MEASURES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }

    fn measured() {
        HOST_MEASURES.with(|count| count.set(count.get() + 1));
    }

    /// Every text layout from the host's entry through its paint: the recipe's, the host's own,
    /// and every word an action laid out to paint itself.
    fn work() -> (usize, usize, usize) {
        (
            LINES_LAID.with(std::cell::Cell::get),
            HOST_MEASURES.with(std::cell::Cell::get),
            ACTION_WORDS_LAID.with(std::cell::Cell::get),
        )
    }

    /// One line of the test list: its words, its actions' words, the words of its parts below.
    #[derive(Clone, PartialEq)]
    struct Line {
        heading: Option<(&'static str, usize)>,
        title: Option<&'static str>,
        indent: f32,
        name: &'static str,
        value: String,
        actions: Vec<&'static str>,
        beside: bool,
        below: Vec<&'static str>,
    }

    /// What a pass drew of one line: the name's slot, each action's slot with the rect it drew
    /// at, and the lowest bottom of every part it painted.
    type Drawn = (usize, Rect, Vec<(Rect, Rect)>, f32);

    /// Everything the host's laid lines are made from, as one key (`lay_lines` reads only this
    /// and the lines it names): the lines' revision, the columns, the width, the tail, the font
    /// generation, the scale, the font atlas's epoch and the theme. The host compares keys, never
    /// lines; a missing input cannot be left out of one and not the other.
    #[derive(Clone, Copy, PartialEq)]
    struct LineInputs {
        lines: Revision,
        columns: LineColumns,
        width: f32,
        tail: f32,
        fonts: u64,
        scale: f32,
        atlas: u64,
        dark: bool,
    }

    impl LineInputs {
        fn of(ui: &Ui, list: &List, width: f32) -> Self {
            Self {
                lines: list.lines.revision(),
                columns: list.columns,
                width,
                tail: list.tail,
                fonts: crate::design::theme::font_generation(ui.ctx()),
                scale: ui.ctx().pixels_per_point(),
                atlas: crate::design::theme::atlas_epoch(ui.ctx()),
                dark: ui.visuals().dark_mode,
            }
        }
    }

    /// The host's laid lines: their key, each laid line, each action's words and each part's
    /// words as laid, and each line's height over its name and whole height by the test's own
    /// count.
    struct Kept {
        inputs: LineInputs,
        laid: Vec<Laid>,
        words: Vec<Vec<Arc<Galley>>>,
        below: Vec<Vec<Arc<Galley>>>,
        counted: Vec<(f32, f32)>,
    }

    /// A pane of lines, laid out before paint and kept by their key: a quiet frame lays out and
    /// measures nothing, from its entry through its paint. In the region of a `Content` pane,
    /// or in place.
    struct List {
        lines: Revised<Vec<Line>>,
        columns: LineColumns,
        tail: f32,
        in_place: bool,
        kept: Option<Kept>,
        blocks: Revised<Vec<Block>>,
        region: RegionState,
        drawn: Vec<Drawn>,
    }

    impl List {
        /// Change the lines, and their revision with them.
        fn edit(&mut self, change: impl FnOnce(&mut Vec<Line>)) {
            self.lines.revise(|lines| {
                change(lines);
                Changed::Yes
            });
        }
    }

    fn lines() -> Vec<Line> {
        let line = |name, value: &str, actions: Vec<&'static str>, beside| Line {
            heading: None,
            title: None,
            indent: 0.0,
            name,
            value: value.to_owned(),
            actions,
            beside,
            below: Vec::new(),
        };
        vec![
            Line {
                heading: Some(("Pinned", 0)),
                ..line("Torches", "3", vec!["−1", "+1", "Light one"], true)
            },
            Line {
                title: Some("Inspiration"),
                indent: crate::design::tokens::L,
                ..line("Brannoch", "1 / 1", vec!["Use", "Give"], true)
            },
            Line {
                indent: crate::design::tokens::L,
                ..line("Iselle", "0 / 1", vec!["Use", "Give"], true)
            },
            Line {
                below: vec![
                    "Two fields count Rations once: the line that leaves one out says so here.",
                ],
                ..line(
                    "Rations",
                    "12",
                    vec!["Eat one", "Buy 5", "Share with the party", "Count again"],
                    false,
                )
            },
        ]
    }

    fn action(words: &str) -> Action<'_> {
        Action::secondary(words).density(Density::Read)
    }

    fn actions(ui: &Ui, at: usize, line: &Line) -> Vec<LineAction> {
        line.actions
            .iter()
            .map(|words| {
                measured();
                LineAction {
                    key: Id::new(("action_line_test", at, *words)),
                    width: action(words).natural_width(ui),
                }
            })
            .collect()
    }

    /// A part's words laid out as the test draws them (Small, wrapped in the line's width).
    fn part(ui: &Ui, words: &str, width: f32) -> Arc<Galley> {
        measured();
        ui.fonts_mut(|fonts| {
            fonts.layout(
                words.to_owned(),
                Type::Small.font(ui),
                colors().muted,
                width,
            )
        })
    }

    /// The height over the name line by the test's own count: the heading, the title.
    fn over(ui: &Ui, line: &Line, width: f32) -> f32 {
        measured();
        let mut height = 0.0;
        if let Some((title, count)) = line.heading {
            let group = Heading::Group { title, count };
            let words = ui
                .fonts_mut(|fonts| fonts.layout_job(group_heading_text(ui, title, count)))
                .size()
                .y;
            height += group.above() + words + group.below();
        }
        if let Some(title) = line.title {
            height +=
                ui.fonts_mut(|fonts| {
                    fonts.layout(title.to_owned(), Type::Body.font(ui), colors().text, width)
                })
                .size()
                .y + LINE_GAP;
        }
        height
    }

    /// The line's whole height by the test's own count: over it, the rows its actions take (an
    /// action starts a new row when it would pass the edge and is not a row's first), the parts
    /// below.
    fn expected(ui: &Ui, (at, line): (usize, &Line), columns: LineColumns, width: f32) -> f32 {
        let control = Density::Read.control();
        let start = if line.beside {
            columns.name + S + columns.value + S
        } else {
            line.indent
        };
        let mut rows = 0;
        let mut used = 0.0;
        for action in actions(ui, at, line) {
            if rows == 0 || (used > 0.0 && start + used + action.width > width + 0.5) {
                rows += 1;
                used = 0.0;
            }
            used += action.width + S;
        }
        let line_rows = if line.beside { rows.max(1) } else { 1 + rows };
        over(ui, line, width)
            + line_rows as f32 * control
            + (line_rows - 1) as f32 * XS
            + line
                .below
                .iter()
                .map(|words| LINE_GAP + part(ui, words, width).size().y)
                .sum::<f32>()
    }

    /// Lay the host's lines from their key alone (`ui` is only the fonts' handle): each line by
    /// the recipe, each action's words in the width the recipe gives it, each part's words.
    fn lay_lines(ui: &Ui, inputs: LineInputs, lines: &[Line]) -> Kept {
        let mut kept = Kept {
            inputs,
            laid: Vec::new(),
            words: Vec::new(),
            below: Vec::new(),
            counted: Vec::new(),
        };
        for (at, line) in lines.iter().enumerate() {
            let actions = actions(ui, at, line);
            let below: Vec<Arc<Galley>> = line
                .below
                .iter()
                .map(|words| part(ui, words, inputs.width))
                .collect();
            let heights: Vec<f32> = below.iter().map(|galley| galley.size().y).collect();
            let recipe = ActionLine {
                key: Id::new(("action_line_test", at)),
                heading: line.heading,
                title: line.title,
                indent: line.indent,
                name: line.name,
                value: &line.value,
                columns: inputs.columns,
                actions: &actions,
                beside: line.beside,
                below: &heights,
                tail: inputs.tail,
            };
            kept.words.push(
                line.actions
                    .iter()
                    .zip(recipe.action_widths(inputs.width))
                    .map(|(words, width)| {
                        measured();
                        action(words).laid_words(ui, width)
                    })
                    .collect(),
            );
            kept.laid.push(recipe.lay(ui, inputs.width));
            kept.below.push(below);
            kept.counted.push((
                over(ui, line, inputs.width),
                expected(ui, (at, line), inputs.columns, inputs.width) + inputs.tail,
            ));
        }
        kept
    }

    /// Draw one laid line's parts in its slots from what was laid: its name and value, each
    /// action with its laid words (in its keyed slot), each part's laid words. Lays out no text.
    fn draw(ui: &mut crate::design::LaidUi<'_>, (at, line): (usize, &Line), kept: &Kept) -> Drawn {
        let name = ui.part(slot::NAME, line.name, false).rect;
        let value = ui.part(slot::VALUE, &line.value, false).rect;
        let mut lowest = name.bottom().max(value.bottom());
        let mut drew = Vec::new();
        for (index, words) in line.actions.iter().enumerate() {
            let at_slot = slot::FIRST_ACTION + index;
            let laid = Arc::clone(&kept.words[at][index]);
            let rect = ui.in_slot(at_slot, |ui| action(words).laid(laid).show(ui).rect);
            lowest = lowest.max(rect.bottom());
            drew.push((ui.slot(at_slot), rect));
        }
        for (index, galley) in kept.below[at].iter().enumerate() {
            let at_slot = slot::FIRST_ACTION + line.actions.len() + index;
            let galley = Arc::clone(galley);
            let rect = ui.in_slot(at_slot, |ui| {
                ui.add(egui::Label::new(egui::WidgetText::Galley(galley)))
                    .rect
            });
            lowest = lowest.max(rect.bottom());
        }
        (at, ui.slot(slot::NAME), drew, lowest)
    }

    fn harness(width: f32, scale: f32, (tail, in_place): (f32, bool)) -> Harness<'static, List> {
        let list = List {
            lines: Revised::new(lines()),
            columns: LineColumns {
                name: 120.0,
                value: 48.0,
            },
            tail,
            in_place,
            kept: None,
            blocks: Revised::new(Vec::new()),
            region: RegionState::new("action_line_test"),
            drawn: Vec::new(),
        };
        let mut harness = Harness::builder()
            // A logical width: the Trackers panel is never narrower than `PANEL_MIN_WIDTH`.
            .with_size(vec2(width, 900.0))
            .with_pixels_per_point(scale)
            .themed_ui_state(
                |ui, list: &mut List| {
                    let frame = PaneFrame::new(ui);
                    let messages =
                        Messages::<NoPress>::new(&frame, "action_line_test", ui.available_width());
                    let _ = messages.pane(ui, Body::Content, |ui| {
                        let width = ui.available_width();
                        // The host keeps its layouts by their key: a quiet frame compares keys,
                        // lays out and measures nothing.
                        let inputs = LineInputs::of(ui, list, width);
                        if list.kept.as_ref().is_none_or(|kept| kept.inputs != inputs) {
                            let kept = lay_lines(ui, inputs, &list.lines);
                            list.blocks.set(kept.laid.iter().map(Laid::block).collect());
                            list.kept = Some(kept);
                        }
                        list.drawn.clear();
                        let List {
                            lines,
                            kept,
                            blocks,
                            region,
                            drawn,
                            in_place,
                            ..
                        } = list;
                        let kept = kept.as_ref().expect("laid above");
                        if *in_place {
                            for (at, line) in lines.iter().enumerate() {
                                let _ = draw_laid(ui, &kept.laid[at], |ui| {
                                    drawn.push(draw(ui, (at, line), kept));
                                });
                            }
                        } else {
                            BlockRegion::new("action_line_test", region, blocks).show_laid(
                                ui,
                                |at| &kept.laid[at],
                                |ui, at| drawn.push(draw(ui, (at, &lines[at]), kept)),
                            );
                        }
                    });
                },
                list,
            );
        harness.run_steps(3);
        harness
    }

    /// Issue #683 and the review of PR 876: a line's measured height is its painted height, at
    /// every width and scale, in both layouts, with a tail and without, in the region and in
    /// place, with the app's own actions in their slots and real words painted under a line:
    /// each action draws exactly at its slot, the block ends at its lowest painted part (and its
    /// tail), and the height equals the test's own count. Red: a wrapped row with no `XS`; a
    /// part's height that is not its words'.
    #[test]
    fn a_line_paints_at_its_measured_height_at_every_width() {
        let mut wrapped = 0;
        for (tail, in_place) in [
            (0.0, false),
            (LINE_GAP, false),
            (0.0, true),
            (LINE_GAP, true),
        ] {
            for scale in [1.0, 1.5] {
                for width in [340.0, 420.0, 560.0, 720.0, 960.0] {
                    let harness = harness(width, scale, (tail, in_place));
                    let list = harness.state();
                    let kept = list.kept.as_ref().expect("laid");
                    let case = format!("{width}@{scale}, tail {tail}, in place {in_place}");
                    assert_eq!(
                        list.drawn.len(),
                        list.lines.len(),
                        "{case}: every line draws"
                    );
                    for (at, name, actions, lowest) in &list.drawn {
                        let size = kept.laid[*at].size();
                        let (over, whole) = kept.counted[*at];
                        let top = name.top() - over;
                        assert!(
                            actions
                                .iter()
                                .all(|(slot, drew)| (slot.min - drew.min).length() < 0.5
                                    && (slot.size() - drew.size()).length() < 0.5),
                            "{case}: line {at}'s actions draw at their slots: {actions:?}"
                        );
                        assert!(
                            (lowest + tail - top - size.y).abs() < 0.5
                                && (size.y - whole).abs() < 0.5,
                            "{case}: line {at} is {} tall, painted to {}, counted {whole}",
                            size.y,
                            lowest + tail - top
                        );
                        wrapped += usize::from(
                            actions
                                .windows(2)
                                .any(|pair| pair[1].0.top() > pair[0].0.top() + 0.5),
                        );
                    }
                }
            }
        }
        assert!(wrapped > 0, "some width wraps a line's actions");
    }
    /// Review of PR 876 (P2): an action wider than a whole row of actions takes a row of its
    /// own, cut to the row's room, so every action stays in the block (width 340, columns 120 and
    /// 48, actions 40, 40 and 200: the third ended at 384). Red: the wrap that let it run on.
    #[test]
    fn a_wide_action_stays_in_its_block() {
        let mut harness = Harness::new_ui_state(
            |ui, slots: &mut Vec<(Rect, Rect)>| {
                let actions = [40.0_f32, 40.0, 200.0].map(|width| LineAction {
                    key: Id::new(("wide", width.to_bits())),
                    width,
                });
                let laid = ActionLine {
                    key: Id::new("wide"),
                    heading: None,
                    title: None,
                    indent: 0.0,
                    name: "Torches",
                    value: "3",
                    columns: LineColumns {
                        name: 120.0,
                        value: 48.0,
                    },
                    actions: &actions,
                    beside: true,
                    below: &[],
                    tail: 0.0,
                }
                .lay(ui, 340.0);
                slots.clear();
                let block = draw_laid(ui, &laid, |ui| {
                    for at in 0..3 {
                        slots.push((ui.slot(slot::FIRST_ACTION + at), Rect::NOTHING));
                    }
                });
                for slot in slots.iter_mut() {
                    slot.1 = block;
                }
            },
            Vec::new(),
        );
        harness.run_steps(2);
        let slots = harness.state();
        let block = slots[0].1;
        assert!(
            slots
                .iter()
                .all(|(slot, _)| block.expand(0.5).contains_rect(*slot))
                && slots[2].0.top() > slots[1].0.bottom()
                && (slots[2].0.right() - block.left() - 340.0).abs() < 0.5,
            "the 200 action takes a row of its own, cut at the block's edge: {slots:?}"
        );
    }

    /// A quiet frame lays out and measures no text, from the host's entry through its paint:
    /// no line by the recipe, no measure by the host, no word by an action (each paints its laid
    /// words); the host compares its key, never its lines. A new value, a new action, a wider
    /// name column, a new font atlas, another theme and another scale each lay the lines out
    /// again, and the pass shows it. Red: actions that lay their own words; a key without the
    /// atlas; a host that lays its lines every frame.
    #[test]
    fn a_quiet_frame_lays_no_line_and_a_change_lays_it_again() {
        let laid = || LINES_LAID.with(std::cell::Cell::get);
        let mut harness = harness(560.0, 1.0, (0.0, false));
        let before = work();
        harness.run_steps(3);
        assert_eq!(
            work(),
            before,
            "quiet frames lay out and measure no text (recipe, host, actions)"
        );
        let before = laid();
        harness
            .state_mut()
            .edit(|lines| lines[0].value = "4".to_owned());
        harness.step();
        assert!(
            laid() > before && harness.query_by_label("4").is_some(),
            "a new value is laid out and shown in the same pass"
        );
        let before = laid();
        harness
            .state_mut()
            .edit(|lines| lines[0].actions.push("Douse all"));
        harness.step();
        assert!(
            laid() > before && harness.query_by_label("Douse all").is_some(),
            "a new action is laid out and shown"
        );
        let before = laid();
        let name = harness.state().drawn[0].1;
        harness.state_mut().columns.name += 40.0;
        harness.step();
        assert!(
            laid() > before && harness.state().drawn[0].1.width() > name.width() + 39.5,
            "a wider name column lays the lines out again"
        );
        // A new font atlas at the same width, scale and theme: egui makes its atlas again for
        // a new text alpha mode, and again when it comes back.
        let before = laid();
        let alpha = harness.ctx.style().visuals.text_alpha_from_coverage;
        harness.ctx.all_styles_mut(|style| {
            style.visuals.text_alpha_from_coverage = egui::epaint::AlphaFromCoverage::Linear;
        });
        harness.step();
        harness
            .ctx
            .all_styles_mut(|style| style.visuals.text_alpha_from_coverage = alpha);
        harness.step();
        assert!(laid() > before, "a new atlas lays the lines out again");
        let before = laid();
        harness.ctx.set_visuals(egui::Visuals::light());
        harness.step();
        assert!(laid() > before, "another theme lays the lines out again");
        let before = laid();
        harness.ctx.set_pixels_per_point(1.5);
        harness.step();
        assert!(laid() > before, "another scale lays the lines out again");
    }

    /// What the threshold case drew: the tail, the one-row width, and at each width its block,
    /// every rect it painted, and each action's top.
    type Threshold = (f32, Option<f32>, Vec<(Rect, Vec<Rect>, Vec<f32>)>);

    /// Sol 2-018: a real Trackers line at its wrap threshold. With a long action ("Score for Old
    /// Guard"), a part below it and a nonzero tail, one point wider than its one-row width the
    /// actions take one row, and one point narrower the last wraps; at both, everything painted
    /// (the actions' responses, the part's painted words) lies in the measured block, and the
    /// lowest painted bottom with the tail ends it. Red: a row that wraps one point early.
    #[test]
    fn a_long_action_at_its_wrap_threshold_stays_in_its_block() {
        const WORDS: [&str; 3] = ["Burn a torch", "Crowd loves it", "Score for Old Guard"];
        const BELOW: &str =
            "Two fields count Old Guard once: the line that leaves one out says so here.";
        let mut harness = Harness::builder()
            .with_size(vec2(1200.0, 400.0))
            .themed_ui_state(
                |ui, state: &mut Threshold| {
                    let actions: Vec<LineAction> = WORDS
                        .iter()
                        .map(|words| LineAction {
                            key: Id::new(("threshold", *words)),
                            width: Action::secondary(words)
                                .density(Density::Read)
                                .natural_width(ui),
                        })
                        .collect();
                    let line = ActionLine {
                        key: Id::new("threshold"),
                        heading: None,
                        title: None,
                        indent: 0.0,
                        name: "Old Guard",
                        value: "2",
                        columns: LineColumns {
                            name: 120.0,
                            value: 48.0,
                        },
                        actions: &actions,
                        beside: true,
                        below: &[],
                        tail: LINE_GAP,
                    };
                    let threshold = one_row_width(&line);
                    state.1 = Some(threshold);
                    state.2.clear();
                    for width in [threshold + 1.0, threshold - 1.0] {
                        // The part's words, measured as the caller does: wrapped in the line's
                        // width.
                        let below = [ui
                            .fonts_mut(|fonts| {
                                fonts.layout(
                                    BELOW.to_owned(),
                                    Type::Small.font(ui),
                                    colors().muted,
                                    width,
                                )
                            })
                            .size()
                            .y];
                        let line = ActionLine {
                            below: &below,
                            ..line
                        };
                        let laid = line.lay(ui, width);
                        let mut painted = Vec::new();
                        let mut tops = Vec::new();
                        let block = draw_laid(ui, &laid, |ui| {
                            painted.push(ui.part(slot::NAME, line.name, false).rect);
                            painted.push(ui.part(slot::VALUE, line.value, false).rect);
                            for (at, words) in WORDS.iter().enumerate() {
                                let rect = ui.in_slot(slot::FIRST_ACTION + at, |ui| {
                                    Action::secondary(words)
                                        .density(Density::Read)
                                        .show(ui)
                                        .rect
                                });
                                tops.push(rect.top());
                                painted.push(rect);
                            }
                            painted.push(ui.in_slot(line.below_slot(0), |ui| {
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, BELOW).color(colors().muted),
                                    )
                                    .wrap(),
                                )
                                .rect
                            }));
                        });
                        state.2.push((block, painted, tops));
                    }
                    state.0 = LINE_GAP;
                },
                (0.0, None, Vec::new()),
            );
        harness.run_steps(2);
        let (tail, threshold, drawn) = harness.state();
        let threshold = threshold.expect("measured");
        assert_eq!(drawn.len(), 2);
        let ((wide, wide_painted, wide_tops), (narrow, narrow_painted, narrow_tops)) =
            (&drawn[0], &drawn[1]);
        assert!(
            wide_tops.iter().all(|top| (top - wide_tops[0]).abs() < 0.5)
                && narrow_tops[2] > narrow_tops[0] + 0.5,
            "at {} one row, at {} the last action wraps: {wide_tops:?} {narrow_tops:?}",
            threshold + 1.0,
            threshold - 1.0
        );
        for (block, painted) in [(wide, wide_painted), (narrow, narrow_painted)] {
            if let Err(fault) = painted_in_block(*block, *tail, painted) {
                panic!("at width {}: {fault}", block.width());
            }
        }
        assert!(
            (narrow.height() - wide.height() - Density::Read.control() - XS).abs() < 0.5,
            "the wrapped row adds one control and XS: {wide:?} {narrow:?}"
        );
    }

    /// Review of PR 876 (P2): each action's slot is keyed by its action, so hiding an action
    /// before the focused one keeps the focus on it, never hands it to another. Red: slots keyed
    /// by their place.
    #[test]
    fn hiding_an_earlier_action_keeps_the_focus_on_its_own() {
        let mut harness = harness(560.0, 1.0, (0.0, false));
        harness.get_by_label("Light one").focus();
        harness.run_steps(2);
        assert!(harness.get_by_label("Light one").is_focused());
        harness.state_mut().edit(|lines| {
            lines[0].actions.remove(0);
        });
        harness.run_steps(2);
        assert!(
            harness.get_by_label("Light one").is_focused()
                && !harness.get_by_label("+1").is_focused(),
            "the focus stays on Light one"
        );
    }
}
