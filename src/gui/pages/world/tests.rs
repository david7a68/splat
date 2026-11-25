//! The World page's screenshots and interaction tests. Captures pair with the references by
//! name (`<scenario>__<step>__<w>x<h>`), in `artifacts/screenshots/gpui/`.
use crate::fixtures::Seed;
use crate::gui::harness::Shell;
use crate::{Page, world::page::View};
use gpui::{px, size};
use std::path::PathBuf;
use std::time::Duration;
use uuid::Uuid;

/// The open campaign's folder.
fn root(shell: &mut Shell) -> PathBuf {
    shell.read(|root| {
        root.app
            .campaigns
            .active_campaign()
            .expect("a campaign")
            .path
    })
}

/// A World entry of `kind` at `path`, as the scenarios write one.
fn entry(shell: &mut Shell, path: &str, name: &str, kind: &str, fields: &str) -> Uuid {
    let id = splat::character::stable_id(path);
    let file = root(shell).join(path);
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

/// Read the notes again and wait until the index holds `id` (or, with `None`, any new load).
fn reload(shell: &mut Shell, id: Option<Uuid>) {
    let before = shell.read(|root| root.app.lookup.index().map(|index| index.generation()));
    shell.update(|root, _, _| root.app.lookup.refresh());
    // The notes load on a background thread, in real time: wait up to 30 s of it.
    for _ in 0..1500 {
        std::thread::sleep(Duration::from_millis(20));
        shell.wait(Duration::from_millis(50));
        let ready = shell.read(|root| {
            root.app.lookup.index().is_some_and(|index| {
                Some(index.generation()) != before
                    && id.is_none_or(|id| index.position(id).is_some())
            })
        });
        if ready {
            return;
        }
    }
    panic!("the notes did not load again");
}

/// Wait, in real time, until `ready` holds (the app reads the notes again by itself).
fn until(shell: &mut Shell, what: &str, ready: impl Fn(&crate::gui::SplatRoot) -> bool) {
    for _ in 0..1500 {
        if shell.read(&ready) {
            return;
        }
        std::thread::sleep(Duration::from_millis(20));
        shell.wait(Duration::from_millis(50));
    }
    panic!("{what}: it did not happen");
}

/// A World note in the flat form (`splat_type` and the other values at the top level), as the
/// egui scenarios write one.
fn flat_entry(shell: &mut Shell, path: &str, name: &str, fields: &str, body: &str) -> Uuid {
    let id = splat::character::stable_id(path);
    let file = root(shell).join(path);
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(
        file,
        format!(
            "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: world\nsplat_name: {name}\n{fields}\n---\n\n# {name}\n\n{body}"
        ),
    )
    .unwrap();
    id
}

/// Press the target whose name starts with `prefix` (a list row: `Wren · alive`).
fn click_starting(shell: &mut Shell, prefix: &str) {
    shell.settle();
    let names = crate::gui::kit::probe::names();
    let name = names
        .iter()
        .find(|name| name.starts_with(prefix))
        .unwrap_or_else(|| panic!("no target starts with {prefix:?}: {names:?}"))
        .clone();
    shell.click(&name);
}

/// The note Lookup shows, by its name.
fn lookup_shows(shell: &mut Shell) -> Option<String> {
    shell.read(|root| {
        let crate::lookup::panel_view::Showing::Entry(at) = root.app.lookup.showing() else {
            return None;
        };
        root.app
            .lookup
            .index()
            .and_then(|index| index.items.get(at).map(|item| item.entry.name.clone()))
    })
}

/// Open Lookup and search for `name`: it shows that note. The words go in as one input, as
/// egui's scenarios typed them: Lookup opened again on the same words keeps its result list.
fn look_up(shell: &mut Shell, name: &str) {
    shell.click("Lookup");
    shell.insert_text(name);
    shell.settle();
    let query = shell.read(|root| (root.app.lookup.open, root.app.lookup.query().to_owned()));
    assert_eq!(
        lookup_shows(shell).as_deref(),
        Some(name),
        "Lookup shows {name}: {query:?} {:?}",
        crate::gui::kit::probe::names()
    );
}

/// The id of the note named `name`.
fn note_id(shell: &mut Shell, name: &str) -> Uuid {
    shell.read(|root| {
        root.app
            .lookup
            .index()
            .and_then(|index| {
                index
                    .items
                    .iter()
                    .find(|item| item.entry.name == name)
                    .map(|item| item.entry.id)
            })
            .unwrap_or_else(|| panic!("the note {name}"))
    })
}

/// The World page, with its notes loaded.
fn world(name: &str, window: gpui::Size<gpui::Pixels>) -> Shell {
    world_over(name, Seed::World, window)
}

/// The World page over the fixture `seed`, with its notes loaded.
fn world_over(name: &str, seed: Seed, window: gpui::Size<gpui::Pixels>) -> Shell {
    let mut shell = Shell::with_size(name, seed, window);
    shell.update(|root, _, cx| root.show_page(Page::World, cx));
    for _ in 0..1500 {
        if shell.read(|root| root.app.lookup.index().is_some()) {
            return shell;
        }
        std::thread::sleep(Duration::from_millis(20));
        shell.wait(Duration::from_millis(50));
    }
    panic!("{name}: the notes did not load");
}

fn default_size() -> gpui::Size<gpui::Pixels> {
    size(px(1200.0), px(800.0))
}

fn small_size() -> gpui::Size<gpui::Pixels> {
    size(px(960.0), px(640.0))
}

/// The list, New's menu, an NPC's form and the note it makes (scenario `world`).
#[test]
fn world_list_and_new_npc() {
    let mut shell = world("world", small_size());
    shell.capture("world__01-world-list__960x640");
    let mut shell = world("world", default_size());
    shell.capture("world__01-world-list__1200x800");
    shell.click("New ▾");
    shell.capture("world__02-world-new-menu__1200x800");
    shell.click("NPC…");
    assert_eq!(
        shell.read(|root| root.app.world_page.new_kind()),
        Some("npc")
    );
    shell.click("Name");
    shell.type_text("Watchkeeper");
    assert_eq!(
        shell.read(|root| root.app.world_page.new_fields().0.to_owned()),
        "Watchkeeper"
    );
    shell.capture("world__03-world-new-npc-form__1200x800");
    shell.click("Create NPC");
    let file = root(&mut shell).join("NPCs/Watchkeeper.md");
    assert!(file.exists(), "Create NPC writes {file:?}");
    reload(&mut shell, None);
    assert!(shell.read(|root| root.app.world_page.new_kind().is_none()));
    assert!(shell.read(|root| root.app.world_page.selected().is_some()));
    shell.capture("world__04-world-new-note-created__1200x800");
}

/// An NPC's place and stat block, a change with its Undo (scenario `world-places`).
#[test]
fn world_places_change_and_undo() {
    let mut shell = world("world-places", default_size());
    entry(
        &mut shell,
        "Locations/Ashfall Keep.md",
        "Ashfall Keep",
        "location",
        "  aliases: [The Keep]\n",
    );
    let rook = entry(
        &mut shell,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        "  in: '[[The Keep]]'\n",
    );
    entry(
        &mut shell,
        "Locations/Loop One.md",
        "Loop One",
        "location",
        "  in: '[[Loop Two]]'\n",
    );
    entry(
        &mut shell,
        "Locations/Loop Two.md",
        "Loop Two",
        "location",
        "  in: '[[Loop One]]'\n",
    );
    std::fs::write(
        root(&mut shell).join("Visitors.md"),
        "# Visitors\n\nRook Ambry guards [[Ashfall Keep]].\n",
    )
    .unwrap();
    reload(&mut shell, Some(rook));
    shell.update(|root, _, _| root.app.world_page.press_note(rook));
    shell.capture("world-places__01-npc-place__1200x800");
    // One Change column (iter-3 D): each value's Change starts at the same x, whatever its
    // value's width (`not set` and `Ashfall Keep`).
    let lefts: Vec<_> = ["Change Stat block", "Change Place"]
        .map(|name| {
            shell
                .bounds(name)
                .unwrap_or_else(|| panic!("{name}"))
                .origin
                .x
        })
        .into();
    assert!(
        lefts.iter().all(|x| (*x - lefts[0]).abs() < gpui::px(1.0)),
        "the Changes share one column: {lefts:?}"
    );
    let path = root(&mut shell).join("NPCs/Rook Ambry.md");
    let original = std::fs::read(&path).unwrap();
    shell.click("Change Stat block");
    shell.type_text("Guard");
    shell.click("Guard");
    assert!(
        std::fs::read_to_string(&path)
            .unwrap()
            .contains("splat_stat_block:")
    );
    reload(&mut shell, None);
    shell.capture("world-places__02-npc-stat-block-changed__1200x800");
    shell.click("Undo change to Stat block");
    assert_eq!(
        std::fs::read(&path).unwrap(),
        original,
        "Undo restores the bytes"
    );
}

/// Open questions, from the egui fixture: each question has its tick box, the box writes `[x]`
/// to that line of its note, and `Undo tick` opens it again (scenario `world-facts`).
#[test]
fn world_facts_questions_tick_and_undo() {
    let mut shell = world("world-facts", default_size());
    let horn = flat_entry(
        &mut shell,
        "Lore/Horn Rings.md",
        "Horn Rings",
        "splat_type: lore\nsplat_in: ''\nsplat_status: ''\nsplat_part_of: []",
        "Tieflings wear one ring on their horns for each kept ancestor.\n\n- [ ] Can a horn ring be sold?\n- [x] A horn ring marks an ancestor.\n",
    );
    for (path, name, fields, body) in [
        (
            "Factions/Arena Guild.md",
            "Arena Guild",
            "splat_type: faction",
            "The organizers of the race, under court supervision.\n",
        ),
        (
            "Places/The Arena.md",
            "The Arena",
            "splat_type: location",
            "The arena at the capital, where the race starts.\n",
        ),
        (
            "NPCs/Zarel.md",
            "Zarel",
            "splat_type: npc\nsplat_status: alive\nsplat_part_of:\n- '[[Arena Guild]]'",
            "Tiefling. The Games-Master, who runs the race.\n\n- **Look:** Gilded [[Horn Rings|horn rings]], loud and new.\n- **Says:** \"Filed!\"\n\nSee [[Horn Rings]].\n",
        ),
        (
            "NPCs/Haltha.md",
            "Haltha",
            "splat_type: npc\nsplat_status: alive",
            "- **Look:** Dull iron [[Horn Rings|horn rings]], and old scars that look a little too even.\n",
        ),
        (
            "NPCs/Tordra.md",
            "Tordra",
            "splat_type: npc\nsplat_status: alive",
            "An elf convict, who frees other prisoners.\n",
        ),
        (
            "History/The Revolt.md",
            "The Revolt",
            "splat_type: event\nsplat_when: 90 years ago\nsplat_until: 85 years ago",
            "The miners took the Arena and held it for five years.\n",
        ),
        (
            "History/The Long Siege.md",
            "The Long Siege",
            "splat_type: event\nsplat_when: In the winter before the founding",
            "The guild held the Arena through the winter siege, and fed the miners’ families from its own stores until the thaw came and the soldiers went home.\n",
        ),
        (
            "History/The Massacre.md",
            "The Massacre",
            "splat_type: event\nsplat_when: 120 years ago",
            "Soldiers fired on protesting miners at [[The Arena]].\n",
        ),
    ] {
        flat_entry(&mut shell, path, name, fields, body);
    }
    std::fs::write(
        root(&mut shell).join("Visitors.md"),
        "# Visitors\n\nSee [[Horn Rings]].\n",
    )
    .unwrap();
    reload(&mut shell, Some(horn));
    shell.click("Open questions · 1 question");
    assert_eq!(
        shell.read(|root| root.app.world_page.view()),
        View::Questions
    );
    let question = "Open: Can a horn ring be sold?";
    assert!(
        shell.bounds(question).is_some(),
        "the question has its tick box"
    );
    shell.capture("world-facts__13-open-questions__1200x800");
    let path = root(&mut shell).join("Lore/Horn Rings.md");
    let open = std::fs::read_to_string(&path).unwrap();
    shell.click(question);
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        open.replacen(
            "- [ ] Can a horn ring be sold?",
            "- [x] Can a horn ring be sold?",
            1
        ),
        "the box ticks that line of its note"
    );
    reload(&mut shell, None);
    shell.capture("world-facts__14-question-ticked__1200x800");
    let undo = undo_words(
        crate::design::outcome::Verb::Tick,
        "“Can a horn ring be sold?”",
    );
    let undo = shell
        .bounds(&undo)
        .map(|_| undo.clone())
        .unwrap_or_else(|| format!("{undo} in Horn Rings"));
    shell.click(&undo);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), open, "Undo tick");
}

