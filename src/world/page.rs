//! The World page: what is true in this world. One list of notes and entries, and the Reader.
use super::{Nav, Reader};
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
    pub run_open: bool,
    new_kind: Option<&'static str>,
    new_name: String,
    note_type: String,
    repair: Option<super::repair::Repair>,
    kept_repairs: Vec<super::repair::Repair>,
    campaign: Option<Uuid>,
    generated: Option<crate::npc_generator::Generated>,
    generator_problems: Vec<String>,
    generator_sources: Vec<String>,
    error: Option<String>,
    repaired: Option<Uuid>,
}

impl WorldPage {
    pub fn dirty(&self) -> bool {
        self.repair
            .as_ref()
            .is_some_and(super::repair::Repair::dirty)
            || self.kept_repairs.iter().any(super::repair::Repair::dirty)
    }

    fn keep_repair(&mut self) {
        if let Some(repair) = self.repair.take() {
            self.kept_repairs.push(repair);
        }
    }

    fn set_campaign(&mut self, id: Uuid) {
        if self.campaign != Some(id) {
            self.campaign = Some(id);
            self.selected = None;
            self.repair = None;
            self.kept_repairs.clear();
            self.reader = Reader::default();
        }
    }

    pub fn reveal(&mut self, campaign: Uuid, id: Uuid) {
        self.set_campaign(campaign);
        self.selected = Some(id);
        self.keep_repair();
        self.new_kind = None;
        self.generated = None;
        self.error = None;
    }

