//! What a campaign uses: package selections, and the repair of invalid notes. This is
//! a rare setup task, so it stays calm, with one scrolling list at 2,000+ rows.
pub use crate::content_workspace::repair_problem;
use crate::state_context::StateContext;
use crate::{
    campaigns::Campaigns,
    content_workspace::{
        CampaignDocument, Catalogue, Closure, EntryRef, PackageSelection, Selection,
    },
    design::{self, icons},
    library::{note_path, write_note},
    persistence::{Campaign, Module},
};
use splat::character::campaign_package::{self, Proposal};
use std::{
    collections::BTreeSet,
    fs,
    sync::mpsc,
    time::{Duration, Instant},
};
use uuid::Uuid;

/// Sheets refresh in the background, so the campaign package needs another look now and then.
const CHECK_EVERY: Duration = Duration::from_secs(5 * 60);

pub struct Snapshot {
    document: CampaignDocument,
    catalogue: Catalogue,
    /// The closure of these selections. It is too slow to calculate on every frame.
    closure: (Vec<PackageSelection>, Closure),
    subtitles: std::collections::HashMap<Uuid, String>,
    /// The issue list's error lines (`issue_errors`), worked out once for this catalogue and
    /// closure: a reload makes a new snapshot, and a new closure drops them (`selection_bar`).
    issues: Option<Vec<String>>,
}

impl Snapshot {
    /// The issue list's error lines for `campaign`, from the kept ones while they stand.
    fn issue_lines(&mut self, campaign: Uuid) -> &[String] {
        let (catalogue, closure) = (&self.catalogue, &self.closure.1);
        self.issues.get_or_insert_with(|| {
            let flags = catalogue
                .entries
                .get(&campaign)
                .map(|entry| entry.entry.flags.as_slice())
                .unwrap_or_default();
            issue_errors(catalogue, closure, flags)
                .into_iter()
                .cloned()
                .collect()
        })
    }
}
/// An invalid note in repair: its path, its text on disk and its draft, with Save's check of the
/// draft. The check runs only where the draft changes (a load, an edit, Discard, Undo discard), so
/// an unchanged frame neither hashes nor parses the text.
pub struct Repair {
    path: String,
    /// The text on disk, with its revision (layout PR 9b-C): `dirty` compares two revisions on
    /// a quiet frame.
    baseline: crate::design::memo::Revised<String>,
    draft: crate::design::text_draft::TextDraft,
    /// The last answer of `dirty`'s compare.
    same: crate::design::text_draft::KeptCompare,
    /// Why Save would refuse `draft`: Save's own check without writing, shared with World.
    refusal: Option<String>,
    /// The line of `draft` to mark.
    error_line: Option<usize>,
    /// A Save of this draft that failed on the disk: Save's reason. Only this repair owns it. It
    /// ends at this repair's next edit, its successful Save, a reload or another campaign.
    failure: Option<String>,
}
impl Repair {
    fn new(path: String, text: String, campaign: &Campaign) -> Self {
        let mut repair = Self {
            path,
            baseline: crate::design::memo::Revised::new(text.clone()),
            draft: crate::design::text_draft::TextDraft::new(text),
            same: crate::design::text_draft::KeptCompare::default(),
            refusal: None,
            error_line: None,
            failure: None,
        };
        repair.check(campaign);
        repair
    }
    /// Ask Save's question of the draft again.
    fn check(&mut self, campaign: &Campaign) {
        (self.refusal, self.error_line) =
            crate::world::repair::refusal(campaign, &self.path, &self.draft);
    }
    /// The draft changed: check it again. A failed Save no longer describes it.
    fn draft_changed(&mut self, campaign: &Campaign, checks: &mut usize) {
        *checks += 1;
        self.check(campaign);
        self.failure = None;
    }
    fn dirty(&self) -> bool {
        !self
            .draft
            .same_as(self.baseline.revision(), &self.baseline, &self.same)
    }
    /// Save's check of the draft as the save row's typed diagnosis: the one value that gives
    /// the save row's `Invalid` and the footer's message of the same fault their tone (R41).
    /// A note that does not parse cannot be read, so it is an error in every place.
    fn diagnosis(&self) -> Option<design::save_row::Diagnosis<'_>> {
        self.refusal
            .as_deref()
            .map(design::save_row::Diagnosis::refused)
    }
    /// Save asks its own question again, then writes. A refusal is Invalid with its cause, never
    /// a failed Save: `Not saved.` belongs only to a write that failed on the disk. True when the
    /// draft was written.
    fn save(&mut self, campaign: &Campaign) -> bool {
        let (refusal, line) = crate::world::repair::refusal(campaign, &self.path, &self.draft);
        if refusal.is_some() {
            (self.refusal, self.error_line) = (refusal, line);
            self.failure = None;
            return false;
        }
        match write_note(
            &campaign.path,
            &self.path,
            Some(&self.baseline),
            &self.draft,
        ) {
            Ok(_) => {
                self.baseline = crate::design::memo::Revised::new(self.draft.as_str().to_owned());
                self.failure = None;
                true
            }
            // The cause stays with this repair, in its state slot.
            Err(reason) => {
                self.failure = Some(reason);
                false
            }
        }
    }
}
type Loaded = (Vec<Repair>, Result<Snapshot, String>);
struct Checked {
    applied: Option<Result<usize, String>>,
    proposals: Result<Vec<Proposal>, String>,
}
/// What the last check of the campaign package found. Nothing moves until the DM applies it.
#[derive(Default)]
struct Reconcile {
    campaign: Option<Uuid>,
    checked_at: Option<Instant>,
    pending: Option<mpsc::Receiver<Checked>>,
    applying: bool,
    proposals: Vec<Proposal>,
    /// Proposals that the DM unticked.
    declined: BTreeSet<String>,
    error: Option<String>,
}

/// The error that says Campaign.md is missing, as `Campaigns` words it.
const MISSING_CAMPAIGN: &str = "Campaign.md is missing";

/// One result on the Campaign page. Each Undo stays with its own object and action; the
/// message space below `Save selections` lists every live one, newest first. A plain result
/// (no Undo) gives way to any newer result.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PageResult {
    CampaignWrite,
    /// A missing package's folder found again, by the package.
    Recovered(Uuid),
    Discarded,
    /// A package manifest written again, by the package.
    ManifestWrite(Uuid),
    Removed,
    /// A repair's discarded draft, by the stamp its Discard took (round 26: one order for every
    /// live Undo on the page, repairs' and the page's alike).
    RepairDiscard(u64),
    Saved,
    Applied,
}

/// Who a Campaign page result belongs to: its campaign, and what it is about. One result per
/// kind and object (a package's manifest write, a repair's discarded draft), so a newer one of
/// the same kind and object replaces it, and a campaign switch hides it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct PageOwner {
    campaign: Uuid,
    kind: PageResult,
}

impl design::results::Owner for PageOwner {
    type Scope = Uuid;
    fn scope(&self) -> &Uuid {
        &self.campaign
    }
}

/// What a kept result's action left: its words and its Undo come from it, and only from it. The
/// store holds it with the result's ticket (one owner), so an Undo acts on the data its own
/// line showed, never on a newer action's.
enum Payload {
    CampaignWrite(crate::content_workspace::ManifestWrite),
    /// The found folder, and the registration it replaced.
    Recovered {
        module: Module,
        previous: Option<Module>,
    },
    /// What `Discard` dropped, and the Campaign.md text it was a draft of.
    Discarded {
        selections: Vec<PackageSelection>,
        baseline: Option<String>,
    },
    ManifestWrite {
        package: Uuid,
        name: String,
        write: crate::content_workspace::ManifestWrite,
    },
    Removed(RemovedPackage),
    /// A repair's discarded draft: its path and words, by the stamp its Discard took.
    RepairDiscard {
        stamp: u64,
        path: String,
        text: String,
    },
    Saved,
    /// How many changes the apply made.
    Applied(usize),
}

impl Payload {
    fn kind(&self) -> PageResult {
        match self {
            Self::CampaignWrite(_) => PageResult::CampaignWrite,
            Self::Recovered { module, .. } => PageResult::Recovered(module.id),
            Self::Discarded { .. } => PageResult::Discarded,
            Self::ManifestWrite { package, .. } => PageResult::ManifestWrite(*package),
            Self::Removed(_) => PageResult::Removed,
            Self::RepairDiscard { stamp, .. } => PageResult::RepairDiscard(*stamp),
            Self::Saved => PageResult::Saved,
            Self::Applied(_) => PageResult::Applied,
        }
    }
}

