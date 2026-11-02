//! Lookup: search and read any entry beside the current page, without navigating.
//! It covers built-in, package and campaign-owned entries in one index (T2.3).
use crate::{
    campaigns::Campaigns,
    content_workspace::Catalogue,
    design::{self, Action, Density, Tone, Type, colors, icons, reader as style},
    library::{presentation, view},
    persistence::{Campaign, Module},
};
use eframe::egui;
use splat::content::{Entry, Reference};
use std::{
    collections::HashMap,
    sync::{Arc, mpsc},
};
use uuid::Uuid;

const RESULT_ROW: f32 = 40.0;
pub const SEARCH_NAME: &str = "Search everything";
/// How many lines of a long note show where the words were found.
const FOUND_LINES: usize = 3;
const FOUND_FROM_LINES: usize = 12;

/// Where something the DM wrote lives, so that it can open there.
#[derive(Clone, Debug, PartialEq)]
pub enum WrittenIn {
    Session(Uuid),
    Event(String),
}

/// Something the DM wrote, for searching beside the books: a session's note or a timeline event.
pub struct Written {
    pub title: String,
    pub detail: String,
    pub text: String,
    pub place: WrittenIn,
}

/// One line of the results.
#[derive(Clone, Copy, PartialEq)]
enum Hit {
    Entry(usize),
    Written(usize),
}

/// True if `name` appears in `text` as whole words, in any case.
pub fn names_word(text: &str, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let boundary = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric());
    let (text, name) = (text.to_lowercase(), name.to_lowercase());
    text.match_indices(&name).any(|(at, _)| {
        boundary(text[..at].chars().next_back()) && boundary(text[at + name.len()..].chars().next())
    })
}

pub struct Item {
    pub entry: Entry,
    lower: String,
    pub subtitle: String,
    pub owner: String,
    /// Written for this campaign, not from a package.
    private: bool,
    pub section: presentation::Section,
    builtin: bool,
    from_sheet: bool,
}

/// Every entry the active campaign can see, including all registered packages.
#[derive(Default)]
pub struct ContentIndex {
    pub items: Vec<Item>,
    by_id: HashMap<Uuid, usize>,
    by_key: HashMap<String, usize>,
    spells: Vec<usize>,
    pub errors: Vec<String>,
}

/// Among equal matches: the DM's own entry, then the book, then a sheet's copy.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Origin {
    Own,
    Book,
    Sheet,
}

impl Item {
    fn origin(&self) -> Origin {
        if self.from_sheet {
            Origin::Sheet
        } else if self.private {
            Origin::Own
        } else {
            Origin::Book
        }
    }
}

impl ContentIndex {
    fn load(modules: &[Module], campaign: Option<&Campaign>) -> Result<Self, String> {
        let catalogue = Catalogue::load(modules, campaign)?;
        let owner_name = |owner: Uuid, private: bool| {
            if private {
                campaign.map_or_else(
                    || "Campaign".to_owned(),
                    |c| format!("Campaign · {}", c.name),
                )
            } else {
                catalogue.package_name(owner)
            }
        };
        let builtin = catalogue.modules.first().map(|module| module.id);
        const FROM_SHEET: &str = "From a character sheet";
        // A spell copied from a character sheet is not the book's text. It sorts after.
        let from_sheet = |entry: &Entry| {
            Uuid::parse_str(
                std::path::Path::new(&entry.path)
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .as_ref(),
            )
            .is_ok()
        };
        let mut items: Vec<Item> = catalogue
            .entries
            .values()
            .filter(|e| !["package", "campaign", "reference"].contains(&e.entry.kind.as_str()))
            .map(|e| Item {
                lower: e.entry.name.to_lowercase(),
                subtitle: presentation::subtitle(&e.entry),
                owner: if from_sheet(&e.entry) {
                    FROM_SHEET.to_owned()
                } else {
                    owner_name(e.owner, e.private)
                },
                private: e.private,
                section: presentation::Section::for_entry(&e.entry),
                builtin: Some(e.owner) == builtin,
                from_sheet: from_sheet(&e.entry),
                entry: e.entry.clone(),
            })
            .collect();
        items.sort_by(|a, b| a.lower.cmp(&b.lower).then(a.entry.id.cmp(&b.entry.id)));
        let mut index = Self {
            errors: catalogue.errors.clone(),
            ..Default::default()
        };
        for (position, item) in items.iter().enumerate() {
            index.by_id.insert(item.entry.id, position);
            if let Some(key) = &item.entry.key {
                index.by_key.insert(key.clone(), position);
            }
            if item.entry.kind == "spell" {
                index.spells.push(position);
            }
        }
        index.items = items;
        Ok(index)
    }

