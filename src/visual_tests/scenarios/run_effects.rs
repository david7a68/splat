//! F2, effects and turn reminders. An effect with a count joins a row from the dock, its count
//! goes down when its participant's turn ends, the notice line repeats it on that turn, and the
//! row's targets spend, roll and remove. Each surface is shown with Lookup closed and open at
//! 960x640, and at 1200x800.
use super::*;

const EFFECT: &str = "Effect: words for this row";
const WOLF: &str = "Cinder Wolf 1";
const CULTIST: &str = "Ember Cultist";
const ISELLE: &str = "Iselle Marrowind";
const HOLD: &str = "Wis save DC 15 (Hold Person)";

/// Add an effect to the selected row by its key, as a DM at the table does.
fn by_key(d: &mut Driver<'_>, words: &str) {
    d.press(Key::E);
    d.type_text(words);
    d.press(Key::Enter);
}

/// Add an effect by the pointer, beside Lookup, where the letter keys are text.
fn by_pointer(d: &mut Driver<'_>, row: &str, words: &str) {
    d.click_row(row);
    d.click(EFFECT);
    d.type_text(words);
    d.press(Key::Enter);
}

/// A row's accessible name, which holds each effect's whole words.
fn row_name(d: &Driver<'_>, name: &str) -> String {
    let start = format!("{name}, ");
    d.nodes()
        .iter()
        .find(|node| node.name.starts_with(&start))
        .map_or_else(String::new, |node| node.name.clone())
}

/// The notice line's reminders on this participant's turn: their hover and accessible name.
fn reminders(d: &Driver<'_>, name: &str) -> Option<String> {
    let start = format!("Reminders for {name}: ");
    d.nodes()
        .iter()
        .find_map(|node| node.name.strip_prefix(&start).map(str::to_owned))
}

/// The heights of these rows, as the roster draws them.
fn heights(d: &Driver<'_>, rows: &[&str]) -> Vec<Option<f32>> {
    rows.iter()
        .map(|name| d.row_node(name).map(|row| row.shown.height()))
        .collect()
}

