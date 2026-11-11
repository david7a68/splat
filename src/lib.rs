//! Desktop application and shared content APIs.
pub use splat_core::{character, content, execution, memo};
extern crate self as splat;

// The visual suite runs many app instances at once. With glibc's malloc it used
// more memory and CPU; see `visual_tests::limit_allocator`.
#[cfg(all(test, not(target_env = "msvc")))]
#[global_allocator]
static TEST_ALLOCATOR: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod art;
pub mod auth;
mod campaign_content;
mod campaigns;
mod combat_sync;
mod commands;
mod content_workspace;
mod copy_dialog;
mod creature_filter;
mod credentials;
mod design;
mod dndbeyond;
mod encounter_run;
mod encounter_settings;
mod encounters;
mod file_batch;
mod folder_check;
mod hermetic;
mod history;
mod library;
#[cfg(test)]
mod load_bench;
mod lookup;
mod loot_editor;
mod markdown_source;
mod markdown_view;
mod matching;
mod npc_generator;
mod party;
mod party_bar;
mod party_stream;
mod party_sync;
mod persistence;
mod processing;
mod recording;
mod roll_table;
mod rolls;
mod sessions;
mod settings;
mod sheet_writer;
mod timeline;
mod touchpad;
mod ui_state;
#[cfg(test)]
mod visual_tests;
mod world;

use design::{Action, Density, Status, Tone, Type, colors, icons};
use eframe::egui;

