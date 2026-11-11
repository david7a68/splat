//! F9: a journey's encounter checks, from the plan's Travel row and from Timeline. The campaign
//! has two marked tables, so the first journey starts with no table chosen.
use super::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

const ROAD: &str = "# Road\n\n## On the road\n\n<!-- splat:rolls key=road dice=1d20 per=day -->\n\n| d20 | On the road |\n| --- | --- |\n| 1-14 | Nothing stirs. |\n| 15-17 | [[Bridge Ambush]] waits by the old quarry gate, where the road narrows between walls of loose stone. |\n| 18-20 | A toll gang blocks the way: [[Shrine of Cinders]]. |\n";
const AMBUSH: &str = "[[Bridge Ambush]] waits by the old quarry gate";
const MORE_TRAVEL: &str = "More actions for Travel";
const ROLL_FOR_ENCOUNTERS: &str = "Roll for encounters";
const PLAN_AMBUSH: &str = "Plan “Bridge Ambush”";

fn folder(d: &mut Driver<'_>) -> PathBuf {
    let campaign = d.app_mut().campaigns.active_campaign().unwrap();
    crate::sessions::folder(&campaign).unwrap()
}

fn timeline_file(dir: &Path) -> String {
    fs::read_to_string(dir.join("Timeline.md")).unwrap_or_default()
}

/// The session note's words, as the file holds them.
fn note_file(dir: &Path) -> String {
    fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|file| file.file_name() != "Timeline.md")
        .map(|file| fs::read_to_string(file.path()).unwrap_or_default())
        .find(|text| text.contains("## Plan for tonight"))
        .expect("the session note")
}

/// The journey's checks as the app holds them.
fn checks(d: &Driver<'_>) -> Vec<crate::timeline::Check> {
    d.app()
        .timeline
        .events
        .iter()
        .find(|event| event.kind == crate::timeline::Kind::Travel)
        .map(|event| event.checks.clone())
        .unwrap_or_default()
}

