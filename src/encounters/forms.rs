//! The encounter editor's other views as plain data and acts, for the GPUI page: the creature pane that `Make it yours` opens, a missing row's
//! replacement picker, the search's `Use it` lines, and the narrow layout's tabs (`replace_missing`).
#![warn(deprecated)]
use super::*;
use crate::state_context::StateContext;
use model::ResultRow;

/// The creature open beside the roster (`creature_pane`), as data.
pub struct CreatureView {
    pub path: String,
    pub name: String,
    pub hp: String,
    pub armor_class: String,
    pub body: String,
    pub dirty: bool,
    /// The save row's word (`DraftState`): its icon, words and colour.
    pub state: (String, String, crate::design::Color),
    /// Why Save waits (`Nothing to save` while unchanged).
    pub save_blocked: Option<String>,
    /// `Discard creature changes` shows.
    pub discard: bool,
    /// The last Save failed: the save row says `Not saved` and the footer says why.
    pub error: Option<String>,
    /// Hit points take the keyboard (the pane just opened); asking ends it.
    pub focus_hp: bool,
    /// The creature's file changed on disk: the conflict shows in the pane's place.
    pub disk: Option<disk::DiskView>,
}

/// A creature pane's field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreatureField {
    Name,
    Hp,
    ArmorClass,
    Body,
}

/// A press in the creature pane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreatureAct {
    Save,
    /// `Done`, or `Discard creature changes`: the pane closes and its changes go.
    Close,
}

/// Which pane the narrow encounter or group editor shows (`EditorPane`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NarrowPane {
    /// The roster, or a group's members.
    Roster,
    /// Add participants, Add creatures, or the open creature.
    Add,
}

/// A search with no match as typed: a creature in a package the campaign does not use, with
/// `Use it` (the package and entry `use_package_entry` takes).
pub struct UnusedCreature {
    pub package: Uuid,
    pub entry: Uuid,
    pub words: String,
}

impl Draft {
    /// `Use` in a missing row's replacement picker: the row (and with `all` every row of the
    /// same missing creature) takes `creature`, keeping its old name as its changed name, with a
    /// result whose Undo gives the rows back.
    pub(super) fn replace_missing(
        &mut self,
        selected: usize,
        creature: Uuid,
        all: bool,
        snapshot: &Snapshot,
    ) {
        let before = self.setup.roster.clone();
        replace_roster(&mut self.setup.roster, selected, creature, all, snapshot);
        let old = roster_name(&before[selected], None, &snapshot.known_creature_names);
        let rows = before
            .iter()
            .zip(&self.setup.roster)
            .filter(|(was, now)| was.creature != now.creature)
            .count();
        let new = snapshot
            .catalogue
            .entries
            .get(&creature)
            .map_or_else(|| "the new creature".to_owned(), |e| e.entry.name.clone());
        self.new_results.push(DraftUndo::Replacement {
            before,
            after: self.setup.roster.clone(),
            words: replacement_words(&old, rows, &new),
            replaced: old,
        });
    }
}

impl Encounters {
    /// The creature pane, while a creature is open beside the roster.
    pub fn creature_view(&mut self) -> Option<CreatureView> {
        let editor = self.creature.as_mut()?;
        let focus_hp = std::mem::take(&mut editor.focus);
        let disk = editor.recovery.view(
            &editor.text().unwrap_or_default(),
            ("creature", &editor.fields.name),
        );
        let dirty = editor.dirty();
        // M0186: the save row by its one rule. The encounter beside it owns the primary, so Save
        // stays secondary; a failed Save says `Not saved` until the next one works.
        let mut state = design::DraftState::new(
            design::save_row::DraftOrigin::Saved,
            if dirty {
                design::save_row::Changed::Yes
            } else {
                design::save_row::Changed::No
            },
            design::save_row::Primary::Elsewhere,
        )
        .discard_named("Discard creature changes");
        if editor.error.is_some() {
            state = state.not_saved();
        }
        let status = state.status();
        Some(CreatureView {
            state: (status.icon.to_owned(), status.word.to_owned(), status.color),
            save_blocked: state.save_blocked().map(str::to_owned),
            discard: state.discard_shown(),
            path: editor.path.clone(),
            name: editor.fields.name.clone(),
            hp: editor.fields.hp.clone(),
            armor_class: editor.fields.armor_class.clone(),
            body: editor.fields.body.to_string(),
            dirty,
            error: editor.error.clone(),
            focus_hp,
            disk,
        })
    }

    /// Whether a creature is open beside the roster.
    pub fn creature_open(&self) -> bool {
        self.creature.is_some()
    }

