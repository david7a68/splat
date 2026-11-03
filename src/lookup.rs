//! Lookup: search and read any entry beside the current page, without navigating.
//! It covers built-in, package and campaign-owned entries in one index (T2.3), and the
//! campaign's plain notes, with each heading as its own result.
use crate::{
    campaigns::Campaigns,
    content_workspace::Catalogue,
    design::{self, Action, Density, Tone, Type, colors, icons, reader as style},
    library::{presentation, view},
    persistence::{Campaign, Module},
    world::{self, Go, Nav, Reader, Role},
};
use eframe::egui;
use splat::content::{Entry, Reference};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::{Arc, mpsc},
};
use uuid::Uuid;

const RESULT_ROW: f32 = design::XXL;
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
    names_lower_word(&text.to_lowercase(), &name.to_lowercase())
}

/// `names_word`, for words already in lower case.
fn names_lower_word(text: &str, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let boundary = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric());
    text.match_indices(name).any(|(at, _)| {
        boundary(text[..at].chars().next_back()) && boundary(text[at + name.len()..].chars().next())
    })
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
    /// For notes and world entries, whose words can also be searched.
    lower_body: String,
}

/// Every entry the active campaign can see, including all registered packages.
#[derive(Default)]
pub struct ContentIndex {
    pub items: Vec<Item>,
    usable: Option<HashSet<Uuid>>,
    package_owners: HashMap<Uuid, Uuid>,
    by_id: HashMap<Uuid, usize>,
    by_key: HashMap<String, usize>,
    spells: Vec<usize>,
    creatures: Vec<usize>,
    /// Lower-case file stems and paths in the campaign, as Obsidian finds a link's target.
    files: HashMap<String, Vec<usize>>,
    /// Plain world notes and world entries.
    world: Vec<usize>,
    mentions: HashMap<Uuid, Vec<usize>>,
    /// What the campaign's files looked like just before this was read.
    files_seen: u64,
    pub errors: Vec<String>,
    pub broken_world: Vec<(String, String)>,
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

