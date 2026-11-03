//! Desk work in World, including a real disk edit while the app stays open.
use super::*;
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

pub fn broken(d: &mut Driver<'_>) {
    let path = root(d).join("Notes/Broken.md");
    fs::write(&path, "---\nsplat_schema: 1\nsplat_id: [\n---\n").unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.expect("1 note cannot be read");
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
    d.click("Save repaired note");
    d.settle();
    d.observe(
        "W5",
        !d.has("1 note cannot be read") && fs::read_to_string(&path).unwrap() == fixed,
        "Saving repairs the note and removes the warning",
    );
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
    d.click_containing("Rook Ambry · NPC");
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
        d.has("Rook Ambry (NPC)") && d.has("Visitors"),
        "Here and Mentioned in show the people and notes at a place",
    );
    d.state("place-here-and-mentions", Sizes::All);
    d.click_containing("Loop One · Location");
    d.observe(
        "World places",
        d.visible_text("its place is inside itself"),
        "A place loop remains visible and says why",
    );
    d.state("place-loop", Sizes::All);
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
}

pub fn tables(d: &mut Driver<'_>) {
    let body = "# Table Workshop\n\n<!-- splat:rolls key=valid dice=1d6 -->\n\n| d6 | Result |\n|---|---|\n| 1 | Rain |\n| 2-5 | Fog |\n| 6 | Sun |\n\n<!-- splat:rolls key=gap dice=1d4 -->\n\n| d4 | Gap |\n|---|---|\n| 1 | One |\n| 3-4 | Later |\n\n<!-- splat:rolls key=overlap dice=1d4 -->\n\n| d4 | Overlap |\n|---|---|\n| 1-3 | First |\n| 3-4 | Second |\n\n<!-- splat:rolls key=words dice=1d6 -->\n\n| d6 | Words |\n|---|---|\n| 1 | One |\n| 2 to 5 | Fog |\n| 6 | Six |\n";
    fs::write(root(d).join("Table Workshop.md"), body).unwrap();
    reload(d);
    d.click(NAV_WORLD);
    d.click("Table Workshop · ");
    d.state("authored-table-validation", Sizes::All);
    d.type_into("Typed roll on 1d6", "4");
    d.press(Key::Enter);
    d.observe(
        "World table author",
        d.visible_text("Fog") && d.visible_text("Typed 4"),
        "A valid range accepts a typed roll and shows the matched row",
    );
    d.state("authored-table-typed-roll", Sizes::All);
    d.scroll_at(READER, 5);
    d.state("authored-table-gap-and-overlap", Sizes::All);
    d.observe(
        "World table author",
        d.visible_text("overlap") || d.visible_text("gap"),
        "An invalid table says what needs an edit",
    );
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
        d.visible_text("npc-name: Names.md"),
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
        d.visible_text("cannot be used") && d.visible_text("Built-in names"),
        "A bad campaign table states why the built-in list stands in",
    );
    d.state("generator-bad-table", Sizes::All);
}
