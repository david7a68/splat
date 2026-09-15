//! Module authoring is independent of the active campaign.
use crate::{campaigns::Campaigns, persistence::Module};
use eframe::egui;
use splat::content::{self, Entry};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc,
};
use uuid::Uuid;

struct Note {
    path: String,
    text: String,
    entry: Option<Entry>,
    error: Option<String>,
}
impl Note {
    fn name(&self) -> &str {
        self.entry.as_ref().map_or(&self.path, |e| &e.name)
    }
    fn kind(&self) -> &str {
        self.entry
            .as_ref()
            .map_or("Invalid note", |e| e.kind.as_str())
    }
}
struct Loaded {
    notes: Vec<Note>,
    diagnostics: Vec<String>,
}

fn local_module(path: &Path) -> Result<Module, String> {
    let path = path.canonicalize().map_err(|e| e.to_string())?;
    let (entries, _) = content::read_directory(&path)?;
    let manifests: Vec<_> = entries.iter().filter(|e| e.kind == "package").collect();
    if manifests.len() != 1 || entries.iter().any(|e| e.kind == "campaign") {
        return Err(
            "Choose a module folder with exactly one package manifest and no campaign manifest."
                .into(),
        );
    }
    let manifest = manifests[0];
    let ns = manifest
        .fields
        .get("splat_namespace")
        .and_then(|v| v.as_str())
        .ok_or("The module needs a namespace.")?;
    if !content::resource_key(&format!("{ns}:library")) || ns == "dnd2024" {
        return Err(
            "Use a valid module namespace other than the built-in dnd2024 namespace.".into(),
        );
    }
    Ok(Module {
        id: manifest.id,
        name: manifest.name.clone(),
        path,
    })
}

pub(crate) fn note_text(name: &str, kind: &str, namespace: Option<&str>) -> String {
    let mut fields = serde_yaml_ng::Mapping::new();
    fields.insert("splat_schema".into(), 1.into());
    fields.insert("splat_id".into(), Uuid::new_v4().to_string().into());
    fields.insert("splat_kind".into(), kind.into());
    fields.insert("splat_name".into(), name.into());
    if namespace.is_none() {
        fields.insert(
            format!("splat_{kind}").into(),
            serde_yaml_ng::Value::Mapping(serde_yaml_ng::Mapping::new()),
        );
    }
    if let Some(ns) = namespace {
        fields.insert("splat_namespace".into(), ns.into());
    }
    format!(
        "---\n{}---\n\n# {}\n\n",
        serde_yaml_ng::to_string(&fields).expect("string fields"),
        name
    )
}

fn create_module(path: &Path, name: &str, namespace: &str) -> Result<Module, String> {
    if !path.is_absolute()
        || name.trim().is_empty()
        || namespace == "dnd2024"
        || !content::resource_key(&format!("{namespace}:library"))
    {
        return Err(
            "Enter a name, a new absolute folder path, and a namespace such as my_homebrew.".into(),
        );
    }
    fs::create_dir(path).map_err(|e| format!("Could not create a new module folder: {e}"))?;
    let text = note_text(name.trim(), "package", Some(namespace));
    if let Err(error) = write_note(path, "Module.md", None, &text) {
        let _ = fs::remove_dir(path);
        return Err(error);
    }
    local_module(path)
}

// Refuse traversal and symlinks, and preserve external edits before atomic replacement.
pub(crate) fn note_path(root: &Path, relative: &str) -> Result<PathBuf, String> {
    if fs::symlink_metadata(root)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_symlink()
    {
        return Err(
            "The module folder was replaced with a symbolic link. Register the actual folder."
                .into(),
        );
    }
    let relative = Path::new(relative);
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)))
    {
        return Err("Use a relative note path within the module.".into());
    }
    let mut path = root.to_path_buf();
    for component in relative.components() {
        path.push(component);
        if fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("Editing through symbolic links is not supported.".into());
        }
    }
    Ok(path)
}

