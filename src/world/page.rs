//! The World page: what is true in this world. One list of notes and entries, and the Reader.
use crate::world::{Nav, Reader, checks::tick_file, file_fault::FileFault};
use crate::{
    design::{
        self, Type, colors, icons,
        outcome::{
            Choice, Done, FaultMessage, MessageChoice, Messages, Notice, PagePlaceholder,
            PaneFrame, Said, Verb,
        },
        pane::Body,
    },
    lookup::{ContentIndex, WorldGroups},
    persistence::Campaign,
};
use eframe::egui;
use std::sync::Arc;
use uuid::Uuid;

const ROW: f32 = 40.0;
/// Draws `add`, a page's fault, with its words at the reader measure (round 24 AG2: recovery
/// steps are prose), past the message's bar and icon (`design::message_choice_row`'s inset).
/// The Library's unreadable reader keeps the same measure (`library::at_reader_measure`).
fn at_reader_measure<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let inset = design::S
        + 1.0
        + design::number_width(ui, design::icons::ERROR, &Type::Body.font(ui))
        + design::XS;
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(design::READER_MEASURE + inset));
        add(ui)
    })
    .inner
}

const WORLD_SEARCH: design::FieldId =
    crate::field_id!("world_search").once(design::Once::PageSearch);
const NEW_NAME: design::FieldId = crate::field_id!("world_new_name").once(design::Once::CreateForm);
const NEW_NOTE_TYPE: design::FieldId =
    crate::field_id!("world_new_note_type").once(design::Once::CreateForm);
const LIST_WIDTH: f32 = 260.0;
/// More rows than this in one check are counted, not shown.
const CHECK_ROWS: usize = 40;
/// The line above the World card while Lookup holds a second unsaved NPC (round 26 FB8).
pub fn kept_in_lookup(lookup: &str, card: &str) -> String {
    format!("{lookup} is not saved. It is in Lookup. Save or discard {card} to show {lookup} here.")
}

/// The advice page's row and title: what to check, not what is broken (round 26 FF3, FH1).
pub const PROBLEMS_TITLE: &str = "Things to check";

/// What the Things to check page lists, in the words of its sections.
const PROBLEMS_INTRO: &str = "These lists show notes with a date outside an event, a link to no note, no summary, no links, or a name or sentence on two notes. Splat only lists them. You change the notes.";
pub use crate::world::UNSAVED_NPC;

/// A repair cause must stay whole even when Lookup narrows the list. So must the path: a long
/// one wraps, because a cut path cannot tell two files in one folder apart. A note that cannot
/// be read is an error, so its cause follows the error icon in the error colour; the words stay
/// muted, as every row's do.
fn broken_row(ui: &mut egui::Ui, selected: bool, path: &str, cause: &str) -> egui::Response {
    let width = (ui.available_width() - design::S * 2.0).max(design::XXL);
    let font = if selected { Type::Strong } else { Type::Body };
    let title = ui
        .painter()
        .layout(path.to_owned(), font.font(ui), colors().text, width);
    let format = |color| egui::TextFormat {
        font_id: Type::Small.font(ui),
        color,
        ..Default::default()
    };
    // The icon has its own column, so a wrapped cause starts each line at its first word, as
    // every wrapped status does (round 27 FH3, FF4).
    let icon = ui.painter().layout_no_wrap(
        icons::ERROR.to_owned(),
        Type::Small.font(ui),
        colors().result.error,
    );
    let gap = ui
        .painter()
        .layout_no_wrap(" ".to_owned(), Type::Small.font(ui), colors().muted)
        .size()
        .x;
    let words_x = icon.size().x + gap;
    let mut job = egui::text::LayoutJob::default();
    job.append(cause, 0.0, format(colors().muted));
    job.wrap.max_width = (width - words_x).max(design::XXL);
    let detail = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let height = (title.size().y + detail.size().y + design::S).max(ROW);
    design::row(
        ui,
        height,
        selected,
        &format!("{path} · {cause}"),
        |ui, rect| {
            let position = rect.min + egui::vec2(design::S, design::XS);
            ui.painter().galley(position, title.clone(), colors().text);
            let below = position + egui::vec2(0.0, title.size().y);
            ui.painter()
                .galley(below, icon.clone(), colors().result.error);
            ui.painter()
                .galley(below + egui::vec2(words_x, 0.0), detail, colors().muted);
        },
    )
}

/// `design::two_line_row`, with each warning mark in the detail in its own tone before the
/// muted words, as Library's rows have.
/// A list row: its title, and under it its detail, or `narrow` in its place when the detail does
/// not fit the row whole. The row's accessible name keeps the whole detail.
fn marked_row(
    ui: &mut egui::Ui,
    selected: bool,
    title: &str,
    detail: &str,
    narrow: Option<&str>,
) -> egui::Response {
    design::row(
        ui,
        ROW,
        selected,
        &format!("{title} · {detail}"),
        |ui, rect| {
            let c = colors();
            let width = rect.width() - 2.0 * design::S;
            let font = if selected { Type::Strong } else { Type::Body };
            let galley = design::one_line(ui, title, font.font(ui), c.text, width);
            let total = galley.size().y + Type::Small.line();
            let top = rect.center().y - total / 2.0;
            ui.painter()
                .galley(egui::pos2(rect.left() + design::S, top), galley, c.text);
            let detail = match narrow {
                Some(narrow) if design::number_width(ui, detail, &Type::Small.font(ui)) > width => {
                    narrow
                }
                _ => detail,
            };
            let detail =
                crate::library::marked_line(ui, detail, Type::Small.font(ui), c.muted, width);
            ui.painter().galley(
                egui::pos2(rect.left() + design::S, top + total - Type::Small.line()),
                detail,
                c.muted,
            );
        },
    )
}

/// What the page's error slot holds.
#[derive(Clone)]
enum PageError {
    Words(String),
    /// A note that the system will not read, so its repair could not open.
    Unread(Unread),
}

/// The pane names the file, the recovery and, in Technical details, the system's words.
#[derive(Clone)]
struct Unread {
    fault: FileFault,
    /// The index load the pane was last shown under, to tell a later load apart.
    seen: Option<u64>,
    /// Try again asked for a new load; when it comes, the note opens again.
    again: bool,
}

impl From<String> for PageError {
    fn from(words: String) -> Self {
        Self::Words(words)
    }
}

/// A generated NPC with the table facts its card shows.
type Held = (
    crate::npc_generator::Generated,
    Vec<crate::npc_generator::TableProblem>,
    Vec<String>,
);

/// The page's one kept result (`Note repaired.`, a generated NPC's discard). A later one
/// replaces it; it shows only while its own state holds (`page_messages`).
struct PageKept(crate::world::PageResult);

#[derive(Clone, Debug, PartialEq, Eq)]
struct PageSlot;

impl design::results::Owner for PageSlot {
    type Scope = ();
    fn scope(&self) -> &() {
        &()
    }
}

impl design::results::Kept for PageKept {
    type Owner = PageSlot;
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Stays,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> PageSlot {
        PageSlot
    }
}

/// The result of the card's `Copy the marker`: the held NPC it was pressed for. It has no Undo;
/// it shows while that NPC is on the card. Another NPC on the card (a roll, Lookup's card), its
/// save or discard, and a campaign change end it; a new press replaces it.
struct MarkerCopied(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct MarkerSlot;

impl design::results::Owner for MarkerSlot {
    type Scope = ();
    fn scope(&self) -> &() {
        &()
    }
}

impl design::results::Kept for MarkerCopied {
    type Owner = MarkerSlot;
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Stays,
        press: design::results::Press::Nothing,
    };
    fn owner(&self) -> MarkerSlot {
        MarkerSlot
    }
}

/// One ticked question: its note and the line that holds its box, in its campaign.
#[derive(Clone, Debug, PartialEq, Eq)]
struct TickOwner {
    campaign: Uuid,
    note: Uuid,
    at: usize,
}

impl design::results::Owner for TickOwner {
    type Scope = Uuid;
    fn scope(&self) -> &Uuid {
        &self.campaign
    }
}

/// A tick's result lives while its note holds the ticked line where the tick wrote it (the
/// check of the notes after each load, `disk_changed`), until its Undo, or a new tick of that
/// line.
impl design::results::Kept for Tick {
    type Owner = TickOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> TickOwner {
        TickOwner {
            campaign: self.campaign,
            note: self.note,
            at: self.at,
        }
    }
}

/// A question that the DM ticked in Open questions, kept for its `Undo tick` by the note's ID
/// and the line that holds its box.
struct Tick {
    campaign: Uuid,
    note: Uuid,
    /// The body line that holds the box, and that line before the tick.
    at: usize,
    line: String,
    /// The question's words, and the name of the note that holds it: the result says
    /// `“<question>” ticked in <note>.` and its Undo `Undo tick “<question>” in <note>`.
    question: String,
    note_name: String,
}

/// What the DM did in a checks view: open a page at a heading, or tick the question at this
/// place in `Checks::questions`.
enum CheckAct {
    Open(usize, Option<String>),
    Tick(usize),
}

/// What the page's larger part shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum View {
    #[default]
    Reader,
    Questions,
    Problems,
}

#[derive(Default)]
pub struct WorldPage {
    search: String,
    selected: Option<Uuid>,
    pub reader: Reader,
    /// What only the app can do, for it to take after the page draws.
    pub ask_lookup: Option<String>,
    pub reload: bool,
    pub run_open: bool,
    new_kind: Option<&'static str>,
    new_name: String,
    note_type: String,
    repair: Option<crate::world::repair::Repair>,
    kept_repairs: Vec<crate::world::repair::Repair>,
    campaign: Option<Uuid>,
    /// An unsaved generated NPC is a held draft. Only Save NPC and Discard end it.
    generated: Option<crate::npc_generator::Generated>,
    /// Whether the held NPC's card is the view. If not, one row above the view names it.
    generated_shown: bool,
    /// The NPC that Discard let go, with its table facts, for one `Undo discard`.
    discarded_npc: Option<Held>,
    /// The page's result (`Note repaired.` or the NPC's discard), with its order.
    results: design::Results<PageSlot, PageKept>,
    generator_problems: Vec<crate::npc_generator::TableProblem>,
    generator_sources: Vec<String>,
    error: Option<PageError>,
    repaired: Option<Uuid>,
    /// Lookup's link found this page holding an unsaved NPC, so Lookup kept its own.
    lookup_kept: bool,
    /// The NPC Lookup holds now, refreshed by the app each frame while `lookup_kept`.
    pub lookup_npc: Option<String>,
    view: View,
    /// The checks for one load of the index.
    checks: Option<(u64, crate::world::checks::Checks)>,
    /// Check rows with nothing left that stay in the list (FE4): a row whose page was open when
    /// its last item went. Leaving the page keeps it, so no press moves the list; the next check
    /// of the notes (a new index) ends it.
    kept_empty: Vec<View>,
    /// Repair causes are read and parsed only when the index generation changes.
    broken_details: std::collections::BTreeMap<String, BrokenDetail>,
    /// Ticked questions with a live `Undo tick`.
    ticks: design::Results<TickOwner, Tick>,
    /// Why the last tick, or its Undo, wrote nothing.
    tick_error: Option<String>,
    /// The card's `Copy the marker` result (`MarkerCopied`).
    marker_copied: design::Results<MarkerSlot, MarkerCopied>,
}

/// One unreadable note's row words.
struct BrokenDetail {
    /// The row's second line.
    cause: String,
    /// The whole sentence, on hover.
    hover: String,
}

