//! The roster's rows: the initiative and the turn
//! mark, the side's shape, the picture, the name over its conditions, AC, temporary HP, the HP
//! readout and bar, and the HP word and sync state. A live row selects on click, adds a target
//! on Ctrl+click, opens its initiative on a click of the number, and offers its menu on a
//! right-click; the ended view's rows only show.
use super::measure;
use super::on_run;
use crate::design::{hp_state, hp_word};
use crate::encounter_run::effect::Time;
use crate::encounter_run::rolling::RowChoice;
use crate::encounter_run::view::{self, RowColumns};
use crate::encounter_run::{Command, Condition, Participant, Phase, Run};
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::rolls::Mode;
use gpui::{
    AnyElement, ClickEvent, Context, ElementId, Hsla, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Pixels, SharedString, Styled, Window, div, prelude::*, px,
};
use splat_party::combat_sync::SyncState;
use theme::{Density, RunScale, Type, TypeExt, colors};

/// An initiative field's width: a whole total of three characters (`-12`) with the caret, inside
/// the field's padding (`kit::TextField` scrolls words it cannot show whole).
pub(super) const INITIATIVE_FIELD: Pixels = px(60.0);

/// A row's place in the turn order.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum TurnMark {
    Current,
    Next,
    None,
}

impl TurnMark {
    pub(super) fn of(index: usize, current: Option<usize>, next_up: Option<usize>) -> Self {
        if current == Some(index) {
            Self::Current
        } else if next_up == Some(index) {
            Self::Next
        } else {
            Self::None
        }
    }
}

/// How a row is used: live (it acts), or shown in the ended view's final state.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Use {
    Live,
    Final,
}

/// The sync state's status (`view::sync_status`).
pub(super) fn sync_status(state: SyncState) -> kit::Status {
    super::status(&crate::encounter_run::view::sync_status(state))
}

/// The side's shape: a square for the party, a diamond for enemies, a circle for others. The
/// dock and tooltip give the word.
fn side_mark(side: &str, size: f32) -> AnyElement {
    let c = colors().side;
    let size = px(size);
    match side {
        "party" => div().size(size).flex_none().bg(c.party).into_any_element(),
        "enemies" => {
            let color = c.enemies;
            gpui::canvas(
                |_, _, _| {},
                move |bounds, _, window, _| {
                    let center = bounds.center();
                    let reach = bounds.size.width / 2.0 * 1.25;
                    let mut path = gpui::PathBuilder::fill();
                    path.move_to(gpui::point(center.x, center.y - reach));
                    path.line_to(gpui::point(center.x + reach, center.y));
                    path.line_to(gpui::point(center.x, center.y + reach));
                    path.line_to(gpui::point(center.x - reach, center.y));
                    path.close();
                    if let Ok(path) = path.build() {
                        window.paint_path(path, color);
                    }
                },
            )
            .size(size)
            .flex_none()
            .into_any_element()
        }
        _ => div()
            .size(size)
            .flex_none()
            .rounded_full()
            .bg(c.other)
            .into_any_element(),
    }
}

/// How a roster fits its room.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Fit {
    /// Every row at its scale's height, with the spare room shared out.
    Natural,
    /// The list needs the height: name-only rows shrink, the rest give up their padding.
    Compressed,
    /// Compressed, and each row keeps one change and a count of the rest.
    Summary,
}

/// How many of a row's changed-stat bonuses its second line shows.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Bonuses {
    All,
    One,
    None,
}

/// What a press on a part of the second line does.
#[derive(Clone, PartialEq, Debug)]
enum Act {
    None,
    /// A condition, or the count of the rest: the Conditions tab.
    Conditions,
    /// `No initiative`: the initiative field.
    Initiative,
    /// The count of the changes: the stat block, where they are.
    Changes,
    /// An official condition's name: its rule in Lookup, beside the run.
    Rule(String),
    /// An effect's count: spend a use, mark it spent or roll its recharge, or remove it when
    /// its time is up. Its place in the row's conditions, and its name, which must still be
    /// there at the press.
    Effect(EffectAct, usize, String),
}

/// What a press on an effect's count does (`effect_links` in the egui run).
#[derive(Clone, Copy, PartialEq, Debug)]
enum EffectAct {
    /// `Legendary actions 2 of 3`: one use (at 0, every use back).
    Spend,
    /// `Fire Breath ready`: mark it spent; `spent`: roll 1d6 for it.
    Recharge,
    /// `time is up`: the effect goes.
    Remove,
}

/// One part of the second line: its words, colour and press.
#[derive(Clone, Debug)]
struct Part {
    words: String,
    color: Hsla,
    act: Act,
    /// `No initiative` and a sync conflict are underlined.
    underline: bool,
    /// A state word at the mark's size (a sync state that opens the line).
    mark: bool,
    /// It follows the part before after a space, not ` · `: an effect's count after its name.
    joined: bool,
}

/// A row's second line as it fits.
#[derive(Clone, Debug, Default)]
struct Line {
    parts: Vec<Part>,
    /// The line takes the small type at the mark's size to stay on one line.
    small: bool,
    /// Even the small type needs two lines: the row grows by one.
    wraps: bool,
    /// The roll words take a line of their own under it.
    badge_below: bool,
    /// The line's width as drawn, where the roll words start after it.
    used: f32,
}

/// One row's fitted layout.
#[derive(Clone, Debug)]
pub(super) struct RowPlan {
    line: Line,
    /// The name needs a second line in its column.
    name_wraps: bool,
    height: f32,
    line_y: f32,
}

/// The whole roster's fit for one frame: its scale, columns, each row's layout (in order), and
/// whether it is longer than its room.
#[derive(Clone)]
pub(super) struct Plan {
    pub(super) scale: RunScale,
    columns: RowColumns,
    fit: Fit,
    rows: Vec<RowPlan>,
    /// The list's room, as the scale was chosen for it.
    pub(super) room: f32,
    /// Too tall even at its tightest: it scrolls, with the line that says what is out of view.
    pub(super) overflow: bool,
}

impl Plan {
    /// Each row's top and bottom in the list, with its participant.
    pub(super) fn spans(&self, order: &[usize]) -> Vec<(f32, f32, usize)> {
        let mut top = 0.0;
        order
            .iter()
            .zip(&self.rows)
            .map(|(&index, row)| {
                let span = (top, top + row.height, index);
                top += row.height;
                span
            })
            .collect()
    }
}

/// How a list uses room it does not need.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum Spare {
    /// The live roster: spare room goes to the rows, so no gap opens under the list.
    ToRows,
    /// The ended view's final state: rows keep their scale's height, and the loot follows.
    Keep,
}

/// The last plan and the hash of what it was fitted from (`plan_key`): a frame whose rows,
/// room and width did not change takes it, and no row's words are measured again.
#[derive(Default)]
pub(super) struct PlanCache(Option<(u64, Plan)>);

