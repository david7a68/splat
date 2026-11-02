pub mod auth;
mod campaign_content;
mod campaigns;
mod combat_sync;
mod content_workspace;
mod copy_dialog;
mod credentials;
mod design;
mod dndbeyond;
mod encounter_run;
mod encounter_settings;
mod encounters;
mod hermetic;
mod library;
mod lookup;
mod markdown_view;
mod party;
mod party_stream;
mod party_sync;
mod persistence;
mod processing;
mod recording;
mod sessions;
mod settings;
mod timeline;
mod ui_state;
#[cfg(test)]
mod visual_tests;

use design::{Action, Density, Status, Tone, Type, colors, icons};
use eframe::egui;

fn main() -> eframe::Result {
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
            Ok(Box::<Splat>::default())
        }),
    )
}

struct Splat {
    page: Page,
    campaign_tab: CampaignTab,
    settings_tab: SettingsTab,
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
    processing: processing::settings::ProcessingSettings,
    /// What the open run last told the timeline.
    run_fact: Option<timeline::RunFact>,
    /// New events and runs go to a session dated before today.
    old_session: bool,
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
            processing: Default::default(),
            run_fact: None,
            old_session: false,
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

/// One flat level. A run is a place, not a mode of a tab.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
enum Page {
    Run,
    #[default]
    Encounters,
    Sessions,
    Timeline,
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
            Self::Sessions => "Sessions",
            Self::Timeline => "Timeline",
            Self::Party => "Party",
            Self::Library => "Library",
            Self::Campaign => "Campaign",
            Self::Settings => "Settings",
        }
    }
    fn from_name(name: &str) -> Option<Self> {
        [
            Self::Encounters,
            Self::Sessions,
            Self::Timeline,
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
}

impl Splat {
    /// Open where the DM left off: the open run, then the last page. The first time,
    /// Encounters if a campaign is active, otherwise the Library, which needs none.
    fn start(&mut self) {
        self.started = true;
        let saved = self.ui_store.load();
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
                .restore_run(&open.path, active.id, &open.synced);
        }
    }

    fn remember(&mut self) {
        let active = self.campaigns.active_campaign();
        let state = ui_state::UiState {
            page: Some(self.page.name().to_owned()).filter(|_| self.page != Page::Run),
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
        } else if self.campaign_content.dirty()
            || self.encounters.dirty()
            || self.library.campaign_draft()
        {
            Some("Save or discard your changes to change campaigns.")
        } else {
            None
        }
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
                format!("Library entry \u{201c}{name}\u{201d} in {owner} has unsaved changes."),
                Page::Library,
            ));
        }
        if self.campaign_content.dirty() {
            items.push((
                format!("Package choices for {campaign} have unsaved changes."),
                Page::Campaign,
            ));
        }
        if self.encounters.draft_dirty() {
            let name = self.encounters.draft_name().unwrap_or_default();
            items.push((
                format!("Encounter \u{201c}{name}\u{201d} has unsaved changes."),
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
                    "D&D Beyond has not confirmed changes to {}. They can be lost.",
                    names.join(", ")
                ),
                Page::Run,
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
        if let Some(label) = self.sessions.recording_label() {
            items.push((
                format!(
                    "{label} is recording. Closing stops it, and what is recorded so far is kept."
                ),
                Page::Sessions,
            ));
        }
        if let Some(label) = self.sessions.processing_label() {
            items.push((
                format!(
                    "{label} is processing. Closing stops it. What is done is kept, and processing it again goes on from there."
                ),
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
        let background =
            usize::from(self.sessions.recording()) + usize::from(self.sessions.processing());
        let (title, keep_label, discard_label, advice) = if items.len() == background {
            (
                "Work in progress",
                "Keep working",
                "Stop and close",
                "Nothing already saved is lost either way.",
            )
        } else if only_sync {
            (
                "Sync has not finished",
                "Return to run",
                "Close anyway",
                "Return, and wait for sync or resolve the conflict. Or close and lose these changes.",
            )
        } else {
            (
                "Unsaved changes",
                "Keep editing",
                "Discard and close",
                "Return and save, or close and lose these changes.",
            )
        };
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut discard = false;
        design::dialog(ctx, "unsaved_on_close", title, |ui| {
            for (item, _) in &items {
                ui.add(egui::Label::new(item.as_str()).wrap());
            }
            ui.label(Type::Small.rich(ui, advice).color(colors().muted));
            ui.add_space(design::S);
            ui.horizontal(|ui| {
                keep |= Action::primary(keep_label).show(ui).clicked();
                ui.add_space(design::L);
                discard = Action::destructive(discard_label).show(ui).clicked();
            });
        });
        if keep {
            self.confirm_close = false;
            // The first thing named is where the DM goes, if anything needs them there.
            if items.len() != background {
                self.page = items[0].1;
            }
        }
        if discard {
            self.discard_on_close = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn rail(&mut self, ctx: &egui::Context, connection: Connection) {
        let c = colors();
        let rail = if ctx.content_rect().width() >= design::RAIL_WIDE_FROM {
            design::RAIL_WIDE_WIDTH
        } else {
            design::RAIL_WIDTH
        };
        let page = self.visible_page();
        egui::SidePanel::left("navigation")
            .resizable(false)
            .exact_width(rail)
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
                ui.add_enabled_ui(blocked.is_none(), |ui| {
                    ui.spacing_mut().interact_size.y = Density::Read.control();
                    self.campaigns.selector(ui, blocked);
                });
                ui.add_space(design::S);
                if self.encounters.has_run() {
                    let strip = self.encounters.run().map(|run| run.strip());
                    let detail = strip.as_ref().map(|strip| {
                        if strip.running {
                            format!("Round {}", strip.round)
                        } else {
                            "Not in combat".to_owned()
                        }
                    });
                    if design::nav_item(
                        ui,
                        icons::RUN,
                        Page::Run.name(),
                        detail.as_deref(),
                        page == Some(Page::Run),
                    )
                    .clicked()
                    {
                        self.page = Page::Run;
                    }
                }
                let recording = self.sessions.recording_detail();
                for (page, icon) in [
                    (Page::Encounters, icons::ENCOUNTERS),
                    (Page::Sessions, icons::SESSIONS),
                    (Page::Timeline, icons::TIMELINE),
                    (Page::Party, icons::PARTY),
                    (Page::Library, icons::LIBRARY),
                    (Page::Campaign, icons::CAMPAIGN),
                ] {
                    let current = self.visible_page() == Some(page);
                    let detail = recording.as_deref().filter(|_| page == Page::Sessions);
                    if design::nav_item(ui, icon, page.name(), detail, current).clicked() {
                        self.page = page;
                    }
                }
                // Push the rest down in tree order, so Tab still follows reading order.
                let statuses = self.statuses(connection);
                let status_height = statuses.len() as f32 * (Type::Small.line() + 2.0);
                let lost = usize::from(self.sessions.busy_with_edit())
                    + usize::from(self.timeline.not_saved());
                let bottom = 3.0 * (Density::Scan.control() + 2.0)
                    + status_height
                    + lost as f32 * (Density::Read.control() + 2.0)
                    + design::S;
                ui.add_space((ui.available_height() - bottom).max(0.0));
                for state in statuses {
                    design::status(ui, state);
                }
                // Words that could not be saved lead to where the DM can choose.
                for (lost, word, target) in [
                    (
                        self.sessions.busy_with_edit(),
                        "Notes not saved",
                        Page::Sessions,
                    ),
                    (
                        self.timeline.not_saved(),
                        "Timeline not saved",
                        Page::Timeline,
                    ),
                ] {
                    if lost
                        && design::status_link(
                            ui,
                            Status {
                                icon: icons::ERROR,
                                word,
                                color: colors().document.invalid,
                            },
                        )
                        .clicked()
                    {
                        self.page = target;
                    }
                }
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
                .on_hover_text("This session's notes, beside this page (Ctrl+J)")
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
            });
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
        let mut statuses = vec![match connection {
            Connection::Connecting => Status {
                icon: icons::BUSY,
                word: "Connecting",
                color: c.connection.busy,
            },
            Connection::Connected => Status {
                icon: icons::ONLINE,
                word: "Connected",
                color: c.connection.connected,
            },
            Connection::Failing => Status {
                icon: icons::WARN,
                word: "Session failing",
                color: c.result.warning,
            },
            Connection::Offline => Status {
                icon: icons::OFFLINE,
                word: "Not connected",
                color: c.connection.offline,
            },
        }];
        for (dirty, word) in [
            (self.library.dirty(), "Library draft"),
            (self.encounters.draft_dirty(), "Encounter draft"),
            (self.campaign_content.dirty(), "Campaign draft"),
        ] {
            if dirty {
                statuses.push(Status {
                    icon: icons::DRAFT,
                    word,
                    color: c.document.unsaved,
                });
            }
        }
        if let Some((icon, word, color)) = self.sessions.recording_status() {
            statuses.push(Status { icon, word, color });
        }
        for word in self.sessions.alert_words() {
            statuses.push(Status {
                icon: icons::ERROR,
                word,
                color: c.result.error,
            });
        }
        if self.timeline.unreadable.is_some() {
            statuses.push(Status {
                icon: icons::ERROR,
                word: "Timeline not readable",
                color: c.document.invalid,
            });
        }
        if self.sessions.just_saved() {
            statuses.push(Status {
                icon: icons::OK,
                word: "Recording saved",
                color: c.document.saved,
            });
        }
        if self.sessions.processing() {
            statuses.push(Status {
                icon: icons::AUDIO,
                word: "Processing audio",
                color: c.connection.busy,
            });
        } else if self.sessions.job_warned() {
            statuses.push(Status {
                icon: icons::WARN,
                word: "Processed: check it",
                color: c.result.warning,
            });
        } else if self.sessions.job_done() {
            statuses.push(Status {
                icon: icons::SUCCESS,
                word: "Processing done",
                color: c.result.success,
            });
        }
        // A run files itself under this session, and the run's own page has no room to say so.
        if self.run_fact.is_some() && self.old_session {
            statuses.push(Status {
                icon: icons::WARN,
                word: "Old session",
                color: c.result.warning,
            });
        }
        if self.party.sync_stopped {
            statuses.push(Status {
                icon: icons::SYNC_OFF,
                word: "Party sync stopped",
                color: c.sync.pending,
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
                                .rich(ui, format!("Run · {}", strip.name))
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
                                    icon: icons::OFFLINE,
                                    word: "Sync stopped",
                                    color: c.result.error,
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

    fn no_campaign(&mut self, ui: &mut egui::Ui, title: &str) {
        let mut settings = false;
        design::page_header(ui, self.page.name(), None, |_| {});
        let icon = match self.page {
            Page::Encounters => icons::ENCOUNTERS,
            Page::Party => icons::PARTY,
            _ => icons::CAMPAIGN,
        };
        design::empty_state(
            ui,
            icon,
            title,
            "Add a campaign in Settings. The Library works without one.",
            |ui| {
                settings = Action::primary("Open Settings").show(ui).clicked();
            },
        );
        if settings {
            self.page = Page::Settings;
        }
    }

    fn page(&mut self, ui: &mut egui::Ui) {
        let active = self.campaigns.active_campaign();
        let (_, session) = self.settings.campaign_session();
        match self.page {
            Page::Settings => {
                design::page_header(
                    ui,
                    "Settings",
                    Some("Your D&D Beyond account, your campaign list, and session recording"),
                    |_| {},
                );
                let tab_changed = design::segmented(
                    ui,
                    &mut self.settings_tab,
                    &[
                        (SettingsTab::Account, "Account and campaigns"),
                        (SettingsTab::Recording, "Recording and processing"),
                    ],
                );
                if tab_changed {
                    self.processing.look_again();
                }
                if self.settings_tab == SettingsTab::Recording {
                    egui::ScrollArea::vertical()
                        .id_salt("recording_settings")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.add_space(design::S);
                            ui.set_max_width(design::FORM_WIDTH + design::XXL);
                            self.processing.show(ui);
                        });
                    return;
                }
                let blocked = self.blocked_switch();
                egui::ScrollArea::vertical()
                    .id_salt("settings_page")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add_space(design::S);
                        let account = |ui: &mut egui::Ui, settings: &mut settings::Settings| {
                            settings.show(ui);
                        };
                        let list =
                            |ui: &mut egui::Ui,
                             campaigns: &mut campaigns::Campaigns,
                             dndbeyond: &mut dndbeyond::DndBeyond| {
                                if let Some(reason) = blocked {
                                    design::message(ui, Tone::Info, reason);
                                }
                                design::locked(ui, blocked, |ui| {
                                    campaigns.show(ui, session.clone(), dndbeyond);
                                });
                            };
                        if ui.available_width() >= 720.0 {
                            // A clear gutter, so that the left column's text does not run on.
                            // Not `ui.columns`, whose justified layout stretches the words in a paragraph.
                            let width = (ui.available_width() - design::XL) / 2.0;
                            ui.horizontal_top(|ui| {
                                ui.spacing_mut().item_spacing.x = design::XL;
                                let column = egui::vec2(width, 0.0);
                                let layout = egui::Layout::top_down(egui::Align::Min);
                                ui.allocate_ui_with_layout(column, layout, |ui| {
                                    // Both ways, so the second column starts in one place.
                                    ui.set_width(width);
                                    ui.spacing_mut().item_spacing.x = 8.0;
                                    account(ui, &mut self.settings);
                                });
                                ui.allocate_ui_with_layout(column, layout, |ui| {
                                    ui.set_width(width);
                                    ui.spacing_mut().item_spacing.x = 8.0;
                                    list(ui, &mut self.campaigns, &mut self.dndbeyond);
                                });
                            });
                        } else {
                            account(ui, &mut self.settings);
                            ui.add_space(design::L);
                            list(ui, &mut self.campaigns, &mut self.dndbeyond);
                        }
                    });
            }
            Page::Sessions => {
                if active.is_some() {
                    self.sessions.show(ui, &self.processing, &mut self.timeline);
                } else {
                    self.no_campaign(ui, "No campaign to keep notes for yet");
                }
            }
            Page::Timeline => {
                if let Some(campaign) = &active {
                    let links = self.sessions.links();
                    let current = self.sessions.current();
                    self.timeline_page.open_run = self.run_fact.as_ref().map(|fact| fact.id);
                    self.timeline_page.dated = self.sessions.current_dated();
                    self.timeline_page.show(
                        ui,
                        &mut self.timeline,
                        campaign.id,
                        &campaign.name,
                        &links,
                        current,
                    );
                } else {
                    self.no_campaign(ui, "No campaign, so no timeline yet");
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
                    self.encounters.show(ui, &self.campaigns);
                    if self.encounters.saved {
                        self.encounters.saved = false;
                        if let Some(campaign) = &active {
                            self.campaign_content.refresh_destination(&campaign.path);
                            self.lookup.refresh();
                        }
                    }
                } else {
                    self.no_campaign(ui, "No campaign to prepare yet");
                }
            }
            Page::Party => {
                if active.is_some() {
                    self.party.show(
                        ui,
                        session,
                        active.is_some_and(|c| c.dndbeyond_campaign_id.is_some()),
                    );
                } else {
                    self.no_campaign(ui, "No campaign, so no party yet");
                }
            }
            Page::Campaign => {
                let Some(campaign) = active else {
                    self.no_campaign(ui, "No campaign selected");
                    return;
                };
                let mut open_entries = false;
                design::page_header(ui, "Campaign", Some(&campaign.name), |ui| {
                    open_entries = Action::quiet("Campaign entries")
                        .icon(icons::DOCUMENT)
                        .hint("This campaign's own entries, in the Library")
                        .show(ui)
                        .clicked();
                });
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
                    }
                    CampaignTab::RunSettings => {
                        self.encounters.run_settings(ui, &self.campaigns);
                        if self.encounters.saved {
                            self.encounters.saved = false;
                            self.campaign_content.refresh_destination(&campaign.path);
                        }
                    }
                    CampaignTab::Link => {
                        // The same guard as Settings: a link or folder change would stop
                        // the run's sync, or hide an open draft.
                        let blocked = self.blocked_switch();
                        egui::ScrollArea::vertical()
                            .id_salt("campaign_link")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                if let Some(reason) = blocked {
                                    ui.add_space(design::S);
                                    design::message(ui, Tone::Info, reason);
                                }
                                design::locked(ui, blocked, |ui| {
                                    self.campaigns.show_active(ui, session, &mut self.dndbeyond);
                                    self.campaigns.show_form_if_editing(ui);
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
                if let Some(run) = self.encounters.run_mut() {
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
                    self.page = Page::Encounters;
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

    /// Requests that cross pages: read beside a draft, or open an entry where it lives.
    fn route(&mut self, ctx: &egui::Context) {
        if let Some(id) = self
            .library
            .peek
            .take()
            .or_else(|| self.encounters.peek.take())
        {
            self.notes_give_way();
            self.lookup.open_id(id);
        }
        if let Some(position) = self
            .encounters
            .run_mut()
            .and_then(|run| run.lookup_request.take())
        {
            self.notes_give_way();
            self.lookup.open_position(position);
        }
        if std::mem::take(&mut self.encounters.go_to_run) {
            self.page = Page::Run;
        }
        if std::mem::take(&mut self.encounters.wants_run) {
            self.page = Page::Run;
        }
        if std::mem::take(&mut self.party.wants_settings) {
            self.page = Page::Settings;
        }
        if std::mem::take(&mut self.party.wants_link_page) {
            self.page = Page::Campaign;
            self.campaign_tab = CampaignTab::Link;
        }
        if std::mem::take(&mut self.sessions.wants_sessions_page) {
            self.page = Page::Sessions;
        }
        if std::mem::take(&mut self.sessions.wants_processing_settings) {
            self.page = Page::Settings;
            self.settings_tab = SettingsTab::Recording;
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
            self.copy_dialog = Some(copy_dialog::CopyDialog::new(source, &self.campaigns, ctx));
        }
        if let Some(entry) = self.lookup.open_in_library.take()
            && self.library.reveal(&entry)
        {
            self.page = Page::Library;
        }
    }
}

impl eframe::App for Splat {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if !self.started {
            self.start();
        }
        if ctx.input(|i| i.viewport().close_requested()) {
            // An edit in its pause before saving is saved now, not reported as a loss.
            self.sessions.flush();
            self.timeline.save();
            if (self.library.dirty()
                || self.campaign_content.dirty()
                || self.encounters.unsaved()
                || self.encounters.pending_sync()
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
        if self.confirm_close {
            self.close_prompt(ctx);
        }
        self.apply_focus_step(ctx);
        self.settings.poll(ctx);
        let (epoch, session) = self.settings.campaign_session();
        self.dndbeyond.begin_frame();
        self.dndbeyond
            .poll(ctx, epoch, session.is_some(), &mut self.campaigns);
        self.lookup.poll(ctx, &self.campaigns);
        self.encounters.poll(ctx, &self.campaigns);
        self.lookup.library_draft = self.library.dirty();
        self.processing.poll(ctx);
        self.processing.recording_now = self.sessions.recording();
        let open = self.campaigns.active_campaign();
        self.sessions.poll(ctx, open.as_ref(), &mut self.timeline);
        self.old_session = self.sessions.current_dated().is_some();
        let fact = self.encounters.run().map(encounter_run::Run::timeline_fact);
        if fact != self.run_fact {
            if let Some(fact) = &fact {
                self.timeline.record_run(fact, self.sessions.current());
            }
            self.run_fact = fact;
        }
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
            session.is_some(),
            self.page == Page::Party,
        );
        self.encounters
            .tick_sync(ctx, session, epoch, active.as_ref());
        self.sessions.on_page = self.page == Page::Sessions;
        self.sessions.banner(ctx, &self.processing);
        self.run_strip(ctx);
        // With no panel left, going on typing must reach the note and never change the run.
        if self.notes_gave_way && !self.lookup.open {
            self.notes_gave_way = false;
            self.sessions.notes_come_back();
        }
        if self.lookup.open {
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
        self.route(ctx);
        if let Some(dialog) = &mut self.copy_dialog {
            dialog.draft = self.encounters.draft_name();
            let open = dialog.show(ctx, &self.campaigns);
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
        self.remember();
    }
}
