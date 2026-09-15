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
}
impl CopyDialog {
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
            source,
            destination: None,
            folder: String::new(),
            plan: None,
            catalogue: None,
            pending: Some(rx),
            error: None,
            completed: None,
            refresh: None,
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
        let mut open = !ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut close = false;
        egui::Window::new("Copy entry").default_width(560.0).resizable(true).collapsible(false).open(&mut open).show(ctx, |ui| {
            ui.heading(&self.source.entry.name);
            ui.label("Create an independent copy with a new identity. Existing references stay unchanged.");
            if let Some(message) = &self.completed {
                ui.label(message);
                ui.label("The destination refreshes automatically when it has no unsaved changes.");
                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() { close = true; }
                    if ui.button("Copy another").clicked() { self.completed = None; self.plan = None; }
                });
                return;
            }
            if self.pending.is_some() { ui.spinner(); ui.label("Checking content dependencies…"); }
            if let Some(error) = &self.error { ui.colored_label(egui::Color32::LIGHT_RED, error); }
            let destinations: Vec<(bool,Uuid,String,PathBuf,Option<Campaign>)> = campaigns.modules().iter().map(|m| (false,m.id,format!("Module · {}",m.name),m.path.clone(),None))
                .chain(campaigns.all().iter().map(|c| (true,c.id,format!("Campaign · {}",c.name),c.path.clone(),Some(c.clone())))).collect();
            let old = self.destination;
            egui::ComboBox::from_id_salt("copy_destination").selected_text(destinations.iter().find(|(c,id,..)| Some((*c,*id)) == self.destination).map_or("Choose destination", |d| d.2.as_str())).show_ui(ui, |ui| {
                for (campaign, id, label, ..) in &destinations { ui.selectable_value(&mut self.destination, Some((*campaign,*id)), label); }
            });
            ui.label("Copy name"); let name_changed = ui.text_edit_singleline(&mut self.name).changed();
            ui.label("Existing subfolder (leave blank for the owner folder)"); let folder_changed = ui.text_edit_singleline(&mut self.folder).changed();
            if old != self.destination || name_changed || folder_changed { self.plan = None; self.error = None; }
            if ui.add_enabled(self.catalogue.is_some() && self.destination.is_some(), egui::Button::new("Review copy")).clicked()
                && let Some(destination) = destinations.iter().find(|(c,id,..)| Some((*c,*id)) == self.destination) {
                    let result = check_destination(&destination.3, destination.1, destination.0).and_then(|_| CopyPlan::prepare(&self.source, &destination.3, destination.4.as_ref().map(|c| c.id), &self.name, &self.folder, self.catalogue.as_ref().unwrap()));
                    match result {
                        Ok(plan) => { self.plan = Some(plan); self.error = None; }, Err(e) => self.error = Some(e),
                    }
            }
            if let Some(plan) = &self.plan {
                ui.separator();
                ui.strong("Ready to copy");
                ui.label(plan.root.join(&plan.relative).display().to_string());
                if plan.dependencies.is_empty() { ui.label("No required content dependencies."); }
                else { ui.collapsing(format!("Dependencies kept as references: {}", plan.dependencies.len()), |ui| { for d in &plan.dependencies { ui.label(d); } }); }
                ui.collapsing("Review Markdown", |ui| { egui::ScrollArea::both().max_height(200.0).show(ui, |ui| { ui.monospace(&plan.text); }); });
                if ui.button("Create copy").clicked() {
                    let result = destinations.iter().find(|(c,id,..)| Some((*c,*id)) == self.destination).ok_or_else(|| "Destination is no longer registered.".to_owned()).and_then(|d| {
                        check_destination(&d.3, d.1, d.0)?;
                        if d.3 != plan.root { return Err("Destination folder changed. Review the copy again.".into()); }
                        plan.save()
                    });
                    match result { Ok(_) => { self.completed = Some(format!("Saved: {}", plan.relative)); self.refresh = Some(plan.root.clone()); self.error = None; }, Err(e) => self.error = Some(e) }
                }
            }
        });
        open && !close
    }
}
