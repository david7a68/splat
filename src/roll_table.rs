//! Roll tables: a one-line marker above a table in any note makes it rollable. A row has no
//! effect by itself. The Markdown reader draws the control, and the app decides whose dice roll.
use crate::design::{self, Action, Density, Tone, Type, colors};
use eframe::egui;
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
        match faces {
            Some(faces) => format!("{} {}: {faces:?} = {value}. {row}", self.key, self.dice),
            None => format!("{}: {value}, typed. {row}", self.key),
        }
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

/// The key and dice of a `splat:rolls` marker.
pub fn rolls_marker(html: &str) -> Option<Result<(String, String), String>> {
    let inner = html.trim().strip_prefix(ROLLS_MARKER)?;
    let inner = inner.strip_suffix("-->").unwrap_or(inner);
    let (mut key, mut dice) = (None, None);
    for word in inner.split_whitespace() {
        if let Some(value) = word.strip_prefix("key=") {
            key = Some(value.to_owned());
        } else if let Some(value) = word.strip_prefix("dice=") {
            dice = Some(value.to_owned());
        }
    }
    Some(match (key, dice) {
        (Some(key), Some(dice)) => Ok((key, dice)),
        _ => Err("This roll table's marker needs key= and dice=.".to_owned()),
    })
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
                "The first column must be a number or a range such as 2–5, not \"{first}\"."
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
        if row.min != next || row.max < row.min {
            return Err(format!("{dice} needs a row for {next}. It has a gap here."));
        }
        next = row.max + 1;
    }
    if next != high + 1 {
        return Err(format!(
            "{dice} can roll up to {high}, and the rows stop before that."
        ));
    }
    Ok(parsed)
}

#[derive(Clone, Default)]
struct RollState {
    typed: String,
    result: Option<String>,
}

fn roll_state_id(key: &str) -> egui::Id {
    egui::Id::new(("roll_table", key))
}

fn request_id() -> egui::Id {
    egui::Id::new("roll_table_request")
}

/// **Roll** and a box for a typed result, under a marked table.
pub fn roll_control(
    ui: &mut egui::Ui,
    marker: &Result<(String, String), String>,
    table: &[Vec<String>],
) {
    let checked = marker
        .as_ref()
        .map_err(String::clone)
        .and_then(|(key, dice)| Ok((key, dice, table_rows(dice, table)?)));
    let (key, dice, rows) = match checked {
        Ok(checked) => checked,
        // One bad table says so where it is. The rest of the note still reads.
        Err(error) => {
            design::message(ui, Tone::Warning, &error);
            return;
        }
    };
    let id = roll_state_id(key);
    let mut state: RollState = ui.data(|data| data.get_temp(id)).unwrap_or_default();
    let mut request = None;
    ui.horizontal_wrapped(|ui| {
        if Action::secondary(&format!("Roll {dice}"))
            .icon(design::icons::DICE)
            .density(Density::Read)
            .name(&format!("Roll {key}"))
            .show(ui)
            .clicked()
        {
            request = Some(None);
        }
        let response = ui.add(
            egui::TextEdit::singleline(&mut state.typed)
                .desired_width(48.0)
                .min_size(egui::vec2(48.0, Density::Read.control()))
                .hint_text("or type"),
        );
        design::named(ui, &response, &format!("Typed result for {key}"));
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
        ui.data_mut(|data| data.insert_temp(request_id(), roll));
    }
    if let Some(result) = &state.result {
        ui.label(Type::Stat.rich(ui, result).color(colors().text));
    }
    ui.data_mut(|data| data.insert_temp(id, state));
}

pub fn take_roll(ctx: &egui::Context) -> Option<TableRoll> {
    ctx.data_mut(|data| data.remove_temp::<TableRoll>(request_id()))
}

/// Show a roll's result under its table.
pub fn settle_roll(ctx: &egui::Context, key: &str, result: String) {
    let id = roll_state_id(key);
    ctx.data_mut(|data| {
        let mut state: RollState = data.get_temp(id).unwrap_or_default();
        state.result = Some(result);
        data.insert_temp(id, state);
    });
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
        assert_eq!(roll_alone(&roll).unwrap(), "gifts: 4, typed. A potion");
    }

    #[test]
    fn markers_need_their_key_and_dice() {
        assert_eq!(
            rolls_marker("<!-- splat:rolls key=gifts dice=1d6 -->\n"),
            Some(Ok(("gifts".to_owned(), "1d6".to_owned())))
        );
        assert!(matches!(
            rolls_marker("<!-- splat:rolls key=gifts -->"),
            Some(Err(_))
        ));
        assert!(rolls_marker("<!-- splat:inventory -->").is_none());
    }
}
