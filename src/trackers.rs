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
    /// The line of a tracker with `per` that the change is for. With none, the line the act
    /// was made on. A day mark names it, because a day has no line of its own.
    #[serde(default, rename = "for")]
    pub instance: Option<String>,
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

/// A mark on the campaign's day count: at one day, or every so many days from a day. It raises
/// a notice as a mark at a value does, and never acts.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DayMark {
    /// The row's name. With none, the first sentence of `say`.
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub at: Option<u64>,
    #[serde(default)]
    pub every: Option<u64>,
    #[serde(default)]
    pub from: Option<u64>,
    pub say: String,
    #[serde(default)]
    pub tables: Vec<MarkTable>,
    #[serde(default)]
    pub changes: Vec<Change>,
}

impl DayMark {
    /// The days it comes due on, up to `today`, oldest first. `at: 721` is the start of Day
    /// 721. `every: 30, from: 2` is Day 32, Day 62 and so on; with no `from`, a schedule counts
    /// from Day 1, the clock's first day.
    pub fn days_until(&self, today: u64) -> Vec<u64> {
        match (self.at, self.every) {
            (Some(at), None) => (at <= today).then_some(at).into_iter().collect(),
            (None, Some(every)) if every > 0 => {
                let from = self.from.unwrap_or(1);
                (1..)
                    .map(|n: u64| from.saturating_add(n.saturating_mul(every)))
                    .take_while(|day| *day <= today && *day < u64::MAX)
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    /// The row's name for reading.
    pub fn title(&self) -> &str {
        self.name
            .as_deref()
            .unwrap_or_else(|| first_sentence(&self.say))
    }

    pub fn says(&self) -> Says<'_> {
        Says {
            say: &self.say,
            tables: &self.tables,
            changes: &self.changes,
        }
    }
}

/// What a notice says and offers: a mark at a value's, or a day mark's.
#[derive(Clone, Copy)]
pub struct Says<'a> {
    pub say: &'a str,
    pub tables: &'a [MarkTable],
    pub changes: &'a [Change],
}

impl Mark {
    pub fn says(&self) -> Says<'_> {
        Says {
            say: &self.say,
            tables: &self.tables,
            changes: &self.changes,
        }
    }
}

