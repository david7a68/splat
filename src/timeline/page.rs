//! The Timeline page: the whole campaign in story order, apart from any one session.
use super::{Book, Event, Kind, clock, clock_now, clock_words, day_numbers, reference};
use crate::design::{self, Action, Density, Tone, Type, colors, icons};
use eframe::egui;
use std::time::{Duration, Instant};
use uuid::Uuid;

const ROW: f32 = 44.0;
const SPINE: f32 = 16.0;
const TEXT_AT: f32 = 34.0;
const TWO_COLUMNS_FROM: f32 = 640.0;
const FILTERS_ON_ONE_ROW_FROM: f32 = 720.0;
const UNDO_ADD_FOR: Duration = Duration::from_secs(30);
const COPIED_FOR: Duration = Duration::from_secs(4);

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
    filter: Filter,
    search: String,
    session: Option<Uuid>,
    selected: Option<String>,
    copied: Option<(String, Instant)>,
    focus_title: bool,
    scroll_to: Option<usize>,
    /// The Place after list just opened, so it shows the event's own place.
    show_place: bool,
    /// An event just added with the buttons, with one step of undo.
    just_added: Option<(String, Instant)>,
    /// The list opens where the story is now, at its end.
    opened: bool,
    /// The run open now, whose event must stay while it runs.
    pub open_run: Option<Uuid>,
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
    let mut added = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = design::XS;
        ui.label(Type::Small.rich(ui, "Add").color(colors().muted));
        first(ui);
        for &kind in kinds {
            if Action::secondary(kind.word())
                .icon(kind.icon())
                .name(&format!("Add {}", kind.word()))
                .density(Density::Read)
                .blocked_if(blocked, "The timeline file must be repaired first.")
                .show(ui)
                .clicked()
            {
                added = Some(book.add(kind, session));
            }
        }
    });
    added
}

/// What stops the timeline from saving. The DM must choose, so this cannot be dismissed.
pub fn problems(ui: &mut egui::Ui, book: &mut Book) {
    if let Some(count) = book.dropped()
        && design::message_with_action(
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
    if let Some(error) = book.unreadable.clone() {
        design::message(ui, Tone::Error, &error);
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
            "Timeline.md changed on disk while you were editing. Nothing is saved until you choose.",
        );
        ui.horizontal(|ui| {
            if Action::secondary("Overwrite with my timeline")
                .show(ui)
                .clicked()
            {
                book.overwrite();
            }
            if Action::quiet("Reload and lose my edits").show(ui).clicked() {
                book.reload_keeping_undo();
            }
        });
    } else if let Some(error) = book.error.clone() {
        design::message(ui, Tone::Error, &error);
        if Action::secondary("Try to save again").show(ui).clicked() {
            book.error = None;
            book.save();
        }
    }
}

