//! Desktop application and shared content APIs.
/// Typed messages (`design::outcome`). Public so that `sentence!` works and its compile-fail
/// proofs can name the types; screens use them through `design`.
#[doc(hidden)]
pub use design::outcome;
pub use splat_character::{character, memo};
pub use splat_execution::execution;

pub mod content {
    pub use splat_content::content::*;

    pub mod definitions {
        pub use splat_content::content::definitions::*;
        pub use splat_execution::execution::catalogue::{CatalogueReport, verify};
    }
}

extern crate self as splat;

// The GUI tests run many app instances at once. With glibc's malloc they used more memory and
// CPU.
#[cfg(all(test, not(target_env = "msvc")))]
#[global_allocator]
static TEST_ALLOCATOR: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use splat_content_ui::art;
pub mod auth {
    #[cfg(test)]
    pub use splat_content_ui::auth::connect_live;
    pub use splat_content_ui::auth::{Session, connect};
}
use splat_campaign_packages::campaign_content;
use splat_content_ui::campaigns;
#[cfg(test)]
use splat_party::combat_sync;
mod commands;
#[cfg(test)]
mod content_integration_tests;
use splat_content_ui::content_workspace;
use splat_content_ui::copy_dialog;
use splat_sessions_ui::credentials;
#[path = "design/app.rs"]
mod design;
#[cfg(test)]
mod fixtures;
mod gui;
// The layout parts are public only so that their `compile_fail` doc tests can name them.
#[doc(hidden)]
pub use design::extras;
pub use design::layout;
// The pane is public only so that its `compile_fail` doc tests can name it.
/// The shared fact part (ruling M5), public only so that its doc tests can name it.
#[doc(hidden)]
pub use design::fact;
#[doc(hidden)]
pub use design::pane;
pub use design::value_block;
/// The ownership and cost parts of `design` (kept results, the save row, per-change caches),
/// public only so that their compile-fail doc tests can name them. The app uses them through
/// `design`.
#[doc(hidden)]
pub mod ownership {
    pub use crate::design::components::{Footer, ResultOrder};
    pub use crate::design::memo::{Changed, Key, Memo, Revised, Revision};
    pub use crate::design::results::{
        AtLimit, Cause, Change, DiskChanged, Ends, FailedAct, Fallback, File, Folder, Gone, Kept,
        Kind, Limit, Mismatch, Name, Owner, OwnerChanged, Part, Press, Projection, Recovery,
        Refusal, Results, Retained, Rows, RunRule, Shown, Step, Thing, Way,
    };
    pub use crate::design::save_row::{
        Diagnosis, DraftOrigin, DraftState, Primary, SaveWeight, SaveWord,
    };
}
use splat_content_ui::dndbeyond;
use splat_content_ui::hermetic;
use splat_content_ui::history;
use splat_design::frame_clock;
use splat_encounters_ui::encounter_run;
#[cfg(test)]
use splat_encounters_ui::encounter_settings;
use splat_encounters_ui::encounters;
use splat_library_page::library;
#[cfg(test)]
mod load_bench;
use splat_content_ui::lookup;
use splat_content_ui::markdown_view;
use splat_content_ui::persistence;
use splat_content_ui::roll_table;
use splat_content_ui::rolls;
use splat_party::party;
use splat_sessions_ui::processing;
#[cfg(test)]
use splat_sessions_ui::recording;
use splat_sessions_ui::sessions;
mod settings;
#[cfg(test)]
use splat_content_ui::sheet_writer;
#[cfg(test)]
use splat_content_ui::test_support;
use splat_sessions_ui::timeline;
mod touchpad;
#[path = "trackers/app.rs"]
mod trackers;
mod ui_state;
mod world {
    pub use splat_content_ui::world::*;
    pub use splat_world_page::page;
}

use design::{Status, colors, icons};
use eframe::egui;
pub use splat_design::{choice, create_reasons, field_id, key, lead, sentence, word};

/// Start Splat: a command when there are arguments, else the app's window.
pub fn run() {
    let Some(_instance) = start_process() else {
        return;
    };
    gui::run();
}

/// Run a command and exit when there are arguments; otherwise take this computer's instance lock.
/// `None` when another Splat holds it (the DM was told).
fn start_process() -> Option<Option<ui_state::InstanceLock>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        std::process::exit(commands::run(&args, &mut std::io::stdout().lock()));
    }
    let Some(dirs) = directories::ProjectDirs::from("", "", "splat") else {
        return Some(None);
    };
    match ui_state::InstanceLock::acquire(&dirs.config_dir().join("instance.lock")) {
        Ok(lock) => Some(Some(lock)),
        Err(error) => {
            rfd::MessageDialog::new()
                .set_title("Splat is already open")
                .set_description(error)
                .show();
            None
        }
    }
}

struct Splat {
    page: Page,
    campaign_tab: CampaignTab,
    settings_tab: SettingsTab,
    /// How far a touchpad scroll moves content. It belongs to this computer, not a campaign.
    touchpad_speed: touchpad::TouchpadSpeed,
    shown_page: Page,
    /// Notes closed for Lookup, and they come back with it closed.
    notes_gave_way: bool,
    started: bool,
    confirm_close: bool,
    discard_on_close: bool,
    library: library::Library,
    campaign_content: campaign_content::CampaignContent,
    copy_dialog: Option<copy_dialog::CopyDialog>,
    settings: settings::Settings,
    campaigns: campaigns::Campaigns,
    dndbeyond: dndbeyond::DndBeyond,
    party: party::Party,
    party_page: splat_party_page::PartyPage,
    encounters: encounters::Encounters,
    lookup: lookup::Lookup,
    /// The house rules' trackers and the ledger; Lookup shows them, and the rail counts them.
    trackers: trackers::Trackers,
    sessions: sessions::page::Sessions,
    timeline: timeline::Book,
    timeline_page: timeline::page::TimelinePage,
    world_page: world::page::WorldPage,
    processing: processing::settings::ProcessingSettings,
    /// What the open run last told the timeline.
    run_fact: Option<timeline::RunFact>,
    /// Where Close run goes: the page the run was opened from.
    run_origin: Page,
    /// What the party's carried state was last read for.
    carry_key: Option<(Option<uuid::Uuid>, timeline::PartyState, bool, Page, bool)>,
    /// New events and runs go to a session dated before today.
    old_session: bool,
    art: art::Loader,
    ui_store: ui_state::Store,
}

impl Default for Splat {
    fn default() -> Self {
        Self {
            page: Page::default(),
            campaign_tab: CampaignTab::default(),
            settings_tab: SettingsTab::default(),
            touchpad_speed: touchpad::TouchpadSpeed::default(),
            shown_page: Page::default(),
            notes_gave_way: false,
            started: false,
            confirm_close: false,
            discard_on_close: false,
            library: Default::default(),
            campaign_content: Default::default(),
            copy_dialog: None,
            settings: Default::default(),
            campaigns: Default::default(),
            dndbeyond: Default::default(),
            party: Default::default(),
            party_page: Default::default(),
            encounters: Default::default(),
            lookup: Default::default(),
            trackers: trackers::Trackers::with_ids(hermetic::new_uuid),
            sessions: Default::default(),
            timeline: Default::default(),
            timeline_page: Default::default(),
            world_page: Default::default(),
            processing: Default::default(),
            run_fact: None,
            run_origin: Page::Encounters,
            carry_key: None,
            old_session: false,
            art: Default::default(),
            ui_store: ui_state::Store::user(),
        }
    }
}

