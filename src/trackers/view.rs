//! The Trackers view of the Lookup panel: the Due rows, then the pinned trackers in the note's
//! order, then `More trackers`, in one scroll area; the results sit in the panel's message
//! footer. The view draws from the ledger as read, and every press is one act.
use super::{
    Rules, Says, State, TrackerFault, Trackers, Unrecorded, ledger::Entry, ledger::NoticeRef,
    mark_of,
};
use crate::design::outcome::{
    Banner, Fault, FaultMessage, KeptOutcome, MessageChoice, Outcome, PagePlaceholder, Pressed,
    Said, Sentence, UndoStep, Verb,
};
use crate::design::{self, Action, Density, Type, colors, icons};
use eframe::egui;
use std::collections::HashSet;

/// What the view keeps between frames: a typed amount and the DM's unticked proposals.
#[derive(Default)]
pub(crate) struct ViewState {
    /// A typed amount: the tracker, its instance, the action's label, and the words so far.
    asking: Option<(String, Option<String>, String, String)>,
    focus_asking: bool,
    /// Proposed changes the DM unticked, by notice. Ticked is the default.
    unticked: HashSet<(NoticeRef, usize)>,
    /// The act pressed in this frame. The shell applies it after Lookup drew, once it has read
    /// the day count again, so an act uses the day at the click.
    pressed: Option<Act>,
    /// The quick action pressed last, and where it was, for a few passes after the press.
    keep: Option<Keep>,
    /// Blank room after the trackers, so the scroll can hold a pressed action in its place
    /// near the end of the view. It shrinks as the DM scrolls back up.
    room: f32,
    /// A name wider than its column, as shortened for that width, so a frame with no change
    /// does not shorten it again.
    fitted: std::collections::HashMap<(String, u32), String>,
    /// How many times a name was shortened.
    #[cfg(test)]
    shortened: usize,
    /// The panel's columns, with the trackers' revision and the width they were measured for.
    columns: Option<(u64, u32, Columns)>,
    /// The widest value slot this panel needed since its definitions were read: an Undo that
    /// takes `1000` back to `999` keeps the room, so no action moves back and forth. It ends
    /// when the definitions are read again (`Trackers::rules` replaced) or the campaign changes.
    pub(super) widest_value: f32,
    /// How many times the columns were measured.
    #[cfg(test)]
    pub(super) measured: usize,
}

/// The columns that every tracker line of the panel shares, so the values end at one edge
/// and the first actions start at one edge, whatever the values are (FE1, AE2).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Columns {
    /// The name column, from the line's start, with the indent of a tracker with `per`.
    name: f32,
    /// The value column: as wide as the widest value slot of the panel.
    value: f32,
    /// Does every line's first quick action fit after the value column? Then each line's
    /// actions follow its value, all from one edge, and wrap to the line's start; else every
    /// line has its actions on the rows under it, from its name's edge, so no line's first
    /// action starts at another edge.
    beside: bool,
}

/// The fewest digits a value slot holds, so a value that gains a digit moves no action.
const SLOT_DIGITS: usize = 3;

/// The text a value's slot is measured on: its digits and its goal's, at least
/// `SLOT_DIGITS`, as zeros. The digits are tabular, so every value up to that many digits
/// fits the slot exactly.
fn value_slot(value: i64, goal: Option<i64>) -> String {
    let digits = |n: i64| n.unsigned_abs().to_string().len();
    let wide = digits(value).max(goal.map_or(0, digits)).max(SLOT_DIGITS);
    let sign = if value < 0 { "-" } else { "" };
    let zeros = "0".repeat(wide);
    match goal {
        Some(goal) => format!("{sign}{zeros} / {goal}"),
        None => format!("{sign}{zeros}"),
    }
}

