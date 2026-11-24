//! The Encounters page's screenshots and interaction tests. Captures are named
//! `<egui-scenario>__<step>__<w>x<h>` so they pair with `artifacts/screenshots/egui-reference/`.
use crate::Page;
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;
use gpui::{px, size};
use std::time::Duration;

/// The page after its campaign is read (the read runs on a thread of its own).
fn ready(shell: &mut Shell) {
    shell.update(|root, _, cx| root.show_page(Page::Encounters, cx));
    for _ in 0..400 {
        let reading =
            shell.read(|root| root.app.encounters.busy() || !root.app.encounters.loaded());
        if !reading {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
        shell.wait(Duration::from_millis(50));
    }
    shell.settle();
}

fn small(name: &str, seed: Seed) -> Shell {
    Shell::with_size(name, seed, size(px(960.0), px(640.0)))
}

/// The list: In progress, Ready to run, Groups and History, at both sizes.
#[test]
fn encounter_list() {
    let mut shell = Shell::new("encounter-build", Seed::Campaign);
    ready(&mut shell);
    assert!(
        shell.bounds("Start fight").is_some(),
        "a Ready row's Start fight"
    );
    assert!(shell.bounds("Resume").is_some(), "the paused run's Resume");
    shell.capture("encounter-build__01-encounter-list__1200x800");
    let mut shell = small("encounter-build", Seed::Campaign);
    ready(&mut shell);
    shell.capture("encounter-build__01-encounter-list__960x640");
}

/// History lists ended runs by session; Open reads one in place and Close closes it.
#[test]
fn history_opens_a_run_in_place() {
    let mut shell = Shell::new("encounter-history", Seed::History);
    ready(&mut shell);
    shell.capture("encounter-history__01-encounters-with-history__1200x800");
    let open = shell
        .read(|_| crate::gui::kit::probe::names())
        .into_iter()
        .find(|name| name.starts_with("id:history-open:"))
        .expect("an Open in History");
    shell.click(&open);
    assert!(shell.bounds("Final state").is_some() || shell.bounds("Continue fight").is_some());
    assert!(shell.read(|root| root.app.encounters.history_open()));
    shell.capture("encounter-history__02-history-open__1200x800");
    shell.click(&open);
    assert!(!shell.read(|root| root.app.encounters.history_open()));
}

/// The first target whose name starts with `prefix`, after a frame.
fn first_target(shell: &mut Shell, prefix: &str) -> String {
    shell.settle();
    crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with(prefix))
        .unwrap_or_else(|| panic!("no target starts with {prefix:?}"))
}

/// The roster's length in the open draft.
fn roster_len(shell: &mut Shell) -> usize {
    shell.read(|root| {
        root.app
            .encounters
            .roster_names()
            .map_or(0, |names| names.len())
    })
}

/// New encounter: name it, find a creature, add it, save with the button, and go back.
#[test]
fn a_new_encounter_is_built_and_saved() {
    let mut shell = Shell::new("encounter-build", Seed::Campaign);
    ready(&mut shell);
    shell.click("New encounter");
    assert!(shell.read(|root| root.app.encounters.editor_open()));
    shell.type_text("Ashfall Crossing");
    assert_eq!(
        shell.read(|root| root.app.encounters.draft_name()),
        Some("Ashfall Crossing".to_owned())
    );
    shell.capture("encounter-build__02-new-encounter__1200x800");
    shell.click("Find a creature or party member");
    shell.type_text("wolf");
    shell.capture("encounter-build__03-find-creature__1200x800");
    let before = roster_len(&mut shell);
    let add = first_target(&mut shell, "id:add:");
    shell.click(&add);
    assert_eq!(
        roster_len(&mut shell),
        before + 1,
        "Add puts the creature on the roster"
    );
    // Enter adds the best match as typed, and the search keeps the keyboard.
    shell.click("Find a creature or party member");
    shell.press("enter");
    assert_eq!(
        roster_len(&mut shell),
        before + 2,
        "Enter adds the best match"
    );
    // A new encounter starts with the party, so Add participants says so.
    let names = shell.read(|root| root.app.encounters.roster_names().unwrap_or_default());
    assert!(
        names.iter().any(|name| name == "Brannoch Vale"),
        "{names:?}"
    );
    assert!(
        shell.bounds("Add party").is_none(),
        "no Add party while the party is in"
    );
    shell.capture("encounter-build__04-creatures-added__1200x800");
    assert!(shell.read(|root| root.app.encounters.encounter_dirty()));
    shell.click("Save encounter");
    assert!(
        !shell.read(|root| root.app.encounters.encounter_dirty()),
        "Save writes the draft"
    );
    shell.capture("encounter-build__05-saved__1200x800");
    shell.click("id:encounter-back");
    assert!(!shell.read(|root| root.app.encounters.editor_open()));
    assert!(
        shell.bounds("Ashfall Crossing").is_some()
            || shell.read(|root| {
                root.app
                    .encounters
                    .saved_names()
                    .iter()
                    .any(|name| name == "Ashfall Crossing")
            })
    );
}

