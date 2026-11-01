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
use eframe::egui::{Key, Pos2, pos2};

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
    const SPELL: &str = "Fireball";
    const CLASS_ROW: &str = "Wizard · ";
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
        d.click_containing(SPELL);
        d.state("spell", Sizes::All);
        d.goal("open a spell");

        search_library(d, "wizard");
        d.scroll_until(ENTRY_LIST, CLASS_ROW, 80);
        d.click_containing(CLASS_ROW);
        d.state("class-traits-and-features", Sizes::All);
        d.goal("open a class");

        let default_width = d.at_size(DEFAULT, Driver::longest_visible_line);
        let large_width = d.at_size(LARGE, Driver::longest_visible_line);
        let long_entry = default_width <= 75 && large_width <= 75;
        // The same measure beside a page, in the panel's smaller prose.
        d.begin_task();
        look_up(d, "fireball");
        let panel_default = d.at_size(DEFAULT, Driver::longest_visible_line);
        let panel_large = d.at_size(LARGE, Driver::longest_visible_line);
        d.press(Key::Escape);
        d.begin_task();
        d.observe(
            "T6.1",
            long_entry && panel_default <= 75 && panel_large <= 75,
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
    const OTHER_NOTE: &str = "Ashfall";
    const NOTE_FILE: &str = "modules/ashen-marches/Rules/Ash Exposure.md";
    const SAVE: &str = "Save";
    const DISCARD: &str = "Discard changes";
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
        d.expect(LOCAL_PACKAGE);
        d.state("local-package", Sizes::All);
        d.check_tab_order("library, local package");
        d.goal("open a local package");

        d.click_containing(NOTE);
        d.expect(NOTE);
        d.observe(
            "Package scope",
            d.has_containing(NOTE) && !d.has_containing("Adult Red Dragon"),
            "Choosing a package limits the catalogue to it. The active campaign stays",
        );
        d.click(EDIT_MARKDOWN);
        d.state("edit-markdown", Sizes::All);
        d.goal("open a note for editing");

        d.type_into_editor("Rangers of the ash line are exempt. ");
        d.expect(UNSAVED_NOTE);
        d.state("dirty", Sizes::All);
        d.goal("change the note");

        let other_row_locked = d.is_disabled(OTHER_NOTE);
        d.click_containing(OTHER_NOTE);
        d.observe(
            "T8.2",
            !other_row_locked && d.in_view(OTHER_NOTE) && d.has_containing(UNSAVED_NOTE),
            "Read another entry while a draft is open. It opens in Lookup, and the draft stays",
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

        d.type_into_editor("Rangers carry spare cloth. ");
        let on_disk = d.root.join(NOTE_FILE);
        let edited =
            std::fs::read_to_string(&on_disk).expect("read note") + "\nEdited elsewhere.\n";
        std::fs::write(&on_disk, edited).expect("edit note on disk");
        d.click(SAVE);
        let kept = d.has_containing(CHANGED_ON_DISK) && d.has_containing(UNSAVED_NOTE);
        d.observe(
            "T8.3",
            kept,
            "A failed save keeps the draft and states the fix",
        );
        d.state("save-conflict", Sizes::Default);
        d.click(DISCARD);
        d.goal("handle a failed save");

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
    }
}

/// W9: choose what a campaign uses, then edit a campaign-owned entry.
pub mod campaign_content {
    use super::*;
    const PAGE_TITLE: &str = "entries chosen";
    const MODULE_ROW: &str = "Ashen Marches · ";
    const PARTIAL: &str = "4 selected";
    const CHANGED: &str = "5 selected";
    const UNSELECTED_ENTRY: &str = "Ash Wraith";
    const DISCARD_SELECTION: &str = "Discard selection changes";
    const CAMPAIGN_ENTRY: &str = "Captain Varra";
    const SAVE_ENTRY: &str = "Save";
    const SAVED: &str = "Saved: entry changes.";
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
        d.click_containing(REQUIRED);
        let reasons = d.in_view_containing(REQUIRED_BY);
        d.state("required-dependencies", Sizes::All);
        d.click(ENTRIES_LISTING);
        d.observe(
            "T9.1",
            reasons,
            "Required dependencies and their reasons are one click away, with the built-in package open",
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
        d.type_into(SEARCH_LIBRARY, OWN_ENTRY_SEARCH);
        let row = d
            .nodes()
            .into_iter()
            .find(|node| node.name.starts_with(CAMPAIGN_ENTRY) && node.in_view());
        d.observe(
            "T8.4",
            row.is_some_and(|row| !row.disabled),
            "Typing finds a campaign-owned entry, as a row in the results",
        );
        d.click(CLEAR_SEARCH);
        d.begin_task();
        d.click_containing(CAMPAIGN_ENTRY);
        d.state("entry-read", Sizes::Default);
        d.review("T6.4", "the campaign entry reader");
        d.click(EDIT_MARKDOWN);
        d.state("entry-edit", Sizes::Default);
        d.goal("open a campaign entry for editing");

        d.type_into_editor("Scarred by the fire at the bridge. ");
        d.state("entry-dirty", Sizes::All);
        d.click(SAVE_ENTRY);
        d.expect(SAVED);
        d.state("entry-saved", Sizes::Default);
        d.goal("save the campaign entry");

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
            d.is_disabled(SAVE_SELECTIONS) && d.has_containing("before saving"),
            "Save disabled by a repair states its reason",
        );
    }
}

