//! What a campaign uses: package selections, and the repair of invalid notes. This is
//! a rare setup task, so it stays calm, with one scrolling list at 2,000+ rows.
use crate::{
    campaigns::Campaigns,
    content_workspace::{
        CampaignDocument, Catalogue, Closure, EntryRef, PackageSelection, Selection,
    },
    design::{self, Action, Density, Type, colors, icons},
    library::{note_path, write_note},
    persistence::{Campaign, Module},
};
use eframe::egui;
use splat::character::campaign_package::{self, Move, Proposal};
use splat::content::Entry;
use std::{
    collections::BTreeSet,
    fs,
    sync::mpsc,
    time::{Duration, Instant},
};
use uuid::Uuid;

/// Sheets refresh in the background, so the campaign package needs another look now and then.
const CHECK_EVERY: Duration = Duration::from_secs(5 * 60);

pub(crate) struct Snapshot {
    document: CampaignDocument,
    catalogue: Catalogue,
    /// The closure of these selections. It is too slow to calculate on every frame.
    closure: (Vec<PackageSelection>, Closure),
    subtitles: std::collections::HashMap<Uuid, String>,
}
/// An invalid note in repair: its path, its text on disk and its draft, with Save's check of the
/// draft. The check runs only where the draft changes (a load, an edit, Discard, Undo discard), so
/// an unchanged frame neither hashes nor parses the text.
pub(crate) struct Repair {
    path: String,
    baseline: String,
    draft: String,
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
            baseline: text.clone(),
            draft: text,
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
        self.draft != self.baseline
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
                self.baseline = self.draft.clone();
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
    /// How many changes the last apply made: its result's words (`applied_words`).
    applied: Option<usize>,
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

/// A kept Campaign page result. Its words come from the state its action left (the write, the
/// found folder, the discarded draft), which also says whether it still lives (`live`).
struct PageKept(PageOwner);

impl design::results::Kept for PageKept {
    type Owner = PageOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // A newer action of another kind ends a plain result (`new_result!`).
        owner_changed: design::results::OwnerChanged::Ends,
        // A reload keeps it while its state lives (`live`).
        disk_changed: design::results::DiskChanged::Stays,
        // Its Undo leaves the store only when it works (`undo_by`).
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> PageOwner {
        self.0
    }
}

/// A new result comes first, with the order its store issues at the action. It ends the plain
/// results, never another action's Undo. A macro, so it runs while other fields of the page are
/// borrowed.
macro_rules! new_result {
    ($page:expr, $kind:expr) => {
        let _ = $page.message.take();
        let _ = $page.reconcile.applied.take();
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
        $page.results.record(PageKept(PageOwner {
            campaign,
            kind: $kind,
        }));
    };
}

/// What the DM pressed in the page's footer: a result's Undo (its ticket names the result), an
/// encounter that a removal left without creatures, or the write of a missing Campaign.md.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ContentPress {
    Undo,
    /// An encounter a removal left without creatures, by its place in the removal's list.
    Open(usize),
    WriteCampaign,
    /// A broken manifest's choices, by its package.
    OpenManifest(Uuid),
    WriteManifest(Uuid),
    /// An unavailable package's choices, by its place among the campaign's selections.
    UseFolder(usize),
    Register(usize),
    Find(usize),
    Remove(usize),
    /// An encounter that uses an unavailable package's creatures.
    OpenEntry(Uuid),
    /// An unavailable package's details link: it shows or hides the package's reference.
    Details(usize),
}

