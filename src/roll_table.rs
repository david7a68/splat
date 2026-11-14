//! Roll tables: a one-line marker above a table in any note makes it rollable. A row has no
//! effect by itself. The Markdown reader draws the control, and the app decides whose dice roll.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use crate::design::{self, Action, Density, Tone, Type, colors};
use eframe::egui;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use splat::content::ROLLS_MARKER;
use splat::execution::dice::{Dice, Random};

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
                    dest_url.strip_prefix(crate::markdown_view::NOTE_LINK),
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
pub(crate) fn fault_count(body: &str) -> usize {
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
    assert_eq!(fault_count(&valid.replace("dice=1d4", "dice=1d4 per=watch")), 0);
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

#[derive(Clone, Default)]
struct RollState {
    typed: String,
    result: Option<String>,
    value: Option<i64>,
}

/// One table's roll state in one reader. `reader` is the reader's own `Ui` id, so World and
/// Lookup showing the same table keep their own typed roll, result and field (round 26 AE1).
fn roll_state_id(reader: egui::Id, key: &str) -> egui::Id {
    egui::Id::new(("roll_table", reader, key))
}

fn request_id() -> egui::Id {
    egui::Id::new("roll_table_request")
}

/// The roll state that asked for the pending roll, so its result goes back to that reader only.
fn requester_id() -> egui::Id {
    egui::Id::new("roll_table_requester")
}

/// **Roll** and a box for a typed result, under a marked table. `reader` is the id of the
/// reader that shows the table: the same one `matched_row` gets.
pub fn roll_control(
    ui: &mut egui::Ui,
    reader: egui::Id,
    marker: &Result<Marker, String>,
    table: &[Vec<String>],
) {
    let checked = marker
        .as_ref()
        .map_err(String::clone)
        .and_then(|marker| Ok((marker, table_rows(&marker.dice, table)?)));
    let (Marker { key, dice, per }, rows) = match checked {
        Ok(checked) => checked,
        // One bad table says so where it is. The rest of the note still reads.
        Err(error) => {
            design::message(ui, Tone::Warning, &error);
            return;
        }
    };
    let id = roll_state_id(reader, key);
    let mut state: RollState = ui.data(|data| data.get_temp(id)).unwrap_or_default();
    let mut request = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = design::S;
        if Action::secondary(&format!("Roll {dice}"))
            .icon(design::icons::DICE)
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            request = Some(None);
        }
        let response = design::number_field_with_id(
            ui,
            id.with("typed"),
            &format!("Typed roll for {}", design::reader::human_label(key)),
            &mut state.typed,
            "Typed roll",
            design::XXL * 3.0,
            Density::Read.control(),
        );
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            match state.typed.trim().parse::<i64>() {
                Ok(value) => request = Some(Some(value)),
                Err(_) if state.typed.trim().is_empty() => {}
                Err(_) => state.result = Some("Type a whole number.".to_owned()),
            }
            state.typed.clear();
        }
    });
    if let Some(typed) = request {
        let roll = TableRoll {
            key: key.clone(),
            dice: dice.clone(),
            typed,
            rows,
        };
        ui.data_mut(|data| {
            data.insert_temp(request_id(), roll);
            data.insert_temp(requester_id(), id);
        });
    }
    // One line under the Roll row is kept for the result, empty until the first roll, so a roll
    // moves nothing below the table. A result longer than the line ends in `…`; its hover and
    // its accessible name keep it whole, and the table marks the rolled row.
    let line = Type::Body.line();
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), line), egui::Sense::hover());
    if let Some(result) = &state.result {
        let mut slot = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        slot.add(egui::Label::new(Type::Body.rich(&slot, result).color(colors().text)).truncate());
    }
    // The table still rolls; only a journey's checks need the word. Round 27 FE4: every warning
    // under a table has one form, `design::message`, as a fault in its dice or rows has.
    if let Some(warning) = per.warning() {
        design::message(ui, Tone::Warning, &warning);
    }
    ui.data_mut(|data| data.insert_temp(id, state));
}