#[cfg(test)]
thread_local! {
    /// How many plans were fitted (a test of the cache).
    pub(super) static PLANS_MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// `plan`, or the cached one when nothing it reads changed.
pub(super) fn plan_cached(
    cache: &mut PlanCache,
    run: &Run,
    order: &[usize],
    width: f32,
    room: f32,
    spare: Spare,
    window: &Window,
) -> Plan {
    let key = plan_key(run, order, width, room, spare);
    if let Some((kept, plan)) = &cache.0
        && *kept == key
    {
        return plan.clone();
    }
    #[cfg(test)]
    PLANS_MADE.with(|made| made.set(made.get() + 1));
    let plan = plan(run, order, width, room, spare, window);
    cache.0 = Some((key, plan.clone()));
    plan
}

/// Every fact a plan is fitted from, hashed: the room, the order and the turn, and each row's
/// words and marks as `parts`, `fit_line` and `row_plan` read them.
fn plan_key(run: &Run, order: &[usize], width: f32, room: f32, spare: Spare) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    width.to_bits().hash(&mut hash);
    room.to_bits().hash(&mut hash);
    (spare == Spare::ToRows).hash(&mut hash);
    order.hash(&mut hash);
    run.current().hash(&mut hash);
    for (index, p) in run.state.participants.iter().enumerate() {
        p.name.hash(&mut hash);
        p.side.hash(&mut hash);
        p.present.hash(&mut hash);
        p.initiative.is_some().hash(&mut hash);
        p.concentration.hash(&mut hash);
        (p.hp, p.max_hp).hash(&mut hash);
        p.conditions.hash(&mut hash);
        p.npc_identity().hash(&mut hash);
        p.definition.name.hash(&mut hash);
        p.loot_ready().hash(&mut hash);
        p.changes
            .as_ref()
            .filter(|_| p.stat_changes())
            .map(|changes| changes.overrides.table_bonuses(&changes.base))
            .hash(&mut hash);
        run.row_sync_word(index).hash(&mut hash);
        run.row_badge(index).hash(&mut hash);
        run.conc_save_dc(index).hash(&mut hash);
    }
    hash.finish()
}

/// The run's list of rows in `width` (inside the scroll column's lane) and `room`: its scale, then each row
/// laid out in the measured columns; spare room goes to the rows (`Spare`); a list that needs the
/// height compresses, then summarises, and the rest scrolls under a line that counts it.
pub(super) fn plan(
    run: &Run,
    order: &[usize],
    width: f32,
    room: f32,
    spare: Spare,
    window: &Window,
) -> Plan {
    let count = order.len();
    let mut scale = RunScale::fit(room, width, count);
    let total = |rows: &[RowPlan]| -> f32 { rows.iter().map(|row| row.height).sum() };
    let laid = |scale: RunScale, fit: Fit, width: f32, even: f32| {
        let columns = columns(run, scale, fit, width, window);
        let rows: Vec<RowPlan> = order
            .iter()
            .map(|&index| row_plan(run, index, scale, &columns, fit, even, window))
            .collect();
        (columns, rows)
    };
    let (_, rows) = laid(scale, Fit::Natural, width, 0.0);
    let natural = total(&rows);
    let (mut fit, mut even) = (Fit::Natural, 0.0);
    if natural <= room {
        // Spare room goes to the rows, so a short list leaves no gap under it.
        if spare == Spare::ToRows {
            let left = room - natural - 2.0;
            let share = left.max(0.0) / count.max(1) as f32;
            scale.row += share.min(scale.row * 0.4);
        }
    } else {
        fit = Fit::Compressed;
        // What is left goes back to the rows, shortest first, so the pitch evens out.
        let (_, rows) = laid(scale, fit, width, 0.0);
        let heights: Vec<f32> = rows.iter().map(|row| row.height).collect();
        let cap = heights.iter().copied().fold(scale.row, f32::max);
        even = view::even_height(&heights, room - 2.0, cap);
        let (_, rows) = laid(scale, fit, width, even);
        if total(&rows) > room {
            fit = Fit::Summary;
            let (_, rows) = laid(scale, fit, width, 0.0);
            let heights: Vec<f32> = rows.iter().map(|row| row.height).collect();
            let cap = heights.iter().copied().fold(scale.row, f32::max);
            even = view::even_height(&heights, room - f32::from(theme::HAIRLINE_WIDTH), cap);
        }
    }
    // `width` is the scroll column's lane-less width: a list too tall even at its tightest
    // scrolls at that pitch beside its bar, whose lane is always kept (`kit::scroll`).
    let (columns, rows) = laid(scale, fit, width, even);
    let overflow = total(&rows) > room;
    Plan {
        scale,
        columns,
        fit,
        rows,
        room,
        overflow,
    }
}

/// The columns every row of the list shares.
fn columns(run: &Run, scale: RunScale, fit: Fit, width: f32, window: &Window) -> RowColumns {
    let narrow = fit != Fit::Natural || scale.row <= theme::RUN_COMPACT.row;
    let name = |words: &str| {
        f32::from(measure::width_at(
            window,
            Type::Name,
            Some(px(scale.name)),
            words,
        ))
    };
    let longest = run
        .state
        .participants
        .iter()
        .fold(0.0_f32, |longest, p| longest.max(name(&p.name) + 1.0));
    let state = px(scale.mark.max(12.0));
    let state_width = [
        SyncState::FirstRead,
        SyncState::ReadOnly,
        SyncState::Pending,
        SyncState::Stopped,
    ]
    .into_iter()
    .map(|state_of| {
        let status = sync_status(state_of);
        f32::from(
            measure::width_at(window, Type::Heading, Some(state), &status.word)
                + measure::width_at(window, Type::Heading, Some(state), "  "),
        ) + f32::from(theme::ICON_SLOT.min(state * 1.4))
            + f32::from(theme::S)
    })
    .fold((scale.mark * 8.5).clamp(84.0, 118.0), f32::max);
    view::row_columns(to_design(scale), narrow, width, longest, state_width)
}

/// The theme's run scale as the design's.
fn to_design(scale: RunScale) -> crate::design::RunScale {
    crate::design::RunScale {
        row: scale.row,
        name: scale.name,
        hp: scale.hp,
        hp_max: scale.hp_max,
        stat: scale.stat,
        condition: scale.condition,
        mark: scale.mark,
        max_width: scale.max_width,
    }
}

/// The name line's height.
fn name_line(scale: RunScale, fit: Fit) -> f32 {
    scale.name + if fit == Fit::Natural { 4.0 } else { 2.0 }
}