pub fn run(d: &mut Driver<'_>) {
    let campaign = d.root.join("campaigns/emberfall");
    fs::write(campaign.join("Road.md"), ROAD).unwrap();
    d.app_mut().lookup.refresh();
    d.settle();
    d.click(NAV_SESSIONS);
    d.click("New session");
    d.type_into("Session title", "The Crossing");
    d.click("Create session");
    d.expect("Session 1: The Crossing");
    let dir = folder(d);
    show_plan(d);
    // A day sets the clock, then the journey.
    d.click_add_event("Add Day");
    d.click_add_event("Add Travel");
    d.click(MORE_TRAVEL);
    d.observe(
        "T11.10",
        d.has(ROLL_FOR_ENCOUNTERS) && !d.has("Encounter checks"),
        "A Travel row is as before until the DM asks; its menu offers the checks",
    );
    d.begin_task();
    d.click(ROLL_FOR_ENCOUNTERS);
    d.expect("Encounter checks added to this journey.");
    d.observe(
        "T11.10",
        d.has("Choose a table")
            && d.has("Choose a table first.")
            && d.focused_name().as_deref() == Some("How long")
            && timeline_file(&dir).contains("  - Checks table:\n"),
        "With several tables and no earlier choice, the block waits for a table, and focus is in How long",
    );
    // The event holds the block through a restart, with no table chosen yet.
    d.restart();
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click("Travel");
    d.observe(
        "T11.10",
        d.has("Choose a table") && d.has("Encounter checks"),
        "The journey's checks wait for a table after a restart",
    );
    d.click("Choose a table");
    d.click("On the road");
    d.expect("· 1d20 each day");
    d.type_into("How long", "2 days");
    let travel = |d: &Driver<'_>| {
        d.app()
            .timeline
            .events
            .iter()
            .find(|event| event.kind == crate::timeline::Kind::Travel)
            .map(|event| event.duration.clone())
            .unwrap_or_default()
    };
    d.observe(
        "T11.9",
        d.has("Roll for Day 1")
            && d.has("Roll for Day 2")
            && d.has("Roll all 2")
            && travel(d).is_empty(),
        "The lines follow the length as it is typed; the event takes it at Enter",
    );
    d.press(Key::Enter);
    d.observe(
        "T11.9",
        travel(d) == "2 days" && d.visible_text("Day 1, 00:00 to Day 3, 00:00"),
        "Enter gives the journey its length",
    );
    // Counted at 1200x800, as the design counts it. At 960x640 the open row scrolls with the
    // plan, as open rows do, and `Roll all` can need one scroll more.
    d.set_base(DEFAULT);
    d.begin_task();
    d.click("Roll all 2");
    d.expect("2 travel checks rolled.");
    d.goal_within("T11.9", "roll a two-day journey's checks", 1);
    d.set_base(SMALL);
    let file = timeline_file(&dir);
    d.observe(
        "T11.9",
        file.contains("  - Check at 0h: \"") && file.contains("  - Check at 24h: \""),
        format!("Each check is a quoted line of the event: {file}"),
    );
    d.click("Undo roll");
    d.observe(
        "T11.9",
        checks(d).is_empty() && d.has("Roll all 2"),
        "Undo roll takes back the press's two results",
    );
    d.click("Roll all 2");
    d.click("Clear the check for Day 2");
    d.expect("Check cleared.");
    d.type_into("Enter a roll for Day 2", "16");
    d.press(Key::Enter);
    d.expect("1 travel check rolled.");
    d.observe(
        "T11.10",
        d.has(PLAN_AMBUSH)
            && timeline_file(&dir).contains(&format!("  - Check at 24h: \"16 · {AMBUSH}")),
        "A player's roll finds its row, and a result that links an encounter offers it",
    );
    d.state("travel-checks-rolled", Sizes::All);
    d.state_prepared("travel-checks-rolled-lookup", Sizes::Small, |d| {
        if !d.has("Close Lookup") {
            d.click(NAV_LOOKUP);
        }
        d.expect("Encounter checks");
    });
    d.click("Close Lookup");
    // Act A: plan the fight. It is the next row, with Start fight.
    d.begin_task();
    d.click(PLAN_AMBUSH);
    d.expect("“Bridge Ambush” is in tonight’s plan.");
    d.goal_within("T11.10", "plan a result as a fight", 1);
    let parts = [
        d.has_containing("In the plan"),
        !d.has(PLAN_AMBUSH),
        // The fixture's Bridge Ambush has a paused run, so its row resumes it.
        d.has("Start fight: Bridge Ambush") || d.has("Resume: Bridge Ambush"),
        note_file(&dir).contains("- [[Bridge Ambush]]"),
    ];
    d.observe(
        "T11.10",
        parts.iter().all(|part| *part),
        format!("One click puts the fight in the plan, and the result says so: {parts:?}"),
    );
    // Act B, unrelated: clear the first day's result. Then Undo of A takes only A's line.
    d.click("Clear the check for Day 1");
    d.expect("Check cleared.");
    d.click("Undo plan line");
    d.observe(
        "T11.10",
        !note_file(&dir).contains("- [[Bridge Ambush]]")
            && d.has(PLAN_AMBUSH)
            && checks(d).len() == 1,
        "Undo plan line takes back the plan line and leaves the cleared check",
    );
    d.click("Undo clear");
    d.observe(
        "T11.9",
        checks(d).len() == 2,
        "Undo clear puts the result back",
    );
    // A shorter journey deletes no result.
    d.replace_text_in("How long", "1 day");
    d.press(Key::Enter);
    d.observe(
        "T11.9",
        d.has("Outside this journey now") && checks(d).len() == 2,
        "A result outside the shorter journey stays, under its own line",
    );
    d.state("travel-checks-outside", Sizes::Default);
    // An edit in Obsidian: Splat shows the words as they are, and the offer goes with the link.
    d.app_mut().timeline.save();
    let path = dir.join("Timeline.md");
    let edited = fs::read_to_string(&path)
        .unwrap()
        .lines()
        .map(|line| {
            if line.starts_with("  - Check at 24h:") {
                "  - Check at 24h: 16 · Two wolves, driven off.".to_owned()
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&path, edited).unwrap();
    d.app_mut().timeline.read_if_changed();
    d.settle();
    d.observe(
        "T11.9",
        d.has_containing("Two wolves, driven off.") && !d.has(PLAN_AMBUSH),
        "A result edited in Obsidian shows its words",
    );
    d.click("Clear the check for Day 2");
    d.observe(
        "T11.9",
        !timeline_file(&dir).contains("Check at 24h") && checks(d).len() == 1,
        "Clear removes the edited line",
    );
    // No checks waits until no result stands.
    d.click("On the road");
    d.observe(
        "T11.10",
        d.has("Clear the results first."),
        "No checks waits while a result stands",
    );
    d.press(Key::Escape);
    d.click("Clear the check for Day 1");
    d.click("On the road");
    d.click("No checks");
    d.observe(
        "T11.10",
        !d.has("Encounter checks") && !timeline_file(&dir).contains("Checks table"),
        "No checks takes the block off the row and its line out of the event",
    );
    // On Timeline, the event's own menu has the same act, and the block is under How long.
    d.click(NAV_TIMELINE);
    d.click_containing("Travel ·");
    d.click(MORE_TRAVEL);
    d.click(ROLL_FOR_ENCOUNTERS);
    d.expect("Encounter checks added to this journey.");
    d.click("Choose a table");
    d.click("On the road");
    d.click("Roll for Day 1");
    d.observe(
        "T11.9",
        d.has("Encounter checks") && checks(d).len() == 1 && d.has("Clear the check for Day 1"),
        "Timeline's detail column holds the same block",
    );
    d.state("travel-checks-timeline", Sizes::Default);
    d.state_prepared("travel-checks-timeline-lookup", Sizes::Small, |d| {
        if !d.has("Close Lookup") {
            d.click(NAV_LOOKUP);
        }
        d.expect("Encounter checks");
    });
}
