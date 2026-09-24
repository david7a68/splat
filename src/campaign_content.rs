use crate::{
    campaigns::Campaigns,
    content_workspace::{
        CampaignDocument, Catalogue, CopySource, EntryRef, PackageSelection, Selection,
    },
    library::{note_path, write_note},
    persistence::Campaign,
};
use eframe::egui;
use splat::content::Entry;
use std::{fs, sync::mpsc};
use uuid::Uuid;

struct Snapshot {
    document: CampaignDocument,
    catalogue: Catalogue,
    local_entries: std::sync::OnceLock<Vec<Uuid>>,
}
#[derive(Default)]
pub struct CampaignContent {
    active: Option<Campaign>,
    pending: Option<mpsc::Receiver<Result<Snapshot, String>>>,
    snapshot: Option<Snapshot>,
    saved_selections: Vec<PackageSelection>,
    module: Option<Uuid>,
    search: String,
    entry_matches: Option<(Uuid, String, Vec<Uuid>)>,
    local: bool,
    selected: Option<Uuid>,
    baseline: String,
    draft: String,
    editing: bool,
    error: Option<String>,
    message: Option<String>,
    disabled_selections: std::collections::BTreeMap<Uuid, Selection>,
    refresh_pending: bool,
    repairs: Vec<(String, String, String)>,
    pub copy: Option<CopySource>,
}
impl CampaignContent {
    pub fn dirty(&self) -> bool {
        self.repairs
            .iter()
            .any(|(_, baseline, draft)| baseline != draft)
            || self.draft != self.baseline
            || self
                .snapshot
                .as_ref()
                .is_some_and(|s| s.document.selections != self.saved_selections)
    }
    pub fn refresh_destination(&mut self, root: &std::path::Path) {
        if !self.dirty() && self.active.as_ref().is_some_and(|c| c.path == root) {
            self.refresh_pending = true;
        }
    }
    fn reload(&mut self, ctx: &egui::Context, campaigns: &Campaigns, campaign: Campaign) {
        self.refresh_pending = false;
        self.disabled_selections.clear();
        self.repairs = splat::content::read_directory(&campaign.path)
            .map(|(_, errors)| {
                errors
                    .into_iter()
                    .filter_map(|e| {
                        let text =
                            fs::read_to_string(note_path(&campaign.path, &e.path).ok()?).ok()?;
                        Some((e.path, text.clone(), text))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let modules = campaigns.modules().to_vec();
        self.active = Some(campaign.clone());
        self.snapshot = None;
        self.entry_matches = None;
        self.selected = None;
        self.draft.clear();
        self.baseline.clear();
        self.error = None;
        self.message = None;
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let result = CampaignDocument::load(&campaign).and_then(|document| {
                Ok(Snapshot {
                    document,
                    catalogue: Catalogue::load(&modules, Some(&campaign))?,
                    local_entries: Default::default(),
                })
            });
            let _ = tx.send(result);
            ctx.request_repaint();
        });
    }
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns) {
        let Some(active) = campaigns.active_campaign() else {
            ui.label("Select a campaign first.");
            return;
        };
        if self.refresh_pending {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
        if self
            .active
            .as_ref()
            .is_none_or(|c| c.id != active.id || c.path != active.path)
        {
            if self.dirty() {
                ui.heading("Unsaved campaign content");
                ui.label("The previous campaign still has pending changes. Select it again to save them, or discard and switch.");
                if ui.button("Discard and switch campaign").clicked() {
                    self.reload(ui.ctx(), campaigns, active);
                }
                return;
            }
            self.reload(ui.ctx(), campaigns, active.clone());
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(snapshot) => {
                            self.saved_selections = snapshot.document.selections.clone();
                            self.snapshot = Some(snapshot);
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Loading stopped. Reload to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        ui.heading("Campaign content");
        ui.label("Choose shared rules and keep campaign-specific copies here.");
        let previous_tab = self.local;
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.local, false, "Modules");
            ui.selectable_value(&mut self.local, true, "Campaign entries");
            if ui
                .add_enabled(
                    !self.dirty() && self.pending.is_none(),
                    egui::Button::new("Reload content"),
                )
                .clicked()
            {
                self.reload(ui.ctx(), campaigns, active.clone());
            }
        });
        if previous_tab != self.local || self.dirty() {
            self.message = None;
        }
        let mut repaired = false;
        for (path, baseline, draft) in &mut self.repairs {
            ui.collapsing(format!("Repair invalid note: {path}"), |ui| {
                egui::ScrollArea::vertical()
                    .max_height(250.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(draft)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY),
                        );
                    });
                if ui
                    .add_enabled(draft != baseline, egui::Button::new("Save repaired note"))
                    .clicked()
                {
                    let result = Entry::parse(path, draft).and_then(|entry| {
                        if entry.kind == "package"
                            || (entry.kind == "campaign" && entry.id != active.id)
                        {
                            return Err("Keep campaign ownership consistent with settings.".into());
                        }
                        write_note(&active.path, path, Some(baseline), draft)
                    });
                    match result {
                        Ok(_) => {
                            *baseline = draft.clone();
                            repaired = true;
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                if ui.button("Discard repair draft").clicked() {
                    *draft = baseline.clone();
                }
            });
        }
        if repaired && !self.dirty() {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        if let Some(message) = &self.message {
            ui.colored_label(egui::Color32::LIGHT_GREEN, message);
        }
        if !active.path.exists() {
            ui.label("This campaign folder does not exist yet.");
            if ui.button("Create campaign folder").clicked() {
                match fs::create_dir_all(&active.path) {
                    Ok(()) => self.reload(ui.ctx(), campaigns, active.clone()),
                    Err(e) => self.error = Some(e.to_string()),
                }
            }
        }
        if self.pending.is_some() {
            ui.spinner();
            ui.label("Loading campaign content…");
            return;
        }
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        if snapshot.document.baseline.is_none() {
            ui.weak("No campaign manifest saved yet. Saving selections creates Campaign.md with the default SRD selection.");
        }
        if self.local {
            self.local_entries(ui);
        } else {
            self.modules(ui);
        }
    }
    fn modules(&mut self, ui: &mut egui::Ui) {
        let snapshot = self.snapshot.as_mut().unwrap();
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let closure = catalogue.closure(&document.selections, document.campaign.id);
        ui.horizontal(|ui| {
            let dirty = document.selections != self.saved_selections || document.baseline.is_none();
            if ui
                .add_enabled(
                    dirty && self.repairs.is_empty(),
                    egui::Button::new("Save selections"),
                )
                .clicked()
            {
                match document.save() {
                    Ok(()) => {
                        self.saved_selections = document.selections.clone();
                        self.message = Some("Campaign selections saved.".into());
                        self.error = None;
                    }
                    Err(e) => self.error = Some(e),
                }
            }
            if ui
                .add_enabled(
                    document.selections != self.saved_selections,
                    egui::Button::new("Discard selection changes"),
                )
                .clicked()
            {
                document.selections = self.saved_selections.clone();
            }
            ui.weak(format!(
                "{} direct · {} required",
                closure
                    .direct
                    .iter()
                    .filter(|id| catalogue.selectable(**id))
                    .count(),
                closure
                    .required
                    .keys()
                    .filter(|id| catalogue.selectable(**id))
                    .count()
            ));
        });
        ui.weak("All entries includes future additions. Individual selection stays fixed. Required entries are included separately.");
        let errors: Vec<_> = catalogue
            .errors
            .iter()
            .chain(closure.errors.iter())
            .collect();
        if !errors.is_empty() {
            ui.collapsing(
                format!(
                    "{} content issues · affected uses unavailable",
                    errors.len()
                ),
                |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .show(ui, |ui| {
                            for e in errors {
                                ui.label(e);
                            }
                        });
                },
            );
        }
        // Unavailable registrations and references remain explicit and editable.
        let unavailable: Vec<_> = document
            .selections
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                catalogue
                    .resolve_ref(&s.package)
                    .ok()
                    .is_none_or(|id| !catalogue.modules.iter().any(|m| m.id == id))
            })
            .map(|(i, s)| (i, s.package.label()))
            .collect();
        let mut remove = None;
        for (i, label) in unavailable {
            ui.horizontal(|ui| {
                ui.label(format!("Unavailable module: {label}"));
                if ui.button("Remove selection").clicked() {
                    remove = Some(i);
                }
            });
        }
        if let Some(i) = remove {
            document.selections.remove(i);
        }
        ui.separator();
        if !closure.required.is_empty() {
            egui::CollapsingHeader::new("Required dependencies · all modules")
                .default_open(true)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .show(ui, |ui| {
                            for (id, reasons) in &closure.required {
                                ui.label(format!(
                                    "{} — required by {}",
                                    catalogue.entries[id].entry.name,
                                    reasons.iter().cloned().collect::<Vec<_>>().join(", ")
                                ));
                            }
                        });
                });
        }
        egui::SidePanel::left("campaign_modules")
            .default_width(235.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for module in &catalogue.modules {
                        let existing = document.selections.iter().position(|s| {
                            catalogue.resolve_ref(&s.package).ok() == Some(module.id)
                        });
                        let count = existing.map_or(0, |i| match &document.selections[i].selection {
                            Selection::All => module.entries.iter().filter(|id| catalogue.selectable(**id)).count(),
                            Selection::Selected { entries } => entries.len(),
                        });
                        let partial = existing.is_some_and(|i| matches!(&document.selections[i].selection, Selection::Selected { entries } if !entries.is_empty()));
                        let mut enabled = count > 0;
                        ui.horizontal(|ui| {
                            if ui.add(egui::Checkbox::new(&mut enabled, "").indeterminate(partial)).changed() {
                                self.message = None;
                                if enabled {
                                    if let Some(i) = existing { document.selections.remove(i); }
                                    document.selections.push(PackageSelection {
                                        package: EntryRef::id(module.id),
                                        selection: self.disabled_selections.remove(&module.id).unwrap_or(Selection::All),
                                    });
                                } else if let Some(i) = existing {
                                    let removed = document.selections.remove(i);
                                    self.disabled_selections.insert(module.id, removed.selection);
                                }
                            }
                            if ui
                                .selectable_label(self.module == Some(module.id), &module.name)
                                .clicked()
                            {
                                self.module = Some(module.id);
                                self.search.clear();
                            }
                        });
                        ui.weak(format!(
                            "{} · {} entries",
                            if partial { format!("{count} selected") } else if enabled { "All".into() } else { "None".into() },
                            module
                                .entries
                                .iter()
                                .filter(|id| catalogue.selectable(**id))
                                .count()
                        ));
                        ui.add_space(8.0);
                    }
                });
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let Some(module) = catalogue
                .modules
                .iter()
                .find(|m| Some(m.id) == self.module)
                .or_else(|| catalogue.modules.first())
            else {
                return;
            };
            self.module = Some(module.id);
            ui.heading(&module.name);
            let selection_index = document
                .selections
                .iter()
                .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module.id));
            let entry_ids: Vec<_> = module
                .entries
                .iter()
                .copied()
                .filter(|id| catalogue.selectable(*id))
                .collect();
            let all =
                selection_index.is_some_and(|i| document.selections[i].selection == Selection::All);
            ui.horizontal(|ui| {
                if ui.button("Select all").clicked() {
                    if let Some(i) = selection_index {
                        document.selections[i].selection = Selection::All;
                    } else {
                        document.selections.push(PackageSelection {
                            package: EntryRef::id(module.id),
                            selection: Selection::All,
                        });
                    }
                }
                if ui.button("Clear direct selection").clicked()
                    && let Some(i) = selection_index
                {
                    document.selections.remove(i);
                }
                ui.weak(if all {
                    "Entire module enabled"
                } else {
                    "Individual entries"
                });
            });
            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text("Find an entry…")
                    .desired_width(ui.available_width()),
            );
            let query = self.search.to_lowercase();
            if self
                .entry_matches
                .as_ref()
                .is_none_or(|(module_id, cached_query, _)| {
                    *module_id != module.id || *cached_query != query
                })
            {
                let mut ids: Vec<_> = entry_ids
                    .iter()
                    .copied()
                    .filter(|id| {
                        catalogue.entries[id]
                            .entry
                            .name
                            .to_lowercase()
                            .contains(&query)
                    })
                    .collect();
                ids.sort_by_cached_key(|id| catalogue.entries[id].entry.name.to_lowercase());
                self.entry_matches = Some((module.id, query.clone(), ids));
            }
            let ids = &self.entry_matches.as_ref().unwrap().2;
            let mut toggled = None;
            egui::ScrollArea::vertical()
                .id_salt(("package_entries", module.id, &query))
                .show_rows(ui, 26.0, ids.len(), |ui, range| {
                    for index in range {
                        let id = ids[index];
                        let entry = &catalogue.entries[&id].entry;
                        let required = closure.required.contains_key(&id);
                        let mut selected = closure.direct.contains(&id) || required;
                        ui.push_id(id, |ui| {
                            ui.horizontal(|ui| {
                                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                ui.set_min_height(26.0);
                                if ui
                                    .add_enabled(
                                        !required,
                                        egui::Checkbox::new(&mut selected, &entry.name),
                                    )
                                    .on_hover_text(&entry.name)
                                    .on_disabled_hover_text(&entry.name)
                                    .changed()
                                {
                                    toggled = Some((id, selected));
                                }
                                ui.weak(&entry.kind);
                                if let Some(reasons) = closure.required.get(&id) {
                                    ui.weak("Required").on_hover_text(format!(
                                        "Required by {}",
                                        reasons.iter().cloned().collect::<Vec<_>>().join(", ")
                                    ));
                                }
                            });
                        });
                    }
                });
            if let Some((id, checked)) = toggled {
                let selection_index = document
                    .selections
                    .iter()
                    .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module.id));
                let mut selected = match selection_index.map(|i| &document.selections[i].selection)
                {
                    Some(Selection::All) => entry_ids
                        .iter()
                        .map(|id| EntryRef::id(*id))
                        .collect::<Vec<_>>(),
                    Some(Selection::Selected { entries }) => entries.clone(),
                    None => Vec::new(),
                };
                selected.retain(|r| catalogue.resolve_ref(r).ok() != Some(id));
                if checked {
                    selected.push(EntryRef::id(id));
                }
                let empty = selected.is_empty();
                let selection = Selection::Selected { entries: selected };
                if empty {
                    if let Some(i) = selection_index {
                        document.selections.remove(i);
                    }
                } else if let Some(i) = selection_index {
                    document.selections[i].selection = selection;
                } else {
                    document.selections.push(PackageSelection {
                        package: EntryRef::id(module.id),
                        selection,
                    });
                }
            }
        });
    }
    fn local_entries(&mut self, ui: &mut egui::Ui) {
        let snapshot = self.snapshot.as_ref().unwrap();
        let root = &snapshot.document.campaign.path;
        let campaign = &snapshot.document.campaign;
        let dirty = self.draft != self.baseline;
        ui.weak("Independent campaign entries. Shared module originals remain unchanged.");
        let entry_ids = snapshot.local_entries.get_or_init(|| {
            let mut ids: Vec<_> = snapshot
                .catalogue
                .entries
                .values()
                .filter(|e| e.private && !["campaign", "package"].contains(&e.entry.kind.as_str()))
                .map(|e| e.entry.id)
                .collect();
            ids.sort_by_cached_key(|id| snapshot.catalogue.entries[id].entry.name.to_lowercase());
            ids
        });
        let entries = entry_ids;
        if entries.is_empty() {
            ui.label("No campaign entries yet. Copy an entry from Library to this campaign.");
            return;
        }
        egui::SidePanel::left("local_entries")
            .default_width(235.0)
            .show_inside(ui, |ui| {
                let row_height = ui.spacing().interact_size.y;
                egui::ScrollArea::vertical()
                    .id_salt("campaign_local_entries")
                    .show_rows(ui, row_height, entries.len(), |ui, range| {
                        for row in range {
                            let entry = &snapshot.catalogue.entries[&entries[row]].entry;
                            let response = ui
                                .push_id(entry.id, |ui| {
                                    ui.add_enabled_ui(!dirty, |ui| {
                                        ui.add_sized(
                                            [ui.available_width(), row_height],
                                            egui::Button::selectable(
                                                self.selected == Some(entry.id),
                                                &entry.name,
                                            )
                                            .truncate(),
                                        )
                                    })
                                    .inner
                                })
                                .inner;
                            if response.on_hover_text(&entry.name).clicked() {
                                match note_path(root, &entry.path)
                                    .and_then(|p| fs::read_to_string(p).map_err(|e| e.to_string()))
                                {
                                    Ok(text) => {
                                        self.message = None;
                                        self.selected = Some(entry.id);
                                        self.baseline = text.clone();
                                        self.draft = text;
                                        self.error = None;
                                    }
                                    Err(e) => self.error = Some(e),
                                }
                            }
                        }
                    });
            });
        let mut updated = None;
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let Some(entry) = entries
                .iter()
                .find(|id| Some(**id) == self.selected)
                .map(|id| &snapshot.catalogue.entries[id].entry)
            else {
                ui.label("Select a campaign entry.");
                return;
            };
            ui.heading(&entry.name);
            ui.weak(format!("{} · Campaign-owned", entry.kind));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.editing, false, "Read");
                ui.selectable_value(&mut self.editing, true, "Edit Markdown");
                if ui
                    .add_enabled(dirty, egui::Button::new("Save entry"))
                    .clicked()
                {
                    let result = Entry::parse(&entry.path, &self.draft).and_then(|parsed| {
                        if parsed.id != entry.id
                            || parsed.kind != entry.kind
                            || parsed.key != entry.key
                        {
                            return Err("Keep the entry's UUID, kind and key unchanged.".into());
                        }
                        write_note(root, &entry.path, Some(&self.baseline), &self.draft)
                    });
                    match result {
                        Ok(e) => {
                            self.baseline = self.draft.clone();
                            updated = Some(e);
                            self.message = Some("Campaign entry saved.".into());
                            self.error = None;
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                if ui
                    .add_enabled(dirty, egui::Button::new("Discard draft"))
                    .clicked()
                {
                    self.draft = self.baseline.clone();
                }
                if ui
                    .add_enabled(!dirty, egui::Button::new("Copy to…"))
                    .clicked()
                {
                    self.copy = Some(CopySource {
                        entry: (*entry).clone(),
                        text: self.baseline.clone(),
                        root: Some(root.clone()),
                        campaign: Some(campaign.clone()),
                    });
                }
            });
            egui::ScrollArea::both().show(ui, |ui| {
                if self.editing {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.draft)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY),
                    );
                } else {
                    let parsed = Entry::parse(&entry.path, &self.draft);
                    crate::markdown_view::show(
                        ui,
                        parsed.as_ref().map_or(self.draft.as_str(), |e| &e.body),
                    );
                }
            });
        });
        if let Some(entry) = updated {
            let id = entry.id;
            let snapshot = self.snapshot.as_mut().unwrap();
            snapshot.local_entries.take();
            self.entry_matches = None;
            snapshot.catalogue.entries.get_mut(&id).unwrap().entry = entry;
        }
    }
}
