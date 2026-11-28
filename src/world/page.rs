//! The World page: what is true in this world. One list of notes and entries, and the Reader.
use crate::world::{Nav, Reader, checks::tick_file, file_fault::FileFault};
use crate::{
    design::{
        self, icons,
        outcome::{Done, FaultMessage, Messages, Notice, PaneFrame, Said, Verb},
    },
    lookup::{ContentIndex, WorldGroups},
    persistence::Campaign,
};
use std::sync::Arc;
use uuid::Uuid;

/// More rows than this in one check are counted, not shown.
pub const CHECK_ROWS: usize = 40;
/// The line above the World card while Lookup holds a second unsaved NPC (round 26 FB8).
pub fn kept_in_lookup(lookup: &str, card: &str) -> String {
    format!("{lookup} is not saved. It is in Lookup. Save or discard {card} to show {lookup} here.")
}

/// The advice page's row and title: what to check, not what is broken (round 26 FF3, FH1).
pub const PROBLEMS_TITLE: &str = "Things to check";

/// What the Things to check page lists, in the words of its sections.
pub const PROBLEMS_INTRO: &str = "These lists show notes with a date outside an event, a link to no note, no summary, no links, or a name or sentence on two notes. Splat only lists them. You change the notes.";
pub use crate::world::UNSAVED_NPC;

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
    /// The list's groups for one load of the index and one set of search words. A search reads
    /// the text of every note, so a frame with neither changed reads none (`groups`).
    groups: Option<GroupsKept>,
}

