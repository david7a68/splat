//! The journeys J2, J3, J4 and J6 of the workflow score, played on the real app and measured
//! as the audit's evening is (J1, the scenario `dm-audit`): each act is counted by its kind,
//! and the driver sees each page change. Each is one scenario, `journey-j2` and so on, so its
//! window, its campaign and its dice are its own, and a failure names its journey.
//!
//! Each journey walks section 2 of `docs/design/session-cycle.md` step by step, in tasks in
//! the DM's words. A task that the design does not count (a fight's set-up, its turns, its
//! end) is measured too, as the app costs it today. `artifacts/dm-audit/README.md` puts each
//! measured task beside the design's count.
//!
//! G6 holds them as it holds J1: each journey's `counts.json` under
//! `artifacts/dm-audit/journeys/`, compared in every gate by `measure`.
//! `devenv shell -- audit-freeze journeys` writes every row of the four; a new task goes in with
//! `devenv shell -- audit-freeze add`, which refuses when a frozen task changed.
use super::*;
use crate::visual_tests::scenarios::trackers::write_the_race;

/// The command that writes every row of the four journeys again.
const JOURNEYS_COMMAND: &str = "devenv shell -- audit-freeze journeys";

/// Play one journey and hold it to its frozen files.
fn journey(d: &mut Driver<'_>, name: &'static str, walk: fn(&mut Audit<'_, '_>)) {
    let record = play(d, walk);
    let files = FrozenFiles {
        what: name,
        folder: format!("artifacts/dm-audit/journeys/{name}"),
        counts_command: JOURNEYS_COMMAND,
    };
    measure(d, &files, &record);
}

pub fn j2(d: &mut Driver<'_>) {
    journey(d, "J2", improvised_evening);
}

pub fn j3(d: &mut Driver<'_>) {
    journey(d, "J3", broken_evening);
}

pub fn j4(d: &mut Driver<'_>) {
    journey(d, "J4", the_race);
}

pub fn j6(d: &mut Driver<'_>) {
    journey(d, "J6", week_between);
}

/// J2's planned NPC: a World NPC whose stat block is the fixture's creature Captain Varra.
const OSTREL: &str = "Warden Ostrel";
const WOLVES: &str = "Wolf Pack";
const SHRINE: &str = "Shrine of Cinders";
const WATCH: &str = "Ember Watch";
const GATE: &str = "Gate Watch";
const PATROL: &str = "Ford Patrol";
const VERORI: &str = "Verori";
/// How a Log line links tonight's session.
const TONIGHT_LINK: &str = "[[Session 2 - The Crossing]]";
/// The dock's `Effect`, by its accessible name.
const EFFECT: &str = "Effect: words for this row";

fn campaign_root(d: &mut Driver<'_>) -> PathBuf {
    d.app_mut()
        .campaigns
        .active_campaign()
        .expect("a campaign")
        .path
}

/// A new encounter, as the DM made it in prep: a copy of the fixture's `from`, roster and all,
/// under its own name and ID, with these words at the end of its note.
fn encounter(d: &mut Driver<'_>, from: &str, name: &str, words: &str) {
    let root = campaign_root(d);
    let text = fs::read_to_string(root.join(format!("Encounters/{from}.md")))
        .expect("read the fixture's encounter");
    let old = splat::character::stable_id(&format!("visual:encounter:{from}"));
    let new = splat::character::stable_id(&format!("visual:encounter:{name}"));
    let text = text.replace(&old.to_string(), &new.to_string()).replace(
        &format!("splat_name: {from}"),
        &format!("splat_name: {name}"),
    );
    fs::write(
        root.join(format!("Encounters/{name}.md")),
        format!("{}\n\n{words}\n", text.trim_end()),
    )
    .expect("write the encounter");
}

/// A World NPC of the campaign, as the DM wrote one, with these extra fields of `splat_world`.
fn npc(d: &mut Driver<'_>, name: &str, fields: &str, words: &str) {
    let path = format!("NPCs/{name}.md");
    let text = splat::content::formats::fixture(
        "World.md",
        &format!(
            "splat_id: {}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n  type: npc\n  status: alive\n{fields}",
            splat::character::stable_id(&path)
        ),
        &format!("\n# {name}\n\n{words}\n"),
    )
    .expect("an NPC note");
    let file = campaign_root(d).join(path);
    fs::create_dir_all(file.parent().expect("a folder")).expect("make the NPC folder");
    fs::write(file, text).expect("write the NPC");
}

/// The campaign's marked road table: `[[Wolf Pack]]` on a d20 of `wolves` or less, and nothing
/// above it. The journey's dice come from its scenario's name, so its rolls are the same on
/// every run.
fn road_table(d: &mut Driver<'_>, wolves: u32) {
    let table = format!(
        "# Road\n\n## On the road\n\n<!-- splat:rolls key=road dice=1d20 per=day -->\n\n| d20 | On the road |\n| --- | --- |\n| 1-{wolves} | [[{WOLVES}]] picks up the trail. |\n| {}-20 | Nothing stirs. |\n",
        wolves + 1
    );
    fs::write(campaign_root(d).join("Road.md"), table).expect("write the road table");
}

/// When the plan's rows do not fit, the scenes fold to `N scenes`; the DM opens the fold to
/// reach this scene's row.
fn unfold_scenes(a: &mut Audit<'_, '_>, scene: &str) {
    if a.d.has(scene) {
        return;
    }
    let fold = a.d.nodes().into_iter().find(|node| {
        node.name.split_once(' ').is_some_and(|(n, word)| {
            n.parse::<usize>().is_ok() && ["scene", "scenes"].contains(&word)
        })
    });
    if let Some(fold) = fold {
        a.click(Find, &fold.name);
    }
}

/// Tonight's note, as the DM wrote it in prep: these lines under `## Plan for tonight`.
fn tonight(d: &mut Driver<'_>, number: u32, title: &str, plan: &str) {
    d.click(NAV_SESSIONS);
    d.click("New session");
    d.type_into("Session title", title);
    d.click("Create session");
    d.expect(&format!("Session {number}: {title}"));
    let body = d.editor_text().expect("tonight's note");
    d.replace_editor_text(&body.replacen(
        "## Plan for tonight\n",
        &format!("## Plan for tonight\n\n{plan}"),
        1,
    ));
    d.press_ctrl(Key::S);
    show_plan(d);
}

/// A whole fight played before the journey starts: rolled, begun and ended, its run closed.
fn played_fight(d: &mut Driver<'_>, name: &str) {
    d.click(&format!("Start fight: {name}"));
    roll_and_begin(d);
    end_fight(d);
    close_run(d);
}

fn roll_and_begin(d: &mut Driver<'_>) {
    d.click("Roll NPC initiative");
    for (name, total) in PLAYERS {
        d.type_into(&format!("Initiative for {name}"), total);
    }
    d.click("Begin combat");
}

fn end_fight(d: &mut Driver<'_>) {
    if !d.has("End encounter…") && d.has("Close menu") {
        d.click("Close menu");
    }
    d.click("End encounter…");
    d.click("End encounter");
}