pub(crate) fn write_note(
    root: &Path,
    relative: &str,
    baseline: Option<&str>,
    text: &str,
) -> Result<Entry, String> {
    let entry = Entry::parse(relative, text)?;
    let path = note_path(root, relative)?;
    let current = match fs::read_to_string(&path) {
        Ok(text) => Some(text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(e.to_string()),
    };
    if baseline.is_none() && current.is_some() {
        return Err("A note with this name already exists. Choose another name.".into());
    }
    if current.as_deref() != baseline {
        return Err(
            "This note changed on disk. Keep your draft, or discard it and reload before saving."
                .into(),
        );
    }
    let mut temp = tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent folder")?)
        .map_err(|e| e.to_string())?;
    temp.write_all(text.as_bytes())
        .and_then(|_| temp.as_file().sync_all())
        .map_err(|e| e.to_string())?;
    if baseline.is_some() {
        temp.persist(&path).map_err(|e| e.to_string())?;
    } else {
        temp.persist_noclobber(&path).map_err(|e| e.to_string())?;
    }
    Ok(entry)
}

fn load(module: Option<&Module>) -> Result<Loaded, String> {
    let mut notes = Vec::new();
    let mut diagnostics = Vec::new();
    if let Some(module) = module {
        let actual = local_module(&module.path)?;
        if actual.id != module.id {
            return Err(
                "The module at this folder has a different identity. Register it separately."
                    .into(),
            );
        }
        let (entries, errors) = content::read_directory(&module.path)?;
        let report = content::validate(&entries);
        diagnostics.extend(
            report
                .errors
                .into_iter()
                .chain(report.warnings)
                .map(|d| format!("{}: {}", d.path, d.message)),
        );
        for entry in entries {
            let text = fs::read_to_string(note_path(&module.path, &entry.path)?)
                .map_err(|e| e.to_string())?;
            notes.push(Note {
                path: entry.path.clone(),
                text,
                entry: Some(entry),
                error: None,
            });
        }
        for error in errors {
            let text = fs::read_to_string(note_path(&module.path, &error.path)?)
                .map_err(|e| e.to_string())?;
            diagnostics.push(format!("{}: {}", error.path, error.message));
            notes.push(Note {
                path: error.path,
                text,
                entry: None,
                error: Some(error.message),
            });
        }
    } else {
        for entry in content::definitions::entries()? {
            let text = content::definitions::source_file(&entry.path)
                .ok_or("Missing built-in note")?
                .to_owned();
            notes.push(Note {
                path: entry.path.clone(),
                text,
                entry: Some(entry),
                error: None,
            });
        }
    }
    notes.sort_by_cached_key(|n| n.name().to_lowercase());
    Ok(Loaded { notes, diagnostics })
}

#[derive(Default)]
pub struct Library {
    module: Option<Module>,
    loaded: Option<Loaded>,
    pending: Option<mpsc::Receiver<Result<Loaded, String>>>,
    selected: Option<usize>,
    draft: String,
    search: String,
    kind: String,
    error: Option<String>,
    form: bool,
    path: String,
    name: String,
    namespace: String,
    new_name: String,
    new_kind: String,
    source: bool,
    message: Option<String>,
    reselect: Option<String>,
    pub copy: Option<crate::content_workspace::CopySource>,
}
impl Library {
    pub fn dirty(&self) -> bool {
        self.loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .is_some_and(|n| n.text != self.draft)
    }
    pub fn refresh_destination(&mut self, root: &Path, ctx: &egui::Context) {
        if !self.dirty() && self.module.as_ref().is_some_and(|m| m.path == root) {
            self.start_load(ctx);
        }
    }
    fn start_load(&mut self, ctx: &egui::Context) {
        self.reselect = self
            .loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .map(|n| n.path.clone());
        self.message = None;
        let module = self.module.clone();
        let ctx = ctx.clone();
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.loaded = None;
        self.selected = None;
        self.error = None;
        std::thread::spawn(move || {
            let _ = tx.send(load(module.as_ref()));
            ctx.request_repaint();
        });
    }
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &mut Campaigns) {
        if self.loaded.is_none() && self.pending.is_none() && self.error.is_none() {
            self.start_load(ui.ctx());
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(l) => {
                            self.selected = self
                                .reselect
                                .as_ref()
                                .and_then(|p| l.notes.iter().position(|n| &n.path == p));
                            if let Some(i) = self.selected {
                                self.draft = l.notes[i].text.clone();
                            }
                            self.loaded = Some(l);
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Module loading stopped. Reload to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if let (Some(module), Some(loaded)) = (&mut self.module, &self.loaded)
            && let Some(manifest) = loaded
                .notes
                .iter()
                .filter_map(|n| n.entry.as_ref())
                .find(|e| e.id == module.id)
            && module.name != manifest.name
        {
            module.name = manifest.name.clone();
            if let Err(e) = campaigns.rename_module(module.id, module.name.clone()) {
                self.error = Some(e);
            }
        }
        ui.heading("Library");
        ui.label("Work on reusable modules. Campaign selection does not change this workspace.");
        let dirty = self.dirty();
        ui.horizontal(|ui| {
            ui.add_enabled_ui(!dirty && self.pending.is_none(), |ui| {
                let mut id = self.module.as_ref().map(|m| m.id);
                let old = id;
                egui::ComboBox::from_id_salt("library_module")
                    .selected_text(
                        self.module
                            .as_ref()
                            .map_or("D&D 2024 · SRD 5.2.1", |m| &m.name),
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut id, None, "D&D 2024 · SRD 5.2.1");
                        for m in campaigns.modules() {
                            ui.selectable_value(&mut id, Some(m.id), &m.name);
                        }
                    });
                if id != old {
                    self.module = campaigns
                        .modules()
                        .iter()
                        .find(|m| Some(m.id) == id)
                        .cloned();
                    self.search.clear();
                    self.kind.clear();
                    self.start_load(ui.ctx());
                }
                if ui.button("Reload").clicked() {
                    self.start_load(ui.ctx());
                }
                if ui.button("Manage modules").clicked() {
                    self.form = !self.form;
                }
            });
            ui.weak(if self.module.is_none() {
                "Built-in · Read only"
            } else {
                "Local module"
            });
        });
        if self.form {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label("Register an existing module folder, or create a new module.");
                ui.horizontal(|ui| {
                    ui.label("Absolute folder path");
                    ui.text_edit_singleline(&mut self.path);
                });
                ui.label("For a new module only:");
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut self.name);
                    ui.label("Namespace");
                    ui.text_edit_singleline(&mut self.namespace);
                });
                ui.add_enabled_ui(!dirty && self.pending.is_none(), |ui| {
                    ui.horizontal(|ui| {
                        let register = ui.button("Register folder").clicked();
                        let create = ui.button("Create module").clicked();
                        if register || create {
                            let result = if create {
                                create_module(
                                    Path::new(self.path.trim()),
                                    &self.name,
                                    self.namespace.trim(),
                                )
                            } else {
                                local_module(Path::new(self.path.trim()))
                            };
                            match result.and_then(|m| {
                                campaigns.register_module(m.clone())?;
                                Ok(m)
                            }) {
                                Ok(m) => {
                                    self.module = Some(m);
                                    self.form = false;
                                    self.search.clear();
                                    self.kind.clear();
                                    self.start_load(ui.ctx());
                                }
                                Err(e) => self.error = Some(e),
                            }
                        }
                        if self.module.is_some()
                            && ui
                                .button("Unregister module")
                                .on_hover_text("Remove the app registration. Keep all files.")
                                .clicked()
                        {
                            match campaigns.unregister_module(self.module.as_ref().unwrap().id) {
                                Ok(()) => {
                                    self.module = None;
                                    self.start_load(ui.ctx());
                                }
                                Err(e) => self.error = Some(e),
                            }
                        }
                    });
                });
            });
        }
        if let Some(m) = &self.module {
            ui.weak(m.path.display().to_string());
        }
        if let Some(e) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, format!("Error: {e}"));
        }
        if let Some(message) = &self.message {
            ui.label(message);
        }
        if self.pending.is_some() {
            ui.spinner();
            ui.label("Loading module…");
            return;
        }
        if dirty {
            ui.label("Unsaved changes. Save or discard before selecting another entry or module.");
        }
        self.new_entry(ui, dirty);
        let Some(loaded) = &self.loaded else {
            return;
        };
        if !loaded.diagnostics.is_empty() {
            ui.collapsing(format!("{} validation messages", loaded.diagnostics.len()), |ui| {
                egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                    for message in &loaded.diagnostics { ui.label(message); }
                });
                ui.weak("References to other modules may remain unresolved in this module-only check.");
            });
        }
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.text_edit_singleline(&mut self.search);
            egui::ComboBox::from_id_salt("library_kind")
                .selected_text(if self.kind.is_empty() {
                    "All kinds"
                } else {
                    &self.kind
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.kind, String::new(), "All kinds");
                    let kinds: std::collections::BTreeSet<_> =
                        loaded.notes.iter().map(Note::kind).collect();
                    for kind in kinds {
                        ui.selectable_value(&mut self.kind, kind.to_owned(), kind);
                    }
                });
        });
        ui.separator();
        let query = self.search.to_lowercase();
        let matches: Vec<_> = loaded
            .notes
            .iter()
            .enumerate()
            .filter(|(_, n)| {
                (self.kind.is_empty() || n.kind() == self.kind)
                    && (n.name().to_lowercase().contains(&query)
                        || n.path.to_lowercase().contains(&query))
            })
            .map(|(i, _)| i)
            .collect();
        ui.label(format!(
            "{} of {} entries",
            matches.len(),
            loaded.notes.len()
        ));
        let mut select = None;
        egui::SidePanel::left("library_entries")
            .resizable(true)
            .default_width(250.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for i in matches {
                        let note = &loaded.notes[i];
                        if ui
                            .add_enabled(
                                !dirty,
                                egui::Button::selectable(
                                    self.selected == Some(i),
                                    format!("{}\n{}", note.name(), note.kind()),
                                ),
                            )
                            .clicked()
                        {
                            select = Some(i);
                        }
                    }
                });
            });
        if let Some(i) = select {
            self.selected = Some(i);
            self.draft = self.loaded.as_ref().unwrap().notes[i].text.clone();
            self.error = None;
        }
        egui::CentralPanel::default().show_inside(ui, |ui| self.editor(ui));
    }

    fn new_entry(&mut self, ui: &mut egui::Ui, dirty: bool) {
        if self.module.is_none() || self.loaded.is_none() {
            return;
        }
        ui.collapsing("New entry", |ui| {
            ui.label("Create a draft note, then add its rules in the Markdown editor.");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_name);
                if self.new_kind.is_empty() {
                    self.new_kind = "creature".into();
                }
                egui::ComboBox::from_id_salt("new_kind")
                    .selected_text(&self.new_kind)
                    .show_ui(ui, |ui| {
                        for kind in ["creature", "spell", "item", "effect", "rule", "rule_group"] {
                            ui.selectable_value(&mut self.new_kind, kind.into(), kind);
                        }
                    });
                if ui
                    .add_enabled(
                        !dirty && !self.new_name.trim().is_empty(),
                        egui::Button::new("Create entry"),
                    )
                    .clicked()
                {
                    let name = self.new_name.trim();
                    if name.contains(['/', '\\', '\n', '\r']) || name == "." || name == ".." {
                        self.error =
                            Some("Use a name without path separators or line breaks.".into());
                        return;
                    }
                    let path = format!("{name}.md");
                    let text = note_text(name, &self.new_kind, None);
                    match write_note(&self.module.as_ref().unwrap().path, &path, None, &text) {
                        Ok(entry) => {
                            let l = self.loaded.as_mut().unwrap();
                            let created_path = path.clone();
                            l.notes.push(Note {
                                path,
                                text: text.clone(),
                                entry: Some(entry),
                                error: None,
                            });
                            l.notes.sort_by_cached_key(|n| n.name().to_lowercase());
                            self.selected = l.notes.iter().position(|n| n.path == created_path);
                            self.search.clear();
                            self.kind.clear();
                            self.source = true;
                            self.draft = text;
                            self.new_name.clear();
                            self.error = None;
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
            });
        });
    }

    fn editor(&mut self, ui: &mut egui::Ui) {
        let Some(i) = self.selected else {
            ui.label("Select an entry to read or edit it.");
            return;
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        ui.heading(note.name());
        ui.weak(&note.path);
        if let Some(e) = &note.entry {
            ui.weak(format!("{} · {}", e.kind, e.id));
            if let Ok(Some(c)) = content::definitions::conversion(e) {
                ui.label(if c.engine_gaps.is_empty() {
                    "Reference information"
                } else {
                    "Rules specified · Engine support incomplete"
                });
            } else {
                ui.weak("Draft structure does not establish engine support.");
            }
        }
        if let Some(error) = &note.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        let dirty = self.draft != note.text;
        if dirty {
            self.message = None;
        }
        let mut save = dirty
            && self.module.is_some()
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.source, false, "Read");
            ui.selectable_value(
                &mut self.source,
                true,
                if self.module.is_some() {
                    "Edit Markdown"
                } else {
                    "Markdown source"
                },
            );
            if note
                .entry
                .as_ref()
                .is_some_and(|e| !["package", "campaign", "reference"].contains(&e.kind.as_str()))
                && ui
                    .add_enabled(!dirty, egui::Button::new("Copy to…"))
                    .clicked()
            {
                self.copy = Some(crate::content_workspace::CopySource {
                    entry: note.entry.as_ref().unwrap().clone(),
                    text: note.text.clone(),
                    root: self.module.as_ref().map(|m| m.path.clone()),
                    campaign: None,
                });
            }
            if self.module.is_some() {
                save |= ui.add_enabled(dirty, egui::Button::new("Save")).clicked();
                if ui
                    .add_enabled(dirty, egui::Button::new("Discard changes"))
                    .clicked()
                {
                    self.draft = note.text.clone();
                    self.error = None;
                }
            }
        });
        egui::ScrollArea::both()
            .id_salt("entry_body")
            .show(ui, |ui| {
                if self.source {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.draft)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .interactive(self.module.is_some()),
                    );
                } else {
                    let parsed = Entry::parse(&note.path, &self.draft);
                    crate::markdown_view::show(
                        ui,
                        parsed.as_ref().map_or(self.draft.as_str(), |e| &e.body),
                    );
                }
            });
        if save {
            let module = self.module.as_ref().unwrap();
            let result = Entry::parse(&note.path, &self.draft).and_then(|entry| {
                if let Some(old) = &note.entry {
                    if entry.id != old.id || entry.kind != old.kind {
                        return Err("Keep this entry's UUID and kind unchanged.".into());
                    }
                    if old.kind == "package"
                        && entry.fields.get("splat_namespace") != old.fields.get("splat_namespace")
                    {
                        return Err("Keep the module namespace unchanged.".into());
                    }
                }
                if self
                    .loaded
                    .as_ref()
                    .unwrap()
                    .notes
                    .iter()
                    .enumerate()
                    .any(|(other, n)| {
                        other != i
                            && n.entry.as_ref().is_some_and(|e| {
                                e.id == entry.id || (entry.key.is_some() && e.key == entry.key)
                            })
                    })
                {
                    return Err("Another entry already uses this UUID or key.".into());
                }
                write_note(&module.path, &note.path, Some(&note.text), &self.draft)
            });
            match result {
                Ok(entry) => {
                    let l = self.loaded.as_mut().unwrap();
                    l.notes[i].entry = Some(entry);
                    l.notes[i].text = self.draft.clone();
                    l.notes[i].error = None;
                    self.message = Some("Saved: entry changes.".into());
                    let entries: Vec<_> = l.notes.iter().filter_map(|n| n.entry.clone()).collect();
                    let r = content::validate(&entries);
                    l.diagnostics = r
                        .errors
                        .into_iter()
                        .chain(r.warnings)
                        .map(|d| format!("{}: {}", d.path, d.message))
                        .collect();
                    l.diagnostics.extend(
                        l.notes
                            .iter()
                            .filter_map(|n| n.error.as_ref().map(|e| format!("{}: {e}", n.path))),
                    );
                    self.error = None;
                }
                Err(e) => self.error = Some(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_register_and_edit_preserves_identity_and_external_changes() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("module");
        let m = create_module(&root, "My Homebrew", "my_homebrew").unwrap();
        assert_eq!(local_module(&root).unwrap().id, m.id);
        assert!(create_module(&root, "Overwrite", "other").is_err());
        let text = note_text("Goblin", "creature", None);
        let entry = write_note(&root, "Goblin.md", None, &text).unwrap();
        assert!(write_note(&root, "Goblin.md", None, &text).is_err());
        let edited = format!("{text}New description\n");
        assert_eq!(
            write_note(&root, "Goblin.md", Some(&text), &edited)
                .unwrap()
                .id,
            entry.id
        );
        fs::write(root.join("Goblin.md"), "external edit").unwrap();
        assert!(write_note(&root, "Goblin.md", Some(&edited), &text).is_err());
        assert_eq!(
            fs::read_to_string(root.join("Goblin.md")).unwrap(),
            "external edit"
        );
        assert!(write_note(&root, "../escape.md", None, &text).is_err());
    }
    #[test]
    fn every_new_entry_kind_is_a_valid_non_executable_draft() {
        for kind in ["creature", "spell", "item", "effect", "rule", "rule_group"] {
            let text = note_text("Name: with YAML punctuation", kind, None);
            let entry = Entry::parse("Draft.md", &text).unwrap();
            assert_eq!(entry.kind, kind);
            assert_eq!(entry.name, "Name: with YAML punctuation");
        }
    }

    #[test]
    fn library_frame_keeps_campaign_selection_and_draft_independent() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.register_module(m.clone()).unwrap();
        let mut library = Library {
            loaded: Some(load(Some(&m)).unwrap()),
            module: Some(m.clone()),
            selected: Some(0),
            ..Default::default()
        };
        library.draft = library.loaded.as_ref().unwrap().notes[0].text.clone();
        library.draft.push_str("Unsaved description");
        let expected = library.draft.clone();
        let ctx = egui::Context::default();
        for size in [egui::vec2(1000.0, 700.0), egui::vec2(800.0, 600.0)] {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
                },
            );
            assert!(!output.shapes.is_empty());
            assert!(campaigns.active_campaign().is_none());
            assert_eq!(library.draft, expected);
        }
        campaigns.unregister_module(m.id).unwrap();
        assert!(m.path.join("Module.md").exists());
    }

    #[test]
    fn editor_renders_read_and_source_views_without_changing_draft() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        let mut library = Library {
            loaded: Some(load(Some(&m)).unwrap()),
            module: Some(m),
            selected: Some(0),
            ..Default::default()
        };
        library.draft = library.loaded.as_ref().unwrap().notes[0].text.clone();
        library
            .draft
            .push_str("Unsaved content\n\n| Name | Value |\n| --- | --- |\n| Armor | 12 |\n");
        let expected = library.draft.clone();
        let ctx = egui::Context::default();
        for source in [false, true] {
            library.source = source;
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 700.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| library.editor(ui));
                },
            );
            assert!(!output.shapes.is_empty());
            assert_eq!(library.draft, expected);
            assert!(library.dirty());
        }
    }

    #[test]
    fn loading_keeps_invalid_notes_available_for_repair() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        fs::write(
            m.path.join("Broken.md"),
            "---\nsplat_schema: 999\n---\nBroken",
        )
        .unwrap();
        let loaded = load(Some(&m)).unwrap();
        assert_eq!(loaded.notes.len(), 2);
        assert!(loaded.notes.iter().any(|n| n.error.is_some()));
        assert!(!loaded.diagnostics.is_empty());
    }
}
