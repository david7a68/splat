//! The same disk comparison for encounter, group and creature drafts.
use crate::{
    design::{self, Action, Tone},
    library::note_path,
};
use eframe::egui;
use std::{fs, path::Path};

#[derive(Clone, Default)]
pub(super) struct Recovery {
    pub change: Option<Change>,
    copies_accepted: bool,
}

#[derive(Clone)]
pub(super) struct Change {
    pub path: String,
    pub text: Option<String>,
    pub copies: Vec<String>,
}

impl Recovery {
    pub fn check(
        &mut self,
        root: &Path,
        path: &mut String,
        baseline: Option<&str>,
    ) -> Result<(), String> {
        let Some(baseline) = baseline else {
            return Ok(());
        };
        let id = crate::content_workspace::note_id(baseline);
        let (entries, errors) = splat::content::read_directory(root)?;
        let mut peers = Vec::new();
        for other in entries
            .into_iter()
            .map(|entry| entry.path)
            .chain(errors.into_iter().map(|error| error.path))
        {
            if other != *path
                && let Ok(text) = fs::read_to_string(note_path(root, &other)?)
                && id.is_some()
                && crate::content_workspace::note_id(&text) == id
            {
                peers.push((other, text));
            }
        }
        let mut current = read(root, path)?;
        if current.is_none() && peers.len() == 1 {
            let (moved, text) = peers.remove(0);
            *path = moved;
            current = Some(text);
        }
        if current.as_deref() != Some(baseline) || (!peers.is_empty() && !self.copies_accepted) {
            self.change = Some(Change {
                path: path.clone(),
                text: current,
                copies: peers.into_iter().map(|(path, _)| path).collect(),
            });
            return Err(
                "This entry changed on disk. Your draft is safe. Choose which version to keep."
                    .into(),
            );
        }
        self.change = None;
        Ok(())
    }

    pub fn accept(&mut self) -> Option<Change> {
        self.copies_accepted = true;
        self.change.take()
    }

    pub fn show(&self, ui: &mut egui::Ui, draft: &str) -> Option<bool> {
        let change = self.change.as_ref()?;
        design::message(
            ui,
            Tone::Warning,
            &format!(
                "{} changed on disk. Your draft is safe until you choose.",
                change.path
            ),
        );
        for path in &change.copies {
            design::message(
                ui,
                Tone::Warning,
                &format!("Another file uses this identity: {path}. Both files stay."),
            );
        }
        let mut choice = None;
        ui.horizontal_wrapped(|ui| {
            if Action::secondary("Overwrite with my draft")
                .show(ui)
                .clicked()
            {
                choice = Some(true);
            }
            if Action::destructive(if change.text.is_some() {
                "Reload and lose my draft"
            } else {
                "Let it go"
            })
            .show(ui)
            .clicked()
            {
                choice = Some(false);
            }
        });
        if let Some(text) = &change.text {
            design::draft_comparison_styled(
                ui,
                draft,
                text,
                (&change.path, text),
                design::Type::Body.font(ui),
                design::Type::Body.line(),
            );
        }
        choice
    }
}

fn read(root: &Path, path: &str) -> Result<Option<String>, String> {
    match fs::read_to_string(note_path(root, path)?) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("{path}: {error}")),
    }
}

impl super::Encounters {
    pub(super) fn encounter_disk_view(
        &mut self,
        ui: &mut egui::Ui,
        campaign: &crate::persistence::Campaign,
        modules: &[crate::persistence::Module],
    ) -> bool {
        let Some(draft) = self.draft.as_mut() else {
            return false;
        };
        if draft.recovery.change.is_none() {
            if let Some(group) = draft
                .groups
                .iter_mut()
                .find(|group| group.has_disk_change())
            {
                match group.disk_choice(ui, &campaign.path) {
                    Ok(Some(true)) => self.save_draft(ui.ctx(), campaign, modules),
                    Ok(Some(false)) => self.error = None,
                    Err(error) => self.error = Some(error),
                    _ => {}
                }
                return true;
            }
            return false;
        }
        let choice = draft.recovery.show(ui, &draft.text().unwrap_or_default());
        if let Some(keep) = choice {
            if keep {
                if let Some(change) = draft.recovery.accept() {
                    draft.path = change.path;
                    draft.baseline = change.text.clone();
                    if let Some(text) = change.text {
                        draft.source = text;
                    }
                    self.save_draft(ui.ctx(), campaign, modules);
                }
            } else {
                let path = draft.path.clone();
                let old = self.draft.take();
                self.draft = super::Draft::open(&campaign.path, &path).ok();
                self.discarded = old;
                self.error = None;
            }
        }
        true
    }
}