/// What a file of the campaign holds now, read after the act. A file that cannot be read fails
/// the journey: an outcome is never checked against nothing.
fn file(d: &Driver<'_>, path: &str) -> String {
    let path = d.root.join("campaigns/emberfall").join(path);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// The campaign's `Timeline.md` as saved, after the save that the real app makes on its own.
fn timeline_file(d: &mut Driver<'_>) -> String {
    d.app_mut().timeline.save();
    d.settle();
    let campaign = d.app_mut().campaigns.active_campaign().expect("a campaign");
    let path = crate::sessions::folder(&campaign)
        .expect("the campaign's session folder")
        .join("Timeline.md");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// Each event of the saved `Timeline.md` with the day it falls on, as the app counts days.
fn saved_days(d: &mut Driver<'_>) -> Vec<(crate::timeline::Event, u32)> {
    let events = crate::timeline::parse(&timeline_file(d)).expect("Timeline.md reads");
    let days = crate::timeline::day_numbers(&events);
    events.into_iter().zip(days).collect()
}

/// The saved event with this title, and its day.
fn saved_event(d: &mut Driver<'_>, title: &str) -> Option<(crate::timeline::Event, u32)> {
    saved_days(d)
        .into_iter()
        .find(|(event, _)| event.title == title)
}

/// The open session note's text on disk, as it is: nothing saved first.
fn note_on_disk(d: &Driver<'_>) -> String {
    let path = d
        .app()
        .sessions
        .selected_path()
        .expect("an open session note");
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// The open session note on disk, after the save that the real app makes on its own after a
/// pause. Test builds save only when told to.
fn saved_note(d: &mut Driver<'_>) -> String {
    d.app_mut().sessions.flush();
    d.settle();
    note_on_disk(d)
}

/// The words under `## heading` of a note, to the next heading of that level.
fn section<'t>(text: &'t str, heading: &str) -> &'t str {
    let start = format!("\n## {heading}\n");
    text.find(&start).map_or("", |at| {
        let rest = &text[at + start.len()..];
        rest.find("\n## ").map_or(rest, |end| &rest[..end])
    })
}

/// The saved run of this fight: the newest run file under `Runs/` that names it.
fn run_file(d: &mut Driver<'_>, fight: &str) -> String {
    let campaign = d.app_mut().campaigns.active_campaign().expect("a campaign");
    let folder = d
        .app()
        .encounters
        .run_directory(&campaign)
        .expect("the runs' folder");
    let mut runs: Vec<_> = fs::read_dir(&folder)
        .unwrap_or_else(|error| panic!("read {}: {error}", folder.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let text = fs::read_to_string(&path).ok()?;
            let modified = fs::metadata(&path).ok()?.modified().ok()?;
            text.contains(fight).then_some((modified, text))
        })
        .collect();
    runs.sort_by_key(|(modified, _)| *modified);
    runs.pop()
        .unwrap_or_else(|| panic!("no run file under {} names {fight}", folder.display()))
        .1
}

/// The run's set-up as the audit's evening counts it: roll the NPCs, type each player's
/// initiative, and begin.
fn counted_roll_and_begin(a: &mut Audit<'_, '_>) {
    setup_fight(a);
    a.click(Do, "Begin combat");
}

/// A fight's own ending as the run costs it today, with an outcome in the DM's words.
fn counted_end(a: &mut Audit<'_, '_>, outcome: Option<&str>) {
    a.click(Do, "End encounter…");
    match outcome {
        Some(words) => {
            a.act(Do, "type the outcome", |d| d.type_text(words));
            a.act(Do, "end with Enter", |d| d.press(Key::Enter));
        }
        None => a.click(Do, "End encounter"),
    }
}

/// `Add rewards` for this played fight, where the fit rule puts it: on the row, else the last
/// link of the opened row, else a line of the row's menu (section 2.5, row 10).
fn add_rewards(a: &mut Audit<'_, '_>, fight: &str) {
    let on_row = format!("Add rewards: {fight}");
    if a.d.has(&on_row) {
        a.click(Do, &on_row);
        return;
    }
    if !a.d.has("Add rewards") {
        a.click(Find, &format!("More actions for {fight}"));
    }
    a.click(Do, "Add rewards");
}

/// The length of a journey whose checks were just added. The design puts the focus in `How
/// long`; when it is not there, the DM clicks the field first.
fn type_length(a: &mut Audit<'_, '_>) {
    let focus = a.d.focused_name();
    a.fact(format!(
        "after Roll for encounters, the focus is in {focus:?}"
    ));
    if focus.as_deref() == Some("How long") {
        a.act(Do, "type the length", |d| d.type_text("2 days"));
    } else {
        a.act(Do, "click How long, and type the length", |d| {
            d.type_into("How long", "2 days");
        });
    }
}

/// J2, section 2.2: the party skips the planned fight, talks to a planned NPC and then attacks
/// him, takes loot nobody planned, and travels two days with a random encounter on the second.
fn improvised_evening(a: &mut Audit<'_, '_>) {
    a.setup(
        "prep: Warden Ostrel, the Wolf Pack encounter and the road table",
        |d| {
            let varra = splat::character::stable_id("visual:note:varra");
            npc(
                d,
                OSTREL,
                &format!("  creature: {{id: {varra}}}\n"),
                "Keeps the ford for the watch.",
            );
            encounter(
                d,
                "Bridge Ambush",
                WOLVES,
                "Wolves at the edge of the firelight.",
            );
            road_table(d, 9);
        },
    );
    a.setup(
        "play last week's session, then quit and open the app",
        last_week,
    );
    a.setup("work at 1200x800", |d| d.set_base((1200.0, 800.0)));
    a.setup(
        "prep: tonight's plan holds a fight and a talk with Ostrel",
        |d| {
            tonight(
                d,
                2,
                TONIGHT_SESSION,
                &format!("- [[{SHRINE}]]\n- [ ] [[{OSTREL}]]\n"),
            );
        },
    );
    a.setup("save, quit and open the app at the table", restart);
    a.phase = "during";

    a.task("Play the planned NPC: read Warden Ostrel");
    a.fact(format!(
        "the planned fight stays on the plan, skipped: {}",
        a.d.has(&format!("Start fight: {SHRINE}"))
    ));
    a.act(Panel, "open him from the scene", |d| {
        d.click_starting(&format!("{OSTREL} (opens"));
    });

    a.task("The party attacks Warden Ostrel: start the fight");
    let start = format!("Start a fight with {OSTREL}");
    if !a.d.has(&start) {
        a.click(Find, &format!("More actions for {OSTREL}"));
    }
    a.click(Nav, &start);
    a.act(Panel, "close Lookup, to see his stat block", |d| {
        d.press(Key::Escape);
    });
    a.fact(format!(
        "the run of Fight with {OSTREL} is open: {}",
        a.d.has_containing(&format!("{RUN_HEADING}Fight with {OSTREL}"))
    ));

    a.task("Roll and begin the fight with Warden Ostrel");
    counted_roll_and_begin(a);

    a.task("Warden Ostrel gives up: end the fight");
    counted_end(a, Some("Ostrel gave up"));

    a.task("He gave up, and the party holds him: a line in his Log");
    a.act(Panel, "Ctrl+K shows his page again", |d| {
        d.press_ctrl(Key::K)
    });
    a.click(Do, "Add to Log");
    a.act(Do, "type the line", |d| {
        d.type_text("Gave up at the ford; the party holds him.");
    });
    a.act(Do, "add it", |d| d.press(Key::Enter));
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));
    let log = file(a.d, &ostrel_path())
        .split("\n## Log\n")
        .nth(1)
        .unwrap_or("")
        .to_owned();
    a.outcome(
        "his Log holds the line once, stamped with tonight's session and Day 1",
        log.matches(&format!(
            "{TONIGHT_LINK}, Day 1: Gave up at the ford; the party holds him."
        ))
        .count()
            == 1,
        &log,
    );

    a.task("Close the fight, keep the rewards, and add loot nobody planned");
    a.act(Nav, CLOSE_RUN, close_run);
    add_rewards(a, &format!("Fight with {OSTREL}"));
    a.act(Do, "click under the new block", |d| {
        d.click_line_end("  - XP ")
    });
    a.act(Do, "type the loot", |d| {
        d.type_text("\n  - Ostrel’s signet ring")
    });
    let note = saved_note(a.d);
    let rewards = section(&note, "Loot and rewards");
    let block = rewards
        .split("\n- ")
        .filter(|block| block.contains(&format!("|Fight with {OSTREL}]]")))
        .collect::<Vec<_>>();
    a.outcome(
        "the note holds one block for the fight, its XP line, and the ring under it",
        block.len() == 1
            && block[0].contains("\n  - XP ")
            && block[0].trim_end().ends_with("\n  - Ostrel’s signet ring")
            && note.matches("Ostrel’s signet ring").count() == 1,
        rewards,
    );
    let fight = saved_event(a.d, &format!("Fight with {OSTREL}"));
    a.outcome(
        "Timeline.md holds the fight with Ostrel on Day 1, with the outcome",
        fight
            .as_ref()
            .is_some_and(|(event, day)| *day == 1 && event.result.starts_with("Ostrel gave up")),
        &fight,
    );

    a.task("Two days overland: roll for encounters on the road");
    if !a.d.has("Add Travel") {
        a.click(Find, "More Add actions");
    }
    a.click(Do, "Add Travel");
    a.click(Find, "More actions for Travel");
    a.click(Do, "Roll for encounters");
    type_length(a);
    a.click(Do, "Roll all 2");
    let travel = saved_event(a.d, "Travel");
    let timeline = timeline_file(a.d);
    a.outcome(
        "Timeline.md holds the journey on Day 1: 2 days, the road table, and one check for each \
         day, the second naming the Wolf Pack",
        travel
            .as_ref()
            .is_some_and(|(event, day)| *day == 1 && event.duration == "2 days")
            && timeline.matches("  - Checks table: road\n").count() == 1
            && timeline.matches("  - Check at ").count() == 2
            && timeline.lines().any(|line| {
                line.starts_with("  - Check at 24h: ") && line.contains(&format!("[[{WOLVES}]]"))
            }),
        &timeline,
    );

    a.task("The second day names the Wolf Pack: plan it and start it");
    a.click(Do, &format!("Plan “{WOLVES}”"));
    a.click(Nav, &format!("Start fight: {WOLVES}"));
    let plan = section(&note_on_disk(a.d), "Plan for tonight").to_owned();
    a.outcome(
        "the plan holds the Wolf Pack once, above the skipped fight",
        plan.matches(&format!("[[{WOLVES}]]")).count() == 1
            && plan.contains(&format!("- [[{WOLVES}]]\n- [[{SHRINE}]]\n")),
        &plan,
    );

    a.task("Roll and begin the Wolf Pack fight");
    counted_roll_and_begin(a);

    a.task("End the Wolf Pack fight");
    counted_end(a, None);
    a.act(Nav, CLOSE_RUN, close_run);

    // The Wolf Pack came from the check at 24h, the second day of the journey, so its fight is
    // on Day 2. To become an assertion when the road encounter's day is repaired on `main`
    // (FINDINGS.md, J2-2): the app puts it on Day 3 today.
    let wolves = saved_event(a.d, WOLVES).map(|(_, day)| day);
    a.fact(format!(
        "the Wolf Pack fight is on Day {wolves:?}; the design says Day 2 (J2-2, recorded, not checked)"
    ));

    a.phase = "after";
    a.task("Put the Wolf Pack fight's rewards in the note");
    add_rewards(a, WOLVES);
    let rewards = section(&saved_note(a.d), "Loot and rewards").to_owned();
    a.outcome(
        "the note holds one block for each fight, Ostrel's first, each with its XP line",
        rewards.matches(&format!("|{WOLVES}]]\n  - XP ")).count() == 1
            && rewards
                .matches(&format!("|Fight with {OSTREL}]]\n  - XP "))
                .count()
                == 1
            && rewards.find(OSTREL) < rewards.find(WOLVES),
        &rewards,
    );

    a.task("Mark the talk with Warden Ostrel done, and log what happened");
    unfold_scenes(a, &format!("Open: {OSTREL}"));
    a.click(Do, &format!("Open: {OSTREL}"));
    a.click(Panel, &format!("Add to Log: {OSTREL}"));
    a.act(Do, "type the line", |d| {
        d.type_text("Handed to the watch at the ford.");
    });
    a.act(Do, "add it", |d| d.press(Key::Enter));
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));
    let log = file(a.d, &ostrel_path())
        .split("\n## Log\n")
        .nth(1)
        .unwrap_or("")
        .to_owned();
    a.outcome(
        "his Log holds both lines once each, in order, both stamped",
        log.matches(&format!("{TONIGHT_LINK}, Day ")).count() == 2
            && log.matches("Handed to the watch at the ford.").count() == 1
            && log.find("Gave up at the ford") < log.find("Handed to the watch"),
        &log,
    );
    let plan = section(&saved_note(a.d), "Plan for tonight").to_owned();
    a.outcome(
        "the talk with Ostrel is ticked done once in the plan",
        plan.matches(&format!("- [x] [[{OSTREL}]]")).count() == 1
            && !plan.contains(&format!("- [ ] [[{OSTREL}]]")),
        &plan,
    );
}

