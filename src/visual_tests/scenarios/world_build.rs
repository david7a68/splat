//! Desk work in World, including a real disk edit while the app stays open.
use super::*;
use crate::design;
use std::fs;
use uuid::Uuid;

fn root(d: &Driver<'_>) -> std::path::PathBuf {
    d.root.join("campaigns/emberfall")
}

fn entry(d: &Driver<'_>, path: &str, name: &str, kind: &str, fields: &str) -> Uuid {
    let id = splat::character::stable_id(path);
    let path = root(d).join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, splat::content::formats::convert("World.md", &format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n  type: {kind}\n{fields}---\n\n# {name}\n\nA note for tonight.\n")).unwrap()).unwrap();
    id
}

fn reload(d: &mut Driver<'_>) {
    d.app_mut().lookup.refresh();
    d.settle();
}

fn reveal_editor_passage(d: &mut Driver<'_>, words: &str) {
    d.scroll_up_at(READER, 200);
    for _ in 0..200 {
        if d.visible_passage(words) {
            return;
        }
        d.scroll_at(READER, 1);
    }
    panic!("The editor did not show {words}");
}

pub fn broken(d: &mut Driver<'_>) {
    let path = root(d).join("Notes/Broken.md");
    fs::write(&path, "---\nsplat_schema: 2\nsplat_id: [\n---\n").unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.expect("1 note cannot be read");
    d.observe(
        "W5",
        d.visible_text("4 notes"),
        "The World total includes the note that cannot be read",
    );
    d.state("broken-note-listed", Sizes::All);
    d.click_containing("Notes/Broken.md");
    d.expect("Save repaired note");
    d.observe(
        "W5",
        d.visible_text("Line 3 opens a list") && d.has("Edit in Obsidian"),
        "The broken note opens the shared repair editor and its file actions",
    );
    d.state("broken-note-repair", Sizes::All);
    let external = "---\nsplat_schema: 2\nsplat_id: [\n---\n\nOutside edit kept.\n";
    fs::write(&path, external).unwrap();
    d.settle();
    assert_eq!(d.editor_text().as_deref(), Some(external));
    d.state("clean-repair-follows-disk", Sizes::All);
    let unfinished = format!("{external}\nMy unfinished draft.\n");
    let outside = format!("{external}\nNew words on disk.\n");
    d.replace_text_in("Note text", &unfinished);
    fs::write(&path, &outside).unwrap();
    d.settle();
    d.click("Overwrite with my draft");
    assert_eq!(fs::read_to_string(&path).unwrap(), outside);
    assert_eq!(d.editor_text().as_deref(), Some(unfinished.as_str()));
    d.state("invalid-conflict-draft-kept-for-repair", Sizes::All);
    let fixed = format!(
        "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Repaired watchkeeper\nsplat_type: npc\n---\n\nThe watchkeeper is back.\n",
        splat::character::stable_id("repaired-watchkeeper")
    );
    d.replace_text_in("Note text", &fixed);
    fs::write(&path, format!("{external}\nAnother outside edit.\n")).unwrap();
    d.settle();
    d.expect("This entry changed on disk. Your draft is safe until you choose.");
    d.state("dirty-repair-compares-disk", Sizes::All);
    d.click("Reload and lose my draft");
    d.expect("Undo reload");
    assert_eq!(
        d.editor_text().as_deref(),
        Some(format!("{external}\nAnother outside edit.\n").as_str())
    );
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        format!("{external}\nAnother outside edit.\n")
    );
    d.state("repair-reloaded-with-undo", Sizes::All);
    d.click("Undo reload");
    assert_eq!(d.editor_text().as_deref(), Some(fixed.as_str()));
    d.state("repair-reload-undone", Sizes::All);
    fs::write(&path, format!("{external}\nA later outside edit.\n")).unwrap();
    d.settle();
    d.click("Reload and lose my draft");
    d.replace_text_in("Note text", &fixed);
    d.observe(
        "iter-1-A-reload-next-edit",
        !d.has("Undo reload") && d.editor_text().as_deref() == Some(fixed.as_str()),
        "A new edit ends Undo reload and keeps the newly typed draft",
    );
    d.expect_same("Editing after reload restores the same draft as Undo reload; the recovery control is gone in both states.");
    d.state("repair-edited-after-reload", Sizes::All);
    d.observe_both(
        "iter-18-A-world-lock",
        "A World repair states the campaign switch lock",
        |d| d.visible_text(crate::encounters::draft_switch_reason()),
    );
    d.click("Setting · ");
    d.click_containing("Notes/Broken.md");
    d.observe(
        "W5",
        d.editor_text().as_deref() == Some(&fixed),
        "Reading another note keeps the repair draft",
    );
    // A second disk edit lets the DM save the draft in one explicit choice.
    fs::write(&path, format!("{external}\nOne more outside edit.\n")).unwrap();
    d.settle();
    d.click("Overwrite with my draft");
    d.settle();
    d.observe(
        "W5",
        !d.has("1 note cannot be read") && fs::read_to_string(&path).unwrap() == fixed,
        "Saving repairs the note and removes the warning",
    );
    d.wait_for_text("Note repaired.");
    d.expect("The watchkeeper is back.");
    d.state("broken-note-repaired", Sizes::Default);

    fs::write(&path, "---\nsplat_schema: 2\nsplat_id: [\n---\n").unwrap();
    reload(d);
    look_up(d, "watchkeeper");
    d.expect("1 note cannot be read");
    d.state("broken-note-in-lookup", Sizes::All);
    d.click("Show");
    d.expect("Save repaired note");
    d.observe(
        "W5",
        d.current_page() == "World" && d.has("Note text"),
        "Lookup opens the broken file on World",
    );
    d.replace_editor_text(&fixed);
    d.click("Save repaired note");
    d.settle();
    d.observe(
        "W5",
        !d.has("1 note cannot be read") && fs::read_to_string(&path).unwrap() == fixed,
        "Save repaired note writes the repair and removes the warning",
    );
}

