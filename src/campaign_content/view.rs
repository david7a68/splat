//! Campaign → Packages' reads and acts, free of any toolkit, so the egui page
//! (`CampaignContent::show`) and the GPUI tab (`src/gui/pages/campaigns/packages.rs`) share
//! them.
use super::{
    Cause, CampaignContent, ContentFault, ContentPress, MISSING_CAMPAIGN, PageResult, Payload,
    end_results, manifest_mark, manifest_on_disk, missing_package, needed_by, package_count,
    removal_of,
};
use crate::campaigns::Campaigns;
use crate::content_workspace::{EntryRef, PackageSelection, Selection};
use crate::design::outcome::plain::PlainLine;
use crate::persistence::Campaign;
use eframe::egui;
use uuid::Uuid;

/// What the page can show now.
pub enum Ready {
    /// No campaign is open.
    NoCampaign,
    /// The previous campaign has changes: Discard and switch, or go back to it.
    PreviousUnsaved,
    /// The page for this campaign.
    Page(Campaign),
}

/// The save row above the package list.
pub struct SaveBar {
    /// The choices differ from Campaign.md.
    pub changed: bool,
    /// Save selections can save (changed, or never saved).
    pub dirty: bool,
    /// The repair that blocks Save, in words.
    pub repair: Option<String>,
    /// A repair's draft blocks Reload from disk.
    pub reload_blocked: bool,
    /// The reason beside the changes (`Discard` is the one way to switch while a repair stands).
    pub switch_reason: Option<&'static str>,
    /// Save selections is the primary action; an open run owns `Next turn` instead.
    pub primary: bool,
}

/// An invalid note in repair (`repair_section`): its message, and its editor while open.
pub struct RepairRow {
    pub path: String,
    /// The message's words (`ContentFault::Unreadable`): the file and, while closed, what is
    /// wrong.
    pub words: String,
    pub open: bool,
    /// What is wrong, for the message (none while the editor is open).
    pub problem: Option<String>,
    /// The draft and the line Save's check marks.
    pub draft: String,
    pub error_line: Option<usize>,
}

/// One package row.
pub struct PackageRow {
    pub id: Uuid,
    pub name: String,
    pub checked: bool,
    pub indeterminate: bool,
    pub selected: bool,
    /// `All chosen · 42 entries`, after a fault mark.
    pub detail: String,
    /// The fault mark's icon and words, when its manifest has a fault.
    pub mark: Option<(&'static str, &'static str)>,
    /// `Ember Cultist use its creatures.`
    pub note: Option<String>,
}

/// One entry row of the package on the right.
pub struct EntryRow {
    pub id: Uuid,
    pub name: String,
    pub subtitle: String,
    /// `needed by Ember Cultist`, when another choice needs it.
    pub needed: Option<String>,
    pub checked: bool,
    /// Needed by another choice: it stays chosen.
    pub locked: Option<String>,
}

/// The package on the right.
pub struct PackageView {
    pub id: Uuid,
    pub name: String,
    pub count: usize,
    pub all: bool,
    /// `Choose all` or `Include future additions`.
    pub choose_all: &'static str,
    pub chosen: bool,
}

/// One row of `Needed by your entries`.
pub struct NeededRow {
    pub id: Uuid,
    pub name: String,
    pub detail: String,
}

/// The issues above the package list.
pub struct Issues {
    pub heading: String,
    /// The Campaign.md flags, each with whether it offers `Choose in Rules and runs`.
    pub flags: Vec<(String, bool)>,
    /// The issue list shows (no repair or missing package hides it).
    pub shown: bool,
    /// Other errors, in their words.
    pub errors: Vec<String>,
    /// Missing dependencies: their words, then each owner's repair (`Repair X`, its file).
    pub missing: Vec<(String, Vec<(String, std::path::PathBuf)>)>,
    /// Campaign entries that need something gone, each a way to open it.
    pub affected: Vec<(Uuid, String)>,
}

