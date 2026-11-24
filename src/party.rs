//! The Party page and its view state.
use crate::auth::Session;
use crate::design::{self, Action, Status, Type, colors, icons};
use eframe::egui;
use splat_character::character::{self, EntitySnapshot};
use splat_content::content;
use splat_party::party::{
    LiveValues, Party as PartyState, SHEET_PAGES, SHEET_PORTRAIT, SyncStopped, calendar_time,
    record_age, sheet_conditions, sheet_entries, sheet_labels, sheet_page, sheet_row, spell_dc,
    text, val,
};
use std::time::Duration;

#[derive(Default)]
pub struct PartyPage {
    page: usize,
    search: String,
    content_entry: Option<(String, Result<content::Entry, String>)>,
    /// Compared once for each content version, because this page redraws every frame.
    character_review: Option<Result<character::check::Review, String>>,
    pub wants_link_page: bool,
    pub wants_settings: bool,
    pub run_open: bool,
}

impl PartyPage {
    pub fn show(
        &mut self,
        party: &mut PartyState,
        ui: &mut egui::Ui,
        session: Option<Session>,
        linked: bool,
    ) {
        Party { view: self, party }.show(ui, session, linked);
    }
}

impl PartyPage {
    /// The page's footer lines (`page_footer`), for a UI that draws them itself.
    pub fn footer_lines(
        &mut self,
        party: &mut PartyState,
        connected: bool,
        linked: bool,
    ) -> Vec<design::outcome::plain::PlainLine<PartyPress>> {
        let frame = design::outcome::PaneFrame::headless();
        Party { view: self, party }
            .page_footer(&frame, connected, linked)
            .into_plain()
    }

    /// Do a press of the page's footer; a file to show comes back as its URL.
    pub fn footer_press(
        &mut self,
        party: &mut PartyState,
        press: design::outcome::Press<PartyPress>,
    ) -> Option<String> {
        Party { view: self, party }.press(Some(press))
    }
}

/// The Party page's header: its state words and which actions it offers.
pub struct PartyHeader {
    /// `Not connected · Fetched 3 min ago`, with its icon first.
    pub context: String,
    pub icon: &'static str,
    /// Why Refresh party waits.
    pub refresh_reason: Option<String>,
    /// `Connect to refresh` shows.
    pub connect: bool,
    /// `Refresh party` shows.
    pub refresh: bool,
}

impl PartyHeader {
    pub fn of(party: &PartyState, connected: bool, linked: bool) -> Self {
        let reader = &party.reader;
        let ready = party.combat.ready();
        let fetched = reader
            .selected
            .as_ref()
            .and_then(|id| reader.sheets.get(id))
            .filter(|_| !ready)
            .map(|record| format!(" · Fetched {}", record_age(record)))
            .unwrap_or_default();
        let refresh_reason = if !connected {
            Some(format!("Not connected{fetched} · Connect to refresh"))
        } else if !linked {
            Some("Link this campaign to D&D Beyond first.".to_owned())
        } else if reader.loading() {
            Some("Already refreshing.".to_owned())
        } else {
            None
        };
        let checked =
            reader.selected.as_ref().is_some_and(|id| {
                reader.online_ids.contains(id) && !reader.errors.contains_key(id)
            }) && reader.error.is_none();
        let (icon, word) = if !connected {
            (icons::OFFLINE, "Not connected")
        } else if ready {
            (icons::ONLINE, "Connected · Sync reads this sheet")
        } else if !checked {
            (icons::ONLINE, "Connected")
        } else if reader.stream_open {
            (icons::ONLINE, "Live updates")
        } else {
            (icons::ONLINE, "Connected · Checking for updates")
        };
        // Another computer saves the sheets from automatic reads. Say so, because this
        // computer's notes change only through that computer and Obsidian Sync.
        let writer = reader
            .writer_elsewhere
            .as_ref()
            .map(|name| format!(" · Sheets save on {name}"))
            .unwrap_or_default();
        let refresh = linked && !reader.members.is_empty();
        Self {
            context: format!("{icon} {word}{fetched}{writer}"),
            icon,
            refresh_reason,
            connect: refresh && !connected && reader.conflicts.is_empty(),
            refresh,
        }
    }
}

/// `HP here is from tonight’s fights, as of …`, while tonight's fights count.
pub fn tonight_words(party: &PartyState) -> Option<String> {
    let carry = party.tonight.as_ref()?;
    let mut parts = Vec::new();
    if carry.full || !carry.values.is_empty() {
        parts.push(format!(
            "HP here is from tonight’s fights, as of {}.",
            carry.from
        ));
    }
    if !carry.missing.is_empty() {
        parts.push(format!(
            "Could not read the run of {}.",
            carry.missing.join(", ")
        ));
    }
    Some(parts.join(" "))
}

struct Party<'a> {
    view: &'a mut PartyPage,
    party: &'a mut PartyState,
}

