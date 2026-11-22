//! Check a tracker act against the current ledger before it writes.
use super::*;
use splat_content::dice::Random;

/// A quick action as a press names it: its label, and its place in `quick` when the press came
/// from a drawn button, so of two actions with one label each press makes its own changes.
#[derive(Clone, Copy, Debug)]
pub struct QuickRef<'a> {
    pub label: &'a str,
    pub at: Option<usize>,
}

impl<'a> From<&'a str> for QuickRef<'a> {
    fn from(label: &'a str) -> Self {
        Self { label, at: None }
    }
}

/// A tracker act, with its inputs and the identity of the row that offered it.
pub enum Act<'a> {
    Press {
        key: &'a str,
        instance: Option<&'a str>,
        action: QuickRef<'a>,
        asked: Option<i64>,
    },
    Roll {
        drawn: &'a Notice,
        table: &'a str,
    },
    Done {
        drawn: &'a Notice,
        ticked: &'a [bool],
    },
    Dismiss {
        drawn: &'a Notice,
    },
    DismissAll {
        drawn: &'a Notice,
    },
}

#[derive(Clone, Copy)]
struct Record<'a> {
    rules: &'a Rules,
    state: &'a State,
    session: Option<Uuid>,
    new_id: fn() -> Uuid,
}

impl Act<'_> {
    /// Build the operation from the rules and ledger as they stand now.
    pub fn build(
        self,
        rules: &Rules,
        ledger: &Ledger,
        today: &Today,
        session: Option<Uuid>,
        new_id: fn() -> Uuid,
    ) -> Result<Op, Refused> {
        let state = &replay(rules, ledger, today);
        let record = Record {
            rules,
            state,
            session,
            new_id,
        };
        match self {
            Act::Press {
                key,
                instance,
                action,
                asked,
            } => press_op(record, key, instance, action, asked),
            Act::Roll { drawn, table } => {
                let notice = &drawn.id;
                let open = open_as_drawn(state, drawn)?;
                let mark = mark_of(rules, open).ok_or(Refused::Changed(Part::Mark))?;
                let names = mark
                    .tables
                    .iter()
                    .find(|t| t.key() == table)
                    .ok_or(Refused::Changed(Part::Table))?
                    .names();
                let rolled = open.rolls.iter().filter(|roll| roll.table == table).count();
                let name = if names.is_empty() {
                    None
                } else {
                    Some(names.get(rolled).ok_or(Refused::EveryNameRolled)?.clone())
                };
                let (_, value, row) = roll_table(rules, table, new_id)?;
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
                    new_id,
                ))
            }
            Act::Done { drawn, ticked } => {
                let notice = &drawn.id;
                let open = open_as_drawn(state, drawn)?;
                let mark = mark_of(rules, open).ok_or(Refused::Changed(Part::Mark))?;
                if mark.changes.len() != ticked.len() {
                    return Err(Refused::Changed(Part::Mark));
                }
                // The label of the notice's end, as `label` names it.
                let why = format!("{} done", open.name(rules));
                let mut values = HashMap::new();
                let mut entries = Vec::new();
                for (change, _) in mark.changes.iter().zip(ticked).filter(|(_, on)| **on) {
                    let target = change.tracker.as_deref().unwrap_or(&open.key);
                    let instance = change.instance.as_deref().or(open.instance.as_deref());
                    let mut entry =
                        change_entry(record, &mut values, target, instance, change, None)?;
                    entry.why = Some(why.clone());
                    entries.push(entry);
                }
                entries.push(Entry {
                    notice: Some(notice.clone()),
                    done: true,
                    ..Entry::default()
                });
                Ok(stamp(entries, session, new_id))
            }
            Act::Dismiss { drawn } => {
                let notice = &drawn.id;
                open_as_drawn(state, drawn)?;
                Ok(stamp(
                    vec![Entry {
                        notice: Some(notice.clone()),
                        dismissed: true,
                        ..Entry::default()
                    }],
                    session,
                    new_id,
                ))
            }
            Act::DismissAll { drawn } => {
                let open = open_as_drawn(state, drawn)?;
                let NoticeRef::Day { mark, .. } = &open.id else {
                    return Err(Refused::Changed(Part::Notice));
                };
                if open.days != drawn.days {
                    return Err(Refused::Changed(Part::DayCount));
                }
                let ends = open.days.iter().map(|day| Entry {
                    notice: Some(NoticeRef::Day {
                        mark: mark.clone(),
                        day: *day,
                    }),
                    dismissed: true,
                    ..Entry::default()
                });
                Ok(stamp(ends.collect(), session, new_id))
            }
        }
    }
}

/// The total and faces of `dice`; `None` when Splat cannot roll them.
fn roll_dice(dice: &str, new_id: fn() -> Uuid) -> Option<(i64, Vec<i64>)> {
    let mut random = Random::seeded(new_id().as_u128() as u64);
    let roll = Dice::parse(dice).ok()?.roll(&mut random, false).ok()?;
    Some((roll.total, roll.dice))
}