impl CampaignContent {
    /// The page's work before it draws (`show`'s first steps): a new campaign's load, a load
    /// that came in, and a changed draft's end of the Saved result.
    pub fn prepare(&mut self, ctx: &egui::Context, campaigns: &Campaigns) -> Ready {
        let scope = (self.module, self.dependencies);
        if self.notice_scope.is_some_and(|previous| previous != scope) {
            self.clear_notice();
        }
        self.notice_scope = Some(scope);
        let Some(active) = campaigns.active_campaign() else {
            return Ready::NoCampaign;
        };
        if self.refresh_pending {
            self.reload(ctx, campaigns, active.clone());
        }
        if self
            .active
            .as_ref()
            .is_none_or(|c| c.id != active.id || c.path != active.path)
        {
            if self.dirty() {
                return Ready::PreviousUnsaved;
            }
            self.reload(ctx, campaigns, active.clone());
        }
        self.take_load(campaigns, &active);
        if self.dirty() {
            self.end_plain(PageResult::Saved);
        }
        Ready::Page(active)
    }

    /// `Discard and switch campaign`.
    pub fn discard_and_switch(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        if let Some(active) = campaigns.active_campaign() {
            self.reload(ctx, campaigns, active);
        }
    }

    /// The page is loading.
    pub fn loading(&self) -> bool {
        self.pending.is_some()
    }

    /// The package list cannot show: the page's errors are its content.
    pub fn errors_are_content(&self) -> bool {
        self.body_missing()
    }

