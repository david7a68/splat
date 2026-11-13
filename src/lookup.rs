//! Lookup: search and read any entry beside the current page, without navigating.
//! It covers built-in, package and campaign-owned entries in one index (T2.3), and the
//! campaign's plain notes, with each heading as its own result.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use crate::{
    campaigns::Campaigns,
    content_workspace::Catalogue,
    design::{self, Action, Density, Tone, Type, colors, icons, reader as style},
    history::{self, Step, Trail},
    library::{presentation, view},
    persistence::{Campaign, Module},
    world::{self, Go, Nav, Reader, Role, lines::Line},
};
use eframe::egui;
use splat::content::{Entry, Reference};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::{Arc, mpsc},
};
use uuid::Uuid;

use crate::matching::{self, Rank, Typed, names_lower_word};

const RESULT_ROW: f32 = design::XXL;
/// Result rows before `Show all`, and the most rows its list shows before it scrolls.
const RESULTS_SHOWN: usize = 5;

/// How many result rows the list shows. `Show all` takes a line as tall as a row, so one match
/// more than `RESULTS_SHOWN` shows as a row in that room, not behind a click (round 27 FJ2).
fn rows_shown(matches: usize, show_all: bool) -> usize {
    if show_all || matches <= RESULTS_SHOWN + 1 {
        matches
    } else {
        RESULTS_SHOWN
    }
}
pub(crate) const RESULTS_LISTED: usize = 8;
const PANEL_MARGIN: f32 = design::M;

pub const SEARCH_NAME: &str = "Search everything";
const LOOKUP_SEARCH: design::FieldId = crate::field_id!("lookup_search");

/// The search field's placeholder for this width of the panel.
fn search_hint(width: f32) -> &'static str {
    if width < design::XXL * 10.0 {
        "Search…"
    } else {
        "Search entries and notes…"
    }
}
/// What an empty search finds, below the search field.
pub(crate) const SEARCH_HELP: &str = "Search built-in rules, every package, this campaign’s own entries, session notes and the timeline. Press Escape to close.";
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
#[derive(Clone, Copy, Debug, PartialEq)]
enum Hit {
    Entry(usize),
    Written(usize),
}

/// True if `name` appears in `text` as whole words, in any case.
pub fn names_word(text: &str, name: &str) -> bool {
    names_lower_word(&text.to_lowercase(), &name.to_lowercase())
}

/// Something the DM wrote, in lower case and with its words, for search.
struct WrittenSearch {
    title: String,
    text: String,
    text_words: Vec<String>,
}

impl WrittenSearch {
    fn new(written: &Written) -> Self {
        let text = written.text.to_lowercase();
        Self {
            title: written.title.to_lowercase(),
            text_words: matching::word_set(&text),
            text,
        }
    }
}

/// What the DM wrote that has these words: the title, then the text, as
/// `matching::Typed::with_text` ranks them. True marks a result found only with a typo.
fn search_written(written: &[WrittenSearch], query: &str) -> Vec<(usize, bool)> {
    let typed = Typed::new(query);
    let mut found: Vec<(Rank, usize)> = written
        .iter()
        .enumerate()
        .filter_map(|(at, written)| {
            let title = typed.name(&written.title);
            let rank = typed.with_text(title, &written.text, &written.text_words, 6)?;
            Some((rank, at))
        })
        .collect();
    found.sort_unstable();
    found
        .into_iter()
        .map(|(rank, at)| (at, rank.typo))
        .collect()
}

/// Entries, then what the DM wrote, each found as typed. Then the same, found only with a
/// typo, so that a typo never comes before something found as typed.
fn merge_hits(entries: &[(usize, bool)], written: &[(usize, bool)]) -> Vec<Hit> {
    let hits = |typo: bool| {
        let entries = entries.iter().filter(move |(_, t)| *t == typo);
        let written = written.iter().filter(move |(_, t)| *t == typo);
        entries
            .map(|&(at, _)| Hit::Entry(at))
            .chain(written.map(|&(at, _)| Hit::Written(at)))
    };
    hits(false).chain(hits(true)).collect()
}

/// Where an item came from.
#[derive(Clone, Debug, PartialEq)]
enum Source {
    Entry,
    /// A plain note. It has no file identity of its own.
    Note(Role),
    /// A heading in a plain note: the note, and the heading's words.
    Heading(Uuid, String),
}

/// The World page's group for every dated entry.
pub const HISTORY: &str = "History";

/// The World page's list, by group, with each row's position and how deep it sits.
pub type WorldGroups = BTreeMap<(u8, String), Vec<(usize, usize)>>;

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
    source: Source,
    /// Duplicate World results keep their paths so the DM can choose between equal names.
    duplicate_identity: bool,
    /// For notes and world entries, whose words can also be searched.
    lower_body: String,
    /// A note's or a world entry's text, one fact at a time.
    pub lines: Vec<Line>,
    /// The words of `lower`, and the different words of `lower_body`, for a search by words.
    name_words: Vec<String>,
    body_words: Vec<String>,
    /// A world entry's whole file text as the index read it: a hash, so a check can tell
    /// whether the file still holds the note on screen.
    source_text: Option<u64>,
}

/// A line on one page that links to another page or names it. With no line, only the page
/// is known: the name is in words that are not a line, such as code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mention {
    pub source: usize,
    pub line: Option<usize>,
}

