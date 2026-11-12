//! The Timeline page: the whole campaign in story order, apart from any one session.
use super::{Book, Event, Kind, checks, clock, clock_now, clock_words, day_numbers, reference};
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
    /// One column: the detail hides the list, so the open event's results end its form.
    narrow: bool,
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
    /// The campaign's marked tables, for a journey's encounter checks.
    pub tables: Option<std::sync::Arc<crate::loot_editor::Sources>>,
    focus_length: bool,
    /// What the DM typed in a journey's checks, and each journey's last result with its Undo.
    checks: checks::State,
    checks_done: checks::Results,
}

/// One button for each of these kinds of event. Returns the new event's ID.
pub fn quick_add(
    ui: &mut egui::Ui,
    book: &mut Book,
    session: Option<Uuid>,
    kinds: &[Kind],
    first: impl FnOnce(&mut egui::Ui),
) -> Option<String> {
    quick_add_with(ui, book, session, kinds, Vec::new(), first).and_then(|chosen| chosen.ok())
}

/// `quick_add`, with lines of the strip's More menu alone, after the kinds. A new event's ID
/// comes back as `Ok`, and a chosen line of `more` as `Err` with its index there.
pub fn quick_add_with(
    ui: &mut egui::Ui,
    book: &mut Book,
    session: Option<Uuid>,
    kinds: &[Kind],
    more: Vec<Action<'_>>,
    first: impl FnOnce(&mut egui::Ui),
) -> Option<Result<String, usize>> {
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
    // Fight and Long rest stay visible; the rarer kinds share More when space is short.
    let always_visible: Vec<usize> = kinds
        .iter()
        .enumerate()
        .filter(|(_, kind)| matches!(kind, Kind::Encounter | Kind::LongRest))
        .map(|(index, _)| index)
        .collect();
    design::hanging_add_strip(ui, first, actions, &always_visible, more).map(|index| {
        let Some(&kind) = kinds.get(index) else {
            return Err(index - kinds.len());
        };
        Ok(if kind == Kind::LongRest {
            book.add_long_rest(session)
        } else {
            book.add(kind, session)
        })
    })
}

/// A pane's body between the cursor and `bottom`, the top of its message footer: an editor or
/// a list that fills its height stops above the footer.
pub(crate) fn body_above<R>(
    ui: &mut egui::Ui,
    bottom: f32,
    add: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let body = egui::Rect::from_min_max(
        ui.cursor().min,
        egui::pos2(ui.max_rect().right(), bottom.max(ui.cursor().top())),
    );
    ui.scope_builder(egui::UiBuilder::new().max_rect(body), add)
        .inner
}

/// What the DM chose in the Timeline list's message footer, done after the footer is drawn.
#[derive(Clone, Debug, PartialEq)]
enum ListAct {
    UndoLinks,
    UpdateLinks,
    UndoRuling,
    UndoRemove,
    UndoAdd(String),
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
    let mut action = rest_ruling_line(ui, event);
    if undo && let Some(undo) = rest_ruling_result(ui, event) {
        action = Some(undo);
    }
    action
}

/// The ruling now in force and the action that changes it. The action changes data, so it is a
/// quiet button after the status, never a link.
pub fn rest_ruling_line(ui: &mut egui::Ui, event: &Event) -> Option<RestRulingAction> {
    if event.kind != Kind::LongRest {
        return None;
    }
    let mut action = None;
    ui.horizontal_wrapped(|ui| {
        design::status(ui, long_rest_status(event.restores));
        ui.add_space(design::L - ui.spacing().item_spacing.x);
        if Action::quiet(if event.restores {
            "Mark not at full"
        } else {
            "Mark everyone at full"
        })
        .density(Density::Read)
        .show(ui)
        .clicked()
        {
            action = Some(RestRulingAction::Set(!event.restores));
        }
    });
    action
}

/// The ruling's result: a finished change, so it has the success tone in both directions. It
/// says the ruling now in force, and what it does to the next fight.
pub fn rest_ruling_result(ui: &mut egui::Ui, event: &Event) -> Option<RestRulingAction> {
    if event.kind != Kind::LongRest {
        return None;
    }
    let words = if event.restores {
        REST_RESTORES_WORDS
    } else {
        "The next fight starts from the HP the party has now."
    };
    design::undo_message(ui, Tone::Success, words, "Undo ruling").then_some(RestRulingAction::Undo)
}