    pub fn open_repair(&mut self, campaign: &Campaign, path: &str) {
        self.set_campaign(campaign.id);
        self.keep_repair();
        let loaded = if let Some(at) = self.kept_repairs.iter().position(|r| r.path == path) {
            Ok(self.kept_repairs.remove(at))
        } else {
            super::repair::Repair::open(campaign, path)
        };
        match loaded {
            Ok(repair) => {
                self.repair = Some(repair);
                self.error = None;
                self.new_kind = None;
                self.generated = None;
            }
            Err(error) => self.error = Some(error),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        index: Option<&Arc<ContentIndex>>,
        campaign: &Campaign,
    ) {
        self.set_campaign(campaign.id);
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("reader_surface"), egui::Id::new("world"))
        });
        let broken = index.map_or(0, |index| index.broken_world.len());
        let count = index.map_or(0, |index| {
            index.world_groups("").values().map(Vec::len).sum::<usize>() + broken
        });
        let broken_links = index.map_or(0, |index| {
            index
                .world_groups("")
                .values()
                .flatten()
                .filter(|(at, _)| {
                    !super::broken_links(&index.items[*at], index).is_empty()
                        || index
                            .duplicate_world
                            .contains_key(&index.items[*at].entry.path)
                        || !index.items[*at].entry.flags.is_empty()
                        || crate::roll_table::fault_count(&index.items[*at].entry.body) > 0
                })
                .count()
        });
        let mut create = None;
        design::page_header(
            ui,
            "World",
            (count > 0)
                .then(|| design::count(count, "note", "notes"))
                .as_deref(),
            |ui| {
                create = design::locked(
                    ui,
                    index
                        .is_none()
                        .then_some("Wait for the notes to load before creating a note."),
                    new_menu,
                );
                if broken_links > 0 {
                    let response = design::status_link(
                        ui,
                        design::Status {
                            icon: icons::WARN,
                            word: &design::count(
                                broken_links,
                                "note needs fixing",
                                "notes need fixing",
                            ),
                            color: colors().result.warning,
                        },
                    );
                    egui::Popup::menu(&response).show(|ui| {
                        if let Some(index) = index {
                            for &(at, _) in index.world_groups("").values().flatten() {
                                let item = &index.items[at];
                                if (!super::broken_links(item, index).is_empty()
                                    || index.duplicate_world.contains_key(&item.entry.path)
                                    || !item.entry.flags.is_empty()
                                    || crate::roll_table::fault_count(&item.entry.body) > 0)
                                    && design::link(ui, &item.entry.name).clicked()
                                {
                                    self.selected = Some(item.entry.id);
                                    ui.close();
                                }
                            }
                        }
                    });
                }
                if broken > 0 {
                    design::status(
                        ui,
                        design::Status {
                            icon: icons::WARN,
                            word: &format!(
                                "{} cannot be read",
                                design::count(broken, "note", "notes")
                            ),
                            color: colors().result.warning,
                        },
                    );
                }
            },
        );
        if let Some(kind) = create.take() {
            self.begin_new(kind, index.map(AsRef::as_ref));
        }
        let Some(index) = index else {
            ui.horizontal(|ui| {
                design::spinner(ui);
                ui.label(
                    Type::Small
                        .rich(ui, "Wait for the notes to load before creating a note.")
                        .color(colors().muted),
                );
            });
            return;
        };
        let groups = index.world_groups(&self.search.trim().to_lowercase());
        if broken == 0
            && groups.is_empty()
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
                    "Find a note",
                    width,
                );
                egui::ScrollArea::vertical()
                    .id_salt("world_list")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if groups.is_empty() && index.broken_world.is_empty() {
                            ui.label(
                                Type::Body
                                    .rich(ui, "Nothing here matches.")
                                    .color(colors().muted),
                            );
                        }
                        self.list(ui, index, &groups);
                        let broken: Vec<_> = index
                            .broken_world
                            .iter()
                            .filter(|(path, _)| {
                                path.to_lowercase().contains(&self.search.to_lowercase())
                            })
                            .collect();
                        if !broken.is_empty() {
                            design::group_heading(ui, "Cannot be read", broken.len());
                            for (path, problem) in broken {
                                let response = design::two_line_row(
                                    ui,
                                    ROW,
                                    self.repair.as_ref().is_some_and(|r| r.path == *path),
                                    path,
                                    if crate::formats_ui::is_old(problem) {
                                        "Old layout"
                                    } else {
                                        "Cannot be read"
                                    },
                                )
                                .on_hover_text(problem);
                                if response.clicked() {
                                    self.open_repair(campaign, path);
                                }
                            }
                        }
                        let drafts: Vec<_> = self
                            .kept_repairs
                            .iter()
                            .filter(|r| {
                                r.dirty()
                                    && !index.broken_world.iter().any(|(path, _)| path == &r.path)
                            })
                            .map(|r| r.path.clone())
                            .collect();
                        if !drafts.is_empty() {
                            design::group_heading(ui, "Repair drafts", drafts.len());
                            for path in drafts {
                                if design::two_line_row(ui, ROW, false, &path, "Unsaved repair")
                                    .clicked()
                                {
                                    self.open_repair(campaign, &path);
                                }
                            }
                        }
                    });
            });
        let mut nav = None;
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                if let Some(repair) = &mut self.repair {
                    if repair.show(ui, campaign) {
                        self.selected = repair.entry_id();
                        self.repaired = self.selected;
                        self.repair = None;
                        self.reload = true;
                    }
                    return;
                }
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
                        "No note open",
                        "Choose one from the list, or make a new one.",
                        |_| {},
                    );
                    return;
                };
                egui::ScrollArea::vertical()
                    .id_salt(("world_reader", item.entry.id))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.repaired == Some(item.entry.id) {
                            design::status(
                                ui,
                                design::Status {
                                    icon: icons::OK,
                                    word: "Note repaired.",
                                    color: colors().document.saved,
                                },
                            );
                        }
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
        self.keep_repair();
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
            self.generator_sources = tables.sources;
            self.generator_problems = tables.problems;
        }
    }

    fn creation_form(&mut self, ui: &mut egui::Ui, index: &ContentIndex, campaign: &Campaign) {
        let mut made = None;
        if let Some(kind) = self.new_kind {
            design::group_label(
                ui,
                match kind {
                    "npc" => "New NPC",
                    "location" => "New location",
                    _ => "New note",
                },
            );
            design::text_field(ui, "Name", &mut self.new_name, None, None);
            if kind == "npc" && !self.new_name.trim().is_empty() {
                ui.label(
                    Type::Small
                        .rich(
                            ui,
                            format!(
                                "Will create NPCs/{}.md",
                                crate::sessions::file_safe(&self.new_name)
                            ),
                        )
                        .color(colors().muted),
                );
            }
            if kind == "note" {
                design::text_field(
                    ui,
                    "Type (optional)",
                    &mut self.note_type,
                    Some("Leave empty for a plain note. For example: faction or quest."),
                    None,
                );
            }
            ui.horizontal(|ui| {
                let reason = self
                    .new_name
                    .trim()
                    .is_empty()
                    .then_some("Type a name first.");
                if (if self.run_open {
                    design::Action::secondary(match kind {
                        "npc" => "Create NPC",
                        "location" => "Create location",
                        _ => "Create note",
                    })
                } else {
                    design::Action::primary(match kind {
                        "npc" => "Create NPC",
                        "location" => "Create location",
                        _ => "Create note",
                    })
                })
                .blocked(reason)
                .show(ui)
                .clicked()
                {
                    let kind = if kind == "note" {
                        self.note_type.trim()
                    } else {
                        kind
                    };
                    made = Some(super::create_entry(campaign, &self.new_name, kind));
                }
                design::blocked_reason(ui, reason);
                ui.add_space(design::L);
                if design::Action::quiet("Cancel").show(ui).clicked() {
                    self.new_kind = None;
                }
            });
        }
        if let Some(npc) = &self.generated {
            let (saved, again, discard) = crate::lookup::generated_card(
                ui,
                npc,
                &self.generator_problems,
                &self.generator_sources,
                self.run_open,
            );
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
                self.note_type.clear();
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
        for ((kind, title), rows) in groups {
            ui.add_space(design::XS);
            if *kind == 3 {
                design::folder_group_heading(ui, title, rows.len());
            } else {
                design::group_heading(ui, title, rows.len());
            }
            let spacing = std::mem::replace(&mut ui.spacing_mut().item_spacing.y, 0.0);
            for &(position, depth) in rows {
                let entry = &index.items[position].entry;
                let problems = super::broken_links(&index.items[position], index);
                let faults = crate::roll_table::fault_count(&entry.body);
                let duplicate = index.duplicate_world.contains_key(&entry.path);
                let name = if duplicate {
                    std::path::Path::new(&entry.path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                } else {
                    entry.name.clone()
                };
                let detail = if duplicate {
                    format!("{} Duplicate identity", icons::WARN)
                } else if !entry.flags.is_empty() {
                    format!(
                        "{} {}",
                        icons::WARN,
                        design::count(entry.flags.len(), "value not read", "values not read")
                    )
                } else if faults > 0 {
                    format!(
                        "{} {}",
                        icons::WARN,
                        design::count(faults, "table fault", "table faults")
                    )
                } else if !problems.is_empty() {
                    format!(
                        "{} {}",
                        icons::WARN,
                        if problems.len() > 1 {
                            "2 broken links"
                        } else {
                            problems[0]
                        }
                    )
                } else if entry.kind == "world" {
                    if index.items.iter().any(|other| {
                        other.entry.id != entry.id
                            && other.entry.kind == "world"
                            && other.entry.name == entry.name
                    }) {
                        let file = std::path::Path::new(&entry.path)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        match super::status(entry) {
                            "" => file,
                            status => format!("{file} · {status}"),
                        }
                    } else {
                        match super::status(entry) {
                            "" => String::new(),
                            status => status.to_owned(),
                        }
                    }
                } else {
                    String::new()
                };
                let selected = self.selected == Some(entry.id);
                ui.horizontal(|ui| {
                    ui.add_space(depth as f32 * design::S);
                    let response = if detail.is_empty() {
                        design::row(
                            ui,
                            design::Density::Read.row(),
                            selected,
                            &format!("{} · ", entry.name),
                            |ui, rect| {
                                let galley = design::one_line(
                                    ui,
                                    &entry.name,
                                    Type::Body.font(ui),
                                    colors().text,
                                    rect.width() - design::S * 2.0,
                                );
                                ui.painter().galley(
                                    egui::pos2(
                                        rect.left() + design::S,
                                        rect.center().y - galley.size().y / 2.0,
                                    ),
                                    galley,
                                    colors().text,
                                );
                            },
                        )
                    } else {
                        design::two_line_row(ui, ROW, selected, &name, &detail)
                            .on_hover_text(&entry.path)
                    };
                    if response.clicked() {
                        self.selected = Some(entry.id);
                        self.keep_repair();
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
    let label = format!("New {}", icons::DOWN);
    let response = design::Action::secondary(&label)
        .name("New ▾")
        .icon(icons::ADD)
        .show(ui);
    egui::Popup::menu(&response).show(|ui| {
        for (word, kind) in [
            ("Note…", "note"),
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
