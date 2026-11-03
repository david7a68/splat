//! A scripted evening of play on the real app, for `artifacts/dm-audit/`: prep at the desk,
//! a session at the table, and the wrap-up. Every page change is seen by the driver, so a
//! count cannot miss one. Not part of the visual suite, and it enforces nothing.
//!
//! Each test is ignored, so `cargo test` never runs it, and each needs its own process:
//! `cargo test --locked --bin splat visual_tests::dm_audit::dm_audit -- --exact --ignored --nocapture`
//! writes `artifacts/dm-audit/steps.tsv` and `summary.tsv`. With `SPLAT_VISUAL_DIR` it also
//! renders the captures that the report refers to.
use super::{
    driver::{Driver, Sizes},
    fixture::{Seed, World},
    steps::*,
};
use eframe::egui::Key;
use std::{
    fmt::Write as _,
    fs,
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
};

/// What an interaction was for. Only `Do` changes the game or the record.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    /// Go to another top-level page.
    Nav,
    /// Change a tab or a sub-view inside a page.
    Sub,
    /// Open or close a panel beside the page.
    Panel,
    /// Search for or select something, to reach it.
    Find,
    /// The work itself.
    Do,
}
use Kind::{Do, Find, Nav, Panel, Sub};

const TONIGHT: &str = "Ashfall Crossing";
const SECOND: &str = "Shrine of Cinders";
const IMPROVISED: &str = "Tavern brawl";
const COPY: &str = "Ogre copy";
const PLAYERS: [(&str, &str); 5] = [
    ("Brannoch Vale", "15"),
    ("Iselle Marrowind", "19"),
    ("Kestrel Dunmoor", "21"),
    ("Oren Thistlewick", "11"),
    ("Sable Harrowgate", "13"),
];

struct Row {
    phase: &'static str,
    task: String,
    kind: Kind,
    what: String,
    interactions: u32,
    pages: Vec<String>,
    from: String,
    blocked: bool,
}

struct Audit<'d, 'a> {
    d: &'d mut Driver<'a>,
    phase: &'static str,
    task: String,
    rows: Vec<Row>,
    facts: Vec<String>,
    dir: PathBuf,
    blocks: usize,
}

impl<'a> Audit<'_, 'a> {
    fn task(&mut self, name: &str) {
        self.task = name.to_owned();
    }

    /// Do one thing as the DM would, and record what it cost.
    fn act(&mut self, kind: Kind, what: &str, step: impl FnOnce(&mut Driver<'a>)) {
        let before = self.d.interaction_count();
        let trail = self.d.page_trail.len();
        let from = self.d.current_page();
        let result = catch_unwind(AssertUnwindSafe(|| step(self.d)));
        let blocked = result.is_err();
        if blocked {
            self.blocks += 1;
            let text = self.d.describe();
            fs::write(
                self.dir.join(format!("blocked-{:02}.txt", self.blocks)),
                format!("{} / {what}\n{text}", self.task),
            )
            .expect("write what was on screen");
        }
        self.rows.push(Row {
            phase: self.phase,
            task: self.task.clone(),
            kind,
            what: what.to_owned(),
            interactions: self.d.interaction_count() - before,
            pages: self.d.page_trail[trail..]
                .iter()
                .map(|(_, page)| page.clone())
                .collect(),
            from,
            blocked,
        });
    }

    fn click(&mut self, kind: Kind, name: &str) {
        self.act(kind, name, |d| d.click(name));
    }

    fn fact(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.facts
            .push(format!("{} / {}: {text}", self.phase, self.task));
    }

    fn shot(&mut self, name: &str) {
        let name = name.to_owned();
        let _ = catch_unwind(AssertUnwindSafe(|| self.d.state(&name, Sizes::Default)));
    }
}

/// Save what waits for its pause, as the real app would have by now. Test builds save
/// only when told to.
fn restart(d: &mut Driver<'_>) {
    d.app_mut().sessions.flush();
    d.app_mut().timeline.save();
    d.settle();
    d.restart();
}

fn hp(d: &Driver<'_>, name: &str) -> String {
    d.nodes()
        .iter()
        .find(|node| node.name.contains(name) && node.name.contains("HP "))
        .map_or_else(String::new, |node| node.name.clone())
}