/// W7: read the party's cached sheets, and the states with nothing to show.
pub mod party {
    use super::*;
    const FIRST_MEMBER: &str = "Brannoch Vale";
    const SECOND_MEMBER: &str = "Iselle Marrowind";
    const OFFLINE: &str = "Offline";
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

        d.unreached(
            "signed-in party with live updates and combat sync (T7.3)",
            "the offline sheet, because test builds cannot create a session",
        );
    }
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
    const EDITOR_NAME: &str = "Encounter name";
    const BACK_TO_LIST: &str = "Back to encounters";

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
        let detail = nodes
            .iter()
            .find(|node| node.name == RESULT_NAME && node.in_view())
            .and_then(|name| {
                nodes.iter().find(|node| {
                    node.name.contains("AC ")
                        && node.rect.min.y >= name.rect.min.y
                        && node.rect.min.y <= name.rect.max.y + 4.0
                        && (node.rect.min.x - name.rect.min.x).abs() < 4.0
                })
            })
            .map_or_else(String::new, |node| node.name.clone());
        d.observe(
            "T5.1",
            ["CR ", "HP ", "AC "]
                .iter()
                .all(|word| detail.contains(word)),
            format!("The Wolf result shows CR, HP and AC: {detail}"),
        );
        d.click_nth(ADD, 0);
        d.expect("6. ");
        d.click_below("6. ", DUPLICATE);
        d.expect("7. ");
        d.type_text(BOSS);
        d.click_nth(ADD, 0);
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

        d.unreached(
            "encounter difficulty and summary",
            "the saved roster, because the app has no difficulty or summary view",
        );
        // Read a full stat block in Lookup, with the editor still open.
        let before = d.interaction_count();
        d.click_nth(READ, 0);
        let cost = d.interaction_count() - before;
        let read = d.has_containing("ARMOR CLASS");
        d.state("stat-block-beside-editor", Sizes::All);
        let still_editing = d.has_containing(EDITOR_NAME);
        d.press(Key::Escape);
        d.observe(
            "T5.2",
            cost <= 2 && read && still_editing,
            format!(
                "A full stat block opens in {cost} beside the editor, and the draft stays open"
            ),
        );
        d.begin_task();

        d.clear_text_in(FIND_CREATURE);
        let rows = d.nodes().iter().filter(|node| node.name == ADD).count();
        d.observe(
            "T5.4",
            rows <= VIRTUALISED_LIMIT,
            format!("{rows} result rows are built each frame with no search text"),
        );

        d.click(BACK_TO_LIST);
        d.expect(SAVED_RUNS);
        let nodes = d.nodes();
        let dated = nodes.iter().any(|node| {
            node.name.contains(RUN)
                && node.name.contains("In progress")
                && node.name.chars().filter(char::is_ascii_digit).count() >= 8
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
            format!("Saved runs show a date and status: {dated}. Grouped under their encounter: {grouped}"),
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
        d.click(NEXT_TURN);
        d.expect(CURRENT_TURN);
        d.state("next-turn", Sizes::Default);
        d.goal_within("T1.1", "advance the turn", 1);
        let after = d.row_node(ANCHOR_ROW).map(|row| row.rect.min.y);
        d.observe(
            "Stable rows",
            anchor.is_some() && anchor == after,
            format!("Rows hold their places on Next turn: {anchor:?} then {after:?}"),
        );

        d.click_containing(TARGET);
        d.state("participant-selected", Sizes::Default);
        d.type_text(DAMAGE);
        d.press(Key::Enter);
        d.expect(AFTER_DAMAGE);
        d.observe_both("T1.2", "The damaged row stays in view", |d| {
            d.row_node(TARGET).is_some_and(|row| row.fully_in_view())
        });
        d.state("hp-changed", Sizes::All);
        d.goal_within("T1.2", "damage a named participant who is not current", 3);

        d.click_containing(TARGET);
        d.click(CONDITIONS);
        d.state("condition-tab", Sizes::All);
        d.click(CONDITION);
        d.expect(CONDITION);
        d.state("condition-added", Sizes::All);
        d.goal_within("T1.5", "add an official condition", 3);
        d.observe(
            "T1.5",
            d.has_containing(CONDITION),
            "The official condition came from the picker. Nothing was typed",
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
        d.goal("undo the last action");

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
            next && damage && undo && search && back,
            format!(
                "Direct key commands, {keys} key presses in all: next turn {next}, damage {damage}, undo {undo}, lookup {search}, close {back}"
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
        // A real session cannot exist in a test build, so the words are shown directly.
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
                Account::NotConnected | Account::Failed => {}
            }
        }
        d.observe(
            "T10.3",
            words,
            "Busy, connected, not connected, failed and keyring warning each have words. Displayed by a simulated state",
        );
        d.app_mut().settings.simulate(Account::NotConnected);
        d.settle();
        d.expect(NOT_CONNECTED);
        d.unreached(
            "connected account and remote campaign list",
            "account words with a simulated state, because test builds cannot create a session",
        );

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
    const CLOSE_PROMPT: &str = "A Library entry has unsaved changes.";
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
        d.expect(CLOSE_PROMPT);
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
        d.goal("see the other empty tabs");
    }
}