pub fn places(d: &mut Driver<'_>) {
    entry(
        d,
        "Locations/Ashfall Keep.md",
        "Ashfall Keep",
        "location",
        "  aliases: [The Keep]\n",
    );
    entry(
        d,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        "  in: '[[The Keep]]'\n",
    );
    entry(
        d,
        "Locations/Loop One.md",
        "Loop One",
        "location",
        "  in: '[[Loop Two]]'\n",
    );
    entry(
        d,
        "Locations/Loop Two.md",
        "Loop Two",
        "location",
        "  in: '[[Loop One]]'\n",
    );
    fs::write(
        root(d).join("Visitors.md"),
        "# Visitors\n\nRook Ambry guards [[Ashfall Keep]].\n",
    )
    .unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Rook Ambry · ");
    d.observe(
        "W3",
        d.has("Ashfall Keep") && d.has("Change Stat block") && d.has("Change In"),
        "An NPC can use a place's alias in its in link",
    );
    d.state("npc-place", Sizes::All);
    let original = fs::read(root(d).join("NPCs/Rook Ambry.md")).unwrap();
    let before = d.interaction_count();
    d.click("Change Stat block");
    d.type_text("Guard");
    d.click("Guard");
    d.observe(
        "W3",
        d.interaction_count() - before <= 3,
        "Give an NPC a stat block in three interactions",
    );
    assert!(
        fs::read_to_string(root(d).join("NPCs/Rook Ambry.md"))
            .unwrap()
            .contains("splat_stat_block:")
    );
    d.expect_same("Adding the stat block keeps the note's content and offers Undo.");
    d.state("npc-stat-block-changed", Sizes::All);
    d.click("Undo stat block change");
    d.observe(
        "W3",
        fs::read(root(d).join("NPCs/Rook Ambry.md")).unwrap() == original,
        "Undo restores the previous content byte for byte",
    );
    d.click("Change In");
    d.click("New location…");
    d.type_into("Location name", "Watch House");
    d.click("Create location here");
    d.expect("In: The Keep → Watch House.");
    d.state("npc-new-place", Sizes::All);
    d.click("Undo place change");
    d.click("Ashfall Keep");
    d.observe(
        "World places",
        d.has("Rook Ambry") && d.has("Visitors"),
        "Here and Mentioned in show the people and notes at a place",
    );
    d.state("place-here-and-mentions", Sizes::All);
    d.click_containing("Loop One · ");
    d.observe(
        "World places",
        d.visible_text("Loop One is in Loop Two"),
        "A place loop remains visible and says why",
    );
    d.state("place-loop", Sizes::All);

    d.click_containing("Rook Ambry · ");
    let path = root(d).join("NPCs/Rook Ambry.md");
    let external = fs::read_to_string(&path)
        .unwrap()
        .replace("A note for tonight.", "## Watch schedule\n\nObsidian added the watch schedule.\n\nRook takes the first watch at the east gate. At midnight, the relief patrol leaves Watch House and checks the bridge.\n\nThe captain keeps a spare key in the guard room. Visitors must give their names before they enter the keep.");
    fs::write(&path, &external).unwrap();
    // No focus event: Change must re-read the file before it patches this one key.
    d.click("Change In");
    d.type_text("Watch House");
    d.click("Watch House");
    let after = fs::read_to_string(&path).unwrap();
    assert!(after.contains("Obsidian added the watch schedule."));
    assert_ne!(
        splat::content::Entry::parse("NPCs/Rook Ambry.md", &after)
            .unwrap()
            .fields["splat_world"]["in"],
        splat::content::Entry::parse("NPCs/Rook Ambry.md", &external)
            .unwrap()
            .fields["splat_world"]["in"]
    );
    d.wait_for_text("Obsidian added the watch schedule.");
    d.state("place-change-keeps-disk-edit", Sizes::All);
    d.click("Undo place change");
    assert_eq!(fs::read_to_string(&path).unwrap(), external);
}

pub fn round_trip(d: &mut Driver<'_>) {
    d.click(NAV_WORLD);
    d.click("Setting · ");
    d.expect("Edit in Obsidian");
    d.state("before-disk-edit", Sizes::All);
    let path = root(d).join("Setting.md");
    fs::write(&path, "# Setting\n\nThe cold war ended this morning.\n").unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("The cold war ended this morning.");
    d.observe(
        "World round trip",
        d.visible_text("The cold war ended this morning.")
            && fs::read_to_string(path).unwrap()
                == "# Setting\n\nThe cold war ended this morning.\n",
        "Returning to the app reloads the selected note without changing its text",
    );
    d.state("after-disk-edit", Sizes::All);
    d.click("New ▾");
    d.click("Note…");
    d.type_into("Name", "Ashfall Faction");
    d.type_into("Type (optional)", "faction");
    d.click("Create note");
    d.expect("Edit in Obsidian");
    d.observe(
        "W12",
        fs::read_to_string(root(d).join("Notes/Ashfall Faction.md"))
            .unwrap()
            .contains("type: faction"),
        "New Note accepts an optional World type and offers the file editor",
    );
    d.state("new-faction-note", Sizes::All);
    go_campaign(d);
    d.click(TAB_ENTRIES);
    d.click_containing("Captain Varra · Creature");
    d.click("Make NPC note");
    d.expect("Change Stat block");
    d.observe(
        "W13",
        d.current_page() == "World" && d.has("Captain Varra"),
        "Make NPC note opens the linked person in World",
    );
    d.state("campaign-creature-as-npc", Sizes::All);
}

pub fn tables(d: &mut Driver<'_>) {
    let body = "# Table Workshop\n\n<!-- splat:rolls key=valid dice=1d6 -->\n\n| d6 | Result |\n|---|---|\n| 1 | Rain |\n| 2-5 | Fog |\n| 6 | Sun |\n\n<!-- splat:rolls key=gap dice=1d4 -->\n\n| d4 | Gap |\n|---|---|\n| 1 | One |\n| 3-4 | Later |\n\n<!-- splat:rolls key=overlap dice=1d4 -->\n\n| d4 | Overlap |\n|---|---|\n| 1-3 | First |\n| 3-4 | Second |\n\n<!-- splat:rolls key=words dice=1d6 -->\n\n| d6 | Words |\n|---|---|\n| 1 | One |\n| 2 to 5 | Fog |\n| 6 | Six |\n";
    let body = body.replace(
        "<!-- splat:rolls key=words",
        &format!(
            "{}<!-- splat:rolls key=words",
            "\nThe next table checks a range written as words.\n".repeat(8)
        ),
    );
    fs::write(root(d).join("Table Workshop.md"), body).unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Table Workshop · ");
    d.observe_both(
        "W6",
        "Table faults appear in the World list and warning count",
        |d| d.has_containing("3 table faults") && d.visible_text("1 note needs fixing"),
    );
    d.state("authored-table-validation", Sizes::All);
    d.observe_both(
        "World typed roll field",
        "The typed roll field shares the Roll button edges and has an S gap",
        |d| {
            let Some(button) = d.place_of("Roll 1d6") else {
                return false;
            };
            let Some(field) = d.place_of("Typed roll for Valid") else {
                return false;
            };
            (button.top() - field.top()).abs() <= 1.0
                && (button.bottom() - field.bottom()).abs() <= 1.0
                && (field.left() - button.right() - design::S).abs() <= 1.0
                && (design::XXL * 3.0..=design::XXL * 3.0 + design::S).contains(&field.width())
        },
    );
    d.type_into("Typed roll for Valid", "4");
    d.press(Key::Enter);
    d.observe(
        "World table author",
        d.visible_text("Fog") && d.visible_text("Typed 4"),
        "A valid range accepts a typed roll and shows the matched row",
    );
    d.state("authored-table-typed-roll", Sizes::All);
    d.set_base(SMALL);
    d.scroll_until(READER, "The first column must be a number", 24);
    d.wait_for_text("The first column must be a number");
    d.observe(
        "World table author",
        d.visible_text("The first column must be a number"),
        "An invalid table says what needs an edit",
    );
    d.state_prepared("authored-table-gap-and-overlap", Sizes::All, |d| {
        d.scroll_until(READER, "The first column must be a number", 24);
    });
}

pub fn generator(d: &mut Driver<'_>) {
    let path = root(d).join("Names.md");
    fs::write(&path, "# Names\n\n<!-- splat:rolls key=npc-name dice=1d2 -->\n\n| d2 | Name |\n|---|---|\n| 1-2 | Tally |\n").unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click("New ▾");
    d.click("Generate NPC");
    d.expect("Tally");
    d.observe(
        "W14",
        d.visible_text("Names from") && d.has("Names.md"),
        "The generator names the campaign table used for names",
    );
    d.state("generator-campaign-table", Sizes::All);
    d.click("Discard");
    fs::write(path, "# Names\n\n<!-- splat:rolls key=npc-name dice=1d6 -->\n\n| d6 | Name |\n|---|---|\n| 1 | Tally |\n").unwrap();
    reload(d);
    d.click("New ▾");
    d.click("Generate NPC");
    d.observe(
        "W14",
        d.visible_text("the npc-name table stops at 1, but 1d6 rolls up to 6.")
            && d.visible_text("Built-in names stand in until it is fixed."),
        "A bad campaign table states why the built-in list stands in",
    );
    d.state("generator-bad-table", Sizes::All);
    d.click("Open Names.md");
    d.wait_for_text("Names");
    d.observe(
        "W9",
        d.has("Close Lookup") && !d.has("New NPC \"Names.md\""),
        "The warning opens its source note in Lookup",
    );
}

