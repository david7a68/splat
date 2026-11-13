//! A member's history and the problems list in the house rule note's reader (F4 of the
//! session-cycle design). Each tracker and each day mark of the note has a folded section with
//! its entries in the ledger's order (a tracker's starts with its start value); a row with
//! another session's stamp has `Remove` in its menu. `Remove` takes the entry out of
//! `Trackers.md` through `commit`, with the others of its press and the entries of each notice
//! it raised, as Undo does, and it has one Undo, as the Timeline's removal has. The rows and the
//! problems are worked out when the ledger or the definitions change; a folded section does no
//! work, and an open one draws only the rows in view.
use super::TrackerFault;
use super::{
    NoticeRef, Rules, State, Today, Trackers, Unrecorded, label, ledger, place_name, replay,
    result_words, with_dependents,
};
use crate::design::outcome::PagePlaceholder;
use crate::design::{self, Action, Density, Tone, Type, colors};
use crate::world::{FooterAct, FooterKind, FooterLine, FooterRank};
use eframe::egui;
use ledger::{Entry, Ledger, Op};
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(test)]
thread_local! {
    /// Rows worked out by `rows`: the work that grows with the ledger.
    pub(super) static PREPARED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Open sections drawn, and rows drawn in them.
    pub(super) static DRAWN: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
}

/// One row of a member's history: the entry as the file held it, its words, and the value it
/// left.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub entry: Entry,
    pub words: String,
    pub value: Option<i64>,
}

/// Each entry a `Remove` took, with the entry it followed then, as read.
type Placed = Vec<(Option<Entry>, Entry)>;

/// The last `Remove`: the entries it took, each with the entry it followed, for its one Undo.
#[derive(Clone, Debug, PartialEq)]
pub struct Removed {
    /// The house rule note whose reader the DM acted in.
    pub note: Uuid,
    placed: Placed,
    pub words: String,
    /// When it was made, for the footer's order.
    order: crate::design::ResultOrder,
}

/// The member an entry belongs to: the tracker it changed, the tracker whose change raised its
/// notice, or the day mark of its day notice.
fn member(ledger: &Ledger, entry: &Entry) -> Option<String> {
    match &entry.notice {
        None => entry.tracker.clone(),
        Some(NoticeRef::Value { change, .. }) => ledger
            .changes
            .iter()
            .find(|raised| raised.id == *change)
            .and_then(|raised| raised.tracker.clone()),
        Some(NoticeRef::Day { mark, .. }) => Some(mark.clone()),
    }
}

/// Each member's history, by key, in the ledger's order: a tracker's changes, and the rolls and
/// ends of its notices; a day mark's rolls and ends.
pub fn rows(rules: &Rules, ledger: &Ledger, state: &State) -> HashMap<String, Vec<Row>> {
    let mut rows: HashMap<String, Vec<Row>> = HashMap::new();
    for entry in &ledger.changes {
        let Some(key) = member(ledger, entry) else {
            continue;
        };
        if rules.def(&key).is_none() && rules.day_mark(&key).is_none() {
            continue;
        }
        #[cfg(test)]
        PREPARED.with(|n| n.set(n.get() + 1));
        let one = std::slice::from_ref(entry);
        let words = if entry.notice.is_some() {
            result_words(rules, ledger, state, one).to_string()
        } else {
            let place = entry
                .instance
                .as_ref()
                .map_or(String::new(), |name| format!("{name} "));
            format!("{}: {place}{}", label(rules, ledger, one), amount(entry))
        };
        rows.entry(key).or_default().push(Row {
            entry: entry.clone(),
            words,
            value: state.after.get(&entry.id).copied(),
        });
    }
    rows
}

/// A change's amount as a row says it: `+1`, `-2` or `set to 4`.
fn amount(entry: &Entry) -> String {
    match (entry.set, entry.add) {
        (Some(set), _) => format!("set to {set}"),
        (None, Some(add)) => format!("{add:+}"),
        (None, None) => String::new(),
    }
}

/// What a `Remove` names for one press: its label with each line it changed and by how much
/// (`Take seal: Keyring +1`, the tracker's name for a tracker without lines), so the result says
/// which of two rows with one label went. A notice's press is its label (`Bell done`).
fn removed_words(rules: &Rules, ledger: &Ledger, press: &[Entry]) -> String {
    let name = label(rules, ledger, press);
    let changes: Vec<String> = press
        .iter()
        .filter(|entry| entry.notice.is_none())
        .filter_map(|entry| {
            let line = match &entry.instance {
                Some(line) => line.clone(),
                None => rules
                    .def(entry.tracker.as_deref()?)
                    .map(|def| def.name.clone())?,
            };
            Some(format!("{line} {}", amount(entry)))
        })
        .collect();
    if changes.is_empty() {
        name
    } else {
        format!("{name}: {}", changes.join(", "))
    }
}

