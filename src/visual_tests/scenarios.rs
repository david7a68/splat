//! Named usage patterns, scripted step by step as a user works. Goals carry the
//! test IDs from `docs/ux-intent.md` section 8.
//!
//! A redesign can rename or move controls. Each scenario keeps its own labels in
//! its first lines, and the labels several share are in `steps.rs`, so a script
//! says what the user does and a rename changes one line.
use super::{
    driver::{DEFAULT, Driver, LARGE, SMALL, Sizes},
    fixture::{CAMPAIGN, MODULE, QUIET_CAMPAIGN, RUN, RUN_12, RUN_20, UNLINKED_CAMPAIGN},
    steps::*,
};
use eframe::egui::{self, Key, Pos2, pos2};

/// The middle of the reading pane and of the entry list, at the minimum window.
const READER: Pos2 = pos2(720.0, 400.0);
const ENTRY_LIST: Pos2 = pos2(400.0, 400.0);

fn escape_closes(d: &mut Driver<'_>, view: &str, still_open: &str, close_with: &str) {
    d.press(Key::Escape);
    let closed = !d.has_containing(still_open);
    d.keyboard_result(
        "escape-closes",
        view,
        closed,
        format!(
            "Escape {} it",
            if closed { "closes" } else { "does not close" }
        ),
    );
    if !closed {
        d.click(close_with);
    }
}

/// W6: find content in the built-in library, read it, and return to the list.
pub mod library_browse {
    use super::*;
    const ENTRY_COUNT: &str = "2065 entries";
    const SEARCH_RESULTS: &str = "Search results";
    const CREATURE: &str = "Adult Black Dragon";
    const SPELL_ROW: &str = "Fireball · ";
    const CLASS_ROW: &str = "Wizard · Class";
    const CREATURES_SECTION: &str = "Creatures   ";
    const CREATURES_COUNT: &str = "336 entries";
    const NEXT_CREATURE: &str = "Aboleth";
    const BACK: &str = "Back";
    const LINKED_QUERY: &str = "bard subclass";
    const LINKED_ROW: &str = "Level 3: Bard Subclass";
    const LINKED_TITLE: &str = "Bard Subclass";
    const LINK: &str = "College of Lore";

    pub fn run(d: &mut Driver<'_>) {
        d.expect(ENTRY_COUNT);
        d.state("first-launch", Sizes::AllAndHiDpi);
        d.check_tab_order("library, first launch");
        d.begin_task();

        search_library(d, "dragon");
        d.expect(SEARCH_RESULTS);
        d.state("search-dragon", Sizes::All);
        d.goal("search for dragons");

        d.click_containing(CREATURE);
        d.expect(CREATURE);
        d.state("creature-stat-block", Sizes::AllAndHiDpi);
        d.goal("open a creature stat block");
        d.review(
            "T6.4",
            "the Library reader, to compare with a campaign entry and a run",
        );

        search_library(d, "fireball");
        d.click_starting(SPELL_ROW);
        d.state("spell", Sizes::All);
        d.goal("open a spell");

        search_library(d, "wizard");
        d.scroll_until(ENTRY_LIST, CLASS_ROW, 80);
        d.click_containing(CLASS_ROW);
        d.state("class-traits-and-features", Sizes::All);
        d.goal("open a class");

        let default_width = d.at_size(DEFAULT, Driver::longest_visible_line);
        let large_width = d.at_size(LARGE, Driver::longest_visible_line);
        // Zero would mean no text was found, which is no pass.
        let long_entry = (20..=75).contains(&default_width) && (20..=75).contains(&large_width);
        // The same measure beside a page, in the panel's smaller prose.
        d.begin_task();
        look_up(d, "fireball");
        let panel_default = d.at_size(DEFAULT, Driver::longest_visible_line);
        let panel_large = d.at_size(LARGE, Driver::longest_visible_line);
        d.press(Key::Escape);
        d.begin_task();
        d.observe(
            "T6.1",
            long_entry && (20..=75).contains(&panel_default) && (20..=75).contains(&panel_large),
            format!(
                "Longest line, class: {default_width} at 1200x800, {large_width} at 1920x1080. Lookup: {panel_default}, {panel_large}"
            ),
        );

        d.scroll_at(READER, 12);
        d.state("class-scrolled", Sizes::Default);
        d.goal("scroll down a long entry");

        // Follow a link in the reader, then come back in one click.
        d.begin_task();
        search_library(d, LINKED_QUERY);
        d.click_containing(LINKED_ROW);
        d.click(LINK);
        d.state("followed-link", Sizes::Default);
        let offered = d.has(BACK) && !d.is_disabled(BACK);
        let away = !d.in_view(LINKED_TITLE);
        let before = d.interaction_count();
        d.click(BACK);
        let cost = d.interaction_count() - before;
        d.observe(
            "T6.2",
            offered
                && away
                && cost == 1
                && d.in_view(LINKED_TITLE)
                && d.has_containing(SEARCH_RESULTS),
            "Back after a link returns to the prior entry and its search in 1",
        );
        d.state("back-after-link", Sizes::Default);
        d.begin_task();
        d.not_measured(
            "T6.3",
            "frame times are not stable while scenarios share the CPU",
        );

        d.click(CLEAR_SEARCH);
        d.click_containing(CREATURES_SECTION);
        d.expect(CREATURES_COUNT);
        d.state("creature-list", Sizes::Default);
        d.click_containing(NEXT_CREATURE);
        d.expect(NEXT_CREATURE);
        d.goal("return through the list and open another entry");
    }
}

/// W8: edit a note in a local package, then manage packages and add an entry.
pub mod library_author {
    use super::*;
    const LOCAL_PACKAGE: &str = "Local package";
    const NOTE: &str = "Ash Exposure";
    const NOTE_ROW: &str = "Ash Exposure · ";
    const OTHER_NOTE: &str = "Ashfall";
    const NOTE_FILE: &str = "modules/ashen-marches/Rules/Ash Exposure.md";
    const SAVE: &str = "Save";
    const DISCARD: &str = "Discard changes";
    const DRAFT_WORDS: &str = "Rangers of the ash line are exempt. ";
    const SECOND_WORDS: &str = "Rangers carry spare cloth. ";
    const THIRD_WORDS: &str = "Cloth wears out in a week. ";
    const OTHER_NOTE_TEXT: &str = "Hot ash falls";
    const OVERWRITE: &str = "Overwrite with my draft";
    const RELOAD_DISK: &str = "Reload and lose my draft";
    const SHOW_DISK: &str = "Show disk version";
    const EXTERNAL_STATUS: &str = "Changed on disk";
    const UNDO_DISCARD: &str = "Undo discard";
    const CHANGED_ON_DISK: &str = "This note changed on disk";
    const PACKAGE_ACTIONS: &str = "Packages";
    const MANAGE_PACKAGES: &str = "Manage packages…";
    const MANAGE_TITLE: &str = "Register an existing package folder";
    const CLOSE_WINDOW: &str = "Close the package form";
    const NEW_ENTRY: &str = "New entry";
    const CREATE_ENTRY: &str = "Create entry";
    const NEW_NAME: &str = "Cinder Hound";
    const NEW_NAME_FIELD: &str = "Entry name";

    pub fn run(d: &mut Driver<'_>) {
        // The app opens on Encounters when a campaign is active.
        go_library(d);
        d.begin_task();
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.click(MODULE);
        d.goal("open a local package");
        d.click_starting(NOTE_ROW);
        d.expect(LOCAL_PACKAGE);
        d.state("local-package", Sizes::All);
        d.check_tab_order("library, local package");

        d.expect(NOTE);
        d.observe(
            "Package scope",
            d.has_containing(NOTE) && !d.has_containing("Adult Red Dragon"),
            "Choosing a package limits the catalogue to it. The active campaign stays",
        );
        d.click(EDIT_MARKDOWN);
        d.state("edit-markdown", Sizes::All);
        d.observe(
            "Clean word",
            d.visible_text("No changes"),
            "A clean document says so, in one word, as Encounters and Campaign do",
        );
        d.goal("open a note for editing");

        d.type_into_editor(DRAFT_WORDS);
        d.expect(UNSAVED_NOTE);
        d.state("dirty", Sizes::All);
        d.goal("change the note");

        let other_row_locked = d.is_disabled(OTHER_NOTE);
        d.click_containing(OTHER_NOTE);
        let draft_here =
            |d: &Driver<'_>, words: &str| d.editor_text().is_some_and(|text| text.contains(words));
        d.observe(
            "T8.2",
            !other_row_locked
                && d.visible_text(OTHER_NOTE_TEXT)
                && d.has_containing(UNSAVED_NOTE)
                && draft_here(d, DRAFT_WORDS),
            "Read another entry while a draft is open. Its text opens in Lookup, and the draft's words stay in the editor",
        );
        d.state("read-beside-draft", Sizes::Default);
        d.press(Key::Escape);
        d.begin_task();

        d.press_ctrl(Key::S);
        let saved = !d.has_containing(UNSAVED_NOTE);
        d.keyboard_result(
            "ctrl-s",
            "library, dirty note",
            saved,
            "Ctrl+S saves a dirty note, as artifacts/ui-review.md says".into(),
        );
        d.state("saved-with-ctrl-s", Sizes::Default);
        d.goal("save with the keyboard");

        // External change: the draft stays, and each way out is a control that works.
        d.type_into_editor(SECOND_WORDS);
        let on_disk = d.root.join(NOTE_FILE);
        let edited =
            std::fs::read_to_string(&on_disk).expect("read note") + "\nEdited elsewhere.\n";
        std::fs::write(&on_disk, edited).expect("edit note on disk");
        d.click(SAVE);
        d.observe(
            "T8.3",
            d.has_containing(CHANGED_ON_DISK)
                && d.has(OVERWRITE)
                && d.has(RELOAD_DISK)
                && d.has(SHOW_DISK)
                && d.visible_text(EXTERNAL_STATUS)
                && draft_here(d, SECOND_WORDS),
            "An external change keeps the draft's words, and offers overwrite, reload and the disk version",
        );
        d.state("save-conflict", Sizes::Default);
        d.goal("handle a failed save");
        d.click(SHOW_DISK);
        d.observe(
            "T8.3",
            d.has_containing("Edited elsewhere."),
            "The disk version can be read before choosing",
        );
        d.click(RELOAD_DISK);
        d.observe(
            "T8.3",
            draft_here(d, "Edited elsewhere.")
                && !draft_here(d, SECOND_WORDS)
                && d.has(UNDO_DISCARD),
            "Reloading takes the disk version, and the draft is one undo away",
        );
        d.click(UNDO_DISCARD);
        d.observe(
            "Discard undo",
            draft_here(d, SECOND_WORDS),
            "Undo brings the dropped draft back",
        );
        d.click(SAVE);
        d.observe(
            "T8.3",
            !d.has_containing(CHANGED_ON_DISK)
                && std::fs::read_to_string(&on_disk).is_ok_and(|text| text.contains(SECOND_WORDS)),
            "After the reload, the same draft saves",
        );

        // The same way out when the file changes again: overwrite.
        d.type_into_editor(THIRD_WORDS);
        let edited = std::fs::read_to_string(&on_disk).expect("read note") + "\nEdited again.\n";
        std::fs::write(&on_disk, edited).expect("edit note on disk");
        d.click(SAVE);
        d.click(OVERWRITE);
        let text = std::fs::read_to_string(&on_disk).expect("read note");
        d.observe(
            "T8.3",
            text.contains(THIRD_WORDS) && !text.contains("Edited again."),
            "Overwrite puts the draft on disk in place of the other change",
        );

        // Invalid note: the save is refused, the words stay, and the reason is on screen.
        d.type_into_editor(THIRD_WORDS);
        d.replace_editor_text("no front matter at all");
        d.click(SAVE);
        d.observe(
            "T8.3",
            d.has_containing("Error: ") && draft_here(d, "no front matter at all"),
            "An invalid note keeps the draft and states why",
        );
        d.click(DISCARD);
        d.observe(
            "Discard undo",
            d.has(UNDO_DISCARD) && !draft_here(d, "no front matter at all"),
            "Discard takes one press, and offers undo in place of a dialog",
        );
        d.click(UNDO_DISCARD);
        d.observe(
            "Discard undo",
            draft_here(d, "no front matter at all"),
            "Undo brings back the discarded draft",
        );
        d.click(DISCARD);
        d.goal("try each way out of a failed save");

        d.click(PACKAGE_ACTIONS);
        d.state("package-actions", Sizes::Default);
        d.click(MANAGE_PACKAGES);
        d.expect(MANAGE_TITLE);
        d.state("manage-packages", Sizes::All);
        escape_closes(d, "manage packages form", MANAGE_TITLE, CLOSE_WINDOW);
        d.goal("open and close the package form");

        d.click(NEW_ENTRY);
        d.state("new-entry-form", Sizes::Default);
        d.type_into(NEW_NAME_FIELD, NEW_NAME);
        d.click(CREATE_ENTRY);
        d.expect(NEW_NAME);
        d.state("new-entry-created", Sizes::All);
        d.goal("create a new entry");

        // Name clash: the same name again is refused, and the typed name stays.
        d.type_into(NEW_NAME_FIELD, NEW_NAME);
        d.click(CREATE_ENTRY);
        d.observe(
            "T8.3",
            d.has_containing("already exists")
                && d.field_text(NEW_NAME_FIELD).as_deref() == Some(NEW_NAME),
            "A name clash states the fix, and the typed name stays",
        );
    }
}

/// W9: choose what a campaign uses, then edit a campaign-owned entry.
pub mod campaign_content {
    use super::*;
    const PAGE_TITLE: &str = "entries chosen";
    const MODULE_ROW: &str = "Ashen Marches · ";
    const PARTIAL: &str = "5 of 6";
    const CHANGED: &str = "6 of 6";
    const UNSELECTED_ENTRY: &str = "Ash Wraith";
    const DISCARD_SELECTION: &str = "Discard selection changes";
    const CAMPAIGN_ENTRY: &str = "Captain Varra";
    const SIBLING_ENTRY: &str = "Bridge Ambush";
    const SAVE_ENTRY: &str = "Save";
    const SAVED: &str = "Saved";
    const REQUIRED: &str = "Required dependencies";
    const REQUIRED_BY: &str = "required by";
    const ENTRIES_LISTING: &str = "Entries";
    const OWN_ENTRY_SEARCH: &str = "varra";
    const RELOAD: &str = "Reload content";
    const REPAIR: &str = "Repair invalid note";
    const SAVE_SELECTIONS: &str = "Save selections";
    const BROKEN_NOTE: &str = "campaigns/emberfall/Notes/Broken.md";

    pub fn run(d: &mut Driver<'_>) {
        go_campaign(d);
        d.expect(PAGE_TITLE);
        d.state("packages-tab", Sizes::Default);
        d.check_tab_order("campaign, packages");
        // No click: the brief asks for these on screen, with the built-in package open.
        let reasons = d.visible_text(REQUIRED_BY);
        let counted = d.has_containing(REQUIRED);
        // The list fills its pane, rather than stopping short of the end.
        d.state("required-dependencies", Sizes::All);
        d.click(ENTRIES_LISTING);
        d.observe(
            "T9.1",
            reasons && counted,
            "Required dependencies and their reasons are on screen, with the built-in package open",
        );
        d.begin_task();
        d.click_containing(MODULE_ROW);
        d.expect(PARTIAL);
        d.state("packages-partial-selection", Sizes::All);
        d.goal("see a partial package selection");

        d.click(UNSELECTED_ENTRY);
        d.expect(CHANGED);
        d.state("selection-changed", Sizes::Default);
        d.click(DISCARD_SELECTION);
        d.goal("change and discard a selection");

        d.click(TAB_ENTRIES);
        d.expect(CAMPAIGN_ENTRY);
        d.state("own-entries-in-library", Sizes::All);
        d.begin_task();
        let in_list = |d: &Driver<'_>, name: &str| {
            d.nodes()
                .into_iter()
                .any(|node| node.name.starts_with(name) && node.in_view() && !node.disabled)
        };
        let sibling_before = in_list(d, SIBLING_ENTRY);
        d.type_into(SEARCH_LIBRARY, OWN_ENTRY_SEARCH);
        let found = in_list(d, CAMPAIGN_ENTRY);
        let sibling_after = in_list(d, SIBLING_ENTRY);
        d.observe(
            "T8.4",
            sibling_before && found && !sibling_after,
            format!("Typing finds a campaign-owned entry, and {SIBLING_ENTRY} leaves the list: before {sibling_before}, after {sibling_after}"),
        );
        d.click(CLEAR_SEARCH);
        d.begin_task();
        d.click_containing(&format!("{CAMPAIGN_ENTRY} · Creature"));
        d.state("entry-read", Sizes::All);
        d.review("T6.4", "the campaign entry reader");
        // Read from the reader's own place: the list beside it has numbers of its own.
        let reader = d.painted_from_label(CAMPAIGN_ENTRY);
        let stats = ["ARMOR", "16", "POINTS", "52", "INITIATIVE", "+2", "Medium"]
            .iter()
            .all(|word| reader.split_whitespace().any(|shown| shown == *word));
        d.observe(
            "T6.4",
            stats && !reader.contains("—"),
            format!("A creature the DM wrote reads with its armor class, hit points and initiative, and no empty cells: {reader}"),
        );
        d.click(EDIT_MARKDOWN);
        d.state("entry-edit", Sizes::Default);
        d.goal("open a campaign entry for editing");

        d.type_into_editor("Scarred by the fire at the bridge. ");
        d.state("entry-dirty", Sizes::Default);
        d.click(SAVE_ENTRY);
        d.expect(SAVED);
        d.state("entry-saved", Sizes::Default);
        d.goal("save the campaign entry");

        // A picture opens at full size, over the page, and Escape closes it.
        d.click("Read");
        d.click(CAMPAIGN_ENTRY);
        d.state("picture-full-size", Sizes::Default);
        d.press(Key::Escape);
        d.settle();
        d.expect(EDIT_MARKDOWN);

        std::fs::write(
            d.root.join(BROKEN_NOTE),
            "---\nsplat_schema: 1\nsplat_id: [\n---\n",
        )
        .expect("write an invalid note");
        go_campaign(d);
        d.click(RELOAD);
        d.expect(REPAIR);
        d.click(TAB_PACKAGES);
        d.state("repair-needed", Sizes::Default);
        d.observe(
            "T9.2",
            d.is_disabled(SAVE_SELECTIONS) && d.visible_text("before saving"),
            "Save disabled by a repair states its reason",
        );
        // The repair opens here, with the note's text in it.
        d.click_containing(REPAIR);
        d.state("repair-open", Sizes::Default);
        d.observe(
            "Repair says why",
            d.visible_text("Invalid YAML"),
            "The repair says what is wrong before the DM edits anything",
        );
        d.observe(
            "T9.2",
            d.editor_text()
                .is_some_and(|text| text.contains("splat_id")),
            "The repair is reachable without leaving the page",
        );
    }
}

/// W7: read the party's cached sheets, and the states with nothing to show.
pub mod party {
    use super::*;
    const FIRST_MEMBER: &str = "Brannoch Vale";
    const SECOND_MEMBER: &str = "Iselle Marrowind";
    const OFFLINE: &str = "Not connected";
    const FRESHNESS: &str = "Fetched";
    const PAGES: [(&str, &str); 4] = [
        ("Actions", "page-actions"),
        ("Spells", "page-spells"),
        ("Equipment", "page-equipment"),
        ("Features", "page-features"),
    ];
    const OVERVIEW: &str = "Overview";
    const SOURCE: &str = "Source & parity";
    const COMPARE: [&str; 3] = ["AC 19", "HP 58/62", "Level 6 Fighter"];
    const NO_CHARACTERS: &str = "No local characters yet";
    const NOT_LINKED: &str = "Link this campaign to D&D Beyond";