/// A kept Campaign page result: its campaign and what its action left. Its words come from its
/// own payload; it lives until an end event of its own ends it.
struct PageKept {
    campaign: Uuid,
    payload: Payload,
}

impl design::results::Kept for PageKept {
    type Owner = PageOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // A newer action of another kind ends a plain result (`new_result!`), and the page
        // ends a result whose state went (`end_results`).
        owner_changed: design::results::OwnerChanged::Ends,
        // A reload checks each result against the files (`end_results` at the load).
        disk_changed: design::results::DiskChanged::Stays,
        // Its Undo leaves the store only when it works (`undo_by`).
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> PageOwner {
        PageOwner {
            campaign: self.campaign,
            kind: self.payload.kind(),
        }
    }
}

/// The removal result of `campaign`, if it has one.
fn removal_of(
    results: &design::Results<PageOwner, PageKept>,
    campaign: Uuid,
) -> Option<&RemovedPackage> {
    let owner = PageOwner {
        campaign,
        kind: PageResult::Removed,
    };
    results
        .get(&owner)
        .and_then(|shown| match &shown.value().payload {
            Payload::Removed(removed) => Some(removed),
            _ => None,
        })
}

/// End the results of one campaign whose payload `ends` names.
fn end_results(
    results: &mut design::Results<PageOwner, PageKept>,
    campaign: Uuid,
    ends: impl Fn(&Payload) -> bool,
) {
    let owners: Vec<PageOwner> = results
        .newest_first(&campaign)
        .map(|shown| shown.value())
        .filter(|kept| ends(&kept.payload))
        .map(design::results::Kept::owner)
        .collect();
    for owner in owners {
        results.owner_changed(&owner);
    }
}

/// A new result comes first, with the order its store issues at the action, and with its
/// payload. It ends the plain results, never another action's Undo. A macro, so it runs while
/// other fields of the page are borrowed.
macro_rules! new_result {
    ($page:expr, $payload:expr) => {
        let campaign = $page
            .active
            .as_ref()
            .map_or(Uuid::nil(), |active| active.id);
        for plain in [PageResult::Saved, PageResult::Applied] {
            $page.results.owner_changed(&PageOwner {
                campaign,
                kind: plain,
            });
        }
        $page.results.record(PageKept {
            campaign,
            payload: $payload,
        });
        // A newer act's answer: a failed one is no longer the answer to the last act.
        $page.answer = None;
    };
}

/// `new_result!` as a function, for the shared acts (`view`).
fn new_result_of(page: &mut CampaignContent, payload: Payload) {
    new_result!(page, payload);
}

/// What the DM pressed in the page's footer: a result's Undo (its ticket names the result), an
/// encounter that a removal left without creatures, or the write of a missing Campaign.md.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentPress {
    Undo,
    WriteCampaign,
    /// A broken manifest's choices, by its package.
    OpenManifest(Uuid),
    WriteManifest(Uuid),
    /// An unavailable package's choices, by its place among the campaign's selections.
    UseFolder(usize),
    Register(usize),
    Find(usize),
    Remove(usize),
    /// An encounter: one that uses an unavailable package's creatures, or one that a removal
    /// left without creatures.
    OpenEntry(Uuid),
    /// An unavailable package's details link: it shows or hides the package's reference.
    Details(usize),
}

