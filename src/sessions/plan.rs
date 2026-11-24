//! The Session page's plan: its rows (planned, open and played fights, groups and events), the
//! Add strip, a row's menu and its opened detail.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{Plan, Sessions, kept, kept::PlanKind};
use crate::{
    design::{
        self, Tone,
        outcome::{
            Choice, Fault as MessageFault, FaultMessage, KeptLine, KeptOutcome, MessageChoice,
            Messages, Notice, Outcome, PaneFrame, Said, Sentence, Severity, UndoStep, Verb,
        },
    },
    encounter_run::fight_summary,
    encounters::PlanFight,
    lookup::ContentIndex,
    sessions::{
        ActFault, Apply, NoteUndo, PlanLine, TaskLine, add_scene, change_line, each_linked_name,
        insert_in_plan, linked_names, plain_links, plan_lines, ticked,
    },
    timeline::{
        self, Book, checks,
        page::{PlanNames, added_message, file_act, file_messages},
        removed_event_words,
    },
};
use eframe::egui;
use std::{
    collections::BTreeMap,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use uuid::Uuid;

/// How long the plan's Add result and its Undo live (`kept::PlanAdded`).
pub(super) const UNDO_ADD_FOR: Duration = Duration::from_secs(30);

/// The inputs of the plan's Add words besides its store: the book's revision (the event's rest
/// ruling and the fights in its session) and the saved-run facts' (which of them keep their HP).
pub(super) type AddedInputs = (design::memo::Revision, design::memo::Revision);

/// A plan notice that the DM must check: a deleted run that kept a problem, a rewards fault.
enum PlanFault {
    Check(Sentence),
    /// A fault whose cause gives its own tone (R41: an unreadable run folder is an error).
    Toned(Sentence, Severity),
}

impl MessageFault for PlanFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Check(_) => Severity::NeedsFixing,
            Self::Toned(_, severity) => *severity,
        }
    }
    fn cause(&self) -> Sentence {
        match self {
            Self::Check(words) | Self::Toned(words, _) => words.clone(),
        }
    }
}

/// What the DM chose in the plan's message footer. It is done after the footer is drawn, so
/// the footer's messages borrow nothing that the plan's rows change.
#[derive(Clone, Debug, PartialEq)]
pub enum FooterAct {
    GoToGroup,
    UndoGroupDiscard,
    UndoGroupRemove,
    UndoDeletedRun,
    /// `Undo add`: the press's ticket names the result (`Sessions::added`).
    UndoAdd,
    UndoRemove,
    UndoRuling,
    Scene(SceneAct),
    Rewards(RewardsAct),
    UndoPlanLine,
    File(timeline::page::FileAct),
}

fn pressed(act: FooterAct) -> super::SessionAct {
    super::SessionAct::Plan(act)
}

/// Words the plan builds as text from its data (event labels, rewards, a removal), or that come
/// from a file or the system. The one place this file takes them as a sentence.
#[allow(
    clippy::disallowed_methods,
    reason = "words built from the data of the plan, or from a file or the system, as text (design::results::STAGED_ALLOWS)"
)]
fn as_sentence(words: &str) -> Sentence {
    Sentence::outside(words).unwrap_or(crate::sentence!("Something changed."))
}
/// A list longer than this gets a search, and it scrolls instead of leaving the window.
pub(super) const FIGHT_SEARCH_FROM: usize = 8;

/// The names an Add menu lists for its search words: those that hold them, any case.
pub(super) fn searched<'a>(names: impl IntoIterator<Item = &'a str>, search: &str) -> Vec<&'a str> {
    let query = search.trim().to_lowercase();
    names
        .into_iter()
        .filter(|name| name.to_lowercase().contains(&query))
        .collect()
}

/// Source-ID matches take priority. Legacy names must identify one planned encounter.
pub(super) fn planned_saved_run<'a>(
    fight: &'a PlanFight,
    fights: &[PlanFight],
    runs: &'a [crate::encounter_run::SavedRunInfo],
) -> Option<&'a crate::encounter_run::SavedRunInfo> {
    fight.saved.as_ref().or_else(|| {
        let mut named = fights.iter().filter(|other| other.name == fight.name);
        named.next()?;
        if named.next().is_some() {
            return None;
        }
        // One row descriptor, not a rebuilt catalogue. The shared helper owns saved-name
        // suffix handling; no source ID is inferred for a legacy run.
        let templates = [crate::encounters::Template {
            path: String::new(),
            id: Uuid::nil(),
            name: fight.name.clone(),
            summary: None,
            difficulty: Default::default(),
            repeatable: false,
        }];
        runs.iter().find(|run| {
            run.standing == crate::encounter_run::Standing::Unfinished
                && run.source.is_none()
                && crate::encounters::template_of(run, &templates).is_some()
        })
    })
}

/// A fight's players in the party bar's order, so the two read alike side by side. A player the
/// party no longer has follows, in the fight's order.
fn in_party_order<'a>(
    rows: &'a [crate::encounter_run::FightRow],
    party: &[Uuid],
) -> Vec<&'a crate::encounter_run::FightRow> {
    let mut players: Vec<_> = rows.iter().filter(|row| row.player).collect();
    players.sort_by_key(|row| {
        party
            .iter()
            .position(|id| *id == row.id)
            .unwrap_or(usize::MAX)
    });
    players
}

/// Remove an isolated plan line, but keep words embedded in a sentence.
fn unlink_group(body: &mut String, name: &str) {
    *body = unlink_planned_fight(body, name, 0);
}

fn unlink_planned_fight(body: &str, name: &str, occurrence: usize) -> String {
    let mut in_plan = false;
    let mut found = 0;
    let mut removed = false;
    body.split_inclusive('\n')
        .filter_map(|line| {
            let heading = line.trim();
            if heading.starts_with("## ") {
                in_plan = heading.eq_ignore_ascii_case("## Plan for tonight");
            }
            if !in_plan || removed {
                return Some(line.to_owned());
            }
            let mut start = 0;
            while let Some(at) = line[start..].find("[[") {
                let at = start + at;
                let Some(end) = line[at..].find("]]").map(|end| at + end + 2) else {
                    break;
                };
                let target = &line[at + 2..end - 2];
                if target
                    .split('|')
                    .next()
                    .is_some_and(|target| target.trim().eq_ignore_ascii_case(name))
                {
                    if found == occurrence {
                        removed = true;
                        let prefix = line[..at].trim();
                        if matches!(
                            prefix,
                            "" | "-" | "*" | "- [ ]" | "- [x]" | "- [X]" | "* [ ]" | "* [x]"
                        ) && checks::marked(&line[end..]).words.trim().is_empty()
                        {
                            return None;
                        }
                        return Some(format!(
                            "{}{}{}",
                            &line[..at],
                            target.rsplit('|').next().unwrap_or(target),
                            &line[end..]
                        ));
                    }
                    found += 1;
                }
                start = end;
            }
            Some(line.to_owned())
        })
        .collect()
}

/// A played event's unfinished saved run, unless it is the open run.
pub(super) fn planned_event_run<'a>(
    event: &crate::timeline::Event,
    plan: &Plan<'a>,
    running: bool,
) -> Option<&'a crate::encounter_run::SavedRunInfo> {
    plan.saved_runs.iter().find(|saved| {
        saved.standing == crate::encounter_run::Standing::Unfinished
            && saved.id.is_some()
            && saved.id == event.run
            && !running
    })
}

/// Why a played fight's summary cannot show: its run folder's cause (R41, in Encounters' words
/// and tone), or another cause in words.
#[derive(Debug)]
pub(crate) enum FightFault {
    Folder(crate::encounter_run::RunFolderFault),
    Other(String),
}

/// The open played fight's summary, with the run's file as last found (or why not), by its run.
pub(crate) type KeptFight = (
    Uuid,
    Option<Result<std::path::PathBuf, crate::encounter_run::FindRunFault>>,
    Result<crate::encounter_run::FightSummary, FightFault>,
    // The summary's log as read: a hash of its entries, taken once at the read, which keys the
    // Log's laid-out view (`log_view`), so a log read again with other words is laid out again.
    LogRevision,
);

/// A hash of a played fight's log, taken when its summary is read from disk.
pub(crate) type LogRevision = u64;

/// The revision of `log`: its entries' words, in order.
fn log_revision(log: &[String]) -> LogRevision {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    log.hash(&mut hasher);
    hasher.finish()
}

/// The removal action a deletion of this run follows: the book's live removal, when its event
/// is that same run's. A deletion in another fight or session replaces no removal.
fn removal_replaced_by(book: &Book, deleted_run: Option<Uuid>) -> Option<u64> {
    book.removed_event()
        .filter(|event| deleted_run.is_some() && event.run == deleted_run)
        .and(book.removal_action())
}

/// Where a scene stands among the plan's rows: before the planned fight at that position, or
/// after the last, and its place among the scenes there.
type Slot = (usize, usize);

/// A scene of the plan: a task line that links no fight and no group.
pub(super) struct Scene {
    pub(super) task: TaskLine,
    /// The one entry that its words link, when they link exactly one.
    pub(super) entry: Option<SceneEntry>,
    /// Its words link a name and no entry has it (X14): it says `Not found`.
    pub(super) not_found: bool,
    /// Its entry had a second line when it was last found, so `Not found` takes that line; else
    /// it ends the words' own line, so the row keeps its height (review round 1).
    pub(super) second_line: bool,
}

pub(super) struct SceneEntry {
    /// Its kind, status and place, for a World entry.
    pub(super) detail: Option<String>,
    /// The fight that its `Start fight` starts, for an NPC or a creature.
    pub(super) start: Option<crate::world::StartFight>,
    /// The entry's ID, when it is a campaign World entry whose Log takes a line (F6).
    pub(super) log: Option<(Uuid, String)>,
}

/// A World entry's kind, status and place, as a scene's second line and a suggestion say it.
fn world_detail(index: &ContentIndex, at: usize) -> String {
    let item = &index.items[at];
    let kind = if crate::world::world_type(&item.entry) == "npc" {
        "NPC".to_owned()
    } else {
        item.subtitle.clone()
    };
    let place = index
        .place_of(item.entry.id)
        .map(|place| index.items[place].entry.name.clone());
    [
        Some(kind),
        Some(crate::world::status(&item.entry).to_owned()),
        place,
    ]
    .into_iter()
    .flatten()
    .filter(|part| !part.trim().is_empty())
    .collect::<Vec<_>>()
    .join(" · ")
}

fn is_world(index: &ContentIndex, at: usize) -> bool {
    crate::world::is_world(&index.items[at])
}

/// Whether a plan line's linked names hold a planned fight or a saved group. A task line that
/// links neither is a scene.
pub(super) fn links_fight_or_group(
    names: &[String],
    fights: &[PlanFight],
    groups: &[Arc<crate::encounter_run::RunGroup>],
) -> bool {
    names.iter().any(|name| {
        fights
            .iter()
            .any(|fight| fight.name.eq_ignore_ascii_case(name))
            || groups
                .iter()
                .any(|group| group.name.eq_ignore_ascii_case(name))
    })
}

/// The one entry that a scene's words link, when they link exactly one.
fn scene_entry(index: &ContentIndex, words: &str) -> Option<SceneEntry> {
    let mut found = Vec::new();
    for name in each_linked_name(words) {
        if let crate::world::Go::Item(at, _) = index.resolve_link(&name, None)
            && !found.contains(&at)
        {
            found.push(at);
        }
    }
    let [at] = found[..] else {
        return None;
    };
    let item = &index.items[at];
    Some(SceneEntry {
        detail: is_world(index, at).then(|| world_detail(index, at)),
        start: crate::world::start_fight(item, index),
        log: crate::world::takes_log(item, index).then(|| (item.entry.id, item.entry.name.clone())),
    })
}

/// Whether a scene's words link a name and every one of them is missing from the campaign: its
/// entry is gone. A name that two entries share is not gone. With the content not yet read,
/// this is not known, and the caller says nothing.
fn links_no_entry(index: &ContentIndex, words: &str) -> bool {
    let names = each_linked_name(words);
    !names.is_empty()
        && names
            .iter()
            .all(|name| matches!(index.resolve_link(name, None), crate::world::Go::Missing(_)))
}

/// A scene's words that are outside its links, as `Add to Log` starts its field with them:
/// `[[Captain Varra]]: the ropes` gives `the ropes`.
pub(super) fn words_outside_links(words: &str) -> String {
    let mut out = String::new();
    let mut rest = words;
    while let Some(start) = rest.find("[[") {
        out.push_str(&rest[..start]);
        match rest[start..].find("]]") {
            Some(end) => rest = &rest[start + end + 2..],
            None => {
                rest = &rest[start..];
                break;
            }
        }
    }
    out.push_str(rest);
    out.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(|c: char| c.is_whitespace() || matches!(c, ':' | ';' | ',' | '-' | '–' | '—'))
        .to_owned()
}

/// One line of the plan as the plan draws it.
enum ModelLine {
    Scene(Scene),
    /// The planned fights that one line links, in its order.
    Fights(Vec<String>),
}

/// The note's plan as rows: which lines are scenes, with what each scene links, and which
/// fights each other line plans. The one owner of that rule; Read uses it too.
pub(super) struct PlanModel {
    lines: Vec<ModelLine>,
}

impl PlanModel {
    /// The scene on model line `line` (a line a projection listed as a scene).
    pub(super) fn scene(&self, line: usize) -> &Scene {
        match &self.lines[line] {
            ModelLine::Scene(scene) => scene,
            ModelLine::Fights(_) => unreachable!("a projection lists only scene lines as scenes"),
        }
    }

    /// The work that the cache keeps off quiet frames: the note's plan read line by line, and
    /// each scene's link resolved.
    pub(super) fn build(
        body: &str,
        fights: &[PlanFight],
        groups: &[Arc<crate::encounter_run::RunGroup>],
        index: Option<&ContentIndex>,
    ) -> Self {
        let lines = plan_lines(body)
            .into_iter()
            .map(|line| {
                let (names, task) = match line {
                    PlanLine::Task(task) => (each_linked_name(&task.words), Some(task)),
                    PlanLine::Links(names) => (names, None),
                };
                match task.filter(|_| !links_fight_or_group(&names, fights, groups)) {
                    Some(task) => {
                        let entry = index.and_then(|index| scene_entry(index, &task.words));
                        let not_found =
                            index.is_some_and(|index| links_no_entry(index, &task.words));
                        ModelLine::Scene(Scene {
                            task,
                            entry,
                            not_found,
                            second_line: false,
                        })
                    }
                    None => ModelLine::Fights(
                        names
                            .into_iter()
                            .filter_map(|name| {
                                fights
                                    .iter()
                                    .find(|fight| fight.name.eq_ignore_ascii_case(&name))
                                    .map(|fight| fight.name.clone())
                            })
                            .collect(),
                    ),
                }
            })
            .collect();
        Self { lines }
    }

    /// What each linked name's entry looked like when it was found: whether it had a second
    /// line. A scene whose entry is gone keeps the row it had: `Not found` on that second line,
    /// else at the end of its words' line. `seen` is the plan's, kept from one build to the next.
    fn place_not_found(&mut self, seen: &mut std::collections::HashMap<String, bool>) {
        for line in &mut self.lines {
            let ModelLine::Scene(scene) = line else {
                continue;
            };
            let names = each_linked_name(&scene.task.words);
            if let Some(entry) = &scene.entry {
                for name in &names {
                    seen.insert(name.to_lowercase(), entry.detail.is_some());
                }
            }
            scene.second_line = scene.not_found
                && names
                    .iter()
                    .any(|name| seen.get(&name.to_lowercase()) == Some(&true));
        }
    }

    /// Each scene's whole line, as the note holds it.
    pub(super) fn scene_lines(&self) -> Vec<&str> {
        self.lines
            .iter()
            .filter_map(|line| match line {
                ModelLine::Scene(scene) => Some(scene.task.line.as_str()),
                ModelLine::Fights(_) => None,
            })
            .collect()
    }
}

/// The plan model, built again only when what it reads changes: the session, the note's words
/// (its draft's revision), the campaign's content (the index's generation), or the names of the
/// campaign's encounters and groups.
#[derive(Default)]
pub(super) struct PlanModelCache {
    key: Option<(Option<Uuid>, u64, Option<u64>, design::memo::Revision)>,
    model: Option<Arc<PlanModel>>,
    /// How often a frame asked for the model, and how often it was built.
    pub(super) reached: usize,
    pub(super) builds: usize,
    /// Per linked name, whether its entry had a second line when last found
    /// (`PlanModel::place_not_found`).
    seen: std::collections::HashMap<String, bool>,
}

/// What a plan projection is worked out from: revisions and ids only, so a quiet frame compares
/// a few words and hashes no name (layout 11c A2a).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProjectionInputs {
    session: Option<Uuid>,
    draft: u64,
    book: design::memo::Revision,
    /// The campaign's fights and groups: both come from the Encounters snapshot, whose revision
    /// `Plan::fights_revision` is.
    fights: design::memo::Revision,
    index: Option<u64>,
    // Not the saved runs (review of PR 820): the projection reads a run only to say whether it
    // ended, and a fight lists only its UNFINISHED run (`Encounters::plan_fights`), so no
    // projected word changes with them. The rows read a fight's run themselves.
}

/// The plan's projection, kept by its inputs' revision.
#[derive(Default)]
pub(super) struct ProjectionCache {
    inputs: Option<design::memo::Revised<ProjectionInputs>>,
    kept: design::memo::Memo<design::memo::Revision, Arc<PlanProjection>>,
    /// How often a frame asked for the projection, and how often one was built (the
    /// quiet-frame proof counts both).
    pub(super) reached: usize,
    pub(super) builds: usize,
}

impl ProjectionCache {}

/// What the plan's closed rows read from their data (layout 11c A2a, the v2 review's item 5),
/// worked out once per change of the note, the Timeline, the fights and groups, the saved runs,
/// the content index or the session: the scenes and fights at their places, the groups the note
/// links, the heading's count, the Timeline's days, which events the note links, and each planned
/// fight's removal facts. A quiet frame reads it and copies nothing.
pub(super) struct PlanProjection {
    pub(super) model: Arc<PlanModel>,
    /// The planned fights still to come, by their place in `Plan::fights`.
    pub(super) planned: Vec<usize>,
    /// Each scene's line among the model's lines, at each place (before each planned fight and
    /// after the last).
    pub(super) scenes: Vec<Vec<usize>>,
    /// The place a row stands at before each model line, and after the last.
    pub(super) line_place: Vec<(usize, usize)>,
    pub(super) scene_count: usize,
    /// The groups the note links that are not fights, by their place in `Plan::groups`.
    pub(super) planned_groups: Vec<usize>,
    /// The Timeline's events in this session.
    pub(super) events_here: usize,
    /// The ids of the events the note links (`^id`).
    pub(super) linked: std::collections::HashSet<String>,
    pub(super) days: Vec<u32>,
    /// Each planned fight's name, and whether its saved run ended.
    pub(super) planned_results: Vec<(String, bool)>,
}

impl PlanProjection {
    fn build(
        model: Arc<PlanModel>,
        body: &str,
        book: &Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
    ) -> Self {
        let mut played: BTreeMap<String, usize> = BTreeMap::new();
        for event in &book.events {
            if event.is_in(session) && event.run.is_some() {
                *played.entry(event.title.to_lowercase()).or_default() += 1;
            }
        }
        let mut planned = Vec::new();
        let mut scenes: Vec<Vec<usize>> = vec![Vec::new()];
        let mut line_place = Vec::with_capacity(model.lines.len() + 1);
        let place_now =
            |scenes: &Vec<Vec<usize>>| (scenes.len() - 1, scenes.last().map_or(0, Vec::len));
        for (at, line) in model.lines.iter().enumerate() {
            line_place.push(place_now(&scenes));
            match line {
                ModelLine::Scene(_) => scenes
                    .last_mut()
                    .expect("one list of scenes for each place")
                    .push(at),
                ModelLine::Fights(names) => {
                    for name in names {
                        let Some(found) = plan.fights.iter().position(|fight| fight.name == *name)
                        else {
                            continue;
                        };
                        let done = played
                            .entry(plan.fights[found].name.to_lowercase())
                            .or_default();
                        let waiting = *done == 0;
                        *done = done.saturating_sub(1);
                        if waiting {
                            planned.push(found);
                            scenes.push(Vec::new());
                        }
                    }
                }
            }
        }
        line_place.push(place_now(&scenes));
        let planned_groups = linked_names(body)
            .iter()
            .filter(|name| {
                !plan
                    .fights
                    .iter()
                    .any(|fight| fight.name.eq_ignore_ascii_case(name))
            })
            .filter_map(|name| {
                plan.groups
                    .iter()
                    .position(|group| group.name.eq_ignore_ascii_case(name))
            })
            .collect();
        let planned_results = planned
            .iter()
            .map(|&at| {
                let fight = &plan.fights[at];
                (
                    fight.name.clone(),
                    planned_saved_run(fight, plan.fights, plan.saved_runs).is_some_and(|saved| {
                        saved.standing == crate::encounter_run::Standing::Ended
                    }),
                )
            })
            .collect();
        Self {
            scene_count: scenes.iter().map(Vec::len).sum(),
            events_here: book
                .events
                .iter()
                .filter(|event| event.is_in(session))
                .count(),
            linked: book
                .events
                .iter()
                .filter(|event| body.contains(&format!("^{}", event.id)))
                .map(|event| event.id.clone())
                .collect(),
            days: timeline::day_numbers(&book.events),
            model,
            planned,
            scenes,
            line_place,
            planned_groups,
            planned_results,
        }
    }
}

/// Where a scene act's words show: under the scene's row, under the Scene field, or in the
/// plan's message footer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    Row(Slot),
    Field,
    Footer,
}

/// Where each scene result shows in this frame, decided once before anything is drawn
/// (`Sessions::shown_at`).
#[derive(Clone, Copy, Default)]
pub(super) struct Shown {
    pub(super) added: Option<Place>,
    pub(super) removed: Option<Place>,
    pub(super) fault: Option<Place>,
}

/// What a scene act could not do, while the note's words are the words it met.
#[derive(Debug)]
pub(super) struct Fault {
    pub(super) what: SceneFault,
    pub(super) met: String,
    pub(super) place: Place,
    /// The act waits in the draft: its words end when the note is saved.
    pub(super) waits: bool,
}

/// What a scene act could not do.
#[derive(Debug)]
pub(super) enum SceneFault {
    /// It was refused and wrote nothing: what was not done to which scene, why, and the next
    /// step (ruling M7).
    Refused(design::results::Refusal),
    /// It is in the draft, and the note could not be saved: it waits for the next save.
    NotSaved,
}

impl MessageFault for SceneFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Refused(refusal) => kept::RefusalFault(refusal).severity(),
            Self::NotSaved => Severity::NotSaved,
        }
    }
    fn cause(&self) -> Sentence {
        match self {
            Self::Refused(refusal) => refusal.words(),
            Self::NotSaved => crate::sentence!("Notes not saved."),
        }
    }
}

/// What the scenes' fold measured on the last frame. Scenes fold first, so that the next
/// fight's row and its primary action stay in view and nothing else in the plan moves or narrows
/// because scenes exist. Each row counts as it is when closed. An opened row folds the scenes
/// only when its details push the next fight out of view and the scenes sit above that fight,
/// or, in a plan with no fight still to come, when the rows with its details do not fit; the
/// Scene field, its suggestions and the plan's results never fold them.
#[derive(Default)]
struct Fold {
    /// The pane's height, once a frame measured it.
    viewport: Option<f32>,
    measure: fold::Measure,
    /// The DM opened (`true`) or closed (`false`) the folded scenes in this session.
    open: Option<bool>,
    /// Whether the scenes give way, as last decided. It holds while the DM types a scene.
    overflow: bool,
}