impl Party<'_> {
    #[allow(clippy::disallowed_methods)]
    fn show(&mut self, ui: &mut egui::Ui, session: Option<Session>, linked: bool) {
        let header = PartyHeader::of(self.party, session.is_some(), linked);
        let can_refresh = header.refresh_reason.as_deref();
        let context = header.context.clone();
        // The page's header (M0410): the title, its state, then its actions by measure.
        const REFRESH: design::Word = crate::word!("Refresh party");
        const CONNECT: design::Word = crate::word!("Connect to refresh");
        let mut line = design::TitleLine::new("Party")
            .context(Some(&context))
            .page();
        if header.refresh {
            // A signed-out local-edit notice has its own Connect D&D Beyond, so the header then
            // shows only the state: one action, one name.
            if header.connect {
                line = line.action(design::TitleAction::link(CONNECT));
            }
            line = line.action(
                design::TitleAction::quiet(REFRESH)
                    .icon(icons::REFRESH)
                    .blocked(can_refresh),
            );
        }
        let header = line.show(ui);
        let refresh = header.clicked(REFRESH);
        if header.clicked(CONNECT) {
            self.view.wants_settings = true;
        }
        if linked && session.is_some() {
            design::blocked_reason(ui, can_refresh);
        }
        if refresh && let Some(session) = session.clone() {
            self.party.reader.refresh_by_hand(session, ui.ctx());
        }
        // Round 27 FA4, FH1: the page's notices and results that come and go sit in one footer,
        // below the list and the sheet. So a notice that comes, or its close icon, moves no
        // member row and no sheet. Round 28: the pane places it, `S` below the content or pinned.
        let frame = design::outcome::PaneFrame::new(ui);
        let footer = self.page_footer(&frame, session.is_some(), linked);
        // The list and the sheet are columns that each scroll; the footer is pinned below them.
        // An empty state is short content, and the footer follows it.
        let columns = linked && !self.party.reader.members.is_empty();
        let body = if columns {
            design::Body::Fills(design::Fills::because(design::FillsReason::Columns))
        } else {
            design::Body::Content
        };
        let paned = footer.pane(ui, body, |ui| {
            self.page_body(ui, session, linked);
        });
        self.page_pressed(ui.ctx(), paned.pressed);
    }

    /// The page below its header, above its footer: the empty states, or the list and the sheet.
    fn page_body(&mut self, ui: &mut egui::Ui, session: Option<Session>, linked: bool) {
        let mut open_link = false;
        if !linked {
            design::empty_state(
                ui,
                icons::PARTY,
                if session.is_none() {
                    "Connect D&D Beyond to see the party"
                } else {
                    "Link this campaign to D&D Beyond to see its party"
                },
                if session.is_none() {
                    "First connect your D&D Beyond account. Then choose its campaign under Campaign → Link and folder."
                } else {
                    "Choose its D&D Beyond campaign under Campaign → Link and folder."
                },
                |ui| {
                    // Linking needs a session, so that comes first.
                    if session.is_none() {
                        self.view.wants_settings |= if self.view.run_open {
                            Action::secondary("Connect D&D Beyond")
                        } else {
                            Action::primary("Connect D&D Beyond")
                        }
                        .show(ui)
                        .clicked();
                    } else if if self.view.run_open {
                        Action::secondary("Link this campaign")
                    } else {
                        Action::primary("Link this campaign")
                    }
                    .show(ui)
                    .clicked()
                    {
                        open_link = true;
                    }
                },
            );
            self.view.wants_link_page = open_link;
            return;
        }
        ui.ctx().request_repaint_after(Duration::from_secs(1));
        if self.party.reader.members.is_empty() {
            if self.party.reader.loaded {
                design::empty_state(
                    ui,
                    icons::PARTY,
                    "This campaign has no party members",
                    "Add characters in D&D Beyond, then refresh the party.",
                    |ui| {
                        if design::link(ui, "Connect D&D Beyond").clicked() {
                            self.view.wants_settings = true;
                        }
                    },
                );
            } else if !self.party.reader.loading() {
                design::empty_state(
                    ui,
                    icons::PARTY,
                    "No characters yet",
                    "Connect to D&D Beyond in Settings to load this party.",
                    |ui| {
                        if if self.view.run_open {
                            Action::secondary("Connect D&D Beyond")
                        } else {
                            Action::primary("Connect D&D Beyond")
                        }
                        .show(ui)
                        .clicked()
                        {
                            self.view.wants_settings = true;
                        }
                    },
                );
            }
            return;
        }
        if let Some(words) = tonight_words(self.party) {
            design::help(ui, &words);
        }
        // Each row gives its portrait this much, and keeps the same room for its text.
        let portrait_room = ROW_PORTRAIT + 2.0 * design::S;
        let list_width = (ui.available_width() * 0.46 + portrait_room).clamp(384.0, 624.0);
        let mut selected = self.party.reader.selected.clone();
        design::Column::left("party_members", list_width, 384.0..=694.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: 0,
                right: design::M as i8,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                // Round 25 family: the list keeps the scroll bar's lane whether or not the bar
                // shows, so no row or header column moves when the list starts to scroll.
                let rows_width = ui.available_width() - design::lane_width();
                ui.scope(|ui| {
                    ui.set_max_width(rows_width);
                    member_header(ui);
                });
                design::rule(ui);
                design::ScrollColumn::new("party_members_list").show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for m in &self.party.reader.members {
                        let record = self.party.reader.sheets.get(&m.id);
                        let portrait = self.portrait(record);
                        let is_selected = selected.as_ref() == Some(&m.id);
                        let from_fights =
                            record.and_then(|record| self.party.tonight_values(record));
                        if member_row(
                            ui,
                            m,
                            record,
                            portrait.as_deref(),
                            from_fights,
                            is_selected,
                            self.party.reader.conflicts.contains_key(&m.id),
                        )
                        .clicked()
                        {
                            selected = Some(m.id.clone());
                            self.view.search.clear();
                        }
                    }
                });
            });
        if selected != self.party.reader.selected
            && let Some(id) = selected
        {
            self.party.reader.select(id);
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::M as i8,
                right: 0,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| self.sheet_view(ui, session));
    }

    /// The page's one message footer (round 27 FA4, FH1), below the list and the sheet. Its
    /// order: the errors (the party's read, a character's read beside its kept edits), then the
    /// moved-aside sheets' results, the local-edit warnings with their actions, the storage
    /// warnings and the stopped-sync notice. Each keeps its owner and end event: a moved-aside
    /// sheet's result ends at its own close icon, a local-edit warning when its sheet is
    /// replaced, the stopped-sync notice at its close icon. The messages own their words, so the
    /// body can change the page; `page_pressed` does the press after the page is drawn.
    fn page_footer<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        connected: bool,
        linked: bool,
    ) -> design::outcome::Messages<'f, PartyPress> {
        use design::outcome::{Choice, FaultMessage, Lost, MessageChoice, Said};
        let reader = &self.party.reader;
        let mut messages =
            design::outcome::Messages::new(frame, "party_page_footer", design::LIST_MEASURE);
        let show_file = |path: &std::path::Path| {
            Choice::new(
                MessageChoice::Link {
                    word: "Show file",
                    hover: None,
                },
                PartyPress::ShowFile(path.to_path_buf()),
            )
            .hover(path.display().to_string())
        };
        if linked {
            if let Some(error) = &reader.error {
                messages.outcome(FaultMessage::new(&PartyFault::Read(error)));
            }
            for id in reader.conflicts.keys() {
                if reader.selected.as_ref() != Some(id)
                    && let Some(error) = reader.errors.get(id)
                {
                    messages.outcome(FaultMessage::new(&PartyFault::Character {
                        name: reader.member_name(id),
                        error,
                    }));
                }
            }
            // Round 25 FD3: one dismiss form on the page, the close icon at the message end.
            for shown in reader.archived.newest_first(&()) {
                let line = shown.outcome(frame, |archived| {
                    let name = reader.member_name(&archived.owner);
                    Said::new(crate::sentence!(
                        "{}’s sheet now matches D&D Beyond. Your edited version is kept.",
                        name
                    ))
                    .choice(show_file(&archived.path))
                    .choice(Choice::new(
                        MessageChoice::Dismiss(Lost::Nothing),
                        PartyPress::DismissArchived,
                    ))
                    .into()
                });
                messages.kept(line);
            }
            let several = reader.conflicts.len() > 1;
            let waiting = reader.force_waiting();
            // `at` is the warning's place among the conflicts: its id is looked up at the press.
            for (at, (id, path)) in reader.conflicts.iter().enumerate() {
                let name = reader.member_name(id);
                let fault = FaultMessage::new(&PartyFault::LocalEdits { name, connected });
                let fault = if connected {
                    // Safe first: Show file, then Replace, which changes the active sheet (the
                    // part puts `L` before it). Its lock keeps its reason's room while the
                    // forced read waits and after, so nothing in the footer moves.
                    fault.choice(show_file(path)).choice(
                        Choice::new(
                            MessageChoice::Lose("Replace with D&D Beyond sheet"),
                            PartyPress::Replace(several.then_some(at)),
                        )
                        .locked_when(waiting, crate::sentence!("Waiting for D&D Beyond.")),
                    )
                } else {
                    // The warning asks the DM to connect, so the way to connect is first.
                    fault
                        .choice(
                            Choice::new(MessageChoice::Choice(CONNECT), PartyPress::Connect).hover(
                                "Opens Settings to connect your D&D Beyond account".to_owned(),
                            ),
                        )
                        .choice(show_file(path))
                };
                messages.outcome(fault);
            }
            let warnings: std::collections::BTreeSet<_> = reader.warnings.values().collect();
            for warning in warnings {
                messages.outcome(FaultMessage::new(&PartyFault::Storage(warning)));
            }
        }
        if self.party.sync_stopped {
            messages.outcome(SyncStopped::notice().choice(Choice::new(
                MessageChoice::Dismiss(Lost::Nothing),
                PartyPress::DismissStopped,
            )));
        }
        messages
    }

    /// Do what the DM pressed in the page's footer.
    fn page_pressed(
        &mut self,
        ctx: &egui::Context,
        pressed: Option<design::outcome::Press<PartyPress>>,
    ) {
        if let Some(url) = self.press(pressed) {
            ctx.open_url(egui::OpenUrl::new_tab(url));
        }
    }

    /// Do a footer press; a file to show comes back as its URL.
    fn press(&mut self, pressed: Option<design::outcome::Press<PartyPress>>) -> Option<String> {
        let design::outcome::Press { id, ticket } = pressed?;
        match id {
            PartyPress::DismissStopped => self.party.sync_stopped = false,
            // Its own result only: a newer one keeps its line.
            PartyPress::DismissArchived => {
                if let Some(ticket) = ticket {
                    let _ = self.party.reader.archived.use_press_by(ticket);
                }
            }
            PartyPress::Connect => self.view.wants_settings = true,
            PartyPress::Replace(None) => self.party.reader.force_read(),
            PartyPress::Replace(Some(at)) => {
                if let Some(id) = self.party.reader.conflicts.keys().nth(at).cloned() {
                    self.party.reader.force_read_character(&id);
                }
            }
            PartyPress::ShowFile(path) => {
                if let Ok(url) = url::Url::from_file_path(&path) {
                    return Some(url.to_string());
                }
            }
        }
        None
    }

    fn portrait(&self, record: Option<&EntitySnapshot>) -> Option<std::path::PathBuf> {
        crate::party_sync::portrait_path(self.party.reader.local_scope()?, record?)
    }

    /// The Combat sync heading, which says the sync state while it is closed. Returns whether
    /// it is open.
    fn sync_heading(&mut self, ui: &mut egui::Ui) -> bool {
        // Until the first read, `On` would say more than Splat knows: the heading says it waits,
        // with the first-read icon and colour.
        let state: egui::WidgetText = if !self.party.combat.enabled {
            format!("{} Combat sync · Off", icons::SYNC_OFF).into()
        } else if self.party.combat.state() == crate::combat_sync::SyncState::FirstRead {
            let first_read = colors().sync.first_read;
            let font = egui::TextStyle::Button.resolve(ui.style());
            let mut job = egui::text::LayoutJob::default();
            for (text, color) in [
                (format!("{} ", icons::BUSY), first_read),
                ("Combat sync · ".to_owned(), ui.visuals().text_color()),
                ("Waiting for the first sheet read".to_owned(), first_read),
            ] {
                job.append(
                    &text,
                    0.0,
                    egui::TextFormat {
                        font_id: font.clone(),
                        color,
                        ..Default::default()
                    },
                );
            }
            job.into()
        } else {
            format!("{} Combat sync · On", icons::ONLINE).into()
        };
        // One id for every state, so the panel stays open when the DM ticks its box.
        design::Section::new(state)
            .id("combat_sync_header")
            .open(std::mem::take(&mut self.party.wants_combat_sync).then_some(true))
            .show(ui, |_| ())
            .body_returned
            .is_some()
    }

    fn sync_panel(&mut self, ui: &mut egui::Ui) {
        // The sheet's scroll bar floats over its right edge. Keep the panel clear of it.
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width() - design::L);
            self.party.show_combat_sync(ui);
        });
        design::rule(ui);
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces; the selected character’s read error at the top of its sheet is a banner (design::results::STAGED_ALLOWS)"
    )]
    fn sheet_view(&mut self, ui: &mut egui::Ui, session: Option<Session>) {
        let c = colors();
        let Some(id) = &self.party.reader.selected.clone() else {
            ui.label(
                Type::Body
                    .rich(ui, "Select a character to read their sheet.")
                    .color(c.muted),
            );
            return;
        };
        if let Some(error) = self.party.reader.errors.get(id) {
            // The selected character's own read error stays at the top of its sheet (the
            // footer names the others').
            design::outcome::banner(ui, design::outcome::Banner::Fault(&PartyFault::Read(error)));
        }
        if let Some(record) = most_recent_sheet(self.party, id) {
            let portrait = self.portrait(Some(record));
            ui.horizontal(|ui| {
                if let Some(path) = &portrait {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::splat(SHEET_PORTRAIT),
                        egui::Sense::empty(),
                    );
                    crate::art::portrait(ui, rect, Some(path), &record.definition.splat_name, None);
                }
                design::heading(ui, design::Heading::Name(&record.definition.splat_name));
            });
            // Round 27 FI12: the sheet's link follows its kind line, `L` after the words, so it
            // stays with the sheet it opens at any width (it was at the pane's far edge).
            let line = egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true);
            // The link's press target is `XL` high, so the line is too; a `with_layout` here
            // would take the column's whole height.
            ui.allocate_ui_with_layout(egui::vec2(ui.available_width(), design::XL), line, |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(Type::Small.rich(ui, class_line(record)).color(c.muted));
                design::gap(ui, design::Gap::L);
                if design::small_link(ui, "D&D Beyond ↗").clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(format!(
                        "https://www.dndbeyond.com/characters/{id}"
                    )));
                }
            });
        }
        // Round 26: the sync panel's result sits in the sheet's footer, below the sheet, so what
        // is above stays. It ends at its close icon. Round 28: the sheet's pane is its scroll
        // column, one per page, so each page keeps its own scroll.
        let frame = design::outcome::PaneFrame::new(ui);
        let sheet_footer = |page: Option<usize>, combat: &crate::combat_sync::CombatSync| {
            let mut messages = design::outcome::Messages::new(
                &frame,
                ("party_sheet_footer", id, page),
                design::LIST_MEASURE,
            );
            if session.is_some() {
                combat.footer_messages(&frame, &mut messages);
            }
            messages
        };
        let Some(record) = most_recent_sheet(self.party, id) else {
            // The panel scrolls above the footer when it needs more room.
            let paned =
                sheet_footer(None, &self.party.combat).pane(ui, design::Body::Content, |ui| {
                    if session.is_some() && self.sync_heading(ui) {
                        self.sync_panel(ui);
                    }
                    if !self.party.reader.loading() {
                        ui.label(
                            Type::Body
                                .rich(ui, "No local copy is available for this character.")
                                .color(c.muted),
                        );
                    }
                });
            self.party.combat.footer_press(paned.pressed);
            return;
        };
        let before = self.view.page;
        design::overflow_tabs_small(ui, &mut self.view.page, &SHEET_PAGES);
        if before != self.view.page {
            self.view.search.clear();
        }
        design::rule(ui);
        if self.view.page > 0 && sheet_entries(record, self.view.page).len() > 8 {
            let width = 240.0_f32.min(ui.available_width());
            let _ = crate::field_id!("party_page_filter")
                .once(crate::design::Once::PageSearch)
                .search(
                    ui,
                    "Filter this page",
                    &mut self.view.search,
                    &format!("Filter {}…", SHEET_PAGES[self.view.page].1.to_lowercase()),
                    width,
                );
        }
        if self.view.page == 0
            && self
                .view
                .content_entry
                .as_ref()
                .is_none_or(|(hash, _)| hash != &record.source.content_hash)
        {
            let entry = record.entry();
            self.view.character_review = entry
                .as_ref()
                .ok()
                .map(|entry| character::check::review(record, entry));
            self.view.content_entry = Some((record.source.content_hash.clone(), entry));
        }
        // The sheet pane starts below the tabs.
        let paned = sheet_footer(Some(self.view.page), &self.party.combat).pane(
            ui,
            design::Body::Content,
            |ui| {
                // The heading and its body are one block, below the tabs: the tabs never sit
                // between them. Both scroll with the sheet, as one scroll is easier at the table
                // than a scroll inside a scroll.
                if session.is_some() && self.sync_heading(ui) {
                    self.sync_panel(ui);
                }
                let Some(record) = most_recent_sheet(self.party, id) else {
                    return;
                };
                // One person has one HP on this page: the list's, while tonight's fights count.
                let live = self
                    .party
                    .tonight_values(record)
                    .map(|(hp, temporary_hp)| LiveValues {
                        hp: Some(hp),
                        max_hp: record.stat("max_hp"),
                        temporary_hp,
                        death_saves: record.initial.death_saves.clone(),
                        death_saves_shown: false,
                        conditions: sheet_labels(record),
                    });
                sheet_page(ui, record, self.view.page, &self.view.search, live.as_ref());
                if self.view.page == 0 {
                    design::gap(ui, design::Gap::M);
                    let details = design::quiet_section(ui, "Technical details", |ui| {
                        source_and_parity(
                            ui,
                            record,
                            self.view.content_entry.as_ref().map(|(_, entry)| entry),
                            self.view.character_review.as_ref(),
                            self.party.reader.campaign_path(),
                            self.party.reader.local_scope(),
                        );
                    });
                    // Its heading is a row of the sheet: the footer never cuts it through.
                    sheet_row(ui, details.header_response.rect);
                }
            },
        );
        self.party.combat.footer_press(paned.pressed);
    }
}