/// One row's layout: its second line folded to fit, its name's lines, its height and where its
/// name line sits.
fn row_plan(
    run: &Run,
    index: usize,
    scale: RunScale,
    columns: &RowColumns,
    fit: Fit,
    even: f32,
    window: &Window,
) -> RowPlan {
    let p = &run.state.participants[index];
    let line = fit_line(run, index, scale, columns, fit, window);
    let name_wraps = f32::from(measure::width_at(
        window,
        Type::Name,
        Some(px(scale.name)),
        &p.name,
    )) > columns.name_width;
    let name_line = name_line(scale, fit);
    let name_extra = if name_wraps { scale.name * 1.25 } else { 0.0 };
    let second_height = scale.condition + 2.0;
    let badge_alone = line.parts.is_empty() && run.row_badge(index).is_some();
    let lines = usize::from(!line.parts.is_empty())
        + usize::from(line.wraps)
        + usize::from(line.badge_below || badge_alone);
    let second = second_height * lines as f32;
    let sync = run.row_sync_word(index);
    let tight = fit != Fit::Natural
        && line.parts.is_empty()
        && !badge_alone
        && sync.is_none()
        && !name_wraps;
    let hairline = f32::from(theme::HAIRLINE_WIDTH);
    let height = if tight {
        view::TIGHT_ROW.min(scale.row).max(even)
    } else if fit != Fit::Natural {
        (name_line + name_extra + second + hairline).max(even)
    } else {
        let top = ((scale.row - name_line - second_height) / 2.0).max(1.0);
        scale.row.max(top + name_line + name_extra + second + 3.0)
    };
    // The current turn's frame is inside the row: its rows keep their words clear of it.
    let height = if !tight && run.current() == Some(index) {
        height + 2.0 * f32::from(theme::FOCUS_WIDTH)
    } else {
        height
    };
    let line_y = if fit != Fit::Natural && lines > 0 {
        let content = name_line + name_extra + second;
        ((height - content) / 2.0).max(hairline / 2.0) + name_line / 2.0
    } else {
        // A row with one line centres it until there is room for two. Then every row shares
        // one place, with or without a second line, so adding one moves no number.
        let basis = match fit {
            Fit::Natural => scale.row,
            Fit::Compressed | Fit::Summary => {
                even.max(height.min(1.0 + name_line + second_height + 1.0))
            }
        };
        if basis < 1.0 + name_line + second_height {
            height / 2.0 - 2.0
        } else {
            ((basis - name_line - second_height) / 2.0).max(1.0) + name_line / 2.0
        }
    };
    RowPlan {
        line,
        name_wraps,
        height,
        line_y,
    }
}

/// The parts of a row's second line, showing `conditions` of its conditions and its changes as
/// `bonuses` says, in this order: the state words when they have no column, Absent, No
/// initiative, Conc, the conditions and the count of the rest, the side, `as <stat block>`, the
/// changes, Loot, the sync state when it has no column, and the save DC.
fn parts(
    run: &Run,
    index: usize,
    inline: bool,
    conditions: usize,
    bonuses: Bonuses,
    short: Option<&str>,
) -> Vec<Part> {
    let c = colors();
    let p = &run.state.participants[index];
    let part = |words: String, color: Hsla, act: Act| Part {
        words,
        color,
        act,
        underline: false,
        mark: false,
        joined: false,
    };
    let mut parts = Vec::new();
    if inline && let Some((icon, word, color)) = hp_word(p.hp, p.max_hp) {
        parts.push(part(
            format!("{icon} {word}"),
            theme::color(color),
            Act::None,
        ));
    }
    if !p.present {
        parts.push(part(
            format!("{} Absent", icons::ABSENT),
            c.muted,
            Act::None,
        ));
    }
    if p.present && p.initiative.is_none() {
        parts.push(Part {
            underline: true,
            ..part("No initiative".into(), c.accent, Act::Initiative)
        });
    }
    let conc = p.concentration.then(|| {
        part(
            format!("{} Conc", icons::CONCENTRATING),
            c.participant.concentrating,
            Act::None,
        )
    });
    parts.extend(conc);
    let order = condition_order(run, index);
    for (shown, &at) in order.iter().take(conditions).enumerate() {
        let short = short.filter(|_| shown == 0);
        parts.extend(condition_parts(&p.conditions[at], at, short));
    }
    if order.len() > conditions {
        parts.push(part(
            crate::design::count(order.len() - conditions, "more", "more").replace(' ', "\u{a0}"),
            c.participant.condition,
            Act::Conditions,
        ));
    }
    if !["party", "enemies"].contains(&p.side.as_str()) {
        parts.push(part(
            crate::design::reader::human_label(&p.side),
            c.side.other,
            Act::None,
        ));
    }
    if p.npc_identity() {
        parts.push(part(
            format!("as {}", p.definition.name),
            c.muted,
            Act::None,
        ));
    }
    let changes = p
        .changes
        .as_ref()
        .filter(|_| p.stat_changes())
        .map(|changes| changes.overrides.table_bonuses(&changes.base));
    if let Some(list) = changes {
        if bonuses == Bonuses::None && !list.is_empty() {
            parts.push(part(
                format!(
                    "{}\u{a0}{}",
                    icons::CHANGED,
                    crate::design::count(list.len(), "change", "changes")
                )
                .replace(' ', "\u{a0}"),
                c.participant.changed,
                Act::Changes,
            ));
        } else {
            parts.push(part(
                format!("{}\u{a0}Changed", icons::CHANGED),
                c.participant.changed,
                Act::None,
            ));
            let shown = match bonuses {
                Bonuses::All => list.len(),
                Bonuses::One => 1,
                Bonuses::None => 0,
            };
            for bonus in list.iter().take(shown) {
                parts.push(part(
                    bonus.replace(' ', "\u{a0}"),
                    c.participant.changed,
                    Act::None,
                ));
            }
            if bonuses == Bonuses::One && list.len() > 1 {
                parts.push(part(
                    crate::design::count(list.len() - 1, "more", "more").replace(' ', "\u{a0}"),
                    c.participant.changed,
                    Act::Changes,
                ));
            }
        }
    }
    if p.loot_ready() {
        parts.push(part(
            format!("{}\u{a0}Loot", icons::LOOT),
            c.accent,
            Act::None,
        ));
    }
    if inline && let Some(sync) = run.row_sync_word(index) {
        let status = sync_status(sync);
        parts.push(Part {
            underline: sync == SyncState::Conflict,
            mark: true,
            ..part(
                format!("{} {}", status.icon, status.word),
                status.color,
                Act::None,
            )
        });
    }
    if let Some(dc) = run.conc_save_dc(index) {
        // R7: right after the row's own `Conc` mark the save says `save DC 10`.
        let beside = parts
            .last()
            .is_some_and(|last| last.color == c.participant.concentrating);
        parts.push(part(
            if beside {
                format!("save DC {dc}")
            } else {
                format!("Conc save DC {dc}")
            },
            c.participant.concentrating,
            Act::None,
        ));
    }
    parts
}