/// What the Campaign page says is wrong, in its footer or its body. The tone comes from the
/// severity, as everywhere.
enum ContentFault<'a> {
    /// Save's or a load's error, or the change check's (the system's or the workspace's words).
    Error(&'a str),
    /// Campaign.md is missing: what Splat can write (the campaigns' preview words). A missing
    /// file is a warning (R41), as the missing manifests and folders are.
    MissingCampaign(&'a str),
    /// A note the page needs cannot be read; `problem` is Save's refusal or that the repair is
    /// not saved, while the repair is closed.
    Unreadable {
        path: &'a str,
        problem: Option<&'a str>,
    },
    /// A repair's Save failed on the disk.
    RepairNotSaved { path: &'a str, cause: &'a str },
    /// Save's own check refuses an open repair's draft: `found`, the diagnosis that also gives
    /// the repair's save row its word, so both say the fault in one tone (R41).
    RepairRefused {
        path: &'a str,
        found: &'a design::save_row::Diagnosis<'a>,
    },
    /// A package manifest that cannot be used; `on_disk`: it is there but broken (the error
    /// tone), else it is gone (a warning).
    Manifest {
        package: &'a str,
        problem: &'a str,
        on_disk: bool,
    },
    /// A chosen package that is not available, and where it was.
    Unavailable {
        message: &'a str,
        it_was: Option<&'a str>,
        /// The encounters that use its creatures (`A and B`), and how many.
        used_by: Option<(&'a str, usize)>,
        /// The package's reference, while its Technical details are open.
        reference: Option<&'a str>,
    },
    /// Encounters a package removal left without creatures.
    LeftWithout(&'a design::outcome::Sentence),
}

impl design::outcome::Fault for ContentFault<'_> {
    fn severity(&self) -> design::outcome::Severity {
        use design::outcome::Severity;
        match self {
            Self::Error(_) | Self::Unreadable { .. } => Severity::CannotRead,
            Self::RepairNotSaved { .. } => Severity::NotSaved,
            Self::RepairRefused { found, .. } => match found.tone() {
                Some(design::Tone::Error) => Severity::CannotRead,
                _ => Severity::NeedsFixing,
            },
            Self::Manifest { on_disk: true, .. } => Severity::CannotRead,
            Self::MissingCampaign(_)
            | Self::Manifest { on_disk: false, .. }
            | Self::Unavailable { .. }
            | Self::LeftWithout(_) => Severity::NeedsFixing,
        }
    }

    fn place(&self) -> Option<design::outcome::FaultPlace<'_>> {
        match self {
            Self::RepairRefused { path, .. } => Some(design::outcome::FaultPlace {
                name: (*path).into(),
                line: None,
            }),
            _ => None,
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "errors, refusals and manifest problems carry the system’s, the workspace’s or the content layer’s own words"
    )]
    fn cause(&self) -> design::outcome::Sentence {
        use design::outcome::Sentence;
        // `Sentence::outside` makes the first letter a capital. Words that start with a small
        // letter most often start with a path or a file name (`notes/a.md: …`, `package.md
        // changed on disk.`), so checked words come first and the path keeps its spelling.
        let outside = |words: &str| {
            let words = words.trim();
            if words.chars().next()?.is_lowercase() {
                Sentence::outside(format!("Problem: {words}"))
            } else {
                Sentence::outside(words)
            }
        };
        match self {
            Self::Error(error) => {
                outside(error).unwrap_or(crate::sentence!("Something went wrong."))
            }
            Self::MissingCampaign(preview) => {
                outside(preview).unwrap_or(crate::sentence!("Campaign.md is missing."))
            }
            Self::Unreadable { path, problem } => {
                let first = crate::sentence!("{} cannot be read. Fix it here or in World.", path);
                match problem.and_then(outside) {
                    Some(problem) => first.then(problem),
                    None => first,
                }
            }
            Self::RepairNotSaved { path, cause } => {
                let first = crate::sentence!("{} is not saved.", path);
                match outside(cause) {
                    Some(cause) => first.then(cause),
                    None => first,
                }
            }
            Self::RepairRefused { found, .. } => found
                .blocking()
                .and_then(&outside)
                .unwrap_or(crate::sentence!("This needs fixing.")),
            // What Replace keeps and replaces is its choice's own help line (ruling M8).
            Self::Manifest {
                package, problem, ..
            } => outside(&manifest_message(package, problem))
                .unwrap_or(crate::sentence!("The package manifest cannot be used.")),
            Self::Unavailable {
                message,
                it_was,
                used_by,
                reference,
            } => {
                let mut words =
                    outside(message).unwrap_or(crate::sentence!("This package is not available."));
                if let Some(folder) = it_was {
                    words = words.then(crate::sentence!("It was in {}/.", folder));
                }
                words = match used_by {
                    Some((names, 1)) => {
                        words.then(crate::sentence!("{} uses its creatures.", names))
                    }
                    Some((names, _)) => {
                        words.then(crate::sentence!("{} use its creatures.", names))
                    }
                    None => words,
                };
                match reference {
                    Some(reference) => {
                        words.then(crate::sentence!("Package reference: {}.", reference))
                    }
                    None => words,
                }
            }
            Self::LeftWithout(words) => (*words).clone(),
        }
    }
}

#[derive(Default)]
pub struct CampaignContent {
    /// The unavailable packages whose Technical details are open, by their reference.
    details: BTreeSet<String>,
    /// The page's kept results, each with the order its action took (newest first) and what
    /// its action left (`Payload`): the one owner of each result's words and Undo.
    results: design::Results<PageOwner, PageKept>,
    /// How often the page prepared a package removal's words: once per removal.
    removal_text_preparations: usize,
    reconcile: Reconcile,
    active: Option<Campaign>,
    pending: Option<mpsc::Receiver<Loaded>>,
    snapshot: Option<Snapshot>,
    saved_selections: Vec<PackageSelection>,
    module: Option<Uuid>,
    dependencies: bool,
    notice_scope: Option<(Option<Uuid>, bool)>,
    search: String,
    entry_matches: Option<(Uuid, String, Vec<Uuid>)>,
    /// The DM's choice of list. Until they choose, a package opens on the list it needs.
    pub run_open: bool,
    pub open_lookup: Option<Uuid>,
    pub open_run_settings: bool,
    pub found_package: Option<Module>,
    pub forget_package: Option<Uuid>,
    /// Issue #509: this frame wrote the open campaign's package choices (Save selections,
    /// Remove from this campaign, Undo of a removal). Campaign.md written again from Splat's copy,
    /// and its Undo, recover the file and reload this page only, as before.
    /// The app takes it after the page (`Splat::route`) and reads Lookup and Encounters again,
    /// so their searches follow what the campaign uses now.
    pub choices_written: bool,
    error: Option<String>,
    /// The newest failed answer to an act of the DM (ruling Q-x): a refused folder pick, a
    /// failed Undo or write, a repair's failed Save. First in the footer. See [`Answer`] for its
    /// owner and ends.
    answer: Option<Answer>,
    disabled_selections: std::collections::BTreeMap<Uuid, Selection>,
    refresh_pending: bool,
    repairs: Vec<Repair>,
    /// How often a frame checked a repair draft again. Only an edit, Discard or Undo discard does.
    repair_checks: usize,
    /// The next stamp for a repair's Discard. Round 26: one result per repair, so a second
    /// repair's Discard keeps the first one's Undo.
    repair_stamp: u64,
    /// The repair the DM edited or focused last. It owns the primary Save while it is open and
    /// changed; other changed repairs keep their drafts with a secondary Save.
    active_repair: Option<String>,
}
/// The rule: the answer is the result of the LAST act; any later act, failed or successful,
/// replaces or ends it. A failed one is held here; a successful one ends it (its own result, if
/// it has one, is a kept result). The newest failed answer to an act of the DM, in one campaign:
/// the footer's one ordering rule is that it comes first, before the kept results, which are
/// older answers. A newer act
/// ends or replaces it: a new result (`new_result!`), the next press of a package choice, another
/// failure (`Self::fail`), the package found again (`package_restored`). A load of the page and a
/// change of campaign (`forget_discards_of_other_campaigns`, also while the page is hidden) end
/// it too. It shows only while its cause stands: a refused pick while its package is missing, a
/// repair's failure while that repair still has it.
struct Answer {
    campaign: Uuid,
    cause: Cause,
    words: String,
}

/// The newest failed answer of an act in the `active` campaign.
fn answer_in(active: &Option<Campaign>, cause: Cause, words: String) -> Option<Answer> {
    Some(Answer {
        campaign: active.as_ref().map_or(Uuid::nil(), |active| active.id),
        cause,
        words,
    })
}

enum Cause {
    /// A folder pick that Splat refused (`Find the folder…`, `Register it again`): the folder
    /// holds another package, or none. The package, by its reference's label (as `details`
    /// keeps it).
    Refused { package: String },
    /// An Undo, a write or a removal that failed.
    Failed,
    /// A repair's Save that failed, by the repair's path; the repair keeps the failure as its
    /// state, and the footer says it here while this is the newest answer.
    NotSaved { path: String },
}

struct RemovedPackage {
    /// The removed package's name, for its Undo (`Undo remove <name>`).
    name: String,
    /// The result's words, prepared once when the removal succeeds.
    words: design::outcome::Sentence,
    /// The warning's words after the result's, when encounters lose creatures; prepared with
    /// `words`.
    warning: Option<design::outcome::Sentence>,
    before: Vec<PackageSelection>,
    after: Vec<PackageSelection>,
    users: Vec<(Uuid, String)>,
}

/// A folder recovery must keep the selected package's identity. `name` is the package the DM
/// looks for, as the campaign names it.
fn find_package_folder(
    reference: &EntryRef,
    name: Option<&str>,
    folder: &std::path::Path,
) -> Result<Module, String> {
    let module = crate::library::local_module(folder)?;
    let matches = if let Some(id) = reference.id {
        id == module.id
    } else {
        let catalogue = Catalogue::load(std::slice::from_ref(&module), None)?;
        catalogue.resolve_ref(reference).ok() == Some(module.id)
    };
    if !matches {
        // The fault first, then what to do, and both things by name.
        let picked = folder.file_name().map_or_else(
            || folder.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        return Err(format!(
            "The folder {picked}/ holds a different package. Choose the folder that holds {}.",
            name.unwrap_or("this package")
        ));
    }
    Ok(module)
}

fn relocated_package(module: &Module, registered: &[Module]) -> Option<Module> {
    let parents: BTreeSet<_> = std::iter::once(module)
        .chain(registered)
        .filter_map(|module| module.path.parent())
        .collect();
    let mut found = std::collections::BTreeMap::new();
    for parent in parents {
        for folder in fs::read_dir(parent).ok().into_iter().flatten().flatten() {
            if folder.file_name().to_string_lossy().starts_with('.') {
                continue;
            }
            if let Ok(candidate) = crate::library::local_module(&folder.path())
                && candidate.id == module.id
                && candidate.path != module.path
            {
                found.insert(candidate.path.clone(), candidate);
            }
        }
    }
    (found.len() == 1).then(|| found.into_values().next().unwrap())
}

/// A package removal's result words, and the warning's words when the removal leaves encounters
/// without creatures. Round 27 FE5: the result is a success sentence in every case; the warning
/// follows it. Prepared once, when the removal succeeds.
fn removal_words(
    name: &str,
    users: &[(Uuid, String)],
) -> (design::outcome::Sentence, Option<design::outcome::Sentence>) {
    let warning = (!users.is_empty()).then(|| {
        let names = and_list(users.iter().map(|(_, name)| name.as_str()));
        if users.len() == 1 {
            crate::sentence!("{} now has creatures Splat cannot find.", names)
        } else {
            crate::sentence!("{} now have creatures Splat cannot find.", names)
        }
    });
    (
        crate::sentence!("{} removed from this campaign.", name),
        warning,
    )
}

/// Names as one phrase: `A`, `A and B`, `A, B and C`.
fn and_list<'a>(names: impl Iterator<Item = &'a str>) -> String {
    let names: Vec<&str> = names.collect();
    match names.split_last() {
        None => String::new(),
        Some((last, [])) => (*last).to_owned(),
        Some((last, before)) => format!("{} and {last}", before.join(", ")),
    }
}

/// An apply's result: how many changes it made, and where the old notes are.
fn applied_words(count: usize) -> design::outcome::Sentence {
    if count == 1 {
        crate::sentence!("1 change applied. The old note is archived beside its path.")
    } else {
        crate::sentence!(
            "{} applied. The old notes are archived beside their paths.",
            design::count(count, "change", "changes")
        )
    }
}

/// A faulty manifest's warning, in the page's footer.
fn manifest_message(package: &str, problem: &str) -> String {
    format!(
        "{package}: {problem}{}",
        if problem == crate::content_workspace::DELETED_MANIFEST {
            ""
        } else {
            ". Entries are still available."
        }
    )
}

/// A chosen package that is not available, and what Splat knows about where it went.
struct Unavailable {
    index: usize,
    selection: PackageSelection,
    registered: Option<Module>,
    known: Option<Module>,
    unregistered: bool,
    relocated: Option<Module>,
    message: String,
}

/// The page's package warnings, gathered once a frame (round 26 FD1).
#[derive(Default)]
struct PackageWarnings {
    unavailable: Vec<Unavailable>,
}

/// A package row's count, in one form: `All chosen · 2,065 entries`, `None chosen · 6 entries`,
/// `3 of 6 chosen`. Round 27 FA6: a ticked box counts as chosen, whoever ticked it, so an entry
/// that other choices need (ticked and locked) counts too; the list `Needed by your entries`
/// says which they are. `ticked` is the DM's choices and the needed entries together.
/// `indeterminate` is the package box's own state: a box that is not fully ticked never says
/// `All chosen`.
fn package_count(ticked: usize, total: usize, indeterminate: bool) -> String {
    let entries = || {
        if total == 0 {
            String::new()
        } else {
            format!(" · {}", design::count(total, "entry", "entries"))
        }
    };
    let ticked = ticked.min(total);
    if ticked == 0 {
        format!("None chosen{}", entries())
    } else if ticked == total && !indeterminate {
        format!("All chosen{}", entries())
    } else {
        design::outcome::chosen_count(ticked, total, "")
    }
}

/// Whether a package's faulty manifest is still on the disk: then it cannot be read, an error;
/// a deleted one is a warning. The Campaign message and the package row ask the same question.
fn manifest_on_disk(module: &Module, path: &str, problem: &str) -> bool {
    problem != crate::content_workspace::DELETED_MANIFEST && module.path.join(path).exists()
}

/// The mark that leads a package row's second line when its manifest has a fault: the fault's
/// icon, drawn in its tone, and the words its accessible name says in place of the icon. The
/// row is too narrow for the words beside the count; the message above names the fault.
fn manifest_mark(on_disk: bool) -> (&'static str, &'static str) {
    if on_disk {
        (icons::ERROR, "Manifest cannot be read")
    } else {
        (icons::WARN, "Manifest deleted")
    }
}

/// A failed Undo, as the page's error: checked words first, so a path at the start of the
/// cause (`package.md changed on disk.`) keeps its spelling (`Sentence::outside` would make its
/// first letter a capital).
fn undo_failed(cause: &str) -> String {
    format!("Undo did not work: {}", cause.trim())
}

/// One kept result as the footer says it, from its own payload: its words, and its Undo. While a
/// draft waits, a manifest write's Undo is locked (`locked`): the write happened, so its words
/// stay.
#[allow(
    clippy::disallowed_methods,
    reason = "a manifest write’s words are the content workspace’s own (`ManifestWrite::message`)"
)]
fn outcome_of(payload: &Payload, locked: bool) -> design::outcome::KeptOutcome<ContentPress> {
    use design::outcome::{Done, KeptOutcome, Notice, Said, Sentence, UndoStep, Verb};
    let undo = |verb: Verb, object: &str| {
        UndoStep::of(verb, object.to_owned()).pressed_as(ContentPress::Undo)
    };
    let discarded = |name: &str| {
        KeptOutcome::Notice(
            Notice::new(crate::sentence!("{} draft discarded.", name)),
            undo(Verb::Discard, name),
        )
    };
    match payload {
        Payload::CampaignWrite(_) => Said::new(crate::sentence!("Campaign.md was written again."))
            .undo(undo(Verb::Write, "Campaign.md"))
            .into(),
        // One verb for the result and its Undo (M0003): `Watch recovered in packages/watch/.`,
        // its Undo told apart by the same folder.
        Payload::Recovered { module, .. } => Done::new(Verb::Recover, module.name.clone())
            .object_first()
            .within(format!("{}/", folder_words(&module.path)))
            .undo(ContentPress::Undo)
            .into(),
        Payload::Discarded { .. } => discarded("Campaign packages"),
        Payload::RepairDiscard { path, .. } => discarded(path.rsplit('/').next().unwrap_or(path)),
        Payload::ManifestWrite { name, write, .. } => Said::new(
            Sentence::outside(format!("{name}: {}", write.message()))
                .unwrap_or(crate::sentence!("The manifest was written again.")),
        )
        .undo(undo(Verb::Write, write.path()).locked_when(
            locked,
            crate::sentence!("Save or discard campaign changes first."),
        ))
        .into(),
        Payload::Removed(removed) => Said::new(removed.words.clone())
            .undo(undo(Verb::Remove, &removed.name))
            .into(),
        Payload::Saved => Said::new(crate::sentence!("Campaign selections saved.")).into(),
        Payload::Applied(count) => Said::new(applied_words(*count)).into(),
    }
}

