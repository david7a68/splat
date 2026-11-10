//! Explicit file conversion. A preview owns its baselines; Undo owns only saved bytes.
use crate::{
    campaigns::Campaigns,
    design::{self, Action, Tone, Type},
    file_batch::{self, Change},
    library::note_path,
};
use eframe::egui;
#[cfg(test)]
use splat::content::Entry;
use splat::content::formats;
use std::{fs, path::PathBuf};

pub fn is_old(problem: &str) -> bool {
    problem.contains(splat::content::OLD_LAYOUT)
}

pub fn old_message(path: &str) -> String {
    format!("{path} uses the old layout. Splat no longer reads it.")
}

pub fn old_notice(ui: &mut egui::Ui, message: &str, paths: Vec<PathBuf>) {
    let run_open = ui
        .ctx()
        .data(|data| data.get_temp::<bool>(egui::Id::new("conversion_run_open")))
        .unwrap_or(false);
    if design::locked(
        ui,
        run_open.then_some("Close the run before converting files."),
        |ui| design::message_with_action(ui, Tone::Warning, message, "Convert files…"),
    ) {
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("convert_old_files"), paths);
        });
    }
}

pub struct Preview {
    owner: usize,
    root: PathBuf,
    path: String,
    before: String,
    after: String,
}

const GENERATED_REASON: &str = "Its converter will overwrite these files on its next run.";

struct OwnerPreview {
    root: PathBuf,
    name: String,
    checked: bool,
    reason: Option<&'static str>,
}

