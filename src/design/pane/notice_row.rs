//! A notice's row in a list, as a recipe (layout 11c scope C): a notice that came due, with the
//! steps it asks for. Every part is measured with the fonts before anything paints, so the
//! closed plan holds the row as a measured block, and the Trackers panel's Due rows leave the
//! panel's own row registration.
//!
//! The geometry, decided before paint:
//!
//! - **A heading** (optional): a list group's heading bound to its first row (SH9), as
//!   `ActionLine` lays one: `Heading::above` over its words (none when the heading starts its
//!   column, as `design::heading` leaves it out there) and `Heading::below` under them. It is the
//!   block's head (L6).
//! - **The words** (Body, wrapped). A row with nothing to do but finish (no roll, no check)
//!   ends its words with its finish actions where they fit after the last line, `S` after it,
//!   on a line as tall as a control with the words centred on it (`line_end`); else the actions
//!   take a line of their own at the right.
//! - **The rolls** (optional): each roll's action, as wide as the caller measured it, then its
//!   reason (Small, muted) `S` after it when it has one; `S` apart, wrapping at the row's edge, a
//!   control tall each, the rows the column's gap apart. An action wider than the row takes a row
//!   of its own, cut to the row (it cuts its words there); a reason that does not fit after its
//!   action takes the lines under it, wrapped at the row's edge, and the next roll starts under
//!   them. Every part stays in the block. Each roll and each check box has its own key, so its
//!   control keeps its id when another roll or check comes or goes.
//! - **The results** (optional): Body words set in by `L`, wrapped there.
//! - **The checks**: one check box per proposed change, each a row of the column's width as
//!   tall as the check box draws its words there (`check_height`).
//! - **The finish** (when the words did not end with it): a line a control tall with the finish
//!   actions at its right end.
//! - **The tail**: a gap after the row, inside the block.
//!
//! Each part after the first stands the column's gap (`LINE_GAP`) below the one before.
use egui::{Id, Rect, Ui, pos2, vec2};

use super::action_line::{WrapRoom, wrap};
use super::laid::{Laid, LaidText};
use crate::design::components::group_heading_text;
use crate::design::extras::Heading;
use crate::design::theme::LINE_GAP;
use crate::design::tokens::{Density, L, S, Type, XS, colors};
use egui::Galley;
use std::sync::Arc;

/// One roll of a notice row: its own key, its action's width as it measures, and why it is
/// blocked, if it is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoticeRoll<'a> {
    pub key: Id,
    pub width: f32,
    /// The narrowest its action paints (`Action::least_width`): a row narrower than this gives
    /// it no place, and the host draws nothing there.
    pub least: f32,
    pub reason: Option<&'a str>,
}

/// One check of a notice row: its own key and its check box's words.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoticeCheck<'a> {
    pub key: Id,
    pub words: &'a str,
}

/// A notice's row in a list (layout 11c scope C).
#[derive(Clone, Copy)]
pub struct NoticeRow<'a> {
    /// The row's stable key (its notice).
    pub key: Id,
    /// A list group's heading bound to this row (its title and count), or none.
    pub heading: Option<(&'a str, usize)>,
    /// The heading starts its column: no space above it.
    pub first: bool,
    /// The notice's words.
    pub words: &'a str,
    pub rolls: &'a [NoticeRoll<'a>],
    /// The rolls' results, one line each.
    pub results: Option<&'a str>,
    /// Each check box: its key and its words.
    pub checks: &'a [NoticeCheck<'a>],
    /// The finish actions' width (`Done` and the row's menu, with the gap between).
    pub finish: f32,
    /// The gap after the row, inside the block.
    pub tail: f32,
}

