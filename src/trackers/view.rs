//! The Trackers view of the Lookup panel: the Due rows, then the pinned trackers in the note's
//! order, then `More trackers`, in one scroll area; the result line keeps its place at the
//! panel's bottom edge. The view draws from the ledger as read, and every press is one act.
use super::{
    Place, Rules, State, Trackers, label, ledger::Entry, ledger::NoticeRef, mark_of, result_words,
    short_name,
};
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
}

/// One act the DM asked for in this frame.
enum Act {
    Press {
        key: String,
        instance: Option<String>,
        label: String,
        asked: Option<i64>,
    },
    Roll(NoticeRef, String),
    Done(NoticeRef, Vec<bool>),
    Dismiss(NoticeRef),
    Undo(Vec<Entry>),
    AddToTimeline(String, String),
    UndoAdd(String),
    Retry,
    ReadAgain,
}

impl Trackers {
    /// Does a typed amount hold Escape? Then Escape ends the amount, not Lookup.
    pub fn takes_escape(&self) -> bool {
        self.view.asking.is_some()
    }
}

/// The empty state, the file that cannot be read, or the trackers with the result line.
pub fn show(ui: &mut egui::Ui, trackers: &mut Trackers) {
    if !trackers.in_play() {
        ui.add_space(design::S);
        for words in [
            "No house rules are in play.",
            "Turn one on in Campaign → Rules and runs.",
        ] {
            ui.label(Type::Body.rich(ui, words).color(colors().muted));
        }
        return;
    }
    let mut act = None;
    let has_result = trackers.pending.is_some()
        || matches!(trackers.reason, Some((Place::Result, _)))
        || !trackers.newest().is_empty();
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
    egui::ScrollArea::vertical()
        .id_salt("trackers_view")
        .auto_shrink([false, false])
        .show(ui, |ui| body(ui, trackers, &mut act));
    if let Some(act) = act {
        apply(trackers, act);
    }
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
                trackers.view.unticked.retain(|(id, _)| *id != notice);
            }
        }
        Act::Dismiss(notice) => trackers.dismiss(&notice),
        Act::Undo(entries) => trackers.undo(&entries),
        Act::AddToTimeline(entry, label) => trackers.wants_timeline = Some((entry, label)),
        Act::UndoAdd(event) => {
            trackers.added = None;
            trackers.wants_undo_add = Some(event);
        }
        Act::Retry => trackers.retry(),
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
        ledger,
        view,
        reason,
        pending,
        ..
    } = trackers;
    let blocked = pending
        .as_ref()
        .map(|_| "Trackers not saved. Try again first.");
    if !state.due.is_empty() {
        design::group_heading(ui, "Due", state.due.len());
        for notice in &state.due {
            notice_row(ui, rules, state, ledger, view, reason, blocked, notice, act);
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
            tracker_lines(ui, rules, state, view, reason, blocked, key, def, act);
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
                    tracker_lines(ui, rules, state, view, reason, blocked, key, def, act);
                }
            },
        );
    }
    // A member that cannot be used is reported, and the others still work.
    for problem in rules.problems.iter().cloned().chain(
        rules
            .trackers
            .iter()
            .filter_map(|t| t.def.as_ref().err().map(|e| format!("{}: {e}", t.key))),
    ) {
        design::small_message(ui, Tone::Error, &problem);
    }
}

