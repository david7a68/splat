//! The Timeline page: the whole campaign in story order, apart from any one session.
use super::{Book, Event, Kind, clock, clock_now, clock_words, day_numbers, reference};
use crate::design::{self, Action, Density, Status, Tone, Type, colors, icons};
use eframe::egui;
use std::time::{Duration, Instant};
use uuid::Uuid;

const ROW: f32 = 44.0;
const SPINE: f32 = 16.0;
const TEXT_AT: f32 = 34.0;
const TWO_COLUMNS_FROM: f32 = 640.0;
const FILTERS_ON_ONE_ROW_FROM: f32 = design::LIST_MEASURE;
const UNDO_ADD_FOR: Duration = Duration::from_secs(30);
const COPIED_FOR: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Default, PartialEq)]
enum Filter {
    #[default]
    All,
    Days,
    Encounters,
    Rests,
    Travel,
    Events,
}

impl Filter {
    fn includes(self, kind: Kind) -> bool {
        match self {
            Self::All => true,
            Self::Days => kind == Kind::Day,
            Self::Encounters => kind == Kind::Encounter,
            Self::Rests => matches!(kind, Kind::ShortRest | Kind::LongRest),
            Self::Travel => kind == Kind::Travel,
            Self::Events => kind == Kind::Event,
        }
    }
}

/// A session, as the timeline knows it.
#[derive(Clone, Debug)]
pub struct SessionLink {
    pub id: Uuid,
    pub label: String,
    /// The events its note links to.
    pub mentions: Vec<String>,
}

#[derive(Default)]
pub struct TimelinePage {
    removed_mentions: usize,
    filter: Filter,
    search: String,
    session: Option<Uuid>,
    selected: Option<String>,
    copied: Option<(String, Instant)>,
    focus_title: bool,
    reset_detail: bool,
    scroll_to: Option<usize>,
    /// The Place after list just opened, so it shows the event's own place.
    show_place: bool,
    /// An event just added with the buttons, with one step of undo.
    just_added: Option<(String, Instant)>,
    /// The list opens where the story is now, at its end.
    opened: bool,
    /// The run open now, whose event must stay while it runs.
    pub open_run: Option<Uuid>,
    /// Saved runs from the same campaign snapshot used by the Session plan.
    pub saved_runs: Vec<crate::encounter_run::SavedRunInfo>,
    /// The DM followed an open fight's status back to the run.
    pub wants_back_to_run: bool,
    /// Filters and the selection belong to one campaign.
    campaign: Option<Uuid>,
    /// The DM asked to open this session's page.
    pub open_session: Option<Uuid>,
    /// The DM chose where new events go.
    pub new_events_to: Option<Option<Uuid>>,
    /// The session new events go to looks like an old one, in words.
    pub dated: Option<String>,
    /// The DM is playing that session after all.
    pub play_current: bool,
    /// The campaign's locations and their other names, to suggest for an event's place.
    pub places: Vec<(Uuid, Vec<String>)>,
    /// The DM asked to read this location.
    pub open_place: Option<Uuid>,
}

/// One button for each of these kinds of event. Returns the new event's ID.
pub fn quick_add(
    ui: &mut egui::Ui,
    book: &mut Book,
    session: Option<Uuid>,
    kinds: &[Kind],
    first: impl FnOnce(&mut egui::Ui),
) -> Option<String> {
    let blocked = book.unreadable.is_some();
    let names: Vec<_> = kinds
        .iter()
        .map(|kind| format!("Add {}", kind.word()))
        .collect();
    let actions = kinds
        .iter()
        .zip(&names)
        .map(|(kind, name)| {
            Action::quiet(kind.word())
                .icon(kind.icon())
                .name(name)
                .density(Density::Read)
                .blocked_if(blocked, "The timeline file must be repaired first.")
        })
        .collect();
    let always_visible = kinds.iter().position(|kind| *kind == Kind::LongRest);
    design::hanging_add_strip(ui, first, actions, always_visible).map(|index| {
        let kind = kinds[index];
        if kind == Kind::LongRest {
            book.add_long_rest(session)
        } else {
            book.add(kind, session)
        }
    })
}

pub enum RestRulingAction {
    Set(bool),
    Undo,
}

impl RestRulingAction {
    pub fn apply(self, book: &mut Book, id: &str) {
        match self {
            Self::Set(restores) => book.set_restores(id, restores),
            Self::Undo => book.undo_restores(),
        }
    }
}

/// Both pages use the same ruling words, icon and colour.
pub fn long_rest_status(restores: bool) -> Status<'static> {
    Status {
        icon: if restores {
            icons::OK
        } else {
            icons::LONG_REST
        },
        word: if restores {
            "Everyone at full"
        } else {
            "Not at full"
        },
        color: if restores {
            colors().document.saved
        } else {
            colors().muted
        },
    }
}

pub fn rest_ruling(ui: &mut egui::Ui, event: &Event, undo: bool) -> Option<RestRulingAction> {
    if event.kind != Kind::LongRest {
        return None;
    }
    let mut action = None;
    ui.horizontal_wrapped(|ui| {
        design::status(ui, long_rest_status(event.restores));
        if design::link(
            ui,
            if event.restores {
                "Mark not at full"
            } else {
                "Mark everyone at full"
            },
        )
        .clicked()
        {
            action = Some(RestRulingAction::Set(!event.restores));
        }
    });
    if undo && design::undo_message(ui, Tone::Info, "Long rest ruling changed.", "Undo ruling") {
        action = Some(RestRulingAction::Undo);
    }
    action
}