impl WorldPage {
    /// All file reads and repair parsing stay behind this generation check.
    fn refresh_checks(
        &mut self,
        index: &ContentIndex,
        mut read: impl FnMut(&str) -> Option<Result<String, FileFault>>,
    ) {
        if self
            .checks
            .as_ref()
            .is_some_and(|(generation, _)| *generation == index.generation())
        {
            return;
        }
        self.checks = Some((index.generation(), crate::world::checks::run(index)));
        self.kept_empty.clear();
        // A tick's Undo lives while its note holds the ticked line where the tick wrote it.
        self.ticks.disk_changed(|tick| {
            index
                .position(tick.note)
                .and_then(|at| read(&index.items[at].entry.path))
                .and_then(Result::ok)
                .is_some_and(|text| tick_file(&text, tick.at, &tick.line, false).is_some())
        });
        self.broken_details = index
            .broken_world
            .iter()
            .map(|(path, problem)| {
                let detail = match read(path) {
                    // The system's words stay out of the list; the repair pane keeps them.
                    Some(Err(fault)) => BrokenDetail {
                        cause: fault.cause().to_owned(),
                        hover: fault.headline(),
                    },
                    text => BrokenDetail {
                        cause: text
                            .and_then(Result::ok)
                            .and_then(|text| {
                                crate::content_workspace::repair_problem(path, &text).0
                            })
                            .unwrap_or_else(|| problem.clone()),
                        hover: problem.clone(),
                    },
                };
                (path.clone(), detail)
            })
            .collect();
    }

    pub fn dirty(&self) -> bool {
        self.repair
            .as_ref()
            .is_some_and(crate::world::repair::Repair::dirty)
            || self
                .kept_repairs
                .iter()
                .any(crate::world::repair::Repair::dirty)
    }

    fn keep_repair(&mut self) {
        if let Some(repair) = self.repair.take() {
            self.kept_repairs.push(repair);
        }
    }

    /// A campaign change is blocked while a draft or a generated NPC is unsaved, so these
    /// belong to the campaign that was open.
    fn set_campaign(&mut self, id: Uuid) {
        if self.campaign != Some(id) {
            self.campaign = Some(id);
            self.selected = None;
            self.repair = None;
            self.kept_repairs.clear();
            // Typed Log lines are kept, by campaign; the switch guard counts them as unsaved.
            self.reader = Reader::keep_log_lines(std::mem::take(&mut self.reader));
            self.generated = None;
            self.generated_shown = false;
            self.discarded_npc = None;
            self.marker_copied.owner_changed(&MarkerSlot);
            // The ticks' store keeps them: each shows only in its own campaign (`TickOwner`), and
            // a campaign change is none of a tick's end events.
            self.tick_error = None;
        }
    }

    pub fn reveal(&mut self, campaign: Uuid, id: Uuid) {
        self.set_campaign(campaign);
        self.view = View::Reader;
        self.new_kind = None;
        self.error = None;
        self.open_note(id);
    }

    /// Opens a note in the reader. A held NPC goes to its row above the reader.
    fn open_note(&mut self, id: Uuid) {
        self.selected = Some(id);
        self.view = View::Reader;
        self.keep_repair();
        self.leave_card();
        // The note takes the pane, as it does from an open repair.
        if matches!(self.error, Some(PageError::Unread(_))) {
            self.error = None;
        }
    }

    /// Another view takes the pane. A held NPC keeps its row; the Undo for a discarded NPC
    /// ends, as an Undo message ends when the DM moves on.
    fn leave_card(&mut self) {
        self.generated_shown = false;
        self.discarded_npc = None;
    }

    /// An unsaved generated NPC, which Lookup's link must not replace.
    pub fn has_generated(&self) -> bool {
        self.generated.is_some()
    }

    /// The name of the unsaved generated NPC, for the app's list of unsaved work.
    pub fn held_npc(&self) -> Option<&str> {
        self.generated.as_ref().map(|npc| npc.name.as_str())
    }

    /// The rail's link to the held NPC: its card becomes the view.
    pub fn open_held(&mut self) {
        self.show_held();
    }

    /// Lookup's link kept Lookup's NPC because this page holds its own. The page opens on
    /// its own card.
    pub fn lookup_kept(&mut self) {
        self.lookup_kept = true;
        self.show_held();
    }

    /// The held NPC's card becomes the view again.
    fn show_held(&mut self) {
        if self.generated.is_some() {
            self.view = View::Reader;
            self.keep_repair();
            self.new_kind = None;
            self.error = None;
            self.generated_shown = true;
        }
    }

    /// Whether the app should tell this page which NPC Lookup holds.
    pub fn wants_lookup_npc(&self) -> bool {
        self.lookup_kept
    }

    /// The NPC Lookup holds now. When Lookup lets it go, the reason line goes too.
    pub fn set_lookup_npc(&mut self, name: Option<String>) {
        self.lookup_kept &= name.is_some();
        self.lookup_npc = name;
    }

    /// Lookup's generated NPC, with the full help for using the campaign's own names.
    pub fn show_generated(&mut self, card: crate::lookup::GeneratedCard) {
        self.set_campaign(card.campaign);
        self.view = View::Reader;
        self.keep_repair();
        self.new_kind = None;
        self.error = None;
        self.lookup_kept = false;
        self.marker_copied.owner_changed(&MarkerSlot);
        self.generated = Some(card.npc);
        self.generated_shown = true;
        self.discarded_npc = None;
        self.generator_problems = card.problems;
        self.generator_sources = card.sources;
    }