/// Measure the panel's columns: every line of every tracker that can be used, pinned or not,
/// so opening `More trackers` moves no pinned value. Only `columns` calls it, when the
/// trackers' revision or the width changed.
fn measure_columns(ui: &egui::Ui, rules: &Rules, state: &State, view: &mut ViewState) -> Columns {
    #[cfg(test)]
    {
        view.measured += 1;
    }
    let name_font = Type::Body.font(ui);
    let value_font = Type::Numeral.font(ui);
    let mut name: f32 = 0.0;
    let mut value: f32 = 0.0;
    let mut first: f32 = 0.0;
    for tracker in &rules.trackers {
        let Ok(def) = &tracker.def else { continue };
        if let Some(quick) = def.quick.first() {
            first = first.max(
                Action::secondary(&quick.label)
                    .density(Density::Read)
                    .natural_width(ui),
            );
        }
        let indent = if def.per.is_empty() { 0.0 } else { design::L };
        for instance in def.instances() {
            let line = instance.unwrap_or(&def.name);
            name = name.max(indent + design::number_width(ui, line, &name_font));
            let slot = value_slot(state.value(rules, &tracker.key, instance), def.goal);
            value = value.max(design::number_width(ui, &slot, &value_font));
        }
    }
    let name = name.min(ui.available_width() * NAME_SHARE);
    // The value column never narrows while the definitions stay (`widest_value`).
    let value = value.max(view.widest_value);
    view.widest_value = value;
    // The line's parts and the gaps between them: name, S, value, S, then the actions.
    let used = name + value + 2.0 * design::S + 4.0 * ui.spacing().item_spacing.x;
    Columns {
        name,
        value,
        beside: first <= ui.available_width() - used,
    }
}

/// The panel's columns, measured again only when the trackers' revision or the width changed.
fn columns(
    ui: &egui::Ui,
    rules: &Rules,
    state: &State,
    view: &mut ViewState,
    revision: u64,
) -> Columns {
    let width = ui.available_width().to_bits();
    match view.columns {
        Some((seen, at, columns)) if seen == revision && at == width => columns,
        _ => {
            let columns = measure_columns(ui, rules, state, view);
            view.columns = Some((revision, width, columns));
            columns
        }
    }
}

/// How many passes after a press the view still holds the pressed action in place. The Due
/// list can settle over more than one: a scroll bar that comes with it narrows the words.
const KEEP_PASSES: u64 = 6;

/// A pressed quick action: its line and label, its top on screen at the press, the pass of the
/// press, its top as drawn in the pass now, and the height of the view's content at the press.
struct Keep {
    place: (String, Option<String>, String),
    top: f32,
    pass: u64,
    now: Option<f32>,
    content: Option<f32>,
}

impl ViewState {
    /// The amount field again, with the number typed for an act that recorded nothing.
    pub(super) fn ask_again(
        &mut self,
        key: &str,
        instance: Option<&str>,
        label: &str,
        amount: i64,
    ) {
        self.asking = Some((
            key.to_owned(),
            instance.map(str::to_owned),
            label.to_owned(),
            amount.to_string(),
        ));
        self.focus_asking = true;
    }

    /// Hold this quick action in place on screen for the passes after its press.
    fn hold(&mut self, ui: &egui::Ui, key: &str, instance: &Option<String>, label: &str, top: f32) {
        self.keep = Some(Keep {
            place: (key.to_owned(), instance.clone(), label.to_owned()),
            top,
            pass: ui.ctx().cumulative_pass_nr(),
            now: None,
            content: None,
        });
    }

    /// The words in the amount field, while it is open.
    #[cfg(test)]
    pub(super) fn typed(&self) -> Option<&str> {
        self.asking.as_ref().map(|(_, _, _, typed)| typed.as_str())
    }
}

/// The empty state's route to the switches of the house rules in play.
pub(crate) const RULES_ROUTE: &str = "Campaign → Rules and runs";

/// The Due list's words after the Timeline's own, while the day count is not known.
pub(crate) const DAY_MARKS_WAIT: Sentence =
    crate::sentence!("No day mark shows until the Timeline is read.");

/// One act the DM asked for in this frame.
pub(crate) enum Act {
    Press {
        key: String,
        instance: Option<String>,
        label: String,
        asked: Option<i64>,
    },
    Roll(super::Notice, String),
    Done(super::Notice, Vec<bool>),
    Dismiss(super::Notice),
    DismissAll(super::Notice),
    Undo(Vec<Entry>),
    AddToTimeline(String, String),
    UndoAdd(String),
    ReadAgain,
}

impl Trackers {
    /// Does a typed amount hold Escape? Then Escape ends the amount, not Lookup.
    pub fn takes_escape(&self) -> bool {
        self.view.asking.is_some()
    }

    /// Was an act pressed in the view this frame?
    pub fn pressed(&self) -> bool {
        self.view.pressed.is_some()
    }

    /// Apply the act pressed in the view. The shell calls it after `follow_clock`, so an act
    /// on a day notice reads the day count's one answer, also when it is not known.
    pub fn apply_pressed(&mut self) {
        if let Some(act) = self.view.pressed.take() {
            apply(self, act);
        }
    }

    /// `Done` pressed on this row, as a click in the view leaves it for the shell.
    #[cfg(test)]
    pub(crate) fn press_done_in_view(&mut self, notice: &super::Notice, ticked: Vec<bool>) {
        self.view.pressed = Some(Act::Done(notice.clone(), ticked));
    }
}