/// A hash of a file's whole text, for comparing a file with the one the index read.
pub(crate) fn text_hash(text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

fn next_generation() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Every entry the active campaign can see, including all registered packages.
#[derive(Default)]
pub struct ContentIndex {
    pub(crate) campaign: Option<Campaign>,
    pub items: Vec<Item>,
    usable: Option<HashSet<Uuid>>,
    package_owners: HashMap<Uuid, Uuid>,
    by_id: HashMap<Uuid, usize>,
    by_key: HashMap<String, usize>,
    spells: Vec<usize>,
    creatures: Vec<usize>,
    /// Lower-case file stems and paths in the campaign, as Obsidian finds a link's target.
    files: HashMap<String, Vec<usize>>,
    /// Every name a note link can use for an item, in item order: its lower-case name, file
    /// stem and path, and its aliases. Headings are not link names.
    link_names: HashMap<String, Vec<usize>>,
    /// Different for each index this process loads, so a cache of its answers can tell when
    /// the index changed.
    generation: u64,
    /// Plain world notes and world entries.
    world: Vec<usize>,
    mentions: HashMap<Uuid, Vec<Mention>>,
    /// What each world entry is part of: each group's name as written, and the group when
    /// found.
    groups: HashMap<Uuid, Vec<(String, Option<usize>)>>,
    /// The world entries that are part of each entry.
    members: HashMap<Uuid, Vec<usize>>,
    /// The dated entries that concern each entry, oldest first.
    history: HashMap<Uuid, Vec<usize>>,
    /// World entries whose `In` chain leads back to themselves. Each one's list row shows
    /// `Place loop`; the reader shows the loop's warning from `errors`.
    place_loop: HashSet<Uuid>,
    /// What the campaign's files looked like just before this was read.
    files_seen: u64,
    pub errors: Vec<String>,
    /// The loot editor's tables and items, read the first time a page asks.
    loot: std::sync::OnceLock<Arc<crate::loot_editor::Sources>>,
    /// Each World note's link problems in its list row's words, found the first time a result
    /// row asks. Any file change makes a new index, so this starts empty again (round 26 AE2).
    link_problems: std::sync::OnceLock<HashMap<usize, String>>,
    pub broken_world: Vec<(String, String)>,
    pub duplicate_world: BTreeMap<String, Vec<String>>,
    /// World notes whose name another World note also has, found once when the index loads, so
    /// a result or a chooser names their folder without a search on each frame.
    shared_world_names: HashSet<usize>,
}

#[cfg(test)]
thread_local! {
    /// `shown_name` calls on this (the UI) thread.
    static SHOWN_NAMES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Runs of the shared-name search, by campaign folder. The index loads on another thread, and
/// scenarios run side by side, so each counts under its own folder.
#[cfg(test)]
static SHARED_NAME_SEARCHES: std::sync::Mutex<BTreeMap<std::path::PathBuf, usize>> =
    std::sync::Mutex::new(BTreeMap::new());

/// How often `shown_name` ran on this thread, and how often the shared-name search ran for
/// the campaign in this folder.
#[cfg(test)]
pub(crate) fn shown_name_counts(campaign: &std::path::Path) -> crate::test_support::Counts {
    crate::test_support::Counts {
        reached: SHOWN_NAMES.with(std::cell::Cell::get),
        work: SHARED_NAME_SEARCHES
            .lock()
            .unwrap()
            .get(campaign)
            .copied()
            .unwrap_or(0),
    }
}

/// Among equal matches: the DM's own entry, then the book, then a sheet's copy.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Origin {
    Own,
    Book,
    Sheet,
}

pub(crate) fn entry_origin(entry: Option<&Entry>, private: bool) -> Origin {
    let from_sheet = entry.is_some_and(|entry| {
        entry.fields.contains_key("splat_character")
            || std::path::Path::new(&entry.path)
                .file_stem()
                .is_some_and(|stem| Uuid::parse_str(&stem.to_string_lossy()).is_ok())
    });
    origin(private, from_sheet)
}

fn origin(private: bool, from_sheet: bool) -> Origin {
    if from_sheet {
        Origin::Sheet
    } else if private {
        Origin::Own
    } else {
        Origin::Book
    }
}

impl Item {
    /// How the name fits a search, with the words split at load.
    fn name_rank(&self, typed: &Typed) -> Option<Rank> {
        typed.name_with_words(&self.lower, self.name_words.iter().map(String::as_str))
    }

    fn origin(&self) -> Origin {
        origin(self.private, self.from_sheet)
    }

    pub fn is_note(&self) -> bool {
        matches!(self.source, Source::Note(_))
    }

    /// The hash of this world entry's whole file text as the index read it (`text_hash`).
    pub(crate) fn source_text(&self) -> Option<u64> {
        self.source_text
    }

    pub fn is_world_note(&self) -> bool {
        self.source == Source::Note(Role::World)
    }

    pub fn is_private(&self) -> bool {
        self.private
    }

    pub fn is_builtin(&self) -> bool {
        self.builtin
    }

    /// What a plain note is for. Nothing, for an entry or a heading.
    pub fn note_role(&self) -> Option<Role> {
        match self.source {
            Source::Note(role) => Some(role),
            _ => None,
        }
    }

    /// What the subject is: the first paragraph of its text.
    pub fn summary(&self) -> &str {
        world::lines::summary(&self.lines)
    }

    pub(crate) fn result_detail(&self) -> String {
        self.result_detail_marked(None)
    }

    /// The result detail with a fault mark (`⚠ Place not found`) before the optional summary,
    /// so a cut detail keeps the fault longest.
    pub(crate) fn result_detail_marked(&self, fault: Option<&str>) -> String {
        let source = if self.private {
            Some("Campaign")
        } else if self.builtin {
            None
        } else {
            Some(self.owner.as_str())
        };
        // The file is what tells two duplicate rows apart, so it comes first and is cut last.
        let subtitle = if self.duplicate_identity {
            format!("{} · {}", self.entry.path, self.subtitle)
        } else {
            self.subtitle.clone()
        };
        let mut detail = presentation::list_subtitle(&subtitle, source);
        if let Some(fault) = fault {
            detail = format!("{detail} · {} {fault}", design::icons::WARN);
        }
        // A world page says what it is in its first paragraph.
        match self.summary() {
            "" => detail,
            summary => format!("{detail} · {summary}"),
        }
    }

    fn file_keys(&self) -> [String; 2] {
        let path = self.entry.path.trim_end_matches(".md").to_lowercase();
        let stem = path.rsplit('/').next().unwrap_or(&path).to_owned();
        [stem, path]
    }
}

/// The Lookup items for one plain note: the note, then one for each heading.
fn note_items(
    note: world::PlainNote,
    owner: &str,
    images: &HashMap<String, std::path::PathBuf>,
) -> Vec<Item> {
    let subtitle = match note.role {
        Role::World => "World note",
        Role::Session => "Session note",
        Role::Transcript => "Transcript",
    };
    let lines = world::lines::lines(&note.body, &note.name);
    let item = |entry: Entry, subtitle: String, lower_body: String, source: Source| Item {
        lower: entry.name.to_lowercase(),
        subtitle,
        owner: owner.to_owned(),
        private: true,
        section: presentation::Section::World,
        builtin: false,
        from_sheet: false,
        source,
        duplicate_identity: false,
        lower_body,
        lines: Vec::new(),
        name_words: Vec::new(),
        body_words: Vec::new(),
        source_text: None,
        entry,
    };
    // The reader already shows the name as the title.
    let shown = note
        .body
        .trim_start()
        .strip_prefix(&format!("# {}\n", note.name))
        .unwrap_or(&note.body);
    let body = world::with_links(shown, images);
    let headings = world::headings(&body);
    let mut main = item(
        world::note_entry(&note.path, &note.name, body),
        subtitle.to_owned(),
        note.body.to_lowercase(),
        Source::Note(note.role),
    );
    main.lines = lines;
    let mut items = Vec::new();
    for heading in headings {
        // A note's own title is the note.
        if heading.eq_ignore_ascii_case(&note.name) {
            continue;
        }
        let name = heading.clone();
        let path = format!("{}#{heading}", note.path);
        items.push(item(
            world::note_entry(&path, &name, String::new()),
            format!("Heading in {}", note.name),
            String::new(),
            Source::Heading(main.entry.id, heading),
        ));
    }
    items.push(main);
    items
}

impl Item {
    /// Is this a note of the campaign's own, read from its folder, not a package's entry?
    pub fn is_campaign_note(&self) -> bool {
        self.private
    }
}

impl ContentIndex {
    pub(crate) fn load(modules: &[Module], campaign: Option<&Campaign>) -> Result<Self, String> {
        let catalogue = Catalogue::load(modules, campaign)?;
        let usable = campaign.map(|campaign| {
            let mut usable: HashSet<_> = catalogue
                .entries
                .values()
                .filter(|entry| entry.private)
                .map(|entry| entry.entry.id)
                .collect();
            if let Ok(document) = crate::content_workspace::CampaignDocument::load(campaign) {
                let closure = catalogue.closure(&document.selections, campaign.id);
                usable.extend(closure.direct);
                usable.extend(closure.required.keys().copied());
            }
            usable.extend(
                catalogue
                    .duplicate_world
                    .iter()
                    .map(|entry| world::note_id(&entry.entry.path)),
            );
            usable
        });
        let owner_name = |owner: Uuid, private: bool| {
            if private {
                "Campaign".to_owned()
            } else {
                catalogue.package_name(owner)
            }
        };
        let builtin = catalogue.modules.first().map(|module| module.id);
        const FROM_SHEET: &str = "From a character sheet";
        // A spell copied from a character sheet is not the book's text. It sorts after.
        let from_sheet = |entry: &Entry| {
            entry.fields.contains_key("splat_character")
                || Uuid::parse_str(
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
            .chain(catalogue.duplicate_world.iter())
            .filter(|e| !["package", "campaign", "reference"].contains(&e.entry.kind.as_str()))
            .map(|e| {
                let duplicate_identity = catalogue
                    .duplicate_world
                    .iter()
                    .any(|other| other.entry.id == e.entry.id);
                Item {
                    duplicate_identity,
                    lower: e.entry.name.to_lowercase(),
                    subtitle: if duplicate_identity {
                        format!(
                            "{} · {} Same {} in two files",
                            presentation::subtitle(&e.entry),
                            icons::WARN,
                            crate::world::IdentityNoun::of(&e.entry).word()
                        )
                    } else {
                        presentation::subtitle(&e.entry)
                    },
                    owner: if from_sheet(&e.entry) {
                        FROM_SHEET.to_owned()
                    } else {
                        owner_name(e.owner, e.private)
                    },
                    private: e.private,
                    section: presentation::Section::for_entry(&e.entry),
                    builtin: Some(e.owner) == builtin,
                    from_sheet: from_sheet(&e.entry),
                    source: Source::Entry,
                    // Another name finds a world entry as its words do.
                    lower_body: if e.entry.kind == "world" {
                        format!("{} {}", world::aliases(&e.entry).join(" "), e.entry.body)
                            .to_lowercase()
                    } else {
                        String::new()
                    },
                    lines: if e.entry.kind == "world" {
                        world::lines::lines(&e.entry.body, &e.entry.name)
                    } else {
                        Vec::new()
                    },
                    name_words: Vec::new(),
                    body_words: Vec::new(),
                    source_text: None,
                    entry: {
                        let mut entry = e.entry.clone();
                        if e.private
                            && entry.kind == "world"
                            && catalogue
                                .duplicate_world
                                .iter()
                                .any(|other| other.entry.path == entry.path)
                        {
                            entry.id = world::note_id(&entry.path);
                        }
                        entry
                    },
                }
            })
            .collect();
        let files_seen = content_fingerprint(modules, campaign);
        if let Some(campaign) = campaign {
            let sessions = crate::sessions::folder(campaign)
                .unwrap_or_else(|_| campaign.path.join(crate::sessions::DEFAULT_FOLDER));
            let scan = world::scan(&campaign.path, &sessions);
            // Each World entry's whole file text, as the folder reading saw it (byte order mark
            // and line ends included), kept as a hash. It is kept only when that reading's entry
            // is the one on screen, so a file that changed between the two readings has none.
            let folder = splat::content::read_folder(&campaign.path)?;
            let read: HashMap<&str, &splat::content::Entry> = folder
                .entries
                .iter()
                .map(|entry| (entry.path.as_str(), entry))
                .collect();
            for item in &mut items {
                if item.entry.kind == "world" {
                    item.source_text = read
                        .get(item.entry.path.as_str())
                        .filter(|read| {
                            read.name == item.entry.name
                                && (std::ptr::eq::<serde_yaml_ng::Mapping>(
                                    &*read.fields,
                                    &*item.entry.fields,
                                ) || *read.fields == *item.entry.fields)
                                && read.body == item.entry.body
                        })
                        .and(folder.text(&item.entry.path))
                        .map(text_hash);
                    item.entry.body = world::with_links(&item.entry.body, &scan.images);
                }
            }
            let owner = owner_name(campaign.id, true);
            for note in scan.notes {
                items.extend(note_items(note, &owner, &scan.images));
            }
        }
        for item in &mut items {
            item.name_words = matching::words(&item.lower).map(str::to_owned).collect();
            // A session's words are searched from what the DM wrote, not from here.
            if !matches!(item.source, Source::Note(Role::Session | Role::Transcript)) {
                item.body_words = matching::word_set(&item.lower_body);
            }
        }
        items.sort_by(|a, b| a.lower.cmp(&b.lower).then(a.entry.id.cmp(&b.entry.id)));
        let mut index = Self {
            campaign: campaign.cloned(),
            usable,
            package_owners: catalogue
                .entries
                .iter()
                .map(|(id, entry)| (*id, entry.owner))
                .collect(),
            duplicate_world: catalogue
                .duplicate_world
                .iter()
                .map(|entry| {
                    (
                        entry.entry.path.clone(),
                        catalogue
                            .duplicate_world
                            .iter()
                            .filter(|other| other.entry.id == entry.entry.id)
                            .map(|other| other.entry.path.clone())
                            .collect(),
                    )
                })
                .collect(),
            errors: catalogue.errors.clone(),
            files_seen,
            generation: next_generation(),
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
            if item.entry.kind == "creature" {
                index.creatures.push(position);
            }
            let heading = matches!(item.source, Source::Heading(..));
            if !heading {
                let [stem, path] = item.file_keys();
                // `resolve_link` compares aliases without regard to ASCII case only.
                let aliases = world::aliases(&item.entry)
                    .into_iter()
                    .map(str::to_ascii_lowercase);
                for name in [item.lower.clone(), stem, path].into_iter().chain(aliases) {
                    let positions = index.link_names.entry(name).or_default();
                    if positions.last() != Some(&position) {
                        positions.push(position);
                    }
                }
            }
            if item.private && !heading {
                let [stem, path] = item.file_keys();
                index.files.entry(stem).or_default().push(position);
                if item.entry.path.contains('/') {
                    index.files.entry(path).or_default().push(position);
                }
                for alias in world::aliases(&item.entry) {
                    let positions = index.files.entry(alias.to_lowercase()).or_default();
                    if !positions.contains(&position) {
                        positions.push(position);
                    }
                }
            }
            if item.is_world_note() || item.entry.kind == "world" {
                index.world.push(position);
            }
        }
        if let Some(campaign) = campaign {
            index.broken_world = splat::content::read_folder(&campaign.path)?
                .errors
                .iter()
                .map(|error| (error.path.clone(), error.message.clone()))
                .collect();
        }
        index.items = items;
        index.shared_world_names = index.find_shared_world_names();
        index.mentions = index.find_mentions();
        // Relations and history are read once, so a reader does no search on each frame.
        for at in index.world.clone() {
            let groups = index.find_groups(&index.items[at].entry);
            for group in groups.iter().filter_map(|(_, group)| *group) {
                if group != at {
                    let id = index.items[group].entry.id;
                    index.members.entry(id).or_default().push(at);
                }
            }
            if !groups.is_empty() {
                index.groups.insert(index.items[at].entry.id, groups);
            }
        }
        for at in index.world.clone() {
            let id = index.items[at].entry.id;
            let history = index.find_history(id);
            if !history.is_empty() {
                index.history.insert(id, history);
            }
        }
        let (in_loop, loops) = index.place_loops();
        index.place_loop = in_loop;
        index.errors.extend(loops);
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

    /// The campaign's tables for the NPC generator, from its world notes and entries. Paths
    /// decide which comes first, so the result does not depend on how the index was built.
    pub fn npc_tables(&self) -> crate::npc_generator::Tables {
        let mut notes: Vec<(&str, &str)> = self
            .items
            .iter()
            .filter(|item| item.is_world_note() || (item.private && item.entry.kind == "world"))
            .map(|item| (item.entry.path.as_str(), item.entry.body.as_str()))
            .collect();
        notes.sort_unstable();
        crate::npc_generator::Tables::read(notes)
    }

    /// The campaign's roll table with this key, ready to roll, or why there can be none.
    pub fn roll_table(&self, key: &str) -> Result<crate::roll_table::TableRoll, String> {
        self.loot_sources().roll(key)
    }

    /// A World note's link problems as its list row says them (`Place not found`), or `None`.
    /// Found for every note once per index, so a result row costs a map lookup.
    pub(crate) fn link_problem_words(&self, at: usize) -> Option<&str> {
        self.link_problems
            .get_or_init(|| {
                (0..self.items.len())
                    .filter(|&at| self.items[at].entry.kind == "world")
                    .filter_map(|at| {
                        let problems = world::broken_links(&self.items[at], self);
                        (!problems.is_empty()).then(|| (at, world::link_problem_words(&problems)))
                    })
                    .collect()
            })
            .get(&at)
            .map(String::as_str)
    }

    /// A result row's detail: what the entry is, its link fault when it has one, then its
    /// summary. World's list row and Lookup's result say the same fault (round 26 AE2).
    pub(crate) fn result_detail(&self, at: usize) -> String {
        self.items[at].result_detail_marked(self.link_problem_words(at))
    }

    /// What a loot list can name: each roll table in the campaign's own notes by its key, the
    /// first by path, and the items this campaign can use. Read once for this index.
    pub fn loot_sources(&self) -> Arc<crate::loot_editor::Sources> {
        self.loot
            .get_or_init(|| {
                let mut notes: Vec<(&str, &str)> = self
                    .items
                    .iter()
                    .filter(|item| item.is_world_note() || item.private)
                    .map(|item| (item.entry.path.as_str(), item.entry.body.as_str()))
                    .collect();
                notes.sort_unstable();
                let items = (0..self.items.len())
                    .filter(|&at| {
                        let item = &self.items[at];
                        item.entry.kind == "item"
                            && (item.private
                                || self
                                    .usable
                                    .as_ref()
                                    .is_none_or(|usable| usable.contains(&item.entry.id)))
                    })
                    .map(|at| {
                        let item = &self.items[at];
                        crate::loot_editor::Item {
                            name: item.entry.name.clone(),
                            lower: item.lower.clone(),
                            link: self.link_to(at),
                            detail: item.result_detail(),
                        }
                    })
                    .collect();
                Arc::new(crate::loot_editor::Sources {
                    tables: crate::loot_editor::Sources::tables_from(notes),
                    items,
                })
            })
            .clone()
    }

    /// The lines that link to each note or entry, and that name each world entry in whole
    /// words.
    fn find_mentions(&self) -> HashMap<Uuid, Vec<Mention>> {
        let mut found: HashMap<Uuid, Vec<Mention>> = HashMap::new();
        let sources: Vec<(usize, Vec<String>)> = (0..self.items.len())
            .filter(|&at| !self.items[at].lower_body.is_empty())
            .map(|at| (at, world::links_in(&self.items[at].lower_body)))
            .collect();
        // A session is a target too, so a Log line shows on the session it names.
        let sessions =
            (0..self.items.len()).filter(|&at| self.items[at].note_role() == Some(Role::Session));
        for target in self.world.iter().copied().chain(sessions) {
            let item = &self.items[target];
            let keys = item.file_keys();
            let mut names = Vec::new();
            if item.entry.kind == "world" {
                names.push(item.entry.name.to_lowercase());
                names.extend(world::aliases(&item.entry).iter().map(|a| a.to_lowercase()));
            }
            for &(source, ref links) in &sources {
                let from = &self.items[source];
                if source == target
                    || !(links.iter().any(|link| keys.contains(link))
                        || names
                            .iter()
                            .any(|name| names_lower_word(&from.lower_body, name)))
                {
                    continue;
                }
                let mentions = found.entry(item.entry.id).or_default();
                let before = mentions.len();
                mentions.extend(
                    from.lines
                        .iter()
                        .enumerate()
                        .filter(|(_, line)| {
                            line.links_to(&keys) || names.iter().any(|name| line.names(name))
                        })
                        .map(|(line, _)| Mention {
                            source,
                            line: Some(line),
                        }),
                );
                if mentions.len() == before {
                    mentions.push(Mention { source, line: None });
                }
            }
        }
        found
    }

    fn parent(&self, position: usize) -> Option<usize> {
        let entry = &self.items[position].entry;
        self.world_target(entry, "in")
    }

    /// Flat World links resolve by file, then key, UUID, and unique catalogue name.
    pub(crate) fn world_target(&self, entry: &Entry, key: &str) -> Option<usize> {
        self.world_value_target(entry.fields.get("splat_world")?.get(key)?)
    }

    /// What this entry is part of: each group's name as written, and the group when found.
    pub(crate) fn groups_of(&self, entry: &Entry) -> &[(String, Option<usize>)] {
        self.groups.get(&entry.id).map_or(&[], Vec::as_slice)
    }

    fn find_groups(&self, entry: &Entry) -> Vec<(String, Option<usize>)> {
        world::part_of(entry)
            .into_iter()
            .map(|value| {
                (
                    world::link_name(value, "Group"),
                    self.world_value_target(value),
                )
            })
            .collect()
    }

    /// The world entries that are part of this one, by name.
    pub fn members(&self, id: Uuid) -> &[usize] {
        self.members.get(&id).map_or(&[], Vec::as_slice)
    }

    /// The dated entries that concern this one, oldest first: each event that links to it,
    /// names it, is in it or is part of it. An event with a date Splat cannot read is last.
    pub fn history_of(&self, id: Uuid) -> &[usize] {
        self.history.get(&id).map_or(&[], Vec::as_slice)
    }

    fn find_history(&self, id: Uuid) -> Vec<usize> {
        let Some(here) = self.position(id) else {
            return Vec::new();
        };
        let mut events: Vec<usize> = self
            .mentions_of(id)
            .into_iter()
            .chain(self.members(id).iter().copied())
            .chain(self.inside(&self.items[here].entry))
            .filter(|&at| world::when(&self.items[at].entry).is_some())
            .collect();
        events.sort_by(|&a, &b| self.event_order(a).cmp(&self.event_order(b)));
        events.dedup();
        events
    }

    /// Oldest first, then by name. The position makes two equal events one after the other.
    pub(crate) fn event_order(&self, at: usize) -> (std::cmp::Reverse<Option<u64>>, &str, usize) {
        let entry = &self.items[at].entry;
        (
            std::cmp::Reverse(world::when(entry).and_then(world::ago)),
            self.items[at].lower.as_str(),
            at,
        )
    }

    /// The folder that holds most of the campaign's entries of this type.
    pub(crate) fn type_folder(&self, kind: &str) -> Option<String> {
        world::common_folder(
            self.world
                .iter()
                .map(|&at| &self.items[at])
                .filter(|item| item.private && item.entry.kind == "world")
                .map(|item| (item.entry.path.as_str(), world::world_type(&item.entry))),
            kind,
        )
    }

    pub(crate) fn world_value_target(&self, value: &serde_yaml_ng::Value) -> Option<usize> {
        let raw = value
            .as_str()
            .or_else(|| value.get("link").and_then(serde_yaml_ng::Value::as_str));
        if let Some(raw) = raw {
            if let Some(link) = raw.strip_prefix("[[").and_then(|s| s.strip_suffix("]]")) {
                let name = link
                    .split(['|', '#'])
                    .next()?
                    .trim()
                    .trim_end_matches(".md")
                    .to_lowercase();
                let matches = self.files.get(&name).cloned().unwrap_or_else(|| {
                    self.items
                        .iter()
                        .enumerate()
                        .filter(|(_, item)| {
                            !matches!(item.source, Source::Heading(..))
                                && item.file_keys().contains(&name)
                        })
                        .map(|(at, _)| at)
                        .collect()
                });
                return match matches.as_slice() {
                    [at] => Some(*at),
                    _ => None,
                };
            }
            if let Some(at) = self.by_key.get(raw) {
                return Some(*at);
            }
            if let Ok(id) = Uuid::parse_str(raw) {
                return self.by_id.get(&id).copied();
            }
            // A heading in a note has a name too, and it is not an entry.
            let mut matches = self.items.iter().enumerate().filter(|(_, item)| {
                !matches!(item.source, Source::Heading(..))
                    && item.entry.name.eq_ignore_ascii_case(raw)
            });
            let (at, _) = matches.next()?;
            return matches.next().is_none().then_some(at);
        }
        if let Some(key) = value.get("key").and_then(serde_yaml_ng::Value::as_str) {
            return self.by_key.get(key).copied();
        }
        value
            .get("id")
            .and_then(serde_yaml_ng::Value::as_str)
            .and_then(|id| Uuid::parse_str(id).ok())
            .and_then(|id| self.by_id.get(&id).copied())
    }

    pub fn place_of(&self, id: Uuid) -> Option<usize> {
        self.parent(self.position(id)?)
    }

    pub(crate) fn place_choices(&self, id: Uuid) -> Vec<usize> {
        let Some(at) = self.position(id) else {
            return Vec::new();
        };
        let Some(link) = self.items[at]
            .entry
            .fields
            .get("splat_world")
            .and_then(|fields| fields.get("in"))
            .and_then(serde_yaml_ng::Value::as_str)
        else {
            return Vec::new();
        };
        let Some(target) = link
            .strip_prefix("[[")
            .and_then(|link| link.strip_suffix("]]"))
        else {
            return Vec::new();
        };
        match self.resolve_link(target, Some(id)) {
            Go::Many(choices) => choices,
            _ => Vec::new(),
        }
    }

    /// The World notes whose name another World note in another file also has. It runs once
    /// per index; the test count is kept here.
    fn find_shared_world_names(&self) -> HashSet<usize> {
        #[cfg(test)]
        if let Some(campaign) = &self.campaign {
            *SHARED_NAME_SEARCHES
                .lock()
                .unwrap()
                .entry(campaign.path.clone())
                .or_default() += 1;
        }
        let mut by_name: HashMap<&str, Vec<usize>> = HashMap::new();
        for &at in &self.world {
            let item = &self.items[at];
            if item.is_private() {
                by_name
                    .entry(item.entry.name.as_str())
                    .or_default()
                    .push(at);
            }
        }
        by_name
            .into_values()
            .filter(|found| {
                found
                    .iter()
                    .any(|&at| self.items[at].entry.path != self.items[found[0]].entry.path)
            })
            .flatten()
            .collect()
    }

    /// A World note's name as a result or a chooser shows it: with its folder when another World
    /// note has the same name (`Watch House (Locations)`), so the two can be told apart. The
    /// shared names are found when the index loads.
    pub(crate) fn shown_name(&self, at: usize) -> String {
        #[cfg(test)]
        SHOWN_NAMES.with(|count| count.set(count.get() + 1));
        let entry = &self.items[at].entry;
        if !self.shared_world_names.contains(&at) {
            return entry.name.clone();
        }
        match entry.path.rsplit_once('/') {
            Some((folder, _)) => format!("{} ({folder})", entry.name),
            None => format!("{} ({})", entry.name, entry.path),
        }
    }

    /// Whether this world entry's `In` chain leads back to it.
    pub(crate) fn in_place_loop(&self, id: Uuid) -> bool {
        self.place_loop.contains(&id)
    }

    /// The entries in a place loop, and one warning for each.
    fn place_loops(&self) -> (HashSet<Uuid>, Vec<String>) {
        let starts: Vec<usize> = self
            .world
            .iter()
            .copied()
            .filter(|&start| {
                std::iter::successors(self.parent(start), |&at| self.parent(at))
                    .take(self.world.len())
                    .any(|at| at == start)
            })
            .collect();
        let in_loop = starts.iter().map(|&at| self.items[at].entry.id).collect();
        let warnings = starts
            .into_iter()
            .map(|start| {
                let mut at = start;
                let mut parts = Vec::new();
                for _ in 0..self.world.len() {
                    let Some(parent) = self.parent(at) else {
                        break;
                    };
                    parts.push(format!(
                        "{} is in {}",
                        self.items[at].entry.name, self.items[parent].entry.name
                    ));
                    if parent == start {
                        break;
                    }
                    at = parent;
                }
                format!("{}. Change one of them.", parts.join(", and "))
            })
            .collect();
        (in_loop, warnings)
    }

    pub fn position_of(&self, reference: &Reference) -> Option<usize> {
        match reference {
            Reference::Id(id) => self.by_id.get(id),
            Reference::Key(key) => self.by_key.get(key),
        }
        .copied()
    }

    /// Where a link in a note leads.
    pub fn follow(&self, target: &str, from: &Item) -> Go {
        self.resolve_link(target, Some(from.entry.id))
    }

    /// Shared Obsidian link rule for every reader. Only World notes and entries
    /// this campaign can use are name targets. Explicit reference keys still work.
    pub fn resolve_link(&self, target: &str, from: Option<Uuid>) -> Go {
        let target = target.split('|').next().unwrap_or(target);
        let (name, heading) = target
            .split_once('#')
            .map_or((target.trim(), None), |(name, heading)| {
                (name.trim(), Some(heading.trim()))
            });
        if name.eq_ignore_ascii_case("timeline")
            && let Some(id) = heading.and_then(|h| h.strip_prefix('^'))
        {
            return Go::Timeline(id.into());
        }
        let heading = heading
            .filter(|h| !h.starts_with('^'))
            .map(|h| h.rsplit('#').next().unwrap_or(h).to_owned());
        if name.is_empty() {
            return from
                .and_then(|id| self.position(id))
                .map_or_else(|| Go::Missing(target.into()), |at| Go::Item(at, heading));
        }
        let reference =
            Uuid::parse_str(name).map_or_else(|_| Reference::Key(name.into()), Reference::Id);
        if let Some(at) = self.position_of(&reference) {
            return Go::Item(at, heading);
        }
        let found = self.named(&name.trim_end_matches(".md").to_lowercase());
        match found.as_slice() {
            &[at] => Go::Item(at, heading),
            [] => Go::Missing(name.into()),
            _ => Go::Many(found),
        }
    }

    /// The wiki link that a note can use for a search result. A bare name is best, as
    /// Obsidian also reads it, when it names the file and nothing else. Otherwise the link
    /// names the item exactly and shows its name: the DM's own by path, a book's by key.
    /// A heading that a link cannot name, such as `Room #3`, links to its note. Of two
    /// headings with the same words, a link finds the first, in Obsidian too.
    pub(crate) fn link_to(&self, position: usize) -> String {
        let (at, heading) = self.target_of(position);
        let heading = heading.filter(|heading| !heading.contains(['#', '|', '[', ']', '^']));
        let item = &self.items[at];
        let finds = |target: &str| matches!(self.resolve_link(target, None), Go::Item(found, _) if found == at);
        let section = heading
            .as_deref()
            .map_or_else(String::new, |heading| format!("#{heading}"));
        let name = item.entry.name.trim();
        let path = item.entry.path.split('#').next().unwrap_or_default();
        let path = path.strip_suffix(".md").unwrap_or(path);
        let stem = path.rsplit('/').next().unwrap_or(path);
        if !name.contains(['|', '#', '[', ']']) && stem.eq_ignore_ascii_case(name) && finds(name) {
            return format!("[[{name}{section}]]");
        }
        let key = item.entry.key.as_deref().filter(|key| finds(key));
        let path = Some(path).filter(|path| !path.is_empty() && finds(path));
        let target = if item.private || item.is_note() {
            path.or(key)
        } else {
            key.or(path)
        }
        .map_or_else(|| item.entry.id.to_string(), str::to_owned);
        let shown: String = heading
            .as_deref()
            .unwrap_or(name)
            .chars()
            .filter(|c| !matches!(c, '|' | '[' | ']'))
            .collect();
        format!("[[{target}{section}|{shown}]]")
    }

    /// Prepare wiki links in any reader, retaining exactly the note's display words.
    pub(crate) fn broken_link_count(&self, body: &str, from: Uuid) -> usize {
        let visible = body
            .lines()
            .filter(|line| !presentation::hidden_provenance_line(line))
            .collect::<Vec<_>>()
            .join("\n");
        world::links_in(&visible)
            .into_iter()
            .filter(|target| self.link_is_missing(target, from))
            .count()
    }

    /// A link from this page names nothing the campaign can open.
    pub(crate) fn link_is_missing(&self, target: &str, from: Uuid) -> bool {
        let page = target
            .split(['|', '#'])
            .next()
            .unwrap_or(target)
            .trim()
            .trim_start_matches("../")
            .trim_end_matches(".md");
        // The index answers first, because the books are a long list to search.
        matches!(self.resolve_link(target, Some(from)), Go::Missing(_))
            && !splat::content::builtin::entries().iter().any(|entry| {
                entry.path.trim_end_matches(".md")
                    == page.strip_prefix("srd-5.2.1/").unwrap_or(page)
            })
    }

    /// The campaign's world notes and entries, in name order.
    pub(crate) fn world_pages(&self) -> &[usize] {
        &self.world
    }

    /// The items a link can name with this lower-case key, in item order. Only World notes,
    /// session notes and entries this campaign can use are name targets. A session is one
    /// because a world entry's Log links to the session that changed it.
    fn named(&self, key: &str) -> Vec<usize> {
        self.link_names
            .get(key)
            .into_iter()
            .flatten()
            .copied()
            .filter(|&at| {
                let item = &self.items[at];
                matches!(item.note_role(), Some(Role::World | Role::Session))
                    || self
                        .usable
                        .as_ref()
                        .is_none_or(|usable| usable.contains(&item.entry.id))
            })
            .collect()
    }

    /// A campaign file has this name, in any folder. Obsidian then may not know which a
    /// bare link means.
    pub(crate) fn has_file_named(&self, stem: &str) -> bool {
        self.files.contains_key(&stem.to_lowercase())
    }

    /// Which load of the index this is. See `generation` on the struct.
    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    pub fn reader_markdown(&self, body: &str, from: Option<Uuid>) -> String {
        let source = world::with_links(body, &HashMap::new());
        let prefix = "splat-note:";
        let mut output = String::new();
        let mut rest = source.as_str();
        while let Some(start) = rest.find(prefix) {
            let end = rest[start..]
                .find('>')
                .map(|end| start + end)
                .unwrap_or(rest.len());
            output.push_str(&rest[..start]);
            let target = &rest[start + prefix.len()..end];
            output.push_str(
                if matches!(self.resolve_link(target, from), Go::Missing(_)) {
                    "splat-missing:"
                } else {
                    prefix
                },
            );
            output.push_str(target);
            rest = &rest[end..];
        }
        output.push_str(rest);
        output
    }

    pub fn unused_creatures(&self, query: &str) -> Vec<(Uuid, Uuid, String, String)> {
        if query.trim().is_empty() {
            return Vec::new();
        }
        self.items
            .iter()
            .filter(|item| {
                item.entry.kind == "creature"
                    && !item.private
                    && item.lower.contains(query)
                    && self
                        .usable
                        .as_ref()
                        .is_some_and(|usable| !usable.contains(&item.entry.id))
            })
            .filter_map(|item| {
                // Owner IDs come from the loaded package manifest, kept in the index.
                self.package_owners.get(&item.entry.id).map(|&owner| {
                    (
                        owner,
                        item.entry.id,
                        item.entry.name.clone(),
                        item.owner.clone(),
                    )
                })
            })
            .take(5)
            .collect()
    }

    pub fn campaign_dependencies(&self, entry: &Entry) -> Vec<usize> {
        let mut found = Vec::new();
        if let Ok(references) = splat::content::required_references(entry) {
            found.extend(
                references
                    .iter()
                    .filter_map(|reference| self.position_of(reference)),
            );
        }
        if let Ok(Some(reading)) = entry.reading() {
            found.extend(reading.related.keys().filter_map(|target| {
                self.position_of(
                    &Uuid::parse_str(target)
                        .map_or_else(|_| Reference::Key(target.clone()), Reference::Id),
                )
            }));
        }
        for target in world::links_in(&entry.body) {
            match self.resolve_link(&target, Some(entry.id)) {
                Go::Item(at, _) => found.push(at),
                Go::Many(many) => found.extend(many),
                _ => {}
            }
        }
        found.retain(|&at| {
            self.items[at].private
                && !self.items[at].is_note()
                && self.items[at].entry.id != entry.id
        });
        found.sort_unstable();
        found.dedup();
        found
    }

    /// The campaign's locations, each with its name first, then its other names.
    pub fn places(&self) -> Vec<(Uuid, Vec<String>)> {
        self.world
            .iter()
            .map(|&at| &self.items[at].entry)
            .filter(|entry| entry.kind == "world" && world::world_type(entry) == "location")
            .map(|entry| {
                let mut names = vec![entry.name.clone()];
                names.extend(world::aliases(entry).into_iter().map(str::to_owned));
                (entry.id, names)
            })
            .collect()
    }

    /// World entries whose place is this one.
    pub fn inside(&self, entry: &Entry) -> Vec<usize> {
        if entry.kind != "world" {
            return Vec::new();
        }
        self.world
            .iter()
            .copied()
            .filter(|&at| {
                self.parent(at)
                    .is_some_and(|p| self.items[p].entry.id == entry.id)
            })
            .collect()
    }

    /// The pages that link to this one or name it, each once.
    pub fn mentions_of(&self, id: Uuid) -> Vec<usize> {
        let mut pages: Vec<usize> = self.lines_about(id).iter().map(|m| m.source).collect();
        pages.dedup();
        pages
    }

    /// What play changed in a session: each Log line of a world entry that links to the
    /// session's note, with the entry it is on.
    pub fn world_changes(&self, session_note: &std::path::Path) -> Vec<(usize, &Line)> {
        let Some(campaign) = &self.campaign else {
            return Vec::new();
        };
        let path = session_note
            .strip_prefix(&campaign.path)
            .unwrap_or(session_note)
            .to_string_lossy()
            .replace('\\', "/");
        self.lines_about(world::note_id(&path))
            .iter()
            .filter_map(|mention| {
                let line = &self.items[mention.source].lines[mention.line?];
                (self.items[mention.source].entry.kind == "world" && line.in_log())
                    .then_some((mention.source, line))
            })
            .collect()
    }

    /// Every line on another page that links to this one or names it, by page.
    pub fn lines_about(&self, id: Uuid) -> &[Mention] {
        self.mentions.get(&id).map_or(&[], Vec::as_slice)
    }

    /// Creatures by name, best first as `matching::Typed::name` ranks them.
    pub fn creatures_matching(&self, query: &str, limit: usize) -> Vec<usize> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }
        let typed = Typed::new(&query);
        let mut found: Vec<(Rank, usize)> = self
            .creatures
            .iter()
            .filter_map(|&at| Some((self.items[at].name_rank(&typed)?, at)))
            .collect();
        found.sort_unstable();
        found.into_iter().take(limit).map(|(_, at)| at).collect()
    }

    /// NPCs, locations as a tree, other types, then notes by folder.
    pub fn world_groups(&self, search: &str) -> WorldGroups {
        let mut groups = WorldGroups::new();
        let mut locations = Vec::new();
        for &at in &self.world {
            let item = &self.items[at];
            if !item.lower.contains(search) && !item.lower_body.contains(search) {
                continue;
            }
            let entry = &item.entry;
            let group = if entry.kind != "world" {
                let folder = entry
                    .path
                    .rsplit_once('/')
                    .map_or("Notes", |(folder, _)| folder);
                (3, folder.to_owned())
            } else {
                match world::world_type(entry) {
                    // Whatever its type, a dated entry is history, and reads in date order.
                    _ if world::when(entry).is_some() => (2, HISTORY.to_owned()),
                    "npc" => (0, world::type_group("npc")),
                    "location" => {
                        locations.push(at);
                        continue;
                    }
                    other => (2, world::type_group(other)),
                }
            };
            groups.entry(group).or_default().push((at, 0));
        }
        if let Some(events) = groups.get_mut(&(2, HISTORY.to_owned())) {
            events.sort_by(|&(a, _), &(b, _)| self.event_order(a).cmp(&self.event_order(b)));
        }
        if !locations.is_empty() {
            groups.insert(
                (1, world::type_group("location")),
                self.location_tree(&locations),
            );
        }
        groups
    }

    fn location_tree(&self, locations: &[usize]) -> Vec<(usize, usize)> {
        fn walk(
            children: &BTreeMap<Option<usize>, Vec<usize>>,
            parent: Option<usize>,
            depth: usize,
            rows: &mut Vec<(usize, usize)>,
        ) {
            for &at in children.get(&parent).into_iter().flatten() {
                rows.push((at, depth));
                walk(children, Some(at), depth + 1, rows);
            }
        }
        let present: HashSet<usize> = locations.iter().copied().collect();
        let mut children: BTreeMap<Option<usize>, Vec<usize>> = BTreeMap::new();
        for &at in locations {
            let parent = self.parent(at).filter(|parent| present.contains(parent));
            children.entry(parent).or_default().push(at);
        }
        let mut rows = Vec::new();
        walk(&children, None, 0, &mut rows);
        // A loop has no root. It still shows, flat, and the index reports it.
        let seen: HashSet<usize> = rows.iter().map(|&(at, _)| at).collect();
        rows.extend(
            locations
                .iter()
                .filter(|at| !seen.contains(at))
                .map(|&at| (at, 0)),
        );
        rows
    }

    /// The note a result opens, and the heading to show, for a heading's result.
    fn target_of(&self, position: usize) -> (usize, Option<String>) {
        if let Source::Heading(note, heading) = &self.items[position].source
            && let Some(note) = self.position(*note)
        {
            return (note, Some(heading.clone()));
        }
        (position, None)
    }

    pub fn position(&self, id: Uuid) -> Option<usize> {
        self.by_id.get(&id).copied()
    }

    /// Best first: exact name, then a prefix, then a word, then anywhere in the name, then
    /// each word of the search in any order. Then the section's name, then a note's words. A
    /// typo comes last: in a name, and then, last of all, in a note's words.
    pub(crate) fn search(&self, query: &str) -> Vec<usize> {
        self.search_typos(query)
            .into_iter()
            .map(|(at, _)| at)
            .collect()
    }

    /// `search`, with true for a result found only with a typo.
    fn search_typos(&self, query: &str) -> Vec<(usize, bool)> {
        let typed = Typed::new(query);
        let mut found: Vec<(Rank, Origin, usize)> = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                // What the DM wrote in a session has its own results, from the session page.
                if matches!(item.source, Source::Note(Role::Session | Role::Transcript)) {
                    return None;
                }
                let name = item.name_rank(&typed);
                let section = || item.section.label().to_lowercase() == query;
                let name = matching::with_other(name, name.is_none_or(|r| r.typo) && section());
                let rank = typed.with_text(name, &item.lower_body, &item.body_words, 7)?;
                Some((rank, item.origin(), index))
            })
            .collect();
        found.sort_unstable();
        found
            .into_iter()
            .map(|(rank, _, index)| (index, rank.typo))
            .collect()
    }
}

