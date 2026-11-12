//! A scripted evening of play on the real app, for `artifacts/dm-audit/`: prep at the desk,
//! a session at the table, and the wrap-up. Every page change is seen by the driver, so a
//! count cannot miss one.
//!
//! The scenario `dm-audit` is G6. It plays the frozen evening and fails when a task's page
//! transitions, interactions or path differ from `artifacts/dm-audit/counts.json`, when a task
//! is missing, new or out of order, or when a counted act finds no control.
//!
//! G6 runs in every gate. A change that makes a task cost more or fewer acts fails it; when that
//! is meant, its author runs `devenv shell -- audit-freeze counts`, which writes every row from
//! this run, and says in the pull request why. A feature that adds its own acts adds them as new
//! tasks and runs `devenv shell -- audit-freeze add`: that writes a row for each new task and
//! leaves every frozen row as it is; it refuses, and writes nothing, when a frozen task's
//! transitions, interactions or path differ, or when a frozen task is missing or out of order.
//!
//! Set-up acts make the fixture ready. They are listed in `steps.tsv` as `Setup` and never
//! counted, so a set-up change cannot move a count.
//!
//! `dm_audit_stale_library` is a separate audit, ignored, run by hand in its own process.
use super::{
    driver::Driver,
    fixture::{Seed, World},
    steps::*,
};
use eframe::egui::Key;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Write as _,
    fs,
    panic::{AssertUnwindSafe, catch_unwind},
    path::{Path, PathBuf},
};

/// `all` writes the frozen files from this run; `add` writes rows for new tasks only. Either
/// takes the place of the G6 check.
const WRITE: &str = "SPLAT_DM_AUDIT_WRITE";
/// The road table of the journey's encounter checks (F9).
const ROAD_TABLE: &str = "# Road\n\n## On the road\n\n<!-- splat:rolls key=road dice=1d20 per=day -->\n\n| d20 | On the road |\n| --- | --- |\n| 1-14 | Nothing stirs. |\n| 15-20 | Wolves pick up the trail. |\n";
/// A house rule note with one tracker and one quick action (F4).
fn torch_rules(id: uuid::Uuid) -> String {
    format!(
        "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: rule\nsplat_name: Torch Rules\n---\n\n<!-- splat:yaml path=splat_members -->\n\n```yaml\ntrackers:\n  torches:\n    name: Torches\n    start: 6\n    min: 0\n    pin: true\n    quick:\n    - label: Burn a torch\n      add: -1\n```\n\n<!-- splat:yaml path=splat_rule -->\n\n```yaml\nmechanic: house:torches\nmode: augment\nimplementation: splat:house_rules\nparameters: {{}}\n```\n"
    )
}

/// The one way a feature adds its tasks.
const ADD_COMMAND: &str = "devenv shell -- audit-freeze add";
/// How an author records counts that change on purpose.
const COUNTS_COMMAND: &str = "devenv shell -- audit-freeze counts";
/// What a new task's line says, so that `add` knows the lines it may write.
const NEW_TASK: &str = "a new task, not in counts.json";

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
    /// Making the fixture ready. Never counted.
    Setup,
}
use Kind::{Do, Find, Nav, Panel, Sub};

const TONIGHT: &str = "Ashfall Crossing";
const SECOND: &str = "Shrine of Cinders";
const IMPROVISED: &str = "Tavern brawl";
const COPY: &str = "Ogre copy";
const LAST_WEEK: &str = "The Road to Emberfall";
const TONIGHT_SESSION: &str = "The Crossing";
/// What last week's note says, and what the Last time line and Lookup show of it.
const MILLER: &str =
    "The miller Hobb told the party the bridge toll collectors have not been paid.";
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
    /// Why the act found no control, if it did not.
    blocked: Option<String>,
}

