//! Measured fact/action rows (migration-32 layout PR 4; design §3 I6, §4.1; ruling L2).
//!
//! A list whose rows each hold facts and an action (a creature result and its `Add`): the part
//! measures every row's facts and every action recipe from data, and places the action column
//! exactly `L` after the widest facts. Spare width lies after the actions, never between the
//! facts and the actions. When the facts, `L` and the actions do not fit, each row's actions wrap
//! under its facts at the text edge. An action recipe includes its alternates (`Add`, the
//! `In roster` status, a blocked action with its recovery), so a row whose action changes keeps
//! the column where it was.
//!
//! Aligned columns cannot bound every row's distance from its own facts: a long outlier leaves a
//! short row far from the column. Each row's gap (`action.left - facts.right`) is recorded; a
//! gap above `4L` (ruling L2) is a counted, named residual, never a silent pass.

use super::{
    Action, Density, Status,
    tokens::{L, Type},
};
use egui::{self, Id, Rect, Ui, UiBuilder, pos2, vec2};

/// The largest gap between a row's facts and its action that is not a residual (ruling L2).
#[cfg(any(test, feature = "test-support"))]
pub const RESIDUAL_GAP: f32 = 4.0 * L;

/// A row's action: a secondary action with its words, its accessible name, the reason it is
/// blocked and the recovery beside it then; or a status in its place.
#[derive(Clone, Copy)]
pub enum RowAction<'a> {
    Action {
        words: &'a str,
        name: &'a str,
        blocked: Option<&'a str>,
        recovery: Option<&'a str>,
    },
    Status(Status<'a>),
}

/// What a row's action did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pressed {
    pub action: bool,
    pub recovery: bool,
}

impl RowAction<'_> {
    /// Its whole width, as drawn.
    pub fn width(&self, ui: &Ui) -> f32 {
        let gap = ui.spacing().item_spacing.x;
        match self {
            Self::Action {
                words, recovery, ..
            } => {
                let action = Action::secondary(words)
                    .density(Density::Read)
                    .natural_width(ui);
                action
                    + recovery.map_or(0.0, |recovery| {
                        gap + Action::secondary(recovery)
                            .density(Density::Read)
                            .natural_width(ui)
                    })
            }
            Self::Status(status) => {
                let font = Type::Body.font(ui);
                super::number_width(ui, status.icon, &font)
                    + gap
                    + super::number_width(ui, status.word, &font)
            }
        }
    }

    fn show(&self, ui: &mut Ui) -> Pressed {
        match *self {
            Self::Action {
                words,
                name,
                blocked,
                recovery,
            } => {
                let action = Action::secondary(words)
                    .name(name)
                    .density(Density::Read)
                    .blocked(blocked)
                    .show(ui)
                    .clicked();
                let recovery = recovery.is_some_and(|recovery| {
                    blocked.is_some()
                        && Action::secondary(recovery)
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                });
                Pressed { action, recovery }
            }
            Self::Status(status) => {
                let _ = super::status(ui, status);
                Pressed::default()
            }
        }
    }
}

/// The columns of a list, from its widest facts `F` and its widest action `A`: the actions start
/// `L` after `F`; when `F + L + A` exceeds the room, they wrap under each row's facts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Columns {
    pub facts: f32,
    pub actions: f32,
    pub wrapped: bool,
}

impl Columns {
    /// The columns for these facts and actions in `room` points (its whole width).
    pub fn measure(
        room: f32,
        facts: impl IntoIterator<Item = f32>,
        actions: impl IntoIterator<Item = f32>,
    ) -> Self {
        let facts = facts.into_iter().fold(0.0, f32::max);
        let actions = actions.into_iter().fold(0.0, f32::max);
        Self {
            facts,
            actions,
            wrapped: facts + L + actions > room + 0.5,
        }
    }

    /// The list's width: the facts, `L` and the actions; the spare room lies after them.
    pub fn width(&self, room: f32) -> f32 {
        if self.wrapped {
            room
        } else {
            self.facts + L + self.actions
        }
    }
}

