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

/// Each `Clear the check for …` in view, with the vertical centre of its result's first line:
/// the words painted nearest at its left whose first line meets the Clear's height.
fn result_lines(d: &Driver<'_>) -> Vec<(f32, f32)> {
    let clears: Vec<egui::Rect> = d
        .nodes()
        .into_iter()
        .filter(|node| node.name.starts_with("Clear the check for ") && node.in_view())
        .map(|node| node.rect)
        .collect();
    clears
        .into_iter()
        .filter_map(|clear| {
            let mut nearest: Option<(f32, f32)> = None;
            d.each_painted_text(|text, clip| {
                let Some(row) = text.galley.rows.first() else {
                    return;
                };
                let first = row.rect().translate(text.pos.to_vec2());
                if clip.intersects(first)
                    && first.right() <= clear.left()
                    && first.bottom() > clear.top()
                    && first.top() < clear.bottom()
                    && nearest.is_none_or(|(left, _)| first.left() > left)
                {
                    nearest = Some((first.left(), first.center().y));
                }
            });
            nearest.map(|(_, centre)| (centre, clear.center().y))
        })
        .collect()
}

/// Faults of the open table menu (review of PR 654): its choices (the tables and `No checks`,
/// under the menu's button) inside a frame that stays in the window and keeps `S` clear of each
/// splitter and scroll bar beside it.
fn table_menu_faults(d: &Driver<'_>) -> Vec<String> {
    let nodes = d.nodes();
    let Some(button) = nodes
        .iter()
        .filter(|node| {
            node.name == "On the road"
                && node.role == egui::accesskit::Role::Button
                && node.in_view()
        })
        .map(|node| node.rect)
        .min_by(|a, b| a.top().total_cmp(&b.top()))
    else {
        return vec!["the table menu's button is not in view".to_owned()];
    };
    let Some(no_checks) = nodes
        .iter()
        .find(|node| node.name == "No checks" && node.in_view())
        .map(|node| node.rect)
    else {
        let names: Vec<_> = nodes
            .iter()
            .filter(|node| node.name.contains("check") || node.name.contains("road"))
            .map(|node| (node.name.clone(), node.role, node.in_view()))
            .collect();
        return vec![format!("the open menu has no `No checks`: {names:?}")];
    };
    // The menu's choices: `No checks` and the tables above it, under the button.
    let choices = nodes
        .iter()
        .filter(|node| {
            node.in_view()
                && node.rect.top() >= button.bottom() - 1.0
                && node.rect.bottom() <= no_checks.bottom() + 1.0
                && (node.rect.left() - no_checks.left()).abs() < crate::design::XL
        })
        .fold(no_checks, |frame, node| frame.union(node.rect));
    let frame = choices + egui::Frame::menu(&d.app_style()).total_margin();
    let window = d.window_size();
    let mut faults = Vec::new();
    if frame.left() < 0.0 || frame.right() > window.0 || frame.bottom() > window.1 {
        faults.push(format!("the menu {frame:?} leaves the window {window:?}"));
    }
    for lane in d.chrome_lanes() {
        if lane.bottom() <= frame.top() || lane.top() >= frame.bottom() {
            continue;
        }
        let clear = (lane.left() - frame.right()).max(frame.left() - lane.right());
        if clear < crate::design::S - 0.5 {
            faults.push(format!(
                "the menu {frame:?} is {clear:.1} px from the lane {lane:?}"
            ));
        }
    }
    faults
}

