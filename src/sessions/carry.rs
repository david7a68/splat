//! Carry-over (F5 of `docs/design/session-cycle.md`): what the last session left, added to this
//! session's note in one act with its exact Undo; and an embed of a session note's heading,
//! which Read draws by its target.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{Plan, Sessions, prose};
use crate::{
    content_workspace::patch_fields,
    design::{self, Action, Tone, Type, colors},
    encounter_run::RunGroup,
    encounters::PlanFight,
    sessions::{
        ActFault, Note, NoteUndo, body_of, each_linked_name, list_recordings, properties_yaml,
        section_text, split_note, task_parts, ticked,
    },
    timeline::Book,
};
use eframe::egui;
use serde::Deserialize;
use serde_yaml_ng::Value;
use std::{collections::BTreeMap, fs, path::Path, sync::Arc};
use uuid::Uuid;

/// The line above a recap that a recording's processing wrote, as Carry over copies it.
const DRAFT_LINE: &str = "> This draft was written automatically. Check it against your memory.";
const RECAP: &str = "Recap of last time";
const PLAN: &str = "Plan for tonight";
const THREADS: &str = "Open threads";
const WHAT_HAPPENED: &str = "What happened";

/// One part that a note carried, as `carried_parts` names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Part {
    Recap,
    Plan,
    Threads,
    /// A word Splat does not write. It is kept out of what Splat writes back.
    #[serde(other)]
    Other,
}

impl Part {
    fn word(self) -> &'static str {
        match self {
            Self::Recap => "recap",
            Self::Plan => "plan",
            Self::Threads => "threads",
            Self::Other => "",
        }
    }
}

/// What a note's properties say it carried: the one owner of "is it done?".
#[derive(Debug, Default, Deserialize)]
struct Carried {
    carried_from: Option<Uuid>,
    #[serde(default)]
    carried_parts: Vec<Part>,
}

/// The carry-over properties of a note's whole text. Properties that cannot be read carry
/// nothing.
fn carried(text: &str) -> Carried {
    properties_yaml(text)
        .and_then(|inner| serde_yaml_ng::from_str(inner).ok())
        .unwrap_or_default()
}

/// The lines under `## heading` that hold words, up to the next heading of that level.
fn section_lines<'a>(body: &'a str, heading: &str) -> impl Iterator<Item = &'a str> {
    let mut lines = body.lines();
    let found = lines
        .find(|line| line.trim_end().strip_prefix("## ") == Some(heading))
        .is_some();
    lines
        .take_while(move |line| found && !line.starts_with("## "))
        .map(str::trim_end)
        .filter(|line| !line.trim().is_empty())
}

/// What the last session left, as its note and its recordings hold it when read.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Source {
    pub(super) id: Uuid,
    /// `Session 1`, as the result and a refusal name it.
    pub(super) name: String,
    /// The name an Obsidian link uses: the note's file name.
    link: String,
    /// The note has prose under `What happened`.
    written: bool,
    /// The newest processed recording's recap. It is the recap while `What happened` has no
    /// prose, and it tells a draft that is still that recap from one the DM changed.
    processed: Option<String>,
    /// The plan's lines still to come, as this note gets them: links kept, boxes empty.
    plan: Vec<String>,
    /// The open threads that are not ticked.
    threads: Vec<String>,
}

impl Source {
    /// Read the last session's note and its recordings now. `Err` holds the words of a note
    /// that cannot be read.
    pub(super) fn read(
        note: &Note,
        folder: &Path,
        plan: &Plan<'_>,
        book: &Book,
    ) -> Result<Self, String> {
        let name = format!("Session {}", note.number);
        let text = fs::read_to_string(&note.path)
            .map_err(|_| format!("Splat cannot read {name}’s note."))?;
        let body = body_of(&text);
        let mut played: BTreeMap<String, usize> = BTreeMap::new();
        // A fight whose run is still unfinished (paused) is still to come; it keeps `Resume`.
        let paused = |run: Uuid| {
            plan.saved_runs.iter().any(|saved| {
                saved.id == Some(run)
                    && saved.standing == crate::encounter_run::Standing::Unfinished
            })
        };
        for event in &book.events {
            if event.is_in(Some(note.id)) && event.run.is_some_and(|run| !paused(run)) {
                *played.entry(event.title.to_lowercase()).or_default() += 1;
            }
        }
        let written = section_text(body, WHAT_HAPPENED).and_then(|words| prose(&words));
        Ok(Self {
            id: note.id,
            name,
            link: note.link(),
            processed: processed_recap(folder, note.id),
            written: written.is_some(),
            plan: to_come(body, plan.fights, plan.groups, played),
            threads: section_lines(body, THREADS)
                .filter(|line| !task_parts(line).is_some_and(|(done, _)| done))
                .map(str::to_owned)
                .collect(),
        })
    }

    /// The processed recap, which the Last time line reads when `What happened` has none.
    pub(super) fn processed(&self) -> Option<&str> {
        self.processed.as_deref()
    }
}

/// The recap of the newest recording of a session that was processed.
pub(super) fn processed_recap(folder: &Path, session: Uuid) -> Option<String> {
    list_recordings(folder, session)
        .into_iter()
        .filter(|recording| recording.summary)
        .find_map(|recording| {
            let text = fs::read_to_string(crate::processing::summary_path(&recording.path)).ok()?;
            section_text(body_of(&text), "Recap")
        })
}

/// The lines of a plan that are still to come, as the plan pane counts them: a scene that is
/// not done, a line that links a fight that was not played (also one with a paused run), and a
/// line that links a group. Copied with links kept and boxes empty.
fn to_come(
    body: &str,
    fights: &[PlanFight],
    groups: &[Arc<RunGroup>],
    mut played: BTreeMap<String, usize>,
) -> Vec<String> {
    let mut lines = Vec::new();
    for line in section_lines(body, PLAN) {
        let names = each_linked_name(line);
        let task = task_parts(line);
        let keep = if super::plan::links_fight_or_group(&names, fights, groups) {
            let mut waiting = false;
            for name in &names {
                if let Some(fight) = fights
                    .iter()
                    .find(|fight| fight.name.eq_ignore_ascii_case(name))
                {
                    let done = played.entry(fight.name.to_lowercase()).or_default();
                    waiting |= *done == 0;
                    *done = done.saturating_sub(1);
                } else {
                    waiting |= groups
                        .iter()
                        .any(|group| group.name.eq_ignore_ascii_case(name));
                }
            }
            waiting
        } else {
            task.is_some_and(|(done, _)| !done)
        };
        if keep {
            lines.push(if task.is_some() {
                ticked(line, false)
            } else {
                line.to_owned()
            });
        }
    }
    lines
}

/// What the act does with the recap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Recap {
    /// Carried before, or the DM's words in this note were kept then.
    Done,
    /// This note has the DM's own words under the heading. They stay, and the part is done.
    Kept,
    /// The last session has no recap yet.
    Later,
    /// The embed line of the last session's `What happened`.
    Embed,
    /// The processed recap, copied under its warning line.
    Draft,
    /// The embed takes the place of a draft that is still the processed recap.
    Swap,
}

/// What `Carry over` adds to this note now.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Offer {
    recap: Recap,
    plan: Vec<String>,
    threads: Vec<String>,
}

/// A line without its list mark or task box, as two lines compare.
fn bare(line: &str) -> &str {
    let line = line.trim();
    task_parts(line).map_or_else(
        || {
            line.strip_prefix("- ")
                .or_else(|| line.strip_prefix("* "))
                .or_else(|| line.strip_prefix("+ "))
                .unwrap_or(line)
                .trim()
        },
        |(_, words)| words,
    )
}

/// The draft as a copied processed recap stands in a note.
fn draft_block(recap: &str) -> String {
    format!("{DRAFT_LINE}\n\n{}", recap.trim())
}

fn embed_line(source: &Source) -> String {
    format!("![[{}#{WHAT_HAPPENED}]]", source.link)
}

