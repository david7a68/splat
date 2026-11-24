//! Captures of the run page for side-by-side review with the reference captures. Each test
//! builds the state of one scenario's run steps and captures it as `<scenario>__<step>__<w>x<h>`,
//! so it pairs with its reference by name.
//! The asserts only check that each state was reached, so a fixture change fails here rather
//! than giving a wrong picture.
use crate::Page;
use crate::encounter_run::{Command, Panel, Phase, Run};
use crate::fixtures::{RUN, RUN_12, RUN_20, RUN_ARENA, Seed};
use crate::gui::harness::Shell;
use crate::gui::kit;
use gpui::{ElementId, point, px, size};

const SHRINE: &str = "Encounters/Shrine of Cinders.md";

fn small(name: &str, seed: Seed) -> Shell {
    Shell::with_size(name, seed, size(px(960.0), px(640.0)))
}

/// Close the open run, if any, as Close run does when nothing waits.
fn close(shell: &mut Shell) {
    shell.update(|root, _, cx| {
        if root.app.encounters.has_run() {
            let campaign = root.app.campaigns.active_campaign().expect("the campaign");
            root.app.encounters.close_shown_run(&campaign);
            root.app.run_page_end(true);
        }
        cx.notify();
    });
    assert!(
        !shell.read(|root| root.app.encounters.has_run()),
        "the run closes"
    );
}

