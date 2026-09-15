use crate::persistence::{Project, Store, UserData};
use eframe::egui;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Projects {
    store: Option<Store>,
    data: UserData,
    error: Option<String>,
    writable: bool,
    editing: Option<Uuid>,
    name: String,
    path: String,
}

impl Default for Projects {
    fn default() -> Self {
        let mut result = Self {
            store: None,
            data: UserData::default(),
            error: None,
            writable: false,
            editing: None,
            name: String::new(),
            path: String::new(),
        };
        match Store::user() {
            Ok(store) => {
                result.store = Some(store);
                result.reload();
            }
            Err(error) => result.error = Some(error),
        }
        result
    }
}

impl Projects {
    fn reload(&mut self) {
        if let Some(store) = &mut self.store {
            match store.load() {
                Ok(data) => {
                    self.data = data;
                    self.error = None;
                    self.writable = true;
                    self.clear_form();
                }
                Err(error) => {
                    self.error = Some(error);
                    self.writable = false;
                }
            }
        }
    }

    fn save(&mut self, data: UserData) {
        if let Some(store) = &mut self.store {
            match store.save(&data) {
                Ok(()) => {
                    self.data = data;
                    self.error = None;
                    self.clear_form();
                }
                Err(error) => self.error = Some(error),
            }
        }
    }

    fn clear_form(&mut self) {
        self.editing = None;
        self.name.clear();
        self.path.clear();
    }

    pub fn active_name(&self) -> Option<&str> {
        self.data
            .projects
            .iter()
            .find(|p| Some(p.id) == self.data.active_project_id)
            .map(|p| p.name.as_str())
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Projects");
        ui.label("Register project folders here. Removing an entry does not delete its files.");
        if let Some(store) = &self.store {
            ui.weak(store.path.display().to_string());
        }
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        if ui.button("Reload project list").clicked() {
            self.reload();
        }
        ui.add_enabled_ui(self.writable, |ui| {
            for project in self.data.projects.clone() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(
                                self.data.active_project_id == Some(project.id),
                                &project.name,
                            )
                            .clicked()
                        {
                            let mut data = self.data.clone();
                            data.active_project_id = Some(project.id);
                            self.save(data);
                        }
                        if ui.button("Edit").clicked() {
                            self.editing = Some(project.id);
                            self.name = project.name.clone();
                            self.path = project.path.to_string_lossy().into_owned();
                        }
                        if ui.button("Remove").clicked() {
                            let mut data = self.data.clone();
                            data.projects.retain(|p| p.id != project.id);
                            if data.active_project_id == Some(project.id) {
                                data.active_project_id = None;
                            }
                            self.save(data);
                        }
                    });
                    ui.label(project.path.display().to_string());
                    if !project.path.is_dir() {
                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "Folder not available. Edit the path to locate it again.",
                        );
                    }
                });
            }
            ui.add_space(8.0);
            ui.label(if self.editing.is_some() {
                "Edit project"
            } else {
                "Add project"
            });
            ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Project name"));
            ui.add(
                egui::TextEdit::singleline(&mut self.path)
                    .hint_text("Absolute folder path")
                    .desired_width(500.0),
            );
            ui.horizontal(|ui| {
                if ui.button("Save project").clicked() {
                    let path = PathBuf::from(self.path.trim());
                    if self.name.trim().is_empty() || !path.is_absolute() {
                        self.error =
                            Some("Enter a project name and an absolute folder path.".into());
                    } else {
                        let mut data = self.data.clone();
                        if let Some(project) = data
                            .projects
                            .iter_mut()
                            .find(|p| Some(p.id) == self.editing)
                        {
                            project.name = self.name.trim().into();
                            project.path = path;
                        } else {
                            let id = Uuid::new_v4();
                            data.projects.push(Project {
                                id,
                                name: self.name.trim().into(),
                                path,
                            });
                            if data.active_project_id.is_none() {
                                data.active_project_id = Some(id);
                            }
                        }
                        self.save(data);
                    }
                }
                if self.editing.is_some() && ui.button("Cancel edit").clicked() {
                    self.clear_form();
                }
            });
        });
    }
}