/// What the rail says about the D&D Beyond account.
#[derive(Clone, Copy)]
enum Connection {
    Connecting,
    Connected,
    /// The session still works, but renewing it fails. It ends soon.
    Failing,
    Offline,
}

/// The rail's Run slot with no fight open. It keeps the slot, so no rail target moves.
pub(crate) const NO_FIGHT_OPEN: &str = "No fight open";

/// The empty Run slot's accessible name. It does not start like an open run's `Run · …`.
pub(crate) fn no_run_name() -> String {
    format!("Run: {NO_FIGHT_OPEN}")
}

/// One flat level. A run is a place, not a mode of a tab.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
enum Page {
    Run,
    #[default]
    Encounters,
    Sessions,
    Timeline,
    World,
    Party,
    Library,
    Campaign,
    Settings,
}

/// FF2: the page (and Campaign tab) that holds the draft a rail status names. The rail's draft
/// list and this table share one constant for each draft, so a new draft names its page here.
fn draft_page(word: &str) -> Option<(Page, Option<CampaignTab>)> {
    Some(match word {
        LIBRARY_DRAFT => (Page::Library, None),
        WORLD_DRAFT | WORLD_REPAIR_DRAFT => (Page::World, None),
        encounters::ENCOUNTER_DRAFT | encounters::GROUP_DRAFT | encounters::CREATURE_DRAFT => {
            (Page::Encounters, None)
        }
        CAMPAIGN_DRAFT => (Page::Campaign, Some(CampaignTab::Packages)),
        encounters::RUN_SETTINGS_DRAFT => (Page::Campaign, Some(CampaignTab::RunSettings)),
        _ => return None,
    })
}
const LIBRARY_DRAFT: &str = "Library draft";
const WORLD_DRAFT: &str = "World draft";
const WORLD_REPAIR_DRAFT: &str = "World repair draft";
const CAMPAIGN_DRAFT: &str = "Campaign draft";