mod fold {
    /// What one frame measured for the fold, down the rows.
    #[derive(Default, Clone, Copy)]
    pub(super) struct Measure {
        /// The closed rows' height without the scenes, and the scenes' height when shown.
        pub(super) closed: f32,
        pub(super) scenes: f32,
        /// The opened rows' details, in a plan with no fight still to come.
        pub(super) details: f32,
        pub(super) next: Option<Next>,
    }

    /// The next fight still to come.
    #[derive(Default, Clone, Copy)]
    pub(super) struct Next {
        /// The closed rows above it without the scenes, and its own row with its details.
        pub(super) bottom: f32,
        /// The opened rows' details above it.
        pub(super) details: f32,
        /// The scenes above it, when shown.
        pub(super) scenes: f32,
    }

    impl Measure {
        /// This frame's measure while the scenes are folded: they keep their last height.
        pub(super) fn folded(self, now: Self) -> Self {
            Self {
                scenes: self.scenes,
                next: now.next.map(|next| Next {
                    scenes: self.next.map_or(0.0, |last| last.scenes),
                    ..next
                }),
                ..now
            }
        }
    }
}

impl Fold {
    /// Whether the scenes give way: the closed rows, with the scenes, are taller than the pane;
    /// or the opened rows' details push the next fight's row out of the pane and scenes sit
    /// above that fight; or, with no fight still to come, the rows with the opened rows'
    /// details are taller than the pane.
    fn measured(&self) -> bool {
        let Some(viewport) = self.viewport else {
            return false;
        };
        let m = &self.measure;
        let rows = m.closed + m.scenes > viewport + 0.5;
        let pushed = match m.next {
            Some(next) => {
                next.scenes > 0.0
                    && next.details > 0.0
                    && next.bottom + next.scenes + next.details > viewport + 0.5
            }
            // With no fight still to come, the rows with an opened row's details.
            None => m.details > 0.0 && m.closed + m.scenes + m.details > viewport + 0.5,
        };
        rows || pushed
    }

    /// Whether the scenes fold now, and whether the heading offers `N scenes`. While `hold`, the
    /// fold stays as it was, so a scene that the DM just added stays where the DM sees it.
    fn state(&mut self, count: usize, hold: bool) -> (bool, bool) {
        if !hold {
            self.overflow = self.measured();
        }
        if count == 0 {
            return (false, false);
        }
        let folded = self.open.map_or(self.overflow, |open| !open);
        (folded, self.overflow || self.open.is_some())
    }
}

/// What the plan's scenes keep between frames. Each result belongs to its act and ends as that
/// act's Undo does: at any other change of the note's words, or in another note.
#[derive(Default)]
pub(super) struct Scenes {
    /// The Scene field, while it is open.
    pub(super) field: Option<design::AddField>,
    /// `Scene added to tonight.`
    pub(super) added: Option<NoteUndo>,
    /// The words of the scene in `added`, which a refused Undo of it names.
    pub(super) added_words: String,
    /// A scene taken out of tonight: its line's place among the plan's lines, its words, and
    /// its Undo. The place is the line that now follows it, which stays while the note is as the
    /// act left it, also when a fight before it is played.
    pub(super) removed: Option<(usize, String, NoteUndo)>,
    pub(super) fault: Option<Fault>,
    fold: Fold,
    /// The Scene field had focus on the last frame.
    pub(super) typing: bool,
}

impl Scenes {
    /// An Undo ends for good at another change of the note's words or in another note, also
    /// when the words come back to what the act left.
    pub(super) fn end_what_changed(&mut self, draft: Option<&crate::sessions::Draft>) {
        if self.fault.as_ref().is_some_and(|fault| {
            draft.is_none_or(|draft| {
                !draft.body.same_words(&fault.met) || (fault.waits && !draft.unsaved())
            })
        }) {
            self.fault = None;
        }
        // A scene act whose save failed is written once a later save wrote the note.
        if let Some(draft) = draft {
            for undo in self
                .added
                .iter_mut()
                .chain(self.removed.iter_mut().map(|(.., undo)| undo))
            {
                undo.follow_save(draft);
            }
        }
        let live = |undo: &NoteUndo| draft.is_some_and(|draft| undo.live(draft));
        if self.added.as_ref().is_some_and(|undo| !live(undo)) {
            self.added = None;
        }
        if self.removed.as_ref().is_some_and(|(.., undo)| !live(undo)) {
            self.removed = None;
        }
    }

    /// The note moved to a new file, and the Undos follow it.
    pub(super) fn moved(&mut self, from: &Path, to: &Path) {
        if let Some(undo) = &mut self.added {
            undo.moved(from, to);
        }
        if let Some((.., undo)) = &mut self.removed {
            undo.moved(from, to);
        }
    }
}

/// A scene act that the plan takes after it draws.
#[derive(Clone, Debug, PartialEq)]
pub enum SceneAct {
    Add(String),
    Tick(Slot, TaskLine),
    /// The row's place, and its line's place among the plan's lines.
    Remove(Slot, usize, TaskLine),
    UndoAdd,
    /// Where its Undo was drawn, for what it could not do.
    UndoRemove(Place),
    StartFight(crate::world::StartFight),
    /// A done scene's `Add to Log`: its entry, its name, and the scene's words outside its link.
    AddToLog(Uuid, String, String),
}

/// Each link to a fight under the plan's heading, in order: where its line starts in the words,
/// the fight, and whether it is still to come. A played event of the session takes one link of
/// its name, so a plan can hold the same fight again.
fn plan_links<'a>(
    body: &str,
    events: &[timeline::Event],
    session: Option<Uuid>,
    fights: &'a [PlanFight],
) -> Vec<(usize, &'a PlanFight, bool)> {
    let mut played: BTreeMap<String, usize> = BTreeMap::new();
    for event in events {
        if event.is_in(session) && event.run.is_some() {
            *played.entry(event.title.to_lowercase()).or_default() += 1;
        }
    }
    let mut links = Vec::new();
    let mut inside = false;
    let mut start = 0;
    for line in body.split_inclusive('\n') {
        let at = start;
        start += line.len();
        let text = line.trim_end_matches(['\n', '\r']);
        if !inside {
            inside = text.trim_end().strip_prefix("## ") == Some("Plan for tonight");
            continue;
        }
        if text.starts_with("## ") {
            break;
        }
        for name in each_linked_name(text) {
            if let Some(fight) = fights
                .iter()
                .find(|fight| fight.name.eq_ignore_ascii_case(&name))
            {
                let done = played.entry(fight.name.to_lowercase()).or_default();
                links.push((at, fight, *done == 0));
                *done = done.saturating_sub(1);
            }
        }
    }
    links
}

/// The note's words with a plan line before the first line still to come, so that its fight is
/// the next row. With nothing to come, it goes at the plan's end.
fn plan_first(
    body: &str,
    line: &str,
    events: &[timeline::Event],
    session: Option<Uuid>,
    fights: &[PlanFight],
) -> String {
    let mut words = body.to_owned();
    match plan_links(body, events, session, fights)
        .into_iter()
        .find(|(_, _, waiting)| *waiting)
    {
        Some((at, _, _)) => words.insert_str(at, &format!("{line}\n")),
        None => insert_in_plan(&mut words, line),
    }
    words
}

/// The plan line of `Plan “<name>”`: the link, and the mark of the journey and the check it was
/// clicked on, so that starting the fight plays it at that check (F9, W5-D1).
fn road_plan_line(book: &Book, journey: &str, chosen: &checks::Planned) -> String {
    let mark = book
        .events
        .iter()
        .find(|event| event.id == journey)
        .and_then(|event| event.checks.iter().find(|check| chosen.on(check)))
        .and_then(|check| checks::RoadMark::new(journey, check));
    match mark {
        Some(mark) => format!("- [[{}]] {}", chosen.name, mark.comment()),
        None => format!("- [[{}]]", chosen.name),
    }
}

/// Row 123: the one rule of a check result's offer, on the plan and on Timeline. For a name the
/// result links: nothing when it is no fight and no group of the campaign (`known`), else whether
/// a row for it is still to come in the plan (`coming`).
fn offer_state<'a>(
    name: &str,
    mut known: impl Iterator<Item = &'a str>,
    mut coming: impl Iterator<Item = &'a str>,
) -> Option<bool> {
    known
        .any(|known| known.eq_ignore_ascii_case(name))
        .then(|| coming.any(|coming| coming.eq_ignore_ascii_case(name)))
}

/// Row 123: a session plan as Timeline's check results see it: the names the campaign can plan,
/// and those with a row still to come in that session's plan.
#[derive(Debug, Default, PartialEq)]
pub struct JourneyPlan {
    known: Vec<String>,
    coming: Vec<String>,
}

impl JourneyPlan {
    /// The plan as names: those the campaign can plan, and those with a row still to come.
    pub(super) fn of(known: Vec<String>, coming: Vec<String>) -> Self {
        Self { known, coming }
    }

    /// What `checks::View::planned` asks, by the plan's own rule (`offer_state`).
    pub fn get(&self, name: &str) -> Option<bool> {
        offer_state(
            name,
            self.known.iter().map(String::as_str),
            self.coming.iter().map(String::as_str),
        )
    }

    /// Read from the plan's words: the fights still to come take one link each, as the plan's
    /// rows do (`plan_links`), and a linked group is to come. The offer and the write ask this
    /// one reader (row 123, point 3).
    fn read(
        body: &str,
        events: &[timeline::Event],
        session: Option<Uuid>,
        fights: &[String],
        groups: &[String],
    ) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        timeline::page::JOURNEY_PLAN_READS.with(|reads| reads.set(reads.get() + 1));
        let stubs = plan_fight_stubs(fights);
        let mut coming: Vec<String> = plan_links(body, events, session, &stubs)
            .into_iter()
            .filter(|(_, _, waiting)| *waiting)
            .map(|(_, fight, _)| fight.name.clone())
            .collect();
        coming.extend(linked_names(body).into_iter().filter(|name| {
            !fights.iter().any(|fight| fight.eq_ignore_ascii_case(name))
                && groups.iter().any(|group| group.eq_ignore_ascii_case(name))
        }));
        Self {
            known: fights.iter().chain(groups).cloned().collect(),
            coming,
        }
    }
}

/// Row 123: `Plan “<name>”`, done: the journey and the check it was clicked on, the note it
/// wrote (its session, its label and its file), the line it wrote and that note's Undo. Kept in
/// one `design::Results` store (`Travel::planned`) that both readers, the Session plan and
/// Timeline, read. It lasts while the note's words are what the act left, as every note act's
/// Undo does. Once its Undo was refused (R45), its Undo is never offered again: the refusal
/// stays until its own events end it, among them the line leaving the note.
struct PlannedLine {
    /// The source of the note's words that the result last held at (`follow_plan_line`): the
    /// draft's revision, or the disk generation. Taken right after the act.
    seen: NoteSource,
    /// The owner: the campaign the act was made in. The result ends when the campaign changes
    /// or the session's note leaves the folder (`Sessions::planned`).
    campaign: Option<Uuid>,
    journey: String,
    chosen: checks::Planned,
    note: Uuid,
    /// The session's label at the act (`Session 1: The Crossing`), for its answers: never the
    /// label of the session selected now.
    session: String,
    /// What tells this line apart from a line that reads alike (the same fight in a session of
    /// the same label), made at `record` and kept on the entry: its journey's label, then a
    /// number (`Travel · 2`). `None` when no other line reads alike then.
    apart: Option<String>,
    /// Its number among lines that read alike and share the journey's label (1 for the first).
    ordinal: u32,
    /// The journey's label at the act.
    journey_label: String,
    /// The plan line as the act wrote it: a refused answer ends when the note no longer holds it.
    line: String,
    path: std::path::PathBuf,
    undo: NoteUndo,
}

/// The owner of a plan line's result and of its refused Undo: the object of the act, the
/// campaign, the session's note and the journey. A new plan line for the same note and journey
/// replaces it; one for another note or journey stands beside it, and neither ends the other's
/// answer.
#[derive(Clone, PartialEq, Eq)]
struct PlanLineOwner {
    campaign: Option<Uuid>,
    note: Uuid,
    journey: String,
}

impl design::results::Owner for PlanLineOwner {
    type Scope = Option<Uuid>;
    fn scope(&self) -> &Option<Uuid> {
        &self.campaign
    }
}

impl design::results::Kept for PlannedLine {
    type Owner = PlanLineOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> PlanLineOwner {
        PlanLineOwner {
            campaign: self.campaign,
            note: self.note,
            journey: self.journey.clone(),
        }
    }
}

/// A plan line's result words and its Undo step. Alone in its footer: `“Shrine of Cinders” is
/// in tonight’s plan.` and `Undo add plan line`. Beside other plan lines (`several`), each names
/// its session, and what the entry keeps to tell it apart (`PlannedLine::apart`), so the DM can
/// tell which note a control changes: `“Shrine of Cinders” is in the plan of Session 1: Road.`
/// and `Undo add plan line for Shrine of Cinders in Session 1: Road`. The told-apart part is
/// kept whole by every fit of the Undo (`UndoStep::tell_apart`).
fn plan_line_words(planned: &PlannedLine, several: bool) -> (Sentence, UndoStep) {
    let step = UndoStep::of(Verb::Add, "plan line");
    if !several {
        return (as_sentence(&planned.chosen.words()), step);
    }
    let (name, place) = (&planned.chosen.name, plan_line_place(planned));
    let words = as_sentence(&format!("“{name}” is in the plan of {place}."));
    (words, step.tell_apart(format!("for {name} in {place}")))
}

/// What tells a new plan line apart from the lines that read alike (`alike`: each one's journey
/// label, number and stored words): nothing when there is none; else its journey's label; else,
/// when another alike line has that label, the label and the next number. The words are then
/// compared with the stored words of every alike line, and the number goes on until no line has
/// them (review N6: a journey titled `Travel · 2` meets the numbered `Travel · 2`). Lines already
/// kept never change their words.
fn told_apart(journey_label: &str, alike: &[(&str, u32, Option<&str>)]) -> (Option<String>, u32) {
    if alike.is_empty() {
        return (None, 1);
    }
    let same = alike
        .iter()
        .filter(|(label, ..)| *label == journey_label)
        .map(|(_, ordinal, _)| *ordinal)
        .max();
    let (mut apart, mut ordinal) = match same {
        None => (journey_label.to_owned(), 1),
        Some(last) => (format!("{journey_label} · {}", last + 1), last + 1),
    };
    while alike
        .iter()
        .any(|(.., taken)| *taken == Some(apart.as_str()))
    {
        ordinal += 1;
        apart = format!("{journey_label} · {ordinal}");
    }
    (Some(apart), ordinal)
}

/// The session's label, then what tells the line apart: `Session 1: Road · Travel · 2`.
fn plan_line_place(planned: &PlannedLine) -> String {
    match &planned.apart {
        Some(apart) => format!("{} · {apart}", planned.session),
        None => planned.session.clone(),
    }
}

impl PlannedLine {
    /// The answer to a refused Undo of this line (R45): what was not done, to which fight, in
    /// which session, why, and the one next step, by hand. Its Undo is not offered again.
    fn refusal(&self, fault: &ActFault, unreadable: bool) -> design::results::Refusal {
        use design::results::{
            Cause, Change, FailedAct, Fallback, File, Kind, Name, Recovery, Refusal, Thing,
        };
        let what = Thing::new(
            Kind::PlanLine,
            Name::new(self.chosen.name.as_str()).unwrap_or(Name::fallback(Fallback::ThisFight)),
        );
        let from =
            Name::new(self.session.as_str()).unwrap_or(Name::fallback(Fallback::ThatSession));
        let file = File::SessionNote;
        let cause = match fault {
            ActFault::LineChanged => Cause::Changed(file),
            ActFault::Changed => Cause::Conflict(file),
            ActFault::Missing if unreadable => Cause::Unreadable(file),
            ActFault::Missing => Cause::Gone(file),
            ActFault::OtherNote => Cause::Replaced(file),
            // Only a draft of its own reaches here: it was dropped, so the file still holds the
            // line.
            ActFault::NotSaved(..) => Cause::NotSaved(file),
        };
        Refusal::new(
            FailedAct::NotRemoved { what, from },
            cause,
            Recovery::ByHand {
                file,
                change: Change::RemovePlanLine,
            },
        )
    }
}

/// Row 123, point 2: a session note that no draft holds, as the periodic disk check last read
/// it: its campaign and file, its words, and a count that changes only when the words do.
struct Watched {
    campaign: Option<Uuid>,
    path: std::path::PathBuf,
    body: Option<String>,
    generation: u64,
    /// The disk check (`Sessions::last_check`) that read it.
    tick: Option<Instant>,
}

/// Where a note's words come from for Timeline: the open draft (its revision), or the file as
/// the last disk check read it (its generation).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NoteSource {
    Draft(u64),
    Disk(u64),
}

/// Fights known only by their names: what `plan_links` and `plan_first` read of a fight.
fn plan_fight_stubs(names: &[String]) -> Vec<PlanFight> {
    names
        .iter()
        .map(|name| PlanFight {
            saved: None,
            id: Uuid::nil(),
            name: name.clone(),
            path: String::new(),
            summary: Default::default(),
            revision: None,
            difficulty: Default::default(),
        })
        .collect()
}

/// The result of a plan line's act that wrote nothing, before its cause.
const NOT_ADDED: &str = "Plan line not added.";

/// What a plan line's write did.
enum Written {
    /// The fight is already to come in the words as they are now: nothing was written.
    Already,
    Done(Result<NoteUndo, ActFault>),
}

/// Row 123, point 3: `Plan “<name>”`'s line into `draft`, through the note's checked path
/// (`Apply::Now`: it follows the disk first). The words as they are then decide, by the offer's
/// own reader: a fight still to come there gets no second line.
fn write_plan_line(
    draft: &mut crate::sessions::Draft,
    note: Uuid,
    book: &Book,
    journey: &str,
    chosen: &checks::Planned,
    fights: &[PlanFight],
    groups: &[String],
) -> Written {
    let line = road_plan_line(book, journey, chosen);
    let names: Vec<String> = fights.iter().map(|fight| fight.name.clone()).collect();
    let mut already = false;
    let written = draft.apply(note, Apply::Now, |words| {
        let plan = JourneyPlan::read(words, &book.events, Some(note), &names, groups);
        if plan.get(&chosen.name) == Some(true) {
            already = true;
            return None;
        }
        Some(plan_first(words, &line, &book.events, Some(note), fights))
    });
    if already {
        Written::Already
    } else {
        Written::Done(written)
    }
}

/// A journey's encounter checks in the plan: what the DM typed, each journey's last result with
/// its Undo, and the plan line's Undo. The length being typed is the Timeline book's.
#[derive(Default)]
pub(super) struct Travel {
    pub(super) block: checks::State,
    /// The open journey's start on the clock, while the Timeline is as it was: its count of
    /// changes, the event, its place in the list, and the minute.
    start: Option<(u64, String, Option<usize>, Option<u64>)>,
    pub(super) focus_length: bool,
    /// Each journey's last Timeline act, by its event. Leaving the page ends them.
    pub(super) done: checks::Results,
    /// `Plan “<name>”`: the journey, the name and the check it was clicked on, and the note's
    /// Undo, with its order and the refusal of its last Undo. It lasts while the note's words are
    /// what the act left, as every note act's Undo does; a page change keeps it. The one store
    /// both readers read.
    planned: design::Results<PlanLineOwner, PlannedLine>,
    /// Row 123: the plan that Timeline's open journey offers to, while its session, its note,
    /// the Timeline and the campaign's names are as they were.
    journey_plan: Option<(JourneyKey, Arc<JourneyPlan>)>,
    /// Row 123, point 2: the session notes Timeline reads that no draft holds, by session.
    watched: BTreeMap<Uuid, Watched>,
    /// The last generation a watched note was given.
    generation: u64,
    /// The campaign and the folder's listing (`Sessions::notes_revision`) that the plan lines'
    /// owners were last checked against (`reconcile_plan_owners`).
    owners_seen: Option<(Option<Uuid>, u64)>,
    /// The Timeline revision that the plan lines' journeys were last checked against
    /// (`end_plan_answers_of_lost_journeys`).
    journeys_seen: Option<crate::design::memo::Revision>,
    /// What the plan lines' results were last followed at (`follow_plan_line`): the campaign,
    /// the selection, the open draft's revision and the disk check. Any change of the store or
    /// of a watched note's words forgets it.
    followed: Option<FollowKey>,
}

impl Travel {
    /// `planned` changed, or what a look at it read: the next look walks it again.
    fn forget_look(&mut self) {
        self.followed = None;
    }
}

/// The label of the journey `id` (`Ford crossing`), for its plan line's told-apart words.
fn journey_label(book: &Book, id: &str) -> String {
    let days = timeline::day_numbers(&book.events);
    book.events
        .iter()
        .position(|event| event.id == id)
        .map_or_else(String::new, |at| book.events[at].label(days[at]))
}

/// See `Travel::followed`.
type FollowKey = (
    Option<Uuid>,
    Option<Uuid>,
    Option<(u64, bool)>,
    Option<Instant>,
);

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Tests: how many times a plan line's owners (notes, journeys) were looked up. An
    /// unchanged frame looks up none.
    pub(super) static PLAN_OWNER_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Tests: how many times the plan lines' results were walked to follow their notes.
    pub(super) static PLAN_FOLLOW_WALKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// What Timeline's view of a session plan is read from: the campaign, the session and its file,
/// the note's words (`NoteSource`), the Timeline's count of changes, and the read of the
/// campaign's names.
type JourneyKey = (
    Option<Uuid>,
    Uuid,
    std::path::PathBuf,
    NoteSource,
    u64,
    Option<Uuid>,
);

const REWARDS_BLOCKED: &str = "Splat cannot read this fight’s run, so its rewards are not known.";

/// What the DM pressed for a played fight: `Add rewards`, or `Undo rewards`, by its event.
#[derive(Clone, Debug, PartialEq)]
pub enum RewardsAct {
    Add(String, Uuid),
    Undo(String),
}

/// One `Add rewards` that holds: the block as it left it, and the words of its result.
struct RewardsDone {
    written: crate::sessions::Rewarded,
    /// `XP 400, as in your last rewards.`, or the words for no XP or a choice not known.
    choice: String,
    with_xp: bool,
    not_rolled: usize,
    unknown: u64,
    /// The fight's name as the act read it, so the footer searches no DM data each frame
    /// (review of PR 296, item 1).
    fight: String,
}

