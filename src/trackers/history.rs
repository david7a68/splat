//! A tracker's history and the problems list in the house rule note's reader (F4 of the
//! session-cycle design). Each tracker of the note has a folded section with its start value and
//! its entries in the ledger's order; a row with another session's stamp has `Remove` in its
//! menu. `Remove` takes the entry out of `Trackers.md` through `commit`, with the others of its
//! press and the entries of each notice it raised, as Undo does, and it has one Undo, as the
//! Timeline's removal has. The rows and the problems are worked out when the ledger or the
//! definitions change; a folded section does no work.
use super::{
    Rules, State, Trackers, Unrecorded, label, ledger, notice_place, place_name, replay,
    result_words, with_dependents,
};
use crate::design::{self, Action, Density, Tone, Type, colors};
use crate::world::{FooterAct, FooterKind, FooterLine, FooterRank};
use eframe::egui;
use ledger::{Entry, Ledger, Op};
use std::collections::HashMap;
use uuid::Uuid;

/// One row of a tracker's history: the entry as the file held it, its words, and the value it
/// left.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub entry: Entry,
    pub words: String,
    pub value: Option<i64>,
}

/// The last `Remove`: the entries it took, each with the entry it followed, for its one Undo.
#[derive(Clone, Debug, PartialEq)]
pub struct Removed {
    /// The house rule note whose reader the DM acted in.
    pub note: Uuid,
    placed: Placed,
    pub words: String,
    /// When it was made (`world::next_order`), for the footer's order.
    order: u64,
}

/// Each tracker's history, by key, in the ledger's order: its changes, and the rolls and ends
/// of its notices.
pub fn rows(rules: &Rules, ledger: &Ledger, state: &State) -> HashMap<String, Vec<Row>> {
    let mut rows: HashMap<String, Vec<Row>> = HashMap::new();
    for entry in &ledger.changes {
        let one = std::slice::from_ref(entry);
        let (key, words) = match (&entry.notice, &entry.tracker) {
            (Some(notice), _) => (
                notice_place(ledger, notice).0,
                result_words(rules, ledger, state, one),
            ),
            (None, Some(key)) => {
                let amount = match (entry.set, entry.add) {
                    (Some(set), _) => format!("set to {set}"),
                    (None, Some(add)) => format!("{add:+}"),
                    (None, None) => String::new(),
                };
                let place = entry
                    .instance
                    .as_ref()
                    .map_or(String::new(), |name| format!("{name} "));
                (
                    key.clone(),
                    format!("{}: {place}{amount}", label(rules, ledger, one)),
                )
            }
            (None, None) => continue,
        };
        if rules.def(&key).is_none() {
            continue;
        }
        rows.entry(key).or_default().push(Row {
            entry: entry.clone(),
            words,
            value: state.after.get(&entry.id).copied(),
        });
    }
    rows
}

/// The problems list, each with the note it belongs to: a note in play that cannot be used, a
/// member that cannot be used, a ledger row that counts for nothing, and a value below its `min`
/// (an edit in Obsidian can force one; Splat changes nothing).
pub fn problems(rules: &Rules, state: &State) -> Vec<(Option<Uuid>, String)> {
    let mut out: Vec<(Option<Uuid>, String)> = rules
        .problems
        .iter()
        .map(|problem| (None, problem.clone()))
        .collect();
    for tracker in &rules.trackers {
        if let Err(error) = &tracker.def {
            out.push((Some(tracker.note), format!("{}: {error}", tracker.key)));
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
            format!(
                "{} in {} {names} {key}, which no house rule note in play has. {are} not counted.",
                design::count(*count, "entry", "entries"),
                ledger::FILE
            ),
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
                    format!(
                        "{} is {value}, below its min of {min}.",
                        place_name(rules, &tracker.key, instance)
                    ),
                ));
            }
        }
    }
    out
}

/// Each entry a `Remove` took, with the ID of the entry it followed then.
type Placed = Vec<(Option<String>, Entry)>;