fn folder_words(path: &std::path::Path) -> String {
    path.parent()
        .and_then(|parent| parent.file_name())
        .map_or_else(
            || path.display().to_string(),
            |parent| {
                format!(
                    "{}/{}",
                    parent.to_string_lossy(),
                    path.file_name().unwrap_or_default().to_string_lossy()
                )
            },
        )
}

impl CampaignContent {
    pub fn package_restored(&mut self, result: Result<(), String>) {
        match result {
            Ok(()) => {
                self.refresh_pending = true;
                self.error = None;
                self.answer = None;
            }
            Err(error) => self.fail(Cause::Failed, error),
        }
    }

    /// An act failed: its words are the newest answer, first in the footer, in place of any
    /// older one.
    fn fail(&mut self, cause: Cause, words: String) {
        self.answer = answer_in(&self.active, cause, words);
    }

    /// A failed act of the page's content (write Campaign.md again, create the campaign folder):
    /// with no package list, the page's errors are its content (round 27 FB3), so the failure
    /// takes that place and ends an older answer; else it is the newest answer in the footer.
    fn fail_or_content(&mut self, words: String) {
        if self.body_missing() {
            self.error = Some(words);
            self.answer = None;
        } else {
            self.fail(Cause::Failed, words);
        }
    }
    /// How often a frame checked a repair draft again.
    #[cfg(any(test, feature = "test-support"))]
    pub fn repair_checks(&self) -> usize {
        self.repair_checks
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn busy(&self) -> bool {
        self.pending.is_some() || self.reconcile.pending.is_some()
    }

    /// Whether the package selection differs from the saved one, for tests.
    #[cfg(any(test, feature = "test-support"))]
    pub fn selections_changed(&self) -> bool {
        self.snapshot
            .as_ref()
            .is_some_and(|s| s.document.selections != self.saved_selections)
    }

    pub fn dirty(&self) -> bool {
        self.repairs.iter().any(Repair::dirty)
            || self
                .snapshot
                .as_ref()
                .is_some_and(|s| s.document.selections != self.saved_selections)
    }
    pub fn context(&self) -> String {
        self.snapshot.as_ref().map_or_else(String::new, |snapshot| {
            let chosen = snapshot
                .closure
                .1
                .direct
                .iter()
                .filter(|id| snapshot.catalogue.selectable(**id))
                .count();
            let needed = snapshot
                .closure
                .1
                .required
                .keys()
                .filter(|id| {
                    !snapshot.closure.1.direct.contains(id)
                        && snapshot.catalogue.selectable(**id)
                        && snapshot.catalogue.entries.get(id).is_some_and(|entry| {
                            snapshot
                                .catalogue
                                .modules
                                .iter()
                                .any(|module| !module.private && module.id == entry.owner)
                        })
                })
                .count();
            // Round 27 FA6: a ticked box counts as chosen, whoever ticked it, as each package
            // row counts it.
            design::count(chosen + needed, "entry chosen", "entries chosen")
        })
    }

    /// The page wrote the open campaign's package choices: the check is due again, and the app
    /// reads Lookup and Encounters again (`choices_written`).
    fn wrote_choices(&mut self) {
        self.reconcile.checked_at = None;
        self.choices_written = true;
    }

    /// Runs on every frame, on any page, so the sidebar can say when the DM has moves to review.
    pub fn tick(&mut self, ctx: &StateContext, campaigns: &Campaigns) {
        let active = campaigns.active_campaign();
        self.finish_check(active.as_ref().map(|campaign| campaign.id));
        // Round 26: a campaign change ends the Undos of the campaign that was open, at the
        // change, even while this page is hidden; a page reload is not the only end.
        self.forget_discards_of_other_campaigns(active.as_ref());
        if cfg!(any(test, feature = "test-support")) {
            return;
        }
        let Some(active) = active else {
            return;
        };
        if self.check_due(active.id) {
            self.start_check(ctx, campaigns, active.clone(), None);
        }
    }

    /// The one place that changes the check's campaign. Its stored data (proposals, declined
    /// choices, error, the time of the last check) belongs to that campaign and ends when the
    /// owner changes, whether or not a check is running.
    fn set_check_owner(&mut self, owner: Option<Uuid>) {
        if self.reconcile.campaign != owner {
            self.reconcile.proposals.clear();
            self.reconcile.declined.clear();
            self.reconcile.error = None;
            self.reconcile.checked_at = None;
        }
        self.reconcile.campaign = owner;
    }

    /// An Apply runs: its writes are under way, so the campaign switch waits for it
    /// (`Splat::blocked_switch`) and it is never cancelled.
    pub fn applying(&self) -> bool {
        self.reconcile.pending.is_some() && self.reconcile.applying
    }

    /// Whether `tick` starts a check for the app's campaign `active`: none is running, and the
    /// last one was for another campaign or is old.
    fn check_due(&self, active: Uuid) -> bool {
        let due = self.reconcile.campaign != Some(active)
            || self
                .reconcile
                .checked_at
                .is_none_or(|at| at.elapsed() >= CHECK_EVERY);
        due && self.reconcile.pending.is_none()
    }
    /// Words for the sidebar, when the campaign package has something to review.
    pub fn review_detail(&self) -> Option<String> {
        let count = self.reconcile.proposals.len();
        (count > 0).then(|| design::count(count, "to review", "to review"))
    }
    /// The check (and its Apply) that `start_check` started, when it completes. It belongs to the
    /// campaign that started it (`reconcile.campaign`); a completion after the DM opened another
    /// campaign (`active`) is dropped whole, its proposals, its result and its failure, so it is
    /// never shown in the other campaign; the next check runs for the campaign open then.
    fn finish_check(&mut self, active: Option<Uuid>) {
        let Some(rx) = &self.reconcile.pending else {
            return;
        };
        let checked = match rx.try_recv() {
            Ok(checked) => checked,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => Checked {
                applied: None,
                proposals: Err("The campaign package check stopped.".into()),
            },
        };
        self.reconcile.pending = None;
        self.reconcile.applying = false;
        let owner = self.reconcile.campaign;
        if owner != active {
            // Not this campaign's: the check is due again for the one open now.
            self.set_check_owner(None);
            return;
        }
        self.reconcile.checked_at = Some(Instant::now());
        self.reconcile.error = None;
        match checked.proposals {
            Ok(proposals) => {
                self.reconcile.proposals = proposals;
            }
            Err(error) => self.reconcile.error = Some(error),
        }
        match checked.applied {
            Some(Ok(count)) => {
                new_result!(self, Payload::Applied(count));
                self.refresh_pending = !self.dirty();
            }
            // The DM's Apply failed: the newest answer (ruling Q-x), in the campaign that
            // started it.
            Some(Err(error)) => {
                self.answer = Some(Answer {
                    campaign: owner.unwrap_or(Uuid::nil()),
                    cause: Cause::Failed,
                    words: error,
                });
            }
            None => {}
        }
    }
    /// With `keys`, carries out those proposals first. The check itself changes nothing.
    fn start_check(
        &mut self,
        ctx: &StateContext,
        campaigns: &Campaigns,
        campaign: Campaign,
        keys: Option<BTreeSet<String>>,
    ) {
        let (tx, rx) = mpsc::channel();
        self.set_check_owner(Some(campaign.id));
        self.reconcile.pending = Some(rx);
        self.reconcile.applying = keys.is_some();
        let modules = campaigns.modules().to_vec();
        let ctx = ctx.waker();
        std::thread::spawn(move || {
            let destinations = crate::content_workspace::destinations(&campaign.path, &modules);
            let records = campaign.dndbeyond_campaign_id.as_ref().map_or_else(
                || Ok(Vec::new()),
                |remote| {
                    crate::hermetic::character_cache()
                        .and_then(|cache| cache.load_all(campaign.id, remote))
                },
            );
            let applied = keys.map(|keys| {
                records.as_ref().map_err(Clone::clone).and_then(|records| {
                    campaign_package::apply(&campaign.path, &destinations, records, &keys)
                })
            });
            let proposals = records.and_then(|records| {
                campaign_package::check(&campaign.path, &destinations, &records)
            });
            let _ = tx.send(Checked { applied, proposals });
            ctx.request_repaint();
        });
    }
    pub fn refresh_destination(&mut self, _root: &std::path::Path) {
        // A package write can also save this campaign's selection (Use in this campaign).
        if !self.dirty() && self.active.is_some() {
            self.refresh_pending = true;
        }
    }
    /// Discarded work belongs to its campaign. Undo must not save it into another: a change of
    /// campaign ends the open campaign's discards, removal and Campaign.md write.
    fn forget_discards_of_other_campaigns(&mut self, campaign: Option<&Campaign>) {
        // A failed answer is the answer to an act in its campaign: a change of campaign ends it.
        if self
            .answer
            .as_ref()
            .is_some_and(|answer| campaign.is_none_or(|campaign| campaign.id != answer.campaign))
        {
            self.answer = None;
        }
        // Two owners: the loaded page's campaign (`self.active`, set by `reload`, which stays while
        // the page is hidden) and the running check's campaign (`reconcile.campaign`, set by
        // `start_check`). A check (or an Apply) still running is cancelled only when the app's
        // campaign is not ITS campaign: once per change, also when the DM comes back before it
        // completes (A -> B -> A); a check started for the app's campaign is never cancelled.
        // The check's stored data (its proposals, declined choices, error and time) is its
        // campaign's too: it ends with it, also when no check runs any more. A running Apply is
        // never cancelled: the campaign switch waits for it (`applying`, the switch lock).
        if !self.applying()
            && self.reconcile.campaign.is_some()
            && campaign.map(|campaign| campaign.id) != self.reconcile.campaign
        {
            self.reconcile.pending = None;
            self.reconcile.applying = false;
            self.set_check_owner(None);
        }
        if let Some(old) = &self.active
            && campaign.is_none_or(|campaign| campaign.id != old.id)
        {
            end_results(&mut self.results, old.id, |payload| {
                matches!(
                    payload,
                    Payload::Discarded { .. }
                        | Payload::RepairDiscard { .. }
                        | Payload::Removed(_)
                        | Payload::CampaignWrite(_)
                )
            });
        }
    }
    /// Undo package removal checks the package choices only. If the file's choices changed since
    /// the removal, that Undo would refuse: the result and its Undo go. Only the removal of the
    /// file's own campaign is judged.
    fn forget_changed_removal(&mut self, document: &crate::content_workspace::CampaignDocument) {
        end_results(
            &mut self.results,
            document.campaign.id,
            |payload| matches!(payload, Payload::Removed(removed) if removed.after != document.selections),
        );
    }
    /// `Undo discard` of the package choices belongs to Campaign.md as it was. When the file goes
    /// missing or changes on disk, the discarded draft and its Undo go.
    fn forget_discard_of_changed_file(
        &mut self,
        campaign: Uuid,
        loaded: Result<&CampaignDocument, &String>,
    ) {
        end_results(&mut self.results, campaign, |payload| {
            matches!(payload, Payload::Discarded { baseline, .. }
                if !loaded.is_ok_and(|document| document.baseline == *baseline))
        });
    }
    /// A plain result's state went (a new draft, a reload): its line goes.
    fn end_plain(&mut self, plain: PageResult) {
        if let Some(active) = &self.active {
            end_results(&mut self.results, active.id, |payload| {
                payload.kind() == plain
            });
        }
    }
    fn reload(&mut self, ctx: &StateContext, campaigns: &Campaigns, campaign: Campaign) {
        self.forget_discards_of_other_campaigns(Some(&campaign));
        // A write whose file changed since is stale: its Undo would only refuse. A switch back
        // to a campaign reloads it, so its own writes are checked here.
        end_results(&mut self.results, campaign.id, |payload| match payload {
            Payload::CampaignWrite(write) | Payload::ManifestWrite { write, .. } => {
                !write.on_disk()
            }
            _ => false,
        });
        self.refresh_pending = false;
        self.reconcile.checked_at = None;
        self.disabled_selections.clear();
        self.repairs.clear();
        let modules = campaigns.modules().to_vec();
        self.active = Some(campaign.clone());
        self.snapshot = None;
        self.entry_matches = None;
        self.error = None;
        self.answer = None;
        self.end_plain(PageResult::Saved);
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.waker();
        std::thread::spawn(move || {
            let _ = tx.send(load_snapshot(&campaign, &modules));
            ctx.request_repaint();
        });
    }
    pub fn clear_notice(&mut self) {
        self.end_plain(PageResult::Saved);
    }

    /// A load that came in: its repairs, its snapshot or its error (`show`'s step).
    fn take_load(&mut self, campaigns: &Campaigns, active: &Campaign) {
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok((repairs, result)) => {
                    self.pending = None;
                    self.repairs = repairs;
                    // A repair's discard result ends with its repair: Saved, fixed in World or
                    // deleted. It lives through a reload that finds the repair again.
                    let repairs = &self.repairs;
                    let campaign = active.id;
                    end_results(&mut self.results, campaign, |payload| {
                        matches!(payload, Payload::RepairDiscard { path, .. }
                            if !repairs.iter().any(|repair| &repair.path == path))
                    });
                    self.forget_discard_of_changed_file(
                        campaign,
                        result.as_ref().map(|s| &s.document),
                    );
                    match result {
                        Ok(snapshot) => {
                            campaigns.remember_campaign_document(&snapshot.document);
                            self.saved_selections = snapshot.document.selections.clone();
                            self.forget_changed_removal(&snapshot.document);
                            if self.module.is_none() {
                                self.dependencies = true;
                            }
                            self.snapshot = Some(snapshot);
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Loading stopped. Reload to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }

    /// The page's one message footer (round 26), made before the body. Ruling Q-x: the answer to
    /// the DM's last act comes first. So the newest failed answer (`Answer`) leads, then the kept
    /// results (one with a live Undo first, newest first, as the store's orders say), the
    /// warning about encounters a removal left without creatures, each repair's failed Save or
    /// refusal, the page's errors, and last the package faults, which come from the files and
    /// not from an act. Each keeps its owner and end event: a repair's message names its file
    /// and ends with that repair's state; a kept result lives while its state does (`live`) and
    /// in its own campaign only.
    #[allow(
        clippy::disallowed_methods,
        reason = "ruling Q-x: the answer to the DM’s last act leads the footer (`Ban::Answer`)"
    )]
    fn page_footer_with<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        repair_open: &dyn Fn(&str) -> bool,
        campaigns: &Campaigns,
        active: &Campaign,
    ) -> design::outcome::Messages<'f, ContentPress> {
        use design::outcome::{Choice, ChoiceKind, FaultMessage, MessageChoice};
        let mut messages = design::outcome::Messages::new();
        let locked = self.dirty();
        let warnings = self.package_warnings_of(campaigns);
        // Ruling Q-x: the newest failed answer first, while its cause stands. A repair's failure
        // shown here is not said again below.
        let answer = self.answer.as_ref().filter(|answer| {
            answer.campaign == active.id
                && match &answer.cause {
                    Cause::Refused { package } => warnings
                        .unavailable
                        .iter()
                        .any(|missing| missing.selection.package.label() == *package),
                    Cause::Failed => true,
                    Cause::NotSaved { path } => self.repairs.iter().any(|repair| {
                        &repair.path == path && repair.failure.as_ref() == Some(&answer.words)
                    }),
                }
        });
        let first_failure = answer.and_then(|answer| match &answer.cause {
            Cause::NotSaved { path } => Some(path.as_str()),
            _ => None,
        });
        if let Some(answer) = answer {
            let fault = match &answer.cause {
                Cause::NotSaved { path } => FaultMessage::new(&ContentFault::RepairNotSaved {
                    path,
                    cause: &answer.words,
                }),
                Cause::Refused { .. } | Cause::Failed => {
                    FaultMessage::new(&ContentFault::Error(&answer.words))
                }
            };
            // The one counted use (`Ban::Answer`, exactly one call).
            messages.answer(fault);
        }
        for shown in self.results.newest_first(&active.id) {
            let line = shown.outcome(frame, |kept| outcome_of(&kept.payload, locked));
            messages.kept(line);
        }
        // Round 27 FE5: the encounters a removal leaves without creatures, each a way to open it.
        if let Some((removed, warning)) = self
            .removal(active.id)
            .and_then(|removed| removed.warning.as_ref().map(|warning| (removed, warning)))
        {
            let mut fault = FaultMessage::new(&ContentFault::LeftWithout(warning));
            for (id, name) in &removed.users {
                fault = fault.choice(Choice::named_as(
                    ChoiceKind::Safe,
                    crate::choice!("Open {}", name),
                    ContentPress::OpenEntry(*id),
                ));
            }
            messages.outcome(fault);
        }
        // A failed Save or a refusal belongs to its repair: its words name the file (round 27
        // FB5: two notes can share a name in different folders). Round 27 FB1, AB2: a refusal's
        // cause shows once. While the repair is closed the message above says it; only an open
        // repair's message drops it, and then the footer says it.
        for repair in &self.repairs {
            let open = repair_open(&repair.path);
            if let Some(failure) = repair
                .failure
                .as_ref()
                .filter(|_| first_failure != Some(repair.path.as_str()))
            {
                messages.outcome(FaultMessage::new(&ContentFault::RepairNotSaved {
                    path: &repair.path,
                    cause: failure,
                }));
            } else if let Some(found) = repair.diagnosis().filter(|_| open) {
                messages.outcome(FaultMessage::new(&ContentFault::RepairRefused {
                    path: &repair.path,
                    found: &found,
                }));
            }
        }
        // Round 27 FB3, AB1: with no package list to show, the page's errors are its content, at
        // the top of the page (`page`), not in the footer.
        if !self.body_missing() {
            if let Some(error) = &self.error {
                // One cause, one message: a missing Campaign.md says what Splat can write and
                // offers it on the same line.
                if error.contains(MISSING_CAMPAIGN) {
                    let (preview, action) = campaigns.campaign_write_preview(active);
                    messages.outcome(
                        FaultMessage::new(&ContentFault::MissingCampaign(&preview)).choice(
                            Choice::new(MessageChoice::Choice(action), ContentPress::WriteCampaign),
                        ),
                    );
                } else {
                    messages.outcome(FaultMessage::new(&ContentFault::Error(error)));
                }
            }
            if let Some(error) = &self.reconcile.error {
                messages.outcome(FaultMessage::new(&ContentFault::Error(error)));
            }
        }
        // The package faults come from the files, not from an act: after every answer.
        self.package_messages(&mut messages, campaigns, &warnings.unavailable);
        messages
    }