fn ostrel_path() -> String {
    format!("NPCs/{OSTREL}.md")
}

/// J3, section 2.3, at 960x640 with Lookup open: a fight ended by mistake, a crash in round 3
/// with an effect pinned to a creature, Lookup after the restart, a D&D Beyond sync conflict
/// (simulated, as the scenario `run-sync-conflict` does; its Review and choices are real), an
/// edit made in Obsidian, the same edit while Splat holds unsaved words, and a removed scene's
/// Undo after the next fight begins (W6-D1 of the first scored round).
fn broken_evening(a: &mut Audit<'_, '_>) {
    a.setup(
        "prep: the encounters, and the Tournament's trackers in play",
        |d| {
            encounter(
                d,
                "Bridge Ambush",
                WOLVES,
                "Wolves at the edge of the firelight.",
            );
            encounter(d, SHRINE, WATCH, "The watch holds the gate.");
            race_in_play(d);
        },
    );
    a.setup(
        "play last week's session, then quit and open the app",
        last_week,
    );
    a.setup("work at 960x640", |d| d.set_base((960.0, 640.0)));
    a.setup(
        "prep: tonight's plan holds three fights and two scenes",
        |d| {
            tonight(
                d,
                2,
                TONIGHT_SESSION,
                &format!(
                    "- [[{SHRINE}]]\n- [[{WOLVES}]]\n- [[{WATCH}]]\n- [ ] {ROPES}\n- [ ] {LEDGER}\n"
                ),
            );
        },
    );
    a.setup("save, quit and open the app at the table", restart);
    a.setup(
        "a notice is due from before the fight: the first bell",
        |d| {
            d.press_ctrl(Key::T);
            d.click("Cistern or Sands");
            d.press_ctrl(Key::T);
        },
    );
    a.setup("sync with D&D Beyond is on", |d| {
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        d.settle();
    });
    a.setup("play the first fight", |d| played_fight(d, SHRINE));
    a.setup("the second fight, in round 2", |d| {
        d.click(&format!("Start fight: {WOLVES}"));
        roll_and_begin(d);
        for _ in 0..8 {
            d.click(NEXT_TURN);
        }
    });
    a.setup("Lookup shows last week's note, found by `miller`", |d| {
        d.press_ctrl(Key::K);
        d.type_text("miller");
        d.click_starting(&format!("Session 1: {LAST_WEEK}"));
    });
    a.phase = "during";
    let start = (
        a.d.window_size(),
        a.d.app().lookup.open,
        a.d.visible_text(MILLER),
        a.d.rail_words(),
        a.d.has_containing("Round 2"),
    );
    a.outcome(
        "the start: 960x640, Lookup open on the found note, the rail says Due 1, round 2",
        start.0 == (960.0, 640.0) && start.1 && start.2 && start.3.contains("Due 1") && start.4,
        &start,
    );

    a.task("End the fight by mistake");
    a.click(Do, "End encounter…");
    a.click(Do, "End encounter");

    a.task("Take the ending back");
    a.click(Do, "Undo: end encounter");
    let run = run_file(a.d, WOLVES);
    a.outcome(
        "the run file is in round 2 again, and not ended",
        a.d.has_containing("Round 2") && run_round(&run) == Some(2) && !run_ended(&run),
        &run,
    );

    // At 960x640 with Lookup open, the dock has no room for `Effect`, and `E` is off while
    // Lookup is open: the Conditions tab's `Other effect` field is the way (section 7).
    a.task("Pin Poisoned 3 rounds to a wolf");
    a.act(Find, "select the wolf", |d| d.click_row("Cinder Wolf 2"));
    if a.d.has(EFFECT) {
        a.click(Do, EFFECT);
        a.act(Do, "type the effect", |d| d.type_text("Poisoned 3 rounds"));
    } else {
        a.click(Sub, CONDITIONS);
        a.act(Do, "type the effect in Other effect", |d| {
            d.type_into("Other effect", "Poisoned 3 rounds");
        });
    }
    a.act(Do, "add with Enter", |d| d.press(Key::Enter));

    a.task("Play on into round 3");
    for _ in 0..8 {
        a.click(Do, NEXT_TURN);
    }
    let undo_before = a.d.name_starting("Undo");

    a.setup("the app stops in round 3, and opens again", |d| {
        d.restart();
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        d.settle();
    });
    a.task("After the crash: back to the fight");
    if a.d.current_page() != "Run" {
        a.click(Nav, BACK_TO_RUN);
    }
    let run = run_file(a.d, WOLVES);
    let wolf = run_participant(&run, "Cinder Wolf 2").map(|wolf| wolf.to_string());
    let after = (
        run_round(&run),
        a.d.row_words("Cinder Wolf 2"),
        a.d.name_starting("Undo"),
        a.d.rail_words(),
    );
    a.outcome(
        "the run file holds round 3 and the effect; the wolf's row says Poisoned 2; Undo names \
         the last saved step; the rail still says Due 1, read from Trackers.md",
        after.0 == Some(3)
            && wolf
                .as_deref()
                .is_some_and(|wolf| wolf.contains("Poisoned"))
            && after.1.contains("Poisoned 2")
            && after.2 == undo_before
            && after.3.contains("Due 1")
            && bell_due(a.d),
        (&after, &undo_before, &wolf),
    );

    a.task("Lookup is closed after the restart: find the note again");
    a.act(Panel, "open Lookup", |d| d.press_ctrl(Key::K));
    if !a.d.visible_text(MILLER) {
        a.act(Find, "search: miller", |d| d.type_text("miller"));
    }

    a.setup(
        "D&D Beyond: Kestrel Dunmoor's sheet changed, on the wolf's turn",
        |d| {
            d.app_mut().encounters.simulate_sync_conflict(KESTREL);
            d.settle();
        },
    );
    a.task("A sync conflict on Kestrel Dunmoor: review it and choose");
    a.click(Do, "Review");
    a.click(Do, USE_REMOTE);
    a.act(Find, "select the wolf again", |d| {
        d.click_row("Cinder Wolf 2")
    });
    a.act(Panel, "open Lookup", |d| d.press_ctrl(Key::K));
    a.act(Find, "open the found note again", |d| {
        d.click_starting(&format!("Session 1: {LAST_WEEK}"));
    });
    let run = run_file(a.d, WOLVES);
    let back = (
        a.d.row_words(KESTREL),
        a.d.row_hp(KESTREL),
        run_participant(&run, KESTREL).and_then(|kestrel| kestrel["hp"].as_i64()),
        a.d.focused_row_name(),
        a.d.visible_text(MILLER),
    );
    a.outcome(
        "Kestrel's row has no conflict, and the run file holds the HP the DM chose; the wolf is \
         selected; Lookup shows the note again",
        !back.0.contains("Conflict")
            && back.1.is_some()
            && back.1 == back.2
            && back
                .3
                .as_deref()
                .is_some_and(|name| name.contains("Cinder Wolf 2"))
            && back.4,
        &back,
    );

    a.setup(
        "in Obsidian, the DM ticks a scene and adds a plan line",
        |d| {
            let text = note_on_disk(d);
            obsidian_writes(
                d,
                &text.replacen(
                    &format!("- [ ] {LEDGER}\n"),
                    &format!("- [x] {LEDGER}\n- [ ] {FERRY}\n"),
                    1,
                ),
            );
        },
    );
    // Nothing opens on the Run page; Splat held no unsaved words, so it takes the file's text.
    a.task("The note changed in Obsidian: see it in Splat");
    a.outcome(
        "no conflict: the rail does not say Notes changed",
        !a.d.rail_words().contains("Notes changed"),
        a.d.rail_words(),
    );

    a.setup(
        "the DM types a line in the note; it is not saved yet",
        |d| {
            d.press_ctrl(Key::J);
            d.type_into_editor("\nThe wolves fear fire.\n");
            d.press_ctrl(Key::K);
            d.click_starting(&format!("Session 1: {LAST_WEEK}"));
        },
    );
    a.setup("in Obsidian, the DM adds another plan line", |d| {
        let text = note_on_disk(d);
        obsidian_writes(
            d,
            &text.replacen(
                &format!("- [ ] {FERRY}\n"),
                &format!("- [ ] {FERRY}\n- [ ] {TOLL}\n"),
                1,
            ),
        );
    });

    a.task("End the second fight");
    counted_end(a, None);
    a.act(Nav, CLOSE_RUN, close_run);

    a.phase = "after the fight";
    // Before Splat's save after a pause, a plan act meets the unsaved words and the changed file.
    a.task("Add the Wolf Pack's rewards while the note has unsaved words and changed on disk");
    let before = note_on_disk(a.d);
    add_rewards(a, WOLVES);
    let refused = (
        a.d.visible_text("The note changed on disk. Choose in the note first."),
        note_on_disk(a.d),
    );
    a.outcome(
        "the act is refused with its words, and the file is as Obsidian left it",
        refused.0 && refused.1 == before && refused.1.contains(TOLL),
        &refused,
    );
    a.setup("Splat's save after a pause meets the changed file", |d| {
        d.app_mut().sessions.flush();
        d.settle();
    });
    a.task("The note changed on disk: choose a version, and add the rewards again");
    a.outcome(
        "the rail says Notes changed",
        a.d.rail_words().contains("Notes changed"),
        a.d.rail_words(),
    );
    a.click(Do, "Overwrite with my draft");
    add_rewards(a, WOLVES);
    let note = note_on_disk(a.d);
    a.outcome(
        "the note holds the chosen version, the DM's words with the first Obsidian edit, and \
         the rewards once; Lookup still shows the found note",
        note.contains("The wolves fear fire.")
            && note.contains(&format!("- [x] {LEDGER}\n- [ ] {FERRY}\n"))
            && !note.contains(TOLL)
            && section(&note, "Loot and rewards")
                .matches(&format!("|{WOLVES}]]"))
                .count()
                == 1
            && a.d.visible_text(MILLER),
        &note,
    );

    a.task("Take a scene off tonight's plan");
    let plan_before = section(&note_on_disk(a.d), "Plan for tonight").to_owned();
    unfold_scenes(a, &format!("More actions for {ROPES}"));
    a.click(Find, &format!("More actions for {ROPES}"));
    a.click(Do, "Remove from tonight");

    a.task("Begin the fight before it");
    a.click(Nav, &format!("Start fight: {WATCH}"));

    a.task("The scene is wanted after all: take the removal back");
    a.click(Nav, NAV_SESSIONS);
    a.click(Do, "Undo remove");
    let plan = section(&note_on_disk(a.d), "Plan for tonight").to_owned();
    a.outcome(
        "the plan is as before the removal: the scene at its old place, the Obsidian lines kept",
        plan == plan_before && plan.matches(ROPES).count() == 1,
        (&plan, &plan_before),
    );
}

