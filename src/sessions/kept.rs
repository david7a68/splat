//! The Session page's kept results, each kind in its own `design::Results` store, with its owner
//! and the events that end it (`Kept::ENDS`). The store issues each result's order at the act,
//! and a footer press reaches its own entry by the press's ticket.
use crate::design::results::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// The Session page of one campaign's session folder: a result of the page that one later act
/// of the same kind replaces. A campaign switch hides it (another folder), and coming back
/// shows it again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Page {
    pub root: PathBuf,
}

impl Owner for Page {
    type Scope = Path;
    fn scope(&self) -> &Path {
        &self.root
    }
}

/// One session of a campaign's session folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct OfSession {
    pub root: PathBuf,
    pub session: Option<Uuid>,
}

impl Owner for OfSession {
    type Scope = Path;
    fn scope(&self) -> &Path {
        &self.root
    }
}

/// One campaign's session folder setting: the Session folder form (Campaign's Link and folder
/// page). It is the campaign's folder, not the session folder, because a move changes the
/// session folder. Another campaign hides its results, and coming back shows them again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FolderLink {
    pub campaign: PathBuf,
}

impl Owner for FolderLink {
    type Scope = Path;
    fn scope(&self) -> &Path {
        &self.campaign
    }
}

/// `<event> added to <session>.` with `Undo add`: the plan's Add strip added an event to the
/// session's timeline. It lives `plan::UNDO_ADD_FOR` on the frame clock; the Sessions owner
/// update (`Sessions::settle_timed`) ends it then, and when the events no longer hold what the
/// add left (their count, its label), checked only when the book's revision changed. Another
/// session, or the DM's next act on the plan or its note, ends it (`owner_changed`).
pub(super) struct PlanAdded {
    pub owner: OfSession,
    /// The added event.
    pub id: String,
    /// Its label as the add left it (`Day 2 · Long rest`).
    pub label: String,
    /// The session's name at the act (`Session 3`), as the label is fixed at the act.
    pub to: Option<String>,
    /// How many events the book held after the add.
    pub count: usize,
}

impl Kept for PlanAdded {
    type Owner = OfSession;
    const ENDS: Ends = Ends {
        owner_changed: OwnerChanged::Ends,
        // The owner update's events check ends it (`Sessions::settle_timed`).
        disk_changed: DiskChanged::Checked,
        press: Press::Undo,
    };
    const LIFE: Option<std::time::Duration> = Some(super::plan::UNDO_ADD_FOR);
    fn owner(&self) -> OfSession {
        self.owner.clone()
    }
}

/// `Moved <counts> to <folder>.`: Move files and save moved the session files of one campaign.
pub(super) struct FolderMoved {
    pub link: FolderLink,
    /// What moved, in words: `1 note, 4 events and 2 templates`.
    pub counts: String,
    /// The folder as the DM typed it.
    pub typed: String,
    /// The folder the files are in now.
    pub to: PathBuf,
}

impl Kept for FolderMoved {
    type Owner = FolderLink;
    const ENDS: Ends = Ends {
        // The DM's next edit of the folder field, and the next save or failed save of the
        // folder (`folder_view`).
        owner_changed: OwnerChanged::Ends,
        // A disk scan (`refresh`) ends it when the folder it moved to is gone.
        disk_changed: DiskChanged::Checked,
        press: Press::Nothing,
    };
    fn owner(&self) -> FolderLink {
        self.link.clone()
    }
}

/// The Sessions page's one notice (#597): what the last act or a job said, in its tone, for one
/// session or for whichever is open (`session: None`), and whether it arrived by itself (a
/// recording processed while the DM was elsewhere), which places it in the bottom banner. The
/// page holds one: a new one replaces it (`record`), and a press of its close icon, an act that
/// answers it or a change of what it was about ends it (`Sessions::end_notice`). A success is a
/// kept line by the store's order and ticket; the other tones are notices with no order.
pub(super) struct PageNotice {
    pub page: Page,
    pub tone: crate::design::Tone,
    pub words: String,
    pub session: Option<Uuid>,
    pub arrived: bool,
}

impl Kept for PageNotice {
    type Owner = Page;
    const ENDS: Ends = Ends {
        // The page ends it where its rules say (`end_notice`).
        owner_changed: OwnerChanged::Ends,
        disk_changed: DiskChanged::Stays,
        // Its close icon uses it up.
        press: Press::Use,
    };
    fn owner(&self) -> Page {
        self.page.clone()
    }
}