    pub fn run(d: &mut Driver<'_>) {
        go_party(d);
        d.expect(FIRST_MEMBER);
        d.expect(OFFLINE);
        d.state("list-and-sheet", Sizes::AllAndHiDpi);
        d.check_tab_order("party, character sheet");
        d.goal("see the party and a sheet");
        d.observe(
            "T7.2",
            d.in_view_containing(FRESHNESS),
            "Freshness text is visible",
        );
        let row = d
            .nodes()
            .into_iter()
            .find(|node| node.name.starts_with(FIRST_MEMBER) && node.in_view())
            .map_or_else(String::new, |node| node.name);
        d.observe(
            "Party compare",
            COMPARE.iter().all(|word| row.contains(word)),
            format!("A member's row shows AC, HP and class with nothing opened: {row}"),
        );

        d.click_containing(SECOND_MEMBER);
        d.state("second-character", Sizes::Default);
        d.goal("change character");
        d.scroll_at(pos2(720.0, 500.0), 4);
        d.state("sheet-scrolled", Sizes::Default);
        for (page, state) in PAGES {
            d.click(page);
            d.state(state, Sizes::Default);
            d.observe(
                "T7.2",
                d.in_view_containing(FRESHNESS),
                format!("Freshness text on {page}"),
            );
        }
        d.click(OVERVIEW);
        d.click(SOURCE);
        d.state("source-and-parity", Sizes::Default);
        d.goal("read every sheet page");

        switch_campaign(d, QUIET_CAMPAIGN);
        d.expect(NO_CHARACTERS);
        d.state("no-characters-signed-out", Sizes::All);
        d.goal("see a linked campaign with no characters");

        switch_campaign(d, UNLINKED_CAMPAIGN);
        d.expect(NOT_LINKED);
        d.state("campaign-not-linked", Sizes::All);
        d.goal("see a campaign with no link");

        // A note edited outside Splat stops its save. Force read needs a session, then replaces
        // the note and keeps the edited version in an archive.
        let copies = d.root.join("campaigns/emberfall/Characters/D&D Beyond");
        let note = std::fs::read_dir(&copies)
            .expect("list character copies")
            .map(|file| file.expect("read character copy").path())
            .find(|path| path.is_file() && path.extension().is_some_and(|x| x == "md"))
            .expect("a character copy");
        std::fs::write(&note, EDITED_NOTE).expect("edit the copy on disk");
        switch_campaign(d, CAMPAIGN);
        d.expect(CONNECT_FIRST);
        d.observe(
            "Force read needs a session",
            !d.has(FORCE_READ) && std::fs::read_to_string(&note).is_ok_and(|t| t == EDITED_NOTE),
            "Signed out, the warning says to connect, and the edited note stays",
        );
        d.state("copy-has-local-edits-signed-out", Sizes::Default);

        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        d.settle();
        d.expect(FORCE_READ);
        d.state("copy-has-local-edits", Sizes::Default);
        d.click(FORCE_READ);
        d.settle();
        d.expect(REPLACED);
        d.state("force-read-replaced", Sizes::Default);
        let archive = note.with_extension("md.splat-archive");
        d.observe(
            "Force read replaces and keeps",
            std::fs::read_to_string(&note).is_ok_and(|t| t != EDITED_NOTE && t.starts_with("---"))
                && std::fs::read_to_string(&archive).is_ok_and(|t| t == EDITED_NOTE)
                && !d.has(FORCE_READ),
            "The note holds the sheet, the archive holds the edited words, and the warning is gone",
        );
        d.click(DISMISS);
        d.settle();
        d.observe(
            "Force read report dismisses",
            !d.has_containing(REPLACED),
            "Dismiss removes the report of where the edits went",
        );
    }
    const FORCE_READ: &str = "Force read";
    const CONNECT_FIRST: &str = "Connect to D&D Beyond in Settings to use it.";
    const REPLACED: &str = "Force read replaced local edits";
    const DISMISS: &str = "Dismiss";
    const EDITED_NOTE: &str = "My own words";
}

/// W5: build an encounter from a cold start, then inspect a saved one.
pub mod encounter_build {
    use super::*;
    const NEW_NAME: &str = "Ashfall Crossing";
    const MONSTER: &str = "wolf";
    const BOSS: &str = "cultist";
    const DUPLICATE: &str = "Duplicate";
    const SAVED_NOTE: &str = "Encounter saved.";
    const SAVED_ENCOUNTER: &str = "Bridge Ambush";
    const SAVED_FIRST_SLOT: &str = "1. Brannoch Vale";
    const VIRTUALISED_LIMIT: usize = 100;
    const READ: &str = "Read";
    const RESULT_NAME: &str = "Wolf";
    const OWN_CREATURE: &str = "Cinder Wolf";
    const EDITOR_NAME: &str = "Encounter name";
    const BACK_TO_LIST: &str = "Back to encounters";
    const NO_SUCH_CREATURE: &str = "zzz";
    const SAVED_ROWS: [&str; 8] = [
        "1. Brannoch Vale",
        "2. Iselle Marrowind",
        "3. Kestrel Dunmoor",
        "4. Oren Thistlewick",
        "5. Sable Harrowgate",
        "6. Wolf",
        "7. Wolf",
        "8. Cultist",
    ];

    pub fn run(d: &mut Driver<'_>) {
        // T5.3 measures the whole build, so it starts from the cold start.
        d.begin_task();
        d.expect(SAVED_RUNS);
        d.state("encounter-list", Sizes::All);
        d.check_tab_order("encounters, list");

        // A new draft selects its name, and adding moves focus to the search.
        d.click(NEW_ENCOUNTER);
        d.type_text(NEW_NAME);
        d.state("new-encounter", Sizes::Default);

        d.click(ADD_PARTY);
        d.expect("5. ");
        d.type_text(MONSTER);
        d.state("find-creature", Sizes::Default);
        // The result row for the plain Wolf, not a word anywhere on the page.
        let nodes = d.nodes();
        // The line under its name, read as the DM sees it: in columns.
        let detail = nodes
            .iter()
            .find(|node| node.name == RESULT_NAME && node.in_view())
            .map_or_else(String::new, |name| {
                d.painted_in(eframe::egui::Rect::from_min_max(
                    pos2(name.rect.min.x - 1.0, name.rect.max.y),
                    pos2(name.rect.min.x + 400.0, name.rect.max.y + 18.0),
                ))
            });
        d.observe(
            "T5.1",
            ["CR ", "HP ", "AC "]
                .iter()
                .all(|word| detail.contains(word)),
            format!("The Wolf result shows CR, HP and AC: {detail}"),
        );
        // A campaign's own creature has its challenge rating in its text, not in a field.
        let own = d
            .nodes()
            .iter()
            .find(|node| node.name == OWN_CREATURE && node.in_view())
            .map_or_else(String::new, |name| {
                d.painted_in(eframe::egui::Rect::from_min_max(
                    pos2(name.rect.min.x - 1.0, name.rect.max.y),
                    pos2(name.rect.min.x + 400.0, name.rect.max.y + 18.0),
                ))
            });
        d.observe(
            "T5.1",
            own.contains("CR "),
            format!("A campaign creature's result shows its CR too: {own}"),
        );
        d.click_first(ADD);
        d.expect("6. ");
        d.click_below("6. ", DUPLICATE);
        d.expect("7. ");
        d.type_text(BOSS);
        d.click_first(ADD);
        d.expect("8. ");
        d.state("creatures-added", Sizes::All);
        d.click(SAVE_ENCOUNTER);
        d.expect(SAVED_NOTE);
        d.state("saved", Sizes::Default);
        d.goal_within(
            "T5.3",
            "save a roster of party and monsters from a cold start",
            10,
        );
        d.observe(
            "T5.3",
            SAVED_ROWS.iter().all(|row| d.has_label(row)),
            format!("The saved roster has the 8 rows asked for: {SAVED_ROWS:?}"),
        );

        d.unreached(
            "encounter difficulty and summary",
            "the saved roster, because the app has no difficulty or summary view",
        );
        // Read a full stat block in Lookup, with the editor still open.
        let before = d.interaction_count();
        d.click_first(READ);
        let cost = d.interaction_count() - before;
        let read = d.has_containing("ARMOR CLASS");
        d.state("stat-block-beside-editor", Sizes::All);
        let still_editing = d.has_containing(EDITOR_NAME);
        // The real loop: read, decide, add. The results stay beside what was just read.
        let add_in_view = d.nodes().iter().any(|n| n.name == ADD && n.in_view());
        d.press(Key::Escape);
        d.observe(
            "T5.2",
            cost <= 2 && read && still_editing && add_in_view,
            format!(
                "A full stat block opens in {cost} beside the editor. The draft stays open, and an Add button is in view: {add_in_view}"
            ),
        );
        d.begin_task();

        // A search with no match says so, and what to try.
        d.replace_text_in(FIND_CREATURE, NO_SUCH_CREATURE);
        d.state("no-search-results", Sizes::Default);
        d.observe(
            "T5.1",
            d.visible_text("No creatures match"),
            "A search with no match says so",
        );
        d.begin_task();

        d.clear_text_in(FIND_CREATURE);
        let rows = d.nodes().iter().filter(|node| node.name == ADD).count();
        d.observe(
            "T5.4",
            rows > 0 && rows <= VIRTUALISED_LIMIT,
            format!("{rows} result rows are built each frame with no search text, from about 340"),
        );

        d.click(BACK_TO_LIST);
        d.expect(SAVED_RUNS);
        let nodes = d.nodes();
        let dated = nodes.iter().any(|node| {
            node.name.contains(RUN)
                && node.name.contains("In progress")
                && node.name.contains("Saved ")
        });
        let grouped = nodes
            .iter()
            .position(|node| node.name == RUN.to_uppercase())
            .zip(
                nodes
                    .iter()
                    .position(|node| node.name.contains(RUN) && node.name.contains("Round")),
            )
            .is_some_and(|(group, run)| group < run);
        d.observe(
            "T4.4",
            dated && grouped,
            format!("Saved runs show when they were saved, and their status: {dated}. Grouped under their encounter: {grouped}"),
        );
        d.state("saved-runs", Sizes::Default);

        d.expect(SAVED_ENCOUNTER);
        open_saved_encounter(d, SAVED_ENCOUNTER);
        d.expect(SAVED_FIRST_SLOT);
        d.state("saved-roster", Sizes::All);
        d.goal("open a saved encounter");
    }
}

/// W1: run combat at the table: turn order, hit points and conditions.
pub mod encounter_run {
    use super::*;
    const TARGET: &str = "Ember Cultist";
    const DAMAGE: &str = "7";
    const AFTER_DAMAGE: &str = "HP 26/33";
    const CONDITION: &str = "Frightened";
    const UNDO: &str = "Undo: ";
    const SECOND_ENCOUNTER: &str = "Shrine of Cinders";
    const ROLL_NPCS: &str = "Roll NPC initiative";
    const KEYBOARD_LOOKUP: &str = "bless";
    const ANCHOR_ROW: &str = "Ember Cultist";

    pub fn run(d: &mut Driver<'_>) {
        d.expect(SAVED_RUNS);
        d.state("saved-runs", Sizes::Default);
        d.click_beside(RUN, RESUME);
        d.expect(&format!("{RUN_HEADING}{RUN}"));
        d.expect(CURRENT_TURN);
        d.state("initiative-list", Sizes::AllAndHiDpi);
        d.check_tab_order("encounter run");
        d.goal("resume a run in progress");
        d.review("T1.8", "the initiative list");
        d.review("T4.5", "the live view, to compare with run-setup");

        let anchor = d.row_node(ANCHOR_ROW).map(|row| row.rect.min.y);
        let before = current_participant(d);
        d.click(NEXT_TURN);
        let after = current_participant(d);
        d.observe(
            "T1.1",
            before.is_some() && after.is_some() && before != after,
            format!("Next turn moves the current turn: {before:?} then {after:?}"),
        );
        d.state("next-turn", Sizes::Default);
        d.goal_within("T1.1", "advance the turn", 1);
        let after = d.row_node(ANCHOR_ROW).map(|row| row.rect.min.y);
        d.observe(
            "Stable rows",
            anchor.is_some() && anchor == after,
            format!("Rows hold their places on Next turn: {anchor:?} then {after:?}"),
        );

        d.click_row(TARGET);
        d.state("participant-selected", Sizes::Default);
        d.type_text(DAMAGE);
        d.press(Key::Enter);
        d.observe(
            "T1.2",
            d.row_node(TARGET)
                .is_some_and(|row| row.name.contains(AFTER_DAMAGE))
                && d.row_words(TARGET).contains("26"),
            format!("The row paints the new HP: {}", d.row_words(TARGET)),
        );
        d.observe_both("T1.2", "The damaged row stays in view", |d| {
            d.row_node(TARGET).is_some_and(|row| row.fully_in_view())
        });
        d.state("hp-changed", Sizes::All);
        d.goal_within("T1.2", "damage a named participant who is not current", 3);

        d.click_row(TARGET);
        d.click(CONDITIONS);
        d.state("condition-tab", Sizes::All);
        let clean = !d.row_words(TARGET).contains(CONDITION);
        d.click(CONDITION);
        d.state("condition-added", Sizes::All);
        d.goal_within("T1.5", "add an official condition", 3);
        let row = d.row_words(TARGET);
        d.observe(
            "T1.5",
            clean && row.contains(CONDITION) && row.contains("26"),
            format!("The row gains the condition and keeps its HP. Nothing was typed: {row}"),
        );

        // The words on screen, in full, not only the accessible name.
        let undo_text = format!("{UNDO}{CONDITION} on {TARGET}");
        d.observe(
            "T1.6",
            d.visible_text(&undo_text),
            format!("At 960x640, the button shows in full: {undo_text}"),
        );
        d.click_containing(UNDO);
        d.state("undo", Sizes::Default);
        d.observe(
            "T1.6",
            !d.row_words(TARGET).contains(CONDITION),
            "Undo removes what it named",
        );
        d.goal("undo the last action");

        // Shift+Enter heals, and Enter damages. The Keys tab says so.
        d.click_row(TARGET);
        let hp = d.row_hp(TARGET);
        d.type_text("4");
        d.press(Key::Enter);
        let hurt = d.row_hp(TARGET);
        d.type_text("3");
        d.press_shift(Key::Enter);
        let healed = d.row_hp(TARGET);
        d.observe(
            "T1.7",
            hurt == hp.map(|hp| hp - 4) && healed == hp.map(|hp| hp - 1),
            format!("Enter takes 4, then Shift+Enter gives back 3: {hp:?}, {hurt:?}, {healed:?}"),
        );
        d.press(Key::U);
        d.press(Key::U);

        // A target by key: the dock names the row the amount will act on.
        let selected_before = d.focused_row_name();
        d.press(Key::ArrowUp);
        let target = selected_before.is_some() && selected_before != d.focused_row_name();

        d.begin_task();
        let before = current_participant(d);
        d.press(Key::N);
        let next = current_participant(d) != before;
        d.state("next-turn-by-keyboard", Sizes::Default);
        let hp_before = d
            .nodes()
            .iter()
            .filter(|n| n.name.contains("HP "))
            .map(|n| n.name.clone())
            .collect::<Vec<_>>();
        d.type_text("3");
        d.press(Key::Enter);
        let hp_after = d
            .nodes()
            .iter()
            .filter(|n| n.name.contains("HP "))
            .map(|n| n.name.clone())
            .collect::<Vec<_>>();
        let damage = hp_before != hp_after;
        d.press(Key::U);
        let undone = d
            .nodes()
            .iter()
            .filter(|n| n.name.contains("HP "))
            .map(|n| n.name.clone())
            .collect::<Vec<_>>();
        let undo = undone == hp_before;
        d.press_ctrl(Key::K);
        d.type_text(KEYBOARD_LOOKUP);
        let search = d.has("Bless");
        d.state("lookup-by-keyboard", Sizes::Default);
        d.press(Key::Escape);
        let back = !d.has(SEARCH_LOOKUP);
        let keys = d.goal("run a turn, damage, undo and a lookup by keyboard alone");
        d.observe(
            "T1.7",
            next && target && damage && undo && search && back,
            format!(
                "Direct key commands, {keys} key presses in all: next turn {next}, target {target}, damage {damage}, undo {undo}, lookup {search}, close {back}"
            ),
        );
        d.expect(RUN_HEADING);
        // Tab and Enter also reach the same control.
        let tabbed = d.activate_by_keyboard(NEXT_TURN);
        d.keyboard_result(
            "tab-to-next-turn",
            "encounter run",
            tabbed,
            "Tab and Enter reach Next turn".into(),
        );
        d.begin_task();

        close_run(d);
        d.expect(NAV_ENCOUNTERS);
        run_saved_encounter(d, SECOND_ENCOUNTER);
        d.expect(&format!("{RUN_HEADING}{SECOND_ENCOUNTER}"));
        d.state("run-setup", Sizes::All);
        d.click(ROLL_NPCS);
        d.state("npc-initiative-rolled", Sizes::Default);
        d.goal("start a new run and roll initiative");

        d.click("Roll NPC initiative another way");
        d.state("npc-roll-menu", Sizes::Default);
        d.check_tab_order("NPC roll menu");
        d.click("Individually");
        d.state("npc-initiative-rolled-individually", Sizes::Default);
        d.goal_within(
            "Roll once another way",
            "roll NPC initiative another way",
            2,
        );
    }
}

/// W10: connect to D&D Beyond and manage campaigns.
pub mod settings {
    use super::*;
    use crate::settings::Account;
    const ACCOUNT_WORDS: [(Account, &str); 5] = [
        (Account::Busy, "Connecting"),
        (Account::Connected, "Connected to D&D Beyond"),
        (Account::NotConnected, "Not connected"),
        (Account::Failed, "the last attempt failed"),
        (Account::KeyringWarning, "was not saved"),
    ];
    const REMOVE_CAMPAIGN: &str = "Remove Quiet Harbor from Splat";
    const REMOVE_TITLE: &str = "Remove Quiet Harbor?";
    const NOT_CONNECTED: &str = "Not connected";
    const COOKIE_HELP: &str = "How to get your session cookie";
    const COOKIE_HELP_TEXT: &str = "Sign in to D&D Beyond in your browser";
    const COOKIE_FIELD: &str = "Paste session cookie";
    const SYNTHETIC_COOKIE: &str = "synthetic-not-a-real-cookie";
    const CONNECT: &str = "Connect";
    const DISCONNECT: &str = "Disconnect";
    const OFFLINE_ERROR: &str = "Could not reach D&D Beyond";
    const CAMPAIGNS_SECTION: &str = "Campaigns";
    const EDIT_DETAILS: &str = "Edit details";
    const EDIT_FORM: &str = "Edit campaign details";
    const CANCEL: &str = "Cancel";
    const ADD_FROM_REMOTE: &str = "Add from D&D Beyond";

    pub fn run(d: &mut Driver<'_>) {
        go_settings(d);
        d.expect(NOT_CONNECTED);
        d.state("connect-form", Sizes::AllAndHiDpi);
        d.check_tab_order("settings");
        d.goal("open settings");

        d.click(COOKIE_HELP);
        d.expect(COOKIE_HELP_TEXT);
        d.state("cookie-help", Sizes::Default);
        d.click(COOKIE_HELP);
        d.type_into(COOKIE_FIELD, SYNTHETIC_COOKIE);
        d.state("cookie-entered", Sizes::Default);
        d.click(CONNECT);
        d.expect(OFFLINE_ERROR);
        d.state("connect-failed", Sizes::All);
        d.goal("try to connect without a network");
        // The cookie stays for a retry. There is nothing saved, so nothing to disconnect.
        let retry = !d.is_disabled(CONNECT) && !d.has(DISCONNECT);
        d.observe(
            "T10.3",
            retry,
            "After a failed attempt, Connect can retry at once",
        );
        // Each state through the same fields a real attempt sets.
        let mut words = true;
        for (state, word) in ACCOUNT_WORDS {
            d.app_mut().settings.simulate(state);
            d.settle();
            words &= d.in_view_containing(word);
            // Not connected and failed already have their captures, from real attempts.
            match state {
                Account::Busy => d.state("account-connecting", Sizes::Default),
                Account::Connected => d.state("account-connected", Sizes::Default),
                Account::KeyringWarning => d.state("account-keyring-warning", Sizes::Default),
                Account::NotConnected | Account::Failed | Account::RefreshFailing => {}
            }
        }
        d.observe(
            "T10.3",
            words,
            "Busy, connected, not connected, failed and keyring warning each have words, from a test session",
        );
        d.app_mut().settings.simulate(Account::NotConnected);
        d.settle();
        d.expect(NOT_CONNECTED);

        d.row_of(CAMPAIGNS_SECTION);
        d.state("campaign-list", Sizes::All);
        d.goal("find campaign management");

        d.click(EDIT_DETAILS);
        d.expect(EDIT_FORM);
        d.state("edit-campaign", Sizes::Default);
        d.click(CANCEL);
        d.click(ADD_FROM_REMOTE);
        d.state("add-from-remote", Sizes::Default);
        d.click(REMOVE_CAMPAIGN);
        d.expect(REMOVE_TITLE);
        d.state("confirm-remove-campaign", Sizes::Default);
        d.press(Key::Escape);
        d.goal("edit a campaign's details");
    }
}