/// The order a row shows its conditions in: the effects first (on the turn's row, one due on
/// this turn leads), then the plain conditions (section 1.4). Each is its place in the row's
/// conditions.
fn condition_order(run: &Run, index: usize) -> Vec<usize> {
    let conditions = &run.state.participants[index].conditions;
    let turn = run.current() == Some(index);
    let mut effects: Vec<usize> = (0..conditions.len())
        .filter(|&at| conditions[at].is_effect())
        .collect();
    if turn {
        effects.sort_by_key(|&at| !conditions[at].due());
    }
    effects.extend((0..conditions.len()).filter(|&at| !conditions[at].is_effect()));
    effects
}

/// An official condition's name (`Poisoned`, `Exhaustion (2)`): it has a rule to open.
fn official(name: &str) -> bool {
    let base = name.split(" (").next().unwrap_or(name).trim();
    crate::encounter_run::effect::CONDITIONS
        .iter()
        .any(|(_, official)| official.eq_ignore_ascii_case(base))
}

/// FD8: words of more than four show their first four and `…`.
fn first_words(words: &str) -> String {
    if words.split_whitespace().count() > 4 {
        let first: Vec<&str> = words.split_whitespace().take(4).collect();
        format!("{}…", first.join(" "))
    } else {
        words.to_owned()
    }
}

/// One condition's parts: its name, then its count after a space (or ` · ` for the words of
/// a time), each its own target. An official condition's name opens its rule; other words
/// open Conditions. A count of uses spends one, a recharge marks it spent or rolls for it, and
/// `time is up` removes the effect. `short` stands for the name in a tight row.
fn condition_parts(condition: &Condition, at: usize, short: Option<&str>) -> Vec<Part> {
    let c = colors();
    let name = condition.name();
    let name_act = if official(name) {
        Act::Rule(name.to_owned())
    } else {
        Act::Conditions
    };
    let part = |words: String, color: Hsla, act: Act, joined: bool| Part {
        words,
        color,
        act,
        underline: false,
        mark: false,
        joined,
    };
    let Some(count) = condition.count() else {
        let words = short.map_or_else(|| first_words(&condition.words()), str::to_owned);
        return vec![part(words, c.participant.condition, name_act, false)];
    };
    let words = short.map_or_else(|| first_words(name), str::to_owned);
    let effect = |act| Act::Effect(act, at, name.to_owned());
    let (words_of_count, color, act) = match condition.time() {
        Some(Time::Rounds { left: 0, .. }) => (
            format!("{} {count}", icons::WARN),
            c.result.warning,
            effect(EffectAct::Remove),
        ),
        Some(Time::Uses { .. }) => (count, c.participant.condition, effect(EffectAct::Spend)),
        Some(Time::Recharge { spent: true, .. }) => (
            format!("{count} {}", icons::DICE),
            c.participant.condition,
            effect(EffectAct::Recharge),
        ),
        Some(Time::Recharge { .. }) => {
            (count, c.participant.condition, effect(EffectAct::Recharge))
        }
        _ => (count, c.participant.condition, Act::Conditions),
    };
    // `Bless 9`, but `Bless · last round`.
    let joined = words_of_count.starts_with(|c: char| c.is_ascii_digit())
        || !matches!(condition.time(), Some(Time::Rounds { .. }));
    vec![
        part(words, c.participant.condition, name_act, false),
        part(words_of_count.replace(' ', "\u{a0}"), color, act, joined),
    ]
}

/// The separator between two parts: ` · ` that keeps to its next word.
const DOT: &str = " ·\u{a0}";

/// The words of `parts` on one line, with their separators.
fn line_text(parts: &[Part]) -> String {
    let mut text = String::new();
    for (at, part) in parts.iter().enumerate() {
        if at > 0 {
            text.push_str(if part.joined { " " } else { DOT });
        }
        text.push_str(&part.words);
    }
    text
}

/// The width of `parts` on one line in `ty` at `size`.
fn line_width(window: &Window, parts: &[Part], ty: Type, size: Pixels) -> f32 {
    f32::from(measure::width_at(window, ty, Some(size), &line_text(parts)))
}

/// The second line folded to its room: the changes give way first
/// (all, then one and a count, then the count), then the conditions down to one and `N more`;
/// then the line takes the small type; then it wraps and the row grows. The roll words follow
/// the line, or take their own line when they have no room after it.
fn fit_line(
    run: &Run,
    index: usize,
    scale: RunScale,
    columns: &RowColumns,
    fit: Fit,
    window: &Window,
) -> Line {
    let p = &run.state.participants[index];
    let narrow = fit != Fit::Natural || scale.row <= theme::RUN_COMPACT.row;
    let role = if narrow { Type::Body } else { Type::Strong };
    let size = px(scale.condition);
    let room = columns.temp_right + columns.bar_inset - columns.name_left;
    let badge = run.row_badge(index);
    let badge_width = badge.as_ref().map_or(0.0, |badge| {
        f32::from(theme::S + measure::width_at(window, Type::Small, None, badge))
    });
    // R8: the roll words keep their room after the line before a condition does.
    let fits = |parts: &[Part], ty: Type, size: Pixels| {
        line_width(window, parts, ty, size) + badge_width <= room
    };
    let mut bonuses = if fit == Fit::Summary {
        Bonuses::One
    } else {
        Bonuses::All
    };
    let mut shown = p.conditions.len();
    let mut short: Option<String> = None;
    let mut parts = parts(run, index, columns.inline_state, shown, bonuses, None);
    let first_effect = condition_order(run, index)
        .first()
        .map(|&at| &p.conditions[at])
        .filter(|condition| condition.count().is_some())
        .map(|condition| condition.name().to_owned());
    loop {
        if fits(&parts, role, size) {
            break;
        }
        if bonuses != Bonuses::None && p.changes.is_some() && p.stat_changes() {
            bonuses = if bonuses == Bonuses::All {
                Bonuses::One
            } else {
                Bonuses::None
            };
        } else if shown == 2
            && short.is_none()
            && let Some(name) = &first_effect
        {
            // Before the first effect is alone with `N more`, its name gives up words so the
            // next condition keeps its place: `Legendary… 2 of 3 · Prone · 1 more`.
            let whole = line_width(window, &parts, role, size);
            let name_width = f32::from(measure::width_at(window, role, Some(size), name));
            let room_for_name = room - badge_width - (whole - name_width);
            let cut = crate::design::words_that_fit_by(name, room_for_name, |words| {
                f32::from(measure::width_at(window, role, Some(size), words))
            });
            if cut == *name || cut == "…" {
                shown -= 1;
            } else {
                short = Some(cut);
                let trial = self::parts(
                    run,
                    index,
                    columns.inline_state,
                    shown,
                    bonuses,
                    short.as_deref(),
                );
                if !fits(&trial, role, size) {
                    short = None;
                    shown -= 1;
                }
            }
        } else if shown > 1 {
            short = None;
            shown -= 1;
        } else {
            break;
        }
        parts = self::parts(
            run,
            index,
            columns.inline_state,
            shown,
            bonuses,
            short.as_deref(),
        );
    }
    let mut line = Line {
        parts,
        ..Line::default()
    };
    let bare = |parts: &[Part], ty: Type, size: Pixels| line_width(window, parts, ty, size) <= room;
    if !bare(&line.parts, role, size) {
        line.small = true;
        if !bare(&line.parts, Type::Small, px(scale.mark)) {
            line.wraps = true;
        }
    }
    let (ty, at) = if line.small {
        (Type::Small, px(scale.mark))
    } else {
        (role, size)
    };
    // R8: the roll words follow the line while they fit there whole; else their own line.
    line.used = line_width(window, &line.parts, ty, at);
    if badge.is_some() && !line.wraps {
        line.badge_below = line.used + badge_width > room;
    }
    line
}

