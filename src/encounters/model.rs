//! The Encounters page as plain data, and its acts, for a view that does not draw with egui (the
//! GPUI page, `src/gui/pages/encounters/`). The egui view keeps its own drawing; both views call
//! the acts here and in `Encounters`, so the logic has one home. Nothing here draws or measures:
//! a view fits the words itself.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::*;
use crate::design::outcome::Plain;
use crate::encounter_run::SavedRunInfo;
use std::path::PathBuf;

pub use super::groups::{GroupEdit, GroupView, MemberAct, MemberRow};

/// What the page shows now (`Encounters::show_page`).
pub enum PageShown {
    /// Another campaign's encounter is open: its fault stands in place of the page.
    OtherCampaign(Plain<FooterPress>),
    List,
    Encounter,
    Group,
}

/// A footer line: the message, and which store its Undo reaches. The store's revision when the
/// line was made goes with the press, so a line whose result was replaced or ended since does
/// nothing, as a ticket's press does in egui (`Results::undo_by`).
pub struct Line {
    pub message: Plain<FooterPress>,
    pub press: Option<LinePress>,
}

/// Which kept result a footer line's Undo reaches.
#[derive(Clone, Debug, PartialEq)]
pub enum LinePress {
    Kept(KeptResult, crate::design::memo::Revision),
    Editor(Uuid, DraftResult, crate::design::memo::Revision),
    Member(Uuid, crate::design::memo::Revision),
    Separation(usize, crate::design::memo::Revision),
}

/// The list's header.
pub struct Header {
    pub context: Option<&'static str>,
    pub new_blocked: Option<&'static str>,
    pub group_blocked: Option<&'static str>,
    pub reload_blocked: Option<&'static str>,
}

/// The list's body.
pub enum Body {
    /// The first read of the campaign is in progress.
    Reading,
    /// Nothing is read yet (a failed read: the footer says why).
    Nothing,
    /// No encounter, run or group yet.
    Empty { issues: Vec<String> },
    Ledger(Box<Ledger>),
}

/// In progress, Ready to run, Groups and History.
pub struct Ledger {
    pub issues: Vec<String>,
    pub in_progress: Vec<RunRow>,
    /// After the count: the rows' order (`Arranged::in_progress_order`).
    pub in_progress_order: Option<&'static str>,
    /// Why there is no row: the run folder's fault, or no unfinished run.
    pub in_progress_reason: Option<String>,
    pub ready: Vec<ReadyRow>,
    /// Why every Start waits (an unsaved draft).
    pub ready_blocked: Option<&'static str>,
    pub groups: Vec<GroupSet>,
    pub group_count: usize,
    pub history: History,
}

/// An In progress row: a paused run, or the open one.
pub struct RunRow {
    pub path: PathBuf,
    pub name: String,
    /// Its second line; it starts with the run icon unless the run is open.
    pub detail: String,
    /// The open run's state word (`Running`), with its link back.
    pub open: Option<String>,
    pub edit: Option<String>,
    pub resume_primary: bool,
    pub blocked: Option<&'static str>,
    /// `Start another fight`, with its reason when it waits.
    pub start_another: Option<(String, Option<&'static str>)>,
    /// `Delete the saved run`, with its reason when it waits; none for the open run.
    pub delete: Option<Option<&'static str>>,
}

/// A Ready to run row.
pub struct ReadyRow {
    pub path: String,
    pub name: String,
    pub detail: String,
    /// The difficulty's form after the detail: each part with whether it is the word.
    pub form: Vec<(String, bool)>,
    /// The party's budgets, on hover over the word.
    pub budgets: Option<String>,
    pub start_primary: bool,
    pub start_blocked: Option<&'static str>,
    /// The short reason Start waits, painted after it; `go_to_run` adds the link to the run.
    pub short_reason: Option<&'static str>,
    pub go_to_run: bool,
    pub removable: bool,
}

/// One sub-heading of Groups and its rows.
pub struct GroupSet {
    pub title: String,
    pub rows: Vec<GroupRow>,
}

pub struct GroupRow {
    pub path: String,
    pub name: String,
    pub detail: String,
    pub blocked: Option<&'static str>,
    /// Each member fault: its message and whether its repair is Packages (else Edit group).
    pub faults: Vec<(Plain<()>, bool)>,
    /// The group's own problems (no member can join), in the warning tone.
    pub problems: Vec<String>,
}

/// History.
pub struct History {
    pub count: usize,
    /// Why it is empty or cannot be read; then nothing else shows.
    pub reason: Option<String>,
    pub sessions: Vec<(String, Vec<PastRow>)>,
    pub unreadable: Vec<String>,
}

pub struct PastRow {
    pub path: PathBuf,
    pub name: String,
    pub label: String,
    pub detail: String,
    pub delete_blocked: Option<&'static str>,
    /// Start fight and Edit for its encounter, while it exists.
    pub template: Option<(String, Option<&'static str>)>,
    pub read: Option<ReadRun>,
}

/// The ended run read in History.
pub struct ReadRun {
    pub end_reason: String,
    /// Why it cannot go on, standing where Continue fight would be.
    pub not_last: Option<String>,
    pub continue_blocked: Option<&'static str>,
    pub rows: Vec<FinalRow>,
    pub log: Vec<String>,
}

/// A Final state row: its name, HP, temporary HP, then the rest in their colours.
pub struct FinalRow {
    pub name: String,
    pub hp: String,
    pub temp: Option<String>,
    pub rest: Vec<(String, egui::Color32)>,
}

const NEW_GROUP_HINT: &str = "A reusable set of creatures to add to a running encounter";

impl Encounters {
    /// What `show_page` does before it draws: the campaign check, and which view shows.
    pub fn page_frame(&mut self, campaigns: &Campaigns) -> Option<PageShown> {
        let active = campaigns.active_campaign()?;
        if self
            .active
            .as_ref()
            .is_none_or(|c| c.id != active.id || c.path != active.path)
        {
            let fault = design::outcome::FaultMessage::new(&EncounterFault::PreviousCampaignDraft);
            return Some(PageShown::OtherCampaign(
                design::outcome::Outcome::Fault(fault).plain(),
            ));
        }
        if self.draft_dirty() {
            self.message = None;
        }
        Some(if self.group_draft.is_some() {
            self.group_in_plan = false;
            PageShown::Group
        } else if self.draft.is_some() {
            PageShown::Encounter
        } else {
            PageShown::List
        })
    }

    /// Ctrl+S from a view that takes keys as actions (the GPUI page): the open creature while
    /// it has changes, else the open group or encounter, as `save_by_key` saves a key no pane
    /// took. Only while an editor is open (`save_shortcut`).
    pub fn save_by_key_now(&mut self, ctx: &egui::Context, active: &Campaign, modules: &[Module]) {
        if self.creature.is_some() || self.draft.is_some() || self.group_draft.is_some() {
            self.save_key = true;
            self.save_by_key(ctx, active, modules);
        }
    }

    /// Whether the campaign's encounters are read.
    pub fn loaded(&self) -> bool {
        self.snapshot.is_some()
    }

    /// The open draft's roster, by each row's shown name.
    pub fn roster_names(&self) -> Option<Vec<String>> {
        let draft = self.draft.as_ref()?;
        let no_names = BTreeMap::new();
        let snapshot = self.snapshot.as_ref();
        Some(
            draft
                .setup
                .roster
                .iter()
                .map(|slot| {
                    let definition = snapshot.and_then(|snapshot| {
                        let id = snapshot
                            .catalogue
                            .resolve_ref(slot.creature.as_ref()?)
                            .ok()?;
                        snapshot.catalogue.entries.get(&id).map(|entry| &entry.entry)
                    });
                    roster_name(
                        slot,
                        definition,
                        snapshot.map_or(&no_names, |snapshot| &snapshot.known_creature_names),
                    )
                })
                .collect(),
        )
    }