/// The first sentence of a notice's words: its short name.
fn first_sentence(say: &str) -> &str {
    say.split_once('.').map_or(say, |(first, _)| first).trim()
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

/// A day mark of a note in play, or why its definition cannot be used.
#[derive(Clone, Debug, PartialEq)]
pub struct Dated {
    pub key: String,
    pub def: Result<DayMark, String>,
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
    /// Every day mark of those notes, in the notes' order.
    pub day_marks: Vec<Dated>,
    /// A note in play that cannot be used, with the reason, and the note when it is known: a note
    /// in play whose file is gone or moved keeps its problem in its own reader.
    pub problems: Vec<(Option<Uuid>, String)>,
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
            &self.day_marks,
            &self.problems,
            &self.unreadable,
        ) == (
            &other.notes,
            &other.trackers,
            &other.day_marks,
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

    pub fn day_mark(&self, key: &str) -> Option<&DayMark> {
        self.day_marks
            .iter()
            .find(|mark| mark.key == key)
            .and_then(|mark| mark.def.as_ref().ok())
    }

    /// A day mark's name for reading, or its key when the mark cannot be used.
    pub fn day_title<'a>(&'a self, key: &'a str) -> &'a str {
        self.day_mark(key).map_or(key, DayMark::title)
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
                problems: vec![(None, error)],
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
            notes.push(Err((
                id,
                format!(
                    "A house rule note in play is not found: {id}. Switch it off in Campaign → Rules and runs."
                ),
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
                Ok(_) => notes.push(Err((id, moved))),
                Err(error) => return Rules::cannot_read(format!("{path} cannot be read: {error}")),
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                notes.push(Err((id, moved)))
            }
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
fn read_rules(
    tables: Arc<Sources>,
    notes: Vec<Result<splat::content::Entry, (Uuid, String)>>,
) -> Rules {
    let mut rules = Rules {
        tables: Some(tables),
        ..Rules::default()
    };
    let mut owners: HashMap<String, Vec<String>> = HashMap::new();
    for note in notes {
        let entry = match note {
            Ok(entry) => entry,
            Err((id, problem)) => {
                rules.problems.push((Some(id), problem));
                continue;
            }
        };
        let entry = &entry;
        if !is_house_rule_note(entry) {
            rules.problems.push((
                Some(entry.id),
                format!(
                    "{} is in play, but it is not a house rule note.",
                    entry.name
                ),
            ));
            continue;
        }
        rules.notes.push(entry.name.clone());
        let members = |part: &str| {
            entry
                .fields
                .get("splat_members")
                .and_then(|members| members.get(part))
                .and_then(Value::as_mapping)
                .into_iter()
                .flatten()
                .filter_map(|(key, value)| Some((key.as_str()?.to_owned(), value.clone())))
        };
        for (key, _) in members("trackers").chain(members("day_marks")) {
            owners.entry(key).or_default().push(entry.name.clone());
        }
        for (key, value) in members("trackers") {
            rules.trackers.push(Tracker {
                key,
                def: serde_yaml_ng::from_value::<Definition>(value)
                    .map_err(|error| error.to_string()),
                note: entry.id,
            });
        }
        for (key, value) in members("day_marks") {
            rules.day_marks.push(Dated {
                key,
                def: serde_yaml_ng::from_value::<DayMark>(value).map_err(|error| error.to_string()),
                note: entry.id,
            });
        }
    }
    // A key names one member across the notes in play: a tracker or a day mark.
    let shared = |key: &str| {
        owners.get(key).filter(|o| o.len() > 1).map(|notes| {
            format!(
                "The key {key} is in {}. A key names one tracker or day mark.",
                notes.join(" and ")
            )
        })
    };
    let checked: Vec<_> = rules
        .trackers
        .iter()
        .map(
            |tracker| match (&tracker.def, shared(&tracker.key).as_ref()) {
                (Err(error), _) | (Ok(_), Some(error)) => Err(error.clone()),
                (Ok(def), None) => check(def, &rules).map(|()| def.clone()),
            },
        )
        .collect();
    let dated: Vec<_> = rules
        .day_marks
        .iter()
        .map(|mark| match (&mark.def, shared(&mark.key).as_ref()) {
            (Err(error), _) | (Ok(_), Some(error)) => Err(error.clone()),
            (Ok(def), None) => check_day_mark(def, &rules).map(|()| def.clone()),
        })
        .collect();
    for (tracker, def) in rules.trackers.iter_mut().zip(checked) {
        tracker.def = def;
    }
    for (mark, def) in rules.day_marks.iter_mut().zip(dated) {
        mark.def = def;
    }
    rules
}

/// A table key resolves to a marked table of the campaign.
fn check_table(rules: &Rules, key: &str) -> Result<(), String> {
    match &rules.tables {
        Some(tables) => tables.problem(key).map_or(Ok(()), Err),
        None => Err(format!("No roll table has the key “{key}”.")),
    }
}

/// A change names a tracker in play and a line that tracker has, and one amount. `own` is the
/// tracker the change belongs to; a day mark has none, so its changes name their tracker.
fn check_change(
    rules: &Rules,
    own: Option<&Definition>,
    change: &Change,
    alone: bool,
) -> Result<(), String> {
    let (key, target) = match &change.tracker {
        Some(key) => {
            let Some(target) = rules.trackers.iter().find(|t| t.key == *key) else {
                return Err(format!("No tracker in play has the key “{key}”."));
            };
            (key.as_str(), target.def.as_ref().ok())
        }
        None => (
            "This tracker",
            Some(own.ok_or("A day mark’s change names its tracker.")?),
        ),
    };
    if let Some(target) = target {
        match (&change.instance, target.per.is_empty()) {
            (Some(_), true) => {
                return Err(format!(
                    "{key} has no per, so a change to it names no line."
                ));
            }
            (Some(word), false) if !target.per.contains(word) => {
                return Err(format!("{key} has no line for {word}."));
            }
            (None, false) if change.tracker.is_some() => {
                let lines = own.map_or(&[][..], |own| own.per.as_slice());
                if lines.is_empty() {
                    return Err(format!("{key} has per, so a change to it needs a name."));
                }
                if let Some(word) = lines.iter().find(|word| !target.per.contains(word)) {
                    return Err(format!("{key} has no line for {word}."));
                }
            }
            _ => {}
        }
    }
    match (&change.add, change.set) {
        (Some(add), None) => match add.read()? {
            Add::Ask { .. } if !alone => Err("ask works only in an action of one change.".into()),
            _ => Ok(()),
        },
        (None, Some(_)) => Ok(()),
        _ => Err("A change has add or set.".into()),
    }
}

/// A notice's tables and proposed changes resolve, and no proposed change asks.
fn check_says(rules: &Rules, own: Option<&Definition>, says: Says<'_>) -> Result<(), String> {
    for t in says.tables {
        check_table(rules, t.key())?;
    }
    for c in says.changes {
        check_change(rules, own, c, false)?;
        if matches!(c.add.as_ref().map(Amount::read), Some(Ok(Add::Ask { .. }))) {
            return Err("A mark’s change cannot ask.".into());
        }
    }
    Ok(())
}

/// Every word of a definition resolves: trackers, tables, instances and amounts.
fn check(def: &Definition, rules: &Rules) -> Result<(), String> {
    if let Start::Per(values) = &def.start
        && let Some(word) = values.keys().find(|word| !def.per.contains(word))
    {
        return Err(format!("start names “{word}”, which is not in per."));
    }
    for quick in &def.quick {
        let one = Change {
            tracker: None,
            instance: None,
            add: quick.add.clone(),
            set: quick.set,
        };
        let change = |c: &Change, alone| check_change(rules, Some(def), c, alone);
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
            check_table(rules, key).map_err(|error| format!("{}: {error}", quick.label))?;
        }
    }
    for mark in &def.marks {
        check_says(rules, Some(def), mark.says())?;
    }
    Ok(())
}

/// A day mark has `at` or `every`, a day of 1 or more, `from` only with `every`, and words that
/// resolve.
fn check_day_mark(mark: &DayMark, rules: &Rules) -> Result<(), String> {
    match (mark.at, mark.every, mark.from) {
        (Some(0), None, _) | (None, Some(0), _) => {
            return Err("A day mark’s day is 1 or more.".into());
        }
        (Some(_), None, Some(_)) => return Err("from works only with every.".into()),
        (Some(_), None, _) | (None, Some(_), _) => {}
        _ => return Err("A day mark has at or every.".into()),
    }
    check_says(rules, None, mark.says())
}

// ---------------------------------------------------------------------------------------------
// The replay: values and notices are worked out from the entries, never stored.

/// The campaign's day count, as the Timeline holds it now: its one owner is `timeline::Book`.
#[derive(Clone, Debug, PartialEq)]
pub enum Today {
    /// The day the story is on; 0 before the first `Day`.
    Day(u64),
    /// `Timeline.md` cannot be read, with the Timeline's words. The day marks freeze: no day
    /// notice is raised or cleared, and no act on one is recorded.
    NotRead(String),
}

impl Default for Today {
    fn default() -> Self {
        Self::Day(0)
    }
}

/// The one answer to "what is the day count now, or why is it not known": the view, the rail's
/// count and every act read it. Not known while `Timeline.md` cannot be read, while it changed
/// or cannot be read under an edit that waits to be saved, and while that waits for the DM's
/// choice; then no day notice shows, and the Timeline keeps the draft.
fn today(book: &crate::timeline::Book) -> Today {
    let why = book
        .unreadable
        .clone()
        .or_else(|| book.moved_under_edit.clone())
        .or_else(|| {
            book.conflict
                .then(|| crate::timeline::CHANGED_UNDER_DRAFT.to_owned())
        });
    match why {
        Some(why) => Today::NotRead(why),
        None => Today::Day(crate::timeline::day_now(&book.events).unwrap_or(0)),
    }
}

/// An open notice, with its rolls so far.
#[derive(Clone, Debug, PartialEq)]
pub struct Notice {
    pub id: NoticeRef,
    /// The member that raised it: the tracker whose change crossed a mark, or the day mark.
    pub key: String,
    pub instance: Option<String>,
    pub rolls: Vec<TableRoll>,
    /// A day mark's open days, oldest first: one row for them all, which acts on the first.
    /// Empty for a mark at a value.
    pub days: Vec<u64>,
}

/// What the ledger says now, for the definitions in play.
#[derive(Clone, Debug, Default)]
pub struct State {
    values: HashMap<(String, Option<String>), i64>,
    /// The open notices, oldest first: the day marks' rows, then the marks at a value.
    pub due: Vec<Notice>,
    /// The value of each change entry right after it.
    after: HashMap<String, i64>,
    /// The marks each change entry crossed.
    raised: HashMap<String, Vec<i64>>,
    /// The notices each change entry ended by taking the value back below their marks, by mark.
    lowered: HashMap<String, Vec<i64>>,
    /// How many entries name each tracker that no note in play has, in the file's order. They
    /// count for nothing, and the view names them.
    pub unknown: Vec<(String, usize)>,
    /// Why the day count is not known while day marks are in play. Then no day notice shows.
    pub clock_not_read: Option<String>,
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

    /// Did a change raise this notice, open or ended? A day notice comes from the day count,
    /// which no entry of the ledger moves.
    fn raised(&self, id: &NoticeRef) -> bool {
        match id {
            NoticeRef::Value { change, at } => self
                .raised
                .get(change)
                .is_some_and(|marks| marks.contains(at)),
            NoticeRef::Day { .. } => true,
        }
    }

    fn count_unknown(&mut self, key: &str) {
        match self.unknown.iter_mut().find(|(known, _)| known == key) {
            Some((_, count)) => *count += 1,
            None => self.unknown.push((key.to_owned(), 1)),
        }
    }
}

/// The value is a plain sum: `start`, then every change in ledger order. A row for a tracker
/// that is not in play is kept in the file and counts for nothing here. A day mark's notices
/// are its days up to `today` that no entry ends; raising one writes nothing.
pub fn replay(rules: &Rules, ledger: &Ledger, today: &Today) -> State {
    let mut state = State::default();
    // The entries about day notices: which days ended, and the rolls on each.
    let mut ended = std::collections::HashSet::new();
    let mut day_rolls: HashMap<NoticeRef, Vec<TableRoll>> = HashMap::new();
    // The notices at a value that an entry anywhere in the ledger names: a roll on one, its
    // Done or its Dismiss. A fall never ends one of them, whatever act or edit put the fall
    // before the entry.
    let acted_on: std::collections::HashSet<&NoticeRef> = ledger
        .changes
        .iter()
        .filter_map(|entry| entry.notice.as_ref())
        .collect();
    for entry in &ledger.changes {
        if let Some(id @ NoticeRef::Day { mark, .. }) = &entry.notice {
            if rules.day_mark(mark).is_none() {
                if !rules.day_marks.iter().any(|m| m.key == *mark) {
                    state.count_unknown(mark);
                }
            } else if entry.done || entry.dismissed {
                ended.insert(id.clone());
            } else if let Some(Roll::Table(roll)) = &entry.roll {
                day_rolls.entry(id.clone()).or_default().push(roll.clone());
            }
            continue;
        }
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
                state.count_unknown(key);
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
        end_below_marks(&mut state, &acted_on, entry, after);
        for mark in &def.marks {
            if before < mark.at && mark.at <= after {
                state.due.push(Notice {
                    id: NoticeRef::Value {
                        change: entry.id.clone(),
                        at: mark.at,
                    },
                    key: key.clone(),
                    instance: entry.instance.clone(),
                    rolls: Vec::new(),
                    days: Vec::new(),
                });
                state
                    .raised
                    .entry(entry.id.clone())
                    .or_default()
                    .push(mark.at);
            }
        }
    }
    let marks = rules
        .day_marks
        .iter()
        .filter_map(|m| m.def.as_ref().ok().map(|def| (m.key.as_str(), def)));
    let today = match today {
        Today::Day(day) => *day,
        Today::NotRead(why) => {
            if marks.clone().next().is_some() {
                state.clock_not_read = Some(why.clone());
            }
            return state;
        }
    };
    let mut dated: Vec<Notice> = marks
        .filter_map(|(key, def)| {
            let id = |day| NoticeRef::Day {
                mark: key.to_owned(),
                day,
            };
            let days: Vec<u64> = def
                .days_until(today)
                .into_iter()
                .filter(|day| !ended.contains(&id(*day)))
                .collect();
            let first = id(*days.first()?);
            Some(Notice {
                rolls: day_rolls.remove(&first).unwrap_or_default(),
                id: first,
                key: key.to_owned(),
                instance: None,
                days,
            })
        })
        .collect();
    // Oldest first; marks due on one day keep their notes' order.
    dated.sort_by_key(|notice| notice.days[0]);
    dated.append(&mut state.due);
    state.due = dated;
    state
}

/// A notice at a value is due because the value is at or past its mark. A change that takes the
/// value back below the mark ends each such notice of its line that no entry of the ledger
/// names (no roll, `Done` or `Dismiss`, before or after the fall): a wrong press and its
/// opposite leave nothing due. A notice the DM acted on stays. Nothing is written; Undo of the
/// change brings the notice back.
fn end_below_marks(
    state: &mut State,
    acted_on: &std::collections::HashSet<&NoticeRef>,
    entry: &Entry,
    after: i64,
) {
    let mut ended = Vec::new();
    state.due.retain(|notice| match &notice.id {
        NoticeRef::Value { at, .. }
            if *at > after
                && Some(&notice.key) == entry.tracker.as_ref()
                && notice.instance == entry.instance
                && !acted_on.contains(&notice.id) =>
        {
            ended.push(*at);
            false
        }
        _ => true,
    });
    if !ended.is_empty() {
        state.lowered.insert(entry.id.clone(), ended);
    }
}

/// The notice the DM clicked, as the files on disk hold it now: the same change and mark, for
/// the same tracker and name, or the same day of the same day mark. An outside edit that moved
/// it, or a day count that went back, writes nothing.
fn open_as_drawn<'a>(state: &'a State, drawn: &Notice) -> Result<&'a Notice, String> {
    if let (NoticeRef::Day { .. }, Some(why)) = (&drawn.id, &state.clock_not_read) {
        return Err(format!("{why} Nothing was recorded."));
    }
    let open = state
        .notice(&drawn.id)
        .ok_or("That notice is no longer due. Nothing was recorded.")?;
    if open.key != drawn.key || open.instance != drawn.instance {
        return Err("That notice changed on disk. Nothing was recorded.".into());
    }
    Ok(open)
}

/// What a notice says and offers: its mark's, or its day mark's.
pub fn mark_of<'a>(rules: &'a Rules, notice: &Notice) -> Option<Says<'a>> {
    match &notice.id {
        NoticeRef::Value { at, .. } => rules
            .def(&notice.key)?
            .marks
            .iter()
            .find(|mark| mark.at == *at)
            .map(Mark::says),
        NoticeRef::Day { mark, .. } => rules.day_mark(mark).map(DayMark::says),
    }
}

