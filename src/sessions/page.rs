//! The Sessions page: notes, templates and recordings, one campaign at a time. The
//! recorder and the processing job live here, and keep going while the DM uses other pages.
use super::{
    DEFAULT_FOLDER, Draft, DraftState, NewNote, Note, Recording, Template, body_of, create_note,
    create_template, delete_recording, each_linked_name, folder as session_folder, folder_setting,
    fresh_line_in_note, insert_in_note, insert_in_plan, linked_names, list_notes, list_recordings,
    list_templates, nested_in_note, new_recording_path, parts_of, plain_links, recording_title,
    rename_links, rename_links_in_file, save_folder, section_text, unreadable_notes,
    update_details, without_empty_sections, without_own_heading,
};
use crate::{
    credentials::{self, KeySlot},
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    encounter_run::{FightSummary, fight_summary},
    encounters::PlanFight,
    lookup::{Written, WrittenIn},
    markdown_view,
    persistence::Campaign,
    processing::{self, Job, Request, settings::ProcessingSettings},
    recording::{Health, Recorder},
    timeline::{
        self, Book, Kind,
        page::{SessionLink, problems, quick_add, undo_removed},
    },
};
use eframe::egui::{self, Color32};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

const UNDO_ADD_FOR: Duration = Duration::from_secs(30);
const SAVED_WORD_FOR: Duration = Duration::from_secs(8);
const LIST_WIDTH: f32 = 216.0;
const LIST_WIDTHS: std::ops::RangeInclusive<f32> = 170.0..=420.0;
const STATUS_WIDTH: f32 = 128.0;
const TWO_COLUMNS_FROM: f32 = 640.0;
/// The plan and the note stand side by side from here.
const PLAN_BESIDE_NOTE_FROM: f32 = 720.0;
const PLAN_WIDTH: f32 = 330.0;
const PLAN_MAX_WIDTH: f32 = 480.0;
/// A list longer than this gets a search, and it scrolls instead of leaving the window.
const FIGHT_SEARCH_FROM: usize = 8;
const FIGHT_MENU_WIDTH: f32 = 240.0;
const FIGHT_MENU_HEIGHT: f32 = 240.0;
/// The kinds a session adds by one press. A fight has its own control, for what it can name.
const PLAN_KINDS: [Kind; 5] = [
    Kind::Day,
    Kind::LongRest,
    Kind::ShortRest,
    Kind::Travel,
    Kind::Event,
];
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);
const SCAN_EVERY: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Default, PartialEq)]
enum View {
    #[default]
    Notes,
    Templates,
}

#[derive(Clone, Copy, Default, PartialEq)]
enum Tab {
    #[default]
    Plan,
    /// In a narrow page, the note has its own tab. In a wide one, it stands beside the plan.
    Note,
    Recordings,
}

/// What the rest of the app tells this page about fights.
pub struct Plan<'a> {
    pub fights: &'a [PlanFight],
    pub groups: &'a [std::sync::Arc<crate::encounter_run::RunGroup>],
    /// Why no group can be assembled now.
    pub group_blocked: Option<&'a str>,
    /// A group being assembled takes the plan's place, beside the note.
    pub group_open: bool,
    /// A new group started in the plan was discarded there.
    pub group_discarded: bool,
    /// A fight is open that something can still join.
    pub arrivals_open: bool,
    pub group_editor: std::cell::RefCell<&'a mut dyn FnMut(&mut egui::Ui)>,
    /// Why no fight can start now.
    pub blocked: Option<&'a str>,
    pub run_directory: Option<PathBuf>,
    pub open_run: Option<Uuid>,
    pub open_state: String,
    pub open_encounter: Option<&'a str>,
}

/// What the previous session left, for the top of this one.
struct LastTime {
    id: Uuid,
    label: String,
    what_happened: Option<String>,
}

/// The new-session form, or the same fields for a session that exists.
struct Form {
    editing: Option<Uuid>,
    number: String,
    title: String,
    date: String,
    template: String,
    error: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum Made {
    Transcript,
    Marked,
    Review,
    Summary,
}

impl Made {
    fn word(self) -> &'static str {
        match self {
            Self::Transcript => "Raw transcript",
            Self::Marked => "Clean transcript",
            Self::Review => "What the check changed",
            Self::Summary => "Summary",
        }
    }

    fn path(self, recording: &Path) -> PathBuf {
        match self {
            Self::Transcript => processing::transcript_path(recording),
            Self::Marked => processing::marked_path(recording),
            Self::Review => processing::review_path(recording),
            Self::Summary => processing::summary_path(recording),
        }
    }
}

/// A text from a recording, read once, one part at a time.
struct Reading {
    recording: PathBuf,
    name: String,
    made: Made,
    parts: Vec<(String, String)>,
    part: usize,
    /// The words to bring into view, once, when a marker opened this.
    mark: Option<String>,
    /// The marker that opened this, with its key, so it can be added from here.
    marker: Option<(crate::timeline::Marker, String)>,
    /// A raw transcript that stops early: parts done, and parts in all.
    short_of: Option<(u64, u64)>,
}

/// An event just added, with its one step of undo. It goes at the next thing the DM does.
struct Added {
    id: String,
    label: String,
    at: Instant,
    count: usize,
}

/// Something failed where the DM may not be looking. It stays until they dismiss it.
struct Alert {
    word: &'static str,
    words: String,
    session: Uuid,
    /// The recording stopped, so one press can start it again.
    can_record: bool,
    retry: Option<PathBuf>,
}

struct Live {
    session: Uuid,
    label: String,
    recorder: Recorder,
}

struct LinkUndoState {
    notes: Vec<(PathBuf, String)>,
    events: Vec<timeline::Event>,
}

#[derive(Default)]
pub struct Sessions {
    pub run_open: bool,
    campaign: Option<Campaign>,
    folder: Option<PathBuf>,
    folder_error: Option<String>,
    folder_text: String,
    folder_notice: Option<String>,
    view: View,
    tab: Tab,
    /// The list of sessions stands beside this one, and it already gives the date.
    list_beside: bool,
    /// The note shows as a reader. A new or current session opens for writing.
    reading_note: bool,
    /// The event open in place in the plan.
    open_event: Option<String>,
    reveal_event: bool,
    open_planned: Option<String>,
    show_fight_log: bool,
    deleted_run: Option<crate::encounter_run::DeletedRun>,
    row_notice: Option<(Option<Uuid>, usize)>,
    updated_links: Option<Vec<(Draft, String)>>,
    link_undo_state: Option<LinkUndoState>,
    link_problem: Option<String>,
    removed_fight: Option<(Option<Uuid>, usize, String, String, String)>,
    plan_notice_state: Option<(String, Vec<timeline::Event>)>,
    removed_event_in_plan: bool,
    removed_group: Option<(Option<Uuid>, usize, String, String, f32)>,
    /// The fight behind the open event, read once.
    fight: Option<(Uuid, Result<FightSummary, String>)>,
    /// What the session before the selected one left, read once for each selection.
    last_time: Option<(Uuid, Option<LastTime>)>,
    notes: Vec<Note>,
    unreadable: Vec<String>,
    mentions: BTreeMap<Uuid, Vec<String>>,
    selected: Option<Uuid>,
    today: Option<String>,
    /// In a narrow page, the list stands in for the session.
    list_only: bool,
    templates_list_only: bool,
    /// Where new events and runs go, if the DM chose. Otherwise the newest session.
    events_to: Option<Option<Uuid>>,
    /// The session chosen when the app last closed, until its campaign opens.
    pub restored_session: Option<(Uuid, Option<Uuid>)>,
    fight_search: String,
    focus_fight_search: bool,
    draft: Option<Draft>,
    form: Option<Form>,
    templates: Vec<Template>,
    template: Option<String>,
    template_draft: Option<Draft>,
    new_template: Option<String>,
    template_error: Option<String>,
    deleted_template: Option<(String, String)>,
    recordings: Vec<Recording>,
    live: Option<Live>,
    confirm_record: bool,
    notice: Option<(Tone, String)>,
    /// The session a notice is about. None means whichever is open now.
    notice_session: Option<Uuid>,
    /// A job ended while the DM may be elsewhere.
    job_done: bool,
    /// It finished with something the DM must look at.
    job_warned: bool,
    alerts: Vec<Alert>,
    added: Option<Added>,
    job: Option<(Job, Uuid)>,
    reading: Option<Reading>,
    confirm_delete: Option<PathBuf>,
    confirm_again: Option<PathBuf>,
    pub notes_open: bool,
    focus_notes: bool,
    /// Back to the note, with its caret where it was.
    refocus_note: bool,
    focus_form_title: bool,
    /// The open run's name, and the session its event is in.
    pub open_run: Option<(String, Uuid)>,
    /// Words typed elsewhere that belong at the note's caret.
    words_for_note: String,
    words_new_line: bool,
    /// A step of the run came between. The next word typed starts a line of its own.
    break_before_words: bool,
    note_editor_id: Option<egui::Id>,
    move_run_here: Option<(Uuid, String)>,
    moved_run: Option<MovedRun>,
    /// When a recording last stopped, so the rail can say it was kept.
    saved_at: Option<Instant>,
    note_had_focus: bool,
    /// The caret to bring into view, on the frame after the editor has laid out.
    caret_to_show: Option<usize>,
    last_check: Option<Instant>,
    last_scan: Option<Instant>,
    /// The campaign's manifest changed, so what holds a copy of it must reload.
    pub manifest_changed: Option<PathBuf>,
    /// The Sessions page is showing, so a button there need not lead here.
    pub on_page: bool,
    pub wants_processing_settings: bool,
    pub wants_recording_input: bool,
    /// The DM asked to see the session this is about.
    pub wants_sessions_page: bool,
    pub wants_event: Option<String>,
    /// The encounter to start, by its path.
    pub wants_fight: Option<String>,
    pub wants_campaign_link: bool,
    pub wants_runs_refresh: bool,
    pub wants_new_fight: bool,
    pub wants_new_group: bool,
    pub wants_group_undo: bool,
    /// The session whose plan a group is being assembled in. It is the one that gets the link.
    group_session: Option<Uuid>,
    /// Notes that kept a group's old name, because they could not be written.
    rename_problem: Option<String>,
    /// The open fight's Arrivals panel, for a planned group that has come.
    pub wants_arrivals: bool,
    /// The path of a group to edit in the plan.
    pub wants_group: Option<String>,
    group_search: String,
    focus_group_search: bool,
    /// What tonight's note links, kept while another session's note is open.
    tonight_links: Option<(Uuid, Vec<String>)>,
    pub wants_back_to_run: bool,
    /// A saved run to open, to read in full.
    pub wants_saved_run: Option<PathBuf>,
    /// A name in a note's link to read in Lookup.
    pub wants_lookup: Option<String>,
}

struct MovedRun {
    before: timeline::Event,
    open_run: Option<(String, Uuid)>,
    body: String,
    events: Vec<timeline::Event>,
    words: String,
}

fn processing_llm(settings: &ProcessingSettings) -> String {
    let config = &settings.config;
    if config.provider == processing::Provider::CodexCli {
        format!("Codex ({})", config.llm_model.trim())
    } else {
        processing::host(&config.llm_url)
    }
}

fn processing_destination(settings: &ProcessingSettings) -> String {
    let config = &settings.config;
    let llm = processing_llm(settings);
    format!(
        "Process sends the audio and the session and campaign names to {} for a transcript. Then the transcript, this session's name, notes and events go to {llm} twice: once to clean them, and once to check that work against the raw words.",
        processing::host(&config.speech_url)
    )
}

pub fn processed_warning() -> Status<'static> {
    Status {
        icon: icons::WARN,
        word: "Processed, with a warning",
        color: colors().result.warning,
    }
}

/// Words that wrap at the width they are given.
fn plan_summary(summary: &str) -> String {
    summary
        .split(" · ")
        .map(|part| part.replace(' ', "\u{a0}"))
        .collect::<Vec<_>>()
        .join(" ·\u{a0}")
}

/// The recording count belongs to the tab's label and click target.
fn recordings_tab(ui: &mut egui::Ui, shown: &mut Tab, count: usize) -> bool {
    if count == 0 {
        return design::segmented(ui, shown, &[(Tab::Recordings, "Recordings")]);
    }
    let c = colors();
    let selected = *shown == Tab::Recordings;
    let font = if selected { Type::Strong } else { Type::Body }.font(ui);
    let color = if selected { c.text } else { c.muted };
    let mut job = egui::text::LayoutJob::default();
    job.append(
        "Recordings",
        0.0,
        egui::TextFormat::simple(font.clone(), color),
    );
    job.append(
        &format!(" {}", design::count(count, "", "")),
        0.0,
        egui::TextFormat::simple(font, c.muted),
    );
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(galley.size().x + 2.0 * design::S, Density::Read.control()),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!(
                "Recordings, {}",
                design::count(count, "recording", "recordings")
            ),
        )
    });
    if ui.is_rect_visible(rect) {
        if response.hovered() {
            ui.painter()
                .rect_filled(rect, design::RADIUS_CONTROL, c.hover);
        }
        ui.painter()
            .galley(rect.center() - galley.size() / 2.0, galley, color);
        if selected {
            let inset = design::XS / 2.0;
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.left() + inset, rect.bottom() - inset),
                    egui::pos2(rect.right() - inset, rect.bottom()),
                ),
                0.0,
                c.accent,
            );
        }
        if response.has_focus() {
            design::focus_ring(ui, rect);
        }
    }
    if response.clicked() && !selected {
        *shown = Tab::Recordings;
        return true;
    }
    false
}

fn wrapped_words(ui: &mut egui::Ui, kind: Type, words: impl Into<String>, color: Color32) {
    ui.add(egui::Label::new(kind.rich(ui, words).color(color)).wrap());
}

/// Keep the note's words while removing the group link that adds it to the plan.
fn unlink_group(body: &mut String, name: &str) {
    let mut out = String::new();
    let mut rest = body.as_str();
    while let Some(start) = rest.find("[[") {
        out.push_str(&rest[..start]);
        rest = &rest[start..];
        let Some(end) = rest.find("]]") else {
            break;
        };
        let link = &rest[..end + 2];
        let inner = &rest[2..end];
        if inner
            .split('|')
            .next()
            .is_some_and(|target| target.trim().eq_ignore_ascii_case(name))
        {
            out.push_str(inner.rsplit('|').next().unwrap_or(inner));
        } else {
            out.push_str(link);
        }
        rest = &rest[end + 2..];
    }
    out.push_str(rest);
    *body = out;
}

fn unlink_planned_fight(body: &str, name: &str, occurrence: usize) -> String {
    let mut in_plan = false;
    let mut found = 0;
    let mut removed = false;
    body.split_inclusive('\n')
        .filter_map(|line| {
            let heading = line.trim();
            if heading.starts_with("## ") {
                in_plan = heading.eq_ignore_ascii_case("## Plan for tonight");
            }
            if !in_plan || removed {
                return Some(line.to_owned());
            }
            let mut start = 0;
            while let Some(at) = line[start..].find("[[") {
                let at = start + at;
                let Some(end) = line[at..].find("]]").map(|end| at + end + 2) else {
                    break;
                };
                let target = &line[at + 2..end - 2];
                if target
                    .split('|')
                    .next()
                    .is_some_and(|target| target.trim().eq_ignore_ascii_case(name))
                {
                    if found == occurrence {
                        removed = true;
                        let prefix = line[..at].trim();
                        if matches!(prefix, "" | "-" | "*") && line[end..].trim().is_empty() {
                            return None;
                        }
                        return Some(format!(
                            "{}{}{}",
                            &line[..at],
                            target.rsplit('|').next().unwrap_or(target),
                            &line[end..]
                        ));
                    }
                    found += 1;
                }
                start = end;
            }
            Some(line.to_owned())
        })
        .collect()
}

/// One plan row: an icon, a compact text column, and actions centred beside both lines.
fn plan_row(
    ui: &mut egui::Ui,
    widths: &[f32],
    icon: &str,
    mut words: impl FnMut(&mut egui::Ui),
    mut summary: impl FnMut(&mut egui::Ui),
    mut action: impl FnMut(&mut egui::Ui),
) {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = design::S;
        ui.label(Type::Body.rich(ui, icon).color(colors().muted));
        ui.vertical(|ui| {
            let left = ui.cursor().min;
            let title_width = (ui.available_width() - widths[1] - design::S).max(design::XL);
            let text = ui.allocate_ui_with_layout(
                egui::vec2(title_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(title_width);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    words(ui);
                    summary(ui);
                },
            );
            let action_rect = egui::Rect::from_min_size(
                egui::pos2(ui.max_rect().right() - widths[1], left.y),
                egui::vec2(
                    widths[1],
                    text.response.rect.height().max(Density::Read.control()),
                ),
            );
            let mut action_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(action_rect)
                    .layout(egui::Layout::right_to_left(egui::Align::Center)),
            );
            action(&mut action_ui);
        });
    });
}

/// A paused run needs its state here; the whole saved-run record stays in Encounters.
fn plan_saved_status(saved: &crate::encounter_run::SavedRunInfo) -> String {
    crate::encounters::run_state_word(saved, true)
}

fn plan_link(ui: &mut egui::Ui, label: &str, open: bool) -> egui::Response {
    let response = design::wrapped_link(ui, label);
    design::named(ui, &response, label);
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_expanded(open));
    response
}

fn plan_details(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal_top(|ui| {
        ui.add_space(Type::Body.size() + design::S);
        ui.vertical(add);
    });
}

fn plan_rule(ui: &mut egui::Ui) {
    ui.add_space(design::S);
    design::rule(ui);
}

/// A title and its date wrap together, and expose the full title to assistive tools.
fn note_title(ui: &mut egui::Ui, title: &str, date: &str) {
    let mut job = egui::text::LayoutJob::default();
    job.append(
        title,
        0.0,
        egui::TextFormat {
            font_id: Type::Title.font(ui),
            color: colors().text,
            ..Default::default()
        },
    );
    job.append(
        &format!("  {date}"),
        0.0,
        egui::TextFormat {
            font_id: Type::Small.font(ui),
            color: colors().muted,
            ..Default::default()
        },
    );
    let response = ui.add(egui::Label::new(job).wrap());
    design::named(ui, &response, &format!("{title} · {date}"));
}

/// A played fight names its result and last round once, on the row's second line.
fn played_fight_summary(event: &timeline::Event) -> String {
    let state = |round: &str, paused, ended| {
        round
            .parse()
            .ok()
            .map(|round| crate::encounter_run::state_words(round, paused, ended))
    };
    if let Some((outcome, round)) = event.result.rsplit_once(", round ") {
        state(round, false, true).map(|words| format!("{outcome} · {words}"))
    } else if let Some(round) = event.result.strip_prefix("Ended in round ") {
        state(round, false, true)
    } else if let Some(round) = event.result.strip_prefix("Round ") {
        state(round, true, false)
    } else {
        None
    }
    .unwrap_or_else(|| event.result.clone())
}

/// The words of one of the plan's add menus.
struct PlanMenu<'a> {
    word: &'a str,
    icon: &'a str,
    name: &'a str,
    new: &'a str,
    blocked: Option<&'a str>,
    find: &'a str,
    hint: &'a str,
}

enum MenuChoice {
    New,
    Named(String),
}

/// A button whose menu offers a new thing, then the campaign's, with a search when they are many.
fn plan_menu(
    ui: &mut egui::Ui,
    menu: &PlanMenu<'_>,
    names: &[&str],
    search: &mut String,
    focus: &mut bool,
) -> Option<MenuChoice> {
    let response = Action::quiet(menu.word)
        .icon(menu.icon)
        .name(menu.name)
        .density(Density::Read)
        .show(ui);
    if response.clicked() {
        *focus = true;
    }
    let mut choice = None;
    egui::Popup::menu(&response)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            // A fixed place, whatever the list's length.
            if Action::quiet(menu.new)
                .icon(icons::ADD)
                .density(Density::Read)
                .blocked(menu.blocked)
                .show(ui)
                .clicked()
            {
                choice = Some(MenuChoice::New);
                ui.close();
            }
            if let Some(reason) = menu.blocked {
                wrapped_words(ui, Type::Small, reason, colors().muted);
            }
            if names.len() > FIGHT_SEARCH_FROM {
                let field =
                    design::search_field(ui, menu.find, search, menu.hint, FIGHT_MENU_WIDTH);
                if std::mem::take(focus) {
                    field.request_focus();
                }
            }
            ui.separator();
            let query = search.trim().to_lowercase();
            let mut chosen = None;
            egui::ScrollArea::vertical()
                .id_salt(menu.word)
                .max_height(FIGHT_MENU_HEIGHT)
                .show(ui, |ui| {
                    for name in names {
                        if name.to_lowercase().contains(&query) {
                            let row = ui.selectable_label(false, *name);
                            design::named(ui, &row, &format!("Plan {name}"));
                            if row.clicked() {
                                chosen = Some((*name).to_owned());
                            }
                        }
                    }
                });
            if let Some(name) = chosen {
                choice = Some(MenuChoice::Named(name));
                search.clear();
                ui.close();
            }
        });
    choice
}

/// One column of the wide page, the full height of the page.
fn page_column(ui: &mut egui::Ui, width: f32, height: f32, add: impl FnOnce(&mut egui::Ui)) {
    ui.allocate_ui_with_layout(
        egui::vec2(width, height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_width(width);
            ui.set_min_height(height);
            add(ui);
        },
    );
}

/// A note's words as a reader sees them: links as words, and no empty headings.
fn words_to_read(body: &str, book: &Book) -> String {
    let _ = book;
    without_empty_sections(&crate::world::with_links(body, &Default::default()))
}

fn updated_event_links(body: &str, book: &Book) -> (String, usize) {
    let days = timeline::day_numbers(&book.events);
    let mut out = String::new();
    let mut copied = 0;
    let mut count = 0;
    timeline::each_reference(body, |start, end, id, label| {
        out.push_str(&body[copied..start]);
        if let Some((at, event)) = book
            .events
            .iter()
            .enumerate()
            .find(|(_, event)| event.id == id)
            && let Some(label) = label
            && book
                .renamed_links
                .get(id)
                .is_some_and(|old| old.iter().any(|old| old == label))
            && label != timeline::reference_label(event, days[at])
        {
            out.push_str(&timeline::reference(event, days[at]));
            count += 1;
        } else {
            out.push_str(&body[start..end]);
        }
        copied = end;
    });
    out.push_str(&body[copied..]);
    (out, count)
}

/// A recap uses prose, never a line made only of links.
fn recap_preview(body: &str) -> Option<String> {
    for heading in ["Recap of last time", "What happened"] {
        if let Some(section) = section_text(body, heading) {
            let lines: Vec<_> = section
                .lines()
                .filter(|line| {
                    let mut prose = line.trim().trim_start_matches(['-', '*', ' ']).to_owned();
                    while let Some(start) = prose.find("[[") {
                        let Some(end) = prose[start..].find("]]") else {
                            break;
                        };
                        prose.replace_range(start..start + end + 2, "");
                    }
                    !prose.trim().is_empty()
                })
                .collect();
            if !lines.is_empty() {
                return Some(plain_links(&lines.join("\n")));
            }
        }
    }
    None
}