    /// The saved encounters' names, as the list has them.
    pub fn saved_names(&self) -> Vec<String> {
        self.snapshot.as_ref().map_or_else(Vec::new, |snapshot| {
            snapshot
                .templates
                .iter()
                .map(|template| template.name.clone())
                .collect()
        })
    }

    /// Whether an ended run is read in History.
    pub fn history_open(&self) -> bool {
        self.history_read.is_some()
    }

    /// The list's header: its context and why each action waits.
    pub fn list_header(&self) -> Header {
        let loading = self.pending.is_some();
        let blocked = self.draft_dirty().then_some(BLOCKED_BY_DRAFT);
        let reading = loading.then_some("Still reading this campaign.");
        let reload_blocked = blocked.or(reading).or(self
            .settings_unsaved()
            .then_some("Save or discard the changes in Campaign, Rules and runs first."));
        Header {
            context: self.snapshot.is_none().then_some("Reading encounters…"),
            new_blocked: blocked.or(reading),
            group_blocked: blocked.or(reading),
            reload_blocked,
        }
    }

    /// The hint of New group.
    pub fn new_group_hint() -> &'static str {
        NEW_GROUP_HINT
    }

    /// New group, from the list's header or its empty state.
    pub fn new_group(&mut self) {
        self.search.clear();
        self.group_in_plan = false;
        self.group_draft = Some(GroupDraft::new());
        self.expanded = None;
        self.focus_name = true;
        self.error = None;
        self.message = None;
    }

    /// New encounter, from the list's header or its empty state.
    pub fn new_encounter(&mut self) {
        self.start_new_draft();
    }

    /// Reload from disk: a run stays open through it, so only the content is read again.
    pub fn reload_from_disk(
        &mut self,
        ctx: &egui::Context,
        active: &Campaign,
        campaigns: &Campaigns,
    ) {
        if self.has_run() {
            self.error = None;
            self.refresh_content(ctx, campaigns, &active.path);
        } else {
            self.reload(ctx, active.clone(), campaigns.modules().to_vec());
        }
    }

    /// The list's body (`ledger_body`), as data.
    pub fn ledger_data(&mut self, story: &Story<'_>) -> Body {
        if self.pending.is_some() && self.snapshot.is_none() {
            return Body::Reading;
        }
        let loading = self.pending.is_some();
        let start_blocked = self.start_blocked();
        let group_blocked = self.draft_dirty().then_some(BLOCKED_BY_DRAFT);
        let history_blocked = self.history_blocked;
        let live = self
            .run
            .as_ref()
            .and_then(crate::encounter_run::Run::live_info);
        let Some(snapshot) = &self.snapshot else {
            return Body::Nothing;
        };
        let issues = snapshot.issues.clone();
        if snapshot.templates.is_empty() && snapshot.runs.is_empty() && snapshot.groups.is_empty()
        {
            return Body::Empty { issues };
        }
        let mut page = history::arrange(
            &snapshot.templates,
            &snapshot.runs,
            live.as_ref(),
            story.events,
            story.sessions,
        );
        page.runs_unread = snapshot.runs_unread.as_ref();
        let in_progress = in_progress_rows(
            &page,
            &snapshot.templates,
            live.as_ref(),
            story,
            (start_blocked, loading),
        );
        let in_progress_reason = match page.runs_unread {
            Some(fault) => Some(fault.list_words(false).to_owned()),
            None if page.in_progress.is_empty() => {
                Some("No unfinished runs. Start a fight from Ready to run.".to_owned())
            }
            None => None,
        };
        let ready = ready_rows(&page, live.as_ref(), start_blocked);
        let groups = group_sets(snapshot, group_blocked);
        let read = self.history_read.as_ref().map(|(path, summary)| list::Reading {
            path,
            summary,
            not_last: history_blocked,
        });
        let history = history_view(&page, start_blocked, live.as_ref(), read.as_ref());
        Body::Ledger(Box::new(Ledger {
            issues,
            in_progress_order: page.in_progress_order(),
            in_progress,
            in_progress_reason,
            ready_blocked: start_blocked,
            ready,
            group_count: snapshot.groups.len(),
            groups,
            history,
        }))
    }

    /// The list's footer lines, newest result first, then the reasons (`ledger_view`).
    pub fn list_footer(&mut self) -> Vec<Line> {
        let mut lines = self.kept_plain(true);
        if let Some(error) = &self.error {
            lines.push(Line {
                message: design::outcome::Outcome::Fault(design::outcome::FaultMessage::new(
                    &EncounterFault::Failed(error.clone()),
                ))
                .plain(),
                press: None,
            });
        }
        if let Some(folder) = self
            .snapshot
            .as_ref()
            .and_then(|snapshot| snapshot.runs_unread.clone())
        {
            let choice = design::outcome::Choice::new(
                design::outcome::MessageChoice::Choice("Choose folder"),
                FooterPress::ChooseFolder,
            );
            lines.push(Line {
                message: design::outcome::Outcome::Fault(
                    design::outcome::FaultMessage::new(&EncounterFault::RunFolder(folder))
                        .choice(choice),
                )
                .plain(),
                press: None,
            });
        }
        lines
    }

    /// The kept results this view shows (`kept_results`), as footer lines.
    pub fn kept_plain(&mut self, page: bool) -> Vec<Line> {
        let kinds = self.kept_results(page);
        let Some(root) = self.active.as_ref().map(|active| active.path.clone()) else {
            return Vec::new();
        };
        let revision = self.kept.revision();
        kinds
            .into_iter()
            .filter(|kind| {
                self.kept
                    .get(&KeptOwner {
                        root: root.clone(),
                        kind: *kind,
                    })
                    .is_some()
            })
            .map(|kind| Line {
                message: self.kept_outcome(kind).plain(),
                press: Some(LinePress::Kept(kind, revision)),
            })
            .collect()
    }