/// The words of an Undo, as the footer's line names it.
fn undo_words(verb: crate::design::outcome::Verb, object: &str) -> String {
    crate::design::outcome::UndoStep::of(verb, object).label()
}

/// Generate NPC holds a card; the checks view and another note keep its row above them; Discard
/// has an Undo that brings the same NPC back (scenario `world-held-npc`, the egui fixture: three
/// NPCs in `People/`, an open question and a note that cannot be read).
#[test]
fn world_held_npc_row_discard_and_undo() {
    let mut shell = world("world-held-npc", default_size());
    let mut wren = None;
    for name in ["Wren", "Oskar", "Ilsa"] {
        let id = entry(&mut shell, &format!("People/{name}.md"), name, "npc", "");
        wren.get_or_insert(id);
    }
    std::fs::write(
        root(&mut shell).join("Bells.md"),
        "# Bells\n\nThe bells ring at dusk.\n\n- [ ] Who rings the bells?\n",
    )
    .unwrap();
    std::fs::write(
        root(&mut shell).join("Broken.md"),
        "---\nsplat_schema: 2\nsplat_id: [\n---\n",
    )
    .unwrap();
    reload(&mut shell, wren);
    assert!(
        shell.bounds("1 note cannot be read").is_some(),
        "the fixture has a note that cannot be read"
    );
    shell.click("New ▾");
    shell.click("Generate NPC");
    let name = shell
        .read(|root| root.app.world_page.held_npc().map(str::to_owned))
        .expect("a held NPC");
    assert!(shell.bounds("Save NPC").is_some(), "the card shows");
    shell.capture("world-held-npc__01-held-npc-card__1200x800");
    let row = format!("{}: {name}", crate::world::UNSAVED_NPC);
    click_starting(&mut shell, "Things to check · ");
    assert!(shell.bounds(&row).is_some(), "the held NPC keeps its row");
    shell.capture("world-held-npc__02-held-npc-row-over-problems__1200x800");
    click_starting(&mut shell, "Wren · ");
    assert!(shell.bounds(&row).is_some(), "the held NPC keeps its row");
    assert!(shell.bounds("Save NPC").is_none(), "the note took the pane");
    shell.capture("world-held-npc__03-held-npc-row-over-note__1200x800");
    shell.click("Show");
    shell.click("Discard");
    assert!(shell.read(|root| root.app.world_page.held_npc().is_none()));
    let undo = undo_words(crate::design::outcome::Verb::Discard, &name);
    assert!(shell.bounds(&undo).is_some(), "Discard has {undo:?}");
    shell.capture("world-held-npc__04-held-npc-discarded__1200x800");
    shell.click(&undo);
    assert_eq!(
        shell.read(|root| root.app.world_page.held_npc().map(str::to_owned)),
        Some(name),
        "Undo brings the same NPC back"
    );
}