const ROPES: &str = "Ask the watch about the ropes";
const LEDGER: &str = "Find the miller's ledger";
const FERRY: &str = "Pay the ferryman";
const TOLL: &str = "Pay the toll";
const KESTREL: &str = "Kestrel Dunmoor";
const USE_REMOTE: &str = "Use D&D Beyond values";

/// The ledger holds the first bell's crossing, and no end of its notice: the bell is due.
fn bell_due(d: &Driver<'_>) -> bool {
    let entries = ledger(d);
    entries
        .iter()
        .find(|entry| entry.why.as_deref() == Some("Cistern or Sands"))
        .is_some_and(|cistern| !notice_done(&entries, &cistern.id))
}

/// Obsidian writes the open session note. The app reads a note that changed on disk on a poll
/// of one second of real time.
fn obsidian_writes(d: &mut Driver<'_>, text: &str) {
    let path = d
        .app()
        .sessions
        .selected_path()
        .expect("tonight's note")
        .to_owned();
    fs::write(path, text).expect("write tonight's note");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    d.settle();
}

/// A saved run's state, as its file holds it.
fn run_state(run: &str) -> serde_json::Value {
    let run: serde_json::Value = serde_json::from_str(run).expect("the run file reads");
    run["state"].clone()
}

/// The round a saved run is in.
fn run_round(run: &str) -> Option<u64> {
    run_state(run)["round"].as_u64()
}