pub fn run(d: &mut Driver<'_>) {
    resume_run_from_cold_start(d);
    let current = current_participant(d).expect("a turn");
    // Iselle is Frightened before the DM gives it a time, so her row shows the condition
    // from the start: a time on a condition that shows keeps the row (review of PR 276).
    d.aside(|d| {
        d.click_row(ISELLE);
        by_key(d, "Frightened");
    });
    d.click_row(&current);
    let next_turn = d.place_of("Next turn");
    let rows = [current.as_str(), WOLF, CULTIST, ISELLE];
    let before = heights(d, &rows);

    // T20.1: by the key, 3 acts with the row selected; focus goes back to Amount.
    d.begin_task();
    by_key(d, "Bless 10 rounds");
    d.goal_within("T20.1", "pin an effect with a count to the selected row", 3);
    d.observe(
        "T20.1",
        row_name(d, &current).contains("Bless 10") && d.focused_name().as_deref() == Some("Amount"),
        format!(
            "The row says the count, and Amount has focus: {} / {:?}",
            row_name(d, &current),
            d.focused_name()
        ),
    );
    by_key(d, &format!("{HOLD} on its turn"));
    // T20.2: on its turn the reminders repeat the row's words on the notice line, at 1200x800
    // where the line has room, and Next turn has not moved. At 960x640 the state notice and
    // Undo leave none, and the row has them.
    let shown = d.at_size(DEFAULT, |d| reminders(d, &current));
    d.observe(
        "T20.2",
        shown.as_deref() == Some(&format!("Bless 10 · {HOLD}")[..]),
        format!("The notice line names the reminders: {shown:?}"),
    );
    d.observe(
        "T20.2",
        d.place_of("Next turn") == next_turn,
        "Next turn keeps its place beside the reminders",
    );
    d.state("effects-on-its-turn", Sizes::All);

    // A label with no time is as today: it shows on no notice and does not count.
    d.click_row(WOLF);
    by_key(d, "Hexed until the hag dies");
    // T20.5: a use is spent in 1, on the row; a spent recharge rolls in 1.
    by_key(d, "Legendary actions 3 uses");
    d.click_row(CULTIST);
    by_key(d, "Fire Breath recharge 5-6");
    d.begin_task();
    d.click(&format!("Spend one use of Legendary actions of {WOLF}"));
    d.goal_within("T20.5", "spend a use on the row", 1);
    d.observe(
        "T20.5",
        row_name(d, WOLF).contains("Legendary actions 2 of 3"),
        format!("The row counts the use: {}", row_name(d, WOLF)),
    );
    d.click("Mark Fire Breath spent");
    d.begin_task();
    d.click("Roll 1d6. 5–6 makes it ready");
    d.goal_within("T20.5", "roll a spent recharge on the row", 1);
    d.observe(
        "T20.5",
        d.name_starting("Undo") == format!("Undo: recharge roll for {CULTIST}"),
        format!("The roll is one named step: {}", d.name_starting("Undo")),
    );
    d.state("effects-on-rows", Sizes::Default);

    // T20.3: the count goes down when its participant's turn ends.
    d.click_row(&current);
    d.press(Key::N);
    d.observe(
        "T20.3",
        row_name(d, &current).contains("Bless 9") && reminders(d, &current).is_none(),
        format!("After its turn: {}", row_name(d, &current)),
    );
    // A crash is not a turn: the saved count shows again, and Undo of Next turn gives it back.
    d.restart();
    d.observe(
        "T20.3",
        row_name(d, &current).contains("Bless 9"),
        format!("After a restart: {}", row_name(d, &current)),
    );
    d.click_containing("Undo: next turn");
    d.observe(
        "T20.3",
        row_name(d, &current).contains("Bless 10"),
        format!(
            "Undo of Next turn gives the count back: {}",
            row_name(d, &current)
        ),
    );

    // T20.1 beside Lookup: the letter keys are text, so the pointer adds an effect in 4, and
    // Lookup stays as the DM left it.
    look_up(d, "frightened");
    d.begin_task();
    by_pointer(d, ISELLE, "Frightened 10 rounds");
    d.goal_within("T20.1", "pin an effect by the pointer beside Lookup", 4);
    d.observe(
        "T20.1",
        row_name(d, ISELLE).contains("Frightened 10") && d.has(SEARCH_LOOKUP),
        format!(
            "The effect joined its row, and Lookup stays: {}",
            row_name(d, ISELLE)
        ),
    );
    d.state("effects-beside-lookup", Sizes::Default);
    // T20.4: no row changed its height for its effects.
    d.press_ctrl(Key::K);
    let after = heights(d, &rows);
    d.observe(
        "T20.4",
        after == before,
        format!("Rows keep their heights: {before:?} then {after:?}"),
    );
    // T20.5: an official condition's name opens its rule beside the run; its count is a
    // second target, which opens Conditions.
    d.click("Rule: Frightened");
    d.expect(SEARCH_LOOKUP);
    d.press_ctrl(Key::K);
    d.click("Frightened 10. Remove it in Conditions");
    d.observe(
        "F2",
        d.in_view("Other effect"),
        "The count opens Conditions, which lists each effect",
    );
    d.state("effects-in-conditions", Sizes::Default);

    // T20.6: damage to a row marked Conc gives its save's DC, with no action, notice or new
    // line, until the next step.
    let before = heights(d, &rows);
    d.click_row(&current);
    d.type_text("5");
    d.press(Key::Enter);
    let dc = row_name(d, &current).contains("Conc save DC 10");
    let after = heights(d, &rows);
    d.observe(
        "T20.6",
        dc && after == before,
        format!(
            "The row names the save's DC and keeps its height: {}",
            row_name(d, &current)
        ),
    );
    d.press(Key::N);
    d.observe(
        "T20.6",
        !row_name(d, &current).contains("Conc save DC"),
        "The DC lasts until the next step",
    );
}