/// What one evening measured.
#[derive(Default)]
struct Record {
    rows: Vec<Row>,
    facts: Vec<String>,
    /// Every counted task in order, also one that needs no act, with the page it starts on.
    tasks: Vec<(&'static str, String, String)>,
}

/// One task's measured cost. `counts.json` holds these, and G6 compares them.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
struct TaskCount {
    phase: String,
    task: String,
    transitions: usize,
    interactions: u32,
    /// The pages the task went through, from where it started.
    path: String,
}

#[derive(Serialize, Deserialize)]
struct Frozen {
    about: String,
    tasks: Vec<TaskCount>,
}

struct Audit<'d, 'a> {
    d: &'d mut Driver<'a>,
    phase: &'static str,
    task: String,
    record: Record,
}

impl<'a> Audit<'_, 'a> {
    fn task(&mut self, name: &str) {
        assert!(
            !self
                .record
                .tasks
                .iter()
                .any(|(phase, task, _)| *phase == self.phase && task == name),
            "the audit has two tasks named {name:?}"
        );
        self.task = name.to_owned();
        let start = self.d.current_page();
        self.record.tasks.push((self.phase, name.to_owned(), start));
    }

    /// Do one thing as the DM would, and record what it cost.
    fn act(&mut self, kind: Kind, what: &str, step: impl FnOnce(&mut Driver<'a>)) {
        let before = self.d.interaction_count();
        let trail = self.d.page_trail.len();
        let from = self.d.current_page();
        let result = catch_unwind(AssertUnwindSafe(|| step(self.d)));
        let blocked = result
            .err()
            .map(|panic| super::panic_line(panic.as_ref()).to_owned());
        let task = if kind == Kind::Setup {
            "Set-up".to_owned()
        } else {
            self.task.clone()
        };
        self.record.rows.push(Row {
            phase: self.phase,
            task,
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

    /// Make the fixture ready. Listed, never counted; a failure stops the evening.
    fn setup(&mut self, what: &str, step: impl FnOnce(&mut Driver<'a>)) {
        self.act(Kind::Setup, what, step);
        if let Some(why) = &self.record.rows.last().and_then(|row| row.blocked.clone()) {
            panic!("dm-audit: the set-up step {what:?} failed: {why}");
        }
    }

    fn click(&mut self, kind: Kind, name: &str) {
        self.act(kind, name, |d| d.click(name));
    }

    fn fact(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.record
            .facts
            .push(format!("{} / {}: {text}", self.phase, self.task));
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
    new_session(d, LAST_WEEK);
    d.type_into_editor(&format!("\n{MILLER}\n"));
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
    a.act(Do, "type the title", |d| d.type_text(TONIGHT_SESSION));
    a.click(Do, "Create session");

    a.task("Recall what happened last session, and where the story stands");
    recall_last_session(a);

    a.task("Compare the party's passive Perception, to plan an ambush");
    a.click(Nav, NAV_PARTY);
    a.fact(format!(
        "Party compares without opening a sheet: {}",
        a.d.has_containing("Per 14")
    ));

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

    // F1: the word ends each planned fight's summary. A summary with no room for it gives it on
    // the opened row's line, for one Find, and one more to close the row again.
    a.task("Is each planned fight right for this party?");
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
        a.click(Find, TONIGHT);
        a.fact(format!(
            "the opened row says {:?}",
            a.d.painted_in(eframe::egui::Rect::EVERYTHING)
                .split("Read encounter")
                .nth(1)
                .unwrap_or_default()
                .lines()
                .next()
        ));
        a.click(Find, TONIGHT);
    }
}

/// The Last time line previews only a recap or `What happened`, and last week's words are under
/// another heading. So the DM opens the last session from its link, reads, and comes back.
fn recall_last_session(a: &mut Audit<'_, '_>) {
    a.d.expect("Last time:");
    a.fact(format!(
        "the Last time line shows last week's words with no act: {}",
        a.d.visible_text(MILLER)
    ));
    a.act(Find, "open last session from Last time", |d| {
        d.click_link(&format!("Session 1: {LAST_WEEK}"));
    });
    a.fact(format!(
        "last week's words are in view: {}",
        a.d.visible_text(MILLER)
    ));
    a.act(Find, "back to tonight's session", |d| {
        d.click_starting(&format!("Session 2: {TONIGHT_SESSION}"));
    });
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
    a.setup("save, quit and open the app at the table", restart);
    a.task = "Open the app at the table".into();
    a.fact(format!("the app reopened on {}", a.d.current_page()));

    a.task("Start recording");
    a.setup("a microphone is there", |d| {
        d.app_mut().processing.simulate_input("table.mic");
        d.settle();
    });
    a.click(Do, "Record this session");
    a.click(Do, "Start recording");
    a.setup("the recorder hears the table", |d| {
        d.app_mut()
            .sessions
            .simulate_recorder_health(crate::recording::Health::Hearing);
        d.settle();
    });

    a.task("Read last session's recap aloud");
    recall_last_session(a);

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
    if !a.d.has("Add Travel") {
        a.click(Find, "More Add actions");
    }
    a.click(Do, "Add Travel");

    // F9: one roll on the campaign's road table for the journey, in the Travel row.
    a.setup("the campaign has a marked road table", |d| {
        let campaign = d.app_mut().campaigns.active_campaign().expect("a campaign");
        fs::write(campaign.path.join("Road.md"), ROAD_TABLE).expect("write the road table");
        d.app_mut().lookup.refresh();
        d.settle();
    });
    a.task("Check the road for encounters");
    a.click(Find, "More actions for Travel");
    a.click(Do, "Roll for encounters");
    a.click(Do, "Roll for This journey");
    a.fact(format!(
        "the journey holds one check: {}",
        a.d.has_containing("Clear the check for This journey")
    ));

    a.task("Ambush: read the read-aloud text and start the fight");
    // Review of PR 274, item 1: after the road check, the plan's one-time reveal puts the next
    // row clear of the footer, so `Start fight` is pressed with no scroll first.
    let start = format!("Start fight: {TONIGHT}");
    a.act(Nav, &start, |d| {
        assert!(
            d.nodes()
                .iter()
                .any(|node| node.name == start && node.fully_in_view()),
            "{start} is fully in view, so no scroll comes first"
        );
        d.click(&start);
    });
    a.fact(format!(
        "setup shows the read-aloud text: {}",
        a.d.visible_text("Ash falls on the crossing.")
    ));
    setup_fight(a);
    a.click(Do, "Begin combat");

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
    // A DM's act, so it is counted. The later turns then play on the HP from before it.
    a.task("The table rules that the Fireball missed: undo it");
    a.act(Do, "Undo with U", |d| d.press(Key::U));

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
    a.click(Nav, CLOSE_RUN);
    a.fact(format!(
        "after the fight, the page is {}",
        a.d.current_page()
    ));

    a.task("Record a short rest");
    if !a.d.has("Add Short rest") {
        a.click(Find, "More Add actions");
    }
    a.click(Do, "Add Short rest");

    // F4: a house rule's number, between fights. Trackers is a view of Lookup: Ctrl+T, one
    // press, Ctrl+T.
    a.setup("the campaign has a house rule note in play", |d| {
        let campaign = d.app_mut().campaigns.active_campaign().expect("a campaign");
        let id = splat::character::stable_id("Torch Rules.md");
        fs::write(campaign.path.join("Torch Rules.md"), torch_rules(id))
            .expect("write the house rule note");
        let manifest = campaign.path.join("Campaign.md");
        let text = fs::read_to_string(&manifest).expect("read Campaign.md");
        let list = serde_yaml_ng::to_value(vec![serde_json::json!({ "id": id })]).unwrap();
        let text = splat::content::formats::patch_model(&text, &[("splat_in_play", Some(list))])
            .expect("put the note in play");
        fs::write(manifest, text).expect("write Campaign.md");
        d.app_mut().lookup.refresh();
        d.settle();
    });
    a.task("Mark a house rule's number between fights");
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));
    a.click(Do, "Burn a torch");
    a.act(Panel, "Ctrl+T", |d| d.press_ctrl(Key::T));
    let ledger =
        a.d.app_mut()
            .campaigns
            .active_campaign()
            .map(|campaign| campaign.path.join("Trackers.md"))
            .and_then(|path| fs::read_to_string(path).ok())
            .unwrap_or_default();
    a.fact(format!(
        "the ledger holds the press: {}",
        ledger.contains("why: Burn a torch")
    ));

    a.task("Next fight: start it, with the party as it stands");
    a.click(Nav, &format!("Start fight: {SECOND}"));
    a.fact(format!(
        "Kestrel at the end of the last fight: [{hurt}]. At setup of the next: [{}]",
        hp(a.d, "Kestrel Dunmoor")
    ));

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

    // F3 of the session-cycle design: the same fight from the book's Bandit in Lookup.
    a.task("A fight that nobody planned, with a Bandit from the book, then talk");
    a.act(Panel, "open Lookup", |d| d.press_ctrl(Key::K));
    a.act(Find, "search: bandit", |d| d.type_text("bandit"));
    a.act(Find, "open the Bandit", |d| d.click_starting("Bandit · "));
    a.click(Nav, "Start a fight with Bandit");
    a.fact(format!(
        "the run of Fight with Bandit is open: {}",
        a.d.has_containing("Fight with Bandit")
    ));
    a.act(Nav, "close the run again", close_run);
    a.act(Panel, "close Lookup", |d| d.press(Key::Escape));
    a.fact(format!("closing it comes back to: {}", a.d.current_page()));

    a.task("Stop recording");
    a.click(Do, "Stop recording");
}

fn after_the_session(a: &mut Audit<'_, '_>) {
    a.phase = "after";
    a.setup("save, quit and open the app after the session", restart);
    a.fact(format!("the app reopened on {}", a.d.current_page()));
    a.setup("the recording service has its keys", |d| {
        d.app_mut().processing.simulate_keys();
        d.settle();
    });

    a.task("Review how the fight went: log and final HP");
    a.act(Find, "open the fight", |d| d.click(TONIGHT));
    a.fact(format!(
        "final HP reads in place: {}",
        a.d.visible_text("Kestrel ") && a.d.has("Log")
    ));

    // The fight's place is typed on the Timeline page, which its row links to.
    a.task("Say where the fight happened");
    a.act(Nav, "open the fight on the Timeline", |d| {
        d.click_link("Timeline")
    });
    a.act(Do, "type where it happened", |d| {
        d.type_into("Where", "The ford");
    });
    a.click(Nav, NAV_SESSIONS);

    a.task("Process the recording and read the summary");
    a.click(Sub, "Recordings, 1 recording");
    a.act(Do, "Process", |d| d.click_containing("Process:"));
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

    // A played fight has no `Add to notes`. The Timeline's detail of the fight copies its link,
    // and the DM pastes it in the note.
    a.task("Link the fight in the note");
    a.act(Nav, "open the fight on the Timeline", |d| {
        d.click_link("Timeline");
    });
    a.click(Do, "Copy link");
    a.click(Nav, NAV_SESSIONS);
    a.act(Do, "paste the link at the end of the note", |d| {
        d.paste_into_editor()
    });
    a.fact(format!(
        "the note holds the fight's link: {}",
        a.d.editor_text()
            .is_some_and(|text| text.contains("[[Timeline#^") && text.contains(TONIGHT))
    ));

    a.task("Check the party's HP for next time");
    a.click(Nav, NAV_PARTY);
    a.fact(format!(
        "Kestrel on Party after the evening: [{}]",
        a.d.nodes()
            .iter()
            .find(|node| node.name.contains("Kestrel Dunmoor ·"))
            .map_or_else(String::new, |node| node.name.clone())
    ));
}

/// Each counted task's cost, in the order played. Set-up rows count for nothing.
fn counts(record: &Record) -> Vec<TaskCount> {
    record
        .tasks
        .iter()
        .map(|(phase, task, start)| {
            let of: Vec<&Row> = record
                .rows
                .iter()
                .filter(|r| r.kind != Kind::Setup && r.phase == *phase && r.task == *task)
                .collect();
            let mut path = start.clone();
            for page in of.iter().flat_map(|r| r.pages.iter()) {
                let _ = write!(path, " > {page}");
            }
            TaskCount {
                phase: (*phase).to_owned(),
                task: task.clone(),
                transitions: of.iter().map(|r| r.pages.len()).sum(),
                interactions: of.iter().map(|r| r.interactions).sum(),
                path,
            }
        })
        .collect()
}

fn steps(rows: &[Row]) -> String {
    let mut steps = String::from("phase\ttask\tkind\twhat\tinteractions\tfrom\tpages\tblocked\n");
    for row in rows {
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
            row.blocked.is_some()
        );
    }
    steps
}

fn summary(record: &Record) -> String {
    let rows: Vec<&Row> = record
        .rows
        .iter()
        .filter(|row| row.kind != Kind::Setup)
        .collect();
    let tasks = counts(record);
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Phase totals\nphase\ttasks\ttransitions\tinteractions\tnav\tsub\tpanel\tfind\tdo"
    );
    for phase in ["before", "during", "after", "total"] {
        let of: Vec<&&Row> = rows
            .iter()
            .filter(|row| phase == "total" || row.phase == phase)
            .collect();
        let count = |kind| {
            of.iter()
                .filter(|row| row.kind == kind)
                .map(|row| row.interactions)
                .sum::<u32>()
        };
        let _ = writeln!(
            out,
            "{phase}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            tasks
                .iter()
                .filter(|task| phase == "total" || task.phase == phase)
                .count(),
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
    for task in &tasks {
        let overhead: u32 = rows
            .iter()
            .filter(|r| r.phase == task.phase && r.task == task.task && r.kind != Do)
            .map(|r| r.interactions)
            .sum();
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{overhead}\t{}",
            task.phase, task.task, task.transitions, task.interactions, task.path,
        );
    }
    let _ = writeln!(out, "\nSet-up, not counted");
    for row in record.rows.iter().filter(|row| row.kind == Kind::Setup) {
        let _ = writeln!(out, "{}\t{}", row.phase, row.what);
    }
    let _ = writeln!(out, "\nBlocked");
    for row in rows.iter().filter(|row| row.blocked.is_some()) {
        let _ = writeln!(out, "{}\t{}\t{}", row.phase, row.task, row.what);
    }
    let _ = writeln!(out, "\nFacts");
    for fact in &record.facts {
        let _ = writeln!(out, "{fact}");
    }
    out
}

/// The frozen counts. An empty file proves nothing, so it fails.
fn read_frozen() -> Frozen {
    let path = folder().join("counts.json");
    let text = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("G6: cannot read {}: {error}", path.display()));
    let frozen: Frozen = serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("G6: cannot read {}: {error}", path.display()));
    assert!(
        !frozen.tasks.is_empty(),
        "G6: {} holds no task, so it can prove nothing",
        path.display()
    );
    frozen
}