/// The egui round trip: a note re-read after a disk edit, a new faction note, an edit with a
/// typed status and Ctrl+S; an open Log line leaves the form's Save and Cancel open; a disk edit
/// under a changed draft waits for the DM's choice, and Reload takes the file (scenario
/// `world-edit-round-trip`).
#[test]
fn world_edit_round_trip() {
    let mut shell = world("world-edit-round-trip", default_size());
    click_starting(&mut shell, "Setting · ");
    assert!(shell.bounds("Edit in Obsidian").is_some());
    shell.capture("world-edit-round-trip__01-before-disk-edit__1200x800");
    let setting = root(&mut shell).join("Setting.md");
    let edited = "# Setting\n\nThe cold war ended this morning.\n";
    std::fs::write(&setting, edited).unwrap();
    reload(&mut shell, None);
    let id = note_id(&mut shell, "Setting");
    assert!(
        shell.read(|root| {
            let index = root.app.lookup.index().unwrap();
            index.items[index.position(id).unwrap()]
                .entry
                .body
                .contains("The cold war ended this morning.")
        }),
        "the note is read again"
    );
    assert_eq!(shell.read(|root| root.app.world_page.selected()), Some(id));
    assert_eq!(
        std::fs::read_to_string(&setting).unwrap(),
        edited,
        "reading writes nothing"
    );
    shell.capture("world-edit-round-trip__02-after-disk-edit__1200x800");
    shell.click("New ▾");
    shell.click("Note…");
    shell.click("Name");
    shell.type_text("Ashfall Faction");
    shell.click("Type (optional)");
    shell.type_text("faction");
    shell.click("Create note");
    let path = root(&mut shell).join("Factions/Ashfall Faction.md");
    assert!(
        std::fs::read_to_string(&path)
            .unwrap()
            .contains("type: faction"),
        "New Note takes an optional World type"
    );
    reload(&mut shell, None);
    let faction = note_id(&mut shell, "Ashfall Faction");
    assert_eq!(
        shell.read(|root| root.app.world_page.selected()),
        Some(faction)
    );
    shell.capture("world-edit-round-trip__03-new-faction-note__1200x800");
    shell.click("Edit Ashfall Faction");
    let campaign = shell.read(|root| root.app.campaigns.active_campaign().unwrap());
    let editing = |shell: &mut Shell| {
        shell.read(|root| root.app.world_page.reader.editing(faction, &campaign))
    };
    assert!(editing(&mut shell));
    shell.click("Status");
    shell.type_text("ready");
    shell.capture("world-edit-round-trip__04-world-edit-without-run__1200x800");
    shell.press("ctrl-s");
    assert!(
        std::fs::read_to_string(&path).unwrap().contains("ready"),
        "Ctrl+S saves the status"
    );
    reload(&mut shell, None);
    assert!(!editing(&mut shell));
    // An open Log line waits for the form, and the form's own buttons still close it: a clean
    // form's Cancel, and Save once it has changes.
    shell.click("Add to Log");
    shell.click("Edit Ashfall Faction");
    assert!(editing(&mut shell));
    shell.click("Cancel");
    assert!(!editing(&mut shell), "Cancel closes the clean form");
    shell.click("Edit Ashfall Faction");
    shell.click("Status");
    shell.type_text(" now");
    shell.click("Save changes");
    assert!(
        std::fs::read_to_string(&path)
            .unwrap()
            .contains("ready now"),
        "Save writes while the Log line waits"
    );
    reload(&mut shell, None);
    assert!(!editing(&mut shell));
    // A disk edit under a changed draft.
    shell.click("Edit Ashfall Faction");
    shell.click("Status");
    shell.type_text(" later");
    let outside = std::fs::read_to_string(&path).unwrap() + "\nChanged in Obsidian.\n";
    std::fs::write(&path, &outside).unwrap();
    reload(&mut shell, None);
    shell.press("ctrl-s");
    assert!(
        shell.bounds("Reload and lose my draft").is_some(),
        "a disk edit waits for the DM's choice"
    );
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        outside,
        "nothing written"
    );
    shell.click("Reload and lose my draft");
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        outside,
        "Reload writes nothing"
    );
    assert!(shell.bounds("Reload and lose my draft").is_none());
}

/// A note that cannot be read: the header's count opens its repair, which saves the fixed text
/// and opens the note with `Note repaired.` (scenario `world-broken-note`).
#[test]
fn world_broken_note_repair() {
    let mut shell = world("world-broken-note", default_size());
    let path = root(&mut shell).join("NPCs/Broken.md");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let id = splat::character::stable_id("NPCs/Broken.md");
    std::fs::write(
        &path,
        format!("---\nsplat_id: {id}\nsplat_kind: world\nsplat_name: [Broken\n---\n\n# Broken\n"),
    )
    .unwrap();
    reload(&mut shell, None);
    shell.capture("world-broken-note__01-broken-note-listed__1200x800");
    shell.click("1 note cannot be read");
    assert!(shell.read(|root| root.app.world_page.repair_open("NPCs/Broken.md")));
    // Save refuses the text as it stands, changed or not: the row says Invalid, not No changes.
    assert!(
        shell.bounds("Save row: Invalid").is_some(),
        "{:?}",
        crate::gui::kit::probe::names()
    );
    shell.capture("world-broken-note__02-broken-note-repair__1200x800");
    let fixed = splat::content::formats::fixture(
        "World.md",
        &format!(
            "splat_id: {id}\nsplat_kind: world\nsplat_name: Broken\nsplat_world:\n  type: npc\n"
        ),
        "\n# Broken\n",
    )
    .unwrap();
    shell.update(|root, _, _| {
        root.app
            .world_page
            .repair_mut()
            .expect("the repair")
            .set_text(&fixed);
    });
    shell.press("ctrl-s");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), fixed);
    reload(&mut shell, Some(id));
    assert_eq!(shell.read(|root| root.app.world_page.selected()), Some(id));
    shell.capture("world-broken-note__10-broken-note-repaired__1200x800");
}