/// What the DM pressed in the Party page's footer, done after the page is drawn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PartyPress {
    DismissStopped,
    /// The close icon of a moved-aside sheet's result (the press's ticket names which).
    DismissArchived,
    Connect,
    /// The local-edit warning whose Replace the DM pressed: `None` when there is one, which
    /// replaces every one. The warning's place among the conflicts, looked up at the press.
    Replace(Option<usize>),
    ShowFile(std::path::PathBuf),
}

/// What the Party page's footer says is wrong.
enum PartyFault<'a> {
    /// The party's read failed (D&D Beyond's or the system's words).
    Read(&'a str),
    /// One character's read failed, beside its kept edits.
    Character { name: &'a str, error: &'a str },
    /// The character's sheet note has the DM's own edits, so sync stopped writing it.
    LocalEdits { name: &'a str, connected: bool },
    /// The local copy could not be stored (the system's words).
    Storage(&'a str),
}

impl design::outcome::Fault for PartyFault<'_> {
    fn severity(&self) -> design::outcome::Severity {
        match self {
            Self::Read(_) | Self::Character { .. } => design::outcome::Severity::CannotRead,
            Self::LocalEdits { .. } | Self::Storage(_) => design::outcome::Severity::NeedsFixing,
        }
    }

    fn place(&self) -> Option<design::outcome::FaultPlace<'_>> {
        match self {
            Self::Character { name, .. } => Some(design::outcome::FaultPlace {
                name: (*name).into(),
                line: None,
            }),
            _ => None,
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "a failed read or store says D&D Beyond’s or the system’s own words"
    )]
    fn cause(&self) -> design::outcome::Sentence {
        use design::outcome::Sentence;
        match self {
            Self::Read(error) | Self::Character { error, .. } => {
                Sentence::outside(*error).unwrap_or(crate::sentence!("The read did not finish."))
            }
            Self::LocalEdits { name, connected } => {
                let first = crate::sentence!(
                    "{}’s sheet file has your own edits, so Splat stopped updating it.",
                    name
                );
                if *connected {
                    first.then(crate::sentence!("Replacing keeps your version as a copy."))
                } else {
                    first.then(crate::sentence!(
                        "Connect, then replace it. Replacing keeps your version as a copy."
                    ))
                }
            }
            Self::Storage(warning) => Sentence::outside(format!("Local storage: {warning}"))
                .unwrap_or(crate::sentence!("Local storage did not save.")),
        }
    }
}