/// The slots of a laid notice row: the heading, the finish actions, then one per roll
/// (`FIRST_ROLL + at`), then one per check (`NoticeRow::check_slot`).
pub mod slot {
    pub const HEADING: usize = 0;
    pub const FINISH: usize = 1;
    pub const FIRST_ROLL: usize = 2;
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many notice rows were laid out (the quiet-frame proof: a host keeps its layouts).
    pub static NOTICES_LAID: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// How many notice rows were laid out so far on this thread (test builds).
#[cfg(any(test, feature = "test-support"))]
pub fn notices_laid() -> usize {
    NOTICES_LAID.with(std::cell::Cell::get)
}

/// How tall egui's check box draws `words` in a row `width` wide: its icon and the gap after it,
/// then its words wrapped in the rest, at least an interact height.
pub fn check_height(ui: &Ui, words: &str, width: f32) -> f32 {
    let spacing = ui.spacing();
    let room = (width - spacing.icon_width - spacing.icon_spacing).max(0.0);
    let font = egui::TextStyle::Body.resolve(ui.style());
    let galley = ui
        .fonts_mut(|fonts| fonts.layout(words.to_owned(), font, egui::Color32::PLACEHOLDER, room));
    galley
        .size()
        .y
        .max(spacing.interact_size.y)
        .max(spacing.icon_width)
}

impl NoticeRow<'_> {
    /// The slot of check `at`.
    pub fn check_slot(&self, at: usize) -> usize {
        slot::FIRST_ROLL + self.rolls.len() + at
    }

    /// Whether the row's words end with its finish actions (no roll, no check).
    fn ends_words(&self) -> bool {
        self.rolls.is_empty() && self.checks.is_empty()
    }

