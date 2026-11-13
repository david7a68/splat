//! Character viewer, with combat sync on from the time Splat connects to D&D Beyond.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use crate::rolls::Chip;
use crate::{
    auth::Session,
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    party_sync::{Reader, Stored},
};
use eframe::egui;
use serde_json::Value;
use splat::character::{ABILITIES, EntitySnapshot};
use splat::content::dice_text::{D20, DamagePart, RollLink};
use std::time::Duration;
#[derive(Default)]
pub struct Party {
    /// HP from the fights of the session being played, when there are any.
    pub tonight: Option<crate::encounter_run::Carry>,
    reader: Reader,
    page: usize,
    search: String,
    combat: crate::combat_sync::CombatSync,
    epoch: u64,
    content_entry: Option<(String, Result<splat::content::Entry, String>)>,
    /// Compared once for each content version, because this page redraws every frame.
    character_review: Option<Result<splat::character::check::Review, String>>,
    connected: bool,
    /// The campaign that `sync_turned_off` is about.
    campaign: Option<uuid::Uuid>,
    /// A read finished with new characters, so their copies are on disk for whoever lists the party.
    pub new_copies: bool,
    /// The DM turned combat sync off here. It stays off until they turn it on.
    sync_turned_off: bool,
    /// Sync was on when the connection ended, so it stopped. The rail says so.
    pub sync_stopped: bool,
    /// Open the combat sync control when the rail links here.
    pub wants_combat_sync: bool,
    /// The DM asked to go where this campaign's D&D Beyond link is set.
    pub wants_link_page: bool,
    pub wants_settings: bool,
    pub run_open: bool,
}
impl Party {
    /// A read of the party is running. The rail shows a spinner beside Connected.
    pub fn busy(&self) -> bool {
        self.reader.loading()
    }

    /// Keeps every sheet current, whichever page is showing. The shell calls this every frame.
    pub fn sync(
        &mut self,
        ctx: &egui::Context,
        epoch: u64,
        campaign: Option<(uuid::Uuid, String, std::path::PathBuf)>,
        session: Option<Session>,
        visible: bool,
    ) {
        self.epoch = epoch;
        self.connected = session.is_some();
        let local = campaign.as_ref().map(|(id, _, _)| *id);
        if local != self.campaign {
            self.campaign = local;
            self.sync_turned_off = false;
        }
        if !self.connected {
            if self.combat.enabled {
                self.sync_stopped = true;
            }
            self.combat.stop();
        }
        let wake = self.connected && campaign.is_some();
        self.reader.sync(ctx, epoch, campaign, self.connected);
        self.reader.on_page = visible;
        self.reader.tick(session.clone(), ctx);
        self.tick_combat(ctx, session, visible);
        if !self.reader.loading() && std::mem::take(&mut self.reader.new_copy) {
            self.new_copies = true;
        }
        if wake {
            ctx.request_repaint_after(Duration::from_secs(1));
        }
    }

    /// Combat sync for the selected character. Away from this page the reader keeps the sheet
    /// current, so this runs only to finish what the DM changed here.
    fn tick_combat(&mut self, ctx: &egui::Context, session: Option<Session>, visible: bool) {
        self.combat.keep_all_unsent();
        let Some((id, campaign)) = self.reader.selected.clone().and_then(|id| {
            let campaign = self.reader.sheets.get(&id)?.source.campaign_id.clone();
            Some((id, campaign))
        }) else {
            self.combat.stop();
            return;
        };
        if session.is_some() && !self.combat.enabled && !self.sync_turned_off {
            self.combat.start();
        }
        if self.combat.enabled {
            self.sync_stopped = false;
        }
        if visible || self.combat.busy() || self.combat.dirty() {
            self.combat.tick(ctx, session, self.epoch, &campaign, &id);
        }
        if self.combat.updated {
            if let Some(record) = self.combat.record().cloned() {
                // Each read gets here on the frame's thread. With the same sheet, only its age
                // changes, and that is not worth a write to disk.
                let same = self
                    .reader
                    .sheets
                    .get(&id)
                    .is_some_and(|old| old.source.content_hash == record.source.content_hash);
                if !same {
                    if let Some(scope) = self.reader.local_scope() {
                        let saved = crate::hermetic::character_cache()
                            .and_then(|cache| cache.save(scope, &record))
                            .map(|_| ());
                        self.reader.stored(Stored::Sheet(id.clone()), saved);
                    }
                    self.reader.save_campaign_copy(&id, &record);
                }
                self.reader.sheets.insert(id, record);
            }
            self.combat.updated = false;
        }
    }

    /// Combat sync on this page found a conflict. The rail says so, wherever the DM is.
    pub fn sync_conflict(&self) -> bool {
        self.combat.enabled && self.combat.state() == crate::combat_sync::SyncState::Conflict
    }

    /// A read of the selected sheet, in D&D Beyond's own fields. Test builds have no route
    /// to a live one, so a visual test shows the sync panel's editable values this way.
    /// For visual tests: swap in a party's members and its read error, and get the old ones back
    /// (an empty party whose read failed, then the party as it was).
    #[cfg(test)]
    pub fn swap_members_and_error(
        &mut self,
        members: Vec<crate::party_sync::Member>,
        error: Option<String>,
    ) -> (Vec<crate::party_sync::Member>, Option<String>) {
        (
            std::mem::replace(&mut self.reader.members, members),
            std::mem::replace(&mut self.reader.error, error),
        )
    }

    #[cfg(test)]
    pub fn simulate_sheet_read(&mut self, max_hp: i64, sheet: &Value) {
        let id = self
            .reader
            .selected
            .as_deref()
            .and_then(|id| id.parse().ok());
        self.combat
            .simulate_sheet(id.expect("a selected character"), max_hp, sheet);
    }

    /// How often the sync panel asked for its stepper wells, and how often it read the values
    /// to measure them.
    #[cfg(test)]
    pub fn sync_well_counts(&self) -> (usize, usize) {
        self.combat.well_counts
    }