/// A notice's short name: the first sentence of its words, and the name it is for.
pub fn short_name(rules: &Rules, tracker: &str, instance: Option<&str>, at: i64) -> String {
    let say = rules
        .def(tracker)
        .and_then(|def| def.marks.iter().find(|mark| mark.at == at))
        .map_or("Notice", |mark| mark.say.as_str());
    let first = first_sentence(say);
    match instance {
        Some(name) => format!("{first} for {name}"),
        None => first.to_owned(),
    }
}

/// A day notice's short name: the mark's name and the day it is for.
fn day_name(rules: &Rules, mark: &str, day: u64) -> String {
    format!("{}, Day {day}", rules.day_title(mark))
}

impl Notice {
    /// Its short name, as the row's actions and the result line say it.
    pub fn name(&self, rules: &Rules) -> String {
        match &self.id {
            NoticeRef::Value { at, .. } => {
                short_name(rules, &self.key, self.instance.as_deref(), *at)
            }
            NoticeRef::Day { mark, day } => day_name(rules, mark, *day),
        }
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
                    .and_then(NoticeRef::change)
                    .is_some_and(|change| out.iter().any(|kept| kept.id == change))
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

/// A notice's short name, from the ledger and the definitions alone: a day notice's mark and
/// day, or the first sentence of a mark's words with the line of the change that raised it.
fn notice_name(rules: &Rules, ledger: &Ledger, notice: &NoticeRef) -> String {
    match notice {
        NoticeRef::Value { change, at } => {
            let raised = ledger.changes.iter().find(|entry| entry.id == *change);
            short_name(
                rules,
                raised
                    .and_then(|e| e.tracker.as_deref())
                    .unwrap_or_default(),
                raised.and_then(|e| e.instance.as_deref()),
                *at,
            )
        }
        NoticeRef::Day { mark, day } => day_name(rules, mark, *day),
    }
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
        // `Dismiss all` ends a day mark's open days in one press: it names the mark and the count.
        let ends = entries.iter().filter(|e| e.dismissed).count();
        if let (NoticeRef::Day { mark, .. }, true) = (notice, ends > 1) {
            return format!("{} ×{ends} dismissed", rules.day_title(mark));
        }
        let short = notice_name(rules, ledger, notice);
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

/// The title of the Major event that `Add to Timeline` makes: the act's label, with the line it
/// changed, as the action's accessible name says it (`Take seal: Old Guard`).
pub fn timeline_title(rules: &Rules, ledger: &Ledger, entries: &[Entry]) -> String {
    let label = label(rules, ledger, entries);
    match head(entries).filter(|entry| entry.notice.is_none()) {
        Some(Entry {
            instance: Some(line),
            ..
        }) => format!("{label}: {line}"),
        _ => label,
    }
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
    let mut words = act_words(rules, ledger, state, entries);
    // Every way a notice leaves the Due list by a fall says so once: a press, a linked change,
    // or a ticked change of `Done`.
    for entry in entries {
        for at in state.lowered.get(&entry.id).into_iter().flatten() {
            let name = short_name(
                rules,
                entry.tracker.as_deref().unwrap_or_default(),
                entry.instance.as_deref(),
                *at,
            );
            words.push_str(&format!(" {name} is no longer due."));
        }
    }
    words
}

/// The act's own words: the notice it ended or rolled on, or the values it left.
fn act_words(rules: &Rules, ledger: &Ledger, state: &State, entries: &[Entry]) -> String {
    let label = label(rules, ledger, entries);
    // An act on a notice says what happened in a sentence; `Undo: <label>` keeps the label.
    if let Some(entry) = head(entries).filter(|e| e.notice.is_some())
        && let Some(notice) = &entry.notice
    {
        let short = notice_name(rules, ledger, notice);
        let ends = entries.iter().filter(|e| e.dismissed).count();
        if let (NoticeRef::Day { mark, .. }, true) = (notice, entry.dismissed && ends > 1) {
            return format!("{} is dismissed for {ends} days.", rules.day_title(mark));
        }
        if entry.done {
            return format!("{short} is done.");
        }
        if entry.dismissed {
            return format!("{short} is dismissed.");
        }
        if let Some(Roll::Table(roll)) = &entry.roll {
            let name = roll
                .name
                .as_ref()
                .map_or(String::new(), |n| format!(" for {n}"));
            let mut words = format!(
                "Rolled {} on {}{name}: {}",
                roll.value,
                rules.table_name(&roll.table),
                roll.row.trim()
            );
            if !words.ends_with(['.', '!', '?']) {
                words.push('.');
            }
            return words;
        }
    }
    // Each value it left is a sentence of its own; `Undo: <label>` below names the act.
    let values: Vec<String> = entries
        .iter()
        .filter(|entry| entry.notice.is_none())
        .filter_map(|entry| {
            let after = state.after.get(&entry.id)?;
            Some(format!(
                "{} changed to {after}.",
                line_name(rules, entry.tracker.as_deref()?, entry.instance.as_deref())
            ))
        })
        .collect();
    let mut words = if values.is_empty() {
        format!("{label}.")
    } else {
        values.join(" ")
    };
    for entry in entries {
        if let Some(Roll::Dice(text)) = &entry.roll {
            match &entry.row {
                Some(row) => words.push_str(&format!(" {}: {row}", rolled(text))),
                None => words.push_str(&format!(" {}.", rolled(text))),
            }
        }
    }
    let raised: Vec<(String, i64)> = entries
        .iter()
        .flat_map(|entry| {
            state.raised.get(&entry.id).into_iter().flatten().map(|at| {
                let name = short_name(
                    rules,
                    entry.tracker.as_deref().unwrap_or_default(),
                    entry.instance.as_deref(),
                    *at,
                );
                (name, *at)
            })
        })
        .collect();
    // Two notices with one name say their marks: `Bell at 20 and Bell at 40 are due.`
    let due: Vec<String> = raised
        .iter()
        .map(|(name, at)| {
            if raised.iter().filter(|(other, _)| other == name).count() > 1 {
                format!("{name} at {at}")
            } else {
                name.clone()
            }
        })
        .collect();
    match due.as_slice() {
        [] => {}
        [one] => words.push_str(&format!(" {one} is due.")),
        [many @ .., last] => {
            words.push_str(&format!(" {} and {last} are due.", many.join(", ")));
        }
    }
    words
}

/// A tracker's line in a sentence: `Seals for Party`, or the tracker's name alone.
fn line_name(rules: &Rules, key: &str, instance: Option<&str>) -> String {
    let name = rules.def(key).map_or(key, |def| def.name.as_str());
    match instance {
        Some(instance) => format!("{name} for {instance}"),
        None => name.to_owned(),
    }
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
            instance: None,
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
        // A change that names its line with `for:` goes there, else to the line pressed.
        let line = change.instance.as_deref().or(instance);
        let mut entry = change_entry(rules, state, &mut values, target, line, change, asked)?;
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
    /// `Add to Timeline` for this entry, with this title: the shell has the Timeline.
    pub wants_timeline: Option<(String, String)>,
    /// The campaign's day count, as the Timeline held it when its count of changes was
    /// `clock_seen`. The shell reads it again at each act, so an act uses the day at the click.
    today: Today,
    clock_seen: Option<(u64, bool, Option<String>)>,
    /// `Undo add` of this event.
    pub wants_undo_add: Option<String>,
    pub(crate) view: view::ViewState,
    /// Counts each `settle`, so the view measures its columns again only when a value or a
    /// definition changed.
    pub(crate) revision: u64,
    /// `Campaign → Rules and runs` in the empty state: the shell shows that page.
    pub wants_rules: bool,
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
        timeline: &crate::timeline::Book,
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
        self.follow_clock(timeline);
        if let Some(index) = index
            && !self
                .index
                .as_ref()
                .is_some_and(|old| Arc::ptr_eq(old, &index))
        {
            self.manifest = Some(CampaignDocument::load(campaign).map(|document| document.path));
            self.index = Some(index);
            let rules = self.rules_now();
            if !rules.same_definitions(&self.rules) {
                // New definitions: the value column fits them afresh.
                self.view.widest_value = 0.0;
            }
            self.rules = rules;
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

    /// Follow the campaign's day count. The Timeline's count of changes says when it can have
    /// moved, so the day is worked out again only then, never in each frame.
    pub fn follow_clock(&mut self, timeline: &crate::timeline::Book) {
        let seen = (
            timeline.changes(),
            timeline.conflict,
            timeline.moved_under_edit.clone(),
        );
        if self.clock_seen.as_ref() == Some(&seen) {
            return;
        }
        self.clock_seen = Some(seen);
        let today = today(timeline);
        if today != self.today {
            self.today = today;
            self.settle();
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
        self.revision += 1;
        self.state = replay(&self.rules, &self.ledger, &self.today);
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
        let today = self.today.clone();
        if let Err(Some(reason)) =
            self.commit(|rules, ledger| build(rules, &replay(rules, ledger, &today)))
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
            self.view.widest_value = 0.0;
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
            let why = format!("{} done", open.name(rules));
            let mut values = HashMap::new();
            let mut entries = Vec::new();
            for (change, _) in mark.changes.iter().zip(ticked).filter(|(_, on)| **on) {
                let target = change.tracker.as_deref().unwrap_or(&open.key);
                let instance = change.instance.as_deref().or(open.instance.as_deref());
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

    /// `Dismiss all` on a day mark's row: each of its open days ends, as one act, and nothing is
    /// applied. The days must be the ones the row showed.
    pub fn dismiss_all(&mut self, drawn: &Notice) {
        let session = self.played;
        self.act(|_, state| {
            let open = open_as_drawn(state, drawn)?;
            let NoticeRef::Day { mark, .. } = &open.id else {
                return Err("Only a day mark’s row has Dismiss all.".into());
            };
            if open.days != drawn.days {
                return Err("The day count changed. Nothing was recorded.".into());
            }
            let ends = open.days.iter().map(|day| Entry {
                notice: Some(NoticeRef::Day {
                    mark: mark.clone(),
                    day: *day,
                }),
                dismissed: true,
                ..Entry::default()
            });
            Ok(stamp(ends.collect(), session))
        });
    }

    /// `Undo: <label>`: these entries, exactly as the line named them, leave the ledger. When a
    /// later entry about a notice would then count for nothing, it writes nothing and names it.
    pub fn undo(&mut self, expected: &[Entry]) {
        let session = self.played;
        let today = self.today.clone();
        if let Err(Some(reason)) = self.commit(|rules, ledger| {
            let target = undo_target(ledger, session);
            if target != expected {
                return Err("The ledger changed on disk. Nothing was undone.".into());
            }
            let op = Op::Remove(target);
            if let Some(name) = history::orphaned(rules, &today, ledger, &ledger.apply(&op)?) {
                return Err(format!("{name} depends on this act. Nothing was undone."));
            }
            Ok(op)
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

    /// Review of PR 284, finding 3: the day count that day marks use is the clock's, so a fight
    /// placed inside a journey never takes it back, and a fight that ends after the journey
    /// carries it on.
    #[test]
    fn pr284_review_the_day_count_never_goes_back_for_a_fight_inside_a_journey() {
        use crate::timeline::{Kind, RunFact, RunState, Within};
        let session = Some(Uuid::from_u128(1));
        let mut book = crate::timeline::Book::default();
        book.add(Kind::Day, session);
        let road = book.add(Kind::Travel, session);
        book.event_mut(&road).unwrap().duration = "2 days".into();
        assert_eq!(today(&book), Today::Day(3));
        let fact = |state| RunFact {
            id: Uuid::from_u128(7),
            copied_from: None,
            name: "Wolf Pack".into(),
            round: 1,
            state,
            steps: 1,
        };
        let within = Within {
            journey: road,
            at: 24,
        };
        book.record_change(
            Some(&fact(RunState::Setup)),
            &fact(RunState::Running),
            session,
            Some(&within),
        );
        assert_eq!(
            today(&book),
            Today::Day(3),
            "the fight is on Day 2; the count stays"
        );
        let fight = book.events.iter().position(|e| e.run.is_some()).unwrap();
        book.events[fight].duration = "2 days".into();
        assert_eq!(today(&book), Today::Day(4), "the fight ends on Day 4");
    }

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
        /// The campaign's Timeline: the day count's one owner.
        timeline: crate::timeline::Book,
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
        let mut timeline = crate::timeline::Book::default();
        timeline.open(Some(dir.path()));
        let mut trackers = Trackers::default();
        trackers.poll(
            Some(index.clone()),
            Some(&campaign),
            Some(TONIGHT),
            &timeline,
        );
        Race {
            _dir: dir,
            campaign,
            trackers,
            index,
            timeline,
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
        assert_eq!(words, "Elapsed race time changed to 20. Bell is due.");
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
            // The result is a sentence; Undo keeps the label.
            assert_eq!(
                result_words(&t.rules, &t.ledger, &t.state, t.newest()),
                format!(
                    "Rolled {} on Rival Track for {name}: {}",
                    roll.value, roll.row
                )
            );
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
            "Bell is done."
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
            words.starts_with("Crowd Favor changed to 0. Rolled ")
                && words.ends_with(": A healing potion."),
            "{words}"
        );
        // One change that passes two marks names both, in one sentence.
        let jump = Ledger {
            changes: vec![Entry {
                id: "c-0000j040".into(),
                tracker: Some("race_clock".into()),
                add: Some(40),
                why: Some("Enter room".into()),
                ..Entry::default()
            }],
            ..Ledger::default()
        };
        let jumped = replay(&t.rules, &jump, &Today::Day(1));
        assert_eq!(
            result_words(&t.rules, &jump, &jumped, &jump.changes),
            "Elapsed race time changed to 40. Bell at 20 and Bell at 40 are due."
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
        // keeps both presses. The notice the wrong press raised had nothing on it, so the value
        // back below its mark ends it; nothing is written for that.
        t.press("seals", Some("Old Guard"), "Lose seal", None);
        assert_eq!(value(&race, "seals", Some("Old Guard")), 0);
        let t = &mut race.trackers;
        assert_eq!(
            t.due(),
            0,
            "a notice with nothing on it ends when its value falls"
        );
        assert_eq!(
            result_words(&t.rules, &t.ledger, &t.state, t.newest()),
            "Seals for Old Guard changed to 0. First seal for Old Guard is no longer due."
        );
        let lowered = t.newest().to_vec();
        t.undo(&lowered);
        assert_eq!(
            t.state.due,
            [drawn],
            "Undo of the fall brings the notice back"
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
            "Elapsed race time changed to 30. Souls in the jar changed to 1."
        );
        t.undo(&pressed);
        assert_eq!(value(&race, "souls", None), 0);
        assert_eq!(value(&race, "race_clock", None), 20);
        // The line then names the act before it: Undo steps back, newest first.
        assert_eq!(newest_label(&race.trackers), "Take seal");
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

    /// Two day marks for the Tournament: the month's end every 30 days from Day 2, as the design
    /// counts it, and the wagon on one day, with a change for one line of a tracker with `per`.
    /// The wagon comes second in the note and first in the Due list: oldest day first.
    const DAY_MARKS: &str = "> day_marks:\n>   month_end:\n>     name: End of month\n>     every: 30\n>     from: 2\n>     say: Move the rival powers.\n>     changes:\n>     - {tracker: favor, add: 1}\n>   wagon:\n>     name: Supply wagon\n>     at: 3\n>     say: The supply wagon arrives. The party takes a seal.\n>     changes:\n>     - {tracker: seals, for: Party, add: 1}\n";

    fn with_day_marks(race: &mut Race) {
        let note = race.campaign.path.join("Tournament.md");
        let text = fs::read_to_string(&note).unwrap();
        let marked = text.replacen("> trackers:\n", &format!("{DAY_MARKS}> trackers:\n"), 1);
        assert_ne!(marked, text);
        fs::write(&note, marked).unwrap();
        race.index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
        let index = Some(race.index.clone());
        race.trackers
            .poll(index, Some(&race.campaign), Some(TONIGHT), &race.timeline);
    }

    /// An event on the Timeline, saved, and the day count followed as the shell does.
    fn add_event(race: &mut Race, kind: crate::timeline::Kind, length: &str) -> String {
        let id = race.timeline.add(kind, Some(TONIGHT));
        race.timeline.event_mut(&id).unwrap().duration = length.into();
        race.timeline.touch();
        race.timeline.save();
        race.trackers.follow_clock(&race.timeline);
        id
    }

    fn day(mark: &str, day: u64) -> NoticeRef {
        NoticeRef::Day {
            mark: mark.into(),
            day,
        }
    }

    #[test]
    fn day_marks_follow_the_campaigns_day_count() {
        use crate::timeline::{Book, Kind};
        use std::os::unix::fs::PermissionsExt;
        let mut race = race();
        // With no day mark in play, a Timeline that cannot be read says nothing here.
        let plain = replay(
            &race.trackers.rules,
            &race.trackers.ledger,
            &Today::NotRead("Timeline.md could not be read.".into()),
        );
        assert_eq!(plain.clock_not_read, None);
        with_day_marks(&mut race);
        let marks = &race.trackers.rules.day_marks;
        assert_eq!(marks.len(), 2);
        assert!(marks.iter().all(|mark| mark.def.is_ok()), "{marks:?}");
        // Before the first Day the clock has not started: nothing is due.
        assert_eq!(race.trackers.due(), 0);
        add_event(&mut race, Kind::Day, "");
        add_event(&mut race, Kind::Travel, "2 days");
        let t = &race.trackers;
        assert_eq!((t.due(), &t.state.due[0].id), (1, &day("wagon", 3)));
        assert_eq!(
            file(&race),
            "",
            "a day notice is worked out, and raising it writes nothing"
        );

        // Several days in one step: 60 days of travel pass Day 32 and Day 62. One row, oldest
        // first, acts on its first day.
        let travel = add_event(&mut race, Kind::Travel, "60 days");
        let t = &race.trackers;
        let month = t.state.due[1].clone();
        assert_eq!(
            (t.due(), &month.id, month.days.as_slice()),
            (2, &day("month_end", 32), &[32, 62][..])
        );
        // Done on the wagon applies its change to the line it names; that raises Party's seal.
        let wagon = t.state.due[0].clone();
        race.trackers.done(&wagon, &[true]);
        assert_eq!(value(&race, "seals", Some("Party")), 1);
        let t = &race.trackers;
        assert_eq!(newest_label(t), "Supply wagon, Day 3 done");
        // Next ends the oldest day only, with today's values.
        race.trackers.done(&month, &[true]);
        let t = &race.trackers;
        assert_eq!(t.reason, None);
        assert_eq!(newest_label(t), "End of month, Day 32 done");
        assert_eq!(value(&race, "favor", None), 1);
        let rows: Vec<_> = t.state.due.iter().map(|n| n.id.clone()).collect();
        assert_eq!(rows[0], day("month_end", 62), "{rows:?}");

        // Remove in a day mark's history, from the note's reader. The change that a Done applied
        // takes its Done with it, so the day is due again; Undo remove puts both back.
        let note = t.rules.day_marks[0].note;
        let applied = t.history["favor"].last().unwrap().entry.clone();
        assert!(t.history["month_end"].iter().any(|row| row.entry.done
            && row.entry.press.is_some()
            && row.entry.press == applied.press));
        race.trackers.remove(note, &applied);
        let t = &race.trackers;
        assert_eq!(
            t.removed.as_ref().map(|r| r.words.as_str()),
            Some("Removed End of month, Day 32 done: Crowd Favor +1.")
        );
        assert_eq!(
            t.state
                .due
                .iter()
                .find(|n| n.key == "month_end")
                .map(|n| n.days.clone()),
            Some(vec![32, 62])
        );
        assert_eq!(value(&race, "favor", None), 0);
        race.trackers.undo_remove();
        assert_eq!(value(&race, "favor", None), 1);
        assert_eq!(race.trackers.state.due[0].id, day("month_end", 62));
        // A day notice comes from the day count, so no entry raises it: the wagon's Done goes
        // with the seal it applied and the seal's notice, and Day 3 is due again.
        let t = &race.trackers;
        let wagon = t.history["wagon"]
            .iter()
            .find(|row| row.entry.done)
            .unwrap()
            .entry
            .clone();
        race.trackers.remove(note, &wagon);
        let t = &race.trackers;
        assert_eq!(t.history_reason, None);
        assert_eq!(value(&race, "seals", Some("Party")), 0);
        assert!(t.state.due.iter().any(|n| n.id == day("wagon", 3)));
        assert!(!t.state.due.iter().any(|n| n.key == "seals"));
        race.trackers.undo_remove();
        assert_eq!(value(&race, "seals", Some("Party")), 1);
        let t = &race.trackers;
        let rows: Vec<_> = t.state.due.iter().map(|n| n.id.clone()).collect();
        assert_eq!(rows[0], day("month_end", 62), "{rows:?}");

        // The clock goes back by the Timeline's Undo: a day no longer reached goes away, and a
        // Done stays in the ledger. Forward again, a done day stays done.
        race.timeline.undo_add(&travel);
        race.timeline.save();
        race.trackers.follow_clock(&race.timeline);
        let t = &race.trackers;
        assert!(
            t.state
                .due
                .iter()
                .all(|n| matches!(n.id, NoticeRef::Value { .. }))
        );
        assert!(file(&race).contains("day: 32"));
        add_event(&mut race, Kind::Travel, "90 days");
        add_event(&mut race, Kind::Travel, "1 day");
        let month = race.trackers.state.due[0].clone();
        assert_eq!(month.days, [62, 92]);

        // Dismiss all of a row drawn before the clock moved on writes nothing.
        let before = file(&race);
        add_event(&mut race, Kind::Travel, "30 days");
        race.trackers.dismiss_all(&month);
        assert_eq!(
            race.trackers.reason,
            Some(Unrecorded::Refused(
                "The day count changed. Nothing was recorded.".into()
            ))
        );
        assert_eq!(file(&race), before);
        // Dismiss all ends each open day as one act; one Undo takes them back.
        let month = race.trackers.state.due[0].clone();
        assert_eq!(month.days, [62, 92, 122]);
        race.trackers.dismiss_all(&month);
        let t = &race.trackers;
        assert_eq!(newest_label(t), "End of month ×3 dismissed");
        assert!(t.state.due.iter().all(|n| n.key != "month_end"));
        let dismissed = t.newest().to_vec();
        assert_eq!(dismissed.len(), 3);
        // A, then an unrelated B with another stamp, then Undo of A: B stays.
        race.trackers.played = Some(Uuid::from_u128(2));
        race.trackers.press("race_clock", None, "Enter room", None);
        race.trackers.played = Some(TONIGHT);
        race.trackers.settle();
        race.trackers.undo(&dismissed);
        let t = &race.trackers;
        assert_eq!(t.state.due[0].days, [62, 92, 122]);
        assert_eq!(value(&race, "race_clock", None), 10, "B’s change stays");

        // An edit in Obsidian a moment before ends Day 62: the click on the row drawn before
        // writes nothing and says so, and the row draws from the file.
        let month = race.trackers.state.due[0].clone();
        let path = ledger::path(&race.campaign.path);
        let outside = format!(
            "{}- id: c-0b51de62\n  notice:\n    mark: month_end\n    day: 62\n  done: true\n- id: c-01dca1e5\n  notice: {{mark: old_calendar, day: 5}}\n  dismissed: true\n---\n",
            file(&race).trim_end().trim_end_matches("---")
        );
        fs::write(&path, &outside).unwrap();
        race.trackers.done(&month, &[true]);
        assert_eq!(
            race.trackers.reason,
            Some(Unrecorded::Refused(
                "That notice is no longer due. Nothing was recorded.".into()
            ))
        );
        assert_eq!(file(&race), outside);
        assert_eq!(race.trackers.state.due[0].days, [92, 122]);
        // An entry for a day mark that no note in play has is kept and named, as a tracker's.
        assert_eq!(race.trackers.state.unknown, [("old_calendar".into(), 1)]);

        // A failed save records nothing and keeps nothing.
        let month = race.trackers.state.due[0].clone();
        let mut mode = fs::metadata(&race.campaign.path).unwrap().permissions();
        mode.set_mode(0o555);
        fs::set_permissions(&race.campaign.path, mode.clone()).unwrap();
        race.trackers.done(&month, &[true]);
        mode.set_mode(0o755);
        fs::set_permissions(&race.campaign.path, mode).unwrap();
        assert!(matches!(
            race.trackers.reason,
            Some(Unrecorded::NotSaved(_))
        ));
        assert_eq!(file(&race), outside);
        assert_eq!(race.trackers.state.due[0].days, [92, 122]);

        // A Timeline that cannot be read: no day notice shows, never an old one, the Due list
        // says why, and an act on a day row writes nothing. A notice at a value still shows.
        let timeline = race.campaign.path.join(crate::timeline::FILE);
        let readable = fs::read(&timeline).unwrap();
        fs::set_permissions(&timeline, fs::Permissions::from_mode(0o000)).unwrap();
        race.timeline.reload();
        race.trackers.follow_clock(&race.timeline);
        let t = &race.trackers;
        let why = t.state.clock_not_read.clone().unwrap_or_default();
        assert!(why.starts_with("Timeline.md could not be read."), "{why}");
        assert_eq!(t.due(), 1, "Party's First seal");
        race.trackers.dismiss(&month);
        assert_eq!(
            race.trackers.reason,
            Some(Unrecorded::Refused(format!("{why} Nothing was recorded.")))
        );
        assert_eq!(file(&race), outside);
        fs::set_permissions(&timeline, fs::Permissions::from_mode(0o644)).unwrap();

        // A day passes while Splat is closed: another program adds 30 days to the Timeline.
        // After a restart the row and the last act's Undo come from the files.
        let mut other = Book::default();
        other.open(Some(&race.campaign.path));
        let id = other.add(Kind::Travel, None);
        other.event_mut(&id).unwrap().duration = "30 days".into();
        other.touch();
        other.save();
        assert_ne!(fs::read(&timeline).unwrap(), readable);
        let mut book = Book::default();
        book.open(Some(&race.campaign.path));
        let mut again = Trackers::default();
        again.poll(
            Some(race.index.clone()),
            Some(&race.campaign),
            Some(TONIGHT),
            &book,
        );
        assert_eq!(again.state.due[0].days, [92, 122, 152]);
        assert_eq!(newest_label(&again), "End of month, Day 32 done");

        // The act reads the day at the click. Obsidian takes the Timeline back to Day 3 while
        // the shell's Timeline still holds Day 153; the press in the view writes nothing.
        let mut app = crate::Splat {
            trackers: again,
            ..Default::default()
        };
        app.timeline.open(Some(&race.campaign.path));
        app.trackers.follow_clock(&app.timeline);
        let month = app.trackers.state.due[0].clone();
        assert_eq!(month.days, [92, 122, 152]);
        // Next pressed in the view ends the oldest day, and the row moves on to the next.
        app.trackers.press_done_in_view(&month, vec![true]);
        app.apply_tracker_act();
        assert_eq!(app.trackers.reason, None);
        assert_eq!(app.trackers.state.due[0].days, [122, 152]);
        let month = app.trackers.state.due[0].clone();
        let before = file(&race);
        // A Timeline edit waits for its pause, and under it the file changes on disk, or
        // cannot be read: the day count is not known. One answer serves the act and the view:
        // the click records nothing, no day row shows, the Due list says why, and the draft is
        // kept. When the file reads as it was, the rows come back.
        let saved = fs::read(&timeline).unwrap();
        let edited = app.timeline.events[0].id.clone();
        app.timeline.event_mut(&edited).unwrap().title = "Dawn".into();
        app.timeline.touch();
        let mut outside = String::from_utf8(saved.clone()).unwrap();
        outside.push_str("\nWritten in Obsidian.\n");
        let day_rows = |t: &Trackers| {
            t.state
                .due
                .iter()
                .filter(|n| matches!(n.id, NoticeRef::Day { .. }))
                .count()
        };
        let value_rows = app.trackers.due() - day_rows(&app.trackers);
        for (text, mode, why) in [
            (
                outside.as_bytes(),
                0o644,
                crate::timeline::CHANGED_UNDER_DRAFT,
            ),
            (&saved[..], 0o000, "Timeline.md could not be read."),
        ] {
            fs::write(&timeline, text).unwrap();
            fs::set_permissions(&timeline, fs::Permissions::from_mode(mode)).unwrap();
            app.trackers.press_done_in_view(&month, vec![true]);
            app.apply_tracker_act();
            fs::set_permissions(&timeline, fs::Permissions::from_mode(0o644)).unwrap();
            let t = &app.trackers;
            let shown = t.state.clock_not_read.clone().unwrap_or_default();
            assert!(shown.starts_with(why), "{shown}");
            assert_eq!(
                t.reason,
                Some(Unrecorded::Refused(format!(
                    "{shown} Nothing was recorded."
                )))
            );
            assert_eq!((day_rows(t), t.due()), (0, value_rows), "{why}");
            assert_eq!(file(&race), before);
            assert!(app.timeline.unsaved() && app.timeline.events[0].title == "Dawn");
            // The file reads as it was: the rows come back, with the draft still waiting.
            fs::write(&timeline, &saved).unwrap();
            app.timeline.read_if_changed();
            app.trackers.follow_clock(&app.timeline);
            let t = &app.trackers;
            assert_eq!(t.state.clock_not_read, None);
            assert_eq!(t.state.due[0].days, [122, 152]);
        }
        app.timeline.save();
        app.timeline.read_if_changed();
        let mut back = Book::default();
        back.open(Some(&race.campaign.path));
        let ids: Vec<String> = back
            .events
            .iter()
            .filter(|event| event.kind == Kind::Travel)
            .skip(1)
            .map(|event| event.id.clone())
            .collect();
        for id in &ids {
            back.remove(id);
        }
        back.save();
        app.trackers.press_done_in_view(&month, vec![true]);
        app.apply_tracker_act();
        assert_eq!(
            app.trackers.reason,
            Some(Unrecorded::Refused(
                "That notice is no longer due. Nothing was recorded.".into()
            ))
        );
        assert_eq!(file(&race), before);
        assert!(app.trackers.state.due.iter().all(|n| n.key != "month_end"));
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
                        "name: Teams\nper: [Party]\nquick:\n- label: Score\n  changes: [{tracker: rivals, add: 1}]\n- label: Score for Old Guard\n  changes: [{tracker: rivals, for: Old Guard, add: 1}]",
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
        let state = replay(&rules, &Ledger::default(), &Today::default());
        assert_eq!(
            press_op(&rules, &state, "teams", Some("Party"), "Score", None, None),
            Err("Rivals has no line for Party. Nothing was recorded.".into())
        );
        // A change that names its line with `for:` goes there, from whichever line is pressed.
        let pressed = "Score for Old Guard";
        let Ok(Op::Append(entries)) =
            press_op(&rules, &state, "teams", Some("Party"), pressed, None, None)
        else {
            panic!("the press records");
        };
        assert_eq!(entries[0].instance.as_deref(), Some("Old Guard"));
        // A day mark has no line of its own, so its change names its tracker, and `for:` names
        // a line that tracker has.
        let day_mark = |yaml: &str| {
            let mark = serde_yaml_ng::from_str::<DayMark>(yaml).unwrap();
            check_day_mark(&mark, &rules)
        };
        assert_eq!(
            day_mark(
                "{every: 7, say: Rest., changes: [{tracker: rivals, for: Old Guard, add: 1}]}"
            ),
            Ok(())
        );
        for (yaml, why) in [
            (
                "{at: 3, every: 7, say: Rest.}",
                "A day mark has at or every.",
            ),
            ("{say: Rest.}", "A day mark has at or every."),
            (
                "{at: 3, from: 2, say: Rest.}",
                "from works only with every.",
            ),
            ("{every: 0, say: Rest.}", "A day mark’s day is 1 or more."),
            (
                "{at: 3, say: Rest., changes: [{add: 1}]}",
                "A day mark’s change names its tracker.",
            ),
            (
                "{at: 3, say: Rest., changes: [{tracker: rivals, add: 1}]}",
                "rivals has per, so a change to it needs a name.",
            ),
            (
                "{at: 3, say: Rest., changes: [{tracker: rivals, for: Party, add: 1}]}",
                "rivals has no line for Party.",
            ),
        ] {
            assert_eq!(day_mark(yaml), Err(why.into()), "{yaml}");
        }
        // `every: 30, from: 2` is Day 32, Day 62; with no `from` a schedule counts from Day 1.
        let month = serde_yaml_ng::from_str::<DayMark>("{every: 30, from: 2, say: x}").unwrap();
        assert_eq!(month.days_until(91), [32, 62]);
        let plain = serde_yaml_ng::from_str::<DayMark>("{every: 30, say: x}").unwrap();
        assert_eq!(plain.days_until(31), [31]);
        // `Done` whose ticked change takes a value back below a mark says that notice is no
        // longer due, as a press does.
        let rules = Rules {
            notes: vec!["Rules".into()],
            trackers: vec![Tracker {
                key: "rivals".into(),
                note: Uuid::nil(),
                def: Ok(def(
                    "name: Rivals\nper: [Old Guard]\nmarks: [{at: 1, say: First.}]",
                )),
            }],
            day_marks: vec![Dated {
                key: "rest".into(),
                def: Ok(serde_yaml_ng::from_str(
                    "{at: 7, say: Rest., changes: [{tracker: rivals, for: Old Guard, add: -1}]}",
                )
                .unwrap()),
                note: Uuid::nil(),
            }],
            ..Rules::default()
        };
        let change = |id: &str, add: i64| Entry {
            id: id.into(),
            tracker: Some("rivals".into()),
            instance: Some("Old Guard".into()),
            add: Some(add),
            ..Entry::default()
        };
        let done = vec![
            change("c-00000002", -1),
            Entry {
                id: "c-00000003".into(),
                notice: Some(NoticeRef::Day {
                    mark: "rest".into(),
                    day: 7,
                }),
                done: true,
                ..Entry::default()
            },
        ];
        let ledger = Ledger {
            changes: [vec![change("c-00000001", 1)], done.clone()].concat(),
            ..Ledger::default()
        };
        let state = replay(&rules, &ledger, &Today::Day(7));
        assert!(state.due.is_empty(), "{:?}", state.due);
        assert_eq!(
            result_words(&rules, &ledger, &state, &done),
            "Rest, Day 7 is done. First for Old Guard is no longer due."
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
        // A notice with a roll on it stays when the value falls below its mark: the DM acted
        // on it. Here an edit in Obsidian takes the clock back to 0, then away again.
        let path = ledger::path(&race.campaign.path);
        let back = Entry {
            id: "c-0b5e1e01".into(),
            tracker: Some("race_clock".into()),
            add: Some(-20),
            ..Entry::default()
        };
        ledger::commit(&path, |_| Ok(Op::Append(vec![back.clone()]))).unwrap();
        let t = &mut race.trackers;
        t.read_disk();
        assert_eq!(
            (
                value(&race, "race_clock", None),
                race.trackers.state.due.len()
            ),
            (0, 1),
            "the rolled bell stays"
        );
        ledger::commit(&path, |_| Ok(Op::Remove(vec![back]))).unwrap();
        let t = &mut race.trackers;
        t.read_disk();
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
        assert!(
            rows[2].starts_with("Rolled ") && rows[2].contains(" on Wandering, 0:00 to 0:39: "),
            "{}",
            rows[2]
        );
        assert_eq!(rows[3], "Bell is done.");
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
            Some(
                "Removed Enter room: Elapsed race time +10. With it: Wandering, 0:00 to 0:39 roll; Bell done."
            )
        );
        assert_eq!(t.ledger.changes, kept.changes[..1]);
        assert_eq!(
            (value(&race, "race_clock", None), race.trackers.due()),
            (10, 0)
        );
        // A notice raised since needs the removal: its bell crosses 20 only while the room is
        // out. Undo remove would leave its roll with no notice, so it writes nothing.
        let t = &mut race.trackers;
        t.press("race_clock", None, "Cistern or Sands", None);
        let bell = t.state.due[0].clone();
        t.roll_notice(&bell, "wander_early");
        let before = file(&race);
        let t = &mut race.trackers;
        t.undo_remove();
        assert_eq!(
            t.history_reason,
            Some((
                note,
                Unrecorded::Refused(
                    "Wandering, 0:00 to 0:39 roll depends on the removal. Remove that first. Nothing was put back."
                        .into()
                )
            ))
        );
        assert!(
            t.removed.is_some(),
            "the Undo stays for when that roll is gone"
        );
        assert_eq!(file(&race), before);

        // A notice with a roll on it stays, whatever puts a fall before the roll: a Remove, an
        // Undo, an `Undo remove` or an edit in Obsidian. Rooms of an older session: A to 20
        // raises the bell, B to 30, C back to 20, then a roll on A's bell.
        let path = ledger::path(&race.campaign.path);
        let older = Some(Uuid::from_u128(1));
        let room = |id: &str, add: i64, session| Entry {
            id: id.into(),
            tracker: Some("race_clock".into()),
            add: Some(add),
            session,
            ..Entry::default()
        };
        let rolled = |id: &str, change: &str, at: i64, table: &str| Entry {
            id: id.into(),
            notice: Some(NoticeRef::Value {
                change: change.into(),
                at,
            }),
            roll: Some(Roll::Table(TableRoll {
                table: table.into(),
                name: None,
                value: 3,
                row: "Nothing comes.".into(),
            })),
            session: older,
            ..Entry::default()
        };
        let write = |entries: Vec<Entry>| {
            fs::remove_file(&path).ok();
            ledger::commit(&path, |_| Ok(Op::Append(entries))).unwrap();
            fs::read_to_string(&path).unwrap()
        };
        let bell_with_roll = |t: &Trackers| {
            t.state
                .due
                .iter()
                .any(|n| n.id.change() == Some("c-0000000a") && n.rolls.len() == 1)
        };
        let rooms = |b_session| {
            vec![
                room("c-0000000a", 20, older),
                room("c-0000000b", 10, b_session),
                room("c-0000000c", -10, older),
                rolled("c-0000000d", "c-0000000a", 20, "wander_early"),
            ]
        };
        write(rooms(older));
        let t = &mut race.trackers;
        t.read_disk();
        let b = t.history["race_clock"]
            .iter()
            .find(|row| row.entry.id == "c-0000000b")
            .unwrap()
            .entry
            .clone();
        t.remove(note, &b);
        assert_eq!(t.history_reason, None);
        assert_eq!(value(&race, "race_clock", None), 10);
        assert!(
            bell_with_roll(&race.trackers),
            "Remove of B keeps the bell and its roll"
        );
        write(rooms(Some(TONIGHT)));
        let t = &mut race.trackers;
        t.read_disk();
        let newest = t.newest().to_vec();
        assert_eq!(newest.len(), 1, "B is tonight's newest act");
        t.undo(&newest);
        assert_eq!(t.reason, None);
        assert!(
            bell_with_roll(&race.trackers),
            "Undo of B keeps the bell and its roll"
        );
        // A correction that took the clock back to 0 is removed, the bell is rolled, and then
        // `Undo remove` puts the correction back before the roll: the bell stays, with its roll,
        // also as the saved file rebuilds it.
        write(vec![
            room("c-0000000a", 20, older),
            room("c-0000000e", -20, older),
        ]);
        let t = &mut race.trackers;
        t.read_disk();
        let correction = t.history["race_clock"][1].entry.clone();
        t.remove(note, &correction);
        let bell = t.state.due[0].clone();
        t.roll_notice(&bell, "wander_early");
        t.undo_remove();
        assert_eq!(t.history_reason, None);
        assert_eq!(value(&race, "race_clock", None), 0);
        assert!(
            bell_with_roll(&race.trackers),
            "Undo remove keeps the bell and its roll"
        );
        let t = &race.trackers;
        let saved = ledger::parse_bytes(Some(&fs::read(&path).unwrap())).unwrap();
        assert_eq!(replay(&t.rules, &saved, &t.today).due, t.state.due);
        // An Undo that would leave a roll with no notice writes nothing and names it: D crosses
        // 40 only because of tonight's B.
        let before = write(vec![
            room("c-0000000a", 20, older),
            room("c-0000000b", 10, Some(TONIGHT)),
            room("c-0000000f", 10, older),
            rolled("c-00000010", "c-0000000f", 40, "wander_middle"),
        ]);
        let t = &mut race.trackers;
        t.read_disk();
        let newest = t.newest().to_vec();
        t.undo(&newest);
        assert_eq!(
            t.reason,
            Some(Unrecorded::Refused(
                "Wandering, 0:40 to 1:19 roll depends on this act. Nothing was undone.".into()
            ))
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), before);

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
            Some("Removed Typed in Obsidian: Crowd Favor +2.")
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
        // An Undo whose place changed on disk: the entry before the removed one was edited in
        // Obsidian. It puts nothing back, says so, and ends.
        let t = &mut race.trackers;
        let clock = t.history["race_clock"][0].entry.clone();
        t.remove(note, &clock);
        assert!(t.removed.is_some());
        let edited = fs::read_to_string(&path)
            .unwrap()
            .replace("row: A healing potion.", "row: A healing potion, edited.");
        fs::write(&path, &edited).unwrap();
        t.read_disk();
        t.undo_remove();
        assert_eq!(
            t.history_reason,
            Some((
                note,
                Unrecorded::Refused(
                    "The entry before it changed on disk. Nothing was put back.".into()
                )
            ))
        );
        assert_eq!(
            (t.removed.is_none(), fs::read_to_string(&path).unwrap()),
            (true, edited)
        );
        fs::write(&path, &text).unwrap();
        race.trackers.read_disk();

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
        t.poll(
            Some(index),
            Some(&race.campaign),
            Some(TONIGHT),
            &race.timeline,
        );
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
            &race.timeline,
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
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn an_open_history_prepares_its_rows_once_and_draws_only_those_in_view() {
        use crate::test_support::{CachedWork, Counts};
        use egui_kittest::{Harness, kittest::Queryable};
        let mut race = race();
        let note = race
            .index
            .items
            .iter()
            .find(|item| item.entry.name == "Tournament")
            .unwrap()
            .entry
            .clone();
        let mut t = std::mem::take(&mut race.trackers);
        t.ledger.changes = (0..5000)
            .map(|n| Entry {
                id: format!("c-{n:08x}"),
                tracker: Some("favor".into()),
                add: Some(1),
                why: Some("Crowd loves it".into()),
                ..Entry::default()
            })
            .collect();
        t.settle();
        let mut h = Harness::builder()
            .with_size(eframe::egui::vec2(346.0, 640.0))
            .build_ui_state(
                move |ui, t: &mut Trackers| {
                    eframe::egui::ScrollArea::vertical().show(ui, |ui| history::show(ui, t, &note));
                },
                t,
            );
        crate::design::theme::install(&h.ctx);
        let counts = |_: &mut Harness<'_, Trackers>| {
            let (reached, _) = history::DRAWN.with(std::cell::Cell::get);
            Counts {
                reached,
                work: history::PREPARED.with(std::cell::Cell::get),
            }
        };
        let drawn = || history::DRAWN.with(std::cell::Cell::get).1;
        // Folded, the section draws no row.
        h.run_steps(3);
        let folded = drawn();
        h.run_steps(3);
        assert_eq!(drawn(), folded, "a folded history draws no row");
        let work = CachedWork::start("Open history rows", &mut h, counts);
        h.get_by_label("CROWD FAVOR HISTORY 5,000").click();
        h.run_steps(3);
        let shown = h.query_all_by_label("Crowd loves it: +1").next().is_some();
        work.warm(&mut h, shown, 0)
            .expect("the open history shows its rows");
        // Unchanged frames and a resize prepare no row again, and each frame draws only the
        // rows in view: at 640 px, some 20 rows of 5,000.
        let before = drawn();
        work.steady(&mut h, |h| {
            h.run_steps(3);
            h.set_size(eframe::egui::vec2(400.0, 640.0));
            h.run_steps(2);
            h.query_all_by_label("Crowd loves it: +1").next().is_some()
        })
        .expect("unchanged frames prepare no row");
        let per_frame = (drawn() - before) / 5;
        assert!(
            (1..=30).contains(&per_frame),
            "an open history of 5,000 rows drew {per_frame} rows a frame"
        );
        // A change of the ledger prepares the rows again, once.
        work.refresh(&mut h, 5001, |h| {
            let t = h.state_mut();
            t.ledger.changes.push(Entry {
                id: "c-ffffffff".into(),
                tracker: Some("favor".into()),
                add: Some(1),
                why: Some("Crowd loves it".into()),
                ..Entry::default()
            });
            t.settle();
            h.run_steps(2);
            h.query_by_label("CROWD FAVOR HISTORY 5,001").is_some()
        })
        .expect("a changed ledger prepares its rows again");
    }

    /// FE1, AE2: every tracker line of the panel shares one value column and one first-action
    /// edge, and a value that gains a digit moves no action. The columns are measured once for
    /// each change of the trackers, never on an unchanged frame.
    #[test]
    fn the_panel_shares_its_columns_and_measures_them_once_per_change() {
        use egui_kittest::{Harness, kittest::Queryable};
        let mut race = race();
        let t = std::mem::take(&mut race.trackers);
        let mut h = Harness::builder()
            // Wide enough that every first action fits after the values: the layout in which a
            // wider value could push the actions.
            .with_size(eframe::egui::vec2(480.0, 640.0))
            .build_ui_state(|ui, t: &mut Trackers| view::show(ui, t, false), t);
        crate::design::theme::install(&h.ctx);
        h.run_steps(4);
        let left = |h: &Harness<'_, Trackers>, name: &str| h.get_by_label(name).rect().left();
        let firsts = |h: &Harness<'_, Trackers>| {
            [
                "Enter room",
                "Crowd loves it",
                "Take seal: Party",
                "Take seal: Keyring",
            ]
            .map(|name| left(h, name))
        };
        let before = firsts(&h);
        assert!(
            before.iter().all(|x| (x - before[0]).abs() < 0.5),
            "the first actions of three trackers start at one edge: {before:?}"
        );
        let measured = h.state().view.measured;
        h.run_steps(5);
        assert_eq!(
            h.state().view.measured,
            measured,
            "an unchanged frame measures no column"
        );
        // 0 / 120 becomes 10 / 120, and Crowd Favor 0 becomes 1: measured again, no move.
        h.state_mut().press("race_clock", None, "Enter room", None);
        h.state_mut().press("favor", None, "Crowd loves it", None);
        h.run_steps(4);
        let t = h.state();
        assert_eq!(t.state.value(&t.rules, "race_clock", None), 10);
        let changed = h.state().view.measured;
        assert!(
            changed > measured,
            "a change of the values measures the columns again"
        );
        h.run_steps(5);
        assert_eq!(
            h.state().view.measured,
            changed,
            "unchanged frames after the change measure no column"
        );
        assert_eq!(
            firsts(&h),
            before,
            "a value with one more digit moves no action"
        );
        // 999 / 120, then `Fight minute` makes 1000 / 120, wider than the slot of three
        // digits; Undo takes it back to 999. The value column keeps the room it needed, so the
        // actions do not move back and forth.
        let typed = Entry {
            id: "c-00000999".into(),
            tracker: Some("race_clock".into()),
            set: Some(999),
            why: Some("Typed in Obsidian".into()),
            ..Entry::default()
        };
        ledger::commit(&ledger::path(&race.campaign.path), |_| {
            Ok(Op::Append(vec![typed]))
        })
        .unwrap();
        h.state_mut().read_disk();
        h.run_steps(4);
        h.state_mut()
            .press("race_clock", None, "Fight minute", None);
        h.run_steps(4);
        let t = h.state();
        assert_eq!(t.state.value(&t.rules, "race_clock", None), 1000);
        let wide = firsts(&h);
        let pressed = h.state().newest().to_vec();
        h.state_mut().undo(&pressed);
        h.run_steps(4);
        let t = h.state();
        assert_eq!(t.state.value(&t.rules, "race_clock", None), 999);
        assert_eq!(
            firsts(&h),
            wide,
            "Undo of 1000 / 120 keeps the value column, and no action moves back"
        );
    }

    #[test]
    fn no_note_in_play_reads_no_ledger_and_shows_nothing() {
        let mut race = race();
        // A Remove's result and its Undo are in the note's reader while the note is in play.
        let t = &mut race.trackers;
        let note = t.rules.trackers[0].note;
        t.played = None;
        t.press("favor", None, "Crowd loves it", None);
        t.played = Some(TONIGHT);
        t.settle();
        let older = t.history["favor"][0].entry.clone();
        t.remove(note, &older);
        assert_eq!(history::footer_lines(t, note).len(), 1);
        // The note is renamed outside Splat before the next click. The note is still in play, so
        // its reader keeps the click's refusal and says what became of the note.
        t.played = None;
        t.press("favor", None, "Crowd loves it", None);
        t.played = Some(TONIGHT);
        t.settle();
        let other = t.history["favor"][0].entry.clone();
        let (file_now, renamed) = (
            race.campaign.path.join("Tournament.md"),
            race.campaign.path.join("Renamed.md"),
        );
        let ledger_before = fs::read(ledger::path(&race.campaign.path)).unwrap();
        fs::rename(&file_now, &renamed).unwrap();
        let t = &mut race.trackers;
        t.remove(note, &other);
        assert_eq!(
            fs::read(ledger::path(&race.campaign.path)).unwrap(),
            ledger_before
        );
        let words: Vec<String> = history::footer_lines(t, note)
            .into_iter()
            .map(|line| line.words)
            .collect();
        assert!(
            words.contains(&"A house rule note changed on disk. Nothing was recorded.".to_owned()),
            "{words:?}"
        );
        assert!(t.problems.contains(&(
            Some(note),
            "Tournament is not where it was. Splat reads the notes again when its window gets focus."
                .into()
        )));
        fs::rename(&renamed, &file_now).unwrap();
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
        let index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
        race.trackers.poll(
            Some(index.clone()),
            Some(&race.campaign),
            Some(TONIGHT),
            &race.timeline,
        );
        assert!(
            history::footer_lines(&race.trackers, note).is_empty(),
            "a note out of play shows no Undo remove"
        );
        fs::write(ledger::path(&race.campaign.path), "not a ledger").unwrap();
        let mut trackers = Trackers::default();
        trackers.poll(Some(index), Some(&race.campaign), None, &race.timeline);
        assert!(!trackers.in_play());
        assert_eq!(trackers.due(), 0);
        assert!(trackers.unreadable.is_none(), "the ledger is not read");
    }
}