/// Each counted act that found no control, one line each.
fn blocked(rows: &[Row]) -> Vec<String> {
    rows.iter()
        .filter(|row| row.kind != Kind::Setup)
        .filter_map(|row| {
            let why = row.blocked.as_ref()?;
            Some(format!(
                "{} / {}: the act {:?} found no control: {why}",
                row.phase, row.task, row.what
            ))
        })
        .collect()
}

/// What differs between the frozen counts and this evening's, one line each.
fn differences(frozen: &[TaskCount], measured: &[TaskCount], rows: &[Row]) -> Vec<String> {
    let name =
        |task: &TaskCount| format!("the task \"{}\" ({} the session)", task.task, task.phase);
    let mut out = blocked(rows);
    for old in frozen {
        match measured
            .iter()
            .find(|new| new.phase == old.phase && new.task == old.task)
        {
            None => out.push(format!(
                "{} is in counts.json, but the evening no longer plays it",
                name(old)
            )),
            Some(new) if new != old => out.push(format!(
                "{} now takes {} interactions and {} page changes, through {}. counts.json says {} \
                 interactions and {} page changes, through {}",
                name(old),
                new.interactions,
                new.transitions,
                new.path,
                old.interactions,
                old.transitions,
                old.path
            )),
            Some(_) => {}
        }
    }
    for new in measured {
        if !frozen
            .iter()
            .any(|old| old.phase == new.phase && old.task == new.task)
        {
            out.push(format!(
                "{} is {NEW_TASK} ({} page changes, {} interactions). Add its row with `{ADD_COMMAND}`",
                name(new),
                new.transitions,
                new.interactions
            ));
        }
    }
    let order: Vec<String> = frozen.iter().map(name).collect();
    let kept: Vec<String> = measured
        .iter()
        .map(name)
        .filter(|task| order.contains(task))
        .collect();
    if kept != order && order.iter().all(|task| kept.contains(task)) {
        out.push("the frozen tasks are played in another order".to_owned());
    }
    out
}