/// Keep a note's plan links in its file, but avoid listing the same fights twice in Read.
fn compact_plan_for_reading(body: &str, fights: &[PlanFight], beside: bool) -> String {
    let Some(plan) = section_text(body, "Plan for tonight") else {
        return body.to_owned();
    };
    let lines: Vec<_> = plan
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let only_fights = !lines.is_empty()
        && lines.iter().all(|line| {
            let link = line
                .trim()
                .strip_prefix("- ")
                .or_else(|| line.trim().strip_prefix("* "));
            link.and_then(|link| {
                link.strip_prefix("[[")
                    .and_then(|link| link.strip_suffix("]]"))
            })
            .is_some_and(|name| {
                fights.iter().any(|fight| {
                    fight
                        .name
                        .eq_ignore_ascii_case(name.split('|').next().unwrap_or(name))
                })
            })
        });
    if !only_fights {
        return body.to_owned();
    }
    let words = format!(
        "{}, in the plan",
        design::count(lines.len(), "fight", "fights")
    );
    let heading = body
        .split_inclusive('\n')
        .scan(0, |offset, line| {
            let at = *offset;
            *offset += line.len();
            Some((at, line))
        })
        .find(|(_, line)| line.trim_end() == "## Plan for tonight")
        .map_or(0, |(at, _)| at);
    let start = heading + body[heading..].find(&plan).expect("the plan section");
    let mut reading = body.to_owned();
    if beside {
        reading.replace_range(heading..start + plan.len(), "");
    } else {
        reading.replace_range(start..start + plan.len(), &words);
    }
    reading
}

fn has_written_text(body: &str) -> bool {
    body.lines().any(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with('#') && !matches!(line, "-" | "*" | "---")
    })
}

/// The fields of one event, in place in a session's plan. Returns whether any changed.
fn event_fields(ui: &mut egui::Ui, event: &mut timeline::Event) -> bool {
    let title = design::text_field(ui, "Title", &mut event.title, None, None);
    let mut changed = title.changed();
    design::field(ui, "Notes", None, None, |ui| {
        let response = ui.add(
            egui::TextEdit::multiline(&mut event.notes)
                .desired_width(ui.available_width() - 2.0 * design::S)
                .desired_rows(2)
                .margin(egui::vec2(design::S, design::S)),
        );
        design::named(ui, &response, "Notes on this event");
        changed |= response.changed();
    });
    changed
}

/// Read and a transcript use the width of `Write`'s text, so a note keeps its line breaks
/// between the views. The DM's own notes follow the column, unlike the books' rules prose.
fn note_width<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width() - 2.0 * design::S);
        add(ui)
    })
    .inner
}

/// The words for a note that looks like last week's. A forgotten `New session` puts
/// tonight's notes and audio there, and neither has a way to move afterward.
fn dated_before_today(note: &Note, today: &str) -> Option<String> {
    (note.date.as_str() < today).then(|| {
        format!(
            "This session is dated {}. Play it tonight, or start a new one.",
            note.date
        )
    })
}

/// Words at the note's caret. For a new thought after the end of a line of words, they start
/// a line of their own, and with no words only that break goes in.
fn insert_at_caret(
    ctx: &egui::Context,
    id: egui::Id,
    draft: &mut Draft,
    words: &str,
    new_line: bool,
) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    let Some(range) = state.cursor.char_range() else {
        if !words.is_empty() {
            insert_in_note(&mut draft.body, words);
            draft.edited();
        }
        return;
    };
    let at = range.primary.index;
    let chars: Vec<char> = draft.body.chars().collect();
    let ends_words = at > 0
        && chars.get(at - 1).is_some_and(|c| *c != '\n')
        && chars.get(at).is_none_or(|c| *c == '\n');
    let added = if new_line && ends_words {
        format!("\n\n{words}")
    } else {
        words.to_owned()
    };
    if added.is_empty() {
        return;
    }
    let byte = draft
        .body
        .char_indices()
        .nth(at)
        .map_or(draft.body.len(), |(byte, _)| byte);
    draft.body.insert_str(byte, &added);
    draft.edited();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::one(
            egui::text::CCursor::new(at + added.chars().count()),
        )));
    egui::TextEdit::store_state(ctx, id, state);
}

/// A list row whose title wraps, because a session's title is often long.
fn session_row(
    ui: &mut egui::Ui,
    selected: bool,
    title: &str,
    detail: &str,
    state: Option<Status<'_>>,
) -> egui::Response {
    let c = colors();
    let width = ui.available_width() - 2.0 * design::S;
    let font = if selected { Type::Strong } else { Type::Body };
    let galley = ui
        .painter()
        .layout(title.to_owned(), font.font(ui), c.text, width);
    let detail = detail.replace(
        &format!("{} Playing", icons::RUN),
        &format!("{}\u{a0}Playing", icons::RUN),
    );
    let word = state.map(|state| (format!("{} {}", state.icon, state.word), state.color));
    let second = design::status_in_text(
        ui,
        &detail,
        word.as_ref().map(|(word, color)| (word.as_str(), *color)),
        width,
    );
    let lines = galley.size().y + second.size().y;
    let height = (lines + design::M).max(Density::Glance.control() + design::S);
    design::row(
        ui,
        height,
        selected,
        &format!("{title} · {detail}"),
        |ui, rect| {
            let top = rect.center().y - lines / 2.0;
            let x = rect.left() + design::S;
            let first = galley.size().y;
            ui.painter().galley(egui::pos2(x, top), galley, c.text);
            ui.painter()
                .galley(egui::pos2(x, top + first), second, c.muted);
        },
    )
}

fn recording_detail(words: &str) -> String {
    if let Some(cancelled) = words.strip_prefix("Processing cancelled: ") {
        if let Some(tail) = cancelled.strip_prefix("Transcribing part ") {
            let part = tail.split('.').next().unwrap_or(tail);
            return format!("Cancelled at part {part}. Continue processing goes on from there.");
        }
        let step = cancelled.split(". Cancelled.").next().unwrap_or(cancelled);
        return format!("Cancelled during {step}. Continue processing goes on from there.");
    }
    if words.starts_with("Stopped by itself:") {
        return format!("{}. Check the microphone in", words.trim_end_matches('.'));
    }
    let detail = words.strip_prefix("Failed: ").unwrap_or(words);
    if let Some((step, rest)) = detail.split_once(", ")
        && let Some((elapsed, error)) = rest.split_once(". ")
        && elapsed.ends_with(" s")
    {
        return format!(
            "Stopped at {}, after {elapsed}. {error}",
            step.replacen(": ", ", ", 1).to_lowercase()
        );
    }
    detail.to_owned()
}

fn clock(seconds: u64) -> String {
    if seconds < 3600 {
        format!("{}:{:02}", seconds / 60, seconds % 60)
    } else {
        format!(
            "{}:{:02}:{:02}",
            seconds / 3600,
            (seconds / 60) % 60,
            seconds % 60
        )
    }
}

/// A length that cannot be read as a time of day.
fn spoken_length(seconds: u64) -> String {
    match (seconds / 3600, (seconds / 60) % 60) {
        (0, 0) => format!("{seconds} s"),
        (0, minutes) => format!("{minutes} min"),
        (hours, minutes) => format!("{hours} h {minutes} min"),
    }
}

fn megabytes(bytes: u64) -> String {
    format!("{:.0} MB", bytes as f64 / 1_000_000.0)
}

impl Sessions {
    /// Follow the active campaign, watch the recorder and the job, and save what is due.
    pub fn poll(&mut self, ctx: &egui::Context, active: Option<&Campaign>, book: &mut Book) {
        if self.campaign.as_ref().map(|c| (c.id, &c.path)) != active.map(|c| (c.id, &c.path)) {
            self.open_campaign(active);
        }
        book.open(self.folder.as_deref());
        if self.just_saved() {
            ctx.request_repaint_after(SAVED_WORD_FOR);
        }
        if let Some(live) = &mut self.live {
            live.recorder.poll();
            if let Health::Stopped(words) = live.recorder.health().clone() {
                let label = live.label.clone();
                let session = live.session;
                let kept = if self.stop_recorder(false) {
                    "What it captured is kept."
                } else {
                    "Nothing was captured."
                };
                self.raise(Alert {
                    session,
                    can_record: true,
                    retry: None,
                    word: "Recording stopped",
                    words: format!(
                        "The recording of {label} stopped by itself. {kept} Check the microphone, then record again. The recorder said: {}.", words.trim_end_matches(['.', '!', '?'])
                    ),
                });
            } else {
                ctx.request_repaint_after(Duration::from_secs(1));
            }
        }
        if let Some((job, session)) = &mut self.job {
            let session = *session;
            if let Some(result) = job.poll() {
                self.finish_job(result, session);
            } else {
                ctx.request_repaint_after(Duration::from_millis(250));
            }
        }
        for draft in [&mut self.draft, &mut self.template_draft]
            .into_iter()
            .flatten()
        {
            if draft.state == DraftState::Unsaved {
                #[cfg(not(test))]
                if draft.due_at(Instant::now()) {
                    draft.save();
                }
                ctx.request_repaint_after(Duration::from_millis(500));
            }
        }
        let now = Instant::now();
        if self
            .last_check
            .is_none_or(|at| now.duration_since(at) >= CHECK_DISK_EVERY)
        {
            self.last_check = Some(now);
            // The live file's length and size change, and it can appear after the first look.
            if self.live.as_ref().map(|live| live.session) == self.selected
                && self.selected.is_some()
            {
                self.refresh_recordings();
            }
            for draft in [&mut self.draft, &mut self.template_draft]
                .into_iter()
                .flatten()
            {
                draft.follow_disk();
            }
        }
        if let Some(id) = self.draft.as_mut().and_then(|draft| draft.restored.take()) {
            self.refresh();
            self.selected = Some(id);
            self.refresh_recordings();
        }
        // Notes can be renamed or moved in Obsidian, so the list follows the folder.
        if self
            .last_scan
            .is_none_or(|at| now.duration_since(at) >= SCAN_EVERY)
        {
            self.last_scan = Some(now);
            self.scan_folder();
        }
        book.session_links = self
            .notes
            .iter()
            .map(|note| (note.id, note.link()))
            .collect();
        book.poll(ctx);
    }