/// Escape closes an open picker, and the page keeps the note.
#[test]
fn world_escape_closes_a_picker() {
    let mut shell = world("world-escape", default_size());
    let rook = entry(&mut shell, "NPCs/Rook.md", "Rook", "npc", "");
    reload(&mut shell, Some(rook));
    shell.update(|root, _, _| root.app.world_page.press_note(rook));
    shell.click("Change Stat block");
    assert!(shell.read(|root| root.app.world_page.reader.picker_view().is_some()));
    shell.press("escape");
    assert!(shell.read(|root| root.app.world_page.reader.picker_view().is_none()));
    assert_eq!(
        shell.read(|root| root.app.world_page.selected()),
        Some(rook)
    );
}

/// A file renamed outside Splat: the note offers both renames; `Rename the note` writes the
/// file's name with an Undo (scenario `world-note-renamed`).
#[test]
fn world_note_renamed_takes_the_file_name() {
    let mut shell = world("world-note-renamed", default_size());
    let rook = entry(&mut shell, "NPCs/Rook Ambry.md", "Rook Ambry", "npc", "");
    let root_path = root(&mut shell);
    std::fs::rename(
        root_path.join("NPCs/Rook Ambry.md"),
        root_path.join("NPCs/Rook Vale.md"),
    )
    .unwrap();
    reload(&mut shell, Some(rook));
    shell.update(|root, _, _| root.app.world_page.press_note(rook));
    assert!(shell.bounds("Rename the file to Rook Ambry.md").is_some());
    shell.capture("world-note-renamed__01-renamed-npc-keeps-old-name-in-world__1200x800");
    shell.click("Rename the note to Rook Vale");
    let text = std::fs::read_to_string(root_path.join("NPCs/Rook Vale.md")).unwrap();
    assert!(text.contains("splat_name: Rook Vale"), "{text}");
    reload(&mut shell, None);
    shell.capture("world-note-renamed__03-renamed-npc-name-updated__1200x800");
}

/// Two files with one identity: each has its repair, in World and in Lookup; the copy's repair
/// keeps the stat block and has its Undo (scenario `world-note-duplicated`, over the egui
/// fixture: the campaign with its Guard stat block).
#[test]
fn world_note_duplicated_offers_each_repair() {
    let mut shell = world_over("world-note-duplicated", Seed::Campaign, default_size());
    let guard = note_id(&mut shell, "Guard");
    entry(
        &mut shell,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        &format!("  creature: {{id: {guard}}}\n"),
    );
    let root_path = root(&mut shell);
    let original = root_path.join("NPCs/Rook Ambry.md");
    let copy = root_path.join("NPCs/Rook Ambry copy.md");
    let before = std::fs::read_to_string(&original).unwrap();
    std::fs::copy(&original, &copy).unwrap();
    reload(&mut shell, None);
    shell.click(&format!(
        "Rook Ambry.md · {} Same note in two files",
        crate::gui::icons::WARN
    ));
    assert!(
        shell.bounds("Guard").is_some(),
        "the note keeps its stat block: {:?}",
        crate::gui::kit::probe::names()
    );
    shell.capture("world-note-duplicated__03-duplicate-world-files-and-repair__1200x800");
    let before_count = shell.read(|root| root.app.lookup.index().unwrap().duplicate_world.len());
    assert!(before_count > 0, "the index finds the duplicate");
    shell.press("ctrl-k");
    shell.type_text("Rook Ambry");
    shell.settle();
    shell.capture("world-note-duplicated__04-duplicate-world-files-in-lookup__1200x800");
    shell.press("escape");
    shell.update(|root, _, _| root.app.lookup.close());
    shell.click("Make Rook Ambry copy.md a separate note");
    until(
        &mut shell,
        "the repair gives the copy its own identity",
        |root| {
            root.app
                .lookup
                .index()
                .is_some_and(|index| index.duplicate_world.is_empty())
        },
    );
    assert_eq!(
        std::fs::read_to_string(&original).unwrap(),
        before,
        "the original keeps its bytes"
    );
    shell.capture("world-note-duplicated__05-world-copy-repaired__1200x800");
    let undo = "Undo separate Rook Ambry copy.md";
    assert!(
        shell.bounds(undo).is_some(),
        "{:?}",
        crate::gui::kit::probe::names()
    );
    shell.click(undo);
    assert_eq!(
        std::fs::read_to_string(&copy).unwrap(),
        before,
        "Undo gives the copy its identity back"
    );
    reload(&mut shell, None);
    shell.capture("world-note-duplicated__06-world-copy-repair-undone__1200x800");
}

/// The main states at the smallest window.
#[test]
fn world_at_960() {
    let mut shell = world("world-places", small_size());
    let rook = entry(
        &mut shell,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        "  in: '[[Ashfall Keep]]'\n",
    );
    entry(
        &mut shell,
        "Locations/Ashfall Keep.md",
        "Ashfall Keep",
        "location",
        "",
    );
    reload(&mut shell, Some(rook));
    shell.update(|root, _, _| root.app.world_page.press_note(rook));
    shell.capture("world-places__01-npc-place__960x640");
    shell.click("Edit Rook Ambry");
    shell.capture("world-edit-round-trip__04-world-edit-without-run__960x640");
    shell.press("escape");
    shell.click("New ▾");
    shell.click("Generate NPC");
    shell.capture("world-held-npc__01-held-npc-card__960x640");
}

/// A malformed value is flagged on its line, below the file line and above the values, and
/// reading never changes the file (scenario `file-formats`).
#[test]
fn file_formats_flagged_value() {
    let mut shell = Shell::with_size("file-formats", Seed::Formats, default_size());
    shell.update(|root, _, cx| root.show_page(Page::World, cx));
    until(&mut shell, "the notes load", |root| {
        root.app.lookup.index().is_some()
    });
    let path = root(&mut shell).join("NPCs/Malformed.md");
    let before = std::fs::read(&path).unwrap();
    let id = shell.read(|root| {
        let index = root.app.lookup.index().unwrap();
        index
            .items
            .iter()
            .find(|item| item.entry.name == "Malformed")
            .map(|item| item.entry.id)
            .expect("the fixture's Malformed note")
    });
    shell.update(|root, _, _| root.app.world_page.press_note(id));
    shell.settle();
    let file = shell.bounds("Show file").expect("the file line");
    let status = shell.bounds("Change Status").expect("the Status row");
    assert!(file.bottom() < status.top());
    shell.capture("file-formats__01-flagged-malformed-value__1200x800");
    assert_eq!(
        std::fs::read(&path).unwrap(),
        before,
        "reading never writes"
    );
}