/// The message footer, then the empty state, a file that cannot be read, or the trackers.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub fn show(ui: &mut egui::Ui, trackers: &mut Trackers, from_top: bool) {
    let mut act = None;
    // The footer comes first: why an act recorded nothing stays in view, also when the row it
    // was pressed on went away.
    let bottom = footer(ui, trackers, &mut act);
    if !trackers.in_play() {
        ui.add_space(design::S);
        design::empty_state(
            ui,
            icons::BELL,
            "No house rules in play",
            "Turn one on to see its trackers here.",
            |ui| {
                // One route has one form: a bordered button, as `Campaign → Packages` is.
                if Action::secondary(RULES_ROUTE).show(ui).clicked() {
                    trackers.wants_rules = true;
                }
            },
        );
        trackers.view.pressed = act;
        return;
    }
    // What is due is in view when the view opens.
    let mut area = egui::ScrollArea::vertical()
        .id_salt("trackers_view")
        .max_height((bottom - ui.cursor().top()).max(0.0))
        .auto_shrink([false, false]);
    if from_top {
        area = area.vertical_scroll_offset(0.0);
        trackers.view.keep = None;
    }
    let room = trackers.view.room;
    let output = area.show(ui, |ui| {
        body(ui, trackers, &mut act);
        ui.add_space(room);
    });
    hold_pressed_place(ui, &mut trackers.view, output);
    trackers.view.pressed = act;
}

/// A press that raises or ends a Due row changes the list's height above the trackers. In the
/// passes after the press the view scrolls by as much, so the pressed action is where it was and
/// a second press of a still pointer presses it again. It acts only while the content's height
/// differs from the press's; the DM's own wheel or drag, or a view opened again, ends it. The
/// room after the trackers is only what the scroll needs to hold it there.
fn hold_pressed_place(
    ui: &egui::Ui,
    view: &mut ViewState,
    output: egui::scroll_area::ScrollAreaOutput<()>,
) {
    let mut state = output.state;
    let pass = ui.ctx().cumulative_pass_nr();
    let content = output.content_size.y - view.room;
    let scrolled = ui.input(|i| {
        i.raw_scroll_delta != egui::Vec2::ZERO
            || i.smooth_scroll_delta != egui::Vec2::ZERO
            || i.pointer.is_decidedly_dragging()
    });
    view.keep
        .take_if(|keep| scrolled || pass > keep.pass + KEEP_PASSES);
    if let Some(keep) = view.keep.as_mut()
        && (*keep.content.get_or_insert(content) - content).abs() > 0.5
        && let Some(now) = keep.now.take()
        && (now - keep.top).abs() > 0.5
    {
        let offset = (state.offset.y + now - keep.top).max(0.0);
        let end = output.content_size.y - output.inner_rect.height();
        view.room += (offset - end).max(0.0);
        state.offset.y = offset;
        state.store(ui.ctx(), output.id);
        ui.ctx()
            .request_discard("a press moved the Due list above it");
        return;
    }
    view.room = (state.offset.y + output.inner_rect.height() - content).max(0.0);
}

fn apply(trackers: &mut Trackers, act: Act) {
    match act {
        Act::Press {
            key,
            instance,
            label,
            asked,
        } => trackers.press(&key, instance.as_deref(), &label, asked),
        Act::Roll(notice, table) => trackers.roll_notice(&notice, &table),
        Act::Done(notice, ticked) => {
            trackers.done(&notice, &ticked);
            if trackers.reason.is_none() {
                trackers.view.unticked.retain(|(id, _)| *id != notice.id);
            }
        }
        Act::Dismiss(notice) => trackers.dismiss(&notice),
        Act::DismissAll(notice) => trackers.dismiss_all(&notice),
        Act::Undo(entries) => trackers.undo(&entries),
        Act::AddToTimeline(entry, label) => trackers.wants_timeline = Some((entry, label)),
        Act::UndoAdd(event) => {
            trackers.added = None;
            trackers.wants_undo_add = Some(event);
        }
        Act::ReadAgain => {
            trackers.baseline = None;
            trackers.read_disk();
        }
    }
}