fn evening(d: &mut Driver<'_>) -> Record {
    d.set_base((1200.0, 800.0));
    let mut audit = Audit {
        d,
        phase: "before",
        task: String::new(),
        record: Record::default(),
    };
    audit.setup(
        "play last week's session, then quit and open the app",
        last_week,
    );
    audit.setup("work at 1200x800", |d| d.set_base((1200.0, 800.0)));
    before_the_session(&mut audit);
    during_the_session(&mut audit);
    after_the_session(&mut audit);
    audit.record
}

fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("artifacts/dm-audit")
}

/// G6. The scenario `dm-audit`: play the frozen evening and compare each task's counts.
pub fn run(d: &mut Driver<'_>) {
    let record = evening(d);
    let measured = counts(&record);
    let total = |tasks: &[TaskCount]| {
        (
            tasks.iter().map(|t| t.transitions).sum::<usize>(),
            tasks.iter().map(|t| t.interactions).sum::<u32>(),
        )
    };
    let mode = std::env::var(WRITE).ok();
    if let Some(mode) = mode.as_deref() {
        let blocked = blocked(&record.rows);
        assert!(
            blocked.is_empty(),
            "dm-audit: an act found no control, so nothing is frozen:\n{}",
            blocked.join("\n")
        );
        let about = match mode {
            "all" => {
                "G6 reads this in every gate. `devenv shell -- audit-freeze counts` writes it; a \
                      feature adds its new tasks with `devenv shell -- audit-freeze add`, which \
                      leaves every row here as it is."
                    .to_owned()
            }
            "add" => {
                let frozen = read_frozen();
                let refused: Vec<String> = differences(&frozen.tasks, &measured, &record.rows)
                    .into_iter()
                    .filter(|line| !line.contains(NEW_TASK))
                    .collect();
                assert!(
                    refused.is_empty(),
                    "dm-audit: `{ADD_COMMAND}` writes rows for new tasks only, and a frozen task differs, \
                     so nothing is written. Put the frozen task back as it was:\n{}",
                    refused.join("\n")
                );
                frozen.about
            }
            other => panic!("dm-audit: {WRITE} is `all` or `add`, not {other:?}"),
        };
        let frozen = Frozen {
            about,
            tasks: measured,
        };
        let dir = folder();
        fs::write(dir.join("steps.tsv"), steps(&record.rows)).expect("write steps.tsv");
        fs::write(dir.join("summary.tsv"), summary(&record)).expect("write summary.tsv");
        fs::write(
            dir.join("counts.json"),
            serde_json::to_string_pretty(&frozen).expect("encode counts") + "\n",
        )
        .expect("write counts.json");
        println!("dm-audit: froze {}", dir.display());
        return;
    }
    let frozen = read_frozen();
    let found = differences(&frozen.tasks, &measured, &record.rows);
    assert!(
        found.is_empty(),
        "G6: the audit's evening (artifacts/dm-audit/) does not cost what counts.json says. {}.\n\
         If this change is meant, run `{COUNTS_COMMAND}` and say in the pull request why the task \
         costs more or less. If it is not meant, your change made the task cost more or less: \
         artifacts/dm-audit/steps.tsv lists each act of the task.\nAll {} differences:\n{}",
        found[0],
        found.len(),
        found.join("\n")
    );
    let (transitions, interactions) = total(&measured);
    d.observe(
        "G6",
        true,
        format!(
            "{} tasks, {transitions} page transitions and {interactions} interactions, as frozen",
            measured.len()
        ),
    );
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