/// The fixture’s run in progress, opened as Resume opens it.
fn open_saved(shell: &mut Shell, name: &str) {
    close(shell);
    shell.update(|root, _, cx| {
        let campaign = root
            .app
            .campaigns
            .active_campaign()
            .expect("the fixture’s campaign");
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

/// A new run of an encounter, in setup, as `Start fight` starts it.
fn start(shell: &mut Shell, path: &str) {
    close(shell);
    shell.update(|root, _, cx| {
        let campaign = root
            .app
            .campaigns
            .active_campaign()
            .expect("the fixture’s campaign");
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

/// Change the open run, then settle.
fn on_run<R>(shell: &mut Shell, change: impl FnOnce(&mut Run) -> R) -> R {
    shell.update(|root, _, cx| {
        let result = change(root.app.encounters.run_mut().expect("an open run"));
        cx.notify();
        result
    })
}

/// Read the open run.
fn with_run<R>(shell: &mut Shell, read: impl FnOnce(&Run) -> R) -> R {
    shell.read(|root| read(root.app.encounters.run().expect("an open run")))
}

fn index(shell: &mut Shell, name: &str) -> usize {
    with_run(shell, |run| {
        run.state
            .participants
            .iter()
            .position(|p| p.name == name)
            .unwrap_or_else(|| panic!("no participant {name}"))
    })
}

fn phase(shell: &mut Shell) -> Phase {
    with_run(shell, |run| run.state.phase.clone())
}

fn count(shell: &mut Shell) -> usize {
    with_run(shell, |run| run.state.participants.len())
}

fn selected(shell: &mut Shell) -> String {
    with_run(shell, |run| {
        run.state.participants[run.selected].name.clone()
    })
}

fn current_name(shell: &mut Shell) -> Option<String> {
    with_run(shell, |run| {
        run.current()
            .map(|at| run.state.participants[at].name.clone())
    })
}

fn row_text(shell: &mut Shell, name: &str) -> String {
    let at = index(shell, name);
    with_run(shell, |run| {
        run.state.participants[at]
            .conditions
            .iter()
            .map(|c| c.words())
            .collect::<Vec<_>>()
            .join(" · ")
    })
}

/// Select a row as the DM's click does when nothing is armed.
fn select(shell: &mut Shell, name: &str) {
    let at = index(shell, name);
    on_run(shell, |run| run.select(at));
    assert_eq!(selected(shell), name);
}

/// The first target laid out whose name starts with `prefix`.
fn target_starting(shell: &mut Shell, prefix: &str) -> String {
    shell.settle();
    kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with(prefix))
        .unwrap_or_else(|| {
            panic!(
                "no target starts with {prefix:?}. Targets: {:?}",
                kit::probe::names()
            )
        })
}

fn click_starting(shell: &mut Shell, prefix: &str) {
    let name = target_starting(shell, prefix);
    shell.click(&name);
}

fn has_target(shell: &mut Shell, name: &str) -> bool {
    shell.settle();
    shell.bounds(name).is_some()
}

fn open_lookup(shell: &mut Shell) {
    shell.press("ctrl-k");
    assert!(shell.read(|root| root.app.lookup.open), "Lookup opens");
    for _ in 0..200 {
        if shell.read(|root| root.app.lookup.index().is_some()) {
            return;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    panic!("Lookup’s index was not read");
}

// ---------------------------------------------------------------------------------------------
// run-rolls

const ATTACKER: &str = "Cinder Wolf 1";
const TARGET: &str = "Cinder Wolf 2";
const CASTER: &str = "Ember Cultist";
const PLAYER: &str = "Iselle Marrowind";

fn rolls_tiles(shell: &mut Shell) {
    open_saved(shell, RUN);
    select(shell, ATTACKER);
    assert!(with_run(shell, |run| run.panel == Panel::Reference));
    target_starting(shell, "Bite");
}

/// Scenario `run-rolls`: the attack tiles, an armed attack, its roll on a row, a row's menu, a
/// damage question, a save’s targets and their damage, and a player's attack.
#[test]
fn run_rolls() {
    let mut shell = Shell::new("run-rolls", Seed::Campaign);
    rolls_tiles(&mut shell);
    // A tile alone on its line takes the line, as egui's `tile_lines` did.
    let bite = target_starting(&mut shell, "Bite");
    let width = shell.bounds(&bite).expect("the Bite tile").size.width;
    assert!(
        f32::from(width) > crate::rolls::plain::TILE_WIDTH + 40.0,
        "the lone Bite tile fills its line: {width:?}"
    );
    shell.capture("run-rolls__01-tiles-in-the-stat-block__1200x800");

    click_starting(&mut shell, "Bite");
    assert!(with_run(&mut shell, |run| run.armed.is_some()), "Bite arms");
    shell.capture("run-rolls__02-attack-armed__1200x800");

    let lines = with_run(&mut shell, Run::log_len);
    shell.click(TARGET);
    assert!(
        with_run(&mut shell, |run| run.armed.is_some()),
        "the attack stays armed after its roll"
    );
    assert!(
        with_run(&mut shell, Run::log_len) > lines,
        "the attack is rolled"
    );
    shell.capture("run-rolls__03-attack-rolled__1200x800");
    shell.press("enter");
    // As in the egui scenario: a second attack and its damage, then Escape stops the action.
    shell.click(TARGET);
    shell.press("enter");
    shell.press("escape");
    assert!(with_run(&mut shell, |run| run.armed.is_none()));
    shell.click(ATTACKER);
    // Bite armed with advantage, as the tile's press with Shift held arms it (egui's tile
    // menu, `Roll with advantage`; the GPUI tile has no menu yet).
    shell.update(|root, _, cx| {
        let run = root.app.encounters.run().expect("an open run");
        let row = run.panel_row();
        let bite = run
            .chips(row)
            .into_iter()
            .find(|chip| {
                chip.label.starts_with("Bite")
                    && matches!(chip.link, splat::content::dice_text::RollLink::D20 { .. })
            })
            .expect("the Bite attack");
        // The tile's mouse-up makes the row the roller, then its press asks.
        crate::rolls::set_roller(&root.ctx, Some(row));
        crate::rolls::plain::ask_pressed_at(&root.ctx, &bite.link, None, true, false);
        cx.notify();
    });
    shell.settle();
    assert_eq!(
        with_run(&mut shell, |run| run.armed.as_ref().map(|armed| armed.mode)),
        Some(crate::rolls::Mode::Advantage),
        "Bite is armed with advantage"
    );

    // The row menu, with Bite armed (a right-click on the row opens it).
    let target = index(&mut shell, TARGET);
    let key = with_run(&mut shell, |run| run.state.participants[target].key.clone());
    assert!(
        with_run(&mut shell, |run| !run.row_menu_sections(target).is_empty()),
        "the target’s row has a menu while Bite is armed"
    );
    shell.update(|_, window, cx| {
        kit::menu::open_popup(
            ElementId::Name(format!("run-row-menu-{key}").into()),
            window,
            cx,
        );
    });
    shell.capture("run-rolls__05-row-menu__1200x800");
    // The first Escape closes the row menu, the second stops the armed action.
    shell.press("escape");
    shell.press("escape");
    assert!(
        with_run(&mut shell, |run| run.armed.is_none()),
        "Escape stops the armed action"
    );

    // A damage that depends on what the DM knows asks before it rolls.
    select(&mut shell, CASTER);
    click_starting(&mut shell, "Scorching Sickle");
    shell.click(TARGET);
    assert!(
        with_run(&mut shell, |run| run.asking.is_some()),
        "the attack asks its question"
    );
    shell.capture("run-rolls__06-damage-question__1200x800");
    on_run(&mut shell, |run| {
        run.row_choice(target, crate::encounter_run::rolling::RowChoice::Cancel);
        run.disarm();
    });

    // A save: each clicked row is a target.
    select(&mut shell, CASTER);
    click_starting(&mut shell, "Ember Burst");
    shell.click(ATTACKER);
    shell.click(TARGET);
    assert_eq!(with_run(&mut shell, |run| run.save_targets().len()), 2);
    shell.capture("run-rolls__07-save-targets__1200x800");
    let before = with_run(&mut shell, |run| {
        (
            run.state.participants[index_in(run, ATTACKER)].hp,
            run.state.participants[target].hp,
        )
    });
    shell.click("Roll damage");
    shell.press("enter");
    let after = with_run(&mut shell, |run| {
        (
            run.state.participants[index_in(run, ATTACKER)].hp,
            run.state.participants[target].hp,
        )
    });
    assert_ne!(before, after, "the save’s damage is applied");
    shell.capture("run-rolls__08-save-applied__1200x800");
    on_run(&mut shell, Run::disarm);

    // A player rolls their own dice: the tile, on the Actions page of the linked sheet, only
    // targets the row.
    select(&mut shell, PLAYER);
    shell.click("id:run-sheet-pages-Actions");
    click_starting(&mut shell, "Dagger");
    shell.click(ATTACKER);
    assert!(with_run(&mut shell, |run| run.armed.is_some()));
    shell.capture("run-rolls__09-player-attack__1200x800");

    let mut shell = small("run-rolls", Seed::Campaign);
    rolls_tiles(&mut shell);
    shell.capture("run-rolls__01-tiles-in-the-stat-block__960x640");
    click_starting(&mut shell, "Bite");
    shell.capture("run-rolls__02-attack-armed__960x640");
}

fn index_in(run: &Run, name: &str) -> usize {
    run.state
        .participants
        .iter()
        .position(|p| p.name == name)
        .unwrap_or_else(|| panic!("no participant {name}"))
}

// ---------------------------------------------------------------------------------------------
// run-sync-states

const SYNC_STATES: [(&str, crate::combat_sync::SyncState); 5] = {
    use crate::combat_sync::SyncState;
    [
        ("Brannoch Vale", SyncState::Conflict),
        ("Iselle Marrowind", SyncState::Matching),
        ("Kestrel Dunmoor", SyncState::Pending),
        ("Oren Thistlewick", SyncState::ReadOnly),
        ("Sable Harrowgate", SyncState::Error),
    ]
};

fn every_sync_state(shell: &mut Shell) {
    open_saved(shell, RUN);
    shell.update(|root, _, _| {
        root.app
            .settings
            .simulate(crate::settings::Account::Connected);
        for (name, state) in SYNC_STATES {
            root.app.encounters.simulate_sync_state(name, state);
        }
    });
    let states = with_run(shell, |run| run.syncs.len());
    assert!(states >= 5, "every linked row has a sync state: {states}");
}

/// Scenario `run-sync-states`: each sync state on its row, a linked row with sync off, and the
/// ended view with a conflict.
#[test]
fn run_sync_states() {
    use crate::combat_sync::SyncState;
    let mut shell = Shell::new("run-sync-states", Seed::Campaign);
    every_sync_state(&mut shell);
    shell.capture("run-sync-states__01-every-sync-state__1200x800");

    shell.update(|root, _, _| {
        root.app
            .encounters
            .simulate_sync_state("Sable Harrowgate", SyncState::Off);
    });
    select(&mut shell, "Sable Harrowgate");
    shell.capture("run-sync-states__02-linked-row-sync-off__1200x800");

    shell.update(|root, _, _| {
        root.app
            .encounters
            .simulate_sync_state("Sable Harrowgate", SyncState::Error);
        root.app
            .encounters
            .simulate_sync_conflict("Iselle Marrowind");
    });
    shell.click("End encounter…");
    shell.click("id:run-end-confirm");
    assert!(matches!(phase(&mut shell), Phase::Ended { .. }));
    // The ended view keeps the sync's choices and a way to stop it, or the run cannot close.
    assert!(
        has_target(&mut shell, "Stop sync for Iselle Marrowind"),
        "the ended view shows the unsettled sync: {:?}",
        kit::probe::names()
    );
    shell.capture("run-sync-states__03-ended-with-conflict__1200x800");

    let mut shell = small("run-sync-states", Seed::Campaign);
    every_sync_state(&mut shell);
    shell.capture("run-sync-states__01-every-sync-state__960x640");
}

// ---------------------------------------------------------------------------------------------
// run-add-creature

const FIELD: &str = "Find or name a creature";

fn arrivals_made_up(shell: &mut Shell) {
    open_saved(shell, RUN);
    on_run(shell, Run::show_arrivals);
    assert!(with_run(shell, |run| run.panel == Panel::Groups));
}

/// Scenario `run-add-creature`: Arrivals empty, a made-up creature from its HP to its row, a found
/// creature, every match, and a group whose Add waits for a total.
#[test]
fn run_add_creature() {
    let mut shell = Shell::new("run-add-creature", Seed::Groups);
    arrivals_made_up(&mut shell);
    shell.capture("run-add-creature__01-arrivals-empty__1200x800");

    shell.click(FIELD);
    shell.type_text("Ghost of the Mill");
    assert!(with_run(&mut shell, |run| run.creature_add().made_up));
    shell.capture("run-add-creature__02-arrivals-made-up-needs-hp__1200x800");

    for (field, words) in [
        ("New creature HP", "22"),
        ("New creature AC", "14"),
        ("New creature initiative total", "9"),
    ] {
        shell.click(field);
        shell.type_text(words);
    }
    assert!(with_run(&mut shell, |run| run
        .creature_add()
        .blocked
        .is_none()));
    shell.capture("run-add-creature__03-arrivals-made-up-ready__1200x800");

    let before = count(&mut shell);
    shell.click("Add Ghost of the Mill to the initiative list");
    assert_eq!(count(&mut shell), before + 1, "the ghost joins");
    shell.capture("run-add-creature__04-arrivals-made-up-added__1200x800");

    select(&mut shell, "Ghost of the Mill");
    on_run(&mut shell, |run| run.panel = Panel::Reference);
    shell.settle();
    // Nine rows fit the roster: nothing scrolls and no line says what is out of view.
    let end = shell.read(|root| f32::from(root.pages.run.roster_scroll.max_offset().y));
    assert!(end < 1.0, "the roster of nine scrolls by {end}");
    shell.capture("run-add-creature__05-made-up-creature-sheet__1200x800");

    let ghost = index(&mut shell, "Ghost of the Mill");
    on_run(&mut shell, |run| run.run_command(Command::Damage(ghost, 5)));
    assert_eq!(
        with_run(&mut shell, |run| run.state.participants[ghost].hp),
        Some(17)
    );
    shell.capture("run-add-creature__06-made-up-creature-after-damage__1200x800");
    on_run(&mut shell, Run::undo);

    on_run(&mut shell, Run::show_arrivals);
    shell.click(FIELD);
    shell.type_text("wolf");
    let (matches, _) = with_run(&mut shell, |run| run.creature_matches());
    assert!(matches.len() > 1, "wolf finds several creatures");
    shell.capture("run-add-creature__07-arrivals-found__1200x800");

    shell.click("id:run-creature-Wolf");
    assert!(with_run(&mut shell, |run| run.ad_hoc.picked.is_some()));
    shell.click("New creature count");
    shell.type_text("2");
    shell.capture("run-add-creature__08-arrivals-found-chosen__1200x800");

    let before = count(&mut shell);
    shell.click("Add Wolf to the initiative list");
    assert_eq!(count(&mut shell), before + 2, "two wolves join");
    on_run(&mut shell, Run::show_arrivals);
    shell.click(FIELD);
    shell.type_text("wolf");
    shell.capture("run-add-creature__09-arrivals-found-all__1200x800");

    // A group whose first member has no initiative bonus: its Add waits for a typed total.
    const RIDERS: &str = "Mist Riders";
    let mut shell = Shell::new("run-add-creature", Seed::Groups);
    shell.update(|root, _, _| {
        let campaign = root.app.campaigns.active_campaign().unwrap();
        let rider = splat::character::stable_id("visual:note:mist-rider");
        for (relative, fields) in [
            (
                "Notes/Mist Rider.md".to_owned(),
                format!(
                    "splat_id: {rider}\nsplat_kind: creature\nsplat_name: Mist Rider\nsplat_creature: {{size: medium, type: undead}}\nsplat_stats:\n  max_hp: {{value: 9}}\n  armor_class: {{value: 12}}\n"
                ),
            ),
            (
                format!("Encounter Groups/{RIDERS}.md"),
                format!(
                    "splat_id: {}\nsplat_kind: encounter_group\nsplat_name: {RIDERS}\nsplat_encounter_group:\n  side: rivals\n  members:\n  - creature: {{id: {rider}}}\n    count: 2\n",
                    splat::character::stable_id("visual:group:mist-riders")
                ),
            ),
        ] {
            let text = splat::content::formats::fixture(&relative, &fields, "").unwrap();
            crate::library::write_note(&campaign.path, &relative, None, &text).unwrap();
        }
    });
    arrivals_made_up(&mut shell);
    let add = format!("Add {RIDERS} to the initiative list");
    assert!(has_target(&mut shell, &add), "Arrivals lists {RIDERS}");
    shell.capture("run-add-creature__11-arrivals-add-waits-for-a-total__1200x800");

    let mut shell = small("run-add-creature", Seed::Groups);
    arrivals_made_up(&mut shell);
    shell.click(FIELD);
    shell.type_text("Ghost of the Mill");
    shell.capture("run-add-creature__02-arrivals-made-up-needs-hp__960x640");
}

// ---------------------------------------------------------------------------------------------
// run-edge-states

const LONG_EFFECT: &str =
    "Wreathed in the cinders of the old shrine, so every light within thirty feet burns low";
const EDGE_TARGET: &str = "Kestrel Dunmoor";

fn long_effect(shell: &mut Shell) {
    open_saved(shell, RUN);
    select(shell, EDGE_TARGET);
    on_run(shell, Run::open_conditions_tab);
    shell.click("Other effect");
    shell.type_text(LONG_EFFECT);
    shell.click("id:run-other-effect-add");
    assert!(
        row_text(shell, EDGE_TARGET).contains("Wreathed"),
        "the effect joins the row"
    );
}

/// Scenario `run-edge-states`: a long effect, Conc on and off with its Undo, every row absent,
/// the run changed on disk, the close dialog while not saved, and saving it as a new run.
#[test]
fn run_edge_states() {
    let mut shell = Shell::new("run-edge-states", Seed::Campaign);
    long_effect(&mut shell);
    shell.capture("run-edge-states__01-long-effect-text__1200x800");

    let at = index(&mut shell, EDGE_TARGET);
    on_run(&mut shell, |run| {
        run.run_command(Command::Concentration(at, true))
    });
    assert!(with_run(&mut shell, |run| run.state.participants[at].concentration));
    shell.capture("run-edge-states__02-start-concentration-undo__1200x800");
    on_run(&mut shell, |run| {
        run.run_command(Command::Concentration(at, false))
    });
    assert!(!with_run(&mut shell, |run| run.state.participants[at].concentration));
    shell.capture("run-edge-states__03-stop-concentration-undo__1200x800");

    on_run(&mut shell, |run| run.panel = Panel::Reference);
    if has_target(&mut shell, "Read all conditions") {
        shell.click("Read all conditions");
    } else {
        on_run(&mut shell, Run::open_conditions_tab);
    }
    assert_eq!(
        with_run(&mut shell, |run| run.panel),
        Panel::Conditions,
        "the full effect is read in Conditions"
    );
    shell.capture("run-edge-states__04-full-effect-in-conditions__1200x800");

    let all = count(&mut shell);
    on_run(&mut shell, |run| {
        for at in 0..all {
            run.select(at);
            run.run_command(Command::Presence(at, false));
        }
    });
    assert!(with_run(&mut shell, |run| run
        .state
        .participants
        .iter()
        .all(|p| !p.present)));
    shell.capture("run-edge-states__05-all-absent__1200x800");
    on_run(&mut shell, |run| {
        for at in 0..all {
            run.run_command(Command::Presence(at, true));
        }
    });

    // A second instance writes the file: saving stops and says so.
    let path = with_run(&mut shell, |run| {
        run.path().map(std::path::Path::to_path_buf)
    })
    .expect("a saved run");
    let text = std::fs::read_to_string(&path).expect("read the run") + "\n";
    std::fs::write(&path, text).expect("change the run on disk");
    shell.click(EDGE_TARGET);
    shell.type_text("3");
    shell.press("enter");
    for _ in 0..100 {
        if with_run(&mut shell, Run::has_disk_conflict) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
    }
    assert!(
        with_run(&mut shell, Run::has_disk_conflict),
        "the run sees the file changed on disk"
    );
    shell.capture("run-edge-states__06-changed-on-disk__1200x800");

    shell.click("Close run");
    assert!(has_target(&mut shell, "Save as a new run and close"));
    shell.capture("run-edge-states__07-close-while-not-saved__1200x800");

    // As in the egui scenario: Escape keeps the run, Replace saves over the changed file; a
    // second change on disk is saved as a new run beside it; a third, to the new file, opens
    // the same close choices for it.
    shell.press("escape");
    assert!(
        !has_target(&mut shell, "Keep running"),
        "Escape keeps the run"
    );
    shell.click("Replace the file with this run");
    assert!(!with_run(&mut shell, Run::has_disk_conflict));
    let change_on_disk = |shell: &mut Shell, path: &std::path::Path, damage: &str| {
        let text = std::fs::read_to_string(path).expect("read the run") + "\n";
        std::fs::write(path, text).expect("change the run on disk");
        shell.click(EDGE_TARGET);
        shell.type_text(damage);
        shell.press("enter");
        for _ in 0..100 {
            if with_run(shell, Run::has_disk_conflict) {
                return;
            }
            shell.wait(std::time::Duration::from_millis(50));
        }
        panic!("the run did not see its file changed on disk");
    };
    change_on_disk(&mut shell, &path, "1");
    shell.click("Save as a new run");
    let new_path = with_run(&mut shell, |run| {
        run.path().map(std::path::Path::to_path_buf)
    })
    .expect("the new run's file");
    assert!(
        new_path != path && new_path.exists(),
        "Save as a new run saves beside the changed file: {new_path:?}"
    );
    change_on_disk(&mut shell, &new_path, "1");
    shell.click("Close run");
    assert!(has_target(&mut shell, "Save as a new run and close"));
    shell.capture("run-edge-states__08-save-new-run-before-close__1200x800");

    let mut shell = small("run-edge-states", Seed::Campaign);
    long_effect(&mut shell);
    shell.capture("run-edge-states__01-long-effect-text__960x640");
}

// ---------------------------------------------------------------------------------------------
// run-effects

const ISELLE: &str = "Iselle Marrowind";
const HOLD: &str = "Wis save DC 15 (Hold Person)";

fn effect(shell: &mut Shell, name: &str, words: &str) {
    let at = index(shell, name);
    on_run(shell, |run| {
        run.select(at);
        run.run_command(Command::Condition(at, words.to_owned()));
    });
}

fn effects_on_its_turn(shell: &mut Shell) -> String {
    open_saved(shell, RUN);
    let current = current_name(shell).expect("a turn");
    effect(shell, ISELLE, "Frightened");
    effect(shell, &current, "Bless 10 rounds");
    effect(shell, &current, &format!("{HOLD} on its turn"));
    assert!(row_text(shell, &current).contains("Bless 10"));
    current
}

/// Scenario `run-effects`: effects on the current row, uses and a recharge on rows, an effect beside
/// Lookup, and the effects in Conditions.
#[test]
fn run_effects() {
    let mut shell = Shell::new("run-effects", Seed::Campaign);
    let current = effects_on_its_turn(&mut shell);
    shell.capture("run-effects__01-effects-on-its-turn__1200x800");

    effect(&mut shell, ATTACKER, "Hexed until the hag dies");
    effect(&mut shell, ATTACKER, "Legendary actions 3 uses");
    effect(&mut shell, CASTER, "Fire Breath recharge 5-6");
    // The counts act from their rows, as the DM's clicks do.
    shell.click(&format!("Spend one use of Legendary actions of {ATTACKER}"));
    assert!(row_text(&mut shell, ATTACKER).contains("2 of 3"));
    shell.click("Mark Fire Breath spent");
    shell.click("Roll 1d6. 5–6 makes it ready");
    assert!(row_text(&mut shell, CASTER).contains("Fire Breath"));
    shell.capture("run-effects__02-effects-on-rows__1200x800");

    // Beside Lookup: an effect added from Conditions.
    let _ = current;
    open_lookup(&mut shell);
    shell.type_text("frightened");
    select(&mut shell, ISELLE);
    effect(&mut shell, ISELLE, "Frightened 10 rounds");
    if !shell.read(|root| root.app.lookup.open) {
        shell.press("ctrl-k");
    }
    assert!(shell.read(|root| root.app.lookup.open));
    assert!(row_text(&mut shell, ISELLE).contains("Frightened 10"));
    shell.capture("run-effects__03-effects-beside-lookup__1200x800");

    shell.press("escape");
    on_run(&mut shell, Run::open_conditions_tab);
    assert_eq!(with_run(&mut shell, |run| run.panel), Panel::Conditions);
    shell.capture("run-effects__04-effects-in-conditions__1200x800");

    let mut shell = small("run-effects", Seed::Campaign);
    effects_on_its_turn(&mut shell);
    shell.capture("run-effects__01-effects-on-its-turn__960x640");
}

// ---------------------------------------------------------------------------------------------
// run-overrides

/// Scenario `run-overrides`: a changed creature's card in the run, and setup with its row.
#[test]
fn run_overrides() {
    let mut shell = Shell::new("run-overrides", Seed::Groups);
    open_saved(&mut shell, RUN_ARENA);
    select(&mut shell, "Rival Captain");
    shell.capture("run-overrides__01-overridden-participant__1200x800");

    start(&mut shell, &format!("Encounters/{RUN_ARENA}.md"));
    assert_eq!(phase(&mut shell), Phase::Setup);
    assert!(with_run(&mut shell, |run| run
        .state
        .participants
        .iter()
        .any(|p| p.name == "Rival Captain")));
    shell.capture("run-overrides__04-setup-with-a-changed-row__1200x800");
}

// ---------------------------------------------------------------------------------------------
// run-lookup

fn in_the_run(shell: &mut Shell) {
    open_saved(shell, RUN);
    select(shell, CASTER);
}

/// Scenario `run-lookup`: the run, a spell read beside it, entries found, the run back, a spell
/// opened from the stat block, and Lookup’s Back by key.
#[test]
fn run_lookup() {
    let mut shell = Shell::new("run-lookup", Seed::Campaign);
    in_the_run(&mut shell);
    shell.capture("run-lookup__01-in-the-run__1200x800");

    open_lookup(&mut shell);
    shell.type_text("fireball");
    assert!(shell.read(|root| root.app.lookup.match_count()) > 0);
    shell.capture("run-lookup__02-reading-spell-beside-run__1200x800");

    shell.press("ctrl-a");
    shell.type_text("captain varra");
    assert_eq!(
        shell.read(|root| root.app.lookup.query().to_owned()),
        "captain varra"
    );
    shell.capture("run-lookup__05-found-package-and-campaign-entries__1200x800");

    shell.press("escape");
    if shell.read(|root| root.app.lookup.open) {
        shell.press("escape");
    }
    assert!(!shell.read(|root| root.app.lookup.open));
    assert_eq!(selected(&mut shell), CASTER, "the selection is kept");
    shell.capture("run-lookup__06-back-in-the-run__1200x800");

    click_starting(&mut shell, "Ashfall");
    assert!(
        shell.read(|root| root.app.lookup.open),
        "the spell opens beside the run"
    );
    shell.capture("run-lookup__07-spell-from-stat-block__1200x800");

    shell.press("alt-left");
    shell.capture("run-lookup__08-lookup-back-by-key__1200x800");

    let mut shell = small("run-lookup", Seed::Campaign);
    in_the_run(&mut shell);
    open_lookup(&mut shell);
    shell.type_text("fireball");
    shell.capture("run-lookup__02-reading-spell-beside-run__960x640");
}

// ---------------------------------------------------------------------------------------------
// column-resize

/// Scenario `column-resize`, the run's steps: the panel at its narrowest and widest, Next turn with
/// the wide panel, and Lookup beside the run; then the panel's splitter by drag and keys.
#[test]
fn column_resize() {
    let mut shell = Shell::new("column-resize", Seed::Campaign);
    open_saved(&mut shell, RUN);
    shell.update(|_, _, cx| {
        kit::edge::set_column_width("run_panel", Some(0.0), cx);
        cx.notify();
    });
    shell.capture("column-resize__01-run-panel-narrow__1200x800");

    shell.update(|_, _, cx| {
        kit::edge::set_column_width("run_panel", Some(10_000.0), cx);
        cx.notify();
    });
    shell.capture("column-resize__02-run-panel-wide__1200x800");

    let before = current_name(&mut shell);
    shell.click("Next turn");
    assert_ne!(before, current_name(&mut shell));
    shell.capture("column-resize__06-run-rail-keeps-roster-room__1200x800");

    open_lookup(&mut shell);
    shell.capture("column-resize__07-lookup-beside-run-rail__1200x800");

    let mut shell = small("column-resize", Seed::Campaign);
    open_saved(&mut shell, RUN);
    shell.update(|_, _, cx| {
        kit::edge::set_column_width("run_panel", Some(0.0), cx);
        cx.notify();
    });
    shell.capture("column-resize__01-run-panel-narrow__960x640");

    let mut shell = Shell::new("column-resize", Seed::Campaign);
    open_saved(&mut shell, RUN);
    // The panel's splitter (`kit::EdgeColumn`): a drag moves its edge, the width is saved under
    // its id for the next start, Left widens it by `L`, and Home gives the natural width back.
    let saved = |shell: &mut Shell| {
        shell.read(|root| {
            root.remembered
                .as_ref()
                .and_then(|state| state.column_widths.get("run_panel").copied())
        })
    };
    let splitter = "Resize fight panel column";
    let edge = shell.bounds(splitter).expect("the panel splitter");
    let natural = edge.center().x;
    shell.drag(edge.center(), point(natural - px(40.0), edge.center().y));
    let dragged = saved(&mut shell).expect("the dragged width is saved");
    let edge = shell.bounds(splitter).expect("the panel splitter");
    assert!((f32::from(natural - edge.center().x) - 40.0).abs() <= 1.0);
    shell.click_at(edge.center());
    shell.press("left");
    assert_eq!(saved(&mut shell), Some(dragged + 16.0));
    shell.press("home");
    assert_eq!(saved(&mut shell), None);
    let edge = shell.bounds(splitter).expect("the panel splitter");
    assert_eq!(edge.center().x, natural);
}

// ---------------------------------------------------------------------------------------------
// npc-in-encounter

/// Wait while the Encounters page reads its campaign (on a thread of its own).
fn encounters_read(shell: &mut Shell) {
    for _ in 0..400 {
        let reading =
            shell.read(|root| root.app.encounters.busy() || !root.app.encounters.loaded());
        if !reading {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
        shell.wait(std::time::Duration::from_millis(50));
    }
    shell.settle();
}

/// Scenario `npc-in-encounter`: a named NPC added to a new encounter, then the encounter's setup.
#[test]
fn npc_in_encounter() {
    let mut shell = Shell::new("npc-in-encounter", Seed::Campaign);
    shell.update(|root, _, _| {
        let campaign = root.app.campaigns.active_campaign().unwrap();
        let catalogue =
            crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
        let guard = catalogue
            .entries
            .values()
            .find(|entry| entry.entry.kind == "creature" && entry.entry.name == "Guard")
            .unwrap()
            .entry
            .id;
        let text = splat::content::formats::fixture(
            "Rook.md",
            &format!(
                "splat_id: {}\nsplat_kind: world\nsplat_name: Rook Ambry\nsplat_world:\n  type: npc\n  creature: {{id: {guard}}}\n",
                crate::hermetic::new_uuid()
            ),
            "",
        )
        .unwrap();
        std::fs::create_dir_all(campaign.path.join("NPCs")).unwrap();
        crate::library::write_note(&campaign.path, "NPCs/Rook Ambry.md", None, &text).unwrap();
        let modules = root.app.campaigns.modules().to_vec();
        root.app.encounters.refresh_now(&campaign, &modules);
    });
    shell.update(|root, _, cx| root.show_page(Page::Encounters, cx));
    encounters_read(&mut shell);
    shell.click("Reload from disk");
    encounters_read(&mut shell);
    shell.click("New encounter");
    assert!(shell.read(|root| root.app.encounters.editor_open()));
    shell.type_text("Rook’s ambush");
    shell.click("Find a creature or party member");
    shell.type_text("Rook Ambry");
    let add = target_starting(&mut shell, "id:add:");
    shell.click(&add);
    let names = shell.read(|root| root.app.encounters.roster_names().unwrap_or_default());
    assert!(names.iter().any(|name| name == "Rook Ambry"), "{names:?}");
    shell.click("Save encounter");
    assert!(!shell.read(|root| root.app.encounters.encounter_dirty()));

    let path = shell.read(|root| {
        let campaign = root.app.campaigns.active_campaign().unwrap();
        std::fs::read_dir(campaign.path.join("Encounters"))
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .find(|name| name.starts_with("Rook"))
            .expect("the saved encounter")
    });
    start(&mut shell, &format!("Encounters/{path}"));
    assert_eq!(phase(&mut shell), Phase::Setup);
    index(&mut shell, "Rook Ambry");
    shell.capture("npc-in-encounter__03-npc-in-setup__1200x800");
}

// ---------------------------------------------------------------------------------------------
// run-large-roster

/// Scenario `run-large-roster`: rosters of 12 and 20, and the middle of a round of 20.
#[test]
fn run_large_roster() {
    let mut shell = Shell::new("run-large-roster", Seed::LargeRosters);
    open_saved(&mut shell, RUN_12);
    assert_eq!(count(&mut shell), 12);
    shell.capture("run-large-roster__01-roster-of-12__1200x800");

    open_saved(&mut shell, RUN_20);
    assert_eq!(count(&mut shell), 20);
    shell.capture("run-large-roster__02-roster-of-20__1200x800");

    for _ in 0..=10 {
        shell.click("Next turn");
    }
    shell.capture("run-large-roster__03-mid-round__1200x800");

    let mut shell = small("run-large-roster", Seed::LargeRosters);
    open_saved(&mut shell, RUN_20);
    assert_eq!(count(&mut shell), 20);
    shell.capture("run-large-roster__02-roster-of-20__960x640");

    // A roster far longer than its room builds only the rows in view and a view's height each
    // side (`kit::placed_rows`), so a frame costs what a short roster's does; it still scrolls
    // to the selection and says what is out of view.
    let mut shell = Shell::new("run-large-roster", Seed::LargeRosters);
    open_saved(&mut shell, RUN_20);
    let built = || super::live::ROWS_BUILT.with(std::cell::Cell::get);
    let twenty = shell.frame_cost(10);
    let twenty_built = built();
    on_run(&mut shell, |run| {
        let model = run.state.participants[run.state.order[0]].clone();
        for at in 0..280 {
            let mut copy = model.clone();
            copy.key = format!("copy-{at}");
            copy.name = format!("Copy {at}");
            run.state.order.push(run.state.participants.len());
            run.state.participants.push(copy);
        }
    });
    assert_eq!(count(&mut shell), 300);
    shell.settle();
    // A frame in which nothing the rows show changed fits no row again; a change does.
    let made = || super::roster::PLANS_MADE.with(std::cell::Cell::get);
    let before = made();
    let many = shell.frame_cost(10);
    assert_eq!(made(), before, "still frames reuse the roster's plan");
    let first = with_run(&mut shell, |run| run.state.order[0]);
    on_run(&mut shell, |run| {
        run.run_command(Command::Condition(first, "Prone".into()))
    });
    shell.settle();
    assert!(made() > before, "a change fits the rows again");
    on_run(&mut shell, Run::undo);
    eprintln!(
        "run-large-roster frame cost: 20 rows {twenty:?} ({twenty_built} built), 300 rows {many:?} ({} built)",
        built()
    );
    assert!(
        (1..80).contains(&built()),
        "only the rows near the view are built: {}",
        built()
    );
    shell.capture("run-large-roster__04-roster-of-300__1200x800");
    assert!(
        !has_target(&mut shell, "Copy 279"),
        "a row far out of view is not built"
    );
    let last = index(&mut shell, "Copy 279");
    // A selection (a click on a row, the panel's, a step) brings its row into view.
    on_run(&mut shell, |run| run.select(last));
    shell.settle();
    // The last row is in view: the roster is scrolled to its end.
    let (offset, end) = shell.read(|root| {
        let handle = &root.pages.run.roster_scroll;
        (
            -f32::from(handle.offset().y),
            f32::from(handle.max_offset().y),
        )
    });
    assert!(
        offset > 1000.0 && (end - offset).abs() < 1.0,
        "{offset} of {end}"
    );
    assert!(
        has_target(&mut shell, "Copy 279"),
        "the selected row is built once it is in view"
    );
    assert!(
        (1..80).contains(&built()),
        "the view at the end builds its own rows only: {}",
        built()
    );
    shell.capture("run-large-roster__05-last-row-selected__1200x800");
    // The keys step the selection through rows that were never built; each comes into view.
    let first = with_run(&mut shell, |run| run.state.order[0]);
    on_run(&mut shell, |run| run.select(first));
    assert!(!has_target(&mut shell, "Copy 50"));
    for _ in 0..70 {
        shell.press("down");
    }
    assert_eq!(selected(&mut shell), "Copy 50");
    assert!(
        has_target(&mut shell, "Copy 50"),
        "the row the keys reached is built"
    );
}

// ---------------------------------------------------------------------------------------------
// run-absent-player

const ABSENT: &str = "Brannoch Vale";
const OTHERS: [(&str, &str); 4] = [
    ("Iselle Marrowind", "19"),
    ("Kestrel Dunmoor", "21"),
    ("Oren Thistlewick", "11"),
    ("Sable Harrowgate", "13"),
];

fn ready_with_one_absent(shell: &mut Shell) {
    start(shell, SHRINE);
    let at = index(shell, ABSENT);
    on_run(shell, |run| run.run_command(Command::Presence(at, false)));
    shell.click("Roll NPCs");
    for (name, total) in OTHERS {
        shell.click(&format!("Initiative for {name}"));
        shell.type_text(total);
    }
    assert_eq!(with_run(shell, Run::setup_blocked), None, "ready to begin");
}

/// Scenario `run-absent-player`: setup with one absent, the fight begun without them, back with no
/// initiative, its field in the row, and the initiative set in combat.
#[test]
fn run_absent_player() {
    let mut shell = Shell::new("run-absent-player", Seed::Campaign);
    ready_with_one_absent(&mut shell);
    shell.capture("run-absent-player__01-ready-with-one-absent__1200x800");

    shell.click("Begin combat");
    assert_eq!(phase(&mut shell), Phase::Running);
    shell.capture("run-absent-player__02-began-without-them__1200x800");

    shell.click("Next turn");
    let at = index(&mut shell, ABSENT);
    on_run(&mut shell, |run| {
        run.select(at);
        run.run_command(Command::Presence(at, true));
    });
    assert!(with_run(&mut shell, |run| run.state.participants[at].present));
    assert_eq!(
        with_run(&mut shell, |run| run.state.participants[at].initiative),
        None
    );
    shell.capture("run-absent-player__03-back-with-no-initiative__1200x800");

    shell.click(&format!("No initiative for {ABSENT}"));
    assert_eq!(with_run(&mut shell, |run| run.live_initiative), Some(at));
    shell.capture("run-absent-player__04-initiative-field-in-row__1200x800");

    // The field has the keyboard: the typed total and Enter set it.
    shell.type_text("20");
    shell.press("enter");
    assert_eq!(
        with_run(&mut shell, |run| run.state.participants[at].initiative),
        Some(20)
    );
    shell.capture("run-absent-player__05-initiative-set-in-combat__1200x800");

    let mut shell = small("run-absent-player", Seed::Campaign);
    ready_with_one_absent(&mut shell);
    shell.capture("run-absent-player__01-ready-with-one-absent__960x640");
}

// ---------------------------------------------------------------------------------------------
// run-restart

/// Scenario `run-restart`, the run's steps: the run before a restart, and the paused run resumed
/// from its saved file.
#[test]
fn run_restart() {
    let mut shell = Shell::new("run-restart", Seed::Campaign);
    open_saved(&mut shell, RUN);
    select(&mut shell, CASTER);
    shell.click("Next turn");
    let current = current_name(&mut shell);
    shell.capture("run-restart__01-before-restart__1200x800");

    shell.click("id:run-close");
    assert!(!shell.read(|root| root.app.encounters.has_run()));
    open_saved(&mut shell, RUN);
    assert_eq!(
        current_name(&mut shell),
        current,
        "the saved turn comes back"
    );
    shell.capture("run-restart__06-paused-run-resumed-from-rail__1200x800");
}

// ---------------------------------------------------------------------------------------------
// run-roster-conflict

const LOADED_ROW: &str = "Kestrel Dunmoor";
const ONE_MORE_LINE: &str = "Cinder Wolf 3";
const LONG_MESSAGE: &str =
    "Brannoch Vale HP 58 → 54, Iselle Marrowind HP 27 → 18, Kestrel Dunmoor HP 11 → 8";
const LOADED_CONDITIONS: [&str; 5] = ["Blinded", "Charmed", "Grappled", "Paralyzed", "Restrained"];

fn conflict_of_12(shell: &mut Shell) {
    use crate::combat_sync::SyncState;
    open_saved(shell, RUN_12);
    shell.update(|root, _, _| {
        root.app
            .settings
            .simulate(crate::settings::Account::Connected);
        root.app
            .encounters
            .simulate_sync_state("Brannoch Vale", SyncState::Conflict);
    });
    assert_eq!(count(shell), 12);
}

/// Scenario `run-roster-conflict`: a conflict in a roster of 12, a sheet message on one line, the
/// conditions read from their summary, the loaded roster with a fault, and a full round of 20.
#[test]
fn run_roster_conflict() {
    let mut shell = Shell::new("run-roster-conflict", Seed::LargeRosters);
    conflict_of_12(&mut shell);
    shell.capture("run-roster-conflict__01-roster-of-12-with-conflict__1200x800");

    shell.update(|root, _, _| root.app.encounters.simulate_connection());
    if has_target(&mut shell, "Sync party") {
        shell.click("Sync party");
    } else {
        on_run(&mut shell, Run::toggle_party_sync);
    }
    select(&mut shell, LOADED_ROW);
    on_run(&mut shell, Run::open_conditions_tab);
    for condition in LOADED_CONDITIONS {
        shell.click(&format!("id:run-condition-{condition}"));
    }
    shell.update(|root, _, _| root.app.encounters.simulate_sheet_update(LONG_MESSAGE));
    select(&mut shell, ONE_MORE_LINE);
    on_run(&mut shell, Run::open_conditions_tab);
    shell.click("id:run-condition-Prone");
    shell.update(|root, _, _| root.app.encounters.simulate_sheet_update(LONG_MESSAGE));
    assert!(row_text(&mut shell, LOADED_ROW).contains("Restrained"));
    // The sheet's notice keeps one line: a shorter form of its changes, the Log the whole.
    let whole = with_run(&mut shell, |run| run.notice.clone()).expect("the sheet's notice");
    let words = shell.bounds(&whole).expect("the notice under the roster");
    assert!(
        words.size.height < crate::gui::theme::Type::Small.line() * 1.5,
        "the notice is on one line: {words:?}"
    );
    shell.capture("run-roster-conflict__02-message-on-one-line__1200x800");

    select(&mut shell, LOADED_ROW);
    on_run(&mut shell, |run| run.panel = Panel::Reference);
    if has_target(&mut shell, "Read all conditions") {
        shell.click("Read all conditions");
    } else {
        on_run(&mut shell, Run::open_conditions_tab);
    }
    shell.capture("run-roster-conflict__03-all-conditions-from-summary__1200x800");

    select(&mut shell, ONE_MORE_LINE);
    on_run(&mut shell, Run::open_conditions_tab);
    shell.click("id:run-condition-Prone");
    select(&mut shell, LOADED_ROW);
    shell.update(|root, _, _| {
        root.app.encounters.simulate_run_error(
            "Ember Cultist’s stat block could not be read. Its row still works. Repair it in",
        );
    });
    shell.capture("run-roster-conflict__04-loaded-roster-of-12__1200x800");

    shell.update(|root, _, _| root.app.encounters.simulate_run_error(""));
    open_saved(&mut shell, RUN_20);
    for _ in 0..20 {
        shell.click("Next turn");
    }
    assert_eq!(count(&mut shell), 20);
    shell.capture("run-roster-conflict__05-after-full-round__1200x800");
}

// ---------------------------------------------------------------------------------------------
// run-resume, run-no-groups, run-folder-defect, roll-table-author

/// The Encounters page with its campaign read: each read in flight is waited for on its own
/// result, never a clock.
fn encounters_ready(shell: &mut Shell) {
    shell.update(|root, _, cx| root.show_page(Page::Encounters, cx));
    for _ in 0..10 {
        if !shell.read(|root| root.app.encounters.busy()) {
            break;
        }
        shell.update(|root, _, _| root.app.encounters.wait_for_load());
    }
    assert!(
        shell.read(|root| root.app.encounters.loaded() && !root.app.encounters.busy()),
        "the campaign's encounters are read"
    );
}

/// Scenario `run-resume`: from a cold start, Resume on Encounters opens the paused run at the
/// turn it stopped on, with that participant selected.
#[test]
fn run_resume() {
    const INTERRUPTED: &str = "Cinder Wolf 2";
    let mut shell = Shell::new("run-resume", Seed::Campaign);
    encounters_ready(&mut shell);
    shell.click("Resume");
    assert_eq!(shell.read(|root| root.app.page), Page::Run);
    assert_eq!(current_name(&mut shell).as_deref(), Some(INTERRUPTED));
    assert_eq!(selected(&mut shell), INTERRUPTED);
    shell.capture("run-resume__02-resumed__1200x800");
}

/// Scenario `run-no-groups`: Arrivals in a campaign with no groups says how to make one.
#[test]
fn run_no_groups() {
    let mut shell = Shell::new("run-no-groups", Seed::Campaign);
    open_saved(&mut shell, RUN);
    on_run(&mut shell, Run::show_arrivals);
    assert!(
        has_target(&mut shell, "New group"),
        "Arrivals says how to make a group: {:?}",
        kit::probe::names()
    );
    shell.capture("run-no-groups__01-no-groups__1200x800");
}

/// Scenario `run-folder-defect`: the campaign's run folder edited in Rules and runs warns that
/// the runs stay in the old folder, with the choice to move them.
#[test]
fn run_folder_defect() {
    let mut shell = Shell::new("run-folder-defect", Seed::Campaign);
    shell.update(|root, _, cx| root.show_page(Page::Campaign, cx));
    shell.click("Rules and runs");
    for _ in 0..200 {
        if shell.read(|root| root.app.encounters.run_form().is_some()) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
    }
    let id = shell.read(|root| root.app.campaigns.active_campaign().expect("a campaign").id);
    shell.click(&format!("id:run_folder-{id}"));
    shell.press("ctrl-a");
    shell.type_text("Elsewhere");
    assert_eq!(
        shell.read(|root| root.app.encounters.run_form().expect("the form").folder),
        "Elsewhere"
    );
    // The runs the old folder keeps are read after the edit: wait on the choices they bring.
    for _ in 0..200 {
        if has_target(&mut shell, "Move runs and save") {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
    }
    assert!(
        has_target(&mut shell, "Move runs and save"),
        "the folder move offers its choices: {:?}",
        kit::probe::names()
    );
    shell.capture("run-folder-defect__01-run-folder-edited__1200x800");
}

/// Scenario `roll-table-author`: a note's roll tables as the World reader shows them: an
/// interval it cannot read, a typed roll on a valid table, a gap and an overlap, and the same
/// note in Lookup.
#[test]
fn roll_table_author() {
    let body = format!(
        "# Table Workshop\n\n<!-- splat:rolls key=valid dice=1d6 per=fortnight -->\n\n| d6 | Result |\n|---|---|\n| 1 | Rain |\n| 2-5 | Fog |\n| 6 | Sun |\n\n<!-- splat:rolls key=gap dice=1d4 -->\n\n| d4 | Gap |\n|---|---|\n| 1 | One |\n| 3-4 | Later |\n\n<!-- splat:rolls key=overlap dice=1d4 -->\n\n| d4 | Overlap |\n|---|---|\n| 1-3 | First |\n| 3-4 | Second |\n\n{}<!-- splat:rolls key=words dice=1d6 -->\n\n| d6 | Words |\n|---|---|\n| 1 | One |\n| 2 to 5 | Fog |\n| 6 | Six |\n",
        "\nThe next table checks a range written as words.\n".repeat(8)
    );
    let mut shell = Shell::new("roll-table-author", Seed::World);
    let folder = shell.read(|root| {
        root.app
            .campaigns
            .active_campaign()
            .expect("a campaign")
            .path
    });
    std::fs::write(folder.join("Table Workshop.md"), body).expect("write the note");
    shell.update(|root, _, _| root.app.lookup.refresh());
    let mut note = None;
    for _ in 0..600 {
        note = shell.read(|root| {
            root.app.lookup.index().and_then(|index| {
                index
                    .items
                    .iter()
                    .find(|item| item.entry.name == "Table Workshop")
                    .map(|item| item.entry.id)
            })
        });
        if note.is_some() {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
    }
    assert!(note.is_some(), "the note is read");
    shell.update(|root, _, cx| root.show_page(Page::World, cx));
    click_starting(&mut shell, "Table Workshop · ");
    assert!(
        has_target(&mut shell, "Roll 1d6"),
        "the valid table rolls: {:?}",
        kit::probe::names()
    );
    shell.capture("roll-table-author__01-authored-table-validation__1200x800");

    shell.click("Typed roll for Valid");
    shell.type_text("4");
    shell.press("enter");
    assert!(
        has_target(&mut shell, kit::markdown::ROLL_RESULT),
        "a typed roll shows its row"
    );
    shell.capture("roll-table-author__02-authored-table-typed-roll__1200x800");

    let at = shell.bounds("Roll 1d6").expect("the Roll button").center();
    shell.scroll(
        at,
        gpui::ScrollDelta::Pixels(point(px(0.0), px(-600.0))),
        gpui::Modifiers::default(),
    );
    shell.capture("roll-table-author__03-authored-table-gap-and-overlap__1200x800");

    open_lookup(&mut shell);
    shell.type_text("Table Workshop");
    shell.press("enter");
    assert!(
        has_target(&mut shell, "Roll 1d6"),
        "Lookup's table rolls too"
    );
    shell.capture("roll-table-author__04-authored-table-in-lookup__1200x800");
}