#[allow(
    clippy::disallowed_methods,
    reason = "the view’s body cannot show: a page placeholder; a member’s problem: a row fault; the unknown day count: a banner"
)]
fn body(ui: &mut egui::Ui, trackers: &mut Trackers, act: &mut Option<Act>) {
    if let Some(error) = &trackers.unreadable {
        let path = trackers.path();
        let fault = TrackerFault::LedgerUnreadable(error.clone());
        let hover = path.as_ref().map(|path| path.display().to_string());
        let mut placeholder =
            PagePlaceholder::new(&fault).choice(MessageChoice::Choice("Try again"));
        if let Some(hover) = &hover {
            placeholder = placeholder.choice(MessageChoice::Link {
                word: "Show file",
                hover: Some(hover),
            });
        }
        match placeholder.show(ui) {
            Some(Pressed::Choice(0)) => *act = Some(Act::ReadAgain),
            Some(Pressed::Choice(_)) => {
                if let Some(path) = path
                    && let Ok(url) = reqwest::Url::from_file_path(&path)
                {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url.to_string()));
                }
            }
            _ => {}
        }
        return;
    }
    let Trackers {
        rules,
        state,
        view,
        problems,
        revision,
        ..
    } = trackers;
    // A definition that cannot be read shows no value, never an old one.
    if let Some(error) = &rules.unreadable {
        let _ = PagePlaceholder::new(&TrackerFault::NotesUnreadable(error.clone())).show(ui);
        return;
    }
    // A day count that is not known shows no day notice, never an old one, and says why.
    if let Some(why) = &state.clock_not_read {
        design::outcome::banner(ui, Banner::Fault(&TrackerFault::ClockNotRead(why.clone())));
    }
    if !state.due.is_empty() {
        design::group_heading(ui, "Due", state.due.len());
        for notice in &state.due {
            notice_row(ui, rules, view, notice, act);
        }
        ui.add_space(design::S);
    }
    let usable = |pin: bool| {
        rules
            .trackers
            .iter()
            .filter_map(move |t| t.def.as_ref().ok().map(|def| (t.key.as_str(), def)))
            .filter(move |(_, def)| def.pin == pin)
    };
    let columns = columns(ui, rules, state, view, *revision);
    if usable(true).next().is_some() {
        design::group_heading(ui, "Pinned", 0);
        for (key, def) in usable(true) {
            tracker_lines(ui, rules, state, view, columns, key, def, act);
        }
    }
    let more = usable(false).count();
    if more > 0 {
        ui.add_space(design::S);
        design::section_with_id(
            ui,
            "trackers_more",
            design::group_heading_text(ui, "More trackers", more),
            |ui| {
                for (key, def) in usable(false) {
                    tracker_lines(ui, rules, state, view, columns, key, def, act);
                }
            },
        );
    }
    // A member that cannot be used is reported, and the others still work. So are a row of the
    // ledger that counts for nothing and a value below its `min`.
    for (_, problem) in problems.iter() {
        design::outcome::row_fault(ui, &TrackerFault::Problem(problem.clone()));
    }
}

/// A Due row: its words, its rolls with their results, its proposed changes, and `Done` with
/// the row's menu at the end.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn notice_row(
    ui: &mut egui::Ui,
    rules: &Rules,
    view: &mut ViewState,
    notice: &super::Notice,
    act: &mut Option<Act>,
) {
    let Some(mark) = mark_of(rules, notice) else {
        return;
    };
    let words = notice_words(rules, notice, mark);
    let short = notice.name(rules);
    let ends_words = mark.tables.is_empty() && mark.changes.is_empty();
    if ends_words {
        words_ending_with(ui, &words, finish_width(ui, notice), |ui| {
            finish(ui, &short, notice, mark, view, act)
        });
    } else {
        ui.add(egui::Label::new(Type::Body.rich(ui, &words)).wrap());
    }
    if !mark.tables.is_empty() {
        ui.horizontal_wrapped(|ui| {
            for table in mark.tables {
                let key = table.key();
                let rolled = notice.rolls.iter().filter(|r| r.table == key).count();
                let each_rolled = !table.names().is_empty() && rolled >= table.names().len();
                let label = format!("Roll {}", rules.table_name(key));
                if Action::secondary(&label)
                    .icon(icons::DICE)
                    .density(Density::Read)
                    .blocked(each_rolled.then_some("Each name has its roll."))
                    .show(ui)
                    .clicked()
                {
                    *act = Some(Act::Roll(notice.clone(), key.to_owned()));
                }
                if each_rolled {
                    design::blocked_reason(ui, Some("Each name has its roll."));
                }
            }
        });
        // The results are listed under the buttons, one line each, as one block of words.
        if !notice.rolls.is_empty() {
            let lines: Vec<String> = notice
                .rolls
                .iter()
                .map(|roll| {
                    let name = roll
                        .name
                        .as_deref()
                        .unwrap_or_else(|| rules.table_name(&roll.table));
                    format!("{name}: {} · {}", roll.value, roll.row)
                })
                .collect();
            egui::Frame::NONE
                .inner_margin(egui::Margin {
                    left: design::L as i8,
                    ..egui::Margin::ZERO
                })
                .show(ui, |ui| {
                    ui.add(egui::Label::new(Type::Body.rich(ui, lines.join("\n"))).wrap());
                });
        }
    }
    for (at, change) in mark.changes.iter().enumerate() {
        let key = change.tracker.as_deref().unwrap_or(&notice.key);
        let name = rules.def(key).map_or(key, |def| def.name.as_str());
        let name = match &change.instance {
            Some(line) => format!("{name} ({line})"),
            None => name.to_owned(),
        };
        let amount = match (&change.add, change.set) {
            (_, Some(set)) => format!("set to {set}"),
            (Some(super::Amount::Number(n)), None) => format!("{n:+}"),
            (Some(super::Amount::Words(words)), None) => format!("+{words}"),
            (None, None) => String::new(),
        };
        let slot = (notice.id.clone(), at);
        let mut on = !view.unticked.contains(&slot);
        if ui.checkbox(&mut on, format!("{name} {amount}")).changed() {
            if on {
                view.unticked.remove(&slot);
            } else {
                view.unticked.insert(slot);
            }
        }
    }
    if !ends_words {
        right_line(ui, |ui| finish(ui, &short, notice, mark, view, act));
    }
    ui.add_space(design::XS);
}