    /// The package list cannot show: no load is waiting and none gave a snapshot (a missing
    /// Campaign.md, a load that failed). Then the page's errors are its content.
    fn body_missing(&self) -> bool {
        self.snapshot.is_none() && self.pending.is_none()
    }

    /// The removal result of `campaign`, if it has one.
    fn removal(&self, campaign: Uuid) -> Option<&RemovedPackage> {
        removal_of(&self.results, campaign)
    }

    /// Act on a press in the page's footer. An Undo acts on the result its line showed, by its
    /// ticket, with that result's own payload: a result that a newer one replaced is gone, and
    /// nothing happens. A failed Undo keeps its result and its Undo, with the failure in the
    /// page's errors.
    fn press_result(
        &mut self,
        ctx: &StateContext,
        campaigns: &Campaigns,
        pressed: Option<design::outcome::Press<ContentPress>>,
    ) {
        let Some(press) = pressed else {
            return;
        };
        let design::outcome::Press { id, ticket } = press;
        match (id, ticket) {
            (ContentPress::Undo, Some(ticket)) => {
                // A failed Undo keeps its result and its Undo; its failure is the footer's answer
                // (`Messages::answer`), first in the footer.
                let mut results = std::mem::take(&mut self.results);
                let _ = results.undo_by(ticket, |kept| self.undo_result(&kept.payload));
                self.results = results;
            }
            // An encounter opens in Lookup by its id, whatever the page holds now.
            (ContentPress::OpenEntry(id), _) => self.open_lookup = Some(id),
            (ContentPress::Undo, None) | (ContentPress::WriteCampaign, _) => {}
            (press, _) => self.package_press(ctx, campaigns, press),
        }
    }