/// The NPC keeps both reference links beside a live fight.
pub fn npc_in_lookup(d: &mut Driver<'_>) {
    let modules = d.app_mut().campaigns.modules().to_vec();
    let campaign = d.app_mut().campaigns.active_campaign();
    let catalogue = crate::content_workspace::Catalogue::load(&modules, campaign.as_ref()).unwrap();
    let guard = catalogue
        .entries
        .values()
        .find(|i| i.entry.name == "Guard")
        .unwrap()
        .entry
        .id;
    let place = entry(d, "Locations/Watch House.md", "Watch House", "location", "");
    entry(
        d,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        &format!("  creature: {{id: {guard}}}\n  in: {{id: {place}}}\n"),
    );
    reload(d);
    resume_run_from_cold_start(d);
    look_up(d, "Rook Ambry");
    d.wait_for_text("Stat block");
    d.expect("Watch House");
    d.observe_both(
        "iter-1-A-single-lookup-result",
        "The one open Lookup result does not repeat its row above the reader",
        |d| !d.has("Rook Ambry · NPC · Campaign"),
    );
    d.observe(
        "W2",
        d.in_view(NEXT_TURN) && d.has("Guard") && !d.has("Change Stat block"),
        "The compact NPC reader shows its stat block and place beside the run",
    );
    d.state("npc-with-stat-block-and-place", Sizes::All);
    d.begin_task();
    d.click("Guard");
    d.wait_for_text("ARMOR CLASS");
    d.audit_goal_within("W2", "read an NPC stat block beside the run", 1);
    d.state("npc-stat-block-link", Sizes::All);
    d.click("Back");
    d.click("Watch House");
    d.wait_for_text("Rook Ambry");
    d.state("npc-place-link", Sizes::All);
}

/// A rename on disk keeps an identity-based package link working.
pub fn package_rename(d: &mut Driver<'_>) {
    let package = d.root.join("modules/ashen-marches");
    let (entries, _) = splat::content::read_directory(&package).unwrap();
    let target = entries.iter().find(|e| e.name == "Ashfall").unwrap();
    let path = package.join("Rules/Ash Exposure.md");
    let original = fs::read_to_string(&path).unwrap() + "\nSee [[Cinder Wolf]].\n";
    let reading = serde_yaml_ng::to_value(serde_json::json!({"title": "Ash Exposure", "summary": "The watch knows this spell.", "related": { target.id.to_string(): "Read the watch spell" }})).unwrap();
    fs::write(
        &path,
        crate::content_workspace::patch_fields(&original, &[("splat_reading", Some(reading))])
            .unwrap(),
    )
    .unwrap();
    go_library(d);
    d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
    d.click(MODULE);
    d.click_starting("Ash Exposure · ");
    d.scroll_until(READER, "Read the watch spell", 12);
    d.scroll_at(READER, 1);
    d.wait_for_text("Read the watch spell");
    d.state("before-linked-entry-rename", Sizes::All);
    let old_path = package.join(&target.path);
    let renamed = crate::content_workspace::patch_fields(
        &fs::read_to_string(&old_path).unwrap(),
        &[("splat_name", Some("Watch Ashfall".into()))],
    )
    .unwrap();
    fs::write(package.join("Spells/Watch Ashfall.md"), renamed).unwrap();
    fs::remove_file(old_path).unwrap();
    d.click("Packages");
    d.click("Reload from disk");
    d.scroll_until(READER, "Read the watch spell", 12);
    d.scroll_at(READER, 1);
    d.wait_for_text("Read the watch spell");
    d.click("Read the watch spell");
    d.wait_for_text("Watch Ashfall");
    d.state("linked-entry-renamed-on-disk", Sizes::All);
    d.click("Use Watch Ashfall");
    d.wait_for_text("Undo heading change");
    d.state("renamed-entry-heading-fixed", Sizes::All);
    d.click("Undo heading change");
    d.wait_for_text("Use Watch Ashfall");
    assert!(
        fs::read_to_string(package.join("Spells/Watch Ashfall.md"))
            .unwrap()
            .contains("# Ashfall")
    );
    d.click("Use Watch Ashfall");

    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    d.click(EDIT_MARKDOWN);
    let draft = d.editor_text().unwrap();
    let renamed =
        crate::content_workspace::patch_fields(&draft, &[("splat_name", Some("Ash Wolf".into()))])
            .unwrap()
            .replace("# Cinder Wolf", "# Ash Wolf");
    d.replace_text_in("Markdown editor", &renamed);
    d.click("Save");
    d.wait_for_text("Update 1 link");
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.state("renamed-creature-offers-link-update", Sizes::All);
    d.click("Update 1 link");
    d.wait_for_text("Undo link update");
    assert!(fs::read_to_string(&path).unwrap().contains("[[Ash Wolf]]"));
    d.state("renamed-creature-links-updated", Sizes::All);
    d.click("Undo link update");
    assert!(
        fs::read_to_string(&path)
            .unwrap()
            .contains("[[Cinder Wolf]]")
    );
    d.state("renamed-creature-link-update-undone", Sizes::All);

    let moved = d.root.join("modules/watch-package");
    fs::rename(&package, &moved).unwrap();
    go_campaign(d);
    d.click("Reload from disk");
    d.expect("Find the folder…");
    d.state("package-folder-moved-on-disk", Sizes::All);
    let wrong = d.root.join("modules/wrong-package");
    fs::create_dir(&wrong).unwrap();
    fs::write(
        wrong.join("Package.md"),
        crate::library::note_text("Wrong package", "package", Some("wrong")),
    )
    .unwrap();
    crate::hermetic::closed::next_folder(wrong);
    d.click("Find the folder…");
    d.expect("Choose the folder for this package.");
    d.state("package-wrong-folder-picked", Sizes::All);
    crate::hermetic::closed::next_folder(moved.clone());
    d.click("Find the folder…");
    d.wait_for_text("Undo folder recovery");
    d.expect("Ashen Marches found in modules/watch-package/.");
    d.state("package-correct-folder-picked", Sizes::All);
    d.click("Undo folder recovery");
    d.wait_for_text("Use this folder");
    d.click("Use this folder");
    d.wait_for_text("Undo folder recovery");
    go_library(d);
    d.click("Packages");
    d.click("Reload from disk");
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.scroll_until(READER, "Read the watch spell", 12);
    d.scroll_at(READER, 1);
    d.wait_for_text("Read the watch spell");
    d.click("Read the watch spell");
    d.wait_for_text("Watch Ashfall");
    d.expect_same("Restoring the moved package preserves the same linked reader content.");
    d.state("package-folder-restored-with-links", Sizes::All);
    let package = moved;
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("My moved draft.\n");
    let old = package.join("Rules/Ash Exposure.md");
    let moved = package.join("Moved/Ash Exposure.md");
    fs::create_dir_all(moved.parent().unwrap()).unwrap();
    fs::rename(&old, &moved).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Save my draft to Moved/Ash Exposure.md");
    d.observe_both("W8", "The old folder follows Show file on its line", |d| {
        d.place_of("Show file")
            .zip(d.place_of("moved from Rules/"))
            .is_some_and(|(file, from)| {
                from.left() >= file.right()
                    && (file.center().y - from.center().y).abs() < crate::design::XS
            })
    });
    d.state("draft-file-moved-on-disk", Sizes::All);
    let before_save = fs::read_to_string(&moved).unwrap();
    d.click("Save my draft to Moved/Ash Exposure.md");
    d.wait_for_text("Saved");
    assert!(!old.exists());
    let saved = fs::read_to_string(&moved).unwrap();
    assert!(saved.contains("My moved draft."));
    let id = crate::content_workspace::note_id(&saved).unwrap();
    let (entries, errors) = splat::content::read_directory(&package).unwrap();
    assert!(errors.is_empty());
    assert_eq!(entries.iter().filter(|entry| entry.id == id).count(), 1);
    reveal_editor_passage(d, "My moved draft.");
    d.state("draft-saved-at-moved-path", Sizes::All);
    assert_ne!(saved, before_save);
    assert!(!d.has("Undo save"));
}