/// The copy dialog, and the unsaved-changes prompt when closing.
pub mod dialogs {
    use super::*;
    const CREATURES_SECTION: &str = "Creatures   ";
    const CREATURE: &str = "Aboleth";
    const COPY_TITLE: &str = "Copy entry";
    const CLOSE_WINDOW: &str = "Cancel";
    const NOTE: &str = "Ash Exposure";
    const CLOSE_PROMPT: &str = "has unsaved changes.";
    const KEEP_EDITING: &str = "Keep editing";

    pub fn run(d: &mut Driver<'_>) {
        go_library(d);
        d.begin_task();
        d.click_containing(CREATURES_SECTION);
        d.click_containing(CREATURE);
        d.click(COPY_TO);
        d.expect(COPY_TITLE);
        d.state("copy-dialog", Sizes::All);
        d.goal("open the copy dialog");

        escape_closes(d, "copy dialog", COPY_TITLE, CLOSE_WINDOW);

        d.click(COPY_TO);
        d.observe(
            "Copy default",
            d.visible_text(&format!("Campaign · {CAMPAIGN}")),
            "The dialog starts on the active campaign",
        );
        d.open_combo_at(COPY_DESTINATION_DROPDOWN);
        d.state("copy-destination-menu", Sizes::Default);
        d.click(&format!("Campaign · {CAMPAIGN}"));
        d.click(REVIEW_COPY);
        d.expect("Ready to copy");
        d.state("copy-review", Sizes::All);
        d.observe(
            "Copy review actions",
            d.in_view("Create copy") && d.in_view("Cancel") && !d.has("Review copy"),
            "The review ends in one row of actions",
        );
        d.click(CREATE_COPY);
        d.expect("Saved: ");
        d.state("copy-complete", Sizes::Default);
        d.click(CLOSE_COPY);
        d.goal("copy an entry to a campaign");

        open_local_package(d, MODULE);
        d.click_containing(NOTE);
        d.click(EDIT_MARKDOWN);
        d.type_into_editor("Draft ruling. ");
        d.expect(UNSAVED_NOTE);
        d.request_close();
        d.observe(
            "Close prompt",
            d.has_containing(&format!("\u{201c}{NOTE}\u{201d}"))
                && d.has_containing(MODULE)
                && d.has_containing(CLOSE_PROMPT),
            "The prompt names the entry, and the package it is in",
        );
        d.state("unsaved-changes-on-close", Sizes::All);
        d.goal("ask to close with unsaved changes");

        escape_closes(d, "unsaved changes prompt", CLOSE_PROMPT, KEEP_EDITING);
        d.expect(UNSAVED_NOTE);
        d.state("kept-editing", Sizes::Default);
        d.goal("return to the draft");
    }
}

/// W10: nothing set up yet, then a new campaign's placeholder tabs.
pub mod empty {
    use super::*;
    const NO_CAMPAIGN: &str = "No campaign selected";
    const ADD_CAMPAIGN: &str = "Add campaign";
    const NAME_FIELD: &str = "Campaign name";
    const FOLDER_FIELD: &str = "Folder";
    const OPEN_CAMPAIGN: &str = "Open campaign";
    const NEW_CAMPAIGN: &str = "Ashes of Emberfall";
    const NEW_FOLDER: &str = "new-campaign";
    const NO_ENTRIES: &str = "No campaign entries yet";
    const OPEN_SETTINGS: &str = "Open Settings";
    const NO_CAMPAIGN_ENCOUNTERS: &str = "No campaign to prepare yet";
    const NO_CAMPAIGN_PARTY: &str = "No campaign, so no party yet";
    const NOT_LINKED: &str = "Link this campaign to D&D Beyond";
    const CONTENT_TITLE: &str = "entries chosen";
    const NEXT_STEP: &str = "Next step: add one in Settings";

    pub fn run(d: &mut Driver<'_>) {
        d.observe(
            "T10.1",
            d.has(NAV_LIBRARY) && !d.has_containing("Error") && d.has_containing(NEXT_STEP),
            "The Library works with no campaign, and one next step is stated",
        );
        d.review("T10.1", "first launch shows one next step");
        d.state("first-launch", Sizes::Default);
        go_campaign(d);
        d.expect(NO_CAMPAIGN);
        d.state("no-campaign", Sizes::All);
        d.check_tab_order("campaign, no campaign");
        d.goal("see the no-campaign state");
        d.begin_task();
        let campaign_says = d.has_containing(NO_CAMPAIGN) && d.has(OPEN_SETTINGS);
        go_encounters(d);
        let encounters_says = d.has_containing(NO_CAMPAIGN_ENCOUNTERS) && d.has(OPEN_SETTINGS);
        d.state("encounters-no-campaign", Sizes::Default);
        go_party(d);
        let party_says = d.has_containing(NO_CAMPAIGN_PARTY) && d.has(OPEN_SETTINGS);
        d.state("party-no-campaign", Sizes::Default);
        d.observe(
            "T10.2",
            campaign_says && encounters_says && party_says,
            format!("With no campaign, each page says what to do. Campaign {campaign_says}, Encounters {encounters_says}, Party {party_says}"),
        );
        d.begin_task();

        go_settings(d);
        d.expect(ADD_CAMPAIGN);
        d.state("settings-no-campaigns", Sizes::All);
        let folder = d.root.join(NEW_FOLDER);
        std::fs::create_dir_all(&folder).expect("create campaign folder");
        d.type_into(NAME_FIELD, NEW_CAMPAIGN);
        d.type_into(FOLDER_FIELD, &folder.display().to_string());
        d.click(OPEN_CAMPAIGN);
        d.expect(NEW_CAMPAIGN);
        d.state("campaign-added", Sizes::Default);
        d.goal("add a first campaign");
        d.observe(
            "Campaign added",
            d.has("Build an encounter"),
            "After adding a campaign, the next step is a button",
        );

        go_campaign(d);
        d.expect(CONTENT_TITLE);
        d.click(TAB_ENTRIES);
        d.expect(NO_ENTRIES);
        d.state("no-campaign-entries", Sizes::Default);
        go_encounters(d);
        d.state("no-encounters", Sizes::Default);
        go_party(d);
        d.expect(NOT_LINKED);
        d.state("party-without-link", Sizes::Default);
        d.observe(
            "Party next step",
            d.has("Connect D&D Beyond") && !d.has("Link this campaign"),
            "With no session, the next step is to connect, because linking needs one",
        );
        d.goal("see the other empty tabs");
    }
}

/// W2: look something up during a run, then return. Lookup opens beside the roster,
/// so the run stays on screen and Next turn stays clickable.
pub mod run_lookup {
    use super::*;
    const SECOND_RESULT: &str = "Delayed Blast Fireball";
    const PARTICIPANT: &str = "Ember Cultist";
    const SPELL_QUERY: &str = "fireball";
    const SPELL: &str = "Fireball";
    const PACKAGE_ENTRY: &str = "Ash Exposure";
    const CAMPAIGN_ENTRY: &str = "Captain Varra";
    const SPELL_IN_STAT_BLOCK: &str = "Ashfall";
    const SPELL_BODY: &str = "Hot ash falls";
    const OTHER_NOTE: &str = "Ashfall";
    const NOTE: &str = "Ash Exposure";
    const DRAFT_WORDS: &str = "Unfinished ruling. ";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_row(PARTICIPANT);
        d.state("in-the-run", Sizes::All);
        d.goal("be in a run, with a participant selected");

        look_up(d, SPELL_QUERY);
        d.expect(SPELL);
        d.state("reading-spell-beside-run", Sizes::All);
        d.goal_within("T2.1", "read a spell from a run", 3);
        // In a short panel the text gets the height, and the list is one click away.
        d.observe(
            "T2.1",
            d.has("Show the results") && !d.has_containing(SECOND_RESULT),
            "At 960x640 the best match opens by itself, with its results closed",
        );
        let builtin_found = d.in_view(SPELL);
        d.observe_both("T2.1", "Roster and Next turn stay visible", |d| {
            d.in_view(NEXT_TURN)
                && d.row_node(PARTICIPANT)
                    .is_some_and(|row| row.fully_in_view())
        });
        // A click on a result leaves no field in focus. Typing must not run commands.
        let turn = current_participant(d);
        d.harness_focus_nothing();
        d.press(Key::N);
        d.observe(
            "T2.1",
            current_participant(d) == turn,
            "With Lookup open, the letter N does not advance the turn",
        );
        // Pressed, with Lookup open, and it acts. Then undone, so the script goes on.
        let before = current_participant(d);
        d.click(NEXT_TURN);
        let after = current_participant(d);
        d.observe(
            "T2.1",
            before != after && d.in_view(SPELL),
            format!("Next turn acts with Lookup open: {before:?} then {after:?}"),
        );
        // Next turn put the focus in Amount. A search word typed now is not a command.
        d.press(Key::N);
        let held = current_participant(d);
        d.observe(
            "T2.1",
            held == after,
            format!("With Lookup open, N in Amount does not advance: {after:?} then {held:?}"),
        );
        d.click_containing("Undo: next turn");
        d.click_row(PARTICIPANT);
        d.begin_task();

        look_up_again(d, &PACKAGE_ENTRY.to_lowercase());
        let package_found = d.in_view(PACKAGE_ENTRY);
        look_up_again(d, &CAMPAIGN_ENTRY.to_lowercase());
        let campaign_found = d.in_view(CAMPAIGN_ENTRY);
        d.state("found-package-and-campaign-entries", Sizes::Default);
        d.observe(
            "T2.3",
            builtin_found && package_found && campaign_found,
            format!("Lookup with no package switch: built-in {builtin_found}, package entry {package_found}, campaign entry {campaign_found}"),
        );

        d.begin_task();
        d.press(Key::Escape);
        d.expect(RUN_HEADING);
        d.state("back-in-the-run", Sizes::All);
        d.goal_within("T2.2", "return to the run", 1);
        d.observe(
            "T2.2",
            participant_is_selected(d, PARTICIPANT),
            "The selected participant is unchanged on return",
        );

        d.review("T6.4", "the run's creature reader");
        let linked = d
            .nodes()
            .iter()
            .any(|node| node.is_link() && node.name.contains(SPELL_IN_STAT_BLOCK));
        d.begin_task();
        let unread = !d.visible_text(SPELL_BODY);
        d.click(SPELL_IN_STAT_BLOCK);
        let opened = d.visible_text(SPELL_BODY) && d.in_view(NEXT_TURN);
        d.state("spell-from-stat-block", Sizes::Default);
        d.observe(
            "T2.4",
            linked && unread && opened,
            "A spell named in a run stat block opens its text beside the roster",
        );
        let cost = d.goal("read the creature's details in the run");
        d.observe(
            "T2.4",
            cost <= 1,
            format!("It took {cost}, against a target of 1"),
        );
        d.press(Key::Escape);

        // A draft elsewhere does not block Lookup, and Lookup does not touch the draft.
        go_library(d);
        open_local_package(d, MODULE);
        d.click_containing(NOTE);
        d.click(EDIT_MARKDOWN);
        d.type_into_editor(DRAFT_WORDS);
        d.expect(UNSAVED_NOTE);
        d.begin_task();
        look_up(d, &OTHER_NOTE.to_lowercase());
        d.state("lookup-with-draft-open", Sizes::All);
        d.goal("try a lookup with a draft open");
        d.press(Key::Escape);
        let kept = d
            .editor_text()
            .is_some_and(|text| text.contains(DRAFT_WORDS));
        d.observe(
            "T2.5",
            kept && d.has_containing(UNSAVED_NOTE) && d.in_view(NEXT_TURN),
            "Lookup works while a Library draft is unsaved. The draft's own words are still in the editor, and the run strip stays",
        );
    }
}

/// No fixture has a long name. A long name is never cut: the `clipped` check reads every
/// painted word in these states.
pub mod run_long_names {
    use super::*;
    const RENAMED: &str = "Iselle Marrowind";
    const BACK_TO_RUN: &str = "Back to run";
    const LONG_NAME: &str = "Seraphina Thornwood-Ashgrave of the Ninth Bell";
    const FIRST_NAME: &str = "Seraphina";
    const ENEMY: &str = "Cinder Wolf 1";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        // A Bloodied enemy has its mark and its word side by side: they must not read as one.
        d.click_row(ENEMY);
        d.type_text("5");
        d.press(Key::Enter);
        d.app_mut().encounters.simulate_rename(RENAMED, LONG_NAME);
        d.settle();
        d.click_row(LONG_NAME);
        d.state("long-name", Sizes::All);
        let row = d.row_words(LONG_NAME);
        d.observe(
            "Long names",
            row.contains(LONG_NAME),
            format!("A long name is whole on its row at 960x640: {row}"),
        );
        // The strip names whose turn it is, from any page.
        go_library(d);
        d.state("long-name-on-the-strip", Sizes::Default);
        d.click(BACK_TO_RUN);
        d.begin_task();
        // The header names the current participant first. Two turns on, that is this name.
        d.click(NEXT_TURN);
        d.state("long-name-has-the-turn", Sizes::All);
        let turn = current_participant(d);
        let header = d.header_rect();
        let words = d.painted_in(header);
        let size = d.painted_size(header, FIRST_NAME);
        let row_size = d
            .row_node(LONG_NAME)
            .and_then(|row| d.painted_size(row.shown, LONG_NAME));
        d.observe(
            "Long names",
            turn.as_deref() == Some(LONG_NAME)
                && words.contains(&format!("{FIRST_NAME}’s turn"))
                && size > row_size
                && row_size.is_some(),
            format!(
                "At 960x640 the header says \"{words}\" at {size:?} px, larger than the row's name at {row_size:?} px"
            ),
        );
        go_library(d);
        d.state("long-name-has-the-turn-on-the-strip", Sizes::Default);
        d.not_measured(
            "Long names",
            "outside a run: only the run's rows, header, dock, panel and strip have long-name data",
        );
    }
}

/// A player who did not arrive: combat begins without them, and they can come back.
pub mod run_absent_player {
    use super::*;
    const ENCOUNTER: &str = "Shrine of Cinders";
    const PLAYER: &str = "Brannoch Vale";
    const OTHERS: [(&str, &str); 4] = [
        ("Iselle Marrowind", "19"),
        ("Kestrel Dunmoor", "21"),
        ("Oren Thistlewick", "11"),
        ("Sable Harrowgate", "13"),
    ];
    const ROLL_NPCS: &str = "Roll NPC initiative";
    const BEGIN: &str = "Begin combat";
    const NO_INITIATIVE: &str = "No initiative";
    const MIDDLE: &str = "Oren Thistlewick";
    const LAST: &str = "Sable Harrowgate";
    const TURN_ORDER_FIRST: &str = "Kestrel Dunmoor";

    pub fn run(d: &mut Driver<'_>) {
        run_saved_encounter(d, ENCOUNTER);
        d.click(&format!("Absent: {PLAYER}"));
        d.click(ROLL_NPCS);
        // The keyboard path skips the absent: after the roll, and after each Enter.
        let field = |name: &str| format!("Initiative for {name}");
        let after_roll = d.focused_name();
        d.observe(
            "Absent player",
            after_roll.as_deref() == Some(field("Iselle Marrowind").as_str()),
            format!("After the roll, the first present player's field has focus: {after_roll:?}"),
        );
        d.type_text(OTHERS[0].1);
        d.press(Key::Enter);
        d.type_text(OTHERS[1].1);
        // One more absent, in the middle: Enter goes past that one too.
        d.click(&format!("Absent: {MIDDLE}"));
        d.click(&field(OTHERS[1].0));
        d.press(Key::Enter);
        let past_middle = d.focused_name();
        d.observe(
            "Absent player",
            past_middle.as_deref() == Some(field(LAST).as_str()),
            format!("Enter goes past an absent player in the middle: {past_middle:?}"),
        );
        d.click(&format!("Mark present: {MIDDLE}"));
        for (name, total) in OTHERS {
            if d.field_text(&field(name)).as_deref() != Some(total) {
                d.type_into(&field(name), total);
            }
        }
        d.state("ready-with-one-absent", Sizes::All);
        // The turn order follows the totals, with no place taken by the absent player.
        let turn_order = d.painted_from_label(TURN_ORDER_FIRST);
        d.observe(
            "Absent player",
            turn_order.contains(TURN_ORDER_FIRST),
            format!("The setup's turn order starts with the highest total: {turn_order}"),
        );
        d.observe(
            "Absent player",
            !d.is_disabled(BEGIN) && d.visible_text("Everyone is ready"),
            "Combat can begin with a player absent, and with no initiative for them",
        );
        d.click(BEGIN);
        d.state("began-without-them", Sizes::All);
        let first = current_participant(d);
        d.observe(
            "Absent player",
            first.as_deref() == Some(TURN_ORDER_FIRST) && d.row_words(PLAYER).contains("Absent"),
            format!("The first turn is {first:?}, and their row says Absent"),
        );
        d.begin_task();

        // Back in the second turn: last in the order, and the DM says when it is.
        d.click(NEXT_TURN);
        let turn = current_participant(d);
        d.click_row(PLAYER);
        d.click("Mark present");
        d.state("back-with-no-initiative", Sizes::All);
        let last = roster_places(d).last().map(|row| row.0.clone());
        d.observe(
            "Absent player",
            d.row_words(PLAYER).contains(NO_INITIATIVE)
                && last.as_deref() == Some(PLAYER)
                && current_participant(d) == turn,
            format!(
                "Back with none: the row says so, they are last ({last:?}), and the turn stays"
            ),
        );
        d.click("Record");
        d.type_into(&format!("Initiative for {PLAYER}"), "20");
        d.click("Set initiative");
        d.state("initiative-set-in-combat", Sizes::Default);
        let order: Vec<String> = roster_places(d).into_iter().map(|row| row.0).collect();
        d.observe(
            "Absent player",
            order.first().map(String::as_str) == Some(TURN_ORDER_FIRST)
                && order.get(1).map(String::as_str) == Some(PLAYER)
                && current_participant(d) == turn
                && !d.row_words(PLAYER).contains(NO_INITIATIVE),
            format!("Their place follows the total, and the turn stays: {order:?}"),
        );
        d.begin_task();
    }
}

/// W4: cold start to the first turn of a new run.
pub mod prep_to_table {
    use super::*;
    const ENCOUNTER: &str = "Shrine of Cinders";
    const ROLL_NPCS: &str = "Roll NPC initiative";
    const BEGIN: &str = "Begin combat";
    const ROUND: &str = "Round 1";
    const PLAYERS: [(&str, &str); 5] = [
        ("Brannoch Vale", "15"),
        ("Iselle Marrowind", "19"),
        ("Kestrel Dunmoor", "21"),
        ("Oren Thistlewick", "11"),
        ("Sable Harrowgate", "13"),
    ];

    pub fn run(d: &mut Driver<'_>) {
        run_saved_encounter(d, ENCOUNTER);
        d.expect(&format!("{RUN_HEADING}{ENCOUNTER}"));
        d.state("run-setup", Sizes::All);
        d.goal_within("T4.1", "cold start to run setup", 3);
        d.review("T4.5", "the setup view, to compare with the live view");

        // A player who did not arrive can be marked, in Setup, and marked back.
        d.begin_task();
        let (player, _) = PLAYERS[0];
        let inits = |d: &Driver<'_>| -> Vec<f32> {
            d.nodes()
                .iter()
                .filter(|node| node.name.starts_with("Initiative for "))
                .map(|node| node.rect.min.x)
                .collect()
        };
        let before = inits(d);
        d.click(&format!("Absent: {player}"));
        let marked = d.has(&format!("Mark present: {player}"));
        let after = inits(d);
        d.state("absent-at-start", Sizes::All);
        d.observe(
            "Absent player",
            d.visible_text("6 need initiative"),
            "Someone absent is not counted among those who still need initiative",
        );
        d.observe(
            "T4.3",
            !before.is_empty() && before == after && d.visible_text("Absent ·"),
            format!("Marking a player absent moves no field, and the row says so: {before:?} then {after:?}"),
        );
        d.click(&format!("Mark present: {player}"));
        d.observe(
            "Absent player",
            marked && d.has(&format!("Absent: {player}")),
            "Setup has a word for a player who did not arrive, and undoing it takes one press",
        );
        d.begin_task();