/// The HP readout in a column `width` wide, its numbers on a line `line` high: current in the
/// state's colour, `/max` muted, a bar under it.
fn hp_block(p: &Participant, scale: RunScale, width: Pixels, line: Pixels) -> impl IntoElement {
    let c = colors();
    let state = hp_state(p.hp, p.max_hp);
    let current_color = kit::hp::current_color(state);
    let fill_color = kit::hp::hp_color(state);
    let current = p.hp.map_or_else(|| "?".to_owned(), |hp| hp.to_string());
    let max = p
        .max_hp
        .map_or_else(|| "?".to_owned(), |max| max.to_string());
    let fraction = match (p.hp, p.max_hp) {
        (Some(hp), Some(max)) if max > 0 => (hp as f32 / max as f32).clamp(0.0, 1.0),
        _ => 0.0,
    };
    let temp = match p.max_hp {
        Some(max) if max > 0 && p.temporary_hp > 0 => {
            Some((p.temporary_hp as f32 / max as f32).clamp(0.05, 1.0))
        }
        _ => None,
    };
    let bar_height = px((scale.row / 14.0).clamp(3.0, 5.0));
    let readout = div()
        .flex()
        .flex_row()
        .items_baseline()
        .justify_end()
        .h(line)
        .child(
            div()
                .ty(Type::NumeralLarge)
                .text_size(px(scale.hp))
                .line_height(line)
                .text_color(current_color)
                .child(current),
        )
        .child(
            div()
                .ty(Type::Numeral)
                .text_size(px(scale.hp_max))
                .text_color(c.muted)
                .child(crate::design::fraction("", &max)),
        );
    let bar = div()
        .relative()
        .w_full()
        .h(bar_height)
        .rounded(px(1.0))
        .bg(c.hp.track)
        .child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .h_full()
                .w(gpui::relative(fraction))
                .rounded(px(1.0))
                .bg(fill_color),
        )
        .children(temp.map(|share| {
            div()
                .absolute()
                .top(px(-2.0))
                .left_0()
                .h(px(2.0))
                .w(gpui::relative(share))
                .bg(c.hp.temp)
        }));
    div()
        .flex()
        .flex_col()
        .flex_none()
        .w(width)
        .gap(px(2.0))
        .child(readout)
        .child(bar)
}

/// The row's second line, drawn: each part in its colour, a press where it has one.
fn second_line(
    p: &Participant,
    line: &Line,
    scale: RunScale,
    fit: Fit,
    live: bool,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let narrow = fit != Fit::Natural || scale.row <= theme::RUN_COMPACT.row;
    let (ty, size) = if line.small {
        (Type::Small, px(scale.mark))
    } else if narrow {
        (Type::Body, px(scale.condition))
    } else {
        (Type::Strong, px(scale.condition))
    };
    let mut words = div()
        .flex()
        .flex_row()
        .items_baseline()
        .when(line.wraps, |line| line.flex_wrap())
        .when(!line.wraps, |line| line.whitespace_nowrap())
        .min_w_0();
    for (at, part) in line.parts.iter().enumerate() {
        if at > 0 {
            words = words.child(
                div()
                    .ty(ty)
                    .text_size(size)
                    .text_color(c.muted)
                    .whitespace_nowrap()
                    .child(if part.joined { "\u{a0}" } else { DOT }),
            );
        }
        let id = ElementId::Name(format!("run-row-{}-part-{at}", p.key).into());
        let mut element = div()
            .id(id)
            .ty(if part.mark { Type::Heading } else { ty })
            .text_size(if part.mark { px(scale.mark) } else { size })
            .text_color(part.color)
            .whitespace_nowrap()
            .when(part.underline || (live && part.act != Act::None), |words| {
                words.underline()
            })
            .child(part.words.clone());
        if live && part.act != Act::None {
            let (act, key) = (part.act.clone(), p.key.clone());
            element = element.cursor_pointer().on_click(cx.listener(
                move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                    cx.stop_propagation();
                    cx.notify();
                    let Some(run) = root.app.encounters.run_mut() else {
                        return;
                    };
                    let Some(index) = run.state.participants.iter().position(|p| p.key == key)
                    else {
                        return;
                    };
                    match act.clone() {
                        Act::Conditions => {
                            run.select(index);
                            run.open_conditions_tab();
                        }
                        Act::Changes => {
                            run.select(index);
                            run.panel = crate::encounter_run::Panel::Reference;
                        }
                        Act::Initiative => {
                            run.live_initiative = Some(index);
                            run.focus_field = Some(index);
                            run.setup_inputs.insert(index, String::new());
                        }
                        // Whoever the DM asks about is the one they are about to change.
                        Act::Rule(name) => {
                            run.select(index);
                            let rule = run
                                .content
                                .as_deref()
                                .and_then(|content| content.condition_rule(&name));
                            match rule {
                                Some(rule) => run.lookup_request = Some(rule),
                                None => run.open_conditions_tab(),
                            }
                        }
                        Act::Effect(act, at, name) => {
                            run.select(index);
                            let there = run.state.participants[index]
                                .conditions
                                .get(at)
                                .is_some_and(|condition| condition.name() == name);
                            if there {
                                run.run_command(match act {
                                    EffectAct::Spend => Command::Spend(index, at),
                                    EffectAct::Recharge => Command::Recharge(index, at),
                                    EffectAct::Remove => Command::RemoveCondition(index, at),
                                });
                            }
                        }
                        Act::None => {}
                    }
                },
            ));
            let name = act_name(p, &part.act, &part.words);
            let element = if matches!(part.act, Act::Rule(_) | Act::Effect(..)) {
                element.with_tooltip_words(name.clone())
            } else {
                element
            };
            words = words.child(kit::target(element, name));
        } else {
            words = words.child(kit::target(element, part.words.clone()));
        }
    }
    words.into_any_element()
}