    /// Undo one result with its own payload: `Ok` when it worked (the store then drops it),
    /// `Err` when it failed (the failure is in the page's errors, and the result and its Undo
    /// stay).
    fn undo_result(&mut self, payload: &Payload) -> Result<(), ()> {
        match payload {
            Payload::CampaignWrite(write) => match write.undo() {
                Ok(_) => self.refresh_pending = true,
                Err(error) => {
                    self.fail(Cause::Failed, undo_failed(&error));
                    return Err(());
                }
            },
            Payload::Recovered { module, previous } => {
                if let Some(previous) = previous {
                    self.found_package = Some(previous.clone());
                } else {
                    self.forget_package = Some(module.id);
                }
            }
            Payload::RepairDiscard { path, text, .. } => {
                if let Some(repair) = self.repairs.iter_mut().find(|r| &r.path == path)
                    && let Some(active) = self.active.clone()
                {
                    let _ = repair.draft.replace(text.clone());
                    repair.draft_changed(&active, &mut self.repair_checks);
                }
            }
            Payload::Discarded { selections, .. } => {
                if let Some(snapshot) = self.snapshot.as_mut() {
                    snapshot.document.selections = selections.clone();
                }
            }
            // This package's write only; the others keep theirs.
            Payload::ManifestWrite { write, .. } => match write.undo() {
                Ok(_) => self.refresh_pending = true,
                Err(error) => {
                    self.fail(Cause::Failed, undo_failed(&error));
                    return Err(());
                }
            },
            Payload::Removed(removed) => {
                if let Some(snapshot) = self.snapshot.as_mut() {
                    let document = &mut snapshot.document;
                    if document.selections != removed.after {
                        self.fail(
                            Cause::Failed,
                            "Campaign choices changed. Keep them and edit the package choices here."
                                .into(),
                        );
                        return Err(());
                    }
                    let after = document.selections.clone();
                    document.selections = removed.before.clone();
                    match document.save() {
                        Ok(()) => {
                            self.saved_selections = document.selections.clone();
                            self.wrote_choices();
                        }
                        Err(error) => {
                            document.selections = after;
                            self.fail(Cause::Failed, undo_failed(&error));
                            return Err(());
                        }
                    }
                }
            }
            Payload::Saved | Payload::Applied(_) => {}
        }
        // The Undo worked: a newer answer than any failure before it, also its own failed try.
        self.answer = None;
        Ok(())
    }