/// A Due row's words. A mark at a value names its line: `Old Guard: First seal.` A day mark
/// names its day, and a row for several days says how many and which comes next, so an older
/// period says so before it runs: `End of month ×3. Next: Day 32. Move the rival powers.`
fn notice_words(rules: &Rules, notice: &super::Notice, mark: Says<'_>) -> String {
    match (&notice.id, notice.days.as_slice(), &notice.instance) {
        (NoticeRef::Day { day, .. }, [_], _) => format!("Day {day}: {}", mark.say),
        (NoticeRef::Day { mark: key, day }, days, _) => format!(
            "{} ×{}. Next: Day {day}. {}",
            rules.day_title(key),
            days.len(),
            mark.say
        ),
        (_, _, Some(name)) => format!("{name}: {}", mark.say),
        (_, _, None) => mark.say.to_owned(),
    }
}

/// The words, with `add` at the right end of their last line when it fits there, and else on
/// a line of its own at the right. The last line is its own label, so nothing overlaps words.
pub(super) fn words_ending_with(
    ui: &mut egui::Ui,
    words: &str,
    trailing: f32,
    add: impl FnOnce(&mut egui::Ui),
) {
    let width = ui.available_width();
    let galley = ui
        .painter()
        .layout(words.to_owned(), Type::Body.font(ui), colors().text, width);
    let gap = ui.spacing().item_spacing.x;
    let fits = galley
        .rows
        .last()
        .is_some_and(|row| width - row.rect().width() >= trailing + gap);
    if !fits {
        ui.add(egui::Label::new(Type::Body.rich(ui, words)).wrap());
        right_line(ui, add);
        return;
    }
    let before: usize = galley.rows[..galley.rows.len() - 1]
        .iter()
        .map(|row| row.char_count_including_newline())
        .sum();
    let split = words
        .char_indices()
        .nth(before)
        .map_or(words.len(), |(at, _)| at);
    let (head, last) = words.split_at(split);
    if head.trim().is_empty() {
        ui.horizontal(|ui| {
            ui.label(Type::Body.rich(ui, last.trim()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), add);
        });
        return;
    }
    // The last line keeps the words' pitch: it follows the line above with no gap, and its
    // actions start at its top, so they reach below it and never over the words above.
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.add(egui::Label::new(Type::Body.rich(ui, head.trim_end())).wrap());
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.label(Type::Body.rich(ui, last.trim()));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), add);
        });
    });
}

/// A line of one control's height, laid out from its right end.
fn right_line(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), Density::Read.control()),
        egui::Layout::right_to_left(egui::Align::Center),
        add,
    );
}

/// The row's last action: `Done`, which applies the ticked changes and ends one notice. On a
/// day mark's row of several days it names the day it ends, the oldest: `Done: Day 32`.
fn finish_label(notice: &super::Notice) -> String {
    match (&notice.id, notice.days.len()) {
        (NoticeRef::Day { day, .. }, 2..) => format!("Done: Day {day}"),
        _ => "Done".to_owned(),
    }
}

