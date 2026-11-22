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
        let (start, mut y) = if self.beside {
            (self.columns.name + S + self.columns.value + S, top)
        } else {
            (self.indent, top + control + XS)
        };
        let room = (width - start).max(0.0);
        let mut x = start;
        let mut rects = Vec::with_capacity(self.actions.len());
        for action in self.actions {
            if x > start + 0.5 && x + action.width > width + 0.5 {
                x = start;
                y += control + XS;
            }
            let shown = action.width.min(room);
            rects.push(Rect::from_min_size(pos2(x, y), vec2(shown, control)));
            x += shown + S;
        }
        (rects, y + control)
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
    use crate::design::components::Action;
    use crate::design::memo::Revised;
    use crate::design::outcome::{Messages, NoPress, PaneFrame};
    use crate::design::pane::{Block, BlockRegion, Body, RegionState, draw_laid};
    use crate::design::test_harness::Themed;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    thread_local! {
        /// Every measure of text the test host makes from its entry through its paint: each
        /// action's width, each part's height, its own count of each line.
        static HOST_MEASURES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }

    fn measured() {
        HOST_MEASURES.with(|count| count.set(count.get() + 1));
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

    /// Everything a laid line is made from (the host's key): the lines, the columns, the width,
    /// the tail, the font generation, the scale, the font atlas's epoch and the theme.
    type Inputs = (Vec<Line>, LineColumns, u32, u32, u64, u32, u64, bool);

    /// The host's kept layouts: their inputs, each laid line, and each line's height over its
    /// name and whole height by the test's own count.
    type Kept = (Inputs, Vec<Laid>, Vec<(f32, f32)>);

    /// A pane of lines, laid out before paint and kept by every input: a quiet frame lays none
    /// out and measures nothing. In the region of a `Content` pane, or in place.
    struct List {
        lines: Vec<Line>,
        columns: LineColumns,
        tail: f32,
        in_place: bool,
        kept: Option<Kept>,
        blocks: Revised<Vec<Block>>,
        region: RegionState,
        drawn: Vec<Drawn>,
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

    fn actions(ui: &Ui, at: usize, line: &Line) -> Vec<LineAction> {
        line.actions
            .iter()
            .map(|words| {
                measured();
                LineAction {
                    key: Id::new(("action_line_test", at, *words)),
                    width: Action::secondary(words)
                        .density(Density::Read)
                        .natural_width(ui),
                }
            })
            .collect()
    }

    /// A part's words laid out as the test draws them (Small, wrapped in the line's width).
    fn part(ui: &Ui, words: &str, width: f32) -> std::sync::Arc<egui::Galley> {
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
    /// below, the tail.
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

    /// The host's key: every input of its laid lines.
    fn inputs(ui: &Ui, list: &List, width: f32) -> Inputs {
        (
            list.lines.clone(),
            list.columns,
            width.to_bits(),
            list.tail.to_bits(),
            crate::design::theme::font_generation(ui.ctx()),
            ui.ctx().pixels_per_point().to_bits(),
            crate::design::theme::atlas_epoch(ui.ctx()),
            ui.visuals().dark_mode,
        )
    }

    /// Draw one laid line's parts in its slots: its name and value, each action (in its keyed
    /// slot), each part below as wrapped words. Returns what it painted.
    fn draw(ui: &mut crate::design::LaidUi<'_>, (at, line): (usize, &Line)) -> Drawn {
        let name = ui.part(slot::NAME, line.name, false).rect;
        let value = ui.part(slot::VALUE, &line.value, false).rect;
        let mut lowest = name.bottom().max(value.bottom());
        let mut drew = Vec::new();
        for (index, words) in line.actions.iter().enumerate() {
            let at = slot::FIRST_ACTION + index;
            let rect = ui.in_slot(at, |ui| {
                Action::secondary(words)
                    .density(Density::Read)
                    .show(ui)
                    .rect
            });
            lowest = lowest.max(rect.bottom());
            drew.push((ui.slot(at), rect));
        }
        for (index, words) in line.below.iter().enumerate() {
            let at = slot::FIRST_ACTION + line.actions.len() + index;
            let rect = ui.in_slot(at, |ui| {
                ui.add(egui::Label::new(Type::Small.rich(ui, *words).color(colors().muted)).wrap())
                    .rect
            });
            lowest = lowest.max(rect.bottom());
        }
        (at, ui.slot(slot::NAME), drew, lowest)
    }

    fn harness(width: f32, scale: f32, (tail, in_place): (f32, bool)) -> Harness<'static, List> {
        let list = List {
            lines: lines(),
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
                        // The host keeps its layouts by every input: a quiet frame lays none out
                        // and measures nothing.
                        let now = inputs(ui, list, width);
                        if list.kept.as_ref().is_none_or(|(at, ..)| *at != now) {
                            let laid: Vec<Laid> = list
                                .lines
                                .iter()
                                .enumerate()
                                .map(|(at, line)| {
                                    let actions = actions(ui, at, line);
                                    let below: Vec<f32> = line
                                        .below
                                        .iter()
                                        .map(|words| part(ui, words, width).size().y)
                                        .collect();
                                    ActionLine {
                                        key: Id::new(("action_line_test", at)),
                                        heading: line.heading,
                                        title: line.title,
                                        indent: line.indent,
                                        name: line.name,
                                        value: &line.value,
                                        columns: list.columns,
                                        actions: &actions,
                                        beside: line.beside,
                                        below: &below,
                                        tail: list.tail,
                                    }
                                    .lay(ui, width)
                                })
                                .collect();
                            let counted = list
                                .lines
                                .iter()
                                .enumerate()
                                .map(|line| {
                                    (
                                        over(ui, line.1, width),
                                        expected(ui, line, list.columns, width) + list.tail,
                                    )
                                })
                                .collect();
                            list.blocks.set(laid.iter().map(Laid::block).collect());
                            list.kept = Some((now, laid, counted));
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
                        let laid = &kept.as_ref().expect("laid above").1;
                        if *in_place {
                            for (at, line) in lines.iter().enumerate() {
                                let _ = draw_laid(ui, &laid[at], |ui| {
                                    drawn.push(draw(ui, (at, line)));
                                });
                            }
                        } else {
                            BlockRegion::new("action_line_test", region, blocks).show_laid(
                                ui,
                                |at| &laid[at],
                                |ui, at| drawn.push(draw(ui, (at, &lines[at]))),
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
                    let (_, laid, counted) = list.kept.as_ref().expect("laid");
                    let case = format!("{width}@{scale}, tail {tail}, in place {in_place}");
                    assert_eq!(
                        list.drawn.len(),
                        list.lines.len(),
                        "{case}: every line draws"
                    );
                    for (at, name, actions, lowest) in &list.drawn {
                        let size = laid[*at].size();
                        let (over, whole) = counted[*at];
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

    /// A quiet frame lays no line out and measures nothing, from the host's entry through its
    /// paint; a new value, a new action, a wider name column, a new font atlas, another theme and
    /// another scale each lay the lines out again, and the pass shows it. Red: a host that lays
    /// its lines every frame; a key without the atlas.
    #[test]
    fn a_quiet_frame_lays_no_line_and_a_change_lays_it_again() {
        let laid = || LINES_LAID.with(std::cell::Cell::get);
        let host = || HOST_MEASURES.with(std::cell::Cell::get);
        let mut harness = harness(560.0, 1.0, (0.0, false));
        let (before, measures) = (laid(), host());
        harness.run_steps(3);
        assert_eq!(
            (laid(), host()),
            (before, measures),
            "quiet frames lay no line out and measure nothing"
        );
        harness.state_mut().lines[0].value = "4".to_owned();
        harness.step();
        assert!(
            laid() > before && harness.query_by_label("4").is_some(),
            "a new value is laid out and shown in the same pass"
        );
        let before = laid();
        harness.state_mut().lines[0].actions.push("Douse all");
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
        harness.state_mut().lines[0].actions.remove(0);
        harness.run_steps(2);
        assert!(
            harness.get_by_label("Light one").is_focused()
                && !harness.get_by_label("+1").is_focused(),
            "the focus stays on Light one"
        );
    }
}
