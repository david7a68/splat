//! The Library page as data and acts, for a UI that draws it itself (the GPUI port,
//! `docs/gpui-port.md`). The egui page (`Library::show`) shares the list's logic with it
//! (`begin`, `section_counts`, `find_matches`, `row_subtitle`, `list_empty_words`); the reader's
//! data here follows `Library::editor_pane` step by step, and its footer lines are the same typed
//! lines (`design::outcome::port`). The cleanup wave deletes the egui page and keeps this.
use super::*;
pub use super::presentation::Section;
pub use crate::design::outcome::port::{Kind as ControlKind, Line as FooterLine};
pub use crate::matching::CLOSE_NAMES;
use crate::design::outcome::Fault as _;
use std::sync::Arc;

// ---------------------------------------------------------------------------------------------
// The list's logic, shared by both UIs.

/// Each section's count of the notes in `scope` (`All` counts the whole scope).
pub(super) fn section_counts(loaded: &Loaded, scope: Scope) -> Vec<(Section, usize)> {
    let notes = &loaded.notes;
    Section::GROUPS
        .iter()
        .flat_map(|(_, sections)| sections.iter())
        .map(|&section| {
            (
                section,
                notes
                    .iter()
                    .filter(|n| {
                        loaded.in_scope(n, scope)
                            && (section == Section::All || n.section() == section)
                    })
                    .count(),
            )
        })
        .collect()
}

/// The notes the list shows for `query` in `scope`'s `section` (an unreadable `kind` in the
/// Cannot be read section) with `filter`, best first, and where the matches found only with a
/// typo start. A query always searches the whole scope.
pub(super) fn find_matches(
    loaded: &Loaded,
    (scope, section, unreadable_kind): (Scope, Section, Option<Section>),
    query: &str,
    filter: &Filter,
) -> (Vec<usize>, usize) {
    let filtering = (section == Section::Creatures || !query.is_empty()) && !filter.is_empty();
    let parsed = Query::parse(query);
    let typed = Typed::new(&parsed.words);
    let facets = loaded.facets();
    let mut found: Vec<(Rank, usize)> = loaded
        .notes
        .iter()
        .enumerate()
        .filter(|(i, n)| {
            loaded.in_scope(n, scope)
                && (!filtering || facets[*i].as_ref().is_some_and(|f| filter.matches(f)))
                && (!query.is_empty()
                    || section != Section::Unreadable
                    || unreadable_kind.is_none_or(|kind| n.unreadable_kind() == kind))
        })
        .filter_map(|(i, n)| {
            n.search_rank(section, &parsed, &typed, facets[i].as_ref())
                .map(|rank| (rank, i))
        })
        .collect();
    if !parsed.words.is_empty() {
        // Best first: the exact name, then a prefix, then a whole word, then each word in any
        // order (a creature's whole word and a word that only holds the search are one tier,
        // M3). A path, section or fact match comes next, and a typo comes last. The sort is
        // stable, so the list's order (by name) stays inside each rank and origin.
        found.sort_by_key(|&(rank, i)| {
            (
                rank,
                crate::lookup::entry_origin(
                    loaded.notes[i].entry.as_ref(),
                    matches!(loaded.notes[i].owner, Owner::Campaign(_)),
                ),
            )
        });
    }
    let typo_from = found
        .iter()
        .position(|(rank, _)| rank.typo)
        .unwrap_or(found.len());
    (found.into_iter().map(|(_, i)| i).collect(), typo_from)
}

/// An entry row's second line: its kind line without the scope's own owner, led by its table
/// faults or its broken links when it has any.
pub fn row_subtitle(
    loaded: &Loaded,
    note: &Note,
    line: &str,
    scope: Scope,
    index: Option<&crate::lookup::ContentIndex>,
) -> String {
    let subtitle = match (&note.owner, scope) {
        (Owner::Campaign(_), Scope::Campaign) => line.strip_suffix(" · Campaign").unwrap_or(line),
        (Owner::Package(module), Scope::Package(id)) if module.id == id => line
            .strip_suffix(&format!(" · {}", module.name))
            .unwrap_or(line),
        _ => line,
    };
    let broken = note
        .entry
        .as_ref()
        .zip(index)
        .filter(|_| note.owner != Owner::Builtin)
        .map_or(0, |(entry, index)| index.broken_link_count(&entry.body, entry.id));
    let faults = note
        .entry
        .as_ref()
        .map_or(0, |entry| crate::roll_table::fault_count(&entry.body));
    if faults > 0 {
        format!(
            "{} {} · {subtitle}",
            icons::WARN,
            design::count(faults, "table fault", "table faults")
        )
    } else if broken > 0 {
        // One fault gets one mark: when the file's every counted problem is a missing link,
        // the links' count is its only cause.
        let needs_fixing = format!("{} {NEEDS_FIXING} · ", icons::WARN);
        let rest = subtitle
            .strip_prefix(&needs_fixing)
            .filter(|_| loaded.link_faults_only.contains(&Loaded::file_key(note)))
            .unwrap_or(subtitle);
        format!(
            "{} {} · {rest}",
            icons::WARN,
            design::count(broken, "broken link", "broken links")
        )
    } else {
        subtitle.to_owned()
    }
}

/// The empty list's sentence, and whether `Search Everything` follows it (a wider scope can find
/// more).
pub fn list_empty_words(
    campaigns: &Campaigns,
    scope: Scope,
    filtered: bool,
    searching: bool,
) -> (String, bool) {
    let wider = (filtered || searching) && scope != Scope::All;
    let text = if filtered && wider {
        format!(
            "No creatures match these filters in {}. Remove a filter, or search Everything.",
            scope_name(campaigns, scope)
        )
    } else if filtered {
        "No creatures match these filters. Remove a filter.".to_owned()
    } else if scope == Scope::Campaign && !searching {
        "No campaign entries yet. Copy an entry here with Copy to…, or create one with New entry."
            .to_owned()
    } else if searching && wider {
        format!(
            "No entries match in {}. Try fewer words, or search Everything.",
            scope_name(campaigns, scope)
        )
    } else if searching {
        "No entries match. Try fewer words.".to_owned()
    } else {
        "No entries in this section.".to_owned()
    };
    (text, wider)
}

/// A tone's icon and colour (egui's, for `bridge::color`).
pub fn tone_parts(tone: Tone) -> (&'static str, egui::Color32) {
    let result = colors().result;
    match tone {
        Tone::Info => (icons::INFO, result.info),
        Tone::Success => (icons::SUCCESS, result.success),
        Tone::Warning => (icons::WARN, result.warning),
        Tone::Error => (icons::ERROR, result.error),
    }
}

/// A package form's reason, in its words.
fn reason_words(reason: PackageReason) -> &'static str {
    design::layout::CreateReasons::words(reason)
}

/// The words of a fault as its typed message states them, and its tone.
pub fn fault_line(fault: &LibraryFault<'_>) -> (Tone, String) {
    (
        fault.severity().tone(),
        design::outcome::fault_words(fault).as_str().to_owned(),
    )
}

// ---------------------------------------------------------------------------------------------
// The page's views.

/// The header's issues count and its menu.
pub struct Issues {
    pub word: String,
    /// A counted file cannot be read: the error tone.
    pub unreadable: bool,
    /// The faults' words, then a stopped read's.
    pub lines: Vec<String>,
    /// Each file with broken links: its note and its words.
    pub links: Vec<(usize, String)>,
    /// While a draft holds the open entry, no other entry opens: why.
    pub open_lock: Option<String>,
}