    pub fn open_repair(&mut self, campaign: &Campaign, path: &str) {
        self.set_campaign(campaign.id);
        self.keep_repair();
        let loaded = if let Some(at) = self.kept_repairs.iter().position(|r| r.path == path) {
            Ok(self.kept_repairs.remove(at))
        } else {
            crate::world::repair::Repair::open(campaign, path)
        };
        match loaded {
            Ok(repair) => {
                self.repair = Some(repair);
                self.error = None;
                self.new_kind = None;
                self.leave_card();
            }
            Err(crate::world::repair::OpenError::Words(error)) => self.error = Some(error.into()),
            Err(crate::world::repair::OpenError::Unread(fault)) => {
                // The pane is about this file now, as an opened repair would be.
                self.error = Some(PageError::Unread(Unread {
                    fault,
                    seen: None,
                    again: false,
                }));
                self.new_kind = None;
                self.leave_card();
            }
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        index: Option<&Arc<ContentIndex>>,
        campaign: &Campaign,
        trackers: &mut dyn crate::world::TrackerUi,
    ) {
        self.set_campaign(campaign.id);
        // Lookup's reason line ends with the NPC it explains.
        if self.generated.is_none() {
            self.lookup_kept = false;
            self.lookup_npc = None;
        }
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("reader_surface"), egui::Id::new("world"))
        });
        let broken = index.map_or(0, |index| index.broken_world.len());
        // The fault records, found once per index: the count, its menu and the rows read them.
        let faults = index.map(|index| index.note_faults());
        let count = faults.map_or(0, |faults| faults.notes() + broken);
        let broken_links = faults.map_or(0, |faults| faults.listed().len());
        let mut create = None;
        // M0603 (ruling M21): the header is the shared title line: the title and its count,
        // New with its menu, the count of notes that need fixing with its menu, and the count
        // that cannot be read, which opens the first one's repair.
        const NEW: design::Word = crate::word!("New");
        let context = (count > 0).then(|| design::count(count, "note", "notes"));
        let fixing = design::count(broken_links, "note needs fixing", "notes need fixing");
        let unreadable = format!("{} cannot be read", design::count(broken, "note", "notes"));
        let mut open = None;
        let mut line = design::TitleLine::new("World")
            .context(context.as_deref())
            .menu(
                design::TitleAction::secondary(NEW)
                    .icon(icons::ADD)
                    .name("New ▾")
                    .blocked(
                        index
                            .is_none()
                            .then_some("Wait for the notes to load before creating a note."),
                    ),
                |ui| create = new_choices(ui),
            );
        if broken_links > 0 {
            line = line.status(
                design::Status {
                    icon: icons::WARN,
                    word: &fixing,
                    color: colors().result.warning,
                },
                |ui| {
                    if let (Some(index), Some(faults)) = (index, faults) {
                        for &at in faults.listed() {
                            let item = &index.items[at];
                            if design::link(ui, &item.entry.name).clicked() {
                                open = Some(item.entry.id);
                                ui.close();
                            }
                        }
                    }
                },
            );
        }
        // Every note this count holds cannot be read, which is an error in its row, this count
        // and its repair alike.
        if broken > 0 {
            line = line.status_press(design::Status {
                icon: icons::ERROR,
                word: &unreadable,
                color: colors().result.error,
            });
        }
        // The header's menus keep to the column under their control (as the Library's header,
        // issue #504): the list's content or the reader's, never over the list's scroll bar or
        // the splitter.
        let columns = header_columns(ui, index.is_some());
        let shown = line
            .page()
            .menu_room(move |control| {
                columns
                    .iter()
                    .copied()
                    .find(|column| column.x_range().contains(control.center().x))
                    .unwrap_or_else(|| columns[columns.len() - 1])
            })
            .show(ui);
        if let Some(id) = open {
            self.open_note(id);
        }
        if shown.status_pressed
            && let Some(index) = index
        {
            self.press_unreadable(campaign, index);
        }
        if let Some(kind) = create.take() {
            self.begin_new(kind, index.map(AsRef::as_ref));
        }
        let Some(index) = index else {
            ui.horizontal(|ui| {
                design::spinner(ui);
                ui.label(
                    Type::Small
                        .rich(ui, "Wait for the notes to load before creating a note.")
                        .color(colors().muted),
                );
            });
            return;
        };
        self.refresh_checks(index, |path| {
            crate::library::note_path(&campaign.path, path)
                .ok()
                .map(|file| {
                    std::fs::read_to_string(file).map_err(|error| FileFault::read(path, &error))
                })
        });
        let groups = index.world_groups(&self.search.trim().to_lowercase());
        if broken == 0
            && groups.is_empty()
            && self.search.trim().is_empty()
            && self.new_kind.is_none()
            && self.generated.is_none()
            && self.error.is_none()
        {
            design::empty_state(
                ui,
                icons::WORLD,
                "No world notes yet",
                "World notes are the Markdown files in this campaign’s World folder.",
                |ui| {
                    create = new_menu(ui);
                },
            );
            if let Some(kind) = create.take() {
                self.begin_new(kind, Some(index));
            }
            return;
        }
        list_column().show_inside(ui, |ui| {
            let width = ui.available_width();
            let _ = WORLD_SEARCH.search(
                ui,
                "Search the world",
                &mut self.search,
                "Find a note",
                width,
            );
            // The list column's own scroll: it keeps its lane, so nothing moves when the bar
            // comes or goes; its rows fill the column.
            design::ScrollColumn::new("world_list")
                .full_width()
                .show(ui, |ui| {
                    if groups.is_empty() && index.broken_world.is_empty() {
                        ui.label(
                            Type::Body
                                .rich(ui, "Nothing here matches.")
                                .color(colors().muted),
                        );
                    }
                    if self.search.trim().is_empty() {
                        self.check_rows(ui);
                    }
                    self.list(ui, index, &groups);
                    let broken: Vec<_> = index
                        .broken_world
                        .iter()
                        .filter(|(path, _)| {
                            path.to_lowercase().contains(&self.search.to_lowercase())
                        })
                        .collect();
                    if !broken.is_empty() {
                        // The list's last row left no gap after it: the heading takes its
                        // whole space, as the list's own headings do.
                        let spacing = ui.spacing().item_spacing.y;
                        if !groups.is_empty() {
                            ui.spacing_mut().item_spacing.y = 0.0;
                        }
                        design::heading(
                            ui,
                            design::Heading::Group {
                                title: "Cannot be read",
                                count: broken.len(),
                            },
                        );
                        ui.spacing_mut().item_spacing.y = spacing;
                        for (path, problem) in broken {
                            let detail = self.broken_details.get(path);
                            let open = self.repair.as_ref().is_some_and(|r| r.path == *path)
                                || matches!(
                                    &self.error,
                                    Some(PageError::Unread(unread)) if unread.fault.path == *path
                                );
                            let response = broken_row(
                                ui,
                                open,
                                path,
                                detail.map_or(problem.as_str(), |detail| &detail.cause),
                            )
                            .on_hover_text(detail.map_or(problem.as_str(), |detail| &detail.hover));
                            if response.clicked() {
                                self.open_repair(campaign, path);
                            }
                        }
                    }
                    let drafts: Vec<_> = self
                        .kept_repairs
                        .iter()
                        .filter(|r| {
                            r.dirty() && !index.broken_world.iter().any(|(path, _)| path == &r.path)
                        })
                        .map(|r| r.path.clone())
                        .collect();
                    if !drafts.is_empty() {
                        design::heading(
                            ui,
                            design::Heading::Group {
                                title: "Repair drafts",
                                count: drafts.len(),
                            },
                        );
                        for path in drafts {
                            if design::two_line_row(ui, ROW, false, &path, "Unsaved repair")
                                .clicked()
                            {
                                self.open_repair(campaign, &path);
                            }
                        }
                    }
                });
        });
        let mut nav = None;
        // An `L` gutter, so a quiet action that starts a line can reach into it and stay clear
        // of the World column's splitter.
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::L as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                self.held_row(ui);
                if let Some(repair) = &mut self.repair {
                    if repair.show(ui, campaign, self.run_open) {
                        self.repair_saved(campaign);
                    }
                    return;
                }
                if matches!(self.error, Some(PageError::Unread(..))) {
                    self.unread_pane(ui, index, campaign);
                    return;
                }
                if self.new_kind.is_some() || self.card_shown() || self.error.is_some() {
                    self.creation_form(ui, index, campaign);
                    return;
                }
                if self.view != View::Reader {
                    // The checks view is its pane's scroll column; Open questions' ticks and
                    // their reason sit in its footer.
                    let frame = PaneFrame::new(ui);
                    let questions = self.view == View::Questions;
                    let mut messages =
                        Messages::new(&frame, ("world_checks", questions), ui.available_width());
                    if questions {
                        self.questions_messages(&frame, &mut messages, campaign);
                    }
                    let paned = messages.pane(ui, Body::Content, |ui| {
                        design::reader::measure(ui, |ui| self.checks_view(ui, index))
                    });
                    if let Some(press) = paned.pressed
                        && press.id == crate::world::FooterAct::Untick
                        && let Some(ticket) = press.ticket
                    {
                        self.untick(index, campaign, ticket);
                    }
                    match paned.body {
                        Some(CheckAct::Open(page, heading)) => {
                            self.open_note(index.items[page].entry.id);
                            self.reader.show_heading(heading);
                        }
                        Some(CheckAct::Tick(at)) => self.tick_question(index, campaign, at),
                        None => {}
                    }
                    return;
                }
                nav = self.reader.choices(ui, index);
                let Some(item) = self
                    .selected
                    .and_then(|id| index.position(id))
                    .map(|position| &index.items[position])
                else {
                    // The result sits in the pane's footer, as it does below a note (round 26
                    // FF1), so the empty state's advice keeps its place.
                    let frame = PaneFrame::new(ui);
                    let mut messages = Messages::new(&frame, "world_footer", ui.available_width());
                    self.page_messages(&frame, &mut messages, None);
                    // A Log line whose entry is gone says its words here too.
                    self.reader
                        .gone_log_messages(&frame, &mut messages, Some(campaign.id));
                    let paned = messages.pane(ui, Body::Content, |ui| {
                        design::empty_state(
                            ui,
                            icons::WORLD,
                            "No note open",
                            "Choose one from the list, or make a new one.",
                            |_| {},
                        );
                    });
                    if let Some(press) = paned.pressed {
                        let _ = self.footer_press(ui.ctx(), press, None, campaign, trackers);
                    }
                    return;
                };
                // A later result on the repaired note replaces `Note repaired.`.
                if self.repaired == Some(item.entry.id)
                    && self.reader.has_change_result(campaign, &item.entry.path)
                {
                    self.repaired = None;
                }
                // Every result, reason and Undo of the open note sits in the pane's footer,
                // below the reader, so none of them moves the note (round 26). The reader is the
                // pane's scroll column, one for each note.
                let frame = PaneFrame::new(ui);
                let mut messages = Messages::new(
                    &frame,
                    ("world_reader", item.entry.id),
                    ui.available_width(),
                );
                self.reader
                    .footer(&frame, &mut messages, item, index, Some(campaign));
                self.page_messages(&frame, &mut messages, Some(item.entry.id));
                trackers.history_footer(item.entry.id, &frame, &mut messages);
                // As in Library, a wide table may use the pane; prose keeps its measure.
                // The reader pane is always the editor form (P2): an edit form's text draws in its
                // region and gives way, so Save stays in view under a long note from the first
                // frame the form shows; with no form, the unused region is a content pane.
                let reader = &mut self.reader;
                let paned = crate::markdown_view::with_wide_tables(ui, |ui| {
                    messages.pane_editor(ui, |ui, region| {
                        reader.read_in_pane(
                            ui,
                            item,
                            index,
                            Some(campaign),
                            Some(&mut *trackers),
                            Some(region),
                        )
                    })
                });
                nav = nav.take().or(paned.body);
                if let Some(press) = paned.pressed {
                    let pressed =
                        self.footer_press(ui.ctx(), press, Some(item), campaign, trackers);
                    nav = nav.take().or(pressed);
                }
            });
        if let Some(kind) = create.take() {
            self.begin_new(kind, Some(index));
        }
        if let Some(nav) = nav {
            self.take_nav(nav, index);
        }
    }

    /// New note, NPC or location keeps a held NPC in its row. Generate NPC shows the held
    /// NPC's card, where Reroll rolls another.
    fn begin_new(&mut self, kind: &'static str, index: Option<&ContentIndex>) {
        if kind == "generate" && self.generated.is_some() {
            self.show_held();
            return;
        }
        self.view = View::Reader;
        self.error = None;
        self.keep_repair();
        self.new_kind = (kind != "generate").then_some(kind);
        if kind != "generate" {
            self.leave_card();
        } else if let Some(index) = index {
            self.roll(index);
        }
    }

    /// A new NPC for the card. A failed roll keeps the NPC the card had.
    fn roll(&mut self, index: &ContentIndex) {
        let tables = index.npc_tables();
        match crate::npc_generator::roll(&tables) {
            Ok(npc) => {
                self.generated = Some(npc);
                self.marker_copied.owner_changed(&MarkerSlot);
                self.generated_shown = true;
                self.discarded_npc = None;
                self.generator_sources = tables.sources;
                self.generator_problems = tables.problems;
            }
            Err(error) => self.error = Some(error.into()),
        }
    }

    /// Whether the held NPC's card is the view.
    fn card_shown(&self) -> bool {
        self.generated.is_some() && self.generated_shown
    }

    /// Above another view: the held NPC's one row. The Undo for a discarded NPC is a result, so
    /// it sits in the pane footer.
    fn held_row(&mut self, ui: &mut egui::Ui) {
        if let Some(npc) = self.generated.as_ref().filter(|_| !self.generated_shown)
            && crate::world::unsaved_npc_row(ui, &npc.name)
        {
            self.show_held();
        }
    }

    /// The page's result, while its own state holds: the discarded NPC while no NPC is held,
    /// `Note repaired.` while the repaired note is open.
    fn page_messages<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, crate::world::FooterAct>,
        open: Option<Uuid>,
    ) {
        let Some(shown) = self.results.get(&PageSlot) else {
            return;
        };
        let shows = match &shown.value().0 {
            crate::world::PageResult::Discarded(_) => {
                self.discarded_npc.is_some() && self.generated.is_none()
            }
            crate::world::PageResult::Repaired => open.is_some() && self.repaired == open,
        };
        if shows {
            messages.kept(shown.outcome(frame, |kept| kept.0.outcome()));
        }
    }

    /// A press in a pane that shows the reader's footer: the page's own result and the
    /// trackers' are taken here, the reader's by the reader.
    fn footer_press(
        &mut self,
        ctx: &egui::Context,
        press: crate::design::outcome::Press<crate::world::FooterAct>,
        item: Option<&crate::lookup::Item>,
        campaign: &Campaign,
        trackers: &mut dyn crate::world::TrackerUi,
    ) -> Option<Nav> {
        match press.id {
            crate::world::FooterAct::Page => {
                let ticket = press.ticket?;
                // Only the discard has an Undo; a replaced result (`Gone`) does nothing.
                if self
                    .results
                    .undo_by(ticket, |kept| match kept.0 {
                        crate::world::PageResult::Discarded(_) => Ok(()),
                        crate::world::PageResult::Repaired => Err(()),
                    })
                    .is_ok()
                {
                    self.undo_discard();
                }
                None
            }
            crate::world::FooterAct::Trackers => {
                trackers.undo_remove(press.ticket);
                None
            }
            _ => self
                .reader
                .footer_act(ctx, press, item.map(|item| item.entry.id), campaign),
        }
    }

    /// `Undo discard`: the same NPC, with its table facts, on its card again.
    fn undo_discard(&mut self) {
        if let Some((npc, problems, sources)) = self.discarded_npc.take() {
            self.generated = Some(npc);
            self.generator_problems = problems;
            self.generator_sources = sources;
            self.show_held();
        }
    }

    /// A note that the system will not read. The pane names the file and how to recover it;
    /// the system's words wait in Technical details. Nothing here writes the file.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn unread_pane(&mut self, ui: &mut egui::Ui, index: &ContentIndex, campaign: &Campaign) {
        let Some(fault) = self.unread_view(index, campaign) else {
            return;
        };
        design::reader::title(ui, &fault.file);
        // No editor link: an editor cannot open a file that Splat cannot read either.
        crate::world::folder_file_line(ui, &campaign.path, &fault.path, false);
        // The title and file line name the file, so the error says `this note` (FG6). M0604
        // (rulings M20, M23): the note's fault in place of its page, its recovery steps in its
        // words, then `Try again` and `Close note` with the shared gap (round 18's `L` apart
        // gives way). The recovery is prose: the words keep the reader measure (round 24 AG2).
        #[derive(Clone, Copy, PartialEq)]
        enum Pane {
            Again,
            Close,
        }
        let words = fault.words.clone();
        let chosen = at_reader_measure(ui, |ui| {
            PagePlaceholder::new(&crate::world::ReaderFault::Unreadable(words))
                .choice(Choice::new(MessageChoice::Choice("Try again"), Pane::Again))
                .choice(Choice::new(
                    MessageChoice::Choice("Close note"),
                    Pane::Close,
                ))
                .show(ui)
        });
        let (again, cancel) = (chosen == Some(Pane::Again), chosen == Some(Pane::Close));
        design::gap(ui, design::Gap::S);
        design::quiet_section(ui, "Technical details", |ui| {
            ui.add(
                egui::Label::new(Type::Small.rich(ui, &fault.system).color(colors().muted)).wrap(),
            );
        });
        if cancel {
            self.error = None;
        } else if again {
            // Read the folder again first, so the list's cause is current too.
            self.unread_again();
        }
    }

    /// A creation form or the held NPC's card, with the pane's footer: the card's `Copy the
    /// marker` result, the line that names the NPC Lookup keeps (round 28 rows 7 and 44: a
    /// state of the card, so the card never moves when it comes), and the error of the last
    /// Create or Save (round 27 FG2: no body places). The form is the pane's scroll column.
    fn creation_form(&mut self, ui: &mut egui::Ui, index: &ContentIndex, campaign: &Campaign) {
        let form = self.new_kind.is_some() || self.card_shown();
        let frame = PaneFrame::new(ui);
        let mut messages: Messages<'_, crate::world::FooterAct> =
            Messages::new(&frame, "world_creation", ui.available_width());
        let card = self.generated.as_ref().filter(|_| self.generated_shown);
        if let Some(npc) = card {
            // Two NPCs: the one Lookup keeps and the one on this card. Each is named, so the DM
            // knows which to save and which moves here. It is a state: an info notice.
            if let Some(name) = self.lookup_npc.as_ref().filter(|_| self.lookup_kept) {
                messages.outcome(Notice::new(crate::world::said(&kept_in_lookup(
                    name, &npc.name,
                ))));
            }
            if let Some(shown) = self
                .marker_copied
                .get(&MarkerSlot)
                .filter(|shown| shown.value().0 == npc.name)
            {
                // `lookup::MARKER_COPIED`.
                messages.kept(shown.outcome(&frame, |_| {
                    Said::new(crate::sentence!(
                        "Copied. Change 1d6 to the number of names."
                    ))
                    .into()
                }));
            }
        }
        if let Some(PageError::Words(error)) = &self.error
            && form
        {
            messages.outcome(FaultMessage::new(&crate::world::ReaderFault::Action(
                error.clone(),
            )));
        }
        let _ = messages.pane(ui, Body::Content, |ui| {
            self.creation_body(ui, index, campaign)
        });
    }

    /// Inline creation (round 27 ruling): the Label title and the fields, the named Create
    /// action after the last field on that field's line, centred on it, then one kept line
    /// below, as tall as a control: quiet `Cancel` at its start, then the reason or the new
    /// file's path. Typing moves nothing.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state`, and raw egui calls (design::results::STAGED_ALLOWS)"
    )]
    fn creation_body(&mut self, ui: &mut egui::Ui, index: &ContentIndex, campaign: &Campaign) {
        if let Some(kind) = self.new_kind {
            design::heading(
                ui,
                design::Heading::Label(match kind {
                    "npc" => "New NPC",
                    "location" => "New location",
                    _ => "New note",
                }),
            );
            let reason = self
                .new_name
                .trim()
                .is_empty()
                .then_some("Type a name first.");
            let label = match kind {
                "npc" => "Create NPC",
                "location" => "Create location",
                _ => "Create note",
            };
            let create = || {
                if self.run_open {
                    design::Action::secondary(label)
                } else {
                    design::Action::primary(label)
                }
            };
            let create_width = create().natural_width(ui);
            // The last field shares its line with Create; the others take the form's width.
            let created = {
                let name = &mut self.new_name;
                let note_type = &mut self.note_type;
                let field =
                    |ui: &mut egui::Ui,
                     last: bool,
                     draw: &mut dyn FnMut(&mut egui::Ui) -> egui::Response| {
                        if !last {
                            draw(ui);
                            return false;
                        }
                        let width = (ui.available_width() - create_width - design::S)
                            .clamp(design::XXL, design::FORM_WIDTH);
                        // Create sits `S` after the field's column, centred on the field itself.
                        // The column's height is the line's, so the kept line follows it.
                        let column = ui.allocate_ui_with_layout(
                            egui::vec2(width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| draw(ui).rect,
                        );
                        let field = column.inner;
                        let at = egui::Rect::from_min_size(
                            egui::pos2(
                                column.response.rect.right() + design::S,
                                field.center().y - design::Density::Scan.control() / 2.0,
                            ),
                            egui::vec2(create_width, design::Density::Scan.control()),
                        );
                        crate::world::in_rect(ui, at, |ui| {
                            create().blocked(reason).show(ui).clicked()
                        })
                    };
                let mut name_field = |ui: &mut egui::Ui| {
                    design::labelled_field(ui, NEW_NAME, "Name", name, None, None)
                };
                if kind == "note" {
                    field(ui, false, &mut name_field);
                    field(ui, true, &mut |ui: &mut egui::Ui| {
                        design::labelled_field(
                            ui,
                            NEW_NOTE_TYPE,
                            "Type (optional)",
                            note_type,
                            Some("Leave empty for a plain note. For example: faction or quest."),
                            None,
                        )
                    })
                } else {
                    field(ui, true, &mut name_field)
                }
            };
            if created {
                self.create(campaign);
            }
            // The kept line: Cancel at its start, then the reason, or the file the name makes.
            ui.horizontal(|ui| {
                ui.set_min_height(design::Density::Read.control());
                if design::Action::quiet("Cancel")
                    .density(design::Density::Read)
                    .line_start()
                    .show(ui)
                    .clicked()
                {
                    self.new_kind = None;
                }
                design::gap_to_next(ui, design::Gap::L);
                if reason.is_some() {
                    design::blocked_reason(ui, reason);
                } else if kind == "npc" {
                    let path = crate::world::new_entry_path(Some(index), &self.new_name, "npc");
                    ui.add(
                        egui::Label::new(
                            Type::Small
                                .rich(ui, format!("Will create {path}"))
                                .color(colors().muted),
                        )
                        .wrap(),
                    );
                }
            });
        }
        if let Some(npc) = self.generated.as_ref().filter(|_| self.generated_shown) {
            let choice = crate::lookup::generator_card(
                ui,
                npc,
                &self.generator_problems,
                &self.generator_sources,
                self.run_open,
                crate::lookup::NamesHelp::Marker,
                &crate::world::new_entry_path(Some(index), &npc.name, "npc"),
            );
            for (pressed, act) in [
                (choice.copied_marker, CardAct::CopiedMarker),
                (choice.save, CardAct::Save),
                (choice.again, CardAct::Again),
                (choice.discard, CardAct::Discard),
            ] {
                if pressed {
                    self.card_act(act, campaign, index);
                }
            }
        }
        // With no form and no card, the error is the page's content, with its Cancel; under a
        // form or a card it is in the footer (`creation_form`).
        if let Some(PageError::Words(error)) = self.error.clone()
            && self.new_kind.is_none()
            && !self.card_shown()
        {
            // M0607 (ruling M20): the error in place of the page's body, with its Cancel at the
            // message text.
            let cancel = PagePlaceholder::new(&crate::world::ReaderFault::Action(error))
                .choice(Choice::new(MessageChoice::Choice("Cancel"), ()))
                .show(ui)
                .is_some();
            if cancel {
                self.error = None;
            }
        }
    }

    /// The two lists of what the notes still need, above the notes themselves.
    fn check_rows(&mut self, ui: &mut egui::Ui) {
        for (view, title, detail, open) in self.check_rows_shown() {
            if design::two_line_row(ui, ROW, open, title, &detail).clicked() {
                self.press_check_row(view);
            }
        }
    }

    /// The open questions, or the problems. Returns the page a press asks for, with the
    /// heading to show.
    fn checks_view(&mut self, ui: &mut egui::Ui, index: &ContentIndex) -> Option<CheckAct> {
        let (_, checks) = self.checks.as_ref()?;
        let mut open = None;
        let mut tick = None;
        let note = |ui: &mut egui::Ui, words: &str| {
            ui.add(egui::Label::new(Type::Small.rich(ui, words).color(colors().muted)).wrap());
        };
        let more = |ui: &mut egui::Ui, count: usize| {
            if count > CHECK_ROWS {
                note(ui, &design::count(count - CHECK_ROWS, "more", "more"));
            }
        };
        // One page's lines, then its name. The page answers "which questions are open?", so
        // each line is `Body` first and opens the note at its heading; the note that holds them
        // follows in `Small` (round 26 FE4). An open question has its box before its words.
        let questions = self.view == View::Questions;
        let mut by_page = |ui: &mut egui::Ui, found: &[crate::world::checks::Found]| {
            let mut place = 0;
            for group in found.chunk_by(|a, b| a.page == b.page).take(CHECK_ROWS) {
                ui.push_id(group[0].page, |ui| {
                    for line in group {
                        let clicked = if questions {
                            let (ticked, clicked) = question_row(ui, index, line);
                            if ticked {
                                tick = Some(place);
                            }
                            clicked
                        } else {
                            design::wrapped_link(ui, &line.words).clicked()
                        };
                        if clicked {
                            open = Some(CheckAct::Open(line.page, line.heading.clone()));
                        }
                        place += 1;
                    }
                    ui.horizontal_wrapped(|ui| {
                        if questions {
                            design::gap(ui, design::Gap::XXL);
                        }
                        ui.spacing_mut().item_spacing.x = 0.0;
                        note(ui, "In ");
                        if design::small_link(ui, &index.items[group[0].page].entry.name).clicked()
                        {
                            open = Some(CheckAct::Open(group[0].page, None));
                        }
                    });
                    design::gap(ui, design::Gap::S);
                });
            }
            more(ui, found.chunk_by(|a, b| a.page == b.page).count());
        };
        if self.view == View::Questions {
            design::reader::title(ui, "Open questions");
            design::reader::subtitle(
                ui,
                "Each unticked task line in the world’s notes. Tick a question when you decide.",
            );
            design::gap(ui, design::Gap::S);
            if checks.questions.is_empty() {
                note(ui, "No open questions.");
            }
            by_page(ui, &checks.questions);
            return tick.map(CheckAct::Tick).or(open);
        }
        design::reader::title(ui, PROBLEMS_TITLE);
        design::reader::subtitle(ui, PROBLEMS_INTRO);
        if !checks.loose_dates.is_empty() {
            design::heading(ui, design::Heading::Section("Dates outside events"));
            note(
                ui,
                "A date in words belongs on an event’s note. An event has a date that Splat can put in order.",
            );
            by_page(ui, &checks.loose_dates);
        }
        let mut pages = |ui: &mut egui::Ui, pages: &[usize]| {
            for &page in pages {
                if design::link(ui, &index.items[page].entry.name).clicked() {
                    open = Some(CheckAct::Open(page, None));
                }
            }
        };
        if !checks.wanted.is_empty() {
            design::heading(ui, design::Heading::Section("Pages to write"));
            note(
                ui,
                "A link names each of these, and no note has the name. The note that most lines want is first.",
            );
            for (name, lines, from) in checks.wanted.iter().take(CHECK_ROWS) {
                ui.horizontal_wrapped(|ui| {
                    ui.label(Type::Body.rich(ui, name).color(colors().text));
                    note(
                        ui,
                        &format!("{} in", design::count(*lines, "line", "lines")),
                    );
                    pages(ui, from);
                });
            }
            more(ui, checks.wanted.len());
        }
        for (title, words, found) in [
            (
                "No summary",
                "Start each of these with one paragraph that says what the subject is. Lists and Lookup show it.",
                &checks.no_summary,
            ),
            (
                "Nothing points here",
                "These notes show on no other note. No line names them, they name no other note, and they are in no place and part of no group.",
                &checks.alone,
            ),
        ] {
            if !found.is_empty() {
                design::heading(ui, design::Heading::Section(title));
                note(ui, words);
                ui.horizontal_wrapped(|ui| pages(ui, &found[..found.len().min(CHECK_ROWS)]));
                more(ui, found.len());
            }
        }
        for (title, words, found) in [
            (
                "One name on two notes",
                "A line that uses the name shows on each of these notes.",
                &checks.shared_names,
            ),
            (
                "Said twice",
                "The same sentence is on each of these notes. Keep it on the note of its subject.",
                &checks.repeated,
            ),
        ] {
            if !found.is_empty() {
                design::heading(ui, design::Heading::Section(title));
                note(ui, words);
                for (said, on) in found.iter().take(CHECK_ROWS) {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(Type::Small.rich(ui, said).color(colors().text));
                        pages(ui, on);
                    });
                }
                more(ui, found.len());
            }
        }
        open
    }

    /// Tick the question at this place in Open questions: one character of its note, at the
    /// line its box was drawn from, as read. A changed line, or an open edit form of the note,
    /// writes nothing and says why in the pane's footer.
    fn tick_question(&mut self, index: &ContentIndex, campaign: &Campaign, place: usize) {
        let Some(found) = self
            .checks
            .as_ref()
            .and_then(|(_, checks)| checks.questions.get(place))
        else {
            return;
        };
        let Some((at, line)) = found.open_box.clone() else {
            return;
        };
        let item = &index.items[found.page];
        let (note, name) = (item.entry.id, item.entry.name.clone());
        let question = found.words.clone();
        let result = Self::write_tick(&self.reader, campaign, item, at, &line, true);
        self.tick_error = result.err();
        if self.tick_error.is_none() {
            // A new tick of this line replaces its earlier result (`record`).
            self.ticks.record(Tick {
                campaign: campaign.id,
                note,
                at,
                line,
                question,
                note_name: name,
            });
            self.reload = true;
        }
    }

    /// `Undo tick`: the box of that question, in that note at that line, is open again while
    /// the line holds the tick. The press's ticket names the tick; a replaced or ended one does
    /// nothing; a failed write keeps the result and says why.
    fn untick(
        &mut self,
        index: &ContentIndex,
        campaign: &Campaign,
        ticket: crate::design::results::Ticket,
    ) {
        let reader = &self.reader;
        let result = self.ticks.undo_by(ticket, |tick| {
            if tick.campaign != campaign.id {
                return Err(
                    "This tick belongs to another campaign. Nothing was changed.".to_owned(),
                );
            }
            match index.position(tick.note) {
                Some(position) => Self::write_tick(
                    reader,
                    campaign,
                    &index.items[position],
                    tick.at,
                    &tick.line,
                    false,
                ),
                None => Err("The note of this question is gone. Nothing was changed.".to_owned()),
            }
        });
        match result {
            Ok(_) => {
                self.tick_error = None;
                self.reload = true;
            }
            Err(crate::design::results::ByTicket::Failed(error)) => self.tick_error = Some(error),
            Err(crate::design::results::ByTicket::Gone) => {}
        }
    }

    /// Set one question's box in its note through the World's one write path, which writes
    /// only when the file still holds what this act read.
    fn write_tick(
        reader: &Reader,
        campaign: &Campaign,
        item: &crate::lookup::Item,
        at: usize,
        line: &str,
        done: bool,
    ) -> Result<(), String> {
        let name = &item.entry.name;
        if reader.editing(item.entry.id, campaign)
            || reader.edited_elsewhere.contains(&item.entry.id)
        {
            return Err(format!("Save or discard your edit of {name} first."));
        }
        let before = crate::world::file_fault::read_note(&campaign.path, &item.entry.path)?;
        // Another note that took this file's place outside Splat is not this question's note.
        if splat::content::note_id(&before) != Some(item.entry.id) {
            return Err(format!("{name} changed on disk. Nothing was written."));
        }
        let after = tick_file(&before, at, line, done)
            .ok_or_else(|| format!("This question changed in {name}. Nothing was written."))?;
        crate::library::write_note(&campaign.path, &item.entry.path, Some(&before), &after)
            .map(|_| ())
    }

    /// Open questions' footer: each tick's result with its Undo, newest first (`Done` with
    /// `Verb::Tick`: one verb for the result and its Undo), then why the last tick wrote nothing.
    fn questions_messages<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, crate::world::FooterAct>,
        campaign: &Campaign,
    ) {
        for shown in self.ticks.newest_first(&campaign.id) {
            messages.kept(shown.outcome(frame, |tick| {
                Done::new(Verb::Tick, format!("\u{201c}{}\u{201d}", tick.question))
                    .object_first()
                    .within(tick.note_name.clone())
                    .undo(crate::world::FooterAct::Untick)
                    .into()
            }));
        }
        if let Some(error) = &self.tick_error {
            messages.outcome(FaultMessage::new(&crate::world::ReaderFault::Action(
                error.clone(),
            )));
        }
    }

    fn list(&mut self, ui: &mut egui::Ui, index: &ContentIndex, groups: &WorldGroups) {
        let spacing = ui.spacing().item_spacing.y;
        for (at, ((kind, title), rows)) in groups.iter().enumerate() {
            // A group's rows have no gap between them, so after the first group nothing has left
            // the usual gap after it, and a heading takes its whole space (`L`) itself.
            if at > 0 {
                ui.spacing_mut().item_spacing.y = 0.0;
            }
            if *kind == 3 {
                // A folder's heading is not one of the part's kinds: it takes the space that the
                // part gives a group heading. No token is the `10` that follows the first group
                // after the search field, so that one keeps its old `XS`.
                design::gap(
                    ui,
                    if at > 0 {
                        design::Gap::L
                    } else {
                        design::Gap::XS
                    },
                );
                ui.spacing_mut().item_spacing.y = spacing;
                design::folder_group_heading(ui, design::FolderGroup::of_path(title), rows.len());
            } else {
                design::heading(
                    ui,
                    design::Heading::Group {
                        title,
                        count: rows.len(),
                    },
                );
            }
            ui.spacing_mut().item_spacing.y = 0.0;
            for &(position, depth) in rows {
                let entry = &index.items[position].entry;
                let ListRow {
                    name,
                    detail,
                    narrow,
                    ..
                } = list_row(index, position, depth);
                let selected = self.row_selected(entry.id);
                // A row is set in one step for each level of depth.
                design::indent(ui, depth, design::Gap::S, |ui| {
                    let response = if detail.is_empty() {
                        design::row(
                            ui,
                            design::Density::Read.row(),
                            selected,
                            &format!("{} · ", entry.name),
                            |ui, rect| {
                                let galley = design::one_line(
                                    ui,
                                    &entry.name,
                                    Type::Body.font(ui),
                                    colors().text,
                                    rect.width() - design::S * 2.0,
                                );
                                ui.painter().galley(
                                    egui::pos2(
                                        rect.left() + design::S,
                                        rect.center().y - galley.size().y / 2.0,
                                    ),
                                    galley,
                                    colors().text,
                                );
                            },
                        )
                    } else {
                        marked_row(ui, selected, &name, &detail, narrow.as_deref())
                            .on_hover_text(&entry.path)
                    };
                    if response.clicked() {
                        self.open_note(entry.id);
                    }
                });
            }
            ui.spacing_mut().item_spacing.y = spacing;
        }
    }
}

