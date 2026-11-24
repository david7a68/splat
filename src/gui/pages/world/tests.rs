//! The World page's screenshots and interaction tests. Captures pair with the egui references by
//! name (`<egui-scenario>__<step>__<w>x<h>`), in `artifacts/screenshots/gpui/`.
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;
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
    for _ in 0..200 {
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

/// The World page, with its notes loaded.
fn world(name: &str, window: gpui::Size<gpui::Pixels>) -> Shell {
    let mut shell = Shell::with_size(name, Seed::World, window);
    shell.update(|root, _, cx| root.show_page(Page::World, cx));
    for _ in 0..200 {
        if shell.read(|root| root.app.lookup.index().is_some()) {
            break;
        }
        shell.wait(Duration::from_millis(50));
    }
    shell
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
    assert_eq!(shell.read(|root| root.app.world_page.new_kind()), Some("npc"));
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
    assert!(shell.bounds("Change Stat block").is_some());
    assert!(shell.bounds("Change Place").is_some());
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
    assert_eq!(std::fs::read(&path).unwrap(), original, "Undo restores the bytes");
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
    assert_eq!(shell.read(|root| root.app.world_page.view()), View::Questions);
    shell.capture("world-facts__13-open-questions__1200x800");
}
