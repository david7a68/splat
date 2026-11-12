//! The Trackers view of the Lookup panel: the Due rows, then the pinned trackers in the note's
//! order, then `More trackers`, in one scroll area; the result line keeps its place at the
//! panel's bottom edge. The view draws from the ledger as read, and every press is one act.
use super::{Rules, Says, State, Trackers, Unrecorded, ledger::Entry, ledger::NoticeRef, mark_of};
use crate::design::{self, Action, Density, Tone, Type, colors, icons};
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

    /// The words in the amount field, while it is open.
    #[cfg(test)]
    pub(super) fn typed(&self) -> Option<&str> {
        self.asking.as_ref().map(|(_, _, _, typed)| typed.as_str())
    }
}

/// The Due list's words after the Timeline's own, while the day count is not known.
pub(crate) const DAY_MARKS_WAIT: &str = "No day mark shows until the Timeline is read.";

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

    /// `Done` or `Next` pressed on this row, as a click in the view leaves it for the shell.
    #[cfg(test)]
    pub(crate) fn press_done_in_view(&mut self, notice: &super::Notice, ticked: Vec<bool>) {
        self.view.pressed = Some(Act::Done(notice.clone(), ticked));
    }
}

/// The result line, then the empty state, a file that cannot be read, or the trackers.
pub fn show(ui: &mut egui::Ui, trackers: &mut Trackers, from_top: bool) {
    let mut act = None;
    // The result line comes first: why an act recorded nothing stays in view, also when the
    // row it was pressed on went away.
    let has_result = trackers.reason.is_some() || trackers.result.is_some();
    if has_result && trackers.unreadable.is_none() {
        egui::TopBottomPanel::bottom("trackers_result")
            .frame(egui::Frame::NONE)
            .show_separator_line(false)
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.add_space(design::XS);
                design::rule(ui);
                ui.add_space(design::XS);
                result_line(ui, trackers, &mut act);
            });
    }
    if !trackers.in_play() {
        ui.add_space(design::S);
        for words in [
            "No house rules are in play.",
            "Turn one on in Campaign → Rules and runs.",
        ] {
            ui.label(Type::Body.rich(ui, words).color(colors().muted));
        }
        trackers.view.pressed = act;
        return;
    }
    // What is due is in view when the view opens.
    let mut area = egui::ScrollArea::vertical()
        .id_salt("trackers_view")
        .auto_shrink([false, false]);
    if from_top {
        area = area.vertical_scroll_offset(0.0);
    }
    area.show(ui, |ui| body(ui, trackers, &mut act));
    trackers.view.pressed = act;
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

fn body(ui: &mut egui::Ui, trackers: &mut Trackers, act: &mut Option<Act>) {
    if let Some(error) = &trackers.unreadable {
        let path = trackers.path();
        design::message_with_actions(
            ui,
            Tone::Error,
            &format!(
                "{} cannot be read: {error} The history in the file is safe. Splat writes nothing to it until it can be read.",
                super::ledger::FILE
            ),
            |ui| {
                if Action::secondary("Try again")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    *act = Some(Act::ReadAgain);
                }
                if let Some(path) = path
                    && design::link(ui, "Show file")
                        .on_hover_text(path.display().to_string())
                        .clicked()
                    && let Ok(url) = reqwest::Url::from_file_path(&path)
                {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(url.to_string()));
                }
            },
        );
        return;
    }
    let Trackers {
        rules,
        state,
        view,
        problems,
        ..
    } = trackers;
    // A definition that cannot be read shows no value, never an old one.
    if let Some(error) = &rules.unreadable {
        design::message(
            ui,
            Tone::Error,
            &format!(
                "{error} Splat shows no tracker and records nothing until it can be read. It reads the notes again when its window gets focus."
            ),
        );
        return;
    }
    // A day count that is not known shows no day notice, never an old one, and says why.
    if let Some(why) = &state.clock_not_read {
        design::small_message(ui, Tone::Error, &format!("{why} {DAY_MARKS_WAIT}"));
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
    if usable(true).next().is_some() {
        design::group_heading(ui, "Pinned", 0);
        for (key, def) in usable(true) {
            tracker_lines(ui, rules, state, view, key, def, act);
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
                    tracker_lines(ui, rules, state, view, key, def, act);
                }
            },
        );
    }
    // A member that cannot be used is reported, and the others still work. So are a row of the
    // ledger that counts for nothing and a value below its `min`.
    for (_, problem) in problems.iter() {
        design::small_message(ui, Tone::Error, problem);
    }
}