    /// A change made on this page has not reached D&D Beyond.
    pub fn unconfirmed(&self) -> bool {
        self.combat.unconfirmed()
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    /// The Party page's copy of a character, for the run's side panel.
    pub fn sheet(&self, id: &str) -> Option<&EntitySnapshot> {
        self.reader.sheets.get(id)
    }

    /// Where tonight's fights left this character, if they did.
    fn tonight_values(&self, record: &EntitySnapshot) -> Option<(i64, i64)> {
        let carry = self.tonight.as_ref()?;
        carry
            .values
            .get(&record.definition.splat_id)
            .copied()
            .or_else(|| {
                carry
                    .full
                    .then(|| (record.stat("max_hp"), 0))
                    .and_then(|(hp, temp)| Some((hp?, temp)))
            })
    }

    /// The members' identities in a run, in the party bar's order.
    pub fn order(&self) -> Vec<uuid::Uuid> {
        self.reader
            .members
            .iter()
            .filter_map(|member| self.reader.sheets.get(&member.id))
            .map(|record| record.definition.splat_id)
            .collect()
    }
    /// The party has members to show.
    pub fn has_members(&self) -> bool {
        !self.reader.members.is_empty()
    }

    /// Each member at a glance. An open fight's values come first, then tonight's fights,
    /// then the sheet.
    pub fn glances(
        &self,
        run: &std::collections::BTreeMap<uuid::Uuid, crate::encounter_run::FightRow>,
    ) -> Vec<crate::party_bar::Glance> {
        self.reader
            .members
            .iter()
            .map(|member| {
                let record = self.reader.sheets.get(&member.id);
                let in_run = record.and_then(|r| run.get(&r.definition.splat_id));
                let (hp, temp) = in_run
                    .map(|row| (row.hp, row.temporary_hp))
                    .or_else(|| {
                        let (hp, temp) = self.tonight_values(record?)?;
                        Some((Some(hp), temp))
                    })
                    .unwrap_or_else(|| {
                        (
                            record.map(|r| r.initial.hp),
                            record.map_or(0, |r| r.initial.temporary_hp),
                        )
                    });
                let stat = |key| record.and_then(|r| r.stat(key));
                crate::party_bar::Glance {
                    id: member.id.clone(),
                    name: member.name.clone(),
                    portrait: self.portrait(record),
                    hp,
                    max_hp: in_run.and_then(|row| row.max_hp).or(stat("max_hp")),
                    temp,
                    ac: stat("armor_class"),
                    perception: stat("passive_perception"),
                    insight: stat("passive_insight"),
                    investigation: stat("passive_investigation"),
                    spell_dc: record.and_then(spell_dc),
                    conditions: in_run.map_or_else(
                        || record.map_or_else(Vec::new, sheet_labels),
                        |row| row.conditions.clone(),
                    ),
                }
            })
            .collect()
    }

    /// Where the bar's numbers come from, as a status.
    pub fn glance_source(&self, from_run: bool) -> crate::party_bar::Source {
        let c = colors();
        if from_run {
            return crate::party_bar::Source {
                icon: icons::RUN,
                word: "From the open fight".to_owned(),
                short: "This fight",
                color: c.muted,
            };
        }
        if let Some(carry) = self
            .tonight
            .as_ref()
            .filter(|carry| carry.full || !carry.values.is_empty())
        {
            return crate::party_bar::Source {
                icon: if carry.values.is_empty() {
                    icons::LONG_REST
                } else {
                    icons::ENCOUNTERS
                },
                word: format!("After {}", carry.from),
                short: if carry.values.is_empty() {
                    "Long rest"
                } else {
                    "Last fight"
                },
                color: c.muted,
            };
        }
        match self
            .reader
            .sheets
            .values()
            .min_by_key(|record| record.source.fetched_at)
        {
            Some(oldest) => {
                let fresh = freshness(oldest, self.connected);
                crate::party_bar::Source {
                    icon: fresh.icon,
                    word: format!("Sheets fetched {}", record_age(oldest)),
                    short: "Sheets",
                    color: fresh.color,
                }
            }
            None => crate::party_bar::Source {
                icon: icons::WARN,
                word: "No sheets yet".to_owned(),
                short: "No sheets",
                color: c.muted,
            },
        }
    }

    /// Open this member's sheet the next time the page shows.
    pub fn choose(&mut self, id: String) {
        self.reader.select(id);
    }

    #[allow(clippy::disallowed_methods)]
    pub fn show(&mut self, ui: &mut egui::Ui, session: Option<Session>, linked: bool) {
        let c = colors();
        let fetched = self
            .reader
            .selected
            .as_ref()
            .and_then(|id| self.reader.sheets.get(id))
            .filter(|_| !self.combat.ready())
            .map(|record| format!(" · Fetched {}", record_age(record)))
            .unwrap_or_default();
        let connect_reason = format!("Not connected{fetched} · Connect to refresh");
        let can_refresh = if session.is_none() {
            Some(connect_reason.as_str())
        } else if !linked {
            Some("Link this campaign to D&D Beyond first.")
        } else if self.reader.loading() {
            Some("Already refreshing.")
        } else {
            None
        };
        let checked = self.reader.selected.as_ref().is_some_and(|id| {
            self.reader.online_ids.contains(id) && !self.reader.errors.contains_key(id)
        }) && self.reader.error.is_none();
        let (status_icon, status_word, _status_color) = if session.is_none() {
            (icons::OFFLINE, "Not connected", c.connection.offline)
        } else if self.combat.ready() {
            (
                icons::ONLINE,
                "Connected · Sync reads this sheet",
                c.connection.connected,
            )
        } else if !checked {
            // Round 24 FK2: `Fetched 3 h ago` after it says the sheet is the one Splat saved.
            (icons::ONLINE, "Connected", c.connection.busy)
        } else if self.reader.stream_open {
            (icons::ONLINE, "Live updates", c.connection.connected)
        } else {
            (
                icons::ONLINE,
                "Connected · Checking for updates",
                c.connection.connected,
            )
        };
        // Another computer saves the sheets from automatic reads. Say so, because this
        // computer's notes change only through that computer and Obsidian Sync.
        let writer = self
            .reader
            .writer_elsewhere
            .as_ref()
            .map(|name| format!(" · Sheets save on {name}"))
            .unwrap_or_default();
        let context = format!("{status_icon} {status_word}{fetched}{writer}");
        let mut refresh = false;
        design::page_header(ui, "Party", Some(&context), |ui| {
            if linked && !self.reader.members.is_empty() {
                refresh = Action::quiet("Refresh party")
                    .icon(icons::REFRESH)
                    .blocked(can_refresh)
                    .show(ui)
                    .clicked();
                // A signed-out local-edit notice has its own Connect D&D Beyond, so the
                // header then shows only the state: one action, one name.
                if session.is_none()
                    && !self.reader.members.is_empty()
                    && self.reader.conflicts.is_empty()
                    && design::link(ui, "Connect to refresh").clicked()
                {
                    self.wants_settings = true;
                }
            }
        });
        if linked && session.is_some() {
            design::blocked_reason(ui, can_refresh);
        }
        if refresh && let Some(session) = session.clone() {
            self.reader.refresh_by_hand(session, ui.ctx());
        }
        // Round 27 FA4, FH1: the page's notices and results that come and go sit in one footer,
        // below the list and the sheet. So a notice that comes, or its close icon, moves no
        // member row and no sheet. Round 28: the pane places it, `S` below the content or pinned.
        let presses = PagePresses::default();
        let footer = self.page_footer(&presses, session.is_some(), linked);
        // The list and the sheet are columns that each scroll; the footer is pinned below them.
        // An empty state is short content, and the footer follows it.
        let columns = linked && !self.reader.members.is_empty();
        let body = if columns {
            design::Body::Fills(design::Fills::because(design::FillsReason::Columns))
        } else {
            design::Body::Content
        };
        footer.pane(ui, body, |ui| {
            self.page_body(ui, session, linked);
        });
        self.page_pressed(presses);
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
                        self.wants_settings |= if self.run_open {
                            Action::secondary("Connect D&D Beyond")
                        } else {
                            Action::primary("Connect D&D Beyond")
                        }
                        .show(ui)
                        .clicked();
                    } else if if self.run_open {
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
            self.wants_link_page = open_link;
            return;
        }
        ui.ctx().request_repaint_after(Duration::from_secs(1));
        if self.reader.members.is_empty() {
            if self.reader.loaded {
                design::empty_state(
                    ui,
                    icons::PARTY,
                    "This campaign has no party members",
                    "Add characters in D&D Beyond, then refresh the party.",
                    |ui| {
                        if design::link(ui, "Connect D&D Beyond").clicked() {
                            self.wants_settings = true;
                        }
                    },
                );
            } else if !self.reader.loading() {
                design::empty_state(
                    ui,
                    icons::PARTY,
                    "No characters yet",
                    "Connect to D&D Beyond in Settings to load this party.",
                    |ui| {
                        if if self.run_open {
                            Action::secondary("Connect D&D Beyond")
                        } else {
                            Action::primary("Connect D&D Beyond")
                        }
                        .show(ui)
                        .clicked()
                        {
                            self.wants_settings = true;
                        }
                    },
                );
            }
            return;
        }
        if let Some(carry) = &self.tonight {
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
            let words = parts.join(" ");
            design::help(ui, &words);
        }
        // Each row gives its portrait this much, and keeps the same room for its text.
        let portrait_room = ROW_PORTRAIT + 2.0 * design::S;
        let list_width = (ui.available_width() * 0.46 + portrait_room).clamp(384.0, 624.0);
        let mut selected = self.reader.selected.clone();
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
                    for m in &self.reader.members {
                        let record = self.reader.sheets.get(&m.id);
                        let portrait = self.portrait(record);
                        let is_selected = selected.as_ref() == Some(&m.id);
                        let from_fights = record.and_then(|record| self.tonight_values(record));
                        if member_row(
                            ui,
                            m,
                            record,
                            portrait.as_deref(),
                            from_fights,
                            is_selected,
                            self.reader.conflicts.contains_key(&m.id),
                        )
                        .clicked()
                        {
                            selected = Some(m.id.clone());
                            self.search.clear();
                        }
                    }
                });
            });
        if selected != self.reader.selected
            && let Some(id) = selected
        {
            self.reader.select(id);
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
    /// replaced-sheet results, the local-edit notices with their actions, the storage warnings
    /// and the stopped-sync notice. Each keeps its owner and end event: a result or the
    /// stopped-sync notice ends at its close icon, a local-edit notice when its sheet is
    /// replaced. The messages own their words, so the body can change the page while the footer
    /// waits for it; a press is kept in `presses` and done by `page_pressed`.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn page_footer<'p>(
        &self,
        presses: &'p PagePresses,
        connected: bool,
        linked: bool,
    ) -> design::Footer<'p> {
        let reader = &self.reader;
        let mut footer = design::Footer::new("party_page_footer", design::LIST_MEASURE);
        if linked {
            if let Some(error) = reader.error.clone() {
                footer.reason(error.clone(), move |ui| {
                    design::message(ui, Tone::Error, &error);
                });
            }
            for id in reader.conflicts.keys() {
                if reader.selected.as_ref() != Some(id)
                    && let Some(error) = reader.errors.get(id)
                {
                    let words = format!("{}: {error}", reader.member_name(id));
                    footer.reason(words.clone(), move |ui| {
                        design::message(ui, Tone::Error, &words);
                    });
                }
            }
            // Round 25 FD3: one dismiss form on the page, the close icon at the message end.
            for path in &reader.archived {
                let name = reader
                    .archived_owners
                    .get(&**path)
                    .map(|id| reader.member_name(id))
                    .unwrap_or("This character");
                let words =
                    format!("{name}’s sheet now matches D&D Beyond. Your edited version is kept.");
                let dismiss = &presses.dismiss_archived;
                let order = reader
                    .archived_orders
                    .get(&**path)
                    .copied()
                    .unwrap_or_default();
                // A shared path: no copy each frame.
                let path = std::sync::Arc::clone(path);
                footer.result(order, words.clone(), false, move |ui| {
                    if dismissable(ui, |ui| {
                        storage_notice(ui, Tone::Success, &words, &["Show file"], |ui| {
                            show_file(ui, &path);
                        });
                    }) {
                        dismiss.set(true);
                    }
                });
            }
            let several = reader.conflicts.len() > 1;
            let waiting = reader.force_waiting();
            // `at` is the notice's place among the conflicts: its id is looked up at the press.
            for (at, (id, path)) in reader.conflicts.iter().enumerate() {
                let name = reader.member_name(id);
                let next = if connected {
                    "Replacing keeps your version as a copy."
                } else {
                    "Connect, then replace it. Replacing keeps your version as a copy."
                };
                let words = format!(
                    "{name}’s sheet file has your own edits, so Splat stopped updating it.\n{next}"
                );
                let action = if !connected {
                    CONNECT
                } else if waiting {
                    "Waiting for D&D Beyond."
                } else {
                    "Replace with D&D Beyond sheet"
                };
                let (settings, replace) = (&presses.wants_settings, &presses.replace);
                let path = std::sync::Arc::clone(path);
                footer.notice(words.clone(), move |ui| {
                    storage_notice(ui, Tone::Warning, &words, &[action, "Show file"], |ui| {
                        if !connected {
                            // The message asks the DM to connect, so the way to connect is on
                            // its line.
                            if Action::secondary(action)
                                .density(Density::Read)
                                .hint("Opens Settings to connect your D&D Beyond account")
                                .show(ui)
                                .clicked()
                            {
                                settings.set(true);
                            }
                        } else if waiting {
                            ui.label(Type::Small.rich(ui, action).color(colors().muted));
                        } else if Action::secondary(action)
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            replace.set(Some(several.then_some(at)));
                        }
                        show_file(ui, &path);
                    });
                });
            }
            let warnings: std::collections::BTreeSet<_> = reader.warnings.values().collect();
            for warning in warnings {
                let words = format!("Local storage: {warning}");
                footer.notice(words.clone(), move |ui| {
                    design::message(ui, Tone::Warning, &words);
                });
            }
        }
        if self.sync_stopped {
            let dismiss = &presses.dismiss_stopped;
            footer.notice(SYNC_STOPPED, move |ui| {
                if dismissable(ui, |ui| {
                    design::message(ui, Tone::Warning, SYNC_STOPPED);
                }) {
                    dismiss.set(true);
                }
            });
        }
        footer
    }

    /// Do what the DM pressed in the page's footer.
    fn page_pressed(&mut self, presses: PagePresses) {
        if presses.dismiss_stopped.get() {
            self.sync_stopped = false;
        }
        if presses.dismiss_archived.get() {
            self.reader.archived.clear();
            self.reader.archived_orders.clear();
            self.reader.archived_owners.clear();
        }
        self.wants_settings |= presses.wants_settings.get();
        match presses.replace.take() {
            Some(None) => self.reader.force_read(),
            Some(Some(at)) => {
                if let Some(id) = self.reader.conflicts.keys().nth(at).cloned() {
                    self.reader.force_read_character(&id);
                }
            }
            None => {}
        }
    }

    fn portrait(&self, record: Option<&EntitySnapshot>) -> Option<std::path::PathBuf> {
        crate::party_sync::portrait_path(self.reader.local_scope()?, record?)
    }

    /// The open combat sync panel, with a rule under it.
    /// The Combat sync heading, which says the sync state while it is closed. Returns whether
    /// it is open.
    fn sync_heading(&mut self, ui: &mut egui::Ui) -> bool {
        // Until the first read, `On` would say more than Splat knows: the heading says it waits,
        // with the first-read icon and colour.
        let state: egui::WidgetText = if !self.combat.enabled {
            format!("{} Combat sync · Off", icons::SYNC_OFF).into()
        } else if self.combat.state() == crate::combat_sync::SyncState::FirstRead {
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
        egui::CollapsingHeader::new(state)
            .id_salt("combat_sync_header")
            .icon(design::section_icon)
            .open(std::mem::take(&mut self.wants_combat_sync).then_some(true))
            .show(ui, |_| ())
            .body_returned
            .is_some()
    }

    fn sync_panel(&mut self, ui: &mut egui::Ui) {
        let was = self.combat.enabled;
        // The sheet's scroll bar floats over its right edge. Keep the panel clear of it.
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width() - design::L);
            self.combat.show(ui);
        });
        if was != self.combat.enabled {
            self.sync_turned_off = !self.combat.enabled;
        }
        design::rule(ui);
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn sheet_view(&mut self, ui: &mut egui::Ui, session: Option<Session>) {
        let c = colors();
        let Some(id) = &self.reader.selected.clone() else {
            ui.label(
                Type::Body
                    .rich(ui, "Select a character to read their sheet.")
                    .color(c.muted),
            );
            return;
        };
        if let Some(error) = self.reader.errors.get(id) {
            design::message(ui, Tone::Error, error);
        }
        if let Some(record) = newer(self.combat.record(), self.reader.sheets.get(id)) {
            let portrait = self.portrait(Some(record));
            ui.horizontal(|ui| {
                if let Some(path) = &portrait {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::splat(SHEET_PORTRAIT),
                        egui::Sense::empty(),
                    );
                    crate::art::portrait(ui, rect, Some(path), &record.definition.splat_name, None);
                }
                ui.label(
                    Type::Display
                        .rich(ui, &record.definition.splat_name)
                        .color(c.text),
                );
            });
            // Round 27 FI12: the sheet's link follows its kind line, `L` after the words, so it
            // stays with the sheet it opens at any width (it was at the pane's far edge).
            let line = egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true);
            // The link's press target is `XL` high, so the line is too; a `with_layout` here
            // would take the column's whole height.
            ui.allocate_ui_with_layout(egui::vec2(ui.available_width(), design::XL), line, |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(Type::Small.rich(ui, class_line(record)).color(c.muted));
                ui.add_space(design::L);
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
        let dismissed = std::cell::Cell::new(false);
        let sheet_footer = |page: Option<usize>, combat: &crate::combat_sync::CombatSync| {
            let mut footer =
                design::Footer::new(("party_sheet_footer", id, page), design::LIST_MEASURE);
            if session.is_some() {
                combat.footer_messages(&mut footer, &dismissed);
            }
            footer
        };
        let Some(record) = newer(self.combat.record(), self.reader.sheets.get(id)) else {
            // The panel scrolls above the footer when it needs more room.
            sheet_footer(None, &self.combat).pane(ui, design::Body::Content, |ui| {
                if session.is_some() && self.sync_heading(ui) {
                    self.sync_panel(ui);
                }
                if !self.reader.loading() {
                    ui.label(
                        Type::Body
                            .rich(ui, "No local copy is available for this character.")
                            .color(c.muted),
                    );
                }
            });
            if dismissed.get() {
                self.combat.end_action_notice();
            }
            return;
        };
        let before = self.page;
        design::overflow_tabs_small(ui, &mut self.page, &SHEET_PAGES);
        if before != self.page {
            self.search.clear();
        }
        design::rule(ui);
        if self.page > 0 && sheet_entries(record, self.page).len() > 8 {
            ui.horizontal(|ui| {
                design::search_field(
                    ui,
                    "Filter this page",
                    &mut self.search,
                    &format!("Filter {}…", SHEET_PAGES[self.page].1.to_lowercase()),
                    240.0_f32.min(ui.available_width() - 70.0),
                );
                if !self.search.is_empty()
                    && Action::quiet("Clear")
                        .density(Density::Scan)
                        .show(ui)
                        .clicked()
                {
                    self.search.clear();
                }
            });
        }
        if self.page == 0
            && self
                .content_entry
                .as_ref()
                .is_none_or(|(hash, _)| hash != &record.source.content_hash)
        {
            let entry = record.entry();
            self.character_review = entry
                .as_ref()
                .ok()
                .map(|entry| splat::character::check::review(record, entry));
            self.content_entry = Some((record.source.content_hash.clone(), entry));
        }
        // The sheet pane starts below the tabs.
        sheet_footer(Some(self.page), &self.combat).pane(ui, design::Body::Content, |ui| {
            {
                // The heading and its body are one block, below the tabs: the tabs never sit
                // between them. Both scroll with the sheet, as one scroll is easier at the table
                // than a scroll inside a scroll.
                if session.is_some() && self.sync_heading(ui) {
                    self.sync_panel(ui);
                }
                let Some(record) = newer(self.combat.record(), self.reader.sheets.get(id)) else {
                    return;
                };
                // One person has one HP on this page: the list's, while tonight's fights count.
                let live = self
                    .tonight_values(record)
                    .map(|(hp, temporary_hp)| LiveValues {
                        hp: Some(hp),
                        max_hp: record.stat("max_hp"),
                        temporary_hp,
                        death_saves: record.initial.death_saves.clone(),
                        death_saves_shown: false,
                        conditions: sheet_labels(record),
                    });
                sheet_page(ui, record, self.page, &self.search, live.as_ref());
                if self.page == 0 {
                    ui.add_space(design::M);
                    design::quiet_section(ui, "Technical details", |ui| {
                        source_and_parity(
                            ui,
                            record,
                            self.content_entry.as_ref().map(|(_, entry)| entry),
                            self.character_review.as_ref(),
                            self.reader.campaign_path(),
                            self.reader.local_scope(),
                        );
                    });
                }
            }
        });
        if dismissed.get() {
            self.combat.end_action_notice();
        }
    }
}