    /// The rule for an official condition, by the name on a run's row.
    pub fn condition_rule(&self, condition: &str) -> Option<usize> {
        let name = condition.split(" (").next()?.trim().to_lowercase();
        if !crate::combat_sync::CONDITIONS
            .iter()
            .any(|(_, official)| official.eq_ignore_ascii_case(&name))
        {
            return None;
        }
        let wanted = format!("{name} [condition]");
        self.items.iter().position(|item| item.lower == wanted)
    }

    pub fn position(&self, id: Uuid) -> Option<usize> {
        self.by_id.get(&id).copied()
    }

    /// Best first: exact name, then a prefix, then a word, then anywhere in the name.
    fn search(&self, query: &str) -> Vec<usize> {
        let mut found: Vec<(u8, Origin, usize)> = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                let rank = if item.lower == query {
                    0
                } else if item.lower.starts_with(query) {
                    1
                } else if names_word(&item.lower, query) {
                    2
                } else if item.lower.contains(query) {
                    3
                } else if item.section.label().to_lowercase() == query {
                    4
                } else {
                    return None;
                };
                Some((rank, item.origin(), index))
            })
            .collect();
        found.sort_unstable();
        found.into_iter().map(|(.., index)| index).collect()
    }
}

impl view::Links for ContentIndex {
    fn find(&self, reference: &Reference) -> Option<(usize, String)> {
        let index = match reference {
            Reference::Id(id) => self.by_id.get(id),
            Reference::Key(key) => self.by_key.get(key),
        }?;
        Some((*index, self.items[*index].entry.name.clone()))
    }
    fn find_spell(&self, reference: &Reference) -> Option<(usize, String)> {
        self.find(reference)
            .filter(|(index, _)| self.items[*index].entry.kind == "spell")
    }
    fn spells_named_in(&self, text: &str) -> Vec<(usize, String)> {
        self.spells
            .iter()
            .filter(|index| names_word(text, &self.items[**index].entry.name))
            .map(|index| (*index, self.items[*index].entry.name.clone()))
            .collect()
    }
}

#[derive(Default)]
pub struct Lookup {
    pub open: bool,
    query: String,
    index: Option<Arc<ContentIndex>>,
    pending: Option<mpsc::Receiver<Result<ContentIndex, String>>>,
    error: Option<String>,
    scope: Option<(Option<Uuid>, Vec<Uuid>)>,
    matches: Option<(String, Vec<Hit>)>,
    current: Option<usize>,
    /// What the DM wrote, read once each time the panel opens.
    written: Vec<Written>,
    written_stale: bool,
    /// The note or event being read, in place of an entry.
    reading_written: Option<usize>,
    /// Identity, because a rebuilt index can change positions.
    history: Vec<Uuid>,
    waiting_for: Option<Uuid>,
    focus_search: bool,
    results_open: bool,
    highlight: usize,
    /// An entry the DM asked to see in the Library.
    pub open_in_library: Option<Entry>,
    /// A note or an event the DM asked to see where it lives.
    pub open_written: Option<WrittenIn>,
    /// A draft there means this entry cannot open there yet. Reading here still works.
    pub library_draft: bool,
}

impl Lookup {
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub fn index(&self) -> Option<Arc<ContentIndex>> {
        self.index.clone()
    }

    /// Load or reload in the background when the active campaign or packages change.
    pub fn poll(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        let active = campaigns.active_campaign();
        let scope = (
            active.as_ref().map(|c| c.id),
            campaigns.modules().iter().map(|m| m.id).collect::<Vec<_>>(),
        );
        if self.scope.as_ref() != Some(&scope) {
            self.scope = Some(scope);
            self.start_load(ctx, campaigns.modules().to_vec(), active);
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    let identity = self.current_id();
                    match result {
                        Ok(index) => {
                            self.current = identity.and_then(|id| index.position(id));
                            if let Some(id) = self.waiting_for.take() {
                                self.current = index.position(id);
                            }
                            self.index = Some(Arc::new(index));
                            self.matches = None;
                            self.error = None;
                        }
                        Err(error) => self.error = Some(error),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Loading stopped. Switch pages to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }

    /// Content on disk changed, for example after a copy.
    pub fn refresh(&mut self) {
        self.scope = None;
    }

    fn start_load(
        &mut self,
        ctx: &egui::Context,
        modules: Vec<Module>,
        campaign: Option<Campaign>,
    ) {
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(ContentIndex::load(&modules, campaign.as_ref()));
            ctx.request_repaint();
        });
    }