/// Faults in the open block's layout (V4, V22, V23): `Encounter checks` is a label above the
/// table menu, whose word starts at the label's edge; the dice words share the menu's line, or start their own line with their words;
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
    // The menu starts its line, so its word starts at the label's text edge (`line_start`, SD1);
    // its padding hangs left of it.
    let words = |name: &str, at: egui::Rect| d.painted_rect_in(name, at.expand(1.0));
    let edges = words("Encounter checks", label).zip(words("On the road", menu));
    if label.bottom() > menu.top()
        || !edges.is_some_and(|(label, word)| (label.left() - word.left()).abs() <= 1.0)
    {
        faults.push(format!(
            "the label {label:?} is not above the menu {menu:?} with its word at the label's \
             edge: {edges:?}"
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
    let faults = block_faults(
        d,
        Some("2 travel checks rolled."),
        "Clear the check for Day 1",
    );
    // Layout 11c (M0547): each result's words have their first line centred on its Clear.
    for size in [SMALL, DEFAULT, LARGE] {
        let lines = d.at_size(size, result_lines);
        d.observe(
            "m32-L11c-result-line",
            lines.len() >= 2
                && lines
                    .iter()
                    .all(|(words, clear)| (words - clear).abs() <= 1.0),
            format!("At {size:?}, each result's first line and its Clear, by centre: {lines:?}"),
        );
    }
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
    d.click_starting("Undo roll ");
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
    let faults = block_faults(
        d,
        Some("1 travel check rolled."),
        "Clear the check for Day 1",
    );
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
    let in_plan = d
        .nodes()
        .into_iter()
        .find(|node| node.name.contains("In the plan"))
        .and_then(|node| {
            Some(
                node.rect
                    .translate(-d.place_of("Encounter checks")?.min.to_vec2()),
            )
        });
    let after = below_checks(d);
    let message = from_label(d, "“Bridge Ambush” is in tonight’s plan.");
    d.observe(
        "T11.10",
        below.iter().all(Option::is_some)
            && after == below
            && d.has("Undo add plan line")
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
        d.scroll_until(at, "Undo add plan line", 8);
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
        note_file(&dir) == planned_note && d.has("Undo add plan line"),
        "Undo plan line stays through a visit to another page",
    );
    // Act B, unrelated: clear the first day's result. Then Undo of A takes only A's line.
    d.click("Clear the check for Day 1");
    d.expect("Check cleared.");
    d.click("Undo add plan line");
    d.observe(
        "T11.10",
        !note_file(&dir).contains("- [[Bridge Ambush]]")
            && d.has(PLAN_AMBUSH)
            && checks(d).len() == 1,
        "Undo plan line takes back the plan line and leaves the cleared check",
    );
    d.click_starting("Undo clear ");
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
        d.has_containing("In the plan") && !d.has("Undo add plan line") && after == below,
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
    // Review of PR 654: the open table menu is captured at each size, and its frame stays in
    // the window and keeps `S` clear of each splitter and scroll bar beside it.
    d.state_prepared("travel-checks-table-menu", Sizes::All, |d| {
        // A resize closes an open menu: it opens again at each size, then settles (its first
        // pass draws no choices).
        if !d
            .nodes()
            .iter()
            .any(|node| node.name == "No checks" && node.in_view())
        {
            d.click("On the road");
        }
        d.settle();
        let size = d.window_size();
        let mut faults = table_menu_faults(d);
        // M-T0.7: the reason under No checks starts at the choice's painted words (±1 px), below
        // them, and its words are not justified.
        let (choice, note) = (
            super::painted_bounds(d, "No checks"),
            super::painted_bounds(d, "Clear the results first."),
        );
        if !choice.zip(note).is_some_and(|(choice, note)| {
            (note.left() - choice.left()).abs() <= 1.0 && note.top() >= choice.bottom()
        }) {
            faults.push(format!(
                "the reason {note:?} does not start at No checks' words {choice:?}"
            ));
        }
        d.each_painted_text(|text, _| {
            if text.galley.job.text.starts_with("Clear the results") && text.galley.job.justify {
                faults.push(format!(
                    "the reason's words are justified: {:?}",
                    text.galley.rect
                ));
            }
        });
        d.observe(
            "s654-table-menu-clear",
            faults.is_empty(),
            format!("At {size:?}, the open table menu: {faults:?}"),
        );
    });
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
        .any(|node| node.name.starts_with("Undo roll ") && node.fully_in_view())
        && d.visible_text("1 travel check rolled.");
    d.observe(
        "T11.9",
        faults.is_empty() && seen && before.is_some() && before == after,
        format!("Timeline: the result is in view on its kept line ({seen}), and moves no line ({before:?}, {after:?}): {faults:?}"),
    );
    // Round 29 G142, final review 4a: after the roll, the result and the plan line under it
    // are in view at 960 px (the detail scrolls least to show them).
    let line_in_view = d
        .nodes()
        .iter()
        .any(|node| node.name.contains("In the plan") && node.fully_in_view());
    d.observe(
        "iter-29-S1-G142-reveal",
        line_in_view,
        "Timeline at 960: the rolled result's plan line is in view after the roll",
    );
    // Row 123 (iter-29 G142): a rolled result on Timeline offers its fight to the plan of the
    // journey's session, as on the plan. Bridge Ambush is in that plan already.
    d.observe(
        "iter-29-S1-G142",
        d.has_containing("In the plan") && !d.has(PLAN_AMBUSH),
        "Timeline: the rolled Bridge Ambush says In the plan, as the plan of its session does",
    );
    // Point 4: frames with nothing changed copy no names and read no plan.
    let counts = || {
        (
            crate::timeline::page::PLAN_NAMES_MADE.with(std::cell::Cell::get),
            crate::timeline::page::JOURNEY_PLAN_READS.with(std::cell::Cell::get),
        )
    };
    let before = counts();
    for _ in 0..3 {
        d.settle();
    }
    let after = counts();
    d.observe(
        "iter-29-S1-G142",
        before == after,
        format!("Unchanged frames copy no names and read no plan: {before:?}, {after:?}"),
    );
    // Final review 2, 4b: at each size the rolled result's plan line is in view.
    let seen = [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            d.nodes()
                .iter()
                .any(|node| node.name.contains("In the plan") && node.fully_in_view())
        })
    });
    d.observe(
        "iter-29-S1-G142-reveal",
        seen.iter().all(|seen| *seen),
        format!("Timeline: In the plan is in view at 960, 1200 and 1920: {seen:?}"),
    );
    // The same, at each capture as it is taken.
    d.state_prepared("travel-checks-timeline", Sizes::All, |d| {
        // The capture's resize puts the form back at its top; the scenario scrolls to the line
        // (the reveal at the roll is proved above).
        show_plan_line(d);
        let seen = d
            .nodes()
            .iter()
            .any(|node| node.name.contains("In the plan") && node.fully_in_view());
        d.observe(
            "iter-29-S1-G142-reveal",
            seen,
            "The capture of state 06 shows In the plan whole",
        );
    });
    d.state_prepared("travel-checks-timeline-lookup", Sizes::Small, |d| {
        if !d.has("Close Lookup") {
            d.click(NAV_LOOKUP);
        }
        d.expect("Encounter checks");
        // Final review 2, 4d: beside Lookup the form is narrower; the scenario scrolls to the
        // plan line so the capture shows it whole.
        show_plan_line(d);
        let seen = d
            .nodes()
            .iter()
            .any(|node| node.name.contains("In the plan") && node.fully_in_view());
        d.observe(
            "iter-29-S1-G142-reveal",
            seen,
            "Beside Lookup, the capture shows In the plan whole",
        );
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
        d.has_containing("Undo roll ") && journey(d, "Travel").checks.len() == 1,
        "Timeline: another journey's act leaves this journey's Undo",
    );
    d.click_starting("Undo roll ");
    d.observe(
        "T11.9",
        journey(d, "Travel").checks.is_empty()
            && journey(d, "Journey B").checks_table.as_deref() == Some("road"),
        "Timeline: Undo roll takes back only this journey's roll",
    );
    timeline_plans_a_result(d);
    field_history_belongs_to_its_event(d);
    // The same on the plan.
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click("Roll for Day 1");
    d.click("Journey B");
    d.click("Roll for This journey");
    d.click("Travel");
    d.observe(
        "T11.9",
        d.has_containing("Undo roll ") && journey(d, "Travel").checks.len() == 1,
        "Plan: another journey's act leaves this journey's Undo",
    );
    d.click_starting("Undo roll ");
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
        if !wheel {
            // Sol 2-011 (review of #852): a key ends the plan's reveal when it goes to the plan's
            // column. The DM's pointer rests on the plan, with nothing focused, before the ask,
            // so the key's frame is the first after it.
            d.hover_over(plan_point);
        }
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
    // Review of #852 (`m-p0-row-reveal-beside`): a scroll over Lookup is input to Lookup's column,
    // not to the plan's (`design::reveal::dm_scrolls`), so a pending reveal of the plan lands.
    if !d.has("Close Lookup") {
        d.click(NAV_LOOKUP);
    }
    let plan_point = d.place_of("Travel").expect("the Travel row").center();
    d.scroll_up_at(plan_point, 10);
    d.settle();
    let lookup_point =
        d.place_of("Close Lookup").expect("Lookup is open").center() + egui::vec2(0.0, 160.0);
    // The pointer rests over Lookup first, so the wheel's frame is the first after the ask.
    d.hover_over(lookup_point);
    let pass = d.passes() as u64;
    d.app_mut()
        .sessions
        .simulate_pending_reveal(&travel_id, pass);
    d.wheel_here(1);
    d.settle();
    let shown = d
        .nodes()
        .iter()
        .any(|node| node.name == "Journey B" && node.fully_in_view());
    d.observe(
        "m-p0-row-reveal-beside",
        shown,
        format!("A scroll over Lookup leaves the plan's reveal: the next row in view {shown}"),
    );
    d.click("Close Lookup");
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

/// Review of PR 313: egui keeps a field's cursor and text Undo under its id, so each event's
/// **How long** and **Notes** have an id of their own. Edit journey A, open journey B, focus its
/// field, press Ctrl+Z: B never gets A's words, and A keeps what it had.
/// Row 123 (iter-29 G142): on Timeline, `Plan “<name>”` under a result writes the plan line into
/// the note of the journey's session; its result and Undo are in the list's footer, and the
/// offer becomes `In the plan`. Undo there takes the line back. The journey ends with no check.
fn timeline_plans_a_result(d: &mut Driver<'_>) {
    const PLAN_SHRINE: &str = "Plan “Shrine of Cinders”";
    const PLANNED: &str = "“Shrine of Cinders” is in tonight’s plan.";
    let dir = folder(d);
    d.click_starting("Travel ·");
    d.type_into("Enter a roll for Day 1", "19");
    d.press(Key::Enter);
    d.expect("1 travel check rolled.");
    d.observe(
        "iter-29-S1-G142",
        d.has(PLAN_SHRINE) && !note_file(&dir).contains("[[Shrine of Cinders]]"),
        "Timeline: a result that links a fight not in the plan offers it",
    );
    // Final review 2, 4c: no field or row moves when the result comes.
    let places = |d: &Driver<'_>| {
        (
            d.place_of("Title"),
            d.place_of(&d.name_starting("Travel ·")),
            d.place_of("Clear the check for Day 1"),
        )
    };
    let unmoved = places(d);
    d.click(PLAN_SHRINE);
    let moved = places(d);
    d.observe(
        "iter-29-S1-G142-planned",
        unmoved.0.is_some() && unmoved.1.is_some() && unmoved == moved,
        format!("The result moves no field or row: {unmoved:?}, {moved:?}"),
    );
    let whole = [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            let nodes = d.nodes();
            let whole = |name: &str| {
                nodes
                    .iter()
                    .any(|node| node.name == name && node.fully_in_view())
            };
            whole(PLANNED) && whole("Undo add plan line")
        })
    });
    d.observe(
        "iter-29-S1-G142-planned",
        whole.iter().all(|whole| *whole),
        format!("The footer's result and its Undo are whole at each size: {whole:?}"),
    );
    let note = note_file(&dir);
    d.observe(
        "iter-29-S1-G142",
        note.contains("- [[Shrine of Cinders]]")
            && d.has(PLANNED)
            && d.has("Undo add plan line")
            && d.has_containing("In the plan")
            && !d.has(PLAN_SHRINE),
        format!("Timeline: the press writes the plan line, and the footer says so: {note}"),
    );
    // Final review point 4: with the result in the footer, quiet frames check no note words,
    // copy no names and read no plan.
    let counts = || {
        (
            crate::timeline::page::PLAN_LINE_CHECKS.with(std::cell::Cell::get),
            crate::timeline::page::PLAN_NAMES_MADE.with(std::cell::Cell::get),
            crate::timeline::page::JOURNEY_PLAN_READS.with(std::cell::Cell::get),
        )
    };
    let before = counts();
    for _ in 0..3 {
        d.settle();
    }
    let after = counts();
    d.observe(
        "iter-29-S1-G142",
        before == after && d.has(PLANNED),
        format!("Quiet frames after a Plan check nothing again: {before:?}, {after:?}"),
    );
    d.click("Undo add plan line");
    d.observe(
        "iter-29-S1-G142",
        !note_file(&dir).contains("[[Shrine of Cinders]]")
            && d.has(PLAN_SHRINE)
            && !d.has(PLANNED)
            && journey(d, "Travel").checks.len() == 1,
        "Timeline: Undo add plan line takes back only the line; the result stays",
    );
    d.click("Clear the check for Day 1");
    d.observe(
        "iter-29-S1-G142",
        journey(d, "Travel").checks.is_empty(),
        "The journey has no check again",
    );
}

