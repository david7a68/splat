//! The same source editor for broken notes in Campaign and World.
#![warn(deprecated)] // Its old message functions are gone; the parent file still allows them.
use super::file_fault::FileFault;
use crate::design::memo::Revised;
use crate::design::text_draft::{DraftKey, KeptCompare, TextDraft};
use crate::{
    design::{
        self,
        outcome::{
            Fault, FaultMessage, KeptOutcome, Messages, Notice, PaneFrame, Sentence, Severity,
            UndoStep, Verb,
        },
    },
    library::write_note,
    persistence::Campaign,
};
use uuid::Uuid;

/// The repair editor's accessible name. It names the file, so a DM who tabs between two open
/// repairs hears which is which. Selectors that ask for `Note text` still find it.
pub fn editor_name(path: &str) -> String {
    format!("Note text: {path}")
}

/// Why Save would not write this draft: a path outside its folder, a fault in its text and the
/// line to fix, or an identity that the campaign does not allow. It writes nothing and reads no
/// file: a refusal from the disk (read only, missing, a write error) is Save's own failure. Save, the save row's state, its cause
/// and Save's availability all use it, so `Unsaved` always means that Save would write.
pub fn refusal(
    campaign: &Campaign,
    path: &str,
    draft: &TextDraft,
) -> (Option<String>, Option<usize>) {
    // The save row asks on every frame; a parse of a long note takes about 1 ms. The result is
    // kept by the draft's key (layout PR 9b): a quiet frame hashes none of its words.
    static REFUSALS: splat::memo::Memo<(Option<String>, Option<usize>)> =
        splat::memo::Memo::new(64);
    REFUSALS.get_or(
        &[
            campaign.id.as_bytes(),
            path.as_bytes(),
            &draft.key().to_bytes(),
        ],
        || {
            let draft = draft.as_str();
            if let Some(rule) = crate::library::note_path_rule(path) {
                return (Some(rule.into()), None);
            }
            let (problem, line) = crate::content_workspace::repair_problem(path, draft);
            if problem.is_some() {
                return (problem, line);
            }
            match splat::content::Entry::parse(path, draft) {
                Ok(entry)
                    if entry.kind == "package"
                        || (entry.kind == "campaign" && entry.id != campaign.id) =>
                {
                    (Some(OWNERSHIP.into()), None)
                }
                Ok(_) => (None, None),
                Err(error) => (Some(error), None),
            }
        },
    )
}

/// Why Save refuses a note that would claim a package, or another campaign, as its identity.
pub const OWNERSHIP: &str = "Keep campaign ownership consistent with settings.";

/// Why a repair did not open.
#[derive(Debug)]
pub enum OpenError {
    /// The file is there, but the system will not read it as text.
    Unread(FileFault),
    Words(String),
}

/// The repair whose draft a kept result belongs to: one file of one campaign.
#[derive(Clone, Debug, PartialEq, Eq)]
struct RepairOwner {
    campaign: Uuid,
    path: String,
}

impl design::results::Owner for RepairOwner {
    type Scope = Uuid;
    fn scope(&self) -> &Uuid {
        &self.campaign
    }
}

/// A result of the repair's draft with a live Undo: the draft that Discard or Reload let go.
/// One per draft: a Discard after a Reload replaces it (`Results::record`). It ends at an edit
/// or a Save (`owner_changed`), at a newer file on disk (`disk_changed`), and at its Undo.
enum RepairUndo {
    Discard { owner: RepairOwner, draft: String },
    Reload { owner: RepairOwner, draft: String },
}

impl RepairUndo {
    fn draft(&self) -> &str {
        match self {
            Self::Discard { draft, .. } | Self::Reload { draft, .. } => draft,
        }
    }
}

impl design::results::Kept for RepairUndo {
    type Owner = RepairOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Ends,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> RepairOwner {
        match self {
            Self::Discard { owner, .. } | Self::Reload { owner, .. } => owner.clone(),
        }
    }
}