    /// What cannot wait, on every page. It sits at the bottom, so that it never moves `Next
    /// turn` under a pointer parked on it. A silence takes no banner: it is often only a quiet
    /// table, and it shows in the rail and on the run's own line.
    pub fn banner(
        &mut self,
        ctx: &egui::Context,
        settings: &ProcessingSettings,
        book: &Book,
        run_open: bool,
    ) {
        let mut stalled = self
            .live
            .as_ref()
            .filter(|live| matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_)))
            .map(|live| {
                format!(
                    "{}: not saving audio after {}. What was saved is kept. Record again starts a new file.",
                    live.label,
                    spoken_length(live.recorder.seconds())
                )
            });
        let row_here = |session| {
            self.on_page
                && self.selected == Some(session)
                && self.tab == Tab::Recordings
                && self.view == View::Notes
                && self.form.is_none()
                && (self.list_beside || !self.list_only)
                && self.reading.is_none()
        };
        if self
            .live
            .as_ref()
            .is_some_and(|live| row_here(live.session))
        {
            stalled = None;
        }
        if stalled.is_none() && self.alerts.iter().all(|alert| row_here(alert.session)) {
            return;
        }
        let mut retry = None;
        let mut record_again = None;
        let mut open = None;
        let mut stop = false;
        egui::TopBottomPanel::bottom("session_banner")
            .frame(
                egui::Frame::new()
                    .fill(colors().raised)
                    .inner_margin(egui::Margin::symmetric(design::M as i8, design::XS as i8)),
            )
            .show(ctx, |ui| {
                if let Some(words) = &stalled {
                    let session = self.live.as_ref().map(|live| live.session);
                    design::message(ui, Tone::Error, words);
                    ui.horizontal(|ui| {
                        if (if run_open {
                            Action::secondary("Record again")
                        } else {
                            Action::primary("Record again")
                        })
                        .show(ui)
                        .clicked()
                        {
                            record_again = session;
                        }
                        let here = self.on_page && self.selected == session;
                        if !here && Action::quiet("Open the session").show(ui).clicked() {
                            open = session;
                        }
                        ui.add_space(design::L);
                        stop = Action::quiet("Stop recording")
                            .name("Stop recording")
                            .show(ui)
                            .clicked();
                    });
                }
                let mut dismiss = None;
                for (index, alert) in self
                    .alerts
                    .iter()
                    .enumerate()
                    .filter(|(_, alert)| !row_here(alert.session))
                {
                    design::message(ui, Tone::Error, &alert.words);
                    ui.horizontal_wrapped(|ui| {
                        let missing_input = alert.can_record
                            && (settings.input_missing()
                                || alert.words.to_lowercase().contains("no such device"));
                        if missing_input
                            && (if run_open {
                                Action::secondary("Choose an input")
                            } else {
                                Action::primary("Choose an input")
                            })
                            .show(ui)
                            .clicked()
                        {
                            self.wants_recording_input = true;
                        }
                        if let Some(path) = &alert.retry
                            && (if run_open {
                                Action::secondary("Continue processing")
                            } else {
                                Action::primary("Continue processing")
                            })
                            .blocked_if(
                                self.job.is_some() || self.live.is_some(),
                                "Stop recording or wait for processing to finish.",
                            )
                            .show(ui)
                            .clicked()
                        {
                            retry = Some((alert.session, path.clone()));
                        }
                        if alert.can_record
                            && self.live.is_none()
                            && (if missing_input {
                                Action::quiet("Record again")
                            } else if run_open {
                                Action::secondary("Record again")
                            } else {
                                Action::primary("Record again")
                            })
                            .show(ui)
                            .clicked()
                        {
                            record_again = Some(alert.session);
                        }
                        let here = self.on_page
                            && self.selected == Some(alert.session)
                            && self.tab == Tab::Recordings;
                        if !here && Action::quiet("Open the session").show(ui).clicked() {
                            open = Some(alert.session);
                        }
                        ui.add_space(design::L);
                        if Action::quiet("Dismiss this error").show(ui).clicked() {
                            dismiss = Some(index);
                        }
                    });
                }
                if let Some(index) = dismiss {
                    self.alerts.remove(index);
                }
            });
        if let Some((session, path)) = retry {
            self.process_recording(ctx, &path, settings, book, session);
        }
        if stop {
            self.finish_recording();
        }
        if let Some(session) = record_again {
            self.finish_recording();
            if Some(session) != self.selected {
                self.select(Some(session));
            }
            // Pressed where the notice cannot show, so a failure stays in the banner.
            if let Err(error) = self.start_recording(settings, session) {
                self.raise(Alert {
                    word: "Recording stopped",
                    words: format!("The recording did not start again. {error}"),
                    session,
                    can_record: true,
                    retry: None,
                });
            }
        }
        if let Some(session) = open {
            self.wants_sessions_page = true;
            self.view = View::Notes;
            self.form = None;
            self.tab = Tab::Recordings;
            if Some(session) != self.selected {
                self.select(Some(session));
            }
        }
    }

    fn scan_folder(&mut self) {
        if self.form.is_some() {
            return;
        }
        self.refresh();
        if let (Some(id), Some(draft)) = (self.selected, &self.draft) {
            self.mentions
                .insert(id, timeline::referenced_ids(&draft.body));
            if !draft.unsaved() {
                match self.notes.iter().find(|note| note.id == id) {
                    Some(note) if note.path != draft.path => {
                        self.draft = None;
                        self.select(Some(id));
                    }
                    None => {
                        self.draft = None;
                        self.select(self.notes.first().map(|note| note.id));
                    }
                    Some(_) => {}
                }
            }
        }
    }

    fn finish_job(&mut self, result: Result<Option<String>, String>, session: Uuid) {
        let job = self.job.take();
        let failed_recording = job.as_ref().map(|(job, _)| job.recording.clone());
        let progress = job
            .as_ref()
            .map(|(job, _)| job.progress.words())
            .unwrap_or_default();
        if let Some(path) = &failed_recording {
            let words = match &result {
                Err(error) => Some(format!(
                    "{}: {progress}. {error}",
                    if error == processing::CANCELLED {
                        "Processing cancelled"
                    } else {
                        "Failed"
                    }
                )),
                _ => None,
            };
            if let Some(words) = words {
                let _ = fs::write(path.with_extension("state.txt"), words);
            } else {
                let _ = fs::remove_file(path.with_extension("state.txt"));
            }
        }
        let label = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or_else(|| "this session".to_owned(), Note::label);
        self.notice_session = Some(session);
        match result {
            Ok(None) => {
                self.job_done = true;
                self.job_warned = false;
                self.notice = Some((
                    Tone::Success,
                    format!(
                        "{label} is processed. Read the result, and add the events you want to the timeline."
                    ),
                ));
            }
            Ok(Some(warning)) => {
                self.job_done = true;
                self.job_warned = true;
                // The recording's row has the words, for as long as the result stands.
                let _ = warning;
                self.notice = None;
            }
            Err(error) if error == processing::CANCELLED => {
                self.notice = None;
                self.raise(Alert {
                    session,
                    can_record: false,
                    retry: failed_recording,
                    word: "Cancelled",
                    words: "Processing cancelled. Continue processing goes on from there."
                        .to_owned(),
                });
            }
            Err(error) => {
                self.notice = None;
                self.raise(Alert {
                    session,
                    can_record: false,
                    retry: failed_recording,
                    word: "Processing failed",
                    words: format!("Processing {label} failed. What is done is kept. {error}"),
                });
            }
        }
        if self.selected == Some(session) {
            self.refresh_recordings();
        }
    }

    fn open_campaign(&mut self, active: Option<&Campaign>) {
        self.flush();
        // A live recording blocks a campaign switch, so none is lost here.
        self.draft = None;
        self.template_draft = None;
        self.selected = None;
        self.template = None;
        self.reading = None;
        self.form = None;
        self.notice = None;
        self.deleted_run = None;
        self.row_notice = None;
        self.removed_group = None;
        self.plan_notice_state = None;
        self.removed_event_in_plan = false;
        self.updated_links = None;
        self.link_undo_state = None;
        self.removed_fight = None;
        self.campaign = active.cloned();
        self.folder_error = None;
        match active {
            Some(campaign) => {
                self.folder_text =
                    folder_setting(campaign).unwrap_or_else(|_| DEFAULT_FOLDER.to_owned());
                match session_folder(campaign) {
                    Ok(folder) => self.folder = Some(folder),
                    Err(error) => {
                        self.folder = None;
                        self.folder_error = Some(error);
                    }
                }
            }
            None => self.folder = None,
        }
        self.refresh();
        self.events_to = self
            .restored_session
            .take()
            .filter(|(campaign, _)| Some(*campaign) == active.map(|c| c.id))
            .map(|(_, session)| session);
        self.alerts.clear();
        self.job_done = false;
        self.job_warned = false;
        self.deleted_template = None;
        self.folder_notice = None;
        self.notice_session = None;
        self.select(self.current().or(self.notes.first().map(|note| note.id)));
    }

    /// The session the DM chose to play, when they chose.
    pub fn chosen_session(&self) -> Option<Option<Uuid>> {
        self.events_to
    }

    pub fn refresh(&mut self) {
        let Some(folder) = &self.folder else {
            self.notes.clear();
            self.mentions.clear();
            self.templates.clear();
            return;
        };
        self.notes = list_notes(folder);
        self.unreadable = unreadable_notes(folder);
        self.templates = list_templates(folder);
        self.mentions = self
            .notes
            .iter()
            .filter_map(|note| {
                let text = fs::read_to_string(&note.path).ok()?;
                Some((note.id, timeline::referenced_ids(&text)))
            })
            .collect();
    }

    fn refresh_recordings(&mut self) {
        self.recordings = match (&self.folder, self.selected) {
            (Some(folder), Some(session)) => list_recordings(folder, session),
            _ => Vec::new(),
        };
    }

    /// Save what can be saved. Called on the way to anything that can lose a draft.
    pub fn flush(&mut self) {
        for draft in [&mut self.draft, &mut self.template_draft]
            .into_iter()
            .flatten()
        {
            draft.save();
        }
    }

    pub fn select(&mut self, session: Option<Uuid>) {
        self.flush();
        self.list_only = false;
        // Words that could not be saved stay on screen, with the choice of what to do.
        if self.draft.as_ref().is_some_and(Draft::needs_attention) {
            self.write_in_note();
            return;
        }
        // Already open, so there is nothing to read again.
        if session == self.selected && self.draft.is_some() {
            return;
        }
        // The panel is for the session being played. It must not follow the page to an
        // older note, or the next words typed at the table go there.
        if self.notes_open && session != self.current() {
            self.notes_open = false;
        }
        self.selected = session;
        self.draft = session
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .and_then(|note| Draft::open(&note.path).ok());
        self.reading = None;
        self.added = None;
        self.open_event = None;
        self.fight = None;
        self.reading_note = session != self.current();
        if self.notice_session.is_none() {
            self.notice = None;
        }
        self.confirm_record = false;
        self.confirm_again = None;
        self.refresh_recordings();
    }

    /// Where new events and runs go: the newest session, unless the DM chose. Reading an
    /// older note does not change it, or tonight's run would file under last week.
    pub fn current(&self) -> Option<Uuid> {
        match self.events_to {
            Some(Some(id)) if self.notes.iter().any(|note| note.id == id) => Some(id),
            Some(None) => None,
            _ => self.notes.first().map(|note| note.id),
        }
    }

    /// Today, as of when the app started. A game that passes midnight is still tonight.
    fn today(&mut self) -> String {
        self.today
            .get_or_insert_with(crate::hermetic::today)
            .clone()
    }

    /// The words if new events go to a session that looks like an old one.
    pub fn current_dated(&mut self) -> Option<String> {
        let today = self.today();
        let current = self.current()?;
        let note = self.notes.iter().find(|note| note.id == current)?;
        dated_before_today(note, &today)
    }

    pub fn play_current(&mut self) {
        let current = self.current();
        if current != self.selected {
            self.select(current);
        }
        if current == self.selected {
            self.play_this_one();
        }
    }

    /// From anywhere: to the form for a new session.
    fn start_new_session(&mut self) {
        self.view = View::Notes;
        self.open_form();
        self.wants_sessions_page = true;
    }

    /// The DM prepared this session earlier, and is playing it now.
    fn play_this_one(&mut self) {
        let today = self.today();
        let confirming = self.confirm_record;
        let Some(note) = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .cloned()
        else {
            return;
        };
        self.flush();
        if self.draft.as_ref().is_some_and(Draft::needs_attention) {
            return;
        }
        match update_details(&note, &note.title, &today, note.number) {
            Ok(_) => {
                self.draft = None;
                self.refresh();
                let selected = self.selected;
                self.select(selected);
                // Pressed in the step before recording, that step is still what comes next.
                self.confirm_record = confirming;
                self.focus_notes = self.notes_open;
            }
            Err(error) => {
                self.notice_session = None;
                self.notice = Some((Tone::Error, error));
            }
        }
    }

    /// The words for working in a session other than the one being played.
    fn not_being_played(&self) -> Option<String> {
        if self.selected.is_none() || self.selected == self.current() {
            return None;
        }
        let note = self
            .notes
            .iter()
            .find(|note| Some(note.id) == self.selected)?;
        Some(format!(
            "Session {} is not the session being played. What you write and add goes here.",
            note.number
        ))
    }

    /// The DM chose where new events and runs go.
    pub fn set_current(&mut self, session: Option<Uuid>) {
        self.events_to = Some(session);
        if self.notes_open && session != self.selected {
            self.notes_open = false;
        }
    }

    pub fn links(&self) -> Vec<SessionLink> {
        self.notes
            .iter()
            .map(|note| SessionLink {
                id: note.id,
                label: note.label(),
                mentions: self.mentions.get(&note.id).cloned().unwrap_or_default(),
            })
            .collect()
    }

    pub fn open(&mut self, session: Uuid) {
        self.view = View::Notes;
        self.form = None;
        self.select(Some(session));
    }

    pub fn open_live_recording(&mut self) {
        if let Some(session) = self
            .live
            .as_ref()
            .map(|live| live.session)
            .or_else(|| self.job.as_ref().map(|(_, session)| *session))
            .or_else(|| self.alerts.last().map(|alert| alert.session))
            .or(self.notice_session)
        {
            self.open(session);
            self.tab = Tab::Recordings;
        }
    }

    /// Only this explicit action changes a note after an event rename.
    pub fn event_link_offer(&mut self, ui: &mut egui::Ui, book: &Book, event: &str) {
        if book.renamed_links.is_empty() && self.updated_links.is_none() {
            return;
        }
        let Some(renamed) = book.renamed_links.get(event) else {
            return;
        };
        let Some(current) = book.events.iter().find(|item| item.id == event) else {
            return;
        };
        let words = format!(
            "Notes still say “{}” for {}.",
            renamed.join("” or “"),
            current.title
        );
        let mut scoped = Book::default();
        scoped.events = book.events.clone();
        scoped
            .renamed_links
            .insert(event.to_owned(), renamed.clone());
        let mut changes = Vec::new();
        let mut count = 0;
        for note in &self.notes {
            let draft =
                if let Some(draft) = self.draft.as_ref().filter(|draft| draft.path == note.path) {
                    draft.clone()
                } else if let Ok(draft) = Draft::open(&note.path) {
                    draft
                } else {
                    continue;
                };
            let (body, found) = updated_event_links(&draft.body, &scoped);
            if found > 0 {
                count += found;
                changes.push((draft, body));
            }
        }
        if count > 0
            && design::message_with_action(
                ui,
                Tone::Info,
                &words,
                &format!("Update {} in notes", design::count(count, "link", "links")),
            )
        {
            self.update_event_link_notes(changes);
            self.link_undo_state = Some(LinkUndoState {
                notes: self.note_bodies(),
                events: book.events.clone(),
            });
        }
        if let Some(problem) = &self.link_problem {
            design::message(ui, Tone::Warning, problem);
        }
    }

    pub fn event_link_undo(&mut self, ui: &mut egui::Ui) {
        if self.updated_links.is_some()
            && design::message_with_action(
                ui,
                Tone::Info,
                "Links updated in notes.",
                "Undo update links",
            )
        {
            self.undo_event_link_notes();
        }
        if let Some(problem) = &self.link_problem {
            design::message(ui, Tone::Warning, problem);
        }
    }

    fn note_bodies(&self) -> Vec<(PathBuf, String)> {
        self.notes
            .iter()
            .filter_map(|note| {
                let body = self
                    .draft
                    .as_ref()
                    .filter(|draft| draft.path == note.path)
                    .map(|draft| draft.body.clone())
                    .or_else(|| {
                        fs::read_to_string(&note.path)
                            .ok()
                            .map(|text| body_of(&text).to_owned())
                    })?;
                let disk = fs::read_to_string(&note.path).unwrap_or_default();
                Some((note.path.clone(), format!("{body}\0{disk}")))
            })
            .collect()
    }

    pub fn expire_event_link_undo(&mut self, on_timeline: bool, book: &Book) {
        let changed = self
            .link_undo_state
            .as_ref()
            .is_some_and(|state| state.notes != self.note_bodies() || state.events != book.events);
        if !on_timeline || changed || self.draft.as_ref().is_some_and(Draft::needs_attention) {
            self.updated_links = None;
            self.link_undo_state = None;
            self.link_problem = None;
        }
    }

    fn update_event_link_notes(&mut self, changes: Vec<(Draft, String)>) {
        let mut undo = Vec::new();
        for (before, body) in changes {
            let mut next = before.clone();
            if next.needs_attention() {
                self.link_problem =
                    Some("Save or resolve the changed note before updating its links.".into());
                continue;
            }
            next.body = body;
            next.edited();
            next.save();
            if next.unsaved() {
                self.link_problem = Some(format!(
                    "Links in {} were not saved. Resolve the note's save problem and try again.",
                    next.path.display()
                ));
            } else {
                undo.push((before, next.body.clone()));
            }
            if self
                .draft
                .as_ref()
                .is_some_and(|draft| draft.path == next.path)
            {
                self.draft = Some(next);
            }
        }
        self.updated_links = (!undo.is_empty()).then_some(undo);
        self.last_time = None;
    }

    fn undo_event_link_notes(&mut self) {
        let mut remaining = Vec::new();
        for (before, after) in self.updated_links.take().unwrap_or_default() {
            let current = self
                .draft
                .as_ref()
                .filter(|draft| draft.path == before.path)
                .cloned()
                .or_else(|| Draft::open(&before.path).ok());
            if let Some(mut current) =
                current.filter(|draft| draft.body == after && !draft.needs_attention())
            {
                current.body = before.body.clone();
                current.edited();
                current.save();
                if current.unsaved() {
                    remaining.push((before.clone(), after));
                }
                if self
                    .draft
                    .as_ref()
                    .is_some_and(|draft| draft.path == current.path)
                {
                    self.draft = Some(current);
                }
            } else {
                self.link_problem = Some("A note changed after its links were updated. Keep or resolve that edit before Undo.".into());
                remaining.push((before, after));
            }
        }
        self.updated_links = (!remaining.is_empty()).then_some(remaining);
        self.last_time = None;
    }

    pub fn recording(&self) -> bool {
        self.live.is_some()
    }

    pub fn rail_recording_status(&self) -> Option<Status<'static>> {
        let c = colors();
        if let Some((icon, word, color)) = self.recording_status() {
            return Some(Status { icon, word, color });
        }
        if self.processing() {
            return Some(Status {
                icon: icons::AUDIO,
                word: "Processing audio",
                color: c.connection.busy,
            });
        }
        if let Some(word) = self.alert_words().last() {
            return Some(Status {
                icon: if word == "Cancelled" {
                    icons::WARN
                } else {
                    icons::ERROR
                },
                word,
                color: if word == "Cancelled" {
                    c.result.warning
                } else {
                    c.result.error
                },
            });
        }
        if self.job_warned() {
            return Some(Status {
                word: "Processed",
                ..processed_warning()
            });
        }
        if self.job_done() {
            return Some(Status {
                icon: icons::SUCCESS,
                word: "Processing done",
                color: c.result.success,
            });
        }
        self.just_saved().then_some(Status {
            icon: icons::OK,
            word: "Recording saved",
            color: c.document.saved,
        })
    }

    /// The same timed recording words on the rail and the session page.
    pub fn recording_detail(&self) -> Option<String> {
        let Some(live) = self.live.as_ref() else {
            return self
                .rail_recording_status()
                .map(|status| status.word.to_owned());
        };
        let (_, word, _) = self.recording_status()?;
        Some(if word == "Not saving audio" {
            word.to_owned()
        } else {
            format!(
                "{} {}",
                if word == "Recording silence" {
                    "Silence"
                } else {
                    word
                },
                clock(live.recorder.seconds())
            )
        })
    }

    /// The word the rail keeps in view. A failed recording cannot look like a live one.
    pub fn recording_status(&self) -> Option<(&'static str, &'static str, Color32)> {
        let c = colors();
        self.live.as_ref().map(|live| match live.recorder.health() {
            Health::Stalled | Health::Stopped(_) => {
                (icons::ERROR, "Not saving audio", c.result.error)
            }
            Health::Silent => (icons::WARN, "Recording silence", c.result.warning),
            Health::Starting | Health::Hearing => (icons::RECORD, "Recording", c.recording),
        })
    }

    /// A word for the run's own notice line. It takes no room of its own, so nothing moves.
    pub fn run_notice(&self) -> Option<&'static str> {
        self.live
            .as_ref()
            .filter(|live| *live.recorder.health() == Health::Silent)
            .map(|_| "Recording silence. Check the input in Settings.")
    }

    /// A recording stopped a moment ago, and its file is kept.
    pub fn just_saved(&self) -> bool {
        self.saved_at
            .is_some_and(|at| at.elapsed() < SAVED_WORD_FOR)
    }

    /// A job finished, and the DM has not looked at it yet.
    pub fn job_done(&self) -> bool {
        self.job_done
    }

    pub fn job_warned(&self) -> bool {
        self.job_warned
    }

    /// What failed, for the rail, until the DM dismisses it or tries again.
    pub fn alert_words(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.alerts.iter().map(|alert| alert.word)
    }

    /// One alert of each kind, so that a later failure cannot hide a stopped recording.
    fn raise(&mut self, alert: Alert) {
        self.alerts.retain(|old| old.can_record != alert.can_record);
        self.alerts.push(alert);
    }

    pub fn recording_label(&self) -> Option<&str> {
        self.live.as_ref().map(|live| live.label.as_str())
    }

    pub fn processing(&self) -> bool {
        self.job.is_some()
    }

    /// The session being processed, by name.
    pub fn processing_label(&self) -> Option<String> {
        let (_, session) = self.job.as_ref()?;
        self.notes
            .iter()
            .find(|note| note.id == *session)
            .map(Note::label)
    }

    /// Each draft that closing the app would lose, by name.
    pub fn not_saved(&self) -> Vec<String> {
        let mut items = Vec::new();
        if let Some(draft) = self.draft.as_ref().filter(|draft| draft.needs_attention()) {
            items.push(format!(
                "Notes in “{}” are not saved.",
                draft.path.file_stem().unwrap_or_default().to_string_lossy()
            ));
        }
        if let Some(draft) = self
            .template_draft
            .as_ref()
            .filter(|draft| draft.needs_attention())
        {
            items.push(format!(
                "Template “{}” is not saved.",
                draft.path.file_stem().unwrap_or_default().to_string_lossy()
            ));
        }
        items
    }

    pub fn notes_changed_on_disk(&self) -> bool {
        self.draft
            .as_ref()
            .is_some_and(|draft| draft.state == DraftState::Conflict)
    }

    pub fn busy_with_edit(&self) -> bool {
        !self.not_saved().is_empty()
    }

    fn finish_recording(&mut self) {
        self.stop_recorder(true);
    }

    /// Stop, and say what is true of the file. Returns whether it holds any audio.
    fn stop_recorder(&mut self, tell: bool) -> bool {
        let Some(live) = self.live.take() else {
            return false;
        };
        let (session, label) = (live.session, live.label.clone());
        let stopped = match live.recorder.health() {
            Health::Stopped(words) => Some(format!(
                "Stopped by itself: {}.",
                words.trim_end_matches(['.', '!', '?'])
            )),
            _ => None,
        };
        let path = live.recorder.stop();
        if let Some(words) = stopped {
            let _ = fs::write(path.with_extension("state.txt"), words);
        }
        let has_audio =
            crate::recording::Layout::read(&path).is_ok_and(|layout| layout.data_length > 0);
        if !has_audio {
            // An empty file is not a recording, and it must not look like one.
            let _ = fs::remove_file(&path);
        }
        if tell {
            self.notice_session = Some(session);
            self.notice = Some(if has_audio {
                self.saved_at = Some(Instant::now());
                (
                    Tone::Success,
                    format!("Recording saved: {}.", recording_title(&path)),
                )
            } else {
                (Tone::Warning, format!("Nothing was recorded for {label}."))
            });
        }
        self.refresh_recordings();
        has_audio
    }

    fn start_recording(
        &mut self,
        settings: &ProcessingSettings,
        session: Uuid,
    ) -> Result<(), String> {
        let (Some(folder), Some(tool)) = (&self.folder, settings.tool()) else {
            return Err("No recorder is ready.".to_owned());
        };
        let label = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or_else(|| "this session".to_owned(), Note::label);
        let path = new_recording_path(folder, session);
        match Recorder::start(tool, settings.input(), path) {
            Ok(recorder) => {
                self.notice = None;
                self.notice_session = None;
                // A new recording answers an old failure of this session.
                self.alerts
                    .retain(|alert| !(alert.can_record && alert.session == session));
                self.live = Some(Live {
                    session,
                    label,
                    recorder,
                });
                self.refresh_recordings();
            }
            Err(error) => return Err(error),
        }
        Ok(())
    }

    fn start_processing(
        &mut self,
        ctx: &egui::Context,
        recording: &Path,
        settings: &ProcessingSettings,
        book: &Book,
    ) {
        if let Some(session) = self.selected {
            self.process_recording(ctx, recording, settings, book, session);
        }
    }

    fn process_recording(
        &mut self,
        ctx: &egui::Context,
        recording: &Path,
        settings: &ProcessingSettings,
        book: &Book,
        session: Uuid,
    ) {
        let note = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or_else(String::new, Note::label);
        let days = timeline::day_numbers(&book.events);
        let events: Vec<String> = book
            .events
            .iter()
            .zip(&days)
            .filter(|(event, _)| event.session == Some(session))
            .map(|(event, day)| format!("- {}", event.label(*day)))
            .collect();
        let context = format!(
            "{note}\n\n{}\n\nEvents the DM already recorded:\n{}",
            if self.selected == Some(session) {
                self.draft
                    .as_ref()
                    .map_or_else(String::new, |draft| draft.body.clone())
            } else {
                self.notes
                    .iter()
                    .find(|note| note.id == session)
                    .and_then(|note| fs::read_to_string(&note.path).ok())
                    .map(|text| body_of(&text).to_owned())
                    .unwrap_or_default()
            },
            events.join("\n")
        );
        let (speech_key, llm_key) = settings.held_keys();
        self.notice = None;
        self.notice_session = None;
        let _ = fs::remove_file(recording.with_extension("state.txt"));
        self.job_done = false;
        self.job_warned = false;
        self.alerts
            .retain(|alert| alert.can_record || alert.session != session);
        let job = processing::start(
            ctx,
            Request {
                recording: recording.to_owned(),
                config: settings.config.clone(),
                context,
                speech_hint: format!(
                    "A Dungeons & Dragons session at the table. {note}, in the campaign {}.",
                    self.campaign.as_ref().map_or("", |c| c.name.as_str())
                ),
                speech_key,
                llm_key,
                codex_program: "codex".into(),
            },
            |request| {
                // A key goes only to the service it was saved for.
                if request.speech_key.is_none() && request.config.speech_key_fits() {
                    request.speech_key = credentials::load_key(KeySlot::Speech)?;
                }
                if request.llm_key.is_none()
                    && request.config.provider != processing::Provider::CodexCli
                    && request.config.llm_key_fits()
                {
                    request.llm_key = credentials::load_key(KeySlot::Llm)?;
                }
                Ok(())
            },
        );
        self.job = Some((job, session));
    }

    /// Why `Record` cannot start now, in words.
    fn record_blocked(&self, settings: &ProcessingSettings) -> Option<String> {
        if let Some(live) = &self.live {
            Some(format!("Stop the recording of {} first.", live.label))
        } else if settings.probing() {
            Some("Looking for a recorder.".to_owned())
        } else if settings.tool().is_none() {
            Some("No recorder found. See Settings.".to_owned())
        } else if settings.input_missing() {
            Some("The input chosen in Settings is not connected.".to_owned())
        } else {
            None
        }
    }

    fn record_control(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        // Stop is here whichever session is open. It can be urgent, if a player asks.
        if self.live.as_ref().is_some_and(|live| {
            matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
        }) {
            return;
        }
        if let Some(live) = &self.live {
            if self.tab == Tab::Recordings
                && self.reading.is_none()
                && self.selected == Some(live.session)
                && !self.recordings.is_empty()
            {
                return;
            }
            let name = if Some(live.session) == self.selected {
                "Stop recording".to_owned()
            } else {
                format!("Stop recording of {}", live.label)
            };
            if Action::secondary("Stop recording")
                .icon(icons::STOP_RECORDING)
                .name(&name)
                .hint(&name)
                .show(ui)
                .clicked()
            {
                self.finish_recording();
            }
            return;
        }
        if self.confirm_record
            || (self.tab == Tab::Recordings
                && self.reading.is_none()
                && self.recordings.iter().any(|recording| {
                    recording
                        .status
                        .as_deref()
                        .is_some_and(|words| words.starts_with("Stopped by itself:"))
                }))
        {
            return;
        }
        let reason = self.record_blocked(settings);
        if Action::secondary("Record")
            .icon(icons::RECORD)
            .name("Record this session")
            .blocked(reason.as_deref())
            .show(ui)
            .clicked()
        {
            self.confirm_record = true;
        }
        if let Some(reason) = reason {
            ui.label(Type::Small.rich(ui, reason).color(colors().muted));
            ui.add_space(design::L);
        }
    }

    /// The live state, or the step before it. Always in the same place.
    fn record_line(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        // While a recording runs, the line keeps one height, so the note below never moves.
        let live = self.live.as_ref().is_some_and(|live| {
            !matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
        });
        let top = ui.cursor().top();
        self.record_line_body(ui, settings);
        if live && !self.confirm_record {
            let used = ui.cursor().top() - top;
            ui.add_space((Density::Read.control() + design::XS - used).max(0.0));
        }
    }

    fn record_line_body(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        let c = colors();
        if let Some(live) = self
            .live
            .as_ref()
            .filter(|live| Some(live.session) == self.selected)
        {
            let time = clock(live.recorder.seconds());
            match live.recorder.health() {
                Health::Stalled | Health::Stopped(_) => {}
                health => {
                    let (word, color) = match health {
                        Health::Starting => ("Waiting for sound", c.muted),
                        Health::Silent => ("Silent for over a minute", c.result.warning),
                        // Only what is true now. A table goes quiet, and that is no warning.
                        _ if live.recorder.heard_just_now() => ("Hearing sound", c.muted),
                        _ => ("Quiet", c.muted),
                    };
                    ui.horizontal_wrapped(|ui| {
                        // The first thing this page must say, large enough for a glance.
                        ui.label(
                            Type::Strong
                                .rich(
                                    ui,
                                    format!(
                                        "{} {} {time}",
                                        if *health == Health::Silent {
                                            icons::WARN
                                        } else {
                                            icons::RECORD
                                        },
                                        if *health == Health::Silent {
                                            "Recording silence"
                                        } else {
                                            "Recording"
                                        }
                                    ),
                                )
                                .color(if *health == Health::Silent {
                                    c.result.warning
                                } else {
                                    c.recording
                                }),
                        );
                        ui.label(Type::Small.rich(ui, word).color(color));
                        if *health == Health::Silent
                            && Action::quiet("Check the input in Settings")
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                        {
                            self.wants_processing_settings = true;
                        }
                    });
                }
            }
        } else if let (Some(live), Some((icon, word, color))) =
            (&self.live, self.recording_status())
        {
            let words = format!(
                "{word} {} in {}",
                clock(live.recorder.seconds()),
                live.label
            );
            design::status(
                ui,
                Status {
                    icon,
                    word: &words,
                    color,
                },
            )
            .on_hover_text(&words);
        } else if settings.input_missing() {
            let mut open = false;
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    Type::Small
                        .rich(ui, "The input chosen in Settings is not connected.")
                        .color(c.result.warning),
                );
                open = Action::quiet("Open Settings")
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
            });
            if open {
                self.wants_processing_settings = true;
            }
        } else if self.confirm_record {
            let label = self
                .selected
                .and_then(|id| self.notes.iter().find(|note| note.id == id))
                .map_or_else(String::new, Note::label);
            let today = self.today();
            let dated = self
                .selected
                .and_then(|id| self.notes.iter().find(|note| note.id == id))
                .and_then(|note| dated_before_today(note, &today));
            let other = if self.selected != self.current() {
                " This is not the session being played.".to_owned()
            } else {
                dated
                    .as_ref()
                    .map_or_else(String::new, |words| format!(" {words}"))
            };
            design::message(
                ui,
                if other.is_empty() {
                    Tone::Info
                } else {
                    Tone::Warning
                },
                &format!(
                    "Record {label}.{other} Tell everyone at the table first. The audio stays in the campaign folder until you select Process."
                ),
            );
            ui.horizontal(|ui| {
                if dated.is_some()
                    && self.selected == self.current()
                    && Action::secondary("Play this one").show(ui).clicked()
                {
                    self.play_this_one();
                }
                if (if self.run_open {
                    Action::secondary("Start recording")
                } else {
                    Action::primary("Start recording")
                })
                .show(ui)
                .clicked()
                {
                    self.confirm_record = false;
                    if let Some(session) = self.selected
                        && let Err(error) = self.start_recording(settings, session)
                    {
                        self.notice_session = None;
                        self.notice = Some((Tone::Error, error));
                    }
                }
                if Action::quiet("Not now").show(ui).clicked() {
                    self.confirm_record = false;
                }
            });
        }
    }

    /// A note's state says it is the note, beside a control about audio.
    fn note_save_status(draft: &Draft) -> Status<'static> {
        let c = colors();
        let (icon, word, color) = match &draft.state {
            DraftState::Clean => (icons::OK, "Notes saved", c.document.clean),
            DraftState::Saved => (icons::OK, "Notes saved", c.document.saved),
            DraftState::Unsaved => (icons::BUSY, "Saving…", c.muted),
            DraftState::Conflict => (icons::WARN, "Changed on disk", c.document.external),
            DraftState::Missing => (icons::ERROR, "Notes not saved", c.document.invalid),
            DraftState::Failed(_) => (icons::ERROR, "Notes not saved", c.document.invalid),
        };
        Status { icon, word, color }
    }

    pub fn attention_status(&self) -> Option<Status<'static>> {
        self.draft
            .as_ref()
            .filter(|draft| draft.needs_attention())
            .map(Self::note_save_status)
    }

    fn note_status(ui: &mut egui::Ui, draft: &Draft) {
        design::status(ui, Self::note_save_status(draft));
    }

    fn draft_status(ui: &mut egui::Ui, draft: &Draft) {
        let c = colors();
        let (icon, word, color) = match &draft.state {
            DraftState::Clean => (icons::OK, "Saved", c.document.clean),
            DraftState::Saved => (icons::OK, "Saved", c.document.saved),
            DraftState::Unsaved => (icons::DRAFT, "Unsaved", c.document.unsaved),
            DraftState::Conflict => (icons::WARN, "Changed on disk", c.document.external),
            DraftState::Missing => (icons::WARN, "File gone", c.document.external),
            DraftState::Failed(_) => (icons::ERROR, "Not saved", c.document.invalid),
        };
        design::status(ui, Status { icon, word, color });
    }

    fn draft_problem(ui: &mut egui::Ui, draft: &mut Draft) {
        if draft.dropped.is_some()
            && design::message_with_action(ui, Tone::Info, "Draft dropped.", "Undo reload")
        {
            draft.undo_reload();
        }
        match draft.state.clone() {
            DraftState::Conflict => {
                design::message(
                    ui,
                    Tone::Warning,
                    "This file changed on disk while you were editing. Nothing is saved until you choose, and other sessions open after that.",
                );
                ui.horizontal_wrapped(|ui| {
                    if Action::secondary("Overwrite with my draft")
                        .show(ui)
                        .clicked()
                    {
                        draft.overwrite();
                    }
                    ui.add_space(design::L);
                    if Action::destructive("Reload and lose my draft")
                        .show(ui)
                        .clicked()
                        && draft.reload_keeping_undo().is_err()
                    {
                        draft.state = DraftState::Missing;
                    }
                });
            }
            DraftState::Missing => {
                design::message(
                    ui,
                    Tone::Warning,
                    "This file is gone from the folder. It may have been moved or renamed. Your words are still here. Its recordings and events stay with the old session, and do not move to a new file. Other sessions wait until you save.",
                );
                if Action::secondary("Save my words as a new file")
                    .show(ui)
                    .clicked()
                {
                    draft.save_again();
                }
            }
            DraftState::Failed(error) => {
                design::message(ui, Tone::Error, &error);
                if Action::secondary("Try to save again").show(ui).clicked() {
                    draft.save();
                }
            }
            _ => {}
        }
    }

    fn open_form(&mut self) {
        self.list_only = false;
        self.focus_form_title = true;
        // A template edited a moment ago is the one a new session starts from.
        self.flush();
        self.refresh();
        let number = self.notes.iter().map(|note| note.number).max().unwrap_or(0) + 1;
        let template = self
            .templates
            .iter()
            // The standard one, or a customized copy of it. A name that only sorts first,
            // such as a downtime template, is not what a new session wants.
            .find(|template| template.name == super::BUILT_IN_TEMPLATES[0].0)
            .or(self.templates.first())
            .map_or_else(String::new, |template| template.name.clone());
        self.form = Some(Form {
            editing: None,
            number: number.to_string(),
            title: String::new(),
            date: crate::hermetic::today(),
            template,
            error: None,
        });
    }

    fn show_form(&mut self, ui: &mut egui::Ui) {
        let Some(mut form) = self.form.take() else {
            return;
        };
        let heading = if form.editing.is_some() {
            "Session details"
        } else {
            "New session"
        };
        let c = colors();
        design::group_heading(ui, heading, 0);
        ui.add_space(design::XS);
        design::field(ui, "Number", None, None, |ui| {
            design::number_field(
                ui,
                "Number",
                &mut form.number,
                "",
                design::L * 5.0,
                Density::Read.control(),
            );
        });
        let title = design::text_field(
            ui,
            "Session title",
            &mut form.title,
            Some("Optional. You can name it after you play."),
            None,
        );
        // The one field a DM usually types, often with people waiting.
        if std::mem::take(&mut self.focus_form_title) {
            title.request_focus();
        }
        let title_entered = title.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        design::text_field(ui, "Date", &mut form.date, Some("As YYYY-MM-DD."), None);
        if form.editing.is_none() {
            design::field(ui, "Template", None, None, |ui| {
                let inner = egui::ComboBox::from_id_salt("session_template")
                    .selected_text(&form.template)
                    .width(ui.available_width().min(design::FORM_WIDTH))
                    .icon(design::combo_icon)
                    .show_ui(ui, |ui| {
                        for template in &self.templates {
                            ui.selectable_value(
                                &mut form.template,
                                template.name.clone(),
                                &template.name,
                            );
                        }
                    });
                design::named(ui, &inner.response, "Session template");
            });
        }
        if form.editing.is_none()
            && let Some(folder) = &self.folder
        {
            let path = super::note_path(folder, form.number.parse().unwrap_or(1), &form.title);
            let shown = path.strip_prefix(folder).unwrap_or(&path);
            wrapped_words(
                ui,
                Type::Small,
                format!("File: {}", shown.display()),
                c.muted,
            );
        }
        if let Some(error) = &form.error {
            design::message(ui, Tone::Error, error);
        }
        let mut done = title_entered;
        let mut cancel = false;
        ui.horizontal(|ui| {
            let label = if form.editing.is_some() {
                "Save details"
            } else {
                "Create session"
            };
            done |= if self.open_run.is_some() {
                Action::secondary(label)
            } else {
                Action::primary(label)
            }
            .show(ui)
            .clicked();
            cancel = Action::quiet("Cancel").show(ui).clicked();
        });
        if cancel {
            return;
        }
        if done {
            match self.submit_form(&form) {
                Ok(id) => {
                    self.refresh();
                    self.select(Some(id));
                    if form.editing.is_none() {
                        self.say_where_the_run_stays(id);
                    }
                    self.write_in_note();
                    // A new session is for typing in, under What happened.
                    self.focus_notes = form.editing.is_none();
                    return;
                }
                Err(error) => form.error = Some(error),
            }
        }
        self.form = Some(form);
    }

    /// The warning about an old session has gone, and the open run's event has not moved.
    fn say_where_the_run_stays(&mut self, new: Uuid) {
        self.moved_run = None;
        let Some((name, session)) = self.open_run.clone().filter(|(_, session)| *session != new)
        else {
            return;
        };
        let Some(note) = self.notes.iter().find(|note| note.id == session) else {
            return;
        };
        self.notice = Some((Tone::Info, format!("{name} stays in {}.", note.label())));
        self.notice_session = Some(new);
        self.move_run_here = self.notice.as_ref().map(|(_, words)| (new, words.clone()));
    }

    fn move_run_to_session(&mut self, book: &mut Book, run: Uuid, session: Uuid) {
        let Some(event) = book.events.iter_mut().find(|event| event.run == Some(run)) else {
            return;
        };
        let before = event.clone();
        event.session = Some(session);
        event.continued_in = None;
        book.touch();
        let open_run = self.open_run.clone();
        let name = open_run.as_ref().map_or("Fight", |(name, _)| name.as_str());
        let number = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or(1, |note| note.number);
        let words = format!("{name} moved to Session {number}.");
        self.notice = Some((Tone::Success, words.clone()));
        self.notice_session = Some(session);
        self.moved_run = Some(MovedRun {
            before,
            open_run,
            body: self
                .draft
                .as_ref()
                .map_or_else(String::new, |draft| draft.body.clone()),
            events: book.events.clone(),
            words,
        });
        self.open_run = self.open_run.take().map(|(name, _)| (name, session));
        self.move_run_here = None;
    }

    fn expire_move(&mut self, book: &Book) {
        if self.moved_run.as_ref().is_some_and(|moved| {
            self.notice
                .as_ref()
                .is_none_or(|(_, words)| *words != moved.words)
        }) {
            self.moved_run = None;
        }
        if self.moved_run.as_ref().is_some_and(|moved| {
            moved.events != book.events
                || self
                    .draft
                    .as_ref()
                    .is_none_or(|draft| draft.body != moved.body)
        }) {
            self.moved_run = None;
            self.notice = None;
            self.notice_session = None;
        }
    }

    fn undo_move(&mut self, book: &mut Book) {
        self.expire_move(book);
        if let Some(moved) = self.moved_run.take() {
            if let Some(event) = book
                .events
                .iter_mut()
                .find(|event| event.id == moved.before.id)
            {
                *event = moved.before;
                book.touch();
            }
            self.open_run = moved.open_run;
            self.notice = None;
            self.notice_session = None;
        }
    }

    fn submit_form(&mut self, form: &Form) -> Result<Uuid, String> {
        let number: u32 = form
            .number
            .trim()
            .parse()
            .map_err(|_| "The number must be a whole number.".to_owned())?;
        // A date's order is how Splat tells an old session from tonight's.
        let date = form.date.trim().as_bytes();
        if date.len() != 10
            || date.iter().enumerate().any(|(at, c)| {
                if at == 4 || at == 7 {
                    *c != b'-'
                } else {
                    !c.is_ascii_digit()
                }
            })
        {
            return Err("Enter the date as YYYY-MM-DD.".to_owned());
        }
        if let Some(id) = form.editing {
            let note = self
                .notes
                .iter()
                .find(|note| note.id == id)
                .cloned()
                .ok_or("This session is gone from the folder.")?;
            self.flush();
            if self.draft.as_ref().is_some_and(Draft::needs_attention) {
                return Err("Choose what to do with the unsaved notes first.".to_owned());
            }
            update_details(&note, &form.title, &form.date, number)?;
            // The file changed, so the draft reads it again.
            self.draft = None;
            return Ok(id);
        }
        let folder = self.folder.as_ref().ok_or("No session folder.")?;
        let campaign = self.campaign.as_ref().ok_or("No campaign.")?;
        let template = self
            .templates
            .iter()
            .find(|template| template.name == form.template)
            .map_or("", |template| template.body.as_str());
        let note = create_note(
            folder,
            &NewNote {
                number,
                title: &form.title,
                date: form.date.trim(),
                campaign: &campaign.name,
                template,
            },
        )?;
        // Whatever was chosen before, a new session is the one being played now.
        self.events_to = None;
        Ok(note.id)
    }

    fn list_pane(&mut self, ui: &mut egui::Ui) {
        let locked = self.draft.as_ref().is_some_and(Draft::needs_attention);
        let mut choose = None;
        let live = self.live.as_ref().map(|live| live.session);
        egui::ScrollArea::vertical()
            .id_salt("session_list")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_enabled_ui(!locked, |ui| {
                    for note in &self.notes {
                        let job = self.job.as_ref().map(|(_, session)| *session);
                        let result = self.notice_session.filter(|_| self.job_done);
                        let state = if live == Some(note.id) || job == Some(note.id) {
                            self.rail_recording_status()
                        } else {
                            self.alerts
                                .iter()
                                .rev()
                                .find(|alert| alert.session == note.id)
                                .map(|alert| {
                                    let cancelled = alert.word == "Cancelled";
                                    Status {
                                        icon: if cancelled { icons::WARN } else { icons::ERROR },
                                        word: if cancelled {
                                            "Cancelled"
                                        } else if alert.can_record {
                                            "Stopped"
                                        } else {
                                            "Failed"
                                        },
                                        color: if cancelled {
                                            colors().result.warning
                                        } else {
                                            colors().result.error
                                        },
                                    }
                                })
                        };
                        let detail = if let Some(state) = state {
                            format!("{} {}", state.icon, state.word)
                        } else if result == Some(note.id) {
                            let word = if self.job_warned {
                                "Processed, with a warning"
                            } else {
                                "Processed"
                            };
                            format!("{} · {word}", note.date)
                        } else {
                            note.date.clone()
                        };
                        let detail = if self.current() == Some(note.id) {
                            format!("{detail} · {} Playing", icons::RUN)
                        } else {
                            detail
                        };
                        if session_row(
                            ui,
                            self.selected == Some(note.id),
                            &note.label(),
                            &detail,
                            state,
                        )
                        .clicked()
                        {
                            choose = Some(note.id);
                        }
                    }
                });
            });
        if let Some(id) = choose {
            self.form = None;
            self.list_only = false;
            self.select(Some(id));
        }
    }

    /// The one editor of a session's note, on the page and in the panel beside any page.
    fn note_editor(&mut self, ui: &mut egui::Ui, in_panel: bool) {
        let Some(draft) = &mut self.draft else {
            design::message(ui, Tone::Error, "This note could not be opened.");
            return;
        };
        // Beside a page with its own draft, Ctrl+S goes to where the keyboard is.
        if (!in_panel || self.note_had_focus)
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S))
        {
            draft.save();
        }
        if in_panel {
            Self::draft_problem(ui, draft);
        }
        if draft.state == DraftState::Conflict {
            if draft.on_disk.is_none() {
                draft.show_disk();
            }
            if let Some(disk) = &draft.on_disk {
                design::draft_comparison(ui, &draft.body, disk, (&draft.path, disk));
                return;
            }
        }
        let focus = std::mem::take(&mut self.focus_notes);
        // A stray click leaves the keyboard on nothing, and the DM's eyes can be on the table.
        // With no field to go to, what they type still belongs in the note.
        if in_panel
            && !egui::Popup::is_any_open(ui.ctx())
            && ui
                .ctx()
                .memory(|memory| memory.focused().is_none() && memory.top_modal_layer().is_none())
        {
            let typed: String = ui.input_mut(|i| {
                let mut typed = String::new();
                i.events.retain(|event| match event {
                    egui::Event::Text(text) | egui::Event::Paste(text) => {
                        typed.push_str(text);
                        false
                    }
                    _ => true,
                });
                typed
            });
            if !typed.is_empty() {
                self.words_for_note = typed;
                self.words_new_line = self.break_before_words;
                self.refocus_note = true;
            }
        }
        if self.break_before_words
            && let Some(id) = self.note_editor_id
            && ui.ctx().memory(|memory| memory.has_focus(id))
        {
            let (typed, correcting) = ui.input(|i| {
                let typed = i.events.iter().any(|e| matches!(e, egui::Event::Text(_)));
                let correcting = i.events.iter().any(|e| {
                    matches!(e, egui::Event::Key { key, pressed: true, .. } if matches!(
                        key,
                        egui::Key::Backspace
                            | egui::Key::Delete
                            | egui::Key::ArrowLeft
                            | egui::Key::ArrowRight
                            | egui::Key::ArrowUp
                            | egui::Key::ArrowDown
                            | egui::Key::Home
                            | egui::Key::End
                    ))
                });
                (typed, correcting)
            });
            // A correction goes on with the same line.
            if typed || correcting {
                self.break_before_words = false;
            }
            if typed && !correcting {
                insert_at_caret(ui.ctx(), id, draft, "", true);
            }
        }
        let heading_id = ui.id().with(("note_heading_offset", self.selected));
        let editor_height = ui.available_height()
            + if in_panel {
                ui.data(|data| data.get_temp::<f32>(heading_id).unwrap_or(0.0))
            } else {
                0.0
            };
        egui::ScrollArea::vertical()
            .id_salt(("session_editor", self.selected, in_panel))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Beside a page, Tab indents. It must not walk to `Next turn`, where the next
                // space would press it. Escape still leaves the panel.
                let mut layouter = crate::markdown_source::layouter;
                let output = egui::TextEdit::multiline(&mut draft.body)
                    .lock_focus(in_panel)
                    .font(design::reader::prose_type(ui).font(ui))
                    .layouter(&mut layouter)
                    .margin(egui::vec2(design::S, design::S))
                    .desired_width(ui.available_width())
                    .min_size(egui::vec2(0.0, editor_height))
                    .show(ui);
                crate::markdown_source::paint_section_rules(ui, &output);
                if in_panel && let Some(at) = draft.body.find("## What happened") {
                    // Even a short note must scroll its repeated heading out of the panel.
                    let heading = draft.body[..at].chars().count();
                    let offset = output
                        .galley
                        .pos_from_cursor(egui::text::CCursor::new(heading))
                        .top();
                    ui.data_mut(|data| data.insert_temp(heading_id, offset.max(0.0)));
                }
                let response = output.response;
                design::named(ui, &response, "Note text");
                self.note_had_focus = response.has_focus();
                self.note_editor_id = Some(response.id);
                // A click chooses the place, so what is typed goes exactly there.
                if focus || response.clicked() {
                    self.break_before_words = false;
                }
                if std::mem::take(&mut self.refocus_note) {
                    response.request_focus();
                    let words = std::mem::take(&mut self.words_for_note);
                    if !words.is_empty() {
                        // These words had their break, and what follows goes on from them.
                        self.break_before_words = false;
                        let new_line = std::mem::take(&mut self.words_new_line);
                        insert_at_caret(ui.ctx(), response.id, draft, &words, new_line);
                    }
                }
                if let Some(caret) = self.caret_to_show.take() {
                    let rect = output
                        .galley
                        .pos_from_cursor(egui::text::CCursor::new(if in_panel {
                            draft
                                .body
                                .find("## What happened")
                                .map_or(caret, |at| draft.body[..at].chars().count())
                        } else {
                            caret
                        }))
                        .translate(output.galley_pos.to_vec2());
                    ui.scroll_to_rect(
                        rect,
                        Some(if in_panel {
                            egui::Align::TOP
                        } else {
                            egui::Align::Center
                        }),
                    );
                }
                if focus {
                    response.request_focus();
                    let (caret, changed) = fresh_line_in_note(&mut draft.body);
                    if changed {
                        draft.edited();
                    }
                    self.caret_to_show = Some(caret);
                    let end = egui::text::CCursor::new(caret);
                    let mut state =
                        egui::TextEdit::load_state(ui.ctx(), response.id).unwrap_or_default();
                    state
                        .cursor
                        .set_char_range(Some(egui::text::CCursorRange::one(end)));
                    egui::TextEdit::store_state(ui.ctx(), response.id, state);
                }
                if response.changed() {
                    draft.edited();
                    if let Some(session) = self.selected {
                        self.mentions
                            .insert(session, timeline::referenced_ids(&draft.body));
                    }
                }
            });
    }

    pub fn fresh_focus_in_panel(&mut self) {
        self.focus_notes = self.notes_open;
    }

    /// A button took the keyboard. It goes back to the panel, or to the page's editor if
    /// the DM was typing there.
    /// Words the DM typed in the run's Amount, going on with the note.
    pub fn words_for_note(&mut self, words: String) {
        self.words_for_note = words;
        self.words_new_line = true;
    }

    /// After a step of the run, what is typed next is a new thought. Nothing changes in the
    /// note until the DM types it, so a click alone leaves the note as it was.
    pub fn after_a_step_back_to_note(&mut self, notes_gave_way: bool) {
        self.return_focus_to_note();
        self.break_before_words = self.refocus_note || notes_gave_way;
    }

    pub fn return_focus_to_note(&mut self) {
        let in_page_editor = self.on_page
            && !self.reading_note
            && self.tab != Tab::Recordings
            && self.note_had_focus;
        self.refocus_note = self.notes_open || in_page_editor;
    }

    /// Open or close the note beside the current page. It opens on the session that new
    /// events go to, because that is the one being played. The caret starts on a fresh line.
    pub fn toggle_notes(&mut self, on_this_page: bool) {
        // A form open here holds typed words, and it comes first.
        if on_this_page && self.form.is_some() {
            return;
        }
        let open = on_this_page || !self.notes_open;
        if !on_this_page {
            self.notes_open = open;
        }
        if open {
            let current = self.current();
            if current.is_some() && current != self.selected {
                self.select(current);
            }
            // Words that could not be saved came first, and they are on the page.
            if current.is_some() && self.selected != current {
                self.notes_open = false;
                self.wants_sessions_page = true;
                return;
            }
            self.focus_notes = true;
            if on_this_page {
                self.view = View::Notes;
                self.write_in_note();
            }
        }
    }

    /// Back after Lookup, with the caret where it was, on the session being played.
    pub fn notes_come_back(&mut self) {
        if self.selected.is_some() && self.current() == self.selected {
            self.notes_open = true;
            self.refocus_note = true;
        } else if !self.on_page {
            self.toggle_notes(false);
        }
    }

    pub fn notes_panel(&mut self, ctx: &egui::Context) {
        let width = design::side_panel_width(ctx.content_rect().width());
        design::Column::panel("session_notes", width)
            .page_keeps(design::PANEL_LEAVES_PAGE)
            .frame(
                egui::Frame::new()
                    .fill(colors().page)
                    .stroke(egui::Stroke::new(design::HAIRLINE_WIDTH, colors().rule))
                    .inner_margin(egui::Margin::same(design::M as i8)),
            )
            .show(ctx, |ui| {
                let title = self
                    .selected
                    .and_then(|id| self.notes.iter().find(|note| note.id == id))
                    .map_or_else(|| "Notes".to_owned(), Note::label);
                let date = self.selected.and_then(|id| self.notes.iter().find(|note| note.id == id)).map_or("", |note| note.date.as_str());
                let mut close = false;
                if self.selected.is_none() {
                    close = design::panel_header(ui, &title, "Close notes");
                } else {
                    ui.horizontal_top(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            close = design::icon_button(ui, icons::CLOSE, "Close notes").clicked();
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| note_title(ui, &title, date));
                        });
                    });
                }
                if let Some(draft) = &self.draft { Self::note_status(ui, draft); }
                design::rule(ui);
                if close { self.notes_open = false; return; }
                if self.selected.is_none() {
                    design::message(ui, Tone::Info, "No session yet.");
                    if Action::primary("New session").show(ui).clicked() {
                        self.start_new_session();
                    }
                    return;
                }
                if self.current().is_none() {
                    design::message(
                        ui,
                        Tone::Info,
                        "New events go to no session, so no note is open here. Choose a session on the Timeline page.",
                    );
                    return;
                }
                let today = self.today();
                let note = self
                    .selected
                    .and_then(|id| self.notes.iter().find(|note| note.id == id));
                let mut play = false;
                let mut start = false;
                if let Some(note) = note
                    && let Some(words) = dated_before_today(note, &today)
                {
                        design::message(ui, Tone::Warning, &words);
                        ui.horizontal(|ui| {
                            play = Action::secondary("Play it tonight")
                                .density(Density::Read)
                                .show(ui)
                                .clicked();
                            start = Action::quiet("New session")
                                .density(Density::Read)
                                .show(ui)
                                .clicked();
                        });
                }
                if play {
                    self.play_this_one();
                }
                if start {
                    self.start_new_session();
                }
                self.note_editor(ui, true);
            });
    }

    fn read_tab(&mut self, ui: &mut egui::Ui, book: &Book, plan: &Plan<'_>, beside: bool) {
        let Some(draft) = &self.draft else {
            return;
        };
        let note = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id));
        let body = note.map_or_else(
            || draft.body.clone(),
            |note| without_own_heading(&draft.body, note),
        );
        let compact = compact_plan_for_reading(&body, plan.fights, beside);
        let summary = (!beside && compact != body)
            .then(|| section_text(&compact, "Plan for tonight"))
            .flatten();
        let text = words_to_read(&compact, book);
        egui::ScrollArea::vertical()
            .id_salt(("session_reader", self.selected))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                note_width(ui, |ui| {
                    if !has_written_text(&compact) {
                        ui.horizontal(|ui| {
                            ui.label(
                                Type::Small
                                    .rich(ui, "Nothing written yet.")
                                    .color(colors().muted),
                            );
                            if design::link(ui, "Write").clicked() {
                                self.write_in_note();
                                self.focus_notes = true;
                            }
                        });
                    } else if let Some(summary) = &summary
                        && let Some((before, after)) = text.split_once(summary)
                    {
                        markdown_view::show_reader(ui, before);
                        ui.label(Type::Small.rich(ui, summary).color(colors().muted));
                        markdown_view::show_reader(ui, after);
                    } else {
                        markdown_view::show_reader(ui, &text);
                    }
                });
            });
        if let Some(target) = markdown_view::take_note_link(ui.ctx()) {
            if let Some(id) = target.strip_prefix("Timeline#^") {
                self.wants_event = Some(id.to_owned());
            } else {
                self.wants_lookup = Some(target);
            }
        }
    }

    /// Show the note, ready for words.
    fn write_in_note(&mut self) {
        self.tab = Tab::Note;
        self.reading_note = false;
    }

    /// The note, as one view: words to write, or the same words to read.
    fn note_pane(&mut self, ui: &mut egui::Ui, book: &Book, plan: &Plan<'_>, beside: bool) {
        if self.reading_note {
            self.read_tab(ui, book, plan, beside);
        } else {
            self.note_editor(ui, false);
        }
    }

    /// A run closed, so what was read of any fight may be out of date.
    pub fn forget_fight(&mut self) {
        self.fight = None;
    }

    /// A note's link that no entry answers to. Saying so is better than a link that does nothing.
    pub fn no_entry_named(&mut self, name: &str) {
        self.notice = Some((Tone::Info, format!("No entry is named {name}.")));
        self.notice_session = None;
    }

    /// Words at the end of the plan in this session's note, so that the file holds the plan.
    pub fn add_to_plan(&mut self, name: &str) {
        let (Some(draft), Some(session)) = (&mut self.draft, self.selected) else {
            return;
        };
        insert_in_plan(&mut draft.body, &format!("- [[{name}]]"));
        draft.edited();
        self.mentions
            .insert(session, timeline::referenced_ids(&draft.body));
    }

    /// A saved group has a new name. Every note's link to it follows, so that no plan loses it.
    pub fn rename_links(&mut self, old: &str, new: &str) {
        if let (Some(draft), Some(session)) = (&mut self.draft, self.selected)
            && rename_links(&mut draft.body, old, new)
        {
            draft.edited();
            self.mentions
                .insert(session, timeline::referenced_ids(&draft.body));
        }
        if let Some((_, names)) = &mut self.tonight_links {
            for name in names
                .iter_mut()
                .filter(|name| name.eq_ignore_ascii_case(old))
            {
                new.clone_into(name);
            }
        }
        let kept: Vec<String> = self
            .notes
            .iter()
            .filter(|note| Some(note.id) != self.selected)
            .filter(|note| rename_links_in_file(&note.path, old, new).is_err())
            .map(Note::label)
            .collect();
        self.rename_problem = (!kept.is_empty()).then(|| {
            format!(
                "{} could not be changed, and still links {old}. Change the link to {new}.",
                kept.join(", ")
            )
        });
    }

    /// The names tonight's note links, for the run. The note on screen gives its words not yet
    /// saved. Another session's note on screen does not change what tonight's plan holds.
    pub fn tonight_links(&mut self) -> Vec<String> {
        let tonight = self.current();
        if self.selected == tonight
            && let (Some(id), Some(draft)) = (tonight, &self.draft)
        {
            self.tonight_links = Some((id, linked_names(&draft.body)));
        }
        self.tonight_links
            .as_ref()
            .filter(|(id, _)| Some(*id) == tonight)
            .map_or_else(Vec::new, |(_, names)| names.clone())
    }

    /// Everything the DM wrote, for Lookup. The open note gives the words not yet saved.
    pub fn written(&self, book: &Book) -> Vec<Written> {
        let mut written: Vec<Written> = self
            .notes
            .iter()
            .filter_map(|note| {
                let body = if self.selected == Some(note.id) {
                    self.draft.as_ref().map(|draft| draft.body.clone())
                } else {
                    fs::read_to_string(&note.path)
                        .ok()
                        .map(|text| body_of(&text).to_owned())
                }?;
                let body = without_own_heading(&body, note);
                Some(Written {
                    title: note.label(),
                    detail: format!("Session note · {}", note.date),
                    text: words_to_read(&body, book),
                    place: WrittenIn::Session(note.id),
                })
            })
            .collect();
        let days = timeline::day_numbers(&book.events);
        written.extend(book.events.iter().zip(&days).map(|(event, day)| {
            let summary = event.summary(None);
            Written {
                title: event.label(*day),
                detail: if summary.is_empty() {
                    "Timeline".to_owned()
                } else {
                    format!("Timeline · {summary}")
                },
                text: [
                    event.place.as_str(),
                    event.result.as_str(),
                    event.notes.as_str(),
                ]
                .into_iter()
                .filter(|part| !part.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n\n"),
                place: WrittenIn::Event(event.id.clone()),
            }
        }));
        written
    }

    /// What the session before this one left: where the DM picks up the thread.
    fn last_time_block(&mut self, ui: &mut egui::Ui, _book: &Book) {
        let Some(selected) = self.selected else {
            return;
        };
        if self.last_time.as_ref().map(|(id, _)| *id) != Some(selected) {
            let previous = self
                .notes
                .iter()
                .position(|note| note.id == selected)
                .and_then(|at| self.notes.get(at + 1));
            let found = previous.map(|note| {
                let text = fs::read_to_string(&note.path).unwrap_or_default();
                let body = body_of(&text);
                LastTime {
                    id: note.id,
                    label: note.label(),
                    what_happened: recap_preview(body),
                }
            });
            self.last_time = Some((selected, found));
        }
        let Some((_, Some(last))) = &self.last_time else {
            return;
        };
        let preview = last
            .what_happened
            .as_deref()
            .and_then(|words| words.lines().find(|line| !line.trim().is_empty()))
            .unwrap_or("");
        let full = format!(
            "Last time: {} — {}",
            last.label,
            last.what_happened
                .as_deref()
                .unwrap_or("Nothing written yet.")
        );
        let mut open = false;
        ui.horizontal(|ui| {
            ui.label(Type::Small.rich(ui, "Last time:").color(colors().muted));
            open = design::link(ui, &last.label).on_hover_text(&full).clicked();
            let words = design::words_that_fit(
                ui,
                &if preview.is_empty() {
                    String::new()
                } else {
                    format!("— {preview}")
                },
                &Type::Small.font(ui),
                ui.available_width(),
            );
            ui.label(Type::Small.rich(ui, words).color(colors().muted))
                .on_hover_text(&full);
        });
        if open {
            self.select(Some(last.id));
        }
    }

    fn removed_group_message(
        &self,
        ui: &mut egui::Ui,
        session: Option<Uuid>,
        position: usize,
    ) -> bool {
        let Some((id, at, _, _, height)) = &self.removed_group else {
            return false;
        };
        if *id != session || *at != position {
            return false;
        }
        ui.allocate_ui(egui::vec2(ui.available_width(), *height), |ui| {
            ui.set_min_height(*height);
            design::message_with_action(
                ui,
                Tone::Info,
                "Group removed from this plan.",
                "Undo remove from plan",
            )
        })
        .inner
    }

    /// The session as a list, top to bottom: what happened, then what is still to come. It
    /// replaces a walk between Encounters, Timeline and the note.
    fn plan_pane(&mut self, ui: &mut egui::Ui, book: &mut Book, plan: &Plan<'_>) {
        let session = self.selected;
        let here = self.group_session == session;
        if plan.group_open && here {
            (plan.group_editor.borrow_mut())(ui);
            return;
        }
        let c = colors();
        if let Some((body, events)) = &self.plan_notice_state
            && (self.draft.as_ref().is_none_or(|draft| draft.body != *body)
                || book.events != *events)
        {
            self.deleted_run = None;
            self.row_notice = None;
            self.removed_group = None;
            self.removed_fight = None;
            self.removed_event_in_plan = false;
            self.plan_notice_state = None;
        }
        if self
            .removed_group
            .as_ref()
            .is_some_and(|(id, _, _, after, _)| {
                *id == session && self.draft.as_ref().is_none_or(|draft| draft.body != *after)
            })
        {
            self.removed_group = None;
        }
        problems(ui, book);
        if plan.group_open && self.not_being_played().is_none() {
            let label = self
                .notes
                .iter()
                .find(|note| Some(note.id) == self.group_session)
                .map_or_else(|| "another session".to_owned(), Note::label);
            if design::message_with_action(
                ui,
                Tone::Info,
                &format!("A group is being assembled in the plan of {label}."),
                "Go to it",
            ) {
                self.select(self.group_session);
            }
        }
        if plan.group_discarded && here && design::draft_discarded(ui) {
            self.wants_group_undo = true;
        }
        if let Some(problem) = &self.rename_problem {
            design::message(ui, Tone::Warning, problem);
        }
        let mut added = None;
        egui::TopBottomPanel::bottom(egui::Id::new(("session_plan_add", session)))
            .resizable(false)
            .show_separator_line(false)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                top: design::XS as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                self.plan_messages(ui, book, session, plan);
                added = quick_add(ui, book, session, &PLAN_KINDS, |ui| {
                    self.fight_menu(ui, plan);
                    self.group_menu(ui, plan);
                });
                if let Some(id) = &added {
                    // A long rest and its HP ruling are one operation and one Undo.
                    if let Some(event) = book.events.iter_mut().find(|event| event.id == *id) {
                        event.title = event.kind.word().to_owned();
                    }
                    let next_order = book.next_order();
                    if let Some(event) = book.events.iter_mut().find(|event| event.id == *id)
                        && event.kind == Kind::LongRest
                    {
                        event.restores = true;
                        event.played_order = next_order;
                    }
                    let days = timeline::day_numbers(&book.events);
                    let index = book.events.iter().position(|event| event.id == *id);
                    self.added = Some(Added {
                        id: id.clone(),
                        label: index.map_or_else(String::new, |at| book.events[at].label(days[at])),
                        at: Instant::now(),
                        count: book.events.len(),
                    });
                    self.open_event = None;
                }
            });
        let days = timeline::day_numbers(&book.events);
        let body = self
            .draft
            .as_ref()
            .map_or_else(String::new, |draft| draft.body.clone());
        let mut played: BTreeMap<String, usize> = BTreeMap::new();
        for event in &book.events {
            if event.is_in(session) && event.run.is_some() {
                *played.entry(event.title.to_lowercase()).or_default() += 1;
            }
        }
        let fight_plan = section_text(&body, "Plan for tonight").unwrap_or_default();
        let planned: Vec<&PlanFight> = each_linked_name(&fight_plan)
            .iter()
            .filter_map(|name| {
                plan.fights
                    .iter()
                    .find(|fight| fight.name.eq_ignore_ascii_case(name))
            })
            .filter(|fight| {
                let done = played.entry(fight.name.to_lowercase()).or_default();
                let waiting = *done == 0;
                *done = done.saturating_sub(1);
                waiting
            })
            .collect();
        let planned_groups: Vec<_> = linked_names(&body)
            .iter()
            .filter(|name| {
                !plan
                    .fights
                    .iter()
                    .any(|fight| fight.name.eq_ignore_ascii_case(name))
            })
            .filter_map(|name| {
                plan.groups
                    .iter()
                    .find(|group| group.name.eq_ignore_ascii_case(name))
                    .cloned()
            })
            .collect();
        design::group_heading(
            ui,
            if session == self.current() {
                "Tonight"
            } else {
                "This session"
            },
            book.events
                .iter()
                .filter(|event| event.is_in(session))
                .count()
                + planned.len()
                + planned_groups.len(),
        );
        let mut insert = None;
        let mut remove = None;
        let mut remove_group = None;
        let mut undo_group_remove = false;
        let mut remove_fight = None;
        let mut delete_run = None;
        let mut toggle = None;
        let mut changed = false;
        let removed_row = book.removed_row().filter(|_| self.removed_event_in_plan);
        let mut undo_row_remove = false;
        let event_count = book.events.len();
        let open_event = self.open_event.clone();
        egui::CentralPanel::default().frame(egui::Frame::new()).show_inside(ui, |ui| {
            egui::ScrollArea::vertical().id_salt(("session_plan", session))
                .auto_shrink([false, false]).show(ui, |ui| {
                let width = ui.available_width() - design::M;
                let action_width = [Action::primary("Start fight").icon(icons::RUN), Action::primary("Resume").icon(icons::RUN), Action::quiet("Add to notes"), Action::secondary("Arrivals").icon(icons::GROUP)]
                    .into_iter().map(|action| action.density(Density::Read).natural_width(ui)).fold(0.0, f32::max)
                    + Density::Read.control() + design::S;
                let widths = [width - action_width - design::S, action_width];
                let mut any = false;
                let mut primary_available = plan.open_run.is_none() && !self.confirm_record;
                for (at, event) in book.events.iter_mut().enumerate() {
                    if let Some((removed_at, label)) = &removed_row && *removed_at == at {
                        ui.add_space(design::S);
                        undo_row_remove |= design::message_with_action(ui, Tone::Info, &format!("Removed “{label}”."), "Undo remove");
                    }
                    if !event.is_in(session) { continue; }
                    any = true;
                    let day = days.get(at).copied().unwrap_or(0);
                    let label = event.label(day);
                    let is_open = open_event.as_ref() == Some(&event.id);
                    let running = event.run.is_some() && event.run == plan.open_run;
                    let saved_fight = plan.fights.iter().find(|fight| fight.saved.as_ref()
                        .is_some_and(|saved| saved.id == event.run && event.run.is_some()));
                    let saved = saved_fight.and_then(|fight| fight.saved.as_ref()).filter(|_| !running);
                    let summary = if event.run.is_some() {
                        played_fight_summary(event)
                    } else {
                        event.summary(None)
                    };
                    let row = ui.push_id(event.id.clone(), |ui| {
                        plan_row(ui, &widths, if is_open { icons::DOWN } else { icons::NEXT }, |ui| {
                            if plan_link(ui, &label, is_open).clicked() { toggle = Some(event.id.clone()); }
                        }, |ui| {
                            if running {
                                if design::status_link(ui, Status { icon: icons::RUN,
                                    word: &plan.open_state, color: c.text }).clicked() {
                                    self.wants_back_to_run = true;
                                }
                            } else if let Some(saved) = saved {
                                design::status(ui, Status { icon: icons::RUN,
                                    word: &plan_saved_status(saved), color: c.muted });
                            } else if !summary.is_empty() {
                                wrapped_words(ui, Type::Small, plan_summary(&summary), c.muted);
                            }
                            if event.kind == Kind::LongRest && event.restores {
                                design::status(ui, Status { icon: icons::OK,
                                    word: "Everyone at full", color: c.document.saved });
                            }
                        }, |ui| {
                            if let Some(saved) = saved {
                                design::row_menu(ui, &label, |ui| {
                                    if let Some(fight) = saved_fight
                                        && Action::quiet("Start a new run").blocked(plan.blocked).show(ui).clicked() {
                                        self.wants_fight = Some(fight.path.clone()); ui.close();
                                    }
                                    if Action::quiet("Delete the saved run").blocked(plan.blocked).show(ui).clicked() {
                                        delete_run = Some(saved.clone()); ui.close();
                                    }
                                    if Action::quiet("Remove").name("Remove this event").show(ui).clicked() {
                                        remove = Some(event.id.clone()); ui.close();
                                    }
                                });
                                let action = if std::mem::take(&mut primary_available) { Action::primary("Resume") } else { Action::secondary("Resume") };
                                if action.icon(icons::RUN).name(&format!("Resume: {label}")).blocked(plan.blocked).density(Density::Read).show(ui).clicked() {
                                    self.wants_saved_run = Some(saved.path.clone());
                                }
                            } else {
                                // Overflow is always at the row's right edge.
                                let more = design::icon_button(ui, icons::EXPAND, &format!("More actions for {label}"));
                                egui::Popup::menu(&more).show(|ui| {
                                    if Action::quiet("Remove").name("Remove this event")
                                        .blocked_if(running, "This run is open. Close it first.").show(ui).clicked() {
                                        remove = Some(event.id.clone()); ui.close();
                                    }
                                    if running { wrapped_words(ui, Type::Small, "This run is open. Close it first.", c.muted); }
                                });
                                if !running && event.run.is_none() {
                                    let linked = body.contains(&format!("^{}", event.id));
                                    if linked { design::status(ui, Status { icon: icons::OK, word: "In notes", color: c.document.saved }); }
                                    else if Action::quiet("Add to notes").name(&format!("Add to notes: {label}"))
                                        .density(Density::Read).show(ui).clicked() { insert = Some(timeline::reference(event, day)); }
                                }
                            }
                        });
                        if is_open {
                            plan_details(ui, |ui| {
                            if running {
                                self.fight = None;
                                wrapped_words(ui, Type::Small, "This fight is running. Its final HP shows here when it ends.", c.muted);
                            } else if let Some(run) = event.run {
                                self.fight_block(ui, run, &event.id, plan);
                            } else {
                                changed |= event_fields(ui, event);
                                if event.kind == Kind::LongRest && event.restores
                                    && design::link(ui, "Not at full").clicked() {
                                    event.restores = false;
                                    changed = true;
                                }
                                if design::link(ui, "Timeline").clicked() { self.wants_event = Some(event.id.clone()); }
                            }
                            });
                        }
                        if self.row_notice == Some((session, at)) && self.deleted_run.is_some() {
                            ui.add_space(design::S);
                            self.deleted_run_message(ui);
                        }
                        plan_rule(ui);
                    });
                    // The opened row and the next row stay in reach.
                    if is_open && std::mem::take(&mut self.reveal_event) { row.response.scroll_to_me(Some(egui::Align::TOP)); }
                }
                if let Some((at, label)) = &removed_row && *at >= event_count {
                    ui.add_space(design::S);
                    undo_row_remove |= design::message_with_action(ui, Tone::Info, &format!("Removed “{label}”."), "Undo remove");
                }
                for (position, fight) in planned.iter().enumerate() {
                    self.removed_fight_message(ui, session, position);
                    any = true;
                    let running = plan.open_run.is_some() && plan.open_encounter == Some(fight.name.as_str());
                    ui.push_id((&fight.path, position), |ui| {
                        plan_row(ui, &widths, if self.open_planned.as_ref() == Some(&fight.path) { icons::DOWN } else { icons::NEXT }, |ui| {
                            if plan_link(ui, &fight.name, self.open_planned.as_ref() == Some(&fight.path)).clicked() {
                                self.open_planned = (self.open_planned.as_ref() != Some(&fight.path)).then(|| fight.path.clone());
                            }
                        }, |ui| {
                            if running {
                                if design::status_link(ui, Status { icon: icons::RUN,
                                    word: &plan.open_state, color: c.text }).clicked() {
                                    self.wants_back_to_run = true;
                                }
                            } else if let Some(saved) = &fight.saved {
                                design::status(ui, Status { icon: icons::RUN, word: &plan_saved_status(saved), color: c.muted });
                            } else { wrapped_words(ui, Type::Small, plan_summary(&fight.summary), c.muted); }
                        }, |ui| {
                            design::row_menu(ui, &fight.name, |ui| {
                                if Action::quiet("Remove from tonight").show(ui).clicked() {
                                    remove_fight = Some((position, fight.name.clone())); ui.close();
                                }
                                if fight.saved.is_some() {
                                    if Action::quiet("Start a new run").blocked(plan.blocked).show(ui).clicked() {
                                        self.wants_fight = Some(fight.path.clone()); ui.close();
                                    }
                                    if Action::quiet("Delete the saved run").blocked(plan.blocked).show(ui).clicked() {
                                        delete_run = fight.saved.clone(); ui.close();
                                    }
                                }
                            });
                            if running {
                                // The run strip owns Back to run.
                            } else if let Some(saved) = &fight.saved {
                                let action = if std::mem::take(&mut primary_available) { Action::primary("Resume") } else { Action::secondary("Resume") };
                                if action.icon(icons::RUN).name(&format!("Resume: {}", fight.name)).density(Density::Read).blocked(plan.blocked).show(ui).clicked() {
                                    self.wants_saved_run = Some(saved.path.clone());
                                }
                            } else {
                                let action = if std::mem::take(&mut primary_available) {
                                    Action::primary("Start fight")
                                } else { Action::secondary("Start fight") };
                                if action.icon(icons::RUN).name(&format!("Start fight: {}", fight.name))
                                    .density(Density::Read).blocked(plan.blocked).show(ui).clicked() {
                                    self.wants_fight = Some(fight.path.clone());
                                }
                            }
                        });
                        if self.row_notice == Some((session, event_count + position)) && self.deleted_run.is_some() {
                            ui.add_space(design::S);
                            self.deleted_run_message(ui);
                        }
                        if self.open_planned.as_ref() == Some(&fight.path) {
                            plan_details(ui, |ui| {
                                if design::link(ui, "Read encounter").clicked() { self.wants_lookup = Some(fight.name.clone()); }
                            });
                        }
                        plan_rule(ui);
                    });
                }
                self.removed_fight_message(ui, session, planned.len());
                for (position, group) in planned_groups.iter().enumerate() {
                    undo_group_remove |= self.removed_group_message(ui, session, position);
                    any = true;
                    let row = ui.push_id(group.id, |ui| {
                        let mut read_group = false;
                        let group_action = if plan.arrivals_open { Action::quiet("Arrivals").icon(icons::GROUP) } else { Action::quiet("Edit") };
                        let action_width = group_action.density(Density::Read).natural_width(ui) + Density::Read.control() + design::S;
                        let group_widths = [width - action_width - design::S, action_width];
                        plan_row(ui, &group_widths, icons::GROUP, |ui| {
                            if design::wrapped_link(ui, &group.name).clicked() { read_group = true; }
                        }, |ui| {
                            let summary = format!("{} · {}{} · {}", group.xp_label(), group.member_summary(), if group.members.iter().any(|m| !m.overrides.is_empty()) { " · Changed" } else { "" }, design::reader::human_label(&group.side));
                            let shown = design::words_that_fit(ui, &summary, &Type::Small.font(ui), ui.available_width());
                            ui.label(Type::Small.rich(ui, shown).color(c.muted)).on_hover_text(&summary);
                            if let Some(problem) = &group.problem { wrapped_words(ui, Type::Small, problem, c.result.warning); }
                        }, |ui| {
                            design::row_menu(ui, &group.name, |ui| {
                                if Action::quiet("Remove").name(&format!("Remove group {} from the plan", group.name))
                                    .blocked(plan.group_blocked).show(ui).clicked() {
                                    remove_group = Some((position, group.name.clone(), 0.0)); ui.close();
                                }
                            });
                            if plan.arrivals_open {
                                if Action::quiet("Arrivals").icon(icons::GROUP)
                                    .name(&format!("Go to Arrivals for {}", group.name)).density(Density::Read).show(ui).clicked() {
                                    self.wants_arrivals = true;
                                }
                            } else if Action::quiet("Edit").name(&format!("Edit group {}", group.name))
                                .density(Density::Read).blocked(plan.group_blocked).show(ui).clicked() {
                                self.wants_group = Some(group.path.clone()); self.group_session = self.selected;
                            }
                        });
                        if read_group { self.wants_group = Some(group.path.clone()); self.group_session = self.selected; }
                        plan_rule(ui);
                    });
                    if let Some((at, _, height)) = &mut remove_group && *at == position { *height = row.response.rect.height(); }
                }
                undo_group_remove |= self.removed_group_message(ui, session, planned_groups.len());
                if !any && self.removed_group.is_none() {
                    design::empty_state(ui, icons::SESSIONS, "Nothing planned yet.",
                        "Use Add below to plan a fight, a group, or an event.", |_| {});
                }
            });
        });
        if let Some((position, name)) = remove_fight
            && let Some(draft) = &mut self.draft
        {
            let occurrence = book
                .events
                .iter()
                .filter(|event| {
                    event.is_in(session)
                        && event.run.is_some()
                        && event.title.eq_ignore_ascii_case(&name)
                })
                .count()
                + planned[..position]
                    .iter()
                    .filter(|fight| fight.name.eq_ignore_ascii_case(&name))
                    .count();
            let before = draft.body.clone();
            let after = unlink_planned_fight(&before, &name, occurrence);
            if before != after {
                draft.body = after.clone();
                draft.edited();
                self.removed_fight = Some((session, position, name, before, after.clone()));
                self.plan_notice_state = Some((after, book.events.clone()));
            }
        }
        if undo_group_remove
            && let Some((_, _, before, _, _)) = self.removed_group.take()
            && let Some(draft) = &mut self.draft
        {
            draft.body = before;
            draft.edited();
        }
        if let Some((position, name, height)) = remove_group
            && let Some(draft) = &mut self.draft
        {
            let before = draft.body.clone();
            unlink_group(&mut draft.body, &name);
            draft.edited();
            self.removed_group = Some((session, position, before, draft.body.clone(), height));
            self.plan_notice_state = Some((draft.body.clone(), book.events.clone()));
        }
        if changed {
            book.touch();
        }
        if let Some(saved) = delete_run {
            match crate::encounter_run::DeletedRun::delete(&saved) {
                Ok(deleted) => {
                    self.row_notice = book
                        .events
                        .iter()
                        .position(|event| event.run == saved.id)
                        .or_else(|| {
                            planned
                                .iter()
                                .position(|fight| {
                                    fight.saved.as_ref().is_some_and(|run| run.id == saved.id)
                                })
                                .map(|at| event_count + at)
                        })
                        .map(|at| (session, at));
                    self.plan_notice_state = Some((
                        self.draft
                            .as_ref()
                            .map_or_else(String::new, |draft| draft.body.clone()),
                        book.events.clone(),
                    ));
                    self.deleted_run = Some(deleted);
                    self.wants_runs_refresh = true;
                }
                Err(error) => self.rename_problem = Some(error),
            }
        }
        if let Some(id) = toggle {
            self.fight = None;
            self.show_fight_log = false;
            self.reveal_event = true;
            self.open_event = (self.open_event.as_ref() != Some(&id)).then_some(id);
        }
        if let Some(id) = remove {
            book.remove(&id);
            self.removed_event_in_plan = true;
            self.plan_notice_state = Some((
                self.draft
                    .as_ref()
                    .map_or_else(String::new, |draft| draft.body.clone()),
                book.events.clone(),
            ));
            self.open_event = None;
        }
        if undo_row_remove {
            book.undo_remove();
            self.removed_event_in_plan = false;
            self.plan_notice_state = None;
        }
        if let (Some(link), Some(draft), Some(session)) = (insert, &mut self.draft, session) {
            insert_in_note(&mut draft.body, &link);
            draft.edited();
            self.added = None;
            self.mentions
                .insert(session, timeline::referenced_ids(&draft.body));
        }
    }

    fn removed_fight_message(&mut self, ui: &mut egui::Ui, session: Option<Uuid>, position: usize) {
        let Some((id, at, name, before, after)) = &self.removed_fight else {
            return;
        };
        if *id != session
            || *at != position
            || self.draft.as_ref().is_none_or(|draft| draft.body != *after)
        {
            return;
        }
        ui.add_space(design::S);
        if design::message_with_action(
            ui,
            Tone::Info,
            &format!("Removed “{name}” from tonight."),
            "Undo remove",
        ) {
            if let Some(draft) = &mut self.draft {
                draft.body = before.clone();
                draft.edited();
            }
            self.removed_fight = None;
            self.plan_notice_state = None;
        }
    }

    fn deleted_run_message(&mut self, ui: &mut egui::Ui) {
        if let Some(deleted) = &self.deleted_run {
            let words = deleted.message();
            if design::message_with_action(
                ui,
                Tone::Info,
                &words,
                crate::encounter_run::DeletedRun::UNDO_LABEL,
            ) {
                match deleted.restore() {
                    Ok(()) => {
                        self.deleted_run = None;
                        self.wants_runs_refresh = true;
                    }
                    Err(error) => self.rename_problem = Some(error),
                }
            }
        }
    }

    /// What can be undone in the plan: the event just added or just removed.
    fn plan_messages(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
    ) {
        if self.row_notice.is_none() {
            self.deleted_run_message(ui);
        }
        let days = timeline::day_numbers(&book.events);
        // It goes at the DM's next edit, or soon, so that it cannot undo the wrong thing.
        if let Some(just) = &self.added {
            let current = book
                .events
                .iter()
                .position(|event| event.id == just.id)
                .map(|at| book.events[at].label(days[at]));
            if just.at.elapsed() >= UNDO_ADD_FOR
                || just.count != book.events.len()
                || current.as_ref() != Some(&just.label)
            {
                self.added = None;
            } else {
                ui.ctx().request_repaint_after(UNDO_ADD_FOR);
            }
        }
        if let Some(just) = &self.added {
            let to = self
                .notes
                .iter()
                .find(|note| Some(note.id) == session)
                .map_or_else(String::new, |note| format!(" to Session {}", note.number));
            let restores = book
                .events
                .iter()
                .any(|event| event.id == just.id && event.restores);
            let (id, words) = (
                just.id.clone(),
                if restores {
                    let names =
                        plan.fights
                            .iter()
                            .filter(|fight| {
                                fight.saved.as_ref().is_some_and(|saved| {
                                    !plan_saved_status(saved).starts_with("Setup")
                                })
                            })
                            .map(|fight| format!("{} keeps its saved HP.", fight.name))
                            .collect::<Vec<_>>();
                    if names.is_empty() {
                        "Long rest added. Everyone starts the next fight at full.".to_owned()
                    } else {
                        format!(
                            "Long rest added. New fights start at full. {}",
                            names.join(" ")
                        )
                    }
                } else {
                    format!("{} added{to}.", just.label)
                },
            );
            if design::message_with_action(ui, Tone::Success, &words, "Undo") {
                book.remove(&id);
                self.added = None;
            }
        } else if !self.removed_event_in_plan {
            undo_removed(ui, book);
        }
    }

    /// Add a fight to the plan: one the campaign has, or a new one.
    fn fight_menu(&mut self, ui: &mut egui::Ui, plan: &Plan<'_>) {
        let names: Vec<&str> = plan
            .fights
            .iter()
            .map(|fight| fight.name.as_str())
            .collect();
        let menu = PlanMenu {
            word: "Fight",
            icon: icons::ENCOUNTERS,
            name: "Add a fight",
            new: "New encounter",
            blocked: plan.blocked,
            find: "Find a fight",
            hint: "Find a fight…",
        };
        match plan_menu(
            ui,
            &menu,
            &names,
            &mut self.fight_search,
            &mut self.focus_fight_search,
        ) {
            Some(MenuChoice::New) => self.wants_new_fight = true,
            Some(MenuChoice::Named(name)) => self.add_to_plan(&name),
            None => {}
        }
    }

    /// Add a group to the plan: one the campaign has, or a new one assembled here.
    fn group_menu(&mut self, ui: &mut egui::Ui, plan: &Plan<'_>) {
        let names: Vec<&str> = plan
            .groups
            .iter()
            .map(|group| group.name.as_str())
            .collect();
        let menu = PlanMenu {
            word: "Group",
            icon: icons::GROUP,
            name: "Add a group",
            new: "New group",
            blocked: plan.group_blocked,
            find: "Find a group",
            hint: "Find a group…",
        };
        match plan_menu(
            ui,
            &menu,
            &names,
            &mut self.group_search,
            &mut self.focus_group_search,
        ) {
            Some(MenuChoice::New) => {
                self.wants_new_group = true;
                self.group_session = self.selected;
            }
            Some(MenuChoice::Named(name)) => self.add_to_plan(&name),
            None => {}
        }
    }

    /// How a fight ended, in place: final HP and the log, with no run to reopen.
    fn fight_block(&mut self, ui: &mut egui::Ui, run: Uuid, event_id: &str, plan: &Plan<'_>) {
        let path = plan
            .run_directory
            .as_ref()
            .map(|directory| directory.join(format!("{run}.json")));
        if self.fight.as_ref().map(|(id, _)| *id) != Some(run) {
            let result = match (&path, &self.campaign) {
                (Some(path), Some(campaign)) => fight_summary(path, campaign.id),
                _ => Err("The run folder is not available.".to_owned()),
            };
            self.fight = Some((run, result));
        }
        let Some((_, result)) = &self.fight else {
            return;
        };
        match result {
            Err(error) => {
                design::message(
                    ui,
                    Tone::Info,
                    &format!("This fight's run cannot be read. {error}"),
                );
            }
            Ok(summary) => {
                let known = |n: Option<i64>| n.map_or_else(|| "?".to_owned(), |n| n.to_string());
                let mut cells = Vec::new();
                let mut widest = 0.0_f32;
                for row in summary.rows.iter().filter(|row| row.player) {
                    let first = row.name.split_whitespace().next().unwrap_or(&row.name);
                    let hp = format!("{}/{}", known(row.hp), known(row.max_hp));
                    let mut detail = format!("{}: {hp} HP", row.name);
                    if row.temporary_hp > 0 {
                        detail.push_str(&format!(" · {}", design::temp_hp_text(row.temporary_hp)));
                    }
                    if !row.present {
                        detail.push_str(" · Absent");
                    }
                    for condition in &row.conditions {
                        detail.push_str(&format!(" · {condition}"));
                    }
                    let mut words = egui::text::LayoutJob::default();
                    let format = |color| egui::TextFormat {
                        font_id: Type::Small.font(ui),
                        color,
                        ..Default::default()
                    };
                    words.append(first, 0.0, format(colors().muted));
                    words.append(
                        &format!(" {hp}"),
                        0.0,
                        format(design::hp_color(design::hp_state(row.hp, row.max_hp))),
                    );
                    if row.temporary_hp > 0 {
                        words.append(
                            &format!(" {}", design::temp_hp_text(row.temporary_hp)),
                            0.0,
                            format(colors().hp.temp),
                        );
                    }
                    widest = widest.max(ui.painter().layout_job(words.clone()).size().x);
                    cells.push((words, detail));
                }
                let across = if 3.0 * widest + 2.0 * design::S <= ui.available_width() {
                    3
                } else {
                    2
                };
                let cell_width =
                    (ui.available_width() - (across - 1) as f32 * design::S) / across as f32;
                egui::Grid::new(("played_party_hp", run))
                    .num_columns(across)
                    .min_col_width(cell_width)
                    .max_col_width(cell_width)
                    .min_row_height(Type::Small.line())
                    .spacing(egui::vec2(design::S, design::XS))
                    .show(ui, |ui| {
                        for (at, (words, detail)) in cells.into_iter().enumerate() {
                            ui.add(egui::Label::new(words).wrap()).on_hover_text(detail);
                            if (at + 1) % across == 0 {
                                ui.end_row();
                            }
                        }
                    });
                ui.horizontal(|ui| {
                    // Padding gives this short word a target of at least 24 px.
                    let log = design::link(ui, " Log ");
                    log.widget_info(|| {
                        egui::WidgetInfo::labeled(egui::WidgetType::Link, true, "Log")
                    });
                    if log.clicked() {
                        self.show_fight_log = !self.show_fight_log;
                    }
                    ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                    if design::link(ui, "Whole run").clicked() {
                        self.wants_saved_run = path;
                    }
                    ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                    if design::link(ui, "Timeline").clicked() {
                        self.wants_event = Some(event_id.to_owned());
                    }
                });
                if self.show_fight_log {
                    egui::ScrollArea::vertical()
                        .id_salt(("fight_log", run))
                        .max_height(design::READER_MEASURE / 3.0)
                        .show(ui, |ui| {
                            for line in &summary.log {
                                wrapped_words(ui, Type::Small, line, colors().muted);
                            }
                        });
                }
            }
        }
    }

    fn recording_row(
        &mut self,
        ui: &mut egui::Ui,
        recording: &Recording,
        settings: &ProcessingSettings,
        book: &Book,
    ) {
        let c = colors();
        let live = self
            .live
            .as_ref()
            .is_some_and(|live| live.recorder.path() == recording.path);
        let length = recording
            .seconds
            .map_or_else(|| "Unknown length".to_owned(), spoken_length);
        let mut delete = false;
        let mut again = false;
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                design::row_menu(ui, &recording.title, |ui| {
                    if recording.summary
                        && Action::quiet("Process again…")
                            .name(&format!("Process again: {}", recording.name))
                            .blocked(if live {
                                Some("Stop recording first.")
                            } else if self.job.is_some() {
                                Some("Wait for processing to finish.")
                            } else {
                                None
                            })
                            .show(ui)
                            .clicked()
                    {
                        again = true;
                        ui.close();
                    }
                    let busy = live
                        || self
                            .job
                            .as_ref()
                            .is_some_and(|(job, _)| job.recording == recording.path);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        delete = Action::quiet("Delete…")
                            .name(&format!("Delete recording: {}", recording.name))
                            .density(Density::Read)
                            .blocked_if(busy, "This recording is in use.")
                            .show(ui)
                            .clicked();
                        if busy {
                            ui.label(
                                Type::Small
                                    .rich(ui, "This recording is in use.")
                                    .color(c.muted),
                            );
                        }
                    });
                    if delete {
                        ui.close();
                    }
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Label::new(Type::Strong.rich(ui, &recording.title).color(c.text))
                            .wrap(),
                    );
                });
            });
        });
        let processing = self
            .job
            .as_ref()
            .is_some_and(|(job, _)| job.recording == recording.path);
        ui.label(
            Type::Small
                .rich(ui, format!("{length} · {}", megabytes(recording.bytes)))
                .color(c.muted),
        );
        let stored = &recording.status;
        let (icon, word, color) = if live {
            self.recording_status()
                .unwrap_or((icons::RECORD, "Recording", c.recording))
        } else if processing {
            (icons::BUSY, "Processing", c.connection.busy)
        } else if let Some(words) = stored.as_deref() {
            let state = if words.starts_with("Failed") {
                "Failed"
            } else if words.starts_with("Processing cancelled") {
                "Cancelled"
            } else {
                "Stopped"
            };
            if state == "Cancelled" {
                (icons::WARN, state, c.result.warning)
            } else {
                (icons::ERROR, state, c.result.error)
            }
        } else if recording.warning.is_some() {
            {
                let status = processed_warning();
                (status.icon, status.word, status.color)
            }
        } else if recording.summary {
            (icons::OK, "Processed", c.result.success)
        } else if recording.transcript {
            (icons::DRAFT, "Partly processed", c.muted)
        } else {
            (icons::DOCUMENT, "Not processed", c.muted)
        };
        if !processing {
            design::status(ui, Status { icon, word, color }).on_hover_text(word);
        }
        let stalled = live
            && self.live.as_ref().is_some_and(|live| {
                matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
            });
        if stalled && let Some(live) = &self.live {
            ui.add(egui::Label::new(Type::Small.rich(ui, format!("Not saving audio after {}. What was saved is kept. Record again starts a new file.", spoken_length(live.recorder.seconds()))).color(c.muted)).wrap());
        }
        if let Some(words) = stored.as_ref().filter(|_| !live && !processing) {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, recording_detail(words)).color(c.muted))
                        .wrap(),
                );
                if words.starts_with("Stopped by itself:")
                    && ui
                        .add(egui::Link::new(Type::Small.rich(ui, " Settings")))
                        .clicked()
                {
                    self.wants_recording_input = true;
                }
            });
        }
        if let Some(warning) = recording.warning.as_ref().filter(|_| !live && !processing) {
            design::message(ui, Tone::Warning, warning);
        }
        let job_here = self
            .job
            .as_ref()
            .is_some_and(|(job, _)| job.recording == recording.path);
        let mut open = None;
        let views: Vec<Made> = [
            (Made::Summary, recording.summary),
            (Made::Marked, recording.marked),
            (Made::Review, recording.review && !recording.check_missing()),
            (Made::Transcript, recording.transcript),
        ]
        .into_iter()
        .filter_map(|(made, exists)| exists.then_some(made))
        .collect();
        if !views.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for made in views {
                    if design::link(ui, made.word())
                        .on_hover_text(format!("Read this for {}", recording.name))
                        .clicked()
                    {
                        open = Some(made);
                    }
                    if made == Made::Transcript && recording.check_missing() {
                        ui.label(Type::Small.rich(ui, "No check ran").color(c.muted));
                    }
                }
            });
        }
        let mut process = false;
        let mut stop = false;
        let mut record_again = false;
        ui.horizontal_wrapped(|ui| {
            if stalled {
                record_again = Action::secondary("Record again").show(ui).clicked();
            }
            if !live
                && !job_here
                && stored
                    .as_ref()
                    .is_some_and(|words| words.starts_with("Stopped by itself:"))
            {
                record_again = Action::secondary("Record again")
                    .density(Density::Read)
                    .blocked_if(
                        self.live.is_some() || self.job.is_some(),
                        "Stop recording or wait for processing to finish.",
                    )
                    .show(ui)
                    .clicked();
            }
            if live {
                stop = Action::secondary("Stop recording")
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
            }
            if job_here {
                let words = self
                    .job
                    .as_ref()
                    .map(|(job, _)| job.progress.words())
                    .unwrap_or_default();
                {
                    design::spinner(ui);
                    ui.label(
                        Type::Small
                            .rich(ui, format!("Processing · {words}"))
                            .color(c.muted),
                    );
                    let cancelling = self.job.as_ref().is_some_and(|(job, _)| job.cancelling());
                    if cancelling {
                        ui.label(
                            Type::Small
                                .rich(ui, "Cancelling. This ends after the current step.")
                                .color(c.muted),
                        );
                    } else if Action::quiet("Cancel")
                        .name("Cancel processing")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                        && let Some((job, _)) = &self.job
                    {
                        job.cancel();
                    }
                }
            }
            let reason = if live {
                Some("Stop recording first.".to_owned())
            } else if self.job.is_some() {
                Some("One recording is processed at a time.".to_owned())
            } else {
                settings.problem()
            };
            if !job_here && !recording.summary {
                let label = if recording.summary {
                    "Process again"
                } else if recording.transcript
                    || recording.status.as_ref().is_some_and(|words| {
                        words.starts_with("Failed") || words.starts_with("Processing cancelled")
                    })
                {
                    "Continue processing"
                } else {
                    "Process"
                };
                let pressed = Action::secondary(label)
                    .name(&format!("{label}: {}", recording.name))
                    .density(Density::Read)
                    .blocked(reason.as_deref())
                    .show(ui)
                    .clicked();
                if live {
                    design::blocked_reason(ui, Some("Stop recording first."));
                }
                if let Some(reason) = reason.as_ref().filter(|_| !live && self.job.is_none()) {
                    let speech = processing::host(&settings.config.speech_url);
                    let named = if reason.starts_with("Save an API key for this speech service") {
                        format!(
                            "Save an {} key in Settings",
                            if speech == "api.openai.com" {
                                "OpenAI"
                            } else {
                                &speech
                            }
                        )
                    } else {
                        reason.clone()
                    };
                    if design::link(ui, &named).clicked() {
                        self.wants_processing_settings = true;
                    }
                }
                process = pressed && !recording.summary;
                ui.add_space(design::L);
            }
        });
        if self.confirm_again.as_deref() == Some(recording.path.as_path()) {
            design::message(
                ui,
                Tone::Info,
                &format!(
                    "This sends the raw transcript, this session's name, notes and events to {} twice. The audio is not sent again. It replaces the summary, the clean transcript and what the check changed. Events you added stay on the timeline.",
                    processing_llm(settings)
                ),
            );
            ui.horizontal(|ui| {
                if Action::secondary("Not now").show(ui).clicked() {
                    self.confirm_again = None;
                }
                ui.add_space(design::L);
                process = Action::destructive("Replace the results")
                    .show(ui)
                    .clicked();
            });
        }
        if stop {
            self.finish_recording();
        }
        if record_again && let Some(session) = self.selected {
            if stalled {
                self.finish_recording();
            }
            if let Err(error) = self.start_recording(settings, session) {
                self.notice = Some((Tone::Error, error));
            }
        }
        if again {
            self.confirm_again = Some(recording.path.clone());
        }
        if process {
            self.confirm_again = None;
            let path = recording.path.clone();
            self.start_processing(ui.ctx(), &path, settings, book);
        }
        if delete {
            self.confirm_delete = Some(recording.path.clone());
        }
        if let Some(made) = open {
            self.open_reading(recording, made, None, None);
        }
    }

    /// A message of success has done its work once the DM has moved on.
    fn clear_success(&mut self) {
        // News of another session's job waits until the DM has seen it.
        if matches!(self.notice, Some((Tone::Success, _)))
            && (self.notice_session.is_none() || self.notice_session == self.selected)
        {
            self.notice = None;
            self.notice_session = None;
        }
    }

    /// Read a text once, and open it at a time, or at its start.
    fn open_reading(
        &mut self,
        recording: &Recording,
        made: Made,
        at: Option<&str>,
        marker: Option<(crate::timeline::Marker, String)>,
    ) {
        let mark = marker.as_ref().map(|(marker, _)| marker.reading_words());
        let text = fs::read_to_string(made.path(&recording.path)).unwrap_or_default();
        let body = timeline::markers_for_reading(body_of(&text));
        let parts = parts_of(&body);
        self.clear_success();
        // The last part that starts at or before the time. A time is not always a heading.
        let part = at
            .and_then(|at| parts.iter().rposition(|(time, _)| time.as_str() <= at))
            .unwrap_or(0);
        self.reading = Some(Reading {
            recording: recording.path.clone(),
            name: recording.title.clone(),
            made,
            parts,
            part,
            mark,
            marker,
            short_of: matches!(made, Made::Transcript)
                .then(|| processing::transcript_short_of(&recording.path))
                .flatten(),
        });
    }

    fn markers(&mut self, ui: &mut egui::Ui, recording: &Recording, book: &mut Book) {
        if recording.markers.is_empty() {
            return;
        }
        let c = colors();
        let session = self.selected;
        ui.add_space(design::L);
        design::group_heading(
            ui,
            "Events marked in this recording",
            recording.markers.len(),
        );
        ui.label(
            Type::Small
                .rich(
                    ui,
                    "These are suggestions from the LLM. Only what you add goes on the timeline.",
                )
                .color(c.muted),
        );
        let action = 120.0;
        let widths = [ui.available_width() - action - design::XL, action];
        let mut read_at: Option<(String, crate::timeline::Marker, String)> = None;
        let days = timeline::day_numbers(&book.events);
        for (index, marker) in recording.markers.iter().enumerate() {
            let title = if marker.title.is_empty() {
                marker.kind.word()
            } else {
                marker.title.as_str()
            };
            let key = marker.key(&recording.name);
            let added = book.event_of_marker(&key).map(|event| event.id.clone());
            let mut facts = vec![marker.kind.word()];
            for fact in [&marker.duration, &marker.place] {
                if !fact.is_empty() {
                    facts.push(fact);
                }
            }
            design::table_row(ui, &widths, design::S, |ui, cell| {
                if cell == 0 {
                    ui.horizontal_top(|ui| {
                        ui.label(Type::Body.rich(ui, marker.kind.icon()).color(c.muted));
                        ui.vertical(|ui| {
                            ui.add(
                                egui::Label::new(Type::Body.rich(ui, title).color(c.text)).wrap(),
                            );
                            ui.horizontal_wrapped(|ui| {
                                if !marker.at.is_empty()
                                    && recording.marked
                                    && design::link(ui, &marker.at)
                                        .on_hover_text("Read the transcript here")
                                        .clicked()
                                {
                                    read_at =
                                        Some((marker.at.clone(), marker.clone(), key.clone()));
                                }
                                let mut facts = facts.join(" · ");
                                if let Some(id) = &added {
                                    let at = book.events.iter().position(|event| event.id == *id);
                                    let before = at.and_then(|at| at.checked_sub(1));
                                    facts.push_str(&before.map_or_else(
                                        || " · First on the timeline".to_owned(),
                                        |before| {
                                            format!(
                                                " · After {}",
                                                book.events[before].label(days[before])
                                            )
                                        },
                                    ));
                                }
                                ui.add(
                                    egui::Label::new(Type::Small.rich(ui, facts).color(c.muted))
                                        .wrap(),
                                );
                            });
                        });
                    });
                } else {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if let Some(id) = &added {
                            if design::status_link(
                                ui,
                                Status {
                                    icon: icons::OK,
                                    word: "On the timeline",
                                    color: c.result.success,
                                },
                            )
                            .clicked()
                            {
                                self.wants_event = Some(id.clone());
                            }
                        } else if Action::secondary("Add")
                            .name(&format!("Add to timeline: {title} ({index})"))
                            .density(Density::Read)
                            .blocked_if(
                                book.unreadable.is_some(),
                                "The timeline file must be repaired first.",
                            )
                            .show(ui)
                            .clicked()
                        {
                            book.add_marker(marker, &key, session);
                        }
                    });
                }
            });
        }
        if let Some((at, marker, key)) = read_at {
            self.open_reading(recording, Made::Marked, Some(&at), Some((marker, key)));
        }
    }

    /// One text from a recording, in the whole pane, one part at a time. A 5-hour transcript
    /// is about 30 parts of a size the Reader handles.
    fn reading_view(&mut self, ui: &mut egui::Ui, book: &mut Book) {
        let c = colors();
        let session = self.selected;
        let Some(reading) = &self.reading else {
            return;
        };
        let path = reading.recording.clone();
        let mut made = reading.made;
        let options: Vec<_> = [Made::Summary, Made::Marked, Made::Review, Made::Transcript]
            .into_iter()
            .filter(|made| {
                made.path(&path).exists()
                    && (*made != Made::Review
                        || self
                            .recordings
                            .iter()
                            .find(|recording| recording.path == path)
                            .is_none_or(|recording| !recording.check_missing()))
            })
            .map(|made| (made, made.word()))
            .collect();
        let mut back = false;
        let mut add_to_notes = None;
        let in_notes = self.draft.as_ref().map_or("", |draft| draft.body.as_str());
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if reading.made == Made::Summary {
                    let summary = nested_in_note(reading.parts[0].1.trim());
                    if in_notes.contains(summary.trim()) {
                        design::status(
                            ui,
                            Status {
                                icon: icons::OK,
                                word: "In notes",
                                color: c.document.saved,
                            },
                        );
                    } else if Action::secondary("Add to notes")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        add_to_notes = Some(summary);
                    }
                }
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    back = Action::quiet("Recordings")
                        .icon(icons::BACK)
                        .name("Back to recordings")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                    ui.label(Type::Small.rich(ui, &reading.name).color(c.muted));
                });
            });
        });
        // Reserve each segment's full width before choosing the control. Its row never wraps.
        let segments_width: f32 =
            options
                .iter()
                .map(|(_, label)| {
                    design::number_width(ui, label, &Type::Strong.font(ui))
                        .max(design::number_width(ui, label, &Type::Body.font(ui)))
                        + 2.0 * design::S
                })
                .sum();
        let switched = if segments_width + design::XS <= ui.available_width() {
            design::segmented(ui, &mut made, &options)
        } else {
            let inner = egui::ComboBox::from_id_salt("recording_view")
                .selected_text(made.word())
                .show_ui(ui, |ui| {
                    for (option, label) in &options {
                        ui.selectable_value(&mut made, *option, *label);
                    }
                });
            design::named(ui, &inner.response, "Recording view");
            made != reading.made
        };
        if switched
            && let Some(recording) = self
                .recordings
                .iter()
                .find(|recording| recording.path == path)
                .cloned()
        {
            self.open_reading(&recording, made, None, None);
        }
        let Some(reading) = &mut self.reading else {
            return;
        };
        if let Some((done, parts)) = reading.short_of {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "Transcribed so far: {done} of {parts} parts. Continue processing adds the rest."
                ),
            );
        }
        if matches!(reading.made, Made::Summary | Made::Marked | Made::Review) {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        "This is a draft from an LLM. Check it against your own memory.",
                    )
                    .color(c.muted),
            );
        }
        let count = reading.parts.len();
        if count > 1 {
            ui.horizontal(|ui| {
                if Action::quiet("Earlier")
                    .icon(icons::UP)
                    .name("Earlier part")
                    .density(Density::Read)
                    .blocked_if(reading.part == 0, "This is the first part.")
                    .show(ui)
                    .clicked()
                {
                    reading.part -= 1;
                }
                ui.label(
                    Type::Small
                        .rich(ui, format!("Part {} of {count}", reading.part + 1))
                        .color(c.muted),
                );
                let inner = egui::ComboBox::from_id_salt("recording_part")
                    .selected_text(&reading.parts[reading.part].0)
                    .width(120.0)
                    .icon(design::combo_icon)
                    .show_ui(ui, |ui| {
                        for (at, (time, _)) in reading.parts.iter().enumerate() {
                            ui.selectable_value(&mut reading.part, at, time);
                        }
                    });
                design::named(ui, &inner.response, "Go to a time");
                if Action::quiet("Later")
                    .icon(icons::DOWN)
                    .name("Later part")
                    .density(Density::Read)
                    .blocked_if(reading.part + 1 == count, "This is the last part.")
                    .show(ui)
                    .clicked()
                {
                    reading.part += 1;
                }
            });
        }
        ui.add_space(design::XS);
        egui::ScrollArea::vertical()
            .id_salt((
                "recording_text",
                &reading.recording,
                reading.made.word(),
                reading.part,
            ))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                note_width(ui, |ui| {
                    let body = &reading.parts[reading.part].1;
                    let body = body.replace(
                        "> This is a draft from an LLM. Check it against your own memory.\n",
                        "",
                    );
                    let heading = format!("## {}", reading.parts[reading.part].0);
                    let body = if count > 1 {
                        body.trim_start()
                            .strip_prefix(&heading)
                            .filter(|rest| rest.starts_with('\n'))
                            .map(str::trim_start)
                            .unwrap_or(&body)
                    } else {
                        &body
                    };
                    let at = reading
                        .marker
                        .as_ref()
                        .map(|(marker, _)| marker.reading_words())
                        .as_deref()
                        .or(reading.mark.as_deref())
                        .and_then(|mark| body.find(mark));
                    match at {
                        Some(at) => {
                            markdown_view::show_reader(ui, &body[..at]);
                            if reading.mark.is_some() {
                                ui.scroll_to_cursor(Some(egui::Align::TOP));
                            }
                            if let Some((marker, key)) = &reading.marker {
                                let line_end =
                                    body[at..].find('\n').map_or(body.len(), |end| at + end);
                                ui.horizontal_wrapped(|ui| {
                                    markdown_view::show_reader(ui, &body[at..line_end]);
                                    if book.event_of_marker(key).is_some() {
                                        design::status(
                                            ui,
                                            Status {
                                                icon: icons::OK,
                                                word: "On the timeline",
                                                color: c.document.saved,
                                            },
                                        );
                                    } else if Action::secondary("Add")
                                        .name("Add this event to the timeline")
                                        .blocked_if(
                                            book.unreadable.is_some(),
                                            "The timeline file must be repaired first.",
                                        )
                                        .show(ui)
                                        .clicked()
                                    {
                                        book.add_marker(marker, key, session);
                                    }
                                });
                                markdown_view::show_reader(ui, &body[line_end..]);
                            } else {
                                markdown_view::show_reader(ui, &body[at..]);
                            }
                        }
                        None => markdown_view::show_reader(ui, body),
                    }
                });
                reading.mark = None;
                // A part fills several screens, so the way on is also where the reading ends.
                ui.add_space(design::S);
                if reading.part + 1 < count
                    && Action::secondary("Go on to the next part")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                {
                    reading.part += 1;
                }
            });
        if back {
            self.reading = None;
        }
        if let Some(body) = add_to_notes
            && let Some(draft) = &mut self.draft
        {
            insert_in_note(&mut draft.body, &body);
            draft.edited();
            self.notice_session = None;
            self.notice = Some((Tone::Success, "Added to the notes.".to_owned()));
        }
    }

    fn recordings_tab(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
    ) {
        if self.reading.is_some() {
            self.reading_view(ui, book);
            return;
        }
        let offers_process = self.recordings.iter().any(|recording| {
            !recording.summary
                && self
                    .live
                    .as_ref()
                    .is_none_or(|live| live.recorder.path() != recording.path)
                && self
                    .job
                    .as_ref()
                    .is_none_or(|(job, _)| job.recording != recording.path)
        });
        if offers_process
            && self.live.is_none()
            && self.job.is_none()
            && settings.problem().is_none()
        {
            ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(ui, processing_destination(settings))
                            .color(colors().muted),
                    )
                    .wrap(),
                );
            });
        }
        let recordings = self.recordings.clone();
        if recordings.is_empty() {
            ui.add_space(design::M);
            let here = self
                .live
                .as_ref()
                .is_some_and(|live| Some(live.session) == self.selected);
            let reason = self.record_blocked(settings);
            let elsewhere = self.live.as_ref().map(|live| live.label.clone());
            let words = if here {
                "The recorder is starting. Its file appears here in a moment.".to_owned()
            } else if let Some(label) = elsewhere {
                format!("No recordings of this session. {label} is recording now.")
            } else if let Some(reason) = &reason {
                format!("No recordings of this session. {reason}")
            } else {
                "No recordings of this session. Select Record above to make one.".to_owned()
            };
            design::empty_state(ui, icons::RECORD, "Session recordings", &words, |_| {});
            if !here
                && self.live.is_none()
                && reason.is_some()
                && Action::quiet("Open Settings").show(ui).clicked()
            {
                self.wants_processing_settings = true;
            }
            return;
        }
        egui::ScrollArea::vertical()
            .id_salt(("session_recordings", self.selected))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
                for recording in &recordings {
                    ui.add_space(design::S);
                    ui.scope_builder(
                        egui::UiBuilder::new().id(("recording_row", &recording.path)),
                        |ui| {
                            self.recording_row(ui, recording, settings, book);
                        },
                    );
                    self.markers(ui, recording, book);
                    // Each marker row ends in a rule already.
                    if recording.markers.is_empty() {
                        design::rule(ui);
                    }
                }
            });
    }

    fn delete_dialog(&mut self, ctx: &egui::Context) {
        let Some(path) = self.confirm_delete.clone() else {
            return;
        };
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut delete = false;
        design::dialog(ctx, "delete_recording", "Delete this recording?", |ui| {
            ui.add(
                egui::Label::new(format!(
                    "This removes {} and what was made from it from the campaign folder. You cannot undo this. The timeline stays as it is.",
                    recording_title(&path)
                ))
                .wrap(),
            );
            ui.add_space(design::S);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), Density::Read.control()),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    delete = Action::destructive("Delete recording").show(ui).clicked();
                    ui.add_space(design::L);
                    keep |= Action::secondary("Keep the recording").show(ui).clicked();
                },
            );
        });
        if keep {
            self.confirm_delete = None;
        }
        if delete {
            self.confirm_delete = None;
            self.reading = None;
            self.notice_session = None;
            self.notice = Some(match delete_recording(&path) {
                Ok(()) => (Tone::Success, "Recording deleted.".to_owned()),
                Err(error) => (Tone::Error, format!("Not deleted. {error}")),
            });
            self.refresh_recordings();
        }
    }

    fn session_details_action(&mut self, ui: &mut egui::Ui, note: &Note) {
        if Action::quiet("Details")
            .name("Edit session details")
            .show(ui)
            .clicked()
        {
            self.form = Some(Form {
                editing: Some(note.id),
                number: note.number.to_string(),
                title: note.title.clone(),
                date: note.date.clone(),
                template: String::new(),
                error: None,
            });
        }
        if let Some(draft) = &self.draft {
            Self::note_status(ui, draft);
        }
    }

    fn session_pane(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
    ) {
        let c = colors();
        let Some(note) = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .cloned()
        else {
            // Its file went from the folder, and the words are still here. The same
            // header as a note, so that the page does not jump at the next scan.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                self.record_control(ui, settings);
                if let Some(draft) = &self.draft {
                    ui.allocate_ui_with_layout(
                        egui::vec2(STATUS_WIDTH, Density::Scan.control()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| Self::note_status(ui, draft),
                    );
                    let name = draft.path.file_stem().unwrap_or_default().to_string_lossy();
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        ui.add(egui::Label::new(Type::Title.rich(ui, name).color(c.text)).wrap());
                    });
                }
            });
            self.record_line(ui, settings);
            if let Some(draft) = &mut self.draft {
                Self::draft_problem(ui, draft);
                self.note_editor(ui, false);
            }
            return;
        };
        let title_on_own_line = ui.available_width() < TWO_COLUMNS_FROM;
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.record_control(ui, settings);
                if !title_on_own_line
                    || self
                        .draft
                        .as_ref()
                        .is_some_and(|draft| draft.state == DraftState::Conflict)
                {
                    self.session_details_action(ui, &note);
                }
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    note_title(ui, &note.label(), &note.date);
                });
            });
        });
        self.record_line(ui, settings);
        if let Some(draft) = &mut self.draft {
            Self::draft_problem(ui, draft);
        }
        self.expire_move(book);
        let shown = self.notice_session.is_none() || self.notice_session == self.selected;
        if shown && self.job_done {
            self.job_done = false;
            self.job_warned = false;
        }
        if let Some((tone, words)) = self.notice.clone().filter(|_| shown) {
            if let Some(session) = self
                .move_run_here
                .as_ref()
                .filter(|(session, notice)| {
                    Some(*session) == self.selected && *notice == words && plan.open_run.is_some()
                })
                .map(|(session, _)| *session)
            {
                if design::message_with_action(ui, tone, &words, "Move it here")
                    && let Some(run) = plan.open_run
                {
                    self.move_run_to_session(book, run, session);
                }
            } else if self.moved_run.is_some() {
                if design::message_with_action(ui, tone, &words, "Undo move") {
                    self.undo_move(book);
                }
            } else {
                ui.horizontal_top(|ui| {
                    let button = Density::Read.control() + design::S;
                    let width = ui.available_width() - button;
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| design::message(ui, tone, &words),
                    );
                    if design::icon_button(ui, icons::CLOSE, "Dismiss").clicked() {
                        self.notice = None;
                        self.notice_session = None;
                    }
                });
            }
        }
        if let Some(copy) = note.also.first() {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "Another file claims this session: {}. Splat edits {}. Remove or rename the other one.",
                    copy.file_name().unwrap_or_default().to_string_lossy(),
                    note.path.file_name().unwrap_or_default().to_string_lossy()
                ),
            );
        }
        let today = self.today();
        let writing = self.tab != Tab::Recordings && !self.reading_note;
        if writing
            && self.selected == self.current()
            && let Some(words) = dated_before_today(&note, &today)
            && design::message_with_action(ui, Tone::Warning, &words, "Play this one")
        {
            self.play_this_one();
        }
        if let Some(mut words) = self.not_being_played() {
            let current = if plan.group_open {
                self.group_session
            } else {
                self.current()
            };
            if plan.group_open {
                let number = self
                    .notes
                    .iter()
                    .find(|note| Some(note.id) == self.group_session)
                    .map(|note| note.number);
                if let Some(number) = number {
                    words = format!(
                        "Session {} is not the session being played. A group is being assembled in Session {number}.",
                        note.number
                    );
                }
            }
            let next = self
                .notes
                .iter()
                .find(|note| Some(note.id) == current)
                .map(|note| format!("Open Session {}", note.number));
            let mut open = false;
            ui.horizontal_wrapped(|ui| {
                design::message(ui, Tone::Warning, &words);
                if let Some(next) = next {
                    open = design::link(ui, &next).clicked();
                }
            });
            if open {
                self.select(current);
            }
        }
        // A group being assembled needs the height, and last time can wait for it.
        if !(plan.group_open && self.group_session == self.selected) {
            self.last_time_block(ui, book);
        }
        if self
            .draft
            .as_ref()
            .is_some_and(|draft| draft.state == DraftState::Conflict)
        {
            self.note_editor(ui, false);
            return;
        }
        let editing_group_here = plan.group_open && self.group_session == self.selected;
        let beside = ui.available_width() >= PLAN_BESIDE_NOTE_FROM && !editing_group_here;
        // In a wide page, the note is on the plan's tab, so there is no second one to choose.
        let mut shown = if beside && self.tab == Tab::Note {
            Tab::Plan
        } else {
            self.tab
        };
        let tabs: &[(Tab, &str)] = if beside {
            &[(Tab::Plan, "Plan and notes")]
        } else {
            &[(Tab::Plan, "Plan"), (Tab::Note, "Note")]
        };
        ui.horizontal(|ui| {
            let changed = ui
                .horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let changed = design::segmented(ui, &mut shown, tabs);
                    recordings_tab(ui, &mut shown, self.recordings.len()) || changed
                })
                .inner;
            if changed {
                self.tab = shown;
                self.clear_success();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if title_on_own_line {
                    self.session_details_action(ui, &note);
                }
                if shown == Tab::Note || (shown == Tab::Plan && beside) {
                    let (word, icon) = if self.reading_note {
                        ("Write", icons::EDIT)
                    } else {
                        ("Read", icons::DOCUMENT)
                    };
                    if Action::quiet(word)
                        .icon(icon)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        self.reading_note = !self.reading_note;
                    }
                }
            });
        });
        ui.add_space(design::XS);
        match shown {
            Tab::Plan if beside => {
                let height = ui.available_height();
                let plan_width = (ui.available_width() * 0.4).clamp(PLAN_WIDTH, PLAN_MAX_WIDTH);
                let note_width = ui.available_width() - plan_width - design::L;
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = design::L;
                    page_column(ui, plan_width, height, |ui| {
                        self.plan_pane(ui, book, plan);
                    });
                    page_column(ui, note_width, height, |ui| {
                        self.note_pane(ui, book, plan, beside)
                    });
                });
            }
            Tab::Plan => self.plan_pane(ui, book, plan),
            Tab::Note => self.note_pane(ui, book, plan, beside),
            Tab::Recordings => {
                self.recordings_tab(ui, settings, book);
            }
        }
    }

    fn notes_view(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
    ) {
        if !self.unreadable.is_empty() {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "Splat cannot read the properties of {}, so it is not listed. The file is safe. Repair it to bring it back.",
                    self.unreadable.join(", ")
                ),
            );
        }
        let holding = self.draft.as_ref().is_some_and(Draft::needs_attention);
        if self.notes.is_empty() && self.form.is_none() && !holding {
            let mut start = false;
            design::empty_state(
                ui,
                icons::SESSIONS,
                "No session notes yet",
                "Start a session from a template. Notes are Markdown files in the campaign folder.",
                |ui| {
                    start = if self.open_run.is_some() {
                        Action::secondary("New session")
                    } else {
                        Action::primary("New session")
                    }
                    .show(ui)
                    .clicked();
                },
            );
            if start {
                self.open_form();
            }
            return;
        }
        // The first session's form has no list to stand beside.
        if self.notes.is_empty() && !holding {
            egui::ScrollArea::vertical()
                .id_salt("session_form")
                .auto_shrink([false, false])
                .show(ui, |ui| self.show_form(ui));
            return;
        }
        // Beside Lookup at a small window, two columns would crush the session.
        self.list_beside = ui.available_width() >= TWO_COLUMNS_FROM;
        if !self.list_beside {
            if self.list_only && self.form.is_none() {
                self.list_pane(ui);
            } else {
                if self.form.is_none()
                    && Action::quiet("All sessions")
                        .icon(icons::BACK)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                {
                    self.list_only = true;
                }
                if self.form.is_some() {
                    egui::ScrollArea::vertical()
                        .id_salt("session_form")
                        .auto_shrink([false, false])
                        .show(ui, |ui| self.show_form(ui));
                } else {
                    self.session_pane(ui, settings, book, plan);
                }
            }
            return;
        }
        design::Column::left("session_list_panel", LIST_WIDTH, LIST_WIDTHS)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| self.list_pane(ui));
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                if self.form.is_some() {
                    egui::ScrollArea::vertical()
                        .id_salt("session_form")
                        .auto_shrink([false, false])
                        .show(ui, |ui| self.show_form(ui));
                } else {
                    self.session_pane(ui, settings, book, plan);
                }
            });
    }

    fn select_template(&mut self, name: &str) {
        self.flush();
        if self
            .template_draft
            .as_ref()
            .is_some_and(Draft::needs_attention)
        {
            return;
        }
        self.template = Some(name.to_owned());
        self.template_draft = self
            .templates
            .iter()
            .find(|template| template.name == name)
            .and_then(|template| template.path.as_deref())
            .and_then(|path| Draft::open(path).ok());
    }

    fn templates_view(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        if self.template.is_none()
            && let Some(first) = self.templates.first().map(|t| t.name.clone())
        {
            self.select_template(&first);
        }
        let mut choose = None;
        // Beside Lookup at a small window, two columns would crush the editor.
        let narrow = ui.available_width() < TWO_COLUMNS_FROM;
        if narrow && self.templates_list_only {
            self.template_list(ui, &mut choose);
        } else if narrow {
            if Action::quiet("All templates")
                .icon(icons::BACK)
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                self.templates_list_only = true;
            }
        } else {
            design::Column::left("template_list_panel", LIST_WIDTH, LIST_WIDTHS)
                .frame(egui::Frame::new().inner_margin(egui::Margin {
                    right: design::S as i8,
                    ..Default::default()
                }))
                .show_inside(ui, |ui| self.template_list(ui, &mut choose));
        }
        if let Some(name) = choose {
            self.templates_list_only = false;
            self.select_template(&name);
        }
        if narrow && self.templates_list_only {
            return;
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                if self.new_template.is_none() { ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(
                                ui,
                                "A new session starts from a template. Splat fills in {{number}}, {{title}}, {{date}} and {{campaign}}.",
                            )
                            .color(c.muted),
                    )
                    .wrap(),
                ); }
                if let Some((name, _)) = self.deleted_template.clone()
                    && design::message_with_action(
                        ui,
                        Tone::Info,
                        &format!("Deleted “{name}”."),
                        "Undo delete",
                    )
                {
                    let (name, body) = self.deleted_template.take().expect("just read");
                    if let Some(folder) = &self.folder {
                        match create_template(folder, &name, &body) {
                            Ok(_) => {
                                self.refresh();
                                self.select_template(&name);
                            }
                            Err(error) => self.template_error = Some(error),
                        }
                    }
                }
                if let Some(error) = &self.template_error {
                    design::message(ui, Tone::Error, error);
                }
                // One thing at a time, so that the DM does not type into another template.
                if self.new_template.is_some() {
                    self.new_template_form(ui);
                } else {
                    self.template_editor(ui);
                }
            });
    }

    fn template_list(&mut self, ui: &mut egui::Ui, choose: &mut Option<String>) {
        if design::link(ui, "← Sessions").clicked() {
            self.view = View::Notes;
        }
        ui.add_space(design::S);
        egui::ScrollArea::vertical()
            .id_salt("template_list")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for template in &self.templates {
                    let kind = if template.path.is_some() {
                        "Campaign"
                    } else {
                        "Built-in"
                    };
                    if design::two_line_row(
                        ui,
                        Density::Glance.control() + design::S,
                        self.new_template.is_none()
                            && self.template.as_deref() == Some(template.name.as_str()),
                        &template.name,
                        kind,
                    )
                    .clicked()
                    {
                        *choose = Some(template.name.clone());
                    }
                }
            });
    }

    fn new_template_form(&mut self, ui: &mut egui::Ui) {
        let Some(mut name) = self.new_template.take() else {
            return;
        };
        ui.add_space(design::L);
        design::group_label(ui, "New template");
        ui.add_space(design::M);
        design::text_field(
            ui,
            "Template name",
            &mut name,
            Some("Starts with the title line only."),
            None,
        );
        let mut keep = true;
        ui.horizontal(|ui| {
            if if self.open_run.is_some() {
                Action::secondary("Create template")
            } else {
                Action::primary("Create template")
            }
            .show(ui)
            .clicked()
                && let Some(folder) = &self.folder
            {
                let body = "# Session {{number}}: {{title}}\n\n";
                match create_template(folder, &name, body) {
                    Ok(template) => {
                        keep = false;
                        self.template_error = None;
                        self.refresh();
                        self.select_template(&template.name);
                    }
                    Err(error) => self.template_error = Some(error),
                }
            }
            if Action::quiet("Cancel").show(ui).clicked() {
                keep = false;
                self.template_error = None;
            }
        });
        if keep {
            self.new_template = Some(name);
        }
    }

    fn template_editor(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let Some(template) = self
            .template
            .as_ref()
            .and_then(|name| self.templates.iter().find(|t| &t.name == name))
            .cloned()
        else {
            return;
        };
        let mut customize = false;
        let mut delete = false;
        ui.horizontal(|ui| {
            ui.label(Type::Heading.rich(ui, &template.name).color(c.text));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if template.path.is_some() {
                    let more = design::icon_button(
                        ui,
                        icons::EXPAND,
                        &format!("More actions for {}", template.name),
                    );
                    egui::Popup::menu(&more).show(|ui| {
                        let built_in = super::BUILT_IN_TEMPLATES
                            .iter()
                            .any(|(name, _)| *name == template.name);
                        delete = Action::quiet(if built_in {
                            "Go back to built-in"
                        } else {
                            "Delete"
                        })
                        .name(if built_in {
                            "Go back to built-in"
                        } else {
                            "Delete this template"
                        })
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                        if delete {
                            ui.close();
                        }
                    });
                    if let Some(draft) = &self.template_draft {
                        Self::draft_status(ui, draft);
                    }
                } else {
                    customize = Action::secondary("Customize")
                        .name("Customize this template")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                    design::status(
                        ui,
                        Status {
                            icon: icons::READ_ONLY,
                            word: "Read-only",
                            color: c.document.read_only,
                        },
                    );
                }
            });
        });
        if customize && let Some(folder) = &self.folder {
            match create_template(folder, &template.name, &template.body) {
                Ok(created) => {
                    self.refresh();
                    self.select_template(&created.name);
                }
                Err(error) => self.template_error = Some(error),
            }
            return;
        }
        if delete && let Some(path) = &template.path {
            match fs::remove_file(path) {
                Ok(()) => {
                    let body = self
                        .template_draft
                        .take()
                        .map_or_else(|| template.body.clone(), |draft| draft.body);
                    self.deleted_template = Some((template.name.clone(), body));
                    self.template = None;
                    self.refresh();
                }
                Err(error) => self.template_error = Some(format!("Not deleted. {error}")),
            }
            return;
        }
        if let Some(draft) = &mut self.template_draft {
            if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
                draft.save();
            }
            Self::draft_problem(ui, draft);
        }
        egui::ScrollArea::vertical()
            .id_salt(("template_editor", &template.name))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(draft) = &mut self.template_draft {
                    let mut layouter = crate::markdown_source::layouter;
                    let output = egui::TextEdit::multiline(&mut draft.body)
                        .font(design::reader::prose_type(ui).font(ui))
                        .layouter(&mut layouter)
                        .margin(egui::vec2(design::S, design::S))
                        .desired_width(ui.available_width())
                        .min_size(egui::vec2(0.0, ui.available_height()))
                        .show(ui);
                    crate::markdown_source::paint_section_rules(ui, &output);
                    let response = output.response;
                    design::named(ui, &response, "Template text");
                    if response.changed() {
                        draft.edited();
                        self.deleted_template = None;
                    }
                } else {
                    let mut body = template.body.clone();
                    let mut layouter = crate::markdown_source::layouter;
                    let output = egui::TextEdit::multiline(&mut body)
                        .font(design::reader::prose_type(ui).font(ui))
                        .layouter(&mut layouter)
                        .desired_width(ui.available_width())
                        .min_size(egui::vec2(0.0, ui.available_height()))
                        .frame(false)
                        .interactive(false)
                        .show(ui);
                    design::named(ui, &output.response, "Built-in template text");
                    crate::markdown_source::paint_section_rules(ui, &output);
                }
            });
    }

    pub fn folder_view(&mut self, ui: &mut egui::Ui, book: &Book) {
        let c = colors();
        let Some(campaign) = self.campaign.clone() else {
            return;
        };
        ui.add_space(design::XS);
        let edited = design::text_field(
            ui,
            "Session folder",
            &mut self.folder_text,
            Some(
                "Inside the campaign folder, unless the path is absolute. A name such as 30 Sessions is fine.",
            ),
            self.folder_error.as_deref(),
        )
        .changed();
        if edited {
            self.folder_notice = None;
        }
        if let Some(folder) = &self.folder {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, format!("Now: {}", folder.display()))
                        .color(c.muted),
                )
                .wrap(),
            );
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(
                            ui,
                            "Notes, Timeline.md, Templates and Recordings are all here.",
                        )
                        .color(c.muted),
                )
                .wrap(),
            );
        }
        let changed = super::resolve(&campaign, &self.folder_text).ok() != self.folder.clone();
        let files = self
            .folder
            .as_ref()
            .map(|folder| super::storage_files(folder))
            .transpose();
        let has_files = changed
            && files
                .as_ref()
                .ok()
                .and_then(|files| files.as_ref())
                .is_some_and(|files| !files.is_empty());
        let mut move_counts = String::new();
        if has_files {
            let files = files.as_ref().unwrap().as_ref().unwrap();
            let notes = self.notes.len();
            let templates = files
                .iter()
                .filter(|path| {
                    self.folder
                        .as_ref()
                        .is_some_and(|folder| path.starts_with(folder.join(super::TEMPLATES)))
                })
                .count();
            let recordings = files
                .iter()
                .filter(|path| path.extension().is_some_and(|ext| ext == "wav"))
                .count();
            let mut counts = vec![
                design::count(notes, "note", "notes"),
                design::count(book.events.len(), "event", "events"),
                design::count(templates, "template", "templates"),
                design::count(recordings, "recording", "recordings"),
            ]
            .into_iter()
            .filter(|count| !count.is_empty())
            .collect::<Vec<_>>();
            let last = counts.pop().unwrap_or_default();
            let counts = if counts.is_empty() {
                last
            } else {
                format!("{} and {last}", counts.join(", "))
            };
            let counts = if counts.is_empty() {
                "Timeline.md".to_owned()
            } else {
                counts
            };
            move_counts = counts.clone();
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "The current folder keeps {counts}. Splat will not show these files from the new folder."
                ),
            );
        }
        ui.add_space(design::XS);
        let blocked = if self.live.is_some() {
            Some("Stop the recording first.")
        } else if self.job.is_some() {
            Some("Wait for processing to finish.")
        } else if self.busy_with_edit() || book.not_saved() {
            Some("Choose what to do with the unsaved file first.")
        } else if files.is_err() {
            Some("Could not read the session files. Check the current folder first.")
        } else if !changed {
            Some("No changes to save.")
        } else {
            None
        };
        let mut move_files = false;
        let save = ui
            .horizontal_wrapped(|ui| {
                if changed {
                    design::status(
                        ui,
                        Status {
                            icon: icons::DRAFT,
                            word: "Unsaved",
                            color: colors().document.unsaved,
                        },
                    );
                    if Action::quiet("Discard").show(ui).clicked() {
                        if let Ok(saved) = folder_setting(&campaign) {
                            self.folder_text = saved;
                        }
                        self.folder_error = None;
                    }
                }
                design::blocked_reason(ui, blocked);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if has_files {
                        move_files = Action::secondary("Move files and save")
                            .blocked(blocked)
                            .show(ui)
                            .clicked();
                        Action::quiet("Save without moving")
                            .blocked(blocked)
                            .show(ui)
                            .clicked()
                            || move_files
                    } else {
                        Action::secondary("Save folder")
                            .blocked(blocked)
                            .show(ui)
                            .clicked()
                    }
                })
                .inner
            })
            .inner;
        if save {
            self.flush();
            // The save just now can fail. Those words stay on screen, as `select` keeps them.
            if self.draft.as_ref().is_some_and(Draft::needs_attention) {
                self.folder_error = Some(
                    "Not changed. Your note could not be saved. Choose what to do with it first."
                        .to_owned(),
                );
                return;
            }
            let saved = if move_files {
                super::move_storage(&campaign, &self.folder_text)
            } else {
                save_folder(&campaign, &self.folder_text)
            };
            match saved {
                Ok(folder) => {
                    self.folder = Some(folder);
                    self.folder_error = None;
                    self.folder_notice =
                        move_files.then(|| format!("Moved {move_counts} to {}.", self.folder_text));
                    self.manifest_changed = Some(campaign.path.clone());
                    self.draft = None;
                    self.template_draft = None;
                    self.template = None;
                    self.refresh();
                    self.select(self.notes.first().map(|note| note.id));
                }
                Err(error) => {
                    self.folder_error = Some(error);
                    self.folder_notice = None;
                }
            }
        }
        if let Some(words) = &self.folder_notice {
            design::message(ui, Tone::Success, words);
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
    ) {
        let (page_title, count, singular, plural) = if self.view == View::Templates {
            ("Templates", self.templates.len(), "template", "templates")
        } else {
            ("Session", self.notes.len(), "session", "sessions")
        };
        let context = if count == 0 {
            String::new()
        } else {
            design::count(count, singular, plural)
        };
        let mut new_session = false;
        let in_form = self.form.is_some();
        let empty = self.notes.is_empty();
        let locked = self.draft.as_ref().is_some_and(Draft::needs_attention);
        let mut stop = false;
        design::page_header(ui, page_title, Some(&context), |ui| {
            let list_alone = self.list_only && ui.ctx().content_rect().width() < 1500.0;
            if self.view == View::Notes && !in_form && !empty {
                new_session = Action::secondary("New session")
                    .icon(icons::ADD)
                    .blocked_if(locked, "Other sessions wait until you choose what to keep.")
                    .show(ui)
                    .clicked();
            }
            if self.view == View::Templates
                && Action::secondary("New template")
                    .icon(icons::ADD)
                    .show(ui)
                    .clicked()
            {
                self.templates_list_only = false;
                self.new_template = Some(String::new());
                self.template_error = None;
            }
            if self.live.is_some() && (self.view != View::Notes || in_form || list_alone || empty) {
                stop = Action::quiet("Stop recording")
                    .icon(icons::STOP_RECORDING)
                    .show(ui)
                    .clicked();
            }
            if self.view == View::Notes && Action::quiet("Templates").show(ui).clicked() {
                self.view = View::Templates;
            }
        });
        if stop {
            self.finish_recording();
        }
        if new_session {
            self.open_form();
        }
        if self.folder.is_none() {
            design::message(
                ui,
                Tone::Error,
                self.folder_error
                    .as_deref()
                    .unwrap_or("The session folder is not available."),
            );
            if design::link(ui, "Open Campaign folder settings").clicked() {
                self.wants_campaign_link = true;
            }
            return;
        }
        match self.view {
            View::Notes => self.notes_view(ui, settings, book, plan),
            View::Templates => self.templates_view(ui),
        }
        self.delete_dialog(ui.ctx());
    }
}