/// Everything the table did before the evening starts: the last session is in the past.
fn last_week(d: &mut Driver<'_>) {
    d.click(NAV_SESSIONS);
    new_session(d, "The Road to Emberfall");
    d.type_into_editor(
        "\nThe miller Hobb told the party the bridge toll collectors have not been paid.\n",
    );
    d.press_ctrl(Key::S);
    d.click(NAV_TIMELINE);
    d.click("Add Day");
    d.click(NAV_ENCOUNTERS);
    restart(d);
}

fn before_the_session(a: &mut Audit<'_, '_>) {
    a.phase = "before";

    a.task("Start tonight's session note");
    a.click(Nav, NAV_SESSIONS);
    a.click(Do, "New session");
    a.act(Do, "type the title", |d| d.type_text("The Crossing"));
    a.click(Do, "Create session");

    a.task("Recall what happened last session, and where the story stands");
    a.d.expect("Last time:");
    a.shot("before-recap");

    a.task("Compare the party's passive Perception, to plan an ambush");
    a.click(Nav, NAV_PARTY);
    a.fact(format!(
        "Party compares without opening a sheet: {}",
        a.d.has_containing("Per 14")
    ));
    a.shot("before-party");

    a.task("Build tonight's encounter");
    a.click(Nav, NAV_SESSIONS);
    a.click(Find, "Add a fight");
    a.click(Nav, "New encounter");
    a.act(Do, "type the name", |d| d.type_text(TONIGHT));
    a.act(Do, "go on to the search", |d| d.press(Key::Enter));
    a.act(Find, "search: wolf", |d| d.type_text("wolf"));
    a.act(Find, "read the Wolf stat block", |d| d.click("Wolf"));
    a.act(Panel, "close the stat block", |d| d.press(Key::Escape));
    a.act(Do, "add a Wolf", |d| d.click_first(ADD));
    a.act(Do, "duplicate it", |d| {
        d.click_below("6. ", "More actions for Wolf");
        d.click("Duplicate");
    });
    a.act(Find, "search: ogre", |d| {
        d.replace_text_in(FIND_CREATURE, "ogre");
    });
    a.act(Do, "add an Ogre", |d| d.click_first(ADD));
    a.click(Do, SAVE_ENCOUNTER);
    a.shot("before-encounter-saved");

    a.task("Make the boss your own: copy it, change it, and swap it in");
    a.act(Do, "Make it yours", |d| {
        d.click_first("More actions for Ogre");
        d.click("Make it yours");
    });
    a.act(Do, "change its hit points", |d| {
        d.type_text("90");
    });
    a.act(Do, "save the creature", |d| d.press_ctrl(Key::S));
    a.click(Do, SAVE_ENCOUNTER);
    a.fact(format!(
        "the roster has a row for the copy: {}",
        a.d.has_label(&format!("8. {COPY}"))
    ));

    a.task("Write the read-aloud text and tactics for the encounter");
    a.act(Do, "write the text", |d| {
        d.type_into(
            "Read-aloud and notes",
            "Ash falls on the crossing. The wolves circle first.",
        );
    });
    a.act(Do, "save", |d| d.press_ctrl(Key::S));

    a.task("Back to the session plan, with tonight's encounter in it");
    a.act(Nav, "Back to session", |d| d.click("Back to session"));
    a.click(Find, "Add a fight");
    a.act(Do, "plan the second fight", |d| {
        d.click(&format!("Plan {SECOND}"));
    });
    a.fact(format!(
        "the plan offers tonight's fight: {}",
        a.d.has(&format!("Start fight: {TONIGHT}"))
    ));
    a.shot("before-plan");
}

/// One turn. The player or monster acts, and the DM records the result.
fn play_turn(a: &mut Audit<'_, '_>, enemies: &[&str], heroes: &[&str], turn: usize) {
    let current = current_participant(a.d).unwrap_or_default();
    let target = if heroes.iter().any(|hero| current.contains(hero)) {
        enemies[turn % enemies.len()]
    } else {
        heroes[turn % heroes.len()]
    };
    let amount = if heroes.contains(&target) { "2" } else { "4" };
    a.act(Find, "select the target", |d| d.click_row(target));
    a.act(Do, "type the damage", |d| d.type_text(amount));
    a.act(Do, "apply with Enter", |d| d.press(Key::Enter));
    a.click(Do, NEXT_TURN);
}