/// The name of a part's press, as its hover and the accessibility tree give it.
fn act_name(p: &Participant, act: &Act, words: &str) -> String {
    let condition = |at: usize| p.conditions.get(at);
    match act {
        Act::Initiative => format!("No initiative for {}", p.name),
        Act::Rule(name) => format!("Rule: {name}"),
        Act::Effect(EffectAct::Remove, _, name) => format!("Remove {name}"),
        Act::Effect(EffectAct::Spend, at, name) => match condition(*at).and_then(Condition::time) {
            Some(Time::Uses { left: 0, .. }) => {
                format!("Give back every use of {name} of {}", p.name)
            }
            _ => format!("Spend one use of {name} of {}", p.name),
        },
        Act::Effect(EffectAct::Recharge, at, name) => {
            match condition(*at).and_then(Condition::time) {
                Some(Time::Recharge {
                    spent: true,
                    recharge,
                }) => {
                    let range = if recharge[0] == recharge[1] {
                        recharge[0].to_string()
                    } else {
                        format!("{}–{}", recharge[0], recharge[1])
                    };
                    format!("Roll 1d6. {range} makes it ready")
                }
                _ => format!("Mark {name} spent"),
            }
        }
        Act::Conditions | Act::Changes | Act::None => words.to_owned(),
    }
}

/// The accessible name of a row: every fact it shows, as `row_name` gives it.
fn row_name(p: &Participant, turn: TurnMark, sync: Option<SyncState>) -> String {
    let display = |value: Option<i64>| value.map_or_else(|| "—".to_owned(), |n| n.to_string());
    let mut parts = vec![
        p.name.clone(),
        p.side.clone(),
        format!(
            "HP {}",
            crate::design::fraction(display(p.hp), display(p.max_hp))
        ),
        format!("Temp {}", p.temporary_hp),
        format!("AC {}", display(p.armor_class)),
        format!("Initiative {}", display(p.initiative)),
    ];
    match turn {
        TurnMark::Current => parts.push("Current turn".into()),
        TurnMark::Next => parts.push("Next up".into()),
        TurnMark::None => {}
    }
    if !p.present {
        parts.push("Absent".into());
    }
    if let Some((_, word, _)) = hp_word(p.hp, p.max_hp) {
        parts.push(word.into());
    }
    if p.concentration {
        parts.push("Concentrating".into());
    }
    parts.extend(
        p.conditions
            .iter()
            .map(crate::encounter_run::Condition::words),
    );
    if p.loot_ready() {
        parts.push("Loot".into());
    }
    if let Some(sync) = sync {
        parts.push(sync_status(sync).word.to_string());
    }
    parts.join(", ")
}

