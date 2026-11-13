//! The World page: what is true in this world. One list of notes and entries, and the Reader.
use super::{Nav, Reader, file_fault::FileFault};
use crate::{
    design::{self, Type, colors, icons},
    lookup::{ContentIndex, WorldGroups},
    persistence::Campaign,
};
use eframe::egui;
use std::sync::Arc;
use uuid::Uuid;

const ROW: f32 = 40.0;
const LIST_WIDTH: f32 = 260.0;
/// More rows than this in one check are counted, not shown.
const CHECK_ROWS: usize = 40;
/// The line above the World card while Lookup holds a second unsaved NPC (round 26 FB8).
pub(crate) fn kept_in_lookup(lookup: &str, card: &str) -> String {
    format!("{lookup} is not saved. It is in Lookup. Save or discard {card} to show {lookup} here.")
}

/// The advice page's row and title: what to check, not what is broken (round 26 FF3, FH1).
pub(crate) const PROBLEMS_TITLE: &str = "Things to check";

/// What the Things to check page lists, in the words of its sections.
const PROBLEMS_INTRO: &str = "These lists show notes with a date outside an event, a link to no note, no summary, no links, or a name or sentence on two notes. Splat only lists them. You change the notes.";
/// The words for a held NPC, in the rail and in the row above another view. The rail's link
/// is found by this constant.
pub const UNSAVED_NPC: &str = "Unsaved NPC";

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
fn marked_row(ui: &mut egui::Ui, selected: bool, title: &str, detail: &str) -> egui::Response {
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

/// A question that the DM ticked in Open questions, kept for its `Undo tick` by the note's ID
/// and the line that holds its box.
struct Tick {
    campaign: Uuid,
    note: Uuid,
    /// The body line that holds the box, and that line before the tick.
    at: usize,
    line: String,
    /// The result's words: the question and its note.
    words: String,
}

/// What the DM did in a checks view: open a page at a heading, or tick the question at this
/// place in `Checks::questions`.
enum CheckAct {
    Open(usize, Option<String>),
    Tick(usize),
}

/// What the page's larger part shows.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum View {
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
    repair: Option<super::repair::Repair>,
    kept_repairs: Vec<super::repair::Repair>,
    campaign: Option<Uuid>,
    /// An unsaved generated NPC is a held draft. Only Save NPC and Discard end it.
    generated: Option<crate::npc_generator::Generated>,
    /// Whether the held NPC's card is the view. If not, one row above the view names it.
    generated_shown: bool,
    /// The NPC that Discard let go, with its table facts, for one `Undo discard`.
    discarded_npc: Option<Held>,
    /// When the page's result (`Note repaired.` or the NPC's discard) was made.
    result_order: crate::design::ResultOrder,
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
    checks: Option<(u64, super::checks::Checks)>,
    /// Repair causes are read and parsed only when the index generation changes.
    broken_details: std::collections::BTreeMap<String, BrokenDetail>,
    /// Ticked questions with a live `Undo tick`, oldest first.
    ticks: Vec<Tick>,
    /// Why the last tick, or its Undo, wrote nothing.
    tick_error: Option<String>,
    /// The held NPC whose card's `Copy the marker` was pressed, by name, with the order the
    /// press issued: the footer says `MARKER_COPIED` while that card is the view. Another NPC on
    /// the card (a roll, Lookup's card), its save or discard, and a campaign change end it.
    marker_copied: Option<(String, crate::design::ResultOrder)>,
}

/// One unreadable note's row words.
struct BrokenDetail {
    /// The row's second line.
    cause: String,
    /// The whole sentence, on hover.
    hover: String,
}

