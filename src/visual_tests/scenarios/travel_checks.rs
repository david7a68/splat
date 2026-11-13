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

/// The journey with this title, as the app holds it.
fn journey(d: &Driver<'_>, title: &str) -> crate::timeline::Event {
    d.app()
        .timeline
        .events
        .iter()
        .find(|event| event.kind == crate::timeline::Kind::Travel && event.title == title)
        .cloned()
        .expect("the journey")
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

/// Faults in the open block's layout (V4, V22, V23): `Encounter checks` is a label above the
/// table menu; the dice words share the menu's line, or start their own line with their words;
/// and the act's result, when there is one, sits on its kept line between the menu's line and
/// `next`, the first control under it.
fn block_faults(d: &Driver<'_>, result: Option<&str>, next: &str) -> Vec<String> {
    let nodes = d.nodes();
    let place = |name: &str| {
        nodes
            .iter()
            .find(|node| node.name == name)
            .map(|node| node.rect)
    };
    let (Some(label), Some(menu), Some(next)) =
        (place("Encounter checks"), place("On the road"), place(next))
    else {
        return vec![format!("the label, the menu or {next:?} is missing")];
    };
    let mut faults = Vec::new();
    // The menu's target starts at the label's edge, as a field's frame does.
    if label.bottom() > menu.top() || (label.left() - menu.left()).abs() > 1.0 {
        faults.push(format!(
            "the label {label:?} is not above the menu {menu:?}"
        ));
    }
    match nodes
        .iter()
        .find(|node| node.name.ends_with("1d20 each day"))
    {
        Some(dice) if dice.name.starts_with('·') => {
            if !(menu.top()..=menu.bottom()).contains(&dice.rect.center().y) {
                faults.push(format!("{:?} {:?} starts a line", dice.name, dice.rect));
            }
        }
        Some(dice) if dice.rect.top() < menu.bottom() => {
            faults.push(format!("{:?} lost its `·` on the menu's line", dice.name));
        }
        Some(_) => {}
        None => faults.push("no dice words".to_owned()),
    }
    if let Some(words) = result {
        match place(words) {
            Some(at) if at.top() >= menu.bottom() && at.bottom() <= next.top() => {}
            at => faults.push(format!(
                "{words:?} {at:?} is not between the menu {menu:?} and {next:?}"
            )),
        }
    }
    faults
}

/// The tops of the open Travel row's parts below its checks, from its label `Encounter checks`:
/// Title, Notes and the row's `Timeline` link.
fn below_checks(d: &Driver<'_>) -> Vec<Option<f32>> {
    let nodes = d.nodes();
    let top =
        |found: Option<&crate::visual_tests::driver::Found>| found.map(|node| node.rect.top());
    let label = top(nodes.iter().find(|node| node.name == "Encounter checks"));
    let named = |name: &str| top(nodes.iter().find(|node| node.name == name));
    let link = top(nodes
        .iter()
        .find(|node| node.name == "Timeline" && node.is_link()));
    [named("Title"), named("Notes on this event"), link]
        .into_iter()
        .map(|at| Some(at? - label?))
        .collect()
}

/// From the bottom of the table menu's line to the top of a control under it.
fn below_menu(d: &Driver<'_>, name: &str) -> Option<f32> {
    Some(d.place_of(name)?.top() - d.place_of("On the road")?.bottom())
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
    const STARTED: &str = "Encounter checks added to this journey.";
    d.expect(STARTED);
    d.observe(
        "T11.10",
        d.has("Choose a table")
            && d.has("Choose a table first.")
            && d.focused_name().as_deref() == Some("How long")
            && timeline_file(&dir).contains("  - Checks table:\n"),
        "With several tables and no earlier choice, the block waits for a table, and focus is in How long",
    );
    // App round 2: where the line has no room, the result shortens by whole words, never
    // inside a word (at 1200x800 the note is beside the plan). Review round 1: widths between,
    // so that a cut that is blind to words lands inside one at some of them.
    for size in [
        SMALL,
        (1040.0, 800.0),
        (1080.0, 800.0),
        (1120.0, 800.0),
        (1160.0, 800.0),
        DEFAULT,
        (1240.0, 800.0),
        (1280.0, 800.0),
        LARGE,
    ] {
        // The line that paints the result: whole, or its start and `…`.
        let line = d.at_size(size, |d| {
            d.painted_lines_in(egui::Rect::EVERYTHING)
                .into_iter()
                .find(|line| {
                    line == STARTED
                        || line
                            .strip_suffix('…')
                            .is_some_and(|kept| !kept.is_empty() && STARTED.starts_with(kept))
                })
        });
        let whole_words = line.as_deref().is_some_and(|line| {
            line == STARTED
                || line.strip_suffix('…').is_some_and(|kept| {
                    STARTED
                        .strip_prefix(kept)
                        .is_some_and(|rest| rest.starts_with(' '))
                })
        });
        d.observe(
            "T11.10-whole-words",
            whole_words,
            format!("At {size:?}, the result reads {line:?}"),
        );
    }
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
    let length_at = d.place_of("How long");
    d.begin_task();
    d.click("Roll all 2");
    d.expect("2 travel checks rolled.");
    d.goal_within("T11.9", "roll a two-day journey's checks", 1);
    let faults = block_faults(d, Some("2 travel checks rolled."), "Clear the check for Day 1");
    d.observe(
        "T11.9",
        faults.is_empty() && length_at.is_some() && d.place_of("How long") == length_at,
        format!(
            "The label is above the table, and the result's kept line moves no field: {faults:?}"
        ),
    );
    // W1-D1: the checks come first in the open row, Title and Notes after them.
    let order = (d.place_of("Clear the check for Day 2"), d.place_of("Title"));
    d.observe(
        "T11.10",
        matches!(order, (Some(last), Some(title)) if title.top() > last.bottom()),
        format!("The open Travel row shows its checks before Title: {order:?}"),
    );
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
    // Round 27 FE7: How long comes first, the results follow the table, and the plan offer's
    // line ends the checks with no reserved band before Title.
    for size in [SMALL, DEFAULT, LARGE] {
        let (ordered, detail) = d.at_size(size, |d| {
            let length = d.place_of("How long");
            let label = d.place_of("Encounter checks");
            let offer = d.place_of(PLAN_AMBUSH);
            let title = d.place_of("Title");
            (
                length
                    .zip(label)
                    .is_some_and(|(length, label)| length.bottom() < label.top())
                    && offer.zip(title).is_some_and(|(offer, title)| {
                        title.top() > offer.bottom()
                            && title.top() - offer.bottom() <= crate::design::L + 1.0
                    }),
                format!("How long {length:?}, label {label:?}, offer {offer:?}, Title {title:?}"),
            )
        });
        d.observe(
            "iter-27-S-FE7",
            ordered,
            format!("At {size:?}, How long is above the checks and Title follows them: {detail}"),
        );
    }
    d.state("travel-checks-rolled", Sizes::All);
    d.state_prepared("travel-checks-rolled-lookup", Sizes::Small, |d| {
        if !d.has("Close Lookup") {
            d.click(NAV_LOOKUP);
        }
        d.expect("Encounter checks");
    });
    let faults = block_faults(d, Some("1 travel check rolled."), "Clear the check for Day 1");
    d.observe(
        "T11.9",
        faults.is_empty(),
        format!("At 960x640 beside Lookup, the block keeps its layout: {faults:?}"),
    );
    d.click("Close Lookup");
    // Act A: plan the fight. It is the next row, with Start fight.
    // Places from the block's label: planning reveals the new fight row, which can scroll the plan.
    let from_label = |d: &Driver<'_>, name: &str| {
        let label = d.place_of("Encounter checks")?;
        Some(d.place_of(name)?.translate(-label.min.to_vec2()))
    };
    let below = below_checks(d);
    let offer = from_label(d, PLAN_AMBUSH);
    d.begin_task();
    d.click(PLAN_AMBUSH);
    d.expect("“Bridge Ambush” is in tonight’s plan.");
    d.goal_within("T11.10", "plan a result as a fight", 1);
    // Round 27 FE7: the message and its Undo are in the plan's footer; `In the plan` takes the
    // offer's place, so nothing below the checks moves.
    let in_plan = from_label(d, "In the plan");
    let after = below_checks(d);
    let message = from_label(d, "“Bridge Ambush” is in tonight’s plan.");
    d.observe(
        "T11.10",
        below.iter().all(Option::is_some)
            && after == below
            && d.has("Undo plan line")
            && matches!((offer, in_plan), (Some(offer), Some(at)) if offer.y_range().contains(at.center().y))
            && matches!((offer, message), (Some(offer), Some(at)) if at.top() > offer.bottom()),
        format!("In the plan is on the check's line ({offer:?}, {in_plan:?}), the message {message:?} is in the footer, not there, and nothing moves ({below:?}, {after:?})"),
    );
    d.state("travel-checks-planned", Sizes::Small);
    // Beside Lookup the row is narrower and taller: the plan scrolls to the message.
    d.state_prepared("travel-checks-planned-lookup", Sizes::Small, |d| {
        if !d.has("Close Lookup") {
            d.click(NAV_LOOKUP);
        }
        let at = d.place_of("Encounter checks").expect("the block").center();
        d.scroll_until(at, "Undo plan line", 8);
    });
    d.click("Close Lookup");
    let parts = [
        d.has_containing("in tonight’s plan"),
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
    // The plan line's Undo is the note's: a page change, with the note as the act left it,
    // keeps it.
    let planned_note = note_file(&dir);
    d.click(NAV_TIMELINE);
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.observe(
        "T11.10",
        note_file(&dir) == planned_note && d.has("Undo plan line"),
        "Undo plan line stays through a visit to another page",
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
    // An edit of the note ends the plan line's Undo: `In the plan` takes the message's line.
    d.click(PLAN_AMBUSH);
    let below = below_checks(d);
    write_note(d);
    d.type_into_editor("\nThe road is quiet.\n");
    show_plan(d);
    let after = below_checks(d);
    d.observe(
        "T11.10",
        d.has_containing("In the plan") && !d.has("Undo plan line") && after == below,
        format!(
            "After a note edit, In the plan stands in the message's place ({below:?}, {after:?})"
        ),
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
    // A journey of no length has no check, as none is at its end; its results stay.
    d.replace_text_in("How long", "0 days");
    d.press(Key::Enter);
    d.observe(
        "T11.9",
        !d.has("Roll for Day 1")
            && !d.has("Roll for This journey")
            && d.has("Clear the check for Day 1")
            && d.has("Clear the check for Day 2")
            && checks(d).len() == 2,
        "A journey of zero length has no check to roll, and keeps its results",
    );
    d.replace_text_in("How long", "1 day");
    d.press(Key::Enter);
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
    let before = below_menu(d, "Roll for Day 1");
    d.click("Roll for Day 1");
    d.observe(
        "T11.9",
        d.has("Encounter checks") && checks(d).len() == 1 && d.has("Clear the check for Day 1"),
        "Timeline's detail column holds the same block",
    );
    // V23, V28: at 960x640 the result is in view where the DM pressed Roll, on its kept line,
    // and the line under it is where `Roll` was.
    let after = below_menu(d, "Clear the check for Day 1");
    let faults = block_faults(
        d,
        Some("1 travel check rolled."),
        "Clear the check for Day 1",
    );
    let seen = d
        .nodes()
        .iter()
        .any(|node| node.name == "Undo roll" && node.fully_in_view())
        && d.visible_text("1 travel check rolled.");
    d.observe(
        "T11.9",
        faults.is_empty() && seen && before.is_some() && before == after,
        format!("Timeline: the result is in view on its kept line ({seen}), and moves no line ({before:?}, {after:?}): {faults:?}"),
    );
    d.state("travel-checks-timeline", Sizes::Default);
    d.state_prepared("travel-checks-timeline-lookup", Sizes::Small, |d| {
        if !d.has("Close Lookup") {
            d.click(NAV_LOOKUP);
        }
        d.expect("Encounter checks");
    });
    if d.has("Close Lookup") {
        d.click("Close Lookup");
    }
    // A, an unrelated B on another journey, then Undo of A, on Timeline: A's result is kept by
    // its event, so B's act leaves it.
    d.click_add_event("Add Travel");
    d.type_text("Journey B");
    d.click("More actions for Journey B");
    d.click(ROLL_FOR_ENCOUNTERS);
    d.expect("Encounter checks added to this journey.");
    d.click_starting("Travel ·");
    d.observe(
        "T11.9",
        d.has("Undo roll") && journey(d, "Travel").checks.len() == 1,
        "Timeline: another journey's act leaves this journey's Undo",
    );
    d.click("Undo roll");
    d.observe(
        "T11.9",
        journey(d, "Travel").checks.is_empty()
            && journey(d, "Journey B").checks_table.as_deref() == Some("road"),
        "Timeline: Undo roll takes back only this journey's roll",
    );
    // The same on the plan.
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click("Roll for Day 1");
    d.click("Journey B");
    d.click("Roll for This journey");
    d.click("Travel");
    d.observe(
        "T11.9",
        d.has("Undo roll") && journey(d, "Travel").checks.len() == 1,
        "Plan: another journey's act leaves this journey's Undo",
    );
    d.click("Undo roll");
    d.observe(
        "T11.9",
        journey(d, "Travel").checks.is_empty() && journey(d, "Journey B").checks.len() == 1,
        "Plan: Undo roll takes back only this journey's roll",
    );
    // Review of PR 274: the DM's own scroll ends a pending reveal. A reveal of the row after the
    // open Travel row waits for its next draw; a wheel step, then a keyboard step, come first,
    // and the plan stays where the DM put it through later draws.
    let travel_id = journey(d, "Travel").id;
    let travel_top = |d: &Driver<'_>| d.place_of("Travel").map(|rect| rect.top());
    let plan_point = d.place_of("Travel").expect("the Travel row").center();
    // The plan starts at its top, with the next row below the view.
    d.scroll_up_at(plan_point, 10);
    d.settle();
    for (step, wheel) in [("wheel", true), ("keyboard", false)] {
        let pass = d.passes() as u64;
        d.app_mut()
            .sessions
            .simulate_pending_reveal(&travel_id, pass);
        if wheel {
            // One notch, short enough that a reveal left pending would win the later draws.
            d.scroll_up_at(plan_point, 1);
        } else {
            d.press(Key::PageUp);
        }
        let moved = travel_top(d);
        d.settle();
        d.settle();
        let later = travel_top(d);
        let hidden = !d
            .nodes()
            .iter()
            .any(|node| node.name == "Journey B" && node.fully_in_view());
        d.observe(
            &format!("review-274-reveal-{step}"),
            moved.is_some() && moved == later && hidden,
            format!(
                "After the {step} step the Travel row stays at {moved:?} (later {later:?}); the next row stays out of view {hidden}"
            ),
        );
    }
    // One owner for a length being typed: leaving a page gives the journey what was typed
    // there, and a newer length typed on the other page wins, also after a restart.
    d.replace_text_in("How long", "2 days");
    d.click(NAV_TIMELINE);
    d.observe(
        "T11.9",
        journey(d, "Travel").duration == "2 days",
        "Leaving the plan gives the journey the length typed there",
    );
    d.replace_text_in("How long", "3 days");
    d.click(NAV_SESSIONS);
    d.observe(
        "T11.9",
        journey(d, "Travel").duration == "3 days",
        "Leaving Timeline gives the journey the newer length",
    );
    d.app_mut().timeline.save();
    d.restart();
    d.observe(
        "T11.9",
        journey(d, "Travel").duration == "3 days"
            && timeline_file(&dir).contains("Duration: 3 days"),
        "The newer length is the one after a restart",
    );
    road_fight_keeps_its_day(d, &dir);
}

/// W5-D1: a fight planned from the day-two check of a two-day journey is played on Day 2. A
/// fresh run begun from its plan row enters the Timeline at Day 2, 00:00, inside the journey,
/// and the journey keeps its days; `Undo: begin combat` gives the file back as it was, also
/// after a restart. When the check changed on disk before the fight began, the fight follows
/// the last event and the run says so.
fn road_fight_keeps_its_day(d: &mut Driver<'_>, dir: &Path) {
    const PLAN_SHRINE: &str = "Plan “Shrine of Cinders”";
    d.click(NAV_SESSIONS);
    show_plan(d);
    if !d.has("How long") {
        d.click("Travel");
    }
    d.replace_text_in("How long", "2 days");
    d.press(Key::Enter);
    d.type_into("Enter a roll for Day 2", "18");
    d.press(Key::Enter);
    d.click(PLAN_SHRINE);
    d.expect("“Shrine of Cinders” is in tonight’s plan.");
    let journey_before = journey(d, "Travel");
    let travel_start = |d: &Driver<'_>| {
        let events = &d.app().timeline.events;
        let at = events
            .iter()
            .position(|event| event.id == journey_before.id)?;
        crate::timeline::clock(events)[at]
    };
    d.observe(
        "W5-D1",
        travel_start(d) == Some(0) && journey_before.duration == "2 days",
        "The journey runs from Day 1, 00:00 for two days before the fight",
    );
    // A restart between Plan and the start of the fight: the plan line holds the check.
    d.app_mut().timeline.save();
    d.restart();
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.observe(
        "W5-D1",
        d.has_containing("Day 2 of Travel")
            && note_file(dir).contains("- [[Shrine of Cinders]] <!-- splat:check journey="),
        "The planned fight's row says the day it is played on, also after a restart",
    );
    d.state("travel-checks-road-day", Sizes::All);
    d.state_prepared("travel-checks-road-day-lookup", Sizes::Standard, |d| {
        if !d.has("Close Lookup") {
            d.click(NAV_LOOKUP);
        }
        assert!(d.has_containing("Day 2"), "the day beside Lookup");
    });
    if d.has("Close Lookup") {
        d.click("Close Lookup");
    }
    // A fresh run, begun from the plan row.
    let before = timeline_file(dir);
    d.click("Start fight: Shrine of Cinders");
    begin_shrine(d);
    d.app_mut().timeline.save();
    let road_fight = |d: &Driver<'_>| {
        let events = &d.app().timeline.events;
        let at = events
            .iter()
            .position(|event| event.title == "Shrine of Cinders" && event.run.is_some())?;
        Some((at, crate::timeline::clock(events)[at]))
    };
    let travel_at = |d: &Driver<'_>| {
        d.app()
            .timeline
            .events
            .iter()
            .position(|event| event.id == journey_before.id)
    };
    let placed = road_fight(d);
    d.observe(
        "W5-D1",
        placed.is_some_and(|(at, start)| {
            start == Some(24 * 60) && travel_at(d).is_some_and(|travel| at == travel + 1)
        }) && journey(d, "Travel") == journey_before
            && timeline_file(dir).contains(&format!("journey={} at=24", journey_before.id)),
        format!("The road fight starts at Day 2, 00:00, inside the journey, which keeps its days: {placed:?}"),
    );
    d.click(NAV_TIMELINE);
    d.click_starting("Shrine of Cinders ·");
    d.observe(
        "W5-D1",
        d.visible_text("Starts: Day 2, 00:00"),
        "Timeline shows the road fight on Day 2",
    );
    d.state("travel-checks-road-fight", Sizes::Default);
    d.click_starting("Travel ·");
    d.observe(
        "W5-D1",
        d.visible_text("Starts: Day 1, 00:00") && d.visible_text("Ends: Day 3, 00:00"),
        "The journey still runs from Day 1 to Day 3",
    );
    // Review of PR 284: a restart between the fight's begin and its Undo.
    d.click(BACK_TO_RUN);
    d.restart();
    d.expect("Undo: begin combat");
    d.click("Undo: begin combat");
    d.app_mut().timeline.save();
    d.observe(
        "W5-D1",
        road_fight(d).is_none() && timeline_file(dir) == before,
        "Undo: begin combat gives Timeline.md back as it was",
    );
    // Review finding 2: the check changes on disk a moment before Begin, while the DM's own
    // edit of that journey waits for its pause, so Splat has not read the file and its save
    // cannot merge. Begin reads the file: an ordinary fight, and the run says why.
    fs::write(
        dir.join("Timeline.md"),
        timeline_file(dir).replace("\"18 · A toll gang", "\"19 · A toll gang"),
    )
    .unwrap();
    let notes = &mut d
        .app_mut()
        .timeline
        .event_mut(&journey_before.id)
        .unwrap()
        .notes;
    *notes = "Fog on the second day.".into();
    d.app_mut().timeline.touch();
    d.click("Begin combat");
    let events = d.app().timeline.events.clone();
    let last = events
        .iter()
        .rposition(|event| event.title != "Shrine of Cinders");
    let placed = road_fight(d);
    d.observe(
        "W5-D1",
        d.has_containing("Its travel check changed, so this fight follows the last event.")
            && placed.is_some_and(|(at, _)| Some(at) > last)
            && events.iter().all(|event| event.within.is_none())
            && timeline_file(dir).contains("\"19 · A toll gang"),
        format!("A check changed on disk gives an ordinary fight, and the run says so: {placed:?}"),
    );
    d.click("Undo: begin combat");
    // The DM chooses the file over the waiting edit.
    d.app_mut().timeline.reload();
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.observe(
        "W5-D1",
        !d.has_containing("Day 2 of Travel"),
        "With its check changed, the planned row names no day",
    );
}

/// Shrine of Cinders' setup, as the DM plays it: the NPCs' rolls, the players' numbers, and
/// Begin combat.
fn begin_shrine(d: &mut Driver<'_>) {
    d.click("Roll NPC initiative");
    for (name, total) in [
        ("Brannoch Vale", "18"),
        ("Iselle Marrowind", "15"),
        ("Kestrel Dunmoor", "12"),
        ("Oren Thistlewick", "9"),
        ("Sable Harrowgate", "6"),
    ] {
        d.type_into(&format!("Initiative for {name}"), total);
    }
    d.click("Begin combat");
    d.expect(NEXT_TURN);
}
