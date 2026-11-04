//! Explicit file conversion. A preview owns its baselines; Undo owns only saved bytes.
use crate::{
    campaigns::Campaigns,
    design::{self, Action, Tone, Type},
    file_batch::{self, Change},
    library::{note_path, write_note},
};
use eframe::egui;
use splat::content::{Entry, formats};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Preview {
    root: PathBuf,
    path: String,
    before: String,
    after: String,
}

#[derive(Default)]
pub struct State {
    preview: Option<Vec<Preview>>,
    selected: usize,
    undo: Vec<Change>,
    notice_file: Option<PathBuf>,
    skipped: Vec<String>,
    error: Option<String>,
}

impl State {
    pub fn remember(&mut self, root: &Path, path: &str, before: &str, after: &str) {
        self.undo = vec![Change::text(root.join(path), Some(before), Some(after))];
        self.notice_file = Some(root.join(path));
    }

    pub fn scan(&mut self, campaigns: &Campaigns) {
        let catalogue = crate::content_workspace::Catalogue::load(
            campaigns.modules(),
            campaigns.active_campaign().as_ref(),
        )
        .ok();
        let mut roots: Vec<_> = campaigns
            .active_campaign()
            .map(|c| c.path.clone())
            .into_iter()
            .collect();
        roots.extend(campaigns.modules().iter().map(|m| m.path.clone()));
        self.skipped.clear();
        let mut preview = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for root in roots {
            let Ok((entries, errors)) = splat::content::read_directory(&root) else {
                continue;
            };
            self.skipped.extend(
                errors
                    .into_iter()
                    .map(|e| format!("{}: {}", e.path, e.message)),
            );
            for entry in entries {
                if !formats::eligible(&entry) || formats::schema(&entry.fields) != Ok(1) {
                    continue;
                }
                let result = (|| {
                    let path = note_path(&root, &entry.path)?;
                    if !seen.insert(path.clone()) {
                        return Ok(None);
                    }
                    if fs::metadata(&path)
                        .map_err(|e| e.to_string())?
                        .permissions()
                        .readonly()
                    {
                        return Err("Read only".into());
                    }
                    let before = fs::read_to_string(path).map_err(|e| e.to_string())?;
                    let after = formats::convert_resolved(
                        &entry.path,
                        &before,
                        catalogue
                            .iter()
                            .flat_map(|cat| cat.entries.values().map(|item| &item.entry)),
                    )?;
                    Ok::<_, String>(Some(Preview {
                        root: root.clone(),
                        path: entry.path.clone(),
                        before,
                        after,
                    }))
                })();
                match result {
                    Ok(Some(file)) => preview.push(file),
                    Ok(None) => {}
                    Err(error) => self.skipped.push(format!("{}: {error}", entry.path)),
                }
            }
        }
        preview.sort_by(|a, b| a.root.cmp(&b.root).then(a.path.cmp(&b.path)));
        self.selected = 0;
        self.preview = Some(preview);
    }

    fn apply(&mut self) {
        let Some(files) = self.preview.take() else {
            return;
        };
        self.undo.clear();
        self.notice_file = None;
        for file in files {
            match write_note(&file.root, &file.path, Some(&file.before), &file.after) {
                Ok(_) => self.undo.push(Change::text(
                    file.root.join(&file.path),
                    Some(&file.before),
                    Some(&file.after),
                )),
                Err(error) => self.skipped.push(format!("{}: {error}", file.path)),
            }
        }
    }

    pub fn undo(&mut self) -> Result<(), String> {
        let changes: Vec<_> = self
            .undo
            .iter()
            .map(|change| Change {
                path: change.path.clone(),
                before: change.after.clone(),
                after: change.before.clone(),
            })
            .collect();
        file_batch::apply(&changes)?;
        self.undo.clear();
        Ok(())
    }

