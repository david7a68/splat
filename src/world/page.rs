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
/// More rows than this in one check are counted, not shown.
const CHECK_ROWS: usize = 40;

/// A repair cause must stay whole even when Lookup narrows the list.
fn broken_row(ui: &mut egui::Ui, selected: bool, path: &str, cause: &str) -> egui::Response {
    let width = (ui.available_width() - design::S * 2.0).max(design::XXL);
    let font = if selected { Type::Strong } else { Type::Body };
    let title = design::one_line(ui, path, font.font(ui), colors().text, width);
    let detail = ui.painter().layout(
        cause.to_owned(),
        Type::Small.font(ui),
        colors().muted,
        width,
    );
    let height = (title.size().y + detail.size().y + design::S).max(ROW);
    design::row(
        ui,
        height,
        selected,
        &format!("{path} · {cause}"),
        |ui, rect| {
            let position = rect.min + egui::vec2(design::S, design::XS);
            ui.painter().galley(position, title.clone(), colors().text);
            ui.painter().galley(
                position + egui::vec2(0.0, title.size().y),
                detail,
                colors().muted,
            );
        },
    )
}

/// What the page's larger part shows.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum View {
    #[default]
    Reader,
    Questions,
    Problems,
}

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
    generator_problems: Vec<crate::npc_generator::TableProblem>,
    generator_sources: Vec<String>,
    error: Option<String>,
    repaired: Option<Uuid>,
    view: View,
    /// The checks for one load of the index.
    checks: Option<(u64, super::checks::Checks)>,
    /// Repair causes are read and parsed only when the index generation changes.
    broken_details: std::collections::BTreeMap<String, String>,
}