pub fn take_roll(ctx: &egui::Context) -> Option<TableRoll> {
    ctx.data_mut(|data| data.remove_temp::<TableRoll>(request_id()))
}

/// Show a roll's result under its table, in the reader that asked for it.
pub fn settle_roll(ctx: &egui::Context, result: String) {
    ctx.data_mut(|data| {
        let Some(id) = data.get_temp::<egui::Id>(requester_id()) else {
            return;
        };
        data.remove::<egui::Id>(requester_id());
        let mut state: RollState = data.get_temp(id).unwrap_or_default();
        state.value = result
            .split_whitespace()
            .nth(1)
            .and_then(|word| word.parse().ok());
        state.result = Some(result);
        data.insert_temp(id, state);
    });
}

pub fn matched_row(
    ui: &egui::Ui,
    reader: egui::Id,
    marker: &Result<Marker, String>,
    table: &[Vec<String>],
) -> Option<usize> {
    let Marker { key, dice, .. } = marker.as_ref().ok()?;
    let value = ui
        .data(|data| data.get_temp::<RollState>(roll_state_id(reader, key)))?
        .value?;
    table_rows(dice, table)
        .ok()?
        .iter()
        .position(|row| (row.min..=row.max).contains(&value))
        .map(|row| row + 1)
}