        d.click("Close run");
        let asked = d.has("Keep running");
        d.press(Key::Escape);
        d.observe(
            "Setup close",
            asked && !d.has("Keep running") && d.has("Close run"),
            "Escape backs out of the close prompt in Setup",
        );
        d.begin_task();

        d.click(ROLL_NPCS);
        d.state("npcs-rolled", Sizes::Default);
        d.goal("roll NPC initiative");

        let mut row_moved = false;
        let mut focus_is_here = false;
        // Every field's place, so that a move anywhere in the list is seen.
        let places = |d: &mut Driver<'_>| -> Vec<f32> {
            PLAYERS
                .iter()
                .map(|(player, _)| d.row_of(&format!("Initiative for {player}")).min.y)
                .collect()
        };
        for (index, (player, total)) in PLAYERS.iter().enumerate() {
            let field = format!("Initiative for {player}");
            let before = places(d);
            if std::mem::take(&mut focus_is_here) {
                d.type_text(total);
            } else {
                d.type_into(&field, total);
            }
            let after = places(d);
            row_moved |= before
                .iter()
                .zip(&after)
                .any(|(before, after)| (before - after).abs() > 1.0);
            if index == 0 {
                d.state("first-player-initiative", Sizes::Default);
                d.goal_within("T4.3", "enter one player's initiative", 2);
            }
            // Enter goes to the next player who has no initiative.
            if index + 1 == PLAYERS.len() / 2 {
                d.press(Key::Enter);
                let next = format!("Initiative for {}", PLAYERS[index + 1].0);
                let focused = d.focused_name();
                d.observe(
                    "T4.3",
                    focused.as_deref() == Some(next.as_str()),
                    format!("Enter moves to {next}. Focus is on {focused:?}"),
                );
                focus_is_here = true;
            }
        }
        d.observe(
            "T4.3",
            !row_moved,
            "Rows do not move during initiative entry",
        );
        d.state("all-initiative-entered", Sizes::Default);
        // Tied rows keep their arrows in two fixed places, so that a second click on one lands
        // on the same arrow.
        let arrows = |d: &Driver<'_>, word: &str| -> Vec<f32> {
            d.nodes()
                .iter()
                .filter(|n| n.name.starts_with("Move ") && n.name.contains(word))
                .map(|n| n.rect.min.x)
                .collect()
        };
        let (ups, downs) = (arrows(d, " before "), arrows(d, " after "));
        d.observe(
            "T4.3",
            !ups.is_empty()
                && ups.windows(2).all(|x| x[0] == x[1])
                && downs.windows(2).all(|x| x[0] == x[1]),
            format!("Tie arrows have fixed places: up {ups:?}, down {downs:?}"),
        );
        d.goal("enter the other players' initiative");

        d.click(BEGIN);
        d.expect(ROUND);
        d.expect(CURRENT_TURN);
        d.state("first-turn", Sizes::All);
        d.goal("begin combat");
    }
}

/// W4: the app restarted mid-session. Return to the interrupted turn.
pub mod run_resume {
    use super::*;
    const INTERRUPTED: &str = "Cinder Wolf 2";

    pub fn run(d: &mut Driver<'_>) {
        d.state("cold-start", Sizes::Default);
        resume_run_from_cold_start(d);
        d.expect(CURRENT_TURN);
        d.state("resumed", Sizes::All);
        d.goal_within("T4.2", "cold start to the interrupted turn", 2);
        // The fixture's saved turn, not whatever the screen says now.
        let current = current_participant(d).unwrap_or_default();
        d.observe(
            "T4.2",
            current == INTERRUPTED && participant_is_selected(d, INTERRUPTED),
            format!("The saved turn was {INTERRUPTED}. Now {current} is current, and selected"),
        );
    }
}

/// W1 at scale: rosters of 12 and 20, at the minimum window size.
pub mod run_large_roster {
    use super::*;
    pub const ROSTER_12: [&str; 12] = [
        "Brannoch Vale",
        "Iselle Marrowind",
        "Kestrel Dunmoor",
        "Oren Thistlewick",
        "Sable Harrowgate",
        "Cinder Wolf 1",
        "Cinder Wolf 2",
        "Cinder Wolf 3",
        "Cinder Wolf 4",
        "Cinder Wolf 5",
        "Ember Cultist 1",
        "Ember Cultist 2",
    ];
    const ROUND_LENGTH: usize = 20;

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN_12);
        d.state("roster-of-12", Sizes::All);
        d.goal("open a run with 12 participants");

        let whole_roster = |d: &Driver<'_>| {
            let nodes = d.nodes();
            let roster = ROSTER_12
                .iter()
                .filter(|name| {
                    nodes
                        .iter()
                        .any(|n| n.name.starts_with(&format!("{name},")) && n.fully_in_view())
                })
                .count();
            roster == ROSTER_12.len() && d.in_view(APPLY_DAMAGE)
        };
        d.observe_both("T1.1", "Next turn is visible with 12 participants", |d| {
            d.in_view(NEXT_TURN)
        });
        d.observe_both(
            "T1.3",
            "All 12 rows are fully in view, with the damage entry",
            whole_roster,
        );
        // At the minimum, 12 rows hold their places, use the room, and share a pitch.
        let places = roster_places(d);
        let dock = d.place_of("Amount").map_or(0.0, |rect| rect.min.y);
        let bottom = places
            .iter()
            .fold(0.0_f32, |low, place| low.max(place.1.max.y));
        let (short, tall) = places
            .iter()
            .fold((f32::MAX, 0.0_f32), |(short, tall), place| {
                (short.min(place.1.height()), tall.max(place.1.height()))
            });
        d.observe(
            "Large roster",
            places.len() == 12 && dock - bottom < 12.0 && tall - short <= 3.0,
            format!(
                "12 rows at 960x640 leave {:.0} px above the dock, and are {short:.1} to {tall:.1} px tall",
                dock - bottom
            ),
        );
        d.click(NEXT_TURN);
        d.goal_within("T1.1", "advance the turn with 12 participants", 1);
        let after = roster_places(d);
        d.observe(
            "Stable rows",
            places.len() == 12 && places == after,
            format!(
                "With 12 rows at 960x640, no row moves on Next turn: {places:?} then {after:?}"
            ),
        );

        close_run(d);
        d.begin_task();
        resume_run(d, RUN_20);
        d.state("roster-of-20", Sizes::All);
        d.goal("open a run with 20 participants");

        // A full round, working at the minimum window, then at the default size. Each
        // click must leave the whole current row in view, with no resize to help it.
        let current_row_in_view = |d: &Driver<'_>| {
            d.nodes()
                .iter()
                .any(|node| node.name.contains(CURRENT_TURN) && node.fully_in_view())
        };
        let mut missed_small = 0;
        let mut seen = std::collections::BTreeSet::new();
        let mut stuck = 0;
        let mut resized = Vec::new();
        for turn in 0..ROUND_LENGTH {
            let before = current_participant(d);
            let rows_before = roster_places(d);
            d.click(NEXT_TURN);
            let after = current_participant(d);
            // The list scrolls, so rows may move together. None may change its height.
            let rows_after = roster_places(d);
            let same_height = rows_before.len() == 20
                && rows_before
                    .iter()
                    .zip(&rows_after)
                    .all(|(a, b)| a.0 == b.0 && (a.1.height() - b.1.height()).abs() < 0.5);
            if !same_height {
                resized.push(turn);
            }
            stuck += usize::from(before == after);
            seen.extend(after);
            if !current_row_in_view(d) {
                missed_small += 1;
            }
            if turn == ROUND_LENGTH / 2 {
                d.state("mid-round", Sizes::Default);
            }
        }
        d.goal("advance through a full round of 20");
        d.observe(
            "Stable rows",
            resized.is_empty(),
            format!("With 20 rows at 960x640, no row changes height on Next turn: {resized:?}"),
        );
        d.begin_task();
        d.set_base(DEFAULT);
        let mut missed_default = 0;
        for _ in 0..ROUND_LENGTH {
            d.click(NEXT_TURN);
            if !current_row_in_view(d) {
                missed_default += 1;
            }
        }
        d.set_base(SMALL);
        d.begin_task();
        d.observe(
            "T1.4",
            missed_small == 0 && missed_default == 0 && stuck == 0 && seen.len() == ROUND_LENGTH,
            format!("Over 20 turns the current row was not fully in view {missed_small} times at 960x640, {missed_default} at 1200x800. The turn stayed put {stuck} times, and {} of {ROUND_LENGTH} participants had a turn", seen.len()),
        );

        // T2.2 at scale: Lookup and back leave the list's scroll and selection alone.
        d.scroll_at(pos2(300.0, 300.0), 3);
        let row = d
            .nodes()
            .into_iter()
            .find(|node| node.name.contains(",") && node.name.contains("HP ") && node.in_view())
            .expect("a row in view");
        let (name, before) = (row.name.clone(), row.rect.min.y);
        look_up(d, "bless");
        d.press(Key::Escape);
        let after = d
            .nodes()
            .into_iter()
            .find(|node| node.name == name)
            .map(|row| row.rect.min.y);
        d.observe(
            "T2.2",
            after == Some(before),
            format!("The roster's scroll is unchanged after Lookup: {before} then {after:?}"),
        );
        d.begin_task();
    }
}

/// W3: a D&D Beyond conflict during a run. The conflict and the sheet update are
/// simulated, because test builds cannot read a live sheet. Every other step is real.
pub mod run_sync_conflict {
    use super::*;
    const AMOUNT_FIELD: &str = "Amount";
    const CLOSE_RUN: &str = "Close run";
    const CLOSE_WHY: &str = "Sync first";
    const SYNCED: &str = "Brannoch Vale";
    const USE_REMOTE: &str = "Use D&D Beyond values";
    const CONFLICT_TEXT: &str = "changed";
    const PARTY: [&str; 5] = [
        "Brannoch Vale",
        "Iselle Marrowind",
        "Kestrel Dunmoor",
        "Oren Thistlewick",
        "Sable Harrowgate",
    ];
    const ROW_SYNC_WORDS: [&str; 2] = ["First read", "Synced"];
    const DISMISS_NOTICE: &str = "Dismiss this message";
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const UNDO_CLEARED: &str = "Undo history was cleared";
    const CLOSE_PROMPT: &str = "has not confirmed changes";
    const DISABLED_REASON: &str = "Wait for pending sync";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        // Sync needs a session. The rail must not say "Not connected" beside synced rows.
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        d.settle();
        d.state("run-before-conflict", Sizes::Default);
        d.goal("resume the run");

        d.app_mut().encounters.simulate_sync_conflict(SYNCED);
        d.settle();
        d.state("conflict-with-nothing-expanded", Sizes::All);
        // The footer's main way out says why it is off, with no hover.
        d.observe(
            "G1",
            d.is_disabled(CLOSE_RUN) && d.visible_text(CLOSE_WHY),
            "While sync has work to finish, Close run is off, and its own label says why",
        );
        let on_row = d.row_node(SYNCED).is_some_and(|row| row.fully_in_view())
            && d.row_words(SYNCED).contains("Conflict");
        d.observe(
            "T3.2",
            on_row,
            "The conflict is on that participant's row, with nothing expanded",
        );
        let reason = d.tooltip_contains(CLOSE_RUN, DISABLED_REASON);
        d.observe(
            "T3.5",
            d.is_disabled(CLOSE_RUN) && reason,
            "A disabled Close run states its reason on hover",
        );

        d.request_close();
        d.observe(
            "T3.5",
            d.has_containing(&format!("{CLOSE_PROMPT} to {SYNCED}")),
            "Closing the app with pending sync warns, and the warning names the character",
        );
        d.state("close-with-pending-sync", Sizes::Default);
        d.press(Key::Escape);

        // From another page: on the strip, and Next turn still works.
        go_library(d);
        let strip = format!("Conflict: {SYNCED}");
        let shown = d.in_view_containing(&strip);
        let before = strip_turn(d);
        d.click(NEXT_TURN);
        let after = strip_turn(d);
        d.observe(
            "T3.2",
            shown && before.is_some() && before != after,
            format!("The strip shows the conflict: {shown}. Next turn still acts: {before:?} then {after:?}"),
        );
        d.click(BACK_TO_RUN);
        d.begin_task();

        d.click_containing(SYNCED);
        d.row_of(USE_REMOTE);
        d.expect(CONFLICT_TEXT);
        d.state("conflict-panel", Sizes::All);
        d.observe(
            "Conflict panel words",
            d.visible_text("HP conflict") && d.visible_text("HP from D&D Beyond"),
            "The conflict panel names the conflict, and its buttons name the values",
        );
        d.click(USE_REMOTE);
        d.state("conflict-resolved", Sizes::Default);
        d.goal_within("T3.2", "resolve a conflict", 2);
        let resolved = d
            .row_node(SYNCED)
            .is_some_and(|row| !row.name.contains("Conflict"));
        d.observe(
            "T3.2",
            resolved,
            "After resolving, the row no longer says Conflict",
        );

        d.app_mut().encounters.simulate_connection();
        d.settle();
        let before = d.interaction_count();
        d.click(SYNC_PARTY);
        let cost = d.interaction_count() - before;
        d.state("party-sync-on", Sizes::Default);
        let all_rows = PARTY.iter().all(|name| {
            ROW_SYNC_WORDS
                .iter()
                .any(|word| d.row_words(name).contains(word))
        });
        d.observe(
            "T3.3",
            cost <= 2 && all_rows && d.has(STOP_SYNC),
            format!("Simulated connection: sync turned on for the whole party in {cost}. Every party row paints one of {ROW_SYNC_WORDS:?}: {all_rows}"),
        );

        // Clear what an earlier update left, so this update has to make the message.
        if d.has(DISMISS_NOTICE) {
            d.click(DISMISS_NOTICE);
        }
        let clear_before = !d.has_containing(UNDO_CLEARED);
        let rows_before = roster_places(d);
        let hp_before = d.row_hp(SYNCED).unwrap_or(0);
        let amount_before = d.place_of(AMOUNT_FIELD);
        d.app_mut().encounters.simulate_read(SYNCED, 3);
        d.settle();
        d.state("undo-cleared-by-sheet-update", Sizes::Default);
        let changed = format!("{SYNCED} HP {hp_before} → {}", hp_before - 3);
        // A notice goes under the list, and no row moves from where it was.
        let rows_after = roster_places(d);
        d.observe(
            "Stable rows",
            rows_before.len() == 8
                && rows_before == rows_after
                && amount_before == d.place_of(AMOUNT_FIELD),
            format!("A notice moves no row and not the dock at 960x640: {rows_before:?} then {rows_after:?}"),
        );
        d.observe(
            "T3.4",
            clear_before
                && d.visible_text(&format!("From D&D Beyond: {changed}. {UNDO_CLEARED}")),
            format!("Simulated sheet read: a message says, only after it, what changed from what to what: {changed}"),
        );
        // It stays through what the DM does next, and the DM dismisses it.
        d.click(NEXT_TURN);
        d.observe(
            "T3.4",
            d.visible_text(UNDO_CLEARED) && d.has(DISMISS_NOTICE),
            "The message stays after the next turn, until the DM dismisses it",
        );
        d.click(DISMISS_NOTICE);
        d.observe(
            "T3.4",
            !d.visible_text(UNDO_CLEARED),
            "Dismissing removes it",
        );
        d.click(STOP_SYNC);
        d.begin_task();
        d.unreached(
            "wrong campaign, a duplicate character, and a live read after a reconnect (a simulated read goes through the real comparison)",
            "the error state on the row, because these need a live session to arise",
        );
    }

    /// The turn the strip names, on a page other than the run.
    fn strip_turn(d: &Driver<'_>) -> Option<String> {
        d.nodes()
            .into_iter()
            .find(|node| node.name.ends_with("’s turn"))
            .map(|node| node.name)
    }
}

/// Go through every top-level page and back. Each page should keep its state.
pub mod nav_state {
    use super::*;
    const QUERY: &str = "dragon";
    const CREATURE: &str = "Adult Black Dragon";
    const SEARCH_RESULTS: &str = "Search results";
    const MEMBER: &str = "Iselle Marrowind";
    const SHEET_PAGE: &str = "Spells";
    const SHEET_PAGE_TEXT: &str = "Spellcasting";
    const ENCOUNTER: &str = "Bridge Ambush";
    const ROSTER_TEXT: &str = "1. Brannoch Vale";
    const NOT_IMPLEMENTED: &str = "not implemented";
    const CAMPAIGN_TAB_TEXT: &str = "Run folder";
    const REMOVED_PAGES: [&str; 2] = ["Creatures", "Simulations"];

    pub fn run(d: &mut Driver<'_>) {
        go_library(d);
        d.begin_task();
        search_library(d, QUERY);
        d.click_containing(CREATURE);
        d.state("library-set-up", Sizes::DefaultAndHiDpi);

        go_party(d);
        d.click_containing(MEMBER);
        d.click(SHEET_PAGE);
        d.state("party-set-up", Sizes::Default);

        go_encounters(d);
        open_saved_encounter(d, ENCOUNTER);
        d.expect(ROSTER_TEXT);
        d.state("encounters-set-up", Sizes::Default);

        go_campaign(d);
        d.click(TAB_RUN_SETTINGS);
        d.goal("set up state on three pages");

        go_settings(d);
        d.state("settings", Sizes::Default);

        go_library(d);
        let library = d.has_containing(SEARCH_RESULTS) && d.in_view(CREATURE);
        d.state("library-returned", Sizes::Default);

        go_campaign(d);
        let campaign = d.in_view_containing(CAMPAIGN_TAB_TEXT);
        d.state("campaign-returned", Sizes::Default);

        go_party(d);
        let party = d.in_view_containing(MEMBER) && d.in_view_containing(SHEET_PAGE_TEXT);
        d.state("party-returned", Sizes::Default);

        go_encounters(d);
        let encounters = d.in_view_containing(ROSTER_TEXT);
        d.state("encounters-returned", Sizes::Default);
        d.goal("return to every page");
        d.observe(
            "G2",
            library && campaign && party && encounters,
            format!("State kept: Library {library}, Campaign reopens its last tab {campaign}, Party {party}, Encounters {encounters}"),
        );

        let removed = REMOVED_PAGES.iter().all(|page| !d.has(page));
        d.observe(
            "G3",
            removed && !d.has_containing(NOT_IMPLEMENTED),
            format!(
                "Creatures and Simulations are gone: {removed}. No page says it is not implemented"
            ),
        );
        d.not_measured(
            "G5",
            "the app ships one theme, so there is nothing to compare yet",
        );
        // Any violation shows in this run's totals. This says that the checks did run.
        let (minimum, high) = d.checks_run();
        d.observe(
            "G4",
            minimum >= d.states_shown() && high >= 1,
            format!("This scenario's checks ran {minimum} times at 960x640 and {high} at 2x"),
        );
    }
}

/// W7 from a run: open a participant's sheet during combat, then return.
pub mod party_from_run {
    use super::*;
    const MEMBER: &str = "Kestrel Dunmoor";
    const SHEET_TITLE: &str = "Kestrel Dunmoor";
    const SHEET_PAGE: &str = "Record";
    const SHEET_SUBPAGE: &str = "Spells";
    const CASTER: &str = "Iselle Marrowind";
    const SHEET_ONLY_TEXT: &str = "Ability scores";
    const OTHER_ROW: &str = "Oren Thistlewick";
    const FRESHNESS: &str = "Fetched";
    const SHEET_TAB: &str = "Sheet";
    const ZERO_DAMAGE: &str = "60";
    const ABSENT_ROW: &str = "Oren Thistlewick";
    const ABSENT: &str = "Absent";
    const MANY_CONDITIONS: [&str; 5] =
        ["Blinded", "Charmed", "Grappled", "Paralyzed", "Restrained"];
    const END_ENCOUNTER: &str = "End encounter…";
    const END_NOW: &str = "End encounter";
    const ENDED: &str = "Ended: ";
    const UNDO_ENDING: &str = "Undo: end encounter";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.state("in-the-run", Sizes::Default);
        d.goal("resume the run");