/// The page's header line and the line under it.
pub struct Header {
    /// `N entries`, when any.
    pub context: Option<String>,
    /// `New entry`, when it shows, with why it waits.
    pub new_entry: Option<Option<String>>,
    /// Why `Packages` waits.
    pub packages_blocked: Option<String>,
    pub issues: Option<Issues>,
    /// With no campaign open: its words and its action's.
    pub no_campaign: Option<(String, &'static str)>,
}

/// One row of the sections column.
pub struct SectionRow {
    pub label: &'static str,
    pub count: usize,
    /// Whether the count is painted (All entries paints none without a search).
    pub painted: bool,
    pub selected: bool,
    pub section: Section,
    /// An unreadable kind, under Cannot be read.
    pub kind: Option<Section>,
}

/// A group of the sections column: its heading (none for All entries), its count and its rows.
pub struct SectionGroup {
    pub title: &'static str,
    pub count: usize,
    pub rows: Vec<SectionRow>,
}

/// What the entry list shows above its rows when it has none.
pub enum EmptyList {
    /// A package with no entries yet (`No entries in <name> yet`), and whether the New entry
    /// form is open (it shows only the title then).
    Package { name: String, form: bool },
    /// Why nothing matches, and whether `Search Everything` follows.
    Words { words: String, wider: bool },
}

/// The list column's state this frame.
pub struct ListView {
    pub about: Option<(usize, String, bool)>,
    pub groups: Vec<SectionGroup>,
    /// The section drop-down of a compact page: each section with a count, its full and short
    /// label, and the chosen one.
    pub section_choices: Vec<(Section, &'static str)>,
    pub chosen_section: Option<Section>,
    /// The heading over the rows (`Search results`, `Matches`, a section), with the count.
    pub heading: Option<(&'static str, usize)>,
    pub close_names: bool,
    pub empty: Option<EmptyList>,
    pub rows: usize,
    pub selected_row: Option<usize>,
    /// The list should scroll to the selected row (it arrived by a link).
    pub reveal: bool,
    pub creatures_or_search: bool,
    /// The list is hidden: a changed file or a narrow draft needs the room.
    pub list_hidden: bool,
}

/// An entry row: the note, its name, its second line and its hover.
pub struct EntryRow {
    pub note: usize,
    pub name: String,
    pub subtitle: String,
    pub hover: String,
    pub selected: bool,
}

/// What the reader's toolbar holds.
pub struct Toolbar {
    pub source: bool,
    pub source_label: &'static str,
    /// The mode switch waits: the body states the file's fault.
    pub mode_blocked: Option<String>,
    pub back: bool,
    /// Each extra: its words, what it does and why it waits.
    pub actions: Vec<(&'static str, ReaderAct, Option<String>)>,
    /// Steady causes said once under the toolbar.
    pub reasons: Vec<String>,
    /// The draft's reason on the toolbar's line, with no save row.
    pub draft_reason: Option<String>,
}

/// A reader toolbar extra.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReaderAct {
    Copy,
    Make,
    Choose,
}

/// The owner and file line.
pub struct FileLine {
    /// The owner's status: icon, words, colour (egui's, for `bridge::color`).
    pub status: (&'static str, String, egui::Color32),
    /// The folder and the shown path, when the owner has a folder.
    pub file: Option<(PathBuf, String)>,
    /// `moved from <folder>/`.
    pub moved_from: Option<String>,
    pub obsidian: bool,
    /// Only the folder can be shown (a deleted manifest, a file gone from disk).
    pub folder_only: bool,
}

/// The save row.
pub struct SaveRowView {
    pub word: (String, String, egui::Color32),
    pub discard: bool,
    /// Why other acts wait for this draft: its long and short form (the long one when it fits).
    pub reason: Option<(String, String)>,
    /// The campaign-switch reason after Discard.
    pub lock: Option<String>,
    pub save_blocked: Option<String>,
    pub primary: bool,
}

/// A notice in the reader's body, with its one repair.
pub struct BodyNotice {
    pub tone: Tone,
    pub words: String,
    pub repair: Option<(String, Option<String>, Option<String>)>,
    /// The system's own words, under `Technical details`.
    pub details: Option<String>,
}

/// What the disk change shows: its message and its choices.
pub struct DiskView {
    pub tone: Tone,
    pub words: String,
    /// The safe choice's words; None when the file cannot be read (Try again is the safe one).
    pub keep: String,
    pub lose: String,
    /// The file cannot be read now: its system words.
    pub details: Option<String>,
    /// The file's text now, beside the draft; None when it has gone (the draft alone shows).
    pub disk_text: Option<String>,
    pub unread: bool,
}

/// The reader pane this frame.
pub struct ReaderView {
    pub note: usize,
    pub key: (OwnerKey, String),
    pub title: String,
    pub subtitle: String,
    pub toolbar: Toolbar,
    pub file_line: FileLine,
    pub save_row: Option<SaveRowView>,
    pub editable: bool,
    pub body: String,
    pub notices: Vec<BodyNotice>,
    pub disk: Option<DiskView>,
    pub draft: String,
    pub draft_revision: u64,
    pub link_choices: Vec<(usize, String)>,
}

impl Library {
    /// Whether the Library is loading.
    pub fn loading(&self) -> bool {
        self.pending.is_some()
    }

    /// Whether the catalogue is loaded.
    pub fn loaded(&self) -> bool {
        self.loaded.is_some()
    }

    /// A failed load with nothing loaded: its fault's tone and words.
    pub fn load_fault(&self) -> Option<(Tone, String)> {
        if self.loaded.is_some() {
            return None;
        }
        self.error
            .as_ref()
            .map(|error| fault_line(&LibraryFault::from(error)))
    }

    /// Whether the open note's file changed on disk (the comparison owns the pane).
    pub fn disk_changed(&self) -> bool {
        self.disk_change.is_some()
    }

    /// Whether Edit Markdown is open.
    pub fn source_open(&self) -> bool {
        self.source
    }

    /// Which form shows in the reader's place: the package form, the New entry form or none.
    pub fn forms(&self) -> (bool, bool) {
        (self.form, self.new_form)
    }

    /// The open note.
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// The open entry's draft.
    pub fn draft_text(&self) -> &str {
        self.draft.as_str()
    }

    pub fn search_text(&self) -> &str {
        &self.search
    }

    pub fn scope(&self) -> Scope {
        self.scope
    }

    pub fn section(&self) -> Section {
        self.section
    }

    /// The scope's notes, counted.
    pub fn total(&self) -> usize {
        self.loaded.as_ref().map_or(0, |loaded| {
            loaded
                .notes
                .iter()
                .filter(|n| loaded.in_scope(n, self.scope))
                .count()
        })
    }

    /// The campaign scope with no entries of its own: the page shows its empty state.
    pub fn empty_scope(&self) -> bool {
        self.loaded.is_some() && self.scope == Scope::Campaign && self.total() == 0
    }

    /// The open draft's lock for the page's own acts (`draft_reason`), or the load's.
    pub fn page_locked(&self) -> Option<&'static str> {
        self.draft_reason().or(self
            .pending
            .is_some()
            .then_some("Wait for the Library to load."))
    }

    /// The header line (`show`'s `TitleLine`) and the no-campaign line under it.
    pub fn header(
        &self,
        campaigns: &Campaigns,
        index: Option<&Arc<crate::lookup::ContentIndex>>,
        wide: bool,
    ) -> Header {
        let total = self.total();
        let locked = self.page_locked();
        let no_place = Places::of(campaigns).create();
        let stopped_reading = self.stopped_reading();
        let broken = self
            .loaded
            .as_ref()
            .zip(index)
            .map(|(loaded, index)| loaded.broken_link_notes(index))
            .unwrap_or_default();
        let open_lock = match self.draft_reason() {
            Some(_) if self.disk_change.is_none() => Some("Save or discard this entry first."),
            reason => reason,
        };
        let issues = self
            .loaded
            .as_ref()
            .filter(|loaded| {
                !loaded.diagnostics.is_empty() || !broken.is_empty() || stopped_reading.is_some()
            })
            .map(|loaded| Issues {
                word: design::count(
                    loaded.files_to_fix(&broken, stopped_reading.as_ref().map(|(file, _)| file)),
                    "file needs fixing",
                    "files need fixing",
                ),
                unreadable: loaded.unreadable_counted || stopped_reading.is_some(),
                lines: loaded
                    .diagnostics
                    .iter()
                    .map(|message| message.text.clone())
                    .chain(stopped_reading.as_ref().map(|(_, text)| text.clone()))
                    .collect(),
                links: broken
                    .iter()
                    .map(|&(at, count)| {
                        (
                            at,
                            format!(
                                "{}: {}",
                                loaded.file_label(&loaded.notes[at]),
                                design::count(count, "broken link", "broken links")
                            ),
                        )
                    })
                    .collect(),
                open_lock: (!broken.is_empty())
                    .then_some(open_lock)
                    .flatten()
                    .map(str::to_owned),
            });
        let no_campaign = campaigns.active_campaign().is_none().then(|| {
            (
                format!(
                    "{} {}",
                    if campaigns.all().is_empty() {
                        "No campaign yet."
                    } else {
                        "No campaign open."
                    },
                    no_place
                        .as_ref()
                        .map_or("The Library works without one.", |reason| reason.as_str())
                ),
                if campaigns.all().is_empty() {
                    "Add campaign"
                } else {
                    "Choose a campaign"
                },
            )
        });
        Header {
            context: (total > 0 && wide).then(|| design::count(total, "entry", "entries")),
            new_entry: (!(self.scope == Scope::Campaign && total == 0)).then(|| {
                locked
                    .map(str::to_owned)
                    .or(no_place.as_ref().map(|reason| reason.as_str().to_owned()))
            }),
            packages_blocked: locked.map(str::to_owned),
            issues,
            no_campaign,
        }
    }

    /// An open draft's file that stopped reading: its file and its short cause.
    fn stopped_reading(&self) -> Option<(FileId, String)> {
        self.disk_change
            .as_ref()
            .and_then(|change| change.fault.as_ref())
            .zip(
                self.selected
                    .and_then(|at| self.loaded.as_ref()?.notes.get(at)),
            )
            .map(|(fault, note)| {
                (
                    Loaded::file_key(note),
                    format!(
                        "{}: {}",
                        self.loaded.as_ref().unwrap().file_label(note),
                        fault.cause()
                    ),
                )
            })
    }

    // -----------------------------------------------------------------------------------------
    // The header's acts.

    /// `New entry` in the header.
    pub fn open_new_entry(&mut self) {
        self.new_form = true;
        self.new_folder = kind_folder(&self.new_kind).into();
        self.focus_new_name = true;
        self.form = false;
    }

    /// `Reload from disk`.
    pub fn reload(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        self.start_load(ctx, campaigns);
    }

    /// `Manage packages…`: the package form opens, or closes.
    pub fn toggle_package_form(&mut self) {
        self.new_form = false;
        self.form = !self.form;
        self.focus_package_name = self.form;
    }

    /// A file of the issues menu.
    pub fn open_broken(&mut self, at: usize) {
        self.open_entry(at);
        self.reveal_row = true;
    }

    /// `Add campaign` or `Choose a campaign` under the header.
    pub fn ask_for_settings(&mut self) {
        self.wants_settings = true;
    }

    // -----------------------------------------------------------------------------------------
    // The list.

    /// The list's work for this frame, after `begin` (as `show` does it between the header and
    /// the columns): the scope's counts, the scope notice's end, the matches, and the selection
    /// that follows the list. None while nothing is loaded.
    pub fn list_view(
        &mut self,
        campaigns: &Campaigns,
        index: Option<&crate::lookup::ContentIndex>,
        compact: bool,
        width: f32,
    ) -> Option<ListView> {
        let loaded = self.loaded.as_ref()?;
        let dirty = self.dirty();
        let scope = self.scope;
        if self
            .scope_counts
            .as_ref()
            .is_none_or(|(known, _)| *known != scope)
        {
            self.scope_counts = Some((scope, section_counts(loaded, scope)));
        }
        let counts = self
            .scope_counts
            .as_ref()
            .map(|(_, counts)| counts.clone())
            .unwrap_or_default();
        let count_of = |section: Section| {
            counts
                .iter()
                .find(|(s, _)| *s == section)
                .map_or(0, |(_, n)| *n)
        };
        let searching = !self.search.trim().is_empty();
        if self
            .kept
            .scope_notice()
            .is_some_and(|(_, widened)| *widened != scope || searching)
        {
            let _ = self.kept.take_scope_notice();
        }
        let about_open = self.selected.is_some_and(|at| {
            loaded.notes[at].is_manifest()
                && matches!(scope, Scope::Package(id) if loaded.notes[at].owner.is_package(id))
        });
        let about = match scope {
            Scope::Package(package) => loaded
                .notes
                .iter()
                .enumerate()
                .find(|(_, n)| n.owner.is_package(package) && n.is_manifest())
                .map(|(index, note)| {
                    (
                        index,
                        note.about_label(loaded.has_problem(note)),
                        about_open,
                    )
                }),
            _ => None,
        };
        // The matches, as `show` keeps them.
        let query = self.search.trim().to_lowercase();
        let filtering =
            (self.section == Section::Creatures || searching) && !self.filter.is_empty();
        if self.matches_scope != Some(scope)
            || self
                .matches
                .as_ref()
                .is_none_or(|(section, cached_query, _)| {
                    *section != self.section || *cached_query != query
                })
            || self.matched_filter != self.filter
        {
            self.matched_filter = self.filter.clone();
            self.reveal_row |= self.matches_scope != Some(scope);
            self.matches_scope = Some(scope);
            let (found, typo_from) = find_matches(
                loaded,
                (scope, self.section, self.unreadable_kind),
                &query,
                &self.filter,
            );
            self.typo_from = typo_from;
            self.matches = Some((self.section, query.clone(), found));
        }
        let mut groups = Vec::new();
        for &(group, sections) in Section::GROUPS {
            if sections.iter().all(|section| count_of(*section) == 0) {
                continue;
            }
            let mut rows = Vec::new();
            for &section in sections {
                let count = count_of(section);
                if count == 0 {
                    continue;
                }
                if section == Section::Unreadable {
                    for kind in Section::GROUPS
                        .iter()
                        .flat_map(|(_, sections)| sections.iter())
                        .copied()
                    {
                        let count = loaded
                            .notes
                            .iter()
                            .filter(|note| {
                                note.error.is_some()
                                    && loaded.in_scope(note, scope)
                                    && note.unreadable_kind() == kind
                            })
                            .count();
                        if count == 0 {
                            continue;
                        }
                        rows.push(SectionRow {
                            label: if kind == Section::Unreadable {
                                "Entries"
                            } else {
                                kind.label()
                            },
                            count,
                            painted: true,
                            selected: !about_open
                                && self.section == section
                                && self.unreadable_kind == Some(kind)
                                && !searching,
                            section,
                            kind: Some(kind),
                        });
                    }
                    continue;
                }
                let (count, painted) = if section == Section::All {
                    let found = self
                        .matches
                        .as_ref()
                        .filter(|(_, kept, _)| {
                            searching && *kept == query && self.matches_scope == Some(scope)
                        })
                        .map(|(.., found)| found.len());
                    (found.unwrap_or(count), found.is_some())
                } else {
                    (count, true)
                };
                rows.push(SectionRow {
                    label: section.label(),
                    count,
                    painted,
                    selected: self.section == section && !searching && !about_open,
                    section,
                    kind: None,
                });
            }
            groups.push(SectionGroup {
                title: group,
                count: sections.iter().map(|section| count_of(*section)).sum(),
                rows,
            });
        }
        let section_choices = Section::GROUPS
            .iter()
            .flat_map(|(_, sections)| sections.iter().copied())
            .filter(|section| count_of(*section) > 0)
            .map(|section| (section, section.label()))
            .collect();
        let matches = &self.matches.as_ref().unwrap().2;
        // Keep the reader in sync with the visible list without dropping an unsaved draft.
        let whole_catalogue = self.section == Section::All && query.is_empty();
        let mut select = None;
        if !dirty
            && self
                .selected
                .is_none_or(|i| !matches.contains(&i) && !loaded.notes[i].is_manifest())
        {
            // A name found only with a typo is a guess. It does not open by itself.
            select = matches
                .first()
                .copied()
                .filter(|_| !whole_catalogue && self.typo_from > 0);
            if select.is_none() {
                self.selected = None;
                self.draft.clear();
            }
        }
        let list_hidden = self.disk_change.is_some() || (width < 480.0 && (dirty || self.source));
        let headed = searching || filtering || (compact && self.section != Section::All);
        let heading = headed.then(|| {
            (
                if searching {
                    "Search results"
                } else if filtering {
                    "Matches"
                } else {
                    self.section.label()
                },
                matches.len(),
            )
        });
        let empty = matches.is_empty().then(|| match scope {
            Scope::Package(id)
                if !searching
                    && !loaded.notes.iter().any(|note| {
                        note.owner.is_package(id)
                            && note.entry.as_ref().is_some_and(|entry| entry.kind != "package")
                    }) =>
            {
                EmptyList::Package {
                    name: campaigns
                        .modules()
                        .iter()
                        .find(|m| m.id == id)
                        .map_or("this package", |m| m.name.as_str())
                        .to_owned(),
                    form: self.new_form,
                }
            }
            _ => {
                let filtered = !self.filter.is_empty()
                    && (searching || self.section == Section::Creatures);
                let (words, wider) = list_empty_words(campaigns, scope, filtered, searching);
                EmptyList::Words { words, wider }
            }
        });
        let rows = matches.len();
        let close_names = searching && !matches.is_empty() && self.typo_from == 0;
        let creatures_or_search = self.section == Section::Creatures || searching;
        if let Some(i) = select {
            self.select(i);
        }
        let selected = self.selected;
        let selected_row = selected.and_then(|selected| {
            let matches = &self.matches.as_ref()?.2;
            if let Some(row) = self
                .selected_row
                .filter(|&row| matches.get(row) == Some(&selected))
            {
                return Some(row);
            }
            let row = matches.iter().position(|i| *i == selected);
            self.selected_row = row;
            row
        });
        let reveal = std::mem::take(&mut self.reveal_row) && selected_row.is_some();
        self.list_hidden = list_hidden;
        let _ = index;
        Some(ListView {
            about,
            groups,
            section_choices,
            chosen_section: (!about_open && !searching).then_some(self.section),
            heading,
            close_names,
            empty,
            rows,
            selected_row,
            reveal,
            creatures_or_search,
            list_hidden,
        })
    }

    /// Row `row` of the list's matches.
    pub fn entry_row(
        &self,
        row: usize,
        index: Option<&crate::lookup::ContentIndex>,
    ) -> Option<EntryRow> {
        let loaded = self.loaded.as_ref()?;
        let i = *self.matches.as_ref()?.2.get(row)?;
        let note = &loaded.notes[i];
        let subtitle = row_subtitle(loaded, note, &loaded.subtitles()[i], self.scope, index);
        Some(EntryRow {
            note: i,
            name: note.name().to_owned(),
            hover: format!("{}\n{}\n{}", note.name(), subtitle, note.path),
            subtitle,
            selected: self.selected == Some(i),
        })
    }

    /// A list press, a link or a reload's choice opens note `i` (`show`'s `select`).
    pub(super) fn select(&mut self, i: usize) {
        // Round 26 SF1, SB1: a list click leaves the open entry as a link does, so Back
        // returns to it and Forward ends.
        if self.selected != Some(i) {
            self.trail.leave(self.here());
        }
        self.selected = Some(i);
        let _ = self
            .draft
            .replace(self.loaded.as_ref().unwrap().notes[i].text.as_str());
        self.error = None;
        self.source = false;
    }

    /// A press on an entry row: it opens, or, while a draft is open, it opens beside the draft
    /// in Lookup.
    pub fn press_entry(&mut self, i: usize) {
        if self.dirty() {
            self.peek = self
                .loaded
                .as_ref()
                .and_then(|loaded| loaded.notes.get(i)?.entry.as_ref())
                .map(|entry| entry.id);
        } else {
            self.select(i);
        }
    }

    /// The scope drop-down's choices: each scope and its words, the chosen one, and where the
    /// second group starts.
    pub fn scope_choices(&self, campaigns: &Campaigns) -> (Vec<(Scope, String)>, usize, usize) {
        let campaign = campaigns.active_id().is_some();
        let mut choices = Vec::new();
        if campaign {
            choices.push((Scope::Uses, "Can use".to_owned()));
        }
        choices.push((Scope::All, "Everything".to_owned()));
        let group = choices.len();
        choices.push((Scope::Builtin, BUILTIN_NAME.to_owned()));
        for module in campaigns.modules() {
            choices.push((Scope::Package(module.id), module.name.clone()));
        }
        if campaign {
            choices.push((Scope::Campaign, "Campaign entries".to_owned()));
        }
        let scope = if !campaign && matches!(self.scope, Scope::Uses | Scope::Campaign) {
            Scope::All
        } else {
            self.scope
        };
        let chosen = choices
            .iter()
            .position(|(choice, _)| *choice == scope)
            .unwrap_or(0);
        (choices, chosen, group)
    }

    /// A scope from the drop-down. Choosing here never changes the active campaign; the
    /// selected entry stays if it is in the scope, and a draft always stays.
    pub fn choose_scope(&mut self, scope: Scope) {
        if scope != self.scope {
            self.section = Section::All;
        }
        self.scope = scope;
    }

    /// A section row (or the compact drop-down): with an unreadable `kind` under Cannot be read.
    pub fn choose_section(&mut self, section: Section, kind: Option<Section>) {
        if kind.is_some() {
            self.section = section;
            self.unreadable_kind = kind;
            self.matches = None;
            self.search.clear();
            return;
        }
        let about_open = self.about_open();
        if about_open && !self.dirty() {
            self.selected = None;
            self.draft.clear();
        }
        self.section = section;
        self.unreadable_kind = None;
        self.search.clear();
    }

    /// Whether the package's About is open in its own scope.
    fn about_open(&self) -> bool {
        let Some(loaded) = &self.loaded else {
            return false;
        };
        self.selected.is_some_and(|at| {
            loaded.notes[at].is_manifest()
                && matches!(self.scope, Scope::Package(id) if loaded.notes[at].owner.is_package(id))
        })
    }

    /// The About row: the package's manifest opens.
    pub fn open_about(&mut self, index: usize) {
        if self.dirty() {
            return;
        }
        let Some(loaded) = &self.loaded else {
            return;
        };
        // Round 26 SB1: a list click records the trail, as a link does.
        if self.selected != Some(index) {
            self.trail.leave(place(
                loaded,
                self.selected,
                self.scope,
                self.section,
                &self.search,
            ));
        }
        let text = loaded.notes[index].text.as_str().to_owned();
        self.selected = Some(index);
        let _ = self.draft.replace(text);
        self.section = Section::All;
        self.unreadable_kind = None;
        self.source = false;
    }

    /// The search field's words. Clear empties them; nothing else resets.
    pub fn set_search(&mut self, words: &str) {
        self.search = words.to_owned();
    }

    /// `Search Everything`: the same words and filters, in the widest scope.
    pub fn search_everything(&mut self) {
        self.scope = Scope::All;
        self.section = Section::All;
    }

    /// The empty campaign scope's link: the entries the campaign can use.
    pub fn show_usable(&mut self) {
        self.scope = Scope::Uses;
    }

    /// The empty campaign scope's `New entry`.
    pub fn start_entry(&mut self) {
        self.new_form = true;
        self.new_folder = kind_folder(&self.new_kind).into();
    }

    /// The empty package's `New entry`: a creature, in its folder.
    pub fn start_package_entry(&mut self) {
        self.new_form = true;
        self.focus_new_name = true;
        self.new_kind = "creature".into();
        self.new_folder = "Creatures".into();
    }

    /// Whether a field asked for the keyboard (a form that opened), once.
    pub fn take_focus_new_name(&mut self) -> bool {
        std::mem::take(&mut self.focus_new_name)
    }

    pub fn take_focus_package_name(&mut self) -> bool {
        std::mem::take(&mut self.focus_package_name)
    }

    /// Whether the editor asked for the keyboard (a repair, a new creature, Ctrl+J's way back).
    pub fn take_focus_editor(&mut self) -> bool {
        std::mem::take(&mut self.focus_markdown)
            | std::mem::take(&mut self.focus_repair)
            | std::mem::take(&mut self.focus_hp)
    }

    // -----------------------------------------------------------------------------------------
    // Footers.

    /// The footer of a pane with no reader (a form, the empty campaign scope): `form_messages`.
    pub fn form_footer(&self, campaigns: &Campaigns) -> Vec<FooterLine<LibraryPress>> {
        let frame = PaneFrame::headless();
        self.form_messages(&frame, design::READER_MEASURE, campaigns)
            .into_lines()
    }

    /// The empty reader's footer: Remove's notice and the page's results and error.
    pub fn empty_reader_footer(&self, campaigns: &Campaigns) -> Vec<FooterLine<LibraryPress>> {
        let frame = PaneFrame::headless();
        let results = self.reader_results(campaigns);
        let mut messages = Messages::new(&frame, "library_footer", design::READER_MEASURE);
        if !self.dirty()
            && let Some(line) = self.kept.outcome(&frame, KeptKind::Removed, || {
                self.outcome_of(KeptKind::Removed, &results)
            })
        {
            messages.kept(line);
        }
        self.page_messages(&frame, &mut messages, &results, campaigns);
        page_faults(&mut messages, &[self.error.as_ref()]);
        messages.into_lines()
    }

    /// The reader's kept results and its lock (`editor_pane`'s `ReaderResults`).
    fn reader_results(&self, campaigns: &Campaigns) -> ReaderResults {
        ReaderResults {
            package_use: self.kept.package_use().is_some_and(|undo| {
                campaigns
                    .active_campaign()
                    .is_some_and(|c| c.id == undo.campaign.id)
            }),
            manifest: self.kept.manifest().is_some(),
            locked: if self.disk_change.is_some() {
                Some(&CHANGED_FILE_FIRST)
            } else if self.dirty() {
                Some(&UNDO_WAITS_FOR_DRAFT)
            } else {
                None
            },
            lock_can_come: self.source || self.dirty() || self.disk_change.is_some(),
            identities: self.kept.identities().is_some(),
            identity: self.kept.identity().is_some(),
        }
    }

    /// A press in any Library footer, as `show` and `editor_pane` act on it: the empty reader's
    /// Undo remove, the editor's own presses, and the page's.
    pub fn press_footer(
        &mut self,
        press: Press<LibraryPress>,
        ctx: &egui::Context,
        campaigns: &mut Campaigns,
    ) {
        let kind = press
            .ticket
            .as_ref()
            .and_then(|ticket| self.kept.kind_by(ticket));
        match (&press.id, kind) {
            (LibraryPress::Undo, Some(KeptKind::Removed)) => self.undo_remove(),
            (LibraryPress::Undo, Some(KeptKind::Discarded)) => self.undo_discard(),
            (LibraryPress::Undo, Some(KeptKind::Heading)) => self.heading_press(false),
            (LibraryPress::Undo, Some(KeptKind::Vital)) => self.undo_vital(),
            (LibraryPress::Undo, Some(KeptKind::Identities)) => {
                let result = match self.kept.identities() {
                    Some((undo, _)) => undo.undo().map(|()| {
                        let _ = self.kept.take_identities();
                        self.stale = true;
                        self.content_changed =
                            campaigns.active_campaign().map(|campaign| campaign.path);
                    }),
                    None => Ok(()),
                };
                if let Err(error) = result {
                    self.error = Some(error.into());
                }
            }
            (LibraryPress::Undo, Some(KeptKind::Identity)) => {
                if let Err(error) = self.undo_new_identity() {
                    self.error = Some(error.into());
                }
            }
            (LibraryPress::Undo, Some(KeptKind::Copy)) => {
                let at = press
                    .ticket
                    .as_ref()
                    .and_then(|ticket| self.kept.copy_at(ticket));
                if let Some(at) = at
                    && let Err(error) = self.undo_dependency(at)
                {
                    self.error = Some(error.into());
                }
            }
            (LibraryPress::FixHeading, _) => self.heading_press(true),
            (LibraryPress::UseVital(key, label, value), _) => {
                let (key, label, value) = (*key, *label, *value);
                self.use_vital(key, label, value);
            }
            (LibraryPress::Repair, _) => self.repair(),
            (LibraryPress::SeparateEntry(at), _) => {
                let at = *at;
                self.open_entry(at);
                if let Err(error) = self.give_new_identity() {
                    self.error = Some(error.into());
                }
            }
            (LibraryPress::SeparateEntries(at), _) => {
                let at = *at;
                let paths = self.loaded.as_ref().and_then(|loaded| {
                    let owner = &loaded.notes.get(at)?.owner;
                    loaded
                        .duplicate_folder_paths(owner)
                        .zip(owner.root().map(Path::to_path_buf))
                });
                if let Some((paths, root)) = paths
                    && let Err(error) = self.separate_entries(root, &paths)
                {
                    self.error = Some(error.into());
                }
            }
            (LibraryPress::CopyDependency(id), _) => {
                let id = *id;
                if let Err(error) = self.copy_dependency_of(id, campaigns) {
                    self.error = Some(error.into());
                }
            }
            _ => self.apply_footer_press(press, campaigns),
        }
        let _ = ctx;
    }

    /// Undo remove: the entry comes back with its draft, as the empty reader's footer gives it.
    fn undo_remove(&mut self) {
        if let Some((_, _, draft)) = self.kept.take_discarded()
            && let Some(LetGo {
                note, used, chosen, ..
            }) = self.kept.take_removed()
            && let Some(loaded) = self.loaded.as_mut()
        {
            if used && let (Some(uses), Some(entry)) = (loaded.uses.as_mut(), note.entry.as_ref()) {
                uses.insert(entry.id);
            }
            if chosen
                && let (Some(set), Some(entry)) = (loaded.chosen.as_mut(), note.entry.as_ref())
            {
                set.insert(entry.id);
            }
            self.selected = Some(loaded.notes.len());
            loaded.notes.push(note);
            loaded.invalidate();
            loaded.refresh_diagnostics();
            let _ = self.draft.replace(draft);
            self.draft_revision += 1;
            self.source = true;
            self.disk_change = Some(DiskChange {
                text: None,
                moved_to: None,
                fault: None,
            });
            self.invalidate();
        }
    }

    /// Undo discard: the draft comes back while the editor still holds the saved text.
    fn undo_discard(&mut self) {
        let Some(note) = self.open_note() else {
            return;
        };
        if self.saved_is_draft(note)
            && let Some((_, _, text)) = self.kept.take_discarded()
        {
            let _ = self.draft.replace(text);
            self.draft_revision += 1;
        }
    }

    /// The heading fix (`Use <name>`) or its Undo, on the draft as it is now.
    fn heading_press(&mut self, fix: bool) {
        let Some(i) = self.selected else {
            return;
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let editable = note.owner.editable();
        let conflict = self.disk_change.is_some();
        let file: FileId = (note.owner.key(), note.path.clone());
        let current = fix
            .then(|| Entry::parse(&note.path, &self.draft).ok())
            .flatten()
            .filter(|_| editable && !conflict);
        let heading_now = HeadingChoice::new(
            current.as_ref(),
            &self.draft,
            self.kept.heading(),
            &file,
            &self.same_heading,
        );
        let pressed = (
            fix && heading_now.fix.is_some(),
            !fix && self.heading_undo_applies(&file),
        );
        if pressed.0 || pressed.1 {
            self.draft_revision += 1;
        }
        let save = heading_now.apply(pressed, &file, &mut self.draft, &mut self.kept);
        if save {
            self.save_selected(false);
        }
    }

    /// `Use <value>`: the stat fix, computed again from the current text.
    fn use_vital(&mut self, key: &'static str, label: &'static str, value: i64) {
        let Some(note) = self.open_note() else {
            return;
        };
        let file: FileId = (note.owner.key(), note.path.clone());
        let current = Entry::parse(&note.path, &self.draft)
            .ok()
            .filter(|_| note.owner.editable())
            .filter(|entry| {
                creature::vital_checks(entry)
                    .iter()
                    .any(|(at, _, now, _)| *at == key && *now == value)
            });
        if let Some(entry) = current {
            Self::fix_stat(
                (
                    &mut self.kept,
                    &mut self.draft,
                    &mut self.draft_revision,
                    &mut self.error,
                ),
                file,
                &entry,
                (key, label, value),
            );
        }
    }

    /// Undo stat change.
    fn undo_vital(&mut self) {
        let Some(note) = self.open_note() else {
            return;
        };
        let file: FileId = (note.owner.key(), note.path.clone());
        if self.stat_undo_applies(&file)
            && let Some((_, before, _, _)) = self.kept.take_vital()
        {
            let _ = self.draft.replace(before);
            self.draft_revision += 1;
        }
    }

    /// The repair of the open file: a manifest's recovery, or Edit Markdown at the fault.
    pub fn repair(&mut self) {
        match self.recovery() {
            Some((module, path, exists)) => self.write_manifest(module, &path, exists),
            None => {
                self.source = true;
                self.focus_repair = true;
            }
        }
    }

    /// The open package manifest's recovery, when it is broken: its package, path and whether
    /// a file is there.
    fn recovery(&self) -> Option<(Module, String, bool)> {
        self.selected
            .and_then(|at| self.loaded.as_ref()?.notes.get(at))
            .filter(|note| note.is_manifest() && note.error.is_some())
            .and_then(|note| match &note.owner {
                Owner::Package(module) => {
                    let path = note.path.clone();
                    let exists = note.error.as_deref()
                        != Some(crate::content_workspace::DELETED_MANIFEST)
                        && module.path.join(&path).exists();
                    Some((module.clone(), path, exists))
                }
                _ => None,
            })
    }

    /// The copy's Undo (`DependencyUndo`): the link goes back and the copy goes.
    fn undo_dependency(&mut self, at: usize) -> Result<(), String> {
        let Some(DependencyUndo {
            root,
            path,
            before,
            after,
            copied_path,
            copied_text,
            ..
        }) = self.kept.copies().get(at)
        else {
            return Ok(());
        };
        let copied = note_path(root, copied_path)?;
        if fs::read_to_string(&copied).map_err(|e| FileFault::read(copied_path, &e).message())?
            != *copied_text
        {
            return Err("The copy changed on disk. Keep it, and change the link in Markdown.".into());
        }
        write_note(root, path, Some(after), before)?;
        if let Err(error) = fs::remove_file(copied) {
            let _ = write_note(root, path, Some(before), after);
            return Err(error.to_string());
        }
        self.content_changed = Some(root.clone());
        self.kept.remove_copy(at);
        self.stale = true;
        Ok(())
    }

    /// `Copy <entry> to <package>`: the campaign entry `id` that the open package entry links to.
    fn copy_dependency_of(&mut self, id: Uuid, campaigns: &Campaigns) -> Result<(), String> {
        let Some(i) = self.selected else {
            return Ok(());
        };
        let loaded = self.loaded.as_ref().unwrap();
        let note = &loaded.notes[i];
        if let Some(source) = loaded
            .notes
            .iter()
            .find(|n| n.entry.as_ref().is_some_and(|e| e.id == id))
            && let Some(root) = Self::copy_dependency(&mut self.kept, note, source, campaigns)?
        {
            self.content_changed = Some(root);
            self.stale = true;
        }
        Ok(())
    }

    fn open_note(&self) -> Option<&Note> {
        self.loaded.as_ref()?.notes.get(self.selected?)
    }

    // -----------------------------------------------------------------------------------------
    // The reader.

    /// The reader pane's data this frame (`editor_pane` before it draws), or None with no
    /// entry open. `index` is the link index (`reader_link_index`).
    pub fn reader_view(
        &mut self,
        campaigns: &Campaigns,
        index: Option<&Arc<crate::lookup::ContentIndex>>,
        subtitle: &str,
    ) -> Option<(ReaderView, Vec<FooterLine<LibraryPress>>)> {
        let i = self.selected?;
        let recovery = self.recovery();
        let draft_lock = self.draft_lock();
        let parsed_draft = self.parsed_draft(i);
        let parsed: &Result<Entry, String> = &parsed_draft;
        let results = self.reader_results(campaigns);
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let reading = parsed
            .as_ref()
            .ok()
            .and_then(|entry| entry.reading().ok().flatten());
        let file_name = Path::new(&note.path)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned());
        let title = match &reading {
            Some(reading) => reading.title.clone(),
            None if note.entry.is_none()
                && note.error.as_deref() != Some(crate::content_workspace::DELETED_MANIFEST) =>
            {
                file_name.clone().unwrap_or_else(|| note.name().to_owned())
            }
            None => note.name().to_owned(),
        };
        let reading_subtitle = reader_subtitle(
            &note.owner,
            &row_line_without_fault(subtitle),
            reading
                .as_ref()
                .and_then(|_| presentation::class_level(parsed.as_ref().ok()?)),
            parsed
                .as_ref()
                .ok()
                .and_then(|entry| entry.fields.get("splat_spell"))
                .and_then(spell_kind),
        );
        let dirty = !self.saved_is_draft(note);
        let editable = note.owner.editable();
        let fault = note.fault.clone();
        if fault.is_some() {
            self.source = false;
        }
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let conflict = self.disk_change.is_some();
        let mut duplicate_indices = self.loaded.as_ref().unwrap().duplicates(i);
        if !duplicate_indices.is_empty() {
            duplicate_indices.push(i);
            let notes = &self.loaded.as_ref().unwrap().notes;
            let first = crate::content_workspace::duplicate_repair_path(
                duplicate_indices.iter().filter_map(|&at| {
                    notes[at]
                        .owner
                        .root()
                        .map(|root| root.join(&notes[at].path))
                }),
            );
            duplicate_indices.sort_by_key(|&at| {
                notes[at]
                    .owner
                    .root()
                    .map(|root| root.join(&notes[at].path))
                    != first
            });
        }
        self.just_saved &= !dirty && self.source;
        let d = colors().document;
        let stopped_reading = self
            .disk_change
            .as_ref()
            .is_some_and(|change| change.fault.is_some());
        let about = note.entry.as_ref().is_some_and(|e| e.kind == "package");
        let mut every_chosen = false;
        let owner_fault = match &note.owner {
            Owner::Package(_) if stopped_reading => Some(FaultStatus::CannotRead),
            Owner::Package(_) if about => None,
            Owner::Package(_) if note.manifest_gone => Some(FaultStatus::ManifestDeleted),
            Owner::Package(_) | Owner::Campaign(_)
                if note.error.is_some() || note.fault.is_some() || stopped_reading =>
            {
                Some(FaultStatus::CannotRead)
            }
            _ => None,
        };
        let owner_name = match &note.owner {
            Owner::Package(module) => module.name.as_str(),
            _ => "Campaign",
        };
        let package_word = match &note.owner {
            Owner::Package(module) if let Some(fault) = owner_fault => fault.words(&module.name),
            Owner::Package(_) if about && campaigns.active_campaign().is_none() => {
                "No campaign open".into()
            }
            Owner::Package(module) if about => {
                let loaded = self.loaded.as_ref().unwrap();
                let entries: Vec<_> = loaded
                    .notes
                    .iter()
                    .filter(|n| {
                        n.owner.is_package(module.id)
                            && n.entry.as_ref().is_some_and(|e| {
                                !["package", "reference"].contains(&e.kind.as_str())
                            })
                    })
                    .collect();
                let chosen = entries.iter().filter(|n| loaded.chose(n)).count();
                let needed = entries
                    .iter()
                    .filter(|n| !loaded.chose(n) && loaded.in_scope(n, Scope::Uses))
                    .count();
                every_chosen = chosen == entries.len();
                chosen_count(chosen, needed, entries.len())
            }
            Owner::Package(module) => format!(
                "{} · {}",
                module.name,
                if self.loaded.as_ref().unwrap().in_scope(note, Scope::Uses) {
                    "Chosen for this campaign"
                } else {
                    "Not chosen"
                }
            ),
            _ => String::new(),
        };
        let refusal = (note.error.is_some() || dirty)
            .then(|| self.loaded.as_ref().unwrap().draft_refusal(i, parsed))
            .flatten();
        let save_refusal = refusal.clone().filter(|_| dirty);
        let (repair_problem, _repair_line) =
            draft_repair_problem(note, &self.draft, refusal, recovery.as_ref());
        let here = note.owner.root().map(|root| root.join(&note.path));
        let missing_link = self
            .repair_link
            .as_mut()
            .filter(|link| here.as_ref() == Some(&link.path))
            .and_then(|link| {
                let line = (*link.found.get_or(parsed_draft.revision(), || {
                    parsed
                        .as_ref()
                        .ok()
                        .and_then(|entry| repair_row(entry, &link.target))
                }))?;
                Some((line, link.words.clone()))
            });
        if let Some(failure) = &self.save_failure {
            let draft = DraftView::new(
                &self.draft,
                #[cfg(any(test, feature = "test-support"))]
                &self.failure_checks,
            );
            if !failure.owns(note, self.draft_revision, draft) {
                self.save_failure = None;
            }
        }
        let failed = self
            .save_failure
            .as_ref()
            .map(|failure| failure.reason.clone());
        let mut draft_state = design::DraftState::new(
            design::save_row::DraftOrigin::Saved,
            if dirty {
                design::save_row::Changed::Yes
            } else {
                design::save_row::Changed::No
            },
            if self.run_open {
                design::save_row::Primary::Elsewhere
            } else {
                design::save_row::Primary::Draft
            },
        );
        if let Some(refusal) = &save_refusal {
            draft_state = draft_state.check(design::save_row::Diagnosis::refused(refusal.as_str()));
        }
        let manifest_gone = fault.is_none()
            && note.error.is_some()
            && recovery.as_ref().is_some_and(|(_, _, exists)| !exists);
        if (fault.is_some() || repair_problem.is_some()) && !manifest_gone {
            draft_state = draft_state.check(design::save_row::Diagnosis::unreadable());
        }
        if conflict {
            draft_state = draft_state.changed_on_disk();
        }
        if failed.is_some() {
            draft_state = draft_state.not_saved();
        }
        if self.just_saved {
            draft_state = draft_state.just_saved();
        }
        let can_copy = note
            .entry
            .as_ref()
            .is_some_and(|e| !["package", "campaign", "reference"].contains(&e.kind.as_str()));
        let can_make = matches!(note.owner, Owner::Campaign(_))
            && note.entry.as_ref().is_some_and(|e| e.kind == "creature");
        let choose = match &note.owner {
            Owner::Package(_)
                if about && campaigns.active_campaign().is_some() && !every_chosen =>
            {
                Some("Choose all")
            }
            Owner::Package(_)
                if !about
                    && note.entry.is_some()
                    && !self.loaded.as_ref().unwrap().in_scope(note, Scope::Uses) =>
            {
                Some("Choose for this campaign")
            }
            _ => None,
        };
        // The toolbar (`ReaderToolbar::show`).
        let blocks = self.draft_blocks();
        let on_disk = draft_state.file_changed();
        let held = draft_state.discard_shown() || on_disk;
        let (long, short) = if on_disk {
            (
                design::save_row::CHANGED_ON_DISK,
                design::save_row::CHANGED_ON_DISK,
            )
        } else {
            (blocks.long(), blocks.short())
        };
        let draft_reason = held.then_some(long);
        let copy_waits = Places::of(campaigns).copy();
        let mut actions: Vec<(&'static str, ReaderAct, Option<String>)> = Vec::new();
        let mut reasons = Vec::new();
        if can_copy {
            let reason = match &copy_waits {
                Some(reason) => {
                    reasons.push(reason.as_str().to_owned());
                    Some(reason.as_str().to_owned())
                }
                None => draft_reason.map(str::to_owned),
            };
            actions.push(("Copy to…", ReaderAct::Copy, reason));
        }
        if can_make {
            actions.push((
                "Make NPC note",
                ReaderAct::Make,
                draft_reason.map(str::to_owned),
            ));
        }
        if let Some(label) = choose {
            actions.push((label, ReaderAct::Choose, draft_reason.map(str::to_owned)));
        }
        let cites = can_make || choose.is_some() || (can_copy && copy_waits.is_none());
        let save_row = editable && (self.source || dirty) && !conflict;
        let toolbar = Toolbar {
            source: self.source,
            source_label: if editable {
                "Edit Markdown"
            } else {
                "Markdown source"
            },
            mode_blocked: fault.as_ref().map(|_| UNREAD_EDIT.as_str().to_owned()),
            back: self.trail.peek(Step::Back).is_some(),
            actions,
            reasons,
            draft_reason: (!save_row && cites && held).then(|| long.to_owned()),
        };
        let save_row = save_row.then(|| {
            let status = draft_state.status();
            SaveRowView {
                word: (status.icon.to_owned(), status.word.to_owned(), status.color),
                discard: draft_state.discard_shown(),
                reason: (held && cites).then(|| (long.to_owned(), short.to_owned())),
                lock: draft_state.lock_reason().map(str::to_owned),
                save_blocked: draft_state.save_blocked().map(str::to_owned),
                primary: draft_state.save_weight() == design::save_row::SaveWeight::Primary,
            }
        });
        // The owner and file line.
        let read_only = self
            .save_failure
            .as_ref()
            .is_some_and(|failure| failure.read_only.is_some())
            && note.error.is_none()
            && !stopped_reading;
        let status = match &note.owner {
            Owner::Builtin => (icons::READ_ONLY, "Built-in · Read only".to_owned(), d.read_only),
            _ if let Some(fault) = owner_fault => {
                let (icon, color) = tone_parts(fault.severity().tone());
                (icon, fault.words(owner_name), color)
            }
            Owner::Package(module) if read_only => (
                icons::READ_ONLY,
                format!("{} · Read only", module.name),
                d.read_only,
            ),
            Owner::Package(_) => (icons::EDIT, package_word.clone(), d.clean),
            _ if read_only => (icons::READ_ONLY, "Campaign · Read only".to_owned(), d.read_only),
            _ => (icons::EDIT, "Campaign · Editable".to_owned(), d.clean),
        };
        let unread = fault.is_some() || stopped_reading;
        let moved_path = self
            .disk_change
            .as_ref()
            .and_then(|change| change.moved_to.clone());
        let file_gone_now = self.disk_change.as_ref().is_some_and(|change| {
            change.text.is_none() && change.moved_to.is_none() && change.fault.is_none()
        });
        let deleted_manifest = recovery.as_ref().is_some_and(|(_, _, exists)| !exists);
        let file_line = FileLine {
            status,
            file: note.owner.root().map(|root| {
                (
                    root.to_path_buf(),
                    moved_path.clone().unwrap_or_else(|| note.path.clone()),
                )
            }),
            moved_from: moved_path.as_ref().map(|_| {
                format!(
                    "moved from {}/",
                    Path::new(&note.path)
                        .parent()
                        .unwrap_or(Path::new(""))
                        .display()
                )
            }),
            obsidian: note
                .owner
                .root()
                .is_some_and(|root| obsidian_for(root, &note.owner))
                && !unread,
            folder_only: deleted_manifest || file_gone_now,
        };
        // The body's subtitle (`editor_pane`'s reader measure).
        let kind_line = reader_kind_line(&note.owner, &reading_subtitle);
        let kind_line = kind_line.strip_suffix(" · Campaign").unwrap_or(&kind_line);
        let body_subtitle = if note.error.is_some() {
            match note.unreadable_kind() {
                Section::Unreadable | Section::Other => "",
                Section::PackageDetails => "Package",
                kind => kind.label().trim_end_matches('s'),
            }
            .to_owned()
        } else if kind_line == "Campaign" {
            note.section().label().trim_end_matches('s').to_owned()
        } else {
            kind_line.to_owned()
        };
        let mut notices = Vec::new();
        if let Some(fault) = &fault {
            let (tone, words) = fault_line(&LibraryFault::Unread(fault, false));
            notices.push(BodyNotice {
                tone,
                words,
                repair: Some(("Try again".to_owned(), None, None)),
                details: Some(fault.system.clone()),
            });
        }
        if let Some(problem) = &repair_problem {
            let lock = draft_lock.as_ref().map(|lock| lock.as_str().to_owned());
            let (tone, words) = match &recovery {
                Some((_, _, exists)) => fault_line(&LibraryFault::Repair(
                    problem,
                    crate::content_workspace::manifest_fault_tone(*exists),
                )),
                None => fault_line(&LibraryFault::Repair(problem, Tone::Error)),
            };
            let repair = match &recovery {
                Some((_, path, true)) => Some((
                    format!("Replace with a basic {path}"),
                    lock,
                    Some(manifest_replace_help(path).as_str().to_owned()),
                )),
                Some((_, _, false)) => Some(("Write the manifest again".to_owned(), lock, None)),
                None if self.source => None,
                None => Some(("Fix in Edit Markdown".to_owned(), None, None)),
            };
            notices.push(BodyNotice {
                tone,
                words,
                repair,
                details: None,
            });
        }
        // A failed parse must never render YAML properties as book prose. The body loses the
        // vitals a stat block shows and its first heading, which the reader's title is (as
        // `view::show_with_related` reads it).
        let body = parsed.as_ref().map_or(String::new(), |entry| {
            let body = if creature::has_stats(entry) {
                creature::without_vitals(entry, &entry.body)
            } else if ["spell", "item"].contains(&entry.kind.as_str()) {
                splat::content::stat_blocks::reading_body(entry, &entry.body)
            } else {
                entry.body.clone()
            };
            let trimmed = body.trim_start();
            trimmed
                .strip_prefix(&format!("# {}\n", note.name()))
                .unwrap_or(trimmed)
                .to_owned()
        });
        // The disk change's placeholder.
        let disk = self.disk_change.as_ref().map(|change| {
            if let Some(fault) = &change.fault {
                let (tone, words) = fault_line(&LibraryFault::Unread(fault, true));
                return DiskView {
                    tone,
                    words,
                    keep: "Try again".to_owned(),
                    lose: "Reload and lose my draft".to_owned(),
                    details: Some(fault.system.clone()),
                    disk_text: None,
                    unread: true,
                };
            }
            let names = if change.text.is_none() {
                self.loaded.as_ref().unwrap().dependents(note)
            } else {
                DependentNames::default()
            };
            let text = if change.moved_to.is_some() {
                MOVED_ON_DISK
            } else if change.text.is_some() {
                changed_on_disk(note.entry.as_ref().map_or("", |entry| &entry.name))
            } else if names.is_empty() {
                DELETED_ON_DISK
            } else {
                DELETED_ON_DISK.then(crate::sentence!("{} needs this entry.", name_list(&names)))
            };
            let (tone, words) = fault_line(&LibraryFault::Warning(&text));
            DiskView {
                tone,
                words,
                keep: match (&change.moved_to, &change.text) {
                    (Some(path), _) => format!("Save my draft to {path}"),
                    (None, None) => "Save my draft again".to_owned(),
                    (None, Some(_)) => "Overwrite with my draft".to_owned(),
                },
                lose: if change.text.is_none() {
                    "Remove entry and lose draft"
                } else {
                    "Reload and lose my draft"
                }
                .to_owned(),
                details: None,
                disk_text: change.text.clone(),
                unread: false,
            }
        });
        // The footer (`editor_pane`'s typed footer, built the same way).
        let heading = HeadingChoice::new(
            (editable && !conflict)
                .then(|| parsed.as_ref().ok())
                .flatten(),
            &self.draft,
            self.kept.heading(),
            &(note.owner.key(), note.path.clone()),
            &self.same_heading,
        );
        let choices_in_reader = !self.source && !conflict;
        let dependencies: Vec<(Uuid, Sentence, (String, String))> = match (&note.owner, &note.entry)
        {
            (Owner::Package(package), Some(entry)) if choices_in_reader => index
                .map(|index| {
                    index
                        .campaign_dependencies(entry)
                        .into_iter()
                        .map(|at| {
                            let dependency = &index.items[at].entry;
                            (
                                dependency.id,
                                crate::sentence!(
                                    "{} links to {}, which is in this campaign only. Another campaign that uses {} will not find it.",
                                    entry.name,
                                    dependency.name,
                                    package.name
                                ),
                                (dependency.name.clone(), package.name.clone()),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        let edit = editable && (self.source || dirty) && !conflict;
        let vital_result = edit
            .then(|| self.stat_result_for(&(note.owner.key(), note.path.clone())))
            .flatten();
        let vitals = match (edit, parsed) {
            (true, Ok(entry)) => creature::vital_checks(entry),
            _ => Vec::new(),
        };
        let broken: Vec<Sentence> = match (edit, index, parsed) {
            (true, Some(index), Ok(entry)) if note.owner.editable() && !note.is_manifest() => {
                let skip = missing_link.as_ref().map(|(at, _)| *at);
                let fresh = self
                    .broken_lines
                    .as_ref()
                    .is_some_and(|(seen, known, at, _)| {
                        *seen == parsed_draft.revision()
                            && *known == index.generation()
                            && *at == skip
                    });
                if !fresh {
                    let warnings = broken_link_warnings(&self.draft, entry, index, skip);
                    self.broken_lines =
                        Some((parsed_draft.revision(), index.generation(), skip, warnings));
                }
                self.broken_lines
                    .as_ref()
                    .map(|(_, _, _, warnings)| warnings.clone())
                    .unwrap_or_default()
            }
            _ => Vec::new(),
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        if !self.saved_is_draft(note) {
            let _ = self.kept.take_discarded();
        }
        let frame = PaneFrame::headless();
        let mut messages = Messages::new(&frame, "library_footer", design::READER_MEASURE);
        let undo = |step: UndoStep| step.pressed_as(LibraryPress::Undo);
        if discard_is_for(self.kept.discarded(), note)
            && let Some(line) = self.kept.outcome(&frame, KeptKind::Discarded, || {
                Some(KeptOutcome::Notice(
                    Notice::new(crate::sentence!("{} draft discarded.", note.name())),
                    undo(UndoStep::of(Verb::Discard, note.name())),
                ))
            })
        {
            messages.kept(line);
        }
        if let Some(words) = &heading.undo
            && let Some(line) = self.kept.outcome(&frame, KeptKind::Heading, || {
                Some(
                    Said::new(words.clone())
                        .undo(undo(HEADING_CHANGE_UNDO))
                        .into(),
                )
            })
        {
            messages.kept(line);
        }
        if let Some(words) = vital_result
            && let Some(line) = self.kept.outcome(&frame, KeptKind::Vital, || {
                Some(Said::new(words).undo(undo(STAT_CHANGE_UNDO)).into())
            })
        {
            messages.kept(line);
        }
        for copy in self.kept.copies().iter().rev().filter(|copy| {
            choices_in_reader
                && copy.path == note.path
                && note.owner.root() == Some(copy.root.as_path())
        }) {
            if let Some(line) = self.kept.copy_outcome(&frame, copy, || {
                Said::new(crate::sentence!(
                    "{} copied to {} and linked.",
                    copy.name,
                    copy.package
                ))
                .undo(undo(UndoStep::of(Verb::Copy, copy.name.as_str())))
                .into()
            }) {
                messages.kept(line);
            }
        }
        self.page_messages(&frame, &mut messages, &results, campaigns);
        if edit {
            let repair = recovery.as_ref().filter(|_| note.error.is_some());
            let reason_tone = recovery
                .as_ref()
                .filter(|_| note.error.is_some())
                .map_or(Tone::Error, |(_, _, exists)| {
                    crate::content_workspace::manifest_fault_tone(*exists)
                });
            let problem = repair_problem
                .as_deref()
                .or(repair.and(note.error.as_deref()));
            if let Some(problem) = problem {
                let fault = FaultMessage::new(&LibraryFault::Repair(problem, reason_tone));
                messages.outcome(match repair {
                    Some((_, path, exists)) => {
                        fault.choice(recovery_choice(path, *exists, self.draft_lock()))
                    }
                    None => fault,
                });
            }
            if let Some(failure) = &failed {
                messages.outcome(FaultMessage::new(&LibraryFault::NotSaved(failure)));
            }
            if let Some((line, words)) = &missing_link {
                messages.outcome(FaultMessage::new(&LibraryFault::Link {
                    line: *line,
                    words,
                }));
            }
            for words in &broken {
                messages.outcome(Notice::warning(words.clone()));
            }
            for (key, label, value, words) in &vitals {
                messages.outcome(Notice::warning(words.clone()).choice(Choice::named(
                    crate::choice!("Use {}", value),
                    LibraryPress::UseVital(key, label, *value),
                )));
            }
        }
        if let Some((words, name, _)) = &heading.fix {
            messages.outcome(Notice::warning(words.clone()).choice(Choice::named(
                crate::choice!("Use {}", name),
                LibraryPress::FixHeading,
            )));
        }
        if !duplicate_indices.is_empty() {
            messages.outcome(duplicate_notice(
                &egui::Context::default(),
                self.loaded.as_ref().unwrap(),
                note,
                &duplicate_indices,
                self.draft_lock(),
            ));
        }
        for (id, words, (name, package)) in &dependencies {
            let choice = Choice::named(
                crate::choice!("Copy {} to {}", name, package),
                LibraryPress::CopyDependency(*id),
            );
            messages.outcome(
                Notice::warning(words.clone()).choice(match self.draft_lock() {
                    Some(reason) => choice.blocked(reason),
                    None => choice,
                }),
            );
        }
        page_faults(&mut messages, &[self.error.as_ref()]);
        let footer = messages.into_lines();
        let link_choices = if self.link_choices.len() > 1 {
            let loaded = self.loaded.as_ref().unwrap();
            self.link_choices
                .iter()
                .filter_map(|id| {
                    loaded
                        .notes
                        .iter()
                        .enumerate()
                        .find(|(_, n)| n.entry.as_ref().is_some_and(|e| e.id == *id))
                        .map(|(at, note)| (at, format!("{} · {}", note.name(), note.owner.label())))
                })
                .collect()
        } else {
            Vec::new()
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        Some((
            ReaderView {
                note: i,
                key: (note.owner.key(), note.path.clone()),
                title,
                subtitle: body_subtitle,
                toolbar,
                file_line,
                save_row,
                editable,
                body,
                notices,
                disk,
                draft: self.draft.as_str().to_owned(),
                draft_revision: self.draft_revision,
                link_choices,
            },
            footer,
        ))
    }

    /// The subtitle line the reader starts from: the open note's list line.
    pub fn open_subtitle(&self) -> String {
        self.selected
            .zip(self.loaded.as_ref())
            .and_then(|(i, loaded)| loaded.subtitles().get(i).cloned())
            .unwrap_or_default()
    }

    // -----------------------------------------------------------------------------------------
    // The reader's acts.

    /// The mode switch: Read or Edit Markdown.
    pub fn set_source(&mut self, source: bool) {
        if self.source == source {
            return;
        }
        let error = self.open_note().is_some_and(|note| note.error.is_some());
        let fault = self.open_note().is_some_and(|note| note.fault.is_some());
        if fault {
            return;
        }
        self.source = source;
        let _ = self.kept.take_scope_notice();
        self.focus_repair = self.source && error;
    }

    /// The draft's new words, from the editor.
    pub fn edit_draft(&mut self, words: &str) {
        if self.draft.as_str() == words {
            return;
        }
        let _ = self.draft.replace(words);
        self.draft_revision += 1;
        if let Some(note) = self.open_note()
            && !self.saved_is_draft(note)
        {
            let _ = self.kept.take_discarded();
        }
    }

    /// Save, or Ctrl+S: only when the save row would save.
    pub fn save(&mut self, campaigns: &Campaigns) {
        if self.save_allowed(campaigns) {
            self.save_selected(false);
        }
    }

    /// Whether Save would run now (`DraftState::can_save` for the open draft).
    pub fn save_allowed(&mut self, campaigns: &Campaigns) -> bool {
        let Some(i) = self.selected else {
            return false;
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        if !note.owner.editable() || self.disk_change.is_some() {
            return false;
        }
        let _ = campaigns;
        let dirty = !self.saved_is_draft(note);
        if !dirty {
            return false;
        }
        let parsed = self.parsed_draft(i);
        let note = &self.loaded.as_ref().unwrap().notes[i];
        self.loaded.as_ref().unwrap().draft_refusal(i, &parsed).is_none() || note.error.is_some()
    }

    /// Discard: the saved text comes back, with an Undo.
    pub fn discard(&mut self) {
        let Some(i) = self.selected else {
            return;
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        Self::discard_draft(
            (
                &mut self.kept,
                &mut self.draft,
                &mut self.draft_revision,
                &mut self.error,
            ),
            note,
        );
    }

    /// Back: while a draft is open, the entry Back would open goes beside it in Lookup.
    pub fn back(&mut self) {
        if self.dirty() {
            self.peek = self.trail.peek(Step::Back).and_then(|target| {
                let notes = &self.loaded.as_ref()?.notes;
                notes[target.find(notes)?].entry.as_ref().map(|e| e.id)
            });
        } else {
            self.go(Step::Back);
        }
    }

    /// A toolbar extra.
    pub fn reader_act(&mut self, act: ReaderAct, campaigns: &Campaigns) {
        let Some(note) = self.open_note() else {
            return;
        };
        match act {
            ReaderAct::Copy => {
                self.copy = note
                    .entry
                    .as_ref()
                    .map(|entry| crate::content_workspace::CopySource {
                        entry: entry.clone(),
                        text: note.text.to_string(),
                        root: note.owner.root().map(Path::to_path_buf),
                        campaign: match &note.owner {
                            Owner::Campaign(campaign) => Some(campaign.clone()),
                            _ => None,
                        },
                    });
            }
            ReaderAct::Make => {
                if let (Owner::Campaign(campaign), Some(entry)) = (&note.owner, &note.entry) {
                    let campaign = campaign.clone();
                    match crate::world::make_npc_note(&campaign, entry) {
                        Ok(id) => {
                            self.stale = true;
                            self.content_changed = Some(campaign.path.clone());
                            self.open_world_npc = Some(id);
                        }
                        Err(error) => self.error = Some(error.into()),
                    }
                }
            }
            ReaderAct::Choose => {
                if let Owner::Package(module) = &note.owner {
                    if note.entry.as_ref().is_some_and(|e| e.kind == "package") {
                        let module = module.clone();
                        self.choose_all(&module, campaigns);
                    } else if let Some(entry) = &note.entry {
                        self.use_entry = Some((module.id, entry.id));
                    }
                }
            }
        }
    }

    /// The disk change's safe choice: overwrite (or save again, or save at the moved path).
    pub fn overwrite_with_draft(&mut self) {
        let Some(i) = self.selected else {
            return;
        };
        let mut file_gone = false;
        if let Some(change) = self.disk_change.take() {
            if let Some(path) = change.moved_to {
                self.loaded.as_mut().unwrap().notes[i].path = path;
                self.stale = true;
            }
            match change.text {
                Some(text) => {
                    self.loaded.as_mut().unwrap().notes[i].text =
                        crate::design::memo::Revised::new(text)
                }
                None => file_gone = true,
            }
            self.loaded.as_mut().unwrap().invalidate();
        }
        self.save_selected(file_gone);
    }

    /// The disk change's red choice: the file wins and the draft goes, with an Undo.
    pub fn lose_draft(&mut self) {
        if let Some(i) = self.selected {
            self.reload_note_from_disk(i);
        }
    }

    /// Try again: the open file is read again.
    pub fn read_again(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        if let Some(root) = self
            .open_note()
            .and_then(|note| note.owner.root().map(Path::to_path_buf))
        {
            self.refresh_destination(&root, ctx, campaigns);
        }
    }

    /// A link in the reader's body: it opens its entry, offers its choices, or, beside a draft,
    /// opens in Lookup.
    pub fn follow_link(&mut self, target: &str, index: Option<&crate::lookup::ContentIndex>) {
        let Some(index) = index else {
            return;
        };
        let from = self
            .open_note()
            .and_then(|note| note.entry.as_ref().map(|entry| entry.id));
        self.link_choices = match index.resolve_link(target, from) {
            crate::world::Go::Item(at, _) => vec![index.items[at].entry.id],
            crate::world::Go::Many(positions) => positions
                .iter()
                .map(|&at| index.items[at].entry.id)
                .collect(),
            _ => Vec::new(),
        };
        if self.link_choices.len() == 1 {
            let id = self.link_choices.remove(0);
            match self.loaded.as_ref().and_then(|loaded| {
                loaded
                    .notes
                    .iter()
                    .position(|n| n.entry.as_ref().is_some_and(|e| e.id == id))
            }) {
                Some(at) => self.open_link(at),
                None => self.peek = Some(id),
            }
        }
    }

    /// One of a link's choices, or a link that found one entry.
    pub fn open_link(&mut self, index: usize) {
        self.link_choices.clear();
        if self.dirty() {
            // Keep the draft. The linked entry opens beside it, in Lookup.
            self.peek = self
                .loaded
                .as_ref()
                .and_then(|l| l.notes.get(index))
                .and_then(|n| n.entry.as_ref())
                .map(|e| e.id);
        } else {
            self.open_entry(index);
            // Followed from the text, so the list may not show it yet.
            self.reveal_row = true;
        }
    }

    /// The draft's title for the reader's tests.
    pub fn open_name(&self) -> Option<&str> {
        self.open_note().map(Note::name)
    }
}

// ---------------------------------------------------------------------------------------------
// The forms.

/// The New entry form this frame.
pub struct NewEntryView {
    pub name: String,
    pub kind: String,
    pub kinds: Vec<(String, String)>,
    pub places: Vec<String>,
    pub place: usize,
    pub folder: String,
    /// The `Adds this entry to the campaign’s package choices` box, when it shows: ticked.
    pub use_box: Option<bool>,
    pub blocked: Option<&'static str>,
    /// The file Create entry will write: its folder, path and destination.
    pub preview: Option<(PathBuf, String, String)>,
    /// Folder suggestions for the folder field.
    pub root: Option<PathBuf>,
}

/// Manage packages this frame.
pub struct PackageFormView {
    pub path: String,
    pub add_blocked: Option<String>,
    pub browse_blocked: Option<String>,
    /// A copied folder: its warning words, whether the name form is open, and its id.
    pub copy: Option<CopyView>,
    pub name: String,
    pub folder: String,
    pub create_blocked: Option<String>,
    pub create_primary: bool,
    pub clear: bool,
    pub preview: Option<String>,
    pub chosen: Option<bool>,
    pub registered: Vec<(Uuid, String, String)>,
    pub unregister_blocked: Option<String>,
}

/// A copied package folder, in Manage packages.
pub struct CopyView {
    pub warning: String,
    pub naming: bool,
    pub id: String,
    pub repair_blocked: Option<String>,
    pub name: String,
    pub chosen: Option<bool>,
    pub help: &'static str,
    pub register_blocked: Option<String>,
    pub register_primary: bool,
}

/// The unregister confirmation.
pub struct UnregisterView {
    pub title: String,
    pub words: String,
    pub users: Option<String>,
    pub folder: String,
}

impl Library {
    /// The places a new entry can go: the campaign, then each package.
    fn new_places(campaigns: &Campaigns) -> Vec<Owner> {
        let mut places: Vec<Owner> = campaigns
            .modules()
            .iter()
            .cloned()
            .map(Owner::Package)
            .collect();
        if let Some(campaign) = campaigns.active_campaign() {
            places.insert(0, Owner::Campaign(campaign));
        }
        places
    }

    /// The New entry form's chosen place, and the default.
    fn new_place_now(&self, places: &[Owner]) -> (usize, usize) {
        let in_scope = places.iter().position(|owner| match self.scope {
            Scope::Package(id) => owner.is_package(id),
            Scope::Campaign => matches!(owner, Owner::Campaign(_)),
            _ => false,
        });
        let default_place = in_scope.unwrap_or(0);
        (
            self.new_place
                .filter(|place| *place < places.len())
                .unwrap_or(default_place),
            default_place,
        )
    }

    /// The New entry form, or None when it has nothing to offer.
    pub fn new_entry_view(&mut self, campaigns: &Campaigns) -> Option<NewEntryView> {
        if self.new_kind.is_empty() {
            self.new_kind = "creature".into();
            self.new_folder = kind_folder(&self.new_kind).into();
        }
        self.loaded.as_ref()?;
        let places = Self::new_places(campaigns);
        if places.is_empty() {
            return None;
        }
        let (place, _) = self.new_place_now(&places);
        let word = |kind: &str| {
            let word = kind.replace('_', " ");
            format!("{}{}", word[..1].to_uppercase(), &word[1..])
        };
        let kinds = ["creature", "spell", "item", "effect", "rule", "rule_group"]
            .into_iter()
            .map(|kind| (kind.to_owned(), word(kind)))
            .collect();
        let blocked = if self.dirty() {
            Some("Save or discard this entry first.")
        } else if self.new_name.trim().is_empty() {
            Some("Type a name first.")
        } else {
            None
        };
        let relative = self.new_relative();
        let destination = match &places[place] {
            Owner::Package(package) => package.name.clone(),
            Owner::Campaign(campaign) => campaign.name.clone(),
            Owner::Builtin => BUILTIN_NAME.to_owned(),
        };
        Some(NewEntryView {
            name: self.new_name.clone(),
            kind: self.new_kind.clone(),
            kinds,
            places: places.iter().map(Owner::label).collect(),
            place,
            folder: self.new_folder.clone(),
            use_box: (matches!(places[place], Owner::Package(_))
                && campaigns.active_campaign().is_some())
            .then_some(!self.new_not_used),
            blocked,
            preview: (blocked.is_none() && !self.new_name.trim().is_empty())
                .then(|| {
                    places[place]
                        .root()
                        .map(|root| (root.to_path_buf(), relative.clone(), destination.clone()))
                })
                .flatten(),
            root: places[place].root().map(Path::to_path_buf),
        })
    }

    fn new_relative(&self) -> String {
        if self.new_folder.trim().is_empty() {
            format!("{}.md", self.new_name.trim())
        } else {
            format!(
                "{}/{}.md",
                self.new_folder.trim().trim_end_matches('/'),
                self.new_name.trim()
            )
        }
    }

    pub fn set_new_name(&mut self, words: &str) {
        if self.new_name != words {
            self.new_name = words.to_owned();
            // The create attempt's error ends when the DM changes the name.
            self.new_error = None;
        }
    }

    pub fn set_new_kind(&mut self, kind: &str) {
        if self.new_kind != kind {
            self.new_kind = kind.to_owned();
            self.new_folder = kind_folder(&self.new_kind).into();
        }
    }

    pub fn set_new_place(&mut self, place: usize, campaigns: &Campaigns) {
        let places = Self::new_places(campaigns);
        let (_, default_place) = self.new_place_now(&places);
        self.new_place = (place != default_place).then_some(place);
    }

    pub fn set_new_folder(&mut self, words: &str) {
        self.new_folder = words.to_owned();
    }

    pub fn toggle_new_used(&mut self) {
        self.new_not_used = !self.new_not_used;
    }

    /// Cancel, or Escape: the New entry form closes.
    pub fn close_new_entry(&mut self) {
        self.new_form = false;
        self.new_error = None;
    }

    /// Create entry (`new_entry`'s create).
    pub fn create_entry(&mut self, campaigns: &Campaigns) {
        if self.dirty() || self.new_name.trim().is_empty() || self.loaded.is_none() {
            return;
        }
        let places = Self::new_places(campaigns);
        if places.is_empty() {
            return;
        }
        let (place, _) = self.new_place_now(&places);
        let relative = self.new_relative();
        let name = self.new_name.trim();
        if name.contains(['/', '\\', '\n', '\r']) || name == "." || name == ".." {
            self.new_error = Some(PageError::Own(NAME_WITHOUT_SEPARATORS.clone()));
            return;
        }
        let path = relative;
        let text = note_text(name, &self.new_kind, None);
        let owner = places[place].clone();
        let root = owner.root().expect("a writable owner").to_path_buf();
        let left = self.here();
        match write_note(&root, &path, None, &text) {
            Ok(entry) => {
                let used = self.choose_new_entry(&owner, entry.id, campaigns);
                let l = self.loaded.as_mut().unwrap();
                if !self.new_not_used
                    && used.is_ok()
                    && let (Some(uses), Some(chosen)) = (&mut l.uses, &mut l.chosen)
                {
                    uses.insert(entry.id);
                    chosen.insert(entry.id);
                }
                let created_path = path.clone();
                let created_owner = owner.key();
                l.notes.push(Note {
                    path,
                    text: crate::design::memo::Revised::new(text.clone()),
                    entry: Some(entry),
                    error: None,
                    fault: None,
                    manifest_gone: false,
                    owner,
                });
                l.notes.sort_by_cached_key(|n| natural_key(n.name()));
                l.invalidate();
                self.matches = None;
                self.scope_counts = None;
                self.selected = file_at(&l.notes, (created_owner, &created_path));
                if self.selected.is_some() {
                    self.trail.leave(left);
                }
                self.reveal_row = true;
                self.search.clear();
                self.section = Section::All;
                self.source = true;
                let _ = self.draft.replace(text);
                self.focus_hp = self.new_kind == "creature";
                self.new_name.clear();
                self.new_error = None;
                self.new_folder.clear();
                self.new_form = false;
                self.error = used.err();
                if self
                    .selected
                    .is_some_and(|i| !l.in_scope(&l.notes[i], self.scope))
                {
                    self.scope = Scope::All;
                }
                self.content_changed = Some(root);
            }
            Err(e) => self.new_error = Some(e.into()),
        }
    }

    /// Manage packages, this frame. It also ends a copied folder's state when the path left it.
    pub fn package_form_view(&mut self, campaigns: &Campaigns) -> PackageFormView {
        if self.copied_package.as_ref().is_some_and(|copy| {
            Path::new(self.path.trim()).canonicalize().ok().as_ref() != Some(&copy.path)
        }) {
            self.copied_package = None;
            self.copy_name_open = false;
        }
        let lock = self.package_lock();
        let add_reason = lock
            .or(self.copied_package.is_some().then_some(PackageReason::Copy))
            .or(self
                .path
                .trim()
                .is_empty()
                .then_some(PackageReason::NoFolder));
        let campaign = campaigns.active_campaign().is_some();
        let copy = self.copied_package.as_ref().map(|copy| {
            let name = campaigns
                .modules()
                .iter()
                .find(|m| m.id == copy.id)
                .map_or(copy.name.as_str(), |m| m.name.as_str());
            let warning = crate::sentence!("This folder is a copy of {}.", name);
            let chosen = !self.copy_not_used;
            CopyView {
                warning: fault_line(&LibraryFault::Warning(&warning)).1,
                naming: self.copy_name_open,
                id: copy.id.to_string(),
                repair_blocked: self.page_lock().map(|reason| reason.as_str().to_owned()),
                name: self.copy_name.clone(),
                chosen: campaign.then_some(chosen),
                help: if chosen {
                    "Uses every entry in this package, including future entries."
                } else {
                    "This campaign keeps the original. The copy is registered but not chosen."
                },
                register_blocked: self
                    .package_lock()
                    .or(self
                        .copy_name
                        .trim()
                        .is_empty()
                        .then_some(PackageReason::NoName))
                    .map(|reason| reason_words(reason).to_owned()),
                register_primary: !self.run_open,
            }
        });
        let parent = self.package_parent(campaigns);
        let namespace = Self::namespace(&self.name);
        let preview = (!self.name.trim().is_empty())
            .then(|| format!("Will create {}", parent.join(&namespace).display()));
        PackageFormView {
            path: self.path.clone(),
            add_blocked: add_reason.map(|reason| reason_words(reason).to_owned()),
            browse_blocked: lock.map(|_| "Browse waits for the reason below.".to_owned()),
            copy,
            name: self.name.clone(),
            folder: parent.display().to_string(),
            create_blocked: self
                .package_lock()
                .or(self.name.trim().is_empty().then_some(PackageReason::NoName))
                .map(|reason| reason_words(reason).to_owned()),
            create_primary: !self.run_open,
            clear: !self.name.is_empty() || self.package_folder_changed,
            preview,
            chosen: campaign.then_some(!self.package_not_used),
            registered: campaigns
                .modules()
                .iter()
                .map(|module| {
                    (
                        module.id,
                        module.name.clone(),
                        module.path.display().to_string(),
                    )
                })
                .collect(),
            unregister_blocked: self.page_locked().map(str::to_owned),
        }
    }

    fn package_parent(&mut self, campaigns: &Campaigns) -> PathBuf {
        let default_parent = || {
            campaigns
                .modules()
                .last()
                .and_then(|module| module.path.parent())
                .map(Path::to_path_buf)
                .or_else(|| {
                    campaigns
                        .package_folder("new_package")
                        .ok()
                        .and_then(|p| p.parent().map(Path::to_path_buf))
                })
                .unwrap_or_default()
        };
        self.package_path.get_or_insert_with(default_parent).clone()
    }

    fn namespace(name: &str) -> String {
        name.trim()
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    }

    pub fn close_package_form(&mut self) {
        self.form = false;
    }

    pub fn set_package_path(&mut self, words: &str) {
        self.path = words.to_owned();
    }

    pub fn browse_package_path(&mut self) {
        if self.package_lock().is_none()
            && let Some(path) = crate::hermetic::pick_folder()
        {
            self.path = path.display().to_string();
        }
    }

    /// Add folder: register the folder, or find that it is a copy of a registered package.
    pub fn add_package_folder(&mut self, campaigns: &mut Campaigns) {
        if self.package_lock().is_some()
            || self.copied_package.is_some()
            || self.path.trim().is_empty()
        {
            return;
        }
        match local_module(Path::new(self.path.trim())).and_then(|module| {
            if let Some(original) = campaigns
                .modules()
                .iter()
                .find(|m| m.id == module.id && m.path != module.path)
            {
                self.copy_name = format!("{} copy", original.name);
                self.copy_name_open = false;
                self.copy_not_used = campaigns
                    .active_campaign()
                    .and_then(|campaign| CampaignDocument::load(&campaign).ok())
                    .is_some_and(|document| {
                        document
                            .selections
                            .iter()
                            .any(|selection| selection.package.id == Some(module.id))
                    });
                self.copied_package = Some(module);
                self.error = None;
                return Ok(None);
            }
            campaigns.register_module(module.clone())?;
            Ok(Some(module))
        }) {
            Ok(Some(module)) => {
                self.scope = Scope::Package(module.id);
                self.form = false;
                self.stale = true;
            }
            Ok(None) => {}
            Err(error) => self.error = Some(error.into()),
        }
    }

    /// `Register as a new package`: the name form opens.
    pub fn open_copy_name(&mut self) {
        if self.page_lock().is_none() {
            self.copy_name_open = true;
        }
    }

    pub fn cancel_copy_name(&mut self) {
        self.copy_name_open = false;
    }

    pub fn set_copy_name(&mut self, words: &str) {
        self.copy_name = words.to_owned();
    }

    pub fn toggle_copy_used(&mut self) {
        self.copy_not_used = !self.copy_not_used;
    }

    /// Register: the copy gets a new identity and is registered.
    pub fn register_copied_package(&mut self, campaigns: &mut Campaigns) {
        if self.package_lock().is_some() || self.copy_name.trim().is_empty() {
            return;
        }
        if let Err(error) = self.register_copy(campaigns) {
            self.error = Some(error.into());
        }
    }

    pub fn set_package_name(&mut self, words: &str) {
        self.name = words.to_owned();
    }

    pub fn set_package_folder(&mut self, words: &str, campaigns: &Campaigns) {
        if words != self.package_parent(campaigns).display().to_string() {
            self.package_path = Some(PathBuf::from(words));
            self.package_folder_changed = true;
        }
    }

    pub fn browse_package_folder(&mut self) {
        if let Some(chosen) = crate::hermetic::pick_folder() {
            self.package_path = Some(chosen);
            self.package_folder_changed = true;
        }
    }

    pub fn toggle_package_used(&mut self) {
        self.package_not_used = !self.package_not_used;
    }

    /// Clear: the new package's name and folder go back.
    pub fn clear_package(&mut self) {
        self.name.clear();
        self.package_path = None;
        self.package_folder_changed = false;
    }

    /// Create package: the folder and its manifest, registered, chosen when asked; the New
    /// entry form opens in it.
    pub fn create_package(&mut self, campaigns: &mut Campaigns) {
        if self.package_lock().is_some() || self.name.trim().is_empty() {
            return;
        }
        let namespace = Self::namespace(&self.name);
        let root = self.package_parent(campaigns).join(&namespace);
        let created = fs::create_dir_all(root.parent().unwrap_or(Path::new("")))
            .map_err(|e| e.to_string())
            .and_then(|()| create_module(&root, &self.name, &namespace))
            .and_then(|module| {
                campaigns.register_module(module.clone())?;
                if !self.package_not_used
                    && let Some(campaign) = campaigns.active_campaign()
                {
                    crate::content_workspace::use_in_campaign(
                        &campaign,
                        module.id,
                        &module.name,
                        None,
                    )?;
                }
                Ok(module)
            });
        match created {
            Ok(module) => {
                self.content_changed = campaigns.active_campaign().map(|c| c.path);
                self.scope = Scope::Package(module.id);
                self.form = false;
                self.new_form = true;
                self.new_place = None;
                self.new_name.clear();
                self.new_folder = kind_folder(&self.new_kind).into();
                self.new_not_used = false;
                self.focus_new_name = true;
                self.stale = true;
            }
            Err(error) => self.error = Some(error.into()),
        }
    }

    /// `Unregister…` on a registered package's menu: the confirmation opens.
    pub fn ask_unregister(&mut self, id: Uuid, campaigns: &Campaigns) {
        if self.page_locked().is_some() {
            return;
        }
        self.confirm_unregister = campaigns.modules().iter().find(|m| m.id == id).cloned();
    }

    /// The unregister confirmation, while it is open.
    pub fn unregister_view(&self, campaigns: &Campaigns) -> Option<UnregisterView> {
        let module = self.confirm_unregister.as_ref()?;
        let count = self
            .loaded
            .as_ref()
            .map_or(0, |l| unregister_count(&l.notes, module.id));
        let users = campaigns.active_campaign().and_then(|campaign| {
            let catalogue =
                crate::content_workspace::Catalogue::load(campaigns.modules(), Some(&campaign))
                    .ok()?;
            let entry_ids: Vec<_> = catalogue
                .entries
                .values()
                .filter(|entry| entry.owner == module.id && catalogue.selectable(entry.entry.id))
                .map(|entry| crate::content_workspace::EntryRef::id(entry.entry.id))
                .collect();
            let selection = crate::content_workspace::PackageSelection {
                name: Some(module.name.clone()),
                package: crate::content_workspace::EntryRef::id(module.id),
                selection: crate::content_workspace::Selection::Selected { entries: entry_ids },
            };
            let users = selection.campaign_users(&catalogue);
            if users.is_empty() {
                return None;
            }
            let mut names = Vec::new();
            for (id, _) in &users {
                if let Some(user) = catalogue.entries.get(id) {
                    for reference in content::required_references(&user.entry).unwrap_or_default() {
                        if let Ok(id) = catalogue.resolve(&reference)
                            && let Some(entry) = catalogue.entries.get(&id)
                            && entry.owner == module.id
                        {
                            names.push(entry.entry.name.clone());
                        }
                    }
                }
            }
            names.sort();
            names.dedup();
            let users: Vec<_> = users.into_iter().map(|(_, name)| name).collect();
            Some(format!(
                "{} uses {}, in {}.",
                campaign.name,
                name_list(&names),
                name_list(&users)
            ))
        });
        Some(UnregisterView {
            title: format!("Unregister {}?", module.name),
            words: format!(
                "{} leave Splat’s lists, and any campaign that selects them can no longer use them.",
                design::count(count, "entry", "entries")
            ),
            users,
            folder: format!("The files stay in {}.", module.path.display()),
        })
    }

    /// Keep the package: the confirmation closes.
    pub fn keep_package(&mut self) {
        self.confirm_unregister = None;
    }

    /// Unregister: the package leaves Splat's lists; its files stay.
    pub fn confirm_unregister(&mut self, campaigns: &mut Campaigns) {
        if let Some(module) = self.confirm_unregister.take() {
            self.unregister(module, campaigns);
        }
    }

    /// Whether a modal confirmation is open.
    pub fn unregister_open(&self) -> bool {
        self.confirm_unregister.is_some()
    }
}
