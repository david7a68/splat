//! What the Encounters page shows where: what is in play, what is ready, and what has happened.
//! An encounter leaves Ready when a run of it ends, so nothing is written to its note, and an
//! undo of the ending brings it back. The story's sessions come from the timeline.
use crate::{
    encounter_run::{SavedRunInfo, Standing},
    timeline::{Event, page::SessionLink},
};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

/// One encounter note, as the page lists it.
pub(super) struct Template {
    pub path: String,
    pub id: Uuid,
    pub name: String,
    pub summary: Option<String>,
    /// It stays ready after a run ends.
    pub repeatable: bool,
}

pub(super) struct ReadyRow<'a> {
    pub template: &'a Template,
    /// A run of it has not ended, so starting it again is probably not what the DM means.
    pub unfinished_run: bool,
}

pub(super) struct PastRun<'a> {
    pub run: &'a SavedRunInfo,
    /// The encounter to run again or edit, if it is still there.
    pub template: Option<&'a Template>,
}

/// Ended runs under one heading.
pub(super) struct PastSession<'a> {
    pub label: &'a str,
    pub runs: Vec<PastRun<'a>>,
}

#[derive(Default)]
pub(super) struct Arranged<'a> {
    /// The open run first, whatever its phase, so the way back to it is always here.
    pub in_progress: Vec<&'a SavedRunInfo>,
    pub ready: Vec<ReadyRow<'a>>,
    pub history: Vec<PastSession<'a>>,
    pub unreadable: Vec<&'a SavedRunInfo>,
}

impl Arranged<'_> {
    pub fn ended_count(&self) -> usize {
        self.history.iter().map(|session| session.runs.len()).sum()
    }
}

const NO_SESSION: &str = "No session";

/// The template a run came from. A save with no record of it matches by name, but only when
/// one template has that name: entries can share a name, and a wrong match would hide one.
fn template_of<'a>(run: &SavedRunInfo, templates: &'a [Template]) -> Option<&'a Template> {
    if let Some(source) = run.source {
        return templates.iter().find(|template| template.id == source);
    }
    let mut named = templates
        .iter()
        .filter(|template| template.name == run.encounter);
    let first = named.next()?;
    named.next().is_none().then_some(first)
}

/// `runs` is the list on disk, newest first. `open` is the open run as it is now. It replaces
/// its own file's row, and it may have no file in the list yet.
pub(super) fn arrange<'a>(
    templates: &'a [Template],
    runs: &'a [SavedRunInfo],
    open: Option<&'a SavedRunInfo>,
    events: &[Event],
    sessions: &'a [SessionLink],
) -> Arranged<'a> {
    let on_disk = runs
        .iter()
        .filter(|run| open.is_none_or(|open| run.id != open.id));
    let mut page = Arranged::default();
    page.in_progress.extend(open);
    let (mut ended, mut unfinished) = (BTreeSet::new(), BTreeSet::new());
    if let Some(open) = open
        && let Some(template) = template_of(open, templates)
    {
        match open.standing {
            Standing::Ended => ended.insert(template.id),
            _ => unfinished.insert(template.id),
        };
    }
    let mut past = Vec::new();
    for run in on_disk {
        let template = template_of(run, templates);
        match run.standing {
            Standing::Unreadable => page.unreadable.push(run),
            Standing::Unfinished => {
                page.in_progress.push(run);
                unfinished.extend(template.map(|template| template.id));
            }
            Standing::Ended => {
                ended.extend(template.map(|template| template.id));
                past.push(PastRun { run, template });
            }
        }
    }
    page.ready = templates
        .iter()
        .filter(|template| template.repeatable || !ended.contains(&template.id))
        .map(|template| ReadyRow {
            template,
            unfinished_run: unfinished.contains(&template.id),
        })
        .collect();
    page.history = by_session(past, events, sessions);
    page
}