/// The saved run is ended.
fn run_ended(run: &str) -> bool {
    run_state(run)["phase"].get("Ended").is_some()
}

/// A participant of a saved run, by name.
fn run_participant(run: &str, name: &str) -> Option<serde_json::Value> {
    run_state(run)["participants"]
        .as_array()?
        .iter()
        .find(|participant| participant["name"] == name)
        .cloned()
}

/// The Tournament's note with a day mark on the day after the race, and Campaign.md with the
/// note in play.
fn race_in_play(d: &mut Driver<'_>) {
    write_the_race(d);
    let root = campaign_root(d);
    let id = splat::character::stable_id("Tournament.md");
    let manifest = root.join("Campaign.md");
    let text = fs::read_to_string(&manifest).expect("read Campaign.md");
    let list = serde_yaml_ng::to_value(vec![serde_json::json!({ "id": id })]).unwrap();
    let text = splat::content::formats::patch_model(&text, &[("splat_in_play", Some(list))])
        .expect("put the note in play");
    fs::write(manifest, text).expect("write Campaign.md");
}

/// J4, section 2.4: the Tournament between fights; the arena fight of three rounds, with one
/// tracker act in it by Ctrl+T, as the owner ruled, and the rival group that joins it on its
/// seal's notice; then a day mark that comes due between sessions.
fn the_race(a: &mut Audit<'_, '_>) {
    a.setup("the Tournament's note is in play", race_in_play);
    a.setup(
        "prep: the Old Guard's 1-2 seals group, and the Tournament's prize day on Day 2",
        |d| {
            let root = campaign_root(d);
            let group = splat::content::formats::fixture(
                &format!("Encounter Groups/{OLD_GUARD}.md"),
                &format!(
                    "splat_id: {}\nsplat_kind: encounter_group\nsplat_name: {OLD_GUARD}\nsplat_encounter_group:\n  side: rivals\n  members:\n  - creature: {{key: 'dnd2024:creatures/goblin_warrior'}}\n    count: 3\n",
                    splat::character::stable_id("visual:group:old-guard")
                ),
                "\nThe Old Guard's runners, sent for the first seal.\n",
            )
            .expect("an encounter group note");
            fs::create_dir_all(root.join("Encounter Groups")).expect("make the groups' folder");
            fs::write(root.join(format!("Encounter Groups/{OLD_GUARD}.md")), group)
                .expect("write the Old Guard's group");
            let path = root.join("Tournament.md");
            let text = fs::read_to_string(&path).expect("read the Tournament");
            let mark = "> day_marks:\n>   prizes:\n>     name: Prize day\n>     at: 2\n>     say: Pay each team its prize money.\n>     changes:\n>     - {tracker: favor, add: 1}\n";
            fs::write(&path, text.replacen("> trackers:\n", &format!("{mark}> trackers:\n"), 1))
                .expect("write the Tournament");
        },
    );
    a.setup("open the app on it", |d| {
        d.restart();
        d.set_base((1200.0, 800.0));
    });
    a.setup("the race is on Day 1", |d| {
        d.click(NAV_TIMELINE);
        d.click("Add Day");
    });
    a.setup("prep: tonight's plan holds the arena fight", |d| {
        tonight(d, 1, "The Race", &format!("- [[{SHRINE}]]\n"));
    });
    a.setup("save, quit and open the app at the table", restart);
    a.setup("the race so far: the clock at 30, Favor 2, no seals", |d| {
        d.press_ctrl(Key::T);
        d.click("Cistern or Sands");
        d.click("Done: Bell");
        d.click("Enter room");
        d.click("Crowd loves it");
        d.click("Crowd loves it");
        d.press_ctrl(Key::T);
    });
    a.phase = "between fights";
    let values = ledger(a.d);
    a.outcome(
        "the start: the clock at 30, Favor 2, no seals",
        sum(&values, "race_clock") == 30
            && sum(&values, "favor") == 2
            && sum(&values, "seals") == 0,
        &values,
    );

    a.task("Open the numbers");
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));

    a.task("The party enters a room: the clock reaches 40, and a bell is due");
    a.click(Do, "Enter room");

    a.task("The bell: roll each rival and the wandering table");
    for _ in 0..3 {
        a.click(Do, "Roll Rival Track");
    }
    a.click(Do, "Roll Wandering, 0:40 to 1:19");

    a.task("The Old Guard takes its first seal");
    a.click(Do, "Take seal: Old Guard");

    a.task("Put the seal on the Timeline");
    a.click(Do, "Add to Timeline");
    let seal = saved_event(a.d, "Take seal");
    a.outcome(
        "Timeline.md holds the seal as an event of Day 1",
        seal.as_ref().is_some_and(|(_, day)| *day == 1),
        &seal,
    );

    a.task("Finish the bell");
    a.click(Do, "Done: Bell");

    a.task("Keep the seal's notice in mind for the next fight");
    let values = ledger(a.d);
    a.outcome(
        "the ledger holds four bell rolls for the clock at 40, the Old Guard's seal, and the \
         rail says one notice is due",
        sum(&values, "race_clock") == 40
            && rolls_on(&values, "rival_track") == 3
            && rolls_on(&values, "wander_middle") == 1
            && old_guard_seal(&values).is_some()
            && a.d.has_containing("1 notice is due"),
        &values,
    );

    a.task("The party enters the arena");
    a.click(Do, "Enter room");

    a.phase = "the fight";
    a.task("Start the arena fight");
    let start = format!("Start fight: {SHRINE}");
    if !a.d.has(&start) && a.d.has("Plan") {
        a.click(Sub, "Plan");
    }
    a.click(Nav, &start);
    // Lookup stays open on Trackers in the run; the DM closes it to see the fight's panel.
    if a.d.has("Crowd loves it") {
        a.act(Panel, "Ctrl+T, to see the fight's panel", |d| {
            d.press_ctrl(Key::T)
        });
    }

    a.task("Roll and begin the arena fight");
    counted_roll_and_begin(a);

    a.task("Round 1");
    let first = turns_to_round(a, 2);

    a.task("The crowd loves a move in the fight: mark Crowd Favor");
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));
    if !a.d.has("Crowd loves it") {
        a.act(Panel, "Ctrl+T again: Trackers is not in view", |d| {
            d.press_ctrl(Key::T);
        });
    }
    a.click(Do, "Crowd loves it");
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));

    a.task("The seal's notice is due: the Old Guard's group joins the fight");
    a.outcome(
        "the rail says the seal's notice is due in the fight",
        a.d.rail_words().contains("Due 1"),
        a.d.rail_words(),
    );
    a.click(Find, "Add to fight");
    a.click(Do, &add_group(OLD_GUARD));

    a.task("Round 2");
    let second = turns_to_round(a, 3);

    a.task("Round 3");
    for _ in 1..second {
        a.click(Do, NEXT_TURN);
    }

    a.task("End the arena fight");
    counted_end(a, None);
    let run = run_file(a.d, SHRINE);
    let fight = saved_event(a.d, SHRINE);
    a.outcome(
        "the saved run ended in round 3 with the Old Guard's group in it; Timeline.md says round 3 \
         on Day 1",
        run_round(&run) == Some(3)
            && run.contains("Goblin Warrior")
            && fight
                .as_ref()
                .is_some_and(|(event, day)| *day == 1 && event.result.contains("round 3")),
        (&fight, first, second, &run),
    );
    a.act(Nav, CLOSE_RUN, close_run);

    a.phase = "after the fight";
    a.task("Three minutes passed in the fight");
    if !a.d.has("Fight minute") {
        a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));
    }
    for _ in 0..3 {
        a.click(Do, "Fight minute");
    }

    a.task("The rivals came: finish the seal's notice");
    a.click(Do, "Done: First seal for Old Guard");
    let values = ledger(a.d);
    a.outcome(
        "the ledger holds the fight's Favor, three fight minutes, and the seal's notice done",
        sum(&values, "race_clock") == 53
            && sum(&values, "favor") == 3
            && values
                .iter()
                .filter(|entry| entry.why.as_deref() == Some("Fight minute"))
                .count()
                == 3
            && old_guard_seal(&values).is_some_and(|seal| notice_done(&values, &seal.id)),
        &values,
    );

    a.task("The crowd loved a move, then threw a gift");
    a.click(Do, "Crowd loves it");
    a.click(Do, "Throw a gift");

    a.task("A wrong press on Throw a gift, seen after the next right press");
    a.click(Do, "Throw a gift");
    a.click(Do, "Crowd loves it");
    a.click(Do, "Crowd loves it");
    let values = ledger(a.d);
    let presses: Vec<_> = values
        .iter()
        .filter(|entry| entry.tracker.as_deref() == Some("favor"))
        .map(|entry| entry.why.clone().unwrap_or_default())
        .collect();
    a.outcome(
        "the ledger keeps both presses and the opposite one, in order, with Favor at 4 and a gift \
         roll for each Throw a gift",
        presses.ends_with(&[
            "Crowd loves it".to_owned(),
            "Throw a gift".to_owned(),
            "Throw a gift".to_owned(),
            "Crowd loves it".to_owned(),
            "Crowd loves it".to_owned(),
        ]) && sum(&values, "favor") == 4
            && values
                .iter()
                .filter(|entry| {
                    entry.why.as_deref() == Some("Throw a gift")
                        && matches!(entry.roll, Some(Roll::Dice(_)))
                })
                .count()
                == 2,
        &values,
    );

    a.task("A soul is collected: the clock passes 60");
    a.click(Do, "Collect soul");

    a.task("The second bell");
    for _ in 0..3 {
        a.click(Do, "Roll Rival Track");
    }
    a.click(Do, "Roll Wandering, 0:40 to 1:19");
    a.click(Do, "Done: Bell");
    let values = ledger(a.d);
    a.outcome(
        "the clock is at 63, one soul is in the jar, and the second bell's four rolls are done",
        sum(&values, "race_clock") == 63
            && sum(&values, "souls") == 1
            && rolls_on(&values, "rival_track") == 6
            && rolls_on(&values, "wander_middle") == 2
            && !a.d.has_containing("notice is due"),
        &values,
    );

    a.setup("save, quit and open the app after the session", restart);
    a.phase = "between sessions";
    a.task("A new day between sessions: the prize day comes due");
    a.click(Nav, NAV_TIMELINE);
    a.click(Do, "Add Day");
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));
    let done =
        a.d.nodes()
            .iter()
            .find(|node| node.name.starts_with("Done: Prize day"))
            .map_or_else(|| "Done: Prize day".to_owned(), |node| node.name.clone());
    a.click(Do, &done);
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));
    let values = ledger(a.d);
    let today = crate::timeline::day_now(
        &crate::timeline::parse(&timeline_file(a.d)).expect("Timeline.md reads"),
    );
    a.outcome(
        "the story is on Day 2, and the ledger holds the prize day's mark done",
        today == Some(2)
            && values.iter().any(|entry| {
                entry.done
                    && matches!(&entry.notice, Some(NoticeRef::Day { mark, day: 2 }) if mark == "prizes")
            })
            && sum(&values, "favor") == 5,
        (today, &values),
    );
}

