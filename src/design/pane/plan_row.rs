//! The Session plan's closed row as a recipe (migration-32, layout 11c, scope A2b; design note
//! v2 §3.2 and review item 4): every part of the row is measured with the fonts before anything
//! paints, so the whole-block plan knows its height and its place, and the plan's shared column
//! is known before any row draws.
//!
//! The row's geometry is today's `plan_row_with`, decided before paint:
//!
//! - **The lead slot**, `lead_width × XL` at the row's start, its glyph centred: every row's
//!   words start at one column (`x0`, the slot and `S`).
//! - **The title**, a link that wraps in `width − x0 − actions − S` (at least `XL`), its press
//!   target at least 24 px tall with the words centred in it.
//! - **The action place**, `actions × Density::Read.control()` at the row's top: `L` after the
//!   plan's widest facts where the shared column fits (`PlanColumns`, from every closed row,
//!   computed before paint), else at the row's far edge.
//! - **The summary**, under the title and the action place, in `width − x0 − Read.control − S`:
//!   words that wrap (in at most so many rows), and parts the row draws itself in slots of their
//!   own measured height (a status, a run link, a message).
//! - **The rule** after the row (the line gap, `S`, the hairline, the line gap), part of the
//!   block.
//!
//! The row's facts (its title and its summary lines) and its action recipe make the shared
//! column; a change of either lays every row out again (each row's layout revision changes).
use egui::{Id, Pos2, Rect, Ui, pos2, vec2};

use super::laid::{Laid, LaidText};
use crate::design::theme::LINE_GAP;
use crate::design::tokens::{Density, L, S, Type, XL, colors};

/// One part of a closed row's summary, under its title.
#[derive(Clone, Copy)]
pub enum SummaryPart<'a> {
    /// Words that wrap in the summary's width, in at most `max_rows` rows.
    Words {
        text: &'a str,
        role: Type,
        color: egui::Color32,
        max_rows: usize,
    },
    /// A part the row draws itself (`LaidUi::in_slot`), measured by that part: its height, and
    /// the width its facts take.
    Part { height: f32, width: f32 },
    /// Words of several colours laid out in the summary's width, in at most `max_rows` rows, and
    /// drawn in their slot by `LaidUi::part` (with a hover, a name, or as a link: its press
    /// target at least `XL` each way).
    Job {
        job: &'a egui::text::LayoutJob,
        max_rows: usize,
        link: bool,
    },
}

/// A closed Session plan row (layout 11c A2b).
#[derive(Clone, Copy)]
pub struct PlanRow<'a> {
    /// The row's stable key (its event, fight or group).
    pub key: Id,
    /// The lead slot's glyph: a fight's caret, a group's icon.
    pub lead: &'a str,
    /// The title, drawn as a link.
    pub title: &'a str,
    pub summary: &'a [SummaryPart<'a>],
    /// The row's own action place: its widest action recipe.
    pub actions: f32,
    /// The action place's height, as the actions measure it (one line: `Density::Read.control()`;
    /// more when they wrap): the summary starts under it and the block holds it (review of PR
    /// 820).
    pub actions_height: f32,
    /// The rule after the row is part of the block (a closed row); an opened row draws its
    /// details first, then the rule.
    pub rule: bool,
}

/// The plan's shared column: its widest facts and its widest action recipe, from every closed
/// row before any row draws (it replaces a measure kept from the last pass).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlanColumns {
    pub facts: f32,
    pub actions: f32,
}

impl PlanColumns {
    /// The column of these rows' facts and action recipes (`PlanRow::facts`).
    pub fn of(rows: impl IntoIterator<Item = (f32, f32)>) -> Self {
        rows.into_iter()
            .fold(Self::default(), |columns, (facts, actions)| Self {
                facts: columns.facts.max(facts),
                actions: columns.actions.max(actions),
            })
    }
}

/// The slots of a laid plan row, in order.
pub mod slot {
    /// The lead slot (its glyph is a laid text).
    pub const LEAD: usize = 0;
    /// The action place.
    pub const ACTIONS: usize = 1;
    /// The rule after the row.
    pub const RULE: usize = 2;
    /// The first summary part's slot (`SummaryPart::Part`, in order).
    pub const FIRST_PART: usize = 3;
}

/// The lead slot's width: every row's leading slot (a caret, an icon, a scene's box).
pub fn lead_width() -> f32 {
    Type::Body.size()
}