fn setup_fight(a: &mut Audit<'_, '_>) {
    a.click(Do, "Roll NPC initiative");
    for (index, (name, total)) in PLAYERS.iter().enumerate() {
        a.act(Do, &format!("type {name}'s initiative"), |d| {
            if d.focused_name().as_deref() == Some(format!("Initiative for {name}").as_str()) {
                d.type_text(total);
            } else {
                d.type_into(&format!("Initiative for {name}"), total);
            }
        });
        if index + 1 < PLAYERS.len() {
            a.act(Do, "next field with Enter", |d| d.press(Key::Enter));
        }
    }
}

fn during_the_session(a: &mut Audit<'_, '_>) {
    a.phase = "during";
    restart(a.d);
    a.task("Open the app at the table");
    a.fact(format!("the app reopened on {}", a.d.current_page()));

    a.task("Start recording");
    a.d.app_mut().processing.simulate_input("table.mic");
    a.d.settle();
    a.click(Do, "Record this session");
    a.click(Do, "Start recording");
    a.d.app_mut()
        .sessions
        .simulate_recorder_health(crate::recording::Health::Hearing);
    a.d.settle();

    a.task("Read last session's recap aloud");
    a.d.expect("Last time:");

    a.task("A player asks who Captain Varra is");
    a.act(Panel, "open Lookup", |d| d.press_ctrl(Key::K));
    a.act(Find, "search: varra", |d| d.type_text("varra"));
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));

    a.task("Note what the party decides");
    a.act(Panel, "go to the note", |d| d.press_ctrl(Key::J));
    a.act(Do, "type the note", |d| d.type_text("They take the ford."));
    a.fact(format!(
        "the words reached the note: {}",
        a.d.editor_text()
            .is_some_and(|text| text.contains("They take the ford."))
    ));

    a.task("The party travels for half a day: record it");
    a.click(Do, "Add Travel");
    a.shot("during-plan");

    a.task("Ambush: read the read-aloud text and start the fight");
    a.click(Nav, &format!("Start fight: {TONIGHT}"));
    a.fact(format!(
        "setup shows the read-aloud text: {}",
        a.d.visible_text("Ash falls on the crossing.")
    ));
    a.shot("during-setup");
    setup_fight(a);
    a.click(Do, "Begin combat");
    a.shot("during-live");

    let heroes: Vec<&str> = PLAYERS.iter().map(|(name, _)| *name).collect();
    let enemies = ["Wolf 1", "Wolf 2", COPY];

    a.task("Combat: round 1");
    for turn in 0..8 {
        play_turn(a, &enemies, &heroes, turn);
    }

    a.task("A player casts Fireball: read the spell");
    a.act(Panel, "open Lookup", |d| d.press_ctrl(Key::K));
    a.act(Find, "search: fireball", |d| d.type_text("fireball"));
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));

    a.task("Fireball hits three enemies: apply half to one, full to two");
    a.act(Find, "select Wolf 1", |d| d.click_row("Wolf 1"));
    a.act(Find, "add Wolf 2", |d| d.ctrl_click_row("Wolf 2"));
    a.act(Find, "add the boss", |d| d.ctrl_click_row(COPY));
    a.act(Do, "half for Wolf 1", |d| d.click("Half damage: Wolf 1"));
    a.act(Do, "type the damage", |d| d.type_text("28"));
    a.act(Do, "apply with Enter", |d| d.press(Key::Enter));
    a.fact(format!(
        "one step of undo for all three: {}",
        a.d.has_containing("to 3 participants")
    ));
    a.d.press(Key::U);

    a.task("A wolf frightens Oren: mark the condition");
    a.act(Find, "select Oren", |d| d.click_row("Oren Thistlewick"));
    a.click(Sub, CONDITIONS);
    a.click(Do, "Frightened");

    a.task("What does Frightened do?");
    a.act(Panel, "open the rule from the row", |d| {
        d.click("Rule: Frightened")
    });
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));

    a.task("Iselle starts concentrating: mark it");
    a.act(Find, "select Iselle", |d| d.click_row("Iselle Marrowind"));
    a.click(Do, "Concentrating");

    a.task("Combat: round 2");
    for turn in 0..8 {
        play_turn(a, &enemies, &heroes, turn + 2);
    }

    a.task("Check Iselle's spell save DC and spell list");
    a.act(Find, "select Iselle", |d| d.click_row("Iselle Marrowind"));
    a.click(Sub, "Spells");
    a.act(Find, "select the current turn again", |d| {
        d.press(Key::Escape)
    });

    a.task("Roll a save for a monster");
    a.act(Do, "type the dice", |d| d.replace_text_in("Dice", "1d20+3"));
    a.act(Do, "roll with Enter", |d| d.press(Key::Enter));
    a.fact(format!(
        "the roll is beside the amount, with the stat block or sheet in view: {}",
        a.d.has("Dice") && (a.d.has("Stat block") || a.d.has("Sheet"))
    ));

    a.task("On a player's turn, read a monster's reaction");
    a.act(Find, "select the Wolf", |d| d.click_row("Wolf 1"));
    a.act(Find, "select the current turn again", |d| {
        d.press(Key::Escape)
    });

    a.task("A player asks what the miller said last session");
    a.act(Panel, "open Lookup", |d| d.press_ctrl(Key::K));
    a.act(Find, "search: miller", |d| d.type_text("miller"));
    a.fact(format!(
        "Lookup finds last session's notes: {}",
        a.d.visible_text("The miller Hobb told the party")
    ));
    a.shot("during-lookup-notes");
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));

    a.task("Note a moment beside the fight");
    a.act(Panel, "open Notes", |d| d.press_ctrl(Key::J));
    a.act(Do, "type the note", |d| {
        d.type_text("The boss howled and fled.")
    });
    a.act(Panel, "close Notes", |d| d.press(Key::Escape));

    a.task("Combat: round 3");
    for turn in 0..8 {
        play_turn(a, &enemies, &heroes, turn + 4);
    }

    a.task("A wolf is dead: take it out of the order");
    a.act(Find, "select Wolf 2", |d| d.click_row("Wolf 2"));
    a.click(Do, "Absent");

    let hurt = hp(a.d, "Kestrel Dunmoor");
    a.task("End the fight");
    a.click(Do, "End encounter…");
    a.act(Do, "type the outcome", |d| {
        d.type_text("The party held the ford")
    });
    a.act(Do, "end with Enter", |d| d.press(Key::Enter));
    a.shot("during-ended");
    a.click(Nav, CLOSE_RUN);
    a.fact(format!(
        "after the fight, the page is {}",
        a.d.current_page()
    ));

    a.task("Record a short rest");
    a.click(Do, "Add Short rest");

    a.task("Next fight: start it, with the party as it stands");
    a.click(Nav, &format!("Start fight: {SECOND}"));
    a.fact(format!(
        "Kestrel at the end of the last fight: [{hurt}]. At setup of the next: [{}]",
        hp(a.d, "Kestrel Dunmoor")
    ));
    a.shot("during-second-setup");

    a.task("Next fight: roll and begin");
    setup_fight(a);
    a.click(Do, "Begin combat");

    a.task("Next fight: end it after a short exchange");
    a.click(Do, "End encounter…");
    a.click(Do, "End encounter");
    a.click(Nav, CLOSE_RUN);

    a.task("The players start a fight you did not plan, and then talk their way out");
    a.click(Find, "Add a fight");
    a.click(Nav, "New encounter");
    a.act(Do, "type the name", |d| d.type_text(IMPROVISED));
    a.act(Do, "go on to the search", |d| d.press(Key::Enter));
    a.act(Find, "search: bandit", |d| d.type_text("bandit"));
    a.act(Do, "add a Bandit", |d| d.click_first(ADD));
    a.click(Nav, RUN_ENCOUNTER);
    a.fact(format!(
        "the improvised run is open: {}",
        a.d.current_page()
    ));
    a.act(Nav, "close the run again", close_run);
    a.fact(format!("closing it comes back to: {}", a.d.current_page()));

    a.task("Stop recording");
    a.click(Do, "Stop recording");
}