/// The list of notes' column: one definition for the column and for the header's menus.
fn list_column() -> design::Column {
    design::Column::left("world_list_panel", LIST_WIDTH, 190.0..=480.0).frame(
        egui::Frame::new().inner_margin(egui::Margin {
            right: design::S as i8,
            ..Default::default()
        }),
    )
}

/// The page's columns that `ui` lays out next, left to right, for the header's menus
/// (`TitleLine::menu_room`): the list's and the reader's, each its content, `S` inside its
/// panel, where the list's scroll bar starts. The header asks the list for its width before it
/// draws (`Column::width_within`). While the notes load there is no list: one column.
fn header_columns(ui: &egui::Ui, listed: bool) -> Vec<egui::Rect> {
    let mut room = ui.available_rect_before_wrap();
    let mut columns = Vec::new();
    if listed {
        let width = list_column().width_within(ui.ctx(), room);
        columns.push(egui::Rect::from_x_y_ranges(
            room.left()..=room.left() + width,
            room.y_range(),
        ));
        room.min.x += width;
    }
    columns.push(room);
    columns
        .into_iter()
        .map(|column| column.shrink2(egui::vec2(design::S, 0.0)))
        .collect()
}

/// The same next step in the header and in an empty reader: the empty reader's `New ▾` with its
/// menu (the header's is its title line's menu action, `TitleLine::menu`).
fn new_menu(ui: &mut egui::Ui) -> Option<&'static str> {
    let label = format!("New {}", icons::DOWN);
    let response = design::Action::secondary(&label)
        .name("New ▾")
        .icon(icons::ADD)
        .show(ui);
    design::layout::menu_popup(ui, &response)
        .show(new_choices)
        .and_then(|shown| shown.inner)
}

