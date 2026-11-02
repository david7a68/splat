//! The UI vocabulary every scenario shares. When a redesign moves or renames a
//! page, fix it here. Labels used by one scenario live at the top of that scenario.
use super::{driver::Driver, fixture::RUN};

pub const NAV_LIBRARY: &str = "Library";
pub const NAV_CAMPAIGN: &str = "Campaign";
pub const NAV_SETTINGS: &str = "Settings";
pub const NAV_PARTY: &str = "Party";
pub const NAV_ENCOUNTERS: &str = "Encounters";
pub const NAV_LOOKUP: &str = "Lookup";
pub const NAV_SESSIONS: &str = "Sessions";
pub const NAV_TIMELINE: &str = "Timeline";
pub const TAB_ENTRIES: &str = "Campaign entries";
pub const TAB_PACKAGES: &str = "Packages";
pub const TAB_RUN_SETTINGS: &str = "Rules and runs";

pub const SEARCH_LIBRARY: &str = "Search the Library";
pub const SEARCH_LOOKUP: &str = "Search everything";
pub const CLEAR_SEARCH: &str = "Clear";
pub const SIDEBAR_CAMPAIGN_DROPDOWN: usize = 0;
pub const LIBRARY_PACKAGE_DROPDOWN: usize = 1;
pub const COPY_DESTINATION_DROPDOWN: usize = 2;
pub const BACK_TO_RUN: &str = "Back to run";
pub const COPY_TO: &str = "Copy to…";
pub const REVIEW_COPY: &str = "Review copy";
pub const CREATE_COPY: &str = "Create copy";
pub const CLOSE_COPY: &str = "Close";
pub const EDIT_MARKDOWN: &str = "Edit Markdown";
pub const UNSAVED_NOTE: &str = "Unsaved";

pub const SAVED_RUNS: &str = "Saved runs (";
pub const RESUME: &str = "Resume";
pub const RUN_HEADING: &str = "Run · ";
pub const CURRENT_TURN: &str = "Current turn";
pub const NEXT_TURN: &str = "Next turn";
pub const APPLY_DAMAGE: &str = "Damage";
pub const CONDITIONS: &str = "Conditions for this participant";
pub const CLOSE_RUN: &str = "Close run";
pub const CLOSE_NOW: &str = "Close now";
pub const NEW_ENCOUNTER: &str = "New encounter";
pub const SAVE_ENCOUNTER: &str = "Save encounter";
pub const RUN_ENCOUNTER: &str = "Run encounter";
pub const EDIT_ENCOUNTER: &str = "Edit";
pub const FIND_CREATURE: &str = "Find a creature or party member";
pub const ADD_PARTY: &str = "Add party";
pub const ADD: &str = "Add";

pub fn go_library(d: &mut Driver<'_>) {
    d.click(NAV_LIBRARY);
}

pub fn go_settings(d: &mut Driver<'_>) {
    d.click(NAV_SETTINGS);
}

pub fn go_campaign(d: &mut Driver<'_>) {
    d.click(NAV_CAMPAIGN);
}

pub fn go_encounters(d: &mut Driver<'_>) {
    d.click(NAV_ENCOUNTERS);
}

/// A new session from the first template, with a title.
pub fn new_session(d: &mut Driver<'_>, title: &str) {
    d.click("New session");
    d.expect("Create session");
    d.type_into("Session title", title);
    d.click("Create session");
    d.expect(&format!("Session 1: {title}"));
}

pub fn go_party(d: &mut Driver<'_>) {
    d.click(NAV_PARTY);
}

/// Open Lookup and type, with the search field focused for you.
pub fn look_up(d: &mut Driver<'_>, text: &str) {
    d.click(NAV_LOOKUP);
    d.type_text(text);
}

/// With Lookup open, search for something else.
pub fn look_up_again(d: &mut Driver<'_>, text: &str) {
    d.replace_text_in(SEARCH_LOOKUP, text);
}

/// From a cold start, open the saved run in progress. The app opens on Encounters.
pub fn resume_run_from_cold_start(d: &mut Driver<'_>) {
    resume_run(d, RUN);
}

pub fn resume_run(d: &mut Driver<'_>, name: &str) {
    d.click_beside(name, RESUME);
    d.expect(&format!("{RUN_HEADING}{name}"));
}

/// Close the run. It takes a second, inline step, far from Next turn.
pub fn close_run(d: &mut Driver<'_>) {
    d.click(CLOSE_RUN);
    d.click(CLOSE_NOW);
}

pub fn open_saved_encounter(d: &mut Driver<'_>, name: &str) {
    d.click_beside_exact(name, EDIT_ENCOUNTER);
}

pub fn run_saved_encounter(d: &mut Driver<'_>, name: &str) {
    d.click_beside_exact(name, RUN_ENCOUNTER);
}

pub fn open_local_package(d: &mut Driver<'_>, name: &str) {
    d.open_combo_at(LIBRARY_PACKAGE_DROPDOWN);
    d.click(name);
}

pub fn search_library(d: &mut Driver<'_>, text: &str) {
    if d.has(CLEAR_SEARCH) {
        d.click(CLEAR_SEARCH);
    }
    d.type_into(SEARCH_LIBRARY, text);
}

pub fn switch_campaign(d: &mut Driver<'_>, name: &str) {
    d.open_combo_at(SIDEBAR_CAMPAIGN_DROPDOWN);
    d.click(name);
}

/// Every run row's participant and place, in list order. A row names its HP and AC.
pub fn roster_places(d: &Driver<'_>) -> Vec<(String, eframe::egui::Rect)> {
    d.nodes()
        .into_iter()
        .filter(|node| node.name.contains(", HP ") && node.name.contains(", AC "))
        .map(|node| {
            let name = node.name.split(", ").next().unwrap_or_default().to_owned();
            (name, node.rect)
        })
        .collect()
}

/// The participant on the row marked as the current turn.
pub fn current_participant(d: &Driver<'_>) -> Option<String> {
    d.nodes()
        .iter()
        .find(|node| node.name.contains(CURRENT_TURN))
        .and_then(|node| node.name.split(", ").next().map(str::to_owned))
}

/// The run shows the selected participant's name as a heading in its side panel.
pub fn participant_is_selected(d: &Driver<'_>, name: &str) -> bool {
    d.has_label(&format!("Selected: {name}"))
}

/// Copy one Library entry from the open reader into the active campaign, which is
/// the destination the dialog starts with.
pub fn copy_open_entry_to_campaign(d: &mut Driver<'_>) {
    d.click(COPY_TO);
    d.click(REVIEW_COPY);
    d.expect("Ready to copy");
    d.click(CREATE_COPY);
    d.expect("Saved: ");
}