impl Offer {
    /// What the act would add to a note's whole text, or `None` when nothing is left to carry
    /// from `source`.
    pub(super) fn of(text: &str, source: &Source) -> Option<Self> {
        let carried = carried(text);
        if carried.carried_from.is_some_and(|from| from != source.id) {
            return None;
        }
        let body = body_of(text);
        let done = |part| carried.carried_parts.contains(&part);
        let own = section_text(body, RECAP);
        let recap = if done(Part::Recap) {
            let draft = source.processed.as_deref().map(draft_block);
            if source.written && own.is_some() && own == draft {
                Recap::Swap
            } else {
                Recap::Done
            }
        } else if own.is_some() {
            Recap::Kept
        } else if source.written {
            Recap::Embed
        } else if source.processed.is_some() {
            Recap::Draft
        } else {
            Recap::Later
        };
        let has: Vec<&str> = body.lines().map(bare).collect();
        let fresh = |part, lines: &[String]| -> Vec<String> {
            if done(part) {
                return Vec::new();
            }
            lines
                .iter()
                .filter(|line| !has.contains(&bare(line)))
                .cloned()
                .collect()
        };
        let offer = Self {
            recap,
            plan: fresh(Part::Plan, &source.plan),
            threads: fresh(Part::Threads, &source.threads),
        };
        let adds = matches!(offer.recap, Recap::Embed | Recap::Draft | Recap::Swap)
            || !offer.plan.is_empty()
            || !offer.threads.is_empty();
        adds.then_some(offer)
    }

    /// `the recap, 2 plan lines and 2 open threads`: only what the act adds.
    fn listed(&self, recap: &str) -> String {
        let count = |count: usize, one: &str, more: &str| match count {
            0 => None,
            1 => Some(format!("1 {one}")),
            count => Some(format!("{count} {more}")),
        };
        let parts: Vec<String> = [
            matches!(self.recap, Recap::Embed | Recap::Draft).then(|| recap.to_owned()),
            count(self.plan.len(), "plan line", "plan lines"),
            count(self.threads.len(), "open thread", "open threads"),
        ]
        .into_iter()
        .flatten()
        .collect();
        match parts.as_slice() {
            [] => String::new(),
            [one] => one.clone(),
            [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
        }
    }

    /// The action's hover: what it will add.
    pub(super) fn hover(&self) -> String {
        let listed = self.listed("recap");
        let mut words = if listed.is_empty() {
            String::new()
        } else {
            format!("Add the last session’s {listed} to this note.")
        };
        if self.recap == Recap::Swap {
            if !words.is_empty() {
                words.push(' ');
            }
            words.push_str("Put a link to the last session’s notes in place of the draft recap.");
        }
        words
    }

    /// The act's result: it names only what it added.
    fn result(&self, source: &Source) -> String {
        let listed = self.listed("the recap");
        let mut words = if listed.is_empty() {
            String::new()
        } else {
            format!("Carried over from {}: {listed}.", source.name)
        };
        let after = match self.recap {
            Recap::Kept => "Your recap was kept.",
            Recap::Later => "The recap is still to come.",
            Recap::Swap => "The recap now follows your notes.",
            Recap::Done | Recap::Embed | Recap::Draft => "",
        };
        if !after.is_empty() {
            if !words.is_empty() {
                words.push(' ');
            }
            words.push_str(after);
        }
        words
    }
}

/// The byte range of the words under `## heading`: from after the heading's line to the next
/// heading of that level, or the end.
fn section_span(body: &str, heading: &str) -> Option<(usize, usize)> {
    let mut at = 0;
    let mut start = None;
    for line in body.split_inclusive('\n') {
        let next = at + line.len();
        let text = line.trim_end();
        if let Some(start) = start {
            if text.starts_with("## ") {
                return Some((start, at));
            }
        } else if text.strip_prefix("## ") == Some(heading) {
            start = Some(next);
        }
        at = next;
    }
    start.map(|start| (start, body.len()))
}

/// Put `block` below what a heading holds, or add the heading at the end of the note.
fn add_below(body: &mut String, heading: &str, block: &str) {
    match section_span(body, heading) {
        Some((start, end)) => {
            let kept = start + body[start..end].trim_end().len();
            let list = body[start..kept]
                .lines()
                .last()
                .is_some_and(|line| bare(line) != line.trim());
            let gap = if kept > start && !list { "\n\n" } else { "\n" };
            let tail = if end < body.len() { "\n\n" } else { "\n" };
            // A heading on the last line, with no line end, ends its line first.
            let start_gap = if body[..start].ends_with('\n') {
                ""
            } else {
                "\n"
            };
            body.replace_range(kept..end, &format!("{start_gap}{gap}{block}{tail}"));
        }
        None => {
            let kept = body.trim_end().len();
            body.truncate(kept);
            let gap = if kept == 0 { "" } else { "\n\n" };
            body.push_str(&format!("{gap}## {heading}\n\n{block}\n"));
        }
    }
}

/// This note's whole text with what is left to carry from `source`, and what the act added.
/// `None` when nothing is left, or the note's properties cannot be written.
pub(super) fn carry(text: &str, source: &Source) -> Option<(String, Offer)> {
    let offer = Offer::of(text, source)?;
    let (prefix, body) = split_note(text).unwrap_or(("", text));
    let mut body = body.to_owned();
    match offer.recap {
        Recap::Embed => add_below(&mut body, RECAP, &embed_line(source)),
        Recap::Draft => add_below(
            &mut body,
            RECAP,
            &draft_block(source.processed.as_deref().unwrap_or_default()),
        ),
        Recap::Swap => {
            let (start, end) = section_span(&body, RECAP)?;
            let tail = if end < body.len() { "\n\n" } else { "\n" };
            body.replace_range(start..end, &format!("\n{}{tail}", embed_line(source)));
        }
        Recap::Done | Recap::Kept | Recap::Later => {}
    }
    if !offer.plan.is_empty() {
        add_below(&mut body, PLAN, &offer.plan.join("\n"));
    }
    if !offer.threads.is_empty() {
        add_below(&mut body, THREADS, &offer.threads.join("\n"));
    }
    let mut parts = carried(text).carried_parts;
    parts.retain(|part| *part != Part::Other);
    for (part, now) in [
        (
            Part::Recap,
            offer.recap != Recap::Later && offer.recap != Recap::Done,
        ),
        (Part::Plan, !offer.plan.is_empty()),
        (Part::Threads, !offer.threads.is_empty()),
    ] {
        if now && !parts.contains(&part) {
            parts.push(part);
        }
    }
    parts.sort_by_key(|part| *part as u8);
    let text = patch_fields(
        &format!("{prefix}{body}"),
        &[
            ("carried_from", Some(Value::String(source.id.to_string()))),
            (
                "carried_parts",
                Some(Value::Sequence(
                    parts
                        .iter()
                        .map(|part| Value::String(part.word().to_owned()))
                        .collect(),
                )),
            ),
        ],
    )
    .ok()?;
    Some((text, offer))
}

/// The result of the last note act of this feature on one session, kept by that session's ID:
/// another session, another page or an act elsewhere leaves it.
enum Outcome {
    /// What the act did, and its Undo, while the note's text is what the act left.
    Done {
        words: String,
        undo: NoteUndo,
        /// The draft's revision when the Undo was last checked.
        seen: u64,
    },
    /// Why the act wrote nothing, until the note's text changes.
    Refused { words: String, seen: u64 },
}

/// The results of carry-over acts, by the session note each act changed; Read's prepared words
/// of the open note; and whether the selected session is prepared, as last worked out.
#[derive(Default)]
pub(super) struct Carries {
    results: BTreeMap<Uuid, Outcome>,
    /// When each session's carry-over act was done: its result's footer order.
    orders: BTreeMap<Uuid, design::ResultOrder>,
    /// Read's words, for what they were made from. A folder scan prepares them again, as the
    /// notes that embeds show can change.
    read: Option<(ReadKey, Arc<Prepared>)>,
    /// How often a frame asked for Read's words, and how often they were made.
    #[cfg(test)]
    pub(super) read_counts: crate::test_support::Counts,
    /// `preparing`'s answer, for the session, the current session, the open run's and the live
    /// recording's sessions, and the number of the Timeline's events it was worked out for.
    preparing: Option<(PreparingKey, bool)>,
}

type PreparingKey = (Uuid, Option<Uuid>, Option<Uuid>, Option<Uuid>, usize);

impl Carries {
    /// The act on `session` made `outcome`: it replaces that session's earlier result, with a new
    /// order of its own. Other sessions' results keep theirs.
    fn record(&mut self, session: Uuid, outcome: Outcome) {
        self.results.insert(session, outcome);
        self.orders.insert(session, design::ResultOrder::issue());
    }