impl Page {
    fn name(self) -> &'static str {
        match self {
            Self::Run => "Run",
            Self::Encounters => "Encounters",
            Self::Sessions => "Session",
            Self::Timeline => "Timeline",
            Self::World => "World",
            Self::Party => "Party",
            Self::Library => "Library",
            Self::Campaign => "Campaign",
            Self::Settings => "Settings",
        }
    }
    fn from_name(name: &str) -> Option<Self> {
        // The page's name until it became the place for tonight.
        if name == "Sessions" {
            return Some(Self::Sessions);
        }
        [
            Self::Encounters,
            Self::Sessions,
            Self::Timeline,
            Self::World,
            Self::Party,
            Self::Library,
            Self::Campaign,
            Self::Settings,
        ]
        .into_iter()
        .find(|page| page.name() == name)
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
enum CampaignTab {
    #[default]
    Packages,
    RunSettings,
    Link,
}

#[derive(Clone, Copy, Default, PartialEq)]
enum SettingsTab {
    #[default]
    Account,
    Recording,
    Touchpad,
}

impl Splat {
    /// Restore what this computer remembers (`ui_state`): the page, the session played, the
    /// open run, the touchpad speed. The app calls it once before its first frame.
    fn restore(&mut self) -> ui_state::UiState {
        self.started = true;
        let saved = self.ui_store.load();
        self.touchpad_speed = saved.touchpad_speed;
        self.sessions.restored_session = saved.playing;
        self.sessions.party_bar = saved.party_bar;
        self.sessions.list_hidden = saved.session_list_hidden;
        let active = self.campaigns.active_campaign();
        self.page =
            saved
                .page
                .as_deref()
                .and_then(Page::from_name)
                .unwrap_or(if active.is_some() {
                    Page::Encounters
                } else {
                    Page::Library
                });
        if let (Some(open), Some(active)) = (&saved.open_run, &active)
            && open.campaign == active.id
        {
            self.encounters
                .restore_run(&open.path, active, &open.synced);
        }
        saved
    }

    /// What this computer remembers now, with the UI's column widths.
    fn ui_state_now(
        &self,
        column_widths: std::collections::BTreeMap<String, f32>,
    ) -> ui_state::UiState {
        let active = self.campaigns.active_campaign();
        ui_state::UiState {
            column_widths,
            touchpad_speed: self.touchpad_speed,
            party_bar: self.sessions.party_bar,
            session_list_hidden: self.sessions.list_hidden,
            // With a run open, the page it began on: Close run goes back there after a restart.
            page: Some(
                if self.page == Page::Run {
                    self.run_origin
                } else {
                    self.page
                }
                .name()
                .to_owned(),
            ),
            playing: active
                .as_ref()
                .zip(self.sessions.chosen_session())
                .map(|(campaign, session)| (campaign.id, session)),
            open_run: self.encounters.run().and_then(|run| {
                Some(ui_state::OpenRun {
                    campaign: active.as_ref()?.id,
                    path: run.path()?.to_path_buf(),
                    synced: run.synced_keys(),
                })
            }),
        }
    }

    /// F9, W5-D1: a fight that enters the Timeline now, from a plan line that a journey's check
    /// wrote, is played at that check (`Sessions::road_at_begin`). Else it goes after the
    /// session's last event as any fight does, and the run's notice line says why.
    fn road_of_entering_fight(
        &mut self,
        fact: &timeline::RunFact,
    ) -> Option<(timeline::checks::RoadMark, timeline::Within)> {
        let entering = fact.state == timeline::RunState::Running
            && !self.timeline.events.iter().any(|event| {
                event.run == Some(fact.id)
                    || (fact.copied_from.is_some() && event.run == fact.copied_from)
            });
        if !entering {
            return None;
        }
        let session = self.sessions.current();
        self.sessions
            .road_at_begin(&self.timeline, session, &fact.name)?
            .inspect_err(|why| self.tell_run(why.words()))
            .ok()
    }

    /// The fight just placed is saved and checked again after the merge
    /// (`Sessions::confirm_road`).
    fn confirm_road(&mut self, fact: &timeline::RunFact, mark: &timeline::checks::RoadMark) {
        let session = self.sessions.current();
        if let Some(why) =
            self.sessions
                .confirm_road(&mut self.timeline, session, fact.id, &fact.name, mark)
        {
            self.tell_run(why.words());
        }
    }

    fn tell_run(&mut self, words: &str) {
        if let Some(run) = self.encounters.run_mut() {
            run.tell(words.to_owned());
        }
    }

    /// Where the party stands in the session being played, from the timeline and the last
    /// fight's file. It is read again when any of those could have changed.
    fn update_carry(&mut self) {
        let session = self.sessions.current();
        let state = timeline::party_state(&self.timeline.events, session);
        let active = self.campaigns.active_campaign();
        let directory = active
            .as_ref()
            .and_then(|campaign| self.encounters.run_directory(campaign));
        // The folder comes with a background read, so a restart gets here before it does.
        let key = (
            session,
            state.clone(),
            self.run_fact.is_some(),
            self.page,
            directory.is_some(),
        );
        if self.carry_key.as_ref() == Some(&key) {
            return;
        }
        self.carry_key = Some(key);
        let carry = active
            .filter(|_| state.rested || !state.fights.is_empty())
            .map(|campaign| {
                let mut carry = encounter_run::Carry {
                    rest_order: state.rest_order,
                    from: "the long rest".to_owned(),
                    full: state.rested,
                    values: Default::default(),
                    missing: Vec::new(),
                };
                for (run, name) in state.fights {
                    let read = directory
                        .as_ref()
                        .ok_or_else(String::new)
                        .and_then(|directory| encounter_run::find_run(directory, campaign.id, run))
                        .and_then(|path| encounter_run::party_at_end(&path, campaign.id));
                    match read {
                        // A later fight has the newer word on anyone who was in it.
                        Ok(values) => {
                            carry.values.extend(values);
                            carry.from = name;
                        }
                        Err(_) => carry.missing.push(name),
                    }
                }
                carry
            });
        self.party.tonight.clone_from(&carry);
        if let Some(run) = self.encounters.run_mut()
            && let Some(carry) = &carry
        {
            run.remember_rest(carry.rest_order);
        }
        self.encounters.carry = carry;
    }

    /// The page the rail marks.
    fn visible_page(&self) -> Option<Page> {
        Some(self.page)
    }

    fn blocked_switch(&self) -> Option<&'static str> {
        if self.encounters.has_run() {
            Some("Close the run to change campaigns.")
        } else if self.sessions.recording() {
            Some("Stop the recording to change campaigns.")
        } else if self.sessions.processing() {
            Some("Wait for processing to finish, or cancel it, to change campaigns.")
        } else if self.campaign_content.applying() {
            Some("Wait until Apply has finished.")
        } else if self.sessions.busy_with_edit() || self.timeline.not_saved() {
            Some("Resolve the unsaved session file to change campaigns.")
        } else if self.world_page.dirty()
            || self.world_page.has_generated()
            || self.lookup.generated_name().is_some()
            || self.any_unsaved_edits()
            || self.campaign_content.dirty()
            || self.encounters.dirty()
            || self.library.campaign_draft()
        {
            Some("Save or discard your changes to change campaigns.")
        } else {
            None
        }
    }

    /// A World entry form with changes, in World or Lookup.
    fn any_unsaved_edits(&self) -> bool {
        self.world_page.reader.has_unsaved_edits() || self.lookup.reader.has_unsaved_edits()
    }

    /// Everything closing the app would lose, by name, with the page where it can be kept.
    fn unsaved_items(&self) -> Vec<(String, Page)> {
        let campaign = self
            .campaigns
            .active_campaign()
            .map_or_else(|| "this campaign".to_owned(), |c| c.name);
        let mut items = Vec::new();
        if let Some((name, owner)) = self.library.draft_place() {
            items.push((
                format!("\u{201c}{name}\u{201d} in {owner} has unsaved changes."),
                Page::Library,
            ));
        }
        if self.campaign_content.dirty() {
            items.push((
                format!("Package choices for {campaign} have unsaved changes."),
                Page::Campaign,
            ));
        }
        if self.world_page.dirty() {
            items.push((
                "A World note repair has unsaved changes.".into(),
                Page::World,
            ));
        }
        if let Some(name) = self.world_page.held_npc() {
            items.push((
                format!("The generated NPC \u{201c}{name}\u{201d} is not saved."),
                Page::World,
            ));
        }
        // An NPC that Lookup rolled and holds; Lookup opens beside any page.
        if let Some(name) = self.lookup.generated_name() {
            items.push((
                format!("The generated NPC \u{201c}{name}\u{201d} is not saved."),
                self.page,
            ));
        }
        for (reader, page) in [
            (&self.world_page.reader, Page::World),
            (&self.lookup.reader, self.page),
        ] {
            for (_, name) in reader.unsaved_edits() {
                items.push((
                    format!("Edits to \u{201c}{name}\u{201d} are not saved."),
                    page,
                ));
            }
        }
        let name = self.encounters.draft_name().unwrap_or_default();
        if self.encounters.encounter_dirty() {
            items.push((
                format!(
                    "{} \u{201c}{name}\u{201d} has unsaved changes.",
                    self.encounters.draft_kind()
                ),
                Page::Encounters,
            ));
        }
        if self.encounters.creature_dirty() {
            items.push((
                format!("A creature in \u{201c}{name}\u{201d} has unsaved changes."),
                Page::Encounters,
            ));
        }
        if self.encounters.settings_unsaved() {
            items.push((
                format!("Run settings for {campaign} have unsaved changes."),
                Page::Campaign,
            ));
        }
        let names = self.encounters.pending_sync_names();
        if !names.is_empty() {
            items.push((
                format!(
                    "D&D Beyond has not confirmed changes to {}. The saved run keeps HP, death saves and conditions, and asks you to choose when you next sync.{}",
                    names.join(", "),
                    // A close keeps Undo (it is saved with the run), so only these are lost.
                    if self.encounters.run().is_some_and(|run| !run.unsent_only_here_names().is_empty()) { " Unsent spell slots and limited uses are lost." } else { "" }
                ),
                Page::Run,
            ));
        }
        if self.party.unconfirmed() {
            items.push((
                "D&D Beyond has not confirmed a change made on the Party page.".to_owned(),
                Page::Party,
            ));
        }
        if let Some(name) = self.encounters.run_not_saved() {
            items.push((
                format!("Run \u{201c}{name}\u{201d} could not be saved."),
                Page::Run,
            ));
        }
        for item in self.sessions.not_saved() {
            items.push((item, Page::Sessions));
        }
        if self.timeline.not_saved() {
            items.push(("The timeline is not saved.".to_owned(), Page::Timeline));
        }
        if self.sessions.recording_label().is_some() {
            items.push((
                "Closing stops the recording. What is recorded so far is kept.".to_owned(),
                Page::Sessions,
            ));
        }
        if self.sessions.processing_label().is_some() {
            items.push((
                "Closing stops processing. What is done is kept. Continue processing goes on from there.".to_owned(),
                Page::Sessions,
            ));
        }
        items
    }

    /// What the World page, Lookup and the roll tables in them need from the rest of the app.
    fn follow_world(&mut self, ctx: &egui::Context) {
        let sides = self.encounters.run().map(|run| run.side_names());
        let live = self.encounters.run().is_some_and(|run| run.strip().running);
        let in_run = self
            .encounters
            .run()
            .map(|run| run.arrivals_in_run())
            .unwrap_or_default();
        self.lookup.run_open = self.encounters.has_run();
        self.world_page.run_open = self.encounters.has_run();
        if let Some(words) = self.world_page.ask_lookup.take() {
            self.notes_give_way();
            self.lookup.search_for(&words);
        }
        if let Some(path) = self.lookup.open_world_repair.take()
            && let Some(campaign) = self.campaigns.active_campaign()
        {
            self.world_page.open_repair(&campaign, &path);
            self.lookup.close();
            self.page = Page::World;
        }
        if let Some(card) = self.lookup.open_world_names.take() {
            // An unsaved NPC on the World page is never replaced. Lookup keeps its own, and the
            // World page's card shows the same help.
            if self.world_page.has_generated() {
                self.lookup.keep_generated(card);
                self.world_page.lookup_kept();
            } else {
                self.world_page.show_generated(card);
            }
            self.lookup.close();
            self.page = Page::World;
        }
        if self.world_page.wants_lookup_npc() {
            self.world_page
                .set_lookup_npc(self.lookup.generated_name().map(str::to_owned));
        }
        if let Some(id) = ctx
            .data_mut(|data| data.remove_temp::<uuid::Uuid>(egui::Id::new("reveal_encounter_npc")))
            && let Some(campaign) = self.campaigns.active_campaign()
        {
            self.world_page.reveal(campaign.id, id);
            self.page = Page::World;
        }
        if let Some(id) = self.library.open_world_npc.take()
            && let Some(campaign) = self.campaigns.active_campaign()
        {
            self.world_page.reveal(campaign.id, id);
            self.lookup.refresh();
            self.page = Page::World;
        }
        let mut refresh = std::mem::take(&mut self.world_page.reload);
        let played = self.sessions.current_link();
        let day = timeline::day_now(&self.timeline.events);
        let start_blocked = self.encounters.plan_blocked();
        // A Log line whose entry is gone waits for no one; the footer says its words.
        if let Some(index) = self.lookup.index() {
            self.world_page.reader.forget_gone_log_lines(&index);
            self.lookup.reader.forget_gone_log_lines(&index);
        }
        // An edit form in one reader holds back the other's Log field for that entry, and the
        // one save row on screen says so (ruling Q-t).
        let pass = ctx.cumulative_pass_nr();
        self.lookup
            .reader
            .share_with(&mut self.world_page.reader, pass);
        self.world_page
            .reader
            .share_with(&mut self.lookup.reader, pass);
        let mut start_fight = None;
        // A reader's `Add to fight` (round 27 FJ5), with the reader that asked, for its reason.
        let mut add_to_fight = None;
        // The campaign and run that own a refusal of `Add to fight`.
        let run_owner = self
            .encounters
            .run()
            .map(|run| run.id())
            .and_then(|run| Some((self.campaigns.active_campaign()?.id, run)));
        // Beside the encounter editor, Lookup's reader offers no Start fight: the editor's own
        // primary Start fight is the one there.
        self.lookup.reader.beside_editor =
            self.page == Page::Encounters && self.encounters.editor_open();
        for (at, reader) in [&mut self.world_page.reader, &mut self.lookup.reader]
            .into_iter()
            .enumerate()
        {
            reader.start_blocked = start_blocked;
            if let Some(fight) = reader.start_fight.take() {
                start_fight = Some(fight);
            }
            if let Some(fight) = reader.add_to_fight.take() {
                add_to_fight = Some((at, fight));
            }
            reader.played.clone_from(&played);
            reader.day = day;
            reader.sides = sides.clone();
            reader.run_owner = run_owner;
            reader.live = live;
            reader.in_run = in_run.clone();
            if let Some(index) = reader.select_run.take()
                && let Some(run) = self.encounters.run_mut()
            {
                run.select_arrival(index);
                self.page = Page::Run;
            }
            if let Some(id) = reader.open_timeline.take() {
                self.timeline_page.reveal(&id, &self.timeline);
                self.page = Page::Timeline;
            }
            if let Some(join) = reader.join.take()
                && let Some(run) = self.encounters.run_mut()
            {
                run.add_group(join.group, join.total);
                self.page = Page::Run;
                refresh |= join.saved;
            }
        }
        if refresh {
            self.lookup.refresh();
        }
        if let Some(fight) = start_fight {
            self.start_fight_with(&fight);
        }
        // The run's own add: one creature joins, and the run header's Undo takes it away. A
        // refusal waits in the asking reader's footer, owned by this campaign and run; a join
        // ends the entry's refusal in both readers.
        // An NPC joins under their own name with their stat block (round 28 FJ7).
        if let Some((at, add)) = add_to_fight
            && let Some(index) = self.lookup.index()
            && let Some(stat_block) = index
                .position(add.stat_block)
                .map(|position| std::sync::Arc::new(index.items[position].entry.clone()))
            && let Some(run) = self.encounters.run_mut()
        {
            let npc = add
                .npc
                .then(|| index.position(add.entry))
                .flatten()
                .map(|position| &index.items[position].entry);
            match run.add_entry_to_fight(stat_block, npc) {
                Ok(()) => {
                    self.world_page.reader.added_to_fight(add.entry);
                    self.lookup.reader.added_to_fight(add.entry);
                }
                Err(reason) => {
                    let reader = if at == 0 {
                        &mut self.world_page.reader
                    } else {
                        &mut self.lookup.reader
                    };
                    reader.refuse(add.entry, reason);
                }
            }
        }
        if let Some(roll) = roll_table::take_roll(ctx) {
            let result = match self.encounters.run_mut() {
                Some(run) if !run.ended() => run.roll_table(roll),
                _ => roll_table::roll_alone(&roll),
            };
            roll_table::settle_roll(ctx, result.unwrap_or_else(|error| error));
        }
    }

    /// A fight that nobody planned, from a reader's `Start fight`: it starts, or Encounters says
    /// why it cannot, as a planned fight's `Start fight` does.
    fn start_fight_with(&mut self, fight: &world::StartFight) {
        let Some(campaign) = self.campaigns.active_campaign() else {
            return;
        };
        self.encounters
            .start_fight_with(fight, &campaign, self.campaigns.modules());
        if !self.encounters.go_to_run {
            self.page = Page::Encounters;
        }
    }

    /// A press in Trackers, applied after Lookup drew. The act reads the day count at the click:
    /// the Timeline reads its file again if it changed, and Trackers follows it.
    fn apply_tracker_act(&mut self) {
        if self.trackers.pressed() {
            // When the file changed or cannot be read under an edit that waits for its pause,
            // the Timeline says so, and the day count is not known: no day act records.
            self.timeline.read_if_changed();
            self.trackers.follow_clock(trackers::clock(&self.timeline));
            self.trackers.apply_pressed();
        }
    }

    /// `Add to Timeline` after a tracker act: a second act, a Major event with the act's label
    /// and its line in the session being played, with the Timeline's own `Undo add`.
    fn add_tracker_act_to_timeline(&mut self) {
        if let Some((entry, title)) = self.trackers.wants_timeline.take() {
            let event = self
                .timeline
                .add(timeline::Kind::Event, self.trackers.played);
            if let Some(added) = self.timeline.event_mut(&event) {
                added.title = title.clone();
            }
            self.trackers.added = Some(trackers::Added {
                entry,
                event,
                title,
                at: std::time::Instant::now(),
                not_saved: None,
            });
        }
        if let Some(added) = &mut self.trackers.added {
            added.not_saved = self.timeline.event_not_saved(&added.event);
        }
        if let Some(event) = self.trackers.wants_undo_add.take() {
            self.timeline.undo_add(&event);
        }
    }

    /// One panel beside the page at a time.
    fn toggle_lookup(&mut self) {
        self.notes_give_way();
        self.lookup.toggle();
    }

    fn notes_give_way(&mut self) {
        self.notes_gave_way |= self.sessions.notes_open;
        self.sessions.notes_open = false;
    }

    fn toggle_notes(&mut self) {
        self.notes_gave_way = false;
        self.lookup.close();
        self.sessions.toggle_notes(self.page == Page::Sessions);
    }

    /// What is unsure, always where the DM looks (principle 6).
    fn statuses(&self, connection: Connection) -> Vec<Status<'static>> {
        let c = colors();
        let _ = connection;
        let mut account = self.settings.connection_status();
        if self.settings.signed_out() {
            account.word = "Signed out";
        }
        let mut statuses = vec![account];
        for (dirty, word) in [
            (self.library.dirty(), LIBRARY_DRAFT),
            (
                self.world_page.dirty() || self.any_unsaved_edits(),
                if self.world_page.dirty() {
                    WORLD_REPAIR_DRAFT
                } else {
                    WORLD_DRAFT
                },
            ),
            // A held NPC locks the campaign switcher, so the rail names it like other drafts.
            // Round 28 FJ6: the rail names an NPC that Lookup holds too, also when Lookup is closed.
            (
                self.world_page.has_generated() || self.lookup.generated_name().is_some(),
                world::page::UNSAVED_NPC,
            ),
            (
                self.encounters.encounter_dirty(),
                self.encounters.draft_word(),
            ),
            (self.encounters.creature_dirty(), encounters::CREATURE_DRAFT),
            (self.campaign_content.dirty(), CAMPAIGN_DRAFT),
            (
                self.encounters.settings_unsaved(),
                encounters::RUN_SETTINGS_DRAFT,
            ),
        ] {
            if dirty {
                statuses.push(Status {
                    icon: icons::DRAFT,
                    word,
                    color: c.document.unsaved,
                });
            }
        }
        if self.sessions.folder_missing() {
            statuses.push(Status {
                icon: icons::WARN,
                word: "Session folder not found",
                color: c.result.warning,
            });
        } else if self.timeline.unreadable.is_some() {
            statuses.push(Status {
                icon: icons::ERROR,
                word: "Timeline not readable",
                color: c.document.invalid,
            });
        }
        // A run files itself under this session, and the run's own page has no room to say so.
        // A run that is over files nothing more.
        let filing = self
            .run_fact
            .as_ref()
            .is_some_and(|fact| !matches!(fact.state, timeline::RunState::Ended(_)));
        if filing && self.old_session {
            // R41: the tone that the Session and Notes footers say it in.
            statuses.push(sessions::page::NotTonight::rail());
        }
        if self.party.sync_stopped {
            // R41: the tone of the Party footer's notice.
            statuses.push(party::SyncStopped::rail());
        }
        if self.party.sync_conflict() {
            statuses.push(Status {
                icon: icons::WARN,
                word: "Party sync conflict",
                color: c.sync.conflict,
            });
        }
        statuses
    }

    /// Before the Run page draws (egui and GPUI): the app's facts that the run shows. `room` is
    /// the page's width and `window` the window's content width. False when there is no run.
    pub(crate) fn run_page_begin(&mut self, room: f32, window: f32) -> bool {
        if !self.encounters.has_run() {
            return false;
        }
        self.encounters.run_reads();
        let party = &self.party;
        let copy = self.encounters.run_copy_label();
        if let Some(run) = self.encounters.run_mut() {
            run.copy_label = copy;
            run.account_status = Some(self.settings.connection_status());
            run.recording_notice = self.sessions.run_notice();
            let panel = self.lookup.open || self.sessions.notes_open;
            // Beside a panel, a letter is the next word typed, never a command. A click
            // on Next turn moves the caret, so stray typing must not change the run.
            run.lookup_open = panel;
            run.notes_open = self.sessions.notes_open;
            // A wide window has room for both, so the stat block stays.
            // The roster comes first. The stat block stays only with room for both.
            let stat_block = design::side_panel_width(window);
            run.hide_panel = panel && room - stat_block < design::RUN_COMPACT.max_width;
            run.content = self.lookup.index();
            let sheet = run.wanted_sheet().and_then(|id| party.sheet(&id));
            run.set_sheet(sheet);
        }
        true
    }

    /// After the Run page drew (egui and GPUI): `closed` when the DM closed the run.
    pub(crate) fn run_page_end(&mut self, closed: bool) {
        if closed {
            self.page = self.run_origin;
            self.sessions.forget_fight();
        }
        if let Some(words) = self
            .encounters
            .run_mut()
            .and_then(|run| run.for_note.take())
        {
            self.sessions.words_for_note(words);
        }
        if self
            .encounters
            .run_mut()
            .is_some_and(|run| std::mem::take(&mut run.return_focus))
        {
            self.sessions.after_a_step_back_to_note(self.notes_gave_way);
        }
        if let Some(run) = self.encounters.run_mut()
            && std::mem::take(&mut run.close_lookup)
        {
            // What the DM asked for is in the side panel. Notes give way only when
            // they hide it, and their words stay in the draft.
            if run.hide_panel {
                self.sessions.notes_open = false;
                self.notes_gave_way = false;
            }
            self.lookup.close();
        }
    }

    /// A press on a dice link. Beside the run, the run rolls it on its own dice and keeps it in
    /// the Log; anywhere else it is a preview roll that changes nothing.
    fn route_rolls(&mut self, ctx: &egui::Context) {
        let Some(ask) = rolls::take_ask(ctx) else {
            return;
        };
        // Only a number in a participant's own stat block or sheet rolls for the run. One in
        // Lookup, even beside the run, is a preview on the entry's own action line.
        match self
            .encounters
            .run_mut()
            .filter(|_| self.page == Page::Run && ask.roller.is_some())
        {
            Some(run) => run.roll_link(ask),
            None => rolls::preview(ctx, &ask),
        }
    }

    /// Requests that cross pages: read beside a draft, or open an entry where it lives.
    /// What the copy dialog asked for as it drew (both UIs): the copy opens in the Library,
    /// joins the open encounter draft, or refreshes the pages that read its folder; a dialog
    /// that closed goes.
    pub(crate) fn follow_copy_dialog(&mut self, ctx: &egui::Context, open: bool) {
        let Some(dialog) = &mut self.copy_dialog else {
            return;
        };
        if let Some(entry) = dialog.edit_copy.take() {
            self.library.edit_copy(entry);
            self.page = Page::Library;
        }
        if let Some(id) = dialog.add_to_draft.take() {
            self.encounters.add_to_draft(id);
        }
        if let Some(root) = dialog.refresh.take() {
            self.library
                .refresh_destination(&root, ctx, &self.campaigns);
            self.campaign_content.refresh_destination(&root);
            self.encounters.refresh_content(ctx, &self.campaigns, &root);
            self.lookup.refresh();
        }
        if !open {
            self.copy_dialog = None;
        }
    }

    fn route(&mut self, ctx: &egui::Context) {
        if let Some(target) = markdown_view::take_note_link(ctx) {
            let from = ctx.data_mut(|data| {
                data.remove_temp::<Option<uuid::Uuid>>(egui::Id::new("clicked_reader_document"))
                    .flatten()
            });
            self.notes_give_way();
            self.lookup.open_note_link(&target, from, ctx);
        }
        if let Some((package, entry)) = self.library.use_entry.take().or_else(|| {
            ctx.data_mut(|data| {
                data.remove_temp::<(uuid::Uuid, uuid::Uuid)>(egui::Id::new("use_package_entry"))
            })
        }) && let Some(campaign) = self.campaigns.active_campaign()
            && let Some(module) = self
                .campaigns
                .modules()
                .iter()
                .find(|module| module.id == package)
        {
            match content_workspace::use_in_campaign(&campaign, package, &module.name, Some(entry))
            {
                Ok(()) => {
                    self.library
                        .refresh_destination(&campaign.path, ctx, &self.campaigns);
                    self.campaign_content.refresh_destination(&campaign.path);
                    self.encounters
                        .refresh_content(ctx, &self.campaigns, &campaign.path);
                    self.lookup.refresh();
                }
                Err(error) => {
                    self.library.report_error(error);
                }
            }
        }
        if let Some(module) = self.campaign_content.found_package.take() {
            let root = module.path.clone();
            let result = self.campaigns.restore_module(module);
            if result.is_ok() {
                self.library
                    .refresh_destination(&root, ctx, &self.campaigns);
                self.encounters.refresh_content(ctx, &self.campaigns, &root);
                self.lookup.refresh();
            }
            self.campaign_content.package_restored(result);
        }
        if let Some(id) = self.campaign_content.forget_package.take() {
            let result = self.campaigns.unregister_module(id);
            if result.is_ok() {
                self.lookup.refresh();
            }
            self.campaign_content.package_restored(result);
        }
        // Issue #509: the Campaign page wrote the package choices. What Library, Encounters and
        // Lookup hold of what the campaign uses is now out of date.
        if std::mem::take(&mut self.campaign_content.choices_written)
            && let Some(campaign) = self.campaigns.active_campaign()
        {
            self.library
                .refresh_destination(&campaign.path, ctx, &self.campaigns);
            self.encounters
                .refresh_content(ctx, &self.campaigns, &campaign.path);
            self.lookup.refresh();
        }
        if let Some(words) = markdown_view::take_lookup_link(ctx) {
            self.notes_give_way();
            self.lookup.open_words(&words);
        }
        if let Some(id) = self.library.peek.take() {
            self.notes_give_way();
            self.lookup.open_id(id);
        }
        if let Some(id) = self.encounters.peek.take() {
            self.notes_give_way();
            self.lookup.open_creature_result(id);
        }
        if let Some(path) = ctx.data_mut(|data| {
            data.remove_temp::<std::path::PathBuf>(egui::Id::new("open_library_repair"))
        }) {
            if crate::world::repair_in_world(&path)
                && let Some(campaign) = self.campaigns.active_campaign()
                && let Ok(relative) = path.strip_prefix(&campaign.path)
            {
                self.world_page.reveal(
                    campaign.id,
                    crate::world::note_id(&relative.to_string_lossy().replace('\\', "/")),
                );
                self.page = Page::World;
            } else {
                let link = ctx.data_mut(|data| {
                    data.remove_temp::<Option<(splat::content::Reference, String)>>(egui::Id::new(
                        "open_library_repair_link",
                    ))
                });
                self.library.repair_file_link(path, link.flatten());
                self.page = Page::Library;
            }
        }
        // Prep's `Edit encounter` names the run's encounter by its ID; the Encounters page finds
        // it in its own list. When it cannot open, the reason goes back to the run's Prep.
        if let Some(id) =
            ctx.data_mut(|data| data.remove_temp::<uuid::Uuid>(egui::Id::new("edit_run_encounter")))
            && let Some(campaign) = self.campaigns.active_campaign()
        {
            match self.encounters.edit_from_plan(id, None, &campaign) {
                Ok(()) => self.page = Page::Encounters,
                Err(reason) => {
                    // The draft that refused it, by kind, so its words never go stale.
                    let refusal = match crate::design::first_unsaved(&self.encounters) {
                        Some(draft) => encounter_run::PrepRefusal::Draft(draft),
                        None => encounter_run::PrepRefusal::Other(reason),
                    };
                    if let Some(run) = self.encounters.run_mut() {
                        run.prep_refusal = Some(refusal);
                    }
                }
            }
        }
        // Round 30 review F3: a draft refusal ends when no draft blocks (saved, discarded, the
        // editor closed); it follows the draft that blocks now. Only while one is stored.
        if self.encounters.run().is_some_and(|run| {
            matches!(run.prep_refusal, Some(encounter_run::PrepRefusal::Draft(_)))
        }) {
            let now = crate::design::first_unsaved(&self.encounters);
            if let Some(run) = self.encounters.run_mut() {
                run.prep_refusal = now.map(encounter_run::PrepRefusal::Draft);
            }
        }
        if ctx
            .data_mut(|data| data.remove_temp::<bool>(egui::Id::new("open_campaign_packages")))
            .is_some()
        {
            self.page = Page::Campaign;
            self.campaign_tab = CampaignTab::Packages;
        }
        if let Some(position) = self
            .encounters
            .run_mut()
            .and_then(|run| run.lookup_request.take())
        {
            self.notes_give_way();
            self.lookup.open_position(position);
        }
        self.follow_world(ctx);
        if let Some(place) = self.timeline_page.open_place.take() {
            self.notes_give_way();
            self.lookup.open_id(place);
        }
        if std::mem::take(&mut self.campaigns.wants_encounters) {
            self.page = Page::Encounters;
        }
        let active = self.campaigns.active_campaign();
        // What the Library, Campaign and Lookup hold of this file is now out of date.
        if std::mem::take(&mut self.encounters.saved)
            && let Some(campaign) = &active
        {
            self.library
                .refresh_destination(&campaign.path, ctx, &self.campaigns);
            self.campaign_content.refresh_destination(&campaign.path);
            self.lookup.refresh();
        }
        if let Some(name) = self.encounters.saved_for_plan.take() {
            self.sessions.add_to_plan(&name);
        }
        if let Some((old, new)) = self.encounters.renamed_group.take() {
            self.sessions.rename_links(&old, &new);
        }
        if let Some(ticket) = self.sessions.wants_group_undo.take() {
            self.encounters.undo_group_discard_in_plan(ticket);
        }
        if std::mem::take(&mut self.sessions.wants_arrivals) {
            self.encounters.show_arrivals();
            self.page = Page::Run;
        }
        if std::mem::take(&mut self.sessions.wants_new_group) {
            // The plan's New group is blocked by the same value; a refusal keeps every draft.
            let _ = self.encounters.new_group_in_plan();
        }
        if let Some(path) = self.sessions.wants_group.take()
            && let Some(campaign) = &active
        {
            match self.encounters.open_group_in_plan(&path, campaign) {
                encounters::PlanOpen::Opened => {}
                // B4 review F1: the plan paints why under the group's line.
                encounters::PlanOpen::Refused(_) => {
                    self.sessions.plan_group_refused = self
                        .encounters
                        .plan_group_id(&path)
                        .map(|id| (self.sessions.selected_id(), id));
                }
                encounters::PlanOpen::Failed => self.page = Page::Encounters,
            }
        }
        // The end events of the plan's refusal, on any page, read only while one is stored
        // (B4b N1): another session is selected; nothing unsaved refuses any more (saved,
        // discarded, campaign changed); or a new read of the campaign lost the group (B4c F2,
        // also while the plan is not drawn). B4c F1: an unchanged frame compares no draft.
        match self.sessions.plan_group_refused {
            Some((session, group))
                if session != self.sessions.selected_id()
                    || !self.encounters.plan_refusal_stands(ctx, group) =>
            {
                self.sessions.plan_group_refused = None;
                self.encounters.forget_plan_refusal();
            }
            Some(_) => {}
            None => self.encounters.forget_plan_refusal(),
        }
        self.encounters.planned_groups = self.sessions.tonight_links();
        // The editor's way back belongs to its route: leaving the editor another way ends it.
        if self.page != Page::Encounters {
            self.encounters.end_session_route();
        }
        if let Some(session) = self.encounters.wants_session.take() {
            // Through the Session page's own guards for an unsaved note.
            self.sessions.select(Some(session));
            self.page = Page::Sessions;
        }
        if std::mem::take(&mut self.sessions.wants_campaign_link) {
            self.page = Page::Campaign;
            self.campaign_tab = CampaignTab::Link;
        }
        if std::mem::take(&mut self.trackers.wants_rules) {
            self.page = Page::Campaign;
            self.campaign_tab = CampaignTab::RunSettings;
        }
        if std::mem::take(&mut self.encounters.wants_run_folder) {
            self.page = Page::Campaign;
            self.campaign_tab = CampaignTab::RunSettings;
        }
        if let Some(campaign) = &active {
            if std::mem::take(&mut self.sessions.wants_runs_refresh) {
                self.encounters.read_runs_again(campaign);
            }
            let asked =
                self.sessions.wants_fight.is_some() || self.sessions.wants_saved_run.is_some();
            if let Some(path) = self.sessions.wants_fight.take() {
                self.encounters
                    .run_path(&path, campaign, self.campaigns.modules());
            }
            if let Some(path) = self.sessions.wants_saved_run.take() {
                self.encounters.open_saved_run(&path, campaign);
            }
            if let Some(fight) = self.sessions.wants_start_fight.take() {
                self.start_fight_with(&fight);
            }
            if let Some((id, name, words)) = self.sessions.wants_log.take() {
                self.notes_give_way();
                self.lookup.open_id(id);
                self.lookup.reader.open_log(campaign.id, id, &name, &words);
            }
            // Encounters shows why, and a button that seems dead tells the table nothing.
            if asked && !self.encounters.go_to_run {
                self.page = Page::Encounters;
            }
        }
        if std::mem::take(&mut self.sessions.wants_new_fight) {
            // Refused while a draft is unsaved; Encounters shows that draft and why.
            let _ = self.encounters.new_from_plan(self.sessions.selected_id());
            self.page = Page::Encounters;
        }
        if let Some(id) = self.sessions.wants_edit_encounter.take()
            && let Some(campaign) = &active
        {
            match self
                .encounters
                .edit_from_plan(id, self.sessions.selected_id(), campaign)
            {
                Ok(()) => self.page = Page::Encounters,
                Err(why) => self.sessions.encounter_not_opened(why),
            }
        }
        if std::mem::take(&mut self.sessions.wants_back_to_run)
            || std::mem::take(&mut self.timeline_page.wants_back_to_run)
        {
            self.page = Page::Run;
        }
        if let Some(id) = self.campaign_content.open_lookup.take() {
            self.lookup.open_id(id);
        }
        if std::mem::take(&mut self.sessions.wants_content_refresh) {
            self.lookup.refresh();
            self.lookup.files_changed = true;
        }
        if std::mem::take(&mut self.sessions.wants_index_refresh) {
            self.lookup.refresh();
        }
        if let Some(name) = self.sessions.wants_lookup.take() {
            self.notes_give_way();
            if !self.lookup.open_name(&name) {
                self.sessions.no_entry_named(&name);
            }
        }
        if std::mem::take(&mut self.encounters.go_to_run) {
            if std::mem::take(&mut self.encounters.run_from_session) {
                self.run_origin = Page::Sessions;
            } else if self.page != Page::Run {
                self.run_origin = self.page;
            }
            self.page = Page::Run;
        }
        if ctx
            .data_mut(|data| data.remove_temp::<bool>(egui::Id::new("open_account_settings")))
            .unwrap_or(false)
        {
            self.page = Page::Settings;
            self.settings_tab = SettingsTab::default();
            self.settings.focus_sign_in();
        }
        if let Some(run) = self.encounters.run_mut() {
            // An act that makes an ended fight live again goes by the rule, with the Timeline
            // as it is now.
            if let Some(act) = run.wants_unend.take() {
                run.unend(act, &mut self.timeline);
                ctx.request_repaint();
            }
            if std::mem::take(&mut run.wants_sign_in) {
                self.page = Page::Settings;
                self.settings_tab = SettingsTab::default();
                self.settings.focus_sign_in();
            }
            let new_group = std::mem::take(&mut run.wants_new_group);
            if std::mem::take(&mut run.wants_packages) {
                self.page = Page::Campaign;
                self.campaign_tab = CampaignTab::Packages;
            }
            let group = run.wants_group_editor.take();
            if let Some(path) = group
                && let Some(campaign) = &active
            {
                let _ = self.encounters.open_group_in_plan(&path, campaign);
                self.page = Page::Encounters;
            }
            if new_group {
                let _ = self.encounters.new_group_in_plan();
                self.page = Page::Encounters;
            }
        }
        if std::mem::take(&mut self.party_page.wants_settings) {
            self.settings_tab = SettingsTab::Account;
            self.settings.focus_sign_in();
            self.page = Page::Settings;
            self.settings_tab = SettingsTab::Account;
            self.settings.focus_sign_in();
        }
        if std::mem::take(&mut self.party_page.wants_link_page) {
            self.page = Page::Campaign;
            self.campaign_tab = CampaignTab::Link;
        }
        if std::mem::take(&mut self.sessions.wants_sessions_page) {
            self.page = Page::Sessions;
        }
        if std::mem::take(&mut self.sessions.wants_recording_input) {
            self.page = Page::Settings;
            self.settings_tab = SettingsTab::Recording;
            self.processing.focus_input = true;
        }
        if std::mem::take(&mut self.sessions.wants_processing_settings) {
            self.page = Page::Settings;
            self.settings_tab = SettingsTab::Recording;
            self.processing.focus_speech_key = std::mem::take(&mut self.sessions.wants_speech_key);
        }
        if let Some(id) = self.sessions.wants_event.take() {
            self.timeline_page.reveal(&id, &self.timeline);
            self.page = Page::Timeline;
        }
        if std::mem::take(&mut self.timeline_page.play_current) {
            self.sessions.play_current();
        }
        if let Some(session) = self.timeline_page.new_events_to.take() {
            self.sessions.set_current(session);
        }
        if let Some(session) = self.timeline_page.open_session.take() {
            self.sessions.open(session);
            self.page = Page::Sessions;
        }
        if let Some(root) = self.sessions.manifest_changed.take() {
            self.campaign_content.refresh_destination(&root);
            self.encounters.refresh_content(ctx, &self.campaigns, &root);
            self.timeline.open(None);
        }
        if let Some(source) = self.library.copy.take() {
            let mut dialog = copy_dialog::CopyDialog::new(source, &self.campaigns, ctx);
            if let Some(package) = self.library.copy_to_package.take() {
                dialog.prefer_package(package);
            }
            self.copy_dialog = Some(dialog);
        }
        match self.lookup.open_written.take() {
            Some(lookup::WrittenIn::Session(id)) => {
                self.sessions.open(id);
                self.page = Page::Sessions;
            }
            Some(lookup::WrittenIn::Event(id)) => {
                self.timeline_page.reveal(&id, &self.timeline);
                self.page = Page::Timeline;
            }
            None => {}
        }
        if let Some(entry) = self.lookup.open_in_library.take() {
            // Lookup offers this for typed World notes and plain ones alike.
            if (entry.kind == "world" || entry.kind == world::NOTE_KIND)
                && let Some(campaign) = self.campaigns.active_campaign()
            {
                self.world_page.reveal(campaign.id, entry.id);
                self.lookup.close();
                self.page = Page::World;
            } else if self.library.reveal(&entry) {
                self.page = Page::Library;
            }
        }
    }
}