/// `Done` and the row's menu, with the gap between them.
fn finish_width(ui: &egui::Ui, notice: &super::Notice) -> f32 {
    Action::quiet(&finish_label(notice))
        .density(Density::Read)
        .natural_width(ui)
        + ui.spacing().item_spacing.x
        + Density::Read.control()
}

/// The row's menu with `Dismiss` (and `Dismiss all` on a row of several days), then `Done`,
/// laid out from the right.
fn finish(
    ui: &mut egui::Ui,
    short: &str,
    notice: &super::Notice,
    mark: Says<'_>,
    view: &ViewState,
    act: &mut Option<Act>,
) {
    design::row_menu(ui, short, |ui| {
        if Action::quiet("Dismiss")
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            *act = Some(Act::Dismiss(notice.clone()));
            ui.close();
        }
        if notice.days.len() > 1
            && Action::quiet("Dismiss all")
                .density(Density::Read)
                .show(ui)
                .clicked()
        {
            *act = Some(Act::DismissAll(notice.clone()));
            ui.close();
        }
    });
    let finish = finish_label(notice);
    let name = format!("Done: {short}");
    if Action::quiet(&finish)
        .density(Density::Read)
        .name(&name)
        .show(ui)
        .clicked()
    {
        let ticked = (0..mark.changes.len())
            .map(|at| !view.unticked.contains(&(notice.id.clone(), at)))
            .collect();
        *act = Some(Act::Done(notice.clone(), ticked));
    }
}

/// A tracker's lines: one line, or a heading and one line for each instance. Every line stands
/// in the panel's columns.
#[allow(clippy::too_many_arguments)]
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn tracker_lines(
    ui: &mut egui::Ui,
    rules: &Rules,
    state: &State,
    view: &mut ViewState,
    columns: Columns,
    key: &str,
    def: &super::Definition,
    act: &mut Option<Act>,
) {
    let value_text = |instance: Option<&str>| {
        let value = state.value(rules, key, instance);
        match def.goal {
            Some(goal) => format!("{value} / {goal}"),
            None => value.to_string(),
        }
    };
    let indent = if def.per.is_empty() {
        0.0
    } else {
        ui.label(Type::Body.rich(ui, &def.name));
        design::L
    };
    // Names and values stand in the panel's columns, so the values read down one edge and the
    // first actions start at one edge. A long name is cut at a whole word, and its hover keeps it.
    let name_width = (columns.name - indent).max(0.0);
    let top = egui::Layout::left_to_right(egui::Align::Min);
    for instance in def.instances() {
        let head = |ui: &mut egui::Ui, view: &mut ViewState| {
            ui.add_space(indent);
            let height = Density::Read.control();
            let name = instance.unwrap_or(&def.name);
            name_cell(ui, view, name, egui::vec2(name_width, height));
            ui.add_space(design::S);
            let (rect, _) =
                ui.allocate_exact_size(egui::vec2(columns.value, height), egui::Sense::hover());
            design::number(
                ui.painter(),
                rect.right(),
                rect.center().y,
                &value_text(instance),
                &Type::Numeral.font(ui),
                colors().text,
            );
        };
        if columns.beside {
            // The actions follow the value, and the ones that do not fit wrap to the line's
            // start, so a wrapped row holds as many as the panel's width allows.
            ui.horizontal_wrapped(|ui| {
                head(ui, view);
                ui.add_space(design::S);
                quick_actions(ui, view, key, def, instance, act);
            });
        } else {
            // A first action does not fit after the values: every line has its actions on the
            // rows under it, from its name's edge.
            ui.with_layout(top, |ui| head(ui, view));
            if !def.quick.is_empty() {
                ui.with_layout(top, |ui| {
                    ui.add_space(indent);
                    ui.horizontal_wrapped(|ui| quick_actions(ui, view, key, def, instance, act));
                });
            }
        }
    }
    ui.add_space(design::XS);
}