    fn current_id(&self) -> Option<Uuid> {
        let index = self.index.as_ref()?;
        Some(index.items.get(self.current?)?.entry.id)
    }

    /// Is the panel open with old words, or none? The shell then passes them in.
    pub fn wants_written(&self) -> bool {
        self.open && self.written_stale
    }

    pub fn set_written(&mut self, written: Vec<Written>) {
        self.written = written;
        self.written_stale = false;
        self.matches = None;
        self.reading_written = None;
    }

    /// An entry named in a note's link, if there is one by that name.
    pub fn open_name(&mut self, name: &str) -> bool {
        let found = self.index.as_ref().and_then(|index| {
            let wanted = name.to_lowercase();
            index
                .search(&wanted)
                .into_iter()
                .find(|at| index.items[*at].lower == wanted)
        });
        if let Some(position) = found {
            self.open_position(position);
        }
        found.is_some()
    }

    pub fn toggle(&mut self) {
        self.written_stale = true;
        self.open = !self.open;
        self.focus_search = self.open;
        self.results_open = self.current.is_none() || !self.query.is_empty();
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    /// Open one entry from a link somewhere else in the app.
    pub fn open_position(&mut self, position: usize) {
        self.open = true;
        self.written_stale = true;
        self.reading_written = None;
        self.navigate(position);
        self.results_open = false;
        self.query.clear();
    }

    pub fn open_id(&mut self, id: Uuid) {
        self.open = true;
        self.written_stale = true;
        self.reading_written = None;
        self.results_open = false;
        // The field must not name something other than the entry on screen.
        self.query.clear();
        match self.index.as_ref().and_then(|index| index.position(id)) {
            Some(position) => self.navigate(position),
            None => self.waiting_for = Some(id),
        }
    }

    fn navigate(&mut self, position: usize) {
        if let Some(id) = self.current_id()
            && self.current != Some(position)
        {
            self.history.push(id);
        }
        self.current = Some(position);
    }

    /// What the DM wrote that has these words. A name match comes before words in a note.
    fn search_written(&self, query: &str) -> Vec<usize> {
        let mut found: Vec<(bool, usize)> = self
            .written
            .iter()
            .enumerate()
            .filter_map(|(at, written)| {
                let in_title = written.title.to_lowercase().contains(query);
                (in_title || written.text.to_lowercase().contains(query)).then_some((!in_title, at))
            })
            .collect();
        found.sort_unstable();
        found.into_iter().map(|(_, at)| at).collect()
    }

    fn back(&mut self) {
        self.reading_written = None;
        if let Some(id) = self.history.pop()
            && let Some(position) = self.index.as_ref().and_then(|index| index.position(id))
        {
            self.current = Some(position);
            self.results_open = false;
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let width = design::side_panel_width(ctx.content_rect().width());
        design::Column::panel("lookup", width)
            .page_keeps(design::PANEL_LEAVES_PAGE)
            .frame(
                egui::Frame::new()
                    .fill(colors().page)
                    .stroke(egui::Stroke::new(design::HAIRLINE_WIDTH, colors().rule))
                    .inner_margin(egui::Margin::same(design::M as i8)),
            )
            .show(ctx, |ui| self.panel(ui));
    }

    fn panel(&mut self, ui: &mut egui::Ui) {
        if design::panel_header(ui, "Lookup", "Close Lookup") {
            self.close();
        }
        let width = ui.available_width();
        let before = self.query.clone();
        let response = design::search_field(
            ui,
            SEARCH_NAME,
            &mut self.query,
            "Spell, creature, rule, note…",
            width,
        );
        if std::mem::take(&mut self.focus_search) {
            design::focus_and_select(ui, response.id, &self.query);
        }
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        let Some(index) = self.index.clone() else {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label(
                    Type::Body
                        .rich(ui, "Reading content…")
                        .color(colors().muted),
                );
            });
            return;
        };
        if !index.errors.is_empty() {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        format!(
                            "{} notes could not be read, so they are not here.",
                            index.errors.len()
                        ),
                    )
                    .color(colors().document.invalid),
            )
            .on_hover_text(index.errors.join("\n"));
        }
        let query = self.query.trim().to_lowercase();
        if self.matches.as_ref().is_none_or(|(q, _)| *q != query) {
            let found = if query.is_empty() {
                Vec::new()
            } else {
                index
                    .search(&query)
                    .into_iter()
                    .map(Hit::Entry)
                    .chain(self.search_written(&query).into_iter().map(Hit::Written))
                    .collect()
            };
            self.matches = Some((query.clone(), found));
            self.highlight = 0;
        }
        let matches = self.matches.as_ref().unwrap().1.clone();
        if before != self.query && !query.is_empty() {
            // In a short panel the best match opens by itself, and its text gets the height.
            self.results_open = matches.is_empty() || ui.available_height() >= 600.0;
            match matches.first() {
                Some(Hit::Entry(top)) => {
                    self.current = Some(*top);
                    self.reading_written = None;
                }
                Some(Hit::Written(top)) => self.reading_written = Some(*top),
                None => {}
            }
        }
        let mut chosen = None;
        if !query.is_empty()
            && !self.results_open
            && (self.current.is_some() || self.reading_written.is_some())
            && Action::quiet(&design::count(matches.len(), "result", "results"))
                .icon(icons::DOWN)
                .density(Density::Read)
                .name("Show the results")
                .show(ui)
                .clicked()
        {
            self.results_open = true;
        }
        if response.has_focus() {
            let (down, up) = ui.input_mut(|i| {
                (
                    i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown),
                    i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp),
                )
            });
            if down && self.highlight + 1 < matches.len() {
                self.highlight += 1;
                chosen = matches.get(self.highlight).copied();
            }
            if up && self.highlight > 0 {
                self.highlight -= 1;
                chosen = matches.get(self.highlight).copied();
            }
        }
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.results_open = false;
        }
        let show_results = self.results_open && !query.is_empty();
        if show_results {
            ui.horizontal(|ui| {
                ui.label(
                    Type::Small
                        .rich(
                            ui,
                            format!(
                                "{} · everything",
                                design::count(matches.len(), "result", "results")
                            ),
                        )
                        .color(colors().muted),
                );
            });
            if matches.is_empty() {
                ui.label(
                    Type::Body
                        .rich(ui, "Nothing found. Check the spelling, or try fewer words.")
                        .color(colors().muted),
                );
            }
            // The reader stays the larger part of the panel, even at the minimum window.
            let rows = matches
                .len()
                .min(if ui.available_height() < 600.0 { 3 } else { 5 });
            ui.spacing_mut().item_spacing.y = 0.0;
            egui::ScrollArea::vertical()
                .id_salt("lookup_results")
                .max_height(RESULT_ROW * rows as f32)
                .auto_shrink([false, true])
                .show_rows(ui, RESULT_ROW, matches.len(), |ui, range| {
                    for row in range {
                        let hit = matches[row];
                        let (title, detail, selected) = match hit {
                            Hit::Entry(position) => {
                                let item = &index.items[position];
                                (
                                    item.entry.name.as_str(),
                                    format!("{} · {}", item.subtitle, item.owner),
                                    self.reading_written.is_none()
                                        && self.current == Some(position),
                                )
                            }
                            Hit::Written(at) => {
                                let written = &self.written[at];
                                (
                                    written.title.as_str(),
                                    written.detail.clone(),
                                    self.reading_written == Some(at),
                                )
                            }
                        };
                        let response =
                            design::two_line_row(ui, RESULT_ROW, selected, title, &detail);
                        if response.clicked() {
                            chosen = Some(hit);
                        }
                    }
                });
            ui.spacing_mut().item_spacing.y = design::XS;
        }
        if let Some(hit) = chosen {
            match hit {
                Hit::Entry(position) => {
                    self.reading_written = None;
                    if self.current != Some(position) {
                        self.navigate(position);
                    }
                }
                Hit::Written(at) => self.reading_written = Some(at),
            }
            // Arrow keys walk the list. A click on a row means read it.
            if !response.has_focus() {
                self.results_open = false;
            }
        }
        design::rule(ui);
        if let Some(at) = self.reading_written {
            self.written_reader(ui, at, &query);
            return;
        }
        let Some(current) = self.current.and_then(|c| index.items.get(c)) else {
            if query.is_empty() {
                ui.add_space(design::S);
                ui.label(
                    Type::Body
                        .rich(
                            ui,
                            "Search built-in rules, every package, this campaign's own entries, session notes and the timeline. Press Escape to close.",
                        )
                        .color(colors().muted),
                );
            }
            return;
        };
        ui.horizontal(|ui| {
            if !self.history.is_empty()
                && Action::quiet("Back")
                    .icon(icons::BACK)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
            {
                self.back();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if Action::quiet("Open in Library")
                    .icon(icons::OPEN_OUT)
                    .density(Density::Read)
                    .blocked_if(
                        self.library_draft,
                        "Save or discard the Library draft first.",
                    )
                    .show(ui)
                    .clicked()
                {
                    self.open_in_library = Some(current.entry.clone());
                }
            });
        });
        let mut link = None;
        egui::ScrollArea::vertical()
            .id_salt(("lookup_reader", current.entry.id))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let title = current
                    .entry
                    .reading()
                    .ok()
                    .flatten()
                    .map_or_else(|| current.entry.name.clone(), |reading| reading.title);
                style::title(ui, &title);
                style::subtitle(ui, &format!("{} · {}", current.subtitle, current.owner));
                link = view::show(
                    ui,
                    &current.entry.name,
                    Some(&current.entry),
                    &current.entry.body,
                    &view::Origin {
                        path: &current.entry.path,
                        owner: &current.owner,
                        builtin: current.builtin,
                    },
                    index.as_ref(),
                );
            });
        if let Some(position) = link {
            self.navigate(position);
            self.results_open = false;
            // The field must not name something other than the entry on screen.
            self.query.clear();
        }
    }

    /// A note or an event, with the lines that have the words first: a long note must not
    /// make the DM hunt for the sentence that answers the table's question.
    fn written_reader(&mut self, ui: &mut egui::Ui, at: usize, query: &str) {
        let Some(written) = self.written.get(at) else {
            self.reading_written = None;
            return;
        };
        let place = match written.place {
            WrittenIn::Session(_) => "Open this session",
            WrittenIn::Event(_) => "Open on the timeline",
        };
        let mut open = false;
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                open = Action::quiet(place)
                    .icon(icons::OPEN_OUT)
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
            });
        });
        egui::ScrollArea::vertical()
            .id_salt(("lookup_written", at))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                style::title(ui, &written.title);
                style::subtitle(ui, &written.detail);
                let found: Vec<&str> = written
                    .text
                    .lines()
                    .filter(|line| !query.is_empty() && line.to_lowercase().contains(query))
                    .take(FOUND_LINES)
                    .collect();
                // A short note is all in view, and needs no second copy of its own line.
                if !found.is_empty() && written.text.lines().count() > FOUND_FROM_LINES {
                    design::group_label(ui, "Found here");
                    for line in found {
                        let words = line.trim_start_matches(['#', '-', '*', ' ']);
                        ui.add(
                            egui::Label::new(Type::Body.rich(ui, words).color(colors().text))
                                .wrap(),
                        );
                    }
                    design::rule(ui);
                }
                crate::markdown_view::show_compact(ui, &written.text);
            });
        if open {
            self.open_written = Some(written.place.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_package_named_campaign_is_not_the_dms_own_entry() {
        let mut item = ContentIndex::load(&[], None).unwrap().items.remove(0);
        item.owner = "Campaign Companion".into();
        item.private = false;
        item.from_sheet = false;
        assert_eq!(item.origin(), Origin::Book, "a package ranks with the book");
        item.owner = "Campaign · Emberfall".into();
        item.private = true;
        assert_eq!(item.origin(), Origin::Own, "the DM's own entry ranks first");
        item.from_sheet = true;
        assert_eq!(item.origin(), Origin::Sheet, "a sheet's copy ranks last");
        assert!(Origin::Own < Origin::Book && Origin::Book < Origin::Sheet);
    }

    #[test]
    fn opening_an_entry_from_elsewhere_clears_the_old_search() {
        let mut lookup = Lookup {
            query: "captain varra".into(),
            ..Default::default()
        };
        lookup.open_id(Uuid::new_v4());
        assert!(lookup.query.is_empty());
    }

    #[test]
    fn names_match_whole_words_only() {
        assert!(names_word(
            "The cultist casts Ashfall, using Wisdom",
            "Ashfall"
        ));
        assert!(!names_word("The Ashfallen", "Ashfall"));
        assert!(names_word("casts fire bolt.", "Fire Bolt"));
        assert!(!names_word("anything", ""));
    }

    #[test]
    fn search_ranks_exact_then_prefix_then_word_and_covers_built_in_content() {
        let index = ContentIndex::load(&[], None).unwrap();
        let results = index.search("fireball");
        assert_eq!(index.items[results[0]].entry.name, "Fireball");
        let wolves = index.search("wolf");
        assert_eq!(index.items[wolves[0]].entry.name, "Wolf");
        assert!(wolves.len() > 1);
    }
}
