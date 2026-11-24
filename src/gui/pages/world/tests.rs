//! The World page's screenshots and interaction tests. Captures pair with the egui references by
//! name (`<egui-scenario>__<step>__<w>x<h>`), in `artifacts/screenshots/gpui/`.
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

/// A World entry of `kind` at `path`, as the egui scenarios write one (`world_build::entry`).
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

/// The World page, with its notes loaded.
fn world(name: &str, window: gpui::Size<gpui::Pixels>) -> Shell {
    let mut shell = Shell::with_size(name, Seed::World, window);
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

/// The list, New's menu, an NPC's form and the note it makes (egui scenario `world`).
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

/// An NPC's place and stat block, a change with its Undo (egui scenario `world-places`).
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

/// The checks views open from their rows, and a press on a check row is the view.
#[test]
fn world_check_rows_open_their_views() {
    let mut shell = world("world-facts", default_size());
    std::fs::write(
        root(&mut shell).join("Questions.md"),
        "# Questions\n\n- [ ] Who rules the Keep?\n",
    )
    .unwrap();
    reload(&mut shell, None);
    shell.click("Open questions · 1 question");
    assert_eq!(
        shell.read(|root| root.app.world_page.view()),
        View::Questions
    );
    shell.capture("world-facts__13-open-questions__1200x800");
}

/// The words of an Undo, as the footer's line names it.
fn undo_words(verb: crate::design::outcome::Verb, object: &str) -> String {
    crate::design::outcome::UndoStep::of(verb, object).label()
}

/// Generate NPC holds a card; another note keeps its row above the reader; Discard has an Undo
/// that brings the same NPC back (egui scenario `world-held-npc`).
#[test]
fn world_held_npc_row_discard_and_undo() {
    let mut shell = world("world-held-npc", default_size());
    shell.click("New ▾");
    shell.click("Generate NPC");
    let name = shell
        .read(|root| root.app.world_page.held_npc().map(str::to_owned))
        .expect("a held NPC");
    assert!(shell.bounds("Save NPC").is_some(), "the card shows");
    shell.capture("world-held-npc__01-held-npc-card__1200x800");
    shell.click("Gate Mechanics · ");
    let row = format!("{}: {name}", crate::world::UNSAVED_NPC);
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

/// Edit, a typed status, Ctrl+S: one write; a disk edit under a changed draft waits for the
/// DM's choice, and Reload takes the file (egui scenario `world-edit-round-trip`).
#[test]
fn world_edit_round_trip() {
    let mut shell = world("world-edit-round-trip", default_size());
    let faction = entry(
        &mut shell,
        "Factions/Ashfall Faction.md",
        "Ashfall Faction",
        "faction",
        "",
    );
    reload(&mut shell, Some(faction));
    shell.update(|root, _, _| root.app.world_page.press_note(faction));
    shell.click("Edit Ashfall Faction");
    let campaign = shell.read(|root| root.app.campaigns.active_campaign().unwrap());
    assert!(shell.read(|root| root.app.world_page.reader.editing(faction, &campaign)));
    shell.click("Status");
    shell.type_text("ready");
    shell.capture("world-edit-round-trip__04-world-edit-without-run__1200x800");
    let path = root(&mut shell).join("Factions/Ashfall Faction.md");
    shell.press("ctrl-s");
    assert!(
        std::fs::read_to_string(&path).unwrap().contains("ready"),
        "Ctrl+S saves the status"
    );
    reload(&mut shell, None);
    assert!(!shell.read(|root| root.app.world_page.reader.editing(faction, &campaign)));
    // A disk edit under a changed draft.
    shell.click("Edit Ashfall Faction");
    shell.click("Status");
    shell.type_text(" now");
    let outside = std::fs::read_to_string(&path)
        .unwrap()
        .replace("A note for tonight.", "Changed in Obsidian.");
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
    shell.capture("world-edit-round-trip__02-after-disk-edit__1200x800");
    shell.click("Reload and lose my draft");
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        outside,
        "Reload writes nothing"
    );
    assert!(shell.bounds("Reload and lose my draft").is_none());
}

/// A note that cannot be read: the header's count opens its repair, which saves the fixed text
/// and opens the note with `Note repaired.` (egui scenario `world-broken-note`).
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

/// Ticking an open question writes its box; `Undo tick` opens it again (egui scenario
/// `world-facts`).
#[test]
fn world_question_tick_and_undo() {
    let mut shell = world("world-facts-tick", default_size());
    let id = entry(&mut shell, "Factions/Guild.md", "Guild", "faction", "");
    let path = root(&mut shell).join("Factions/Guild.md");
    let text = std::fs::read_to_string(&path).unwrap() + "\n- [ ] Who rules the Keep?\n";
    std::fs::write(&path, &text).unwrap();
    reload(&mut shell, Some(id));
    shell.click("Open questions · 1 question");
    shell.click("Open: Who rules the Keep?");
    assert!(
        std::fs::read_to_string(&path)
            .unwrap()
            .contains("- [x] Who rules the Keep?")
    );
    reload(&mut shell, None);
    shell.capture("world-facts__14-question-ticked__1200x800");
    let undo = undo_words(crate::design::outcome::Verb::Tick, "“Who rules the Keep?”");
    let undo = shell
        .bounds(&undo)
        .map(|_| undo.clone())
        .unwrap_or_else(|| format!("{undo} in Guild"));
    shell.click(&undo);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), text, "Undo tick");
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
/// file's name with an Undo (egui scenario `world-note-renamed`).
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

/// Two files with one identity: each has its repair (egui scenario `world-note-duplicated`).
#[test]
fn world_note_duplicated_offers_each_repair() {
    let mut shell = world("world-note-duplicated", default_size());
    entry(&mut shell, "NPCs/Rook Ambry.md", "Rook Ambry", "npc", "");
    let root_path = root(&mut shell);
    std::fs::copy(
        root_path.join("NPCs/Rook Ambry.md"),
        root_path.join("NPCs/Rook Ambry copy.md"),
    )
    .unwrap();
    reload(&mut shell, None);
    shell.click(&format!(
        "Rook Ambry.md · {} Same note in two files",
        crate::gui::icons::WARN
    ));
    shell.capture("world-note-duplicated__03-duplicate-world-files-and-repair__1200x800");
    let before = shell.read(|root| root.app.lookup.index().unwrap().duplicate_world.len());
    assert!(before > 0, "the index finds the duplicate");
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
    shell.capture("world-note-duplicated__05-world-copy-repaired__1200x800");
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
/// reading never changes the file (egui scenario `file-formats`).
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
                "\n# Gate Warden\n\n{} to lift the gate.\n\n[[Night Watch#Later that night|Night roster]] is hers to keep.\n",
                check.markdown("+6")
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
            .read(|root| crate::rolls::plain::preview_words(
                root.bridge.ctx(),
                "Gate Warden",
                &[&check]
            ))
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
    let offset = shell.read(|root| root.pages.world.jump.scroll.offset().y);
    assert!(
        offset < px(-200.0),
        "the reader scrolled to Later that night: {offset:?}"
    );
    shell.capture("world-entry-links__02-note-at-its-heading__1200x800");
}

/// The held NPC's card names the campaign's table it used; the table's link opens that note,
/// and a bad table's `Open Names.md` does too (egui scenario `generator-tables`: the card leaves
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
