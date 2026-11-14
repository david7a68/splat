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
        // folder (`folder_view`, `folder_save_row`).
        owner_changed: OwnerChanged::Ends,
        // A disk scan (`refresh`) ends it when the folder it moved to is gone.
        disk_changed: DiskChanged::Checked,
        press: Press::Nothing,
    };
    fn owner(&self) -> FolderLink {
        self.link.clone()
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
    pub body: String,
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
        // A new update, an edit of the notes or the Timeline since, or leaving the Timeline
        // ends it (`expire_event_link_undo`, `update_event_link_notes`).
        owner_changed: OwnerChanged::Ends,
        disk_changed: DiskChanged::Stays,
        press: Press::Undo,
    };
    fn owner(&self) -> Page {
        self.page.clone()
    }
}
