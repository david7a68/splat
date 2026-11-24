#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use crate::design::{self, Action, Tone, Type, colors};
use crate::{
    campaigns::Campaigns,
    content_workspace::{Catalogue, CopyPlan, CopySource, check_destination},
    persistence::Campaign,
};
use eframe::egui;
use splat::content::Entry;
use std::{path::PathBuf, sync::mpsc};
use uuid::Uuid;

const COPY_FOLDER: design::FieldId = crate::field_id!("copy_folder").once(design::Once::CreateForm);
const COPY_NAME: design::FieldId = crate::field_id!("copy_name").once(design::Once::CreateForm);
const COPY_DESTINATION: design::FieldId =
    crate::field_id!("copy_destination").once(design::Once::CreateForm);

pub struct CopyDialog {
    source: CopySource,
    destination: Option<(bool, Uuid)>,
    name: String,
    folder: String,
    plan: Option<CopyPlan>,
    catalogue: Option<Catalogue>,
    pending: Option<mpsc::Receiver<Result<Catalogue, String>>>,
    error: Option<String>,
    completed: Option<String>,
    pub refresh: Option<PathBuf>,
    /// The open encounter draft, if a copied creature could join it now.
    pub draft: Option<String>,
    copied_creature: Option<Uuid>,
    copied_entry: Option<splat::content::Entry>,
    pub edit_copy: Option<splat::content::Entry>,
    focus_name: bool,
    not_used: bool,
    completed_path: Option<PathBuf>,
    /// The DM chose to add the copy to the open draft.
    pub add_to_draft: Option<Uuid>,
}
impl CopyDialog {
    pub fn prefer_package(&mut self, package: Uuid) {
        self.destination = Some((false, package));
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub fn new(source: CopySource, campaigns: &Campaigns, ctx: &egui::Context) -> Self {
        let modules = campaigns.modules().to_vec();
        let campaign = source.campaign.clone();
        let (tx, rx) = mpsc::channel();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(Catalogue::load(&modules, campaign.as_ref()));
            ctx.request_repaint();
        });
        Self {
            name: source.entry.name.clone(),
            // The likeliest place is the active campaign, unless the entry is already there.
            destination: campaigns
                .active_campaign()
                .filter(|active| source.campaign.as_ref().map(|c| c.id) != Some(active.id))
                .map(|active| (true, active.id)),
            source,
            folder: String::new(),
            plan: None,
            catalogue: None,
            pending: Some(rx),
            error: None,
            completed: None,
            refresh: None,
            draft: None,
            copied_creature: None,
            copied_entry: None,
            edit_copy: None,
            focus_name: true,
            not_used: false,
            completed_path: None,
            add_to_draft: None,
        }
    }
    #[allow(
        clippy::disallowed_methods,
        reason = "the dialog’s error is a typed body message in its place (M20, design::results::STAGED_ALLOWS)"
    )]
    pub fn show(&mut self, ctx: &egui::Context, campaigns: &Campaigns) -> bool {
        self.poll();
        let open = !ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut close = false;
        let mut create = false;
        let destinations = Self::destinations(campaigns);
        design::dialog(ctx, "copy_entry", "Copy entry", |ui| {
            ui.label(
                Type::Heading
                    .rich(ui, &self.source.entry.name)
                    .color(colors().text),
            );
            if self.completed.is_none() {
                ui.label(
                    "This makes a separate copy you can change. Links to the original stay as they are.",
                );
            }
            if let Some(message) = &self.completed {
                let response = ui
                    .scope(|ui| design::message(ui, Tone::Success, message))
                    .response;
                if let Some(path) = &self.completed_path {
                    response.on_hover_text(path.display().to_string());
                }
                design::dialog_actions(
                    ui,
                    |_| (),
                    |ui| {
                        if let (Some(id), Some(draft)) = (self.copied_creature, &self.draft) {
                            let label = format!("Add to {draft}");
                            if Action::primary(&label).show(ui).clicked() {
                                self.add_to_draft = Some(id);
                                close = true;
                            }
                        }
                        let edit = if self.copied_creature.is_some() && self.draft.is_some() {
                            Action::secondary("Edit the copy")
                        } else {
                            Action::primary("Edit the copy")
                        };
                        if edit.show(ui).clicked() {
                            self.edit_copy = self.copied_entry.clone();
                            close = true;
                        }
                        if Action::secondary("Copy another").show(ui).clicked() {
                            self.completed = None;
                            self.plan = None;
                        }
                        design::gap(ui, design::Gap::L);
                        if Action::quiet("Close").show(ui).clicked() {
                            close = true;
                        }
                    },
                );
                return;
            }
            if self.pending.is_some() {
                ui.horizontal(|ui| {
                    crate::design::spinner(ui);
                    ui.label("Checking content dependencies…");
                });
            }
            if let Some(error) = &self.error {
                // A typed message in the dialog's body (M20): a copy that needs an entry of this
                // campaign only waits for the DM (the warning tone); any other failure is an
                // error.
                let fault = if error.contains("which is in this campaign only") {
                    crate::campaigns::NotDone::needs_fixing(error)
                } else {
                    crate::campaigns::NotDone::words(error)
                };
                let _ = design::outcome::BodyMessage::<()>::from(
                    design::outcome::FaultMessage::new(&fault),
                )
                .show(ui);
            }
            let old = self.destination;
            design::field(ui, "Destination", None, None, |ui| {
                // Row 4 (AF1, SF1): the drop-down part marks the chosen destination with its
                // check and keeps every other row's words at one x.
                let mut rows = destinations
                    .iter()
                    .map(|(campaign, id, label, ..)| (Some((*campaign, *id)), label.as_str()));
                if let Some((value, label)) = rows.next() {
                    design::Dropdown::new(COPY_DESTINATION, "Destination")
                        .choice(value, label)
                        .choices(rows)
                        .width(ui.available_width())
                        .show(ui, &mut self.destination);
                }
            });
            let response =
                design::labelled_field(ui, COPY_NAME, "Copy name", &mut self.name, None, None);
            if std::mem::take(&mut self.focus_name) {
                design::focus_and_select(ui, response.id, &self.name);
            }
            let name_changed = response.changed();
            let old_folder = self.folder.clone();
            if let Some(destination) = destinations
                .iter()
                .find(|(c, id, ..)| Some((*c, *id)) == self.destination)
            {
                crate::library::folder_field(
                    ui,
                    COPY_FOLDER,
                    "Folder",
                    &mut self.folder,
                    &destination.3,
                    None,
                );
                if !destination.0 && campaigns.active_campaign().is_some() {
                    crate::library::use_checkbox(
                        ui,
                        &mut self.not_used,
                        |_| "Adds the copy to this campaign’s package choices.",
                    );
                }
            }
            let folder_changed = self.folder != old_folder;
            if old != self.destination || name_changed || folder_changed {
                self.plan = None;
                self.error = None;
            }
            let blocked = self.prepare(&destinations);
            if self.plan.is_none() {
                if let Some(reason) = blocked {
                    ui.label(Type::Small.rich(ui, reason).color(colors().muted));
                }
                design::dialog_actions(
                    ui,
                    |_| (),
                    |ui| {
                        if Action::quiet("Cancel").show(ui).clicked() {
                            close = true;
                        }
                    },
                );
            }
            if let Some(plan) = &self.plan {
                design::rule(ui);

                let destination = destinations
                    .iter()
                    .find(|(campaign, id, ..)| Some((*campaign, *id)) == self.destination)
                    .unwrap();
                let name = destination
                    .2
                    .split_once(" · ")
                    .map_or(destination.2.as_str(), |(_, name)| name);
                crate::library::creation_path(ui, &plan.root, &plan.relative, name);
                for dependency in &plan.shared {
                    if let Ok(entry) = Entry::parse(&dependency.relative, &dependency.text) {
                        ui.label(
                            Type::Small
                                .rich(
                                    ui,
                                    // With one dependency the source links to it; with more,
                                    // one may come through another copied entry.
                                    if plan.shared.len() == 1 {
                                        format!(
                                            "Also copies {}, because {} links to it.",
                                            entry.name, self.source.entry.name
                                        )
                                    } else {
                                        format!(
                                            "Also copies {}, because a copied entry links to it.",
                                            entry.name
                                        )
                                    },
                                )
                                .color(colors().muted),
                        );
                    }
                    crate::library::creation_path(ui, &dependency.root, &dependency.relative, name);
                }
                let shared_names: Vec<_> = plan
                    .shared
                    .iter()
                    .filter_map(|dependency| {
                        Entry::parse(&dependency.relative, &dependency.text)
                            .ok()
                            .map(|entry| entry.name)
                    })
                    .collect();
                if plan.dependencies.is_empty() {
                    ui.label(
                        Type::Small
                            .rich(ui, "Copies no other entry.")
                            .color(colors().muted),
                    );
                } else if plan
                    .dependencies
                    .iter()
                    .any(|name| !shared_names.contains(name))
                {
                    design::section(
                        ui,
                        format!("Required entries: {}", plan.dependencies.len()),
                        |ui| {
                            for d in &plan.dependencies {
                                ui.label(d);
                            }
                        },
                    );
                }
                if !plan.dependencies.is_empty() {
                    design::section(ui, "Review Markdown", |ui| {
                        let height =
                            (ui.available_height() - design::Density::Read.control() - design::L)
                                .clamp(design::XXL, design::XXL * 5.0);
                        design::ScrollColumn::new("copy_review_markdown")
                            .sideways_too()
                            .full_width()
                            .fit_height()
                            .max_height(height)
                            .show(ui, |ui| {
                                ui.monospace(&plan.text);
                            });
                    });
                }
                design::dialog_actions(
                    ui,
                    |_| (),
                    |ui| {
                        create = Action::primary(if plan.shared.is_empty() {
                            "Create copy"
                        } else {
                            "Copy both"
                        })
                        .show(ui)
                        .clicked();
                        if Action::quiet("Cancel").show(ui).clicked() {
                            close = true;
                        }
                    },
                );
            }
        });
        if create {
            self.create(&destinations, campaigns);
        }
        open && !close
    }

    /// A catalogue load that finished (both UIs, each frame the dialog shows).
    pub fn poll(&mut self) {
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(c) => self.catalogue = Some(c),
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Content loading stopped. Close and retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }

    /// Every place a copy can go: each package, then each campaign (is a campaign, id, words,
    /// folder, the campaign).
    pub fn destinations(campaigns: &Campaigns) -> Vec<Destination> {
        campaigns
            .modules()
            .iter()
            .map(|m| {
                (
                    false,
                    m.id,
                    format!("Package · {}", m.name),
                    m.path.clone(),
                    None,
                )
            })
            .chain(campaigns.all().iter().map(|c| {
                (
                    true,
                    c.id,
                    format!("Campaign · {}", c.name),
                    c.path.clone(),
                    Some(c.clone()),
                )
            }))
            .collect()
    }

    /// The plan for the chosen destination, when none is made and nothing blocks it; why the
    /// copy waits, otherwise.
    pub fn prepare(&mut self, destinations: &[Destination]) -> Option<&'static str> {
        let blocked = if self.catalogue.is_none() {
            Some("Still checking content dependencies.")
        } else if self.destination.is_none() {
            Some("Choose a destination first.")
        } else {
            None
        };
        if self.plan.is_none()
            && blocked.is_none()
            && let Some(destination) = destinations
                .iter()
                .find(|(c, id, ..)| Some((*c, *id)) == self.destination)
        {
            let result =
                check_destination(&destination.3, destination.1, destination.0).and_then(|_| {
                    CopyPlan::prepare(
                        &self.source,
                        &destination.3,
                        destination.4.as_ref().map(|c| c.id),
                        &self.name,
                        &self.folder,
                        self.catalogue.as_ref().unwrap(),
                    )
                });
            match result {
                Ok(plan) => {
                    self.plan = Some(plan);
                    self.error = None;
                }
                Err(error) => {
                    if error.contains("which is in this campaign only") {
                        match CopyPlan::prepare_sharing(
                            &self.source,
                            &destination.3,
                            &self.name,
                            &self.folder,
                            self.catalogue.as_ref().unwrap(),
                        ) {
                            Ok(plan) => {
                                self.plan = Some(plan);
                                self.error = None;
                            }
                            Err(error) => self.error = Some(error),
                        }
                    } else {
                        self.error = Some(error);
                    }
                }
            }
        }
        blocked
    }

    /// Create copy (or Copy both): the plan is written, and the dialog says where.
    pub fn create(&mut self, destinations: &[Destination], campaigns: &Campaigns) {
        let Some(plan) = &self.plan else {
            return;
        };
        let result = destinations
            .iter()
            .find(|(c, id, ..)| Some((*c, *id)) == self.destination)
            .ok_or_else(|| "Destination is no longer registered.".to_owned())
            .and_then(|d| {
                check_destination(&d.3, d.1, d.0)?;
                if d.3 != plan.root {
                    return Err("Destination folder changed. Review the copy again.".into());
                }
                let entry = plan.save()?;
                if !d.0
                    && !self.not_used
                    && let Some(campaign) = campaigns.active_campaign()
                {
                    crate::content_workspace::use_in_campaign(
                        &campaign,
                        d.1,
                        d.2.strip_prefix("Package · ").unwrap_or(&d.2),
                        Some(entry.id),
                    )
                    .map_err(|e| format!("Copy saved. Could not use it in this campaign: {e}"))?;
                }
                Ok(entry)
            });
        match result {
            Ok(_) => {
                // A creature copied to the active campaign can join the open draft now. A copy
                // elsewhere is not one this draft can use.
                let to_active = self.destination.is_some_and(|(is_campaign, id)| {
                    (is_campaign && campaigns.active_campaign().is_some_and(|a| a.id == id))
                        || (!is_campaign && !self.not_used && campaigns.active_campaign().is_some())
                });
                self.copied_creature = splat::content::Entry::parse(&plan.relative, &plan.text)
                    .ok()
                    .filter(|entry| to_active && entry.kind == "creature")
                    .map(|entry| entry.id);
                self.copied_entry = splat::content::Entry::parse(&plan.relative, &plan.text).ok();
                let destination_name = destinations
                    .iter()
                    .find(|(c, id, ..)| Some((*c, *id)) == self.destination)
                    .map(|d| d.2.split_once(" · ").map_or(d.2.as_str(), |(_, name)| name))
                    .unwrap_or("the destination");
                self.completed = Some(if plan.shared.is_empty() {
                    format!("Copied to {destination_name} as {}.", self.name.trim())
                } else {
                    let names = plan
                        .shared
                        .iter()
                        .filter_map(|p| {
                            splat::content::Entry::parse(&p.relative, &p.text)
                                .ok()
                                .map(|e| e.name)
                        })
                        .collect::<Vec<_>>()
                        .join(" and ");
                    let folder = std::path::Path::new(&plan.relative)
                        .parent()
                        .filter(|p| !p.as_os_str().is_empty())
                        .map_or("the main folder".into(), |p| p.display().to_string());
                    format!(
                        "Copied {} and {names} to {destination_name}, in {folder}.",
                        self.name.trim()
                    )
                });
                self.completed_path = Some(plan.root.join(&plan.relative));
                self.refresh = Some(plan.root.clone());
                self.error = None;
            }
            Err(e) => self.error = Some(e),
        }
    }
}