/// An NPC is read as every entry is (`view::show_with_related`): its dice roll for it, and a
/// note link in its words opens the note it names at the heading it names.
#[test]
fn world_entry_rolls_and_links_to_a_heading() {
    let mut shell = world("world-entry-links", default_size());
    let check = splat::content::dice_text::RollLink::d20(
        splat::content::dice_text::D20::Check,
        6,
        "Strength",
    );
    let warden = splat::character::stable_id("NPCs/Gate Warden.md");
    let file = root(&mut shell).join("NPCs/Gate Warden.md");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(
        &file,
        splat::content::formats::fixture(
            "World.md",
            &format!(
                "splat_id: {warden}\nsplat_kind: world\nsplat_name: Gate Warden\nsplat_world:\n  type: npc\n"
            ),
            &format!(
                "\n# Gate Warden\n\n{} to lift the gate.\n\n[[Night Watch#Later that night|Night roster]] is hers to keep.\n\n{}",
                check.markdown("+6"),
                "She walks the wall at night.\n\n".repeat(60)
            ),
        )
        .unwrap(),
    )
    .unwrap();
    std::fs::write(
        root(&mut shell).join("Night Watch.md"),
        format!(
            "# Night Watch\n\n{}## Later that night\n\nThe end.\n",
            "Words on a line.\n\n".repeat(60)
        ),
    )
    .unwrap();
    reload(&mut shell, Some(warden));
    shell.update(|root, _, _| root.app.world_page.press_note(warden));
    shell.settle();
    // The dice number rolls for this NPC.
    let paragraph = |start: &str| {
        crate::gui::kit::probe::names()
            .into_iter()
            .find(|name| name.starts_with(start))
            .unwrap_or_else(|| panic!("no linked paragraph starts with {start:?}"))
    };
    let dice = shell.bounds(&paragraph("+6")).expect("the dice line");
    shell.click_at(gpui::point(dice.left() + px(8.0), dice.top() + px(10.0)));
    assert!(
        shell
            .read(|root| crate::rolls::plain::preview_words(&root.ctx, "Gate Warden", &[&check]))
            .is_some(),
        "the dice link rolled for Gate Warden"
    );
    shell.capture("world-entry-links__01-npc-with-dice-and-link__1200x800");
    // The note link opens Night Watch in World, scrolled to its heading.
    let link = shell
        .bounds(&paragraph("Night roster"))
        .expect("the link line");
    shell.click_at(gpui::point(link.left() + px(8.0), link.top() + px(10.0)));
    let night = shell.read(|root| {
        root.app
            .lookup
            .index()
            .and_then(|index| {
                index
                    .items
                    .iter()
                    .find(|item| item.entry.name == "Night Watch")
                    .map(|item| item.entry.id)
            })
            .expect("the Night Watch note")
    });
    assert_eq!(
        shell.read(|root| root.app.world_page.selected()),
        Some(night),
        "the link opened Night Watch in World"
    );
    shell.settle();
    let offset = |shell: &mut Shell, id: Uuid| {
        shell.update(|root, _, _| root.pages.world.jump.scroll(id).offset().y)
    };
    let night_at = offset(&mut shell, night);
    assert!(
        night_at < px(-200.0),
        "the reader scrolled to Later that night: {night_at:?}"
    );
    shell.capture("world-entry-links__02-note-at-its-heading__1200x800");
    // Each note keeps its own place: another long note opens at its top, and the first one
    // comes back where the DM left it.
    shell.update(|root, _, _| root.app.world_page.press_note(warden));
    shell.settle();
    assert_eq!(
        offset(&mut shell, warden),
        px(0.0),
        "Gate Warden opens at its top"
    );
    shell.update(|root, _, _| root.app.world_page.press_note(night));
    shell.settle();
    assert_eq!(
        offset(&mut shell, night),
        night_at,
        "Night Watch keeps its place"
    );
}

/// The held NPC's card names the campaign's table it used; the table's link opens that note,
/// and a bad table's `Open Names.md` does too (scenario `generator-tables`: the card leaves
/// a note link, which the frame logic opens in Lookup by its path).
#[test]
fn world_card_table_links_open_the_note() {
    let mut shell = world("generator-tables", default_size());
    let names = root(&mut shell).join("Names.md");
    std::fs::write(
        &names,
        "# Names\n\n<!-- splat:rolls key=npc-name dice=1d2 -->\n\n| d2 | Name |\n|---|---|\n| 1-2 | Tally |\n",
    )
    .unwrap();
    reload(&mut shell, None);
    shell.click("New ▾");
    shell.click("Generate NPC");
    assert_eq!(
        shell.read(|root| root.app.world_page.held_npc().map(str::to_owned)),
        Some("Tally".to_owned())
    );
    shell.capture("generator-tables__01-generator-campaign-table__1200x800");
    let showing = |shell: &mut Shell| {
        shell.read(|root| match root.app.lookup.showing() {
            crate::lookup::panel_view::Showing::Entry(at) => root
                .app
                .lookup
                .index()
                .map(|index| index.items[at].entry.name.clone()),
            _ => None,
        })
    };
    shell.click("Names.md");
    assert!(shell.read(|root| root.app.lookup.open), "Lookup opens");
    assert_eq!(
        showing(&mut shell).as_deref(),
        Some("Names"),
        "the table's note"
    );
    assert_eq!(
        shell.read(|root| root.app.lookup.query().to_owned()),
        "",
        "the note opens by its path, not by a search"
    );
    shell.press("escape");
    shell.click("Discard");
    std::fs::write(
        &names,
        "# Names\n\n<!-- splat:rolls key=npc-name dice=1d6 -->\n\n| d6 | Name |\n|---|---|\n| 1 | Tally |\n",
    )
    .unwrap();
    reload(&mut shell, None);
    shell.click("New ▾");
    shell.click("Generate NPC");
    shell.capture("generator-tables__02-generator-bad-table__1200x800");
    shell.click("Open Names.md");
    assert!(shell.read(|root| root.app.lookup.open), "Lookup opens");
    assert_eq!(showing(&mut shell).as_deref(), Some("Names"));
}

/// In Lookup, an open Log line keeps Escape while it has the keyboard; with the note's edit form
/// open in Lookup, the clean form's Cancel closes it; and World's footer does not repeat the
/// form's reason while Lookup's form is on screen, but says it once the form is not drawn.
#[test]
fn world_log_line_and_the_edit_form_in_lookup() {
    let mut shell = world("world-log-line", default_size());
    let faction = entry(
        &mut shell,
        "Factions/Garrow Watch.md",
        "Garrow Watch",
        "faction",
        "",
    );
    reload(&mut shell, Some(faction));
    let campaign = shell.read(|root| root.app.campaigns.active_campaign().unwrap());
    let log_open = |shell: &mut Shell| shell.bounds("Log line for Garrow Watch").is_some();
    // Escape in Lookup's Log field ends the line, not Lookup.
    look_up(&mut shell, "Garrow Watch");
    shell.click("Add to Log");
    assert!(log_open(&mut shell), "the Log field opens");
    shell.press("escape");
    assert!(
        shell.read(|root| root.app.lookup.open),
        "Escape in the field keeps Lookup open"
    );
    assert!(!log_open(&mut shell), "Escape closes the Log field");
    shell.press("escape");
    let takes = shell.read(|root| root.app.lookup.reader.takes_escape());
    assert!(
        !shell.read(|root| root.app.lookup.open),
        "with no field, Escape closes Lookup: takes {takes}"
    );
    // The Log line in World, the edit form in Lookup.
    shell.update(|root, _, _| root.app.world_page.press_note(faction));
    shell.click("Add to Log");
    look_up(&mut shell, "Garrow Watch");
    shell.click("Edit Garrow Watch");
    assert!(shell.read(|root| root.app.lookup.reader.editing(faction, &campaign)));
    shell.settle();
    assert_eq!(
        shell.read(|root| root.app.world_page.reader.form_shown_elsewhere),
        Some(faction),
        "Lookup's save row is on screen: World's footer does not repeat it"
    );
    // A clean form's Cancel closes it, whatever waits for it.
    shell.click("Cancel");
    assert!(!shell.read(|root| root.app.lookup.reader.editing(faction, &campaign)));
    shell.click("Edit Garrow Watch");
    shell.update(|root, _, _| root.app.lookup.close());
    shell.settle();
    assert_eq!(
        shell.read(|root| root.app.world_page.reader.form_shown_elsewhere),
        None,
        "Lookup is closed: its form is not drawn"
    );
}