    /// The footer order of `session`'s result, issued by the act that made it.
    pub(super) fn order(&self, session: Uuid) -> design::ResultOrder {
        self.orders.get(&session).copied().unwrap_or_default()
    }

    /// The folder was scanned: an embed's note can have changed or gone, and an event can have
    /// moved to another session.
    pub(super) fn scanned(&mut self) {
        self.read = None;
        self.preparing = None;
    }

    /// The note moved to a new file, and its Undo follows it.
    pub(super) fn moved(&mut self, from: &Path, to: &Path) {
        self.read = None;
        for outcome in self.results.values_mut() {
            if let Outcome::Done { undo, .. } = outcome {
                undo.moved(from, to);
            }
        }
    }

    /// A result ends at another change of its note's text. Checked only when the draft's
    /// revision changed, so a quiet frame compares nothing.
    fn end_what_changed(&mut self, session: Uuid, draft: Option<&crate::sessions::Draft>) {
        let Some(draft) = draft else {
            return;
        };
        let revision = draft.revision();
        // A later save writes the note without a change of its words, so an act whose save
        // failed learns of it on any frame, until it does.
        if let Some(Outcome::Done { undo, .. }) = self.results.get_mut(&session)
            && !undo.written()
        {
            undo.follow_save(draft);
        }
        let ended = match self.results.get_mut(&session) {
            Some(Outcome::Done { undo, seen, .. }) if *seen != revision => {
                *seen = revision;
                !undo.live(draft)
            }
            Some(Outcome::Refused { seen, .. }) => *seen != revision,
            _ => false,
        };
        if ended {
            self.results.remove(&session);
        }
    }
}

impl Sessions {
    /// Whether this session is being prepared: it is the session that new events go to, and it
    /// has no event, no run and no live recording. Carry-over is a prep act.
    /// The events are looked through only when their number changes, at a folder scan, or when
    /// another of these inputs changes; not on a quiet frame.
    fn preparing(&mut self, session: Uuid, book: &Book) -> bool {
        let key = (
            session,
            self.current(),
            self.open_run.as_ref().map(|(_, run)| *run),
            self.live.as_ref().map(|live| live.session),
            book.events.len(),
        );
        if let Some((seen, answer)) = self.carries.preparing
            && seen == key
        {
            return answer;
        }
        let answer = key.1 == Some(session)
            && key.2 != Some(session)
            && key.3 != Some(session)
            && !book.events.iter().any(|event| event.is_in(Some(session)));
        self.carries.preparing = Some((key, answer));
        answer
    }

    /// The session `Carry over` reads: the one this note carried from, once it did, or else
    /// the one before it.
    pub(super) fn carry_source(
        &self,
        previous: &Note,
        plan: &Plan<'_>,
        book: &Book,
    ) -> Option<Result<Source, String>> {
        let from = self
            .draft
            .as_ref()
            .and_then(|draft| carried(&draft.text()).carried_from);
        let note = match from {
            Some(id) => self.notes.iter().find(|note| note.id == id)?,
            None => previous,
        };
        let folder = self.folder.as_deref()?;
        Some(Source::read(note, folder, plan, book))
    }

    /// What `Carry over` would add now, as its hover says it, or why it is blocked. Read again
    /// only when the note's words change.
    fn carry_offer(&mut self) -> Option<Result<String, String>> {
        let revision = self.draft.as_ref().map(crate::sessions::Draft::revision)?;
        let (_, Some(last)) = self.last_time.as_mut()? else {
            return None;
        };
        let source = last.source.as_ref()?;
        if last.offer.as_ref().map(|(seen, _)| *seen) != Some(revision) {
            let offer = match source {
                Ok(source) => self
                    .draft
                    .as_ref()
                    .and_then(|draft| Offer::of(&draft.text(), source)),
                Err(_) => None,
            };
            last.offer = Some((revision, offer));
        }
        match (source, last.offer.as_ref()) {
            (Err(words), _) => Some(Err(words.clone())),
            (Ok(_), Some((_, Some(offer)))) => Some(Ok(offer.hover())),
            _ => None,
        }
    }

    /// The Last time line: the last session's link, its preview, and `Carry over` at the end
    /// while this session is prepared. The preview gives the action its room first, then the
    /// link's words shorten, and nothing wraps. The act's result shows under the line.
    pub(super) fn last_time_line(
        &mut self,
        ui: &mut egui::Ui,
        session: Uuid,
        plan: &Plan<'_>,
        book: &Book,
    ) {
        let offer = if self.preparing(session, book) {
            self.carry_offer()
        } else {
            None
        };
        let Some((_, Some(last))) = &self.last_time else {
            return;
        };
        let full = format!(
            "Last time: {} — {}",
            last.label,
            last.what_happened
                .as_deref()
                .unwrap_or("Nothing written yet.")
        );
        let preview = last
            .what_happened
            .as_deref()
            .and_then(|words| words.lines().find(|line| !line.trim().is_empty()))
            .unwrap_or("");
        let (hover, blocked) = match &offer {
            Some(Ok(hover)) => (Some(hover.as_str()), None),
            Some(Err(words)) => (None, Some(words.as_str())),
            None => (None, None),
        };
        let mut open = false;
        let mut carry = false;
        ui.horizontal(|ui| {
            ui.label(Type::Small.rich(ui, "Last time:").color(colors().muted));
            let action = offer.is_some().then(|| {
                let mut action = Action::quiet("Carry over").in_line().blocked(blocked);
                if let Some(hover) = hover {
                    action = action.hint(hover);
                }
                action
            });
            let carry_room = action
                .as_ref()
                .map_or(0.0, |action| action.natural_width(ui) + design::L);
            let label = if action.is_some() {
                design::words_that_fit(
                    ui,
                    &last.label,
                    &Type::Body.font(ui),
                    ui.available_width() - carry_room,
                )
            } else {
                last.label.clone()
            };
            open = design::link(ui, &label).on_hover_text(&full).clicked();
            // Muted words keep the reader's measure, as help does.
            let room = (ui.available_width() - carry_room).min(design::READER_MEASURE);
            let words = preview_that_fits(ui, preview, room);
            if !words.is_empty() || action.is_none() {
                ui.label(Type::Small.rich(ui, words).color(colors().muted))
                    .on_hover_text(&full);
            }
            // `L` after the line's last word, so it reads as this line's action.
            if let Some(action) = action {
                ui.add_space(design::L - ui.spacing().item_spacing.x);
                carry = action.show(ui).clicked();
            }
        });
        let id = last.id;
        if open {
            self.select(Some(id));
            return;
        }
        if carry {
            self.carry_over(session, id, plan, book);
        }
    }