/// The app's frame logic that draws nothing (`gui::SplatRoot` calls it each frame).
impl Splat {
    /// Poll background work (sign-in, D&D Beyond, art, Lookup's index, encounters, processing,
    /// sessions, trackers) and follow the open run into the Timeline. Returns the account's
    /// session epoch and session.
    fn poll_background(&mut self, ctx: &egui::Context) -> (u64, Option<auth::Session>) {
        self.settings.poll(ctx);
        let (epoch, session) = self.settings.campaign_session();
        self.dndbeyond.begin_frame();
        self.dndbeyond
            .poll(ctx, epoch, session.is_some(), &mut self.campaigns);
        self.art.poll(ctx, self.campaigns.modules());
        if ctx
            .data_mut(|data| data.remove_temp::<bool>(egui::Id::new("encounter_identity_changed")))
            .is_some()
        {
            self.lookup.refresh();
            self.lookup.files_changed = true;
        }
        self.lookup.poll(ctx, &self.campaigns);
        if std::mem::take(&mut self.lookup.files_changed) {
            let roots: Vec<_> = self
                .campaigns
                .modules()
                .iter()
                .map(|m| m.path.clone())
                .chain(self.campaigns.active_campaign().map(|c| c.path))
                .collect();
            for root in roots {
                self.library
                    .refresh_destination(&root, ctx, &self.campaigns);
                self.campaign_content.refresh_destination(&root);
                self.encounters.refresh_content(ctx, &self.campaigns, &root);
            }
        }
        ctx.data_mut(|data| {
            let key = egui::Id::new("reader_link_index");
            if let Some(index) = self.lookup.index() {
                data.insert_temp(key, index);
            } else {
                data.remove::<std::sync::Arc<lookup::ContentIndex>>(key);
            }
        });
        self.encounters.poll(ctx, &self.campaigns);
        self.lookup.library_draft = self.library.dirty();
        self.sessions.run_open = self.encounters.has_run();
        self.library.run_open = self.encounters.has_run();
        // Remove's Undo in the Library belongs to the campaign that was open: a switch ends it at
        // once, on whichever page.
        self.library
            .note_campaign(self.campaigns.active_campaign().map(|campaign| campaign.id));
        self.campaign_content.run_open = self.encounters.has_run();
        self.processing.poll(ctx);
        self.processing.recording_now = self.sessions.recording();
        let open = self.campaigns.active_campaign();
        self.timeline.run_open = self.encounters.has_run();
        self.sessions.poll(ctx, open.as_ref(), &mut self.timeline);
        self.sessions
            .expire_event_link_undo(self.page == Page::Timeline, &self.timeline);
        let played = match self.sessions.played_session() {
            sessions::page::Played::Session(id) => Some(id),
            _ => None,
        };
        trackers::poll(
            &mut self.trackers,
            self.lookup.index(),
            open.as_ref(),
            played,
            &self.timeline,
        );
        self.old_session = self.sessions.current_dated().is_some();
        let fact = self.encounters.run().map(encounter_run::Run::timeline_fact);
        if fact != self.run_fact {
            match &fact {
                Some(fact) => {
                    let road = self.road_of_entering_fight(fact);
                    self.timeline.record_change(
                        self.run_fact.as_ref(),
                        fact,
                        self.sessions.current(),
                        road.as_ref().map(|(_, within)| within),
                    );
                    if let Some((mark, _)) = &road {
                        self.confirm_road(fact, mark);
                    }
                }
                // The run closed, so its last result goes to the file now and not after the
                // pause.
                None => self.timeline.save(),
            }
            self.run_fact = fact;
        }
        // Continue fight is for the fight that was played last. An ended fight on screen is
        // told whether it is that fight, when the Timeline or that fight changed.
        self.encounters.follow_continue_rule(&self.timeline);
        self.update_carry();
        self.sessions.open_run = self.run_fact.as_ref().and_then(|fact| {
            let event = self
                .timeline
                .events
                .iter()
                .find(|e| e.run == Some(fact.id))?;
            Some((fact.name.clone(), event.session?))
        });
        (epoch, session)
    }