/// What the DM pressed in the Party page's footer, done after the page is drawn.
#[derive(Default)]
struct PagePresses {
    dismiss_stopped: std::cell::Cell<bool>,
    dismiss_archived: std::cell::Cell<bool>,
    wants_settings: std::cell::Cell<bool>,
    /// The local-edit notice whose Replace the DM pressed: `None` inside replaces every one.
    /// The notice's place among the conflicts, so no id is copied each frame.
    replace: std::cell::Cell<Option<Option<usize>>>,
}

/// Keep each storage message and its actions together at the list measure.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn storage_notice(
    ui: &mut egui::Ui,
    tone: Tone,
    words: &str,
    actions: &[&str],
    add: impl FnOnce(&mut egui::Ui),
) {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
        let actions_width = actions
            .iter()
            // Measure each action as a bordered button: never narrower than it draws.
            .map(|word| {
                Action::secondary(word)
                    .density(Density::Read)
                    .natural_width(ui)
            })
            .sum::<f32>()
            + design::L * actions.len().saturating_sub(1) as f32;
        let icon = match tone {
            Tone::Warning => icons::WARN,
            Tone::Success => icons::SUCCESS,
            Tone::Error => icons::ERROR,
            Tone::Info => icons::INFO,
        };
        // `design::message` draws S and the bar, the icon, one L gap, then the text, less XS.
        let inset = design::S
            + design::HAIRLINE_WIDTH
            + design::number_width(ui, icon, &Type::Body.font(ui))
            + design::L
            - design::XS;
        let text_width = words
            .lines()
            .map(|line| design::number_width(ui, line, &Type::Body.font(ui)))
            .fold(0.0_f32, f32::max)
            + inset;
        if text_width + design::L + actions_width <= ui.available_width() {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = design::L;
                ui.allocate_ui_with_layout(
                    egui::vec2(text_width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
                        design::message(ui, tone, words);
                    },
                );
                add(ui);
            });
        } else {
            let left = ui.cursor().left();
            let text = design::message(ui, tone, words);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = design::L;
                ui.add_space((text.rect.left() - left).max(0.0));
                add(ui);
            });
        }
    });
}

/// A notice that the DM can dismiss: its words and actions, then the close icon at the right end
/// of its first line. Every dismissable Party notice uses this one form. Returns whether the DM
/// dismissed it.
pub(crate) fn dismissable(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) -> bool {
    let mut dismiss = false;
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
        ui.horizontal_top(|ui| {
            let room = ui.available_width() - Density::Read.control() - design::S;
            ui.allocate_ui_with_layout(
                egui::vec2(room, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                add,
            );
            dismiss = design::icon_button(ui, icons::CLOSE, "Dismiss this message").clicked();
        });
    });
    dismiss
}