/// An NPC's optional links stored null, empty or blank: the reader hides an unset place and keeps
/// the Stat block row; the form says `not set` for both; an unrelated save keeps the raw values,
/// and its Undo the bytes (scenario `world-empty-links`).
#[test]
fn world_empty_links() {
    let mut shell = world("world-empty-links", default_size());
    for (step, (label, value)) in [("null", "null"), ("empty", "''"), ("whitespace", "'   '")]
        .into_iter()
        .enumerate()
    {
        let name = format!("Optional {label}");
        let path = format!("NPCs/{name}.md");
        let id = flat_entry(
            &mut shell,
            &path,
            &name,
            &format!("splat_type: npc\nsplat_in: {value}\nsplat_stat_block: {value}"),
            "Keeps the gate.\n",
        );
        let file = root(&mut shell).join(&path);
        let before = std::fs::read(&file).unwrap();
        reload(&mut shell, Some(id));
        click_starting(&mut shell, &format!("{name} · "));
        assert!(shell.bounds("Change Stat block").is_some());
        assert!(
            shell.bounds("Change Place").is_none(),
            "an unset place stays hidden"
        );
        shell.click(&format!("Edit {name}"));
        let order = [
            "Change Stat block",
            "Change Place",
            "Change Part of",
            "Status",
            "Text",
        ]
        .map(|name| shell.bounds(name).map(|bounds| bounds.top()));
        assert!(
            order.iter().all(Option::is_some) && order.windows(2).all(|pair| pair[0] < pair[1]),
            "Edit keeps the Read order: {order:?}"
        );
        shell.click("Status");
        shell.type_text("ready");
        shell.capture(&format!(
            "world-empty-links__{:02}-{label}-links-in-edit__1200x800",
            step + 1
        ));
        shell.click("Save changes");
        let saved = std::fs::read_to_string(&file).unwrap();
        assert!(
            saved.contains(&format!("splat_in: {value}\n"))
                && saved.contains(&format!("splat_stat_block: {value}\n")),
            "an unrelated save keeps both raw values: {saved}"
        );
        reload(&mut shell, None);
        shell.click(&format!("Undo change to {name}"));
        assert_eq!(
            std::fs::read(&file).unwrap(),
            before,
            "Undo restores the bytes"
        );
    }
}