#[cfg(test)]
impl Sessions {
    pub fn busy(&self) -> bool {
        self.job.as_ref().is_some_and(|(job, _)| !job.is_held())
    }

    /// The open note's file is renamed outside Splat, as Obsidian can do.
    pub fn simulate_file_renamed(&mut self) {
        let draft = self.draft.as_ref().expect("an open note");
        let new = draft.path.with_file_name("Renamed in Obsidian.md");
        fs::rename(&draft.path, new).expect("the note");
    }

    /// The open note changes outside Splat, as when Obsidian edits the same file.
    pub fn simulate_disk_change(&mut self) {
        let draft = self.draft.as_ref().expect("an open note");
        let text = fs::read_to_string(&draft.path).expect("the note");
        fs::write(&draft.path, format!("{text}\nWritten in Obsidian.\n")).expect("the note");
    }

    /// A job that stays in progress, for a capture of that state.
    pub fn simulate_job(&mut self, progress: processing::Progress) {
        self.notice = None;
        self.notice_session = None;
        let session = self.selected.expect("an open session");
        let path = self.recordings[0].path.clone();
        self.job = Some((Job::held(path, progress), session));
    }

    /// A job that ends with a warning, stored as a real one is.
    pub fn simulate_job_end_warned(&mut self, words: &str) {
        let (job, session) = self.job.as_ref().expect("a job");
        let session = *session;
        fs::write(processing::warning_path(&job.recording), words).expect("the warning");
        self.finish_job(Ok(Some(words.to_owned())), session);
    }

