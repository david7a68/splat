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
    undone: bool,
    error: Option<String>,
}

impl State {
    pub fn show_any(&mut self, ui: &mut egui::Ui) -> bool {
        let path = self.notice_file.clone();
        self.show(ui, None, path.as_deref())
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.notice_file = None;
        self.error = None;
        self.skipped.clear();
        self.undone = false;
    }

    pub fn leave(&mut self) {
        self.clear();
        self.preview = None;
        self.skipped.clear();
    }

    /// A notice only owns the bytes from this save, on this file.
    pub fn current(&mut self, file: Option<&Path>) -> bool {
        if self
            .notice_file
            .as_deref()
            .is_some_and(|path| Some(path) != file)
            || self
                .undo
                .iter()
                .any(|change| fs::read(&change.path).ok() != change.after)
        {
            self.clear();
        }
        !self.undo.is_empty()
    }

    pub fn has_action(&self) -> bool {
        self.notice_file.is_some() && !self.undo.is_empty()
    }

    pub fn remember(&mut self, root: &Path, path: &str, before: &str, after: &str) {
        self.clear();
        self.undo = vec![Change::text(root.join(path), Some(before), Some(after))];
        self.notice_file = Some(root.join(path));
    }
    pub fn remember_conversion(&mut self, root: &Path, path: &str, before: &str, after: &str) {
        self.clear();
        if Entry::parse(path, before)
            .is_ok_and(|e| splat::content::formats::schema(&e.fields) == Ok(1))
            && Entry::parse(path, after)
                .is_ok_and(|e| splat::content::formats::schema(&e.fields) == Ok(2))
        {
            self.remember(root, path, before, after);
        }
    }