    /// How often the page prepared a package removal's words.
    #[cfg(any(test, feature = "test-support"))]
    pub fn removal_text_preparations(&self) -> usize {
        self.removal_text_preparations
    }

    /// Write Campaign.md again from the copy Splat last read.
    fn write_campaign(&mut self, campaigns: &Campaigns, active: &Campaign) {
        match campaigns.write_campaign_again(active) {
            Ok(write) => {
                new_result!(self, Payload::CampaignWrite(write));
                self.refresh_pending = true;
            }
            Err(error) => self.fail_or_content(error),
        }
    }

    /// Save the package choices: `Save selections` or Ctrl+S (`save_shortcut`), after the
    /// package list took the frame's input (issue #513), so a box ticked in the frame of the key
    /// is in the save. Only changed choices with no repair standing are saved, as Save's own
    /// lock says; else the key does nothing.
    fn save_choices(&mut self) {
        let repairs = !self.repairs.is_empty();
        let Some(snapshot) = self.snapshot.as_mut() else {
            return;
        };
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let dirty = document.selections != self.saved_selections || document.baseline.is_none();
        if !dirty || repairs {
            return;
        }
        for selection in &mut document.selections {
            if let Some(module) = catalogue
                .resolve_ref(&selection.package)
                .ok()
                .and_then(|id| catalogue.modules.iter().find(|module| module.id == id))
            {
                selection.name = Some(module.name.clone());
            }
        }
        match document.save() {
            Ok(()) => {
                self.saved_selections = document.selections.clone();
                new_result!(self, Payload::Saved);
                self.error = None;
                self.wrote_choices();
            }
            // Ruling Q-x: the newest answer, in place of an older one.
            Err(e) => self.answer = answer_in(&self.active, Cause::Failed, e),
        }
    }

    /// Round 26 FD1: the package warnings (a faulty manifest, a chosen package that is not
    /// available) sit in the page's footer, so none of them moves the package list. Gathered once
    /// a frame, for the footer's words and for drawing.
    fn package_warnings_of(&self, campaigns: &Campaigns) -> PackageWarnings {
        let Some(snapshot) = &self.snapshot else {
            return PackageWarnings::default();
        };
        let catalogue = &snapshot.catalogue;
        let unavailable = snapshot
            .document
            .selections
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                catalogue
                    .resolve_ref(&s.package)
                    .ok()
                    .is_none_or(|id| !catalogue.modules.iter().any(|m| m.id == id))
            })
            .map(|(index, selection)| {
                let mut selection = selection.clone();
                let registered = campaigns
                    .modules()
                    .iter()
                    .find(|module| selection.package.id == Some(module.id))
                    .cloned();
                if let Some(module) = &registered {
                    selection.name = Some(module.name.clone());
                }
                let known = registered.clone().or_else(|| {
                    selection
                        .package
                        .id
                        .and_then(|id| campaigns.unregistered_module(id).cloned())
                });
                let unregistered = registered.is_none() && known.is_some();
                let relocated = known
                    .as_ref()
                    .filter(|known| !known.path.is_dir())
                    .and_then(|known| relocated_package(known, campaigns.modules()));
                let message = if let (Some(old), Some(moved)) = (&known, &relocated) {
                    format!(
                        "{} moved from {}/ to {}/.",
                        moved.name,
                        folder_words(&old.path),
                        folder_words(&moved.path)
                    )
                } else if unregistered {
                    format!("{} is not registered.", selection.display_name())
                } else {
                    selection.unavailable_message()
                };
                Unavailable {
                    index,
                    selection,
                    registered,
                    known,
                    unregistered,
                    relocated,
                    message,
                }
            })
            .collect();
        PackageWarnings { unavailable }
    }

    /// The package faults in the page's footer (round 26 FD1: so the package list keeps one y):
    /// each broken manifest and each chosen package that is not available, with its choices. A
    /// choice that writes waits for the page's unsaved changes (`locked_when`: its reason's room
    /// stays, so nothing moves).
    fn package_messages(
        &self,
        messages: &mut design::outcome::Messages<'_, ContentPress>,
        campaigns: &Campaigns,
        unavailable: &[Unavailable],
    ) {
        use design::outcome::{Choice, ChoiceKind, FaultMessage, MessageChoice};
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let catalogue = &snapshot.catalogue;
        let unsaved = self.dirty();
        let wait = || crate::sentence!("Save or discard campaign changes first.");
        for module in campaigns.modules() {
            let Some(problem) = catalogue.manifest_problems.get(&module.id) else {
                continue;
            };
            let path = &catalogue.entries[&module.id].entry.path;
            let exists = manifest_on_disk(module, path, problem);
            let fault = FaultMessage::new(&ContentFault::Manifest {
                package: &module.name,
                problem,
                on_disk: exists,
            });
            let fault = if exists {
                fault
                    .choice(Choice::new(
                        MessageChoice::Choice("Open in Library"),
                        ContentPress::OpenManifest(module.id),
                    ))
                    .choice(
                        Choice::named_as(
                            ChoiceKind::Safe,
                            crate::choice!("Replace with a basic {}", path),
                            ContentPress::WriteManifest(module.id),
                        )
                        // What it keeps and replaces, in the Library's words (one owner).
                        .help(crate::library::manifest_replace_help(path))
                        .locked_when(unsaved, wait()),
                    )
            } else {
                fault.choice(
                    Choice::new(
                        MessageChoice::Choice("Write the manifest again"),
                        ContentPress::WriteManifest(module.id),
                    )
                    .locked_when(unsaved, wait()),
                )
            };
            messages.outcome(fault);
        }
        let unsaved_any = unsaved || self.repairs.iter().any(Repair::dirty);
        for Unavailable {
            index: i,
            selection,
            known,
            unregistered,
            relocated,
            message,
            ..
        } in unavailable
        {
            let it_was = known
                .as_ref()
                .filter(|_| relocated.is_none())
                .map(|known| folder_words(&known.path));
            let users = selection.campaign_users(catalogue);
            let used_by = (!users.is_empty()).then(|| {
                (
                    and_list(users.iter().map(|(_, name)| name.as_str())),
                    users.len(),
                )
            });
            let reference = selection.package.label();
            let mut fault = FaultMessage::new(&ContentFault::Unavailable {
                message,
                it_was: it_was.as_deref(),
                used_by: used_by
                    .as_ref()
                    .map(|(names, count)| (names.as_str(), *count)),
                reference: self
                    .details
                    .contains(reference.as_str())
                    .then_some(reference.as_str()),
            });
            if relocated.is_some() {
                fault = fault.choice(
                    Choice::new(
                        MessageChoice::Choice("Use this folder"),
                        ContentPress::UseFolder(*i),
                    )
                    .locked_when(unsaved, wait()),
                );
            }
            if *unregistered && relocated.is_none() && known.is_some() {
                fault = fault.choice(
                    Choice::new(
                        MessageChoice::Choice("Register it again"),
                        ContentPress::Register(*i),
                    )
                    .locked_when(unsaved, wait()),
                );
            }
            // Round 25 family: a message action without an icon has the secondary border.
            // Ruling R21: the removal loses the package's place in the campaign, so it is a
            // `Lose` choice: the part puts it after every other choice, with `L` before it.
            fault = fault
                .choice(
                    Choice::new(
                        MessageChoice::Choice("Find the folder…"),
                        ContentPress::Find(*i),
                    )
                    .locked_when(
                        unsaved_any,
                        crate::sentence!("Save or discard changes before finding the folder."),
                    ),
                )
                .choice(Choice::new(
                    MessageChoice::Lose("Remove from this campaign"),
                    ContentPress::Remove(*i),
                ));
            // The encounters that use its creatures, each a way to open it.
            for (id, name) in users {
                fault = fault.choice(Choice::named_as(
                    ChoiceKind::Safe,
                    crate::choice!("Open {}", name),
                    ContentPress::OpenEntry(id),
                ));
            }
            // The package's identifier stays out of the words until the DM opens its details.
            fault = fault.choice(Choice::new(
                MessageChoice::Link {
                    word: "Technical details",
                    hover: None,
                },
                ContentPress::Details(*i),
            ));
            messages.outcome(fault);
        }
    }

    /// Do a package fault's choice from the footer: on the state this frame reads, by the
    /// package's id or the selection's place.
    fn package_press(&mut self, ctx: &StateContext, campaigns: &Campaigns, press: ContentPress) {
        let unavailable = self.package_warnings_of(campaigns).unavailable;
        let Some(snapshot) = self.snapshot.as_mut() else {
            return;
        };
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let mut remove = None;
        match press {
            ContentPress::OpenManifest(id) | ContentPress::WriteManifest(id) => {
                let Some(module) = campaigns.modules().iter().find(|module| module.id == id) else {
                    return;
                };
                let (Some(problem), Some(entry)) = (
                    catalogue.manifest_problems.get(&id),
                    catalogue.entries.get(&id),
                ) else {
                    return;
                };
                let path = entry.entry.path.clone();
                if press == ContentPress::OpenManifest(id) {
                    ctx.set(
                        crate::requests::OPEN_LIBRARY_REPAIR,
                        module.path.join(&path),
                    );
                    return;
                }
                let exists = manifest_on_disk(module, &path, problem);
                match crate::content_workspace::ManifestWrite::write_as(module, &path, exists) {
                    Ok(write) => {
                        // The package's manifest is one file for every campaign: a write here
                        // ends another campaign's write of it, whose Undo would undo this one.
                        for campaign in campaigns.all() {
                            self.results.owner_changed(&PageOwner {
                                campaign: campaign.id,
                                kind: PageResult::ManifestWrite(module.id),
                            });
                        }
                        new_result!(
                            self,
                            Payload::ManifestWrite {
                                package: module.id,
                                name: module.name.clone(),
                                write,
                            }
                        );
                        self.refresh_pending = true;
                    }
                    Err(error) => self.answer = answer_in(&self.active, Cause::Failed, error),
                }
                return;
            }
            _ => {}
        }
        if let ContentPress::Details(at) = press {
            if let Some(reference) = unavailable
                .iter()
                .find(|unavailable| unavailable.index == at)
                .map(|unavailable| unavailable.selection.package.label())
                && !self.details.remove(&reference)
            {
                self.details.insert(reference);
            }
            return;
        }
        let at = match press {
            ContentPress::UseFolder(at)
            | ContentPress::Register(at)
            | ContentPress::Find(at)
            | ContentPress::Remove(at) => at,
            _ => return,
        };
        // A new act on a package: an earlier failure no longer answers the last act.
        self.answer = None;
        let Some(Unavailable {
            index: i,
            selection,
            registered,
            known,
            relocated,
            ..
        }) = unavailable
            .into_iter()
            .find(|unavailable| unavailable.index == at)
        else {
            return;
        };
        let found = match press {
            ContentPress::UseFolder(_) => relocated.map(Ok),
            ContentPress::Register(_) => known.as_ref().map(|known| {
                find_package_folder(&selection.package, selection.name.as_deref(), &known.path)
            }),
            ContentPress::Find(_) => crate::hermetic::pick_folder_from(
                known.as_ref().and_then(|module| module.path.parent()),
            )
            .map(|folder| {
                find_package_folder(&selection.package, selection.name.as_deref(), &folder)
            }),
            _ => {
                remove = Some(i);
                None
            }
        };
        match found {
            Some(Ok(module)) => {
                // A package's registration is one for every campaign: a recovery here ends
                // another campaign's recovery of it, whose Undo would put back a registration
                // this one replaced.
                for campaign in campaigns.all() {
                    self.results.owner_changed(&PageOwner {
                        campaign: campaign.id,
                        kind: PageResult::Recovered(module.id),
                    });
                }
                new_result!(
                    self,
                    Payload::Recovered {
                        module: module.clone(),
                        previous: registered.clone(),
                    }
                );
                self.found_package = Some(module);
            }
            Some(Err(words)) => {
                self.answer = answer_in(
                    &self.active,
                    Cause::Refused {
                        package: selection.package.label(),
                    },
                    words,
                )
            }
            None => {}
        }
        if let Some(i) = remove {
            let selection = document.selections[i].clone();
            let before = document.selections.clone();
            let users = selection.campaign_users(catalogue);
            document.selections.remove(i);
            match document.save() {
                Ok(()) => {
                    self.saved_selections = document.selections.clone();
                    self.removal_text_preparations += 1;
                    let name = selection.display_name().to_owned();
                    let (words, warning) = removal_words(&name, &users);
                    new_result!(
                        self,
                        Payload::Removed(RemovedPackage {
                            name,
                            words,
                            warning,
                            before,
                            after: document.selections.clone(),
                            users,
                        })
                    );
                    self.wrote_choices();
                }
                Err(error) => {
                    document.selections = before;
                    self.answer = answer_in(&self.active, Cause::Failed, error);
                }
            }
        }
    }
}

