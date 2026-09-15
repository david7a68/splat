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

    fn persist(&mut self, data: UserData) -> Result<(), String> {
        let store = self.store.as_mut().ok_or("Settings are unavailable.")?;
        store.save(&data)?;
        self.data = data;
        self.error = None;
        Ok(())
    }

    fn save(&mut self, data: UserData) {
        match self.persist(data) {
            Ok(()) => self.clear_form(),
            Err(error) => self.error = Some(error),
        }
    }

    pub fn link_campaigns(&mut self, ids: &[String]) -> Result<(), String> {
        if self.data.active_project_ids.is_empty() {
            return Err("Select a local project.".into());
        }
        let mut data = self.data.clone();
        for project in &mut data.projects {
            if data.active_project_ids.contains(&project.id) {
                for id in ids {
                    if !project.dndbeyond_campaign_ids.contains(id) {
                        project.dndbeyond_campaign_ids.push(id.clone());
                    }
                }
            }
        }
        self.persist(data)
    }

    pub fn unlink_campaign(&mut self, project_id: Uuid, campaign_id: &str) -> Result<(), String> {
        let mut data = self.data.clone();
        if let Some(project) = data.projects.iter_mut().find(|p| p.id == project_id) {
            project
                .dndbeyond_campaign_ids
                .retain(|id| id != campaign_id);
        }
        self.persist(data)
    }

    fn clear_form(&mut self) {
        self.editing = None;
        self.name.clear();
        self.path.clear();
    }

    pub fn active_projects(&self) -> Vec<Project> {
        self.data
            .projects
            .iter()
            .filter(|p| self.data.active_project_ids.contains(&p.id))
            .cloned()
            .collect()
    }

    pub fn selector(&mut self, ui: &mut egui::Ui) {
        let current = self.data.active_project_ids.first().copied();
        let mut selected = current;
        let label = self
            .data
            .projects
            .iter()
            .find(|p| Some(p.id) == current)
            .map(|p| p.name.as_str())
            .unwrap_or("Select project");
        ui.add_enabled_ui(self.writable, |ui| {
            egui::ComboBox::from_id_salt("active_project")
                .selected_text(label)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, None, "No project");
                    for project in &self.data.projects {
                        ui.selectable_value(&mut selected, Some(project.id), &project.name);
                    }
                });
        });
        if selected != current {
            self.select(selected);
        }
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
    }

    fn select(&mut self, id: Option<Uuid>) {
        let mut data = self.data.clone();
        data.active_project_ids = id.into_iter().collect();
        if let Err(error) = self.persist(data) {
            self.error = Some(error);
        }
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
                                self.data.active_project_ids.contains(&project.id),
                                &project.name,
                            )
                            .clicked()
                        {
                            self.select(Some(project.id));
                        }
                        if ui.button("Edit").clicked() {
                            self.editing = Some(project.id);
                            self.name = project.name.clone();
                            self.path = project.path.to_string_lossy().into_owned();
                        }
                        if ui.button("Remove").clicked() {
                            let mut data = self.data.clone();
                            data.projects.retain(|p| p.id != project.id);
                            data.active_project_ids.retain(|id| *id != project.id);
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
                                dndbeyond_campaign_ids: Vec::new(),
                            });
                            data.active_project_ids = vec![id];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switching_projects_keeps_campaign_links_independent() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let mut store = Store::at(path.clone());
        let mut data = store.load().unwrap();
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        for id in [first, second] {
            data.projects.push(Project {
                id,
                name: "Local project".into(),
                path: dir.path().into(),
                dndbeyond_campaign_ids: Vec::new(),
            });
        }
        let mut projects = Projects {
            store: Some(store),
            data,
            writable: true,
            editing: None,
            name: String::new(),
            path: String::new(),
            error: None,
        };
        projects.select(Some(first));
        projects
            .link_campaigns(&["12".into(), "34".into()])
            .unwrap();
        projects.select(Some(second));
        assert_eq!(projects.active_projects().len(), 1);
        assert_eq!(projects.active_projects()[0].id, second);
        projects
            .link_campaigns(&["12".into(), "34".into()])
            .unwrap();
        projects.link_campaigns(&["12".into()]).unwrap();
        projects.unlink_campaign(first, "12").unwrap();

        let data = Store::at(path.clone()).load().unwrap();
        assert_eq!(data.active_project_ids, vec![second]);
        assert_eq!(data.projects[0].dndbeyond_campaign_ids, vec!["34"]);
        assert_eq!(data.projects[1].dndbeyond_campaign_ids, vec!["12", "34"]);
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let keys = saved["projects"][0].as_object().unwrap();
        assert_eq!(keys.len(), 4); // Local id, name, path, and campaign IDs only.
        assert!(keys.contains_key("dndbeyond_campaign_ids"));
        projects.select(None);
        assert!(projects.link_campaigns(&["56".into()]).is_err());
    }
}