/// A line's quick actions, or the amount field in the place of the one that asks.
fn quick_actions(
    ui: &mut egui::Ui,
    view: &mut ViewState,
    key: &str,
    def: &super::Definition,
    instance: Option<&str>,
    act: &mut Option<Act>,
) {
    let place = (key.to_owned(), instance.map(str::to_owned));
    for quick in &def.quick {
        let here = view
            .asking
            .as_ref()
            .is_some_and(|(k, i, l, _)| *k == place.0 && *i == place.1 && *l == quick.label);
        if here {
            // The field takes exactly its action's column, so nothing after it moves.
            let width = Action::secondary(&quick.label)
                .density(Density::Read)
                .natural_width(ui);
            amount_field(ui, view, &quick.label, width, act);
            continue;
        }
        let response = Action::secondary(&quick.label)
            .density(Density::Read)
            .name(&match instance {
                Some(name) => format!("{}: {name}", quick.label),
                None => quick.label.clone(),
            })
            .show(ui);
        if let Some(keep) = view.keep.as_mut().filter(|keep| {
            (&keep.place.0, &keep.place.1, &keep.place.2) == (&place.0, &place.1, &quick.label)
        }) {
            keep.now = Some(response.rect.top());
        }
        if response.clicked() {
            view.hold(ui, &place.0, &place.1, &quick.label, response.rect.top());
            // Read on the click only: a dice amount is parsed to tell it from `ask`.
            if quick.asks() {
                view.asking = Some((
                    place.0.clone(),
                    place.1.clone(),
                    quick.label.clone(),
                    String::new(),
                ));
                view.focus_asking = true;
            } else {
                *act = Some(Act::Press {
                    key: key.to_owned(),
                    instance: place.1.clone(),
                    label: quick.label.clone(),
                    asked: None,
                });
            }
        }
    }
}

/// The most of a tracker's line that its name column takes, so the value and an action keep room.
const NAME_SHARE: f32 = 0.4;

/// A name in its column: exactly `size`, whatever the name's own width. A name wider than the
/// column ends at a whole word with `…`, or is `…` alone when its first word does not fit; its
/// hover says it whole. Only a name that does not fit is shortened, once for each width.
fn name_cell(ui: &mut egui::Ui, view: &mut ViewState, name: &str, size: egui::Vec2) {
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let font = Type::Body.font(ui);
    let shown = if design::number_width(ui, name, &font) <= size.x {
        name
    } else {
        let key = (name.to_owned(), size.x.to_bits());
        if !view.fitted.contains_key(&key) {
            // A width that changes with the window makes new keys: keep the map small.
            if view.fitted.len() >= 64 {
                view.fitted.clear();
            }
            let mut words = design::words_that_fit(ui, name, &font, size.x);
            if design::number_width(ui, &words, &font) > size.x {
                words = "…".to_owned();
            }
            #[cfg(test)]
            {
                view.shortened += 1;
            }
            view.fitted.insert(key.clone(), words);
        }
        view.fitted[&key].as_str()
    };
    let mut cell = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    let response = cell.add(egui::Label::new(Type::Body.rich(ui, shown)));
    if shown != name {
        response.on_hover_text(name);
    }
}

/// `add: ask` puts a number field in the action's place. It holds focus until Enter or Escape.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn amount_field(
    ui: &mut egui::Ui,
    view: &mut ViewState,
    label: &str,
    width: f32,
    act: &mut Option<Act>,
) {
    let Some((key, instance, _, typed)) = view.asking.as_mut() else {
        return;
    };
    // One id for the field, from its tracker, line and action, whatever layout holds it: the
    // panel's two layouts nest it in different `Ui`s, and an automatic id would lose focus and
    // the typed amount when the width crosses between them.
    let id = egui::Id::new(("trackers_amount", key.as_str(), instance.as_deref(), label));
    let response = ui.add_sized(
        egui::vec2(width, Density::Read.control()),
        egui::TextEdit::singleline(typed)
            .id(id)
            .min_size(egui::vec2(0.0, Density::Read.control()))
            .hint_text("Amount"),
    );
    design::named(ui, &response, label);
    if std::mem::take(&mut view.focus_asking) {
        response.request_focus();
    }
    if response.lost_focus() {
        let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
        if enter && let Ok(amount) = typed.trim().parse::<i64>() {
            let (key, instance) = (key.clone(), instance.clone());
            // Enter is the press: the action keeps its place as a click's does.
            view.hold(ui, &key, &instance, label, response.rect.top());
            *act = Some(Act::Press {
                key,
                instance,
                label: label.to_owned(),
                asked: Some(amount),
            });
        }
        view.asking = None;
    }
}

