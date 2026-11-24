//! The Encounters page as plain data, and its acts, for a view that does not draw with egui (the
//! GPUI page, `src/gui/pages/encounters/`). The egui view keeps its own drawing; both views call
//! the acts here and in `Encounters`, so the logic has one home. Nothing here draws or measures:
//! a view fits the words itself.
use super::*;
use crate::design::outcome::Plain;
use crate::encounter_run::SavedRunInfo;
use std::path::PathBuf;

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
                if self.kept.revision() != revision {
                    return;
                }
                let owner = KeptOwner {
                    root: active.path.clone(),
                    kind,
                };
                let mut kept = std::mem::take(&mut self.kept);
                let _ = kept.undo(&owner, |mark| {
                    if self.undo_kept(mark.0.kind, Some((active, campaigns.modules()))) {
                        Ok(())
                    } else {
                        Err(())
                    }
                });
                self.kept = kept;
            }
            (FooterPress::UndoEditor, Some(LinePress::Editor(draft, kind, revision))) => {
                if self.editor_results.revision() != revision {
                    return;
                }
                let owner = EditorOwner {
                    root: active.path.clone(),
                    draft,
                    kind,
                };
                let mut results = std::mem::take(&mut self.editor_results);
                let _ = results.undo(&owner, |mark| match self.draft.as_mut() {
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
                if self.member_results.revision() != revision {
                    return;
                }
                let owner = MemberOwner {
                    root: active.path.clone(),
                    draft,
                };
                let mut results = std::mem::take(&mut self.member_results);
                let _ = results.undo(&owner, |mark| {
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
                if self.separations.revision() != revision {
                    return;
                }
                let Some(owner) = self
                    .separations
                    .newest_first(active.path.as_path())
                    .nth(at)
                    .map(|shown| design::results::Kept::owner(shown.value()))
                else {
                    return;
                };
                match self.separations.undo(&owner, |separation| {
                    separation
                        .change
                        .0
                        .undo(active)
                        .map(|()| separation.path.clone())
                        .map_err(|error| (separation.path.clone(), error))
                }) {
                    Some(Ok((_, path))) => {
                        self.end_separation_failure(&active.path, &path);
                        note_identity_changed(ctx);
                    }
                    Some(Err((path, error))) => {
                        self.separation_error = Some(SeparationFailure {
                            root: active.path.clone(),
                            path,
                            words: error,
                        });
                    }
                    None => {}
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