/// New's choices, in its menu.
fn new_choices(ui: &mut egui::Ui) -> Option<&'static str> {
    let mut choice = None;
    for (word, kind) in [
        // `…` marks a choice that asks for a name first. Generate NPC makes its card at once.
        ("Note…", "note"),
        ("NPC…", "npc"),
        ("Location…", "location"),
        ("Generate NPC", "generate"),
    ] {
        // M0609: a menu's choices are the shared quiet actions, as in the Library's menus.
        if design::Action::quiet(word).show(ui).clicked() {
            choice = Some(kind);
            ui.close();
        }
    }
    choice
}

/// The room before a question's words: its box's 24 px target and an `S` gap.
const QUESTION_BOX: f32 = design::Gap::XXL.size();

/// One open question: its box, when Splat can write its note, then its words as a link that
/// opens the note at its heading. A note that Splat does not write keeps the box's room, so the
/// words keep one column. Returns whether the box and the words were clicked.
fn question_row(
    ui: &mut egui::Ui,
    index: &ContentIndex,
    found: &crate::world::checks::Found,
) -> (bool, bool) {
    let row = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let (slot, _) =
            ui.allocate_exact_size(egui::vec2(QUESTION_BOX, design::XL), egui::Sense::hover());
        let mut ticked = false;
        if found.open_box.is_some() && crate::world::edit::editable(&index.items[found.page], index)
        {
            let mut cell = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(egui::Rect::from_min_size(
                        slot.min,
                        egui::vec2(design::XL, design::XL),
                    ))
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            cell.spacing_mut().interact_size.y = design::XL;
            let mut done = false;
            let response = cell.add(egui::Checkbox::without_text(&mut done));
            design::named(ui, &response, &format!("Open: {}", found.words));
            ticked = response.clicked();
        }
        let clicked = ui
            .vertical(|ui| design::wrapped_link(ui, &found.words).clicked())
            .inner;
        (ticked, clicked)
    });
    design::register_row(ui, row.response.rect);
    row.inner
}

/// One row of the list of notes, as any view draws it.
pub struct ListRow {
    pub position: usize,
    pub id: Uuid,
    /// The row's title: the note's name, or its file's name when two notes share it.
    pub name: String,
    /// The second line (a fault, a date and status, a file), or empty for a one-line row.
    pub detail: String,
    /// The short form of a fault's words, when the whole does not fit.
    pub narrow: Option<String>,
    /// Its depth under its group: one `S` step each.
    pub depth: usize,
    /// The note's path, on hover.
    pub path: String,
}

/// One group of the list: its kind (3 is a folder), its title and its rows.
pub struct ListGroup {
    pub folder: bool,
    pub title: String,
    pub rows: Vec<ListRow>,
}

/// What the page's larger part shows now, in the order `show` decides it.
pub enum Pane {
    Repair,
    Unread,
    /// A creation form, the held NPC's card, or the page's error in place of the body.
    Creation,
    Checks(View),
    /// The note at this place in the index.
    Note(usize),
    /// No note open.
    Empty,
}

/// A press in the held NPC's card.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardAct {
    CopiedMarker,
    Save,
    Again,
    Discard,
}

/// The held NPC's card as data.
pub struct CardView {
    pub npc: crate::npc_generator::Generated,
    pub problems: Vec<crate::npc_generator::TableProblem>,
    pub sources: Vec<String>,
    pub save_path: String,
    /// `<name> is not saved. It is in Lookup. …`, while Lookup keeps its own NPC.
    pub kept_in_lookup: Option<String>,
    /// `Copy the marker`'s result shows.
    pub marker_copied: bool,
}

/// The unreadable note's pane as data.
pub struct UnreadView {
    pub path: String,
    pub file: String,
    /// The fault and its recovery, the words of its message.
    pub words: String,
    /// The system's own words, in Technical details.
    pub system: String,
}

/// The page's view of the World, for a view that is not egui (the GPUI port). Each act here is
/// the one the egui page runs, which calls it too.
impl WorldPage {
    /// The list's search words.
    pub fn search(&self) -> &str {
        &self.search
    }

    pub fn set_search(&mut self, words: &str) {
        words.clone_into(&mut self.search);
    }

    /// The page belongs to this campaign: a new one starts the page again (`set_campaign`), and
    /// a reason line about Lookup's NPC ends with the NPC it explains.
    pub fn follow_campaign(&mut self, campaign: Uuid) {
        self.set_campaign(campaign);
        if self.generated.is_none() {
            self.lookup_kept = false;
            self.lookup_npc = None;
        }
    }