fn show_file(ui: &mut egui::Ui, path: &std::path::Path) {
    if design::link(ui, "Show file")
        .on_hover_text(path.display().to_string())
        .clicked()
        && let Ok(url) = reqwest::Url::from_file_path(path)
    {
        ui.ctx().open_url(egui::OpenUrl::new_tab(url.to_string()));
    }
}

fn newer<'a>(
    a: Option<&'a EntitySnapshot>,
    b: Option<&'a EntitySnapshot>,
) -> Option<&'a EntitySnapshot> {
    match (a, b) {
        (Some(a), Some(b)) if b.source.fetched_at > a.source.fetched_at => Some(b),
        _ => a.or(b),
    }
}

/// What the row and side panel know now. A run's values outrank a stored sheet's.
pub struct LiveValues {
    pub hp: Option<i64>,
    pub max_hp: Option<i64>,
    pub temporary_hp: i64,
    pub conditions: Vec<crate::encounter_run::Condition>,
    pub death_saves: splat::character::DeathSaves,
    /// The run already drew its editable block above this page.
    pub death_saves_shown: bool,
}

/// One entry on a sheet page: its name and its facts.
const ENTRY_ROW: f32 = 40.0;
/// The way to connect, wherever the party asks the DM to.
const CONNECT: &str = "Connect D&D Beyond";
/// Sync stops only when the connection ends, and starts again when it comes back.
const SYNC_STOPPED: &str = "Combat sync stopped when the connection to D&D Beyond ended. Changes not yet sent did not reach D&D Beyond. Connect in Settings, and sync starts again.";

/// What a sheet says a character has left, for a DM who plans an encounter around it.
#[derive(Default)]
pub struct SheetNow {
    hp: i64,
    max_hp: i64,
    temporary_hp: i64,
    armor_class: Option<i64>,
    conditions: Vec<String>,
    /// By level. Ordinary and pact slots each have a line.
    slots: Vec<SlotsLeft>,
}
struct SlotsLeft {
    level: i64,
    left: i64,
    max: i64,
}
impl SheetNow {
    pub fn of(record: &EntitySnapshot) -> Self {
        let mut slots: Vec<_> = record
            .definition
            .splat_resources
            .iter()
            .filter(|(_, pool)| pool.kind.as_deref() == Some("spell_slot"))
            .map(|(key, pool)| SlotsLeft {
                level: pool.level.unwrap_or(0),
                left: *record.initial.resources.get(key).unwrap_or(&pool.max),
                max: pool.max,
            })
            .collect();
        slots.sort_unstable_by_key(|slot| (slot.level, slot.left, slot.max));
        Self {
            hp: record.initial.hp,
            max_hp: record.stat("max_hp").unwrap_or(record.initial.hp),
            temporary_hp: record.initial.temporary_hp,
            armor_class: record.stat("armor_class"),
            conditions: sheet_conditions(record),
            slots,
        }
    }
    /// One row of the roster. A word goes with the number, so that it is not only a number.
    pub fn line(&self) -> String {
        let mut parts = vec![format!("HP {}/{}", self.hp, self.max_hp)];
        match design::hp_state(Some(self.hp), Some(self.max_hp)) {
            design::HpState::Zero => parts.push("0 HP".to_owned()),
            design::HpState::Bloodied => parts.push("Bloodied".to_owned()),
            _ => {}
        }
        if self.temporary_hp > 0 {
            parts.push(format!("Temp {}", self.temporary_hp));
        }
        if let Some(ac) = self.armor_class {
            parts.push(format!("AC {ac}"));
        }
        parts.extend(self.conditions.iter().cloned());
        if !self.slots.is_empty() {
            let slots: Vec<_> = self
                .slots
                .iter()
                .map(|slot| format!("L{} {}/{}", slot.level, slot.left, slot.max))
                .collect();
            parts.push(format!("Slots {}", slots.join(", ")));
        }
        parts.join(" · ")
    }
}

/// The party together, as the roster has it. With no sheet to go on, there is no line.
pub fn party_line(sheets: &[&EntitySnapshot], connected: bool) -> Option<String> {
    let oldest = sheets
        .iter()
        .min_by_key(|record| record.source.fetched_at)?;
    let now: Vec<SheetNow> = sheets.iter().map(|record| SheetNow::of(record)).collect();
    let mut parts = vec![format!(
        "Party now: HP {}/{}",
        now.iter().map(|sheet| sheet.hp).sum::<i64>(),
        now.iter().map(|sheet| sheet.max_hp).sum::<i64>(),
    )];
    let slots = now.iter().flat_map(|sheet| &sheet.slots);
    let (left, max) = slots.fold((0, 0), |(left, max), slot| {
        (left + slot.left, max + slot.max)
    });
    if max > 0 {
        parts.push(format!("Slots {left}/{max}"));
    }
    parts.push(freshness(oldest, connected).word);
    Some(parts.join(" · "))
}

/// How old a sheet is, as a status with words.
pub struct Freshness {
    pub word: String,
    pub icon: &'static str,
    pub color: egui::Color32,
}

pub fn freshness(record: &EntitySnapshot, connected: bool) -> Freshness {
    let age = record_age(record);
    let c = colors().connection;
    if connected {
        Freshness {
            word: format!("Connected · Fetched {age}"),
            icon: icons::ONLINE,
            color: c.connected,
        }
    } else {
        Freshness {
            word: format!("Not connected · Fetched {age}"),
            icon: icons::OFFLINE,
            color: c.offline,
        }
    }
}

fn record_age(record: &EntitySnapshot) -> String {
    fetch_age(crate::hermetic::now().saturating_sub(record.source.fetched_at))
}

/// `Fighter · Level 6`: the one form of a member's classes and level, in the sheet's kind line
/// and in the member's row (round 28 FB12).
fn classes_and_level(classes: &str, level: Option<impl std::fmt::Display>) -> String {
    [
        classes.to_owned(),
        level.map(|n| format!("Level {n}")).unwrap_or_default(),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" · ")
}

fn class_line(record: &EntitySnapshot) -> String {
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
pub const SHEET_PORTRAIT: f32 = 56.0;

/// What a DM asks of the whole party at once. Columns are for comparing, so they get the room
/// a name can spare, and a narrow list says the same on its second line.
const COMPARE: [(&str, &str); 3] = [("PER", "Per"), ("INS", "Ins"), ("DC", "DC")];
const COMPARE_WIDTH: f32 = 30.0;
const COMPARE_FROM: f32 = 480.0;

/// The right edge of one compare column, in the header and in a row.
fn compare_right(hp_right: f32, at: usize) -> f32 {
    hp_right - HP_WIDTH - AC_WIDTH - (2 - at) as f32 * COMPARE_WIDTH
}

/// The conditions a sheet names, as a run's entries: labels, with no count.
fn sheet_labels(record: &EntitySnapshot) -> Vec<crate::encounter_run::Condition> {
    sheet_conditions(record)
        .into_iter()
        .map(crate::encounter_run::Condition::Label)
        .collect()
}

/// The conditions a sheet names.
fn sheet_conditions(record: &EntitySnapshot) -> Vec<String> {
    record
        .initial
        .conditions
        .iter()
        .filter_map(|condition| condition["name"].as_str().map(str::to_owned))
        .collect()
}

/// The highest spell save DC on a sheet, for a caster.
fn spell_dc(record: &EntitySnapshot) -> Option<i64> {
    array(&record.character.casting["saveDcs"])
        .iter()
        .filter_map(|value| value["value"].as_i64())
        .max()
}

fn compare_values(record: Option<&EntitySnapshot>) -> [String; 3] {
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
        "{}{} · {summary} · {compared} · HP {}/{} · AC {}{}",
        member.name,
        if local_edits { " · Local edits" } else { "" },
        hp.map_or_else(|| "?".to_owned(), |n| n.to_string()),
        max_hp.map_or_else(|| "?".to_owned(), |n| n.to_string()),
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
    content_entry: Option<&Result<splat::content::Entry, String>>,
    character_review: Option<&Result<splat::character::check::Review, String>>,
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
                        .filter(|gap| !splat::content::adapters::is_implemented(gap))
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
            crate::encounter_run::calendar_time(record.source.fetched_at),
            crate::encounter_run::calendar_time(record.source.raw_fetched_at),
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

pub(crate) fn has_spells(record: &EntitySnapshot) -> bool {
    let d = &record.source.raw;
    d["spells"]
        .as_object()
        .into_iter()
        .flat_map(|o| o.values())
        .any(|v| !array(v).is_empty())
        || array(&d["classSpells"])
            .iter()
            .any(|class| !array(&class["spells"]).is_empty())
}

/// One page of a sheet. The Party page and the run's side panel both use this.
pub fn sheet_page(
    ui: &mut egui::Ui,
    record: &EntitySnapshot,
    page: usize,
    search: &str,
    live: Option<&LiveValues>,
) {
    crate::rolls::set_entry(ui.ctx(), &record.definition.splat_name);
    match page {
        0 => overview(ui, record, live),
        BACKGROUND_PAGE => background(ui, record),
        _ => details(ui, record, page, search),
    }
}

/// The sheet page with background, details, characteristics, notes and companions.
pub const BACKGROUND_PAGE: usize = 5;
/// The sheet's pages, for Party and the run's side panel. Each index is its place in this list.
pub const SHEET_PAGES: [(usize, &str); 6] = [
    (0, "Overview"),
    (1, "Actions"),
    (2, "Spells"),
    (3, "Equipment"),
    (4, "Features"),
    (BACKGROUND_PAGE, "Background"),
];

/// Labelled prose, one paragraph at a time, at a reading width.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn prose(ui: &mut egui::Ui, label: Option<&str>, text: &str) {
    design::reader::measure(ui, |ui| {
        if let Some(label) = label {
            ui.add_space(design::S);
            ui.label(Type::Small.rich(ui, label).color(colors().muted));
        }
        for paragraph in text.split("\n\n") {
            ui.add(
                egui::Label::new(Type::ProseCompact.rich(ui, paragraph).color(colors().text))
                    .wrap(),
            );
            ui.add_space(design::XS);
        }
    });
}

/// Who the character is, as the player wrote it on D&D Beyond.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn background(ui: &mut egui::Ui, record: &EntitySnapshot) {
    use splat::character::profile::{characteristic_rows, companion_line, detail_rows, story_rows};
    let profile = record.profile();
    let details = detail_rows(&profile.details);
    let characteristics = characteristic_rows(&profile.characteristics);
    let story = story_rows(&profile.story);
    if profile.background.is_none()
        && details.is_empty()
        && characteristics.is_empty()
        && story.is_empty()
        && profile.companions.is_empty()
    {
        section_heading(ui, "Background");
        ui.weak("This sheet has no background, details or notes.");
        return;
    }
    if let Some(background) = &profile.background {
        section_heading(ui, "Background");
        ui.label(Type::Strong.rich(
            ui,
            if background.custom {
                format!("{} (custom)", background.name)
            } else {
                background.name.clone()
            },
        ));
        if let Some((_, feature)) = &profile.background_feature {
            ui.label(format!("Feature: {}", feature.name));
        }
        if !background.feats.is_empty() {
            ui.add(egui::Label::new(format!("Feats: {}", background.feats.join(", "))).wrap());
        }
        if !background.description.is_empty() || profile.background_feature.is_some() {
            design::quiet_section(ui, "Background text", |ui| {
                if !background.description.is_empty() {
                    prose(ui, None, &background.description);
                }
                if let Some((_, feature)) = &profile.background_feature {
                    prose(ui, Some(&feature.name), &feature.description);
                }
            });
        }
    }
    if !details.is_empty() {
        section_heading(ui, "Details");
        sheet_columns(ui, details.len(), |ui, at| {
            let (label, value) = &details[at];
            ui.add(egui::Label::new(format!("{label}: {value}")).wrap());
        });
    }
    for (title, rows) in [("Characteristics", characteristics), ("Notes", story)] {
        if rows.is_empty() {
            continue;
        }
        section_heading(ui, title);
        for (label, text) in rows {
            prose(ui, Some(label), text);
        }
    }
    if !profile.companions.is_empty() {
        sheet_heading(ui, "Companions", profile.companions.len());
        design::help(ui, "Stat blocks are not imported. HP is the printed value.");
        for companion in profile.companions.values() {
            ui.add_space(design::S);
            ui.label(Type::Strong.rich(ui, &companion.name));
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, companion_line(companion))
                        .color(colors().muted),
                )
                .wrap(),
            );
            if !companion.description.is_empty() {
                prose(ui, None, &companion.description);
            }
        }
    }
}