        d.click_containing(MEMBER);
        d.expect(SHEET_TITLE);
        d.state("sheet-beside-run", Sizes::All);
        d.goal_within("T7.1", "open a participant's sheet from the run", 1);
        d.observe_both(
            "T7.1",
            "The roster, Next turn, and the sheet are all in view",
            |d| {
                d.in_view(NEXT_TURN)
                    && d.row_node(MEMBER).is_some_and(|row| row.fully_in_view())
                    && d.row_node(OTHER_ROW).is_some_and(|row| row.fully_in_view())
                    && participant_is_selected(d, MEMBER)
                    && d.in_view_containing(SHEET_ONLY_TEXT)
            },
        );
        d.observe_both("T7.2", "The sheet in the run says how old it is", |d| {
            d.in_view_containing(FRESHNESS)
        });

        // A caster's page, so that the capture shows the state it names.
        d.click_row(CASTER);
        d.click(SHEET_SUBPAGE);
        d.state("sheet-spells-beside-run", Sizes::Default);
        d.observe(
            "Spells beside run",
            d.visible_text("Fire Bolt"),
            "The spells page beside the run lists a caster's spells",
        );
        d.click_row(MEMBER);
        d.begin_task();
        d.click(SHEET_PAGE);
        d.state("record-tab", Sizes::Default);
        d.click(SHEET_TAB);
        d.expect(RUN_HEADING);
        d.state("back-in-the-run", Sizes::Default);
        d.goal("read a page and return to the run");

        // The hardest rows: 0 HP, absent, and many conditions. Words, not only colour.
        d.click_containing(MEMBER);
        d.type_text(ZERO_DAMAGE);
        d.press(Key::Enter);
        d.expect("0 HP");
        d.click_containing(ABSENT_ROW);
        d.click(ABSENT);
        d.click(CONDITIONS);
        for condition in MANY_CONDITIONS {
            d.click(condition);
        }
        d.state("zero-absent-many-conditions", Sizes::Default);
        d.review("T1.8", "0 HP and absent are distinct without colour");
        // Each word on its own row, in full, at the minimum window. The tab that also has
        // these words is closed first, so it cannot answer for the row.
        d.click(SHEET_TAB);
        let row = d.row_words(ABSENT_ROW);
        let unread: Vec<&str> = MANY_CONDITIONS
            .into_iter()
            .filter(|condition| !row.contains(condition))
            .collect();
        let zero = d.row_words(MEMBER);
        d.observe(
            "T1.8",
            unread.is_empty() && row.contains("Absent") && zero.contains("0 HP"),
            format!("At 960x640, {ABSENT_ROW} paints: {row}. {MEMBER} paints: {zero}. Missing: {unread:?}"),
        );
        d.click_containing(MEMBER);
        d.state("zero-hp-selected", Sizes::Default);

        // The third view of a run. It reads differently, and undo can reopen it.
        d.click(END_ENCOUNTER);
        d.click(END_NOW);
        d.expect(ENDED);
        d.state("ended", Sizes::Default);
        d.review("T4.5", "the ended view, to compare with live and setup");
        d.click_containing(UNDO_ENDING);
        d.expect(CURRENT_TURN);
    }
}

/// W8 across pages: copy a Library entry, then add it to an unsaved encounter draft.
pub mod library_to_encounter {
    use super::*;
    const DRAFT_NAME: &str = "Deep Ford";
    const QUERY: &str = "aboleth";
    const ENTRY: &str = "Aboleth";
    const COPY_NAME: &str = "Aboleth copy";
    const SEEDED_ROW: &str = "1. Brannoch Vale";
    const LAST_PARTY_ROW: &str = "5. Sable Harrowgate";

    pub fn run(d: &mut Driver<'_>) {
        d.click(NEW_ENCOUNTER);
        d.type_text(DRAFT_NAME);
        d.click(ADD_PARTY);
        d.expect(SEEDED_ROW);
        d.state("draft-open", Sizes::Default);
        d.begin_task();

        go_library(d);
        search_library(d, QUERY);
        d.click_containing(ENTRY);
        d.state("library-entry", Sizes::Default);
        copy_open_entry_to_campaign(d);
        d.state("copied", Sizes::Default);

        // One press, from the finished dialog, with nothing to reload or search for.
        d.click(&format!("Add to {DRAFT_NAME}"));
        go_encounters(d);
        d.expect(DRAFT_NAME);
        let name_kept = d
            .field_text(&format!("Encounter name: {DRAFT_NAME}"))
            .as_deref()
            == Some(DRAFT_NAME);
        d.state("copy-in-open-draft-results", Sizes::Default);
        d.observe(
            "T8.1",
            name_kept
                && d.has_label(SEEDED_ROW)
                && d.has_label(LAST_PARTY_ROW)
                && d.has_label(&format!("6. {COPY_NAME}")),
            "The party added before the copy is still in the draft, and the copy has joined it with no reload",
        );
        let cost = d.goal("copy an entry and use it in an encounter");
        d.observe(
            "T8.1",
            cost <= 9,
            format!("It took {cost}, with nothing to reload or search for again"),
        );
        d.replace_text_in(FIND_CREATURE, COPY_NAME);
        d.begin_task();

        d.click(SAVE_ENCOUNTER);
        d.state("copy-in-encounter", Sizes::All);
        d.goal("save the encounter with the copy");
    }
}

/// T5.5 from `docs/ux-intent.md`: edit the run folder with no draft, then use every
/// enabled button that was there. Save encounter now exists only with a draft.
pub mod run_folder_defect {
    use super::*;
    const RUN_FOLDER: &str = "Run folder";
    const SAVE_SETTINGS: &str = "Save run settings";
    const DISCARD: &str = "Discard changes";
    const BACK_TO_LIST: &str = "Back to encounters";

    pub fn run(d: &mut Driver<'_>) {
        go_campaign(d);
        d.click(TAB_RUN_SETTINGS);
        d.replace_text_in(RUN_FOLDER, "Elsewhere");
        d.state("run-folder-edited", Sizes::Default);
        d.goal("edit the run folder with no draft");
        // A panic ends this app, so the video stops first and nothing follows.
        d.stop_filming();
        let click = std::panic::AssertUnwindSafe(|| {
            // Every enabled button on the page with no draft, each from a clean start.
            go_encounters(d);
            let names = d.enabled_page_buttons();
            for name in &names {
                if name == NEW_ENCOUNTER || name == SAVE_ENCOUNTER || d.has(name) {
                    d.click_first(name);
                }
                if d.has(CLOSE_RUN) && !d.is_disabled(CLOSE_RUN) {
                    close_run(d);
                }
                go_encounters(d);
                if d.has(DISCARD) && !d.is_disabled(DISCARD) {
                    d.click(DISCARD);
                }
                if d.has(BACK_TO_LIST) && !d.is_disabled(BACK_TO_LIST) {
                    d.click(BACK_TO_LIST);
                }
            }
            go_campaign(d);
            d.click(SAVE_SETTINGS);
            assert!(!names.is_empty(), "the page has buttons to press");
        });
        let panicked = std::panic::catch_unwind(click).is_err();
        d.observe(
            "T5.5",
            !panicked,
            "Pressing every enabled button with no draft open does not panic. See the defect in docs/ux-intent.md W5",
        );
    }
}

/// Every token and component of the design system, in each state.
pub mod gallery {
    use super::*;
    const COMPONENTS: &str = "Components";
    const MESSAGES: &str = "Messages";
    const PRIMARY: &str = "Primary";
    const SECONDARY: &str = "Secondary";

    pub fn run(d: &mut Driver<'_>) {
        d.app_mut().gallery = true;
        d.settle();
        d.state("tokens", Sizes::AllAndHiDpi);
        d.click(COMPONENTS);
        d.state("components", Sizes::AllAndHiDpi);
        d.check_tab_order("gallery, components");
        d.state_hovering("hover-primary", PRIMARY);
        d.state_hovering("hover-secondary", SECONDARY);
        d.press(Key::Tab);
        d.state("focus-ring", Sizes::Default);
        d.click(MESSAGES);
        d.state("messages", Sizes::AllAndHiDpi);
        d.goal("see every part of the design system");
        d.click(COMPONENTS);
        let temp = crate::design::temp_hp_text(5);
        for _ in 0..30 {
            if d.visible_text(&temp) {
                break;
            }
            d.scroll_at(pos2(600.0, 300.0), 1);
        }
        d.observe(
            "Temporary HP",
            d.visible_text(&temp),
            "Temporary HP shows with its mark, as it does in the run and on Party",
        );
    }
}

/// Drag the edges of the rail, a page's lists and a panel, then work at those widths.
/// The widths come back after a restart.
pub mod column_resize {
    use super::*;
    use crate::design::{PANEL_MIN_WIDTH, RAIL_DRAG_MAX_WIDTH, RAIL_WIDTH, S, side_panel_width};

    const CHECK: &str = "Column resize";
    const MEMBER: &str = "Brannoch Vale";
    /// Far past either end of what any column can reach.
    const FAR_LEFT: f32 = 1.0;
    const FAR_RIGHT: f32 = 1190.0;

    fn rail_edge(d: &Driver<'_>) -> f32 {
        d.place_of(NAV_LIBRARY).map_or(0.0, |rect| rect.right() + S)
    }

    fn search(d: &Driver<'_>) -> egui::Rect {
        d.place_of(SEARCH_LIBRARY).unwrap_or(egui::Rect::ZERO)
    }

    fn sections_edge(d: &Driver<'_>) -> f32 {
        search(d).left() - S
    }

    fn entries_edge(d: &Driver<'_>) -> f32 {
        search(d).right() + S
    }

    fn near(a: f32, b: f32) -> bool {
        (a - b).abs() <= 1.0
    }

    pub fn run(d: &mut Driver<'_>) {
        d.set_base(DEFAULT);
        resume_run_from_cold_start(d);
        let run_edge = DEFAULT.0 - crate::design::M - side_panel_width(DEFAULT.0);
        d.drag_column_edge(run_edge, FAR_RIGHT);
        d.state("run-panel-narrow", Sizes::All);
        d.drag_column_edge(DEFAULT.0 - crate::design::M - PANEL_MIN_WIDTH, FAR_LEFT);
        d.state("run-panel-wide", Sizes::All);

        go_library(d);
        d.drag_column_edge(rail_edge(d), RAIL_DRAG_MAX_WIDTH);
        d.observe(
            CHECK,
            near(rail_edge(d), RAIL_DRAG_MAX_WIDTH),
            format!("The rail follows its edge to {}", rail_edge(d)),
        );
        d.drag_column_edge(sections_edge(d), FAR_RIGHT);
        let start = entries_edge(d) - sections_edge(d);
        d.drag_column_edge(entries_edge(d), FAR_RIGHT);
        let wide = (rail_edge(d), sections_edge(d), entries_edge(d));
        d.observe(
            CHECK,
            wide.2 - wide.1 > start,
            "The entry list follows its edge",
        );
        d.observe(
            CHECK,
            wide.2 <= DEFAULT.0 - 320.0,
            "The reader keeps its room, however far the list is dragged",
        );
        d.state("library-wide-columns", Sizes::All);

        d.click(BACK_TO_RUN);
        d.restart();
        go_library(d);
        d.observe(
            CHECK,
            (rail_edge(d), sections_edge(d), entries_edge(d)) == wide,
            "Dragged widths return after a restart",
        );

        d.drag_column_edge(entries_edge(d), FAR_LEFT);
        d.drag_column_edge(sections_edge(d), FAR_LEFT);
        d.drag_column_edge(rail_edge(d), FAR_LEFT);
        d.observe(
            CHECK,
            near(rail_edge(d), RAIL_WIDTH),
            "The rail stops at its narrowest",
        );
        d.state("library-narrow-columns", Sizes::All);

        // Beside the widest rail, the pages and panels must still work.
        d.drag_column_edge(rail_edge(d), RAIL_DRAG_MAX_WIDTH);
        go_party(d);
        d.expect(MEMBER);
        d.state("party-beside-wide-rail", Sizes::All);
        d.click(BACK_TO_RUN);
        d.state("run-beside-wide-rail", Sizes::All);
        d.click(NAV_LOOKUP);
        d.state("lookup-beside-wide-rail", Sizes::All);
        // Lookup gives the page what it needs, here beside a rail of the widest kind.
        let lookup_edge = (DEFAULT.0 - side_panel_width(DEFAULT.0))
            .max(RAIL_DRAG_MAX_WIDTH + crate::design::PANEL_LEAVES_PAGE);
        d.drag_column_edge(lookup_edge, FAR_LEFT);
        d.state("lookup-wide-beside-wide-rail", Sizes::All);
        d.click(NAV_LOOKUP);
        d.expect(NEXT_TURN);
        d.click(NEXT_TURN);
        d.goal("resize the columns and go on working");
    }
}

/// Section 3's table: an open run survives a restart, on the current turn, with the
/// last page and party sync kept. The scenario really starts the app a second time.
pub mod run_restart {
    use super::*;
    const OTHER: &str = "Ember Cultist";
    const PREVIOUS_CAMPAIGN: &str = "previous campaign";
    const RUN_FOLDER: &str = "Run folder";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_row(OTHER);
        d.click(NEXT_TURN);
        let current = current_participant(d).unwrap_or_default();
        d.state("before-restart", Sizes::Default);
        d.goal("be in a run");
        d.restart();
        d.state("after-restart", Sizes::All);
        d.goal("restart");
        d.begin_task();
        d.observe(
            "Restart",
            d.has_containing(RUN_HEADING)
                && current_participant(d).as_deref() == Some(current.as_str())
                && participant_is_selected(d, &current),
            format!("The run reopens by itself on the same turn, {current} selected"),
        );
        // The other pages load too, with the run still open behind them.
        go_encounters(d);
        d.observe(
            "Restart",
            d.has_containing(SAVED_RUNS) && !d.has_containing(PREVIOUS_CAMPAIGN),
            "After a restart into a run, Encounters lists the campaign's encounters",
        );
        d.state("encounters-after-restart", Sizes::Default);
        go_campaign(d);
        d.click(TAB_RUN_SETTINGS);
        d.observe(
            "Restart",
            d.has_containing(RUN_FOLDER),
            "After a restart into a run, Rules and runs loads",
        );
        d.click(BACK_TO_RUN);
        d.begin_task();
        // The prompt shares the footer's narrow row, and must keep every word.
        d.click("Close run");
        d.state("close-run-prompt", Sizes::Default);
        d.click("Keep running");
        d.begin_task();
        close_run(d);
        go_library(d);
        d.restart();
        d.observe(
            "Restart",
            d.has_containing(SEARCH_LIBRARY) && !d.has_containing(RUN_HEADING),
            "With no open run, the app reopens on the last page",
        );
        d.state("restart-on-last-page", Sizes::Default);
        d.goal("restart on a page");
    }
}

/// States a run can reach that a good session does not show.
pub mod run_edge_states {
    use super::*;
    const LONG_EFFECT: &str =
        "Wreathed in the cinders of the old shrine, so every light within thirty feet burns low";
    const TARGET: &str = "Kestrel Dunmoor";
    const OTHER_EFFECT: &str = "Other effect";
    const ADD_EFFECT: &str = "Add";
    const ABSENT: &str = "Absent";
    const CLOSE_ERROR: &str = "Dismiss this message";
    const MARK_PRESENT: &str = "Mark present";
    const KEEP_RUN: &str = "Replace the file with this run";
    const SAVE_AS_NEW: &str = "Save as a new run";
    const ALL: [&str; 8] = [
        "Brannoch Vale",
        "Cinder Wolf 1",
        "Cinder Wolf 2",
        "Iselle Marrowind",
        "Kestrel Dunmoor",
        "Oren Thistlewick",
        "Sable Harrowgate",
        "Ember Cultist",
    ];

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_row(TARGET);
        d.click(CONDITIONS);
        d.type_into(OTHER_EFFECT, LONG_EFFECT);
        d.click(ADD_EFFECT);
        d.state("long-effect-text", Sizes::All);
        d.observe(
            "Long effects",
            d.visible_text(LONG_EFFECT),
            "A long effect wraps on its row and is read in full at 960x640",
        );
        d.goal("record a long effect");

        for name in ALL {
            d.click_row(name);
            d.click(ABSENT);
        }
        d.expect("No participants are present");
        d.observe(
            "Nobody present",
            d.is_disabled(NEXT_TURN),
            "With nobody present, Next turn is off, and a steady line says why",
        );
        d.state("all-absent", Sizes::Default);
        d.goal("see everyone absent");
        for name in ALL {
            d.click_row(name);
            d.click(MARK_PRESENT);
        }
        // A second instance writes the file. Saving must stop and say so.
        let path = d
            .app_mut()
            .encounters
            .run()
            .and_then(|run| run.path().map(std::path::Path::to_path_buf))
            .expect("a saved run");
        let text = std::fs::read_to_string(&path).expect("read the run") + "\n";
        std::fs::write(&path, text).expect("change the run on disk");
        d.click_row(TARGET);
        d.type_text("3");
        d.press(Key::Enter);
        d.expect("changed on disk");
        d.expect("Not saved");
        d.state("changed-on-disk", Sizes::Default);
        d.goal("see the run changed on disk");
        // The state cannot be dismissed, and closing tells the truth about it.
        d.observe(
            "Changed on disk",
            !d.has(CLOSE_ERROR) && d.has(KEEP_RUN) && d.has(SAVE_AS_NEW),
            "The message stays, with both ways out",
        );
        d.click(CLOSE_RUN);
        d.observe(
            "Changed on disk",
            !d.has_containing("stays saved")
                && d.has("Close anyway")
                && d.visible_text("loses unsaved changes"),
            "Close says what it will lose, in place of \"The run stays saved\"",
        );
        d.state("close-while-not-saved", Sizes::Default);
        d.click("Keep running");
        // Keep this run: the file becomes this run, and later turns save by themselves.
        d.click(KEEP_RUN);
        let saved = std::fs::read_to_string(&path).expect("read the run");
        d.observe(
            "Changed on disk",
            d.visible_text("Saved")
                && !d.has_containing("changed on disk")
                && saved.contains(LONG_EFFECT),
            "Keep this run saves it over the changed file",
        );
        // It again, and this time save beside it.
        std::fs::write(&path, saved.clone() + "\n").expect("change the run on disk");
        d.click_row(TARGET);
        d.type_text("1");
        d.press(Key::Enter);
        d.expect("changed on disk");
        d.click(SAVE_AS_NEW);
        let new_path = d
            .app_mut()
            .encounters
            .run()
            .and_then(|run| run.path().map(std::path::Path::to_path_buf))
            .expect("a saved run");
        d.observe(
            "Changed on disk",
            new_path != path
                && new_path.exists()
                && std::fs::read_to_string(&path).is_ok_and(|text| text == saved.clone() + "\n")
                && d.visible_text("Saved"),
            "Save as a new run keeps the changed file as it was, and saves this one beside it",
        );
        d.unreached(
            "HP unknown on a live row",
            "the setup view, because a live run requires known HP (Run::open and Begin refuse otherwise)",
        );
    }
}

