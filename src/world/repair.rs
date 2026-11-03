//! The same source editor for broken notes in Campaign and World.
use crate::{
    design::{self, colors},
    library::write_note,
    persistence::Campaign,
};
use eframe::egui;

pub(crate) fn editor(ui: &mut egui::Ui, path: &str, draft: &mut String, error_line: Option<usize>) {
    egui::ScrollArea::vertical()
        .max_height(design::XXL * 4.0 + design::L + design::S)
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

pub(super) struct Repair {
    pub path: String,
    baseline: String,
    draft: String,
    discarded: Option<String>,
    error: Option<String>,
}

impl Repair {
    pub fn open(campaign: &Campaign, path: &str) -> Result<Self, String> {
        let text = std::fs::read_to_string(crate::library::note_path(&campaign.path, path)?)
            .map_err(|e| e.to_string())?;
        Ok(Self {
            path: path.into(),
            baseline: text.clone(),
            draft: text,
            discarded: None,
            error: None,
        })
    }

    pub fn dirty(&self) -> bool {
        self.draft != self.baseline
    }

    pub fn show(&mut self, ui: &mut egui::Ui, campaign: &Campaign) -> bool {
        let (problem, line) = crate::campaign_content::repair_problem(&self.path, &self.draft);
        design::reader::title(ui, &self.path);
        super::file_actions(ui, campaign, &self.path);
        if let Some(problem) = &problem {
            design::message(ui, design::Tone::Error, problem);
        }
        if let Some(error) = &self.error {
            design::message(ui, design::Tone::Error, error);
        }
        editor(ui, &self.path, &mut self.draft, line);
        let mut saved = false;
        ui.horizontal_wrapped(|ui| {
            if self.draft != self.baseline
                && design::Action::quiet("Discard repair draft")
                    .show(ui)
                    .clicked()
            {
                self.discarded = Some(self.draft.clone());
                self.draft = self.baseline.clone();
            }
            if let Some(discarded) = &self.discarded
                && design::Action::quiet("Undo discard").show(ui).clicked()
            {
                self.draft = discarded.clone();
                self.discarded = None;
            }
            let reason = if self.draft == self.baseline {
                Some("No changes yet")
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
                let result =
                    splat::content::Entry::parse(&self.path, &self.draft).and_then(|entry| {
                        if entry.kind == "package"
                            || (entry.kind == "campaign" && entry.id != campaign.id)
                        {
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
                        saved = true;
                    }
                    Err(error) => self.error = Some(error),
                }
            }
            design::blocked_reason(ui, reason);
        });
        saved
    }
}