/// Compare the sheet with the SRD rules. D&D Beyond stays the source of truth.
fn character_check(ui: &mut egui::Ui, review: &Result<splat::character::check::Review, String>) {
    use splat::character::check::FindingKind;
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

fn campaign_option_status(ui: &mut egui::Ui, entry: &splat::content::Entry) {
    use splat::content::character::{CampaignOptionState, Sheet};
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
pub(crate) fn fetch_age(seconds: u64) -> String {
    match seconds {
        0..60 => "just now".into(),
        60..3600 => format!("{} min ago", seconds / 60),
        3600..86400 => format!("{} h ago", seconds / 3600),
        86400..172800 => "1 day ago".into(),
        _ => format!("{} days ago", seconds / 86400),
    }
}
fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("—")
}
fn array(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn section_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(design::L);
    design::group_heading(ui, title, 0);
}
const TILE_WIDTH: f32 = 112.0;

fn tile_columns(room: f32, gap: f32) -> usize {
    [6, 3]
        .into_iter()
        .find(|n| *n as f32 * (TILE_WIDTH + gap) <= room)
        .unwrap_or(2)
}

fn sheet_column_widths(room: f32, gap: f32) -> Vec<f32> {
    let spans: &[usize] = match tile_columns(room, gap) {
        6 => &[2, 2, 2],
        3 => &[2, 1],
        _ => &[1, 1],
    };
    let mut widths: Vec<_> = spans
        .iter()
        .map(|span| *span as f32 * (TILE_WIDTH + gap) - gap)
        .collect();
    let last = widths.len() - 1;
    widths[last] = (room - widths[..last].iter().sum::<f32>() - gap * last as f32).max(0.0);
    widths
}

fn sheet_columns(ui: &mut egui::Ui, count: usize, mut row: impl FnMut(&mut egui::Ui, usize)) {
    let widths = sheet_column_widths(ui.available_width(), ui.spacing().item_spacing.x);
    let rows = count.div_ceil(widths.len());
    for line in 0..rows {
        ui.horizontal_top(|ui| {
            for (column, width) in widths.iter().copied().enumerate() {
                ui.allocate_ui_with_layout(
                    egui::vec2(width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_width(width);
                        let at = column * rows + line;
                        if at < count {
                            row(ui, at);
                        }
                    },
                );
            }
        });
    }
}

/// A row of stat tiles that breaks evenly: six across, or three, or two. A lone tile on a
/// second line reads as a different kind of value.
fn tiles(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    let each = TILE_WIDTH + ui.spacing().item_spacing.x;
    let room = ui.available_width();
    let across = tile_columns(room, ui.spacing().item_spacing.x) as f32;
    ui.allocate_ui_with_layout(
        egui::vec2(across * each, 0.0),
        egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(true),
        add,
    );
}

/// One label line and one value line, so tiles in a row keep their values level.
fn metric(ui: &mut egui::Ui, label: &str, value: impl ToString) {
    tile(ui, |ui| design::stat_tile(ui, label, &value.to_string()));
}
/// A tile whose number rolls.
fn roll_metric(ui: &mut egui::Ui, label: &str, value: &str, link: &RollLink) {
    tile(ui, |ui| {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.add(egui::Label::new(Type::Small.rich(ui, label).color(colors().muted)).truncate());
            crate::rolls::link(ui, value, Type::NumeralLarge, link);
        });
    });
}
/// A tile's room, the same for every tile in a row.
fn tile(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    ui.allocate_ui(egui::vec2(TILE_WIDTH, 44.0), |ui| {
        ui.set_min_height(design::XXL + design::M);
        ui.set_width(TILE_WIDTH);
        add(ui);
    });
}

/// A bonus in the sheet's numeral type that rolls, or a dash when the sheet has none.
fn numeral_link(ui: &mut egui::Ui, bonus: Option<i64>, link: impl FnOnce(i64) -> RollLink) {
    match bonus {
        Some(bonus) => {
            crate::rolls::link(ui, &design::signed(bonus), Type::Numeral, &link(bonus));
        }
        None => {
            ui.label(Type::Numeral.rich(ui, "—").color(colors().text));
        }
    }
}

/// A d20 roll for an ability, a save or a skill, named in book case.
fn d20(what: D20, bonus: i64, name: &str) -> RollLink {
    RollLink::d20(what, bonus, design::reader::human_label(name))
}

/// `DEX` or `dexterity` as the book names it.
fn ability_name(short: &str) -> Option<&'static str> {
    splat::content::dice_text::ABILITIES
        .into_iter()
        .find(|name| short.len() >= 3 && name.to_lowercase().starts_with(&short.to_lowercase()))
}

