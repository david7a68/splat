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
    // X6: the added effect's Undo has one form, `Undo: add effect <words>`; a shortened target
    // keeps the copy number that tells Cinder Wolf 2 from Cinder Wolf 1, or goes.
    let first = current.split(' ').next().unwrap_or_default();
    let copy = current.rsplit(' ').next().unwrap_or_default();
    for size in [SMALL, DEFAULT] {
        d.at_size_mut(size, |d| {
            undo_paints_a_form(d, "Undo: add effect Wis save", first, copy)
        });
    }
    // W1-D1: on its turn the row leads with the effect due on it, though the DM added it
    // after Bless. X10: a count of the rest ends the effects' slot with no dot before it.
    for size in [SMALL, DEFAULT, LARGE] {
        let words = d.at_size(size, |d| d.row_words(&current));
        let (hold, bless) = (words.find("Wis save"), words.find("Bless"));
        d.observe(
            "W1-D1",
            hold.is_some_and(|hold| bless.is_none_or(|bless| hold < bless))
                && !words.contains("· ⚑"),
            format!("At {size:?} the current row paints: {words}"),
        );
    }
    d.state("effects-on-its-turn", Sizes::All);

    // A label with no time is as today: it shows on no notice and does not count.
    d.click_row(WOLF);
    by_key(d, "Hexed until the hag dies");
    let recharged = format!(" for {CULTIST}: Fire Breath ");
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
    // X4: the roll shows its die in the dice total's slot, and one line in the run's footer
    // names the participant, the effect and the result.
    let result = d
        .nodes()
        .into_iter()
        .map(|node| node.name)
        .find(|name| name.starts_with("Rolled ") && name.contains(&recharged));
    let die = result
        .as_deref()
        .and_then(|words| words.split_whitespace().nth(1))
        .map(|die| format!("Rolled {die} for {CULTIST}: Fire Breath"));
    // Review of PR 292: the dock's total is named by its participant and effect.
    d.observe(
        "X4",
        die.as_deref().is_some_and(|die| d.has(die)),
        format!("The footer says {result:?}, and the dock shows {die:?}"),
    );
    d.state("effects-on-rows", Sizes::Default);
    // X4: Undo of the roll takes its die and its line away with it; the roll again brings both.
    d.click_containing("Undo: recharge roll");
    d.observe(
        "X4",
        !d.has_containing(&recharged) && die.as_deref().is_none_or(|die| !d.has(die)),
        "Undo of the roll ends its result",
    );
    d.click("Roll 1d6. 5–6 makes it ready");
    let die = d
        .nodes()
        .into_iter()
        .map(|node| node.name)
        .find(|name| name.starts_with("Rolled ") && name.contains(&recharged))
        .and_then(|words| {
            words
                .split_whitespace()
                .nth(1)
                .map(|die| format!("Rolled {die} for {CULTIST}: Fire Breath"))
        });

    // T20.3: the count goes down when its participant's turn ends.
    d.click_row(&current);
    d.press(Key::N);
    d.observe(
        "T20.3",
        row_name(d, &current).contains("Bless 9") && reminders(d, &current).is_none(),
        format!("After its turn: {}", row_name(d, &current)),
    );
    d.observe(
        "X4",
        die.is_some()
            && !d.has_containing(&recharged)
            && die.as_deref().is_none_or(|die| !d.has(die)),
        format!("The recharge's result line and its die {die:?} go at the next act"),
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

    // T20.1 beside Lookup at 960x640: the letter keys are text, and the dock has no room for
    // `Effect` after Absent (its name slot and the dice total's place), so the pointer takes
    // the Conditions tab's field, as F2 says: select, Conditions, the field, the words, Enter.
    // Conditions closes Lookup, as a run task does, and Ctrl+K shows it again.
    look_up(d, "frightened");
    d.observe(
        "T20.1",
        !d.has(EFFECT),
        "Beside Lookup at 960x640 the dock has no room for Effect",
    );
    d.begin_task();
    d.click_row(ISELLE);
    d.click("Conditions for this participant");
    d.click("Other effect");
    d.type_text("Frightened 10 rounds");
    d.press(Key::Enter);
    d.goal_within("T20.1", "pin an effect by the pointer beside Lookup", 5);
    d.observe(
        "T20.1",
        row_name(d, ISELLE).contains("Frightened 10"),
        format!("The effect joined its row: {}", row_name(d, ISELLE)),
    );
    d.press_ctrl(Key::K);
    d.observe("T20.1", d.has(SEARCH_LOOKUP), "Ctrl+K shows Lookup again");
    d.state("effects-beside-lookup", Sizes::Default);
    // T20.4: no row changed its height for a time: each row has the height of the same row
    // with each time taken away, where each effect is its plain words (rule P, as the ruling on
    // PR 276 states it; X5). A plain label can grow a row as it always could.
    d.press_ctrl(Key::K);
    let after = heights(d, &rows);
    let untimed = d.aside(|d| {
        let timed = d.app_mut().encounters.simulate_untimed();
        d.settle();
        let untimed = heights(d, &rows);
        d.app_mut().encounters.simulate_conditions(timed);
        d.settle();
        untimed
    });
    d.observe(
        "T20.4",
        after == untimed && heights(d, &rows) == after,
        format!(
            "Rows keep the heights they have with no time: {untimed:?} then {after:?}; before \
             any effect {before:?}"
        ),
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
    // X11: a ticked condition shows its count on its tick, and Other effects holds other
    // words only. X7: the help says what Splat does with a time. X8: the dock has no room for
    // `Effect` here, so the field's words give an example with a time.
    let tick = format!("{} Frightened 10", crate::design::icons::OK);
    d.observe(
        "X11",
        d.visible_text(&tick) && !d.has("More actions for Frightened 10"),
        format!("The tick says {tick:?}, and Other effects does not list it again"),
    );
    d.observe(
        "X7",
        d.has_containing("A number after the words counts down at the end of that creature’s turn: Bless 10 becomes Bless 9.")
            && !d.has_containing("No number changes by itself."),
        "The Conditions help says what is true now",
    );
    d.observe(
        "X8",
        !d.has(EFFECT) && d.visible_text("Other effect, such as Bless 10 rounds"),
        "Where the dock has no room for Effect, the field says how to give a time",
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
