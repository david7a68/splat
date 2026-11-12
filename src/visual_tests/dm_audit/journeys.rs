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

/// The campaign's marked road table, with the result on these d20 rolls naming `[[Wolf Pack]]`.
fn road_table(d: &mut Driver<'_>, wolves: &str) {
    let table = ROAD_TABLE.replace(
        "| 15-20 | Wolves pick up the trail. |",
        &format!("| {wolves} | [[{WOLVES}]] picks up the trail. |"),
    );
    fs::write(campaign_root(d).join("Road.md"), table).expect("write the road table");
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

/// What a file of the fixture's campaign holds now, or nothing.
fn file(d: &Driver<'_>, path: &str) -> String {
    fs::read_to_string(d.root.join("campaigns/emberfall").join(path)).unwrap_or_default()
}

/// What the campaign's `Timeline.md` holds now.
fn timeline_file(d: &mut Driver<'_>) -> String {
    let campaign = d.app_mut().campaigns.active_campaign().expect("a campaign");
    crate::sessions::folder(&campaign)
        .ok()
        .and_then(|folder| fs::read_to_string(folder.join("Timeline.md")).ok())
        .unwrap_or_default()
}

/// The open session note's text on disk.
fn note_on_disk(d: &Driver<'_>) -> String {
    d.app()
        .sessions
        .selected_path()
        .and_then(|path| fs::read_to_string(path).ok())
        .unwrap_or_default()
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
            road_table(d, "11-20");
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
    a.fact(format!(
        "the planned fight stays on the plan, skipped: {}",
        a.d.has(&format!("Start fight: {SHRINE}"))
    ));

    a.task("Play the planned NPC: read Warden Ostrel");
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
    a.fact(format!(
        "his note holds the line: {}",
        file(a.d, &format!("NPCs/{OSTREL}.md")).contains("the party holds him.")
    ));

    a.task("Close the fight, keep the rewards, and add loot nobody planned");
    a.act(Nav, CLOSE_RUN, close_run);
    add_rewards(a, &format!("Fight with {OSTREL}"));
    a.act(Do, "click under the new block", |d| {
        d.click_line_end("  - XP ")
    });
    a.act(Do, "type the loot", |d| {
        d.type_text("\n  - Ostrel’s signet ring")
    });
    a.fact(format!(
        "the ring is under the fight's block: {}",
        a.d.editor_text().is_some_and(|text| text
            .split("## Loot and rewards")
            .nth(1)
            .is_some_and(|rest| rest.contains(&format!("|Fight with {OSTREL}]]"))
                && rest.contains("  - Ostrel’s signet ring")))
    ));

    a.task("Two days overland: roll for encounters on the road");
    if !a.d.has("Add Travel") {
        a.click(Find, "More Add actions");
    }
    a.click(Do, "Add Travel");
    a.click(Find, "More actions for Travel");
    a.click(Do, "Roll for encounters");
    type_length(a);
    a.click(Do, "Roll all 2");
    let timeline = timeline_file(a.d);
    a.fact(format!(
        "Timeline.md holds the length and two checks: {}",
        timeline.contains("2 days")
    ));

    a.task("The second day names the Wolf Pack: plan it and start it");
    a.click(Do, &format!("Plan “{WOLVES}”"));
    a.click(Nav, &format!("Start fight: {WOLVES}"));
    a.fact(format!(
        "the note plans the Wolf Pack above the skipped fight: {}",
        note_on_disk(a.d).contains(&format!("- [[{WOLVES}]]\n- [[{SHRINE}]]"))
    ));

    a.task("Roll and begin the Wolf Pack fight");
    counted_roll_and_begin(a);

    a.task("End the Wolf Pack fight");
    counted_end(a, None);
    a.act(Nav, CLOSE_RUN, close_run);

    a.phase = "after";
    a.task("Put the Wolf Pack fight's rewards in the note");
    add_rewards(a, WOLVES);

    a.task("Mark the talk with Warden Ostrel done, and log what happened");
    a.click(Do, &format!("Open: {OSTREL}"));
    a.click(Panel, &format!("Add to Log: {OSTREL}"));
    a.act(Do, "type the line", |d| {
        d.type_text("Handed to the watch at the ford.");
    });
    a.act(Do, "add it", |d| d.press(Key::Enter));
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));
}