pub fn package_about(d: &mut Driver<'_>) {
    go_library(d);
    d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
    d.click(MODULE);
    d.click("About Ashen Marches");
    d.wait_for_text("Used here · 5 of 6 entries");
    d.observe(
        "W2 fixture",
        d.visible_text("Used here · 5 of 6 entries")
            && !d.visible_text("Engine support")
            && !d.visible_text("Library / Other entries"),
        "Four selected entries and one required entry count as package use",
    );
    d.state("package-about", Sizes::All);
    d.click("Use every entry");
    d.wait_for_text("All entries used here");
    d.state("package-every-entry-used", Sizes::All);
    d.click("Undo package use");
    d.wait_for_text("Used here · 5 of 6 entries");
    d.expect_same("Undo package use restores the original package selection and About reader.");
    d.state("package-use-undone", Sizes::All);
    d.click(EDIT_MARKDOWN);
    let original = d.editor_text().unwrap();
    let renamed = crate::content_workspace::patch_fields(
        &original,
        &[("splat_name", Some("Emberfall Watch".into()))],
    )
    .unwrap();
    d.replace_text_in(
        "Markdown editor",
        &format!("{renamed}\nA package for the watch at Emberfall.\n"),
    );
    d.click("Save");
    d.click("Read");
    d.wait_for_text("A package for the watch at Emberfall.");
    d.state("package-about-edited", Sizes::All);
    go_campaign(d);
    d.wait_for_text("Emberfall Watch");
    d.state("renamed-package-in-campaign", Sizes::All);
}

/// External edits refresh all catalogue users; a draft still needs a choice.
pub fn package_manifest_broken(d: &mut Driver<'_>) {
    let folder = d.root.join("modules/ashen-marches");
    let path = folder.join("Package.md");
    fs::rename(folder.join("Module.md"), &path).unwrap();
    let valid = fs::read_to_string(&path).unwrap();
    fs::write(&path, valid.replace("splat_kind: package", "splat_kind: [")).unwrap();
    go_library(d);
    open_local_package(d, MODULE);
    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    d.expect("22 (4d8 + 4)");
    d.state("manifest-invalid-library-keeps-entries", Sizes::All);
    go_campaign(d);
    d.click("Reload from disk");
    d.expect("Package.md cannot be read (line 4)");
    assert!(!d.has("Find the folder…"));
    d.state("manifest-invalid-campaign", Sizes::All);
    d.click("Open in Library");
    d.expect("Markdown editor");
    d.expect("Line 4 opens a list");
    d.expect("Replace with a basic Package.md");
    d.expect("Keeps the id, name and namespace. Drops the rest.");
    d.state("manifest-opened-for-repair", Sizes::All);
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    d.expect("Cinder Wolf");
    assert!(!d.has_containing("Not found"));
    d.state("manifest-invalid-encounter-keeps-creatures", Sizes::All);
    fs::write(&path, &valid).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_campaign(d);
    d.click("Reload from disk");
    assert!(!d.has_containing("Package.md cannot be read"));
    d.state("manifest-repaired", Sizes::All);
    fs::remove_file(&path).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.click("Reload from disk");
    d.expect("Package.md was deleted");
    assert!(!d.has("Find the folder…"));
    d.state("manifest-deleted-campaign", Sizes::All);
    d.click("Write Package.md again");
    d.settle();
    assert!(!d.has_containing("Package.md was deleted"));
    assert_eq!(
        crate::library::local_module(&folder).unwrap().id,
        crate::content_workspace::note_id(&valid).unwrap()
    );
    d.expect("Package.md was written again.");
    d.expect("Undo write Package.md");
    d.state("manifest-written-again-campaign", Sizes::All);
    d.click("Undo write Package.md");
    d.wait_for_text("Package.md was deleted");
    assert!(!path.exists());
    d.click("Write Package.md again");
    d.settle();
    fs::remove_file(&path).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    d.expect("22 (4d8 + 4)");
    d.state("manifest-deleted-library-keeps-entries", Sizes::All);
    search_library(d, "Package.md");
    d.click_starting("Package.md · ");
    d.expect("Write Package.md again");
    d.state("manifest-deleted-library-repair", Sizes::All);
    d.click("Write Package.md again");
    d.settle();
    assert!(!d.has_containing("Package.md was deleted"));
    d.expect("Package.md was written again.");
    d.expect("Undo write Package.md");
    d.state("manifest-written-again-library", Sizes::All);
    go_encounters(d);
    d.expect("Cinder Wolf");
    assert!(!d.has_containing("Not found"));
    d.expect_same("Deleting the manifest keeps the encounter's creature entries as before.");
    d.state("manifest-restored-encounter-keeps-creatures", Sizes::All);
}