/// A played fight's name for its rewards result and fault, read once at the act.
fn fight_name(book: &Book, event: &str) -> String {
    #[cfg(any(test, feature = "test-support"))]
    FIGHT_NAME_READS.with(|reads| reads.set(reads.get() + 1));
    book.events
        .iter()
        .find(|one| one.id == event)
        .map_or_else(|| "this fight".to_owned(), |one| one.title.clone())
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times a rewards act read a fight's name from the Timeline.
    static FIGHT_NAME_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// What the selected note says now, read once for each revision of its draft.
struct RewardsSeen {
    revision: u64,
    /// The Timeline IDs that a list line starts with: those fights say `In notes`.
    listed: std::collections::BTreeSet<String>,
    /// The XP lines' sum and how many fights it sums.
    tonight: (u64, usize),
}

/// The rewards record (F7) in the plan. Each result is kept by its session note's ID and its
/// fight, and shows in the plan's message footer, naming the fight, while its block stands as
/// the act left it (round 27 FN1, FK3, SK2): the fight's row keeps only `✓ In notes`. Another
/// act, another session or another page leaves it.
#[derive(Default)]
pub(super) struct Rewards {
    done: BTreeMap<(Uuid, String), RewardsDone>,
    /// Each result of `done` in the plan's store: its footer order (issued at the act, so the
    /// footer lists results newest first) and its Undo's ticket.
    marks: design::Results<RewardsOwner, RewardsMark>,
    /// What an act could not do: the note, the fight, the whole words (with the fight's name,
    /// read at the act), the draft's revision it met, and its tone (R41: a run folder's cause
    /// keeps its own). A change of the note ends it.
    fault: Option<(Uuid, String, String, u64, Severity)>,
    seen: Option<RewardsSeen>,
}

/// Whose rewards result it is: the note the act wrote in, and the played fight's event. A note's
/// ID is its own in every campaign.
#[derive(Clone, Debug, PartialEq, Eq)]
struct RewardsOwner {
    note: Uuid,
    event: String,
}

impl design::results::Owner for RewardsOwner {
    type Scope = ();
    fn scope(&self) -> &() {
        &()
    }
}

/// A rewards result in the plan's store: its owner only. Its words and its Undo come from
/// `Rewards::done`, which ends it.
struct RewardsMark(RewardsOwner);

impl design::results::Kept for RewardsMark {
    type Owner = RewardsOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // Its block no longer stands as the act left it, or its Undo was done (`Rewards::see`).
        owner_changed: design::results::OwnerChanged::Ends,
        // The note's own words end it, when the draft reads them.
        disk_changed: design::results::DiskChanged::Stays,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> RewardsOwner {
        self.0.clone()
    }
}

impl Rewards {
    /// Read the selected note again when its draft changed; a result whose block no longer
    /// stands as the act left it ends.
    pub(super) fn see(
        &mut self,
        draft: Option<&crate::sessions::Draft>,
        session: Option<Uuid>,
        book: &Book,
    ) {
        let (Some(draft), Some(note)) = (draft, session) else {
            self.seen = None;
            return;
        };
        if self
            .seen
            .as_ref()
            .is_some_and(|seen| seen.revision == draft.revision())
        {
            return;
        }
        let fights = played_fights(book);
        let mut ended = Vec::new();
        self.done.retain(|(id, event), done| {
            let holds = *id != note
                || crate::sessions::remove_rewards(&draft.body, &done.written).is_some();
            if !holds {
                ended.push(RewardsOwner {
                    note: *id,
                    event: event.clone(),
                });
            }
            holds
        });
        for owner in &ended {
            self.marks.owner_changed(owner);
        }
        self.seen = Some(RewardsSeen {
            revision: draft.revision(),
            listed: crate::sessions::listed_references(&draft.body),
            tonight: crate::sessions::xp_in_note(&draft.body, &|id| fights.contains(id)),
        });
    }

    pub(super) fn listed(&self, event: &str) -> bool {
        self.seen
            .as_ref()
            .is_some_and(|seen| seen.listed.contains(event))
    }

    /// The note's rewards results and its fault, in the plan's message footer, each naming its
    /// fight by the name the act read. Results with their Undo go newest first among the
    /// footer's results; the fault is a notice until the note changes.
    fn in_footer<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, super::SessionAct>,
        note: Uuid,
        revision: u64,
    ) {
        if let Some((id, _, words, met, severity)) = &self.fault
            && (*id, *met) == (note, revision)
        {
            messages.outcome(FaultMessage::new(&PlanFault::Toned(
                as_sentence(words),
                *severity,
            )));
        }
        // One walk of the store, each entry's data found by its key: no lookup of the store per
        // result (review of PR 658, P2).
        for shown in self.marks.newest_first(&()) {
            let owner = &shown.value().0;
            if owner.note != note {
                continue;
            }
            let event = &owner.event;
            let Some(done) = self.done.get(&(note, event.clone())) else {
                continue;
            };
            let fight = &done.fight;
            let mut words = format!(
                "Rewards for “{fight}” added under Loot and rewards. {}",
                done.choice
            );
            if done.with_xp
                && let Some(RewardsSeen {
                    tonight: (xp, fights @ 1..),
                    ..
                }) = &self.seen
            {
                words.push_str(&format!(
                    " XP tonight: {} for {fights} {}.",
                    crate::encounter_run::grouped_number(*xp),
                    if *fights == 1 { "fight" } else { "fights" }
                ));
            }
            match done.not_rolled {
                0 => {}
                1 => words.push_str(" 1 loot roll was not made."),
                n => words.push_str(&format!(" {n} loot rolls were not made.")),
            }
            match done.unknown {
                0 => {}
                1 => words.push_str(" 1 award is not known."),
                n => words.push_str(&format!(" {n} awards are not known.")),
            }
            let undo = pressed(FooterAct::Rewards(RewardsAct::Undo(event.clone())));
            messages.kept(shown.outcome(frame, |_| {
                Said::new(as_sentence(&words))
                    .undo(UndoStep::of(Verb::Add, "rewards").pressed_as(undo))
                    .into()
            }));
        }
    }
}

/// A fight's rewards block: its reference, then under it the XP line, when it has one, and
/// each loot line.
fn rewards_block(reference: &str, xp: Option<&str>, loot: &[String]) -> String {
    let mut block = format!("- {reference}\n");
    for line in xp.into_iter().chain(loot.iter().map(String::as_str)) {
        block.push_str(&format!("  - {line}\n"));
    }
    block
}

/// The words of an `Add rewards` result for its XP choice, from the newest block before it.
fn rewards_choice(with_xp: Option<bool>, xp: u64) -> String {
    match with_xp {
        Some(true) => format!(
            "XP {}, as in your last rewards.",
            crate::encounter_run::grouped_number(xp)
        ),
        Some(false) => "No XP, as in your last rewards.".to_owned(),
        None => "XP line added. Delete it in the note to leave XP out from now on.".to_owned(),
    }
}

/// The Timeline IDs of the fights that were played: a reference to another event is not a
/// rewards block.
fn played_fights(book: &Book) -> std::collections::BTreeSet<String> {
    book.events
        .iter()
        .filter(|event| event.run.is_some())
        .map(|event| event.id.clone())
        .collect()
}

impl Sessions {
    /// `Add rewards` or `Undo rewards`, through the note's one act path: it follows the disk,
    /// patches one block and saves at once. The run, the earlier notes and the Timeline are read
    /// now. What it could not do shows under the fight's row.
    /// The footer's `Undo add rewards` by its line's `ticket`: a result replaced or ended since
    /// is gone, and nothing changes; a refused Undo keeps its result and its line.
    fn undo_rewards(
        &mut self,
        ticket: design::results::Ticket,
        event: String,
        book: &Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
    ) {
        let Some(note) = session else {
            return;
        };
        let mut marks = std::mem::take(&mut self.rewards.marks);
        let _ = marks.undo_by(ticket, |_| {
            self.rewards_act(RewardsAct::Undo(event.clone()), book, session, plan);
            if self.rewards.done.contains_key(&(note, event.clone())) {
                Err(())
            } else {
                Ok(())
            }
        });
        self.rewards.marks = marks;
    }

    fn rewards_act(
        &mut self,
        act: RewardsAct,
        book: &Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
    ) {
        let (Some(note), Some(draft)) = (session, &mut self.draft) else {
            return;
        };
        let rewards = &mut self.rewards;
        rewards.fault = None;
        // The fight's name, read once here: the result and the fault keep it.
        let fight = match &act {
            RewardsAct::Undo(event) => rewards
                .done
                .get(&(note, event.clone()))
                .map(|done| done.fight.clone())
                .unwrap_or_default(),
            RewardsAct::Add(event, _) => fight_name(book, event),
        };
        let fault = |words: &str| format!("Rewards for “{fight}” not added. {words}");
        let (event, result) = match act {
            RewardsAct::Undo(event) => {
                let Some(done) = rewards.done.get(&(note, event.clone())) else {
                    return;
                };
                let written = done.written.clone();
                let result = draft.apply(note, Apply::Now, |text| {
                    crate::sessions::remove_rewards(text, &written)
                });
                if matches!(result, Ok(_) | Err(ActFault::NotSaved(..))) {
                    rewards.done.remove(&(note, event.clone()));
                }
                (event, result.map(drop))
            }
            RewardsAct::Add(event, run) => {
                let Some(at) = book.events.iter().position(|one| one.id == event) else {
                    return;
                };
                let reference =
                    timeline::reference(&book.events[at], timeline::day_numbers(&book.events)[at]);
                // R41 (round 30): a run folder's cause says itself (missing, unreadable);
                // `REWARDS_BLOCKED` is for the run's own file.
                let read = match (&plan.run_directory, &self.campaign) {
                    (Some(directory), Some(campaign)) => {
                        crate::encounter_run::find_run_typed(directory, campaign.id, run)
                            .map_err(|fault| match fault {
                                crate::encounter_run::FindRunFault::Folder(folder) => {
                                    (folder.message(), folder.severity())
                                }
                                crate::encounter_run::FindRunFault::File(_) => {
                                    (REWARDS_BLOCKED.to_owned(), Severity::NeedsFixing)
                                }
                            })
                            .and_then(|path| {
                                splat_session_records::rewards::rewards(&path, campaign.id).map_err(
                                    |_| (REWARDS_BLOCKED.to_owned(), Severity::NeedsFixing),
                                )
                            })
                    }
                    _ => Err((REWARDS_BLOCKED.to_owned(), Severity::NeedsFixing)),
                };
                let found = match read {
                    Ok(found) => found,
                    Err((why, severity)) => {
                        rewards.fault =
                            Some((note, event, fault(&why), draft.revision(), severity));
                        return;
                    }
                };
                let earlier = self.folder.as_deref().map_or_else(
                    || vec![None],
                    |folder| crate::sessions::earlier_notes(folder, note),
                );
                let fights = played_fights(book);
                let (xp, xp_sum, unknown) = found.xp_line();
                let (mut made, mut refused) = (None, None);
                let result = draft.apply(note, Apply::Now, |text| {
                    // Put there on disk a moment ago: the row then says `In notes`.
                    if crate::sessions::listed_references(text).contains(&event) {
                        return None;
                    }
                    let notes =
                        std::iter::once(Some(text)).chain(earlier.iter().map(Option::as_deref));
                    let with_xp =
                        crate::sessions::rewards_with_xp(notes, &|id| fights.contains(id));
                    let xp_shown = (with_xp != Some(false)).then_some(xp.as_str());
                    let block = rewards_block(&reference, xp_shown, &found.loot);
                    let (after, written, at) =
                        match crate::sessions::add_rewards(text, &event, &block) {
                            Ok(made) => made,
                            Err(why) => {
                                refused = Some(why);
                                return None;
                            }
                        };
                    // The note beside the plan shows the block's last line.
                    let shown = after[..at + block.len() - 1].chars().count();
                    made = Some((written, with_xp, shown));
                    Some(after)
                });
                if let (Ok(_) | Err(ActFault::NotSaved(..)), Some((written, with_xp, shown))) =
                    (&result, made)
                {
                    let has_xp = with_xp != Some(false);
                    rewards.done.insert(
                        (note, event.clone()),
                        RewardsDone {
                            written,
                            choice: rewards_choice(with_xp, xp_sum),
                            with_xp: has_xp,
                            not_rolled: found.not_rolled,
                            unknown: unknown * u64::from(has_xp),
                            fight: fight.clone(),
                        },
                    );
                    rewards.marks.record(RewardsMark(RewardsOwner {
                        note,
                        event: event.clone(),
                    }));
                    self.caret_to_show = Some(shown);
                }
                if let Some(why) = refused {
                    rewards.fault = Some((
                        note,
                        event,
                        fault(why),
                        draft.revision(),
                        Severity::NeedsFixing,
                    ));
                    return;
                }
                (event, result.map(drop))
            }
        };
        if let Err(failed) = result {
            rewards.fault = Some((
                note,
                event,
                fault(failed.words()),
                draft.revision(),
                Severity::NeedsFixing,
            ));
        }
    }
}

impl Sessions {
    /// The DM left the Session page: the journeys' Timeline results and their Undo end. The plan
    /// line's Undo is the note's, and stays while the note is as the act left it.
    pub fn left_page(&mut self) {
        self.travel.done.clear();
    }

    /// The open journey's start on the clock, worked out again only when the Timeline changed
    /// or another journey opened.
    /// Also the open event's place in the list, for `checks::settle_length`.
    pub(super) fn journey_start(
        &mut self,
        book: &Book,
        open: Option<&str>,
    ) -> (Option<usize>, Option<u64>) {
        let Some(open) = open else {
            return (None, None);
        };
        if let Some((changes, id, at, start)) = &self.travel.start
            && *changes == book.changes()
            && id == open
        {
            return (*at, *start);
        }
        let at = book.events.iter().position(|event| event.id == open);
        let start = at
            .filter(|at| checks::shown(&book.events[*at]))
            .and_then(|at| timeline::clock(&book.events)[at]);
        self.travel.start = Some((book.changes(), open.to_owned(), at, start));
        (at, start)
    }

    /// The mark of the plan line that a fight beginning now takes by the plan's rule (one played
    /// event of `events` takes one link of its name), read from the session note as the file
    /// holds it at this moment. `None` for a line with no mark, and for a note that cannot be
    /// read: then nothing is said about travel.
    pub fn road_mark(
        &self,
        session: Option<Uuid>,
        name: &str,
        events: &[timeline::Event],
    ) -> Option<checks::RoadMark> {
        let note = self.notes.iter().find(|note| Some(note.id) == session)?;
        let text = std::fs::read_to_string(&note.path).ok()?;
        let fight = plan_fight_stubs(&[name.to_owned()]);
        let (at, _, _) = plan_links(&text, events, session, &fight)
            .into_iter()
            .find(|(_, _, waiting)| *waiting)?;
        checks::marked(text[at..].lines().next().unwrap_or_default()).mark
    }

    /// F9, W5-D1, at `Begin combat`: where a fight that enters the Timeline now is played. One
    /// read of the files as they are on disk decides: `Timeline.md` gives the played fights,
    /// so the plan line this fight takes, and the journey and its check; the Timeline as Splat
    /// holds it must agree. `None` for a fight with no mark (an ordinary fight, no notice).
    pub fn road_at_begin(
        &self,
        book: &Book,
        session: Option<Uuid>,
        name: &str,
    ) -> Option<Result<(checks::RoadMark, timeline::Within), checks::OffRoad>> {
        let disk = book.on_disk();
        // With `Timeline.md` not read, the plan line is the one the Timeline in memory gives.
        let mark = self.road_mark(session, name, disk.as_deref().unwrap_or(&book.events))?;
        let placed = disk.ok_or(checks::OffRoad::NotRead).and_then(|disk| {
            mark.place(&disk, session)?;
            mark.place(&book.events, session)
        });
        Some(placed.map(|within| (mark, within)))
    }

    /// The fight just placed at `mark` is saved at once. After that save's merge with the file,
    /// the plan line it takes and its check are read again; when either is not the one the mark
    /// named, the fight goes after the session's last event, and the reason comes back.
    pub fn confirm_road(
        &self,
        book: &mut Book,
        session: Option<Uuid>,
        run: Uuid,
        name: &str,
        mark: &checks::RoadMark,
    ) -> Option<checks::OffRoad> {
        book.save();
        let others: Vec<_> = book
            .events
            .iter()
            .filter(|event| event.run != Some(run))
            .cloned()
            .collect();
        let why = match self.road_mark(session, name, &others) {
            Some(read) if read == *mark => mark.place(&book.events, session).err(),
            _ => Some(checks::OffRoad::Changed),
        };
        if why.is_some() {
            book.leave_journey(run);
        }
        why
    }

    /// Take back the plan line of `Plan “…”` (`ticket`: the entry its line showed), while its
    /// owner holds and the note is as that act left it: through the session's own draft when one
    /// is open for it, else by the note's own checked path on its file (R26: it never selects
    /// that session or touches another draft). A refused Undo keeps the entry with its refusal
    /// (`Results::undo_or_refuse`): both readers' lines then open with it.
    pub fn undo_plan_line(&mut self, ticket: design::results::Ticket) {
        self.reconcile_plan_owners();
        let Some((note, path, refused)) = self.travel.planned.get_by(&ticket).map(|shown| {
            (
                shown.value().note,
                shown.value().path.clone(),
                shown.refusal().is_some(),
            )
        }) else {
            return;
        };
        // R45: once refused, the line's Undo is not offered again; a press that still reaches
        // it does nothing.
        if refused {
            return;
        }
        let draft = (self.selected == Some(note))
            .then_some(self.draft.as_mut())
            .flatten();
        let own_draft = draft.is_none();
        let _ = self.travel.planned.undo_or_refuse(ticket, |planned| {
            // Final review point 1: a line whose own save failed and that a later save of its
            // note wrote is taken back from the file, by the checked path: the Undo learns that
            // from the note's draft (the open one, or one read now) before it runs.
            let mut undo = planned.undo.clone();
            let result = if let Some(draft) = draft {
                // A draft that waits for a move or a choice refuses through `undo` itself. R45:
                // when the draft holds the reversal and only its save failed, the note no longer
                // holds the line, so the refusal ends at once (`follow_plan_line`, below); the
                // failed save is the note's own fault, which its warning says.
                undo.follow_save(draft);
                undo.undo(draft)
            } else {
                // R45: only the exact file the act recorded (its path, its session's id and the
                // words the act left), through the note's checked path.
                match crate::sessions::Draft::open(&planned.path) {
                    Ok(mut other) => {
                        undo.follow_save(&other);
                        undo.undo(&mut other)
                    }
                    Err(_) => Err(ActFault::Missing),
                }
            };
            result.map_err(|fault| planned.refusal(&fault, planned.path.exists()))
        });
        self.travel.forget_look();
        if own_draft {
            // What the file holds now: a draft that could not save is dropped unpublished.
            self.read_note_again(note, &path);
        }
        self.follow_plan_line();
    }