    /// The checks of the notes, read once per load of the index; `campaign` is the folder the
    /// repairs read from.
    pub fn refresh(&mut self, index: &ContentIndex, campaign: &Campaign) {
        self.refresh_checks(index, |path| {
            crate::library::note_path(&campaign.path, path)
                .ok()
                .map(|file| {
                    std::fs::read_to_string(file).map_err(|error| FileFault::read(path, &error))
                })
        });
    }

    /// The header's count words: `3 notes`, `2 notes need fixing`, `1 note cannot be read`,
    /// each when it is not zero.
    pub fn header_counts(index: &ContentIndex) -> (Option<String>, Option<String>, Option<String>) {
        let broken = index.broken_world.len();
        let faults = index.note_faults();
        let count = faults.notes() + broken;
        let fixing = faults.listed().len();
        (
            (count > 0).then(|| design::count(count, "note", "notes")),
            (fixing > 0).then(|| design::count(fixing, "note needs fixing", "notes need fixing")),
            (broken > 0)
                .then(|| format!("{} cannot be read", design::count(broken, "note", "notes"))),
        )
    }

    /// The header's `cannot be read` count: the first such note's repair opens.
    pub fn press_unreadable(&mut self, campaign: &Campaign, index: &ContentIndex) {
        if let Some((path, _)) = index.broken_world.first() {
            self.search.clear();
            self.open_repair(campaign, path);
        }
    }

    /// The page has nothing at all: no note, no search, no form, no NPC and no error.
    pub fn is_empty(&self, index: &ContentIndex, groups: &WorldGroups) -> bool {
        index.broken_world.is_empty()
            && groups.is_empty()
            && self.search.trim().is_empty()
            && self.new_kind.is_none()
            && self.generated.is_none()
            && self.error.is_none()
    }

    /// The list's groups and rows for the search words.
    pub fn list_groups(&self, index: &ContentIndex) -> Vec<ListGroup> {
        let groups = index.world_groups(&self.search.trim().to_lowercase());
        groups
            .iter()
            .map(|((kind, title), rows)| ListGroup {
                folder: *kind == 3,
                title: title.clone(),
                rows: rows
                    .iter()
                    .map(|&(position, depth)| list_row(index, position, depth))
                    .collect(),
            })
            .collect()
    }

    /// Whether the note's row is the chosen one: the reader shows it, and nothing else holds
    /// the pane.
    pub fn row_selected(&self, id: Uuid) -> bool {
        self.view == View::Reader
            && self.repair.is_none()
            && self.new_kind.is_none()
            && !self.card_shown()
            && self.error.is_none()
            && self.selected == Some(id)
    }

    /// A press on a note's row, or on a link to it in the header's menu.
    pub fn press_note(&mut self, id: Uuid) {
        self.open_note(id);
    }

    /// The two rows of checks above the notes (Open questions, Things to check): each view,
    /// title, detail and whether it is open. A row with nothing left stays while its page is
    /// open, and until the notes are checked again.
    pub fn check_rows_shown(&mut self) -> Vec<(View, &'static str, String, bool)> {
        let Some((_, checks)) = &self.checks else {
            return Vec::new();
        };
        let rows = [
            (
                View::Questions,
                "Open questions",
                design::count(checks.questions.len(), "question", "questions"),
                checks.questions.len(),
            ),
            (
                View::Problems,
                PROBLEMS_TITLE,
                design::count(checks.problems(), "suggestion", "suggestions"),
                checks.problems(),
            ),
        ];
        let mut shown = Vec::new();
        for (view, title, detail, count) in rows {
            let open = self.view == view;
            if count == 0 && open && !self.kept_empty.contains(&view) {
                self.kept_empty.push(view);
            }
            let kept = self.kept_empty.contains(&view);
            let detail = match (count, view) {
                (0, View::Questions) => "No open questions".to_owned(),
                (0, _) => "No suggestions".to_owned(),
                _ => detail,
            };
            if count > 0 || kept {
                shown.push((view, title, detail, open));
            }
        }
        shown
    }

    /// A press on a check row: its page becomes the view.
    pub fn press_check_row(&mut self, view: View) {
        self.view = view;
        self.keep_repair();
        self.new_kind = None;
        self.leave_card();
        self.error = None;
    }

    /// The notes that cannot be read and match the search: path, row words and hover words, and
    /// whether its repair is open.
    pub fn broken_rows(&self, index: &ContentIndex) -> Vec<(String, String, String, bool)> {
        let search = self.search.to_lowercase();
        index
            .broken_world
            .iter()
            .filter(|(path, _)| path.to_lowercase().contains(&search))
            .map(|(path, problem)| {
                let detail = self.broken_details.get(path);
                let open = self.repair.as_ref().is_some_and(|r| r.path == *path)
                    || matches!(
                        &self.error,
                        Some(PageError::Unread(unread)) if unread.fault.path == *path
                    );
                (
                    path.clone(),
                    detail.map_or(problem.clone(), |detail| detail.cause.clone()),
                    detail.map_or(problem.clone(), |detail| detail.hover.clone()),
                    open,
                )
            })
            .collect()
    }

    /// Whether the repair of the note at `path`, or its unreadable pane, is open.
    pub fn repair_open(&self, path: &str) -> bool {
        self.repair.as_ref().is_some_and(|r| r.path == path)
            || matches!(
                &self.error,
                Some(PageError::Unread(unread)) if unread.fault.path == path
            )
    }

    /// The paths of repairs with unsaved changes whose note no longer needs one.
    pub fn draft_repairs(&self, index: &ContentIndex) -> Vec<String> {
        self.kept_repairs
            .iter()
            .filter(|r| r.dirty() && !index.broken_world.iter().any(|(path, _)| path == &r.path))
            .map(|r| r.path.clone())
            .collect()
    }

    /// What the page's larger part shows.
    pub fn pane(&self, index: &ContentIndex) -> Pane {
        if self.repair.is_some() {
            Pane::Repair
        } else if matches!(self.error, Some(PageError::Unread(..))) {
            Pane::Unread
        } else if self.new_kind.is_some() || self.card_shown() || self.error.is_some() {
            Pane::Creation
        } else if self.view != View::Reader {
            Pane::Checks(self.view)
        } else {
            match self.selected.and_then(|id| index.position(id)) {
                Some(at) => Pane::Note(at),
                None => Pane::Empty,
            }
        }
    }

    /// The held NPC's name, while its card is not the view: its row above the view.
    pub fn held_name(&self) -> Option<&str> {
        self.generated
            .as_ref()
            .filter(|_| !self.generated_shown)
            .map(|npc| npc.name.as_str())
    }

    /// `Show` on the held NPC's row.
    pub fn press_show_held(&mut self) {
        self.show_held();
    }

    /// A choice of New (`note`, `npc`, `location`, `generate`).
    pub fn press_new(&mut self, kind: &'static str, index: Option<&ContentIndex>) {
        self.begin_new(kind, index);
    }