/// A draft's file: the session note or the template a draft edits. A result of that draft is
/// its file's; another draft's file hides it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct DraftFile {
    pub path: PathBuf,
}

impl Owner for DraftFile {
    type Scope = Path;
    fn scope(&self) -> &Path {
        &self.path
    }
}

/// `Reloaded from disk. …` with `Undo reload`, or `Draft discarded.` with `Undo discard`: the
/// words a draft's warning dropped (M0437). The words stay with the draft (`Draft::dropped`),
/// which lets them go at an edit, at its Undo or at another change of its file; this entry is
/// their result, recorded at the act, and ends with them (`Sessions::end_dropped`).
pub(super) struct Dropped {
    pub file: DraftFile,
    pub reloaded: bool,
}

impl Kept for Dropped {
    type Owner = DraftFile;
    const ENDS: Ends = Ends {
        // The draft letting the words go ends it (`end_dropped`).
        owner_changed: OwnerChanged::Ends,
        // Its Undo puts the words back as the draft holds them; a file change ends the words.
        disk_changed: DiskChanged::Stays,
        press: Press::Undo,
    };
    fn owner(&self) -> DraftFile {
        self.file.clone()
    }
}

/// `<file> is now a separate session.` and its Undo. The copy's file holds the new identity
/// until the Undo writes the old one back.
pub(super) struct Separate {
    pub page: Page,
    pub undo: crate::sessions::IdentityUndo,
}

impl Kept for Separate {
    type Owner = Page;
    const ENDS: Ends = Ends {
        // A later separate-session act replaces it (`record`); no other act of the page ends it.
        owner_changed: OwnerChanged::Stays,
        // Its Undo reads the copy's file at the press, and says why when it cannot apply.
        disk_changed: DiskChanged::Stays,
        press: Press::Undo,
    };
    fn owner(&self) -> Page {
        self.page.clone()
    }
}

/// Round 27 FI3, FN2: a marker's `Add` result, kept by the session whose recording holds the
/// marker and the Timeline event it made. It shows in that session's footer while the event
/// stands; `Undo add` or leaving the Recordings tab ends it.
pub(super) struct MarkerAdd {
    pub owner: OfSession,
    pub id: String,
    pub title: String,
}

impl Kept for MarkerAdd {
    type Owner = OfSession;
    const ENDS: Ends = Ends {
        // Leaving the Recordings tab ends it (`end_marker_results`).
        owner_changed: OwnerChanged::Ends,
        // It shows only while its event stands (the footer checks the Timeline each frame).
        disk_changed: DiskChanged::Stays,
        press: Press::Undo,
    };
    fn owner(&self) -> OfSession {
        self.owner.clone()
    }
}

/// `<fight> moved to Session <n>.` and `Undo move`: the run's event before the move, and the
/// state the Undo checks that nothing changed since.
pub(super) struct Moved {
    pub page: Page,
    /// The session the run moved to: its footer shows the result.
    pub session: Uuid,
    pub before: crate::timeline::Event,
    pub open_run: Option<(String, Uuid)>,
    /// The note's words when the run moved, with their revision (layout PR 9b-C).
    pub body: crate::design::memo::Revised<String>,
    pub events: Vec<crate::timeline::Event>,
    /// The fight's name and the session's number, for `<fight> moved to Session <n>.`
    pub name: String,
    pub number: u32,
}

impl Kept for Moved {
    type Owner = Page;
    const ENDS: Ends = Ends {
        // A change to the Timeline or the note since the move (also one a reload brings), and
        // the `stays in` notice of a new session, end it (`expire_move`,
        // `say_where_the_run_stays`).
        owner_changed: OwnerChanged::Ends,
        disk_changed: DiskChanged::Stays,
        press: Press::Undo,
    };
    fn owner(&self) -> Page {
        self.page.clone()
    }
}

/// `Deleted “<template>”.` and `Undo delete`: the template's name and its words, which the Undo
/// writes back to a file of that name.
pub(super) struct DeletedTemplate {
    pub page: Page,
    pub name: String,
    pub body: String,
}