pub fn long_rest_added_message(saved_fights: impl Iterator<Item = String>) -> String {
    let mut words = "Long rest added. Everyone starts the next fight at full.".to_owned();
    for name in saved_fights {
        words.push_str(&format!(" {name} keeps its saved HP."));
    }
    words
}

/// What stops the timeline from saving. The DM must choose, so this cannot be dismissed.
/// Returns true when the comparison needs the rest of the pane.
pub fn problems(ui: &mut egui::Ui, book: &mut Book) -> bool {
    if let Some(count) = book.dropped()
        && design::undo_message(
            ui,
            Tone::Info,
            &format!(
                "Reloaded. Your version, with {}, is gone unless you undo.",
                design::count(count, "event", "events")
            ),
            "Undo reload",
        )
    {
        book.undo_reload();
    }
    for flag in &book.flags {
        design::message(
            ui,
            Tone::Warning,
            &format!("Timeline.md, line {}: {}", flag.line, flag.message),
        );
    }
    if !book.flags.is_empty()
        && let Some(path) = &book.path
        && let Some(folder) = path.parent()
    {
        crate::world::entry_file_actions(ui, folder, super::FILE, true);
    }
    if let Some(warning) = &book.duplicate {
        design::message(ui, Tone::Warning, warning);
    }
    if let Some(error) = book.unreadable.clone() {
        if crate::formats_ui::is_old(&error) {
            crate::formats_ui::old_notice(
                ui,
                &crate::formats_ui::old_message(super::FILE),
                book.path.clone().into_iter().collect(),
            );
        } else {
            design::message(ui, Tone::Error, &error);
        }
        ui.horizontal(|ui| {
            if Action::secondary("Read the file again").show(ui).clicked() {
                book.reload();
            }
            if book.gone
                && Action::quiet("Save this timeline there again")
                    .show(ui)
                    .clicked()
            {
                book.save_again();
            }
        });
    } else if book.conflict {
        design::message(
            ui,
            Tone::Warning,
            "Timeline.md changed on disk. Your draft is safe until you choose.",
        );
        ui.horizontal_wrapped(|ui| {
            if Action::secondary("Overwrite with my timeline")
                .show(ui)
                .clicked()
            {
                book.overwrite();
            }
            ui.add_space(design::L);
            if Action::destructive("Reload and lose my timeline")
                .show(ui)
                .clicked()
            {
                book.reload_keeping_undo();
            }
        });
        if book.conflict {
            match book.conflict_comparison() {
                Ok((draft, disk)) => {
                    let face = egui::TextStyle::Monospace.resolve(ui.style());
                    let height = ui.fonts_mut(|fonts| fonts.row_height(&face));
                    let id = ("timeline_conflict", &book.path, &disk);
                    design::draft_comparison_styled(ui, &draft, &disk, id, face, height);
                }
                Err(error) => {
                    design::message(ui, Tone::Error, &error);
                }
            }
            return true;
        }
    } else if let Some(error) = book.error.clone() {
        design::message(ui, Tone::Error, &error);
        if Action::secondary("Try to save again").show(ui).clicked() {
            book.error = None;
            book.save();
        }
    }
    false
}

/// The one step of undo after a removal. Returns whether it is showing.
pub fn undo_removed(ui: &mut egui::Ui, book: &mut Book) -> bool {
    let Some(removed) = book.removed().map(str::to_owned) else {
        return false;
    };
    if design::undo_message(
        ui,
        Tone::Info,
        &format!("Removed “{removed}”."),
        "Undo remove",
    ) {
        book.undo_remove();
    }
    true
}

impl TimelinePage {
    /// Show one event, whatever the filters were hiding.
    pub fn reveal(&mut self, id: &str, book: &Book) {
        self.filter = Filter::All;
        self.search.clear();
        self.session = None;
        self.reset_detail = true;
        self.selected = Some(id.to_owned());
        self.scroll_to = book.events.iter().position(|event| event.id == id);
    }

    fn matches(&self, event: &Event, day: u32) -> bool {
        if !self.filter.includes(event.kind) {
            return false;
        }
        if self.session.is_some() && event.session != self.session {
            return false;
        }
        let search = self.search.trim().to_lowercase();
        if search.is_empty() {
            return true;
        }
        [
            event.label(day).as_str(),
            event.place.as_str(),
            event.notes.as_str(),
            event.result.as_str(),
            event.kind.word(),
        ]
        .iter()
        .any(|text| text.to_lowercase().contains(&search))
    }