fn after_the_session(a: &mut Audit<'_, '_>) {
    a.phase = "after";
    restart(a.d);
    a.fact(format!("the app reopened on {}", a.d.current_page()));
    a.d.app_mut().processing.simulate_keys();
    a.d.settle();

    a.task("Review how the fight went: log and final HP");
    a.act(Find, "open the fight", |d| d.click(TONIGHT));
    a.fact(format!(
        "final HP reads in place: {}",
        a.d.has_label("FINAL HP") || a.d.visible_text("FINAL HP")
    ));
    a.shot("after-fight-in-place");

    a.task("Say where the fight happened");
    a.click(Sub, "Details");
    a.act(Do, "type where it happened", |d| {
        d.type_into("Where", "The ford");
    });

    a.task("Process the recording and read the summary");
    a.click(Sub, "Recordings (1)");
    a.act(Do, "Process", |d| d.click_containing("Process:"));
    a.shot("after-processed");
    a.click(Sub, "Summary");
    a.click(Sub, "Back to recordings");

    a.task("Add a suggested event to the timeline");
    a.act(Do, "add the marker", |d| {
        d.click_containing("Add to timeline: Rest at the mill");
    });

    a.task("Write the wrap-up in the note");
    a.click(Sub, "Plan and notes");
    a.act(Do, "type the wrap-up", |d| {
        d.type_into_editor("\nThe party held the ford. Next: the shrine.\n");
    });

    a.task("Link the fight in the note");
    a.act(Do, "Add to notes", |d| {
        d.click_containing(&format!("Add to notes: {TONIGHT}"));
    });

    a.task("Check the party's HP for next time");
    a.click(Nav, NAV_PARTY);
    a.fact(format!(
        "Kestrel on Party after the evening: [{}]",
        a.d.nodes()
            .iter()
            .find(|node| node.name.contains("Kestrel Dunmoor ·"))
            .map_or_else(String::new, |node| node.name.clone())
    ));
    a.shot("after-party");
}