    /// A press of a footer line's Undo or choice. A line whose store changed since it was made
    /// does nothing (its result may be another now).
    pub fn footer_press(
        &mut self,
        ctx: &egui::Context,
        press: Option<LinePress>,
        id: FooterPress,
        active: &Campaign,
        campaigns: &Campaigns,
    ) {
        match (id, press) {
            (FooterPress::ChooseFolder, _) => self.wants_run_folder = true,
            (FooterPress::Undo, Some(LinePress::Kept(kind, revision))) => {
                let owner = KeptOwner {
                    root: active.path.clone(),
                    kind,
                };
                let mut kept = std::mem::take(&mut self.kept);
                let _ = kept.undo_seen(revision, &owner, |mark| {
                    if self.undo_kept(mark.0.kind, Some((active, campaigns.modules()))) {
                        Ok(())
                    } else {
                        Err(())
                    }
                });
                self.kept = kept;
            }
            (FooterPress::UndoEditor, Some(LinePress::Editor(draft, kind, revision))) => {
                let owner = EditorOwner {
                    root: active.path.clone(),
                    draft,
                    kind,
                };
                let mut results = std::mem::take(&mut self.editor_results);
                let _ = results.undo_seen(revision, &owner, |mark| match self.draft.as_mut() {
                    Some(open) if open.results_key.id() == mark.owner.draft && mark.undo.holds(open) => {
                        open.undo_result(&mark.undo);
                        Ok(())
                    }
                    _ => Err(()),
                });
                self.editor_results = results;
                self.expanded = None;
            }
            (FooterPress::UndoMember, Some(LinePress::Member(draft, revision))) => {
                let owner = MemberOwner {
                    root: active.path.clone(),
                    draft,
                };
                let mut results = std::mem::take(&mut self.member_results);
                let _ = results.undo_seen(revision, &owner, |mark| {
                    let group = self
                        .group_draft
                        .iter_mut()
                        .chain(
                            self.draft
                                .iter_mut()
                                .flat_map(|draft| draft.groups.iter_mut()),
                        )
                        .find(|group| group.results_key() == mark.owner.draft);
                    group.ok_or(())?.undo_member_result(&mark.undo)
                });
                self.member_results = results;
                self.expanded = None;
            }
            (FooterPress::UndoSeparation, Some(LinePress::Separation(at, revision))) => {
                let Some(owner) = self
                    .separations
                    .newest_first(active.path.as_path())
                    .nth(at)
                    .map(|shown| design::results::Kept::owner(shown.value()))
                else {
                    return;
                };
                match self.separations.undo_seen(revision, &owner, |separation| {
                    separation
                        .change
                        .0
                        .undo(active)
                        .map(|()| separation.path.clone())
                        .map_err(|error| (separation.path.clone(), error))
                }) {
                    Ok((_, path)) => {
                        self.end_separation_failure(&active.path, &path);
                        note_identity_changed(ctx);
                    }
                    Err(design::results::ByTicket::Failed((path, error))) => {
                        self.separation_error = Some(SeparationFailure {
                            root: active.path.clone(),
                            path,
                            words: error,
                        });
                    }
                    Err(design::results::ByTicket::Gone) => {}
                }
            }
            _ => {}
        }
    }
}

fn in_progress_rows(
    page: &history::Arranged<'_>,
    templates: &[Template],
    live: Option<&SavedRunInfo>,
    story: &Story<'_>,
    (blocked, loading): (Option<&'static str>, bool),
) -> Vec<RunRow> {
    page.in_progress
        .iter()
        .enumerate()
        .map(|(index, run)| {
            let open = live.is_some_and(|live| live.id == run.id);
            let template = history::template_of(run, templates);
            RunRow {
                path: run.path.clone(),
                name: run.encounter.clone(),
                detail: list::paused_run_detail(run, &page.in_progress, open, story),
                open: open.then(|| list::run_state_word(run, false)),
                edit: template.map(|template| template.path.clone()),
                resume_primary: index == 0 && live.is_none(),
                blocked,
                start_another: template.map(|template| (template.path.clone(), blocked)),
                delete: (!open).then(|| loading.then_some("Still reading this campaign.")),
            }
        })
        .collect()
}

fn ready_rows(
    page: &history::Arranged<'_>,
    live: Option<&SavedRunInfo>,
    blocked: Option<&'static str>,
) -> Vec<ReadyRow> {
    page.ready
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let template = row.template;
            let (open, short_reason) = list::ready_reason(page, row, live);
            let no_participants = template
                .summary
                .as_ref()
                .is_none_or(|roster| roster.words.is_empty());
            let start_blocked = if row.unfinished_run {
                Some(if open {
                    "This encounter has an open run. Go back to the run."
                } else {
                    "This encounter has an unfinished run. Resume it in In progress."
                })
            } else if no_participants {
                Some("Add participants first.")
            } else {
                blocked
            };
            let detail = list::ready_detail(template).to_owned();
            let line = template
                .summary
                .as_ref()
                .filter(|_| !row.unfinished_run)
                .and_then(|roster| {
                    crate::difficulty::Line::roster(&template.difficulty, &roster.parts)
                        .fit(|_| true)
                });
            let (detail, form) = match line {
                Some(fitted) => (fitted.lead, fitted.form),
                None => (detail, Vec::new()),
            };
            ReadyRow {
                path: template.path.clone(),
                name: template.name.clone(),
                detail,
                form,
                budgets: template
                    .difficulty
                    .rating
                    .as_ref()
                    .map(crate::difficulty::Rating::budget_line),
                start_primary: index == 0 && page.in_progress.is_empty() && live.is_none(),
                start_blocked,
                short_reason,
                go_to_run: open && short_reason.is_some(),
                removable: !list::has_saved_run(page, row),
            }
        })
        .collect()
}

fn group_sets(snapshot: &Snapshot, blocked: Option<&'static str>) -> Vec<GroupSet> {
    let row = |group: &crate::encounter_run::RunGroup| GroupRow {
        path: group.path.clone(),
        name: group.name.clone(),
        detail: groups::row_detail(group),
        blocked,
        faults: group
            .warnings
            .iter()
            .map(|fault| {
                (
                    design::outcome::Outcome::<()>::Fault(design::outcome::FaultMessage::new(
                        fault,
                    ))
                    .plain(),
                    fault.kind == splat_content_ui::roster::FaultKind::NotInPackages,
                )
            })
            .collect(),
        problems: group.own_problems().into_iter().map(str::to_owned).collect(),
    };
    let mut sets = Vec::new();
    let any: Vec<_> = snapshot
        .groups
        .iter()
        .filter(|group| group.encounter.is_none())
        .map(|group| row(group))
        .collect();
    if !any.is_empty() {
        sets.push(GroupSet {
            title: "For any encounter".to_owned(),
            rows: any,
        });
    }
    for (id, name) in snapshot.encounter_choices() {
        let own: Vec<_> = snapshot
            .groups
            .iter()
            .filter(|group| group.encounter == Some(id))
            .map(|group| row(group))
            .collect();
        if !own.is_empty() {
            sets.push(GroupSet {
                title: format!("For {name}"),
                rows: own,
            });
        }
    }
    let orphans: Vec<_> = snapshot
        .groups
        .iter()
        .filter(|group| group.encounter.is_some() && group.encounter_name.is_none())
        .map(|group| row(group))
        .collect();
    if !orphans.is_empty() {
        sets.push(GroupSet {
            title: groups::ENCOUNTER_DELETED.to_owned(),
            rows: orphans,
        });
    }
    sets
}

fn history_view(
    page: &history::Arranged<'_>,
    blocked: Option<&'static str>,
    live: Option<&SavedRunInfo>,
    read: Option<&list::Reading<'_>>,
) -> History {
    if let Some(fault) = page.runs_unread {
        return History {
            count: 0,
            reason: Some(fault.list_words(true).to_owned()),
            sessions: Vec::new(),
            unreadable: Vec::new(),
        };
    }
    if page.ended_count() == 0 && page.unreadable.is_empty() {
        return History {
            count: 0,
            reason: Some("No ended fights yet.".to_owned()),
            sessions: Vec::new(),
            unreadable: Vec::new(),
        };
    }
    let sessions = page
        .history
        .iter()
        .map(|session| {
            let runs = session
                .runs
                .iter()
                .map(|past| {
                    let run = past.run;
                    let here = live.is_some_and(|live| live.id == run.id);
                    let read = read.filter(|read| read.path == run.path).map(|read| {
                        let not_last = read.not_last.map(|reason| {
                            crate::encounter_run::continue_reason(reason, past.template.is_some())
                                .to_owned()
                        });
                        read_run(read.summary, blocked.filter(|_| !here), not_last)
                    });
                    PastRow {
                        path: run.path.clone(),
                        name: run.encounter.clone(),
                        label: run.label.clone(),
                        detail: list::history_detail(past),
                        delete_blocked: here.then_some("Close this run before deleting its save."),
                        template: past.template.map(|template| {
                            let start = blocked.or(template
                                .summary
                                .as_ref()
                                .is_none_or(|roster| roster.words.is_empty())
                                .then_some("Add participants first."));
                            (template.path.clone(), start)
                        }),
                        read,
                    }
                })
                .collect();
            (session.label.to_owned(), runs)
        })
        .collect();
    History {
        count: page.ended_count(),
        reason: None,
        sessions,
        unreadable: page.unreadable.iter().map(|run| run.label.clone()).collect(),
    }
}

