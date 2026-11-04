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
    owner: usize,
    root: PathBuf,
    path: String,
    before: String,
    after: String,
}

const GENERATED_REASON: &str = "Generated package; its converter writes the layout";

struct OwnerPreview {
    root: PathBuf,
    name: String,
    checked: bool,
    package: bool,
    reason: Option<&'static str>,
}

impl OwnerPreview {
    fn new(root: PathBuf, name: String, campaign: bool) -> Self {
        let generated = !campaign && crate::content_workspace::generated_package(&root);
        Self {
            root,
            name,
            checked: campaign,
            package: !campaign,
            reason: generated.then_some(GENERATED_REASON),
        }
    }
}

#[derive(Default)]
pub struct State {
    preview: Option<Vec<Preview>>,
    owners: Vec<OwnerPreview>,
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
        self.owners = campaigns
            .active_campaign()
            .map(|c| OwnerPreview::new(c.path.clone(), c.name.clone(), true))
            .into_iter()
            .collect();
        let mut modules: Vec<_> = campaigns.modules().iter().collect();
        modules.sort_by_key(|m| m.name.to_lowercase());
        self.owners.extend(
            modules
                .into_iter()
                .map(|m| OwnerPreview::new(m.path.clone(), m.name.clone(), false)),
        );
        self.skipped.clear();
        let mut preview = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for (owner, group) in self.owners.iter().enumerate() {
            if group.reason.is_some() {
                continue;
            }
            let root = &group.root;
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
                        owner,
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
        preview.sort_by(|a, b| a.owner.cmp(&b.owner).then(a.path.cmp(&b.path)));
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
            if !self
                .owners
                .get(file.owner)
                .is_some_and(|owner| owner.checked && owner.reason.is_none())
                || self.owners.get(file.owner).is_some_and(|owner| {
                    owner.package && crate::content_workspace::generated_package(&file.root)
                })
            {
                continue;
            }
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
                egui::ScrollArea::vertical()
                    .id_salt("conversion_owners")
                    .max_height(design::XXL * 4.0)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        for owner in &mut self.owners {
                            ui.add_enabled_ui(owner.reason.is_none(), |ui| {
                                ui.checkbox(&mut owner.checked, &owner.name);
                                if let Some(reason) = owner.reason {
                                    ui.label(Type::Small.rich(ui, reason));
                                }
                            });
                        }
                    });
                let selected_files: Vec<_> = files
                    .iter()
                    .filter(|file| {
                        self.owners[file.owner].checked && self.owners[file.owner].reason.is_none()
                    })
                    .collect();
                ui.label(format!(
                    "{} files can use the new layout.",
                    selected_files.len()
                ));
                let mut kinds = std::collections::BTreeMap::<String, usize>::new();
                for file in &selected_files {
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
                            for (owner, group) in self.owners.iter().enumerate() {
                                if group.reason.is_some() {
                                    continue;
                                }
                                ui.label(Type::Small.rich(ui, &group.name));
                                for (at, file) in files
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, file)| file.owner == owner)
                                {
                                    ui.selectable_value(&mut self.selected, at, &file.path);
                                }
                            }
                        });
                    design::named(ui, &picker.response, "File to convert");
                    let file = &files[self.selected];
                    let height = (ui.ctx().content_rect().height() * 0.4).min(design::XXL * 8.0);
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
                            blocked.or(selected_files
                                .is_empty()
                                .then_some("Select an owner with files to convert.")),
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

    fn snapshot(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        let mut files = Vec::new();
        for item in fs::read_dir(root).unwrap() {
            let path = item.unwrap().path();
            if path.is_dir() {
                files.extend(snapshot(&path));
            } else {
                files.push((path.clone(), fs::read(path).unwrap()));
            }
        }
        files.sort();
        files
    }

    #[test]
    fn fmt_f2_conversion_defaults_to_campaign_and_never_writes_generated_or_unticked_packages() {
        let dir = tempfile::tempdir().unwrap();
        let world = crate::visual_tests::fixture::World::build(
            dir.path(),
            &dir.path().join("cache"),
            "conversion-owners",
            crate::visual_tests::fixture::Seed::Formats,
        )
        .unwrap();
        let app = world.app();
        let before: Vec<_> = app
            .campaigns
            .modules()
            .iter()
            .map(|m| snapshot(&m.path))
            .collect();
        let mut state = State::default();
        state.scan(&app.campaigns);
        assert!(state.owners[0].checked);
        assert!(state.owners[1..].iter().all(|owner| !owner.checked));
        let generated = state
            .owners
            .iter()
            .position(|owner| owner.name == "Generated Book")
            .unwrap();
        assert_eq!(state.owners[generated].reason, Some(GENERATED_REASON));
        assert!(
            state
                .preview
                .as_ref()
                .unwrap()
                .iter()
                .all(|file| file.owner != generated)
        );
        // Even a stale or forged selection cannot opt a generated package in.
        state.owners[generated].checked = true;
        state.apply();
        assert!(!state.undo.is_empty(), "campaign files converted");
        for (module, before) in app.campaigns.modules().iter().zip(before) {
            assert_eq!(snapshot(&module.path), before, "{}", module.name);
        }
    }

    #[test]
    fn fmt_f2_generated_marker_added_after_preview_still_blocks_conversion() {
        let dir = tempfile::tempdir().unwrap();
        let world = crate::visual_tests::fixture::World::build(
            dir.path(),
            &dir.path().join("cache"),
            "conversion-marker-race",
            crate::visual_tests::fixture::Seed::Formats,
        )
        .unwrap();
        let app = world.app();
        let mut state = State::default();
        state.scan(&app.campaigns);
        let group = state
            .owners
            .iter_mut()
            .find(|owner| owner.name == "Ashen Marches")
            .unwrap();
        group.checked = true;
        fs::copy(group.root.join("Module.md"), group.root.join("Package.md")).unwrap();
        let manifest = fs::read_to_string(group.root.join("Package.md")).unwrap();
        fs::write(
            group.root.join("Package.md"),
            splat::content::patch_fields(&manifest, &[("splat_generated", Some(true.into()))])
                .unwrap(),
        )
        .unwrap();
        let before = snapshot(&group.root);
        let root = group.root.clone();
        state.apply();
        assert_eq!(snapshot(&root), before);
    }

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
            owners: vec![OwnerPreview::new(
                dir.path().into(),
                "Campaign".into(),
                true,
            )],
            preview: Some(vec![Preview {
                owner: 0,
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
        state.owners.push(OwnerPreview::new(
            dir.path().into(),
            "Campaign".into(),
            true,
        ));
        for name in ["Rook.md", "Second.md", "Changed.md"] {
            fs::write(dir.path().join(name), old).unwrap();
        }
        state.preview = Some(
            ["Rook.md", "Second.md", "Changed.md"]
                .into_iter()
                .map(|name| Preview {
                    owner: 0,
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