/// A Due row: its words, its rolls with their results, its proposed changes, and `Done` with
/// the row's menu at the end.
#[allow(clippy::too_many_arguments)]
fn notice_row(
    ui: &mut egui::Ui,
    rules: &Rules,
    _state: &State,
    ledger: &super::ledger::Ledger,
    view: &mut ViewState,
    reason: &Option<(Place, String)>,
    blocked: Option<&str>,
    notice: &super::Notice,
    act: &mut Option<Act>,
) {
    let Some(mark) = mark_of(rules, notice) else {
        return;
    };
    let words = match &notice.instance {
        Some(name) => format!("{name}: {}", mark.say),
        None => mark.say.clone(),
    };
    ui.add(egui::Label::new(Type::Body.rich(ui, &words)).wrap());
    if !mark.tables.is_empty() {
        ui.horizontal_wrapped(|ui| {
            for table in &mark.tables {
                let key = table.key();
                let rolled = notice.rolls.iter().filter(|r| r.table == key).count();
                let each_rolled = !table.names().is_empty() && rolled >= table.names().len();
                let label = format!("Roll {}", rules.table_name(key));
                if Action::secondary(&label)
                    .icon(icons::DICE)
                    .density(Density::Read)
                    .blocked(blocked.or(each_rolled.then_some("Each name has its roll.")))
                    .show(ui)
                    .clicked()
                {
                    *act = Some(Act::Roll(notice.id.clone(), key.to_owned()));
                }
            }
        });
        for roll in &notice.rolls {
            let name = roll
                .name
                .as_deref()
                .unwrap_or_else(|| rules.table_name(&roll.table));
            ui.horizontal(|ui| {
                ui.add_space(design::L);
                ui.add(
                    egui::Label::new(
                        Type::Body.rich(ui, format!("{name}: {} · {}", roll.value, roll.row)),
                    )
                    .wrap(),
                );
            });
        }
    }
    for (at, change) in mark.changes.iter().enumerate() {
        let key = change.tracker.as_deref().unwrap_or(&notice.tracker);
        let name = rules.def(key).map_or(key, |def| def.name.as_str());
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
    let short = short_name(
        rules,
        &notice.tracker,
        notice.instance.as_deref(),
        notice.id.at,
    );
    let _ = ledger;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        design::row_menu(ui, &short, |ui| {
            if Action::quiet("Dismiss")
                .density(Density::Read)
                .blocked(blocked)
                .show(ui)
                .clicked()
            {
                *act = Some(Act::Dismiss(notice.id.clone()));
                ui.close();
            }
        });
        if Action::quiet("Done")
            .density(Density::Read)
            .name(&format!("Done: {short}"))
            .blocked(blocked)
            .show(ui)
            .clicked()
        {
            let ticked = (0..mark.changes.len())
                .map(|at| !view.unticked.contains(&(notice.id.clone(), at)))
                .collect();
            *act = Some(Act::Done(notice.id.clone(), ticked));
        }
    });
    if let Some((Place::Notice(id), why)) = reason
        && *id == notice.id
    {
        design::small_message(ui, Tone::Warning, why);
    }
    ui.add_space(design::XS);
}

/// A tracker's lines: one line, or a heading and one line for each instance.
#[allow(clippy::too_many_arguments)]
fn tracker_lines(
    ui: &mut egui::Ui,
    rules: &Rules,
    state: &State,
    view: &mut ViewState,
    reason: &Option<(Place, String)>,
    blocked: Option<&str>,
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
                let asks = matches!(
                    quick.add.as_ref().map(super::Amount::read),
                    Some(Ok(super::Add::Ask { .. }))
                );
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
                    .blocked(blocked)
                    .show(ui);
                if response.clicked() {
                    if asks {
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
        if let Some((Place::Tracker(k, i), why)) = reason
            && (k.as_str(), i.as_deref()) == (key, instance)
        {
            design::small_message(ui, Tone::Warning, why);
        }
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

/// The last act of the session being played, with `Undo: <label>` and `Add to Timeline`; or the
/// save that waits; or why the last act wrote nothing.
fn result_line(ui: &mut egui::Ui, trackers: &Trackers, act: &mut Option<Act>) {
    if let Some((_, error)) = &trackers.pending {
        design::message_with_actions(
            ui,
            Tone::Error,
            &format!("Trackers not saved. {error} The act waits."),
            |ui| {
                if Action::secondary("Try again")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    *act = Some(Act::Retry);
                }
            },
        );
        return;
    }
    if let Some((Place::Result, why)) = &trackers.reason {
        design::message(ui, Tone::Warning, why);
    }
    let entries = trackers.newest().to_vec();
    let Some(first) = entries.first() else {
        return;
    };
    let label = label(&trackers.rules, &trackers.ledger, &entries);
    if let Some(added) = trackers
        .added
        .as_ref()
        .filter(|added| added.entry == first.id)
    {
        if design::undo_message(ui, Tone::Success, "Added to the Timeline.", "Undo add") {
            *act = Some(Act::UndoAdd(added.event.clone()));
        }
        return;
    }
    let words = result_words(&trackers.rules, &trackers.ledger, &trackers.state, &entries);
    design::message_with_actions(ui, Tone::Success, &words, |ui| {
        if Action::quiet(&format!("Undo: {label}"))
            .icon(icons::UNDO)
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            *act = Some(Act::Undo(entries.clone()));
        }
        if Action::quiet("Add to Timeline")
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            *act = Some(Act::AddToTimeline(first.id.clone(), label.clone()));
        }
    });
}