    pub fn scan(&mut self, campaigns: &Campaigns) {
        self.clear();
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
            let Ok((entries, errors)) = splat::content::read_directory(root) else {
                continue;
            };
            self.skipped.extend(
                errors
                    .into_iter()
                    .map(|e| skipped_file(&e.path, &e.message)),
            );
            for entry in entries {
                if !formats::eligible(&entry) || formats::schema(&entry.fields) != Ok(1) {
                    continue;
                }
                let result = (|| {
                    let path = note_path(root, &entry.path)?;
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
                    Err(error) => self.skipped.push(skipped_file(&entry.path, &error)),
                }
            }
        }
        if let Some(campaign) = campaigns.active_campaign()
            && let Ok(folder) = crate::sessions::folder(&campaign)
        {
            let path = folder.join(crate::timeline::FILE);
            if path.is_file() && !fs::metadata(&path).is_ok_and(|m| m.permissions().readonly()) {
                match fs::read_to_string(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|before| {
                        let after =
                            crate::timeline::formats::convert(&before, &Default::default())?;
                        Ok((before, after))
                    }) {
                    Ok((before, after)) if before != after => preview.push(Preview {
                        owner: 0,
                        root: folder,
                        path: crate::timeline::FILE.into(),
                        before,
                        after,
                    }),
                    Ok(_) => {}
                    Err(error) => self.skipped.push(skipped_file("Timeline.md", &error)),
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
            let change = Change::text(
                file.root.join(&file.path),
                Some(&file.before),
                Some(&file.after),
            );
            let result = if file.path == crate::timeline::FILE {
                (|| {
                    let path = note_path(&file.root, &file.path)?;
                    if [&path, &file.root]
                        .iter()
                        .any(|p| fs::metadata(p).is_ok_and(|m| m.permissions().readonly()))
                    {
                        return Err("This file is read only.".into());
                    }
                    file_batch::apply(std::slice::from_ref(&change))
                })()
            } else {
                write_note(&file.root, &file.path, Some(&file.before), &file.after).map(|_| ())
            };
            match result {
                Ok(()) => self.undo.push(change),
                Err(error) => self.skipped.push(skipped_file(&file.path, &error)),
            }
        }
    }

    pub fn undo(&mut self) -> Result<(), String> {
        let batch = self.notice_file.is_none();
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
        self.clear();
        self.undone = batch;
        Ok(())
    }

    pub fn show_conversion(
        &mut self,
        ui: &mut egui::Ui,
        blocked: Option<&str>,
        file: Option<&Path>,
    ) -> bool {
        self.show_inner(ui, blocked, file, false)
    }

    /// Returns true after disk writes, so the owner can reload its catalogue.
    pub fn show(&mut self, ui: &mut egui::Ui, blocked: Option<&str>, file: Option<&Path>) -> bool {
        self.show_inner(ui, blocked, file, true)
    }

    pub fn show_saved(&mut self, ui: &mut egui::Ui, file: Option<&Path>) -> bool {
        if self.notice_file.is_none() {
            return false;
        }
        self.show_notice(ui, file)
    }

    fn show_notice(&mut self, ui: &mut egui::Ui, file: Option<&Path>) -> bool {
        let mut changed = false;
        if self.current(file) {
            let (words, undo) = self.notice_file.as_ref().map_or_else(
                || {
                    let mut words = format!(
                        "{} now use{} the new layout.",
                        design::count(self.undo.len(), "file", "files"),
                        if self.undo.len() == 1 { "s" } else { "" }
                    );
                    if !self.skipped.is_empty() {
                        words.push_str(&format!(
                            " {} {} skipped.",
                            design::count(self.skipped.len(), "file", "files"),
                            if self.skipped.len() == 1 {
                                "was"
                            } else {
                                "were"
                            }
                        ));
                    }
                    (words, "Undo convert")
                },
                |path| {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    (
                        if name == "Campaign.md" {
                            "Campaign selections saved. Campaign.md now uses the new layout.".into()
                        } else {
                            format!("{name} saved in the new layout.")
                        },
                        if name == "Campaign.md" {
                            "Undo selections"
                        } else {
                            "Undo save"
                        },
                    )
                },
            );
            let tone = if self.skipped.is_empty() {
                Tone::Success
            } else {
                Tone::Warning
            };
            if design::undo_message(ui, tone, &words, undo) {
                match self.undo() {
                    Ok(()) => changed = true,
                    Err(error) => self.error = Some(error),
                }
            }
        }
        changed
    }

    fn show_inner(
        &mut self,
        ui: &mut egui::Ui,
        blocked: Option<&str>,
        file: Option<&Path>,
        action: bool,
    ) -> bool {
        let changed_notice = self.current(file) && (action || self.notice_file.is_none());
        let mut changed = changed_notice && self.show_notice(ui, file);
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        if self.undone {
            design::message(
                ui,
                Tone::Info,
                "Conversion undone. The converted files use the old layout again.",
            );
        }
        if self.preview.is_some() {
            let mut apply = false;
            let mut close = false;
            design::wide_dialog(ui.ctx(), "convert_files", "Convert files", |ui| {
                ui.set_min_width(ui.available_width());
                let files = self.preview.as_ref().unwrap();
                ui.horizontal_wrapped(|ui| {
                    for owner in &mut self.owners {
                        ui.add_enabled_ui(owner.reason.is_none(), |ui| {
                            ui.checkbox(&mut owner.checked, &owner.name);
                        });
                    }
                });
                for owner in &self.owners {
                    if let Some(reason) = owner.reason {
                        ui.label(Type::Small.rich(ui, format!("{}: {reason}", owner.name)));
                    }
                }
                let selected_files: Vec<_> = files
                    .iter()
                    .filter(|file| {
                        self.owners[file.owner].checked && self.owners[file.owner].reason.is_none()
                    })
                    .collect();
                ui.label(format!(
                    "{} use the new layout.",
                    design::count(selected_files.len(), "file can", "files can")
                ));
                let mut kinds = std::collections::BTreeMap::<String, usize>::new();
                for file in &selected_files {
                    if file.path == crate::timeline::FILE {
                        *kinds.entry("timeline".into()).or_default() += 1;
                    } else if let Ok(entry) = Entry::parse(&file.path, &file.before) {
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
                                    kind.replace('_', " ")
                                };
                                if kind == "timeline" {
                                    "the Timeline".into()
                                } else {
                                    design::count(*n, &label, &format!("{label}s"))
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" · "),
                    ),
                );
                if let Some(file) = files.get(self.selected) {
                    let picker = egui::ComboBox::from_id_salt("convert_file")
                        .icon(design::combo_icon)
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
                    let content_height = file.before.lines().count().max(file.after.lines().count())
                        as f32
                        * Type::Mono.line()
                        + design::Density::Read.control() * 2.0;
                    let height = content_height
                        .min(ui.ctx().content_rect().height() * 0.6)
                        .min(
                            (ui.available_height() - design::Density::Read.control() - design::L)
                                .max(design::XXL),
                        );
                    ui.allocate_ui(egui::vec2(ui.available_width(), height), |ui| {
                        ui.set_min_height(height);
                        design::source_comparison(
                            ui,
                            [&file.before, &file.after],
                            ("convert_file", &file.root, &file.path, &file.before),
                        );
                    });
                }
                if !self.skipped.is_empty() {
                    skipped_list(ui, &self.skipped);
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
            if self.undo.is_empty() {
                design::message(
                    ui,
                    Tone::Warning,
                    &format!(
                        "No files converted. {} {} skipped.",
                        design::count(self.skipped.len(), "file", "files"),
                        if self.skipped.len() == 1 {
                            "was"
                        } else {
                            "were"
                        }
                    ),
                );
            }
            skipped_list(ui, &self.skipped);
        }
        changed
    }
}

fn skipped_file(path: &str, error: &str) -> String {
    if error.contains("changed on disk") {
        format!("{path} changed on disk after the preview. Open Convert files again to convert it.")
    } else if error.to_lowercase().contains("read only") {
        format!("{path} is read only. Allow changes to the file, then open Convert files again.")
    } else {
        format!(
            "{path}: {}. Fix the file in Obsidian, then open Convert files again.",
            error.trim_end_matches('.')
        )
    }
}

fn skipped_list(ui: &mut egui::Ui, files: &[String]) {
    design::group_label(ui, "Files skipped");
    for file in files {
        ui.label(file);
    }
}

#[cfg(test)]
pub(crate) fn test_notice_labels(state: &mut State, file: &Path) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::design::theme::install(&ctx);
    ctx.enable_accesskit();
    let mut output = None;
    for _ in 0..3 {
        output = Some(ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                state.show(ui, None, Some(file));
            });
        }));
    }
    output
        .unwrap()
        .platform_output
        .accesskit_update
        .unwrap()
        .nodes
        .into_iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i19_u4_conversion_preview_counts_one_file_and_many_files() {
        for count in [1, 2] {
            let root = std::path::PathBuf::from("/campaign");
            let mut state = State {
                owners: vec![OwnerPreview::new(root.clone(), "Campaign".into(), true)],
                preview: Some(
                    (0..count)
                        .map(|at| Preview {
                            owner: 0,
                            root: root.clone(),
                            path: format!("Rule {at}.md"),
                            before: "old".into(),
                            after: "new".into(),
                        })
                        .collect(),
                ),
                ..Default::default()
            };
            let ctx = egui::Context::default();
            crate::design::theme::install(&ctx);
            let mut output = None;
            for _ in 0..3 {
                output = Some(ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(960.0, 640.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            state.show_conversion(ui, None, None);
                        });
                    },
                ));
            }
            fn texts(shape: &egui::Shape, words: &mut Vec<String>) {
                match shape {
                    egui::Shape::Text(shape) => words.push(shape.galley.text().to_owned()),
                    egui::Shape::Vec(shapes) => {
                        for shape in shapes {
                            texts(shape, words);
                        }
                    }
                    _ => {}
                }
            }
            let mut labels = Vec::new();
            for shape in output.unwrap().shapes {
                texts(&shape.shape, &mut labels);
            }
            let expected = if count == 1 {
                "1 file can use the new layout."
            } else {
                "2 files can use the new layout."
            };
            assert!(labels.iter().any(|label| label == expected), "{labels:?}");
        }
    }

    #[test]
    fn i19_h1_named_save_notice_clears_after_outside_edit_and_second_save() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Rook.md");
        let mut state = State::default();
        fs::write(&path, "saved").unwrap();
        state.remember(dir.path(), "Rook.md", "old", "saved");
        let labels = test_notice_labels(&mut state, &path);
        assert_eq!(
            labels
                .iter()
                .filter(|s| s.starts_with("Undo"))
                .collect::<Vec<_>>(),
            [&"Undo save"]
        );
        fs::write(&path, "Outside edit").unwrap();
        assert!(
            !test_notice_labels(&mut state, &path)
                .iter()
                .any(|s| s.contains("new layout") || s.starts_with("Undo"))
        );
        state.undo().unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "Outside edit");
        fs::write(&path, "saved").unwrap();
        state.remember(dir.path(), "Rook.md", "old", "saved");
        state.remember_conversion(dir.path(), "Rook.md", "saved", "second save");
        fs::write(&path, "second save").unwrap();
        assert!(
            !test_notice_labels(&mut state, &path)
                .iter()
                .any(|s| s.starts_with("Undo"))
        );
    }

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
    fn fmt_f2_f3_owner_filter_keeps_lists_and_timeline_with_their_owner() {
        let dir = tempfile::tempdir().unwrap();
        let world = crate::visual_tests::fixture::World::build(
            dir.path(),
            &dir.path().join("cache"),
            "conversion-list-owners",
            crate::visual_tests::fixture::Seed::Formats,
        )
        .unwrap();
        let app = world.app();
        let campaign = app.campaigns.active_campaign().unwrap();
        let manifest = campaign.path.join("Campaign.md");
        let mut fields = (*Entry::parse("Campaign.md", &fs::read_to_string(&manifest).unwrap())
            .unwrap()
            .fields)
            .clone();
        fields.insert("splat_schema".into(), 1.into());
        fs::write(
            manifest,
            format!("---\n{}---\n", serde_yaml_ng::to_string(&fields).unwrap()),
        )
        .unwrap();
        for root in
            std::iter::once(&campaign.path).chain(app.campaigns.modules().iter().map(|m| &m.path))
        {
            fs::write(root.join("Legacy guards.md"), format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Legacy guards\nsplat_encounter_group: {{side: enemies, members: [{{creature: {{key: 'dnd2024:creatures/goblin_warrior'}}, count: 2, overrides: {{name: Scout, note: Quiet}}}}]}}\n---\n", uuid::Uuid::new_v4())).unwrap();
        }
        let timeline = crate::sessions::open_folder(&campaign)
            .unwrap()
            .join(crate::timeline::FILE);
        let old_timeline = "---\ntimeline_version: 1\ntimeline:\n- id: t-owner-day\n  kind: day\n  title: The crossing\n---\n\n# Timeline\n";
        fs::write(&timeline, old_timeline).unwrap();
        let campaign_before = snapshot(&campaign.path);
        let generated = app
            .campaigns
            .modules()
            .iter()
            .find(|m| m.name == "Generated Book")
            .unwrap();
        let generated_before = snapshot(&generated.path);
        let mut state = State::default();
        state.scan(&app.campaigns);
        let files = state.preview.as_ref().unwrap();
        for kind in ["campaign", "encounter_group", "encounter"] {
            assert!(
                files.iter().any(|f| f.owner == 0
                    && Entry::parse(&f.path, &f.before).is_ok_and(|e| e.kind == kind)),
                "{kind}: {:?}",
                state.skipped
            );
        }
        assert!(
            files
                .iter()
                .any(|f| f.owner == 0 && f.path == crate::timeline::FILE)
        );
        for owner in &mut state.owners {
            owner.checked = owner.name == "Ashen Marches";
        }
        state.apply();
        assert_eq!(snapshot(&campaign.path), campaign_before);
        assert_eq!(snapshot(&generated.path), generated_before);
        let package = app
            .campaigns
            .modules()
            .iter()
            .find(|m| m.name == "Ashen Marches")
            .unwrap();
        assert!(
            fs::read_to_string(package.path.join("Legacy guards.md"))
                .unwrap()
                .contains("splat_schema: 2")
        );
        state.scan(&app.campaigns);
        state.apply();
        assert!(
            fs::read_to_string(&timeline)
                .unwrap()
                .contains("timeline_version: 2")
        );
        assert!(
            fs::read_to_string(campaign.path.join("Legacy guards.md"))
                .unwrap()
                .contains("splat_schema: 2")
        );
        assert_eq!(snapshot(&generated.path), generated_before);
        state.undo().unwrap();
        assert_eq!(snapshot(&campaign.path), campaign_before);
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