pub fn run() -> eframe::Result {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        std::process::exit(commands::run(&args, &mut std::io::stdout().lock()));
    }
    let _instance = if let Some(dirs) = directories::ProjectDirs::from("", "", "splat") {
        match ui_state::InstanceLock::acquire(&dirs.config_dir().join("instance.lock")) {
            Ok(lock) => Some(lock),
            Err(error) => {
                rfd::MessageDialog::new()
                    .set_title("Splat is already open")
                    .set_description(error)
                    .show();
                return Ok(());
            }
        }
    } else {
        None
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([960.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Splat",
        options,
        Box::new(|cc| {
            design::theme::install(&cc.egui_ctx);
            art::install(&cc.egui_ctx);
            Ok(Box::<Splat>::default())
        }),
    )
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
    encounters: encounters::Encounters,
    lookup: lookup::Lookup,
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
    tab_direction: Option<egui::FocusDirection>,
    step_past: Option<(egui::Id, egui::FocusDirection)>,
    #[cfg(test)]
    gallery: bool,
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
            encounters: Default::default(),
            lookup: Default::default(),
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
            tab_direction: None,
            step_past: None,
            #[cfg(test)]
            gallery: false,
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
    /// Open where the DM left off: the open run, then the last page. The first time,
    /// Encounters if a campaign is active, otherwise the Library, which needs none.
    fn start(&mut self, ctx: &egui::Context) {
        self.started = true;
        let saved = self.ui_store.load();
        design::restore_column_widths(ctx, saved.column_widths);
        self.touchpad_speed = saved.touchpad_speed;
        self.sessions.restored_session = saved.playing;
        self.sessions.party_bar = saved.party_bar;
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
    }

    fn remember(&mut self, ctx: &egui::Context) {
        // A drag writes the file once, when it ends, and not on each frame of it.
        if ctx.input(|i| i.pointer.any_down()) {
            return;
        }
        let active = self.campaigns.active_campaign();
        let state = ui_state::UiState {
            column_widths: design::column_widths(ctx),
            touchpad_speed: self.touchpad_speed,
            party_bar: self.sessions.party_bar,
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
        };
        self.ui_store.save(&state);
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

    /// Tab must not stop on a scroll handle: it has no name and no visible focus. egui
    /// has no way to take one out of the order, so step over it here.
    fn skip_unnamed_focus(&mut self, ctx: &egui::Context) {
        let (forward, back) = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::Tab) && !i.modifiers.shift,
                i.key_pressed(egui::Key::Tab) && i.modifiers.shift,
            )
        });
        if forward {
            self.tab_direction = Some(egui::FocusDirection::Next);
        } else if back {
            self.tab_direction = Some(egui::FocusDirection::Previous);
        }
        self.step_past = None;
        let Some(direction) = self.tab_direction else {
            return;
        };
        let Some(id) = ctx.memory(|memory| memory.focused()) else {
            return;
        };
        let unnamed = ctx
            .accesskit_node_builder(id, |node| {
                node.role() == egui::accesskit::Role::Unknown && node.label().is_none()
            })
            .unwrap_or(false);
        if unnamed {
            self.step_past = Some((id, direction));
            ctx.request_repaint();
        } else {
            self.tab_direction = None;
        }
    }

    /// Egui drops a focus move at the start of each frame, so a move made at the end of
    /// `update` is lost. The move is made here, before any widget asks for focus.
    fn apply_focus_step(&self, ctx: &egui::Context) {
        if let Some((id, direction)) = self.step_past
            && ctx.memory(|memory| memory.focused()) == Some(id)
        {
            ctx.memory_mut(|memory| memory.move_focus(direction));
        }
    }

    #[cfg(test)]
    fn active_campaign_name(&self) -> Option<String> {
        self.campaigns.active_campaign().map(|c| c.name)
    }

    /// The page the rail marks. The design gallery is not one of them.
    fn visible_page(&self) -> Option<Page> {
        #[cfg(test)]
        if self.gallery {
            return None;
        }
        Some(self.page)
    }

    fn blocked_switch(&self) -> Option<&'static str> {
        if self.encounters.has_run() {
            Some("Close the run to change campaigns.")
        } else if self.sessions.recording() {
            Some("Stop the recording to change campaigns.")
        } else if self.sessions.processing() {
            Some("Wait for processing to finish, or cancel it, to change campaigns.")
        } else if self.sessions.busy_with_edit() || self.timeline.not_saved() {
            Some("Resolve the unsaved session file to change campaigns.")
        } else if self.world_page.dirty()
            || self.world_page.has_generated()
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
                    "D&D Beyond has not confirmed changes to {}. The saved run keeps HP, death saves and conditions, and asks you to choose when you next sync. {}",
                    names.join(", "),
                    if self.encounters.run().is_some_and(|run| !run.unsent_only_here_names().is_empty()) { "Unsent spell slots and limited uses are lost." } else { "Closing ends Undo for this run." }
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

    fn close_prompt(&mut self, ctx: &egui::Context) {
        let items = self.unsaved_items();
        // Sync finished, or a draft was saved, while the prompt was open.
        if items.is_empty() {
            self.confirm_close = false;
            return;
        }
        let only_sync = items.len() == 1 && !self.encounters.pending_sync_names().is_empty();
        let conflict = self.encounters.run().and_then(|run| run.strip().conflict);
        let only_run_unsaved = items.len() == 1 && self.encounters.run_not_saved().is_some();
        let sync_loss = self
            .encounters
            .run()
            .is_some_and(|run| !run.unsent_only_here_names().is_empty());
        let background =
            usize::from(self.sessions.recording()) + usize::from(self.sessions.processing());
        let (title, keep_label, discard_label, advice) = if items.len() == background {
            (
                if self.sessions.recording() {
                    "Recording has not finished"
                } else {
                    "Processing has not finished"
                },
                if self.sessions.recording() {
                    "Keep recording"
                } else {
                    "Keep processing"
                },
                "Stop and close",
                "",
            )
        } else if only_run_unsaved {
            (
                "This run could not be saved",
                "Keep running",
                "Close and lose changes",
                "Closing loses changes since the last save.",
            )
        } else if only_sync {
            (
                if conflict.is_some() {
                    "Sync conflict"
                } else if self.settings.signed_out() {
                    "Changes not sent to D&D Beyond"
                } else {
                    "Sync has not finished"
                },
                "Keep running",
                if sync_loss {
                    "Close and lose changes"
                } else {
                    "Close now"
                },
                if conflict.is_some() {
                    "Choose which HP to keep, or close and choose next time."
                } else if self.settings.signed_out() {
                    "Connect to send these changes. Closing keeps the saved run."
                } else {
                    "Keep running to finish sync. Closing keeps the saved run."
                },
            )
        } else {
            (
                "Unsaved changes",
                "Keep editing",
                "Discard and close",
                "Return and save, or close and lose these changes.",
            )
        };
        let title = if items.len() == background {
            self.sessions
                .recording_label()
                .map(|label| format!("“{label}” is recording"))
                .or_else(|| {
                    self.sessions
                        .processing_label()
                        .map(|label| format!("“{label}” is processing"))
                })
                .unwrap_or_else(|| title.to_owned())
        } else if only_sync && let Some(name) = &conflict {
            format!("{name} has a sync conflict")
        } else {
            title.to_owned()
        };
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut save_close = false;
        let mut discard = false;
        let mut save_new = false;
        design::dialog(ctx, "unsaved_on_close", &title, |ui| {
            if !only_sync || conflict.is_none() {
                for (item, _) in &items {
                    ui.add(egui::Label::new(item.as_str()).wrap());
                }
            }
            if !advice.is_empty() {
                ui.label(Type::Small.rich(ui, advice).color(colors().muted));
            }
            ui.add_space(design::S);
            let loses = items.len() != background && (!only_sync || sync_loss);
            let chosen = encounter_run::view::close_recovery_actions(
                ui,
                only_sync && conflict.is_some(),
                only_sync && self.settings.signed_out(),
            );
            if chosen.review {
                if let Some(run) = self.encounters.run_mut() {
                    run.review_conflict();
                }
                self.lookup.open = false;
                self.sessions.notes_open = false;
                self.page = Page::Run;
                self.confirm_close = false;
            }
            if chosen.connect {
                self.page = Page::Settings;
                self.settings_tab = SettingsTab::default();
                self.settings.focus_sign_in();
                self.confirm_close = false;
            }
            let (lose, ()) = design::dialog_actions(
                ui,
                |ui| loses && Action::destructive(discard_label).show(ui).clicked(),
                |ui| {
                    if only_run_unsaved {
                        save_new = Action::primary("Save as a new run and close")
                            .show(ui)
                            .clicked();
                    } else if items.len() == 1 && self.library.can_save_and_close() {
                        save_close = Action::primary("Save and close").show(ui).clicked();
                    } else if !loses {
                        discard = Action::secondary(discard_label).show(ui).clicked();
                    }
                    let response = Action::secondary(keep_label).show(ui);
                    let id = egui::Id::new("app_close_default_focused");
                    if !ui.data(|data| data.get_temp::<bool>(id).unwrap_or(false)) {
                        response.request_focus();
                        ui.data_mut(|data| data.insert_temp(id, true));
                    }
                    keep |= response.clicked();
                },
            );
            discard |= lose;
        });
        if keep {
            self.confirm_close = false;
            // The first thing named is where the DM goes, if anything needs them there.
            if items.len() != background {
                self.page = items[0].1;
            }
            // Lookup is a panel, not a page, so it opens beside the page on its draft.
            if let Some((id, _)) = self.lookup.reader.unsaved_edits().first() {
                self.lookup.open_id(*id);
            }
            ctx.data_mut(|data| data.insert_temp(egui::Id::new("resume_draft_focus"), self.page));
            ctx.request_repaint();
        }
        if save_close && self.library.save_for_close() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            self.confirm_close = false;
        }
        if save_new && let Some(run) = self.encounters.run_mut() {
            match run.save_as_new_run() {
                Ok(()) => discard = true,
                Err(_) => {
                    self.confirm_close = false;
                    self.page = Page::Run;
                }
            }
        }
        if discard {
            self.discard_on_close = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn rail(&mut self, ctx: &egui::Context, connection: Connection) {
        let c = colors();
        let at_table = self.visible_page() == Some(Page::Run);
        let rail = if !at_table && ctx.content_rect().width() >= design::RAIL_WIDE_FROM {
            design::RAIL_WIDE_WIDTH
        } else {
            design::RAIL_WIDTH
        };
        let page = self.visible_page();
        design::Column::left(
            "navigation",
            rail,
            design::RAIL_WIDTH..=if at_table {
                rail
            } else {
                design::RAIL_DRAG_MAX_WIDTH
            },
        )
        .page_keeps(design::RAIL_LEAVES_WINDOW)
        .frame(
            egui::Frame::new()
                .fill(c.rail)
                .inner_margin(egui::Margin::same(design::S as i8)),
        )
        .show(ctx, |ui| {
            if self.copy_dialog.is_some() || self.confirm_close {
                ui.disable();
            }
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(Type::Title.rich(ui, "Splat").color(c.text));
            ui.add_space(design::XS);
            let blocked = self.blocked_switch();
            ui.spacing_mut().interact_size.y = Density::Read.control();
            if self.campaigns.selector(ui, blocked) {
                self.campaigns.focus_add();
                self.settings_tab = SettingsTab::Account;
                self.page = Page::Settings;
            }
            ui.add_space(design::S);
            if !self.encounters.has_run() {
                let fights = self.encounters.paused_run_count();
                let paused = self.encounters.paused_run().map(|run| {
                    (
                        run.path.clone(),
                        run.encounter.clone(),
                        crate::encounters::run_state_word(run, false),
                    )
                });
                if let Some((path, name, state)) = paused {
                    if design::paused_run_nav(ui, fights, &name, &state).clicked() {
                        if fights > 1 {
                            self.page = Page::Encounters;
                        } else if let Some(active) = self.campaigns.active_campaign() {
                            self.encounters.open_saved_run(&path, &active);
                        }
                    }
                } else {
                    // The Run slot stays when no fight is open, as tall as a run's item, so
                    // Encounters, Session and the rest never move when a run opens or closes.
                    let response =
                        design::nav_item(ui, icons::RUN, "Run", Some(NO_FIGHT_OPEN), false);
                    design::named(ui, &response, &no_run_name());
                    if response
                        .on_hover_text(format!("{NO_FIGHT_OPEN}. Encounters lists the fights."))
                        .clicked()
                    {
                        self.page = Page::Encounters;
                    }
                }
            } else {
                let detail = self.encounters.run().map(|run| run.state_words(false));
                let copy = self.encounters.run_copy_label();
                if design::run_nav_item(
                    ui,
                    detail.as_deref().unwrap_or("Setup"),
                    page == Some(Page::Run),
                )
                .on_hover_text(format!(
                    "{}{}",
                    detail.as_deref().unwrap_or("Setup"),
                    copy.map_or(String::new(), |copy| format!(" · {copy}"))
                ))
                .clicked()
                {
                    self.page = Page::Run;
                }
            }
            let review = self.campaign_content.review_detail();
            for (page, icon) in [
                (Page::Encounters, icons::ENCOUNTERS),
                (Page::Sessions, icons::SESSIONS),
                (Page::Timeline, icons::TIMELINE),
                (Page::World, icons::WORLD),
                (Page::Party, icons::PARTY),
                (Page::Library, icons::LIBRARY),
                (Page::Campaign, icons::CAMPAIGN),
            ] {
                let current = self.visible_page() == Some(page);
                let detail = match page {
                    Page::Campaign => review.as_deref(),
                    _ => None,
                };
                if design::nav_item(ui, icon, page.name(), detail, current).clicked() {
                    self.page = page;
                }
            }
            // Lookup, Notes, Settings and the account line keep fixed places at the rail's bottom
            // edge. The status slot grows upward above Lookup. Both are drawn in reading order,
            // so Tab still follows it; each is placed by its height from the last pass.
            let mut statuses = self.statuses(connection).into_iter();
            // The account line sits last, below Settings.
            let account = statuses.next();
            let rail_rect = ui.max_rect();
            let slot_id = ui.id().with("rail_status_slot");
            let lower_id = ui.id().with("rail_lower_group");
            let (slot_height, lower_height) =
                ui.data(|d| (d.get_temp::<f32>(slot_id), d.get_temp::<f32>(lower_id)));
            let slot_height = slot_height.unwrap_or(0.0);
            // The account slot always holds two lines, so a status that wraps or adds a line
            // never moves Lookup, Notes or Settings.
            let account_slot = 2.0 * Type::Small.line();
            let lower_height = lower_height.unwrap_or(
                design::XS + 3.0 * (Density::Scan.control() + 2.0) + design::XS + account_slot,
            );
            let top = ui.cursor().top();
            let lower_top = (rail_rect.bottom() - lower_height).max(top + slot_height);
            let slot_top = (lower_top - slot_height).max(top);
            let slot = ui.scope_builder(
                egui::UiBuilder::new().max_rect(egui::Rect::from_min_max(
                    egui::pos2(rail_rect.left(), slot_top),
                    egui::pos2(rail_rect.right(), lower_top.max(slot_top)),
                )),
                |ui| {
                    if let (Some((icon, state, color)), Some(word)) = (
                        self.sessions
                            .rail_recording_status()
                            .map(|status| (status.icon, status.word, status.color)),
                        self.sessions.recording_detail(),
                    ) {
                        let response = design::status_link(
                            ui,
                            Status {
                                icon,
                                word: &word,
                                color,
                            },
                        );
                        let full = if state == "Processed" && self.sessions.job_warned() {
                            sessions::page::processed_warning().word.to_owned()
                        } else {
                            self.sessions
                                .recording_hover()
                                .unwrap_or_else(|| state.to_owned())
                        };
                        if state == "Recording silence" || full != state {
                            design::named(ui, &response, &full);
                        }
                        if response.on_hover_text(full).clicked() {
                            self.sessions.open_live_recording();
                            self.page = Page::Sessions;
                        }
                    }
                    for state in statuses.by_ref() {
                        self.rail_status(ui, state, false);
                    }
                    // Words that could not be saved lead to where the DM can choose.
                    for (lost, word, target) in [
                        (
                            self.sessions.busy_with_edit(),
                            if self.sessions.notes_changed_on_disk() {
                                "Notes changed"
                            } else {
                                "Notes not saved"
                            },
                            Page::Sessions,
                        ),
                        (
                            self.timeline.not_saved(),
                            self.timeline
                                .attention_status()
                                .map_or("Timeline not saved", |state| state.word),
                            Page::Timeline,
                        ),
                    ] {
                        if lost {
                            let response = design::status_link(
                                ui,
                                Status {
                                    icon: self
                                        .sessions
                                        .attention_status()
                                        .filter(|_| target == Page::Sessions)
                                        .or_else(|| {
                                            self.timeline
                                                .attention_status()
                                                .filter(|_| target == Page::Timeline)
                                        })
                                        .map_or(icons::ERROR, |state| state.icon),
                                    word,
                                    color: self
                                        .sessions
                                        .attention_status()
                                        .filter(|_| target == Page::Sessions)
                                        .or_else(|| {
                                            self.timeline
                                                .attention_status()
                                                .filter(|_| target == Page::Timeline)
                                        })
                                        .map_or(colors().document.invalid, |state| state.color),
                                },
                            );
                            if target == Page::Sessions && self.sessions.notes_changed_on_disk() {
                                design::named(ui, &response, "Notes changed on disk");
                            }
                            if response
                                .on_hover_text(
                                    if target == Page::Sessions
                                        && self.sessions.notes_changed_on_disk()
                                    {
                                        "Notes changed on disk"
                                    } else {
                                        word
                                    },
                                )
                                .clicked()
                            {
                                self.page = target;
                            }
                        }
                    }
                },
            );
            let lower = ui.scope_builder(
                egui::UiBuilder::new().max_rect(egui::Rect::from_min_max(
                    egui::pos2(rail_rect.left(), lower_top),
                    egui::pos2(rail_rect.right(), rail_rect.bottom().max(lower_top)),
                )),
                |ui| {
                    ui.add_space(design::XS);
                    if design::nav_toggle(ui, icons::SEARCH, "Lookup", "Ctrl+K", self.lookup.open)
                        .on_hover_text("Search everything, beside this page (Ctrl+K)")
                        .clicked()
                    {
                        self.toggle_lookup();
                    }
                    if design::nav_toggle(
                        ui,
                        icons::DRAFT,
                        "Notes",
                        "Ctrl+J",
                        self.sessions.notes_open && self.page != Page::Sessions,
                    )
                    .on_hover_text("This session’s notes, beside this page (Ctrl+J)")
                    .clicked()
                    {
                        self.toggle_notes();
                    }
                    if design::nav_item(
                        ui,
                        icons::SETTINGS,
                        Page::Settings.name(),
                        None,
                        page == Some(Page::Settings),
                    )
                    .clicked()
                    {
                        self.page = Page::Settings;
                    }
                    if let Some(account) = account {
                        ui.add_space(design::XS);
                        let top = ui.cursor().top();
                        self.rail_status(ui, account, true);
                        ui.expand_to_include_y(top + account_slot);
                    }
                },
            );
            // A new height places the group again before anything is painted.
            let measured = [
                (slot_id, slot_height, slot.response.rect.height()),
                (lower_id, lower_height, lower.response.rect.height()),
            ];
            if measured
                .iter()
                .any(|(_, before, now)| (before - now).abs() > 0.5)
            {
                ui.data_mut(|d| {
                    for (id, _, now) in measured {
                        d.insert_temp(id, now);
                    }
                });
                ui.ctx()
                    .request_discard("The rail’s status slot changed height");
            }
        });
    }

    /// One status line in the rail. Some lead to where the DM can act on them.
    /// The account line also shows a spinner while the party is read.
    fn rail_status(&mut self, ui: &mut egui::Ui, state: Status<'static>, account: bool) {
        let held = self.world_page.held_npc().map(|name| {
            format!(
                "{}: {name}. Show its card on World.",
                world::page::UNSAVED_NPC
            )
        });
        let full = match state.word {
            world::page::UNSAVED_NPC => held.as_deref().unwrap_or(state.word),
            "Signed out" => "Signed out of D&D Beyond",
            "Sync stopped" => {
                "Party sync stopped when D&D Beyond disconnected. Connect, then turn sync on in Party."
            }
            _ => state.word,
        };
        let detail = state.word == "Connected · not remembered";
        let (response, word) = if state.word == "Sync stopped" {
            let response = design::status_link(ui, state);
            if response.clicked() {
                self.party.wants_combat_sync = true;
                self.page = Page::Party;
            }
            {
                let rect = response.rect;
                (response, rect)
            }
        } else if state.word == "Session folder not found" {
            let response = design::status_link(ui, state);
            if response.clicked() {
                self.page = Page::Sessions;
            }
            {
                let rect = response.rect;
                (response, rect)
            }
        } else if state.word == world::page::UNSAVED_NPC {
            // The held NPC's card is on World, wherever the DM is now.
            let response = design::status_link(ui, state);
            if response.clicked() {
                self.world_page.open_held();
                self.page = Page::World;
            }
            {
                let rect = response.rect;
                (response, rect)
            }
        } else if detail {
            design::rail_status_detail(
                ui,
                Status {
                    word: "Connected",
                    ..state
                },
                "Not remembered",
            )
        } else {
            let response = design::wrapped_status(ui, state);
            {
                let rect = response.rect;
                (response, rect)
            }
        };
        // A read can also come from the cache on disk, so the words say updating, not refreshing.
        let updating = account && self.party.connected() && self.party.busy();
        const UPDATING: &str = " · Updating the party";
        let full = if updating {
            format!("{full}{UPDATING}")
        } else {
            full.to_owned()
        };
        if updating {
            design::busy_after(ui, word, colors().connection.busy);
        }
        let response = response.on_hover_text(&full);
        // The second line of a detail status is its own node, so the name leaves it out.
        let name = if detail && updating {
            format!("Connected{UPDATING}")
        } else {
            full.clone()
        };
        if name != state.word {
            design::named(ui, &response, &name);
        }
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
        let log_as = self.sessions.current_link().map(|session| {
            match timeline::day_now(&self.timeline.events) {
                Some(day) => format!("{session}, Day {day}"),
                None => session,
            }
        });
        for reader in [&mut self.world_page.reader, &mut self.lookup.reader] {
            reader.log_as.clone_from(&log_as);
            reader.sides = sides.clone();
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
        if let Some(roll) = roll_table::take_roll(ctx) {
            let key = roll.key.clone();
            let result = match self.encounters.run_mut() {
                Some(run) if !run.ended() => run.roll_table(roll),
                _ => roll_table::roll_alone(&roll),
            };
            roll_table::settle_roll(ctx, &key, result.unwrap_or_else(|error| error));
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
            (self.library.dirty(), "Library draft"),
            (
                self.world_page.dirty() || self.any_unsaved_edits(),
                if self.world_page.dirty() {
                    "World repair draft"
                } else {
                    "World draft"
                },
            ),
            // A held NPC locks the campaign switcher, so the rail names it like other drafts.
            (self.world_page.has_generated(), world::page::UNSAVED_NPC),
            (
                self.encounters.encounter_dirty(),
                self.encounters.draft_word(),
            ),
            (self.encounters.creature_dirty(), "Creature draft"),
            (self.campaign_content.dirty(), "Campaign draft"),
            (self.encounters.settings_unsaved(), "Run settings draft"),
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
            statuses.push(Status {
                icon: icons::WARN,
                word: "Old session",
                color: c.result.warning,
            });
        }
        if self.party.sync_stopped {
            statuses.push(Status {
                icon: icons::SYNC_OFF,
                word: "Sync stopped",
                color: c.sync.pending,
            });
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

    /// While a run is open, every other page keeps the turn in view.
    fn run_strip(&mut self, ctx: &egui::Context) {
        let Some(strip) = self
            .encounters
            .run()
            .filter(|_| self.page != Page::Run)
            .map(encounter_run::Run::strip)
        else {
            return;
        };
        let c = colors();
        egui::TopBottomPanel::top("run_strip")
            .resizable(false)
            .exact_height(Density::Glance.control() + design::S)
            .frame(
                egui::Frame::new()
                    .fill(c.current_row)
                    .inner_margin(egui::Margin::symmetric(design::M as i8, design::XS as i8)),
            )
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if strip.running {
                        ui.label(
                            Type::Heading
                                .rich(ui, format!("Round {}", strip.round))
                                .color(c.text),
                        );
                        ui.add(
                            egui::Label::new(
                                Type::Name
                                    .rich(
                                        ui,
                                        format!(
                                            "{}’s turn",
                                            strip.current.as_deref().unwrap_or("?")
                                        ),
                                    )
                                    .color(c.turn.current),
                            )
                            .truncate(),
                        );
                        if let Some(next) = &strip.next {
                            ui.add(
                                egui::Label::new(
                                    Type::Small
                                        .rich(ui, format!("Next: {next}"))
                                        .color(c.turn.next),
                                )
                                .truncate(),
                            );
                        }
                    } else {
                        ui.label(
                            Type::Heading
                                .rich(
                                    ui,
                                    format!(
                                        "{} · {}",
                                        self.encounters
                                            .run()
                                            .map(|run| run.state_words(false))
                                            .unwrap_or_default(),
                                        strip.name
                                    ),
                                )
                                .color(c.text),
                        );
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if strip.running
                            && Action::primary("Next turn")
                                .icon(icons::NEXT)
                                .show(ui)
                                .clicked()
                            && let Some(run) = self.encounters.run_mut()
                        {
                            run.next_turn();
                            self.sessions.after_a_step_back_to_note(self.notes_gave_way);
                        }
                        if Action::secondary("Back to run").show(ui).clicked() {
                            self.page = Page::Run;
                        }
                        if strip.not_saved {
                            design::status(
                                ui,
                                Status {
                                    icon: icons::ERROR,
                                    word: "Not saved",
                                    color: c.document.invalid,
                                },
                            );
                        }
                        if strip.sync_stopped {
                            design::status(
                                ui,
                                Status {
                                    icon: icons::SYNC_OFF,
                                    word: "Sync stopped",
                                    color: c.sync.pending,
                                },
                            );
                        }
                        if let Some(name) = strip
                            .sync_error
                            .as_ref()
                            .filter(|_| !strip.sync_stopped && strip.conflict.is_none())
                        {
                            design::status(
                                ui,
                                Status {
                                    icon: icons::ERROR,
                                    word: &format!("Sync error: {name}"),
                                    color: c.result.error,
                                },
                            );
                        }
                        if let Some(name) = &strip.conflict {
                            if Action::quiet("Review")
                                .name("Review sync conflict")
                                .show(ui)
                                .clicked()
                                && let Some(run) = self.encounters.run_mut()
                            {
                                run.review_conflict();
                                self.page = Page::Run;
                            }
                            design::status(
                                ui,
                                Status {
                                    icon: icons::WARN,
                                    word: &format!("Conflict: {name}"),
                                    color: c.sync.conflict,
                                },
                            );
                        }
                    });
                });
            });
    }

    /// The party at a glance, on the Session page while a session is played.
    fn party_bar(&mut self, ctx: &egui::Context) {
        let here = self.page == Page::Sessions
            && self.campaigns.active_campaign().is_some()
            && self.party.has_members();
        let shows = here && self.sessions.party_bar_shows();
        // The Session page's header offers the bar back.
        self.sessions.party_hidden = here && !shows;
        if !shows {
            return;
        }
        let run = self
            .encounters
            .run()
            .map(encounter_run::Run::party_now)
            .unwrap_or_default();
        let glances = self.party.glances(&run);
        let source = self.party.glance_source(!run.is_empty());
        // While a fight is open, the bar holds its level until the fight ends.
        let fight = self
            .encounters
            .run()
            .filter(|run| !run.ended())
            .map(encounter_run::Run::id);
        let mut choice = None;
        // A band of the page, not a raised layer: the panel's own hairline closes it.
        egui::TopBottomPanel::top("party_bar")
            .resizable(false)
            .exact_height(party_bar::height() + 2.0 * design::XS)
            .frame(
                egui::Frame::new()
                    .fill(colors().page)
                    .inner_margin(egui::Margin::symmetric(design::M as i8, design::XS as i8)),
            )
            .show(ctx, |ui| {
                if self.copy_dialog.is_some() || self.confirm_close {
                    ui.disable();
                }
                choice = party_bar::show(ui, &glances, &source, fight);
            });
        match choice {
            Some(party_bar::Choice::Open(id)) => {
                self.party.choose(id);
                self.page = Page::Party;
            }
            Some(party_bar::Choice::Hide) => self.sessions.party_bar = Some(false),
            None => {}
        }
    }

    fn no_campaign(&mut self, ui: &mut egui::Ui) {
        let no_campaigns = self.campaigns.all().is_empty();
        design::page_header(ui, self.page.name(), None, |_| {});
        let mut add = false;
        design::empty_state(
            ui,
            icons::CAMPAIGN,
            if no_campaigns {
                "No campaign yet"
            } else {
                "No campaign open"
            },
            "A campaign keeps your notes, party and encounters together. The Library works without one.",
            |ui| {
                if self.page == Page::Campaign && no_campaigns {
                    self.campaigns.show_add_form(ui);
                } else {
                    add = Action::primary(if no_campaigns {
                        "Add campaign"
                    } else {
                        "Choose a campaign"
                    })
                    .show(ui)
                    .clicked();
                }
            },
        );
        if add {
            if no_campaigns {
                self.campaigns.focus_add();
            }
            self.settings_tab = SettingsTab::Account;
            self.page = Page::Settings;
        }
    }

    fn page(&mut self, ui: &mut egui::Ui) {
        let active = self.campaigns.active_campaign();
        let (_, session) = self.settings.campaign_session();
        match self.page {
            Page::Settings => {
                design::page_header(ui, "Settings", None, |_| {});
                let tab_changed = design::segmented(
                    ui,
                    &mut self.settings_tab,
                    &[
                        (SettingsTab::Account, "Account and campaigns"),
                        (SettingsTab::Recording, "Recording and processing"),
                        (SettingsTab::Touchpad, "Touchpad"),
                    ],
                );
                if tab_changed {
                    self.processing.look_again();
                }
                design::rule(ui);
                if self.settings_tab == SettingsTab::Touchpad {
                    ui.add_space(design::L);
                    design::group_heading(ui, "Touchpad", 0);
                    design::small_field(
                        ui,
                        "Scrolling speed",
                        Some("Mouse wheels keep their speed."),
                        None,
                        |ui| {
                            design::segmented(
                                ui,
                                &mut self.touchpad_speed,
                                &touchpad::TouchpadSpeed::ALL,
                            )
                        },
                    );
                    return;
                }
                if self.settings_tab == SettingsTab::Recording {
                    // Line-start quiet actions use the gutter for their hit area.
                    // Keep fields at the usual column edge inside the wider viewport.
                    let mut viewport = ui.available_rect_before_wrap();
                    viewport.min.x -= design::S;
                    // Keep scrolled text below a clear margin under the tab rule.
                    viewport.min.y += design::L;
                    let clip = viewport.intersect(ui.clip_rect());
                    ui.scope_builder(egui::UiBuilder::new().max_rect(viewport), |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("recording_settings")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.set_clip_rect(ui.clip_rect().intersect(clip));
                                let mut fields = ui.available_rect_before_wrap();
                                fields.min.x += design::S;
                                ui.scope_builder(egui::UiBuilder::new().max_rect(fields), |ui| {
                                    ui.set_max_width(design::FORM_WIDTH + design::XXL);
                                    self.processing.show(ui);
                                });
                            });
                    });
                    return;
                }
                let blocked = self.blocked_switch();
                egui::ScrollArea::vertical()
                    .id_salt("settings_page")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(design::L);
                        let account = |ui: &mut egui::Ui, settings: &mut settings::Settings| {
                            settings.show(ui);
                        };
                        let list =
                            |ui: &mut egui::Ui,
                             campaigns: &mut campaigns::Campaigns,
                             dndbeyond: &mut dndbeyond::DndBeyond| {
                                campaigns.show(ui, session.clone(), dndbeyond, blocked);
                            };
                        if ui.ctx().content_rect().width() >= 1000.0 {
                            // A clear gutter, so that the left column's text does not run on.
                            // Not `ui.columns`, whose justified layout stretches the words in a paragraph.
                            let width = (ui.available_width() - design::XL) / 2.0;
                            ui.horizontal_top(|ui| {
                                ui.spacing_mut().item_spacing.x = design::XL;
                                let column = egui::vec2(width, 0.0);
                                let layout = egui::Layout::top_down(egui::Align::Min);
                                for place in 0..2 {
                                    ui.allocate_ui_with_layout(column, layout, |ui| {
                                        // Both ways, so the second column starts in one place.
                                        ui.set_width(width);
                                        ui.spacing_mut().item_spacing.x = 8.0;
                                        if place == 0 {
                                            list(ui, &mut self.campaigns, &mut self.dndbeyond);
                                        } else {
                                            self.settings.campaign_form_open =
                                                self.campaigns.form_open();
                                            account(ui, &mut self.settings);
                                        }
                                    });
                                }
                            });
                        } else {
                            self.settings.campaign_form_open = self.campaigns.form_open();
                            if self.encounters.has_run() {
                                self.settings.show_compact(ui);
                                ui.add_space(design::L);
                                list(ui, &mut self.campaigns, &mut self.dndbeyond);
                            } else {
                                list(ui, &mut self.campaigns, &mut self.dndbeyond);
                                ui.add_space(design::L);
                                self.settings.show_compact(ui);
                            }
                        }
                    });
            }
            Page::Sessions => {
                if let Some(campaign) = &active {
                    let fights = self.encounters.plan_fights();
                    let saved_runs = self.encounters.saved_runs();
                    let groups = self.encounters.plan_groups();
                    let blocked = self.encounters.plan_blocked();
                    let group_blocked = self.encounters.plan_group_blocked();
                    let group_open = self.encounters.group_open_in_plan();
                    let group_discarded = self.encounters.group_discarded_in_plan();
                    let group_discarded_name = self.encounters.discarded_group_name();
                    let arrivals_open = self.encounters.run_takes_arrivals();
                    let run_directory = self.encounters.run_directory(campaign);
                    let open_state = self
                        .encounters
                        .run()
                        .and_then(crate::encounter_run::Run::live_info)
                        .map(|run| crate::encounters::run_state_word(&run, false))
                        .unwrap_or_default();
                    let (encounters, campaigns) = (&mut self.encounters, &self.campaigns);
                    let mut group_editor = |ui: &mut egui::Ui| {
                        encounters.group_in_plan_view(ui, campaign, campaigns);
                    };
                    let plan = sessions::page::Plan {
                        fights: &fights,
                        saved_runs: &saved_runs,
                        groups: &groups,
                        group_blocked,
                        group_open,
                        group_discarded,
                        group_discarded_name,
                        arrivals_open,
                        group_editor: std::cell::RefCell::new(&mut group_editor),
                        blocked,
                        run_directory,
                        open_run: self.run_fact.as_ref().map(|fact| fact.id),
                        open_state,
                        open_encounter: self.run_fact.as_ref().map(|fact| fact.name.as_str()),
                    };
                    self.sessions
                        .show(ui, &self.processing, &mut self.timeline, &plan);
                } else {
                    self.no_campaign(ui);
                }
            }
            Page::Timeline => {
                if self.shown_page != Page::Timeline {
                    self.timeline_page.reopen();
                }
                if let Some(campaign) = &active {
                    let links = self.sessions.links();
                    let current = self.sessions.current();
                    self.timeline_page.open_run = self.run_fact.as_ref().map(|fact| fact.id);
                    self.timeline.run_open = self.encounters.has_run();
                    self.timeline_page.saved_runs = self.encounters.saved_runs();
                    self.timeline_page.dated = self.sessions.current_dated();
                    self.timeline_page.places = self
                        .lookup
                        .index()
                        .map(|index| index.places())
                        .unwrap_or_default();
                    self.timeline_page.show(
                        ui,
                        &mut self.timeline,
                        campaign.id,
                        &links,
                        current,
                        &mut self.sessions,
                    );
                } else {
                    self.no_campaign(ui);
                }
            }
            Page::Library => {
                self.library.show(ui, &mut self.campaigns);
                if let Some((id, name)) = self.library.renamed.take()
                    && let Err(error) = self.campaigns.rename_module(id, name)
                {
                    design::message(ui, Tone::Error, &error);
                }
                if std::mem::take(&mut self.library.wants_settings) {
                    if self.campaigns.all().is_empty() {
                        self.campaigns.focus_add();
                    }
                    self.settings_tab = SettingsTab::Account;
                    self.page = Page::Settings;
                }
                if let Some(root) = self.library.content_changed.take() {
                    self.campaign_content.refresh_destination(&root);
                    self.encounters
                        .refresh_content(ui.ctx(), &self.campaigns, &root);
                    self.lookup.refresh();
                }
            }
            Page::Encounters => {
                if active.is_some() {
                    self.encounters.save_shortcut(ui.ctx(), &self.campaigns);
                    let sessions = self.sessions.links();
                    self.encounters.show(
                        ui,
                        &self.campaigns,
                        &self.party,
                        &encounters::Story {
                            events: &self.timeline.events,
                            sessions: &sessions,
                        },
                    );
                } else {
                    self.no_campaign(ui);
                }
            }
            Page::World => {
                if let Some(campaign) = &active {
                    let index = self.lookup.index();
                    self.world_page.show(ui, index.as_ref(), campaign);
                } else {
                    self.no_campaign(ui);
                }
            }
            Page::Party => {
                self.party.run_open = self.run_fact.is_some();
                if active.is_some() {
                    self.party.show(
                        ui,
                        session,
                        active.is_some_and(|c| c.dndbeyond_campaign_id.is_some()),
                    );
                } else {
                    self.no_campaign(ui);
                }
            }
            Page::Campaign => {
                let Some(_campaign) = active else {
                    self.no_campaign(ui);
                    return;
                };
                let mut open_entries = false;
                design::page_header(
                    ui,
                    "Campaign",
                    (self.campaign_tab == CampaignTab::Packages)
                        .then(|| self.campaign_content.context())
                        .as_deref(),
                    |ui| {
                        open_entries = Action::quiet("Campaign entries")
                            .icon(icons::DOCUMENT)
                            .hint("This campaign’s own entries, in the Library")
                            .show(ui)
                            .clicked();
                    },
                );
                if open_entries {
                    self.library.show_campaign_entries();
                    self.page = Page::Library;
                    return;
                }
                design::segmented(
                    ui,
                    &mut self.campaign_tab,
                    &[
                        (CampaignTab::Packages, "Packages"),
                        (CampaignTab::RunSettings, "Rules and runs"),
                        (CampaignTab::Link, "Link and folder"),
                    ],
                );
                ui.add_space(design::XS);
                match self.campaign_tab {
                    CampaignTab::Packages => {
                        self.campaign_content.show(ui, &self.campaigns);
                        if std::mem::take(&mut self.campaign_content.open_run_settings) {
                            self.campaign_tab = CampaignTab::RunSettings;
                        }
                    }
                    CampaignTab::RunSettings => {
                        self.encounters.run_settings(ui, &self.campaigns);
                    }
                    CampaignTab::Link => {
                        // The same guard as Settings: a link or folder change would stop
                        // the run's sync, or hide an open draft.
                        let blocked = self.blocked_switch();
                        egui::ScrollArea::vertical()
                            .id_salt("campaign_link")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                self.campaigns.show_active(
                                    ui,
                                    session,
                                    &mut self.dndbeyond,
                                    blocked,
                                );
                                design::locked(ui, blocked, |ui| {
                                    let details_open = self.campaigns.show_form_if_editing(ui);
                                    design::rule(ui);
                                    self.sessions.folder_view(
                                        ui,
                                        &self.timeline,
                                        blocked,
                                        details_open,
                                    );
                                });
                            });
                    }
                }
            }
            Page::Run => {
                let Some(campaign) = active else {
                    self.page = Page::Encounters;
                    return;
                };
                if !self.encounters.has_run() {
                    self.page = Page::Encounters;
                    return;
                }
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
                    let stat_block = design::side_panel_width(ui.ctx().content_rect().width());
                    run.hide_panel =
                        panel && ui.available_width() - stat_block < design::RUN_COMPACT.max_width;
                    run.content = self.lookup.index();
                    let sheet = run.wanted_sheet().and_then(|id| party.sheet(&id));
                    run.set_sheet(sheet);
                }
                if self.encounters.show_run(ui, &campaign) {
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
                    self.library.refresh_destination(&campaign.path, ctx);
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
                self.library.refresh_destination(&root, ctx);
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
        if let Some(path) =
            ctx.data_mut(|data| data.remove_temp::<String>(egui::Id::new("edit_run_encounter")))
            && let Some(campaign) = self.campaigns.active_campaign()
        {
            self.encounters.edit_path(&path, &campaign);
            self.page = Page::Encounters;
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
            self.library.refresh_destination(&campaign.path, ctx);
            self.campaign_content.refresh_destination(&campaign.path);
            self.lookup.refresh();
        }
        if let Some(name) = self.encounters.saved_for_plan.take() {
            self.sessions.add_to_plan(&name);
        }
        if let Some((old, new)) = self.encounters.renamed_group.take() {
            self.sessions.rename_links(&old, &new);
        }
        if std::mem::take(&mut self.sessions.wants_group_undo) {
            self.encounters.undo_group_discard_in_plan();
        }
        if std::mem::take(&mut self.sessions.wants_arrivals) {
            self.encounters.show_arrivals();
            self.page = Page::Run;
        }
        if std::mem::take(&mut self.sessions.wants_new_group) {
            self.encounters.new_group_in_plan();
        }
        if let Some(path) = self.sessions.wants_group.take()
            && let Some(campaign) = &active
            && !self.encounters.open_group_in_plan(&path, campaign)
        {
            self.page = Page::Encounters;
        }
        self.encounters.planned_groups = self.sessions.tonight_links();
        if std::mem::take(&mut self.encounters.wants_session) {
            self.page = Page::Sessions;
        }
        if std::mem::take(&mut self.sessions.wants_campaign_link) {
            self.page = Page::Campaign;
            self.campaign_tab = CampaignTab::Link;
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
            // Encounters shows why, and a button that seems dead tells the table nothing.
            if asked && !self.encounters.go_to_run {
                self.page = Page::Encounters;
            }
        }
        if std::mem::take(&mut self.sessions.wants_new_fight) {
            self.encounters.new_from_plan();
            self.page = Page::Encounters;
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
                self.encounters.open_group_in_plan(&path, campaign);
                self.page = Page::Encounters;
            }
            if new_group {
                self.encounters.new_group_in_plan();
                self.page = Page::Encounters;
            }
        }
        if std::mem::take(&mut self.party.wants_settings) {
            self.settings_tab = SettingsTab::Account;
            self.settings.focus_sign_in();
            self.page = Page::Settings;
            self.settings_tab = SettingsTab::Account;
            self.settings.focus_sign_in();
        }
        if std::mem::take(&mut self.party.wants_link_page) {
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
            if entry.kind == "world"
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

impl eframe::App for Splat {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        // eframe calls this before the first `update`, where `start` restores the rest.
        if !self.started {
            self.touchpad_speed = self.ui_store.load().touchpad_speed;
        }
        self.touchpad_speed.apply(&mut raw_input.events);
        history::set_aside(ctx, &mut raw_input.events);
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if !self.started {
            self.start(ctx);
        }
        self.campaign_content.tick(ctx, &self.campaigns);
        if ctx.input(|i| i.viewport().close_requested()) {
            // An edit in its pause before saving is saved now, not reported as a loss.
            self.sessions.flush();
            self.timeline.run_open = self.encounters.has_run();
            self.timeline.save();
            if (self.library.dirty()
                || self.world_page.dirty()
                || self.world_page.has_generated()
                || self.any_unsaved_edits()
                || self.campaign_content.dirty()
                || self.encounters.unsaved()
                || self.encounters.pending_sync()
                || self.party.unconfirmed()
                || self.sessions.recording()
                || self.sessions.processing()
                || self.sessions.busy_with_edit()
                || self.timeline.not_saved())
                && !self.discard_on_close
            {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.confirm_close = true;
            }
        }
        if let Some(page) =
            ctx.data_mut(|data| data.remove_temp::<Page>(egui::Id::new("resume_draft_focus")))
        {
            match page {
                Page::Library => self.library.refocus_draft(),
                Page::Sessions => self.sessions.return_focus_to_note(),
                Page::Encounters => self.encounters.refocus_draft(),
                _ => {}
            }
        }
        if self.confirm_close {
            self.close_prompt(ctx);
        }
        if !self.confirm_close {
            ctx.data_mut(|data| data.remove::<bool>(egui::Id::new("app_close_default_focused")));
        }
        self.apply_focus_step(ctx);
        self.settings.poll(ctx);
        let (epoch, session) = self.settings.campaign_session();
        self.dndbeyond.begin_frame();
        self.dndbeyond
            .poll(ctx, epoch, session.is_some(), &mut self.campaigns);
        self.art.poll(ctx, self.campaigns.modules());
        art::show_viewer(ctx);
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
                self.library.refresh_destination(&root, ctx);
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
        self.campaign_content.run_open = self.encounters.has_run();
        self.processing.poll(ctx);
        self.processing.recording_now = self.sessions.recording();
        let open = self.campaigns.active_campaign();
        self.timeline.run_open = self.encounters.has_run();
        self.sessions.poll(ctx, open.as_ref(), &mut self.timeline);
        self.sessions
            .expire_event_link_undo(self.page == Page::Timeline, &self.timeline);
        self.old_session = self.sessions.current_dated().is_some();
        let fact = self.encounters.run().map(encounter_run::Run::timeline_fact);
        if fact != self.run_fact {
            if let Some(fact) = &fact {
                self.timeline
                    .record_change(self.run_fact.as_ref(), fact, self.sessions.current());
            }
            self.run_fact = fact;
        }
        self.update_carry();
        self.sessions.open_run = self.run_fact.as_ref().and_then(|fact| {
            let event = self
                .timeline
                .events
                .iter()
                .find(|e| e.run == Some(fact.id))?;
            Some((fact.name.clone(), event.session?))
        });
        let modal = self.copy_dialog.is_some()
            || self.confirm_close
            || art::viewer_open(ctx)
            || ctx.memory(|memory| memory.top_modal_layer()).is_some();
        if !modal {
            if ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::K)) {
                self.toggle_lookup();
            }
            if ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::J)) {
                self.toggle_notes();
            }
            // Not on Sessions, where the panel is away, and not over a footer's second step.
            if self.sessions.notes_open
                && self.page != Page::Sessions
                && !(self.page == Page::Run
                    && self
                        .encounters
                        .run()
                        .is_some_and(|run| run.footer_step_open()))
                && !egui::Popup::is_any_open(ctx)
                && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                self.sessions.notes_open = false;
            }
            // A dialog takes Escape first, and a form elsewhere has nothing to do with it.
            if self.lookup.open
                && !self.lookup.reader.takes_escape()
                && ctx.memory(|memory| memory.top_modal_layer()).is_none()
                && !egui::Popup::is_any_open(ctx)
                && !(self.page == Page::Library && self.library.form_open())
                && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
            {
                self.lookup.close();
            }
        }
        let connection = if self.settings.connecting() {
            Connection::Connecting
        } else if self.settings.refresh_failing() {
            Connection::Failing
        } else if session.is_some() {
            Connection::Connected
        } else {
            Connection::Offline
        };
        self.rail(ctx, connection);
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
        self.sessions.on_page = self.page == Page::Sessions;
        self.sessions.banner(
            ctx,
            &self.processing,
            &self.timeline,
            self.encounters.has_run(),
        );
        design::reserve_primary(ctx, false);
        self.lookup.history_input(ctx, self.page == Page::Library);
        self.route_rolls(ctx);
        self.run_strip(ctx);
        self.party_bar(ctx);
        design::reserve_primary(ctx, self.encounters.has_run() && self.page != Page::Run);
        // With no panel left, going on typing must reach the note and never change the run.
        if self.notes_gave_way && !self.lookup.open {
            self.notes_gave_way = false;
            self.sessions.notes_come_back();
        }
        if self.lookup.open {
            if self.lookup.wants_written() {
                self.lookup
                    .set_written(self.sessions.written(&self.timeline));
            }
            self.lookup.show(ctx);
        }
        // The Sessions page has this note itself, so two editors would squeeze a small window.
        if self.sessions.notes_open && self.page != Page::Sessions {
            self.sessions.notes_panel(ctx);
        }
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(colors().page)
                    .inner_margin(egui::Margin::symmetric(design::M as i8, design::S as i8)),
            )
            .show(ctx, |ui| {
                if self.copy_dialog.is_some() || self.confirm_close {
                    ui.disable();
                }
                #[cfg(test)]
                if self.gallery {
                    design::gallery::show(ui);
                    return;
                }
                self.page(ui);
            });
        design::reserve_primary(ctx, false);
        self.route(ctx);
        if let Some(dialog) = &mut self.copy_dialog {
            dialog.draft = self.encounters.draft_name();
            let open = dialog.show(ctx, &self.campaigns);
            if let Some(entry) = dialog.edit_copy.take() {
                self.library.edit_copy(entry);
                self.page = Page::Library;
            }
            if let Some(id) = dialog.add_to_draft.take() {
                self.encounters.add_to_draft(id);
            }
            if let Some(root) = dialog.refresh.take() {
                self.library.refresh_destination(&root, ctx);
                self.campaign_content.refresh_destination(&root);
                self.encounters.refresh_content(ctx, &self.campaigns, &root);
                self.lookup.refresh();
            }
            if !open {
                self.copy_dialog = None;
            }
        }
        // Coming back to a page beside the open note, the keyboard goes back to the note.
        if self.page != self.shown_page {
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
        self.skip_unnamed_focus(ctx);
        self.remember(ctx);
    }
}