/// Why the repair's Save would not write, or did not: the footer's one reason. Its kind gives
/// its tone: a draft Save's check refuses is a note that cannot be read; a write the disk refused
/// is not saved.
enum RepairFault {
    /// Save's own check: a path rule, a fault in the text, an identity the campaign refuses.
    Refused(String),
    /// The disk refused the last Save of this draft.
    NotSaved(String),
}

impl Fault for RepairFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Refused(_) => Severity::CannotRead,
            Self::NotSaved(_) => Severity::NotSaved,
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "Save’s check and the disk give the parser’s and the system’s words"
    )]
    fn cause(&self) -> Sentence {
        match self {
            Self::Refused(words) => Sentence::outside(words.as_str())
                .unwrap_or(crate::sentence!("This note cannot be read.")),
            Self::NotSaved(words) => Sentence::outside(words.as_str())
                .unwrap_or(crate::sentence!("The note was not saved.")),
        }
    }
}

/// What the DM can press in the repair's footer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepairAct {
    Undo,
}

pub struct Repair {
    pub path: String,
    campaign: Uuid,
    /// The text on disk, with its revision (layout PR 9b-C): set when the repair loads, saves or
    /// follows the file, so `dirty` compares two revisions on a quiet frame.
    baseline: Revised<String>,
    draft: TextDraft,
    /// The last answer of `dirty`'s compare, by the draft's and the baseline's revisions.
    same: KeptCompare,
    /// The Undo of the last Discard or Reload of this draft.
    undos: design::Results<RepairOwner, RepairUndo>,
    disk: Option<String>,
    /// Save's failure on the disk (read only, missing, a write error): the draft it tried to
    /// write and Save's reason. This draft owns it. Each end event clears it: an edit, Discard
    /// or its Undo, Reload or its Undo, the file followed from disk, and a successful Save.
    failed: Option<(DraftKey, String)>,
}

impl Repair {
    /// The note's text in the source editor. A file the system will not read keeps its fault
    /// whole, so the page can name the file, the recovery and the system's words apart.
    pub fn open(campaign: &Campaign, path: &str) -> Result<Self, OpenError> {
        let file = crate::library::note_path(&campaign.path, path).map_err(OpenError::Words)?;
        let text = std::fs::read_to_string(file)
            .map_err(|error| OpenError::Unread(FileFault::read(path, &error)))?;
        Ok(Self {
            path: path.into(),
            campaign: campaign.id,
            baseline: Revised::new(text.clone()),
            draft: TextDraft::new(text),
            same: KeptCompare::default(),
            undos: design::Results::new(),
            disk: None,
            failed: None,
        })
    }

    fn owner(&self) -> RepairOwner {
        RepairOwner {
            campaign: self.campaign,
            path: self.path.clone(),
        }
    }

    pub fn entry_id(&self) -> Option<uuid::Uuid> {
        splat::content::Entry::parse(&self.path, &self.draft)
            .ok()
            .map(|e| e.id)
    }

    pub fn dirty(&self) -> bool {
        !self
            .draft
            .same_as(self.baseline.revision(), &self.baseline, &self.same)
    }

    /// Save's last failure on the disk, while the draft is still the one it tried to write.
    fn failure(&self) -> Option<&str> {
        self.failed
            .as_ref()
            .filter(|(tried, _)| *tried == self.draft.key())
            .map(|(_, reason)| reason.as_str())
    }

    /// The file on disk, read again: a draft with no changes follows it, and a changed draft
    /// keeps it for the DM's choice. A failure on the disk ends with any change of the draft.
    pub fn follow_disk(&mut self, campaign: &Campaign) {
        if let Ok(path) = crate::library::note_path(&campaign.path, &self.path)
            && let Ok(text) = std::fs::read_to_string(path)
            && text != *self.baseline
        {
            if self.dirty() {
                self.disk = Some(text);
            } else {
                self.baseline = Revised::new(text.clone());
                let _ = self.draft.replace(text);
                // A newer file ends the Undo of Discard and of Reload, so Undo never hides that
                // change.
                self.undos.disk_changed(|_| false);
                self.failed = None;
            }
        }
        // Save's failure on the disk belongs to the draft it tried to write. Any change ends it,
        // so it never comes back with the same words later.
        if self
            .failed
            .as_ref()
            .is_some_and(|(tried, _)| *tried != self.draft.key())
        {
            self.failed = None;
        }
    }