    /// The act. It reads the last session again at the click, and writes nothing when what the
    /// line offered changed; otherwise one save of this note through `Draft::apply`.
    fn carry_over(&mut self, session: Uuid, previous: Uuid, plan: &Plan<'_>, book: &Book) {
        let Some(previous) = self.notes.iter().find(|note| note.id == previous) else {
            return;
        };
        let shown = self
            .last_time
            .as_ref()
            .and_then(|(_, last)| last.as_ref()?.source.clone());
        let now = self.carry_source(previous, plan, book);
        let seen = self
            .draft
            .as_ref()
            .map_or(0, crate::sessions::Draft::revision);
        let source = match (&now, &shown) {
            (Some(Ok(now)), Some(Ok(shown))) if now == shown => now.clone(),
            (Some(Err(words)), _) => {
                self.carries.record(
                    session,
                    Outcome::Refused {
                        words: words.clone(),
                        seen,
                    },
                );
                return;
            }
            (Some(Ok(now)), _) => {
                // What the line offered is not what the last note holds now: read it again.
                let words = format!(
                    "{}’s note changed. Check what it holds, then carry over again.",
                    now.name
                );
                self.last_time = None;
                self.carries
                    .record(session, Outcome::Refused { words, seen });
                return;
            }
            (None, _) => return,
        };
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let mut offer = None;
        let result = draft.apply_with_properties(session, |text| {
            let (text, done) = carry(text, &source)?;
            offer = Some(done);
            Some(text)
        });
        let seen = draft.revision();
        let outcome = match result {
            Ok(undo) | Err(ActFault::NotSaved(_, undo)) => Outcome::Done {
                words: offer.map(|offer| offer.result(&source)).unwrap_or_default(),
                undo,
                seen,
            },
            Err(fault) => Outcome::Refused {
                words: fault.words().to_owned(),
                seen,
            },
        };
        self.carries.record(session, outcome);
    }

    /// The result of the open session's act, in the Session page's message footer, so that the
    /// Last time line, the tabs and the plan never move when it comes or goes: its words and
    /// `Undo carry over`, or why it wrote nothing. It is that note's, so it shows also when the
    /// last session can no longer be found. A save that failed says so, and the Undo still works.
    pub(super) fn carry_in_footer<'a>(
        &'a self,
        undos: &mut crate::timeline::page::LiveUndos<'a>,
        footer: &mut design::Footer<'a>,
        act: &'a std::cell::Cell<Option<super::SessionAct>>,
    ) {
        let Some(session) = self.selected else {
            return;
        };
        match self.carries.results.get(&session) {
            Some(Outcome::Refused { words, .. }) => {
                // The act's order, issued when it was done (`Carries::record`).
                let order = self.carries.order(session);
                footer.result(order, words.as_str(), false, move |ui| {
                    design::message(ui, Tone::Warning, words);
                });
            }
            Some(Outcome::Done { words, undo, .. }) => {
                let (tone, words) = if undo.written() {
                    (Tone::Success, words.as_str())
                } else {
                    (Tone::Warning, crate::sessions::NOT_SAVED)
                };
                let order = self.carries.order(session);
                undos.push(order, words.to_owned(), move |ui| {
                    if design::undo_message(ui, tone, words, "Undo carry over") {
                        act.set(Some(super::SessionAct::UndoCarry(session)));
                    }
                });
            }
            None => {}
        }
    }

    /// A result ends at another change of its note's text; checked once a frame, before the
    /// footer draws it.
    pub(super) fn end_carry_results(&mut self) {
        if let Some(session) = self.selected {
            self.carries.end_what_changed(session, self.draft.as_ref());
        }
    }

    /// `Undo carry over`: the act's own Undo, on this session's note. When it cannot apply, the
    /// footer says why.
    pub(super) fn undo_carry(&mut self, session: Uuid) {
        let (Some(Outcome::Done { undo, .. }), Some(draft)) =
            (self.carries.results.remove(&session), self.draft.as_mut())
        else {
            return;
        };
        if let Err(fault) = undo.undo(draft) {
            let seen = draft.revision();
            // A refused Undo is an act of its own: its result takes a new order.
            self.carries.record(
                session,
                Outcome::Refused {
                    words: fault.words().to_owned(),
                    seen,
                },
            );
        }
    }
}

/// The mark that stands for an embed line while Read prepares the note's words.
const EMBED_MARK: char = '\u{E000}';

/// One line of the open note that is an embed of a session note's heading, as Read draws it.
#[derive(Clone, Debug)]
pub(super) struct EmbedLine {
    /// The line as the note holds it.
    line: String,
    shown: Shown,
}

#[derive(Clone, Debug)]
enum Shown {
    /// The heading's words, ready to read, under `From <session>`, which links that session.
    Words {
        from: Uuid,
        label: String,
        words: String,
    },
    /// The note or its heading is gone. The line stays in the note.
    Gone(String),
}

/// The Last time preview in this room: a dash and the longest run of the preview's whole words
/// that fits. With no word, nothing: the line never ends in a dash and an ellipsis.
fn preview_that_fits(ui: &egui::Ui, preview: &str, room: f32) -> String {
    let words = design::words_that_fit(ui, &format!("— {preview}"), &Type::Small.font(ui), room);
    // `words_that_fit` keeps a first word that does not fit: here, the dash alone.
    match words.strip_prefix("— ") {
        Some(rest) if rest != "…" => words,
        _ => String::new(),
    }
}

/// Whether a link names this file: the link's whole path, by whole folder names, ends the
/// file's path, as Obsidian reads `Archive/Same`. `Deleted/Same` does not name `Same.md`. A
/// link's separator is `/` only, as the index reads it.
fn names_file(path: &Path, link: &str) -> bool {
    let link = link.to_lowercase();
    let link = link.trim_start_matches('/').trim_end_matches(".md");
    let file = path
        .with_extension("")
        .to_string_lossy()
        .replace('\\', "/")
        .to_lowercase();
    file == link || file.ends_with(&format!("/{link}"))
}

/// The target (a note's name or path) and the heading of a line that is only an embed of a
/// note's heading. An embed inside a callout or a list item is not such a line.
fn embed_target(line: &str) -> Option<(&str, &str)> {
    let inner = line.trim().strip_prefix("![[")?.strip_suffix("]]")?;
    let inner = inner.split('|').next()?;
    let (name, heading) = inner.split_once('#')?;
    let (name, heading) = (name.trim(), heading.trim());
    (!name.is_empty() && !heading.is_empty() && !heading.starts_with('^'))
        .then_some((name, heading))
}

/// One part of Read's words: Markdown, or the embed with this place in the list.
#[derive(Debug)]
enum Chunk {
    Words(String),
    Embed(usize),
}

/// Read's words of the open note, prepared when the note's words, the content, the plan or the
/// folder change, never on a quiet frame.
#[derive(Debug)]
pub(super) struct Prepared {
    /// The note has written words; otherwise Read says `Nothing written yet.`
    pub(super) written: bool,
    /// The plan's count line, in the tab layout, between the two parts.
    pub(super) summary: Option<String>,
    before: Vec<Chunk>,
    after: Vec<Chunk>,
    embeds: Vec<EmbedLine>,
}

/// What a prepared Read was made from: the draft's revision, the content's generation, the
/// plan model it used, and whether the plan stands beside the note.
pub(super) type ReadKey = (u64, Option<u64>, usize, bool);

/// The parts of `text`, split at its marked embed lines.
fn chunks(text: &str) -> Vec<Chunk> {
    let mut out = Vec::new();
    let mut part = String::new();
    for line in text.split_inclusive('\n') {
        match line
            .trim_end()
            .strip_prefix(EMBED_MARK)
            .and_then(|rest| rest.strip_suffix(EMBED_MARK))
            .and_then(|at| at.parse::<usize>().ok())
        {
            Some(at) => {
                if !part.trim().is_empty() {
                    out.push(Chunk::Words(std::mem::take(&mut part)));
                }
                part.clear();
                out.push(Chunk::Embed(at));
            }
            None => part.push_str(line),
        }
    }
    if !part.trim().is_empty() {
        out.push(Chunk::Words(part));
    }
    out
}