pub fn package_file_changed(d: &mut Driver<'_>) {
    let path = d
        .root
        .join("modules/ashen-marches/Creatures/Cinder Wolf.md");
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.click_starting("Cinder Wolf · ");
    d.wait_for_text("Bite");
    d.state("package-before-disk-edit", Sizes::All);
    let before = fs::read_to_string(&path).unwrap();
    let changed = before
        .replace("max_hp: {value: 22}", "max_hp: {value: 28}")
        .replace(
            "hit_points_text: '22 (4d8 + 4)'",
            "hit_points_text: '28 (4d8 + 10)'",
        )
        .replace("HP 22 (4d8 + 4)", "HP 28 (4d8 + 10)")
        .replace("Bite.", "Cinder Bite.");
    fs::write(&path, &changed).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Cinder Bite");
    d.state("package-edit-round-trip", Sizes::All);
    look_up(d, "Cinder Wolf");
    d.wait_for_text("Cinder Bite");
    d.state("changed-package-in-lookup", Sizes::All);
    d.press(Key::Escape);
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("The draft stays safe.\n");
    fs::write(&path, changed.replace("Cinder Bite.", "Ash Bite.")).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("This entry changed on disk. Your draft is safe until you choose.");
    d.expect("Overwrite with my draft");
    d.state("package-draft-changed-on-disk", Sizes::All);
    d.click("Reload and lose my draft");
    d.click("Read");
    d.wait_for_text("Ash Bite");
    d.state("package-disk-version-kept", Sizes::All);

    d.click("What the Library shows: Can use");
    d.click("Ashen Marches");
    let valid = fs::read_to_string(&path).unwrap();
    let broken = valid.replace("splat_kind: creature", "splat_kind: [");
    assert_ne!(broken, valid);
    fs::write(&path, &broken).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Cannot be read");
    d.state("package-yaml-broken-on-disk", Sizes::All);
    d.click(EDIT_MARKDOWN);
    d.expect("Save");
    assert!(
        !d.has_containing("Undo discard"),
        "a later disk edit clears the discarded draft"
    );
    let visible_frame = |d: &Driver<'_>| {
        d.input_frame("Markdown editor")
            .is_some_and(|(frame, clip)| clip.expand(1.0).contains_rect(frame))
    };
    d.observe_both(
        "AD2",
        "All source frame edges remain visible",
        visible_frame,
    );
    assert!(d.at_size((1920.0, 1080.0), visible_frame));
    d.state("package-yaml-repair-editor", Sizes::All);
    let before_scroll = d.input_frame("Markdown editor").unwrap().0;
    d.scroll_at(before_scroll.center(), 3);
    assert_eq!(
        d.input_frame("Markdown editor").unwrap().0,
        before_scroll,
        "Only the source text scrolls"
    );
    d.replace_editor_text(&valid);
    d.click("Save");
    d.click("Read");
    d.wait_for_text("Ash Bite");
    d.expect("Ashen Marches · Used here");
    d.state("package-yaml-repaired", Sizes::All);

    // Keep the original repair proof at step 08; repeat the break for its consumers.
    fs::write(&path, &broken).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    d.expect("Cinder Wolf.md cannot be read (line 4)");
    assert!(!d.has("Replacement creature"));
    d.state("invalid-package-in-encounter", Sizes::All);
    d.click("More actions for Bridge Ambush");
    d.click("Start another fight");
    d.expect("Cinder Wolf");
    assert!(!d.app_mut().encounters.has_run());
    d.state("invalid-package-blocks-new-fight", Sizes::All);
    d.click("Open in Library");
    d.expect("Markdown editor");
    d.expect("Line 4 opens a list");
    d.state("invalid-package-opened-for-repair", Sizes::All);
    go_encounters(d);
    d.click("Back to encounters");
    resume_run_from_cold_start(d);
    d.click_row("Cinder Wolf 1");
    d.click("Stat block");
    d.expect("Bite");
    d.state("invalid-package-in-paused-run", Sizes::All);
    close_run(d);
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.click_starting("Creatures/Cinder Wolf.md · ");
    d.click(EDIT_MARKDOWN);
    d.replace_editor_text(&valid);
    d.click("Save");
    d.click("Read");
    d.wait_for_text("Ashen Marches · Used here");
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    assert!(!d.has_containing("Not found. Its package entry was deleted."));
    d.state("repaired-package-in-encounter", Sizes::All);
    d.click("Back to encounters");
    resume_run_from_cold_start(d);
    d.click_row("Cinder Wolf 1");
    d.click("Stat block");
    d.expect("Bite");
    d.expect_same("The paused run keeps its saved creature snapshot across a package repair.");
    d.state("repaired-package-in-paused-run", Sizes::All);
}