impl view::Links for ContentIndex {
    fn find(&self, reference: &Reference) -> Option<(usize, String)> {
        let index = self.position_of(reference)?;
        Some((index, self.items[index].entry.name.clone()))
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
    pub run_open: bool,
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
    /// `written` prepared for search once, not on each key.
    written_search: Vec<WrittenSearch>,
    /// The note or event being read, in place of an entry.
    reading_written: Option<usize>,
    /// Identity, because a rebuilt index can change positions.
    trail: Trail<Uuid>,
    /// Where the panel was drawn last, for Back and Forward beside the Library.
    panel_rect: Option<egui::Rect>,
    waiting_for: Option<Uuid>,
    focus_search: bool,
    results_open: bool,
    /// `Show all` lists every match in place. New words return to five.
    show_all: bool,
    /// An arrow key moved the highlight, so the list brings that row into view.
    follow_highlight: bool,
    highlight: usize,
    /// An entry the DM asked to see in the Library.
    pub open_in_library: Option<Entry>,
    /// A note or an event the DM asked to see where it lives.
    pub open_written: Option<WrittenIn>,
    /// A draft there means this entry cannot open there yet. Reading here still works.
    pub library_draft: bool,
    campaign: Option<Campaign>,
    pub reader: Reader,
    create_error: Option<String>,
    /// What the generator rolled, until the DM saves it or lets it go.
    generated: Option<crate::npc_generator::Generated>,
    /// Whether that NPC's card is the view. An entry that opens takes its place, under the
    /// row `Unsaved NPC: <name> · Show`.
    generated_shown: bool,
    /// Campaign tables the generator could not use.
    generator_problems: Vec<crate::npc_generator::TableProblem>,
    generator_sources: Vec<String>,
    pub open_world_repair: Option<String>,
    /// A generated NPC that `Use your own names` hands to the World page.
    pub open_world_names: Option<GeneratedCard>,
    /// A check of campaign and package files, away from the frame.
    pub files_changed: bool,
    checking: Option<mpsc::Receiver<u64>>,
    was_focused: bool,
    /// The panel shows the Trackers view in place of what it held, which it keeps.
    trackers_shown: bool,
    /// Ctrl+T opened the panel, so Ctrl+T closes it again.
    trackers_closes: bool,
    /// Trackers just came into view: it shows from its top, where the Due rows are.
    trackers_from_top: bool,
}

/// Include registered packages even when there is no active campaign.
fn content_fingerprint(modules: &[Module], campaign: Option<&Campaign>) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    for root in campaign
        .map(|c| &c.path)
        .into_iter()
        .chain(modules.iter().map(|m| &m.path))
    {
        root.hash(&mut hash);
        world::fingerprint(root).hash(&mut hash);
    }
    hash.finish()
}