/// W2: look something up during a run, then return. Lookup opens beside the roster,
/// so the run stays on screen and Next turn stays clickable.
pub mod run_lookup {
    use super::*;
    const PARTICIPANT: &str = "Ember Cultist";
    const SPELL_QUERY: &str = "fireball";
    const SPELL: &str = "Fireball";
    const PACKAGE_ENTRY: &str = "Ash Exposure";
    const CAMPAIGN_ENTRY: &str = "Captain Varra";
    const SPELL_IN_STAT_BLOCK: &str = "Ashfall";
    const OTHER_NOTE: &str = "Ashfall";
    const NOTE: &str = "Ash Exposure";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_containing(PARTICIPANT);
        d.state("in-the-run", Sizes::All);
        d.goal("be in a run, with a participant selected");

        look_up(d, SPELL_QUERY);
        d.expect(SPELL);
        d.state("reading-spell-beside-run", Sizes::All);
        d.goal_within("T2.1", "read a spell from a run", 3);
        let builtin_found = d.in_view(SPELL);
        d.observe_both(
            "T2.1",
            "Roster and Next turn stay visible and clickable",
            |d| {
                d.in_view(NEXT_TURN)
                    && !d.is_disabled(NEXT_TURN)
                    && d.row_node(PARTICIPANT)
                        .is_some_and(|row| row.fully_in_view())
            },
        );

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
        d.click(SPELL_IN_STAT_BLOCK);
        let opened = d.in_view(SPELL_IN_STAT_BLOCK) && d.in_view(NEXT_TURN);
        d.state("spell-from-stat-block", Sizes::Default);
        d.observe(
            "T2.4",
            linked && opened,
            "A spell named in a run stat block opens in 1, beside the roster",
        );
        d.goal("read the creature's details in the run");
        d.press(Key::Escape);

        // A draft elsewhere does not block Lookup, and Lookup does not touch the draft.
        go_library(d);
        open_local_package(d, MODULE);
        d.click_containing(NOTE);
        d.click(EDIT_MARKDOWN);
        d.type_into_editor("Unfinished ruling. ");
        d.expect(UNSAVED_NOTE);
        d.begin_task();
        look_up(d, &OTHER_NOTE.to_lowercase());
        d.state("lookup-with-draft-open", Sizes::All);
        d.observe(
            "T2.5",
            d.in_view(OTHER_NOTE) && d.has_containing(UNSAVED_NOTE) && d.in_view(NEXT_TURN),
            "Lookup works while a Library draft is unsaved. The draft is kept, and the run strip stays",
        );
        d.goal("try a lookup with a draft open");
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

        d.click(ROLL_NPCS);
        d.state("npcs-rolled", Sizes::Default);
        d.goal("roll NPC initiative");