/// Timeline's removal notice: the plan's words, for the removed event's own session, then how
/// many notes still link to it.
fn timeline_removal_words(book: &Book, label: &str, mentions: usize) -> String {
    let session = book.removed_event().and_then(|event| event.session);
    let days = day_numbers(&book.events);
    let mut words = super::removed_event_words(book, session, &days, label, &[]);
    if mentions > 0 {
        words.push_str(&format!(
            " {} still {} to it.",
            design::count(mentions, "note", "notes"),
            if mentions == 1 { "links" } else { "link" }
        ));
    }
    words
}

/// What a restoring rest does to the next fight. Adding a rest and marking it say the same.
const REST_RESTORES_WORDS: &str = "Everyone starts the next fight at full.";

/// The result of every Add: the event, then the session it went to when the DM added it inside
/// one (`Session 1`). A rest that restores HP then says what that does to the next fight, and
/// which saved fights keep their HP.
pub fn added_message(label: &str, session: Option<&str>, restores: Option<Vec<String>>) -> String {
    let to = session.map_or_else(String::new, |session| format!(" to {session}"));
    let mut words = format!("{label} added{to}.");
    if let Some(saved_fights) = restores {
        words.push_str(&format!(" {REST_RESTORES_WORDS}"));
        for name in saved_fights {
            words.push_str(&format!(" {name} keeps its saved HP."));
        }
    }
    words
}

/// A Timeline flag's words. The file line below the warning holds the path, so the words name
/// only the line.
fn flag_words(flag: &splat::content::layout::Flag) -> String {
    flag.message.clone()
}

/// The file's notices that block nothing: the reload result, each flag with the file line, a
/// duplicate, and a failed save. They arrive from outside edits and autosave, so pages draw
/// them with their other results (Timeline's list column), never above the Add strip or a form.
pub fn file_notices(ui: &mut egui::Ui, book: &mut Book) {
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
    // The warning names the line; the file line below the last one names the file, at its words.
    let folder = book.path.as_deref().and_then(std::path::Path::parent);
    for (at, flag) in book.flags.iter().enumerate() {
        let text = flag_words(flag);
        match folder {
            Some(folder) if at + 1 == book.flags.len() => {
                design::message_with_actions(ui, Tone::Warning, &text, |ui| {
                    crate::encounters::file_line(ui, folder, super::FILE);
                });
            }
            _ => {
                design::message(ui, Tone::Warning, &text);
            }
        }
    }
    if let Some(warning) = &book.duplicate {
        design::message(ui, Tone::Warning, warning);
    }
    if book.unreadable.is_none()
        && !book.conflict
        && let Some(error) = book.error.clone()
    {
        let again = design::message_with_actions(ui, Tone::Error, &error, |ui| {
            Action::secondary("Try to save again").show(ui).clicked()
        });
        if again {
            book.error = None;
            book.save();
        }
    }
}

/// What the DM chose in a file notice in a pane's footer, done after the footer is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileAct {
    UndoReload,
    SaveAgain,
}

/// Round 26 FL1: `file_notices` for a pane with a message footer. The same words and choices,
/// in the footer's order: the reload result with its live Undo, then the failed save (a
/// reason), then the flags and the duplicate (notices). The choice lands in `act`.
pub fn file_notices_in_footer<'a>(
    footer: &mut design::Footer<'a>,
    book: &Book,
    act: &'a std::cell::Cell<Option<FileAct>>,
) {
    if let Some(count) = book.dropped() {
        let words = format!(
            "Reloaded. Your version, with {}, is gone unless you undo.",
            design::count(count, "event", "events")
        );
        footer.result(words.clone(), true, move |ui| {
            if design::undo_message(ui, Tone::Info, &words, "Undo reload") {
                act.set(Some(FileAct::UndoReload));
            }
        });
    }
    let folder = book
        .path
        .as_deref()
        .and_then(std::path::Path::parent)
        .map(std::path::Path::to_path_buf);
    let last = book.flags.len();
    for (at, flag) in book.flags.iter().enumerate() {
        let text = flag_words(flag);
        let folder = folder.clone().filter(|_| at + 1 == last);
        footer.notice(text.clone(), move |ui| match folder {
            Some(folder) => {
                design::message_with_actions(ui, Tone::Warning, &text, |ui| {
                    crate::encounters::file_line(ui, &folder, super::FILE);
                });
            }
            None => {
                design::message(ui, Tone::Warning, &text);
            }
        });
    }
    if let Some(warning) = book.duplicate.clone() {
        footer.notice(warning.clone(), move |ui| {
            design::message(ui, Tone::Warning, &warning);
        });
    }
    if book.unreadable.is_none()
        && !book.conflict
        && let Some(error) = book.error.clone()
    {
        footer.reason(error.clone(), move |ui| {
            let again = design::message_with_actions(ui, Tone::Error, &error, |ui| {
                Action::secondary("Try to save again").show(ui).clicked()
            });
            if again {
                act.set(Some(FileAct::SaveAgain));
            }
        });
    }
}