impl Lookup {
    /// The check that starts when the window gets focus decides whether every page reloads
    /// from disk, so it counts until its answer is in.
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some() || self.checking.is_some()
    }

    pub fn index(&self) -> Option<Arc<ContentIndex>> {
        self.index.clone()
    }

    /// Load or reload in the background when the active campaign or packages change.
    pub fn poll(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        let active = campaigns.active_campaign();
        // Coming back from Obsidian, show what was just written there.
        let focused = ctx.input(|i| i.focused);
        if focused && !self.was_focused {
            let (tx, rx) = mpsc::channel();
            let campaign = active.clone();
            let modules = campaigns.modules().to_vec();
            let waker = ctx.clone();
            std::thread::spawn(move || {
                let _ = tx.send(content_fingerprint(&modules, campaign.as_ref()));
                waker.request_repaint();
            });
            self.checking = Some(rx);
        }
        self.was_focused = focused;
        if let Some(seen) = self.checking.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.checking = None;
            if self
                .index
                .as_ref()
                .is_some_and(|index| index.files_seen != seen)
            {
                self.scope = None;
                self.files_changed = true;
            }
        }
        self.campaign = active.clone();
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
                                self.generated_shown = false;
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
        self.written_search = written.iter().map(WrittenSearch::new).collect();
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

    /// Bold prose opens a unique name, or leaves the choice to the DM in Lookup.
    pub fn open_words(&mut self, words: &str) {
        let found = self.index.as_ref().and_then(|index| {
            let wanted = words.to_lowercase();
            let mut matches = index
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| item.lower == wanted);
            let (position, _) = matches.next()?;
            matches.next().is_none().then_some(position)
        });
        if let Some(position) = found {
            self.open_position(position);
        } else {
            self.search_for(words);
        }
    }

    pub fn toggle(&mut self) {
        self.written_stale = true;
        self.open = !self.open;
        // Lookup opens on what it showed last, also on Trackers, whose keys are its own.
        self.focus_search = self.open && !self.trackers_shown;
        self.trackers_closes = false;
        self.results_open = self.current.is_none() || !self.query.is_empty();
    }

    /// Ctrl+T: show Trackers, and again go back to what Lookup showed. When Lookup was closed,
    /// the second press closes it.
    pub fn toggle_trackers(&mut self) {
        if self.open && self.trackers_shown {
            self.trackers_shown = false;
            if std::mem::take(&mut self.trackers_closes) {
                self.open = false;
            }
        } else {
            // An open Lookup keeps what it shows under Trackers, also a found note. A closed one
            // reads the notes again as it opens.
            self.written_stale |= !self.open;
            self.trackers_closes = !self.open;
            self.open = true;
            self.trackers_shown = true;
            self.trackers_from_top = true;
        }
    }

    /// The rail's `Due N`: Trackers in view, whatever Lookup showed.
    pub fn show_trackers(&mut self) {
        if !(self.open && self.trackers_shown) {
            self.toggle_trackers();
        }
    }

    pub fn on_trackers(&self) -> bool {
        self.open && self.trackers_shown
    }

    /// The DM opened something in Lookup, so the panel shows it.
    fn leave_trackers(&mut self) {
        self.trackers_shown = false;
        self.trackers_closes = false;
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    /// The name of the NPC this panel rolled and has not saved.
    pub fn generated_name(&self) -> Option<&str> {
        self.generated.as_ref().map(|npc| npc.name.as_str())
    }

    /// Take back a generated NPC that the World page did not take.
    pub fn keep_generated(&mut self, card: GeneratedCard) {
        self.generated = Some(card.npc);
        self.generated_shown = true;
        self.generator_problems = card.problems;
        self.generator_sources = card.sources;
    }

    /// Open one entry from a link somewhere else in the app.
    pub fn open_note_link(&mut self, target: &str, from: Option<Uuid>, ctx: &egui::Context) {
        let Some(index) = &self.index else {
            return;
        };
        match index.resolve_link(target, from) {
            Go::Item(at, heading) => {
                self.reader.show_heading(heading);
                self.open_position(at);
            }
            Go::Many(positions) => {
                self.open = true;
                self.leave_trackers();
                self.query.clear();
                self.reader.choose_link(positions, target);
            }
            Go::Timeline(id) => self.reader.open_timeline = Some(id),
            _ => {}
        }
        ctx.request_repaint();
    }

    pub fn open_position(&mut self, position: usize) {
        self.open = true;
        self.leave_trackers();
        self.written_stale = true;
        self.reading_written = None;
        self.navigate(position);
        self.results_open = false;
        self.query.clear();
    }

    /// Open with this search, for a link to something that does not exist yet.
    pub fn search_for(&mut self, words: &str) {
        self.open = true;
        self.leave_trackers();
        self.query = words.to_owned();
        self.focus_search = true;
        self.results_open = true;
    }

    pub fn open_id(&mut self, id: Uuid) {
        self.open = true;
        self.leave_trackers();
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

    /// Read an editor result directly, without repeating the editor's result list.
    pub fn open_creature_result(&mut self, id: Uuid) {
        self.open_id(id);
        if let Some(item) = self
            .index
            .as_ref()
            .and_then(|index| index.position(id).map(|at| &index.items[at]))
        {
            self.query = item.entry.name.clone();
            self.results_open = false;
        }
    }

    fn navigate(&mut self, position: usize) {
        if self.current != Some(position) {
            self.trail.leave(self.current_id());
        }
        self.current = Some(position);
        self.generated_shown = false;
    }

    /// Go to a result. A heading's result opens its note at that heading.
    fn open_result(&mut self, index: &ContentIndex, position: usize) {
        let (position, heading) = index.target_of(position);
        if self.current != Some(position) {
            self.navigate(position);
        }
        // The entry on screen, chosen again, also takes the NPC card's place.
        self.generated_shown = false;
        self.reader.show_heading(heading);
    }

    fn follow(&mut self, nav: Nav) {
        match nav {
            Nav::Item(position) => {
                self.navigate(position);
                self.results_open = false;
                // The field must not name something other than the entry on screen.
                self.query.clear();
            }
            Nav::Missing(words) => self.search_for(&words),
            Nav::Reload(id) => {
                self.refresh();
                self.waiting_for = Some(id);
            }
        }
    }

    /// With nothing that way, nothing changes, so a stray press keeps a written note open.
    fn step(&mut self, step: Step) {
        // Beside Trackers, Back returns to the entry that Lookup held.
        if self.trackers_shown {
            if step == Step::Back && self.holds_something() {
                self.leave_trackers();
            }
            return;
        }
        if self.trail.peek(step).is_none() {
            return;
        }
        self.reading_written = None;
        let here = self.current_id();
        if let Some((_, position)) = self
            .trail
            .go(step, here, |id| self.index.as_ref()?.position(*id))
        {
            self.current = Some(position);
            self.results_open = false;
            self.generated_shown = false;
        }
    }

    /// Back and Forward. Beside the Library page, a key acts here only with focus here,
    /// and a mouse button only with the pointer here; the Library takes the rest.
    pub(crate) fn history_input(&mut self, ctx: &egui::Context, library_page: bool) {
        if !self.open {
            return;
        }
        let over_panel = |pos: egui::Pos2| self.panel_rect.is_some_and(|rect| rect.contains(pos));
        let step = history::take_if(ctx, |source| {
            !library_page
                || match source {
                    history::Request::Key => ctx
                        .memory(|memory| memory.focused())
                        .and_then(|id| ctx.read_response(id))
                        .is_some_and(|response| over_panel(response.rect.center())),
                    history::Request::Button(pos) => over_panel(pos),
                }
        });
        if let Some(step) = step {
            self.step(step);
        }
    }

    /// An entry or a found note, which Trackers keeps while it is in view.
    fn holds_something(&self) -> bool {
        self.current.is_some() || self.reading_written.is_some()
    }

    pub fn show(&mut self, ctx: &egui::Context, trackers: &mut crate::trackers::Trackers) {
        let width = design::side_panel_width(ctx.content_rect().width());
        design::Column::panel("lookup", width)
            .page_keeps(design::PANEL_LEAVES_PAGE)
            .frame(
                egui::Frame::new()
                    .fill(colors().page)
                    .stroke(egui::Stroke::new(design::HAIRLINE_WIDTH, colors().rule))
                    .inner_margin(egui::Margin::same(PANEL_MARGIN as i8)),
            )
            .show(ctx, |ui| self.panel(ui, trackers));
    }

    /// The search field while Trackers is the view: empty, with its placeholder, so it does not
    /// read as a filter of the trackers. Lookup keeps its words for its own view. Words typed
    /// here are a new search, which leaves Trackers for its results.
    fn search_over_trackers(&mut self, ui: &mut egui::Ui, width: f32) -> egui::Response {
        let mut typed = String::new();
        let response = LOOKUP_SEARCH
            .search(ui, SEARCH_NAME, &mut typed, search_hint(width), width)
            .field;
        if !typed.is_empty() {
            self.query = typed;
            self.leave_trackers();
            self.results_open = true;
        }
        response
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn panel(&mut self, ui: &mut egui::Ui, trackers: &mut crate::trackers::Trackers) {
        // With the margin, so a press anywhere on the panel counts as here.
        let panel = ui.max_rect().expand(PANEL_MARGIN);
        self.panel_rect = Some(panel);
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("reader_surface"), egui::Id::new("lookup"))
        });
        let mut close = false;
        if self.trackers_shown {
            let mut back = false;
            ui.horizontal(|ui| {
                // The count belongs to the `DUE N` heading below; the title says nothing more (FH8).
                ui.label(Type::Heading.rich(ui, "Trackers"));
                back = self.holds_something()
                    && Action::quiet("Back")
                        .icon(icons::BACK)
                        .key("Alt+Left")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    close = design::icon_button(ui, icons::CLOSE, "Close Lookup").clicked();
                });
            });
            if back {
                self.leave_trackers();
            }
        } else {
            ui.horizontal(|ui| {
                ui.label(Type::Heading.rich(ui, "Lookup"));
                if self.trail.peek(Step::Back).is_some()
                // The icon buttons' density, so the title line keeps one height with or without
                // Back, and the search field below never moves.
                && Action::quiet("Back")
                    .icon(icons::BACK)
                    .key("Alt+Left")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.step(Step::Back);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    close = design::icon_button(ui, icons::CLOSE, "Close Lookup").clicked();
                    if let Some(current) = self
                    .index
                    .as_ref()
                    .and_then(|index| self.current.and_then(|at| index.items.get(at)))
                    // Every World note opens on World, plain notes too (FD4). A session note
                    // has no World page.
                    && (!current.is_note() || current.is_world_note())
                    && design::locked(
                        ui,
                        self.library_draft
                            .then_some("Save or discard the Library draft first."),
                        |ui| {
                            design::icon_button(
                                ui,
                                icons::OPEN_OUT,
                                if current.entry.kind == "world" || current.is_world_note() {
                                    "Open in World"
                                } else {
                                    "Open in Library"
                                },
                            )
                            .clicked()
                        },
                    ) {
                        self.open_in_library = Some(current.entry.clone());
                    }
                });
            });
        }
        if close {
            self.close();
        }
        let width = ui.available_width();
        let before = self.query.clone();
        let response = if self.trackers_shown {
            self.search_over_trackers(ui, width)
        } else {
            let search =
                LOOKUP_SEARCH.search(ui, SEARCH_NAME, &mut self.query, search_hint(width), width);
            if search.cleared {
                self.current = None;
                self.reader.clear();
                self.reading_written = None;
                self.generated = None;
            }
            search.field
        };
        if std::mem::take(&mut self.focus_search) {
            design::focus_and_select(ui, response.id, &self.query);
        }
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        if self.trackers_shown {
            design::rule(ui);
            crate::trackers::view::show(ui, trackers, std::mem::take(&mut self.trackers_from_top));
            return;
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
        let broken = &index.broken_world;
        if !broken.is_empty() {
            ui.horizontal_wrapped(|ui| {
                // A note that cannot be read is an error here too, as on World's header.
                design::status(
                    ui,
                    design::Status {
                        icon: icons::ERROR,
                        word: &format!(
                            "{} cannot be read",
                            design::count(broken.len(), "note", "notes")
                        ),
                        color: colors().result.error,
                    },
                );
                if design::link(ui, "Show").clicked() {
                    self.open_world_repair = Some(broken[0].0.clone());
                }
            });
        }
        let query = self.query.trim().to_lowercase();
        if self.matches.as_ref().is_none_or(|(q, _)| *q != query) {
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
        let matches = self.matches.as_ref().unwrap().1.clone();
        if before != self.query && !query.is_empty() {
            // Open the best match and keep the other answers in view.
            self.results_open = matches.len() != 1;
            match matches.first() {
                Some(Hit::Entry(top)) => {
                    let (position, heading) = index.target_of(*top);
                    self.current = Some(position);
                    self.reader.show_heading(heading);
                    self.reading_written = None;
                    self.generated_shown = false;
                }
                Some(Hit::Written(top)) => {
                    self.reading_written = Some(*top);
                    self.generated_shown = false;
                }
                None => {}
            }
        }
        if let Some(error) = &self.create_error {
            design::message(ui, Tone::Error, error);
        }
        let matches = &self.matches.as_ref().unwrap().1;
        let mut chosen = None;
        // One result that is already open offers nothing to show (FI6).
        let only_open = match matches.as_slice() {
            [Hit::Entry(only)] => {
                self.reading_written.is_none() && self.current == Some(index.target_of(*only).0)
            }
            [Hit::Written(only)] => self.reading_written == Some(*only),
            _ => false,
        };
        if !query.is_empty()
            && !matches.is_empty()
            && !only_open
            && !self.results_open
            && (self.current.is_some() || self.reading_written.is_some())
            // Outside a message, a line-start action has no icon: its words start at the edge.
            // Its verb says that a press shows the hidden list; a bare count reads as a label.
            && Action::quiet(&format!(
                "Show {}",
                design::count(matches.len(), "result", "results")
            ))
                .density(Density::Read)
                .name("Show the results")
                .line_start()
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
            self.follow_highlight |= down || up;
        }
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.results_open = false;
        }
        let show_results = self.results_open && !query.is_empty();
        if show_results {
            if matches.is_empty() {
                ui.label(
                    Type::Body
                        .rich(ui, "Nothing found. Check the spelling, or try fewer words.")
                        .color(colors().muted),
                );
            }
            // The reader stays the larger part of the panel, even at the minimum window.
            // `Show all` names every match, in a list that scrolls past eight rows.
            let rows = rows_shown(matches.len(), self.show_all);
            let results: Vec<_> = matches
                .iter()
                .take(rows)
                .enumerate()
                .map(|(row, hit)| match *hit {
                    Hit::Entry(position) => {
                        let item = &index.items[position];
                        (
                            item.entry.name.clone(),
                            index.result_detail(position),
                            self.reading_written.is_none() && row == self.highlight,
                        )
                    }
                    Hit::Written(at) => {
                        let written = &self.written[at];
                        (
                            written.title.clone(),
                            written.detail.clone(),
                            self.reading_written == Some(at) && row == self.highlight,
                        )
                    }
                })
                .collect();
            ui.spacing_mut().item_spacing.y = 0.0;
            let follow = std::mem::take(&mut self.follow_highlight);
            // Every row keeps its control, so Tab reaches a match below the list's edge. A row
            // paints only while it is in view.
            egui::ScrollArea::vertical()
                .id_salt("lookup_results")
                .max_height(RESULT_ROW * rows.min(RESULTS_LISTED) as f32)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    for row in 0..rows {
                        let (title, detail, selected) = &results[row];
                        let full = format!("{title} · {detail}");
                        let response = design::row(ui, RESULT_ROW, *selected, &full, |ui, rect| {
                            let name = ui.painter().layout_no_wrap(
                                title.clone(),
                                Type::Body.font(ui),
                                colors().text,
                            );
                            let room = (rect.width() - name.size().x).max(0.0);
                            let fitted = fitted_detail(ui, detail, room);
                            // A fault mark has its own tone, as on a Library row.
                            let tail = crate::library::marked_line(
                                ui,
                                &if fitted.is_empty() {
                                    String::new()
                                } else {
                                    format!("{DETAIL_SEPARATOR}{fitted}")
                                },
                                Type::Small.font(ui),
                                colors().muted,
                                room,
                            );
                            let top = rect.center().y - name.size().y / 2.0;
                            ui.painter().galley(
                                egui::pos2(rect.left(), top),
                                name.clone(),
                                colors().text,
                            );
                            ui.painter().galley(
                                egui::pos2(
                                    rect.left() + name.size().x,
                                    rect.center().y - tail.size().y / 2.0,
                                ),
                                tail,
                                colors().muted,
                            );
                        })
                        .on_hover_text(&full);
                        ui.ctx().accesskit_node_builder(response.id, |node| {
                            node.set_role_description("Lookup result");
                        });
                        // egui does not scroll to a row that Tab focuses, and arrow keys move
                        // the highlight from the search field. The list follows both.
                        // egui's scroll target counts the clip margin as visible, so the row
                        // asks for that margin too and shows whole.
                        if response.gained_focus() || (follow && *selected) {
                            let margin = ui.visuals().clip_rect_margin;
                            ui.scroll_to_rect(response.rect.expand2(egui::vec2(0.0, margin)), None);
                        }
                        if response.clicked() {
                            chosen = Some(matches[row]);
                        }
                    }
                });
            ui.spacing_mut().item_spacing.y = design::XS;
            // The same quiet action as Arrivals: it lists every match in place. It starts a
            // line, so its words start at the result rows' text.
            if rows < matches.len()
                && Action::quiet(&format!("Show all {}", matches.len()))
                    .density(Density::Read)
                    .hint("List every match, without more typing")
                    .line_start()
                    .show(ui)
                    .clicked()
            {
                self.show_all = true;
            }
        }
        if let Some(hit) = chosen {
            if let Some(row) = matches.iter().position(|candidate| *candidate == hit) {
                self.highlight = row;
            }
            match hit {
                Hit::Entry(position) => {
                    self.reading_written = None;
                    self.open_result(&index, position);
                }
                Hit::Written(at) => {
                    self.reading_written = Some(at);
                    self.generated_shown = false;
                }
            }
            // Arrow keys walk the list. A click on a row means read it.
            if !response.has_focus() {
                self.results_open = false;
            }
        }
        // Outside the results, because a new name can also match unrelated prose.
        if let Some(campaign) = self.campaign.clone() {
            let mut made = None;
            let mut generate = false;
            let new_name = !query.is_empty()
                && !index
                    .items
                    .iter()
                    .any(|item| item.entry.name.eq_ignore_ascii_case(&query))
                && !self
                    .written
                    .iter()
                    .any(|item| item.title.eq_ignore_ascii_case(&query));
            let offer_generate =
                query.is_empty() && self.current.is_none() && self.generated.is_none();
            // With no creation action to show, the row takes no room.
            if new_name || offer_generate {
                ui.horizontal_wrapped(|ui| {
                    if new_name {
                        let name = typed_name(&self.query);
                        for (at, (word, kind)) in [("New NPC", "npc"), ("New location", "location")]
                            .into_iter()
                            .enumerate()
                        {
                            let label = format!("{word} “{name}”");
                            // No plus: a line-start action outside a message has no icon, so its
                            // words start at the result names. The second keeps the same form.
                            let action = Action::quiet(&label).density(Density::Read);
                            // The second action starts a line only when it wraps.
                            if world::starts_line(ui, action, at == 0).show(ui).clicked() {
                                made = Some(world::create_entry(&campaign, &name, kind));
                            }
                        }
                    }
                    generate = offer_generate
                        && Action::quiet("Generate NPC")
                            .density(Density::Read)
                            .line_start()
                            .show(ui)
                            .clicked();
                });
            }
            let mut reroll = generate;
            if let Some(npc) = self.generated.as_ref().filter(|_| self.generated_shown) {
                let choice = generator_card(
                    ui,
                    npc,
                    &self.generator_problems,
                    &self.generator_sources,
                    self.run_open,
                    NamesHelp::Link,
                    &world::new_entry_path(Some(&*index), &npc.name, "npc"),
                );
                reroll = choice.again;
                if choice.save {
                    made = Some(world::create_generated(&campaign, npc));
                }
                if choice.discard {
                    self.generated = None;
                    self.create_error = None;
                }
                if choice.own_names {
                    // The NPC moves to the World page, so only one card can save it.
                    self.open_world_names = self.generated.take().map(|npc| GeneratedCard {
                        campaign: campaign.id,
                        npc,
                        problems: std::mem::take(&mut self.generator_problems),
                        sources: std::mem::take(&mut self.generator_sources),
                    });
                    self.create_error = None;
                }
            }
            if reroll {
                let tables = index.npc_tables();
                match crate::npc_generator::roll(&tables) {
                    Ok(npc) => {
                        self.generated = Some(npc);
                        self.generated_shown = true;
                        self.create_error = None;
                    }
                    Err(error) => self.create_error = Some(error),
                }
                self.generator_sources = tables.sources;
                self.generator_problems = tables.problems;
            }
            match made {
                Some(Ok(id)) => {
                    self.generated = None;
                    self.create_error = None;
                    self.query.clear();
                    self.results_open = false;
                    self.refresh();
                    self.waiting_for = Some(id);
                }
                Some(Err(error)) => self.create_error = Some(error),
                None => {}
            }
        }
        design::rule(ui);
        if let Some(npc) = &self.generated {
            if self.generated_shown {
                return;
            }
            if world::page::unsaved_npc_row(ui, &npc.name) {
                self.generated_shown = true;
            }
        }
        if let Some(at) = self.reading_written {
            self.written_reader(ui, at, &query);
            return;
        }
        if self.current.is_none() {
            let _ = self.reader.choices(ui, &index).map(|nav| self.follow(nav));
        }
        let Some(current) = self.current.and_then(|c| index.items.get(c)) else {
            self.gone_log_footer(ui);
            if query.is_empty() {
                design::gap(ui, design::Gap::S);
                // Help keeps the reader measure, as every help line does (round 26 AB2).
                design::help(ui, SEARCH_HELP);
            }
            return;
        };
        let mut nav = self.reader.choices(ui, &index);
        if ui.rect_contains_pointer(ui.available_rect_before_wrap())
            && ui.input(|input| input.smooth_scroll_delta.y != 0.0)
        {
            self.results_open = false;
        }
        self.reader.compact = true;
        let (reader, campaign) = (&mut self.reader, self.campaign.as_ref());
        // The reader's results and their Undo sit in Lookup's footer, pinned below the reader,
        // so a result never moves the kind line, `Add to fight` or the text (round 26 FJ3).
        let lines = world::with_lines(
            reader.footer_lines(current, &index, campaign, None),
            crate::trackers::history::footer_lines(trackers, current.entry.id),
        );
        let (pressed, bottom) = world::draw_footer(ui, "lookup_footer", &lines);
        match pressed {
            Some(world::FooterAct::Trackers) => trackers.undo_remove(),
            Some(act) => {
                if let Some(campaign) = campaign {
                    nav = nav
                        .take()
                        .or(reader.footer_act(ui.ctx(), act, current, campaign));
                }
            }
            None => {}
        }
        egui::ScrollArea::vertical()
            .max_height((bottom - ui.cursor().top()).max(0.0))
            .id_salt(("lookup_reader", current.entry.id))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // The results list above sets its own row spacing. The reader keeps the app's,
                // so an entry reads the same whether a search or a link opened it.
                ui.spacing_mut().item_spacing.y = ui.ctx().style().spacing.item_spacing.y;
                let read = reader.read_with(ui, current, &index, campaign, Some(trackers));
                nav = nav.take().or(read);
            });
        if let Some(nav) = nav {
            self.follow(nav);
        }
    }

    /// With no entry on show, the words of a Log line whose entry is gone still sit in
    /// Lookup's footer, with `Copy text`, until the DM copies or dismisses them.
    fn gone_log_footer(&mut self, ui: &mut egui::Ui) {
        let lines = self
            .reader
            .gone_log_notices(self.campaign.as_ref().map(|campaign| campaign.id));
        if lines.is_empty() {
            return;
        }
        if let (Some(world::FooterAct::GoneLog(id)), _) =
            world::draw_footer(ui, "lookup_footer", &lines)
        {
            self.reader.end_gone_log(id);
        }
    }

    /// A note or an event, with the lines that have the words first: a long note must not
    /// make the DM hunt for the sentence that answers the table's question.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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
                // The `L` gap after the kind line, as every other Lookup entry has. Their reader
                // also keeps one row gap for its line warnings above the prose, so the card's
                // words start where a spell's do.
                ui.add_space(design::L + ui.spacing().item_spacing.y);
                let found: Vec<&str> = written
                    .text
                    .lines()
                    .filter(|line| !query.is_empty() && line.to_lowercase().contains(query))
                    .take(FOUND_LINES)
                    .collect();
                // A short note is all in view, and needs no second copy of its own line.
                if !found.is_empty() && written.text.lines().count() > FOUND_FROM_LINES {
                    design::heading(ui, design::Heading::Label("Found here"));
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

/// A generated NPC with the table facts its card shows.
#[derive(Clone, Debug, PartialEq)]
pub struct GeneratedCard {
    /// The campaign whose tables rolled it, and where it will be saved.
    pub campaign: Uuid,
    pub npc: crate::npc_generator::Generated,
    pub problems: Vec<crate::npc_generator::TableProblem>,
    pub sources: Vec<String>,
}

/// The name a New action gives the words the DM typed. It keeps every letter as typed, with
/// one space between words. Only when the DM typed no capital at all does the first letter
/// become one: `toll at the old mill` gives `Toll at the old mill`, and `McTavish` and
/// `d’Arcy` stay as they are. The action's label, the note's file name and its saved name
/// all come from this one name.
fn typed_name(typed: &str) -> String {
    let name = typed.split_whitespace().collect::<Vec<_>>().join(" ");
    if name.chars().any(char::is_uppercase) {
        return name;
    }
    let mut chars = name.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

/// The separator between a result row's name and the parts of its detail.
const DETAIL_SEPARATOR: &str = " · ";

/// A result row's detail, shortened by whole parts to fit `width` after the separator. A fault
/// part starts with the warning or error mark. The kind and other parts before the fault go
/// first, from the nearest, then the parts after it. The file, which tells duplicate rows
/// apart, stays whole while the fault shortens to whole words and then to its mark; only then
/// is the file cut. No cut is inside a word, and a cut ends with `…`, never on a separator.
/// With no room for a word, the row shows no detail.
pub(crate) fn fitted_detail(ui: &egui::Ui, detail: &str, width: f32) -> String {
    let font = Type::Small.font(ui);
    // Each candidate is measured whole, with its separator, as the row paints it.
    let fits =
        |text: &str| design::number_width(ui, &format!("{DETAIL_SEPARATOR}{text}"), &font) <= width;
    // The longest whole-word cut of `text` that fits once `around` places it in the row.
    let whole_words = |text: &str, around: &dyn Fn(&str) -> String| {
        let mut words: Vec<&str> = text.split_whitespace().collect();
        while words.pop().is_some() {
            while words.last().is_some_and(|word| *word == "·") {
                words.pop();
            }
            if words.is_empty() {
                break;
            }
            let cut = format!("{}…", words.join(" "));
            if fits(&around(&cut)) {
                return cut;
            }
        }
        String::new()
    };
    if detail.is_empty() || fits(detail) {
        return detail.to_owned();
    }
    let parts: Vec<&str> = detail.split(DETAIL_SEPARATOR).collect();
    let fault = |part: &str| part.starts_with(icons::WARN) || part.starts_with(icons::ERROR);
    let Some(first_fault) = parts.iter().position(|part| fault(part)) else {
        return whole_words(detail, &|cut| cut.to_owned());
    };
    let last_fault = parts
        .iter()
        .rposition(|part| fault(part))
        .unwrap_or(first_fault);
    // The order parts leave in: the nearest before the fault (not the file), then the last.
    let mut order: Vec<usize> = (1..first_fault)
        .rev()
        .filter(|&at| !fault(parts[at]))
        .collect();
    order.extend(
        (last_fault + 1..parts.len())
            .rev()
            .filter(|&at| !fault(parts[at])),
    );
    let mut kept = vec![true; parts.len()];
    for at in order {
        kept[at] = false;
        let shown = parts
            .iter()
            .zip(&kept)
            .filter_map(|(part, keep)| keep.then_some(*part))
            .collect::<Vec<_>>()
            .join(DETAIL_SEPARATOR);
        if fits(&shown) {
            return shown;
        }
    }
    let faults = parts
        .iter()
        .copied()
        .filter(|part| fault(part))
        .collect::<Vec<_>>()
        .join(DETAIL_SEPARATOR);
    // The fault's warning mark is a whole word, and it stays while any detail shows.
    let mark = faults
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_owned();
    let shorten = |faults: &str, around: &dyn Fn(&str) -> String| {
        let cut = whole_words(faults, around);
        if cut == format!("{mark}…") {
            mark.clone()
        } else {
            cut
        }
    };
    if fault(parts[0]) {
        return shorten(&faults, &|cut| cut.to_owned());
    }
    // The file tells two duplicate rows apart, so it stays whole while the fault shortens to
    // whole words. Only when the file and the mark do not fit is the file cut, last.
    let file = parts[0];
    let cut = shorten(&faults, &|cut| format!("{file}{DETAIL_SEPARATOR}{cut}"));
    if !cut.is_empty() {
        return format!("{file}{DETAIL_SEPARATOR}{cut}");
    }
    if fits(&format!("{file}{DETAIL_SEPARATOR}{mark}")) {
        return format!("{file}{DETAIL_SEPARATOR}{mark}");
    }
    let file = whole_words(file, &|cut| format!("{cut}{DETAIL_SEPARATOR}{mark}"));
    if file.is_empty() {
        shorten(&faults, &|cut| cut.to_owned())
    } else {
        format!("{file}{DETAIL_SEPARATOR}{mark}")
    }
}

/// How a generator card says that it uses the built-in names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NamesHelp {
    /// The World page, where the DM prepares: the table marker in full.
    Marker,
    /// Lookup: short words and a quiet link to the World page's card.
    Link,
    /// A session note's card keeps the note's keyboard, so it only says so.
    Plain,
}

/// The World page's words for using the campaign's own names. The marker itself is copied, not
/// shown, because the DM types it in another program.
pub(crate) const OWN_NAMES_HELP: &str = "Built-in names. To use your own, add a table of names to a note. Paste the marker on the line above the table.";
/// The exact line that marks a table of names, for `Copy the marker`.
pub(crate) const NAMES_MARKER: &str = "<!-- splat:rolls key=npc-name dice=1d6 -->";
pub(crate) const COPY_MARKER: &str = "Copy the marker";
/// After the copy: what the DM changes in the pasted line. The dice are the marker's own.
pub(crate) const MARKER_COPIED: &str = "Copied. Change 1d6 to the number of names.";
pub(crate) const BUILT_IN_NAMES: &str = "Built-in names.";
pub(crate) const USE_OWN_NAMES: &str = "Use your own names";

/// What the DM chose on a generator card.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct CardChoice {
    pub save: bool,
    pub again: bool,
    pub discard: bool,
    /// Lookup's `Use your own names`: show this NPC on the World page with the full help.
    pub own_names: bool,
    /// `Copy the marker` copied the names marker: the page's footer says what to change.
    pub copied_marker: bool,
}