    /// The creation form's kind, while one is open.
    pub fn new_kind(&self) -> Option<&'static str> {
        self.new_kind
    }

    /// The form's Name and Type words.
    pub fn new_fields(&self) -> (&str, &str) {
        (&self.new_name, &self.note_type)
    }

    pub fn set_new_name(&mut self, words: &str) {
        words.clone_into(&mut self.new_name);
    }

    pub fn set_note_type(&mut self, words: &str) {
        words.clone_into(&mut self.note_type);
    }

    /// The form's Cancel.
    pub fn cancel_new(&mut self) {
        self.new_kind = None;
    }

    /// Whether a run is open, so a form's commit is not the screen's primary.
    pub fn run_open(&self) -> bool {
        self.run_open
    }

    /// `Create note`, `Create NPC` or `Create location`.
    pub fn create(&mut self, campaign: &Campaign) {
        let Some(kind) = self.new_kind else {
            return;
        };
        if self.new_name.trim().is_empty() {
            return;
        }
        let kind = if kind == "note" {
            self.note_type.trim()
        } else {
            kind
        };
        let made = crate::world::create_entry(campaign, &self.new_name, kind);
        self.made(made);
    }

    /// A new note was made, or not: the made note opens, or the error shows.
    fn made(&mut self, made: Result<Uuid, String>) {
        match made {
            Ok(id) => {
                self.selected = Some(id);
                self.view = View::Reader;
                self.new_kind = None;
                // A New form's save leaves a held NPC in its row.
                if self.generated_shown {
                    self.marker_copied.owner_changed(&MarkerSlot);
                    self.generated = None;
                    self.generated_shown = false;
                }
                self.new_name.clear();
                self.note_type.clear();
                self.error = None;
                self.reload = true;
            }
            Err(error) => self.error = Some(error.into()),
        }
    }

    /// The held NPC's card, while it is the view.
    pub fn card(&self, index: &ContentIndex) -> Option<CardView> {
        let npc = self.generated.as_ref().filter(|_| self.generated_shown)?;
        Some(CardView {
            npc: npc.clone(),
            problems: self.generator_problems.clone(),
            sources: self.generator_sources.clone(),
            save_path: crate::world::new_entry_path(Some(index), &npc.name, "npc"),
            kept_in_lookup: self
                .lookup_npc
                .as_ref()
                .filter(|_| self.lookup_kept)
                .map(|name| kept_in_lookup(name, &npc.name)),
            marker_copied: self
                .marker_copied
                .get(&MarkerSlot)
                .is_some_and(|shown| shown.value().0 == npc.name),
        })
    }

    /// A press in the held NPC's card.
    pub fn card_act(&mut self, act: CardAct, campaign: &Campaign, index: &ContentIndex) {
        let Some(npc) = self.generated.as_ref().filter(|_| self.generated_shown) else {
            return;
        };
        match act {
            // Each press is a new result, ahead of the older one it replaces.
            CardAct::CopiedMarker => self.marker_copied.record(MarkerCopied(npc.name.clone())),
            CardAct::Save => {
                let made = crate::world::create_generated(campaign, npc);
                self.made(made);
            }
            CardAct::Again => self.roll(index),
            CardAct::Discard => {
                if let Some(npc) = self.generated.take() {
                    // The copy's result belonged to this card; `Undo discard` does not bring it
                    // back.
                    self.marker_copied.owner_changed(&MarkerSlot);
                    self.results
                        .record(PageKept(crate::world::PageResult::Discarded(
                            npc.name.clone(),
                        )));
                    self.discarded_npc = Some((
                        npc,
                        std::mem::take(&mut self.generator_problems),
                        std::mem::take(&mut self.generator_sources),
                    ));
                    self.generated_shown = false;
                    self.error = None;
                }
            }
        }
    }

    /// The page's error words, and whether they stand under a form (in its footer) or in place
    /// of the body (with Cancel).
    pub fn error_words(&self) -> Option<(&str, bool)> {
        match &self.error {
            Some(PageError::Words(words)) => {
                Some((words, self.new_kind.is_some() || self.card_shown()))
            }
            _ => None,
        }
    }

    /// The error's Cancel.
    pub fn cancel_error(&mut self) {
        self.error = None;
    }

    /// The unreadable note's pane. A later load that no longer lists the file ends it, and a
    /// fixed note opens; after Try again, a file still listed opens again.
    pub fn unread_view(&mut self, index: &ContentIndex, campaign: &Campaign) -> Option<UnreadView> {
        let Some(PageError::Unread(unread)) = &mut self.error else {
            return None;
        };
        let generation = index.generation();
        if unread.seen.is_some_and(|seen| seen != generation) {
            let listed = index
                .broken_world
                .iter()
                .any(|(path, _)| *path == unread.fault.path);
            if !listed || unread.again {
                let path = std::mem::take(&mut unread.fault.path);
                self.error = None;
                if listed {
                    self.open_repair(campaign, &path);
                } else if let Some(item) = index.items.iter().find(|item| item.entry.path == path) {
                    self.open_note(item.entry.id);
                }
                return None;
            }
        }
        unread.seen = Some(generation);
        let fault = &unread.fault;
        Some(UnreadView {
            path: fault.path.clone(),
            file: fault
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&fault.path)
                .to_owned(),
            words: format!(
                "{} {}",
                fault.pane_headline(),
                fault.recovery_with_try_again()
            ),
            system: fault.system.clone(),
        })
    }

    /// The unreadable pane's `Try again`: the folder is read again first.
    pub fn unread_again(&mut self) {
        if let Some(PageError::Unread(unread)) = &mut self.error {
            unread.again = true;
            self.reload = true;
        }
    }

    /// The open repair, for its view.
    pub fn repair_mut(&mut self) -> Option<&mut crate::world::repair::Repair> {
        self.repair.as_mut()
    }

    /// The open repair was saved: the repaired note opens with `Note repaired.`.
    pub fn repair_saved(&mut self, campaign: &Campaign) {
        let Some(repair) = self.repair.take() else {
            return;
        };
        self.view = View::Reader;
        self.selected = repair.entry_id();
        self.repaired = self.selected;
        self.results
            .record(PageKept(crate::world::PageResult::Repaired));
        // The repair is the newer result. A change result kept from before the note broke has
        // nothing left to undo, and would hide `Note repaired.`. Another note's rename keeps its
        // Undo.
        self.reader.clear_notice();
        self.reader.end_name_change_on(campaign, &repair.path);
        self.reload = true;
    }

    /// A press on a repair row (the list's `Cannot be read` and `Repair drafts`).
    pub fn press_repair(&mut self, campaign: &Campaign, path: &str) {
        self.open_repair(campaign, path);
    }

    /// The checks of the notes, for the checks view.
    pub fn checks(&self) -> Option<&crate::world::checks::Checks> {
        self.checks.as_ref().map(|(_, checks)| checks)
    }

    /// A press in a checks view: a page opens at its heading, or a question is ticked.
    pub fn press_open(&mut self, index: &ContentIndex, page: usize, heading: Option<String>) {
        self.open_note(index.items[page].entry.id);
        self.reader.show_heading(heading);
    }

    /// Tick the question at this place in Open questions.
    pub fn press_tick(&mut self, index: &ContentIndex, campaign: &Campaign, place: usize) {
        self.tick_question(index, campaign, place);
    }

    /// The footer of the open pane, as plain lines: the reader's for the note at `item`, the
    /// page's result, the ticks of Open questions, then the trackers' history (`more`).
    pub fn footer_plain(
        &mut self,
        item: Option<&crate::lookup::Item>,
        index: &ContentIndex,
        campaign: &Campaign,
        more: impl for<'f> FnOnce(&'f PaneFrame, &mut Messages<'f, crate::world::FooterAct>),
    ) -> Vec<design::outcome::plain::Line<crate::world::FooterAct>> {
        let frame = design::outcome::plain::frame();
        let mut messages = Messages::new(&frame, "plain", 0.0);
        match (item, self.view) {
            (_, View::Questions) => self.questions_messages(&frame, &mut messages, campaign),
            (Some(item), _) => {
                self.reader
                    .footer(&frame, &mut messages, item, index, Some(campaign));
                self.page_messages(&frame, &mut messages, Some(item.entry.id));
            }
            (None, _) => {
                self.page_messages(&frame, &mut messages, None);
                self.reader
                    .gone_log_messages(&frame, &mut messages, Some(campaign.id));
            }
        }
        more(&frame, &mut messages);
        messages.into_plain()
    }

    /// The creation pane's footer: Lookup's kept NPC, the marker's result and the error of the
    /// last Create or Save.
    pub fn creation_footer(&self) -> Vec<design::outcome::plain::Line<crate::world::FooterAct>> {
        let frame = design::outcome::plain::frame();
        let mut messages: Messages<'_, crate::world::FooterAct> =
            Messages::new(&frame, "plain", 0.0);
        let form = self.new_kind.is_some() || self.card_shown();
        if let Some(npc) = self.generated.as_ref().filter(|_| self.generated_shown) {
            if let Some(name) = self.lookup_npc.as_ref().filter(|_| self.lookup_kept) {
                messages.outcome(Notice::new(crate::world::said(&kept_in_lookup(
                    name, &npc.name,
                ))));
            }
            if let Some(shown) = self
                .marker_copied
                .get(&MarkerSlot)
                .filter(|shown| shown.value().0 == npc.name)
            {
                messages.kept(shown.outcome(&frame, |_| {
                    Said::new(crate::sentence!(
                        "Copied. Change 1d6 to the number of names."
                    ))
                    .into()
                }));
            }
        }
        if let Some(PageError::Words(error)) = &self.error
            && form
        {
            messages.outcome(FaultMessage::new(&crate::world::ReaderFault::Action(
                error.clone(),
            )));
        }
        messages.into_plain()
    }

    /// A footer press in the page's panes: the page's result and ticks are taken here, the
    /// trackers' by `trackers`, the reader's by the reader.
    pub fn footer_press_plain(
        &mut self,
        press: design::outcome::Press<crate::world::FooterAct>,
        index: &ContentIndex,
        campaign: &Campaign,
        trackers: &mut dyn crate::world::TrackerUi,
    ) -> Option<crate::world::model::Effect> {
        let open = self.selected.filter(|_| self.view == View::Reader);
        match press.id {
            crate::world::FooterAct::Page => {
                if let Some(ticket) = press.ticket
                    && self
                        .results
                        .undo_by(ticket, |kept| match kept.0 {
                            crate::world::PageResult::Discarded(_) => Ok(()),
                            crate::world::PageResult::Repaired => Err(()),
                        })
                        .is_ok()
                {
                    self.undo_discard();
                }
                None
            }
            crate::world::FooterAct::Trackers => {
                trackers.undo_remove(press.ticket);
                None
            }
            crate::world::FooterAct::Untick => {
                if let Some(ticket) = press.ticket {
                    self.untick(index, campaign, ticket);
                }
                None
            }
            _ => {
                let (nav, effect) = self.reader.footer_press(press, open, campaign);
                if let Some(nav) = nav {
                    self.take_nav(nav, index);
                }
                effect
            }
        }
    }

    /// What a reader's press asks of the page: another note, Lookup for a missing one, or a
    /// new load of the notes.
    pub fn take_nav(&mut self, nav: Nav, index: &ContentIndex) {
        match nav {
            Nav::Item(position) => {
                self.selected = Some(index.items[position].entry.id);
                self.view = View::Reader;
            }
            Nav::Missing(words) => self.ask_lookup = Some(words),
            Nav::Reload(id) => {
                self.reload = true;
                self.selected = Some(id);
            }
        }
    }

    /// The selected note, for a check.
    pub fn selected(&self) -> Option<Uuid> {
        self.selected
    }

    /// The view, for a check.
    pub fn view(&self) -> View {
        self.view
    }
}