/// Sessions newest first, and in each one the latest event first. The rest go last, newest
/// first: a run with no event, an event with no session, or a session whose note is gone.
fn by_session<'a>(
    past: Vec<PastRun<'a>>,
    events: &[Event],
    sessions: &'a [SessionLink],
) -> Vec<PastSession<'a>> {
    let place: BTreeMap<Uuid, (Uuid, usize)> = events
        .iter()
        .enumerate()
        .filter_map(|(index, event)| Some((event.run?, (event.session?, index))))
        .filter(|(_, (session, _))| sessions.iter().any(|link| link.id == *session))
        .collect();
    let mut placed: BTreeMap<Uuid, Vec<(usize, PastRun<'a>)>> = BTreeMap::new();
    let mut rest = Vec::new();
    for run in past {
        match run.run.id.and_then(|id| place.get(&id)) {
            Some(&(session, index)) => placed.entry(session).or_default().push((index, run)),
            None => rest.push(run),
        }
    }
    let mut history = Vec::new();
    for link in sessions {
        if let Some(mut runs) = placed.remove(&link.id) {
            runs.sort_by_key(|(index, _)| std::cmp::Reverse(*index));
            history.push(PastSession {
                label: &link.label,
                runs: runs.into_iter().map(|(_, run)| run).collect(),
            });
        }
    }
    if !rest.is_empty() {
        history.push(PastSession {
            label: NO_SESSION,
            runs: rest,
        });
    }
    history
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timeline::Kind;
    use std::path::PathBuf;

    fn template(name: &str) -> Template {
        Template {
            path: format!("Encounters/{name}.md"),
            id: Uuid::new_v4(),
            name: name.into(),
            summary: None,
            repeatable: false,
        }
    }
    fn run(from: &Template, standing: Standing) -> SavedRunInfo {
        SavedRunInfo {
            source: Some(from.id),
            ..old_run(&from.name, standing)
        }
    }
    /// A save from before runs recorded their encounter.
    fn old_run(name: &str, standing: Standing) -> SavedRunInfo {
        SavedRunInfo {
            path: PathBuf::from(format!("{name}.json")),
            id: Some(Uuid::new_v4()),
            source: None,
            encounter: name.into(),
            label: String::new(),
            standing,
            saved_at: None,
            unsent: Vec::new(),
        }
    }
    fn event(run: &SavedRunInfo, session: Option<Uuid>) -> Event {
        Event {
            id: Uuid::new_v4().to_string(),
            kind: Kind::Encounter,
            title: String::new(),
            duration: String::new(),
            place: String::new(),
            notes: String::new(),
            result: String::new(),
            session,
            run: run.id,
            marker: String::new(),
        }
    }
    fn link(label: &str) -> SessionLink {
        SessionLink {
            id: Uuid::new_v4(),
            label: label.into(),
            mentions: Vec::new(),
        }
    }
    fn ready_names<'a>(page: &'a Arranged<'a>) -> Vec<&'a str> {
        page.ready
            .iter()
            .map(|row| row.template.name.as_str())
            .collect()
    }

    #[test]
    fn an_ended_run_moves_its_encounter_out_of_ready() {
        let templates = [template("Bridge"), template("Mill")];
        let runs = [run(&templates[0], Standing::Ended)];
        let page = arrange(&templates, &runs, None, &[], &[]);
        assert_eq!(ready_names(&page), ["Mill"]);
        assert_eq!(page.ended_count(), 1);
    }

    #[test]
    fn an_unfinished_run_comes_first_and_leaves_its_encounter_ready() {
        let templates = [template("Bridge")];
        let runs = [run(&templates[0], Standing::Unfinished)];
        let page = arrange(&templates, &runs, None, &[], &[]);
        assert_eq!(page.in_progress.len(), 1);
        assert_eq!(ready_names(&page), ["Bridge"]);
        assert!(page.ready[0].unfinished_run);
    }

    #[test]
    fn a_repeatable_encounter_stays_ready() {
        let mut road = template("Road");
        road.repeatable = true;
        let templates = [road];
        let runs = [run(&templates[0], Standing::Ended)];
        let page = arrange(&templates, &runs, None, &[], &[]);
        assert_eq!(ready_names(&page), ["Road"]);
        assert_eq!(page.ended_count(), 1, "its past runs are still history");
    }

    /// A new run is in no list yet. An ending, and its undo, show before the run closes.
    #[test]
    fn the_open_run_counts_as_it_is_now_and_not_as_its_file_says() {
        let templates = [template("Bridge")];
        let saved = run(&templates[0], Standing::Ended);

        let mut live = run(&templates[0], Standing::Unfinished);
        live.id = saved.id;
        let runs = [saved];
        let page = arrange(&templates, &runs, Some(&live), &[], &[]);
        assert_eq!(ready_names(&page), ["Bridge"], "undo brings it back");
        assert_eq!(page.ended_count(), 0);
        assert_eq!(page.in_progress.len(), 1, "one row, not two");

        let brand_new = run(&templates[0], Standing::Ended);
        let other = [run(&templates[0], Standing::Unfinished)];
        let page = arrange(&templates, &other, Some(&brand_new), &[], &[]);
        assert!(ready_names(&page).is_empty(), "it ended while open");
        assert_eq!(page.in_progress.len(), 2);
        assert_eq!(
            page.in_progress[0].id, brand_new.id,
            "the way back stays first"
        );
        assert!(
            page.in_progress[1..]
                .iter()
                .all(|run| run.id == other[0].id),
            "then the rest"
        );
        assert_eq!(page.ended_count(), 0);
    }

    #[test]
    fn an_old_save_matches_by_name_only_when_one_encounter_has_it() {
        let single = [template("Bridge")];
        let runs = [old_run("Bridge", Standing::Ended)];
        let page = arrange(&single, &runs, None, &[], &[]);
        assert!(ready_names(&page).is_empty());
        assert!(page.history[0].runs[0].template.is_some());

        let twins = [template("New encounter"), template("New encounter")];
        let runs = [old_run("New encounter", Standing::Ended)];
        let page = arrange(&twins, &runs, None, &[], &[]);
        assert_eq!(page.ready.len(), 2, "a shared name proves nothing");
        assert!(page.history[0].runs[0].template.is_none());
    }

    #[test]
    fn a_run_whose_encounter_is_gone_does_not_borrow_another_by_name() {
        let gone = template("Bridge");
        let templates = [template("Bridge")];
        let runs = [run(&gone, Standing::Ended)];
        let page = arrange(&templates, &runs, None, &[], &[]);
        assert_eq!(ready_names(&page), ["Bridge"]);
        assert!(page.history[0].runs[0].template.is_none());
    }

    #[test]
    fn unreadable_runs_stand_apart() {
        let runs = [SavedRunInfo {
            id: None,
            ..old_run("", Standing::Unreadable)
        }];
        let page = arrange(&[], &runs, None, &[], &[]);
        assert_eq!(page.unreadable.len(), 1);
        assert!(page.in_progress.is_empty());
        assert_eq!(page.ended_count(), 0);
    }

    #[test]
    fn history_follows_the_sessions_and_the_order_of_their_events() {
        let t = template("Bridge");
        let [first, second, later, no_event, no_session, lost_note] =
            std::array::from_fn(|_| run(&t, Standing::Ended));
        let (newer, older) = (link("Session 2"), link("Session 1"));
        let events = [
            event(&first, Some(older.id)),
            event(&second, Some(older.id)),
            event(&later, Some(newer.id)),
            event(&no_session, None),
            event(&lost_note, Some(Uuid::new_v4())),
        ];
        // The list on disk has its own order, and the timeline's story decides.
        let runs = [first, later, second, no_event, no_session, lost_note];
        let sessions = [newer, older];
        let templates = [t];
        let page = arrange(&templates, &runs, None, &events, &sessions);
        let words: Vec<_> = page
            .history
            .iter()
            .map(|session| (session.label, session.runs.len()))
            .collect();
        assert_eq!(
            words,
            [("Session 2", 1), ("Session 1", 2), (NO_SESSION, 3)],
            "a session whose note is gone keeps its runs in view"
        );
        assert_eq!(
            page.history[1].runs[0].run.id, runs[2].id,
            "the later event first"
        );
    }
}
