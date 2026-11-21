//! Marked roll tables and their results.
use crate::content::ROLLS_MARKER;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use splat_content::dice::{Dice, Random};

#[derive(Clone, Debug, PartialEq)]
pub struct RollRow {
    pub min: i64,
    pub max: i64,
    pub text: String,
}

/// A roll on a marked table. `typed` is a result the DM read off a player's own dice.
#[derive(Clone, Debug, Default)]
pub struct TableRoll {
    pub key: String,
    pub dice: String,
    pub typed: Option<i64>,
    pub rows: Vec<RollRow>,
}

impl TableRoll {
    /// The result's line in the log and on the table.
    pub fn line(&self, value: i64, faces: Option<&[i64]>) -> String {
        let row = self
            .rows
            .iter()
            .find(|row| (row.min..=row.max).contains(&value))
            .map_or("No row has this result.", |row| row.text.as_str());
        let word = if faces.is_some() { "Rolled" } else { "Typed" };
        format!("{word} {value} on {}: {row}", self.dice)
    }

    /// The number and its row's words, rolled on this stream or typed.
    pub fn draw(&self, random: &mut Random) -> Result<(i64, String), String> {
        let value = match self.typed {
            Some(value) => value,
            None => Dice::parse(&self.dice)?.roll(random, false)?.total,
        };
        let row = self
            .rows
            .iter()
            .find(|row| (row.min..=row.max).contains(&value))
            .ok_or_else(|| format!("{} rolled {value}, and no row has it.", self.dice))?;
        Ok((value, row.text.clone()))
    }

    /// Roll, or take the typed result, on this stream of dice.
    pub fn resolve(&self, random: &mut Random) -> Result<String, String> {
        if let Some(value) = self.typed {
            return Ok(self.line(value, None));
        }
        let roll = Dice::parse(&self.dice)?.roll(random, false)?;
        Ok(self.line(roll.total, Some(&roll.dice)))
    }
}

/// How often a table rolls on a journey: the marker's optional `per` word. A day is the value
/// when there is none.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Per {
    #[default]
    Day,
    /// Eight hours.
    Watch,
    Hours(u64),
    /// A word that Splat cannot read. A journey then rolls one check each day.
    NotRead(String),
}

/// The warning on a table whose `per` word Splat cannot read: it quotes the word the DM wrote,
/// says what a journey does instead, and names the words that work (round 27 FE13).
pub fn per_not_read(word: &str) -> String {
    format!(
        "Splat cannot read how often to check (“per {word}”). It rolls one check each day. Use day, watch or hours such as 4h."
    )
}

impl Per {
    pub fn read(word: &str) -> Self {
        match word {
            "day" => Self::Day,
            "watch" => Self::Watch,
            _ => match word.strip_suffix('h').and_then(|hours| hours.parse().ok()) {
                Some(hours) if hours > 0 => Self::Hours(hours),
                _ => Self::NotRead(word.to_owned()),
            },
        }
    }

    /// The warning for a word Splat cannot read; none for a word it reads.
    pub fn warning(&self) -> Option<String> {
        match self {
            Self::NotRead(word) => Some(per_not_read(word)),
            _ => None,
        }
    }

    /// The hours between two checks.
    pub fn hours(&self) -> u64 {
        match self {
            Self::Day | Self::NotRead(_) => 24,
            Self::Watch => 8,
            Self::Hours(hours) => *hours,
        }
    }

    /// How often, in the words after the dice: `each day`, `each watch`, `every 4 hours`.
    pub fn words(&self) -> String {
        match self {
            Self::Day | Self::NotRead(_) => "each day".to_owned(),
            Self::Watch => "each watch".to_owned(),
            Self::Hours(1) => "every hour".to_owned(),
            Self::Hours(hours) => format!("every {hours} hours"),
        }
    }
}

/// What a `splat:rolls` marker says: the table's key, its dice and how often a journey rolls it.
#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    pub key: String,
    pub dice: String,
    pub per: Per,
}

/// A marker, or why it has none, and the table's rows.
pub type MarkedTable = (Result<Marker, String>, Vec<Vec<String>>);

/// The key, dice and interval of a `splat:rolls` marker.
pub fn rolls_marker(html: &str) -> Option<Result<Marker, String>> {
    let inner = html.trim().strip_prefix(ROLLS_MARKER)?;
    let inner = inner.strip_suffix("-->").unwrap_or(inner);
    let (mut key, mut dice, mut per) = (None, None, Per::Day);
    for word in inner.split_whitespace() {
        if let Some(value) = word.strip_prefix("key=") {
            key = Some(value.to_owned());
        } else if let Some(value) = word.strip_prefix("dice=") {
            dice = Some(value.to_owned());
        } else if let Some(value) = word.strip_prefix("per=") {
            per = Per::read(value);
        }
    }
    Some(match (key, dice) {
        (Some(key), Some(dice)) => Ok(Marker { key, dice, per }),
        _ => Err("This roll table’s marker needs key= and dice=.".to_owned()),
    })
}