fn read_run(
    summary: &crate::encounter_run::FightSummary,
    blocked: Option<&'static str>,
    not_last: Option<String>,
) -> ReadRun {
    let muted = crate::design::colors().muted;
    let rows = summary
        .rows
        .iter()
        .map(|row| {
            let hp = row.hp.map_or_else(|| "?".to_owned(), |hp| hp.to_string());
            let max = row
                .max_hp
                .map_or_else(|| "?".to_owned(), |hp| hp.to_string());
            let conditions = row
                .conditions
                .iter()
                .map(|c| c.words())
                .collect::<Vec<_>>()
                .join(", ");
            let rest = [
                design::hp_word(row.hp, row.max_hp)
                    .map(|(icon, word, color)| (format!("{icon}\u{a0}{word}"), color)),
                (!row.present).then(|| ("Absent".to_owned(), muted)),
                row.concentration
                    .then(crate::encounter_run::view::concentration_mark),
                (!conditions.is_empty()).then_some((conditions, muted)),
            ]
            .into_iter()
            .flatten()
            .collect();
            FinalRow {
                name: row.name.clone(),
                hp: format!("HP {}", crate::combat_sync::fraction(hp, max)),
                temp: (row.temporary_hp > 0)
                    .then(|| format!("{} +{}", icons::TEMP_HP, row.temporary_hp)),
                rest,
            }
        })
        .collect();
    ReadRun {
        end_reason: summary.end_reason.clone(),
        not_last,
        continue_blocked: blocked,
        rows,
        log: summary.log.clone(),
    }
}

// ------------------------------------------------------------------------------------------------
// The encounter editor

/// A press in an editor's head: its back link, Discard, Save, or the run's start.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadAct {
    Back,
    Discard,
    Save,
    /// Start fight (or Resume an unfinished run), saving first when the draft has changes.
    Start { save: bool },
    /// `Start another fight` beside an unfinished run.
    StartAnother { save: bool },
}

/// The editor's back line: the link's words, its accessible name, and the draft that holds it
/// (its long and short reasons).
pub struct BackLine {
    pub word: &'static str,
    pub name: &'static str,
    pub held: Option<(&'static str, &'static str)>,
}

/// The encounter editor, as data.
pub struct EditorView {
    pub draft: Uuid,
    pub name: String,
    pub back: BackLine,
    pub save_primary: bool,
    pub save_blocked: Option<String>,
    pub start_word: &'static str,
    pub start_primary: bool,
    pub start_blocked: Option<String>,
    pub start_hint: &'static str,
    /// `Start another fight` in a menu beside an unfinished run, with its reason.
    pub start_another: Option<Option<String>>,
    /// An open run holds Start: its reason shows before Save.
    pub run_open: bool,
    /// The save row: the state's icon, word and colour; Discard (shown, and why it waits).
    pub state: (String, String, egui::Color32),
    pub discard: Option<Option<&'static str>>,
    /// The campaign-switch reason and the row's other reason.
    pub lock_reason: Option<&'static str>,
    pub other_reason: Option<&'static str>,
    /// The unfinished run's state, at the row's end.
    pub run_state: Option<String>,
    /// What a Save does to an unfinished run.
    pub unfinished_note: Option<&'static str>,
    /// The campaign is still being read: the panes wait.
    pub reading: bool,
}

/// The roster column, as data.
pub struct RosterView {
    pub count: usize,
    /// The difficulty after the heading: its form's parts (each with whether it is the word),
    /// and the party's budgets on hover.
    pub difficulty: Vec<(String, bool)>,
    pub budgets: Option<String>,
    pub rows: Vec<RosterRow>,
    /// Unread roster rows after the last row.
    pub faults_after: Vec<String>,
    /// The sync option, when the campaign is linked to D&D Beyond.
    pub sync: Option<SyncView>,
    pub repeatable: bool,
    pub loot: LootView,
    pub notes: String,
}