/// Do what the DM chose in a footer's file notice.
pub fn file_act(book: &mut Book, act: Option<FileAct>) {
    match act {
        Some(FileAct::UndoReload) => book.undo_reload(),
        Some(FileAct::SaveAgain) => {
            book.error = None;
            book.save();
        }
        None => {}
    }
}

/// What stops the timeline from saving. The DM must choose, so this cannot be dismissed.
/// Returns true when the comparison needs the rest of the pane.
pub fn problems(ui: &mut egui::Ui, book: &mut Book) -> bool {
    if let Some(error) = book.unreadable.clone() {
        // Message choices start at the message text. The old-format notice draws its own.
        let choices = |ui: &mut egui::Ui, book: &mut Book| {
            ui.horizontal_wrapped(|ui| {
                if Action::secondary("Read the file again").show(ui).clicked() {
                    book.reload();
                }
                if book.gone
                    && Action::secondary("Save this timeline there again")
                        .show(ui)
                        .clicked()
                {
                    book.save_again();
                }
            });
        };
        design::message_with_actions(ui, Tone::Error, &error, |ui| choices(ui, book));
    } else if book.conflict {
        design::message_with_actions(
            ui,
            Tone::Warning,
            "Timeline.md changed on disk. Your draft is safe until you choose.",
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    if Action::secondary("Overwrite with my timeline")
                        .show(ui)
                        .clicked()
                    {
                        book.overwrite();
                    }
                    // One `L` gap before the red choice.
                    ui.add_space(design::L - ui.spacing().item_spacing.x);
                    if Action::destructive("Reload and lose my timeline")
                        .show(ui)
                        .clicked()
                    {
                        book.reload_keeping_undo();
                    }
                });
            },
        );
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
    }
    false
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
        ui.label(Type::Small.rich(ui, "Adds to").color(colors().muted));
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
        // The journeys' results and their Undo end when the DM leaves the page.
        self.checks_done.clear();
    }

    fn row(
        &mut self,
        ui: &mut egui::Ui,
        event: &Event,
        day: u32,
        session: Option<&str>,
        written_as: Option<u32>,
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
        // The ruling is a status, in its own colour, as on the plan row and in the detail.
        let mut ruling = None;
        if event.kind == Kind::LongRest {
            if !detail.is_empty() {
                detail.push_str(" · ");
            }
            let status = long_rest_status(event.restores);
            ruling = Some((detail.len(), status.color));
            detail.push_str(&format!("{} {}", status.icon, status.word));
        }
        // Round 26 FG4, FD7: a length the clock cannot count is a warning part, in its colour.
        let mut spans = Vec::new();
        let warning = event.time_warning();
        if let Some(warning) = &warning
            && let Some(start) = detail.find(warning.as_str())
        {
            spans.push((start..start + warning.len(), c.result.warning));
        }
        if let Some((start, color)) = ruling {
            spans.push((start..detail.len(), color));
        }
        // A day heading written out of order keeps its Day icon and says what it was written as.
        let flag = written_as.map(|day| format!("Written as Day {day}"));
        let mut name = if detail.is_empty() {
            title.clone()
        } else {
            format!("{title} · {detail}")
        };
        if let Some(flag) = &flag {
            name.push_str(&format!(" · {flag}"));
        }
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
                event.kind.icon(),
                Type::Name.font(ui),
                if event.kind == Kind::Day {
                    c.text
                } else {
                    c.muted
                },
            );
            // The warning icon and a muted word after the heading, in `Small`.
            let flag = flag.as_ref().map(|flag| {
                let mut job = egui::text::LayoutJob::default();
                for (text, color) in [
                    (icons::WARN.to_owned(), c.result.warning),
                    (format!(" {flag}"), c.muted),
                ] {
                    job.append(
                        &text,
                        0.0,
                        egui::TextFormat::simple(Type::Small.font(ui), color),
                    );
                }
                painter.layout_job(job)
            });
            let flag_room = flag.as_ref().map_or(0.0, |flag| flag.size().x + design::S);
            let galley = if event.kind == Kind::Day {
                let mut job = design::group_heading_text(ui, &title, 0);
                job.wrap.max_width = (width - flag_room).max(0.0);
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
            if let Some(flag) = flag {
                let at = egui::pos2(
                    x + galley.size().x + design::S,
                    top + (galley.size().y - flag.size().y) / 2.0,
                );
                painter.galley(at, flag, c.muted);
            }
            painter.galley(egui::pos2(x, top), galley, c.text);
            if !detail.is_empty() && !open {
                let second = second_line(ui, &detail, &spans, width);
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
        let response = if warning.is_some() {
            response.on_hover_text(format!(
                "{}. {}",
                super::TIME_NOT_COUNTED,
                super::TIME_REPAIR
            ))
        } else {
            response
        };
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

    /// Round 26: the list column's results, reasons and notices sit in its message footer,
    /// pinned below the rows, so a result never moves a row: the link update after a rename and
    /// its Undo, the open event's rest ruling, the file's notices, a removal and an add. Each
    /// keeps its owner and end event; the footer only places them. Returns where the rows end.
    fn list_footer(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        notes: &mut crate::sessions::page::Sessions,
        open: Option<&str>,
    ) -> f32 {
        let act = std::cell::Cell::new(None);
        let file = std::cell::Cell::new(None);
        let mut footer = design::Footer::new("timeline_list_footer", ui.available_width());
        if let Some(words) = notes.event_link_result() {
            let act = &act;
            footer.result(words.clone(), true, move |ui| {
                if design::undo_message(ui, Tone::Success, &words, "Undo link update") {
                    act.set(Some(ListAct::UndoLinks));
                }
            });
        }
        let offer = open
            .filter(|id| book.events.iter().any(|event| event.id == *id))
            .and_then(|id| notes.event_link_offer(book, id));
        if let Some(offer) = &offer {
            let (words, action) = (offer.words.clone(), offer.action.clone());
            let act = &act;
            footer.result(words.clone(), false, move |ui| {
                if design::message_with_action(ui, Tone::Info, &words, &action) {
                    act.set(Some(ListAct::UpdateLinks));
                }
            });
        }
        if let Some(problem) = notes.event_link_problem().map(str::to_owned) {
            footer.reason(problem.clone(), move |ui| {
                design::message(ui, Tone::Warning, &problem);
            });
        }
        if let Some(id) = open
            && let Some(event) = book
                .events
                .iter()
                .find(|event| event.id == id && book.rest_undo_id() == Some(id))
                .filter(|event| event.kind == Kind::LongRest)
        {
            let words = if event.restores {
                REST_RESTORES_WORDS
            } else {
                "The next fight starts from the HP the party has now."
            };
            let act = &act;
            footer.result(words, true, move |ui| {
                if design::undo_message(ui, Tone::Success, words, "Undo ruling") {
                    act.set(Some(ListAct::UndoRuling));
                }
            });
        }
        file_notices_in_footer(&mut footer, book, &file);
        if let Some(label) = book.removed().map(str::to_owned) {
            let mentions = self.removed_mentions;
            let words = timeline_removal_words(book, &label, mentions);
            // A finished change has the success tone; links left to it make it a warning.
            let tone = if mentions > 0 {
                Tone::Warning
            } else {
                Tone::Success
            };
            let act = &act;
            footer.result(words.clone(), true, move |ui| {
                if design::undo_message(ui, tone, &words, "Undo remove") {
                    act.set(Some(ListAct::UndoRemove));
                }
            });
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
            let restores = event
                .filter(|event| event.kind == Kind::LongRest && event.restores)
                .map(|event| {
                    super::fights_that_keep_hp(&book.events, event.session, &self.saved_runs)
                });
            // Timeline lists every session, so its result names only the event.
            let words = added_message(&label, None, restores);
            let act = &act;
            footer.result(words.clone(), true, move |ui| {
                if design::undo_message(ui, Tone::Success, &words, "Undo add") {
                    act.set(Some(ListAct::UndoAdd(id)));
                }
            });
        }
        if book.merged_outside {
            // A result like the others in this column: an icon and a bar, not help text.
            const MERGED: &str = "Timeline.md also changed outside Splat. Both changes are kept.";
            footer.notice(MERGED, |ui| {
                design::message(ui, Tone::Info, MERGED);
            });
        }
        let bottom = footer.show(ui);
        file_act(book, file.take());
        match act.take() {
            Some(ListAct::UndoLinks) => notes.undo_event_link_update(),
            Some(ListAct::UpdateLinks) => {
                if let Some(offer) = offer {
                    notes.accept_event_link_offer(offer, book);
                }
            }
            Some(ListAct::UndoRuling) => {
                if let Some(id) = open {
                    RestRulingAction::Undo.apply(book, id);
                    self.just_added = None;
                }
            }
            Some(ListAct::UndoRemove) => {
                book.undo_remove();
            }
            Some(ListAct::UndoAdd(id)) => {
                book.undo_add(&id);
                self.just_added = None;
                self.selected = None;
            }
            None => {}
        }
        bottom
    }

    /// The open event's results stay in the list column with the other results, so they never
    /// move the form: the link offer after a rename, and the rest ruling's result with its Undo.
    fn event_results(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        notes: &mut crate::sessions::page::Sessions,
        id: &str,
    ) {
        if !book.events.iter().any(|event| event.id == id) {
            return;
        }
        if let Some(offer) = notes.event_link_offer(book, id)
            && design::message_with_action(ui, Tone::Info, &offer.words, &offer.action)
        {
            notes.accept_event_link_offer(offer, book);
        }
        if let Some(problem) = notes.event_link_problem() {
            design::message(ui, Tone::Warning, problem);
        }
        let ruling = book
            .events
            .iter()
            .find(|event| event.id == id && book.rest_undo_id() == Some(id))
            .and_then(|event| rest_ruling_result(ui, event));
        if let Some(action) = ruling {
            action.apply(book, id);
            self.just_added = None;
        }
    }

    fn list(&mut self, ui: &mut egui::Ui, book: &mut Book, sessions: &[SessionLink]) {
        let flagged: std::collections::HashMap<String, u32> = book
            .baseline
            .as_deref()
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .map(super::formats::flagged_days)
            .unwrap_or_default()
            .into_iter()
            .collect();
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
                self.row(
                    ui,
                    event,
                    days[index],
                    session,
                    flagged.get(&event.id).copied(),
                );
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
        checks::settle_length(book, self.selected.as_deref());
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
        let sources = self.tables.clone();
        let tables = sources.as_ref().map_or(&[][..], |sources| &sources.tables);
        let mut checks_act = None;
        let mut undo_checks = false;
        let mut length_taken = checks::Taken::Nothing;
        self.checks_done.follow(book);
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
                if checks::offered(event, tables)
                    && Action::quiet("Roll for encounters").show(ui).clicked()
                {
                    checks_act = Some(checks::Act::Start);
                    ui.close();
                }
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
            let event = &mut book.events[index];
            let mut length = event.duration.clone();
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
                    Some(super::LENGTH_HELP),
                    None,
                    |ui| {
                        let hint = match event.kind {
                            Kind::LongRest => "8 hours",
                            Kind::ShortRest => "1 hour",
                            _ => "",
                        };
                        let focus = std::mem::take(&mut self.focus_length);
                        let (words, taken) =
                            checks::length_input(ui, &mut book.length, event, hint, focus);
                        length = words;
                        length_taken = taken;
                    },
                );
            }
            ruling = rest_ruling_line(ui, event);
            if let Some(start) = starts.filter(|_| event.kind != Kind::Day) {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        Type::Small
                            .rich(ui, format!("Starts: {}", clock_words(start)))
                            .color(c.muted),
                    );
                    // Round 26 FG4: a length the clock cannot count leaves Ends at Starts, so
                    // the warning takes Ends' place on the line that is always there.
                    if let Some(warning) = event.time_warning() {
                        ui.label(Type::Small.rich(ui, warning).color(c.result.warning))
                            .on_hover_text(super::TIME_REPAIR);
                    } else if event.kind != Kind::Day {
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
            // A journey's encounter checks, under its own **How long**.
            if checks::shown(event) {
                ui.add_space(design::M);
                let view = checks::View {
                    event,
                    start: starts,
                    length: &length,
                    tables,
                    planned: &|_| None,
                };
                checks_act = checks::header(ui, &view, &mut self.checks).or(checks_act.take());
                checks_act = checks::lines(ui, &view, &mut self.checks).or(checks_act.take());
                if let Some(done) = self.checks_done.get(&id) {
                    ui.add_space(design::S);
                    undo_checks |= design::undo_message(ui, Tone::Success, &done.words, done.undo);
                }
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
            // With the list hidden, the event's results end its form, so they move no field.
            if self.narrow {
                ui.add_space(design::M);
                file_notices(ui, book);
                self.event_results(ui, book, notes, &id);
            }
        });
        if changed {
            book.touch();
        }
        checks::note_taken(book, length_taken);
        if let Some(act) = checks_act {
            self.checks.clear_problem();
            if act == checks::Act::Start {
                self.focus_length = book.events[index].duration.trim().is_empty();
            }
            match checks::apply(book, &id, &act, tables) {
                Ok(Some(done)) => self.checks_done.keep(done),
                Ok(None) => {}
                Err(error) => self.checks.failed(&act, error),
            }
        }
        if undo_checks {
            self.checks_done.undo(&id, book);
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
                // The clock shows once a day sets it; before that, there is nothing to name.
                let now = clock_now(&book.events)
                    .map_or_else(String::new, |now| format!(" · Now: {}", clock_words(now)));
                format!(
                    "{}{now}",
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
            let bottom = self.list_footer(ui, book, notes, None);
            let mut first = false;
            let open_run = self.open_run.is_some();
            body_above(ui, bottom, |ui| {
                design::empty_state(
                    ui,
                    icons::TIMELINE,
                    "Nothing on the timeline yet",
                    "Add the first day above. Each encounter run appears here by itself, in the session it belongs to.",
                    |ui| {
                        first = if open_run {
                            Action::secondary("Start with day 1")
                        } else {
                            Action::primary("Start with day 1")
                        }
                        .show(ui)
                        .clicked();
                    },
                );
            });
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
        self.narrow = narrow;
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
                let bottom = self.list_footer(ui, book, notes, None);
                body_above(ui, bottom, |ui| self.list(ui, book, sessions));
            }
            return;
        }
        if !event_open {
            let bottom = self.list_footer(ui, book, notes, None);
            body_above(ui, bottom, |ui| self.list(ui, book, sessions));
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
                        let open = self.selected.clone();
                        let bottom = self.list_footer(ui, book, notes, open.as_deref());
                        body_above(ui, bottom, |ui| self.list(ui, book, sessions));
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

/// A row's second line on one line, muted, with the rest ruling from `at` in its own colour.
/// A row's second line: muted, with each status part (the rest ruling, a length the clock
/// cannot count) in its own colour. `spans` are byte ranges in `detail`, in order.
fn second_line(
    ui: &egui::Ui,
    detail: &str,
    spans: &[(std::ops::Range<usize>, egui::Color32)],
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::default();
    let mut at = 0;
    let mut append = |words: &str, color| {
        job.append(
            &words.replace(['\n', '\r'], " "),
            0.0,
            egui::TextFormat::simple(Type::Small.font(ui), color),
        );
    };
    for (range, color) in spans {
        let (start, end) = (
            range.start.max(at).min(detail.len()),
            range.end.min(detail.len()),
        );
        if start >= end {
            continue;
        }
        append(&detail[at..start], colors().muted);
        append(&detail[start..end], *color);
        at = end;
    }
    append(&detail[at..], colors().muted);
    job.wrap.max_width = width.max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    ui.fonts_mut(|fonts| fonts.layout_job(job))
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
                    TimelinePage::default().row(ui, &event, 0, None, None);
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
                    page.row(ui, &event, 1, None, None);
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
        // iter-23-S FL1: each direction says the ruling now in force.
        harness.get_by_label("The next fight starts from the HP the party has now.");
        harness.get_by_label("Mark everyone at full").click();
        harness.run_steps(2);
        assert!(harness.state().events[0].restores);
        harness.get_by_label(REST_RESTORES_WORDS);
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

    /// Row 7 of the plain evening's contract: a line of More alone moves no action of the strip.
    /// A strip that needs no More has none of those lines; one that has More lists them last.
    #[test]
    fn a_line_of_more_alone_moves_no_action() {
        use egui_kittest::kittest::Queryable;
        const KINDS: [Kind; 5] = [
            Kind::Day,
            Kind::LongRest,
            Kind::ShortRest,
            Kind::Travel,
            Kind::Event,
        ];
        let draw = |width: f32, scene: bool| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(width, 300.0))
                .build_state(
                    move |ctx, (book, chosen): &mut (Book, Option<Result<String, usize>>)| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            let more = if scene {
                                vec![
                                    Action::quiet("Scene")
                                        .icon(icons::CHECK_SQUARE)
                                        .name("Add Scene"),
                                ]
                            } else {
                                Vec::new()
                            };
                            if let Some(picked) =
                                quick_add_with(ui, book, None, &KINDS, more, |ui| {
                                    let _ = Action::quiet("Fight").icon(icons::ENCOUNTERS).show(ui);
                                })
                            {
                                *chosen = Some(picked);
                            }
                        });
                    },
                    (Book::default(), None),
                );
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            harness
        };
        let shown = |harness: &egui_kittest::Harness<'_, _>| {
            [
                "Add Day",
                "Add Long rest",
                "Add Short rest",
                "Add Travel",
                "Add Major event",
                "More Add actions",
                "Add Scene",
            ]
            .map(|name| harness.query_by_label(name).map(|node| node.rect()))
        };
        for width in [360.0, 1200.0] {
            let without = draw(width, false);
            let mut with = draw(width, true);
            assert_eq!(shown(&without), shown(&with), "at {width} px");
            let more = with.query_by_label("More Add actions").is_some();
            assert_eq!(more, width < 1000.0, "at {width} px");
            if more {
                with.get_by_label("More Add actions").click();
                with.run_steps(2);
                let scene = with.get_by_label("Add Scene").rect();
                for name in ["Add Day", "Add Short rest", "Add Travel", "Add Major event"] {
                    assert!(
                        with.get_by_label(name).rect().bottom() <= scene.top(),
                        "{name}"
                    );
                }
                with.get_by_label("Add Scene").click();
                with.run_steps(2);
                assert_eq!(with.state().1, Some(Err(0)));
                assert!(with.state().0.events.is_empty());
            }
        }
    }

    #[test]
    fn iter12_c_ft4_timeline_removal_names_the_fight_from_its_own_session() {
        let (here, elsewhere) = (Some(Uuid::new_v4()), Some(Uuid::new_v4()));
        let mut book = Book::default();
        let mut fight = |session, result: &str| {
            let id = book.add(Kind::Encounter, session);
            let event = book.events.iter_mut().find(|event| event.id == id).unwrap();
            event.title = "Bridge Ambush".to_owned();
            event.result = result.to_owned();
            event.run = Some(Uuid::new_v4());
            id
        };
        let paused = fight(here, "Round 2");
        fight(here, "Held the bridge, round 1");
        // A fight of the same name in another session is not "the other" one.
        let other = fight(elsewhere, "Round 3");
        book.remove(&paused);
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 0),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” and its result stay."
        );
        // The notes that still link to it stay last.
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 2),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” and its result \
             stay. 2 notes still link to it."
        );
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 1),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” and its result \
             stay. 1 note still links to it."
        );
        // The session comes from the removed event: the one in the other session stands alone.
        book.undo_remove();
        book.remove(&other);
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 0),
            "Removed “Bridge Ambush” (Round 3 · Paused)."
        );
    }

    /// An event with no session has no "other" event of its name: the sentence is left out.
    #[test]
    fn iter12_c2_removal_without_a_session_names_no_other_event() {
        let mut book = Book::default();
        let mut fight = |result: &str| {
            let id = book.add(Kind::Encounter, None);
            let event = book.events.iter_mut().find(|event| event.id == id).unwrap();
            event.title = "Bridge Ambush".to_owned();
            event.result = result.to_owned();
            event.run = Some(Uuid::new_v4());
            id
        };
        let paused = fight("Round 2");
        fight("Held the bridge, round 1");
        book.remove(&paused);
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 1),
            "Removed “Bridge Ambush” (Round 2 · Paused). 1 note still links to it."
        );
    }
}