fn field_history_belongs_to_its_event(d: &mut Driver<'_>) {
    const A_LENGTH: &str = "5 days";
    const A_NOTES: &str = "Wolves on the road.";
    d.click_starting("Travel ·");
    let first = journey(d, "Travel");
    d.replace_text_in("How long", A_LENGTH);
    // Moving to Notes ends the length's draft, so journey A holds it.
    d.type_into("Notes on this event", A_NOTES);
    d.click_starting("Journey B ·");
    d.observe(
        "pr-313-field-owner",
        d.has("How long") && journey(d, "Journey B").duration.is_empty(),
        "Journey B is open with no length of its own",
    );
    d.type_into("How long", "");
    d.press_ctrl(Key::Z);
    let length = d.field_text("How long");
    d.observe(
        "pr-313-field-owner",
        length.as_deref() == Some(""),
        format!("Ctrl+Z in journey B's length gives it no words of journey A's: {length:?}"),
    );
    d.type_into("Notes on this event", "");
    d.press_ctrl(Key::Z);
    let notes = d.editor_text();
    d.observe(
        "pr-313-field-owner",
        notes.as_deref().unwrap_or_default().is_empty(),
        format!("Ctrl+Z in journey B's notes gives it no words of journey A's: {notes:?}"),
    );
    // Leaving B ends any draft it holds: what each journey holds is now settled.
    d.click_starting("Travel ·");
    let (a, b) = (journey(d, "Travel"), journey(d, "Journey B"));
    d.observe(
        "pr-313-field-owner",
        a.duration == A_LENGTH && a.notes == A_NOTES && b.duration.is_empty() && b.notes.is_empty(),
        format!(
            "A keeps {:?} and {:?}; B holds {:?} and {:?}",
            a.duration, a.notes, b.duration, b.notes
        ),
    );
    // Back to the journey's first state, for the steps after this one.
    d.replace_text_in("How long", &first.duration);
    d.replace_text_in("Notes on this event", &first.notes);
    d.click_starting("Journey B ·");
    d.click_starting("Travel ·");
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
    // Round 27 FE10: the fight's Timeline row says its day of the journey, in the plan row's
    // words; Must stay true: the journey's own row does not say it.
    let row = d.name_starting("Shrine of Cinders ·");
    let travel_row = d.name_starting("Travel ·");
    d.observe(
        "iter-27-S-FE10",
        row.ends_with(" · Day 2 of Travel")
            && travel_row.starts_with("Travel ·")
            && !travel_row.contains("of Travel"),
        format!(
            "The road fight's row says Day 2 of Travel: {row:?}; the journey's: {travel_row:?}"
        ),
    );
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

/// Round 29 G142, final review 2: the plan act on Timeline, in a scenario of its own so that no
/// state of `travel-checks` is renumbered. A journey's rolled result offers its fight; the press
/// puts the result and `Undo add plan line` in the list's footer (state `plan-on-timeline`, all
/// sizes). Then Timeline's own footer press of an Undo that fails, for a journey that is not
/// open: the line stays, the footer says why (state `plan-undo-failed`), and the fault ends with
/// its journey. The suite runs in a namespace where a folder's permission bits do not stop a
/// save, so the Undo fails the way a changed note makes it fail: the note changes outside Splat
/// just before the press (`hold_disk_check` holds every disk check until
/// `release_disk_check` after the press, whatever the time between frames, so no poll reads the
/// change first).
pub fn plan_act(d: &mut Driver<'_>) {
    const FORD: &str = "Ford crossing";
    const PLAN_SHRINE: &str = "Plan “Shrine of Cinders”";
    const PLANNED: &str = "“Shrine of Cinders” is in tonight’s plan.";
    const OUTSIDE: &str = "Written in Obsidian.";
    const REFUSED: &str = "The plan line for Shrine of Cinders was not removed from Session 1: \
        The Crossing. The session note changed on disk. Open that session’s notes and remove the \
        plan line there.";
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
    d.click(NAV_TIMELINE);
    d.click_add_event("Add Travel");
    d.type_text(FORD);
    d.click(&format!("More actions for {FORD}"));
    d.click(ROLL_FOR_ENCOUNTERS);
    d.expect("Encounter checks added to this journey.");
    if d.has("Choose a table") {
        d.click("Choose a table");
        d.click("On the road");
    }
    d.type_into("Enter a roll for This journey", "19");
    d.press(Key::Enter);
    d.expect(PLAN_SHRINE);
    d.click(PLAN_SHRINE);
    d.expect(PLANNED);
    let whole = [SMALL, DEFAULT, LARGE].map(|size| {
        d.at_size(size, |d| {
            let nodes = d.nodes();
            let whole = |name: &str| {
                nodes
                    .iter()
                    .any(|node| node.name == name && node.fully_in_view())
            };
            whole(PLANNED)
                && whole("Undo add plan line")
                && nodes
                    .iter()
                    .any(|node| node.name.contains("In the plan") && node.fully_in_view())
        })
    });
    d.observe(
        "iter-29-S1-G142-planned",
        whole.iter().all(|whole| *whole) && note_file(&dir).contains("- [[Shrine of Cinders]]"),
        format!("The footer's result, its Undo and In the plan are whole at each size: {whole:?}"),
    );
    d.state_prepared("plan-on-timeline", Sizes::All, |d| {
        // The capture's resize puts the form back at its top; the scenario scrolls to the line.
        show_plan_line(d);
        let nodes = d.nodes();
        let whole = |name: &str| {
            nodes
                .iter()
                .any(|node| node.name == name && node.fully_in_view())
        };
        let shown = whole(PLANNED)
            && whole("Undo add plan line")
            && nodes
                .iter()
                .any(|node| node.name.contains("In the plan") && node.fully_in_view());
        d.observe(
            "iter-29-S1-G142-planned",
            shown,
            "The capture after the press shows the result, its Undo and In the plan whole",
        );
    });
    let note_path = fs::read_dir(&dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|file| file.path())
        .find(|path| {
            fs::read_to_string(path).is_ok_and(|text| text.contains("## Plan for tonight"))
        })
        .expect("the session note");
    let ford = journey(d, FORD).id;
    // The journey closes: another event opens.
    d.click_add_event("Add Day");
    d.app_mut().sessions.hold_disk_check();
    fs::write(&note_path, format!("{}{OUTSIDE}\n", note_file(&dir))).unwrap();
    d.click("Undo add plan line");
    d.app_mut().sessions.release_disk_check();
    d.settle();
    // Structural phase, M1: the refusal is the footer's first message, whole, in the warning
    // tone, naming the fight, the session, the cause and the next step; no older success
    // stands above it (the result's words are gone while its Undo waits).
    let refusal_first = |d: &Driver<'_>| {
        let nodes = d.nodes();
        let node = nodes
            .iter()
            .find(|node| node.name == REFUSED && node.fully_in_view());
        let footer: Vec<_> = d
            .typed_messages()
            .into_iter()
            .filter(|(_, place, _)| *place == crate::design::outcome::MessagePlace::Footer)
            .collect();
        let message = node.and_then(|node| {
            footer
                .iter()
                .find(|(rect, ..)| rect.contains(node.rect.center()))
                .copied()
        });
        let first = message.is_some_and(|(rect, _, tone)| {
            tone == crate::design::Tone::Warning
                && !footer.iter().any(|(other, ..)| {
                    other.top() < rect.top() - 0.5 && other.x_range().intersects(rect.x_range())
                })
        });
        let older = nodes.iter().any(|node| node.name.contains(PLANNED));
        (
            first && !older,
            format!(
                "refusal {:?}, its message {message:?}, footer {footer:?}, older success {older}",
                node.map(|node| node.rect)
            ),
        )
    };
    let (first, why) = refusal_first(d);
    d.observe(
        "iter-29-S1-G142-undo-fault",
        first && note_file(&dir).contains("[[Shrine of Cinders]]"),
        format!("Timeline's footer opens with the whole refusal, and the line stays: {why}"),
    );
    d.state_prepared("plan-undo-failed", Sizes::All, |d| {
        let (first, why) = refusal_first(d);
        d.observe(
            "s30-B1-refusal-first",
            first,
            format!("The capture opens its footer with the whole refusal: {why}"),
        );
    });
    // Final review 3, point 1: a block taller than the view. A check near the view's bottom is
    // rolled: its result's plan line comes whole into view (the reveal is the check's row, not
    // the block).
    const LONG: &str = "Long road";
    d.set_base(SMALL);
    d.click_add_event("Add Travel");
    d.type_text(LONG);
    d.click(&format!("More actions for {LONG}"));
    d.click(ROLL_FOR_ENCOUNTERS);
    d.expect("Encounter checks added to this journey.");
    if d.has("Choose a table") {
        d.click("Choose a table");
        d.click("On the road");
    }
    d.replace_text_in("How long", "20 days");
    d.press(Key::Enter);
    d.type_into("Enter a roll for Day 1", "5");
    d.press(Key::Enter);
    // A point low in the detail column's scroll body, wherever a roll left the block.
    let column = egui::pos2(
        d.place_of("Title").expect("the form").center().x,
        SMALL.1 - 80.0,
    );
    d.scroll_until(column, "Enter a roll for Day 15", 40);
    // Day 15's line at the view's lower edge, so its result and offer reach below it, and
    // the block reaches above and below the view.
    let field = d
        .place_of("Enter a roll for Day 15")
        .expect("Day 15's field");
    d.scroll_points_at(column, field.bottom() - (SMALL.1 - 10.0));
    d.type_into("Enter a roll for Day 15", "19");
    d.press(Key::Enter);
    let offer = d
        .nodes()
        .into_iter()
        .find(|node| node.name.contains("In the plan") || node.name == PLAN_SHRINE);
    d.observe(
        "iter-29-S1-G142-reveal-long",
        offer.as_ref().is_some_and(|node| node.fully_in_view()),
        format!(
            "A long block: the rolled check's plan line is in view: {:?}",
            offer
                .as_ref()
                .map(|node| (node.name.clone(), node.rect, node.fully_in_view()))
        ),
    );
    // Final review 3, point 2: a reveal left pending by journey A (Ford crossing) when the DM
    // leaves it before its next draw is A's alone: journey B (Long road) opens where it opens.
    // B has a result on Day 20, below its opening view, at the offset A's reveal names.
    d.scroll_until(column, "Enter a roll for Day 20", 40);
    d.type_into("Enter a roll for Day 20", "3");
    d.press(Key::Enter);
    let long = format!("{LONG} ·");
    d.click(NAV_LOOKUP);
    d.click("All events");
    d.click_starting(&long);
    // A place inside the detail's scroll body (Title is above it).
    let opening = d.place_of("Encounter checks");
    d.click("All events");
    d.app_mut()
        .timeline_page
        .simulate_pending_reveal(&ford, 19 * 24);
    d.click_starting(&long);
    let after = d.place_of("Encounter checks");
    d.observe(
        "iter-29-S1-G142-reveal-owner",
        opening.is_some() && opening == after,
        format!("Journey B keeps its opening place: {opening:?}, {after:?}"),
    );
    d.click("Close Lookup");
    // Ruling R42: a later act on another object leads with its own line; A's answer goes down
    // and stays. (Here, after the long road, so no day of it is renumbered.)
    d.click_add_event("Add Day");
    let pushed = |d: &Driver<'_>| {
        let nodes = d.nodes();
        let refusal = nodes
            .iter()
            .find(|node| node.name == REFUSED)
            .map(|node| node.rect);
        let mut footer: Vec<_> = d
            .typed_messages()
            .into_iter()
            .filter(|(rect, place, _)| {
                *place == crate::design::outcome::MessagePlace::Footer
                    && refusal.is_some_and(|words| rect.x_range().intersects(words.x_range()))
            })
            .collect();
        footer.sort_by(|a, b| a.0.top().total_cmp(&b.0.top()));
        let first = footer.first().copied();
        (
            first.is_some_and(|(rect, _, tone)| {
                tone == crate::design::Tone::Success
                    && refusal.is_some_and(|words| !rect.contains(words.center()))
            }),
            format!("first {first:?}, refusal {refusal:?}, footer {footer:?}"),
        )
    };
    let (down, why) = pushed(d);
    d.observe(
        "s30-B1-refusal-pushed-down",
        down,
        format!("Add Day's result leads; A's refusal stays below it: {why}"),
    );
    d.state_prepared("plan-answer-pushed-down", Sizes::Default, |d| {
        let (down, why) = pushed(d);
        d.observe(
            "s30-B1-refusal-pushed-down",
            down,
            format!("The capture shows Add Day's result first, the refusal below: {why}"),
        );
    });
    // M1's second reader: the session's own plan shows the same stored answer, once, whole, in
    // its tone; its Undo waits while the note is not as the act left it, so none is drawn.
    d.click(NAV_SESSIONS);
    show_plan(d);
    let in_plan = |d: &Driver<'_>| {
        let nodes = d.nodes();
        // The message, not the footer's group: the group is named with every message's words,
        // so with one message it has the same name, around the message's rect.
        let named: Vec<_> = nodes.iter().filter(|node| node.name == REFUSED).collect();
        let shown: Vec<_> = named
            .iter()
            .copied()
            .filter(|node| {
                !named
                    .iter()
                    .any(|inner| inner.rect != node.rect && node.rect.contains_rect(inner.rect))
            })
            .collect();
        let whole = shown.len() == 1 && shown[0].fully_in_view();
        let tone = shown.first().and_then(|node| {
            d.typed_messages()
                .into_iter()
                .find(|(rect, place, _)| {
                    *place == crate::design::outcome::MessagePlace::Footer
                        && rect.contains(node.rect.center())
                })
                .map(|(_, _, tone)| tone)
        });
        let undo = nodes
            .iter()
            .any(|node| node.name.starts_with("Undo add plan line"));
        (
            whole && tone == Some(crate::design::Tone::Warning) && !undo,
            format!(
                "refusals {:?}, tone {tone:?}, an Undo drawn {undo}",
                shown
                    .iter()
                    .map(|node| (node.rect, node.fully_in_view()))
                    .collect::<Vec<_>>()
            ),
        )
    };
    d.state_prepared("plan-answer-in-session-plan", Sizes::All, |d| {
        show_plan(d);
        let (once, why) = in_plan(d);
        d.observe(
            "s30-B1-refusal-in-session-plan",
            once,
            format!("The session's plan paints the refusal once, whole, warning, no Undo: {why}"),
        );
    });
    d.click(NAV_TIMELINE);
    d.observe(
        "s30-B1-refusal-stays",
        d.has_containing("was not removed from Session 1"),
        "Other acts on other objects never ended A's answer",
    );
    d.app_mut().timeline.remove(&ford);
    d.settle();
    d.observe(
        "iter-29-S1-G142-undo-fault",
        !d.has_containing("was not removed from Session 1"),
        "The fault ends with its journey",
    );
}

/// Before a capture: the open journey's `In the plan` line whole in view. A capture's resize puts
/// the form back at its top, and beside Lookup the form is narrower; the scenario scrolls the
/// form, a few points at a time, until the line is whole. The reveal at the roll is proved by
/// its own checks.
fn show_plan_line(d: &mut Driver<'_>) {
    let whole = |d: &Driver<'_>| {
        d.nodes()
            .iter()
            .any(|node| node.name.contains("In the plan") && node.fully_in_view())
    };
    let Some(block) = d.place_of("Encounter checks") else {
        return;
    };
    d.scroll_until(block.center(), "In the plan", 6);
    for _ in 0..8 {
        if whole(d) {
            return;
        }
        d.scroll_points_at(block.center(), 24.0);
    }
}