/// A place link with two matching files, chosen in World and in Lookup; two link faults on one
/// row; a note moved with its place keeps it, and a Change writes the moved file (scenario
/// `world-note-moved`).
#[test]
fn world_note_moved() {
    let mut shell = world("world-note-moved", default_size());
    let folder = root(&mut shell);
    std::fs::create_dir_all(folder.join("Locations")).unwrap();
    std::fs::create_dir_all(folder.join("Other")).unwrap();
    std::fs::write(
        folder.join("Locations/Watch House.md"),
        "# Watch House\n\nThe original watch house.\n",
    )
    .unwrap();
    std::fs::copy(
        folder.join("Locations/Watch House.md"),
        folder.join("Other/Watch House.md"),
    )
    .unwrap();
    let npc = entry(
        &mut shell,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        "  in: '[[Watch House]]'\n",
    );
    entry(
        &mut shell,
        "Locations/North Gate.md",
        "North Gate",
        "location",
        "",
    );
    reload(&mut shell, Some(npc));
    click_starting(&mut shell, "Rook Ambry · ");
    // The place link's warning, icon and words, stays on one line (as in egui).
    let one_line = |shell: &mut Shell, step: &str| {
        let link = shell
            .bounds("Watch House · 2 matches")
            .expect("the place link");
        assert!(
            link.size.height < crate::gui::theme::Type::Body.line() * 2.0,
            "{step}: the place link is on one line: {link:?}"
        );
    };
    one_line(&mut shell, "01");
    shell.click("Watch House · 2 matches");
    for path in ["Locations/Watch House.md", "Other/Watch House.md"] {
        assert!(shell.bounds(&format!("Always use {path}")).is_some());
    }
    assert!(shell.bounds("1 note needs fixing").is_some());
    let chooser_in_column = |shell: &mut Shell| {
        let change = shell.bounds("Change Place").expect("Change Place");
        let places: Vec<_> = ["Locations/Watch House.md", "Other/Watch House.md"]
            .iter()
            .map(|path| {
                let link = shell.bounds(path).expect("the file link");
                let action = shell
                    .bounds(&format!("Always use {path}"))
                    .expect("its action");
                (link, action)
            })
            .collect();
        let fits = places.iter().all(|(link, action)| {
            (action.left() - change.left()).abs() <= px(1.0) && action.left() > link.right()
        });
        (
            fits,
            format!("Change {change:?}, (file, action) {places:?}"),
        )
    };
    let (fits, places) = chooser_in_column(&mut shell);
    assert!(
        fits,
        "each action stands in the Change column, clear of its file: {places}"
    );
    shell.capture("world-note-moved__01-duplicate-place-chooser-in-world__1200x800");
    shell.click("Always use Locations/Watch House.md");
    assert!(shell.bounds("Undo change to Place").is_some());
    reload(&mut shell, None);
    shell.capture("world-note-moved__02-place-choice-kept-with-undo__1200x800");
    shell.click("Undo change to Place");
    reload(&mut shell, None);
    // The header count's menu opens the note that needs fixing.
    click_starting(&mut shell, "North Gate · ");
    assert!(shell.bounds("Edit North Gate").is_some());
    shell.click("1 note needs fixing");
    shell.click("Rook Ambry");
    assert_eq!(shell.read(|root| root.app.world_page.selected()), Some(npc));
    let npc_path = folder.join("NPCs/Rook Ambry.md");
    let original = std::fs::read_to_string(&npc_path).unwrap();
    std::fs::write(
        &npc_path,
        format!(
            "---\nsplat_schema: 2\nsplat_id: {npc}\nsplat_kind: world\nsplat_name: Rook Ambry\nsplat_type: npc\nsplat_in: '[[Watch House]]'\nsplat_part_of: ['[[Lost Guild]]']\n---\n\n# Rook Ambry\n\nA note for tonight.\n"
        ),
    )
    .unwrap();
    reload(&mut shell, None);
    click_starting(&mut shell, "Rook Ambry · ");
    let names = crate::gui::kit::probe::names();
    assert!(
        names
            .iter()
            .any(|name| name.starts_with("Rook Ambry · ") && name.contains("2 links need fixing")),
        "the row names both link faults: {names:?}"
    );
    one_line(&mut shell, "03");
    shell.capture("world-note-moved__03-mixed-link-problems-need-fixing__1200x800");
    std::fs::write(&npc_path, &original).unwrap();
    reload(&mut shell, None);
    shell.click("Watch House · 2 matches");
    shell.click("Locations/Watch House.md");
    look_up(&mut shell, "Rook Ambry");
    shell.click("Watch House · 2 matches");
    assert!(
        shell
            .bounds("Always use Locations/Watch House.md")
            .is_some()
    );
    // Lookup's panel is narrower: each action stays in the panel, clear of its file.
    let edge = shell.bounds("Close Lookup").expect("Close Lookup").right();
    for path in ["Locations/Watch House.md", "Other/Watch House.md"] {
        let link = shell.bounds(path).expect("the file link");
        let action = shell
            .bounds(&format!("Always use {path}"))
            .expect("its action");
        let clear = action.left() > link.right() || action.top() >= link.bottom();
        assert!(
            clear && action.right() <= edge,
            "in Lookup, {path}'s action is clear of it and in the panel: {link:?} {action:?} edge {edge:?}"
        );
    }
    one_line(&mut shell, "04");
    shell.capture("world-note-moved__04-duplicate-place-chooser-in-lookup__1200x800");
    // As in egui: the DM follows a file from Lookup's chooser (a step on Lookup's Back trail),
    // then closes Lookup.
    shell.click("Locations/Watch House.md");
    assert_eq!(lookup_shows(&mut shell).as_deref(), Some("Watch House"));
    shell.update(|root, _, _| root.app.lookup.close());
    // World's chooser is open on Rook Ambry when Obsidian takes the other copy away.
    click_starting(&mut shell, "Rook Ambry · ");
    shell.click("Watch House · 2 matches");
    assert!(shell.bounds("Always use Other/Watch House.md").is_some());
    // Obsidian moves the place and the NPC; the other copy goes.
    std::fs::remove_file(folder.join("Other/Watch House.md")).unwrap();
    std::fs::create_dir_all(folder.join("Locations/Moved")).unwrap();
    std::fs::rename(
        folder.join("Locations/Watch House.md"),
        folder.join("Locations/Moved/Watch House.md"),
    )
    .unwrap();
    std::fs::rename(&npc_path, folder.join("Locations/Moved/Rook Ambry.md")).unwrap();
    reload(&mut shell, Some(npc));
    click_starting(&mut shell, "Rook Ambry · ");
    assert!(shell.bounds("Watch House").is_some(), "the place is kept");
    // One match left: no chooser, and no words that ask for a choice, in either reader.
    let chooser_open = |shell: &mut Shell, host: crate::gui::pages::world::reader::Host| {
        shell.update(|root, _, _| {
            let index = root.app.lookup.index().expect("the notes");
            let item = &index.items[index.position(npc).expect("Rook Ambry")];
            host.reader(root).place_chooser_open(item, &index)
        })
    };
    assert!(
        !chooser_open(&mut shell, crate::gui::pages::world::reader::Host::World),
        "World's chooser ends with the duplicate"
    );
    shell.capture("world-note-moved__05-moved-note-keeps-place-in-world__1200x800");
    look_up(&mut shell, "Rook Ambry");
    assert!(
        !chooser_open(&mut shell, crate::gui::pages::world::reader::Host::Lookup),
        "Lookup's chooser ends with the duplicate"
    );
    assert!(
        shell.bounds("Back").is_some(),
        "Lookup can go Back to Watch House: {:?}",
        crate::gui::kit::probe::names()
    );
    shell.capture("world-note-moved__06-moved-note-keeps-place-in-lookup__1200x800");
    shell.update(|root, _, _| root.app.lookup.close());
    let moved = folder.join("Locations/Moved/Rook Ambry.md");
    let external = format!(
        "{}\n## Patrol after the move\n\nRook keeps the east gate key in the watch room. The relief patrol leaves at midnight and checks the bridge before it returns to the north gate.\n\nKeep this note from Obsidian when the place changes.\n",
        std::fs::read_to_string(&moved).unwrap()
    );
    std::fs::write(&moved, &external).unwrap();
    reload(&mut shell, None);
    shell.click("Change Place");
    shell.type_text("North Gate");
    shell.click("North Gate");
    let written = std::fs::read_to_string(&moved).unwrap();
    assert!(
        written.contains("Keep this note from Obsidian"),
        "{written}"
    );
    assert!(!npc_path.exists(), "the Change writes the moved file");
    reload(&mut shell, None);
    shell.capture("world-note-moved__07-change-writes-moved-npc__1200x800");
}

/// An NPC whose place and stat block are deleted outside Splat: each says `Not found`, in World
/// and in Lookup (scenario `world-note-deleted`).
#[test]
fn world_note_deleted() {
    let mut shell = world("world-note-deleted", default_size());
    entry(
        &mut shell,
        "Locations/Watch House.md",
        "Watch House",
        "location",
        "",
    );
    let block_text = crate::library::note_text("Watchkeeper Guard", "creature", None);
    let block = splat::content::Entry::parse("Watchkeeper Guard.md", &block_text).unwrap();
    std::fs::write(root(&mut shell).join("Watchkeeper Guard.md"), block_text).unwrap();
    let npc = entry(
        &mut shell,
        "NPCs/Watchkeeper.md",
        "Watchkeeper",
        "npc",
        &format!(
            "  in: '[[Watch House]]'\n  creature: {{id: {}, link: '[[Watchkeeper Guard]]'}}\n",
            block.id
        ),
    );
    reload(&mut shell, Some(npc));
    click_starting(&mut shell, "Watchkeeper · ");
    assert!(shell.bounds("Watch House").is_some());
    shell.capture("world-note-deleted__01-npc-before-place-deleted__1200x800");
    std::fs::remove_file(root(&mut shell).join("Locations/Watch House.md")).unwrap();
    reload(&mut shell, None);
    shell.capture("world-note-deleted__02-npc-place-deleted-in-world__1200x800");
    look_up(&mut shell, "Watchkeeper");
    shell.capture("world-note-deleted__03-npc-place-deleted-in-lookup__1200x800");
    shell.update(|root, _, _| root.app.lookup.close());
    std::fs::remove_file(root(&mut shell).join("Watchkeeper Guard.md")).unwrap();
    reload(&mut shell, None);
    shell.click("Search the world");
    shell.type_text("Watchkeeper");
    shell.settle();
    assert!(shell.bounds("Clear").is_some(), "the search offers Clear");
    shell.capture("world-note-deleted__04-npc-stat-block-deleted-in-world__1200x800");
    look_up(&mut shell, "Watchkeeper");
    // Lookup opened again on its last words keeps the result list: the one match, with its
    // problem count.
    let names = crate::gui::kit::probe::names();
    assert!(
        shell.read(|root| root.app.lookup.results_shown())
            && names
                .iter()
                .any(|name| name.starts_with("Watchkeeper") && name.contains("2 broken links")),
        "the result row names its problems: {names:?}"
    );
    shell.capture("world-note-deleted__05-npc-stat-block-deleted-in-lookup__1200x800");
    // Clear empties World's search and goes.
    shell.update(|root, _, _| root.app.lookup.close());
    shell.settle();
    shell.click("Clear");
    assert_eq!(
        shell.read(|root| root.app.world_page.search().to_owned()),
        ""
    );
    assert!(shell.bounds("Clear").is_none());
}

