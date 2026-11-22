use crate::encounter_run::SavedRunInfo;
use uuid::Uuid;

/// One encounter, as a session's plan offers it.
pub struct PlanFight {
    pub saved: Option<crate::encounter_run::SavedRunInfo>,
    /// The encounter's ID, which `Edit encounter` opens it by.
    pub id: uuid::Uuid,
    pub name: String,
    pub path: String,
    /// Its roster's typed parts (G118), so the plan's row names a member once.
    pub summary: crate::difficulty::Summary,
    /// The roster's revision (`Roster::revision`), for the plan's kept line; `None` with no
    /// roster.
    pub revision: Option<splat_design::ownership::Revision>,
    pub difficulty: crate::difficulty::Difficulty,
}
/// One encounter note, as the page lists it.
pub struct Template {
    pub path: String,
    pub id: Uuid,
    pub name: String,
    /// Its roster's summary, worked out in memory from its saved setup (never written).
    pub summary: Option<Roster>,
    /// How hard its saved roster is for the party. Shown only while it is ready to run.
    pub difficulty: crate::difficulty::Difficulty,
    /// It stays in Ready while a run is unfinished, with Start fight blocked.
    pub repeatable: bool,
}

/// G118: an encounter's roster summary as the list holds it. The typed parts go to its
/// difficulty line (so a member the summary shows is not named again after `XP not known`); the
/// same parts as words are its hover and the plan's text. Built once, where the template is
/// built from its setup; nothing of it is read from or written to a file.
pub struct Roster {
    pub parts: crate::difficulty::Summary,
    pub words: String,
    /// The row's hover: the words, then the difficulty's form (`… · XP not known`).
    pub hover: String,
    /// The Ready row's and the plan's kept lines are keyed by it (ruling M6): a new revision
    /// for each build with its parts and the fight's difficulty, so a rebuilt roster (a new
    /// read, another party) is fitted again, and an unchanged frame never is.
    pub revision: splat_design::ownership::Revision,
}

impl Roster {
    pub fn new(
        parts: crate::difficulty::Summary,
        difficulty: &crate::difficulty::Difficulty,
    ) -> Self {
        let words = parts.text();
        Self {
            hover: match difficulty.form() {
                Some(form) if !words.is_empty() => format!("{words} · {form}"),
                _ => words.clone(),
            },
            words,
            // A roster is built whole and never changed after: its build is its one revision.
            revision: splat_design::ownership::Revised::new(()).revision(),
            parts,
        }
    }
}

/// The template a run came from. A save with no record of it matches by name, but only when
/// one template has that name: entries can share a name, and a wrong match would hide one.
pub fn template_of<'a>(run: &SavedRunInfo, templates: &'a [Template]) -> Option<&'a Template> {
    if let Some(source) = run.source {
        return templates.iter().find(|template| template.id == source);
    }
    let name = run
        .encounter
        .split_once(" · saved ")
        .map_or(run.encounter.as_str(), |(name, _)| name);
    let mut named = templates.iter().filter(|template| template.name == name);
    let first = named.next()?;
    named.next().is_none().then_some(first)
}

/// Read the state from old and current run labels without exposing round zero.
pub fn run_state_word(run: &SavedRunInfo, paused: bool) -> String {
    let round = run
        .label
        .split(" · ")
        .find_map(|part| part.strip_prefix("Round "))
        .and_then(|part| part.parse::<u32>().ok())
        .unwrap_or(0);
    let state = if run.standing == crate::encounter_run::Standing::Ended {
        if round == 0 {
            "Ended · Setup".to_owned()
        } else {
            format!("Ended · Round {round}")
        }
    } else if round == 0 {
        "Setup".to_owned()
    } else {
        format!("Round {round}")
    };
    if paused && run.standing == crate::encounter_run::Standing::Unfinished {
        format!("{state} · Paused")
    } else {
        state
    }
}