    /// The pane's footer (round 26 FA1): a result with a live Undo (Undo discard or Undo
    /// reload), then Save's reason. The comparison of a changed file owns the pane alone.
    fn footer_lines<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, RepairAct>,
        problem: Option<&str>,
    ) {
        if self.disk.is_none() {
            let name = self
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&self.path)
                .to_owned();
            if let Some(shown) = self.undos.get(&self.owner()) {
                messages.kept(shown.outcome(frame, |undo| {
                    match undo {
                        RepairUndo::Discard { .. } => KeptOutcome::Notice(
                            Notice::new(crate::sentence!("{} draft discarded.", name)),
                            // `Undo discard of Broken.md`: the name every discarded draft's Undo
                            // has in the app.
                            UndoStep::of(Verb::Discard, name.clone()).pressed_as(RepairAct::Undo),
                        ),
                        RepairUndo::Reload { .. } => KeptOutcome::Notice(
                            Notice::new(crate::sentence!(
                                "Reloaded from disk. Your draft is kept until you edit."
                            )),
                            // `Undo reload Broken.md`, the name its discard's Undo has (M0548).
                            UndoStep::of(Verb::Reload, name.clone()).pressed_as(RepairAct::Undo),
                        ),
                    }
                }));
            }
            let fault = match (problem, self.failure()) {
                (Some(problem), _) => Some(RepairFault::Refused(problem.to_owned())),
                (None, Some(failure)) => Some(RepairFault::NotSaved(failure.to_owned())),
                (None, None) => None,
            };
            if let Some(fault) = &fault {
                messages.outcome(FaultMessage::new(fault));
            }
        }
    }

    /// The DM's choice after the file changed on disk under a changed draft. True when an
    /// Overwrite saved. `refused`: Save's own check refuses the draft.
    fn choose(
        &mut self,
        campaign: &Campaign,
        choice: design::DiskConflictChoice,
        disk: String,
        refused: bool,
    ) -> bool {
        self.baseline = Revised::new(disk);
        self.disk = None;
        match choice {
            design::DiskConflictChoice::Reload => {
                let draft = self.draft.replace(self.baseline.as_str());
                // Reload replaces Discard's Undo: one Undo per draft.
                self.undos.record(RepairUndo::Reload {
                    owner: self.owner(),
                    draft,
                });
                // Reload ends the failure even when the file holds the draft's words.
                self.failed = None;
                false
            }
            // An unfinished repair stays editable; only valid text can be written. The disk
            // did not fail, so the editor shows only the cause, from the same check as the
            // state word: `Not saved.` is for a disk failure.
            design::DiskConflictChoice::Overwrite if refused => {
                self.undos.owner_changed(&self.owner());
                false
            }
            design::DiskConflictChoice::Overwrite => self.save(campaign),
        }
    }

    fn save(&mut self, campaign: &Campaign) -> bool {
        let result = match refusal(campaign, &self.path, &self.draft) {
            (Some(reason), _) => Err(reason),
            (None, _) => write_note(
                &campaign.path,
                &self.path,
                Some(&self.baseline),
                &self.draft,
            ),
        };
        match result {
            Ok(_) => {
                self.baseline = Revised::new(self.draft.as_str().to_owned());
                // A saved draft ends its Undo results.
                self.undos.owner_changed(&self.owner());
                self.failed = None;
                true
            }
            Err(error) => {
                self.failed = Some((self.draft.key(), error));
                false
            }
        }
    }
}

