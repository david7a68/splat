//! The shared catalogue of built-in, package, and campaign entries.
#[path = "class.rs"]
pub mod class;
#[path = "creature.rs"]
pub mod creature;
#[path = "presentation.rs"]
pub mod presentation;
#[path = "reader.rs"]
pub mod reader;
#[path = "spells.rs"]
pub mod spells;
#[path = "view.rs"]
pub mod view;
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue},
    design::{
        self, Tone, icons,
        outcome::{Choice, FaultMessage, MessageChoice, Messages, Notice, Sentence, UndoStep},
    },
    matching::{Rank, Typed, with_other},
    persistence::{Campaign, Module},
    world::file_fault::FileFault,
};
use eframe::egui;
use presentation::Section;
use splat::content::{
    self, Entry,
    creature_filter::{Facets, Query},
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub const BUILTIN_NAME: &str = "D&D 2024 · SRD 5.2.1";

/// The choice after a package copy's registration names what it changes, in its result's words.
pub const USE_COPY: &str = "Use the copy in encounters";
pub const USE_COPY_RESULT: Sentence = crate::sentence!("Encounters now use the copy.");

/// Why source editing is blocked for a file that the system will not read.
pub const UNREAD_EDIT: design::outcome::Sentence =
    splat::sentence!("Splat cannot read this file. Fix it outside Splat, then choose Try again.");

/// A file that cannot be read: what happened, how to recover, and Try again, which starts at the
/// message words. The system's words go to Technical details. Nothing here changes the file.
/// Returns whether Try again was chosen.
/// A read fault's first sentence in the reader, below the file line that names the file: it
/// says `this file`, so the path shows once. Messages away from the reader keep the path.
fn fault_here(fault: &FileFault) -> String {
    use crate::world::file_fault::Kind;
    match fault.kind {
        Kind::NotText => "Splat cannot read this file as text.".to_owned(),
        Kind::ReadDenied => "Splat has no permission to read this file.".to_owned(),
        Kind::Gone => "Splat cannot find this file.".to_owned(),
        Kind::CannotRead => "Splat cannot open this file.".to_owned(),
        Kind::WriteDenied | Kind::ReadOnly | Kind::Exists | Kind::DiskFull | Kind::CannotWrite => {
            fault.headline()
        }
    }
}

/// What is wrong with a repair draft now, and the line to mark. Any reason Save would refuse the
/// draft (`refusal`, from `Loaded::draft_refusal`) is the notice, a YAML fault in its own words. A
/// saved file whose own text does not parse has a parse fault: a draft Save would write has no
/// notice. Another fault (a read fault, a deleted manifest, a manifest with another identity) is
/// not in the draft's text, so its stored words stay until the file changes.
pub fn draft_repair_problem(
    note: &Note,
    draft: &str,
    refusal: Option<String>,
    recovery: Option<&(Module, String, bool)>,
) -> (Option<String>, Option<usize>) {
    let (problem, line) = crate::content_workspace::repair_problem(&note.path, draft);
    // A file the system will not read has no text to repair: its reader names the read fault.
    if note.fault.is_some() {
        return (None, line);
    }
    let Some(error) = note.error.as_ref() else {
        // A readable entry's draft: Save's own refusal, with a YAML fault in its own words.
        return (refusal.map(|refusal| problem.unwrap_or(refusal)), line);
    };
    if recovery.is_some_and(|(_, _, exists)| !exists) {
        return (Some(error.clone()), line);
    }
    let parse_fault = || {
        crate::content_workspace::repair_problem(&note.path, &note.text)
            .0
            .is_some()
    };
    match problem.or(refusal) {
        Some(problem) => (Some(problem), line),
        None if parse_fault() => (None, line),
        None => (Some(error.clone()), line),
    }
}

/// Whose file a note is: its owner's kind and id. Settings check package and campaign UUIDs
/// apart (`persistence.rs`), so a campaign and a package may share one: the kind is part of a
/// file's identity (M-L0.4a). Every file key in the Library is a [`FileId`].
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum OwnerKey {
    #[default]
    Builtin,
    Package(Uuid),
    Campaign(Uuid),
}

/// One file in the Library: its owner (kind and id) and its path in the owner's folder. Two
/// packages can each have `Rules/Watch.md`, and so can a campaign and a package with one UUID:
/// they are different files.
pub type FileId = (OwnerKey, String);

/// Who owns an entry, and so whether and where it can be saved.
#[derive(Clone, Default, PartialEq, Debug)]
pub enum Owner {
    #[default]
    Builtin,
    Package(Module),
    Campaign(Campaign),
}

impl Owner {
    pub fn folder(&self) -> String {
        self.root()
            .and_then(Path::file_name)
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    }
    pub fn id(&self) -> Option<Uuid> {
        match self {
            Self::Builtin => None,
            Self::Package(module) => Some(module.id),
            Self::Campaign(campaign) => Some(campaign.id),
        }
    }
    /// Whether this is the package `id`. A campaign may have the same UUID as a package
    /// (settings check package and campaign UUIDs apart), so the kind is part of the check: a
    /// UUID alone never stands for a package (issue #505, M-L0.5).
    pub fn is_package(&self, id: Uuid) -> bool {
        matches!(self, Self::Package(module) if module.id == id)
    }
    /// The owner's kind and id: the owner part of a [`FileId`].
    pub fn key(&self) -> OwnerKey {
        match self {
            Self::Builtin => OwnerKey::Builtin,
            Self::Package(module) => OwnerKey::Package(module.id),
            Self::Campaign(campaign) => OwnerKey::Campaign(campaign.id),
        }
    }
    pub fn root(&self) -> Option<&Path> {
        match self {
            Self::Builtin => None,
            Self::Package(module) => Some(&module.path),
            Self::Campaign(campaign) => Some(&campaign.path),
        }
    }
    pub fn editable(&self) -> bool {
        !matches!(self, Self::Builtin)
    }
    pub fn label(&self) -> String {
        match self {
            Self::Builtin => "Built-in package · Read only".into(),
            Self::Package(module) => format!("Package · {}", module.name),
            Self::Campaign(_) => "Campaign".into(),
        }
    }
    /// Where an entry came from.
    pub fn source(&self) -> &str {
        match self {
            Self::Builtin => BUILTIN_NAME,
            Self::Package(module) => &module.name,
            Self::Campaign(_) => "Campaign",
        }
    }
}

pub fn name_list(names: &[String]) -> String {
    let shown = &names[..names.len().min(3)];
    if names.len() > 3 {
        return format!("{} and {} more", shown.join(", "), names.len() - 3);
    }
    match shown {
        [] => String::new(),
        [name] => name.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// NPC roster rows keep their stat block and display name, rather than the NPC identity.
pub fn encounter_uses_entry(encounter: &Entry, entry: &Entry) -> bool {
    if content::required_references(encounter)
        .is_ok_and(|refs| refs.contains(&content::Reference::Id(entry.id)))
    {
        return true;
    }
    let Some(world) = entry
        .fields
        .get("splat_world")
        .filter(|_| entry.kind == "world" && crate::world::fights(entry))
    else {
        return false;
    };
    let Some(creature) = world
        .get("creature")
        .and_then(|creature| creature.get("id"))
        .and_then(serde_yaml_ng::Value::as_str)
    else {
        return false;
    };
    encounter
        .fields
        .get("splat_encounter")
        .and_then(|setup| setup.get("roster"))
        .and_then(serde_yaml_ng::Value::as_sequence)
        .is_some_and(|roster| {
            roster.iter().any(|slot| {
                slot.get("creature")
                    .and_then(|reference| reference.get("id"))
                    .and_then(serde_yaml_ng::Value::as_str)
                    == Some(creature)
                    && slot
                        .get("overrides")
                        .and_then(|changes| changes.get("name"))
                        .and_then(serde_yaml_ng::Value::as_str)
                        == Some(entry.name.as_str())
            })
        })
}

/// What a press in a Library typed footer asks for (M-L0.4b). A kept result's press comes back
/// with its ticket, which names the entry its line showed: the act runs only while that entry
/// stands, so a press never reaches a newer result of the same kind or owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LibraryPress {
    /// The Undo of a result in the Library's own store (`LibraryKept`): the ticket gives its kind.
    Undo,
    /// The link offer's `Update <n> links`: a kept offer line (`KeptOutcome::Offer`), so its
    /// press comes back with the offer's ticket.
    UpdateLinks,
    /// The link update's Undo.
    UndoLinks,
    /// The registration's Undo.
    UndoRegistration,
    /// `Use the copy in encounters`, on the registration's line: the registration's ticket.
    UseCopy,
    /// The source change's Undo.
    UndoRosters,
    /// The first heading's fix, `Use <name>`.
    FixHeading,
    /// A stat line's fix, `Use <value>`: its stat's key and label, and the value it writes.
    UseVital(&'static str, &'static str, i64),
    /// A broken manifest's recovery (`Write the manifest again`, `Replace with a basic …`).
    Repair,
    /// `Make <file> a separate entry`: that file's note.
    SeparateEntry(usize),
    /// `Make all N files in <folder>/ separate entries`: a note of that folder's owner.
    SeparateEntries(usize),
    /// `Copy <entry> to <package>`: the campaign entry the open package entry links to.
    CopyDependency(Uuid),
}

/// The heading fix's result: its Undo.
pub const HEADING_CHANGE_UNDO: UndoStep = UndoStep::words("Undo heading change");
/// The stat fix's result: its Undo.
pub const STAT_CHANGE_UNDO: UndoStep = UndoStep::words("Undo stat change");

/// A broken manifest's recovery choice, locked while the draft holds it. Replacing a manifest
/// that is there says what it keeps and replaces, in its help line (M8).
pub fn recovery_choice(path: &str, exists: bool, lock: Option<Sentence>) -> Choice<LibraryPress> {
    let choice = if exists {
        Choice::named(
            crate::choice!("Replace with a basic {}", path),
            LibraryPress::Repair,
        )
        .help(manifest_replace_help(path))
    } else {
        Choice::new(
            MessageChoice::Choice("Write the manifest again"),
            LibraryPress::Repair,
        )
    };
    match lock {
        Some(reason) => choice.blocked(reason),
        None => choice,
    }
}

/// What replacing a manifest keeps and replaces (`content_workspace`'s words, checked).
pub fn manifest_replace_help(path: &str) -> Sentence {
    crate::sentence!(
        "This keeps the package name and its links. It replaces the other contents of {}, including its description. The entries stay unchanged.",
        path
    )
}

/// What a duplicate repair choice leaves each encounter that uses the entry with, as its help
/// line (M8): the shared form (`world::duplicate_consequence`, M0178).
fn duplicate_consequence(users: &[String], remaining: &str, keeps: bool) -> Sentence {
    crate::world::duplicate_consequence(crate::world::KeptBy::Encounters(users), remaining, keeps)
}

/// The duplicate warning and its repair choices: one per file, and one per folder whose files
/// all repeat another folder's, each with its consequence in its help line (M8), locked while
/// the draft holds them. The encounters a folder's batch leaves on the other folder are found
/// once per load (an egui cache keyed by the whole owner key and the notes).
pub fn duplicate_notice(
    ctx: &egui::Context,
    loaded: &Loaded,
    note: &Note,
    duplicate_indices: &[usize],
    lock: Option<Sentence>,
) -> Notice<LibraryPress> {
    let locked = |choice: Choice<LibraryPress>| match &lock {
        Some(reason) => choice.blocked(reason.clone()),
        None => choice,
    };
    // M0178: the shared fault's words (`world::DuplicateIdentity`), as World and Encounters say
    // them.
    let mut notice = Notice::warning(design::outcome::Fault::cause(
        &crate::world::DuplicateIdentity(crate::world::IdentityNoun::Entry),
    ));
    let users: Vec<_> = loaded
        .notes
        .iter()
        .filter_map(|other| {
            let entry = other.entry.as_ref()?;
            (entry.kind == "encounter" && encounter_uses_entry(entry, note.entry.as_ref()?))
                .then(|| entry.name.clone())
        })
        .collect();
    let in_folder = |at: usize| {
        let target = &loaded.notes[at];
        let filename = Path::new(&target.path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let same_filename = duplicate_indices.iter().any(|&other| {
            other != at
                && Path::new(&loaded.notes[other].path).file_name()
                    == Path::new(&target.path).file_name()
        });
        if same_filename {
            format!("{filename} in {}/", target.owner.folder())
        } else {
            filename.into_owned()
        }
    };
    for (position, &at) in duplicate_indices.iter().enumerate() {
        let choice = locked(Choice::named(
            crate::world::separate_choice(crate::world::IdentityNoun::Entry, &in_folder(at)),
            LibraryPress::SeparateEntry(at),
        ));
        let remaining = duplicate_indices.iter().copied().find(|&other| other != at);
        notice = notice.choice(match remaining.filter(|_| !users.is_empty()) {
            Some(remaining) => choice.help(duplicate_consequence(
                &users,
                &in_folder(remaining),
                position == 0,
            )),
            None => choice,
        });
    }
    let mut shown = std::collections::HashSet::new();
    for (position, &at) in duplicate_indices.iter().enumerate() {
        let owner = &loaded.notes[at].owner;
        if !shown.insert(owner.key()) {
            continue;
        }
        let Some(paths) = loaded.duplicate_folder_paths(owner) else {
            continue;
        };
        // Encounters outside the batch whose source changes; worked out once per load.
        let slot = egui::Id::new((
            "duplicate_folder_users",
            // The whole owner key: a campaign and a package with one UUID each have their own
            // encounters outside their batch (Astra, PR 477).
            owner.key(),
            std::ptr::from_ref(loaded) as usize,
            loaded.notes.len(),
            // A reload in place changes some text; this cheap sum notices it.
            loaded
                .notes
                .iter()
                .map(|note| note.text.len())
                .sum::<usize>(),
        ));
        let users = ctx
            .data(|data| data.get_temp::<Vec<String>>(slot))
            .unwrap_or_else(|| {
                let batch: std::collections::HashSet<&String> = paths.iter().collect();
                let targets: Vec<&Entry> = loaded
                    .notes
                    .iter()
                    .filter(|target| {
                        target.owner.key() == owner.key() && batch.contains(&target.path)
                    })
                    .filter_map(|target| target.entry.as_ref())
                    .collect();
                let mut users: Vec<String> = Vec::new();
                for other in &loaded.notes {
                    // An encounter inside the batch gets the new identities with it.
                    if other.owner.key() == owner.key() && batch.contains(&other.path) {
                        continue;
                    }
                    let Some(encounter) = other.entry.as_ref() else {
                        continue;
                    };
                    if encounter.kind == "encounter"
                        && !users.contains(&encounter.name)
                        && targets
                            .iter()
                            .any(|entry| encounter_uses_entry(encounter, entry))
                    {
                        users.push(encounter.name.clone());
                    }
                }
                ctx.data_mut(|data| data.insert_temp(slot, users.clone()));
                users
            });
        let choice = locked(Choice::named(
            crate::choice!(
                "Make all {} files in {}/ separate entries",
                paths.len(),
                owner.folder()
            ),
            LibraryPress::SeparateEntries(at),
        ));
        let remaining = duplicate_indices
            .iter()
            .map(|&other| &loaded.notes[other].owner)
            .find(|other| other.key() != owner.key());
        notice = notice.choice(match remaining.filter(|_| !users.is_empty()) {
            Some(remaining) => choice.help(duplicate_consequence(
                &users,
                &format!("the files in {}/", remaining.folder()),
                position == 0,
            )),
            None => choice,
        });
    }
    notice
}

/// The page's last error (`Library::error`, the New entry form's `new_error`): the app's own
/// words, a checked sentence, or words from outside the app (the system's, the workspace's,
/// the content layer's), which only the typed footer's escape takes (ruling M17).
#[derive(Clone, Debug, PartialEq)]
pub enum PageError {
    Own(Sentence),
    Outside(String),
    /// The app's own words, then the outside words that say why (`because`).
    Because {
        own: Sentence,
        cause: String,
        words: String,
    },
}

impl From<String> for PageError {
    fn from(words: String) -> Self {
        Self::Outside(words)
    }
}

impl PageError {
    /// The app's own words `own`, then the outside words `cause`.
    pub fn because(own: &Sentence, cause: String) -> Self {
        Self::Because {
            words: format!("{own} {cause}"),
            own: own.clone(),
            cause,
        }
    }

    /// Its words, for the tests.
    #[cfg(any(test, feature = "test-support"))]
    pub fn words(&self) -> &str {
        match self {
            Self::Own(words) => words.as_str(),
            Self::Outside(words) | Self::Because { words, .. } => words,
        }
    }
}

/// Why `Undo package choice` wrote nothing: the campaign's choices changed since (note §4).
pub static CHOICES_CHANGED: Sentence = crate::sentence!(
    "Campaign choices changed. Keep them and edit the package choices in Campaign."
);
/// Why `Undo unregister` wrote nothing: its folder holds another package now.
pub static FOLDER_HOLDS_ANOTHER_PACKAGE: Sentence =
    crate::sentence!("That folder now holds a different package.");
/// Why loading stopped before it finished.
pub static LOADING_STOPPED: Sentence =
    crate::sentence!("Library loading stopped. Reload to retry.");
/// Why a new entry made in a package is not chosen for the open campaign (its cause follows).
pub static ENTRY_NOT_CHOSEN: Sentence =
    crate::sentence!("Entry created. Splat could not use it in this campaign.");
/// Why a save's search for its moved file stopped: one folder has two files with its ID.
pub static SEVERAL_FILES_KEEP_ONE: Sentence =
    crate::sentence!("Several files have this entry’s ID. Keep one file, then try again.");
/// Why a save's search for its moved file stopped: two folders have a file with its ID.
pub static SEVERAL_FILES_CHOOSE_ONE: Sentence =
    crate::sentence!("Several files have this entry’s ID. Choose one in Library.");
/// Why a new entry was not made: its name.
pub static NAME_WITHOUT_SEPARATORS: Sentence =
    crate::sentence!("Use a name without path separators or line breaks.");

/// A fault of the Library page, as its typed footer states it.
pub enum LibraryFault<'a> {
    /// The app's own words, checked.
    Own(&'a Sentence),
    /// Words from outside the app: the system's or the workspace's.
    Error(&'a str),
    /// The app's own words, then the outside words that say why.
    Because(&'a Sentence, &'a str),
    /// A repair draft's fault (the content layer's words, a YAML fault), or Save's refusal
    /// before the write, in the tone its diagnosis gives (a deleted manifest is a warning).
    Repair(&'a str, Tone),
    /// The disk's failure of the last Save of this draft (the system's words).
    NotSaved(&'a str),
    /// The link Campaign sent the DM here to fix: its line and the content layer's words for it,
    /// in the form of the broken-link lines beside it (`Line 21: …`).
    Link { line: usize, words: &'a str },
    /// The app's own warning, checked (a file changed on disk, a copied package folder).
    Warning(&'a Sentence),
    /// A file that cannot be read: what is wrong and its recovery (World's `FileFault` words),
    /// and whether a draft of it waits (`Your draft is safe here until you choose.`).
    Unread(&'a FileFault, bool),
}

impl<'a> From<&'a PageError> for LibraryFault<'a> {
    fn from(error: &'a PageError) -> Self {
        match error {
            PageError::Own(words) => Self::Own(words),
            PageError::Outside(words) => Self::Error(words),
            PageError::Because { own, cause, .. } => Self::Because(own, cause),
        }
    }
}

impl design::outcome::Fault for LibraryFault<'_> {
    fn severity(&self) -> design::outcome::Severity {
        use design::outcome::Severity;
        match self {
            Self::Repair(_, Tone::Warning) | Self::Link { .. } | Self::Warning(_) => {
                Severity::NeedsFixing
            }
            Self::Repair(..) | Self::Unread(..) => Severity::CannotRead,
            // An act's write or read did not work: the error tone, as the legacy line had.
            Self::Own(_) | Self::Error(_) | Self::Because(..) | Self::NotSaved(_) => {
                Severity::NotSaved
            }
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "the page’s error carries the system’s or the workspace’s words (ruling M17)"
    )]
    fn cause(&self) -> Sentence {
        match self {
            Self::Own(words) | Self::Warning(words) => (*words).clone(),
            // World's words for a read fault and its recovery, which the Library shows too.
            Self::Unread(fault, draft) => Sentence::outside(if *draft {
                format!(
                    "{} Your draft is safe here until you choose. {}",
                    fault_here(fault),
                    fault.recovery_with_try_again()
                )
            } else {
                format!("{} {}", fault_here(fault), fault.recovery_with_try_again())
            })
            .unwrap_or(crate::sentence!("Splat cannot read this file.")),
            Self::Because(own, cause) => match Sentence::outside(*cause) {
                Some(cause) => (*own).clone().then(cause),
                None => (*own).clone(),
            },
            // `Error:` leads, as before; the words after it keep their spelling (a path).
            Self::Error(words) => Sentence::outside_after(crate::lead!("Error:"), *words)
                .unwrap_or(crate::sentence!("Something went wrong.")),
            Self::Repair(words, _) => {
                Sentence::outside(*words).unwrap_or(crate::sentence!("This file needs fixing."))
            }
            Self::NotSaved(words) => {
                Sentence::outside(*words).unwrap_or(crate::sentence!("The draft was not saved."))
            }
            Self::Link { line, words } => {
                let fix = crate::sentence!(
                    "Replace the link on this line with another creature, or remove its row."
                );
                match Sentence::outside(format!("Line {line}: {words}")) {
                    Some(words) => words.then(fix),
                    None => fix,
                }
            }
        }
    }
}

/// The page's errors as typed footer faults: a form's own error first, then the page's.
pub fn page_faults(messages: &mut Messages<'_, LibraryPress>, errors: &[Option<&PageError>]) {
    for error in errors.iter().flatten() {
        messages.outcome(FaultMessage::new(&LibraryFault::from(*error)));
    }
}

/// A copied package's registration: its result's Undo.
pub const PACKAGE_REGISTRATION_UNDO: UndoStep = UndoStep::words("Undo package registration");
/// `Use the copy in encounters`: its result's Undo.
pub const SOURCE_CHANGE_UNDO: UndoStep = UndoStep::words("Undo source change");
/// One file made a separate entry from Library's duplicate warning: its result's Undo. World's
/// own Undo of the same act names its file (`Undo separate <file>`); this one is Library's.
pub const SEPARATE_ENTRY_UNDO: UndoStep = UndoStep::words("Undo separate entry");
/// `Choose all` on a package About page: its result's Undo.
pub const PACKAGE_CHOICE_UNDO: UndoStep = UndoStep::words("Undo package choice");
/// A link update after a rename: its result's Undo.
pub const LINK_UPDATE_UNDO: UndoStep = UndoStep::words("Undo link update");
/// Why an act waits for a campaign entry's draft, as a typed lock: `DraftBlocks`' own words.
pub static DRAFT_BLOCKS_IN_CAMPAIGN: Sentence =
    crate::sentence!("Save or discard to copy, add an entry, change packages or switch campaign.");
/// Why an act waits for a package entry's draft, as a typed lock: `DraftBlocks`' own words.
pub static DRAFT_BLOCKS_IN_PACKAGE: Sentence =
    crate::sentence!("Save or discard to copy, add an entry or change packages.");

/// Why the page's other actions wait while the open entry's file changed on disk.
pub static CHANGED_FILE_FIRST: design::outcome::Sentence =
    splat::sentence!("Choose what to do with the changed file first.");
/// Why a kept result's Undo waits while the open entry has a draft (ruling Q-u: a locked Undo
/// says why on its own line).
pub static UNDO_WAITS_FOR_DRAFT: design::outcome::Sentence =
    splat::sentence!("Save or discard this draft to undo.");

/// Where an entry can go: ONE policy for `Copy to…` (into a package or any campaign: the copy
/// dialog lists them all) and `New entry` (into a package or the open campaign). Each action's
/// availability and its words come from here, so the two cannot disagree about why they wait.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Places {
    pub packages: bool,
    pub campaigns: bool,
    open_campaign: bool,
}

impl Places {
    pub fn of(campaigns: &Campaigns) -> Self {
        Self {
            packages: !campaigns.modules().is_empty(),
            campaigns: !campaigns.all().is_empty(),
            open_campaign: campaigns.active_campaign().is_some(),
        }
    }

    /// Why `Copy to…` waits: no package and no campaign to copy into.
    pub fn copy(self) -> Option<design::outcome::Sentence> {
        (!self.packages && !self.campaigns)
            .then(|| splat::sentence!("Add a campaign or a package to copy entries into."))
    }

    /// Why `New entry` waits: no package and no open campaign to make it in.
    pub fn create(self) -> Option<design::outcome::Sentence> {
        if self.packages || self.open_campaign {
            None
        } else if self.campaigns {
            Some(splat::sentence!(
                "Open a campaign or add a package to create entries."
            ))
        } else {
            Some(splat::sentence!(
                "Add a campaign or a package to create entries."
            ))
        }
    }
}

/// The reader's kind line keeps its kind, class, level and source. A spell's level part
/// names its school too, as the books print it (`spell`: the list's part and the reader's).
pub fn reader_subtitle(
    owner: &Owner,
    listed: &str,
    class_level: Option<String>,
    spell: Option<(String, String)>,
) -> String {
    let listed = match spell {
        Some((list, reader)) => listed
            .split(" · ")
            .map(|part| if part == list { reader.as_str() } else { part })
            .collect::<Vec<_>>()
            .join(" · "),
        None => listed.to_owned(),
    };
    match class_level {
        Some(level) => format!("Class feature · {level} · {}", owner.source()),
        None if *owner == Owner::Builtin => format!("{listed} · {BUILTIN_NAME}"),
        None => listed,
    }
}

/// A spell's level part of the list line (`Level 1`, `Cantrip`), and the reader's form of it with
/// the spell's school (`Level 1 Evocation`, `Evocation Cantrip`), from its `splat_spell`
/// fields. `None` without a school.
pub fn spell_kind(spell: &serde_yaml_ng::Value) -> Option<(String, String)> {
    use serde_yaml_ng::Value;
    let school = spell.get("school")?.as_str()?.trim();
    let mut letters = school.chars();
    let school = format!("{}{}", letters.next()?.to_uppercase(), letters.as_str());
    Some(match spell.get("level").and_then(Value::as_u64)? {
        0 => ("Cantrip".to_owned(), format!("{school} Cantrip")),
        level => (format!("Level {level}"), format!("Level {level} {school}")),
    })
}

/// The reader's kind line from its list line. The owner line names the package, so the kind line
/// does not. Where two packages share that name, the list adds the folder, and only the folder
/// stays: `Creature · CR 1 · ashen-marches/`.
pub fn reader_kind_line(owner: &Owner, listed: &str) -> String {
    let Owner::Package(module) = owner else {
        return listed.to_owned();
    };
    let name = format!(" · {}", module.name);
    let folder = format!(" · {}/", owner.folder());
    match listed.strip_suffix(&format!("{name}{folder}")) {
        Some(head) => format!("{head}{folder}"),
        None => listed.strip_suffix(&name).unwrap_or(listed).to_owned(),
    }
}

/// The scope's name, as its menu says it.
pub fn scope_name(campaigns: &Campaigns, scope: Scope) -> String {
    match scope {
        Scope::Uses => "Can use".to_owned(),
        Scope::All => "Everything".to_owned(),
        Scope::Builtin => BUILTIN_NAME.to_owned(),
        Scope::Package(id) => campaigns
            .module_positions()
            .get(&id)
            .map(|&at| &campaigns.modules()[at])
            .map_or_else(|| "A package".to_owned(), |m| m.name.clone()),
        Scope::Campaign => "Campaign entries".to_owned(),
    }
}

/// The action after an empty search or filter that keeps the words in the widest scope.
pub const SEARCH_EVERYTHING: &str = "Search Everything";

/// What the catalogue shows. The default is what the active campaign can use.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
pub enum Scope {
    #[default]
    Uses,
    All,
    Builtin,
    Package(Uuid),
    Campaign,
}

/// A package's About line count, in Campaign's form: `All entries chosen`, `5 of 6 entries
/// chosen`, `No entries chosen`. Round 27 FA6: a ticked box counts as chosen, whoever ticked
/// it, so the entries the DM's choices need (ticked and locked in Campaign) count too. The
/// partial count is the one shared form (`design::outcome::chosen_count`, M0306).
pub fn chosen_count(chosen: usize, needed: usize, total: usize) -> String {
    match (chosen + needed).min(total) {
        0 => "No entries chosen".into(),
        ticked if ticked == total => "All entries chosen".into(),
        ticked => design::outcome::chosen_count(ticked, total, "entries"),
    }
}

/// How many entries leave Splat's lists when a package is unregistered. The manifest is not one.
pub fn unregister_count(notes: &[Note], package: Uuid) -> usize {
    notes
        .iter()
        .filter(|n| {
            n.owner.is_package(package)
                && !n.is_manifest()
                && n.entry.as_ref().is_none_or(|e| e.kind != "reference")
        })
        .count()
}

pub struct Note {
    pub path: String,
    /// The file's text, with its revision (layout PR 9b-C): set when the note loads, saves or
    /// follows the disk, so an open draft's compare with it is two revisions on a quiet frame.
    pub text: crate::design::memo::Revised<String>,
    pub entry: Option<Entry>,
    pub error: Option<String>,
    /// Why the file could not be read at all, from the system error's kind. Such a note has no
    /// text, so it offers no source editing or repair until a read succeeds.
    pub fault: Option<FileFault>,
    /// The package's manifest was deleted: this note stands in for it. Its entries still
    /// read, so it is a fault to fix (a warning), not a file that cannot be read.
    pub manifest_gone: bool,
    pub owner: Owner,
}
impl Note {
    pub fn is_manifest(&self) -> bool {
        // A readable note is a manifest only by its kind; the file name and the kind line are
        // guesses for a note that cannot be read.
        matches!(self.owner, Owner::Package(_))
            && match &self.entry {
                Some(entry) => entry.kind == "package",
                None => {
                    matches!(self.path.as_str(), "Package.md" | "Module.md")
                        || self.text.lines().any(|line| line == "splat_kind: package")
                }
            }
    }

    /// The About row's words. `flagged` is true when the header counts a problem in this file,
    /// such as a first heading that differs from the name. The mark has the fault's tone: a
    /// manifest that cannot be read is an error, and a deleted one or another problem is a warning.
    pub fn about_label(&self, flagged: bool) -> String {
        let name = match &self.owner {
            Owner::Package(module) => &module.name,
            _ => self.name(),
        };
        let deleted = self.error.as_deref() == Some(crate::content_workspace::DELETED_MANIFEST);
        let mark = if self.error.is_some() && !deleted {
            Some(icons::ERROR)
        } else if deleted || flagged {
            Some(icons::WARN)
        } else {
            None
        };
        format!(
            "{}About {name}",
            mark.map_or_else(String::new, |mark| format!("{mark} "))
        )
    }

    pub fn unreadable_kind(&self) -> Section {
        if self.is_manifest() {
            return Section::PackageDetails;
        }
        let declared = self
            .text
            .lines()
            .find_map(|line| line.strip_prefix("splat_kind: "))
            .map(|kind| Section::for_kind(kind.trim().trim_matches(['\'', '"'])))
            .unwrap_or(Section::Unreadable);
        if ![Section::Other, Section::Unreadable].contains(&declared) {
            return declared;
        }
        self.text
            .lines()
            .find_map(|line| {
                let kind = line.strip_prefix("splat_")?.split_once(':')?.0;
                let section = Section::for_kind(kind);
                (![Section::Other, Section::Unreadable].contains(&section)).then_some(section)
            })
            .unwrap_or(Section::Unreadable)
    }
    pub fn name(&self) -> &str {
        if self.error.as_deref() == Some(crate::content_workspace::DELETED_MANIFEST)
            && let Owner::Package(module) = &self.owner
        {
            return &module.name;
        }
        self.entry.as_ref().map_or(&self.path, |e| &e.name)
    }
    pub fn section(&self) -> Section {
        self.entry
            .as_ref()
            .map_or(Section::Unreadable, Section::for_entry)
    }
    /// How the row matches, as `matching::with_other` ranks it, or None if it does not. Plain
    /// words match the name (in any order, or with a typo), path or section, or a creature's
    /// facts. `habitat:forest` and the other fact terms match creatures only. A creature's name
    /// ranks in the one creature order (M3, `Typed::creature`), as Lookup and the encounter
    /// search rank it; every other kind keeps its word tiers.
    pub fn search_rank(
        &self,
        section: Section,
        query: &Query,
        typed: &Typed,
        facets: Option<&Facets>,
    ) -> Option<Rank> {
        let words = query.words.as_str();
        if words.is_empty() && !query.names_facts() {
            return (section == Section::All || self.section() == section)
                .then_some(Rank::typed(0));
        }
        if query.names_facts() && !facets.is_some_and(|facets| query.matches_facts(facets)) {
            return None;
        }
        if words.is_empty() {
            return Some(Rank::typed(0));
        }
        let other = self.path.to_lowercase().contains(words)
            || self.section().label().to_lowercase().contains(words)
            || facets.is_some_and(|facets| query.words_in(self.name(), facets));
        let name = self.name().to_lowercase();
        let name = if self.entry.as_ref().is_some_and(|e| e.kind == "creature") {
            typed.creature(&name, crate::matching::words(&name))
        } else {
            typed.name(&name)
        };
        with_other(name, other)
    }
}
#[derive(Default)]
pub struct DuplicateIndex {
    ids: std::collections::HashMap<Uuid, Vec<usize>>,
    pub keys: std::collections::HashMap<String, Vec<usize>>,
}

/// An entry as other entries name it: its id, and its key when it has one.
pub type EntryRef = (Uuid, Option<String>);

/// The entries that need one entry, by name, as `Loaded` keeps them: a lookup hands out the
/// kept names (an `Arc`), never a copy. Only `new` makes one, from names just found, so every
/// copy of names is counted in tests.
#[derive(Clone, Default)]
pub struct DependentNames(std::sync::Arc<[String]>);

impl DependentNames {
    pub fn new(names: Vec<String>) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        DEPENDENT_NAMES.with(|copied| copied.set(copied.get() + names.len()));
        Self(names.into())
    }
}

impl std::ops::Deref for DependentNames {
    type Target = [String];
    fn deref(&self) -> &[String] {
        &self.0
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times the notes were read to count a rename's links.
    pub static RENAME_LINK_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many times the notes were read to find the entries that need one entry.
    pub static DEPENDENT_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many dependent names were copied into a new `DependentNames`.
    pub static DEPENDENT_NAMES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The load of the link index a scan used (`ContentIndex::generation`, never its address),
/// and each listed entry's position and broken link count.
type BrokenLinks = (u64, Vec<(usize, usize)>);

/// A problem the Library header counts, and the file it belongs to.
pub struct Diagnostic {
    pub file: FileId,
    /// The menu line: the file (with its owner when another owner has the same path) and the problem.
    pub text: String,
}

/// Marks a checked entry whose path another owner shares, so its problems go back to its own note.
const SHARED_PATH_TAG: &str = "\u{0}note:";

#[derive(Default)]
pub struct Loaded {
    pub notes: Vec<Note>,
    pub owners: Vec<Owner>,
    /// Problems that kept content from loading. The notes cannot show these again.
    /// Each keyed by its package or campaign id, so two packages with one name count twice.
    load_errors: Vec<(FileId, String)>,
    pub diagnostics: Vec<Diagnostic>,
    /// Files the header counts that have no message of their own: each file of a duplicate
    /// identity or key, whose row says `Same entry in two files`. The check names one side only.
    duplicate_files: Vec<FileId>,
    /// Files whose every counted problem is a link to an entry that cannot be found. Their row
    /// says `N broken links` once, with no second `Needs fixing` mark.
    pub link_faults_only: std::collections::HashSet<FileId>,
    /// A counted file cannot be read, so the header count has the error tone.
    pub unreadable_counted: bool,
    /// Paths that more than one note has, such as `Rules/Watch.md` in two packages, with the
    /// owner name of each note there. Rows that name one of these files name its owner too.
    /// `invalidate` clears it.
    shared_paths: std::sync::OnceLock<std::collections::HashMap<String, Vec<String>>>,
    /// What the active campaign can use: its selections and what they require.
    pub uses: Option<std::collections::HashSet<Uuid>>,
    /// The entries the DM chose for the active campaign, without the entries those choices
    /// need. Counts that say `chosen` count only these (round 26 FC2, FD4).
    pub chosen: Option<std::collections::HashSet<Uuid>>,
    pub subtitles: std::sync::OnceLock<Vec<String>>,
    pub counts: std::sync::OnceLock<Vec<(Section, usize)>>,
    pub duplicate_ids: std::sync::OnceLock<std::collections::HashSet<Uuid>>,
    pub duplicates: std::sync::OnceLock<DuplicateIndex>,
    /// Each note's creature facts, for search and the creature filter.
    pub facets: std::sync::OnceLock<Vec<Option<Facets>>>,
    /// Listed entries with a broken link, found against this link index. A new index, an edited
    /// note (`invalidate`) or a reload (a new `Loaded`) finds them again.
    broken_links: std::sync::Mutex<Option<BrokenLinks>>,
    /// How many links a rename's offer would change, for its old and new names. A reload (a new
    /// `Loaded`) or an edited note (`invalidate`) counts them again.
    rename_links: std::sync::Mutex<Option<((String, String), usize)>>,
    /// The entries that need an entry (by its id and key), by name: Remove's result and a
    /// deleted draft's warning show them in every frame. A reload (a new `Loaded`) or an edited
    /// note (`invalidate`) finds them again.
    pub dependents: std::sync::Mutex<std::collections::HashMap<EntryRef, DependentNames>>,
    /// Creatures' Spellcasting resolved through these notes, cleared with the other caches.
    pub spells: spells::SpellCache,
}
impl Loaded {
    #[cfg(any(test, feature = "test-support"))]
    pub fn fixture(notes: Vec<Note>) -> Self {
        Self {
            notes,
            ..Self::default()
        }
    }

    pub fn facets(&self) -> &[Option<Facets>] {
        self.facets.get_or_init(|| {
            self.notes
                .iter()
                .map(|note| note.entry.as_ref().and_then(Facets::of))
                .collect()
        })
    }
    /// The listed entries whose rows say `broken link`, as (note position, broken link count):
    /// campaign and package entries, not manifests. Scanning every body is the work the cache
    /// keeps out of each frame, the header menu included.
    pub fn broken_link_notes(
        &self,
        index: &std::sync::Arc<crate::lookup::ContentIndex>,
    ) -> Vec<(usize, usize)> {
        let mut cache = self
            .broken_links
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some((known, notes)) = cache.as_ref()
            && *known == index.generation()
        {
            return notes.clone();
        }
        let notes: Vec<(usize, usize)> = self
            .notes
            .iter()
            .enumerate()
            .filter(|(_, note)| note.owner != Owner::Builtin && !note.is_manifest())
            .filter_map(|(at, note)| {
                let entry = note.entry.as_ref()?;
                let count = index.broken_link_count(&entry.body, entry.id);
                (count > 0).then_some((at, count))
            })
            .collect();
        *cache = Some((index.generation(), notes.clone()));
        notes
    }

    /// The offer checked against the disk just before its write: the entry's own file still
    /// holds it with the new name (an outside rename since the last load ends the offer).
    /// The linking files are checked as they are written: each must still hold what was read.
    pub fn offer_holds_on_disk(&self, offer: &LinkOffer) -> bool {
        let source = &offer.source;
        let Ok(path) = note_path(&source.root, &source.path) else {
            return false;
        };
        // The exact bytes the rename saved: an outside edit since then, even to the body, ends
        // the offer.
        fs::read_to_string(path).is_ok_and(|text| text == source.text)
            && Entry::parse(&source.path, &source.text)
                .is_ok_and(|entry| entry.id == offer.owner && entry.name == offer.new)
            && !self.name_taken_on_disk(offer)
    }

    /// Whether another file on disk, in any editable root of the notes, has the offer's identity,
    /// or its old or new name or file stem. Only the offer's own source file (its root and path)
    /// is left out: a new file with the same identity is a conflict too. The loaded notes may be
    /// older than the disk, so the write reads the folders again. Built-in entries do not
    /// change, so the loaded check covers them.
    fn name_taken_on_disk(&self, offer: &LinkOffer) -> bool {
        let mut roots: Vec<&Path> = self.notes.iter().filter_map(|n| n.owner.root()).collect();
        roots.sort_unstable();
        roots.dedup();
        let taken = |name: &str| {
            [offer.old.as_str(), offer.new.as_str()]
                .iter()
                .any(|wanted| name.eq_ignore_ascii_case(wanted))
        };
        roots.into_iter().any(|root| {
            let Ok((entries, _)) = content::read_directory(root) else {
                // A folder that cannot be read cannot be checked: the write waits.
                return true;
            };
            let source = root == offer.source.root.as_path();
            entries.iter().any(|entry| {
                !(source && entry.path == offer.source.path)
                    && (entry.id == offer.owner
                        || taken(&entry.name)
                        || Path::new(&entry.path)
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .is_some_and(taken))
            })
        })
    }

    /// True while a rename's offer still holds: its entry still has the new name, and the old and
    /// new names name no other entry or file.
    pub fn offer_holds(&self, offer: &LinkOffer) -> bool {
        self.notes
            .iter()
            .position(|note| {
                note.entry.as_ref().is_some_and(|e| e.id == offer.owner)
                    && note.owner.root() == Some(offer.source.root.as_path())
            })
            .is_some_and(|at| {
                let note = &self.notes[at];
                note.path == offer.source.path
                    && *note.text == offer.source.text
                    && note
                        .entry
                        .as_ref()
                        .is_some_and(|entry| entry.name == offer.new)
                    && self.can_rename_links(at, &offer.old, &offer.new)
            })
    }

    /// How many links in the notes name `old`, for the offer to change them to `new`. Counted
    /// once per pair of names until `invalidate` or a reload.
    pub fn rename_link_count(&self, old: &str, new: &str) -> usize {
        let mut cache = self
            .rename_links
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(((known_old, known_new), count)) = cache.as_ref()
            && known_old == old
            && known_new == new
        {
            return *count;
        }
        #[cfg(any(test, feature = "test-support"))]
        RENAME_LINK_SCANS.with(|scans| scans.set(scans.get() + 1));
        let count = self
            .notes
            .iter()
            .filter(|note| note.owner.root().is_some())
            .map(|note| crate::world::rename_note_links(&note.text, old, new).1)
            .sum();
        *cache = Some(((old.to_owned(), new.to_owned()), count));
        count
    }

    pub fn can_rename_links(&self, selected: usize, old: &str, new: &str) -> bool {
        !self.notes.iter().enumerate().any(|(at, note)| {
            at != selected
                && note.entry.is_some()
                && [old, new].iter().any(|name| {
                    note.name().eq_ignore_ascii_case(name)
                        || Path::new(&note.path)
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .is_some_and(|stem| stem.eq_ignore_ascii_case(name))
                })
        })
    }
    pub fn duplicate_folder_paths(&self, owner: &Owner) -> Option<Vec<String>> {
        let root = owner.root()?;
        let mut other_root = None;
        let mut paths = Vec::new();
        for (at, note) in self.notes.iter().enumerate() {
            if note
                .entry
                .as_ref()
                .is_none_or(|entry| entry.kind == "package")
            {
                continue;
            }
            let duplicates = self.duplicates(at);
            if duplicates.is_empty() {
                continue;
            }
            if duplicates.len() != 1 {
                return None;
            }
            let pair = &self.notes[duplicates[0]];
            let (here, there) = (note.owner.root()?, pair.owner.root()?);
            let counterpart = if here == root {
                there
            } else if there == root {
                here
            } else {
                return None;
            };
            if counterpart == root || other_root.is_some_and(|known| known != counterpart) {
                return None;
            }
            other_root = Some(counterpart);
            if here == root {
                paths.push(note.path.clone());
            }
        }
        (paths.len() > 1).then_some(paths)
    }
    pub fn duplicate_ids(&self) -> &std::collections::HashSet<Uuid> {
        self.duplicate_ids.get_or_init(|| {
            self.notes
                .iter()
                .enumerate()
                .filter(|(at, _)| !self.duplicates(*at).is_empty())
                .filter_map(|(_, note)| note.entry.as_ref().map(|entry| entry.id))
                .collect()
        })
    }
    pub fn dependents(&self, note: &Note) -> DependentNames {
        let Some(id) = crate::content_workspace::note_id(&note.text) else {
            return DependentNames::default();
        };
        let known = (id, note.entry.as_ref().and_then(|entry| entry.key.clone()));
        let mut cache = self
            .dependents
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(names) = cache.get(&known) {
            return names.clone();
        }
        #[cfg(any(test, feature = "test-support"))]
        DEPENDENT_SCANS.with(|scans| scans.set(scans.get() + 1));
        let names = DependentNames::new(
            self.notes
                .iter()
                .filter_map(|other| {
                    let entry = other.entry.as_ref()?;
                    (entry.id != id
                        && content::required_references(entry).is_ok_and(|refs| {
                            refs.contains(&content::Reference::Id(id))
                                || note
                                    .entry
                                    .as_ref()
                                    .and_then(|entry| entry.key.as_ref())
                                    .is_some_and(|key| {
                                        refs.contains(&content::Reference::Key(key.clone()))
                                    })
                        }))
                    .then(|| entry.name.clone())
                })
                .collect(),
        );
        cache.insert(known, names.clone());
        names
    }
    fn duplicate_index(&self) -> &DuplicateIndex {
        self.duplicates.get_or_init(|| {
            let mut index = DuplicateIndex::default();
            for (at, note) in self.notes.iter().enumerate() {
                if let Some(entry) = &note.entry {
                    index.ids.entry(entry.id).or_default().push(at);
                    if let Some(key) = &entry.key {
                        index.keys.entry(key.clone()).or_default().push(at);
                    }
                }
            }
            index
        })
    }
    /// Why Save would refuse this parse of note `index`'s draft, without writing anything. Save
    /// asks this before it writes, and a repair draft shows the same reason as its notice. It
    /// reads the identity index, which `invalidate` clears.
    pub fn draft_refusal(&self, index: usize, draft: &Result<Entry, String>) -> Option<String> {
        let entry = match draft {
            Ok(entry) => entry,
            Err(error) => return Some(error.clone()),
        };
        let note = &self.notes[index];
        if let Some(old) = &note.entry {
            if entry.id != old.id || entry.kind != old.kind {
                return Some("Keep this entry’s UUID and kind unchanged.".into());
            }
            if matches!(note.owner, Owner::Campaign(_)) && entry.key != old.key {
                return Some("Keep the entry’s UUID, kind and key unchanged.".into());
            }
            if old.kind == "package"
                && entry.fields.get("splat_namespace") != old.fields.get("splat_namespace")
            {
                return Some("Keep the package namespace unchanged.".into());
            }
        }
        let lookup = self.duplicate_index();
        let other = |at: &Vec<usize>| at.iter().any(|&at| at != index);
        let id_taken = note.entry.as_ref().is_none_or(|old| old.id != entry.id)
            && lookup.ids.get(&entry.id).is_some_and(other);
        let key_taken = entry.key.as_ref().is_some_and(|key| {
            note.entry
                .as_ref()
                .is_none_or(|old| old.key.as_ref() != Some(key))
                && lookup.keys.get(key).is_some_and(other)
        });
        (id_taken || key_taken).then(|| "Another entry already uses this UUID or key.".into())
    }
    pub fn duplicates(&self, index: usize) -> Vec<usize> {
        let Some(target) = self.notes[index].entry.as_ref() else {
            return Vec::new();
        };
        let lookup = self.duplicate_index();
        let mut matches: Vec<_> = lookup
            .ids
            .get(&target.id)
            .into_iter()
            .flatten()
            .chain(
                target
                    .key
                    .as_ref()
                    .and_then(|key| lookup.keys.get(key))
                    .into_iter()
                    .flatten(),
            )
            .copied()
            .filter(|&at| at != index)
            .collect();
        // The original scan returns each matching note once, in Library order.
        matches.sort_unstable();
        matches.dedup();
        matches
    }
    /// Checks all the loaded content as one, so that a reference can resolve to built-in
    /// content or to another package. The check of one package alone cannot see those.
    pub fn refresh_diagnostics(&mut self) {
        let shared = self.shared_paths();
        // The check names a problem by entry path only. An entry whose path another owner shares
        // is checked under a tag that names its note, so its problems stay with its own file.
        let tagged: std::collections::HashMap<usize, Entry> = self
            .notes
            .iter()
            .enumerate()
            .filter(|(_, note)| shared.contains_key(&note.path))
            .filter_map(|(at, note)| {
                let mut entry = note.entry.clone()?;
                entry.path = format!("{SHARED_PATH_TAG}{at}");
                Some((at, entry))
            })
            .collect();
        let entries: Vec<_> = self
            .notes
            .iter()
            .enumerate()
            .filter_map(|(at, n)| tagged.get(&at).or(n.entry.as_ref()))
            // The SRD pages have no notes in the Library, but entries can refer to them.
            .chain(content::builtin::entries())
            .collect();
        let report = content::validate_all(&entries);
        let checks = report.errors.into_iter().chain(report.warnings);
        let mut diagnostics: Vec<Diagnostic> = self
            .load_errors
            .iter()
            .map(|(file, text)| Diagnostic {
                file: file.clone(),
                text: text.clone(),
            })
            .collect();
        for check in checks {
            let note = match check.path.strip_prefix(SHARED_PATH_TAG) {
                Some(at) => at.parse::<usize>().ok().and_then(|at| self.notes.get(at)),
                None => self.notes.iter().find(|note| note.path == check.path),
            };
            diagnostics.push(match note {
                Some(note) => self.note_problem(note, &check.message),
                None => Diagnostic {
                    file: (OwnerKey::Builtin, check.path.clone()),
                    text: format!("{}: {}", check.path, check.message),
                },
            });
        }
        for note in &self.notes {
            if let Some(error) = &note.error {
                diagnostics.push(self.note_problem(note, error));
            }
        }
        for note in self
            .notes
            .iter()
            .filter(|note| note.owner != Owner::Builtin)
        {
            if let Some(entry) = &note.entry {
                for flag in &entry.flags {
                    diagnostics.push(self.note_problem(note, &flag.message));
                }
                let faults = crate::roll_table::fault_count(&entry.body);
                if faults > 0 {
                    diagnostics.push(
                        self.note_problem(
                            note,
                            &design::count(faults, "table fault", "table faults"),
                        ),
                    );
                }
            }
            if let Some(entry) = &note.entry
                && let Some((old, _)) = presentation::heading_fix(&note.text, entry)
            {
                diagnostics.push(self.note_problem(
                    note,
                    &format!(
                        "The first heading says {old}, but the name is {}.",
                        entry.name
                    ),
                ));
            }
        }
        // Every file of a duplicate identity or key counts, as both rows are marked.
        self.duplicate_files = (0..self.notes.len())
            .filter(|&at| !self.duplicates(at).is_empty())
            .map(|at| Self::file_key(&self.notes[at]))
            .collect();
        // A file whose problems are all links to missing entries: each problem is a reference
        // that resolves to nothing, the way the check resolves them, and each such reference is
        // a link in the file's own text, which its row counts as a broken link.
        let ids: std::collections::HashSet<Uuid> = entries.iter().map(|entry| entry.id).collect();
        let keys: std::collections::HashSet<&str> = entries
            .iter()
            .filter_map(|entry| entry.key.as_deref())
            .collect();
        self.link_faults_only = self
            .notes
            .iter()
            .filter(|note| note.owner != Owner::Builtin)
            .filter_map(|note| {
                let entry = note.entry.as_ref()?;
                let unresolved: Vec<String> = content::required_references(entry)
                    .ok()?
                    .into_iter()
                    .filter_map(|reference| match reference {
                        content::Reference::Id(id) => (!ids.contains(&id)).then(|| id.to_string()),
                        content::Reference::Key(key) => {
                            (!keys.contains(key.as_str())).then(|| key.to_lowercase())
                        }
                    })
                    .collect();
                if unresolved.is_empty() {
                    return None;
                }
                let links = crate::world::links_in(&entry.body);
                let file = Self::file_key(note);
                let problems = diagnostics.iter().filter(|d| d.file == file).count();
                (problems == unresolved.len()
                    && unresolved.iter().all(|target| {
                        links
                            .iter()
                            .any(|link| link.split('|').next().unwrap_or("").trim() == target)
                    }))
                .then_some(file)
            })
            .collect();
        self.unreadable_counted = self.notes.iter().any(|note| {
            note.entry.is_none()
                && note.error.is_some()
                && note.error.as_deref() != Some(crate::content_workspace::DELETED_MANIFEST)
        });
        self.diagnostics = diagnostics;
        // A row marks each file the header counts, so the rows follow the new problems.
        self.subtitles.take();
    }

    /// Paths that more than one note has, each with the owner names there. Built once per change
    /// of the notes.
    fn shared_paths(&self) -> &std::collections::HashMap<String, Vec<String>> {
        self.shared_paths.get_or_init(|| {
            let mut owners: std::collections::HashMap<String, Vec<String>> =
                std::collections::HashMap::new();
            for note in &self.notes {
                owners
                    .entry(note.path.clone())
                    .or_default()
                    .push(note.owner.source().to_owned());
            }
            owners.retain(|_, names| names.len() > 1);
            owners
        })
    }

    /// The note's file as a menu row names it: its path, after its owner when another owner has
    /// the same path (`Ashen Marches · Rules/Watch.md`). When that other owner has the same name
    /// too, the row names the folder (`Ashen Marches · ashen-copy/Rules/Watch.md`).
    pub fn file_label(&self, note: &Note) -> String {
        let source = note.owner.source();
        match self.shared_paths().get(&note.path) {
            None => note.path.clone(),
            Some(names) if names.iter().filter(|name| *name == source).count() > 1 => {
                format!("{source} · {}/{}", note.owner.folder(), note.path)
            }
            Some(_) => format!("{source} · {}", note.path),
        }
    }

    pub fn file_key(note: &Note) -> FileId {
        (note.owner.key(), note.path.clone())
    }

    fn note_problem(&self, note: &Note, message: &str) -> Diagnostic {
        Diagnostic {
            file: Self::file_key(note),
            text: format!("{}: {message}", self.file_label(note)),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn diagnostic_texts(&self) -> Vec<&str> {
        self.diagnostics.iter().map(|d| d.text.as_str()).collect()
    }

    /// True when the header counts a problem in this note's file.
    pub fn has_problem(&self, note: &Note) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.file.0 == note.owner.key() && d.file.1 == note.path)
    }

    /// How many files the header counts: each file once, by owner and path, for a problem, a
    /// broken link, a duplicate identity or more than one.
    /// `extra` is a file the loaded notes do not know is faulty yet: an open draft's file that
    /// stopped reading.
    pub fn files_to_fix(&self, broken: &[(usize, usize)], extra: Option<&FileId>) -> usize {
        self.diagnostics
            .iter()
            .map(|d| (d.file.0, d.file.1.as_str()))
            .chain(broken.iter().map(|&(at, _)| {
                let note = &self.notes[at];
                (note.owner.key(), note.path.as_str())
            }))
            .chain(
                self.duplicate_files
                    .iter()
                    .map(|(owner, path)| (*owner, path.as_str())),
            )
            .chain(extra.map(|(owner, path)| (*owner, path.as_str())))
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    /// Whether the DM chose this entry for the active campaign, not only needed it. With no
    /// campaign open, nothing is chosen.
    pub fn chose(&self, note: &Note) -> bool {
        self.chosen
            .as_ref()
            .zip(note.entry.as_ref())
            .is_some_and(|(chosen, entry)| chosen.contains(&entry.id))
    }

    pub fn in_scope(&self, note: &Note, scope: Scope) -> bool {
        if note.is_manifest() {
            return false;
        }
        match scope {
            Scope::All => true,
            Scope::Uses => match &self.uses {
                None => true,
                Some(uses) => {
                    matches!(note.owner, Owner::Campaign(_))
                        || note
                            .entry
                            .as_ref()
                            .is_some_and(|entry| self.duplicate_ids().contains(&entry.id))
                        || note.entry.as_ref().is_some_and(|e| uses.contains(&e.id))
                }
            },
            Scope::Builtin => note.owner == Owner::Builtin,
            Scope::Package(id) => note.owner.is_package(id),
            Scope::Campaign => matches!(note.owner, Owner::Campaign(_)),
        }
    }

    fn resolve_reference(&self, reference: &content::Reference) -> Option<usize> {
        let mut matches = self.notes.iter().enumerate().filter_map(|(i, note)| {
            let entry = note.entry.as_ref()?;
            let matches = match reference {
                content::Reference::Id(id) => entry.id == *id,
                content::Reference::Key(key) => entry.key.as_ref() == Some(key),
            };
            matches.then_some(i)
        });
        let first = matches.next()?;
        matches.next().is_none().then_some(first)
    }

    pub fn resolve_spell(&self, reference: &content::Reference) -> Option<usize> {
        self.resolve_reference(reference).filter(|&index| {
            self.notes[index]
                .entry
                .as_ref()
                .is_some_and(|entry| entry.kind == "spell")
        })
    }

    pub fn subtitles(&self) -> &Vec<String> {
        self.subtitles.get_or_init(|| self.build_subtitles())
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn count(&self, section: Section) -> usize {
        self.counts
            .get_or_init(|| {
                Section::GROUPS
                    .iter()
                    .flat_map(|(_, sections)| sections.iter())
                    .map(|&section| {
                        (
                            section,
                            self.notes
                                .iter()
                                .filter(|n| section == Section::All || n.section() == section)
                                .count(),
                        )
                    })
                    .collect()
            })
            .iter()
            .find(|(s, _)| *s == section)
            .map_or(0, |(_, n)| *n)
    }
    pub fn invalidate(&mut self) {
        *self
            .broken_links
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        *self
            .rename_links
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
        self.dependents
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
        self.spells.clear();
        self.subtitles.take();
        self.counts.take();
        self.duplicate_ids.take();
        self.duplicates.take();
        self.facets.take();
        self.shared_paths.take();
    }
    pub fn build_subtitles(&self) -> Vec<String> {
        let mut subtitles: Vec<_> = self
            .notes
            .iter()
            .map(|note| {
                let source = match &note.owner {
                    Owner::Builtin => None,
                    Owner::Package(module) => Some(module.name.as_str()),
                    Owner::Campaign(_) => Some("Campaign"),
                };
                let line = note.entry.as_ref().map_or_else(
                    || {
                        // A file the system will not read names its short cause, as World's
                        // row does. Its owner line in the reader keeps `Cannot be read`. A file
                        // that cannot be read is an error, so its mark has the error tone, as in
                        // the reader and the header count.
                        if let Some(fault) = &note.fault {
                            return format!("{} {}", icons::ERROR, fault.cause());
                        }
                        let (_, line) =
                            crate::content_workspace::repair_problem(&note.path, &note.text);
                        format!(
                            "{} {}",
                            icons::ERROR,
                            line.map_or_else(
                                || "Cannot be read".to_owned(),
                                |line| format!("Line {line}"),
                            )
                        )
                    },
                    presentation::subtitle,
                );
                // Round 25 AC2: each known repair cause is named on its own, after one mark,
                // so a duplicate that also has a heading fault says both.
                let mut causes = Vec::new();
                if note
                    .entry
                    .as_ref()
                    .is_some_and(|entry| self.duplicate_ids().contains(&entry.id))
                {
                    causes.push(SAME_ENTRY);
                }
                if note
                    .entry
                    .as_ref()
                    .is_some_and(|entry| presentation::heading_fix(&note.text, entry).is_some())
                {
                    causes.push(HEADING_DIFFERS);
                }
                let line = if !causes.is_empty() {
                    format!("{} {} · {line}", icons::WARN, causes.join(" · "))
                } else if note.entry.as_ref().is_some_and(|entry| {
                    // A table fault marks its row where the row is drawn.
                    crate::roll_table::fault_count(&entry.body) == 0
                }) && !note.is_manifest()
                    && !line.contains(icons::WARN)
                    && self.has_problem(note)
                {
                    // Every other fault the header counts in this file marks its row too.
                    format!("{} {NEEDS_FIXING} · {line}", icons::WARN)
                } else {
                    line
                };
                let mut subtitle = presentation::list_subtitle(&line, source);
                if let Owner::Package(module) = &note.owner
                    && self.owners.iter().any(|owner| matches!(owner, Owner::Package(other) if other.id != module.id && other.name == module.name))
                { subtitle.push_str(&format!(" · {}/", note.owner.folder())); }
                subtitle
            })
            .collect();
        // Imported sections share a broad parent. Use the closest named subsection
        // before a repeated heading, from the source spans, rather than a page number.
        let parent_of = |note: &Note| {
            note.entry
                .as_ref()
                .and_then(|entry| entry.fields.get("splat_conversion"))
                .and_then(|meta| meta.get("parent_name"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned()
        };
        let mut name_counts = std::collections::HashMap::new();
        for note in &self.notes {
            *name_counts
                .entry((parent_of(note), note.name()))
                .or_insert(0_usize) += 1;
        }
        for (i, note) in self.notes.iter().enumerate() {
            let duplicates = name_counts[&(parent_of(note), note.name())];
            if duplicates < 2 {
                continue;
            }
            let Some(entry) = &note.entry else {
                continue;
            };
            let Some(meta) = entry.fields.get("splat_conversion") else {
                continue;
            };
            let parent = meta.get("parent_name").and_then(|v| v.as_str());
            let first = meta
                .get("line_span")
                .and_then(|span| span.get("first"))
                .and_then(|v| v.as_u64());
            let Some(first) = first else {
                continue;
            };
            let context = self
                .notes
                .iter()
                .filter(|other| other.owner == note.owner)
                .filter_map(|other| {
                    let meta = other.entry.as_ref()?.fields.get("splat_conversion")?;
                    if meta.get("parent_name").and_then(|v| v.as_str()) != parent
                        || name_counts[&(parent_of(other), other.name())] > 1
                    {
                        return None;
                    }
                    let start = meta.get("line_span")?.get("first")?.as_u64()?;
                    (if parent.is_some_and(|parent| parent.eq_ignore_ascii_case(note.name())) {
                        start > first
                    } else {
                        start < first
                    })
                    .then_some((start, other.name()))
                })
                .min_by_key(|(start, _)| {
                    if parent.is_some_and(|parent| parent.eq_ignore_ascii_case(note.name())) {
                        *start
                    } else {
                        u64::MAX - *start
                    }
                });
            if let Some((_, context)) = context
                && let Some(parent) = parent
            {
                if parent.eq_ignore_ascii_case(note.name()) {
                    subtitles[i].push_str(&format!(" · {context}"));
                } else {
                    subtitles[i] = subtitles[i]
                        .replace(&format!(" · {parent}"), &format!(" · {parent} › {context}"));
                }
            }
        }
        subtitles
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Each spell the Library's notes look up for a stat block.
    pub static SPELL_LOOKUPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl view::Links for Loaded {
    fn find(&self, reference: &content::Reference) -> Option<(usize, String)> {
        let index = self.resolve_reference(reference)?;
        Some((index, self.notes[index].name().to_owned()))
    }
    fn find_spell(&self, reference: &content::Reference) -> Option<(usize, String)> {
        #[cfg(any(test, feature = "test-support"))]
        SPELL_LOOKUPS.with(|count| count.set(count.get() + 1));
        let index = self.resolve_spell(reference)?;
        Some((index, self.notes[index].name().to_owned()))
    }
    fn spell_cache(&self) -> Option<&spells::SpellCache> {
        Some(&self.spells)
    }
    fn spells_named_in(&self, text: &str) -> Vec<(usize, String)> {
        #[cfg(any(test, feature = "test-support"))]
        SPELL_LOOKUPS.with(|count| count.set(count.get() + 1));
        self.notes
            .iter()
            .enumerate()
            .filter(|(_, note)| {
                note.entry.as_ref().is_some_and(|e| e.kind == "spell")
                    && crate::lookup::names_word(text, note.name())
            })
            .map(|(index, note)| (index, note.name().to_owned()))
            .collect()
    }
    fn spell_named(&self, name: &str) -> Option<(usize, String)> {
        #[cfg(any(test, feature = "test-support"))]
        SPELL_LOOKUPS.with(|count| count.set(count.get() + 1));
        self.notes
            .iter()
            .position(|note| {
                note.entry.as_ref().is_some_and(|e| e.kind == "spell")
                    && note.name().eq_ignore_ascii_case(name)
            })
            .map(|index| (index, self.notes[index].name().to_owned()))
    }
    fn entry(&self, index: usize) -> Option<&Entry> {
        self.notes.get(index)?.entry.as_ref()
    }
}

/// Whose a copied package's registration result is: the copy (its package id) in the campaign
/// it was registered in. The store keeps one result per owner, shows it only in its campaign,
/// and keys the copy's `Use the copy in encounters` result by the same owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegistrationOwner {
    pub package: Uuid,
    pub campaign: Option<Uuid>,
}

impl design::results::Owner for RegistrationOwner {
    type Scope = Option<Uuid>;
    fn scope(&self) -> &Option<Uuid> {
        &self.campaign
    }
}

/// A copied package's registration and its kept result: everything its words say and its
/// Undo needs, in its store entry. Undo takes the entry whole, so the words go with the data.
pub struct Registration {
    /// The copy the registration wrote and registered.
    pub copy: crate::content_workspace::PackageCopy,
    /// The campaign open at the registration: the result's scope.
    pub campaign: Option<Campaign>,
    /// The campaign file the registration wrote, when it chose the copy: its path, its words
    /// before and after.
    pub written: Option<(String, String, String)>,
    /// The copy's choices in that campaign as the registration left them (none when it did not
    /// choose it). A later use change differs from them.
    pub choices: Vec<crate::content_workspace::PackageSelection>,
    /// R11: the clause (`, with no entries chosen for this campaign`) says what the
    /// registration chose. It holds until the copy's use changes after it; it never comes back.
    pub clause: bool,
    /// A copied file or the source change's files changed outside Splat: Undo waits.
    pub blocked: bool,
    /// Another campaign chose the copy: its Undo would leave that campaign's choices naming
    /// identities it removes, so it waits, and names that campaign.
    pub used_by: Option<String>,
}

impl design::results::Kept for Registration {
    type Owner = RegistrationOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // A second registration of the same folder ends it (`register_copy`).
        owner_changed: design::results::OwnerChanged::Ends,
        // A reload ends it once its copy is no longer registered (`reconcile_registrations`).
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> RegistrationOwner {
        RegistrationOwner {
            package: self.copy.module.id,
            campaign: self.campaign.as_ref().map(|campaign| campaign.id),
        }
    }
}

/// What a registration's check reads besides itself, made at the event that runs it.
pub struct RegistrationCheck {
    /// Each source change's owner, and whether its Undo waits.
    pub sources: Vec<(RegistrationOwner, bool)>,
    /// Every registered package's manifest, to read a choice that names a package by its key
    /// or by a link.
    pub manifests: Vec<Entry>,
    /// Every campaign the DM has.
    pub campaigns: Vec<Campaign>,
}

impl design::results::Reconcile for Registration {
    type With = RegistrationCheck;
    /// R11's one end event, at each write or refresh event and each reload (also after an
    /// outside edit). The clause ends when the copy's choices in the campaign differ from what
    /// the registration left, or encounters use the copy. Undo waits while a copied file or a
    /// source-change file changed, or while another campaign chose the copy.
    fn reconcile(&mut self, check: &RegistrationCheck) {
        use design::results::Kept;
        let package = self.copy.module.id;
        let owner = self.owner();
        let source = check
            .sources
            .iter()
            .find(|(source, _)| *source == owner)
            .map(|(_, blocked)| *blocked);
        self.blocked = source == Some(true) || !self.copy.on_disk();
        let own = self.campaign.as_ref().map(|campaign| campaign.id);
        self.used_by = used_elsewhere(package, own, &check.campaigns, &check.manifests);
        if self.clause {
            let choices_changed = self.campaign.as_ref().is_some_and(|campaign| {
                CampaignDocument::load(campaign).is_ok_and(|document| {
                    copy_choices(&document, package, &check.manifests) != self.choices
                })
            });
            if choices_changed || source.is_some() {
                self.clause = false;
            }
        }
    }
}

impl Registration {
    /// Why Undo waits, if it does.
    pub fn lock(&self) -> Option<design::outcome::Sentence> {
        if self.blocked {
            Some(crate::content_workspace::UNDO_ENDED_REASON)
        } else {
            self.used_by.as_deref().map(used_reason)
        }
    }
}

/// A campaign file's words without the package list's tasks that name `package`: each one's
/// lines as the reader gives them (`list_formats::package_tasks`: its task line and its `Only:`
/// lines), and nothing else, so Undo keeps every other choice and every other word.
fn without_package_lines(text: &str, package: Uuid, links: &[&str]) -> String {
    let mut drop: Vec<std::ops::Range<usize>> = content::list_formats::package_tasks(text)
        .into_iter()
        .filter(|task| task_names_package(task, package, links))
        .flat_map(|task| task.lines)
        .collect();
    drop.sort_by_key(|range| range.start);
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    for range in drop {
        out.push_str(&text[at..range.start]);
        at = range.end;
    }
    out.push_str(&text[at..]);
    out
}

/// True when a task of the package list names `package`: by its id after ` ^` (spaces around
/// the id do not hide it) or, with no id, by one of `links`, the names that named it.
fn task_names_package(
    task: &content::list_formats::PackageTask,
    package: Uuid,
    links: &[&str],
) -> bool {
    if task.id.is_empty() {
        links.contains(&task.name.as_str())
    } else {
        Uuid::parse_str(task.id.trim()).is_ok_and(|id| id == package)
    }
}

/// Why a registration's Undo is refused when taking the copy out of the campaign's package
/// list would change another package's choice.
pub const PACKAGE_LIST_UNSAFE: &str = "The campaign’s package list has a form Splat cannot change safely, so Undo changed nothing. Remove this package from the list by hand, then undo.";

/// Why a registration's Undo waits while another campaign chose the copy.
pub fn used_reason(campaign: &str) -> design::outcome::Sentence {
    splat::sentence!(
        "{} uses this package. Remove it from {} first, then undo.",
        campaign,
        campaign
    )
}

/// The package a choice in a campaign file names, in each form the file may give: its id, its
/// key, or a link to its manifest. The one reading of a choice for the clause's check and for
/// Undo's removal.
pub fn chosen_package(
    reference: &crate::content_workspace::EntryRef,
    manifests: &[Entry],
) -> Option<Uuid> {
    match (reference.id, &reference.key, &reference.link) {
        (Some(id), _, _) => Some(id),
        (None, Some(key), _) => manifests
            .iter()
            .find(|manifest| manifest.key.as_deref() == Some(key.as_str()))
            .map(|manifest| manifest.id),
        (None, None, Some(link)) => content::formats::resolve_readable(link, manifests).ok(),
        (None, None, None) => None,
    }
}

/// Every registered package's manifest.
pub fn package_manifests(campaigns: &Campaigns) -> Vec<Entry> {
    campaigns
        .modules()
        .iter()
        .filter_map(|module| {
            content::read_folder(&module.path)
                .ok()?
                .entries
                .iter()
                .find(|entry| entry.kind == "package")
                .cloned()
        })
        .collect()
}

/// The first campaign other than `own` that chose `package`.
pub fn used_elsewhere(
    package: Uuid,
    own: Option<Uuid>,
    campaigns: &[Campaign],
    manifests: &[Entry],
) -> Option<String> {
    campaigns
        .iter()
        .filter(|campaign| Some(campaign.id) != own)
        .find(|campaign| {
            CampaignDocument::load(campaign)
                .is_ok_and(|document| !copy_choices(&document, package, manifests).is_empty())
        })
        .map(|campaign| campaign.name.clone())
}

/// The copy's package choices in a campaign file, in any form that names it.
pub fn copy_choices(
    document: &CampaignDocument,
    package: Uuid,
    manifests: &[Entry],
) -> Vec<crate::content_workspace::PackageSelection> {
    document
        .selections
        .iter()
        .filter(|selection| chosen_package(&selection.package, manifests) == Some(package))
        .cloned()
        .collect()
}

impl Registration {
    /// The result's words. They say what the registration did, not what the campaign uses now.
    pub fn words(&self) -> Sentence {
        let name = &self.copy.module.name;
        match (self.clause, self.written.is_some()) {
            (false, _) => crate::sentence!("{} is registered.", name),
            (true, true) => crate::sentence!(
                "{} is registered, with every entry chosen for this campaign.",
                name
            ),
            (true, false) => crate::sentence!(
                "{} is registered, with no entries chosen for this campaign.",
                name
            ),
        }
    }

    /// The campaign file's part of Undo: the copy's choices leave it, every other choice and
    /// word stays. When the file still holds exactly what the registration wrote, it goes back
    /// to its words before. The write is checked against the words read here.
    pub fn campaign_undo(
        &self,
        manifests: &[Entry],
    ) -> Result<Option<(PathBuf, String, String, String)>, String> {
        let Some(campaign) = &self.campaign else {
            return Ok(None);
        };
        let document = CampaignDocument::load(campaign)?;
        if let Some((path, before, after)) = &self.written
            && *path == document.path
            && document.text == *after
        {
            return Ok(Some((
                campaign.path.clone(),
                path.clone(),
                after.clone(),
                before.clone(),
            )));
        }
        let keep: Vec<_> = document
            .selections
            .iter()
            .filter(|selection| {
                chosen_package(&selection.package, manifests) != Some(self.copy.module.id)
            })
            .cloned()
            .collect();
        if keep.len() == document.selections.len()
            && !document
                .text
                .contains(&format!(" ^{}", self.copy.module.id))
        {
            return Ok(None);
        }
        // Only the copy's own task lines (with their `Only:` lines) leave the file, by the
        // reader's ranges, from the file's own words. The file's writer (`text_with`) is not
        // used: it would rewrite other packages' lines (a hand-written `Only: [[spell/aid]]`
        // becomes a key reference), and this Undo touches nothing but the copy's lines.
        let links: Vec<&str> = document
            .selections
            .iter()
            .filter(|selection| {
                chosen_package(&selection.package, manifests) == Some(self.copy.module.id)
            })
            .filter_map(|selection| selection.package.link.as_deref())
            .collect();
        let text = without_package_lines(&document.text, self.copy.module.id, &links);
        // The guard after the removal, before the write: the file read back must hold exactly
        // the choices that stay. A form of the list that the change would alter otherwise (an
        // item that turns into a code block, say) refuses the Undo, and nothing is written.
        let read_back = Entry::parse(&document.path, &text).ok().and_then(|entry| {
            match entry.fields.get("splat_packages") {
                Some(value) => serde_yaml_ng::from_value::<
                    Vec<crate::content_workspace::PackageSelection>,
                >(value.clone())
                .ok(),
                None => Some(Vec::new()),
            }
        });
        if read_back.as_ref() != Some(&keep) {
            return Err(PACKAGE_LIST_UNSAFE.into());
        }
        Ok(Some((
            campaign.path.clone(),
            document.path.clone(),
            document.text.clone(),
            text,
        )))
    }
}

/// `Use the copy in encounters`: the encounter files it changed, with its Undo. It belongs to
/// its registration's owner: that registration's Undo also reverses it, and no other's does.
pub struct RostersUse {
    pub owner: RegistrationOwner,
    pub edits: crate::content_workspace::ContentEdits,
    /// One of its files changed outside Splat: its Undo (and the registration's) waits.
    pub blocked: bool,
}

impl design::results::Kept for RostersUse {
    type Owner = RegistrationOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // Its registration's Undo ends it.
        owner_changed: design::results::OwnerChanged::Ends,
        // A reload ends it with its registration (`reconcile_registrations`).
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> RegistrationOwner {
        self.owner
    }
}

impl design::results::Reconcile for RostersUse {
    type With = ();
    fn reconcile(&mut self, (): &()) {
        self.blocked = !self.edits.on_disk();
    }
}

/// A draft that `Remove entry and lose draft` dropped after its file left the disk, for one step of undo, and
/// whether this campaign used its entry then.
pub struct LetGo {
    pub note: Note,
    pub used: bool,
    /// Whether the DM chose the entry itself, not only needed it.
    pub chosen: bool,
    /// The campaign that was open when the entry was removed, once the page has seen it. Its
    /// `used` flag and the catalogue it goes back into belong to that campaign.
    pub campaign: Option<Option<Uuid>>,
}

/// The offer to change links after a rename. It belongs to the renamed entry (by its id, in
/// its root) and shows only in that entry's reader. It carries the version of the entry's file
/// it was made from: the root, the path and the exact text the rename saved. The offer holds
/// only while that file still has those bytes (round 26 review).
/// Its footer order is the store's (`Library::renamed_links`), issued when the rename (or the
/// Undo that gave the offer back) made it.
#[derive(Clone)]
pub struct LinkOffer {
    pub owner: Uuid,
    pub source: OfferSource,
    pub old: String,
    pub new: String,
}

/// The entry a link offer or a link result belongs to: its identity in its root. The root is
/// its scope, so another campaign's or package's entry with the same identity never shows it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkOwner {
    pub id: Uuid,
    pub root: PathBuf,
}

impl design::results::Owner for LinkOwner {
    type Scope = Path;
    fn scope(&self) -> &Path {
        &self.root
    }
}

impl design::results::Kept for LinkOffer {
    type Owner = LinkOwner;
    /// A saved edit of its entry ends it (a rename that passes the check gives a new one); a
    /// reload keeps it while it still holds; its press uses it up, written or not.
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Use,
    };
    fn owner(&self) -> LinkOwner {
        LinkOwner {
            id: self.owner,
            root: self.source.root.clone(),
        }
    }
}

/// The renamed entry's file as the rename saved it.
#[derive(Clone, PartialEq)]
pub struct OfferSource {
    pub root: PathBuf,
    pub path: String,
    pub text: String,
}

/// A link update's result and its Undo. It belongs to the renamed entry and shows only in its
/// reader: while another entry is open it is hidden, not ended. It ends on its owner's next link
/// update or saved edit, its Undo, or a change on disk to a file it wrote. Its Undo gives back
/// the rename's offer, so the links that say the old name again can be changed again (round 26
/// FD6).
pub struct LinkResult {
    pub owner: Uuid,
    /// The offer's source version, which its Undo gives back to the offer.
    pub source: OfferSource,
    pub edits: crate::content_workspace::ContentEdits,
    pub old: String,
    pub new: String,
}

impl design::results::Kept for LinkResult {
    type Owner = LinkOwner;
    /// A saved edit of its entry ends it; a reload keeps it while its files hold what it wrote;
    /// it leaves by its Undo only when the Undo works.
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> LinkOwner {
        LinkOwner {
            id: self.owner,
            root: self.source.root.clone(),
        }
    }
}

/// What the typed footers of a pane need to know besides the stores: whether this pane shows
/// the package choice, the manifest write and the separate-entry results, and the open draft's
/// lock on their Undo.
#[derive(Default)]
pub struct ReaderResults {
    /// Every entry in this package is now chosen for this campaign.
    pub package_use: bool,
    /// A manifest write's result.
    pub manifest: bool,
    /// A folder's separate-entries result.
    pub identities: bool,
    /// One file's separate-entry result.
    pub identity: bool,
    /// Why Undo of a file change waits now: the open draft, or its changed file. The Undo
    /// paints it on its own line (ruling Q-u); while open, the line keeps its room.
    pub locked: Option<&'static design::outcome::Sentence>,
    /// Whether that lock can come in place: in Edit Markdown, or with a draft already open. In
    /// Read with no draft, it comes only with a change of mode, which lays the pane out again,
    /// so the Undo keeps no room for it.
    pub lock_can_come: bool,
}

impl ReaderResults {
    /// `step`'s Undo, locked while the open entry's draft (or its changed file) holds it. Each
    /// Undo says the reason under its own words, its room kept while open (R29), so the lock
    /// coming or going moves nothing below it.
    pub fn draft_lock<A>(&self, step: UndoStep, id: A) -> design::outcome::Undo<A> {
        let undo = step.pressed_as(id);
        if !self.lock_can_come && self.locked.is_none() {
            return undo;
        }
        undo.locked_when(
            self.locked.is_some(),
            self.locked.unwrap_or(&UNDO_WAITS_FOR_DRAFT).clone(),
        )
    }
}

/// Why the scope widened to show the entry `name`: the scope notice's one form of words.
pub fn scope_notice_words(name: &str) -> Sentence {
    crate::sentence!("Scope is now Everything, to show {}.", name)
}

/// The result of `Choose all` on a package About page.
pub const PACKAGE_USE_RESULT: Sentence =
    crate::sentence!("Every entry in this package is now chosen for this campaign.");

/// Why a link update wrote nothing: the renamed entry changed on disk since the offer was made.
pub static LINKS_NOT_CHANGED: Sentence = crate::sentence!(
    "No links were changed. This entry changed on disk since it was renamed. Reload from disk, then rename it again."
);

/// The warning for a draft whose file changed on disk (R10): the entry by its name, or `This
/// entry` when it has none.
pub fn changed_on_disk(name: &str) -> Sentence {
    let thing = if name.trim().is_empty() {
        "This entry"
    } else {
        name.trim()
    };
    crate::sentence!(
        "{} changed on disk. Your draft is safe until you choose.",
        thing
    )
}

/// The warning for a draft whose file moved on disk. Its two choices name what each one does.
pub const MOVED_ON_DISK: Sentence =
    crate::sentence!("This entry moved on disk. Your draft is safe until you choose.");
/// The warning for a draft whose file was deleted on disk.
pub const DELETED_ON_DISK: Sentence =
    crate::sentence!("This entry’s file was deleted on disk. Your draft is safe until you choose.");

/// A row's short cause for an entry whose identity or key another file claims too.
pub const SAME_ENTRY: &str = "Same entry in two files";
/// A row's short cause for a first heading that does not say the entry's name.
pub const HEADING_DIFFERS: &str = "First heading differs";
/// A row's short cause for any other problem the header counts in its file.
pub const NEEDS_FIXING: &str = "Needs fixing";

/// A list row's line without the fault it leads with (`⚠ First heading differs · `). The reader
/// states that fault in its own warning, so its kind line does not. The rest of the line stays,
/// with the folder that tells two packages of one name apart.
pub fn row_line_without_fault(line: &str) -> String {
    let Some(mut rest) = line.strip_prefix(&format!("{} ", icons::WARN)) else {
        return line.to_owned();
    };
    // One mark, then one or more causes, each followed by ` · `.
    let mut stripped = false;
    while let Some(after) = [SAME_ENTRY, HEADING_DIFFERS, NEEDS_FIXING]
        .into_iter()
        .find_map(|cause| rest.strip_prefix(&format!("{cause} · ")))
    {
        rest = after;
        stripped = true;
    }
    if stripped {
        rest.to_owned()
    } else {
        line.to_owned()
    }
}

pub fn local_module(path: &Path) -> Result<Module, String> {
    let path = path.canonicalize().map_err(|e| e.to_string())?;
    let (entries, _) = content::read_directory(&path)?;
    module_from_entries(path, &entries)
}

fn module_from_entries(path: PathBuf, entries: &[Entry]) -> Result<Module, String> {
    let manifests: Vec<_> = entries.iter().filter(|e| e.kind == "package").collect();
    if manifests.len() != 1 || entries.iter().any(|e| e.kind == "campaign") {
        return Err(
            "Choose a package folder with exactly one package manifest and no campaign manifest."
                .into(),
        );
    }
    let manifest = manifests[0];
    let ns = manifest
        .fields
        .get("splat_namespace")
        .and_then(|v| v.as_str())
        .ok_or("The package needs a namespace.")?;
    if !content::resource_key(&format!("{ns}:library")) || ns == "dnd2024" {
        return Err(
            "Use a valid package namespace other than the built-in dnd2024 namespace.".into(),
        );
    }
    Ok(Module {
        id: manifest.id,
        name: manifest.name.clone(),
        path,
    })
}

pub fn note_text(name: &str, kind: &str, namespace: Option<&str>) -> String {
    let mut fields = serde_yaml_ng::Mapping::new();
    fields.insert("splat_schema".into(), 2.into());
    fields.insert(
        "splat_id".into(),
        crate::hermetic::new_uuid().to_string().into(),
    );
    fields.insert("splat_kind".into(), kind.into());
    fields.insert("splat_name".into(), name.into());
    if namespace.is_none() {
        fields.insert(
            format!("splat_{kind}").into(),
            serde_yaml_ng::Value::Mapping(serde_yaml_ng::Mapping::new()),
        );
    }
    if let Some(ns) = namespace {
        fields.insert("splat_namespace".into(), ns.into());
    }
    if kind == "world" {
        fields.remove("splat_world");
        fields.insert("splat_type".into(), "other".into());
    }
    if kind == "creature" {
        fields.insert(
            "splat_creature".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "size: medium\ntype: humanoid\nalignment: Neutral\nmovement_text: 30 ft.\ncr_text: '1 (XP 200; PB +2)'\n",
            )
            .expect("creature identity"),
        );
        fields.insert(
            "splat_stats".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "initiative: {value: 0}\nspeed: {value: 30, unit: ft}\nstrength: {value: 10}\ndexterity: {value: 10}\nconstitution: {value: 10}\nintelligence: {value: 10}\nwisdom: {value: 10}\ncharisma: {value: 10}\n",
            )
            .expect("creature fields"),
        );
    }
    if kind == "spell" {
        fields.insert("splat_spell".into(), serde_yaml_ng::from_str::<serde_yaml_ng::Value>("level: 1\nschool: evocation\ncasting_time: Action\nrange_text: 60 feet\ncomponents: [verbal, somatic]\nduration: Instantaneous\n").expect("spell fields"));
    }
    if kind == "item" {
        fields.insert(
            "splat_item".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "category: wondrous_item\nrarity: common\nequipment: {attunement: none}\n",
            )
            .expect("item fields"),
        );
    }
    if ["creature", "spell", "item"].contains(&kind) {
        fields.insert("splat_schema".into(), 2.into());
        let table = content::stat_blocks::render(&fields, kind);
        for key in content::stat_blocks::data_roots(kind) {
            fields.remove(*key);
        }
        let yaml = serde_yaml_ng::to_string(&fields).expect("string fields");
        return format!(
            "---\n{yaml}---\n\n# {name}\n\n{table}{}",
            if kind == "creature" {
                "## Traits\n\n**Trait.** Describe the trait.\n\n## Actions\n\n**Attack.** Describe the attack.\n\n"
            } else if kind == "spell" {
                "Describe the spell.\n"
            } else {
                "Describe the item and its use.\n"
            }
        );
    }
    if kind == "campaign" {
        fields.remove("splat_campaign");
    }
    let yaml = serde_yaml_ng::to_string(&fields).expect("string fields");
    format!(
        "---\n{yaml}---\n\n# {name}\n\n{}",
        if kind == "campaign" {
            "## Packages\n\n## Rule choices\n\n| Rule | Choice |\n| --- | --- |\n"
        } else if kind == "effect" {
            "Describe what changes and when it ends.\n"
        } else if kind == "rule" {
            "Describe when and how this rule applies.\n"
        } else if kind == "rule_group" {
            "## Rules\n\nDescribe the rules this group enables.\n"
        } else {
            ""
        }
    )
}

/// Existing folders, with an optional kind folder as the first suggestion.
pub fn kind_folder(kind: &str) -> &'static str {
    match kind {
        "creature" => "Creatures",
        "spell" => "Spells",
        "item" => "Items",
        "effect" => "Effects",
        _ => "Rules",
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many folder reads `visit_folders` made on this thread, for the tests' idle bound.
    pub static FOLDER_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Every folder under `root` (no hidden ones) and `kind`'s own folder, sorted, each once: the
/// folder field's suggestions, read from disk.
pub fn folder_choices(root: &Path, kind: Option<&str>) -> Vec<String> {
    let mut folders = Vec::new();
    visit_folders(root, root, &mut folders);
    if let Some(kind) = kind {
        folders.push(kind_folder(kind).to_owned());
    }
    folders.sort();
    folders.dedup();
    folders
}

fn visit_folders(root: &Path, path: &Path, folders: &mut Vec<String>) {
    #[cfg(any(test, feature = "test-support"))]
    FOLDER_READS.with(|reads| reads.set(reads.get() + 1));
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        if entry.file_type().is_ok_and(|kind| kind.is_dir())
            && !entry.file_name().to_string_lossy().starts_with('.')
        {
            if let Ok(path) = entry.path().strip_prefix(root) {
                folders.push(path.to_string_lossy().to_string());
            }
            visit_folders(root, &entry.path(), folders);
        }
    }
}

pub const REGISTER: design::Word = crate::word!("Register");
pub const BROWSE: design::Word = crate::word!("Browse…");
pub const CANCEL: design::Word = crate::word!("Cancel");

pub fn create_module(path: &Path, name: &str, namespace: &str) -> Result<Module, String> {
    if !path.is_absolute()
        || name.trim().is_empty()
        || namespace == "dnd2024"
        || !content::resource_key(&format!("{namespace}:library"))
    {
        return Err(
            "Enter a name, a new absolute folder path, and a namespace such as my_homebrew.".into(),
        );
    }
    fs::create_dir(path).map_err(|e| format!("Could not create a new package folder: {e}"))?;
    let text = note_text(name.trim(), "package", Some(namespace));
    if let Err(error) = write_note(path, "Package.md", None, &text) {
        let _ = fs::remove_dir(path);
        return Err(error);
    }
    local_module(path)
}

// Refuse traversal and symlinks, and preserve external edits before atomic replacement.
/// The rule a note's path must keep, checked without the disk: relative, inside its folder.
pub(crate) fn note_path_rule(relative: &str) -> Option<&'static str> {
    let relative = Path::new(relative);
    (relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_))))
    .then_some("Use a relative note path within the package.")
}

pub fn note_path(root: &Path, relative: &str) -> Result<PathBuf, String> {
    if fs::symlink_metadata(root)
        .map_err(|error| {
            crate::world::file_fault::FileFault::read(&root.display().to_string(), &error).message()
        })?
        .file_type()
        .is_symlink()
    {
        return Err(
            "The package folder was replaced with a symbolic link. Register the actual folder."
                .into(),
        );
    }
    if let Some(rule) = note_path_rule(relative) {
        return Err(rule.into());
    }
    let relative = Path::new(relative);
    let mut path = root.to_path_buf();
    for component in relative.components() {
        path.push(component);
        if fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("Editing through symbolic links is not supported.".into());
        }
    }
    Ok(path)
}

/// What makes `write_note` refuse a note before it writes: the file is read only, or the
/// campaign folder is. Save's reason names it and says what to do (AL1, FL2, SL1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadOnly {
    File,
    Folder,
}

impl ReadOnly {
    pub fn of(root: &Path, path: &Path) -> Option<Self> {
        if fs::metadata(path).is_ok_and(|m| m.permissions().readonly()) {
            Some(Self::File)
        } else if fs::metadata(root).is_ok_and(|m| m.permissions().readonly()) {
            Some(Self::Folder)
        } else {
            None
        }
    }

    pub fn reason(self) -> &'static str {
        match self {
            Self::File => "This file is read only. Make it writable, then Save again.",
            Self::Folder => "The campaign folder is read only. Make it writable, then Save again.",
        }
    }
}