/// Part of keeps what Splat did not read and what Obsidian changed: a missing group on one line,
/// a pick refused over an unread list, a disk conflict that names each change, an Overwrite the
/// note already held, and an Overwrite that keeps the unread groups (scenario
/// `world-part-of-safety`).
#[test]
fn world_part_of_safety() {
    let mut shell = world("world-part-of-safety", default_size());
    flat_entry(
        &mut shell,
        "Factions/Added Guild.md",
        "Added Guild",
        "splat_type: faction",
        "A new group.\n",
    );
    flat_entry(
        &mut shell,
        "Factions/Outside Guild.md",
        "Outside Guild",
        "splat_type: faction",
        "A group added in Obsidian.\n",
    );
    let path = root(&mut shell).join("NPCs/Safe Rook.md");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let id = splat::character::stable_id("NPCs/Safe Rook.md");
    let readable = format!(
        "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: world\nsplat_name: Safe Rook\nsplat_type: npc\nsplat_part_of: # Keep the stored groups.\n- \"[[Zarel|the Archdevil]]\" # Keep the alias.\nsplat_status: alive\n---\n\n# Safe Rook\n\nKeeps the gate.\n"
    );
    let unread_groups = format!(
        "splat_part_of: # Keep the stored groups.\n- '[[Zarel|the Archdevil]]'\n- {{id: {}, link: '[[Lost Guild|the lost]]'}}\n",
        splat::character::stable_id("Factions/Lost Guild.md")
    );
    let unread = readable.replace(
        "splat_part_of: # Keep the stored groups.\n- \"[[Zarel|the Archdevil]]\" # Keep the alias.\n",
        &unread_groups,
    );
    assert_ne!(unread, readable, "fixture: an unread group list");
    let write = |text: &str| std::fs::write(&path, text).unwrap();
    write(&readable);
    reload(&mut shell, Some(id));
    click_starting(&mut shell, "Safe Rook · ");
    assert!(shell.bounds("Change Part of").is_some());
    shell.capture("world-part-of-safety__01-missing-group-on-one-line__1200x800");
    // A pick over a list that turned unread on disk writes nothing.
    shell.click("Change Part of");
    shell.type_text("Added Guild");
    write(&unread);
    shell.click("Added Guild");
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        unread,
        "nothing written"
    );
    reload(&mut shell, None);
    shell.capture("world-part-of-safety__02-read-picker-unread-disk-kept__1200x800");
    // The draft adds a group while Obsidian adds another: the choice names each change.
    write(&readable);
    reload(&mut shell, None);
    let zarel = "- \"[[Zarel|the Archdevil]]\" # Keep the alias.\n";
    let outside = readable.replace(
        zarel,
        &format!("{zarel}- '[[Outside Guild]]' # Added in Obsidian.\n"),
    );
    let pick_added = |shell: &mut Shell| {
        shell.click("Edit Safe Rook");
        shell.click("Change Part of");
        shell.type_text("Added Guild");
        shell.click("Added Guild");
    };
    pick_added(&mut shell);
    write(&outside);
    shell.click("Save changes");
    assert!(shell.bounds("Overwrite with my draft").is_some());
    shell.capture("world-part-of-safety__03-readable-group-conflict-names-each-change__1200x800");
    shell.click("Overwrite with my draft");
    let saved = std::fs::read_to_string(&path).unwrap();
    assert!(
        saved.contains("Added Guild") && saved.contains("Outside Guild"),
        "Overwrite adds the draft's group and keeps the outside one: {saved}"
    );
    // The draft adds a group that Obsidian added in another spelling: the note has it already.
    write(&readable);
    reload(&mut shell, None);
    let other = readable.replace(
        zarel,
        &format!("{zarel}- \"[[Factions/Added Guild|Guild]]\" # Added in Obsidian.\n"),
    );
    pick_added(&mut shell);
    write(&other);
    shell.click("Save changes");
    shell.click("Overwrite with my draft");
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        other,
        "the note already held the change"
    );
    // The save reads the note again: Part of shows the group the note holds (as egui waited
    // for it), and the form is closed with nothing to undo.
    until(&mut shell, "Part of shows Added Guild", |_| {
        crate::gui::kit::probe::names()
            .iter()
            .any(|name| name == "Added Guild")
    });
    assert!(
        shell.bounds("Save changes").is_none()
            && shell.bounds("Overwrite with my draft").is_none()
            && shell.bounds("Undo change to Safe Rook").is_none(),
        "{:?}",
        crate::gui::kit::probe::names()
    );
    shell.capture("world-part-of-safety__04-overwrite-already-in-note__1200x800");
    // A changed draft over a list that turned unread on disk.
    write(&readable);
    reload(&mut shell, None);
    shell.click("Edit Safe Rook");
    shell.click("Status");
    shell.press("ctrl-a");
    shell.type_text("ready");
    shell.click("Change Part of");
    shell.type_text("Added Guild");
    shell.click("Added Guild");
    write(&unread);
    shell.click("Save changes");
    assert!(shell.bounds("Reload and lose my draft").is_some());
    // The save reads the note again, so its list row names the unread list, not the old
    // missing group; the choice still waits.
    until(&mut shell, "the row says 1 value not read", |_| {
        crate::gui::kit::probe::names().iter().any(|name| {
            name.starts_with("Safe Rook · ")
                && name.contains("1 value not read")
                && !name.contains("Group not found")
        })
    });
    assert!(shell.bounds("Reload and lose my draft").is_some());
    shell.capture("world-part-of-safety__05-group-draft-disk-conflict__1200x800");
    shell.click("Overwrite with my draft");
    let saved = std::fs::read_to_string(&path).unwrap();
    assert!(
        saved.contains(&unread_groups),
        "Overwrite keeps the whole unread Part of: {saved}"
    );
    assert!(saved.contains("splat_status: ready"), "{saved}");
    reload(&mut shell, None);
    shell.capture("world-part-of-safety__06-overwrite-keeps-unread-disk-groups__1200x800");
    shell.click("Undo change to Safe Rook");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), unread);
    reload(&mut shell, None);
    shell.click("Edit Safe Rook");
    assert!(shell.bounds("Edit in Obsidian").is_some());
    shell.capture("world-part-of-safety__07-unread-groups-edit-repair-link__1200x800");
    shell.click("Cancel");
}