/// What a laid summary is made of: its galleys and its parts' slots, from its top.
struct Summary {
    texts: Vec<LaidText>,
    parts: Vec<Rect>,
    /// Each part's laid words (a `Job`), in the parts' order.
    part_texts: Vec<Option<std::sync::Arc<egui::Galley>>>,
    height: f32,
    /// The widest line, from the summary's left.
    width: f32,
}

impl PlanRow<'_> {
    fn x0() -> f32 {
        lead_width() + S
    }

    fn title_width(&self, width: f32) -> f32 {
        (width - Self::x0() - self.actions - S).max(XL)
    }

    fn summary_width(width: f32) -> f32 {
        (width - Self::x0() - Density::Read.control() - S).max(XL)
    }

    /// The summary's room in a row `width` wide: a part laid out before the row (a kept plan
    /// line, a group's members) is fitted to it.
    pub fn summary_room(width: f32) -> f32 {
        Self::summary_width(width)
    }

    /// The row's actions stand in the plan's shared column (`columns`, which holds this row's
    /// facts), `L` after its widest facts; else at the row's far edge (layout PR 5a-2, I6).
    pub fn in_column(&self, ui: &Ui, width: f32, columns: PlanColumns) -> bool {
        self.column(self.title(ui, width).size().x, width, columns)
    }

    fn column(&self, title_width: f32, width: f32, columns: PlanColumns) -> bool {
        let facts = columns.facts.max(title_width);
        columns.facts > 0.0
            && columns.actions >= self.actions - 0.5
            && facts + L + columns.actions <= width - Self::x0() + 0.5
    }

    /// The title's words: an empty title shows `Untitled` (review of PR 820), so the link is
    /// never empty.
    fn words(&self) -> &str {
        if self.title.trim().is_empty() {
            "Untitled"
        } else {
            self.title
        }
    }

    fn title(&self, ui: &Ui, width: f32) -> std::sync::Arc<egui::Galley> {
        let text = Type::Body
            .rich(ui, self.words())
            .color(colors().accent)
            .underline();
        egui::WidgetText::from(text).into_galley(
            ui,
            Some(egui::TextWrapMode::Wrap),
            self.title_width(width),
            egui::TextStyle::Body,
        )
    }

    /// The summary laid out at `left` and `top`, in `room` points.
    fn summary(&self, ui: &Ui, (left, top, room): (f32, f32, f32)) -> Summary {
        let mut summary = Summary {
            texts: Vec::new(),
            parts: Vec::new(),
            part_texts: Vec::new(),
            height: 0.0,
            width: 0.0,
        };
        for part in self.summary {
            let y = top + summary.height;
            match *part {
                SummaryPart::Words {
                    text,
                    role,
                    color,
                    max_rows,
                } => {
                    let mut job = egui::text::LayoutJob::single_section(
                        text.to_owned(),
                        egui::TextFormat::simple(role.font(ui), color),
                    );
                    job.wrap.max_width = room;
                    job.wrap.max_rows = max_rows;
                    job.wrap.break_anywhere = false;
                    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
                    summary.width = summary.width.max(galley.size().x);
                    summary.height += galley.size().y;
                    summary.texts.push(LaidText {
                        at: pos2(left, y),
                        galley,
                    });
                }
                SummaryPart::Part { height, width } => {
                    summary.width = summary.width.max(width);
                    summary.height += height;
                    summary
                        .parts
                        .push(Rect::from_min_size(pos2(left, y), vec2(room, height)));
                    summary.part_texts.push(None);
                }
                SummaryPart::Job {
                    job,
                    max_rows,
                    link,
                } => {
                    let mut job = job.clone();
                    job.wrap.max_width = room;
                    job.wrap.max_rows = max_rows;
                    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
                    let size = if link {
                        galley.size().max(vec2(XL, XL))
                    } else {
                        galley.size()
                    };
                    summary.width = summary.width.max(size.x);
                    summary.height += size.y;
                    summary.parts.push(Rect::from_min_size(pos2(left, y), size));
                    summary.part_texts.push(Some(galley));
                }
            }
        }
        summary
    }

    /// The row's facts (its title and every summary line, from its words' column) and its
    /// action recipe, at `width`: what the shared column is made of.
    pub fn facts(&self, ui: &Ui, width: f32) -> (f32, f32) {
        let title = self.title(ui, width).size().x;
        let summary = self
            .summary(ui, (0.0, 0.0, Self::summary_width(width)))
            .width;
        (title.max(summary), self.actions)
    }

    /// The row laid out at `width` with the plan's shared column.
    pub fn lay(&self, ui: &Ui, width: f32, columns: PlanColumns) -> Laid {
        let x0 = Self::x0();
        let lead = ui.fonts_mut(|fonts| {
            fonts.layout_no_wrap(self.lead.to_owned(), Type::Body.font(ui), colors().muted)
        });
        let lead_slot = Rect::from_min_size(Pos2::ZERO, vec2(lead_width(), XL));
        let lead_at = lead_slot.center() - lead.size() / 2.0;
        let title = self.title(ui, width);
        let title_height = title.size().y.max(24.0);
        let title_width = title.size().x;
        // The action place: `L` after the plan's widest facts where the shared column fits, else
        // at the row's far edge (layout PR 5a-2, I6).
        let action_x = if self.column(title_width, width, columns) {
            x0 + columns.facts.max(title_width) + L
        } else {
            width - self.actions
        };
        let actions =
            Rect::from_min_size(pos2(action_x, 0.0), vec2(self.actions, self.actions_height));
        let summary_top = title_height.max(actions.bottom());
        let summary = self.summary(ui, (x0, summary_top, Self::summary_width(width)));
        let row = (summary_top + summary.height).max(actions.bottom());
        // After the row, the plan column's line gap (`LINE_GAP`, egui's item gap there); then,
        // for a closed row, `S`, the hairline and the line gap again, as `plan_rule` draws them
        // after an opened row's details (layout 11c A2c: the same height in place and in the
        // region).
        let rule = if self.rule {
            Rect::from_min_size(pos2(0.0, row + LINE_GAP + S), vec2(width, 1.0))
        } else {
            Rect::from_min_size(pos2(0.0, row + LINE_GAP), vec2(width, 0.0))
        };
        let height = rule.bottom() + if self.rule { LINE_GAP } else { 0.0 };
        let mut texts = vec![LaidText {
            at: Pos2::new(lead_at.x, lead_at.y),
            galley: lead,
        }];
        texts.extend(summary.texts);
        let mut slots = vec![lead_slot, actions, rule];
        slots.extend(summary.parts);
        let mut slot_texts = vec![None, None, None];
        slot_texts.extend(summary.part_texts);
        Laid::new(self.key, vec2(width, height), None, texts, slots)
            .with_slot_texts(slot_texts)
            .with_links(vec![(
                LaidText {
                    at: pos2(x0, 0.0),
                    galley: title,
                },
                // A press target at least 24 px wide (review of PR 820).
                vec2(title_width.max(24.0), title_height),
            )])
    }
}

