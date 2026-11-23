//! The Library page and its authoring state.
#[path = "library/kept.rs"]
mod kept;
use crate::{
    campaigns::Campaigns,
    content_workspace::CampaignDocument,
    design::{
        self, Action, Density, Status, Tone, Type, colors, icons,
        outcome::{
            Banner, BodyFault, Choice, FaultMessage, FaultStatus, KeptOutcome, MessageChoice,
            Messages, Notice, Offer, PagePlaceholder, PaneFrame, Press, Repair, Said, Sentence,
            UndoStep, Verb,
        },
        reader as reader_style,
        results::{ByTicket, Ticket},
    },
    history::{self, Step, Trail},
    matching::{CLOSE_NAMES, Rank, Typed},
    persistence::{Campaign, Module},
    world::file_fault::FileFault,
};
use eframe::egui;
use kept::{KeptKind, LibraryKept};
use presentation::Section;
use splat::content::{
    self, Entry,
    creature_filter::{Filter, Query},
};
pub use splat_content_ui::library::*;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
};
use uuid::Uuid;

const SEARCH_NAME: &str = "Search the Library";

/// The place of the open entry, for Back and Forward. A free function, so a list closure that
/// holds the notes can record the place it leaves (round 26 SB1).
fn place(
    loaded: &Loaded,
    selected: Option<usize>,
    scope: Scope,
    section: Section,
    search: &str,
) -> Option<Place> {
    let current = loaded.notes.get(selected?)?;
    Some(Place {
        owner: current.owner.key(),
        path: current.path.clone(),
        scope,
        section,
        search: search.to_owned(),
    })
}

struct SelectionUndo {
    campaign: Campaign,
    before: Vec<crate::content_workspace::PackageSelection>,
    after: Vec<crate::content_workspace::PackageSelection>,
}

struct DependencyUndo {
    root: PathBuf,
    path: String,
    before: String,
    after: String,
    copied_path: String,
    copied_text: String,
    /// The copied entry's name and the package's, for its result (`<entry> copied to <package>
    /// and linked.`) and its `Undo copy <entry>`. Each copy is one of the Library's kept
    /// results (`LibraryKept::push_copy`), owned by the file it linked and the path it copied
    /// (a file at the same path in another package never shows it); it ends on its own Undo, or
    /// on a reload that finds one of its files changed (round 26 review).
    name: String,
    package: String,
}

struct IdentityUndo {
    root: PathBuf,
    path: String,
    before: String,
    after: String,
}

/// The draft as parsed, with the revision of that parse: a new parse (new draft text) is a new
/// revision, and caches of what the parse holds are keyed by it.
type ParsedDraft = design::memo::Revised<std::sync::Arc<Result<Entry, String>>>;

/// A missing link Campaign's repair sent to a file: the file, the typed reference and the
/// fault's words. `found` is the row that links it in one parse of the draft, keyed by that
/// parse's revision; the draft's parse is rebuilt only when its text changes, so the row is
/// looked up again only then.
struct RepairLink {
    path: PathBuf,
    target: splat::content::Reference,
    words: String,
    found: design::memo::Memo<design::memo::Revision, Option<usize>>,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static REPAIR_LOOKUPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
thread_local! {
    /// How many times the entry list searched the matches for the selected row (issue #539).
    static ROW_SEARCHES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The row that links a repair's reference. Every repair lookup goes through here, and the
/// test count is kept here.
fn repair_row(entry: &Entry, target: &splat::content::Reference) -> Option<usize> {
    #[cfg(any(test, feature = "test-support"))]
    REPAIR_LOOKUPS.with(|count| count.set(count.get() + 1));
    splat::content::list_formats::row_linking(entry, target)
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static BROKEN_LINE_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Edit Markdown's warning for each link to no entry in a draft, as the row's `N broken links`
/// counts them (`Line 17: This link names no entry.`), outside the front matter and code fences.
/// `skip` is the line the link Campaign sent the DM to fix, which has its own words. The whole
/// preparation (the scan and the words) goes through here, and the test count is kept here.
fn broken_link_warnings(
    draft: &str,
    entry: &Entry,
    index: &crate::lookup::ContentIndex,
    skip: Option<usize>,
) -> Vec<Sentence> {
    #[cfg(any(test, feature = "test-support"))]
    BROKEN_LINE_SCANS.with(|count| count.set(count.get() + 1));
    let mut lines = Vec::new();
    let mut front = false;
    let mut fence = false;
    for (at, line) in draft.lines().enumerate() {
        let trimmed = line.trim();
        if at == 0 && trimmed == "---" {
            front = true;
            continue;
        }
        if front {
            front = trimmed != "---";
            continue;
        }
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fence = !fence;
            continue;
        }
        if fence || presentation::hidden_provenance_line(line) {
            continue;
        }
        for target in crate::world::links_in(line) {
            if index.link_is_missing(&target, entry.id) && skip != Some(at + 1) {
                lines.push(crate::sentence!(
                    "Line {}: This link names no entry.",
                    at + 1
                ));
            }
        }
    }
    lines
}

/// The broken-link warnings of one parsed draft against one link index, with the line Campaign's
/// link has. A new draft (a new parse), a new index or another such line prepares them again; an
/// unchanged frame prepares nothing and borrows them. The parse is known by its revision.
type BrokenLines = (
    design::memo::Revision,
    // The index's load (`ContentIndex::generation`), never its address.
    u64,
    Option<usize>,
    Vec<Sentence>,
);

#[derive(Default)]
pub struct Library {
    /// The draft as last parsed, with its path. The editor needs the parse on every frame,
    /// and parsing a long note each time took most of the frame.
    draft_parse: Option<(String, crate::design::text_draft::DraftKey, ParsedDraft)>,
    /// The list should scroll to the selected entry, which arrived by a link.
    reveal_row: bool,
    /// The selected entry's row in the matches when last found (issue #539): checked in one
    /// step on a quiet frame (`matches[row]` is still the entry), searched again only when not.
    selected_row: Option<usize>,
    /// The entry list's region (layout step 3b) and its rows: whole rows only, one offset writer.
    entry_region: Option<design::RegionState>,
    entry_rows: Option<design::memo::Revised<Vec<design::Block>>>,
    list_hidden: bool,
    /// The Library's kept results (Undos and notices that stay), with their footer order: data
    /// and mark change together behind one boundary (`kept`, M-L0.4a).
    kept: LibraryKept,
    /// A Save that failed on the disk, for the draft it tried to write.
    save_failure: Option<SaveFailure>,
    /// Raised by every event that changes the open draft (an edit, Discard and its Undo, a
    /// reload, a repair action). A failed Save keeps the revision it wrote, so no frame compares
    /// the draft's words to know whether it is still that draft.
    draft_revision: u64,
    /// Test only: how often the failed-Save check ran, and how often it read the draft's words
    /// (counted by `DraftView` itself).
    #[cfg(any(test, feature = "test-support"))]
    failure_checks: std::cell::Cell<(usize, usize)>,
    /// Each copied package's registration result, with all its Undo needs, by copy and campaign.
    registrations: design::Results<RegistrationOwner, Registration>,
    copy_not_used: bool,
    /// Each registration's `Use the copy in encounters` result, by the same owner.
    rosters: design::Results<RegistrationOwner, RostersUse>,
    /// Offers to change links after a rename, each owned by the renamed entry.
    renamed_links: design::Results<LinkOwner, LinkOffer>,
    /// Link updates and the names their links now say, each owned by the renamed entry.
    link_edits: design::Results<LinkOwner, LinkResult>,
    /// The press in the pane's typed footer that `show` applies after the pane is drawn
    /// (`apply_footer_press`): the page's results and the package's.
    footer_press: Option<Press<LibraryPress>>,
    copied_package: Option<Module>,
    copy_name: String,
    copy_name_open: bool,
    /// A save was refused because the file changed. The draft waits for the DM's choice.
    disk_change: Option<DiskChange>,
    /// The note on screen was saved, and has not changed since.
    just_saved: bool,
    overwrite_disk: bool,
    scope: Scope,
    scope_menu: Option<ScopeMenu>,
    load_key: Option<LoadKey>,
    stale: bool,
    matches_scope: Option<Scope>,
    scope_counts: Option<(Scope, Vec<(Section, usize)>)>,
    confirm_unregister: Option<Module>,
    new_place: Option<usize>,
    /// A saved note's owner folder, so other pages can read it again.
    pub content_changed: Option<PathBuf>,
    /// A package's new name, for the registration.
    pub renamed: Option<(Uuid, String)>,
    /// The DM asked to go and add a campaign.
    pub wants_settings: bool,
    loaded: Option<Loaded>,
    pending: Option<mpsc::Receiver<Result<Loaded, String>>>,
    selected: Option<usize>,
    draft: crate::design::text_draft::TextDraft,
    /// The last answers of the draft's compares (layout PR 9b-C): with its note's saved text, and
    /// with the heading and stat fixes' words. Each by the two revisions it compared.
    same_saved: crate::design::text_draft::KeptCompare,
    same_heading: crate::design::text_draft::KeptCompare,
    same_vital: crate::design::text_draft::KeptCompare,
    search: String,
    matches: Option<(Section, String, Vec<usize>)>,
    /// Chosen creature facts. They apply in Creatures and to search results.
    filter: Filter,
    /// The filter that `matches` used.
    matched_filter: Filter,
    /// Where results found only with a typo start in `matches`.
    typo_from: usize,
    section: Section,
    unreadable_kind: Option<Section>,
    error: Option<PageError>,
    form: bool,
    path: String,
    name: String,
    pub run_open: bool,
    pub open_world_npc: Option<Uuid>,
    pending_edit: Option<Entry>,
    pending_repair: Option<PathBuf>,
    /// A missing link that Campaign's repair opened this file for. Edit Markdown names it at its
    /// line while the draft still holds it; it ends when another note opens or a Save writes.
    repair_link: Option<RepairLink>,
    /// The open draft's broken-link lines, for Edit Markdown's warnings (round 25 FH6).
    broken_lines: Option<BrokenLines>,
    new_form: bool,
    focus_new_name: bool,
    focus_hp: bool,
    focus_repair: bool,
    focus_markdown: bool,
    new_name: String,
    new_kind: String,
    new_folder: String,
    new_not_used: bool,
    /// The New entry form's last create error. It shows in the pane's message footer and ends
    /// when the form closes, the name changes, or an entry is created.
    new_error: Option<PageError>,
    package_not_used: bool,
    focus_package_name: bool,
    package_path: Option<PathBuf>,
    package_folder_changed: bool,
    pub use_entry: Option<(Uuid, Uuid)>,
    source: bool,
    reselect: Option<FileId>,
    pub copy: Option<crate::content_workspace::CopySource>,
    pub copy_to_package: Option<Uuid>,
    /// An entry to read beside the open draft, in Lookup.
    pub peek: Option<Uuid>,
    trail: Trail<Place>,
    link_choices: Vec<Uuid>,
    /// Test only: how often the registration results were checked (`reconcile_registrations`).
    #[cfg(any(test, feature = "test-support"))]
    reconciles: usize,
}

pub const EDITOR_MAX_WIDTH: f32 = 720.0;

fn obsidian_for(root: &Path, owner: &Owner) -> bool {
    matches!(owner, Owner::Campaign(_))
        || root
            .ancestors()
            .any(|parent| parent.join(".obsidian").is_dir())
}

/// The reader's owner and file line is one Read row for every owner, whatever it holds
/// (status, `Show file`, a quiet `Choose all`), so the title below it does not move
/// between a built-in entry, a campaign entry and a package About.
fn file_line_row(ui: &mut egui::Ui, wrap: bool, add: impl FnOnce(&mut egui::Ui)) {
    let height = Density::Read.control();
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), height),
        egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(wrap),
        |ui| {
            ui.set_min_height(height);
            add(ui);
        },
    );
}
/// What is on disk now, in place of what the draft started from.
struct DiskChange {
    /// None when the file has gone, or cannot be read.
    text: Option<String>,
    moved_to: Option<String>,
    /// Why the file is there but cannot be read. The draft cannot be saved over it.
    fault: Option<FileFault>,
}

/// What a note's file holds now: its text, None when it has gone, or why it cannot be read.
fn read_disk(root: &Path, relative: &str) -> Result<Option<String>, FileFault> {
    let Ok(path) = note_path(root, relative) else {
        return Ok(None);
    };
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(FileFault::read(relative, &error)),
    }
}

/// Check readable and broken notes, so a move cannot create a second identity.
fn moved_note(root: &Path, id: Uuid, old: &str) -> Result<Option<(String, String)>, PageError> {
    let (entries, errors) = content::read_directory(root)?;
    let mut found = None;
    for path in entries
        .into_iter()
        .map(|e| e.path)
        .chain(errors.into_iter().map(|e| e.path))
    {
        if path == old {
            continue;
        }
        // A file that cannot be read might hold this ID: say which, rather than guess.
        let text = fs::read_to_string(note_path(root, &path)?)
            .map_err(|e| FileFault::read(&path, &e).message())?;
        if crate::content_workspace::note_id(&text) == Some(id) {
            if found.is_some() {
                return Err(PageError::Own(SEVERAL_FILES_KEEP_ONE.clone()));
            }
            found = Some((path, text));
        }
    }
    Ok(found)
}
/// An entry Back or Forward returns to, with the list as it was there.
struct Place {
    owner: OwnerKey,
    path: String,
    scope: Scope,
    section: Section,
    search: String,
}
impl Place {
    fn find(&self, notes: &[Note]) -> Option<usize> {
        file_at(notes, (self.owner, &self.path))
    }
}

/// The note that is this file: its owner's kind and id, and its path. The one key that Back and
/// Forward (`Place::find`), the reselection after a reload and the new entry's selection use,
/// so a campaign file never stands for a package file with the same UUID and path (M-L0.4a).
fn file_at(notes: &[Note], (owner, path): (OwnerKey, &str)) -> Option<usize> {
    notes
        .iter()
        .position(|note| note.owner.key() == owner && note.path == path)
}
/// The issues count's menu: the faults, then a link for each file with broken links that opens
/// it (as in World). Round 26 FB1: as wide as its content, up to the reader measure, and never
/// more than its room (a longer link is cut and a message wraps). M0330: its scroll column is as
/// tall as its lines up to six controls, so one link leaves no empty band and a long list
/// scrolls. M0331: while a draft holds the open entry (`open_lock`), no other entry opens; the
/// menu says why and lists the files as text.
fn issues_menu(
    ui: &mut egui::Ui,
    loaded: &Loaded,
    broken: &[(usize, usize)],
    stopped_reading: &Option<(FileId, String)>,
    open_lock: Option<&str>,
    open_broken: &mut Option<usize>,
) {
    let link_label = |at: usize, count: usize| {
        format!(
            "{}: {}",
            loaded.file_label(&loaded.notes[at]),
            design::count(count, "broken link", "broken links")
        )
    };
    let font = Type::Small.font(ui);
    let natural = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), colors().text)
            .size()
            .x
    };
    let width = loaded
        .diagnostics
        .iter()
        .map(|message| natural(&message.text))
        .chain(stopped_reading.as_ref().map(|(_, text)| natural(text)))
        .chain(
            broken
                .iter()
                .map(|&(at, count)| natural(&link_label(at, count))),
        )
        .fold(design::XL, f32::max)
        .min(design::READER_MEASURE)
        .ceil();
    // The scroll column keeps its lane beside the widest line.
    ui.set_width((width + design::lane_width()).min(ui.available_width()));
    design::ScrollColumn::new("library_file_issues")
        .max_height(design::XXL * 6.0)
        .fit_height()
        .show(ui, |ui| {
            for message in loaded
                .diagnostics
                .iter()
                .map(|message| message.text.as_str())
                .chain(stopped_reading.as_ref().map(|(_, text)| text.as_str()))
            {
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, message).color(colors().muted)).wrap(),
                );
            }
            if !broken.is_empty() {
                design::blocked_reason(ui, open_lock);
            }
            for &(at, count) in broken {
                let label = link_label(at, count);
                if open_lock.is_some() {
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, &label).color(colors().muted)).wrap(),
                    );
                } else if design::small_link(ui, &label).clicked() {
                    *open_broken = Some(at);
                    ui.close();
                }
            }
        });
}

impl Library {
    /// The typed outcome of the Library's kept result `kind` (M-L0.4b), made from its data in
    /// the store: its words and its Undo, with the lock the open draft puts on it (`results`).
    /// The one place a kind's line is made for a typed footer. `None` for a kind that no typed
    /// footer shows yet (the reader's and the editor's, in 4b-2) or whose data has gone.
    #[allow(
        clippy::disallowed_methods,
        reason = "a manifest write’s words start with the file’s path from disk (ruling M17)"
    )]
    fn outcome_of(
        &self,
        kind: KeptKind,
        results: &ReaderResults,
    ) -> Option<KeptOutcome<LibraryPress>> {
        let undo = |step: UndoStep| step.pressed_as(LibraryPress::Undo);
        Some(match kind {
            KeptKind::Unregistered => {
                let module = self.kept.unregistered()?;
                // A finished change: the package left this campaign's lists.
                Said::new(crate::sentence!(
                    "{} is unregistered. Its files stay in {}.",
                    module.name,
                    module.path.display()
                ))
                .undo(undo(UndoStep::of(Verb::Unregister, module.name.as_str())))
                .into()
            }
            KeptKind::PackageUse => Said::new(PACKAGE_USE_RESULT)
                .undo(undo(PACKAGE_CHOICE_UNDO))
                .into(),
            KeptKind::Manifest => {
                let write = self.kept.manifest()?;
                let step = UndoStep::of(Verb::Write, write.path().to_string());
                Said::new(
                    Sentence::outside(write.message())
                        .unwrap_or(crate::sentence!("The manifest was written again.")),
                )
                .undo(results.draft_lock(step, LibraryPress::Undo))
                .into()
            }
            KeptKind::Removed => {
                let LetGo { note, .. } = self.kept.removed()?;
                let names = self
                    .loaded
                    .as_ref()
                    .map(|loaded| loaded.dependents(note))
                    .unwrap_or_default();
                let words = if names.is_empty() {
                    crate::sentence!("{} removed. Its draft was discarded.", note.name())
                } else {
                    crate::sentence!(
                        "{} removed. Its draft was discarded. {} now needs a missing entry.",
                        note.name(),
                        name_list(&names)
                    )
                };
                KeptOutcome::Notice(
                    Notice::new(words),
                    undo(UndoStep::of(Verb::Remove, note.name())),
                )
            }
            KeptKind::Identities => {
                let (_, words) = self.kept.identities()?;
                Said::new(words.clone())
                    .undo(
                        results.draft_lock(
                            UndoStep::of(Verb::Separate, "entries"),
                            LibraryPress::Undo,
                        ),
                    )
                    .into()
            }
            KeptKind::Identity => {
                let undo = self.kept.identity()?;
                let file = Path::new(&undo.path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                Said::new(crate::sentence!("{} is now a separate entry.", file))
                    .undo(results.draft_lock(SEPARATE_ENTRY_UNDO, LibraryPress::Undo))
                    .into()
            }
            // Drawn where their show checks live: the editor pane's footer (Discarded, Heading,
            // Vital, each copy) and as a notice (ScopeNotice).
            KeptKind::ScopeNotice
            | KeptKind::Discarded
            | KeptKind::Heading
            | KeptKind::Vital
            | KeptKind::Copy => return None,
        })
    }

    /// Why the page's own acts wait: the open draft, or the load (`Wait for the Library to load.`).
    fn page_lock(&self) -> Option<Sentence> {
        self.draft_lock().or(self
            .pending
            .is_some()
            .then(|| crate::sentence!("Wait for the Library to load.")))
    }

    /// Why an act waits for the open draft, as a typed lock (`draft_reason`'s words).
    fn draft_lock(&self) -> Option<Sentence> {
        if self.disk_change.is_some() {
            Some(CHANGED_FILE_FIRST.clone())
        } else if self.dirty() {
            Some(match self.draft_blocks() {
                design::DraftBlocks::EntryInCampaign => DRAFT_BLOCKS_IN_CAMPAIGN.clone(),
                design::DraftBlocks::EntryInPackage => DRAFT_BLOCKS_IN_PACKAGE.clone(),
            })
        } else {
            None
        }
    }

    /// The page's results and this pane's kept results as typed lines of a footer with no
    /// reader: a form's, the empty scope's and the empty reader's (M-L0.4b). Each comes from its
    /// own store, with its ticket, under the rules `show` gives the reader's legacy footer
    /// (`PageResults`); `results` says whether this pane shows the package choice and the
    /// manifest write, and the open draft's lock. The scope notice and the link offer have no
    /// Undo: the scope notice is a notice, the link offer a kept offer, both in the info tone.
    fn page_messages<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, LibraryPress>,
        results: &ReaderResults,
        campaigns: &Campaigns,
    ) {
        if let Some((name, _)) = self.kept.scope_notice() {
            // It wrote nothing: the scope only widened to show the entry.
            messages.outcome(Notice::new(scope_notice_words(name)));
        }
        // A link offer and a link result show only in their own entry's reader (or a form in
        // its place).
        let open = self.open_link_owner();
        if let Some(shown) = open
            .as_ref()
            .and_then(|owner| self.renamed_links.get(owner))
        {
            let offer = shown.value();
            let count = self.link_rename_count(&offer.old, &offer.new);
            if count > 0 {
                // The offer names both names, so the DM can check the words before the write.
                // A kept offer line: nothing was done yet, so the info tone, never a success.
                messages.kept(shown.outcome(frame, |offer| {
                    Offer::new(
                        crate::sentence!("Change links from “{}” to “{}”.", offer.old, offer.new),
                        Choice::named(
                            crate::choice!("Update {}", design::count(count, "link", "links")),
                            LibraryPress::UpdateLinks,
                        ),
                    )
                    .into()
                }));
            }
        }
        if let Some(shown) = open.as_ref().and_then(|owner| self.link_edits.get(owner)) {
            messages.kept(shown.outcome(frame, |result| {
                Said::new(crate::sentence!("Links now say “{}”.", result.new))
                    .undo(LINK_UPDATE_UNDO.pressed_as(LibraryPress::UndoLinks))
                    .into()
            }));
        }
        // This campaign's newest registration result. Another campaign's stays in the store and
        // shows there.
        let registration = self.shown_registration(campaigns);
        if let Some(owner) = &registration
            && let Some(shown) = self.registrations.get(owner)
        {
            // The other choice starts below, at the message words. Only Undo is quiet.
            let use_copy = (campaigns.active_campaign().is_some()
                && self.rosters.get(owner).is_none())
            .then(|| {
                let choice = Choice::new(MessageChoice::Choice(USE_COPY), LibraryPress::UseCopy);
                match self.draft_lock() {
                    Some(reason) => choice.blocked(reason),
                    None => choice,
                }
            });
            messages.kept(shown.outcome(frame, |registered| {
                // After an outside change the Undo stays, blocked, with its reason. The result
                // still happened.
                let undo = PACKAGE_REGISTRATION_UNDO.pressed_as(LibraryPress::UndoRegistration);
                let undo = match registered.lock() {
                    Some(reason) => undo.blocked(reason),
                    None => undo,
                };
                let said = Said::new(registered.words()).undo(undo);
                match use_copy {
                    Some(choice) => said.choice(choice),
                    None => said,
                }
                .into()
            }));
        }
        if let Some(shown) = registration.and_then(|owner| self.rosters.get(&owner)) {
            messages.kept(shown.outcome(frame, |source| {
                let undo = SOURCE_CHANGE_UNDO.pressed_as(LibraryPress::UndoRosters);
                let undo = if source.blocked {
                    undo.blocked(crate::content_workspace::UNDO_ENDED_REASON)
                } else {
                    undo
                };
                Said::new(USE_COPY_RESULT).undo(undo).into()
            }));
        }
        for (kind, shows) in [
            (KeptKind::Unregistered, true),
            (KeptKind::PackageUse, results.package_use),
            (KeptKind::Manifest, results.manifest),
            (KeptKind::Identities, results.identities),
            (KeptKind::Identity, results.identity),
        ] {
            if shows
                && let Some(line) = self
                    .kept
                    .outcome(frame, kind, || self.outcome_of(kind, results))
            {
                messages.kept(line);
            }
        }
    }

    /// The stat fix (`Use <value>`) of `file`: the draft gets the value, and the fix's Undo is
    /// the newest kept result (its file whole: owner kind, owner id and path). It takes the
    /// parts it changes, so the editor can call it while it reads the open note.
    fn fix_stat(
        (kept, draft, revision, error): DraftParts<'_>,
        file: FileId,
        entry: &Entry,
        (key, label, value): (&'static str, &str, i64),
    ) {
        match creature::use_vital(draft, entry, key, value) {
            Ok(after) => {
                kept.set_vital((
                    file,
                    draft.as_str().to_owned(),
                    crate::design::memo::Revised::new(after.clone()),
                    crate::sentence!("The stat block now says {} {}.", label, value),
                ));
                let _ = draft.replace(after);
                // The draft changed: a failed Save of the old words no longer owns it.
                *revision += 1;
            }
            Err(failure) => *error = Some(failure.into()),
        }
    }

    /// The stat fix's result words while its Undo shows: the open file is the fix's own file
    /// (owner kind, owner id, path) and the draft is the text the fix wrote.
    fn stat_result_for(&self, file: &FileId) -> Option<Sentence> {
        self.kept
            .vital()
            .filter(|(at, _, after, _)| {
                at.0 == file.0
                    && at.1 == file.1
                    && self
                        .draft
                        .same_as(after.revision(), after, &self.same_vital)
            })
            .map(|(_, _, _, result)| result.clone())
    }

    /// Whether a press of `Undo stat change` acts now, in the frame of the press: the open file
    /// is the fix's own file and the draft is still the text the fix wrote (typed text is never
    /// replaced).
    fn stat_undo_applies(&self, file: &FileId) -> bool {
        self.kept.vital().is_some_and(|(at, _, after, _)| {
            at.0 == file.0
                && at.1 == file.1
                && self
                    .draft
                    .same_as(after.revision(), after, &self.same_vital)
        })
    }

    /// Whether a press of `Undo heading change` acts now: as `stat_undo_applies`, for the
    /// heading fix.
    fn heading_undo_applies(&self, file: &FileId) -> bool {
        self.kept.heading().is_some_and(|(at, _, after)| {
            at.0 == file.0
                && at.1 == file.1
                && self
                    .draft
                    .same_as(after.revision(), after, &self.same_heading)
        })
    }

    /// The footer order of the open entry's link result: the order its own update issued
    /// (review of PR 298), for the check that proves it.
    #[cfg(any(test, feature = "test-support"))]
    pub fn open_link_result_order(&self) -> Option<design::ResultOrder> {
        let owner = self.open_link_owner()?;
        self.link_edits
            .get(&owner)
            .map(|shown| shown.order_for_caller_test())
    }

    /// The open entry as the owner of its link offer and link result.
    fn open_link_owner(&self) -> Option<LinkOwner> {
        let (id, root) = self.open_entry_key()?;
        Some(LinkOwner { id, root })
    }

    pub fn refocus_draft(&mut self) {
        self.focus_markdown = true;
    }
    fn register_copy(&mut self, campaigns: &mut Campaigns) -> Result<(), String> {
        let source = self
            .copied_package
            .clone()
            .ok_or("Choose the copied folder first.")?;
        if campaigns
            .modules()
            .iter()
            .any(|m| m.path.starts_with(&source.path) || source.path.starts_with(&m.path))
        {
            return Err("Choose a separate copied folder first.".into());
        }
        // Point 2 of the review: the campaign and the copy's choices in it are recorded also
        // when the registration writes no campaign file, so a later use change shows.
        let campaign = campaigns.active_campaign();
        let campaign_document = if !self.copy_not_used {
            campaign.as_ref().map(CampaignDocument::load).transpose()?
        } else {
            None
        };
        let copy = crate::content_workspace::PackageCopy::write(&source.path, &self.copy_name)?;
        if let Err(error) = campaigns.register_module(copy.module.clone()) {
            copy.undo()?;
            return Err(error);
        }
        let mut written = None;
        let mut choices = Vec::new();
        if let Some(mut document) = campaign_document {
            let before = document.text.clone();
            document
                .selections
                .push(crate::content_workspace::PackageSelection {
                    name: Some(copy.module.name.clone()),
                    package: crate::content_workspace::EntryRef::id(copy.module.id),
                    selection: crate::content_workspace::Selection::All,
                });
            if let Err(error) = document.save() {
                let unregister = campaigns.unregister_module(copy.module.id);
                let restore = copy.undo();
                return Err(format!(
                    "Could not use the copy: {error}. {} {}",
                    unregister.err().unwrap_or_default(),
                    restore.err().unwrap_or_default()
                ));
            }
            choices = copy_choices(&document, copy.module.id, &package_manifests(campaigns));
            written = Some((document.path, before, document.text));
        }
        // One result per copied folder in a campaign: a registration of the same folder again
        // (after an unregister that no reload has seen yet) ends the older one by the store's
        // owner event, so its Undo cannot reach the newer registration's files.
        let scope = campaign.as_ref().map(|campaign| campaign.id);
        let older: Vec<RegistrationOwner> = self
            .registrations
            .newest_first(&scope)
            .filter(|shown| shown.value().copy.module.path == copy.module.path)
            .map(|shown| design::results::Kept::owner(shown.value()))
            .collect();
        for owner in &older {
            self.registrations.owner_changed(owner);
            self.rosters.owner_changed(owner);
        }
        self.registrations.record(Registration {
            copy,
            campaign,
            written,
            choices,
            clause: true,
            blocked: false,
            used_by: None,
        });
        self.copied_package = None;
        self.copy_name_open = false;
        self.form = false;
        self.stale = true;
        self.scope = Scope::All;
        self.content_changed = Some(source.path.clone());
        Ok(())
    }

    /// `undo_package_registration` by the ticket of the line pressed (M-L0.4b): a registration
    /// replaced or ended since its line was drawn is gone, and nothing changes.
    fn undo_registration_by(
        &mut self,
        ticket: Ticket,
        campaigns: &mut Campaigns,
    ) -> Result<(), String> {
        let Some(owner) = self
            .registrations
            .get_by(&ticket)
            .map(|shown| design::results::Kept::owner(shown.value()))
        else {
            return Ok(());
        };
        let manifests = package_manifests(campaigns);
        Self::registration_in_use(&owner, campaigns, &manifests)?;
        let rosters = &self.rosters;
        let registered = match self.registrations.undo_by(ticket, |registered| {
            Self::registration_undo(registered, rosters, &owner, &manifests, campaigns)
        }) {
            Ok((registered, ())) => registered,
            Err(ByTicket::Gone) => return Ok(()),
            Err(ByTicket::Failed(error)) => return Err(error),
        };
        self.registration_undone(&owner, &registered, &manifests);
        Ok(())
    }

    /// Point 1 of the review: while another campaign chose the copy, Undo is refused; its
    /// choices there would name identities that Undo removes. Checked again at the press, as
    /// well as by the reconcile event that locks the Undo.
    fn registration_in_use(
        owner: &RegistrationOwner,
        campaigns: &Campaigns,
        manifests: &[Entry],
    ) -> Result<(), String> {
        match used_elsewhere(owner.package, owner.campaign, campaigns.all(), manifests) {
            Some(name) => Err(used_reason(&name).as_str().to_owned()),
            None => Ok(()),
        }
    }

    /// The registration's Undo itself: one checked write of the copy, the campaign file and the
    /// source change, then the unregister; a failed unregister puts the files back.
    fn registration_undo(
        registered: &Registration,
        rosters: &design::Results<RegistrationOwner, RostersUse>,
        owner: &RegistrationOwner,
        manifests: &[Entry],
        campaigns: &mut Campaigns,
    ) -> Result<(), String> {
        let mut extra: Vec<_> = registered.campaign_undo(manifests)?.into_iter().collect();
        if let Some(source) = rosters.get(owner) {
            extra.extend(source.value().edits.inverse_files());
        }
        let edits = registered.copy.undo_with(extra)?;
        if let Err(error) = campaigns.unregister_module(registered.copy.module.id) {
            return Err(match edits.undo() {
                Ok(()) => error,
                Err(restore) => format!("{error}. Could not restore the copy: {restore}"),
            });
        }
        Ok(())
    }

    /// The registration's Undo worked. The source change it reversed ends with it; so does the
    /// result of choosing its entries, whose data the Undo also reversed.
    fn registration_undone(
        &mut self,
        owner: &RegistrationOwner,
        registered: &Registration,
        manifests: &[Entry],
    ) {
        self.rosters.owner_changed(owner);
        let package = registered.copy.module.id;
        if self.kept.package_use().is_some_and(|undo| {
            registered.campaign.as_ref().map(|c| c.id) == Some(undo.campaign.id)
                && undo
                    .after
                    .iter()
                    .any(|selection| chosen_package(&selection.package, manifests) == Some(package))
        }) {
            let _ = self.kept.take_package_use();
        }
        self.content_changed = Some(registered.copy.module.path.clone());
        self.stale = true;
    }

    /// The registration whose result the page shows: the newest of the open campaign. Another
    /// campaign's stays in the store and shows when that campaign is open again.
    fn shown_registration(&self, campaigns: &Campaigns) -> Option<RegistrationOwner> {
        let scope = campaigns.active_campaign().map(|campaign| campaign.id);
        self.registrations
            .newest_first(&scope)
            .next()
            .map(|shown| design::results::Kept::owner(shown.value()))
    }

    /// The registration results at a reload, the one event that every write and every outside
    /// change reaches: a result whose copy is no longer registered ends (with its source
    /// change), and each one left checks its clause and its Undo (`Registration::reconcile`).
    /// It reads the campaign file and the copied files, so it runs once per reload, never per
    /// frame.
    fn reconcile_registrations(&mut self, campaigns: &Campaigns) {
        #[cfg(any(test, feature = "test-support"))]
        {
            self.reconciles += 1;
        }
        let registered = |package: Uuid| campaigns.modules().iter().any(|m| m.id == package);
        self.registrations
            .disk_changed(|registration| registered(registration.copy.module.id));
        let registrations = &self.registrations;
        self.rosters
            .disk_changed(|source| registrations.get(&source.owner).is_some());
        self.rosters.reconcile(&());
        let scopes: Vec<Option<Uuid>> = std::iter::once(None)
            .chain(campaigns.all().iter().map(|campaign| Some(campaign.id)))
            .collect();
        let check = RegistrationCheck {
            sources: scopes
                .iter()
                .flat_map(|scope| self.rosters.newest_first(scope))
                .map(|shown| (shown.value().owner, shown.value().blocked))
                .collect(),
            manifests: package_manifests(campaigns),
            campaigns: campaigns.all().to_vec(),
        };
        self.registrations.reconcile(&check);
    }

    /// On reload, each result with an Undo checks that its files still hold what its action
    /// wrote. If any file differs, Undo would only refuse: the result and its Undo go. A message
    /// with another action stays and blocks its Undo instead. Call this once per reload, not every
    /// frame: it reads the files.
    fn forget_changed_undos(&mut self) {
        use crate::content_workspace::{CampaignDocument, holds};
        self.kept.forget_manifest_if_changed();
        if self
            .kept
            .identities()
            .is_some_and(|(edits, _)| !edits.on_disk())
        {
            let _ = self.kept.take_identities();
        }
        if self
            .kept
            .identity()
            .is_some_and(|undo| !holds(&undo.root, &undo.path, &undo.after))
        {
            let _ = self.kept.take_identity();
        }
        self.kept.retain_copies(|undo| {
            holds(&undo.root, &undo.path, &undo.after)
                && holds(&undo.root, &undo.copied_path, &undo.copied_text)
        });
        // Its Undo checks the package choices only, so other campaign edits keep it. It ends
        // only when a read proves the choices differ: a campaign file that cannot be read now
        // keeps it, as its failed Undo does (note §4), so the DM can try again once it reads.
        if self.kept.package_use().is_some_and(|undo| {
            CampaignDocument::load(&undo.campaign)
                .is_ok_and(|document| document.selections != undo.after)
        }) {
            let _ = self.kept.take_package_use();
        }
        self.link_edits
            .disk_changed(|result| result.edits.on_disk());
    }

    /// A new entry made in a package is chosen for the open campaign, unless the DM unticked
    /// it. `Err`: why it could not be, in the app's words with the cause after them.
    fn choose_new_entry(
        &self,
        owner: &Owner,
        entry: Uuid,
        campaigns: &Campaigns,
    ) -> Result<(), PageError> {
        let (false, Owner::Package(module), Some(campaign)) =
            (self.new_not_used, owner, campaigns.active_campaign())
        else {
            return Ok(());
        };
        crate::content_workspace::use_in_campaign(&campaign, module.id, &module.name, Some(entry))
            .map_err(|cause| PageError::because(&ENTRY_NOT_CHOSEN, cause))
    }

    /// The manifest repair: `Package.md` written again for `module`. Its result, with Undo, is
    /// the newest kept result.
    fn write_manifest(&mut self, module: Module, path: &str, exists: bool) {
        match crate::content_workspace::ManifestWrite::write_as(&module, path, exists) {
            Ok(write) => {
                self.kept.set_manifest(write);
                self.stale = true;
                self.content_changed = Some(module.path);
            }
            Err(error) => self.error = Some(error.into()),
        }
    }

    /// `Choose all` on a package About page: every entry of `module` is chosen for the open
    /// campaign. Its result, with Undo, is the newest kept result.
    fn choose_all(&mut self, module: &Module, campaigns: &Campaigns) {
        let Some(campaign) = campaigns.active_campaign() else {
            return;
        };
        let result = CampaignDocument::load(&campaign).and_then(|before| {
            crate::content_workspace::use_in_campaign(&campaign, module.id, &module.name, None)?;
            let after = CampaignDocument::load(&campaign)?;
            Ok(SelectionUndo {
                campaign: campaign.clone(),
                before: before.selections,
                after: after.selections,
            })
        });
        match result {
            Ok(undo) => {
                self.kept.set_package_use(undo);
                self.stale = true;
                self.reconcile_registrations(campaigns);
                self.content_changed = Some(campaign.path);
            }
            Err(error) => self.error = Some(error.into()),
        }
    }

    /// A folder's batch repair: each of `paths` in `root` gets a new identity. Its result, with
    /// Undo, is the newest kept result.
    fn separate_entries(&mut self, root: PathBuf, paths: &[String]) -> Result<(), String> {
        let edits = crate::content_workspace::ContentEdits::new_identities(&root, paths)?;
        let folder = root.file_name().unwrap_or_default().to_string_lossy();
        self.kept.set_identities((
            edits,
            crate::sentence!(
                "{} in {}/ are now separate entries.",
                design::count(paths.len(), "file", "files"),
                folder
            ),
        ));
        self.stale = true;
        self.content_changed = Some(root);
        Ok(())
    }

    /// The editor's Discard of `note`'s draft: the draft goes back to the file's words, and the
    /// words it dropped are the newest kept result, with Undo. It takes the parts it changes, so
    /// the editor can call it while it reads the open note.
    fn discard_draft((kept, draft, revision, error): DraftParts<'_>, note: &Note) {
        kept.set_discarded((
            note.owner.key(),
            note.path.clone(),
            draft.as_str().to_owned(),
        ));
        let _ = draft.replace(note.text.as_str());
        // The draft changed: a failed Save of the old words no longer owns it.
        *revision += 1;
        *error = None;
    }

    /// The copy of a campaign entry that the package entry `note` links to (`source`) into the
    /// package, and the link moved to the copy: one result per copy, with Undo, the newest kept
    /// result. The package's folder, which changed; `None` when `note` is not a package's.
    fn copy_dependency(
        kept: &mut LibraryKept,
        note: &Note,
        source: &Note,
        campaigns: &Campaigns,
    ) -> Result<Option<PathBuf>, String> {
        let Owner::Package(package) = &note.owner else {
            return Ok(None);
        };
        let source = crate::content_workspace::CopySource {
            entry: source
                .entry
                .clone()
                .ok_or("The linked entry cannot be read.")?,
            text: source.text.to_string(),
            root: source.owner.root().map(Path::to_path_buf),
            campaign: None,
        };
        let catalogue = crate::content_workspace::Catalogue::load(
            campaigns.modules(),
            campaigns.active_campaign().as_ref(),
        )?;
        let plan = crate::content_workspace::CopyPlan::prepare(
            &source,
            &package.path,
            None,
            &source.entry.name,
            "",
            &catalogue,
        )?;
        let copied = plan.save()?;
        let after =
            crate::content_workspace::redirect_reference(&note.text, &source.entry, copied.id)?;
        if let Err(error) = write_note(&package.path, &note.path, Some(&note.text), &after) {
            let _ = fs::remove_file(package.path.join(&plan.relative));
            return Err(error);
        }
        kept.push_copy(DependencyUndo {
            root: package.path.clone(),
            path: note.path.clone(),
            before: note.text.to_string(),
            after,
            copied_path: plan.relative.clone(),
            copied_text: plan.text.clone(),
            name: source.entry.name.clone(),
            package: package.name.clone(),
        });
        Ok(Some(package.path.clone()))
    }

    fn give_new_identity(&mut self) -> Result<(), String> {
        let index = self.selected.ok_or("Open the copy first.")?;
        let note = &self.loaded.as_ref().unwrap().notes[index];
        note.entry.as_ref().ok_or("Repair this entry first.")?;
        let root = note
            .owner
            .root()
            .ok_or("Built-in entries are read only.")?
            .to_path_buf();
        let before = note.text.to_string();
        let id = crate::hermetic::new_uuid();
        let after = crate::content_workspace::new_note_identity(&before, &note.path, id)?;
        let updated = write_note(&root, &note.path, Some(&before), &after)?;
        self.kept.set_identity(IdentityUndo {
            root: root.clone(),
            path: note.path.clone(),
            before,
            after: after.clone(),
        });
        let loaded = self.loaded.as_mut().unwrap();
        loaded.notes[index].entry = Some(updated);
        loaded.notes[index].text = crate::design::memo::Revised::new(after.clone());
        loaded.invalidate();
        loaded.refresh_diagnostics();
        let _ = self.draft.replace(after);
        // The draft changed: a failed Save of the old words no longer owns it.
        self.draft_revision += 1;
        self.stale = true;
        self.content_changed = Some(root);
        Ok(())
    }

    fn undo_new_identity(&mut self) -> Result<(), String> {
        let undo = self.kept.identity().ok_or("Nothing to undo.")?;
        write_note(&undo.root, &undo.path, Some(&undo.after), &undo.before)?;
        self.stale = true;
        self.content_changed = Some(undo.root.clone());
        if let Some(index) = self.selected {
            let note = &mut self.loaded.as_mut().unwrap().notes[index];
            if note.path == undo.path && note.owner.root() == Some(undo.root.as_path()) {
                note.text = crate::design::memo::Revised::new(undo.before.clone());
                note.entry = Entry::parse(&note.path, &note.text).ok();
                let _ = self.draft.replace(note.text.as_str());
                // The draft changed: a failed Save of the old words no longer owns it.
                self.draft_revision += 1;
            }
        }
        let _ = self.kept.take_identity();
        self.loaded.as_mut().unwrap().invalidate();
        self.loaded.as_mut().unwrap().refresh_diagnostics();
        Ok(())
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn repairing_package_about(&self) -> bool {
        self.selected
            .and_then(|at| self.loaded.as_ref()?.notes.get(at))
            .is_some_and(|note| {
                note.is_manifest()
                    && note.error.is_some()
                    && self.source
                    && self.scope == Scope::Package(note.owner.id().unwrap())
                    && self.section == Section::All
            })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn selected_name(&self) -> Option<&str> {
        self.selected
            .and_then(|i| self.loaded.as_ref()?.notes.get(i))
            .map(Note::name)
    }

    pub fn report_error(&mut self, error: String) {
        self.error = Some(error.into());
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    /// The DM chose the file over the draft. The draft stays one step of undo away.
    fn reload_note_from_disk(&mut self, index: usize) {
        let Some(change) = self.disk_change.take() else {
            return;
        };
        // Whether this campaign uses the entry now, and whether the DM chose it, for Undo
        // remove to give back.
        let used = self.loaded.as_ref().is_some_and(|loaded| {
            loaded
                .notes
                .get(index)
                .is_some_and(|note| loaded.in_scope(note, Scope::Uses))
        });
        let chosen = self.loaded.as_ref().is_some_and(|loaded| {
            loaded
                .notes
                .get(index)
                .is_some_and(|note| loaded.chose(note))
        });
        let Some(note) = self.loaded.as_mut().and_then(|l| l.notes.get_mut(index)) else {
            return;
        };
        if let Some(path) = change.moved_to {
            note.path = path;
        }
        let draft = self.draft.replace(String::new());
        self.draft_revision += 1;
        self.kept
            .set_discarded((note.owner.key(), note.path.clone(), draft));
        if let Some(fault) = change.fault {
            // The note becomes the file as it is: unreadable, with its fault and no text.
            note.entry = None;
            note.error = Some(fault.cause().to_owned());
            note.fault = Some(fault);
            note.text = crate::design::memo::Revised::default();
            self.stale = true;
            if let Some(loaded) = &mut self.loaded {
                loaded.invalidate();
            }
            self.matches = None;
            self.scope_counts = None;
            return;
        }
        match change.text {
            Some(text) => {
                self.stale = true;
                match Entry::parse(&note.path, &text) {
                    Ok(entry) => {
                        note.entry = Some(entry);
                        note.error = None;
                    }
                    Err(error) => {
                        note.entry = None;
                        note.error = Some(error);
                    }
                }
                note.text = crate::design::memo::Revised::new(text.clone());
                let _ = self.draft.replace(text);
            }
            None => {
                self.kept.set_removed(LetGo {
                    note: Note {
                        path: note.path.clone(),
                        text: note.text.clone(),
                        entry: note.entry.clone(),
                        error: note.error.clone(),
                        fault: note.fault.clone(),
                        manifest_gone: note.manifest_gone,
                        owner: note.owner.clone(),
                    },
                    used,
                    chosen,
                    campaign: None,
                });
                let _ = self.draft.replace(note.text.as_str());
                self.stale = true;
                self.selected = None;
            }
        }
        if let Some(loaded) = &mut self.loaded {
            loaded.invalidate();
        }
        self.matches = None;
        self.scope_counts = None;
    }

    fn open_entry(&mut self, index: usize) {
        if self.dirty() {
            return;
        }
        let _ = self.kept.take_removed();
        let Some(note) = self.loaded.as_ref().and_then(|l| l.notes.get(index)) else {
            return;
        };
        if self.selected != Some(index) {
            self.trail.leave(self.here());
        }
        if !note.is_manifest() && !self.loaded.as_ref().unwrap().in_scope(note, self.scope) {
            self.scope = Scope::All;
            self.kept
                .set_scope_notice((note.name().to_owned(), Scope::All));
        }
        self.selected = Some(index);
        let _ = self.draft.replace(note.text.as_str());
        self.disk_change = None;
        self.section = if note.is_manifest() {
            Section::All
        } else {
            note.section()
        };
        self.search.clear();
        self.matches = None;
        self.source = false;
        self.error = None;
    }

    /// The open draft's entry name and owner, if it has changes.
    pub fn draft_place(&self) -> Option<(String, String)> {
        let note = self
            .loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .filter(|n| !self.saved_is_draft(n))?;
        Some((
            note.name().to_owned(),
            match &note.owner {
                Owner::Package(module) => format!("the {} package", module.name),
                Owner::Campaign(campaign) => format!("the {} campaign", campaign.name),
                Owner::Builtin => "the built-in package".to_owned(),
            },
        ))
    }
    /// Whether the open draft's words are note `note`'s saved text: compared once per pair of
    /// revisions (layout PR 9b-C).
    fn saved_is_draft(&self, note: &Note) -> bool {
        self.draft
            .same_as(note.text.revision(), &note.text, &self.same_saved)
    }

    pub fn dirty(&self) -> bool {
        self.loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .is_some_and(|n| !self.saved_is_draft(n))
    }
    pub fn form_open(&self) -> bool {
        self.form || self.new_form || self.confirm_unregister.is_some()
    }
    /// A draft in the campaign's own folder must block a campaign switch.
    pub fn campaign_draft(&self) -> bool {
        self.dirty()
            && self
                .loaded
                .as_ref()
                .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
                .is_some_and(|n| matches!(n.owner, Owner::Campaign(_)))
    }
    /// Remove's Undo (and the draft it keeps) belongs to the removed entry in the campaign that was
    /// open. The app tells the Library the open campaign on every frame, whichever page shows, so
    /// the Undo ends at the switch itself, even while the Library is hidden: a switch there and
    /// back never brings it back.
    pub fn note_campaign(&mut self, campaign: Option<Uuid>) {
        self.kept.note_removed_campaign(campaign);
    }

    /// Remove's Undo also ends when any entry is selected, however the selection was made: the
    /// list, the About row, `reveal` (Lookup and other pages), Back or Forward, or `open_entry`.
    /// Its own Undo selects the entry it gives back after taking it.
    fn end_remove_on_selection(&mut self) {
        if self.kept.removed().is_some() && self.selected.is_some() {
            let _ = self.kept.take_removed();
            let _ = self.kept.take_discarded();
        }
    }

    /// Show an entry where it lives. False if a draft is open: it stays in Lookup.
    pub fn reveal(&mut self, entry: &Entry) -> bool {
        if self.dirty() {
            return false;
        }
        self.reveal_row = true;
        let Some((index, in_scope)) = self.loaded.as_ref().and_then(|l| {
            let index = l
                .notes
                .iter()
                .position(|n| n.entry.as_ref().is_some_and(|e| e.id == entry.id))?;
            Some((index, l.in_scope(&l.notes[index], self.scope)))
        }) else {
            return false;
        };
        // Round 26 review: every way to open another entry leaves the open one on the trail,
        // before the scope, the search or the selection changes.
        if self.selected != Some(index) {
            self.trail.leave(self.here());
        }
        if !in_scope {
            self.scope = Scope::All;
        }
        self.search.clear();
        self.section = Section::for_entry(entry);
        self.source = false;
        self.matches = None;
        self.selected = Some(index);
        let _ = self
            .draft
            .replace(self.loaded.as_ref().unwrap().notes[index].text.as_str());
        true
    }
    pub fn edit_copy(&mut self, entry: Entry) {
        self.pending_edit = Some(entry);
    }
    /// Open a file for repair; with a missing link's reference and words, at that link.
    /// The repair guidance has one owner, its entry. When no load and no pending repair is in
    /// progress and the selected entry is not the owner, the guidance ends, however the
    /// selection was made: the list, the keyboard, search, `reveal`, Back or Forward, a link, or
    /// a repair whose file could not be opened.
    fn end_repair_on_owner_change(&mut self) {
        if self.pending.is_some() || self.stale || self.pending_repair.is_some() {
            return;
        }
        let here = self
            .selected
            .and_then(|i| self.loaded.as_ref()?.notes.get(i))
            .and_then(|note| Some(note.owner.root()?.join(&note.path)));
        if self
            .repair_link
            .as_ref()
            .is_some_and(|link| here.as_ref() != Some(&link.path))
        {
            self.repair_link = None;
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn repair_lookups(&self) -> usize {
        REPAIR_LOOKUPS.with(std::cell::Cell::get)
    }

    /// The broken-link scans: an unchanged frame does none.
    #[cfg(any(test, feature = "test-support"))]
    pub fn broken_line_scans(&self) -> usize {
        BROKEN_LINE_SCANS.with(std::cell::Cell::get)
    }
    pub fn repair_file_link(
        &mut self,
        path: PathBuf,
        link: Option<(splat::content::Reference, String)>,
    ) {
        self.repair_link = link.map(|(target, words)| RepairLink {
            path: path.clone(),
            target,
            words,
            found: design::memo::Memo::new(),
        });
        self.pending_repair = Some(path);
        self.stale = true;
    }
    /// The close dialog's Save is offered only when Save would accept the draft: its own check
    /// before the write, not only a parse.
    pub fn can_save_and_close(&self) -> bool {
        self.dirty()
            && self.disk_change.is_none()
            && self
                .selected
                .zip(self.loaded.as_ref())
                .is_some_and(|(i, loaded)| {
                    let note = &loaded.notes[i];
                    loaded
                        .draft_refusal(i, &Entry::parse(&note.path, &self.draft))
                        .is_none()
                })
    }
    pub fn save_for_close(&mut self) -> bool {
        if !self.can_save_and_close() {
            return false;
        }
        self.save_selected(false);
        !self.dirty()
    }

    pub fn refresh_destination(
        &mut self,
        root: &Path,
        _ctx: &egui::Context,
        campaigns: &Campaigns,
    ) {
        // Point 3 of the review: the registration results follow the write at its event,
        // even while a draft holds the reload back.
        self.reconcile_registrations(campaigns);
        if self.dirty()
            && let Some(note) = self
                .selected
                .and_then(|i| self.loaded.as_ref()?.notes.get(i))
            && note.owner.root() == Some(root)
        {
            match read_disk(root, &note.path) {
                Err(fault) => {
                    self.disk_change = Some(DiskChange {
                        text: None,
                        moved_to: None,
                        fault: Some(fault),
                    });
                }
                Ok(text) if text.as_deref() != Some(&note.text) => {
                    let moved = (text.is_none()
                        && self
                            .loaded
                            .as_ref()
                            .unwrap()
                            .duplicates(self.selected.unwrap())
                            .is_empty())
                    .then(|| {
                        crate::content_workspace::note_id(&note.text)
                            .and_then(|id| moved_note(root, id, &note.path).ok().flatten())
                    })
                    .flatten();
                    self.disk_change = Some(DiskChange {
                        text: moved.as_ref().map(|(_, text)| text.clone()).or(text),
                        moved_to: moved.map(|(path, _)| path),
                        fault: None,
                    });
                }
                // The file reads as the draft's start again: a fault it had is over.
                Ok(_) => {
                    if self
                        .disk_change
                        .as_ref()
                        .is_some_and(|change| change.fault.is_some())
                    {
                        self.disk_change = None;
                    }
                }
            }
        }
        // Discard's and Remove's Undo belong to one file. A refresh ends them only when it is
        // about that file: the refreshed folder is its owner's, and the file on disk is no longer
        // what the Undo was made against (another write, or the deleted file is back). A write to
        // an unrelated entry, such as choosing a creature for the campaign, keeps them.
        let changed_here = |owner: &Owner, path: &str, expected: Option<&str>| {
            owner.root() == Some(root)
                && read_disk(root, path).map_or(true, |text| text.as_deref() != expected)
        };
        let gone_changed = self
            .kept
            .removed()
            .is_some_and(|LetGo { note, .. }| changed_here(&note.owner, &note.path, None));
        if gone_changed {
            let _ = self.kept.take_removed();
        }
        let discard_changed = self.kept.discarded().is_some_and(|(owner, path, _)| {
            let loaded = self.loaded.as_ref().and_then(|loaded| {
                loaded
                    .notes
                    .iter()
                    .find(|note| note.owner.key() == *owner && &note.path == path)
            });
            match (loaded, self.kept.removed()) {
                (Some(note), _) => changed_here(&note.owner, path, Some(&note.text)),
                (None, Some(LetGo { note, .. })) if &note.path == path => false,
                // Its file is neither loaded nor removed by this Library: nothing owns it.
                (None, _) => true,
            }
        });
        if discard_changed || gone_changed {
            let _ = self.kept.take_discarded();
        }
        self.stale = true;
    }
    /// The open entry's file, whole (owner kind, owner id, path): what a reload selects again.
    fn open_file_id(&self) -> Option<FileId> {
        self.loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .map(|n| (n.owner.key(), n.path.clone()))
    }
    fn invalidate(&mut self) {
        self.matches = None;
        self.scope_counts = None;
    }
    fn start_load(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        self.reselect = self.open_file_id();
        self.load_key = Some(load_key(campaigns));
        let modules = campaigns.modules().to_vec();
        let campaign = campaigns.active_campaign();
        let ctx = ctx.clone();
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.loaded = None;
        self.invalidate();
        self.selected = None;
        self.error = None;
        std::thread::spawn(move || {
            let _ = tx.send(load_all(&modules, campaign.as_ref()));
            ctx.request_repaint();
        });
    }
    /// The draft of note `i`, parsed. The same revision of the draft keeps the same parse (layout
    /// PR 9b: a quiet frame compares no words).
    fn parsed_draft(&mut self, i: usize) -> ParsedDraft {
        let path = &self.loaded.as_ref().unwrap().notes[i].path;
        if let Some((parsed_path, draft, parsed)) = &self.draft_parse
            && parsed_path == path
            && *draft == self.draft.key()
        {
            return parsed.clone();
        }
        let parsed =
            design::memo::Revised::new(std::sync::Arc::new(Entry::parse(path, &self.draft)));
        self.draft_parse = Some((path.clone(), self.draft.key(), parsed.clone()));
        parsed
    }

    /// Why Manage packages' forms wait, as the form's own reasons (`draft_reason` and loading).
    fn package_lock(&self) -> Option<PackageReason> {
        if self.disk_change.is_some() {
            Some(PackageReason::ChangedFile)
        } else if self.dirty() {
            let campaign = self
                .selected
                .and_then(|i| self.loaded.as_ref().map(|l| &l.notes[i].owner))
                .is_some_and(|owner| matches!(owner, Owner::Campaign(_)));
            Some(if campaign {
                PackageReason::CampaignDraft
            } else {
                PackageReason::Draft
            })
        } else {
            self.pending.is_some().then_some(PackageReason::Loading)
        }
    }

    /// The section rows' column, on a wide page with no disk change. M0333: the section rows
    /// keep the scroll column's lane, so the column is that much wider and the rows keep their
    /// room.
    fn sections_column(&self, compact: bool) -> Option<design::Column> {
        (!compact && self.disk_change.is_none()).then(|| {
            let lane = design::lane_width();
            let (width, room) = (166.0 + lane, 150.0 + lane..=280.0 + lane);
            design::Column::left("library_sections", width, room)
                // The drag handle is on this edge, so the rows stop short of it.
                .frame(egui::Frame::new().inner_margin(egui::Margin {
                    right: design::S as i8,
                    ..Default::default()
                }))
        })
    }

    /// The entry list's column in a room `width` wide, or none: beside Lookup at the minimum
    /// window, a draft needs the room more than the list. A wide window's list gets room too,
    /// so that long names keep their second line.
    fn entries_column(&self, ui: &egui::Ui, width: f32, dirty: bool) -> Option<design::Column> {
        if self.disk_change.is_some() || (width < 480.0 && (dirty || self.source)) {
            return None;
        }
        let min_list_width =
            (design::number_width(ui, "Can use", &Type::Strong.font(ui)) + design::XXL + design::L)
                * 2.0
                + design::S * 2.0
                + ui.spacing().item_spacing.x;
        let list_width = (width * 0.38).clamp(min_list_width, 420.0);
        Some(
            design::Column::left("library_entries", list_width, min_list_width..=560.0).frame(
                egui::Frame::new().inner_margin(egui::Margin {
                    left: design::S as i8,
                    right: design::S as i8,
                    top: 0,
                    bottom: 0,
                }),
            ),
        )
    }

    /// The page's columns that `ui` lays out next, left to right: the section rows and the
    /// entry list when they show, then the reader. The header asks the columns for their widths
    /// before they draw (`Column::width_within`), so each menu of the header keeps inside the
    /// column under its control, clear of its splitters and the list's scroll bar (issue
    /// #504). Each column is its content: `S` inside its panel's edges, where its frame's
    /// margin ends and its scroll bar starts. With no columns (a load, an empty scope) the page
    /// is one column.
    fn page_columns(
        &self,
        ui: &egui::Ui,
        compact: bool,
        dirty: bool,
        total: usize,
    ) -> Vec<egui::Rect> {
        let mut room = ui.available_rect_before_wrap();
        let mut columns = Vec::new();
        if self.pending.is_none()
            && self.loaded.is_some()
            && !(self.scope == Scope::Campaign && total == 0)
        {
            let mut split = |room: &mut egui::Rect, width: f32| {
                columns.push(egui::Rect::from_x_y_ranges(
                    room.left()..=room.left() + width,
                    room.y_range(),
                ));
                room.min.x += width;
            };
            if let Some(column) = self.sections_column(compact) {
                let width = column.width_within(ui.ctx(), room);
                split(&mut room, width);
            }
            if let Some(column) = self.entries_column(ui, room.width(), dirty) {
                let width = column.width_within(ui.ctx(), room);
                split(&mut room, width);
            }
        }
        columns.push(room);
        columns
            .into_iter()
            .map(|column| column.shrink2(egui::vec2(design::S, 0.0)))
            .collect()
    }

    fn draft_reason(&self) -> Option<&'static str> {
        if self.disk_change.is_some() {
            Some(CHANGED_FILE_FIRST.as_str())
        } else if self.dirty() {
            Some(self.draft_blocks().long())
        } else {
            None
        }
    }

    /// What the open entry's draft blocks: the part holds the words of its reason.
    fn draft_blocks(&self) -> design::DraftBlocks {
        let campaign = self
            .selected
            .and_then(|i| self.loaded.as_ref().map(|l| &l.notes[i].owner))
            .is_some_and(|owner| matches!(owner, Owner::Campaign(_)));
        if campaign {
            design::DraftBlocks::EntryInCampaign
        } else {
            design::DraftBlocks::EntryInPackage
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &mut Campaigns) {
        // The New entry form's error ends with the form, however it closed.
        if !self.new_form {
            self.new_error = None;
        }
        self.end_repair_on_owner_change();
        self.end_remove_on_selection();
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("reader_surface"), egui::Id::new("library"))
        });
        let key = load_key(campaigns);
        if self.load_key.is_none() && self.loaded.is_some() {
            self.load_key = Some(key.clone());
        }
        let changed = self.load_key.as_ref().is_some_and(|known| *known != key);
        if self.loaded.is_none() && self.pending.is_none() && self.error.is_none() {
            self.start_load(ui.ctx(), campaigns);
        } else if (changed || self.stale) && !self.dirty() && self.pending.is_none() {
            self.stale = false;
            self.start_load(ui.ctx(), campaigns);
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(l) => {
                            for module in campaigns.modules().to_vec() {
                                if let Some(name) = l
                                    .notes
                                    .iter()
                                    .filter(|n| n.owner.is_package(module.id) && n.error.is_none())
                                    .find_map(|n| {
                                        n.entry
                                            .as_ref()
                                            .filter(|entry| entry.kind == "package")
                                            .map(|entry| entry.name.clone())
                                    })
                                    && name != module.name
                                {
                                    let _ = campaigns.rename_module(module.id, name);
                                }
                            }
                            self.selected = self
                                .reselect
                                .as_ref()
                                .and_then(|(owner, path)| file_at(&l.notes, (*owner, path)));
                            if let Some(i) = self.selected {
                                let _ = self.draft.replace(l.notes[i].text.as_str());
                                // The draft changed: a failed Save of the old words no longer owns it.
                                self.draft_revision += 1;
                            }
                            self.reveal_row |= self.selected.is_some();
                            self.focus_repair =
                                self.selected.is_some_and(|i| l.notes[i].error.is_some());
                            self.forget_changed_undos();
                            self.reconcile_registrations(campaigns);
                            // An offer whose entry changed outside Splat, or whose names now name
                            // another entry, goes; it is checked again before any write too.
                            self.renamed_links
                                .disk_changed(|offer| l.offer_holds(offer));
                            self.loaded = Some(l);
                            self.invalidate();
                        }
                        Err(e) => self.error = Some(e.into()),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some(PageError::Own(LOADING_STOPPED.clone()));
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if self.pending.is_none()
            && !self.stale
            && let Some(entry) = self.pending_edit.take()
        {
            if self.reveal(&entry) {
                self.source = true;
            } else {
                self.pending_edit = Some(entry);
            }
        }
        if self.pending.is_none()
            && !self.stale
            && !self.dirty()
            && let Some(path) = self.pending_repair.take()
            && let Some(index) = self.loaded.as_ref().and_then(|loaded| {
                loaded.notes.iter().position(|note| {
                    note.owner
                        .root()
                        .is_some_and(|root| root.join(&note.path) == path)
                })
            })
        {
            let note = &self.loaded.as_ref().unwrap().notes[index];
            self.search.clear();
            if note.is_manifest() {
                self.scope = Scope::Package(note.owner.id().unwrap());
                self.section = Section::All;
                self.unreadable_kind = None;
            } else {
                self.scope = Scope::All;
                self.section = Section::Unreadable;
                self.unreadable_kind = Some(note.unreadable_kind());
            }
            self.open_entry(index);
            self.source = true;
            self.focus_repair = true;
        }
        let dirty = self.dirty();
        let locked = self.draft_reason().or(self
            .pending
            .is_some()
            .then_some("Wait for the Library to load."));
        // Beside Lookup at the minimum window, everything here gets smaller.
        let compact = ui.available_width() < 900.0 || self.disk_change.is_some();
        let mut open_form = false;
        let mut reload = false;
        let total = self.loaded.as_ref().map_or(0, |loaded| {
            loaded
                .notes
                .iter()
                .filter(|n| loaded.in_scope(n, self.scope))
                .count()
        });
        let columns = self.page_columns(ui, compact, dirty, total);
        let link_index = ui.ctx().data(|data| {
            data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                "reader_link_index",
            ))
        });
        // What keeps another entry from opening (`open_entry`) while a draft holds this one, in
        // that act's words; a file changed on disk says its own.
        let open_lock = match self.draft_reason() {
            Some(_) if self.disk_change.is_none() => Some("Save or discard this entry first."),
            reason => reason,
        };
        let mut open_broken = None;
        // An open draft's file that stopped reading has no reload yet: the header counts it now,
        // as a reload would, and names its short cause.
        let stopped_reading = self
            .disk_change
            .as_ref()
            .and_then(|change| change.fault.as_ref())
            .zip(
                self.selected
                    .and_then(|at| self.loaded.as_ref()?.notes.get(at)),
            )
            .map(|(fault, note)| {
                (
                    Loaded::file_key(note),
                    format!(
                        "{}: {}",
                        self.loaded.as_ref().unwrap().file_label(note),
                        fault.cause()
                    ),
                )
            });
        // M0327-M0329 (ruling M21): the header is the shared title line: the title and its
        // count, `New entry`, the `Packages` menu and the issues count, placed by measure.
        const NEW_ENTRY: design::Word = crate::word!("New entry");
        const PACKAGES: design::Word = crate::word!("Packages");
        let context = (total > 0 && ui.available_width() >= design::READER_MEASURE)
            .then(|| design::count(total, "entry", "entries"));
        // The places policy that `Copy to…` shares; the page line under the header paints its
        // reason.
        let no_place = Places::of(campaigns).create();
        let broken = self
            .loaded
            .as_ref()
            .zip(link_index.as_ref())
            .map(|(loaded, index)| loaded.broken_link_notes(index))
            .unwrap_or_default();
        let issues = self.loaded.as_ref().filter(|loaded| {
            !loaded.diagnostics.is_empty() || !broken.is_empty() || stopped_reading.is_some()
        });
        let word = issues.map(|loaded| {
            design::count(
                loaded.files_to_fix(&broken, stopped_reading.as_ref().map(|(file, _)| file)),
                "file needs fixing",
                "files need fixing",
            )
        });
        let mut line = design::TitleLine::new("Library").context(context.as_deref());
        if !(self.scope == Scope::Campaign && total == 0) {
            line = line.action(
                design::TitleAction::secondary(NEW_ENTRY)
                    .icon(icons::ADD)
                    .blocked(locked.or(no_place.as_ref().map(|reason| reason.as_str()))),
            );
        }
        line = line.menu(design::TitleAction::quiet(PACKAGES).blocked(locked), |ui| {
            if Action::quiet("Reload from disk")
                .icon(icons::REFRESH)
                .show(ui)
                .clicked()
            {
                reload = true;
                ui.close();
            }
            if Action::quiet("Manage packages…")
                .icon(icons::SETTINGS)
                .show(ui)
                .clicked()
            {
                open_form = true;
                ui.close();
            }
        });
        if let Some((loaded, word)) = issues.zip(word.as_deref()) {
            // The count has the tone of the most severe fault it counts: a file that cannot be
            // read is an error.
            let unreadable = loaded.unreadable_counted || stopped_reading.is_some();
            let status = Status {
                icon: if unreadable {
                    icons::ERROR
                } else {
                    icons::WARN
                },
                word,
                color: if unreadable {
                    colors().result.error
                } else {
                    colors().result.warning
                },
            };
            let broken = &broken;
            let stopped_reading = &stopped_reading;
            let open_broken = &mut open_broken;
            line = line.status(status, move |ui| {
                issues_menu(ui, loaded, broken, stopped_reading, open_lock, open_broken);
            });
        }
        // Issue #504: each menu of the line keeps inside the column under its control, clear of
        // its splitters and the list's scroll bar.
        let line = line.page().menu_room(move |control| {
            columns
                .iter()
                .copied()
                .find(|column| column.x_range().contains(control.center().x))
                .unwrap_or_else(|| columns[columns.len() - 1])
        });
        let new_entry = line.show(ui).clicked(NEW_ENTRY);
        if let Some(at) = open_broken {
            self.open_entry(at);
            self.reveal_row = true;
        }
        if new_entry {
            self.new_form = true;
            self.new_folder = kind_folder(&self.new_kind).into();
            self.focus_new_name = true;
            self.form = false;
        }
        if reload {
            self.start_load(ui.ctx(), campaigns);
        }
        if open_form {
            self.new_form = false;
            self.form = !self.form;
            self.focus_package_name = self.form;
        }
        if campaigns.active_campaign().is_none() {
            // New entry's reason, from the places policy it shares with `Copy to…`.
            let words = format!(
                "{} {}",
                if campaigns.all().is_empty() {
                    "No campaign yet."
                } else {
                    "No campaign open."
                },
                Places::of(campaigns)
                    .create()
                    .as_ref()
                    .map_or("The Library works without one.", |reason| reason.as_str())
            );
            ui.horizontal_wrapped(|ui| {
                ui.label(Type::Small.rich(ui, words).color(colors().muted));
                if Action::primary(if campaigns.all().is_empty() {
                    "Add campaign"
                } else {
                    "Choose a campaign"
                })
                .density(Density::Read)
                .show(ui)
                .clicked()
                {
                    self.wants_settings = true;
                }
            });
        }
        design::gap(ui, design::Gap::XS);
        self.unregister_dialog(ui.ctx(), campaigns);
        // The page's error and results sit in the pane's typed footer (below the reader, the
        // editor, the empty reader or a form), so they never move the columns, the tabs, the
        // file line, the title or the body; each footer reads them from their stores
        // (`page_messages`), and their presses are applied after the pane is drawn.
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Loading the Library…");
            });
            return;
        }

        let Some(loaded) = &self.loaded else {
            // Nothing loaded: there is no pane, so a failed load says so here (M20: a typed body
            // fault).
            if let Some(error) = &self.error {
                #[allow(
                    clippy::disallowed_methods,
                    reason = "a failed load where the page has no pane (ruling M20)"
                )]
                let _ = design::outcome::banner(ui, Banner::Fault(&LibraryFault::from(error)));
            }
            return;
        };
        // Each scope has its own counts. Search and sections stay within the scope.
        let scope = self.scope;
        let about_open = self.selected.is_some_and(|at| {
            loaded.notes[at].is_manifest()
                && matches!(scope, Scope::Package(id) if loaded.notes[at].owner.is_package(id))
        });
        if self
            .scope_counts
            .as_ref()
            .is_none_or(|(known, _)| *known != scope)
        {
            let notes = &loaded.notes;
            let counts = Section::GROUPS
                .iter()
                .flat_map(|(_, sections)| sections.iter())
                .map(|&section| {
                    (
                        section,
                        notes
                            .iter()
                            .filter(|n| {
                                loaded.in_scope(n, scope)
                                    && (section == Section::All || n.section() == section)
                            })
                            .count(),
                    )
                })
                .collect();
            self.scope_counts = Some((scope, counts));
        }
        let count_of = |section: Section, counts: &Vec<(Section, usize)>| {
            counts
                .iter()
                .find(|(s, _)| *s == section)
                .map_or(0, |(_, n)| *n)
        };
        let counts = self
            .scope_counts
            .as_ref()
            .map(|(_, counts)| counts.clone())
            .unwrap_or_default();
        let searching = !self.search.trim().is_empty();
        // The scope notice stands while the list keeps the scope it widened to, with no query:
        // the DM's own scope change or search ends it. (`open_entry` changes the scope itself,
        // so the list's last scope is not the notice's: comparing with it ended the notice in
        // the frame after it came, before any footer drew it.) Settled here, before every
        // footer of the page: the empty scope's, a form's and the reader's.
        if self
            .kept
            .scope_notice()
            .is_some_and(|(_, widened)| *widened != scope || searching)
        {
            let _ = self.kept.take_scope_notice();
        }
        let empty_scope = scope == Scope::Campaign && count_of(Section::All, &counts) == 0;
        if empty_scope {
            ui.scope(|ui| {
                ui.set_max_width(166.0);
                Self::scope_menu(
                    ui,
                    compact,
                    campaigns,
                    &mut self.scope,
                    &mut self.section,
                    &mut self.scope_menu,
                );
            });
            // Round 26 review: the page's results and error sit in this pane's footer too, below
            // the form or the empty state.
            let frame = PaneFrame::new(ui);
            let messages = self.form_messages(&frame, ui.available_width(), campaigns);
            let mut can_use = false;
            // The footer holds its own lines: the body below changes the Library.
            let paned = messages.pane(ui, design::Body::Content, |ui| {
                if self.new_form {
                    self.new_entry(ui, dirty, campaigns);
                } else {
                    design::empty_state_with_body(
                        ui,
                        icons::LIBRARY,
                        "No entries of your own yet",
                        |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;
                                ui.label(Type::Body.rich(ui, "Copy an ").color(colors().muted));
                                can_use = design::link(ui, "entry you can use").clicked();
                                ui.label(
                                    Type::Body
                                        .rich(ui, ", or make a new one here.")
                                        .color(colors().muted),
                                );
                            });
                        },
                        |ui| {
                            if Action::primary("New entry")
                                .icon(icons::ADD)
                                .show(ui)
                                .clicked()
                            {
                                self.new_form = true;
                                self.new_folder = kind_folder(&self.new_kind).into();
                            }
                        },
                    );
                }
            });
            if can_use {
                self.scope = Scope::Uses;
            }
            if let Some(press) = paned.pressed {
                self.apply_footer_press(press, campaigns);
            }
            return;
        }
        if let Some(column) = self.sections_column(compact) {
            column.show_inside(ui, |ui| {
                Self::scope_menu(
                    ui,
                    compact,
                    campaigns,
                    &mut self.scope,
                    &mut self.section,
                    &mut self.scope_menu,
                );
                if let Scope::Package(package) = scope
                    && let Some((index, note)) = loaded
                        .notes
                        .iter()
                        .enumerate()
                        .find(|(_, n)| n.owner.is_package(package) && n.is_manifest())
                    && about_row(ui, &note.about_label(loaded.has_problem(note)), about_open)
                        .clicked()
                    && !dirty
                {
                    // Round 26 SB1: a list click records the trail, as a link does.
                    if self.selected != Some(index) {
                        self.trail.leave(place(
                            loaded,
                            self.selected,
                            self.scope,
                            self.section,
                            &self.search,
                        ));
                    }
                    self.selected = Some(index);
                    let _ = self.draft.replace(note.text.as_str());
                    self.section = Section::All;
                    self.unreadable_kind = None;
                    self.source = false;
                }
                // M0333: the shared scroll column keeps its lane, so a count never moves
                // when the bar comes or goes.
                design::ScrollColumn::new("section_list").show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for &(group, sections) in Section::GROUPS {
                        // Keep the empty encounter section discoverable, but hide an unused fallback.
                        if sections
                            .iter()
                            .all(|section| count_of(*section, &counts) == 0)
                        {
                            continue;
                        }
                        if !group.is_empty() {
                            design::heading(
                                ui,
                                design::Heading::Group {
                                    title: group,
                                    count: sections
                                        .iter()
                                        .map(|section| count_of(*section, &counts))
                                        .sum(),
                                },
                            );
                        }
                        for &section in sections {
                            let count = count_of(section, &counts);
                            if count == 0 {
                                continue;
                            }
                            if section == Section::Unreadable {
                                for kind in Section::GROUPS
                                    .iter()
                                    .flat_map(|(_, sections)| sections.iter())
                                    .copied()
                                {
                                    let count = loaded
                                        .notes
                                        .iter()
                                        .filter(|note| {
                                            note.error.is_some()
                                                && loaded.in_scope(note, scope)
                                                && note.unreadable_kind() == kind
                                        })
                                        .count();
                                    if count == 0 {
                                        continue;
                                    }
                                    let label = if kind == Section::Unreadable {
                                        "Entries"
                                    } else {
                                        kind.label()
                                    };
                                    if section_row(
                                        ui,
                                        label,
                                        count,
                                        !about_open
                                            && self.section == section
                                            && self.unreadable_kind == Some(kind)
                                            && !searching,
                                    )
                                    .clicked()
                                    {
                                        self.section = section;
                                        self.unreadable_kind = Some(kind);
                                        self.matches = None;
                                        self.search.clear();
                                    }
                                }
                                continue;
                            }
                            let selected = self.section == section && !searching && !about_open;
                            let label = section.label();
                            // Round 30 AI4: with no search, All entries paints no
                            // count, which would repeat the header's; its name keeps
                            // it. During a search it paints the number of matches,
                            // zero included, from the list's kept result for this
                            // query and scope (no new work); until that result exists,
                            // no count.
                            let (count, painted) = if section == Section::All {
                                let query = self.search.trim().to_lowercase();
                                let found = self
                                    .matches
                                    .as_ref()
                                    .filter(|(_, kept, _)| {
                                        searching
                                            && *kept == query
                                            && self.matches_scope == Some(scope)
                                    })
                                    .map(|(.., found)| found.len());
                                (found.unwrap_or(count), found.is_some())
                            } else {
                                (count, true)
                            };
                            let response = section_row_counted(ui, label, count, selected, painted);
                            if response.clicked() {
                                if about_open && !dirty {
                                    self.selected = None;
                                    self.draft.clear();
                                }
                                self.section = section;
                                self.unreadable_kind = None;
                                self.search.clear();
                            }
                        }
                    }
                });
            });
        }
        // A query always searches the whole scope. Clearing it returns to the selected section.
        let query = self.search.trim().to_lowercase();
        let searching = !query.is_empty();
        let filtering =
            (self.section == Section::Creatures || searching) && !self.filter.is_empty();
        if self.matches_scope != Some(scope)
            || self
                .matches
                .as_ref()
                .is_none_or(|(section, cached_query, _)| {
                    *section != self.section || *cached_query != query
                })
            || self.matched_filter != self.filter
        {
            self.matched_filter = self.filter.clone();
            self.reveal_row |= self.matches_scope != Some(scope);
            self.matches_scope = Some(scope);
            let parsed = Query::parse(&query);
            let typed = Typed::new(&parsed.words);
            let facets = loaded.facets();
            let mut found: Vec<(Rank, usize)> = loaded
                .notes
                .iter()
                .enumerate()
                .filter(|(i, n)| {
                    loaded.in_scope(n, scope)
                        && (!filtering
                            || facets[*i].as_ref().is_some_and(|f| self.filter.matches(f)))
                        && (!query.is_empty()
                            || self.section != Section::Unreadable
                            || self
                                .unreadable_kind
                                .is_none_or(|kind| n.unreadable_kind() == kind))
                })
                .filter_map(|(i, n)| {
                    n.search_rank(self.section, &parsed, &typed, facets[i].as_ref())
                        .map(|rank| (rank, i))
                })
                .collect();
            if !parsed.words.is_empty() {
                // Best first: the exact name, then a prefix, then a whole word, then each word in
                // any order (a creature's whole word and a word that only holds the search are one
                // tier, M3). A path, section or fact match comes next, and a typo comes last. The
                // sort is stable, so the list's order (by name) stays inside each rank and origin.
                found.sort_by_key(|&(rank, i)| {
                    (
                        rank,
                        crate::lookup::entry_origin(
                            loaded.notes[i].entry.as_ref(),
                            matches!(loaded.notes[i].owner, Owner::Campaign(_)),
                        ),
                    )
                });
            }
            self.typo_from = found
                .iter()
                .position(|(rank, _)| rank.typo)
                .unwrap_or(found.len());
            let found = found.into_iter().map(|(_, i)| i).collect();
            self.matches = Some((self.section, query.clone(), found));
        }
        let matches = &self.matches.as_ref().unwrap().2;
        // Keep the reader in sync with the visible list without dropping an unsaved draft.
        let mut select = None;
        // The whole catalogue has no first entry worth opening for the DM.
        let whole_catalogue = self.section == Section::All && query.is_empty();
        if !dirty
            && self
                .selected
                .is_none_or(|i| !matches.contains(&i) && !loaded.notes[i].is_manifest())
        {
            // A name found only with a typo is a guess. It does not open by itself.
            select = matches
                .first()
                .copied()
                .filter(|_| !whole_catalogue && self.typo_from > 0);
            if select.is_none() {
                self.selected = None;
                self.draft.clear();
            }
        }
        let selected = select.or(self.selected);
        let subtitles = loaded.subtitles();
        let link_index = ui.ctx().data(|data| {
            data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                "reader_link_index",
            ))
        });
        let mut peek = None;
        let mut search_everything = false;
        let entries = self.entries_column(ui, ui.available_width(), dirty);
        self.list_hidden = entries.is_none();
        if let Some(column) = entries {
            column.show_inside(ui, |ui| {
                if compact {
                    let half = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;
                    ui.horizontal_top(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(half, 0.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                            Self::scope_menu(ui, compact, campaigns, &mut self.scope, &mut self.section, &mut self.scope_menu);
                        });
                        // M0334: the shared drop-down. As the section rows of a wider window do,
                        // it shows no chosen section while About is open or a search lists the
                        // whole scope, so choosing any section ends the search.
                        let short = |section: Section| match section {
                            Section::All => "All",
                            Section::Characters => "Characters",
                            _ => section.label(),
                        };
                        let mut sections = Section::GROUPS
                            .iter()
                            .flat_map(|(_, sections)| sections.iter().copied())
                            .filter(|section| count_of(*section, &counts) > 0);
                        if let Some(first) = sections.next() {
                            let mut chosen = (!about_open && !searching).then_some(self.section);
                            let mut dropdown = design::Dropdown::new(LIBRARY_SECTION, "Section")
                                .choice_with_short_label(Some(first), first.label(), short(first));
                            for section in sections {
                                dropdown = dropdown.choice_with_short_label(
                                    Some(section),
                                    section.label(),
                                    short(section),
                                );
                            }
                            if dropdown.width(half).show(ui, &mut chosen).changed
                                && let Some(section) = chosen
                            {
                                if about_open && !dirty {
                                    self.selected = None;
                                    self.draft.clear();
                                }
                                self.section = section;
                                self.search.clear();
                            }
                        }
                    });
                }
                let width = ui.available_width();
                // Clear empties the text; the search chooses nothing else to reset.
                let _ = LIBRARY_SEARCH.search(
                    ui,
                    SEARCH_NAME,
                    &mut self.search,
                    "Search by name or detail…",
                    width,
                );
                if compact
                    && let Scope::Package(package) = scope
                    && let Some((index, note)) = loaded.notes.iter().enumerate().find(|(_, n)| n.owner.is_package(package) && n.is_manifest())
                    && about_row(ui, &note.about_label(loaded.has_problem(note)), about_open).clicked() && !dirty
                {
                    if self.selected != Some(index) {
                        self.trail.leave(place(
                            loaded,
                            self.selected,
                            self.scope,
                            self.section,
                            &self.search,
                        ));
                    }
                    self.selected = Some(index);
                    let _ = self.draft.replace(note.text.as_str());
                    self.section = Section::All;
                    self.unreadable_kind = None;
                    self.source = false;
                }
                if self.section == Section::Creatures || searching {
                    let notes = &loaded.notes;
                    let facets = loaded.facets();
                    let changed = crate::creature_filter::filter_bar(ui, &mut self.filter, || {
                        notes
                            .iter()
                            .zip(facets)
                            .filter(|(note, _)| loaded.in_scope(note, scope))
                            .filter_map(|(_, facets)| facets.as_ref())
                            .collect()
                    });
                    if changed {
                        ui.ctx().request_repaint();
                    }
                }
                let headed = searching || filtering || (compact && self.section != Section::All);
                if headed {
                    design::heading(ui, design::Heading::Group { title: if searching { "Search results" } else if filtering { "Matches" } else { self.section.label() }, count: matches.len() });
                } else {
                    design::gap(ui, design::Gap::XS);
                }
                if searching && !matches.is_empty() && self.typo_from == 0 {
                    ui.add(
                        egui::Label::new(
                            Type::Body.rich(ui, CLOSE_NAMES)
                                .color(colors().muted),
                        )
                        .wrap(),
                    );
                }
                if matches.is_empty() {
                    // A heading owns the space after it.
                    if !headed {
                        design::gap(ui, design::Gap::S);
                    }
                    if let Scope::Package(id) = scope && !searching
                        && !loaded.notes.iter().any(|note| note.owner.is_package(id) && note.entry.as_ref().is_some_and(|entry| entry.kind != "package")) {
                        let name = campaigns.modules().iter().find(|m| m.id == id).map_or("this package", |m| m.name.as_str());
                        let mut create = false;
                        if self.new_form {
                            ui.label(Type::Title.rich(ui, icons::LIBRARY).color(colors().muted));
                            ui.label(Type::Title.rich(ui, format!("No entries in {name} yet")).color(colors().text));
                        } else {
                            design::empty_state(ui, icons::LIBRARY, &format!("No entries in {name} yet"), "Create the first entry.", |ui| {
                                create = Action::secondary("New entry").icon(icons::ADD).show(ui).clicked();
                            });
                        }
                        if create { self.new_form = true; self.focus_new_name = true; self.new_kind = "creature".into(); self.new_folder = "Creatures".into(); }
                    } else {
                    let filtered = !self.filter.is_empty() && (searching || self.section == Section::Creatures);
                    // Where a wider scope can find more, the sentence names this scope, and quiet
                    // `Search Everything` below it keeps the words and the filters.
                    let wider = (filtered || searching) && scope != Scope::All;
                    let text = if filtered && wider {
                        format!("No creatures match these filters in {}. Remove a filter, or search Everything.", scope_name(campaigns, scope))
                    } else if filtered {
                        "No creatures match these filters. Remove a filter.".to_owned()
                    } else if scope == Scope::Campaign && !searching {
                        "No campaign entries yet. Copy an entry here with Copy to…, or create one with New entry.".to_owned()
                    } else if searching && wider {
                        format!("No entries match in {}. Try fewer words, or search Everything.", scope_name(campaigns, scope))
                    } else if searching {
                        "No entries match. Try fewer words.".to_owned()
                    } else {
                        "No entries in this section.".to_owned()
                    };
                    // At the rows' text edge, so the action's hanging padding stays clear of the
                    // splitter at the column's left edge.
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: design::S as i8,
                            ..Default::default()
                        })
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(Type::Body.rich(ui, &text).color(colors().muted))
                                    .wrap(),
                            );
                            if wider {
                                search_everything = Action::quiet(SEARCH_EVERYTHING)
                                    .density(Density::Read)
                                    .line_start()
                                    .show(ui)
                                    .clicked();
                            }
                        });
                    }
                }
                ui.spacing_mut().item_spacing.y = 0.0;
                // The list's own region (layout step 3b): whole rows only, drawn in its scroll
                // column with its lane; only the rows in view draw. It is the one writer of its
                // offset: the selected entry that arrived by a link is a jump to its row.
                let rows = self
                    .entry_rows
                    .get_or_insert_with(|| design::memo::Revised::new(Vec::new()));
                if rows.len() != matches.len() {
                    rows.revise(|rows| {
                        *rows = vec![design::Block::Row(design::Row::Entry); matches.len()];
                        design::memo::Changed::Yes
                    });
                }
                let region = self
                    .entry_region
                    .get_or_insert_with(|| design::RegionState::new("library_entries"));
                let selected_row = selected.and_then(|selected| {
                    if let Some(row) = self
                        .selected_row
                        .filter(|&row| matches.get(row) == Some(&selected))
                    {
                        return Some(row);
                    }
                    #[cfg(test)]
                    ROW_SEARCHES.with(|searches| searches.set(searches.get() + 1));
                    let row = matches.iter().position(|i| *i == selected);
                    self.selected_row = row;
                    row
                });
                if std::mem::take(&mut self.reveal_row)
                    && let Some(row) = selected_row
                {
                    region.navigate(design::Navigation::Jump { block: row });
                }
                // One Tab stop, a list box: Tab enters at the selected entry; Up and Down move.
                design::BlockRegion::new(
                    ("entry_list", self.section.label(), &query, scope),
                    region,
                    rows,
                )
                .name("Library entries")
                .selected(selected_row)
                .show(ui, |ui, row| {
                        {
                            let i = matches[row];
                            let note = &loaded.notes[i];
                            let line = &subtitles[i];
                            let subtitle = match (&note.owner, scope) {
                                (Owner::Campaign(_), Scope::Campaign) => line.strip_suffix(" · Campaign").unwrap_or(line),
                                (Owner::Package(module), Scope::Package(id)) if module.id == id => line.strip_suffix(&format!(" · {}", module.name)).unwrap_or(line),
                                _ => line,
                            };
                            let broken = note.entry.as_ref().zip(link_index.as_ref()).filter(|_| note.owner != Owner::Builtin).map_or(0, |(entry, index)| index.broken_link_count(&entry.body, entry.id));
                            let faults = note.entry.as_ref().map_or(0, |entry| crate::roll_table::fault_count(&entry.body));
                            let subtitle = if faults > 0 {
                                format!("{} {} · {subtitle}", icons::WARN, design::count(faults, "table fault", "table faults"))
                            } else if broken > 0 {
                                // One fault gets one mark: when the file's every counted problem
                                // is a missing link, the links' count is its only cause.
                                let needs_fixing = format!("{} {NEEDS_FIXING} · ", icons::WARN);
                                let rest = subtitle
                                    .strip_prefix(&needs_fixing)
                                    .filter(|_| loaded.link_faults_only.contains(&Loaded::file_key(note)))
                                    .unwrap_or(subtitle);
                                format!("{} {} · {rest}", icons::WARN, design::count(broken, "broken link", "broken links"))
                            } else { subtitle.to_owned() };
                            let response = ui
                                .push_id(i, |ui| {
                                    entry_row(
                                        ui,
                                        note.name(),
                                        &subtitle,
                                        selected == Some(i),
                                    )
                                })
                                .inner;
                            if response
                                .on_hover_text(format!(
                                    "{}\n{}\n{}",
                                    note.name(),
                                    subtitle,
                                    note.path
                                ))
                                .clicked()
                            {
                                if dirty {
                                    peek = note.entry.as_ref().map(|e| e.id);
                                } else {
                                    select = Some(i);
                                }
                            }
                        }
                    });
            });
        }
        if peek.is_some() {
            self.peek = peek;
        }
        if search_everything {
            // The same words and filters, in the widest scope.
            self.scope = Scope::All;
            self.section = Section::All;
        }
        if let Some(i) = select {
            // Round 26 SF1, SB1: a list click leaves the open entry as a link does, so Back
            // returns to it and Forward ends.
            if self.selected != Some(i) {
                self.trail.leave(self.here());
            }
            self.selected = Some(i);
            let _ = self
                .draft
                .replace(self.loaded.as_ref().unwrap().notes[i].text.as_str());
            self.error = None;
            self.source = false;
        }
        let subtitle = self
            .selected
            .map(|i| subtitles[i].clone())
            .unwrap_or_default();
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: if self.disk_change.is_some() {
                    0
                } else {
                    design::L as i8
                },
                right: 0,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                if self.form || self.new_form {
                    // A form has no reader: the page's results sit in the pane's message footer,
                    // `S` below a short form, pinned under a long one (round 26; round 28 pane).
                    let frame = PaneFrame::new(ui);
                    let messages = self.form_messages(&frame, ui.available_width(), campaigns);
                    // The footer holds its own lines: the form below changes the Library.
                    let paned = messages.pane(ui, design::Body::Content, |ui| {
                        if self.form {
                            self.package_form(ui, campaigns);
                        } else {
                            self.new_entry(ui, dirty, campaigns);
                        }
                    });
                    self.footer_press = paned.pressed;
                } else {
                    self.editor(ui, &subtitle, campaigns);
                }
            });
        if let Some(press) = self.footer_press.take() {
            self.apply_footer_press(press, campaigns);
        }
    }

    /// The footer of a pane with no reader below a form or the empty scope (M-L0.4b): the
    /// page's results, then the form's own error and the page's, typed.
    fn form_messages<'f>(
        &self,
        frame: &'f PaneFrame,
        width: f32,
        campaigns: &Campaigns,
    ) -> Messages<'f, LibraryPress> {
        let mut messages = Messages::new(
            frame,
            "library_form_footer",
            width.min(design::READER_MEASURE),
        );
        self.page_messages(frame, &mut messages, &ReaderResults::default(), campaigns);
        page_faults(
            &mut messages,
            &[
                self.new_error.as_ref().filter(|_| self.new_form),
                self.error.as_ref(),
            ],
        );
        messages
    }

    /// The open entry's identity and root: what owns a link offer and its result.
    fn open_entry_key(&self) -> Option<(Uuid, PathBuf)> {
        let note = self.loaded.as_ref()?.notes.get(self.selected?)?;
        Some((note.entry.as_ref()?.id, note.owner.root()?.to_path_buf()))
    }

    /// How many links the offer after a rename would change. The loaded notes keep the count
    /// until they change, so an unchanged frame reads no note.
    fn link_rename_count(&self, old: &str, new: &str) -> usize {
        self.loaded
            .as_ref()
            .map_or(0, |loaded| loaded.rename_link_count(old, new))
    }

    /// The link offer's write, after its press used the offer up (by its ticket).
    fn update_links(&mut self, offer: LinkOffer, campaigns: &Campaigns) {
        if let Some(loaded) = &self.loaded {
            // The offer is checked again before the write: its entry still has the new name, and
            // neither name names another entry. A stale offer goes and writes nothing.
            // The offer is checked against the loaded notes first, then, directly before the
            // write (after the scan and after each linking file is checked against the disk),
            // against the entry's own file on disk. A stale offer goes and writes nothing.
            if !loaded.offer_holds(&offer) {
                self.error = Some(PageError::Own(LINKS_NOT_CHANGED.clone()));
            } else {
                let files: Vec<_> = loaded
                    .notes
                    .iter()
                    .filter_map(|note| {
                        let root = note.owner.root()?;
                        let (after, changed) =
                            crate::world::rename_note_links(&note.text, &offer.old, &offer.new);
                        (changed > 0).then(|| {
                            (
                                root.to_path_buf(),
                                note.path.clone(),
                                note.text.to_string(),
                                after,
                            )
                        })
                    })
                    .collect();
                let written = crate::content_workspace::ContentEdits::write_checked(files, || {
                    if loaded.offer_holds_on_disk(&offer) {
                        Ok(())
                    } else {
                        Err(LINKS_NOT_CHANGED.as_str().into())
                    }
                });
                let LinkOffer {
                    owner,
                    source,
                    old,
                    new,
                    ..
                } = offer;
                match written {
                    Ok(edits) => {
                        // The owner's new update ends its earlier result.
                        self.link_edits.record(LinkResult {
                            owner,
                            source,
                            edits,
                            old,
                            new,
                        });
                        self.stale = true;
                        self.content_changed =
                            campaigns.active_campaign().map(|campaign| campaign.path);
                    }
                    Err(error) => self.error = Some(error.into()),
                }
            }
        }
    }

    /// Undo unregister: the package comes back to this campaign's lists while its folder still
    /// holds it. A failure keeps the result and its Undo, with the error.
    fn undo_unregister(&mut self, campaigns: &mut Campaigns) {
        let Some(module) = self.kept.unregistered().cloned() else {
            return;
        };
        let result = local_module(&module.path)
            .map_err(PageError::from)
            .and_then(|found| {
                if found.id != module.id {
                    return Err(PageError::Own(FOLDER_HOLDS_ANOTHER_PACKAGE.clone()));
                }
                campaigns.restore_module(module).map_err(PageError::from)
            });
        match result {
            Ok(()) => {
                let _ = self.kept.take_unregistered();
                self.stale = true;
            }
            Err(error) => self.error = Some(error),
        }
    }

    /// `Use the copy in encounters` for the registration of `owner`. The use changes here; the
    /// registration's clause ends at this write's event (`reconcile_registrations`), as for
    /// every other use change.
    fn use_copy(&mut self, owner: RegistrationOwner, campaigns: &Campaigns) {
        if let Some(registered) = self.registrations.get(&owner)
            && let Some(campaign) = campaigns.active_campaign()
            && self.rosters.get(&owner).is_none()
        {
            match registered.value().copy.use_in_rosters(&campaign) {
                Ok(edits) => {
                    self.rosters.record(RostersUse {
                        owner,
                        edits,
                        blocked: false,
                    });
                    self.content_changed = Some(campaign.path);
                    self.stale = true;
                    self.reconcile_registrations(campaigns);
                }
                Err(error) => self.error = Some(error.into()),
            }
        }
    }

    /// A press in a Library typed footer (M-L0.4b), in the frame of the press, after the pane is
    /// drawn. A kept result's press acts by its ticket: on the entry its line showed, while that
    /// entry stands. A ticket of a result replaced or ended since does nothing.
    fn apply_footer_press(
        &mut self,
        Press { id, ticket }: Press<LibraryPress>,
        campaigns: &mut Campaigns,
    ) {
        let Some(ticket) = ticket else {
            return;
        };
        match id {
            LibraryPress::Undo => match self.kept.kind_by(&ticket) {
                // Its kind holds one result: the one the ticket names, in this frame.
                Some(KeptKind::Unregistered) => self.undo_unregister(campaigns),
                Some(KeptKind::PackageUse) => self.undo_package_use(),
                Some(KeptKind::Manifest) => self.undo_manifest(),
                // Gone (replaced or ended since its line was drawn), or a kind that no typed
                // footer shows yet.
                _ => {}
            },
            LibraryPress::UpdateLinks => {
                if self.loaded.is_some()
                    && let Some(offer) = self.renamed_links.use_press_by(ticket)
                {
                    self.update_links(offer, campaigns);
                }
            }
            LibraryPress::UndoLinks => {
                match self
                    .link_edits
                    .undo_by(ticket, |result| result.edits.undo())
                {
                    Ok((result, ())) => self.links_undone(result, campaigns),
                    Err(ByTicket::Failed(error)) => self.error = Some(error.into()),
                    Err(ByTicket::Gone) => {}
                }
            }
            LibraryPress::UndoRegistration => {
                if let Err(error) = self.undo_registration_by(ticket, campaigns) {
                    self.error = Some(error.into());
                }
            }
            LibraryPress::UseCopy => {
                // The registration the line showed, in the frame of the press.
                if let Some(owner) = self
                    .registrations
                    .get_by(&ticket)
                    .map(|shown| design::results::Kept::owner(shown.value()))
                {
                    self.use_copy(owner, campaigns);
                }
            }
            LibraryPress::UndoRosters => {
                match self.rosters.undo_by(ticket, |source| source.edits.undo()) {
                    Ok(_) => self.rosters_undone(campaigns),
                    Err(ByTicket::Failed(error)) => self.error = Some(error.into()),
                    Err(ByTicket::Gone) => {}
                }
            }
            // The editor's own presses act in `editor_pane`.
            LibraryPress::FixHeading
            | LibraryPress::UseVital(..)
            | LibraryPress::Repair
            | LibraryPress::SeparateEntry(_)
            | LibraryPress::SeparateEntries(_)
            | LibraryPress::CopyDependency(_) => {}
        }
    }

    /// A link update's Undo worked: the links say the old name again, so the rename's offer
    /// comes back in the result's place, with a new order. The reload checks it again, as every
    /// offer. The offer keeps the source version it was made from: its write checks it again.
    fn links_undone(&mut self, result: LinkResult, campaigns: &Campaigns) {
        let LinkResult {
            owner,
            source,
            old,
            new,
            ..
        } = result;
        self.renamed_links.record(LinkOffer {
            owner,
            source,
            old,
            new,
        });
        self.stale = true;
        self.content_changed = campaigns.active_campaign().map(|campaign| campaign.path);
    }

    /// The source change's Undo worked.
    fn rosters_undone(&mut self, campaigns: &Campaigns) {
        self.content_changed = campaigns.active_campaign().map(|campaign| campaign.path);
        self.stale = true;
        self.reconcile_registrations(campaigns);
    }

    /// What the catalogue shows. Choosing here never changes the active campaign. M0341: the
    /// shared drop-down; `All` and `Can use` are one group, `D&D 2024`, the packages and the
    /// campaign's entries another, under a rule.
    fn scope_menu(
        ui: &mut egui::Ui,
        compact: bool,
        campaigns: &Campaigns,
        current: &mut Scope,
        section: &mut Section,
        cache: &mut Option<ScopeMenu>,
    ) {
        let campaign = campaigns.active_id().is_some();
        let mut scope = if !campaign && matches!(*current, Scope::Uses | Scope::Campaign) {
            Scope::All
        } else {
            *current
        };
        let before = scope;
        let width = if compact {
            ui.available_width()
        } else {
            ui.available_width().min(166.0 + design::lane_width())
        };
        let positions = campaigns.module_positions();
        let layout = ScopeMenuLayout {
            catalogue: positions.revision(),
            campaign,
            width,
            font: Type::Strong.font(ui),
            fonts: design::outcome::font_key(ui.ctx()),
            scale: ui.ctx().pixels_per_point(),
        };
        cache.take_if(|menu| menu.layout != layout);
        let menu = cache.get_or_insert_with(|| {
            let fit = |words: &str| scope_label(ui, words, width - design::XXL - design::L);
            let menu = design::Dropdown::new(LIBRARY_SCOPE, "What the Library shows").width(width);
            let mut dropdown = if campaign {
                menu.choice_with_short_label(Scope::Uses, "Can use", fit("Can use"))
                    .choice_with_short_label(Scope::All, "Everything", fit("Everything"))
            } else {
                menu.choice_with_short_label(Scope::All, "Everything", fit("Everything"))
            }
            .group()
            .choice_with_short_label(Scope::Builtin, BUILTIN_NAME, fit("D&D 2024"));
            for module in campaigns.modules() {
                dropdown = dropdown.choice_with_short_label(
                    Scope::Package(module.id),
                    module.name.clone(),
                    fit(&module.name),
                );
            }
            if campaign {
                dropdown = dropdown.choice_with_short_label(
                    Scope::Campaign,
                    "Campaign entries",
                    fit("Campaign entries"),
                );
            }
            ScopeMenu { layout, dropdown }
        });
        let all = usize::from(campaign);
        let first_package = all + 2;
        let row = match scope {
            Scope::Uses => Some(0),
            Scope::All => Some(all),
            Scope::Builtin => Some(all + 1),
            Scope::Package(id) => positions.get(&id).map(|at| first_package + at),
            Scope::Campaign => Some(first_package + positions.len()),
        };
        menu.dropdown.show_at(ui, &mut scope, row);
        *current = scope;
        if scope != before {
            // The selected entry can stay if it is in this scope. A draft always stays.
            *section = Section::All;
        }
    }

    /// Show the catalogue limited to this campaign's own entries.
    pub fn show_campaign_entries(&mut self) {
        self.scope = Scope::Campaign;
        self.search.clear();
        self.section = Section::All;
    }

    fn unregister_dialog(&mut self, ctx: &egui::Context, campaigns: &mut Campaigns) {
        let Some(module) = self.confirm_unregister.clone() else {
            return;
        };
        let count = self
            .loaded
            .as_ref()
            .map_or(0, |l| unregister_count(&l.notes, module.id));
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut remove = false;
        let title = format!("Unregister {}?", module.name);
        design::dialog(ctx, "unregister_package", &title, |ui| {
            ui.add(
                egui::Label::new(format!(
                    "{} leave Splat’s lists, and any campaign that selects them can no longer use them.", design::count(count, "entry", "entries")
                ))
                .wrap(),
            );
            if let Some(campaign) = campaigns.active_campaign()
                && let Ok(catalogue) =
                    crate::content_workspace::Catalogue::load(campaigns.modules(), Some(&campaign))
            {
                let entry_ids: Vec<_> = catalogue
                    .entries
                    .values()
                    .filter(|entry| {
                        entry.owner == module.id && catalogue.selectable(entry.entry.id)
                    })
                    .map(|entry| crate::content_workspace::EntryRef::id(entry.entry.id))
                    .collect();
                let selection = crate::content_workspace::PackageSelection {
                    name: Some(module.name.clone()),
                    package: crate::content_workspace::EntryRef::id(module.id),
                    selection: crate::content_workspace::Selection::Selected { entries: entry_ids },
                };
                let users = selection.campaign_users(&catalogue);
                if !users.is_empty() {
                    let mut names = Vec::new();
                    for (id, _) in &users {
                        if let Some(user) = catalogue.entries.get(id) {
                            for reference in
                                content::required_references(&user.entry).unwrap_or_default()
                            {
                                if let Ok(id) = catalogue.resolve(&reference)
                                    && let Some(entry) = catalogue.entries.get(&id)
                                    && entry.owner == module.id
                                {
                                    names.push(entry.entry.name.clone());
                                }
                            }
                        }
                    }
                    names.sort();
                    names.dedup();
                    let users: Vec<_> = users.into_iter().map(|(_, name)| name).collect();
                    ui.label(format!(
                        "{} uses {}, in {}.",
                        campaign.name,
                        name_list(&names),
                        name_list(&users)
                    ));
                }
            }
            design::help(ui, &format!("The files stay in {}.", module.path.display()));
            design::gap(ui, design::Gap::S);
            design::dialog_actions(
                ui,
                |ui| {
                    remove = Action::destructive("Unregister").show(ui).clicked();
                },
                |ui| {
                    keep |= Action::secondary("Keep the package").show(ui).clicked();
                },
            );
        });
        if keep {
            self.confirm_unregister = None;
        }
        if remove {
            self.confirm_unregister = None;
            self.unregister(module, campaigns);
        }
    }

    /// Unregister `module` from Settings: its files stay. Its result, with Undo, is the newest
    /// kept result.
    fn unregister(&mut self, module: Module, campaigns: &mut Campaigns) {
        match campaigns.unregister_module(module.id) {
            Ok(()) => {
                self.kept.set_unregistered(module);
                self.scope = Scope::default();
                self.stale = true;
            }
            Err(e) => self.error = Some(e.into()),
        }
    }

    /// Registering a package is a form in the page, not a window over it.
    #[allow(
        clippy::disallowed_methods,
        reason = "a copied package folder’s warning in the form, with its repair (ruling M20)"
    )]
    fn package_form(&mut self, ui: &mut egui::Ui, campaigns: &mut Campaigns) {
        let c = colors();
        let locked = self.draft_reason().or(self
            .pending
            .is_some()
            .then_some("Wait for the Library to load."));
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.form = false;
        }
        // The pane scrolls the form; it has no scroll area of its own.
        ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                design::gap(ui, design::Gap::L);
                ui.horizontal(|ui| {
                    ui.label(
                        Type::Heading
                            .rich(ui, "Manage packages")
                            .color(colors().text),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if Action::quiet("Close")
                            .name("Close the package form")
                            .show(ui)
                            .clicked()
                        {
                            self.form = false;
                        }
                    });
                });
                design::heading(
                    ui,
                    design::Heading::Group {
                        title: "Add a package folder",
                        count: 0,
                    },
                );
                if self.copied_package.as_ref().is_some_and(|copy| {
                    Path::new(self.path.trim()).canonicalize().ok().as_ref() != Some(&copy.path)
                }) {
                    self.copied_package = None;
                    self.copy_name_open = false;
                }
                let lock = self.package_lock();
                let add_reason = lock
                    .or(self.copied_package.is_some().then_some(PackageReason::Copy))
                    .or(self
                        .path
                        .trim()
                        .is_empty()
                        .then_some(PackageReason::NoFolder));
                // Round 26 FB6 and round 28 FA7: the one inline creation form. Add folder follows
                // the field (after Browse…) on its line; the control-high kept line holds the
                // reason. For a copy, the reason is the next step: the warning below says the fact.
                let form = design::InlineCreate::new()
                    .field(
                        design::CreateField::new(PACKAGE_FOLDER, "Package folder", &mut self.path)
                            // Round 28 K4a: Browse locks with the form, says why, and its press
                            // is the part's to refuse.
                            .side_action(design::SideAction::new(BROWSE).locked_when(
                                lock.is_some(),
                                crate::sentence!("Browse waits for the reason below."),
                            )),
                    )
                    .create(design::Create::secondary(ADD_FOLDER).blocked(add_reason))
                    .kept_line(design::KeptLine::new())
                    .show(ui);
                if form.side
                    && let Some(path) = crate::hermetic::pick_folder()
                {
                    self.path = path.display().to_string();
                }
                let add = form.create;
                {
                    if add {
                        match local_module(Path::new(self.path.trim())).and_then(|module| {
                            if let Some(original) = campaigns
                                .modules()
                                .iter()
                                .find(|m| m.id == module.id && m.path != module.path)
                            {
                                self.copy_name = format!("{} copy", original.name);
                                self.copy_name_open = false;
                                self.copy_not_used = campaigns
                                    .active_campaign()
                                    .and_then(|campaign| CampaignDocument::load(&campaign).ok())
                                    .is_some_and(|document| {
                                        document.selections.iter().any(|selection| {
                                            selection.package.id == Some(module.id)
                                        })
                                    });
                                self.copied_package = Some(module);
                                self.error = None;
                                return Ok(None);
                            }
                            campaigns.register_module(module.clone())?;
                            Ok(Some(module))
                        }) {
                            Ok(Some(module)) => {
                                self.scope = Scope::Package(module.id);
                                self.form = false;
                                self.stale = true;
                            }
                            Ok(None) => {}
                            Err(error) => self.error = Some(error.into()),
                        }
                    }
                }
                if let Some(copy) = self.copied_package.clone() {
                    if Path::new(self.path.trim()).canonicalize().ok().as_ref() != Some(&copy.path)
                    {
                        self.copied_package = None;
                    } else {
                        let name = campaigns
                            .modules()
                            .iter()
                            .find(|m| m.id == copy.id)
                            .map_or(copy.name.as_str(), |m| m.name.as_str());
                        let warning = crate::sentence!("This folder is a copy of {}.", name);
                        let register = if self.copy_name_open {
                            let _ = BodyFault::<()>::new(&LibraryFault::Warning(&warning)).show(ui);
                            design::heading(
                                ui,
                                design::Heading::Group {
                                    title: "Register a copy",
                                    count: 0,
                                },
                            );
                            // Round 27 FD2, round 28 FA7: the inline creation form. The choice
                            // first, then the name with `Register` at the end of its line, then
                            // the kept line: quiet Cancel at its start, the reason after it. The
                            // commit keeps one word, so typing moves nothing.
                            let register_reason = self.package_lock().or(self
                                .copy_name
                                .trim()
                                .is_empty()
                                .then_some(PackageReason::NoName));
                            // The commit of an inline creation form has one weight: primary, as
                            // `Create package` is, unless a run owns the primary.
                            let commit = if self.run_open {
                                design::Create::secondary(REGISTER)
                            } else {
                                design::Create::primary(REGISTER)
                            }
                            .blocked(register_reason);
                            let field =
                                design::CreateField::new(COPY_NAME, "New package name", &mut self.copy_name);
                            let kept = design::KeptLine::new().cancel(CANCEL, true);
                            let mut chosen = !self.copy_not_used;
                            let help = if chosen {
                                "Uses every entry in this package, including future entries."
                            } else {
                                "This campaign keeps the original. The copy is registered but not chosen."
                            };
                            let form = if campaigns.active_campaign().is_some() {
                                design::InlineCreate::new()
                                    .choice(
                                        design::CreateChoice::new("Choose for this campaign", &mut chosen)
                                            .help(help),
                                    )
                                    .field(field)
                                    .create(commit)
                                    .kept_line(kept)
                                    .show(ui)
                            } else {
                                design::InlineCreate::new()
                                    .field(field)
                                    .create(commit)
                                    .kept_line(kept)
                                    .show(ui)
                            };
                            self.copy_not_used = !chosen;
                            if form.cancel {
                                self.copy_name_open = false;
                            }
                            form.create
                        } else {
                            let repair = Repair::new("Register as a new package", ());
                            BodyFault::new(&LibraryFault::Warning(&warning))
                                .repair(match self.page_lock() {
                                    Some(reason) => repair.blocked(reason),
                                    None => repair,
                                })
                                .show(ui)
                                .is_some()
                        };
                        if register {
                            if self.copy_name_open {
                                if let Err(error) = self.register_copy(campaigns) {
                                    self.error = Some(error.into());
                                }
                            } else {
                                self.copy_name_open = true;
                            }
                        }
                        design::quiet_section(ui, "Technical details", |ui| {
                            ui.label(copy.id.to_string());
                        });
                    }
                }

                if !self.copy_name_open {
                    design::heading(
                        ui,
                        design::Heading::Group {
                            title: "New package",
                            count: 0,
                        },
                    );
                    let reason = self
                        .package_lock()
                        .or(self.name.trim().is_empty().then_some(PackageReason::NoName));
                    let default_parent = || {
                        campaigns
                            .modules()
                            .last()
                            .and_then(|module| module.path.parent())
                            .map(Path::to_path_buf)
                            .or_else(|| {
                                campaigns
                                    .package_folder("new_package")
                                    .ok()
                                    .and_then(|p| p.parent().map(Path::to_path_buf))
                            })
                            .unwrap_or_default()
                    };
                    let parent = self.package_path.get_or_insert_with(default_parent).clone();
                    let namespace: String = self
                        .name
                        .trim()
                        .to_lowercase()
                        .chars()
                        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                        .collect();
                    let root: Result<PathBuf, String> = Ok(parent.join(&namespace));
                    // Round 27: what Create package will make is the kept line's preview, so
                    // typing moves nothing below the form.
                    let preview = root
                        .as_ref()
                        .ok()
                        .filter(|_| !self.name.trim().is_empty())
                        .map(|root| format!("Will create {}", root.display()));
                    let commit = if self.run_open {
                        design::Create::secondary(CREATE_PACKAGE)
                    } else {
                        design::Create::primary(CREATE_PACKAGE)
                    }
                    .blocked(reason);
                    let clear = !self.name.is_empty() || self.package_folder_changed;
                    let mut folder = parent.display().to_string();
                    let focus = std::mem::take(&mut self.focus_package_name);
                    let name = design::CreateField::new(NEW_PACKAGE_NAME, "Name", &mut self.name)
                        .focus(focus);
                    let folder_field =
                        design::CreateField::new(NEW_PACKAGE_FOLDER, "Folder", &mut folder)
                            .side_action(BROWSE);
                    let kept = design::KeptLine::new()
                        .cancel(CLEAR, clear)
                        .preview(preview.as_deref());
                    let mut chosen = !self.package_not_used;
                    // Round 26 FB6: the choice comes before the last field, so Create package
                    // follows that field on its line.
                    let form = if campaigns.active_campaign().is_some() {
                        design::InlineCreate::new()
                            .field(name)
                            .choice(
                                design::CreateChoice::new("Choose for this campaign", &mut chosen)
                                    .help("Uses every entry in this package, including future entries."),
                            )
                            .field(folder_field)
                            .create(commit)
                            .kept_line(kept)
                            .show(ui)
                    } else {
                        design::InlineCreate::new()
                            .field(name)
                            .field(folder_field)
                            .create(commit)
                            .kept_line(kept)
                            .show(ui)
                    };
                    self.package_not_used = !chosen;
                    if folder != parent.display().to_string() {
                        self.package_path = Some(PathBuf::from(folder));
                        self.package_folder_changed = true;
                    }
                    if form.side
                        && let Some(chosen) = crate::hermetic::pick_folder()
                    {
                        self.package_path = Some(chosen);
                        self.package_folder_changed = true;
                    }
                    if form.cancel {
                        self.name.clear();
                        self.package_path = None;
                        self.package_folder_changed = false;
                    }
                    let create = form.create;
                    {
                        if create {
                            let namespace: String = self
                                .name
                                .trim()
                                .to_lowercase()
                                .chars()
                                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                                .collect();
                            match root
                                .and_then(|root| {
                                    fs::create_dir_all(
                                        root.parent().ok_or("No package folder is available.")?,
                                    )
                                    .map_err(|e| e.to_string())?;
                                    create_module(&root, &self.name, &namespace)
                                })
                                .and_then(|module| {
                                    campaigns.register_module(module.clone())?;
                                    if !self.package_not_used
                                        && let Some(campaign) = campaigns.active_campaign()
                                    {
                                        crate::content_workspace::use_in_campaign(
                                            &campaign,
                                            module.id,
                                            &module.name,
                                            None,
                                        )?;
                                    }
                                    Ok(module)
                                }) {
                                Ok(module) => {
                                    self.content_changed =
                                        campaigns.active_campaign().map(|c| c.path);
                                    self.scope = Scope::Package(module.id);
                                    self.form = false;
                                    self.new_form = true;
                                    self.new_place = None;
                                    self.new_name.clear();
                                    self.new_folder = kind_folder(&self.new_kind).into();
                                    self.new_not_used = false;
                                    self.focus_new_name = true;
                                    self.stale = true;
                                }
                                Err(error) => self.error = Some(error.into()),
                            }
                        }
                    }
                }
                design::heading(
                    ui,
                    design::Heading::Group {
                        title: "Registered packages",
                        count: 0,
                    },
                );
                design::rule(ui);
                if campaigns.modules().is_empty() {
                    ui.label(Type::Body.rich(ui, "None yet.").color(c.muted));
                }
                for module in campaigns.modules().to_vec() {
                    ui.push_id(module.id, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.set_max_width((ui.available_width() - design::XXL).max(120.0));
                                ui.label(Type::Strong.rich(ui, &module.name).color(c.text));
                                ui.add(
                                    egui::Label::new(
                                        Type::Small
                                            .rich(
                                                ui,
                                                module
                                                    .path
                                                    .display()
                                                    .to_string()
                                                    .replace('/', "/\u{200b}"),
                                            )
                                            .color(c.muted),
                                    )
                                    .wrap(),
                                );
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    design::row_menu(ui, &module.name, |ui| {
                                        if Action::destructive("Unregister…")
                                            .density(Density::Read)
                                            .name(&format!("Unregister {}", module.name))
                                            .blocked(locked)
                                            .show(ui)
                                            .clicked()
                                        {
                                            self.confirm_unregister = Some(module.clone());
                                            ui.close();
                                        }
                                    });
                                },
                            );
                        });
                    });
                    design::rule(ui);
                }
            });
    }

    fn new_entry(&mut self, ui: &mut egui::Ui, dirty: bool, campaigns: &Campaigns) {
        if self.new_kind.is_empty() {
            self.new_kind = "creature".into();
            self.new_folder = kind_folder(&self.new_kind).into();
        }
        if self.loaded.is_none() {
            return;
        }
        if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.new_form = false;
            self.new_error = None;
            return;
        }
        // Where a new entry can go: the campaign, or a package. Never the built-in rules.
        let mut places: Vec<Owner> = campaigns
            .modules()
            .iter()
            .cloned()
            .map(Owner::Package)
            .collect();
        if let Some(campaign) = campaigns.active_campaign() {
            places.insert(0, Owner::Campaign(campaign));
        }
        if places.is_empty() {
            return;
        }
        let in_scope = places.iter().position(|owner| match self.scope {
            Scope::Package(id) => owner.is_package(id),
            Scope::Campaign => matches!(owner, Owner::Campaign(_)),
            _ => false,
        });
        let default_place = in_scope.unwrap_or(0);
        let mut place = self
            .new_place
            .filter(|place| *place < places.len())
            .unwrap_or(default_place);
        ui.vertical(|ui| {
            ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
            design::heading(
                ui,
                design::Heading::Group {
                    title: "New entry",
                    count: 0,
                },
            );
            ui.label(
                Type::Small
                    .rich(ui, "Creates the entry, then opens it in the editor.")
                    .color(colors().muted),
            );
            ui.vertical(|ui| {
                let response = design::labelled_field(
                    ui,
                    NEW_ENTRY_NAME,
                    "Entry name",
                    &mut self.new_name,
                    None,
                    None,
                );
                // The create attempt's error ends when the DM changes the name.
                if response.changed() {
                    self.new_error = None;
                }
                if std::mem::take(&mut self.focus_new_name) {
                    response.request_focus();
                }
                if self.new_kind.is_empty() {
                    self.new_kind = "creature".into();
                }
                let old_kind = self.new_kind.clone();
                design::field(ui, "Kind", None, None, |ui| {
                    let word = |kind: &str| {
                        let word = kind.replace('_', " ");
                        format!("{}{}", word[..1].to_uppercase(), &word[1..])
                    };
                    let mut kinds = ["spell", "item", "effect", "rule", "rule_group"]
                        .into_iter()
                        .map(|kind| (kind.to_owned(), word(kind)));
                    design::Dropdown::new(NEW_ENTRY_KIND, "Kind of entry")
                        .choice("creature".to_owned(), word("creature"))
                        .choices(kinds.by_ref())
                        .width(ui.available_width())
                        .show(ui, &mut self.new_kind);
                });
                if self.new_kind != old_kind {
                    self.new_folder = kind_folder(&self.new_kind).into();
                }
                design::field(ui, "Destination", None, None, |ui| {
                    let mut rows = places
                        .iter()
                        .enumerate()
                        .map(|(i, owner)| (i, owner.label()));
                    if let Some((first, label)) = rows.next() {
                        design::Dropdown::new(NEW_ENTRY_PLACE, "Destination")
                            .choice(first, label)
                            .choices(rows)
                            .width(ui.available_width())
                            .show(ui, &mut place);
                    }
                });
                // Round 26 FB6: the choice comes before the last field, so Create entry follows
                // that field on its line; Cancel and the reason are on the line below.
                if matches!(places[place], Owner::Package(_))
                    && campaigns.active_campaign().is_some()
                {
                    use_checkbox(
                        ui,
                        &mut self.new_not_used,
                        |_| "Adds this entry to the campaign’s package choices.",
                    );
                }
                let blocked = if dirty {
                    Some("Save or discard this entry first.")
                } else if self.new_name.trim().is_empty() {
                    Some("Type a name first.")
                } else {
                    None
                };
                let create_width = Action::primary("Create entry").natural_width(ui);
                let mut create = false;
                ui.horizontal_top(|ui| {
                    let width = ui.available_width() - create_width - ui.spacing().item_spacing.x;
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            folder_field(
                                ui,
                                NEW_ENTRY_FOLDER,
                                "Folder",
                                &mut self.new_folder,
                                places[place].root().unwrap(),
                                Some(&self.new_kind),
                            );
                        },
                    );
                    // The button stands on the field's line, below its label.
                    let drop = design::L + design::XS;
                    design::dropped(ui, egui::vec2(ui.available_width(), drop), drop, |ui| {
                        create = Action::primary("Create entry")
                            .blocked(blocked)
                            .show(ui)
                            .clicked();
                    });
                });
                let relative = if self.new_folder.trim().is_empty() {
                    format!("{}.md", self.new_name.trim())
                } else {
                    format!(
                        "{}/{}.md",
                        self.new_folder.trim().trim_end_matches('/'),
                        self.new_name.trim()
                    )
                };
                // One kept line: quiet Cancel at its start, then the reason, or the file Create
                // entry will write. Round 27: the preview lives in this line, so typing moves
                // nothing.
                ui.horizontal(|ui| {
                    ui.set_min_height(Density::Read.control());
                    if Action::quiet("Cancel")
                        .density(Density::Read)
                        .line_start()
                        .show(ui)
                        .clicked()
                    {
                        self.new_form = false;
                        self.new_error = None;
                    }
                    design::gap(ui, design::Gap::S); // With the line's item gap (`S`): `L`.
                    if blocked.is_some() {
                        design::blocked_reason(ui, blocked);
                    } else if !self.new_name.trim().is_empty() {
                        let destination = match &places[place] {
                            Owner::Package(package) => &package.name,
                            Owner::Campaign(campaign) => &campaign.name,
                            Owner::Builtin => BUILTIN_NAME,
                        };
                        creation_path(ui, places[place].root().unwrap(), &relative, destination);
                    }
                });
                // A closure, so an early `return` ends only the creation.
                (|| {
                    if create {
                        let name = self.new_name.trim();
                        if name.contains(['/', '\\', '\n', '\r']) || name == "." || name == ".." {
                            self.new_error = Some(PageError::Own(NAME_WITHOUT_SEPARATORS.clone()));
                            return;
                        }
                        let path = relative.clone();
                        let text = note_text(name, &self.new_kind, None);
                        let owner = places[place].clone();
                        let root = owner.root().expect("a writable owner").to_path_buf();
                        // The entry the new one replaces in the reader, for Back.
                        let left = self.here();
                        match write_note(&root, &path, None, &text) {
                            Ok(entry) => {
                                let used = self.choose_new_entry(&owner, entry.id, campaigns);
                                let l = self.loaded.as_mut().unwrap();
                                if !self.new_not_used
                                    && used.is_ok()
                                    && let (Some(uses), Some(chosen)) = (&mut l.uses, &mut l.chosen)
                                {
                                    uses.insert(entry.id);
                                    chosen.insert(entry.id);
                                }
                                let created_path = path.clone();
                                let created_owner = owner.key();
                                l.notes.push(Note {
                                    path,
                                    text: crate::design::memo::Revised::new(text.clone()),
                                    entry: Some(entry),
                                    error: None,
                                    fault: None,
                                    manifest_gone: false,
                                    owner,
                                });
                                l.notes.sort_by_cached_key(|n| natural_key(n.name()));
                                l.invalidate();
                                self.matches = None;
                                self.scope_counts = None;
                                self.selected = file_at(&l.notes, (created_owner, &created_path));
                                if self.selected.is_some() {
                                    self.trail.leave(left);
                                }
                                self.reveal_row = true;
                                self.search.clear();
                                self.section = Section::All;
                                self.source = true;
                                let _ = self.draft.replace(text);
                                self.focus_hp = self.new_kind == "creature";
                                self.new_name.clear();
                                self.new_error = None;
                                self.new_folder.clear();
                                self.new_form = false;
                                self.error = used.err();
                                // Show the new entry, wherever the DM chose to put it.
                                if self
                                    .selected
                                    .is_some_and(|i| !l.in_scope(&l.notes[i], self.scope))
                                {
                                    self.scope = Scope::All;
                                }
                                self.content_changed = Some(root);
                            }
                            Err(e) => self.new_error = Some(e.into()),
                        }
                    }
                })();
            });
        });
        self.new_place = (place != default_place).then_some(place);
    }

    /// The open entry, as Back or Forward returns to it.
    fn here(&self) -> Option<Place> {
        place(
            self.loaded.as_ref()?,
            self.selected,
            self.scope,
            self.section,
            &self.search,
        )
    }

    fn go(&mut self, step: Step) {
        if self.dirty() {
            return;
        }
        let here = self.here();
        let Some((target, index)) = self.trail.go(step, here, |target| {
            target.find(&self.loaded.as_ref()?.notes)
        }) else {
            return;
        };
        self.selected = Some(index);
        self.reveal_row = true;
        let _ = self
            .draft
            .replace(self.loaded.as_ref().unwrap().notes[index].text.as_str());
        self.disk_change = None;
        let _ = self.kept.take_scope_notice();
        self.scope = target.scope;
        self.section = target.section;
        self.search = target.search;
        self.matches = None;
        self.source = false;
        self.error = None;
    }

    /// Keep the reader at the pane's left edge, with its scroll bar beside its measure.
    fn editor(&mut self, ui: &mut egui::Ui, subtitle: &str, campaigns: &Campaigns) {
        let spare = ui.available_width() - view::WIDE_READER;
        if spare <= 0.0 || self.disk_change.is_some() {
            self.editor_pane(ui, subtitle, campaigns, None);
            return;
        }
        // The reader's lines keep their widest measure. Its body may still reach the pane's
        // edge, for a wide table.
        let pane_right = ui.max_rect().right();
        ui.horizontal_top(|ui| {
            let size = egui::vec2(view::WIDE_READER, ui.available_height());
            ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_max_width(view::WIDE_READER);
                self.editor_pane(ui, subtitle, campaigns, Some(pane_right));
            });
        });
    }

    fn save_selected(&mut self, file_gone: bool) {
        let Some(i) = self.selected else {
            return;
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let mut file_gone = file_gone;
        if self.loaded.as_ref().unwrap().duplicates(i).is_empty()
            && (file_gone
                || note_path(note.owner.root().unwrap(), &note.path)
                    .is_ok_and(|path| !path.exists()))
        {
            let moved = crate::content_workspace::note_id(&note.text)
                .map(|id| {
                    let mut owners = vec![note.owner.clone()];
                    for owner in &self.loaded.as_ref().unwrap().owners {
                        if !owners.contains(owner) {
                            owners.push(owner.clone());
                        }
                    }
                    for other in &self.loaded.as_ref().unwrap().notes {
                        if other.owner.root().is_some() && !owners.contains(&other.owner) {
                            owners.push(other.owner.clone());
                        }
                    }
                    let mut found = None;
                    for owner in owners {
                        let old = if owner == note.owner {
                            note.path.as_str()
                        } else {
                            ""
                        };
                        if let Some((path, text)) = moved_note(owner.root().unwrap(), id, old)? {
                            if found.is_some() {
                                return Err(PageError::Own(SEVERAL_FILES_CHOOSE_ONE.clone()));
                            }
                            found = Some((owner, path, text));
                        }
                    }
                    Ok(found)
                })
                .transpose();
            match moved {
                Ok(Some(Some((owner, path, text)))) => {
                    let note = &mut self.loaded.as_mut().unwrap().notes[i];
                    note.path = path;
                    note.owner = owner;
                    if *note.text != text {
                        self.disk_change = Some(DiskChange {
                            text: Some(text),
                            moved_to: Some(note.path.clone()),
                            fault: None,
                        });
                        self.error = Some(PageError::Own(NOTE_CHANGED.clone()));
                        return;
                    }
                    file_gone = false;
                    self.stale = true;
                }
                Err(error) => {
                    self.error = Some(error);
                    return;
                }
                _ => {}
            }
        }
        // A package whose manifest is known to be gone (its warning says so). Its path is known
        // ABSENT, which is not the same as an empty file. Save writes the draft as a new
        // manifest only when the path is still free and no other file holds the package's
        // registered identity; it never writes over a file (the no-clobber write).
        enum Gone {
            Create,
            Elsewhere(String),
            Back(Option<String>),
            Unread(FileFault),
            Failed(PageError),
        }
        let gone = {
            let note = &self.loaded.as_ref().unwrap().notes[i];
            match (&note.owner, note.manifest_gone) {
                (Owner::Package(module), true) => {
                    let root = module.path.as_path();
                    Some(match moved_note(root, module.id, &note.path) {
                        // The manifest came back under another name: no second one.
                        Ok(Some((path, _))) => Gone::Elsewhere(path),
                        Err(error) => Gone::Failed(error),
                        Ok(None) => match read_disk(root, &note.path) {
                            Ok(None) => Gone::Create,
                            // Any file there now, an empty one too: the comparison decides.
                            Ok(Some(text)) => Gone::Back(Some(text)),
                            Err(fault) => Gone::Unread(fault),
                        },
                    })
                }
                _ => None,
            }
        };
        match gone {
            Some(Gone::Create) => file_gone = true,
            Some(Gone::Elsewhere(path)) => {
                self.error = Some(PageError::Own(crate::sentence!(
                    "Not saved. This package’s manifest is now in {}. Reload from disk to open it there.",
                    path
                )));
                return;
            }
            Some(Gone::Failed(error)) => {
                self.error = Some(error);
                return;
            }
            Some(Gone::Back(text)) => {
                self.disk_change = Some(DiskChange {
                    text,
                    moved_to: None,
                    fault: None,
                });
                return;
            }
            Some(Gone::Unread(fault)) => {
                self.disk_change = Some(DiskChange {
                    text: None,
                    moved_to: None,
                    fault: Some(fault),
                });
                return;
            }
            None => {}
        }
        // The write check reads the identity index from the notes as they are now.
        self.loaded.as_mut().unwrap().duplicates.take();
        let loaded = self.loaded.as_ref().unwrap();
        let note = &loaded.notes[i];
        let root = note.owner.root().expect("an editable note").to_path_buf();
        let refused = loaded.draft_refusal(i, &Entry::parse(&note.path, &self.draft));
        let result = match &refused {
            Some(error) => Err(error.clone()),
            None => {
                let baseline = (!file_gone).then_some(note.text.as_str());
                write_note(&root, &note.path, baseline, &self.draft)
            }
        };
        match result {
            Ok(entry) => {
                let l = self.loaded.as_mut().unwrap();
                // A saved edit of the entry ends its link update's result and its earlier offer,
                // whatever this save offers. A rename that passes the check gives a new offer.
                // Other entries keep theirs.
                let owner = LinkOwner {
                    id: entry.id,
                    root: root.clone(),
                };
                self.link_edits.owner_changed(&owner);
                self.renamed_links.owner_changed(&owner);
                if let Some(old) = &l.notes[i].entry
                    && old.name != entry.name
                    && entry.kind != "package"
                    && l.can_rename_links(i, &old.name, &entry.name)
                {
                    self.renamed_links.record(LinkOffer {
                        owner: entry.id,
                        source: OfferSource {
                            root: root.clone(),
                            path: l.notes[i].path.clone(),
                            text: self.draft.as_str().to_owned(),
                        },
                        old: old.name.clone(),
                        new: entry.name.clone(),
                    });
                }
                if entry.kind == "package"
                    && let Owner::Package(module) = &l.notes[i].owner
                {
                    self.renamed = Some((module.id, entry.name.clone()));
                    let id = module.id;
                    for note in &mut l.notes {
                        if let Owner::Package(owner) = &mut note.owner
                            && owner.id == id
                        {
                            owner.name = entry.name.clone();
                        }
                    }
                }
                self.stale |= l.notes[i]
                    .entry
                    .as_ref()
                    .is_none_or(|old| old.id != entry.id);
                l.notes[i].entry = Some(entry);
                l.notes[i].text = crate::design::memo::Revised::new(self.draft.as_str().to_owned());
                l.notes[i].error = None;
                // A manifest written from its draft is on disk again.
                l.notes[i].manifest_gone = false;
                l.invalidate();
                self.matches = None;
                self.scope_counts = None;
                self.just_saved = true;
                self.save_failure = None;
                self.content_changed = Some(root);
                l.refresh_diagnostics();
                self.error = None;
                // Written: the repair it was opened for is done. A failed Save keeps it.
                self.repair_link = None;
            }
            Err(e) => match read_disk(&root, &note.path) {
                // The file is there but cannot be read: the draft stays, the file keeps its bytes.
                Err(fault) => {
                    self.disk_change = Some(DiskChange {
                        text: None,
                        moved_to: None,
                        fault: Some(fault),
                    });
                }
                Ok(text) if e == NOTE_CHANGED.as_str() => {
                    self.disk_change = Some(DiskChange {
                        moved_to: None,
                        text,
                        fault: None,
                    });
                }
                // The disk refused the write: the draft's state slot says so with Save's reason.
                Ok(_) if refused.is_none() => {
                    self.save_failure = Some(SaveFailure {
                        owner: note.owner.key(),
                        path: note.path.clone(),
                        revision: self.draft_revision,
                        reason: e,
                        read_only: note_path(&root, &note.path)
                            .ok()
                            .and_then(|path| ReadOnly::of(&root, &path)),
                    });
                }
                // Save refused the draft before the write: the refusal is the draft's own reason,
                // shown with its state while that draft lives. A page error would outlive the
                // draft (round 26: it now shows in the same footer).
                Ok(_) => {}
            },
        }
    }

    /// Undo package choice (note §4, M-L0.4b): the campaign's choices go back to what they
    /// were, and the result leaves only when that write works. A file that cannot be read or
    /// written keeps the result and its Undo, with the error, so the DM can try again. Choices
    /// changed since (by the DM, in Campaign or another page) end it with the words that say
    /// so, as a reload would: Undo would undo those changes too.
    fn undo_package_use(&mut self) {
        /// Why the Undo wrote nothing.
        enum NotUndone {
            Changed,
            Failed(String),
        }
        let Some(undo) = self.kept.package_use() else {
            return;
        };
        let result = CampaignDocument::load(&undo.campaign)
            .map_err(NotUndone::Failed)
            .and_then(|mut document| {
                if document.selections != undo.after {
                    return Err(NotUndone::Changed);
                }
                document.selections = undo.before.clone();
                document.save().map_err(NotUndone::Failed)
            });
        match result {
            Ok(()) => {
                let path = undo.campaign.path.clone();
                let _ = self.kept.take_package_use();
                self.stale = true;
                self.content_changed = Some(path);
            }
            Err(NotUndone::Changed) => {
                let _ = self.kept.take_package_use();
                self.error = Some(PageError::Own(CHOICES_CHANGED.clone()));
            }
            Err(NotUndone::Failed(error)) => self.error = Some(error.into()),
        }
    }

    /// Undo write of a manifest: the file gets its old text back. A failure keeps the result
    /// and its Undo, with the error.
    fn undo_manifest(&mut self) {
        let Some(write) = self.kept.manifest() else {
            return;
        };
        match write.undo() {
            Ok(root) => {
                self.content_changed = Some(root);
                let _ = self.kept.take_manifest();
                self.stale = true;
            }
            Err(error) => self.error = Some(error.into()),
        }
    }

    /// `pane_right` is the pane's right edge, where the reader's body may end.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces, and the disk-change warnings above the comparison (design::results::STAGED_ALLOWS, ruling M20)"
    )]
    fn editor_pane(
        &mut self,
        ui: &mut egui::Ui,
        subtitle: &str,
        campaigns: &Campaigns,
        pane_right: Option<f32>,
    ) {
        // The pane's kept results. They sit in the pane's message footer, so a result never moves
        // the tabs, the file line, the title or the body.
        let mut results = ReaderResults {
            package_use: self.kept.package_use().is_some_and(|undo| {
                campaigns
                    .active_campaign()
                    .is_some_and(|c| c.id == undo.campaign.id)
            }),
            manifest: self.kept.manifest().is_some(),
            locked: if self.disk_change.is_some() {
                Some(&CHANGED_FILE_FIRST)
            } else if self.dirty() {
                Some(&UNDO_WAITS_FOR_DRAFT)
            } else {
                None
            },
            lock_can_come: self.source || self.dirty() || self.disk_change.is_some(),
            ..Default::default()
        };
        let mut undo_remove = false;
        if self.selected.is_none() {
            // With no entry open, the results and Remove's notice sit in the pane's message
            // footer, below the empty reader (round 26). Its id is the reader's, so one pane has
            // one footer. A typed footer (M-L0.4b): each kept line has its ticket.
            let frame = PaneFrame::new(ui);
            let mut messages = Messages::new(
                &frame,
                "library_footer",
                ui.available_width().min(design::READER_MEASURE),
            );
            if !self.dirty()
                && let Some(line) = self.kept.outcome(&frame, KeptKind::Removed, || {
                    self.outcome_of(KeptKind::Removed, &results)
                })
            {
                messages.kept(line);
            }
            self.page_messages(&frame, &mut messages, &results, campaigns);
            page_faults(&mut messages, &[self.error.as_ref()]);
            // The empty reader is the pane's body; the footer follows it (round 28 pane).
            let paned = messages.pane(ui, design::Body::Content, |ui| {
                design::empty_state(
                    ui,
                    icons::LIBRARY,
                    "No entry open",
                    if campaigns.active_campaign().is_some() || !campaigns.modules().is_empty() {
                        "Choose one from the list, or make a new one."
                    } else {
                        "Choose one from the list."
                    },
                    |_| {},
                );
            });
            // Undo remove acts here, as before; every other press after the pane is drawn, with
            // the reader's (`show`). Each by its line's ticket.
            match paned.pressed {
                Some(Press {
                    id: LibraryPress::Undo,
                    ticket: Some(ticket),
                }) if self.kept.kind_by(&ticket) == Some(KeptKind::Removed) => undo_remove = true,
                pressed => self.footer_press = pressed,
            }
        }
        let recovery = self
            .selected
            .and_then(|at| self.loaded.as_ref()?.notes.get(at))
            .filter(|note| note.is_manifest() && note.error.is_some())
            .and_then(|note| match &note.owner {
                Owner::Package(module) => {
                    let path = note.path.clone();
                    let exists = note.error.as_deref()
                        != Some(crate::content_workspace::DELETED_MANIFEST)
                        && module.path.join(&path).exists();
                    Some((module.clone(), path, exists))
                }
                _ => None,
            });
        if undo_remove
            && let Some((_, _, draft)) = self.kept.take_discarded()
            && let Some(LetGo {
                note, used, chosen, ..
            }) = self.kept.take_removed()
            && let Some(loaded) = self.loaded.as_mut()
        {
            // The reload after Remove entry and lose draft left the entry out of what this campaign uses and
            // counted the entries that need it. Undo gives back both with the draft.
            if used && let (Some(uses), Some(entry)) = (loaded.uses.as_mut(), note.entry.as_ref()) {
                uses.insert(entry.id);
            }
            if chosen
                && let (Some(set), Some(entry)) = (loaded.chosen.as_mut(), note.entry.as_ref())
            {
                set.insert(entry.id);
            }
            self.selected = Some(loaded.notes.len());
            loaded.notes.push(note);
            loaded.invalidate();
            loaded.refresh_diagnostics();
            let _ = self.draft.replace(draft);
            // The draft changed: a failed Save of the old words no longer owns it.
            self.draft_revision += 1;
            self.source = true;
            self.disk_change = Some(DiskChange {
                text: None,
                moved_to: None,
                fault: None,
            });
            self.invalidate();
            // This frame drew the empty reader's pane; the next one draws the entry.
            ui.ctx().request_repaint();
            return;
        }
        let Some(i) = self.selected else {
            return;
        };
        // Overwrite was chosen: what is on disk now becomes what this save replaces.
        let overwrite = std::mem::take(&mut self.overwrite_disk);
        let mut file_gone = false;
        if overwrite && let Some(change) = self.disk_change.take() {
            if let Some(path) = change.moved_to {
                self.loaded.as_mut().unwrap().notes[i].path = path;
                self.stale = true;
            }
            match change.text {
                Some(text) => {
                    self.loaded.as_mut().unwrap().notes[i].text =
                        crate::design::memo::Revised::new(text)
                }
                None => file_gone = true,
            }
            self.loaded.as_mut().unwrap().invalidate();
        }
        let draft_lock = self.draft_lock();
        let parsed_draft = self.parsed_draft(i);
        let parsed: &Result<Entry, String> = &parsed_draft;
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let reading = parsed
            .as_ref()
            .ok()
            .and_then(|entry| entry.reading().ok().flatten());
        // A file with no entry to read is titled by its file name; the file line names its path.
        let file_name = Path::new(&note.path)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned());
        let title = match &reading {
            Some(reading) => reading.title.as_str(),
            None if note.entry.is_none()
                && note.error.as_deref() != Some(crate::content_workspace::DELETED_MANIFEST) =>
            {
                file_name.as_deref().unwrap_or(note.name())
            }
            None => note.name(),
        };
        // A duplicate keeps its folder too: the two readers differ only there.
        let reading_subtitle = reader_subtitle(
            &note.owner,
            &row_line_without_fault(subtitle),
            reading
                .as_ref()
                .and_then(|_| presentation::class_level(parsed.as_ref().ok()?)),
            parsed
                .as_ref()
                .ok()
                .and_then(|entry| entry.fields.get("splat_spell"))
                .and_then(spell_kind),
        );
        let dirty = !self.saved_is_draft(note);
        let editable = note.owner.editable();
        // A file the system will not read has no text to edit: the reader names its fault.
        let fault = note.fault.clone();
        if fault.is_some() {
            self.source = false;
        }
        let conflict = self.disk_change.is_some();
        let mut duplicate_indices = self.loaded.as_ref().unwrap().duplicates(i);
        if !duplicate_indices.is_empty() {
            duplicate_indices.push(i);
            let notes = &self.loaded.as_ref().unwrap().notes;
            let first = crate::content_workspace::duplicate_repair_path(
                duplicate_indices.iter().filter_map(|&at| {
                    notes[at]
                        .owner
                        .root()
                        .map(|root| root.join(&notes[at].path))
                }),
            );
            duplicate_indices.sort_by_key(|&at| {
                notes[at]
                    .owner
                    .root()
                    .map(|root| root.join(&notes[at].path))
                    != first
            });
        }
        let mut new_identity = None;
        let mut new_identities = None;
        // Only a side-by-side comparison of draft and disk text takes the full width. A deleted
        // file's lone draft keeps the source editor's measure.
        let editor_width = if self
            .disk_change
            .as_ref()
            .is_some_and(|change| change.text.is_some())
        {
            ui.available_width()
        } else {
            ui.available_width().min(EDITOR_MAX_WIDTH)
        };
        self.just_saved &= !dirty && self.source;
        // Ctrl+S is taken only where a draft could be saved; it saves exactly when Save can
        // (`DraftState::can_save`, below).
        let save_key = design::save_row::save_key(ui, dirty && editable && !conflict);
        let d = colors().document;
        // The file stopped reading under the draft: its owner line says so, as for a file that
        // never read.
        let stopped_reading = self
            .disk_change
            .as_ref()
            .is_some_and(|change| change.fault.is_some());
        let about = note.entry.as_ref().is_some_and(|e| e.kind == "package");
        let mut use_every = None;
        let mut every_chosen = false;
        // Row 125: a fault on the owner line is typed, from the note's typed state (its file
        // fault, its parse, a deleted manifest), never from its error words. The kind gives the
        // status words and the tone (`outcome::fault_status`), as the row and the message have.
        let owner_fault = match &note.owner {
            Owner::Package(_) if stopped_reading => Some(FaultStatus::CannotRead),
            Owner::Package(_) if about => None,
            Owner::Package(_) if note.manifest_gone => Some(FaultStatus::ManifestDeleted),
            Owner::Package(_) | Owner::Campaign(_)
                if note.error.is_some() || note.fault.is_some() || stopped_reading =>
            {
                Some(FaultStatus::CannotRead)
            }
            _ => None,
        };
        let owner_name = match &note.owner {
            Owner::Package(module) => module.name.as_str(),
            _ => "Campaign",
        };
        let package_word = match &note.owner {
            Owner::Package(module) if let Some(fault) = owner_fault => fault.words(&module.name),
            Owner::Package(_) if about && campaigns.active_campaign().is_none() => {
                "No campaign open".into()
            }
            Owner::Package(module) if about => {
                let loaded = self.loaded.as_ref().unwrap();
                let entries: Vec<_> = loaded
                    .notes
                    .iter()
                    .filter(|n| {
                        n.owner.is_package(module.id)
                            && n.entry.as_ref().is_some_and(|e| {
                                !["package", "reference"].contains(&e.kind.as_str())
                            })
                    })
                    .collect();
                // Round 27 FA6: the DM's choices and the entries they need, as Campaign's
                // package row counts its ticked boxes.
                let chosen = entries.iter().filter(|n| loaded.chose(n)).count();
                let needed = entries
                    .iter()
                    .filter(|n| !loaded.chose(n) && loaded.in_scope(n, Scope::Uses))
                    .count();
                every_chosen = chosen == entries.len();
                chosen_count(chosen, needed, entries.len())
            }
            Owner::Package(module) => format!(
                "{} · {}",
                module.name,
                if self.loaded.as_ref().unwrap().in_scope(note, Scope::Uses) {
                    "Chosen for this campaign"
                } else {
                    "Not chosen"
                }
            ),
            _ => String::new(),
        };
        // A ready state means Save would accept the draft. Every draft with changes, and every
        // repair draft, asks Save's own check before the write (parse, identity, ownership):
        // the parse is cached, and the identity index is cached until the notes change.
        let refusal = (note.error.is_some() || dirty)
            .then(|| self.loaded.as_ref().unwrap().draft_refusal(i, parsed))
            .flatten();
        // Why Save refuses this draft now. It blocks Save and Ctrl+S, and the state is Invalid.
        let save_refusal = refusal.clone().filter(|_| dirty);
        let (repair_problem, repair_line) =
            draft_repair_problem(note, &self.draft, refusal, recovery.as_ref());
        // A missing link Campaign sent here: the row that holds it in the parsed draft.
        let here = note.owner.root().map(|root| root.join(&note.path));
        let missing_link = self
            .repair_link
            .as_mut()
            .filter(|link| here.as_ref() == Some(&link.path))
            .and_then(|link| {
                // Looked up again only for a new parse, that is, new draft text.
                let line = (*link.found.get_or(parsed_draft.revision(), || {
                    parsed
                        .as_ref()
                        .ok()
                        .and_then(|entry| repair_row(entry, &link.target))
                }))?;
                // The content layer's words; the fault adds the entry, the line and the fix.
                Some((line, link.words.clone()))
            });
        let repair_line = repair_line.or(missing_link.as_ref().map(|(line, _)| *line));
        // A Save that failed on the disk, while this is still the draft it tried to write.
        if let Some(failure) = &self.save_failure {
            #[cfg(any(test, feature = "test-support"))]
            {
                let (checks, reads) = self.failure_checks.get();
                self.failure_checks.set((checks + 1, reads));
            }
            let draft = DraftView::new(
                &self.draft,
                #[cfg(any(test, feature = "test-support"))]
                &self.failure_checks,
            );
            if !failure.owns(note, self.draft_revision, draft) {
                self.save_failure = None;
            }
        }
        let failed = self
            .save_failure
            .as_ref()
            .map(|failure| failure.reason.clone());
        // The save row's state. Its word, Save's weight and why Save waits all come from it
        // (`design::save_row`). A parse fault ends with the draft that parses: the saved file
        // stays unreadable on the owner line until Save, but the draft is not Invalid.
        let mut draft_state = design::DraftState::new(
            design::save_row::DraftOrigin::Saved, // New entry writes the file before it opens
            if dirty {
                design::save_row::Changed::Yes
            } else {
                design::save_row::Changed::No
            },
            if self.run_open {
                design::save_row::Primary::Elsewhere
            } else {
                design::save_row::Primary::Draft
            },
        );
        if let Some(refusal) = &save_refusal {
            draft_state = draft_state.check(design::save_row::Diagnosis::refused(refusal.as_str()));
        }
        // Lead ruling (s30 B11 F4, R41): a deleted manifest is a missing file, not unreadable
        // text: its warning stays on the owner line and in the footer, and a draft whose text
        // Save accepts says the ready word (`Unsaved`), never `Invalid`. Any other repair
        // problem is unreadable data: `Invalid`, an error. Neither blocks Save (B3's contract).
        let manifest_gone = fault.is_none()
            && note.error.is_some()
            && recovery.as_ref().is_some_and(|(_, _, exists)| !exists);
        if (fault.is_some() || repair_problem.is_some()) && !manifest_gone {
            draft_state = draft_state.check(design::save_row::Diagnosis::unreadable());
        }
        if conflict {
            draft_state = draft_state.changed_on_disk();
        }
        if failed.is_some() {
            // An action-time failure: the disk refused the last Save of this draft.
            draft_state = draft_state.not_saved();
        }
        if self.just_saved {
            draft_state = draft_state.just_saved();
        }
        // Overwrite was chosen in the comparison: it saves whatever the draft holds.
        let mut save = overwrite || (save_key && draft_state.can_save());
        let mut step = history::take_if(ui.ctx(), |_| true);
        let can_copy = note
            .entry
            .as_ref()
            .is_some_and(|e| !["package", "campaign", "reference"].contains(&e.kind.as_str()));
        let can_make = matches!(note.owner, Owner::Campaign(_))
            && note.entry.as_ref().is_some_and(|e| e.kind == "creature");
        // Round 25 FF3: the campaign choice is a toolbar action, so it never sends the file line's
        // path to a second row: the title keeps one place for chosen and not-chosen entries.
        let choose = match &note.owner {
            Owner::Package(_)
                if about && campaigns.active_campaign().is_some() && !every_chosen =>
            {
                Some("Choose all")
            }
            Owner::Package(_)
                if !about
                    && note.entry.is_some()
                    && !self.loaded.as_ref().unwrap().in_scope(note, Scope::Uses) =>
            {
                Some("Choose for this campaign")
            }
            _ => None,
        };
        // Each control waits for one cause, from ONE value: the open draft (its save row, or the
        // line under the toolbar when there is none, paints the reason), `Copy to…`'s places
        // policy that it shares with `New entry` (painted under the toolbar), or, for the mode
        // switch, the unreadable file that the body states.
        let copy_availability = match Places::of(campaigns).copy() {
            Some(reason) => design::Availability::Waits(reason),
            None => design::Availability::Draft,
        };
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Extra {
            Copy,
            Make,
            Choose,
        }
        let edit_label = if editable {
            "Edit Markdown"
        } else {
            "Markdown source"
        };
        let was_source = self.source;
        let blocks = self.draft_blocks();
        let mut toolbar = design::ReaderToolbar::new(
            self.source,
            edit_label,
            "Markdown source",
            match &fault {
                Some(_) => design::ModeLock::BodyStatesFault(UNREAD_EDIT),
                None => design::ModeLock::Open,
            },
        )
        .back(
            self.trail.peek(Step::Back).is_some(),
            "Return to the entry before this link. During a draft, read it in Lookup.",
        );
        // M0355 (round 31 reader-toolbar-icon-form): every toolbar action has one form, its
        // words; Back is navigation and keeps its arrow.
        if can_copy {
            toolbar = toolbar.action("Copy to…", Extra::Copy, copy_availability);
        }
        if can_make {
            toolbar = toolbar.action("Make NPC note", Extra::Make, design::Availability::Draft);
        }
        if let Some(label) = choose {
            toolbar = toolbar.action(label, Extra::Choose, design::Availability::Draft);
        }
        // The toolbar, the file line between, and the save row with the draft's reason are one
        // part's draw: it paints the reason itself, on the save row's line or under the toolbar.
        let row_for = |source: bool| {
            if editable && (source || dirty) && !conflict {
                design::DraftRow::Save {
                    label: "Save",
                    width: editor_width,
                }
            } else {
                design::DraftRow::None
            }
        };
        let file_line = |ui: &mut egui::Ui| {
            if let Some(root) = note.owner.root() {
                let moved_path = self
                    .disk_change
                    .as_ref()
                    .and_then(|change| change.moved_to.as_deref());
                let shown_path = moved_path.unwrap_or(&note.path);
                let moved_from = moved_path.map(|_| {
                    format!(
                        "moved from {}/",
                        Path::new(&note.path)
                            .parent()
                            .unwrap_or(Path::new(""))
                            .display()
                    )
                });
                // No editor link for a file that cannot be read, whether it never read or stopped
                // reading under a draft: an editor cannot open it either.
                let unread = fault.is_some()
                    || self
                        .disk_change
                        .as_ref()
                        .is_some_and(|change| change.fault.is_some());
                let obsidian = obsidian_for(root, &note.owner) && !unread;
                // The draft's file has gone from disk: no link can show it, so the line offers its
                // folder, as a deleted manifest's does.
                let file_gone_now = self.disk_change.as_ref().is_some_and(|change| {
                    change.text.is_none() && change.moved_to.is_none() && change.fault.is_none()
                });
                let deleted_manifest = recovery.as_ref().is_some_and(|(_, _, exists)| !exists);
                let file_line = |ui: &mut egui::Ui| {
                    if deleted_manifest {
                        show_folder(ui, root);
                    } else if file_gone_now {
                        gone_file_parts(ui, root, shown_path);
                    } else if let Some(from) = &moved_from {
                        // Round 27 FD7: one statement about the path, then its links: the new
                        // path, the muted old folder, then `Show file`.
                        moved_file_parts(ui, root, shown_path, from, obsidian);
                    } else {
                        crate::world::entry_file_parts(ui, root, shown_path, obsidian);
                    }
                };
                // FL1: once Save has found the file or its folder read only, the owner line says so,
                // as long as that failure lasts (the next edit, another entry or a Save ends it).
                let read_only = self
                    .save_failure
                    .as_ref()
                    .is_some_and(|failure| failure.read_only.is_some())
                    && note.error.is_none()
                    && !stopped_reading;
                let read_only_word;
                let campaign_fault_word;
                let owner_word = match &note.owner {
                    Owner::Package(module) if read_only => {
                        read_only_word = format!("{} · Read only", module.name);
                        read_only_word.as_str()
                    }
                    Owner::Package(_) => package_word.as_str(),
                    Owner::Campaign(_) if let Some(fault) = owner_fault => {
                        campaign_fault_word = fault.words(owner_name);
                        campaign_fault_word.as_str()
                    }
                    _ if read_only => "Campaign · Read only",
                    _ => "Campaign · Editable",
                };
                let file_below = ui.available_width()
                    < design::number_width(ui, owner_word, &Type::Small.font(ui))
                        + design::number_width(ui, "Show file", &Type::Body.font(ui))
                        + design::XXL * 2.0
                        + if obsidian {
                            design::number_width(ui, "Edit in Obsidian", &Type::Body.font(ui))
                        } else {
                            0.0
                        };
                // The path shortens from the front and keeps its file name whole
                // (`…/Cinder Wolf copy.md`). When even that does not fit after the owner words and
                // their action, the path takes its own Read row: it is measured where it would start.
                let filename = Path::new(shown_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                let shortest_path = if filename == *shown_path {
                    filename.to_string()
                } else {
                    format!("…/{filename}")
                };
                let path_room = if deleted_manifest {
                    design::number_width(ui, "Show folder", &Type::Body.font(ui))
                } else {
                    design::number_width(ui, &shortest_path, &Type::Small.font(ui))
                        + design::L
                        + moved_from.as_ref().map_or(0.0, |from| {
                            design::number_width(ui, from, &Type::Small.font(ui)) + design::S
                        })
                        + if file_gone_now {
                            design::number_width(ui, "Show folder", &Type::Body.font(ui))
                        } else {
                            crate::world::entry_file_links_width(ui, obsidian)
                        }
                };
                let mut path_below = false;
                let owner_line = |ui: &mut egui::Ui| {
                    // In a narrow reader (Lookup open beside it at 960), the campaign choice keeps
                    // its first word rather than a cut: `Ashen Marches · Chosen`.
                    let short_word;
                    let owner_word = match owner_word.strip_suffix(" for this campaign") {
                        Some(short)
                            if design::number_width(ui, owner_word, &Type::Small.font(ui))
                                + design::XL
                                > ui.available_width() =>
                        {
                            short_word = short.to_owned();
                            short_word.as_str()
                        }
                        _ => owner_word,
                    };
                    if let Some(fault) = owner_fault {
                        design::outcome::fault_status(ui, owner_name, fault);
                    } else {
                        design::status(
                            ui,
                            if read_only {
                                Status {
                                    icon: icons::READ_ONLY,
                                    word: owner_word,
                                    color: d.read_only,
                                }
                            } else {
                                Status {
                                    icon: icons::EDIT,
                                    word: owner_word,
                                    color: d.clean,
                                }
                            },
                        );
                    }
                    if !file_below {
                        if ui.available_width() >= path_room {
                            file_line(ui);
                        } else {
                            path_below = true;
                        }
                    }
                };
                file_line_row(ui, file_below, owner_line);
                if file_below || path_below {
                    file_line_row(ui, false, file_line);
                }
            } else {
                file_line_row(ui, false, |ui| {
                    design::status(
                        ui,
                        Status {
                            icon: icons::READ_ONLY,
                            word: "Built-in · Read only",
                            color: d.read_only,
                        },
                    );
                });
            }
        };
        let shown = toolbar.show(ui, &draft_state, blocks, row_for, file_line);
        let toolbar_press = shown.press;
        self.source = shown.source;
        if self.source != was_source {
            let _ = self.kept.take_scope_notice();
            self.focus_repair = self.source && note.error.is_some();
        }
        // The mode's flags, after the toggle above: the pass that opens Edit Markdown draws the
        // editor's pane (its footer identity and scroll) from the start, not Read's.
        // Read: the reader's messages sit in the pane's message footer, below the body.
        let choices_in_reader = !self.source && !conflict;
        let save_row = editable && (self.source || dirty);
        // Edit Markdown's save row, with the pane's message footer below the editor.
        let slot_row = save_row && !conflict;
        results.identities = self.kept.identities().is_some();
        results.identity = self.kept.identity().is_some();
        let mut repair_pressed = false;
        let mut read_again = false;
        // The heading warning and its Undo. Read and Edit Markdown both draw them in the pane's
        // message footer, with the duplicate warning, so one file's warnings share one place and
        // the body never moves.
        let heading = HeadingChoice::new(
            (editable && !conflict)
                .then(|| parsed.as_ref().ok())
                .flatten(),
            &self.draft,
            self.kept.heading(),
            &(note.owner.key(), note.path.clone()),
            &self.same_heading,
        );
        // Read's messages for the pane's message footer: a package entry's links to entries in
        // this campaign only, each with its copy action, and the copy's result.
        let dependencies: Vec<(Uuid, Sentence, (String, String))> = match (&note.owner, &note.entry)
        {
            (Owner::Package(package), Some(entry)) if choices_in_reader => ui
                .ctx()
                .data(|data| {
                    data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                        "reader_link_index",
                    ))
                })
                .map(|index| {
                    index
                        .campaign_dependencies(entry)
                        .into_iter()
                        .map(|at| {
                            let dependency = &index.items[at].entry;
                            (
                                dependency.id,
                                // Row 145 (FC6): say what the link is and why it matters.
                                crate::sentence!(
                                    "{} links to {}, which is in this campaign only. Another campaign that uses {} will not find it.",
                                    entry.name,
                                    dependency.name,
                                    package.name
                                ),
                                (dependency.name.clone(), package.name.clone()),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        // What Edit Markdown's footer shows, gathered with the save row. The footer is built from
        // it before the body and drawn by the pane after the body (`Messages::pane_editor`, round
        // 28 P2); its press is applied after the pane returns.
        struct EditData {
            vital_result: Option<Sentence>,
            vitals: Vec<(&'static str, &'static str, i64, Sentence)>,
            broken_cache: Vec<Sentence>,
            broken_taken: bool,
            reason_tone: Tone,
        }
        let mut edit_data = None;
        // Save and Discard are one row, as far apart as the pane allows.
        if slot_row {
            let row = shown.save.unwrap_or_default();
            save |= row.save;
            if row.discard {
                Self::discard_draft(
                    (
                        &mut self.kept,
                        &mut self.draft,
                        &mut self.draft_revision,
                        &mut self.error,
                    ),
                    note,
                );
            }
            // The pane's message footer, below the editor: `S` under a short draft, pinned under
            // a long one, whose editor region scrolls (round 28 P2).
            // Results that carry a live Undo come first (Discard's, the heading and stat fixes',
            // the reader's results with Undo): a broken file's reason never clears after Discard
            // or Reload, so a reason first would hide the only way back. Then Save's reason (a
            // repair draft's parse error with its recovery, a refusal before the write, the
            // disk's failure). Then the notices: results with no Undo, the missing link Campaign
            // sent here, the other broken links, the first-heading and stat warnings. Why other
            // actions wait is painted by `design::ReaderToolbar`, never by the footer.
            let vital_result = self.stat_result_for(&(note.owner.key(), note.path.clone()));
            let vitals = match (editable, &parsed) {
                (true, Ok(entry)) => creature::vital_checks(entry),
                _ => Vec::new(),
            };
            // Round 25 FH6: each broken link the row counts has its own line here, so the DM
            // knows where every one is. The link Campaign sent here has its own words.
            let index = ui.ctx().data(|data| {
                data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                    "reader_link_index",
                ))
            });
            // Borrowed out of the cache for this frame and put back after the footer is drawn.
            let mut broken_taken = false;
            let broken_cache: Vec<Sentence> = match (&index, parsed) {
                (Some(index), Ok(entry)) if note.owner.editable() && !note.is_manifest() => {
                    let skip = missing_link.as_ref().map(|(at, _)| *at);
                    let fresh = self
                        .broken_lines
                        .as_ref()
                        .is_some_and(|(seen, known, at, _)| {
                            *seen == parsed_draft.revision()
                                && *known == index.generation()
                                && *at == skip
                        });
                    if !fresh {
                        let warnings = broken_link_warnings(&self.draft, entry, index, skip);
                        self.broken_lines =
                            Some((parsed_draft.revision(), index.generation(), skip, warnings));
                    }
                    broken_taken = true;
                    self.broken_lines
                        .as_mut()
                        .map_or_else(Default::default, |(_, _, _, warnings)| {
                            std::mem::take(warnings)
                        })
                }
                _ => Default::default(),
            };
            let reason_tone = recovery
                .as_ref()
                .filter(|_| note.error.is_some())
                .map_or(Tone::Error, |(_, _, exists)| {
                    crate::content_workspace::manifest_fault_tone(*exists)
                });
            edit_data = Some(EditData {
                vital_result,
                vitals,
                broken_cache,
                broken_taken,
                reason_tone,
            });
        }
        // The toolbar's press, now that the draft's reason is painted.
        if toolbar_press == Some(design::ToolbarPress::Back) {
            step = Some(Step::Back);
        }
        let extra = match toolbar_press {
            Some(design::ToolbarPress::Action(extra)) => Some(extra),
            _ => None,
        };
        let copy_clicked = matches!(extra, Some(Extra::Copy));
        let make_clicked = matches!(extra, Some(Extra::Make));
        let choose_clicked = matches!(extra, Some(Extra::Choose));
        if choose_clicked && let Owner::Package(module) = &note.owner {
            if about {
                use_every = Some(module.clone());
            } else if let Some(entry) = &note.entry {
                self.use_entry = Some((module.id, entry.id));
            }
        }
        if copy_clicked {
            self.copy = Some(crate::content_workspace::CopySource {
                entry: note.entry.as_ref().unwrap().clone(),
                text: note.text.to_string(),
                root: note.owner.root().map(Path::to_path_buf),
                campaign: match &note.owner {
                    Owner::Campaign(campaign) => Some(campaign.clone()),
                    _ => None,
                },
            });
        }
        if make_clicked && let (Owner::Campaign(campaign), Some(entry)) = (&note.owner, &note.entry)
        {
            match crate::world::make_npc_note(campaign, entry) {
                Ok(id) => {
                    self.stale = true;
                    self.content_changed = Some(campaign.path.clone());
                    self.open_world_npc = Some(id);
                }
                Err(error) => self.error = Some(error.into()),
            }
        }
        // Live, because Discard in this same frame changed the draft.
        if !self.saved_is_draft(note) {
            let _ = self.kept.take_discarded();
        }
        // The pane's footer, typed (M-L0.4b): Edit Markdown's (with a save row), Read's, or a
        // comparison's. Built before the body from the stores, each kept result with its ticket;
        // the pane draws the body and then it (`Messages::pane`, `pane_editor`), so it follows a
        // short body, and its one press is read after the pane (round 28, row G6 and P2). Keyed
        // by the entry, as the old body scroll was: each entry keeps its own offset.
        let frame = PaneFrame::new(ui);
        let mut messages = Messages::new(
            &frame,
            (
                if self.source {
                    "library_source"
                } else {
                    "library_footer"
                },
                note.owner.key(),
                note.path.clone(),
            ),
            if edit_data.is_some() || self.source {
                editor_width
            } else {
                editor_width.min(design::READER_MEASURE)
            },
        );
        let undo = |step: UndoStep| step.pressed_as(LibraryPress::Undo);
        // Kept results with a live Undo: Discard's, the heading and stat fixes', each copy.
        if discard_is_for(self.kept.discarded(), note)
            && let Some(line) = self.kept.outcome(&frame, KeptKind::Discarded, || {
                Some(KeptOutcome::Notice(
                    Notice::new(crate::sentence!("{} draft discarded.", note.name())),
                    undo(UndoStep::of(Verb::Discard, note.name())),
                ))
            })
        {
            messages.kept(line);
        }
        if let Some(words) = &heading.undo
            && let Some(line) = self.kept.outcome(&frame, KeptKind::Heading, || {
                Some(
                    Said::new(words.clone())
                        .undo(undo(HEADING_CHANGE_UNDO))
                        .into(),
                )
            })
        {
            messages.kept(line);
        }
        if let Some(words) = edit_data
            .as_ref()
            .and_then(|data| data.vital_result.clone())
            && let Some(line) = self.kept.outcome(&frame, KeptKind::Vital, || {
                Some(Said::new(words).undo(undo(STAT_CHANGE_UNDO)).into())
            })
        {
            messages.kept(line);
        }
        // Round 25 FF8: each copy's result names the entry and the package, in its Undo's words
        // too (ruling M9).
        for copy in self.kept.copies().iter().rev().filter(|copy| {
            choices_in_reader
                && copy.path == note.path
                && note.owner.root() == Some(copy.root.as_path())
        }) {
            if let Some(line) = self.kept.copy_outcome(&frame, copy, || {
                Said::new(crate::sentence!(
                    "{} copied to {} and linked.",
                    copy.name,
                    copy.package
                ))
                .undo(undo(UndoStep::of(Verb::Copy, copy.name.as_str())))
                .into()
            }) {
                messages.kept(line);
            }
        }
        // The page's results and this pane's kept results.
        self.page_messages(&frame, &mut messages, &results, campaigns);
        if let Some(data) = &edit_data {
            // A repair draft's fault in its tone (a deleted manifest is a warning), or Save's
            // refusal before the write, a YAML fault in its own words. A broken manifest's
            // recovery depends on the saved file only, so it stays while the DM edits.
            let repair = recovery.as_ref().filter(|_| note.error.is_some());
            let problem = repair_problem
                .as_deref()
                .or(repair.and(note.error.as_deref()));
            if let Some(problem) = problem {
                let fault = FaultMessage::new(&LibraryFault::Repair(problem, data.reason_tone));
                messages.outcome(match repair {
                    Some((_, path, exists)) => {
                        fault.choice(recovery_choice(path, *exists, self.draft_lock()))
                    }
                    None => fault,
                });
            }
            // The disk's failure of the last Save of this draft.
            if let Some(failure) = &failed {
                messages.outcome(FaultMessage::new(&LibraryFault::NotSaved(failure)));
            }
            // The repair this note was opened for.
            if let Some((line, words)) = &missing_link {
                messages.outcome(FaultMessage::new(&LibraryFault::Link {
                    line: *line,
                    words,
                }));
            }
            for words in &data.broken_cache {
                messages.outcome(Notice::warning(words.clone()));
            }
            for (key, label, value, words) in &data.vitals {
                messages.outcome(Notice::warning(words.clone()).choice(Choice::named(
                    crate::choice!("Use {}", value),
                    LibraryPress::UseVital(key, label, *value),
                )));
            }
        }
        if let Some((words, name, _)) = &heading.fix {
            messages.outcome(Notice::warning(words.clone()).choice(Choice::named(
                crate::choice!("Use {}", name),
                LibraryPress::FixHeading,
            )));
        }
        if !duplicate_indices.is_empty() {
            messages.outcome(duplicate_notice(
                ui,
                self.loaded.as_ref().unwrap(),
                note,
                &duplicate_indices,
                self.draft_lock(),
            ));
        }
        // A package entry's links to entries in this campaign only, each with its copy action.
        for (id, words, (name, package)) in &dependencies {
            let choice = Choice::named(
                crate::choice!("Copy {} to {}", name, package),
                LibraryPress::CopyDependency(*id),
            );
            messages.outcome(
                Notice::warning(words.clone()).choice(match self.draft_lock() {
                    Some(reason) => choice.blocked(reason),
                    None => choice,
                }),
            );
        }
        page_faults(&mut messages, &[self.error.as_ref()]);
        let mut messages = Some(messages);
        let mut footer_pressed = None;
        let mut reload_from_disk = false;
        let mut read_disk_again = false;
        if let Some(fault) = self
            .disk_change
            .as_ref()
            .and_then(|change| change.fault.clone())
        {
            // The file is there but cannot be read. Saving the draft over it would replace bytes
            // Splat never read, so the choices are to read it again or to let the draft go
            // (M20: a typed placeholder; its red choice `L` after the safe one).
            let choice = at_reader_measure(ui, |ui| {
                PagePlaceholder::new(&LibraryFault::Unread(&fault, true))
                    .choice(Choice::new(
                        MessageChoice::Choice("Try again"),
                        DiskChoice::Again,
                    ))
                    .choice(Choice::new(
                        MessageChoice::Lose("Reload and lose my draft"),
                        DiskChoice::Reload,
                    ))
                    .show(ui)
            });
            design::gap(ui, design::Gap::S);
            design::quiet_section(ui, "Technical details", |ui| {
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, &fault.system).color(colors().muted))
                        .wrap(),
                );
            });
            read_disk_again = choice == Some(DiskChoice::Again);
            reload_from_disk = choice == Some(DiskChoice::Reload);
        } else if let Some(change) = &mut self.disk_change {
            // R10 (G16, G164, G165): a changed entry is named, as Encounters and World name
            // theirs; `This entry` only for an entry with no name.
            let names = if change.text.is_none() {
                self.loaded.as_ref().unwrap().dependents(note)
            } else {
                DependentNames::default()
            };
            // The entries that need a deleted one follow its words, so the choices keep one
            // line: the safe one, then the red one `L` after it.
            let text = if change.moved_to.is_some() {
                MOVED_ON_DISK
            } else if change.text.is_some() {
                changed_on_disk(note.entry.as_ref().map_or("", |entry| &entry.name))
            } else if names.is_empty() {
                DELETED_ON_DISK
            } else {
                DELETED_ON_DISK.then(crate::sentence!("{} needs this entry.", name_list(&names)))
            };
            // The choices start at the message text, as in World and the run.
            let save = match (&change.moved_to, &change.text) {
                (Some(path), _) => Choice::named(
                    crate::choice!("Save my draft to {}", path),
                    DiskChoice::Overwrite,
                ),
                (None, None) => Choice::new(
                    MessageChoice::Choice("Save my draft again"),
                    DiskChoice::Overwrite,
                ),
                (None, Some(_)) => Choice::new(
                    MessageChoice::Choice("Overwrite with my draft"),
                    DiskChoice::Overwrite,
                ),
            };
            let lose = Choice::new(
                MessageChoice::Lose(if change.text.is_none() {
                    "Remove entry and lose draft"
                } else {
                    "Reload and lose my draft"
                }),
                DiskChoice::Reload,
            );
            let choice = PagePlaceholder::new(&LibraryFault::Warning(&text))
                .choice(save)
                .choice(lose)
                .show(ui);
            let (overwrite, reload) = (
                choice == Some(DiskChoice::Overwrite),
                choice == Some(DiskChoice::Reload),
            );
            self.overwrite_disk |= overwrite;
            reload_from_disk = reload;
        }
        let mut open_link = None;
        let mut pressed_link = None;
        if self.source && self.disk_change.is_none() {
            // Edit Markdown (or a built-in's source): the editor is the pane's editor region, as
            // tall as its text while the pane fits, and scrolling inside the region when it does
            // not, so the footer under it stays in view (round 28 P2). With no save row the
            // footer is empty, and the pane is still the editor's.
            let footer = messages.take().expect("the footer is built above");
            footer_pressed = footer
                .pane_editor(ui, |ui, region| {
                    // The region draws the editor's fixed frame, with the text and its lane
                    // inside it (layout PR 9).
                    ui.scope(|ui| {
                        ui.set_max_width(editor_width);
                        let output = region.editor(
                            ui,
                            note.markdown_field(),
                            &mut self.draft,
                            design::pane::Face::Font(
                                egui::TextStyle::Monospace.resolve(ui.style()),
                            ),
                            |edit| edit.font(egui::TextStyle::Monospace).interactive(editable),
                        );
                        // Each edit is a new draft: a failed Save of the old words ends.
                        if output.response.changed() {
                            self.draft_revision += 1;
                        }
                        pressed_link = crate::markdown_source::pressed_note_link(
                            ui,
                            &output,
                            note.entry.as_ref().map(|entry| entry.id),
                        );
                        let response = output.response;
                        design::named(ui, &response, "Markdown editor");
                        if std::mem::take(&mut self.focus_markdown) {
                            response.request_focus();
                        }
                        // The caret goes to the broken line; the region reveals a caret that
                        // moved outside its view, centred (P2).
                        if std::mem::take(&mut self.focus_repair)
                            && let Some(line) = repair_line
                        {
                            let at = self
                                .draft
                                .lines()
                                .take(line.saturating_sub(1))
                                .map(|line| line.chars().count() + 1)
                                .sum();
                            response.request_focus();
                            let mut state = egui::TextEdit::load_state(ui.ctx(), response.id)
                                .unwrap_or_default();
                            state
                                .cursor
                                .set_char_range(Some(egui::text::CCursorRange::one(
                                    egui::text::CCursor::new(at),
                                )));
                            state.store(ui.ctx(), response.id);
                        }
                        if std::mem::take(&mut self.focus_hp)
                            && let Some(at) = self
                                .draft
                                .find("initiative: ")
                                .map(|at| at + "initiative: ".len())
                        {
                            response.request_focus();
                            let at = self.draft[..at].chars().count();
                            let mut state = egui::TextEdit::load_state(ui.ctx(), response.id)
                                .unwrap_or_default();
                            state
                                .cursor
                                .set_char_range(Some(egui::text::CCursorRange::two(
                                    egui::text::CCursor::new(at),
                                    egui::text::CCursor::new(at + 1),
                                )));
                            state.store(ui.ctx(), response.id);
                        }
                    });
                })
                .pressed;
        } else {
            // The body's words, in Read (in the pane) and beside an open conflict (in its own
            // scroll area, as Edit Markdown's form keeps until the pane part P2).
            let right = pane_right
                .filter(|_| self.disk_change.is_none())
                .unwrap_or(ui.max_rect().right())
                .max(ui.max_rect().right());
            let mut content = |ui: &mut egui::Ui| {
                if let Some(change) = &mut self.disk_change {
                    if let Some(text) = &change.text {
                        // The side-by-side comparison is the one state that takes the full width.
                        ui.set_max_width(editor_width);
                        design::grouped_draft_comparison(
                            ui,
                            &self.draft,
                            text,
                            (&note.path, text),
                            true,
                        );
                    } else {
                        design::heading(
                            ui,
                            design::Heading::Group {
                                title: "Your draft",
                                count: 0,
                            },
                        );
                        let mut draft = self.draft.as_str();
                        // The field belongs to this note: its owner and path key it.
                        let field = DISK_DRAFT.of(note.owner.id()).with(note.field_key());
                        let response = field.multi_line(ui, &mut draft, |edit| {
                            edit.font(egui::TextStyle::Monospace)
                                .margin(egui::vec2(design::S, design::S))
                                .desired_width(editor_width - 2.0 * design::S)
                                .interactive(false)
                        });
                        design::named(ui, &response, "Your draft");
                    }
                    return;
                }
                // A failed parse must never render YAML properties as book prose.
                let body = parsed.as_ref().map_or("", |e| e.body.as_str());
                ui.push_id(&note.path, |ui| {
                    reader_style::measure(ui, |ui| {
                        design::gap(ui, design::Gap::XS);
                        let reading_subtitle = reader_kind_line(&note.owner, &reading_subtitle);
                        let subtitle = reading_subtitle.as_str();
                        let subtitle = subtitle.strip_suffix(" · Campaign").unwrap_or(subtitle);
                        let subtitle = if note.error.is_some() {
                            match note.unreadable_kind() {
                                Section::Unreadable | Section::Other => "",
                                Section::PackageDetails => "Package",
                                kind => kind.label().trim_end_matches('s'),
                            }
                        } else if subtitle == "Campaign" {
                            note.section().label().trim_end_matches('s')
                        } else {
                            subtitle
                        };
                        view::title_with_art(ui, title, subtitle, parsed.as_ref().ok());
                        // Round 27 FH9: `L` between the kind line and its
                        // first warning, as between a kind line and prose,
                        // so the kind word does not read as the warning's.
                        if fault.is_some() || repair_problem.is_some() {
                            design::gap_to_next(ui, design::Gap::L);
                        }
                        if let Some(fault) = &fault {
                            read_again = fault_notice(ui, fault);
                        }
                        // A read fault has no repair problem: there is no text to repair.
                        if let Some(problem) = &repair_problem {
                            repair_pressed = repair_notice(
                                ui,
                                problem,
                                recovery.as_ref(),
                                draft_lock.clone(),
                                false,
                            );
                        }
                    });
                    open_link = view::show_with_related(
                        ui,
                        note.name(),
                        parsed.as_ref().ok(),
                        body,
                        &view::Origin {
                            path: &note.path,
                            owner: &note.owner.label(),
                            builtin: note.owner == Owner::Builtin,
                            // The draft's parse holds the entry, its body and its name.
                            revision: egui::Id::new(parsed_draft.revision()).value(),
                        },
                        self.loaded.as_ref().unwrap(),
                        // The Library draws no value block above the entry.
                        false,
                        // A package entry's campaign links sit in the
                        // pane's message footer (round 26).
                        |_| None,
                    );
                });
            };
            // The body takes the whole pane, so a wide table can use it; prose keeps its
            // measure. Read (with or without a changed draft) and an open conflict: the pane draws
            // the body, then its message footer `S` below a short body, or pinned when the body
            // scrolls (round 28 rows 17, 50, 53, 54; G6; P2 for the draft's footer).
            let footer = messages.take().expect("the footer is built above");
            let body = egui::Rect::from_min_max(
                ui.cursor().min,
                egui::pos2(right, ui.max_rect().bottom()),
            );
            ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
                crate::markdown_view::with_wide_tables(ui, |ui| {
                    footer_pressed = footer
                        .pane(ui, design::Body::Content, |ui| content(ui))
                        .pressed;
                });
            });
        }
        // The footer's one press, after the pane drew it. A kept result's acts by its line's
        // ticket: on the kind its ticket names now, in this frame.
        let (mut discard_undo, mut heading_undo, mut vital_undo) = (false, false, false);
        let (mut heading_fix, mut vital_use) = (false, None);
        let (mut undo_identities, mut undo_identity) = (false, false);
        let (mut copy_dependency, mut undo_dependency) = (None, None);
        if let Some(press) = footer_pressed {
            let kind = press
                .ticket
                .as_ref()
                .and_then(|ticket| self.kept.kind_by(ticket));
            match (press.id, kind) {
                (LibraryPress::Undo, Some(KeptKind::Discarded)) => discard_undo = true,
                (LibraryPress::Undo, Some(KeptKind::Heading)) => heading_undo = true,
                (LibraryPress::Undo, Some(KeptKind::Vital)) => vital_undo = true,
                (LibraryPress::Undo, Some(KeptKind::Identities)) => undo_identities = true,
                (LibraryPress::Undo, Some(KeptKind::Identity)) => undo_identity = true,
                (LibraryPress::Undo, Some(KeptKind::Copy)) => {
                    undo_dependency = press
                        .ticket
                        .as_ref()
                        .and_then(|ticket| self.kept.copy_at(ticket));
                }
                (LibraryPress::FixHeading, _) => heading_fix = true,
                (LibraryPress::UseVital(key, label, value), _) => {
                    vital_use = Some((key, label, value));
                }
                (LibraryPress::Repair, _) => repair_pressed = true,
                (LibraryPress::SeparateEntry(at), _) => new_identity = Some(at),
                (LibraryPress::SeparateEntries(at), _) => {
                    let loaded = self.loaded.as_ref().unwrap();
                    let owner = &loaded.notes[at].owner;
                    new_identities = loaded
                        .duplicate_folder_paths(owner)
                        .zip(owner.root().map(Path::to_path_buf))
                        .map(|(paths, root)| (root, paths));
                }
                (LibraryPress::CopyDependency(id), _) => copy_dependency = Some(id),
                // The page's results and the package's: after the pane, in `show`.
                _ => self.footer_press = Some(press),
            }
        }
        if let Some(data) = edit_data
            && data.broken_taken
            && let Some((_, _, _, warnings)) = &mut self.broken_lines
        {
            *warnings = data.broken_cache;
        }
        let draft_before = self.draft_revision;
        // A press acts on the draft as it is now, not as the footer saw it: the body may have
        // changed it in this same frame (a key and a screen reader's activation in one frame).
        // A fix is computed again from the current text; an Undo acts only while the draft is
        // still the text its action left, else it does nothing and its entry stays (its line
        // comes back with its own draft). Typed text is never replaced.
        if discard_undo
            && self.saved_is_draft(note)
            && let Some((_, _, text)) = self.kept.take_discarded()
        {
            let _ = self.draft.replace(text);
            // The draft changed: a failed Save of the old words no longer owns it.
            self.draft_revision += 1;
        }
        let current = (heading_fix || vital_use.is_some())
            .then(|| Entry::parse(&note.path, &self.draft).ok())
            .flatten()
            .filter(|_| editable && !conflict);
        // The open file, whole (owner kind, owner id, path): a fix's Undo acts only on it.
        let file: FileId = (note.owner.key(), note.path.clone());
        let heading_now = HeadingChoice::new(
            current.as_ref(),
            &self.draft,
            self.kept.heading(),
            &file,
            &self.same_heading,
        );
        let pressed_heading = (
            heading_fix && heading_now.fix.is_some(),
            heading_undo && self.heading_undo_applies(&file),
        );
        if pressed_heading.0 || pressed_heading.1 {
            self.draft_revision += 1;
        }
        save |= heading_now.apply(pressed_heading, &file, &mut self.draft, &mut self.kept);
        // The stat fix again from the current text: its check must still say this value.
        let current = vital_use.and_then(|(key, label, value)| {
            let entry = Entry::parse(&note.path, &self.draft)
                .ok()
                .filter(|_| editable)?;
            creature::vital_checks(&entry)
                .iter()
                .any(|(at, _, now, _)| *at == key && *now == value)
                .then_some((key, label, value, entry))
        });
        if let Some((key, label, value, entry)) = current {
            Self::fix_stat(
                (
                    &mut self.kept,
                    &mut self.draft,
                    &mut self.draft_revision,
                    &mut self.error,
                ),
                file.clone(),
                &entry,
                (key, label, value),
            );
        }
        if vital_undo
            && self.stat_undo_applies(&file)
            && let Some((_, before, _, _)) = self.kept.take_vital()
        {
            let _ = self.draft.replace(before);
            // The draft changed: a failed Save of the old words no longer owns it.
            self.draft_revision += 1;
        }
        // Live, as above: a press that changed the draft ends Discard's Undo.
        if !self.saved_is_draft(note) {
            let _ = self.kept.take_discarded();
        }
        if self.draft_revision != draft_before {
            ui.ctx().request_repaint();
        }
        if repair_pressed {
            if let Some((module, path, exists)) = recovery {
                self.write_manifest(module, &path, exists);
            } else {
                self.source = true;
                self.focus_repair = true;
            }
            return;
        }
        let reconnect = (|| -> Result<(), String> {
            if let Some(at) = undo_dependency
                && let Some(DependencyUndo {
                    root,
                    path,
                    before,
                    after,
                    copied_path,
                    copied_text,
                    ..
                }) = self.kept.copies().get(at)
            {
                let copied = note_path(root, copied_path)?;
                if fs::read_to_string(&copied)
                    .map_err(|e| FileFault::read(copied_path, &e).message())?
                    != *copied_text
                {
                    return Err(
                        "The copy changed on disk. Keep it, and change the link in Markdown."
                            .into(),
                    );
                }
                write_note(root, path, Some(after), before)?;
                if let Err(error) = fs::remove_file(copied) {
                    let _ = write_note(root, path, Some(before), after);
                    return Err(error.to_string());
                }
                self.content_changed = Some(root.clone());
                self.kept.remove_copy(at);
                self.stale = true;
            }
            if let Some(id) = copy_dependency
                && let Some(source) = self
                    .loaded
                    .as_ref()
                    .unwrap()
                    .notes
                    .iter()
                    .find(|n| n.entry.as_ref().is_some_and(|e| e.id == id))
                && let Some(root) = Self::copy_dependency(&mut self.kept, note, source, campaigns)?
            {
                self.content_changed = Some(root);
                self.stale = true;
            }
            Ok(())
        })();
        if let Err(error) = reconnect {
            self.error = Some(error.into());
        }
        if reload_from_disk {
            self.reload_note_from_disk(i);
            return;
        }
        // Try again reads the file now. With no draft, every file is read again and the note opens
        // with its text or its new fault. With a draft (after Undo discard), the file is compared
        // with it as any outside change is: a load waits while a draft is open.
        if (read_again || read_disk_again)
            && let Some(root) = note.owner.root().map(Path::to_path_buf)
        {
            self.refresh_destination(&root, ui.ctx(), campaigns);
            return;
        }
        if let Some(target) =
            pressed_link.or_else(|| crate::markdown_view::take_note_link(ui.ctx()))
            && let Some(index) = ui.ctx().data(|data| {
                data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                    "reader_link_index",
                ))
            })
        {
            let from = note.entry.as_ref().map(|entry| entry.id);
            let choices = match index.resolve_link(&target, from) {
                crate::world::Go::Item(at, heading) => {
                    crate::markdown_view::want_heading(ui.ctx(), heading);
                    vec![index.items[at].entry.id]
                }
                crate::world::Go::Many(positions) => positions
                    .iter()
                    .map(|&at| index.items[at].entry.id)
                    .collect(),
                _ => Vec::new(),
            };
            self.link_choices = choices;
        }
        if self.link_choices.len() == 1 {
            let id = self.link_choices.remove(0);
            open_link = self
                .loaded
                .as_ref()
                .unwrap()
                .notes
                .iter()
                .position(|n| n.entry.as_ref().is_some_and(|e| e.id == id));
            if open_link.is_none() {
                self.peek = Some(id);
            }
        } else if !self.link_choices.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for id in &self.link_choices {
                    if let Some((at, note)) = self
                        .loaded
                        .as_ref()
                        .unwrap()
                        .notes
                        .iter()
                        .enumerate()
                        .find(|(_, n)| n.entry.as_ref().is_some_and(|e| e.id == *id))
                        && design::link(ui, &format!("{} · {}", note.name(), note.owner.label()))
                            .clicked()
                    {
                        open_link = Some(at);
                    }
                }
            });
            if open_link.is_some() {
                self.link_choices.clear();
            }
        }
        if let Some(module) = use_every {
            self.choose_all(&module, campaigns);
        }

        if save {
            self.save_selected(file_gone);
        }
        let identity_result = if let Some((root, paths)) = new_identities {
            self.separate_entries(root, &paths)
        } else if undo_identities {
            self.kept.identities().unwrap().0.undo().map(|()| {
                let _ = self.kept.take_identities();
                self.stale = true;
                self.content_changed = campaigns.active_campaign().map(|campaign| campaign.path);
            })
        } else if let Some(at) = new_identity {
            self.open_entry(at);
            self.give_new_identity()
        } else if undo_identity {
            self.undo_new_identity()
        } else {
            Ok(())
        };
        if let Err(error) = identity_result {
            self.error = Some(error.into());
        }
        if let Some(index) = open_link {
            if dirty {
                // Keep the draft. The linked entry opens beside it, in Lookup.
                self.peek = self
                    .loaded
                    .as_ref()
                    .and_then(|l| l.notes.get(index))
                    .and_then(|n| n.entry.as_ref())
                    .map(|e| e.id);
            } else {
                self.open_entry(index);
                // Followed from the text, so the list may not show it yet.
                self.reveal_row = true;
            }
        }
        if let Some(step) = step {
            if dirty {
                self.peek = self.trail.peek(step).and_then(|target| {
                    let notes = &self.loaded.as_ref()?.notes;
                    notes[target.find(notes)?].entry.as_ref().map(|e| e.id)
                });
            } else {
                self.go(step);
            }
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
#[path = "library/workflow_tests.rs"]
mod workflow_tests;
#[cfg(feature = "test-support")]
pub use workflow_tests::{
    copy_registration_and_undo_keep_campaign_choices_and_later_edits,
    iter14_e_changed_files_end_every_undo_result,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_workspace::Catalogue;

    impl Library {}

    #[test]
    fn duplicate_checks_keep_id_key_matches_and_refresh_after_edits() {
        // Row 115 (FH10): a new rule or effect has no heading that repeats its kind line.
        for (kind, word) in [("rule", "Rule"), ("effect", "Effect")] {
            let text = note_text("Watch Orders", kind, None);
            assert!(!text.contains(&format!("## {word}\n")), "{text}");
            assert!(text.contains("\n# Watch Orders\n\nDescribe "), "{text}");
        }
        let base = Entry::parse("Rule.md", &note_text("Rule", "rule", None)).unwrap();
        let mut loaded = Loaded::default();
        // Same ID, same key, both, no key, and unreadable notes all occur in a
        // loaded Library. A key collision must mark both distinct IDs.
        for (id, key) in [
            (1, Some("test:a")),
            (2, Some("test:a")),
            (1, Some("test:b")),
            (3, Some("test:b")),
            (1, Some("test:a")),
            (4, None),
            (5, None),
        ] {
            let mut entry = base.clone();
            entry.id = Uuid::from_u128(id);
            entry.key = key.map(str::to_owned);
            loaded.notes.push(Note {
                path: format!("{}.md", loaded.notes.len()),
                text: crate::design::memo::Revised::default(),
                entry: Some(entry),
                error: None,
                fault: None,
                manifest_gone: false,
                owner: Owner::Builtin,
            });
        }
        loaded.notes.push(Note {
            path: "Broken.md".into(),
            text: crate::design::memo::Revised::default(),
            entry: None,
            error: Some("Invalid YAML".into()),
            fault: None,
            manifest_gone: false,
            owner: Owner::Builtin,
        });
        let check = |loaded: &Loaded| {
            let mut ids = std::collections::HashSet::new();
            for (at, note) in loaded.notes.iter().enumerate() {
                let expected: Vec<_> = loaded
                    .notes
                    .iter()
                    .enumerate()
                    .filter_map(|(other, candidate)| {
                        let target = note.entry.as_ref()?;
                        let candidate = candidate.entry.as_ref()?;
                        (at != other
                            && (target.id == candidate.id
                                || (target.key.is_some() && target.key == candidate.key)))
                            .then_some(other)
                    })
                    .collect();
                assert_eq!(loaded.duplicates(at), expected);
                if !expected.is_empty() {
                    ids.insert(note.entry.as_ref().unwrap().id);
                }
            }
            assert_eq!(loaded.duplicate_ids(), &ids);
        };
        check(&loaded);
        for (at, note) in loaded.notes.iter_mut().enumerate() {
            if let Some(entry) = &mut note.entry {
                entry.id = Uuid::from_u128(at as u128 + 10);
                entry.key = None;
            }
        }
        loaded.invalidate();
        check(&loaded);
        assert!(loaded.duplicate_ids().is_empty());
    }

    #[test]
    fn i20_c1_stat_flags_reach_list_subtitles_and_header_count() {
        let dir = tempfile::tempdir().unwrap();
        let module = create_module(&dir.path().join("book"), "Book", "book").unwrap();
        for (name, kind, table) in [
            (
                "Wolf",
                "creature",
                "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Hit Points | about twenty |\n",
            ),
            (
                "Spark",
                "spell",
                "## Spell\n\n| Field | Value |\n| --- | --- |\n| Level | many |\n",
            ),
            (
                "Bear",
                "creature",
                "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Armor Class | thick fur |\n| Hit Points | about forty |\n",
            ),
        ] {
            fs::write(module.path.join(format!("{name}.md")), format!(
                "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: {kind}\nsplat_name: {name}\n---\n\n{table}", Uuid::new_v4()
            )).unwrap();
        }
        let mut loaded = load(Some(&module)).unwrap();
        loaded.refresh_diagnostics();
        for name in ["Wolf", "Spark"] {
            let at = loaded
                .notes
                .iter()
                .position(|note| note.name() == name)
                .unwrap();
            assert!(loaded.subtitles()[at].contains("1 value not read"));
        }
        // FD5: the Library row uses World's words, in the plural too.
        let bear = loaded
            .notes
            .iter()
            .position(|note| note.name() == "Bear")
            .unwrap();
        assert!(
            loaded.subtitles()[bear].contains("2 values not read"),
            "{}",
            loaded.subtitles()[bear]
        );
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| library.show(ui, &mut Campaigns::default()));
            },
        );
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("3 files need fixing"))));
        let diagnostics = &library.loaded.as_ref().unwrap().diagnostic_texts();
        assert!(diagnostics.iter().any(|s| s.starts_with("Wolf.md: Line ")
            && s.contains(": Hit Points “about twenty” was not read")));
        assert!(
            diagnostics
                .iter()
                .any(|s| s.starts_with("Spark.md: Line ")
                    && s.contains(": Level “many” was not read"))
        );
    }

    /// iter-12-E-FD2: an entry with a broken link counts in the header once, beside unreadable and
    /// flagged files. The scan is cached against the link index until a note changes.
    #[test]
    fn broken_links_count_in_the_header_once() {
        let dir = tempfile::tempdir().unwrap();
        let module = create_module(&dir.path().join("book"), "Book", "book").unwrap();
        let note = |name: &str, kind: &str, body: &str| {
            fs::write(
                module.path.join(format!("{name}.md")),
                format!(
                    "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: {kind}\nsplat_name: {name}\n---\n\n{body}",
                    Uuid::new_v4()
                ),
            )
            .unwrap();
        };
        note("Linker", "rule", "See [[Gone Page]].\n");
        note("Fine", "rule", "See [[Linker]].\n");
        // A flag and a broken link in one file: one file to fix.
        note(
            "Both",
            "creature",
            "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Hit Points | about twenty |\n\n[[Gone Too]]\n",
        );
        fs::write(
            module.path.join("Unreadable.md"),
            "---\nsplat_kind: [\n---\n",
        )
        .unwrap();
        let mut loaded = load(Some(&module)).unwrap();
        loaded.refresh_diagnostics();
        let index = std::sync::Arc::new(
            crate::lookup::ContentIndex::load(std::slice::from_ref(&module), None).unwrap(),
        );
        let names = |loaded: &Loaded, notes: Vec<(usize, usize)>| -> Vec<String> {
            notes
                .into_iter()
                .map(|(at, _)| loaded.notes[at].name().to_owned())
                .collect()
        };
        assert_eq!(
            names(&loaded, loaded.broken_link_notes(&index)),
            ["Both", "Linker"]
        );
        // Cached: a changed body is not scanned again until the note is invalidated.
        let fine = loaded
            .notes
            .iter()
            .position(|n| n.name() == "Fine")
            .unwrap();
        loaded.notes[fine].entry.as_mut().unwrap().body =
            "See [[Nowhere]] and [[Nor Here]].\n".into();
        assert_eq!(loaded.broken_link_notes(&index).len(), 2);
        loaded.invalidate();
        assert_eq!(
            names(&loaded, loaded.broken_link_notes(&index)),
            ["Both", "Fine", "Linker"]
        );
        // The menu reads each count from the cache: two links in Fine, one in each other file.
        assert_eq!(
            loaded
                .broken_link_notes(&index)
                .iter()
                .map(|&(_, count)| count)
                .collect::<Vec<_>>(),
            [1, 2, 1]
        );
        // A new index finds them again, with the same notes.
        loaded.notes[fine].entry.as_mut().unwrap().body = "See [[Linker]].\n".into();
        let other = std::sync::Arc::new(
            crate::lookup::ContentIndex::load(std::slice::from_ref(&module), None).unwrap(),
        );
        assert_eq!(loaded.broken_link_notes(&other).len(), 2);

        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        ctx.data_mut(|data| data.insert_temp(egui::Id::new("reader_link_index"), other));
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| library.show(ui, &mut Campaigns::default()));
            },
        );
        let painted = |words: &str| {
            output.shapes.iter().any(|shape| {
                matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains(words))
            })
        };
        // Linker, Both (flag and link) and the unreadable file: three, not four.
        assert!(painted("3 files need fixing"));
    }

    /// iter-13-E-SD1: the header counts files by owner and path. Two packages each with
    /// `Rules/Watch.md` and a broken link are two files with two rows that name their owners. A
    /// file with a problem and a broken link still counts once, and a check's problem stays with
    /// the package whose file has it.
    #[test]
    fn iter13_e_files_to_fix_count_by_owner_and_path() {
        let dir = tempfile::tempdir().unwrap();
        let ashen = create_module(&dir.path().join("ashen"), "Ashen Marches", "ashen").unwrap();
        let ember = create_module(&dir.path().join("ember"), "Ember Coast", "ember").unwrap();
        let write = |module: &Module, path: &str, kind: &str, extra: &str, body: &str| {
            let full = module.path.join(path);
            fs::create_dir_all(full.parent().unwrap()).unwrap();
            fs::write(
                full,
                format!(
                    "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: {kind}\nsplat_name: Watch\n{extra}---\n\n{body}",
                    Uuid::new_v4()
                ),
            )
            .unwrap();
        };
        write(&ashen, "Rules/Watch.md", "rule", "", "See [[Gone Page]].\n");
        // The same path in another package: an item that needs a key no package has.
        let item = splat::content::formats::fixture("Rules/Watch.md", &format!("splat_id: {}\nsplat_kind: item\nsplat_key: ember:items/watch\n\
                 splat_name: Watch\nsplat_item: {{category: gear}}\nsplat_requires:\n  - key: nowhere:items/gone\n", Uuid::new_v4()), "\nSee [[Gone Page]].\n")
        .expect("schema-2 fixture");
        fs::create_dir_all(ember.path.join("Rules")).unwrap();
        fs::write(ember.path.join("Rules/Watch.md"), item).unwrap();
        // A flag and a broken link in one file: one file to fix.
        write(
            &ashen,
            "Both.md",
            "creature",
            "",
            "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Hit Points | about twenty |\n\n[[Gone Too]]\n",
        );
        let modules = [ashen.clone(), ember.clone()];
        let loaded = load_all(&modules, None).unwrap();
        let index = std::sync::Arc::new(crate::lookup::ContentIndex::load(&modules, None).unwrap());
        let broken = loaded.broken_link_notes(&index);
        let watch = |module: &Module| {
            loaded
                .notes
                .iter()
                .find(|n| n.owner.id() == Some(module.id) && n.path == "Rules/Watch.md")
                .unwrap()
        };
        let both = loaded.notes.iter().find(|n| n.path == "Both.md").unwrap();
        // Both Watch files and Both.md have a broken link.
        let mut broken_files: Vec<_> = broken
            .iter()
            .map(|&(at, _)| loaded.file_label(&loaded.notes[at]))
            .collect();
        broken_files.sort();
        assert_eq!(
            broken_files,
            [
                "Ashen Marches · Rules/Watch.md",
                "Both.md",
                "Ember Coast · Rules/Watch.md"
            ]
        );
        // The check's problem goes to Ember Coast's file only, and its row names the owner.
        let unresolved: Vec<_> = loaded
            .diagnostics
            .iter()
            .filter(|d| d.text.contains("nowhere:items/gone"))
            .collect();
        assert_eq!(unresolved.len(), 1, "{:?}", loaded.diagnostic_texts());
        assert_eq!(
            unresolved[0].file,
            (OwnerKey::Package(ember.id), "Rules/Watch.md".to_owned())
        );
        assert!(
            unresolved[0]
                .text
                .starts_with("Ember Coast · Rules/Watch.md: "),
            "{}",
            unresolved[0].text
        );
        assert!(loaded.has_problem(watch(&ember)));
        assert!(!loaded.has_problem(watch(&ashen)));
        // A path only one note has keeps its plain row.
        assert!(loaded.has_problem(both));
        assert!(
            loaded
                .diagnostic_texts()
                .iter()
                .any(|text| text.starts_with("Both.md: Line") && text.contains(": Hit Points"))
        );
        // Two Watch files and Both.md: three, not two (by path) and not five (by problem).
        assert_eq!(loaded.files_to_fix(&broken, None), 3);
        // Round 24 FH5 (must stay true): a file with a flag beside its broken link, and a file
        // whose requirement needs a catalogue match, keep `Needs fixing` for the fault the link
        // count does not name.
        assert!(!loaded.link_faults_only.contains(&Loaded::file_key(both)));
        assert!(
            !loaded
                .link_faults_only
                .contains(&Loaded::file_key(watch(&ember)))
        );
        assert!(
            !loaded.unreadable_counted,
            "warnings only: the count stays amber"
        );

        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        ctx.data_mut(|data| data.insert_temp(egui::Id::new("reader_link_index"), index));
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| library.show(ui, &mut Campaigns::default()));
            },
        );
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("3 files need fixing"))));

        // iter-13-E2 review 1: a second package with the same name and path. Its rows name the
        // folder, so the two Ashen Marches rows differ; Ember Coast keeps its plain owner row.
        let copy =
            create_module(&dir.path().join("ashen-copy"), "Ashen Marches", "ashcopy").unwrap();
        write(&copy, "Rules/Watch.md", "rule", "", "See [[Gone Page]].\n");
        let modules = [ashen.clone(), ember.clone(), copy.clone()];
        let loaded = load_all(&modules, None).unwrap();
        let index = std::sync::Arc::new(crate::lookup::ContentIndex::load(&modules, None).unwrap());
        let broken = loaded.broken_link_notes(&index);
        let mut broken_files: Vec<_> = broken
            .iter()
            .map(|&(at, _)| loaded.file_label(&loaded.notes[at]))
            .collect();
        broken_files.sort();
        assert_eq!(
            broken_files,
            [
                "Ashen Marches · ashen-copy/Rules/Watch.md",
                "Ashen Marches · ashen/Rules/Watch.md",
                "Both.md",
                "Ember Coast · Rules/Watch.md"
            ]
        );
        assert_eq!(loaded.files_to_fix(&broken, None), 4);

        // Two packages with the same name that both fail to load are two problems, and their
        // messages name the folders.
        let gone = [
            create_module(&dir.path().join("lost-one"), "Lost Book", "lostone").unwrap(),
            create_module(&dir.path().join("lost-two"), "Lost Book", "losttwo").unwrap(),
        ];
        for module in &gone {
            fs::remove_dir_all(&module.path).unwrap();
        }
        let loaded = load_all(&gone, None).unwrap();
        assert_eq!(
            loaded.files_to_fix(&[], None),
            2,
            "{:?}",
            loaded.diagnostic_texts()
        );
        let texts = loaded.diagnostic_texts();
        assert!(
            texts
                .iter()
                .any(|text| text.starts_with("Lost Book · lost-one/: ")),
            "{texts:?}"
        );
        assert!(
            texts
                .iter()
                .any(|text| text.starts_with("Lost Book · lost-two/: ")),
            "{texts:?}"
        );
    }

    /// iter-13-E-FD1: the About row warns for any problem the header counts in the manifest, such
    /// as a first heading that differs from the name. A clean manifest has no warning.
    #[test]
    fn iter13_e_about_row_warns_for_a_counted_manifest_problem() {
        let dir = tempfile::tempdir().unwrap();
        let module = create_module(&dir.path().join("ember"), "Emberfall", "ember").unwrap();
        let manifest_path = module.path.join("Package.md");
        let clean = fs::read_to_string(&manifest_path).unwrap();
        let about = |loaded: &Loaded| {
            let manifest = loaded.notes.iter().find(|n| n.is_manifest()).unwrap();
            manifest.about_label(loaded.has_problem(manifest))
        };
        let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
        assert_eq!(loaded.diagnostic_texts(), Vec::<&str>::new());
        assert_eq!(about(&loaded), "About Emberfall");
        let heading = clean
            .lines()
            .find(|line| line.starts_with("# "))
            .expect("the manifest has a first heading");
        fs::write(&manifest_path, clean.replacen(heading, "# Ember Notes", 1)).unwrap();
        let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
        let manifest = loaded.notes.iter().find(|n| n.is_manifest()).unwrap();
        // The header counts it, and the manifest still reads: only the counted problem warns.
        assert!(manifest.error.is_none());
        assert_eq!(loaded.files_to_fix(&[], None), 1);
        assert_eq!(about(&loaded), format!("{} About Emberfall", icons::WARN));
    }

    fn navigation_library() -> Library {
        let notes = ["First", "Second", "Destination"]
            .into_iter()
            .map(|name| {
                let text = note_text(name, "rule", None);
                Note {
                    path: if name == "Destination" {
                        "Destination.md".into()
                    } else {
                        "Rule.md".into()
                    },
                    entry: Some(Entry::parse("Rule.md", &text).unwrap()),
                    text: crate::design::memo::Revised::new(text),
                    error: None,
                    fault: None,
                    manifest_gone: false,
                    owner: Owner::Package(Module {
                        id: Uuid::new_v4(),
                        name: name.into(),
                        path: name.into(),
                    }),
                }
            })
            .collect();
        Library {
            loaded: Some(Loaded::fixture(notes)),
            ..Default::default()
        }
    }

    #[test]
    fn iter2_d_copied_folder_blocks_add_until_the_path_changes() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::{NodeT, Queryable};
        let folder = tempfile::tempdir().unwrap();
        let other = tempfile::tempdir().unwrap();
        let library = Library {
            path: folder.path().display().to_string(),
            copied_package: Some(Module {
                id: Uuid::new_v4(),
                name: "Copied package".into(),
                path: folder.path().canonicalize().unwrap(),
            }),
            ..Default::default()
        };
        let mut campaigns = Campaigns::default();
        let mut harness = egui_kittest::Harness::builder().themed_ui_state(
            move |ui, library| library.package_form(ui, &mut campaigns),
            library,
        );
        harness.run_steps(2);
        assert!(
            harness
                .get_by_label("Add folder")
                .accesskit_node()
                .is_disabled()
        );
        harness
            .get_by_role_and_label(egui::accesskit::Role::TextInput, "Package folder")
            .focus();
        harness.key_press_modifiers(egui::Modifiers::CTRL, egui::Key::A);
        harness
            .get_by_role_and_label(egui::accesskit::Role::TextInput, "Package folder")
            .type_text(&other.path().display().to_string());
        harness.run_steps(2);
        assert!(
            !harness
                .get_by_label("Add folder")
                .accesskit_node()
                .is_disabled()
        );
        assert!(harness.state().copied_package.is_none());
        // Round 28 K4a (the review of PR 341): while the Library loads, Browse is disabled and
        // its description says why; once it has loaded, Browse is open again.
        let browse = |harness: &egui_kittest::Harness<'_, Library>| {
            // The folder form's Browse (the new package form below has its own).
            let node = harness
                .query_all_by_label("Browse…")
                .next()
                .expect("Browse")
                .accesskit_node();
            (
                node.is_disabled(),
                node.description().map(|words| words.to_owned()),
            )
        };
        assert_eq!(browse(&harness), (false, None));
        let (_loading, rx) = mpsc::channel();
        harness.state_mut().pending = Some(rx);
        harness.run_steps(2);
        assert_eq!(
            browse(&harness),
            (true, Some("Browse waits for the reason below.".to_owned()))
        );
        assert!(
            harness
                .query_all_by_label_contains("Wait for the Library to load.")
                .next()
                .is_some(),
            "the reason it points to"
        );
        harness.state_mut().pending = None;
        harness.run_steps(2);
        assert_eq!(browse(&harness), (false, None));
    }

    #[test]
    fn iter2_d_about_selects_its_own_navigation_row() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::{NodeT, Queryable};
        let mut library = navigation_library();
        let owner = library.loaded.as_ref().unwrap().notes[0].owner.clone();
        let package = owner.id().unwrap();
        let text = note_text("First", "package", Some("first"));
        library.loaded.as_mut().unwrap().notes[0] = Note {
            path: "Package.md".into(),
            entry: Some(Entry::parse("Package.md", &text).unwrap()),
            text: crate::design::memo::Revised::new(text),
            error: None,
            fault: None,
            manifest_gone: false,
            owner,
        };
        library.loaded.as_mut().unwrap().notes[1].owner =
            library.loaded.as_ref().unwrap().notes[0].owner.clone();
        library.scope = Scope::Package(package);
        let mut campaigns = Campaigns::default();
        let mut harness = egui_kittest::Harness::builder()
            .themed_ui_state(move |ui, library| library.show(ui, &mut campaigns), library);
        harness.set_size(egui::vec2(1920.0, 1080.0));
        harness.run_steps(2);
        harness.get_by_label("About First").click();
        harness.run_steps(2);
        assert_eq!(
            harness
                .get_by_label("About First")
                .accesskit_node()
                .toggled(),
            Some(egui::accesskit::Toggled::True)
        );
        assert_eq!(
            harness
                .get_by_label("All entries   1")
                .accesskit_node()
                .toggled(),
            Some(egui::accesskit::Toggled::False)
        );
        harness.get_by_label("All entries   1").click();
        harness.run_steps(2);
        assert_eq!(
            harness
                .get_by_label("About First")
                .accesskit_node()
                .toggled(),
            Some(egui::accesskit::Toggled::False)
        );
        assert_eq!(
            harness
                .get_by_label("All entries   1")
                .accesskit_node()
                .toggled(),
            Some(egui::accesskit::Toggled::True)
        );
    }

    #[test]
    fn link_updates_stop_when_a_name_or_filename_is_ambiguous() {
        let mut library = navigation_library();
        let loaded = library.loaded.as_mut().unwrap();
        assert!(loaded.can_rename_links(0, "First", "New name"));
        assert!(!loaded.can_rename_links(0, "First", "Second"));
        loaded.notes[1].path = "Elsewhere/FIRST.md".into();
        assert!(!loaded.can_rename_links(0, "First", "New name"));
        loaded.notes[1].path = "Second.md".into();
        loaded.notes[1].entry.as_mut().unwrap().name = "First".into();
        assert!(!loaded.can_rename_links(0, "First", "New name"));

        // Round 24 FC4: the offer's count reads the notes once; an unchanged frame reads none,
        // and an edited note (`invalidate`) is counted again.
        let scans = || RENAME_LINK_SCANS.with(std::cell::Cell::get);
        let start = scans();
        let before = loaded.rename_link_count("Watch", "Ward");
        assert_eq!(scans(), start + 1);
        assert_eq!(loaded.rename_link_count("Watch", "Ward"), before);
        assert_eq!(scans(), start + 1, "an unchanged frame reads no note");
        let linker = loaded
            .notes
            .iter()
            .position(|note| note.owner.root().is_some())
            .expect("fixture: a note in a folder");
        loaded.notes[linker].text.revise(|text| {
            text.push_str("\nSee [[Watch]].\n");
            crate::design::memo::Changed::Yes
        });
        loaded.invalidate();
        assert_eq!(loaded.rename_link_count("Watch", "Ward"), before + 1);
        assert_eq!(scans(), start + 2, "an edited note is counted again");
    }

    #[test]
    fn saving_a_note_moved_to_another_owner_does_not_create_a_second_file() {
        let temp = tempfile::tempdir().unwrap();
        let first = create_module(&temp.path().join("first"), "First", "first").unwrap();
        let second = create_module(&temp.path().join("second"), "Second", "second").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(first.path.join("Wolf.md"), &text).unwrap();
        let loaded = load_all(&[first.clone(), second.clone()], None).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: (format!("{text}\nMy draft.\n")).into(),
            ..Default::default()
        };
        fs::rename(first.path.join("Wolf.md"), second.path.join("Moved.md")).unwrap();
        library.save_selected(false);
        assert!(library.error.is_none(), "{:?}", library.error);
        assert!(!first.path.join("Wolf.md").exists());
        assert_eq!(
            fs::read_to_string(second.path.join("Moved.md")).unwrap(),
            library.draft.as_str()
        );
    }

    #[test]
    fn saving_a_moved_and_edited_note_opens_comparison_before_writing() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: (format!("{text}\nMy draft.\n")).into(),
            ..Default::default()
        };
        fs::rename(module.path.join("Wolf.md"), module.path.join("Moved.md")).unwrap();
        let outside = format!("{text}\nOutside edit.\n");
        fs::write(module.path.join("Moved.md"), &outside).unwrap();
        library.save_selected(false);
        assert!(library.disk_change.is_some());
        assert_eq!(
            fs::read_to_string(module.path.join("Moved.md")).unwrap(),
            outside
        );
        assert!(library.draft.contains("My draft."));
    }

    #[test]
    fn repairing_a_created_package_replaces_its_actual_manifest() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let (entries, _) = content::read_directory(&module.path).unwrap();
        let path = module.path.join(&entries[0].path);
        let valid = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            valid.replace("splat_namespace: test", "splat_namespace: ["),
        )
        .unwrap();
        let repair = crate::content_workspace::ManifestWrite::write(&module).unwrap();
        assert!(repair.replaced());
        assert_eq!(content::read_directory(&module.path).unwrap().0.len(), 1);
        repair.undo().unwrap();
        fs::write(&path, valid).unwrap();
        assert_eq!(local_module(&module.path).unwrap().id, module.id);
    }

    /// M2's named debt (ruling R35, packet E4): the legacy reason paths, whose reason is a hover
    /// unless the caller paints it, and the staged result adapter, counted exactly in the two
    /// files of this lane outside their tests. A new raw `.blocked(…)` (also `Copy to…` taken
    /// out of the toolbar part), a new `design::locked(…)`, a new old Undo message or a new
    /// adapter use (also through an aliased import, which spells its name) fails here; a removed
    /// one lowers its line in the same change. The app-wide Clippy bans come in the late bans PR.
    #[test]
    fn m2_legacy_reason_paths_are_counted() {
        const COUNTS: &[(&str, &[(&str, usize)])] = &[
            (
                "src/library.rs",
                &[
                    ("kept_result", 0),
                    // The typed locks of the footers (M-L0.4b: `page_messages`, the duplicate
                    // notice, the dependencies, the recovery choice) and the old waiting
                    // controls left in the page's body. A typed lock is not debt, but this count
                    // reads text.
                    (".blocked(", 12),
                    ("blocked_if(", 0),
                    ("design::locked(", 0),
                    ("undo_message(", 0),
                ],
            ),
            (
                "src/library/catalogue.rs",
                &[
                    ("kept_result", 0),
                    (".blocked(", 3),
                    ("blocked_if(", 0),
                    ("design::locked(", 0),
                    ("undo_message(", 0),
                ],
            ),
            (
                "src/campaigns.rs",
                &[
                    ("kept_result", 0),
                    (".blocked(", 4),
                    ("blocked_if(", 0),
                    ("design::locked(", 1),
                    ("undo_message(", 0),
                ],
            ),
        ];
        let workspace = crate::workspace();
        for (path, counts) in COUNTS {
            let text = fs::read_to_string(workspace.join(path)).unwrap();
            let body = &text[..text.find("#[cfg(test)]\nmod tests").unwrap_or(text.len())];
            for (pattern, count) in *counts {
                assert_eq!(
                    body.matches(pattern).count(),
                    *count,
                    "{path}: `{pattern}` is counted debt (M2). Draw a waiting control through \
                     `design::ReaderToolbar` or a typed message, or change this line on purpose."
                );
            }
        }
    }

    /// Round 30 SH1: a folder field reads its folders once per open of its suggestions menu;
    /// a redraw reads nothing; a close (with no closed draw between), a form that was not drawn,
    /// another root or kind, and another menu each start a fresh read.
    #[test]
    fn folder_suggestions_read_once_per_open_menu() {
        let temp = tempfile::tempdir().unwrap();
        for folder in ["Creatures/Undead", "Spells", ".hidden"] {
            fs::create_dir_all(temp.path().join(folder)).unwrap();
        }
        let other = tempfile::tempdir().unwrap();
        let ctx = egui::Context::default();
        let reads = || FOLDER_READS.with(std::cell::Cell::get);
        // One pass of the app: the menu `id`'s state this pass, at `root` with `kind`.
        let pass = |id: &str, state: (bool, bool), root: &Path, kind: Option<&str>| {
            let mut folders = std::sync::Arc::default();
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                folders = folder_suggestions(ctx, egui::Id::new(id), state, root, kind);
            });
            folders
        };
        let (closed, opening, open) = ((false, false), (false, true), (true, false));
        let spell = Some("spell");
        let before = reads();
        for _ in 0..3 {
            assert!(pass("a", closed, temp.path(), spell).is_empty());
        }
        assert_eq!(reads(), before, "a closed menu reads no folder");
        let folders = pass("a", opening, temp.path(), spell);
        assert_eq!(*folders, ["Creatures", "Creatures/Undead", "Spells"]);
        let one_read = reads() - before;
        assert!(one_read > 0);
        for _ in 0..3 {
            assert_eq!(*pass("a", open, temp.path(), spell), *folders);
        }
        assert_eq!(
            reads() - before,
            one_read,
            "an open menu's redraw reads nothing"
        );
        // Choosing a folder closes the menu; the next pass opens it again at once, with no
        // closed draw between: a fresh read shows a folder made meanwhile.
        fs::create_dir(temp.path().join("Items")).unwrap();
        assert!(pass("a", opening, temp.path(), spell).contains(&"Items".to_owned()));
        assert_eq!(reads() - before, 2 * one_read + 1);
        // Its form was not drawn for a while (no pass of this menu): the session ended.
        pass("b", opening, other.path(), None);
        pass("b", open, other.path(), None);
        fs::create_dir(temp.path().join("Rules")).unwrap();
        assert!(pass("a", open, temp.path(), spell).contains(&"Rules".to_owned()));
        // Another kind, and another root, are other lists.
        let at = reads();
        assert!(!pass("a", open, temp.path(), None).contains(&"Spells/".to_owned()));
        assert!(reads() > at, "another kind reads again");
        let at = reads();
        assert!(pass("a", open, other.path(), None).is_empty());
        assert!(reads() > at, "another root reads again");
    }

    /// Round 30 B11 F4, Astra F8, and the B2b review's F1 and F2: Save on the draft of a
    /// package manifest that is known to be gone. An empty file that came back at its path is
    /// a file, never overwritten: the comparison opens. The manifest back under another name
    /// (same identity) is refused with a painted reason, with no second manifest. A free path is
    /// created from the draft.
    #[test]
    fn a_gone_manifest_draft_is_created_only_on_a_free_path() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let manifest = module.path.join("Package.md");
        let valid = fs::read_to_string(&manifest).unwrap();
        fs::remove_file(&manifest).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let at = loaded
            .notes
            .iter()
            .position(|note| note.manifest_gone)
            .expect("the gone manifest is kept as a note");
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(at);
        let _ = library.draft.replace(valid.clone());
        // F2: an empty file is there at Save.
        fs::write(&manifest, "").unwrap();
        library.save_selected(false);
        assert_eq!(
            fs::read_to_string(&manifest).unwrap(),
            "",
            "never written over"
        );
        assert!(library.disk_change.is_some(), "the comparison opens");
        assert_eq!(library.draft.as_str(), valid, "the draft stays");
        // F1: the manifest came back as Module.md, with this package's identity.
        fs::remove_file(&manifest).unwrap();
        library.disk_change = None;
        fs::write(module.path.join("Module.md"), &valid).unwrap();
        library.save_selected(false);
        assert!(!manifest.exists(), "no second manifest");
        assert!(
            library
                .error
                .as_ref()
                .map(PageError::words)
                .is_some_and(|error| error.contains("Module.md")),
            "the refusal names where the manifest is: {:?}",
            library.error
        );
        // A free path, with no other manifest: Save creates it from the draft.
        fs::remove_file(module.path.join("Module.md")).unwrap();
        library.error = None;
        library.save_selected(false);
        assert_eq!(fs::read_to_string(&manifest).unwrap(), valid);
        assert!(library.just_saved);
    }

    #[test]
    fn copying_without_a_destination_is_blocked() {
        let campaigns = Campaigns::default();
        let places = Places::of(&campaigns);
        assert_eq!(
            places.copy().as_ref().map(|reason| reason.as_str()),
            Some("Add a campaign or a package to copy entries into.")
        );
        // One policy for both actions: with nowhere to put an entry, New entry waits too.
        assert_eq!(
            places.create().as_ref().map(|reason| reason.as_str()),
            Some("Add a campaign or a package to create entries.")
        );
        // A package is a place for both; a campaign that is not open is one only for a copy.
        let mut package = places;
        package.packages = true;
        assert_eq!((package.copy(), package.create()), (None, None));
        let mut closed = places;
        closed.campaigns = true;
        assert_eq!(closed.copy(), None);
        // The checks find the typed Undo lock by the same words.
        assert_eq!(
            crate::content_workspace::UNDO_ENDED_REASON.as_str(),
            crate::content_workspace::UNDO_ENDED
        );

        assert_eq!(
            closed.create().as_ref().map(|reason| reason.as_str()),
            Some("Open a campaign or add a package to create entries.")
        );
    }

    #[test]
    fn duplicate_package_id_save_keeps_both_files() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let copy = format!("{text}\nVariant on disk.\n");
        fs::write(module.path.join("Wolf copy.md"), &copy).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: (format!("{text}\nEdited original.\n")).into(),
            ..Default::default()
        };
        library.save_selected(false);
        assert!(library.error.is_none());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            library.draft.as_str()
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            copy
        );
        let id = crate::content_workspace::note_id(&text).unwrap();
        let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
        assert!(!catalogue.entries.contains_key(&id));
        let closure = catalogue.closure(
            &[crate::content_workspace::PackageSelection {
                name: Some(module.name.clone()),
                package: crate::content_workspace::EntryRef::id(module.id),
                selection: crate::content_workspace::Selection::All,
            }],
            Uuid::new_v4(),
        );
        assert!(!closure.direct.contains(&id));
        assert!(closure.errors.iter().any(|error| {
            error.contains("These two files claim to be the same entry: Wolf copy.md and Wolf.md.")
        }));
        assert!(
            catalogue.unreadable[&id]
                .word
                .contains("These two files claim to be the same entry: Wolf copy.md and Wolf.md.")
        );
        let copy_index = library
            .loaded
            .as_ref()
            .unwrap()
            .notes
            .iter()
            .position(|n| n.path == "Wolf copy.md")
            .unwrap();
        library.selected = Some(copy_index);
        let _ = library.draft.replace(format!("{copy}\nEdited copy.\n"));
        library.save_selected(false);
        let original = fs::read_to_string(module.path.join("Wolf.md")).unwrap();
        let saved_copy = library.draft.as_str().to_owned();
        library.give_new_identity().unwrap();
        assert_ne!(crate::content_workspace::note_id(&library.draft), Some(id));
        let repaired = Entry::parse("Wolf copy.md", &library.draft).unwrap();
        assert_eq!(repaired.name, "Wolf copy");
        assert!(repaired.body.contains("# Wolf copy\n"));
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            original
        );
        assert!(
            Catalogue::load(std::slice::from_ref(&module), None)
                .unwrap()
                .entries
                .contains_key(&id)
        );
        library.undo_new_identity().unwrap();
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            saved_copy
        );
        assert_eq!(
            library
                .loaded
                .as_ref()
                .unwrap()
                .duplicates(copy_index)
                .len(),
            1
        );
        library.give_new_identity().unwrap();
        fs::write(module.path.join("Wolf copy.md"), "A later disk edit.").unwrap();
        assert!(library.undo_new_identity().is_err());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            "A later disk edit."
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            original
        );
    }

    #[test]
    fn manifest_only_hand_repair_keeps_original_and_both_library_files() {
        let temp = tempfile::tempdir().unwrap();
        let original =
            create_module(&temp.path().join("original"), "Original", "original").unwrap();
        let copy = create_module(&temp.path().join("copy"), "Copy", "copy").unwrap();
        let manifest = fs::read_to_string(original.path.join("Package.md")).unwrap();
        let hand_edit = crate::content_workspace::patch_fields(
            &manifest,
            &[("splat_id", Some(copy.id.to_string().into()))],
        )
        .unwrap();
        fs::write(copy.path.join("Package.md"), hand_edit).unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(original.path.join("Wolf.md"), &text).unwrap();
        fs::write(copy.path.join("Wolf.md"), &text).unwrap();
        let id = crate::content_workspace::note_id(&text).unwrap();
        let modules = [original.clone(), copy];
        let catalogue = Catalogue::load(&modules, None).unwrap();
        assert_eq!(catalogue.entries[&id].owner, original.id);
        assert!(
            catalogue
                .errors
                .iter()
                .any(|e| e.contains("These two files claim to be the same entry:"))
        );
        let loaded = load_all(&modules, None).unwrap();
        let wolves: Vec<_> = loaded
            .notes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.name() == "Wolf" && n.owner.root().is_some())
            .collect();
        assert_eq!(wolves.len(), 2);
        // Round 24 AC1: both files of the duplicate count in the header, as both rows are
        // marked, though the check names one side.
        let wolf_files: Vec<FileId> = wolves.iter().map(|(_, n)| Loaded::file_key(n)).collect();
        let named = loaded
            .diagnostics
            .iter()
            .filter(|d| wolf_files.contains(&d.file))
            .map(|d| &d.file)
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert!(named < 2, "fixture: the check names one Wolf file only");
        let counted: std::collections::HashSet<&FileId> = loaded
            .diagnostics
            .iter()
            .map(|d| &d.file)
            .chain(&wolf_files)
            .collect();
        assert_eq!(
            loaded.files_to_fix(&[], None),
            counted.len(),
            "iter-24-L-AC1"
        );
        for (at, _) in wolves {
            assert_eq!(loaded.duplicates(at).len(), 1);
            assert!(loaded.build_subtitles()[at].contains("Same entry in two files"));
        }
    }

    #[test]
    fn duplicate_repair_keeps_key_named_original_after_atomic_save() {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("Wolf.md");
        let copy = temp.path().join("Wolf copy.md");
        let text = note_text("Wolf", "creature", None).replace(
            "splat_kind: creature",
            "splat_key: test:creatures/wolf\nsplat_kind: creature",
        );
        fs::write(&original, &text).unwrap();
        fs::write(&copy, &text).unwrap();
        write_note(
            temp.path(),
            "Wolf.md",
            Some(&text),
            &format!("{text}\nEdit original.\n"),
        )
        .unwrap();
        assert_eq!(
            crate::content_workspace::duplicate_repair_path([original, copy.clone()]),
            Some(copy)
        );
    }

    #[test]
    fn deleted_duplicate_is_not_treated_as_a_move_to_the_other_file() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        fs::write(module.path.join("Wolf copy.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: (format!("{text}\nMy draft.\n")).into(),
            ..Default::default()
        };
        fs::remove_file(module.path.join("Wolf.md")).unwrap();
        library.refresh_destination(
            &module.path,
            &egui::Context::default(),
            &Campaigns::default(),
        );
        assert!(library.disk_change.as_ref().unwrap().moved_to.is_none());
        library.save_selected(false);
        assert!(!module.path.join("Wolf.md").exists());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            text
        );
        library.save_selected(true);
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            library.draft.as_str()
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            text
        );
    }

    #[test]
    fn moved_draft_save_does_not_duplicate_its_id() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Moved rule", "rule", None);
        fs::write(module.path.join("Rule.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Rule.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: (format!("{text}\nMy draft.\n")).into(),
            ..Default::default()
        };
        fs::create_dir(module.path.join("Moved")).unwrap();
        fs::rename(
            module.path.join("Rule.md"),
            module.path.join("Moved/Rule.md"),
        )
        .unwrap();
        library.refresh_destination(
            &module.path,
            &egui::Context::default(),
            &Campaigns::default(),
        );
        library.save_selected(true);
        assert!(
            !module.path.join("Rule.md").exists(),
            "must not recreate the old path"
        );
        assert!(
            fs::read_to_string(module.path.join("Moved/Rule.md"))
                .unwrap()
                .contains("My draft.")
        );
    }

    #[test]
    fn repaired_package_note_reloads_campaign_uses() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test package", "test").unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path: temp.path().join("campaign"),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        fs::create_dir_all(&campaign.path).unwrap();
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let text = note_text("Test rule", "rule", None);
        let id = Entry::parse("Rule.md", &text).unwrap().id;
        fs::write(module.path.join("Rule.md"), &text).unwrap();
        crate::content_workspace::use_in_campaign(&campaign, module.id, &module.name, Some(id))
            .unwrap();
        assert!(
            load_all(std::slice::from_ref(&module), Some(&campaign))
                .unwrap()
                .uses
                .unwrap()
                .contains(&id)
        );
        fs::write(
            module.path.join("Rule.md"),
            text.replace("splat_kind: rule", "splat_kind: ["),
        )
        .unwrap();
        let loaded = load_all(std::slice::from_ref(&module), Some(&campaign)).unwrap();
        assert!(!loaded.uses.as_ref().unwrap().contains(&id));
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Rule.md" && n.owner.id() == Some(module.id))
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: (text.clone()).into(),
            ..Default::default()
        };
        library.save_selected(false);
        assert!(library.error.is_none(), "{:?}", library.error);
        assert!(library.stale, "the next frame must rebuild campaign uses");
        let loaded = load_all(&[module], Some(&campaign)).unwrap();
        assert!(
            loaded.uses.unwrap().contains(&id),
            "the repaired note is Chosen for this campaign"
        );
    }

    #[test]
    fn scope_picker_fits_whole_words_and_reuses_unchanged_choices() {
        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let budget = design::number_width(ui, "Ashen…", &Type::Strong.font(ui));
                assert_eq!(scope_label(ui, "Ashen Marches", budget), "Ashen…");
                assert_eq!(
                    scope_label(ui, "Ashen Marches", budget / 2.0)
                        .matches('…')
                        .count(),
                    1
                );
            });
        });

        let temp = tempfile::tempdir().unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        let package = |folder: &str, name: &str| Module {
            id: Uuid::new_v4(),
            name: name.into(),
            path: temp.path().join(folder),
        };
        let first = package("first", "First package");
        let second = package("second", "Ashen Marches");
        campaigns.register_module(first.clone()).unwrap();
        campaigns.register_module(second.clone()).unwrap();

        ctx.enable_accesskit();
        let scale = std::cell::Cell::new(1.0);
        let mut cache = None;
        let mut draw = |campaigns: &Campaigns, current: Scope, width: f32, expected: &str| {
            use crate::campaigns::MODULE_READS;
            MODULE_READS.set(0);
            SCOPE_LABEL_FITS.set(0);
            let mut scope = current;
            let mut section = Section::All;
            let mut input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 480.0),
                )),
                ..Default::default()
            };
            input
                .viewports
                .get_mut(&egui::ViewportId::ROOT)
                .unwrap()
                .native_pixels_per_point = Some(scale.get());
            let output = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    Library::scope_menu(ui, true, campaigns, &mut scope, &mut section, &mut cache);
                });
            });
            let name = output
                .platform_output
                .accesskit_update
                .unwrap()
                .nodes
                .into_iter()
                .find(|(_, node)| node.role() == egui::accesskit::Role::ComboBox)
                .and_then(|(_, node)| node.label().map(str::to_owned));
            assert_eq!(name.as_deref(), Some(expected), "closed scope name");
            assert!(
                MODULE_READS.get() <= 1,
                "the picker reads the catalogue at most once per rebuild, got {}",
                MODULE_READS.get()
            );
            (MODULE_READS.get(), SCOPE_LABEL_FITS.get())
        };
        let selected = Scope::Package(second.id);
        let ashen = "What the Library shows: Ashen Marches";
        let renamed = "What the Library shows: Renamed package";
        assert_eq!(draw(&campaigns, selected, 100.0, ashen).1, 4);
        for _ in 0..3 {
            assert_eq!(
                draw(&campaigns, selected, 100.0, ashen),
                (0, 0),
                "steady closed frame"
            );
        }
        campaigns
            .rename_module(second.id, "Renamed package".into())
            .unwrap();
        assert_eq!(draw(&campaigns, selected, 100.0, renamed).1, 4);
        // Restoring the first package moves it behind the selected package.
        campaigns.restore_module(first).unwrap();
        assert_eq!(draw(&campaigns, selected, 100.0, renamed).1, 4);
        assert_eq!(draw(&campaigns, selected, 200.0, renamed).1, 4, "new width");
        ctx.style_mut(|style| {
            style
                .text_styles
                .insert(Type::Strong.style(), egui::FontId::proportional(21.0));
        });
        assert_eq!(draw(&campaigns, selected, 200.0, renamed).1, 4, "new font");
        scale.set(2.0);
        assert_eq!(
            draw(&campaigns, selected, 200.0, renamed).1,
            4,
            "new display scale"
        );
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .reverse();
        ctx.set_fonts(fonts);
        assert_eq!(
            draw(&campaigns, selected, 200.0, renamed).1,
            4,
            "replaced font family"
        );
        campaigns.open_for_test(&sd1_campaign(temp.path()));
        assert_eq!(
            draw(
                &campaigns,
                Scope::Campaign,
                200.0,
                "What the Library shows: Campaign entries"
            )
            .1,
            6
        );
        assert_eq!(draw(&campaigns, selected, 200.0, renamed), (0, 0));
        campaigns.unregister_module(second.id).unwrap();
        assert_eq!(
            draw(&campaigns, selected, 200.0, "What the Library shows").1,
            5
        );
        assert_eq!(
            draw(&campaigns, selected, 200.0, "What the Library shows"),
            (0, 0),
            "missing selection"
        );
    }

    #[test]
    fn creature_template_uses_fields_on_every_read_surface() {
        let text = note_text("Copper Drake", "creature", None);
        assert!(
            !text
                .lines()
                .any(|line| line.trim_start().starts_with("value:"))
        );
        assert!(text.contains("| Speed | 30 ft. |"));
        assert!(text.contains("splat_schema: 2"));
        assert!(!text.contains("splat_stats:"));
        let entry = Entry::parse("Copper Drake.md", &text).unwrap();
        assert!(entry.fields["splat_stats"]["max_hp"].get("value").is_none());
        assert!(
            entry.fields["splat_stats"]["armor_class"]
                .get("value")
                .is_none()
        );
        assert_eq!(
            entry.fields["splat_creature"]["type"].as_str(),
            Some("humanoid")
        );
        for key in [
            "strength",
            "dexterity",
            "constitution",
            "intelligence",
            "wisdom",
            "charisma",
        ] {
            assert_eq!(entry.fields["splat_stats"][key]["value"].as_i64(), Some(10));
        }
        assert_eq!(
            presentation::challenge_text(&entry),
            Some("1 (XP 200; PB +2)")
        );
        assert_eq!(presentation::subtitle(&entry), "Creature · CR 1");
        assert!(!entry.body.contains("Speed 30"));
        assert!(!entry.body.contains("Str 10"));
        assert!(!entry.body.contains("CR 1"));
        let mut conflicting = entry.clone();
        conflicting.body.push_str("\nCR 9 (XP 5,000; PB +4)\n");
        assert_eq!(
            presentation::challenge_text(&conflicting),
            Some("1 (XP 200; PB +2)")
        );
        assert_eq!(presentation::subtitle(&conflicting), "Creature · CR 1");
    }

    #[test]
    fn heading_repair_changes_only_the_first_h1() {
        let text = note_text("Watch Ashfall", "spell", None)
            .replace("# Watch Ashfall", "# Ashfall")
            + "\n# Another heading\n";
        let entry = Entry::parse("Ashfall.md", &text).unwrap();
        let (old, after) = presentation::heading_fix(&text, &entry).unwrap();
        assert_eq!(old, "Ashfall");
        assert_eq!(after, text.replacen("# Ashfall", "# Watch Ashfall", 1));
    }

    #[test]
    fn back_keeps_an_unsaved_homebrew_draft() {
        let mut library = navigation_library();
        library.open_entry(0);
        library.open_entry(1);
        library.draft.push_str("Unsaved ruling.");
        let draft = library.draft.as_str().to_owned();
        library.go(Step::Back);
        assert_eq!(library.selected, Some(1));
        assert_eq!(library.draft.as_str(), draft);
        assert!(library.trail.peek(Step::Back).is_some());
    }

    #[test]
    fn forward_returns_to_what_back_left_until_another_link() {
        let mut library = navigation_library();
        library.open_entry(0);
        library.open_entry(1);
        library.go(Step::Back);
        assert_eq!(library.selected, Some(0));
        library.go(Step::Forward);
        assert_eq!(library.selected, Some(1));
        library.go(Step::Back);
        library.open_entry(2);
        assert!(
            library.trail.peek(Step::Forward).is_none(),
            "a new link starts a new trail"
        );
    }

    #[test]
    fn back_returns_to_the_right_owner_and_scope() {
        let mut library = navigation_library();
        let owner = library.loaded.as_ref().unwrap().notes[1]
            .owner
            .id()
            .unwrap();
        library.scope = Scope::Package(owner);
        library.open_entry(1);
        library.search = "Second".into();
        library.open_entry(2);
        library.scope = Scope::All;
        library.go(Step::Back);
        assert_eq!(library.selected, Some(1));
        assert_eq!(library.scope, Scope::Package(owner));
        assert_eq!(library.search, "Second");
    }

    #[test]
    fn following_a_link_reveals_an_entry_outside_the_package_scope() {
        let mut library = navigation_library();
        let owner = library.loaded.as_ref().unwrap().notes[1]
            .owner
            .id()
            .unwrap();
        library.scope = Scope::Package(owner);
        library.open_entry(1);
        library.open_entry(2);
        let loaded = library.loaded.as_ref().unwrap();
        assert!(loaded.in_scope(&loaded.notes[2], library.scope));
        library.go(Step::Back);
        assert_eq!(library.scope, Scope::Package(owner));
    }
    /// Run alone so the first load starts without a warmed process cache.
    #[test]
    #[ignore]
    fn srd_library_load_time() {
        for run in 0..3 {
            let start = std::time::Instant::now();
            let loaded = load(None).unwrap();
            assert_eq!(loaded.notes.len(), 2065);
            let load_time = start.elapsed();
            let _ = loaded.subtitles();
            let _ = loaded.count(Section::All);
            let elapsed = start.elapsed();
            eprintln!("SRD run {run}: load={load_time:?}, ready={elapsed:?}");
            assert!(elapsed.as_secs_f64() < 1.0, "SRD library must load in <1s");
        }
    }

    /// Exercise the editable package path with the same notes, read from disk.
    #[test]
    #[ignore]
    fn user_library_load_time() {
        fn copy_notes(source: &Path, destination: &Path) {
            fs::create_dir_all(destination).unwrap();
            for item in fs::read_dir(source).unwrap() {
                let path = item.unwrap().path();
                let target = destination.join(path.file_name().unwrap());
                if path.is_dir() {
                    copy_notes(&path, &target);
                } else if path.extension().is_some_and(|ext| ext == "md") {
                    fs::copy(path, target).unwrap();
                }
            }
        }
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "User package", "user").unwrap();
        copy_notes(
            &crate::workspace().join("builtin/dnd2024-definitions"),
            &module.path,
        );
        for run in 0..3 {
            let start = std::time::Instant::now();
            let loaded = load(Some(&module)).unwrap();
            assert_eq!(loaded.notes.len(), 2066);
            assert!(loaded.notes.iter().all(|note| note.entry.is_some()));
            let _ = loaded.subtitles();
            let _ = loaded.count(Section::All);
            let elapsed = start.elapsed();
            eprintln!("User package run {run}: ready={elapsed:?}");
            assert!(elapsed.as_secs_f64() < 1.0, "User library must load in <1s");
        }
    }

    #[test]
    fn duplicate_package_rows_keep_the_folder_last_and_whole() {
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(256.0, 100.0),
            )),
            ..Default::default()
        };
        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(240.0);
                let response = entry_row(
                    ui,
                    "Cinder Wolf",
                    "Same entry in two files · Creature · CR 1 · Ashen Marches · ashen-marches-copy/",
                    false,
                );
                assert_eq!(response.rect.height(), ENTRY_ROW_HEIGHT);
            });
        });
        // Round 25 FF5: the folder ends the row, as it ends the reader's kind line. The row is
        // too narrow for the whole line, so the words before the folder shorten, never the folder.
        let lines: Vec<String> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text.starts_with("Same entry") => {
                    Some(text.galley.job.text.clone())
                }
                _ => None,
            })
            .collect();
        assert!(
            lines.len() == 1
                && lines[0].starts_with("Same entry")
                && lines[0].contains('…')
                && lines[0].ends_with(" · ashen-marches-copy/"),
            "{lines:?}"
        );
    }

    #[test]
    fn entry_rows_keep_both_lines_at_a_fixed_height() {
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(186.0, 100.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(170.0);
                    let response = entry_row(
                        ui,
                        &"Long entry name ".repeat(30),
                        &"Long subtitle ".repeat(30),
                        false,
                    );
                    assert_eq!(response.rect.height(), 44.0);
                });
            },
        );
        let rows: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if !text.galley.job.text.is_empty() => Some(&text.galley),
                _ => None,
            })
            .collect();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.iter()
                .all(|galley| galley.rows.len() == 1 && galley.rect.width() <= 162.0)
        );
    }

    #[test]
    fn list_metadata_is_refreshed_after_content_changes() {
        let mut loaded = Loaded::fixture(vec![Note {
            path: "first.md".into(),
            text: crate::design::memo::Revised::default(),
            entry: None,
            error: None,
            fault: None,
            manifest_gone: false,
            owner: Owner::Builtin,
        }]);
        assert_eq!(loaded.count(Section::All), 1);
        assert_eq!(loaded.subtitles().len(), 1);
        loaded.notes.push(Note {
            path: "second.md".into(),
            text: crate::design::memo::Revised::default(),
            entry: None,
            error: None,
            fault: None,
            manifest_gone: false,
            owner: Owner::Builtin,
        });
        loaded.invalidate();
        assert_eq!(loaded.count(Section::All), 2);
        assert_eq!(loaded.subtitles().len(), 2);
    }

    #[test]
    fn reader_names_the_package_once() {
        let package = Owner::Package(Module {
            id: Uuid::new_v4(),
            name: "Ashen Marches".into(),
            path: "ashen".into(),
        });
        // The list leaves the built-in package unsaid, and already names any other.
        assert_eq!(
            reader_subtitle(&Owner::Builtin, "Spell · Level 3", None, None),
            format!("Spell · Level 3 · {BUILTIN_NAME}")
        );
        assert_eq!(
            reader_subtitle(&package, "Spell · Level 3 · Ashen Marches", None, None),
            "Spell · Level 3 · Ashen Marches"
        );
        // Round 30 FD10: the reader names a spell's school with its level, as the books do;
        // the list keeps its short line.
        let spell = |fields: &str| spell_kind(&serde_yaml_ng::from_str(fields).unwrap());
        assert_eq!(
            reader_subtitle(
                &package,
                "Spell · Level 1 · Ashen Marches",
                None,
                spell("level: 1\nschool: evocation\n")
            ),
            "Spell · Level 1 Evocation · Ashen Marches"
        );
        assert_eq!(
            reader_subtitle(
                &package,
                "Spell · Cantrip",
                None,
                spell("level: 0\nschool: illusion\n")
            ),
            "Spell · Illusion Cantrip"
        );
        assert_eq!(spell("level: 1\n"), None);
        // A class page's own line replaces the list's, so it needs the name too.
        assert_eq!(
            reader_subtitle(&package, "Rule", Some("Bard · Level 3".into()), None),
            "Class feature · Bard · Level 3 · Ashen Marches"
        );
        // Round 23 FE8: the kind line under the owner line leaves the package name out. Where
        // two packages share the name, the folder alone tells them apart.
        assert_eq!(
            reader_kind_line(&package, "Creature · CR 1 · Ashen Marches"),
            "Creature · CR 1"
        );
        assert_eq!(
            reader_kind_line(&package, "Creature · CR 1 · Ashen Marches · ashen/"),
            "Creature · CR 1 · ashen/"
        );
        // Round 24 AC2, SC1: a duplicate's reader leaves out its row's fault, and keeps the
        // folder that tells the two files apart.
        assert_eq!(
            reader_kind_line(
                &package,
                &row_line_without_fault(&format!(
                    "{} {SAME_ENTRY} · Creature · CR 1 · Ashen Marches · ashen/",
                    icons::WARN
                ))
            ),
            "Creature · CR 1 · ashen/"
        );
        // Round 25 AC2: a row with two causes names both, and the reader leaves both out.
        assert_eq!(
            reader_kind_line(
                &package,
                &row_line_without_fault(&format!(
                    "{} {SAME_ENTRY} · {HEADING_DIFFERS} · Creature · CR 1 · Ashen Marches · ashen/",
                    icons::WARN
                ))
            ),
            "Creature · CR 1 · ashen/"
        );
        // Lookalike: another owner's line keeps every part.
        let campaign = Owner::Campaign(Campaign {
            id: Uuid::new_v4(),
            name: "Ashen Marches".into(),
            path: "ashen".into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        });
        assert_eq!(
            reader_kind_line(&campaign, "Creature · CR 1 · Ashen Marches · ashen/"),
            "Creature · CR 1 · Ashen Marches · ashen/"
        );
    }

    /// Run explicitly to measure the complete library UI while scrolling the built-in package.
    #[test]
    #[ignore]
    fn library_scroll_frame_times() {
        let mut library = Library {
            loaded: Some(load(None).unwrap()),
            ..Default::default()
        };
        let mut campaigns = Campaigns::default();
        let ctx = egui::Context::default();
        let mut times = Vec::new();
        for frame in 0..150 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                time: Some(frame as f64 / 60.0),
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(300.0, 450.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, -30.0),
                        modifiers: Default::default(),
                    },
                ],
                ..Default::default()
            };
            let start = std::time::Instant::now();
            let output = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
            });
            let _ = ctx.tessellate(output.shapes, output.pixels_per_point);
            if frame >= 30 {
                times.push(start.elapsed().as_secs_f64() * 1000.0);
            }
        }
        times.sort_by(f64::total_cmp);
        println!(
            "Library entries: {}; frame CPU ms: median={:.3}, p95={:.3}, max={:.3}",
            library.loaded.as_ref().unwrap().notes.len(),
            times[times.len() / 2],
            times[times.len() * 95 / 100],
            times.last().unwrap()
        );
    }

    #[test]
    fn repeated_rule_headings_name_their_subsection() {
        let loaded = load(None).unwrap();
        let subtitles = loaded.subtitles();
        for (suffix, context) in [
            ("ability_modifier", "Ability Checks"),
            ("ability_modifier_2", "Saving Throws"),
            ("ability_modifier_3", "Attack Rolls"),
        ] {
            let row = loaded
                .notes
                .iter()
                .position(|note| {
                    note.entry.as_ref().and_then(|entry| entry.key.as_deref())
                        == Some(format!("dnd2024:rules/d20_tests_{suffix}").as_str())
                })
                .unwrap();
            assert_eq!(subtitles[row], format!("Rule · D20 Tests › {context}"));
        }
    }

    #[test]
    fn same_name_entries_have_distinct_visible_labels() {
        let loaded = load(None).unwrap();
        let subtitles = loaded.subtitles();
        let labels: std::collections::HashSet<_> = loaded
            .notes
            .iter()
            .zip(subtitles)
            .map(|(note, subtitle)| (note.name(), subtitle))
            .collect();
        assert_eq!(labels.len(), loaded.notes.len());
    }

    #[test]
    fn imported_overview_renders_as_prose_without_source_markup() {
        for (path, expected) in [
            (
                "definitions/glossary/ability_check.md",
                "represents using one of the six abilities",
            ),
            (
                "definitions/spells/bless.md",
                "Whenever a target makes an attack roll",
            ),
            (
                "definitions/spells/command.md",
                "Approach. The target moves toward you by the shortest",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier_2.md",
                "Physically resist direct force",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier.md",
                "Notice things in the environment or in creatures’ behavior",
            ),
            (
                "definitions/spells/divine_smite.md",
                "if the target is a Fiend or an Undead.",
            ),
            (
                "definitions/spells/augury.md",
                "circumstances, such as other spells, that might change the results.",
            ),
        ] {
            let text = content::definitions::source_file(path).unwrap().to_owned();
            let entry = Entry::parse(path, &text).unwrap();
            let overview = presentation::reading_sections(&entry.body, &entry.name)
                .into_iter()
                .find(|section| section.title == "Source text")
                .unwrap();
            let readable = presentation::overview_markdown(&overview.body);
            if path.ends_with("command.md") {
                for option in ["Approach.", "Drop.", "Flee.", "Grovel.", "Halt."] {
                    assert!(readable.contains(&format!("\n\n{option}")), "{readable}");
                }
                assert!(
                    readable.contains("holding and then ends its turn."),
                    "{readable}"
                );
            }
            if path.ends_with("augury.md") {
                assert!(
                    readable.contains(
                        "| Weal | Good |\n| Woe | Bad |\n| Weal and woe | Good and bad |"
                    ),
                    "{readable}"
                );
            }
            let mut library = Library {
                loaded: Some(Loaded::fixture(vec![Note {
                    path: entry.path.clone(),
                    text: crate::design::memo::Revised::new(text.clone()),
                    entry: Some(entry),
                    error: None,
                    fault: None,
                    manifest_gone: false,
                    owner: Owner::Builtin,
                }])),
                selected: Some(0),
                draft: (text).into(),
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let subtitles = library.loaded.as_ref().unwrap().subtitles().clone();
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 700.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, &subtitles[0], &Campaigns::default())
                    });
                },
            );
            let text = output
                .shapes
                .iter()
                .filter_map(|s| match &s.shape {
                    egui::Shape::Text(t) => Some(t.galley.job.text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            assert!(text.contains(expected), "{text}");
            assert!(!text.contains('\\'), "{text}");
            assert!(!text.contains("[["), "{text}");
            assert!(!text.contains("Original layout:"), "{text}");
        }
    }
    #[test]
    fn creature_spell_links_open_named_spells_and_protect_drafts() {
        let mut notes: Vec<Note> = [
            "creatures/adult_bronze_dragon",
            "spells/detect_magic",
            "spells/guiding_bolt",
            "spells/shapechange",
            "spells/speak_with_animals",
            "spells/thaumaturgy",
            "spells/detect_thoughts",
            "spells/water_breathing",
            "creatures/druid",
        ]
        .map(|name| {
            let path = format!("definitions/{name}.md");
            let source = content::definitions::source_file(&path).unwrap();
            Note {
                entry: Some(Entry::parse(&path, source).unwrap()),
                path,
                text: crate::design::memo::Revised::new(source.into()),
                error: None,
                fault: None,
                manifest_gone: false,
                owner: Owner::Builtin,
            }
        })
        .into_iter()
        .collect();
        // The Druid's list lines end with `\`, and here it also casts a spell no note has.
        let druid = notes.last_mut().unwrap();
        druid.text = crate::design::memo::Revised::new(druid.text.replacen(
            "\n\nSpellcasting. The druid",
            "\n\nDread Word. The druid casts Vellora's Storm, using the same spellcasting ability as Spellcasting.\n\nSpellcasting. The druid",
            1,
        ));
        druid.entry = Some(Entry::parse(&druid.path, &druid.text).unwrap());
        let loaded = Loaded::fixture(notes);
        let references = creature::spell_references(loaded.notes[0].entry.as_ref().unwrap());
        assert_eq!(references.len(), 7);
        for (reference, index) in references.iter().zip(1..8) {
            assert_eq!(loaded.resolve_spell(reference), Some(index));
        }
        // The Druid keeps its list under `options`, not `spells`.
        let path = "definitions/creatures/druid.md";
        let druid = Entry::parse(path, content::definitions::source_file(path).unwrap()).unwrap();
        let keys: Vec<_> = creature::spell_references(&druid)
            .into_iter()
            .map(|reference| match reference {
                content::Reference::Key(key) => key,
                content::Reference::Id(id) => id.to_string(),
            })
            .collect();
        assert_eq!(
            keys.first().map(String::as_str),
            Some("dnd2024:spells/druidcraft")
        );
        assert_eq!(
            keys.last().map(String::as_str),
            Some("dnd2024:spells/moonbeam")
        );
        assert_eq!(keys.len(), 7);
        let by_id = content::Reference::Id(loaded.notes[2].entry.as_ref().unwrap().id);
        assert_eq!(loaded.resolve_spell(&by_id), Some(2));
        assert_eq!(
            loaded.resolve_spell(&content::Reference::Key("missing:spell".into())),
            None
        );

        let mut library = Library {
            draft: loaded.notes[0].text.as_str().into(),
            loaded: Some(loaded),
            selected: Some(0),
            section: Section::Creatures,
            search: "bronze".into(),
            ..Default::default()
        };
        for width in [390.0, 900.0] {
            for dirty in [true, false] {
                library.open_entry(0);
                library.search = "bronze".into();
                library.matches = Some((Section::Creatures, "bronze".into(), vec![0]));
                if dirty {
                    library.draft.push_str("\nUnsaved note.\n");
                }
                let ctx = egui::Context::default();
                // The Library is lent to each frame, so the test can change its draft between.
                let frame = |library: &mut Library, events: Vec<egui::Event>| {
                    ctx.run(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(
                                egui::Pos2::ZERO,
                                egui::vec2(width, 6000.0),
                            )),
                            events,
                            ..Default::default()
                        },
                        |ctx| {
                            egui::CentralPanel::default().show(ctx, |ui| {
                                library.editor(ui, "Definition · creature", &Campaigns::default())
                            });
                        },
                    )
                };
                frame(&mut library, vec![]);
                let output = frame(&mut library, vec![]);
                // The stat block's spells were resolved once, with the notes' other caches:
                // another unchanged frame looks none up. Layout PR 9b-R1c: nor does the shared
                // view read the entry again.
                let looked = SPELL_LOOKUPS.with(std::cell::Cell::get);
                let read = crate::library::view::ENTRY_PREPARES.with(std::cell::Cell::get);
                frame(&mut library, vec![]);
                assert_eq!(
                    SPELL_LOOKUPS.with(std::cell::Cell::get),
                    looked,
                    "an unchanged frame looks no spell up"
                );
                assert_eq!(
                    crate::library::view::ENTRY_PREPARES.with(std::cell::Cell::get),
                    read,
                    "9b-R1c: an unchanged frame reads the entry again in no view"
                );
                let texts: Vec<_> = output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text),
                        _ => None,
                    })
                    .collect();
                // Each spell is a tile under its list's label; no line repeats them.
                let find = |words: &str| {
                    texts
                        .iter()
                        .find(|t| t.galley.job.text == words)
                        .unwrap_or_else(|| panic!("{words} is drawn"))
                };
                let (detect, bolt) = (find("Detect Magic"), find("Guiding Bolt"));
                assert!(find("At Will").pos.y < detect.pos.y);
                assert!(
                    !texts
                        .iter()
                        .any(|t| t.galley.job.text.contains("Spell descriptions"))
                );
                // Guiding Bolt is cast at level 2: one more die, and the dragon's bonus to hit.
                assert!(
                    texts.iter().any(|t| t.galley.job.text.starts_with("+10 ·")
                        && t.galley.job.text.contains("5d6"))
                );
                assert!(
                    texts
                        .iter()
                        .any(|t| t.galley.job.text.starts_with("Level 2 ·"))
                );
                // A spell's facts come from its entry; a range of Self from its printed text.
                assert!(
                    texts
                        .iter()
                        .any(|t| t.galley.job.text == "Level 1 · Action or Ritual · Self"),
                    "Speak with Animals keeps its range"
                );
                let spellcasting = texts
                    .iter()
                    .find(|t| t.galley.job.text.starts_with("Spellcasting."))
                    .unwrap();
                let legendary = find("Legendary Actions");
                assert!(bolt.pos.y > spellcasting.pos.y && bolt.pos.y < legendary.pos.y);
                assert!(!texts.iter().any(|t| t.galley.job.text.contains("^splat-")));
                let centre = |t: &egui::epaint::TextShape| t.pos + t.galley.rect.center().to_vec2();
                // Detect Magic rolls nothing, so a click opens it. Guiding Bolt rolls, so its
                // menu reads it.
                let read_by_menu = width > 400.0 && !dirty;
                let position = centre(if read_by_menu { bolt } else { detect });
                let button = if read_by_menu {
                    egui::PointerButton::Secondary
                } else {
                    egui::PointerButton::Primary
                };
                frame(&mut library, vec![egui::Event::PointerMoved(position)]);
                for pressed in [true, false] {
                    frame(
                        &mut library,
                        vec![egui::Event::PointerButton {
                            pos: position,
                            button,
                            pressed,
                            modifiers: egui::Modifiers::NONE,
                        }],
                    );
                }
                if read_by_menu {
                    let output = frame(&mut library, vec![]);
                    let read = output
                        .shapes
                        .iter()
                        .find_map(|shape| match &shape.shape {
                            egui::Shape::Text(text)
                                if text.galley.job.text == "Read Guiding Bolt" =>
                            {
                                Some(text.pos + text.galley.rect.center().to_vec2())
                            }
                            _ => None,
                        })
                        .expect("the menu reads Guiding Bolt");
                    frame(&mut library, vec![egui::Event::PointerMoved(read)]);
                    for pressed in [true, false] {
                        frame(
                            &mut library,
                            vec![egui::Event::PointerButton {
                                pos: read,
                                button: egui::PointerButton::Primary,
                                pressed,
                                modifiers: egui::Modifiers::NONE,
                            }],
                        );
                    }
                }
                if dirty {
                    assert_eq!(library.selected, Some(0));
                    assert!(library.draft.ends_with("Unsaved note.\n"));
                    let _ = library
                        .draft
                        .replace(library.loaded.as_ref().unwrap().notes[0].text.as_str());
                } else {
                    let (place, title) = if read_by_menu {
                        (2, "# Guiding Bolt")
                    } else {
                        (1, "# Detect Magic")
                    };
                    assert_eq!(library.selected, Some(place));
                    assert_eq!(library.section, Section::Spells);
                    assert!(library.search.is_empty());
                    assert!(library.matches.is_none());
                    assert!(!library.source);
                    assert!(library.draft.contains(title));
                    // Layout PR 9b-R1c (lead brief Sol 2-017): the followed link is a production
                    // event that changes what the reader shows. The next frame shows the spell's
                    // own words, and the frame after it reuses them: no reading of the entry, the
                    // same words. (A spell's source text is still made and parsed on each frame,
                    // in the section reader: layout PR 9b-R1c-2b.)
                    let words = |output: &egui::FullOutput| {
                        output
                            .shapes
                            .iter()
                            .filter_map(|shape| match &shape.shape {
                                egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                    };
                    let spell = &library.loaded.as_ref().unwrap().notes[place];
                    let prose = spell
                        .entry
                        .as_ref()
                        .unwrap()
                        .body
                        .lines()
                        .map(str::trim)
                        // Its description: the longest paragraph, not a heading, a list, a table
                        // or a line of facts.
                        .filter(|line| {
                            !line.starts_with(['#', '|', '-', '*', '>']) && !line.ends_with('\\')
                        })
                        .max_by_key(|line| line.len())
                        .expect("the spell has a line of prose")
                        .to_owned();
                    let opened = words(&frame(&mut library, vec![]));
                    let counted = crate::library::view::ENTRY_PREPARES.with(std::cell::Cell::get);
                    let again = words(&frame(&mut library, vec![]));
                    let shows = |words: &[String]| {
                        let start: String = prose.chars().take(24).collect();
                        words.iter().any(|text| text.contains(&start))
                    };
                    assert!(
                        shows(&opened) && !opened.iter().any(|text| text == "Legendary Actions"),
                        "9b-R1c-opened: the followed spell's words show ({prose:?}), not the \
                         dragon's: {opened:?}"
                    );
                    assert!(
                        again == opened
                            && crate::library::view::ENTRY_PREPARES.with(std::cell::Cell::get)
                                == counted,
                        "9b-R1c-opened-reuse: the next frame reads the entry again in no view"
                    );
                    // The draft changed as an edit leaves it (`TextDraft::replace`), the same
                    // entry: the Library's revision is the draft's parse, so the next frame shows
                    // the new words, and the frame after it reuses them.
                    let text = library.draft.to_string();
                    let start: String = prose.chars().take(24).collect();
                    let upper = start.to_uppercase();
                    let _ = library.draft.replace(text.replacen(&start, &upper, 1));
                    let edited = words(&frame(&mut library, vec![]));
                    let counted = crate::library::view::ENTRY_PREPARES.with(std::cell::Cell::get);
                    let again = words(&frame(&mut library, vec![]));
                    assert!(
                        edited.iter().any(|text| text.contains(&upper))
                            && !shows(&edited)
                            && again == edited
                            && crate::library::view::ENTRY_PREPARES.with(std::cell::Cell::get)
                                == counted,
                        "9b-R1c-draft: the edited draft's words show ({upper:?}), then the next \
                         frame reuses them: {edited:?}"
                    );
                    let _ = library.draft.replace(&text);
                }
            }
        }
        // A stat block whose lists end their lines with `\` and that casts a spell no note has:
        // an unchanged frame looks none up either.
        library.open_entry(8);
        let ctx = egui::Context::default();
        let mut frame = || {
            ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(900.0, 6000.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Definition · creature", &Campaigns::default())
                    });
                },
            )
        };
        frame();
        let output = frame();
        let drawn = |words: &str| {
            output.shapes.iter().any(|shape| {
                matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.starts_with(words))
            })
        };
        assert!(
            drawn("Dread Word.") && drawn("At Will"),
            "the Druid is drawn"
        );
        let looked = SPELL_LOOKUPS.with(std::cell::Cell::get);
        frame();
        assert_eq!(
            SPELL_LOOKUPS.with(std::cell::Cell::get),
            looked,
            "an unchanged frame of the Druid looks no spell up"
        );
        // A change to the notes clears what was resolved through them, with their other caches.
        let loaded = library.loaded.as_mut().unwrap();
        let dragon = loaded.notes[0].entry.clone().unwrap();
        spells::resolve(&dragon, &*loaded);
        let looked = SPELL_LOOKUPS.with(std::cell::Cell::get);
        spells::resolve(&dragon, &*loaded);
        assert_eq!(SPELL_LOOKUPS.with(std::cell::Cell::get), looked);
        loaded.invalidate();
        spells::resolve(&dragon, &*loaded);
        assert!(
            SPELL_LOOKUPS.with(std::cell::Cell::get) > looked,
            "a change resolves again"
        );
        // Conflicting identities must not choose an arbitrary destination.
        let duplicate = &loaded.notes[2];
        loaded.notes.push(Note {
            path: "duplicate.md".into(),
            text: duplicate.text.clone(),
            entry: duplicate.entry.clone(),
            error: None,
            fault: None,
            manifest_gone: false,
            owner: Owner::Builtin,
        });
        assert_eq!(loaded.resolve_spell(&by_id), None);
    }

    #[test]
    fn creature_read_view_hides_rule_anchors() {
        let path = "definitions/creatures/adult_bronze_dragon.md";
        let source = content::definitions::source_file(path).unwrap();
        let mut library = Library {
            loaded: Some(Loaded::fixture(vec![Note {
                path: path.into(),
                text: crate::design::memo::Revised::new(source.into()),
                entry: Some(Entry::parse(path, source).unwrap()),
                error: None,
                fault: None,
                manifest_gone: false,
                owner: Owner::Builtin,
            }])),
            selected: Some(0),
            draft: source.into(),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 6000.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    library.editor(ui, "Definition · creature", &Campaigns::default())
                });
            },
        );
        let text = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(text.iter().any(|text| text.contains("Spellcasting.")));
        assert!(
            !text.iter().any(|text| text.contains("^splat-")),
            "{text:?}"
        );
        assert!(
            !text.contains(&"Rule"),
            "Empty mechanics section should be hidden"
        );
    }

    #[test]
    fn reviewed_summary_keeps_rules_and_source_and_opens_related_entry() {
        let loaded = load(None).unwrap();
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == "Level 3: Bard Subclass")
            .unwrap();
        let entry = loaded.notes[index].entry.as_ref().unwrap();
        let mut original = entry.clone();
        original.fields.remove("splat_reading");
        assert_eq!(
            content::contracts::specification(entry).unwrap(),
            content::contracts::specification(&original).unwrap()
        );
        assert_eq!(
            serde_json::to_value(content::required_references(entry).unwrap()).unwrap(),
            serde_json::to_value(content::required_references(&original).unwrap()).unwrap()
        );
        assert!(
            entry
                .body
                .contains("of Lore subclass is detailed after this class’s description")
        );
        for width in [390.0, 900.0] {
            let mut library = Library {
                loaded: Some(load(None).unwrap()),
                ..Default::default()
            };
            library.open_entry(index);
            let ctx = egui::Context::default();
            let mut frame = |events| {
                ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 1400.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(
                                ui,
                                "Definition · class feature · Bard",
                                &Campaigns::default(),
                            )
                        });
                    },
                )
            };
            frame(vec![]);
            let output = frame(vec![]);
            let texts: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text),
                    _ => None,
                })
                .collect();
            assert!(
                texts
                    .iter()
                    .all(|text| text.pos.x + text.galley.size().x <= width)
            );
            let rendered = texts
                .iter()
                .map(|text| text.galley.job.text.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            for expected in [
                "Bard Subclass",
                "Bard · Level 3 · D&D 2024 · SRD 5.2.1",
                "Choose a Bard subclass.",
                "Subclasses",
                "Source text",
            ] {
                assert!(rendered.contains(expected), "{rendered}");
            }
            assert!(!rendered.contains("You gain a Bard subclass of your choice."));
            // The published wording is still readable, and the related link navigates.
            for label in ["Source text", "College of Lore"] {
                let current = frame(vec![]);
                let text = current
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text),
                        _ => None,
                    })
                    .find(|text| text.galley.job.text == label)
                    .unwrap();
                let pos = text.pos + text.galley.size() * 0.5;
                for pressed in [true, false] {
                    frame(vec![
                        egui::Event::PointerMoved(pos),
                        egui::Event::PointerButton {
                            pos,
                            button: egui::PointerButton::Primary,
                            pressed,
                            modifiers: Default::default(),
                        },
                    ]);
                }
                if label == "Source text" {
                    let output = frame(vec![]);
                    assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("You gain a Bard subclass of your choice."))));
                }
            }
            assert!(library.draft.contains("# College of Lore"));
        }
    }

    #[test]
    fn library_toolbar_keeps_back_visible_with_more_menu() {
        let loaded = load(None).unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(0);
        library.open_entry(1);
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let mut rendered = Vec::new();
        for _ in 0..3 {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(360.0, 640.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Entry", &Campaigns::default());
                    });
                },
            );
            rendered = output
                .shapes
                .into_iter()
                .filter_map(|shape| match shape.shape {
                    egui::Shape::Text(text)
                        if shape.clip_rect.contains_rect(text.visual_bounding_rect()) =>
                    {
                        Some(text.galley.job.text.clone())
                    }
                    _ => None,
                })
                .collect();
        }
        for word in ["Read", "Back", "More"] {
            assert!(
                rendered.iter().any(|text| text == word),
                "Missing {word}: {rendered:?}"
            );
        }

        // M2 (packet E1): a campaign creature's draft with changes and a heading that no longer
        // matches its name: `Make NPC note` waits for the draft, and the draft's reason is
        // painted once, by the save row's slot, whatever the footer holds (here the heading
        // warning), at a wide pane and behind `More` in a narrow one.
        for width in [900.0, 360.0] {
            let text = note_text("Cinder Wolf", "creature", None);
            let note = Note {
                path: "Cinder Wolf.md".into(),
                entry: Some(Entry::parse("Cinder Wolf.md", &text).unwrap()),
                text: crate::design::memo::Revised::new(text.clone()),
                error: None,
                fault: None,
                manifest_gone: false,
                owner: Owner::Campaign(Campaign {
                    id: Uuid::new_v4(),
                    name: "Ashen Marches".into(),
                    path: "ashen".into(),
                    dndbeyond_campaign_id: None,
                    dndbeyond_campaign_name: None,
                }),
            };
            let mut library = Library {
                loaded: Some(Loaded::fixture(vec![note])),
                ..Default::default()
            };
            library.open_entry(0);
            library.source = true;
            let _ = library
                .draft
                .replace(text.replacen("# Cinder Wolf", "# Ash Wolf", 1));
            let ctx = egui::Context::default();
            crate::design::theme::install(&ctx);
            let mut painted = Vec::new();
            for _ in 0..4 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 800.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(ui, "Entry", &Campaigns::default());
                        });
                    },
                );
                painted = output
                    .shapes
                    .into_iter()
                    .filter_map(|shape| match shape.shape {
                        egui::Shape::Text(text) if text.fallback_color.a() > 0 => {
                            Some(text.galley.job.text.clone())
                        }
                        _ => None,
                    })
                    .collect();
            }
            assert!(
                painted
                    .iter()
                    .any(|text| text.starts_with("The first heading says")),
                "at {width}: the footer holds the heading warning: {painted:?}"
            );
            let reason = design::DraftBlocks::EntryInCampaign.long();
            assert_eq!(
                painted
                    .iter()
                    .filter(|text| text.as_str() == reason)
                    .count(),
                1,
                "at {width}: the draft's reason is painted once: {painted:?}"
            );
        }
    }

    #[test]
    fn source_editor_keeps_its_frame_inside_the_remaining_height() {
        for size in [egui::vec2(480.0, 480.0), egui::vec2(960.0, 920.0)] {
            let source = format!(
                "{}\n{}",
                note_text("Long rule", "rule", None),
                "Long source line.\n".repeat(200)
            );
            let mut library = Library {
                loaded: Some(Loaded::fixture(vec![Note {
                    path: "Long rule.md".into(),
                    text: crate::design::memo::Revised::new(source.clone()),
                    entry: Some(Entry::parse("Long rule.md", &source).unwrap()),
                    error: None,
                    fault: None,
                    manifest_gone: false,
                    owner: Owner::Builtin,
                }])),
                selected: Some(0),
                source: true,
                draft: (source).into(),
                ..Default::default()
            };
            let ctx = egui::Context::default();
            crate::design::theme::install(&ctx);
            for _ in 0..3 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(ui, "Rule", &Campaigns::default());
                        });
                    },
                );
                let frames: Vec<_> = output
                    .shapes
                    .iter()
                    .filter_map(|shape| {
                        if let egui::Shape::Rect(rect) = &shape.shape
                            && rect.stroke.color == colors().field_border
                            && rect.rect.height() > design::XXL
                        {
                            Some((rect.rect, shape.clip_rect))
                        } else {
                            None
                        }
                    })
                    .collect();
                assert!(!frames.is_empty(), "The source needs a visible frame");
                for (frame, clip) in frames {
                    assert!(
                        clip.expand(design::HAIRLINE_WIDTH).contains_rect(frame),
                        "Frame {frame:?} leaves viewport {clip:?}"
                    );
                    assert!(
                        frame.bottom() <= size.y,
                        "Frame leaves the window: {frame:?}"
                    );
                }
            }
        }
    }

    /// The words a built-in entry paints in the reader, in order.
    fn reader_text(name: &str) -> String {
        let loaded = load(None).unwrap();
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == name)
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(index);
        let ctx = egui::Context::default();
        let mut frame = || {
            ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(390.0, 2400.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Definition", &Campaigns::default())
                    });
                },
            )
        };
        frame();
        frame()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn an_item_has_one_identity_line_with_its_attunement_before_the_rules() {
        let text = reader_text("Dragon Orb");
        assert!(!text.contains("Source classification"), "{text}");
        assert!(!text.contains("wondrous item · artifact"), "{text}");
        let identity = text.find("(Requires Attunement)").expect("the book's line");
        assert!(
            identity < text.find("An orb is an etched crystal").unwrap(),
            "{text}"
        );
    }

    #[test]
    fn a_class_that_grants_its_features_links_to_them_and_its_spell_list() {
        let text = reader_text("Wizard");
        assert!(text.contains("Class features"), "{text}");
        assert!(text.find("Arcane Recovery").unwrap() < text.find("Signature Spells").unwrap());
        assert!(text.contains("Wizard Spell List"), "{text}");
        assert!(text.contains("Wizard Progression"), "{text}");
        // Row 107 (AG3): the table's own title is its one heading.
        assert_eq!(text.matches("Wizard Features").count(), 1, "{text}");
        assert!(!text.contains("Class table"), "{text}");
        // Tight rows in one case, as a spell's are, and each option on its own row.
        assert!(text.contains("Primary ability\nIntelligence"), "{text}");
        assert!(text.contains("Option A\n"), "{text}");
        assert!(text.contains("Option B\n55 GP"), "{text}");
    }

    #[test]
    fn a_subclass_lists_its_features() {
        let text = reader_text("College of Lore");
        assert!(text.contains("Subclass features"), "{text}");
        assert!(text.contains("Cutting Words"), "{text}");
    }

    #[test]
    fn class_reader_is_readable_and_opens_features() {
        let loaded = load(None).unwrap();
        // Row 144 (FG6): no built-in name ends in a colon, which reads as a cut title.
        let colons: Vec<_> = loaded
            .notes
            .iter()
            .map(Note::name)
            .filter(|name| name.ends_with(':'))
            .collect();
        assert!(loaded.notes.len() > 100 && colons.is_empty(), "{colons:?}");
        // The old names stay aliases: a saved link `[[Bard Subclass:]]` opens the renamed entry,
        // which shows its new name, and Lookup's search finds it by the old name too.
        let index = crate::lookup::ContentIndex::load(&[], None).unwrap();
        for (old, new) in [
            ("Barbarian Subclass:", "Barbarian Subclass"),
            ("Bard Subclass:", "Bard Subclass"),
            ("Druid Subclass:", "Druid Subclass"),
        ] {
            let crate::world::Go::Item(at, _) = index.resolve_link(old, None) else {
                panic!("row-144-link: [[{old}]] does not resolve");
            };
            assert_eq!(index.items[at].entry.name, new, "row-144-link");
            let found = index.search(old);
            assert!(
                found
                    .first()
                    .is_some_and(|&at| index.items[at].entry.name == new),
                "row-144-search: {old} finds {:?}",
                found
                    .iter()
                    .take(3)
                    .map(|&at| &index.items[at].entry.name)
                    .collect::<Vec<_>>()
            );
        }
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == "Barbarian")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(index);
        let ctx = egui::Context::default();
        let mut frame = |events| {
            ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(390.0, 2400.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Definition · class", &Campaigns::default())
                    });
                },
            )
        };
        frame(vec![]);
        let output = frame(vec![]);
        let texts: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text),
                _ => None,
            })
            .collect();
        // Stat-sized table cells can extend sideways inside their scroll area.
        // Prose and feature links must still fit without horizontal scrolling.
        assert!(output.shapes.iter().all(|shape| {
            let egui::Shape::Text(text) = &shape.shape else {
                return true;
            };
            text.pos.x + text.galley.size().x <= 390.0
                || (shape.clip_rect.right() <= 390.0
                    && text
                        .galley
                        .job
                        .sections
                        .iter()
                        .all(|section| section.format.font_id.size == Type::Stat.size()))
        }));
        let rendered = texts
            .iter()
            .map(|text| text.galley.job.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        // A label and its value, as a spell's facts are: on one row, in one case.
        assert!(rendered.contains("Primary ability\nStrength"), "{rendered}");
        assert!(rendered.contains("Starting equipment"));
        assert!(!rendered.contains("dnd2024:class_features/"));
        assert!(rendered.find("Rage").unwrap() < rendered.find("Relentless Rage").unwrap());
        // The class table names the feature too. The link is in the list below it. Its own
        // title is its heading, once (row 107).
        assert_eq!(
            rendered.matches("Barbarian Features").count(),
            1,
            "{rendered}"
        );
        assert!(!rendered.contains("Class table"), "{rendered}");
        let link = texts
            .iter()
            .rfind(|text| text.galley.job.text == "Relentless Rage")
            .unwrap();
        let pos = link.pos + link.galley.size() * 0.5;
        for pressed in [true, false] {
            frame(vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: Default::default(),
                },
            ]);
        }
        assert!(library.draft.contains("# Level 11: Relentless Rage"));
    }

    #[test]
    fn creature_read_view_shows_one_styled_stat_block() {
        let path = "definitions/creatures/vampire_familiar.md";
        let source = content::definitions::source_file(path).unwrap();
        let entry = Entry::parse(path, source).unwrap();
        for width in [390.0, 800.0] {
            let mut library = Library {
                loaded: Some(Loaded::fixture(vec![Note {
                    path: path.into(),
                    text: crate::design::memo::Revised::new(source.into()),
                    entry: Some(entry.clone()),
                    error: None,
                    fault: None,
                    manifest_gone: false,
                    owner: Owner::Builtin,
                }])),
                selected: Some(0),
                draft: source.into(),
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let mut rendered = String::new();
            for _ in 0..2 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 2400.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(ui, "Definition · creature", &Campaigns::default())
                        });
                    },
                );
                rendered = output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
            }
            assert_eq!(rendered.matches("65 (10d8 + 20)").count(), 1, "{rendered}");
            for expected in [
                "ARMOR CLASS",
                "STR",
                "Save +5",
                "Vampiric Connection.",
                // An action is a tile with its name; its rules text is in hover.
                "Umbral Dagger",
                "Deathless Agility.",
                "except from its vampire master",
            ] {
                assert!(
                    rendered.contains(expected),
                    "Missing {expected}: {rendered}"
                );
            }
            for duplicate in [
                "Creature data",
                "Abilities data",
                "Stats data",
                "MOD SAVE",
                "AC 15",
            ] {
                assert!(!rendered.contains(duplicate), "{rendered}");
            }
            assert_eq!(library.draft.as_str(), source);
        }
    }

    #[test]
    fn iter8_e_package_plain_readme_is_not_a_second_manifest() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::rename(
            module.path.join("Package.md"),
            module.path.join("Module.md"),
        )
        .unwrap();
        fs::write(module.path.join("Package.md"), "Notes on this adventure.").unwrap();
        let folder = content::read_folder(&module.path).unwrap();
        assert!(folder.errors.is_empty(), "{:?}", folder.errors);
        let loaded = load(Some(&module)).unwrap();
        assert!(loaded.notes.iter().all(|note| note.error.is_none()));
        assert_eq!(
            loaded
                .notes
                .iter()
                .filter(|note| note.is_manifest())
                .count(),
            1
        );
        assert!(loaded.notes.iter().all(|note| note.path != "Package.md"));
        assert_eq!(
            crate::content_workspace::package_manifest_path(&module),
            "Module.md"
        );
        assert!(
            Catalogue::load(std::slice::from_ref(&module), None)
                .unwrap()
                .manifest_problems
                .is_empty()
        );
    }

    #[test]
    fn iter8_e_deleted_manifest_notice_and_action_do_not_guess_a_filename() {
        for filename in ["Package.md", "Module.md"] {
            let temp = tempfile::tempdir().unwrap();
            let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
            if filename == "Module.md" {
                fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
            }
            fs::remove_file(module.path.join(filename)).unwrap();
            let loaded = load(Some(&module)).unwrap();
            assert_eq!(loaded.notes.len(), 1);
            assert_eq!(
                loaded.notes[0].error.as_deref(),
                Some(crate::content_workspace::DELETED_MANIFEST)
            );
            // Round 24 FC2: a deleted manifest is a warning, not the error of an unreadable one.
            assert!(
                loaded.notes[0].about_label(false).starts_with(icons::WARN),
                "iter-24-L-FC2-deleted: {}",
                loaded.notes[0].about_label(false)
            );
            assert!(!loaded.unreadable_counted, "iter-24-L-FC2-deleted-header");
            let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
            assert_eq!(
                catalogue.manifest_problems[&module.id],
                "The package manifest was deleted. Entries are still available. Write it again, or restore it from a backup."
            );
            let mut library = Library {
                loaded: Some(loaded),
                selected: Some(0),
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let mut rendered = String::new();
            for _ in 0..2 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1200.0, 2400.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor_pane(ui, "Cannot be read", &Campaigns::default(), None);
                        });
                    },
                );
                rendered = output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
            }
            assert!(
                rendered.contains("The package manifest was deleted."),
                "{rendered}"
            );
            assert!(rendered.contains("Write the manifest again"), "{rendered}");
            assert!(!rendered.contains("Missing YAML frontmatter"), "{rendered}");
            assert!(rendered.contains("Test · Manifest deleted"), "{rendered}");
            // Row 125 (G148): the owner line states the warning the About row and the message
            // state, with its icon and tone; not a muted edit pencil.
            let warning = colors().result.warning;
            assert_eq!(
                owner_line_look(&mut library, "Test · Manifest deleted"),
                Some((icons::WARN.to_owned(), warning, warning)),
                "row-125-manifest-deleted ({filename})"
            );
            assert!(rendered.contains("Show folder"), "{rendered}");
            assert!(!rendered.contains("Show file"), "{rendered}");
            assert!(!rendered.contains("Package.md"), "{rendered}");
            assert!(!rendered.contains("Module.md"), "{rendered}");
            for guess in [
                "Package.md was deleted",
                "Module.md was deleted",
                "Write Package.md again",
                "Write Module.md again",
            ] {
                assert!(!rendered.contains(guess), "{rendered}");
            }
        }
    }

    #[test]
    fn iter9_e_empty_or_malformed_manifest_still_opens_for_repair() {
        for filename in ["Package.md", "Module.md"] {
            let temp = tempfile::tempdir().unwrap();
            let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
            if filename == "Module.md" {
                fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
            }
            let path = module.path.join(filename);
            for bad in ["", "---\nsplat_id: [\n---\n"] {
                fs::write(&path, bad).unwrap();
                let loaded = load(Some(&module)).unwrap();
                let note = loaded.notes.iter().find(|note| note.is_manifest()).unwrap();
                assert_eq!(note.path, filename);
                assert_eq!(note.text.as_str(), bad);
                assert!(note.error.is_some());
                assert!(
                    crate::content_workspace::repair_problem(filename, bad)
                        .0
                        .is_some()
                );
                let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
                assert!(catalogue.manifest_problems[&module.id].starts_with(filename));
                assert!(catalogue.manifest_problems[&module.id].contains("cannot be read"));
            }
            fs::write(&path, [0xff]).unwrap();
            let loaded = load(Some(&module)).unwrap();
            assert!(
                loaded
                    .notes
                    .iter()
                    .any(|note| note.path == filename && note.error.is_some())
            );
            // Moving a broken file out and back must not leave a stale parse fault.
            let moved = module.path.join("Moved.md");
            fs::rename(&path, &moved).unwrap();
            assert!(
                load(Some(&module))
                    .unwrap()
                    .notes
                    .iter()
                    .any(|note| note.is_manifest() && note.error.is_some())
            );
            fs::remove_file(&moved).unwrap();
            fs::write(
                &path,
                content::patch_fields(
                    &note_text("Test", "package", Some("test")),
                    &[("splat_id", Some(module.id.to_string().into()))],
                )
                .unwrap(),
            )
            .unwrap();
            assert!(
                Catalogue::load(std::slice::from_ref(&module), None)
                    .unwrap()
                    .manifest_problems
                    .is_empty()
            );
            assert!(
                load(Some(&module))
                    .unwrap()
                    .notes
                    .iter()
                    .all(|note| note.error.is_none())
            );
        }
    }

    #[test]
    fn iter9_e_manifest_write_rechecks_the_offered_action() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let path = module.path.join("Package.md");
        fs::remove_file(&path).unwrap();
        // A missing-file Write must not become a replacement after an outside edit.
        let bad = "---\nsplat_id: [\n---\n";
        fs::write(&path, bad).unwrap();
        assert!(
            crate::content_workspace::ManifestWrite::write_as(&module, "Package.md", false)
                .is_err()
        );
        assert_eq!(fs::read(&path).unwrap(), bad.as_bytes());
        // Nor may a Replace become a creation after the file moves.
        let moved = module.path.join("Moved.md");
        fs::rename(&path, &moved).unwrap();
        assert!(
            crate::content_workspace::ManifestWrite::write_as(&module, "Package.md", true).is_err()
        );
        assert!(!path.exists());
        assert_eq!(fs::read(&moved).unwrap(), bad.as_bytes());
        // A README and a stale Write together still preserve the README.
        fs::write(&path, "My README.\n").unwrap();
        assert!(
            crate::content_workspace::ManifestWrite::write_as(&module, "Package.md", false)
                .is_err()
        );
        assert_eq!(fs::read(&path).unwrap(), b"My README.\n");
    }

    #[test]
    fn iter9_e_plain_manifest_named_readme_is_never_replaced() {
        // A divider and a heading underline are prose, not front matter.
        for (filename, readme) in [
            (
                "Package.md",
                "A package description without front matter.\n",
            ),
            ("Module.md", "A package description without front matter.\n"),
            ("Package.md", "Overview\n---\n\nNotes.\n\n---\n\nMore.\n"),
            ("Module.md", "Notes.\r\n\r\n---\r\n\r\nMore.\r\n"),
        ] {
            let temp = tempfile::tempdir().unwrap();
            let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
            fs::remove_file(module.path.join("Package.md")).unwrap();
            let path = module.path.join(filename);
            fs::write(&path, readme).unwrap();
            let loaded = load(Some(&module)).unwrap();
            let note = loaded.notes.iter().find(|note| note.is_manifest()).unwrap();
            assert_eq!(
                note.error.as_deref(),
                Some(crate::content_workspace::DELETED_MANIFEST)
            );
            assert_eq!(note.name(), "Test");
            assert_ne!(note.path, filename);
            let write = crate::content_workspace::ManifestWrite::write(&module).unwrap();
            assert!(!write.replaced());
            assert_ne!(write.path(), filename);
            assert_eq!(
                write.message(),
                format!(
                    "{} was written again. It now has the package’s name and ID in the current \
                     format.",
                    write.path()
                )
            );
            assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
            write.undo().unwrap();
            assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
            // Two README names leave no safe manifest destination.
            let other = if filename == "Package.md" {
                "Module.md"
            } else {
                "Package.md"
            };
            fs::write(module.path.join(other), readme).unwrap();
            assert!(crate::content_workspace::ManifestWrite::write(&module).is_err());
            assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
            assert_eq!(
                fs::read(module.path.join(other)).unwrap(),
                readme.as_bytes()
            );
        }
    }

    /// AD1: on reload, every Undo result checks that its files still hold what its action wrote.
    /// Each caller is given one outside change; the check never writes a file.

    #[test]
    fn iter10_e_stale_manifest_write_result_clears_on_reload() {
        use crate::content_workspace::ManifestWrite;
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let path = module.path.join("Package.md");
        fs::remove_file(&path).unwrap();
        let mut write = Some(ManifestWrite::write(&module).unwrap());
        // The reload just after the write keeps the result and its Undo.
        ManifestWrite::forget_if_changed(&mut write);
        assert!(write.as_ref().is_some_and(ManifestWrite::on_disk));
        // The written file becomes a README: the result and its Undo go.
        let readme = "Notes on this adventure.\n";
        fs::write(&path, readme).unwrap();
        ManifestWrite::forget_if_changed(&mut write);
        assert!(write.is_none());
        assert_eq!(fs::read(&path).unwrap(), readme.as_bytes());
        // A deleted file also ends the result.
        fs::remove_file(&path).unwrap();
        let mut write = Some(ManifestWrite::write(&module).unwrap());
        fs::remove_file(&path).unwrap();
        ManifestWrite::forget_if_changed(&mut write);
        assert!(write.is_none());
        // A replacement whose file changed back to the broken text also ends.
        let broken = "---\nsplat_kind: [\n---\n";
        fs::write(&path, broken).unwrap();
        let mut write = Some(ManifestWrite::write(&module).unwrap());
        assert!(write.as_ref().unwrap().replaced());
        fs::write(&path, broken).unwrap();
        ManifestWrite::forget_if_changed(&mut write);
        assert!(write.is_none());
        assert_eq!(fs::read(&path).unwrap(), broken.as_bytes());
    }

    #[test]
    fn iter8_e_manifest_repair_message_names_actual_file() {
        for filename in ["Package.md", "Module.md"] {
            let temp = tempfile::tempdir().unwrap();
            let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
            if filename == "Module.md" {
                fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
            }
            fs::write(module.path.join(filename), "---\nsplat_kind: [\n---\n").unwrap();
            let write = crate::content_workspace::ManifestWrite::write(&module).unwrap();
            assert_eq!(write.path(), filename);
            assert!(write.replaced());
            // Round 24 FF5: the result and its Undo (`Undo write <file>`) share one verb.
            // Round 27 FC5: a replaced file's result says that Undo puts the old text back.
            assert_eq!(
                write.message(),
                format!(
                    "{filename} was written again. It now has the package’s name and ID in the \
                     current format. Undo puts your old text back."
                )
            );
            write.undo().unwrap();
        }
    }

    #[test]
    fn iter8_e_new_package_manifest_is_package_md() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("new"), "New", "new").unwrap();
        assert!(module.path.join("Package.md").is_file());
        assert!(!module.path.join("Module.md").exists());
        let loaded = load(Some(&module)).unwrap();
        let manifest = loaded.notes.iter().find(|note| note.is_manifest()).unwrap();
        assert_eq!(manifest.path, "Package.md");
        assert_eq!(manifest.entry.as_ref().unwrap().id, module.id);
    }

    #[test]
    fn create_register_and_edit_preserves_identity_and_external_changes() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("module");
        let m = create_module(&root, "My Homebrew", "my_homebrew").unwrap();
        assert_eq!(local_module(&root).unwrap().id, m.id);
        assert!(create_module(&root, "Overwrite", "other").is_err());
        let text = note_text("Goblin", "creature", None);
        let entry = write_note(&root, "Goblin.md", None, &text).unwrap();
        assert!(write_note(&root, "Goblin.md", None, &text).is_err());
        let edited = format!("{text}New description\n");
        assert_eq!(
            write_note(&root, "Goblin.md", Some(&text), &edited)
                .unwrap()
                .id,
            entry.id
        );
        fs::write(root.join("Goblin.md"), "external edit").unwrap();
        assert!(write_note(&root, "Goblin.md", Some(&edited), &text).is_err());
        assert_eq!(
            fs::read_to_string(root.join("Goblin.md")).unwrap(),
            "external edit"
        );
        assert!(write_note(&root, "../escape.md", None, &text).is_err());
    }
    #[test]
    fn undo_discard_is_offered_only_to_the_note_it_came_from() {
        let temp = tempfile::tempdir().unwrap();
        let first = create_module(&temp.path().join("first"), "First", "first").unwrap();
        let second = create_module(&temp.path().join("second"), "Second", "second").unwrap();
        let note = |module: &Module, path: &str| Note {
            path: path.into(),
            text: crate::design::memo::Revised::default(),
            entry: None,
            error: None,
            fault: None,
            manifest_gone: false,
            owner: Owner::Package(module.clone()),
        };
        let discarded = (
            OwnerKey::Package(first.id),
            "a.md".to_string(),
            "words".to_string(),
        );
        assert!(discard_is_for(Some(&discarded), &note(&first, "a.md")));
        assert!(!discard_is_for(Some(&discarded), &note(&first, "b.md")));
        assert!(!discard_is_for(Some(&discarded), &note(&second, "a.md")));
        // M-L0.4a: a campaign with the package's UUID (settings check the two apart) holds a
        // different file at the same path: the package's Undo is not offered there.
        let campaign = Note {
            owner: Owner::Campaign(Campaign {
                id: first.id,
                name: "Campaign".into(),
                path: temp.path().join("campaign"),
                dndbeyond_campaign_id: None,
                dndbeyond_campaign_name: None,
            }),
            ..note(&first, "a.md")
        };
        assert!(!discard_is_for(Some(&discarded), &campaign));
    }

    /// M-L0.4a, proof 4: three files that differ only by their owner. A is a package's
    /// `Rules/X.md`; B1 a campaign's with A's UUID (settings check package and campaign UUIDs
    /// apart); B2 another package's. Each holds `text`.
    fn same_path_notes(text: &str, shared: Uuid, root: &Path) -> Vec<Note> {
        let module = |id: Uuid, name: &str| Module {
            id,
            name: name.into(),
            path: root.join(name),
        };
        let note = |owner: Owner| Note {
            path: "Rules/X.md".into(),
            entry: Entry::parse("Rules/X.md", text).ok(),
            text: crate::design::memo::Revised::new(text.to_owned()),
            error: None,
            fault: None,
            manifest_gone: false,
            owner,
        };
        vec![
            note(Owner::Package(module(shared, "a"))),
            note(Owner::Campaign(Campaign {
                id: shared,
                name: "B1".into(),
                path: root.join("b1"),
                dndbeyond_campaign_id: None,
                dndbeyond_campaign_name: None,
            })),
            note(Owner::Package(module(Uuid::new_v4(), "b2"))),
        ]
    }

    /// M-L0.4a, proof 4: a creature whose first heading is not its name, and whose Hit Points
    /// line is not its stat block's, for the two real fixes; and its file A.
    fn fix_fixture() -> (String, Entry, FileId, Uuid) {
        let text = format!(
            "{}\n**Hit Points** 45\n",
            note_text("Guard", "creature", None)
                .replace("| Hit Points |  |", "| Hit Points | 1 |")
                .replacen("# Guard", "# Old Guard", 1)
        );
        let entry = Entry::parse("Rules/X.md", &text).unwrap();
        let shared = Uuid::new_v4();
        (
            text,
            entry,
            (OwnerKey::Package(shared), "Rules/X.md".into()),
            shared,
        )
    }

    /// The entry at `at` opens; its file, whole.
    fn open_file(library: &mut Library, at: usize) -> FileId {
        library.open_entry(at);
        assert_eq!(library.selected, Some(at), "fixture: entry {at} opens");
        let note = &library.loaded.as_ref().unwrap().notes[at];
        (note.owner.key(), note.path.clone())
    }

    /// The three same-path files of `same_path_notes`, each holding `text`, with `kept`.
    fn same_path_library(text: &str, shared: Uuid, root: &Path, kept: LibraryKept) -> Library {
        Library {
            loaded: Some(Loaded::fixture(same_path_notes(text, shared, root))),
            kept,
            ..Default::default()
        }
    }

    /// The B files of proof 4: B1 differs from A by its owner's kind only, B2 by its owner's id.
    const OTHER_FILES: [(usize, &str); 2] =
        [(1, "same UUID, other kind"), (2, "same path, other owner")];

    #[test]
    fn a_heading_fix_undo_belongs_to_its_whole_file() {
        // M-L0.4a, proof 4: the heading fix's Undo shows and acts only on its own file (owner
        // kind, owner id, path). B gets A's exact fixed text, so one owner check stands between B
        // and A's Undo: B1 the kind, B2 the owner id. Main's lifecycle stays: a selection does not
        // end it, and it comes back on A.
        let temp = tempfile::tempdir().unwrap();
        let (text, entry, a, shared) = fix_fixture();
        let mut kept = LibraryKept::default();
        let mut after = crate::design::text_draft::TextDraft::new(text.clone());
        assert!(
            HeadingChoice::new(
                Some(&entry),
                &crate::design::text_draft::TextDraft::new(text.clone()),
                None,
                &a,
                &crate::design::text_draft::KeptCompare::default()
            )
            .apply((true, false), &a, &mut after, &mut kept)
        );
        let mut library = same_path_library(&after, shared, temp.path(), kept);
        let after_entry = Entry::parse("Rules/X.md", &after).unwrap();
        let shows = |library: &Library, file: &FileId| {
            HeadingChoice::new(
                Some(&after_entry),
                &library.draft,
                library.kept.heading(),
                file,
                &crate::design::text_draft::KeptCompare::default(),
            )
            .undo
            .is_some()
        };
        let file = open_file(&mut library, 0);
        assert!(
            shows(&library, &file) && library.heading_undo_applies(&file),
            "A's heading Undo shows and acts on A"
        );
        for (at, case) in OTHER_FILES {
            let file = open_file(&mut library, at);
            assert!(!shows(&library, &file), "heading show: {case}");
        }
        for (at, case) in OTHER_FILES {
            let file = open_file(&mut library, at);
            assert!(
                !library.heading_undo_applies(&file),
                "heading press: {case}"
            );
        }
        let file = open_file(&mut library, 0);
        assert!(
            shows(&library, &file) && library.heading_undo_applies(&file),
            "heading A → B → A: the Undo is back on A"
        );
    }

    #[test]
    fn a_stat_fix_undo_belongs_to_its_whole_file() {
        // M-L0.4a, proof 4: as for the heading fix, for the stat fix (its own slot).
        let temp = tempfile::tempdir().unwrap();
        let (text, entry, a, shared) = fix_fixture();
        let (key, label, value, _) = creature::vital_checks(&entry)[0].clone();
        let mut fixed = Library {
            draft: (text).into(),
            ..Default::default()
        };
        Library::fix_stat(
            (
                &mut fixed.kept,
                &mut fixed.draft,
                &mut fixed.draft_revision,
                &mut fixed.error,
            ),
            a,
            &entry,
            (key, label, value),
        );
        let kept = std::mem::take(&mut fixed.kept);
        let mut library = same_path_library(&fixed.draft, shared, temp.path(), kept);
        let file = open_file(&mut library, 0);
        assert!(
            library.stat_result_for(&file).is_some() && library.stat_undo_applies(&file),
            "A's stat Undo shows and acts on A"
        );
        for (at, case) in OTHER_FILES {
            let file = open_file(&mut library, at);
            assert!(
                library.stat_result_for(&file).is_none(),
                "stat show: {case}"
            );
        }
        for (at, case) in OTHER_FILES {
            let file = open_file(&mut library, at);
            assert!(!library.stat_undo_applies(&file), "stat press: {case}");
        }
        let file = open_file(&mut library, 0);
        assert!(
            library.stat_result_for(&file).is_some() && library.stat_undo_applies(&file),
            "stat A → B → A: the Undo is back on A"
        );
    }

    #[test]
    fn a_heading_fix_and_a_stat_fix_keep_their_own_slots() {
        // M-L0.4a, proof 4: a stat fix, then a heading fix. With the draft back at each fix's
        // text, that fix's Undo applies: one kind's fix never ends the other's.
        let (text, entry, a, _) = fix_fixture();
        let (key, label, value, _) = creature::vital_checks(&entry)[0].clone();
        let mut library = Library {
            draft: (text).into(),
            ..Default::default()
        };
        Library::fix_stat(
            (
                &mut library.kept,
                &mut library.draft,
                &mut library.draft_revision,
                &mut library.error,
            ),
            a.clone(),
            &entry,
            (key, label, value),
        );
        let stat_after = library.draft.as_str().to_owned();
        let stat_entry = Entry::parse("Rules/X.md", &stat_after).unwrap();
        let mut heading_after = crate::design::text_draft::TextDraft::new(stat_after.clone());
        HeadingChoice::new(
            Some(&stat_entry),
            &crate::design::text_draft::TextDraft::new(stat_after.clone()),
            None,
            &a,
            &crate::design::text_draft::KeptCompare::default(),
        )
        .apply((true, false), &a, &mut heading_after, &mut library.kept);
        assert_ne!(
            stat_after,
            heading_after.as_str(),
            "fixture: the heading fix changed it"
        );
        let _ = library.draft.replace(stat_after);
        assert!(
            library.stat_undo_applies(&a),
            "a heading fix keeps the stat Undo"
        );
        let _ = library.draft.replace(heading_after.into_string());
        assert!(
            library.heading_undo_applies(&a),
            "the heading Undo stands beside the stat Undo"
        );
    }

    #[test]
    fn every_kept_result_comes_with_its_mark_from_its_real_writer() {
        // M-L0.4a, proof 2: each REAL writer (the method the UI calls) stores its result's data
        // and its mark in one call (`LibraryKept::set_*`, `push_copy`): empty → present, the
        // newest first; a second act of the kind replaces the first (a newer order, still one
        // mark of the kind). A copy is one result per copy: the second is the newest and the
        // first stays.
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        for name in ["Ash", "Brand", "Cinder", "Dusk"] {
            fs::write(
                m.path.join(format!("{name}.md")),
                note_text(name, "rule", None),
            )
            .unwrap();
        }
        // The copies' fixture: two encounters of the package, each linking to a creature of
        // another package.
        let source = create_module(&temp.path().join("source"), "Source", "source").unwrap();
        for (creature, encounter) in [("Wolf", "Ambush"), ("Boar", "Raid")] {
            let text = note_text(creature, "creature", None);
            let id = Entry::parse("x.md", &text).unwrap().id;
            fs::write(source.path.join(format!("{creature}.md")), text).unwrap();
            let path = format!("{encounter}.md");
            let fields = format!(
                "splat_id: {}\nsplat_kind: encounter\nsplat_name: {encounter}\nsplat_encounter:\n  roster:\n  - key: foe\n    creature: {{id: {id}}}\n    side: enemies\n    controller: {{kind: manual, role: gm}}\n  sides:\n    party: {{hostile_to: [enemies]}}\n    enemies: {{hostile_to: [party]}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 20, resolution_events: 10000}}\n",
                Uuid::new_v4()
            );
            let text =
                content::formats::fixture(&path, &fields, &format!("\n# {encounter}\n")).unwrap();
            fs::write(m.path.join(path), text).unwrap();
        }
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        let unregistered: Vec<Module> = ["U1", "U2"]
            .into_iter()
            .map(|name| create_module(&temp.path().join(name), name, &name.to_lowercase()).unwrap())
            .collect();
        for module in [&m, &source].into_iter().chain(&unregistered) {
            campaigns.register_module(module.clone()).unwrap();
        }
        let campaign = sd1_campaign(temp.path());
        campaigns.open_for_test(&campaign);
        let mut library = Library {
            loaded: Some(load_all(&[m.clone(), source.clone()], None).unwrap()),
            ..Default::default()
        };
        let at = |library: &Library, name: &str| {
            let path = format!("{name}.md");
            let notes = &library.loaded.as_ref().unwrap().notes;
            notes.iter().position(|note| note.path == path).unwrap()
        };
        let newest = |library: &Library| library.kept.kinds_newest_first().first().copied();
        // Both acts of one kind: the second replaces the first, with a newer order.
        let replaced = |library: &Library, kind: KeptKind, before: Option<design::ResultOrder>| {
            let marks = library.kept.kinds_newest_first();
            assert_eq!(marks.first(), Some(&kind), "{kind:?}: the newest");
            assert_eq!(
                marks.iter().filter(|at| **at == kind).count(),
                1,
                "{kind:?}: one"
            );
            assert!(
                library.kept.order_of(kind) > before,
                "{kind:?}: a newer order"
            );
        };

        // ScopeNotice, by `open_entry` of an entry outside the scope.
        for name in ["Ash", "Brand"] {
            let before = library.kept.order_of(KeptKind::ScopeNotice);
            library.scope = Scope::Package(Uuid::new_v4());
            library.open_entry(at(&library, name));
            replaced(&library, KeptKind::ScopeNotice, before);
        }
        // M-L0.4b: the notice stands through the list's next frames, where the footer draws it
        // (main's list ended it in the first frame, as the scope it had listed was not the
        // notice's: no footer ever drew it). The DM's own search, or scope, ends it.
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let painted = |library: &mut Library, campaigns: &mut Campaigns| {
            let mut texts = Vec::new();
            for _ in 0..2 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1200.0, 800.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| library.show(ui, campaigns));
                    },
                );
                texts = output
                    .shapes
                    .into_iter()
                    .filter_map(|shape| match shape.shape {
                        egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
            }
            texts.join("\n")
        };
        let notice = "Scope is now Everything, to show Brand.";
        let shown = painted(&mut library, &mut campaigns);
        assert!(
            shown.contains(notice) && library.kept.scope_notice().is_some(),
            "the scope notice shows after the list's frames\n{shown}"
        );
        library.search = "Ash".into();
        let searched = painted(&mut library, &mut campaigns);
        assert!(
            !searched.contains(notice) && library.kept.scope_notice().is_none(),
            "a search ends the scope notice\n{searched}"
        );
        library.search.clear();
        library.scope = Scope::Package(Uuid::new_v4());
        library.open_entry(at(&library, "Ash"));
        assert!(
            painted(&mut library, &mut campaigns).contains("Scope is now Everything, to show Ash."),
            "fixture: a new notice shows"
        );
        library.scope = Scope::Builtin;
        let _ = painted(&mut library, &mut campaigns);
        assert!(
            library.kept.scope_notice().is_none(),
            "the DM's scope change ends the scope notice"
        );
        // Astra on 3a2f6d42: the change to an empty Campaign scope (this campaign has no
        // entries of its own) ends it too, before that scope's own footer draws.
        library.scope = Scope::Package(Uuid::new_v4());
        library.open_entry(at(&library, "Brand"));
        assert!(
            painted(&mut library, &mut campaigns)
                .contains("Scope is now Everything, to show Brand."),
            "fixture: a new notice shows"
        );
        library.scope = Scope::Campaign;
        let empty = painted(&mut library, &mut campaigns);
        assert!(
            empty.contains("No entries of your own yet"),
            "fixture: the empty Campaign scope\n{empty}"
        );
        assert!(
            library.kept.scope_notice().is_none() && !empty.contains("Scope is now Everything"),
            "an empty Campaign scope ends the scope notice\n{empty}"
        );
        library.scope = Scope::All;
        // Identity, by `give_new_identity` of the open entry.
        for name in ["Ash", "Brand"] {
            let before = library.kept.order_of(KeptKind::Identity);
            library.selected = Some(at(&library, name));
            library.give_new_identity().unwrap();
            replaced(&library, KeptKind::Identity, before);
        }
        // Discarded, by `reload_note_from_disk` of a changed file under a draft (twice), then by
        // the editor's Discard (twice); Removed, by the reload of a file that has gone under a
        // draft (Ash, then Brand).
        let reload = |library: &mut Library, name: &str, text: Option<&str>| {
            let index = at(library, name);
            library.selected = Some(index);
            let _ = library.draft.replace("An unsaved draft.");
            library.disk_change = Some(DiskChange {
                text: text.map(str::to_owned),
                moved_to: None,
                fault: None,
            });
            library.reload_note_from_disk(index);
        };
        for words in ["A changed file.", "Changed again."] {
            let before = library.kept.order_of(KeptKind::Discarded);
            reload(&mut library, "Ash", Some(words));
            replaced(&library, KeptKind::Discarded, before);
        }
        for name in ["Cinder", "Dusk"] {
            let before = library.kept.order_of(KeptKind::Discarded);
            let index = at(&library, name);
            library.selected = Some(index);
            let _ = library.draft.replace(format!("A draft of {name}."));
            let note = &library.loaded.as_ref().unwrap().notes[index];
            Library::discard_draft(
                (
                    &mut library.kept,
                    &mut library.draft,
                    &mut library.draft_revision,
                    &mut library.error,
                ),
                note,
            );
            replaced(&library, KeptKind::Discarded, before);
            assert_eq!(
                library.draft.as_str(),
                note.text.as_str(),
                "Discard: the file's words"
            );
        }
        for name in ["Ash", "Brand"] {
            let before = library.kept.order_of(KeptKind::Removed);
            reload(&mut library, name, None);
            replaced(&library, KeptKind::Removed, before);
            // The draft it let go is a discard too: both, Remove's the newest.
            assert!(library.kept.discarded().is_some() && library.kept.removed().is_some());
        }
        // Heading and Vital, by their real fixes.
        let text = format!(
            "{}\n**Hit Points** 45\n",
            note_text("Guard", "creature", None)
                .replace("| Hit Points |  |", "| Hit Points | 1 |")
                .replacen("# Guard", "# Old Guard", 1)
        );
        let entry = Entry::parse("Rules/X.md", &text).unwrap();
        let file: FileId = (OwnerKey::Package(m.id), "Rules/X.md".into());
        for _ in 0..2 {
            let before = library.kept.order_of(KeptKind::Heading);
            let mut draft = crate::design::text_draft::TextDraft::new(text.clone());
            HeadingChoice::new(
                Some(&entry),
                &crate::design::text_draft::TextDraft::new(text.clone()),
                None,
                &file,
                &crate::design::text_draft::KeptCompare::default(),
            )
            .apply((true, false), &file, &mut draft, &mut library.kept);
            replaced(&library, KeptKind::Heading, before);
        }
        let (key, label, value, _) = creature::vital_checks(&entry)[0].clone();
        for _ in 0..2 {
            let before = library.kept.order_of(KeptKind::Vital);
            let _ = library.draft.replace(text.clone());
            Library::fix_stat(
                (
                    &mut library.kept,
                    &mut library.draft,
                    &mut library.draft_revision,
                    &mut library.error,
                ),
                file.clone(),
                &entry,
                (key, label, value),
            );
            replaced(&library, KeptKind::Vital, before);
        }
        // Unregistered, by `unregister` (from its dialog).
        let mut tickets = Vec::new();
        for module in &unregistered {
            let before = library.kept.order_of(KeptKind::Unregistered);
            library.unregister(module.clone(), &mut campaigns);
            replaced(&library, KeptKind::Unregistered, before);
            tickets.push(library.kept.ticket_of(KeptKind::Unregistered).unwrap());
        }
        // M-L0.4b: a press acts by its line's ticket. The second unregister replaced the first:
        // the first line's ticket reaches nothing, and the newer one's Undo works.
        let registered = |campaigns: &Campaigns, module: &Module| {
            campaigns
                .modules()
                .iter()
                .any(|known| known.id == module.id)
        };
        let undo = |ticket| Press {
            id: LibraryPress::Undo,
            ticket: Some(ticket),
        };
        let mut tickets = tickets.into_iter();
        let (stale, newer) = (tickets.next().unwrap(), tickets.next().unwrap());
        library.apply_footer_press(undo(stale), &mut campaigns);
        assert!(
            library.kept.unregistered() == Some(&unregistered[1])
                && !registered(&campaigns, &unregistered[0])
                && !registered(&campaigns, &unregistered[1]),
            "a replaced result's ticket reaches nothing"
        );
        library.apply_footer_press(undo(newer), &mut campaigns);
        assert!(
            library.kept.unregistered().is_none() && registered(&campaigns, &unregistered[1]),
            "the newer line's Undo registers U2 again: {:?}",
            library.error
        );
        // Manifest, by the repair's `write_manifest` of a deleted manifest (twice).
        for _ in 0..2 {
            let before = library.kept.order_of(KeptKind::Manifest);
            fs::remove_file(m.path.join("Package.md")).unwrap();
            library.write_manifest(m.clone(), "Package.md", false);
            assert_eq!(library.error, None, "fixture: the manifest was written");
            replaced(&library, KeptKind::Manifest, before);
        }
        // Identities, by a folder's `separate_entries`.
        for name in ["Cinder", "Dusk"] {
            let before = library.kept.order_of(KeptKind::Identities);
            library
                .separate_entries(m.path.clone(), &[format!("{name}.md")])
                .unwrap();
            replaced(&library, KeptKind::Identities, before);
        }
        // PackageUse, by `Choose all` (`choose_all`) for the open campaign.
        for _ in 0..2 {
            let before = library.kept.order_of(KeptKind::PackageUse);
            library.choose_all(&m, &campaigns);
            replaced(&library, KeptKind::PackageUse, before);
        }
        // Copy, by `copy_dependency`: one result per copy, the newest first, the first kept.
        let copies = |library: &Library| {
            library
                .kept
                .kinds_newest_first()
                .into_iter()
                .filter(|kind| *kind == KeptKind::Copy)
                .count()
        };
        for (encounter, creature) in [("Ambush", "Wolf"), ("Raid", "Boar")] {
            let shown = copies(&library);
            let loaded = library.loaded.as_ref().unwrap();
            let find = |name: &str| {
                loaded
                    .notes
                    .iter()
                    .find(|note| note.path == format!("{name}.md"))
                    .unwrap()
            };
            let root = Library::copy_dependency(
                &mut library.kept,
                find(encounter),
                find(creature),
                &campaigns,
            )
            .unwrap();
            assert_eq!(root.as_deref(), Some(m.path.as_path()), "fixture: copied");
            assert_eq!(newest(&library), Some(KeptKind::Copy), "Copy: the newest");
            assert_eq!(
                (copies(&library), library.kept.copies().len()),
                (shown + 1, shown + 1),
                "Copy: one mark per copy, the first kept"
            );
        }
        // M-L0.4b: a copy's Undo finds its own copy by its line's ticket, whichever is newer.
        for (at, copy) in library.kept.copies().iter().enumerate() {
            let ticket = library.kept.copy_ticket_for_test(copy).unwrap();
            assert_eq!(
                library.kept.copy_at(&ticket),
                Some(at),
                "each copy's own ticket"
            );
        }
        // The boundary's static rung (Astra, PR 477): no method of `LibraryKept` hands out a
        // mutable payload, so no caller can swap one result's data under another's mark.
        let leaks: Vec<&str> = include_str!("library/kept.rs")
            .lines()
            .filter(|line| {
                line.split_once("-> ")
                    .is_some_and(|(_, returns)| returns.contains("&mut"))
            })
            .collect();
        assert!(
            leaks.is_empty(),
            "kept.rs returns a mutable payload: {leaks:?}"
        );
    }

    #[test]
    fn each_folder_repair_names_the_encounters_of_its_own_owner() {
        // Astra's case on PR 477: a package and a campaign with one UUID both hold the
        // duplicate creature and the duplicate encounter, the encounter under another name in
        // each. Each folder's batch choice names the encounter outside its own batch: the other
        // owner's. Its cache is keyed by the whole owner key, so the second choice never reuses
        // the first one's names.
        let root = tempfile::tempdir().unwrap();
        let shared = Uuid::new_v4();
        let creature = note_text("Cinder Wolf", "creature", None);
        let wolf = Entry::parse("Creatures/Cinder Wolf.md", &creature)
            .unwrap()
            .id;
        let fight = Uuid::new_v4();
        let encounter = |name: &str| {
            content::formats::fixture(
                "Encounters/Bridge.md",
                &format!(
                    "splat_id: {fight}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter:\n  roster:\n  - key: wolf\n    creature: {{id: {wolf}}}\n    side: enemies\n    controller: {{kind: manual, role: gm}}\n  sides:\n    party: {{hostile_to: [enemies]}}\n    enemies: {{hostile_to: [party]}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 20, resolution_events: 10000}}\n"
                ),
                &format!("\n# {name}\n"),
            )
            .unwrap()
        };
        let package = Owner::Package(Module {
            id: shared,
            name: "Ashen Marches".into(),
            path: root.path().join("ashen-marches"),
        });
        let campaign = Owner::Campaign(Campaign {
            id: shared,
            name: "Ember".into(),
            path: root.path().join("ember"),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        });
        let note = |path: &str, text: String, owner: &Owner| Note {
            path: path.into(),
            entry: Entry::parse(path, &text).ok(),
            text: crate::design::memo::Revised::new(text),
            error: None,
            fault: None,
            manifest_gone: false,
            owner: owner.clone(),
        };
        let loaded = Loaded::fixture(vec![
            note("Creatures/Cinder Wolf.md", creature.clone(), &package),
            note("Encounters/Bridge.md", encounter("Bridge Ambush"), &package),
            note("Creatures/Cinder Wolf.md", creature, &campaign),
            note("Encounters/Bridge.md", encounter("Bridge Fight"), &campaign),
        ]);
        assert!(
            loaded.notes.iter().all(|note| note.entry.is_some())
                && loaded.duplicates(0) == [2]
                && loaded.duplicates(1) == [3],
            "fixture: the creature and the encounter are each in both owners"
        );
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let mut painted = Vec::new();
        for _ in 0..2 {
            let output = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The warning in a typed footer, as the reader draws it (M-L0.4b).
                    let frame = PaneFrame::new(ui);
                    let mut messages = Messages::<LibraryPress>::new(&frame, "duplicates", 600.0);
                    messages.outcome(duplicate_notice(
                        ui,
                        &loaded,
                        &loaded.notes[0],
                        &[0, 2],
                        None,
                    ));
                    let _ = messages.pane(ui, design::Body::Content, |_| ());
                });
            });
            painted = output
                .shapes
                .into_iter()
                .filter_map(|shape| match shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();
        }
        // Issue #505 (M-L0.5): the campaign with the package's UUID is never the package: not in
        // its scope, not in what unregistering it takes away.
        assert!(
            loaded.in_scope(&loaded.notes[0], Scope::Package(shared))
                && !loaded.in_scope(&loaded.notes[2], Scope::Package(shared)),
            "the package's scope holds its own notes only"
        );
        assert_eq!(
            unregister_count(&loaded.notes, shared),
            2,
            "unregistering the package counts its own two entries"
        );
        // Every package check takes the owner's kind: the page's code compares no owner's
        // UUID alone (`Owner::is_package`).
        let check = ["owner.id() ", "== Some("].concat();
        for source in [
            include_str!("library.rs"),
            include_str!("library/catalogue.rs"),
        ] {
            let code = source.split("\nmod tests {").next().unwrap();
            assert!(!code.contains(&check), "a package check by UUID alone");
        }
        for words in [
            // The package's batch leaves the campaign's encounter on the campaign's files.
            "Bridge Fight keeps the files in ember/.",
            // The campaign's batch moves the package's encounter onto the package's files.
            "Bridge Ambush will then use the files in ashen-marches/.",
        ] {
            assert!(
                painted.iter().any(|text| text == words),
                "each batch names its own owner's outside encounter: {words}\n{painted:#?}"
            );
        }
    }

    #[test]
    fn the_empty_readers_undos_name_their_object_whole_or_short() {
        use crate::design::test_harness::Themed as _;
        // M-L0.4b, the three proofs of the M9 labels (`Undo unregister <package>`, `Undo remove
        // <name>`) in the empty reader's typed footer:
        // 8. At every width each Undo shows its whole label where it fits the Undo's own room
        //    (the line, capped at `LIST_MEASURE`, less the message's inset and the action's icon
        //    and padding), else its short form: never cut, never past the pane.
        // 9. Its accessible name is the whole label at every width, also where it shows short.
        // 10. An unchanged frame builds and measures no label, and makes no plan.
        use egui_kittest::kittest::{NodeT, Queryable};
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        // Long names, as the DM's data may hold, so the widths cross each label's boundary.
        let entry = "Ash Exposure in the Burning Wastes";
        let package = "The Ashen Marches of the Cinder Reach";
        let creature = note_text(entry, "creature", None);
        let id = Entry::parse("x.md", &creature).unwrap().id;
        fs::write(m.path.join(format!("{entry}.md")), creature).unwrap();
        // Four encounters need it, so the Remove line names the first three "and 1 more".
        for at in 1..=4 {
            let name = format!("Bridge {at}");
            let path = format!("{name}.md");
            let fields = format!(
                "splat_id: {}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter:\n  roster:\n  - key: foe\n    creature: {{id: {id}}}\n    side: enemies\n    controller: {{kind: manual, role: gm}}\n  sides:\n    party: {{hostile_to: [enemies]}}\n    enemies: {{hostile_to: [party]}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 20, resolution_events: 10000}}\n",
                Uuid::new_v4()
            );
            let text = content::formats::fixture(&path, &fields, &format!("\n# {name}\n")).unwrap();
            fs::write(m.path.join(path), text).unwrap();
        }
        let mut library = Library {
            loaded: Some(load(Some(&m)).unwrap()),
            ..Default::default()
        };
        // The real Remove: a draft over a file that has gone.
        let index = library
            .loaded
            .as_ref()
            .unwrap()
            .notes
            .iter()
            .position(|note| note.path == format!("{entry}.md"))
            .unwrap();
        library.selected = Some(index);
        let _ = library.draft.replace("An unsaved draft.");
        library.disk_change = Some(DiskChange {
            text: None,
            moved_to: None,
            fault: None,
        });
        library.reload_note_from_disk(index);
        library.kept.set_unregistered(Module {
            id: Uuid::new_v4(),
            name: package.into(),
            path: temp.path().join("modules/ashen-marches"),
        });
        assert!(library.selected.is_none(), "fixture: no entry open");
        let mut harness = egui_kittest::Harness::builder().themed_ui_state(
            |ui, library: &mut Library| library.editor(ui, "Entry", &Campaigns::default()),
            library,
        );
        let (unregister, remove) = (
            format!("Undo unregister {package}"),
            format!("Undo remove {entry}"),
        );
        let labels = [
            ("Undo unregister", unregister.as_str()),
            ("Undo remove", remove.as_str()),
        ];
        let mut forms = std::collections::BTreeSet::new();
        for width in (260..=900).step_by(4) {
            let width = width as f32;
            harness.set_size(egui::vec2(width, 700.0));
            harness.run_steps(2);
            // Each text with its rect, the rect it is clipped to, and whether its layout cut it
            // (`elided`: egui's `…` is in the galley, not in its words): a text past its clip or
            // elided is cut.
            let painted: Vec<(String, egui::Rect, egui::Rect, bool)> = harness
                .output()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some((
                        text.galley.job.text.clone(),
                        text.galley.rect.translate(text.pos.to_vec2()),
                        shape.clip_rect,
                        text.galley.elided,
                    )),
                    _ => None,
                })
                .collect();
            for (short, whole) in labels {
                let named = harness
                    .query_all(egui_kittest::kittest::By::new().predicate(|node| {
                        node.role() == egui::accesskit::Role::Button
                            && node.label().is_some_and(|label| label.starts_with(short))
                    }))
                    .map(|node| node.accesskit_node().label().unwrap_or_default())
                    .collect::<Vec<_>>();
                assert_eq!(named, [whole], "9: the whole accessible name at {width}");
                let shown: Vec<_> = painted
                    .iter()
                    .filter(|(text, ..)| text.starts_with(short))
                    .collect();
                assert!(
                    shown.len() == 1
                        && (shown[0].0 == whole || shown[0].0 == short)
                        && shown[0].1.right() <= shown[0].2.right().min(width) + 0.5
                        && !shown[0].3,
                    "8: whole where it fits, else short, never cut, at {width}: {shown:?}"
                );
                forms.insert((short, shown[0].0 == whole));
            }
            assert!(
                !painted
                    .iter()
                    .any(|(text, _, _, elided)| *elided || text.contains('…')),
                "8: nothing cut at {width}"
            );
        }
        for (short, _) in labels {
            assert!(
                forms.contains(&(short, true)) && forms.contains(&(short, false)),
                "fixture: the widths reach the whole and the short form of {short}: {forms:?}"
            );
        }
        // 10. Quiet frames at one width: no footer plan, and no content-sized work for the
        // Remove line's words (the entries that need the removed one, found once per content).
        harness.set_size(egui::vec2(700.0, 700.0));
        harness.run_steps(2);
        let scans = || DEPENDENT_SCANS.with(std::cell::Cell::get);
        let copied = || DEPENDENT_NAMES.with(std::cell::Cell::get);
        assert!(
            scans() > 0 && copied() >= 4,
            "fixture: the Remove line found the four encounters that need its entry"
        );
        let painted: Vec<String> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                _ => None,
            })
            .collect();
        assert!(
            painted
                .iter()
                .any(|text| text.contains("Bridge 1, Bridge 2, Bridge 3 and 1 more now needs")),
            "fixture: the Remove line names them\n{painted:#?}"
        );
        let (quiet, scanned, names) = (crate::design::outcome::footer_work(), scans(), copied());
        harness.run_steps(4);
        let after = crate::design::outcome::footer_work();
        assert_eq!(
            scans(),
            scanned,
            "10: an unchanged frame reads no note for the Remove line"
        );
        assert_eq!(
            copied(),
            names,
            "10: an unchanged frame copies none of the names that need the removed entry"
        );
        assert!(
            after.compared > quiet.compared,
            "the frames drew the footer"
        );
        assert_eq!(
            (
                after.plans - quiet.plans,
                after.sorted - quiet.sorted,
                after.named - quiet.named,
                after.undo_fits - quiet.undo_fits,
                after.laid - quiet.laid,
            ),
            (0, 0, 0, 0, 0),
            "10: an unchanged frame of the empty reader's footer"
        );
    }

    #[test]
    fn back_keeps_a_campaign_file_apart_from_a_package_file() {
        // M-L0.4a, proof 4: a package file and a campaign file with one UUID and one path (the
        // package's loads first), and another file. Back finds the campaign's file, never the
        // package's.
        let temp = tempfile::tempdir().unwrap();
        let shared = Uuid::new_v4();
        let text = note_text("Watch", "rule", None);
        let notes = || {
            let mut notes = same_path_notes(&text, shared, temp.path());
            notes[2].path = "Other.md".into();
            notes
        };
        let mut library = Library {
            loaded: Some(Loaded::fixture(notes())),
            ..Default::default()
        };
        library.open_entry(1);
        library.open_entry(2);
        library.go(Step::Back);
        assert_eq!(library.selected, Some(1), "Back opens the campaign's file");
    }

    #[test]
    fn a_reload_reselects_the_campaign_file_not_the_package_file() {
        // M-L0.4a, proof 4: as above, through the real reload: `start_load` keeps the open
        // file's identity, and `show` takes the new notes in.
        let temp = tempfile::tempdir().unwrap();
        let shared = Uuid::new_v4();
        let text = note_text("Watch", "rule", None);
        let notes = || {
            let mut notes = same_path_notes(&text, shared, temp.path());
            notes[2].path = "Other.md".into();
            notes
        };
        let mut library = Library {
            loaded: Some(Loaded::fixture(notes())),
            ..Default::default()
        };
        library.open_entry(1);
        library.reselect = library.open_file_id();
        library.loaded = None;
        library.selected = None;
        let (send, receive) = mpsc::channel();
        send.send(Ok(Loaded::fixture(notes()))).unwrap();
        library.pending = Some(receive);
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
        });
        assert_eq!(
            library.selected,
            Some(1),
            "the reload reselects the campaign's file"
        );
        // Issue #539: the list finds the selected entry's row once; quiet frames search the
        // matches no more.
        let mut frame = || {
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
            });
        };
        frame();
        frame();
        let searched = ROW_SEARCHES.with(std::cell::Cell::get);
        frame();
        frame();
        frame();
        assert!(
            searched > 0 && ROW_SEARCHES.with(std::cell::Cell::get) == searched,
            "#539: the selected row was found ({searched} searches) and then kept"
        );
    }

    #[test]
    fn library_frame_keeps_campaign_selection_and_draft_independent() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.register_module(m.clone()).unwrap();
        let mut library = Library {
            loaded: Some(load(Some(&m)).unwrap()),
            selected: Some(0),
            ..Default::default()
        };
        let _ = library
            .draft
            .replace(library.loaded.as_ref().unwrap().notes[0].text.as_str());
        library.draft.push_str("Unsaved description");
        let expected = library.draft.as_str().to_owned();
        library.search = "no matching entry".into();
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
        });
        assert!(campaigns.active_campaign().is_none());
        assert_eq!(library.draft.as_str(), expected);
        campaigns.unregister_module(m.id).unwrap();
        assert!(m.path.join("Package.md").exists());
    }

    #[test]
    fn loading_keeps_invalid_notes_available_for_repair() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        fs::write(
            m.path.join("Broken.md"),
            "---\nsplat_schema: 999\n---\nBroken",
        )
        .unwrap();
        let loaded = load(Some(&m)).unwrap();
        assert_eq!(loaded.notes.len(), 2);
        assert!(loaded.notes.iter().any(|n| n.error.is_some()));
        let loaded = load_all(std::slice::from_ref(&m), None).unwrap();
        assert!(
            loaded
                .diagnostic_texts()
                .iter()
                .any(|d| d.starts_with("Broken.md"))
        );
    }

    #[test]
    fn package_manifest_failure_keeps_library_entries_and_a_named_warning() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::write(
            module.path.join("Wolf.md"),
            note_text("Wolf", "creature", None),
        )
        .unwrap();
        fs::write(module.path.join("Package.md"), "---\nsplat_id: [\n---\n").unwrap();
        for deleted in [false, true] {
            if deleted {
                fs::remove_file(module.path.join("Package.md")).unwrap();
            }
            let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
            assert!(loaded.notes.iter().any(|note| note.name() == "Wolf"));
            assert!(
                loaded
                    .diagnostic_texts()
                    .iter()
                    .any(|message| message.contains("Package.md"))
            );
        }
    }

    #[test]
    fn iter3_d_a_readable_entry_named_like_a_manifest_stays_an_entry() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::rename(
            module.path.join("Package.md"),
            module.path.join("Module.md"),
        )
        .unwrap();
        fs::write(
            module.path.join("Package.md"),
            note_text("Package Wolf", "creature", None),
        )
        .unwrap();
        fs::write(module.path.join("Module.md"), "---\nsplat_id: [\n---\n").unwrap();
        let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
        let wolf = loaded
            .notes
            .iter()
            .find(|note| note.path == "Package.md")
            .unwrap();
        assert!(!wolf.is_manifest());
        assert!(loaded.in_scope(wolf, Scope::Package(module.id)));
        assert_eq!(unregister_count(&loaded.notes, module.id), 1);
    }

    #[test]
    fn iter3_d_manifests_never_enter_content_counts_or_results() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::write(
            module.path.join("Wolf.md"),
            note_text("Wolf", "creature", None),
        )
        .unwrap();
        for text in [
            "---\nsplat_id: [\n---\n",
            "---\nsplat_kind: package\nsplat_id: [\n---\n",
        ] {
            fs::write(module.path.join("Package.md"), text).unwrap();
            let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
            let manifest = loaded.notes.iter().find(|note| note.is_manifest()).unwrap();
            assert!(manifest.error.is_some());
            // Round 24 FC2: a manifest that cannot be read is an error on the About row.
            assert!(
                manifest.about_label(false).starts_with(icons::ERROR)
                    && !manifest.about_label(false).contains(icons::WARN),
                "iter-24-L-FC2: {}",
                manifest.about_label(false)
            );
            assert!(loaded.unreadable_counted, "iter-24-L-FC2-header");
            assert!(
                loaded
                    .diagnostic_texts()
                    .iter()
                    .any(|line| line.contains("Package.md"))
            );
            for scope in [Scope::All, Scope::Uses, Scope::Package(module.id)] {
                assert!(
                    !loaded.in_scope(manifest, scope),
                    "manifest entered {scope:?}"
                );
            }
            assert_eq!(
                loaded
                    .notes
                    .iter()
                    .filter(|note| loaded.in_scope(note, Scope::Package(module.id)))
                    .count(),
                1
            );
            assert_eq!(unregister_count(&loaded.notes, module.id), 1);
            assert_eq!(
                loaded
                    .notes
                    .iter()
                    .filter(|note| loaded.in_scope(note, Scope::Package(module.id))
                        && note.section() == Section::Unreadable)
                    .count(),
                0
            );
        }
    }

    /// Writes a package whose one item needs `needs`.
    fn package_needing(root: &Path, name: &str, namespace: &str, needs: &str) -> Module {
        let module = create_module(&root.join(name), name, namespace).unwrap();
        let note = splat::content::formats::fixture(
            "Fixture.md",
            &format!(
                "splat_id: {}\nsplat_kind: item\nsplat_key: {namespace}:items/own\n\
             splat_name: Own\nsplat_item: {{category: gear}}\nsplat_requires:\n  - key: {needs}\n",
                Uuid::new_v4()
            ),
            "",
        )
        .expect("schema-2 fixture");
        fs::write(module.path.join("Own.md"), note).unwrap();
        module
    }

    #[test]
    fn references_resolve_across_the_whole_library() {
        let temp = tempfile::tempdir().unwrap();
        let modules = [
            // One reference to built-in content, and one to the other package.
            package_needing(temp.path(), "First", "first", "dnd2024:equipment/battleaxe"),
            package_needing(temp.path(), "Second", "second", "first:items/own"),
        ];
        let mut loaded = load_all(&modules, None).unwrap();
        assert_eq!(loaded.diagnostic_texts(), Vec::<&str>::new());
        // A save checks again, and that check must agree with the first.
        loaded.refresh_diagnostics();
        assert_eq!(loaded.diagnostic_texts(), Vec::<&str>::new());
    }

    #[test]
    fn a_reference_to_nothing_is_still_reported() {
        let temp = tempfile::tempdir().unwrap();
        let module = package_needing(temp.path(), "First", "first", "missing:items/gone");
        let loaded = load_all(&[module], None).unwrap();
        assert_eq!(
            loaded.diagnostic_texts().len(),
            1,
            "{:?}",
            loaded.diagnostic_texts()
        );
        assert!(loaded.diagnostic_texts()[0].contains("missing:items/gone"));
    }

    #[test]
    fn a_save_keeps_the_problems_that_kept_content_from_loading() {
        let temp = tempfile::tempdir().unwrap();
        let mut module = package_needing(temp.path(), "First", "first", "first:items/own");
        module.path = temp.path().join("deleted");
        let mut loaded = load_all(&[module], None).unwrap();
        assert_eq!(
            loaded.diagnostic_texts().len(),
            1,
            "{:?}",
            loaded.diagnostic_texts()
        );
        loaded.refresh_diagnostics();
        assert_eq!(
            loaded.diagnostic_texts().len(),
            1,
            "{:?}",
            loaded.diagnostic_texts()
        );
    }
    /// Bytes that are not UTF-8 text, as a file saved in another encoding holds.
    const NOT_TEXT: [u8; 3] = [0xff, 0xfe, b'\n'];

    fn sd1_campaign(root: &Path) -> Campaign {
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path: root.join("campaign"),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        fs::create_dir_all(campaign.path.join("Notes")).unwrap();
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        campaign
    }

    /// The text the reader paints for the selected note, after two frames.
    fn sd1_painted(library: &mut Library) -> String {
        sd1_painted_at(library, (900.0, 1400.0))
    }

    /// The text the reader paints for the selected note, after two frames at `size`.
    fn sd1_painted_at(library: &mut Library, size: (f32, f32)) -> String {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let mut texts = Vec::new();
        for _ in 0..2 {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(size.0, size.1),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Entry", &Campaigns::default());
                    });
                },
            );
            texts = output
                .shapes
                .into_iter()
                .filter_map(|shape| match shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                    _ => None,
                })
                .collect();
        }
        texts.join("\n")
    }

    /// Types `words` into the open Markdown editor, as the DM does: focus, then text input.
    fn sd1_type(library: &mut Library, words: &str) {
        sd1_input(library, vec![vec![egui::Event::Text(words.to_owned())]]);
    }

    /// Sends each batch of input to the open Markdown editor, one frame each, in one context
    /// (so the editor's own Undo remembers the earlier frames). The editor has focus.
    fn sd1_input(library: &mut Library, batches: Vec<Vec<egui::Event>>) {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let note = &library.loaded.as_ref().unwrap().notes[library.selected.unwrap()];
        let id = note.markdown_field().id();
        let input = |events, time| egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 1400.0),
            )),
            events,
            time: Some(time),
            ..Default::default()
        };
        let frames = [vec![], vec![]]
            .into_iter()
            .chain(batches.into_iter().flat_map(|batch| {
                // A quiet frame after each batch, a second later, so the editor settles its Undo.
                [batch, vec![]]
            }));
        let mut last = 0.0;
        for (at, events) in frames.enumerate() {
            last = at as f64 * 2.0;
            let _ = ctx.run(input(events, last), |ctx| {
                ctx.memory_mut(|memory| memory.request_focus(id));
                egui::CentralPanel::default().show(ctx, |ui| {
                    library.editor(ui, "Entry", &Campaigns::default());
                });
            });
        }
        // Layout PR 9b: the focused editor's quiet frames lay out and hash none of its text.
        let work = crate::design::pane::text_work(|| {
            for at in 1..=3 {
                let _ = ctx.run(input(Vec::new(), last + f64::from(at)), |ctx| {
                    ctx.memory_mut(|memory| memory.request_focus(id));
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Entry", &Campaigns::default());
                    });
                });
            }
        });
        assert!(
            work.is_quiet(3),
            "9b-quiet: the Library editor's quiet frames: {work:?}"
        );
    }

    /// Row 125: the owner line's status that ends with `words`, after two frames: its icon, the
    /// icon's colour and the words' colour.
    fn owner_line_look(
        library: &mut Library,
        words: &str,
    ) -> Option<(String, egui::Color32, egui::Color32)> {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let mut look = None;
        for _ in 0..2 {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1200.0, 1400.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Entry", &Campaigns::default());
                    });
                },
            );
            look = output.shapes.iter().find_map(|shape| {
                let egui::Shape::Text(text) = &shape.shape else {
                    return None;
                };
                let job = &text.galley.job;
                (job.text.ends_with(words) && job.sections.len() >= 3).then(|| {
                    let icon = &job.sections[0];
                    (
                        job.text[icon.byte_range.clone()].to_owned(),
                        icon.format.color,
                        job.sections[job.sections.len() - 1].format.color,
                    )
                })
            });
        }
        look
    }

    fn sd1_select(loaded: Loaded, owner: Option<Uuid>, path: &str) -> Library {
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == path && note.owner.id() == owner)
            .unwrap_or_else(|| panic!("{path} is listed"));
        let draft = loaded.notes[selected].text.clone();
        Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: draft.as_str().into(),
            ..Default::default()
        }
    }

    /// SD1: both load paths (package and campaign) keep why a file could not be read. The plain
    /// cases of the same callers stay as they were: a readable entry has no fault, and a file with
    /// a YAML fault keeps its parse message and its text for repair.
    #[test]
    fn iter20_e_sd1_both_load_paths_keep_the_read_fault() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let campaign = sd1_campaign(temp.path());
        let wolf = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &wolf).unwrap();
        fs::write(module.path.join("Bad.md"), NOT_TEXT).unwrap();
        let yaml = wolf.replace("splat_kind: creature", "splat_kind: [");
        fs::write(module.path.join("Yaml.md"), &yaml).unwrap();
        fs::write(campaign.path.join("Notes/Bad.md"), NOT_TEXT).unwrap();
        fs::write(campaign.path.join("Notes/Yaml.md"), &yaml).unwrap();
        let rule = note_text("Rule", "rule", None);
        fs::write(campaign.path.join("Notes/Rule.md"), &rule).unwrap();

        let loaded = load_all(std::slice::from_ref(&module), Some(&campaign)).unwrap();
        let note = |owner: Uuid, path: &str| {
            loaded
                .notes
                .iter()
                .find(|note| note.owner.id() == Some(owner) && note.path == path)
                .unwrap_or_else(|| panic!("{path} is listed"))
        };
        for (owner, path) in [(module.id, "Bad.md"), (campaign.id, "Notes/Bad.md")] {
            let bad = note(owner, path);
            let fault = bad.fault.as_ref().expect("SD1-kind-kept");
            assert_eq!(
                fault.kind,
                crate::world::file_fault::Kind::NotText,
                "{path}"
            );
            assert_eq!(fault.path, path, "SD1-fault-names-the-file");
            assert_eq!(
                bad.error.as_deref(),
                Some("Not a text file"),
                "SD1-plain-cause: {path}"
            );
            assert!(bad.text.is_empty() && bad.entry.is_none());
            // FD2 (round 21): the row's second line names the short cause, as World's row does.
            let at = loaded
                .notes
                .iter()
                .position(|n| std::ptr::eq(n, bad))
                .unwrap();
            // Round 24 AE1, FE2: the cause follows the error mark, as the reader marks it.
            assert!(
                loaded.subtitles()[at].starts_with(&format!("{} Not a text file", icons::ERROR)),
                "iter21-E-FD2-row, iter-24-L-AE1: {path}: {}",
                loaded.subtitles()[at]
            );
            // The list's problem text is the plain cause, never the system's words.
            assert!(
                !loaded
                    .diagnostic_texts()
                    .iter()
                    .any(|text| text.contains("UTF-8") || text.contains("stream")),
                "{:?}",
                loaded.diagnostic_texts()
            );
        }
        for (owner, path) in [(module.id, "Yaml.md"), (campaign.id, "Notes/Yaml.md")] {
            let broken = note(owner, path);
            assert!(broken.fault.is_none(), "a YAML fault is not a read fault");
            // Must stay true: a YAML fault's row still names its line.
            let at = loaded
                .notes
                .iter()
                .position(|n| std::ptr::eq(n, broken))
                .unwrap();
            assert!(
                loaded.subtitles()[at].starts_with(&format!("{} Line ", icons::ERROR)),
                "iter21-E-FD2-yaml-line, iter-24-L-AE1: {path}: {}",
                loaded.subtitles()[at]
            );
            assert_eq!(
                broken.text.as_str(),
                yaml,
                "a YAML fault keeps its text for repair"
            );
            assert!(
                broken
                    .error
                    .as_deref()
                    .is_some_and(|e| e != "Not a text file")
            );
        }
        for (owner, path) in [(module.id, "Wolf.md"), (campaign.id, "Notes/Rule.md")] {
            let plain = note(owner, path);
            assert!(plain.fault.is_none() && plain.error.is_none() && plain.entry.is_some());
            // Must stay true: a readable row has no mark.
            let at = loaded
                .notes
                .iter()
                .position(|n| std::ptr::eq(n, plain))
                .unwrap();
            assert!(
                !loaded.subtitles()[at].contains(icons::ERROR)
                    && !loaded.subtitles()[at].contains(icons::WARN),
                "{path}: {}",
                loaded.subtitles()[at]
            );
        }
        // Round 24: a counted file that cannot be read gives the header count the error tone.
        assert!(loaded.unreadable_counted, "iter-24-L-AE1-header");
        assert_eq!(fs::read(module.path.join("Bad.md")).unwrap(), NOT_TEXT);
        assert_eq!(
            fs::read(campaign.path.join("Notes/Bad.md")).unwrap(),
            NOT_TEXT
        );
    }

    /// SD1: a file the system refuses to read names that cause, from the error's kind.
    #[cfg(unix)]
    #[test]
    fn iter20_e_sd1_a_refused_read_names_its_own_cause() {
        use std::os::unix::fs::PermissionsExt;
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let file = module.path.join("Locked.md");
        fs::write(&file, note_text("Locked", "creature", None)).unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o000)).unwrap();
        if fs::read(&file).is_ok() {
            // Root reads every file; this case cannot be made here.
            fs::set_permissions(&file, fs::Permissions::from_mode(0o644)).unwrap();
            return;
        }
        let loaded = load(Some(&module));
        fs::set_permissions(&file, fs::Permissions::from_mode(0o644)).unwrap();
        let loaded = loaded.unwrap();
        let note = loaded.notes.iter().find(|n| n.path == "Locked.md").unwrap();
        assert_eq!(
            note.fault.as_ref().map(|fault| fault.kind),
            Some(crate::world::file_fault::Kind::ReadDenied)
        );
        assert_eq!(note.error.as_deref(), Some("No permission to read"));
    }

    /// SD1: the reader names the real cause with Show file, Try again and Technical details, and
    /// offers no editor link, no repair of empty text and no source editing. A campaign note with
    /// a YAML fault, in the same reader, still offers both: the check can fail.
    #[test]
    fn iter20_e_sd1_reader_names_the_fault_and_blocks_source_repair() {
        let temp = tempfile::tempdir().unwrap();
        let campaign = sd1_campaign(temp.path());
        fs::write(campaign.path.join("Notes/Bad.md"), NOT_TEXT).unwrap();
        let yaml = note_text("Yaml", "rule", None).replace("splat_kind: rule", "splat_kind: [");
        fs::write(campaign.path.join("Notes/Yaml.md"), &yaml).unwrap();
        // Another loaded note, whose UUID a repair draft can take.
        let other = note_text("Other", "rule", None);
        fs::write(campaign.path.join("Notes/Other.md"), &other).unwrap();

        let loaded = load_all(&[], Some(&campaign)).unwrap();
        let mut library = sd1_select(loaded, Some(campaign.id), "Notes/Yaml.md");
        // Row 125 (G143, G144): `Cannot be read` on the owner line is the error, with its icon and
        // tone, as the row and the header have it; not a muted edit pencil.
        let error = colors().result.error;
        assert_eq!(
            owner_line_look(&mut library, "Campaign · Cannot be read"),
            Some((icons::ERROR.to_owned(), error, error)),
            "row-125-cannot-be-read"
        );
        let yaml_painted = sd1_painted(&mut library);
        for words in ["Edit in Obsidian", "Fix in Edit Markdown", "Show file"] {
            assert!(
                yaml_painted.contains(words),
                "control: {words}\n{yaml_painted}"
            );
        }

        // iter-22-E-SD1: a repair draft's notice and state come from its current parse. The saved
        // file stays unreadable on the owner line until Save.
        library.source = true;
        let painted = sd1_painted(&mut library);
        for words in [
            "Line 4 opens a list",
            "Invalid",
            "Campaign · Cannot be read",
        ] {
            assert!(
                painted.contains(words),
                "SD1-saved-fault: {words}\n{painted}"
            );
        }
        // The stored error has its own words: the old code fell back to them.
        let stored = library.loaded.as_ref().unwrap().notes[library.selected.unwrap()]
            .error
            .clone()
            .unwrap();
        let stored = stored.split(" at line").next().unwrap().to_owned();
        assert!(!stored.contains("opens a list"), "{stored}");
        let valid = yaml.replace("splat_kind: [", "splat_kind: rule");
        let _ = library.draft.replace(valid.clone());
        let painted = sd1_painted(&mut library);
        for words in ["opens a list", stored.as_str()] {
            assert!(
                !painted.contains(words),
                "SD1-valid-notice: {words}\n{painted}"
            );
        }
        assert!(!painted.contains("Invalid"), "SD1-valid-state\n{painted}");
        assert!(painted.contains("Unsaved"), "SD1-valid-unsaved\n{painted}");
        assert!(
            painted.contains("Campaign · Cannot be read"),
            "SD1-owner-line-stays\n{painted}"
        );
        // The reader shows the same draft: it has no notice either.
        library.source = false;
        let painted = sd1_painted(&mut library);
        for words in ["opens a list", stored.as_str()] {
            assert!(
                !painted.contains(words),
                "SD1-reader-notice: {words}\n{painted}"
            );
        }
        assert!(
            !painted.contains("Fix in Edit Markdown"),
            "SD1-reader-fix\n{painted}"
        );
        // Edited back to a new fault: the notice names the draft's line, not the saved one.
        library.source = true;
        let _ = library
            .draft
            .replace(valid.replace("splat_name: Yaml", "splat_name: ["));
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Line 5 opens a list"),
            "SD1-new-fault\n{painted}"
        );
        assert!(!painted.contains("Line 4"), "SD1-old-fault-gone\n{painted}");
        assert!(painted.contains("Invalid"), "SD1-invalid-again\n{painted}");
        // Discard goes back to the saved text and its fault.
        let _ = library.draft.replace(yaml.clone());
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Line 4 opens a list"),
            "SD1-discard\n{painted}"
        );
        assert!(painted.contains("Invalid"), "SD1-discard-state\n{painted}");
        // A draft that parses but takes another note's UUID: Save would refuse it, so the notice
        // names that cause and the state is Invalid. Save leaves the file as it was.
        let id_line = |text: &str| {
            text.lines()
                .find(|line| line.starts_with("splat_id: "))
                .unwrap()
                .to_owned()
        };
        let taken = valid.replace(&id_line(&valid), &id_line(&other));
        assert_ne!(taken, valid);
        assert!(Entry::parse("Notes/Yaml.md", &taken).is_ok());
        let _ = library.draft.replace(taken.clone());
        let painted = sd1_painted(&mut library);
        let cause = "Another entry already uses this UUID or key.";
        assert!(painted.contains(cause), "SD1-taken-notice\n{painted}");
        assert!(painted.contains("Invalid"), "SD1-taken-state\n{painted}");
        assert!(
            !painted.contains("Unsaved"),
            "SD1-taken-not-unsaved\n{painted}"
        );
        library.save_selected(false);
        assert_eq!(
            fs::read(campaign.path.join("Notes/Yaml.md")).unwrap(),
            yaml.as_bytes(),
            "SD1-taken-not-written"
        );
        assert_eq!(library.draft.as_str(), taken, "SD1-taken-draft-kept");
        let painted = sd1_painted(&mut library);
        assert!(painted.contains(cause), "SD1-taken-after-save\n{painted}");
        // Must stay true: a draft Save accepts has no notice, and Save writes it.
        let _ = library.draft.replace(valid.clone());
        let painted = sd1_painted(&mut library);
        assert!(!painted.contains(cause), "SD1-valid-again\n{painted}");
        library.save_selected(false);
        assert_eq!(
            fs::read_to_string(campaign.path.join("Notes/Yaml.md")).unwrap(),
            valid,
            "SD1-valid-written"
        );

        // Round 23 lead note: a ready state means Save would accept the draft. The saved note
        // reads now. A draft of it that parses but takes another UUID is refused by Save's own
        // check, so its state is Invalid with Save's reason, Save is not offered (here or in the
        // close dialog), and a Save attempt leaves the file's bytes as they are.
        let path = campaign.path.join("Notes/Yaml.md");
        assert!(
            library.loaded.as_ref().unwrap().notes[library.selected.unwrap()]
                .error
                .is_none()
        );
        let saved = fs::read(&path).unwrap();
        let moved_id =
            format!("{valid}\nA new line.\n").replace(&id_line(&valid), &id_line(&other));
        assert!(Entry::parse("Notes/Yaml.md", &moved_id).is_ok());
        let _ = library.draft.replace(moved_id.clone());
        let painted = sd1_painted(&mut library);
        let reason = "Keep this entry’s UUID and kind unchanged.";
        assert!(painted.contains(reason), "ready-refused-reason\n{painted}");
        assert!(
            painted.contains("Invalid"),
            "ready-refused-state\n{painted}"
        );
        assert!(
            !painted.contains("Unsaved"),
            "ready-refused-not-unsaved\n{painted}"
        );
        assert!(!library.can_save_and_close(), "ready-refused-no-close-save");
        library.save_selected(false);
        assert_eq!(fs::read(&path).unwrap(), saved, "ready-refused-not-written");
        assert_eq!(library.draft.as_str(), moved_id, "ready-refused-draft-kept");
        // Must stay true: the same note's draft that Save accepts is Unsaved, with no reason.
        let _ = library.draft.replace(format!("{valid}\nA new line.\n"));
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Unsaved"),
            "ready-accepted-unsaved\n{painted}"
        );
        assert!(
            !painted.contains(reason),
            "ready-accepted-no-reason\n{painted}"
        );
        assert!(library.can_save_and_close(), "ready-accepted-close-save");

        // An action-time failure: the folder is read-only, so Save fails on the disk. The state
        // slot says Not saved with Save's own reason, not Unsaved, and the file keeps its bytes.
        use std::os::unix::fs::PermissionsExt;
        let notes = campaign.path.join("Notes");
        let writable = fs::metadata(&notes).unwrap().permissions();
        fs::set_permissions(&notes, fs::Permissions::from_mode(0o555)).unwrap();
        let accepted = library.draft.as_str().to_owned();
        // The failed-Save check counts each run, and `DraftView` counts each read of the words.
        let failure_work =
            crate::test_support::CachedWork::start("Failed-Save check", &mut library, |library| {
                let (reached, work) = library.failure_checks.get();
                crate::test_support::Counts { reached, work }
            });
        library.save_selected(false);
        let reason = library
            .save_failure
            .as_ref()
            .map(|failure| failure.reason.clone())
            .expect("disk-failure-recorded");
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Not saved"),
            "disk-failure-state\n{painted}"
        );
        // The check reads none of the words, even while it is new.
        failure_work
            .warm(&mut library, painted.contains("Not saved"), 0)
            .expect("disk-failure-check-reached");
        assert!(
            painted.contains(&reason),
            "disk-failure-reason {reason}\n{painted}"
        );
        assert!(
            !painted.contains("Unsaved"),
            "disk-failure-not-unsaved\n{painted}"
        );
        assert_eq!(fs::read(&path).unwrap(), saved, "disk-failure-not-written");
        assert_eq!(library.draft.as_str(), accepted, "disk-failure-draft-kept");
        // iter-24-B FL1, lookalike outside the rule: a folder inside the campaign refused the
        // write, not the file or the campaign folder, so the owner line stays Editable.
        assert_eq!(
            library
                .save_failure
                .as_ref()
                .and_then(|failure| failure.read_only),
            None,
            "FL1-folder-inside-not-read-only"
        );
        assert!(
            painted.contains("Campaign · Editable") && !painted.contains("Read only"),
            "FL1-folder-inside-owner-line\n{painted}"
        );
        // Unchanged frames and a resize keep the failure and never read the draft's words to do
        // so. Each paint is two frames, and each frame checks.
        let steady = failure_work
            .steady(&mut library, |library| {
                [
                    (900.0, 1400.0),
                    (900.0, 1400.0),
                    (960.0, 640.0),
                    (1920.0, 1080.0),
                ]
                .into_iter()
                .all(|size| sd1_painted_at(library, size).contains("Not saved"))
            })
            .expect("disk-failure-no-draft-scan");
        assert!(
            steady.reached >= 8,
            "disk-failure-checked-each-frame: {steady:?}"
        );
        // Only that draft owns it: the next edit in the editor ends it, and going back to the
        // same words does not bring it back.
        assert!(library.source, "fixture: Edit Markdown is open");
        failure_work
            .refresh(&mut library, 0, |library| {
                sd1_type(library, "More.");
                assert_ne!(
                    library.draft.as_str(),
                    accepted,
                    "fixture: the editor took the words"
                );
                let painted = sd1_painted(library);
                painted.contains("Unsaved") && !painted.contains("Not saved")
            })
            .expect("disk-failure-edit-ends");
        // Edit, then the editor's Undo back to the very words that failed: still ended.
        let failed_again = |library: &mut Library| {
            library.save_selected(false);
            assert!(
                sd1_painted(library).contains("Not saved"),
                "fixture: the Save failed again"
            );
        };
        let _ = library.draft.replace(accepted.clone());
        library.draft_revision += 1;
        failed_again(&mut library);
        sd1_input(
            &mut library,
            vec![
                vec![egui::Event::Text("More.".into())],
                vec![egui::Event::Key {
                    key: egui::Key::Z,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND,
                }],
            ],
        );
        assert_eq!(
            library.draft.as_str(),
            accepted,
            "fixture: Undo gave back the failed words"
        );
        let painted = sd1_painted(&mut library);
        assert!(
            !painted.contains("Not saved") && painted.contains("Unsaved"),
            "disk-failure-stays-ended\n{painted}"
        );
        // Never on another file, even with the same words: the failure is Yaml.md's.
        failed_again(&mut library);
        let yaml_at = library.selected.unwrap();
        let other_at = library
            .loaded
            .as_ref()
            .unwrap()
            .notes
            .iter()
            .position(|note| note.path == "Notes/Other.md")
            .unwrap();
        library.selected = Some(other_at);
        let _ = library.draft.replace(accepted.clone());
        let painted = sd1_painted(&mut library);
        assert!(
            !painted.contains("Not saved"),
            "disk-failure-not-on-other\n{painted}"
        );
        library.selected = Some(yaml_at);
        let painted = sd1_painted(&mut library);
        assert!(
            !painted.contains("Not saved"),
            "disk-failure-owner-ended\n{painted}"
        );
        // It comes back on another failed Save, and a Save that succeeds ends it.
        library.save_selected(false);
        assert!(
            sd1_painted(&mut library).contains("Not saved"),
            "disk-failure-again"
        );
        fs::set_permissions(&notes, writable).unwrap();
        library.save_selected(false);
        let painted = sd1_painted(&mut library);
        assert!(
            !painted.contains("Not saved"),
            "disk-failure-save-ends\n{painted}"
        );
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            accepted,
            "disk-failure-then-written"
        );
        // iter-24-B FL1, AL1, FL2, SL1: the file itself is read only. Save's reason says what to
        // do next, and the owner line says Read only while that failure lasts. The next edit
        // ends both.
        let file_permissions = fs::metadata(&path).unwrap().permissions();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o444)).unwrap();
        let _ = library
            .draft
            .replace(format!("{accepted}\nA locked file.\n"));
        library.draft_revision += 1;
        library.save_selected(false);
        let painted = sd1_painted(&mut library);
        fs::set_permissions(&path, file_permissions).unwrap();
        assert_eq!(
            library
                .save_failure
                .as_ref()
                .and_then(|failure| failure.read_only),
            Some(ReadOnly::File),
            "FL1-kind"
        );
        assert!(
            painted.contains("Campaign · Read only")
                && !painted.contains("Campaign · Editable")
                && painted.contains("This file is read only. Make it writable, then Save again."),
            "FL1-read-only-owner-line\n{painted}"
        );
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            accepted,
            "FL1-not-written"
        );
        sd1_type(&mut library, "More.");
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Campaign · Editable") && !painted.contains("Read only"),
            "FL1-edit-ends\n{painted}"
        );
        // The repair-link flow: an entry that cannot be read opens as a repair draft in Edit
        // Markdown (as `repair_file` opens it). Its fixed draft parses, so Save would accept it,
        // and the disk refuses the write: the same Not saved state, for this draft.
        let broken = note_text("Broken", "rule", None).replace("splat_kind: rule", "splat_kind: [");
        fs::write(notes.join("Broken.md"), &broken).unwrap();
        let loaded = load_all(&[], Some(&campaign)).unwrap();
        let mut repair = sd1_select(loaded, Some(campaign.id), "Notes/Broken.md");
        repair.source = true;
        assert!(
            sd1_painted(&mut repair).contains("Invalid"),
            "fixture: a repair draft"
        );
        let fixed = broken.replace("splat_kind: [", "splat_kind: rule");
        sd1_input(
            &mut repair,
            vec![
                vec![egui::Event::Key {
                    key: egui::Key::A,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::COMMAND,
                }],
                vec![egui::Event::Text(fixed.clone())],
            ],
        );
        assert_eq!(
            repair.draft.as_str(),
            fixed,
            "fixture: the editor holds the fixed draft"
        );
        let writable = fs::metadata(&notes).unwrap().permissions();
        fs::set_permissions(&notes, fs::Permissions::from_mode(0o555)).unwrap();
        repair.save_selected(false);
        let painted = sd1_painted(&mut repair);
        let reason = repair
            .save_failure
            .as_ref()
            .map(|failure| failure.reason.clone())
            .expect("repair-disk-failure-recorded");
        fs::set_permissions(&notes, writable).unwrap();
        assert!(
            painted.contains("Not saved")
                && painted.contains(&reason)
                && !painted.contains("Unsaved"),
            "repair-disk-failure-state\n{painted}"
        );
        assert_eq!(
            fs::read_to_string(notes.join("Broken.md")).unwrap(),
            broken,
            "repair-disk-failure-not-written"
        );

        // Lookalike outside the rule: another identity is not in the draft's parse, so a draft
        // that parses keeps the stored fault and Invalid until the file changes.
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let manifest = module.path.join("Package.md");
        let text = fs::read_to_string(&manifest).unwrap();
        let other = text.replace(&module.id.to_string(), &Uuid::new_v4().to_string());
        assert_ne!(other, text);
        fs::write(&manifest, &other).unwrap();
        let mut library = sd1_select(load(Some(&module)).unwrap(), Some(module.id), "Package.md");
        library.source = true;
        let _ = library.draft.replace(format!("{other}\nA description.\n"));
        assert!(
            crate::content_workspace::repair_problem("Package.md", &library.draft)
                .0
                .is_none()
        );
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("another identity"),
            "SD1-lookalike-notice\n{painted}"
        );
        assert!(
            painted.contains("Invalid"),
            "SD1-lookalike-state\n{painted}"
        );

        let loaded = load_all(&[], Some(&campaign)).unwrap();
        let mut library = sd1_select(loaded, Some(campaign.id), "Notes/Bad.md");
        // Edit Markdown was chosen before (for example by a repair link): it does not open.
        library.source = true;
        let painted = sd1_painted(&mut library);
        assert!(!library.source, "SD1-no-source-editing");
        for words in [
            // Round 24 FE5: the file line names the file; the error says `this file`.
            "Splat cannot read this file as text.",
            "Splat reads notes as UTF-8 text.",
            "Then choose Try again.",
            "Try again",
            "Show file",
            "Technical details",
            "Campaign · Cannot be read",
        ] {
            assert!(painted.contains(words), "SD1-shows: {words}\n{painted}");
        }
        for words in [
            "Edit in Obsidian",
            "Fix in Edit Markdown",
            "Missing YAML frontmatter",
            "Missing frontmatter",
            "Save",
            "cannot read Notes/Bad.md",
        ] {
            assert!(!painted.contains(words), "SD1-hides: {words}\n{painted}");
        }
        assert_eq!(
            fs::read(campaign.path.join("Notes/Bad.md")).unwrap(),
            NOT_TEXT
        );
    }

    /// SD1: a package manifest that is not text is never replaced by the manifest repair.
    #[test]
    fn iter20_e_sd1_an_unreadable_manifest_offers_no_replacement() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::write(module.path.join("Package.md"), NOT_TEXT).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let mut library = sd1_select(loaded, Some(module.id), "Package.md");
        assert!(
            library.loaded.as_ref().unwrap().notes[library.selected.unwrap()].is_manifest(),
            "the manifest is found by its file name"
        );
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Splat cannot read this file as text."),
            "{painted}"
        );
        assert!(painted.contains("Try again"), "{painted}");
        assert!(
            !painted.contains("Replace with a basic"),
            "SD1-no-replace\n{painted}"
        );
        assert!(!painted.contains("Write the manifest again"), "{painted}");
        assert_eq!(fs::read(module.path.join("Package.md")).unwrap(), NOT_TEXT);
    }

    /// SD1, action-time failure: a note that read as text when opened stops reading. Save, a
    /// later disk check and Reload each keep the draft or its Undo, name the cause, and never
    /// change the file's bytes. When the file reads again, the fault ends.
    #[test]
    fn iter20_e_sd1_a_file_that_stops_reading_keeps_the_draft_and_its_bytes() {
        let temp = tempfile::tempdir().unwrap();
        // A package in an Obsidian vault, so its file line offers Edit in Obsidian while it reads.
        fs::create_dir(temp.path().join(".obsidian")).unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let file = module.path.join("Wolf.md");
        let text = note_text("Wolf", "creature", None);
        fs::write(&file, &text).unwrap();
        let ctx = egui::Context::default();
        let mut library = sd1_select(load(Some(&module)).unwrap(), Some(module.id), "Wolf.md");
        let draft = format!("{text}\nMy draft.\n");
        let _ = library.draft.replace(draft.clone());
        let painted = sd1_painted(&mut library);
        assert!(
            painted.contains("Edit in Obsidian"),
            "control: the vault link shows\n{painted}"
        );
        fs::write(&file, NOT_TEXT).unwrap();

        // Save: the write reads the file first and refuses it.
        library.save_selected(false);
        let fault = |library: &Library| {
            library
                .disk_change
                .as_ref()
                .and_then(|change| change.fault.as_ref())
                .map(|fault| fault.kind)
        };
        assert_eq!(
            fault(&library),
            Some(crate::world::file_fault::Kind::NotText),
            "SD1-save"
        );
        assert!(library.error.is_none(), "{:?}", library.error);
        assert_eq!(library.draft.as_str(), draft, "SD1-save-keeps-draft");
        assert_eq!(fs::read(&file).unwrap(), NOT_TEXT, "SD1-save-keeps-bytes");
        let painted = sd1_painted(&mut library);
        for words in [
            "Splat cannot read this file as text. Your draft is safe here until you choose.",
            "Try again",
            "Reload and lose my draft",
            "Technical details",
        ] {
            assert!(
                painted.contains(words),
                "SD1-conflict-shows: {words}\n{painted}"
            );
        }
        for words in [
            "Save my draft again",
            "Overwrite with my draft",
            "was deleted on disk",
            "Edit in Obsidian",
        ] {
            assert!(
                !painted.contains(words),
                "SD1-conflict-hides: {words}\n{painted}"
            );
        }

        // A later disk check (focus, or Try again) finds the same fault, not a deleted file.
        library.disk_change = None;
        library.refresh_destination(&module.path, &ctx, &Campaigns::default());
        assert_eq!(
            fault(&library),
            Some(crate::world::file_fault::Kind::NotText),
            "SD1-check"
        );
        assert!(library.disk_change.as_ref().unwrap().text.is_none());

        // The file reads as before again: the fault ends and the draft stays.
        fs::write(&file, &text).unwrap();
        library.refresh_destination(&module.path, &ctx, &Campaigns::default());
        assert!(library.disk_change.is_none(), "SD1-fault-ends");
        assert_eq!(library.draft.as_str(), draft);

        // Reload and lose my draft: the note becomes the unreadable file; Undo keeps the draft.
        fs::write(&file, NOT_TEXT).unwrap();
        library.refresh_destination(&module.path, &ctx, &Campaigns::default());
        let selected = library.selected.unwrap();
        library.reload_note_from_disk(selected);
        let note = &library.loaded.as_ref().unwrap().notes[selected];
        assert_eq!(
            note.fault.as_ref().map(|fault| fault.kind),
            Some(crate::world::file_fault::Kind::NotText),
            "SD1-reload"
        );
        assert!(note.text.is_empty() && note.entry.is_none());
        assert_eq!(
            library.kept.discarded().map(|(_, _, text)| text.as_str()),
            Some(draft.as_str()),
            "SD1-reload-undo"
        );
        assert!(library.stale, "the next frame reads every file again");
        assert_eq!(fs::read(&file).unwrap(), NOT_TEXT, "SD1-reload-keeps-bytes");

        // With no draft, a disk check reloads, and the new reading keeps the fault.
        let mut library = sd1_select(
            {
                fs::write(&file, &text).unwrap();
                load(Some(&module)).unwrap()
            },
            Some(module.id),
            "Wolf.md",
        );
        fs::write(&file, NOT_TEXT).unwrap();
        library.refresh_destination(&module.path, &ctx, &Campaigns::default());
        assert!(library.disk_change.is_none() && library.stale);
        let loaded = load(Some(&module)).unwrap();
        let note = loaded.notes.iter().find(|n| n.path == "Wolf.md").unwrap();
        assert!(note.fault.is_some(), "SD1-reread");
        assert_eq!(fs::read(&file).unwrap(), NOT_TEXT);
    }

    /// FD2: the choice after a package copy's registration names what it changes (encounters),
    /// in the words of its result.
    #[test]
    fn iter20_e_fd2_the_copy_choice_names_what_it_changes() {
        assert!(
            USE_COPY.contains("encounters"),
            "FD2-names-encounters: {USE_COPY}"
        );
        let choice = USE_COPY.trim_end_matches(" in encounters").to_lowercase();
        assert!(
            USE_COPY_RESULT
                .as_str()
                .to_lowercase()
                .contains(&format!("encounters now {choice}")),
            "FD2-same-words: {USE_COPY} / {}",
            USE_COPY_RESULT.as_str()
        );
        // M-L0.4b: the typed footer's checked words are the shared words of World and the
        // workspace, which other pages still draw.
        assert_eq!(
            design::outcome::Fault::cause(&crate::world::DuplicateIdentity(
                crate::world::IdentityNoun::Entry
            ))
            .as_str(),
            crate::world::DUPLICATE_ENTRY
        );
        assert_eq!(
            manifest_replace_help("Package.md").as_str(),
            crate::content_workspace::manifest_replace_consequence("Package.md")
        );
    }

    /// SD1: a save that looks for its moved file names an unreadable file in plain words.
    #[test]
    fn iter20_e_sd1_a_moved_file_search_names_an_unreadable_file() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let mut library = sd1_select(load(Some(&module)).unwrap(), Some(module.id), "Wolf.md");
        let _ = library.draft.replace(format!("{text}\nMy draft.\n"));
        fs::remove_file(module.path.join("Wolf.md")).unwrap();
        fs::write(module.path.join("Other.md"), NOT_TEXT).unwrap();
        library.save_selected(false);
        let error = library
            .error
            .as_ref()
            .map(PageError::words)
            .expect("the search stops at the unreadable file")
            .to_owned();
        assert!(
            error.starts_with("Splat cannot read Other.md as text."),
            "SD1-moved: {error}"
        );
        assert!(!error.contains("stream"), "{error}");
        assert!(
            matches!(library.error, Some(PageError::Outside(_))),
            "the system's words stay outside words"
        );
        assert!(!module.path.join("Wolf.md").exists(), "nothing is written");
        assert_eq!(fs::read(module.path.join("Other.md")).unwrap(), NOT_TEXT);
        // Two files in its folder have its ID: the app's own refusal, checked.
        fs::remove_file(module.path.join("Other.md")).unwrap();
        for name in ["Wolf A.md", "Wolf B.md"] {
            fs::write(module.path.join(name), &text).unwrap();
        }
        library.save_selected(false);
        assert_eq!(
            library.error,
            Some(PageError::Own(SEVERAL_FILES_KEEP_ONE.clone())),
            "one folder with two files of its ID"
        );
        // One file in each of two folders: the other refusal, checked.
        fs::remove_file(module.path.join("Wolf B.md")).unwrap();
        let other = create_module(&temp.path().join("other"), "Other", "other").unwrap();
        fs::write(other.path.join("Wolf.md"), &text).unwrap();
        library
            .loaded
            .as_mut()
            .unwrap()
            .owners
            .push(Owner::Package(other));
        library.save_selected(false);
        assert_eq!(
            library.error,
            Some(PageError::Own(SEVERAL_FILES_CHOOSE_ONE.clone())),
            "two folders, each with a file of its ID"
        );
    }
}

/// What a draft writer changes (the stat fix, the editor's Discard): the kept results, the
/// draft, its revision and the page's error.
type DraftParts<'a> = (
    &'a mut LibraryKept,
    &'a mut crate::design::text_draft::TextDraft,
    &'a mut u64,
    &'a mut Option<PageError>,
);

/// The warning for a first heading that differs from the entry's name (its words, its action
/// and the fixed text), and the result message whose `Undo heading change` shows for this draft.
struct HeadingChoice {
    /// The warning, the entry's name (the fix's `Use <name>`) and the draft the fix writes.
    pub fix: Option<(Sentence, String, String)>,
    /// The fix's result, while its Undo shows.
    pub undo: Option<Sentence>,
}

impl HeadingChoice {
    /// `entry` is `None` when the file cannot take the fix (read only, unparsed or in a conflict).
    /// The fix's Undo shows only for its own file (owner kind, owner id and path: `FileId`)
    /// while the draft is the text the fix wrote (M-L0.4a).
    /// `same`: the kept answer of the draft's compare with the fix's words (layout PR 9b-C).
    pub fn new(
        entry: Option<&Entry>,
        draft: &crate::design::text_draft::TextDraft,
        undo: Option<&kept::HeadingFix>,
        file: &FileId,
        same: &crate::design::text_draft::KeptCompare,
    ) -> Self {
        let Some(entry) = entry else {
            return Self {
                fix: None,
                undo: None,
            };
        };
        Self {
            fix: presentation::heading_fix(draft, entry).map(|(old, after)| {
                (
                    crate::sentence!(
                        "The first heading says {}, but the name is {}.",
                        old,
                        entry.name
                    ),
                    entry.name.clone(),
                    after,
                )
            }),
            undo: undo
                .is_some_and(|(at, _, after)| {
                    at.0 == file.0 && at.1 == file.1 && draft.same_as(after.revision(), after, same)
                })
                .then(|| crate::sentence!("The first heading now says {}.", entry.name)),
        }
    }

    /// Applies a press on the fix or its Undo to the draft. Returns true when the draft changed
    /// and must be saved.
    pub fn apply(
        &self,
        (fix, undo): (bool, bool),
        file: &FileId,
        draft: &mut crate::design::text_draft::TextDraft,
        kept: &mut LibraryKept,
    ) -> bool {
        let mut changed = false;
        if fix && let Some((_, _, after)) = &self.fix {
            kept.set_heading((
                file.clone(),
                draft.as_str().to_owned(),
                crate::design::memo::Revised::new(after.clone()),
            ));
            let _ = draft.replace(after.as_str());
            changed = true;
        }
        if undo && let Some((_, before, _)) = kept.take_heading() {
            let _ = draft.replace(before);
            changed = true;
        }
        changed
    }
}

/// A Save that failed on the disk (a read-only file or folder, a write error): an action-time
/// failure. It belongs to the draft Save tried to write, by owner, path and draft revision. The draft's
/// state slot says `Not saved` with Save's reason until the draft changes, another entry opens,
/// or a Save succeeds. Nothing reads the disk to show it.
struct SaveFailure {
    pub owner: OwnerKey,
    pub path: String,
    pub revision: u64,
    pub reason: String,
    /// FL1: Save found the file or its folder read only, so the owner line says `Read only`
    /// for as long as this failure lasts.
    pub read_only: Option<ReadOnly>,
}

impl SaveFailure {
    /// True while the open draft is still the one this Save tried to write: the same entry and no
    /// change event since. The check reaches the draft's words only through `draft`, which counts
    /// each read in tests; it needs none of them, so no frame compares or hashes the draft.
    pub fn owns(&self, note: &Note, revision: u64, draft: DraftView<'_>) -> bool {
        let _ = draft;
        self.owner == note.owner.key() && self.path == note.path && self.revision == revision
    }
}

use draft_view::DraftView;

/// The open draft's words, as the failed-Save check may see them. The field is private to this
/// module, so the only way to the words is `Deref`, and in tests every read adds one to the
/// check's read count.
mod draft_view {
    pub(super) struct DraftView<'a> {
        text: &'a str,
        #[cfg(any(test, feature = "test-support"))]
        checks: &'a std::cell::Cell<(usize, usize)>,
    }

    impl<'a> DraftView<'a> {
        pub(super) fn new(
            text: &'a str,
            #[cfg(any(test, feature = "test-support"))] checks: &'a std::cell::Cell<(usize, usize)>,
        ) -> Self {
            Self {
                text,
                #[cfg(any(test, feature = "test-support"))]
                checks,
            }
        }
    }

    impl std::ops::Deref for DraftView<'_> {
        type Target = str;
        fn deref(&self) -> &str {
            #[cfg(any(test, feature = "test-support"))]
            {
                let (checks, reads) = self.checks.get();
                self.checks.set((checks, reads + 1));
            }
            self.text
        }
    }
}

crate::create_reasons! {
    /// Why a creation form of Manage packages waits: every reason it can show, so its kept line
    /// keeps room for the tallest (round 28 FA7: one inline creation form for all three).
    enum PackageReason {
        ChangedFile => "Choose what to do with the changed file first.",
        CampaignDraft => "Save or discard to copy, add an entry, change packages or switch campaign.",
        Draft => "Save or discard to copy, add an entry or change packages.",
        Loading => "Wait for the Library to load.",
        NoFolder => "Choose a folder first.",
        Copy => "Register it as a new package to add it.",
        NoName => "Type a name first.",
    }
}
