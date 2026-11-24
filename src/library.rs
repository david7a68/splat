//! The Library page and its authoring state.
#[path = "library/kept.rs"]
mod kept;
#[path = "library/model.rs"]
pub mod model;
use crate::{
    campaigns::Campaigns,
    content_workspace::CampaignDocument,
    design::{
        self, Tone, colors, icons,
        outcome::{
            Choice, FaultMessage, FaultStatus, KeptOutcome, MessageChoice, Messages, Notice, Offer,
            PaneFrame, Press, Said, Sentence, UndoStep, Verb,
        },
        results::{ByTicket, Ticket},
    },
    history::{Step, Trail},
    matching::{Rank, Typed},
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
    copied_package: Option<Module>,
    copy_name: String,
    copy_name_open: bool,
    /// A save was refused because the file changed. The draft waits for the DM's choice.
    disk_change: Option<DiskChange>,
    /// The note on screen was saved, and has not changed since.
    just_saved: bool,
    scope: Scope,
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
    /// The start of every Library frame, before anything draws (both UIs): the New entry form's
    /// error ends with the form; the repair guidance and Remove's Undo end with their owners;
    /// a load starts, or its result arrives; an edit or a repair another page asked for opens.
    pub fn begin(&mut self, ctx: &egui::Context, campaigns: &mut Campaigns) {
        // The New entry form's error ends with the form, however it closed.
        if !self.new_form {
            self.new_error = None;
        }
        self.end_repair_on_owner_change();
        self.end_remove_on_selection();
        // The scope notice stands while the list keeps the scope it widened to, with no query:
        // the DM's own scope change or search ends it. (`open_entry` changes the scope itself,
        // so the list's last scope is not the notice's.) Settled here, before every footer of
        // the page: the empty Campaign scope's, a form's and the reader's.
        let searching = !self.search.trim().is_empty();
        if self
            .kept
            .scope_notice()
            .is_some_and(|(_, widened)| *widened != self.scope || searching)
        {
            let _ = self.kept.take_scope_notice();
        }
        let key = load_key(campaigns);
        if self.load_key.is_none() && self.loaded.is_some() {
            self.load_key = Some(key.clone());
        }
        let changed = self.load_key.as_ref().is_some_and(|known| *known != key);
        if self.loaded.is_none() && self.pending.is_none() && self.error.is_none() {
            self.start_load(ctx, campaigns);
        } else if (changed || self.stale) && !self.dirty() && self.pending.is_none() {
            self.stale = false;
            self.start_load(ctx, campaigns);
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

    /// Show the catalogue limited to this campaign's own entries.
    pub fn show_campaign_entries(&mut self) {
        self.scope = Scope::Campaign;
        self.search.clear();
        self.section = Section::All;
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
}

#[cfg(feature = "test-support")]
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
        let library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        assert_eq!(
            issues_word(&library, None),
            Some("3 files need fixing".to_owned())
        );
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

        let library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        // Linker, Both (flag and link) and the unreadable file: three, not four.
        assert_eq!(
            issues_word(&library, Some(&other)),
            Some("3 files need fixing".to_owned())
        );
    }

    /// The header's issues count (`Library::header`), as the page's title line shows it.
    fn issues_word(
        library: &Library,
        index: Option<&std::sync::Arc<crate::lookup::ContentIndex>>,
    ) -> Option<String> {
        library
            .header(&Campaigns::default(), index, true)
            .issues
            .map(|issues| issues.word)
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

        let library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        assert_eq!(
            issues_word(&library, Some(&index)),
            Some("3 files need fixing".to_owned())
        );

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
        let folder = tempfile::tempdir().unwrap();
        let other = tempfile::tempdir().unwrap();
        let mut library = Library {
            path: folder.path().display().to_string(),
            copied_package: Some(Module {
                id: Uuid::new_v4(),
                name: "Copied package".into(),
                path: folder.path().canonicalize().unwrap(),
            }),
            loaded: Some(Loaded::default()),
            ..Default::default()
        };
        let campaigns = Campaigns::default();
        let form = library.package_form_view(&campaigns);
        assert!(form.add_blocked.is_some() && form.copy.is_some());
        library.set_package_path(&other.path().display().to_string());
        let form = library.package_form_view(&campaigns);
        assert_eq!(form.add_blocked, None);
        assert!(form.copy.is_none() && library.copied_package.is_none());
        // Round 28 K4a (the review of PR 341): while the Library loads, Browse waits and says
        // why; once it has loaded, Browse is open again.
        assert_eq!(form.browse_blocked, None);
        let (_loading, rx) = mpsc::channel();
        library.pending = Some(rx);
        let form = library.package_form_view(&campaigns);
        assert_eq!(
            form.browse_blocked.as_deref(),
            Some("Browse waits for the reason below.")
        );
        assert_eq!(
            form.add_blocked.as_deref(),
            Some("Wait for the Library to load."),
            "the reason it points to"
        );
        library.pending = None;
        assert_eq!(library.package_form_view(&campaigns).browse_blocked, None);
    }

    #[test]
    fn iter2_d_about_selects_its_own_navigation_row() {
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
        library.loaded.as_mut().unwrap().invalidate();
        library.scope = Scope::Package(package);
        let campaigns = Campaigns::default();
        // The About row's and the All entries row's selected marks, as the list shows them.
        let marks = |library: &mut Library| {
            let list = library
                .list_view(&campaigns, None, false, 1920.0)
                .expect("loaded");
            let (index, label, about) = list.about.expect("the About row");
            assert_eq!(label, "About First");
            let all = list
                .groups
                .iter()
                .flat_map(|group| &group.rows)
                .find(|row| row.section == Section::All)
                .expect("the All entries row");
            (index, about, all.selected)
        };
        let (about, ..) = marks(&mut library);
        library.open_about(about);
        assert_eq!(marks(&mut library), (about, true, false));
        library.choose_section(Section::All, None);
        assert_eq!(marks(&mut library), (about, false, true));
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
                    // The typed locks of the page's footer (M-L0.4b: `page_messages`). A typed
                    // lock is not debt, but this count reads text.
                    (".blocked(", 3),
                    ("blocked_if(", 0),
                    ("design::locked(", 0),
                    ("undo_message(", 0),
                ],
            ),
            (
                "src/library/catalogue.rs",
                &[
                    ("kept_result", 0),
                    (".blocked(", 2),
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

    /// Round 30 SH1: the New entry folder field reads its folders once per open of its
    /// suggestions menu (`Library::new_folder_choices_if`): while the menu stays open on the same
    /// root and kind it reads nothing; a menu that opens again (nothing kept), another kind and
    /// another root each read again.
    #[test]
    fn folder_suggestions_read_once_per_open_menu() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("book"), "Book", "book").unwrap();
        for folder in ["Creatures/Undead", "Spells", ".hidden"] {
            fs::create_dir_all(module.path.join(folder)).unwrap();
        }
        let other = create_module(&temp.path().join("other"), "Other", "other").unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.register_module(module.clone()).unwrap();
        campaigns.register_module(other.clone()).unwrap();
        let mut library = Library {
            loaded: Some(Loaded::default()),
            scope: Scope::Package(module.id),
            ..Default::default()
        };
        library.open_new_entry();
        library.set_new_kind("spell");
        let reads = || FOLDER_READS.with(std::cell::Cell::get);
        let before = reads();
        let (root, folders) = library.new_folder_choices_if(&campaigns, None).unwrap();
        assert_eq!(root, module.path);
        assert_eq!(folders, ["Creatures", "Creatures/Undead", "Spells"]);
        let one_read = reads() - before;
        assert!(one_read > 0);
        for _ in 0..3 {
            assert!(
                library
                    .new_folder_choices_if(&campaigns, Some((&root, "spell")))
                    .is_none()
            );
        }
        assert_eq!(
            reads() - before,
            one_read,
            "an open menu's redraw reads nothing"
        );
        // The menu opens again: a fresh read shows a folder made meanwhile.
        fs::create_dir(module.path.join("Items")).unwrap();
        let (_, folders) = library.new_folder_choices_if(&campaigns, None).unwrap();
        assert!(folders.contains(&"Items".to_owned()));
        // Another kind, and another root, are other lists.
        library.set_new_kind("rule");
        let at = reads();
        let (_, folders) = library
            .new_folder_choices_if(&campaigns, Some((&root, "spell")))
            .expect("another kind reads again");
        assert!(!folders.contains(&"Spells/".to_owned()) && reads() > at);
        library.scope = Scope::Package(other.id);
        let at = reads();
        let (other_root, folders) = library
            .new_folder_choices_if(&campaigns, Some((&root, "rule")))
            .expect("another root reads again");
        assert_eq!(other_root, other.path);
        assert!(!folders.contains(&"Items".to_owned()) && reads() > at);
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

    /// The scope drop-down's choices (`Library::scope_choices`) follow the catalogue: a renamed
    /// package shows its new name, an open campaign adds its two scopes, and an unregistered
    /// selection falls back to the first choice.
    #[test]
    fn scope_picker_fits_whole_words_and_reuses_unchanged_choices() {
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
        let mut library = Library {
            scope: Scope::Package(second.id),
            ..Default::default()
        };
        let chosen = |library: &Library, campaigns: &Campaigns| {
            let (choices, chosen, _) = library.scope_choices(campaigns);
            choices[chosen].1.clone()
        };
        assert_eq!(chosen(&library, &campaigns), "Ashen Marches");
        campaigns
            .rename_module(second.id, "Renamed package".into())
            .unwrap();
        assert_eq!(chosen(&library, &campaigns), "Renamed package");
        campaigns.open_for_test(&sd1_campaign(temp.path()));
        let (choices, _, group) = library.scope_choices(&campaigns);
        assert_eq!(
            choices
                .iter()
                .map(|(_, words)| words.as_str())
                .collect::<Vec<_>>(),
            [
                "Can use",
                "Everything",
                BUILTIN_NAME,
                "First package",
                "Renamed package",
                "Campaign entries"
            ]
        );
        assert_eq!(group, 2);
        library.scope = Scope::Campaign;
        assert_eq!(chosen(&library, &campaigns), "Campaign entries");
        library.scope = Scope::Package(second.id);
        campaigns.unregister_module(second.id).unwrap();
        assert_eq!(chosen(&library, &campaigns), "Can use", "missing selection");
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
            let readable = presentation::overview_markdown(overview.body);
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
            // The reader shows the overview as this prose: the expected words, with no source
            // markup.
            assert!(readable.contains(expected), "{readable}");
            assert!(!readable.contains('\\'), "{readable}");
            assert!(!readable.contains("[["), "{readable}");
            assert!(!readable.contains("Original layout:"), "{readable}");
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
        // Detect Magic and Guiding Bolt open from the stat block's spell tiles
        // (`Library::open_link`, the reader's link): a clean reader opens the spell in the
        // Library, in its section and outside the search; a draft stays, and the spell opens
        // beside it in Lookup.
        for (dirty, place, title) in [
            (false, 1, "# Detect Magic"),
            (false, 2, "# Guiding Bolt"),
            (true, 2, ""),
        ] {
            library.open_entry(0);
            library.search = "bronze".into();
            library.matches = Some((Section::Creatures, "bronze".into(), vec![0]));
            library.peek = None;
            if dirty {
                library.draft.push_str("\nUnsaved note.\n");
            }
            library.open_link(place);
            if dirty {
                assert_eq!(library.selected, Some(0));
                assert!(library.draft.ends_with("Unsaved note.\n"));
                assert_eq!(
                    library.peek,
                    library.loaded.as_ref().unwrap().notes[place]
                        .entry
                        .as_ref()
                        .map(|entry| entry.id),
                    "the spell opens beside the draft"
                );
                let _ = library
                    .draft
                    .replace(library.loaded.as_ref().unwrap().notes[0].text.as_str());
            } else {
                assert_eq!(library.selected, Some(place));
                assert_eq!(library.section, Section::Spells);
                assert!(library.search.is_empty());
                assert!(library.matches.is_none());
                assert!(!library.source);
                assert!(library.draft.contains(title));
            }
        }
        // The Druid's stat block lists end their lines with `\` and cast a spell no note has:
        // its spells resolve, and the same notes resolve them once.
        library.open_entry(8);
        let loaded = library.loaded.as_mut().unwrap();
        let druid = loaded.notes[8].entry.clone().unwrap();
        spells::resolve(&druid, &*loaded);
        let looked = SPELL_LOOKUPS.with(std::cell::Cell::get);
        spells::resolve(&druid, &*loaded);
        assert_eq!(
            SPELL_LOOKUPS.with(std::cell::Cell::get),
            looked,
            "the Druid's spells are looked up once"
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
    fn reviewed_summary_keeps_rules_and_source() {
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
    }

    #[test]
    fn library_toolbar_keeps_back_and_says_the_draft_reason_once() {
        let loaded = load(None).unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(0);
        library.open_entry(1);
        let painted = sd1_painted(&mut library);
        for word in ["Read", "Back"] {
            assert!(
                painted.lines().any(|text| text == word),
                "Missing {word}: {painted}"
            );
        }

        // M2 (packet E1): a campaign creature's draft with changes and a heading that no longer
        // matches its name: `Make NPC note` waits for the draft, and the draft's reason is said
        // once, by the save row's slot, whatever the footer holds (here the heading warning).
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
        let painted = sd1_painted(&mut library);
        assert!(
            painted
                .lines()
                .any(|text| text.starts_with("The first heading says")),
            "the footer holds the heading warning: {painted}"
        );
        let reason = design::DraftBlocks::EntryInCampaign.long();
        assert_eq!(
            painted.lines().filter(|text| *text == reason).count(),
            1,
            "the draft's reason is said once: {painted}"
        );
    }

    #[test]
    fn class_names_have_no_colon_and_old_names_stay_aliases() {
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
            let rendered = sd1_painted(&mut library);
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
                Some((icons::WARN.to_owned(), warning)),
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
        let painted = page_words;
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
        // The warning in the reader's typed footer, as plain lines (M-L0.4b).
        let frame = PaneFrame::plain();
        let mut messages =
            Messages::<LibraryPress>::new(&frame, "duplicates", design::READER_MEASURE);
        messages.outcome(duplicate_notice(
            &egui::Context::default(),
            &loaded,
            &loaded.notes[0],
            &[0, 2],
            None,
        ));
        let painted = footer_words(&messages.into_plain());
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
    fn the_empty_readers_undos_name_their_object_whole() {
        // M-L0.4b, the M9 labels (`Undo unregister <package>`, `Undo remove <name>`) in the
        // empty reader's typed footer: each Undo carries its whole label (the GPUI footer shows
        // its short form where the whole one does not fit), and an unchanged frame does no
        // content-sized work for the Remove line's words.
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        // Long names, as the DM's data may hold.
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
        let campaigns = Campaigns::default();
        let words = reader_words(&mut library, &campaigns);
        for whole in [
            format!("Undo unregister {package}"),
            format!("Undo remove {entry}"),
        ] {
            assert_eq!(
                words.iter().filter(|text| **text == whole).count(),
                1,
                "the whole label: {whole}\n{words:#?}"
            );
        }
        let scans = || DEPENDENT_SCANS.with(std::cell::Cell::get);
        let copied = || DEPENDENT_NAMES.with(std::cell::Cell::get);
        assert!(
            scans() > 0 && copied() >= 4,
            "fixture: the Remove line found the four encounters that need its entry"
        );
        assert!(
            words
                .iter()
                .any(|text| text.contains("Bridge 1, Bridge 2, Bridge 3 and 1 more now needs")),
            "fixture: the Remove line names them\n{words:#?}"
        );
        let (scanned, names) = (scans(), copied());
        for _ in 0..4 {
            let _ = reader_words(&mut library, &campaigns);
        }
        assert_eq!(
            scans(),
            scanned,
            "an unchanged frame reads no note for the Remove line"
        );
        assert_eq!(
            copied(),
            names,
            "an unchanged frame copies none of the names that need the removed entry"
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
        let _ = page_words(&mut library, &mut campaigns);
        assert_eq!(
            library.selected,
            Some(1),
            "the reload reselects the campaign's file"
        );
        // Issue #539: the list finds the selected entry's row once; quiet frames search the
        // matches no more.
        let mut frame = || {
            let _ = page_words(&mut library, &mut campaigns);
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
        let _ = page_words(&mut library, &mut campaigns);
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

    /// The words of a footer's lines (`Library::reader_view`, `empty_reader_footer`), as the
    /// pane's footer shows them: each line's words, help, Undo and choices, and its file.
    fn footer_words(lines: &[model::FooterLine<LibraryPress>]) -> Vec<String> {
        let mut words = Vec::new();
        for line in lines {
            words.push(line.words.clone());
            words.extend(line.help.iter().cloned());
            if let Some(undo) = &line.undo {
                words.push(undo.label.clone());
            }
            for choice in &line.choices {
                words.push(choice.words.clone());
                words.extend(choice.help.clone());
            }
            if let Some(file) = &line.file {
                words.push(file.shown.clone());
            }
        }
        words
    }

    /// The words the Library's reader pane shows for the selected note, one part a line, as
    /// the GPUI pane draws its `ReaderView` (`gui::pages::library::reader`): the toolbar, the
    /// owner and file line, the save row (with its long reason), the body's notices, the disk
    /// change and the footer. With no entry open, the empty reader and its footer. Each call is
    /// two frames of the pane.
    fn sd1_painted(library: &mut Library) -> String {
        let campaigns = Campaigns::default();
        let _ = reader_words(library, &campaigns);
        reader_words(library, &campaigns).join("\n")
    }

    /// One frame of `sd1_painted`'s reader pane, with `campaigns`.
    fn reader_words(library: &mut Library, campaigns: &Campaigns) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        {
            let subtitle = library.open_subtitle();
            let Some((view, lines)) = library.reader_view(campaigns, None, &subtitle) else {
                words.push("No entry open".into());
                words.extend(footer_words(&library.empty_reader_footer(campaigns)));
                return words;
            };
            words.push(view.title.clone());
            words.push(view.subtitle.clone());
            let bar = &view.toolbar;
            words.push("Read".into());
            words.push(bar.source_label.into());
            if bar.back {
                words.push("Back".into());
            }
            for (label, ..) in &bar.actions {
                words.push((*label).into());
            }
            words.extend(bar.draft_reason.clone());
            words.extend(bar.reasons.iter().cloned());
            let line = &view.file_line;
            words.push(format!("{} {}", line.status.0, line.status.1));
            if let Some((_, relative)) = &line.file {
                if line.folder_only {
                    if !relative.is_empty() && !line.status.1.contains("deleted") {
                        words.push(relative.clone());
                    }
                    words.push("Show folder".into());
                } else {
                    words.push(relative.clone());
                    words.extend(line.moved_from.clone());
                    if line.obsidian {
                        words.push("Edit in Obsidian".into());
                    }
                    words.push("Show file".into());
                }
            }
            if let Some(row) = &view.save_row {
                words.push(format!("{} {}", row.word.0, row.word.1));
                if row.discard {
                    words.push("Discard".into());
                }
                words.extend(
                    row.reason
                        .as_ref()
                        .map(|(long, _)| long.clone())
                        .or(row.lock.clone()),
                );
                words.push("Save".into());
            }
            for notice in &view.notices {
                words.push(notice.words.clone());
                if let Some((repair, _, help)) = &notice.repair {
                    words.push(repair.clone());
                    words.extend(help.clone());
                }
                if let Some(details) = &notice.details {
                    words.push("Technical details".into());
                    words.push(details.clone());
                }
            }
            if let Some(disk) = &view.disk {
                words.push(disk.words.clone());
                words.push(disk.keep.clone());
                words.push(disk.lose.clone());
                if let Some(details) = &disk.details {
                    words.push("Technical details".into());
                    words.push(details.clone());
                }
            }
            words.extend(footer_words(&lines));
        }
        words
    }

    /// The words of two frames of the whole page, as the GPUI page runs them: `begin`, the
    /// list's work (`list_view`), then the empty Campaign scope's pane or the reader pane.
    fn page_words(library: &mut Library, campaigns: &mut Campaigns) -> String {
        let ctx = egui::Context::default();
        let mut words = Vec::new();
        for _ in 0..2 {
            library.begin(&ctx, campaigns);
            if library.empty_scope() {
                words = vec!["No entries of your own yet".to_owned()];
                words.extend(footer_words(&library.form_footer(campaigns)));
                continue;
            }
            let _ = library.list_view(campaigns, None, false, 1200.0);
            words = reader_words(library, campaigns);
        }
        words.join("\n")
    }

    /// Types `words` at the end of the open Markdown editor's text, as the GPUI editor reports
    /// an edit (`Library::edit_draft`).
    fn sd1_type(library: &mut Library, words: &str) {
        let draft = format!("{}{words}", library.draft.as_str());
        library.edit_draft(&draft);
    }

    /// Row 125: the owner line's status when its words end with `words`: its icon and its
    /// colour, which the icon and the words share.
    fn owner_line_look(library: &mut Library, words: &str) -> Option<(String, egui::Color32)> {
        let subtitle = library.open_subtitle();
        let (view, _) = library.reader_view(&Campaigns::default(), None, &subtitle)?;
        let (icon, said, color) = view.file_line.status;
        said.ends_with(words).then(|| (icon.to_owned(), color))
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
            Some((icons::ERROR.to_owned(), error)),
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
        // Unchanged frames keep the failure and never read the draft's words to do so. Each
        // paint is two frames, and each frame checks.
        let steady = failure_work
            .steady(&mut library, |library| {
                (0..4).all(|_| sd1_painted(library).contains("Not saved"))
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
        sd1_type(&mut library, "More.");
        // The editor's Undo reports the words before the edit.
        library.edit_draft(&accepted);
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
        // Select all, then type the fixed text.
        repair.edit_draft(&fixed);
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
