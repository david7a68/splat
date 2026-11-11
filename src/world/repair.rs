//! The same source editor for broken notes in Campaign and World.
use super::file_fault::FileFault;
use crate::{
    design::{self, colors},
    library::write_note,
    persistence::Campaign,
};
use eframe::egui;

pub(crate) fn editor(ui: &mut egui::Ui, path: &str, draft: &mut String, error_line: Option<usize>) {
    egui::ScrollArea::vertical()
        .max_height(design::XXL * 6.0 + design::L + design::S + design::XS)
        .id_salt(("repair", path))
        .show(ui, |ui| {
            let output = egui::TextEdit::multiline(draft)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .show(ui);
            design::named(ui, &output.response, "Note text");
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
        });
}

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

    pub fn show(&mut self, ui: &mut egui::Ui, campaign: &Campaign) -> bool {
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
        let (problem, line) = crate::campaign_content::repair_problem(&self.path, &self.draft);
        design::reader::title(
            ui,
            std::path::Path::new(&self.path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .as_ref(),
        );
        super::file_actions(ui, campaign, &self.path);
        if problem.as_deref().is_some_and(crate::formats_ui::is_old) {
            crate::formats_ui::old_notice(
                ui,
                &crate::formats_ui::old_message(&self.path),
                vec![campaign.path.join(&self.path)],
            );
            return false;
        }
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
        if self.reloaded.is_some() && design::reloaded_draft(ui) {
            self.draft = self.reloaded.take().unwrap();
        }
        if let Some(error) = &self.error {
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
                if design::Action::secondary("Save repaired note")
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
                    let state = if dirty {
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
        let result = splat::content::Entry::parse(&self.path, &self.draft).and_then(|entry| {
            if entry.kind == "package" || (entry.kind == "campaign" && entry.id != campaign.id) {
                return Err("Keep campaign ownership consistent with settings.".into());
            }
            write_note(
                &campaign.path,
                &self.path,
                Some(&self.baseline),
                &self.draft,
            )
        });
        match result {
            Ok(_) => {
                self.baseline = self.draft.clone();
                self.reloaded = None;
                true
            }
            Err(error) => {
                self.error = Some(error);
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

    fn show(repair: &mut Repair, campaign: &Campaign) {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                assert!(!repair.show(ui, campaign));
            });
        });
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
        repair.draft = draft.clone();
        for moved in [true, false] {
            if moved {
                std::fs::rename(&path, temp.path().join("Moved.md")).unwrap();
            } else {
                std::fs::remove_file(&path).unwrap();
            }
            show(&mut repair, &campaign);
            assert_eq!(repair.draft, draft);
            assert!(!repair.save(&campaign));
            assert!(!path.exists());
            assert_eq!(repair.draft, draft);
            std::fs::write(&path, before).unwrap();
        }
        assert!(repair.save(&campaign));
        assert!(!repair.dirty());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), draft);
    }
}
