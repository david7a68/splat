//! Trackers (F4 of the session-cycle design), the slice between fights: house rule notes in
//! play, their trackers and marks, the ledger's replay into values and the Due list, and the
//! acts that write the ledger. Splat counts and shows; a mark raises a notice and never acts.
//!
//! One owner for each fact: a definition is in its house rule note, a value and a notice are
//! entries of `Trackers.md`, and nothing else remembers an act.
pub mod history;
pub mod ledger;
pub mod view;

use crate::{
    content_workspace::CampaignDocument, lookup::ContentIndex, loot_editor::Sources,
    persistence::Campaign,
};
use ledger::{Entry, Fault, Ledger, NoticeRef, Op, Roll, TableRoll};
use serde::Deserialize;
use serde_yaml_ng::Value;
use splat::execution::dice::{Dice, Random};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use uuid::Uuid;

/// A `rule` entry with this implementation is a house rule note.
pub const IMPLEMENTATION: &str = "splat:house_rules";
/// The ledger is read again this often, so an edit in Obsidian shows.
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);
/// Today's `Undo add` of an event keeps its 30 seconds.
pub const UNDO_ADD_FOR: Duration = Duration::from_secs(30);

// ---------------------------------------------------------------------------------------------
// Definitions: the members of a house rule note.

/// A tracker: a named number. Its history is in the ledger.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    pub name: String,
    #[serde(default)]
    pub start: Start,
    #[serde(default)]
    pub min: Option<i64>,
    #[serde(default)]
    pub goal: Option<i64>,
    #[serde(default)]
    pub per: Vec<String>,
    #[serde(default)]
    pub pin: bool,
    #[serde(default)]
    pub quick: Vec<Quick>,
    #[serde(default)]
    pub marks: Vec<Mark>,
}

/// The first value: one number, or one for each `per` word.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Start {
    Value(i64),
    Per(HashMap<String, i64>),
}

impl Default for Start {
    fn default() -> Self {
        Self::Value(0)
    }
}

/// A named click that makes one or more linked changes.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Quick {
    pub label: String,
    #[serde(default)]
    pub add: Option<Amount>,
    #[serde(default)]
    pub set: Option<i64>,
    #[serde(default)]
    pub changes: Vec<Change>,
    #[serde(default)]
    pub table: Option<String>,
}

/// One change of a quick action or a mark: this tracker, or another one.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Change {
    #[serde(default)]
    pub tracker: Option<String>,
    #[serde(default)]
    pub add: Option<Amount>,
    #[serde(default)]
    pub set: Option<i64>,
}

impl Quick {
    /// Does a press ask for a number first? `ask` or `-ask` on the action, or on one of its
    /// changes: the one rule for the amount field.
    pub fn asks(&self) -> bool {
        self.add
            .iter()
            .chain(self.changes.iter().filter_map(|change| change.add.as_ref()))
            .any(|amount| matches!(amount.read(), Ok(Add::Ask { .. })))
    }
}

/// A number, a dice expression, `ask` or `-ask`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Amount {
    Number(i64),
    Words(String),
}

/// What an amount asks for at the click.
#[derive(Clone, Debug, PartialEq)]
pub enum Add {
    Number(i64),
    Dice(String),
    Ask { minus: bool },
}

impl Amount {
    fn read(&self) -> Result<Add, String> {
        match self {
            Self::Number(n) => Ok(Add::Number(*n)),
            Self::Words(words) => match words.trim() {
                "ask" => Ok(Add::Ask { minus: false }),
                "-ask" => Ok(Add::Ask { minus: true }),
                dice => Dice::parse(dice)
                    .map(|_| Add::Dice(dice.to_owned()))
                    .map_err(|_| format!("“{dice}” is not a number, dice, ask or -ask.")),
            },
        }
    }
}

/// A value that raises a notice when a value rises past it.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Mark {
    pub at: i64,
    pub say: String,
    #[serde(default)]
    pub tables: Vec<MarkTable>,
    #[serde(default)]
    pub changes: Vec<Change>,
}

/// A table that a notice rolls: by its key, or with the names it rolls for (`for:`).
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MarkTable {
    Key(String),
    Named {
        table: String,
        #[serde(rename = "for")]
        names: Vec<String>,
    },
}

impl MarkTable {
    pub fn key(&self) -> &str {
        match self {
            Self::Key(key) | Self::Named { table: key, .. } => key,
        }
    }

    pub fn names(&self) -> &[String] {
        match self {
            Self::Key(_) => &[],
            Self::Named { names, .. } => names,
        }
    }
}

impl Definition {
    /// The value before any entry, for this instance.
    pub fn start(&self, instance: Option<&str>) -> i64 {
        match &self.start {
            Start::Value(value) => *value,
            Start::Per(values) => instance.and_then(|i| values.get(i)).copied().unwrap_or(0),
        }
    }

    /// The instances: one line for each `per` word, or one line with no name.
    pub fn instances(&self) -> Vec<Option<&str>> {
        if self.per.is_empty() {
            vec![None]
        } else {
            self.per.iter().map(|word| Some(word.as_str())).collect()
        }
    }

    /// Does this tracker have a line for this instance? With `per`, a name of it; else none.
    fn has_line(&self, instance: Option<&str>) -> bool {
        match instance {
            None => self.per.is_empty(),
            Some(name) => self.per.iter().any(|word| word == name),
        }
    }
}

/// A tracker of a note in play, or why its definition cannot be used.
#[derive(Clone, Debug, PartialEq)]
pub struct Tracker {
    pub key: String,
    pub def: Result<Definition, String>,
    /// The house rule note that defines it.
    pub note: Uuid,
}

/// The house rule notes in play and what they define. Read again when the content changes.
#[derive(Clone, Default)]
pub struct Rules {
    /// The notes in play that were found, by name.
    pub notes: Vec<String>,
    /// Every tracker of those notes, in the notes' order.
    pub trackers: Vec<Tracker>,
    /// A note in play that cannot be used, with the reason.
    pub problems: Vec<String>,
    /// `Campaign.md` or a note in play that cannot be read, with the reason. It is no empty
    /// list: no value shows and no act is made until the file can be read.
    pub unreadable: Option<String>,
    /// The campaign's marked tables.
    pub tables: Option<Arc<Sources>>,
}

impl Rules {
    pub fn in_play(&self) -> bool {
        !self.notes.is_empty() || !self.problems.is_empty() || self.unreadable.is_some()
    }

    /// The same notes, trackers and problems: what the view draws and an act reads.
    fn same_definitions(&self, other: &Rules) -> bool {
        (
            &self.notes,
            &self.trackers,
            &self.problems,
            &self.unreadable,
        ) == (
            &other.notes,
            &other.trackers,
            &other.problems,
            &other.unreadable,
        )
    }

    pub fn def(&self, key: &str) -> Option<&Definition> {
        self.trackers
            .iter()
            .find(|tracker| tracker.key == key)
            .and_then(|tracker| tracker.def.as_ref().ok())
    }

    /// A table's name for reading: its heading, never its key.
    pub fn table_name<'a>(&'a self, key: &'a str) -> &'a str {
        self.tables
            .as_ref()
            .and_then(|tables| tables.table(key))
            .map_or(key, |table| table.name())
    }
}

/// `splat_in_play: [{id: …}]`.
pub fn read_in_play(value: Option<&Value>) -> Result<Vec<Uuid>, String> {
    #[derive(Deserialize)]
    struct Reference {
        id: Uuid,
    }
    match value {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(value) => serde_yaml_ng::from_value::<Vec<Reference>>(value.clone())
            .map(|list| list.into_iter().map(|r| r.id).collect())
            .map_err(|error| format!("splat_in_play: {error}")),
    }
}

/// Is this entry a house rule note?
pub fn is_house_rule_note(entry: &splat::content::Entry) -> bool {
    entry.kind == "rule"
        && entry
            .fields
            .get("splat_rule")
            .and_then(|rule| rule.get("implementation"))
            .and_then(Value::as_str)
            == Some(IMPLEMENTATION)
}