fn summary(rows: &[Row], facts: &[String]) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Phase totals\nphase\ttasks\ttransitions\tinteractions\tnav\tsub\tpanel\tfind\tdo"
    );
    for phase in ["before", "during", "after"] {
        let of: Vec<&Row> = rows.iter().filter(|row| row.phase == phase).collect();
        let count = |kind| {
            of.iter()
                .filter(|row| row.kind == kind)
                .map(|row| row.interactions)
                .sum::<u32>()
        };
        let mut tasks: Vec<&str> = of.iter().map(|row| row.task.as_str()).collect();
        tasks.dedup();
        let _ = writeln!(
            out,
            "{phase}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            tasks.len(),
            of.iter().map(|row| row.pages.len()).sum::<usize>(),
            of.iter().map(|row| row.interactions).sum::<u32>(),
            count(Nav),
            count(Sub),
            count(Panel),
            count(Find),
            count(Do),
        );
    }
    let _ = writeln!(
        out,
        "\nTasks\nphase\ttask\ttransitions\tinteractions\toverhead\tpath"
    );
    let mut tasks: Vec<(&str, &str)> = rows.iter().map(|r| (r.phase, r.task.as_str())).collect();
    tasks.dedup();
    for (phase, task) in tasks {
        let of: Vec<&Row> = rows
            .iter()
            .filter(|r| r.phase == phase && r.task == task)
            .collect();
        let overhead: u32 = of
            .iter()
            .filter(|r| r.kind != Do)
            .map(|r| r.interactions)
            .sum();
        let path: Vec<String> = of.iter().flat_map(|r| r.pages.iter().cloned()).collect();
        let start = of.first().map_or("", |r| r.from.as_str());
        let _ = writeln!(
            out,
            "{phase}\t{task}\t{}\t{}\t{overhead}\t{start} > {}",
            of.iter().map(|r| r.pages.len()).sum::<usize>(),
            of.iter().map(|r| r.interactions).sum::<u32>(),
            path.join(" > "),
        );
    }
    let _ = writeln!(out, "\nBlocked");
    for row in rows.iter().filter(|row| row.blocked) {
        let _ = writeln!(out, "{}\t{}\t{}", row.phase, row.task, row.what);
    }
    let _ = writeln!(out, "\nFacts");
    for fact in facts {
        let _ = writeln!(out, "{fact}");
    }
    out
}