/// The repair as data, for a view that is not egui (the GPUI port).
pub struct RepairView {
    /// The file's name, the pane's title.
    pub file: String,
    pub path: String,
    pub text: String,
    /// The file on disk changed under a changed draft: its text, for the DM's choice.
    pub disk: Option<String>,
    /// Why Save would not write, and the line to fix.
    pub problem: Option<String>,
    pub line: Option<usize>,
    pub dirty: bool,
    /// Save failed on the disk for this draft.
    pub failed: bool,
}

impl Repair {
    /// The repair as it stands, after following its file (`follow_disk`).
    pub fn view(&mut self, campaign: &Campaign) -> RepairView {
        self.follow_disk(campaign);
        let (problem, line) = refusal(campaign, &self.path, &self.draft);
        RepairView {
            file: std::path::Path::new(&self.path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            path: self.path.clone(),
            text: self.draft.as_str().to_owned(),
            disk: self.disk.clone(),
            problem,
            line,
            dirty: self.dirty(),
            failed: self.failure().is_some(),
        }
    }

    /// The footer's lines as data.
    pub fn footer_plain(
        &self,
        campaign: &Campaign,
    ) -> Vec<design::outcome::plain::PlainLine<RepairAct>> {
        let (problem, _) = refusal(campaign, &self.path, &self.draft);
        let frame = design::outcome::PaneFrame::plain();
        let mut messages = Messages::new(&frame, "plain", 0.0);
        self.footer_lines(&frame, &mut messages, problem.as_deref());
        messages.into_plain()
    }

    /// The DM's words in the editor: an edit ends both Undo results.
    pub fn set_text(&mut self, words: &str) {
        if self.draft.as_str() != words {
            let _ = self.draft.replace(words);
            self.undos.owner_changed(&self.owner());
            self.failed = None;
        }
    }

    /// `Save repaired note`. True when it saved.
    pub fn press_save(&mut self, campaign: &Campaign) -> bool {
        let (problem, _) = refusal(campaign, &self.path, &self.draft);
        problem.is_none() && self.dirty() && self.save(campaign)
    }

    /// `Discard`: the draft goes back to the file, with its Undo.
    pub fn press_discard(&mut self) {
        let draft = self.draft.replace(self.baseline.as_str());
        self.undos.record(RepairUndo::Discard {
            owner: self.owner(),
            draft,
        });
        self.failed = None;
    }

    /// The choice after the file changed under the draft. True when an Overwrite saved.
    pub fn press_choice(
        &mut self,
        campaign: &Campaign,
        choice: design::DiskConflictChoice,
    ) -> bool {
        let Some(disk) = self.disk.clone() else {
            return false;
        };
        let (problem, _) = refusal(campaign, &self.path, &self.draft);
        self.choose(campaign, choice, disk, problem.is_some())
    }

    /// A footer press: the Undo of Discard or Reload, by its ticket.
    pub fn press_footer(&mut self, press: design::outcome::Press<RepairAct>) {
        let Some(ticket) = press.ticket else {
            return;
        };
        if let Ok((undo, ())) = self.undos.undo_by(ticket, |_| Ok::<_, ()>(())) {
            let _ = self.draft.replace(undo.draft());
            self.failed = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::memo::Changed;
    use crate::design::save_row::{DraftState, Primary};

    fn campaign(root: &std::path::Path) -> Campaign {
        Campaign {
            id: uuid::Uuid::new_v4(),
            name: "Repair checks".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        }
    }

    /// The repair as its pane shows it (`Repair::view` and its footer), in the one rule of a
    /// save row (`DraftState`): whether Save is primary, and the state word and the footer's
    /// words, one per line.
    fn painted(repair: &mut Repair, campaign: &Campaign, run_open: bool) -> (bool, String) {
        let view = repair.view(campaign);
        let mut state = DraftState::new(
            design::save_row::DraftOrigin::Saved,
            Changed::from(view.dirty),
            if run_open {
                Primary::Elsewhere
            } else {
                Primary::Draft
            },
        )
        .locks_campaign();
        if let Some(problem) = &view.problem {
            state = state.check(design::save_row::Diagnosis::refused(problem.as_str()));
        }
        if view.failed {
            state = state.not_saved();
        }
        let mut words = format!("{}\n", state.word().words());
        if view.disk.is_none() {
            for line in repair.footer_plain(campaign) {
                words.push_str(&line.words);
                words.push('\n');
            }
        }
        (
            state.save_weight() == design::save_row::SaveWeight::Primary,
            words,
        )
    }

    fn show_with(repair: &mut Repair, campaign: &Campaign, run_open: bool) -> bool {
        painted(repair, campaign, run_open).0
    }

    fn show(repair: &mut Repair, campaign: &Campaign) {
        let _ = repair.view(campaign);
    }

    #[test]
    fn iter5_d_repair_keeps_bad_input_and_does_not_write_on_read() {
        let temp = tempfile::tempdir().unwrap();
        let campaign = campaign(temp.path());
        let path = temp.path().join("Broken.md");
        let header = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: encounter\nsplat_name: Broken\n---\n",
            uuid::Uuid::new_v4()
        );
        for row in [
            "| Enemies |",                                  // Short row.
            "| Enemies | Goblin | | | | extra |",           // Extra cell.
            "| Enemies | | | | |",                          // Empty value.
            "| Enemies | [[Missing]] | | not a change | |", // Unreadable value.
        ] {
            let text = format!(
                "{header}\n| Side | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n{row}\n"
            );
            std::fs::write(&path, &text).unwrap();
            let mut repair = Repair::open(&campaign, "Broken.md").unwrap();
            repair.draft.push_str("\nMy unsent text.\n");
            let draft = repair.draft.as_str().to_owned();
            show(&mut repair, &campaign);
            assert_eq!(repair.draft.as_str(), draft, "{row}");
            assert_eq!(std::fs::read_to_string(&path).unwrap(), text, "{row}");
        }
        for value in ["[", "", "not-an-id"] {
            let text = format!("---\nsplat_schema: 2\nsplat_id: {value}\n---\n");
            std::fs::write(&path, &text).unwrap();
            let mut repair = Repair::open(&campaign, "Broken.md").unwrap();
            repair.draft.push_str("\nMy unsent text.\n");
            let draft = repair.draft.as_str().to_owned();
            assert!(!repair.save(&campaign), "{value}");
            assert_eq!(repair.draft.as_str(), draft);
            assert_eq!(std::fs::read_to_string(&path).unwrap(), text);
        }
        std::fs::write(&path, [0xff]).unwrap();
        match Repair::open(&campaign, "Broken.md") {
            Err(OpenError::Unread(fault)) => {
                assert_eq!(fault.kind, super::super::file_fault::Kind::NotText);
                assert_eq!(fault.path, "Broken.md");
            }
            other => panic!("AD2-open-kind: {:?}", other.err()),
        }
        assert_eq!(std::fs::read(&path).unwrap(), [0xff], "AD1-bytes");
    }

    #[test]
    fn iter5_d_repair_keeps_draft_when_source_disappears_and_recovers() {
        let temp = tempfile::tempdir().unwrap();
        let campaign = campaign(temp.path());
        let path = temp.path().join("Broken.md");
        let before = "---\nsplat_schema: 2\nsplat_id: [\n---\n";
        std::fs::write(&path, before).unwrap();
        let mut repair = Repair::open(&campaign, "Broken.md").unwrap();
        let draft = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Kept\n---\n\nMy unsent text.\n",
            uuid::Uuid::new_v4()
        );
        // Round 22 FD3: one weight for a save row. Save is primary while the draft has changes
        // it can save, and secondary when unchanged, still invalid, or when a run owns Next turn.
        // Round 23 FF2: the state word. A changed draft that still cannot be read is Invalid,
        // as in Library; Unsaved means Save would write.
        // Round 24 FD2, AC3, SC2: an unchanged draft that Save refuses is Invalid too, with
        // its cause above; `No changes` is only for a readable unchanged draft (below).
        let (primary, words) = painted(&mut repair, &campaign, false);
        assert!(!primary, "FD3-clean");
        assert!(
            words.contains("Invalid")
                && !words.contains("No changes")
                && words.contains("Line 3 opens a list with [ that never closes."),
            "iter-24-FD2-unchanged-invalid\n{words}"
        );
        let _ = repair.draft.replace(format!("{before}\nStill broken.\n"));
        let (primary, words) = painted(&mut repair, &campaign, false);
        assert!(!primary, "FD3-invalid");
        assert!(
            words.contains("Invalid") && !words.contains("Unsaved"),
            "FF2-invalid\n{words}"
        );
        let _ = repair.draft.replace(draft.clone());
        let (primary, words) = painted(&mut repair, &campaign, false);
        assert!(primary, "FD3-changed");
        assert!(
            words.contains("Unsaved") && !words.contains("Invalid"),
            "FF2-changed\n{words}"
        );
        // Round 23 review: a draft that parses, but that Save refuses (another campaign's
        // identity), is Invalid with Save's reason, before and after a Save attempt. Save is
        // not primary and writes nothing.
        let other = splat::content::formats::fixture(
            "Fixture.md",
            &format!(
                "splat_id: {}\nsplat_kind: campaign\nsplat_name: Other\nsplat_namespace: other\n",
                uuid::Uuid::new_v4()
            ),
            "",
        )
        .unwrap();
        assert!(
            splat::content::Entry::parse("Broken.md", &other).is_ok(),
            "fixture parses"
        );
        let _ = repair.draft.replace(other.clone());
        for attempt in ["before", "after"] {
            let (primary, words) = painted(&mut repair, &campaign, false);
            assert!(!primary, "review-refused-primary-{attempt}");
            assert!(
                words.contains("Invalid")
                    && words.contains(OWNERSHIP)
                    && !words.contains("Unsaved"),
                "review-refused-state-{attempt}\n{words}"
            );
            assert!(!repair.save(&campaign), "review-refused-save-{attempt}");
            assert_eq!(
                std::fs::read_to_string(&path).unwrap(),
                before,
                "review-refused-bytes-{attempt}"
            );
        }
        assert_eq!(repair.draft.as_str(), other);
        let _ = repair.draft.replace(draft.clone());
        assert!(!show_with(&mut repair, &campaign, true), "FD3-run-open");
        assert_eq!(repair.draft.as_str(), draft);
        // Round 23 re-check: before Save the state comes from the draft's own checks, with no
        // disk read. A Save that fails on the disk shows Save's own reason and `Not saved` in
        // place of `Unsaved` until the draft changes. The expected reason is the one the write
        // gives, which refuses before it writes anything.
        let failed_save = |repair: &mut Repair, case: &str| {
            let expected =
                crate::library::write_note(&campaign.path, "Broken.md", Some(before), &draft)
                    .unwrap_err();
            let (_, words) = painted(repair, &campaign, false);
            assert!(
                words.contains("Unsaved") && !words.contains("Not saved"),
                "recheck-{case}-before\n{words}"
            );
            assert!(!repair.save(&campaign), "recheck-{case}-save");
            let (primary, words) = painted(repair, &campaign, false);
            assert!(!primary, "recheck-{case}-primary");
            assert!(
                words.contains("Not saved")
                    && words.contains(&expected)
                    && !words.contains("Unsaved"),
                "recheck-{case}-after: {expected}\n{words}"
            );
            // Another frame keeps it: only the draft or a successful Save ends it.
            let (_, words) = painted(repair, &campaign, false);
            assert!(words.contains("Not saved"), "recheck-{case}-kept\n{words}");
            expected
        };
        for moved in [true, false] {
            if moved {
                std::fs::rename(&path, temp.path().join("Moved.md")).unwrap();
            } else {
                std::fs::remove_file(&path).unwrap();
            }
            show(&mut repair, &campaign);
            assert_eq!(repair.draft.as_str(), draft);
            failed_save(&mut repair, if moved { "moved" } else { "missing" });
            assert!(!path.exists());
            assert_eq!(repair.draft.as_str(), draft);
            std::fs::write(&path, before).unwrap();
            // The file coming back is not an edit of the draft: the cause stays until one.
            let (_, words) = painted(&mut repair, &campaign, false);
            assert!(
                words.contains("Not saved"),
                "recheck-{moved}-file-back\n{words}"
            );
            repair.draft.push_str(" ");
            show(&mut repair, &campaign);
            let _ = repair.draft.replace(draft.clone());
        }
        // A read-only file: Save refuses, the bytes stay, and the next edit ends the cause.
        let mut permissions = std::fs::metadata(&path).unwrap().permissions();
        permissions.set_readonly(true);
        std::fs::set_permissions(&path, permissions.clone()).unwrap();
        let reason = failed_save(&mut repair, "read-only");
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            before,
            "recheck-read-only-bytes"
        );
        repair.draft.push_str("One more word.\n");
        let (primary, words) = painted(&mut repair, &campaign, false);
        assert!(
            words.contains("Unsaved") && !words.contains("Not saved") && !words.contains(&reason),
            "recheck-read-only-edit-ends\n{words}"
        );
        // Round 24 contract: one key after the disk failure gives Save the primary back.
        assert!(primary, "iter-24-D-read-only-key-primary");
        // Going back to the same words does not bring the old cause back.
        let _ = repair.draft.replace(draft.clone());
        let (_, words) = painted(&mut repair, &campaign, false);
        assert!(
            !words.contains("Not saved"),
            "recheck-read-only-stays-ended\n{words}"
        );
        #[allow(clippy::permissions_set_readonly_false)]
        permissions.set_readonly(false);
        std::fs::set_permissions(&path, permissions.clone()).unwrap();
        // Review repair 4: after a failed Save, the file outside Splat comes to hold the draft's
        // exact words, and the DM chooses Reload. Reload ends the old disk failure although the
        // text is equal: the state says No changes and the old reason is gone.
        permissions.set_readonly(true);
        std::fs::set_permissions(&path, permissions.clone()).unwrap();
        let reason = failed_save(&mut repair, "reload");
        #[allow(clippy::permissions_set_readonly_false)]
        permissions.set_readonly(false);
        std::fs::set_permissions(&path, permissions).unwrap();
        std::fs::write(&path, &draft).unwrap();
        show(&mut repair, &campaign);
        let disk = repair
            .disk
            .clone()
            .expect("the outside write shows the disk choices");
        assert!(!repair.choose(&campaign, design::DiskConflictChoice::Reload, disk, false));
        assert_eq!(repair.draft.as_str(), draft);
        let (_, words) = painted(&mut repair, &campaign, false);
        assert!(
            words.contains("No changes")
                && !words.contains("Not saved")
                && !words.contains(&reason),
            "review-4-reload-equal-text\n{words}"
        );
        // A path outside the campaign folder is refused by the draft's own checks, with no disk
        // read: Invalid, Save's reason, and nothing written.
        let mut outside = Repair {
            path: "../Outside.md".into(),
            campaign: campaign.id,
            baseline: Revised::new(before.to_owned()),
            draft: draft.clone().into(),
            same: KeptCompare::default(),
            undos: design::Results::new(),
            disk: None,
            failed: None,
        };
        let (primary, words) = painted(&mut outside, &campaign, false);
        assert!(
            !primary
                && words.contains("Invalid")
                && words.contains("Use a relative note path within the package."),
            "recheck-path\n{words}"
        );
        assert!(!outside.save(&campaign), "recheck-path-save");
        assert!(
            !temp.path().parent().unwrap().join("Outside.md").exists(),
            "recheck-path-bytes"
        );
        assert!(repair.save(&campaign));
        assert!(!repair.dirty());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), draft);
        // The lookalike: a readable unchanged draft says `No changes`, not Invalid.
        let (primary, words) = painted(&mut repair, &campaign, false);
        assert!(
            !primary && words.contains("No changes") && !words.contains("Invalid"),
            "iter-24-FD2-unchanged-readable\n{words}"
        );
    }
}