/// The catalogue's and the closure's errors that the issue list shows: not those that a
/// Campaign.md flag or a package's manifest fault already says.
fn issue_errors<'c>(
    catalogue: &'c Catalogue,
    closure: &'c Closure,
    campaign_flags: &[splat::content::layout::Flag],
) -> Vec<&'c String> {
    catalogue
        .errors
        .iter()
        .filter(|error| {
            !campaign_flags.iter().any(|flag| {
                flag.words()
                    .map_or_else(|| error.contains(&flag.message), |w| error.contains(&w))
            }) && !catalogue.manifest_problems.iter().any(|(id, problem)| {
                error.contains(problem)
                    || catalogue
                        .entries
                        .get(id)
                        .is_some_and(|entry| error.contains(&entry.entry.path))
            })
        })
        .chain(&closure.errors)
        .collect()
}

/// Whether a chosen package is not in the catalogue.
fn missing_package(catalogue: &Catalogue, document: &CampaignDocument) -> bool {
    document.selections.iter().any(|selection| {
        catalogue
            .resolve_ref(&selection.package)
            .ok()
            .is_none_or(|id| !catalogue.modules.iter().any(|module| module.id == id))
    })
}

/// Read what the Packages page shows: invalid notes to repair, and the campaign's packages.
pub fn load_snapshot(campaign: &Campaign, modules: &[Module]) -> Loaded {
    let repairs: Vec<Repair> = splat::content::read_folder(&campaign.path)
        .map(|folder| {
            folder
                .errors
                .iter()
                .filter_map(|e| {
                    let path = note_path(&campaign.path, &e.path).ok()?;
                    let text = match folder.text(&e.path) {
                        Some(text) => text.to_owned(),
                        None => fs::read_to_string(path).ok()?,
                    };
                    Some(Repair::new(e.path.clone(), text, campaign))
                })
                .collect()
        })
        .unwrap_or_default();
    let result = CampaignDocument::load_with_modules(campaign, modules).and_then(|document| {
        let catalogue = Catalogue::load(modules, Some(campaign))?;
        let closure = catalogue.closure(&document.selections, document.campaign.id);
        Ok(Snapshot {
            closure: (document.selections.clone(), closure),
            document,
            subtitles: crate::library::catalogue_subtitles(&catalogue),
            catalogue,
            issues: None,
        })
    });
    (repairs, result)
}

/// `needed by Ember Cultist, Arena Final`: the one form of what keeps an entry selected, in the
/// package list's rows and in `Needed by your entries` (round 28 FB7).
fn needed_by(reasons: impl Iterator<Item = String>) -> String {
    format!("needed by {}", reasons.collect::<Vec<_>>().join(", "))
}

#[cfg(any(test, feature = "test-support"))]
#[path = "campaign_content/tests.rs"]
mod tests;
#[path = "campaign_content/view.rs"]
pub mod view;

#[cfg(feature = "test-support")]
pub use tests::applied_changes_reload_the_catalogue_and_failures_are_shown;
