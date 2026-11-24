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

/// Open the saved encounter `name` (`Encounters/<name>.md`) as its list's Edit does; with no
/// name, the first one the list offers.
fn open_encounter(shell: &mut Shell, name: &str) {
    if name.is_empty() {
        let edit = first_target(shell, "id:edit:Encounters/");
        shell.click(&edit);
    } else {
        let path = format!("Encounters/{name}.md");
        shell.update(move |root, _, cx| {
            let active = root.app.campaigns.active_campaign().unwrap();
            root.app.encounters.list_act(
                splat_encounters_ui::encounters::ListAction::Edit(path.clone()),
                &active,
                &root.app.campaigns,
            );
            cx.notify();
        });
    }
    shell.settle();
    assert!(shell.read(|root| root.app.encounters.editor_open()));
}

/// Open the saved group named `name` from the list's Groups.
fn open_group(shell: &mut Shell, name: &str) {
    let edit = first_target(shell, &format!("id:edit-group:Encounter Groups/{name}"));
    shell.click(&edit);
    assert!(shell.read(|root| root.app.encounters.group_open().is_some()));
    shell.settle();
}

/// The open draft's roster rows as the view shows them: each row's name and second line.
fn roster_lines(shell: &mut Shell) -> Vec<(String, String)> {
    shell.update(|root, _, _| {
        let ctx = root.bridge.ctx().clone();
        let linked = root
            .app
            .campaigns
            .active_campaign()
            .is_some_and(|active| active.dndbeyond_campaign_id.is_some());
        root.app
            .encounters
            .roster_view(&ctx, &root.app.party, linked)
            .map(|roster| {
                roster
                    .rows
                    .iter()
                    .map(|row| {
                        let words: Vec<String> =
                            row.parts.iter().map(|(words, ..)| words.clone()).collect();
                        (row.name.clone(), words.join(" · ").replace('\u{a0}', " "))
                    })
                    .collect()
            })
            .unwrap_or_default()
    })
}

