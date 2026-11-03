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
    new_kind: Option<&'static str>,
    new_name: String,
    generated: Option<crate::npc_generator::Generated>,
    generator_problems: Vec<String>,
    error: Option<String>,
}

impl WorldPage {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        index: Option<&Arc<ContentIndex>>,
        campaign: &Campaign,
    ) {
        let count = index.map_or(0, |index| {
            index.world_groups("").values().map(Vec::len).sum()
        });
        let mut create = None;
        design::page_header(
            ui,
            "World",
            Some(&design::count(count, "note", "notes")),
            |ui| {
                create = new_menu(ui);
            },
        );
        if let Some(kind) = create.take() {
            self.begin_new(kind, index.map(AsRef::as_ref));
        }
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
        if groups.is_empty()
            && self.search.trim().is_empty()
            && self.new_kind.is_none()
            && self.generated.is_none()
            && self.error.is_none()
        {
            design::empty_state(
                ui,
                icons::WORLD,
                "No world notes yet",
                "World notes are the Markdown files in this campaign's World folder.",
                |ui| {
                    create = new_menu(ui);
                },
            );
            if let Some(kind) = create.take() {
                self.begin_new(kind, Some(index));
            }
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
                if self.new_kind.is_some() || self.generated.is_some() || self.error.is_some() {
                    self.creation_form(ui, index, campaign);
                    return;
                }
                nav = self.reader.choices(ui, index);
                let Some(item) = self
                    .selected
                    .and_then(|id| index.position(id))
                    .map(|position| &index.items[position])
                else {
                    design::empty_state(
                        ui,
                        icons::WORLD,
                        "Read the world notes",
                        "World notes are the Markdown files in this campaign's World folder.",
                        |ui| {
                            create = new_menu(ui);
                        },
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
        if let Some(kind) = create.take() {
            self.begin_new(kind, Some(index));
        }
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

    fn begin_new(&mut self, kind: &'static str, index: Option<&ContentIndex>) {
        self.error = None;
        self.new_kind = (kind != "generate").then_some(kind);
        self.generated = None;
        if kind == "generate"
            && let Some(index) = index
        {
            let tables = index.npc_tables();
            match crate::npc_generator::roll(&tables) {
                Ok(npc) => self.generated = Some(npc),
                Err(error) => self.error = Some(error),
            }
            self.generator_problems = tables.problems;
        }
    }

    fn creation_form(&mut self, ui: &mut egui::Ui, index: &ContentIndex, campaign: &Campaign) {
        let mut made = None;
        if let Some(kind) = self.new_kind {
            design::group_label(
                ui,
                if kind == "npc" {
                    "New NPC"
                } else {
                    "New location"
                },
            );
            design::text_field(ui, "Name", &mut self.new_name, None, None);
            ui.horizontal(|ui| {
                let reason = self
                    .new_name
                    .trim()
                    .is_empty()
                    .then_some("Type a name first.");
                if design::Action::primary("Create note")
                    .blocked(reason)
                    .show(ui)
                    .clicked()
                {
                    made = Some(super::create_entry(campaign, &self.new_name, kind));
                }
                design::blocked_reason(ui, reason);
                if design::Action::quiet("Cancel").show(ui).clicked() {
                    self.new_kind = None;
                }
            });
        }
        if let Some(npc) = &self.generated {
            let (saved, again, discard) =
                crate::lookup::generated_card(ui, npc, &self.generator_problems);
            if saved {
                made = Some(super::create_generated(campaign, npc));
            }
            if again {
                self.begin_new("generate", Some(index));
            }
            if discard {
                self.generated = None;
            }
        }
        match made {
            Some(Ok(id)) => {
                self.selected = Some(id);
                self.new_kind = None;
                self.generated = None;
                self.new_name.clear();
                self.error = None;
                self.reload = true;
            }
            Some(Err(error)) => self.error = Some(error),
            None => {}
        }
        if let Some(error) = &self.error {
            design::message(ui, design::Tone::Error, error);
            if self.new_kind.is_none()
                && self.generated.is_none()
                && design::Action::quiet("Cancel").show(ui).clicked()
            {
                self.error = None;
            }
        }
    }

    fn list(&mut self, ui: &mut egui::Ui, index: &ContentIndex, groups: &WorldGroups) {
        for ((_, title), rows) in groups {
            ui.add_space(design::XS);
            design::group_heading(ui, title, rows.len());
            let spacing = std::mem::replace(&mut ui.spacing_mut().item_spacing.y, 0.0);
            for &(position, depth) in rows {
                let entry = &index.items[position].entry;
                let detail = if entry.kind == "world" {
                    match super::status(entry) {
                        "" => super::type_label(world_type(entry)),
                        status => status.to_owned(),
                    }
                } else {
                    String::new()
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

/// The same next step in the header and in an empty reader.
fn new_menu(ui: &mut egui::Ui) -> Option<&'static str> {
    let mut choice = None;
    let response = design::Action::secondary("New ▾").icon(icons::ADD).show(ui);
    egui::Popup::menu(&response).show(|ui| {
        for (word, kind) in [
            ("NPC", "npc"),
            ("Location", "location"),
            ("Generate NPC", "generate"),
        ] {
            if ui.button(word).clicked() {
                choice = Some(kind);
                ui.close();
            }
        }
    });
    choice
}