    pub fn simulate_job_end(&mut self, result: Result<Option<String>, String>) {
        let session = self.job.as_ref().expect("a job").1;
        self.finish_job(result, session);
    }

    pub fn simulate_recorder_health(&mut self, health: Health) {
        if let Some(live) = &mut self.live {
            live.recorder.simulate_health(health);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn narrow_plan_summaries_never_end_a_line_with_a_separator() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for summary in [
                    "Held the bridge · Ended · Round 1",
                    "5 party · 2 × Cinder Wolf · Ember Cultist",
                ] {
                    for width in [140.0, 180.0, 220.0, 330.0] {
                        let galley = ui.painter().layout(
                            plan_summary(summary),
                            Type::Small.font(ui),
                            colors().muted,
                            width,
                        );
                        for row in &galley.rows {
                            let text: String = row.glyphs.iter().map(|glyph| glyph.chr).collect();
                            assert!(!text.trim_end().ends_with('·'), "{width}: {text}");
                        }
                    }
                }
            });
        });
    }

    #[test]
    fn removing_one_planned_fight_keeps_other_links_prose_and_sections() {
        let body = "## Recap of last time\n- [[Bridge]]\n## Plan for tonight\n- [[Bridge]]\n- [[Bridge]]\nMeet [[Bridge|the guard]] at dusk.\n- [[Shrine]]\n## What happened\n- [[Bridge]]\n";
        let changed = unlink_planned_fight(body, "Bridge", 1);
        assert_eq!(
            changed,
            body.replacen("- [[Bridge]]\n- [[Bridge]]", "- [[Bridge]]", 1)
        );
        assert_eq!(
            unlink_planned_fight(body, "Bridge", 2),
            body.replace(
                "Meet [[Bridge|the guard]] at dusk.",
                "Meet the guard at dusk."
            )
        );
        assert_eq!(unlink_planned_fight(body, "Missing", 0), body);
    }

    #[test]
    fn link_update_undo_expires_on_navigation_note_edits_and_event_edits() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 1,
                title: "",
                date: "2026-11-03",
                campaign: "Test",
                template: "Original words.",
            },
        )
        .unwrap();
        let mut sessions = Sessions {
            notes: vec![note.clone()],
            ..Default::default()
        };
        let mut book = Book::default();
        for edit in 0..3 {
            sessions.updated_links = Some(Vec::new());
            sessions.link_undo_state = Some(LinkUndoState {
                notes: sessions.note_bodies(),
                events: book.events.clone(),
            });
            sessions.expire_event_link_undo(true, &book);
            assert!(sessions.updated_links.is_some());
            match edit {
                0 => sessions.expire_event_link_undo(false, &book),
                1 => {
                    fs::write(&note.path, "Changed outside Splat.").unwrap();
                    sessions.expire_event_link_undo(true, &book);
                }
                _ => {
                    book.add(Kind::Day, None);
                    sessions.expire_event_link_undo(true, &book);
                }
            }
            assert!(sessions.updated_links.is_none());
        }
    }

    #[test]
    fn event_link_updates_save_and_undo_as_one_batch_without_overwriting_external_edits() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        let id = book.add(Kind::LongRest, None);
        let body = format!(
            "## What happened\n\n[[Timeline#^{id}|Long rest]] and [[Timeline#^{id}|our rest]].\n"
        );
        let notes: Vec<_> = (1..=2)
            .map(|number| {
                create_note(
                    dir.path(),
                    &NewNote {
                        number,
                        title: "",
                        date: "2026-01-01",
                        campaign: "Test",
                        template: &body,
                    },
                )
                .unwrap()
            })
            .collect();
        let original: Vec<_> = notes
            .iter()
            .map(|note| fs::read(&note.path).unwrap())
            .collect();
        book.events[0].title = "Rest at the mill".into();
        book.touch();
        // Merely renaming never writes a note.
        for (note, before) in notes.iter().zip(&original) {
            assert_eq!(fs::read(&note.path).unwrap(), *before);
        }
        let changes = notes
            .iter()
            .map(|note| {
                let draft = Draft::open(&note.path).unwrap();
                let (updated, count) = updated_event_links(&draft.body, &book);
                assert_eq!(count, 1);
                (draft, updated)
            })
            .collect();
        let mut sessions = Sessions::default();
        sessions.update_event_link_notes(changes);
        assert_eq!(sessions.updated_links.as_ref().unwrap().len(), 2);
        for note in &notes {
            let saved = fs::read_to_string(&note.path).unwrap();
            assert!(saved.contains("|Rest at the mill]]") && saved.contains("|our rest]]"));
        }
        sessions.undo_event_link_notes();
        assert!(sessions.updated_links.is_none());
        for (note, before) in notes.iter().zip(&original) {
            assert_eq!(fs::read(&note.path).unwrap(), *before);
        }
        // An external edit after the preview cannot be overwritten by accepting it.
        let draft = Draft::open(&notes[0].path).unwrap();
        let updated = updated_event_links(&draft.body, &book).0;
        let outside = format!(
            "{}External edit.\n",
            fs::read_to_string(&notes[0].path).unwrap()
        );
        fs::write(&notes[0].path, &outside).unwrap();
        sessions.update_event_link_notes(vec![(draft, updated)]);
        assert_eq!(fs::read_to_string(&notes[0].path).unwrap(), outside);
        assert!(sessions.updated_links.is_none());
    }

    #[test]
    fn event_link_update_changes_old_titles_and_keeps_custom_aliases() {
        let mut book = Book::default();
        let id = book.add(Kind::LongRest, None);
        book.events[0].title = "Rest at the mill".into();
        book.renamed_links
            .insert(id.clone(), vec!["Long rest".into()]);
        let original = format!(
            "## What happened\n\n[[Timeline#^{id}|Long rest]] and [[Timeline#^{id}|Long rest]].\n[[Timeline#^{id}|our rest]]\n"
        );
        let (updated, count) = updated_event_links(&original, &book);
        assert_eq!(count, 2);
        assert_eq!(updated.matches("|Rest at the mill]]").count(), 2);
        assert!(updated.contains("|our rest]]"));
        assert_eq!(updated_event_links(&updated, &book).1, 0);
    }

    #[test]
    fn recap_uses_prose_and_skips_link_only_lines() {
        let body = "## Recap of last time\n\n## What happened\n\n- [[Timeline#^t-rest|Long rest]]\n\nWe reached the mill.\n";
        assert_eq!(recap_preview(body).as_deref(), Some("We reached the mill."));
        assert_eq!(recap_preview("## What happened\n- [[Bridge]]\n"), None);
        assert_eq!(
            recap_preview("## Recap of last time\nWe won.\n\n## What happened\nWe ran.").as_deref(),
            Some("We won.")
        );
    }

    #[test]
    fn read_plan_summary_keeps_mixed_notes_and_repeated_fights_in_the_file() {
        let fights = [PlanFight {
            saved: None,
            name: "Bridge Ambush".into(),
            path: String::new(),
            summary: String::new(),
        }];
        let source = "## Plan for tonight\n\n- [[Bridge Ambush]]\n- [[Bridge Ambush]]\n\n## What happened\n\nWe crossed the bridge.\n";
        let compact = compact_plan_for_reading(source, &fights, true);
        assert!(!compact.contains("Plan for tonight"));
        assert!(!compact.contains("[[Bridge Ambush]]"));
        assert!(compact.contains("We crossed the bridge."));
        assert_eq!(source.matches("[[Bridge Ambush]]").count(), 2);
        let tab = compact_plan_for_reading(source, &fights, false);
        assert!(tab.contains("2 fights, in the plan"));
        assert!(!tab.contains("at left"));
        let empty = source.replace("We crossed the bridge.", "");
        assert!(!has_written_text(&compact_plan_for_reading(
            &empty, &fights, true
        )));
        let repeated = format!(
            "## Recap of last time\n\n- [[Bridge Ambush]]\n- [[Bridge Ambush]]\n\n{source}"
        );
        let compact = compact_plan_for_reading(&repeated, &fights, true);
        assert_eq!(compact.matches("[[Bridge Ambush]]").count(), 2);
        assert!(compact.starts_with("## Recap of last time\n\n- [[Bridge Ambush]]"));
        let mixed = source.replace("- [[Bridge Ambush]]\n\n##", "- Ask the ferryman.\n\n##");
        assert_eq!(compact_plan_for_reading(&mixed, &fights, true), mixed);
    }

    #[test]
    fn move_run_undo_restores_the_event_and_expires_after_an_edit() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("note.md");
        fs::write(&path, "## What happened\n").unwrap();
        let old = Uuid::from_u128(1);
        let new = Uuid::from_u128(2);
        let run = Uuid::from_u128(3);
        let mut sessions = Sessions {
            draft: Some(Draft::open(&path).unwrap()),
            open_run: Some(("Bridge Ambush".into(), old)),
            ..Default::default()
        };
        let mut book = Book::default();
        book.add(Kind::Encounter, Some(old));
        book.events[0].run = Some(run);
        book.events[0].continued_in = Some(new);
        let before = book.events.clone();
        sessions.move_run_to_session(&mut book, run, new);
        assert_eq!(book.events[0].session, Some(new));
        assert_eq!(book.events[0].continued_in, None);
        sessions.undo_move(&mut book);
        assert_eq!(book.events, before);
        assert_eq!(sessions.open_run.as_ref().unwrap().1, old);
        sessions.move_run_to_session(&mut book, run, new);
        sessions.draft.as_mut().unwrap().body.push_str("A note.");
        sessions.expire_move(&book);
        assert!(sessions.moved_run.is_none() && sessions.notice.is_none());
        sessions.move_run_to_session(&mut book, run, old);
        book.add(Kind::Travel, Some(new));
        sessions.expire_move(&book);
        assert!(sessions.moved_run.is_none() && sessions.notice.is_none());
    }

    #[test]
    fn read_view_keeps_inline_note_and_event_links_without_rewriting_the_note() {
        let mut book = Book::default();
        let id = book.add(Kind::LongRest, None);
        book.events[0].title = "Rest at the mill".into();
        let source = format!("A [[Bridge Ambush]] before [[Timeline#^{id}|Old name]].");
        let rendered = words_to_read(&source, &book);
        assert!(rendered.contains("[Bridge Ambush](<splat-note:Bridge Ambush>)"));
        assert!(rendered.contains(&format!("[Old name](<splat-note:Timeline#^{id}>)")));
        assert!(source.contains("|Old name]]"));
    }
}