/// J3, section 2.3, at 960x640 with Lookup open: a fight ended by mistake, a crash in round 3
/// with an effect pinned to a creature, Lookup after the restart, an edit made in Obsidian, and
/// a removed scene's Undo after the next fight begins (W6-D1 of the first scored round). A real
/// D&D Beyond sync conflict (step 4) is not here: the test build has no route to sync.
fn broken_evening(a: &mut Audit<'_, '_>) {
    a.setup("prep: the Wolf Pack and Ember Watch encounters", |d| {
        encounter(
            d,
            "Bridge Ambush",
            WOLVES,
            "Wolves at the edge of the firelight.",
        );
        encounter(d, SHRINE, WATCH, "The watch holds the gate.");
    });
    a.setup(
        "play last week's session, then quit and open the app",
        last_week,
    );
    a.setup("work at 960x640", |d| d.set_base((960.0, 640.0)));
    a.setup("prep: tonight's plan holds three fights and a scene", |d| {
        tonight(
            d,
            2,
            TONIGHT_SESSION,
            &format!(
                "- [[{SHRINE}]]\n- [[{WOLVES}]]\n- [[{WATCH}]]\n- [ ] Ask the watch about the ropes\n"
            ),
        );
    });
    a.setup("save, quit and open the app at the table", restart);
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

    a.task("End the fight by mistake");
    a.click(Do, "End encounter…");
    a.click(Do, "End encounter");

    a.task("Take the ending back");
    a.click(Do, "Undo: end encounter");
    a.fact(format!(
        "play is back in round 2: {}",
        a.d.has_containing("Round 2")
    ));

    // At 960x640 with Lookup open, the dock has no room for `Effect`, and `E` is off while
    // Lookup is open: the Conditions tab's `Other effect` field is the way.
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
    a.fact(format!(
        "Lookup is open after the effect: {}",
        a.d.app().lookup.open
    ));

    a.task("Play on into round 3");
    for _ in 0..8 {
        a.click(Do, NEXT_TURN);
    }

    a.setup("the app stops in round 3, and opens again", |d| d.restart());
    a.task("After the crash: back to the fight");
    a.fact(format!("the app opened on {}", a.d.current_page()));
    if a.d.current_page() != "Run" {
        a.click(Nav, BACK_TO_RUN);
    }
    a.fact(format!(
        "the wolf's row holds the effect with its count: {}",
        a.d.nodes()
            .iter()
            .any(|node| node.name.starts_with("Cinder Wolf 2") && node.name.contains("Poisoned 2"))
    ));
    a.fact(format!(
        "Undo names the last saved step: {}",
        a.d.name_starting("Undo")
    ));

    a.task("Lookup is closed after the restart: find the note again");
    a.act(Panel, "open Lookup", |d| d.press_ctrl(Key::K));
    if !a.d.visible_text(MILLER) {
        a.act(Find, "search: miller", |d| d.type_text("miller"));
    }
    a.fact(format!(
        "last week's words are in view: {}",
        a.d.visible_text(MILLER)
    ));

    a.setup("in Obsidian, the DM adds a line to tonight's plan", |d| {
        let path = d.app().sessions.selected_path().expect("tonight's note");
        let text = fs::read_to_string(path).expect("read tonight's note");
        fs::write(
            path,
            text.replacen(
                "- [ ] Ask the watch about the ropes\n",
                "- [ ] Ask the watch about the ropes\n- [ ] Pay the ferryman\n",
                1,
            ),
        )
        .expect("write tonight's note");
        d.focus_window(false);
        d.focus_window(true);
        d.settle();
    });
    a.task("The note changed in Obsidian: see it in Splat");
    a.fact(format!(
        "the app holds the file's new line: {}",
        a.d.app().sessions.selected_path().is_some()
            && note_on_disk(a.d).contains("Pay the ferryman")
    ));

    a.task("End the second fight");
    counted_end(a, None);
    a.act(Nav, CLOSE_RUN, close_run);

    a.phase = "after the fight";
    a.task("Take a scene off tonight's plan");
    a.click(Find, "More actions for Ask the watch about the ropes");
    a.click(Do, "Remove from tonight");

    a.task("Begin the fight before it");
    a.click(Nav, &format!("Start fight: {WATCH}"));

    a.task("The scene is wanted after all: take the removal back");
    a.click(Nav, NAV_SESSIONS);
    a.click(Do, "Undo remove");
    a.fact(format!(
        "the scene's line is back in the note: {}",
        note_on_disk(a.d).contains("- [ ] Ask the watch about the ropes\n")
    ));
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

/// J4, section 2.4: the Tournament between fights, and one tracker act in a fight by Ctrl+T,
/// as the owner ruled. Then a day mark that comes due between sessions.
fn the_race(a: &mut Audit<'_, '_>) {
    a.setup("the Tournament's note is in play", race_in_play);
    a.setup("open the app on it", |d| {
        d.restart();
        d.set_base((1200.0, 800.0));
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

    a.task("Finish the bell");
    a.click(Do, "Done: Bell");

    a.task("Keep the seal's notice in mind for the next fight");
    a.fact(format!(
        "the rail says one notice is due: {}",
        a.d.has_containing("1 notice is due")
    ));

    a.task("The party enters the arena");
    a.click(Do, "Enter room");

    a.phase = "the fight";
    a.task("Start the arena fight");
    let start = format!("Start fight: {SHRINE}");
    if !a.d.has(&start) && a.d.has("Plan") {
        a.click(Sub, "Plan");
    }
    a.click(Nav, &start);

    a.task("Roll and begin the arena fight");
    counted_roll_and_begin(a);

    a.task("The crowd loves a move in the fight: mark Crowd Favor");
    a.fact(format!(
        "in the fight, before Ctrl+T: Trackers in view {}, Lookup open {}, on Trackers {}",
        a.d.has("Crowd loves it"),
        a.d.app().lookup.open,
        a.d.app().lookup.on_trackers()
    ));
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));
    if !a.d.has("Crowd loves it") {
        a.act(Panel, "Ctrl+T again: Trackers is not in view", |d| {
            d.press_ctrl(Key::T);
        });
    }
    a.click(Do, "Crowd loves it");
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));

    a.task("End the arena fight");
    counted_end(a, None);
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

    a.task("The crowd loved a move, then threw a gift");
    a.click(Do, "Crowd loves it");
    a.click(Do, "Throw a gift");

    a.task("A wrong press on Throw a gift, seen after the next right press");
    a.click(Do, "Throw a gift");
    a.click(Do, "Crowd loves it");
    a.click(Do, "Crowd loves it");

    a.task("A soul is collected: the clock passes 60");
    a.click(Do, "Collect soul");

    a.task("The second bell");
    for _ in 0..3 {
        a.click(Do, "Roll Rival Track");
    }
    a.click(Do, "Roll Wandering, 0:40 to 1:19");
    a.click(Do, "Done: Bell");
    a.fact(format!(
        "the ledger holds tonight's presses: {}",
        file(a.d, "Trackers.md").matches("why: ").count()
    ));

    a.setup(
        "the Tournament's note has a day mark on the next day",
        |d| {
            let day = crate::timeline::day_now(&d.app().timeline.events).unwrap_or(0) + 1;
            let path = campaign_root(d).join("Tournament.md");
            let text = fs::read_to_string(&path).expect("read the Tournament");
            let mark = format!(
                "> day_marks:\n>   prizes:\n>     name: Prize day\n>     at: {day}\n>     say: Pay each team its prize money.\n>     changes:\n>     - {{tracker: favor, add: 1}}\n"
            );
            fs::write(&path, text.replacen("> trackers:\n", &format!("{mark}> trackers:\n"), 1))
                .expect("write the Tournament");
            d.app_mut().lookup.refresh();
            d.settle();
        },
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
    a.fact(format!(
        "the ledger holds the day mark's Done: {}",
        file(a.d, "Trackers.md").contains("mark: prizes")
    ));
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
        road_table(d, "15-20");
    });
    a.setup(
        "play last week's session, then quit and open the app",
        last_week,
    );
    a.setup("work at 1200x800", |d| d.set_base((1200.0, 800.0)));
    a.setup("prep: the session's plan holds five fights", |d| {
        tonight(
            d,
            2,
            TONIGHT_SESSION,
            &format!(
                "- [[{SHRINE}]]\n- [[{WOLVES}]]\n- [[{WATCH}]]\n- [[{GATE}]]\n- [[{PATROL}]]\n"
            ),
        );
        let body = d.editor_text().expect("tonight's note");
        d.replace_editor_text(&body.replacen(
            "## What happened\n",
            "## What happened\n\nThe party crossed the ford and burned the shrine.\n",
            1,
        ));
        d.press_ctrl(Key::S);
        show_plan(d);
    });
    a.setup("save, quit and open the app at the table", restart);
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
    a.fact(format!(
        "his note holds the status: {}, and a Log line for it: {}",
        file(a.d, "NPCs/Verori.md").contains("status: dead"),
        file(a.d, "NPCs/Verori.md").contains("→ dead")
    ));

    a.task("He gave the gate key away first");
    a.click(Do, "Add to Log");
    a.act(Do, "type the line", |d| {
        d.type_text("Gave the gate key to Iselle.");
    });
    a.act(Do, "add it", |d| d.press(Key::Enter));
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));

    a.task("Back to the evening");
    a.act(Nav, CLOSE_RUN, close_run);

    a.task("Rewards for each of the three fights");
    for fight in [SHRINE, WOLVES, WATCH] {
        add_rewards(a, fight);
    }
    a.fact(format!(
        "the note holds three blocks, each with its XP line: {}",
        note_on_disk(a.d).matches("\n  - XP ").count()
    ));

    a.setup("a week passes: save, quit and open the app", restart);
    a.phase = "a week later";

    a.task("Start the new session");
    a.click(Do, "New session");
    a.act(Do, "type the title", |d| d.type_text("The North Gate"));
    a.click(Do, "Create session");

    a.task("Carry over what the last session left");
    a.click(Do, "Carry over");
    a.fact(format!(
        "the new note holds the recap, the unplayed fights and carried_from: {}",
        note_on_disk(a.d).contains("carried_from: ")
            && note_on_disk(a.d).contains(&format!("[[{GATE}]]"))
    ));

    a.task("Plan three scenes");
    a.act(Do, "click under the plan's last line", |d| {
        d.click_line_end(&format!("- [[{PATROL}]]"));
    });
    for scene in [
        "Meet the gate's new keeper",
        "Search Verori's rooms",
        "The ferryman's warning",
    ] {
        a.act(Do, "type a scene", |d| {
            d.type_text(&format!("\n- [ ] {scene}"))
        });
    }
    a.fact(format!(
        "the plan shows the three scenes: {}",
        a.d.has_containing("Open: The ferryman's warning")
    ));

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
    let on_rows = words(a.d);
    a.fact(format!("the planned fights' rows say {on_rows:?}"));
    if on_rows.len() < 2 {
        a.click(Find, GATE);
        a.click(Find, GATE);
    }

    // The plan has no way into the encounter editor: the DM goes by Encounters.
    a.task("The Gate Watch is too easy: add a creature in the encounter editor");
    a.click(Nav, NAV_ENCOUNTERS);
    a.act(Find, "Edit beside Gate Watch", |d| {
        d.click_beside_exact(GATE, EDIT_ENCOUNTER);
    });
    a.act(Find, "search: ogre", |d| d.type_into(FIND_CREATURE, "ogre"));
    a.act(Do, "add an Ogre", |d| d.click_first(ADD));
    a.click(Do, SAVE_ENCOUNTER);
    a.click(Nav, NAV_SESSIONS);
    a.fact(format!("the row's word after the change: {:?}", words(a.d)));

    a.task("Prepare the road: a Travel row with its checks");
    if !a.d.has("Add Travel") {
        a.click(Find, "More Add actions");
    }
    a.click(Do, "Add Travel");
    a.click(Find, "More actions for Travel");
    a.click(Do, "Roll for encounters");
    type_length(a);
    a.act(Do, "keep it with Enter", |d| d.press(Key::Enter));

    a.task("Start the first fight");
    a.click(Nav, &format!("Start fight: {GATE}"));
}
