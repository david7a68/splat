//! The same source editor for broken notes in Campaign and World.
use super::file_fault::FileFault;
use crate::{
    design::{self, colors},
    library::write_note,
    persistence::Campaign,
};
use eframe::egui;

/// The source editor. True when the DM changed the text or moved focus into it this frame.
pub(crate) fn editor(
    ui: &mut egui::Ui,
    path: &str,
    draft: &mut String,
    error_line: Option<usize>,
) -> bool {
    egui::ScrollArea::vertical()
        .max_height(design::XXL * 6.0 + design::L + design::S + design::XS)
        .id_salt(("repair", path))
        .show(ui, |ui| {
            let output = egui::TextEdit::multiline(draft)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .show(ui);
            design::named(ui, &output.response, "Note text");
            let active = output.response.changed() || output.response.gained_focus();
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
            active
        })
        .inner
}

/// Why Save would not write this draft: a path outside its folder, a fault in its text and the
/// line to fix, or an identity that the campaign does not allow. It writes nothing and reads no
/// file: a refusal from the disk (read only, missing, a write error) is Save's own failure. Save, the save row's state, its cause
/// and Save's availability all use it, so `Unsaved` always means that Save would write.
pub(super) fn refusal(
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

pub(super) struct Repair {
    pub path: String,
    baseline: String,
    draft: String,
    discarded: Option<String>,
    reloaded: Option<String>,
    error: Option<String>,
    disk: Option<String>,
    /// Save's failure on the disk (read only, missing, a write error): the draft it tried to
    /// write and Save's reason. This draft owns it. It shows until the draft changes or a Save
    /// succeeds.
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
            baseline: text.clone(),
            draft: text,
            discarded: None,
            reloaded: None,
            error: None,
            disk: None,
            failed: None,
        })
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
                self.discarded = None;
                // A newer file ends the reload's Undo, so Undo never hides that change.
                self.reloaded = None;
                self.error = None;
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
            let choice = design::disk_conflict_choices(ui, "note");
            design::grouped_draft_comparison(ui, &self.draft, &disk, &self.path, true);
            if let Some(choice) = choice {
                self.baseline = disk;
                self.disk = None;
                self.error = None;
                self.discarded = None;
                match choice {
                    design::DiskConflictChoice::Reload => {
                        self.reloaded =
                            Some(std::mem::replace(&mut self.draft, self.baseline.clone()));
                    }
                    design::DiskConflictChoice::Overwrite => {
                        // An unfinished repair stays editable; only valid text can be written.
                        if problem.is_some() {
                            self.error =
                                Some(format!("Not saved. {}", problem.as_deref().unwrap()));
                            return false;
                        }
                        return self.save(campaign);
                    }
                }
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

    /// The editor, its messages and its save row.
    fn edit(
        &mut self,
        ui: &mut egui::Ui,
        campaign: &Campaign,
        run_open: bool,
        problem: Option<String>,
        line: Option<usize>,
    ) -> bool {
        if self.reloaded.is_some() && design::reloaded_draft(ui) {
            self.draft = self.reloaded.take().unwrap();
        }
        if let Some(error) = self
            .error
            .as_ref()
            .or(self.failure().map(str::to_owned).as_ref())
        {
            design::message(ui, design::Tone::Error, error);
        } else if let Some(problem) = &problem {
            design::message(ui, design::Tone::Error, problem);
        }
        let before = self.draft.clone();
        editor(ui, &self.path, &mut self.draft, line);
        if self.draft != before {
            self.reloaded = None;
            self.error = None;
        }
        let mut saved = false;
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let reason = if self.draft == self.baseline {
                    Some("No changes to save.")
                } else {
                    problem.as_deref()
                };
                // One weight for a save row: primary while the draft has changes Save can
                // write, unless an open run owns `Next turn`. After Save failed on the disk it
                // stays available to try again, but is not primary.
                let failed = self.failure().is_some();
                if (if reason.is_none() && !run_open && !failed {
                    design::Action::primary("Save repaired note")
                } else {
                    design::Action::secondary("Save repaired note")
                })
                .blocked(reason)
                .show(ui)
                .clicked()
                    || (reason.is_none()
                        && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)))
                {
                    saved = self.save(campaign);
                }
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let dirty = self.dirty();
                    // A changed draft that still cannot be read is Invalid, as in Library; the
                    // message above names the cause. Unsaved means Save would write.
                    let state = if dirty && problem.is_some() {
                        design::Status {
                            icon: design::icons::ERROR,
                            word: "Invalid",
                            color: colors().document.invalid,
                        }
                    } else if dirty && failed {
                        // Save failed on the disk; the message above gives Save's reason.
                        design::Status {
                            icon: design::icons::ERROR,
                            word: "Not saved",
                            color: colors().document.invalid,
                        }
                    } else if dirty {
                        design::Status {
                            icon: design::icons::DRAFT,
                            word: "Unsaved",
                            color: colors().document.unsaved,
                        }
                    } else {
                        design::Status {
                            icon: design::icons::OK,
                            word: "No changes",
                            color: colors().muted,
                        }
                    };
                    if design::save_row_state(ui, state, dirty, None) {
                        self.discarded = Some(self.draft.clone());
                        self.draft = self.baseline.clone();
                        // The restored text has its own problem, if any; drop the old message.
                        self.error = None;
                    }
                    if self.dirty() {
                        design::blocked_reason(ui, Some(crate::encounters::draft_switch_reason()));
                    }
                });
            });
        });
        if let Some(discarded) = &self.discarded
            && design::draft_discarded(ui, self.path.rsplit('/').next().unwrap_or(&self.path))
        {
            self.draft = discarded.clone();
            self.discarded = None;
            self.error = None;
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
                self.reloaded = None;
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
        let (primary, words) = painted(&mut repair, &campaign, false);
        assert!(!primary, "FD3-clean");
        assert!(
            words.contains("No changes") && !words.contains("Invalid"),
            "FF2-clean\n{words}"
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
        let (_, words) = painted(&mut repair, &campaign, false);
        assert!(
            words.contains("Unsaved") && !words.contains("Not saved") && !words.contains(&reason),
            "recheck-read-only-edit-ends\n{words}"
        );
        // Going back to the same words does not bring the old cause back.
        repair.draft = draft.clone();
        let (_, words) = painted(&mut repair, &campaign, false);
        assert!(
            !words.contains("Not saved"),
            "recheck-read-only-stays-ended\n{words}"
        );
        #[allow(clippy::permissions_set_readonly_false)]
        permissions.set_readonly(false);
        std::fs::set_permissions(&path, permissions).unwrap();
        // A path outside the campaign folder is refused by the draft's own checks, with no disk
        // read: Invalid, Save's reason, and nothing written.
        let mut outside = Repair {
            path: "../Outside.md".into(),
            baseline: before.into(),
            draft: draft.clone(),
            discarded: None,
            reloaded: None,
            error: None,
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
    }
}
