//! The World page: what is true in this world. One list of notes and entries, and the Reader.
use super::{Nav, Reader, world_type};
use crate::{
    design::{self, Type, colors, icons},
    lookup::{ContentIndex, WorldGroups},
    persistence::Campaign,
};
use eframe::egui;
use std::sync::Arc;
use uuid::Uuid;

const ROW: f32 = 40.0;
const LIST_WIDTH: f32 = 260.0;

#[derive(Default)]
pub struct WorldPage {
    search: String,
    selected: Option<Uuid>,
    pub reader: Reader,
    /// What only the app can do, for it to take after the page draws.
    pub ask_lookup: Option<String>,
    pub reload: bool,
}

impl WorldPage {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        index: Option<&Arc<ContentIndex>>,
        campaign: &Campaign,
    ) {
        design::page_header(ui, "World", Some(&campaign.name), |_| {});
        let Some(index) = index else {
            ui.horizontal(|ui| {
                design::spinner(ui);
                ui.label(
                    Type::Body
                        .rich(ui, "Reading the notes…")
                        .color(colors().muted),
                );
            });
            return;
        };
        let groups = index.world_groups(&self.search.trim().to_lowercase());
        if groups.is_empty() && self.search.trim().is_empty() {
            design::empty_state(
                ui,
                icons::WORLD,
                "No world notes yet",
                "Markdown notes in this campaign's folder appear here, with no change to the files.",
                |_| {},
            );
            return;
        }
        design::Column::left("world_list_panel", LIST_WIDTH, 190.0..=480.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                let width = ui.available_width();
                design::search_field(
                    ui,
                    "Search the world",
                    &mut self.search,
                    "Search notes and entries",
                    width,
                );
                egui::ScrollArea::vertical()
                    .id_salt("world_list")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if groups.is_empty() {
                            ui.label(
                                Type::Body
                                    .rich(ui, "Nothing here matches.")
                                    .color(colors().muted),
                            );
                        }
                        self.list(ui, index, &groups);
                    });
            });
        let mut nav = None;
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                nav = self.reader.choices(ui, index);
                let Some(item) = self
                    .selected
                    .and_then(|id| index.position(id))
                    .map(|position| &index.items[position])
                else {
                    ui.label(
                        Type::Body
                            .rich(ui, "Choose a note to read it.")
                            .color(colors().muted),
                    );
                    return;
                };
                egui::ScrollArea::vertical()
                    .id_salt(("world_reader", item.entry.id))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let read = self.reader.read(ui, item, index, Some(campaign));
                        nav = nav.take().or(read);
                    });
            });
        match nav {
            Some(Nav::Item(position)) => self.selected = Some(index.items[position].entry.id),
            Some(Nav::Missing(words)) => self.ask_lookup = Some(words),
            Some(Nav::Reload(id)) => {
                self.reload = true;
                self.selected = Some(id);
            }
            None => {}
        }
    }

    fn list(&mut self, ui: &mut egui::Ui, index: &ContentIndex, groups: &WorldGroups) {
        for ((_, title), rows) in groups {
            ui.add_space(design::XS);
            ui.label(Type::StatLabel.rich(ui, title).color(colors().muted));
            let spacing = std::mem::replace(&mut ui.spacing_mut().item_spacing.y, 0.0);
            for &(position, depth) in rows {
                let entry = &index.items[position].entry;
                let detail = if entry.kind == "world" {
                    match super::status(entry) {
                        "" => super::type_label(world_type(entry)),
                        status => status.to_owned(),
                    }
                } else {
                    entry.path.clone()
                };
                let selected = self.selected == Some(entry.id);
                ui.horizontal(|ui| {
                    ui.add_space(depth as f32 * design::S);
                    if design::two_line_row(ui, ROW, selected, &entry.name, &detail).clicked() {
                        self.selected = Some(entry.id);
                    }
                });
            }
            ui.spacing_mut().item_spacing.y = spacing;
        }
    }
}