    /// Where a new event goes, said before it happens. An encounter run goes there too.
    fn new_events_line(
        &mut self,
        ui: &mut egui::Ui,
        sessions: &[SessionLink],
        current: Option<Uuid>,
    ) {
        ui.label(Type::Small.rich(ui, "Adds to:").color(colors().muted));
        let label = current
            .and_then(|id| sessions.iter().find(|session| session.id == id))
            .map_or("No session", |session| short_session(&session.label));
        let mut chosen = current;
        let inner = egui::ComboBox::from_id_salt("timeline_new_events_session")
            .truncate()
            .selected_text(label)
            .width(design::READER_MEASURE / 4.0)
            .icon(design::combo_icon)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut chosen, None, "No session");
                for session in sessions {
                    ui.selectable_value(&mut chosen, Some(session.id), &session.label);
                }
            });
        design::named(ui, &inner.response, "Session for new events");
        inner.response.on_hover_text(label);
        if chosen != current {
            self.new_events_to = Some(chosen);
        }
    }

    fn filters(&mut self, ui: &mut egui::Ui, sessions: &[SessionLink], current: Option<Uuid>) {
        let before = (self.filter, self.search.clone(), self.session);
        let tabs = |ui: &mut egui::Ui, filter: &mut Filter| {
            design::segmented(
                ui,
                filter,
                &[
                    (Filter::All, "All"),
                    (Filter::Days, "Days"),
                    (Filter::Encounters, "Fights"),
                    (Filter::Rests, "Rests"),
                    (Filter::Travel, "Travel"),
                    (Filter::Events, "Major events"),
                ],
            );
        };
        let wide = ui.available_width() >= FILTERS_ON_ONE_ROW_FROM;
        if !wide {
            tabs(ui, &mut self.filter);
        }
        ui.horizontal_wrapped(|ui| {
            if wide {
                tabs(ui, &mut self.filter);
            }
            design::search_field(ui, "Search the timeline", &mut self.search, "Search", 160.0);
            let label = self
                .session
                .and_then(|id| sessions.iter().find(|session| session.id == id))
                .map_or("All sessions", |session| session.label.as_str())
                .to_owned();
            let inner = egui::ComboBox::from_id_salt("timeline_session_filter")
                .truncate()
                .selected_text(&label)
                .width(150.0)
                .icon(design::combo_icon)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.session, None, "All sessions");
                    for session in sessions {
                        ui.selectable_value(&mut self.session, Some(session.id), &session.label);
                    }
                });
            design::named(ui, &inner.response, "Show one session");
            inner.response.on_hover_text(label);
            self.new_events_line(ui, sessions, current);
        });
        if before != (self.filter, self.search.clone(), self.session) {
            self.reset_detail = true;
        }
    }

    pub fn reopen(&mut self) {
        self.reset_detail = true;
    }

    fn row(
        &mut self,
        ui: &mut egui::Ui,
        event: &Event,
        day: u32,
        session: Option<&str>,
        flagged: bool,
    ) {
        let c = colors();
        let selected = self.selected.as_deref() == Some(event.id.as_str());
        let title = event.label(day);
        let open = event.run.is_some() && event.run == self.open_run;
        let summary = super::played_fight_summary(event, open);
        let mut detail = event.summary_with_result(session, &summary);
        // Repeat the day only when a filter or search can hide its heading.
        if event.kind != Kind::Day
            && day > 0
            && (self.filter != Filter::All
                || !self.search.trim().is_empty()
                || self.session.is_some())
        {
            detail = if detail.is_empty() {
                format!("Day {day}")
            } else {
                format!("Day {day} · {detail}")
            };
        }
        if event.kind == Kind::LongRest {
            if !detail.is_empty() {
                detail.push_str(" · ");
            }
            let status = long_rest_status(event.restores);
            detail.push_str(&format!("{} {}", status.icon, status.word));
        }
        let name = if detail.is_empty() {
            title.clone()
        } else {
            format!("{title} · {detail}")
        };
        let response = design::row(ui, ROW, selected, &name, |ui, rect| {
            let painter = ui.painter();
            let spine = rect.left() + SPINE;
            let width = rect.width() - TEXT_AT - design::S;
            let icon_y = rect.center().y;
            if event.kind != Kind::Day {
                for (from, to) in [(rect.top(), icon_y - 11.0), (icon_y + 11.0, rect.bottom())] {
                    painter.vline(
                        spine,
                        egui::Rangef::new(from, to),
                        egui::Stroke::new(design::HAIRLINE_WIDTH, c.rule),
                    );
                }
            }
            painter.text(
                egui::pos2(spine, icon_y),
                egui::Align2::CENTER_CENTER,
                if flagged {
                    icons::WARN
                } else {
                    event.kind.icon()
                },
                Type::Name.font(ui),
                if flagged {
                    c.result.warning
                } else if event.kind == Kind::Day {
                    c.text
                } else {
                    c.muted
                },
            );
            let galley = if event.kind == Kind::Day {
                let mut job = design::group_heading_text(ui, &title, 0);
                job.wrap.max_width = width;
                job.wrap.max_rows = 1;
                painter.layout_job(job)
            } else {
                let font = if selected { Type::Strong } else { Type::Body };
                design::one_line(ui, &title, font.font(ui), c.text, width)
            };
            let lines = if detail.is_empty() {
                galley.size().y
            } else {
                galley.size().y + Type::Small.line()
            };
            let top = rect.center().y - lines / 2.0;
            let x = rect.left() + TEXT_AT;
            painter.galley(egui::pos2(x, top), galley, c.text);
            if !detail.is_empty() && !open {
                let second = design::one_line(ui, &detail, Type::Small.font(ui), c.muted, width);
                painter.galley(
                    egui::pos2(x, top + lines - Type::Small.line()),
                    second,
                    c.muted,
                );
            }
        });
        if open && !summary.is_empty() {
            // The link uses the same second-line position as every other row.
            let title_height = ui.fonts_mut(|fonts| fonts.row_height(&Type::Body.font(ui)));
            let top = response.rect.center().y - (title_height + Type::Small.line()) / 2.0;
            let rect = egui::Rect::from_min_max(
                egui::pos2(response.rect.left() + TEXT_AT, top + title_height),
                egui::pos2(
                    response.rect.right() - design::S,
                    top + title_height + Type::Small.line(),
                ),
            );
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .accessibility_parent(response.id),
                |ui| {
                    ui.spacing_mut().interact_size.y = 0.0;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        let (before, after) = detail.split_once(&summary).unwrap_or(("", ""));
                        ui.label(Type::Small.rich(ui, before).color(c.muted));
                        if design::inline_status_link(
                            ui,
                            Status {
                                icon: icons::RUN,
                                word: &summary,
                                color: c.text,
                            },
                        )
                        .clicked()
                        {
                            self.wants_back_to_run = true;
                        }
                        ui.add(
                            egui::Label::new(Type::Small.rich(ui, after).color(c.muted)).truncate(),
                        );
                    });
                },
            );
        }
        if response.clicked() {
            self.reset_detail = self.selected.as_ref() != Some(&event.id);
            self.selected = Some(event.id.clone());
            self.focus_title = false;
        }
    }

    /// Locations the typed place could be, or a link to the one it is. The place stays text.
    fn place_help(&mut self, ui: &mut egui::Ui, event: &mut Event) -> bool {
        let typed = event.place.trim().to_lowercase();
        if typed.is_empty() {
            return false;
        }
        let exact: Vec<_> = self
            .places
            .iter()
            .filter(|(_, names)| names.iter().any(|name| name.to_lowercase() == typed))
            .collect();
        if let [(id, names)] = exact.as_slice() {
            if design::link(ui, &format!("Read {}", names[0])).clicked() {
                self.open_place = Some(*id);
            }
            return false;
        }
        let close: Vec<_> = self
            .places
            .iter()
            .filter_map(|(_, names)| {
                names
                    .iter()
                    .find(|name| name.to_lowercase().contains(&typed))
            })
            .take(5)
            .collect();
        if close.is_empty() {
            return false;
        }
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            for name in close {
                if design::link(ui, name).clicked() {
                    event.place = name.clone();
                    changed = true;
                }
            }
        });
        changed
    }

    fn list_messages(&mut self, ui: &mut egui::Ui, book: &mut Book) {
        if let Some(label) = book.removed() {
            let mentions = self.removed_mentions;
            let words = if mentions == 0 {
                format!("Removed “{label}”.")
            } else {
                format!(
                    "Removed “{label}”. {} still {} to it.",
                    design::count(mentions, "note", "notes"),
                    if mentions == 1 { "links" } else { "link" }
                )
            };
            let tone = if mentions > 0 {
                Tone::Warning
            } else {
                Tone::Info
            };
            if design::undo_message(ui, tone, &words, "Undo remove") {
                book.undo_remove();
            }
        }
        // It goes when the DM moves on, or soon, so that it cannot undo the wrong event.
        self.just_added = self
            .just_added
            .take()
            .filter(|(id, at)| at.elapsed() < UNDO_ADD_FOR && self.selected.as_ref() == Some(id));
        if let Some((id, _)) = self.just_added.clone() {
            ui.ctx().request_repaint_after(UNDO_ADD_FOR);
            let days = day_numbers(&book.events);
            let label = book
                .events
                .iter()
                .position(|event| event.id == id)
                .map_or_else(String::new, |at| book.events[at].label(days[at]));
            let event = book.events.iter().find(|event| event.id == id);
            let words = if let Some(event) =
                event.filter(|event| event.kind == Kind::LongRest && event.restores)
            {
                long_rest_added_message(
                    super::fights_that_keep_hp(&book.events, event.session, &self.saved_runs)
                        .into_iter(),
                )
            } else {
                format!("{label} added.")
            };
            if design::undo_message(ui, Tone::Success, &words, "Undo add") {
                book.undo_add(&id);
                self.just_added = None;
                self.selected = None;
            }
        }
    }

    fn list(&mut self, ui: &mut egui::Ui, book: &mut Book, sessions: &[SessionLink]) {
        self.list_messages(ui, book);
        if book.merged_outside {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        "Timeline.md also changed outside Splat. Both changes are kept.",
                    )
                    .color(colors().muted),
            );
        }
        let flagged = book
            .baseline
            .as_deref()
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .map(super::formats::flagged_days)
            .unwrap_or_default();
        let days = day_numbers(&book.events);
        let rows: Vec<usize> = (0..book.events.len())
            .filter(|&index| self.matches(&book.events[index], days[index]))
            .collect();
        if rows.is_empty() {
            // An unreadable file has its own message, and no filter caused it.
            if book.unreadable.is_none() {
                design::empty_state(
                    ui,
                    icons::TIMELINE,
                    "No matching events",
                    "Clear the search and filters to see the whole timeline.",
                    |ui| {
                        if design::link(ui, "Show all events").clicked() {
                            self.search.clear();
                            self.filter = Filter::All;
                            self.session = None;
                        }
                    },
                );
            }
            return;
        }
        let step = ROW + ui.spacing().item_spacing.y;
        let mut area = egui::ScrollArea::vertical()
            .id_salt("timeline_list")
            .auto_shrink([false, false]);
        if let Some(target) = self.scroll_to.take()
            && let Some(row) = rows.iter().position(|&index| index == target)
        {
            area = area.vertical_scroll_offset((row as f32 * step - step).max(0.0));
        }
        area.show_rows(ui, ROW, rows.len(), |ui, range| {
            for &index in &rows[range] {
                let event = &book.events[index];
                let session = event
                    .session
                    .and_then(|id| sessions.iter().find(|session| session.id == id))
                    .map(|session| short_session(&session.label));
                self.row(ui, event, days[index], session, flagged.contains(&event.id));
            }
        });
    }

    fn detail(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        sessions: &[SessionLink],
        notes: &mut crate::sessions::page::Sessions,
    ) {
        let c = colors();
        let Some(id) = self
            .selected
            .clone()
            .filter(|id| book.events.iter().any(|event| event.id == *id))
        else {
            ui.label(Type::Heading.rich(ui, "An event").color(c.text));
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(
                            ui,
                            "Select an event to name it, place it or note what it means. Each event has its own link, for a session note or an Obsidian note.",
                        )
                        .color(c.muted),
                )
                .wrap(),
            );
            return;
        };
        let days = day_numbers(&book.events);
        let index = book
            .events
            .iter()
            .position(|event| event.id == id)
            .expect("the selected event");
        let link = reference(&book.events[index], days[index]);
        let unsaved = book.event_unsaved(&id);
        let blocked = book
            .unreadable
            .as_deref()
            .or(book.error.as_deref())
            .or(if book.conflict {
                Some("Changed on disk. Choose what to keep above.")
            } else {
                None
            });
        let mut changed = false;
        let mut removed = false;
        let mut placed: Option<Option<String>> = None;
        let labels: Vec<String> = book
            .events
            .iter()
            .zip(&days)
            .map(|(event, day)| {
                let session = event
                    .session
                    .and_then(|id| sessions.iter().find(|session| session.id == id));
                let mut label = event.label(*day);
                if event.kind != Kind::Day && *day > 0 {
                    label.push_str(&format!(" · Day {day}"));
                }
                if let Some(session) = session {
                    label.push_str(&format!(" · {}", short_session(&session.label)));
                }
                label
            })
            .collect();
        let ids: Vec<String> = book.events.iter().map(|event| event.id.clone()).collect();
        let starts = clock(&book.events)[index];
        self.copied = self
            .copied
            .take()
            .filter(|(_, at)| at.elapsed() < COPIED_FOR);
        if self.copied.is_some() {
            ui.ctx().request_repaint_after(COPIED_FOR);
        }
        let event = &book.events[index];
        let title = event.label(days[index]);
        let icon_only = event.kind == Kind::Day || event.title.trim().is_empty();
        let kind = header_kind(event);
        let needed = design::number_width(ui, &title, &Type::Heading.font(ui))
            + design::number_width(ui, &kind, &Type::Small.font(ui))
            + design::number_width(ui, "Saving blocked", &Type::Small.font(ui))
            + design::number_width(ui, icons::OK, &Type::Small.font(ui))
            + Action::secondary("Copy link")
                .icon(icons::COPY)
                .density(Density::Read)
                .natural_width(ui)
            + Density::Read.control()
            + design::S * 5.0;
        let title_line = |ui: &mut egui::Ui| {
            let kind_label = ui.label(Type::Small.rich(ui, &kind).color(c.muted));
            if icon_only {
                design::named(ui, &kind_label, event.kind.word());
                kind_label.on_hover_text(event.kind.word());
            }
            let shown =
                design::words_that_fit(ui, &title, &Type::Heading.font(ui), ui.available_width());
            ui.label(Type::Heading.rich(ui, shown).color(c.text))
                .on_hover_text(&title);
        };
        let mut actions = |ui: &mut egui::Ui| {
            let (icon, word, color) = if blocked.is_some() {
                (
                    icons::WARN,
                    if unsaved { "Unsaved" } else { "Saving blocked" },
                    c.document.external,
                )
            } else if unsaved {
                (icons::BUSY, "Saving…", c.muted)
            } else {
                (icons::OK, "Saved", c.document.clean)
            };

            let more = design::icon_button(
                ui,
                icons::EXPAND,
                &format!("More actions for {}", event.label(days[index])),
            );
            egui::Popup::menu(&more).show(|ui| {
                let open_run = event.run.is_some() && event.run == self.open_run;
                removed = Action::quiet("Remove")
                    .name("Remove this event")
                    .blocked_if(open_run, "This run is open. Close it first.")
                    .show(ui)
                    .clicked();
                if open_run {
                    ui.label(
                        Type::Small
                            .rich(ui, "This run is open. Close it first.")
                            .color(c.muted),
                    );
                }
                if removed {
                    ui.close();
                }
            });
            let copied = self
                .copied
                .as_ref()
                .is_some_and(|(copied, _)| *copied == id);
            if Action::secondary(if copied { "Copied" } else { "Copy link" })
                .icon(if copied { icons::OK } else { icons::COPY })
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                ui.ctx().copy_text(link.clone());
                self.copied = Some((id.clone(), Instant::now()));
            }
            design::status(ui, design::Status { icon, word, color });
        };
        let width_id = ui
            .id()
            .with(("timeline_form_width", ui.available_width().to_bits()));
        let header_width = ui
            .ctx()
            .data(|data| data.get_temp::<f32>(width_id))
            .unwrap_or_else(|| {
                (ui.available_width() - ui.spacing().scroll.allocated_width())
                    .min(design::FORM_WIDTH)
            });
        ui.allocate_ui_with_layout(
            egui::vec2(header_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                if needed <= ui.available_width() {
                    ui.horizontal(|ui| {
                        title_line(ui);
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            &mut actions,
                        );
                    });
                } else {
                    ui.horizontal(title_line);
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), Density::Read.control()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        actions,
                    );
                }
            },
        );
        if let Some(reason) = blocked {
            ui.label(Type::Small.rich(ui, reason).color(c.muted));
        }
        design::rule(ui);
        ui.scope(|ui| {
            ui.set_max_width(header_width);
            let event = &mut book.events[index];
            ui.add_space(design::XS);
            let hint = (event.kind == Kind::Day)
                .then_some("Optional. Days number themselves, so a name is extra.");
            let title = design::text_field(ui, "Title", &mut event.title, hint, None);
            changed |= title.changed();
            if std::mem::take(&mut self.focus_title) {
                title.request_focus();
            }
        });
        ui.add_space(design::S);
        design::rule(ui);
        let can_undo_ruling = book.rest_undo_id() == Some(id.as_str());
        let mut ruling = None;
        let mut area = egui::ScrollArea::vertical()
            .id_salt(("timeline_detail", &id))
            .auto_shrink([false, false]);
        if std::mem::take(&mut self.reset_detail) {
            area = area.vertical_scroll_offset(0.0);
        }
        let clip = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        area.show(ui, |ui| {
            ui.set_clip_rect(ui.clip_rect().intersect(clip));
            let form_width = ui.available_width().min(design::FORM_WIDTH);
            ui.ctx()
                .data_mut(|data| data.insert_temp(width_id, form_width));
            if (form_width - header_width).abs() > design::HAIRLINE_WIDTH {
                ui.ctx().request_repaint();
            }
            ui.set_max_width(form_width);
            notes.event_link_offer(ui, book, &id);
            let event = &mut book.events[index];
            ui.add_space(design::M);
            let day_start = starts
                .filter(|_| event.kind == Kind::Day)
                .map(|start| format!("Starts: {}", clock_words(start)));
            design::field(ui, "Kind", day_start.as_deref(), None, |ui| {
                let inner = egui::ComboBox::from_id_salt("timeline_event_kind")
                    .selected_text(event.kind.word())
                    .width(ui.available_width().min(design::FORM_WIDTH))
                    .icon(design::combo_icon)
                    .show_ui(ui, |ui| {
                        for kind in Kind::ALL {
                            changed |= ui
                                .selectable_value(&mut event.kind, kind, kind.word())
                                .changed();
                        }
                    });
                design::named(ui, &inner.response, "Kind of event");
            });
            if event.kind != Kind::Day {
                ui.add_space(design::M);
                design::field(
                    ui,
                    "How long",
                    Some("A number and a unit move the clock: 3 days, 8 hours, an hour."),
                    None,
                    |ui| {
                        let hint = match event.kind {
                            Kind::LongRest => "8 hours",
                            Kind::ShortRest => "1 hour",
                            _ => "",
                        };
                        changed |= design::text_input(
                            ui,
                            "How long",
                            &mut event.duration,
                            hint,
                            ui.available_width(),
                        )
                        .changed();
                    },
                );
            }
            ruling = rest_ruling(ui, event, can_undo_ruling);
            if let Some(start) = starts.filter(|_| event.kind != Kind::Day) {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        Type::Small
                            .rich(ui, format!("Starts: {}", clock_words(start)))
                            .color(c.muted),
                    );
                    if event.kind != Kind::Day {
                        ui.label(
                            Type::Small
                                .rich(
                                    ui,
                                    format!(
                                        "Ends: {}",
                                        clock_words(start.saturating_add(event.minutes()))
                                    ),
                                )
                                .color(c.muted),
                        );
                    }
                });
            }
            ui.add_space(design::M);
            changed |= design::text_field(ui, "Where", &mut event.place, None, None).changed();
            changed |= self.place_help(ui, event);
            ui.add_space(design::M);
            design::field(ui, "Session", None, None, |ui| {
                let label = event
                    .session
                    .and_then(|id| sessions.iter().find(|session| session.id == id))
                    .map_or("No session", |session| session.label.as_str());
                let inner = egui::ComboBox::from_id_salt("timeline_event_session")
                    .truncate()
                    .selected_text(label)
                    .width(ui.available_width().min(design::FORM_WIDTH))
                    .icon(design::combo_icon)
                    .show_ui(ui, |ui| {
                        changed |= ui
                            .selectable_value(&mut event.session, None, "No session")
                            .changed();
                        for session in sessions {
                            changed |= ui
                                .selectable_value(
                                    &mut event.session,
                                    Some(session.id),
                                    &session.label,
                                )
                                .changed();
                        }
                    });
                design::named(ui, &inner.response, "Session of this event");
            });
            // A session whose note has gone has nothing to open.
            if let Some(session) = event.session
                && sessions.iter().any(|known| known.id == session)
                && design::link(ui, "Open this session").clicked()
            {
                self.open_session = Some(session);
            }
            if !event.result.is_empty() {
                ui.add_space(design::M);
                design::field(ui, "From the run", None, None, |ui| {
                    ui.label(
                        Type::Body
                            .rich(
                                ui,
                                super::played_fight_summary(
                                    event,
                                    event.run.is_some() && event.run == self.open_run,
                                ),
                            )
                            .color(c.text),
                    );
                });
            }
            ui.add_space(design::M);
            design::field(ui, "Notes", None, None, |ui| {
                let response = ui.add(
                    egui::TextEdit::multiline(&mut event.notes)
                        .desired_width(ui.available_width() - 2.0 * design::S)
                        .desired_rows(3)
                        .margin(egui::vec2(design::S, design::S)),
                );
                design::named(ui, &response, "Notes on this event");
                changed |= response.changed();
            });
            for (label, value) in &event.extra {
                if let (Some(label), Some(value)) = (label.as_str(), value.as_str()) {
                    ui.label(
                        Type::Body
                            .rich(ui, format!("{label}: {value}"))
                            .color(c.muted),
                    );
                }
            }
            design::quiet_section(ui, "Link text", |ui| {
                ui.add(egui::Label::new(Type::Mono.rich(ui, &link).color(c.muted)).wrap());
            });
            let mentions: Vec<&SessionLink> = sessions
                .iter()
                .filter(|session| session.mentions.contains(&id))
                .collect();
            if !mentions.is_empty() {
                design::field(ui, "Mentioned in", None, None, |ui| {
                    for session in mentions {
                        if design::link(ui, &session.label).clicked() {
                            self.open_session = Some(session.id);
                        }
                    }
                });
            }
            ui.add_space(design::XS);
            ui.add_space(design::M);
            design::field(ui, "Place after", None, None, |ui| {
                let before = index
                    .checked_sub(1)
                    .map(|at| labels[at].clone())
                    .unwrap_or_else(|| "Start of the timeline".to_owned());
                let inner = egui::ComboBox::from_id_salt("timeline_place_after")
                    .selected_text(before)
                    .width(ui.available_width().min(design::FORM_WIDTH))
                    .icon(design::combo_icon)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(false, "Start of the timeline")
                            .clicked()
                        {
                            placed = Some(None);
                        }
                        for (at, label) in labels.iter().enumerate() {
                            if at != index {
                                let row = ui.selectable_label(at + 1 == index, label);
                                if row.clicked() {
                                    placed = Some(Some(ids[at].clone()));
                                }
                                if at + 1 == index && std::mem::take(&mut self.show_place) {
                                    row.scroll_to_me(Some(egui::Align::Center));
                                }
                            }
                        }
                    });
                design::named(ui, &inner.response, "Place this event after");
                if inner.response.clicked() {
                    self.show_place = true;
                }
            });
        });
        if changed {
            book.touch();
        }
        if let Some(action) = ruling {
            action.apply(book, &id);
            self.just_added = None;
        }
        if let Some(after) = placed {
            book.place_after(&id, after.as_deref());
            self.scroll_to = book.events.iter().position(|event| event.id == id);
        }
        if removed {
            self.just_added = None;
            self.removed_mentions = sessions
                .iter()
                .filter(|session| session.mentions.contains(&id))
                .count();
            book.remove(&id);
            self.selected = None;
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        campaign_id: Uuid,
        sessions: &[SessionLink],
        current: Option<Uuid>,
        notes: &mut crate::sessions::page::Sessions,
    ) {
        let page_width = ui.available_width();
        // Only a real change: the first show can already have an event to reveal.
        if self
            .campaign
            .is_some_and(|campaign| campaign != campaign_id)
        {
            *self = Self {
                open_run: self.open_run,
                saved_runs: std::mem::take(&mut self.saved_runs),
                ..Default::default()
            };
        }
        self.campaign = Some(campaign_id);
        design::page_header(
            ui,
            "Timeline",
            Some(&if book.events.is_empty() {
                String::new()
            } else {
                format!(
                    "{} · Now: {}",
                    if self.filter != Filter::All
                        || !self.search.trim().is_empty()
                        || self.session.is_some()
                    {
                        let days = day_numbers(&book.events);
                        let shown = book
                            .events
                            .iter()
                            .zip(days)
                            .filter(|(event, day)| self.matches(event, *day))
                            .count();
                        format!(
                            "{} of {}",
                            if shown == 0 {
                                "0".into()
                            } else {
                                design::count(shown, "", "")
                            },
                            design::count(book.events.len(), "event", "events")
                        )
                    } else {
                        design::count(book.events.len(), "event", "events")
                    },
                    clock_now(&book.events).map_or_else(|| "not set".to_owned(), clock_words)
                )
            }),
            |_| {},
        );
        if problems(ui, book) {
            return;
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
            book.save();
        }
        if let Some(id) = quick_add(
            ui,
            book,
            current,
            &[
                Kind::Encounter,
                Kind::Day,
                Kind::LongRest,
                Kind::ShortRest,
                Kind::Travel,
                Kind::Event,
            ],
            |_| {},
        ) {
            self.reveal(&id, book);
            self.focus_title = true;
            self.just_added = Some((id, Instant::now()));
        }
        if let Some(words) = self.dated.clone()
            && design::message_with_action(ui, Tone::Warning, &words, "Play this one")
        {
            self.play_current = true;
        }
        ui.add_space(design::XS);
        if book.events.is_empty() && book.unreadable.is_none() {
            self.list_messages(ui, book);
            let mut first = false;
            design::empty_state(
                ui,
                icons::TIMELINE,
                "Nothing on the timeline yet",
                "Add the first day above. Each encounter run appears here by itself, in the session it belongs to.",
                |ui| {
                    first = if self.open_run.is_some() {
                        Action::secondary("Start with day 1")
                    } else {
                        Action::primary("Start with day 1")
                    }
                    .show(ui)
                    .clicked();
                },
            );
            if first {
                let id = book.add(Kind::Day, current);
                self.reveal(&id, book);
                self.focus_title = true;
                self.just_added = Some((id, Instant::now()));
            }
            return;
        }
        if !std::mem::replace(&mut self.opened, true) && self.scroll_to.is_none() {
            self.scroll_to = book.events.len().checked_sub(1);
        }
        // One column, with an event open: the filters would only push it down.
        let narrow = page_width < TWO_COLUMNS_FROM;
        let event_open = self
            .selected
            .as_ref()
            .is_some_and(|id| book.events.iter().any(|event| event.id == *id));
        if !(narrow && event_open) {
            self.filters(ui, sessions, current);
        }
        ui.add_space(design::XS);
        // Beside Lookup or Notes at a small window, two columns would crush the list.
        if narrow {
            if event_open {
                if Action::quiet("All events")
                    .icon(icons::BACK)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.selected = None;
                }
                self.detail(ui, book, sessions, notes);
            } else {
                notes.expire_event_link_undo(true, book);
                notes.event_link_undo(ui);
                self.list(ui, book, sessions);
            }
            return;
        }
        if !event_open {
            notes.event_link_undo(ui);
            self.list(ui, book, sessions);
            return;
        }
        let width = design::side_panel_width(ui.available_width());
        let content_width = ui.available_width().min(design::LIST_MEASURE + width);
        ui.allocate_ui_with_layout(
            egui::vec2(content_width, ui.available_height()),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                design::Column::panel("timeline_detail_panel", width)
                    .frame(egui::Frame::new().inner_margin(egui::Margin {
                        left: design::S as i8,
                        ..Default::default()
                    }))
                    .show_inside(ui, |ui| self.detail(ui, book, sessions, notes));
                egui::CentralPanel::default()
                    // The panel's drag handle lies on this edge, so the rows stop short of it.
                    .frame(egui::Frame::new().inner_margin(egui::Margin {
                        right: design::S as i8,
                        ..Default::default()
                    }))
                    .show_inside(ui, |ui| {
                        notes.expire_event_link_undo(true, book);
                        notes.event_link_undo(ui);
                        self.list(ui, book, sessions)
                    });
            },
        );
    }
}

