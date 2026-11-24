//! Lookup's panel acts and reads, free of any toolkit, for the GPUI panel
//! (`src/gui/pages/lookup/`).
#![warn(deprecated)]
use super::{
    Hit, Landing, Lookup, OnScreen, RESULTS_LISTED, merge_hits, names_the_query, rows_shown,
    search_written, typed_name,
};
use crate::history::Step;
use crate::world;
use uuid::Uuid;

/// One result row as the panel lists it.
pub struct ResultRow {
    pub title: String,
    pub detail: String,
    pub selected: bool,
}

/// What the reader part of the panel shows now.
pub enum Showing {
    /// The held NPC's card.
    Generated,
    /// A found note or event (its index in the found notes).
    Written(usize),
    /// An entry of the index (its position).
    Entry(usize),
    Nothing,
}

impl Lookup {
    /// The words in the search field.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// The search field asked for the keyboard (once).
    pub fn take_focus_search(&mut self) -> bool {
        std::mem::take(&mut self.focus_search)
    }

    /// The DM typed in the search field: the matches follow, and the best one opens while the
    /// other answers stay in view (one search is one step on Back's trail).
    pub fn set_query(&mut self, words: &str) {
        if self.trackers_shown {
            if words.is_empty() {
                return;
            }
            self.leave_trackers();
            self.results_open = true;
        }
        if self.query == words {
            return;
        }
        self.query = words.to_owned();
        if self.query.trim().is_empty() {
            self.typed_shown = false;
        }
        let Some(index) = self.index.clone() else {
            return;
        };
        let query = self.refresh_matches();
        if query.is_empty() {
            return;
        }
        let matches = self
            .matches
            .as_ref()
            .map(|(_, m)| m.clone())
            .unwrap_or_default();
        self.results_open = matches.len() != 1;
        match matches.first() {
            Some(Hit::Entry(top)) => {
                let (position, heading) = index.target_of(*top);
                self.show_typed(Landing::Entry(position));
                self.reader.show_heading(heading);
            }
            Some(Hit::Written(top)) => self.show_typed(Landing::Written(*top)),
            None => {}
        }
    }

    /// The field's Clear: the reader empties too.
    pub fn clear_search(&mut self) {
        self.query.clear();
        self.current = None;
        self.reader.clear();
        self.reading_written = None;
        self.generated = None;
        self.typed_shown = false;
    }

    /// The matches of the words now (worked out once per query), and the trimmed lowercase
    /// query.
    pub fn refresh_matches(&mut self) -> String {
        let query = self.query.trim().to_lowercase();
        if let Some(index) = self.index.clone()
            && self.matches.as_ref().is_none_or(|(q, _)| *q != query)
        {
            let found = if query.is_empty() {
                Vec::new()
            } else {
                merge_hits(
                    &index.search_typos(&query),
                    &search_written(&self.written_search, &query),
                )
            };
            self.matches = Some((query.clone(), found));
            self.highlight = 0;
            self.show_all = false;
        }
        query
    }

    fn hits(&self) -> &[Hit] {
        self.matches.as_ref().map_or(&[], |(_, hits)| hits)
    }

    /// How many matches the words found.
    pub fn match_count(&self) -> usize {
        self.hits().len()
    }

    /// The result list shows (the words found something to choose from).
    pub fn results_shown(&self) -> bool {
        self.results_open && !self.query.trim().is_empty()
    }

    /// `Show N results` offers the hidden list: there are matches, the one on screen is not the
    /// only one, and an entry or a note is open.
    pub fn offers_results(&self) -> bool {
        let Some(index) = &self.index else {
            return false;
        };
        let only_open = match self.hits() {
            [Hit::Entry(only)] => {
                self.reading_written.is_none() && self.current == Some(index.target_of(*only).0)
            }
            [Hit::Written(only)] => self.reading_written == Some(*only),
            _ => false,
        };
        !self.query.trim().is_empty()
            && !self.hits().is_empty()
            && !only_open
            && !self.results_open
            && (self.current.is_some() || self.reading_written.is_some())
    }

    /// `Show N results`.
    pub fn open_results(&mut self) {
        self.results_open = true;
    }

    /// Enter in the field: the reader keeps the view.
    pub fn close_results(&mut self) {
        self.results_open = false;
    }