/// Deleting a file leaves its uses visible and its reader link unresolved.
pub fn package_delete_entry(d: &mut Driver<'_>) {
    let package = d.root.join("modules/ashen-marches");
    let rules = package.join("Rules/Ash Exposure.md");
    fs::write(
        &rules,
        format!(
            "{}\n\n[[Cinder Wolf]]\n",
            fs::read_to_string(&rules).unwrap()
        ),
    )
    .unwrap();
    go_library(d);
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.wait_for_text("Cinder Wolf");
    d.state("package-entry-before-delete", Sizes::All);
    fs::remove_file(package.join("Creatures/Cinder Wolf.md")).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    d.wait_for_text("Not found. Its package entry was deleted.");
    d.observe(
        "Missing creature names",
        d.has_containing("Cinder Wolf") && !d.has_containing("actor_wolf_"),
        "Deleted entries keep their last known name and hide reference keys",
    );
    d.state("deleted-package-entry-in-encounter", Sizes::All);
    d.click("More actions for Bridge Ambush");
    d.click("Start another fight");
    assert!(!d.app_mut().encounters.has_run());
    d.state("deleted-package-blocks-new-fight", Sizes::All);
    go_library(d);
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.wait_for_text("Cinder Wolf");
    let unresolved = d
        .app_mut()
        .lookup
        .index()
        .unwrap()
        .reader_markdown("[[Cinder Wolf]]", None)
        .contains("splat-missing:");
    d.observe(
        "Deleted package link",
        unresolved,
        "The deleted creature's words remain without a working link",
    );
    d.expect_same(
        "The note keeps its words and layout; only the deleted creature link becomes unavailable.",
    );
    d.state("deleted-package-entry-reader-link", Sizes::All);
    go_encounters(d);
    d.click_beside(
        "6. Cinder Wolf",
        "Options: starting HP, temporary HP, initiative group and changes",
    );
    d.expect("Replacement creature");
    d.expect("Replace all 2 Cinder Wolf rows");
    d.type_into("Replacement creature", "Guard");
    d.expect("Use Guard");
    d.state("deleted-package-entry-choose-replacement", Sizes::All);
    d.click("Use Guard");
    d.expect("as\u{a0}Guard");
    d.expect("Undo replacement");
    assert!(!d.has_containing("Not found. Its package entry was deleted."));
    assert!(
        !d.has_containing("have no creature")
            && !d.has_containing("Choose a replacement in the roster.")
    );
    let dirty = d.app_mut().encounters.draft_dirty();
    d.observe(
        "Replace deleted creature",
        dirty,
        "Choosing a replacement edits the draft; it does not save it",
    );
    d.state("deleted-package-entry-replaced", Sizes::All);
    d.observe_both("W8", "Changed keeps the same roster row pitch", |d| {
        let top = |name: &str| {
            d.nodes()
                .into_iter()
                .find(|node| node.name.starts_with(name))
                .unwrap()
                .rect
                .top()
        };
        let pitch = top("6. Cinder Wolf") - top("5. Sable Harrowgate");
        (top("7. Cinder Wolf") - top("6. Cinder Wolf") - pitch).abs() < 1.0
            && (top("8. Ember Cultist") - top("7. Cinder Wolf") - pitch).abs() < 1.0
    });
    d.click("More actions for Bridge Ambush");
    d.click("Start another fight");
    d.expect("Begin combat");
    d.state("replacement-starts-another-fight", Sizes::All);
    close_run(d);
    go_encounters(d);
    d.click("Undo replacement");
    d.expect("Not found. Its package entry was deleted.");
    d.expect_same("Undo replacement restores the original missing-creature rows and warning.");
    d.state("deleted-package-entry-replacement-undone", Sizes::All);
    go_library(d);
    search_library(d, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("The draft survives a deleted file.\n");
    fs::remove_file(&rules).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text(
        "This entry's file has gone from disk. Your draft is safe here until you choose.",
    );
    d.expect("Ember Cultist needs this entry.");
    d.state("draft-file-deleted-on-disk", Sizes::All);
    d.expect("YOUR DRAFT");
    d.click("Save my draft again");
    d.wait_for_text("Saved");
    reveal_editor_passage(d, "The draft survives a deleted file.");
    assert!(
        fs::read_to_string(&rules)
            .unwrap()
            .contains("The draft survives a deleted file.")
    );
    d.state("deleted-draft-file-saved", Sizes::All);
    d.click("Read");
    d.expect("The draft survives a deleted file.");
    d.click(EDIT_MARKDOWN);
    d.type_into_editor("A draft to let go.\n");
    fs::remove_file(&rules).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Let it go");
    d.click("Let it go");
    d.wait_for_text("No entry open");
    d.expect("Ash Exposure let go. Ember Cultist now needs a missing entry.");
    assert!(!d.has_containing("Ash Exposure · "));
    d.expect("Undo let go");
    d.state("deleted-draft-let-go", Sizes::All);
    d.click("Undo let go");
    d.expect("Let it go");
    d.expect("A draft to let go.");
    d.state("deleted-draft-restored", Sizes::All);
    d.click("Let it go");
    d.wait_for_text("No entry open");
    d.focus_window(false);
    d.focus_window(true);
    d.expect("No entry open");
    assert!(!d.has_containing("Ash Exposure · "));
    d.expect_same("The deleted entry stays absent after focus returns.");
    d.state("deleted-draft-after-focus", Sizes::All);
}

/// A copied file keeps its own save target until the DM repairs its identity.
pub fn package_duplicate_id(d: &mut Driver<'_>) {
    let package = d.root.join("modules/ashen-marches");
    let original = package.join("Creatures/Cinder Wolf.md");
    let copy = package.join("Creatures/Cinder Wolf copy.md");
    let before = fs::read_to_string(&original).unwrap();
    let original_id = crate::content_workspace::note_id(&before);
    let copy_text = before
        .replace("splat_name: Cinder Wolf", "splat_name: Cinder Wolf copy")
        .replace("max_hp: {value: 22}", "max_hp: {value: 44}")
        .replace("HP 22 (", "HP 44 (")
        .replace("| Hit Points | 22 (", "| Hit Points | 44 (");
    fs::write(&copy, &copy_text).unwrap();
    go_library(d);
    search_library(d, "Cinder Wolf");
    d.wait_for_text("Duplicate identity");
    d.state("duplicate-package-library-list", Sizes::All);
    d.click_starting("Cinder Wolf · ");
    d.expect("Make Cinder Wolf copy.md a separate entry");
    d.click(EDIT_MARKDOWN);
    d.state("duplicate-original-editor", Sizes::All);
    d.type_into_editor("Saved only to the original.\n");
    d.click("Save");
    d.wait_for_text("Saved");
    assert!(
        fs::read_to_string(&original)
            .unwrap()
            .contains("Saved only to the original.")
    );
    assert_eq!(fs::read_to_string(&copy).unwrap(), copy_text);
    go_encounters(d);
    open_saved_encounter(d, "Bridge Ambush");
    d.expect("Cinder Wolf copy.md and Cinder Wolf.md share an identity. Choose one in Library.");
    d.state("duplicate-package-in-roster", Sizes::All);
    d.click("Open in Library");
    d.expect("Make Cinder Wolf copy.md a separate entry");
    assert_eq!(
        d.app_mut().library.selected_name(),
        Some("Cinder Wolf copy")
    );
    d.click("Read");
    d.state("duplicate-copy-reader", Sizes::All);
    d.expect("Creature · CR 1");
    d.click("Make Cinder Wolf copy.md a separate entry");
    d.wait_for_text("Undo separate entry");
    d.expect("Cinder Wolf copy.md is now a separate entry.");
    assert_eq!(
        crate::content_workspace::note_id(&fs::read_to_string(&original).unwrap()),
        original_id
    );
    let copy_id = crate::content_workspace::note_id(&fs::read_to_string(&copy).unwrap());
    assert_ne!(original_id, copy_id);
    d.state("duplicate-copy-new-identity", Sizes::All);
    d.click("Undo separate entry");
    d.wait_for_text("Duplicate identity");
    assert_eq!(
        crate::content_workspace::note_id(&fs::read_to_string(&copy).unwrap()),
        original_id
    );
    d.expect_same("Undo restores both identity warnings and the original copy reader.");
    d.state("duplicate-identity-restored", Sizes::All);
    d.click("Make Cinder Wolf copy.md a separate entry");
    d.wait_for_text("Undo separate entry");
    d.expect("Cinder Wolf copy.md is now a separate entry.");
    d.click("Use in this campaign");
    d.wait_for_text("Ashen Marches · Used here");
    go_encounters(d);
    d.expect("Bridge Ambush");
    d.observe_both(
        "W10",
        "The repaired roster keeps the original wolf stats",
        |d| d.nodes().iter().any(|node| node.name == "HP\u{a0}22"),
    );
    d.state("repaired-encounter-keeps-original-wolves", Sizes::All);
    d.type_into(FIND_CREATURE, "Cinder Wolf");
    d.expect("Cinder Wolf copy");
    d.expect("HP 44");
    d.state("repaired-creatures-are-separate-results", Sizes::All);
}

/// Copy a book creature into a shared package, then change its HP.
pub fn book_to_package(d: &mut Driver<'_>) {
    go_library(d);
    search_library(d, "Aboleth");
    d.click_starting("Aboleth · ");
    d.click(COPY_TO);
    d.click("Destination");
    d.click(&format!("Package · {MODULE}"));
    d.replace_text_in("Copy name", "Watch Aboleth");
    d.type_into("Folder", "Creatures");
    d.wait_for_text("Creatures/Watch Aboleth.md");
    d.state("book-copy-to-package", Sizes::All);
    d.click(CREATE_COPY);
    d.expect("Copied to Ashen Marches as Watch Aboleth.");
    d.click("Edit the copy");
    d.expect("Markdown editor");
    let original = d.editor_text().unwrap();
    assert!(original.contains("> [!quote]- Printed stat block (not read; edit the table above)"));
    let archived = original.replace("HP 150 (20d10 + 40)", "HP 180 (20d10 + 70)");
    assert_ne!(original, archived);
    d.replace_text_in("Markdown editor", &archived);
    d.observe_both("X2", "Archived printed HP is not used or checked", |d| {
        !d.has_containing("Hit Points line says") && !d.has("Use 180")
    });
    reveal_editor_passage(d, "> [!quote]- Printed stat block");
    d.state("book-creature-printed-record", Sizes::All);
    d.click("Read");
    d.scroll_up_at(READER, 30);
    d.expect("150 (20d10 + 40)");
    d.observe_both(
        "X2",
        "The reader uses the table and hides the printed record",
        |d| !d.has_containing("Printed stat block") && !d.has("180 (20d10 + 70)"),
    );
    d.state("book-creature-printed-record-ignored", Sizes::All);
    d.click("Edit Markdown");
    let edited = archived.replace(
        "| Hit Points | 150 (20d10 + 40) |",
        "| Hit Points | 180 (20d10 + 40) |",
    );
    assert_ne!(archived, edited);
    d.replace_text_in("Markdown editor", &edited);
    assert!(!edited.contains("    value:"));
    d.observe_both(
        "W1/C1",
        "A book copy has no false broken-link warning",
        |d| !d.has_containing("broken link"),
    );
    d.click("Save");
    d.click("Read");
    d.expect("Watch Aboleth");
    d.scroll_up_at(READER, 30);
    d.expect("180 (20d10 + 40)");
    assert!(
        fs::read_to_string(
            d.root
                .join("modules/ashen-marches/Creatures/Watch Aboleth.md")
        )
        .unwrap()
        .contains("# Watch Aboleth")
    );
    d.state("book-creature-changed-in-package", Sizes::All);
}

pub fn world_note_moved(d: &mut Driver<'_>) {
    let folder = root(d);
    fs::create_dir_all(folder.join("Locations")).unwrap();
    fs::create_dir_all(folder.join("Other")).unwrap();
    fs::write(
        folder.join("Locations/Watch House.md"),
        "# Watch House\n\nThe original watch house.\n",
    )
    .unwrap();
    fs::copy(
        folder.join("Locations/Watch House.md"),
        folder.join("Other/Watch House.md"),
    )
    .unwrap();
    let npc = entry(
        d,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        "  in: '[[Watch House]]'\n",
    );
    entry(d, "Locations/North Gate.md", "North Gate", "location", "");
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Rook Ambry · ");
    d.click_containing("Watch House · 2 matches");
    d.expect("This link matches more than one note. Choose one.");
    d.state("duplicate-place-chooser-in-world", Sizes::All);
    d.click("Always use Locations/Watch House.md");
    d.state("place-choice-kept-with-undo", Sizes::All);
    d.click("Undo place change");
    d.click_containing("Watch House · 2 matches");
    d.click("Locations/Watch House.md");
    look_up(d, "Rook Ambry");
    d.click_containing("Watch House · 2 matches");
    d.expect("This link matches more than one note. Choose one.");
    d.state("duplicate-place-chooser-in-lookup", Sizes::All);
    d.click("Locations/Watch House.md");
    d.press(Key::Escape);
    fs::remove_file(folder.join("Other/Watch House.md")).unwrap();
    fs::create_dir_all(folder.join("Locations/Moved")).unwrap();
    fs::rename(
        folder.join("Locations/Watch House.md"),
        folder.join("Locations/Moved/Watch House.md"),
    )
    .unwrap();
    fs::rename(
        folder.join("NPCs/Rook Ambry.md"),
        folder.join("Locations/Moved/Rook Ambry.md"),
    )
    .unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.click_containing("Rook Ambry · ");
    d.expect("Watch House");
    d.state("moved-note-keeps-place-in-world", Sizes::All);
    look_up(d, "Rook Ambry");
    d.expect("Watch House");
    d.state("moved-note-keeps-place-in-lookup", Sizes::All);
    d.press(Key::Escape);
    let moved_npc = folder.join("Locations/Moved/Rook Ambry.md");
    let before_change = fs::read_to_string(&moved_npc).unwrap();
    let external = format!(
        "{before_change}\n## Patrol after the move\n\nRook keeps the east gate key in the watch room. The relief patrol leaves at midnight and checks the bridge before it returns to the north gate.\n\nKeep this note from Obsidian when the place changes.\n"
    );
    fs::write(&moved_npc, &external).unwrap();
    d.click("Change In");
    d.type_text("North Gate");
    d.click("North Gate");
    assert!(
        fs::read_to_string(&moved_npc)
            .unwrap()
            .contains("Keep this note from Obsidian")
    );
    assert_eq!(
        crate::content_workspace::note_id(
            &fs::read_to_string(folder.join("Locations/Moved/Rook Ambry.md")).unwrap()
        ),
        Some(npc)
    );
    assert!(!folder.join("NPCs/Rook Ambry.md").exists());
    d.state("change-writes-moved-npc", Sizes::All);
}

pub fn world_note_deleted(d: &mut Driver<'_>) {
    entry(d, "Locations/Watch House.md", "Watch House", "location", "");
    let block_text = crate::library::note_text("Watchkeeper Guard", "creature", None);
    let block = splat::content::Entry::parse("Watchkeeper Guard.md", &block_text).unwrap();
    fs::write(root(d).join("Watchkeeper Guard.md"), block_text).unwrap();
    entry(
        d,
        "NPCs/Watchkeeper.md",
        "Watchkeeper",
        "npc",
        &format!(
            "  in: '[[Watch House]]'\n  creature: {{id: {}, link: '[[Watchkeeper Guard]]'}}\n",
            block.id
        ),
    );
    reload(d);
    d.click(NAV_WORLD);
    d.click_containing("Watchkeeper · ");
    d.wait_for_text("Watch House");
    d.state("npc-before-place-deleted", Sizes::All);
    fs::remove_file(root(d).join("Locations/Watch House.md")).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Watch House · Not found");
    d.state("npc-place-deleted-in-world", Sizes::All);
    look_up(d, "Watchkeeper");
    d.wait_for_text("In Watch House · Not found");
    d.state("npc-place-deleted-in-lookup", Sizes::All);
    d.press(Key::Escape);
    fs::remove_file(root(d).join("Watchkeeper Guard.md")).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.wait_for_text("Watchkeeper Guard · Not found");
    d.type_into("Search the world", "Watchkeeper");
    d.state("npc-stat-block-deleted-in-world", Sizes::All);
    look_up(d, "Watchkeeper");
    d.wait_for_text("Watchkeeper Guard · Not found");
    d.state("npc-stat-block-deleted-in-lookup", Sizes::All);
    d.press(Key::Escape);
    go_encounters(d);
    d.click(NEW_ENCOUNTER);
    d.type_into(FIND_CREATURE, "Watchkeeper");
    d.expect("No creatures match");
    d.state("npc-deleted-stat-block-encounter-search", Sizes::All);
}

/// Copying a typed note must keep both files reachable.
pub fn world_note_duplicated(d: &mut Driver<'_>) {
    let campaign = d.app_mut().campaigns.active_campaign().unwrap();
    let catalogue = crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
    let guard = catalogue
        .entries
        .values()
        .find(|entry| entry.entry.name == "Guard" && entry.entry.kind == "creature")
        .unwrap()
        .entry
        .id;
    let id = entry(
        d,
        "NPCs/Rook Ambry.md",
        "Rook Ambry",
        "npc",
        &format!("  creature: {{id: {guard}}}\n"),
    );
    go_encounters(d);
    d.click("Reload from disk");
    d.click(NEW_ENCOUNTER);
    d.type_text("Rook's watch");
    d.type_into(FIND_CREATURE, "Rook Ambry");
    d.click_beside_exact("Rook Ambry", "Add");
    d.click(SAVE_ENCOUNTER);
    d.wait_for_text("Saved");
    let original = root(d).join("NPCs/Rook Ambry.md");
    let copy = root(d).join("NPCs/Rook Ambry copy.md");
    let before = fs::read_to_string(&original).unwrap();
    fs::copy(&original, &copy).unwrap();
    d.focus_window(false);
    d.focus_window(true);
    d.expect("6. Rook Ambry");
    d.replace_text_in(FIND_CREATURE, "Rook Ambry");
    d.expect("Rook Ambry copy.md and Rook Ambry.md share one ID.");
    d.expect("New identity for Rook Ambry copy.md");
    d.state("copied-world-npc-encounter-keeps-guard", Sizes::All);
    d.click("New identity for Rook Ambry copy.md");
    d.wait_for_text("Undo new identity");
    d.state("duplicate-npc-repaired-from-encounter", Sizes::All);
    d.click("Undo new identity");
    d.wait_for_text("New identity for Rook Ambry copy.md");
    d.click(NAV_WORLD);
    d.click_starting("Rook Ambry.md ·");
    d.expect("New identity for Rook Ambry copy.md");
    d.expect("New identity for Rook Ambry.md");
    d.state("duplicate-world-files-and-repair", Sizes::All);
    look_up(d, "Rook Ambry");
    d.expect("New identity for Rook Ambry copy.md");
    d.state("duplicate-world-files-in-lookup", Sizes::All);
    d.press(Key::Escape);
    d.click("New identity for Rook Ambry copy.md");
    d.wait_for_text("Undo new identity");
    assert_eq!(
        crate::content_workspace::note_id(&fs::read_to_string(&original).unwrap()),
        Some(id)
    );
    assert_ne!(
        crate::content_workspace::note_id(&fs::read_to_string(&copy).unwrap()),
        Some(id)
    );
    assert_eq!(fs::read_to_string(&original).unwrap(), before);
    d.expect("Rook Ambry copy");
    d.state("world-copy-repaired", Sizes::All);
    d.click("Undo new identity");
    d.wait_for_text("New identity for Rook Ambry copy.md");
    assert_eq!(fs::read_to_string(&copy).unwrap(), before);
    d.expect_same("Undo restores the same copied files and identity warning.");
    d.state("world-copy-repair-undone", Sizes::All);
    d.click("New identity for Rook Ambry copy.md");
    d.wait_for_text("Undo new identity");
    let same_name = before.replace("splat_name: Rook Ambry", "splat_name: Rook Ambry copy");
    fs::write(&original, same_name).unwrap();
    reload(d);
    d.expect("Rook Ambry.md");
    d.expect("Rook Ambry copy.md");
    d.state("same-name-npcs-show-file-names", Sizes::All);
}

pub fn world_note_renamed(d: &mut Driver<'_>) {
    entry(d, "NPCs/Rook Ambry.md", "Rook Ambry", "npc", "");
    fs::rename(
        root(d).join("NPCs/Rook Ambry.md"),
        root(d).join("NPCs/Mara Vell.md"),
    )
    .unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click_starting("Rook Ambry ·");
    d.expect("NPCs/Mara Vell.md");
    d.expect("The file is Mara Vell.md, but the name is Rook Ambry.");
    d.state("renamed-npc-keeps-old-name-in-world", Sizes::All);
    look_up(d, "Rook Ambry");
    d.expect("Rook Ambry");
    d.state("renamed-npc-keeps-old-name-in-lookup", Sizes::All);
    d.press(Key::Escape);
    d.click("Use Mara Vell");
    d.wait_for_text("Undo name change");
    d.state("renamed-npc-name-updated", Sizes::All);
    look_up(d, "Mara Vell");
    d.expect("Mara Vell");
    d.state("renamed-npc-new-name-in-lookup", Sizes::All);
    d.press(Key::Escape);
    d.click("Undo name change");
    d.wait_for_text("Use Mara Vell");
}

pub fn package_folder_copied(d: &mut Driver<'_>) {
    fn copy_folder(from: &std::path::Path, to: &std::path::Path) {
        fs::create_dir_all(to).unwrap();
        for file in fs::read_dir(from).unwrap() {
            let file = file.unwrap();
            let target = to.join(file.file_name());
            if file.path().is_dir() {
                copy_folder(&file.path(), &target);
            } else {
                fs::copy(file.path(), target).unwrap();
            }
        }
    }
    let copied = d.root.join("modules/ashen-marches-copy");
    copy_folder(&d.root.join("modules/ashen-marches"), &copied);
    let before = fs::read(copied.join("Module.md")).unwrap();
    go_library(d);
    d.click("Packages");
    d.click("Manage packages…");
    d.type_into("Package folder", &copied.to_string_lossy());
    d.click("Add folder");
    d.expect("This folder is a copy of Ashen Marches.");
    assert_eq!(d.app_mut().campaigns.modules().len(), 1);
    d.state("copied-package-names-duplicate-identity", Sizes::All);
    d.click("Register as a new package");
    d.clear_text_in("New package name");
    d.expect("Type a name first.");
    assert!(d.is_disabled("Register package"));
    d.state("copied-package-needs-a-name", Sizes::All);
    d.replace_text_in("New package name", "Ashen Reaches");
    d.state("copied-package-new-name", Sizes::All);
    d.click("Register Ashen Reaches");
    d.wait_for_text("Undo package registration");
    search_library(d, "Cinder Wolf");
    d.expect("Ashen Marches");
    d.expect("Ashen Reaches");
    assert_eq!(d.app_mut().campaigns.modules().len(), 2);
    d.state("copied-package-registered-alongside-original", Sizes::All);
    go_campaign(d);
    d.expect("Ashen Reaches");
    d.state("campaign-after-copy-registration", Sizes::All);
    go_library(d);
    d.click("Use the copy's entries");
    d.wait_for_text("Undo source change");
    go_encounters(d);
    if d.has("Back to encounters") {
        d.click("Back to encounters");
    }
    open_saved_encounter(d, RUN);
    d.type_into(FIND_CREATURE, "Cinder Wolf");
    d.expect("Ashen Reaches");
    d.state("encounter-uses-copy-with-kept-changes", Sizes::All);
    go_library(d);
    d.click("Undo source change");
    d.click("Undo package registration");
    d.wait_for_text("Ashen Marches");
    assert_eq!(d.app_mut().campaigns.modules().len(), 1);
    assert_eq!(fs::read(copied.join("Module.md")).unwrap(), before);
    d.state("copied-package-registration-undone", Sizes::All);
    let hand_edit = crate::content_workspace::patch_fields(
        std::str::from_utf8(&before).unwrap(),
        &[
            ("splat_id", Some(Uuid::new_v4().to_string().into())),
            ("splat_namespace", Some("manual_copy".into())),
            ("splat_key", Some("manual_copy:library".into())),
        ],
    )
    .unwrap();
    fs::write(copied.join("Module.md"), hand_edit).unwrap();
    let module = crate::library::local_module(&copied).unwrap();
    d.app_mut().campaigns.register_module(module).unwrap();
    d.settle();
    d.wait_for_text("Duplicate identity");
    d.state("manifest-only-hand-repair-keeps-both-files", Sizes::All);
    search_library(d, "Cinder Wolf");
    d.click_containing("· ashen-marches/");
    d.expect("Make Cinder Wolf.md in ashen-marches-copy/ a separate entry");
    d.expect_same("The duplicate identity view already selects this file and names both folders.");
    d.state("same-filenames-name-the-folder", Sizes::All);
    let (copied_entries, _) = splat::content::read_directory(&copied).unwrap();
    let count = copied_entries
        .iter()
        .filter(|entry| entry.kind != "package")
        .count();
    d.click(&format!(
        "Make all {count} files in ashen-marches-copy/ separate entries"
    ));
    d.wait_for_text("Undo separate entries");
    d.expect(&format!(
        "{count} files in ashen-marches-copy/ are now separate entries."
    ));
    d.state("copied-folder-identities-repaired-together", Sizes::All);
    d.click("Undo separate entries");
    d.wait_for_text("Make Cinder Wolf.md in ashen-marches-copy/ a separate entry");
    d.expect_same(
        "Undo restores the same duplicate identities and folder-specific repair actions.",
    );
    d.state("copied-folder-identities-undone", Sizes::All);
    d.click(&format!(
        "Make all {count} files in ashen-marches-copy/ separate entries"
    ));
    let wolf = copied_entries
        .iter()
        .find(|entry| entry.name == "Cinder Wolf")
        .unwrap();
    fs::copy(
        d.root.join("modules/ashen-marches").join(&wolf.path),
        copied.join(&wolf.path),
    )
    .unwrap();
    d.click("Packages");
    d.click("Reload from disk");
    search_library(d, "Cinder Wolf");
    d.click_containing("· ashen-marches/");
    d.expect("Make Cinder Wolf.md in ashen-marches-copy/ a separate entry");
    d.state("one-entry-copied-between-registered-packages", Sizes::All);
    go_encounters(d);
    if d.has("Back to encounters") {
        d.click("Back to encounters");
    }
    open_saved_encounter(d, RUN);
    d.expect("Cinder Wolf");
    d.expect("Ember Cultist");
    d.state(
        "manifest-only-hand-repair-keeps-original-roster",
        Sizes::All,
    );
}