/// The one row above another view while an unsaved generated NPC is held, on World and in
/// Lookup: `Unsaved NPC: <name> · Show`. Returns whether the DM chose `Show`.
pub(crate) fn unsaved_npc_row(ui: &mut egui::Ui, name: &str) -> bool {
    let mut show = false;
    ui.horizontal_wrapped(|ui| {
        ui.label(
            Type::Body
                .rich(ui, format!("{UNSAVED_NPC}: {name}"))
                .color(colors().text),
        );
        ui.label(Type::Body.rich(ui, "·").color(colors().muted));
        show = design::link(ui, "Show").clicked();
    });
    ui.add_space(design::S);
    show
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
        self.checks = Some((index.generation(), super::checks::run(index)));
        // A tick's Undo lives while its note holds the ticked line where the tick wrote it.
        self.ticks.retain(|tick| {
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
                            .and_then(|text| crate::campaign_content::repair_problem(path, &text).0)
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
            .is_some_and(super::repair::Repair::dirty)
            || self.kept_repairs.iter().any(super::repair::Repair::dirty)
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
            self.marker_copied = None;
            self.ticks.clear();
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
        self.marker_copied = None;
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
            super::repair::Repair::open(campaign, path)
        };
        match loaded {
            Ok(repair) => {
                self.repair = Some(repair);
                self.error = None;
                self.new_kind = None;
                self.leave_card();
            }
            Err(super::repair::OpenError::Words(error)) => self.error = Some(error.into()),
            Err(super::repair::OpenError::Unread(fault)) => {
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
        trackers: &mut crate::trackers::Trackers,
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
        let count = index.map_or(0, |index| {
            index.world_groups("").values().map(Vec::len).sum::<usize>() + broken
        });
        let broken_links = index.map_or(0, |index| {
            index
                .world_groups("")
                .values()
                .flatten()
                .filter(|(at, _)| {
                    !super::broken_links(&index.items[*at], index).is_empty()
                        || index
                            .duplicate_world
                            .contains_key(&index.items[*at].entry.path)
                        || !index.items[*at].entry.flags.is_empty()
                        || crate::roll_table::fault_count(&index.items[*at].entry.body) > 0
                })
                .count()
        });
        let mut create = None;
        design::page_header(
            ui,
            "World",
            (count > 0)
                .then(|| design::count(count, "note", "notes"))
                .as_deref(),
            |ui| {
                create = design::locked(
                    ui,
                    index
                        .is_none()
                        .then_some("Wait for the notes to load before creating a note."),
                    new_menu,
                );
                if broken_links > 0 {
                    let response = design::status_link(
                        ui,
                        design::Status {
                            icon: icons::WARN,
                            word: &design::count(
                                broken_links,
                                "note needs fixing",
                                "notes need fixing",
                            ),
                            color: colors().result.warning,
                        },
                    );
                    egui::Popup::menu(&response).show(|ui| {
                        if let Some(index) = index {
                            for &(at, _) in index.world_groups("").values().flatten() {
                                let item = &index.items[at];
                                if (!super::broken_links(item, index).is_empty()
                                    || index.duplicate_world.contains_key(&item.entry.path)
                                    || !item.entry.flags.is_empty()
                                    || crate::roll_table::fault_count(&item.entry.body) > 0)
                                    && design::link(ui, &item.entry.name).clicked()
                                {
                                    self.open_note(item.entry.id);
                                    ui.close();
                                }
                            }
                        }
                    });
                }
                // Every note this count holds cannot be read, which is an error in its row,
                // this count and its repair alike.
                if broken > 0
                    && design::status_link(
                        ui,
                        design::Status {
                            icon: icons::ERROR,
                            word: &format!(
                                "{} cannot be read",
                                design::count(broken, "note", "notes")
                            ),
                            color: colors().result.error,
                        },
                    )
                    .clicked()
                    && let Some((path, _)) = index.and_then(|index| index.broken_world.first())
                {
                    self.search.clear();
                    self.open_repair(campaign, path);
                }
            },
        );
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
        design::Column::left("world_list_panel", LIST_WIDTH, 190.0..=480.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                let width = ui.available_width();
                design::search_field(
                    ui,
                    "Search the world",
                    &mut self.search,
                    "Find a note",
                    width,
                );
                egui::ScrollArea::vertical()
                    .id_salt("world_list")
                    .auto_shrink([false, false])
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
                            ui.add_space(design::XS);
                            design::group_heading(ui, "Cannot be read", broken.len());
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
                                r.dirty()
                                    && !index.broken_world.iter().any(|(path, _)| path == &r.path)
                            })
                            .map(|r| r.path.clone())
                            .collect();
                        if !drafts.is_empty() {
                            design::group_heading(ui, "Repair drafts", drafts.len());
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
                        self.view = View::Reader;
                        self.selected = repair.entry_id();
                        self.repaired = self.selected;
                        self.result_order = crate::design::ResultOrder::issue();
                        // The repair is the newer result. A change result kept from before the
                        // note broke has nothing left to undo, and would hide `Note repaired.`.
                        // Another note's rename keeps its Undo.
                        self.reader.clear_notice();
                        if self
                            .reader
                            .name_undo
                            .as_ref()
                            .is_some_and(|change| change.shows_on(campaign, &repair.path))
                        {
                            self.reader.name_undo = None;
                        }
                        self.repair = None;
                        self.reload = true;
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
                    let bottom = if self.view == View::Questions {
                        self.questions_footer(ui, index, campaign)
                    } else {
                        ui.max_rect().bottom()
                    };
                    let act = egui::ScrollArea::vertical()
                        .id_salt(("world_checks", self.view == View::Questions))
                        .max_height((bottom - ui.cursor().top()).max(0.0))
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            design::reader::measure(ui, |ui| self.checks_view(ui, index))
                        })
                        .inner;
                    match act {
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
                    let mut lines: Vec<_> =
                        self.result(None).iter().map(|(r, _)| r.line()).collect();
                    // A Log line whose entry is gone says its words here too.
                    lines.extend(self.reader.gone_log_notices(Some(campaign.id)));
                    match super::draw_footer(ui, "world_footer", &lines).0 {
                        Some(super::FooterAct::GoneLog(id)) => self.reader.end_gone_log(id),
                        Some(_) => self.undo_discard(),
                        None => {}
                    }
                    design::empty_state(
                        ui,
                        icons::WORLD,
                        "No note open",
                        "Choose one from the list, or make a new one.",
                        |_| {},
                    );
                    return;
                };
                // A later result on the repaired note replaces `Note repaired.`.
                if self.repaired == Some(item.entry.id)
                    && self.reader.has_change_result(campaign, &item.entry.path)
                {
                    self.repaired = None;
                }
                // Every result, reason and Undo of the open note sits in the pane's footer,
                // pinned below the reader, so none of them moves the note (round 26).
                let page = self.result(Some(item.entry.id));
                let lines = super::with_lines(
                    self.reader.footer_lines(
                        item,
                        index,
                        Some(campaign),
                        page.as_ref().map(|(page, order)| (page, *order)),
                    ),
                    crate::trackers::history::footer_lines(trackers, item.entry.id),
                );
                let (pressed, bottom) = super::draw_footer(ui, "world_footer", &lines);
                match pressed {
                    Some(super::FooterAct::Page) => self.undo_discard(),
                    Some(super::FooterAct::Trackers) => trackers.undo_remove(),
                    Some(act) => nav = self.reader.footer_act(ui.ctx(), act, item, campaign),
                    None => {}
                }
                // As in Library, a wide table may use the pane; prose keeps its measure.
                crate::markdown_view::with_wide_tables(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt(("world_reader", item.entry.id))
                        // The reader ends `S` above the footer, so the footer never covers it.
                        .max_height((bottom - ui.cursor().top()).max(0.0))
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let read = self.reader.read_with(
                                ui,
                                item,
                                index,
                                Some(campaign),
                                Some(&mut *trackers),
                            );
                            nav = nav.take().or(read);
                        });
                });
            });
        if let Some(kind) = create.take() {
            self.begin_new(kind, Some(index));
        }
        match nav {
            Some(Nav::Item(position)) => {
                self.selected = Some(index.items[position].entry.id);
                self.view = View::Reader;
            }
            Some(Nav::Missing(words)) => self.ask_lookup = Some(words),
            Some(Nav::Reload(id)) => {
                self.reload = true;
                self.selected = Some(id);
            }
            None => {}
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
                self.marker_copied = None;
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
            && unsaved_npc_row(ui, &npc.name)
        {
            self.show_held();
        }
    }

    /// The result this page shows in the footer: the NPC Discard let go, or the repair that made
    /// the open note readable, with when it was made.
    fn result(
        &self,
        open: Option<Uuid>,
    ) -> Option<(super::PageResult, crate::design::ResultOrder)> {
        if let Some((npc, _, _)) = self.discarded_npc.as_ref()
            && self.generated.is_none()
        {
            Some((
                super::PageResult::Discarded(npc.name.clone()),
                self.result_order,
            ))
        } else if open.is_some() && self.repaired == open {
            Some((super::PageResult::Repaired, self.result_order))
        } else {
            None
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
    fn unread_pane(&mut self, ui: &mut egui::Ui, index: &ContentIndex, campaign: &Campaign) {
        let Some(PageError::Unread(unread)) = &mut self.error else {
            return;
        };
        // A later load that no longer lists the file (fixed, moved or deleted outside Splat)
        // ends the pane, and a fixed note opens in the reader. After Try again, a file the load
        // still lists opens again: in the source editor, or here with its new fault.
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
                return;
            }
        }
        unread.seen = Some(generation);
        let fault = unread.fault.clone();
        design::reader::title(ui, fault.path.rsplit('/').next().unwrap_or(&fault.path));
        // No editor link: an editor cannot open a file that Splat cannot read either.
        super::entry_file_actions(ui, &campaign.path, &fault.path, false);
        // The title and file line name the file, so the error says `this note` (FG6).
        let (again, cancel) =
            design::message_with_actions(ui, design::Tone::Error, &fault.pane_headline(), |ui| {
                // The recovery is prose: it keeps the reader measure inside the message's
                // list measure, so its last step does not wrap across a long return.
                ui.scope(|ui| {
                    ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
                    ui.add(
                        egui::Label::new(
                            Type::Body
                                .rich(ui, fault.recovery_with_try_again())
                                .color(colors().text),
                        )
                        .wrap(),
                    );
                });
                ui.horizontal_wrapped(|ui| {
                    // Message actions without an icon have the secondary border, which starts
                    // at the message text. `L` between them: the layout's own gap is part of it.
                    let again = design::Action::secondary("Try again")
                        .density(design::Density::Read)
                        .show(ui)
                        .clicked();
                    ui.add_space(design::L - ui.spacing().item_spacing.x);
                    let cancel = design::Action::secondary("Close note")
                        .density(design::Density::Read)
                        .show(ui)
                        .clicked();
                    (again, cancel)
                })
                .inner
            });
        ui.add_space(design::S);
        design::quiet_section(ui, "Technical details", |ui| {
            ui.add(
                egui::Label::new(Type::Small.rich(ui, &fault.system).color(colors().muted)).wrap(),
            );
        });
        if cancel {
            self.error = None;
        } else if again && let Some(PageError::Unread(unread)) = &mut self.error {
            // Read the folder again first, so the list's cause is current too.
            unread.again = true;
            self.reload = true;
        }
    }

    /// A creation form or the held NPC's card, with the pane's footer: the card's `Copy the
    /// marker` result and the error of the last Create or Save (round 27 FG2: no body places).
    fn creation_form(&mut self, ui: &mut egui::Ui, index: &ContentIndex, campaign: &Campaign) {
        let form = self.new_kind.is_some() || self.card_shown();
        // The copy's result, with the order its press issued; it has no Undo.
        let copied = self
            .generated
            .as_ref()
            .filter(|_| self.generated_shown)
            .and_then(|npc| {
                self.marker_copied
                    .as_ref()
                    .filter(|(name, _)| *name == npc.name)
                    .map(|(_, order)| *order)
            });
        let error = match &self.error {
            Some(PageError::Words(error)) if form => Some(error.clone()),
            _ => None,
        };
        let bottom = if copied.is_none() && error.is_none() {
            ui.max_rect().bottom()
        } else {
            let mut footer = design::Footer::new("world_footer", ui.available_width());
            if let Some(order) = copied {
                footer.result(order, crate::lookup::MARKER_COPIED, false, |ui| {
                    design::message(ui, design::Tone::Success, crate::lookup::MARKER_COPIED);
                });
            }
            if let Some(error) = error {
                let words = error.clone();
                footer.reason(error, move |ui| {
                    design::message(ui, design::Tone::Error, &words);
                });
            }
            footer.show(ui)
        };
        egui::ScrollArea::vertical()
            .id_salt("world_creation")
            .max_height((bottom - ui.cursor().top()).max(0.0))
            .auto_shrink([false, false])
            .show(ui, |ui| self.creation_body(ui, index, campaign));
    }

    /// Inline creation (round 27 ruling): the Label title and the fields, the named Create
    /// action after the last field on that field's line, centred on it, then one kept line
    /// below, as tall as a control: quiet `Cancel` at its start, then the reason or the new
    /// file's path. Typing moves nothing.
    fn creation_body(&mut self, ui: &mut egui::Ui, index: &ContentIndex, campaign: &Campaign) {
        let mut made = None;
        if let Some(kind) = self.new_kind {
            design::group_label(
                ui,
                match kind {
                    "npc" => "New NPC",
                    "location" => "New location",
                    _ => "New note",
                },
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
                        super::in_rect(ui, at, |ui| create().blocked(reason).show(ui).clicked())
                    };
                let mut name_field =
                    |ui: &mut egui::Ui| design::text_field(ui, "Name", name, None, None);
                if kind == "note" {
                    field(ui, false, &mut name_field);
                    field(ui, true, &mut |ui: &mut egui::Ui| {
                        design::text_field(
                            ui,
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
                let kind = if kind == "note" {
                    self.note_type.trim()
                } else {
                    kind
                };
                made = Some(super::create_entry(campaign, &self.new_name, kind));
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
                ui.add_space(design::L - ui.spacing().item_spacing.x);
                if reason.is_some() {
                    design::blocked_reason(ui, reason);
                } else if kind == "npc" {
                    let path = super::new_entry_path(Some(index), &self.new_name, "npc");
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
            if let Some(name) = self.lookup_npc.as_ref().filter(|_| self.lookup_kept) {
                // Two NPCs: the one Lookup keeps and the one on this card. Each is named, so the
                // DM knows which to save and which moves here. It is a state, so it is an info
                // message with its icon (round 26 FB8).
                design::small_message(ui, design::Tone::Info, &kept_in_lookup(name, &npc.name));
            }
            let choice = crate::lookup::generator_card(
                ui,
                npc,
                &self.generator_problems,
                &self.generator_sources,
                self.run_open,
                crate::lookup::NamesHelp::Marker,
                &super::new_entry_path(Some(index), &npc.name, "npc"),
            );
            if choice.copied_marker {
                // Each press is a new result, ahead of the older one it replaces.
                self.marker_copied = Some((npc.name.clone(), crate::design::ResultOrder::issue()));
            }
            if choice.save {
                made = Some(super::create_generated(campaign, npc));
            }
            if choice.again {
                self.roll(index);
            }
            if choice.discard
                && let Some(npc) = self.generated.take()
            {
                // The copy's result belonged to this card; `Undo discard` does not bring it back.
                self.marker_copied = None;
                self.result_order = crate::design::ResultOrder::issue();
                self.discarded_npc = Some((
                    npc,
                    std::mem::take(&mut self.generator_problems),
                    std::mem::take(&mut self.generator_sources),
                ));
                self.generated_shown = false;
                self.error = None;
            }
        }
        match made {
            Some(Ok(id)) => {
                self.selected = Some(id);
                self.view = View::Reader;
                self.new_kind = None;
                // A New form's save leaves a held NPC in its row.
                if self.generated_shown {
                    self.marker_copied = None;
                    self.generated = None;
                    self.generated_shown = false;
                }
                self.new_name.clear();
                self.note_type.clear();
                self.error = None;
                self.reload = true;
            }
            Some(Err(error)) => self.error = Some(error.into()),
            None => {}
        }
        // With no form and no card, the error is the page's content, with its Cancel; under a
        // form or a card it is in the footer (`creation_form`).
        if let Some(PageError::Words(error)) = self.error.clone()
            && self.new_kind.is_none()
            && !self.card_shown()
        {
            // Cancel starts at the message text, as every message's choices do. A message
            // action without an icon has the secondary border (round 25 family).
            let cancel = design::message_with_actions(ui, design::Tone::Error, &error, |ui| {
                design::Action::secondary("Cancel")
                    .density(design::Density::Read)
                    .show(ui)
                    .clicked()
            });
            if cancel {
                self.error = None;
            }
        }
    }

    /// The two lists of what the notes still need, above the notes themselves.
    fn check_rows(&mut self, ui: &mut egui::Ui) {
        let Some((_, checks)) = &self.checks else {
            return;
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
                // Round 26 FF3, FH1: advice, not faults. The header's `needs fixing` is the
                // only fault count, so this row names suggestions.
                PROBLEMS_TITLE,
                design::count(checks.problems(), "suggestion", "suggestions"),
                checks.problems(),
            ),
        ];
        for (view, title, detail, count) in rows {
            if count > 0
                && design::two_line_row(ui, ROW, self.view == view, title, &detail).clicked()
            {
                self.view = view;
                self.keep_repair();
                self.new_kind = None;
                self.leave_card();
                self.error = None;
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
        let mut by_page = |ui: &mut egui::Ui, found: &[super::checks::Found]| {
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
                            ui.add_space(QUESTION_BOX);
                        }
                        ui.spacing_mut().item_spacing.x = 0.0;
                        note(ui, "In ");
                        if design::small_link(ui, &index.items[group[0].page].entry.name).clicked()
                        {
                            open = Some(CheckAct::Open(group[0].page, None));
                        }
                    });
                    ui.add_space(design::S);
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
            ui.add_space(design::S);
            by_page(ui, &checks.questions);
            return tick.map(CheckAct::Tick).or(open);
        }
        design::reader::title(ui, PROBLEMS_TITLE);
        design::reader::subtitle(ui, PROBLEMS_INTRO);
        if !checks.loose_dates.is_empty() {
            design::reader::section_heading(ui, "Dates outside events");
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
            design::reader::section_heading(ui, "Pages to write");
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
                design::reader::section_heading(ui, title);
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
                design::reader::section_heading(ui, title);
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
        let words = format!("\u{201c}{}\u{201d} ticked in {name}.", found.words);
        let result = self.write_tick(campaign, item, at, &line, true);
        self.tick_error = result.err();
        if self.tick_error.is_none() {
            self.ticks
                .retain(|tick| !(tick.note == note && tick.at == at));
            self.ticks.push(Tick {
                campaign: campaign.id,
                note,
                at,
                line,
                words,
            });
            self.reload = true;
        }
    }

    /// `Undo tick`: the box of that question, in that note at that line, is open again while
    /// the line holds the tick.
    fn untick(&mut self, index: &ContentIndex, campaign: &Campaign, note: Uuid, at: usize) {
        let Some(place) = self
            .ticks
            .iter()
            .position(|tick| tick.note == note && tick.at == at)
        else {
            return;
        };
        let tick = &self.ticks[place];
        let result = if tick.campaign != campaign.id {
            Err("This tick belongs to another campaign. Nothing was changed.".to_owned())
        } else {
            match index.position(note) {
                Some(position) => {
                    self.write_tick(campaign, &index.items[position], at, &tick.line, false)
                }
                None => Err("The note of this question is gone. Nothing was changed.".to_owned()),
            }
        };
        self.tick_error = result.err();
        if self.tick_error.is_none() {
            self.ticks.remove(place);
            self.reload = true;
        }
    }

    /// Set one question's box in its note through the World's one write path, which writes
    /// only when the file still holds what this act read.
    fn write_tick(
        &self,
        campaign: &Campaign,
        item: &crate::lookup::Item,
        at: usize,
        line: &str,
        done: bool,
    ) -> Result<(), String> {
        let name = &item.entry.name;
        if self.reader.editing(item.entry.id, campaign)
            || self.reader.edited_elsewhere.contains(&item.entry.id)
        {
            return Err(format!("Save or discard your edit of {name} first."));
        }
        let before = super::file_fault::read_note(&campaign.path, &item.entry.path)?;
        // Another note that took this file's place outside Splat is not this question's note.
        if splat::content::note_id(&before) != Some(item.entry.id) {
            return Err(format!("{name} changed on disk. Nothing was written."));
        }
        let after = tick_file(&before, at, line, done)
            .ok_or_else(|| format!("This question changed in {name}. Nothing was written."))?;
        crate::library::write_note(&campaign.path, &item.entry.path, Some(&before), &after)
            .map(|_| ())
    }

    /// Open questions' footer: each tick's result with its `Undo tick`, newest first, then why
    /// the last tick wrote nothing. Returns where the questions end.
    fn questions_footer(
        &mut self,
        ui: &mut egui::Ui,
        index: &ContentIndex,
        campaign: &Campaign,
    ) -> f32 {
        let mut lines: Vec<_> = self
            .ticks
            .iter()
            .rev()
            .map(|tick| {
                super::FooterLine::new(
                    super::FooterRank::Undo,
                    super::CHANGED,
                    &tick.words,
                    super::FooterKind::Undo {
                        label: "Undo tick",
                        act: super::FooterAct::Untick(tick.note, tick.at),
                    },
                )
            })
            .collect();
        lines.extend(self.tick_error.as_ref().map(|error| {
            super::FooterLine::new(
                super::FooterRank::Reason,
                design::Tone::Error,
                error,
                super::FooterKind::Error { editor: None },
            )
        }));
        let (pressed, bottom) = super::draw_footer(ui, "world_footer", &lines);
        if let Some(super::FooterAct::Untick(note, at)) = pressed {
            self.untick(index, campaign, note, at);
        }
        bottom
    }

    fn list(&mut self, ui: &mut egui::Ui, index: &ContentIndex, groups: &WorldGroups) {
        for ((kind, title), rows) in groups {
            ui.add_space(design::XS);
            if *kind == 3 {
                design::folder_group_heading(ui, title, rows.len());
            } else {
                design::group_heading(ui, title, rows.len());
            }
            let spacing = std::mem::replace(&mut ui.spacing_mut().item_spacing.y, 0.0);
            for &(position, depth) in rows {
                let entry = &index.items[position].entry;
                let problems = super::broken_links(&index.items[position], index);
                let faults = crate::roll_table::fault_count(&entry.body);
                let duplicate = index.duplicate_world.contains_key(&entry.path);
                let name = if duplicate {
                    std::path::Path::new(&entry.path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                } else {
                    entry.name.clone()
                };
                let detail = if duplicate {
                    format!("{} Same note in two files", icons::WARN)
                } else if !entry.flags.is_empty() {
                    format!(
                        "{} {}",
                        icons::WARN,
                        design::count(entry.flags.len(), "value not read", "values not read")
                    )
                } else if faults > 0 {
                    format!(
                        "{} {}",
                        icons::WARN,
                        design::count(faults, "table fault", "table faults")
                    )
                } else if !problems.is_empty() {
                    format!("{} {}", icons::WARN, super::link_problem_words(&problems))
                } else if entry.kind == "world" {
                    if index.items.iter().any(|other| {
                        other.entry.id != entry.id
                            && other.entry.kind == "world"
                            && other.entry.name == entry.name
                    }) {
                        let file = std::path::Path::new(&entry.path)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        match super::status(entry) {
                            "" => file,
                            status => format!("{file} · {status}"),
                        }
                    } else {
                        [
                            super::date_words(entry),
                            Some(super::status(entry).to_owned()),
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
                let selected = self.view == View::Reader
                    && self.repair.is_none()
                    && self.new_kind.is_none()
                    && !self.card_shown()
                    && self.error.is_none()
                    && self.selected == Some(entry.id);
                ui.horizontal(|ui| {
                    ui.add_space(depth as f32 * design::S);
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
                        marked_row(ui, selected, &name, &detail).on_hover_text(&entry.path)
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

/// The same next step in the header and in an empty reader.
fn new_menu(ui: &mut egui::Ui) -> Option<&'static str> {
    let mut choice = None;
    let label = format!("New {}", icons::DOWN);
    let response = design::Action::secondary(&label)
        .name("New ▾")
        .icon(icons::ADD)
        .show(ui);
    egui::Popup::menu(&response).show(|ui| {
        for (word, kind) in [
            // `…` marks a choice that asks for a name first. Generate NPC makes its card at once.
            ("Note…", "note"),
            ("NPC…", "npc"),
            ("Location…", "location"),
            ("Generate NPC", "generate"),
        ] {
            if ui.button(word).clicked() {
                choice = Some(kind);
                ui.close();
            }
        }
    });
    choice
}

/// The room before a question's words: its box's 24 px target and an `S` gap.
const QUESTION_BOX: f32 = design::XL + design::S;

/// One open question: its box, when Splat can write its note, then its words as a link that
/// opens the note at its heading. A note that Splat does not write keeps the box's room, so the
/// words keep one column. Returns whether the box and the words were clicked.
fn question_row(
    ui: &mut egui::Ui,
    index: &ContentIndex,
    found: &super::checks::Found,
) -> (bool, bool) {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let (slot, _) =
            ui.allocate_exact_size(egui::vec2(QUESTION_BOX, design::XL), egui::Sense::hover());
        let mut ticked = false;
        if found.open_box.is_some() && super::edit::editable(&index.items[found.page], index) {
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
    })
    .inner
}

/// The note's text with the box of an open question set: `question` is the question as read
/// (unticked), its first line and continuation lines joined by `\n`, and `at` its first line in
/// the note's body. `None` unless that place holds the whole question, its box in the other
/// state: the act never looks for another line that reads the same.
pub(super) fn tick_file(text: &str, at: usize, question: &str, done: bool) -> Option<String> {
    let front =
        crate::sessions::split_note(text).map_or(0, |(front, _)| front.matches('\n').count());
    let mut lines = question.split('\n');
    let line = lines.next()?;
    let mut held = text.split('\n').skip(front + at + 1);
    if !lines.all(|wanted| held.next().map(|at| at.trim_end_matches('\r')) == Some(wanted)) {
        return None;
    }
    let ticked = crate::sessions::ticked(line, true);
    let (from, to) = if done {
        (line, ticked.as_str())
    } else {
        (ticked.as_str(), line)
    };
    crate::sessions::change_line(text, front + at, from, Some(to))
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
            .build_ui_state(
                {
                    let index = index.clone();
                    move |ui, page: &mut WorldPage| {
                        page.show(ui, Some(&index), &shown, &mut Default::default())
                    }
                },
                WorldPage::default(),
            );
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.state_mut().begin_new("generate", Some(&index));
        harness.run_steps(2);
        let npc = held(harness.state()).expect("the generator rolled an NPC");
        let rolled = harness.state().generated.clone().unwrap();
        let row = format!("Unsaved NPC: {npc}");
        assert!(harness.query_by_label("Save NPC").is_some());
        assert!(
            harness.query_by_label(&row).is_none(),
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
            let row_shown = harness.query_by_label(&row).is_some()
                && harness.query_by_label("Save NPC").is_none();
            if !(reached && kept && row_shown) {
                failed.push(format!(
                    "{id}: reached {reached}, kept {kept}, row {row_shown}"
                ));
                // Hold the same NPC again so the next path starts from the same state.
                let page = harness.state_mut();
                page.generated = Some(rolled.clone());
                page.generated_shown = true;
                harness.run_steps(2);
                continue;
            }
            harness.get_by_label("Show").click();
            harness.run_steps(2);
            if harness.query_by_label("Save NPC").is_none()
                || harness.query_by_label(&row).is_some()
            {
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
        assert!(harness.query_by_label(&row).is_none());
    }

    /// Discard lets the NPC go with `Undo discard`, which brings the same NPC back. Moving on
    /// ends the Undo, and a new NPC replaces it.
    #[test]
    fn iter15_d_ad1_discard_has_undo() {
        let dir = tempfile::tempdir().unwrap();
        let (campaign, index) = held_npc_campaign(dir.path());
        let mut harness = Harness::builder()
            .with_size(egui::vec2(1200.0, 800.0))
            .build_ui_state(
                {
                    let index = index.clone();
                    move |ui, page: &mut WorldPage| {
                        page.show(ui, Some(&index), &campaign, &mut Default::default())
                    }
                },
                WorldPage::default(),
            );
        design::theme::install(&harness.ctx);
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