pub fn write_note(
    root: &Path,
    relative: &str,
    baseline: Option<&str>,
    text: &str,
) -> Result<Entry, String> {
    let entry = Entry::parse(relative, text)?;
    let path = note_path(root, relative)?;
    if let Some(read_only) = ReadOnly::of(root, &path) {
        return Err(read_only.reason().into());
    }
    let current = match fs::read_to_string(&path) {
        Ok(text) => Some(text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(crate::world::file_fault::FileFault::read(relative, &e).message()),
    };
    if baseline.is_none() && current.is_some() {
        return Err("A note with this name already exists. Choose another name.".into());
    }
    if current.as_deref() != baseline {
        return Err(NOTE_CHANGED.as_str().into());
    }
    // The system's words name no file; the DM reads which note and how to recover.
    let failed = crate::world::file_fault::write_error(relative);
    if baseline.is_none() {
        fs::create_dir_all(path.parent().ok_or("Missing parent folder")?).map_err(&failed)?;
    }
    let mut temp = tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent folder")?)
        .map_err(&failed)?;
    temp.write_all(text.as_bytes())
        .and_then(|_| temp.as_file().sync_all())
        .map_err(&failed)?;
    if baseline.is_some() {
        temp.persist(&path).map_err(|error| failed(error.error))?;
    } else {
        temp.persist_noclobber(&path)
            .map_err(|error| failed(error.error))?;
    }
    Ok(entry)
}

/// A note that the folder reading reported a problem with. A file the system will not read keeps
/// its fault, from the error's kind, so the reader names the real cause and never offers a
/// repair of text it does not have. Nothing here changes the file.
fn problem_note(
    root: &Path,
    folder: &content::Folder,
    error: content::Diagnostic,
    owner: Owner,
) -> Result<Note, String> {
    let full = note_path(root, &error.path)?;
    let read = match folder.text(&error.path) {
        Some(text) => Ok(text.to_owned()),
        // A folder that could not be listed: its own error kind says why.
        None if full.is_dir() => fs::read_dir(&full).map(|_| String::new()),
        None => fs::read_to_string(&full),
    };
    let (text, fault) = match read {
        Ok(text) => (text, None),
        Err(e) => (String::new(), Some(FileFault::read(&error.path, &e))),
    };
    Ok(Note {
        error: Some(
            fault
                .as_ref()
                .map_or(error.message, |fault| fault.cause().to_owned()),
        ),
        path: error.path,
        text: crate::design::memo::Revised::new(text),
        entry: None,
        fault,
        manifest_gone: false,
        owner,
    })
}

pub fn load(module: Option<&Module>) -> Result<Loaded, String> {
    let mut notes = Vec::new();
    if let Some(module) = module {
        let path = module.path.canonicalize().map_err(|e| e.to_string())?;
        let folder = content::read_folder(&path)?;
        let (entries, mut errors) = (folder.entries.clone(), folder.errors.clone());
        let manifest_path = crate::content_workspace::manifest_path_in(&folder, module);
        let has_manifest = entries.iter().any(|entry| entry.kind == "package");
        let missing_manifest = !has_manifest
            && !crate::content_workspace::manifest_file_exists(module, &manifest_path);
        // The folder was just read; its texts are the ones these entries came from.
        let text_of = |path: &str| -> Result<String, String> {
            let full = note_path(&module.path, path)?;
            match folder.text(path) {
                Some(text) => Ok(text.to_owned()),
                None => fs::read_to_string(full).map_err(|e| FileFault::read(path, &e).message()),
            }
        };
        if !has_manifest
            && !missing_manifest
            && !errors.iter().any(|error| error.path == manifest_path)
        {
            let message = text_of(&manifest_path)
                .and_then(|text| Entry::parse(&manifest_path, &text))
                .err();
            if let Some(message) = message {
                errors.push(content::Diagnostic {
                    path: manifest_path.clone(),
                    message,
                });
            }
        }
        for entry in entries {
            let text = text_of(&entry.path)?;
            let mismatched = entry.kind == "package" && entry.id != module.id;
            notes.push(Note {
                path: entry.path.clone(),
                text: crate::design::memo::Revised::new(text),
                entry: (!mismatched).then_some(entry),
                error: mismatched.then(|| "Package details have another identity. Replace with the registered details, or restore the original id.".into()),
                fault: None,
                manifest_gone: false,
                owner: Owner::Package(module.clone()),
            });
        }
        if missing_manifest {
            notes.push(Note {
                path: manifest_path.clone(),
                text: crate::design::memo::Revised::default(),
                entry: None,
                error: Some(crate::content_workspace::DELETED_MANIFEST.into()),
                fault: None,
                manifest_gone: true,
                owner: Owner::Package(module.clone()),
            });
        }
        for error in errors {
            notes.push(problem_note(
                &module.path,
                &folder,
                error,
                Owner::Package(module.clone()),
            )?);
        }
    } else {
        for entry in content::definitions::entries()?.iter().cloned() {
            let text = content::definitions::source_file(&entry.path)
                .ok_or("Missing built-in note")?
                .to_owned();
            notes.push(Note {
                path: entry.path.clone(),
                text: crate::design::memo::Revised::new(text),
                entry: Some(entry),
                error: None,
                fault: None,
                manifest_gone: false,
                owner: Owner::Builtin,
            });
        }
    }
    notes.sort_by_cached_key(|n| natural_key(n.name()));
    Ok(Loaded {
        notes,
        ..Default::default()
    })
}

/// The whole catalogue: built-in, every package, and the active campaign's own entries.
pub fn load_all(modules: &[Module], campaign: Option<&Campaign>) -> Result<Loaded, String> {
    let mut all = load(None)?;
    all.owners = modules
        .iter()
        .cloned()
        .map(Owner::Package)
        .chain(campaign.cloned().map(Owner::Campaign))
        .collect();
    for module in modules {
        // A load error names its package, and its folder when another package has the same name.
        let owner = Owner::Package(module.clone());
        let name = if modules.iter().filter(|m| m.name == module.name).count() > 1 {
            format!("{} · {}/", module.name, owner.folder())
        } else {
            module.name.clone()
        };
        let key = (OwnerKey::Package(module.id), String::new());
        if let Err(error) = splat::content::art::ArtIndex::default().add_package(&module.path) {
            all.load_errors
                .push((key.clone(), format!("{name}: {error}")));
        }
        match load(Some(module)) {
            Ok(package) => {
                all.notes.extend(package.notes);
            }
            Err(error) => all.load_errors.push((key, format!("{name}: {error}"))),
        }
    }
    if let Some(campaign) = campaign {
        let owner = Owner::Campaign(campaign.clone());
        match content::read_folder(&campaign.path) {
            Ok(folder) => {
                let (entries, errors) = (folder.entries.clone(), folder.errors.clone());
                let text_of = |path: &str| -> Result<String, String> {
                    let full = note_path(&campaign.path, path)?;
                    match folder.text(path) {
                        Some(text) => Ok(text.to_owned()),
                        None => fs::read_to_string(full)
                            .map_err(|e| FileFault::read(path, &e).message()),
                    }
                };
                for entry in entries.into_iter().filter(|e| {
                    e.kind != "campaign" && !splat::character::campaign_package::is_manifest(e)
                }) {
                    let text = text_of(&entry.path)?;
                    all.notes.push(Note {
                        path: entry.path.clone(),
                        text: crate::design::memo::Revised::new(text),
                        entry: Some(entry),
                        error: None,
                        fault: None,
                        manifest_gone: false,
                        owner: owner.clone(),
                    });
                }
                for error in errors {
                    all.notes
                        .push(problem_note(&campaign.path, &folder, error, owner.clone())?);
                }
            }
            Err(error) => all.load_errors.push((
                (OwnerKey::Campaign(campaign.id), String::new()),
                format!("{}: {error}", campaign.name),
            )),
        }
        // If the selections cannot be read, say so and show everything rather than nothing.
        match CampaignDocument::load(campaign)
            .and_then(|document| Ok((document, Catalogue::load(modules, Some(campaign))?)))
        {
            Ok((document, catalogue)) => {
                let closure = catalogue.closure(&document.selections, campaign.id);
                all.chosen = Some(closure.direct.iter().copied().collect());
                all.uses = Some(
                    closure
                        .direct
                        .iter()
                        .copied()
                        .chain(closure.required.keys().copied())
                        .collect(),
                );
            }
            Err(error) => all.load_errors.push((
                (OwnerKey::Campaign(campaign.id), String::new()),
                format!("{}: {error}", campaign.name),
            )),
        }
    }
    all.notes.sort_by_cached_key(|n| natural_key(n.name()));
    all.refresh_diagnostics();
    Ok(all)
}

/// Undo discard offers words only to the note they came from. Two owners can each have a
/// note at the same path.
pub fn discard_is_for(discarded: Option<&(OwnerKey, String, String)>, note: &Note) -> bool {
    discarded.is_some_and(|(owner, path, _)| *owner == note.owner.key() && *path == note.path)
}

/// The catalogue needs reloading when either of these changes.
pub type LoadKey = (Option<Uuid>, Vec<Uuid>);

pub fn load_key(campaigns: &Campaigns) -> LoadKey {
    (
        campaigns.active_id(),
        campaigns.modules().iter().map(|m| m.id).collect(),
    )
}

pub fn catalogue_subtitles(
    catalogue: &crate::content_workspace::Catalogue,
) -> std::collections::HashMap<Uuid, String> {
    let loaded = Loaded {
        notes: catalogue
            .entries
            .values()
            .map(|e| Note {
                path: e.entry.path.clone(),
                text: crate::design::memo::Revised::default(),
                entry: Some(e.entry.clone()),
                error: None,
                fault: None,
                manifest_gone: false,
                owner: Owner::Builtin,
            })
            .collect(),
        ..Default::default()
    };
    loaded
        .notes
        .iter()
        .zip(loaded.subtitles())
        .filter_map(|(note, suffix)| Some((note.entry.as_ref()?.id, suffix.clone())))
        .collect()
}

/// Sort so that "Level 2" comes before "Level 18".
pub fn natural_key(name: &str) -> Vec<(String, u64)> {
    let lower = name.to_lowercase();
    let mut parts = Vec::new();
    let mut text = String::new();
    let mut chars = lower.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            let mut number = u64::from(c.to_digit(10).unwrap_or(0));
            while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
                number = number.saturating_mul(10).saturating_add(u64::from(digit));
                chars.next();
            }
            parts.push((std::mem::take(&mut text), number));
        } else {
            text.push(c);
        }
    }
    parts.push((text, 0));
    parts
}

pub static NOTE_CHANGED: Sentence = crate::sentence!("This entry changed on disk.");