pub fn most_recent_sheet<'a>(party: &'a PartyState, id: &str) -> Option<&'a EntitySnapshot> {
    match (party.combat.record(), party.reader.sheets.get(id)) {
        (Some(live), Some(saved)) if saved.source.fetched_at > live.source.fetched_at => {
            Some(saved)
        }
        (live, saved) => live.or(saved),
    }
}

/// The way to connect, wherever the party asks the DM to.
const CONNECT: &str = "Connect D&D Beyond";
/// `Fighter · Level 6`: the one form of a member's classes and level, in the sheet's kind line
/// and in the member's row (round 28 FB12).
pub fn classes_and_level(classes: &str, level: Option<impl std::fmt::Display>) -> String {
    [
        classes.to_owned(),
        level.map(|n| format!("Level {n}")).unwrap_or_default(),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" · ")
}

pub fn class_line(record: &EntitySnapshot) -> String {
    format!(
        "{} · {}",
        record.character.species,
        classes_and_level(
            &record
                .character
                .classes
                .iter()
                .map(|c| text(&c["name"]))
                .collect::<Vec<_>>()
                .join(" / "),
            Some(record.character.level),
        )
    )
}

const AC_WIDTH: f32 = 34.0;
const HP_WIDTH: f32 = 112.0;
const MEMBER_ROW: f32 = 54.0;
const ROW_PORTRAIT: f32 = 38.0;
/// What a DM asks of the whole party at once. Columns are for comparing, so they get the room
/// a name can spare, and a narrow list says the same on its second line.
const COMPARE: [(&str, &str); 3] = [("PER", "Per"), ("INS", "Ins"), ("DC", "DC")];
const COMPARE_WIDTH: f32 = 30.0;
const COMPARE_FROM: f32 = 480.0;

