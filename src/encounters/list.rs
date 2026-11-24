//! The encounter ledger and its local editor controls.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::*;
use crate::encounter_run::{FightSummary, SavedRunInfo};
use std::path::Path;

/// Why a Ready row's Start fight is blocked while its run is open (FJ7), and the link to it.
pub const FIGHT_OPEN: &str = "This fight is open.";

/// A Ready row's second line: its roster's words.
pub(super) fn ready_detail(template: &Template) -> &str {
    template
        .summary
        .as_ref()
        .map(|roster| roster.words.as_str())
        .filter(|words| !words.is_empty())
        .unwrap_or("No participants yet")
}

/// FJ7: whether a Ready row's run is open, and the short reason its Start is blocked; the long
/// form is on Start's hover. An open run also gets its link, at the reason.
pub(super) fn ready_reason(
    page: &history::Arranged<'_>,
    row: &history::ReadyRow<'_>,
    live: Option<&SavedRunInfo>,
) -> (bool, Option<&'static str>) {
    let template = row.template;
    let unfinished = page.in_progress.iter().find(|run| {
        run.source == Some(template.id) || (run.source.is_none() && run.encounter == template.name)
    });
    let open = unfinished.is_some_and(|run| live.is_some_and(|live| live.id == run.id));
    let reason = if open {
        Some(FIGHT_OPEN)
    } else if row.unfinished_run {
        Some("This fight is unfinished.")
    } else if template
        .summary
        .as_ref()
        .is_none_or(|roster| roster.words.is_empty())
    {
        Some("Add participants first.")
    } else {
        None
    };
    (open, reason)
}

/// A History row's second line: the outcome, the state, the date, and a deleted encounter.
pub(super) fn history_detail(past: &history::PastRun<'_>) -> String {
    let run = past.run;
    let date = run
        .saved_at
        .map(|at| crate::encounter_run::saved_time(at, crate::hermetic::now()))
        .unwrap_or_else(|| "Date unknown".to_owned());
    let outcome = run
        .label
        .split(" · Ended")
        .next()
        .filter(|_| !run.label.starts_with("Ended"));
    format!(
        "{}{} · {date}{}",
        outcome.map_or_else(String::new, |outcome| format!("{outcome} · ")),
        run_state_word(run, false),
        if past.template.is_none() {
            " · Encounter deleted"
        } else {
            ""
        }
    )
}

pub use splat_session_records::encounters::run_state_word;

/// A duplicate name uses its session first, and has only one save time.
pub(super) fn paused_run_detail(
    run: &SavedRunInfo,
    runs: &[&SavedRunInfo],
    open: bool,
    story: &Story<'_>,
) -> String {
    let session_of = |run: &SavedRunInfo| {
        story
            .events
            .iter()
            .find(|event| event.run == run.id && run.id.is_some())
            .and_then(|event| event.session)
            .and_then(|id| story.sessions.iter().find(|session| session.id == id))
    };
    let duplicates = runs
        .iter()
        .filter(|other| other.encounter == run.encounter)
        .collect::<Vec<_>>();
    let session = session_of(run);
    let mut parts = if open {
        Vec::new()
    } else {
        run.label
            .trim_end_matches('.')
            .split(" · ")
            .filter(|part| !part.is_empty() && !part.starts_with("Saved "))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    let needs_date = duplicates.len() > 1
        && (session.is_none()
            || duplicates.iter().any(|other| {
                other.path != run.path
                    && session_of(other).map(|session| session.id)
                        == session.map(|session| session.id)
            }));
    if duplicates.len() > 1
        && let Some(session) = session
    {
        parts.push(session.label.clone());
    }
    if needs_date {
        let time = run
            .saved_at
            .map(|at| {
                let same_day = duplicates.iter().any(|other| {
                    other.path != run.path
                        && other
                            .saved_at
                            .is_some_and(|other| other / 86_400 == at / 86_400)
                });
                if same_day {
                    splat_party::party::calendar_time(at)
                } else {
                    crate::encounter_run::calendar_date(at)
                }
            })
            .unwrap_or_else(|| "Save time unknown".into());
        parts.push(time);
    } else if let Some(at) = run.saved_at {
        parts.push(format!(
            "Saved {}",
            crate::encounter_run::saved_time(at, crate::hermetic::now())
        ));
    } else if open {
        parts.push("Save time unknown".into());
    }
    parts.join(" · ")
}

/// Whether a run of this encounter is saved: unfinished, ended, or in a file that cannot be read
/// but names it. Only an encounter with none can be removed.
pub(super) fn has_saved_run(page: &history::Arranged<'_>, row: &history::ReadyRow<'_>) -> bool {
    let template = row.template;
    row.unfinished_run
        || page
            .history
            .iter()
            .flat_map(|session| &session.runs)
            .any(|past| {
                past.template
                    .is_some_and(|other| other.path == template.path)
            })
        || page.unreadable.iter().any(|run| {
            run.source == Some(template.id) || run.encounter.eq_ignore_ascii_case(&template.name)
        })
}

/// The ended run read in History.
pub(super) struct Reading<'a> {
    pub path: &'a Path,
    pub summary: &'a FightSummary,
    /// Why it cannot go on, when it is not the fight that was played last.
    pub not_last: Option<&'static str>,
}