impl Kept for DeletedTemplate {
    type Owner = Page;
    const ENDS: Ends = Ends {
        // An edit in the template editor ends it.
        owner_changed: OwnerChanged::Ends,
        // Its Undo refuses an existing file of that name, and says why.
        disk_changed: DiskChanged::Stays,
        press: Press::Undo,
    };
    fn owner(&self) -> Page {
        self.page.clone()
    }
}

/// `Links now say “<title>”.` and `Undo link update`: the notes a link update wrote. The update's
/// own state (what each note held) stays with the session page (`updated_links`); this entry is
/// its result, and ends with it.
pub(super) struct LinkUpdate {
    pub page: Page,
    pub title: String,
}

impl Kept for LinkUpdate {
    type Owner = Page;
    const ENDS: Ends = Ends {
        // A new update ends it. Note or Timeline edits and leaving Timeline also end it
        // unless a partial update or refused Undo still needs a retry.
        owner_changed: OwnerChanged::Ends,
        disk_changed: DiskChanged::Stays,
        press: Press::Undo,
    };
    fn owner(&self) -> Page {
        self.page.clone()
    }
}

/// One kind of the plan's kept result whose data the plan's own fields hold: a scene act's Undo
/// (`Scenes::added`, `Scenes::removed`), the group removed from tonight (`removed_group`), the
/// deleted run (`deleted_run`). One of each kind per campaign, as each kind's data is held once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PlanKind {
    SceneAdded,
    SceneRemoved,
    GroupRemoved,
    DeletedRun,
}

/// Whose plan result it is: the campaign's session folder and its kind. A campaign switch hides
/// it, and ends its data (`open_campaign`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PlanOwner {
    pub root: PathBuf,
    pub kind: PlanKind,
}

impl Owner for PlanOwner {
    type Scope = Path;
    fn scope(&self) -> &Path {
        &self.root
    }
}

/// A plan result in the plan's store: its owner only, for the footer's order (issued at the act)
/// and the press's ticket. Its words and its Undo come from the data its kind holds, which ends
/// it: when that data goes (an Undo, the note's next edit, another act of the kind), the plan
/// ends this entry before its footer is made (`Sessions::end_plan_marks`).
pub(super) struct PlanMark(pub PlanOwner);

impl Kept for PlanMark {
    type Owner = PlanOwner;
    const ENDS: Ends = Ends {
        // Its data was dropped (`end_plan_marks`).
        owner_changed: OwnerChanged::Ends,
        // A reload keeps the data it undoes; the data's own checks end it.
        disk_changed: DiskChanged::Stays,
        // Its Undo puts its data back; it leaves only when that was done.
        press: Press::Undo,
    };
    fn owner(&self) -> PlanOwner {
        self.0.clone()
    }
}

/// A refused act on a session note (a scene act, Carry over, their Undo), as its line says it:
/// the refusal's words, and the severity of its tone (R41: an unreadable note is an error).
pub(super) struct RefusalFault<'a>(pub &'a crate::design::results::Refusal);

impl crate::design::outcome::Fault for RefusalFault<'_> {
    fn severity(&self) -> crate::design::outcome::Severity {
        match self.0.tone() {
            crate::design::Tone::Error => crate::design::outcome::Severity::CannotRead,
            _ => crate::design::outcome::Severity::NeedsFixing,
        }
    }
    fn cause(&self) -> crate::design::outcome::Sentence {
        self.0.words()
    }
}

/// Why a note act could not change the session note, from the act's fault, and the step that
/// lets the DM try again. `undo`: the act is an Undo, so a changed line changed after the act.
pub(super) fn note_cause(
    fault: &crate::sessions::ActFault,
    undo: bool,
) -> (crate::design::results::Cause, crate::design::results::Step) {
    use crate::design::results::{Cause, File, Step};
    use crate::sessions::ActFault;
    let file = File::SessionNote;
    match fault {
        ActFault::LineChanged if undo => (Cause::ChangedAfter(file), Step::Check(file)),
        ActFault::LineChanged => (Cause::LineChanged(file), Step::Check(file)),
        ActFault::Changed => (Cause::Conflict(file), Step::ChooseWords(file)),
        ActFault::Missing => (Cause::Gone(file), Step::Check(file)),
        ActFault::OtherNote => (Cause::Replaced(file), Step::Check(file)),
        ActFault::NotSaved(..) => (Cause::NotSaved(file), Step::Check(file)),
    }
}