/// One roster row, laid out by the list's `plan` (`at` is its place in the order): the turn
/// bar, the initiative and the turn mark, the side's mark, the picture, the name, AC (or the
/// save owed), temporary HP, HP and its bar, and the state words in their column, on the name
/// line; the second line under the name, reaching under AC and temporary HP.
#[allow(clippy::too_many_arguments)]
pub(super) fn row(
    root: &SplatRoot,
    run: &Run,
    index: usize,
    turn: TurnMark,
    plan: &Plan,
    at: usize,
    use_: Use,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let layout = &plan.rows[at];
    let columns = &plan.columns;
    let scale = plan.scale;
    let live = use_ == Use::Live;
    let current = turn == TurnMark::Current;
    let selected = live && (run.selected == index || run.also.contains(&index));
    let sync = run.row_sync_word(index);
    let narrow = plan.fit != Fit::Natural || scale.row <= theme::RUN_COMPACT.row;
    let text = if p.present {
        c.text
    } else {
        c.participant.absent
    };
    let height = px(layout.height);
    let name_line = px(name_line(scale, plan.fit));
    let line_top = px(layout.line_y) - name_line / 2.0;
    let initiative_words = p
        .initiative
        .map_or_else(|| "—".to_owned(), |n| n.to_string());
    let mark = match turn {
        TurnMark::Current => Some(("TURN", c.turn.current)),
        TurnMark::Next => Some(("NEXT", c.turn.next)),
        TurnMark::None => None,
    };
    let initiative_width = px(columns.initiative_right - 6.0) - theme::XS;
    let editing = live && run.live_initiative == Some(index);
    // The initiative, with the turn mark under it (beside it in narrow columns), or a field
    // while the DM changes it.
    let initiative: AnyElement = if editing {
        let text = run.setup_inputs.get(&index).cloned().unwrap_or_default();
        let name = format!("Initiative for {}", p.name);
        let focus = run.focus_field == Some(index);
        // Escape, a click away, or Enter with no number leaves the total as it was.
        let close = move |root: &mut SplatRoot, cx: &mut Context<SplatRoot>| {
            if let Some(run) = root.app.encounters.run_mut()
                && run.live_initiative == Some(index)
            {
                run.live_initiative = None;
                run.focus_field = None;
            }
            cx.notify();
        };
        // The field keeps the column's place, so the name after it does not move; it is
        // wider than the column, over the gap and the side's mark, to show a whole total.
        div()
            .relative()
            .w(initiative_width)
            .h(name_line)
            .flex_none()
            .child(
                div()
                    .key_context("RunField")
                    .absolute()
                    .left_0()
                    .top((name_line - Density::Read.control()) / 2.0)
                    .w(INITIATIVE_FIELD)
                    .child(
                        kit::TextField::new(
                            ElementId::Name(format!("run-live-initiative-{}", p.key).into()),
                            text,
                        )
                        .name(name)
                        .focus_selected(focus)
                        .on_change(
                            cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                                if let Some(run) = root.app.encounters.run_mut() {
                                    run.setup_inputs.insert(index, super::number_text(words));
                                }
                                cx.notify();
                            }),
                        )
                        .on_enter(super::plain(cx, move |root, _, cx| {
                            if let Some(run) = root.app.encounters.run_mut() {
                                let total = run
                                    .setup_inputs
                                    .get(&index)
                                    .and_then(|text| crate::design::parse_signed(text));
                                if let Some(total) = total
                                    && run.state.participants[index].initiative != Some(total)
                                {
                                    run.run_command(Command::Initiative(index, Some(total)));
                                }
                                run.live_initiative = None;
                            }
                            cx.notify();
                        }))
                        .on_escape(super::plain(cx, move |root, _, cx| close(root, cx)))
                        .on_blur(super::plain(cx, move |root, _, cx| close(root, cx))),
                    ),
            )
            .into_any_element()
    } else {
        let number = div()
            .ty(Type::Numeral)
            .text_size(px(scale.stat))
            .text_color(if current { c.turn.current } else { c.muted })
            .child(initiative_words);
        let word = mark.map(|(word, color)| {
            div()
                .ty(Type::Label)
                .text_size(px(scale.mark))
                .text_color(color)
                .child(word)
        });
        let mut column = div()
            .id(ElementId::Name(
                format!("run-row-{}-initiative", p.key).into(),
            ))
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .justify_end()
            .gap(px(4.0))
            .w(initiative_width)
            .h(name_line)
            .flex_none();
        column = if narrow {
            // `TURN` on the number's line, so a row's height never depends on the turn.
            column.children(word).child(number)
        } else {
            column.child(number).children(word.map(|word| {
                div()
                    .absolute()
                    .right_0()
                    .top(name_line - px(2.0))
                    .child(word)
            }))
        };
        if live && run.state.phase == Phase::Running && (!selected || run.also.is_empty()) {
            let key = p.key.clone();
            // The number's own press: the row under it does not also select, so the keyboard goes to the field, not to Amount.
            let column = column.cursor_pointer().on_click(cx.listener(
                move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                    cx.stop_propagation();
                    let Some(run) = root.app.encounters.run_mut() else {
                        return;
                    };
                    let Some(index) = run.state.participants.iter().position(|p| p.key == key)
                    else {
                        return;
                    };
                    run.live_initiative = Some(index);
                    run.focus_field = Some(index);
                    let total = run.state.participants[index].initiative;
                    run.setup_inputs
                        .insert(index, total.map_or_else(String::new, |n| n.to_string()));
                    cx.notify();
                },
            ));
            kit::target(
                kit::WithTooltip::with_tooltip(column, "Change initiative"),
                format!("Change initiative for {}", p.name),
            )
            .into_any_element()
        } else {
            column.into_any_element()
        }
    };
    let picture = px(columns.picture.min(layout.height - 4.0).max(16.0));
    let art = run.row_art(&root.ctx, p);
    let name = div()
        .w(px(columns.name_width))
        .flex_none()
        .min_h(name_line)
        .flex()
        .items_center()
        .child(
            div()
                .w_full()
                .ty(Type::Name)
                .text_size(px(scale.name))
                .line_height(px(scale.name * 1.25))
                .text_color(text)
                .when(layout.name_wraps, |name| name.line_clamp(2))
                .when(!layout.name_wraps, |name| name.truncate())
                .child(p.name.clone()),
        );
    let display = |value: Option<i64>| value.map_or_else(|| "—".to_owned(), |n| n.to_string());
    let save = live.then(|| run.save_on_row(index)).flatten();
    let (value, label, strong) = match &save {
        Some((ability, total)) => (total.to_string(), ability.clone(), true),
        None => (display(p.armor_class), "AC".to_owned(), false),
    };
    let ac = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_end()
        .gap(px(4.0))
        .w(px(columns.ac_width))
        .h(name_line)
        .flex_none()
        .child(
            div()
                .ty(Type::Small)
                .text_color(if current || strong { c.muted } else { c.faint })
                .child(label),
        )
        .child(
            div()
                .ty(Type::Numeral)
                .text_size(px(scale.stat))
                .text_color(if strong || !p.present { text } else { c.muted })
                .child(value),
        );
    let temp = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_end()
        .gap(px(2.0))
        .w(px(columns.temp_width))
        .h(name_line)
        .flex_none()
        .when(p.temporary_hp > 0, |temp| {
            temp.child(kit::icon(icons::TEMP_HP, Type::Small, c.hp.temp))
                .child(
                    div()
                        .ty(Type::Numeral)
                        .text_size(px(scale.stat))
                        .text_color(c.hp.temp)
                        .child(format!("+{}", p.temporary_hp)),
                )
        });
    // The HP word and sync state take their own column, after the HP, which keeps its place;
    // without room for that column they open the second line.
    let state = (!columns.inline_state).then(|| {
        let mut words = div()
            .flex()
            .flex_col()
            .justify_center()
            .gap(px(2.0))
            .pl(theme::S)
            .w(px(columns.state_width))
            .min_h(name_line)
            .flex_none();
        if let Some((icon, word, color)) = hp_word(p.hp, p.max_hp) {
            let color = if current && word == "HP unknown" {
                c.muted
            } else {
                theme::color(color)
            };
            words = words.child(state_word(icon, word, color, scale));
        }
        if let Some(sync) = sync {
            let status = sync_status(sync);
            words = words.child(state_word_owned(
                status.icon.clone(),
                status.word.clone(),
                status.color,
                scale,
            ));
        }
        words
    });
    let first = div()
        .absolute()
        .left_0()
        .top(line_top)
        .w_full()
        .flex()
        .flex_row()
        .items_start()
        .child(div().w(px(6.0) + theme::XS).flex_none())
        .child(initiative)
        .child(div().w(px(columns.gap)).flex_none())
        .child(
            div()
                .w(px(scale.name * 0.7))
                .h(name_line)
                .flex_none()
                .flex()
                .items_center()
                .child(side_mark(&p.side, scale.name * 0.55)),
        )
        .child(
            div()
                .w(px(columns.picture))
                .h(name_line)
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .child(super::portrait(
                    art,
                    &p.name,
                    picture.min(px(columns.picture)),
                    !p.present,
                )),
        )
        .child(div().w(px(view::PICTURE_GAP)).flex_none())
        .child(name)
        .child(ac)
        .child(temp)
        .child(hp_block(p, scale, px(columns.hp_width), name_line))
        .children(state);
    let name_extra = if layout.name_wraps {
        px(scale.name * 1.25)
    } else {
        px(0.0)
    };
    let condition_line = px(scale.condition + 2.0);
    let second_room = px(columns.temp_right + columns.bar_inset - columns.name_left);
    let second_top = line_top + name_line + name_extra;
    let has_line = !layout.line.parts.is_empty();
    let second = has_line.then(|| {
        div()
            .absolute()
            .left(px(columns.name_left))
            .top(second_top)
            .w(if layout.line.wraps {
                px(columns.hp_right - columns.name_left)
            } else {
                second_room
            })
            .child(second_line(p, &layout.line, scale, plan.fit, live, cx))
    });
    // The roll words: after the second line while they fit there, else on their own line.
    let badge = run.row_badge(index).map(|badge| {
        let words = div()
            .ty(Type::Small)
            .text_color(c.accent)
            .whitespace_nowrap()
            .child(badge);
        let own_line = layout.line.badge_below || !has_line;
        let top = if own_line && has_line {
            second_top + condition_line * if layout.line.wraps { 2.0 } else { 1.0 }
        } else {
            second_top
        };
        let after = if own_line {
            px(0.0)
        } else {
            px(layout.line.used) + theme::S
        };
        div()
            .absolute()
            .top(top)
            .left(px(columns.name_left) + after)
            .h(condition_line)
            .flex()
            .items_center()
            .child(words)
    });
    let mut row = div()
        .id(ElementId::Name(format!("run-row-{}", p.key).into()))
        .relative()
        .w_full()
        .h(height)
        .flex_none()
        .border_b(theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .when(selected && !current, |row| row.bg(c.selected))
        .when(current, |row| {
            row.bg(c.current_row)
                .border(theme::FOCUS_WIDTH)
                .border_color(c.turn.current)
                .rounded(px(2.0))
        })
        .when(live && !selected && !current, |row| {
            row.hover(|s| s.bg(c.hover))
        })
        .child(
            // The current turn's bar at the left edge.
            div()
                .absolute()
                .left_0()
                .top_0()
                .w(px(6.0))
                .h_full()
                .when(current, |bar| bar.bg(c.turn.current)),
        )
        .child(first)
        .children(second)
        .children(badge);
    if selected && !run.also.is_empty() {
        let half = run.halved.contains(&index);
        let key = p.key.clone();
        row = row.child(
            div()
                .id(ElementId::Name(format!("run-row-{}-half", p.key).into()))
                .absolute()
                .left(px(10.0))
                .top(height / 2.0 - px(12.0))
                .size(px(24.0))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .ty(Type::Heading)
                .text_color(c.accent)
                .child(if half {
                    div().child("½")
                } else {
                    kit::icon(icons::OK, Type::Heading, c.accent)
                })
                .on_click(on_run(cx, move |run| {
                    let Some(index) = run.state.participants.iter().position(|p| p.key == key)
                    else {
                        return;
                    };
                    if !run.halved.remove(&index) {
                        run.halved.insert(index);
                    }
                    run.focus_amount = true;
                }))
                .with_tooltip_words("This target saved, so it takes half, rounded down."),
        );
    }
    // A name cut short is whole in hover; the effects' whole words and the save's DC too.
    let effects: Vec<String> = p
        .conditions
        .iter()
        .filter(|condition| condition.is_effect())
        .map(crate::encounter_run::Condition::words)
        .chain(
            run.conc_save_dc(index)
                .map(|dc| format!("Conc save DC {dc}")),
        )
        .collect();
    let side = if effects.is_empty() {
        p.side.clone()
    } else {
        format!("{}\n{}", p.side, effects.join(" · "))
    };
    let row = row.with_tooltip_words(format!("{}\n{side}", p.name));
    let name_words = row_name(p, turn, sync);
    if live {
        let key = p.key.clone();
        let row = row.cursor_pointer().on_click(cx.listener(
            move |root: &mut SplatRoot, event: &ClickEvent, _, cx| {
                let Some(run) = root.app.encounters.run_mut() else {
                    return;
                };
                let Some(index) = run.state.participants.iter().position(|p| p.key == key) else {
                    return;
                };
                // A click away from a row's initiative field closes it, as Escape does (a
                // click on a row takes no focus, so the field would not lose it).
                if run.live_initiative.is_some_and(|open| open != index) {
                    run.live_initiative = None;
                }
                let modifiers = event.modifiers();
                let ctrl = modifiers.secondary(cx);
                let mode = if ctrl {
                    Some(Mode::Disadvantage)
                } else if modifiers.shift {
                    Some(Mode::Advantage)
                } else {
                    None
                };
                if !run.target(index, mode) {
                    if ctrl {
                        run.toggle_target(index);
                    } else {
                        run.select(index);
                    }
                }
                cx.notify();
            },
        ));
        let menu_id = ElementId::Name(format!("run-row-menu-{}", p.key).into());
        let sections = run.row_menu_sections(index);
        let asking = run
            .asking
            .as_ref()
            .filter(|asking| asking.row == index)
            .cloned();
        let row = if !sections.is_empty() || asking.is_some() {
            let opener = menu_id.clone();
            row.on_mouse_down(MouseButton::Right, move |_, window, cx| {
                kit::menu::open_popup(opener.clone(), window, cx);
            })
        } else {
            row
        };
        let menu = row_menu(&menu_id, index, sections, asking, cx);
        let row = kit::target(kit::target(row, name_words), p.name.clone());
        return div()
            .flex()
            .flex_col()
            .w_full()
            .flex_none()
            .child(row)
            .children(menu)
            .into_any_element();
    }
    kit::target(row, name_words).into_any_element()
}