#[cfg(test)]
thread_local! {
    /// Searches of the notes for the list's groups (`WorldPage::groups`), for a check.
    static GROUP_SEARCHES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The list's groups, and the index load and search words they were found for.
struct GroupsKept {
    generation: u64,
    search: String,
    groups: Arc<WorldGroups>,
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

    /// `Undo discard`: the same NPC, with its table facts, on its card again.
    fn undo_discard(&mut self) {
        if let Some((npc, problems, sources)) = self.discarded_npc.take() {
            self.generated = Some(npc);
            self.generator_problems = problems;
            self.generator_sources = sources;
            self.show_held();
        }
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

/// The page's view of the World, for the GPUI page, and its acts.
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

    /// The list's groups for the search words: found again only when the index loads again or
    /// the words change.
    pub fn groups(&mut self, index: &ContentIndex) -> Arc<WorldGroups> {
        let search = self.search.trim().to_lowercase();
        let generation = index.generation();
        match &self.groups {
            Some(kept) if kept.generation == generation && kept.search == search => {
                kept.groups.clone()
            }
            _ => {
                #[cfg(test)]
                GROUP_SEARCHES.with(|count| count.set(count.get() + 1));
                let groups = Arc::new(index.world_groups(&search));
                self.groups = Some(GroupsKept {
                    generation,
                    search,
                    groups: groups.clone(),
                });
                groups
            }
        }
    }

    /// The page has nothing at all: no note, no search, no form, no NPC and no error.
    pub fn is_empty(&mut self, index: &ContentIndex) -> bool {
        index.broken_world.is_empty()
            && self.groups(index).is_empty()
            && self.search.trim().is_empty()
            && self.new_kind.is_none()
            && self.generated.is_none()
            && self.error.is_none()
    }

    /// The list's groups and rows for the search words.
    pub fn list_groups(&mut self, index: &ContentIndex) -> Vec<ListGroup> {
        let groups = self.groups(index);
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
    ) -> Vec<design::outcome::plain::PlainLine<crate::world::FooterAct>> {
        let frame = design::outcome::PaneFrame::new();
        let mut messages = Messages::new();
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
    pub fn creation_footer(
        &self,
    ) -> Vec<design::outcome::plain::PlainLine<crate::world::FooterAct>> {
        let frame = design::outcome::PaneFrame::new();
        let mut messages: Messages<'_, crate::world::FooterAct> = Messages::new();
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

    /// The held NPC's row shows above another view, and its card is not the view.
    fn row_shown(page: &WorldPage, index: &ContentIndex, npc: &str) -> bool {
        page.held_name() == Some(npc) && page.card(index).is_none()
    }

    /// The pane's footer words and Undo labels, with no note open.
    fn footer(page: &mut WorldPage, index: &ContentIndex, campaign: &Campaign) -> Vec<String> {
        page.footer_plain(None, index, campaign, |_, _| ())
            .into_iter()
            .flat_map(|line| {
                std::iter::once(line.words.clone()).chain(line.undo.map(|undo| undo.label))
            })
            .collect()
    }

    /// AD1/SD1/FD1: an unsaved generated NPC is a held draft. Every path to another view keeps
    /// it in one row above that view, Show brings its card back, and Save writes it once.
    /// Each path's failure is collected, so one fault build names every path it breaks.
    #[test]
    fn iter15_d_ad1_every_path_keeps_a_held_npc() {
        let dir = tempfile::tempdir().unwrap();
        let (campaign, index) = held_npc_campaign(dir.path());
        assert_eq!(index.broken_world.len(), 1, "the fixture has a repair");
        let id_of = |name: &str| {
            index
                .items
                .iter()
                .find(|item| item.entry.name == name)
                .unwrap()
                .entry
                .id
        };
        let (rook, bells) = (id_of("Rook"), id_of("Bells"));
        let mut page = WorldPage::default();
        page.follow_campaign(campaign.id);
        page.refresh(&index, &campaign);
        page.press_new("generate", Some(&index));
        let npc = held(&page).expect("the generator rolled an NPC");
        let rolled = page.generated.clone().unwrap();
        assert!(page.card(&index).is_some(), "the card shows");
        assert!(
            !row_shown(&page, &index, &npc),
            "no row while the card shows"
        );

        type Path<'a> = (&'a str, Box<dyn Fn(&mut WorldPage) -> bool + 'a>);
        let paths: Vec<Path> = vec![
            (
                "AD1-problems",
                Box::new(|page| {
                    let listed = page.check_rows_shown().iter().any(|(view, title, ..)| {
                        *view == View::Problems && *title == PROBLEMS_TITLE
                    });
                    page.press_check_row(View::Problems);
                    listed && matches!(page.pane(&index), Pane::Checks(View::Problems))
                }),
            ),
            (
                "AD1-questions",
                Box::new(|page| {
                    page.press_check_row(View::Questions);
                    matches!(page.pane(&index), Pane::Checks(View::Questions))
                        && page.checks().is_some_and(|checks| {
                            checks.questions.iter().any(|q| q.words == "Who pays Rook?")
                        })
                }),
            ),
            (
                "AD1-reveal",
                Box::new(|page| {
                    page.reveal(campaign.id, rook);
                    page.selected() == Some(rook)
                        && matches!(page.pane(&index), Pane::Note(at) if index.items[at].entry.id == rook)
                }),
            ),
            (
                "AD1-repair-opens",
                Box::new(|page| {
                    page.open_repair(&campaign, "Broken.md");
                    matches!(page.pane(&index), Pane::Repair)
                }),
            ),
            (
                "AD1-repair-ends-on-another-note",
                Box::new(|page| {
                    page.open_repair(&campaign, "Broken.md");
                    let opened = page.repair.is_some();
                    page.press_note(bells);
                    opened && page.repair.is_none() && page.selected() == Some(bells)
                }),
            ),
            (
                "AD1-new-note",
                Box::new(|page| {
                    page.press_new("note", Some(&index));
                    let form = page.new_kind() == Some("note")
                        && matches!(page.pane(&index), Pane::Creation);
                    page.cancel_new();
                    form && page.new_kind().is_none()
                }),
            ),
            (
                "AD1-new-npc-saved",
                Box::new(|page| {
                    page.press_new("npc", Some(&index));
                    let form = page.new_kind() == Some("npc");
                    page.set_new_name("Tam");
                    page.create(&campaign);
                    // The New form's save writes only its own note.
                    form && page.new_kind().is_none() && npc_files(dir.path()) == ["Tam.md"]
                }),
            ),
        ];
        let mut failed = Vec::new();
        for (id, path) in &paths {
            let reached = path(&mut page);
            let kept = held(&page).as_deref() == Some(npc.as_str());
            let shown = row_shown(&page, &index, &npc);
            if !(reached && kept && shown) {
                failed.push(format!("{id}: reached {reached}, kept {kept}, row {shown}"));
                // Hold the same NPC again so the next path starts from the same state.
                page.generated = Some(rolled.clone());
                page.generated_shown = true;
                continue;
            }
            page.press_show_held();
            if page.card(&index).is_none() || row_shown(&page, &index, &npc) {
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
                page.press_new("generate", None)
            }),
        ] {
            page.press_check_row(View::Problems);
            open(&mut page);
            if held(&page).as_deref() != Some(npc.as_str())
                || page.card(&index).map(|card| card.npc.name) != Some(npc.clone())
            {
                failed.push(format!("{id}: the held card is not shown"));
            }
        }
        assert!(failed.is_empty(), "{failed:#?}");

        page.card_act(CardAct::Save, &campaign, &index);
        let mut expected = vec![format!("{npc}.md"), "Tam.md".to_owned()];
        expected.sort();
        assert_eq!(
            npc_files(dir.path()),
            expected,
            "AD1-saved: the held NPC saves to one file, beside the New form's note"
        );
        assert!(held(&page).is_none());
        assert!(page.held_name().is_none(), "no row after the save");
    }

    /// Discard lets the NPC go with `Undo discard`, which brings the same NPC back. Moving on
    /// ends the Undo, and a new NPC replaces it.
    #[test]
    fn iter15_d_ad1_discard_has_undo() {
        let dir = tempfile::tempdir().unwrap();
        let (campaign, index) = held_npc_campaign(dir.path());
        let mut page = WorldPage::default();
        page.follow_campaign(campaign.id);
        page.refresh(&index, &campaign);
        page.press_new("generate", Some(&index));
        let npc = held(&page).unwrap();
        let attributes = page.generated.clone().unwrap().attributes;
        let notice = format!("{npc} draft discarded.");
        let undo =
            crate::design::outcome::UndoStep::of(crate::design::outcome::Verb::Discard, &npc)
                .label();
        page.card_act(CardAct::Discard, &campaign, &index);
        assert!(held(&page).is_none(), "Discard lets the NPC go");
        assert!(page.card(&index).is_none());
        let words = footer(&mut page, &index, &campaign);
        assert!(words.contains(&notice), "AD1-discard-notice: {words:?}");
        assert!(words.contains(&undo), "AD1-discard-undo: {words:?}");
        let press = page
            .footer_plain(None, &index, &campaign, |_, _| ())
            .iter()
            .find_map(|line| {
                line.undo
                    .as_ref()
                    .filter(|u| u.label == undo)
                    .and(line.press_undo())
            })
            .expect("the Undo can be pressed");
        page.footer_press_plain(
            press,
            &index,
            &campaign,
            &mut crate::tracker_reader::Reader(&mut Default::default()),
        );
        assert_eq!(
            page.generated.as_ref().map(|n| (&n.name, &n.attributes)),
            Some((&npc, &attributes)),
            "AD1-undo-discard: the same NPC comes back"
        );
        assert!(page.card(&index).is_some());
        assert!(!footer(&mut page, &index, &campaign).contains(&notice));

        // Moving on ends the Undo. The NPC is then gone and nothing was written.
        page.card_act(CardAct::Discard, &campaign, &index);
        assert!(footer(&mut page, &index, &campaign).contains(&notice));
        page.press_check_row(View::Problems);
        assert!(
            !footer(&mut page, &index, &campaign).contains(&notice) && page.discarded_npc.is_none(),
            "AD1-undo-ends: another view ends the Undo"
        );
        assert!(npc_files(dir.path()).is_empty(), "Discard writes nothing");

        // A new NPC replaces a waiting Undo, so Undo never replaces an unsaved NPC.
        page.press_new("generate", Some(&index));
        page.card_act(CardAct::Discard, &campaign, &index);
        page.press_new("generate", Some(&index));
        assert!(page.discarded_npc.is_none());
        assert!(
            !footer(&mut page, &index, &campaign)
                .iter()
                .any(|words| words.starts_with("Undo discard"))
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

    /// The list's groups are found once per load of the index and set of search words: a quiet
    /// frame searches no note.
    #[test]
    fn groups_found_once_per_load_and_search() {
        let dir = tempfile::tempdir().unwrap();
        let (_, index) = held_npc_campaign(dir.path());
        let searches = || GROUP_SEARCHES.with(std::cell::Cell::get);
        let mut page = WorldPage::default();
        let before = searches();
        assert!(!page.groups(&index).is_empty());
        assert!(!page.is_empty(&index));
        let _ = page.list_groups(&index);
        assert_eq!(
            searches(),
            before + 1,
            "the same load and words search the notes once"
        );
        page.set_search("  ROOK ");
        let rook = page.groups(&index);
        assert_eq!(searches(), before + 2, "new words search again");
        assert_eq!(rook.values().map(Vec::len).sum::<usize>(), 1);
        page.set_search("rook");
        let _ = page.groups(&index);
        assert_eq!(
            searches(),
            before + 2,
            "the same words, trimmed and lowercased, search nothing"
        );
        let again = Arc::new(ContentIndex::load(&[], None).unwrap());
        assert!(page.groups(&again).is_empty());
        assert_eq!(searches(), before + 3, "a new load searches again");
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