/// The problems list, each with the note it belongs to: a note in play that cannot be used, a
/// member that cannot be used, a ledger row that counts for nothing, and a value below its `min`
/// (an edit in Obsidian can force one; Splat changes nothing).
pub fn problems(rules: &Rules, state: &State) -> Vec<(Option<Uuid>, String)> {
    let mut out = rules.problems.clone();
    for tracker in &rules.trackers {
        if let Err(error) = &tracker.def {
            out.push((Some(tracker.note), format!("{}: {error}", tracker.key)));
        }
    }
    for mark in &rules.day_marks {
        if let Err(error) = &mark.def {
            out.push((Some(mark.note), format!("{}: {error}", mark.key)));
        }
    }
    for (key, count) in &state.unknown {
        let (names, are) = if *count == 1 {
            ("names", "It is")
        } else {
            ("name", "They are")
        };
        out.push((
            None,
            crate::sentence!(
                "{} in {} {names} {key}, which no house rule note in play has.",
                design::count(*count, "entry", "entries"),
                ledger::FILE
            )
            .then(crate::sentence!("{} not counted.", are))
            .to_string(),
        ));
    }
    for tracker in &rules.trackers {
        let Ok(def) = &tracker.def else { continue };
        let Some(min) = def.min else { continue };
        for instance in def.instances() {
            let value = state.value(rules, &tracker.key, instance);
            if value < min {
                out.push((
                    Some(tracker.note),
                    crate::sentence!(
                        "{} is {value}, below its min of {min}.",
                        place_name(rules, &tracker.key, instance)
                    )
                    .to_string(),
                ));
            }
        }
    }
    out
}

/// Is this note in play? It defines a member, or it is in play and its file cannot be used: then
/// its reader keeps its problem and the words of a click that wrote nothing. A note turned off
/// is neither.
fn in_play(rules: &Rules, note: Uuid) -> bool {
    rules.trackers.iter().any(|t| t.note == note)
        || rules.day_marks.iter().any(|m| m.note == note)
        || rules.problems.iter().any(|(owner, _)| *owner == Some(note))
}

/// The act whose entries about a notice would be left with nothing to name, when the ledger
/// becomes `next`: a notice that a change raised in `ledger` and raises no more in `next`.
pub(super) fn orphaned(
    rules: &Rules,
    today: &Today,
    ledger: &Ledger,
    next: &Ledger,
) -> Option<String> {
    let (before, after) = (replay(rules, ledger, today), replay(rules, next, today));
    let orphan = next.changes.iter().find(|entry| {
        entry
            .notice
            .as_ref()
            .is_some_and(|notice| before.raised(notice) && !after.raised(notice))
    })?;
    let press: Vec<Entry> = next
        .changes
        .iter()
        .filter(|entry| entry.same_press(orphan))
        .cloned()
        .collect();
    Some(label(rules, next, &press))
}

/// The entries that `Remove` takes for the entry the DM clicked, as the file holds them now,
/// with each one's place for its Undo, and the result's words.
fn removal(
    rules: &Rules,
    today: &Today,
    ledger: &Ledger,
    drawn: &Entry,
) -> Result<(Placed, String), String> {
    let found = ledger
        .changes
        .iter()
        .find(|entry| entry.id == drawn.id)
        .ok_or("That entry is no longer in Trackers.md. Nothing was removed.")?;
    if found != drawn {
        return Err("That entry changed on disk. Nothing was removed.".into());
    }
    let target = with_dependents(ledger, found);
    let next = ledger.apply(&Op::Remove(target.clone()))?;
    // A later change can cross a mark only because of this entry's value.
    if let Some(name) = orphaned(rules, today, ledger, &next) {
        return Err(format!(
            "{name} depends on this entry. Remove that first. Nothing was removed."
        ));
    }
    let placed = target
        .iter()
        .map(|entry| {
            let at = ledger.changes.iter().position(|e| e.id == entry.id);
            let after = at
                .and_then(|at| at.checked_sub(1))
                .map(|before| ledger.changes[before].clone());
            (after, entry.clone())
        })
        .collect();
    // The clicked entry's act, then each act that leaves with it.
    let mut names: Vec<String> = Vec::new();
    for entry in std::iter::once(found).chain(&target) {
        let press: Vec<Entry> = target
            .iter()
            .filter(|other| other.same_press(entry))
            .cloned()
            .collect();
        let name = removed_words(rules, ledger, &press);
        if !names.contains(&name) {
            names.push(name);
        }
    }
    let mut words = crate::sentence!("Removed {}.", names[0]);
    if names.len() > 1 {
        words = words.then(crate::sentence!("With it: {}.", names[1..].join("; ")));
    }
    Ok((placed, words.to_string()))
}