/// An encounter saved on Encounters, then edited in the Library. What does the Library write?
#[test]
#[ignore = "an audit, run by hand"]
fn dm_audit_stale_library() {
    let root = std::env::temp_dir().join("splat-dm-audit-stale");
    let _ = fs::remove_dir_all(&root);
    let cache = root.join("character-cache");
    fs::create_dir_all(&cache).expect("create the cache");
    crate::hermetic::closed::use_cache_root(cache.clone());
    crate::hermetic::closed::repeatable_uuids(78u128 << 64);
    let world = World::build(&root.join("w"), &cache, "dm-audit-stale", Seed::Campaign)
        .expect("build the world");
    let mut driver = Driver::new(&world, "dm-audit-stale", &root.join("w"), None, false);
    let d = &mut driver;
    d.set_base((1200.0, 800.0));
    // The DM has the Library open once, then changes the roster on Encounters.
    d.click(NAV_LIBRARY);
    d.click(NAV_ENCOUNTERS);
    open_saved_encounter(d, "Bridge Ambush");
    d.expect("8. ");
    let before = d.has_label("8. Ember Cultist");
    d.type_into(FIND_CREATURE, "ogre");
    d.click_first(ADD);
    d.expect("9. ");
    d.click(SAVE_ENCOUNTER);
    println!("STALE: roster had the cultist: {before}. Ogre added and saved.");
    // Then the DM adds text from the Library.
    d.click(NAV_LIBRARY);
    d.click("Section");
    d.click("Library section: Encounters");
    d.click_containing("Bridge Ambush");
    d.click(EDIT_MARKDOWN);
    let draft_has_ogre = d
        .editor_text()
        .is_some_and(|text| text.matches("key: actor_").count() >= 9);
    println!("STALE: the Library's draft has the ninth roster slot: {draft_has_ogre}");
    d.type_into_editor("Read aloud: ash falls.\n");
    d.press_ctrl(Key::S);
    let conflict = d.has("Overwrite with my draft");
    println!("STALE: Ctrl+S says changed on disk: {conflict}");
    if conflict {
        d.click("Overwrite with my draft");
        if d.has("Overwrite with my draft") {
            d.click("Overwrite with my draft");
        }
    }
    println!(
        "STALE: after the Library's save, still unsaved: {}",
        d.has_containing(UNSAVED_NOTE)
    );
    d.click(NAV_ENCOUNTERS);
    if d.has("Back to encounters") {
        d.click("Back to encounters");
    }
    if d.has("Reload") {
        d.click("Reload");
    }
    open_saved_encounter(d, "Bridge Ambush");
    println!(
        "STALE: after overwrite, the roster has row 9: {}",
        d.has_containing("9. ")
    );
}

#[test]
#[ignore = "an audit, run by hand"]
fn dm_audit() {
    let dir = Path::new("artifacts/dm-audit");
    fs::create_dir_all(dir).expect("create the audit folder");
    let root = std::env::temp_dir().join("splat-dm-audit");
    let _ = fs::remove_dir_all(&root);
    let cache = root.join("character-cache");
    fs::create_dir_all(&cache).expect("create the cache");
    crate::hermetic::closed::use_cache_root(cache.clone());
    crate::hermetic::closed::repeatable_uuids(77u128 << 64);
    let world = World::build(&root.join("evening"), &cache, "dm-audit", Seed::Campaign)
        .expect("build the world");
    let output = std::env::var_os("SPLAT_VISUAL_DIR").map(PathBuf::from);
    if let Some(output) = &output {
        fs::create_dir_all(output).expect("create the capture folder");
    }
    let mut driver = Driver::new(
        &world,
        "dm-audit",
        &root.join("evening"),
        output.as_deref(),
        false,
    );
    driver.set_base((1200.0, 800.0));
    last_week(&mut driver);
    driver.set_base((1200.0, 800.0));
    let mut audit = Audit {
        d: &mut driver,
        phase: "before",
        task: String::new(),
        rows: Vec::new(),
        facts: Vec::new(),
        dir: PathBuf::from("/tmp/dm-audit-blocked"),
        blocks: 0,
    };
    let _ = fs::remove_dir_all(&audit.dir);
    fs::create_dir_all(&audit.dir).expect("create the blocked folder");
    before_the_session(&mut audit);
    during_the_session(&mut audit);
    after_the_session(&mut audit);

    let mut steps = String::from("phase\ttask\tkind\twhat\tinteractions\tfrom\tpages\tblocked\n");
    for row in &audit.rows {
        let _ = writeln!(
            steps,
            "{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}",
            row.phase,
            row.task,
            row.kind,
            row.what,
            row.interactions,
            row.from,
            row.pages.join(">"),
            row.blocked
        );
    }
    fs::write(dir.join("steps.tsv"), steps).expect("write steps");
    let text = summary(&audit.rows, &audit.facts);
    fs::write(dir.join("summary.tsv"), &text).expect("write summary");
    println!("{text}");
}