/// T3.1: each synced row says its own state, with nothing expanded. The states are
/// simulated, because test builds cannot read a live sheet.
pub mod run_sync_states {
    use super::*;
    const KEEP_LOCAL: &str = "Keep Splat values and retry";
    use crate::combat_sync::SyncState;
    const STATES: [(&str, SyncState, &str); 5] = [
        ("Brannoch Vale", SyncState::Conflict, "Conflict"),
        ("Iselle Marrowind", SyncState::Matching, "Synced"),
        ("Kestrel Dunmoor", SyncState::Pending, "Sync pending"),
        ("Oren Thistlewick", SyncState::ReadOnly, "Sync read-only"),
        ("Sable Harrowgate", SyncState::Error, "Sync error"),
    ];
    /// The first read shows after `Sync party`, and "off" is a row with no word at all.
    const FIRST_READ: (&str, SyncState, &str) =
        ("Sable Harrowgate", SyncState::FirstRead, "First read");
    const UNSYNCED_ROW: &str = "Cinder Wolf 1";
    const DISMISS: &str = "Dismiss this message";
    const TWO_LINES: &str =
        "Brannoch Vale, Iselle Marrowind, Kestrel Dunmoor, Oren Thistlewick, Sable Harrowgate";
    const USE_REMOTE: &str = "Use D&D Beyond values";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        for (name, state, _) in STATES {
            d.app_mut().encounters.simulate_sync_state(name, state);
        }
        d.settle();
        for (name, _, word) in STATES {
            let row = d.row_node(name);
            let in_view = row.as_ref().is_some_and(|row| row.fully_in_view());
            let text = row.map_or_else(String::new, |row| row.name);
            d.observe(
                "T3.1",
                in_view && text.contains(word),
                format!("{name}'s row, in view, says {word}: {text}"),
            );
            let painted = d.row_words(name);
            d.observe(
                "T3.1",
                painted.contains(word),
                format!("At 960x640, simulated state: {name}'s row paints {word}: {painted}"),
            );
        }
        let plain = d
            .row_node(UNSYNCED_ROW)
            .map_or_else(String::new, |row| row.name);
        d.observe(
            "T3.1",
            !plain.contains("Sync") && !plain.contains("Conflict"),
            format!("A row with no link says nothing about sync: {plain}"),
        );
        let (name, state, word) = FIRST_READ;
        d.app_mut().encounters.simulate_sync_state(name, state);
        d.settle();
        d.observe(
            "T3.1",
            d.row_words(name).contains(word),
            format!("Simulated: {name}'s row paints {word}"),
        );
        // Back to the state this row had, so that the capture shows the other five.
        let (name, state, _) = STATES[4];
        d.app_mut().encounters.simulate_sync_state(name, state);
        d.settle();
        d.state("every-sync-state", Sizes::All);
        d.goal("show every sync state");

        // At the default size, 8 rows have the room, and a two-line message takes none
        // of it from them. Each row's height, then the same places with the message up.
        d.click(DISMISS);
        let (quiet, quiet_dock) = d.at_size(DEFAULT, |d| (roster_places(d), d.place_of("Amount")));
        d.app_mut().encounters.simulate_sheet_update(TWO_LINES);
        d.settle();
        let (loud, loud_dock) = d.at_size(DEFAULT, |d| (roster_places(d), d.place_of("Amount")));
        let shortest = quiet
            .iter()
            .chain(&loud)
            .fold(f32::MAX, |short, row| short.min(row.1.height()));
        d.observe(
            "Rows have room",
            quiet.len() == 8 && shortest >= 52.0,
            format!("At 1200x800, with and without a two-line message, 8 rows are not compressed: the shortest is {shortest} px"),
        );
        d.observe(
            "Stable rows",
            quiet.len() == 8 && quiet == loud && quiet_dock == loud_dock,
            format!("At 1200x800, a two-line message moves no row and not the dock: {quiet:?} then {loud:?}"),
        );
        d.begin_task();

        // Ending the encounter does not end the sync. Close waits, and the conflict can be
        // resolved in the ended view.
        d.app_mut()
            .encounters
            .simulate_sync_conflict("Iselle Marrowind");
        d.settle();
        d.click("End encounter…");
        d.click("End encounter");
        d.observe(
            "Ended run close",
            d.is_disabled(CLOSE_RUN)
                && d.row_words("Iselle Marrowind").contains("Conflict")
                && d.has(USE_REMOTE),
            "With a conflict, Close is blocked in the ended view, whose row and controls show it",
        );
        d.state("ended-with-conflict", Sizes::Default);
        // A pair of choices is never cut: both are in view, at the minimum window too.
        let both = |d: &Driver<'_>| {
            let nodes = d.nodes();
            [USE_REMOTE, KEEP_LOCAL]
                .iter()
                .all(|name| nodes.iter().any(|n| n.name == *name && n.fully_in_view()))
        };
        d.observe_both(
            "Ended run close",
            "Both choices of a conflict are in view",
            both,
        );
        // Keeping the DM's value changes nothing in the run, so the steps before it stay.
        d.click(KEEP_LOCAL);
        for (name, ..) in STATES {
            let stop = format!("Stop sync for {name}");
            if d.has(&stop) {
                d.click(&stop);
            }
        }
        d.observe(
            "Ended run close",
            !d.is_disabled(CLOSE_RUN),
            "With the sync resolved or stopped, the ended run can close",
        );
        d.click_containing("Undo: end encounter");
        for (name, state, _) in STATES {
            d.app_mut().encounters.simulate_sync_state(name, state);
        }
        d.settle();
        d.app_mut()
            .encounters
            .simulate_sync_conflict("Brannoch Vale");
        d.settle();
        go_library(d);
        d.state("conflict-on-strip", Sizes::All);
    }
}

/// T1.3 under pressure: a sync conflict must not push any of 12 rows out of view. Then
/// the Tab order on a scrolling roster of 20, where scroll handles appear.
pub mod run_roster_conflict {
    use super::*;
    const CLOSE_RUN: &str = "Close run";
    const CLOSE_ANYWAY: &str = "Close anyway";
    use crate::combat_sync::SyncState;
    const SYNCED: &str = "Brannoch Vale";
    const USE_REMOTE: &str = "Use D&D Beyond values";
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const AMOUNT: &str = "Amount";
    const LOADED_ROW: &str = "Kestrel Dunmoor";
    const ONE_MORE_LINE: &str = "Cinder Wolf 3";
    const ONE_MORE_CONDITION: &str = "Prone";
    const LONG_MESSAGE: &str =
        "Brannoch Vale HP 58 → 54, Iselle Marrowind HP 27 → 18, Kestrel Dunmoor HP 11 → 8";
    const LOADED_CONDITIONS: [&str; 5] =
        ["Blinded", "Charmed", "Grappled", "Paralyzed", "Restrained"];

    /// The rows of the 12 that are not in full view.
    fn rows_cut(d: &Driver<'_>) -> Vec<&'static str> {
        let nodes = d.nodes();
        run_large_roster::ROSTER_12
            .into_iter()
            .filter(|name| {
                !nodes
                    .iter()
                    .any(|n| n.name.starts_with(&format!("{name},")) && n.fully_in_view())
            })
            .collect()
    }

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN_12);
        d.app_mut()
            .settings
            .simulate(crate::settings::Account::Connected);
        d.app_mut()
            .encounters
            .simulate_sync_state(SYNCED, SyncState::Conflict);
        d.settle();
        d.state("roster-of-12-with-conflict", Sizes::All);
        let nodes = d.nodes();
        let rows = run_large_roster::ROSTER_12
            .iter()
            .filter(|name| {
                nodes
                    .iter()
                    .any(|n| n.name.starts_with(&format!("{name},")) && n.fully_in_view())
            })
            .count();
        d.observe(
            "T1.3",
            rows == run_large_roster::ROSTER_12.len() && d.in_view(APPLY_DAMAGE),
            format!("During a conflict at 960x640: {rows} of 12 rows in full view, with the damage entry"),
        );
        d.goal("show a conflict with 12 rows");

        // Section 6's hard case: 5 conditions on a row, the party's sync words, and an
        // error line, with all 12 rows and the amount field still in view.
        d.app_mut().encounters.simulate_connection();
        d.settle();
        d.click(SYNC_PARTY);
        d.click_row(LOADED_ROW);
        d.click(CONDITIONS);
        for condition in LOADED_CONDITIONS {
            d.click(condition);
        }
        // A message the DM has not dismissed takes one line here, so no row has to scroll.
        d.app_mut().encounters.simulate_sheet_update(LONG_MESSAGE);
        d.settle();
        d.click_row(ONE_MORE_LINE);
        d.click(CONDITIONS);
        d.click(ONE_MORE_CONDITION);
        d.state("message-on-one-line", Sizes::Default);
        let cut = rows_cut(d);
        d.observe(
            "T1.3",
            cut.is_empty() && d.visible_text("Undo history was cleared. Hover here"),
            format!(
                "At 960x640 with a long message, it takes one line and every row stays: {cut:?}"
            ),
        );
        d.click(ONE_MORE_CONDITION);
        d.click_row(LOADED_ROW);
        let errored = LOADED_CONDITIONS.len() + 1;
        d.app_mut()
            .encounters
            .simulate_run_error("The reference for Ember Cultist could not be read.");
        d.settle();
        d.state("loaded-roster-of-12", Sizes::Default);
        let cut = rows_cut(d);
        d.observe(
            "T1.3",
            cut.is_empty() && d.in_view(AMOUNT) && d.in_view(APPLY_DAMAGE),
            format!("At 960x640 with {errored} extra lines and a message not dismissed, rows not in full view: {cut:?}"),
        );
        d.app_mut().encounters.simulate_run_error("");
        d.settle();
        d.click(STOP_SYNC);
        d.click_containing(SYNCED);
        d.click(USE_REMOTE);
        // Other rows still hold simulated changes, so closing says so first.
        d.click(CLOSE_RUN);
        d.click(CLOSE_ANYWAY);
        d.begin_task();
        resume_run(d, RUN_20);
        d.check_tab_order("run, roster of 20");
        d.goal("tab through a roster of 20");
        for _ in 0..20 {
            d.click(NEXT_TURN);
        }
        d.state("after-full-round", Sizes::All);
    }
}

/// The catalogue opens on what this campaign can use, a switch shows everything, and
/// browsing never changes the active campaign.
pub mod library_scope {
    use super::*;
    const USES: &str = "What this campaign can use";
    const EVERYTHING: &str = "Everything";
    const ACTIVE: &str = "The Emberfall Marches";

    pub fn run(d: &mut Driver<'_>) {
        go_library(d);
        d.begin_task();
        let before = d.app_mut().active_campaign_name();
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.expect(USES);
        d.state("scope-menu", Sizes::Default);
        d.click(EVERYTHING);
        d.state("scope-everything", Sizes::Default);
        d.goal_within("A9", "show everything", 2);
        let after = d.app_mut().active_campaign_name();
        d.observe(
            "A9",
            before == after && after.as_deref() == Some(ACTIVE),
            format!("The active campaign is {after:?} after browsing, as before: {before:?}"),
        );
    }
}

/// The signed-in path. A test session makes the rail, the page, Party and the import list
/// share one state. No request leaves the process: see `crate::hermetic`.
pub mod signed_in {
    use super::*;
    use crate::settings::Account;
    const CONNECTED_PAGE: &str = "Connected to D&D Beyond";
    const RAIL_CONNECTED: &str = "Connected";
    const REMOTE: &str = "Add from D&D Beyond";
    const REMOTE_NAME: &str = "Salt Road";
    const FRESHNESS: &str = "Connected · Cached sheet · Fetched";
    const PAGES: [&str; 4] = ["Actions", "Spells", "Equipment", "Features"];
    const OVERVIEW: &str = "Overview";
    const MEMBER: &str = "Brannoch Vale";
    const SYNC_SECTION: &str = "Combat sync";
    const SYNC_TOGGLE: &str = "Sync combat stats with D&D Beyond";
    const STOPPED: &str = "Combat sync stopped";
    const FIRST_READ: &str = "First read";
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const LINK_TAB: &str = "Link and folder";
    const BLOCKED_REASON: &str = "Close the run to change campaigns.";
    const EDIT_DETAILS: &str = "Edit details";
    const RETRYING: &str = "renewing the session failed";
    const CONFLICT: &str = "Conflict";
    const RAIL_FAILING: &str = "Session failing";
    const RAIL_OFFLINE: &str = "Not connected";
    const STOPPED_ROW: &str = "Sync stopped";
    const NOT_SENT: &str = "Not sent";
    const NOTHING_TO_SEND: &str = "Iselle Marrowind";
    const CLOSE_RUN: &str = "Close run";
    const STRANDED: &str = "were not sent to D&D Beyond";

    pub fn run(d: &mut Driver<'_>) {
        d.app_mut().settings.simulate(Account::Connected);
        d.settle();
        go_settings(d);
        d.observe(
            "T10.3",
            d.visible_text(CONNECTED_PAGE) && d.visible_text(RAIL_CONNECTED),
            "With a session, the page and the rail both say Connected",
        );
        d.state("settings-connected", Sizes::All);
        d.click(REMOTE);
        d.expect(REMOTE_NAME);
        d.state("remote-campaign-list", Sizes::Default);
        d.goal("see the remote campaign list");
        d.observe(
            "Added campaigns",
            d.is_disabled("The Emberfall Marches · Added")
                && d.is_disabled("Quiet Harbor · Added")
                && !d.is_disabled(REMOTE_NAME),
            "A remote campaign already in the list, active or not, says so and is off",
        );
        d.click_containing(REMOTE_NAME);
        d.state("import-remote-campaign", Sizes::Default);
        d.observe(
            "T10.3",
            d.has_containing(REMOTE_NAME),
            "Choosing a remote campaign opens its import, with the name filled in",
        );
        d.click("Cancel");
        d.begin_task();

        go_party(d);
        d.click_containing(MEMBER);
        d.state("party-connected", Sizes::All);
        d.observe(
            "T7.2",
            d.visible_text(FRESHNESS),
            "Signed in: the sheet says it is connected, and how old it is",
        );
        for page in PAGES {
            d.click(page);
            d.observe(
                "T7.2",
                d.visible_text(FRESHNESS),
                format!("Signed in: freshness on {page}"),
            );
        }
        d.click(OVERVIEW);
        d.begin_task();

        // T7.3: sync stops on leaving, and the DM hears of it where they look next.
        d.click_containing(SYNC_SECTION);
        d.click(SYNC_TOGGLE);
        d.state("party-sync-on", Sizes::Default);
        go_library(d);
        let rail = d.visible_text("Party sync stopped");
        go_party(d);
        let page = d.visible_text(STOPPED);
        d.state("party-sync-stopped", Sizes::Default);
        d.observe(
            "T7.3",
            rail && page,
            format!("Leaving Party with sync on: the rail says so: {rail}. The page says so on return: {page}"),
        );
        d.click("Dismiss this message");
        d.observe(
            "T7.3",
            !d.has_containing(STOPPED),
            "The DM can dismiss the message",
        );
        d.begin_task();

        // A short network drop keeps the session, and says so.
        d.app_mut().settings.simulate(Account::RefreshFailing);
        d.settle();
        go_settings(d);
        let rail = d.rail_words();
        d.observe(
            "T10.3",
            d.visible_text(RETRYING)
                && rail.contains(RAIL_FAILING)
                && !rail.contains(RAIL_CONNECTED),
            format!("A failed refresh has its own words, on the page and in the rail: {rail}"),
        );
        d.state("refresh-failing", Sizes::Default);
        d.app_mut().settings.simulate(Account::Connected);
        d.settle();
        d.begin_task();

        // At the table: every party row states its sync, and Stop is offered.
        go_encounters(d);
        resume_run(d, RUN);
        d.click(SYNC_PARTY);
        d.state("run-connected", Sizes::All);
        d.observe(
            "T3.3",
            d.row_words(MEMBER).contains(FIRST_READ) && d.has(STOP_SYNC),
            "Signed in: a real click on Sync party puts its first read on the row",
        );

        // The session ends at the table, with a change on its way. The DM must know, and
        // the change must still count until they decide.
        d.begin_task();
        d.app_mut()
            .encounters
            .simulate_sync_state(MEMBER, crate::combat_sync::SyncState::Pending);
        d.settle();
        d.app_mut().encounters.simulate_session_end();
        d.app_mut().settings.simulate_session_expiry();
        d.settle();
        let row = d.row_words(MEMBER);
        let rail = d.rail_words();
        d.observe(
            "T10.3",
            row.contains(NOT_SENT)
                && d.row_words(NOTHING_TO_SEND).contains(STOPPED_ROW)
                && d.visible_text(&format!("{MEMBER}: kept in Splat, not sent"))
                && rail.contains(RAIL_OFFLINE),
            format!(
                "A session that ends during a run: the row says {row}, the header says so, and the rail says {rail}"
            ),
        );
        d.state("session-ended-during-run", Sizes::All);
        d.click(CLOSE_RUN);
        d.observe(
            "T10.3",
            d.visible_text(&format!("Changes for {MEMBER} {STRANDED}"))
                && !d.visible_text("Nothing is lost")
                && d.visible_text("Undo stops"),
            "Closing after that names the changes that were never sent, and promises only what it keeps",
        );
        d.state("close-with-unsent-changes", Sizes::Default);
        d.press(Key::Escape);
        // Damage recorded after the session ended. Reconnecting must not take it away.
        d.click_row(MEMBER);
        d.type_text("3");
        d.press(Key::Enter);
        let hp_in_run = d.row_hp(MEMBER);
        d.app_mut().settings.simulate(Account::Connected);
        d.settle();
        d.click(SYNC_PARTY);
        let row = d.row_words(MEMBER);
        d.observe(
            "T3.3",
            row.contains(FIRST_READ) && !d.visible_text(STOPPED_ROW),
            format!("After a reconnect, Sync party starts the stopped rows again: {row}"),
        );
        // The sheet still holds what it held before the outage.
        d.app_mut().encounters.simulate_read(MEMBER, 0);
        d.settle();
        let row = d.row_words(MEMBER);
        d.observe(
            "T3.3",
            row.contains(CONFLICT) && d.row_hp(MEMBER) == hp_in_run,
            format!(
                "Simulated read after a reconnect: the HP recorded during the outage stands ({hp_in_run:?}), and the row asks: {row}"
            ),
        );
        d.state("reconnect-conflict", Sizes::Default);
        d.begin_task();

        // The link cannot change under a run.
        d.click(STOP_SYNC);
        go_campaign(d);
        d.click(LINK_TAB);
        d.observe(
            "Locked link",
            d.visible_text(BLOCKED_REASON) && d.is_disabled(EDIT_DETAILS),
            "With a run open, the link tab states why it is locked, and its controls are",
        );
        d.state("link-tab-during-run", Sizes::Default);
        d.begin_task();
    }
}

/// G2: section 3's table, one row at a time. Each row sets state, leaves, and comes back.
pub mod nav_table {
    use super::*;
    const EVERYTHING: &str = "Everything";
    const SECTION: &str = "Creatures   ";
    const ENTRY_ROW: &str = "Aboleth · ";
    const ENTRY: &str = "Aboleth";
    const LOOKUP_QUERY: &str = "fireball";
    const LOOKUP_ENTRY: &str = "Fireball";
    const SELECTED: &str = "Cinder Wolf 7";
    const UNDO: &str = "Undo: next turn";
    const MEMBER: &str = "Iselle Marrowind";
    const SHEET_PAGE: &str = "Spells";
    const FILTER: &str = "Filter this page";
    const FILTER_TEXT: &str = "fire";
    const NOTE_ROW: &str = "Ash Exposure · ";
    const DRAFT_WORDS: &str = "Kept across pages. ";
    const DISCARD: &str = "Discard changes";
    const DRAFT_NAME: &str = "Deep Ford";
    const FIRST_SLOT: &str = "1. Brannoch Vale";
    const LINKED_QUERY: &str = "bard subclass";
    const LINKED_ROW: &str = "Level 3: Bard Subclass";
    const LINK: &str = "College of Lore";
    const LINKED_TITLE: &str = "Bard Subclass";
    const BACK: &str = "Back";
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const FIRST_READ: &str = "First read";
    const ACTIVE: &str = "Active campaign: The Emberfall Marches";
    const BACK_TO_RUN: &str = "Back to run";
    const ENCOUNTER: &str = "Shrine of Cinders";

