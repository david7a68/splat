//! Package authoring is independent of the active campaign.
mod presentation;
use crate::{campaigns::Campaigns, persistence::Module};
use eframe::egui;
use presentation::Section;
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
    fn section(&self) -> Section {
        self.entry
            .as_ref()
            .map_or(Section::Other, Section::for_entry)
    }
    fn matches(&self, section: Section, query: &str) -> bool {
        if !query.is_empty() {
            self.name().to_lowercase().contains(query)
                || self.path.to_lowercase().contains(query)
                || self.section().label().to_lowercase().contains(query)
        } else {
            section == Section::All || self.section() == section
        }
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
    section: Section,
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
        let dark = ui.visuals().dark_mode;
        ui.visuals_mut().override_text_color =
            Some(egui::Color32::from_gray(if dark { 218 } else { 35 }));
        ui.visuals_mut().weak_text_color =
            Some(egui::Color32::from_gray(if dark { 165 } else { 85 }));
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
                    self.error = Some("Package loading stopped. Reload to retry.".into());
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
        ui.label(egui::RichText::new("Library").size(26.0).strong());
        ui.label(
            egui::RichText::new("Shared content for all campaigns")
                .small()
                .color(ui.visuals().weak_text_color()),
        );
        ui.add_space(3.0);
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
                    self.section = Section::All;
                    self.start_load(ui.ctx());
                }
                ui.menu_button("Package actions", |ui| {
                    if ui.button("Reload package").clicked() {
                        self.start_load(ui.ctx());
                        ui.close();
                    }
                    if ui.button("Manage packages…").clicked() {
                        self.form = true;
                        ui.close();
                    }
                });
            });
            ui.weak(if self.module.is_none() {
                "Built-in · Read only"
            } else {
                "Local package"
            });
        });
        if self.form {
            let mut open = true;
            egui::Window::new("Manage packages")
                .open(&mut open)
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                    ui.label("Register an existing package folder, or create a new package.");
                    ui.horizontal(|ui| {
                        ui.label("Absolute folder path");
                        ui.text_edit_singleline(&mut self.path);
                    });
                    ui.label("For a new package only:");
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.name);
                        ui.label("Namespace");
                        ui.text_edit_singleline(&mut self.namespace);
                    });
                    ui.add_enabled_ui(!dirty && self.pending.is_none(), |ui| {
                        ui.horizontal(|ui| {
                            let register = ui.button("Register folder").clicked();
                            let create = ui.button("Create package").clicked();
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
                                        self.section = Section::All;
                                        self.start_load(ui.ctx());
                                    }
                                    Err(e) => self.error = Some(e),
                                }
                            }
                            if self.module.is_some()
                                && ui
                                    .button("Unregister package")
                                    .on_hover_text("Remove the app registration. Keep all files.")
                                    .clicked()
                            {
                                match campaigns.unregister_module(self.module.as_ref().unwrap().id)
                                {
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
            self.form &= open;
        }
        if let Some(e) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, format!("Error: {e}"));
        }
        if let Some(message) = &self.message {
            ui.label(message);
        }
        if self.pending.is_some() {
            ui.spinner();
            ui.label("Loading package…");
            return;
        }
        if dirty {
            ui.label("Unsaved changes. Save or discard before selecting another entry or package.");
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
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text("Search all sections by name or path…")
                    .desired_width((ui.available_width() - 70.0).max(120.0)),
            );
            if !self.search.is_empty() && ui.button("Clear").clicked() {
                self.search.clear();
            }
        });
        ui.add_space(6.0);
        ui.separator();
        let query = self.search.trim().to_lowercase();
        let searching = !query.is_empty();
        egui::SidePanel::left("library_sections")
            .resizable(true)
            .default_width(200.0)
            .width_range(175.0..=250.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("section_list")
                    .show(ui, |ui| {
                        ui.add_space(8.0);
                        if ui
                            .add_enabled(
                                !dirty,
                                egui::Button::selectable(
                                    self.section == Section::All && !searching,
                                    format!("All entries   {}", loaded.notes.len()),
                                )
                                .min_size(egui::vec2(ui.available_width(), 30.0)),
                            )
                            .clicked()
                        {
                            self.section = Section::All;
                            self.search.clear();
                        }
                        for &(group, sections) in Section::GROUPS {
                            // Keep the empty encounter section discoverable, but hide an unused fallback.
                            if sections == [Section::Other]
                                && !loaded.notes.iter().any(|n| n.section() == Section::Other)
                            {
                                continue;
                            }
                            ui.add_space(16.0);
                            ui.label(
                                egui::RichText::new(group.to_uppercase())
                                    .size(11.0)
                                    .color(ui.visuals().weak_text_color()),
                            );
                            for &section in sections {
                                let count = loaded
                                    .notes
                                    .iter()
                                    .filter(|n| n.section() == section)
                                    .count();
                                if ui
                                    .add_enabled(
                                        !dirty,
                                        egui::Button::selectable(
                                            self.section == section && !searching,
                                            format!("{}   {count}", section.label()),
                                        )
                                        .min_size(egui::vec2(ui.available_width(), 30.0)),
                                    )
                                    .clicked()
                                {
                                    self.section = section;
                                    self.search.clear();
                                }
                            }
                        }
                    });
            });
        // A query always searches the whole package. Clearing it returns to the selected section.
        let query = self.search.trim().to_lowercase();
        let searching = !query.is_empty();
        let matches: Vec<_> = loaded
            .notes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.matches(self.section, &query))
            .map(|(i, _)| i)
            .collect();
        // Keep the reader in sync with the visible list without dropping an unsaved draft.
        let mut select = None;
        if !dirty && self.selected.is_none_or(|i| !matches.contains(&i)) {
            select = matches.first().copied();
            if select.is_none() {
                self.selected = None;
                self.draft.clear();
            }
        }
        let selected = select.or(self.selected);
        let list_width = (ui.available_width() * 0.32).clamp(190.0, 290.0);
        egui::SidePanel::left("library_entries")
            .resizable(true)
            .default_width(list_width)
            .width_range(170.0..=360.0)
            .show_inside(ui, |ui| {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(if searching {
                        "Search results"
                    } else {
                        self.section.label()
                    })
                    .size(18.0)
                    .strong(),
                );
                ui.weak(format!(
                    "{} entries{}",
                    matches.len(),
                    if searching { " · all sections" } else { "" }
                ));
                ui.separator();
                egui::ScrollArea::vertical()
                    .id_salt("entry_list")
                    .show(ui, |ui| {
                        if matches.is_empty() {
                            ui.weak("No entries found.");
                        }
                        for i in matches {
                            let note = &loaded.notes[i];
                            let mut text = egui::text::LayoutJob::default();
                            text.append(
                                note.name(),
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::TextStyle::Body.resolve(ui.style()),
                                    color: if selected == Some(i) {
                                        ui.visuals().selection.stroke.color
                                    } else {
                                        ui.visuals().text_color()
                                    },
                                    ..Default::default()
                                },
                            );
                            let subtitle = note
                                .entry
                                .as_ref()
                                .map_or_else(|| "Invalid note".into(), presentation::subtitle);
                            text.append(
                                &format!("\n{subtitle}"),
                                0.0,
                                egui::TextFormat {
                                    font_id: egui::FontId::proportional(12.0),
                                    color: if selected == Some(i) {
                                        ui.visuals().selection.stroke.color
                                    } else {
                                        ui.visuals().weak_text_color()
                                    },
                                    ..Default::default()
                                },
                            );
                            if ui
                                .add_enabled(
                                    !dirty,
                                    egui::Button::selectable(selected == Some(i), text)
                                        .min_size(egui::vec2(ui.available_width(), 40.0)),
                                )
                                .on_hover_text(&note.path)
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
                            self.section = Section::All;
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
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(format!("Library / {}", note.section().label()))
                .size(12.0)
                .color(ui.visuals().weak_text_color()),
        );
        ui.label(egui::RichText::new(note.name()).size(28.0).strong());
        if let Some(entry) = &note.entry {
            ui.label(
                egui::RichText::new(presentation::subtitle(entry))
                    .size(13.0)
                    .color(ui.visuals().weak_text_color()),
            );
        }
        ui.add_space(8.0);
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
                    let body = parsed.as_ref().map_or(self.draft.as_str(), |e| &e.body);
                    ui.push_id(&note.path, |ui| {
                        let mut first_data = true;
                        for (index, section) in presentation::reading_sections(body, note.name())
                            .iter()
                            .enumerate()
                        {
                            let overview =
                                section.title == "Source text" || section.title == "Overview";
                            let data = section.body.contains("<!-- splat:");
                            let label = if section.title == "Source text" {
                                "Overview".to_owned()
                            } else if data {
                                format!("{} data", section.title)
                            } else {
                                section.title.clone()
                            };
                            let open = overview || !data || first_data;
                            if data {
                                first_data = false;
                            }
                            ui.add_space(if index == 0 { 14.0 } else { 10.0 });
                            egui::CollapsingHeader::new(
                                egui::RichText::new(label).size(17.0).strong(),
                            )
                            .id_salt(("reading_section", index))
                            .default_open(open)
                            .show_unindented(ui, |ui| {
                                ui.add_space(5.0);
                                if data {
                                    egui::Frame::new()
                                        .fill(ui.visuals().faint_bg_color)
                                        .inner_margin(10.0)
                                        .corner_radius(4.0)
                                        .show(ui, |ui| {
                                            ui.set_width(ui.available_width());
                                            crate::markdown_view::show_compact(ui, section.body);
                                        });
                                } else if self.module.is_none() && section.title == "Source text" {
                                    let readable = presentation::overview_markdown(section.body);
                                    crate::markdown_view::show(ui, &readable);
                                } else {
                                    crate::markdown_view::show(ui, section.body);
                                }
                            });
                        }
                        ui.add_space(16.0);
                        ui.separator();
                        let support_label = note
                            .entry
                            .as_ref()
                            .and_then(|e| e.fields.get("splat_conversion"))
                            .and_then(|c| c.get("status"))
                            .and_then(|s| s.as_str())
                            .map_or("Engine support · Not established", |s| match s {
                                "specified" => "Engine support · Incomplete",
                                "informational" => "Engine support · Reference only",
                                _ => "Engine support · Conversion incomplete",
                            });
                        ui.collapsing(support_label, |ui| {
                            if let Some(entry) = &note.entry {
                                match content::definitions::conversion(entry) {
                                    Ok(Some(c)) => {
                                        ui.label(match c.status {
                                            content::definitions::Status::Informational => {
                                                "Reference information"
                                            }
                                            content::definitions::Status::Partial => {
                                                "Rule conversion incomplete"
                                            }
                                            content::definitions::Status::Specified => {
                                                "Rules specified · Engine support incomplete"
                                            }
                                        });
                                        for gap in c.conversion_gaps.iter().chain(&c.engine_gaps) {
                                            ui.monospace(gap);
                                        }
                                    }
                                    Ok(None) => {
                                        ui.label(
                                            "Draft structure does not establish engine support.",
                                        );
                                    }
                                    Err(error) => {
                                        ui.colored_label(egui::Color32::LIGHT_RED, error);
                                    }
                                }
                            }
                        });
                        ui.collapsing("Source and identifiers", |ui| {
                            ui.label(if self.module.is_none() {
                                "Built-in package · Read only"
                            } else {
                                "Local package"
                            });
                            ui.label(&note.path);
                            if let Some(entry) = &note.entry {
                                ui.label(format!("{} · {}", entry.kind, entry.id));
                                if let Some(key) = &entry.key {
                                    ui.monospace(key);
                                }
                                if let Some(sources) = entry
                                    .fields
                                    .get("splat_sources")
                                    .and_then(|v| v.as_sequence())
                                {
                                    for source in sources {
                                        let title = source
                                            .get("title")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Source");
                                        let locator = source
                                            .get("locator")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("");
                                        ui.label(format!("{title} · {locator}"));
                                    }
                                }
                            }
                        });
                    });
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
    fn imported_overview_renders_as_prose_without_source_markup() {
        let text = content::definitions::source_file("definitions/glossary/ability_check.md")
            .unwrap()
            .to_owned();
        let entry = Entry::parse("definitions/glossary/ability_check.md", &text).unwrap();
        let mut library = Library {
            loaded: Some(Loaded {
                notes: vec![Note {
                    path: entry.path.clone(),
                    text: text.clone(),
                    entry: Some(entry),
                    error: None,
                }],
                diagnostics: vec![],
            }),
            selected: Some(0),
            draft: text,
            ..Default::default()
        };
        let ctx = egui::Context::default();
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
        let text = output
            .shapes
            .iter()
            .filter_map(|s| match &s.shape {
                egui::Shape::Text(t) => Some(t.galley.job.text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            text.contains("represents using one of the six abilities"),
            "{text}"
        );
        assert!(!text.contains('\\'), "{text}");
        assert!(!text.contains("[["), "{text}");
        assert!(!text.contains("Original layout:"), "{text}");
    }
    #[test]
    fn search_crosses_sections_and_invalid_notes_remain_reachable() {
        let text = note_text("Goblin", "creature", None);
        let note = Note {
            path: "Monsters/Goblin.md".into(),
            entry: Some(Entry::parse("Monsters/Goblin.md", &text).unwrap()),
            text,
            error: None,
        };
        assert!(note.matches(Section::Creatures, ""));
        assert!(!note.matches(Section::Spells, ""));
        assert!(note.matches(Section::Spells, "goblin"));
        assert!(note.matches(Section::Spells, "monsters/"));
        let broken = Note {
            path: "Broken.md".into(),
            text: "invalid".into(),
            entry: None,
            error: Some("Invalid note".into()),
        };
        assert!(broken.matches(Section::Other, ""));
        assert!(broken.matches(Section::All, ""));
        assert!(broken.matches(Section::Spells, "broken"));
    }
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
        library.search = "no matching entry".into();
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