impl Sessions {
    /// The open note's embeds of session notes' headings. Each target is resolved as every
    /// link of the campaign is (`ContentIndex::resolve_link`): a session note shows its
    /// heading's words; a target that names nothing says its source is gone; any other
    /// target, an ambiguous one, and every embed while there is no index stay links.
    fn embeds(
        &self,
        body: &str,
        index: Option<&crate::lookup::ContentIndex>,
        book: &Book,
    ) -> Vec<EmbedLine> {
        let Some(index) = index else {
            return Vec::new();
        };
        let mut embeds = Vec::new();
        for line in body.lines() {
            let Some((name, heading)) = embed_target(line) else {
                continue;
            };
            if embeds
                .iter()
                .any(|embed: &EmbedLine| embed.line == line.trim_end())
            {
                continue;
            }
            let gone = || {
                Shown::Gone(format!(
                    "The recap’s source is gone: {name} › {heading}. Link the session again in Write."
                ))
            };
            let shown = match index.resolve_link(name, None) {
                // A session note at the link's whole path keeps it a link while it has the
                // heading: the index has not read the note yet (`refresh`), or the link names
                // part of its path. Without such a note, or without the heading, it is gone.
                crate::world::Go::Missing(_) => {
                    let mut at = self
                        .notes
                        .iter()
                        .filter(|note| names_file(&note.path, name));
                    match (at.next(), at.next()) {
                        (None, _) => gone(),
                        (Some(note), None)
                            if fs::read_to_string(&note.path)
                                .ok()
                                .and_then(|text| section_text(body_of(&text), heading))
                                .is_none() =>
                        {
                            gone()
                        }
                        _ => continue,
                    }
                }
                crate::world::Go::Item(at, _) => {
                    let item = &index.items[at];
                    if item.note_role() != Some(crate::world::Role::Session) {
                        continue;
                    }
                    let Some(note) = self
                        .notes
                        .iter()
                        .find(|note| note.path.ends_with(&item.entry.path))
                    else {
                        continue;
                    };
                    fs::read_to_string(&note.path)
                        .ok()
                        .and_then(|text| section_text(body_of(&text), heading))
                        .map_or_else(gone, |words| Shown::Words {
                            from: note.id,
                            label: note.label(),
                            words: super::words_to_read(&words, book),
                        })
                }
                _ => continue,
            };
            embeds.push(EmbedLine {
                line: line.trim_end().to_owned(),
                shown,
            });
        }
        embeds
    }

    /// Read's words of the open note, from the cache while nothing they are made from changed.
    pub(super) fn prepared_read(
        &mut self,
        index: Option<&crate::lookup::ContentIndex>,
        model: &Arc<super::plan::PlanModel>,
        plan: &Plan<'_>,
        beside: bool,
        book: &Book,
    ) -> Option<Arc<Prepared>> {
        let Some(draft) = &self.draft else {
            self.carries.read = None;
            return None;
        };
        let key = (
            draft.revision(),
            index.map(crate::lookup::ContentIndex::generation),
            Arc::as_ptr(model) as usize,
            beside,
        );
        #[cfg(test)]
        {
            self.carries.read_counts.reached += 1;
        }
        if let Some((seen, prepared)) = &self.carries.read
            && *seen == key
        {
            return Some(Arc::clone(prepared));
        }
        #[cfg(test)]
        {
            self.carries.read_counts.work += 1;
        }
        let body = self.note_body();
        let embeds = self.embeds(&body, index, book);
        let marked = if embeds.is_empty() {
            body
        } else {
            body.split_inclusive('\n')
                .map(|line| {
                    match embeds
                        .iter()
                        .position(|embed| embed.line == line.trim_end())
                    {
                        Some(at) => format!("{EMBED_MARK}{at}{EMBED_MARK}\n"),
                        None => line.to_owned(),
                    }
                })
                .collect()
        };
        let compact = super::compact_plan_for_reading(
            &marked,
            plan.fights,
            plan.groups,
            &model.scene_lines(),
            beside,
        );
        let summary = (!beside && compact != marked)
            .then(|| section_text(&compact, "Plan for tonight"))
            .flatten();
        let text = super::words_to_read(&compact, book);
        let (before, after, summary) = match summary
            .as_ref()
            .and_then(|summary| text.split_once(summary.as_str()))
        {
            Some((before, after)) => (chunks(before), chunks(after), summary),
            None => (chunks(&text), Vec::new(), None),
        };
        let prepared = Arc::new(Prepared {
            written: super::has_written_text(&compact),
            summary,
            before,
            after,
            embeds,
        });
        self.carries.read = Some((key, Arc::clone(&prepared)));
        Some(prepared)
    }