impl Trackers {
    /// `Remove` on a row of a member's history in the reader of `note`: the entry it was drawn
    /// from, exactly as read. It replaces the Undo of the last `Remove`.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    pub fn remove(&mut self, note: Uuid, drawn: &Entry) {
        let today = self.today.clone();
        let mut made = None;
        let done = self.commit(|rules, ledger| {
            let (placed, words) = removal(rules, &today, ledger, drawn)?;
            let op = Op::Remove(placed.iter().map(|(_, entry)| entry.clone()).collect());
            made = Some((placed, words));
            Ok(op)
        });
        match done {
            Ok(()) => {
                if let Some((placed, words)) = made {
                    self.removed = Some(Removed {
                        note,
                        placed,
                        words,
                        order: crate::design::ResultOrder::issue(),
                    });
                }
            }
            Err(Some(reason)) => self.history_reason = Some((note, reason)),
            Err(None) => {}
        }
    }

    /// `Undo remove`: the entries go back where they were. When one of them, or the entry
    /// before it, changed or is gone on disk, it writes nothing, says so, and the Undo ends.
    /// When a notice raised since needs the entries to stay out, it writes nothing and names it.
    pub fn undo_remove(&mut self) {
        let Some(removed) = self.removed.clone() else {
            return;
        };
        let today = self.today.clone();
        let mut stale = false;
        let done = self.commit(|rules, ledger| {
            let op = Op::Restore(removed.placed.clone());
            let next = ledger.apply(&op).inspect_err(|_| stale = true)?;
            if let Some(name) = orphaned(rules, &today, ledger, &next) {
                return Err(format!(
                    "{name} depends on the removal. Remove that first. Nothing was put back."
                ));
            }
            Ok(op)
        });
        match done {
            Ok(()) => self.removed = None,
            Err(Some(reason)) => {
                if stale {
                    self.removed = None;
                }
                self.history_reason = Some((removed.note, reason));
            }
            Err(None) => {}
        }
    }
}

/// The reader's footer lines of the history of `note`: why the last `Remove` or its Undo wrote
/// nothing, and the last `Remove` with `Undo remove`. A note turned off shows nothing.
pub(crate) fn footer_lines(trackers: &Trackers, note: Uuid) -> Vec<FooterLine> {
    let mut lines = Vec::new();
    if !in_play(&trackers.rules, note) {
        return lines;
    }
    if let Some((_, reason)) = trackers
        .history_reason
        .as_ref()
        .filter(|(owner, _)| *owner == note)
    {
        let (tone, words) = match reason {
            Unrecorded::Refused(why) => (Tone::Warning, why.clone()),
            Unrecorded::NotSaved(error) => (Tone::Error, super::view::not_saved(error).to_string()),
        };
        lines.push(FooterLine::new(
            FooterRank::Reason,
            tone,
            &words,
            FooterKind::Plain,
        ));
    }
    if let Some(removed) = trackers.removed.as_ref().filter(|r| r.note == note) {
        lines.push(
            FooterLine::new(
                FooterRank::Undo,
                crate::world::CHANGED,
                &removed.words,
                FooterKind::Undo {
                    label: "Undo remove",
                    act: FooterAct::Trackers,
                },
            )
            .at(removed.order),
        );
    }
    lines
}