    /// The rows the list shows now (five, or every match after `Show all`).
    pub fn result_rows(&self) -> Vec<ResultRow> {
        let Some(index) = &self.index else {
            return Vec::new();
        };
        let rows = rows_shown(self.hits().len(), self.show_all);
        self.hits()
            .iter()
            .take(rows)
            .enumerate()
            .map(|(row, hit)| match *hit {
                Hit::Entry(position) => ResultRow {
                    title: index.name_of(&index.items[position]).to_owned(),
                    detail: index.result_detail(position),
                    selected: self.reading_written.is_none() && row == self.highlight,
                },
                Hit::Written(at) => ResultRow {
                    title: self.written[at].title.clone(),
                    detail: self.written[at].detail.clone(),
                    selected: self.reading_written == Some(at) && row == self.highlight,
                },
            })
            .collect()
    }

    /// The most rows the list shows before it scrolls.
    pub fn rows_listed() -> usize {
        RESULTS_LISTED
    }

    /// `Show all N` is there: more matches than rows.
    pub fn more_results(&self) -> bool {
        rows_shown(self.hits().len(), self.show_all) < self.hits().len()
    }

    /// `Show all N`.
    pub fn show_all_results(&mut self) {
        self.show_all = true;
    }

    /// A press on a result row (`keep_open`: the field has the keyboard, as after an arrow key).
    pub fn choose_result(&mut self, row: usize, keep_open: bool) {
        let Some(hit) = self.hits().get(row).copied() else {
            return;
        };
        let Some(index) = self.index.clone() else {
            return;
        };
        self.highlight = row;
        match hit {
            Hit::Entry(position) => self.open_result(&index, position),
            Hit::Written(at) => {
                self.show_now(Landing::Written(at));
            }
        }
        if !keep_open {
            self.results_open = false;
        }
    }

    /// An arrow key in the field: the highlight moves and its match opens.
    pub fn move_highlight(&mut self, down: bool) {
        let count = self.hits().len();
        let next = if down {
            (self.highlight + 1 < count).then_some(self.highlight + 1)
        } else {
            self.highlight.checked_sub(1)
        };
        if let Some(row) = next {
            self.choose_result(row, true);
        }
    }

    /// Back is there.
    pub fn can_go_back(&self) -> bool {
        self.trail.peek(Step::Back).is_some()
    }

    /// Back or Forward.
    pub fn go(&mut self, step: Step) {
        self.step(step);
    }

    /// What the reader shows.
    pub fn showing(&self) -> Showing {
        match self.on_screen() {
            OnScreen::Generated => Showing::Generated,
            OnScreen::Written(at) => Showing::Written(at),
            OnScreen::Entry(at) => Showing::Entry(at),
            OnScreen::Nothing => Showing::Nothing,
        }
    }

    /// The index is still being read.
    pub fn reading(&self) -> bool {
        self.index.is_none()
    }

    /// The index's error.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// The last create's or generate's error.
    pub fn create_error(&self) -> Option<&str> {
        self.create_error.as_deref()
    }

    /// World notes that cannot be read, and the first one's path.
    pub fn broken(&self) -> Option<(usize, String)> {
        let index = self.index.as_ref()?;
        let broken = &index.broken_world;
        broken.first().map(|first| (broken.len(), first.0.clone()))
    }

    /// `Show` beside the broken notes: World's repair opens.
    pub fn show_broken(&mut self) {
        if let Some((_, path)) = self.broken() {
            self.open_world_repair = Some(path);
        }
    }