    /// The page's errors as its content: each error's words, and for a missing Campaign.md the
    /// words and the action that writes it.
    pub fn content_errors(
        &self,
        campaigns: &Campaigns,
        active: &Campaign,
    ) -> (Option<(String, Option<&'static str>)>, Option<String>) {
        let error = self.error.as_ref().map(|error| {
            if error.contains(MISSING_CAMPAIGN) {
                let (preview, action) = campaigns.campaign_write_preview(active);
                (preview, Some(action))
            } else {
                (error.clone(), None)
            }
        });
        (error, self.reconcile.error.clone())
    }

    /// Write Campaign.md again (the missing manifest's repair).
    pub fn write_campaign_now(&mut self, campaigns: &Campaigns, active: &Campaign) {
        self.write_campaign(campaigns, active);
    }

    /// The campaign folder is missing: `Create campaign folder`.
    pub fn create_folder(&mut self, ctx: &egui::Context, campaigns: &Campaigns, active: &Campaign) {
        match std::fs::create_dir_all(&active.path) {
            Ok(()) => self.reload(ctx, campaigns, active.clone()),
            Err(e) => self.fail_or_content(e.to_string()),
        }
    }

    /// The page's footer, as plain lines (`page_footer`); `open` says whether a repair's editor
    /// is open.
    pub fn footer_lines(
        &self,
        campaigns: &Campaigns,
        active: &Campaign,
        open: &dyn Fn(&str) -> bool,
    ) -> Vec<PlainLine<ContentPress>> {
        let frame = crate::design::outcome::PaneFrame::plain();
        self.page_footer_with(&frame, open, campaigns, active, 0.0)
            .into_plain()
    }

    /// A press in the page's footer.
    pub fn footer_press(
        &mut self,
        ctx: &egui::Context,
        campaigns: &Campaigns,
        active: &Campaign,
        press: crate::design::outcome::Press<ContentPress>,
    ) {
        let write = press.id == ContentPress::WriteCampaign;
        self.press_result(ctx, campaigns, Some(press));
        if write {
            self.write_campaign(campaigns, active);
        }
    }

    /// The save row above the package list.
    pub fn save_bar(&mut self) -> Option<SaveBar> {
        let snapshot = self.snapshot.as_mut()?;
        let document = &snapshot.document;
        if snapshot.closure.0 != document.selections {
            snapshot.closure = (
                document.selections.clone(),
                snapshot
                    .catalogue
                    .closure(&document.selections, document.campaign.id),
            );
            snapshot.issues = None;
        }
        let document = &snapshot.document;
        let changed = document.selections != self.saved_selections;
        if changed {
            // While the choices differ from Campaign.md, Discard's result has ended, as the
            // egui bar ends it each frame (`choices_edited`).
            end_results(&mut self.results, document.campaign.id, |payload| {
                matches!(payload, Payload::Discarded { .. })
            });
        }
        let repair = self
            .repairs
            .first()
            .map(|repair| format!("Fix {} first.", repair.path));
        Some(SaveBar {
            changed,
            dirty: changed || document.baseline.is_none(),
            switch_reason: changed.then(|| {
                if repair.is_some() {
                    crate::content_actions::blocked_draft_switch_reason()
                } else {
                    crate::content_actions::draft_switch_reason()
                }
            }),
            repair,
            reload_blocked: self.repairs.iter().any(super::Repair::dirty),
            primary: !self.run_open,
        })
    }

    /// The choices changed: Discard's result and the Saved result end.
    pub fn choices_edited(&mut self, campaign: Uuid) {
        end_results(&mut self.results, campaign, |payload| {
            matches!(payload, Payload::Discarded { .. })
        });
    }

    /// `Discard` beside the changed choices: Campaign.md's choices come back, with an Undo.
    pub fn discard_choices(&mut self) {
        let Some(snapshot) = self.snapshot.as_mut() else {
            return;
        };
        let document = &mut snapshot.document;
        let payload = Payload::Discarded {
            selections: std::mem::replace(&mut document.selections, self.saved_selections.clone()),
            baseline: document.baseline.clone(),
        };
        super::new_result_of(self, payload);
    }

    /// `Reload from disk`.
    pub fn reload_now(&mut self, ctx: &egui::Context, campaigns: &Campaigns, active: &Campaign) {
        self.reload(ctx, campaigns, active.clone());
    }

    /// `Save selections` (or Ctrl+S).
    pub fn save_now(&mut self) {
        self.save_choices();
    }

    /// The issues above the package list.
    pub fn issues(&mut self, active: &Campaign) -> Option<Issues> {
        let snapshot = self.snapshot.as_mut()?;
        snapshot.issue_lines(active.id);
        let errors: Vec<String> = snapshot.issues.clone().unwrap_or_default();
        let catalogue = &snapshot.catalogue;
        let document = &snapshot.document;
        let closure = &snapshot.closure.1;
        let affected: Vec<_> = catalogue
            .entries
            .values()
            .filter(|entry| entry.private && entry.entry.kind == "encounter")
            .filter(|entry| {
                splat::content::required_references(&entry.entry).is_ok_and(|references| {
                    references.iter().any(|r| catalogue.resolve(r).is_err())
                })
            })
            .map(|entry| (entry.entry.id, entry.entry.name.clone()))
            .collect();
        let flags: Vec<(String, bool)> = catalogue
            .entries
            .get(&active.id)
            .map(|entry| entry.entry.flags.as_slice())
            .unwrap_or_default()
            .iter()
            .map(|flag| {
                (
                    flag.message.clone(),
                    flag.kind == splat::content::layout::FlagKind::RuleChoice,
                )
            })
            .collect();
        let shown = self.repairs.is_empty()
            && !missing_package(catalogue, document)
            && removal_of(&self.results, active.id).is_none_or(|removed| removed.users.is_empty());
        let heading = if affected.is_empty() {
            crate::content_actions::content_issue_count(errors.len() + flags.len())
        } else {
            let gone = format!(
                "{} need a creature that is gone",
                crate::design::count(affected.len(), "encounter", "encounters")
            );
            if flags.is_empty() {
                gone
            } else {
                format!(
                    "{gone} · {}",
                    crate::design::count(flags.len(), "Campaign.md issue", "Campaign.md issues")
                )
            }
        };
        let missing = closure
            .missing_dependencies
            .iter()
            .map(|issue| {
                let owners = issue
                    .owners
                    .iter()
                    .filter_map(|id| {
                        let path = catalogue.file_path_of(*id)?;
                        Some((
                            format!("Repair {}", catalogue.entries[id].entry.name),
                            path.to_path_buf(),
                        ))
                    })
                    .collect();
                (issue.message.clone(), owners)
            })
            .collect();
        let errors = errors
            .into_iter()
            .filter(|error| {
                !closure
                    .missing_dependencies
                    .iter()
                    .any(|issue| issue.message.as_str() == error.as_str())
            })
            .collect();
        Some(Issues {
            heading,
            flags,
            shown,
            errors,
            missing,
            affected,
        })
    }

    /// The package rows, left.
    pub fn package_rows(&self, campaigns: &Campaigns) -> Vec<PackageRow> {
        let Some(snapshot) = &self.snapshot else {
            return Vec::new();
        };
        let catalogue = &snapshot.catalogue;
        let document = &snapshot.document;
        let closure = &snapshot.closure.1;
        catalogue
            .modules
            .iter()
            .filter(|module| !module.private)
            .map(|module| {
                let existing = document
                    .selections
                    .iter()
                    .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module.id));
                let total = module
                    .entries
                    .iter()
                    .filter(|id| catalogue.selectable(**id))
                    .count();
                let count = existing.map_or(0, |i| match &document.selections[i].selection {
                    Selection::All => total,
                    Selection::Selected { entries } => entries.len(),
                });
                let required = module
                    .entries
                    .iter()
                    .filter(|id| {
                        catalogue.selectable(**id)
                            && closure.required.contains_key(id)
                            && !closure.direct.contains(id)
                    })
                    .count();
                let partial = count + required < total
                    && existing.is_some_and(|i| {
                        matches!(&document.selections[i].selection,
                        Selection::Selected { entries } if !entries.is_empty())
                    });
                let indeterminate = partial || (count == 0 && required > 0);
                let mark = catalogue.manifest_problems.get(&module.id).map(|problem| {
                    let folder = campaigns
                        .modules()
                        .iter()
                        .find(|known| known.id == module.id);
                    manifest_mark(folder.zip(catalogue.entries.get(&module.id)).is_some_and(
                        |(folder, entry)| manifest_on_disk(folder, &entry.entry.path, problem),
                    ))
                });
                let note = (count == 0 && required > 0)
                    .then(|| {
                        let selection = PackageSelection {
                            name: Some(module.name.clone()),
                            package: EntryRef::id(module.id),
                            selection: Selection::All,
                        };
                        let mut users = selection.campaign_users(catalogue);
                        users.sort_by(|a, b| a.1.cmp(&b.1));
                        let names: Vec<&str> =
                            users.iter().map(|(_, name)| name.as_str()).collect();
                        match names.as_slice() {
                            [] => None,
                            [name] => Some(format!("{name} use its creatures.")),
                            [rest @ .., last] => {
                                Some(format!("{} and {last} use its creatures.", rest.join(", ")))
                            }
                        }
                    })
                    .flatten();
                PackageRow {
                    id: module.id,
                    name: module.name.clone(),
                    checked: count > 0,
                    indeterminate,
                    selected: !self.dependencies && self.module == Some(module.id),
                    detail: package_count(count + required, total, indeterminate),
                    mark,
                    note,
                }
            })
            .collect()
    }

    /// A package's box: on chooses it (whole, or as it was before it was turned off); off keeps
    /// its choice to come back.
    pub fn toggle_package(&mut self, campaign: Uuid, module: Uuid, enabled: bool) {
        end_results(&mut self.results, campaign, |payload| {
            matches!(payload, Payload::Saved)
        });
        let Some(snapshot) = self.snapshot.as_mut() else {
            return;
        };
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let Some(name) = catalogue
            .modules
            .iter()
            .find(|m| m.id == module)
            .map(|m| m.name.clone())
        else {
            return;
        };
        let existing = document
            .selections
            .iter()
            .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module));
        if enabled {
            if let Some(i) = existing {
                document.selections.remove(i);
            }
            document.selections.push(PackageSelection {
                name: Some(name),
                package: EntryRef::id(module),
                selection: self
                    .disabled_selections
                    .remove(&module)
                    .unwrap_or(Selection::All),
            });
        } else if let Some(i) = existing {
            let removed = document.selections.remove(i);
            self.disabled_selections.insert(module, removed.selection);
        }
    }

    /// A press on a package row: its entries show on the right.
    pub fn select_package(&mut self, module: Uuid) {
        self.module = Some(module);
        self.dependencies = false;
        self.search.clear();
    }

    /// `Needed by your entries`.
    pub fn select_needed(&mut self) {
        self.dependencies = true;
        self.search.clear();
    }

    /// The right column shows `Needed by your entries`.
    pub fn needed_shown(&self) -> bool {
        self.dependencies
    }

    /// How many entries the campaign's choices need.
    pub fn needed_count(&self) -> usize {
        self.snapshot
            .as_ref()
            .map_or(0, |snapshot| snapshot.closure.1.required.len())
    }

    /// The search of the right column.
    pub fn search_words(&self) -> &str {
        &self.search
    }

    pub fn set_search(&mut self, words: &str) {
        self.search = words.to_owned();
    }

    /// `Needed by your entries`, filtered by the search.
    pub fn needed_rows(&self) -> Vec<NeededRow> {
        let Some(snapshot) = &self.snapshot else {
            return Vec::new();
        };
        let catalogue = &snapshot.catalogue;
        let query = self.search.to_lowercase();
        let mut rows: Vec<NeededRow> = snapshot
            .closure
            .1
            .required
            .iter()
            .filter(|(id, _)| {
                catalogue.entries[id]
                    .entry
                    .name
                    .to_lowercase()
                    .contains(&query)
            })
            .map(|(id, reasons)| {
                let entry = &catalogue.entries[id];
                let owner = if entry.private {
                    "Campaign".to_owned()
                } else {
                    catalogue.package_name(entry.owner)
                };
                NeededRow {
                    id: *id,
                    name: entry.entry.name.clone(),
                    detail: format!(
                        "{} · {owner} · {}",
                        crate::design::reader::human_label(&entry.entry.kind),
                        needed_by(reasons.iter().cloned())
                    ),
                }
            })
            .collect();
        rows.sort_by_key(|row| row.name.to_lowercase());
        rows
    }

    /// The package on the right (the chosen one, else the first).
    pub fn package_view(&mut self) -> Option<PackageView> {
        let snapshot = self.snapshot.as_ref()?;
        let catalogue = &snapshot.catalogue;
        let document = &snapshot.document;
        let closure = &snapshot.closure.1;
        let module = catalogue
            .modules
            .iter()
            .find(|m| Some(m.id) == self.module)
            .or_else(|| catalogue.modules.first())?;
        let selection_index = document
            .selections
            .iter()
            .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module.id));
        let ids: Vec<Uuid> = module
            .entries
            .iter()
            .copied()
            .filter(|id| catalogue.selectable(*id))
            .collect();
        let all =
            selection_index.is_some_and(|i| document.selections[i].selection == Selection::All);
        let view = PackageView {
            id: module.id,
            name: module.name.clone(),
            count: ids.len(),
            all,
            choose_all: if ids
                .iter()
                .all(|id| closure.direct.contains(id) || closure.required.contains_key(id))
            {
                "Include future additions"
            } else {
                "Choose all"
            },
            chosen: selection_index.is_some(),
        };
        self.module = Some(module.id);
        Some(view)
    }

    /// The entry rows of the package on the right, filtered by the search.
    pub fn entry_rows(&mut self) -> Vec<EntryRow> {
        let Some(module) = self.module else {
            return Vec::new();
        };
        let Some(snapshot) = &self.snapshot else {
            return Vec::new();
        };
        let catalogue = &snapshot.catalogue;
        let closure = &snapshot.closure.1;
        let query = self.search.to_lowercase();
        let Some(package) = catalogue.modules.iter().find(|m| m.id == module) else {
            return Vec::new();
        };
        let mut ids: Vec<Uuid> = package
            .entries
            .iter()
            .copied()
            .filter(|id| catalogue.selectable(*id))
            .filter(|id| {
                catalogue.entries[id]
                    .entry
                    .name
                    .to_lowercase()
                    .contains(&query)
            })
            .collect();
        ids.sort_by_cached_key(|id| catalogue.entries[id].entry.name.to_lowercase());
        ids.into_iter()
            .map(|id| {
                let entry = &catalogue.entries[&id].entry;
                let required = closure.required.get(&id);
                EntryRow {
                    id,
                    name: entry.name.clone(),
                    subtitle: snapshot.subtitles.get(&id).cloned().unwrap_or_default(),
                    needed: required.map(|reasons| needed_by(reasons.iter().cloned())),
                    checked: closure.direct.contains(&id) || required.is_some(),
                    locked: required.map(|reasons| {
                        format!(
                            "{} is needed by {}, so it stays selected.",
                            entry.name,
                            reasons.iter().cloned().collect::<Vec<_>>().join(", ")
                        )
                    }),
                }
            })
            .collect()
    }

    /// `Choose all` (or `Include future additions`): the whole package.
    pub fn choose_all(&mut self, module: Uuid) {
        let Some(snapshot) = self.snapshot.as_mut() else {
            return;
        };
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let Some(name) = catalogue
            .modules
            .iter()
            .find(|m| m.id == module)
            .map(|m| m.name.clone())
        else {
            return;
        };
        match document
            .selections
            .iter()
            .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module))
        {
            Some(i) => document.selections[i].selection = Selection::All,
            None => document.selections.push(PackageSelection {
                name: Some(name),
                package: EntryRef::id(module),
                selection: Selection::All,
            }),
        }
    }

    /// `Choose none`.
    pub fn choose_none(&mut self, module: Uuid) {
        let Some(snapshot) = self.snapshot.as_mut() else {
            return;
        };
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        if let Some(i) = document
            .selections
            .iter()
            .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module))
        {
            document.selections.remove(i);
        }
    }

    /// An entry's box in the package on the right.
    pub fn toggle_entry(&mut self, module: Uuid, id: Uuid, checked: bool) {
        let Some(snapshot) = self.snapshot.as_mut() else {
            return;
        };
        let catalogue = &snapshot.catalogue;
        let document = &mut snapshot.document;
        let Some(package) = catalogue.modules.iter().find(|m| m.id == module) else {
            return;
        };
        let entry_ids: Vec<Uuid> = package
            .entries
            .iter()
            .copied()
            .filter(|id| catalogue.selectable(*id))
            .collect();
        let selection_index = document
            .selections
            .iter()
            .position(|s| catalogue.resolve_ref(&s.package).ok() == Some(module));
        let mut selected = match selection_index.map(|i| &document.selections[i].selection) {
            Some(Selection::All) => entry_ids
                .iter()
                .map(|id| EntryRef::id(*id))
                .collect::<Vec<_>>(),
            Some(Selection::Selected { entries }) => entries.clone(),
            None => Vec::new(),
        };
        selected.retain(|r| catalogue.resolve_ref(r).ok() != Some(id));
        if checked {
            selected.push(EntryRef::id(id));
        }
        let empty = selected.is_empty();
        let selection = Selection::Selected { entries: selected };
        if empty {
            if let Some(i) = selection_index {
                document.selections.remove(i);
            }
        } else if let Some(i) = selection_index {
            document.selections[i].selection = selection;
        } else {
            document.selections.push(PackageSelection {
                name: Some(package.name.clone()),
                package: EntryRef::id(module),
                selection,
            });
        }
    }

    /// Open an entry in Lookup (a needed entry, an entry's name).
    pub fn open_in_lookup(&mut self, id: Uuid) {
        self.open_lookup = Some(id);
    }

    /// The repairs waiting, by their files.
    pub fn repair_paths(&self) -> Vec<String> {
        self.repairs
            .iter()
            .map(|repair| repair.path.clone())
            .collect()
    }

    /// The invalid notes in repair, in order, each with its message (`repair_section`);
    /// `open` says whose editor is open. While the editor is open, the message keeps the same
    /// words and the cause moves to the save row (round 25 FD1).
    pub fn repair_rows(&self, open: &dyn Fn(&str) -> bool) -> Vec<RepairRow> {
        self.repairs
            .iter()
            .map(|repair| {
                let open = open(&repair.path);
                let problem = match (&repair.refusal, &repair.failure) {
                    _ if open => None,
                    (Some(refusal), _) => Some(refusal.clone()),
                    (None, Some(_)) => None,
                    (None, None) => Some("The repair has not been saved".into()),
                };
                let words = crate::design::outcome::fault_words(&ContentFault::Unreadable {
                    path: &repair.path,
                    problem: problem.as_deref(),
                })
                .to_string();
                RepairRow {
                    path: repair.path.clone(),
                    words,
                    open,
                    problem,
                    draft: repair.draft.as_str().to_owned(),
                    error_line: repair.error_line,
                }
            })
            .collect()
    }

    /// The repair whose Save is the one primary on the screen: the active repair's, else the
    /// last open one with changes Save would write. None while an open run owns `Next turn`.
    pub fn repair_owner(&self, open: &dyn Fn(&str) -> bool) -> Option<String> {
        if self.run_open {
            return None;
        }
        let changed: Vec<&str> = self
            .repairs
            .iter()
            .filter(|repair| {
                repair.dirty()
                    && repair.refusal.is_none()
                    && repair.failure.is_none()
                    && open(&repair.path)
            })
            .map(|repair| repair.path.as_str())
            .collect();
        self.active_repair
            .as_deref()
            .filter(|path| changed.contains(path))
            .or(changed.last().copied())
            .map(str::to_owned)
    }

    /// The save row's state of the repair at `path`: its word, Discard, the switch reason and
    /// Save's lock and weight (`owner`: the repair that holds the primary).
    pub fn repair_state(
        &self,
        path: &str,
        owner: Option<&str>,
    ) -> Option<crate::design::DraftState<'_>> {
        use crate::design::save_row::{Changed, DraftOrigin, Primary};
        let repair = self.repairs.iter().find(|repair| repair.path == path)?;
        let mut state = crate::design::DraftState::new(
            DraftOrigin::Saved, // a repair edits a file on disk
            if repair.dirty() {
                Changed::Yes
            } else {
                Changed::No
            },
            if owner == Some(path) {
                Primary::Draft
            } else {
                Primary::Elsewhere
            },
        )
        .locks_campaign();
        if let Some(found) = repair.diagnosis() {
            state = state.check(found.into_owned());
        }
        if repair.failure.is_some() {
            state = state.not_saved();
        }
        Some(state)
    }

    /// The repair at `path` became the one the DM works in (an edit, or the editor took the
    /// keyboard).
    pub fn focus_repair(&mut self, path: &str) {
        self.active_repair = Some(path.to_owned());
    }

    /// The repair's draft changed in its editor: Save's check runs again, a failed Save no
    /// longer describes it, and its Discard result ends.
    pub fn repair_edited(&mut self, active: &Campaign, path: &str) {
        self.active_repair = Some(path.to_owned());
        let checks = &mut self.repair_checks;
        if let Some(repair) = self.repairs.iter_mut().find(|repair| repair.path == path) {
            repair.draft_changed(active, checks);
        }
        self.end_repair_discard(active, path);
    }

    /// The repair's editor gave new words (a UI that does not edit the draft in place).
    pub fn edit_repair(&mut self, active: &Campaign, path: &str, words: &str) {
        let Some(repair) = self.repairs.iter_mut().find(|repair| repair.path == path) else {
            return;
        };
        if repair.draft.as_str() == words {
            return;
        }
        let _ = repair.draft.replace(words);
        self.repair_edited(active, path);
    }

    /// While a repair has changes, its Discard result has ended: one discard result per
    /// repair, ended by its next edit or Discard only.
    pub(super) fn end_repair_discard(&mut self, active: &Campaign, path: &str) {
        if self
            .repairs
            .iter()
            .any(|repair| repair.path == path && repair.dirty())
        {
            end_results(&mut self.results, active.id, |payload| {
                matches!(payload, Payload::RepairDiscard { path: p, .. } if p == path)
            });
        }
    }

    /// The repair's `Discard`: its draft goes back to the file's text, with an Undo.
    pub fn discard_repair(&mut self, active: &Campaign, path: &str) {
        end_results(&mut self.results, active.id, |payload| {
            matches!(payload, Payload::RepairDiscard { path: p, .. } if p == path)
        });
        let Some(text) = self
            .repairs
            .iter()
            .find(|repair| repair.path == path)
            .map(|repair| repair.draft.as_str().to_owned())
        else {
            return;
        };
        let stamp = self.repair_stamp;
        self.repair_stamp += 1;
        super::new_result_of(
            self,
            Payload::RepairDiscard {
                stamp,
                path: path.to_owned(),
                text,
            },
        );
        let checks = &mut self.repair_checks;
        if let Some(repair) = self.repairs.iter_mut().find(|repair| repair.path == path) {
            let _ = repair.draft.replace(repair.baseline.as_str());
            repair.draft_changed(active, checks);
        }
    }

    /// The repair's `Save repaired note` (or Ctrl+S): Save asks its own question again, then
    /// writes. A write that failed is the newest answer; a saved repair ends an older failure,
    /// and when no draft is left the page reads the campaign again.
    pub fn save_repair(
        &mut self,
        ctx: &egui::Context,
        campaigns: &Campaigns,
        active: &Campaign,
        path: &str,
    ) {
        let Some(repair) = self.repairs.iter_mut().find(|repair| repair.path == path) else {
            return;
        };
        if repair.save(active) {
            // A repair saved: the last act succeeded, so an older failure is no longer its
            // answer (it stays in its own repair's message).
            self.answer = None;
            if !self.dirty() {
                self.reload(ctx, campaigns, active.clone());
            }
        } else if let Some(failure) = repair.failure.clone() {
            self.fail(
                Cause::NotSaved {
                    path: path.to_owned(),
                },
                failure,
            );
        }
    }
}
