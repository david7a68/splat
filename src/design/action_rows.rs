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
#[cfg(test)]
use eframe::egui::Id;
use eframe::egui::{self, Rect, Ui, UiBuilder, pos2, vec2};

/// The largest gap between a row's facts and its action that is not a residual (ruling L2).
#[cfg(test)]
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
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowRecord {
    pub owner: &'static str,
    pub row: usize,
    pub facts: Rect,
    pub action: Rect,
    pub wrapped: bool,
}

#[cfg(test)]
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

#[cfg(test)]
const ROW_RECORDS: &str = "design_action_row_records";

/// The rows the part drew in the last pass that drew one, for the gate and the part's checks.
#[cfg(test)]
pub fn row_records(ctx: &egui::Context) -> Vec<RowRecord> {
    ctx.data(|data| data.get_temp::<(u64, Vec<RowRecord>)>(Id::new(ROW_RECORDS)))
        .map(|(_, rows)| rows)
        .unwrap_or_default()
}

#[cfg(test)]
fn keep(ui: &Ui, record: RowRecord) {
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
    // The row is one block: its facts and its action, also wrapped under them (N12).
    super::register_row(ui, rect);
    let _ = (facts_width, drawn, owner);
    #[cfg(test)]
    keep(
        ui,
        RowRecord {
            owner,
            row,
            facts: Rect::from_min_size(facts_rect.min, vec2(facts_width, facts_rect.height())),
            action: drawn,
            wrapped: columns.wrapped,
        },
    );
    (out, pressed)
}

/// The height a row needs: `base` (its facts), and an action line when the columns wrap.
pub fn row_height(base: f32, columns: Columns) -> f32 {
    if columns.wrapped {
        base + Density::Read.control()
    } else {
        base
    }
}