    /// Returns true after disk writes, so the owner can reload its catalogue.
    pub fn show(&mut self, ui: &mut egui::Ui, blocked: Option<&str>, file: Option<&Path>) -> bool {
        let mut changed = false;
        if !self.undo.is_empty()
            && self
                .notice_file
                .as_deref()
                .is_none_or(|path| Some(path) == file)
            && design::message_with_action(ui, Tone::Success, "Saved in the new layout.", "Undo")
        {
            match self.undo() {
                Ok(()) => changed = true,
                Err(error) => self.error = Some(error),
            }
        }
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        if self.preview.is_some() {
            let mut apply = false;
            let mut close = false;
            design::dialog(ui.ctx(), "convert_files", "Convert files", |ui| {
                let files = self.preview.as_ref().unwrap();
                ui.label(format!("{} files can use the new layout.", files.len()));
                let mut kinds = std::collections::BTreeMap::<String, usize>::new();
                for file in files {
                    if let Ok(entry) = Entry::parse(&file.path, &file.before) {
                        let kind = if entry.fields.contains_key("splat_character") {
                            "character".into()
                        } else {
                            entry.kind
                        };
                        *kinds.entry(kind).or_default() += 1;
                    }
                }
                ui.label(
                    Type::Small.rich(
                        ui,
                        kinds
                            .iter()
                            .map(|(kind, n)| {
                                let label = if kind == "world" {
                                    "World note".into()
                                } else {
                                    format!("{} file", kind.replace('_', " "))
                                };
                                design::count(*n, &label, &format!("{label}s"))
                            })
                            .collect::<Vec<_>>()
                            .join(" · "),
                    ),
                );
                if let Some(file) = files.get(self.selected) {
                    let picker = egui::ComboBox::from_id_salt("convert_file")
                        .selected_text(&file.path)
                        .show_ui(ui, |ui| {
                            for (at, file) in files.iter().enumerate() {
                                ui.selectable_value(&mut self.selected, at, &file.path);
                            }
                        });
                    design::named(ui, &picker.response, "File to convert");
                    let file = &files[self.selected];
                    let height = (ui.available_height() - design::XXL).min(design::XXL * 10.0);
                    ui.allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
                        design::draft_comparison_labeled(
                            ui,
                            [&file.before, &file.after],
                            ["Before", "After"],
                            ("convert_file", &file.root, &file.path, &file.before),
                            Type::Mono.font(ui),
                            Type::Mono.line(),
                        );
                    });
                }
                if !self.skipped.is_empty() {
                    design::quiet_section(ui, "Files skipped", |ui| {
                        for file in &self.skipped {
                            ui.label(file);
                        }
                    });
                }
                ui.horizontal(|ui| {
                    apply = Action::primary("Convert files")
                        .blocked(
                            blocked.or(files.is_empty().then_some("No files need conversion.")),
                        )
                        .show(ui)
                        .clicked();
                    close = Action::quiet("Cancel").show(ui).clicked();
                });
            });
            if apply {
                self.apply();
                changed = true;
            }
            if close {
                self.preview = None;
            }
        } else if !self.skipped.is_empty() {
            design::quiet_section(ui, "Files skipped", |ui| {
                for file in &self.skipped {
                    ui.label(file);
                }
            });
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(unix)]
    fn fmt_f1_conversion_skips_read_only_files() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Rook.md");
        let before = "---\nsplat_schema: 1\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Rook\nsplat_world: {type: npc}\n---\n";
        fs::write(&path, before).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o444)).unwrap();
        let mut state = State {
            preview: Some(vec![Preview {
                root: dir.path().into(),
                path: "Rook.md".into(),
                before: before.into(),
                after: formats::convert("Rook.md", before).unwrap(),
            }]),
            ..Default::default()
        };
        state.apply();
        assert_eq!(state.skipped.len(), 1);
        assert!(state.undo.is_empty());
        assert_eq!(fs::read_to_string(path).unwrap(), before);
    }

    #[test]
    fn fmt_f1_conversion_skips_conflicts_and_undo_never_clobbers() {
        let dir = tempfile::tempdir().unwrap();
        let old = "---\nsplat_schema: 1\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Rook\nsplat_world: {type: npc}\n---\n";
        let after = formats::convert("Rook.md", old).unwrap();
        let mut state = State::default();
        for name in ["Rook.md", "Second.md", "Changed.md"] {
            fs::write(dir.path().join(name), old).unwrap();
        }
        state.preview = Some(
            ["Rook.md", "Second.md", "Changed.md"]
                .into_iter()
                .map(|name| Preview {
                    root: dir.path().into(),
                    path: name.into(),
                    before: old.into(),
                    after: after.clone(),
                })
                .collect(),
        );
        fs::write(dir.path().join("Changed.md"), "Outside edit").unwrap();
        state.apply();
        assert_eq!(state.skipped.len(), 1);
        assert_eq!(
            fs::read_to_string(dir.path().join("Changed.md")).unwrap(),
            "Outside edit"
        );
        fs::write(dir.path().join("Rook.md"), "Later edit").unwrap();
        assert!(state.undo().is_err());
        assert_eq!(
            fs::read_to_string(dir.path().join("Second.md")).unwrap(),
            after
        );
        assert_eq!(
            fs::read_to_string(dir.path().join("Rook.md")).unwrap(),
            "Later edit"
        );
        fs::write(dir.path().join("Rook.md"), after).unwrap();
        state.undo().unwrap();
        assert_eq!(fs::read_to_string(dir.path().join("Rook.md")).unwrap(), old);
        assert_eq!(
            fs::read_to_string(dir.path().join("Second.md")).unwrap(),
            old
        );
    }
}
