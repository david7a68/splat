use crate::design::{self, Action, Tone, Type, colors};
use crate::{
    campaigns::Campaigns,
    content_workspace::{Catalogue, CopyPlan, CopySource, check_destination},
    persistence::Campaign,
};
use eframe::egui;
use std::{path::PathBuf, sync::mpsc};
use uuid::Uuid;

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
    /// The DM chose to add the copy to the open draft.
    pub add_to_draft: Option<Uuid>,
}
impl CopyDialog {
    #[cfg(test)]
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
            name: format!("{} copy", source.entry.name),
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
            add_to_draft: None,
        }
    }
    pub fn show(&mut self, ctx: &egui::Context, campaigns: &Campaigns) -> bool {
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
        let open = !ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut close = false;
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
                design::message(ui, Tone::Success, message);
                ui.horizontal(|ui| {
                    if let (Some(id), Some(draft)) = (self.copied_creature, &self.draft) {
                        let label = format!("Add to {draft}");
                        if Action::primary(&label).show(ui).clicked() {
                            self.add_to_draft = Some(id);
                            close = true;
                        }
                    }
                    if Action::secondary("Close").show(ui).clicked() {
                        close = true;
                    }
                    if Action::secondary("Copy another").show(ui).clicked() {
                        self.completed = None;
                        self.plan = None;
                    }
                });
                return;
            }
            if self.pending.is_some() {
                ui.horizontal(|ui| {
                    crate::design::spinner(ui);
                    ui.label("Checking content dependencies…");
                });
            }
            if let Some(error) = &self.error {
                design::message(ui, Tone::Error, error);
            }
            let destinations: Vec<(bool, Uuid, String, PathBuf, Option<Campaign>)> = campaigns
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
                .collect();
            let old = self.destination;
            design::field(ui, "Destination", None, None, |ui| {
                let combo = egui::ComboBox::from_id_salt("copy_destination")
                    .icon(design::combo_icon)
                    .width(ui.available_width().min(360.0))
                    .selected_text(
                        destinations
                            .iter()
                            .find(|(c, id, ..)| Some((*c, *id)) == self.destination)
                            .map_or("Choose destination", |d| d.2.as_str()),
                    )
                    .show_ui(ui, |ui| {
                        for (campaign, id, label, ..) in &destinations {
                            ui.selectable_value(
                                &mut self.destination,
                                Some((*campaign, *id)),
                                label,
                            );
                        }
                    });
                design::named_combo(ui, &combo, "Destination");
            });
            let name_changed =
                design::text_field(ui, "Copy name", &mut self.name, None, None).changed();
            let folder_changed = design::text_field(
                ui,
                "Subfolder",
                &mut self.folder,
                Some("An existing subfolder. Leave blank to use the main folder."),
                None,
            )
            .changed();
            if old != self.destination || name_changed || folder_changed {
                self.plan = None;
                self.error = None;
            }
            let blocked = if self.catalogue.is_none() {
                Some("Still checking content dependencies.")
            } else if self.destination.is_none() {
                Some("Choose a destination first.")
            } else {
                None
            };
            let mut review = false;
            ui.horizontal(|ui| {
                review = Action::secondary("Review copy")
                    .blocked(blocked)
                    .show(ui)
                    .clicked();
                if Action::quiet("Cancel").show(ui).clicked() {
                    close = true;
                }
            });
            if review
                && let Some(destination) = destinations
                    .iter()
                    .find(|(c, id, ..)| Some((*c, *id)) == self.destination)
            {
                let result = check_destination(&destination.3, destination.1, destination.0)
                    .and_then(|_| {
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
                    Err(e) => self.error = Some(e),
                }
            }
            if let Some(plan) = &self.plan {
                design::rule(ui);
                ui.label(Type::Heading.rich(ui, "Ready to copy").color(colors().text));
                ui.add(
                    egui::Label::new(plan.root.join(&plan.relative).display().to_string()).wrap(),
                );
                if plan.dependencies.is_empty() {
                    ui.label("No required content dependencies.");
                } else {
                    design::section(
                        ui,
                        format!(
                            "Dependencies kept as references: {}",
                            plan.dependencies.len()
                        ),
                        |ui| {
                            for d in &plan.dependencies {
                                ui.label(d);
                            }
                        },
                    );
                }
                design::section(ui, "Review Markdown", |ui| {
                    egui::ScrollArea::both().max_height(160.0).show(ui, |ui| {
                        ui.monospace(&plan.text);
                    });
                });
                if Action::primary("Create copy").show(ui).clicked() {
                    let result = destinations
                        .iter()
                        .find(|(c, id, ..)| Some((*c, *id)) == self.destination)
                        .ok_or_else(|| "Destination is no longer registered.".to_owned())
                        .and_then(|d| {
                            check_destination(&d.3, d.1, d.0)?;
                            if d.3 != plan.root {
                                return Err(
                                    "Destination folder changed. Review the copy again.".into()
                                );
                            }
                            plan.save()
                        });
                    match result {
                        Ok(_) => {
                            // A creature copied to the active campaign can join the open
                            // draft now. A copy elsewhere is not one this draft can use.
                            let to_active = self.destination.is_some_and(|(is_campaign, id)| {
                                is_campaign
                                    && campaigns.active_campaign().is_some_and(|a| a.id == id)
                            });
                            self.copied_creature =
                                splat::content::Entry::parse(&plan.relative, &plan.text)
                                    .ok()
                                    .filter(|entry| to_active && entry.kind == "creature")
                                    .map(|entry| entry.id);
                            self.completed = Some(format!("Saved: {}", plan.relative));
                            self.refresh = Some(plan.root.clone());
                            self.error = None;
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
            }
        });
        open && !close
    }
}