fn val(record: &EntitySnapshot, key: &str) -> String {
    record
        .stat(key)
        .map(|n| n.to_string())
        .unwrap_or("—".into())
}
fn signed(record: &EntitySnapshot, key: &str) -> String {
    record.stat(key).map(design::signed).unwrap_or("—".into())
}
fn skill_order(record: &EntitySnapshot) -> Vec<usize> {
    let mut order: Vec<_> = (0..record.character.skills.len()).collect();
    order.sort_by(|&a, &b| {
        record.character.skills[a]
            .name
            .cmp(&record.character.skills[b].name)
    });
    order
}
fn overview_death_saves(record: &EntitySnapshot, live: Option<&LiveValues>) -> Option<String> {
    if live.is_some_and(|values| values.death_saves_shown) {
        return None;
    }
    let saves = live.map_or(&record.initial.death_saves, |values| &values.death_saves);
    Some(format!(
        "{} {} · {} {}",
        saves.successes,
        if saves.successes == 1 {
            "success"
        } else {
            "successes"
        },
        saves.failures,
        if saves.failures == 1 {
            "failure"
        } else {
            "failures"
        }
    ))
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub(crate) fn overview(ui: &mut egui::Ui, record: &EntitySnapshot, live: Option<&LiveValues>) {
    let narrow = ui.available_width() < 560.0;
    section_heading(ui, "Combat");
    tiles(ui, |ui| {
        let (hp, max, temp) = live.map_or_else(
            || {
                (
                    record.initial.hp.to_string(),
                    val(record, "max_hp"),
                    record.initial.temporary_hp,
                )
            },
            |live| {
                let known =
                    |value: Option<i64>| value.map_or_else(|| "?".to_owned(), |n| n.to_string());
                (known(live.hp), known(live.max_hp), live.temporary_hp)
            },
        );
        metric(ui, "Hit points", format!("{hp} / {max}"));
        metric(ui, "Armor class", val(record, "armor_class"));
        metric(ui, "Temp HP", temp);
        metric(ui, "Proficiency", signed(record, "proficiency_bonus"));
        metric(ui, "Initiative", signed(record, "initiative"));
        metric(
            ui,
            "Inspiration",
            if record.initial.inspiration {
                "Yes"
            } else {
                "No"
            },
        );
    });
    section_heading(ui, "Ability scores");
    tiles(ui, |ui| {
        for ability in ABILITIES {
            let label = format!(
                "{}{} {}",
                ability[..1].to_uppercase(),
                &ability[1..3],
                val(record, ability)
            );
            let key = format!("{ability}_modifier");
            let value = signed(record, &key);
            match record.stat(&key) {
                Some(bonus) => roll_metric(ui, &label, &value, &d20(D20::Check, bonus, ability)),
                None => metric(ui, &label, value),
            }
        }
    });
    section_heading(ui, "Passive scores & movement");
    tiles(ui, |ui| {
        for (label, key) in [
            ("Perception", "passive_perception"),
            ("Insight", "passive_insight"),
            ("Investigation", "passive_investigation"),
        ] {
            metric(ui, label, val(record, key));
        }
        for (key, stat) in &record.definition.splat_stats {
            if let Some(mode) = key.strip_prefix("speed_") {
                metric(
                    ui,
                    &design::reader::human_label(mode),
                    format!("{} ft", stat.value),
                );
            }
        }
    });
    section_heading(ui, "Saving throws");
    sheet_columns(ui, ABILITIES.len(), |ui, at| {
        let ability = ABILITIES[at];
        ui.horizontal(|ui| {
            numeral_link(ui, record.stat(&format!("{ability}_save")), |bonus| {
                d20(D20::Save, bonus, ability)
            });
            let mut name = ability.to_string();
            name[..1].make_ascii_uppercase();
            ui.label(name);
        });
    });
    section_heading(ui, "Skills");
    let skills = skill_order(record);
    sheet_columns(ui, skills.len(), |ui, at| {
        let skill = &record.character.skills[skills[at]];
        ui.horizontal(|ui| {
            numeral_link(ui, record.stat(&skill.stat), |bonus| {
                d20(D20::Skill, bonus, &skill.name)
            });
            let mut marks = Vec::new();
            if skill.expertise {
                marks.push("Expertise");
            } else if skill.proficient {
                marks.push("Proficient");
            } else if skill.half_proficient {
                marks.push("Half");
            }
            if skill.advantage {
                marks.push("Advantage");
            }
            if skill.disadvantage {
                marks.push("Disadvantage");
            }
            let mut job = egui::text::LayoutJob::default();
            job.append(
                &skill.name,
                0.0,
                egui::TextFormat {
                    font_id: Type::Body.font(ui),
                    color: colors().text,
                    ..Default::default()
                },
            );
            if !marks.is_empty() {
                job.append(
                    &format!(" {}", marks.join(" · ")),
                    0.0,
                    egui::TextFormat {
                        font_id: Type::Small.font(ui),
                        color: colors().muted,
                        ..Default::default()
                    },
                );
            }
            ui.add(egui::Label::new(job).wrap());
        });
    });
    section_heading(ui, "Resources");
    if record.definition.splat_resources.is_empty() {
        ui.weak("No limited-use pools returned.");
    }
    for (key, pool) in &record.definition.splat_resources {
        ui.label(format!(
            "{} · {} / {} remaining",
            pool.name,
            record
                .initial
                .resources
                .get(key)
                .map(|v| v.to_string())
                .unwrap_or("—".into()),
            pool.max
        ));
    }
    ui.add_space(design::S);
    let proficiencies = |ui: &mut egui::Ui| {
        section_heading(ui, "Proficiencies & languages");
        for group in &record.character.proficiencies {
            let values = text(&group["values"]);
            if !values.is_empty() {
                ui.add(egui::Label::new(format!("{}: {values}", text(&group["group"]))).wrap());
            }
        }
    };
    let defenses = |ui: &mut egui::Ui| {
        section_heading(ui, "Defenses & senses");
        for (label, values) in [
            ("Resistance", &record.character.resistances),
            ("Immunity", &record.character.immunities),
            ("Vulnerability", &record.character.vulnerabilities),
        ] {
            for value in values {
                ui.label(format!("{label}: {}", text(&value["name"])));
            }
        }
        for sense in &record.character.senses {
            ui.label(format!(
                "{} · {}",
                text(&sense["name"]),
                text(&sense["distance"])
            ));
        }
        if let Some(words) = overview_death_saves(record, live) {
            section_heading(ui, "Death saves");
            ui.label(words);
        }
        section_heading(ui, "Conditions");
        let names: Vec<String> = live.map_or_else(
            || {
                record
                    .initial
                    .conditions
                    .iter()
                    .map(|condition| {
                        condition["name"]
                            .as_str()
                            .map(str::to_owned)
                            .unwrap_or_else(|| condition.to_string())
                    })
                    .collect()
            },
            |live| {
                live.conditions
                    .iter()
                    .map(crate::encounter_run::Condition::words)
                    .collect()
            },
        );
        if names.is_empty() {
            ui.label("None");
        }
        for name in names {
            ui.label(name);
        }
    };
    if narrow {
        proficiencies(ui);
        defenses(ui);
    } else {
        sheet_columns(ui, 2, |ui, column| {
            if column == 0 {
                proficiencies(ui);
            } else {
                defenses(ui);
            }
        });
    }
}
type SheetEntry = (String, String, String, Option<u64>);

fn sheet_entries(record: &EntitySnapshot, page: usize) -> Vec<SheetEntry> {
    let d = &record.source.raw;
    let mut entries: Vec<(String, String, String, Option<u64>)> = Vec::new();
    let mut add = |v: &Value, source: &str| {
        let def = if v["definition"].is_object() {
            &v["definition"]
        } else {
            v
        };
        let Some(name) = def["name"].as_str() else {
            return;
        };
        let desc = def["description"]
            .as_str()
            .or_else(|| def["snippet"].as_str())
            .unwrap_or("No description supplied.");
        let source = match source {
            "race" => "Species",
            "class" => "Class",
            "feat" => "Feat",
            other => other,
        };
        let meta = match page {
            3 => format!(
                "Qty {} · {}{}",
                v["quantity"].as_u64().unwrap_or(1),
                if v["equipped"] == true {
                    "Equipped"
                } else {
                    "Carried"
                },
                if v["isAttuned"] == true {
                    " · Attuned"
                } else {
                    ""
                }
            ),
            2 => {
                let mut words = Vec::new();
                if def["level"].as_u64().is_some_and(|level| level > 0)
                    && (v["prepared"] == true || v["alwaysPrepared"] == true)
                {
                    words.push("Prepared");
                }
                words.join(" · ")
            }
            _ => source.into(),
        };
        let mut extra = Vec::new();
        if let Some(dice) = def["damage"]["diceString"].as_str() {
            extra.push(format!("Printed damage: {dice}"));
        }
        if let Some(ac) = def["armorClass"].as_u64() {
            extra.push(format!("Armor value: {ac}"));
        }
        if page == 2 {
            if let Some(range) = def["range"]["rangeValue"].as_u64() {
                extra.push(if range == 0 {
                    format!(
                        "Range: {}",
                        def["range"]["origin"].as_str().unwrap_or("see description")
                    )
                } else {
                    format!("Range: {range} ft")
                });
            }
            if def["concentration"] == true {
                extra.push("Concentration".into());
            }
            if def["ritual"] == true {
                extra.push("Ritual".into());
            }
            if let Some(time) = def["activation"]["activationTime"].as_u64() {
                let unit = match def["activation"]["activationType"].as_u64() {
                    Some(1) => "action",
                    Some(3) => "bonus action",
                    Some(4) => "reaction",
                    Some(6) => "minute",
                    Some(7) => "hour",
                    _ => "(see description)",
                };
                extra.push(format!("Casting time: {time} {unit}"));
            }
        }
        let meta = if page == 2 && def["level"].as_u64() == Some(0) {
            String::new()
        } else if extra.is_empty() {
            meta
        } else {
            [meta, extra.join(" · ")]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" · ")
        };
        entries.push((
            name.into(),
            meta,
            splat::character::description_text(desc),
            def["level"].as_u64(),
        ));
    };
    match page {
        BACKGROUND_PAGE => {}
        1 => {
            for (source, values) in d["actions"].as_object().into_iter().flat_map(|o| o.iter()) {
                for v in array(values) {
                    add(v, source);
                }
            }
            for v in array(&d["customActions"]) {
                add(v, "Custom action");
            }
        }
        2 => {
            for (source, values) in d["spells"].as_object().into_iter().flat_map(|o| o.iter()) {
                for v in array(values) {
                    add(v, source);
                }
            }
            for class in array(&d["classSpells"]) {
                for v in array(&class["spells"]) {
                    add(v, "Class spell");
                }
            }
        }
        3 => {
            for v in array(&d["inventory"]) {
                add(v, "");
            }
            for v in array(&d["customItems"]) {
                add(v, "Custom item");
            }
        }
        _ => {
            for v in array(&d["race"]["racialTraits"]) {
                add(v, "Species");
            }
            for class in array(&d["classes"]) {
                for v in array(&class["classFeatures"]) {
                    if v["definition"]["requiredLevel"].as_u64().unwrap_or(0)
                        <= class["level"].as_u64().unwrap_or(0)
                    {
                        add(v, text(&class["definition"]["name"]));
                    }
                }
            }
            for v in array(&d["feats"]) {
                add(v, "Feat");
            }
        }
    }
    if page == 4
        && let (_, Some((_, feature))) = splat::character::profile::background(d)
    {
        entries.push((feature.name, "Background".into(), feature.description, None));
    }
    entries.sort_by(|a, b| {
        if page == 2 {
            a.3.cmp(&b.3).then_with(|| a.0.cmp(&b.0))
        } else {
            a.0.cmp(&b.0)
        }
    });
    if page != 3 && page != 1 {
        entries.dedup();
    }
    entries
}

/// An attack's damage as the sheet prints it: `1d4+3`, or a fixed number.
fn attack_damage(attack: &Value) -> String {
    let value = &attack["damage"]["value"];
    if let Some(n) = value.as_i64() {
        return n.to_string();
    }
    format!(
        "{}{}",
        text(&value["diceString"]),
        value["fixedValue"]
            .as_i64()
            .filter(|n| *n != 0)
            .map(|n| format!("{n:+}"))
            .unwrap_or_default()
    )
}

