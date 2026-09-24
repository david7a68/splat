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
#[derive(Default)]
struct Loaded {
    notes: Vec<Note>,
    diagnostics: Vec<String>,
    subtitles: std::sync::OnceLock<Vec<String>>,
    counts: std::sync::OnceLock<Vec<(Section, usize)>>,
}
impl Loaded {
    fn subtitles(&self) -> &Vec<String> {
        self.subtitles.get_or_init(|| self.build_subtitles())
    }
    fn count(&self, section: Section) -> usize {
        self.counts
            .get_or_init(|| {
                Section::GROUPS
                    .iter()
                    .flat_map(|(_, sections)| sections.iter())
                    .map(|&section| {
                        (
                            section,
                            self.notes
                                .iter()
                                .filter(|n| section == Section::All || n.section() == section)
                                .count(),
                        )
                    })
                    .collect()
            })
            .iter()
            .find(|(s, _)| *s == section)
            .map_or(0, |(_, n)| *n)
    }
    fn invalidate(&mut self) {
        self.subtitles.take();
        self.counts.take();
    }
    fn build_subtitles(&self) -> Vec<String> {
        let mut subtitles: Vec<_> = self
            .notes
            .iter()
            .map(|note| {
                note.entry
                    .as_ref()
                    .map_or_else(|| "Invalid note".into(), presentation::subtitle)
            })
            .collect();
        let mut labels = std::collections::HashMap::new();
        for (i, note) in self.notes.iter().enumerate() {
            labels
                .entry((note.name(), subtitles[i].clone()))
                .or_insert_with(Vec::new)
                .push(i);
        }
        for indices in labels.values().filter(|indices| indices.len() > 1) {
            for &i in indices {
                subtitles[i].push_str(&format!(" · {}", self.notes[i].path));
            }
        }
        subtitles
    }
}

const ENTRY_ROW_HEIGHT: f32 = 44.0;

// Fixed height keeps the virtual scroll position independent of label length.
fn entry_row(ui: &mut egui::Ui, name: &str, subtitle: &str, selected: bool) -> egui::Response {
    let (rect, mut response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ENTRY_ROW_HEIGHT),
        egui::Sense::click(),
    );
    // egui hit-tests the previous frame's rectangles. Paint hover at the current
    // position so it does not trail the pointer while the list scrolls.
    let hovered = response.enabled()
        && ui.rect_contains_pointer(rect)
        && (!ui.ctx().is_using_pointer() || response.is_pointer_button_down_on());
    response.flags.set(egui::response::Flags::HOVERED, hovered);
    if ui.is_rect_visible(rect)
        && (selected || hovered || response.is_pointer_button_down_on() || response.has_focus())
    {
        let visuals = ui.style().interact_selectable(&response, selected);
        ui.painter().rect(
            rect.expand(visuals.expansion),
            visuals.corner_radius,
            visuals.weak_bg_fill,
            visuals.bg_stroke,
            egui::StrokeKind::Inside,
        );
    }
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("{name} · {subtitle}"),
        )
    });
    let padding = ui.spacing().button_padding.x;
    for (text, size, y, color) in [
        (name, 15.0, 4.0, ui.visuals().text_color()),
        (subtitle, 12.0, 24.0, ui.visuals().weak_text_color()),
    ] {
        let color = if selected {
            ui.visuals().selection.stroke.color
        } else {
            color
        };
        let mut job = egui::text::LayoutJob::simple_singleline(
            text.replace(['\n', '\r'], " "),
            egui::FontId::proportional(size),
            color,
        );
        job.wrap.max_width = (response.rect.width() - 2.0 * padding).max(0.0);
        job.wrap.max_rows = 1;
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
        ui.painter()
            .galley(response.rect.min + egui::vec2(padding, y), galley, color);
    }
    response
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
    Ok(Loaded {
        notes,
        diagnostics,
        ..Default::default()
    })
}