/// A state word beside the HP: `♡ Bloodied`, `0 HP`, a sync state.
fn state_word(
    icon: &'static str,
    word: &'static str,
    color: Hsla,
    scale: RunScale,
) -> impl IntoElement {
    state_word_owned(
        SharedString::new_static(icon),
        SharedString::new_static(word),
        color,
        scale,
    )
}

fn state_word_owned(
    icon: SharedString,
    word: SharedString,
    color: Hsla,
    scale: RunScale,
) -> impl IntoElement {
    let size = px(scale.mark.max(12.0));
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(3.0))
        .text_color(color)
        .child(div().icon_font(Type::Small).text_size(size).child(icon))
        .child(
            div()
                .ty(Type::Heading)
                .text_size(size)
                .whitespace_nowrap()
                .child(word),
        )
}

/// The row's menu: the armed action's rolls against it, a reroll with inspiration, or the
/// question that waits on it (`row_menu`). Open from a right-click, or by itself while it asks.
fn row_menu(
    id: &ElementId,
    index: usize,
    sections: Vec<(String, Vec<(String, RowChoice)>)>,
    asking: Option<crate::encounter_run::rolling::Asking>,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let c = colors();
    let open = kit::menu::is_open(id, cx) || asking.is_some();
    if !open {
        return None;
    }
    let mut content = div().flex().flex_col().min_w(px(240.0));
    let choice = |content: gpui::Div,
                  at: String,
                  words: String,
                  choice: RowChoice,
                  cx: &mut Context<SplatRoot>| {
        content.child(
            kit::Button::quiet(ElementId::Name(at.into()), words)
                .plain()
                .density(Density::Read)
                .full_width()
                .on_click(on_run(cx, move |run| run.row_choice(index, choice.clone()))),
        )
    };
    if let Some(asking) = asking {
        content = content.child(
            div()
                .px(theme::S)
                .py(theme::XS)
                .max_w(px(320.0))
                .child(kit::text(Type::Body, c.text, asking.question.clone())),
        );
        for (at, (words, _)) in asking.choices.iter().enumerate() {
            content = choice(
                content,
                format!("run-ask-{index}-{at}"),
                words.clone(),
                RowChoice::Damage(at),
                cx,
            );
        }
        content = choice(
            content,
            format!("run-ask-{index}-cancel"),
            "Cancel".into(),
            RowChoice::Cancel,
            cx,
        );
    } else {
        for (section, (heading, items)) in sections.into_iter().enumerate() {
            content = content.child(div().px(theme::S).py(theme::XS).child(kit::text(
                Type::Small,
                c.muted,
                heading,
            )));
            for (at, (words, row_choice)) in items.into_iter().enumerate() {
                content = choice(
                    content,
                    format!("run-menu-{index}-{section}-{at}"),
                    words,
                    row_choice,
                    cx,
                );
            }
        }
    }
    Some(kit::menu::layer_for(id, content).into_any_element())
}

/// Hover words on an element with an id.
trait TooltipWords: kit::WithTooltip {
    fn with_tooltip_words(self, words: impl Into<SharedString>) -> Self {
        self.with_tooltip(words)
    }
}
impl<T: kit::WithTooltip> TooltipWords for T {}