/// The session's number alone. The row's own title is the event's.
fn short_session(label: &str) -> &str {
    label.split(':').next().unwrap_or(label)
}

#[cfg(test)]
pub fn check_long_rest_add(kinds: &'static [Kind]) {
    use egui_kittest::kittest::Queryable;
    let session = Some(Uuid::new_v4());
    let mut harness = egui_kittest::Harness::builder().build_state(
        move |ctx, book: &mut Book| {
            egui::CentralPanel::default().show(ctx, |ui| {
                quick_add(ui, book, session, kinds, |_| {});
            });
        },
        Book::default(),
    );
    design::theme::install(&harness.ctx);
    harness.run_steps(2);
    harness.get_by_label("Add Long rest").click();
    harness.run_steps(2);
    assert_eq!(harness.state().events.len(), 1);
    assert!(super::party_state(&harness.state().events, session).rested);
    let id = harness.state().events[0].id.clone();
    harness.state_mut().remove(&id);
    assert!(!super::party_state(&harness.state().events, session).rested);
}

fn header_kind(event: &Event) -> String {
    if event.kind == Kind::Day || event.title.trim().is_empty() {
        event.kind.icon().to_owned()
    } else {
        format!("{} {}", event.kind.icon(), event.kind.word())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter3_c_closed_long_rest_rows_show_both_rulings() {
        use egui_kittest::kittest::Queryable;
        for restores in [false, true] {
            let mut book = Book::default();
            book.add(Kind::LongRest, None);
            let mut event = book.events.remove(0);
            event.restores = restores;
            let word = if restores {
                "Everyone at full"
            } else {
                "Not at full"
            };
            let icon = if restores {
                icons::OK
            } else {
                icons::LONG_REST
            };
            let expected = format!("{} · {icon} {word}", event.label(0));
            let mut harness = egui_kittest::Harness::builder().build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    TimelinePage::default().row(ui, &event, 0, None, false);
                });
            });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            harness.get_by_label(&expected);
        }
    }

    #[test]
    fn iter1_c_day_appears_only_when_filters_hide_day_headings() {
        use egui_kittest::kittest::Queryable;
        for filtered in [false, true] {
            let mut book = Book::default();
            book.add(Kind::ShortRest, None);
            let event = book.events.remove(0);
            let detail = event.summary_with_result(None, "");
            let expected_detail = if filtered {
                if detail.is_empty() {
                    "Day 1".into()
                } else {
                    format!("Day 1 · {detail}")
                }
            } else {
                detail
            };
            let expected = if expected_detail.is_empty() {
                event.label(1)
            } else {
                format!("{} · {expected_detail}", event.label(1))
            };
            let mut harness = egui_kittest::Harness::builder().build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut page = TimelinePage::default();
                    if filtered {
                        page.filter = Filter::Rests;
                    }
                    page.row(ui, &event, 1, None, false);
                });
            });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            harness.get_by_label(&expected);
        }
    }

    #[test]
    fn iter1_c_rest_ruling_actions_name_the_change_and_keep_undo() {
        use egui_kittest::kittest::Queryable;
        let mut book = Book::default();
        book.add(Kind::LongRest, None);
        book.events[0].restores = true;
        let event_id = book.events[0].id.clone();
        let mut harness = egui_kittest::Harness::builder().build_state(
            move |ctx, book: &mut Book| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(action) = rest_ruling(ui, &book.events[0], true) {
                        action.apply(book, &event_id);
                    }
                });
            },
            book,
        );
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.get_by_label("Mark not at full").click();
        harness.run_steps(2);
        assert!(!harness.state().events[0].restores);
        harness.get_by_label("Mark everyone at full").click();
        harness.run_steps(2);
        assert!(harness.state().events[0].restores);
        harness.get_by_label("Undo ruling").click();
        harness.run_steps(2);
        assert!(!harness.state().events[0].restores);
    }

    #[test]
    fn an_untitled_rest_names_its_kind_once_in_the_header() {
        let mut book = Book::default();
        book.add(Kind::LongRest, None);
        let mut event = book.events.remove(0);
        assert_eq!(
            format!("{} {}", header_kind(&event), event.label(1))
                .matches("Long rest")
                .count(),
            1
        );
        event.title = "Rest at the mill".into();
        assert!(header_kind(&event).contains("Long rest"));
        assert_eq!(event.label(1), "Rest at the mill");
    }

    #[test]
    fn long_rest_added_on_timeline_restores_and_undo_removes_its_ruling() {
        check_long_rest_add(&Kind::ALL);
    }
}