pub struct RosterRow {
    pub key: String,
    pub index: usize,
    pub name: String,
    /// The row's unread roster rows above it (`Line N: …`).
    pub faults_before: Vec<String>,
    /// The second line's parts: words, colour, and whether it is the Bloodied state.
    pub parts: Vec<(String, egui::Color32, bool)>,
    /// A missing or unreadable creature: the fault's icon, colour and the duplicate's hover.
    pub missing: Option<(&'static str, egui::Color32, Option<String>)>,
    /// The change link's words (`Changed · …`), while the row is closed.
    pub changed: Option<String>,
    pub open: bool,
    /// `Open in World` or `Open in Library` for an unreadable note, with its path.
    pub repair: Option<(&'static str, PathBuf)>,
    /// `Edit`, or `Replace…` for a missing creature.
    pub edit_word: &'static str,
    pub make_yours: bool,
    pub first: bool,
    pub last: bool,
    pub form: Option<RowForm>,
}

/// An open row's fields.
pub struct RowForm {
    pub side: String,
    pub sides: Vec<String>,
    pub group: String,
    /// `Starting HP` and `Temp HP`: on with a value, or off; none while D&D Beyond decides them.
    pub values: Vec<(&'static str, &'static str, Option<i64>)>,
    pub help: &'static str,
}

pub struct SyncView {
    pub on: bool,
    pub line: Option<String>,
    pub connect: bool,
    pub help: &'static str,
}

pub struct LootView {
    /// The note's Loot section holds more than a list: the field does not own it.
    pub owned: bool,
    pub lines: Vec<String>,
}

/// Add participants, as data.
pub struct AddView {
    pub party: PartyButton,
    pub adding_to: Option<String>,
    pub search: String,
    pub filter: Filter,
    pub results: Arc<Vec<ResultRow>>,
    /// No result matches as typed: the close names say so.
    pub close_names: bool,
}

/// The party's line above the search: `Add party`, or `Party in roster`.
pub enum PartyButton {
    Add { blocked: Option<&'static str> },
    InRoster,
    /// A group: party members go on an encounter's roster.
    None,
}

/// A search result.
#[derive(Clone)]
pub struct ResultRow {
    pub id: Uuid,
    pub party: bool,
    pub name: String,
    pub detail: String,
    pub hp_missing: bool,
    pub source: Option<String>,
    pub in_roster: bool,
    pub unavailable: Option<String>,
}

/// A change to the roster from a row's menu.
#[derive(Clone, Copy, Debug)]
pub enum RosterAct {
    Remove(usize),
    Duplicate(usize),
    Up(usize),
    Down(usize),
}

impl Draft {
    /// A roster change from a row's menu (`Encounters::roster`).
    fn roster_act(&mut self, act: RosterAct) -> bool {
        let roster = &mut self.setup.roster;
        match act {
            RosterAct::Remove(i) if i < roster.len() => {
                roster.remove(i);
                false
            }
            RosterAct::Duplicate(i) if i < roster.len() => {
                let mut slot = roster[i].clone();
                slot.key = format!("actor_{}", Uuid::new_v4().simple());
                roster.insert(i + 1, slot);
                true
            }
            RosterAct::Up(i) if i > 0 && i < roster.len() => {
                roster.swap(i, i - 1);
                false
            }
            RosterAct::Down(i) if i + 1 < roster.len() => {
                roster.swap(i, i + 1);
                false
            }
            _ => false,
        }
    }

    /// Every side a row names is one of the encounter's sides.
    fn keep_sides(&mut self) {
        let named: Vec<String> = self
            .setup
            .roster
            .iter()
            .flat_map(|slot| {
                [
                    Some(slot.side.as_str()),
                    slot.overrides.get("side").and_then(Value::as_str),
                ]
                .into_iter()
                .flatten()
                .map(str::to_owned)
            })
            .collect();
        for side in named {
            if !side.trim().is_empty() && !self.setup.sides.contains_key(side.as_str()) {
                self.setup
                    .sides
                    .insert(side.into(), yaml("{hostile_to: []}"));
            }
        }
    }
}

impl Encounters {
    /// The open encounter's unfinished run, if it has one.
    fn unfinished_run(&self) -> Option<SavedRunInfo> {
        let draft = self.draft.as_ref()?;
        self.snapshot
            .as_ref()?
            .runs
            .iter()
            .find(|run| {
                run.standing == crate::encounter_run::Standing::Unfinished
                    && (run.source == Some(draft.id)
                        || (run.source.is_none() && run.encounter == draft.saved_name))
            })
            .cloned()
    }

    /// A press in the encounter editor's head (`editor_view`). True when the editor closed.
    pub fn editor_head_act(
        &mut self,
        ctx: &egui::Context,
        active: &Campaign,
        modules: &[Module],
        act: HeadAct,
    ) -> bool {
        match act {
            HeadAct::Back => {
                self.clear_notice();
                self.draft = None;
                self.message = None;
                self.error = None;
                self.creature = None;
                self.wants_session = std::mem::take(&mut self.from_session);
                self.plan_on_save = false;
                true
            }
            HeadAct::Discard => {
                self.discard_draft(active);
                true
            }
            HeadAct::Save => {
                self.save_draft(ctx, active, modules);
                false
            }
            HeadAct::Start { save } | HeadAct::StartAnother { save } => {
                let unfinished = self.unfinished_run();
                if save {
                    self.save_draft(ctx, active, modules);
                }
                if self.draft_dirty() {
                    return false;
                }
                match (act, &unfinished) {
                    (HeadAct::Start { .. }, Some(run)) => self.open_saved_run(&run.path, active),
                    _ => self.open_run(active, modules),
                }
                // A fight started from the session goes back there, not to this editor.
                if self.from_session.is_some() && self.run.is_some() {
                    self.draft = None;
                    self.from_session = None;
                    self.run_from_session = true;
                    return true;
                }
                false
            }
        }
    }

    /// The encounter editor's head and save row (`editor_view`), as data, with the frame's
    /// checks that the egui editor makes before it draws: Save's refusal for Ctrl+S, the
    /// encounter's groups read once, and the last refusal checked again.
    pub fn editor_frame(&mut self, active: &Campaign) -> Option<EditorView> {
        // After a load in progress, so that a group just saved is among them.
        if self.pending.is_none()
            && let (Some(snapshot), Some(draft)) = (&self.snapshot, self.draft.as_mut())
            && !draft.groups_read
            && let Err(e) = draft.read_groups(&active.path, snapshot)
        {
            self.error = Some(e);
        }
        // A frame here follows an input (GPUI draws on a change), so the edit epoch moves.
        self.refresh_validation_error(false, true);
        let no_names = BTreeMap::new();
        let names = self
            .snapshot
            .as_ref()
            .map(|snapshot| &snapshot.known_creature_names)
            .unwrap_or(&no_names);
        let dirty = self.encounter_dirty();
        let creature_dirty = self.creature_dirty();
        let has_run = self.has_run();
        let refusal = self
            .shows_validation_error()
            .then(|| self.error.clone())
            .flatten();
        let loading = self.pending.is_some();
        let unfinished = self.unfinished_run();
        let draft = self.draft.as_ref()?;
        let roster_reason = Entry::parse(&draft.path, &draft.source)
            .ok()
            .and_then(|entry| roster_fault_reason(&draft.path, &roster_faults(&entry, names)));
        let empty_group = draft.empty_group_reason();
        let roster_empty = draft.setup.roster.is_empty();
        let draft_origin = draft.origin;
        let (draft_id, name) = (draft.id, draft.name.clone());
        let groups = self.owned_groups_diagnosis();
        let own = refusal
            .is_some()
            .then(|| self.validation_error.as_ref().map(Invalid::diagnosis))
            .flatten();
        let missing_origins = [
            groups.origin,
            own.as_ref()
                .filter(|own| own.only_incomplete())
                .map(|_| draft_origin),
        ];
        let origin = if missing_origins.contains(&Some(design::save_row::DraftOrigin::Saved)) {
            design::save_row::DraftOrigin::Saved
        } else if missing_origins.contains(&Some(design::save_row::DraftOrigin::New)) {
            design::save_row::DraftOrigin::New
        } else {
            draft_origin
        };
        let found = groups
            .found
            .clone()
            .and(
                roster_reason
                    .clone()
                    .map_or_else(design::save_row::Diagnosis::valid, unread_rows_found),
            )
            .and(own.unwrap_or_else(design::save_row::Diagnosis::valid));
        let changed = if dirty {
            design::memo::Changed::Yes
        } else {
            design::memo::Changed::No
        };
        let primary = if has_run {
            design::save_row::Primary::Elsewhere
        } else {
            design::save_row::Primary::Draft
        };
        let mut state = design::DraftState::new(origin, changed, primary)
            .locks_campaign()
            .check_when_changed(found);
        if self.message.is_some() {
            state = state.just_saved();
        }
        self.save_refused = dirty && !state.can_save();
        let head = design::EditorHead::new(
            design::EditorKind::Encounter {
                from_session: self.from_session.is_some(),
            },
            &*self,
        );
        let (word, back_name) = head.back_words();
        let start_blocked = roster_reason
            .as_deref()
            .or(empty_group.as_deref())
            .or(has_run
                .then_some(BLOCKED_BY_RUN)
                .or(creature_dirty.then_some(BLOCKED_BY_CREATURE))
                .or(loading.then_some(STILL_READING))
                .or(roster_empty.then_some(ADD_PARTICIPANTS_FIRST)));
        let start_blocked: Option<String> = start_blocked.map(str::to_owned);
        let status = state.status();
        let run_state = unfinished.as_ref().map(|run| {
            list::run_state_word(
                run,
                self.run
                    .as_ref()
                    .and_then(|open| open.path())
                    .is_none_or(|path| path != run.path),
            )
        });
        let follows = self
            .run
            .as_ref()
            .and_then(crate::encounter_run::Run::source)
            .is_some_and(|source| source == draft_id);
        let creature_here = creature_dirty && head.held_by() != Some(design::Unsaved::Creature);
        Some(EditorView {
            draft: draft_id,
            name,
            back: BackLine {
                word,
                name: back_name,
                held: head
                    .held_by()
                    .map(|draft| (draft.reason(), draft.short_reason())),
            },
            save_primary: matches!(
                state.save_weight(),
                design::save_row::SaveWeight::Primary
            ),
            save_blocked: state.save_blocked().map(str::to_owned),
            start_word: if unfinished.is_some() {
                "Resume"
            } else {
                START_FIGHT
            },
            start_primary: !(has_run || dirty),
            start_blocked: if unfinished.is_some() {
                has_run
                    .then_some(BLOCKED_BY_RUN)
                    .or(creature_dirty.then_some(BLOCKED_BY_CREATURE))
                    .or(loading.then_some(STILL_READING))
                    .map(str::to_owned)
            } else {
                start_blocked.clone()
            },
            start_hint: if unfinished.is_some() {
                "Save this encounter and resume its unfinished run."
            } else {
                "Save this encounter and start a run."
            },
            start_another: unfinished.as_ref().map(|_| start_blocked),
            run_open: has_run,
            state: (status.icon.to_owned(), status.word.to_owned(), status.color),
            discard: state
                .discard_shown()
                .then(|| creature_dirty.then_some(BLOCKED_BY_CREATURE)),
            lock_reason: state.lock_reason(),
            other_reason: creature_here
                .then_some(BLOCKED_BY_CREATURE)
                .or(loading.then_some(STILL_READING))
                .or((!has_run && unfinished.is_none() && roster_empty)
                    .then_some(ADD_PARTICIPANTS_FIRST)),
            run_state,
            unfinished_note: unfinished.as_ref().map(|_| {
                if follows {
                    "Save gives this fight’s Prep the new read-aloud text and notes. Other changes apply to the next fight. To add to this fight, use Add to fight in the run."
                } else {
                    "Changes apply to the next fight. To add to this fight, use Add to fight in the run."
                }
            }),
            reading: self.snapshot.is_none(),
        })
    }

    /// The encounter editor's footer (`editor_lines`), as lines.
    pub fn editor_footer(&mut self, ctx: &egui::Context, root: &std::path::Path) -> Vec<Line> {
        use design::outcome::{FaultMessage, Outcome};
        self.record_member_results();
        self.record_editor_results();
        let mut lines = Vec::new();
        if let (Some(active), Some(draft)) = (self.active.as_ref(), self.draft.as_ref()) {
            let revision = self.editor_results.revision();
            for kind in [DraftResult::RemovedGroup, DraftResult::Replacement] {
                let owner = EditorOwner {
                    root: active.path.clone(),
                    draft: draft.results_key.id(),
                    kind,
                };
                if let Some(shown) = self.editor_results.get(&owner) {
                    lines.push(Line {
                        message: shown.value().undo.outcome().plain(),
                        press: Some(LinePress::Editor(owner.draft, kind, revision)),
                    });
                }
            }
        }
        lines.extend(self.kept_plain(false));
        lines.extend(self.separation_plain(root));
        let members: Vec<Uuid> = self
            .draft
            .iter()
            .flat_map(|draft| draft.groups.iter())
            .map(GroupDraft::results_key)
            .collect();
        lines.extend(self.member_plain(root, members));
        if let Some(error) = self.separation_failure() {
            lines.push(fault_line(&EncounterFault::SeparationFailed(error)));
        }
        let dirty = self.encounter_dirty();
        let groups = self.owned_groups_diagnosis();
        let groups_reason = groups.footer.zip(groups.cause).filter(|_| dirty);
        let (groups_reason, groups_cause) = groups_reason.unzip();
        if let Some(error) = self.error.clone()
            && groups_cause.as_deref() != Some(error.as_str())
        {
            let start = self.start_refused.as_ref().filter(|(draft, answers, _)| {
                *answers == error
                    && self.shows_validation_error()
                    && self
                        .draft
                        .as_ref()
                        .is_some_and(|open| open.id == *draft && !open.dirty())
            });
            let fault = if let Some((_, _, refusal)) = start {
                EncounterFault::StartRefused(refusal.clone())
            } else if self.error_tone() == Tone::Warning {
                EncounterFault::Invalid(error)
            } else {
                EncounterFault::Failed(error)
            };
            lines.push(fault_line(&fault));
        }
        if let Some(reason) = groups_reason {
            lines.push(fault_line(&reason));
        }
        if let Some(unread) = self.unread_rows(ctx) {
            for fault in unread.faults() {
                lines.push(Line {
                    message: Outcome::Fault(fault.file_in(root)).plain(),
                    press: None,
                });
            }
        }
        let _ = FaultMessage::<FooterPress>::new;
        lines
    }

    /// The separations of this campaign, newest first, as footer lines.
    fn separation_plain(&self, root: &std::path::Path) -> Vec<Line> {
        let revision = self.separations.revision();
        self.separations
            .newest_first(root)
            .enumerate()
            .map(|(at, shown)| {
                let separation = shown.value();
                Line {
                    message: design::outcome::KeptOutcome::from(
                        design::outcome::Said::new(outcome_words(separation.change.0.message()))
                            .undo(
                                separation
                                    .undo_step()
                                    .pressed_as(FooterPress::UndoSeparation),
                            ),
                    )
                    .plain(),
                    press: Some(LinePress::Separation(at, revision)),
                }
            })
            .collect()
    }

    /// The separations of this campaign, as footer lines (for the group editor).
    pub(super) fn separations_plain(&self, root: &std::path::Path) -> Vec<Line> {
        self.separation_plain(root)
    }

    /// One group draft's member result, as footer lines.
    pub(super) fn member_plain_of(&self, root: &std::path::Path, draft: Uuid) -> Vec<Line> {
        self.member_plain(root, vec![draft])
    }

    /// Each group draft's member result, as footer lines.
    fn member_plain(&self, root: &std::path::Path, drafts: Vec<Uuid>) -> Vec<Line> {
        let revision = self.member_results.revision();
        drafts
            .into_iter()
            .filter_map(|draft| {
                let owner = MemberOwner {
                    root: root.to_owned(),
                    draft,
                };
                self.member_results.get(&owner).map(|shown| Line {
                    message: shown.value().outcome().plain(),
                    press: Some(LinePress::Member(draft, revision)),
                })
            })
            .collect()
    }

    /// The roster column (`roster_column`), as data.
    pub fn roster_view(&mut self, ctx: &egui::Context, party: &Party, linked: bool) -> Option<RosterView> {
        let difficulty = self.draft_difficulty();
        let snapshot = self.snapshot.as_ref()?;
        let draft = self.draft.as_ref()?;
        let names = &snapshot.known_creature_names;
        let faults = Entry::parse(&draft.path, &draft.source)
            .map(|entry| roster_faults(&entry, names))
            .unwrap_or_default();
        let live = linked.then_some(party).filter(|_| draft.setup.combat_sync);
        let sides: Vec<String> = draft
            .setup
            .sides
            .keys()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
        let count = draft.setup.roster.len();
        let rows = draft
            .setup
            .roster
            .iter()
            .enumerate()
            .map(|(i, slot)| {
                roster_row(
                    ctx,
                    (i, count),
                    slot,
                    snapshot,
                    live,
                    self.expanded.as_deref() == Some(slot.key.as_str()),
                    &sides,
                    faults.iter().filter(|fault| fault.before == i).map(RosterFault::words).collect(),
                )
            })
            .collect();
        let (form, budgets) = match difficulty.as_deref() {
            Some(difficulty) => (
                crate::difficulty::Line::roster(difficulty, crate::difficulty::Summary::default())
                    .fit(|_| true)
                    .map(|fitted| fitted.form)
                    .unwrap_or_default(),
                difficulty
                    .rating
                    .as_ref()
                    .map(crate::difficulty::Rating::budget_line)
                    .or_else(|| difficulty.missing.as_ref().and_then(|missing| missing.words())),
            ),
            None => (Vec::new(), None),
        };
        let sync = linked.then(|| {
            let on = draft.setup.combat_sync;
            let line = on
                .then(|| {
                    let sheets: Vec<_> = draft
                        .setup
                        .roster
                        .iter()
                        .filter_map(|slot| slot_sheet(slot, snapshot, party))
                        .collect();
                    crate::party::party_line(&sheets, party.connected())
                })
                .flatten()
                .or_else(|| (!on).then(|| "Nothing in this encounter reaches D&D Beyond.".to_owned()));
            SyncView {
                on,
                line,
                connect: on && !party.connected(),
                help: if on {
                    "The party starts with the HP and conditions on their sheets. When connected, the run and D&D Beyond stay matched."
                } else {
                    "The party starts from their saved copies and the starting values here. To sync a fight, select Sync party in the run."
                },
            }
        });
        Some(RosterView {
            count,
            difficulty: form,
            budgets,
            rows,
            faults_after: faults
                .iter()
                .filter(|fault| fault.before >= count)
                .map(RosterFault::words)
                .collect(),
            sync,
            repeatable: draft.setup.repeatable,
            loot: LootView {
                owned: draft.loot_owned,
                lines: draft.loot.clone(),
            },
            notes: draft.body.to_string(),
        })
    }

    /// Change the open encounter draft (its name, notes, options, a row's fields), then keep
    /// its sides whole.
    pub fn edit_draft(&mut self, edit: impl FnOnce(&mut DraftEdit<'_>)) {
        if let Some(draft) = self.draft.as_mut() {
            edit(&mut DraftEdit(draft));
            draft.keep_sides();
        }
    }

    /// A roster change from a row's menu.
    pub fn roster_act(&mut self, act: RosterAct) {
        if let Some(draft) = self.draft.as_mut() {
            if draft.roster_act(act) {
                self.focus_search = true;
            }
            draft.keep_sides();
        }
    }

    /// A row's Edit: its fields open, or close when open.
    pub fn toggle_row(&mut self, key: &str) {
        self.expanded = if self.expanded.as_deref() == Some(key) {
            None
        } else {
            Some(key.to_owned())
        };
    }

    /// A row's `Make it yours`: a copy of its creature in this campaign.
    pub fn make_yours(&mut self, slot: usize, active: &Campaign, modules: &[Module]) {
        self.make_it_yours(slot, active, modules);
    }

    /// Add participants (`add_pane`), as data. The ranking is kept until its key changes.
    pub fn add_view(&mut self, filter: Filter) -> Option<AddView> {
        if self.draft.is_none() && self.group_draft.is_some() {
            return self.group_add_view(filter);
        }
        let snapshot = self.snapshot.as_ref()?;
        let draft = self.draft.as_mut()?;
        let on_roster = |draft: &Draft, id: Uuid| {
            draft
                .setup
                .roster
                .iter()
                .any(|slot| slot.creature.as_ref().is_some_and(|c| c.id == Some(id)))
        };
        let any_party = snapshot.creatures.iter().any(|c| c.party);
        let missing = snapshot
            .creatures
            .iter()
            .any(|c| c.party && !on_roster(draft, c.id));
        let party = if !missing && any_party {
            PartyButton::InRoster
        } else {
            PartyButton::Add {
                blocked: (!any_party)
                    .then_some("No saved party copies yet. Connect to D&D Beyond in Settings."),
            }
        };
        let adding_to = (!draft.groups.is_empty()).then(|| {
            draft.target_group().map_or_else(
                || "the roster".to_owned(),
                |group| group.name(),
            )
        });
        let group = draft.target_group().is_some();
        let list = snapshot.search_list(group);
        let query = self.search.to_lowercase();
        let key = SearchKey {
            list: list.revision,
            unlinked: list.unlinked,
            query,
            filter: filter.clone(),
        };
        let draft = self.draft.as_ref()?;
        let roster = OnRoster::Slots(&draft.setup.roster);
        let ranked = match self
            .search_ranked
            .as_ref()
            .filter(|kept| kept.0.key == key && (group || roster.names(&kept.0.on_roster)))
        {
            Some(kept) => kept.clone(),
            None => {
                let already = |c: &Creature| !group && snapshot.already_on_roster(draft, c);
                let on_roster = if group { Vec::new() } else { roster.to_vec() };
                let ranked = SearchRanked::rank(key, list, &filter, &already, on_roster);
                let rows = ranked
                    .found
                    .iter()
                    .map(|&at| {
                        let c = &list.creatures[at];
                        ResultRow {
                            id: c.id,
                            party: c.party,
                            name: c.name.clone(),
                            detail: c.detail.clone(),
                            hp_missing: c.hp_missing,
                            source: c.source.clone(),
                            in_roster: already(c),
                            unavailable: c.unavailable.clone(),
                        }
                    })
                    .collect();
                let kept = (Arc::new(ranked), Arc::new(rows));
                self.search_ranked = Some(kept.clone());
                kept
            }
        };
        Some(AddView {
            party,
            adding_to,
            search: self.search.clone(),
            filter,
            close_names: ranked.0.as_typed == 0 && !ranked.0.found.is_empty(),
            results: ranked.1,
        })
    }

    /// Add creatures in the group editor: every creature no NPC links, none on a roster.
    fn group_add_view(&mut self, filter: Filter) -> Option<AddView> {
        let snapshot = self.snapshot.as_ref()?;
        let list = snapshot.search_list(true);
        let key = SearchKey {
            list: list.revision,
            unlinked: list.unlinked,
            query: self.search.to_lowercase(),
            filter: filter.clone(),
        };
        let ranked = match self.search_ranked.as_ref().filter(|kept| kept.0.key == key) {
            Some(kept) => kept.clone(),
            None => {
                let ranked = SearchRanked::rank(key, list, &filter, &|_| false, Vec::new());
                let rows = ranked
                    .found
                    .iter()
                    .map(|&at| {
                        let c = &list.creatures[at];
                        ResultRow {
                            id: c.id,
                            party: c.party,
                            name: c.name.clone(),
                            detail: c.detail.clone(),
                            hp_missing: c.hp_missing,
                            source: c.source.clone(),
                            in_roster: false,
                            unavailable: c.unavailable.clone(),
                        }
                    })
                    .collect();
                let kept = (Arc::new(ranked), Arc::new(rows));
                self.search_ranked = Some(kept.clone());
                kept
            }
        };
        Some(AddView {
            party: PartyButton::None,
            adding_to: None,
            search: self.search.clone(),
            filter,
            close_names: ranked.0.as_typed == 0 && !ranked.0.found.is_empty(),
            results: ranked.1,
        })
    }

    /// The search's words.
    pub fn set_search(&mut self, words: &str) {
        self.search = words.to_owned();
    }

    /// Whether the search asked for the keyboard (after an add); asking ends it.
    pub fn take_focus_search(&mut self) -> bool {
        std::mem::take(&mut self.focus_search)
    }

    /// Enter in the name field: the search takes the keyboard next.
    pub fn ask_focus_search(&mut self) {
        self.focus_search = true;
    }

    /// Whether the name field asked for the keyboard (a new draft); asking ends it.
    pub fn take_focus_name(&mut self) -> bool {
        std::mem::take(&mut self.focus_name)
    }

    /// `Add party`: every party member not on the roster yet.
    pub fn add_party(&mut self) {
        let Some(snapshot) = self.snapshot.as_ref() else {
            return;
        };
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let missing: Vec<Uuid> = snapshot
            .creatures
            .iter()
            .filter(|c| {
                c.party
                    && !draft
                        .setup
                        .roster
                        .iter()
                        .any(|slot| slot.creature.as_ref().is_some_and(|r| r.id == Some(c.id)))
            })
            .map(|c| c.id)
            .collect();
        for id in missing {
            draft.add(id, true);
        }
        self.focus_search = true;
    }

    /// A result's `Add`, or Enter on the best match: to the chosen group, or the roster.
    pub fn add_result(&mut self, id: Uuid, party: bool) {
        let Some(snapshot) = self.snapshot.as_ref() else {
            return;
        };
        if let Some(draft) = self.draft.as_mut() {
            draft.add_selected(id, party, snapshot);
            draft.keep_sides();
        } else if let Some(group) = self.group_draft.as_mut() {
            match snapshot.npc_stat_block(id) {
                Some((stat, name)) => group.add_named(stat, name),
                None => group.add(id),
            }
        }
        self.focus_search = true;
    }

    /// Enter in the search: the best match as typed, when it can be added (`creature_search`).
    pub fn add_best_match(&mut self, results: &[ResultRow], close_names: bool) {
        if self.search.trim().is_empty() || close_names {
            return;
        }
        if let Some(first) = results.first()
            && !first.in_roster
            && first.unavailable.is_none()
        {
            self.add_result(first.id, first.party);
        }
    }
}

/// The edits a view makes to the open encounter draft, by name.
pub struct DraftEdit<'a>(&'a mut Draft);

impl DraftEdit<'_> {
    pub fn name(&mut self, words: &str) {
        self.0.name = words.to_owned();
    }
    pub fn notes(&mut self, words: &str) {
        self.0.body = words.to_owned().into();
    }
    pub fn toggle_sync(&mut self) {
        self.0.setup.combat_sync ^= true;
    }
    pub fn toggle_repeatable(&mut self) {
        self.0.setup.repeatable ^= true;
    }
    pub fn loot(&mut self, lines: Vec<String>) {
        self.0.loot = lines;
    }
    fn slot(&mut self, key: &str) -> Option<&mut Slot> {
        self.0.setup.roster.iter_mut().find(|slot| slot.key == key)
    }
    /// A row's side.
    pub fn side(&mut self, key: &str, side: &str) {
        if let Some(slot) = self.slot(key)
            && !side.trim().is_empty()
        {
            slot.side = side.trim().to_lowercase();
        }
    }
    /// A row's initiative group; blank removes it.
    pub fn initiative_group(&mut self, key: &str, group: &str) {
        if let Some(slot) = self.slot(key) {
            if group.trim().is_empty() {
                slot.extra.remove("initiative_group");
            } else {
                slot.extra.insert("initiative_group".into(), group.into());
            }
        }
    }
    /// A row's starting value (`hp`, `temporary_hp`): on with a value, or off.
    pub fn value(&mut self, key: &str, field: &str, value: Option<i64>) {
        if let Some(slot) = self.slot(key) {
            match value {
                Some(value) => {
                    slot.initial.insert(field.into(), value.max(0).into());
                }
                None => {
                    slot.initial.remove(field);
                }
            }
        }
    }
}

fn fault_line(fault: &dyn design::outcome::Fault) -> Line {
    Line {
        message: design::outcome::Outcome::Fault(design::outcome::FaultMessage::new(fault)).plain(),
        press: None,
    }
}

/// One roster row (`Encounters::roster`), as data.
#[allow(clippy::too_many_arguments)]
fn roster_row(
    ctx: &egui::Context,
    (i, count): (usize, usize),
    slot: &Slot,
    snapshot: &Snapshot,
    live: Option<&Party>,
    open: bool,
    sides: &[String],
    faults_before: Vec<String>,
) -> RosterRow {
    let resolved = slot
        .creature
        .as_ref()
        .and_then(|r| snapshot.catalogue.resolve_ref(r).ok());
    let definition = resolved
        .and_then(|id| snapshot.catalogue.entries.get(&id))
        .map(|e| &e.entry);
    let name = roster_name(slot, definition, &snapshot.known_creature_names);
    let missing = definition.is_none();
    let unreadable = slot
        .creature
        .as_ref()
        .and_then(|reference| snapshot.catalogue.unreadable_ref(reference));
    let creature = resolved.and_then(|id| snapshot.creatures.iter().find(|c| c.id == id));
    let synced = live.is_some() && creature.is_some_and(|c| c.party);
    let sheet = live
        .zip(creature)
        .and_then(|(party, creature)| party.sheet(creature.character.as_deref()?));
    let hp_missing = sheet.is_none()
        && creature.zip(definition).is_some_and(|(creature, entry)| {
            groups::shown_hp_missing(&slot.overrides, entry, &slot.side, creature)
        });
    let detail = sheet.map_or_else(
        || {
            creature
                .zip(definition)
                .map_or_else(String::new, |(creature, entry)| {
                    groups::shown_detail(&slot.overrides, entry, &slot.side, creature)
                })
        },
        |record| crate::party::SheetNow::of(record).line(),
    );
    let detail = detail
        .strip_prefix("Party copy · ")
        .map_or_else(|| detail.clone(), str::to_owned);
    let detail = roster_detail(&detail);
    let detail = match creature.and_then(|creature| creature.source.as_deref()) {
        Some(source) => format!("{detail} · {source}"),
        None => detail,
    };
    let side_icon = if slot.side == "party" {
        icons::PARTY
    } else if slot.side == "enemies" {
        icons::ENCOUNTERS
    } else {
        icons::GROUP
    };
    let identity = definition
        .filter(|_| slot.overrides.contains_key("name"))
        .map(|entry| format!(" · as {}", entry.name))
        .unwrap_or_default();
    let changed = definition.is_some_and(|entry| {
        !(slot.overrides.is_empty()
            || slot.overrides.len() == 1 && snapshot.npc_name_for(entry, &slot.overrides).is_some())
    });
    let side = design::reader::human_label(&slot.side);
    let muted = colors().muted;
    let (fault_icon, fault_color) = missing_creature_mark(unreadable);
    let fault_words = unreadable.map_or(MISSING_CREATURE, |note| {
        if !note.duplicate {
            note.word.as_str()
        } else if crate::world::repair_in_world(&note.path) {
            "Same note in two files"
        } else {
            crate::library::SAME_ENTRY
        }
    });
    let lead = format!("{side_icon} {side}");
    let mut parts: Vec<(String, egui::Color32, bool)> = Vec::new();
    if missing {
        parts.push((lead, muted, false));
        parts.push((fault_words.to_owned(), muted, false));
    } else if detail.is_empty() {
        parts.push((format!("{lead}{identity}"), muted, false));
    } else {
        parts.push((lead, muted, false));
        let whole = format!("{detail}{identity}");
        for part in detail_parts(&whole, hp_missing) {
            parts.push((
                stat_line(part.words),
                part.color(),
                part.words == "Bloodied",
            ));
        }
    }
    let change_word = (changed && !open && !missing)
        .then(|| {
            definition.and_then(|entry| {
                Overrides::parse(&slot.overrides).ok().map(|overrides| {
                    groups::change_words(
                        &overrides,
                        entry,
                        &slot.side,
                        &crate::loot_editor::missing_in(ctx, overrides.loot.as_deref()),
                    )
                })
            })
        })
        .flatten()
        .or_else(|| (changed && !open && !missing).then(|| "Changed".to_owned()));
    let form = (open && !missing).then(|| RowForm {
        side: slot.side.clone(),
        sides: sides.to_vec(),
        group: slot
            .extra
            .get("initiative_group")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned(),
        values: if synced {
            Vec::new()
        } else {
            [("hp", "Starting HP"), ("temporary_hp", "Temp HP")]
                .into_iter()
                .map(|(field, label)| {
                    (
                        field,
                        label,
                        slot.initial
                            .get(field)
                            .and_then(Value::as_u64)
                            .and_then(|value| i64::try_from(value).ok()),
                    )
                })
                .collect()
        },
        help: if synced {
            "D&D Beyond decides this player’s HP while this encounter syncs."
        } else {
            "With a value off, the creature’s own starting state applies."
        },
    });
    RosterRow {
        key: slot.key.clone(),
        index: i,
        name,
        faults_before,
        parts,
        missing: missing.then(|| {
            (
                fault_icon,
                fault_color,
                unreadable
                    .filter(|note| note.duplicate)
                    .map(|note| note.word.clone()),
            )
        }),
        changed: change_word,
        open,
        repair: unreadable.map(|note| {
            (
                if crate::world::repair_in_world(&note.path) {
                    "Open in World"
                } else {
                    "Open in Library"
                },
                note.path.clone(),
            )
        }),
        edit_word: if creature.is_none() { "Replace…" } else { "Edit" },
        make_yours: creature.is_some_and(|c| !c.party),
        first: i == 0,
        last: i + 1 == count,
        form,
    }
}

impl Encounters {
    /// The creature filter's choices: each fact's values among the creatures the open search
    /// offers, with how many have each. Worked out only while the filter's menu is open.
    pub fn filter_choices(
        &self,
    ) -> BTreeMap<splat::content::creature_filter::Facet, Vec<(String, usize)>> {
        let Some(snapshot) = self.snapshot.as_ref() else {
            return BTreeMap::new();
        };
        let unlinked = self
            .draft
            .as_ref()
            .is_none_or(|draft| draft.add_to.is_some());
        splat::content::creature_filter::choices(
            snapshot
                .search_list(unlinked)
                .creatures()
                .filter_map(|c| c.facets.as_ref()),
        )
    }
}
