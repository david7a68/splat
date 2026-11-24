//! A member's history and the problems list in the house rule note's reader (F4 of the
//! session-cycle design). Each tracker and each day mark of the note has a folded section with
//! its entries in the ledger's order (a tracker's starts with its start value); a row with
//! another session's stamp has `Remove` in its menu. `Remove` takes the entry out of
//! `Trackers.md` through `commit`, with the others of its press and the entries of each notice
//! it raised, as Undo does, and it has one Undo, as the Timeline's removal has. The rows and the
//! problems are worked out when the ledger or the definitions change, never per frame. The GPUI
//! reader draws them (`gui::pages::trackers::history`).
use super::{
    NoticeRef, Rules, State, Today, Trackers, Unrecorded, label, ledger, orphaned, place_name,
    result_words, with_dependents,
};
use crate::design::outcome::{FaultMessage, Messages, Said, Sentence, UndoStep, Verb};
use crate::design::{self};
use ledger::{Entry, Ledger, Op, Placed};
use std::collections::HashMap;
use uuid::Uuid;

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
    /// The campaign it was made in: its owner in the store.
    owner: HistoryOwner,
}

/// The owner of the last Remove of a tracker history: the campaign. One Undo of a Remove stands
/// at a time, in whichever note's reader it was made; a campaign switch hides it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryOwner(std::path::PathBuf);

impl crate::design::results::Owner for HistoryOwner {
    type Scope = std::path::Path;
    fn scope(&self) -> &std::path::Path {
        &self.0
    }
}

impl crate::design::results::Kept for Removed {
    type Owner = HistoryOwner;
    const ENDS: crate::design::results::Ends = crate::design::results::Ends {
        // An Undo that found its entries changed or gone on disk ends it.
        owner_changed: crate::design::results::OwnerChanged::Ends,
        // A read of the ledger keeps it: its Undo checks the entries when pressed.
        disk_changed: crate::design::results::DiskChanged::Stays,
        // Its Undo puts the entries back; it leaves only when that works.
        press: crate::design::results::Press::Undo,
    };
    fn owner(&self) -> HistoryOwner {
        self.owner.clone()
    }
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
pub fn in_play(rules: &Rules, note: Uuid) -> bool {
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
/// notice at a value stays due when the same tracker and instance are still at or past its
/// mark, whichever change raises it now (removing the first of two seals leaves the first seal
/// due, raised by the second). A day mark's open days share one row, named by its oldest day:
/// each of its days that is no longer open is named, and a day that opens again ends nothing.
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
            NoticeRef::Value { at, .. } => {
                let stays = after.due.iter().any(|still| {
                    matches!(&still.id, NoticeRef::Value { at: mark, .. } if mark == at)
                        && still.key == notice.key
                        && still.instance == notice.instance
                });
                if !stays {
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
                if let (Some((placed, words, name)), Some(owner)) = (made, self.history_owner()) {
                    // A new Remove replaces the last one and its Undo; the store issues its order.
                    self.removed.record(Removed {
                        note,
                        placed,
                        words,
                        name,
                        owner,
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
    /// `ticket` is the press's: a Remove replaced since its line was drawn does nothing.
    pub fn undo_remove(&mut self, ticket: Option<crate::design::results::Ticket>) {
        let Some(ticket) = ticket else {
            return;
        };
        let Some(removed) = self
            .removed
            .get_by(&ticket)
            .map(|shown| shown.value().clone())
        else {
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
            Ok(()) => {
                let _ = self.removed.undo_by(ticket, |_| Ok::<(), ()>(()));
            }
            Err(Some(reason)) => {
                if stale {
                    self.removed.owner_changed(&removed.owner);
                }
                self.history_reason = Some((removed.note, reason));
            }
            Err(None) => {}
        }
    }

    /// The campaign: the owner of the last Remove.
    fn history_owner(&self) -> Option<HistoryOwner> {
        self.campaign
            .as_ref()
            .map(|campaign| HistoryOwner(campaign.path.clone()))
    }

    /// The last Remove, when it was made in the reader of `note` in this campaign and that
    /// note is in play: the one selection the footer draws its kept line from, so a note out
    /// of play shows no Undo remove (PR 676 review, P1).
    pub(crate) fn removed_in(
        &self,
        note: Uuid,
    ) -> Option<crate::design::results::Shown<'_, Removed>> {
        if !in_play(&self.rules, note) {
            return None;
        }
        self.removed
            .get(&self.history_owner()?)
            .filter(|shown| shown.value().note == note)
    }
}

impl Trackers {
    /// A member's history, by its key: its rows in the ledger's order (none while it has none).
    pub fn history_rows(&self, key: &str) -> &[Row] {
        self.history.get(key).map_or(&[][..], Vec::as_slice)
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
    /// The last `Remove`: a kept line of its store (`footer`).
    Removed,
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
    if let Some(removed) = trackers.removed_in(note) {
        lines.push(FooterLine {
            words: removed.value().words.clone(),
            kind: LineKind::Removed,
        });
    }
    lines
}

/// The reader's footer messages of the history of `note` (`footer_lines`), typed. The last
/// Remove is a kept line of its store (M0567, M0569): its Undo's press carries the store's
/// ticket, so it reaches only the Remove its line showed.
pub fn footer<'f, A: Clone + 'static>(
    trackers: &Trackers,
    note: Uuid,
    frame: &'f crate::design::outcome::PaneFrame,
    messages: &mut Messages<'f, A>,
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
            // Drawn from its store below, with its ticket.
            LineKind::Removed => {}
        }
    }
    if let Some(shown) = trackers.removed_in(note) {
        let line = shown.outcome(frame, |removed| {
            Said::new(said(&removed.words))
                .undo(UndoStep::of(Verb::Remove, removed.name.clone()).pressed_as(undo.clone()))
                .into()
        });
        messages.kept(line);
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

/// `Start 0`, or each name's start value.
pub fn start_words(def: &super::Definition) -> String {
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

/// Why `Remove` is off on a row of the session being played: its own Undo takes it back.
pub const REMOVE_PLAYED: &str = "Use Undo in Trackers for a change of this session.";