    /// One field of the open creature, as typed.
    pub fn edit_creature(&mut self, field: CreatureField, words: &str) {
        let Some(editor) = self.creature.as_mut() else {
            return;
        };
        match field {
            CreatureField::Name => editor.fields.name = words.to_owned(),
            CreatureField::Hp => editor.fields.hp = words.to_owned(),
            CreatureField::ArmorClass => editor.fields.armor_class = words.to_owned(),
            CreatureField::Body => {
                if editor.fields.body.as_str() != words {
                    let _ = editor.fields.body.replace(words);
                }
            }
        }
    }

    /// A press in the creature pane: Save writes it (a refusal stays in its footer), Close
    /// closes it (`after_panes`).
    pub fn creature_act(&mut self, act: CreatureAct, active: &Campaign, modules: &[Module]) {
        match act {
            CreatureAct::Save => self.save_creature(active, modules),
            CreatureAct::Close => {
                self.after_panes(Some(CreatureStep::Close), None, active, modules);
            }
        }
    }

    /// The DM's choice in the creature's disk conflict (`creature_pane`): `keep` takes the
    /// file's place and text as the baseline, so the next Save overwrites it; else the pane
    /// closes and its changes go.
    pub fn creature_disk_choice(&mut self, keep: bool, active: &Campaign, modules: &[Module]) {
        let Some(editor) = self.creature.as_mut() else {
            return;
        };
        if keep {
            if let Some(change) = editor.recovery.accept() {
                editor.path = change.path;
                editor.baseline = change.text.unwrap_or_default();
                editor.error = None;
            }
        } else {
            self.after_panes(Some(CreatureStep::Close), None, active, modules);
        }
    }

    /// The pane the narrow editor shows.
    pub fn narrow_pane(&self) -> NarrowPane {
        match self.editor_pane {
            EditorPane::Roster => NarrowPane::Roster,
            EditorPane::Add => NarrowPane::Add,
        }
    }

    /// A tab of the narrow editor.
    pub fn show_narrow_pane(&mut self, pane: NarrowPane) {
        self.editor_pane = match pane {
            NarrowPane::Roster => EditorPane::Roster,
            NarrowPane::Add => EditorPane::Add,
        };
    }

    /// A missing row's replacement picker: the creatures it offers for `query`, best first, at
    /// most five (`replacement_indices`).
    pub fn replacement_rows(&self, query: &str) -> Vec<ResultRow> {
        let Some(snapshot) = self.snapshot.as_ref() else {
            return Vec::new();
        };
        replacement_indices(&snapshot.creatures, &query.to_lowercase())
            .into_iter()
            .filter_map(|at| snapshot.creatures.get(at))
            .map(|c| ResultRow {
                id: c.id,
                party: c.party,
                name: c.name.clone(),
                detail: c.detail.clone(),
                hp_missing: c.hp_missing,
                source: c.source.clone(),
                in_roster: false,
                unavailable: c.unavailable.clone(),
            })
            .collect()
    }

    /// How many roster rows name the same missing creature as the row `key`.
    pub fn missing_rows(&self, key: &str) -> usize {
        let Some(draft) = self.draft.as_ref() else {
            return 0;
        };
        let Some(slot) = draft.setup.roster.iter().find(|slot| slot.key == key) else {
            return 0;
        };
        draft
            .setup
            .roster
            .iter()
            .filter(|other| other.creature == slot.creature)
            .count()
    }

    /// `Use` in a missing row's replacement picker (`roster`).
    pub fn replace_row(&mut self, key: &str, creature: Uuid, all: bool) {
        let Encounters {
            snapshot,
            draft,
            expanded,
            ..
        } = self;
        let (Some(snapshot), Some(draft)) = (snapshot.as_ref(), draft.as_mut()) else {
            return;
        };
        let Some(selected) = draft.setup.roster.iter().position(|slot| slot.key == key) else {
            return;
        };
        draft.replace_missing(selected, creature, all, snapshot);
        draft.keep_sides();
        *expanded = None;
    }

    /// The search's lines for creatures in packages this campaign does not use, while no
    /// creature matches as typed (`creature_search_with_summary`). Kept by the link index for
    /// the words.
    pub fn unused_creatures(&self, ctx: &StateContext) -> Vec<UnusedCreature> {
        let Some((ranked, _)) = self.search_ranked.as_ref() else {
            return Vec::new();
        };
        if ranked.as_typed > 0 || self.search.trim().is_empty() {
            return Vec::new();
        }
        let Some(index) = ctx.get(crate::requests::LINK_INDEX) else {
            return Vec::new();
        };
        let query = self.search.to_lowercase();
        // A copied note's duplicates are said first, and nothing more (`duplicate_creature_results`).
        if hidden_duplicates(&index, &query) {
            return Vec::new();
        }
        let parsed = Query::parse(&query);
        index
            .unused_creatures_kept(ctx, "encounter_creature_search", &parsed.words)
            .iter()
            .map(|(package, entry, name, owner)| UnusedCreature {
                package: *package,
                entry: *entry,
                words: format!("{name} is in {owner}, which this campaign does not use."),
            })
            .collect()
    }
}