/// A row's measured facts and action, as the gate reads them (test builds).
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowRecord {
    pub owner: &'static str,
    pub row: usize,
    pub facts: Rect,
    /// The facts on the actions' own line (a plan row's title above its summary); the same as
    /// `facts` where all the facts share that line.
    pub line: Rect,
    pub action: Rect,
    /// Where the action column starts, as planned: `L` after the widest facts.
    pub column: f32,
    /// The far edge the row's actions end at when its list keeps the far edge: the row's own
    /// right edge as the row used it (PR 605 review: proven apart from the actions).
    pub edge: f32,
    pub wrapped: bool,
}

#[cfg(any(test, feature = "test-support"))]
impl RowRecord {
    /// The row's gap: from its facts to its action; zero for a wrapped row.
    pub fn gap(&self) -> f32 {
        if self.wrapped {
            0.0
        } else {
            self.action.left() - self.facts.right()
        }
    }

    /// A gap above `4L` (ruling L2): a counted, named residual.
    pub fn residual(&self) -> bool {
        self.gap() > RESIDUAL_GAP + 0.5
    }
}

#[cfg(any(test, feature = "test-support"))]
const ROW_RECORDS: &str = "design_action_row_records";

/// The rows the part drew in the last pass that drew one, for the gate and the part's checks.
#[cfg(any(test, feature = "test-support"))]
pub fn row_records(ctx: &egui::Context) -> Vec<RowRecord> {
    ctx.data(|data| data.get_temp::<(u64, Vec<RowRecord>)>(Id::new(ROW_RECORDS)))
        .map(|(_, rows)| rows)
        .unwrap_or_default()
}

/// Keep a row's record for this pass (the ledger's rows record theirs too, layout PR 5a).
#[cfg(any(test, feature = "test-support"))]
pub fn keep(ui: &Ui, record: RowRecord) {
    let pass = ui.ctx().cumulative_pass_nr();
    ui.ctx().data_mut(|data| {
        let kept = data.get_temp_mut_or_default::<(u64, Vec<RowRecord>)>(Id::new(ROW_RECORDS));
        if kept.0 != pass {
            *kept = (pass, Vec::new());
        }
        kept.1.push(record);
    });
}