        let mut row_moved = false;
        let mut focus_is_here = false;
        for (index, (player, total)) in PLAYERS.iter().enumerate() {
            let field = format!("Initiative for {player}");
            let before = d.row_of(&field).min.y;
            if std::mem::take(&mut focus_is_here) {
                d.type_text(total);
            } else {
                d.type_into(&field, total);
            }
            let after = d.row_of(&field).min.y;
            row_moved |= (before - after).abs() > 1.0;
            if index == 0 {
                d.state("first-player-initiative", Sizes::Default);
                d.goal_within("T4.3", "enter one player's initiative", 2);
            }
            // Enter goes to the next player who has no initiative (review M-7).
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

    pub fn run(d: &mut Driver<'_>) {
        d.state("cold-start", Sizes::Default);
        resume_run_from_cold_start(d);
        d.expect(CURRENT_TURN);
        d.state("resumed", Sizes::All);
        d.goal_within("T4.2", "cold start to the interrupted turn", 2);
        let current = current_participant(d).unwrap_or_default();
        d.observe(
            "T4.2",
            participant_is_selected(d, &current),
            format!("The current participant, {current}, is selected on resume"),
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
        d.click(NEXT_TURN);
        d.goal_within("T1.1", "advance the turn with 12 participants", 1);

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
        for turn in 0..ROUND_LENGTH {
            d.click(NEXT_TURN);
            if !current_row_in_view(d) {
                missed_small += 1;
            }
            if turn == ROUND_LENGTH / 2 {
                d.state("mid-round", Sizes::Default);
            }
        }
        d.goal("advance through a full round of 20");
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
            missed_small == 0 && missed_default == 0,
            format!("Over 20 turns the current row was not fully in view {missed_small} times at 960x640, {missed_default} at 1200x800"),
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
    const ROW_SYNC_WORDS: [&str; 4] = ["Synced", "First read", "Sync ", "Conflict"];
    const SYNC_PARTY: &str = "Sync party";
    const STOP_SYNC: &str = "Stop party sync";
    const UNDO_CLEARED: &str = "Undo history was cleared";
    const CLOSE_PROMPT: &str = "has not confirmed changes";
    const DISABLED_REASON: &str = "Wait for pending sync";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.state("run-before-conflict", Sizes::Default);
        d.goal("resume the run");

        d.app_mut().encounters.simulate_sync_conflict(SYNCED);
        d.settle();
        d.state("conflict-with-nothing-expanded", Sizes::All);
        let on_row = d
            .row_node(SYNCED)
            .is_some_and(|row| row.name.contains("Conflict") && row.fully_in_view());
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
            d.has_containing(CLOSE_PROMPT) && d.has_containing(SYNCED),
            "Closing the app with pending sync warns, and names the character",
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
            d.row_node(name)
                .is_some_and(|row| ROW_SYNC_WORDS.iter().any(|word| row.name.contains(word)))
        });
        d.observe(
            "T3.3",
            cost <= 2 && all_rows && d.has(STOP_SYNC),
            format!("Sync turned on for the whole party in {cost}. Every party row shows a state: {all_rows}"),
        );
        d.click(STOP_SYNC);
        d.begin_task();

        d.app_mut().encounters.simulate_sheet_update(SYNCED);
        d.settle();
        d.state("undo-cleared-by-sheet-update", Sizes::Default);
        d.observe(
            "T3.4",
            d.has_containing(UNDO_CLEARED),
            "A message appears when a sheet update clears undo history",
        );
        d.unreached(
            "wrong campaign, duplicate character and a disconnect during sync",
            "the error state on the row, because these need a live session to arise",
        );
        d.not_measured("T7.3", "Party sync needs a live session");
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
        d.state("library-set-up", Sizes::Default);

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
            "G1",
            "tooltips on disabled controls need hover probes of every control",
        );
        d.not_measured(
            "G5",
            "the app ships one theme, so there is nothing to compare yet",
        );
    }
}

