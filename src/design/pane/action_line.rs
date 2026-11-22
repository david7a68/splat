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
    /// Each action's width, as it measures (`Action::natural_width`), in order.
    pub actions: &'a [f32],
    /// The actions stand after the value; else on the rows under the name.
    pub beside: bool,
    /// Each part under the line, by its own measured height.
    pub below: &'a [f32],
    /// The gap the column leaves after the block (in a region); zero in place.
    pub tail: f32,
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
    /// bottom. An action that would pass the row's edge starts the next row, unless it is the
    /// row's first (it then keeps the row, as egui's wrapping layout does).
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
        let mut x = start;
        let mut rects = Vec::with_capacity(self.actions.len());
        for &action in self.actions {
            if x > start + 0.5 && x + action > width + 0.5 {
                x = start;
                y += control + XS;
            }
            rects.push(Rect::from_min_size(pos2(x, y), vec2(action, control)));
            x += action + S;
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
        Laid::new(self.key, vec2(width, y + self.tail), head, texts, slots)
            .with_slot_texts(slot_texts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::components::Action;
    use crate::design::memo::Revised;
    use crate::design::outcome::{Messages, NoPress, PaneFrame};
    use crate::design::pane::{Block, BlockRegion, Body, RegionState};
    use crate::design::test_harness::Themed;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    /// One line of the test list: its words and its actions' words.
    #[derive(Clone, PartialEq)]
    struct Line {
        heading: Option<(&'static str, usize)>,
        title: Option<&'static str>,
        indent: f32,
        name: &'static str,
        value: String,
        actions: Vec<&'static str>,
        beside: bool,
        below: Vec<f32>,
    }

    /// What a pass drew of one line: the name's slot, each action's slot with the rect it drew
    /// at, and the lowest bottom of every part it drew.
    type Drawn = (usize, Rect, Vec<(Rect, Rect)>, f32);

    /// A Content pane of lines, laid out before paint and kept by their inputs: a quiet frame
    /// lays none out.
    struct List {
        lines: Vec<Line>,
        columns: LineColumns,
        /// The laid lines, by the inputs they were laid from.
        kept: Option<(Vec<Line>, LineColumns, f32, Vec<Laid>)>,
        blocks: Revised<Vec<Block>>,
        region: RegionState,
        drawn: Vec<Drawn>,
        /// Each line's height over its name line and its whole height, by the test's own count,
        /// in this pass.
        counted: Vec<(f32, f32)>,
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
                below: vec![30.0],
                ..line(
                    "Rations",
                    "12",
                    vec!["Eat one", "Buy 5", "Share with the party", "Count again"],
                    false,
                )
            },
        ]
    }

    fn widths(ui: &Ui, line: &Line) -> Vec<f32> {
        line.actions
            .iter()
            .map(|words| {
                Action::secondary(words)
                    .density(Density::Read)
                    .natural_width(ui)
            })
            .collect()
    }

    /// The height over the name line by the test's own count: the heading, the title.
    fn over(ui: &Ui, line: &Line, width: f32) -> f32 {
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
    fn expected(ui: &Ui, line: &Line, columns: LineColumns, width: f32) -> f32 {
        let control = Density::Read.control();
        let start = if line.beside {
            columns.name + S + columns.value + S
        } else {
            line.indent
        };
        let mut rows = 0;
        let mut used = 0.0;
        for action in widths(ui, line) {
            if rows == 0 || (used > 0.0 && start + used + action > width + 0.5) {
                rows += 1;
                used = 0.0;
            }
            used += action + S;
        }
        let line_rows = if line.beside { rows.max(1) } else { 1 + rows };
        over(ui, line, width)
            + line_rows as f32 * control
            + (line_rows - 1) as f32 * XS
            + line.below.iter().map(|part| LINE_GAP + part).sum::<f32>()
    }

    fn harness(width: f32, scale: f32) -> Harness<'static, List> {
        let list = List {
            lines: lines(),
            columns: LineColumns {
                name: 120.0,
                value: 48.0,
            },
            kept: None,
            blocks: Revised::new(Vec::new()),
            region: RegionState::new("action_line_test"),
            drawn: Vec::new(),
            counted: Vec::new(),
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
                        list.counted = list
                            .lines
                            .iter()
                            .map(|line| {
                                (
                                    over(ui, line, width),
                                    expected(ui, line, list.columns, width),
                                )
                            })
                            .collect();
                        // The host keeps its layouts by their inputs: a quiet frame lays none.
                        let fresh = list.kept.as_ref().is_none_or(|(lines, columns, at, _)| {
                            *lines != list.lines
                                || *columns != list.columns
                                || at.to_bits() != width.to_bits()
                        });
                        if fresh {
                            let laid: Vec<Laid> = list
                                .lines
                                .iter()
                                .enumerate()
                                .map(|(at, line)| {
                                    let actions = widths(ui, line);
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
                                        below: &line.below,
                                        tail: 0.0,
                                    }
                                    .lay(ui, width)
                                })
                                .collect();
                            list.blocks.set(laid.iter().map(Laid::block).collect());
                            list.kept = Some((list.lines.clone(), list.columns, width, laid));
                        }
                        list.drawn.clear();
                        let List {
                            lines,
                            kept,
                            blocks,
                            region,
                            drawn,
                            ..
                        } = list;
                        let laid = &kept.as_ref().expect("laid above").3;
                        BlockRegion::new("action_line_test", region, blocks).show_laid(
                            ui,
                            |at| &laid[at],
                            |ui, at| {
                                let line = &lines[at];
                                let name = ui.part(slot::NAME, line.name, false).rect;
                                let value = ui.part(slot::VALUE, &line.value, false).rect;
                                let mut lowest = name.bottom().max(value.bottom());
                                let mut actions = Vec::new();
                                for (index, words) in line.actions.iter().enumerate() {
                                    let at = slot::FIRST_ACTION + index;
                                    let rect = ui.in_slot(at, |ui| {
                                        Action::secondary(words)
                                            .density(Density::Read)
                                            .show(ui)
                                            .rect
                                    });
                                    lowest = lowest.max(rect.bottom());
                                    actions.push((ui.slot(at), rect));
                                }
                                for index in 0..line.below.len() {
                                    let at = slot::FIRST_ACTION + line.actions.len() + index;
                                    lowest = lowest.max(ui.slot(at).bottom());
                                }
                                drawn.push((at, ui.slot(slot::NAME), actions, lowest));
                            },
                        );
                    });
                },
                list,
            );
        harness.run_steps(3);
        harness
    }

    /// Issue #683: a line's measured height is its painted height, at every width and scale, in
    /// both layouts (actions beside the value, and under the name), with the app's own actions in
    /// their slots: each action draws exactly at its slot, the block ends at its lowest drawn
    /// part, and the height equals the test's own count. Red: a wrapped row with no `XS`.
    #[test]
    fn a_line_paints_at_its_measured_height_at_every_width() {
        let mut wrapped = 0;
        for scale in [1.0, 1.5] {
            for width in [340.0, 420.0, 560.0, 720.0, 960.0] {
                let harness = harness(width, scale);
                let list = harness.state();
                let laid = &list.kept.as_ref().expect("laid").3;
                assert_eq!(
                    list.drawn.len(),
                    list.lines.len(),
                    "{width}@{scale}: every line draws"
                );
                for (at, name, actions, lowest) in &list.drawn {
                    let size = laid[*at].size();
                    let (over, whole) = list.counted[*at];
                    let top = name.top() - over;
                    assert!(
                        actions
                            .iter()
                            .all(|(slot, drew)| (slot.min - drew.min).length() < 0.5
                                && (slot.size() - drew.size()).length() < 0.5),
                        "{width}@{scale}: line {at}'s actions draw at their slots: {actions:?}"
                    );
                    assert!(
                        (lowest - top - size.y).abs() < 0.5 && (size.y - whole).abs() < 0.5,
                        "{width}@{scale}: line {at} is {} tall, painted to {}, counted {whole}",
                        size.y,
                        lowest - top
                    );
                    wrapped += usize::from(
                        actions
                            .windows(2)
                            .any(|pair| pair[1].0.top() > pair[0].0.top() + 0.5),
                    );
                }
            }
        }
        assert!(wrapped > 0, "some width wraps a line's actions");
    }

    /// A quiet frame lays no line out; a new value, a new action or a wider name column lays the
    /// lines out again, and the pass shows it. Red: a host that lays its lines every frame.
    #[test]
    fn a_quiet_frame_lays_no_line_and_a_change_lays_it_again() {
        let laid = || LINES_LAID.with(std::cell::Cell::get);
        let mut harness = harness(560.0, 1.0);
        let before = laid();
        harness.run_steps(3);
        assert_eq!(laid(), before, "quiet frames lay no line out");
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
    }
}