/// One attack as an action: its bonus with its damage, or its save DC, or its damage alone.
fn attack_chip(attack: &Value) -> Option<Chip> {
    let name = text(&attack["name"]).to_owned();
    let kind = text(&attack["damage"]["type"]);
    let mut words = format!("{name} · {} {kind}", attack_damage(attack));
    // Where it lands, as a stat block says it, so the tile shows it as other tiles do.
    if let Some(lands) = attack_reach(attack) {
        words.push_str(&format!(". {lands}."));
    }
    let parts: Vec<DamagePart> = attack["damage"]["value"]["diceString"]
        .as_str()
        .filter(|_| attack["damage"]["value"].as_i64().is_none())
        .map(|_| DamagePart {
            dice: splat::content::dice_text::plain_dice(&attack_damage(attack)),
            kind: (!kind.is_empty()).then(|| design::reader::human_label(kind)),
            when: None,
        })
        .into_iter()
        .collect();
    let link = if let Some(bonus) = attack["toHit"].as_i64() {
        RollLink::D20 {
            what: D20::Attack,
            bonus,
            label: name,
            damage: parts,
        }
    } else if let (Some(dc), Some(ability)) = (
        attack["saveInfo"]["value"].as_i64(),
        ability_name(text(&attack["saveInfo"]["label"])),
    ) {
        RollLink::SaveDc {
            label: name,
            ability: ability.to_owned(),
            dc,
            half: false,
            damage: parts,
        }
    } else if !parts.is_empty() {
        RollLink::Damage { label: name, parts }
    } else {
        return None;
    };
    Chip::new(link, words)
}

/// Where a sheet's attack lands, in a stat block's words: `Reach 5 ft` or `Range 80/320 ft`.
/// `None` when the sheet gives no distance.
fn attack_reach(attack: &Value) -> Option<String> {
    let range = &attack["range"];
    let feet = range["value"].as_u64().filter(|feet| *feet > 0)?;
    let reach = range["label"].as_str() == Some("Reach")
        || range["origin"]
            .as_str()
            .is_some_and(|origin| origin == "Melee");
    Some(
        match range["longRangeValue"].as_u64().filter(|long| *long > feet) {
            Some(long) if !reach => format!("Range {feet}/{long} ft"),
            _ if reach => format!("Reach {feet} ft"),
            _ => format!("Range {feet} ft"),
        },
    )
}