    /// For tests: `look` at the newest plan line's entry in this campaign, of `note` when one is
    /// named.
    #[cfg(any(test, feature = "test-support"))]
    fn plan_entry<R>(
        &mut self,
        note: Option<Uuid>,
        look: impl FnOnce(design::results::Shown<'_, PlannedLine>) -> R,
    ) -> Option<R> {
        self.reconcile_plan_owners();
        let campaign = self.campaign.as_ref().map(|campaign| campaign.id);
        self.travel
            .planned
            .newest_first(&campaign)
            .find(|shown| note.is_none_or(|note| shown.value().note == note))
            .map(look)
    }

    /// A plan line's answer, for tests: the words the newest line (of `note`) opens with.
    #[cfg(any(test, feature = "test-support"))]
    pub fn plan_line_refusal_for(&mut self, note: Option<Uuid>) -> Option<String> {
        self.plan_entry(note, |shown| {
            shown.refusal().map(|refusal| refusal.words().to_string())
        })
        .flatten()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn plan_line_refusal(&mut self) -> Option<String> {
        self.plan_line_refusal_for(None)
    }

    /// The tone of a plan line's answer, for tests.
    #[cfg(test)]
    fn plan_line_tone(&mut self, note: Option<Uuid>) -> Option<Tone> {
        self.plan_entry(note, |shown| {
            shown.refusal().map(design::results::Refusal::tone)
        })
        .flatten()
    }

    /// Press the Undo of the newest plan line (of `note`), for tests, by the ticket of the
    /// entry its line shows; the refusal its line then opens with, if any.
    #[cfg(any(test, feature = "test-support"))]
    pub fn try_plan_undo_for(&mut self, note: Option<Uuid>) -> Option<String> {
        if let Some(ticket) = self.plan_entry(note, |shown| shown.ticket_for_caller_test()) {
            self.undo_plan_line(ticket);
        }
        self.plan_line_refusal_for(note)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn try_plan_undo(&mut self) -> Option<String> {
        self.try_plan_undo_for(None)
    }

    /// For tests: the fight of the newest plan line's result in this campaign.
    #[cfg(test)]
    fn planned_fight(&mut self) -> Option<String> {
        self.plan_entry(None, |shown| shown.value().chosen.name.clone())
    }

    /// Row 123, review 3, point 1: each plan line's result holds while its owner does: made in
    /// this campaign, for a session whose note is still in the folder at the file it wrote. Else
    /// the result ends here. A refused answer stays while its note is in the folder, wherever its
    /// file is (its Undo is never offered again, R45). It runs only when the campaign or the
    /// folder's listing changed (`notes_revision`), never on an unchanged frame.
    fn reconcile_plan_owners(&mut self) {
        let campaign = self.campaign.as_ref().map(|campaign| campaign.id);
        let key = (campaign, self.notes_revision);
        if self.travel.owners_seen == Some(key) {
            return;
        }
        self.travel.owners_seen = Some(key);
        if self.travel.planned.is_empty() {
            return;
        }
        #[cfg(any(test, feature = "test-support"))]
        PLAN_OWNER_SCANS.with(|scans| scans.set(scans.get() + 1));
        let notes = &self.notes;
        self.travel.planned.end_unreachable(|planned| {
            planned.campaign == campaign && notes.iter().any(|note| note.id == planned.note)
        });
        // R45: an Undo acts only on the file the act recorded. A result whose note is now at
        // another file ends; a refused answer is not ended by it.
        self.travel.planned.source_changed(|planned| {
            notes
                .iter()
                .find(|note| note.id == planned.note)
                .is_none_or(|note| note.path == planned.path)
        });
        self.travel.forget_look();
    }

    /// Row 123, review 3, point 2: the draft open for `note`, found by its session, wherever its
    /// file is now.
    fn session_draft(&mut self, note: Uuid) -> Option<&mut crate::sessions::Draft> {
        if self.selected == Some(note) {
            self.draft.as_mut()
        } else {
            None
        }
    }

    /// Row 123, review 3, point 3: read a note that no draft holds from its file again, for the
    /// offer and the mentions. Words a dropped draft held are never taken for saved ones.
    fn read_note_again(&mut self, note: Uuid, path: &Path) {
        let body = std::fs::read_to_string(path)
            .ok()
            .map(|text| crate::sessions::body_of(&text).to_owned());
        if let Some(body) = &body {
            self.mentions.insert(note, timeline::referenced_ids(body));
        }
        self.see_words(note, path, body);
    }

    /// Row 123, point 2: where `note`'s words come from now. A note the open draft holds is the
    /// draft's; any other is read once at each disk check (`poll`, every `CHECK_DISK_EVERY`),
    /// and its generation changes only when its words do, whatever the file's time says.
    fn note_source(&mut self, note: Uuid, path: &Path) -> NoteSource {
        if let Some(draft) = self.session_draft(note) {
            return NoteSource::Draft(draft.revision());
        }
        let campaign = self.campaign.as_ref().map(|campaign| campaign.id);
        let tick = self.last_check;
        let fresh = self.travel.watched.get(&note).is_some_and(|watched| {
            watched.campaign == campaign && watched.path == path && watched.tick == tick
        });
        if !fresh {
            let body = std::fs::read_to_string(path)
                .ok()
                .map(|text| crate::sessions::body_of(&text).to_owned());
            self.see_words(note, path, body);
        }
        if let Some(watched) = self.travel.watched.get_mut(&note) {
            watched.tick = tick;
            return NoteSource::Disk(watched.generation);
        }
        NoteSource::Disk(0)
    }

    fn see_words(&mut self, note: Uuid, path: &Path, body: Option<String>) {
        let campaign = self.campaign.as_ref().map(|campaign| campaign.id);
        let same = self.travel.watched.get(&note).is_some_and(|watched| {
            watched.campaign == campaign && watched.path == path && watched.body == body
        });
        if same {
            return;
        }
        self.travel.generation += 1;
        self.travel.forget_look();
        let tick = self.last_check;
        self.travel.watched.insert(
            note,
            Watched {
                campaign,
                path: path.to_owned(),
                body,
                generation: self.travel.generation,
                tick,
            },
        );
    }

    /// Final review point 2: whether `session`'s plan shows the plan line's result: only the
    /// plan of the note that holds the line, wherever its journey is now, while it holds.
    /// Whether the plan line's result still holds, for tests.
    #[cfg(test)]
    fn planned_live(&mut self) -> bool {
        self.follow_plan_line();
        self.plan_entry(None, |_| ()).is_some()
    }

    #[cfg(test)]
    fn plan_line_shown_in(&mut self, session: Option<Uuid>) -> bool {
        self.follow_plan_line();
        session.is_some_and(|note| self.plan_entry(Some(note), |_| ()).is_some())
    }

    /// Row 123, final review: the one place where the note's own events act on the plan line's
    /// result. `poll` calls it every frame on every page, so an edit, a save with other words, an
    /// outside change seen by the disk check, a discard or a move is met as it happens, with no
    /// view of the result drawn. At each change of the note's source (its draft's revision, or
    /// its disk generation), the Undo is live only when the note holds exactly the act's words,
    /// saved: the act's own write, or a later save of that same write. A result whose Undo is not
    /// live ends for good (reopening a view never brings it back), unless its Undo was refused:
    /// that answer stays, with its Undo waiting, until its own events end it (the refused
    /// attempt's life is not its Undo's). A quiet frame compares no words.
    pub(super) fn follow_plan_line(&mut self) {
        self.reconcile_plan_owners();
        let campaign = self.campaign.as_ref().map(|campaign| campaign.id);
        // Nothing to look at unless the store, the selection, the open draft's words or the disk
        // check changed since the last look: an unchanged frame walks no entry.
        let key = (
            campaign,
            self.selected,
            self.draft
                .as_ref()
                .map(|draft| (draft.revision(), draft.unsaved())),
            self.last_check,
        );
        if self.travel.followed == Some(key) {
            return;
        }
        #[cfg(any(test, feature = "test-support"))]
        PLAN_FOLLOW_WALKS.with(|walks| walks.set(walks.get() + 1));
        // A refused answer is watched at its note's file now (R45: it stays through a move); a
        // result at the file its act recorded.
        let notes = &self.notes;
        let held: Vec<(Uuid, std::path::PathBuf, NoteSource)> = self
            .travel
            .planned
            .newest_first(&campaign)
            .map(|shown| {
                let planned = shown.value();
                let path = if shown.refusal().is_some() {
                    notes
                        .iter()
                        .find(|note| note.id == planned.note)
                        .map_or_else(|| planned.path.clone(), |note| note.path.clone())
                } else {
                    planned.path.clone()
                };
                (planned.note, path, planned.seen)
            })
            .collect();
        let mut sources: Vec<(Uuid, NoteSource)> = Vec::with_capacity(held.len());
        let mut changed = false;
        for (note, path, seen) in held {
            let mut source = self.note_source(note, &path);
            // The draft that held the note closed: what its last save left is read from the
            // file now.
            if matches!((seen, source), (NoteSource::Draft(_), NoteSource::Disk(_))) {
                self.read_note_again(note, &path);
                source = self.note_source(note, &path);
            }
            changed |= source != seen;
            sources.push((note, source));
        }
        // Reading a note above can change a watched generation, which forgets the last look:
        // keep this look only now.
        self.travel.followed = Some(key);
        if !changed {
            return;
        }
        #[cfg(any(test, feature = "test-support"))]
        timeline::page::PLAN_LINE_CHECKS.with(|checks| checks.set(checks.get() + 1));
        let selected = self.selected;
        let draft = self.draft.as_ref();
        let watched = &self.travel.watched;
        let source_of = |note: Uuid| {
            sources
                .iter()
                .find(|(known, _)| *known == note)
                .map(|(_, source)| *source)
        };
        // The note's words as its source has them now: the selected draft's, or the file's.
        let body_of = |note: Uuid| match source_of(note) {
            Some(NoteSource::Draft(_)) => draft
                .filter(|_| selected == Some(note))
                .map(|draft| draft.body.as_str()),
            Some(NoteSource::Disk(_)) => watched
                .get(&note)
                .and_then(|watched| watched.body.as_deref()),
            None => None,
        };
        // R45: a refused answer ends when its note's words changed and no longer hold its line
        // (the DM removed it by hand, as the answer asks). Words that cannot be read end nothing.
        self.travel.planned.end_refusals(
            |planned| {
                source_of(planned.note) != Some(planned.seen)
                    && body_of(planned.note).is_some_and(|body| !body.contains(planned.line.trim()))
            },
            |_| false,
        );
        self.travel.planned.source_changed(|planned| {
            let Some(source) = source_of(planned.note) else {
                return true;
            };
            if source == planned.seen {
                return true;
            }
            let holds = match source {
                NoteSource::Draft(_) => draft.is_some_and(|draft| {
                    selected == Some(planned.note) && !draft.unsaved() && planned.undo.live(draft)
                }),
                NoteSource::Disk(_) => watched
                    .get(&planned.note)
                    .is_some_and(|watched| watched.body.as_deref() == Some(planned.undo.after())),
            };
            planned.seen = source;
            holds
        });
    }

    /// Row 123: a refused plan Undo ends with the journey it was for. It reads the Timeline only
    /// when it changed (its revision), never on an unchanged frame.
    pub(super) fn end_plan_answers_of_lost_journeys(&mut self, book: &Book) {
        let revision = book.revision();
        if self.travel.journeys_seen == Some(revision) {
            return;
        }
        self.travel.journeys_seen = Some(revision);
        if self.travel.planned.is_empty() {
            return;
        }
        #[cfg(any(test, feature = "test-support"))]
        PLAN_OWNER_SCANS.with(|scans| scans.set(scans.get() + 1));
        if self.travel.planned.end_refusals(
            |planned| !book.events.iter().any(|event| event.id == planned.journey),
            // R45: a refused line's Undo never comes back; its result ends with its answer.
            |_| false,
        ) > 0
        {
            self.travel.forget_look();
        }
    }

    /// Row 123: an act on the plan line of `note` and `journey` ends that line's refused Undo,
    /// whatever the act does; another line's answer stays.
    fn end_plan_answer_of(&mut self, note: Uuid, journey: &str) {
        if self.travel.planned.end_refusals(
            |planned| planned.note == note && planned.journey == journey,
            // R45: a refused line's Undo never comes back; its result ends with its answer.
            |_| false,
        ) > 0
        {
            self.travel.forget_look();
        }
    }

    /// Row 123: Timeline's view of the plan of `journey`'s session, for its check results'
    /// offers. Read again only when the campaign, the session's note or its words, the Timeline
    /// or the read of the names changed. A journey with no session, or whose note is gone,
    /// offers nothing.
    pub fn journey_plan(
        &mut self,
        book: &Book,
        journey: &str,
        names: &PlanNames,
    ) -> Arc<JourneyPlan> {
        let session = book
            .events
            .iter()
            .find(|event| event.id == journey)
            .and_then(|event| event.session);
        let Some((note, path)) = session.and_then(|id| {
            self.notes
                .iter()
                .find(|note| note.id == id)
                .map(|note| (note.id, note.path.clone()))
        }) else {
            return Arc::default();
        };
        let source = self.note_source(note, &path);
        let key = (
            self.campaign.as_ref().map(|campaign| campaign.id),
            note,
            path,
            source,
            book.changes(),
            names.revision,
        );
        if let Some((known, plan)) = &self.travel.journey_plan
            && *known == key
        {
            return Arc::clone(plan);
        }
        let body = match source {
            NoteSource::Draft(_) => self.draft.as_ref().map(|draft| draft.body.as_str()),
            NoteSource::Disk(_) => self
                .travel
                .watched
                .get(&note)
                .and_then(|watched| watched.body.as_deref()),
        };
        let plan = Arc::new(body.map_or_else(JourneyPlan::default, |body| {
            JourneyPlan::read(body, &book.events, Some(note), &names.fights, &names.groups)
        }));
        self.travel.journey_plan = Some((key, Arc::clone(&plan)));
        plan
    }

    /// Row 123, R26: `Plan “<name>”` pressed on Timeline. It never changes the Sessions page's
    /// selection, open draft, scene results or open event. The journey's session open there: the
    /// line goes through its draft, as the plan's own act. Another session: the act writes that
    /// note itself, through the note's checked path on a draft of its own, and its result and
    /// Undo stay bound to that note. The fault, when there is one, is for Timeline's block.
    pub fn plan_from_timeline(
        &mut self,
        book: &Book,
        journey: &str,
        chosen: &checks::Planned,
        names: &PlanNames,
    ) -> Result<(), String> {
        let session = book
            .events
            .iter()
            .find(|event| event.id == journey)
            .and_then(|event| event.session);
        let Some((note, path, number)) = session.and_then(|id| {
            self.notes
                .iter()
                .find(|note| note.id == id)
                .map(|note| (note.id, note.path.clone(), note.number))
        }) else {
            return Err(format!("{NOT_ADDED} This journey’s session note is gone."));
        };
        // An act on this note's plan line for this journey ends that line's refused Undo,
        // whatever it does; another line's answer stays.
        self.end_plan_answer_of(note, journey);
        let fights = plan_fight_stubs(&names.fights);
        // The session's own draft, found by its session wherever its file went: it writes,
        // unless it waits for a move or a choice; then nothing else writes the note.
        if let Some(draft) = self.session_draft(note) {
            if draft.path != path || draft.needs_attention() {
                return Err(format!(
                    "{NOT_ADDED} Session {number}’s note needs a choice first. Open it in Sessions."
                ));
            }
            return self
                .plan_line_act(book, Some(note), journey, chosen, &fights, &names.groups)
                .map_err(|words| format!("{NOT_ADDED} {words}"));
        }
        let mut other = crate::sessions::Draft::open(&path)
            .map_err(|_| format!("{NOT_ADDED} {}", ActFault::Missing.words()))?;
        let written = write_plan_line(
            &mut other,
            note,
            book,
            journey,
            chosen,
            &fights,
            &names.groups,
        );
        // What the file holds now; the one-act draft is dropped, saved or not.
        self.read_note_again(note, &path);
        match written {
            Written::Already => Ok(()),
            Written::Done(Ok(undo)) => {
                self.keep_plan_line(book, journey, chosen, note, path, undo);
                Ok(())
            }
            // A draft of its own that could not save holds nothing for an Undo.
            Written::Done(Err(fault)) => Err(format!("{NOT_ADDED} {}", fault.words())),
        }
    }

    /// Row 123: each `Plan “<name>”`'s result and `Undo add plan line` as footer lines of this
    /// frame, from the one store both readers read (`Travel::planned`): the Session plan shows
    /// the lines of the note it shows (`in_session`), Timeline's list every line of the campaign
    /// (`None`). While a line's Undo is refused, it opens with the refusal, in the tone of its
    /// cause, and its Undo shows only while the note is as the act left it. Two lines tell their
    /// Undos apart by their fights.
    fn plan_line_kept<'f, A: Clone>(
        &mut self,
        frame: &'f PaneFrame,
        in_session: Option<Option<Uuid>>,
        undo: A,
    ) -> Vec<KeptLine<'f, A>> {
        self.follow_plan_line();
        let campaign = self.campaign.as_ref().map(|campaign| campaign.id);
        let lines: Vec<_> = self
            .travel
            .planned
            .newest_first(&campaign)
            .filter(|shown| in_session.is_none_or(|session| session == Some(shown.value().note)))
            .collect();
        let several = lines.len() > 1;
        lines
            .into_iter()
            .map(|shown| {
                // R45: a refused line offers no Undo again.
                let refused = shown.refusal().is_some();
                let apart = several
                    .then(|| shown.value().apart.as_deref())
                    .flatten()
                    .and_then(design::results::Name::new);
                shown.outcome_told_apart(frame, apart.as_ref(), |planned| {
                    let (words, step) = plan_line_words(planned, several);
                    let said = Said::new(words);
                    if refused {
                        said.into()
                    } else {
                        said.undo(step.pressed_as(undo.clone())).into()
                    }
                })
            })
            .collect()
    }

    /// Timeline's footer lines of the plan lines' results (`plan_line_kept`).
    pub fn plan_line_lines<'f, A: Clone>(
        &mut self,
        frame: &'f PaneFrame,
        undo: A,
    ) -> Vec<KeptLine<'f, A>> {
        self.plan_line_kept(frame, None, undo)
    }

    /// A plan line written: its result, with its order and its Undo, which replaces an earlier
    /// one of the same note and journey. The session's label is taken now, for its answers, and
    /// what tells it apart from a line that reads alike (the same fight in a session of the same
    /// label): its journey's label, then a number past the others' (`PlannedLine::apart`). It is
    /// made here, once, from the other entries, never from a line's place in a list.
    fn keep_plan_line(
        &mut self,
        book: &Book,
        journey: &str,
        chosen: &checks::Planned,
        note: Uuid,
        path: std::path::PathBuf,
        undo: NoteUndo,
    ) {
        let journey_label = journey_label(book, journey);
        let line = road_plan_line(book, journey, chosen);
        let seen = self.note_source(note, &path);
        let session = self
            .notes
            .iter()
            .find(|known| known.id == note)
            .map(crate::sessions::Note::label)
            .unwrap_or_default();
        let campaign = self.campaign.as_ref().map(|campaign| campaign.id);
        let alike: Vec<(&str, u32, Option<&str>)> = self
            .travel
            .planned
            .newest_first(&campaign)
            .map(|shown| shown.value())
            .filter(|other| {
                !(other.note == note && other.journey == journey)
                    && other.chosen.name == chosen.name
                    && other.session == session
            })
            .map(|other| {
                (
                    other.journey_label.as_str(),
                    other.ordinal,
                    other.apart.as_deref(),
                )
            })
            .collect();
        let (apart, ordinal) = told_apart(&journey_label, &alike);
        self.travel.forget_look();
        self.travel.planned.record(PlannedLine {
            seen,
            campaign,
            journey: journey.to_owned(),
            chosen: chosen.clone(),
            note,
            session,
            apart,
            ordinal,
            journey_label,
            line,
            path,
            undo,
        });
    }

    /// `Plan “<name>”`: write its line into the selected session's note, before the first fight
    /// still to come. A fight already to come there, in the words as they are now, writes nothing
    /// and keeps the earlier Undo. The result and its Undo go to `Travel::planned`; the fault
    /// comes back.
    fn plan_line_act(
        &mut self,
        book: &Book,
        session: Option<Uuid>,
        journey: &str,
        chosen: &checks::Planned,
        fights: &[PlanFight],
        groups: &[String],
    ) -> Result<(), String> {
        let (Some(draft), Some(note)) = (&mut self.draft, session) else {
            return Ok(());
        };
        let path = draft.path.clone();
        let written = write_plan_line(draft, note, book, journey, chosen, fights, groups);
        self.mentions
            .insert(note, timeline::referenced_ids(&draft.body));
        match written {
            Written::Already => Ok(()),
            Written::Done(Ok(undo)) => {
                self.keep_plan_line(book, journey, chosen, note, path, undo);
                Ok(())
            }
            Written::Done(Err(fault)) => {
                let words = fault.words().to_owned();
                if let ActFault::NotSaved(_, undo) = fault {
                    self.keep_plan_line(book, journey, chosen, note, path, undo);
                }
                Err(words)
            }
        }
    }

    /// Carry out an act on a journey's checks. `Plan` writes the session note; the others write
    /// `Timeline.md`.
    pub(super) fn travel_act(
        &mut self,
        book: &mut Book,
        session: Option<Uuid>,
        id: &str,
        act: checks::Act,
        plan: &Plan<'_>,
    ) {
        self.travel.block.clear_problem();
        if act == checks::Act::Start {
            // The row opens on its checks, with focus where the length goes.
            self.open_event = Some(id.to_owned());
            self.reveal_event = true;
            self.travel.focus_length = book
                .events
                .iter()
                .any(|event| event.id == id && event.duration.trim().is_empty());
        }
        if let checks::Act::Plan(chosen) = &act {
            // An act on this note's plan line for this journey ends that line's refused Undo,
            // whatever it does; another line's answer stays.
            if let Some(note) = session {
                self.end_plan_answer_of(note, id);
            }
            let groups: Vec<String> = plan.groups.iter().map(|group| group.name.clone()).collect();
            if let Err(words) = self.plan_line_act(book, session, id, chosen, plan.fights, &groups)
            {
                self.travel.block.failed(&act, words);
            }
            return;
        }
        match checks::apply(book, id, &act, plan.tables) {
            Ok(Some(done)) => self.travel.done.keep(done),
            Ok(None) => {}
            Err(error) => self.travel.block.failed(&act, error),
        }
    }

    /// The selected session's ID: where an encounter editor opened from the plan goes back to.
    pub fn selected_id(&self) -> Option<Uuid> {
        self.selected
    }

    /// A planned fight's `Edit encounter` that opened nothing: why, on the page where the DM acted.
    pub fn encounter_not_opened(&mut self, why: String) {
        self.say(Tone::Warning, why, None);
    }

    /// Lead ruling, round 26 (one pane, one footer): a plan group's removal result and its Undo
    /// sit in the plan's message footer, not in the removed row's place.
    /// `Undo remove` of a group removed from tonight: the note's words before the removal.
    fn undo_group_remove(&mut self, _session: Option<Uuid>) {
        if let Some((_, _, before, _, _, _)) = self.removed_group.take()
            && let Some(draft) = &mut self.draft
        {
            let key = draft.body.key();
            let _ = draft.body.replace(before);
            draft.own_write(key);
            draft.edited();
        }
    }

    fn removed_group_message<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, super::SessionAct>,
        session: Option<Uuid>,
    ) {
        let Some((id, _, _, _, _, name)) = &self.removed_group else {
            return;
        };
        if *id != session {
            return;
        }
        self.plan_line(frame, messages, PlanKind::GroupRemoved, || {
            Said::new(crate::sentence!("Removed “{}” from tonight.", name))
                .undo(
                    UndoStep::of(Verb::Remove, name.as_str())
                        .pressed_as(pressed(FooterAct::UndoGroupRemove)),
                )
                .into()
        });
    }

    /// The owner of the plan's result of `kind` in this campaign's session folder.
    fn plan_owner(folder: Option<&std::path::Path>, kind: PlanKind) -> kept::PlanOwner {
        kept::PlanOwner {
            root: folder.map(std::path::Path::to_path_buf).unwrap_or_default(),
            kind,
        }
    }

    /// A plan result of `kind` was just done: the plan's store issues its order now.
    fn mark_plan(
        marks: &mut design::Results<kept::PlanOwner, kept::PlanMark>,
        folder: Option<&std::path::Path>,
        kind: PlanKind,
    ) {
        marks.record(kept::PlanMark(Self::plan_owner(folder, kind)));
    }

    /// Each plan result whose data is gone (its Undo, the note's next edit, another act of its
    /// kind, a campaign switch) ends in the plan's store, before the footer is made.
    fn end_plan_marks(&mut self) {
        let held = [
            (PlanKind::SceneAdded, self.scenes.added.is_some()),
            (PlanKind::SceneRemoved, self.scenes.removed.is_some()),
            (PlanKind::GroupRemoved, self.removed_group.is_some()),
            (PlanKind::DeletedRun, self.deleted_run.is_some()),
        ];
        for (kind, held) in held {
            if !held {
                let owner = Self::plan_owner(self.folder.as_deref(), kind);
                self.plan_marks.owner_changed(&owner);
            }
        }
    }

    /// This frame's footer line of the plan's result of `kind`, made from its data by `make`,
    /// with the order its act's store issued and that entry's ticket.
    fn plan_line<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, super::SessionAct>,
        kind: PlanKind,
        make: impl FnOnce() -> KeptOutcome<super::SessionAct>,
    ) {
        let owner = Self::plan_owner(self.folder.as_deref(), kind);
        if let Some(shown) = self.plan_marks.get(&owner) {
            messages.kept(shown.outcome(frame, |_| make()));
        }
    }

    /// The Undo of the plan's result its footer line's `ticket` names, by `undo`, which says
    /// whether it was done: a result replaced or ended since is gone, and nothing runs.
    fn undo_plan_mark(
        &mut self,
        ticket: Option<design::results::Ticket>,
        undo: impl FnOnce(&mut Self) -> bool,
    ) {
        let Some(ticket) = ticket else {
            return;
        };
        let mut marks = std::mem::take(&mut self.plan_marks);
        let _ = marks.undo_by(ticket, |_| if undo(self) { Ok(()) } else { Err(()) });
        self.plan_marks = marks;
    }

    /// An event the plan's Add strip just added: its title is its kind's word, and its result
    /// (`kept::PlanAdded`) keeps one step of Undo (both UIs).
    pub(super) fn record_plan_add(&mut self, id: &str, book: &mut Book, ctx: &egui::Context) {
        let session = self.selected;
        if let Some(event) = book.events.iter_mut().find(|event| event.id == *id) {
            event.title = event.kind.word().to_owned();
        }
        let days = timeline::day_numbers(&book.events);
        let index = book.events.iter().position(|event| event.id == *id);
        let added = kept::PlanAdded {
            owner: self.of_session(session),
            id: id.to_owned(),
            label: index.map_or_else(String::new, |at| book.events[at].label(days[at])),
            to: self.session_name(session),
            count: book.events.len(),
        };
        self.added.record_timed(added, ctx);
        self.open_event = None;
    }

    /// A press in the plan's footer. Returns the scene's and the rewards' acts (with the
    /// press's ticket), which the caller does after the rows (both UIs).
    pub(super) fn plan_footer_press(
        &mut self,
        press: design::outcome::Press<super::SessionAct>,
        book: &mut Book,
        plan: &Plan<'_>,
    ) -> (
        Option<SceneAct>,
        Option<(RewardsAct, Option<design::results::Ticket>)>,
    ) {
        let session = self.selected;
        let mut scene_act = None;
        let mut rewards = None;
        match press.id {
            super::SessionAct::Plan(act) => match act {
                FooterAct::GoToGroup => self.select(self.group_session),
                FooterAct::UndoGroupDiscard => self.wants_group_undo = press.ticket,
                FooterAct::UndoGroupRemove => self.undo_plan_mark(press.ticket, |this| {
                    this.undo_group_remove(session);
                    this.removed_group.is_none()
                }),
                FooterAct::UndoDeletedRun => self.undo_plan_mark(press.ticket, |this| {
                    this.undo_deleted_run();
                    this.deleted_run.is_none()
                }),
                FooterAct::UndoAdd => {
                    if let Some(ticket) = press.ticket
                        && let Ok((added, ())) = self.added.undo_by(ticket, |_| Ok::<(), ()>(()))
                    {
                        book.undo_add(&added.id);
                    }
                }
                FooterAct::UndoRemove => {
                    if let Some(ticket) = press.ticket {
                        book.undo_remove(ticket);
                        self.removed_event_in_plan = false;
                        self.plan_notice_state = None;
                    }
                }
                FooterAct::UndoRuling => {
                    if let Some(ticket) = press.ticket {
                        book.undo_restores(ticket);
                        self.end_plan_add();
                    }
                }
                // A scene's footer Undo reaches only the result its line showed; the act
                // then takes the scene's data, whatever its save does.
                FooterAct::Scene(act @ (SceneAct::UndoAdd | SceneAct::UndoRemove(_))) => {
                    let mut reached = false;
                    self.undo_plan_mark(press.ticket, |_| {
                        reached = true;
                        true
                    });
                    if reached {
                        scene_act = Some(act);
                    }
                }
                FooterAct::Scene(act) => scene_act = Some(act),
                FooterAct::Rewards(act) => rewards = Some((act, press.ticket)),
                FooterAct::UndoPlanLine => {
                    if let Some(ticket) = press.ticket {
                        self.undo_plan_line(ticket);
                    }
                }
                FooterAct::File(act) => file_act(book, act, press.ticket),
            },
            _ => self.session_press(press, book, plan),
        }
        (scene_act, rewards)
    }

    /// `Remove from tonight` of the planned fight at `position`: its link leaves the note, with
    /// the removal's Undo (both UIs).
    pub(super) fn remove_planned_fight(
        &mut self,
        (position, name): (usize, String),
        book: &Book,
        plan: &Plan<'_>,
        planned: &[usize],
    ) {
        let session = self.selected;
        // A draft is open only for a selected session, so the act has its session.
        if let Some(draft) = &mut self.draft
            && let Some(note) = session
        {
            let occurrence = book
                .events
                .iter()
                .filter(|event| {
                    event.is_in(session)
                        && event.run.is_some()
                        && event.title.eq_ignore_ascii_case(&name)
                })
                .count()
                + planned[..position]
                    .iter()
                    .filter(|&&at| plan.fights[at].name.eq_ignore_ascii_case(&name))
                    .count();
            // A fight that is not in the words changes nothing, and says nothing, as before.
            if let Ok(undo) = draft.apply(note, Apply::WithTyping, |words| {
                let after = unlink_planned_fight(words, &name, occurrence);
                (after != words).then_some(after)
            }) {
                self.plan_notice_state = Some((undo.after().to_owned(), book.events.clone()));
                self.removed_fight = Some((session, position, name, undo));
            }
        }
    }

    /// `Remove from tonight` of a planned group: its link leaves the note (both UIs).
    pub(super) fn remove_planned_group(
        &mut self,
        (position, name, height): (usize, String, f32),
        book: &Book,
    ) {
        let session = self.selected;
        if let Some(draft) = &mut self.draft {
            let before = draft.body.as_str().to_owned();
            let key = draft.body.key();
            draft.body.edit(|body| unlink_group(body, &name));
            draft.own_write(key);
            draft.edited();
            Self::mark_plan(
                &mut self.plan_marks,
                self.folder.as_deref(),
                PlanKind::GroupRemoved,
            );
            self.removed_group = Some((
                session,
                position,
                before,
                draft.body.as_str().to_owned(),
                height,
                name,
            ));
            self.plan_notice_state = Some((draft.body.as_str().to_owned(), book.events.clone()));
        }
    }

    /// The rewards' act, as the rows or the footer gave it (both UIs).
    pub(super) fn do_rewards_act(
        &mut self,
        (act, ticket): (RewardsAct, Option<design::results::Ticket>),
        book: &mut Book,
        plan: &Plan<'_>,
    ) {
        let session = self.selected;
        match (act, ticket) {
            (RewardsAct::Undo(event), Some(ticket)) => {
                self.undo_rewards(ticket, event, book, session, plan);
            }
            (RewardsAct::Undo(_), None) => {}
            (act, _) => self.rewards_act(act, book, session, plan),
        }
    }

    /// A planned event's `Remove` (both UIs).
    pub(super) fn remove_plan_event(&mut self, id: &str, book: &mut Book) {
        book.remove(id);
        self.removed_event_in_plan = true;
        self.plan_notice_state = Some((
            self.draft
                .as_ref()
                .map_or_else(String::new, |draft| draft.body.as_str().to_owned()),
            book.events.clone(),
        ));
        self.open_event = None;
    }

    /// A saved run's `Delete the saved run` (both UIs).
    pub(super) fn delete_plan_run(
        &mut self,
        saved: &crate::encounter_run::SavedRunInfo,
        book: &Book,
    ) {
        match crate::encounter_run::DeletedRun::delete(saved) {
            Ok(deleted) => {
                self.plan_notice_state = Some((
                    self.draft
                        .as_ref()
                        .map_or_else(String::new, |draft| draft.body.as_str().to_owned()),
                    book.events.clone(),
                ));
                self.record_deleted_run(book, deleted, saved.id);
            }
            Err(error) => self.rename_problem = Some(error),
        }
    }

    /// An event row's title: open it in place, or close it (both UIs).
    pub(super) fn toggle_plan_event(&mut self, id: String) {
        self.fight = None;
        self.show_fight_log = false;
        self.reveal_event = true;
        self.open_event = (self.open_event.as_ref() != Some(&id)).then_some(id);
    }

    /// The plan pane's message footer: on the session page the page's own messages too
    /// (`session_messages`), then the plan's results, reasons and notices (both UIs).
    #[allow(clippy::type_complexity)]
    pub(super) fn plan_footer_messages<'f>(
        &mut self,
        frame: &'f PaneFrame,
        book: &mut Book,
        plan: &Plan<'_>,
        page: Option<(
            &crate::processing::settings::ProcessingSettings,
            &crate::sessions::Note,
        )>,
        (shown, planned_results): (Shown, &[(String, bool)]),
        width: f32,
    ) -> Messages<'f, super::SessionAct> {
        // Round 26 FL1, round 28: the plan's results, reasons and notices sit in its pane's
        // message footer, which the part places `S` below the rows (pinned only when the rows
        // fill the pane), so a result never moves a row or the Add strip. On the session page
        // the page's own messages share this footer: one footer for what the DM sees.
        let session = self.selected;
        let here = self.group_session == session;
        self.end_plan_marks();
        let mut messages = match page {
            Some((settings, note)) => {
                self.session_messages(frame, settings, book, plan, note, width)
            }
            None => Messages::new(frame, ("session_plan_footer", session), width),
        };
        if plan.group_open && self.not_being_played().is_none() {
            let label = self
                .notes
                .iter()
                .find(|note| Some(note.id) == self.group_session)
                .map_or_else(
                    || "another session".to_owned(),
                    |note| format!("“{}”", note.label()),
                );
            messages.outcome(
                Notice::new(crate::sentence!(
                    "A group is being assembled in the plan of {}.",
                    label
                ))
                .choice(Choice::new(
                    MessageChoice::Choice("Go to it"),
                    pressed(FooterAct::GoToGroup),
                )),
            );
        }
        // M0244: the Encounters store's own line of the discard, with its order and ticket.
        if here && let Some(line) = (plan.group_discard)(frame) {
            messages.kept(line.map(|()| pressed(FooterAct::UndoGroupDiscard)));
        }
        if let Some(problem) = &self.rename_problem {
            messages.outcome(Outcome::Refused(as_sentence(problem)));
        }
        file_messages(frame, &mut messages, book, |act| {
            pressed(FooterAct::File(act))
        });
        self.plan_messages(frame, &mut messages, book, session, plan, planned_results);
        self.scene_messages(frame, &mut messages, shown);
        self.removed_group_message(frame, &mut messages, session);
        // Round 27 FN1, FK3, SK2: rewards results and their fault are the footer's, by fight.
        self.rewards.see(self.draft.as_ref(), session, book);
        if let (Some(note), Some(draft)) = (session, self.draft.as_ref()) {
            self.rewards
                .in_footer(frame, &mut messages, note, draft.revision());
        }
        // Round 27 FE7: `Plan “<name>”`'s result and `Undo plan line`, in the plan of its
        // journey's session, while the note is as the act left it; a refused Undo's answer,
        // until its own events end it. The same stored line as Timeline's footer.
        for line in self.plan_line_kept(frame, Some(session), pressed(FooterAct::UndoPlanLine)) {
            messages.kept(line);
        }
        messages
    }

    /// Where a scene act's words show, decided as they are shown: at the act's own place while
    /// that place is drawn (the open Scene field, or a row's place among the shown scenes), else
    /// in the plan's footer. Words never go to a place that is not drawn. `rows` are the scenes
    /// at each place, or `None` while they fold.
    pub(super) fn shown_at(&self, place: Place, rows: Option<&[Vec<usize>]>) -> Place {
        let drawn = match place {
            Place::Field => self.scenes.field.is_some(),
            Place::Row((position, at)) => {
                rows.is_some_and(|rows| rows.get(position).is_some_and(|row| at <= row.len()))
            }
            Place::Footer => true,
        };
        if drawn { place } else { Place::Footer }
    }

    /// Keep what a frame measured for the next frame's fold. Folded scenes keep the height they
    /// had when last shown. Returns whether the fold changes in the next frame, which is then
    /// due at once.
    fn keep_fold_measure(
        &mut self,
        folded: bool,
        viewport: f32,
        measure: fold::Measure,
        hold: bool,
    ) -> bool {
        let fold = &mut self.scenes.fold;
        fold.viewport = Some(viewport);
        fold.measure = if folded {
            fold.measure.folded(measure)
        } else {
            measure
        };
        !hold && fold.measured() != fold.overflow
    }

    /// While the DM types a scene, the fold holds.
    fn fold_holds(&self) -> bool {
        self.scenes.typing || self.scenes.field.as_ref().is_some_and(|field| field.focus)
    }

    /// The scenes' fold for a view that is not egui's (the GPUI plan): whether they fold now,
    /// and whether the heading offers `N scenes`.
    pub(super) fn scene_fold(&mut self, count: usize) -> (bool, bool) {
        let hold = self.fold_holds();
        self.scenes.fold.state(count, hold)
    }

    /// `N scenes` pressed while the scenes were `folded`: they open in place, or close again.
    pub(super) fn press_scene_fold(&mut self, folded: bool) {
        self.scenes.fold.open = Some(folded);
    }

    /// What a view that is not egui's measured in a frame, for the next frame's fold (`Fold`):
    /// the pane's room for the rows, the closed rows without the scenes, the scenes, the opened
    /// rows' details, and the next fight still to come (its bottom, and the details and the
    /// scenes above it). Returns whether the fold changes in the next frame.
    pub(super) fn note_fold_measure(
        &mut self,
        folded: bool,
        viewport: f32,
        (closed, scenes, details): (f32, f32, f32),
        next: Option<(f32, f32, f32)>,
    ) -> bool {
        let measure = fold::Measure {
            closed,
            scenes,
            details,
            next: next.map(|(bottom, details, scenes)| fold::Next {
                bottom,
                details,
                scenes,
            }),
        };
        let hold = self.fold_holds();
        self.keep_fold_measure(folded, viewport, measure, hold)
    }

    pub(super) fn plan_projection(
        &mut self,
        book: &Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
        index: Option<&ContentIndex>,
    ) -> Arc<PlanProjection> {
        let model = self.plan_model(index, plan);
        let inputs = ProjectionInputs {
            session,
            draft: self
                .draft
                .as_ref()
                .map_or(0, crate::sessions::Draft::revision),
            book: book.revision(),
            fights: plan.fights_revision,
            index: index.map(ContentIndex::generation),
        };
        let cache = &mut self.plan_projection;
        cache.reached += 1;
        let revision = match &mut cache.inputs {
            Some(kept) => {
                kept.set(inputs);
                kept.revision()
            }
            None => cache
                .inputs
                .insert(design::memo::Revised::new(inputs))
                .revision(),
        };
        let body = self.draft.as_ref().map_or("", |draft| draft.body.as_str());
        let builds = &mut cache.builds;
        Arc::clone(cache.kept.get_or(revision, || {
            *builds += 1;
            Arc::new(PlanProjection::build(model, body, book, session, plan))
        }))
    }

    pub(super) fn plan_model(
        &mut self,
        index: Option<&ContentIndex>,
        plan: &Plan<'_>,
    ) -> Arc<PlanModel> {
        // The fights' and groups' names come from the Encounters snapshot: its revision
        // (`Plan::fights_revision`) keys them, so a quiet frame hashes no name (layout 11c A2a).
        let key = (
            self.selected,
            self.draft
                .as_ref()
                .map_or(0, crate::sessions::Draft::revision),
            index.map(ContentIndex::generation),
            plan.fights_revision,
        );
        let cache = &mut self.plan_model;
        cache.reached += 1;
        if cache.key != Some(key) || cache.model.is_none() {
            cache.builds += 1;
            let body = self.draft.as_ref().map_or("", |draft| draft.body.as_str());
            let mut model = PlanModel::build(body, plan.fights, plan.groups, index);
            model.place_not_found(&mut cache.seen);
            cache.model = Some(Arc::new(model));
            cache.key = Some(key);
        }
        Arc::clone(cache.model.as_ref().expect("the model was just built"))
    }

    /// The scenes' results whose own place is not drawn, in the plan's message footer
    /// (`shown_at`): the Scene field's once it closes, a row's while the scenes fold, and a
    /// removal whose row's place is gone. The Undo stays the note's, so it stays in reach.
    fn scene_messages<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, super::SessionAct>,
        shown: Shown,
    ) {
        if let Some(fault) = &self.scenes.fault
            && shown.fault == Some(Place::Footer)
        {
            messages.outcome(FaultMessage::new(&fault.what));
        }
        if shown.added == Some(Place::Footer)
            && let Some(undo) = &self.scenes.added
            && self.draft.as_ref().is_some_and(|draft| undo.live(draft))
        {
            self.plan_line(frame, messages, PlanKind::SceneAdded, || {
                Said::new(crate::sentence!("Scene added to tonight."))
                    .undo(
                        UndoStep::of(Verb::Add, "scene")
                            .pressed_as(pressed(FooterAct::Scene(SceneAct::UndoAdd))),
                    )
                    .into()
            });
        }
        if shown.removed == Some(Place::Footer)
            && let Some((_, words, undo)) = &self.scenes.removed
            && self.draft.as_ref().is_some_and(|draft| undo.live(draft))
        {
            self.plan_line(frame, messages, PlanKind::SceneRemoved, || {
                Said::new(crate::sentence!("Removed “{}” from tonight.", words))
                    .undo(
                        UndoStep::of(Verb::Remove, words.as_str()).pressed_as(pressed(
                            FooterAct::Scene(SceneAct::UndoRemove(Place::Footer)),
                        )),
                    )
                    .into()
            });
        }
    }

    /// One scene act, through the note's one act path: it follows the disk, changes the one line
    /// that the row was drawn from and saves at once. What it could not do shows where the DM
    /// acted. An act whose save failed keeps its Undo: the fragment waits in the draft.
    pub(super) fn scene_act(&mut self, act: SceneAct, session: Option<Uuid>) {
        if let SceneAct::StartFight(fight) = act {
            self.wants_start_fight = Some(fight);
            return;
        }
        if let SceneAct::AddToLog(id, name, words) = act {
            self.wants_log = Some((id, name, words));
            return;
        }
        let (Some(draft), Some(note)) = (&mut self.draft, session) else {
            return;
        };
        // The session as a refusal names it.
        let within = self
            .notes
            .iter()
            .find(|one| one.id == note)
            .map(crate::sessions::Note::label)
            .and_then(design::results::Name::new)
            .unwrap_or(design::results::Name::fallback(
                design::results::Fallback::ThatSession,
            ));
        let scenes = &mut self.scenes;
        let (marks, folder) = (&mut self.plan_marks, self.folder.as_deref());
        scenes.fault = None;
        // What the act would not have done, to which scene, when it is refused.
        let scene = |words: &str| {
            design::results::Thing::new(
                design::results::Kind::Scene,
                design::results::Name::new(words).unwrap_or(design::results::Name::fallback(
                    design::results::Fallback::ThisScene,
                )),
            )
        };
        use design::results::FailedAct;
        let (result, place, failed) = match act {
            SceneAct::StartFight(_) | SceneAct::AddToLog(..) => return,
            SceneAct::Add(words) => {
                let result = draft.apply(note, Apply::Now, |text| Some(add_scene(text, &words)));
                let failed = FailedAct::NotAdded {
                    what: scene(&plain_links(&words)),
                    to: within.clone(),
                };
                if let Ok(undo) | Err(ActFault::NotSaved(_, undo)) = &result {
                    scenes.added = Some(undo.clone());
                    scenes.added_words = plain_links(&words);
                    Self::mark_plan(marks, folder, PlanKind::SceneAdded);
                    if let Some(field) = &mut scenes.field {
                        field.clear();
                        field.focus = true;
                    }
                }
                (result.map(drop), Place::Field, (failed, false))
            }
            SceneAct::Tick(slot, task) => {
                let what = scene(&plain_links(&task.words));
                let within = within.clone();
                let failed = if task.done {
                    FailedAct::NotUnticked { what, within }
                } else {
                    FailedAct::NotTicked { what, within }
                };
                let result = draft.apply(note, Apply::Now, |text| {
                    change_line(
                        text,
                        task.at,
                        &task.line,
                        Some(&ticked(&task.line, !task.done)),
                    )
                });
                (result.map(drop), Place::Row(slot), (failed, false))
            }
            SceneAct::Remove(slot, line, task) => {
                let result = draft.apply(note, Apply::Now, |text| {
                    change_line(text, task.at, &task.line, None)
                });
                let failed = FailedAct::NotRemoved {
                    what: scene(&plain_links(&task.words)),
                    from: within.clone(),
                };
                if let Ok(undo) | Err(ActFault::NotSaved(_, undo)) = &result {
                    scenes.removed = Some((line, plain_links(&task.words), undo.clone()));
                    Self::mark_plan(marks, folder, PlanKind::SceneRemoved);
                }
                (result.map(drop), Place::Row(slot), (failed, false))
            }
            // The Undo of an add takes the scene back out of the note.
            SceneAct::UndoAdd => (
                scenes.added.take().map_or(Ok(()), |undo| undo.undo(draft)),
                Place::Field,
                (
                    FailedAct::NotRemoved {
                        what: scene(&scenes.added_words),
                        from: within.clone(),
                    },
                    true,
                ),
            ),
            SceneAct::UndoRemove(place) => {
                let removed = scenes.removed.take();
                let failed = FailedAct::NotPutBack {
                    what: scene(removed.as_ref().map_or("", |(_, words, _)| words.as_str())),
                    into: within.clone(),
                };
                (
                    removed.map_or(Ok(()), |(_, _, undo)| undo.undo(draft)),
                    place,
                    (failed, true),
                )
            }
        };
        if let Err(fault) = result {
            let (failed, undo) = failed;
            let what = if matches!(fault, ActFault::NotSaved(..)) {
                SceneFault::NotSaved
            } else {
                let (cause, step) = kept::note_cause(&fault, undo);
                SceneFault::Refused(design::results::Refusal::new(
                    failed,
                    cause,
                    design::results::Recovery::First(step),
                ))
            };
            scenes.fault = Some(Fault {
                met: draft.body.as_str().to_owned(),
                place,
                waits: matches!(what, SceneFault::NotSaved),
                what,
            });
        }
    }

    /// The plan's results end at the next edit of the note or the timeline events.
    pub(super) fn end_plan_results_after_edits(&mut self, book: &Book) {
        if let Some((body, events)) = &self.plan_notice_state
            && (self
                .draft
                .as_ref()
                .is_none_or(|draft| !draft.body.same_words(body))
                || book.events != *events)
        {
            self.deleted_run = None;
            self.removed_group = None;
            self.removed_fight = None;
            self.removed_event_in_plan = false;
            self.plan_notice_state = None;
        }
    }

    /// The deletion's result. While it shows, it replaces the result of the removal action it
    /// follows, when that removal is of this run's own event.
    fn record_deleted_run(
        &mut self,
        book: &Book,
        deleted: crate::encounter_run::DeletedRun,
        run: Option<Uuid>,
    ) {
        Self::mark_plan(
            &mut self.plan_marks,
            self.folder.as_deref(),
            PlanKind::DeletedRun,
        );
        self.deleted_run = Some((deleted, removal_replaced_by(book, run)));
        self.wants_runs_refresh = true;
    }

    /// One step, one result: decided each draw from the two live results. The book's removal
    /// result hides only while the deletion result that followed that same removal action
    /// shows. A new removal, even of the same event, is another action and shows.
    fn removal_superseded(&self, book: &Book) -> bool {
        self.deleted_run
            .as_ref()
            .and_then(|(_, action)| *action)
            .is_some_and(|action| book.removal_action() == Some(action))
    }

    /// Put the deleted run's file back. Its result ends; a removal result it replaced returns.
    fn undo_deleted_run(&mut self) {
        if let Some((deleted, _)) = &self.deleted_run {
            match deleted.restore() {
                Ok(()) => {
                    self.deleted_run = None;
                    self.wants_runs_refresh = true;
                }
                Err(error) => self.rename_problem = Some(error),
            }
        }
    }

    fn deleted_run_words(&self) -> Option<(Tone, String)> {
        self.deleted_run.as_ref().map(|(deleted, _)| {
            let tone = if deleted.unsent.is_empty() {
                Tone::Success
            } else {
                Tone::Warning
            };
            (tone, deleted.message())
        })
    }

    fn deleted_run_message<'f>(
        &self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, super::SessionAct>,
    ) {
        if let Some((tone, words)) = self.deleted_run_words() {
            let undo =
                UndoStep::of(Verb::Delete, "run").pressed_as(pressed(FooterAct::UndoDeletedRun));
            let words = as_sentence(&words);
            self.plan_line(frame, messages, PlanKind::DeletedRun, || {
                if tone == Tone::Success {
                    Said::new(words).undo(undo).into()
                } else {
                    KeptOutcome::Fault(FaultMessage::new(&PlanFault::Check(words)), undo)
                }
            });
        }
    }

    /// What can be undone in the plan: the event just added or just removed, a deleted run, and
    /// a rest's ruling. Round 28 (rows 29, 77, 78a, 131): each is in the footer, never in a
    /// row, and a ruling's result stays while its Undo is live, open or closed.
    fn plan_messages<'f>(
        &mut self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, super::SessionAct>,
        book: &Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
        planned: &[(String, bool)],
    ) {
        self.deleted_run_message(frame, messages);
        // The Add result: its owner update (`settle_timed`) ended it if it is over (its life, the
        // DM's next edit, another session), so this frame finds its kept line, made again only
        // when the store, the book or the saved-run facts changed, and presents it.
        let key = self.of_session(session);
        let inputs = (book.revision(), plan.saved_runs_revision);
        let projection = self.added_view.refresh(&self.added, &key, inputs, |shown| {
            let just = shown.value();
            let restores = book
                .events
                .iter()
                .any(|event| event.id == just.id && event.restores)
                .then(|| timeline::fights_that_keep_hp(&book.events, session, plan.saved_runs));
            let words = as_sentence(&added_message(&just.label, just.to.as_deref(), restores));
            // M0529 waits for layout 11c: an Add Undo that names its event moves under its words
            // and makes the footer a line taller, which shifts the plan under the DM.
            Said::new(words)
                .undo(UndoStep::verb(Verb::Add).pressed_as(pressed(FooterAct::UndoAdd)))
                .into()
        });
        if let Ok(Some(line)) = self.added.present(frame, projection, &key, inputs) {
            messages.kept(line);
        } else if !self.removal_superseded(book)
            && let Some(shown) = book.results.removal.get(&book.file())
        {
            let days = timeline::day_numbers(&book.events);
            let label = shown.value().label().to_owned();
            let words = as_sentence(&removed_event_words(book, session, &days, &label, planned));
            // M-N1.7: the Undo names the event, as Timeline's line of the same removal does.
            messages.kept(shown.outcome(frame, |_| {
                Said::new(words)
                    .undo(
                        UndoStep::of(Verb::Remove, label)
                            .pressed_as(pressed(FooterAct::UndoRemove)),
                    )
                    .into()
            }));
        }
        if let Some(rest) = book.rest_undo_id().filter(|id| {
            book.events
                .iter()
                .any(|event| event.id == *id && event.is_in(session))
        }) {
            timeline::page::ruling_line(
                frame,
                messages,
                book,
                rest,
                pressed(FooterAct::UndoRuling),
            );
        }
    }

    /// The open played fight's summary, read again only for another run or another answer of
    /// its run folder. Returns the run's file, when the folder found it.
    pub(super) fn keep_fight(&mut self, run: Uuid, plan: &Plan<'_>) -> Option<std::path::PathBuf> {
        // R41 (round 30 review F2): the run's file as the folder says now, with the folder's
        // cause typed. The kept summary belongs to this run AND this answer: when the answer
        // changes (the folder comes back, goes, or the file moves), the summary is read again.
        let found = match (&plan.run_directory, &self.campaign) {
            (Some(directory), Some(campaign)) => Some(crate::encounter_run::find_run_typed(
                directory,
                campaign.id,
                run,
            )),
            _ => None,
        };
        let path = found
            .as_ref()
            .and_then(|found| found.as_ref().ok())
            .cloned();
        if self
            .fight
            .as_ref()
            .is_none_or(|(id, kept, _, _)| *id != run || *kept != found)
        {
            let result = match (&found, &self.campaign) {
                (Some(Ok(path)), Some(campaign)) => {
                    fight_summary(path, campaign.id).map_err(FightFault::Other)
                }
                (Some(Err(crate::encounter_run::FindRunFault::Folder(folder))), _) => {
                    Err(FightFault::Folder(folder.clone()))
                }
                (Some(Err(crate::encounter_run::FindRunFault::File(words))), _) => {
                    Err(FightFault::Other(words.clone()))
                }
                _ => Err(FightFault::Other(
                    "The run folder is not available.".to_owned(),
                )),
            };
            let revision = result
                .as_ref()
                .map_or(0, |summary| log_revision(&summary.log));
            self.fight = Some((run, found, result, revision));
        }
        path
    }
}