#[derive(Default)]
pub struct Library {
    module: Option<Module>,
    loaded: Option<Loaded>,
    pending: Option<mpsc::Receiver<Result<Loaded, String>>>,
    selected: Option<usize>,
    draft: String,
    search: String,
    matches: Option<(Section, String, Vec<usize>)>,
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
        self.matches = None;
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
                            self.matches = None;
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
        let searching = !self.search.trim().is_empty();
        egui::SidePanel::left("library_sections")
            .resizable(true)
            .default_width(200.0)
            .width_range(175.0..=250.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("section_list")
                    .show(ui, |ui| {
                        ui.add_space(8.0);
                        for &(group, sections) in Section::GROUPS {
                            // Keep the empty encounter section discoverable, but hide an unused fallback.
                            if sections == [Section::Other] && loaded.count(Section::Other) == 0 {
                                continue;
                            }
                            if !group.is_empty() {
                                ui.add_space(16.0);
                                ui.label(
                                    egui::RichText::new(group.to_uppercase())
                                        .size(11.0)
                                        .color(ui.visuals().weak_text_color()),
                                );
                            }
                            for &section in sections {
                                let count = loaded.count(section);
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
        if self
            .matches
            .as_ref()
            .is_none_or(|(section, cached_query, _)| {
                *section != self.section || *cached_query != query
            })
        {
            self.matches = Some((
                self.section,
                query.clone(),
                loaded
                    .notes
                    .iter()
                    .enumerate()
                    .filter(|(_, n)| n.matches(self.section, &query))
                    .map(|(i, _)| i)
                    .collect(),
            ));
        }
        let matches = &self.matches.as_ref().unwrap().2;
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
        let subtitles = loaded.subtitles();
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
                if matches.is_empty() {
                    ui.weak("No entries found.");
                }
                egui::ScrollArea::vertical()
                    .id_salt(("entry_list", self.section.label(), &query))
                    .show_rows(ui, ENTRY_ROW_HEIGHT, matches.len(), |ui, range| {
                        for row in range {
                            let i = matches[row];
                            let note = &loaded.notes[i];
                            let subtitle = &subtitles[i];
                            let response = ui
                                .push_id(i, |ui| {
                                    ui.add_enabled_ui(!dirty, |ui| {
                                        entry_row(ui, note.name(), subtitle, selected == Some(i))
                                    })
                                    .inner
                                })
                                .inner;
                            if response
                                .on_hover_text(format!(
                                    "{}\n{}\n{}",
                                    note.name(),
                                    subtitle,
                                    note.path
                                ))
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
        let subtitle = self
            .selected
            .map(|i| subtitles[i].clone())
            .unwrap_or_default();
        egui::CentralPanel::default().show_inside(ui, |ui| self.editor(ui, &subtitle));
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
                            l.invalidate();
                            self.matches = None;
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

    fn editor(&mut self, ui: &mut egui::Ui, subtitle: &str) {
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
        if note.entry.is_some() {
            ui.label(
                egui::RichText::new(subtitle)
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
                            let data = section.body.contains("<!-- splat:");
                            let label = if section.title == "Source text" {
                                "Overview".to_owned()
                            } else if data {
                                format!("{} data", section.title)
                            } else {
                                section.title.clone()
                            };
                            let open = !data || first_data;
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
                        let conversion = note
                            .entry
                            .as_ref()
                            .and_then(|e| e.fields.get("splat_conversion"));
                        let status = conversion
                            .and_then(|c| c.get("status"))
                            .and_then(|s| s.as_str());
                        let support = match status {
                            Some("specified") => "Incomplete",
                            Some("informational") => "Reference only",
                            Some("partial") => "Conversion incomplete",
                            _ => "Not established",
                        };
                        ui.collapsing(format!("Engine support · {support}"), |ui| {
                            ui.label(support);
                            if let Some(conversion) = conversion {
                                for key in ["conversion_gaps", "engine_gaps"] {
                                    for gap in conversion
                                        .get(key)
                                        .and_then(|v| v.as_sequence())
                                        .into_iter()
                                        .flatten()
                                    {
                                        if let Some(gap) = gap.as_str() {
                                            ui.monospace(gap);
                                        }
                                    }
                                }
                            } else {
                                ui.label("Draft structure does not establish engine support.");
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
                    l.invalidate();
                    self.matches = None;
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
    fn entry_rows_keep_both_lines_at_a_fixed_height() {
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(186.0, 100.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(170.0);
                    let response = entry_row(
                        ui,
                        &"Long entry name ".repeat(30),
                        &"Long subtitle ".repeat(30),
                        false,
                    );
                    assert_eq!(response.rect.height(), 44.0);
                });
            },
        );
        let rows: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if !text.galley.job.text.is_empty() => Some(&text.galley),
                _ => None,
            })
            .collect();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.iter()
                .all(|galley| galley.rows.len() == 1 && galley.rect.width() <= 162.0)
        );
    }

    #[test]
    fn list_metadata_is_refreshed_after_content_changes() {
        let mut loaded = Loaded {
            notes: vec![Note {
                path: "first.md".into(),
                text: String::new(),
                entry: None,
                error: None,
            }],
            ..Default::default()
        };
        assert_eq!(loaded.count(Section::All), 1);
        assert_eq!(loaded.subtitles().len(), 1);
        loaded.notes.push(Note {
            path: "second.md".into(),
            text: String::new(),
            entry: None,
            error: None,
        });
        loaded.invalidate();
        assert_eq!(loaded.count(Section::All), 2);
        assert_eq!(loaded.subtitles().len(), 2);
    }

    /// Run explicitly to measure the complete library UI while scrolling the built-in package.
    #[test]
    #[ignore]
    fn library_scroll_frame_times() {
        let mut library = Library {
            loaded: Some(load(None).unwrap()),
            ..Default::default()
        };
        let mut campaigns = Campaigns::default();
        let ctx = egui::Context::default();
        let mut times = Vec::new();
        for frame in 0..150 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                time: Some(frame as f64 / 60.0),
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(300.0, 450.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, -30.0),
                        modifiers: Default::default(),
                    },
                ],
                ..Default::default()
            };
            let start = std::time::Instant::now();
            let output = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
            });
            let _ = ctx.tessellate(output.shapes, output.pixels_per_point);
            if frame >= 30 {
                times.push(start.elapsed().as_secs_f64() * 1000.0);
            }
        }
        times.sort_by(f64::total_cmp);
        println!(
            "Library entries: {}; frame CPU ms: median={:.3}, p95={:.3}, max={:.3}",
            library.loaded.as_ref().unwrap().notes.len(),
            times[times.len() / 2],
            times[times.len() * 95 / 100],
            times.last().unwrap()
        );
    }

    #[test]
    fn same_name_entries_have_distinct_visible_labels() {
        let loaded = load(None).unwrap();
        let subtitles = loaded.subtitles();
        let labels: std::collections::HashSet<_> = loaded
            .notes
            .iter()
            .zip(subtitles)
            .map(|(note, subtitle)| (note.name(), subtitle))
            .collect();
        assert_eq!(labels.len(), loaded.notes.len());
    }

    #[test]
    fn imported_overview_renders_as_prose_without_source_markup() {
        for (path, expected) in [
            (
                "definitions/glossary/ability_check.md",
                "represents using one of the six abilities",
            ),
            (
                "definitions/spells/bless.md",
                "Whenever a target makes an attack roll",
            ),
            (
                "definitions/spells/command.md",
                "Approach. The target moves toward you by the shortest",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier_2.md",
                "Strength Physically resist direct force\nDexterity Dodge",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier.md",
                "Wisdom Notice things in the environment or in creatures’ behavior\nCharisma",
            ),
            (
                "definitions/spells/divine_smite.md",
                "if the target is a Fiend or an Undead.",
            ),
            (
                "definitions/spells/augury.md",
                "circumstances, such as other spells, that might change the results.",
            ),
        ] {
            let text = content::definitions::source_file(path).unwrap().to_owned();
            let entry = Entry::parse(path, &text).unwrap();
            let overview = presentation::reading_sections(&entry.body, &entry.name)
                .into_iter()
                .find(|section| section.title == "Source text")
                .unwrap();
            let readable = presentation::overview_markdown(overview.body);
            if path.ends_with("command.md") {
                for option in ["Approach.", "Drop.", "Flee.", "Grovel.", "Halt."] {
                    assert!(readable.contains(&format!("\n\n{option}")), "{readable}");
                }
                assert!(
                    readable.contains("holding and then ends its turn."),
                    "{readable}"
                );
            }
            if path.ends_with("augury.md") {
                assert!(
                    readable.contains(
                        "Weal Good  \nWoe Bad  \nWeal and woe Good and bad  \nIndifference"
                    ),
                    "{readable}"
                );
            }
            let mut library = Library {
                loaded: Some(Loaded {
                    notes: vec![Note {
                        path: entry.path.clone(),
                        text: text.clone(),
                        entry: Some(entry),
                        error: None,
                    }],
                    diagnostics: vec![],
                    ..Default::default()
                }),
                selected: Some(0),
                draft: text,
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let subtitles = library.loaded.as_ref().unwrap().subtitles().clone();
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 700.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| library.editor(ui, &subtitles[0]));
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
            assert!(text.contains(expected), "{text}");
            assert!(!text.contains('\\'), "{text}");
            assert!(!text.contains("[["), "{text}");
            assert!(!text.contains("Original layout:"), "{text}");
        }
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
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
        });
        assert!(campaigns.active_campaign().is_none());
        assert_eq!(library.draft, expected);
        campaigns.unregister_module(m.id).unwrap();
        assert!(m.path.join("Module.md").exists());
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