const OLD_GUARD: &str = "Old Guard, 1-2 seals";

/// Play turns until the run reaches this round. Returns how many turns that took.
fn turns_to_round(a: &mut Audit<'_, '_>, round: u32) -> u32 {
    let mut turns = 0;
    while !a.d.has_containing(&format!("Round {round}")) {
        assert!(turns < 30, "the run never reached round {round}");
        a.click(Do, NEXT_TURN);
        turns += 1;
    }
    turns
}

use crate::trackers::ledger::{Entry, NoticeRef, Roll};

/// The entries of the saved tracker ledger, `Trackers.md`, in order, read as the app reads them.
fn ledger(d: &Driver<'_>) -> Vec<Entry> {
    crate::trackers::ledger::Ledger::parse(&file(d, "Trackers.md"))
        .expect("Trackers.md reads")
        .changes
}

/// The Old Guard's seal.
fn old_guard_seal(entries: &[Entry]) -> Option<&Entry> {
    entries.iter().find(|entry| {
        entry.tracker.as_deref() == Some("seals") && entry.instance.as_deref() == Some("Old Guard")
    })
}

/// A tracker's value from the ledger: the sum of its changes, from 0.
fn sum(entries: &[Entry], tracker: &str) -> i64 {
    entries
        .iter()
        .filter(|entry| entry.tracker.as_deref() == Some(tracker))
        .filter_map(|entry| entry.add)
        .sum()
}