/// The house rule notes in play as the files hold them now: the list in the campaign's manifest
/// (`Campaign.md`, at `manifest` in its folder), and each note of the campaign read from its
/// file, where the content found it. A package's note is the content's. This is the one read of
/// the definitions: when the content changes, and at each act, so that an act never uses a
/// definition that changed on disk. It reads only those files, never the whole folder.
pub fn rules_on_disk(campaign: &Campaign, manifest: &str, index: &ContentIndex) -> Rules {
    let read = || -> Result<Vec<Uuid>, String> {
        let text = std::fs::read_to_string(campaign.path.join(manifest))
            .map_err(|error| format!("{error}."))?;
        let entry = splat::content::Entry::parse(manifest, &text)?;
        if entry.kind != "campaign" || entry.id != campaign.id {
            return Err("it is not this campaign’s manifest.".into());
        }
        read_in_play(entry.fields.get("splat_in_play"))
    };
    let ids = match read() {
        Ok(ids) => ids,
        Err(error) if error.starts_with("splat_in_play") => {
            return Rules {
                problems: vec![error],
                ..Rules::default()
            };
        }
        Err(error) => return Rules::cannot_read(format!("{manifest} cannot be read: {error}")),
    };
    if ids.is_empty() {
        return Rules::default();
    }
    let mut notes = Vec::new();
    for id in ids {
        let Some(item) = index.items.iter().find(|item| item.entry.id == id) else {
            notes.push(Err(format!(
                "A house rule note in play is not found: {id}. Switch it off in Campaign → Rules and runs."
            )));
            continue;
        };
        if !item.is_campaign_note() {
            notes.push(Ok(item.entry.clone()));
            continue;
        }
        let path = &item.entry.path;
        let moved = format!(
            "{} is not where it was. Splat reads the notes again when its window gets focus.",
            item.entry.name
        );
        match std::fs::read_to_string(campaign.path.join(path)) {
            Ok(text) => match splat::content::Entry::parse(path, &text) {
                Ok(entry) if entry.id == id => notes.push(Ok(entry)),
                Ok(_) => notes.push(Err(moved)),
                Err(error) => return Rules::cannot_read(format!("{path} cannot be read: {error}")),
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => notes.push(Err(moved)),
            Err(error) => return Rules::cannot_read(format!("{path} cannot be read: {error}.")),
        }
    }
    read_rules(index.loot_sources(), notes)
}

impl Rules {
    fn cannot_read(error: String) -> Self {
        Self {
            unreadable: Some(error),
            ..Self::default()
        }
    }
}

/// Read the notes in play, or why each cannot be used. Each member is checked alone, so a bad
/// one is reported and the others still work.
fn read_rules(tables: Arc<Sources>, notes: Vec<Result<splat::content::Entry, String>>) -> Rules {
    let mut rules = Rules {
        tables: Some(tables),
        ..Rules::default()
    };
    let mut owners: HashMap<String, Vec<String>> = HashMap::new();
    for note in notes {
        let entry = match note {
            Ok(entry) => entry,
            Err(problem) => {
                rules.problems.push(problem);
                continue;
            }
        };
        let entry = &entry;
        if !is_house_rule_note(entry) {
            rules.problems.push(format!(
                "{} is in play, but it is not a house rule note.",
                entry.name
            ));
            continue;
        }
        rules.notes.push(entry.name.clone());
        let members = entry
            .fields
            .get("splat_members")
            .and_then(|members| members.get("trackers"))
            .and_then(Value::as_mapping);
        for (key, value) in members.into_iter().flatten() {
            let Some(key) = key.as_str() else { continue };
            owners
                .entry(key.to_owned())
                .or_default()
                .push(entry.name.clone());
            rules.trackers.push(Tracker {
                key: key.to_owned(),
                def: serde_yaml_ng::from_value::<Definition>(value.clone())
                    .map_err(|error| error.to_string()),
                note: entry.id,
            });
        }
    }
    let checked: Vec<_> = rules
        .trackers
        .iter()
        .map(|tracker| match &tracker.def {
            Err(error) => Err(error.clone()),
            Ok(def) => match owners.get(&tracker.key).filter(|o| o.len() > 1) {
                Some(notes) => Err(format!(
                    "The key {} is in {}. A key names one tracker.",
                    tracker.key,
                    notes.join(" and ")
                )),
                None => check(def, &rules).map(|()| def.clone()),
            },
        })
        .collect();
    for (tracker, def) in rules.trackers.iter_mut().zip(checked) {
        tracker.def = def;
    }
    rules
}

/// Every word of a definition resolves: trackers, tables, instances and amounts.
fn check(def: &Definition, rules: &Rules) -> Result<(), String> {
    if let Start::Per(values) = &def.start
        && let Some(word) = values.keys().find(|word| !def.per.contains(word))
    {
        return Err(format!("start names “{word}”, which is not in per."));
    }
    let known = |key: &str| rules.trackers.iter().find(|t| t.key == key);
    let table = |key: &str| match &rules.tables {
        Some(tables) => tables.problem(key).map_or(Ok(()), Err),
        None => Err(format!("No roll table has the key “{key}”.")),
    };
    let change = |change: &Change, alone: bool| -> Result<(), String> {
        if let Some(key) = &change.tracker {
            let Some(target) = known(key) else {
                return Err(format!("No tracker in play has the key “{key}”."));
            };
            if let Ok(target) = &target.def
                && !target.per.is_empty()
            {
                if def.per.is_empty() {
                    return Err(format!("{key} has per, so a change to it needs a name."));
                }
                if let Some(word) = def.per.iter().find(|word| !target.per.contains(word)) {
                    return Err(format!("{key} has no line for {word}."));
                }
            }
        }
        match (&change.add, change.set) {
            (Some(add), None) => match add.read()? {
                Add::Ask { .. } if !alone => {
                    Err("ask works only in an action of one change.".into())
                }
                _ => Ok(()),
            },
            (None, Some(_)) => Ok(()),
            _ => Err("A change has add or set.".into()),
        }
    };
    for quick in &def.quick {
        let one = Change {
            tracker: None,
            add: quick.add.clone(),
            set: quick.set,
        };
        match (
            quick.changes.is_empty(),
            one.add.is_some() || one.set.is_some(),
        ) {
            (true, true) => change(&one, true),
            (false, false) => quick
                .changes
                .iter()
                .try_for_each(|c| change(c, quick.changes.len() == 1)),
            _ => Err("An action has add, set or changes.".into()),
        }
        .map_err(|error| format!("{}: {error}", quick.label))?;
        if let Some(key) = &quick.table {
            table(key).map_err(|error| format!("{}: {error}", quick.label))?;
        }
    }
    for mark in &def.marks {
        for t in &mark.tables {
            table(t.key())?;
        }
        for c in &mark.changes {
            change(c, false)?;
            if matches!(c.add.as_ref().map(Amount::read), Some(Ok(Add::Ask { .. }))) {
                return Err("A mark’s change cannot ask.".into());
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// The replay: values and notices are worked out from the entries, never stored.

/// An open notice, with its rolls so far.
#[derive(Clone, Debug, PartialEq)]
pub struct Notice {
    pub id: NoticeRef,
    pub tracker: String,
    pub instance: Option<String>,
    pub rolls: Vec<TableRoll>,
}

/// What the ledger says now, for the definitions in play.
#[derive(Clone, Debug, Default)]
pub struct State {
    values: HashMap<(String, Option<String>), i64>,
    /// The open notices, oldest first.
    pub due: Vec<Notice>,
    /// The value of each change entry right after it.
    after: HashMap<String, i64>,
    /// The marks each change entry crossed.
    raised: HashMap<String, Vec<i64>>,
    /// How many entries name each tracker that no note in play has, in the file's order. They
    /// count for nothing, and the view names them.
    pub unknown: Vec<(String, usize)>,
}

impl State {
    pub fn value(&self, rules: &Rules, key: &str, instance: Option<&str>) -> i64 {
        self.values
            .get(&(key.to_owned(), instance.map(str::to_owned)))
            .copied()
            .or_else(|| rules.def(key).map(|def| def.start(instance)))
            .unwrap_or(0)
    }

    pub fn notice(&self, id: &NoticeRef) -> Option<&Notice> {
        self.due.iter().find(|notice| notice.id == *id)
    }

    /// Did a change raise this notice, open or ended?
    fn raised(&self, id: &NoticeRef) -> bool {
        self.raised
            .get(&id.change)
            .is_some_and(|marks| marks.contains(&id.at))
    }
}

/// The value is a plain sum: `start`, then every change in ledger order. A row for a tracker
/// that is not in play is kept in the file and counts for nothing here.
pub fn replay(rules: &Rules, ledger: &Ledger) -> State {
    let mut state = State::default();
    for entry in &ledger.changes {
        if let Some(id) = &entry.notice {
            let Some(at) = state.due.iter().position(|notice| notice.id == *id) else {
                continue;
            };
            if entry.done || entry.dismissed {
                state.due.remove(at);
            } else if let Some(Roll::Table(roll)) = &entry.roll {
                state.due[at].rolls.push(roll.clone());
            }
            continue;
        }
        let Some(key) = &entry.tracker else { continue };
        let Some(def) = rules.def(key) else {
            if !rules.trackers.iter().any(|tracker| tracker.key == *key) {
                match state.unknown.iter_mut().find(|(known, _)| known == key) {
                    Some((_, count)) => *count += 1,
                    None => state.unknown.push((key.clone(), 1)),
                }
            }
            continue;
        };
        let instance = entry.instance.as_deref();
        if !def.has_line(instance) {
            continue;
        }
        let place = (key.clone(), entry.instance.clone());
        let before = state
            .values
            .get(&place)
            .copied()
            .unwrap_or_else(|| def.start(instance));
        let after = entry.set.unwrap_or(before + entry.add.unwrap_or(0));
        state.values.insert(place, after);
        state.after.insert(entry.id.clone(), after);
        for mark in &def.marks {
            if before < mark.at && mark.at <= after {
                state.due.push(Notice {
                    id: NoticeRef {
                        change: entry.id.clone(),
                        at: mark.at,
                    },
                    tracker: key.clone(),
                    instance: entry.instance.clone(),
                    rolls: Vec::new(),
                });
                state
                    .raised
                    .entry(entry.id.clone())
                    .or_default()
                    .push(mark.at);
            }
        }
    }
    state
}

/// The notice the DM clicked, as the file on disk holds it now: the same change and mark, for
/// the same tracker and name. An outside edit that moved it writes nothing.
fn open_as_drawn<'a>(state: &'a State, drawn: &Notice) -> Result<&'a Notice, String> {
    let open = state
        .notice(&drawn.id)
        .ok_or("That notice is no longer due. Nothing was recorded.")?;
    if open.tracker != drawn.tracker || open.instance != drawn.instance {
        return Err("That notice changed on disk. Nothing was recorded.".into());
    }
    Ok(open)
}

/// The mark that a notice is for.
pub fn mark_of<'a>(rules: &'a Rules, notice: &Notice) -> Option<&'a Mark> {
    rules
        .def(&notice.tracker)?
        .marks
        .iter()
        .find(|mark| mark.at == notice.id.at)
}

/// A notice's short name: the first sentence of its words, and the name it is for.
pub fn short_name(rules: &Rules, tracker: &str, instance: Option<&str>, at: i64) -> String {
    let say = rules
        .def(tracker)
        .and_then(|def| def.marks.iter().find(|mark| mark.at == at))
        .map_or("Notice", |mark| mark.say.as_str());
    let first = say.split_once('.').map_or(say, |(first, _)| first).trim();
    match instance {
        Some(name) => format!("{first} for {name}"),
        None => first.to_owned(),
    }
}

/// A tracker's name for reading, with the instance it changed.
fn place_name(rules: &Rules, key: &str, instance: Option<&str>) -> String {
    let name = rules.def(key).map_or(key, |def| def.name.as_str());
    match instance {
        Some(instance) => format!("{name} ({instance})"),
        None => name.to_owned(),
    }
}

/// The entries that one Undo removes: the newest entry of this session with the others of its
/// press, and every entry about a notice that those changes raised.
pub fn undo_target(ledger: &Ledger, session: Option<Uuid>) -> Vec<Entry> {
    ledger
        .changes
        .iter()
        .rev()
        .find(|entry| entry.session == session)
        .map_or_else(Vec::new, |newest| with_dependents(ledger, newest))
}

/// The entries that leave with this one, by Undo or by `Remove`: the others of its press, and
/// every entry about a notice that those changes raised, in the ledger's order.
fn with_dependents(ledger: &Ledger, seed: &Entry) -> Vec<Entry> {
    let mut out: Vec<&Entry> = ledger
        .changes
        .iter()
        .filter(|entry| entry.same_press(seed))
        .collect();
    loop {
        let more: Vec<&Entry> = ledger
            .changes
            .iter()
            .filter(|entry| !out.iter().any(|kept| kept.id == entry.id))
            .filter(|entry| {
                entry
                    .notice
                    .as_ref()
                    .is_some_and(|notice| out.iter().any(|kept| kept.id == notice.change))
                    || out.iter().any(|kept| entry.same_press(kept))
            })
            .collect();
        if more.is_empty() {
            break;
        }
        out.extend(more);
    }
    out.sort_by_key(|entry| ledger.changes.iter().position(|other| other.id == entry.id));
    out.into_iter().cloned().collect()
}

/// The tracker and instance of the change that raised a notice.
fn notice_place(ledger: &Ledger, notice: &NoticeRef) -> (String, Option<String>) {
    ledger
        .changes
        .iter()
        .find(|entry| entry.id == notice.change)
        .map(|entry| {
            (
                entry.tracker.clone().unwrap_or_default(),
                entry.instance.clone(),
            )
        })
        .unwrap_or_default()
}

/// The entry that names an act: the end of a notice, else a tracker change, else a roll. An
/// Undo also takes the rolls on a notice that the act raised; they do not name it.
fn head(entries: &[Entry]) -> Option<&Entry> {
    entries
        .iter()
        .find(|e| e.done || e.dismissed)
        .or_else(|| entries.iter().find(|e| e.tracker.is_some()))
        .or(entries.first())
}

/// The act's name: the action's own label, or the notice's.
pub fn label(rules: &Rules, ledger: &Ledger, entries: &[Entry]) -> String {
    let Some(entry) = head(entries) else {
        return String::new();
    };
    if let Some(notice) = &entry.notice {
        let (tracker, instance) = notice_place(ledger, notice);
        let short = short_name(rules, &tracker, instance.as_deref(), notice.at);
        return if entry.done {
            format!("{short} done")
        } else if entry.dismissed {
            format!("{short} dismissed")
        } else if let Some(Roll::Table(roll)) = &entry.roll {
            format!("{} roll", rules.table_name(&roll.table))
        } else {
            short
        };
    }
    entry.why.clone().unwrap_or_else(|| {
        format!(
            "{} change",
            place_name(
                rules,
                entry.tracker.as_deref().unwrap_or_default(),
                entry.instance.as_deref()
            )
        )
    })
}

/// `"1d6: [4]"` reads as `Rolled 4 on 1d6`.
fn rolled(text: &str) -> String {
    match text.split_once(": [") {
        Some((dice, faces)) => {
            let total: i64 = faces
                .trim_end_matches(']')
                .split(',')
                .filter_map(|face| face.trim().parse::<i64>().ok())
                .sum();
            format!("Rolled {total} on {dice}")
        }
        None => format!("Rolled {text}"),
    }
}

/// The result line's words for one act, from the entries alone, so they read the same after a
/// restart.
pub fn result_words(rules: &Rules, ledger: &Ledger, state: &State, entries: &[Entry]) -> String {
    let label = label(rules, ledger, entries);
    if let Some(entry) = head(entries).filter(|e| e.notice.is_some()) {
        if entry.done || entry.dismissed {
            return format!("{label}.");
        }
        if let Some(Roll::Table(roll)) = &entry.roll {
            let name = roll
                .name
                .as_ref()
                .map_or(String::new(), |n| format!("{n} "));
            return format!("{label}: {name}{} · {}", roll.value, roll.row);
        }
    }
    let values: Vec<String> = entries
        .iter()
        .filter(|entry| entry.notice.is_none())
        .filter_map(|entry| {
            let after = state.after.get(&entry.id)?;
            Some(format!(
                "{} {after}",
                place_name(rules, entry.tracker.as_deref()?, entry.instance.as_deref())
            ))
        })
        .collect();
    let mut words = if values.is_empty() {
        format!("{label}.")
    } else {
        format!("{label}: {}.", values.join(", "))
    };
    for entry in entries {
        if let Some(Roll::Dice(text)) = &entry.roll {
            match &entry.row {
                Some(row) => words.push_str(&format!(" {}: {row}", rolled(text))),
                None => words.push_str(&format!(" {}.", rolled(text))),
            }
        }
    }
    let due: Vec<String> = entries
        .iter()
        .flat_map(|entry| {
            state.raised.get(&entry.id).into_iter().flatten().map(|at| {
                short_name(
                    rules,
                    entry.tracker.as_deref().unwrap_or_default(),
                    entry.instance.as_deref(),
                    *at,
                )
            })
        })
        .collect();
    if !due.is_empty() {
        words.push_str(&format!(" Due: {}.", due.join(", ")));
    }
    words
}

// ---------------------------------------------------------------------------------------------
// Rolls

fn roll_dice(dice: &str) -> Result<(i64, Vec<i64>), String> {
    let mut random = Random::seeded(crate::hermetic::new_uuid().as_u128() as u64);
    let roll = Dice::parse(dice)?.roll(&mut random, false)?;
    Ok((roll.total, roll.dice))
}

fn faces_text(dice: &str, faces: &[i64]) -> String {
    let faces: Vec<String> = faces.iter().map(ToString::to_string).collect();
    format!("{dice}: [{}]", faces.join(", "))
}

/// A roll on a marked table: its dice as rolled, the total, and the row's words.
fn roll_table(rules: &Rules, key: &str) -> Result<(String, i64, String), String> {
    let table = rules
        .tables
        .as_ref()
        .and_then(|tables| tables.table(key))
        .ok_or_else(|| format!("No roll table has the key “{key}”."))?;
    if let Some(fault) = table.fault() {
        return Err(fault);
    }
    let (total, faces) = roll_dice(&table.dice)?;
    let row = table
        .rows
        .iter()
        .find(|row| (row.min..=row.max).contains(&total))
        .map(|row| row.text.clone())
        .ok_or_else(|| format!("{} rolled {total}, and no row has it.", table.dice))?;
    Ok((faces_text(&table.dice, &faces), total, row))
}

// ---------------------------------------------------------------------------------------------
// Acts: each builds its entries from the ledger on disk at the click.

/// One change's entry, with the value it leaves. A change that would pass `min` is refused.
/// The line's name goes only to a tracker with `per`.
fn change_entry(
    rules: &Rules,
    state: &State,
    values: &mut HashMap<(String, Option<String>), i64>,
    key: &str,
    instance: Option<&str>,
    change: &Change,
    asked: Option<i64>,
) -> Result<Entry, String> {
    let target = rules
        .def(key)
        .ok_or_else(|| format!("No tracker in play has the key “{key}”."))?;
    let instance = instance.filter(|_| !target.per.is_empty());
    if !target.has_line(instance) {
        return Err(format!(
            "{} has no line for {}. Nothing was recorded.",
            target.name,
            instance.unwrap_or("this action")
        ));
    }
    let place = (key.to_owned(), instance.map(str::to_owned));
    let before = values
        .get(&place)
        .copied()
        .unwrap_or_else(|| state.value(rules, key, instance));
    let mut entry = Entry {
        tracker: Some(key.to_owned()),
        instance: place.1.clone(),
        ..Entry::default()
    };
    let after = match (
        change.set,
        change.add.as_ref().map(Amount::read).transpose()?,
    ) {
        (Some(set), _) => {
            entry.set = Some(set);
            entry.was = Some(before);
            set
        }
        (None, Some(Add::Number(n))) => {
            entry.add = Some(n);
            before + n
        }
        (None, Some(Add::Dice(dice))) => {
            let (total, faces) = roll_dice(&dice)?;
            entry.add = Some(total);
            entry.roll = Some(Roll::Dice(faces_text(&dice, &faces)));
            before + total
        }
        (None, Some(Add::Ask { minus })) => {
            let typed = asked.ok_or("Type a number, then press Enter.")?;
            let n = if minus { -typed } else { typed };
            entry.add = Some(n);
            before + n
        }
        (None, None) => return Err("A change has add or set.".into()),
    };
    if let Some(min) = target.min
        && after < min
        && after < before
    {
        return Err(format!("{} cannot go below {min}.", target.name));
    }
    values.insert(place, after);
    Ok(entry)
}

/// IDs, the session's stamp, and one `use` for a press of several entries.
fn stamp(mut entries: Vec<Entry>, session: Option<Uuid>) -> Op {
    let press = (entries.len() > 1).then(|| ledger::new_id("u"));
    for entry in &mut entries {
        entry.id = ledger::new_id("c");
        entry.press = press.clone();
        entry.session = session;
    }
    Op::Append(entries)
}

/// The entries of a quick action, pressed on one line.
fn press_op(
    rules: &Rules,
    state: &State,
    key: &str,
    instance: Option<&str>,
    label: &str,
    asked: Option<i64>,
    session: Option<Uuid>,
) -> Result<Op, String> {
    let def = rules.def(key).ok_or("That tracker is no longer in play.")?;
    if instance.is_some_and(|name| !def.per.iter().any(|word| word == name)) {
        return Err("That name changed in its note. Nothing was recorded.".into());
    }
    let quick = def
        .quick
        .iter()
        .find(|quick| quick.label == label)
        .ok_or("That action changed in its note. Nothing was recorded.")?;
    let changes = if quick.changes.is_empty() {
        vec![Change {
            tracker: None,
            add: quick.add.clone(),
            set: quick.set,
        }]
    } else {
        quick.changes.clone()
    };
    let mut values = HashMap::new();
    let mut entries = Vec::new();
    for change in &changes {
        let target = change.tracker.as_deref().unwrap_or(key);
        let mut entry = change_entry(rules, state, &mut values, target, instance, change, asked)?;
        entry.why = Some(label.to_owned());
        entries.push(entry);
    }
    if let Some(table) = &quick.table {
        let (dice, _, row) = roll_table(rules, table)?;
        entries[0].roll = Some(Roll::Dice(dice));
        entries[0].row = Some(row);
    }
    Ok(stamp(entries, session))
}

/// Why the last act wrote nothing. It shows on the result line, at the bottom edge of the view,
/// until the next act or until the ledger is read again: never beside a row that can go away.
#[derive(Clone, Debug, PartialEq)]
pub enum Unrecorded {
    /// The act does not fit the files on disk now.
    Refused(String),
    /// The save failed. Nothing waits: the DM presses again.
    NotSaved(String),
}

/// `Add to Timeline` made this event for this entry. Its `Undo add` keeps today's 30 seconds.
#[derive(Clone, Debug)]
pub struct Added {
    pub entry: String,
    pub event: String,
    pub at: Instant,
    /// The Timeline's words while this event is not saved in `Timeline.md`.
    pub not_saved: Option<String>,
}

/// The trackers of the active campaign: the definitions in play, the ledger as read, and the
/// acts' results. The shell owns one, beside Lookup, and the rail reads its count.
#[derive(Default)]
pub struct Trackers {
    campaign: Option<Campaign>,
    /// Where the campaign's manifest is, as the content found it, or why it cannot be read.
    manifest: Option<Result<String, String>>,
    /// The content the definitions were read from.
    index: Option<Arc<ContentIndex>>,
    pub rules: Rules,
    pub ledger: Ledger,
    /// Why `Trackers.md` cannot be read. Then no value and no notice shows.
    pub unreadable: Option<String>,
    /// The bytes read or written last; `None` before the first read.
    baseline: Option<Option<Vec<u8>>>,
    pub state: State,
    /// The entries that `Undo` takes back now: worked out when the ledger or the session changes.
    newest: Vec<Entry>,
    /// The newest act's label and the result line's words, worked out with `newest`.
    pub(crate) result: Option<(String, String)>,
    last_check: Option<Instant>,
    /// Why the last act wrote nothing.
    pub reason: Option<Unrecorded>,
    pub added: Option<Added>,
    /// The session being played, for the stamp and for Undo.
    pub played: Option<Uuid>,
    /// `Add to Timeline` for this entry, with this label: the shell has the Timeline.
    pub wants_timeline: Option<(String, String)>,
    /// `Undo add` of this event.
    pub wants_undo_add: Option<String>,
    pub(crate) view: view::ViewState,
    /// Each tracker's history, by key: worked out when the ledger or the definitions change.
    history: HashMap<String, Vec<history::Row>>,
    /// The problems list, each with the house rule note it belongs to: worked out with the
    /// history.
    problems: Vec<(Option<Uuid>, String)>,
    /// The last `Remove` in a tracker's history, for its one Undo, by its entries' IDs.
    removed: Option<history::Removed>,
    /// Why the last `Remove` or its Undo wrote nothing, for the reader of that note.
    history_reason: Option<(Uuid, Unrecorded)>,
}

impl Trackers {
    /// Follow the campaign, its content and the ledger on disk. Called each frame; it reads the
    /// manifest only when the content changes, and the ledger at most once a second, and only
    /// while a note is in play.
    pub fn poll(
        &mut self,
        index: Option<Arc<ContentIndex>>,
        campaign: Option<&Campaign>,
        played: Option<Uuid>,
    ) {
        if self.played != played {
            self.played = played;
            self.settle_result();
        }
        let key = |campaign: &Campaign| (campaign.id, campaign.path.clone());
        if campaign.map(key) != self.campaign.as_ref().map(key) {
            *self = Self {
                campaign: campaign.cloned(),
                played,
                ..Self::default()
            };
        }
        let Some(campaign) = campaign else { return };
        if let Some(index) = index
            && !self
                .index
                .as_ref()
                .is_some_and(|old| Arc::ptr_eq(old, &index))
        {
            self.manifest = Some(CampaignDocument::load(campaign).map(|document| document.path));
            self.index = Some(index);
            self.rules = self.rules_now();
            self.settle();
            self.last_check = None;
        }
        if self.rules.in_play()
            && self
                .last_check
                .is_none_or(|at| at.elapsed() >= CHECK_DISK_EVERY)
        {
            self.last_check = Some(Instant::now());
            self.read_disk();
        }
        if self
            .added
            .as_ref()
            .is_some_and(|added| added.at.elapsed() >= UNDO_ADD_FOR)
        {
            self.added = None;
        }
    }

    fn path(&self) -> Option<PathBuf> {
        self.campaign
            .as_ref()
            .map(|campaign| ledger::path(&campaign.path))
    }

    /// Take the file's text when it changed. A file that cannot be read shows nothing old.
    pub fn read_disk(&mut self) {
        let Some(path) = self.path() else { return };
        let bytes = match ledger::read_bytes(&path) {
            Ok(bytes) => bytes,
            Err(error) => {
                self.unreadable = Some(error);
                self.baseline = None;
                return;
            }
        };
        if self.baseline.as_ref() == Some(&bytes) {
            return;
        }
        // The file changed: the words of the last act that wrote nothing are old.
        self.reason = None;
        self.history_reason = None;
        match ledger::parse_bytes(bytes.as_deref()) {
            Ok(ledger) => {
                self.ledger = ledger;
                self.unreadable = None;
            }
            Err(error) => {
                self.ledger = Ledger::default();
                self.unreadable = Some(error);
            }
        }
        self.baseline = Some(bytes);
        self.settle();
    }

    /// Is a house rule note in play? With none, nothing about trackers shows anywhere.
    pub fn in_play(&self) -> bool {
        self.rules.in_play()
    }

    /// The open notices, or none while the ledger or the definitions cannot be read.
    pub fn due(&self) -> usize {
        if self.in_play() && self.unreadable.is_none() && self.rules.unreadable.is_none() {
            self.state.due.len()
        } else {
            0
        }
    }

    /// The newest act of the session being played, with the entries one Undo takes back.
    pub fn newest(&self) -> &[Entry] {
        &self.newest
    }

    /// Work the values, the notices, the histories and the newest act out of the ledger again.
    fn settle(&mut self) {
        self.state = replay(&self.rules, &self.ledger);
        self.history = history::rows(&self.rules, &self.ledger, &self.state);
        self.problems = history::problems(&self.rules, &self.state);
        self.settle_result();
    }

    /// The newest act of the session being played, and the result line's words for it.
    fn settle_result(&mut self) {
        self.newest = undo_target(&self.ledger, self.played);
        self.result = (!self.newest.is_empty()).then(|| {
            (
                label(&self.rules, &self.ledger, &self.newest),
                result_words(&self.rules, &self.ledger, &self.state, &self.newest),
            )
        });
    }

    /// An act built from the values and notices of the file on disk. Why it wrote nothing
    /// shows on the view's result line.
    fn act(&mut self, build: impl FnOnce(&Rules, &State) -> Result<Op, String>) {
        if let Err(Some(reason)) = self.commit(|rules, ledger| build(rules, &replay(rules, ledger)))
        {
            self.reason = Some(reason);
        }
    }

    /// The definitions in play as the files hold them now.
    fn rules_now(&self) -> Rules {
        match (&self.campaign, &self.manifest, &self.index) {
            (Some(campaign), Some(Ok(manifest)), Some(index)) => {
                rules_on_disk(campaign, manifest, index)
            }
            (_, Some(Err(error)), _) => {
                Rules::cannot_read(format!("Campaign.md cannot be read: {error}"))
            }
            _ => Rules::default(),
        }
    }

    /// Every act passes here. It reads the definitions in play and the ledger as the files hold
    /// them now. When a definition changed since the view drew it, or cannot be read, the act
    /// writes nothing and says why, and the view draws what the files hold. `Err(None)`: the
    /// ledger cannot be read, which the view and the reader say in place of the history.
    fn commit(
        &mut self,
        build: impl FnOnce(&Rules, &Ledger) -> Result<Op, String>,
    ) -> Result<(), Option<Unrecorded>> {
        let Some(campaign) = self.campaign.clone() else {
            return Err(None);
        };
        let rules = self.rules_now();
        let changed = !rules.same_definitions(&self.rules);
        let result = ledger::commit(&ledger::path(&campaign.path), |ledger| {
            if let Some(error) = &rules.unreadable {
                return Err(format!("{error} Nothing was recorded."));
            }
            if changed {
                return Err("A house rule note changed on disk. Nothing was recorded.".into());
            }
            build(&rules, ledger)
        });
        let done = self.finish(result);
        if changed {
            self.rules = rules;
            self.settle();
        }
        done
    }

    fn finish(
        &mut self,
        result: Result<(Vec<u8>, Ledger, Op), Fault>,
    ) -> Result<(), Option<Unrecorded>> {
        match result {
            Ok((bytes, ledger, _)) => {
                self.baseline = Some(Some(bytes));
                self.ledger = ledger;
                self.unreadable = None;
                self.settle();
                // The newest act's words replace why an older one wrote nothing.
                self.reason = None;
                self.history_reason = None;
                Ok(())
            }
            Err(Fault::NotRead(error)) => {
                self.unreadable = Some(error);
                self.baseline = None;
                Err(None)
            }
            Err(Fault::Refused(reason)) => {
                // The view draws again from the file, and says why nothing was recorded.
                self.read_disk();
                Err(Some(Unrecorded::Refused(reason)))
            }
            // Nothing waits: the value, the Due rows and the result line show what the file
            // holds now, also an edit made in Obsidian a moment before.
            Err(Fault::NotSaved(error)) => {
                self.read_disk();
                Err(Some(Unrecorded::NotSaved(error)))
            }
        }
    }

    /// A quick action, on the line of `instance`, named by the label it was drawn with.
    pub fn press(&mut self, key: &str, instance: Option<&str>, label: &str, asked: Option<i64>) {
        let session = self.played;
        self.act(|rules, state| press_op(rules, state, key, instance, label, asked, session));
        // A typed number of an act that recorded nothing stays in its field.
        if let Some(amount) = asked
            && self.reason.is_some()
        {
            self.view.ask_again(key, instance, label, amount);
        }
    }

    /// A notice's `Roll <table>`: for the next name when the table names its rolls.
    pub fn roll_notice(&mut self, drawn: &Notice, table: &str) {
        let notice = &drawn.id;
        let session = self.played;
        self.act(|rules, state| {
            let open = open_as_drawn(state, drawn)?;
            let mark = mark_of(rules, open).ok_or("That mark changed in its note.")?;
            let names = mark
                .tables
                .iter()
                .find(|t| t.key() == table)
                .ok_or("That table changed in its note. Nothing was rolled.")?
                .names();
            let rolled = open.rolls.iter().filter(|roll| roll.table == table).count();
            let name = if names.is_empty() {
                None
            } else {
                Some(names.get(rolled).ok_or("Each name has its roll.")?.clone())
            };
            let (_, value, row) = roll_table(rules, table)?;
            Ok(stamp(
                vec![Entry {
                    notice: Some(notice.clone()),
                    roll: Some(Roll::Table(TableRoll {
                        table: table.to_owned(),
                        name,
                        value,
                        row,
                    })),
                    ..Entry::default()
                }],
                session,
            ))
        });
    }

    /// `Done`: the ticked proposed changes and the notice's end, as one act.
    pub fn done(&mut self, drawn: &Notice, ticked: &[bool]) {
        let notice = &drawn.id;
        let session = self.played;
        self.act(|rules, state| {
            let open = open_as_drawn(state, drawn)?;
            let mark = mark_of(rules, open).ok_or("That mark changed in its note.")?;
            if mark.changes.len() != ticked.len() {
                return Err("That mark changed in its note. Nothing was recorded.".into());
            }
            // The label of the notice's end, as `label` names it.
            let why = format!(
                "{} done",
                short_name(rules, &open.tracker, open.instance.as_deref(), notice.at)
            );
            let mut values = HashMap::new();
            let mut entries = Vec::new();
            for (change, _) in mark.changes.iter().zip(ticked).filter(|(_, on)| **on) {
                let target = change.tracker.as_deref().unwrap_or(&open.tracker);
                let instance = open.instance.as_deref();
                let mut entry =
                    change_entry(rules, state, &mut values, target, instance, change, None)?;
                entry.why = Some(why.clone());
                entries.push(entry);
            }
            entries.push(Entry {
                notice: Some(notice.clone()),
                done: true,
                ..Entry::default()
            });
            Ok(stamp(entries, session))
        });
    }

    /// `Dismiss`: the notice ends, and nothing is applied.
    pub fn dismiss(&mut self, drawn: &Notice) {
        let notice = &drawn.id;
        let session = self.played;
        self.act(|_, state| {
            open_as_drawn(state, drawn)?;
            Ok(stamp(
                vec![Entry {
                    notice: Some(notice.clone()),
                    dismissed: true,
                    ..Entry::default()
                }],
                session,
            ))
        });
    }

    /// `Undo: <label>`: these entries, exactly as the line named them, leave the ledger.
    pub fn undo(&mut self, expected: &[Entry]) {
        let session = self.played;
        if let Err(Some(reason)) = self.commit(|_, ledger| {
            let target = undo_target(ledger, session);
            if target != expected {
                return Err("The ledger changed on disk. Nothing was undone.".into());
            }
            Ok(Op::Remove(target))
        }) {
            self.reason = Some(reason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{content_workspace::CampaignDocument, encounter_settings::Options};
    use std::fs;

    const EXAMPLE: &str = include_str!("../docs/design/world-examples/heralds-tournament.md");

    fn table(heading: &str, key: &str, dice: &str, rows: &[(&str, &str)]) -> String {
        let mut text = format!(
            "## {heading}\n\n<!-- splat:rolls key={key} dice={dice} -->\n\n| {} | Result |\n| --- | --- |\n",
            dice.trim_start_matches('1')
        );
        for (value, row) in rows {
            text.push_str(&format!("| {value} | {row} |\n"));
        }
        text.push('\n');
        text
    }

    /// The vault's Tournament note as the example writes it, and its tables.
    struct Race {
        _dir: tempfile::TempDir,
        campaign: Campaign,
        trackers: Trackers,
        index: Arc<ContentIndex>,
    }

    const TONIGHT: Uuid = Uuid::from_u128(0x0070_eca3_6331_c46c_b5a6_32a6_e352_65da);

    fn race() -> Race {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Heralds".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let start = EXAMPLE.find("````markdown\n").unwrap() + "````markdown\n".len();
        let end = start + EXAMPLE[start..].find("````").unwrap();
        let id = Uuid::new_v4();
        let note = EXAMPLE[start..end].replace("<a new UUID>", &id.to_string());
        fs::write(dir.path().join("Tournament.md"), note).unwrap();
        let wander = [
            ("1-7", "Nothing comes."),
            ("8", "Two Sand Wraiths at the next door."),
        ];
        let tables = [
            table(
                "Rival Track",
                "rival_track",
                "1d6",
                &[
                    ("1", "Loses its way."),
                    ("2–5", "Moves on."),
                    ("6", "Takes a seal."),
                ],
            ),
            table("Wandering, 0:00 to 0:39", "wander_early", "1d8", &wander),
            table("Wandering, 0:40 to 1:19", "wander_middle", "1d8", &wander),
            table("Wandering, 1:20 and later", "wander_late", "1d8", &wander),
            table("Gifts", "gifts", "1d6", &[("1-6", "A healing potion.")]),
        ];
        fs::write(
            dir.path().join("Undercroft Tables.md"),
            format!("# Undercroft Tables\n\n{}", tables.concat()),
        )
        .unwrap();
        // The switch in Campaign → Rules and runs puts the note in play.
        let catalogue = crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
        let mut options = Options::load(&campaign, &catalogue).unwrap();
        assert_eq!(options.house_rules, vec![(id, "Tournament".to_owned())]);
        assert!(options.in_play.is_empty());
        options.in_play.push(id);
        let document = CampaignDocument::load(&campaign).unwrap();
        options.save(&campaign, &document.text).unwrap();
        assert_eq!(
            Options::load(&campaign, &catalogue).unwrap().in_play,
            vec![id]
        );
        let index = Arc::new(ContentIndex::load(&[], Some(&campaign)).unwrap());
        let mut trackers = Trackers::default();
        trackers.poll(Some(index.clone()), Some(&campaign), Some(TONIGHT));
        Race {
            _dir: dir,
            campaign,
            trackers,
            index,
        }
    }

    fn value(race: &Race, key: &str, instance: Option<&str>) -> i64 {
        race.trackers
            .state
            .value(&race.trackers.rules, key, instance)
    }

    fn file(race: &Race) -> String {
        fs::read_to_string(ledger::path(&race.campaign.path)).unwrap_or_default()
    }

    fn newest_label(trackers: &Trackers) -> String {
        label(&trackers.rules, &trackers.ledger, trackers.newest())
    }

    #[test]
    fn the_tournament_note_is_read_as_written_and_a_race_runs_between_fights() {
        let mut race = race();
        let rules = &race.trackers.rules;
        assert_eq!(rules.notes, ["Tournament"]);
        let keys: Vec<_> = rules.trackers.iter().map(|t| t.key.as_str()).collect();
        assert_eq!(
            keys,
            [
                "race_clock",
                "favor",
                "souls",
                "seals",
                "members",
                "activation"
            ]
        );
        for tracker in &rules.trackers {
            assert!(tracker.def.is_ok(), "{}: {:?}", tracker.key, tracker.def);
        }
        assert!(
            !rules.def("activation").unwrap().pin,
            "activation is not pinned"
        );
        // The note is a valid entry: no warning in the Library or the Campaign's review.
        let note = race
            .index
            .items
            .iter()
            .find(|item| item.entry.name == "Tournament")
            .unwrap();
        let checked = splat::content::validate(std::slice::from_ref(&note.entry));
        assert!(
            checked.warnings.is_empty() && checked.errors.is_empty(),
            "{:?} {:?}",
            checked.warnings,
            checked.errors
        );
        assert_eq!(race.trackers.due(), 0);
        assert_eq!(file(&race), "", "nothing is written before an act");

        // A press that passes a mark raises its notice, and the result line names both.
        let t = &mut race.trackers;
        t.press("race_clock", None, "Cistern or Sands", None);
        assert_eq!(t.reason, None);
        assert_eq!(t.due(), 1);
        let words = result_words(&t.rules, &t.ledger, &t.state, t.newest());
        assert_eq!(words, "Cistern or Sands: Elapsed race time 20. Due: Bell.");
        assert_eq!(newest_label(t), "Cistern or Sands");
        // The result line's label and words are worked out with the act, not in each frame.
        assert_eq!(t.result, Some(("Cistern or Sands".into(), words)));

        // `for:` rolls for the next name, and then the button is off.
        let bell = t.state.due[0].clone();
        for name in ["Old Guard", "Keyring", "Convicts"] {
            t.roll_notice(&bell, "rival_track");
            let roll = t
                .state
                .notice(&bell.id)
                .unwrap()
                .rolls
                .last()
                .unwrap()
                .clone();
            assert_eq!(roll.name.as_deref(), Some(name));
            assert_eq!(newest_label(t), "Rival Track roll");
        }
        let before = file(&race);
        let t = &mut race.trackers;
        t.roll_notice(&bell, "rival_track");
        assert_eq!(
            t.reason,
            Some(Unrecorded::Refused("Each name has its roll.".into()))
        );
        assert_eq!(file(&race), before, "a refused act writes nothing");
        // A table with no names rolls as often as it is pressed.
        let t = &mut race.trackers;
        t.roll_notice(&bell, "wander_early");
        t.roll_notice(&bell, "wander_early");
        assert_eq!(t.state.notice(&bell.id).unwrap().rolls.len(), 5);
        t.done(&bell, &[]);
        assert_eq!(t.due(), 0);
        assert_eq!(newest_label(t), "Bell done");
        assert_eq!(
            result_words(&t.rules, &t.ledger, &t.state, t.newest()),
            "Bell done."
        );

        // `min` blocks a press, and says why beside it.
        let before = file(&race);
        let t = &mut race.trackers;
        t.press("favor", None, "Throw a gift", None);
        assert_eq!(
            t.reason,
            Some(Unrecorded::Refused("Crowd Favor cannot go below 0.".into()))
        );
        assert_eq!(file(&race), before);
        let t = &mut race.trackers;
        t.press("favor", None, "Crowd loves it", None);
        t.press("favor", None, "Throw a gift", None);
        let words = result_words(&t.rules, &t.ledger, &t.state, t.newest());
        assert!(
            words.starts_with("Throw a gift: Crowd Favor 0. Rolled ")
                && words.ends_with(": A healing potion."),
            "{words}"
        );

        // An instance has its own value and its own notice.
        t.press("seals", Some("Old Guard"), "Take seal", None);
        assert_eq!(value(&race, "seals", Some("Old Guard")), 1);
        assert_eq!(value(&race, "seals", Some("Party")), 0);
        let t = &mut race.trackers;
        assert_eq!(t.due(), 1);
        assert_eq!(
            short_name(&t.rules, "seals", Some("Old Guard"), 1),
            "First seal for Old Guard"
        );
        // The DM clicks Dismiss on that notice; a moment before, Obsidian moved its seal to
        // Party. The click writes nothing and says so, and the view draws from the file.
        let drawn = t.state.due[0].clone();
        let path = ledger::path(&race.campaign.path);
        let before = fs::read_to_string(&path).unwrap();
        fs::write(&path, before.replace("for: Old Guard", "for: Party")).unwrap();
        let t = &mut race.trackers;
        t.dismiss(&drawn);
        assert_eq!(
            t.reason,
            Some(Unrecorded::Refused(
                "That notice changed on disk. Nothing was recorded.".into()
            ))
        );
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            before.replace("for: Old Guard", "for: Party")
        );
        fs::write(&path, &before).unwrap();
        let t = &mut race.trackers;
        t.read_disk();
        assert_eq!(t.state.due[0], drawn);
        // A wrong press seen late: the opposite action corrects the value, and the history
        // keeps both presses.
        t.press("seals", Some("Old Guard"), "Lose seal", None);
        assert_eq!(value(&race, "seals", Some("Old Guard")), 0);
        let t = &mut race.trackers;
        assert_eq!(
            t.due(),
            1,
            "a mark does not fire, or unfire, when a value falls"
        );

        // One press that changes two trackers is two entries with one `use`; Undo takes both.
        t.press("race_clock", None, "Collect soul", None);
        let pressed = t.newest().to_vec();
        assert_eq!(pressed.len(), 2);
        assert!(pressed[0].press.is_some() && pressed[0].press == pressed[1].press);
        assert_eq!(value(&race, "souls", None), 1);
        let t = &mut race.trackers;
        assert_eq!(
            result_words(&t.rules, &t.ledger, &t.state, t.newest()),
            "Collect soul: Elapsed race time 30, Souls in the jar 1."
        );
        t.undo(&pressed);
        assert_eq!(value(&race, "souls", None), 0);
        assert_eq!(value(&race, "race_clock", None), 20);
        // The line then names the act before it: Undo steps back, newest first.
        assert_eq!(newest_label(&race.trackers), "Lose seal");
        // A dice amount records its roll; `set` records what it replaced.
        let t = &mut race.trackers;
        t.press("activation", None, "Touch controls", None);
        let rolled = value(&race, "activation", None);
        assert!((1..=6).contains(&rolled));
        let t = &mut race.trackers;
        t.press("activation", None, "Missed twice or Ring reset", None);
        assert_eq!(t.newest()[0].was, Some(rolled));
        assert_eq!(value(&race, "activation", None), 0);

        // An act reads the definitions on disk at the click. Done on a mark whose `at` changed
        // in Obsidian, and a press on a tracker whose note was deleted, write nothing.
        let drawn = race.trackers.state.due[0].clone();
        let note = race.campaign.path.join("Tournament.md");
        let text = fs::read_to_string(&note).unwrap();
        assert!(text.contains("at: 1\n"));
        fs::write(&note, text.replace("at: 1\n", "at: 2\n")).unwrap();
        let before = file(&race);
        let t = &mut race.trackers;
        t.done(&drawn, &[]);
        assert_eq!(
            t.reason,
            Some(Unrecorded::Refused(
                "A house rule note changed on disk. Nothing was recorded.".into()
            ))
        );
        assert_eq!(file(&race), before);
        fs::remove_file(&note).unwrap();
        let t = &mut race.trackers;
        t.press("favor", None, "Crowd loves it", None);
        assert_eq!(
            t.reason,
            Some(Unrecorded::Refused(
                "A house rule note changed on disk. Nothing was recorded.".into()
            ))
        );
        assert_eq!(file(&race), before);
    }

    #[test]
    fn a_linked_change_names_a_line_its_tracker_has() {
        let def = |yaml: &str| serde_yaml_ng::from_str::<Definition>(yaml).unwrap();
        let rules = Rules {
            notes: vec!["Rules".into()],
            trackers: vec![
                Tracker {
                    key: "teams".into(),
                    note: Uuid::nil(),
                    def: Ok(def(
                        "name: Teams\nper: [Party]\nquick:\n- label: Score\n  changes: [{tracker: rivals, add: 1}]",
                    )),
                },
                Tracker {
                    key: "rivals".into(),
                    note: Uuid::nil(),
                    def: Ok(def("name: Rivals\nper: [Old Guard]")),
                },
            ],
            ..Rules::default()
        };
        // A nested `add: ask` asks for the amount as a top-level one does.
        assert!(def("name: Gold\nquick: [{label: Spend, changes: [{add: -ask}]}]").quick[0].asks());
        assert!(!rules.def("teams").unwrap().quick[0].asks());
        let teams = rules.def("teams").unwrap();
        assert_eq!(
            check(teams, &rules),
            Err("Score: rivals has no line for Party.".into())
        );
        let state = replay(&rules, &Ledger::default());
        assert_eq!(
            press_op(&rules, &state, "teams", Some("Party"), "Score", None, None),
            Err("Rivals has no line for Party. Nothing was recorded.".into())
        );
    }

    #[test]
    fn undo_and_remove_of_a_change_that_raised_a_notice_take_the_notice_and_its_entries() {
        let mut race = race();
        let t = &mut race.trackers;
        t.press("race_clock", None, "Cistern or Sands", None);
        let bell = t.state.due[0].clone();
        let raised = t.newest().to_vec();
        // The roll is made with no session being played, so it has another stamp.
        t.played = None;
        t.roll_notice(&bell, "wander_early");
        t.played = Some(TONIGHT);
        t.settle();
        let target = t.newest().to_vec();
        assert_eq!(target[0], raised[0]);
        assert_eq!(target.len(), 2, "the change and the roll on its notice");
        assert_eq!(newest_label(t), "Cistern or Sands");
        t.undo(&target);
        assert!(t.ledger.changes.is_empty());
        assert_eq!(t.due(), 0);

        // An older session: two rooms, the bell they raised, its roll and its Done.
        let note = t.rules.trackers[0].note;
        t.played = Some(Uuid::from_u128(1));
        t.press("race_clock", None, "Enter room", None);
        t.press("race_clock", None, "Enter room", None);
        let bell = t.state.due[0].clone();
        t.roll_notice(&bell, "wander_early");
        t.done(&bell, &[]);
        t.played = Some(TONIGHT);
        t.settle();
        let rows: Vec<&str> = t.history["race_clock"]
            .iter()
            .map(|row| row.words.as_str())
            .collect();
        assert_eq!(rows[..2], ["Enter room: +10", "Enter room: +10"]);
        assert!(rows[2].starts_with("Wandering, 0:00 to 0:39 roll: "));
        assert_eq!(rows[3], "Bell done.");
        assert_eq!(t.history["race_clock"][1].value, Some(20));
        // The first room is what the second room's crossing depends on: Remove of it would
        // leave the roll and the Done with no notice, so it writes nothing and names them.
        let before = file(&race);
        let t = &mut race.trackers;
        let first = t.history["race_clock"][0].entry.clone();
        t.remove(note, &first);
        assert_eq!(
            t.history_reason,
            Some((
                note,
                Unrecorded::Refused(
                    "Wandering, 0:00 to 0:39 roll depends on this entry. Remove that first. Nothing was removed."
                        .into()
                )
            ))
        );
        assert_eq!((file(&race), race.trackers.removed.clone()), (before, None));
        // The crossing itself leaves with its notice's roll and Done, as Undo of it would.
        let t = &mut race.trackers;
        let second = t.history["race_clock"][1].entry.clone();
        let kept = t.ledger.clone();
        t.remove(note, &second);
        assert_eq!(t.history_reason, None);
        assert_eq!(
            t.removed.as_ref().map(|r| r.words.as_str()),
            Some("Removed: Enter room. With it: Wandering, 0:00 to 0:39 roll; Bell done.")
        );
        assert_eq!(t.ledger.changes, kept.changes[..1]);
        assert_eq!(
            (value(&race, "race_clock", None), race.trackers.due()),
            (10, 0)
        );

        // A value that an edit in Obsidian forced below its `min` counts as written, and joins
        // the problems list of its note; Splat changes nothing.
        let path = ledger::path(&race.campaign.path);
        let forced = "---\ntrackers_version: 1\nchanges:\n- id: c-0b51de01\n  tracker: favor\n  add: -2\n---\n";
        fs::write(&path, forced).unwrap();
        let t = &mut race.trackers;
        t.read_disk();
        assert_eq!(value(&race, "favor", None), -2);
        assert!(
            race.trackers
                .problems
                .contains(&(Some(note), "Crowd Favor is -2, below its min of 0.".into()))
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), forced);
    }

    #[test]
    fn each_write_path_of_the_ledger_holds() {
        let mut race = race();
        let path = ledger::path(&race.campaign.path);
        // An edit on disk a moment before: the act is applied to the file as it is now.
        fs::write(
            &path,
            "---\ntrackers_version: 1\nchanges:\n- id: c-0b51de00\n  tracker: favor\n  add: 2\n  why: Typed in Obsidian\n---\n",
        )
        .unwrap();
        let t = &mut race.trackers;
        t.press("favor", None, "Throw a gift", None);
        assert_eq!(t.reason, None);
        assert_eq!(t.ledger.changes[0].id, "c-0b51de00");
        assert_eq!(value(&race, "favor", None), 1);

        // A, then an unrelated B with another stamp, then Undo of A: B's change stays.
        let t = &mut race.trackers;
        t.press("favor", None, "Crowd loves it", None);
        let a = t.newest().to_vec();
        t.played = Some(Uuid::from_u128(2));
        t.press("souls", None, "Crowd loves it", None);
        assert!(t.reason.is_some(), "souls has no such action");
        t.press("race_clock", None, "Enter room", None);
        t.played = Some(TONIGHT);
        t.settle();
        assert_eq!(t.newest(), a.as_slice());
        t.undo(&a);
        assert_eq!(value(&race, "favor", None), 1);
        assert_eq!(value(&race, "race_clock", None), 10, "B’s change stays");

        // `Remove` on a row with another stamp, in the reader of its note. The entry it was
        // drawn from must be on disk as it was read; its words stay in that reader, not on the
        // view's result line.
        let t = &mut race.trackers;
        let note = t.rules.trackers[0].note;
        let older = t.history["favor"][0].entry.clone();
        assert_eq!(older.id, "c-0b51de00");
        let text = fs::read_to_string(&path).unwrap();
        let edited = text.replace("why: Typed in Obsidian", "why: Typed again");
        fs::write(&path, &edited).unwrap();
        t.remove(note, &older);
        assert_eq!(
            t.history_reason,
            Some((
                note,
                Unrecorded::Refused("That entry changed on disk. Nothing was removed.".into())
            ))
        );
        assert_eq!(t.reason, None);
        assert_eq!(fs::read_to_string(&path).unwrap(), edited);
        fs::write(&path, &text).unwrap();
        // Remove A, an unrelated act B, then Undo of A: A goes back where it was, and B stays.
        let t = &mut race.trackers;
        t.read_disk();
        t.remove(note, &older);
        assert_eq!(
            t.removed.as_ref().map(|r| r.words.as_str()),
            Some("Removed: Typed in Obsidian.")
        );
        assert_eq!(value(&race, "favor", None), -1, "Splat clamps nothing");
        let t = &mut race.trackers;
        assert!(
            t.problems
                .contains(&(Some(note), "Crowd Favor is -1, below its min of 0.".into()))
        );
        t.press("race_clock", None, "Enter room", None);
        let b = t.newest().to_vec();
        t.undo_remove();
        assert_eq!(t.removed, None);
        assert_eq!(t.ledger.changes[0], older);
        assert_eq!(t.ledger.changes.last(), b.last());
        assert_eq!(value(&race, "favor", None), 1);
        let t = &mut race.trackers;
        t.undo(&b);
        // An Undo whose entry changed on disk puts nothing back, says so, and ends.
        t.remove(note, &older);
        ledger::commit(&path, |_| {
            Ok(Op::Append(vec![Entry {
                add: Some(5),
                ..older.clone()
            }]))
        })
        .unwrap();
        let outside = fs::read_to_string(&path).unwrap();
        let t = &mut race.trackers;
        t.read_disk();
        t.undo_remove();
        assert_eq!(
            t.history_reason,
            Some((
                note,
                Unrecorded::Refused("That entry changed on disk. Nothing was put back.".into())
            ))
        );
        assert_eq!(
            (t.removed.is_none(), fs::read_to_string(&path).unwrap()),
            (true, outside)
        );
        fs::write(&path, &text).unwrap();
        race.trackers.read_disk();
        assert_eq!(value(&race, "favor", None), 1);

        // A failed save writes nothing and keeps nothing: the file stays byte for byte, and the
        // value and the result line stay as the file gives them. The result line says why.
        let entries = race.trackers.ledger.changes.len();
        let before = fs::read(&path).unwrap();
        let shown = race.trackers.result.clone();
        let mut readonly = fs::metadata(&race.campaign.path).unwrap().permissions();
        use std::os::unix::fs::PermissionsExt;
        readonly.set_mode(0o555);
        fs::set_permissions(&race.campaign.path, readonly.clone()).unwrap();
        let t = &mut race.trackers;
        t.press("favor", None, "Crowd loves it", None);
        assert_eq!(fs::read(&path).unwrap(), before);
        let Some(Unrecorded::NotSaved(error)) = &t.reason else {
            panic!("the result line says the save failed: {:?}", t.reason);
        };
        let words = view::not_saved(error);
        assert!(
            words.starts_with("Trackers.md was not saved: ")
                && words.ends_with(". Nothing was recorded."),
            "{words}"
        );
        assert_eq!(t.result, shown);
        assert_eq!(value(&race, "favor", None), 1);
        // A `Remove` whose save fails records nothing, and nothing waits.
        let t = &mut race.trackers;
        t.remove(note, &older);
        assert!(matches!(
            t.history_reason,
            Some((_, Unrecorded::NotSaved(_)))
        ));
        assert_eq!(
            (t.removed.clone(), fs::read(&path).unwrap()),
            (None, before.clone())
        );
        // An edit in Obsidian a moment before a failed save: the view shows what the file holds.
        let outside = String::from_utf8(before.clone()).unwrap().replace(
            "  add: 2\n  why: Typed in Obsidian",
            "  add: 8\n  why: Typed in Obsidian",
        );
        fs::write(&path, &outside).unwrap();
        let t = &mut race.trackers;
        t.press("favor", None, "Crowd loves it", None);
        assert_eq!(fs::read_to_string(&path).unwrap(), outside);
        assert!(matches!(t.reason, Some(Unrecorded::NotSaved(_))));
        assert_eq!(value(&race, "favor", None), 7);
        fs::write(&path, &before).unwrap();
        race.trackers.read_disk();
        // A number typed for `add: ask` stays in its field when the save fails.
        let note = race.campaign.path.join("Tournament.md");
        let text = fs::read_to_string(&note).unwrap();
        let with_ask = text.replace(
            "label: Crowd loves it\n>       add: 1\n",
            "label: Crowd loves it\n>       add: 1\n>     - label: Bet\n>       add: ask\n",
        );
        assert_ne!(with_ask, text);
        fs::write(&note, with_ask).unwrap();
        let index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
        let t = &mut race.trackers;
        t.poll(Some(index), Some(&race.campaign), Some(TONIGHT));
        t.press("favor", None, "Bet", Some(3));
        assert!(matches!(t.reason, Some(Unrecorded::NotSaved(_))));
        assert_eq!(t.view.typed(), Some("3"));
        assert_eq!(fs::read(&path).unwrap(), before);
        // The next press is one more act, and writes exactly one entry.
        readonly.set_mode(0o755);
        fs::set_permissions(&race.campaign.path, readonly.clone()).unwrap();
        let t = &mut race.trackers;
        t.press("favor", None, "Crowd loves it", None);
        assert_eq!(t.reason, None);
        assert_eq!(t.ledger.changes.len(), entries + 1);
        assert_eq!(value(&race, "favor", None), 2);

        // `Add to Timeline` whose Timeline save fails says the Timeline's own words, not
        // success. The event waits in the Timeline's draft, and saves later.
        let mut app = crate::Splat {
            trackers: std::mem::take(&mut race.trackers),
            ..Default::default()
        };
        app.timeline.open(Some(&race.campaign.path));
        let entry = app.trackers.newest()[0].id.clone();
        app.trackers.wants_timeline = Some((entry, "Crowd loves it".into()));
        readonly.set_mode(0o555);
        fs::set_permissions(&race.campaign.path, readonly.clone()).unwrap();
        app.add_tracker_act_to_timeline();
        app.timeline.save();
        readonly.set_mode(0o755);
        fs::set_permissions(&race.campaign.path, readonly.clone()).unwrap();
        // The shell's next frame.
        app.add_tracker_act_to_timeline();
        let words = app
            .trackers
            .added
            .as_ref()
            .and_then(|added| added.not_saved.clone());
        assert!(
            words
                .as_deref()
                .is_some_and(|words| words.starts_with("The timeline was not saved.")),
            "{words:?}"
        );
        app.timeline.save();
        app.add_tracker_act_to_timeline();
        assert_eq!(
            app.trackers
                .added
                .as_ref()
                .map(|added| added.not_saved.clone()),
            Some(None)
        );
        race.trackers = std::mem::take(&mut app.trackers);

        // A restart: the values, the result line and its Undo come from the file.
        let mut again = Trackers::default();
        again.poll(
            Some(race.index.clone()),
            Some(&race.campaign),
            Some(TONIGHT),
        );
        assert_eq!(again.state.value(&again.rules, "favor", None), 2);
        assert_eq!(newest_label(&again), "Crowd loves it");
        assert_eq!(
            again.result.as_ref().map(|(label, _)| label.as_str()),
            Some("Crowd loves it")
        );
        let newest = again.newest().to_vec();
        again.undo(&newest);
        assert_eq!(again.state.value(&again.rules, "favor", None), 1);
        // The other Trackers still holds the old Undo, and the file no longer has it as the
        // newest act of tonight: it writes nothing and says so.
        let before = fs::read(&path).unwrap();
        let stale = race.trackers.newest().to_vec();
        race.trackers.undo(&stale);
        assert_eq!(
            race.trackers.reason,
            Some(Unrecorded::Refused(
                "The ledger changed on disk. Nothing was undone.".into()
            ))
        );
        assert_eq!(fs::read(&path).unwrap(), before);

        // `Campaign.md` that cannot be read is no empty list of rules: the act writes nothing,
        // no value shows, and the words name the file.
        let readable = fs::read(&path).unwrap();
        assert_eq!(value(&race, "favor", None), 1);
        let manifest = race.campaign.path.join("Campaign.md");
        fs::set_permissions(&manifest, fs::Permissions::from_mode(0o000)).unwrap();
        let t = &mut race.trackers;
        t.press("favor", None, "Crowd loves it", None);
        fs::set_permissions(&manifest, fs::Permissions::from_mode(0o644)).unwrap();
        assert_eq!(fs::read(&path).unwrap(), readable);
        let Some(Unrecorded::Refused(why)) = &t.reason else {
            panic!("{:?}", t.reason);
        };
        assert!(
            why.starts_with("Campaign.md cannot be read: ")
                && why.ends_with("Nothing was recorded."),
            "{why}"
        );
        assert!(t.in_play() && t.rules.unreadable.is_some());
        assert_eq!((t.due(), value(&race, "favor", None)), (0, 0));

        // A file that cannot be read shows nothing old, and is never written over.
        fs::write(&path, "---\nchanges: [\n---\n").unwrap();
        let t = &mut race.trackers;
        t.read_disk();
        assert!(t.unreadable.is_some());
        assert_eq!(t.due(), 0);
        assert_eq!(t.state.value(&t.rules, "favor", None), 0);
        t.press("favor", None, "Crowd loves it", None);
        assert_eq!(fs::read_to_string(&path).unwrap(), "---\nchanges: [\n---\n");
    }

    #[test]
    fn no_note_in_play_reads_no_ledger_and_shows_nothing() {
        let race = race();
        // A note in play that was deleted keeps its row in Campaign → Rules and runs, so that the
        // DM can switch it off.
        fs::remove_file(race.campaign.path.join("Tournament.md")).unwrap();
        let mut options = Options::load(
            &race.campaign,
            &crate::content_workspace::Catalogue::load(&[], Some(&race.campaign)).unwrap(),
        )
        .unwrap();
        let id = options.in_play[0];
        assert_eq!(options.house_rules, [(id, format!("{id} (not found)"))]);
        options.in_play.clear();
        let document = CampaignDocument::load(&race.campaign).unwrap();
        options.save(&race.campaign, &document.text).unwrap();
        assert!(
            !fs::read_to_string(race.campaign.path.join("Campaign.md"))
                .unwrap()
                .contains("splat_in_play")
        );
        fs::write(ledger::path(&race.campaign.path), "not a ledger").unwrap();
        let index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
        let mut trackers = Trackers::default();
        trackers.poll(Some(index), Some(&race.campaign), None);
        assert!(!trackers.in_play());
        assert_eq!(trackers.due(), 0);
        assert!(trackers.unreadable.is_none(), "the ledger is not read");
    }
}