impl OwnerPreview {
    fn new(root: PathBuf, name: String, campaign: bool) -> Self {
        let generated = !campaign && crate::content_workspace::generated_package(&root);
        Self {
            root,
            name,
            checked: campaign,
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
    skipped: Vec<String>,
    /// The full path of each skipped file, so a reader can tell that FILES SKIPPED names it.
    skipped_files: Vec<PathBuf>,
    undone: bool,
    error: Option<String>,
}

impl State {
    pub fn scan_paths(&mut self, campaigns: &Campaigns, paths: &[PathBuf]) {
        self.scan(campaigns);
        if let Some(files) = &self.preview {
            self.selected = files
                .iter()
                .position(|file| paths.contains(&file.root.join(&file.path)))
                .unwrap_or(0);
            for owner in &mut self.owners {
                owner.checked = paths.iter().any(|path| path.starts_with(&owner.root));
            }
        }
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.error = None;
        self.skipped.clear();
        self.skipped_files.clear();
        self.undone = false;
    }

    pub fn leave(&mut self) {
        self.clear();
        self.preview = None;
        self.skipped.clear();
        self.skipped_files.clear();
    }

    /// True while the FILES SKIPPED notice shows, names this file and offers `Convert files…`.
    pub fn skips(&self, file: &std::path::Path) -> bool {
        self.preview.is_none() && self.skipped_files.iter().any(|skipped| skipped == file)
    }

    /// Undo remains available while the converted bytes still match the disk.
    pub fn current(&mut self) -> bool {
        if self
            .undo
            .iter()
            .any(|change| fs::read(&change.path).ok() != change.after)
        {
            self.clear();
        }
        !self.undo.is_empty()
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
        self.skipped_files.clear();
        let mut preview = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
        for (owner, group) in self.owners.iter().enumerate() {
            let root = &group.root;
            let Ok((entries, errors)) = splat::content::legacy::read_directory(root) else {
                continue;
            };
            for error in errors {
                self.skipped.push(skipped_file(&error.path, &error.message));
                self.skipped_files.push(root.join(&error.path));
            }
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
                    let mut targets: Vec<_> = catalogue
                        .iter()
                        .flat_map(|cat| cat.entries.values().map(|item| item.entry.clone()))
                        .collect();
                    targets.extend(splat::content::legacy::read_directory(root)?.0);
                    let after =
                        crate::conversion::preview(root, &entry.path, before.clone(), &targets)?
                            .after;
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
                    Err(error) => {
                        self.skipped.push(skipped_file(&entry.path, &error));
                        self.skipped_files.push(root.join(&entry.path));
                    }
                }
            }
        }
        for (owner, group) in self.owners.iter().enumerate() {
            let paths = crate::conversion::runs(&group.root)
                .unwrap_or_default()
                .into_iter()
                .chain(crate::conversion::timelines(&group.root).unwrap_or_default());
            for path in paths {
                let relative = path
                    .strip_prefix(&group.root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let result = fs::read_to_string(&path)
                    .map_err(|e| e.to_string())
                    .and_then(|before| {
                        crate::conversion::preview(&group.root, &relative, before, &[])
                    });
                match result {
                    Ok(file) if file.before != file.after => preview.push(Preview {
                        owner,
                        root: file.root,
                        path: file.path,
                        before: file.before,
                        after: file.after,
                    }),
                    Ok(_) => {}
                    Err(error) => {
                        self.skipped.push(skipped_file(&relative, &error));
                        self.skipped_files.push(path);
                    }
                }
            }
        }
        self.preview = Some(preview);
    }

    fn apply(&mut self) {
        let Some(files) = self.preview.take() else {
            return;
        };
        self.undo.clear();
        for file in files {
            if !self
                .owners
                .get(file.owner)
                .is_some_and(|owner| owner.checked)
            {
                continue;
            }
            let candidate = crate::conversion::File {
                root: file.root.clone(),
                path: file.path.clone(),
                before: file.before,
                after: file.after,
            };
            let result = crate::conversion::publish(&candidate);
            match result {
                Ok(change) => self.undo.push(change),
                Err(error) => {
                    self.skipped.push(skipped_file(&file.path, &error));
                    self.skipped_files.push(file.root.join(&file.path));
                }
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
        self.clear();
        self.undone = true;
        Ok(())
    }

    fn show_notice(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        if self.current() {
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
            let undo = "Undo convert";
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

    pub fn show_conversion(
        &mut self,
        ui: &mut egui::Ui,
        blocked: Option<&str>,
        campaigns: &Campaigns,
    ) -> bool {
        let changed_notice = self.current();
        let mut changed = changed_notice && self.show_notice(ui);
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
                        ui.checkbox(&mut owner.checked, &owner.name);
                    }
                });
                for owner in &self.owners {
                    if let Some(reason) = owner.reason {
                        ui.label(Type::Small.rich(ui, format!("{}: {reason}", owner.name)));
                    }
                }
                let selected_files: Vec<_> = files
                    .iter()
                    .filter(|file| self.owners[file.owner].checked)
                    .collect();
                ui.label(format!(
                    "{} use the new layout.",
                    design::count(selected_files.len(), "file can", "files can")
                ));
                let mut kinds = std::collections::BTreeMap::<String, usize>::new();
                for file in &selected_files {
                    if file.path == crate::timeline::FILE {
                        *kinds.entry("timeline".into()).or_default() += 1;
                    } else if let Ok(entry) =
                        splat::content::legacy::parse(&file.path, &file.before)
                    {
                        let kind = if entry.fields.contains_key("splat_character") {
                            "character".into()
                        } else {
                            entry.kind
                        };
                        *kinds.entry(kind).or_default() += 1;
                    }
                }
                if !kinds.is_empty() {
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
                }
                if let Some(file) = files.get(self.selected) {
                    let picker = egui::ComboBox::from_id_salt("convert_file")
                        .icon(design::combo_icon)
                        .selected_text(&file.path)
                        .show_ui(ui, |ui| {
                            for (owner, group) in self.owners.iter().enumerate() {
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
                    let line_height = Type::Mono
                        .line()
                        .max(ui.fonts_mut(|fonts| fonts.row_height(&Type::Mono.font(ui))));
                    let comparison_id = ("convert_file", &file.root, &file.path, &file.before);
                    // Alignment adds missing rows. Size from the rows the panes paint.
                    let content_height = design::source_comparison_rows(
                        ui,
                        [&file.before, &file.after],
                        comparison_id,
                    ) as f32
                        * line_height;
                    // The panes get the height, not the comparison's headings. Do not
                    // cap it by the modal's previous frame, which can keep it short.
                    let label_height = ui.fonts_mut(|fonts| {
                        fonts
                            .layout_no_wrap(
                                "BEFORE".to_owned(),
                                Type::Label.font(ui),
                                design::colors().muted,
                            )
                            .size()
                            .y
                    });
                    let headings = design::Density::Scan.control()
                        + label_height
                        + ui.spacing().item_spacing.y * 2.0;
                    let scroll_bar = ui.spacing().scroll.allocated_width();
                    let room = ui.ctx().content_rect().bottom()
                        - design::XXL
                        - ui.cursor().top()
                        - headings
                        - scroll_bar
                        - design::Density::Scan.control()
                        - design::L;
                    let pane_height = (content_height
                        .min(ui.ctx().content_rect().height() * 0.6)
                        .min(room.max(line_height))
                        / line_height)
                        .floor()
                        * line_height;
                    let height = pane_height + headings + scroll_bar;
                    let rect = egui::Rect::from_min_size(
                        ui.cursor().min,
                        egui::vec2(ui.available_width(), height),
                    );
                    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                        ui.spacing_mut().interact_size.y = design::Density::Scan.control();
                        ui.set_min_height(height);
                        design::source_comparison(ui, [&file.before, &file.after], comparison_id);
                    });
                }
                if !self.skipped.is_empty() {
                    skipped_list(ui, &self.skipped);
                }
                design::dialog_actions(
                    ui,
                    |_| (),
                    |ui| {
                        apply = Action::primary("Convert files")
                            .blocked(
                                blocked.or(selected_files
                                    .is_empty()
                                    .then_some("Select an owner with files to convert.")),
                            )
                            .show(ui)
                            .clicked();
                        close = Action::quiet("Cancel").show(ui).clicked();
                    },
                );
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
            design::group_label(ui, "Files skipped");
            let mut retry = false;
            for file in &self.skipped {
                retry |= design::locked(ui, blocked, |ui| {
                    design::message_with_action(ui, Tone::Warning, file, "Convert files…")
                });
            }
            if retry {
                let undo = std::mem::take(&mut self.undo);
                let checked: Vec<_> = self
                    .owners
                    .iter()
                    .filter(|owner| owner.checked)
                    .map(|owner| owner.root.clone())
                    .collect();
                self.scan(campaigns);
                for owner in &mut self.owners {
                    owner.checked = checked.contains(&owner.root);
                }
                self.undo = undo;
            }
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
mod tests {
    use super::*;

    #[test]
    fn conversion_preview_bad_inputs_and_missing_source_recover_without_stale_faults() {
        let dir = tempfile::tempdir().unwrap();
        let world = crate::visual_tests::fixture::World::build(
            dir.path(),
            &dir.path().join("cache"),
            "conversion-input-recovery",
            crate::visual_tests::fixture::Seed::Formats,
        )
        .unwrap();
        let app = world.app();
        let campaign = app.campaigns.active_campaign().unwrap();
        let path = campaign.path.join("NPCs/Rook.md");
        let valid = fs::read(&path).unwrap();
        let mut state = State::default();
        // These malformed prose rows remain literal source in the preview. The last
        // two inputs cannot be read as notes and must recover on the next scan.
        for (input, readable) in [
            (
                [
                    valid.as_slice(),
                    b"\n| Field | Value |\n| --- | --- |\n| Short |\n",
                ]
                .concat(),
                true,
            ),
            (
                [
                    valid.as_slice(),
                    b"\n| Field | Value |\n| --- | --- |\n| Extra | value | cell |\n",
                ]
                .concat(),
                true,
            ),
            (
                [
                    valid.as_slice(),
                    b"\n| Field | Value |\n| --- | --- |\n| Empty | |\n",
                ]
                .concat(),
                true,
            ),
            (Vec::new(), false),
            (vec![0xff, 0xfe], false),
        ] {
            fs::write(&path, &input).unwrap();
            state.scan(&app.campaigns);
            assert_eq!(fs::read(&path).unwrap(), input, "Preview never writes");
            let files: Vec<_> = state
                .preview
                .as_ref()
                .unwrap()
                .iter()
                .filter(|file| file.root.join(&file.path) == path)
                .collect();
            assert_eq!(files.len(), usize::from(readable));
            if readable {
                assert_eq!(files[0].before.as_bytes(), input);
            }
            fs::write(&path, &valid).unwrap();
            state.scan(&app.campaigns);
            assert!(
                state
                    .preview
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|file| file.root.join(&file.path) == path)
            );
            assert!(
                !state
                    .skipped
                    .iter()
                    .any(|fault| fault.contains("NPCs/Rook.md"))
            );
        }
        for moved in [false, true] {
            state.scan_paths(&app.campaigns, std::slice::from_ref(&path));
            let moved_path = path.with_file_name("Rook moved.md");
            if moved {
                fs::rename(&path, &moved_path).unwrap();
            } else {
                fs::remove_file(&path).unwrap();
            }
            state.apply();
            assert!(
                !path.exists(),
                "Conversion must not recreate a missing source"
            );
            assert!(!state.undo.iter().any(|change| change.path == path));
            assert!(
                state
                    .skipped
                    .iter()
                    .any(|fault| fault.contains("NPCs/Rook.md"))
            );
            if moved {
                assert_eq!(fs::read(&moved_path).unwrap(), valid);
                fs::rename(&moved_path, &path).unwrap();
            } else {
                fs::write(&path, &valid).unwrap();
            }
            state.scan_paths(&app.campaigns, std::slice::from_ref(&path));
            assert!(
                !state
                    .skipped
                    .iter()
                    .any(|fault| fault.contains("NPCs/Rook.md"))
            );
            state.apply();
            assert!(
                fs::read_to_string(&path)
                    .unwrap()
                    .contains("splat_schema: 2")
            );
            state.undo().unwrap();
            assert_eq!(fs::read(&path).unwrap(), valid);
        }
    }

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
                            state.show_conversion(ui, None, &Campaigns::default());
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

    fn snapshot(root: &std::path::Path) -> Vec<(PathBuf, Vec<u8>)> {
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
        let timeline = campaign.path.join("Sessions").join(crate::timeline::FILE);
        let old_timeline = "---\ntimeline_version: 1\ntimeline:\n- id: t-owner-day\n  kind: day\n  title: The crossing\n---\n\n# Timeline\n";
        fs::create_dir_all(timeline.parent().unwrap()).unwrap();
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
        for kind in ["campaign", "encounter_group"] {
            assert!(
                files.iter().any(|f| f.owner == 0
                    && splat::content::legacy::parse(&f.path, &f.before)
                        .is_ok_and(|e| e.kind == kind)),
                "{kind}: {:?}",
                state.skipped
            );
        }
        assert!(
            files
                .iter()
                .any(|f| f.owner == 0 && f.path.ends_with(crate::timeline::FILE))
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
    fn conversion_defaults_to_campaign_and_allows_generated_opt_in() {
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
                .any(|file| file.owner == generated)
        );
        state.apply();
        assert!(!state.undo.is_empty(), "campaign files converted");
        for (module, before) in app.campaigns.modules().iter().zip(&before) {
            assert_eq!(snapshot(&module.path), *before, "{}", module.name);
        }
        state.scan(&app.campaigns);
        for owner in &mut state.owners {
            owner.checked = owner.name == "Generated Book";
        }
        state.apply();
        assert!(!state.undo.is_empty());
        let generated = app
            .campaigns
            .modules()
            .iter()
            .find(|m| m.name == "Generated Book")
            .unwrap();
        assert!(
            fs::read_to_string(generated.path.join("Creatures/Book Wolf.md"))
                .unwrap()
                .contains("splat_schema: 2")
        );
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
