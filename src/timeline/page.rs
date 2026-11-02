//! The Timeline page: the whole campaign in story order, apart from any one session.
use super::{Book, Event, Kind, day_numbers, reference};
use crate::design::{self, Action, Density, Tone, Type, colors, icons};
use eframe::egui;
use uuid::Uuid;

const ROW: f32 = 44.0;
const SPINE: f32 = 16.0;
const TEXT_AT: f32 = 34.0;

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
    copied: Option<String>,
    focus_title: bool,
    scroll_to: Option<usize>,
    /// The DM asked to open this session's page.
    pub open_session: Option<Uuid>,
}

/// One button for each kind of event. Returns the new event's ID.
pub fn quick_add(
    ui: &mut egui::Ui,
    book: &mut Book,
    session: Option<Uuid>,
    after: Option<&str>,
) -> Option<String> {
    let blocked = book.unreadable.is_some();
    let mut added = None;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = design::XS;
        for kind in Kind::ALL {
            if Action::secondary(kind.word())
                .icon(kind.icon())
                .name(&format!("Add {}", kind.word()))
                .density(Density::Read)
                .blocked_if(blocked, "The timeline file must be repaired first.")
                .show(ui)
                .clicked()
            {
                added = Some(book.add(kind, session, after));
            }
        }
    });
    added
}

/// What stops the timeline from saving. The DM must choose, so this cannot be dismissed.
pub fn problems(ui: &mut egui::Ui, book: &mut Book) {
    if let Some(error) = book.unreadable.clone() {
        design::message(ui, Tone::Error, &error);
        if Action::secondary("Read the file again").show(ui).clicked() {
            book.reload();
        }
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
            if Action::quiet("Reload and lose my edit").show(ui).clicked() {
                book.reload();
            }
        });
    } else if let Some(error) = book.error.clone() {
        design::message(ui, Tone::Error, &error);
        if Action::secondary("Try to save again").show(ui).clicked() {
            book.error = None;
            book.save();
        }
    }
    let removed = book.removed().map(|event| event.label(0));
    if let Some(removed) = removed
        && design::message_with_action(
            ui,
            Tone::Info,
            &format!("Removed “{removed}”."),
            "Undo remove",
        )
    {
        book.undo_remove();
    }
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

    fn filters(&mut self, ui: &mut egui::Ui, sessions: &[SessionLink]) {
        ui.horizontal_wrapped(|ui| {
            design::segmented(
                ui,
                &mut self.filter,
                &[
                    (Filter::All, "All"),
                    (Filter::Days, "Days"),
                    (Filter::Encounters, "Encounters"),
                    (Filter::Rests, "Rests"),
                    (Filter::Travel, "Travel"),
                    (Filter::Events, "Events"),
                ],
            );
            design::text_input(ui, "Search the timeline", &mut self.search, "Search", 160.0);
            let label = self
                .session
                .and_then(|id| sessions.iter().find(|session| session.id == id))
                .map_or("All sessions", |session| session.label.as_str());
            let inner = egui::ComboBox::from_id_salt("timeline_session_filter")
                .selected_text(label)
                .width(150.0)
                .icon(design::combo_icon)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.session, None, "All sessions");
                    for session in sessions {
                        ui.selectable_value(&mut self.session, Some(session.id), &session.label);
                    }
                });
            design::named(ui, &inner.response, "Show one session");
        });
    }

    fn row(&mut self, ui: &mut egui::Ui, event: &Event, day: u32, session: Option<&str>) {
        let c = colors();
        let selected = self.selected.as_deref() == Some(event.id.as_str());
        let title = event.label(day);
        let detail = event.summary(session);
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

    fn list(&mut self, ui: &mut egui::Ui, book: &Book, sessions: &[SessionLink]) {
        let days = day_numbers(&book.events);
        let rows: Vec<usize> = (0..book.events.len())
            .filter(|&index| self.matches(&book.events[index], days[index]))
            .collect();
        if rows.is_empty() {
            ui.add_space(design::M);
            ui.label(
                Type::Body
                    .rich(ui, "Nothing on the timeline matches this.")
                    .color(colors().muted),
            );
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
                            "Select an event to name it, place it or note what it means. Each event has its own reference, which a session note or an Obsidian note can link to.",
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
        let mut changed = false;
        let mut removed = false;
        let mut moved = 0;
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
                        // The reference is what this page is for, so it stays at the top.
                        if Action::secondary("Copy reference")
                            .icon(icons::COPY)
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            ui.ctx().copy_text(link.clone());
                            self.copied = Some(id.clone());
                        }
                        if self.copied.as_deref() == Some(id.as_str()) {
                            ui.label(Type::Small.rich(ui, "Copied").color(c.document.saved));
                        }
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
                        .width(ui.available_width().min(design::FORM_WIDTH) - design::S)
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
                        Some("In the story, in your own words: 3 days, an hour."),
                        None,
                    )
                    .changed();
                }
                changed |= design::text_field(ui, "Where", &mut event.place, None, None).changed();
                design::field(ui, "Session", None, None, |ui| {
                    let label = event
                        .session
                        .and_then(|id| sessions.iter().find(|session| session.id == id))
                        .map_or("No session", |session| session.label.as_str());
                    let inner = egui::ComboBox::from_id_salt("timeline_event_session")
                        .selected_text(label)
                        .width(ui.available_width().min(design::FORM_WIDTH) - design::S)
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
                if let Some(session) = event.session
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
                    "Reference",
                    Some("Paste this in any note. Obsidian follows it here too."),
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
                ui.horizontal_wrapped(|ui| {
                    if Action::quiet("Earlier")
                        .icon(icons::UP)
                        .name("Move earlier")
                        .blocked_if(index == 0, "This is the first event.")
                        .show(ui)
                        .clicked()
                    {
                        moved = -1;
                    }
                    if Action::quiet("Later")
                        .icon(icons::DOWN)
                        .name("Move later")
                        .blocked_if(index + 1 == days.len(), "This is the last event.")
                        .show(ui)
                        .clicked()
                    {
                        moved = 1;
                    }
                    ui.add_space(design::L);
                    removed = Action::destructive("Remove")
                        .name("Remove this event")
                        .show(ui)
                        .clicked();
                });
            });
        if changed {
            book.touch();
        }
        if moved != 0 {
            book.move_by(&id, moved);
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
        campaign: &str,
        sessions: &[SessionLink],
        current: Option<Uuid>,
    ) {
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
        if let Some(id) = quick_add(ui, book, current, self.selected.as_deref()) {
            self.reveal(&id, book);
            self.focus_title = true;
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
                let id = book.add(Kind::Day, current, None);
                self.reveal(&id, book);
            }
            return;
        }
        self.filters(ui, sessions);
        ui.add_space(design::XS);
        let width = design::side_panel_width(ui.available_width());
        egui::SidePanel::right("timeline_detail_panel")
            .resizable(false)
            .exact_width(width)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::M as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| self.detail(ui, book, sessions));
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| self.list(ui, book, sessions));
    }
}

/// The session's number alone. The row's own title is the event's.
fn short_session(label: &str) -> &str {
    label.split(':').next().unwrap_or(label)
}
