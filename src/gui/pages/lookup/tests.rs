//! Lookup: screenshots of the panel beside a page, and its flows: Ctrl+K
//! opens it with the field focused, typing finds and opens the best match, the arrows move
//! through the results, Back returns, Escape closes.
use crate::fixtures::Seed;
use crate::gui::harness::Shell;

fn wait_for_index(shell: &mut Shell) {
    for _ in 0..200 {
        if shell.read(|root| root.app.lookup.index().is_some()) {
            return;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    panic!("Lookup's index was not read");
}

#[test]
fn search_open_and_back() {
    let mut shell = Shell::new("run-lookup", Seed::Campaign);
    shell.press("ctrl-k");
    assert!(shell.read(|root| root.app.lookup.open));
    wait_for_index(&mut shell);
    shell.capture("run-lookup__00-empty-search__1200x800");
    shell.type_text("fire");
    assert_eq!(
        shell.read(|root| root.app.lookup.query().to_owned()),
        "fire"
    );
    let count = shell.read(|root| root.app.lookup.match_count());
    assert!(count > 1, "fire finds several entries: {count}");
    // The best match opens while the others stay listed.
    assert!(shell.read(|root| matches!(
        root.app.lookup.showing(),
        crate::lookup::panel_view::Showing::Entry(_)
    )));
    assert!(shell.read(|root| root.app.lookup.results_shown()));
    shell.capture("run-lookup__02-reading-spell-beside-run__1200x800");
    let first = shell.read(entry_name);
    shell.press("down");
    let second = shell.read(entry_name);
    assert_ne!(first, second, "Down opens the next match");
    // A press on a row reads it and closes the list.
    shell.click("id:lookup-result-0");
    assert_eq!(shell.read(entry_name), first);
    assert!(!shell.read(|root| root.app.lookup.results_shown()));
    shell.press("escape");
    assert!(
        !shell.read(|root| root.app.lookup.open),
        "Escape after a row press closes Lookup"
    );
    // Ctrl+K opens on the last words, selected, so typing replaces them.
    shell.press("ctrl-k");
    shell.type_text("goblin");
    assert_eq!(
        shell.read(|root| root.app.lookup.query().to_owned()),
        "goblin",
        "the old words were selected"
    );
    // Clear empties the field and the reader; then Generate NPC holds an unsaved NPC.
    shell.click("Clear");
    assert!(shell.read(|root| matches!(
        root.app.lookup.showing(),
        crate::lookup::panel_view::Showing::Nothing
    )));
    shell.click("Generate NPC");
    let npc = shell.read(|root| root.app.lookup.generated_name().map(str::to_owned));
    assert!(npc.is_some(), "Generate NPC holds an NPC");
    shell.click(crate::lookup::SEARCH_NAME);
    shell.type_text("goblin");
    assert!(
        shell.read(entry_name).is_some(),
        "the search opens an entry over the card"
    );
    // Emptying the field from the keyboard keeps the entry and the unsaved NPC.
    for _ in 0.."goblin".len() {
        shell.press("backspace");
    }
    assert_eq!(shell.read(|root| root.app.lookup.query().to_owned()), "");
    assert_eq!(
        shell.read(|root| root.app.lookup.generated_name().map(str::to_owned)),
        npc,
        "the unsaved NPC stays"
    );
    // Each shorter word opens its own best match; the last one stays on screen.
    assert!(shell.read(entry_name).is_some(), "an entry stays on screen");
}

/// Escape in the search field closes Lookup.
#[test]
fn escape_in_the_field_closes() {
    let mut shell = Shell::new("lookup-escape", Seed::Campaign);
    shell.press("ctrl-k");
    wait_for_index(&mut shell);
    shell.type_text("fire");
    shell.press("escape");
    assert!(!shell.read(|root| root.app.lookup.open));
}

/// Nothing found says so, and offers New NPC for the typed name.
#[test]
fn nothing_found_offers_new_entries() {
    let mut shell = Shell::new("lookup-new", Seed::Campaign);
    shell.press("ctrl-k");
    wait_for_index(&mut shell);
    shell.type_text("Zzyzx Qwerty");
    assert_eq!(shell.read(|root| root.app.lookup.match_count()), 0);
    assert!(shell.bounds("New NPC “Zzyzx Qwerty”").is_some());
    shell.capture("lookup-new__01-nothing-found__1200x800");
}

/// A World entry of `kind` at `path` in the open campaign.
fn world_entry(shell: &mut Shell, path: &str, name: &str, kind: &str, fields: &str) -> uuid::Uuid {
    let id = splat::character::stable_id(path);
    let file = shell.read(|root| {
        root.app
            .campaigns
            .active_campaign()
            .unwrap()
            .path
            .join(path)
    });
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(
        file,
        splat::content::formats::fixture(
            "World.md",
            &format!(
                "splat_id: {id}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n  type: {kind}\n{fields}"
            ),
            &format!("\n# {name}\n\nA note for tonight.\n"),
        )
        .unwrap(),
    )
    .unwrap();
    id
}

/// Read the content again and wait until the index holds `id`.
fn reload(shell: &mut Shell, id: uuid::Uuid) {
    shell.update(|root, _, _| root.app.lookup.refresh());
    for _ in 0..1500 {
        std::thread::sleep(std::time::Duration::from_millis(20));
        shell.wait(std::time::Duration::from_millis(50));
        if shell.read(|root| {
            root.app
                .lookup
                .index()
                .is_some_and(|index| index.position(id).is_some())
        }) {
            return;
        }
    }
    panic!("the content did not load again");
}

/// The fixture's run in progress, opened as Resume opens it.
fn open_run(shell: &mut Shell) {
    shell.update(|root, _, cx| {
        let campaign = root.app.campaigns.active_campaign().unwrap();
        let modules = root.app.campaigns.modules().to_vec();
        root.app.encounters.refresh_now(&campaign, &modules);
        let path = root
            .app
            .encounters
            .saved_runs()
            .iter()
            .find(|run| run.encounter == crate::fixtures::RUN)
            .expect("the fixture's saved run")
            .path
            .clone();
        root.app.encounters.open_saved_run(&path, &campaign);
        root.show_page(crate::Page::Run, cx);
    });
    assert!(shell.read(|root| root.app.encounters.has_run()));
}

/// An NPC in Lookup is World's reader, compact: its title line's Edit,
/// Add to fight and Add to Log, its Stat block and Place rows with Change, its links, the entry's
/// end sections; Add to fight adds it to the run; its edit form saves, and its Undo takes the
/// change back.
#[test]
fn npc_in_lookup() {
    let mut shell = Shell::new("npc-in-lookup", Seed::Campaign);
    wait_for_index(&mut shell);
    let guard = shell.read(|root| {
        let index = root.app.lookup.index().unwrap();
        index
            .items
            .iter()
            .find(|item| item.entry.name == "Guard")
            .map(|item| item.entry.id)
            .expect("the Guard stat block")
    });
    let place = world_entry(
        &mut shell,
        "Locations/Watch House.md",
        "Watch House",
        "location",
        "",
    );
    let rook = world_entry(
        &mut shell,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        &format!("  creature: {{id: {guard}}}\n  in: {{id: {place}}}\n"),
    );
    reload(&mut shell, rook);
    open_run(&mut shell);
    shell.press("ctrl-k");
    shell.type_text("Rook Ambry");
    assert_eq!(shell.read(entry_name).as_deref(), Some("Rook Ambry"));
    for name in [
        "Edit Rook Ambry",
        "Add Rook Ambry to the fight",
        "Add to Log",
        "Change Stat block",
        "Change Place",
        "Guard",
        "Watch House",
        "Source and identifiers",
    ] {
        assert!(
            shell.bounds(name).is_some(),
            "{name} shows: {:?}",
            crate::gui::kit::probe::names()
        );
    }
    shell.capture("npc-in-lookup__01-npc-with-stat-block-and-place__1200x800");
    // Add to fight: one press, and Rook joins the run under his own name.
    shell.click("Add Rook Ambry to the fight");
    shell.settle();
    assert!(
        shell.read(|root| root.app.encounters.run().is_some_and(|run| run
            .state
            .participants
            .iter()
            .any(|p| p.name == "Rook Ambry"))),
        "Rook Ambry joined the run"
    );
    // The run's Undo takes him back, so the run is as it was before the link below.
    let undo = crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with("Undo") && name.contains("add Rook Ambry"))
        .expect("the run's Undo of the add");
    shell.click(&undo);
    assert!(
        shell.read(|root| root.app.encounters.run().is_some_and(|run| !run
            .state
            .participants
            .iter()
            .any(|p| p.name == "Rook Ambry"))),
        "Undo takes Rook Ambry out of the run"
    );
    // The stat block link opens the Guard, a creature, with its stat block, and adds nothing.
    shell.click("Guard");
    assert_eq!(shell.read(entry_name).as_deref(), Some("Guard"));
    assert!(
        shell.bounds("Source and identifiers").is_some(),
        "the Guard reads as an entry"
    );
    shell.capture("npc-in-lookup__02-npc-stat-block-link__1200x800");
    shell.click("id:lookup-back");
    shell.click("Watch House");
    assert_eq!(shell.read(entry_name).as_deref(), Some("Watch House"));
    shell.capture("npc-in-lookup__03-npc-place-link__1200x800");
    // Rook moves and makes a promise; the DM writes both down without leaving the fight.
    let north = world_entry(
        &mut shell,
        "Locations/North Gate.md",
        "North Gate",
        "location",
        "",
    );
    reload(&mut shell, north);
    shell.click("id:lookup-back");
    assert_eq!(shell.read(entry_name).as_deref(), Some("Rook Ambry"));
    // The edit form in Lookup: a new Place and a new line of Text, then Save changes, and its
    // Undo.
    shell.click("Edit Rook Ambry");
    assert!(
        shell.bounds("Save changes").is_some(),
        "the edit form shows"
    );
    shell.click("Change Place");
    shell.type_text("North");
    shell.click("North Gate");
    shell.click("Text");
    shell.press("ctrl-end");
    shell.press("enter");
    shell.type_text("Never names the captain. Owes Nell a favour.");
    assert!(
        shell.bounds("id:world-pick-cancel").is_none(),
        "the place is chosen and the picker closed"
    );
    shell.capture("npc-in-lookup__04-npc-edit-form-in-lookup__1200x800");
    let save = shell.bounds("Save changes").unwrap();
    assert!(
        save.bottom() <= shell.size().height && save.right() <= shell.size().width,
        "Save changes is inside the panel: {save:?}"
    );
    shell.click("Save changes");
    let path = shell.read(|root| {
        root.app
            .campaigns
            .active_campaign()
            .unwrap()
            .path
            .join("NPCs/Rook Ambry.md")
    });
    let saved = std::fs::read_to_string(&path).unwrap();
    assert!(
        saved.contains("Never names the captain. Owes Nell a favour.")
            && saved.contains("North Gate")
            && !saved.contains(&place.to_string())
            && !saved.contains("Watch House"),
        "Save changes writes the new place and line: {saved}"
    );
    reload(&mut shell, rook);
    shell.settle();
    assert!(
        shell.bounds("Undo change to Rook Ambry").is_some(),
        "the result has its Undo in Lookup's footer: {:?}",
        crate::gui::kit::probe::names()
    );
    shell.capture("npc-in-lookup__05-npc-edit-saved-in-lookup__1200x800");
    shell.click("Undo change to Rook Ambry");
    assert!(
        !std::fs::read_to_string(&path)
            .unwrap()
            .contains("Owes Nell a favour"),
        "Undo takes the change back"
    );
}

fn entry_name(root: &crate::gui::SplatRoot) -> Option<String> {
    let crate::lookup::panel_view::Showing::Entry(at) = root.app.lookup.showing() else {
        return None;
    };
    root.app
        .lookup
        .index()
        .and_then(|index| index.items.get(at).map(|item| item.entry.name.clone()))
}
