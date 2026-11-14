//! The same source editor for broken notes in Campaign and World.
use super::file_fault::FileFault;
use crate::design::memo::Changed;
use crate::{
    design::{
        self, colors,
        outcome::{
            Fault, FaultMessage, KeptOutcome, Messages, Notice, PaneFrame, Sentence, Severity,
            UndoStep, Verb,
        },
        pane::Body,
        save_row::{DraftState, Primary},
    },
    library::write_note,
    persistence::Campaign,
};
use eframe::egui;
use uuid::Uuid;

/// The repair editor's accessible name. It names the file, so a DM who tabs between two open
/// repairs hears which is which. Selectors that ask for `Note text` still find it.
pub(crate) fn editor_name(path: &str) -> String {
    format!("Note text: {path}")
}

/// The repair editor's field. A broken note has no identity that the app can read (its text
/// does not parse), so its path is its owner: two repairs open at once are two fields.
const REPAIR_TEXT: design::layout::FieldName = crate::field_id!("world_repair_text");

/// The source editor itself: the note's text in the editor's face, named for its file, with the
/// error line marked. Its two events (changed, gained focus) and its id.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: a path is a broken note's only identity (design::results::STAGED_ALLOWS)"
)]
fn source_editor(
    ui: &mut egui::Ui,
    path: &str,
    draft: &mut String,
    error_line: Option<usize>,
) -> ((bool, bool), egui::Id) {
    let field = REPAIR_TEXT.of(design::TextKey::from_data(path));
    let output = field.editor(ui, draft, |edit| {
        edit.font(egui::TextStyle::Monospace)
            .desired_width(f32::INFINITY)
    });
    design::named(ui, &output.response, &editor_name(path));
    let active = (output.response.changed(), output.response.gained_focus());
    if let Some(line) = error_line {
        let position = draft
            .lines()
            .take(line.saturating_sub(1))
            .map(|line| line.chars().count() + 1)
            .sum();
        let row = output
            .galley
            .pos_from_cursor(egui::text::CCursor::new(position));
        let y = output.galley_pos.y + row.top();
        let painter = ui.painter().with_clip_rect(output.text_clip_rect);
        painter.vline(
            output.response.rect.left() + design::XS,
            y..=y + row.height(),
            egui::Stroke::new(design::FOCUS_WIDTH, colors().result.error),
        );
    }
    (active, output.response.id)
}

/// The same source editor in a pane's scroll column (World's repair pane): it is as tall as its
/// text, and the pane scrolls it, so it has no scroll of its own (a scroll inside the pane's
/// scroll is refused). Its two events: the DM changed the text, and focus moved into it.
fn editor_in_pane(
    ui: &mut egui::Ui,
    path: &str,
    draft: &mut String,
    error_line: Option<usize>,
) -> (bool, bool) {
    source_editor(ui, path, draft, error_line).0
}

/// The source editor, with its two events apart: the DM changed the text, and focus moved into
/// it. A caller that keeps a result for one draft ends it on a change, not on focus. Campaign's
/// repair keeps it in a scroll of its own, up to six rows high.
pub(crate) fn editor_events(
    ui: &mut egui::Ui,
    path: &str,
    draft: &mut String,
    error_line: Option<usize>,
) -> (bool, bool) {
    const MAX_HEIGHT: f32 = design::XXL * 6.0 + design::L + design::S + design::XS;
    let editor = design::ScrollColumn::new(("repair", path))
        .max_height(MAX_HEIGHT)
        .fit_height()
        .show(ui, |ui| source_editor(ui, path, draft, error_line));
    let (active, id) = editor.inner;
    // The editor's own scroll area takes a scroll target set inside it, so a page that scrolls
    // around the editor never sees it. When the editor gains focus or a screen reader asks to
    // show it, scroll the page to the editor's viewport.
    let reveal = ui.input(|input| {
        input.events.iter().any(|event| {
            matches!(event, egui::Event::AccessKitActionRequest(request)
                if request.action == egui::accesskit::Action::ScrollIntoView
                    && request.target == egui::accesskit::NodeId::from(id.value()))
        })
    });
    if active.1 || reveal {
        // The editor's viewport: its text, up to the editor's maximum height.
        let viewport = egui::Rect::from_min_size(
            editor.inner_rect.min,
            egui::vec2(
                editor.inner_rect.width(),
                editor.content_size.y.min(MAX_HEIGHT),
            ),
        );
        // At once: an animation would be cut short by the editor's own scroll to its cursor.
        ui.scroll_to_rect_animation(viewport, None, egui::style::ScrollAnimation::none());
    }
    active
}