/// What the Campaign page says is wrong, in its footer or its body. The tone comes from the
/// severity, as everywhere.
enum ContentFault<'a> {
    /// Save's or a load's error, or the change check's (the system's or the workspace's words).
    Error(&'a str),
    /// Campaign.md is missing: what Splat can write (the campaigns' preview words).
    MissingCampaign(&'a str),
    /// The campaign folder is not on disk.
    NoFolder,
    /// A note the page needs cannot be read; `problem` is Save's refusal or that the repair is
    /// not saved, while the repair is closed.
    Unreadable {
        path: &'a str,
        problem: Option<&'a str>,
    },
    /// A repair's Save failed on the disk.
    RepairNotSaved { path: &'a str, cause: &'a str },
    /// Save's own check refuses an open repair's draft.
    RepairRefused { path: &'a str, cause: &'a str },
    /// A package manifest that cannot be used; `on_disk`: it is there but broken (the error
    /// tone), else it is gone (a warning).
    Manifest {
        package: &'a str,
        problem: &'a str,
        /// The manifest's path in its package, for what Replace keeps and replaces.
        path: &'a str,
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
    /// A Campaign.md flag or a missing dependency (the content layer's words).
    Flag(&'a str),
    /// Encounters a package removal left without creatures.
    LeftWithout(&'a design::outcome::Sentence),
}

impl design::outcome::Fault for ContentFault<'_> {
    fn severity(&self) -> design::outcome::Severity {
        use design::outcome::Severity;
        match self {
            Self::Error(_) | Self::MissingCampaign(_) | Self::Unreadable { .. } => {
                Severity::CannotRead
            }
            Self::RepairNotSaved { .. } => Severity::NotSaved,
            Self::Manifest { on_disk: true, .. } => Severity::CannotRead,
            Self::NoFolder
            | Self::RepairRefused { .. }
            | Self::Manifest { on_disk: false, .. }
            | Self::Unavailable { .. }
            | Self::Flag(_)
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
        reason = "errors, refusals, manifest problems and flags carry the system’s, the workspace’s or the content layer’s own words"
    )]
    fn cause(&self) -> design::outcome::Sentence {
        use design::outcome::Sentence;
        let outside = |words: &str| Sentence::outside(words);
        match self {
            Self::Error(error) => {
                outside(error).unwrap_or(crate::sentence!("Something went wrong."))
            }
            Self::MissingCampaign(preview) => {
                outside(preview).unwrap_or(crate::sentence!("Campaign.md is missing."))
            }
            Self::NoFolder => crate::sentence!("This campaign folder does not exist yet."),
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
            Self::RepairRefused { cause, .. } | Self::Flag(cause) => {
                outside(cause).unwrap_or(crate::sentence!("This needs fixing."))
            }
            Self::Manifest {
                package,
                problem,
                path,
                on_disk,
            } => {
                let words = outside(&manifest_message(package, problem))
                    .unwrap_or(crate::sentence!("The package manifest cannot be used."));
                // What Replace keeps and replaces, in the Library's words (one owner).
                match on_disk
                    .then(|| {
                        outside(&crate::content_workspace::manifest_replace_consequence(
                            path,
                        ))
                    })
                    .flatten()
                {
                    Some(consequence) => words.then(consequence),
                    None => words,
                }
            }
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
    /// Where each open repair's editor sits this frame. Focus scrolling leaves them to the
    /// editor, which scrolls the page to its own viewport.
    editor_rects: Vec<egui::Rect>,
    /// The page's kept results, each with the order its action took (newest first). Each lives
    /// while its own field holds it (`live`).
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
    /// Each found package folder and the registration it replaced, by the package: one result
    /// and one Undo per package.
    recovered: Vec<(Uuid, (Module, Option<Module>))>,
    error: Option<String>,
    message: Option<String>,
    disabled_selections: std::collections::BTreeMap<Uuid, Selection>,
    refresh_pending: bool,
    removed_package: Option<RemovedPackage>,
    /// Each package manifest written again, with its package's name, by the package: one result
    /// and one Undo per package.
    manifest_writes: Vec<(Uuid, String, crate::content_workspace::ManifestWrite)>,
    campaign_write: Option<crate::content_workspace::ManifestWrite>,
    repairs: Vec<Repair>,
    /// How often a frame checked a repair draft again. Only an edit, Discard or Undo discard does.
    repair_checks: usize,
    /// What `Discard` last dropped, for one step of undo, and the Campaign.md text it was a draft
    /// of. A missing or changed Campaign.md ends it.
    discarded_selections: Option<(Vec<PackageSelection>, Option<String>)>,
    /// Each repair's discarded draft (its path and words), newest first. Round 26: one per
    /// repair, so a second repair's Discard keeps the first one's Undo.
    discarded_repairs: Vec<(String, String, u64)>,
    /// The next stamp for a repair's Discard.
    repair_stamp: u64,
    /// The repair the DM edited or focused last. It owns the primary Save while it is open and
    /// changed; other changed repairs keep their drafts with a secondary Save.
    active_repair: Option<String>,
}
struct RemovedPackage {
    campaign: Uuid,
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

/// Return a useful YAML repair message and the physical line to mark.
pub(crate) fn repair_problem(path: &str, text: &str) -> (Option<String>, Option<usize>) {
    // Repair rows and editors ask on every frame, and a parse of a long note takes about 1 ms.
    static PROBLEMS: splat::memo::Memo<(Option<String>, Option<usize>)> =
        splat::memo::Memo::new(64);
    PROBLEMS.get_or(&[path.as_bytes(), text.as_bytes()], || {
        find_repair_problem(path, text)
    })
}

fn find_repair_problem(path: &str, text: &str) -> (Option<String>, Option<usize>) {
    let Some(problem) = Entry::parse(path, text).err() else {
        return (None, None);
    };
    let yaml = text
        .strip_prefix("---\n")
        .and_then(|text| text.split_once("\n---").map(|(yaml, _)| yaml));
    if let Some(yaml) = yaml
        && let Err(error) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(yaml)
    {
        let mut opens = Vec::new();
        for (index, line) in yaml.lines().enumerate() {
            let mut quote = None;
            for ch in line.chars() {
                if matches!(ch, '\'' | '"') {
                    if quote == Some(ch) {
                        quote = None;
                    } else if quote.is_none() {
                        quote = Some(ch);
                    }
                }
                if quote.is_some() {
                    continue;
                }
                if ch == '#' {
                    break;
                }
                match ch {
                    '[' | '{' => opens.push((ch, index + 2)),
                    ']' | '}' => {
                        opens.pop();
                    }
                    _ => {}
                }
            }
        }
        if let Some((ch, line)) = opens.first() {
            let thing = if *ch == '[' { "list" } else { "mapping" };
            return (
                Some(format!(
                    "Line {line} opens a {thing} with {ch} that never closes."
                )),
                Some(*line),
            );
        }
        if let Some(location) = error.location() {
            let line = location.line() + 1;
            return (
                Some(format!(
                    "Check the YAML on line {line}. Use a name, a colon and its value."
                )),
                Some(line),
            );
        }
    }
    (Some(problem), None)
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
    let number = |n: usize| design::count(n, "", "").trim().to_owned();
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
        format!("{} of {} chosen", number(ticked), number(total))
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

/// `design::two_line_row` for a package, with a fault mark on the second line in its own tone.
/// `spoken` is the second line as the accessible name says it, with words for the mark.
fn package_row(
    ui: &mut egui::Ui,
    selected: bool,
    title: &str,
    detail: &str,
    spoken: &str,
) -> egui::Response {
    design::row(
        ui,
        Density::Glance.row() + design::XS,
        selected,
        &format!("{title} · {spoken}"),
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
            }
            Err(error) => self.error = Some(error),
        }
    }
    /// A repair's draft, by its path, for checks of who owns a result.
    #[cfg(test)]
    pub fn repair_draft(&self, path: &str) -> Option<&str> {
        self.repairs
            .iter()
            .find(|repair| repair.path == path)
            .map(|repair| repair.draft.as_str())
    }
    /// How often a frame checked a repair draft again.
    #[cfg(test)]
    pub fn repair_checks(&self) -> usize {
        self.repair_checks
    }
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some() || self.reconcile.pending.is_some()
    }

    /// Whether the package selection differs from the saved one, for tests.
    #[cfg(test)]
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
    /// Runs on every frame, on any page, so the sidebar can say when the DM has moves to review.
    pub fn tick(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        self.finish_check();
        // Round 26: a campaign change ends the Undos of the campaign that was open, at the
        // change, even while this page is hidden; a page reload is not the only end.
        let active = campaigns.active_campaign();
        if let Some(active) = &active {
            self.forget_discards_of_other_campaigns(active);
        } else if self.active.is_some() {
            self.discarded_selections = None;
            self.discarded_repairs.clear();
            self.removed_package = None;
            self.campaign_write = None;
        }
        if cfg!(test) {
            return;
        }
        let Some(active) = active else {
            return;
        };
        let due = self.reconcile.campaign != Some(active.id)
            || self
                .reconcile
                .checked_at
                .is_none_or(|at| at.elapsed() >= CHECK_EVERY);
        if due && self.reconcile.pending.is_none() {
            self.start_check(ctx, campaigns, active.clone(), None);
        }
    }
    /// Words for the sidebar, when the campaign package has something to review.
    pub fn review_detail(&self) -> Option<String> {
        let count = self.reconcile.proposals.len();
        (count > 0).then(|| design::count(count, "to review", "to review"))
    }
    fn finish_check(&mut self) {
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
        self.reconcile.checked_at = Some(Instant::now());
        self.reconcile.error = None;
        match checked.proposals {
            Ok(proposals) => self.reconcile.proposals = proposals,
            Err(error) => self.reconcile.error = Some(error),
        }
        match checked.applied {
            Some(Ok(count)) => {
                new_result!(self, PageResult::Applied);
                self.reconcile.applied = Some(count);
                self.refresh_pending = !self.dirty();
            }
            Some(Err(error)) => self.reconcile.error = Some(error),
            None => {}
        }
    }
    /// With `keys`, carries out those proposals first. The check itself changes nothing.
    fn start_check(
        &mut self,
        ctx: &egui::Context,
        campaigns: &Campaigns,
        campaign: Campaign,
        keys: Option<BTreeSet<String>>,
    ) {
        let (tx, rx) = mpsc::channel();
        self.reconcile.campaign = Some(campaign.id);
        self.reconcile.pending = Some(rx);
        self.reconcile.applying = keys.is_some();
        let modules = campaigns.modules().to_vec();
        let ctx = ctx.clone();
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
    /// Discarded work belongs to its campaign. Undo must not save it into another.
    fn forget_discards_of_other_campaigns(&mut self, campaign: &Campaign) {
        if self.active.as_ref().is_none_or(|old| old.id != campaign.id) {
            self.discarded_selections = None;
            self.discarded_repairs.clear();
            self.removed_package = None;
            self.campaign_write = None;
        }
    }
    /// Undo package removal checks the package choices only. If the file's choices changed since
    /// the removal, that Undo would refuse: the result and its Undo go.
    fn forget_changed_removal(&mut self, document: &crate::content_workspace::CampaignDocument) {
        if self.removed_package.as_ref().is_some_and(|removed| {
            removed.campaign == document.campaign.id && removed.after != document.selections
        }) {
            self.removed_package = None;
        }
    }
    /// `Undo discard` of the package choices belongs to Campaign.md as it was. When the file goes
    /// missing or changes on disk, the discarded draft and its Undo go.
    fn forget_discard_of_changed_file(&mut self, loaded: Result<&CampaignDocument, &String>) {
        if self
            .discarded_selections
            .as_ref()
            .is_some_and(|(_, baseline)| {
                !loaded.is_ok_and(|document| document.baseline == *baseline)
            })
        {
            self.discarded_selections = None;
        }
    }
    fn reload(&mut self, ctx: &egui::Context, campaigns: &Campaigns, campaign: Campaign) {
        self.forget_discards_of_other_campaigns(&campaign);
        // A write whose file changed since is stale: its Undo would only refuse.
        self.manifest_writes.retain(|(_, _, write)| write.on_disk());
        crate::content_workspace::ManifestWrite::forget_if_changed(&mut self.campaign_write);
        self.refresh_pending = false;
        self.reconcile.checked_at = None;
        self.disabled_selections.clear();
        self.repairs.clear();
        let modules = campaigns.modules().to_vec();
        self.active = Some(campaign.clone());
        self.snapshot = None;
        self.entry_matches = None;
        self.error = None;
        self.message = None;
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(load_snapshot(&campaign, &modules));
            ctx.request_repaint();
        });
    }
    pub fn clear_notice(&mut self) {
        self.message = None;
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: the columns of the package list fill the page (`Fills::because`) (design::results::STAGED_ALLOWS)"
    )]
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns) {
        let scope = (self.module, self.dependencies);
        if self.notice_scope.is_some_and(|previous| previous != scope) {
            self.clear_notice();
        }
        self.notice_scope = Some(scope);
        let Some(active) = campaigns.active_campaign() else {
            ui.label("Select a campaign first.");
            return;
        };
        if self.refresh_pending {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
        if self
            .active
            .as_ref()
            .is_none_or(|c| c.id != active.id || c.path != active.path)
        {
            if self.dirty() {
                ui.label(
                    Type::Section
                        .rich(ui, "Unsaved campaign content")
                        .color(colors().text),
                );
                ui.label("The previous campaign still has pending changes. Select it again to save them, or discard and switch.");
                if Action::destructive("Discard and switch campaign")
                    .show(ui)
                    .clicked()
                {
                    self.reload(ui.ctx(), campaigns, active);
                }
                return;
            }
            self.reload(ui.ctx(), campaigns, active.clone());
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok((repairs, result)) => {
                    self.pending = None;
                    self.repairs = repairs;
                    // A repair's discard result ends with its repair: Saved, fixed in World or
                    // deleted. It lives through a reload that finds the repair again.
                    let repairs = &self.repairs;
                    self.discarded_repairs
                        .retain(|(path, ..)| repairs.iter().any(|repair| &repair.path == path));
                    self.forget_discard_of_changed_file(result.as_ref().map(|s| &s.document));
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
        if self.dirty() {
            self.message = None;
        }
        // Round 25 review: the page scrolls, as the other pages do, so every repair and its Save
        // stay in reach at any window size. The page keeps the scroll bar's lane whether or not
        // the bar shows, so nothing moves when it starts to scroll.
        let width = ui.available_width() - design::lane_width();
        // Round 26: the page's results, reasons and package warnings sit in one footer, so none
        // of them moves the save rows, the repairs or the package list. Round 28: the pane
        // places it. With the package list, the page's columns fill the room and each scrolls
        // (`Fills`), so the footer is pinned; without it (an error, a missing folder), the
        // footer follows the page's content (row 39).
        let fills = self.snapshot.is_some();
        let frame = design::outcome::PaneFrame::new(ui);
        let footer = self.page_footer(
            &frame,
            ui,
            campaigns,
            &active,
            width.min(design::LIST_MEASURE),
        );
        let body = if fills {
            design::Body::Fills(design::Fills::because(design::FillsReason::Columns))
        } else {
            design::Body::Content
        };
        let paned = footer.pane(ui, body, |ui| {
            if fills {
                let page_height = ui.available_height();
                design::ScrollColumn::new("campaign_packages_page").show(ui, |ui| {
                    // egui gives the content only the visible height, so a scroll area inside
                    // it (a repair editor, a list) would shrink below the fold. The page's
                    // content has all the height it needs; the page scrolls instead.
                    ui.allocate_ui(egui::vec2(width, f32::INFINITY), |ui| {
                        self.editor_rects.clear();
                        self.page(ui, campaigns, &active, page_height);
                        self.scroll_to_focus(ui);
                    });
                });
            } else {
                self.editor_rects.clear();
                self.page(ui, campaigns, &active, 0.0);
                self.scroll_to_focus(ui);
            }
        });
        let write = matches!(
            paned.pressed,
            Some(design::outcome::Press {
                id: ContentPress::WriteCampaign,
                ..
            })
        );
        self.press_result(ui.ctx(), campaigns, &active, paned.pressed);
        if write {
            self.write_campaign(campaigns, &active);
        }
    }

    /// Keyboard and screen-reader focus bring the focused control into view. egui scrolls to a
    /// focused text cursor, not to a focused button, so the page does it.
    fn scroll_to_focus(&self, ui: &mut egui::Ui) {
        let id = egui::Id::new("campaign_page_focus");
        let focused = ui.memory(|memory| memory.focused());
        let last = ui
            .data(|data| data.get_temp::<Option<egui::Id>>(id))
            .flatten();
        if focused == last {
            return;
        }
        ui.data_mut(|data| data.insert_temp(id, focused));
        let page = ui.min_rect();
        if let Some(rect) = focused
            .and_then(|focused| ui.ctx().read_response(focused))
            .map(|response| response.rect)
            .filter(|rect| page.x_range().contains(rect.center().x))
            .filter(|rect| page.y_range().contains(rect.center().y))
            .filter(|rect| {
                !self
                    .editor_rects
                    .iter()
                    .any(|editor| editor.intersects(*rect))
            })
        {
            ui.scroll_to_rect_animation(rect, None, egui::style::ScrollAnimation::none());
        }
    }

    /// The Packages page below the tabs, inside its scroll area.
    #[allow(
        clippy::disallowed_methods,
        reason = "a missing campaign folder is the page’s fault with its repair, at the top of the body"
    )]
    fn page(
        &mut self,
        ui: &mut egui::Ui,
        campaigns: &Campaigns,
        active: &Campaign,
        page_height: f32,
    ) {
        let top = ui.cursor().top();
        let active = active.clone();
        if self.snapshot.is_some() {
            self.selection_bar(ui, campaigns, &active);
        }
        if self.body_missing() {
            // Round 27 FB3, AB1: a page with no package list draws its errors as its content,
            // where an empty state starts, so the DM does not look past a blank page for them.
            let write = ui
                .scope(|ui| {
                    ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
                    self.draw_reason(ui, campaigns, &active)
                })
                .inner;
            if write {
                self.write_campaign(campaigns, &active);
            }
        }
        self.repair_section(ui, campaigns, &active);
        if !active.path.exists()
            && design::outcome::BodyFault::new(&ContentFault::NoFolder)
                .repair(design::outcome::Repair::new("Create campaign folder", ()))
                .show(ui)
                .is_some()
        {
            match fs::create_dir_all(&active.path) {
                Ok(()) => self.reload(ui.ctx(), campaigns, active.clone()),
                Err(e) => self.error = Some(e.to_string()),
            }
        }
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Loading campaign content…");
            });
            return;
        }
        self.reconcile_section(ui, campaigns, &active);
        if self.snapshot.is_some() {
            // The package list fills the rest of the window, and keeps room for a few rows when
            // repairs above it are tall; then the page scrolls.
            let rows = 4.0 * (Density::Glance.row() + design::XS) + design::XXL;
            let height = (page_height - (ui.cursor().top() - top)).max(rows);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_min_height(height);
                    self.packages(ui, campaigns, &active);
                },
            );
        }
    }

    /// The page's one message footer (round 26), made before the body: the kept results (one
    /// with a live Undo first, newest first, as the store's orders say), then the reasons (each
    /// repair's failed Save or refusal, the page's errors), then the plain results and the
    /// warning about encounters a removal left without creatures. Each keeps its owner and end
    /// event: a repair's message names its file and ends with that repair's state; a kept
    /// result lives while its state does (`live`) and in its own campaign only.
    fn page_footer<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        ui: &egui::Ui,
        campaigns: &Campaigns,
        active: &Campaign,
        width: f32,
    ) -> design::outcome::Messages<'f, ContentPress> {
        use design::outcome::{Choice, ChoiceKind, FaultMessage, MessageChoice};
        let mut messages =
            design::outcome::Messages::new(frame, ("campaign_page_footer", active.id), width);
        let locked = self.dirty();
        for shown in self.results.newest_first(&active.id) {
            let kind = shown.value().0.kind;
            if self.live(kind, active) {
                let line = shown.outcome(frame, |_| self.outcome_of(kind, locked));
                messages.kept(line);
            }
        }
        // Round 27 FE5: the encounters a removal leaves without creatures, each a way to open it.
        if let Some((removed, warning)) = self
            .removed_package
            .as_ref()
            .filter(|removed| removed.campaign == active.id)
            .and_then(|removed| removed.warning.as_ref().map(|warning| (removed, warning)))
        {
            let mut fault = FaultMessage::new(&ContentFault::LeftWithout(warning));
            for (at, (_, name)) in removed.users.iter().enumerate() {
                fault = fault.choice(Choice::named_as(
                    ChoiceKind::Safe,
                    crate::choice!("Open {}", name),
                    ContentPress::Open(at),
                ));
            }
            messages.outcome(fault);
        }
        // A failed Save or a refusal belongs to its repair: its words name the file (round 27
        // FB5: two notes can share a name in different folders). Round 27 FB1, AB2: a refusal's
        // cause shows once. While the repair is closed the message above says it; only an open
        // repair's message drops it, and then the footer says it.
        for repair in &self.repairs {
            let open = ui.data(|data| {
                data.get_temp::<bool>(egui::Id::new(("repair_open", repair.path.as_str())))
                    .unwrap_or(false)
            });
            if let Some(failure) = &repair.failure {
                messages.outcome(FaultMessage::new(&ContentFault::RepairNotSaved {
                    path: &repair.path,
                    cause: failure,
                }));
            } else if let Some(refusal) = repair.refusal.as_ref().filter(|_| open) {
                messages.outcome(FaultMessage::new(&ContentFault::RepairRefused {
                    path: &repair.path,
                    cause: refusal,
                }));
            }
        }
        // The package faults come before the page's errors: an error that a package choice brings
        // (a wrong folder) comes below the pressed choice, so it does not move.
        let warnings = self.package_warnings_of(campaigns);
        self.package_messages(&mut messages, campaigns, &warnings.unavailable);
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
        messages
    }

    /// The package list cannot show: no load is waiting and none gave a snapshot (a missing
    /// Campaign.md, a load that failed). Then the page's errors are its content.
    fn body_missing(&self) -> bool {
        self.snapshot.is_none() && self.pending.is_none()
    }

    /// The page's errors as its content (no package list to show): each a fault in the body, a
    /// missing Campaign.md with the way to write it. True when the DM asks to write it again.
    #[allow(
        clippy::disallowed_methods,
        reason = "with no package list, the page’s errors are its content: body faults (round 27 FB3, AB1)"
    )]
    fn draw_reason(&self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) -> bool {
        use design::outcome::{BodyFault, Repair};
        let mut write = false;
        if let Some(error) = &self.error {
            if error.contains(MISSING_CAMPAIGN) {
                let (preview, action) = campaigns.campaign_write_preview(active);
                write = BodyFault::new(&ContentFault::MissingCampaign(&preview))
                    .repair(Repair::new(action, ()))
                    .show(ui)
                    .is_some();
            } else {
                let _ = BodyFault::<()>::new(&ContentFault::Error(error)).show(ui);
            }
        }
        if let Some(error) = &self.reconcile.error {
            let _ = BodyFault::<()>::new(&ContentFault::Error(error)).show(ui);
        }
        write
    }

    /// Whether a kept result still lives: its own field holds its state, and a removal is this
    /// campaign's.
    fn live(&self, kind: PageResult, active: &Campaign) -> bool {
        match kind {
            PageResult::CampaignWrite => self.campaign_write.is_some(),
            PageResult::Recovered(package) => self.recovered.iter().any(|(id, _)| *id == package),
            PageResult::Discarded => self.discarded_selections.is_some(),
            PageResult::ManifestWrite(package) => {
                self.manifest_writes.iter().any(|(id, _, _)| *id == package)
            }
            PageResult::Removed => self
                .removed_package
                .as_ref()
                .is_some_and(|removed| removed.campaign == active.id),
            PageResult::RepairDiscard(stamp) => {
                self.discarded_repairs.iter().any(|(.., s)| *s == stamp)
            }
            PageResult::Saved => self.message.is_some(),
            PageResult::Applied => self.reconcile.applied.is_some(),
        }
    }

    /// The file name of a repair's discarded draft, by its stamp.
    fn discarded_repair(&self, stamp: u64) -> Option<&str> {
        self.discarded_repairs
            .iter()
            .find(|(.., s)| *s == stamp)
            .map(|(path, ..)| path.rsplit('/').next().unwrap_or(path))
    }

    /// One live result as the footer says it: its words, and its Undo. While a draft waits, a
    /// manifest write's Undo is locked (`locked`): the write happened, so its words stay.
    #[allow(
        clippy::disallowed_methods,
        reason = "a manifest write’s words are the content workspace’s own (`ManifestWrite::message`)"
    )]
    fn outcome_of(
        &self,
        kind: PageResult,
        locked: bool,
    ) -> design::outcome::KeptOutcome<ContentPress> {
        use design::outcome::{KeptOutcome, Notice, Said, Sentence, UndoStep, Verb};
        let undo = |verb: Verb, object: &str| {
            UndoStep::of(verb, object.to_owned()).pressed_as(ContentPress::Undo)
        };
        let discarded = |name: &str| {
            KeptOutcome::Notice(
                Notice::new(crate::sentence!("{} draft discarded.", name)),
                undo(Verb::Discard, name),
            )
        };
        match kind {
            PageResult::CampaignWrite => {
                Said::new(crate::sentence!("Campaign.md was written again."))
                    .undo(undo(Verb::Write, "Campaign.md"))
                    .into()
            }
            PageResult::Recovered(package) => {
                let (name, folder) = self
                    .recovered
                    .iter()
                    .find(|(id, _)| *id == package)
                    .map(|(_, (module, _))| (module.name.clone(), folder_words(&module.path)))
                    .unwrap_or_default();
                Said::new(crate::sentence!("{} found in {}/.", name, folder))
                    .undo(undo(Verb::Recover, &folder))
                    .into()
            }
            PageResult::Discarded => discarded("Campaign packages"),
            PageResult::RepairDiscard(stamp) => {
                discarded(self.discarded_repair(stamp).unwrap_or("Repair"))
            }
            PageResult::ManifestWrite(package) => {
                let (words, path) = self
                    .manifest_writes
                    .iter()
                    .find(|(id, _, _)| *id == package)
                    .map(|(_, name, write)| {
                        (
                            format!("{name}: {}", write.message()),
                            write.path().to_owned(),
                        )
                    })
                    .unwrap_or_default();
                Said::new(
                    Sentence::outside(words)
                        .unwrap_or(crate::sentence!("The manifest was written again.")),
                )
                .undo(undo(Verb::Write, &path).locked_when(
                    locked,
                    crate::sentence!("Save or discard campaign changes first."),
                ))
                .into()
            }
            PageResult::Removed => {
                let (words, name) = self
                    .removed_package
                    .as_ref()
                    .map(|removed| (removed.words.clone(), removed.name.as_str()))
                    .unwrap_or((crate::sentence!("The package was removed."), ""));
                Said::new(words).undo(undo(Verb::Remove, name)).into()
            }
            PageResult::Saved => Said::new(crate::sentence!("Campaign selections saved.")).into(),
            PageResult::Applied => {
                Said::new(applied_words(self.reconcile.applied.unwrap_or(1))).into()
            }
        }
    }

    /// Act on a press in the page's footer. An Undo acts on the result its line showed, by its
    /// ticket: a result that a newer one replaced is gone, and nothing happens. A failed Undo
    /// keeps its result and its Undo, with the failure in the page's errors.
    fn press_result(
        &mut self,
        ctx: &egui::Context,
        campaigns: &Campaigns,
        active: &Campaign,
        pressed: Option<design::outcome::Press<ContentPress>>,
    ) {
        let Some(design::outcome::Press { id, ticket }) = pressed else {
            return;
        };
        match (id, ticket) {
            (ContentPress::Open(at), _) => {
                if let Some((id, _)) = self
                    .removed_package
                    .as_ref()
                    .and_then(|removed| removed.users.get(at))
                {
                    self.open_lookup = Some(*id);
                }
            }
            (ContentPress::Undo, Some(ticket)) => {
                let mut results = std::mem::take(&mut self.results);
                let _ = results.undo_by(ticket, |kept| self.undo_result(kept.0.kind));
                self.results = results;
            }
            (ContentPress::Undo, None) | (ContentPress::WriteCampaign, _) => {}
            (press, _) => self.package_press(ctx, campaigns, active, press),
        }
    }

    /// Undo one result: `Ok` when it worked (its state is gone), `Err` when it failed (the
    /// failure is in the page's errors, and the result and its Undo stay).
    fn undo_result(&mut self, kind: PageResult) -> Result<(), ()> {
        match kind {
            PageResult::CampaignWrite => {
                if let Some(write) = &self.campaign_write {
                    match write.undo() {
                        Ok(_) => {
                            self.campaign_write = None;
                            self.refresh_pending = true;
                        }
                        Err(error) => {
                            self.error = Some(error);
                            return Err(());
                        }
                    }
                }
            }
            PageResult::Recovered(package) => {
                if let Some(at) = self.recovered.iter().position(|(id, _)| *id == package) {
                    let (_, (module, previous)) = self.recovered.remove(at);
                    if let Some(previous) = previous {
                        self.found_package = Some(previous);
                    } else {
                        self.forget_package = Some(module.id);
                    }
                }
            }
            PageResult::RepairDiscard(stamp) => {
                if let Some(at) = self
                    .discarded_repairs
                    .iter()
                    .position(|(.., s)| *s == stamp)
                {
                    let (path, text, _) = self.discarded_repairs.remove(at);
                    if let Some(repair) = self.repairs.iter_mut().find(|r| r.path == path)
                        && let Some(active) = self.active.clone()
                    {
                        repair.draft = text;
                        repair.draft_changed(&active, &mut self.repair_checks);
                    }
                }
            }
            PageResult::Discarded => {
                if let Some((selections, _)) = self.discarded_selections.take()
                    && let Some(snapshot) = self.snapshot.as_mut()
                {
                    snapshot.document.selections = selections;
                }
            }
            PageResult::ManifestWrite(package) => {
                // This package's write only; the others keep theirs.
                if let Some((_, _, write)) = self
                    .manifest_writes
                    .iter()
                    .find(|(id, _, _)| *id == package)
                {
                    match write.undo() {
                        Ok(_) => {
                            self.manifest_writes.retain(|(id, _, _)| *id != package);
                            self.refresh_pending = true;
                        }
                        Err(error) => {
                            self.error = Some(error);
                            return Err(());
                        }
                    }
                }
            }
            PageResult::Removed => {
                if let Some(removed) = &self.removed_package
                    && let Some(snapshot) = self.snapshot.as_mut()
                {
                    let document = &mut snapshot.document;
                    if document.selections != removed.after {
                        self.error = Some(
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
                            self.removed_package = None;
                            self.reconcile.checked_at = None;
                        }
                        Err(error) => {
                            document.selections = after;
                            self.error = Some(error);
                            return Err(());
                        }
                    }
                }
            }
            PageResult::Saved | PageResult::Applied => {}
        }
        Ok(())
    }

    /// How often the page prepared a package removal's words.
    #[cfg(test)]
    pub fn removal_text_preparations(&self) -> usize {
        self.removal_text_preparations
    }

    /// Write Campaign.md again from the copy Splat last read.
    fn write_campaign(&mut self, campaigns: &Campaigns, active: &Campaign) {
        match campaigns.write_campaign_again(active) {
            Ok(write) => {
                new_result!(self, PageResult::CampaignWrite);
                self.campaign_write = Some(write);
                self.refresh_pending = true;
            }
            Err(error) => self.error = Some(error),
        }
    }

    /// Each change waits for the DM. A blocked one says what they must decide first.
    fn reconcile_section(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        let reconcile = &mut self.reconcile;
        if reconcile.applying {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Applying changes to the campaign package…");
            });
            return;
        }
        if reconcile.proposals.is_empty() {
            return;
        }
        let c = colors();
        ui.label(
            Type::Section
                .rich(ui, "Campaign package: changes to review")
                .color(c.text),
        );
        design::ScrollColumn::new("campaign package changes")
            .max_height(220.0)
            .fit_height()
            .show(ui, |ui| {
                for proposal in &reconcile.proposals {
                    let line = format!(
                        "{} ({}): {}",
                        proposal.name,
                        proposal.kind,
                        match &proposal.change {
                            Move::ToPackage(package) => format!("move to {package}"),
                            Move::ToCampaign =>
                                "move back to the campaign package; its package is no longer enabled"
                                    .to_owned(),
                            Move::Forget => "remove; no current entry uses it".to_owned(),
                        }
                    );
                    if let Some(reason) = &proposal.blocked {
                        ui.add(
                            egui::Label::new(
                                Type::Small
                                    .rich(ui, format!("{line}. Not available: {reason}"))
                                    .color(c.muted),
                            )
                            .wrap(),
                        );
                        continue;
                    }
                    let mut chosen = !reconcile.declined.contains(&proposal.key);
                    if ui.checkbox(&mut chosen, line).changed() {
                        if chosen {
                            reconcile.declined.remove(&proposal.key);
                        } else {
                            reconcile.declined.insert(proposal.key.clone());
                        }
                    }
                }
            });
        let keys: BTreeSet<String> = reconcile
            .proposals
            .iter()
            .filter(|p| p.blocked.is_none() && !reconcile.declined.contains(&p.key))
            .map(|p| p.key.clone())
            .collect();
        let unsaved = self
            .snapshot
            .as_ref()
            .is_some_and(|s| s.document.selections != self.saved_selections);
        let label = format!("Apply {}", design::count(keys.len(), "change", "changes"));
        let apply = Action::secondary(&label)
            .density(Density::Read)
            .hint("Sheets follow the change. Package notes get the D&D Beyond ID and keep their words. Old notes go to archives.")
            .blocked(if unsaved {
                Some("Save or discard your selection changes first.")
            } else if keys.is_empty() {
                Some("Tick a change to apply it.")
            } else {
                None
            })
            .show(ui)
            .clicked();
        if apply && self.reconcile.pending.is_none() {
            self.reconcile.applied = None;
            self.start_check(ui.ctx(), campaigns, active.clone(), Some(keys));
        }
        design::gap(ui, design::Gap::S);
    }

    fn repair_section(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            self.repair_section_content(ui, campaigns, active);
        });
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: a raw heading that a design part replaces; a note that cannot be read is a fault with its repair, where the repair opens (design::results::STAGED_ALLOWS)"
    )]
    fn repair_section_content(
        &mut self,
        ui: &mut egui::Ui,
        campaigns: &Campaigns,
        active: &Campaign,
    ) {
        let mut repaired = false;
        // One primary Save on the screen: the active repair's, else the last open changed one.
        let changed: Vec<&str> = self
            .repairs
            .iter()
            .filter(|repair| {
                repair.dirty()
                    && repair.refusal.is_none()
                    && repair.failure.is_none()
                    && ui.data(|data| {
                        data.get_temp::<bool>(egui::Id::new(("repair_open", repair.path.as_str())))
                            .unwrap_or(false)
                    })
            })
            .map(|repair| repair.path.as_str())
            .collect();
        // Only a draft that Save would write, and whose last Save did not fail on the disk, can
        // hold the primary.
        let owner = (!self.run_open)
            .then(|| {
                self.active_repair
                    .as_deref()
                    .filter(|path| changed.contains(path))
                    .or(changed.last().copied())
                    .map(str::to_owned)
            })
            .flatten();
        let checks = &mut self.repair_checks;
        for repair in &mut self.repairs {
            // The message, the state word and Save all use Save's own check of the draft, which
            // the repair keeps until the draft changes.
            let path = repair.path.clone();
            let path = &path;
            // A failed Save shows once, in the save row's state slot, so the message stops at
            // the way to fix the file.
            let id = egui::Id::new(("repair_open", path.as_str()));
            let mut open = ui.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
            // Round 25 FD1: while the repair is open, its message keeps the same words, and the
            // cause moves to the message space below the save row. So no edit moves the editor.
            let problem = match (&repair.refusal, &repair.failure) {
                _ if open => None,
                (Some(refusal), _) => Some(refusal.clone()),
                (None, Some(_)) => None,
                (None, None) => Some("The repair has not been saved".into()),
            };
            if design::outcome::BodyFault::new(&ContentFault::Unreadable {
                path,
                problem: problem.as_deref(),
            })
            .repair(design::outcome::Repair::new(
                if open { "Close repair" } else { "Repair" },
                (),
            ))
            .show(ui)
            .is_some()
            {
                open = !open;
            }
            ui.data_mut(|data| data.insert_temp(id, open));
            if open {
                // The message above names the file; the editor's accessible name names it too.
                design::group_heading(ui, "Repair", 0);
                ui.scope(|ui| {
                    // What is wrong comes first, and stays true as the DM edits.

                    let editor = ui.scope(|ui| {
                        crate::world::repair::editor_events(
                            ui,
                            path,
                            &mut repair.draft,
                            repair.error_line,
                        )
                    });
                    self.editor_rects.push(editor.response.rect);
                    let (edited, focused) = editor.inner;
                    if edited || focused {
                        self.active_repair = Some(path.clone());
                    }
                    if edited {
                        repair.draft_changed(active, checks);
                    }

                    // One save row (`DraftState`): the word, Discard and the switch reason at
                    // the left, Save at the right. Primary while the draft has changes that
                    // Save would write, unless an open run owns `Next turn` or another repair
                    // is the active one. After Save failed on the disk it stays open to try
                    // again, but `owner` skips it until the next edit.
                    let mut state = design::DraftState::new(
                        if repair.dirty() {
                            design::save_row::Changed::Yes
                        } else {
                            design::save_row::Changed::No
                        },
                        if owner.as_deref() == Some(path.as_str()) {
                            design::save_row::Primary::Draft
                        } else {
                            design::save_row::Primary::Elsewhere
                        },
                    )
                    .locks_campaign();
                    if let Some(refusal) = &repair.refusal {
                        state = state.refused(refusal.as_str());
                    }
                    if repair.failure.is_some() {
                        state = state.not_saved();
                    }
                    let row = design::save_row(ui, &state, "Save repaired note");
                    if row.save
                        || (state.can_save()
                            && ui.input_mut(|i| {
                                i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)
                            }))
                    {
                        repaired |= repair.save(active);
                    }
                    if row.discard {
                        self.discarded_repairs.retain(|(p, ..)| p != path);
                        let stamp = self.repair_stamp;
                        self.repair_stamp += 1;
                        self.discarded_repairs
                            .insert(0, (path.clone(), repair.draft.clone(), stamp));
                        new_result!(self, PageResult::RepairDiscard(stamp));
                        repair.draft = repair.baseline.clone();
                        repair.draft_changed(active, checks);
                    }
                    // This repair's next edit ends its own result only.
                    if repair.dirty() {
                        self.discarded_repairs.retain(|(p, ..)| p != path);
                    }
                });
            }
        }
        if self.repairs.iter().any(|repair| {
            ui.data(|data| {
                data.get_temp::<bool>(egui::Id::new(("repair_open", repair.path.as_str())))
                    .unwrap_or(false)
            })
        }) {
            // The package list below draws the one rule; a second rule here would separate nothing.
            design::gap(ui, design::Gap::L);
        }
        if repaired && !self.dirty() {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
    }

    fn selection_bar(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            self.selection_bar_content(ui, campaigns, active);
        });
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn selection_bar_content(
        &mut self,
        ui: &mut egui::Ui,
        campaigns: &Campaigns,
        active: &Campaign,
    ) {
        let snapshot = self.snapshot.as_mut().unwrap();
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        if snapshot.closure.0 != document.selections {
            snapshot.closure = (
                document.selections.clone(),
                catalogue.closure(&document.selections, document.campaign.id),
            );
        }
        let mut reload = false;
        let c = colors();
        let mut repair_reason = None;
        ui.horizontal(|ui| {
            let dirty = document.selections != self.saved_selections || document.baseline.is_none();
            let repair_text = self
                .repairs
                .first()
                .map(|repair| format!("Fix {} first.", repair.path));
            let repair = repair_text.as_deref();
            let save_key = dirty
                && repair.is_none()
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
            let changed = document.selections != self.saved_selections;
            if changed {
                design::status(
                    ui,
                    design::Status {
                        icon: icons::DRAFT,
                        word: "Unsaved",
                        color: c.document.unsaved,
                    },
                );
                self.discarded_selections = None;
                if Action::quiet("Discard")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    new_result!(self, PageResult::Discarded);
                    self.discarded_selections = Some((
                        std::mem::replace(&mut document.selections, self.saved_selections.clone()),
                        document.baseline.clone(),
                    ));
                }
            } else {
                design::save_row_state(
                    ui,
                    design::Status {
                        icon: icons::OK,
                        word: "No changes",
                        color: c.muted,
                    },
                    false,
                    None,
                );
            }
            if !dirty {
                // Round 28 FB5: a repair's draft blocks Reload, which would drop it; Reload keeps
                // its place and says why, as on Encounters.
                reload = Action::quiet("Reload from disk")
                    .icon(icons::REFRESH)
                    .density(Density::Read)
                    .blocked_if(
                        self.repairs.iter().any(Repair::dirty),
                        "Save or discard your changes first.",
                    )
                    .show(ui)
                    .clicked();
            }
            if document.selections != self.saved_selections {
                // A repair blocks Save selections: Discard is the one way to switch.
                design::blocked_reason(
                    ui,
                    Some(if repair.is_some() {
                        crate::encounters::blocked_draft_switch_reason()
                    } else {
                        crate::encounters::draft_switch_reason()
                    }),
                );
            }
            let pressed = ui
                .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let blocked = repair.or((!dirty).then_some("No changes to save."));
                    let pressed = (if self.run_open {
                        Action::secondary("Save selections")
                    } else {
                        Action::primary("Save selections")
                    })
                    .density(Density::Read)
                    .key("Ctrl+S")
                    .blocked(blocked)
                    .show(ui)
                    .clicked();
                    // One place at every width: beside Save it would fit only once `Reload from
                    // disk` hides, so it goes to the message space below.
                    repair_reason = repair.map(str::to_owned);
                    pressed
                })
                .inner;
            if pressed || save_key {
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
                        self.reconcile.checked_at = None;
                        new_result!(self, PageResult::Saved);
                        self.message = Some("Campaign selections saved.".into());
                        self.error = None;
                    }
                    Err(e) => self.error = Some(e),
                }
            }
        });
        // The repair that blocks Save keeps its own line below Save at every width: it stays while
        // the repair does, so it never comes and goes under the DM's work.
        if let Some(reason) = &repair_reason {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    design::blocked_reason(ui, Some(reason));
                });
            });
        }
        if reload {
            self.reload(ui.ctx(), campaigns, active.clone());
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
                path,
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
                    MessageChoice::Choice("Remove from this campaign"),
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
    fn package_press(
        &mut self,
        ctx: &egui::Context,
        campaigns: &Campaigns,
        active: &Campaign,
        press: ContentPress,
    ) {
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
                    ctx.data_mut(|data| {
                        data.insert_temp(
                            egui::Id::new("open_library_repair"),
                            module.path.join(&path),
                        )
                    });
                    return;
                }
                let exists = manifest_on_disk(module, &path, problem);
                match crate::content_workspace::ManifestWrite::write_as(module, &path, exists) {
                    Ok(write) => {
                        new_result!(self, PageResult::ManifestWrite(module.id));
                        self.manifest_writes.retain(|(id, _, _)| *id != module.id);
                        self.manifest_writes
                            .push((module.id, module.name.clone(), write));
                        self.refresh_pending = true;
                    }
                    Err(error) => self.error = Some(error),
                }
                return;
            }
            ContentPress::OpenEntry(id) => {
                self.open_lookup = Some(id);
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
                new_result!(self, PageResult::Recovered(module.id));
                self.recovered.retain(|(id, _)| *id != module.id);
                self.recovered
                    .push((module.id, (module.clone(), registered.clone())));
                self.found_package = Some(module);
            }
            Some(Err(error)) => self.error = Some(error),
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
                    new_result!(self, PageResult::Removed);
                    self.removal_text_preparations += 1;
                    let name = selection.display_name().to_owned();
                    let (words, warning) = removal_words(&name, &users);
                    self.removed_package = Some(RemovedPackage {
                        campaign: active.id,
                        name,
                        words,
                        warning,
                        before,
                        after: document.selections.clone(),
                        users,
                    });
                    self.reconcile.checked_at = None;
                }
                Err(error) => {
                    document.selections = before;
                    self.error = Some(error);
                }
            }
        }
    }

    fn packages(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            self.packages_content(ui, campaigns, active);
        });
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "Campaign.md flags and missing dependencies are faults in the issue list, each with its repairs"
    )]
    fn packages_content(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        // Review round 26: egui panels set their own clip to the panel's rect, so their rows would
        // paint and take clicks below the page's viewport, under the footer. Each column clips
        // to the page's viewport again, so the page's body ends where `Footer::show` says.
        let viewport = ui.clip_rect();
        let snapshot = self.snapshot.as_mut().unwrap();
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let closure = &snapshot.closure.1;
        let c = colors();

        let affected: Vec<_> = catalogue
            .entries
            .values()
            .filter(|entry| entry.private && entry.entry.kind == "encounter")
            .filter(|entry| {
                splat::content::required_references(&entry.entry).is_ok_and(|references| {
                    references.iter().any(|r| catalogue.resolve(r).is_err())
                })
            })
            .map(|entry| (entry.entry.id, entry.entry.name.clone()))
            .collect();
        let campaign_flags = catalogue
            .entries
            .get(&active.id)
            .map(|entry| entry.entry.flags.as_slice())
            .unwrap_or_default();
        let errors: Vec<_> = catalogue
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
            .collect();
        let missing_package = document.selections.iter().any(|selection| {
            catalogue
                .resolve_ref(&selection.package)
                .ok()
                .is_none_or(|id| !catalogue.modules.iter().any(|module| module.id == id))
        });
        let issues_shown = self.repairs.is_empty()
            && !missing_package
            && !self
                .removed_package
                .as_ref()
                .is_some_and(|removed| removed.campaign == active.id && !removed.users.is_empty());
        let flag_lines = |ui: &mut egui::Ui, open_run_settings: &mut bool| {
            for (at, flag) in campaign_flags.iter().enumerate() {
                // Every flag is about Campaign.md, so one file line follows the last of them.
                let last = at + 1 == campaign_flags.len();
                ui.scope(|ui| {
                    ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
                    let mut fault =
                        design::outcome::BodyFault::new(&ContentFault::Flag(&flag.message));
                    if flag.kind == splat::content::layout::FlagKind::RuleChoice {
                        fault = fault
                            .repair(design::outcome::Repair::new("Choose in Rules and runs", ()));
                    }
                    *open_run_settings |= fault.show(ui).is_some();
                    if last {
                        // The file line starts at the warning's words, as its repairs do.
                        design::message_choice_row(ui, design::Tone::Warning, |ui| {
                            ui.vertical(|ui| {
                                crate::encounters::file_line(ui, &active.path, &document.path)
                            });
                        });
                    }
                });
            }
        };
        // Campaign.md flags stay in view while another repair hides the issue list.
        if !issues_shown && !campaign_flags.is_empty() {
            flag_lines(ui, &mut self.open_run_settings);
        }
        if (!errors.is_empty() || !campaign_flags.is_empty()) && issues_shown {
            let heading = if affected.is_empty() {
                crate::encounters::content_issue_count(errors.len() + campaign_flags.len())
            } else {
                let gone = format!(
                    "{} need a creature that is gone",
                    design::count(affected.len(), "encounter", "encounters")
                );
                if campaign_flags.is_empty() {
                    gone
                } else {
                    format!(
                        "{gone} · {}",
                        design::count(
                            campaign_flags.len(),
                            "Campaign.md issue",
                            "Campaign.md issues"
                        )
                    )
                }
            };
            // A fault count is a status: the warning icon and colour, then the count.
            let job = crate::encounters::issue_count_title(ui, &heading);
            let issues = egui::CollapsingHeader::new(job)
                .icon(design::section_icon)
                .default_open(!campaign_flags.is_empty())
                .show(ui, |ui| {
                    design::ScrollColumn::new("campaign_issues")
                        .max_height(design::FORM_WIDTH / 2.0)
                        .fit_height()
                        .show(ui, |ui| {
                            flag_lines(ui, &mut self.open_run_settings);
                            for e in errors.iter().filter(|error| {
                                !closure
                                    .missing_dependencies
                                    .iter()
                                    .any(|issue| issue.message.as_str() == error.as_str())
                            }) {
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, e.as_str()).color(c.muted),
                                    )
                                    .wrap(),
                                );
                            }
                            for issue in &closure.missing_dependencies {
                                ui.push_id(&issue.reference, |ui| {
                                    // FA6: a dependency issue is a warning like the flags above
                                    // it, and its repairs start at its words, one per owner.
                                    let mut fault = design::outcome::BodyFault::new(
                                        &ContentFault::Flag(&issue.message),
                                    );
                                    for id in &issue.owners {
                                        if let Some(path) = catalogue.file_path_of(*id) {
                                            let owner = &catalogue.entries[id];
                                            fault = fault.repair(design::outcome::Repair::new(
                                                crate::choice!("Repair {}", owner.entry.name),
                                                path.to_path_buf(),
                                            ));
                                        }
                                    }
                                    let repair = fault.show(ui);
                                    if let Some(path) = repair {
                                        ui.ctx().data_mut(|data| {
                                            data.insert_temp(
                                                egui::Id::new("open_library_repair"),
                                                path,
                                            );
                                            // The editor names the missing link and opens at
                                            // its line.
                                            if let Some(target) = &issue.target {
                                                data.insert_temp(
                                                    egui::Id::new("open_library_repair_link"),
                                                    Some((target.clone(), issue.message.clone())),
                                                );
                                            }
                                        });
                                    }
                                    design::quiet_section(ui, "Technical details", |ui| {
                                        ui.label(
                                            Type::Small.rich(ui, issue.technical()).color(c.muted),
                                        );
                                    });
                                });
                            }
                            for entry in catalogue.entries.values().filter(|entry| {
                                entry.private
                                    && !closure
                                        .missing_dependencies
                                        .iter()
                                        .any(|issue| issue.owners.contains(&entry.entry.id))
                            }) {
                                if splat::content::required_references(&entry.entry).is_ok_and(
                                    |references| {
                                        references
                                            .iter()
                                            .any(|reference| catalogue.resolve(reference).is_err())
                                    },
                                ) && design::link(ui, &format!("Open {}", entry.entry.name))
                                    .clicked()
                                {
                                    self.open_lookup = Some(entry.entry.id);
                                }
                            }
                        });
                });
            design::named(ui, &issues.header_response, &heading);
        }
        design::rule(ui);
        design::Column::left("campaign_modules", 236.0, 180.0..=420.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: 0,
                right: design::M as i8,
                top: design::XS as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                ui.set_clip_rect(ui.clip_rect().intersect(viewport));
                design::ScrollColumn::new("package_list").show(ui, |ui| {
                    for module in catalogue.modules.iter().filter(|m| !m.private) {
                        let existing = document.selections.iter().position(|s| {
                            catalogue.resolve_ref(&s.package).ok() == Some(module.id)
                        });
                        let total = module
                            .entries
                            .iter()
                            .filter(|id| catalogue.selectable(**id))
                            .count();
                        let count =
                            existing.map_or(0, |i| match &document.selections[i].selection {
                                Selection::All => total,
                                Selection::Selected { entries } => entries.len(),
                            });
                        // What the campaign's other choices pull in is ticked too.
                        let required = module
                            .entries
                            .iter()
                            .filter(|id| {
                                catalogue.selectable(**id)
                                    && closure.required.contains_key(id)
                                    && !closure.direct.contains(id)
                            })
                            .count();
                        let partial = count + required < total
                            && existing.is_some_and(|i| {
                                matches!(&document.selections[i].selection,
                                    Selection::Selected { entries } if !entries.is_empty())
                            });
                        let mut enabled = count > 0;
                        ui.push_id(module.id, |ui| {
                            let rect = egui::Rect::from_min_size(
                                ui.cursor().min,
                                egui::vec2(
                                    ui.available_width(),
                                    Density::Glance.row() + design::XS,
                                ),
                            );
                            let selected = !self.dependencies && self.module == Some(module.id);
                            let hovered = ui.rect_contains_pointer(rect);
                            if selected || hovered {
                                ui.painter().rect_filled(
                                    rect,
                                    design::RADIUS_CONTROL,
                                    if selected {
                                        colors().selected
                                    } else {
                                        colors().hover
                                    },
                                );
                            }
                            // Round 28 FC12: where the row's name starts, for its note.
                            let name_left = ui
                                .horizontal(|ui| {
                                    let check = ui
                                        .add(egui::Checkbox::new(&mut enabled, "").indeterminate(
                                            partial || (count == 0 && required > 0),
                                        ));
                                    design::named(ui, &check, &format!("Use {}", module.name));
                                    if check.changed() {
                                        self.message = None;
                                        if enabled {
                                            if let Some(i) = existing {
                                                document.selections.remove(i);
                                            }
                                            document.selections.push(PackageSelection {
                                                name: Some(module.name.clone()),
                                                package: EntryRef::id(module.id),
                                                selection: self
                                                    .disabled_selections
                                                    .remove(&module.id)
                                                    .unwrap_or(Selection::All),
                                            });
                                        } else if let Some(i) = existing {
                                            let removed = document.selections.remove(i);
                                            self.disabled_selections
                                                .insert(module.id, removed.selection);
                                        }
                                    }
                                    // Round 24 FF4: a manifest fault the warning above names
                                    // is marked on its row too, in its tone.
                                    let mark = catalogue.manifest_problems.get(&module.id).map(
                                        |problem| {
                                            let folder = campaigns
                                                .modules()
                                                .iter()
                                                .find(|known| known.id == module.id);
                                            manifest_mark(
                                                folder
                                                    .zip(catalogue.entries.get(&module.id))
                                                    .is_some_and(|(folder, entry)| {
                                                        manifest_on_disk(
                                                            folder,
                                                            &entry.entry.path,
                                                            problem,
                                                        )
                                                    }),
                                            )
                                        },
                                    );
                                    let counted = package_count(
                                        count + required,
                                        total,
                                        partial || (count == 0 && required > 0),
                                    );
                                    let (detail, spoken) = match mark {
                                        Some((icon, words)) => (
                                            format!("{icon} {counted}"),
                                            format!("{words} · {counted}"),
                                        ),
                                        None => (counted.clone(), counted),
                                    };
                                    let row = package_row(
                                        ui,
                                        !self.dependencies && self.module == Some(module.id),
                                        &module.name,
                                        &detail,
                                        &spoken,
                                    );
                                    if row.clicked() {
                                        self.module = Some(module.id);
                                        self.dependencies = false;
                                        self.search.clear();
                                    }
                                    row.rect.left() + design::S
                                })
                                .inner;
                            if count == 0 && required > 0 {
                                let selection = PackageSelection {
                                    name: Some(module.name.clone()),
                                    package: EntryRef::id(module.id),
                                    selection: Selection::All,
                                };
                                let users = selection.campaign_users(catalogue);
                                let mut users = users;
                                users.sort_by(|a, b| a.1.cmp(&b.1));
                                let names = users
                                    .iter()
                                    .map(|(_, name)| name.as_str())
                                    .collect::<Vec<_>>();
                                let names = match names.as_slice() {
                                    [] => String::new(),
                                    [name] => (*name).to_owned(),
                                    [rest @ .., last] => {
                                        format!("{} and {last}", rest.join(", "))
                                    }
                                };
                                if !names.is_empty() {
                                    // It starts at the row's name, not at its checkbox.
                                    let mut note = ui.available_rect_before_wrap();
                                    note.min.x = name_left.max(note.min.x);
                                    ui.scope_builder(egui::UiBuilder::new().max_rect(note), |ui| {
                                        ui.label(
                                            Type::Small
                                                .rich(ui, format!("{names} use its creatures."))
                                                .color(c.muted),
                                        );
                                    });
                                }
                            }
                        });
                    }
                    design::rule(ui);
                    let needed_count = if closure.required.is_empty() {
                        String::new()
                    } else {
                        design::count(closure.required.len(), "entry", "entries")
                    };
                    if design::two_line_row(
                        ui,
                        Density::Glance.row() + design::XS,
                        self.dependencies,
                        "Needed by your entries",
                        &needed_count,
                    )
                    .clicked()
                    {
                        self.dependencies = true;
                        self.search.clear();
                    }
                });
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::M as i8,
                right: 0,
                top: design::XS as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                ui.set_clip_rect(ui.clip_rect().intersect(viewport));
                if self.dependencies {
                    ui.horizontal(|ui| {
                        ui.set_min_height(Density::Read.control());
                        design::heading(
                            ui,
                            design::Heading::Group {
                                title: "Needed by your entries",
                                count: closure.required.len(),
                            },
                        );
                    });
                    let width = ui.available_width();
                    let _ = crate::field_id!("needed_entry_search")
                        .of(active.id)
                        .search(
                            ui,
                            "Find a needed entry",
                            &mut self.search,
                            "Find an entry…",
                            width,
                        );
                    let query = self.search.to_lowercase();
                    let mut required: Vec<_> = closure
                        .required
                        .iter()
                        .filter(|(id, _)| {
                            catalogue.entries[id]
                                .entry
                                .name
                                .to_lowercase()
                                .contains(&query)
                        })
                        .collect();
                    required.sort_by_key(|(id, _)| catalogue.entries[id].entry.name.to_lowercase());
                    design::ScrollColumn::new("required_entries")
                        .fit_height()
                        .show(ui, |ui| {
                            for (id, reasons) in required {
                                let entry = &catalogue.entries[id];
                                let owner = if entry.private {
                                    "Campaign".to_owned()
                                } else {
                                    catalogue.package_name(entry.owner)
                                };
                                let detail = format!(
                                    "{} · {owner} · {}",
                                    crate::design::reader::human_label(&entry.entry.kind),
                                    needed_by(reasons.iter().cloned())
                                );
                                if design::two_line_row(
                                    ui,
                                    Density::Glance.row() + design::XS,
                                    false,
                                    &entry.entry.name,
                                    &detail,
                                )
                                .clicked()
                                {
                                    self.open_lookup = Some(*id);
                                }
                            }
                        });
                    return;
                }
                let Some(module) = catalogue
                    .modules
                    .iter()
                    .find(|m| Some(m.id) == self.module)
                    .or_else(|| catalogue.modules.first())
                else {
                    return;
                };
                self.module = Some(module.id);
                let selection_index = document
                    .selections
                    .iter()
                    .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module.id));
                let entry_ids: Vec<_> = module
                    .entries
                    .iter()
                    .copied()
                    .filter(|id| catalogue.selectable(*id))
                    .collect();
                let all = selection_index
                    .is_some_and(|i| document.selections[i].selection == Selection::All);
                ui.horizontal_wrapped(|ui| {
                    ui.set_min_height(Density::Read.control());
                    // Round 26 FA8: a wrapping row places a text job at the bottom of its first
                    // line unless the job says otherwise, so the heading sat 7 px below the
                    // mode and the actions. Centre it on the row's centre line.
                    let mut heading = design::group_heading_text(ui, &module.name, entry_ids.len());
                    for section in &mut heading.sections {
                        section.format.valign = egui::Align::Center;
                    }
                    ui.label(heading);
                    ui.label(
                        Type::Small
                            .rich(
                                ui,
                                if all {
                                    "Whole package, with future additions"
                                } else {
                                    "Chosen entries only"
                                },
                            )
                            .color(c.muted),
                    );
                    if !all
                        && Action::secondary(
                            if entry_ids.iter().all(|id| {
                                closure.direct.contains(id) || closure.required.contains_key(id)
                            }) {
                                "Include future additions"
                            } else {
                                "Choose all"
                            },
                        )
                        .density(Density::Read)
                        .blocked_if(all, "This whole package is already in use.")
                        .hint("Every entry, including future additions")
                        .show(ui)
                        .clicked()
                    {
                        if let Some(i) = selection_index {
                            document.selections[i].selection = Selection::All;
                        } else {
                            document.selections.push(PackageSelection {
                                name: Some(module.name.clone()),
                                package: EntryRef::id(module.id),
                                selection: Selection::All,
                            });
                        }
                    }
                    if Action::quiet("Choose none")
                        .density(Density::Read)
                        .blocked_if(selection_index.is_none(), "Nothing here is chosen.")
                        .show(ui)
                        .clicked()
                        && let Some(i) = selection_index
                    {
                        document.selections.remove(i);
                    }
                });
                let width = ui.available_width();
                let _ = crate::field_id!("package_entry_search")
                    .of(module.id)
                    .search(
                        ui,
                        "Find an entry in this package",
                        &mut self.search,
                        "Find an entry…",
                        width,
                    );
                let query = self.search.to_lowercase();
                if self
                    .entry_matches
                    .as_ref()
                    .is_none_or(|(module_id, cached_query, _)| {
                        *module_id != module.id || *cached_query != query
                    })
                {
                    let mut ids: Vec<_> = entry_ids
                        .iter()
                        .copied()
                        .filter(|id| {
                            catalogue.entries[id]
                                .entry
                                .name
                                .to_lowercase()
                                .contains(&query)
                        })
                        .collect();
                    ids.sort_by_cached_key(|id| catalogue.entries[id].entry.name.to_lowercase());
                    self.entry_matches = Some((module.id, query.clone(), ids));
                }
                let ids = &self.entry_matches.as_ref().unwrap().2;
                let mut toggled = None;
                design::ScrollColumn::new(("package_entries", module.id, &query))
                    .fit_height()
                    .show_rows(ui, 30.0, ids.len(), |ui, range| {
                        for index in range {
                            let id = ids[index];
                            let entry = &catalogue.entries[&id].entry;
                            let required = closure.required.contains_key(&id);
                            let mut selected = closure.direct.contains(&id) || required;
                            ui.push_id(id, |ui| {
                                let row = ui.horizontal(|ui| {
                                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                    ui.set_min_height(30.0);
                                    let why = closure.required.get(&id).map(|reasons| {
                                        reasons.iter().cloned().collect::<Vec<_>>().join(", ")
                                    });
                                    let response = ui
                                        .add_enabled(
                                            !required,
                                            egui::Checkbox::new(&mut selected, ""),
                                        )
                                        .on_hover_text(&entry.name)
                                        .on_disabled_hover_text(why.as_ref().map_or_else(
                                            || entry.name.clone(),
                                            |why| {
                                                format!(
                                                    "{} is needed by {why}, so it stays selected.",
                                                    entry.name
                                                )
                                            },
                                        ));
                                    design::named(ui, &response, &format!("Use {}", entry.name));
                                    if crate::encounters::creature_name_link(ui, &entry.name)
                                        .clicked()
                                    {
                                        self.open_lookup = Some(id);
                                    }
                                    if response.changed() {
                                        toggled = Some((id, selected));
                                    }
                                    // Round 28 FB7: one detail line, in the form of the
                                    // `Needed by your entries` list: `Rule · needed by Ember
                                    // Cultist` (`needed_by`).
                                    let subtitle =
                                        snapshot.subtitles.get(&id).cloned().unwrap_or_default();
                                    let font = Type::Small.font(ui);
                                    let mut job = egui::text::LayoutJob::default();
                                    job.append(
                                        &subtitle,
                                        0.0,
                                        egui::TextFormat::simple(font.clone(), c.muted),
                                    );
                                    if let Some(reasons) = closure.required.get(&id) {
                                        let lead = if subtitle.is_empty() { "" } else { " · " };
                                        job.append(
                                            &format!(
                                                "{lead}{}",
                                                needed_by(reasons.iter().cloned())
                                            ),
                                            0.0,
                                            egui::TextFormat::simple(font, c.document.read_only),
                                        );
                                    }
                                    ui.label(job);
                                });
                                design::register_row(ui, row.response.rect);
                            });
                        }
                    });
                if let Some((id, checked)) = toggled {
                    let selection_index = document
                        .selections
                        .iter()
                        .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module.id));
                    let mut selected =
                        match selection_index.map(|i| &document.selections[i].selection) {
                            Some(Selection::All) => entry_ids
                                .iter()
                                .map(|id| EntryRef::id(*id))
                                .collect::<Vec<_>>(),
                            Some(Selection::Selected { entries }) => entries.clone(),
                            None => Vec::new(),
                        };
                    selected.retain(|r| catalogue.resolve_ref(r).ok() != Some(id));
                    if checked {
                        selected.push(EntryRef::id(id));
                    }
                    let empty = selected.is_empty();
                    let selection = Selection::Selected { entries: selected };
                    if empty {
                        if let Some(i) = selection_index {
                            document.selections.remove(i);
                        }
                    } else if let Some(i) = selection_index {
                        document.selections[i].selection = selection;
                    } else {
                        document.selections.push(PackageSelection {
                            name: Some(module.name.clone()),
                            package: EntryRef::id(module.id),
                            selection,
                        });
                    }
                }
            });
    }
}

