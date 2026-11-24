//! Roll tables: a one-line marker above a table in any note makes it rollable. A row has no
//! effect by itself. The Markdown reader draws the control, and the app decides whose dice roll.
use crate::design::outcome::{BodyMessage, Notice, Sentence};
use crate::design::{self, Action, Density, Type, colors};
use eframe::egui;
use splat::execution::dice::Random;
pub use splat_campaign::roll_table::*;

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

/// A warning under a table, in the words its marker's parser gave: a typed notice in the
/// reader's body, where the table is.
#[allow(
    clippy::disallowed_methods,
    reason = "a body place: a table’s warning stands under the table it is about, in its parser’s words (Ban::Body, Ban::Words)"
)]
fn table_warning(ui: &mut egui::Ui, words: &str) {
    if let Some(words) = Sentence::outside(words) {
        let _ = BodyMessage::<()>::from(Notice::warning(words)).show(ui);
    }
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
            table_warning(ui, &error);
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
            match design::parse_signed(&state.typed) {
                Some(value) => request = Some(Some(value)),
                None if state.typed.trim().is_empty() => {}
                None => state.result = Some(TYPED_NOT_A_NUMBER.to_owned()),
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
    // under a table has one form, `table_warning`, as a fault in its dice or rows has.
    if let Some(warning) = per.warning() {
        table_warning(ui, &warning);
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
    matched_in(ui.ctx(), reader, marker, table)
}

/// The rolled row of a table in one reader (its header is row 0), for any view: the state is
/// kept in `ctx` under the reader's id, as [`roll_control`] keeps it.
pub fn matched_in(
    ctx: &egui::Context,
    reader: egui::Id,
    marker: &Result<Marker, String>,
    table: &[Vec<String>],
) -> Option<usize> {
    let Marker { key, dice, .. } = marker.as_ref().ok()?;
    let value = ctx
        .data(|data| data.get_temp::<RollState>(roll_state_id(reader, key)))?
        .value?;
    table_rows(dice, table)
        .ok()?
        .iter()
        .position(|row| (row.min..=row.max).contains(&value))
        .map(|row| row + 1)
}

/// The last result line of a table in one reader, for a view that is not egui.
pub fn result_in(ctx: &egui::Context, reader: egui::Id, key: &str) -> Option<String> {
    ctx.data(|data| data.get_temp::<RollState>(roll_state_id(reader, key)))?
        .result
}

/// Ask for a roll of a marked table from a view that is not egui: `typed` is a roll the DM
/// typed, `None` to roll the dice. Its result comes back to this reader (`settle_roll`). The
/// error is the warning a bad table shows under itself.
pub fn ask_in(
    ctx: &egui::Context,
    reader: egui::Id,
    marker: &Result<Marker, String>,
    table: &[Vec<String>],
    typed: Option<i64>,
) -> Result<(), String> {
    let marker = marker.as_ref().map_err(String::clone)?;
    let rows = table_rows(&marker.dice, table)?;
    let id = roll_state_id(reader, &marker.key);
    let roll = TableRoll {
        key: marker.key.clone(),
        dice: marker.dice.clone(),
        typed,
        rows,
    };
    ctx.data_mut(|data| {
        data.insert_temp(request_id(), roll);
        data.insert_temp(requester_id(), id);
    });
    Ok(())
}

/// A typed roll that is not a whole number, from a view that is not egui: its result line says
/// so, as [`roll_control`]'s does.
pub fn typed_not_a_number_in(ctx: &egui::Context, reader: egui::Id, key: &str) {
    let id = roll_state_id(reader, key);
    ctx.data_mut(|data| {
        let mut state: RollState = data.get_temp(id).unwrap_or_default();
        state.result = Some(TYPED_NOT_A_NUMBER.to_owned());
        data.insert_temp(id, state);
    });
}

/// The result line of a typed roll that is not a whole number.
const TYPED_NOT_A_NUMBER: &str = "Type a whole number.";

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