/// What the generator rolled, with its choices. Returns save, reroll and discard. A session
/// note's card says only that the names are built in.
pub(crate) fn generated_card(
    ui: &mut egui::Ui,
    npc: &crate::npc_generator::Generated,
    problems: &[crate::npc_generator::TableProblem],
    sources: &[String],
    run_open: bool,
    save_path: &str,
) -> (bool, bool, bool) {
    let choice = generator_card(
        ui,
        npc,
        problems,
        sources,
        run_open,
        NamesHelp::Plain,
        save_path,
    );
    (choice.save, choice.again, choice.discard)
}

/// What the generator rolled, with its choices and the names help for this place.
pub(crate) fn generator_card(
    ui: &mut egui::Ui,
    npc: &crate::npc_generator::Generated,
    problems: &[crate::npc_generator::TableProblem],
    sources: &[String],
    run_open: bool,
    help: NamesHelp,
    save_path: &str,
) -> CardChoice {
    let mut choice = CardChoice::default();
    design::heading(ui, design::Heading::Label("Generated NPC"));
    ui.add(egui::Label::new(Type::Strong.rich(ui, &npc.name).color(colors().text)).wrap());
    for (label, text) in crate::npc_generator::ATTRIBUTES.iter().zip(&npc.attributes) {
        ui.add(
            egui::Label::new(
                Type::Body
                    .rich(ui, format!("{label}: {text}"))
                    .color(colors().text),
            )
            .wrap(),
        );
    }
    if sources.is_empty() && problems.is_empty() {
        match help {
            NamesHelp::Marker => {
                // Help keeps the reader measure, so it wraps above Copy the marker (round 25
                // AE1, FE2, AD2).
                design::help(ui, OWN_NAMES_HELP);
                // The action stays, so the DM can copy again. Its result, which says what to
                // change in the pasted line, is in the page's footer (round 27 FG2).
                // A quiet action that starts a line, outside a message, has no icon: its words
                // start at the card's text edge.
                let response = Action::quiet(COPY_MARKER)
                    .density(Density::Read)
                    .line_start()
                    .show(ui)
                    .on_hover_text(NAMES_MARKER);
                if response.clicked() {
                    ui.ctx().copy_text(NAMES_MARKER.to_owned());
                    choice.copied_marker = true;
                }
            }
            NamesHelp::Plain => {
                ui.label(Type::Small.rich(ui, BUILT_IN_NAMES).color(colors().muted));
            }
            NamesHelp::Link => {
                ui.horizontal_wrapped(|ui| {
                    ui.label(Type::Small.rich(ui, BUILT_IN_NAMES).color(colors().muted));
                    choice.own_names = design::link(ui, USE_OWN_NAMES).clicked();
                });
            }
        }
    }
    for source in sources {
        let (kind, path) = source.split_once(": ").unwrap_or(("npc-name", source));
        let label = if kind == "npc-name" {
            "Names".to_owned()
        } else {
            style::human_label(kind)
        };
        ui.horizontal_wrapped(|ui| {
            ui.label(
                Type::Small
                    .rich(ui, format!("{label} from"))
                    .color(colors().muted),
            );
            if design::link(ui, path).clicked() {
                ui.ctx().data_mut(|data| {
                    data.insert_temp(egui::Id::new("markdown_clicked_note"), path.to_owned())
                });
            }
        });
    }
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
        for problem in problems {
            let opened = ui.scope(|ui| {
                design::message_with_action(
                    ui,
                    Tone::Warning,
                    &problem.message,
                    &format!("Open {}", problem.path),
                )
            });
            opened.response.on_hover_text(&problem.marker);
            if opened.inner {
                ui.ctx().data_mut(|data| {
                    data.insert_temp(egui::Id::new("markdown_clicked_note"), problem.path.clone())
                });
            }
        }
    });
    ui.label(
        Type::Small
            .rich(ui, format!("Will save {save_path}"))
            .color(colors().muted),
    );
    ui.horizontal_wrapped(|ui| {
        choice.save = (if run_open {
            Action::secondary("Save NPC")
        } else {
            Action::primary("Save NPC")
        })
        .density(Density::Read)
        .show(ui)
        .clicked();
        choice.again = Action::secondary("Reroll")
            .icon(icons::DICE)
            .density(Density::Read)
            .show(ui)
            .clicked();
        choice.discard = Action::quiet("Discard")
            .density(Density::Read)
            .show(ui)
            .clicked();
    });
    choice
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FD6: a result row shortens by whole parts. The kind goes first, then the parts after the
    /// fault. The file stays whole while the fault shortens to whole words and then to its mark;
    /// the file is cut last. No cut is inside a word or ends on a separator before `…`.
    #[test]
    fn result_detail_shortens_by_whole_parts() {
        let mut harness = egui_kittest::Harness::new_ui(|ui| {
            let mark = icons::WARN;
            let fault = format!("{mark} Same note in two files");
            let path = "NPCs/Rook Ambry copy.md";
            let detail = format!("{path} · NPC · {fault} · Campaign");
            let font = Type::Small.font(ui);
            let room = |text: &str| design::number_width(ui, &format!(" · {text}"), &font) + 0.5;
            let fit = |width: f32| fitted_detail(ui, &detail, width);
            // A cut keeps whole words of `whole` and ends with `…`.
            let whole_cut = |cut: &str, whole: &str| {
                cut.strip_suffix('…').is_some_and(|kept| {
                    whole.starts_with(kept)
                        && whole[kept.len()..].starts_with(' ')
                        && !kept.trim_end().ends_with('·')
                })
            };
            // Room for all: nothing changes.
            assert_eq!(fit(room(&detail)), detail, "whole");
            // The kind goes first, even when the part after the fault could go instead.
            let no_kind = format!("{path} · {fault} · Campaign");
            assert_eq!(fit(room(&no_kind)), no_kind, "kind first");
            let file_and_fault = format!("{path} · {fault}");
            assert_eq!(
                fit(room(&file_and_fault)),
                file_and_fault,
                "then after the fault"
            );
            // Then the fault shortens to whole words, after the whole file.
            let cut = fit(room(&file_and_fault) - 1.0);
            let rest = cut
                .strip_prefix(&format!("{path} · "))
                .unwrap_or_else(|| panic!("the file stays whole: {cut:?}"));
            assert!(
                whole_cut(rest, &fault),
                "the fault ends on a whole word: {cut:?}"
            );
            // Then to its mark alone, never a lone `…`.
            let file_and_mark = format!("{path} · {mark}");
            assert_eq!(fit(room(&file_and_mark)), file_and_mark, "the mark stays");
            // Only then is the file cut, to whole words, before the mark.
            let short = fit(room(&file_and_mark) - 1.0);
            let file = short
                .strip_suffix(&format!(" · {mark}"))
                .unwrap_or_else(|| panic!("the mark stays: {short:?}"));
            assert!(
                whole_cut(file, path),
                "the file ends on a whole word: {short:?}"
            );
            // No room for a word: no detail, so no lone `…` after a separator.
            assert_eq!(fit(4.0), "", "no room");
            assert_eq!(fitted_detail(ui, "", 200.0), "", "empty");
            // A row that starts with its fault drops what follows, then cuts whole words.
            let first = format!("{fault} · World note · Campaign");
            assert_eq!(
                fitted_detail(ui, &first, room(&fault)),
                fault,
                "fault first"
            );
            let first_cut = fitted_detail(ui, &first, room(&fault) - 1.0);
            assert!(whole_cut(&first_cut, &fault), "{first_cut:?}");
            // A row with no fault ends on a whole word, not on its separator.
            let plain = "Monster · CR 1/4 · Small humanoid (goblinoid) · SRD 5.1";
            let plain_cut = fitted_detail(ui, plain, room("Monster · CR 1/4 · Small humanoid"));
            assert!(whole_cut(&plain_cut, plain), "{plain_cut:?}");
            // The longest realistic file name still ends on a whole word before the mark.
            let long = format!(
                "NPCs/The Venerable Watchkeeper of the Northern Gate copy.md · NPC · {fault}"
            );
            let long_cut = fitted_detail(ui, &long, room(&format!("NPCs/The Venerable… · {mark}")));
            assert_eq!(
                long_cut,
                format!("NPCs/The Venerable… · {mark}"),
                "long file"
            );
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(1);
    }

    /// The table marker stays on the World page. Lookup links there, and a session note
    /// only says so. A campaign table replaces the words in every place.
    #[test]
    fn generator_card_names_help_fits_its_place() {
        use egui_kittest::kittest::Queryable;
        let npc = crate::npc_generator::Generated {
            name: "Wren Hallam".into(),
            attributes: ["A scar".into(), "Calm".into(), "Gold".into()],
        };
        let table = vec!["npc-name: Names.md".to_owned()];
        for (help, sources, marker, short, link) in [
            (NamesHelp::Marker, vec![], true, false, false),
            (NamesHelp::Link, vec![], false, true, true),
            (NamesHelp::Plain, vec![], false, true, false),
            (NamesHelp::Link, table.clone(), false, false, false),
        ] {
            let npc = npc.clone();
            let mut harness = egui_kittest::Harness::new_ui_state(
                move |ui, pressed: &mut (bool, bool)| {
                    let choice = generator_card(
                        ui,
                        &npc,
                        &[],
                        &sources,
                        false,
                        help,
                        "People/Wren Hallam.md",
                    );
                    pressed.0 |= choice.own_names;
                    pressed.1 |= choice.copied_marker;
                },
                (false, false),
            );
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let case = format!("{help:?}");
            assert!(harness.query_by_label("Save NPC").is_some(), "{case}");
            assert_eq!(
                harness.query_by_label(OWN_NAMES_HELP).is_some(),
                marker,
                "{case}"
            );
            assert_eq!(
                harness.query_by_label(BUILT_IN_NAMES).is_some(),
                short,
                "{case}"
            );
            assert_eq!(
                harness.query_by_label(USE_OWN_NAMES).is_some(),
                link,
                "{case}"
            );
            if link {
                assert!(!harness.state().0);
                harness.get_by_label(USE_OWN_NAMES).click();
                harness.run_steps(2);
                assert!(harness.state().0, "{case}: the link reports its press");
            }
            // AD2: every card names the path it was given, which is the path the save writes.
            assert!(
                harness
                    .query_by_label("Will save People/Wren Hallam.md")
                    .is_some(),
                "{case}"
            );
            // FD3: no card shows the raw marker. Only the World card can copy it.
            assert!(
                harness.query_by_label_contains("splat:rolls").is_none(),
                "iter-15-D-FD3-hidden {case}"
            );
            assert_eq!(
                harness.query_by_label(COPY_MARKER).is_some(),
                marker,
                "{case}"
            );
            if marker {
                let will_save = "Will save People/Wren Hallam.md";
                let save_before = harness.get_by_label(will_save).rect();
                harness.get_by_label(COPY_MARKER).click();
                harness.step();
                let copied = harness.output().platform_output.commands.iter().any(
                    |command| matches!(command, egui::OutputCommand::CopyText(text) if text == NAMES_MARKER),
                );
                assert!(
                    copied,
                    "iter-15-D-FD3-copy: the action copies the exact marker"
                );
                harness.run_steps(2);
                // Round 27 FG2: the card reports the copy to its page, whose footer says which
                // dice to change; the card keeps `Copy the marker`, so the DM can copy again.
                assert!(
                    harness.state().1
                        && harness.query_by_label(MARKER_COPIED).is_none()
                        && harness.query_by_label(COPY_MARKER).is_some(),
                    "iter-27-D-FG2-card: the card reports the copy and keeps its action"
                );
                // Round 23 FC4: the save row stays.
                let save_after = harness.get_by_label(will_save).rect();
                assert!(
                    (save_after.top() - save_before.top()).abs() < 0.5,
                    "iter-23-D-FC4: {save_before:?} → {save_after:?}"
                );
            }
        }
    }

    #[test]
    fn repaired_note_has_a_distinct_name_link_and_no_false_mention() {
        let temp = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test".into(),
            path: temp.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let before = splat::content::formats::patch_model(
            &crate::library::note_text("Rook", "world", None),
            &[(
                "splat_world",
                Some(serde_yaml_ng::from_str("{type: npc}").unwrap()),
            )],
        )
        .unwrap();
        let original = Entry::parse("Rook.md", &before).unwrap();
        let id = Uuid::new_v4();
        let after =
            crate::content_workspace::new_note_identity(&before, "Rook copy.md", id).unwrap();
        std::fs::write(temp.path().join("Rook.md"), before).unwrap();
        std::fs::write(temp.path().join("Rook copy.md"), after).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert_eq!(index.items[index.by_id[&id]].entry.name, "Rook copy");
        assert!(index.mentions_of(id).is_empty());
        assert!(
            matches!(index.resolve_link("Rook", None), Go::Item(at, _) if index.items[at].entry.id == original.id)
        );
    }

    #[test]
    fn package_files_are_checked_without_an_active_campaign() {
        use std::fs;
        let folder = tempfile::tempdir().unwrap();
        let modules = vec![Module {
            id: Uuid::new_v4(),
            name: "Watch".into(),
            path: folder.path().to_owned(),
        }];
        let path = folder.path().join("Wolf.md");
        fs::write(&path, "# Wolf\n").unwrap();
        let before = content_fingerprint(&modules, None);
        fs::write(&path, "# Wolf\n\nA changed creature.\n").unwrap();
        let edited = content_fingerprint(&modules, None);
        assert_ne!(before, edited);
        fs::remove_file(path).unwrap();
        assert_ne!(edited, content_fingerprint(&modules, None));
    }

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

    fn browsed_lookup() -> Lookup {
        let mut lookup = Lookup {
            open: true,
            index: Some(Arc::new(ContentIndex::load(&[], None).unwrap())),
            ..Default::default()
        };
        lookup.navigate(0);
        lookup.navigate(1);
        lookup
    }

    #[test]
    fn forward_returns_to_what_back_left_until_another_entry() {
        let mut lookup = browsed_lookup();
        lookup.step(Step::Back);
        assert_eq!(lookup.current, Some(0));
        lookup.step(Step::Forward);
        assert_eq!(lookup.current, Some(1));
        lookup.step(Step::Back);
        lookup.navigate(2);
        assert!(
            lookup.trail.peek(Step::Forward).is_none(),
            "a new entry starts a new trail"
        );
        lookup.reading_written = Some(0);
        lookup.step(Step::Forward);
        assert_eq!(
            lookup.reading_written,
            Some(0),
            "nothing ahead keeps the note open"
        );

        // Row 17 of the session-cycle design: while an unsaved NPC is held, an entry that opens
        // takes the card's place, under the NPC's row, and Back is such an entry. The NPC stays.
        lookup.generated = Some(crate::npc_generator::Generated {
            name: "Tally".into(),
            attributes: Default::default(),
        });
        lookup.generated_shown = true;
        lookup.open_position(3);
        assert_eq!(lookup.current, Some(3));
        assert!(!lookup.generated_shown && lookup.generated.is_some());
        lookup.generated_shown = true;
        let index = lookup.index.clone().unwrap();
        lookup.open_result(&index, 3);
        assert_eq!(lookup.current, Some(3));
        assert!(!lookup.generated_shown, "the entry on screen, chosen again");
        lookup.generated_shown = true;
        lookup.step(Step::Back);
        assert_eq!(lookup.current, Some(2), "Back is as before");
        assert!(!lookup.generated_shown && lookup.generated.is_some());

        // Section 7 records these places as they are. A found note is not on Back's trail: an
        // entry that opens leaves it, and Back does not return to it.
        lookup.reading_written = Some(0);
        lookup.generated_shown = true;
        lookup.open_position(4);
        lookup.step(Step::Back);
        assert_eq!((lookup.current, lookup.reading_written), (Some(2), None));
        assert!(
            !lookup.generated_shown,
            "a found note that opens is a view too"
        );
        // Lookup closed and opened again reads the notes again, so the found note goes, and
        // the entry, the trail and the held NPC stay.
        lookup.reading_written = Some(0);
        lookup.toggle();
        lookup.toggle();
        assert!(lookup.wants_written());
        lookup.set_written(Vec::new());
        assert_eq!((lookup.current, lookup.reading_written), (Some(2), None));
        assert!(lookup.trail.peek(Step::Forward).is_some() && lookup.generated.is_some());
        // Ctrl+T and Ctrl+T again, with Lookup open, give back the found note. The shell passes
        // the notes in when Lookup asks for them, in the same frame.
        let notes = || {
            vec![Written {
                title: "Tonight".into(),
                detail: "Session".into(),
                text: "The missing captain".into(),
                place: WrittenIn::Session(Uuid::from_u128(1)),
            }]
        };
        lookup.set_written(notes());
        lookup.reading_written = Some(0);
        lookup.toggle_trackers();
        if lookup.wants_written() {
            lookup.set_written(notes());
        }
        lookup.toggle_trackers();
        assert_eq!((lookup.current, lookup.reading_written), (Some(2), Some(0)));
    }

    /// Beside the Library, Lookup takes a side button only under the pointer, and leaves
    /// the rest in the input for the Library.
    #[test]
    fn beside_the_library_a_side_button_acts_where_the_pointer_is() {
        let press = |lookup: &mut Lookup, x: f32, library_page: bool| {
            let back = history::tests::press(egui::PointerButton::Extra1, egui::pos2(x, 50.0));
            lookup.panel_rect = Some(egui::Rect::from_min_max(
                egui::pos2(300.0, 0.0),
                egui::pos2(600.0, 400.0),
            ));
            history::tests::frame(vec![back], |ctx| {
                lookup.history_input(ctx, library_page);
                history::take_if(ctx, |_| true)
            })
        };
        let mut lookup = browsed_lookup();
        assert_eq!(press(&mut lookup, 100.0, true), Some(Step::Back));
        assert_eq!(
            lookup.current,
            Some(1),
            "over the Library page, Lookup stays"
        );
        assert_eq!(press(&mut lookup, 400.0, true), None);
        assert_eq!(lookup.current, Some(0), "over Lookup, Lookup goes back");
        let mut lookup = browsed_lookup();
        assert_eq!(press(&mut lookup, 100.0, false), None);
        assert_eq!(lookup.current, Some(0), "on other pages, Lookup goes back");
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
    fn bold_words_open_a_unique_note_and_search_missing_or_ambiguous_names() {
        let (_dir, campaign) = campaign_with(&[
            ("NPCs/Docket.md", "The clerk.\n"),
            ("A/Twin.md", "One.\n"),
            ("B/Twin.md", "Two.\n"),
        ]);
        let index = Arc::new(ContentIndex::load(&[], Some(&campaign)).unwrap());
        let mut lookup = Lookup {
            index: Some(index.clone()),
            query: "old search".into(),
            ..Default::default()
        };
        lookup.open_words("docket");
        assert!(lookup.open);
        assert!(lookup.query.is_empty());
        assert!(!lookup.results_open);
        assert_eq!(
            lookup.current,
            index.position(world::note_id("NPCs/Docket.md"))
        );
        for words in ["Twin", "Unknown name"] {
            lookup.open_words(words);
            assert!(lookup.open && lookup.results_open);
            assert_eq!(lookup.query, words);
        }
    }

    #[test]
    fn copied_world_notes_stay_visible_in_world_and_lookup() {
        let id = Uuid::new_v4();
        let before = entry_note(id, "Rook", "  type: npc");
        let creature = crate::library::note_text("Guard", "creature", None);
        let (dir, campaign) = campaign_with(&[
            ("Rook.md", &before),
            ("Rook copy.md", &before),
            ("Guard.md", &creature),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let guard = index
            .items
            .iter()
            .find(|item| item.entry.path == "Guard.md")
            .unwrap();
        assert_eq!(guard.subtitle, "Creature · CR 1");
        assert!(
            !guard.result_detail().contains("note"),
            "FD6-neighbour: creature result wording stays unchanged"
        );
        assert_eq!(index.duplicate_world.len(), 2);
        for item in index.items.iter().filter(|item| item.entry.name == "Rook") {
            assert_eq!(
                item.subtitle,
                format!("NPC · {} Same note in two files", icons::WARN)
            );
            assert!(!item.subtitle.contains(&item.entry.path));
            assert!(
                item.result_detail().contains(&item.entry.path),
                "Duplicate search results still need their paths to tell them apart"
            );
        }
        assert_eq!(
            index.world_groups("").values().map(Vec::len).sum::<usize>(),
            2
        );
        assert_eq!(
            index
                .items
                .iter()
                .filter(|item| item.entry.name == "Rook")
                .count(),
            2
        );
        assert!(index.errors.iter().any(|error| {
            error.contains("These two files claim to be the same entry: Rook copy.md and Rook.md.")
        }));
        assert!(index.position(id).is_none());
        assert!(
            matches!(index.follow("Rook", &index.items[index.world[0]]), Go::Many(positions) if positions.len() == 2)
        );
        std::fs::write(dir.path().join("Guard copy.md"), &creature).unwrap();
        let copied = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let guards: Vec<_> = copied
            .items
            .iter()
            .filter(|item| ["Guard.md", "Guard copy.md"].contains(&item.entry.path.as_str()))
            .collect();
        assert_eq!(guards.len(), 2);
        for guard in guards {
            assert_eq!(
                guard.subtitle,
                format!("Creature · CR 1 · {} Same entry in two files", icons::WARN)
            );
        }
        std::fs::remove_file(dir.path().join("Guard copy.md")).unwrap();
        std::fs::write(dir.path().join("Rook third.md"), &before).unwrap();
        assert_eq!(
            ContentIndex::load(&[], Some(&campaign))
                .unwrap()
                .duplicate_world
                .len(),
            3
        );
    }

    fn campaign_with(notes: &[(&str, &str)]) -> (tempfile::TempDir, Campaign) {
        let dir = tempfile::tempdir().unwrap();
        for (path, text) in notes {
            let path = dir.path().join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        (dir, campaign)
    }

    fn entry_note(id: Uuid, name: &str, world: &str) -> String {
        splat::content::formats::fixture(
            "Fixture.md",
            &format!(
                "splat_id: {id}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n{world}\n"
            ),
            "",
        )
        .expect("schema-2 fixture")
    }

    #[test]
    fn moved_world_entries_keep_stat_block_place_and_npc_ids() {
        let npc = Uuid::new_v4();
        let place = Uuid::new_v4();
        let block = crate::library::note_text("Guard", "creature", None);
        let block_id = crate::content_workspace::note_id(&block).unwrap();
        let npc_text = entry_note(
            npc,
            "Rook",
            &format!("  type: npc\n  in: {{id: {place}}}\n  creature: {{id: {block_id}}}"),
        );
        let place_text = entry_note(place, "Watch House", "  type: location");
        let (_dir, campaign) = campaign_with(&[
            ("Rook.md", &npc_text),
            ("Watch House.md", &place_text),
            ("Guard.md", &block),
        ]);
        std::fs::create_dir(campaign.path.join("Moved")).unwrap();
        for file in ["Rook.md", "Watch House.md", "Guard.md"] {
            std::fs::rename(
                campaign.path.join(file),
                campaign.path.join("Moved").join(file),
            )
            .unwrap();
        }
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(npc).unwrap()];
        assert_eq!(item.entry.path, "Moved/Rook.md");
        assert_eq!(index.items[index.place_of(npc).unwrap()].entry.id, place);
        let block_at = index
            .position(block_id)
            .expect("the moved stat block still exists");
        assert_eq!(
            index.position_of(&splat::content::Reference::Id(block_id)),
            Some(block_at)
        );
        let catalogue = crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
        assert_eq!(
            catalogue
                .resolve_ref(&crate::content_workspace::EntryRef::id(block_id))
                .unwrap(),
            block_id
        );
        assert_eq!(
            item.entry.fields["splat_world"]["creature"]["id"].as_str(),
            Some(block_id.to_string().as_str())
        );
        assert_eq!(
            std::fs::read_to_string(campaign.path.join("Moved/Rook.md")).unwrap(),
            npc_text
        );
    }

    #[test]
    fn fmt_f1_world_references_use_file_key_id_then_unique_name() {
        let block = crate::library::note_text("Vault Guard", "creature", None);
        let block =
            splat::content::patch_fields(&block, &[("splat_key", Some("pack:guard".into()))])
                .unwrap();
        let block_id = crate::content_workspace::note_id(&block).unwrap();
        let (_dir, campaign) = campaign_with(&[("Moved/Guard.md", &block)]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        for reference in [
            "[[Moved/Guard|The Guard]]".into(),
            "pack:guard".into(),
            block_id.to_string(),
            "Vault Guard".into(),
        ] {
            let mut npc = crate::world::note_entry("Rook.md", "Rook", String::new());
            npc.fields.insert(
                "splat_world".into(),
                serde_yaml_ng::from_str(&format!("{{type: npc, creature: '{reference}'}}"))
                    .unwrap(),
            );
            assert_eq!(
                index
                    .world_target(&npc, "creature")
                    .map(|at| index.items[at].entry.id),
                Some(block_id),
                "{reference}"
            );
        }
        let mut npc = crate::world::note_entry("Rook.md", "Rook", String::new());
        npc.fields.insert(
            "splat_world".into(),
            serde_yaml_ng::from_str("{type: npc, creature: '[[Tuogh]]'}").unwrap(),
        );
        assert!(index.world_target(&npc, "creature").is_none());
    }

    #[test]
    fn moved_plain_note_changes_ephemeral_id_but_name_links_still_resolve() {
        let npc = Uuid::new_v4();
        let text = entry_note(npc, "Rook", "  type: npc\n  in: '[[Watch House]]'");
        let (_dir, campaign) =
            campaign_with(&[("Rook.md", &text), ("Watch House.md", "# Watch House\n")]);
        let before = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let old_id = before.items[before.place_of(npc).unwrap()].entry.id;
        std::fs::create_dir(campaign.path.join("Moved")).unwrap();
        std::fs::rename(
            campaign.path.join("Watch House.md"),
            campaign.path.join("Moved/Watch House.md"),
        )
        .unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let place = &index.items[index.place_of(npc).unwrap()].entry;
        assert_ne!(place.id, old_id);
        assert_eq!(place.path, "Moved/Watch House.md");
        assert!(matches!(
            index.resolve_link("Watch House", Some(npc)),
            Go::Item(..)
        ));
        std::fs::write(campaign.path.join("Watch House.md"), "Second place\n").unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert_eq!(index.place_choices(npc).len(), 2);
        // A place that waits for a choice needs fixing, but it is not missing.
        assert_eq!(
            world::broken_links(&index.items[index.position(npc).unwrap()], &index),
            [world::LinkProblem::PlaceMatches(2)]
        );
    }

    #[test]
    fn broken_link_count_ignores_resolved_and_ambiguous_links() {
        let (_dir, campaign) = campaign_with(&[("A/Twin.md", "One\n"), ("B/Twin.md", "Two\n")]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert_eq!(
            index.broken_link_count("[[Gone]] and [[Twin]] and [[A/Twin]]", Uuid::nil()),
            1
        );
    }

    #[test]
    fn campaign_aboleth_copy_has_no_false_broken_links() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let mut entry = splat::content::Entry::parse("Creatures/Watch Aboleth.md", source).unwrap();
        entry.id = Uuid::new_v4();
        entry.key = None;
        let (_dir, campaign) = campaign_with(&[]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert_eq!(index.broken_link_count(&entry.body, entry.id), 0);
        assert_eq!(
            index.broken_link_count("[[../../srd-5.2.1/pages/258|page 258]]", entry.id),
            0
        );
        assert_eq!(
            index.broken_link_count(
                &format!("{}\n[[A truly missing note]]", entry.body),
                entry.id
            ),
            1
        );
    }

    #[test]
    fn a_note_a_heading_and_past_play_are_all_found() {
        let (_dir, campaign) = campaign_with(&[
            (
                "Gate Mechanics.md",
                "# Gate Mechanics\n\n## Reliability\n\nA failure range of quartz.\n",
            ),
            (
                "Sessions/Session 1.md",
                "---\nsession_id: x\n---\nThe clerk said quartz.\n",
            ),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let names = |query: &str| -> Vec<String> {
            index
                .search(query)
                .into_iter()
                .map(|at| index.items[at].entry.name.clone())
                .collect()
        };
        assert_eq!(names("gate mechanics")[0], "Gate Mechanics");
        assert_eq!(names("reliability")[0], "Reliability");
        // A body match comes after every name match. A session note has its own results.
        assert_eq!(names("quartz"), ["Gate Mechanics"]);
        let hit = index.search("reliability")[0];
        let (note, heading) = index.target_of(hit);
        assert_eq!(index.items[note].entry.name, "Gate Mechanics");
        assert_eq!(heading.as_deref(), Some("Reliability"));
    }

    #[test]
    fn links_resolve_as_obsidian_does_and_do_not_guess() {
        let (_dir, campaign) = campaign_with(&[
            ("Setting.md", "See [[Verori#Culture]] and [[Twin]].\n"),
            ("Powers/Verori.md", "# Verori\n\n## Culture\n"),
            ("A/Twin.md", "One\n"),
            ("B/Twin.md", "Two\n"),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let at = index.position(world::note_id("Setting.md")).unwrap();
        let from = &index.items[at];
        match index.follow("Verori#Culture", from) {
            Go::Item(position, heading) => {
                assert_eq!(index.items[position].entry.path, "Powers/Verori.md");
                assert_eq!(heading.as_deref(), Some("Culture"));
            }
            _ => panic!("the note exists"),
        }
        assert!(matches!(index.follow("powers/verori", from), Go::Item(..)));
        assert!(matches!(index.follow("Twin", from), Go::Many(found) if found.len() == 2));
        assert!(matches!(index.follow("A/Twin", from), Go::Item(..)));
        assert!(
            matches!(index.follow("Ashori Deepcut", from), Go::Missing(name) if name == "Ashori Deepcut")
        );
        assert!(matches!(index.follow("Timeline#^t-1", from), Go::Timeline(id) if id == "t-1"));
        assert!(matches!(index.follow("#Anywhere", from), Go::Item(position, _) if position == at));
        // Who links to a note is listed on it.
        let verori = index.position(world::note_id("Powers/Verori.md")).unwrap();
        assert_eq!(index.mentions_of(index.items[verori].entry.id), [at]);
    }

    #[test]
    fn the_generator_reads_world_notes_and_not_session_notes() {
        let table = |name: &str| {
            format!(
                "<!-- splat:rolls key=npc-name dice=1d2 -->\n\n| d2 | Name |\n|---|---|\n| 1-2 | {name} |\n"
            )
        };
        let session = format!(
            "---\nsession_id: {}\nsession_title: ''\nsession_date: 2026-01-01\nsession_number: 1\n---\n\n{}",
            Uuid::new_v4(),
            table("Session Name")
        );
        let world = table("World Name");
        let (_dir, campaign) = campaign_with(&[
            ("Sessions/Session 1.md", session.as_str()),
            ("NPC Tables.md", world.as_str()),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let tables = index.npc_tables();
        let npc =
            crate::npc_generator::generate(&tables, &mut splat::execution::dice::Random::seeded(1))
                .unwrap();
        assert_eq!(npc.name, "World Name");
    }

    #[test]
    fn world_entries_form_a_tree_and_name_themselves_in_mentions() {
        let city = Uuid::new_v4();
        let city_note = entry_note(city, "Ixdoash", "  type: location");
        let pit = entry_note(
            Uuid::new_v4(),
            "The Pit",
            &format!("  type: location\n  in: {{id: {city}}}"),
        );
        let gravel = entry_note(
            Uuid::new_v4(),
            "Gravel",
            "  type: npc\n  aliases: [Champion]\n  status: alive",
        );
        let (_dir, campaign) = campaign_with(&[
            ("Locations/Ixdoash.md", city_note.as_str()),
            ("Locations/The Pit.md", pit.as_str()),
            ("NPCs/Gravel.md", gravel.as_str()),
            (
                "Sessions/Session 1.md",
                "---\nsession_id: x\n---\nThe champion waved.\n",
            ),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert!(index.errors.is_empty(), "{:?}", index.errors);
        let groups = index.world_groups("");
        let locations = &groups[&(1, "Locations".to_owned())];
        let rows: Vec<_> = locations
            .iter()
            .map(|&(at, depth)| (index.items[at].entry.name.as_str(), depth))
            .collect();
        assert_eq!(rows, [("Ixdoash", 0), ("The Pit", 1)]);
        let at = index.position(city).unwrap();
        let inside = index.inside(&index.items[at].entry);
        assert_eq!(index.items[inside[0]].entry.name, "The Pit");
        let gravel = groups[&(0, "NPCs".to_owned())][0].0;
        let mentions = index.mentions_of(index.items[gravel].entry.id);
        assert_eq!(index.items[mentions[0]].entry.name, "Session 1");
        assert_eq!(index.places().len(), 2);
        // Search narrows the page, and a whole note's words count.
        assert_eq!(index.world_groups("champion").len(), 1);
    }

    fn flat_note(name: &str, fields: &str, body: &str) -> String {
        format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: {name}\n{fields}\n---\n\n# {name}\n\n{body}",
            Uuid::new_v4()
        )
    }

    #[test]
    fn a_fact_has_one_home_and_shows_on_every_page_it_concerns() {
        let zarel = flat_note(
            "Zarel",
            "splat_type: npc\nsplat_in: '[[The Arena]]'\nsplat_part_of:\n- '[[Arena Guild]]'\n- '[[Nobody]]'",
            "Tiefling. The Games-Master.\n\n- **Look:** Gilded [[Horn Rings|horn rings]], loud and new.\n- **Says:** \"Filed!\"\n\nSee [[Horn Rings]].\n",
        );
        let rings = flat_note("Horn Rings", "splat_type: lore", "Rings on horns.\n");
        let guild = flat_note("Arena Guild", "splat_type: faction", "The organizers.\n");
        let arena = flat_note("The Arena", "splat_type: location", "- A list first.\n");
        let massacre = flat_note(
            "The Massacre",
            "splat_type: event\nsplat_when: about 120 years ago",
            "Soldiers fired at [[The Arena]].\n",
        );
        let revolt = flat_note(
            "The Revolt",
            "splat_type: war\nsplat_when: 90 years ago\nsplat_until: 85 years ago",
            "The Arena fell.\n",
        );
        let undated = flat_note("The Feast", "splat_type: event", "At the Arena.\n");
        let (_dir, campaign) = campaign_with(&[
            ("NPCs/Zarel.md", zarel.as_str()),
            ("Lore/Horn Rings.md", rings.as_str()),
            ("Factions/Arena Guild.md", guild.as_str()),
            ("Places/The Arena.md", arena.as_str()),
            ("History/The Revolt.md", revolt.as_str()),
            ("History/The Massacre.md", massacre.as_str()),
            ("History/The Feast.md", undated.as_str()),
            ("Setting.md", "```\n[[Horn Rings]]\n```\n"),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert!(index.errors.is_empty(), "{:?}", index.errors);
        let named = |name: &str| {
            index
                .items
                .iter()
                .position(|item| item.entry.name == name)
                .unwrap()
        };
        let id = |name: &str| index.items[named(name)].entry.id;
        let names = |positions: &[usize]| -> Vec<String> {
            positions
                .iter()
                .map(|&at| index.items[at].entry.name.clone())
                .collect()
        };

        // Lines: the fact, then the line of links. Code holds no line, only the page.
        let about = index.lines_about(id("Horn Rings"));
        let lines: Vec<_> = about
            .iter()
            .map(|mention| {
                let from = &index.items[mention.source];
                (
                    from.entry.name.as_str(),
                    mention
                        .line
                        .map(|line| (from.lines[line].words(), from.lines[line].bare)),
                )
            })
            .collect();
        assert_eq!(
            lines,
            [
                ("Setting", None),
                (
                    "Zarel",
                    Some(("Look: Gilded horn rings, loud and new.".to_owned(), false))
                ),
                ("Zarel", Some(("See Horn Rings.".to_owned(), true))),
            ]
        );
        assert_eq!(
            names(&index.mentions_of(id("Horn Rings"))),
            ["Setting", "Zarel"]
        );

        // Summary: the first paragraph, in Lookup rows and under a link.
        let item = &index.items[named("Zarel")];
        assert_eq!(item.summary(), "Tiefling. The Games-Master.");
        assert!(
            item.result_detail()
                .ends_with("· Tiefling. The Games-Master.")
        );
        assert_eq!(index.items[named("The Arena")].summary(), "");

        // Relations: written on the smaller subject, shown from both ends.
        assert_eq!(names(index.members(id("Arena Guild"))), ["Zarel"]);
        let arena = &index.items[named("The Arena")].entry;
        assert!(names(&index.inside(arena)).contains(&"Zarel".to_owned()));
        let groups = index.groups_of(&item.entry);
        assert_eq!(
            groups[0],
            ("Arena Guild".to_owned(), Some(named("Arena Guild")))
        );
        assert_eq!(groups[1], ("Nobody".to_owned(), None));

        // Events: any dated entry, oldest first, on each page it concerns.
        assert_eq!(
            names(index.history_of(id("The Arena"))),
            ["The Massacre", "The Revolt"]
        );
        let groups = index.world_groups("");
        let history: Vec<usize> = groups[&(2, HISTORY.to_owned())]
            .iter()
            .map(|&(at, _)| at)
            .collect();
        assert_eq!(names(&history), ["The Massacre", "The Revolt"]);
        assert!(groups.contains_key(&(2, "Lore".to_owned())));
        assert!(groups.contains_key(&(2, "Events".to_owned())));

        // Folders belong to the DM: a new entry goes where its type already is.
        assert_eq!(index.type_folder("location").as_deref(), Some("Places"));
        assert_eq!(index.type_folder("quest"), None);
        world::create_entry(&campaign, "Watch House", "location").unwrap();
        assert!(campaign.path.join("Places/Watch House.md").exists());
        world::create_entry(&campaign, "Find the Key", "quest").unwrap();
        assert!(campaign.path.join("Quests/Find the Key.md").exists());
    }

    #[test]
    fn a_place_inside_itself_is_reported_and_still_listed() {
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        let note = |id, name, parent: Uuid| {
            entry_note(
                id,
                name,
                &format!("  type: location\n  in: {{id: {parent}}}"),
            )
        };
        let (first, second) = (note(a, "A", b), note(b, "B", a));
        let (_dir, campaign) = campaign_with(&[
            ("Locations/A.md", first.as_str()),
            ("Locations/B.md", second.as_str()),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert_eq!(index.errors.len(), 2, "{:?}", index.errors);
        assert_eq!(
            index.world_groups("")[&(1, "Locations".to_owned())].len(),
            2
        );
    }

    #[test]
    fn a_new_name_keeps_the_dms_letters() {
        // iter-21-T-FT1: only the first letter becomes a capital, and only with no capital typed.
        for (typed, name) in [
            ("toll at the old mill", "Toll at the old mill"),
            ("  toll   at the  old mill ", "Toll at the old mill"),
            ("McTavish", "McTavish"),
            ("old McTavish", "old McTavish"),
            ("d’Arcy", "d’Arcy"),
            ("d’arcy", "D’arcy"),
            ("éowyn of rohan", "Éowyn of rohan"),
            ("3 crows inn", "3 crows inn"),
            ("   ", ""),
        ] {
            assert_eq!(typed_name(typed), name, "typed {typed:?}");
        }
        // The file name, the saved name and the heading are the name the action shows.
        let (_dir, campaign) = campaign_with(&[]);
        let name = typed_name("toll at the old mill");
        let id = world::create_entry(&campaign, &name, "npc").unwrap();
        let text = std::fs::read_to_string(campaign.path.join("NPCs/Toll at the old mill.md"))
            .expect("the file is named as the action said");
        let (entries, _) = splat::content::read_directory(&campaign.path).unwrap();
        let entry = entries.iter().find(|entry| entry.id == id).unwrap();
        assert_eq!(entry.name, "Toll at the old mill");
        assert!(
            text.contains("\nsplat_name: Toll at the old mill\n"),
            "{text}"
        );
        assert!(text.contains("\n# Toll at the old mill\n"), "{text}");
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
    #[test]
    fn words_find_a_name_in_any_order_and_with_a_typo() {
        let index = ContentIndex::load(&[], None).unwrap();
        let first = |query: &str| index.items[index.search(query)[0]].entry.name.clone();
        let potion = first("potion of healing");
        assert!(potion.starts_with("Potion of Healing"), "{potion}");
        for query in ["healing potion", "potion healing", "potoin of healng"] {
            assert_eq!(first(query), potion, "{query}");
        }
        assert!(
            index
                .search("healing")
                .into_iter()
                .any(|at| index.items[at].entry.name == potion)
        );
        // A typo comes after every name that has the word as typed.
        let fire = index.search("fire");
        let typo = fire
            .iter()
            .position(|&at| !index.items[at].lower.contains("fire"))
            .unwrap_or(fire.len());
        assert!(
            fire[typo..]
                .iter()
                .all(|&at| !index.items[at].lower.contains("fire"))
        );
    }

    #[test]
    fn what_the_dm_wrote_ranks_as_typed_before_a_typo() {
        let written = |title: &str, text: &str| {
            WrittenSearch::new(&Written {
                title: title.into(),
                detail: String::new(),
                text: text.into(),
                place: WrittenIn::Event(String::new()),
            })
        };
        let notes = [
            written("Session 3", "The smuglers paid."),
            written("Session 2", "Smugglers hid at the mill."),
            written("Smugglers at the mill", ""),
            written("Mill smugglers", ""),
        ];
        assert_eq!(
            search_written(&notes, "smugglers"),
            [(2, false), (3, false), (1, false), (0, true)]
        );
        assert_eq!(
            search_written(&notes, "mill smugglers"),
            [(3, false), (2, false), (1, false)]
        );
    }

    #[test]
    fn a_typo_never_comes_before_a_result_as_typed() {
        let hits = merge_hits(&[(5, false), (6, true)], &[(1, false), (2, true)]);
        assert_eq!(
            hits,
            [
                Hit::Entry(5),
                Hit::Written(1),
                Hit::Entry(6),
                Hit::Written(2)
            ]
        );
    }

    #[test]
    fn the_creature_picker_puts_a_typo_last() {
        let index = ContentIndex::load(&[], None).unwrap();
        let names: Vec<String> = index
            .creatures_matching("wolf", 50)
            .into_iter()
            .map(|at| index.items[at].entry.name.clone())
            .collect();
        assert_eq!(names[0], "Wolf");
        let typo = names
            .iter()
            .position(|name| !name.to_lowercase().contains("wolf"));
        if let Some(typo) = typo {
            assert!(
                names[typo..]
                    .iter()
                    .all(|name| !name.to_lowercase().contains("wolf"))
            );
        }
        let typo = index.creatures_matching("wolff", 5);
        assert!(index.items[typo[0]].entry.name.contains("Wolf"));
    }

    #[test]
    fn a_typo_in_note_text_comes_last() {
        let (_dir, campaign) = campaign_with(&[
            ("Harbor.md", "# Harbor\n\nThe smugglers hide here.\n"),
            ("Smuggler Den.md", "# Smuggler Den\n\nA quiet room.\n"),
            ("Market.md", "# Market\n\nSmuglers sell here.\n"),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let names = |query: &str| -> Vec<String> {
            index
                .search(query)
                .into_iter()
                .filter(|&at| index.items[at].is_private())
                .map(|at| index.items[at].entry.name.clone())
                .collect()
        };
        // Words as typed in a note, then a typo in a name, then a typo in a note.
        assert_eq!(names("smugglers"), ["Harbor", "Smuggler Den", "Market"]);
        assert_eq!(names("hide smugglers"), ["Harbor"]);
    }

    #[test]
    fn wiki_links_share_campaign_scope_aliases_headings_and_reference_keys() {
        let (dir, campaign) = campaign_with(&[
            ("Place.md", "# Place\n\n## Door\n"),
            ("Other/Place.md", "# Place\n"),
        ]);
        let text = crate::library::note_text("Campaign rule", "rule", None);
        let rule = Entry::parse("Rule.md", &text).unwrap();
        std::fs::write(dir.path().join("Rule.md"), text).unwrap();
        let aliased = entry_note(
            Uuid::new_v4(),
            "Harbour",
            "  type: location\n  aliases: [Docks]",
        );
        std::fs::write(dir.path().join("Harbour.md"), aliased).unwrap();
        let mut index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert!(
            matches!(index.resolve_link("Docks", None), Go::Item(at, _) if index.items[at].entry.name == "Harbour")
        );
        assert!(
            matches!(index.resolve_link("#Door", Some(rule.id)), Go::Item(at, Some(heading)) if index.items[at].entry.id == rule.id && heading == "Door")
        );
        assert!(
            matches!(index.resolve_link("Place#Door", Some(rule.id)), Go::Many(found) if found.len() == 2)
        );
        assert!(matches!(
            index.resolve_link("Campaign rule|these words", None),
            Go::Item(..)
        ));
        assert!(matches!(
            index.resolve_link("dnd2024:spells/fire_bolt", None),
            Go::Item(..)
        ));
        let at = index
            .items
            .iter()
            .position(|item| item.entry.name == "Campaign rule")
            .unwrap();
        index.usable.as_mut().unwrap().remove(&rule.id);
        assert!(matches!(
            index.resolve_link("Campaign rule", None),
            Go::Missing(_)
        ));
        assert!(
            matches!(index.resolve_link(&rule.id.to_string(), None), Go::Item(position, _) if position == at)
        );
        let rendered = index.reader_markdown(
            "[[Gone|my words]] and [[Place#Door|the door]] and `[[Code]]`",
            None,
        );
        assert!(rendered.contains("[my words](<splat-missing:Gone>)"));
        assert!(rendered.contains("[the door](<splat-note:Place#Door>)"));
        assert!(rendered.contains("`[[Code]]`"));
    }

    /// Link names come from a map built with the index. They must find exactly what a search
    /// of every item found, whatever the case, file name, alias or campaign scope.
    #[test]
    fn link_names_find_what_a_search_of_every_item_finds() {
        fn search(index: &ContentIndex, key: &str) -> Vec<usize> {
            index
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    if matches!(item.source, Source::Heading(..)) {
                        return false;
                    }
                    if !item.is_world_note()
                        && index
                            .usable
                            .as_ref()
                            .is_some_and(|usable| !usable.contains(&item.entry.id))
                    {
                        return false;
                    }
                    item.lower == key
                        || item.file_keys().contains(&key.to_owned())
                        || world::aliases(&item.entry)
                            .iter()
                            .any(|alias| alias.eq_ignore_ascii_case(key))
                })
                .map(|(at, _)| at)
                .collect()
        }
        let (dir, campaign) = campaign_with(&[
            ("Place.md", "# Place\n\n## Door\n"),
            ("Other/Place.md", "# Place\n"),
            ("Wolf.md", "# Wolf\n\nOur own wolf.\n"),
        ]);
        let text = crate::library::note_text("Campaign rule", "rule", None);
        let rule = Entry::parse("Rule.md", &text).unwrap();
        std::fs::write(dir.path().join("Rule.md"), text).unwrap();
        for (name, aliases) in [
            ("Harbour", "[Docks, The OLD Quay]"),
            ("Élan Keep", "[ÉLAN, Keep of Élan]"),
            ("Twin", "[Harbour]"),
        ] {
            std::fs::write(
                dir.path().join(format!("{name}.md")),
                entry_note(
                    Uuid::new_v4(),
                    name,
                    &format!("  type: location\n  aliases: {aliases}"),
                ),
            )
            .unwrap();
        }
        let mut index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        index.usable.as_mut().unwrap().remove(&rule.id);
        let mut keys = vec![
            "missing".to_owned(),
            "door".to_owned(),
            "élan".to_owned(),
            "the old quay".to_owned(),
        ];
        for item in &index.items {
            keys.push(item.lower.clone());
            keys.extend(item.file_keys());
            keys.extend(
                world::aliases(&item.entry)
                    .into_iter()
                    .map(str::to_lowercase),
            );
        }
        let mut shared = 0;
        for key in &keys {
            let found = index.named(key);
            assert_eq!(found, search(&index, key), "{key:?}");
            shared += usize::from(found.len() > 1);
        }
        assert!(shared > 0, "Some names must lead to more than one item");
        assert!(index.named("campaign rule").is_empty());
        assert!(index.named("door").is_empty());
    }

    #[test]
    fn a_result_links_by_name_when_that_finds_it_and_exactly_when_not() {
        let (_dir, campaign) = campaign_with(&[
            (
                "Powers/Verori.md",
                "# Verori\n\n## Culture\n\n## Fire | Ice\n",
            ),
            ("A/Twin.md", "One\n"),
            ("B/Twin.md", "Two\n"),
        ]);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let link = |query: &str| index.link_to(index.search(query)[0]);
        assert_eq!(link("verori"), "[[Verori]]");
        assert_eq!(link("culture"), "[[Verori#Culture]]");
        // A link cannot name this heading, so it names the note.
        assert_eq!(
            index.items[index.search("fire | ice")[0]].entry.name,
            "Fire | Ice"
        );
        assert_eq!(link("fire | ice"), "[[Verori]]");
        let twin = index.position(world::note_id("A/Twin.md")).unwrap();
        assert_eq!(index.link_to(twin), "[[A/Twin|Twin]]");
        // Every result's link leads back to it, a book's too.
        for query in [
            "verori", "culture", "room", "twin", "fireball", "wolf", "goblin",
        ] {
            for position in index.search(query).into_iter().take(5) {
                let link = index.link_to(position);
                let target = link.trim_start_matches("[[").trim_end_matches("]]");
                let (want, heading) = index.target_of(position);
                let heading = heading.filter(|heading| !heading.contains('|'));
                match index.resolve_link(target, None) {
                    Go::Item(at, found) => {
                        assert_eq!((at, found), (want, heading), "{link}");
                    }
                    _ => panic!("{link} does not lead to its result"),
                }
            }
        }
    }
}