/// Paint the rule in its slot (`slot::RULE`).
pub fn rule(ui: &mut super::laid::LaidUi<'_>) {
    let rect = ui.slot(slot::RULE);
    ui.in_slot(slot::RULE, |ui| {
        ui.painter().hline(
            rect.x_range(),
            rect.center().y,
            egui::Stroke::new(crate::design::tokens::HAIRLINE_WIDTH, colors().hairline),
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::test_harness::Themed;
    use crate::design::{
        memo::Revised,
        outcome::{Messages, NoPress, PaneFrame},
        pane::{Block, BlockRegion, Body, RegionState},
    };
    use egui::Vec2;
    use egui_kittest::{Harness, kittest::Queryable};

    /// A Content pane of closed plan rows: each with its title link and its action, laid out
    /// before paint with the plan's shared column.
    struct Rows {
        titles: Vec<String>,
        laid: Vec<Laid>,
        blocks: Revised<Vec<Block>>,
        region: RegionState,
        /// Each drawn row's block rect, its title link's rect and its action's, as drawn.
        drawn: Vec<(usize, Rect, Rect, Rect)>,
        /// The actions wrap to two lines (two buttons, one under the other), as they measure.
        stacked: bool,
        /// Each row's height by the test's own separate layout, in this pass.
        expected: Vec<f32>,
    }

    const SUMMARY: [SummaryPart<'static>; 1] = [SummaryPart::Words {
        text: "Day 2 · Captain Varra and her guards on the road",
        role: Type::Small,
        color: egui::Color32::GRAY,
        max_rows: 3,
    }];

    /// The actions' measured height: one line, or two with `S` between.
    fn actions_height(stacked: bool) -> f32 {
        if stacked {
            2.0 * Density::Read.control() + S
        } else {
            Density::Read.control()
        }
    }

    /// The row's height by a separate layout (the test's own): the title (at least 24 px) beside
    /// the actions, the summary under the taller, then the line gap, `S`, the rule and the line gap (review of
    /// PR 820).
    fn expected_height(ui: &Ui, title: &str, width: f32, stacked: bool) -> f32 {
        let x0 = Type::Body.size() + S;
        let title = egui::WidgetText::from(Type::Body.rich(
            ui,
            if title.trim().is_empty() {
                "Untitled"
            } else {
                title
            },
        ))
        .into_galley(
            ui,
            Some(egui::TextWrapMode::Wrap),
            (width - x0 - 120.0 - S).max(XL),
            egui::TextStyle::Body,
        );
        let SummaryPart::Words {
            text,
            role,
            color,
            max_rows,
        } = SUMMARY[0]
        else {
            unreachable!("the test's summary is words")
        };
        let mut job = egui::text::LayoutJob::single_section(
            text.to_owned(),
            egui::TextFormat::simple(role.font(ui), color),
        );
        job.wrap.max_width = (width - x0 - Density::Read.control() - S).max(XL);
        job.wrap.max_rows = max_rows;
        let summary = ui.fonts_mut(|fonts| fonts.layout_job(job)).size().y;
        title.size().y.max(24.0).max(actions_height(stacked))
            + summary
            + LINE_GAP
            + S
            + 1.0
            + LINE_GAP
    }

    fn harness(titles: &[&str], size: Vec2, scale: f32) -> Harness<'static, Rows> {
        let rows = Rows {
            titles: titles.iter().map(|title| (*title).to_owned()).collect(),
            laid: Vec::new(),
            blocks: Revised::new(Vec::new()),
            region: RegionState::new("plan_row_test"),
            drawn: Vec::new(),
            stacked: false,
            expected: Vec::new(),
        };
        let mut harness = Harness::builder()
            .with_size(size)
            .with_pixels_per_point(scale)
            .themed_ui_state(
                |ui, rows: &mut Rows| {
                    let frame = PaneFrame::new(ui);
                    let messages =
                        Messages::<NoPress>::new(&frame, "plan_row_test", ui.available_width());
                    let _ = messages.pane(ui, Body::Content, |ui| {
                        let width = ui.available_width();
                        let stacked = rows.stacked;
                        rows.expected = rows
                            .titles
                            .iter()
                            .map(|title| expected_height(ui, title, width, stacked))
                            .collect();
                        let specs: Vec<PlanRow<'_>> = rows
                            .titles
                            .iter()
                            .enumerate()
                            .map(|(at, title)| PlanRow {
                                key: Id::new(("plan_row_test", at)),
                                lead: ">",
                                title,
                                summary: &SUMMARY,
                                actions: 120.0,
                                actions_height: actions_height(stacked),
                                rule: true,
                            })
                            .collect();
                        // The shared column, from every row, before any row draws.
                        let columns = PlanColumns::of(specs.iter().map(|row| row.facts(ui, width)));
                        rows.laid = specs
                            .iter()
                            .map(|row| row.lay(ui, width, columns))
                            .collect();
                        rows.blocks.set(rows.laid.iter().map(Laid::block).collect());
                        rows.drawn.clear();
                        let Rows {
                            titles,
                            laid,
                            blocks,
                            region,
                            drawn,
                            ..
                        } = rows;
                        BlockRegion::new("plan_row_test", region, blocks).show_laid(
                            ui,
                            |at| &laid[at],
                            |ui, at| {
                                let link = ui.link(0, &titles[at]).rect;
                                let action = if stacked {
                                    ui.in_slot(slot::ACTIONS, |ui| {
                                        let first = ui.add_sized(
                                            vec2(120.0, Density::Read.control()),
                                            egui::Button::new(format!("Act {at}")),
                                        );
                                        crate::design::extras::space(ui, S);
                                        let second = ui.add_sized(
                                            vec2(120.0, Density::Read.control()),
                                            egui::Button::new(format!("More {at}")),
                                        );
                                        first.rect.union(second.rect)
                                    })
                                } else {
                                    ui.put(slot::ACTIONS, egui::Button::new(format!("Act {at}")))
                                        .rect
                                };
                                rule(ui);
                                let block =
                                    Rect::from_min_size(ui.slot(slot::LEAD).min, laid[at].size());
                                drawn.push((at, block, link, action));
                            },
                        );
                    });
                },
                rows,
            );
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        harness.run_steps(3);
        harness
    }

    /// Layout 11c A2b (review item 4): a closed plan row draws where its recipe laid it: its
    /// title link and its action at their laid rects, every paint and widget inside the block
    /// (the bounds check), at three widths and two scales. Red: the title's wrap from the `Ui`.
    #[test]
    fn a_plan_row_draws_where_its_recipe_laid_it() {
        let titles = [
            "Bridge Ambush",
            "A long journey along the old river road to the mill",
            "Wolves",
        ];
        for scale in [1.0, 1.5] {
            for width in [960.0, 1200.0, 1920.0] {
                let harness = harness(&titles, vec2(width / scale, 700.0), scale);
                let rows = harness.state();
                assert_eq!(
                    rows.drawn.len(),
                    titles.len(),
                    "{width}@{scale}: every row draws"
                );
                for &(at, block, link, action) in &rows.drawn {
                    let laid = &rows.laid[at];
                    let x0 = lead_width() + S;
                    assert!(
                        (link.left() - (block.left() + x0)).abs() < 0.5
                            && (link.top() - block.top()).abs() < 0.5,
                        "{width}@{scale}: row {at}'s title {link:?} at its laid place in {block:?}"
                    );
                    // AccessKit's rects are in physical pixels.
                    let node = harness.get_by_label(titles[at]).rect();
                    let node = Rect::from_min_max(
                        (node.min.to_vec2() / scale).to_pos2(),
                        (node.max.to_vec2() / scale).to_pos2(),
                    );
                    assert!(
                        (node.width() - link.width()).abs() < 1.0,
                        "{width}@{scale}: the link's node {node:?} is its laid rect {link:?}"
                    );
                    let slot = action;
                    assert!(
                        slot.top() >= block.top() - 0.5 && slot.bottom() <= block.bottom() + 0.5,
                        "{width}@{scale}: row {at}'s action {slot:?} inside its block"
                    );
                    let _ = laid;
                    let expected = rows.expected[at];
                    assert!(
                        (block.height() - expected).abs() < 0.5,
                        "{width}@{scale}: row {at} is {} tall, a separate layout says {expected}",
                        block.height()
                    );
                }
            }
        }
    }

    /// The shared column comes from every row before any row draws: one longer title moves every
    /// row's action in the same frame, and lays every row out again (new layout revisions).
    /// Red: the column from the last pass.
    #[test]
    fn a_longer_title_moves_every_rows_action_in_one_frame() {
        let mut harness = harness(&["Wolves", "Bridge", "Mill"], vec2(1200.0, 600.0), 1.0);
        let actions = |harness: &Harness<'_, Rows>| -> Vec<f32> {
            harness
                .state()
                .drawn
                .iter()
                .map(|(_, _, _, action)| action.left())
                .collect()
        };
        let before = actions(&harness);
        let revisions: Vec<_> = harness.state().laid.iter().map(Laid::revision).collect();
        // Wider than every row's summary line, so it is the plan's widest facts.
        harness.state_mut().titles[1] =
            "The bridge over the river at the old mill, where the wolves wait for travellers"
                .to_owned();
        harness.step();
        let after = actions(&harness);
        assert!(
            before.iter().zip(&after).all(|(was, now)| now > was),
            "every row's action moved right in the same frame: {before:?} -> {after:?}"
        );
        assert!(
            harness
                .state()
                .laid
                .iter()
                .zip(&revisions)
                .all(|(laid, was)| laid.revision() != *was),
            "every row was laid out again"
        );
    }

    /// Review of PR 820: actions that wrap (two lines, as they measure) take their height in the
    /// block: the block is as tall as a separate layout says, so the summary starts under them and
    /// nothing goes past the block (the bounds check). Red: the actions' height left out.
    #[test]
    fn wrapped_actions_take_their_height() {
        let mut harness = harness(&["Bridge Ambush", "Wolves"], vec2(520.0, 600.0), 1.0);
        harness.state_mut().stacked = true;
        harness.run_steps(2);
        let rows = harness.state();
        for &(at, block, _, action) in &rows.drawn {
            let expected = rows.expected[at];
            assert!(
                (block.height() - expected).abs() < 0.5 && action.bottom() <= block.bottom() + 0.5,
                "row {at}: {} tall with its two lines of actions, a separate layout says {expected}",
                block.height()
            );
        }
    }

    /// Review of PR 820: a title link is a target at least 24 px wide, and an empty title shows
    /// `Untitled`.
    #[test]
    fn a_short_or_empty_title_keeps_its_target() {
        let harness = harness(&["A", ""], vec2(960.0, 600.0), 1.0);
        let drawn = &harness.state().drawn;
        assert!(
            drawn[0].2.width() >= 24.0 - 0.01,
            "a one-letter title's link: {:?}",
            drawn[0].2
        );
        let painted = harness.output().shapes.iter().any(|shape| {
            matches!(&shape.shape, egui::Shape::Text(text) if text.galley.text() == "Untitled")
        });
        assert!(
            painted
                && harness.query_by_label("Untitled").is_some()
                && drawn[1].2.width() >= 24.0 - 0.01,
            "an empty title shows and says `Untitled`, at least 24 px wide"
        );
    }
}