/// A roster row's menu moves, duplicates and removes it; Ctrl+S saves; Discard brings the
/// saved encounter back and its Undo the draft.
#[test]
fn the_roster_menu_ctrl_s_and_discard_with_undo() {
    let mut shell = Shell::new("encounter-build", Seed::Campaign);
    ready(&mut shell);
    let edit = first_target(&mut shell, "id:edit:");
    shell.click(&edit);
    assert!(shell.read(|root| root.app.encounters.editor_open()));
    let names = shell.read(|root| root.app.encounters.roster_names().unwrap_or_default());
    assert!(names.len() >= 2, "the saved roster has rows: {names:?}");
    // Move the first row down.
    let menu = first_target(&mut shell, "id:roster-menu:");
    shell.click(&menu);
    shell.click("Move down");
    let moved = shell.read(|root| root.app.encounters.roster_names().unwrap_or_default());
    assert_eq!(moved[1], names[0], "Move down swaps the first two rows");
    assert_eq!(moved[0], names[1]);
    // Duplicate, then Remove.
    let menu = first_target(&mut shell, "id:roster-menu:");
    shell.click(&menu);
    shell.click("Duplicate");
    assert_eq!(roster_len(&mut shell), names.len() + 1);
    let menu = first_target(&mut shell, "id:roster-menu:");
    shell.click(&menu);
    shell.click("Remove");
    assert_eq!(roster_len(&mut shell), names.len());
    shell.capture("encounter-build__06-roster-changed__1200x800");
    // Discard: the saved roster comes back, with an Undo that brings the draft back.
    shell.click("Discard");
    let back = shell.read(|root| root.app.encounters.roster_names().unwrap_or_default());
    assert_eq!(back, names, "Discard brings back the saved encounter");
    let undo = first_target(&mut shell, "Undo ");
    shell.click(&undo);
    let again = shell.read(|root| root.app.encounters.roster_names().unwrap_or_default());
    assert_eq!(again[0], names[1], "Undo discard brings back the draft");
    // Ctrl+S saves it.
    assert!(shell.read(|root| root.app.encounters.encounter_dirty()));
    shell.press("ctrl-s");
    assert!(
        !shell.read(|root| root.app.encounters.encounter_dirty()),
        "Ctrl+S saves"
    );
}

/// A Ready row's menu removes the encounter; the footer's Undo brings it back.
#[test]
fn remove_an_encounter_and_undo() {
    let mut shell = Shell::new("encounter-build", Seed::Campaign);
    ready(&mut shell);
    let before = shell.read(|root| root.app.encounters.saved_names());
    shell.settle();
    let ready_menu = crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with("id:menu:Encounters/") && name.ends_with("-button"))
        .expect("a Ready row's menu");
    shell.click(&ready_menu);
    shell.capture("encounter-build__13-encounter-menu-remove__1200x800");
    shell.click("Remove");
    let after = shell.read(|root| root.app.encounters.saved_names());
    assert_eq!(
        after.len() + 1,
        before.len(),
        "Remove takes the encounter away"
    );
    shell.capture("encounter-build__14-encounter-removed__1200x800");
    let undo = first_target(&mut shell, "Undo ");
    shell.click(&undo);
    assert_eq!(shell.read(|root| root.app.encounters.saved_names()), before);
}