/// The rolls on a table that the ledger holds.
fn rolls_on(entries: &[Entry], table: &str) -> usize {
    entries
        .iter()
        .filter(|entry| matches!(&entry.roll, Some(Roll::Table(roll)) if roll.table == table))
        .count()
}

/// The notice that this change raised is marked done.
fn notice_done(entries: &[Entry], change: &str) -> bool {
    entries.iter().any(|entry| {
        entry.done && entry.notice.as_ref().and_then(NoticeRef::change) == Some(change)
    })
}

/// J6, section 2.6: after the last fight, the loot, an NPC's status and Log, the rewards; a
/// week later a new session with what the last one left, three scenes, a fight's difficulty
/// read and changed in the encounter editor, a road with its checks ready, and `Start fight`.
fn week_between(a: &mut Audit<'_, '_>) {
    a.setup("prep: the encounters, the treasure table and Verori", |d| {
        encounter(d, "Bridge Ambush", WOLVES, "Wolves at the edge of the firelight.");
        encounter(
            d,
            SHRINE,
            WATCH,
            "The watch holds the gate.\n\n## Loot\n\n- 120 gp in a locked coffer\n- Roll 2× gems",
        );
        encounter(d, SHRINE, GATE, "Two guards at the north gate.");
        encounter(d, "Bridge Ambush", PATROL, "A patrol on the ford road.");
        fs::write(
            campaign_root(d).join("Treasure.md"),
            "# Treasure\n\n<!-- splat:rolls key=gems dice=1d4 -->\n\n| d4 | Gem |\n| --- | --- |\n| 1 | A cracked garnet (10 gp) |\n| 2 | A river pearl (50 gp) |\n| 3 | A smoky quartz (50 gp) |\n| 4 | A star ruby (100 gp) |\n",
        )
        .expect("write the treasure table");
        npc(d, VERORI, "", "Keeper of the north gate.");
        road_table(d, 9);
    });
    a.setup(
        "play last week's session, then quit and open the app",
        last_week,
    );
    a.setup("work at 1200x800", |d| d.set_base((1200.0, 800.0)));
    a.setup(
        "prep: the session's plan holds five fights and a scene, and two open threads",
        |d| {
            tonight(
                d,
                2,
                TONIGHT_SESSION,
                &format!(
                    "- [[{SHRINE}]]\n- [[{WOLVES}]]\n- [[{WATCH}]]\n- [[{GATE}]]\n- [[{PATROL}]]\n- [ ] {KEY_SCENE}\n"
                ),
            );
            let body = d.editor_text().expect("tonight's note");
            d.replace_editor_text(
                &body
                    .replacen(
                        "## What happened\n",
                        "## What happened\n\nThe party crossed the ford and burned the shrine.\n",
                        1,
                    )
                    .replacen(
                        "## Open threads\n",
                        &format!("## Open threads\n\n- [ ] {THREAD}\n- [x] {DONE_THREAD}\n"),
                        1,
                    ),
            );
            d.press_ctrl(Key::S);
            show_plan(d);
        },
    );
    a.setup("save, quit and open the app at the table", restart);
    let last = note_on_disk(a.d);
    a.outcome(
        "last week's note holds an open thread and an unplayed scene to carry",
        section(&last, "Open threads").contains(&format!("- [ ] {THREAD}\n"))
            && section(&last, "Plan for tonight").contains(&format!("- [ ] {KEY_SCENE}\n")),
        &last,
    );
    a.setup("play two fights, and the third to its end", |d| {
        played_fight(d, SHRINE);
        played_fight(d, WOLVES);
        d.click(&format!("Start fight: {WATCH}"));
        roll_and_begin(d);
        end_fight(d);
    });
    a.phase = "after the last fight";

    a.task("The party searches the last fight");
    a.click(Do, &format!("Roll 2 × gems for {WATCH}"));

    a.task("Verori died here");
    a.act(Panel, "open Lookup", |d| d.press_ctrl(Key::K));
    a.act(Find, "search: verori", |d| d.type_text("verori"));
    // Lookup's reader is compact and has no `Change` rows: the status goes by `Edit`.
    if a.d.has("Change Status") {
        a.click(Do, "Change Status");
        a.act(Do, "type the status", |d| d.type_text("dead"));
        a.act(Do, "save with Enter", |d| d.press(Key::Enter));
    } else {
        a.click(Do, &format!("Edit {VERORI}"));
        a.act(Do, "change the status", |d| {
            d.replace_text_in("Status", "dead")
        });
        a.click(Do, "Save changes");
    }
    let verori = file(a.d, "NPCs/Verori.md");
    a.outcome(
        "his note holds the status dead, and one Log line for it, stamped",
        verori.contains("status: dead")
            && verori
                .matches(&format!("{TONIGHT_LINK}, Day 1: Status alive → dead."))
                .count()
                == 1,
        &verori,
    );

    a.task("He gave the gate key away first");
    a.click(Do, "Add to Log");
    a.act(Do, "type the line", |d| {
        d.type_text("Gave the gate key to Iselle.");
    });
    a.act(Do, "add it", |d| d.press(Key::Enter));
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));
    let verori = file(a.d, "NPCs/Verori.md");
    a.outcome(
        "his Log holds the gate key line once, after the status line, with the same stamp",
        verori
            .matches(&format!(
                "{TONIGHT_LINK}, Day 1: Gave the gate key to Iselle."
            ))
            .count()
            == 1
            && verori.find("alive → dead") < verori.find("Gave the gate key"),
        &verori,
    );

    a.task("Back to the evening");
    a.act(Nav, CLOSE_RUN, close_run);

    a.task("Rewards for each of the three fights");
    for fight in [SHRINE, WOLVES, WATCH] {
        add_rewards(a, fight);
    }
    let rewards = section(&saved_note(a.d), "Loot and rewards").to_owned();
    a.outcome(
        "the note holds one block for each of the three fights, each with its XP line, and the \
         searched loot",
        [SHRINE, WOLVES, WATCH]
            .iter()
            .all(|fight| rewards.matches(&format!("|{fight}]]\n  - XP ")).count() == 1)
            && rewards.matches("\n  - XP ").count() == 3
            && rewards.contains("120 gp in a locked coffer"),
        &rewards,
    );

    a.setup("a week passes: save, quit and open the app", restart);
    a.phase = "a week later";

    a.task("Start the new session");
    a.click(Do, "New session");
    a.act(Do, "type the title", |d| d.type_text("The North Gate"));
    a.click(Do, "Create session");

    a.task("Carry over what the last session left");
    a.click(Do, "Carry over");
    let note = note_on_disk(a.d);
    let (recap, plan, threads) = (
        section(&note, "Recap of last time"),
        section(&note, "Plan for tonight"),
        section(&note, "Open threads"),
    );
    a.outcome(
        "the new note holds, once each: the recap's embed, the unplayed fights and scene, the open \
         thread (not the ticked one), carried_from and the three carried parts",
        recap
            .matches(&format!("![[Session 2 - {TONIGHT_SESSION}#What happened]]"))
            .count()
            == 1
            && [
                format!("- [[{GATE}]]\n"),
                format!("- [[{PATROL}]]\n"),
                format!("- [ ] {KEY_SCENE}\n"),
            ]
            .iter()
            .all(|line| plan.matches(line.as_str()).count() == 1)
            && ![SHRINE, WOLVES, WATCH]
                .iter()
                .any(|fight| plan.contains(fight))
            && threads.matches(&format!("- [ ] {THREAD}\n")).count() == 1
            && !threads.contains(DONE_THREAD)
            && note.contains("carried_from: ")
            && note.contains("carried_parts:\n- recap\n- plan\n- threads\n"),
        &note,
    );

    a.task("Plan three scenes");
    a.act(Do, "click under the plan's last line", |d| {
        d.click_line_end(&format!("- [[{PATROL}]]"));
    });
    for scene in SCENES {
        a.act(Do, "type a scene", |d| {
            d.type_text(&format!("\n- [ ] {scene}"))
        });
    }
    let plan = section(&saved_note(a.d), "Plan for tonight").to_owned();
    a.outcome(
        "the saved plan holds the three scenes once each, after the carried lines",
        SCENES
            .iter()
            .all(|scene| plan.matches(&format!("- [ ] {scene}\n")).count() == 1)
            && plan.find(PATROL) < plan.find(SCENES[0]),
        &plan,
    );

    a.task("Is each fight right for this party?");
    let words = |d: &Driver<'_>| {
        let mut words = Vec::new();
        d.each_painted_text(|text, _| {
            let line = text.galley.job.text.replace('\u{a0}', " ");
            if let Some(word) = ["Above High", "Moderate", "High", "Low"]
                .into_iter()
                .find(|word| line.ends_with(&format!(" · {word}")))
            {
                words.push(word);
            }
        });
        words
    };
    let mut said = words(a.d);
    if said.len() < 2 {
        // A summary with no room for its word: the DM opens each fight's row to read it.
        for fight in [GATE, PATROL] {
            a.click(Find, fight);
            said = words(a.d);
        }
    }
    a.outcome(
        "a difficulty word for each of the two fights",
        said.len() == 2,
        &said,
    );

    // The plan has no way into the encounter editor: the DM goes by Encounters.
    a.task("The Gate Watch is too easy: add a creature in the encounter editor");
    a.click(Nav, NAV_ENCOUNTERS);
    a.act(Find, "Edit beside Gate Watch", |d| {
        d.click_beside_exact(GATE, EDIT_ENCOUNTER);
    });
    a.act(Find, "search: oni", |d| d.type_into(FIND_CREATURE, "oni"));
    a.act(Do, "add an Oni", |d| d.click_first(ADD));
    a.click(Do, SAVE_ENCOUNTER);
    a.click(Nav, NAV_SESSIONS);
    let gate = file(a.d, &format!("Encounters/{GATE}.md"));
    let after = words(a.d);
    a.outcome(
        "the Gate Watch's saved roster holds the Oni, and its row's word changed",
        gate.contains("Oni") && after.len() == 2 && after != said,
        (&said, &after, &gate),
    );

    a.task("Prepare the road: a Travel row with its checks");
    if !a.d.has("Add Travel") {
        a.click(Find, "More Add actions");
    }
    a.click(Do, "Add Travel");
    a.click(Find, "More actions for Travel");
    a.click(Do, "Roll for encounters");
    type_length(a);
    a.act(Do, "keep it with Enter", |d| d.press(Key::Enter));
    let travel = saved_days(a.d)
        .into_iter()
        .rev()
        .find(|(event, _)| event.title == "Travel");
    a.outcome(
        "Timeline.md holds the road of 2 days, ready to roll on the road table",
        travel
            .as_ref()
            .is_some_and(|(event, _)| event.duration == "2 days" && event.checks_table.is_some()),
        &travel,
    );

    a.task("Start the first fight");
    a.click(Nav, &format!("Start fight: {GATE}"));
    let run = run_file(a.d, GATE);
    a.outcome(
        "the new run is the Gate Watch's, with the Oni",
        a.d.current_page() == "Run" && run.contains("Oni"),
        &run,
    );
}

const KEY_SCENE: &str = "Ask Iselle about the gate key";
const THREAD: &str = "Who paid the ferryman to lie?";
const DONE_THREAD: &str = "Where did the shrine's fire come from?";
const SCENES: [&str; 3] = [
    "Meet the gate's new keeper",
    "Search Verori's rooms",
    "The ferryman's warning",
];
