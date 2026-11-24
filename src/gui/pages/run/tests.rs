//! The run's screenshots and interaction tests. Captures are named after the egui scenarios'
//! steps (`encounter-run__04-participant-selected__1200x800`), so they pair with
//! `artifacts/screenshots/egui-reference/`.
use crate::Page;
use crate::encounter_run::{Panel, Phase};
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::{RUN, Seed};
use gpui::{px, size};

/// The fixture's run in progress, opened as Resume opens it.
fn open_saved(shell: &mut Shell, name: &str) {
    shell.update(|root, _, cx| {
        let campaign = root
            .app
            .campaigns
            .active_campaign()
            .expect("the fixture's campaign");
        let modules = root.app.campaigns.modules().to_vec();
        root.app.encounters.refresh_now(&campaign, &modules);
        let runs = root.app.encounters.saved_runs();
        let path = runs
            .iter()
            .find(|run| run.encounter == name || run.label.contains(name))
            .unwrap_or_else(|| {
                panic!(
                    "no saved run of {name}: {:?}",
                    runs.iter().map(|run| &run.encounter).collect::<Vec<_>>()
                )
            })
            .path
            .clone();
        root.app.encounters.open_saved_run(&path, &campaign);
        root.show_page(Page::Run, cx);
    });
    assert!(
        shell.read(|root| root.app.encounters.has_run()),
        "the run opens"
    );
}

/// A new run of an encounter, in setup, as `Run encounter` starts it.
fn start(shell: &mut Shell, path: &str) {
    shell.update(|root, _, cx| {
        let campaign = root
            .app
            .campaigns
            .active_campaign()
            .expect("the fixture's campaign");
        let modules = root.app.campaigns.modules().to_vec();
        root.app.encounters.refresh_now(&campaign, &modules);
        root.app.encounters.run_path(path, &campaign, &modules);
        root.show_page(Page::Run, cx);
    });
    assert!(
        shell.read(|root| root.app.encounters.has_run()),
        "the run starts"
    );
}

fn current_name(shell: &mut Shell) -> Option<String> {
    shell.read(|root| {
        let run = root.app.encounters.run()?;
        run.current()
            .map(|at| run.state.participants[at].name.clone())
    })
}

fn participant(shell: &mut Shell, name: &str) -> crate::encounter_run::Participant {
    shell.read(|root| {
        root.app
            .encounters
            .run()
            .and_then(|run| run.state.participants.iter().find(|p| p.name == name).cloned())
            .unwrap_or_else(|| panic!("no participant {name}"))
    })
}

/// The live run: Next turn by click and by key, a selection, damage typed in Amount, a
/// condition and its Undo (egui `encounter-run`).
#[test]
fn encounter_run_live() {
    let mut shell = Shell::new("encounter-run", Seed::Campaign);
    open_saved(&mut shell, RUN);
    shell.update(|root, _, _| {
        if let Some(run) = root.app.encounters.run_mut() {
            run.simulate_fresh_live_history();
        }
    });
    shell.capture("encounter-run__02-live-with-no-history__1200x800");
    let before = current_name(&mut shell);
    shell.click("Next turn");
    let after = current_name(&mut shell);
    assert!(before.is_some() && before != after, "Next turn moves the turn: {before:?} then {after:?}");
    shell.capture("encounter-run__03-next-turn__1200x800");

    // A click selects a row; the keyboard goes to Amount, so digits and Enter damage it.
    shell.click("Ember Cultist");
    assert_eq!(
        shell.read(|root| {
            let run = root.app.encounters.run().unwrap();
            run.state.participants[run.selected].name.clone()
        }),
        "Ember Cultist"
    );
    shell.capture("encounter-run__04-participant-selected__1200x800");
    let hp = participant(&mut shell, "Ember Cultist").hp.unwrap();
    shell.type_text("7");
    shell.press("enter");
    assert_eq!(participant(&mut shell, "Ember Cultist").hp, Some(hp - 7));
    shell.capture("encounter-run__05-hp-changed__1200x800");

    // Conditions: the dock opens the tab; a toggle adds one; U takes it back.
    shell.click("id:run-dock-conditions");
    assert_eq!(
        shell.read(|root| root.app.encounters.run().unwrap().panel),
        Panel::Conditions
    );
    shell.capture("encounter-run__06-condition-tab__1200x800");
    shell.click("Frightened");
    assert!(
        participant(&mut shell, "Ember Cultist")
            .conditions
            .iter()
            .any(|c| c.name() == "Frightened"),
        "the toggle adds Frightened"
    );
    shell.capture("encounter-run__07-condition-added__1200x800");
    shell.press("u");
    assert!(
        !participant(&mut shell, "Ember Cultist")
            .conditions
            .iter()
            .any(|c| c.name() == "Frightened"),
        "U undoes the condition"
    );
    shell.capture("encounter-run__08-undo__1200x800");

    // N is Next turn while nothing else has the keyboard.
    let before = current_name(&mut shell);
    shell.press("escape n");
    let after = current_name(&mut shell);
    assert_ne!(before, after, "N moves the turn");
    shell.capture("encounter-run__09-next-turn-by-keyboard__1200x800");
}