    /// The row laid out in a column `width` wide.
    pub fn lay(&self, ui: &Ui, width: f32) -> Laid {
        #[cfg(any(test, feature = "test-support"))]
        NOTICES_LAID.with(|laid| laid.set(laid.get() + 1));
        let control = Density::Read.control();
        let c = colors();
        let mut texts = Vec::new();
        let mut slots = vec![Rect::from_min_size(pos2(0.0, 0.0), vec2(0.0, 0.0)); slot::FIRST_ROLL];
        let mut y = 0.0;
        let mut head = None;
        if let Some((title, count)) = self.heading {
            let group = Heading::Group { title, count };
            let galley =
                ui.fonts_mut(|fonts| fonts.layout_job(group_heading_text(ui, title, count)));
            let above = if self.first { 0.0 } else { group.above() };
            let height = above + galley.size().y + group.below();
            texts.push(LaidText {
                at: pos2(0.0, above),
                galley,
            });
            slots[slot::HEADING] = Rect::from_min_size(pos2(0.0, 0.0), vec2(width, height));
            y = height;
            head = Some(height);
        }
        let body = Type::Body.font(ui);
        let layout = |words: &str, room: f32| {
            ui.fonts_mut(|fonts| fonts.layout(words.to_owned(), body.clone(), c.text, room))
        };
        // The words, and the finish actions at the end of their last line where they fit.
        let words = layout(self.words, width);
        let fits = self.ends_words()
            && words
                .rows
                .last()
                .is_some_and(|row| width - row.rect().width() >= self.finish + S);
        if fits {
            let before: usize = words.rows[..words.rows.len() - 1]
                .iter()
                .map(|row| row.char_count_including_newline())
                .sum();
            let split = self
                .words
                .char_indices()
                .nth(before)
                .map_or(self.words.len(), |(at, _)| at);
            let (first_rows, last) = self.words.split_at(split);
            if !first_rows.trim().is_empty() {
                let galley = layout(first_rows.trim_end(), width);
                let height = galley.size().y;
                texts.push(LaidText {
                    at: pos2(0.0, y),
                    galley,
                });
                y += height;
            }
            let line = control.max(Type::Body.line());
            let galley = ui.fonts_mut(|fonts| {
                fonts.layout_no_wrap(last.trim().to_owned(), body.clone(), c.text)
            });
            texts.push(LaidText {
                at: pos2(0.0, y + (line - galley.size().y) / 2.0),
                galley,
            });
            slots[slot::FINISH] = Rect::from_min_size(
                pos2(width - self.finish, y + (line - control) / 2.0),
                vec2(self.finish, control),
            );
            y += line;
        } else {
            let height = words.size().y;
            texts.push(LaidText {
                at: pos2(0.0, y),
                galley: words,
            });
            y += height;
        }
        // The rolls: each action, then its reason, wrapping at the row's edge by the one rule of
        // a list's actions (`pane::wrap`), rows the column's gap apart. A roll
        // whose action and reason do not fit the row together takes a row of its own: its action
        // cut to the row, its reason on the lines under it; the next rolls start under those.
        if !self.rolls.is_empty() {
            y += LINE_GAP;
            let small = Type::Small.font(ui);
            let reasons: Vec<Option<Arc<Galley>>> = self
                .rolls
                .iter()
                .map(|roll| {
                    roll.reason.map(|reason| {
                        ui.fonts_mut(|fonts| {
                            fonts.layout_no_wrap(reason.to_owned(), small.clone(), c.muted)
                        })
                    })
                })
                .collect();
            let item = |at: usize| {
                self.rolls[at].width
                    + reasons[at]
                        .as_ref()
                        .map_or(0.0, |galley| S + galley.size().x)
            };
            let mut at = 0;
            // A run or a wide roll after another part of the rolls stands the column's gap under
            // it.
            let mut placed = false;
            while at < self.rolls.len() {
                // The next run of rolls that each fit the row whole.
                let end = (at..self.rolls.len())
                    .find(|&next| item(next) > width + 0.5)
                    .unwrap_or(self.rolls.len());
                if end > at {
                    if placed {
                        y += LINE_GAP;
                    }
                    placed = true;
                    let widths: Vec<f32> = (at..end).map(item).collect();
                    let wrapped = wrap(
                        &widths,
                        WrapRoom {
                            start: 0.0,
                            width,
                            top: y,
                            height: control,
                            row_gap: LINE_GAP,
                        },
                    );
                    let bottom = wrapped.bottom();
                    for (rect, roll) in wrapped.rects().iter().zip(at..end) {
                        let action = self.rolls[roll].width;
                        slots.push(Rect::from_min_size(rect.min, vec2(action, control)));
                        if let Some(galley) = reasons[roll].clone() {
                            texts.push(LaidText {
                                at: pos2(
                                    rect.left() + action + S,
                                    rect.top() + (control - galley.size().y) / 2.0,
                                ),
                                galley,
                            });
                        }
                    }
                    y = bottom;
                    at = end;
                }
                if at < self.rolls.len() {
                    // A roll too wide for the row with its reason: a row of its own.
                    if placed {
                        y += LINE_GAP;
                    }
                    placed = true;
                    let roll = &self.rolls[at];
                    // Cut to the row, unless the row is narrower than the action can be: then it
                    // has no place (a slot of no width, which the host leaves empty), so nothing
                    // paints outside the block.
                    let cut = roll.width.min(width);
                    let shown = if cut + 0.5 < roll.least { 0.0 } else { cut };
                    slots.push(Rect::from_min_size(pos2(0.0, y), vec2(shown, control)));
                    y += control;
                    if let Some(reason) = roll.reason {
                        let galley = ui.fonts_mut(|fonts| {
                            fonts.layout(reason.to_owned(), small.clone(), c.muted, width)
                        });
                        y += XS;
                        let height = galley.size().y;
                        texts.push(LaidText {
                            at: pos2(0.0, y),
                            galley,
                        });
                        y += height;
                    }
                    at += 1;
                }
            }
        }
        if let Some(results) = self.results {
            y += LINE_GAP;
            let galley = layout(results, (width - L).max(0.0));
            let height = galley.size().y;
            texts.push(LaidText {
                at: pos2(L, y),
                galley,
            });
            y += height;
        }
        for check in self.checks {
            y += LINE_GAP;
            let height = check_height(ui, check.words, width);
            slots.push(Rect::from_min_size(pos2(0.0, y), vec2(width, height)));
            y += height;
        }
        // The finish actions on a line of their own, at its right end; a row narrower than they
        // are gives them no place and no line (a slot of no width, which the host leaves empty).
        if !fits && self.finish <= width + 0.5 {
            y += LINE_GAP;
            slots[slot::FINISH] =
                Rect::from_min_size(pos2(width - self.finish, y), vec2(self.finish, control));
            y += control;
        } else if !fits {
            // No place and no line: a slot of no width at the row's end.
            slots[slot::FINISH] = Rect::from_min_size(pos2(width, y), vec2(0.0, 0.0));
        }
        // Each roll's and check's slot follows its own key, not its place.
        let keys = [None; slot::FIRST_ROLL]
            .into_iter()
            .chain(self.rolls.iter().map(|roll| Some(roll.key)))
            .chain(self.checks.iter().map(|check| Some(check.key)))
            .collect();
        Laid::new(self.key, vec2(width, y + self.tail), head, texts, slots).with_slot_keys(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::components::Action;
    use crate::design::outcome::{Messages, NoPress, PaneFrame};
    use crate::design::pane::{Body, draw_laid};
    use crate::design::test_harness::Themed;
    use egui_kittest::Harness;

    /// One notice row of the test list.
    #[derive(Clone)]
    struct Row {
        heading: Option<(&'static str, usize)>,
        words: &'static str,
        rolls: Vec<(&'static str, Option<&'static str>)>,
        results: Option<&'static str>,
        checks: Vec<&'static str>,
    }

    fn rows() -> Vec<Row> {
        vec![
            Row {
                heading: Some(("Due", 4)),
                words: "Bell. The Cistern floods. Announce the standings. Roll each rival. \
                        Check d6 for wandering.",
                rolls: vec![
                    ("Roll Rival Track", Some("Each name has its roll.")),
                    ("Roll Wandering, 0:40 to 1:19", None),
                ],
                results: Some(
                    "Old Guard: 5 · Moves one room on.\nKeyring: 4 · Moves one room on.\n\
                     Wandering, 0:40 to 1:19: 6 · Nothing comes.",
                ),
                checks: Vec::new(),
            },
            Row {
                heading: None,
                words: "Old Guard: First seal. The party reaches level 2.",
                rolls: Vec::new(),
                results: None,
                checks: Vec::new(),
            },
            Row {
                heading: None,
                words: "Day 32: Move the rival powers.",
                rolls: Vec::new(),
                results: None,
                checks: vec![
                    "Seals (Party) +1",
                    "Torches set to 3, and the party's lamps are filled again for the long \
                     walk",
                ],
            },
            Row {
                heading: None,
                words: "A short one.",
                rolls: Vec::new(),
                results: None,
                checks: Vec::new(),
            },
        ]
    }

    /// A nonzero tail, so the block's end is the lowest painted part plus it.
    const TAIL: f32 = XS;

    fn roll_action(words: &str) -> Action<'_> {
        Action::secondary(words).density(Density::Read)
    }

    fn finish_width(ui: &Ui) -> f32 {
        Action::quiet("Done")
            .density(Density::Read)
            .natural_width(ui)
            + S
            + Density::Read.control()
    }

    /// What one pass drew of a row: its laid rect, and each control's slot, the rect it drew
    /// at, and whether it fills its slot's width (a check box is as wide as its words).
    type Drawn = (Rect, Vec<(Rect, Rect, bool)>);

    /// The rows in a Content pane, each laid out before paint and drawn in place.
    fn harness(width: f32, scale: f32) -> Harness<'static, Vec<Drawn>> {
        let mut harness = Harness::builder()
            .with_size(vec2(width, 1200.0))
            .with_pixels_per_point(scale)
            .themed_ui_state(
                |ui, drawn: &mut Vec<Drawn>| {
                    drawn.clear();
                    let frame = PaneFrame::new(ui);
                    let messages =
                        Messages::<NoPress>::new(&frame, "notice_row_test", ui.available_width());
                    let _ = messages.pane(ui, Body::Content, |ui| {
                        let width = ui.available_width();
                        let finish = finish_width(ui);
                        for (at, row) in rows().into_iter().enumerate() {
                            let rolls: Vec<NoticeRoll<'_>> = row
                                .rolls
                                .iter()
                                .map(|(words, reason)| NoticeRoll {
                                    key: Id::new(("roll", *words)),
                                    width: roll_action(words).natural_width(ui),
                                    least: roll_action(words).least_width(ui),
                                    reason: *reason,
                                })
                                .collect();
                            let checks: Vec<NoticeCheck<'_>> = row
                                .checks
                                .iter()
                                .map(|words| NoticeCheck {
                                    key: Id::new(("check", *words)),
                                    words,
                                })
                                .collect();
                            let recipe = NoticeRow {
                                key: Id::new(("notice_row_test", at)),
                                heading: row.heading,
                                first: at == 0,
                                words: row.words,
                                rolls: &rolls,
                                results: row.results,
                                checks: &checks,
                                finish,
                                tail: TAIL,
                            };
                            let laid = recipe.lay(ui, width);
                            let mut controls = Vec::new();
                            let rect = draw_laid(ui, &laid, |ui| {
                                for (index, (words, _)) in row.rolls.iter().enumerate() {
                                    let at = slot::FIRST_ROLL + index;
                                    let drew =
                                        ui.in_slot(at, |ui| roll_action(words).show(ui).rect);
                                    controls.push((ui.slot(at), drew, true));
                                }
                                for (index, words) in row.checks.iter().enumerate() {
                                    let at = recipe.check_slot(index);
                                    let drew = ui.in_slot(at, |ui| {
                                        let mut on = true;
                                        ui.checkbox(&mut on, *words).rect
                                    });
                                    controls.push((ui.slot(at), drew, false));
                                }
                                let at = slot::FINISH;
                                let drew = ui.in_slot(at, |ui| {
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.spacing_mut().item_spacing.x = S;
                                            let menu = ui
                                                .allocate_exact_size(
                                                    vec2(
                                                        Density::Read.control(),
                                                        Density::Read.control(),
                                                    ),
                                                    egui::Sense::click(),
                                                )
                                                .0;
                                            let done = Action::quiet("Done")
                                                .density(Density::Read)
                                                .show(ui)
                                                .rect;
                                            menu.union(done)
                                        },
                                    )
                                    .inner
                                });
                                controls.push((ui.slot(at), drew, true));
                            });
                            drawn.push((rect, controls));
                        }
                    });
                },
                Vec::new(),
            );
        harness.run_steps(3);
        harness
    }

    /// Layout 11c scope C: a notice row's measured height is its painted height at every width
    /// and scale, in each of its forms (rolls with a reason and results; finish at the end of
    /// the words; on its own line; check boxes that wrap), with the app's own controls in their
    /// slots. The reference: each control's drawn rect, and every painted text's bounds.
    #[test]
    fn a_notice_row_paints_at_its_measured_height_at_every_width() {
        let (mut at_line_end, mut wrapped_checks, mut wrapped_rolls) = (0, 0, 0);
        // Every 6 px from 300 to 720 at scale 1, so a measure that wraps a check's words in the
        // wrong room meets a width where the line count differs; five widths at 1.5.
        let widths = |scale: f32| -> Vec<f32> {
            if scale == 1.0 {
                (0..=70).map(|step| 300.0 + 6.0 * step as f32).collect()
            } else {
                vec![300.0, 340.0, 420.0, 560.0, 720.0]
            }
        };
        let (mut gaps_seen, mut centred) = (0, 0);
        for scale in [1.0, 1.5] {
            for width in widths(scale) {
                let harness = harness(width, scale);
                let drawn = harness.state();
                assert_eq!(
                    drawn.len(),
                    rows().len(),
                    "{width}@{scale}: every row draws"
                );
                // Every painted text (all of the rows are in view in a 1200 px window).
                let texts: Vec<Rect> = harness
                    .output()
                    .shapes
                    .iter()
                    .filter_map(|clipped| match &clipped.shape {
                        // The text's line box: its galley at its place.
                        egui::Shape::Text(text) => {
                            Some(Rect::from_min_size(text.pos, text.galley.size()))
                        }
                        _ => None,
                    })
                    .collect();
                for (index, (rect, controls)) in drawn.iter().enumerate() {
                    for (slot, drew, fills) in controls {
                        let wide = if *fills {
                            (slot.width() - drew.width()).abs() < 0.5
                        } else {
                            drew.width() <= slot.width() + 0.5
                        };
                        assert!(
                            (slot.min - drew.min).length() < 0.5
                                && (slot.height() - drew.height()).abs() < 0.5
                                && wide,
                            "notice-row-slot: {width}@{scale}: row {index} drew {drew:?} in slot {slot:?}"
                        );
                    }
                    // The parts of the row the column's gap apart (an independent contract: what
                    // stands above is what painted): each check box, the first row of rolls, the
                    // results and a finish on its own line start `LINE_GAP` under the lowest part
                    // painted above them. A finish at the end of the words' line shares that line.
                    let parts: Vec<Rect> = controls
                        .iter()
                        .map(|(_, drew, _)| *drew)
                        .chain(
                            texts
                                .iter()
                                .copied()
                                .filter(|text| rect.contains(text.center())),
                        )
                        .collect();
                    let gap_above = |part: Rect| {
                        parts
                            .iter()
                            .filter(|other| other.bottom() <= part.top() + 0.5)
                            .map(|other| part.top() - other.bottom())
                            .fold(f32::INFINITY, f32::min)
                    };
                    let row = &rows()[index];
                    let mut spaced: Vec<Rect> = Vec::new();
                    if !row.rolls.is_empty() {
                        spaced.push(controls[0].1);
                    }
                    let first_check = row.rolls.len();
                    spaced.extend(
                        controls[first_check..controls.len() - 1]
                            .iter()
                            .map(|c| c.1),
                    );
                    let finish = controls.last().expect("the finish").1;
                    let at_line_end = texts.iter().any(|text| {
                        rect.contains(text.center())
                            && text.center().y > finish.top()
                            && text.center().y < finish.bottom()
                            && text.right() < finish.left()
                    });
                    if !at_line_end {
                        spaced.push(finish);
                    }
                    if let Some(results) = row.results {
                        let _ = results;
                        if let Some(text) = texts
                            .iter()
                            .filter(|text| rect.contains(text.center()))
                            .find(|text| (text.left() - rect.left() - L).abs() < 1.0)
                        {
                            spaced.push(*text);
                        }
                    }
                    for part in spaced {
                        let gap = gap_above(part);
                        assert!(
                            (gap - LINE_GAP).abs() < 1.0,
                            "notice-row-gap: {width}@{scale}: row {index}: {part:?} stands {gap} under the part above it, not {LINE_GAP}"
                        );
                        gaps_seen += 1;
                    }
                    // Every painted part lies in the row's block, and the lowest painted bottom
                    // with the row's nonzero tail ends it (live bounds, read from what painted).
                    if let Err(why) = crate::design::pane::painted_in_block(*rect, TAIL, &parts) {
                        panic!("notice-row-height: {width}@{scale}: row {index}: {why}");
                    }
                    // A finish at the end of the words' last line stands on that line's centre.
                    if at_line_end {
                        let last = texts
                            .iter()
                            .filter(|text| {
                                rect.contains(text.center())
                                    && text.center().y > finish.top()
                                    && text.center().y < finish.bottom()
                            })
                            .map(|text| text.center().y)
                            .next();
                        assert!(
                            last.is_some_and(|y| (y - finish.center().y).abs() < 0.5),
                            "notice-row-centre: {width}@{scale}: row {index}: finish {finish:?}, its line's centre {last:?}"
                        );
                        centred += 1;
                    }
                }
                let checks = &drawn[2].1;
                wrapped_checks +=
                    usize::from(checks.iter().any(|(slot, _, _)| slot.height() > 30.0));
                let rolls = &drawn[0].1;
                wrapped_rolls += usize::from(rolls[1].0.top() > rolls[0].0.top() + 1.0);
                let finish = drawn[3].1.last().expect("the finish").0;
                at_line_end +=
                    usize::from(finish.top() < drawn[3].0.top() + Density::Read.control());
            }
        }
        assert!(
            gaps_seen > 0 && centred > 0,
            "notice-row-reached: gaps {gaps_seen}, centred finishes {centred}"
        );
        assert!(
            at_line_end > 0 && wrapped_checks > 0 && wrapped_rolls > 0,
            "notice-row-reached: finish on the words' line {at_line_end}, a wrapped check {wrapped_checks}, wrapped rolls {wrapped_rolls}"
        );
    }

    /// Astra's P2 on PR 885 (Sol 2-018): a roll with the live dice icon and its reason wrap at
    /// their threshold. One point wider than the width where the action and its reason just fit
    /// one row, the reason stands after the action; one point narrower, it takes the line under
    /// it; narrower than the action itself, the action is cut to the row. At each, everything
    /// painted lies in the measured block, which the lowest painted bottom and a nonzero tail
    /// end. The reference: the drawn action's rect and the painted words' line boxes.
    #[test]
    fn a_roll_and_its_reason_wrap_at_their_threshold() {
        use crate::design::icons;
        const LABEL: &str = "Roll Wandering, 0:40 to 1:19 at the cistern gate";
        const REASON: &str = "Each name has its roll.";
        let action = || {
            Action::secondary(LABEL)
                .icon(icons::DICE)
                .density(Density::Read)
        };
        // (the action's width, the reason's width), measured in the test's own harness.
        let widths = {
            let mut h = Harness::builder()
                .with_size(vec2(1200.0, 300.0))
                .themed_ui_state(
                    move |ui, widths: &mut (f32, f32, f32)| {
                        let reason = ui
                            .fonts_mut(|fonts| {
                                fonts.layout_no_wrap(
                                    REASON.to_owned(),
                                    Type::Small.font(ui),
                                    colors().muted,
                                )
                            })
                            .size()
                            .x;
                        *widths = (action().natural_width(ui), reason, action().least_width(ui));
                    },
                    (0.0, 0.0, 0.0),
                );
            h.run_steps(2);
            *h.state()
        };
        let threshold = widths.0 + S + widths.1;
        let mut seen = Vec::new();
        for (width, place) in [
            (threshold + 1.0, "beside"),
            (threshold - 1.0, "under"),
            (widths.0 - 30.0, "cut"),
            // Narrower than the action can be (one control and its icon): it has no place.
            (widths.2 - 6.0, "refused"),
        ] {
            let mut h = Harness::builder()
                .with_size(vec2(1200.0, 400.0))
                .themed_ui_state(
                    move |ui, drawn: &mut (Rect, Rect, Vec<Rect>)| {
                        let rolls = [NoticeRoll {
                            key: Id::new("roll"),
                            width: action().natural_width(ui),
                            least: action().least_width(ui),
                            reason: Some(REASON),
                        }];
                        let laid = NoticeRow {
                            key: Id::new(("threshold", width.to_bits())),
                            heading: None,
                            first: false,
                            words: "Bell.",
                            rolls: &rolls,
                            results: None,
                            checks: &[],
                            finish: finish_width(ui),
                            tail: TAIL,
                        }
                        .lay(ui, width);
                        let mut controls = Vec::new();
                        let block = draw_laid(ui, &laid, |ui| {
                            // The host leaves a slot of no width empty.
                            if ui.slot(slot::FIRST_ROLL).width() > 0.5 {
                                controls.push(ui.in_slot(slot::FIRST_ROLL, |ui| {
                                    action().blocked(Some(REASON)).show(ui).rect
                                }));
                            } else {
                                controls.push(Rect::NOTHING);
                            }
                            if ui.slot(slot::FINISH).width() > 0.5 {
                                controls.push(ui.actions(slot::FINISH, |ui| {
                                    Action::quiet("Done").density(Density::Read).show(ui).rect
                                }));
                            }
                        });
                        *drawn = (block, controls[0], controls);
                    },
                    (Rect::NOTHING, Rect::NOTHING, Vec::new()),
                );
            h.run_steps(3);
            let (block, roll, controls) = h.state().clone();
            let texts: Vec<(String, Rect)> = h
                .output()
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) => Some((
                        text.galley.job.text.clone(),
                        Rect::from_min_size(text.pos, text.galley.size()),
                    )),
                    _ => None,
                })
                .filter(|(_, text)| block.contains(text.center()))
                .collect();
            let parts: Vec<Rect> = controls
                .iter()
                .copied()
                .filter(|rect| *rect != Rect::NOTHING)
                .chain(texts.iter().map(|(_, text)| *text))
                .collect();
            if let Err(why) = crate::design::pane::painted_in_block(block, TAIL, &parts) {
                panic!("notice-row-threshold: {place} at {width}: {why}");
            }
            // The reason, by its words: in the roll's row after it (beside), or under it.
            let reason = texts
                .iter()
                .find(|(words, _)| words == REASON)
                .map(|(_, text)| *text);
            let beside = reason.is_some_and(|text| {
                (text.center().y - roll.center().y).abs() < 1.0 && text.left() > roll.right()
            });
            let under = reason.is_some_and(|text| text.top() >= roll.bottom());
            let ok = match place {
                "beside" => beside,
                "under" => under,
                "cut" => under && roll.width() <= width + 0.5,
                _ => roll == Rect::NOTHING && reason.is_some(),
            };
            assert!(
                ok,
                "notice-row-threshold: {place} at {width}: roll {roll:?}, reason {reason:?}, block {block:?}"
            );
            seen.push(place);
        }
        assert_eq!(seen, ["beside", "under", "cut", "refused"]);
    }

    /// Astra's P2 on PR 885: a check box's control is keyed by its own key, not its place. The
    /// second check box has the keyboard; the first goes (its change was taken back); the one
    /// that stays keeps its focus. Red: slots by their place.
    #[test]
    fn a_check_keeps_its_focus_when_the_one_before_it_goes() {
        use egui_kittest::kittest::Queryable;
        let mut h = Harness::builder()
            .with_size(vec2(500.0, 300.0))
            .themed_ui_state(
                |ui, words: &mut Vec<&'static str>| {
                    let checks: Vec<NoticeCheck<'_>> = words
                        .iter()
                        .map(|words| NoticeCheck {
                            key: Id::new(("check", *words)),
                            words,
                        })
                        .collect();
                    let laid = NoticeRow {
                        key: Id::new("keys"),
                        heading: None,
                        first: false,
                        words: "Day 32: Move the rival powers.",
                        rolls: &[],
                        results: None,
                        checks: &checks,
                        finish: finish_width(ui),
                        tail: TAIL,
                    }
                    .lay(ui, 480.0);
                    let _ = draw_laid(ui, &laid, |ui| {
                        for (at, words) in words.iter().enumerate() {
                            let mut on = true;
                            let _ = ui
                                .in_slot(slot::FIRST_ROLL + at, |ui| ui.checkbox(&mut on, *words));
                        }
                    });
                },
                vec!["Seals (Party) +1", "Torches set to 3"],
            );
        h.run_steps(2);
        h.get_by_label("Torches set to 3").focus();
        h.run_steps(2);
        let focused = h.ctx.memory(|memory| memory.focused());
        assert!(focused.is_some(), "the second check box has the keyboard");
        h.state_mut().remove(0);
        h.run_steps(3);
        assert_eq!(
            h.ctx.memory(|memory| memory.focused()),
            focused,
            "notice-row-keys: the check box that stays keeps its id and its focus"
        );
    }

    /// The heading leaves out its space above when it starts its column, as `design::heading`
    /// does, and keeps it elsewhere.
    #[test]
    fn a_first_heading_has_no_space_above() {
        let harness = Harness::builder()
            .with_size(vec2(400.0, 300.0))
            .themed_ui_state(
                |ui, tops: &mut Vec<f32>| {
                    tops.clear();
                    for first in [true, false] {
                        let laid = NoticeRow {
                            key: Id::new(("first", first)),
                            heading: Some(("Due", 1)),
                            first,
                            words: "A notice.",
                            rolls: &[],
                            results: None,
                            checks: &[],
                            finish: 80.0,
                            tail: 0.0,
                        }
                        .lay(ui, 400.0);
                        tops.push(laid.height());
                    }
                },
                Vec::new(),
            );
        let tops = harness.state();
        let above = Heading::Group {
            title: "Due",
            count: 1,
        }
        .above();
        assert!(
            (tops[1] - tops[0] - above).abs() < 0.5,
            "notice-row-first: heads {tops:?}, above {above}"
        );
    }
}