/// Read what the Packages page shows: invalid notes to repair, and the campaign's packages.
pub(crate) fn load_snapshot(campaign: &Campaign, modules: &[Module]) -> Loaded {
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
        })
    });
    (repairs, result)
}

/// What the DM pressed in the Packages page's footer: the result and its press, done after the
/// pane returns (`CampaignContent::press_result`).
/// `needed by Ember Cultist, Arena Final`: the one form of what keeps an entry selected, in the
/// package list's rows and in `Needed by your entries` (round 28 FB7).
fn needed_by(reasons: impl Iterator<Item = String>) -> String {
    format!("needed by {}", reasons.collect::<Vec<_>>().join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folder_recovery_requires_exactly_one_matching_identity() {
        let temp = tempfile::tempdir().unwrap();
        let old = temp.path().join("original");
        fs::create_dir(&old).unwrap();
        let text = crate::library::note_text("Test", "package", Some("test"));
        fs::write(old.join("Package.md"), &text).unwrap();
        let module = crate::library::local_module(&old).unwrap();
        assert!(relocated_package(&module, &[]).is_none());
        let moved = temp.path().join("moved");
        fs::rename(&old, &moved).unwrap();
        assert_eq!(relocated_package(&module, &[]).unwrap().path, moved);
        let copy = temp.path().join("copy");
        fs::create_dir(&copy).unwrap();
        fs::write(copy.join("Package.md"), text).unwrap();
        assert!(relocated_package(&module, &[]).is_none());
        crate::hermetic::closed::next_folder(moved.clone());
        assert_eq!(
            crate::hermetic::pick_folder_from(Some(temp.path())),
            Some(moved)
        );
        assert!(crate::hermetic::pick_folder().is_none());
    }

    fn campaign(root: &std::path::Path) -> Campaign {
        let campaign = Campaign {
            id: uuid::Uuid::new_v4(),
            name: "Test campaign".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        if campaign.path.is_dir() && !campaign.path.join("Campaign.md").exists() {
            crate::content_workspace::CampaignDocument::new(&campaign)
                .unwrap()
                .save()
                .unwrap();
        }
        campaign
    }

    fn finished(content: &mut CampaignContent, checked: Checked) {
        let (tx, rx) = mpsc::channel();
        content.reconcile.pending = Some(rx);
        tx.send(checked).unwrap();
        content.finish_check();
    }

    #[test]
    fn iter8_e_campaign_deleted_manifest_action_does_not_guess_a_filename() {
        for filename in ["Package.md", "Module.md"] {
            let temp = tempfile::tempdir().unwrap();
            let root = temp.path().join("campaign");
            fs::create_dir(&root).unwrap();
            let active = campaign(&root);
            let package = temp.path().join("package");
            fs::create_dir(&package).unwrap();
            fs::write(
                package.join("Package.md"),
                crate::library::note_text("Test", "package", Some("test")),
            )
            .unwrap();
            let module = crate::library::local_module(&package).unwrap();
            if filename == "Module.md" {
                fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
            }
            let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
            campaigns.register_module(module.clone()).unwrap();
            fs::remove_file(module.path.join(filename)).unwrap();
            let mut content = CampaignContent {
                snapshot: Some(load_snapshot(&active, &[module]).1.unwrap()),
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
                            // Round 26 FD1: the page's footer draws the warnings.
                            let frame = design::outcome::PaneFrame::new(ui);
                            let mut messages =
                                design::outcome::Messages::new(&frame, "warnings", 800.0);
                            let unavailable = content.package_warnings_of(&campaigns).unavailable;
                            content.package_messages(&mut messages, &campaigns, &unavailable);
                            // The footer, then the package list beside it (a Content pane
                            // refuses the list's own scroll column).
                            let _ = messages.pane(ui, design::Body::Content, |_| ());
                            content.packages_content(ui, &campaigns, &active);
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
            assert!(rendered.contains("Write the manifest again"), "{rendered}");
            assert!(
                rendered.contains("The package manifest was deleted."),
                "{rendered}"
            );
            assert!(!rendered.contains("Missing YAML frontmatter"), "{rendered}");
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
    fn package_counts_share_one_form() {
        // Round 24 FF2: `<All, None or N of M> chosen`, then the entries.
        assert_eq!(
            package_count(2065, 2065, false),
            "All chosen · 2,065 entries"
        );
        assert_eq!(package_count(0, 6, false), "None chosen · 6 entries");
        // Round 27 FA6: a needed entry is ticked, so it counts as chosen. Four chosen and one
        // needed of six read `5 of 6 chosen`; five chosen and the one needed read `All chosen`.
        assert_eq!(package_count(4 + 1, 6, true), "5 of 6 chosen");
        assert_eq!(package_count(5 + 1, 6, false), "All chosen · 6 entries");
        // An off package whose three entries others need: three of its boxes are ticked.
        assert_eq!(package_count(3, 6, true), "3 of 6 chosen");
        // Every entry needed, none chosen: the box is not fully ticked, so not `All chosen`.
        assert_eq!(package_count(6, 6, true), "6 of 6 chosen");
        assert_eq!(package_count(0, 0, false), "None chosen");
    }

    #[test]
    fn an_unclosed_list_names_and_marks_its_opening_line() {
        let text = "---\nsplat_schema: 2\nsplat_kind: [\n---\n";
        let (problem, line) = repair_problem("Notes/Broken.md", text);
        assert_eq!(line, Some(3));
        assert_eq!(
            problem.as_deref(),
            Some("Line 3 opens a list with [ that never closes.")
        );
        // Round 23 FA3: the save row asks Save's own question, the one World's repair asks. A
        // draft Save refuses names its cause; one Save writes has none.
        let active = campaign(std::path::Path::new("/nonexistent/splat-campaign"));
        let refusal = |draft: &str| Repair::new("Notes/Broken.md".into(), draft.into(), &active);
        assert_eq!(
            refusal(text).refusal.as_deref(),
            Some("Line 3 opens a list with [ that never closes.")
        );
        assert_eq!(refusal(text).error_line, Some(3));
        let rule = crate::library::note_text("Broken", "rule", None);
        assert_eq!(refusal(&rule).refusal, None);
        // A draft that parses but takes an identity settings own is still refused.
        let ownership = Some("Keep campaign ownership consistent with settings.".to_owned());
        let package = crate::library::note_text("Broken", "package", Some("broken"));
        assert_eq!(refusal(&package).refusal, ownership);
        let other = crate::library::note_text("Broken", "campaign", Some("broken"));
        assert_eq!(refusal(&other).refusal, ownership);
        let own = other.replace(
            &splat::content::Entry::parse("Campaign.md", &other)
                .unwrap()
                .id
                .to_string(),
            &active.id.to_string(),
        );
        assert_eq!(refusal(&own).refusal, None);
        // Round 23 review: the check runs when the draft changes, and the change ends this
        // repair's failed Save.
        let mut repair = refusal(&rule);
        repair.failure = Some("This file is read only.".into());
        let mut checks = 0;
        repair.draft = package.clone();
        repair.draft_changed(&active, &mut checks);
        assert_eq!(
            (repair.refusal, repair.failure, checks),
            (ownership.clone(), None, 1)
        );
        // Round 24 SA1, FA1: Save asks again before it writes. A refusal found then (here the
        // note now belongs to another campaign) is Invalid with its cause, not `Not saved.`.
        let mut repair = refusal(&own);
        assert_eq!(repair.refusal, None);
        let elsewhere = Campaign {
            id: uuid::Uuid::new_v4(),
            ..active.clone()
        };
        assert!(!repair.save(&elsewhere));
        assert_eq!((repair.refusal, repair.failure), (ownership, None));
        // A write that fails on the disk is the failed Save, and only it.
        let mut repair = refusal(&rule);
        assert!(!repair.save(&active));
        assert_eq!(repair.refusal, None);
        assert!(repair.failure.is_some(), "{:?}", repair.failure);
    }

    #[test]
    fn a_check_reaches_the_sidebar_and_the_review_list_and_changes_nothing() {
        let temp = tempfile::tempdir().unwrap();
        let active = campaign(temp.path());
        let campaigns = Campaigns::at(temp.path().join("settings.json"));
        let mut content = CampaignContent::default();
        assert_eq!(content.review_detail(), None);
        let proposal = |key: &str, blocked: Option<&str>| Proposal {
            key: key.into(),
            name: "Witch Bolt".into(),
            kind: "spell".into(),
            change: Move::ToPackage("Player's Handbook".into()),
            blocked: blocked.map(str::to_owned),
        };
        finished(
            &mut content,
            Checked {
                applied: None,
                proposals: Ok(vec![
                    proposal("a", None),
                    proposal("b", Some("It has personal notes.")),
                ]),
            },
        );
        assert_eq!(content.review_detail().as_deref(), Some("2 to review"));
        assert!(content.reconcile.pending.is_none());
        assert!(!content.refresh_pending);
        // Showing the list starts nothing. Only the DM's click applies a change.
        let _ = egui::Context::default().run(Default::default(), |ctx| {
            egui::CentralPanel::default()
                .show(ctx, |ui| content.reconcile_section(ui, &campaigns, &active));
        });
        assert!(content.reconcile.pending.is_none());
    }

    #[test]
    fn applied_changes_reload_the_catalogue_and_failures_are_shown() {
        let mut content = CampaignContent::default();
        finished(
            &mut content,
            Checked {
                applied: Some(Ok(2)),
                proposals: Ok(Vec::new()),
            },
        );
        assert!(content.refresh_pending);
        assert_eq!(content.review_detail(), None);
        assert!(
            applied_words(content.reconcile.applied.unwrap())
                .as_str()
                .starts_with("2 changes")
        );
        finished(
            &mut content,
            Checked {
                applied: Some(Err("A sheet has local edits.".into())),
                proposals: Ok(Vec::new()),
            },
        );
        assert_eq!(
            content.reconcile.error.as_deref(),
            Some("A sheet has local edits.")
        );
    }

    #[test]
    fn closure_follows_a_selection_change() {
        let temp = tempfile::tempdir().unwrap();
        let active = campaign(temp.path());
        let campaigns = Campaigns::at(temp.path().join("settings.json"));
        let catalogue = Catalogue::load(&[], None).unwrap();
        let document = CampaignDocument::load(&active).unwrap();
        let closure = catalogue.closure(&document.selections, active.id);
        let mut content = CampaignContent {
            active: Some(active.clone()),
            snapshot: Some(Snapshot {
                closure: (document.selections.clone(), closure),
                document,
                subtitles: crate::library::catalogue_subtitles(&catalogue),
                catalogue,
            }),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let frame = |content: &mut CampaignContent| {
            let _ = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    content.selection_bar(ui, &campaigns, &active);
                    content.packages(ui, &campaigns, &active);
                });
            });
        };
        let kept =
            |content: &CampaignContent| content.snapshot.as_ref().unwrap().closure.0.as_ptr();
        let before = kept(&content);
        frame(&mut content);
        assert_eq!(
            kept(&content),
            before,
            "the same selections keep the closure"
        );
        let direct =
            |content: &CampaignContent| content.snapshot.as_ref().unwrap().closure.1.direct.len();
        assert!(
            direct(&content) > 0,
            "a new campaign uses the built-in package"
        );
        let snapshot = content.snapshot.as_mut().unwrap();
        snapshot.document.selections.clear();
        frame(&mut content);
        assert_eq!(direct(&content), 0);
    }

    #[test]
    fn undo_discard_stays_in_its_campaign() {
        let temp = tempfile::tempdir().unwrap();
        let (first, second) = (
            campaign(&temp.path().join("a")),
            campaign(&temp.path().join("b")),
        );
        let campaigns = Campaigns::at(temp.path().join("settings.json"));
        let ctx = egui::Context::default();
        let mut content = CampaignContent {
            active: Some(first.clone()),
            discarded_selections: Some((Vec::new(), None)),
            // Round 26: two repairs' discards, each with its own Undo.
            discarded_repairs: vec![
                ("note.md".into(), "words".into(), 0),
                ("other.md".into(), "more".into(), 1),
            ],
            ..Default::default()
        };
        // Wave 2 d (C): each discard is a kept result with its own Undo, in the store's order.
        // A (note.md), then B (other.md); Undo of A in the real footer restores A only: B's
        // result and its Undo stay.
        new_result!(content, PageResult::RepairDiscard(0));
        new_result!(content, PageResult::RepairDiscard(1));
        {
            use egui_kittest::kittest::Queryable;
            let active = first.clone();
            let campaigns = Campaigns::at(temp.path().join("settings.json"));
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(960.0, 640.0))
                .build_ui_state(
                    move |ui, content: &mut CampaignContent| {
                        let frame = design::outcome::PaneFrame::new(ui);
                        let messages = content.page_footer(&frame, ui, &campaigns, &active, 600.0);
                        let paned = messages.pane(ui, design::Body::Content, |_| ());
                        content.press_result(ui.ctx(), &campaigns, &active, paned.pressed);
                    },
                    content,
                );
            harness.run_steps(2);
            assert!(harness.query_by_label("Undo discard of other.md").is_some());
            harness.get_by_label("Undo discard of note.md").click();
            harness.run_steps(3);
            let kept: Vec<_> = harness
                .state()
                .discarded_repairs
                .iter()
                .map(|(path, ..)| path.clone())
                .collect();
            assert_eq!(kept, ["other.md"], "Undo of A ends A only");
            assert!(
                harness.query_by_label("Undo discard of note.md").is_none()
                    && harness.query_by_label("Undo discard of other.md").is_some(),
                "B keeps its result and its Undo"
            );
            content = std::mem::take(harness.state_mut());
        }
        content.discarded_repairs = vec![
            ("note.md".into(), "words".into(), 0),
            ("other.md".into(), "more".into(), 1),
        ];
        content.reload(&ctx, &campaigns, first);
        assert!(content.discarded_selections.is_some(), "a refresh keeps it");
        assert_eq!(content.discarded_repairs.len(), 2, "a refresh keeps both");
        content.reload(&ctx, &campaigns, second);
        assert!(content.discarded_selections.is_none());
        assert!(content.discarded_repairs.is_empty());

        // Round 23 FA8: the discard belongs to Campaign.md as it was read. The same file keeps
        // it; a changed or missing file ends it.
        let folder = temp.path().join("c");
        fs::create_dir(&folder).unwrap();
        let active = campaign(&folder);
        let document = CampaignDocument::load(&active).unwrap();
        let discarded = || Some((Vec::new(), document.baseline.clone()));
        content.discarded_selections = discarded();
        content.forget_discard_of_changed_file(Ok(&CampaignDocument::load(&active).unwrap()));
        assert!(
            content.discarded_selections.is_some(),
            "the same file keeps it"
        );
        let file = active.path.join(&document.path);
        fs::write(
            &file,
            format!("{}\nA later note.\n", fs::read_to_string(&file).unwrap()),
        )
        .unwrap();
        content.forget_discard_of_changed_file(Ok(&CampaignDocument::load(&active).unwrap()));
        assert!(
            content.discarded_selections.is_none(),
            "a changed file ends it"
        );
        content.discarded_selections = discarded();
        fs::remove_file(&file).unwrap();
        let missing = CampaignDocument::load(&active).map(|_| ()).unwrap_err();
        content.forget_discard_of_changed_file(Err(&missing));
        assert!(
            content.discarded_selections.is_none(),
            "a missing file ends it"
        );
    }
    /// AD1: `Undo package removal` goes when the file's package choices changed after it.
    #[test]
    fn iter14_e_changed_choices_end_undo_package_removal() {
        use crate::content_workspace::{CampaignDocument, EntryRef, PackageSelection, Selection};
        let temp = tempfile::tempdir().unwrap();
        for folder in ["a", "b"] {
            fs::create_dir(temp.path().join(folder)).unwrap();
        }
        let active = campaign(&temp.path().join("a"));
        let other = campaign(&temp.path().join("b"));
        let selection = |name: &str| PackageSelection {
            name: Some(name.into()),
            package: EntryRef::id(uuid::Uuid::new_v4()),
            selection: Selection::All,
        };
        let mut document = CampaignDocument::load(&active).unwrap();
        let before = vec![selection("Gone")];
        document.selections = Vec::new();
        document.save().unwrap();
        let removed = |campaign: &Campaign| RemovedPackage {
            campaign: campaign.id,
            name: "Gone".into(),
            words: removal_words("Gone", &[]).0,
            warning: None,
            before: before.clone(),
            after: Vec::new(),
            users: Vec::new(),
        };
        let mut content = CampaignContent {
            removed_package: Some(removed(&active)),
            ..Default::default()
        };
        // A reload of the file as the removal left it keeps Undo, and other edits keep it too.
        let file = active.path.join(&document.path);
        let text = format!("{}\nA later note.\n", fs::read_to_string(&file).unwrap());
        fs::write(&file, &text).unwrap();
        content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
        assert!(content.removed_package.is_some());
        // The file's package choices changed outside Splat: the result and its Undo go.
        let mut changed = CampaignDocument::load(&active).unwrap();
        changed.selections.push(selection("Added in Obsidian"));
        changed.save().unwrap();
        let bytes = fs::read(&file).unwrap();
        content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
        assert!(content.removed_package.is_none());
        assert_eq!(fs::read(&file).unwrap(), bytes, "the check writes nothing");
        // A removal from another campaign is not judged by this campaign's file.
        content.removed_package = Some(removed(&other));
        content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
        assert!(content.removed_package.is_some());
    }

    #[test]
    fn finding_a_package_rejects_another_packages_folder() {
        let dir = tempfile::tempdir().unwrap();
        let expected = dir.path().join("expected");
        let other = dir.path().join("other");
        fs::create_dir(&expected).unwrap();
        fs::create_dir(&other).unwrap();
        let text = crate::library::note_text("Ashen Marches", "package", Some("ashen"));
        let entry = write_note(&expected, "Package.md", None, &text).unwrap();
        let text = crate::library::note_text("Other package", "package", Some("other"));
        write_note(&other, "Package.md", None, &text).unwrap();
        let reference = EntryRef::id(entry.id);
        assert_eq!(
            find_package_folder(&reference, Some("Ashen Marches"), &expected)
                .unwrap()
                .name,
            "Ashen Marches"
        );
        // Round 23 FE3: the fault comes first and names the picked folder and the package.
        let folder = other.file_name().unwrap().to_string_lossy().into_owned();
        assert_eq!(
            find_package_folder(&reference, Some("Ashen Marches"), &other).unwrap_err(),
            format!(
                "The folder {folder}/ holds a different package. Choose the folder that holds Ashen Marches."
            )
        );
    }
}