/// A saved group opens from the list; a member's count changes; a member's removal has an Undo
/// in the footer; Save group writes it; Back returns to the list.
#[test]
fn a_group_is_edited_and_saved() {
    let mut shell = Shell::new("encounter-groups", Seed::Groups);
    ready(&mut shell);
    shell.capture("encounter-groups__01-groups-in-the-list__1200x800");
    let edit = first_target(&mut shell, "id:edit-group:");
    shell.click(&edit);
    assert_eq!(
        shell.read(|root| root.app.encounters.group_open()),
        Some(false)
    );
    shell.capture("encounter-groups__02-group-editor__1200x800");
    let members = shell.read(|root| root.app.encounters.group_members().unwrap_or_default());
    assert!(!members.is_empty(), "the fixture's group has members");
    shell.click("id:member-more-0");
    let more = shell.read(|root| root.app.encounters.group_members().unwrap_or_default());
    assert_eq!(more[0].1, members[0].1 + 1, "+ adds one to the count");
    assert_eq!(
        shell.read(|root| root.app.encounters.group_open()),
        Some(true)
    );
    shell.click("id:member-menu-0-button");
    shell.click("Remove");
    let fewer = shell.read(|root| root.app.encounters.group_members().unwrap_or_default());
    assert_eq!(
        fewer.len() + 1,
        members.len(),
        "Remove takes the member away"
    );
    shell.capture("encounter-groups__10-group-member-removed-with-undo__1200x800");
    let undo = first_target(&mut shell, "Undo ");
    shell.click(&undo);
    let back = shell.read(|root| root.app.encounters.group_members().unwrap_or_default());
    assert_eq!(back.len(), members.len(), "Undo brings the member back");
    shell.click("Save group");
    assert_eq!(
        shell.read(|root| root.app.encounters.group_open()),
        Some(false),
        "saved"
    );
    shell.capture("encounter-groups__09-group-saved__1200x800");
    shell.click("id:group-back");
    assert_eq!(shell.read(|root| root.app.encounters.group_open()), None);
}

/// New group: an empty group cannot be saved; a creature from the search makes it savable.
#[test]
fn a_new_group_needs_a_creature() {
    let mut shell = Shell::new("encounter-groups", Seed::Groups);
    ready(&mut shell);
    shell.click("New group");
    shell.type_text("Ash Hounds");
    assert!(
        shell.bounds("Save group").is_some(),
        "the group editor is open"
    );
    shell.click("Save group");
    assert_eq!(
        shell.read(|root| root.app.encounters.group_members()),
        Some(Vec::new()),
        "an empty group is not saved"
    );
    assert!(
        shell
            .read(|root| root.app.encounters.group_open())
            .is_some()
    );
    shell.click("Find a creature");
    shell.type_text("wolf");
    let add = first_target(&mut shell, "id:add:");
    shell.click(&add);
    assert_eq!(
        shell.read(|root| root.app.encounters.group_members().map(|m| m.len())),
        Some(1)
    );
    shell.capture("encounter-groups__08-group-find-creature__1200x800");
    shell.press("ctrl-s");
    let lines: Vec<String> = shell.update(|root, _, _| {
        let root_path = root.app.campaigns.active_campaign().unwrap().path;
        root.app
            .encounters
            .group_footer(&root_path)
            .into_iter()
            .map(|line| line.words)
            .collect()
    });
    assert_eq!(
        shell.read(|root| root.app.encounters.group_open()),
        Some(false),
        "Ctrl+S saves: {lines:?}"
    );
}

/// Both editors at the smallest window.
#[test]
fn the_editors_at_the_smallest_window() {
    let mut shell = small("encounter-build", Seed::Campaign);
    ready(&mut shell);
    let edit = first_target(&mut shell, "id:edit:Encounters/");
    shell.click(&edit);
    assert!(shell.read(|root| root.app.encounters.editor_open()));
    shell.capture("encounter-build__09-saved-roster__960x640");
    let mut shell = small("encounter-groups", Seed::Groups);
    ready(&mut shell);
    let edit = first_target(&mut shell, "id:edit-group:");
    shell.click(&edit);
    assert_eq!(
        shell.read(|root| root.app.encounters.group_open()),
        Some(false)
    );
    shell.capture("encounter-groups__02-group-editor__960x640");
}
