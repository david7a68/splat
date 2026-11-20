//! House rules in play, and the acts that read or change their tracker ledger.
pub mod ledger;
mod record;
pub use record::{Act, QuickRef};

use crate::loot::Sources;
use ledger::{Entry, Ledger, NoticeRef, Op, Roll, TableRoll};
use serde::Deserialize;
use serde_yaml_ng::Value;
use splat_content::dice::Dice;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

/// A `rule` entry with this implementation is a house rule note.
pub const IMPLEMENTATION: &str = "splat:house_rules";

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
    /// Two amount fields that would have one owner (tracker, line, label), found when the note
    /// is read (`repeat`), never in a frame.
    #[serde(skip)]
    pub repeat: Option<Repeat>,
}

/// Two amount fields with one owner: the view leaves out every one after the first and says why
/// under that line. Two actions with one label and fixed amounts have no field, and load as
/// they are.
#[derive(Clone, Debug, PartialEq)]
pub enum Repeat {
    /// Two lines with this name, and the places in `quick` of the actions that ask: the second
    /// line leaves those out.
    Line { word: String, asking: Vec<usize> },
    /// Two or more actions with this label, and one of them asks: the view matches the open
    /// field by label, so each of them would draw it. Only the first, at `keep`, is drawn.
    Label { label: String, keep: usize },
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
    pub fn same_definitions(&self, other: &Rules) -> bool {
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
pub fn is_house_rule_note(entry: &crate::content::Entry) -> bool {
    entry.kind == "rule"
        && entry
            .fields
            .get("splat_rule")
            .and_then(|rule| rule.get("implementation"))
            .and_then(Value::as_str)
            == Some(IMPLEMENTATION)
}

impl Rules {
    pub fn cannot_read(error: String) -> Self {
        Self {
            unreadable: Some(error),
            ..Self::default()
        }
    }
}

/// Read the notes in play, or why each cannot be used. Each member is checked alone, so a bad
/// one is reported and the others still work.
pub fn read_rules(
    tables: Arc<Sources>,
    notes: Vec<Result<crate::content::Entry, (Uuid, String)>>,
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
                (Ok(def), None) => validate_definition(def, &rules).map(|()| Definition {
                    repeat: repeat(def),
                    ..def.clone()
                }),
            },
        )
        .collect();
    let dated: Vec<_> = rules
        .day_marks
        .iter()
        .map(|mark| match (&mark.def, shared(&mark.key).as_ref()) {
            (Err(error), _) | (Ok(_), Some(error)) => Err(error.clone()),
            (Ok(def), None) => validate_day_mark(def, &rules).map(|()| def.clone()),
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
pub fn validate_definition(def: &Definition, rules: &Rules) -> Result<(), String> {
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

/// Two amount fields that one owner would draw: a label of two or more actions where one of
/// them asks, else (when an action asks) a line name that comes twice. Read once, when the note
/// is read.
pub fn repeat(def: &Definition) -> Option<Repeat> {
    let asking: Vec<usize> = (0..def.quick.len())
        .filter(|&at| def.quick[at].asks())
        .collect();
    let label = asking.iter().find_map(|&at| {
        let label = &def.quick[at].label;
        let mut twins = (0..def.quick.len()).filter(|&i| def.quick[i].label == *label);
        let keep = twins.next()?;
        twins.next().map(|_| Repeat::Label {
            label: label.clone(),
            keep,
        })
    });
    let mut seen = std::collections::HashSet::new();
    label.or_else(|| {
        (!asking.is_empty())
            .then(|| def.per.iter().find(|word| !seen.insert(*word)))
            .flatten()
            .map(|word| Repeat::Line {
                word: word.clone(),
                asking,
            })
    })
}

/// A day mark has `at` or `every`, a day of 1 or more, `from` only with `every`, and words that
/// resolve.
pub fn validate_day_mark(mark: &DayMark, rules: &Rules) -> Result<(), String> {
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
    /// The value just after this ledger entry.
    pub fn value_after(&self, entry: &str) -> Option<i64> {
        self.after.get(entry).copied()
    }

    /// The marks this change took below their limits.
    pub fn lowered(&self, entry: &str) -> &[i64] {
        self.lowered.get(entry).map_or(&[], Vec::as_slice)
    }

    /// The marks this ledger entry crossed.
    pub fn raised_by(&self, entry: &str) -> &[i64] {
        self.raised.get(entry).map_or(&[], Vec::as_slice)
    }
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
pub fn place_name(rules: &Rules, key: &str, instance: Option<&str>) -> String {
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
pub fn with_dependents(ledger: &Ledger, seed: &Entry) -> Vec<Entry> {
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
pub fn notice_name(rules: &Rules, ledger: &Ledger, notice: &NoticeRef) -> String {
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
pub fn head(entries: &[Entry]) -> Option<&Entry> {
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

/// The act whose entries about a notice would be left with nothing to name, when the ledger
/// becomes `next`: a notice that a change raised in `ledger` and raises no more in `next`.
pub fn orphaned(rules: &Rules, today: &Today, ledger: &Ledger, next: &Ledger) -> Option<String> {
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

/// Build the Undo shown for this session against the ledger as it stands now.
pub fn undo_op(
    rules: &Rules,
    today: &Today,
    ledger: &Ledger,
    session: Option<Uuid>,
    expected: &[Entry],
) -> Result<Op, String> {
    let target = undo_target(ledger, session);
    if target != expected {
        return Err("The ledger changed on disk. Nothing was undone.".into());
    }
    let op = Op::Remove(target);
    if let Some(name) = orphaned(rules, today, ledger, &ledger.apply(&op)?) {
        return Err(format!("{name} depends on this act. Nothing was undone."));
    }
    Ok(op)
}