/// With no run open, a result only shows. It uses dice of its own.
pub fn roll_alone(roll: &TableRoll) -> Result<String, String> {
    roll.resolve(&mut Random::seeded(
        crate::hermetic::new_uuid().as_u128() as u64
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_table_must_cover_what_its_dice_can_roll() {
        let row = |a: &str, b: &str| vec![a.to_owned(), b.to_owned()];
        let head = row("d6", "Result");
        let good = [head.clone(), row("1", "a"), row("2–5", "b"), row("6", "c")];
        let rows = table_rows("1d6", &good).unwrap();
        assert_eq!(rows[1].min, 2);
        assert_eq!(rows[1].max, 5);
        let gap = [head.clone(), row("1", "a"), row("3-6", "b")];
        assert!(table_rows("1d6", &gap).unwrap_err().contains("gap"));
        let short = [head.clone(), row("1-5", "a")];
        assert!(table_rows("1d6", &short).unwrap_err().contains("up to 6"));
        let words = [head, row("2 to 5", "a")];
        assert!(table_rows("1d6", &words).unwrap_err().contains("2 to 5"));
    }

    #[test]
    fn a_bad_first_column_is_named_in_curly_quotes() {
        let row = |a: &str, b: &str| vec![a.to_owned(), b.to_owned()];
        let words = [row("d6", "Result"), row("2 to 5", "a")];
        assert_eq!(
            table_rows("1d6", &words).unwrap_err(),
            "The first column must be a number or a range such as 2-5, not “2 to 5”."
        );
    }

    #[test]
    fn a_typed_result_finds_its_row_without_dice() {
        let roll = TableRoll {
            key: "gifts".into(),
            dice: "1d6".into(),
            typed: Some(4),
            rows: vec![RollRow {
                min: 1,
                max: 6,
                text: "A potion".into(),
            }],
        };
        assert_eq!(roll_alone(&roll).unwrap(), "Typed 4 on 1d6: A potion");
    }

    #[test]
    fn a_marker_finds_the_table_right_after_it_and_no_later_one() {
        let note = "<!-- splat:rolls key=gifts dice=1d2 -->\n\n| d2 | Gift |\n|---|---|\n| 1 | A `knife` |\n| 2 | A net |\n\nWords.\n\n| Plain | Table |\n|---|---|\n| a | b |\n\n<!-- splat:rolls key=lost dice=1d2 -->\n\nWords break the link.\n\n| d2 | Lost |\n|---|---|\n| 1-2 | c |\n";
        let found = marked_tables(note);
        assert_eq!(found.len(), 1);
        let (marker, rows) = &found[0];
        let marker = marker.as_ref().unwrap();
        assert_eq!(
            (marker.key.as_str(), marker.dice.as_str()),
            ("gifts", "1d2")
        );
        assert_eq!(rows.len(), 3, "the heading row and two more");
        assert_eq!(rows[1], ["1", "A knife"]);
        assert_eq!(table_rows("1d2", rows).unwrap()[1].text, "A net");
        // The descriptor that names a table outside its note: its note, its heading, its key
        // and its dice. A heading belongs to the tables under it until the next one.
        let headed = format!("# Gifts\n\n## On the `road`\n\n{note}\n## Empty\n");
        let tables = tables_in([("World/Road.md", headed.as_str()), ("Other.md", note)]);
        assert_eq!(tables.len(), 1, "the first table with a key decides it");
        let table = &tables[0];
        assert_eq!(
            (
                table.path.as_str(),
                table.heading.as_deref(),
                table.key.as_str(),
                table.dice.as_str()
            ),
            ("World/Road.md", Some("On the road"), "gifts", "1d2")
        );
        assert_eq!((table.rows.len(), table.fault()), (2, None));
        assert_eq!((table.name(), &table.per), ("On the road", &Per::Day));
        // A row's words keep a link to an encounter, so a journey's result can offer it.
        let linked = "<!-- splat:rolls key=road dice=1d2 -->\n\n| d2 | Road |\n|---|---|\n| 1 | [[Wolf Pack]] pick up the trail. |\n| 2 | Quiet. |\n";
        assert_eq!(
            tables_in([("Road.md", linked)])[0].linked_words(1),
            Some("[[Wolf Pack]] pick up the trail.")
        );
        // The campaign's index holds a note's links as Markdown links: the row's words read as
        // before, and its linked words name the note again.
        let indexed = crate::world::with_links(linked, &Default::default());
        let table = &tables_in([("Road.md", indexed.as_str())])[0];
        assert_eq!(
            (table.rows[0].text.as_str(), table.linked_words(1)),
            (
                "Wolf Pack pick up the trail.",
                Some("[[Wolf Pack]] pick up the trail.")
            )
        );
        let plain = &tables_in([("World/Other.md", note)])[0];
        assert_eq!((plain.heading.as_deref(), plain.name()), (None, "Other"));
    }

    #[test]
    fn markers_need_their_key_and_dice() {
        let marker = |per: &str| {
            rolls_marker(&format!("<!-- splat:rolls key=gifts dice=1d6{per} -->\n"))
                .unwrap()
                .unwrap()
        };
        assert_eq!(
            marker(""),
            Marker {
                key: "gifts".to_owned(),
                dice: "1d6".to_owned(),
                per: Per::Day
            }
        );
        // F9: the interval, with its words after the dice. A word that cannot be read rolls one
        // check each day, and says so.
        for (per, read, hours, words) in [
            (" per=day", Per::Day, 24, "each day"),
            (" per=watch", Per::Watch, 8, "each watch"),
            (" per=4h", Per::Hours(4), 4, "every 4 hours"),
            (" per=1h", Per::Hours(1), 1, "every hour"),
            (" per=0h", Per::NotRead("0h".into()), 24, "each day"),
            (
                " per=fortnight",
                Per::NotRead("fortnight".into()),
                24,
                "each day",
            ),
        ] {
            let found = marker(per).per;
            assert_eq!(
                (&found, found.hours(), found.words().as_str()),
                (&read, hours, words)
            );
        }
        assert!(matches!(
            rolls_marker("<!-- splat:rolls key=gifts -->"),
            Some(Err(_))
        ));
        assert!(rolls_marker("<!-- splat:inventory -->").is_none());
    }
}