/// The view's message footer: why the last act recorded nothing, then the last act of the
/// session being played with `Undo: <label>` and `Add to Timeline`, or, after `Add to
/// Timeline`, `Added to the Timeline.` with `Undo add`. Nothing shows while `Trackers.md`
/// cannot be read. Returns the y where the body ends.
#[allow(
    clippy::disallowed_methods,
    reason = "a refusal carries the ledger’s words"
)]
fn footer(ui: &mut egui::Ui, trackers: &Trackers, act: &mut Option<Act>) -> f32 {
    let shown = trackers.unreadable.is_none();
    let entries = trackers.newest();
    let result = entries.first().zip(trackers.result.as_ref());
    let added = result.and_then(|(first, _)| {
        trackers
            .added
            .as_ref()
            .filter(|added| added.entry == first.id)
    });
    let not_saved = match &trackers.reason {
        Some(Unrecorded::NotSaved(error)) => Some(TrackerFault::NotSaved(error.clone())),
        _ => None,
    };
    // The event waits in the Timeline's draft when its save failed: say so, not success.
    let added_not_saved = added
        .and_then(|added| added.not_saved.as_ref())
        .map(|words| TrackerFault::TimelineNotSaved(words.clone()));
    let (mut reason_pressed, mut result_pressed) = (None, None);
    let mut footer = design::Footer::new("trackers_footer", ui.available_width());
    if shown {
        match (&trackers.reason, &not_saved) {
            (Some(Unrecorded::Refused(why)), _) => {
                footer.outcome(
                    Outcome::Refused(
                        Sentence::outside(why.as_str())
                            .unwrap_or(crate::sentence!("Nothing was recorded.")),
                    ),
                    &mut reason_pressed,
                );
            }
            (_, Some(fault)) => {
                footer.outcome(FaultMessage::new(fault), &mut reason_pressed);
            }
            _ => {}
        }
        let shown = trackers
            .played_owner()
            .and_then(|owner| trackers.results.get(&owner));
        if let (Some(shown), Some((_, (label, words)))) = (shown, result) {
            footer.kept_outcome(
                shown,
                |_| match (added, &added_not_saved) {
                    (Some(_), Some(fault)) => {
                        KeptOutcome::Fault(FaultMessage::new(fault), UndoStep::verb(Verb::Add))
                    }
                    (Some(_), None) => Said::new(crate::sentence!("Added to the Timeline."))
                        .undo(UndoStep::verb(Verb::Add))
                        .into(),
                    (None, _) => Said::new(words.clone())
                        .undo(UndoStep::act(label.as_str()))
                        .choice(MessageChoice::Choice(ADD_TO_TIMELINE))
                        .into(),
                },
                &mut result_pressed,
            );
        }
    }
    let bottom = footer.show_by_hand(ui);
    match (result_pressed, added, result) {
        (Some(Pressed::Undo), Some(added), _) => *act = Some(Act::UndoAdd(added.event.clone())),
        (Some(Pressed::Undo), None, _) => *act = Some(Act::Undo(entries.to_vec())),
        (Some(Pressed::Choice(_)), None, Some((first, _))) => {
            let title = super::timeline_title(&trackers.rules, &trackers.ledger, entries);
            *act = Some(Act::AddToTimeline(first.id.clone(), title));
        }
        _ => {}
    }
    bottom
}

/// The result's choice that makes a Major event of the act.
const ADD_TO_TIMELINE: &str = "Add to Timeline";

/// The words of a failed save: the file, the reason, and that nothing was recorded.
pub(super) fn not_saved(error: &str) -> Sentence {
    TrackerFault::NotSaved(error.to_owned()).cause()
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::Harness;

    /// X16: a name wider than its column ends at a whole word, or is `…` alone when its first
    /// word does not fit; unchanged frames do not shorten it again.
    #[test]
    fn a_name_in_its_column_ends_at_a_whole_word_and_is_shortened_once() {
        const NAMES: [&str; 3] = [
            "Party",
            "Ravenholm Ironside Wardens of the Gate",
            "Supercalifragilisticexpialidocious Smith",
        ];
        let mut h = Harness::builder()
            .with_size(egui::vec2(400.0, 200.0))
            .build_ui_state(
                |ui, view: &mut ViewState| {
                    for name in NAMES {
                        name_cell(ui, view, name, egui::vec2(110.0, 28.0));
                    }
                },
                ViewState::default(),
            );
        crate::design::theme::install(&h.ctx);
        h.run();
        let shown = |view: &ViewState, name: &str| {
            view.fitted
                .iter()
                .find(|((full, _), _)| full == name)
                .map(|(_, shown)| shown.clone())
        };
        let view = h.state();
        assert_eq!(
            shown(view, NAMES[0]),
            None,
            "a name that fits is not shortened"
        );
        let words = shown(view, NAMES[1]).unwrap();
        let kept = words.strip_suffix('…').unwrap();
        assert!(
            !kept.is_empty() && NAMES[1].starts_with(&format!("{kept} ")),
            "{words}"
        );
        assert_eq!(shown(view, NAMES[2]).as_deref(), Some("…"));
        let once = view.shortened;
        assert_eq!(once, 2);
        for _ in 0..3 {
            h.step();
        }
        assert_eq!(
            h.state().shortened,
            once,
            "unchanged frames shorten nothing"
        );
    }
}