/// Why Save would not write this draft: a path outside its folder, a fault in its text and the
/// line to fix, or an identity that the campaign does not allow. It writes nothing and reads no
/// file: a refusal from the disk (read only, missing, a write error) is Save's own failure. Save, the save row's state, its cause
/// and Save's availability all use it, so `Unsaved` always means that Save would write.
pub(crate) fn refusal(
    campaign: &Campaign,
    path: &str,
    draft: &str,
) -> (Option<String>, Option<usize>) {
    // The save row asks on every frame; a parse of a long note takes about 1 ms.
    static REFUSALS: splat::memo::Memo<(Option<String>, Option<usize>)> =
        splat::memo::Memo::new(64);
    REFUSALS.get_or(
        &[campaign.id.as_bytes(), path.as_bytes(), draft.as_bytes()],
        || {
            if let Some(rule) = crate::library::note_path_rule(path) {
                return (Some(rule.into()), None);
            }
            let (problem, line) = crate::campaign_content::repair_problem(path, draft);
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
pub(super) const OWNERSHIP: &str = "Keep campaign ownership consistent with settings.";

/// Why a repair did not open.
#[derive(Debug)]
pub(super) enum OpenError {
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
        reason = "Save's check and the disk give the parser's and the system's words"
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
enum RepairAct {
    Undo,
}

pub(super) struct Repair {
    pub path: String,
    campaign: Uuid,
    baseline: String,
    draft: String,
    /// The Undo of the last Discard or Reload of this draft.
    undos: design::Results<RepairOwner, RepairUndo>,
    disk: Option<String>,
    /// Save's failure on the disk (read only, missing, a write error): the draft it tried to
    /// write and Save's reason. This draft owns it. Each end event clears it: an edit, Discard
    /// or its Undo, Reload or its Undo, the file followed from disk, and a successful Save.
    failed: Option<(String, String)>,
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
            baseline: text.clone(),
            draft: text,
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
        self.draft != self.baseline
    }

    /// Save's last failure on the disk, while the draft is still the one it tried to write.
    fn failure(&self) -> Option<&str> {
        self.failed
            .as_ref()
            .filter(|(tried, _)| *tried == self.draft)
            .map(|(_, reason)| reason.as_str())
    }

    /// `run_open`: an open run owns `Next turn`, the screen's primary action, so Save stays
    /// secondary.
    pub fn show(&mut self, ui: &mut egui::Ui, campaign: &Campaign, run_open: bool) -> bool {
        if let Ok(path) = crate::library::note_path(&campaign.path, &self.path)
            && let Ok(text) = std::fs::read_to_string(path)
            && text != self.baseline
        {
            if self.dirty() {
                self.disk = Some(text);
            } else {
                self.baseline = text.clone();
                self.draft = text;
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
            .is_some_and(|(tried, _)| *tried != self.draft)
        {
            self.failed = None;
        }
        // Save's own checks: the state word, its cause and Save's availability all come from them.
        let (problem, line) = refusal(campaign, &self.path, &self.draft);
        // The pane's footer (round 26 FA1): a result with a live Undo (Undo discard or Undo
        // reload), then Save's reason. The comparison of a changed file owns the pane alone.
        let frame = PaneFrame::new(ui);
        let mut messages = Messages::new(
            &frame,
            ("world_repair_footer", &self.path),
            ui.available_width().min(design::LIST_MEASURE),
        );
        if self.disk.is_none() {
            let name = self
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&self.path)
                .to_owned();
            if let Some(shown) = self.undos.get(&self.owner()) {
                messages.kept(shown.outcome(&frame, |undo| {
                    match undo {
                        RepairUndo::Discard { .. } => KeptOutcome::Notice(
                            Notice::new(crate::sentence!("{} draft discarded.", name)),
                            // `Undo discard of Broken.md`: the name every discarded draft's Undo
                            // has in the app.
                            UndoStep::of(Verb::Discard, format!("of {name}"))
                                .pressed_as(RepairAct::Undo),
                        ),
                        RepairUndo::Reload { .. } => KeptOutcome::Notice(
                            Notice::new(crate::sentence!(
                                "Reloaded from disk. Your draft is kept until you edit."
                            )),
                            UndoStep::verb(Verb::Reload).pressed_as(RepairAct::Undo),
                        ),
                    }
                }));
            }
            let fault = match (&problem, self.failure()) {
                (Some(problem), _) => Some(RepairFault::Refused(problem.clone())),
                (None, Some(failure)) => Some(RepairFault::NotSaved(failure.to_owned())),
                (None, None) => None,
            };
            if let Some(fault) = &fault {
                messages.outcome(FaultMessage::new(fault));
            }
        }
        let paned = messages.pane(ui, Body::Content, |ui| {
            self.body(ui, campaign, run_open, problem, line)
        });
        if let Some(press) = paned.pressed {
            match press.id {
                RepairAct::Undo => {
                    let Some(ticket) = press.ticket else {
                        return paned.body;
                    };
                    // A replaced or ended result (`Gone`) does nothing.
                    if let Ok((undo, ())) = self.undos.undo_by(ticket, |_| Ok::<_, ()>(())) {
                        self.draft = undo.draft().to_owned();
                        self.failed = None;
                        // The draft changed, so Save's check runs again on the words now in the
                        // editor.
                        ui.ctx().request_repaint();
                    }
                }
            }
        }
        paned.body
    }

    /// The pane's body: the title, the file line, then the comparison of a changed file, or the
    /// editor and its save row. True when Save or Overwrite saved.
    fn body(
        &mut self,
        ui: &mut egui::Ui,
        campaign: &Campaign,
        run_open: bool,
        problem: Option<String>,
        line: Option<usize>,
    ) -> bool {
        design::reader::title(
            ui,
            std::path::Path::new(&self.path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .as_ref(),
        );
        super::file_actions(ui, campaign, &self.path);
        if let Some(disk) = self.disk.clone() {
            let choice = design::disk_conflict_choices(ui, design::ConflictSubject::This("note"));
            design::grouped_draft_comparison(ui, &self.draft, &disk, &self.path, true);
            if let Some(choice) = choice {
                return self.choose(campaign, choice, disk, problem.is_some());
            }
            return false;
        }
        // The two-version comparison above keeps the pane's width. The ordinary editor and its
        // save row end together at `LIST_MEASURE`, so Save stays near the state and Discard.
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            self.edit(ui, campaign, run_open, problem, line)
        })
        .inner
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
        self.baseline = disk;
        self.disk = None;
        match choice {
            design::DiskConflictChoice::Reload => {
                let draft = std::mem::replace(&mut self.draft, self.baseline.clone());
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

    /// The editor and its save row; the footer follows the save row (round 27 FE2).
    fn edit(
        &mut self,
        ui: &mut egui::Ui,
        campaign: &Campaign,
        run_open: bool,
        problem: Option<String>,
        line: Option<usize>,
    ) -> bool {
        // Nothing above the editor comes and goes: its reason and its results are in the pane's
        // footer (round 26), so the editor never moves.
        let (changed, _) = editor_in_pane(ui, &self.path, &mut self.draft, line);
        if changed {
            // An edit ends both Undo results: Undo would replace the words just typed.
            self.undos.owner_changed(&self.owner());
            self.failed = None;
        }
        // One rule for the save row (`DraftState`): a draft Save refuses is Invalid, changed or
        // not; after Save failed on the disk it stays open to try again, but is not primary; an
        // open run owns the primary action.
        let mut state = DraftState::new(
            Changed::from(self.dirty()),
            if run_open {
                Primary::Elsewhere
            } else {
                Primary::Draft
            },
        )
        .locks_campaign();
        if let Some(problem) = &problem {
            state = state.refused(problem.as_str());
        }
        if self.failure().is_some() {
            state = state.not_saved();
        }
        let pressed = design::save_row(ui, &state, "Save repaired note");
        let save = pressed.save
            || (state.can_save()
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)));
        let mut saved = false;
        if save {
            saved = self.save(campaign);
        }
        if pressed.discard {
            let draft = std::mem::replace(&mut self.draft, self.baseline.clone());
            // Discard replaces Reload's Undo: one Undo per draft.
            self.undos.record(RepairUndo::Discard {
                owner: self.owner(),
                draft,
            });
            self.failed = None;
            // The footer drawn before this body showed the draft before Discard.
            ui.ctx().request_repaint();
        }
        saved
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
                self.baseline = self.draft.clone();
                // A saved draft ends its Undo results.
                self.undos.owner_changed(&self.owner());
                self.failed = None;
                true
            }
            Err(error) => {
                self.failed = Some((self.draft.clone(), error));
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn campaign(root: &std::path::Path) -> Campaign {
        Campaign {
            id: uuid::Uuid::new_v4(),
            name: "Repair checks".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        }
    }

    /// Draws the repair once. True when it painted an accent-filled (primary) button.
    fn show_with(repair: &mut Repair, campaign: &Campaign, run_open: bool) -> bool {
        painted(repair, campaign, run_open).0
    }

    /// Draws the repair once: whether a primary button showed, and every painted word.
    fn painted(repair: &mut Repair, campaign: &Campaign, run_open: bool) -> (bool, String) {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                assert!(!repair.show(ui, campaign, run_open));
            });
        });
        let accent = colors().accent_fill;
        let mut words = String::new();
        let mut primary = false;
        for clipped in &output.shapes {
            match &clipped.shape {
                egui::Shape::Rect(rect) if rect.fill == accent => primary = true,
                egui::Shape::Text(text) => {
                    words.push_str(&text.galley.job.text);
                    words.push('\n');
                }
                _ => {}
            }
        }
        (primary, words)
    }

    fn show(repair: &mut Repair, campaign: &Campaign) {
        show_with(repair, campaign, false);
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
            let draft = repair.draft.clone();
            show(&mut repair, &campaign);
            assert_eq!(repair.draft, draft, "{row}");
            assert_eq!(std::fs::read_to_string(&path).unwrap(), text, "{row}");
        }
        for value in ["[", "", "not-an-id"] {
            let text = format!("---\nsplat_schema: 2\nsplat_id: {value}\n---\n");
            std::fs::write(&path, &text).unwrap();
            let mut repair = Repair::open(&campaign, "Broken.md").unwrap();
            repair.draft.push_str("\nMy unsent text.\n");
            let draft = repair.draft.clone();
            assert!(!repair.save(&campaign), "{value}");
            assert_eq!(repair.draft, draft);
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
        repair.draft = format!("{before}\nStill broken.\n");
        let (primary, words) = painted(&mut repair, &campaign, false);
        assert!(!primary, "FD3-invalid");
        assert!(
            words.contains("Invalid") && !words.contains("Unsaved"),
            "FF2-invalid\n{words}"
        );
        repair.draft = draft.clone();
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
        repair.draft = other.clone();
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
        assert_eq!(repair.draft, other);
        repair.draft = draft.clone();
        assert!(!show_with(&mut repair, &campaign, true), "FD3-run-open");
        assert_eq!(repair.draft, draft);
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
            assert_eq!(repair.draft, draft);
            failed_save(&mut repair, if moved { "moved" } else { "missing" });
            assert!(!path.exists());
            assert_eq!(repair.draft, draft);
            std::fs::write(&path, before).unwrap();
            // The file coming back is not an edit of the draft: the cause stays until one.
            let (_, words) = painted(&mut repair, &campaign, false);
            assert!(
                words.contains("Not saved"),
                "recheck-{moved}-file-back\n{words}"
            );
            repair.draft.push(' ');
            show(&mut repair, &campaign);
            repair.draft = draft.clone();
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
        repair.draft = draft.clone();
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
        assert_eq!(repair.draft, draft);
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
            baseline: before.into(),
            draft: draft.clone(),
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