/// A copied note the creature search finds (`duplicate_creature_results`, M0178): the fault's
/// words, then each file's repair with what it leaves as its help line.
#[derive(Clone, Debug, PartialEq)]
pub struct DuplicateRepair {
    /// `These two files claim to be the same note.` (or `entry`).
    pub warning: String,
    /// Each file: its path in the campaign, its repair (`Make Rook copy.md a separate note`),
    /// and what the repair leaves the encounters that use it with.
    pub files: Vec<(String, String, Option<String>)>,
}

impl Encounters {
    /// The search's repairs of copied notes: each creature or fighting World note whose name
    /// holds the search's words and that another file claims too, once per identity. The
    /// search says these first; while there are any, it does not say `No creatures match`.
    pub fn duplicate_repairs(&self, ctx: &StateContext) -> Vec<DuplicateRepair> {
        let Some(index) = ctx.get(crate::requests::LINK_INDEX) else {
            return Vec::new();
        };
        if index.campaign.is_none() {
            return Vec::new();
        }
        let query = self.search.to_lowercase();
        let mut seen = std::collections::BTreeSet::new();
        let mut repairs = Vec::new();
        for item in &index.items {
            if !item.entry.name.to_lowercase().contains(&query)
                || !(item.entry.kind == "creature"
                    || (item.entry.kind == "world" && crate::world::fights(&item.entry)))
            {
                continue;
            }
            let Some(paths) = index.duplicate_world.get(&item.entry.path) else {
                continue;
            };
            let mut identity = paths.clone();
            identity.sort();
            if !seen.insert(identity) {
                continue;
            }
            let users: Vec<String> = index
                .items
                .iter()
                .filter(|other| {
                    other.entry.kind == "encounter"
                        && crate::library::encounter_uses_entry(&other.entry, &item.entry)
                })
                .map(|other| other.entry.name.clone())
                .collect();
            let noun = crate::world::IdentityNoun::of(&item.entry);
            let file = |path: &str| path.rsplit('/').next().unwrap_or(path).to_owned();
            let files = paths
                .iter()
                .enumerate()
                .map(|(position, path)| {
                    let help = paths
                        .iter()
                        .find(|other| *other != path)
                        .filter(|_| !users.is_empty())
                        .map(|remaining| {
                            crate::world::duplicate_consequence(
                                crate::world::KeptBy::Encounters(&users),
                                &file(remaining),
                                position == 0,
                            )
                            .as_str()
                            .to_owned()
                        });
                    (
                        path.clone(),
                        crate::world::separate_choice(noun, &file(path)).label(),
                        help,
                    )
                })
                .collect();
            repairs.push(DuplicateRepair {
                warning: crate::design::outcome::Fault::cause(&crate::world::DuplicateIdentity(
                    noun,
                ))
                .as_str()
                .to_owned(),
                files,
            });
        }
        repairs
    }

    /// A copied note's repair pressed in the search: the file at `path` gets an identity of
    /// its own. The separation goes in the page's store, newest, with its Undo (the editors'
    /// footers show it); a failure is the reason, and the last separation stays. The shell
    /// reads the campaign again.
    pub fn separate_copy(&mut self, ctx: &StateContext, path: &str) {
        let Some(active) = self.active.clone() else {
            return;
        };
        match crate::world::new_identity(&active, path) {
            Ok((change, note)) => {
                note_identity_changed(ctx);
                self.end_separation_failure(&active.path, path);
                self.separations.record(Separation {
                    owner: SeparationOwner {
                        root: active.path.clone(),
                        note,
                    },
                    change: Arc::new(IdentityChange(change)),
                    path: path.to_owned(),
                });
            }
            Err(error) => {
                self.separation_error = Some(SeparationFailure {
                    root: active.path.clone(),
                    path: path.to_owned(),
                    words: error,
                });
            }
        }
    }
}

/// Whether the search's words find a creature note with a twin file (the search's repair
/// messages, `duplicate_creature_results`).
fn hidden_duplicates(index: &crate::lookup::ContentIndex, query: &str) -> bool {
    index.campaign.is_some()
        && index.items.iter().any(|item| {
            item.entry.name.to_lowercase().contains(query)
                && (item.entry.kind == "creature"
                    || (item.entry.kind == "world" && crate::world::fights(&item.entry)))
                && index.duplicate_world.contains_key(&item.entry.path)
        })
}