/// A player's whole line in a played fight's block: name, HP, temporary HP, absence and
/// conditions (the cell's hover).
fn fight_row_detail(row: &crate::encounter_run::FightRow) -> String {
    let known = |n: Option<i64>| n.map_or_else(|| "?".to_owned(), |n| n.to_string());
    let hp = crate::design::fraction(known(row.hp), known(row.max_hp));
    let mut detail = format!("{}: {hp} HP", row.name);
    if row.temporary_hp > 0 {
        detail.push_str(&format!(" · {}", design::temp_hp_text(row.temporary_hp)));
    }
    if !row.present {
        detail.push_str(" · Absent");
    }
    for condition in &row.conditions {
        detail.push_str(&format!(" · {}", condition.words()));
    }
    detail
}

/// One player of a played fight's block, for a view that is not egui's: the first name, HP and
/// its max, temporary HP, and the whole line (`fight_row_detail`).
pub struct FightCell {
    pub first: String,
    pub hp: Option<i64>,
    pub max: Option<i64>,
    pub temp: i64,
    pub detail: String,
}

/// The played fight's players in the party's order, as the block lists them.
pub(super) fn fight_cells(
    summary: &crate::encounter_run::FightSummary,
    party: &[Uuid],
) -> Vec<FightCell> {
    in_party_order(&summary.rows, party)
        .into_iter()
        .map(|row| FightCell {
            first: row
                .name
                .split_whitespace()
                .next()
                .unwrap_or(&row.name)
                .to_owned(),
            hp: row.hp,
            max: row.max_hp,
            temp: row.temporary_hp,
            detail: fight_row_detail(row),
        })
        .collect()
}