/// The right edge of one compare column, in the header and in a row.
fn compare_right(hp_right: f32, at: usize) -> f32 {
    hp_right - HP_WIDTH - AC_WIDTH - (2 - at) as f32 * COMPARE_WIDTH
}

pub fn compare_values(record: Option<&EntitySnapshot>) -> [String; 3] {
    let Some(record) = record else {
        return ["—"; 3].map(str::to_owned);
    };
    let dc = spell_dc(record).map_or_else(|| "—".to_owned(), |dc| dc.to_string());
    [
        val(record, "passive_perception"),
        val(record, "passive_insight"),
        dc,
    ]
}

fn member_header(ui: &mut egui::Ui) {
    let c = colors();
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 18.0), egui::Sense::hover());
    let font = Type::Label.font(ui);
    let hp_right = rect.right() - design::S;
    if rect.width() >= COMPARE_FROM {
        for (at, (word, _)) in COMPARE.iter().enumerate() {
            ui.painter().text(
                egui::pos2(compare_right(hp_right, at), rect.center().y),
                egui::Align2::RIGHT_CENTER,
                *word,
                font.clone(),
                c.muted,
            );
        }
    }
    ui.painter().text(
        egui::pos2(rect.left() + design::S, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "NAME",
        font.clone(),
        c.muted,
    );
    ui.painter().text(
        egui::pos2(hp_right, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        "HP",
        font.clone(),
        c.muted,
    );
    ui.painter().text(
        egui::pos2(hp_right - HP_WIDTH, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        "AC",
        font,
        c.muted,
    );
}

/// One member, to compare at a glance: name, AC, HP, then class and conditions.
fn member_row(
    ui: &mut egui::Ui,
    member: &crate::party_sync::Member,
    record: Option<&EntitySnapshot>,
    portrait: Option<&std::path::Path>,
    from_fights: Option<(i64, i64)>,
    selected: bool,
    local_edits: bool,
) -> egui::Response {
    let c = colors();
    let compare = compare_values(record);
    let level = record.map(|r| r.character.level).or(member.level);
    let classes = record.map_or_else(String::new, |r| {
        r.character
            .classes
            .iter()
            .filter_map(|c| c["name"].as_str())
            .collect::<Vec<_>>()
            .join(" / ")
    });
    let conditions = record.map_or_else(Vec::new, sheet_conditions);
    let mut summary = classes_and_level(&classes, level);
    if record.is_none() {
        summary = "No local copy yet".into();
    }

    let hp = from_fights
        .map(|(hp, _)| hp)
        .or_else(|| record.map(|r| r.initial.hp));
    let max_hp = record.and_then(|r| r.stat("max_hp"));
    let ac = record.and_then(|r| r.stat("armor_class"));
    let temp = from_fights.map_or_else(|| record.map_or(0, |r| r.initial.temporary_hp), |v| v.1);
    let compared = COMPARE
        .iter()
        .zip(&compare)
        .filter(|((_, word), value)| *word != "DC" || value.as_str() != "—")
        .map(|((_, word), value)| format!("{word} {value}"))
        .collect::<Vec<_>>()
        .join(" · ");
    let name = format!(
        "{}{} · {summary} · {compared} · HP {} · AC {}{}",
        member.name,
        if local_edits { " · Local edits" } else { "" },
        crate::combat_sync::fraction(
            hp.map_or_else(|| "?".to_owned(), |n| n.to_string()),
            max_hp.map_or_else(|| "?".to_owned(), |n| n.to_string()),
        ),
        ac.map_or_else(|| "?".to_owned(), |n| n.to_string()),
        conditions
            .iter()
            .map(|c| format!(" · {c}"))
            .collect::<String>(),
    );
    // A narrow list has no room for columns, so the same numbers take a line of their own.
    let wide = ui.available_width() >= COMPARE_FROM;
    let height = if wide { MEMBER_ROW } else { MEMBER_ROW + 16.0 };
    let response = design::row(ui, height, selected, &name, |ui, rect| {
        let painter = ui.painter().clone();
        let hp_right = rect.right() - design::S;
        let line = rect.top() + 18.0;
        let picture = egui::Rect::from_center_size(
            egui::pos2(
                rect.left() + design::S + ROW_PORTRAIT / 2.0,
                rect.center().y,
            ),
            egui::Vec2::splat(ROW_PORTRAIT),
        );
        crate::art::portrait(ui, picture, portrait, &member.name, None);
        let text_left = picture.right() + design::S;
        let columns = if wide { 3.0 * COMPARE_WIDTH } else { 0.0 };
        let room = hp_right - HP_WIDTH - AC_WIDTH - columns - design::S - text_left;
        let title = design::one_line(ui, &member.name, Type::Name.font(ui), c.text, room);
        painter.galley(egui::pos2(text_left, line - 10.0), title, c.text);
        if wide {
            for (at, value) in compare.iter().enumerate() {
                design::number(
                    &painter,
                    compare_right(hp_right, at),
                    line,
                    value,
                    &Type::Numeral.font(ui),
                    c.muted,
                );
            }
        }
        let small = Type::Small.font(ui);
        if !wide {
            painter.text(
                egui::pos2(text_left, rect.top() + MEMBER_ROW - 6.0),
                egui::Align2::LEFT_TOP,
                &compared,
                small.clone(),
                c.muted,
            );
        }
        design::number(
            &painter,
            hp_right - HP_WIDTH,
            line,
            &ac.map_or_else(|| "?".to_owned(), |n| n.to_string()),
            &Type::Numeral.font(ui),
            c.text,
        );
        let left = design::hp_readout(
            ui,
            &painter,
            hp_right,
            line,
            hp,
            max_hp,
            &design::HpFonts::standard(ui),
        );
        if temp > 0 {
            // Round 27 FA8: `S` between temporary HP and HP, so the two numbers read apart, as
            // the run's columns keep them.
            painter.text(
                egui::pos2(left - design::S, line),
                egui::Align2::RIGHT_CENTER,
                design::temp_hp_text(temp),
                Type::Numeral.font(ui),
                c.hp.temp,
            );
        }
        let bar = egui::Rect::from_min_max(
            egui::pos2(hp_right - 86.0, rect.bottom() - 12.0),
            egui::pos2(hp_right, rect.bottom() - 9.0),
        );
        design::hp_bar(&painter, bar, hp, max_hp, temp);
        painter.hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(design::HAIRLINE_WIDTH, c.hairline),
        );
    });
    let rect = response.rect;
    let text_left = rect.left() + design::S * 2.0 + ROW_PORTRAIT;
    let detail_rect = egui::Rect::from_min_size(
        egui::pos2(text_left, rect.top() + MEMBER_ROW - 22.0),
        egui::vec2(
            rect.right() - HP_WIDTH - design::M - text_left,
            Type::Small.line(),
        ),
    );
    let mut detail_ui = ui.new_child(
        egui::UiBuilder::new()
            .accessibility_parent(response.id)
            .max_rect(detail_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    detail_ui.set_clip_rect(detail_ui.clip_rect().intersect(detail_rect));
    detail_ui.spacing_mut().item_spacing.x = 0.0;
    detail_ui.spacing_mut().interact_size.y = 0.0;
    // The row takes every press on it. Selectable words would take a press on the detail line,
    // which at 960 holds the row's centre (round 27 review: Sable's `Local edits` row).
    detail_ui.style_mut().interaction.selectable_labels = false;
    detail_ui.label(Type::Small.rich(&detail_ui, &summary).color(c.muted));
    if local_edits {
        detail_ui.label(
            Type::Small
                .rich(&detail_ui, format!(" · {} Local edits", icons::DRAFT))
                .color(c.document.unsaved),
        );
    }
    match design::hp_state(hp, max_hp) {
        design::HpState::Bloodied => {
            detail_ui.label(Type::Small.rich(&detail_ui, " · ").color(c.muted));
            design::status(
                &mut detail_ui,
                Status {
                    icon: icons::BLOODIED,
                    word: "Bloodied",
                    color: c.hp.bloodied,
                },
            );
        }
        design::HpState::Zero => {
            detail_ui.label(Type::Small.rich(&detail_ui, " · 0 HP").color(c.hp.zero));
        }
        _ => {}
    }
    for condition in &conditions {
        detail_ui.label(
            Type::Small
                .rich(&detail_ui, format!(" · {condition}"))
                .color(c.participant.condition),
        );
    }
    response
}

/// The technical record behind a sheet. It stays collapsed: most DMs never need it.
fn source_and_parity(
    ui: &mut egui::Ui,
    record: &EntitySnapshot,
    content_entry: Option<&Result<content::Entry, String>>,
    character_review: Option<&Result<character::check::Review, String>>,
    campaign_path: Option<&std::path::Path>,
    local_scope: Option<uuid::Uuid>,
) {
    let small = |ui: &mut egui::Ui, text: String| {
        design::help(ui, &text);
    };
    small(ui, "D&D Beyond supplies these totals.".into());
    // The first two source notes repeat the totals sentence and internal schema details.
    // Keep later notices, such as two services returning different names.
    for message in record.coverage.iter().skip(2) {
        small(ui, message.clone());
    }
    if let Some(Ok(entry)) = content_entry {
        campaign_option_status(ui, entry);
        if let Some(review) = character_review {
            character_check(ui, review);
        }
        small(
            ui,
            format!(
                "Campaign content: {} inventory rows, {} action definitions.",
                entry.inventory.len(),
                entry
                    .fields
                    .get("splat_actions")
                    .and_then(serde_yaml_ng::Value::as_mapping)
                    .map_or(0, |v| v.len())
            ),
        );
        if let Some(import) = entry.fields.get("splat_import") {
            let list = |key: &str| {
                import
                    .get(key)
                    .and_then(serde_yaml_ng::Value::as_sequence)
                    .cloned()
                    .unwrap_or_default()
            };
            small(
                ui,
                format!(
                    "{} not read yet · {} not supported in Splat",
                    design::count(list("gaps").len(), "rule", "rules"),
                    list("engine_gaps")
                        .iter()
                        .filter_map(serde_yaml_ng::Value::as_str)
                        .filter(|gap| !content::adapters::is_implemented(gap))
                        .count()
                ),
            );
            for (title, key) in [
                ("Rules still to read", "gaps"),
                ("Required character or encounter inputs", "required_inputs"),
            ] {
                design::quiet_section(ui, title, |ui| {
                    for item in list(key) {
                        if let Some(reason) =
                            item.get("reason").and_then(serde_yaml_ng::Value::as_str)
                        {
                            ui.add(egui::Label::new(reason).wrap());
                        }
                    }
                });
            }
        }
    }
    small(
        ui,
        format!(
            "Fetched {} UTC. Source details {} UTC.",
            calendar_time(record.source.fetched_at),
            calendar_time(record.source.raw_fetched_at),
        ),
    );
    if let Some(modified) = &record.source.source_modified {
        small(ui, format!("Beyond source modification: {modified}"));
    }
    small(
        ui,
        "D&D Beyond can take time to show a change in every service. Splat checks again for delayed changes.".into(),
    );
    design::quiet_section(ui, "File details", |ui| {
        small(
            ui,
            format!(
                "Preserved {} source fields and {} computed fields.",
                record.source.raw.as_object().map_or(0, |o| o.len()),
                record.source.computed.as_object().map_or(0, |o| o.len())
            ),
        );
        small(
            ui,
            format!("Local creature UUID: {}", record.definition.splat_id),
        );
        small(
            ui,
            format!(
                "Source character: {} · Campaign: {}",
                record.source.character_id, record.source.campaign_id
            ),
        );
        if let Some(root) = campaign_path {
            small(
                ui,
                format!("Campaign copy: {}", record.campaign_path(root).display()),
            );
        }
        if let Some(scope) = local_scope
            && let Ok(cache) = crate::hermetic::character_cache()
            && let Ok(path) = cache.path(
                scope,
                &record.source.campaign_id,
                &record.source.character_id,
            )
        {
            small(ui, format!("Local copy: {}", path.display()));
        }
    });
}

fn character_check(ui: &mut egui::Ui, review: &Result<character::check::Review, String>) {
    use character::check::FindingKind;
    design::quiet_section(ui, "Character check", |ui| match review {
        Ok(review) => {
            ui.add(
                egui::Label::new(if review.matched.is_empty() {
                    "Nothing could be compared with the SRD rules.".to_owned()
                } else {
                    format!("Matches the SRD rules: {}.", review.matched.join(", "))
                })
                .wrap(),
            );
            for (kind, title) in [
                (FindingKind::Differs, "Differs from the SRD rules"),
                (FindingKind::MissingChoice, "Missing choices"),
                (FindingKind::InvalidChoice, "Choices the rules do not allow"),
                (FindingKind::NotChecked, "Not compared"),
            ] {
                if review.count(kind) > 0 {
                    ui.label(egui::RichText::new(title).strong());
                    for finding in review.findings.iter().filter(|f| f.kind == kind) {
                        ui.add(egui::Label::new(&finding.message).wrap());
                    }
                }
            }
        }
        Err(message) => {
            ui.add(egui::Label::new(format!("The check could not run: {message}")).wrap());
        }
    });
}

fn campaign_option_status(ui: &mut egui::Ui, entry: &content::Entry) {
    use content::character::{CampaignOptionState, Sheet};
    let Some(value) = entry.fields.get("splat_character") else {
        return;
    };
    let Ok(sheet) = serde_yaml_ng::from_value::<Sheet>(value.clone()) else {
        return;
    };
    design::quiet_section(ui, "Optional character rules", |ui| {
        for (key, name) in [
            ("dark_bargain", "Dark Bargain"),
            ("character_threads", "Character Threads"),
            ("runestones", "Runestones"),
        ] {
            match sheet.campaign_options.get(key) {
                Some(CampaignOptionState::Inactive) => {
                    ui.label(format!("{name}: inactive (not granted on the sheet)."));
                }
                Some(CampaignOptionState::Unselected) => {
                    ui.label(format!("{name}: no effect selected. No effect imported."));
                }
                Some(CampaignOptionState::Unknown) => {
                    ui.label(format!("{name}: source selection data is unavailable."));
                }
                Some(CampaignOptionState::Active { selections }) => {
                    for selected in selections {
                        let status = if selected.rule.is_some() {
                            "converted"
                        } else {
                            "needs rule conversion"
                        };
                        ui.label(format!("{name}: {} ({status}).", selected.name));
                    }
                }
                None => (),
            }
        }
        design::help(
            ui,
            "Selections follow the D&D Beyond sheet. Active choices can need further details. Simulation support is separate.",
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iter_3_b_storage_actions_stay_with_their_message() {
        use crate::design;
        use crate::design::test_harness::ThemedPage as _;
        use eframe::egui;
        use egui_kittest::kittest::Queryable;
        // Two characters with local edits, A and B, in the page's real footer: B's Replace
        // forces a read of B alone, and A stays as it was (the press keeps its notice's place
        // among the conflicts, not a copy of its id).
        let party = std::rc::Rc::new(std::cell::RefCell::new(PartyState::default()));
        for (id, path) in [("a", "/campaign/a.md"), ("b", "/campaign/b.md")] {
            party
                .borrow_mut()
                .reader
                .conflicts
                .insert(id.into(), std::path::Path::new(path).into());
        }
        let drawn = party.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(960.0, 640.0))
            .themed_ui(move |ui| {
                let frame = design::outcome::PaneFrame::new(ui);
                let mut party = drawn.borrow_mut();
                let mut page = PartyPage::default();
                let mut view = Party {
                    party: &mut party,
                    view: &mut page,
                };
                let footer = view.page_footer(&frame, true, true);
                let paned = footer.pane(ui, design::Body::Content, |_| ());
                view.page_pressed(ui.ctx(), paned.pressed);
            });
        harness.run_steps(3);
        const REPLACE: &str = "Replace with D&D Beyond sheet";
        // Every Replace and Show file, top to bottom: the footer's controls.
        let controls = |harness: &egui_kittest::Harness<'_>| {
            let mut rects: Vec<egui::Rect> = harness
                .get_all_by_label(REPLACE)
                .chain(harness.get_all_by_label("Show file"))
                .map(|node| node.rect())
                .collect();
            rects.sort_by(|x, y| {
                x.top()
                    .total_cmp(&y.top())
                    .then(x.left().total_cmp(&y.left()))
            });
            rects
        };
        // A press with the pointer, and the URLs that frame asked to open.
        let click = |harness: &mut egui_kittest::Harness<'_>, at: egui::Pos2| {
            let mut urls = Vec::new();
            harness.event(egui::Event::PointerMoved(at));
            for pressed in [true, false] {
                harness.event(egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                });
                harness.run_steps(1);
                for command in &harness.output().platform_output.commands {
                    if let egui::OutputCommand::OpenUrl(url) = command {
                        urls.push(url.url.clone());
                    }
                }
            }
            harness.run_steps(2);
            urls
        };
        let replaces: Vec<_> = harness
            .get_all_by_label(REPLACE)
            .map(|node| node.rect())
            .collect();
        let files: Vec<_> = harness
            .get_all_by_label("Show file")
            .map(|node| node.rect())
            .collect();
        assert_eq!(replaces.len(), 2, "each local-edit notice has its Replace");
        // Review of PR 330: safe first. Each warning has Show file, then Replace (the
        // destructive choice) on its line, exactly `L` after it.
        for replace in &replaces {
            let file = files
                .iter()
                .find(|file| (file.center().y - replace.center().y).abs() <= design::S)
                .copied();
            assert!(
                file.is_some_and(|file| file.right() <= replace.left()
                    && (replace.left() - file.right() - design::L).abs() <= 0.5),
                "Show file {file:?} comes first, `L` before Replace {replace:?}"
            );
        }
        let before = controls(&harness);
        let second = replaces
            .iter()
            .max_by(|x, y| x.top().total_cmp(&y.top()))
            .copied()
            .unwrap();
        let first = replaces
            .iter()
            .min_by(|x, y| x.top().total_cmp(&y.top()))
            .copied()
            .unwrap();
        click(&mut harness, second.center());
        assert_eq!(
            party.borrow().reader.forced(),
            ["b"],
            "Replace on the second notice forces a read of B only; A is unchanged"
        );
        // While the forced read waits, Replace is locked with its reason, and no control in
        // the footer moves: the lock's line keeps its room before, during and after.
        assert!(party.borrow().reader.force_waiting());
        let during = controls(&harness);
        assert_eq!(
            before, during,
            "the footer’s controls stay while the read waits"
        );
        click(&mut harness, first.center());
        harness
            .get_all_by_label(REPLACE)
            .min_by(|x, y| x.rect().top().total_cmp(&y.rect().top()))
            .unwrap()
            .focus();
        harness.run_steps(1);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(2);
        assert_eq!(
            party.borrow().reader.forced(),
            ["b"],
            "a locked Replace takes no pointer press and no key"
        );
        party.borrow_mut().reader.end_forced_read();
        harness.run_steps(3);
        assert_eq!(before, controls(&harness), "and stay after the read");
        // Wave 2 d (C): two moved-aside sheets, A then B, each a kept result with its own close
        // icon. They are the page's, not the selected sheet's: they stay when the DM selects
        // another character, and each acts on its own file. With B selected, A's Show file
        // opens A's file; B's close icon (the newest, first in the footer) ends B's result only.
        {
            let mut party = party.borrow_mut();
            party.reader.conflicts.clear();
            party.reader.select("a".into());
            for (owner, path) in [("a", "/campaign/a-old.md"), ("b", "/campaign/b-old.md")] {
                party.reader.archived.record(crate::party_sync::Archived {
                    path: std::path::Path::new(path).into(),
                    owner: owner.into(),
                });
            }
            party.reader.select("b".into());
        }
        harness.run_steps(3);
        let files: Vec<_> = harness
            .get_all_by_label("Show file")
            .map(|node| node.rect())
            .collect();
        assert_eq!(files.len(), 2, "both results stay when B is selected");
        let oldest = files
            .iter()
            .max_by(|x, y| x.top().total_cmp(&y.top()))
            .copied()
            .unwrap();
        let urls = click(&mut harness, oldest.center());
        assert!(
            urls.len() == 1 && urls[0].ends_with("/campaign/a-old.md"),
            "A’s Show file opens A’s file only: {urls:?}"
        );
        let closes: Vec<_> = harness
            .get_all_by_label(design::outcome::DISMISS_NAME)
            .map(|node| node.rect())
            .collect();
        assert_eq!(closes.len(), 2, "each moved-aside sheet has its close icon");
        let newest = closes
            .iter()
            .min_by(|x, y| x.top().total_cmp(&y.top()))
            .copied()
            .unwrap();
        click(&mut harness, newest.center());
        let left: Vec<_> = party
            .borrow()
            .reader
            .archived
            .newest_first(&())
            .map(|shown| shown.value().owner.clone())
            .collect();
        assert_eq!(left, ["a"], "B’s close icon ends B’s result; A’s stays");
    }
}
