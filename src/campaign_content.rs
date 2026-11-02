//! What a campaign uses: package selections, and the repair of invalid notes. This is
//! a rare setup task, so it stays calm, with one scrolling list at 2,000+ rows.
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef, PackageSelection, Selection},
    design::{self, Action, Density, Tone, Type, colors, icons},
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
}
#[derive(Clone, Copy, Default, PartialEq)]
enum Listing {
    #[default]
    Entries,
    Required,
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
    /// The DM's choice of list. Until they choose, a package opens on the list it needs.
    listing: Option<Listing>,
    error: Option<String>,
    message: Option<String>,
    disabled_selections: std::collections::BTreeMap<Uuid, Selection>,
    refresh_pending: bool,
    repairs: Vec<(String, String, String)>,
    /// What `Discard` last dropped, for one step of undo.
    discarded_selections: Option<Vec<PackageSelection>>,
    discarded_repair: Option<(String, String)>,
}
impl CampaignContent {
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub fn dirty(&self) -> bool {
        self.repairs
            .iter()
            .any(|(_, baseline, draft)| baseline != draft)
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
    /// Discarded work belongs to its campaign. Undo must not save it into another.
    fn forget_discards_of_other_campaigns(&mut self, campaign: &Campaign) {
        if self.active.as_ref().is_none_or(|old| old.id != campaign.id) {
            self.discarded_selections = None;
            self.discarded_repair = None;
        }
    }
    fn reload(&mut self, ctx: &egui::Context, campaigns: &Campaigns, campaign: Campaign) {
        self.forget_discards_of_other_campaigns(&campaign);
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
                ui.label(
                    Type::Section
                        .rich(ui, "Unsaved campaign content")
                        .color(colors().text),
                );
                ui.label("The previous campaign still has pending changes. Select it again to save them, or discard and switch.");
                if Action::destructive("Discard and switch campaign")
                    .show(ui)
                    .clicked()
                {
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
        if self.dirty() {
            self.message = None;
        }
        self.repair_section(ui, campaigns, &active);
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        if let Some(message) = &self.message {
            design::message(ui, Tone::Success, message);
        }
        if !active.path.exists() {
            design::message(
                ui,
                Tone::Warning,
                "This campaign folder does not exist yet.",
            );
            if Action::primary("Create campaign folder").show(ui).clicked() {
                match fs::create_dir_all(&active.path) {
                    Ok(()) => self.reload(ui.ctx(), campaigns, active.clone()),
                    Err(e) => self.error = Some(e.to_string()),
                }
            }
        }
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Loading campaign content…");
            });
            return;
        }
        if self.snapshot.is_some() {
            self.packages(ui, campaigns, &active);
        }
    }

    fn repair_section(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        let mut repaired = false;
        for (path, baseline, draft) in &mut self.repairs {
            design::section(ui, format!("Repair invalid note: {path}"), |ui| {
                egui::ScrollArea::vertical()
                    .max_height(220.0)
                    .id_salt(("repair", path.as_str()))
                    .show(ui, |ui| {
                        let response = ui.add(
                            egui::TextEdit::multiline(draft)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY),
                        );
                        design::named(ui, &response, "Note text");
                    });
                ui.horizontal(|ui| {
                    if Action::secondary("Save repaired note")
                        .density(Density::Read)
                        .blocked_if(draft == baseline, "No changes to save.")
                        .show(ui)
                        .clicked()
                        || (draft != baseline
                            && ui.input_mut(|i| {
                                i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)
                            }))
                    {
                        let result = Entry::parse(path, draft).and_then(|entry| {
                            if entry.kind == "package"
                                || (entry.kind == "campaign" && entry.id != active.id)
                            {
                                return Err(
                                    "Keep campaign ownership consistent with settings.".into()
                                );
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
                    ui.add_space(design::XL);
                    if draft != baseline
                        && Action::destructive("Discard repair draft")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                    {
                        self.discarded_repair = Some((path.clone(), draft.clone()));
                        *draft = baseline.clone();
                    }
                    if draft != baseline
                        && self
                            .discarded_repair
                            .as_ref()
                            .is_some_and(|(p, _)| p == path)
                    {
                        self.discarded_repair = None;
                    }
                    if let Some((_, text)) =
                        self.discarded_repair.as_ref().filter(|(p, _)| p == path)
                    {
                        ui.label(
                            Type::Small
                                .rich(ui, "Draft discarded.")
                                .color(colors().muted),
                        );
                        if Action::secondary("Undo discard")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            *draft = text.clone();
                            self.discarded_repair = None;
                        }
                    }
                });
            });
        }
        if repaired && !self.dirty() {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
    }

    fn packages(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns, active: &Campaign) {
        let snapshot = self.snapshot.as_mut().unwrap();
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let closure = catalogue.closure(&document.selections, document.campaign.id);
        let mut reload = false;
        let c = colors();
        ui.horizontal(|ui| {
            let dirty = document.selections != self.saved_selections || document.baseline.is_none();
            let repair = (!self.repairs.is_empty())
                .then_some("Repair the invalid notes above before saving selections.");
            let save_key = dirty
                && repair.is_none()
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
            let pressed = Action::primary("Save selections")
                .density(Density::Read)
                .key("Ctrl+S")
                .blocked(repair.or((!dirty).then_some("No changes to save.")))
                .show(ui)
                .clicked();
            if pressed || save_key {
                match document.save() {
                    Ok(()) => {
                        self.saved_selections = document.selections.clone();
                        self.message = Some("Campaign selections saved.".into());
                        self.error = None;
                    }
                    Err(e) => self.error = Some(e),
                }
            }
            ui.add_space(design::XL);
            let changed = document.selections != self.saved_selections;
            if changed {
                self.discarded_selections = None;
                if Action::destructive("Discard selection changes")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.discarded_selections = Some(std::mem::replace(
                        &mut document.selections,
                        self.saved_selections.clone(),
                    ));
                }
            } else if self.discarded_selections.is_some() {
                ui.label(Type::Small.rich(ui, "Selections discarded.").color(c.muted));
                if Action::secondary("Undo discard")
                    .density(Density::Read)
                    .hint("Bring back the selection changes you discarded")
                    .show(ui)
                    .clicked()
                    && let Some(selections) = self.discarded_selections.take()
                {
                    document.selections = selections;
                }
            }
            if !self.repairs.is_empty() {
                ui.label(
                    Type::Small
                        .rich(ui, "Repair invalid notes before saving.")
                        .color(c.document.invalid),
                );
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                reload = Action::quiet("Reload content")
                    .icon(icons::REFRESH)
                    .density(Density::Read)
                    .blocked_if(
                        document.selections != self.saved_selections
                            || self.repairs.iter().any(|(_, b, d)| b != d),
                        "Save or discard your changes first.",
                    )
                    .show(ui)
                    .clicked();
            });
        });
        let chosen = closure
            .direct
            .iter()
            .filter(|id| catalogue.selectable(**id))
            .count();
        let required = closure
            .required
            .keys()
            .filter(|id| catalogue.selectable(**id))
            .count();
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(
                        ui,
                        if document.baseline.is_none() {
                            format!(
                                "{chosen} entries chosen · {required} more needed by them. Saving creates Campaign.md with this selection."
                            )
                        } else {
                            format!("{chosen} entries chosen · {required} more needed by them")
                        },
                    )
                    .color(c.muted),
            )
            .wrap(),
        );
        let errors: Vec<_> = catalogue
            .errors
            .iter()
            .chain(closure.errors.iter())
            .collect();
        if !errors.is_empty() {
            design::section(
                ui,
                format!(
                    "{} · affected uses unavailable",
                    design::count(errors.len(), "content issue", "content issues")
                ),
                |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .id_salt("campaign_issues")
                        .show(ui, |ui| {
                            for e in errors {
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, e.as_str()).color(c.muted),
                                    )
                                    .wrap(),
                                );
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
            ui.push_id(i, |ui| {
                ui.horizontal(|ui| {
                    design::message(ui, Tone::Warning, &format!("Unavailable package: {label}"));
                    if Action::quiet("Remove selection")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        remove = Some(i);
                    }
                });
            });
        }
        if let Some(i) = remove {
            document.selections.remove(i);
        }
        design::rule(ui);
        egui::SidePanel::left("campaign_modules")
            .resizable(false)
            .exact_width(236.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: 0,
                right: design::M as i8,
                top: design::XS as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("package_list")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for module in &catalogue.modules {
                            let existing = document.selections.iter().position(|s| {
                                catalogue.resolve_ref(&s.package).ok() == Some(module.id)
                            });
                            let total = module
                                .entries
                                .iter()
                                .filter(|id| catalogue.selectable(**id))
                                .count();
                            let count =
                                existing.map_or(0, |i| match &document.selections[i].selection {
                                    Selection::All => total,
                                    Selection::Selected { entries } => entries.len(),
                                });
                            let partial = existing.is_some_and(|i| {
                                matches!(&document.selections[i].selection,
                                    Selection::Selected { entries } if !entries.is_empty())
                            });
                            let mut enabled = count > 0;
                            ui.push_id(module.id, |ui| {
                                ui.horizontal(|ui| {
                                    let check = ui.add(
                                        egui::Checkbox::new(&mut enabled, "")
                                            .indeterminate(partial),
                                    );
                                    design::named(ui, &check, &format!("Use {}", module.name));
                                    if check.changed() {
                                        self.message = None;
                                        if enabled {
                                            if let Some(i) = existing {
                                                document.selections.remove(i);
                                            }
                                            document.selections.push(PackageSelection {
                                                package: EntryRef::id(module.id),
                                                selection: self
                                                    .disabled_selections
                                                    .remove(&module.id)
                                                    .unwrap_or(Selection::All),
                                            });
                                        } else if let Some(i) = existing {
                                            let removed = document.selections.remove(i);
                                            self.disabled_selections
                                                .insert(module.id, removed.selection);
                                        }
                                    }
                                    if design::two_line_row(
                                        ui,
                                        Density::Glance.row() + design::XS,
                                        self.module == Some(module.id),
                                        &module.name,
                                        &format!(
                                            "{} · {total} entries",
                                            if partial {
                                                format!("{count} selected")
                                            } else if enabled {
                                                "All".into()
                                            } else {
                                                "None".into()
                                            },
                                        ),
                                    )
                                    .clicked()
                                    {
                                        self.module = Some(module.id);
                                        self.search.clear();
                                    }
                                });
                            });
                        }
                    });
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::M as i8,
                right: 0,
                top: design::XS as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                let Some(module) = catalogue
                    .modules
                    .iter()
                    .find(|m| Some(m.id) == self.module)
                    .or_else(|| catalogue.modules.first())
                else {
                    return;
                };
                self.module = Some(module.id);
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
                let all = selection_index
                    .is_some_and(|i| document.selections[i].selection == Selection::All);
                ui.horizontal_wrapped(|ui| {
                    ui.label(Type::Section.rich(ui, &module.name).color(c.text));
                    if Action::secondary("Use all")
                        .density(Density::Read)
                        .blocked_if(all, "This whole package is already in use.")
                        .hint("Every entry, including future additions")
                        .show(ui)
                        .clicked()
                    {
                        if let Some(i) = selection_index {
                            document.selections[i].selection = Selection::All;
                        } else {
                            document.selections.push(PackageSelection {
                                package: EntryRef::id(module.id),
                                selection: Selection::All,
                            });
                        }
                    }
                    if Action::quiet("Use none")
                        .density(Density::Read)
                        .blocked_if(selection_index.is_none(), "Nothing here is chosen.")
                        .show(ui)
                        .clicked()
                        && let Some(i) = selection_index
                    {
                        document.selections.remove(i);
                    }
                    ui.label(
                        Type::Small
                            .rich(
                                ui,
                                if all {
                                    "Whole package, with future additions"
                                } else {
                                    "Chosen entries only"
                                },
                            )
                            .color(c.muted),
                    );
                });
                let required_label = format!("Required dependencies ({})", closure.required.len());
                // The built-in package opens on what the chosen entries pull in, with the reasons.
                let mut listing = self.listing.unwrap_or(
                    if module.id == crate::content_workspace::builtin_manifest().id
                        && !closure.required.is_empty()
                    {
                        Listing::Required
                    } else {
                        Listing::Entries
                    },
                );
                let chosen = listing;
                design::segmented(
                    ui,
                    &mut listing,
                    &[
                        (Listing::Entries, "Entries"),
                        (Listing::Required, required_label.as_str()),
                    ],
                );
                if listing != chosen {
                    self.listing = Some(listing);
                }
                design::rule(ui);
                if listing == Listing::Required {
                    // Whose it is: this list covers the whole campaign, not the package above it.
                    let owner = |id: &uuid::Uuid| {
                        let e = &catalogue.entries[id];
                        if e.private {
                            "Campaign".to_owned()
                        } else {
                            catalogue
                                .modules
                                .iter()
                                .find(|m| m.id == e.owner)
                                .map_or_else(|| "Package".to_owned(), |m| m.name.clone())
                        }
                    };
                    // By name, with the kind: two entries can share a name, and an ID order means nothing.
                    let mut required: Vec<_> = closure.required.iter().collect();
                    required.sort_by_key(|(id, _)| {
                        catalogue.entries[*id].entry.name.to_lowercase()
                    });
                    if required.is_empty() {
                        ui.label(
                            Type::Body
                                .rich(ui, "Nothing else is needed by what is chosen.")
                                .color(c.muted),
                        );
                    }
                    // Before `show_rows`, which reads the pitch from it.
                    ui.spacing_mut().item_spacing.y = 0.0;
                    egui::ScrollArea::vertical()
                        .id_salt("required_entries")
                        .auto_shrink([false, false])
                        .show_rows(ui, 24.0, required.len(), |ui, range| {
                            for (id, reasons) in &required[range] {
                                let entry = &catalogue.entries[id].entry;
                                ui.allocate_ui_with_layout(
                                    egui::vec2(ui.available_width(), 24.0),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                ui.add(egui::Label::new(
                                        Type::Body
                                            .rich(
                                                ui,
                                                format!(
                                                    "{} · {} · {} — required by {}",
                                                    entry.name,
                                                    crate::design::reader::human_label(&entry.kind),
                                                    owner(id),
                                                    match reasons.iter().collect::<Vec<_>>().as_slice() {
                                                        [only] => (*only).clone(),
                                                        [first, rest @ ..] => {
                                                            format!("{first} and {} more", rest.len())
                                                        }
                                                        [] => String::new(),
                                                    }
                                                ),
                                            )
                                            .color(c.text),
                                    )
                                    .truncate(),
                                );
                                    },
                                );
                            }
                        });
                    return;
                }
                design::search_field(
                    ui,
                    "Find an entry in this package",
                    &mut self.search,
                    "Find an entry…",
                    ui.available_width(),
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
                    .auto_shrink([false, false])
                    .show_rows(ui, 30.0, ids.len(), |ui, range| {
                        for index in range {
                            let id = ids[index];
                            let entry = &catalogue.entries[&id].entry;
                            let required = closure.required.contains_key(&id);
                            let mut selected = closure.direct.contains(&id) || required;
                            ui.push_id(id, |ui| {
                                ui.horizontal(|ui| {
                                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                                    ui.set_min_height(30.0);
                                    let why = closure.required.get(&id).map(|reasons| {
                                        reasons.iter().cloned().collect::<Vec<_>>().join(", ")
                                    });
                                    if ui
                                        .add_enabled(
                                            !required,
                                            egui::Checkbox::new(&mut selected, &entry.name),
                                        )
                                        .on_hover_text(&entry.name)
                                        .on_disabled_hover_text(why.as_ref().map_or_else(
                                            || entry.name.clone(),
                                            |why| {
                                                format!(
                                                    "{} is required by {why}, so it stays selected.",
                                                    entry.name
                                                )
                                            },
                                        ))
                                        .changed()
                                    {
                                        toggled = Some((id, selected));
                                    }
                                    ui.label(
                                        Type::Small
                                            .rich(ui, crate::library::presentation::subtitle(entry))
                                            .color(c.muted),
                                    );
                                    if let Some(why) = &why {
                                        ui.label(
                                            Type::Small
                                                .rich(ui, format!("Required by {why}"))
                                                .color(c.document.read_only),
                                        );
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
                    let mut selected =
                        match selection_index.map(|i| &document.selections[i].selection) {
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
        if reload {
            self.reload(ui.ctx(), campaigns, active.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn campaign(root: &std::path::Path) -> Campaign {
        Campaign {
            id: uuid::Uuid::new_v4(),
            name: "Test campaign".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        }
    }

    #[test]
    fn undo_discard_stays_in_its_campaign() {
        let temp = tempfile::tempdir().unwrap();
        let (first, second) = (
            campaign(&temp.path().join("a")),
            campaign(&temp.path().join("b")),
        );
        let campaigns = Campaigns::at(temp.path().join("settings.json"));
        let ctx = egui::Context::default();
        let mut content = CampaignContent {
            active: Some(first.clone()),
            discarded_selections: Some(Vec::new()),
            discarded_repair: Some(("note.md".into(), "words".into())),
            ..Default::default()
        };
        content.reload(&ctx, &campaigns, first);
        assert!(content.discarded_selections.is_some(), "a refresh keeps it");
        assert!(content.discarded_repair.is_some(), "a refresh keeps it");
        content.reload(&ctx, &campaigns, second);
        assert!(content.discarded_selections.is_none());
        assert!(content.discarded_repair.is_none());
    }
}