/// Each marked table in a note, with its marker and its rows, heading row first. A marker
/// belongs to the table right after it, as the reader draws it.
pub fn marked_tables(body: &str) -> Vec<MarkedTable> {
    marked_tables_under_headings(body)
        .into_iter()
        .map(|(_, table, _)| table)
        .collect()
}

/// One marked table, as a place outside its own note names it: its source note, its heading,
/// its key, its dice and its interval, and its rows checked against the dice. A table's
/// identity is its key; its heading is for reading.
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub key: String,
    pub dice: String,
    /// The note that holds it.
    pub path: String,
    /// The note's last heading above it, when there is one.
    pub heading: Option<String>,
    /// How often a journey rolls it.
    pub per: Per,
    /// Why it cannot roll, when it cannot.
    pub problem: Option<String>,
    /// Its rows, checked against its dice. Empty when it cannot roll.
    pub rows: Vec<RollRow>,
    /// Each row's words with their links as `[[…]]`, in the order of `rows`, so a result kept
    /// in a file can still name what it links.
    pub markup: Vec<String>,
}

impl Table {
    /// Its name for reading: its heading, or else its note's name.
    pub fn name(&self) -> &str {
        self.heading.as_deref().unwrap_or_else(|| {
            let file = self.path.rsplit('/').next().unwrap_or(&self.path);
            file.strip_suffix(".md").unwrap_or(file)
        })
    }

    /// A roll on this table, or why it cannot roll.
    pub fn roll(&self, typed: Option<i64>) -> Result<TableRoll, String> {
        match self.fault() {
            Some(fault) => Err(fault),
            None => Ok(TableRoll {
                key: self.key.clone(),
                dice: self.dice.clone(),
                typed,
                rows: self.rows.clone(),
            }),
        }
    }

    /// The words, with their links, of the row that has this result.
    pub fn linked_words(&self, value: i64) -> Option<&str> {
        let at = self
            .rows
            .iter()
            .position(|row| (row.min..=row.max).contains(&value))?;
        self.markup.get(at).map(String::as_str)
    }

    /// Why it cannot roll, with the note that holds it.
    pub fn fault(&self) -> Option<String> {
        let path = &self.path;
        self.problem
            .as_ref()
            .map(|problem| format!("{path}: {problem}"))
    }
}

/// The marked tables with a key in `(path, body)` notes, in that order. The first table with a
/// key decides it, so a bad one says why and no later table takes its place by surprise.
pub fn tables_in<'a>(notes: impl IntoIterator<Item = (&'a str, &'a str)>) -> Vec<Table> {
    let mut tables: Vec<Table> = Vec::new();
    for (path, body) in notes {
        for (heading, (marker, rows), linked) in marked_tables_under_headings(body) {
            let Ok(Marker { key, dice, per }) = marker else {
                continue;
            };
            if tables.iter().any(|table| table.key == key) {
                continue;
            }
            let (rows, problem) = match table_rows(&dice, &rows) {
                Ok(rows) => (rows, None),
                Err(error) => (Vec::new(), Some(error)),
            };
            // A row's words with links, found by its range, as the rows are in dice order.
            let markup = rows
                .iter()
                .map(|row| {
                    linked
                        .iter()
                        .skip(1)
                        .find(|cells| {
                            cells
                                .first()
                                .and_then(|cell| parse_range(cell))
                                .is_some_and(|range| range == (row.min, row.max))
                        })
                        .map_or_else(|| row.text.clone(), |cells| cells[1..].join(" · "))
                })
                .collect();
            tables.push(Table {
                key,
                dice,
                path: path.to_owned(),
                heading,
                per,
                problem,
                rows,
                markup,
            });
        }
    }
    tables
}