#[cfg(test)]
mod iter_2_c_tests {
    use super::*;
    use crate::timeline::{Book, Kind, SessionLink};
    use std::path::PathBuf;

    /// T12.12: Remove is offered only for an encounter with no saved run: none unfinished,
    /// none ended, and no unreadable run that names it.
    #[test]
    fn remove_is_offered_only_for_an_encounter_with_no_saved_run() {
        use crate::encounter_run::Standing;
        let template = |name: &str| Template {
            path: format!("Encounters/{name}.md"),
            id: Uuid::new_v4(),
            name: name.into(),
            summary: Some(super::super::history::Roster::new(
                crate::difficulty::Summary {
                    members: vec![crate::difficulty::Member::new(1, "Wolf", false)],
                    ..Default::default()
                },
                &Default::default(),
            )),
            difficulty: Default::default(),
            repeatable: true,
        };
        let templates = ["Fresh", "Paused", "Ended", "Unreadable"].map(template);
        let run = |at: usize, standing| SavedRunInfo {
            path: PathBuf::from(format!("run-{at}.json")),
            id: Some(Uuid::new_v4()),
            source: Some(templates[at].id),
            encounter: templates[at].name.clone(),
            label: String::new(),
            standing,
            setup: false,
            saved_at: None,
            unsent: vec![],
        };
        let mut unreadable = run(3, Standing::Unreadable);
        unreadable.source = None;
        let runs = [
            run(1, Standing::Unfinished),
            run(2, Standing::Ended),
            unreadable,
        ];
        let page = history::arrange(&templates, &runs, None, &[], &[]);
        let offered: Vec<_> = page
            .ready
            .iter()
            .filter(|row| !has_saved_run(&page, row))
            .map(|row| row.template.name.as_str())
            .collect();
        assert_eq!(offered, ["Fresh"]);
    }

    #[test]
    fn paused_duplicates_use_session_then_one_save_time() {
        let runs = [1, 2].map(|n| SavedRunInfo {
            path: PathBuf::from(format!("run-{n}.json")),
            id: Some(Uuid::from_u128(n)),
            source: None,
            encounter: "Bridge Ambush".into(),
            label: "Setup · Paused · Last: Added a wolf · Saved just now".into(),
            standing: crate::encounter_run::Standing::Unfinished,
            setup: true,
            saved_at: Some(1_767_229_200 + n as u64 * 60),
            unsent: vec![],
        });
        let refs = runs.iter().collect::<Vec<_>>();
        let sessions = [1, 2].map(|n| SessionLink {
            id: Uuid::from_u128(n),
            label: format!("Session {n}"),
            mentions: vec![],
        });
        let mut book = Book::default();
        for (run, session) in runs.iter().zip(&sessions) {
            book.add(Kind::Encounter, Some(session.id));
            book.events.last_mut().unwrap().run = run.id;
        }
        let detail = paused_run_detail(
            &runs[0],
            &refs,
            false,
            &Story {
                events: &book.events,
                sessions: &sessions,
                revision: None,
            },
        );
        assert!(detail.contains("Session 1"));
        assert_eq!(detail.matches("Saved ").count(), 1);
        assert!(!detail.contains("2026-01-01"));
        assert!(detail.contains("Last: Added a wolf"));
        book.events[1].session = Some(sessions[0].id);
        let detail = paused_run_detail(
            &runs[0],
            &refs,
            false,
            &Story {
                events: &book.events,
                sessions: &sessions,
                revision: None,
            },
        );
        assert!(detail.contains("Session 1 · 2026-01-01"));
        assert!(!detail.contains("Saved "));
        let detail = paused_run_detail(
            &runs[0],
            &refs,
            false,
            &Story {
                events: &[],
                sessions: &[],
                revision: None,
            },
        );
        assert_eq!(detail.matches("2026-01-01").count(), 1);
        assert!(!detail.contains("Saved "));
        let detail = paused_run_detail(
            &runs[0],
            &[&runs[0]],
            false,
            &Story {
                events: &[],
                sessions: &[],
                revision: None,
            },
        );
        assert_eq!(detail.matches("Saved ").count(), 1);
        assert!(!detail.contains("2026-01-01"));
    }
}