    pub fn run(d: &mut Driver<'_>) {
        // T4.4 with more than one group: each saved run under its own encounter.
        let nodes = d.nodes();
        let grouped = [RUN, RUN_12, RUN_20].iter().all(|run| {
            let heading = nodes
                .iter()
                .position(|node| node.name == run.to_uppercase());
            let row = nodes
                .iter()
                .position(|node| node.name.starts_with(&format!("{run} · ")));
            matches!((heading, row), (Some(heading), Some(row)) if heading < row)
        });
        d.observe(
            "T4.4",
            grouped,
            "Three saved runs, each under its own encounter's heading",
        );

        // Open run first, because its strip moves every other page down.
        resume_run(d, RUN_20);

        // Library: scope, section, entry, scroll.
        go_library(d);
        d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
        d.click(EVERYTHING);
        d.click_containing(SECTION);
        d.click_starting(ENTRY_ROW);
        d.scroll_at(pos2(720.0, 400.0), 2);
        let entry_scroll = d.scroll_position_of_reader();
        d.begin_task();

        // Open run: selection, amount, scroll, undo, and sync.
        d.click(BACK_TO_RUN);
        d.app_mut().encounters.simulate_connection();
        d.settle();
        d.click(SYNC_PARTY);
        for _ in 0..8 {
            d.click(NEXT_TURN);
        }
        d.click_row(SELECTED);
        d.type_text("7");
        d.begin_task();

        // Party: member, page, filter.
        go_party(d);
        d.click_containing(MEMBER);
        d.click(SHEET_PAGE);
        d.type_into(FILTER, FILTER_TEXT);
        d.begin_task();

        // Lookup: last query and entry, opened over the run.
        d.click(BACK_TO_RUN);
        look_up(d, LOOKUP_QUERY);
        d.expect(LOOKUP_ENTRY);
        // Measured with Lookup open, because it changes the roster's width.
        let run_y = d.row_node(SELECTED).map(|row| row.rect.min.y);

        // Away, then back to each.
        go_settings(d);
        d.click(BACK_TO_RUN);
        let amount = d.field_text("Amount");
        let selected = participant_is_selected(d, SELECTED);
        let row_y = d.row_node(SELECTED).map(|row| row.rect.min.y);
        let undo = d.has_containing(UNDO);
        d.observe(
            "G2",
            amount.as_deref() == Some("7") && selected && row_y == run_y && undo,
            format!("Run: amount {amount:?}, selected {selected}, row {row_y:?} was {run_y:?}, undo {undo}"),
        );
        d.observe(
            "G2",
            d.row_node("Brannoch Vale")
                .is_some_and(|row| row.name.contains(FIRST_READ)),
            "Run: sync stays on across pages",
        );
        d.observe(
            "G2",
            d.field_text(SEARCH_LOOKUP).as_deref() == Some(LOOKUP_QUERY)
                && d.has_label(LOOKUP_ENTRY),
            "Lookup: the query and the entry are as they were",
        );
        d.press(Key::Escape);

        go_library(d);
        let scope = d.visible_text(EVERYTHING);
        let entry = d.has_label(ENTRY);
        let scroll = d.scroll_position_of_reader();
        d.observe(
            "G2",
            scope && entry && scroll == entry_scroll && entry_scroll.is_some_and(|y| y < 300.0),
            format!(
                "Library: scope {scope}, entry {entry}, scroll {scroll:?} was {entry_scroll:?}"
            ),
        );
        go_party(d);
        d.observe(
            "G2",
            d.has_label(MEMBER) && d.field_text(FILTER).as_deref() == Some(FILTER_TEXT),
            "Party: member, page and filter are as they were",
        );
        d.observe(
            "G2",
            d.has(ACTIVE),
            "Active campaign: the selector still names it",
        );

        // Reading history.
        go_library(d);
        search_library(d, LINKED_QUERY);
        d.click_containing(LINKED_ROW);
        d.click(LINK);
        go_settings(d);
        go_library(d);
        d.click(BACK);
        d.observe(
            "G2",
            d.in_view(LINKED_TITLE),
            "Reading history: Back still returns to the entry before the link",
        );
        d.click(CLEAR_SEARCH);
        d.begin_task();

        // Close the run, to reach the two drafts.
        d.click(BACK_TO_RUN);
        d.click(STOP_SYNC);
        close_run(d);

        // Library draft.
        go_library(d);
        open_local_package(d, MODULE);
        d.click_starting(NOTE_ROW);
        d.click(EDIT_MARKDOWN);
        d.type_into_editor(DRAFT_WORDS);
        go_settings(d);
        go_library(d);
        d.observe(
            "G2",
            d.editor_text()
                .is_some_and(|text| text.contains(DRAFT_WORDS)),
            "Library draft: its words are as they were",
        );
        d.click(DISCARD);

        // Encounter draft.
        go_encounters(d);
        d.click(NEW_ENCOUNTER);
        d.type_text(DRAFT_NAME);
        d.click(ADD_PARTY);
        go_settings(d);
        go_encounters(d);
        d.observe(
            "G2",
            d.field_text(&format!("Encounter name: {DRAFT_NAME}"))
                .as_deref()
                == Some(DRAFT_NAME)
                && d.has_label(FIRST_SLOT),
            "Encounter draft: its name and rows are as they were",
        );
        d.click(DISCARD);
        d.goal("walk section 3's table");

        // G3: every control in the rail goes somewhere of its own.
        go_library(d);
        let mut titles = Vec::new();
        for name in d.rail_page_buttons() {
            d.click(&name);
            let title = d.page_title();
            titles.push((name, title));
        }
        let mut distinct: Vec<&Option<String>> = titles.iter().map(|(_, title)| title).collect();
        distinct.sort();
        distinct.dedup();
        d.observe(
            "G3",
            titles.len() >= 5
                && distinct.len() == titles.len()
                && titles.iter().all(|(_, t)| t.is_some()),
            format!("Each control in the rail opens a page with its own title: {titles:?}"),
        );

        // G1: a disabled control says why, on the pages that have the most of them.
        go_encounters(d);
        d.click(NEW_ENCOUNTER);
        let mut silent = d.disabled_without_reason();
        d.click(DISCARD);
        go_settings(d);
        silent.extend(d.disabled_without_reason());
        go_campaign(d);
        silent.extend(d.disabled_without_reason());
        go_party(d);
        silent.extend(d.disabled_without_reason());
        go_library(d);
        search_library(d, &NOTE_ROW.replace(" · ", "").to_lowercase());
        d.click_containing(NOTE_ROW);
        silent.extend(d.disabled_without_reason());
        // The run and everything it locks: the footer, the Encounters list, the selector.
        go_encounters(d);
        resume_run(d, RUN);
        silent.extend(d.disabled_without_reason());
        go_encounters(d);
        silent.extend(d.disabled_without_reason());
        go_settings(d);
        silent.extend(d.disabled_without_reason());
        // The run's tabs, then a fresh run's setup and a finished run's end.
        d.click(BACK_TO_RUN);
        for tab in ["Conditions", "Record", "Log", "Prep"] {
            d.click(tab);
            silent.extend(d.disabled_without_reason());
        }
        d.click("End encounter…");
        d.click("End encounter");
        silent.extend(d.disabled_without_reason());
        d.click("Close run");
        run_saved_encounter(d, ENCOUNTER);
        silent.extend(d.disabled_without_reason());
        close_run(d);
        // A Library draft, and a dialog above it.
        go_library(d);
        d.click_containing(NOTE_ROW);
        d.click("Edit Markdown");
        d.type_into_editor(DRAFT_WORDS);
        silent.extend(d.disabled_without_reason());
        d.click(DISCARD);
        let behind: Vec<String> = d.nodes().into_iter().map(|node| node.name).collect();
        d.click("Copy to…");
        silent.extend(d.disabled_without_reason_except(&behind));
        d.press(Key::Escape);
        d.observe(
            "G1",
            silent.is_empty(),
            format!(
                "Disabled controls with no reason, in words, on hover, across 15 views: {silent:?}"
            ),
        );
    }
}

/// W11: notes, templates and a timeline that stands apart from them.
pub mod session_notes {
    use super::*;

    const TITLE: &str = "Ash on the Road";
    const LONG_TITLE: &str =
        "The Long Road from Emberfall Through the Ash Marshes to the Drowned Mill";
    const NEW_EVENTS_COMBO: usize = 1;
    const UNDO_NEXT_TURN: &str = "Undo: next turn";
    const ADD_DAY: &str = "Add Day";
    const ADD_LONG_REST: &str = "Add Long rest";
    const ADD_SHORT_REST: &str = "Add Short rest";
    const ADD_TRAVEL: &str = "Add Travel";
    const ADD_EVENT: &str = "Add Major event";

    pub fn run(d: &mut Driver<'_>) {
        d.click(NAV_SESSIONS);
        d.expect("No session notes yet");
        d.state("sessions-empty", Sizes::Default);
        d.check_tab_order("sessions-empty");

        d.begin_task();
        d.click("New session");
        d.state("session-form", Sizes::Default);
        d.type_into("Session title", TITLE);
        d.click("Create session");
        d.expect(&format!("Session 1: {TITLE}"));
        d.goal_within("T11.1", "start a titled session from a template", 4);
        d.state("session-write", Sizes::AllAndHiDpi);
        let filled = d
            .editor_text()
            .is_some_and(|text| text.contains("Recap of last time") && text.contains(TITLE));
        d.observe(
            "T11.1",
            filled,
            "The new note holds the template, filled in",
        );

        d.type_into_editor("\nThe road was covered in ash, and the wolves followed us.\n");
        d.press_ctrl(Key::S);
        d.expect("Notes saved");
        d.state("session-written", Sizes::Default);

        d.click("Read");
        d.state("session-read", Sizes::All);

        // A rest, on the session's own page, with no step away from it.
        d.click("Events");
        d.state("session-events-empty", Sizes::Default);
        d.begin_task();
        d.click(ADD_LONG_REST);
        d.goal_within("T11.2", "put a long rest on the timeline", 2);
        d.expect("Long rest added to Session 1.");
        d.state("session-event-added", Sizes::Default);
        d.click(ADD_SHORT_REST);
        d.observe(
            "T11.2",
            d.has_containing("Short rest added to Session 1."),
            "An event added here says so, in words",
        );
        d.click("Undo add");
        d.observe(
            "T11.2",
            !d.has("Short rest") && d.has_containing("Removed"),
            "A mis-press has one step of undo, and that has one too",
        );
        d.click("Undo remove");
        d.expect("Short rest");
        d.click("Short rest");
        d.scroll_until(pos2(780.0, 400.0), "Remove this event", 10);
        d.click("Remove this event");
        d.click(NAV_SESSIONS);
        d.click(ADD_TRAVEL);
        d.click(ADD_EVENT);
        d.state("session-events", Sizes::Default);
        d.click("Add to notes: Long rest");
        d.expect("In notes");
        d.observe(
            "T11.2",
            !d.has("Undo add"),
            "Undo goes at the next thing the DM does, so it cannot undo the wrong event",
        );
        d.state("session-event-in-notes", Sizes::Default);
        d.click("Write");
        let linked = d.editor_text().is_some_and(|text| {
            let what = text.find("## What happened");
            let next = text.find("## People and places");
            let link = text.find("[[Timeline#^");
            matches!((what, link, next), (Some(a), Some(b), Some(c)) if a < b && b < c)
        });
        d.observe(
            "T11.4",
            linked,
            "An event's link goes under What happened in the note",
        );
        d.press_ctrl(Key::S);
        d.click("Read");
        d.expect("ON THE TIMELINE");
        d.state("session-read-linked", Sizes::Default);
        let reads_as_words = !d.visible_text("[[Timeline");
        d.observe(
            "T11.4",
            reads_as_words,
            "The Reader shows the link as its words",
        );

        // The timeline is a place of its own, with every kind of event.
        d.click(NAV_TIMELINE);
        d.expect("Long rest");
        d.state("timeline-session-events", Sizes::All);
        d.check_tab_order("timeline");
        d.click(ADD_DAY);
        d.type_text("Harvest Eve");
        // A day opens what follows it, so move it to the top.
        for _ in 0..3 {
            d.click("Move earlier");
        }
        d.state("timeline-day-added", Sizes::All);
        d.click_containing("Long rest · Day 1 · Session 1");
        d.expect("LINK");
        d.replace_text_in("Title", "Rest at the mill");
        d.type_into("Where", "Old mill");
        d.press_ctrl(Key::S);
        d.expect("Copy link");
        d.click("Copy link");
        d.expect("Copied");
        d.expect("MENTIONED IN");
        d.state("timeline-event", Sizes::AllAndHiDpi);
        d.observe(
            "T11.4",
            d.has_containing(&format!("Session 1: {TITLE}")),
            "An event lists the session notes that link to it",
        );
        // The Reader follows the event's name now, not the name when it was linked.
        d.click_containing("Rest at the mill");
        d.click("Open this session");
        d.click("Read");
        d.observe(
            "T11.4",
            d.visible_text("Rest at the mill"),
            "A renamed event reads under its new name in the note",
        );
        d.click("Write");
        d.click(NAV_TIMELINE);
        d.click("Rests");
        d.state("timeline-filtered", Sizes::Default);
        d.observe(
            "T11.3",
            d.is_disabled("Move earlier"),
            "A filtered list cannot move an event past one it hides",
        );
        d.click("All");
        d.scroll_until(pos2(780.0, 400.0), "Remove this event", 10);
        d.click("Remove this event");
        d.expect("Undo remove");
        d.state("timeline-removed", Sizes::Default);
        d.click_containing("Travel ·");
        d.observe(
            "T11.4",
            d.has("Undo remove"),
            "Selecting another event keeps the undo for a removal",
        );
        d.click("Undo remove");
        d.expect("Rest at the mill");

        // Back to the note from the event, and the file is readable outside Splat.
        d.click_containing("Rest at the mill");
        d.click("Open this session");
        d.expect(&format!("Session 1: {TITLE}"));

        // Words that could not be saved survive anything that would open the note again.
        d.click("Write");
        d.type_into_editor("\nMy words during the conflict.\n");
        d.app_mut().sessions.simulate_disk_change();
        d.press_ctrl(Key::S);
        d.expect("Changed on disk");
        d.expect("Overwrite with my draft");
        d.click("Show disk version");
        d.observe(
            "T11.4",
            d.has_containing("Written in Obsidian."),
            "A conflict shows the file's words, so the DM chooses knowing both",
        );
        d.state("note-conflict", Sizes::Default);
        d.click(NAV_TIMELINE);
        d.observe(
            "T11.4",
            d.has_containing("Notes not saved"),
            "A note that could not be saved stays in the rail on other pages",
        );
        d.state("note-conflict-elsewhere", Sizes::Default);
        d.click("Notes not saved");
        d.expect("Overwrite with my draft");
        d.click(NAV_TIMELINE);
        d.click_containing("Rest at the mill");
        d.click("Open this session");
        d.observe(
            "T11.4",
            d.editor_text()
                .is_some_and(|text| text.contains("My words during the conflict.")),
            "Opening the session again keeps the words that could not be saved",
        );
        d.click("Overwrite with my draft");
        d.expect("Notes saved");

        // A long title wraps, and the controls stay where they are.
        d.click("Edit session details");
        d.replace_text_in("Date", "Nov 2");
        d.click("Save details");
        d.expect("Enter the date as YYYY-MM-DD.");
        d.replace_text_in("Date", crate::hermetic::today().as_str());
        d.replace_text_in("Session title", LONG_TITLE);
        d.click("Save details");
        d.observe(
            "T11.1",
            d.editor_text()
                .is_some_and(|text| text.contains(&format!("# Session 1: {LONG_TITLE}"))),
            "Renaming a session moves the heading the template made",
        );
        d.state("session-long-title", Sizes::All);
        d.click("Edit session details");
        d.replace_text_in("Session title", TITLE);
        d.click("Save details");

        d.click("Templates");
        d.expect("Standard session");
        d.state("templates-built-in", Sizes::All);
        d.click("Customize this template");
        d.expect("This campaign");
        d.state("templates-own", Sizes::Default);
        d.type_into_editor("Edited in the template.\n");
        d.click("New template");
        d.type_into("Template name", "Downtime");
        d.state("templates-new", Sizes::Default);
        d.click("Create template");
        d.expect("Downtime");
        d.click("Delete this template");
        d.expect("Undo delete");
        d.state("templates-deleted", Sizes::Default);
        d.click("Undo delete");
        d.expect("Downtime");

        d.click("Folder");
        d.expect("SESSION FOLDER");
        d.state("session-folder", Sizes::Default);
        d.click("Session notes");

        // Reading last week's note does not send tonight's run there.
        d.click(NAV_TIMELINE);
        d.open_combo_at(NEW_EVENTS_COMBO);
        d.click("No session");
        d.click(NAV_SESSIONS);
        d.click("New session");
        d.type_into("Session title", "Tonight");
        d.click("Create session");
        d.expect("Session 2: Tonight");
        d.observe(
            "T11.1",
            d.editor_text()
                .is_some_and(|text| text.contains("Edited in the template.")),
            "A template edited a moment ago is what a new session starts from",
        );
        d.click_containing(&format!("Session 1: {TITLE}"));
        d.click("Events");
        d.observe(
            "T11.3",
            d.has_containing("These go to Session 1, not the session being played (Session 2)."),
            "Adding an event to an older session says so before it happens",
        );
        d.state("session-events-older", Sizes::Default);
        d.click("Write");
        d.observe(
            "T11.8",
            d.has_containing(
                "You are writing in Session 1, not the session being played (Session 2).",
            ),
            "Writing in an older note says so",
        );
        d.state("session-write-older", Sizes::Default);
        go_encounters(d);
        resume_run(d, RUN);

        // T11.8: a note during a run, with the roster still in view.
        d.begin_task();
        d.press_ctrl(Key::J);
        d.type_text("The ogre fled over the bridge.");
        d.goal_within("T11.8", "take a note during a run", 2);
        d.expect("Close notes");
        d.observe(
            "T11.8",
            d.in_view(NEXT_TURN)
                && d.editor_text()
                    .is_some_and(|text| text.contains("The ogre fled")),
            "The note goes in the session being played, and the run stays in view",
        );
        d.state("run-with-notes", Sizes::All);
        let wide_screen = d.at_size((1536.0, 864.0), |d| d.in_view(NEXT_TURN) && !d.has("Prep"));
        d.observe(
            "T11.8",
            wide_screen,
            "At 1536 px, a panel beside the run leaves the roster its room",
        );
        // A click on Next turn moves the caret, so what is typed next must not change the run.
        let turn = current_participant(d);
        d.click(NEXT_TURN);
        let moved = current_participant(d);
        d.observe(
            "T11.8",
            moved != turn,
            "Next turn still works with Notes open",
        );
        // Words typed next go to the note. A digit and Enter must not become damage.
        let hp = |d: &Driver<'_>| {
            d.nodes()
                .iter()
                .map(|node| node.name.clone())
                .filter(|name| name.contains("HP"))
                .collect::<Vec<_>>()
        };
        let before = hp(d);
        d.type_text("Ogre took 12 from Rohan");
        d.press(Key::Enter);
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("Ogre took 12 from Rohan")),
            "After Next turn, what the DM types still goes to the note",
        );
        d.observe(
            "T11.8",
            before == hp(d),
            "A number typed for the note never becomes damage",
        );
        d.press(Key::N);
        d.press(Key::U);
        d.observe(
            "T11.8",
            current_participant(d) == moved,
            "With Notes open, stray typing never changes the run",
        );
        d.observe(
            "T11.8",
            !d.has("Next turn N"),
            "With Notes open, the run shows no key hint that would not work",
        );
        d.click(UNDO_NEXT_TURN);
        d.observe(
            "T11.8",
            d.editor_text().is_some_and(|text| {
                let what = text.find("## What happened");
                let note = text.find("The ogre fled");
                let next = text.find("## People and places");
                matches!((what, note, next), (Some(a), Some(b), Some(c)) if a < b && b < c)
            }),
            "A note typed at the table goes under What happened",
        );
        // A click on a row takes the keyboard to Amount for one number, and then it goes back.
        d.click_row("Ember Cultist");
        let full = hp(d);
        d.type_text("7");
        d.press(Key::Enter);
        let hurt = hp(d);
        d.observe(
            "T11.8",
            full != hurt,
            "With Notes open, a click on a row still lets a number apply",
        );
        d.type_text("The cultist took 12 more");
        d.press(Key::Enter);
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("The cultist took 12 more")),
            "After a number applies, what the DM types goes to the note",
        );
        d.observe(
            "T11.8",
            hurt == hp(d),
            "The words after a number never become more damage",
        );
        d.click_containing("Undo: 7 damage");
        d.type_text("Undone.");
        d.observe(
            "T11.8",
            full == hp(d) && d.editor_text().is_some_and(|text| text.contains("Undone.")),
            "After Undo by its button, what the DM types goes to the note",
        );
        // After a click on a row, the DM can go on writing. A digit in those words is not damage.
        d.click_row("Ember Cultist");
        let whole = hp(d);
        // One key at a time, as a person types, because the keyboard moves on the way.
        for key in "2 wolves flee".chars() {
            d.type_text(&key.to_string());
        }
        d.press(Key::Enter);
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("2 wolves flee")),
            "After a click on a row, words typed go to the note",
        );
        d.observe(
            "T11.8",
            whole == hp(d),
            "A number in those words never becomes damage",
        );
        // Lookup takes the place of the notes, and they come back with it closed.
        let turn = current_participant(d);
        d.type_text("Iselle casts ");
        d.press_ctrl(Key::K);
        d.observe(
            "T11.8",
            !d.has("Close notes"),
            "One panel beside the page at a time",
        );
        d.press(Key::Escape);
        d.type_text("Bless");
        d.press(Key::N);
        d.observe(
            "T11.8",
            d.has("Close notes")
                && d.editor_text()
                    .is_some_and(|text| text.contains("Iselle casts Bless"))
                && current_participant(d) == turn,
            "After Lookup, the sentence goes on where it was, and typing never changes the run",
        );
        // A stray click takes the keyboard from the note, and the words still go there.
        d.harness_focus_nothing();
        d.type_text(" ");
        d.type_text("too");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("Iselle casts Bless too")),
            "After a stray click, what the DM types still goes on in the note",
        );
        // Tab indents. It must not walk to Next turn, where the next space would press it.
        d.press(Key::Tab);
        d.type_text(" ");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("Bless too\t "))
                && current_participant(d) == turn,
            "Tab and space in the note stay in the note, and the turn does not move",
        );
        // A step changes nothing in the note until the DM types.
        let unchanged = d.editor_text();
        d.click(NEXT_TURN);
        d.observe(
            "T11.8",
            unchanged == d.editor_text(),
            "A click on Next turn alone leaves the note as it was",
        );
        d.type_text("Next thought.");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("\n\nNext thought.")),
            "What is typed after a step starts a line of its own",
        );
        d.click(NEXT_TURN);
        d.press(Key::Backspace);
        d.type_text("!");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("Next thought!")),
            "A correction after a step goes on with the same line",
        );
        d.click_containing("Undo");
        d.click_containing("Undo");
        // What the DM asks for in the side panel must not stay hidden behind the notes.
        d.click_row("Ember Cultist");
        d.click("Conditions for this participant");
        d.observe(
            "T11.8",
            !d.has("Close notes") && d.has_containing("Poisoned"),
            "Conditions show, in place of the notes, when both have no room",
        );
        d.press_ctrl(Key::J);
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("The cultist took 12 more")),
            "Notes closed that way keep their words",
        );
        d.click_row("Iselle Marrowind");
        d.type_text("99");
        d.press(Key::Enter);
        d.click("Death saves");
        d.observe(
            "T11.8",
            !d.has("Close notes") && d.has_containing("0 successes"),
            "Death saves show, in place of the notes, when both have no room",
        );
        d.click_containing("Undo: 99 damage");
        d.press_ctrl(Key::J);
        // The panel must not follow the page to an older note.
        d.click(NAV_SESSIONS);
        d.click_containing(&format!("Session 1: {TITLE}"));
        d.observe(
            "T11.8",
            !d.has("Close notes"),
            "Opening an older note closes the panel, so that no word goes there",
        );
        d.press_ctrl(Key::J);
        d.observe(
            "T11.8",
            d.has_containing("Session 2: Tonight") && !d.has("Close notes"),
            "Notes open on the session being played. On this page the page itself holds them",
        );
        d.state("sessions-with-notes", Sizes::All);
        d.click(BACK_TO_RUN);
        d.press_ctrl(Key::J);
        // Coming back from the page, where the note can change, the caret starts fresh.
        d.click(NAV_SESSIONS);
        d.click(BACK_TO_RUN);
        d.type_text("The wolves returned.");
        d.observe(
            "T11.8",
            d.editor_text()
                .is_some_and(|text| text.contains("The wolves returned.\n\n## People and places")),
            "A second note keeps the next heading on its own line",
        );
        d.press(Key::Escape);
        d.observe(
            "T11.8",
            !d.has("Close notes"),
            "Escape closes the notes beside the run",
        );
        // A note that looks like last week's says so where the words go.
        d.click(NAV_SESSIONS);
        d.click("Edit session details");
        d.replace_text_in("Date", "2025-12-25");
        d.click("Save details");
        d.click(BACK_TO_RUN);
        d.press_ctrl(Key::J);
        d.observe(
            "T11.8",
            d.has_containing("This session is dated 2025-12-25."),
            "The panel warns when the session being played looks like an old one",
        );
        d.state("run-with-old-note", Sizes::Default);
        d.observe(
            "T11.8",
            d.has_containing("Old session"),
            "During a run, the rail says that events are going to an old session",
        );
        d.click("Play this one");
        d.observe(
            "T11.8",
            !d.has_containing("This session is dated"),
            "A session prepared earlier can be played with one press",
        );
        d.press(Key::Escape);

        d.click(NAV_TIMELINE);
        d.expect(RUN);
        d.state("timeline-with-run", Sizes::All);
        // Beside a panel at the minimum window, the list keeps its room.
        d.press_ctrl(Key::K);
        d.state("timeline-beside-lookup", Sizes::All);
        d.click(NAV_SESSIONS);
        d.expect("All sessions");
        d.state("session-beside-lookup", Sizes::Default);
        d.click("All sessions");
        d.expect("Session 1: ");
        d.state("session-list-beside-lookup", Sizes::Default);
        d.click_containing("Session 2: Tonight");
        d.expect("All sessions");
        d.press_ctrl(Key::K);
        d.click(NAV_TIMELINE);
        d.observe(
            "T11.3",
            d.has_containing("Encounter · Round 2 · Session 2"),
            "A run went to the newest session, with no step, though an older note was read",
        );
        d.goal("use the timeline during a run");

        // A new session changes where the next events go, and the open run stays.
        d.click(NAV_SESSIONS);
        d.click("New session");
        d.type_into("Session title", "Later that night");
        d.press(Key::Enter);
        d.observe(
            "T11.3",
            d.has_containing("stays in Session 2"),
            "A new session during a run says that the run's event stays where it was",
        );
        d.state("new-session-during-run", Sizes::Default);

        // Setup beside the notes. Each digit must stay in its field.
        d.click(BACK_TO_RUN);
        d.press_ctrl(Key::J);
        d.expect("Close notes");
        d.app_mut()
            .encounters
            .simulate_sync_conflict("Iselle Marrowind");
        d.settle();
        d.click("Review");
        d.observe(
            "T11.8",
            !d.has("Close notes"),
            "A conflict's controls show, in place of the notes, when both have no room",
        );
        d.click("Use D&D Beyond values");
        d.app_mut().encounters.simulate_session_end();
        d.settle();
        d.click("Close run");
        if d.has("Close anyway") {
            d.click("Close anyway");
        } else {
            d.click("Close now");
        }
        run_saved_encounter(d, "Shrine of Cinders");
        d.press_ctrl(Key::J);
        let note = d.editor_text();
        d.click("Initiative for Iselle Marrowind");
        d.type_text("1");
        d.type_text("7");
        d.observe(
            "T11.8",
            note.is_some() && note == d.editor_text(),
            "Initiative typed beside the notes stays in its field",
        );
        d.state("run-setup-with-notes", Sizes::Default);
        d.click("Initiative for Oren Thistlewick");
        d.type_text("2");
        d.type_text(" ");
        d.type_text("wolves");
        d.observe(
            "T11.8",
            d.editor_text().is_some_and(|text| text.contains(" wolves")),
            "A letter typed in an initiative field goes to the note",
        );
        for (name, total) in [
            ("Brannoch Vale", "17"),
            ("Kestrel Dunmoor", "15"),
            ("Oren Thistlewick", "14"),
            ("Sable Harrowgate", "13"),
            ("Ember Cultist 1", "12"),
            ("Ember Cultist 2", "11"),
        ] {
            d.click(&format!("Initiative for {name}"));
            d.type_text(total);
        }
        d.observe(
            "T11.8",
            d.has_containing("Everyone is ready. 2 tied."),
            "Notes hide the order of ties, so setup says that there are ties",
        );
        d.click("Turn order");
        d.observe(
            "T11.8",
            !d.has("Close notes") && d.has_containing(" ties"),
            "What Notes hide in setup is one click away",
        );
    }
}

