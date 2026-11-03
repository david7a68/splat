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
    fs::write(path, format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n  type: {kind}\n{fields}---\n\n# {name}\n\nA note for tonight.\n")).unwrap();
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
    fs::write(&path, "---\nsplat_schema: 1\nsplat_id: [\n---\n").unwrap();
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
        d.visible_text("line 3 opens a list") && d.has("Edit in Obsidian"),
        "The broken note opens the shared repair editor and its file actions",
    );
    d.state("broken-note-repair", Sizes::All);
    let fixed = format!(
        "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: world\nsplat_name: Repaired watchkeeper\nsplat_world: {{type: npc}}\n---\n\nThe watchkeeper is back.\n",
        splat::character::stable_id("repaired-watchkeeper")
    );
    d.replace_text_in("Note text", &fixed);
    d.click("Setting · ");
    d.click_containing("Notes/Broken.md");
    d.observe(
        "W5",
        d.editor_text().as_deref() == Some(&fixed),
        "Reading another note keeps the repair draft",
    );
    d.click("Save repaired note");
    d.settle();
    d.observe(
        "W5",
        !d.has("1 note cannot be read") && fs::read_to_string(&path).unwrap() == fixed,
        "Saving repairs the note and removes the warning",
    );
    d.wait_for_text("Note repaired.");
    d.expect("The watchkeeper is back.");
    d.state("broken-note-repaired", Sizes::Default);

    fs::write(&path, "---\nsplat_schema: 1\nsplat_id: [\n---\n").unwrap();
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
    let before = d.interaction_count();
    d.click("Change Stat block");
    d.type_text("Guard");
    d.click("Guard");
    d.observe(
        "W3",
        d.interaction_count() - before <= 3,
        "Give an NPC a stat block in three interactions",
    );
    d.expect("Stat block changed.");
    assert!(
        fs::read_to_string(root(d).join("NPCs/Rook Ambry.md"))
            .unwrap()
            .contains("creature:")
    );
    d.expect_same(
        "Adding the stat block changes only its short link and the confirmation message.",
    );
    d.state("npc-stat-block-changed", Sizes::All);
    d.click("Undo World change");
    d.observe(
        "W3",
        !fs::read_to_string(root(d).join("NPCs/Rook Ambry.md"))
            .unwrap()
            .contains("creature:"),
        "Undo removes only the new stat block key",
    );
    d.click("Change In");
    d.click("New location…");
    d.type_into("Location name", "Watch House");
    d.click("Create location here");
    d.expect("Place changed.");
    d.state("npc-new-place", Sizes::All);
    d.click("Undo World change");
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
    d.expect("Place changed.");
    d.state("place-change-keeps-disk-edit", Sizes::All);
    d.click("Undo World change");
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
    d.click("Table Workshop · ");
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
    d.wait_for_text("Fights as");
    d.expect("Watch House");
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
    let original = fs::read_to_string(&path).unwrap();
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

    let moved = d.root.join("modules/watch-package");
    fs::rename(&package, &moved).unwrap();
    go_campaign(d);
    d.click("Reload from disk");
    d.expect("Find the folder…");
    d.state("package-folder-moved-on-disk", Sizes::All);
    // The native folder picker is closed in test builds. Supply its result here.
    let module = crate::library::local_module(&moved).unwrap();
    d.app_mut().campaigns.restore_module(module).unwrap();
    d.click("Reload from disk");
    go_library(d);
    d.click("Packages");
    d.click("Reload from disk");
    d.type_into(SEARCH_LIBRARY, "Ash Exposure");
    d.click_starting("Ash Exposure · ");
    d.wait_for_text("Read the watch spell");
    d.click("Read the watch spell");
    d.wait_for_text("Watch Ashfall");
    d.expect_same("Restoring the moved package preserves the same linked reader content.");
    d.state("package-folder-restored-with-links", Sizes::All);
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
    d.wait_for_text("Invalid note");
    d.state("package-yaml-broken-on-disk", Sizes::All);
    d.click(EDIT_MARKDOWN);
    d.expect("Save");
    d.state("package-yaml-repair-editor", Sizes::All);
    d.replace_editor_text(&valid);
    d.click("Save");
    d.click("Read");
    d.wait_for_text("Ash Bite");
    d.state("package-yaml-repaired", Sizes::All);
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
    d.state("deleted-package-entry-reader-link", Sizes::All);
    go_encounters(d);
    d.click_beside(
        "Cinder Wolf",
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
    let dirty = d.app_mut().encounters.draft_dirty();
    d.observe(
        "Replace deleted creature",
        dirty,
        "Choosing a replacement edits the draft; it does not save it",
    );
    d.state("deleted-package-entry-replaced", Sizes::All);
    d.click("Undo replacement");
    d.expect("Not found. Its package entry was deleted.");
    d.expect_same("Undo replacement restores the original missing-creature rows and warning.");
    d.state("deleted-package-entry-replacement-undone", Sizes::All);
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
    let changed = original.replace("150 (20d10 + 40)", "180 (20d10 + 70)");
    assert_ne!(original, changed);
    d.replace_text_in("Markdown editor", &changed);
    d.expect("Hit Points line says 180, but max_hp is 150. The fight uses max_hp.");
    reveal_editor_passage(d, "HP 180 (20d10 + 70)");
    d.state("book-creature-stat-check", Sizes::All);
    d.click("Use 180");
    d.expect("Undo stat change");
    reveal_editor_passage(d, "| ` max_hp ` | ` 180 ` |");
    d.state("book-creature-stats-updated", Sizes::All);
    d.click("Undo stat change");
    d.expect("Use 180");
    d.click("Use 180");
    d.click("Save");
    d.click("Read");
    d.expect("Watch Aboleth");
    d.scroll_up_at(READER, 30);
    d.expect("180 (20d10 + 70)");
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
            "  in: '[[Watch House]]'\n  creature: {{id: {}}}\n",
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
    d.wait_for_text("Stat block not found");
    d.type_into("Search the world", "Watchkeeper");
    d.state("npc-stat-block-deleted-in-world", Sizes::All);
    look_up(d, "Watchkeeper");
    d.wait_for_text("Stat block not found");
    d.state("npc-stat-block-deleted-in-lookup", Sizes::All);
    d.press(Key::Escape);
    go_encounters(d);
    d.click(NEW_ENCOUNTER);
    d.type_into(FIND_CREATURE, "Watchkeeper");
    d.expect("No creatures match");
    d.state("npc-deleted-stat-block-encounter-search", Sizes::All);
}