/// A copy's destination: is a campaign, its id, its words, its folder, the campaign.
pub type Destination = (bool, Uuid, String, PathBuf, Option<Campaign>);

/// The dialog as data, for a UI that draws it itself (the GPUI port, `docs/gpui-port.md`).
impl CopyDialog {
    pub fn source_name(&self) -> &str {
        &self.source.entry.name
    }

    /// The result after Create copy, its file, and whether `Add to <draft>` is offered.
    pub fn completed(&self) -> Option<(&str, Option<&std::path::Path>, Option<&str>)> {
        let message = self.completed.as_deref()?;
        Some((
            message,
            self.completed_path.as_deref(),
            self.copied_creature.and(self.draft.as_deref()),
        ))
    }

    pub fn loading(&self) -> bool {
        self.pending.is_some()
    }

    /// The error, and whether it is a warning (a copy that needs an entry of this campaign
    /// only waits for the DM).
    pub fn error(&self) -> Option<(&str, bool)> {
        self.error
            .as_deref()
            .map(|error| (error, error.contains("which is in this campaign only")))
    }

    pub fn destination(&self) -> Option<(bool, Uuid)> {
        self.destination
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn folder(&self) -> &str {
        &self.folder
    }

    /// The `Adds the copy to this campaign’s package choices` box: ticked.
    pub fn used(&self) -> bool {
        !self.not_used
    }

    pub fn plan(&self) -> Option<&CopyPlan> {
        self.plan.as_ref()
    }

    /// Whether the name field asks for the keyboard, once.
    pub fn take_focus_name(&mut self) -> bool {
        std::mem::take(&mut self.focus_name)
    }

    /// A new destination, name or folder: the plan is made again.
    pub fn set_destination(&mut self, destination: Option<(bool, Uuid)>) {
        if self.destination != destination {
            self.destination = destination;
            self.plan = None;
            self.error = None;
        }
    }

    pub fn set_name(&mut self, name: &str) {
        if self.name != name {
            self.name = name.to_owned();
            self.plan = None;
            self.error = None;
        }
    }

    pub fn set_folder(&mut self, folder: &str) {
        if self.folder != folder {
            self.folder = folder.to_owned();
            self.plan = None;
            self.error = None;
        }
    }

    pub fn toggle_used(&mut self) {
        self.not_used = !self.not_used;
    }

    /// Copy another: back to the form.
    pub fn copy_another(&mut self) {
        self.completed = None;
        self.plan = None;
    }

    /// Edit the copy: it opens in the Library as the dialog closes.
    pub fn edit_the_copy(&mut self) {
        self.edit_copy = self.copied_entry.clone();
    }

    /// Add to <draft>: the copied creature joins the open encounter draft as the dialog closes.
    pub fn add_copy_to_draft(&mut self) {
        self.add_to_draft = self.copied_creature;
    }
}