/// The problems list and each member's folded history, in the reader of a house rule note in
/// play. A file that cannot be read shows its name and no history or value.
#[allow(
    clippy::disallowed_methods,
    reason = "the history cannot show: a page placeholder; a member’s problem: a row fault"
)]
pub fn show(ui: &mut egui::Ui, trackers: &mut Trackers, note: &splat::content::Entry) {
    if !super::is_house_rule_note(note) || !trackers.in_play() {
        return;
    }
    let id = note.id;
    if let Some(error) = &trackers.rules.unreadable {
        ui.add_space(design::L);
        let _ = PagePlaceholder::new(&TrackerFault::NotesUnreadable(error.clone())).show(ui);
        return;
    }
    if !in_play(&trackers.rules, id) {
        return;
    }
    ui.add_space(design::L);
    if let Some(error) = &trackers.unreadable {
        let _ = PagePlaceholder::new(&TrackerFault::LedgerUnreadable(error.clone())).show(ui);
        return;
    }
    for (_, problem) in trackers
        .problems
        .iter()
        .filter(|(owner, _)| *owner == Some(id))
    {
        design::outcome::row_fault(ui, &TrackerFault::Problem(problem.clone()));
    }
    let mut pressed = None;
    let played = trackers.played;
    let rules = &trackers.rules;
    let sections = rules
        .trackers
        .iter()
        .filter(|t| t.note == id)
        .filter_map(|t| {
            let def = t.def.as_ref().ok()?;
            Some((&t.key, def.name.as_str(), Some(start_words(def))))
        })
        .chain(
            rules
                .day_marks
                .iter()
                .filter(|m| m.note == id)
                .filter_map(|m| Some((&m.key, m.def.as_ref().ok()?.title(), None))),
        );
    for (key, name, start) in sections {
        let rows = trackers.history.get(key).map_or(&[][..], Vec::as_slice);
        design::section_with_id(
            ui,
            ("tracker_history", id, key),
            design::group_heading_text(ui, &format!("{name} history"), rows.len()),
            |ui| {
                if let Some(start) = start {
                    ui.label(Type::Small.rich(ui, start).color(colors().muted));
                }
                rows_in_view(ui, rows, played, &mut pressed);
            },
        );
    }
    if let Some(entry) = pressed {
        trackers.remove(id, &entry);
    }
}

/// `Start 0`, or each name's start value.
fn start_words(def: &super::Definition) -> String {
    match &def.start {
        super::Start::Value(value) => format!("Start {value}"),
        super::Start::Per(_) => {
            let each: Vec<String> = def
                .instances()
                .into_iter()
                .map(|name| format!("{} {}", name.unwrap_or_default(), def.start(name)))
                .collect();
            format!("Start: {}", each.join(", "))
        }
    }
}

/// The rows, each one line of one control's height: the whole list takes its height, and only
/// the rows inside the reader's clip are laid out and drawn, so a long history costs a frame no
/// more than a short one.
fn rows_in_view(
    ui: &mut egui::Ui,
    rows: &[Row],
    played: Option<Uuid>,
    pressed: &mut Option<Entry>,
) {
    let height = Density::Read.control();
    let gap = ui.spacing().item_spacing.y;
    let pitch = height + gap;
    let (area, _) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            (rows.len() as f32 * pitch - gap).max(0.0),
        ),
        egui::Sense::hover(),
    );
    let seen = area.intersect(ui.clip_rect());
    #[cfg(test)]
    DRAWN.with(|n| n.set((n.get().0 + 1, n.get().1)));
    if !seen.is_positive() {
        return;
    }
    let first = ((seen.top() - area.top()) / pitch).floor().max(0.0) as usize;
    let last = (((seen.bottom() - area.top()) / pitch).ceil() as usize).min(rows.len());
    for (at, row) in rows.iter().enumerate().take(last).skip(first) {
        #[cfg(test)]
        DRAWN.with(|n| n.set((n.get().0, n.get().1 + 1)));
        let rect = egui::Rect::from_min_size(
            egui::pos2(area.left(), area.top() + at as f32 * pitch),
            egui::vec2(area.width(), height),
        );
        // Each row's controls are kept by its entry's ID: a menu open on a row that an edit on
        // disk took away never opens on the row that takes its place.
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(rect)
                .id_salt(&row.entry.id)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| history_row(ui, row, row.entry.session != played, pressed),
        );
    }
}

/// A row, laid out from its right end: for another session's entry its menu with `Remove`, else
/// the menu's place, so the values stand in one column; the value it left; then its words, cut
/// with `…` when they do not fit, whole on hover.
fn history_row(ui: &mut egui::Ui, row: &Row, removable: bool, pressed: &mut Option<Entry>) {
    let menu = Density::Read.control();
    if removable {
        design::row_menu(ui, &row.words, |ui| {
            if Action::quiet("Remove")
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                *pressed = Some(row.entry.clone());
                ui.close();
            }
        });
    } else {
        ui.allocate_exact_size(egui::vec2(menu, menu), egui::Sense::hover());
    }
    if let Some(value) = row.value {
        let value = value.to_string();
        let font = Type::Numeral.font(ui);
        let width = design::number_width(ui, &value, &font);
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, menu), egui::Sense::hover());
        design::number(
            ui.painter(),
            rect.right(),
            rect.center().y,
            &value,
            &font,
            colors().text,
        );
    }
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.add(egui::Label::new(Type::Body.rich(ui, &row.words)).truncate());
    });
}