/// W12: record a session, and process it later with the DM's chosen services.
pub mod session_recording {
    use super::*;
    use crate::{
        processing::{CANCELLED, Progress},
        recording::Health,
    };

    const TITLE: &str = "The Bridge";
    const RECORDING: &str = "19:00";
    const WARNING: &str = "The second pass returned no transcript, so the first pass's transcript stands without a check.";

    pub fn run(d: &mut Driver<'_>) {
        d.click(NAV_SESSIONS);
        new_session(d, TITLE);

        // A stale input stops here, rather than record from another microphone.
        d.app_mut().processing.simulate_input("gone.mic");
        d.settle();
        d.observe(
            "T11.5",
            d.is_disabled("Record this session"),
            "A chosen input that is gone blocks recording",
        );
        d.app_mut().processing.simulate_input("table.mic");
        d.settle();

        // T11.5: nothing records without a stated step.
        d.click("Record this session");
        d.expect("Start recording");
        d.state("record-consent", Sizes::Default);
        d.observe(
            "T11.5",
            d.has_containing(&format!("Record Session 1: {TITLE}.")),
            "The step names the session the audio goes to",
        );
        d.observe(
            "T11.5",
            d.has_containing("Tell everyone at the table first"),
            "Recording starts only after the DM is told to tell the table",
        );
        d.click("Start recording");
        d.expect("Stop recording");
        d.expect("Waiting for sound");
        d.state("recording-starting", Sizes::Default);
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Hearing);
        d.settle();
        d.expect("Hearing sound");
        d.state("recording-hearing", Sizes::AllAndHiDpi);
        d.click("Recordings (1)");
        d.expect("· Recording");
        d.state("recording-listed", Sizes::Default);
        d.click("Write");

        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Silent);
        d.settle();
        d.expect("Silent for over a minute");
        // On the run, the same words must not paint over a long Undo.
        go_encounters(d);
        resume_run_from_cold_start(d);
        d.click_row("Ember Cultist");
        d.type_text("7");
        d.press(Key::Enter);
        d.observe(
            "T11.5",
            d.has_containing("Undo: 7 damage") && d.has_containing("Recording silence"),
            "A silent input shows on the run, beside a long Undo",
        );
        d.state("run-silent", Sizes::Default);
        d.click_containing("Undo: 7 damage");
        close_run(d);
        d.click_containing("Sessions · Silent");
        d.expect("Silent for over a minute");
        d.expect("Check the input in Settings");
        d.state("recording-silent", Sizes::Default);
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stalled);
        d.settle();
        d.expect("Not saving audio after");
        d.state("recording-stalled", Sizes::Default);
        d.observe(
            "T11.5",
            d.has("Record again"),
            "A stall offers one press to record again",
        );
        d.observe(
            "T11.5",
            d.has_containing("not saving audio after")
                && d.has_containing("What was saved is kept"),
            "A stall takes a banner, in words, on the session's page too",
        );

        // A failing recording reaches the DM on any other page.
        go_library(d);
        d.observe(
            "T11.5",
            d.has_containing("Sessions · Not saving") && d.has_containing("Not saving audio"),
            "A stall shows in the rail on other pages",
        );
        d.state("recording-stalled-elsewhere", Sizes::Default);
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Hearing);
        d.settle();
        d.observe(
            "T11.5",
            d.has_containing("Rec 00:00:00") && d.has_containing("Recording"),
            "Recording, with its time, stays in the rail on other pages",
        );
        d.state("recording-on-another-page", Sizes::Default);
        // A close does not end it by surprise.
        d.request_close();
        d.expect("Keep working");
        d.state("recording-close-prompt", Sizes::Default);
        d.click("Keep working");
        d.observe(
            "T11.5",
            d.page_title().as_deref() == Some("Library"),
            "Keep working leaves the DM on the page they were on",
        );
        d.click_containing("Sessions · ");
        d.expect("Stop recording");
        d.click("Stop recording");
        d.expect("saved: 2026-01-01 19:00");

        d.click("Recordings (1)");
        d.expect(RECORDING);
        d.observe(
            "T11.5",
            d.has_containing("Not processed"),
            "Stop keeps the recording in the session",
        );
        d.state("recording-saved-no-key", Sizes::Default);

        // T11.6: the default services are named, and what is missing is said.
        d.observe(
            "T11.6",
            d.has_containing("Save an API key for this speech service"),
            "Process states what is missing before it can run",
        );
        d.click("Open Settings");
        d.expect("Speech service");
        d.state("processing-settings", Sizes::All);
        d.observe(
            "T11.6",
            d.field_text("LLM model").as_deref() == Some("gpt-6.1-sol")
                && d.field_text("Speech model").as_deref() == Some("gpt-transcribe"),
            "The defaults are Sol through the Codex CLI, after OpenAI's gpt-transcribe",
        );
        d.scroll_until(pos2(600.0, 400.0), "LLM service", 12);
        d.click("LLM service");
        d.state("processing-llm-choices", Sizes::Default);
        d.click("Anthropic");
        d.state("processing-settings-anthropic", Sizes::Default);
        d.click("LLM service");
        d.click("Codex CLI, GPT-6.1 Sol");
        d.click("Account and campaigns");

        d.app_mut().processing.simulate_keys();
        d.settle();
        // A key saved for one service is not sent to another.
        d.click("Recording and processing");
        d.scroll_until(pos2(600.0, 400.0), "LLM service", 12);
        d.click("LLM service");
        d.click("Anthropic");
        d.observe(
            "T11.6",
            d.has_containing("for another service"),
            "A saved key does not follow the DM to a different service",
        );
        d.state("key-for-another-service", Sizes::Default);
        d.click("LLM service");
        d.click("Codex CLI, GPT-6.1 Sol");
        d.click("Account and campaigns");
        d.click(NAV_SESSIONS);
        d.expect("names to api.openai.com");
        d.state("recording-ready", Sizes::Default);
        d.observe(
            "T11.6",
            d.has_containing("events go to Codex (gpt-6.1-sol) twice"),
            "The page says where the audio and the words will go",
        );

        // Each state of a job, in words.
        d.app_mut()
            .sessions
            .simulate_job(Progress::Transcribing { part: 3, of: 24 });
        d.settle();
        d.expect("Transcribing part 3 of 24");
        d.state("processing-speech", Sizes::Default);
        d.request_close();
        d.expect("processing it again goes on from there");
        d.state("processing-close-prompt", Sizes::Default);
        d.click("Keep working");
        d.app_mut()
            .sessions
            .simulate_job_end(Err(CANCELLED.to_owned()));
        d.settle();
        d.expect("Cancelled");
        d.state("processing-cancelled", Sizes::Default);
        d.app_mut().sessions.simulate_job(Progress::Asking {
            pass: crate::processing::Pass::Review,
            seconds: 42,
        });
        d.settle();
        d.expect("Pass 2 of 2: checking the first pass, 42 s");
        d.state("processing-second-pass", Sizes::Default);
        d.app_mut().sessions.simulate_job_end(Err(
            "The LLM is busy, or your limit is reached. Wait, then process it again.".to_owned(),
        ));
        d.settle();
        // A failure reaches the DM on any other page, and a routine message cannot hide it.
        go_library(d);
        d.observe(
            "T11.6",
            d.has_containing("Processing failed"),
            "A failed job stays in the rail on other pages",
        );
        d.state("processing-failed-elsewhere", Sizes::Default);
        d.click(NAV_SESSIONS);
        d.expect("your limit is reached");
        d.state("processing-failed", Sizes::Default);

        // Trying again answers the failure, so no old word stays beside a new result.
        d.click_containing("Process:");
        d.expect("is processed");
        d.observe(
            "T11.6",
            !d.has_containing("Processing failed") && !d.has_containing("your limit is reached"),
            "A successful retry clears the failure",
        );

        // T11.7: a marker is a suggestion until the DM adds it.
        d.expect("EVENTS MARKED IN THIS RECORDING: 2");
        d.state("recording-processed", Sizes::AllAndHiDpi);
        d.click(NAV_TIMELINE);
        d.observe(
            "T11.7",
            !d.has_containing("Rest at the mill"),
            "Markers do not reach the timeline by themselves",
        );
        d.click(NAV_SESSIONS);
        d.click_containing("Add to timeline: Rest at the mill");
        d.expect("On the timeline");
        d.state("marker-added", Sizes::Default);
        d.observe(
            "T11.7",
            d.has("Add to timeline: Wolves at the bridge (0)"),
            "Adding one marker leaves the other a suggestion",
        );

        // Each text reads in the whole pane, a part at a time, and a marker opens its place.
        d.click("00:10:00");
        d.expect("Part 2 of 2");
        d.expect("Long rest: Rest at the mill");
        d.state("marked-transcript", Sizes::All);
        d.click("Earlier part");
        d.expect("Part 1 of 2");
        d.click("Back to recordings");
        d.click("What the check changed");
        d.expect("Restored");
        d.state("second-pass-review", Sizes::Default);
        d.click("Back to recordings");
        d.click("Summary");
        d.state("recording-summary", Sizes::Default);
        d.click("Back to recordings");
        d.click(NAV_TIMELINE);
        d.observe(
            "T11.7",
            d.has_containing("Rest at the mill"),
            "A marker the DM adds is on the timeline, with its place and length",
        );
        d.click(NAV_SESSIONS);

        // A warning lasts as long as the result, past the message and the rail.
        d.app_mut().sessions.simulate_job(Progress::Asking {
            pass: crate::processing::Pass::Review,
            seconds: 9,
        });
        d.settle();
        go_library(d);
        d.app_mut().sessions.simulate_job_end_warned(WARNING);
        d.settle();
        d.observe(
            "T11.6",
            d.has_containing("Processed: check it"),
            "A result with a warning does not read as done on other pages",
        );
        d.state("processing-warned-elsewhere", Sizes::Default);
        d.click(NAV_SESSIONS);
        d.observe(
            "T11.6",
            d.has_containing("Processed, with a warning"),
            "The recording's own row keeps the warning",
        );
        d.state("recording-warned", Sizes::Default);

        d.click_containing("Process again:");
        d.expect("Replace the results");
        d.state("process-again-confirm", Sizes::Default);
        d.click("Not now");

        d.click_containing("Delete recording:");
        d.expect("Keep the recording");
        d.state("recording-delete", Sizes::Default);
        d.click("Delete recording");
        d.expect("Recording deleted.");

        // A recorder that ends by itself says so, wherever the DM is.
        d.click("Record this session");
        d.click("Start recording");
        go_library(d);
        d.app_mut()
            .sessions
            .simulate_recorder_health(Health::Stopped("no such device".to_owned()));
        d.settle();
        d.observe(
            "T11.5",
            d.has_containing("Recording stopped"),
            "A recording that stops by itself stays in the rail on other pages",
        );
        d.state("recording-stopped-elsewhere", Sizes::Default);
        d.click(NAV_SESSIONS);
        d.expect("no such device");
        d.state("recording-stopped", Sizes::Default);
        d.observe(
            "T11.5",
            d.has("Record again"),
            "A recording that stopped by itself also offers one press to record again",
        );
        d.click("Record again");
        d.expect("Stop recording");
        d.observe(
            "T11.5",
            !d.has_containing("stopped by itself"),
            "Recording again clears the error",
        );
        // An input chosen now cannot reach the recording that is running.
        go_settings(d);
        d.click("Recording and processing");
        d.expect("A new input applies to the next recording");
        d.state("input-during-recording", Sizes::Default);
        d.click_containing("Sessions · ");
        d.click("Stop recording");

        // A note renamed in Obsidian keeps the DM's words, and no two rows share one session.
        d.click("Write");
        d.type_into_editor("Words typed after the rename.\n");
        d.app_mut().sessions.simulate_file_renamed();
        d.press_ctrl(Key::S);
        d.expect("File gone");
        d.state("note-file-gone", Sizes::Default);
        d.click("Save my words as a new file");
        d.expect("Notes saved");
        d.observe(
            "T11.4",
            d.editor_text()
                .is_some_and(|text| text.contains("Words typed after the rename.")),
            "A note whose file went keeps its words, saved under a new identity",
        );
        d.goal("record a session and process it");
    }
}