/// The plain line of a played fight's fault, as its block says it (`fight_block`), for a view
/// that is not egui's: the folder's cause in Encounters' words and tone (R41), or that the run
/// cannot be read.
pub(super) fn fight_fault_line(fault: &FightFault) -> design::outcome::plain::PlainLine<()> {
    match fault {
        FightFault::Folder(folder) => {
            let fault = PlanFault::Toned(as_sentence(&folder.message()), folder.severity());
            FaultMessage::new(&fault).into_plain()
        }
        FightFault::Other(error) => {
            let words = as_sentence(&format!("This fight’s run cannot be read. {error}"));
            Notice::new(words).into_plain()
        }
    }
}

/// How many Log lines a played fight's block holds (`log_view`): as many as a third of the
/// reader's measure, each a line and the gap after it, and at least the newest entry and the
/// notice.
pub fn log_room(line: f32, gap: f32) -> usize {
    ((design::READER_MEASURE / 3.0 + gap) / (line + gap))
        .floor()
        .max(2.0) as usize
}

/// The Log's last line when it leaves entries out or cuts one: what Whole run has.
pub fn log_notice(left_out: usize) -> String {
    match left_out {
        0 => "Whole run has every line whole.".to_owned(),
        1 => "Whole run has 1 more line.".to_owned(),
        more => format!("Whole run has {more} more lines."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::Draft;
    use crate::timeline::Kind;
    use std::{fs, path::PathBuf};

    #[test]
    fn iter23_s_fk4_final_hp_follows_the_party_order() {
        let row = |name: &str, id: u128, player: bool| crate::encounter_run::FightRow {
            id: Uuid::from_u128(id),
            name: name.to_owned(),
            player,
            hp: Some(1),
            max_hp: Some(2),
            temporary_hp: 0,
            conditions: Vec::new(),
            present: true,
            concentration: false,
        };
        let rows = [
            row("Kestrel Dunmoor", 3, true),
            row("Cinder Wolf", 9, false),
            row("Former Member", 8, true),
            row("Iselle Marrowind", 2, true),
            row("Brannoch Vale", 1, true),
        ];
        let party = [1, 2, 3].map(Uuid::from_u128);
        let names: Vec<_> = in_party_order(&rows, &party)
            .into_iter()
            .map(|row| row.name.as_str())
            .collect();
        assert_eq!(
            names,
            [
                "Brannoch Vale",
                "Iselle Marrowind",
                "Kestrel Dunmoor",
                "Former Member"
            ]
        );
    }

    fn legacy_plan_run(name: &str) -> crate::encounter_run::SavedRunInfo {
        crate::encounter_run::SavedRunInfo {
            path: PathBuf::from("legacy.json"),
            id: Some(Uuid::new_v4()),
            source: None,
            encounter: name.into(),
            label: "Round 2".into(),
            standing: crate::encounter_run::Standing::Unfinished,
            setup: false,
            saved_at: None,
            unsent: vec![],
        }
    }

    fn planned_fight(name: &str, path: &str) -> PlanFight {
        PlanFight {
            saved: None,
            id: uuid::Uuid::nil(),
            name: name.into(),
            path: path.into(),
            summary: Default::default(),
            revision: None,
            difficulty: Default::default(),
        }
    }

    /// Review of PR 284, finding 1: the mark is one whole comment and nothing else. The DM's
    /// words after it stay in Read, in `Remove from tonight` and in the line acts that treat the
    /// line as its text; half a mark is no mark, and its line shows as it is.
    #[test]
    fn pr284_review_words_after_the_mark_stay() {
        let mark = "<!-- splat:check journey=t-road at=24 sum=12345678 -->";
        let body =
            format!("## Plan for tonight\n- [[Bridge Ambush]] {mark} Bring silver weapons.\n");
        let fights = [planned_fight(
            "Bridge Ambush",
            "Encounters/Bridge Ambush.md",
        )];
        let removed = unlink_planned_fight(&body, "Bridge Ambush", 0);
        assert_eq!(
            removed,
            body.replace("[[Bridge Ambush]]", "Bridge Ambush"),
            "Remove from tonight keeps the line's words"
        );
        let shown = super::super::compact_plan_for_reading(&body, &fights, &[], &[], true);
        assert!(shown.contains("Bring silver weapons."), "{shown:?}");
        // An isolated marked line is a planned link: Read folds it, Remove takes it whole.
        let alone = format!("## Plan for tonight\n- [[Bridge Ambush]] {mark}\n");
        assert_eq!(
            unlink_planned_fight(&alone, "Bridge Ambush", 0),
            "## Plan for tonight\n"
        );
        let folded = super::super::compact_plan_for_reading(&alone, &fights, &[], &[], true);
        assert!(!folded.contains("Bridge"), "{folded:?}");
        let half = "## Plan for tonight\n- [[Bridge Ambush]] <!-- splat:check journey=t-road\n";
        assert_eq!(
            super::super::compact_plan_for_reading(half, &fights, &[], &[], true),
            half,
            "half a mark is the line's own text"
        );
        // The done box, a rename by `Edit encounter`, and a line change keep the rest byte for
        // byte: they act on the line as its text.
        let line = format!("- [ ] [[Bridge Ambush]] {mark} Bring silver weapons.");
        assert_eq!(
            crate::sessions::ticked(&line, true),
            line.replacen("[ ]", "[x]", 1)
        );
        let mut renamed = line.clone();
        crate::sessions::rename_links(&mut renamed, "Bridge Ambush", "Ford Ambush");
        assert_eq!(renamed, line.replace("Bridge", "Ford"));
    }

    /// A session note whose plan holds two marked lines for Bridge Ambush, at hours 0 and 24 of
    /// one two-day journey, a played fight called Scouts, and the Timeline saved beside it.
    fn two_marked_lines() -> (
        tempfile::TempDir,
        Sessions,
        Book,
        [checks::RoadMark; 2],
        String,
    ) {
        use crate::sessions::{NewNote, create_note};
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 1,
                title: "Road",
                date: "2026-11-12",
                campaign: "Marches",
                template: "## Plan for tonight\n",
            },
        )
        .unwrap();
        let session = Some(note.id);
        let mut b = Book::default();
        b.open(Some(dir.path()));
        b.add(Kind::Day, session);
        let road = b.add(Kind::Travel, session);
        let e = b.event_mut(&road).unwrap();
        e.duration = "2 days".into();
        e.checks = [0, 24]
            .map(|at| timeline::Check::new(at, "16 · [[Bridge Ambush]] waits.".into()))
            .to_vec();
        let marks = [0, 1].map(|at| checks::RoadMark::new(&road, &e.checks[at]).unwrap());
        let text = fs::read_to_string(&note.path).unwrap().replace(
            "## Plan for tonight",
            &format!(
                "## Plan for tonight\n- [[Bridge Ambush]] {}\n- [[Bridge Ambush]] {}",
                marks[0].comment(),
                marks[1].comment()
            ),
        );
        fs::write(&note.path, text).unwrap();
        let sessions = Sessions {
            selected: session,
            notes: vec![note],
            ..Sessions::default()
        };
        let id = b.add(Kind::Encounter, session);
        let played = b.event_mut(&id).unwrap();
        played.title = "Scouts".into();
        played.run = Some(Uuid::from_u128(123));
        b.save();
        (dir, sessions, b, marks, id)
    }

    /// Outside the app, Scouts becomes Bridge Ambush: under the plan's rule it takes the first
    /// marked line.
    fn rename_scouts_outside(dir: &std::path::Path, id: &str) {
        let mut outside = Book::default();
        outside.open(Some(dir));
        outside.event_mut(id).unwrap().title = "Bridge Ambush".into();
        outside.touch();
        outside.save();
    }

    fn begin(b: &mut Book, session: Option<Uuid>, within: &timeline::Within) -> Uuid {
        let setup = timeline::RunFact {
            id: Uuid::from_u128(456),
            copied_from: None,
            name: "Bridge Ambush".into(),
            round: 1,
            state: timeline::RunState::Setup,
            steps: 1,
        };
        let running = timeline::RunFact {
            state: timeline::RunState::Running,
            ..setup.clone()
        };
        b.record_change(Some(&setup), &running, session, Some(within));
        running.id
    }

    fn fight_start(b: &Book, run: Uuid) -> Option<u64> {
        let at = b.events.iter().position(|e| e.run == Some(run)).unwrap();
        timeline::clock(&b.events)[at]
    }

    /// Review round 2, finding 1: Begin takes the plan line that the played fights on disk give,
    /// in the same read that checks the journey, not a count kept in memory.
    #[test]
    fn pr284_r2_begin_uses_current_occurrence() {
        let (dir, sessions, mut b, [_, second], id) = two_marked_lines();
        let session = sessions.selected;
        // A typed journey note waits for its pause, so the Timeline in memory is not read again.
        let road = second.place(&b.events, session).unwrap().journey;
        b.event_mut(&road).unwrap().notes = "Fog on the road.".into();
        b.touch();
        rename_scouts_outside(dir.path(), &id);
        let (mark, within) = sessions
            .road_at_begin(&b, session, "Bridge Ambush")
            .unwrap()
            .unwrap();
        assert_eq!(mark, second, "the first line is taken by the renamed fight");
        let run = begin(&mut b, session, &within);
        assert_eq!(
            sessions.confirm_road(&mut b, session, run, "Bridge Ambush", &mark),
            None
        );
        assert!(b.error.is_none(), "{:?}", b.error);
        assert_eq!(fight_start(&b, run), Some(1440), "Day 2, 00:00");
    }

    /// Review round 2, finding 4: a change that lands after Begin's read meets the production
    /// check after the save's merge, which sends the fight after the session's last event. Here
    /// the outside rename lands then: the fight's plan line is now the second one.
    #[test]
    fn pr284_r2_confirm_road_moves_a_fight_whose_line_changed_after_begin() {
        let (dir, sessions, mut b, [first, _], id) = two_marked_lines();
        let session = sessions.selected;
        let (mark, within) = sessions
            .road_at_begin(&b, session, "Bridge Ambush")
            .unwrap()
            .unwrap();
        assert_eq!(mark, first);
        let run = begin(&mut b, session, &within);
        rename_scouts_outside(dir.path(), &id);
        assert_eq!(
            sessions.confirm_road(&mut b, session, run, "Bridge Ambush", &mark),
            Some(checks::OffRoad::Changed)
        );
        assert_eq!(
            fight_start(&b, run),
            Some(2 * 1440),
            "after the journey, not Day 1"
        );
        assert!(
            !fs::read_to_string(dir.path().join("Timeline.md"))
                .unwrap()
                .contains("journey=")
        );
    }

    /// Review round 2, finding 2: a fight with no mark gets nothing about travel, also when its
    /// note cannot be read at Begin.
    #[test]
    fn pr284_r2_plain_fight_has_no_travel_notice() {
        use crate::sessions::{Draft, NewNote, create_note};
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 1,
                title: "Plain evening",
                date: "2026-11-12",
                campaign: "Marches",
                template: "## Plan for tonight\n- [[Bridge Ambush]]\n",
            },
        )
        .unwrap();
        let session = Some(note.id);
        let sessions = Sessions {
            selected: session,
            notes: vec![note.clone()],
            draft: Some(Draft::open(&note.path).unwrap()),
            ..Sessions::default()
        };
        let book = Book::default();
        assert_eq!(sessions.road_mark(session, "Bridge Ambush", &[]), None);
        fs::remove_file(&note.path).unwrap();
        assert_eq!(sessions.road_mark(session, "Bridge Ambush", &[]), None);
        assert_eq!(
            sessions.road_at_begin(&book, session, "Bridge Ambush"),
            None,
            "an ordinary fight must not get a travel notice when its note goes"
        );
    }

    /// Review round 2, finding 3: a marked line that the DM turns into a done scene shows its
    /// words, never the mark, in Add to Log's field; the line itself keeps the mark.
    #[test]
    fn pr284_r2_scene_log_hides_mark() {
        let line = "- [x] [[Captain Varra]] <!-- splat:check journey=t-road at=24 sum=12345678 --> Bring silver weapons.";
        let body = format!("## Plan for tonight\n{line}\n");
        let model = PlanModel::build(&body, &[], &[], None);
        let ModelLine::Scene(scene) = &model.lines[0] else {
            panic!("a scene")
        };
        assert_eq!(
            words_outside_links(&scene.task.words),
            "Bring silver weapons."
        );
        assert_eq!(scene.task.line, line, "acts on the line keep it whole");
    }

    /// Review round 2, finding 3: Lookup's search of session notes shows found lines as they
    /// read, so a plan line's mark is not among its words.
    #[test]
    fn pr284_r2_search_shows_no_mark() {
        use crate::sessions::{NewNote, create_note};
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 1,
                title: "Road",
                date: "2026-11-12",
                campaign: "Marches",
                template: "## Plan for tonight\n- [[Bridge Ambush]] <!-- splat:check journey=t-road at=24 sum=12345678 --> Bring silver weapons.\n",
            },
        )
        .unwrap();
        let sessions = Sessions {
            notes: vec![note],
            ..Sessions::default()
        };
        let written = sessions.written(&Book::default());
        let text = &written[0].text;
        assert!(
            text.contains("Bring silver weapons.") && !text.contains("splat:check"),
            "{text:?}"
        );
    }

    /// Review finding 2: Begin reads the session note as the file holds it at that moment, not
    /// the open draft, which can be older.
    #[test]
    fn pr284_review_begin_reads_current_note() {
        use crate::sessions::{Draft, NewNote, create_note};
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 1,
                title: "Road",
                date: "2026-11-12",
                campaign: "Marches",
                template: "## Plan for tonight\n",
            },
        )
        .unwrap();
        let session = Some(note.id);
        let mut b = Book::default();
        b.add(Kind::Day, session);
        let road = b.add(Kind::Travel, session);
        let e = b.event_mut(&road).unwrap();
        e.duration = "2 days".into();
        e.checks = vec![timeline::Check::new(
            24,
            "16 · [[Bridge Ambush]] waits.".into(),
        )];
        let mark = checks::RoadMark::new(&road, &e.checks[0]).unwrap();
        let text = fs::read_to_string(&note.path).unwrap().replace(
            "## Plan for tonight",
            &format!(
                "## Plan for tonight\n- [[Bridge Ambush]] {}",
                mark.comment()
            ),
        );
        fs::write(&note.path, &text).unwrap();
        let sessions = Sessions {
            selected: session,
            notes: vec![note.clone()],
            draft: Some(Draft::open(&note.path).unwrap()),
            ..Sessions::default()
        };
        assert_eq!(
            sessions.road_mark(session, "Bridge Ambush", &b.events),
            Some(mark.clone())
        );
        // Obsidian takes the mark off this plan line before Begin combat.
        fs::write(&note.path, text.replace(&mark.comment(), "")).unwrap();
        assert_eq!(
            sessions.road_mark(session, "Bridge Ambush", &b.events),
            None
        );
        fs::remove_file(&note.path).unwrap();
        assert_eq!(
            sessions.road_mark(session, "Bridge Ambush", &b.events),
            None,
            "a note that cannot be read says nothing about travel"
        );
    }

    #[test]
    fn iter9_c2_legacy_recovery_copy_matches_the_unique_planned_fight() {
        let fights = [planned_fight("Bridge Ambush", "Bridge.md")];
        let mut runs = [legacy_plan_run("Bridge Ambush · saved 03:00:00 (2)")];
        assert_eq!(
            planned_saved_run(&fights[0], &fights, &runs).and_then(|run| run.id),
            runs[0].id,
            "FT-1 case A: a legacy recovery copy resumes its unique planned encounter"
        );
        for standing in [
            crate::encounter_run::Standing::Ended,
            crate::encounter_run::Standing::Unreadable,
        ] {
            runs[0].standing = standing;
            assert!(planned_saved_run(&fights[0], &fights, &runs).is_none());
        }
    }

    #[test]
    fn iter9_c2_legacy_run_does_not_guess_between_equal_planned_names() {
        let fights = [
            planned_fight("New encounter", "First.md"),
            planned_fight("New encounter", "Second.md"),
        ];
        let runs = [legacy_plan_run("New encounter")];
        for fight in &fights {
            assert!(
                planned_saved_run(fight, &fights, &runs).is_none(),
                "FT-1 case B: neither equal-named plan row can claim the legacy run"
            );
        }
        let mut sourced = planned_fight("New encounter", "First.md");
        sourced.saved = Some(runs[0].clone());
        assert_eq!(
            planned_saved_run(&sourced, &fights, &runs).and_then(|run| run.id),
            runs[0].id,
            "An existing source-ID match still takes priority"
        );
    }

    #[test]
    fn group_removal_deletes_only_an_isolated_plan_link() {
        let mut body =
            "## Plan for tonight\n- [[Rival Team]]\n## Notes\n- [[Rival Team]]\n".to_owned();
        unlink_group(&mut body, "Rival Team");
        assert_eq!(body, "## Plan for tonight\n## Notes\n- [[Rival Team]]\n");
        let mut body = "## Plan for tonight\nMeet [[Rival Team|the rivals]] at dusk.\n".to_owned();
        unlink_group(&mut body, "Rival Team");
        assert_eq!(body, "## Plan for tonight\nMeet the rivals at dusk.\n");
    }

    #[test]
    fn removing_one_planned_fight_keeps_other_links_prose_and_sections() {
        let body = "## Recap of last time\n- [[Bridge]]\n## Plan for tonight\n- [[Bridge]]\n- [[Bridge]]\nMeet [[Bridge|the guard]] at dusk.\n- [[Shrine]]\n## What happened\n- [[Bridge]]\n";
        let changed = unlink_planned_fight(body, "Bridge", 1);
        assert_eq!(
            changed,
            body.replacen("- [[Bridge]]\n- [[Bridge]]", "- [[Bridge]]", 1)
        );
        assert_eq!(
            unlink_planned_fight(body, "Bridge", 2),
            body.replace(
                "Meet [[Bridge|the guard]] at dusk.",
                "Meet the guard at dusk."
            )
        );
        assert_eq!(unlink_planned_fight(body, "Missing", 0), body);
    }

    #[test]
    fn fmt_f1_unlink_plan_task_removes_whole_line_only_in_plan() {
        let body = "## Recap\n- [ ] [[Bridge]]\n## Plan for tonight\n- [ ] [[Bridge]]\n- [x] [[Bridge|Again]]\nMeet [[Bridge]] at dusk.\n## What happened\n- [ ] [[Bridge]]\n";
        assert_eq!(
            unlink_planned_fight(body, "Bridge", 0),
            body.replacen(
                "## Plan for tonight\n- [ ] [[Bridge]]\n",
                "## Plan for tonight\n",
                1
            )
        );
        assert_eq!(
            unlink_planned_fight(body, "Bridge", 1),
            body.replace("- [x] [[Bridge|Again]]\n", "")
        );
        assert_eq!(
            crate::sessions::each_linked_name("- [ ] [[Bridge]]\n- [x] [[Shrine]]"),
            ["Bridge", "Shrine"]
        );
    }

    /// `Undo remove` is as on `main` when the note is renamed outside Splat: the folder scan
    /// follows the one note with that ID, and the Undo's file goes with it. A copy that claims
    /// the ID, and wins by its plainer name, does not take the Undo.
    #[test]
    fn undo_remove_follows_a_moved_note_and_not_a_copy() {
        use crate::sessions::{Apply, Draft, NewNote, create_note};
        for copy in [false, true] {
            let dir = tempfile::tempdir().unwrap();
            let note = create_note(
                dir.path(),
                &NewNote {
                    number: 1,
                    title: "One",
                    date: "2026-11-01",
                    campaign: "Marches",
                    template: "## Plan for tonight\n\n- [[Bridge]]\n",
                },
            )
            .unwrap();
            let mut draft = Draft::open(&note.path).unwrap();
            let undo = draft
                .apply(note.id, Apply::WithTyping, |words| {
                    Some(words.replace("- [[Bridge]]\n", ""))
                })
                .unwrap();
            draft.save();
            let mut sessions = Sessions {
                selected: Some(note.id),
                notes: vec![note.clone()],
                folder: Some(dir.path().to_owned()),
                draft: Some(draft),
                removed_fight: Some((Some(note.id), 0, "Bridge".into(), undo)),
                ..Sessions::default()
            };
            let renamed = dir.path().join("Session 1, at the ford.md");
            fs::rename(&note.path, &renamed).unwrap();
            let plainer = dir.path().join("S.md");
            if copy {
                fs::copy(&renamed, &plainer).unwrap();
            }
            sessions.scan_folder();
            let draft = sessions.draft.as_ref().unwrap();
            let (owner, _, _, undo) = sessions.removed_fight.as_ref().unwrap();
            assert_eq!(draft.path, if copy { plainer } else { renamed });
            assert_eq!(
                (*owner, draft.body.as_str()),
                (sessions.selected, undo.after())
            );
            assert_eq!(undo.live(draft), !copy, "a copy: {copy}");
        }
    }

    #[test]
    fn a_deleted_run_replaces_a_removal_result_until_the_book_removal_changes() {
        let folder = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        let first = book.add(Kind::Day, None);
        let second = book.add(Kind::Day, None);
        let run = Uuid::from_u128(7);
        book.events[0].run = Some(run);
        book.remove(&first);
        // The plan's Delete the saved run: the file goes, its result keeps the run's id, and
        // the plan's edit witness is taken after the step.
        let delete = |sessions: &mut Sessions, book: &Book, id: Option<Uuid>| {
            let path = folder.path().join("run.json");
            fs::write(&path, "{}").unwrap();
            let info = crate::encounter_run::SavedRunInfo {
                path,
                id,
                source: None,
                encounter: "Bridge Ambush".into(),
                label: String::new(),
                standing: crate::encounter_run::Standing::Unfinished,
                setup: false,
                saved_at: None,
                unsent: Vec::new(),
            };
            let deleted = crate::encounter_run::DeletedRun::delete(&info).unwrap();
            sessions.record_deleted_run(book, deleted, id);
            let body = sessions
                .draft
                .as_ref()
                .map_or_else(String::new, |draft| draft.body.as_str().to_owned());
            sessions.plan_notice_state = Some((body, book.events.clone()));
        };
        let mut sessions = Sessions::default();
        assert!(!sessions.removal_superseded(&book));
        // A deleted run of another fight, or a run with no id, leaves the removal's result.
        for other in [Some(Uuid::from_u128(8)), None] {
            delete(&mut sessions, &book, other);
            assert!(!sessions.removal_superseded(&book));
            sessions.undo_deleted_run();
        }
        // The removed event's own run replaces its result.
        delete(&mut sessions, &book, Some(run));
        assert!(sessions.removal_superseded(&book));
        // Undo delete run ends only the deletion's result: the run file returns, and the
        // removal's result shows again (review of PR 245, path 2).
        sessions.undo_deleted_run();
        assert!(folder.path().join("run.json").exists());
        assert!(book.removed_event().is_some());
        assert!(!sessions.removal_superseded(&book));
        // Undo on Timeline ends the removal, and the plan sees the edit.
        delete(&mut sessions, &book, Some(run));
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove);
        sessions.end_plan_results_after_edits(&book);
        assert!(!sessions.removal_superseded(&book));
        // Removing the same event again is a new step: its result shows (path 1).
        book.remove(&first);
        assert!(
            !sessions.removal_superseded(&book),
            "a new removal of the same event keeps its own result"
        );
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove);
        book.remove(&second);
        assert!(!sessions.removal_superseded(&book));
        // Re-check of PR 245: with a note open, Timeline's Undo remove and a new removal of the
        // same event both happen before the plan draws again. The note and the events then
        // equal the plan's witness, so its cleanup keeps the deletion result; the new removal
        // still shows its message and Undo remove, because it is a different action.
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove);
        let note = folder.path().join("note.md");
        fs::write(&note, "## What happened\n\nThe bridge held.\n").unwrap();
        sessions.draft = Some(Draft::open(&note).unwrap());
        book.remove(&first);
        delete(&mut sessions, &book, Some(run));
        assert!(sessions.removal_superseded(&book));
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove);
        book.remove(&first);
        sessions.end_plan_results_after_edits(&book);
        assert!(
            sessions.deleted_run.is_some(),
            "the plan's cleanup sees no edit, so the deletion result stays"
        );
        assert!(book.removed().is_some());
        assert!(
            !sessions.removal_superseded(&book),
            "the new removal of the same event shows its message and Undo remove"
        );
        // M-N1.2: the deletion's footer line is the plan store's entry, and its Undo acts by
        // the line's ticket. A, a deletion, undone by its line; B, a new deletion; a press left
        // over on A's line reaches nothing, and B's line puts B's run back.
        let owner = Sessions::plan_owner(sessions.folder.as_deref(), PlanKind::DeletedRun);
        let ticket = |sessions: &Sessions| {
            sessions
                .plan_marks
                .get(&owner)
                .expect("the deletion is in the plan's store")
                .ticket_for_caller_test()
        };
        let undo = |this: &mut Sessions| {
            this.undo_deleted_run();
            this.deleted_run.is_none()
        };
        let run_file = folder.path().join("run.json");
        sessions.undo_deleted_run();
        delete(&mut sessions, &book, Some(run));
        let (a, a_again) = (ticket(&sessions), ticket(&sessions));
        sessions.undo_plan_mark(Some(a), undo);
        assert!(run_file.exists(), "A's line puts A's run back");
        delete(&mut sessions, &book, Some(run));
        sessions.undo_plan_mark(Some(a_again), undo);
        assert!(
            !run_file.exists() && sessions.deleted_run.is_some(),
            "a press on A's line leaves B alone"
        );
        let b = ticket(&sessions);
        sessions.undo_plan_mark(Some(b), undo);
        assert!(run_file.exists(), "B's line puts B's run back");
        // Its data gone another way (the note's next edit), the entry ends before the footer.
        delete(&mut sessions, &book, Some(run));
        sessions.deleted_run = None;
        sessions.end_plan_marks();
        assert!(
            sessions.plan_marks.get(&owner).is_none(),
            "the entry ends with its data"
        );
        // Review of PR 629 (P2): a campaign switch ends the old campaign's entries, whatever
        // folder the new campaign has.
        delete(&mut sessions, &book, Some(run));
        assert!(!sessions.plan_marks.is_empty());
        sessions.open_campaign(None);
        assert!(
            sessions.plan_marks.is_empty(),
            "the old campaign's plan results end with the switch"
        );
    }

    #[test]
    fn t11_10_a_planned_result_goes_before_the_first_fight_still_to_come() {
        let session = Uuid::from_u128(7);
        let fights = [
            planned_fight("Wolf Pack", "Encounters/Wolf Pack.md"),
            planned_fight("Bridge Ambush", "Encounters/Bridge Ambush.md"),
        ];
        let body = "# The Crossing\n\n## Plan for tonight\n\n- [[Wolf Pack]]\n- [[Bridge Ambush]]\n\n## What happened\n\n[[Wolf Pack]] came.\n";
        let mut book = Book::default();
        let played = book.add(Kind::Encounter, Some(session));
        let event = book
            .events
            .iter_mut()
            .find(|event| event.id == played)
            .unwrap();
        event.title = "Wolf Pack".into();
        event.run = Some(Uuid::from_u128(8));
        // The first Wolf Pack was played, so Bridge Ambush is the first line to come.
        let after = plan_first(
            body,
            "- [[Wolf Pack]]",
            &book.events,
            Some(session),
            &fights,
        );
        assert_eq!(
            after,
            body.replace(
                "- [[Bridge Ambush]]",
                "- [[Wolf Pack]]\n- [[Bridge Ambush]]"
            )
        );
        let to_come: Vec<&str> = plan_links(&after, &book.events, Some(session), &fights)
            .into_iter()
            .filter(|(_, _, waiting)| *waiting)
            .map(|(_, fight, _)| fight.name.as_str())
            .collect();
        assert_eq!(
            to_come,
            ["Wolf Pack", "Bridge Ambush"],
            "the new fight is the next row"
        );
        // W5-D1: planned from the day-two check of tonight's journey, the line carries the
        // journey and the check, and Remove from tonight takes the line.
        book.add(Kind::Day, Some(session));
        let road = book.add(Kind::Travel, Some(session));
        let travel = book.event_mut(&road).unwrap();
        travel.title = "Road to the ford".into();
        travel.duration = "2 days".into();
        travel.checks = vec![timeline::Check::new(
            24,
            "16 · [[Wolf Pack]] pick up the trail.".into(),
        )];
        let chosen = checks::Planned {
            name: "Wolf Pack".into(),
            check: ("Check at 24h".into(), travel.checks[0].words.clone()),
        };
        let line = road_plan_line(&book, &road, &chosen);
        assert!(
            line.starts_with("- [[Wolf Pack]] <!-- splat:check journey="),
            "{line}"
        );
        let marked = plan_first(body, &line, &book.events, Some(session), &fights);
        assert_eq!(unlink_planned_fight(&marked, "Wolf Pack", 1), body);
        // With every fight played, the line goes at the plan's end, before the next heading.
        let all = book.add(Kind::Encounter, Some(session));
        let event = book
            .events
            .iter_mut()
            .find(|event| event.id == all)
            .unwrap();
        event.title = "Bridge Ambush".into();
        event.run = Some(Uuid::from_u128(9));
        let after = plan_first(
            body,
            "- [[Wolf Pack]]",
            &book.events,
            Some(session),
            &fights,
        );
        assert_eq!(
            after,
            body.replace(
                "- [[Bridge Ambush]]\n",
                "- [[Bridge Ambush]]\n- [[Wolf Pack]]\n"
            )
        );
        // Row 123, point 3: the plan's rows (`PlanModel`) and the offer's and the write's reader
        // (`plan_links`, through `JourneyPlan::read`) read one plan alike: a duplicate, a link
        // with other words, a done task and a struck-out line.
        let odd = "## Plan for tonight\n\n- [[Wolf Pack]]\n- [[wolf pack|the wolves]] again\n- [x] [[Bridge Ambush]]\n- ~~[[Shrine]]~~\n\n## After\n\n[[Shrine]]\n";
        let fights = [
            planned_fight("Wolf Pack", "Encounters/Wolf Pack.md"),
            planned_fight("Bridge Ambush", "Encounters/Bridge Ambush.md"),
            planned_fight("Shrine", "Encounters/Shrine.md"),
        ];
        let rows: Vec<String> = PlanModel::build(odd, &fights, &[], None)
            .lines
            .iter()
            .flat_map(|line| match line {
                ModelLine::Fights(names) => names.clone(),
                ModelLine::Scene(_) => Vec::new(),
            })
            .collect();
        let read: Vec<String> = plan_links(odd, &[], Some(session), &fights)
            .into_iter()
            .map(|(_, fight, _)| fight.name.clone())
            .collect();
        assert_eq!(rows, read, "one reading of the plan's links");
        assert_eq!(rows, ["Wolf Pack", "Wolf Pack", "Bridge Ambush", "Shrine"]);
        let names: Vec<String> = fights.iter().map(|fight| fight.name.clone()).collect();
        let offer = JourneyPlan::read(odd, &[], Some(session), &names, &[]);
        assert_eq!(offer.get("SHRINE"), Some(true));
        assert_eq!(offer.get("Ravens"), None);
    }

    /// Row 123, R26 and points 1 to 3, 5: Timeline's `Plan “<name>”` for a journey of session B
    /// while session A is open on the Sessions page. A keeps its selection, draft, unsent Scene
    /// words, scene Undo and open event; B's note gets the line; the result and Undo are B's.
    #[test]
    fn row_123_timeline_plans_into_another_session_and_leaves_the_open_one() {
        use crate::sessions::{NewNote, create_note};
        let dir = tempfile::tempdir().unwrap();
        let new_note = |number, title| {
            create_note(
                dir.path(),
                &NewNote {
                    number,
                    title,
                    date: "2026-11-14",
                    campaign: "Marches",
                    template: "## Plan for tonight\n",
                },
            )
            .unwrap()
        };
        let b_note = new_note(1, "Road");
        let a_note = new_note(2, "Keep");
        let (a, b) = (a_note.id, b_note.id);
        let mut book = Book::default();
        book.open(Some(dir.path()));
        let road = book.add(Kind::Travel, Some(b));
        let journey = book.event_mut(&road).unwrap();
        journey.duration = "1 day".into();
        journey.checks = vec![timeline::Check::new(
            0,
            "19 · [[Shrine of Cinders]] and [[Bridge Ambush]] wait.".into(),
        )];
        let check = (
            journey.checks[0].label.clone(),
            journey.checks[0].words.clone(),
        );
        let names = PlanNames::new(
            Some(Uuid::from_u128(1)),
            vec!["Shrine of Cinders".into(), "Bridge Ambush".into()],
            Vec::new(),
        );
        let chosen = |name: &str| checks::Planned {
            name: name.into(),
            check: check.clone(),
        };
        let campaign = |name: &str| crate::persistence::Campaign {
            id: Uuid::new_v4(),
            name: name.into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let mut sessions = Sessions {
            campaign: Some(campaign("Marches")),
            selected: Some(a),
            notes: vec![a_note.clone(), b_note.clone()],
            draft: Some(Draft::open(&a_note.path).unwrap()),
            open_event: Some("an event of A".into()),
            last_check: Some(Instant::now()),
            ..Sessions::default()
        };
        // A's scene Undo, and unsent Scene words.
        let scene = sessions
            .draft
            .as_mut()
            .unwrap()
            .apply(a, Apply::Now, |words| {
                Some(format!("{words}- [ ] Watch the ford\n"))
            })
            .unwrap();
        sessions.scenes.added = Some(scene.clone());
        let mut field = design::AddField::default();
        field.words = "Unsent scene".into();
        sessions.scenes.field = Some(field);
        let a_words = sessions.draft.as_ref().unwrap().body.as_str().to_owned();
        let b_file = |shrine: usize| {
            let text = fs::read_to_string(&b_note.path).unwrap();
            assert_eq!(
                text.matches("[[Shrine of Cinders]]").count(),
                shrine,
                "{text}"
            );
            text
        };
        assert_eq!(
            sessions
                .journey_plan(&book, &road, &names)
                .get("Shrine of Cinders"),
            Some(false)
        );
        // Point 1: the press writes B's note, and A is as it was.
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        b_file(1);
        assert_eq!(sessions.selected, Some(a));
        assert_eq!(sessions.draft.as_ref().unwrap().path, a_note.path);
        assert_eq!(sessions.draft.as_ref().unwrap().body.as_str(), a_words);
        assert_eq!(sessions.scenes.added.as_ref(), Some(&scene));
        assert_eq!(
            sessions
                .scenes
                .field
                .as_ref()
                .map(|field| field.words.as_str()),
            Some("Unsent scene")
        );
        assert_eq!(sessions.open_event.as_deref(), Some("an event of A"));
        assert_eq!(
            sessions
                .journey_plan(&book, &road, &names)
                .get("Shrine of Cinders"),
            Some(true)
        );
        assert!(sessions.planned_live(), "the result is B's, and holds");
        // An unrelated act on A leaves B's result; Undo takes back only B's line.
        sessions
            .draft
            .as_mut()
            .unwrap()
            .apply(a, Apply::Now, |words| Some(format!("{words}More.\n")))
            .unwrap();
        assert!(sessions.planned_live());
        assert_eq!(sessions.try_plan_undo(), None);
        b_file(0);
        assert!(sessions.draft.as_ref().unwrap().body.ends_with("More.\n"));
        assert!(sessions.scenes.added.is_some());
        // Point 2: an edit outside Splat with the file's time kept is seen at the next disk check.
        let modified = fs::metadata(&b_note.path).unwrap().modified().unwrap();
        let edit_keeping_time = |text: String| {
            fs::write(&b_note.path, text).unwrap();
            fs::File::options()
                .write(true)
                .open(&b_note.path)
                .unwrap()
                .set_modified(modified)
                .unwrap();
        };
        let text = b_file(0);
        edit_keeping_time(text.replace(
            "## Plan for tonight\n",
            "## Plan for tonight\n- [[Shrine of Cinders]]\n",
        ));
        sessions.last_check = Some(Instant::now());
        assert_eq!(
            sessions
                .journey_plan(&book, &road, &names)
                .get("Shrine of Cinders"),
            Some(true),
            "an outside edit with the same file time changes the offer"
        );
        edit_keeping_time(text.clone());
        sessions.last_check = Some(Instant::now());
        assert_eq!(
            sessions
                .journey_plan(&book, &road, &names)
                .get("Shrine of Cinders"),
            Some(false)
        );
        // Point 3: a line for Bridge Ambush, then Shrine of Cinders added outside before the
        // press that the offer still shows: no second line, and Bridge Ambush's Undo stays.
        sessions
            .plan_from_timeline(&book, &road, &chosen("Bridge Ambush"), &names)
            .unwrap();
        let with_bridge = b_file(0);
        fs::write(
            &b_note.path,
            with_bridge.replace(
                "## Plan for tonight\n",
                "## Plan for tonight\n- [[Shrine of Cinders]]\n",
            ),
        )
        .unwrap();
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        b_file(1);
        assert_eq!(
            sessions.planned_fight().as_deref(),
            Some("Bridge Ambush"),
            "the earlier Undo stays"
        );
        // Point 5: B changed on disk; the failed Undo names its journey.
        let changed = b_file(1).replace("[[Shrine of Cinders]]", "[[Shrine of Cinders]] (moved)");
        fs::write(&b_note.path, changed).unwrap();
        sessions.last_check = Some(Instant::now());
        let words = sessions.try_plan_undo().expect("the Undo is refused");
        assert_eq!(
            words,
            "The plan line for Bridge Ambush was not removed from Session 1: Road. The session \
             note changed on disk. Open that session’s notes and remove the plan line there."
        );
        // Review 3, point 4: the answer stays while its journey stands, also after the change
        // that caused it is seen; it ends with its journey.
        sessions.last_check = Some(Instant::now());
        sessions.follow_plan_line();
        sessions.end_plan_answers_of_lost_journeys(&book);
        assert_eq!(
            sessions.plan_line_refusal().as_deref(),
            Some(words.as_str())
        );
        let mut gone = Book::default();
        gone.touch();
        sessions.end_plan_answers_of_lost_journeys(&gone);
        assert_eq!(
            sessions.plan_line_refusal(),
            None,
            "it ends with its journey"
        );
        assert!(
            !sessions.planned_live(),
            "with it, the result whose Undo waits"
        );
        // Review 3, point 3: a folder that cannot take a replacement file. A failed save leaves
        // the offer at Plan; a failed Undo leaves it at In the plan.
        fs::write(&b_note.path, &text).unwrap();
        let read_only = |only: bool| {
            use std::os::unix::fs::PermissionsExt;
            let mode = if only { 0o555 } else { 0o755 };
            fs::set_permissions(dir.path(), fs::Permissions::from_mode(mode)).unwrap();
        };
        read_only(true);
        let refused =
            sessions.plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names);
        read_only(false);
        assert!(
            refused
                .as_ref()
                .is_err_and(|words| words.starts_with("Plan line not added. ")),
            "{refused:?}"
        );
        b_file(0);
        assert_eq!(
            sessions
                .journey_plan(&book, &road, &names)
                .get("Shrine of Cinders"),
            Some(false),
            "unsaved words are not taken for saved ones"
        );
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        read_only(true);
        let failed = sessions.try_plan_undo();
        read_only(false);
        // E2, R45 (c): B is not selected; its draft of its own was dropped, so the line is still
        // in the file. The answer says so, in the error tone, with its one step, by hand.
        assert_eq!(
            failed.as_deref(),
            Some(
                "The plan line for Shrine of Cinders was not removed from Session 1: Road. The \
                 session note could not be saved. Open that session’s notes and remove the plan \
                 line there."
            )
        );
        assert_eq!(sessions.plan_line_tone(Some(b)), Some(Tone::Error), "R41");
        assert_eq!(sessions.selected, Some(a), "R26: B is not selected");
        b_file(1);
        assert_eq!(
            sessions
                .journey_plan(&book, &road, &names)
                .get("Shrine of Cinders"),
            Some(true),
            "the line that stays in the file is in the plan"
        );
        assert_eq!(
            sessions.try_plan_undo(),
            failed,
            "R45: no Undo again; the press does nothing"
        );
        b_file(1);
        // Review 4: the plan's own `Undo add plan line`, with the journey's block closed, fails;
        // the plan's footer says why while the journey stands, and the fault ends with it.
        fs::write(&b_note.path, &text).unwrap();
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        assert_ne!(sessions.open_event.as_deref(), Some(road.as_str()));
        read_only(true);
        let said = sessions.try_plan_undo();
        read_only(false);
        assert!(
            said.as_deref()
                .is_some_and(|words| words.starts_with("The plan line for Shrine of Cinders ")),
            "{said:?}"
        );
        let without: Vec<_> = book
            .events
            .iter()
            .filter(|event| event.id != road)
            .cloned()
            .collect();
        let mut without_book = Book::default();
        without_book.events = without;
        without_book.touch();
        sessions.end_plan_answers_of_lost_journeys(&without_book);
        assert_eq!(sessions.plan_line_refusal(), None, "J left");
        // Review 3, point 2: B open with unsaved words, its file renamed outside; the scan
        // finds the new file. The act is refused, and nothing writes either file.
        fs::write(&b_note.path, &text).unwrap();
        sessions.select(Some(b));
        sessions
            .draft
            .as_mut()
            .unwrap()
            .body
            .push_str("Typed, not saved.\n");
        sessions.draft.as_mut().unwrap().edited();
        let moved = dir.path().join("Moved road.md");
        fs::rename(&b_note.path, &moved).unwrap();
        sessions.notes[1].path = moved.clone();
        let refused =
            sessions.plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names);
        assert!(
            refused
                .as_ref()
                .is_err_and(|words| words.starts_with("Plan line not added. Session 1")),
            "{refused:?}"
        );
        assert_eq!(
            fs::read_to_string(&moved).unwrap(),
            text,
            "no second writer"
        );
        assert!(!b_note.path.exists());
        fs::rename(&moved, &b_note.path).unwrap();
        sessions.notes[1].path = b_note.path.clone();
        sessions.draft = None;
        sessions.select(Some(a));
        // Final review point 1: B open, its plan line's save fails (the Undo waits in the
        // draft); write access comes back; selecting A saves B's draft with the line. Undo then
        // takes the line out of the file.
        fs::write(&b_note.path, &text).unwrap();
        sessions.select(Some(b));
        read_only(true);
        let unsaved =
            sessions.plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names);
        read_only(false);
        assert!(unsaved.is_err(), "{unsaved:?}");
        b_file(0);
        sessions.select(Some(a));
        b_file(1);
        assert_eq!(sessions.try_plan_undo(), None);
        b_file(0);
        // Final review point 2: planned for the journey in B while A is open. Wherever the
        // journey goes, A's plan shows neither B's result nor its Undo; B's keeps them, and B's
        // note keeps its line.
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        assert!(!sessions.plan_line_shown_in(Some(a)), "not in A's plan");
        assert!(sessions.plan_line_shown_in(Some(b)));
        b_file(1);
        // Final review point 4: a quiet frame checks no words.
        let checks = || timeline::page::PLAN_LINE_CHECKS.with(std::cell::Cell::get);
        sessions.plan_line_shown_in(Some(b));
        let before = checks();
        for _ in 0..3 {
            assert!(sessions.plan_line_shown_in(Some(b)));
        }
        assert_eq!(checks(), before, "the result's validity is cached");
        sessions.travel.planned = design::Results::new();
        // The open session is the journey's: the act goes through its draft, as on the plan.
        fs::write(&b_note.path, &text).unwrap();
        sessions.select(Some(b));
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap_or_else(|fault| panic!("{fault}"));
        assert!(
            sessions
                .draft
                .as_ref()
                .unwrap()
                .body
                .contains("[[Shrine of Cinders]]")
                && sessions.planned_live()
        );
        b_file(1);
        // Final review, last point: with only the Note tab shown (no view of the result drawn),
        // the DM types a letter, it saves, they delete it, it saves. Each frame's `poll` meets
        // the edit: the result ends, and the restored words do not bring it back to either
        // reader.
        let home_campaign = sessions.campaign.clone();
        let frame = |sessions: &mut Sessions| {
            sessions.last_scan = Some(Instant::now());
            sessions.poll(
                &egui::Context::default(),
                home_campaign.as_ref(),
                &mut Book::default(),
            );
        };
        frame(&mut sessions);
        assert!(!sessions.travel.planned.is_empty());
        let restored = sessions.draft.as_ref().unwrap().body.as_str().to_owned();
        for words in [format!("{restored}x"), restored.clone()] {
            let draft = sessions.draft.as_mut().unwrap();
            let _ = draft.body.replace(words);
            draft.edited();
            frame(&mut sessions);
            sessions.draft.as_mut().unwrap().save();
            frame(&mut sessions);
        }
        assert_eq!(sessions.draft.as_ref().unwrap().body.as_str(), restored);
        assert!(
            !sessions.plan_line_shown_in(Some(b)),
            "the plan offers no result"
        );
        // Timeline's footer (`plan_line_result`) shows the result only while this holds.
        assert!(!sessions.planned_live(), "Timeline offers no result");
        assert_eq!(sessions.try_plan_undo(), None, "no Undo");
        b_file(1);
        // Review 3, point 1: another campaign ends the result, and its Undo reaches no note.
        // A live result first (the edit cycle above ended the last one).
        sessions.select(Some(a));
        fs::write(&b_note.path, &text).unwrap();
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        assert!(sessions.planned_live(), "a live result before the change");
        sessions.campaign = Some(campaign("Other"));
        assert!(!sessions.planned_live(), "another campaign ends it");
        assert_eq!(sessions.try_plan_undo(), None);
        b_file(1);
        // So does the session's note leaving the folder.
        sessions.campaign = Some(campaign("Marches"));
        sessions.travel.planned = design::Results::new();
        sessions.select(Some(a));
        fs::write(&b_note.path, &text).unwrap();
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        assert!(sessions.planned_live());
        sessions.notes.retain(|note| note.id != b);
        sessions.notes_revision += 1;
        assert!(!sessions.planned_live());
        assert_eq!(sessions.try_plan_undo(), None);
        b_file(1);
        // Final review point 3: the campaign-change event itself ends the result: A, then B,
        // then A again, with no read of the result between.
        sessions.notes = vec![a_note.clone(), b_note.clone()];
        sessions.notes_revision += 1;
        fs::write(&b_note.path, &text).unwrap();
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        assert!(sessions.planned_live());
        let home = sessions.campaign.clone().unwrap();
        sessions.open_campaign(Some(&campaign("Other")));
        sessions.open_campaign(Some(&home));
        sessions.notes = vec![a_note.clone(), b_note.clone()];
        sessions.notes_revision += 1;
        sessions.last_check = Some(Instant::now());
        assert!(!sessions.planned_live(), "the result ended at the change");
        assert_eq!(sessions.try_plan_undo(), None);
        b_file(1);
        // E1: a refused Undo is owned by its note and journey. A's (B's note, the road) stays
        // through an act on another note's line, failed or successful, and both readers show
        // what is theirs; Undo of A after its source is restored takes A's words and data back
        // together, and the other line stays.
        let keep = book.add(Kind::Travel, Some(a));
        let road_checks = book.event_mut(&road).unwrap().checks.clone();
        book.event_mut(&keep).unwrap().checks = road_checks;
        let a_file = || fs::read_to_string(&a_note.path).unwrap();
        fs::write(&b_note.path, &text).unwrap();
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        let as_left = b_file(1);
        fs::write(&b_note.path, format!("{as_left}Outside.\n")).unwrap();
        sessions.last_check = Some(Instant::now());
        let refused = sessions.try_plan_undo_for(Some(b)).expect("A is refused");
        let before = a_file();
        read_only(true);
        let failed =
            sessions.plan_from_timeline(&book, &keep, &chosen("Shrine of Cinders"), &names);
        read_only(false);
        assert!(failed.is_err(), "{failed:?}");
        assert_eq!(a_file(), before);
        assert_eq!(
            sessions.plan_line_refusal_for(Some(b)).as_deref(),
            Some(refused.as_str()),
            "a failed act on another line keeps A's answer"
        );
        sessions
            .plan_from_timeline(&book, &keep, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        assert!(a_file().contains("[[Shrine of Cinders]]"));
        assert_eq!(
            sessions.plan_line_refusal_for(Some(b)).as_deref(),
            Some(refused.as_str()),
            "a successful act on another line keeps A's answer"
        );
        assert!(sessions.plan_line_shown_in(Some(a)) && sessions.plan_line_shown_in(Some(b)));
        // R45 (a): A's refused line offers no Undo again; a press writes nothing.
        let outside = fs::read_to_string(&b_note.path).unwrap();
        assert_eq!(
            sessions.try_plan_undo_for(Some(b)).as_deref(),
            Some(refused.as_str())
        );
        assert_eq!(fs::read_to_string(&b_note.path).unwrap(), outside);
        // N1: the same fight in two sessions: Timeline's two lines and their Undos name their
        // sessions.
        let painted = |sessions: &mut Sessions| -> Vec<(String, String)> {
            let campaign = sessions.campaign.as_ref().map(|campaign| campaign.id);
            sessions
                .travel
                .planned
                .newest_first(&campaign)
                .map(|shown| {
                    let (words, step) = plan_line_words(shown.value(), true);
                    (words.to_string(), step.label())
                })
                .collect()
        };
        let labels = painted(&mut sessions);
        assert_eq!(
            labels,
            [
                (
                    "“Shrine of Cinders” is in the plan of Session 2: Keep.".to_owned(),
                    "Undo add plan line for Shrine of Cinders in Session 2: Keep".to_owned()
                ),
                (
                    "“Shrine of Cinders” is in the plan of Session 1: Road.".to_owned(),
                    "Undo add plan line for Shrine of Cinders in Session 1: Road".to_owned()
                ),
            ]
        );
        // E5: with a refused answer kept, an unchanged frame looks up no owner and walks no
        // entry.
        let scans = || PLAN_OWNER_SCANS.with(std::cell::Cell::get);
        let walks = || PLAN_FOLLOW_WALKS.with(std::cell::Cell::get);
        sessions.follow_plan_line();
        sessions.end_plan_answers_of_lost_journeys(&book);
        let (still, walked) = (scans(), walks());
        for _ in 0..3 {
            sessions.follow_plan_line();
            sessions.end_plan_answers_of_lost_journeys(&book);
        }
        assert_eq!(scans(), still, "no owner scan on an unchanged frame");
        assert_eq!(
            walks(),
            walked,
            "no walk of the entries on an unchanged frame"
        );
        // R45 (e): both readers' lines, made on unchanged frames, walk no entry either; the
        // refused line offers no Undo, the other line does.
        let draw = |sessions: &mut Sessions| {
            let frame = PaneFrame::plain();
            let timeline = sessions.plan_line_kept(&frame, None, ());
            let plan = sessions.plan_line_kept(&frame, Some(Some(b)), ());
            assert_eq!((timeline.len(), plan.len()), (2, 1));
        };
        draw(&mut sessions);
        let walked = walks();
        for _ in 0..3 {
            draw(&mut sessions);
        }
        assert_eq!(
            walks(),
            walked,
            "no walk of the entries on an unchanged frame"
        );
        // R45 (a): the DM removes A's line by hand, as the answer asks: when A's note no longer
        // holds the line, A's answer ends. The other line stays.
        let by_hand: String = outside
            .lines()
            .filter(|line| !line.contains("[[Shrine of Cinders]]"))
            .map(|line| format!("{line}\n"))
            .collect();
        fs::write(&b_note.path, &by_hand).unwrap();
        sessions.last_check = Some(Instant::now());
        sessions.follow_plan_line();
        assert_eq!(
            sessions.plan_line_refusal_for(Some(b)),
            None,
            "the line is gone: it ends"
        );
        assert!(!sessions.plan_line_shown_in(Some(b)));
        assert!(
            a_file().contains("[[Shrine of Cinders]]"),
            "the other line's data stays"
        );
        assert!(sessions.plan_line_shown_in(Some(a)), "and its result");
        // E1: a refused answer is its note's: a rename of the note's file inside its folder,
        // found by a listing, keeps it (R45: its Undo is never offered again, so no file is
        // followed); an unrelated dirty draft stays as it was.
        fs::write(&b_note.path, &text).unwrap();
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        let as_left = b_file(1);
        fs::write(&b_note.path, format!("{as_left}Outside.\n")).unwrap();
        sessions.last_check = Some(Instant::now());
        let refused = sessions.try_plan_undo_for(Some(b)).expect("refused");
        sessions.select(Some(a));
        let dirty = format!(
            "{}Typed, not saved.\n",
            sessions.draft.as_ref().unwrap().body
        );
        let _ = sessions.draft.as_mut().unwrap().body.replace(dirty.clone());
        sessions.draft.as_mut().unwrap().edited();
        let renamed = dir.path().join("Renamed road.md");
        fs::rename(&b_note.path, &renamed).unwrap();
        sessions.notes[1].path = renamed.clone();
        sessions.notes_revision += 1;
        assert_eq!(
            sessions.plan_line_refusal_for(Some(b)).as_deref(),
            Some(refused.as_str()),
            "the rename keeps A's answer"
        );
        assert_eq!(sessions.draft.as_ref().unwrap().body.as_str(), dirty);
        assert!(
            sessions.draft.as_ref().unwrap().unsaved(),
            "the dirty draft is not saved"
        );
        fs::rename(&renamed, &b_note.path).unwrap();
        sessions.notes[1].path = b_note.path.clone();
        sessions.notes_revision += 1;
        let _ = sessions
            .draft
            .as_mut()
            .unwrap()
            .body
            .replace(dirty.replace("Typed, not saved.\n", ""));
        sessions.draft.as_mut().unwrap().save();
        // E6 (R41): an unreadable file is an error, a missing one a warning, by one rule that
        // both readers' lines take (`Shown::outcome`). Each is a fresh plan line's first Undo.
        let fresh_line = |sessions: &mut Sessions| {
            fs::write(&b_note.path, &text).unwrap();
            sessions
                .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
                .unwrap();
        };
        fresh_line(&mut sessions);
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = |mode| {
                fs::set_permissions(&b_note.path, fs::Permissions::from_mode(mode)).unwrap();
            };
            mode(0o000);
            let unreadable = sessions.try_plan_undo_for(Some(b));
            mode(0o644);
            assert!(
                unreadable
                    .as_deref()
                    .is_some_and(|words| words.contains("Splat cannot read the session note.")),
                "{unreadable:?}"
            );
            assert_eq!(sessions.plan_line_tone(Some(b)), Some(Tone::Error));
        }
        fresh_line(&mut sessions);
        let away = dir.path().join("Away.md");
        fs::rename(&b_note.path, &away).unwrap();
        let gone = sessions.try_plan_undo_for(Some(b));
        fs::rename(&away, &b_note.path).unwrap();
        assert!(
            gone.as_deref()
                .is_some_and(|words| words.contains("The session note is gone from its folder.")),
            "{gone:?}"
        );
        assert_eq!(sessions.plan_line_tone(Some(b)), Some(Tone::Warning));
        // R45 (b): B selected. Its plan line's Undo takes the line out of B's draft and is done,
        // also when the save then fails: that is B's own fault, which B's note warning says
        // (`Try to save again`), not a plan refusal. A's unrelated answer stays.
        let before_a = a_file();
        fs::write(&a_note.path, format!("{before_a}Outside A.\n")).unwrap();
        sessions.last_check = Some(Instant::now());
        let a_refused = sessions.try_plan_undo_for(Some(a)).expect("A is refused");
        fs::write(&b_note.path, &text).unwrap();
        sessions.select(Some(b));
        sessions
            .plan_from_timeline(&book, &road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        b_file(1);
        read_only(true);
        let undone = sessions.try_plan_undo_for(Some(b));
        read_only(false);
        assert_eq!(undone, None, "no plan refusal");
        assert!(!sessions.plan_line_shown_in(Some(b)), "the Undo is done");
        assert!(
            !sessions
                .draft
                .as_ref()
                .unwrap()
                .body
                .contains("[[Shrine of Cinders]]"),
            "the draft holds the reversal"
        );
        assert!(
            matches!(
                sessions.draft.as_ref().unwrap().state,
                crate::sessions::DraftState::Failed(_)
            ),
            "the note's own warning says the save failed"
        );
        assert_eq!(
            sessions.plan_line_refusal_for(Some(a)).as_deref(),
            Some(a_refused.as_str()),
            "A's answer stays"
        );
        sessions.draft.as_mut().unwrap().save();
        b_file(0);
        // R45 (d): two lines that read alike (the same fight, sessions of one label, journeys of
        // one label): each entry keeps what tells it apart, made at its act, and their Undos
        // paint two different labels at a narrow footer.
        let twin = new_note(1, "Road");
        let twin_road = book.add(Kind::Travel, Some(twin.id));
        book.event_mut(&twin_road).unwrap().checks = book.event_mut(&road).unwrap().checks.clone();
        sessions.notes.push(twin.clone());
        sessions.notes_revision += 1;
        sessions.select(Some(a));
        fs::write(&b_note.path, &text).unwrap();
        for journey in [&road, &twin_road] {
            sessions
                .plan_from_timeline(&book, journey, &chosen("Shrine of Cinders"), &names)
                .unwrap();
        }
        let campaign = sessions.campaign.as_ref().map(|campaign| campaign.id);
        let painted: Vec<String> = sessions
            .travel
            .planned
            .newest_first(&campaign)
            .filter(|shown| [b, twin.id].contains(&shown.value().note))
            .map(|shown| plan_line_words(shown.value(), true).1.label())
            .collect();
        assert_eq!(painted.len(), 2, "{painted:?}");
        assert_ne!(painted[0], painted[1], "{painted:?}");
        // Review N6: a third alike line, from a journey titled like another line's numbered
        // words (`Travel · 2`), gets words of its own; the kept lines keep theirs.
        let kept = |sessions: &Sessions, notes: &[Uuid]| -> Vec<Option<String>> {
            let campaign = sessions.campaign.as_ref().map(|campaign| campaign.id);
            sessions
                .travel
                .planned
                .newest_first(&campaign)
                .filter(|shown| notes.contains(&shown.value().note))
                .map(|shown| shown.value().apart.clone())
                .collect()
        };
        let before = kept(&sessions, &[b, twin.id]);
        let third = new_note(1, "Road");
        let third_road = book.add(Kind::Travel, Some(third.id));
        book.event_mut(&third_road).unwrap().checks = book.event_mut(&road).unwrap().checks.clone();
        book.event_mut(&third_road).unwrap().title = "Travel · 2".into();
        sessions.notes.push(third.clone());
        sessions.notes_revision += 1;
        sessions
            .plan_from_timeline(&book, &third_road, &chosen("Shrine of Cinders"), &names)
            .unwrap();
        let all = kept(&sessions, &[b, twin.id, third.id]);
        assert_eq!(all.len(), 3, "{all:?}");
        assert!(
            all[0].is_some() && !all[1..].contains(&all[0]),
            "the third line's words are its own: {all:?}"
        );
        assert_eq!(all[1..], before[..], "the kept lines keep their words");
        assert_eq!(
            told_apart(
                "Travel · 2",
                &[("Travel", 1, None), ("Travel", 2, Some("Travel · 2"))]
            ),
            (Some("Travel · 2 · 2".to_owned()), 2)
        );
    }

    /// Early review F1 of PR 444 (ruling L9): the Add result of a long rest names the fights
    /// that keep their saved HP. Its kept line follows the saved runs' facts by their own
    /// revision: the DM deletes the saved run (the book does not change), and the clause goes
    /// while the rest's result and its Undo stay; Undo of the deletion brings the clause back.
    /// Ported from the egui plan test to the plan's acts and plain footer lines.
    #[test]
    fn long_rest_added_on_plan_restores_and_undo_removes_its_ruling() {
        use crate::design::memo::{Changed, Revised};
        use crate::encounter_run::{SavedRunInfo, Standing};
        use crate::processing::settings::ProcessingSettings;
        use crate::sessions::page::gpui_facade::PlanAct;
        let (session, run) = (Uuid::new_v4(), Uuid::new_v4());
        let mut book = Book::default();
        book.add(Kind::Encounter, Some(session));
        book.events[0].title = "Bridge".into();
        book.events[0].run = Some(run);
        let saved = SavedRunInfo {
            path: PathBuf::from("bridge-run.json"),
            id: Some(run),
            source: None,
            encounter: "Bridge".into(),
            label: "Round 2".into(),
            standing: Standing::Unfinished,
            setup: false,
            saved_at: None,
            unsent: Vec::new(),
        };
        let mut page = Sessions {
            selected: Some(session),
            ..Default::default()
        };
        let settings = ProcessingSettings::default();
        let ctx = egui::Context::default();
        let mut runs = Revised::new(vec![saved.clone()]);
        // The app's frame: the owner update, then the plan's footer.
        let footer = |page: &mut Sessions, book: &mut Book, runs: &Revised<Vec<SavedRunInfo>>| {
            page.settle_timed(&ctx, book);
            with_bare_plan(runs, runs.revision(), None, |plan| {
                page.gv_plan_footer(&settings, book, plan, None, false)
            })
        };
        let keeps = |lines: &[crate::sessions::page::gpui_facade::FooterLine]| {
            lines
                .iter()
                .any(|line| line.words.contains("Bridge keeps its saved HP."))
        };
        let rest_stands = |lines: &[crate::sessions::page::gpui_facade::FooterLine]| {
            lines.iter().any(|line| {
                line.words.contains("Long rest added.")
                    && line
                        .undo
                        .as_ref()
                        .is_some_and(|undo| undo.label == "Undo add")
            })
        };
        let _ = footer(&mut page, &mut book, &runs);
        with_bare_plan(&runs, runs.revision(), None, |plan| {
            page.gv_plan_act(PlanAct::Add(Kind::LongRest), &mut book, plan, None, &ctx);
        });
        let lines = footer(&mut page, &mut book, &runs);
        assert!(
            rest_stands(&lines) && keeps(&lines),
            "the rest's result names the fight that keeps its HP"
        );
        let book_revision = book.revision();
        runs.revise(|runs| {
            runs.clear();
            Changed::Yes
        });
        let lines = footer(&mut page, &mut book, &runs);
        assert_eq!(
            book.revision(),
            book_revision,
            "deleting a saved run leaves the book as it was"
        );
        assert!(
            !keeps(&lines),
            "a deleted saved run keeps no HP: the clause goes"
        );
        assert!(
            rest_stands(&lines),
            "the unrelated deletion leaves the rest's result and its Undo"
        );
        runs.revise(|runs| {
            runs.push(saved.clone());
            Changed::Yes
        });
        let lines = footer(&mut page, &mut book, &runs);
        assert_eq!(book.revision(), book_revision);
        assert!(
            keeps(&lines) && rest_stands(&lines),
            "Undo of the deletion brings the clause back"
        );
    }

    /// A Timeline fight resumes by its run's ID, with no planned source: the row offers its
    /// paused run, and `Resume` asks for that run's file (not another run of the same fight).
    /// An ended or unreadable run, or the run already open, offers none. Ported from the egui
    /// plan test to the plan's rows and acts.
    #[test]
    fn iter5_c_timeline_fight_resumes_by_run_id_without_a_planned_source() {
        use crate::encounter_run::{SavedRunInfo, Standing};
        use crate::sessions::page::gpui_facade::{PlanAct, PlanRow};
        let session = Uuid::new_v4();
        let run = Uuid::new_v4();
        let mut book = Book::default();
        book.add(Kind::Encounter, Some(session));
        book.events[0].title = "Bridge Ambush".into();
        book.events[0].result = "Round 2".into();
        book.events[0].run = Some(run);
        let saved = SavedRunInfo {
            path: PathBuf::from("source-deleted/run.json"),
            id: Some(run),
            source: None,
            encounter: "Bridge Ambush".into(),
            label: "Round 2".into(),
            standing: Standing::Unfinished,
            setup: false,
            saved_at: None,
            unsent: vec!["Kestrel".into()],
        };
        let mut other = saved.clone();
        other.id = Some(Uuid::new_v4());
        other.path = PathBuf::from("other-run.json");
        let mut page = Sessions {
            selected: Some(session),
            ..Default::default()
        };
        let mut runs = vec![other, saved.clone()];
        let ctx = egui::Context::default();
        let mut revisions = crate::design::memo::Revised::new(());
        let resume = |page: &mut Sessions,
                      book: &mut Book,
                      runs: &[SavedRunInfo],
                      open: Option<Uuid>,
                      revision| {
            with_bare_plan(runs, revision, open, |plan| {
                page.gv_plan(book, plan, None)
                    .rows
                    .into_iter()
                    .find_map(|row| match row {
                        PlanRow::Event(event) if event.label == "Bridge Ambush" => event.saved,
                        _ => None,
                    })
                    .map(|saved| saved.path)
            })
        };
        let path = resume(&mut page, &mut book, &runs, None, revisions.revision())
            .expect("the row offers its paused run");
        assert_eq!(path, saved.path, "the run by its ID, not the other run");
        with_bare_plan(&runs, revisions.revision(), None, |plan| {
            page.gv_plan_act(PlanAct::Resume(path), &mut book, plan, None, &ctx);
        });
        assert_eq!(page.wants_saved_run.as_ref(), Some(&saved.path));
        for state in [Standing::Ended, Standing::Unreadable] {
            runs[1].standing = state;
            revisions.revise(|_| crate::design::memo::Changed::Yes);
            let revision = revisions.revision();
            assert_eq!(
                resume(&mut page, &mut book, &runs, None, revision),
                None,
                "{state:?}"
            );
        }
        runs[1].standing = Standing::Unfinished;
        revisions.revise(|_| crate::design::memo::Changed::Yes);
        let revision = revisions.revision();
        assert_eq!(
            resume(&mut page, &mut book, &runs, Some(run), revision),
            None,
            "the open run is not resumed"
        );
    }

    /// A plan with these saved runs and nothing else, all its revisions `revision`: the
    /// Session page's view of the rest of the app in a plan test.
    fn with_bare_plan<R>(
        saved_runs: &[crate::encounter_run::SavedRunInfo],
        revision: crate::design::memo::Revision,
        open_run: Option<Uuid>,
        act: impl FnOnce(&Plan<'_>) -> R,
    ) -> R {
        let plan = Plan {
            fights: &[],
            saved_runs,
            saved_runs_revision: revision,
            fights_revision: revision,
            groups: &[],
            group_blocked: None,
            group_open: false,
            group_discard: &|_| None,
            arrivals_open: false,
            blocked: None,
            run_directory: None,
            open_run,
            open_state: "Round 2".into(),
            open_encounter: None,
            party: &[],
            tables: &[],
        };
        act(&plan)
    }
}