/// A character's actions, from its sheet's attacks.
pub(crate) fn sheet_chips(record: &EntitySnapshot) -> Vec<Chip> {
    record
        .character
        .attacks
        .iter()
        .filter_map(attack_chip)
        .collect()
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn detail_intro(ui: &mut egui::Ui, record: &EntitySnapshot, page: usize, search: &str) {
    match page {
        1 => {
            sheet_heading(ui, "Attacks", record.character.attacks.len());
            // Each attack is an action tile: in a run it arms, elsewhere it previews.
            // A tile keeps its number in the whole list while a search hides others.
            let chips: Vec<(Option<usize>, Chip)> = sheet_chips(record)
                .into_iter()
                .enumerate()
                .filter(|(_, chip)| chip.label.to_lowercase().contains(&search.to_lowercase()))
                .map(|(place, chip)| (Some(place), chip))
                .collect();
            crate::rolls::tiles(ui, &record.definition.splat_name, &chips);
            ui.add_space(design::S);
        }
        2 => {
            section_heading(ui, "Spellcasting");
            // Round 25 AI2: help keeps the reader measure; the spell list below keeps its width.
            design::help(
                ui,
                "Spell text shows the printed numbers. Splat does not work out upcasting or this character’s own bonuses.",
            );
            for (label, key) in [
                ("Spell save DC", "saveDcs"),
                ("Spell attack", "spellAttacks"),
                ("Casting modifier", "modifiers"),
            ] {
                for value in array(&record.character.casting[key]) {
                    ui.label(format!(
                        "{label}: {} · {}",
                        if key == "spellAttacks" {
                            value["value"]
                                .as_i64()
                                .map_or_else(|| value["value"].to_string(), design::signed)
                        } else {
                            value["value"].to_string()
                        },
                        array(&value["sources"])
                            .iter()
                            .filter_map(Value::as_str)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
            for (key, pool) in &record.definition.splat_resources {
                if pool.kind.as_deref() != Some("spell_slot") {
                    continue;
                }
                ui.label(format!(
                    "{} · {} / {} remaining",
                    pool.name,
                    record
                        .initial
                        .resources
                        .get(key)
                        .map(|v| v.to_string())
                        .unwrap_or("—".into()),
                    pool.max
                ));
            }
        }
        _ => {}
    }
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn sheet_heading(ui: &mut egui::Ui, title: &str, count: usize) {
    ui.add_space(design::L);
    design::group_heading(ui, title, count);
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub(crate) fn details(ui: &mut egui::Ui, record: &EntitySnapshot, page: usize, search: &str) {
    detail_intro(ui, record, page, search);
    let entries = sheet_entries(record, page);
    let search = search.to_lowercase();
    let filtered: Vec<_> = entries
        .iter()
        .filter(|(name, source, _, _)| format!("{name} {source}").to_lowercase().contains(&search))
        .collect();
    if page != 2 {
        sheet_heading(
            ui,
            match page {
                1 => "Action descriptions",
                2 => "Spells",
                3 => "Equipment & coins",
                _ => "Features & traits",
            },
            filtered.len(),
        );
    }
    if page == 3 {
        use splat::character::profile;
        ui.label(format!(
            "Coins · {}",
            profile::coins(&record.source.raw)
                .map_or_else(|| "None".into(), |c| profile::coin_line(&c))
        ));
    }
    let mut level_counts = std::collections::BTreeMap::new();
    for row in &filtered {
        *level_counts.entry(row.3).or_insert(0) += 1;
    }
    if filtered.is_empty() {
        ui.weak(if search.is_empty() {
            "No entries returned for this character."
        } else {
            "No entries match this filter."
        });
    }
    let mut last_level = None;
    for (i, (name, meta, description, level)) in filtered.into_iter().enumerate() {
        if page == 2 && (i == 0 || last_level != *level) {
            sheet_heading(
                ui,
                &match level {
                    Some(0) => "Cantrips".to_owned(),
                    Some(n) => format!("Level {n} spells"),
                    None => "Other spells".to_owned(),
                },
                level_counts.get(level).copied().unwrap_or(0),
            );
            last_level = *level;
        }
        // One fixed row each, so a page of spells can be scanned. Its body opens below.
        let id = ui.make_persistent_id((page, i, name));
        let mut state =
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);
        let caret = if state.is_open() {
            icons::DOWN
        } else {
            icons::NEXT
        };
        let row = design::row(
            ui,
            if meta.is_empty() {
                Density::Read.row()
            } else {
                ENTRY_ROW
            },
            state.is_open(),
            &format!("{name} · {meta}"),
            |ui, rect| {
                let top = if meta.is_empty() {
                    rect.center().y - Type::Body.line() / 2.0
                } else {
                    rect.top()
                };
                ui.painter().text(
                    egui::pos2(rect.left() + design::S, top + Type::Body.line() / 2.0),
                    egui::Align2::LEFT_CENTER,
                    caret,
                    Type::Small.font(ui),
                    colors().muted,
                );
                let left = rect.left() + design::XL;
                let width = rect.right() - left - design::S;
                let title = design::one_line(ui, name, Type::Body.font(ui), colors().text, width);
                let detail =
                    design::one_line(ui, meta, Type::Small.font(ui), colors().muted, width);
                ui.painter()
                    .galley(egui::pos2(left, top), title, colors().text);
                ui.painter().galley(
                    egui::pos2(left, rect.top() + Type::Body.line()),
                    detail,
                    colors().muted,
                );
            },
        );
        ui.ctx()
            .accesskit_node_builder(row.id, |node| node.set_expanded(state.is_open()));
        if row.clicked() {
            state.toggle(ui);
        }
        state.show_body_unindented(ui, |ui| {
            ui.add_space(design::XS);
            prose(ui, None, description);
        });
    }
}

#[cfg(test)]
mod sheet_now_tests {
    use super::{EntitySnapshot, SheetNow, SlotsLeft, party_line};
    use serde_json::json;
    #[test]
    fn iter7_b_overflow_tabs_have_no_button_border_and_keep_selected_underline() {
        use crate::design::{self, colors};
        use eframe::egui;
        use egui_kittest::kittest::Queryable;
        for selected in [0, 5] {
            let mut page = selected;
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(350.0, 640.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        design::overflow_tabs(ui, &mut page, &super::SHEET_PAGES)
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let menu = harness.get_by_label("More sheet pages").rect();
            assert!(!harness.output().shapes.iter().any(|shape| matches!(&shape.shape,
                egui::Shape::Rect(rect) if rect.rect.contains(menu.center()) && rect.rect.width() <= menu.width() + 1.0 && rect.stroke.width > 0.0)),
                "iter7-B-FD7: overflow tabs must have no button border");
            if selected == 5 {
                assert!(harness.output().shapes.iter().any(|shape| matches!(&shape.shape,
                    egui::Shape::LineSegment { points, stroke } if stroke.color == colors().accent && (points[0].y - menu.bottom()).abs() < 1.0)),
                    "iter7-B-FD7: the selected overflow page keeps its underline");
            }
        }
    }

    #[test]
    fn iter_3_b_storage_actions_stay_with_their_message() {
        use crate::design::{self, Action, Density, Tone};
        use eframe::egui;
        use egui_kittest::kittest::Queryable;
        for width in [440.0, 960.0, 1920.0] {
            let words = "Sable’s sheet file has your own edits, so Splat stopped updating it.\nReplacing keeps your version as a copy.";
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(width, 640.0))
                .build(|ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        super::storage_notice(
                            ui,
                            Tone::Warning,
                            words,
                            &["Replace with D&D Beyond sheet", "Show file"],
                            |ui| {
                                Action::secondary("Replace with D&D Beyond sheet")
                                    .density(Density::Read)
                                    .show(ui);
                                design::link(ui, "Show file");
                            },
                        );
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let message = harness.get_by_label(words).rect();
            let action = harness.get_by_label("Replace with D&D Beyond sheet").rect();
            assert!(action.right() <= design::LIST_MEASURE + design::L);
            if action.top() >= message.bottom() {
                assert!(
                    (action.left() - message.left()).abs() <= design::XS,
                    "{width}: {message:?}, {action:?}"
                );
            } else {
                assert!(action.left() >= message.right());
                assert!(action.left() - message.right() <= design::L + design::XS);
            }
        }
        // Two characters with local edits, A and B, in the page's real footer: B's Replace
        // forces a read of B alone, and A stays as it was (the press keeps its notice's place
        // among the conflicts, not a copy of its id).
        let party = std::rc::Rc::new(std::cell::RefCell::new(super::Party::default()));
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
            .build_ui(move |ui| {
                let presses = super::PagePresses::default();
                let footer = drawn.borrow().page_footer(&presses, true, true);
                footer.pane(ui, design::Body::Content, |_| ());
                drawn.borrow_mut().page_pressed(presses);
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(3);
        let replaces: Vec<_> = harness
            .get_all_by_label("Replace with D&D Beyond sheet")
            .map(|node| node.rect())
            .collect();
        assert_eq!(replaces.len(), 2, "each local-edit notice has its Replace");
        let second = replaces
            .iter()
            .max_by(|x, y| x.top().total_cmp(&y.top()))
            .copied()
            .unwrap();
        harness.event(egui::Event::PointerMoved(second.center()));
        for pressed in [true, false] {
            harness.event(egui::Event::PointerButton {
                pos: second.center(),
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            harness.run_steps(1);
        }
        harness.run_steps(2);
        assert_eq!(
            party.borrow().reader.forced(),
            ["b"],
            "Replace on the second notice forces a read of B only; A is unchanged"
        );
    }

    #[test]
    fn overview_death_saves_covers_live_stored_hidden_and_all_counts() {
        for successes in 0..=2 {
            for failures in 0..=2 {
                let expected = format!(
                    "{successes} {} · {failures} {}",
                    if successes == 1 {
                        "success"
                    } else {
                        "successes"
                    },
                    if failures == 1 { "failure" } else { "failures" }
                );
                let mut stored = wizard(0);
                stored.initial.death_saves = splat::character::DeathSaves {
                    successes,
                    failures,
                };
                assert_eq!(
                    super::overview_death_saves(&stored, None),
                    Some(expected.clone())
                );
                let mut live = super::LiveValues {
                    hp: Some(1),
                    max_hp: Some(25),
                    temporary_hp: 0,
                    conditions: vec![],
                    death_saves: stored.initial.death_saves.clone(),
                    death_saves_shown: false,
                };
                stored.initial.death_saves.successes = 3;
                stored.initial.death_saves.failures = 3;
                assert_eq!(
                    super::overview_death_saves(&stored, Some(&live)),
                    Some(expected)
                );
                live.hp = Some(0);
                live.death_saves_shown = true;
                assert_eq!(super::overview_death_saves(&stored, Some(&live)), None);
            }
        }
    }

    #[test]
    fn negative_sheet_modifier_uses_true_minus() {
        let mut record = wizard(0);
        record
            .definition
            .splat_stats
            .get_mut("charisma_modifier")
            .unwrap()
            .value = -1;
        assert_eq!(super::signed(&record, "charisma_modifier"), "−1");
    }

    #[test]
    fn overview_skills_are_alphabetical_before_filling_columns() {
        let record = wizard(0);
        let order = super::skill_order(&record);
        let names: Vec<_> = order
            .iter()
            .map(|&at| record.character.skills[at].name.as_str())
            .collect();
        assert_eq!(&names[..3], ["Acrobatics", "Animal Handling", "Arcana"]);
        assert_eq!(names.last(), Some(&"Survival"));
        assert!(names.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn last_sheet_column_uses_all_remaining_room() {
        let room = 420.0;
        let gap = crate::design::S;
        let widths = super::sheet_column_widths(room, gap);
        assert_eq!(widths.len(), 2);
        assert_eq!(widths[1], room - widths[0] - gap);
        assert!(widths[1] > super::TILE_WIDTH);
    }

    #[test]
    fn sheet_columns_start_on_tile_edges_at_every_width() {
        for (room, spans) in [
            (280.0, vec![1, 1]),
            (420.0, vec![2, 1]),
            (800.0, vec![2, 2, 2]),
        ] {
            let gap = crate::design::S;
            let widths = super::sheet_column_widths(room, gap);
            assert_eq!(widths.len(), spans.len());
            let mut left = 0.0;
            let mut tiles = 0;
            for (width, span) in widths.into_iter().zip(spans) {
                assert_eq!(left, tiles as f32 * (super::TILE_WIDTH + gap));
                left += width + gap;
                tiles += span;
            }
        }
    }

    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn wrapped_sheet_cells_keep_the_next_pair_on_one_row() {
        let ctx = eframe::egui::Context::default();
        let mut cells = [eframe::egui::Rect::NOTHING; 4];
        for _ in 0..3 {
            let _ = ctx.run(Default::default(), |ctx| {
                eframe::egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(280.0);
                    super::sheet_columns(ui, 4, |ui, at| {
                        cells[at] = ui
                            .allocate_space(eframe::egui::vec2(
                                super::TILE_WIDTH,
                                crate::design::L * if at == 0 { 3.0 } else { 1.0 },
                            ))
                            .1;
                    });
                });
            });
        }
        assert_eq!(cells[1].top(), cells[3].top());
        assert!(cells[1].top() >= cells[0].bottom());
        assert!(cells[1].top() >= cells[2].bottom());
    }
    fn wizard(fetched: u64) -> EntitySnapshot {
        let abilities: Vec<_> = ["str", "dex", "con", "int", "wis", "cha"]
            .iter()
            .map(|name| json!({"name": name, "score": 14, "modifier": 2, "save": 4}))
            .collect();
        let skills: Vec<_> = [
            "Athletics", "Acrobatics", "Sleight of Hand", "Stealth", "Arcana", "History",
            "Investigation", "Nature", "Religion", "Animal Handling", "Insight", "Medicine",
            "Perception", "Survival", "Deception", "Intimidation", "Performance", "Persuasion",
        ]
        .iter()
        .map(|name| json!({"name": name, "modifier": 5, "isProficient": true, "isExpert": false, "isHalfProficient": false, "isAdvantage": false, "isDisadvantage": false}))
        .collect();
        EntitySnapshot::from_beyond(
            json!({"id": 123, "name": "Example", "campaign": {"id": 456}, "race": {"sizeId": 3, "creatureTypeId": 11}, "classes": [{"id": 8, "level": 1, "hitDiceUsed": 0, "definition": {"name": "Wizard", "spellRules": {"levelSpellSlots": [[], [2]]}}}], "spellSlots": [{"level": 1, "used": 1}], "notes": {"otherNotes": ""}}),
            json!({"characterId": 123, "name": "Example", "campaign": {"id": 456}, "level": 1, "armorClass": 19, "proficiencyBonus": 2, "initiativeBonus": 7, "passivePerception": 15, "passiveInsight": 12, "passiveInvestigation": 13, "abilities": abilities, "skills": skills, "hitPointInfo": {"maximum": 25, "current": 17, "temp": 4}, "deathSaveInfo": {"successCount": 0, "failCount": 0}, "speeds": [{"name": "Walking", "distance": 40}], "race": {"name": "Goblin"}, "classes": [], "attacks": [], "castingInfo": {"saveDcs": []}, "conditions": [{"name": "Poisoned"}], "inspiration": false}),
            "456",
            "123",
            fetched,
            fetched,
        )
        .unwrap()
    }
    #[test]
    fn a_sheet_says_what_is_left_and_the_party_line_adds_it_up() {
        let record = wizard(10);
        assert_eq!(
            SheetNow::of(&record).line(),
            "HP 17/25 · Temp 4 · AC 19 · Poisoned · Slots L1 1/2"
        );
        let older = wizard(5);
        let line = party_line(&[&record, &older], false).unwrap();
        assert!(
            line.starts_with("Party now: HP 34/50 · Slots 2/4 · Not connected"),
            "{line}"
        );
        assert_eq!(party_line(&[], true), None);
    }
    #[test]
    fn one_line_says_what_a_character_has_left() {
        let sheet = SheetNow {
            hp: 9,
            max_hp: 38,
            temporary_hp: 5,
            armor_class: Some(16),
            conditions: vec!["Poisoned".into()],
            slots: vec![
                SlotsLeft {
                    level: 1,
                    left: 2,
                    max: 4,
                },
                SlotsLeft {
                    level: 2,
                    left: 0,
                    max: 3,
                },
            ],
        };
        assert_eq!(
            sheet.line(),
            "HP 9/38 · Bloodied · Temp 5 · AC 16 · Poisoned · Slots L1 2/4, L2 0/3"
        );
        let whole = SheetNow {
            hp: 38,
            max_hp: 38,
            ..SheetNow::default()
        };
        assert_eq!(whole.line(), "HP 38/38");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// iter-20-R-FD8: a Party tile says where its attack lands, as Library and run tiles do,
    /// when the sheet gives it; a sheet without a distance keeps one line.
    #[test]
    fn a_sheets_attack_tile_says_where_it_lands() {
        let attack = |range: Value| json!({"name": "Dagger", "toHit": 6, "damage": {"value": {"diceString": "1d4", "fixedValue": 3}, "type": "Piercing"}, "range": range});
        let words = |range: Value| attack_chip(&attack(range)).unwrap().text;
        assert!(words(json!({"label": "Reach", "value": 5})).ends_with(". Reach 5 ft."));
        assert!(words(json!({"origin": "Ranged", "value": 120})).ends_with(". Range 120 ft."));
        assert!(
            words(json!({"origin": "Ranged", "value": 80, "longRangeValue": 320}))
                .ends_with(". Range 80/320 ft.")
        );
        // Must stay true: no distance, no second line, and the numbers are as before.
        for none in [Value::Null, json!({"origin": "Touch", "value": 0})] {
            let text = words(none);
            assert!(!text.contains("Reach") && !text.contains("Range"), "{text}");
            assert!(text.starts_with("Dagger · "), "{text}");
        }
    }
}