/// A Due row: its words, its rolls with their results, its proposed changes, and `Done` with
/// the row's menu at the end.
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
    if !head.trim().is_empty() {
        ui.add(egui::Label::new(Type::Body.rich(ui, head.trim_end())).wrap());
    }
    ui.horizontal(|ui| {
        ui.label(Type::Body.rich(ui, last.trim()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), add);
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

/// The row's last action: `Next` on a day mark's row of several days, else `Done`. Either
/// applies the ticked changes and ends one notice: the oldest day of the row.
fn finish_label(notice: &super::Notice) -> &'static str {
    if notice.days.len() > 1 {
        "Next"
    } else {
        "Done"
    }
}

/// `Done` or `Next` and the row's menu, with the gap between them.
fn finish_width(ui: &egui::Ui, notice: &super::Notice) -> f32 {
    Action::quiet(finish_label(notice))
        .density(Density::Read)
        .natural_width(ui)
        + ui.spacing().item_spacing.x
        + Density::Read.control()
}

/// The row's menu with `Dismiss` (and `Dismiss all` on a row of several days), then `Done` or
/// `Next`, laid out from the right.
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
    let name = format!("{finish}: {short}");
    if Action::quiet(finish)
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

/// A tracker's lines: one line, or a heading and one line for each instance.
fn tracker_lines(
    ui: &mut egui::Ui,
    rules: &Rules,
    state: &State,
    view: &mut ViewState,
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
    let font = Type::Body.font(ui);
    let measure = |ui: &egui::Ui, text: &str, font: egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font, colors().text)
            .size()
            .x
    };
    let instances = def.instances();
    let indent = if def.per.is_empty() {
        0.0
    } else {
        ui.label(Type::Body.rich(ui, &def.name));
        design::L
    };
    // Names and values stand in columns, so the values of one tracker read down a line.
    let name_width = instances
        .iter()
        .map(|i| measure(ui, i.unwrap_or(&def.name), font.clone()))
        .fold(0.0, f32::max);
    let value_width = instances
        .iter()
        .map(|i| measure(ui, &value_text(*i), Type::Numeral.font(ui)))
        .fold(0.0, f32::max);
    for instance in instances {
        let place = (key.to_owned(), instance.map(str::to_owned));
        ui.horizontal_wrapped(|ui| {
            ui.add_space(indent);
            let height = Density::Read.control();
            let name = instance.unwrap_or(&def.name);
            ui.allocate_ui_with_layout(
                egui::vec2(name_width, height),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| ui.label(Type::Body.rich(ui, name)),
            );
            ui.add_space(design::S);
            let (rect, _) =
                ui.allocate_exact_size(egui::vec2(value_width, height), egui::Sense::hover());
            design::number(
                ui.painter(),
                rect.right(),
                rect.center().y,
                &value_text(instance),
                &Type::Numeral.font(ui),
                colors().text,
            );
            ui.add_space(design::S);
            for quick in &def.quick {
                let here = view.asking.as_ref().is_some_and(|(k, i, l, _)| {
                    *k == place.0 && *i == place.1 && *l == quick.label
                });
                if here {
                    amount_field(ui, view, &quick.label, act);
                    continue;
                }
                let response = Action::secondary(&quick.label)
                    .density(Density::Read)
                    .name(&match instance {
                        Some(name) => format!("{}: {name}", quick.label),
                        None => quick.label.clone(),
                    })
                    .show(ui);
                if response.clicked() {
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
        });
    }
    ui.add_space(design::XS);
}

/// `add: ask` puts a number field in the action's place. It holds focus until Enter or Escape.
fn amount_field(ui: &mut egui::Ui, view: &mut ViewState, label: &str, act: &mut Option<Act>) {
    let Some((key, instance, _, typed)) = view.asking.as_mut() else {
        return;
    };
    let response = ui.add(
        egui::TextEdit::singleline(typed)
            .desired_width(design::XL * 3.0)
            .hint_text("Amount"),
    );
    design::named(ui, &response, label);
    if std::mem::take(&mut view.focus_asking) {
        response.request_focus();
    }
    if response.lost_focus() {
        let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
        if enter && let Ok(amount) = typed.trim().parse::<i64>() {
            *act = Some(Act::Press {
                key: key.clone(),
                instance: instance.clone(),
                label: label.to_owned(),
                asked: Some(amount),
            });
        }
        view.asking = None;
    }
}

/// Why the last act recorded nothing, then the last act of the session being played, with
/// `Undo: <label>` and `Add to Timeline`.
fn result_line(ui: &mut egui::Ui, trackers: &Trackers, act: &mut Option<Act>) {
    match &trackers.reason {
        Some(Unrecorded::Refused(why)) => {
            design::message(ui, Tone::Warning, why);
        }
        Some(Unrecorded::NotSaved(error)) => {
            design::message(ui, Tone::Error, &not_saved(error));
        }
        None => {}
    }
    let entries = trackers.newest();
    let (Some(first), Some((label, words))) = (entries.first(), &trackers.result) else {
        return;
    };
    if let Some(added) = trackers
        .added
        .as_ref()
        .filter(|added| added.entry == first.id)
    {
        // The event waits in the Timeline's draft when its save failed: say so, not success.
        let (tone, words) = match &added.not_saved {
            Some(words) => (Tone::Error, words.as_str()),
            None => (Tone::Success, "Added to the Timeline."),
        };
        if design::undo_message(ui, tone, words, "Undo add") {
            *act = Some(Act::UndoAdd(added.event.clone()));
        }
        return;
    }
    let message = design::message(ui, Tone::Success, words);
    let inset = message.rect.left() - ui.cursor().left();
    let undo_label = format!("Undo: {label}");
    let undo = Action::quiet(&undo_label)
        .icon(icons::UNDO)
        .density(Density::Read);
    let add = Action::quiet("Add to Timeline").density(Density::Read);
    let gap = ui.spacing().item_spacing.x;
    // `Undo` starts at the words; `Add to Timeline` ends the line, or has a line of its own.
    let one_line =
        inset + undo.natural_width(ui) + design::L + add.natural_width(ui) <= ui.available_width();
    let mut undo_clicked = false;
    let mut add_clicked = false;
    ui.horizontal(|ui| {
        ui.add_space((inset - gap).max(0.0));
        undo_clicked = undo.show(ui).clicked();
        if one_line {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                add_clicked = add.show(ui).clicked();
            });
        }
    });
    if !one_line {
        ui.horizontal(|ui| {
            ui.add_space((inset - gap).max(0.0));
            add_clicked = Action::quiet("Add to Timeline")
                .density(Density::Read)
                .show(ui)
                .clicked();
        });
    }
    if undo_clicked {
        *act = Some(Act::Undo(entries.to_vec()));
    }
    if add_clicked {
        *act = Some(Act::AddToTimeline(first.id.clone(), label.clone()));
    }
}

/// The words of a failed save: the file, the reason, and that nothing was recorded.
pub(super) fn not_saved(error: &str) -> String {
    format!(
        "{} was not saved: {}. Nothing was recorded.",
        super::ledger::FILE,
        error.trim_end_matches('.')
    )
}