/// The one step of undo after a removal. Returns whether it is showing.
pub fn undo_removed(ui: &mut egui::Ui, book: &mut Book) -> bool {
    let Some(removed) = book.removed().map(str::to_owned) else {
        return false;
    };
    if design::message_with_action(
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
        ui.horizontal(|ui| {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        if current.is_some() {
                            "New events and runs go after the last event of"
                        } else {
                            "New events and runs go at the end of the timeline, in"
                        },
                    )
                    .color(colors().muted),
            );
            let label = current
                .and_then(|id| sessions.iter().find(|session| session.id == id))
                .map_or("no session", |session| session.label.as_str());
            let mut chosen = current;
            let inner = egui::ComboBox::from_id_salt("timeline_new_events_session")
                .truncate()
                .selected_text(Type::Small.rich(ui, label).color(colors().text))
                .width(200.0)
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
        });
    }

    fn filtered(&self) -> bool {
        self.filter != Filter::All || !self.search.trim().is_empty() || self.session.is_some()
    }

    fn filters(&mut self, ui: &mut egui::Ui, sessions: &[SessionLink]) {
        let tabs = |ui: &mut egui::Ui, filter: &mut Filter| {
            design::segmented(
                ui,
                filter,
                &[
                    (Filter::All, "All"),
                    (Filter::Days, "Days"),
                    (Filter::Encounters, "Encounters"),
                    (Filter::Rests, "Rests"),
                    (Filter::Travel, "Travel"),
                    (Filter::Events, "Major"),
                ],
            );
        };
        let wide = ui.available_width() >= FILTERS_ON_ONE_ROW_FROM;
        if !wide {
            tabs(ui, &mut self.filter);
        }
        ui.horizontal(|ui| {
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
        });
    }

    fn row(&mut self, ui: &mut egui::Ui, event: &Event, day: u32, session: Option<&str>) {
        let c = colors();
        let selected = self.selected.as_deref() == Some(event.id.as_str());
        let title = event.label(day);
        let mut detail = event.summary(session);
        // Filters and scrolling hide the Day rows, so each row says its own.
        if event.kind != Kind::Day && day > 0 {
            detail = if detail.is_empty() {
                format!("Day {day}")
            } else {
                format!("Day {day} · {detail}")
            };
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
            if event.kind == Kind::Day {
                // A day opens a stretch of the list. It starts with a rule, as a heading does.
                painter.hline(
                    rect.x_range(),
                    rect.top() + 0.5,
                    egui::Stroke::new(design::HAIRLINE_WIDTH, c.rule),
                );
            } else {
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
                event.kind.icon(),
                Type::Name.font(ui),
                if event.kind == Kind::Day {
                    c.text
                } else {
                    c.muted
                },
            );
            let font = if event.kind == Kind::Day {
                Type::Heading
            } else if selected {
                Type::Strong
            } else {
                Type::Body
            };
            let galley = design::one_line(ui, &title, font.font(ui), c.text, width);
            let lines = if detail.is_empty() {
                galley.size().y
            } else {
                galley.size().y + Type::Small.line()
            };
            let top = rect.center().y - lines / 2.0;
            let x = rect.left() + TEXT_AT;
            painter.galley(egui::pos2(x, top), galley, c.text);
            if !detail.is_empty() {
                let second = design::one_line(ui, &detail, Type::Small.font(ui), c.muted, width);
                painter.galley(
                    egui::pos2(x, top + lines - Type::Small.line()),
                    second,
                    c.muted,
                );
            }
        });
        if response.clicked() {
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
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            let close = self
                .places
                .iter()
                .filter_map(|(_, names)| names.iter().find(|n| n.to_lowercase().contains(&typed)))
                .take(5);
            for name in close {
                if design::link(ui, name).clicked() {
                    event.place = name.clone();
                    changed = true;
                }
            }
        });
        changed
    }

    fn list(&mut self, ui: &mut egui::Ui, book: &Book, sessions: &[SessionLink]) {
        let days = day_numbers(&book.events);
        if let Some(now) = clock_now(&book.events) {
            ui.label(
                Type::Small
                    .rich(ui, format!("Now: {}", clock_words(now)))
                    .color(colors().muted),
            );
        }
        let rows: Vec<usize> = (0..book.events.len())
            .filter(|&index| self.matches(&book.events[index], days[index]))
            .collect();
        if rows.is_empty() {
            // An unreadable file has its own message, and no filter caused it.
            if book.unreadable.is_none() {
                ui.add_space(design::M);
                ui.label(
                    Type::Body
                        .rich(ui, "Nothing on the timeline matches this.")
                        .color(colors().muted),
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
                self.row(ui, event, days[index], session);
            }
        });
    }

    fn detail(&mut self, ui: &mut egui::Ui, book: &mut Book, sessions: &[SessionLink]) {
        let c = colors();
        undo_removed(ui, book);
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
            if design::message_with_action(
                ui,
                Tone::Success,
                &format!("{label} added."),
                "Undo add",
            ) {
                book.remove(&id);
                self.just_added = None;
                self.selected = None;
                return;
            }
        }
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
        let unsaved = book.unsaved();
        let mut changed = false;
        let mut removed = false;
        let mut moved = 0;
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
        egui::ScrollArea::vertical()
            .id_salt(("timeline_detail", &id))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let event = &mut book.events[index];
                ui.horizontal(|ui| {
                    ui.label(
                        Type::Heading
                            .rich(ui, format!("{} {}", event.kind.icon(), event.kind.word()))
                            .color(c.text),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (icon, word, color) = if unsaved {
                            (icons::DRAFT, "Unsaved", c.document.unsaved)
                        } else {
                            (icons::OK, "Saved", c.document.clean)
                        };
                        // The link is what this page is for, so it stays at the top.
                        if Action::secondary("Copy link")
                            .icon(icons::COPY)
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            ui.ctx().copy_text(link.clone());
                            self.copied = Some((id.clone(), Instant::now()));
                        }
                        self.copied = self
                            .copied
                            .take()
                            .filter(|(_, at)| at.elapsed() < COPIED_FOR);
                        if let Some((copied, _)) = &self.copied {
                            ui.ctx().request_repaint_after(COPIED_FOR);
                            if *copied == id {
                                ui.label(Type::Small.rich(ui, "Copied").color(c.document.saved));
                            }
                        }
                        design::status(ui, design::Status { icon, word, color });
                    });
                });
                ui.add_space(design::XS);
                let hint = (event.kind == Kind::Day)
                    .then_some("Optional. Days number themselves, so a name is extra.");
                let title = design::text_field(ui, "Title", &mut event.title, hint, None);
                changed |= title.changed();
                if std::mem::take(&mut self.focus_title) {
                    title.request_focus();
                }
                design::field(ui, "Kind", None, None, |ui| {
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
                    changed |= design::text_field(
                        ui,
                        "How long",
                        &mut event.duration,
                        Some("A number and a unit move the clock: 3 days, 8 hours, an hour."),
                        None,
                    )
                    .changed();
                }
                if let Some(start) = starts {
                    ui.label(
                        Type::Small
                            .rich(ui, format!("Starts: {}", clock_words(start)))
                            .color(c.muted),
                    );
                }
                changed |= design::text_field(ui, "Where", &mut event.place, None, None).changed();
                changed |= self.place_help(ui, event);
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
                    && Action::quiet("Open this session").show(ui).clicked()
                {
                    self.open_session = Some(session);
                }
                if !event.result.is_empty() {
                    design::field(ui, "From the run", None, None, |ui| {
                        ui.label(Type::Body.rich(ui, &event.result).color(c.text));
                    });
                }
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
                design::field(
                    ui,
                    "Link",
                    Some("Paste this in a session note or an Obsidian note. In Obsidian, it opens this event in Timeline.md."),
                    None,
                    |ui| {
                        ui.add(egui::Label::new(Type::Mono.rich(ui, &link).color(c.muted)).wrap());
                    },
                );
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
                // A swap with a hidden event would look like nothing happened.
                let hidden = self
                    .filtered()
                    .then_some("Show all events to move this one.");
                ui.horizontal_wrapped(|ui| {
                    if Action::quiet("Earlier")
                        .icon(icons::UP)
                        .name("Move earlier")
                        .blocked(hidden.or((index == 0).then_some("This is the first event.")))
                        .show(ui)
                        .clicked()
                    {
                        moved = -1;
                    }
                    if Action::quiet("Later")
                        .icon(icons::DOWN)
                        .name("Move later")
                        .blocked(
                            hidden
                                .or((index + 1 == days.len()).then_some("This is the last event.")),
                        )
                        .show(ui)
                        .clicked()
                    {
                        moved = 1;
                    }
                });
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
                // The open run would add it back, without the DM's words.
                let open_run = event.run.is_some() && event.run == self.open_run;
                removed = Action::quiet("Remove")
                    .name("Remove this event")
                    .blocked_if(open_run, "This run is open. Close it first.")
                    .show(ui)
                    .clicked();
            });
        if changed {
            book.touch();
        }
        if moved != 0 {
            book.move_by(&id, moved);
            self.scroll_to = book.events.iter().position(|event| event.id == id);
        }
        if let Some(after) = placed {
            book.place_after(&id, after.as_deref());
            self.scroll_to = book.events.iter().position(|event| event.id == id);
        }
        if removed {
            book.remove(&id);
            self.selected = None;
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        campaign_id: Uuid,
        campaign: &str,
        sessions: &[SessionLink],
        current: Option<Uuid>,
    ) {
        let page_width = ui.available_width();
        // Only a real change: the first show can already have an event to reveal.
        if self
            .campaign
            .is_some_and(|campaign| campaign != campaign_id)
        {
            *self = Self {
                open_run: self.open_run,
                ..Default::default()
            };
        }
        self.campaign = Some(campaign_id);
        design::page_header(
            ui,
            "Timeline",
            Some(&format!(
                "{campaign} · {}",
                design::count(book.events.len(), "event", "events")
            )),
            |_| {},
        );
        problems(ui, book);
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
            book.save();
        }
        if let Some(id) = quick_add(ui, book, current, &Kind::ALL, |_| {}) {
            self.reveal(&id, book);
            self.focus_title = true;
            self.just_added = Some((id, Instant::now()));
        }
        self.new_events_line(ui, sessions, current);
        if let Some(words) = self.dated.clone()
            && design::message_with_action(ui, Tone::Warning, &words, "Play this one")
        {
            self.play_current = true;
        }
        ui.add_space(design::XS);
        if book.events.is_empty() && book.unreadable.is_none() {
            let mut first = false;
            design::empty_state(
                ui,
                icons::TIMELINE,
                "Nothing on the timeline yet",
                "Add the first day above. Each encounter run appears here by itself, in the session it belongs to.",
                |ui| {
                    first = Action::primary("Start with day 1").show(ui).clicked();
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
            self.filters(ui, sessions);
        }
        ui.add_space(design::XS);
        // Beside Lookup or Notes at a small window, two columns would crush the list.
        if narrow {
            if event_open {
                if Action::quiet("Back to the list")
                    .icon(icons::BACK)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.selected = None;
                }
                self.detail(ui, book, sessions);
            } else {
                undo_removed(ui, book);
                self.list(ui, book, sessions);
            }
            return;
        }
        let width = design::side_panel_width(ui.available_width());
        design::Column::panel("timeline_detail_panel", width)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| self.detail(ui, book, sessions));
        egui::CentralPanel::default()
            // The panel's drag handle lies on this edge, so the rows stop short of it.
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| self.list(ui, book, sessions));
    }
}

/// The session's number alone. The row's own title is the event's.
fn short_session(label: &str) -> &str {
    label.split(':').next().unwrap_or(label)
}
