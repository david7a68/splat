//! Character viewer, with combat sync on from the time Splat connects to D&D Beyond.
use crate::{
    auth::Session,
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    party_sync::{Reader, Stored},
};
use eframe::egui;
use serde_json::Value;
use splat::character::{ABILITIES, EntitySnapshot};
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
    /// The DM asked to go where this campaign's D&D Beyond link is set.
    pub wants_link_page: bool,
    pub wants_settings: bool,
    pub run_open: bool,
}
impl Party {
    #[cfg(test)]
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
        } else if self.reader.loading() {
            (icons::BUSY, "Refreshing", c.connection.busy)
        } else if self.combat.ready() {
            (
                icons::ONLINE,
                "Connected · Sync reads this sheet",
                c.connection.connected,
            )
        } else if !checked {
            (icons::ONLINE, "Connected · Cached sheet", c.connection.busy)
        } else if self.reader.stream_open {
            (icons::ONLINE, "Live updates", c.connection.connected)
        } else {
            (
                icons::ONLINE,
                "Connected · Checking for updates",
                c.connection.connected,
            )
        };
        let context = format!("{status_icon} {status_word}{fetched}");
        let mut refresh = false;
        let mut open_link = false;
        design::page_header(ui, "Party", Some(&context), |ui| {
            if linked && !self.reader.members.is_empty() {
                refresh = Action::quiet("Refresh party")
                    .icon(icons::REFRESH)
                    .blocked(can_refresh)
                    .show(ui)
                    .clicked();
                if session.is_none()
                    && !self.reader.members.is_empty()
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
            self.reader.refresh(session, ui.ctx(), true);
        }
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
                    "First connect your D&D Beyond account. Then choose its campaign under Campaign, Link and folder."
                } else {
                    "Choose its D&D Beyond campaign under Campaign, Link and folder."
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
        if let Some(e) = &self.reader.error {
            design::message(ui, Tone::Error, e);
        }
        if !self.reader.warnings.is_empty()
            || !self.reader.archived.is_empty()
            || !self.reader.conflicts.is_empty()
        {
            egui::ScrollArea::vertical()
                .id_salt("party_storage_notices")
                .max_height(ui.available_height() / 3.0)
                .auto_shrink([false, true])
                .show(ui, |ui| self.storage_notices(ui, session.is_some()));
        }
        ui.ctx().request_repaint_after(Duration::from_secs(1));
        if self.reader.members.is_empty() {
            if self.reader.loaded {
                design::empty_state(
                    ui,
                    icons::PARTY,
                    "This campaign has no party members.",
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
                    "No local characters yet",
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
                    "HP here is from tonight's fights, as of {}.",
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
            ui.add(egui::Label::new(Type::Small.rich(ui, words).color(c.muted)).wrap());
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
                member_header(ui);
                design::rule(ui);
                egui::ScrollArea::vertical()
                    .id_salt("party_members_list")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
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

    /// Keep local edits until the DM chooses to replace them. Replacement keeps a copy.
    fn storage_notices(&mut self, ui: &mut egui::Ui, connected: bool) {
        let warnings: std::collections::BTreeSet<_> = self.reader.warnings.values().collect();
        for warning in warnings {
            design::message(ui, Tone::Warning, &format!("Local storage: {warning}"));
        }
        let archives = self.reader.archived.clone();
        let mut dismiss = false;
        for path in &archives {
            let name = self
                .reader
                .archived_owners
                .get(path)
                .map(|id| self.reader.member_name(id))
                .unwrap_or("This character");
            let words =
                format!("{name}'s sheet now matches D&D Beyond. Your edited version is kept.");
            storage_notice(ui, Tone::Success, &words, &["Show file", "Dismiss"], |ui| {
                show_file(ui, path);
                dismiss |= Action::quiet("Dismiss")
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
            });
        }
        if dismiss {
            self.reader.archived.clear();
            self.reader.archived_owners.clear();
        }
        let conflicts = self.reader.conflicts.clone();
        for (id, path) in &conflicts {
            let name = self.reader.member_name(id).to_owned();
            let next = if connected {
                "Replacing keeps your version as a copy."
            } else {
                "Connect to replace it with the D&D Beyond sheet."
            };
            let words = format!(
                "{name}'s sheet file has your own edits, so Splat stopped updating it.\n{next}"
            );
            let waiting = self.reader.force_waiting();
            let action = if waiting {
                "Waiting for D&D Beyond."
            } else if connected {
                "Replace with D&D Beyond sheet"
            } else {
                "Connect D&D Beyond"
            };
            let actions = if connected {
                vec!["Show file", action]
            } else {
                vec!["Show file"]
            };
            storage_notice(ui, Tone::Warning, &words, &actions, |ui| {
                show_file(ui, path);
                if waiting && connected {
                    ui.label(Type::Small.rich(ui, action).color(colors().muted));
                } else if connected {
                    if Action::quiet(action)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        if conflicts.len() == 1 {
                            self.reader.force_read();
                        } else {
                            self.reader.force_read_character(id);
                        }
                    }
                }
            });
        }
        for id in self.reader.conflicts.keys() {
            if self.reader.selected.as_ref() != Some(id)
                && let Some(error) = self.reader.errors.get(id)
            {
                design::message(
                    ui,
                    Tone::Error,
                    &format!("{}: {error}", self.reader.member_name(id)),
                );
            }
        }
    }

    fn portrait(&self, record: Option<&EntitySnapshot>) -> Option<std::path::PathBuf> {
        crate::party_sync::portrait_path(self.reader.local_scope()?, record?)
    }

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
        if self.sync_stopped {
            let mut dismiss = false;
            ui.horizontal(|ui| {
                dismiss = design::icon_button(ui, icons::CLOSE, "Dismiss this message").clicked();
                design::message(ui, Tone::Warning, SYNC_STOPPED);
            });
            if dismiss {
                self.sync_stopped = false;
            }
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if design::link(ui, "D&D Beyond ↗").clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(format!(
                            "https://www.dndbeyond.com/characters/{id}"
                        )));
                    }
                });
            });
            ui.label(Type::Small.rich(ui, class_line(record)).color(c.muted));
        }
        if session.is_some() {
            // Under the name, and it says its state while closed.
            let state = if self.combat.enabled {
                format!("{} Combat sync · On", icons::ONLINE)
            } else {
                format!("{} Combat sync · Off", icons::SYNC_OFF)
            };
            let was = self.combat.enabled;
            design::section(ui, state, |ui| self.combat.show(ui));
            if was != self.combat.enabled {
                self.sync_turned_off = !self.combat.enabled;
            }
        }
        let Some(record) = newer(self.combat.record(), self.reader.sheets.get(id)) else {
            if !self.reader.loading() {
                ui.label(
                    Type::Body
                        .rich(ui, "No local copy is available for this character.")
                        .color(c.muted),
                );
            }
            return;
        };
        let before = self.page;
        design::segmented(
            ui,
            &mut self.page,
            &[
                (0, "Overview"),
                (1, "Actions"),
                (2, "Spells"),
                (3, "Equipment"),
                (4, "Features"),
            ],
        );
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
                    "Filter entries…",
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
        egui::ScrollArea::vertical()
            .id_salt((id, self.page))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // One person has one HP on this page: the list's, while tonight's fights count.
                let live = self
                    .tonight_values(record)
                    .map(|(hp, temporary_hp)| LiveValues {
                        hp: Some(hp),
                        max_hp: record.stat("max_hp"),
                        temporary_hp,
                        conditions: record
                            .initial
                            .conditions
                            .iter()
                            .filter_map(|condition| condition["name"].as_str().map(str::to_owned))
                            .collect(),
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
            });
    }
}

