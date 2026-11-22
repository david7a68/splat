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
    NoticeRef, Rules, State, Today, Trackers, Unrecorded, label, ledger, orphaned, place_name,
    result_words, with_dependents,
};
use crate::design::memo::{Measure, Memos, Revision};
use crate::design::outcome::PagePlaceholder;
use crate::design::outcome::{FaultMessage, Messages, Said, Sentence, UndoStep, Verb};
use crate::design::{self, Action, Density, Gap, Type, colors};
use egui;
use ledger::{Entry, Ledger, Op, Placed};
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "test-support")]
thread_local! {
    /// Rows worked out by `rows`: the work that grows with the ledger.
    pub static PREPARED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Open sections drawn, and rows drawn in them.
    pub static DRAWN: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
    /// Rows' words fitted to their room (`history_row`).
    pub static FITTED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// One row of a member's history: the entry as the file held it, its words, and the value it
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
    /// What its Undo names: the clicked entry's act (`Undo remove Take seal: Keyring +1`).
    name: String,
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
        #[cfg(feature = "test-support")]
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
            value: state.value_after(&entry.id),
        });
    }
    rows
}

/// A change's amount as a row says it: `+1`, `-2` or `set to 4`.
fn amount(entry: &Entry) -> String {
    match (entry.set, entry.add) {
        (Some(set), _) => format!("set to {set}"),
        (None, Some(add)) => crate::design::signed(add),
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

/// The entries that `Remove` takes for the entry the DM clicked, as the file holds them now,
/// with each one's place for its Undo, and the result's words.
fn removal(
    rules: &Rules,
    today: &Today,
    ledger: &Ledger,
    drawn: &Entry,
) -> Result<(Placed, String, String), super::Refused> {
    use super::{Part, Refused};
    let found = ledger
        .changes
        .iter()
        .find(|entry| entry.id == drawn.id)
        .ok_or(Refused::Gone(Part::Entry))?;
    if found != drawn {
        return Err(Refused::Changed(Part::Entry));
    }
    let target = with_dependents(ledger, found);
    let next = ledger.apply(&Op::Remove(target.clone()))?;
    // A later change can cross a mark only because of this entry's value.
    if let Some(name) = orphaned(rules, today, ledger, &next) {
        return Err(Refused::DependsOn(name));
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
    // A notice that the removal takes off the Due list says so, as an act's fall does
    // (`result_words`): the DM sees Due go down and reads why.
    for name in ended_notices(rules, today, ledger, &next) {
        words = words.then(crate::sentence!("{} is no longer due.", name));
    }
    Ok((placed, words.to_string(), names.swap_remove(0)))
}

/// The names of the notices due with `ledger` that are not due with `next`, oldest first. A
/// day mark's open days share one row, named by its oldest day: each of its days that is no
/// longer open is named, and a day that opens again ends nothing.
fn ended_notices(rules: &Rules, today: &Today, ledger: &Ledger, next: &Ledger) -> Vec<String> {
    let (before, after) = (
        super::replay(rules, ledger, today),
        super::replay(rules, next, today),
    );
    let mut ended = Vec::new();
    for notice in &before.due {
        match &notice.id {
            NoticeRef::Day { mark, .. } => {
                let open: Vec<u64> = after
                    .due
                    .iter()
                    .filter(
                        |still| matches!(&still.id, NoticeRef::Day { mark: m, .. } if m == mark),
                    )
                    .flat_map(|still| still.days.iter().copied())
                    .collect();
                for day in notice.days.iter().filter(|day| !open.contains(day)) {
                    let one = super::Notice {
                        id: NoticeRef::Day {
                            mark: mark.clone(),
                            day: *day,
                        },
                        ..notice.clone()
                    };
                    ended.push(one.name(rules));
                }
            }
            id => {
                if !after.due.iter().any(|still| still.id == *id) {
                    ended.push(notice.name(rules));
                }
            }
        }
    }
    ended
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
        // What the act was, for its refusal: the entry by its row's label, from `Trackers.md`.
        let failed = crate::design::results::FailedAct::NotRemoved {
            what: entry_thing(&label(
                &self.rules,
                &self.ledger,
                std::slice::from_ref(drawn),
            )),
            from: super::named(ledger::FILE),
        };
        let done = self.commit(failed, |rules, ledger| {
            let (placed, words, name) = removal(rules, &today, ledger, drawn)?;
            let op = Op::Remove(placed.iter().map(|(_, entry)| entry.clone()).collect());
            made = Some((placed, words, name));
            Ok(op)
        });
        match done {
            Ok(()) => {
                if let Some((placed, words, name)) = made {
                    self.removed = Some(Removed {
                        note,
                        placed,
                        words,
                        name,
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
        let failed = crate::design::results::FailedAct::NotPutBack {
            what: entry_thing(&removed.name),
            into: super::named(ledger::FILE),
        };
        let done = self.commit(failed, |rules, ledger| {
            super::restore_op(rules, &today, ledger, removed.placed.clone()).map_err(
                |why| match why {
                    super::RestoreFault::Stale(cause) => {
                        stale = true;
                        cause
                    }
                    super::RestoreFault::Dependent(name) => super::Refused::DependsOn(name),
                },
            )
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

impl Removed {
    /// What its Undo names, and the name a refused Undo gives the entry.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

/// A history entry as a refusal names it: `entry “Take seal”`.
fn entry_thing(label: &str) -> crate::design::results::Thing {
    crate::design::results::Thing::new(crate::design::results::Kind::Entry, super::named(label))
}

/// One footer message of a note's history, as `footer_lines` selects it: its words, and its kind.
pub struct FooterLine {
    pub words: String,
    kind: LineKind,
}

enum LineKind {
    /// Why the last `Remove` or its Undo wrote nothing: the act, its cause and the next step.
    Refused(crate::design::results::Refusal),
    /// The save of the history failed, in `view::not_saved`'s words.
    NotSaved(crate::design::outcome::Sentence),
    /// The last `Remove`, with the order `remove` issued for it and what its Undo names.
    Removed(crate::design::ResultOrder, String),
}

/// The footer messages of the history of `note`, in the footer's order: why the last `Remove`
/// or its Undo wrote nothing, and the last `Remove` with `Undo remove`. A note turned off shows
/// nothing. The one selection: `footer` draws exactly these, and the checks read them.
pub fn footer_lines(trackers: &Trackers, note: Uuid) -> Vec<FooterLine> {
    let mut lines = Vec::new();
    if !in_play(&trackers.rules, note) {
        return lines;
    }
    if let Some((_, reason)) = trackers
        .history_reason
        .as_ref()
        .filter(|(owner, _)| *owner == note)
    {
        lines.push(match reason {
            Unrecorded::Refused(refusal) => FooterLine {
                words: refusal.words().to_string(),
                kind: LineKind::Refused(refusal.clone()),
            },
            Unrecorded::NotSaved(error) => {
                let words = super::view::not_saved(error);
                FooterLine {
                    words: words.to_string(),
                    kind: LineKind::NotSaved(words),
                }
            }
        });
    }
    if let Some(removed) = trackers.removed.as_ref().filter(|r| r.note == note) {
        lines.push(FooterLine {
            words: removed.words.clone(),
            kind: LineKind::Removed(removed.order, removed.name.clone()),
        });
    }
    lines
}

/// The reader's footer messages of the history of `note` (`footer_lines`), typed. (A named lead
/// exception in package D's slice: only this footer path moves to the typed footer. `remove`
/// still issues its own order, so the result goes in by the staged path.)
#[allow(
    clippy::disallowed_methods,
    reason = "staged: the last Remove keeps the order `remove` issued (design::results::STAGED_ALLOWS)"
)]
pub fn footer<A: Clone + 'static>(
    trackers: &Trackers,
    note: Uuid,
    messages: &mut Messages<'_, A>,
    said: fn(&str) -> Sentence,
    undo: A,
) {
    for line in footer_lines(trackers, note) {
        match line.kind {
            LineKind::Refused(refusal) => {
                messages.outcome(FaultMessage::new(&super::TrackerFault::Refused(refusal)));
            }
            LineKind::NotSaved(words) => {
                messages.outcome(FaultMessage::new(&NotSaved(words)));
            }
            LineKind::Removed(order, name) => {
                messages.outcome_at(
                    order,
                    Said::new(said(&line.words))
                        .undo(UndoStep::of(Verb::Remove, name).pressed_as(undo.clone())),
                );
            }
        }
    }
}

/// A failed save of the history, in the words `view::not_saved` gives it.
struct NotSaved(crate::design::outcome::Sentence);

impl crate::design::outcome::Fault for NotSaved {
    fn severity(&self) -> crate::design::outcome::Severity {
        crate::design::outcome::Severity::NotSaved
    }
    fn cause(&self) -> crate::design::outcome::Sentence {
        self.0.clone()
    }
}

/// The problems list and each member's folded history, in the reader of a house rule note in
/// play. A file that cannot be read shows its name and no history or value.
#[allow(
    clippy::disallowed_methods,
    reason = "the history cannot show: a page placeholder; a member’s problem: a row fault"
)]
pub fn show(ui: &mut egui::Ui, trackers: &mut Trackers, note: &splat_content::content::Entry) {
    if !super::is_house_rule_note(note) || !trackers.in_play() {
        return;
    }
    let id = note.id;
    if let Some(error) = &trackers.rules.unreadable {
        design::gap(ui, Gap::L);
        let _ = PagePlaceholder::<crate::design::outcome::NoPress>::new(
            &TrackerFault::NotesUnreadable(error.clone()),
        )
        .show(ui);
        return;
    }
    if !in_play(&trackers.rules, id) {
        return;
    }
    design::gap(ui, Gap::L);
    if let Some(error) = &trackers.unreadable {
        let _ = PagePlaceholder::<crate::design::outcome::NoPress>::new(
            &TrackerFault::LedgerUnreadable(error.clone()),
        )
        .show(ui);
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
    let revision = trackers.revision.revision();
    let fits = &mut trackers.view.history_fits;
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
                rows_in_view(ui, rows, played, (fits, revision), &mut pressed);
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
    (fits, revision): (&mut RowFits, Revision),
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
    #[cfg(feature = "test-support")]
    DRAWN.with(|n| n.set((n.get().0 + 1, n.get().1)));
    if !seen.is_positive() {
        return;
    }
    let first = ((seen.top() - area.top()) / pitch).floor().max(0.0) as usize;
    let last = (((seen.bottom() - area.top()) / pitch).ceil() as usize).min(rows.len());
    for (at, row) in rows.iter().enumerate().take(last).skip(first) {
        #[cfg(feature = "test-support")]
        DRAWN.with(|n| n.set((n.get().0, n.get().1 + 1)));
        let rect = egui::Rect::from_min_size(
            egui::pos2(area.left(), area.top() + at as f32 * pitch),
            egui::vec2(area.width(), height),
        );
        design::register_row(ui, rect);
        // Each row's controls are kept by its entry's ID: a menu open on a row that an edit on
        // disk took away never opens on the row that takes its place.
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(rect)
                .id_salt(&row.entry.id)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                let removable = row.entry.session != played;
                history_row(ui, row, removable, (&mut *fits, revision), pressed);
            },
        );
    }
}

/// Why `Remove` is off on a row of the session being played: its own Undo takes it back.
pub const REMOVE_PLAYED: &str = "Use Undo in Trackers for a change of this session.";

/// A row, laid out from its right end: its menu with `Remove` (off, with its reason painted under
/// it, on a row of the session being played), so every row looks alike and the values stand in one column; the
/// value it left; then its words, cut at a whole word with `…` when they do not fit (the shared
/// `words_that_fit`), whole on hover and to assistive technology. The fit is kept by the row's
/// entry with the trackers' revision and its measure (`ViewState::history_fits`).
fn history_row(
    ui: &mut egui::Ui,
    row: &Row,
    removable: bool,
    (fits, revision): (&mut RowFits, Revision),
    pressed: &mut Option<Entry>,
) {
    let menu = Density::Read.control();
    design::row_menu(ui, &row.words, |ui| {
        if Action::quiet("Remove")
            .density(Density::Read)
            .blocked_if(!removable, REMOVE_PLAYED)
            .show(ui)
            .clicked()
        {
            *pressed = Some(row.entry.clone());
            ui.close();
        }
        // Opened by keyboard, the off `Remove` takes no focus and shows no tooltip: the reason
        // stands under it.
        design::blocked_reason(ui, (!removable).then_some(REMOVE_PLAYED));
    });
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
        let (font, room) = (Type::Body.font(ui), ui.available_width());
        let key = (revision, Measure::of(ui, room, &font));
        let shown = fits.get_or(&row.entry.id, key, || {
            #[cfg(feature = "test-support")]
            FITTED.with(|n| n.set(n.get() + 1));
            let words = design::words_that_fit(ui, &row.words, &font, room);
            // A first word wider than the room is `…` alone, so nothing paints over the value.
            if design::number_width(ui, &words, &font) > room {
                "…".to_owned()
            } else {
                words
            }
        });
        let response = ui.add(egui::Label::new(Type::Body.rich(ui, shown.as_str())));
        if *shown != row.words {
            design::named(ui, &response, &row.words);
            response.on_hover_text(&row.words);
        }
    });
}

/// The history rows' words as fitted, by entry: see [`history_row`].
pub(super) type RowFits = Memos<String, (Revision, Measure), String>;