/// `marked_tables`, with the last heading above each table, and its cells again with each link
/// to a note as `[[…]]`. A note in the campaign's index has its links as Markdown links.
type Headed = (Option<String>, MarkedTable, Vec<Vec<String>>);
fn marked_tables_under_headings(body: &str) -> Vec<Headed> {
    let mut found = Vec::new();
    let mut marker = None;
    let mut rows: Option<Vec<Vec<String>>> = None;
    let mut linked: Vec<Vec<String>> = Vec::new();
    let mut link: Option<(String, usize)> = None;
    let mut heading: Option<String> = None;
    let mut in_heading = false;
    for event in Parser::new_ext(body, Options::ENABLE_TABLES) {
        match event {
            Event::Start(Tag::Heading { .. }) if rows.is_none() => {
                marker = None;
                in_heading = true;
                heading = Some(String::new());
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                heading = heading
                    .map(|words| words.trim().to_owned())
                    .filter(|words| !words.is_empty());
            }
            Event::Text(ref text) | Event::Code(ref text) if in_heading => {
                if let Some(words) = &mut heading {
                    words.push_str(text);
                }
            }
            Event::Html(ref text) if rolls_marker(text).is_some() => marker = rolls_marker(text),
            Event::Start(
                Tag::Paragraph | Tag::List(_) | Tag::CodeBlock(_) | Tag::BlockQuote(_),
            ) if rows.is_none() => marker = None,
            Event::Start(Tag::Table(_)) => {
                rows = Some(Vec::new());
                linked.clear();
            }
            Event::Start(Tag::TableHead | Tag::TableRow) => {
                if let Some(rows) = &mut rows {
                    rows.push(Vec::new());
                    linked.push(Vec::new());
                }
            }
            Event::Start(Tag::TableCell) => {
                if let Some(row) = rows.as_mut().and_then(|rows| rows.last_mut()) {
                    row.push(String::new());
                    if let Some(row) = linked.last_mut() {
                        row.push(String::new());
                    }
                }
            }
            Event::Start(Tag::Link { ref dest_url, .. }) if rows.is_some() => {
                if let (Some(target), Some(cell)) = (
                    dest_url.strip_prefix(crate::note::NOTE_LINK),
                    linked.last().and_then(|row| row.last()),
                ) {
                    link = Some((target.to_owned(), cell.len()));
                }
            }
            Event::End(TagEnd::Link) => {
                if let (Some((target, start)), Some(cell)) = (
                    link.take(),
                    linked.last_mut().and_then(|row| row.last_mut()),
                ) {
                    let words = cell.split_off(start);
                    if words == target {
                        cell.push_str(&format!("[[{target}]]"));
                    } else {
                        cell.push_str(&format!("[[{target}|{words}]]"));
                    }
                }
            }
            Event::Text(ref text) | Event::Code(ref text) => {
                if let Some(cell) = rows
                    .as_mut()
                    .and_then(|rows| rows.last_mut())
                    .and_then(|row| row.last_mut())
                {
                    cell.push_str(text);
                    if let Some(cell) = linked.last_mut().and_then(|row| row.last_mut()) {
                        cell.push_str(text);
                    }
                }
            }
            Event::End(TagEnd::Table) => {
                if let (Some(marker), Some(rows)) = (marker.take(), rows.take()) {
                    found.push((heading.clone(), (marker, rows), std::mem::take(&mut linked)));
                }
                rows = None;
            }
            _ => {}
        }
    }
    found
}

/// The table warnings the reader shows, by the reader's checks: a table that cannot roll, and
/// a `per` word Splat cannot read (the table still rolls, but the DM must change the note; round
/// 28 FE3). A table with both has two.
pub fn fault_count(body: &str) -> usize {
    marked_tables(body)
        .into_iter()
        .map(|(marker, rows)| match marker {
            Err(_) => 1,
            Ok(marker) => {
                usize::from(table_rows(&marker.dice, &rows).is_err())
                    + usize::from(marker.per.warning().is_some())
            }
        })
        .sum()
}

#[test]
fn fault_count_keeps_valid_tables_and_counts_each_invalid_table_once() {
    let valid =
        "<!-- splat:rolls key=a dice=1d4 -->\n\n| d4 | Result |\n|---|---|\n| 1-4 | Fine |\n";
    let broken = valid.replace("1-4", "2-4");
    assert_eq!(fault_count(valid), 0);
    assert_eq!(fault_count(&format!("{valid}\n{broken}\n{broken}")), 2);
    // An unreadable `per` word is a warning too; a word Splat reads is not.
    let fortnight = valid.replace("dice=1d4", "dice=1d4 per=fortnight");
    assert_eq!(
        fault_count(&valid.replace("dice=1d4", "dice=1d4 per=watch")),
        0
    );
    assert_eq!(fault_count(&format!("{fortnight}\n{broken}")), 2);
    assert_eq!(fault_count(&fortnight.replace("1-4", "2-4")), 2);
}

fn parse_range(cell: &str) -> Option<(i64, i64)> {
    let cell = cell.trim();
    if let Ok(value) = cell.parse() {
        return Some((value, value));
    }
    let (min, max) = cell.split_once(['–', '—', '-'])?;
    Some((min.trim().parse().ok()?, max.trim().parse().ok()?))
}

/// The rows of a marked table, checked against what its dice can roll.
pub fn table_rows(dice: &str, rows: &[Vec<String>]) -> Result<Vec<RollRow>, String> {
    let (low, high) = Dice::parse(dice)?.range();
    let mut parsed = Vec::new();
    for row in rows.iter().skip(1) {
        let first = row.first().map_or("", String::as_str);
        let Some((min, max)) = parse_range(first) else {
            return Err(format!(
                "The first column must be a number or a range such as 2-5, not “{first}”."
            ));
        };
        parsed.push(RollRow {
            min,
            max,
            text: row[1..].join(" · "),
        });
    }
    parsed.sort_by_key(|row| row.min);
    let mut next = low;
    for row in &parsed {
        if row.min < next {
            return Err(format!(
                "{dice} has an overlap at {}. Use each result once.",
                row.min
            ));
        }
        if row.min > next || row.max < row.min {
            return Err(format!("{dice} needs a row for {next}. It has a gap here."));
        }
        next = row.max + 1;
    }
    if next != high + 1 {
        return Err(format!(
            "the table stops at {}, but {dice} rolls up to {high}.",
            next - 1
        ));
    }
    Ok(parsed)
}