/// W7 from a run: open a participant's sheet during combat, then return.
pub mod party_from_run {
    use super::*;
    const MEMBER: &str = "Kestrel Dunmoor";
    const SHEET_TITLE: &str = "Kestrel Dunmoor";
    const SHEET_PAGE: &str = "Record";
    const SHEET_PAGE_TEXT: &str = "ARMOR CLASS";
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
                    && d.in_view_containing(SHEET_PAGE_TEXT)
            },
        );
        d.observe_both("T7.2", "The sheet in the run says how old it is", |d| {
            d.in_view_containing(FRESHNESS)
        });

        d.click(SHEET_PAGE);
        d.state("sheet-page", Sizes::Default);
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
        // Each condition's name is on screen in full, on its row, at the minimum window.
        let unread: Vec<&str> = MANY_CONDITIONS
            .into_iter()
            .filter(|condition| !d.visible_text(condition))
            .collect();
        d.observe(
            "T1.8",
            unread.is_empty() && d.visible_text("0 HP") && d.visible_text("Absent"),
            format!("At 960x640, words that are not in full view: {unread:?}"),
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

    pub fn run(d: &mut Driver<'_>) {
        d.click(NEW_ENCOUNTER);
        d.type_text(DRAFT_NAME);
        d.state("draft-open", Sizes::Default);

        go_library(d);
        search_library(d, QUERY);
        d.click_containing(ENTRY);
        d.state("library-entry", Sizes::Default);
        copy_open_entry_to_campaign(d);
        d.click(CLOSE_COPY);
        d.state("copied", Sizes::Default);

        go_encounters(d);
        d.expect(DRAFT_NAME);
        d.replace_text_in(FIND_CREATURE, COPY_NAME);
        let found = d.has_label(COPY_NAME);
        d.state("copy-in-open-draft-results", Sizes::All);
        d.observe(
            "T8.1",
            found,
            "The copy can join the unsaved draft with no manual reload, and the draft is kept",
        );

        d.click_nth(ADD, 0);
        d.expect(&format!("1. {COPY_NAME}"));
        d.click(SAVE_ENCOUNTER);
        d.state("copy-in-encounter", Sizes::All);
        d.goal("copy an entry and use it in an encounter");
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
                    d.click_nth(name, 0);
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
    }
}

/// Section 3's table: an open run survives a restart, on the current turn, with the
/// last page and party sync kept. The scenario really starts the app a second time.
pub mod run_restart {
    use super::*;
    const OTHER: &str = "Ember Cultist";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
        d.click_containing(OTHER);
        d.click(NEXT_TURN);
        let current = current_participant(d).unwrap_or_default();
        d.state("before-restart", Sizes::Default);
        d.goal("be in a run");
        d.restart();
        d.state("after-restart", Sizes::All);
        d.goal("restart");
        d.observe(
            "Restart",
            d.has_containing(RUN_HEADING)
                && current_participant(d).as_deref() == Some(current.as_str())
                && participant_is_selected(d, &current),
            format!("The run reopens by itself on the same turn, {current} selected"),
        );
        // The prompt shares the footer's narrow row, and must keep every word.
        d.click("Close run");
        d.state("close-run-prompt", Sizes::Default);
        d.click("Keep running");
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
        d.click_containing(TARGET);
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
            d.click_containing(name);
            d.click(ABSENT);
        }
        d.click(NEXT_TURN);
        d.expect("No participants are present");
        d.state("all-absent", Sizes::All);
        d.goal("see everyone absent");
        d.click(CLOSE_ERROR);
        for name in ALL {
            d.click_containing(name);
            d.click(ABSENT);
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
        d.click_containing(TARGET);
        d.type_text("3");
        d.press(Key::Enter);
        d.expect("changed on disk");
        d.expect("Not saved");
        d.state("changed-on-disk", Sizes::All);
        d.goal("see the run changed on disk");
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
    use crate::combat_sync::SyncState;
    const STATES: [(&str, SyncState, &str); 5] = [
        ("Brannoch Vale", SyncState::Conflict, "Conflict"),
        ("Iselle Marrowind", SyncState::Matching, "Synced"),
        ("Kestrel Dunmoor", SyncState::Pending, "Sync pending"),
        ("Oren Thistlewick", SyncState::ReadOnly, "Sync read-only"),
        ("Sable Harrowgate", SyncState::Error, "Sync error"),
    ];
    const UNSYNCED_ROW: &str = "Cinder Wolf 1";

    pub fn run(d: &mut Driver<'_>) {
        resume_run_from_cold_start(d);
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
            d.observe(
                "T3.1",
                d.visible_text(word),
                format!("At 960x640, the word {word} is on screen in full"),
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
        d.state("every-sync-state", Sizes::All);
        d.goal("show every sync state");
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
    use crate::combat_sync::SyncState;
    const SYNCED: &str = "Brannoch Vale";
    const USE_REMOTE: &str = "Use D&D Beyond values";

    pub fn run(d: &mut Driver<'_>) {
        resume_run(d, RUN_12);
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
        d.click_containing(SYNCED);
        d.click(USE_REMOTE);
        close_run(d);
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