fn faces_text(dice: &str, faces: &[i64]) -> String {
    let faces: Vec<String> = faces.iter().map(ToString::to_string).collect();
    format!("{dice}: [{}]", faces.join(", "))
}

/// A roll on a marked table: its dice as rolled, the total, and the row's words.
fn roll_table(
    rules: &Rules,
    key: &str,
    new_id: fn() -> Uuid,
) -> Result<(String, i64, String), Refused> {
    let table = rules
        .tables
        .as_ref()
        .and_then(|tables| tables.table(key))
        .ok_or_else(|| Refused::UnknownTable(key.to_owned()))?;
    if table.fault().is_some() {
        return Err(Refused::TableCannotRoll(key.to_owned()));
    }
    let (total, faces) =
        roll_dice(&table.dice, new_id).ok_or_else(|| Refused::TableCannotRoll(key.to_owned()))?;
    let row = table
        .rows
        .iter()
        .find(|row| (row.min..=row.max).contains(&total))
        .map(|row| row.text.clone())
        .ok_or_else(|| Refused::NoRow {
            table: key.to_owned(),
            total,
        })?;
    Ok((faces_text(&table.dice, &faces), total, row))
}

// ---------------------------------------------------------------------------------------------
// Acts: each builds its entries from the ledger on disk at the click.

/// One change's entry, with the value it leaves. A change that would pass `min` is refused.
/// The line's name goes only to a tracker with `per`.
fn change_entry(
    record: Record<'_>,
    values: &mut HashMap<(String, Option<String>), i64>,
    key: &str,
    instance: Option<&str>,
    change: &Change,
    asked: Option<i64>,
) -> Result<Entry, Refused> {
    let Record {
        rules,
        state,
        new_id,
        ..
    } = record;
    let target = rules
        .def(key)
        .ok_or_else(|| Refused::UnknownTracker(key.to_owned()))?;
    let instance = instance.filter(|_| !target.per.is_empty());
    if !target.has_line(instance) {
        return Err(Refused::NoLine {
            tracker: target.name.clone(),
            line: instance.map(str::to_owned),
        });
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
        change
            .add
            .as_ref()
            .map(Amount::read)
            .transpose()
            .map_err(|_| Refused::BadAmount(target.name.clone()))?,
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
            let (total, faces) =
                roll_dice(&dice, new_id).ok_or_else(|| Refused::BadAmount(target.name.clone()))?;
            entry.add = Some(total);
            entry.roll = Some(Roll::Dice(faces_text(&dice, &faces)));
            before + total
        }
        (None, Some(Add::Ask { minus })) => {
            let typed = asked.ok_or(Refused::NoNumber)?;
            let n = if minus { -typed } else { typed };
            entry.add = Some(n);
            before + n
        }
        (None, None) => return Err(Refused::NoAmount(target.name.clone())),
    };
    if let Some(min) = target.min
        && after < min
        && after < before
    {
        return Err(Refused::BelowMin {
            tracker: target.name.clone(),
            min,
        });
    }
    values.insert(place, after);
    Ok(entry)
}

/// IDs, the session's stamp, and one `use` for a press of several entries.
fn stamp(mut entries: Vec<Entry>, session: Option<Uuid>, new_id: fn() -> Uuid) -> Op {
    let press = (entries.len() > 1).then(|| ledger::new_id("u", new_id()));
    for entry in &mut entries {
        entry.id = ledger::new_id("c", new_id());
        entry.press = press.clone();
        entry.session = session;
    }
    Op::Append(entries)
}

/// The entries of a quick action, pressed on one line.
fn press_op(
    context: Record<'_>,
    key: &str,
    instance: Option<&str>,
    action: QuickRef<'_>,
    asked: Option<i64>,
) -> Result<Op, Refused> {
    let Record {
        rules,
        session,
        new_id,
        ..
    } = context;
    let QuickRef { label, at } = action;
    let def = rules
        .def(key)
        .ok_or_else(|| Refused::UnknownTracker(key.to_owned()))?;
    if instance.is_some_and(|name| !def.per.iter().any(|word| word == name)) {
        return Err(Refused::Changed(Part::Name));
    }
    let quick = match at {
        Some(at) => def.quick.get(at).filter(|quick| quick.label == label),
        None => def.quick.iter().find(|quick| quick.label == label),
    }
    .ok_or(Refused::Changed(Part::Action))?;
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
        let mut entry = change_entry(context, &mut values, target, line, change, asked)?;
        entry.why = Some(label.to_owned());
        entries.push(entry);
    }
    if let Some(table) = &quick.table {
        let (dice, _, row) = roll_table(rules, table, new_id)?;
        entries[0].roll = Some(Roll::Dice(dice));
        entries[0].row = Some(row);
    }
    Ok(stamp(entries, session, new_id))
}
