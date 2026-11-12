//! Carry-over (F5 of `docs/design/session-cycle.md`): what the last session left, added to this
//! session's note in one act with its exact Undo; and an embed of a session note's heading,
//! which Read draws by its target.
use super::{Plan, Sessions, prose};
use crate::{
    content_workspace::patch_fields,
    design::{self, Action, Tone, Type, colors},
    encounter_run::RunGroup,
    encounters::PlanFight,
    sessions::{
        ActFault, Note, NoteUndo, body_of, each_linked_name, list_recordings, section_text,
        split_note, task_parts, ticked,
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
    let Some((front, _)) = split_note(text) else {
        return Carried::default();
    };
    front
        .trim_start_matches('\u{feff}')
        .trim_end()
        .strip_suffix("---")
        .map(|inner| inner.trim_start_matches("---"))
        .and_then(|inner| serde_yaml_ng::from_str(inner).ok())
        .unwrap_or_default()
}

/// The session a note carried from, once it did. Carry over then reads only that session.
pub(super) fn carried_from(text: &str) -> Option<Uuid> {
    carried(text).carried_from
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
        for event in &book.events {
            if event.is_in(Some(note.id)) && event.run.is_some() {
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
        place: Place,
        words: String,
        /// `Undo carry over` or `Undo link`.
        action: &'static str,
        undo: NoteUndo,
        /// The draft's revision when the Undo was last checked.
        seen: u64,
    },
    /// Why the act wrote nothing, until the note's text changes.
    Refused {
        place: Place,
        words: String,
        seen: u64,
    },
}

impl Outcome {
    fn place(&self) -> Place {
        match self {
            Self::Done { place, .. } | Self::Refused { place, .. } => *place,
        }
    }
}

/// Where the DM acted, so the result shows there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    /// `Carry over`, on the Last time line.
    Line,
    /// `Update the link`, in Read, under the embed.
    Embed,
}

/// The results of carry-over acts, by the session note each act changed, and the embeds of
/// session notes in the open note, as Read draws them.
#[derive(Default)]
pub(super) struct Carries {
    results: BTreeMap<Uuid, Outcome>,
    /// The embeds, for the draft's revision and the content they were read from. A folder
    /// scan reads them again, as the notes they show can change.
    embeds: Option<Embeds>,
}

/// The embeds of the open note, with the draft's revision and the content's generation they
/// were read for.
type Embeds = ((u64, Option<u64>), Arc<Vec<EmbedLine>>);

impl Carries {
    /// The folder was scanned: an embed's note can have changed or gone.
    pub(super) fn scanned(&mut self) {
        self.embeds = None;
    }

    /// The note moved to a new file, and its Undo follows it.
    pub(super) fn moved(&mut self, from: &Path, to: &Path) {
        self.embeds = None;
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
        let ended = match self.results.get_mut(&session) {
            Some(Outcome::Done { undo, seen, .. }) if *seen != revision => {
                *seen = revision;
                undo.follow_save(draft);
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
    fn preparing(&self, session: Uuid, book: &Book) -> bool {
        Some(session) == self.current()
            && self
                .open_run
                .as_ref()
                .is_none_or(|(_, run)| *run != session)
            && self
                .live
                .as_ref()
                .is_none_or(|live| live.session != session)
            && !book.events.iter().any(|event| event.is_in(Some(session)))
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
            .and_then(|draft| carried_from(&draft.text()));
        let note = match from {
            Some(id) => self.notes.iter().find(|note| note.id == id)?,
            None => previous,
        };
        let folder = self.folder.as_deref()?;
        Some(Source::read(note, folder, plan, book))
    }

    /// What `Carry over` would add now, read again only when the note's words change.
    fn carry_offer(&mut self) -> Option<Result<Offer, String>> {
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
            (Ok(_), Some((_, Some(offer)))) => Some(Ok(offer.clone())),
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
        self.carries.end_what_changed(session, self.draft.as_ref());
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
            Some(Ok(offer)) => (Some(offer.hover()), None),
            Some(Err(words)) => (None, Some(words.as_str())),
            None => (None, None),
        };
        let mut open = false;
        let mut carry = false;
        ui.horizontal(|ui| {
            ui.label(Type::Small.rich(ui, "Last time:").color(colors().muted));
            let action = offer.is_some().then(|| {
                let mut action = Action::quiet("Carry over").in_line().blocked(blocked);
                if let Some(hover) = &hover {
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
            let words = design::words_that_fit(
                ui,
                &if preview.is_empty() {
                    String::new()
                } else {
                    format!("— {preview}")
                },
                &Type::Small.font(ui),
                ui.available_width() - carry_room,
            );
            if !words.is_empty() || action.is_none() {
                ui.label(Type::Small.rich(ui, words).color(colors().muted))
                    .on_hover_text(&full);
            }
            if let Some(action) = action {
                ui.add_space((ui.available_width() - carry_room + design::L).max(design::L));
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
        self.carry_result(ui, session, Place::Line);
    }

    /// The act. It reads the last session again at the click, and writes nothing when what the
    /// line offered changed; otherwise one save of this note through `Draft::apply`.
    fn carry_over(&mut self, session: Uuid, previous: Uuid, plan: &Plan<'_>, book: &Book) {
        let Some(previous) = self.notes.iter().find(|note| note.id == previous).cloned() else {
            return;
        };
        let shown = self
            .last_time
            .as_ref()
            .and_then(|(_, last)| last.as_ref()?.source.clone());
        let now = self.carry_source(&previous, plan, book);
        let seen = self
            .draft
            .as_ref()
            .map_or(0, crate::sessions::Draft::revision);
        let source = match (&now, &shown) {
            (Some(Ok(now)), Some(Ok(shown))) if now == shown => now.clone(),
            (Some(Err(words)), _) => {
                self.carries.results.insert(
                    session,
                    Outcome::Refused {
                        place: Place::Line,
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
                self.carries.results.insert(
                    session,
                    Outcome::Refused {
                        place: Place::Line,
                        words,
                        seen,
                    },
                );
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
                place: Place::Line,
                words: offer.map(|offer| offer.result(&source)).unwrap_or_default(),
                action: "Undo carry over",
                undo,
                seen,
            },
            Err(fault) => Outcome::Refused {
                place: Place::Line,
                words: fault.words().to_owned(),
                seen,
            },
        };
        self.carries.results.insert(session, outcome);
    }

    /// The result of this session's act, where the DM acted: its words and its Undo, or why it
    /// wrote nothing. A save that failed says so, and the Undo still works.
    fn carry_result(&mut self, ui: &mut egui::Ui, session: Uuid, place: Place) {
        let undo = match self
            .carries
            .results
            .get(&session)
            .filter(|outcome| outcome.place() == place)
        {
            Some(Outcome::Refused { words, .. }) => {
                design::message(ui, Tone::Warning, words);
                false
            }
            Some(Outcome::Done {
                words,
                action,
                undo,
                ..
            }) => {
                let (tone, words) = if undo.written() {
                    (Tone::Success, words.as_str())
                } else {
                    (
                        Tone::Warning,
                        ActFault::NotSaved(String::new(), undo.clone()).words(),
                    )
                };
                design::undo_message(ui, tone, words, action)
            }
            None => false,
        };
        if !undo {
            return;
        }
        let (Some(Outcome::Done { undo, .. }), Some(draft)) =
            (self.carries.results.remove(&session), self.draft.as_mut())
        else {
            return;
        };
        if let Err(fault) = undo.undo(draft) {
            let seen = draft.revision();
            self.carries.results.insert(
                session,
                Outcome::Refused {
                    place,
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
    /// The line as the note holds it, and its number in the note's words.
    line: String,
    at: usize,
    shown: Shown,
}

#[derive(Clone, Debug)]
enum Shown {
    /// The heading's words, under `From <session>`, which links that session. `update` is the
    /// line with the note's new name, for the embed that Carry over wrote when its note was
    /// renamed outside Obsidian.
    Words {
        from: Uuid,
        label: String,
        words: String,
        update: Option<String>,
    },
    /// The note or its heading is gone. The line stays in the note.
    Gone(String),
}

/// The note's name and heading of a line that is only an embed of a note's heading.
fn embed_target(line: &str) -> Option<(&str, &str)> {
    let inner = line.trim().strip_prefix("![[")?.strip_suffix("]]")?;
    let inner = inner.split('|').next()?;
    let (name, heading) = inner.split_once('#')?;
    let name = name.rsplit('/').next()?.trim();
    let name = name.strip_suffix(".md").unwrap_or(name);
    let heading = heading.trim();
    (!name.is_empty() && !heading.is_empty() && !heading.starts_with('^'))
        .then_some((name, heading))
}

impl Sessions {
    /// The open note's embeds of session notes' headings, read when its words or the content
    /// change and at each folder scan, never on a quiet frame. An embed of another note that
    /// the content holds stays a link, as today.
    fn embeds(&mut self, index: Option<&crate::lookup::ContentIndex>) -> Arc<Vec<EmbedLine>> {
        let Some(draft) = &self.draft else {
            return Arc::default();
        };
        let key = (
            draft.revision(),
            index.map(crate::lookup::ContentIndex::generation),
        );
        if let Some((seen, embeds)) = &self.carries.embeds
            && *seen == key
        {
            return Arc::clone(embeds);
        }
        let carried = carried_from(&draft.text());
        let mut embeds = Vec::new();
        let mut section = "";
        for (at, line) in draft.body.lines().enumerate() {
            if let Some(heading) = line.strip_prefix("## ") {
                section = heading.trim();
                continue;
            }
            let Some((name, heading)) = embed_target(line) else {
                continue;
            };
            let named = self
                .notes
                .iter()
                .find(|note| note.link().eq_ignore_ascii_case(name));
            // The embed that Carry over wrote finds its session by `carried_from`.
            let renamed = || {
                carried
                    .filter(|_| section == RECAP && heading == WHAT_HAPPENED)
                    .and_then(|id| self.notes.iter().find(|note| note.id == id))
            };
            let gone = || Shown::Gone(format!("The recap’s source is gone: {name} › {heading}."));
            let shown = match named.map(|note| (note, None)).or_else(|| {
                renamed().map(|note| (note, Some(format!("![[{}#{heading}]]", note.link()))))
            }) {
                Some((note, update)) => fs::read_to_string(&note.path)
                    .ok()
                    .and_then(|text| section_text(body_of(&text), heading))
                    .map_or_else(gone, |words| Shown::Words {
                        from: note.id,
                        label: note.label(),
                        words,
                        update,
                    }),
                None if index.is_some_and(|index| {
                    matches!(index.resolve_link(name, None), crate::world::Go::Missing(_))
                }) =>
                {
                    gone()
                }
                None => continue,
            };
            embeds.push(EmbedLine {
                line: line.trim_end().to_owned(),
                at,
                shown,
            });
        }
        let embeds = Arc::new(embeds);
        self.carries.embeds = Some((key, Arc::clone(&embeds)));
        embeds
    }

    /// Read's words with each embed line marked, so that Read draws it by its target.
    pub(super) fn mark_embeds(
        &mut self,
        body: &str,
        index: Option<&crate::lookup::ContentIndex>,
    ) -> (String, Arc<Vec<EmbedLine>>) {
        let embeds = self.embeds(index);
        if embeds.is_empty() {
            return (body.to_owned(), embeds);
        }
        let marked = body
            .split_inclusive('\n')
            .map(|line| {
                match embeds
                    .iter()
                    .position(|embed| embed.line == line.trim_end())
                {
                    Some(at) => format!("{EMBED_MARK}{at}{EMBED_MARK}\n"),
                    None => line.to_owned(),
                }
            })
            .collect();
        (marked, embeds)
    }

    /// Read's Markdown, with each marked embed drawn as its heading's words under a muted line
    /// that links its session, or as the words that say its source is gone.
    pub(super) fn show_reader_with_embeds(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        embeds: &[EmbedLine],
        book: &Book,
    ) {
        let mut part = String::new();
        let mut open = None;
        let mut update = None;
        for line in text.split_inclusive('\n') {
            let Some(embed) = line
                .trim_end()
                .strip_prefix(EMBED_MARK)
                .and_then(|rest| rest.strip_suffix(EMBED_MARK))
                .and_then(|at| at.parse::<usize>().ok())
                .and_then(|at| embeds.get(at))
            else {
                part.push_str(line);
                continue;
            };
            if !part.trim().is_empty() {
                crate::markdown_view::show_reader(ui, &part);
            }
            part.clear();
            match &embed.shown {
                Shown::Words {
                    from,
                    label,
                    words,
                    update: new,
                } => {
                    ui.horizontal(|ui| {
                        ui.label(Type::Small.rich(ui, "From").color(colors().muted));
                        if design::small_link(ui, label).clicked() {
                            open = Some(*from);
                        }
                        if let Some(new) = new
                            && Action::quiet("Update the link")
                                .in_line()
                                .show(ui)
                                .clicked()
                        {
                            update = Some((embed.at, embed.line.clone(), new.clone()));
                        }
                    });
                    crate::markdown_view::show_reader(ui, &super::words_to_read(words, book));
                }
                Shown::Gone(words) => {
                    design::small_message(ui, Tone::Warning, words);
                }
            }
            if let Some(session) = self.selected {
                self.carry_result(ui, session, Place::Embed);
            }
        }
        if !part.trim().is_empty() {
            crate::markdown_view::show_reader(ui, &part);
        }
        if let Some(session) = open {
            self.select(Some(session));
        } else if let Some((at, line, new)) = update {
            self.update_link(at, &line, &new);
        }
    }

    /// `Update the link`: the embed line that Carry over wrote names the note's new name. One
    /// save of this note, through the note's one act path.
    fn update_link(&mut self, at: usize, line: &str, new: &str) {
        let (Some(session), Some(draft)) = (self.selected, self.draft.as_mut()) else {
            return;
        };
        let result = draft.apply(session, crate::sessions::Apply::Now, |text| {
            crate::sessions::change_line(text, at, line, Some(new))
        });
        let seen = draft.revision();
        let outcome = match result {
            Ok(undo) | Err(ActFault::NotSaved(_, undo)) => Outcome::Done {
                place: Place::Embed,
                words: format!(
                    "The link now names {}.",
                    embed_target(new).map_or(new, |(name, _)| name)
                ),
                action: "Undo link",
                undo,
                seen,
            },
            Err(fault) => Outcome::Refused {
                place: Place::Embed,
                words: fault.words().to_owned(),
                seen,
            },
        };
        self.carries.results.insert(session, outcome);
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
        ## Plan for tonight\n\n- [[Bridge Ambush]]\n- [x] [[Shrine of Cinders]]\n\
        - [ ] Talk to [[Hobb]] about the ropes\n- [x] Read the rumours\nRemember the toll.\n\n\
        ## What happened\n\nWolves at the ford.\n\n\
        ## Open threads\n\n- [ ] Who burned the ropes?\n- [x] Who paid the toll?\nThe ferry is gone.\n";
    const TONIGHT: &str =
        "## Recap of last time\n\n## Plan for tonight\n\n- [[Ashfall]]\n\n## What happened\n\n";

    fn fight(name: &str) -> PlanFight {
        PlanFight {
            saved: None,
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

    /// The book of the last session: `Bridge Ambush` was played; `Shrine of Cinders` was not.
    fn played(session: Uuid) -> Book {
        let mut book = Book::default();
        book.add(Kind::Encounter, Some(session));
        book.events[0].title = "Bridge Ambush".into();
        book.events[0].run = Some(Uuid::nil());
        book
    }

    fn with_plan<R>(fights: &[PlanFight], add: impl FnOnce(&Plan<'_>) -> R) -> R {
        let mut group = |_: &mut egui::Ui| {};
        let plan = Plan {
            fights,
            saved_runs: &[],
            groups: &[],
            group_blocked: None,
            group_open: false,
            group_discarded: false,
            group_discarded_name: String::new(),
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
            fight("Shrine of Cinders"),
            fight("Ashfall"),
        ];
        let source = with_plan(&fights, |plan| {
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
             - [[Ashfall]]\n- [ ] [[Shrine of Cinders]]\n- [ ] Talk to [[Hobb]] about the ropes\n\n\
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
            "Carried over from Session 1: the recap, 2 plan lines and 2 open threads."
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
            "Carried over from Session 1: 1 plan line and 2 open threads. Your recap was kept."
        );
        assert_eq!(
            offer.hover(),
            "Add the last session’s 1 plan line and 2 open threads to this note."
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
                        egui::CentralPanel::default()
                            .show(ctx, |ui| page.last_time_block(ui, book, plan));
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
        assert!(playing.query_by_label("Carry over").is_none());

        let mut harness = harness(dir.path(), tonight.id, played(last.id), fights());
        harness.run_steps(2);
        assert!(harness.query_by_label("— Wolves at the ford.").is_some());
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
                .query_by_label(
                    "Carried over from Session 1: the recap, 2 plan lines and 2 open threads."
                )
                .is_some()
        );
        assert!(harness.query_by_label("Carry over").is_none());

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
        harness.get_by_label("Undo carry over").click();
        harness.run_steps(2);
        assert_eq!(read(), outside);
        assert!(harness.query_by_label("Undo carry over").is_none());

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
                .query_by_label("The note changed on disk. Choose in the note first.")
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
        assert!(harness.query_by_label("Notes not saved").is_some());
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
        assert!(restarted.query_by_label("Carry over").is_none());
        assert_eq!(read(), written);
    }

    /// Row 18: Read draws an embed of a session note's heading by its target. The embed that
    /// Carry over wrote follows its session by `carried_from` after a rename outside Obsidian,
    /// and `Update the link` writes the new name, with `Undo link`. A source that is gone says so.
    #[test]
    fn read_draws_a_session_embed_by_its_target() {
        let dir = tempfile::tempdir().unwrap();
        let last = note(dir.path(), 1, "The Road", LAST);
        let tonight = note(dir.path(), 2, "The Crossing", TONIGHT);
        let mut draft = Draft::open(&tonight.path).unwrap();
        with_plan(&[], |plan| {
            let source = Source::read(&last, dir.path(), plan, &Book::default()).unwrap();
            draft
                .apply_with_properties(tonight.id, |text| carry(text, &source).map(|(t, _)| t))
                .unwrap();
        });
        let mut page = Sessions {
            selected: Some(tonight.id),
            notes: list_notes(dir.path()),
            folder: Some(dir.path().to_owned()),
            draft: Some(Draft::open(&tonight.path).unwrap()),
            reading_note: true,
            ..Sessions::default()
        };
        page.last_time = Some((tonight.id, None));
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(800.0, 600.0))
            .build_state(
                |ctx, (page, book): &mut (Sessions, Book)| {
                    with_plan(&[], |plan| {
                        egui::CentralPanel::default()
                            .show(ctx, |ui| page.read_tab(ui, book, plan, true));
                    });
                },
                (page, Book::default()),
            );
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        assert!(harness.query_by_label("From").is_some());
        assert!(harness.query_by_label("Session 1: The Road").is_some());
        assert!(
            harness
                .query_by_label_contains("Wolves at the ford.")
                .is_some()
        );
        assert!(harness.query_by_label_contains("What happened").is_none());
        assert!(harness.query_by_label("Update the link").is_none());

        // Renamed outside Obsidian: the text still shows, and the line can name the new file.
        let renamed = dir.path().join("Session 1 - The Old Road.md");
        fs::rename(&last.path, &renamed).unwrap();
        let page = &mut harness.state_mut().0;
        page.notes = list_notes(dir.path());
        page.carries.scanned();
        harness.run_steps(2);
        assert!(
            harness
                .query_by_label_contains("Wolves at the ford.")
                .is_some()
        );
        let before = fs::read_to_string(&tonight.path).unwrap();
        harness.get_by_label("Update the link").click();
        harness.run_steps(2);
        let updated = fs::read_to_string(&tonight.path).unwrap();
        assert_eq!(
            updated,
            before.replace(
                &format!("![[{}#What happened]]", last.link()),
                "![[Session 1 - The Old Road#What happened]]"
            )
        );
        assert!(
            harness
                .query_by_label("The link now names Session 1 - The Old Road.")
                .is_some()
        );
        harness.get_by_label("Undo link").click();
        harness.run_steps(2);
        assert_eq!(fs::read_to_string(&tonight.path).unwrap(), before);

        // The note's heading is gone: the line stays in the note, and Read says so.
        fs::write(
            &renamed,
            fs::read_to_string(&renamed)
                .unwrap()
                .replace("## What happened", "## Later"),
        )
        .unwrap();
        harness.state_mut().0.carries.scanned();
        harness.run_steps(2);
        assert!(
            harness
                .query_by_label_contains(&format!(
                    "The recap’s source is gone: {} › What happened.",
                    last.link()
                ))
                .is_some()
        );
        assert_eq!(fs::read_to_string(&tonight.path).unwrap(), before);
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