/// The entries that `Remove` takes for the entry the DM clicked, as the file holds them now,
/// with each one's place for its Undo, and the result's words.
fn removal(rules: &Rules, ledger: &Ledger, drawn: &Entry) -> Result<(Placed, String), String> {
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
    // A later change can cross a mark only because of this entry's value. Its notice would go,
    // and the notice's own entries would be left with nothing to name.
    let (before, after) = (replay(rules, ledger), replay(rules, &next));
    if let Some(orphan) = next.changes.iter().find(|entry| {
        entry
            .notice
            .as_ref()
            .is_some_and(|notice| before.raised(notice) && !after.raised(notice))
    }) {
        let press: Vec<Entry> = next
            .changes
            .iter()
            .filter(|entry| entry.same_press(orphan))
            .cloned()
            .collect();
        return Err(format!(
            "{} depends on this entry. Remove that first. Nothing was removed.",
            label(rules, ledger, &press)
        ));
    }
    let placed = target
        .iter()
        .map(|entry| {
            let at = ledger.changes.iter().position(|e| e.id == entry.id);
            let after = at
                .and_then(|at| at.checked_sub(1))
                .map(|before| ledger.changes[before].id.clone());
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
        let name = label(rules, ledger, &press);
        if !names.contains(&name) {
            names.push(name);
        }
    }
    let mut words = format!("Removed: {}.", names[0]);
    if names.len() > 1 {
        words.push_str(&format!(" With it: {}.", names[1..].join("; ")));
    }
    Ok((placed, words))
}

impl Trackers {
    /// `Remove` on a row of a tracker's history in the reader of `note`: the entry it was drawn
    /// from, exactly as read. It replaces the Undo of the last `Remove`.
    pub fn remove(&mut self, note: Uuid, drawn: &Entry) {
        let mut made = None;
        let done = self.commit(|rules, ledger| {
            let (placed, words) = removal(rules, ledger, drawn)?;
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
                        order: crate::world::next_order(),
                    });
                }
            }
            Err(Some(reason)) => self.history_reason = Some((note, reason)),
            Err(None) => {}
        }
    }

    /// `Undo remove`: the entries go back where they were. When one of them, or the entry
    /// before it, changed on disk, it writes nothing, says so, and the Undo ends.
    pub fn undo_remove(&mut self) {
        let Some(removed) = self.removed.clone() else {
            return;
        };
        let mut stale = false;
        let done = self.commit(|_, ledger| {
            let op = Op::Restore(removed.placed.clone());
            ledger.apply(&op).inspect_err(|_| stale = true)?;
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

/// The reader's footer lines of the tracker history of `note`: why the last `Remove` or its
/// Undo wrote nothing, and the last `Remove` with `Undo remove`.
pub(crate) fn footer_lines(trackers: &Trackers, note: Uuid) -> Vec<FooterLine> {
    let mut lines = Vec::new();
    if let Some((_, reason)) = trackers
        .history_reason
        .as_ref()
        .filter(|(owner, _)| *owner == note)
    {
        let (tone, words) = match reason {
            Unrecorded::Refused(why) => (Tone::Warning, why.clone()),
            Unrecorded::NotSaved(error) => (Tone::Error, super::view::not_saved(error)),
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

/// The problems list and each tracker's folded history, in the reader of a house rule note in
/// play. A file that cannot be read shows its name and no history or value.
pub fn show(ui: &mut egui::Ui, trackers: &mut Trackers, note: &splat::content::Entry) {
    if !super::is_house_rule_note(note) || !trackers.in_play() {
        return;
    }
    let id = note.id;
    if let Some(error) = &trackers.rules.unreadable {
        ui.add_space(design::L);
        design::message(
            ui,
            Tone::Error,
            &format!("{error} Splat shows no tracker history until it can be read."),
        );
        return;
    }
    let owned = |t: &&super::Tracker| t.note == id;
    let has_problem = trackers
        .problems
        .iter()
        .any(|(owner, _)| *owner == Some(id));
    if !trackers.rules.trackers.iter().any(|t| owned(&t)) && !has_problem {
        return;
    }
    ui.add_space(design::L);
    if let Some(error) = &trackers.unreadable {
        design::message(
            ui,
            Tone::Error,
            &format!(
                "{} cannot be read: {error} The history in the file is safe.",
                ledger::FILE
            ),
        );
        return;
    }
    for (_, problem) in trackers
        .problems
        .iter()
        .filter(|(owner, _)| *owner == Some(id))
    {
        design::small_message(ui, Tone::Error, problem);
    }
    let mut pressed = None;
    let played = trackers.played;
    for tracker in trackers.rules.trackers.iter().filter(owned) {
        let Ok(def) = &tracker.def else { continue };
        let rows = trackers.history.get(&tracker.key);
        let count = rows.map_or(0, Vec::len);
        design::section_with_id(
            ui,
            ("tracker_history", id, &tracker.key),
            design::group_heading_text(ui, &format!("{} history", def.name), count),
            |ui| {
                ui.label(Type::Small.rich(ui, start_words(def)).color(colors().muted));
                for row in rows.into_iter().flatten() {
                    history_row(ui, row, row.entry.session != played, &mut pressed);
                }
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

/// A row: its words, then the value it left and, for another session's entry, its menu with
/// `Remove`, at the end of the words' last line. The menu's place is kept on every row, so the
/// values stand in one column.
fn history_row(ui: &mut egui::Ui, row: &Row, removable: bool, pressed: &mut Option<Entry>) {
    let value = row.value.map(|value| value.to_string());
    let gap = ui.spacing().item_spacing.x;
    let font = Type::Numeral.font(ui);
    let value_width = value.as_ref().map_or(0.0, |value| {
        ui.painter()
            .layout_no_wrap(value.clone(), font.clone(), colors().text)
            .size()
            .x
    });
    let menu = Density::Read.control();
    super::view::words_ending_with(ui, &row.words, value_width + gap + menu, |ui| {
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
        if let Some(value) = &value {
            let (rect, _) =
                ui.allocate_exact_size(egui::vec2(value_width, menu), egui::Sense::hover());
            design::number(
                ui.painter(),
                rect.right(),
                rect.center().y,
                value,
                &font,
                colors().text,
            );
        }
    });
}