/// One row of the list: the note's name or file, its fault, dates or status.
fn list_row(index: &ContentIndex, position: usize, depth: usize) -> ListRow {
    let entry = &index.items[position].entry;
    let duplicate = index.duplicate_world.contains_key(&entry.path);
    let file = || {
        std::path::Path::new(&entry.path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    };
    let name = if duplicate {
        file()
    } else {
        entry.name.clone()
    };
    let fault = index.note_faults().of_note(position);
    let narrow = fault
        .and_then(crate::world::NoteFault::narrow_words)
        .map(|words| format!("{} {words}", icons::WARN));
    let detail = if let Some(fault) = fault {
        format!("{} {}", icons::WARN, fault.words())
    } else if entry.kind == "world" {
        if index.shares_name(position) {
            match crate::world::status(entry) {
                "" => file(),
                status => format!("{} · {status}", file()),
            }
        } else {
            [
                crate::world::date_words(entry),
                Some(crate::world::status(entry).to_owned()),
            ]
            .into_iter()
            .flatten()
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>()
            .join(" · ")
        }
    } else {
        String::new()
    };
    ListRow {
        position,
        id: entry.id,
        name,
        detail,
        narrow,
        depth,
        path: entry.path.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::{
        Harness,
        kittest::{NodeT, Queryable},
    };

    fn note(path: &str, name: &str, kind: &str, body: &str) -> String {
        format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: {name}\nsplat_type: {kind}\n---\n\n# {name}\n\n{body}",
            splat::character::stable_id(path)
        )
    }

    /// A campaign with an open question, a problem and a note that cannot be read.
    fn held_npc_campaign(dir: &std::path::Path) -> (Campaign, Arc<ContentIndex>) {
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Held".into(),
            path: dir.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        for (path, text) in [
            (
                "NPCs/Rook.md",
                note(
                    "NPCs/Rook.md",
                    "Rook",
                    "npc",
                    "Rook keeps the gate.\n\n- [ ] Who pays Rook?\n",
                ),
            ),
            ("Lore/Bells.md", note("Lore/Bells.md", "Bells", "lore", "")),
            (
                "Broken.md",
                "---\nsplat_schema: 2\nsplat_id: [\n---\n".into(),
            ),
        ] {
            let file = dir.join(path);
            std::fs::create_dir_all(file.parent().unwrap()).unwrap();
            std::fs::write(file, text).unwrap();
        }
        let index = Arc::new(ContentIndex::load(&[], Some(&campaign)).unwrap());
        (campaign, index)
    }

    /// A campaign change keeps typed Log words, by their campaign: the switch guard counts
    /// them, and a reader of another campaign never shows them.
    #[test]
    fn a_campaign_change_keeps_typed_log_words() {
        let mut page = WorldPage::default();
        let first = Uuid::new_v4();
        page.set_campaign(first);
        let id = Uuid::new_v4();
        page.reader.open_log(first, id, "Hob", "Words I typed.");
        page.set_campaign(Uuid::new_v4());
        assert!(page.reader.has_unsaved_edits());
        assert_eq!(page.reader.unsaved_edits(), vec![(id, "Hob".to_owned())]);
    }

    fn held(page: &WorldPage) -> Option<String> {
        page.generated.as_ref().map(|npc| npc.name.clone())
    }

    fn npc_files(dir: &std::path::Path) -> Vec<String> {
        let mut files: Vec<_> = std::fs::read_dir(dir.join("NPCs"))
            .unwrap()
            .map(|file| file.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|file| file != "Rook.md")
            .collect();
        files.sort();
        files
    }

    /// AD1/SD1/FD1: an unsaved generated NPC is a held draft. Every path to another view keeps
    /// it in one row above that view, Show brings its card back, and Save writes it once.
    /// Each path's failure is collected, so one fault build names every path it breaks.
    #[test]
    fn iter15_d_ad1_every_path_keeps_a_held_npc() {
        use crate::design::test_harness::Themed as _;
        let dir = tempfile::tempdir().unwrap();
        let (campaign, index) = held_npc_campaign(dir.path());
        assert_eq!(index.broken_world.len(), 1, "the fixture has a repair");
        let rook = index
            .items
            .iter()
            .find(|item| item.entry.name == "Rook")
            .unwrap()
            .entry
            .id;
        let shown = campaign.clone();
        let mut harness = Harness::builder()
            .with_size(egui::vec2(1200.0, 800.0))
            .themed_ui_state(
                {
                    let index = index.clone();
                    move |ui, page: &mut WorldPage| {
                        page.show(
                            ui,
                            Some(&index),
                            &shown,
                            &mut crate::tracker_reader::Reader(&mut Default::default()),
                        )
                    }
                },
                WorldPage::default(),
            );
        harness.run_steps(2);
        harness.state_mut().begin_new("generate", Some(&index));
        harness.run_steps(2);
        let npc = held(harness.state()).expect("the generator rolled an NPC");
        let rolled = harness.state().generated.clone().unwrap();
        // M0602: the row is the draft status, then the name, which the row's sentence names
        // (the name's label; kittest's label queries read a text's value, its name alone).
        let row = format!("Unsaved NPC: {npc}");
        fn row_shown(harness: &Harness<'_, WorldPage>, npc: &str, row: &str) -> bool {
            harness
                .query_all_by_label_contains(npc)
                .any(|node| node.accesskit_node().label().as_deref() == Some(row))
        }
        assert!(harness.query_by_label("Save NPC").is_some());
        assert!(
            !row_shown(&harness, &npc, &row),
            "no row while the card shows"
        );

        type Path<'a> = (
            &'a str,
            Box<dyn Fn(&mut Harness<'_, WorldPage>) -> bool + 'a>,
        );
        let label_starting = |harness: &Harness<'_, WorldPage>, start: &str| {
            harness
                .query_all_by_label_contains(start)
                .find(|node| {
                    node.accesskit_node()
                        .label()
                        .is_some_and(|label| label.starts_with(start))
                })
                .map(|node| node.accesskit_node().label().unwrap())
        };
        let click_starting = move |harness: &mut Harness<'_, WorldPage>, start: &str| {
            let label = label_starting(harness, start).unwrap_or_else(|| panic!("{start}"));
            harness.get_by_label(&label).click();
            harness.run_steps(2);
        };
        let paths: Vec<Path> = vec![
            (
                "AD1-problems",
                Box::new(|harness| {
                    click_starting(harness, "Things to check · ");
                    harness.state().view == View::Problems
                        && harness.query_by_label(PROBLEMS_INTRO).is_some()
                }),
            ),
            (
                "AD1-questions",
                Box::new(|harness| {
                    click_starting(harness, "Open questions · ");
                    harness.state().view == View::Questions
                        && harness.query_by_label("Who pays Rook?").is_some()
                }),
            ),
            (
                "AD1-reveal",
                Box::new(|harness| {
                    let id = campaign.id;
                    harness.state_mut().reveal(id, rook);
                    harness.run_steps(2);
                    harness.state().selected == Some(rook)
                        && harness.query_by_label("Rook keeps the gate.").is_some()
                }),
            ),
            (
                "AD1-repair-opens",
                Box::new(|harness| {
                    harness.state_mut().open_repair(&campaign, "Broken.md");
                    harness.run_steps(2);
                    harness.state().repair.is_some()
                }),
            ),
            (
                "AD1-repair-ends-on-another-note",
                Box::new(|harness| {
                    harness.state_mut().open_repair(&campaign, "Broken.md");
                    harness.run_steps(2);
                    let opened = harness.state().repair.is_some();
                    click_starting(harness, "Bells · ");
                    opened
                        && harness.state().repair.is_none()
                        && harness.state().selected
                            == index
                                .items
                                .iter()
                                .find(|item| item.entry.name == "Bells")
                                .map(|item| item.entry.id)
                }),
            ),
            (
                "AD1-new-note",
                Box::new(|harness| {
                    harness.state_mut().begin_new("note", Some(&index));
                    harness.run_steps(2);
                    let form = harness.query_by_label("Create note").is_some();
                    harness.get_by_label("Cancel").click();
                    harness.run_steps(2);
                    form && harness.state().new_kind.is_none()
                }),
            ),
            (
                "AD1-new-npc-saved",
                Box::new(|harness| {
                    harness.state_mut().begin_new("npc", Some(&index));
                    harness.run_steps(2);
                    let form = harness.query_by_label("Create NPC").is_some();
                    harness
                        .get_by_role_and_label(egui::accesskit::Role::TextInput, "Name")
                        .focus();
                    harness.run_steps(1);
                    harness
                        .get_by_role_and_label(egui::accesskit::Role::TextInput, "Name")
                        .type_text("Tam");
                    harness.run_steps(2);
                    harness.get_by_label("Create NPC").click();
                    harness.run_steps(2);
                    // The New form's save writes only its own note.
                    form && harness.state().new_kind.is_none()
                        && npc_files(dir.path()) == ["Tam.md"]
                }),
            ),
        ];
        let mut failed = Vec::new();
        for (id, path) in &paths {
            let reached = path(&mut harness);
            let kept = held(harness.state()).as_deref() == Some(npc.as_str());
            let shown =
                row_shown(&harness, &npc, &row) && harness.query_by_label("Save NPC").is_none();
            if !(reached && kept && shown) {
                failed.push(format!("{id}: reached {reached}, kept {kept}, row {shown}"));
                // Hold the same NPC again so the next path starts from the same state.
                let page = harness.state_mut();
                page.generated = Some(rolled.clone());
                page.generated_shown = true;
                harness.run_steps(2);
                continue;
            }
            harness.get_by_label("Show").click();
            harness.run_steps(2);
            if harness.query_by_label("Save NPC").is_none() || row_shown(&harness, &npc, &row) {
                failed.push(format!("{id}: Show did not bring the card back"));
            }
        }
        // Lookup's link and Generate NPC open the held card, never a new NPC.
        for (id, open) in [
            (
                "AD1-lookup-link",
                WorldPage::lookup_kept as fn(&mut WorldPage),
            ),
            ("AD1-generate-again", |page: &mut WorldPage| {
                page.begin_new("generate", None)
            }),
        ] {
            click_starting(&mut harness, "Things to check · ");
            open(harness.state_mut());
            harness.run_steps(2);
            if held(harness.state()).as_deref() != Some(npc.as_str())
                || harness.query_by_label("Save NPC").is_none()
            {
                failed.push(format!("{id}: the held card is not shown"));
            }
        }
        assert!(failed.is_empty(), "{failed:#?}");

        harness.get_by_label("Save NPC").click();
        harness.run_steps(2);
        let mut expected = vec![format!("{npc}.md"), "Tam.md".to_owned()];
        expected.sort();
        assert_eq!(
            npc_files(dir.path()),
            expected,
            "AD1-saved: the held NPC saves to one file, beside the New form's note"
        );
        assert!(held(harness.state()).is_none());
        assert!(!row_shown(&harness, &npc, &row));
    }

    /// Discard lets the NPC go with `Undo discard`, which brings the same NPC back. Moving on
    /// ends the Undo, and a new NPC replaces it.
    #[test]
    fn iter15_d_ad1_discard_has_undo() {
        use crate::design::test_harness::Themed as _;
        let dir = tempfile::tempdir().unwrap();
        let (campaign, index) = held_npc_campaign(dir.path());
        let mut harness = Harness::builder()
            .with_size(egui::vec2(1200.0, 800.0))
            .themed_ui_state(
                {
                    let index = index.clone();
                    move |ui, page: &mut WorldPage| {
                        page.show(
                            ui,
                            Some(&index),
                            &campaign,
                            &mut crate::tracker_reader::Reader(&mut Default::default()),
                        )
                    }
                },
                WorldPage::default(),
            );
        harness.run_steps(2);
        harness.state_mut().begin_new("generate", Some(&index));
        harness.run_steps(2);
        let npc = held(harness.state()).unwrap();
        let attributes = harness.state().generated.clone().unwrap().attributes;
        let notice = format!("{npc} draft discarded.");
        // The message and its footer both carry these words (round 26), so count either.
        harness.get_by_label("Discard").click();
        harness.run_steps(2);
        assert!(held(harness.state()).is_none(), "Discard lets the NPC go");
        assert!(harness.query_by_label("Save NPC").is_none());
        assert!(
            harness.query_all_by_label(&notice).next().is_some(),
            "AD1-discard-notice"
        );
        harness
            .get_by_label(&format!("Undo discard of {npc}"))
            .click();
        harness.run_steps(2);
        assert_eq!(
            harness
                .state()
                .generated
                .as_ref()
                .map(|n| (&n.name, &n.attributes)),
            Some((&npc, &attributes)),
            "AD1-undo-discard: the same NPC comes back"
        );
        assert!(harness.query_by_label("Save NPC").is_some());
        assert!(harness.query_all_by_label(&notice).next().is_none());

        // Moving on ends the Undo. The NPC is then gone and nothing was written.
        harness.get_by_label("Discard").click();
        harness.run_steps(2);
        assert!(harness.query_all_by_label(&notice).next().is_some());
        let problems = harness
            .query_all_by_label_contains("Things to check · ")
            .next()
            .unwrap()
            .accesskit_node()
            .label()
            .unwrap();
        harness.get_by_label(&problems).click();
        harness.run_steps(2);
        assert!(
            harness.query_all_by_label(&notice).next().is_none()
                && harness.state().discarded_npc.is_none(),
            "AD1-undo-ends: another view ends the Undo"
        );
        assert!(npc_files(dir.path()).is_empty(), "Discard writes nothing");

        // A new NPC replaces a waiting Undo, so Undo never replaces an unsaved NPC.
        harness.state_mut().begin_new("generate", Some(&index));
        harness.run_steps(2);
        harness.get_by_label("Discard").click();
        harness.run_steps(2);
        harness.state_mut().begin_new("generate", Some(&index));
        harness.run_steps(2);
        assert!(harness.state().discarded_npc.is_none());
        assert!(
            harness
                .query_all_by_label_contains("Undo discard")
                .next()
                .is_none()
        );
    }

    #[test]
    fn iter18_d_an_unreadable_row_states_a_short_cause_not_the_system_words() {
        let mut index = ContentIndex::load(&[], None).unwrap();
        index.broken_world.push((
            "Notes/Broken.md".into(),
            "stream did not contain valid UTF-8".into(),
        ));
        let mut page = WorldPage::default();
        page.refresh_checks(&index, |path| {
            Some(Err(FileFault::read(
                path,
                &std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "stream did not contain valid UTF-8",
                ),
            )))
        });
        let detail = &page.broken_details["Notes/Broken.md"];
        assert_eq!(detail.cause, "Not a text file", "AD2-row-cause");
        assert_eq!(
            detail.hover, "Splat cannot read Notes/Broken.md as text.",
            "AD2-row-hover"
        );
    }

    #[test]
    fn repair_details_read_once_per_index_and_follow_recovery() {
        let mut first = ContentIndex::load(&[], None).unwrap();
        first
            .broken_world
            .push(("Broken.md".into(), "Original parser error".into()));
        let mut page = WorldPage::default();
        let mut reads = 0;
        let mut read = |_: &str| {
            reads += 1;
            Some(Ok("---\nsplat_schema: 2\nsplat_id: [\n---\n".into()))
        };
        page.refresh_checks(&first, &mut read);
        page.refresh_checks(&first, &mut read);
        assert_eq!(
            reads, 1,
            "FD9-cache: the unchanged second render does no file read or repair parse"
        );
        assert!(
            page.broken_details["Broken.md"]
                .cause
                .contains("Line 3 opens a list")
        );

        // A changed index re-reads the note. Failure to read it uses its current indexed error.
        let mut unreadable = ContentIndex::load(&[], None).unwrap();
        unreadable
            .broken_world
            .push(("Broken.md".into(), "The file cannot be opened".into()));
        page.refresh_checks(&unreadable, |_| None);
        assert_eq!(
            page.broken_details["Broken.md"].cause,
            "The file cannot be opened"
        );

        // A move, rename, deletion or repair removes the old row on the next index.
        let recovered = ContentIndex::load(&[], None).unwrap();
        page.refresh_checks(&recovered, |_| {
            panic!("A recovered index needs no repair reads")
        });
        assert!(
            page.broken_details.is_empty(),
            "FD9-recovery: no stale repair cause survives"
        );

        for body in [
            "| Field | Value |\n| --- | --- |\n| Name |\n",
            "| Field | Value |\n| --- | --- |\n| Name | Rook | extra |\n",
            "",
        ] {
            let mut index = ContentIndex::load(&[], None).unwrap();
            index
                .broken_world
                .push(("Broken.md".into(), "Indexed cause".into()));
            page.refresh_checks(&index, |_| Some(Ok(format!("---\nsplat_schema: 2\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Rook\nsplat_type: lore\n---\n{body}"))));
            assert_eq!(
                page.broken_details["Broken.md"].cause, "Indexed cause",
                "FD9-input: readable body syntax does not invent a front matter cause"
            );
        }
    }
}