/// The later of two reads of a sheet. Off this page combat sync is idle, so the reader can be ahead.
/// Actions stay on the first line; the message owns both lines and its full-height bar.
fn storage_notice(
    ui: &mut egui::Ui,
    tone: Tone,
    words: &str,
    actions: &[&str],
    add: impl FnOnce(&mut egui::Ui),
) {
    let room: f32 = actions
        .iter()
        .map(|word| design::number_width(ui, word, &Type::Strong.font(ui)) + design::S * 3.0)
        .sum();
    ui.horizontal_top(|ui| {
        let width = (ui.available_width() - room - design::S).max(design::S);
        ui.allocate_ui_with_layout(
            egui::vec2(width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(width);
                ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
                design::message(ui, tone, words);
            },
        );
        add(ui);
    });
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
    pub conditions: Vec<String>,
}

/// One entry on a sheet page: its name and its facts.
const ENTRY_ROW: f32 = 40.0;
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
            conditions: record
                .initial
                .conditions
                .iter()
                .filter_map(|condition| condition["name"].as_str().map(str::to_owned))
                .collect(),
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

fn class_line(record: &EntitySnapshot) -> String {
    format!(
        "{} · {} · Level {}",
        record.character.species,
        record
            .character
            .classes
            .iter()
            .map(|c| text(&c["name"]))
            .collect::<Vec<_>>()
            .join(" / "),
        record.character.level
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

fn compare_values(record: Option<&EntitySnapshot>) -> [String; 3] {
    let Some(record) = record else {
        return ["—"; 3].map(str::to_owned);
    };
    let dc = array(&record.character.casting["saveDcs"])
        .iter()
        .filter_map(|value| value["value"].as_i64())
        .max()
        .map_or_else(|| "—".to_owned(), |dc| dc.to_string());
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
    let conditions: Vec<String> = record.map_or_else(Vec::new, |r| {
        r.initial
            .conditions
            .iter()
            .filter_map(|condition| condition["name"].as_str().map(str::to_owned))
            .collect()
    });
    let mut summary = [
        level.map(|n| format!("Level {n}")).unwrap_or_default(),
        classes,
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" ");
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
            painter.text(
                egui::pos2(left - design::XS, line),
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
        ui.add(egui::Label::new(Type::Small.rich(ui, text).color(colors().muted)).wrap());
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
    if page == 0 {
        overview(ui, record, live);
    } else {
        details(ui, record, page, search);
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
        ui.weak("Selections follow the D&D Beyond sheet. Active choices can need further details. Simulation support is separate.");
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
fn section_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(design::L);
    design::group_heading(ui, title, 0);
}
fn subsection_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(design::XS);
    design::group_heading(ui, title, 0);
}
const TILE_WIDTH: f32 = 112.0;

/// A row of stat tiles that breaks evenly: six across, or three, or two. A lone tile on a
/// second line reads as a different kind of value.
fn tiles(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    let each = TILE_WIDTH + ui.spacing().item_spacing.x;
    let room = ui.available_width();
    let across = [6.0, 3.0]
        .into_iter()
        .find(|n| n * each <= room)
        .unwrap_or(2.0);
    ui.allocate_ui_with_layout(
        egui::vec2(across * each, 0.0),
        egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(true),
        add,
    );
}

/// One label line and one value line, so tiles in a row keep their values level.
fn metric(ui: &mut egui::Ui, label: &str, value: impl ToString) {
    ui.allocate_ui(egui::vec2(TILE_WIDTH, 44.0), |ui| {
        ui.set_width(TILE_WIDTH - 4.0);
        design::stat_tile(ui, label, &value.to_string());
    });
}
fn val(record: &EntitySnapshot, key: &str) -> String {
    record
        .stat(key)
        .map(|n| n.to_string())
        .unwrap_or("—".into())
}
fn signed(record: &EntitySnapshot, key: &str) -> String {
    record
        .stat(key)
        .map(|n| format!("{n:+}"))
        .unwrap_or("—".into())
}
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
            metric(
                ui,
                &format!(
                    "{}{} {}",
                    ability[..1].to_uppercase(),
                    &ability[1..3],
                    val(record, ability)
                ),
                signed(record, &format!("{ability}_modifier")),
            );
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
                metric(ui, mode, format!("{} ft", stat.value));
            }
        }
    });
    let across = if ui.available_width() >= design::READER_MEASURE {
        3
    } else {
        2
    };
    subsection_heading(ui, "Saving throws");
    ui.columns(across, |cols| {
        for (at, ability) in ABILITIES.iter().enumerate() {
            cols[at % across].horizontal(|ui| {
                ui.label(
                    Type::Numeral
                        .rich(ui, signed(record, &format!("{ability}_save")))
                        .color(colors().text),
                );
                let mut name = ability.to_string();
                name[..1].make_ascii_uppercase();
                ui.label(name);
            });
        }
    });
    subsection_heading(ui, "Skills");
    ui.columns(across, |cols| {
        for (at, skill) in record.character.skills.iter().enumerate() {
            cols[at % across].horizontal(|ui| {
                ui.label(
                    Type::Numeral
                        .rich(ui, signed(record, &skill.stat))
                        .color(colors().text),
                );
                let flags = format!(
                    "{}{}{}",
                    if skill.expertise {
                        " (E)"
                    } else if skill.proficient {
                        " (P)"
                    } else if skill.half_proficient {
                        " (½)"
                    } else {
                        ""
                    },
                    if skill.advantage { " Adv" } else { "" },
                    if skill.disadvantage { " Dis" } else { "" }
                );
                ui.add(egui::Label::new(format!("{}{flags}", skill.name)).wrap());
            });
        }
    });
    ui.add(
        egui::Label::new(
            Type::Small
                .rich(
                    ui,
                    "P: proficient · E: expertise · ½: half proficiency · Adv/Dis: advantage/disadvantage",
                )
                .color(colors().muted),
        )
        .wrap(),
    );
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
        subsection_heading(ui, "Proficiencies & languages");
        for group in &record.character.proficiencies {
            let values = text(&group["values"]);
            if !values.is_empty() {
                ui.add(egui::Label::new(format!("{}: {values}", text(&group["group"]))).wrap());
            }
        }
    };
    let defenses = |ui: &mut egui::Ui| {
        subsection_heading(ui, "Defenses & senses");
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
        subsection_heading(ui, "Death saves");
        ui.label(format!(
            "{} successes · {} failures",
            record.initial.death_saves.successes, record.initial.death_saves.failures
        ));
        subsection_heading(ui, "Conditions");
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
            |live| live.conditions.clone(),
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
        ui.columns(2, |cols| {
            proficiencies(&mut cols[0]);
            defenses(&mut cols[1]);
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

fn detail_intro(ui: &mut egui::Ui, record: &EntitySnapshot, page: usize, search: &str) {
    match page {
        1 => {
            design::group_heading(ui, "Attacks", record.character.attacks.len());
            for attack in &record.character.attacks {
                let value = &attack["damage"]["value"];
                let damage = if let Some(n) = value.as_i64() {
                    n.to_string()
                } else {
                    format!(
                        "{}{}",
                        text(&value["diceString"]),
                        value["fixedValue"]
                            .as_i64()
                            .filter(|n| *n != 0)
                            .map(|n| format!("{n:+}"))
                            .unwrap_or_default()
                    )
                };
                let hit = attack["toHit"]
                    .as_i64()
                    .map(|n| format!("{n:+} to hit"))
                    .unwrap_or_else(|| {
                        format!(
                            "Save DC {} {}",
                            attack["saveInfo"]["value"],
                            text(&attack["saveInfo"]["label"])
                        )
                    });
                if text(&attack["name"])
                    .to_lowercase()
                    .contains(&search.to_lowercase())
                {
                    ui.label(format!(
                        "{} · {hit} · {damage} {}",
                        text(&attack["name"]),
                        text(&attack["damage"]["type"])
                    ));
                }
            }
            ui.add_space(design::S);
        }
        2 => {
            section_heading(ui, "Spellcasting");
            ui.weak("Spell descriptions use their printed values. Upcasting and character-specific formulas are not evaluated.");
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

pub(crate) fn details(ui: &mut egui::Ui, record: &EntitySnapshot, page: usize, search: &str) {
    detail_intro(ui, record, page, search);
    let entries = sheet_entries(record, page);
    let search = search.to_lowercase();
    let filtered: Vec<_> = entries
        .iter()
        .filter(|(name, source, _, _)| format!("{name} {source}").to_lowercase().contains(&search))
        .collect();
    if page != 2 {
        design::group_heading(
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
        let d = &record.source.raw;
        ui.label(format!(
            "Coins · {} cp · {} sp · {} ep · {} gp · {} pp",
            d["currencies"]["cp"],
            d["currencies"]["sp"],
            d["currencies"]["ep"],
            d["currencies"]["gp"],
            d["currencies"]["pp"]
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
            design::group_heading(
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
            ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
            ui.add_space(design::XS);
            for paragraph in description.split("\n\n") {
                ui.add(
                    egui::Label::new(Type::ProseCompact.rich(ui, paragraph).color(colors().text))
                        .wrap(),
                );
                ui.add_space(design::XS);
            }
        });
    }
}

#[cfg(test)]
mod sheet_now_tests {
    use super::{EntitySnapshot, SheetNow, SlotsLeft, party_line};
    use serde_json::json;
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