    /// Party sync and the run's D&D Beyond sync, for the account's session.
    fn sync_party(&mut self, ctx: &egui::Context, epoch: u64, session: Option<auth::Session>) {
        let active = self.campaigns.active_campaign();
        self.party.sync(
            ctx,
            epoch,
            active.as_ref().and_then(|c| {
                c.dndbeyond_campaign_id
                    .clone()
                    .map(|remote| (c.id, remote, c.path.clone()))
            }),
            session.clone(),
            self.page == Page::Party,
        );
        if std::mem::take(&mut self.party.new_copies)
            && let Some(campaign) = &active
        {
            self.encounters
                .refresh_content(ctx, &self.campaigns, &campaign.path);
        }
        self.encounters
            .tick_sync(ctx, session, epoch, active.as_ref());
    }

    /// What follows a change of page: a length typed on the page left is settled, notices end,
    /// and the keyboard goes back to the open note.
    fn follow_page_change(&mut self) {
        // Coming back to a page beside the open note, the keyboard goes back to the note.
        if self.page != self.shown_page {
            // A length typed on the page the DM left is the journey's now, as at focus loss.
            timeline::checks::settle_length(&mut self.timeline, None);
            self.campaign_content.clear_notice();
            self.encounters.clear_notice();
            self.sessions.clear_notice();
            self.world_page.reader.clear_notice();
            let from_sessions = self.shown_page == Page::Sessions;
            self.shown_page = self.page;
            if from_sessions {
                // The note can have changed there, so an old caret could land inside a line.
                self.sessions.fresh_focus_in_panel();
            } else if self.page != Page::Sessions {
                self.sessions.return_focus_to_note();
            }
        }
    }
}

impl Splat {}