    pub fn is_note(&self) -> bool {
        matches!(self.source, Source::Note(_))
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

    fn result_detail(&self) -> String {
        let source = if self.private {
            Some("Campaign")
        } else if self.builtin {
            None
        } else {
            Some(self.owner.as_str())
        };
        presentation::list_subtitle(&self.subtitle, source)
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
    let item = |entry: Entry, subtitle: String, lower_body: String, source: Source| Item {
        lower: entry.name.to_lowercase(),
        subtitle,
        owner: owner.to_owned(),
        private: true,
        section: presentation::Section::World,
        builtin: false,
        from_sheet: false,
        source,
        lower_body,
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
    let main = item(
        world::note_entry(&note.path, &note.name, body),
        subtitle.to_owned(),
        note.body.to_lowercase(),
        Source::Note(note.role),
    );
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

impl ContentIndex {
    fn load(modules: &[Module], campaign: Option<&Campaign>) -> Result<Self, String> {
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
                source: Source::Entry,
                // Another name finds a world entry as its words do.
                lower_body: if e.entry.kind == "world" {
                    format!("{} {}", world::aliases(&e.entry).join(" "), e.entry.body)
                        .to_lowercase()
                } else {
                    String::new()
                },
                entry: e.entry.clone(),
            })
            .collect();
        let files_seen = content_fingerprint(modules, campaign);
        if let Some(campaign) = campaign {
            let sessions = crate::sessions::folder(campaign)
                .unwrap_or_else(|_| campaign.path.join(crate::sessions::DEFAULT_FOLDER));
            let scan = world::scan(&campaign.path, &sessions);
            for item in &mut items {
                if item.entry.kind == "world" {
                    item.entry.body = world::with_links(&item.entry.body, &scan.images);
                }
            }
            let owner = owner_name(campaign.id, true);
            for note in scan.notes {
                items.extend(note_items(note, &owner, &scan.images));
            }
        }
        items.sort_by(|a, b| a.lower.cmp(&b.lower).then(a.entry.id.cmp(&b.entry.id)));
        let mut index = Self {
            usable,
            package_owners: catalogue
                .entries
                .iter()
                .map(|(id, entry)| (*id, entry.owner))
                .collect(),
            errors: catalogue.errors.clone(),
            files_seen,
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
            index.broken_world = splat::content::read_directory(&campaign.path)?
                .1
                .into_iter()
                .map(|error| (error.path, error.message))
                .collect();
        }
        index.items = items;
        index.mentions = index.find_mentions();
        index.errors.extend(index.place_loops());
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

    /// Who links to each note or entry, and who names each world entry in whole words.
    fn find_mentions(&self) -> HashMap<Uuid, Vec<usize>> {
        let mut found: HashMap<Uuid, Vec<usize>> = HashMap::new();
        let sources: Vec<(usize, Vec<String>)> = (0..self.items.len())
            .filter(|&at| !self.items[at].lower_body.is_empty())
            .map(|at| (at, world::links_in(&self.items[at].lower_body)))
            .collect();
        for &target in &self.world {
            let item = &self.items[target];
            let keys = item.file_keys();
            let mut names = Vec::new();
            if item.entry.kind == "world" {
                names.push(item.entry.name.to_lowercase());
                names.extend(world::aliases(&item.entry).iter().map(|a| a.to_lowercase()));
            }
            for (source, links) in &sources {
                let body = &self.items[*source].lower_body;
                if *source != target
                    && (links.iter().any(|link| keys.contains(link))
                        || names.iter().any(|name| names_lower_word(body, name)))
                {
                    found.entry(item.entry.id).or_default().push(*source);
                }
            }
        }
        found
    }

    fn parent(&self, position: usize) -> Option<usize> {
        let entry = &self.items[position].entry;
        if let Some(reference) = world::place(entry) {
            return self.position_of(&reference);
        }
        let link = entry.fields.get("splat_world")?.get("in")?.as_str()?;
        let target = link
            .strip_prefix("[[")?
            .strip_suffix("]]")?
            .split('|')
            .next()?
            .trim();
        match self.follow(target, &self.items[position]) {
            Go::Item(at, _) => Some(at),
            _ => None,
        }
    }

    pub fn place_of(&self, id: Uuid) -> Option<usize> {
        self.parent(self.position(id)?)
    }

    fn place_loops(&self) -> Vec<String> {
        self.world
            .iter()
            .filter(|&&start| {
                std::iter::successors(self.parent(start), |&at| self.parent(at))
                    .take(self.world.len())
                    .any(|at| at == start)
            })
            .map(|&start| {
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
            .collect()
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
        let key = name.trim_end_matches(".md").to_lowercase();
        let found: Vec<_> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                if matches!(item.source, Source::Heading(..)) {
                    return false;
                }
                if !item.is_world_note()
                    && self
                        .usable
                        .as_ref()
                        .is_some_and(|usable| !usable.contains(&item.entry.id))
                {
                    return false;
                }
                item.lower == key
                    || item.file_keys().contains(&key)
                    || world::aliases(&item.entry)
                        .iter()
                        .any(|alias| alias.eq_ignore_ascii_case(&key))
            })
            .map(|(at, _)| at)
            .collect();
        match found.as_slice() {
            &[at] => Go::Item(at, heading),
            [] => Go::Missing(name.into()),
            _ => Go::Many(found),
        }
    }

    /// Prepare wiki links in any reader, retaining exactly the note's display words.
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

    pub fn mentions_of(&self, id: Uuid) -> &[usize] {
        self.mentions.get(&id).map_or(&[], Vec::as_slice)
    }

    /// Creatures by name, exact matches first and then each word's start.
    pub fn creatures_matching(&self, query: &str, limit: usize) -> Vec<usize> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }
        let mut found: Vec<(u8, usize)> = self
            .creatures
            .iter()
            .filter_map(|&at| {
                let name = &self.items[at].lower;
                name.contains(&query).then(|| {
                    let rank = u8::from(*name != query) + u8::from(!name.starts_with(&query));
                    (rank, at)
                })
            })
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

    /// Best first: exact name, then a prefix, then a word, then anywhere in the name. A note's
    /// words come last.
    fn search(&self, query: &str) -> Vec<usize> {
        let mut found: Vec<(u8, Origin, usize)> = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                // What the DM wrote in a session has its own results, from the session page.
                if matches!(item.source, Source::Note(Role::Session | Role::Transcript)) {
                    return None;
                }
                let rank = if item.lower == query {
                    0
                } else if item.lower.starts_with(query) {
                    1
                } else if item.lower.contains(query) {
                    if names_lower_word(&item.lower, query) {
                        2
                    } else {
                        3
                    }
                } else if item.section.label().to_lowercase() == query {
                    4
                } else if item.lower_body.contains(query) {
                    5
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
    campaign: Option<Campaign>,
    pub reader: Reader,
    create_error: Option<String>,
    /// What the generator rolled, until the DM saves it or lets it go.
    generated: Option<crate::npc_generator::Generated>,
    /// Campaign tables the generator could not use.
    generator_problems: Vec<String>,
    generator_sources: Vec<String>,
    pub open_world_repair: Option<String>,
    /// A check of campaign and package files, away from the frame.
    pub files_changed: bool,
    checking: Option<mpsc::Receiver<u64>>,
    was_focused: bool,
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
        self.focus_search = self.open;
        self.results_open = self.current.is_none() || !self.query.is_empty();
    }

    pub fn close(&mut self) {
        self.open = false;
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
        self.written_stale = true;
        self.reading_written = None;
        self.navigate(position);
        self.results_open = false;
        self.query.clear();
    }

    /// Open with this search, for a link to something that does not exist yet.
    pub fn search_for(&mut self, words: &str) {
        self.open = true;
        self.query = words.to_owned();
        self.focus_search = true;
        self.results_open = true;
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

    /// A creature opened from the encounter search keeps its result selected.
    pub fn open_creature_result(&mut self, id: Uuid) {
        self.open_id(id);
        if let Some(item) = self
            .index
            .as_ref()
            .and_then(|index| index.position(id).map(|at| &index.items[at]))
        {
            self.query = item.entry.name.clone();
            self.results_open = true;
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

    /// Go to a result. A heading's result opens its note at that heading.
    fn open_result(&mut self, index: &ContentIndex, position: usize) {
        let (position, heading) = index.target_of(position);
        if self.current != Some(position) {
            self.navigate(position);
        }
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
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("reader_surface"), egui::Id::new("lookup"))
        });
        let mut close = false;
        ui.horizontal(|ui| {
            ui.label(Type::Heading.rich(ui, "Lookup"));
            if !self.history.is_empty()
                && Action::quiet("Back").icon(icons::BACK).show(ui).clicked()
            {
                self.back();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                close = design::icon_button(ui, icons::CLOSE, "Close Lookup").clicked();
                if let Some(current) = self
                    .index
                    .as_ref()
                    .and_then(|index| self.current.and_then(|at| index.items.get(at)))
                    && !current.is_note()
                    && design::locked(
                        ui,
                        self.library_draft
                            .then_some("Save or discard the Library draft first."),
                        |ui| design::icon_button(ui, icons::OPEN_OUT, "Open in Library").clicked(),
                    )
                {
                    self.open_in_library = Some(current.entry.clone());
                }
            });
        });
        if close {
            self.close();
        }
        let width = ui.available_width();
        let before = self.query.clone();
        let show_clear = !self.query.is_empty();
        let response = ui
            .horizontal(|ui| {
                let response = design::search_field(
                    ui,
                    SEARCH_NAME,
                    &mut self.query,
                    if width < design::XXL * 10.0 {
                        "Search…"
                    } else {
                        "Search entries and notes…"
                    },
                    width
                        - if show_clear {
                            Action::quiet("Clear").natural_width(ui) + ui.spacing().item_spacing.x
                        } else {
                            0.0
                        },
                );
                if show_clear && Action::quiet("Clear").show(ui).clicked() {
                    self.query.clear();
                    self.current = None;
                    self.reader = Reader::default();
                    self.reading_written = None;
                    self.generated = None;
                }
                response
            })
            .inner;
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
        if !index.broken_world.is_empty() {
            ui.horizontal_wrapped(|ui| {
                design::status(
                    ui,
                    design::Status {
                        icon: icons::WARN,
                        word: &format!(
                            "{} cannot be read",
                            design::count(index.broken_world.len(), "note", "notes")
                        ),
                        color: colors().result.warning,
                    },
                );
                if design::link(ui, "Show").clicked() {
                    self.open_world_repair = Some(index.broken_world[0].0.clone());
                }
            });
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
            // Open the best match and keep the other answers in view.
            self.results_open = true;
            match matches.first() {
                Some(Hit::Entry(top)) => {
                    let (position, heading) = index.target_of(*top);
                    self.current = Some(position);
                    self.reader.show_heading(heading);
                    self.reading_written = None;
                }
                Some(Hit::Written(top)) => self.reading_written = Some(*top),
                None => {}
            }
        }
        if let Some(error) = &self.create_error {
            design::message(ui, Tone::Error, error);
        }
        let matches = &self.matches.as_ref().unwrap().1;
        let mut chosen = None;
        if !query.is_empty()
            && !matches.is_empty()
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
            if matches.is_empty() {
                ui.label(
                    Type::Body
                        .rich(ui, "Nothing found. Check the spelling, or try fewer words.")
                        .color(colors().muted),
                );
            }
            // The reader stays the larger part of the panel, even at the minimum window.
            let rows = matches.len().min(5);
            let results: Vec<_> = matches
                .iter()
                .take(rows)
                .enumerate()
                .map(|(row, hit)| match *hit {
                    Hit::Entry(position) => {
                        let item = &index.items[position];
                        (
                            item.entry.name.clone(),
                            item.result_detail(),
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
            egui::ScrollArea::vertical()
                .id_salt("lookup_results")
                .max_height(RESULT_ROW * rows as f32)
                .auto_shrink([false, true])
                .show_rows(ui, RESULT_ROW, rows, |ui, range| {
                    for row in range {
                        let (title, detail, selected) = &results[row];
                        let full = format!("{title} · {detail}");
                        let response = design::row(ui, RESULT_ROW, *selected, &full, |ui, rect| {
                            let name = ui.painter().layout_no_wrap(
                                title.clone(),
                                Type::Body.font(ui),
                                colors().text,
                            );
                            let tail = design::one_line(
                                ui,
                                &format!(" · {detail}"),
                                Type::Small.font(ui),
                                colors().muted,
                                (rect.width() - name.size().x).max(0.0),
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
                        if response.clicked() {
                            chosen = Some(matches[row]);
                        }
                    }
                });
            ui.spacing_mut().item_spacing.y = design::XS;
        }
        if show_results && matches.len() > 5 {
            ui.menu_button(
                format!("Show all {}", design::count(matches.len(), "", "").trim()),
                |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(design::READER_MEASURE)
                        .show(ui, |ui| {
                            for hit in matches {
                                let name = match hit {
                                    Hit::Entry(at) => &index.items[*at].entry.name,
                                    Hit::Written(at) => &self.written[*at].title,
                                };
                                if design::link(ui, name).clicked() {
                                    chosen = Some(*hit);
                                    ui.close();
                                }
                            }
                        });
                },
            );
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
                Hit::Written(at) => self.reading_written = Some(at),
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
            ui.horizontal_wrapped(|ui| {
                if !query.is_empty()
                    && !index
                        .items
                        .iter()
                        .any(|item| item.entry.name.eq_ignore_ascii_case(&query))
                    && !self
                        .written
                        .iter()
                        .any(|item| item.title.eq_ignore_ascii_case(&query))
                {
                    let name = self
                        .query
                        .split_whitespace()
                        .map(|word| {
                            let mut chars = word.chars();
                            chars.next().map_or_else(String::new, |first| {
                                first.to_uppercase().collect::<String>()
                                    + &chars.as_str().to_lowercase()
                            })
                        })
                        .collect::<Vec<_>>()
                        .join(" ");
                    for (word, kind) in [("New NPC", "npc"), ("New location", "location")] {
                        let label = format!("{word} \"{name}\"");
                        if Action::quiet(&label)
                            .icon(icons::ADD)
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            made = Some(world::create_entry(&campaign, &name, kind));
                        }
                    }
                }
                generate = query.is_empty()
                    && self.current.is_none()
                    && self.generated.is_none()
                    && Action::quiet("Generate NPC")
                        .icon(icons::DICE)
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
            });
            let mut reroll = generate;
            if let Some(npc) = &self.generated {
                let (saved, again, discard) = generated_card(
                    ui,
                    npc,
                    &self.generator_problems,
                    &self.generator_sources,
                    self.run_open,
                );
                reroll = again;
                if saved {
                    made = Some(world::create_generated(&campaign, npc));
                }
                if discard {
                    self.generated = None;
                    self.create_error = None;
                }
            }
            if reroll {
                let tables = index.npc_tables();
                match crate::npc_generator::roll(&tables) {
                    Ok(npc) => {
                        self.generated = Some(npc);
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
        if let Some(at) = self.reading_written {
            self.written_reader(ui, at, &query);
            return;
        }
        if self.generated.is_some() {
            return;
        }
        if self.current.is_none() {
            let _ = self.reader.choices(ui, &index).map(|nav| self.follow(nav));
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
        let mut nav = self.reader.choices(ui, &index);
        if ui.rect_contains_pointer(ui.available_rect_before_wrap())
            && ui.input(|input| input.smooth_scroll_delta.y != 0.0)
        {
            self.results_open = false;
        }
        self.reader.compact = true;
        let (reader, campaign) = (&mut self.reader, self.campaign.as_ref());
        egui::ScrollArea::vertical()
            .id_salt(("lookup_reader", current.entry.id))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let read = reader.read(ui, current, &index, campaign);
                nav = nav.take().or(read);
            });
        if let Some(nav) = nav {
            self.follow(nav);
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

/// What the generator rolled, with its choices. Returns save, reroll and discard.
pub(crate) fn generated_card(
    ui: &mut egui::Ui,
    npc: &crate::npc_generator::Generated,
    problems: &[String],
    sources: &[String],
    run_open: bool,
) -> (bool, bool, bool) {
    design::group_label(ui, "Generated NPC");
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
        ui.label(
            Type::Small
                .rich(
                    ui,
                    "Built-in names. Mark a table npc-name in any note to use your own.",
                )
                .color(colors().muted),
        );
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
    for problem in problems {
        design::message(ui, Tone::Warning, problem);
        if let Some((path, _)) = problem.split_once(':')
            && design::link(ui, &format!("Open {path}")).clicked()
        {
            ui.ctx().data_mut(|data| {
                data.insert_temp(egui::Id::new("markdown_clicked_note"), path.to_owned())
            });
        }
    }
    let (mut saved, mut again, mut discard) = (false, false, false);
    ui.horizontal_wrapped(|ui| {
        saved = (if run_open {
            Action::secondary("Save NPC")
        } else {
            Action::primary("Save NPC")
        })
        .density(Density::Read)
        .show(ui)
        .clicked();
        again = Action::secondary("Reroll")
            .icon(icons::DICE)
            .density(Density::Read)
            .show(ui)
            .clicked();
        discard = Action::quiet("Discard")
            .density(Density::Read)
            .show(ui)
            .clicked();
    });
    (saved, again, discard)
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
        format!(
            "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n{world}\n---\n"
        )
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
}