    /// The title line's open action's words (`Open in Library`, `Open in World`).
    pub fn open_action_label(&self) -> Option<&'static str> {
        self.open_action_words()
    }

    /// Why the open action waits.
    pub fn open_reason(&self) -> Option<String> {
        self.open_lock()
    }

    /// The open action: the page beside Lookup shows the entry.
    pub fn press_open(&mut self) {
        if let Some(entry) = self
            .index
            .as_ref()
            .and_then(|index| self.open_target(index))
            .map(|current| current.entry.clone())
        {
            self.open_in_library = Some(entry);
        }
    }

    /// The typed name, when no entry or note has it: `New NPC “…”` and `New location “…”`.
    pub fn new_name(&self) -> Option<String> {
        self.campaign.as_ref()?;
        let index = self.index.as_ref()?;
        let query = self.query.trim().to_lowercase();
        (!query.is_empty()
            && !index
                .items
                .iter()
                .any(|item| names_the_query(&item.entry.name, &query))
            && !self
                .written
                .iter()
                .any(|item| item.title.eq_ignore_ascii_case(&query)))
        .then(|| typed_name(&self.query))
    }

    /// `Generate NPC` is offered: nothing typed, open or held.
    pub fn offers_generate(&self) -> bool {
        self.campaign.is_some()
            && self.index.is_some()
            && self.query.trim().is_empty()
            && self.current.is_none()
            && self.generated.is_none()
    }

    /// `New NPC “…”` (`kind` `npc`) or `New location “…”` (`location`).
    pub fn create_named(&mut self, kind: &str) {
        let (Some(campaign), Some(name)) = (self.campaign.clone(), self.new_name()) else {
            return;
        };
        let made = world::create_entry(&campaign, &name, kind);
        self.made(made);
    }

    fn made(&mut self, made: Result<Uuid, String>) {
        match made {
            Ok(id) => {
                self.generated = None;
                self.create_error = None;
                self.query.clear();
                self.results_open = false;
                self.refresh();
                self.waiting_for = Some(id);
            }
            Err(error) => self.create_error = Some(error),
        }
    }

    /// `Generate NPC`, or `Again` on its card: a new NPC from the campaign's tables.
    pub fn generate(&mut self) {
        let Some(index) = self.index.clone() else {
            return;
        };
        let tables = index.npc_tables();
        match crate::npc_generator::roll(&tables) {
            Ok(npc) => {
                self.generated = Some(npc);
                self.show_now(Landing::Card);
                self.create_error = None;
            }
            Err(error) => self.create_error = Some(error),
        }
        self.generator_sources = tables.sources;
        self.generator_problems = tables.problems;
    }

    /// The held NPC.
    pub fn generated(&self) -> Option<&crate::npc_generator::Generated> {
        self.generated.as_ref()
    }

    /// The held NPC's card is the view.
    pub fn generated_on_screen(&self) -> bool {
        self.generated.is_some() && self.generated_shown
    }

    /// The tables the generator rolled from, and those it could not use.
    pub fn generator_facts(&self) -> (&[String], &[crate::npc_generator::TableProblem]) {
        (&self.generator_sources, &self.generator_problems)
    }

    /// Where a saved NPC goes.
    pub fn generated_path(&self) -> Option<String> {
        let npc = self.generated.as_ref()?;
        Some(world::new_entry_path(
            self.index.as_deref(),
            &npc.name,
            "npc",
        ))
    }

    /// `Save` on the card.
    pub fn save_generated(&mut self) {
        let (Some(campaign), Some(npc)) = (self.campaign.clone(), self.generated.clone()) else {
            return;
        };
        let made = world::create_generated(&campaign, &npc);
        self.made(made);
    }

    /// `Discard` on the card.
    pub fn discard_generated(&mut self) {
        self.generated = None;
        self.create_error = None;
    }

    /// `Use your own names`: the NPC moves to the World page.
    pub fn generated_to_world(&mut self) {
        let Some(campaign) = self.campaign.clone() else {
            return;
        };
        self.open_world_names = self.generated.take().map(|npc| super::GeneratedCard {
            campaign: campaign.id,
            npc,
            problems: std::mem::take(&mut self.generator_problems),
            sources: std::mem::take(&mut self.generator_sources),
        });
        self.create_error = None;
    }

    /// The row `<name> ✎ Unsaved NPC · Show`: its card comes back.
    pub fn show_card(&mut self) {
        self.show_now(Landing::Card);
    }

    /// A found note, by its index.
    pub fn written_at(&self, at: usize) -> Option<&super::Written> {
        self.written.get(at)
    }

    /// The found note's own action: it opens where it lives.
    pub fn open_written_place(&mut self, at: usize) {
        if let Some(written) = self.written.get(at) {
            self.open_written = Some(written.place.clone());
        }
    }

    /// The campaign Lookup reads.
    pub fn campaign(&self) -> Option<&crate::persistence::Campaign> {
        self.campaign.as_ref()
    }

    /// A link in an entry: another entry opens, or a search for a missing one.
    pub fn follow_nav(&mut self, nav: world::Nav) {
        self.follow(nav);
    }
}

/// The search field's placeholder for the panel's width.
pub fn search_hint(width: f32) -> &'static str {
    super::search_hint(width)
}