/// Wait until the party's saved sheets are read (a background read), so the synced party rows
/// can show them.
fn wait_for_sheets(shell: &mut Shell) {
    for _ in 0..200 {
        let rows = roster_lines(shell);
        if rows.iter().any(|(_, line)| line.contains('/')) {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
        shell.wait(Duration::from_millis(50));
    }
    shell.settle();
}

/// A roster row's changes form: the change words open it; a number's `Add` writes a change;
/// a display name writes one; Done closes it and the row says its change again.
#[test]
fn a_rows_changes_are_edited_in_its_form() {
    use splat_encounters_ui::encounters::model::ChangesOf;
    let mut shell = Shell::new("run-overrides", Seed::Groups);
    ready(&mut shell);
    open_encounter(&mut shell, crate::visual_tests::fixture::RUN_ARENA);
    shell.capture("run-overrides__02-slot-with-changes__1200x800");
    let change = first_target(&mut shell, "id:roster-change:");
    let key = change.trim_start_matches("id:roster-change:").to_owned();
    shell.click(&change);
    let form = |shell: &mut Shell| {
        let key = key.clone();
        shell.read(move |root| {
            root.app
                .encounters
                .changes_form(&ChangesOf::Row(key))
                .map(|form| {
                    (
                        form.title.clone(),
                        form.name.clone(),
                        form.rows
                            .iter()
                            .map(|row| (row.label, row.how, row.number))
                            .collect::<Vec<_>>(),
                    )
                })
        })
    };
    let (title, _, before) = form(&mut shell).expect("the row's changes are open");
    assert!(title.starts_with("Changes for "), "{title}");
    assert!(
        shell.bounds("Done with changes").is_some(),
        "the form shows its Done"
    );
    shell.capture("run-overrides__03-slot-changes-open__1200x800");
    // AC: Same → Add, then one more with +.
    let ac_how = first_target(&mut shell, "id:changes-how:AC:");
    shell.click(&ac_how);
    shell.click("Add");
    let (_, _, after) = form(&mut shell).unwrap();
    let ac = after.iter().find(|(label, ..)| *label == "AC").unwrap();
    assert_eq!(
        (ac.1, ac.2),
        (splat_encounters_ui::encounters::model::ChangeHow::Add, 1),
        "Add starts at +1: {before:?} → {after:?}"
    );
    shell.click("AC: amount: more");
    let (_, _, after) = form(&mut shell).unwrap();
    let ac = after.iter().find(|(label, ..)| *label == "AC").unwrap();
    assert_eq!(ac.2, 2, "+ adds one to the amount");
    assert!(shell.read(|root| root.app.encounters.encounter_dirty()));
    // The display name is a change of its own.
    shell.click("Display name");
    shell.press("ctrl-a");
    shell.type_text("Captain Rook");
    let (title, name, _) = form(&mut shell).unwrap();
    assert_eq!(name, "Captain Rook");
    assert_eq!(title, "Changes for Captain Rook");
    shell.click("Done with changes");
    assert!(form(&mut shell).is_some(), "the block stays");
    assert!(
        shell.bounds("Done with changes").is_none(),
        "Done closes the form"
    );
    let rows = roster_lines(&mut shell);
    assert!(
        rows.iter().any(|(name, _)| name == "Captain Rook"),
        "the row shows its changed name: {rows:?}"
    );
}

/// A group's member: its change words open its Changes; a member whose creature is gone
/// offers a replacement picker whose `Use` replaces it with an Undo in the footer.
#[test]
fn a_members_changes_and_its_replacement() {
    let mut shell = Shell::new("encounter-groups", Seed::Groups);
    ready(&mut shell);
    open_group(&mut shell, "Rival");
    let change = first_target(&mut shell, "id:member-change-");
    shell.click(&change);
    assert!(shell.bounds("Maximum HP: how it changes").is_some());
    shell.capture("encounter-groups__03-group-member-changes__1200x800");
    shell.click("Done with changes");
    assert!(shell.bounds("Maximum HP: how it changes").is_none());
    shell.click("id:group-back");
    open_group(&mut shell, "Lost");
    let members = shell.read(|root| root.app.encounters.group_members().unwrap_or_default());
    let replace = first_target(&mut shell, "Replace ");
    shell.click(&replace);
    shell.click("Replacement creature");
    shell.type_text("Ember");
    assert!(shell.bounds("Use Ember Cultist").is_some(), "the picker offers it");
    shell.capture("encounter-groups__12-group-member-replacement-search__1200x800");
    shell.click("Use Ember Cultist");
    let replaced = shell.read(|root| root.app.encounters.group_members().unwrap_or_default());
    assert_ne!(replaced, members, "Use replaced the member's creature");
    assert_eq!(
        shell.read(|root| root.app.encounters.group_open()),
        Some(true)
    );
    shell.capture("encounter-groups__13-group-member-replaced__1200x800");
    let undo = first_target(&mut shell, "Undo ");
    shell.click(&undo);
    let back = shell.read(|root| root.app.encounters.group_members().unwrap_or_default());
    assert_eq!(back, members, "Undo puts the old creature back");
}

/// Groups for this encounter: the cards, a new group that takes the search's creatures,
/// Save encounter, and the card's menu. With party sync on, a party row shows the sheet's HP.
#[test]
fn an_encounters_own_groups() {
    let mut shell = Shell::new("encounter-owned-groups", Seed::OwnedGroups);
    ready(&mut shell);
    open_encounter(&mut shell, "Shrine of Cinders");
    wait_for_sheets(&mut shell);
    shell.capture("encounter-owned-groups__04-encounter-with-its-groups__1200x800");
    // The groups end the roster's column: a tall window shows them whole, so its presses land.
    let mut shell = Shell::with_size(
        "encounter-owned-groups",
        Seed::OwnedGroups,
        size(px(1200.0), px(1800.0)),
    );
    ready(&mut shell);
    open_encounter(&mut shell, "Shrine of Cinders");
    wait_for_sheets(&mut shell);
    // Gap 11: the party's rows read the sheets the harness shares (egui shows `HP 58/62`).
    let rows = roster_lines(&mut shell);
    assert!(
        rows.iter()
            .any(|(name, line)| name == "Brannoch Vale" && line.contains("HP 58/62")),
        "a synced party row shows its sheet's HP: {rows:?}"
    );
    let count = |shell: &mut Shell| {
        shell.update(|root, _, _| {
            let ctx = root.bridge.ctx().clone();
            root.app
                .encounters
                .owned_groups(&ctx)
                .map_or(0, |owned| owned.count)
        })
    };
    let before = count(&mut shell);
    shell.click("New group for this encounter");
    assert_eq!(count(&mut shell), before + 1, "a new group for the encounter");
    shell.capture("encounter-owned-groups__05-new-group-in-the-encounter__1200x1800");
    // The new group takes what the search adds.
    shell.type_text("Ash Hounds");
    shell.click("Find a creature or party member");
    shell.type_text("wolf");
    let add = first_target(&mut shell, "id:add:");
    let roster = roster_len(&mut shell);
    shell.click(&add);
    assert_eq!(roster_len(&mut shell), roster, "the roster did not take it");
    let members = shell.update(|root, _, _| {
        let ctx = root.bridge.ctx().clone();
        root.app.encounters.owned_groups(&ctx).map(|owned| {
            owned
                .cards
                .iter()
                .find(|card| card.name == "Ash Hounds")
                .map(|card| card.fields.members.len())
        })
    });
    assert_eq!(members, Some(Some(1)), "the new group took the creature");
    shell.capture("encounter-owned-groups__06-creature-added-to-a-group__1200x1800");
    shell.click("Save encounter");
    assert!(
        !shell.read(|root| root.app.encounters.encounter_dirty()),
        "Save encounter saves its groups"
    );
    shell.capture("encounter-owned-groups__07-encounter-and-group-saved__1200x1800");
    shell.click("More actions for Ash Hounds");
    shell.capture("encounter-owned-groups__08-move-encounter-group-menu__1200x1800");
    shell.click("Make available to any encounter");
    assert_eq!(count(&mut shell), before, "the group left this encounter");
    let undo = first_target(&mut shell, "Undo ");
    shell.click(&undo);
    assert_eq!(count(&mut shell), before + 1, "Undo brings it back");
}

/// The encounter's loot: Enter adds typed words; `roll` offers the campaign's tables, and a
/// table makes a roll line; Remove has an Undo.
#[test]
fn the_encounters_loot_list() {
    // The loot ends the roster's column: a tall window shows it and its suggestions whole.
    let mut shell = Shell::with_size("run-loot", Seed::Loot, size(px(1200.0), px(1400.0)));
    ready(&mut shell);
    open_encounter(&mut shell, crate::visual_tests::fixture::RUN);
    let loot = |shell: &mut Shell| {
        shell.update(|root, _, cx| {
            let ctx = root.bridge.ctx().clone();
            let _ = cx;
            root.app
                .encounters
                .roster_view(&ctx, &root.app.party, false)
                .map(|roster| roster.loot.lines)
                .unwrap_or_default()
        })
    };
    let before = loot(&mut shell);
    shell.click("Add loot to the encounter");
    shell.type_text("120 gp in a locked coffer");
    shell.press("enter");
    let added = loot(&mut shell);
    assert_eq!(added.len(), before.len() + 1, "Enter adds a line");
    assert_eq!(added.last().map(String::as_str), Some("120 gp in a locked coffer"));
    shell.type_text("roll");
    let gems = first_target(&mut shell, "gems");
    shell.capture("run-loot__04-loot-table-suggestion__1200x1400");
    shell.click(&gems);
    let rolled = loot(&mut shell);
    assert_eq!(rolled.last().map(String::as_str), Some("Roll gems"), "{rolled:?}");
    assert!(
        shell.bounds("Table for the encounter loot line 2").is_some()
            || first_target(&mut shell, "Table for the encounter loot line").len() > 0,
        "a roll line has its table menu"
    );
    shell.capture("run-loot__05-loot-in-the-editor__1200x1400");
    let menu = first_target(&mut shell, "More actions for the encounter loot line 1");
    shell.click(&menu);
    shell.click("Remove");
    assert_eq!(loot(&mut shell).len(), rolled.len() - 1, "Remove takes the line");
    shell.click("Undo remove loot");
    assert_eq!(loot(&mut shell), rolled, "Undo puts it back");
}

/// `Make it yours`: the creature pane in Add participants' place; a changed hit points value
/// makes it Unsaved; Save creature writes the copy and closes the pane.
#[test]
fn a_creature_made_yours_is_edited_beside_the_roster() {
    let mut shell = Shell::new("encounter-build", Seed::Campaign);
    ready(&mut shell);
    open_encounter(&mut shell, "");
    let menus: Vec<String> = {
        shell.settle();
        crate::gui::kit::probe::names()
            .into_iter()
            .filter(|name| name.starts_with("id:roster-menu:") && name.ends_with("-button"))
            .collect()
    };
    let mut opened = false;
    for menu in menus {
        shell.click(&menu);
        if shell.bounds("Make it yours").is_some() {
            shell.click("Make it yours");
            opened = true;
            break;
        }
        shell.press("escape");
    }
    assert!(opened, "a creature row offers Make it yours");
    assert!(shell.read(|root| root.app.encounters.creature_open()));
    assert!(shell.bounds("Save creature").is_some());
    shell.capture("encounter-build__15-make-it-yours__1200x800");
    // Hit points take the keyboard as the pane opens.
    shell.type_text("77");
    let dirty = shell.update(|root, _, _| {
        root.app
            .encounters
            .creature_view()
            .map(|view| (view.hp, view.dirty))
    });
    assert_eq!(dirty, Some(("77".to_owned(), true)));
    shell.click("Save creature");
    assert!(
        !shell.read(|root| root.app.encounters.creature_open()),
        "Save creature writes it and closes the pane"
    );
}

/// The encounter's file changed on disk under a draft: Save is refused and the conflict takes
/// the editor's place, with the draft beside the file; Overwrite with my draft saves it.
#[test]
fn a_changed_file_shows_the_draft_beside_it() {
    let mut shell = Shell::new("encounter-build", Seed::Campaign);
    ready(&mut shell);
    open_encounter(&mut shell, "");
    let path = shell.read(|root| {
        let active = root.app.campaigns.active_campaign().unwrap();
        active
            .path
            .join(root.app.encounters.draft_path().unwrap_or_default())
    });
    let text = std::fs::read_to_string(&path).unwrap();
    std::fs::write(&path, text.replace("\n## ", "\nAn outside edit.\n\n## ")).unwrap();
    shell.click("Encounter name");
    shell.press("end");
    shell.type_text(" again");
    shell.click("Save encounter");
    assert!(
        shell.read(|root| root.app.encounters.encounter_disk().is_some()),
        "Save found the change on disk"
    );
    assert!(shell.bounds("Overwrite with my draft").is_some());
    assert!(shell.bounds("Reload and lose my draft").is_some());
    shell.capture("encounter-build__16-encounter-changed-on-disk__1200x800");
    shell.click("Overwrite with my draft");
    assert!(shell.read(|root| root.app.encounters.encounter_disk().is_none()));
    assert!(
        !shell.read(|root| root.app.encounters.encounter_dirty()),
        "Overwrite saved the draft"
    );
    assert!(std::fs::read_to_string(&path).unwrap().contains(" again"));
}

/// Beside Lookup at the smallest window, the roster and Add participants take turns under
/// tabs; a dragged Add participants width is kept for `ui_state`.
#[test]
fn the_narrow_editor_takes_turns_and_a_dragged_width_is_kept() {
    let mut shell = Shell::with_size("column-resize", Seed::Campaign, size(px(900.0), px(640.0)));
    ready(&mut shell);
    open_encounter(&mut shell, "");
    assert!(shell.bounds("Roster").is_some() || first_target(&mut shell, "Roster").len() > 0);
    shell.capture("column-resize__09-roster-follows-add-panel__900x640");
    let add = first_target(&mut shell, "Add participants");
    shell.click(&add);
    assert!(
        shell.bounds("Find a creature or party member").is_some(),
        "the Add tab shows the search"
    );
    let mut shell = Shell::new("column-resize", Seed::Campaign);
    ready(&mut shell);
    shell.update(|root, _, cx| {
        root.set_column_width(super::editor::ADD_PANE, 360.0);
        cx.notify();
    });
    open_encounter(&mut shell, "");
    let search = shell.bounds("Find a creature or party member").unwrap();
    assert!(
        search.size.width < px(360.0),
        "the search fits the kept width: {search:?}"
    );
    assert_eq!(
        shell.read(|root| root.column_width(super::editor::ADD_PANE)),
        Some(360.0)
    );
    shell.capture("column-resize__09-roster-follows-add-panel__1200x800");
}

/// Read the campaign again (the list's Reload from disk) and wait for the read.
fn reload(shell: &mut Shell) {
    shell.click("Reload from disk");
    ready(shell);
}

/// The open roster's difficulty, as its heading says it.
fn roster_difficulty(shell: &mut Shell) -> String {
    shell.update(|root, _, _| {
        let ctx = root.bridge.ctx().clone();
        root.app
            .encounters
            .roster_view(&ctx, &root.app.party, true)
            .map(|roster| {
                roster
                    .difficulty
                    .iter()
                    .map(|(words, _)| words.clone())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default()
    })
}

/// The editor's roster heading says its difficulty, and whose XP is not known
/// (`encounter-difficulty`, with its scenario's creatures and encounters written the same way).
#[test]
fn the_editor_says_the_rosters_difficulty() {
    let mut shell = Shell::new("encounter-difficulty", Seed::Campaign);
    ready(&mut shell);
    let root = shell.read(|root| root.app.campaigns.active_campaign().unwrap().path);
    let creature = |name: &str, cr: Option<&str>| {
        let id = crate::hermetic::new_uuid();
        let relative = format!("Creatures/{name}.md");
        let challenge = cr.map_or_else(String::new, |cr| format!(", cr_text: '{cr}'"));
        let text = splat_content::content::formats::fixture(
            &relative,
            &format!(
                "splat_id: {id}\nsplat_kind: creature\nsplat_name: {name}\nsplat_creature: {{size: large, type: fiend{challenge}}}\nsplat_stats:\n  max_hp: {{value: 120}}\n  armor_class: {{value: 16}}\n  initiative: {{value: 2}}\n"
            ),
            &format!("\n# {name}\n"),
        )
        .unwrap();
        let path = root.join(&relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
        id
    };
    let shrine = std::fs::read_to_string(root.join("Encounters/Shrine of Cinders.md")).unwrap();
    let shrine = splat_content::content::Entry::parse("Encounters/Shrine.md", &shrine).unwrap();
    let setup = shrine.fields["splat_encounter"].clone();
    let cultist: uuid::Uuid = setup["roster"]
        .as_sequence()
        .unwrap()
        .iter()
        .find(|slot| slot["side"] == "enemies")
        .and_then(|slot| slot["creature"]["id"].as_str())
        .unwrap()
        .parse()
        .unwrap();
    let encounter = |name: &str, party: bool, foes: &[uuid::Uuid]| {
        let mut setup = setup.clone();
        let roster = setup["roster"].as_sequence_mut().unwrap();
        roster.retain(|slot| party && slot["side"] == "party");
        for (at, foe) in foes.iter().enumerate() {
            let slot = format!(
                "{{key: actor_foe_{at}, creature: {{id: {foe}}}, side: enemies, controller: {{kind: manual, role: gm}}}}"
            );
            roster.push(serde_yaml_ng::from_str(&slot).unwrap());
        }
        let mut fields = serde_yaml_ng::Mapping::new();
        for (key, value) in [
            (
                "splat_id",
                serde_yaml_ng::Value::from(crate::hermetic::new_uuid().to_string()),
            ),
            ("splat_kind", "encounter".into()),
            ("splat_name", name.into()),
            ("splat_encounter", setup),
        ] {
            fields.insert(key.into(), value);
        }
        let relative = format!("Encounters/{name}.md");
        let text = splat_content::content::formats::fixture(
            &relative,
            &serde_yaml_ng::to_string(&fields).unwrap(),
            "\nWritten for the difficulty scenario.\n",
        )
        .unwrap();
        std::fs::write(root.join(relative), text).unwrap();
    };
    let tyrant = creature("Cinder Tyrant", Some("10"));
    let nameless = creature("Nameless Thing", None);
    encounter("Mill Road", true, &[tyrant, cultist, cultist]);
    encounter("Nameless Visitor", true, &[nameless]);
    reload(&mut shell);
    shell.capture("encounter-difficulty__01-ready-to-run__1200x800");
    open_encounter(&mut shell, "Mill Road");
    let words = roster_difficulty(&mut shell);
    assert!(words.contains("High"), "the roster says High: {words}");
    shell.capture("encounter-difficulty__03-editor-high__1200x800");
    shell.click("id:encounter-back");
    open_encounter(&mut shell, "Nameless Visitor");
    let words = roster_difficulty(&mut shell);
    assert!(
        words.contains("XP not known") && words.contains("Nameless Thing"),
        "the roster says whose XP is not known: {words}"
    );
    shell.capture("encounter-difficulty__05-editor-unknown__1200x800");
}

/// An NPC in the search: it adds its stat block under its own name, then says In roster.
#[test]
fn an_npc_is_found_and_added_by_its_name() {
    let mut shell = Shell::new("npc-in-encounter", Seed::Campaign);
    ready(&mut shell);
    let campaign = shell.read(|root| root.app.campaigns.active_campaign().unwrap());
    let catalogue = crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
    let guard = catalogue
        .entries
        .values()
        .find(|entry| entry.entry.kind == "creature" && entry.entry.name == "Guard")
        .unwrap()
        .entry
        .id;
    let text = splat_content::content::formats::fixture(
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
    reload(&mut shell);
    shell.click("New encounter");
    shell.type_text("Rook's ambush");
    shell.click("Find a creature or party member");
    shell.type_text("Rook Ambry");
    shell.capture("npc-in-encounter__01-npc-result__1200x800");
    shell.click("Add Rook Ambry");
    let names = shell.read(|root| root.app.encounters.roster_names().unwrap_or_default());
    assert!(
        names.iter().any(|name| name == "Rook Ambry"),
        "one Add uses the stat block under the NPC's name: {names:?}"
    );
    assert!(
        shell.bounds("Add Rook Ambry").is_none(),
        "the NPC on the roster says In roster"
    );
    shell.capture("npc-in-encounter__02-npc-in-roster__1200x800");
}

/// No creature matches as typed, but a package the campaign does not use has one: the search
/// says so with `Use it`, which asks to use that package's entry (`use_package_entry`).
#[test]
fn use_it_offers_a_creature_from_an_unused_package() {
    // The fixture's unchosen Ash Wraith shares its words with the SRD's Wraith, which the
    // search finds as typed, so (as in egui) it offers nothing to use. A creature of the same
    // package that the campaign did not choose, with a name of its own, is offered.
    crate::hermetic::closed::repeatable_uuids(1);
    let home = tempfile::tempdir().expect("a test folder");
    let root = home.path().join("use-it");
    let world = crate::visual_tests::fixture::World::build(
        &root,
        &crate::gui::harness::character_cache(),
        "use-it",
        Seed::Campaign,
    )
    .unwrap();
    let relative = "Creatures/Gloam Stalker.md";
    let text = splat_content::content::formats::fixture(
        relative,
        &format!(
            "splat_id: {}\nsplat_kind: creature\nsplat_name: Gloam Stalker\nsplat_creature: {{size: large, type: fiend, cr_text: '2'}}\nsplat_stats:\n  max_hp: {{value: 40}}\n  armor_class: {{value: 13}}\n  initiative: {{value: 2}}\n",
            uuid::Uuid::from_u128(0x9100_0000_0000_0000_0000_0000_0000_0001)
        ),
        "\n# Gloam Stalker\n",
    )
    .unwrap();
    std::fs::write(root.join("modules/ashen-marches").join(relative), text).unwrap();
    let mut shell = Shell::over(world.app(), home, size(px(1200.0), px(800.0)));
    ready(&mut shell);
    shell.click("New encounter");
    shell.click("Find a creature or party member");
    shell.type_text("gloam");
    // Lookup's index, which the search reads, is built in the background.
    let mut unused = Vec::new();
    for _ in 0..400 {
        unused = shell.update(|root, _, _| {
            let ctx = root.bridge.ctx().clone();
            root.app
                .encounters
                .unused_creatures(&ctx)
                .into_iter()
                .map(|unused| unused.words)
                .collect::<Vec<_>>()
        });
        if !unused.is_empty() {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
        shell.wait(Duration::from_millis(50));
    }
    shell.settle();
    assert!(
        unused.iter().any(|words| words.starts_with("Gloam Stalker is in ")),
        "the search names the unused package's creature: {unused:?}"
    );
    shell.capture("encounter-build__17-use-it__1200x800");
    let use_it = first_target(&mut shell, "Use it: Gloam Stalker");
    shell.click(&use_it);
    // The campaign now uses the entry, and the search reads its creatures again.
    let mut usable = false;
    for _ in 0..400 {
        usable = shell.read(|root| root.app.encounters.searchable_creature("Gloam Stalker"));
        if usable {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
        shell.wait(Duration::from_millis(50));
    }
    assert!(usable, "Use it puts the creature in the campaign's search");
}