impl WorldPage {
    /// All file reads and repair parsing stay behind this generation check.
    fn refresh_checks(
        &mut self,
        index: &ContentIndex,
        mut read: impl FnMut(&str) -> Option<String>,
    ) {
        if self
            .checks
            .as_ref()
            .is_some_and(|(generation, _)| *generation == index.generation())
        {
            return;
        }
        self.checks = Some((index.generation(), super::checks::run(index)));
        self.broken_details = index
            .broken_world
            .iter()
            .map(|(path, problem)| {
                let detail = read(path)
                    .and_then(|text| crate::campaign_content::repair_problem(path, &text).0)
                    .unwrap_or_else(|| problem.clone());
                (path.clone(), detail)
            })
            .collect();
    }

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
        self.view = View::Reader;
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
                if broken > 0
                    && design::status_link(
                        ui,
                        design::Status {
                            icon: icons::WARN,
                            word: &format!(
                                "{} cannot be read",
                                design::count(broken, "note", "notes")
                            ),
                            color: colors().result.warning,
                        },
                    )
                    .clicked()
                    && let Some((path, _)) = index.and_then(|index| index.broken_world.first())
                {
                    self.search.clear();
                    self.open_repair(campaign, path);
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
        self.refresh_checks(index, |path| {
            crate::library::note_path(&campaign.path, path)
                .ok()
                .and_then(|path| std::fs::read_to_string(path).ok())
        });
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
                        if self.search.trim().is_empty() {
                            self.check_rows(ui);
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
                            ui.add_space(design::XS);
                            design::group_heading(ui, "Cannot be read", broken.len());
                            for (path, problem) in broken {
                                let response = broken_row(
                                    ui,
                                    self.repair.as_ref().is_some_and(|r| r.path == *path),
                                    path,
                                    if crate::formats_ui::is_old(problem) {
                                        "Old layout"
                                    } else {
                                        self.broken_details
                                            .get(path)
                                            .map_or(problem.as_str(), String::as_str)
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
                        self.view = View::Reader;
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
                if self.view != View::Reader {
                    let opened = egui::ScrollArea::vertical()
                        .id_salt(("world_checks", self.view == View::Questions))
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            design::reader::measure(ui, |ui| self.checks_view(ui, index))
                        })
                        .inner;
                    if let Some((page, heading)) = opened {
                        self.selected = Some(index.items[page].entry.id);
                        self.view = View::Reader;
                        self.reader.show_heading(heading);
                    }
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
            Some(Nav::Item(position)) => {
                self.selected = Some(index.items[position].entry.id);
                self.view = View::Reader;
            }
            Some(Nav::Missing(words)) => self.ask_lookup = Some(words),
            Some(Nav::Reload(id)) => {
                self.reload = true;
                self.selected = Some(id);
            }
            None => {}
        }
    }

    fn begin_new(&mut self, kind: &'static str, index: Option<&ContentIndex>) {
        self.view = View::Reader;
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
                let folder = match index.type_folder("npc") {
                    Some(folder) if folder.is_empty() => folder,
                    Some(folder) => format!("{folder}/"),
                    None => "NPCs/".into(),
                };
                ui.label(
                    Type::Small
                        .rich(
                            ui,
                            format!(
                                "Will create {folder}{}.md",
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

    /// The two lists of what the notes still need, above the notes themselves.
    fn check_rows(&mut self, ui: &mut egui::Ui) {
        let Some((_, checks)) = &self.checks else {
            return;
        };
        let rows = [
            (
                View::Questions,
                "Open questions",
                design::count(checks.questions.len(), "question", "questions"),
                checks.questions.len(),
            ),
            (
                View::Problems,
                "Problems",
                design::count(checks.problems(), "thing to check", "things to check"),
                checks.problems(),
            ),
        ];
        for (view, title, detail, count) in rows {
            if count > 0
                && design::two_line_row(ui, ROW, self.view == view, title, &detail).clicked()
            {
                self.view = view;
                self.keep_repair();
                self.new_kind = None;
                self.generated = None;
                self.error = None;
            }
        }
    }

    /// The open questions, or the problems. Returns the page a press asks for, with the
    /// heading to show.
    fn checks_view(
        &mut self,
        ui: &mut egui::Ui,
        index: &ContentIndex,
    ) -> Option<(usize, Option<String>)> {
        let (_, checks) = self.checks.as_ref()?;
        let mut open = None;
        let note = |ui: &mut egui::Ui, words: &str| {
            ui.add(egui::Label::new(Type::Small.rich(ui, words).color(colors().muted)).wrap());
        };
        let more = |ui: &mut egui::Ui, count: usize| {
            if count > CHECK_ROWS {
                note(ui, &design::count(count - CHECK_ROWS, "more", "more"));
            }
        };
        // One page's lines under its name, as the reader lists lines from other pages.
        let mut by_page = |ui: &mut egui::Ui, found: &[super::checks::Found]| {
            for group in found.chunk_by(|a, b| a.page == b.page).take(CHECK_ROWS) {
                ui.push_id(group[0].page, |ui| {
                    if design::link(ui, &index.items[group[0].page].entry.name).clicked() {
                        open = Some((group[0].page, None));
                    }
                    for line in group {
                        if design::line_link(ui, "", &line.words).clicked() {
                            open = Some((line.page, line.heading.clone()));
                        }
                    }
                });
            }
            more(ui, found.chunk_by(|a, b| a.page == b.page).count());
        };
        if self.view == View::Questions {
            design::reader::title(ui, "Open questions");
            design::reader::subtitle(
                ui,
                "Each unticked task line in the world's notes. Tick it in the note when you decide.",
            );
            ui.add_space(design::S);
            by_page(ui, &checks.questions);
            return open;
        }
        design::reader::title(ui, "Problems");
        design::reader::subtitle(
            ui,
            "Where a fact has no home, or has two. Splat only lists them. You change the notes.",
        );
        if !checks.loose_dates.is_empty() {
            design::reader::section_heading(ui, "Dates outside events");
            note(
                ui,
                "A date in words belongs on an event's page. An event has a date that Splat can put in order.",
            );
            by_page(ui, &checks.loose_dates);
        }
        let mut pages = |ui: &mut egui::Ui, pages: &[usize]| {
            for &page in pages {
                if design::link(ui, &index.items[page].entry.name).clicked() {
                    open = Some((page, None));
                }
            }
        };
        if !checks.wanted.is_empty() {
            design::reader::section_heading(ui, "Pages to write");
            note(
                ui,
                "A link names each of these, and no page has the name. The page that most lines want is first.",
            );
            for (name, lines, from) in checks.wanted.iter().take(CHECK_ROWS) {
                ui.horizontal_wrapped(|ui| {
                    ui.label(Type::Body.rich(ui, name).color(colors().text));
                    note(
                        ui,
                        &format!("{} in", design::count(*lines, "line", "lines")),
                    );
                    pages(ui, from);
                });
            }
            more(ui, checks.wanted.len());
        }
        for (title, words, found) in [
            (
                "No summary",
                "Start each of these with one paragraph that says what the subject is. Lists and Lookup show it.",
                &checks.no_summary,
            ),
            (
                "Nothing points here",
                "These entries show on no other page. No line names them, they name no other entry, and they are in no place and part of no group.",
                &checks.alone,
            ),
        ] {
            if !found.is_empty() {
                design::reader::section_heading(ui, title);
                note(ui, words);
                ui.horizontal_wrapped(|ui| pages(ui, &found[..found.len().min(CHECK_ROWS)]));
                more(ui, found.len());
            }
        }
        for (title, words, found) in [
            (
                "One name on two entries",
                "A line that uses the name shows on each of these pages.",
                &checks.shared_names,
            ),
            (
                "Said twice",
                "The same sentence is on each of these pages. Keep it on the page of its subject.",
                &checks.repeated,
            ),
        ] {
            if !found.is_empty() {
                design::reader::section_heading(ui, title);
                note(ui, words);
                for (said, on) in found.iter().take(CHECK_ROWS) {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(Type::Small.rich(ui, said).color(colors().text));
                        pages(ui, on);
                    });
                }
                more(ui, found.len());
            }
        }
        open
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
                    format!("{} Same entry in two files", icons::WARN)
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
                        [
                            super::date_words(entry),
                            Some(super::status(entry).to_owned()),
                        ]
                        .into_iter()
                        .flatten()
                        .filter(|word| !word.is_empty())
                        .collect::<Vec<_>>()
                        .join(" · ")
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
                        self.view = View::Reader;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repair_details_read_once_per_index_and_follow_recovery() {
        let mut first = ContentIndex::load(&[], None).unwrap();
        first
            .broken_world
            .push(("Broken.md".into(), "Original parser error".into()));
        let mut page = WorldPage::default();
        let mut reads = 0;
        let mut read = |_: &str| {
            reads += 1;
            Some("---\nsplat_schema: 2\nsplat_id: [\n---\n".into())
        };
        page.refresh_checks(&first, &mut read);
        page.refresh_checks(&first, &mut read);
        assert_eq!(
            reads, 1,
            "FD9-cache: the unchanged second render does no file read or repair parse"
        );
        assert!(page.broken_details["Broken.md"].contains("Line 3 opens a list"));

        // A changed index re-reads the note. Failure to read it uses its current indexed error.
        let mut unreadable = ContentIndex::load(&[], None).unwrap();
        unreadable
            .broken_world
            .push(("Broken.md".into(), "The file cannot be opened".into()));
        page.refresh_checks(&unreadable, |_| None);
        assert_eq!(
            page.broken_details["Broken.md"],
            "The file cannot be opened"
        );

        // A move, rename, deletion or repair removes the old row on the next index.
        let recovered = ContentIndex::load(&[], None).unwrap();
        page.refresh_checks(&recovered, |_| {
            panic!("A recovered index needs no repair reads")
        });
        assert!(
            page.broken_details.is_empty(),
            "FD9-recovery: no stale repair cause survives"
        );

        for body in [
            "| Field | Value |\n| --- | --- |\n| Name |\n",
            "| Field | Value |\n| --- | --- |\n| Name | Rook | extra |\n",
            "",
        ] {
            let mut index = ContentIndex::load(&[], None).unwrap();
            index
                .broken_world
                .push(("Broken.md".into(), "Indexed cause".into()));
            page.refresh_checks(&index, |_| Some(format!("---\nsplat_schema: 2\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Rook\nsplat_type: lore\n---\n{body}")));
            assert_eq!(
                page.broken_details["Broken.md"], "Indexed cause",
                "FD9-input: readable body syntax does not invent a front matter cause"
            );
        }
    }
}