/// One row in `rect` (its whole block): its facts, drawn by `facts` in their column (`facts`
/// returns their drawn width), and its action `L` after the widest facts, or under its facts
/// when the columns wrap. `owner` and `row` name it in the residual record.
pub fn action_row<R>(
    ui: &mut Ui,
    rect: Rect,
    columns: Columns,
    owner: &'static str,
    row: usize,
    facts: impl FnOnce(&mut Ui) -> (R, f32),
    action: RowAction<'_>,
) -> (R, Pressed) {
    let control = Density::Read.control();
    let facts_rect = if columns.wrapped {
        Rect::from_min_max(rect.min, pos2(rect.right(), rect.bottom() - control))
    } else {
        Rect::from_min_size(rect.min, vec2(columns.facts, rect.height()))
    };
    let mut facts_ui = ui.new_child(
        UiBuilder::new()
            .id_salt(("facts", row))
            .max_rect(facts_rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    let (out, facts_width) = facts(&mut facts_ui);
    let action_rect = if columns.wrapped {
        Rect::from_min_max(pos2(rect.left(), rect.bottom() - control), rect.max)
    } else {
        Rect::from_min_max(pos2(rect.left() + columns.facts + L, rect.top()), rect.max)
    };
    let mut action_ui = ui.new_child(
        UiBuilder::new()
            .id_salt(("action", row))
            .max_rect(action_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let pressed = action.show(&mut action_ui);
    let drawn = action_ui.min_rect();
    let _ = (facts_width, drawn, owner);
    #[cfg(any(test, feature = "test-support"))]
    keep(
        ui,
        RowRecord {
            owner,
            row,
            facts: Rect::from_min_size(facts_rect.min, vec2(facts_width, facts_rect.height())),
            line: Rect::from_min_size(facts_rect.min, vec2(facts_width, facts_rect.height())),
            action: drawn,
            column: action_rect.left(),
            edge: rect.right(),
            wrapped: columns.wrapped,
        },
    );
    (out, pressed)
}

/// A list whose facts are laid out in their own room (they wrap or give way there): its columns
/// from its last pass, its widest drawn facts and its widest drawn actions (layout PR 5a-2, 5b).
/// `columns` gives them before a row is drawn; `measured` adds the row's own after. When they
/// change, the list is drawn once more, so its actions settle in their column on the next frame.
#[derive(Clone, Copy, Default)]
struct KeptColumns {
    pass: u64,
    last: (f32, f32),
    next: (f32, f32),
}

fn kept_columns<R>(ui: &Ui, list: Id, update: impl FnOnce(&mut KeptColumns) -> R) -> R {
    let pass = ui.ctx().cumulative_pass_nr();
    let (out, changed) = ui.ctx().data_mut(|data| {
        let kept = data.get_temp_mut_or_default::<KeptColumns>(list);
        let mut changed = false;
        if kept.pass != pass {
            if kept.pass + 1 == pass {
                changed = kept.next != kept.last;
                kept.last = kept.next;
            }
            kept.next = (0.0, 0.0);
            kept.pass = pass;
        }
        (update(kept), changed)
    });
    if changed {
        ui.ctx().request_repaint();
    }
    out
}

/// The list's columns from its last pass: (widest facts, widest actions); zero until measured.
/// The part's own: a list places its actions through [`MeasuredColumn`], never by a column of
/// its own (layout L5c, the ban of design §5 5b).
fn columns(ui: &Ui, list: Id) -> (f32, f32) {
    kept_columns(ui, list, |kept| kept.last)
}

/// Add a drawn row's facts and actions widths to this pass's columns.
/// A row wider than the kept column means this pass placed rows with a stale column: the pass
/// is discarded and drawn again with the new one, so the stale positions never reach the screen
/// (PR 641 review, P2). A column that only shrinks follows on the next frame; it never puts an
/// action over facts.
fn measured(ui: &Ui, list: Id, facts: f32, actions: f32) {
    let grew = kept_columns(ui, list, |kept| {
        kept.next = (kept.next.0.max(facts), kept.next.1.max(actions));
        facts > kept.last.0 + 0.5 || actions > kept.last.1 + 0.5
    });
    if grew {
        ui.ctx().request_discard("a list's measured column grew");
    }
}

/// A list whose rows' facts keep their own room, where they wrap or give way, and whose action
/// column comes from what the list drew in its last pass (recipe A, note-5a-2; layout PR 5a-2,
/// 5b and L5c). Each row's facts keep the room they had without the column, so its line breaks
/// and height never depend on it. When the list's widest drawn facts `F`, `L` and its widest
/// drawn actions `A` fit a row, its actions stand in the column `L` after `F`; else every row
/// keeps its actions at its far edge. The part decides where they stand and draws their region
/// ([`ActionSlot`]); a list has no column of its own.
#[derive(Clone, Copy, Debug)]
pub struct MeasuredColumn {
    list: Id,
    at: ColumnAt,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ColumnAt {
    End,
    Start { recipe: f32, facts: f32 },
}

impl MeasuredColumn {
    /// Each row's actions end at the column's end, so the rows' menus line up (the encounter
    /// roster, a group's members).
    pub fn end(list: impl std::hash::Hash) -> Self {
        Self {
            list: Id::new(list),
            at: ColumnAt::End,
        }
    }

    /// Each row's actions start at the column, in the row's own recipe width `recipe` (its
    /// actions with their alternates), for a list of mixed row kinds (the Session plan). `facts`
    /// is the width of the row's facts drawn before its actions (a plan row's title): until the
    /// list is measured, and where they are wider than its widest, they place the column.
    pub fn start(list: impl std::hash::Hash, recipe: f32, facts: f32) -> Self {
        Self {
            list: Id::new(list),
            at: ColumnAt::Start { recipe, facts },
        }
    }

    /// The slot of one row's actions. `line` is the actions' line: its top and bottom, its left
    /// where the actions may start, its right the row's far edge. `left` is where the row's
    /// facts start. Give the drawn row to [`ActionSlot::done`].
    #[must_use = "a slot's row is measured and recorded only by `ActionSlot::done`"]
    pub fn slot(self, ui: &Ui, line: Rect, left: f32) -> ActionSlot {
        let (kept_facts, kept_actions) = columns(ui, self.list);
        let far_edge = line.right();
        let room = far_edge - left;
        let (column, start, rect) = match self.at {
            ColumnAt::End => {
                let column = kept_facts > 0.0 && kept_facts + L + kept_actions <= room + 0.5;
                let end = if column {
                    left + kept_facts + L + kept_actions
                } else {
                    far_edge
                };
                (
                    column,
                    column.then_some(left + kept_facts + L),
                    Rect::from_min_max(line.min, pos2(end.max(line.left()), line.bottom())),
                )
            }
            ColumnAt::Start { recipe, facts } => {
                let widest = kept_facts.max(facts);
                let column = kept_facts > 0.0
                    && kept_actions >= recipe - 0.5
                    && widest + L + kept_actions <= room + 0.5;
                let at = if column {
                    left + widest + L
                } else {
                    far_edge - recipe
                };
                let rect = Rect::from_min_size(pos2(at, line.top()), vec2(recipe, line.height()));
                (column, Some(rect.left()), rect)
            }
        };
        ActionSlot {
            list: self.list,
            at: self.at,
            column,
            start,
            rect,
            far_edge,
            finished: false,
        }
    }
}

/// Where one row's actions stand in a [`MeasuredColumn`], and their region. Draw them with
/// [`ActionSlot::scope`] or [`ActionSlot::child`], then give the drawn row to
/// [`ActionSlot::done`]. It is not `Copy`, and a slot dropped before `done` fails a debug
/// assertion: a row that skipped `done` would not count in its list's column, would not draw
/// its list again when the column grew, and would not be recorded (PR 772 review, P2).
#[derive(Debug)]
#[must_use = "a slot's row is measured and recorded only by `ActionSlot::done`"]
pub struct ActionSlot {
    list: Id,
    at: ColumnAt,
    /// The actions stand in the list's column; else at the row's far edge.
    column: bool,
    /// Where the column starts, as the row's record states it; `None`: where the actions start.
    start: Option<f32>,
    rect: Rect,
    far_edge: f32,
    /// `done` ran.
    finished: bool,
}

impl Drop for ActionSlot {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            debug_assert!(
                self.finished,
                "a measured row's slot was dropped before `ActionSlot::done`"
            );
        }
    }
}

impl ActionSlot {
    /// The actions' region.
    pub fn rect(&self) -> Rect {
        self.rect
    }

    /// Draw the actions in their region, right to left and centred on their line; the row's
    /// layout goes on after the region.
    pub fn scope<R>(&self, ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<R> {
        ui.scope_builder(self.builder(), add)
    }

    /// The region as a child `Ui`, right to left and centred, that does not move the row's
    /// layout: the row places itself around [`ActionSlot::rect`].
    pub fn child(&self, ui: &mut Ui) -> Ui {
        ui.new_child(self.builder())
    }

    fn builder(&self) -> UiBuilder {
        UiBuilder::new()
            .max_rect(self.rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center))
    }

    /// The drawn row: `facts`, every line of facts it drew, from where they start; `line`, its
    /// facts on the actions' own line; `actions`, its drawn actions. Adds the row to the list's
    /// columns for the next pass and, in test builds, records it as `owner`'s row `row`.
    pub fn done(
        mut self,
        ui: &Ui,
        owner: &'static str,
        row: usize,
        facts: Rect,
        line: Rect,
        actions: Rect,
    ) {
        let width = match self.at {
            ColumnAt::End => actions.width(),
            ColumnAt::Start { recipe, .. } => recipe,
        };
        measured(ui, self.list, facts.width(), width);
        #[cfg(any(test, feature = "test-support"))]
        if actions.is_positive() {
            keep(
                ui,
                RowRecord {
                    owner,
                    row,
                    facts,
                    line,
                    action: actions,
                    column: self.start.unwrap_or(actions.left()),
                    edge: self.far_edge,
                    wrapped: !self.column,
                },
            );
        }
        #[cfg(not(any(test, feature = "test-support")))]
        let _ = (owner, row, line, self.column, self.start, self.far_edge);
        self.finished = true;
    }
}

/// The height a row needs: `base` (its facts), and an action line when the columns wrap.
pub fn row_height(base: f32, columns: Columns) -> f32 {
    if columns.wrapped {
        base + Density::Read.control()
    } else {
        base
    }
}