    /// One part of the prepared Read: its Markdown, and each embed as its heading's words under
    /// a muted line that links its session, or as the words that say its source is gone.
    pub(super) fn show_prepared(&mut self, ui: &mut egui::Ui, prepared: &Prepared, after: bool) {
        let mut open = None;
        for chunk in if after {
            &prepared.after
        } else {
            &prepared.before
        } {
            match chunk {
                Chunk::Words(words) => crate::markdown_view::show_reader(ui, words),
                Chunk::Embed(at) => match prepared.embeds.get(*at).map(|embed| &embed.shown) {
                    Some(Shown::Words { from, label, words }) => {
                        ui.horizontal(|ui| {
                            // One gap, `S`, whatever spacing the layout around it has.
                            ui.spacing_mut().item_spacing.x = design::S;
                            ui.label(Type::Small.rich(ui, "From").color(colors().muted));
                            if design::small_link(ui, label).clicked() {
                                open = Some(*from);
                            }
                        });
                        crate::markdown_view::show_reader(ui, words);
                    }
                    Some(Shown::Gone(words)) => {
                        design::small_message(ui, Tone::Warning, words);
                    }
                    None => {}
                },
            }
        }
        if let Some(session) = open {
            self.select(Some(session));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        design,
        sessions::{Apply, Draft, NewNote, create_note, list_notes},
        timeline::Kind,
    };
    use egui_kittest::kittest::Queryable;

    const LAST: &str = "## Recap of last time\n\nThe party left at dawn.\n\n\
        ## Plan for tonight\n\n- [[Bridge Ambush]]\n- [[Ford Ambush]]\n- [x] [[Shrine of Cinders]]\n\
        - [ ] Talk to [[Hobb]] about the ropes\n- [x] Read the rumours\nRemember the toll.\n\n\
        ## What happened\n\nWolves at the ford.\n\n\
        ## Open threads\n\n- [ ] Who burned the ropes?\n- [x] Who paid the toll?\nThe ferry is gone.\n";
    const TONIGHT: &str =
        "## Recap of last time\n\n## Plan for tonight\n\n- [[Ashfall]]\n\n## What happened\n\n";

    fn fight(name: &str) -> PlanFight {
        PlanFight {
            saved: None,
            id: uuid::Uuid::nil(),
            name: name.into(),
            path: format!("Encounters/{name}.md"),
            summary: String::new(),
            difficulty: Default::default(),
        }
    }

    fn note(dir: &Path, number: u32, title: &str, template: &str) -> Note {
        create_note(
            dir,
            &NewNote {
                number,
                title,
                date: "2026-11-01",
                campaign: "Marches",
                template,
            },
        )
        .unwrap()
    }

    /// The run of `Ford Ambush`, which `paused` saves unfinished.
    const FORD_RUN: Uuid = Uuid::from_u128(7);

    /// The book of the last session: `Bridge Ambush` was played and `Ford Ambush` was started;
    /// `Shrine of Cinders` was not.
    fn played(session: Uuid) -> Book {
        let mut book = Book::default();
        for (title, run) in [("Bridge Ambush", Uuid::nil()), ("Ford Ambush", FORD_RUN)] {
            book.add(Kind::Encounter, Some(session));
            let event = book.events.last_mut().unwrap();
            event.title = title.into();
            event.run = Some(run);
        }
        book
    }

    /// `Ford Ambush`'s run, saved and not ended: a paused fight.
    fn paused() -> crate::encounter_run::SavedRunInfo {
        crate::encounter_run::SavedRunInfo {
            path: "Ford Ambush.json".into(),
            id: Some(FORD_RUN),
            source: None,
            encounter: "Ford Ambush".into(),
            label: "Ford Ambush".into(),
            standing: crate::encounter_run::Standing::Unfinished,
            saved_at: None,
            unsent: Vec::new(),
        }
    }

    fn with_plan<R>(fights: &[PlanFight], add: impl FnOnce(&Plan<'_>) -> R) -> R {
        with_runs(fights, &[], add)
    }

    fn with_runs<R>(
        fights: &[PlanFight],
        saved_runs: &[crate::encounter_run::SavedRunInfo],
        add: impl FnOnce(&Plan<'_>) -> R,
    ) -> R {
        let mut group = |_: &mut egui::Ui| {};
        let plan = Plan {
            fights,
            saved_runs,
            groups: &[],
            group_blocked: None,
            group_open: false,
            group_discarded: false,
            group_discarded_name: String::new(),
            group_discarded_order: None,
            arrivals_open: false,
            group_editor: std::cell::RefCell::new(&mut group),
            blocked: None,
            run_directory: None,
            open_run: None,
            open_state: String::new(),
            open_encounter: None,
            party: &[],
            tables: &[],
        };
        add(&plan)
    }

    /// T18.3: the recap's embed, the plan lines still to come and the open threads, each once,
    /// below the DM's words, with both properties in the same save; and its Undo is the exact
    /// inverse, also of properties that were absent.
    #[test]
    fn carry_over_adds_each_part_once_and_its_undo_is_the_exact_inverse() {
        let dir = tempfile::tempdir().unwrap();
        let last = note(dir.path(), 1, "The Road", LAST);
        let tonight = note(dir.path(), 2, "The Crossing", TONIGHT);
        let fights = [
            fight("Bridge Ambush"),
            fight("Ford Ambush"),
            fight("Shrine of Cinders"),
            fight("Ashfall"),
        ];
        // The played fight stays out; the paused one carries, and keeps its `Resume`.
        let source = with_runs(&fights, &[paused()], |plan| {
            Source::read(&last, dir.path(), plan, &played(last.id)).unwrap()
        });
        let before = fs::read_to_string(&tonight.path).unwrap();
        let mut draft = Draft::open(&tonight.path).unwrap();
        let mut offer = None;
        let undo = draft
            .apply_with_properties(tonight.id, |text| {
                let (text, done) = carry(text, &source)?;
                offer = Some(done);
                Some(text)
            })
            .unwrap();
        let after = fs::read_to_string(&tonight.path).unwrap();
        let link = last.link();
        assert!(after.contains(&format!(
            "## Recap of last time\n\n![[{link}#What happened]]\n\n## Plan for tonight\n\n\
             - [[Ashfall]]\n- [[Ford Ambush]]\n- [ ] [[Shrine of Cinders]]\n- [ ] Talk to [[Hobb]] about the ropes\n\n\
             ## What happened\n\n## Open threads\n\n- [ ] Who burned the ropes?\nThe ferry is gone.\n"
        )));
        assert!(after.contains(&format!("carried_from: {}\n", last.id)));
        assert!(after.contains("carried_parts:"));
        assert_eq!(
            carried(&after).carried_parts,
            [Part::Recap, Part::Plan, Part::Threads]
        );
        assert_eq!(
            offer.unwrap().result(&source),
            "Carried over from Session 1: the recap, 3 plan lines and 2 open threads."
        );
        // Every part is done: no action, also when the DM deletes a carried line later.
        assert_eq!(Offer::of(&after, &source), None);
        let trimmed = after.replace("- [ ] Who burned the ropes?\n", "");
        assert_eq!(Offer::of(&trimmed, &source), None);
        // Undo puts back the exact bytes: no properties, no lines.
        undo.undo(&mut draft).unwrap();
        assert_eq!(fs::read_to_string(&tonight.path).unwrap(), before);
        assert!(!before.contains("carried_"));

        // The DM's own recap is kept, and a line this note has, with or without its box, is not
        // added twice. A missing heading is added at the end.
        let own = before
            .replace(
                "## Recap of last time\n\n",
                "## Recap of last time\n\nMy own words.\n\n",
            )
            .replace(
                "- [[Ashfall]]\n",
                "- [[Ashfall]]\n- [x] Talk to [[Hobb]] about the ropes\n",
            )
            .replace("## What happened\n\n", "## What happened\n");
        let (text, offer) = carry(&own, &source).unwrap();
        assert!(text.contains("## Recap of last time\n\nMy own words.\n\n## Plan"));
        assert_eq!(text.matches("about the ropes").count(), 1);
        assert!(text.ends_with(
            "## What happened\n\n## Open threads\n\n- [ ] Who burned the ropes?\nThe ferry is gone.\n"
        ));
        assert_eq!(
            offer.result(&source),
            "Carried over from Session 1: 2 plan lines and 2 open threads. Your recap was kept."
        );
        assert_eq!(
            offer.hover(),
            "Add the last session’s 2 plan lines and 2 open threads to this note."
        );
    }

    /// A processed recording's recap is the second source: copied under its warning line. Once
    /// the DM writes `What happened`, the draft that is still that recap becomes the embed, and
    /// Undo gives back the draft and both properties as they were.
    #[test]
    fn a_draft_recap_becomes_the_embed_and_undo_gives_it_back() {
        let dir = tempfile::tempdir().unwrap();
        let unwritten = LAST.replace("Wolves at the ford.\n\n", "");
        let last = note(dir.path(), 1, "The Road", &unwritten);
        let tonight = note(dir.path(), 2, "The Crossing", TONIGHT);
        let recordings = crate::sessions::recordings_folder(dir.path(), last.id);
        fs::create_dir_all(&recordings).unwrap();
        fs::write(
            crate::processing::summary_path(&recordings.join("2026-11-01 20-00.wav")),
            "---\nsummary_model: \"test\"\n---\n\n> This is a draft from an LLM. Check it against your own memory.\n\n## Recap\n\nThe wolves came at dusk.\n",
        )
        .unwrap();
        let read = |book: &Book| {
            with_plan(&[], |plan| {
                Source::read(&last, dir.path(), plan, book).unwrap()
            })
        };
        let source = read(&Book::default());
        assert_eq!(source.processed(), Some("The wolves came at dusk."));
        let mut draft = Draft::open(&tonight.path).unwrap();
        let mut words = String::new();
        draft
            .apply_with_properties(tonight.id, |text| {
                let (text, offer) = carry(text, &source)?;
                words = offer.result(&source);
                Some(text)
            })
            .unwrap();
        assert!(words.starts_with("Carried over from Session 1: the recap, 1 plan line"));
        let drafted = fs::read_to_string(&tonight.path).unwrap();
        assert!(drafted.contains(&format!(
            "## Recap of last time\n\n{DRAFT_LINE}\n\nThe wolves came at dusk.\n\n## Plan"
        )));
        assert_eq!(Offer::of(&drafted, &source), None);

        // The DM writes What happened in the last note: the draft gives way to the embed.
        fs::write(
            &last.path,
            fs::read_to_string(&last.path).unwrap().replace(
                "## What happened\n\n",
                "## What happened\n\nWolves at the ford.\n\n",
            ),
        )
        .unwrap();
        let source = read(&Book::default());
        let undo = draft
            .apply_with_properties(tonight.id, |text| {
                let (text, offer) = carry(text, &source)?;
                words = offer.result(&source);
                Some(text)
            })
            .unwrap();
        assert_eq!(words, "The recap now follows your notes.");
        let swapped = fs::read_to_string(&tonight.path).unwrap();
        assert!(swapped.contains(&format!(
            "## Recap of last time\n\n![[{}#What happened]]\n\n## Plan",
            last.link()
        )));
        // A draft the DM changed stays.
        let changed = drafted.replace("at dusk.", "at dusk, and I added this.");
        assert_eq!(Offer::of(&changed, &source), None);
        undo.undo(&mut draft).unwrap();
        assert_eq!(fs::read_to_string(&tonight.path).unwrap(), drafted);
    }

    fn harness(
        dir: &Path,
        selected: Uuid,
        book: Book,
        fights: Vec<PlanFight>,
    ) -> egui_kittest::Harness<'static, (Sessions, Book)> {
        let notes = list_notes(dir);
        let path = notes
            .iter()
            .find(|note| note.id == selected)
            .unwrap()
            .path
            .clone();
        let page = Sessions {
            selected: Some(selected),
            notes,
            folder: Some(dir.to_owned()),
            draft: Some(Draft::open(&path).unwrap()),
            ..Sessions::default()
        };
        let harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(800.0, 300.0))
            .build_state(
                move |ctx, (page, book): &mut (Sessions, Book)| {
                    with_plan(&fights, |plan| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            page.last_time_block(ui, book, plan);
                            // The page's footer, as `session_footer` draws the result.
                            page.end_carry_results();
                            let act = std::cell::Cell::new(None);
                            let mut footer = design::Footer::new("carry", ui.available_width());
                            let mut undos = crate::timeline::page::LiveUndos::new();
                            page.carry_in_footer(&mut undos, &mut footer, &act);
                            undos.into_footer(&mut footer);
                            let _bottom = footer.show(ui);
                            if let Some(super::super::SessionAct::UndoCarry(session)) = act.take() {
                                page.undo_carry(session);
                            }
                        });
                    });
                },
                (page, book),
            );
        design::theme::install(&harness.ctx);
        harness
    }

    /// The act from the Last time line, where the DM acted: one save; the result and its Undo
    /// stay through an act on another session; a refusal writes nothing; a failed save keeps
    /// the Undo; after a restart the line has no action.
    #[test]
    fn carry_over_from_the_last_time_line_keeps_its_undo_by_the_session() {
        let dir = tempfile::tempdir().unwrap();
        let last = note(dir.path(), 1, "The Road", LAST);
        let tonight = note(dir.path(), 2, "The Crossing", TONIGHT);
        let fights = || vec![fight("Bridge Ambush"), fight("Shrine of Cinders")];
        let read = || fs::read_to_string(&tonight.path).unwrap();
        let before = read();

        // During play the line is as today: no action.
        let mut book = played(last.id);
        book.add(Kind::Day, Some(tonight.id));
        let mut playing = harness(dir.path(), tonight.id, book, fights());
        playing.run_steps(2);
        assert!(
            playing
                .query_by_label_contains("Session 1: The Road")
                .is_some()
        );
        assert!(playing.query_all_by_label("Carry over").next().is_none());

        let mut harness = harness(dir.path(), tonight.id, played(last.id), fights());
        harness.run_steps(2);
        assert!(
            harness
                .query_all_by_label("— Wolves at the ford.")
                .next()
                .is_some()
        );
        // The last note changes on disk after the line read it: the act writes nothing and says
        // so, and the line reads it again.
        let read_last = fs::read_to_string(&last.path).unwrap();
        fs::write(
            &last.path,
            read_last.replace(
                "The ferry is gone.\n",
                "The ferry is gone.\nThe bridge burned.\n",
            ),
        )
        .unwrap();
        harness.get_by_label("Carry over").click();
        harness.run_steps(2);
        assert_eq!(read(), before);
        assert!(
            harness
                .query_all_by_label(
                    "Session 1’s note changed. Check what it holds, then carry over again."
                )
                .next()
                .is_some()
        );
        fs::write(&last.path, &read_last).unwrap();
        harness.state_mut().0.last_time = None;
        harness.run_steps(2);
        // An edit on disk a moment before is in the words the act changes.
        let outside = before.replace(
            "## What happened\n\n",
            "## What happened\n\nFrom Obsidian.\n\n",
        );
        fs::write(&tonight.path, &outside).unwrap();
        harness.get_by_label("Carry over").click();
        harness.run_steps(2);
        let carried = read();
        assert!(carried.contains("From Obsidian."));
        assert!(carried.contains("- [ ] Who burned the ropes?"));
        assert!(
            harness
                .query_all_by_label(
                    "Carried over from Session 1: the recap, 2 plan lines and 2 open threads."
                )
                .next()
                .is_some()
        );
        assert!(harness.query_all_by_label("Carry over").next().is_none());

        // Review of PR 298, item 2: A's result (tonight's) keeps the order its act issued. B's act
        // on another session comes after it, through the one path every act takes, then the DM
        // goes to B and back to A: A's order is unchanged, and older than B's.
        let a_order = harness.state().0.carries.order(tonight.id);
        harness.state_mut().0.carries.record(
            last.id,
            Outcome::Refused {
                words: "B's act.".into(),
                seen: 0,
            },
        );
        let b_order = harness.state().0.carries.order(last.id);
        harness.state_mut().0.select(Some(last.id));
        harness.run_steps(2);
        harness.state_mut().0.select(Some(tonight.id));
        harness.run_steps(2);
        let a_again = harness.state().0.carries.order(tonight.id);
        assert!(
            a_again == a_order && b_order > a_order,
            "A's order {a_order:?} stays {a_again:?} after B's act {b_order:?}"
        );
        harness.state_mut().0.carries.results.remove(&last.id);

        // An act on another session, then back: the Undo is this session's, and it still works.
        let page = &mut harness.state_mut().0;
        page.select(Some(last.id));
        let draft = page.draft.as_mut().unwrap();
        draft
            .apply(last.id, Apply::Now, |text| {
                Some(text.replace("- [x] Read the rumours", "- [ ] Read the rumours"))
            })
            .unwrap();
        harness.run_steps(2);
        harness.state_mut().0.select(Some(tonight.id));
        harness.run_steps(2);
        // The last session's note goes, and the campaign is read again: the result and its Undo
        // are this note's, and they stay (review of PR 277, finding 3).
        let last_text = fs::read_to_string(&last.path).unwrap();
        fs::remove_file(&last.path).unwrap();
        let rescan = |harness: &mut egui_kittest::Harness<'_, (Sessions, Book)>| {
            let page = &mut harness.state_mut().0;
            page.notes = list_notes(dir.path());
            page.last_time = None;
            page.carries.scanned();
            harness.run_steps(2);
        };
        rescan(&mut harness);
        assert!(
            harness
                .query_all_by_label("Undo carry over")
                .next()
                .is_some()
        );
        fs::write(&last.path, &last_text).unwrap();
        rescan(&mut harness);
        harness.get_by_label("Undo carry over").click();
        harness.run_steps(2);
        assert_eq!(read(), outside);
        assert!(
            harness
                .query_all_by_label("Undo carry over")
                .next()
                .is_none()
        );

        // Unsaved words meet a changed file: nothing is written, and the line says why.
        let draft = harness.state_mut().0.draft.as_mut().unwrap();
        draft.body.push_str("Typed.\n");
        draft.edited();
        let changed = outside.replace("From Obsidian.", "Changed in Obsidian.");
        fs::write(&tonight.path, &changed).unwrap();
        harness.run_steps(1);
        harness.get_by_label("Carry over").click();
        harness.run_steps(2);
        assert_eq!(read(), changed);
        assert!(
            harness
                .query_all_by_label("The note changed on disk. Choose in the note first.")
                .next()
                .is_some()
        );

        // A failed save: the fragment waits in the draft, the line says so, and Undo takes it
        // out of the draft.
        harness.state_mut().0.select(Some(last.id));
        harness.state_mut().0.draft = None;
        harness.state_mut().0.select(Some(tonight.id));
        harness.run_steps(2);
        let locked = |mode| {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(dir.path(), fs::Permissions::from_mode(mode)).unwrap();
        };
        locked(0o555);
        harness.get_by_label("Carry over").click();
        harness.run_steps(2);
        locked(0o755);
        assert_eq!(read(), changed);
        assert!(
            harness
                .query_all_by_label("Notes not saved")
                .next()
                .is_some()
        );
        assert!(
            harness
                .state()
                .0
                .draft
                .as_ref()
                .unwrap()
                .body
                .contains("Who burned the ropes?")
        );
        harness.get_by_label("Undo carry over").click();
        harness.run_steps(2);
        let draft = harness.state().0.draft.as_ref().unwrap();
        assert!(!draft.text().contains("carried_"));
        assert!(!draft.body.contains("Who burned the ropes?"));
        // The save fails again, and a later save writes the note with no change of its words:
        // the result says so, and its Undo writes the note back at once.
        harness.run_steps(1);
        locked(0o555);
        harness.get_by_label("Carry over").click();
        harness.run_steps(2);
        locked(0o755);
        harness.state_mut().0.draft.as_mut().unwrap().save();
        harness.run_steps(2);
        assert!(read().contains("Who burned the ropes?"));
        assert!(
            harness
                .query_all_by_label("Notes not saved")
                .next()
                .is_none()
        );
        // Act B unticked a scene of the last session, so it is still to come now.
        assert!(
            harness
                .query_all_by_label(
                    "Carried over from Session 1: the recap, 3 plan lines and 2 open threads."
                )
                .next()
                .is_some()
        );
        harness.get_by_label("Undo carry over").click();
        harness.run_steps(2);
        assert_eq!(read(), changed);

        // A restart: the note holds what the act wrote, and every part is done.
        let mut draft = Draft::open(&tonight.path).unwrap();
        with_plan(&fights(), |plan| {
            let source = Source::read(&last, dir.path(), plan, &played(last.id)).unwrap();
            draft
                .apply_with_properties(tonight.id, |text| carry(text, &source).map(|(t, _)| t))
                .unwrap();
        });
        let written = read();
        let mut restarted = harness_for(dir.path(), tonight.id, fights());
        restarted.run_steps(2);
        assert!(restarted.query_all_by_label("Carry over").next().is_none());
        assert_eq!(read(), written);
    }

    /// Row 18: Read draws an embed of a session note's heading by its target, resolved as every
    /// campaign link is, never by a part of its name. A source that is gone says so, and no
    /// embed is ever rewritten: the DM's own kept recap neither. Read's words are prepared when
    /// what they are made from changes, not on a quiet frame.
    #[test]
    fn read_draws_a_session_embed_by_its_target() {
        use egui_kittest::kittest::Queryable;
        let root = tempfile::tempdir().unwrap();
        let campaign = crate::persistence::Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: root.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let dir = root.path().join(crate::sessions::DEFAULT_FOLDER);
        fs::create_dir_all(&dir).unwrap();
        let last = note(&dir, 1, "The Road", LAST);
        // Two notes with one file name in two folders (review of PR 277, finding 2).
        let first = note(
            &dir,
            5,
            "First",
            "## What happened\n\nThe correct first session.\n",
        );
        let second = note(
            &dir,
            6,
            "Second",
            "## What happened\n\nThe WRONG second session.\n",
        );
        for (note, folder) in [(&first, "Archive"), (&second, "Recent")] {
            fs::create_dir_all(dir.join(folder)).unwrap();
            fs::rename(&note.path, dir.join(folder).join("Same.md")).unwrap();
        }
        // The DM's own embed is the recap (finding 1), and the notes below embed by target.
        let own = TONIGHT.replace(
            "## Recap of last time\n\n",
            "## Recap of last time\n\n![[My missing session#What happened]]\n\n",
        );
        let tonight = note(
            &dir,
            2,
            "The Crossing",
            &format!(
                "{own}## Notes\n\n![[{}#What happened]]\n\n![[Sessions/Archive/Same#What happened]]\n\n\
                 ![[Archive/Same#What happened]]\n\n![[Same#What happened]]\n\n\
                 ![[Sessions/Deleted/Same#What happened]]\n\n![[Recent/Same#No such part]]\n\n{}",
                last.link(),
                "Some written words.\n".repeat(200)
            ),
        );
        let mut draft = Draft::open(&tonight.path).unwrap();
        with_plan(&[], |plan| {
            let source = Source::read(&last, &dir, plan, &Book::default()).unwrap();
            let (_, offer) = carry(&draft.text(), &source).unwrap();
            assert_eq!(offer.recap, Recap::Kept);
            draft
                .apply_with_properties(tonight.id, |text| carry(text, &source).map(|(t, _)| t))
                .unwrap();
        });
        let carried = fs::read_to_string(&tonight.path).unwrap();
        let page = Sessions {
            selected: Some(tonight.id),
            notes: list_notes(&dir),
            folder: Some(dir.clone()),
            draft: Some(Draft::open(&tonight.path).unwrap()),
            reading_note: true,
            ..Sessions::default()
        };
        let load = || Arc::new(crate::lookup::ContentIndex::load(&[], Some(&campaign)).unwrap());
        let index = std::rc::Rc::new(std::cell::RefCell::new(load()));
        let shared = std::rc::Rc::clone(&index);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(800.0, 4000.0))
            .build_state(
                move |ctx, (page, book): &mut (Sessions, Book)| {
                    let index = Arc::clone(&shared.borrow());
                    ctx.data_mut(|data| {
                        data.insert_temp(egui::Id::new("reader_link_index"), index);
                    });
                    with_plan(&[], |plan| {
                        egui::CentralPanel::default()
                            .show(ctx, |ui| page.read_tab(ui, book, plan, true));
                    });
                },
                (page, Book::default()),
            );
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let has = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, words: &str| {
            harness.query_by_label_contains(words).is_some()
        };
        assert!(
            harness
                .query_all_by_label("Session 1: The Road")
                .next()
                .is_some()
        );
        assert!(has(&harness, "Wolves at the ford."));
        // A target with its folder shows that note. A part of a path, and a name that two notes
        // share, stay links: a session note has that name, so neither is called gone. Never the
        // other session's words.
        assert!(has(&harness, "The correct first session."));
        assert!(!has(&harness, "The WRONG second session."));
        assert!(!has(&harness, "The recap’s source is gone: Archive/Same"));
        assert!(!has(&harness, "The recap’s source is gone: Same"));
        // A path that no note has is gone, though a note has its last name (review of PR 277,
        // round 2), and Read says how to mend it.
        assert!(has(
            &harness,
            "The recap’s source is gone: Sessions/Deleted/Same › What happened. Link the session again in Write."
        ));
        // A part of a path whose note lacks the heading is gone too (review of R3d).
        assert!(has(
            &harness,
            "The recap’s source is gone: Recent/Same › No such part."
        ));
        for (link, names) in [
            ("Archive/SAME.MD", true),
            ("/Sessions/Archive/Same", true),
            ("chive/Same", false),
            ("Archive\\Same", false),
        ] {
            assert_eq!(
                names_file(&dir.join("Archive/Same.md"), link),
                names,
                "{link}"
            );
        }
        // The DM's kept recap names nothing: Read says so, and nothing offers to rewrite it.
        assert!(has(
            &harness,
            "The recap’s source is gone: My missing session › What happened."
        ));
        assert!(
            harness
                .query_all_by_label("Update the link")
                .next()
                .is_none()
        );

        // Quiet frames prepare nothing (finding 5).
        let work = crate::test_support::CachedWork::start(
            "Read's prepared words",
            &mut harness,
            |harness: &mut egui_kittest::Harness<'_, (Sessions, Book)>| {
                harness.state().0.carries.read_counts
            },
        );
        work.steady(&mut harness, |harness| {
            harness.run_steps(3);
            has(harness, "Wolves at the ford.")
        })
        .expect("read-steady");

        // The last note is renamed outside Obsidian: its embed now names nothing, and says so.
        fs::rename(&last.path, dir.join("Session 1 - The Old Road.md")).unwrap();
        *index.borrow_mut() = load();
        let page = &mut harness.state_mut().0;
        page.notes = list_notes(&dir);
        page.carries.scanned();
        harness.run_steps(2);
        assert!(!has(&harness, "Wolves at the ford."));
        assert!(has(
            &harness,
            &format!(
                "The recap’s source is gone: {} › What happened.",
                last.link()
            )
        ));
        assert_eq!(fs::read_to_string(&tonight.path).unwrap(), carried);
    }

    fn harness_for(
        dir: &Path,
        selected: Uuid,
        fights: Vec<PlanFight>,
    ) -> egui_kittest::Harness<'static, (Sessions, Book)> {
        let last = list_notes(dir)
            .into_iter()
            .find(|note| note.number == 1)
            .unwrap();
        harness(dir, selected, played(last.id), fights)
    }
}