/// The live run at the smallest window.
#[test]
fn encounter_run_small() {
    let mut shell = Shell::with_size("encounter-run", Seed::Campaign, size(px(960.0), px(640.0)));
    open_saved(&mut shell, RUN);
    shell.update(|root, _, _| {
        if let Some(run) = root.app.encounters.run_mut() {
            run.simulate_fresh_live_history();
        }
    });
    shell.capture("encounter-run__02-live-with-no-history__960x640");
}

/// Setup: rows with their initiative, Roll NPCs, Begin combat waits for initiative, and typed
/// totals let it begin (egui `encounter-run` steps 12 and 13).
#[test]
fn encounter_run_setup() {
    let mut shell = Shell::new("encounter-run", Seed::Campaign);
    start(&mut shell, "Encounters/Shrine of Cinders.md");
    assert_eq!(
        shell.read(|root| root.app.encounters.run().unwrap().state.phase.clone()),
        Phase::Setup
    );
    shell.capture("encounter-run__12-run-setup__1200x800");
    let reason = shell.read(|root| root.app.encounters.run().unwrap().setup_blocked());
    assert!(reason.is_some_and(|reason| reason.contains("need initiative")));
    shell.click("Roll NPCs");
    let npcs_rolled = shell.read(|root| {
        root.app
            .encounters
            .run()
            .unwrap()
            .state
            .participants
            .iter()
            .filter(|p| p.npc)
            .all(|p| p.initiative.is_some())
    });
    assert!(npcs_rolled, "Roll NPCs rolls every NPC");
    shell.capture("encounter-run__13-npc-initiative-rolled__1200x800");
    // Each player's total, typed in their row's field.
    let players: Vec<String> = shell.read(|root| {
        root.app
            .encounters
            .run()
            .unwrap()
            .state
            .participants
            .iter()
            .filter(|p| p.initiative.is_none())
            .map(|p| p.name.clone())
            .collect()
    });
    for (at, name) in players.iter().enumerate() {
        shell.click(&format!("Initiative for {name}"));
        shell.type_text(&format!("{}", 10 + at));
    }
    assert_eq!(
        shell.read(|root| root.app.encounters.run().unwrap().setup_blocked()),
        None
    );
    shell.click("Begin combat");
    assert_eq!(
        shell.read(|root| root.app.encounters.run().unwrap().state.phase.clone()),
        Phase::Running
    );
}

/// End encounter, its outcome, the ended view, its Undo; then Close run (egui `run-continue`).
#[test]
fn run_end_undo_and_close() {
    let mut shell = Shell::new("run-continue", Seed::Campaign);
    open_saved(&mut shell, RUN);
    shell.click("End encounter…");
    assert!(shell.read(|root| root.app.encounters.run().unwrap().ending));
    shell.type_text("Fled");
    shell.click("id:run-end-confirm");
    assert!(matches!(
        shell.read(|root| root.app.encounters.run().unwrap().state.phase.clone()),
        Phase::Ended { outcome } if outcome == "Fled"
    ));
    shell.capture("run-continue__02-ended-undo-is-the-way-on__1200x800");
    shell.click("Undo: end encounter");
    assert_eq!(
        shell.read(|root| root.app.encounters.run().unwrap().state.phase.clone()),
        Phase::Running
    );
    shell.click("id:run-close");
    assert!(
        shell.read(|root| !root.app.encounters.has_run()),
        "Close run closes a saved run in one step"
    );
    assert_ne!(shell.read(|root| root.app.page), Page::Run);
}

/// On another page the strip keeps the turn; Next turn there moves it, and Back to run returns.
#[test]
fn run_strip_on_another_page() {
    let mut shell = Shell::new("run-long-names", Seed::Campaign);
    open_saved(&mut shell, RUN);
    shell.update(|root, _, cx| root.show_page(Page::Library, cx));
    shell.capture("run-long-names__02-long-name-on-the-strip__1200x800");
    let before = current_name(&mut shell);
    shell.click("id:run-strip-next-turn");
    assert_ne!(before, current_name(&mut shell));
    shell.click("Back to run");
    assert_eq!(shell.read(|root| root.app.page), Page::Run);
}

/// The ended view with loot to roll; Roll all rolls it (egui `run-loot`).
#[test]
fn run_loot() {
    let mut shell = Shell::new("run-loot", Seed::Loot);
    let name = shell.read(|root| {
        root.app
            .encounters
            .saved_runs()
            .first()
            .map(|run| run.encounter.clone())
    });
    let name = name.unwrap_or_else(|| "Sunken Vault".to_owned());
    open_saved(&mut shell, &name);
    shell.capture("run-loot__01-loot-on-the-card__1200x800");
    shell.click("End encounter…");
    shell.click("id:run-end-confirm");
    shell.capture("run-loot__02-ended-with-loot__1200x800");
    let unrolled = shell.read(|root| {
        let run = root.app.encounters.run().unwrap();
        let holders: Vec<_> = run.holders().into_iter().filter(|h| h.open).collect();
        run.unrolled(&holders).len()
    });
    assert!(unrolled > 0, "the ended fight has loot to roll");
    if unrolled > 1 {
        shell.click(&format!("Roll all {unrolled}"));
    } else {
        shell.click("Roll");
    }
    assert!(
        shell.read(|root| !root.app.encounters.run().unwrap().state.loot.is_empty()),
        "the loot is rolled"
    );
    shell.capture("run-loot__03-loot-rolled__1200x800");
}
