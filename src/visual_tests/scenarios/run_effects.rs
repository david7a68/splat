//! F2, effects and turn reminders. An effect with a count joins a row from the dock, its count
//! goes down when its participant's turn ends, the notice line repeats it on that turn, and the
//! row's targets spend, roll and remove. Each surface is shown with Lookup closed and open at
//! 960x640, and at 1200x800.
use super::*;

const EFFECT: &str = "Effect: words for this row";
const FIELD: &str = "Effect";
const WOLF: &str = "Cinder Wolf 1";
const CULTIST: &str = "Ember Cultist";
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

/// The notice line's reminders on this participant's turn: their hover and accessible name.
fn reminders(d: &Driver<'_>, name: &str) -> Option<String> {
    let start = format!("Reminders for {name}: ");
    d.nodes()
        .iter()
        .find_map(|node| node.name.strip_prefix(&start).map(str::to_owned))
}

pub fn run(d: &mut Driver<'_>) {
    resume_run_from_cold_start(d);
    let current = current_participant(d).expect("a turn");
    d.click_row(&current);
    let next_turn = d.place_of("Next turn");
    let rows_before: Vec<_> = [current.as_str(), WOLF, CULTIST]
        .iter()
        .map(|name| d.row_node(name).map(|row| row.shown.height()))
        .collect();

    // T17.1: by the key, 3 acts with the row selected; focus goes back to Amount.
    d.begin_task();
    by_key(d, "Frightened 10 rounds");
    d.goal_within("T20.1", "pin an effect with a count to the selected row", 3);
    d.observe(
        "T20.1",
        d.row_words(&current).contains("Frightened 10")
            && d.focused_name().as_deref() == Some("Amount"),
        format!(
            "The row says the count, and Amount has focus: {} / {:?}",
            d.row_words(&current),
            d.focused_name()
        ),
    );
    by_key(d, &format!("{HOLD} on its turn"));
    // T17.2: on its turn the reminders repeat the row's words on the notice line, and Next
    // turn has not moved.
    d.observe(
        "T20.2",
        reminders(d, &current).as_deref() == Some(&format!("Frightened 10 · {HOLD}")[..]),
        format!("The notice line names the reminders: {:?}", reminders(d, &current)),
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
    // T17.5: a use is spent in 1, on the row; the recharge is marked spent and rolled in 1.
    by_key(d, "Legendary actions 3 uses");
    d.click_row(CULTIST);
    by_key(d, "Fire Breath recharge 5-6");
    d.begin_task();
    d.click(&format!("Spend one use of Legendary actions of {WOLF}"));
    d.goal_within("T20.5", "spend a use on the row", 1);
    d.observe(
        "T20.5",
        d.row_words(WOLF).contains("Legendary actions 2 of 3"),
        format!("The row counts the use: {}", d.row_words(WOLF)),
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
    // No row changed its height for its effects.
    let rows_after: Vec<_> = [current.as_str(), WOLF, CULTIST]
        .iter()
        .map(|name| d.row_node(name).map(|row| row.shown.height()))
        .collect();
    d.observe(
        "T20.4",
        rows_after == rows_before,
        format!("Rows keep their heights: {rows_before:?} then {rows_after:?}"),
    );
    d.state("effects-on-rows", Sizes::Default);

    // T17.3: the count goes down when its participant's turn ends.
    d.click_row(&current);
    d.press(Key::N);
    d.observe(
        "T20.3",
        d.row_words(&current).contains("Frightened 9") && reminders(d, &current).is_none(),
        format!("After its turn: {}", d.row_words(&current)),
    );
    // A crash is not a turn: the saved count shows again, and Undo of Next turn gives it back.
    d.restart();
    d.observe(
        "T20.3",
        d.row_words(&current).contains("Frightened 9"),
        format!("After a restart: {}", d.row_words(&current)),
    );
    d.click_containing("Undo: next turn");
    d.observe(
        "T20.3",
        d.row_words(&current).contains("Frightened 10"),
        format!("Undo of Next turn gives the count back: {}", d.row_words(&current)),
    );

    // An official condition's name opens its rule beside the run; its count is a second
    // target, which opens Conditions.
    d.click("Rule: Frightened");
    d.expect(SEARCH_LOOKUP);
    d.state("effects-beside-lookup", Sizes::Default);
    // T17.1 beside Lookup: the letter keys are text, so the pointer adds an effect in 4.
    d.begin_task();
    by_pointer(d, WOLF, "Bless 10 rounds");
    d.goal_within("T20.1", "pin an effect by the pointer beside Lookup", 4);
    d.observe(
        "T20.1",
        d.row_words(WOLF).contains("Bless 10") || d.row_words(WOLF).contains(icons::PIN),
        format!("The effect joined its row: {}", d.row_words(WOLF)),
    );
    d.press_ctrl(Key::K);
    d.click("Frightened 10. Remove it in Conditions");
    d.observe(
        "F2",
        d.in_view("Other effect"),
        "The count opens Conditions, which lists each effect",
    );
    d.state("effects-in-conditions", Sizes::Default);
}
