//! The Session page's plan: its rows (planned, open and played fights, groups and events), the
//! Add strip, a row's menu and its opened detail.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{Plan, Sessions, kept, kept::PlanKind, plan_summary, wrapped_words};
use crate::{
    design::{
        self, Action, Density, Status, Tone, Type, colors, icons,
        outcome::{
            Choice, Fault as MessageFault, FaultMessage, KeptLine, KeptOutcome, MessageChoice,
            Messages, Notice, Outcome, PaneFrame, PlacedResult, Said, Sentence, Severity, UndoStep,
            Verb,
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
        self, Book, EventDisplay, Kind, checks,
        page::{
            PlanNames, added_message, file_act, file_messages, long_rest_status, problems,
            quick_add_with, rest_ruling,
        },
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

/// One draw of the plan's one-time reveal: after the DM adds a row or acts on one, the row after
/// it scrolls into view above the message footer, so its action needs no extra gesture.
struct Reveal {
    target: Option<String>,
    /// The target row's rect, until the next row is drawn.
    armed: Option<egui::Rect>,
    shown: bool,
}

impl Reveal {
    fn new(target: Option<String>) -> Self {
        Self {
            target,
            armed: None,
            shown: false,
        }
    }

    /// After each row: the row after the target scrolls into view when it is not.
    fn after(&mut self, ui: &egui::Ui, id: Option<&str>, rect: egui::Rect) {
        if self.armed.take().is_some() {
            self.show(ui, rect);
        }
        if id.is_some() && id == self.target.as_deref() {
            self.armed = Some(rect);
        }
    }

    /// After the last row: with no row after it, the target itself.
    fn finish(&mut self, ui: &egui::Ui) {
        if let Some(rect) = self.armed.take() {
            self.show(ui, rect);
        }
    }

    fn show(&mut self, ui: &egui::Ui, rect: egui::Rect) {
        self.shown = ui.clip_rect().contains_rect(rect);
        if !self.shown {
            ui.scroll_to_rect(rect, None);
        }
    }
}

/// How many draws a reveal may take before it gives up.
const REVEAL_DRAWS: u8 = 3;

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
pub(super) enum FooterAct {
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
const FIGHT_SEARCH_FROM: usize = 8;
const FIGHT_MENU_WIDTH: f32 = 240.0;
const FIGHT_MENU_HEIGHT: f32 = 240.0;
/// The kinds a session adds by one press. A fight has its own control, for what it can name.
const PLAN_KINDS: [Kind; 5] = [
    Kind::Day,
    Kind::LongRest,
    Kind::ShortRest,
    Kind::Travel,
    Kind::Event,
];

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

/// Keep each summary item and its separator together, including the rest ruling.
/// An event row's summary as one job (layout 11c A2c): its words, a length the clock cannot
/// count in the warning's colour, and a long rest's status. `None` when it says nothing.
fn plan_event_summary_job(
    ui: &egui::Ui,
    summary: &str,
    restores: Option<bool>,
    warning: Option<&str>,
) -> Option<egui::text::LayoutJob> {
    let mut job = egui::text::LayoutJob::default();
    let format = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: colors().muted,
        ..Default::default()
    };
    let text = plan_summary(summary);
    // Round 26 FG4, FD7: a length the clock cannot count is a warning part, in its colour.
    let warning = warning.map(plan_summary);
    match warning
        .as_deref()
        .and_then(|warning| text.find(warning).map(|at| (at, warning.len())))
    {
        Some((at, len)) => {
            job.append(&text[..at], 0.0, format.clone());
            job.append(
                &text[at..at + len],
                0.0,
                egui::TextFormat {
                    color: colors().result.warning,
                    ..format.clone()
                },
            );
            job.append(&text[at + len..], 0.0, format.clone());
        }
        None => job.append(&text, 0.0, format.clone()),
    }
    if let Some(restores) = restores {
        let status = long_rest_status(restores);
        let words = format!(
            "{}{}\u{a0}{}",
            if summary.is_empty() { "" } else { " ·\u{a0}" },
            status.icon,
            status.word.replace(' ', "\u{a0}")
        );
        job.append(
            &words,
            0.0,
            egui::TextFormat {
                color: status.color,
                ..format
            },
        );
    }
    (!job.text.is_empty()).then_some(job)
}

#[cfg(test)]
fn plan_event_summary(
    ui: &mut egui::Ui,
    summary: &str,
    restores: Option<bool>,
    warning: Option<&str>,
) {
    if let Some(job) = plan_event_summary_job(ui, summary, restores, warning) {
        let response = ui.add(egui::Label::new(job).wrap());
        if warning.is_some() {
            response.on_hover_text(timeline::TIME_REPAIR);
        }
    }
}

/// The plan's door to Arrivals while a fight is open. The run uses the same words.
const ADD_TO_FIGHT: &str = "Add to fight…";

/// A plan group's summary that wraps between members, never inside a member's
/// `N × name (as base)`, as Arrivals does.
fn plan_summary_text(ui: &egui::Ui, group: &crate::encounter_run::RunGroup, room: f32) -> String {
    let summary = group.summary();
    let members = group.member_summary();
    match summary.strip_suffix(&members) {
        Some(head) if !members.is_empty() => format!(
            "{head}{}",
            group.member_summary_wrapped(ui, &Type::Small.font(ui), room)
        ),
        _ => summary,
    }
}

/// The separator between a summary's facts: it wraps with the words after it.
const FACT_SEPARATOR: &str = " ·\u{a0}";

/// FI7: `text` with its facts joined by `FACT_SEPARATOR`, laid out at `width` so that no line
/// ends or starts with a loose `·`: where a line would start at a separator, the separator goes
/// and the line starts with the next fact. Each cut can only let later lines hold more, so it
/// stops after at most one layout per separator; the galleys come from egui's cache.
fn without_loose_separators(
    ui: &egui::Ui,
    text: String,
    font: &egui::FontId,
    width: f32,
) -> String {
    let mut text = text;
    for _ in 0..text.matches(FACT_SEPARATOR).count() {
        let galley =
            ui.fonts_mut(|fonts| fonts.layout(text.clone(), font.clone(), colors().muted, width));
        let mut start = 0;
        let mut dot = None;
        for (index, row) in galley.rows.iter().enumerate() {
            let lead = row
                .glyphs
                .iter()
                .take_while(|glyph| glyph.chr == ' ')
                .count();
            if index > 0 && row.glyphs.get(lead).is_some_and(|glyph| glyph.chr == '·') {
                dot = Some(start + lead);
                break;
            }
            start += row.char_count_including_newline();
        }
        let Some(byte) = dot.and_then(|at| text.char_indices().nth(at).map(|(byte, _)| byte))
        else {
            break;
        };
        let Some(from) = text[..byte].strip_suffix(' ').map(str::len) else {
            break;
        };
        let to = byte
            + '·'.len_utf8()
            + text[byte + '·'.len_utf8()..]
                .chars()
                .next()
                .map_or(0, char::len_utf8);
        text.replace_range(from..to, "\n");
    }
    text
}

/// V9 (review of PR 299): every `·` on a plan row has the same blank on each side. The font's
/// own spacing around `·` is not even, so the row is laid out once, each `·` gets a section of its
/// own, and it moves by half the difference between its two blanks, in whole pixels
/// (`leading_space`; the text after it moves back by as much, so nothing else on the row moves).
fn even_separators(ui: &egui::Ui, job: egui::text::LayoutJob) -> egui::text::LayoutJob {
    // Each `·` in a section of its own, and the text after it in the next one. `dots` holds
    // their sections in text order, which is the galley's glyph order.
    let mut dots = Vec::new();
    let mut split = egui::text::LayoutJob {
        text: job.text.clone(),
        wrap: job.wrap.clone(),
        ..Default::default()
    };
    for section in &job.sections {
        let mut from = section.byte_range.start;
        for (at, chr) in job.text[section.byte_range.clone()].char_indices() {
            let at = section.byte_range.start + at;
            if chr != '·' {
                continue;
            }
            for range in [from..at, at..at + chr.len_utf8()] {
                if range.start == at {
                    dots.push(split.sections.len());
                }
                if !range.is_empty() {
                    split.sections.push(egui::text::LayoutSection {
                        leading_space: 0.0,
                        byte_range: range,
                        format: section.format.clone(),
                    });
                }
            }
            from = at + chr.len_utf8();
        }
        if from < section.byte_range.end {
            split.sections.push(egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: from..section.byte_range.end,
                format: section.format.clone(),
            });
        }
    }
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(split.clone()));
    let mut dot_sections = dots.into_iter();
    for row in &galley.rows {
        let glyphs = &row.glyphs;
        for (at, dot) in glyphs.iter().enumerate() {
            if dot.chr != '·' {
                continue;
            }
            let Some(section) = dot_sections.next() else {
                break;
            };
            let previous = glyphs[..at].iter().rev().find(|g| !g.chr.is_whitespace());
            let next = glyphs[at + 1..].iter().find(|g| !g.chr.is_whitespace());
            let (Some(previous), Some(next)) = (previous, next) else {
                continue;
            };
            let before = dot.pos.x - (previous.pos.x + previous.advance_width);
            let after = next.pos.x - (dot.pos.x + dot.advance_width);
            // Glyphs sit on whole pixels, so the `·` moves by whole pixels: half the
            // difference, rounded, leaves the two blanks within one pixel.
            let pixel = 1.0 / ui.ctx().pixels_per_point();
            let half = ((after - before) / 2.0 / pixel).round() * pixel;
            if let Some(dot_section) = split.sections.get_mut(section) {
                dot_section.leading_space += half;
            }
            if let Some(following) = split.sections.get_mut(section + 1) {
                following.leading_space -= half;
            }
        }
    }
    split
}

/// FL2: a planned fight's summary for the plan: each fact keeps its words together, and the
/// members (`2 × Cinder Wolf, 1 × Ember Cultist`) wrap between members, never inside one.
fn fight_summary_text(summary: &str) -> String {
    summary
        .split(" · ")
        .map(|part| {
            if part.contains(" × ") {
                part.split(", ")
                    .map(|member| member.replace(' ', "\u{a0}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                part.replace(' ', "\u{a0}")
            }
        })
        .collect::<Vec<_>>()
        .join(FACT_SEPARATOR)
}

/// A plan group's summary. Each separator wraps with the words after it, so no line ends with
/// `·`, and a line that would start at a separator starts with the next fact (FI7). A flagged
/// group keeps every fact (`max_rows` of `usize::MAX`).
fn plan_group_summary(ui: &mut egui::Ui, summary: &str, max_rows: usize) {
    let part = plan_group_part(ui, summary, max_rows, ui.available_width());
    let response = ui.add(egui::Label::new(part.job).wrap());
    design::named(ui, &response, &part.name);
    let _ = plan_hover(ui, response, &part.hover);
}

/// A group's summary laid out in `room`, in at most `max_rows` rows (`plan_group_summary`).
fn plan_group_part(ui: &egui::Ui, summary: &str, max_rows: usize, room: f32) -> RowPart {
    #[cfg(any(test, feature = "test-support"))]
    design::text_measured();
    let font = Type::Small.font(ui);
    let text = without_loose_separators(ui, summary.replace(" · ", FACT_SEPARATOR), &font, room);
    let mut job = egui::text::LayoutJob::simple(text, font, colors().muted, room);
    job.wrap.max_rows = max_rows;
    // The no-break spaces only steer the wrap; the name keeps plain spaces.
    let plain = summary.replace('\u{a0}', " ");
    RowPart {
        job,
        link: false,
        name: plain.clone(),
        hover: PlanHover::Text(plain),
    }
}

/// A body link whose word starts at the cursor, as the text above it does, and whose underline
/// covers only the word. The target is the link's own minimum size: at least `XL` square. A
/// word narrower than that gets the extra width evenly on both sides, inside the gaps, so the
/// word stays at the text edge and nothing beside it moves (the `line_start` pattern).
/// Round 27 FH4: the gap on each side of a `·` between the opened row's links: one blank of
/// the links' type, as `Party won · Ended · Round 1` has in one text.
fn link_gap(ui: &egui::Ui) -> f32 {
    ui.painter()
        .layout_no_wrap(" ".to_owned(), Type::Body.font(ui), colors().text)
        .size()
        .x
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

/// One plan row: an icon, a compact text column, and actions centred beside both lines.
fn plan_row(
    ui: &mut egui::Ui,
    (widths, columns): (&[f32], design::PlanColumns),
    icon: &str,
    words: impl FnMut(&mut egui::Ui),
    summary: impl FnMut(&mut egui::Ui),
    action: impl FnMut(&mut egui::Ui),
) {
    plan_row_with(
        ui,
        (widths, columns),
        |ui| {
            ui.label(Type::Body.rich(ui, icon).color(colors().muted));
        },
        words,
        summary,
        action,
    );
}

/// A plan row whose start is `lead`: a fight's caret, or a scene's box. Every row has the one
/// leading slot, so every row's words start at one column. Its actions take the plan's shared
/// column (`columns`: the closed rows laid by their recipe, measured before paint) with what the
/// rows drawn here measured in their last pass (layout 11c A2c: the scenes and a group with
/// warnings, until scope B).
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn plan_row_with(
    ui: &mut egui::Ui,
    (widths, columns): (&[f32], design::PlanColumns),
    lead: impl FnOnce(&mut egui::Ui),
    mut words: impl FnMut(&mut egui::Ui),
    mut summary: impl FnMut(&mut egui::Ui),
    mut action: impl FnMut(&mut egui::Ui),
) {
    // Review of PR 312: the row registers its whole rect, so a footer never cuts through it (N12,
    // R4). The gate's far-action lines this adds at 1920 px wait for ruling R14.
    let row = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = design::S;
        // Round 27 FL4: the lead is centred in its `XL` slot, which is where a title's first
        // line is centred: a fight's link in its `XL` target, a scene's line in its `XL` band
        // (`scene_words`). A caret at the slot's top sat 2 px above its title's centre.
        let (_, slot) = ui.allocate_space(egui::vec2(lead_width(), design::XL));
        lead(
            &mut ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(slot)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            ),
        );
        ui.vertical(|ui| {
            let left = ui.cursor().min;
            let title_width = (ui.available_width() - widths[1] - design::S).max(design::XL);
            let title = ui.allocate_ui_with_layout(
                egui::vec2(title_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_max_width(title_width);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    words(ui);
                },
            );
            // Layout PR 5a-2 (I6), L5c: each row's actions start `L` after the plan's widest facts,
            // in its own recipe's width (with its alternates); the spare width lies after them.
            // The column fits beside the plan's widest recipe. Each title keeps the room and the
            // lines it had, so where the column does not fit the actions keep the row's far edge
            // (`MeasuredColumn`).
            let words_width = title.response.rect.width();
            let slot =
                design::action_rows::MeasuredColumn::start(PLAN_COLUMNS, widths[1], words_width)
                    .over((columns.facts, columns.actions))
                    .slot(
                        ui,
                        egui::Rect::from_min_max(
                            left,
                            egui::pos2(ui.max_rect().right(), left.y + Density::Read.control()),
                        ),
                        left.x,
                    );
            let action_rect = slot.rect();
            let mut action_ui = slot.child(ui);
            action(&mut action_ui);
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.advance_cursor_after_rect(
                title
                    .response
                    .rect
                    .union(action_rect)
                    .union(action_ui.min_rect()),
            );
            ui.spacing_mut().item_spacing.y = 0.0;
            let summary_rect = ui
                .allocate_ui_with_layout(
                    egui::vec2(
                        (ui.available_width() - Density::Read.control() - design::S)
                            .max(design::XL),
                        0.0,
                    ),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        summary(ui);
                    },
                )
                .response
                .rect;
            ui.expand_to_include_rect(action_rect.union(action_ui.min_rect()));
            // The row's facts are every line it drew: its title and its summary (PR 605 review,
            // P1: a short title over a long summary set the column inside the summary).
            let facts_width = words_width.max(summary_rect.right() - left.x);
            slot.done(
                ui,
                "plan",
                0,
                egui::Rect::from_min_max(
                    left,
                    egui::pos2(
                        left.x + facts_width,
                        summary_rect.bottom().max(title.response.rect.bottom()),
                    ),
                ),
                title.response.rect,
                action_ui.min_rect(),
            );
        });
    });
    design::register_row(ui, row.response.rect);
}

/// The list the plan's rows drawn by `plan_row_with` measure their column in.
const PLAN_COLUMNS: &str = "plan_row_columns";

/// A closed plan row's words, laid out before any row draws (layout 11c A2c): its lead, its
/// title, its summary as one laid part, and its action place. The plan's shared column comes
/// from every such row's facts before the first row paints.
struct RowWords {
    lead: &'static str,
    title: String,
    summary: Option<RowPart>,
    /// The action place's width: the row's widest action recipe.
    actions: f32,
}

/// A closed row's summary, laid out in the summary's room: its words, its accessible name and
/// what hover shows.
struct RowPart {
    job: egui::text::LayoutJob,
    link: bool,
    name: String,
    hover: PlanHover,
}

impl RowPart {
    /// The open fight's state as a link, its run icon before the words (`plan_run_link`).
    fn run_link(ui: &egui::Ui, word: &str) -> Self {
        Self {
            job: glyph_link_job(ui, icons::RUN, colors().accent, Type::Small, word),
            link: true,
            name: word.to_owned(),
            hover: PlanHover::None,
        }
    }

    /// A paused run's state (`plan_saved_status`), on one line, whole on hover.
    fn saved(ui: &egui::Ui, saved: &crate::encounter_run::SavedRunInfo) -> Self {
        let word = plan_saved_status(saved);
        let mut job = design::status_job(
            ui,
            Status {
                icon: icons::RUN,
                word: &word,
                color: colors().muted,
            },
        );
        job.wrap.max_rows = 1;
        job.wrap.break_anywhere = true;
        Self {
            name: job.text.clone(),
            job,
            link: false,
            hover: PlanHover::Text(word),
        }
    }

    /// Words that wrap whole, named by their text.
    fn words(job: egui::text::LayoutJob, hover: PlanHover) -> Self {
        Self {
            name: job.text.clone(),
            job,
            link: false,
            hover,
        }
    }
}

impl RowWords {
    fn part(&self) -> Option<design::SummaryPart<'_>> {
        self.summary.as_ref().map(|part| design::SummaryPart::Job {
            job: &part.job,
            max_rows: part.job.wrap.max_rows,
            link: part.link,
        })
    }

    fn row<'a>(
        &'a self,
        key: egui::Id,
        summary: &'a [design::SummaryPart<'a>],
        (rule, tail): (bool, f32),
    ) -> design::PlanRow<'a> {
        design::PlanRow {
            key,
            lead: self.lead,
            title: &self.title,
            summary,
            actions: self.actions,
            actions_height: Density::Read.control(),
            rule,
            tail,
        }
    }

    /// The row's facts and action recipe at `width`, for the plan's shared column.
    fn facts(&self, ui: &egui::Ui, width: f32) -> (f32, f32) {
        let part = self.part();
        self.row(egui::Id::NULL, part.as_slice(), (true, 0.0))
            .facts(ui, width)
    }
}

/// A closed plan row as drawn: its title link and its summary part. The fold counts
/// the row as `plan_row_with` and `plan_rule` measure it: `row`, the row with the column's gap
/// after it; `rule`, the rule its block holds with the gap the column leaves after the block
/// (zero for an opened row, whose rule follows its details).
struct DrawnRow {
    title: egui::Response,
    summary: Option<egui::Response>,
    row: f32,
    rule: f32,
}

/// What the plan's closed rows' actions did in this pass, applied after the rows draw (layout
/// 11c A2c-3: the rows' actions are the same in place and in the plan's region).
struct RowActs {
    insert: Option<String>,
    remove: Option<String>,
    remove_group: Option<(usize, String, f32)>,
    remove_fight: Option<(usize, String)>,
    delete_run: Option<crate::encounter_run::SavedRunInfo>,
    travel_act: Option<(String, checks::Act)>,
    rewards_act: Option<RewardsAct>,
    /// The first Resume or Start fight is the primary action, while no run is open.
    primary_available: bool,
    /// The first planned fight's action, which the opened played row keeps in reach.
    next_action: Option<egui::Rect>,
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

/// A closed row laid out by its recipe, with what the gate's record of it needs.
struct KeptRow {
    words: RowWords,
    laid: design::Laid,
    /// Its block's key: its event, fight or group.
    key: egui::Id,
    /// Its facts' width and whether its actions stand in the shared column: the gate's record
    /// of the row (test builds).
    #[cfg(any(test, feature = "test-support"))]
    facts: f32,
    #[cfg(any(test, feature = "test-support"))]
    in_column: bool,
}

/// A planned group's row: laid by its recipe, or (with warnings) drawn by `plan_row_with`; and
/// its action place.
struct GroupRow {
    row: Option<KeptRow>,
    actions: f32,
    edit_word: &'static str,
    joins_fight: bool,
}

/// The plan's closed rows, laid out before any row draws, with the plan's shared column.
pub(super) struct KeptRows {
    /// Each event row, with its place among the Timeline's events.
    events: Vec<(KeptRow, RewardsAt, usize)>,
    /// Each planned fight's row with its occurrence: how many lines before it plan the same
    /// fight. A fight's row and its controls are keyed by its file and occurrence, never by its
    /// place, so a fight planned above another keeps the other's ids (review of PR 841).
    fights: Vec<(KeptRow, usize)>,
    groups: Vec<GroupRow>,
    columns: design::PlanColumns,
    /// Every laid row's block, in order: the plan's measured region draws them (A2c-3).
    blocks: design::memo::Revised<Vec<design::Block>>,
    /// The fold's sums in the region, as in place: the closed rows' height, and the first
    /// planned fight's row's bottom with the rows above it. Worked out with the rows, so a quiet
    /// frame walks no block (review of PR 869).
    fold: (f32, Option<f32>),
}

/// What the plan's laid rows are made from (layout 11c A2c, review of PR 841): revisions, ids and
/// the measuring environment. A quiet frame compares these and lays out nothing.
#[derive(Clone, PartialEq)]
struct RowsKey {
    /// The projection the rows read (the planned fights and groups at their places): its
    /// revision follows each of its inputs, the content index's too.
    projection: Option<design::memo::Revision>,
    session: Option<Uuid>,
    draft: u64,
    /// The Timeline as it stands at the rows (an Add frame's new event is in it).
    book: design::memo::Revision,
    fights: design::memo::Revision,
    saved: design::memo::Revision,
    days: design::memo::Revision,
    open_event: Option<String>,
    open_planned: Option<String>,
    /// The open run and its state's words (the run link's).
    open_run: Option<(Uuid, String)>,
    arrivals_open: bool,
    refused: Option<(Option<Uuid>, Uuid)>,
    /// The rows' width, the font generation, the scale and the font atlas's epoch (a kept galley
    /// holds places in the atlas).
    env: (f32, u64, f32, u64),
    /// The column the rows drawn by `plan_row_with` measured (scenes, a group with warnings):
    /// the shared column takes it, so a wider scene lays the rows out again (review of PR 841).
    legacy: (f32, f32),
    /// The body is the plan's measured region (layout 11c A2c-3): each block holds the gap the
    /// column leaves after a row in place.
    measured: bool,
}

/// The plan's laid rows, kept by their inputs (`RowsKey`).
#[derive(Default)]
pub(super) struct PlanRowsCache {
    kept: Option<(RowsKey, Arc<KeptRows>)>,
    /// How often a frame asked for the rows, and how often they were laid out (the quiet-frame
    /// proof counts both).
    pub(super) reached: usize,
    pub(super) builds: usize,
}

/// Draw a closed plan row by its recipe at the rows' cursor (layout 11c A2c), in place: its
/// block, then its parts (`draw_row_parts`). Its block holds the rule after it when its recipe
/// lays one (a closed row; an opened row's rule follows its details).
fn draw_plan_row(
    ui: &mut egui::Ui,
    row: &KeptRow,
    open: Option<bool>,
    actions: impl FnOnce(&mut egui::Ui),
) -> DrawnRow {
    let mut drawn = None;
    let _ = design::draw_laid(ui, &row.laid, |l| {
        drawn = Some(draw_row_parts(l, row, open, actions));
    });
    drawn.expect("a laid row draws its parts")
}

/// A laid plan row's parts in its block, in place or in the plan's region (layout 11c A2c): its
/// title link (`open`: whether its details show, for a screen reader), its summary part with its
/// hover, `actions` in its action place, and its rule. In test builds it records the row for the
/// gate's plan check, as `plan_row_with` does.
fn draw_row_parts(
    l: &mut design::LaidUi<'_>,
    row: &KeptRow,
    open: Option<bool>,
    actions: impl FnOnce(&mut egui::Ui),
) -> DrawnRow {
    let words = &row.words;
    let rule_slot = l.slot(design::plan_slot::RULE);
    let rule = rule_slot.height() > 0.0;
    let title = l.link(0, &words.title);
    if let Some(open) = open {
        title
            .ctx
            .accesskit_node_builder(title.id, |node| node.set_expanded(open));
    }
    let summary = words.summary.as_ref().map(|part| {
        let response = l.part(design::plan_slot::FIRST_PART, &part.name, part.link);
        l.in_slot(design::plan_slot::FIRST_PART, |ui| {
            plan_hover(ui, response, &part.hover)
        })
    });
    let slot = l.slot(design::plan_slot::ACTIONS);
    let shown = l.actions(design::plan_slot::ACTIONS, |ui| {
        actions(ui);
        ui.min_rect()
    });
    if rule {
        design::plan_rule(l);
    }
    // The fold counts the row as `plan_row_with` and `plan_rule` measure it: the row with the
    // column's gap after it, then the rule with the gap after it.
    let top = l.slot(design::plan_slot::LEAD).top();
    let (height, rule) = if rule {
        let height = rule_slot.top() - design::S - top;
        (
            height,
            rule_slot.bottom() - top - height + design::theme::LINE_GAP,
        )
    } else {
        (rule_slot.top() - top, 0.0)
    };
    #[cfg(any(test, feature = "test-support"))]
    {
        let block = egui::Rect::from_min_size(l.slot(design::plan_slot::LEAD).min, row.laid.size());
        let left = block.left() + lead_width() + design::S;
        let record = design::action_rows::RowRecord {
            owner: "plan",
            row: 0,
            facts: egui::Rect::from_min_max(
                egui::pos2(left, block.top()),
                egui::pos2(left + row.facts, top + height - design::theme::LINE_GAP),
            ),
            line: title.rect,
            action: shown,
            column: slot.left(),
            edge: block.right(),
            wrapped: !row.in_column,
        };
        l.in_slot(design::plan_slot::LEAD, |ui| {
            design::action_rows::keep(ui, record);
        });
    }
    #[cfg(not(any(test, feature = "test-support")))]
    let _ = (slot, shown);
    DrawnRow {
        title,
        summary,
        row: height,
        rule,
    }
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

/// A paused run needs its state here; the whole saved-run record stays in Encounters.
fn plan_saved_status(saved: &crate::encounter_run::SavedRunInfo) -> String {
    crate::encounters::run_state_word(saved, true)
}

/// The words of a link with a glyph of its own before them (`glyph_link`), as one job.
fn glyph_link_job(
    ui: &egui::Ui,
    glyph: &str,
    glyph_color: egui::Color32,
    role: Type,
    word: &str,
) -> egui::text::LayoutJob {
    let c = colors();
    let plain = egui::TextFormat {
        font_id: role.font(ui),
        color: c.accent,
        ..Default::default()
    };
    let mut job = egui::text::LayoutJob::default();
    job.append(
        glyph,
        0.0,
        egui::TextFormat {
            color: glyph_color,
            ..plain.clone()
        },
    );
    job.append(" ", 0.0, plain.clone());
    job.append(
        word,
        0.0,
        egui::TextFormat {
            underline: egui::Stroke::new(1.0, c.accent.gamma_multiply(0.6)),
            ..plain
        },
    );
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    job
}

/// Link words with a glyph of their own before them, in `glyph_color`. The underline covers only
/// the words; the press target covers both, at least 24 px, and the accessible name is the words.
fn glyph_link(
    ui: &mut egui::Ui,
    glyph: &str,
    glyph_color: egui::Color32,
    role: Type,
    word: &str,
) -> egui::Response {
    let mut job = glyph_link_job(ui, glyph, glyph_color, role, word);
    job.wrap.max_width = ui.available_width().max(1.0);
    let c = colors();
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            galley.size().x.max(design::XL),
            galley.size().y.max(design::XL),
        ),
        egui::Sense::click(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, ui.is_enabled(), word));
    if ui.is_rect_visible(rect) {
        let top = rect.center().y - galley.size().y / 2.0;
        ui.painter()
            .galley(egui::pos2(rect.left(), top), galley, c.accent);
        if response.has_focus() {
            design::focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn plan_details(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal_top(|ui| {
        ui.add_space(Type::Body.size() + design::S);
        ui.vertical(add);
    });
}

/// A planned fight's summary, ended by its difficulty in the one form (F1, FB6):
/// `5 party · 2 × Ember Cultist · XP 400 · Low (up to 2,800)`, or `XP not known` (FB4). The
/// form never gives way (FB2): where it would add a line, the summary gives way from its end,
/// by whole words, with `…`; hover and the accessible name keep the whole summary. FL2, FI7:
/// each fact keeps its words together, the members wrap between members and never inside one,
/// and no line starts or ends with `·`. The plan's rows draw it through `planned_summary_on_road`.
#[cfg(test)]
fn planned_summary(
    ui: &mut egui::Ui,
    summary: &crate::difficulty::Summary,
    difficulty: Option<&crate::difficulty::Difficulty>,
) {
    planned_summary_on_road(ui, summary, None, difficulty);
}

/// `planned_summary` with a journey's day after the summary (`road_day`). Round 28 FE6, Q-s,
/// R39: the day ties the fight to its journey, so it gives way last: the party's size first,
/// then the word's limit, then the XP number, then the roster's members from the end, then the
/// day.
/// A planned fight's line, laid out once for its room and kept (`Kept`): what an unchanged
/// frame paints, with no fitting and no string building.
#[derive(Clone)]
pub(super) struct KeptPlanLine {
    text: PlanText,
    name: String,
    hover: PlanHover,
}

#[derive(Clone)]
enum PlanText {
    /// No form: the summary alone.
    Alone(String),
    Form(egui::text::LayoutJob),
}

#[derive(Clone)]
enum PlanHover {
    None,
    Text(String),
    /// The word's budget, after the whole summary where it was cut.
    Rating(crate::difficulty::Rating, Option<String>),
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Test builds: how many plan lines were fitted (laid out), so a test can see that an
    /// unchanged frame fits none.
    static PLAN_FITS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The plan's kept lines: one per planned fight, by the session, the fight's place in the plan
/// and its note. The Sessions page owns them (`Sessions::plan_lines`).
pub(super) type PlanLines = crate::difficulty::Kept<
    PlanLineId,
    KeptPlanLine,
    (design::memo::Revision, design::memo::Revision),
>;
/// A planned fight's line: its session, its place in the plan and its encounter.
pub(super) type PlanLineId = (Option<Uuid>, usize, Uuid);

/// `planned_summary` with a journey's day after the summary (`road_day`), not kept (tests).
#[cfg(test)]
fn planned_summary_on_road(
    ui: &mut egui::Ui,
    typed: &crate::difficulty::Summary,
    day: Option<&str>,
    difficulty: Option<&crate::difficulty::Difficulty>,
) {
    let line = prepare_plan_line(ui, (typed, ui.available_width()), day, difficulty);
    show_plan_line(ui, &line);
}

/// A kept plan line: fitted again only when its fight's roster, the plan's days (`days`: the
/// revision of the planned fights with their days), its room, its font, the font generation or
/// the display scale changes.
fn kept_plan_line(
    ui: &egui::Ui,
    (kept, room): (&mut PlanLines, f32),
    id: PlanLineId,
    fight: &PlanFight,
    day: Option<&str>,
    days: design::memo::Revision,
) -> KeptPlanLine {
    let font = Type::Small.font(ui);
    let content = (fight.revision.unwrap_or(days), days);
    // Fitted in its room: the plan row's summary room (layout 11c A2c), not the row's width.
    kept.answer(ui, &id, content, room, &font, || {
        let words = fight.summary.text();
        let day = road_day_in(ui, (&words, room), day);
        prepare_plan_line(
            ui,
            (&fight.summary, room),
            day.as_deref(),
            Some(&fight.difficulty),
        )
    })
}

impl KeptPlanLine {
    /// The line as a plan row's summary part, laid out in `room` (layout 11c A2c).
    fn part(self, ui: &egui::Ui, room: f32) -> RowPart {
        let job = match self.text {
            PlanText::Alone(alone) => {
                egui::text::LayoutJob::simple(alone, Type::Small.font(ui), colors().muted, room)
            }
            PlanText::Form(job) => job,
        };
        RowPart {
            job,
            link: false,
            name: self.name,
            hover: self.hover,
        }
    }
}

#[cfg(test)]
fn show_plan_line(ui: &mut egui::Ui, line: &KeptPlanLine) {
    let response = match &line.text {
        PlanText::Alone(alone) => ui.add(
            egui::Label::new(Type::Small.rich(ui, alone.clone()).color(colors().muted)).wrap(),
        ),
        PlanText::Form(job) => ui.add(egui::Label::new(job.clone()).wrap()),
    };
    design::named(ui, &response, &line.name);
    plan_hover(ui, response, &line.hover);
}

/// A plan row summary's hover: its whole words, or a fight's budget line (also its description).
fn plan_hover(ui: &egui::Ui, response: egui::Response, hover: &PlanHover) -> egui::Response {
    match hover {
        PlanHover::None => response,
        PlanHover::Text(hover) => response.on_hover_text(hover),
        PlanHover::Rating(rating, cut) => {
            let response = match cut {
                Some(summary) => response.on_hover_text(summary),
                None => response,
            };
            crate::difficulty::describe(ui, response, rating)
        }
    }
}

fn prepare_plan_line(
    ui: &egui::Ui,
    (typed, width): (&crate::difficulty::Summary, f32),
    day: Option<&str>,
    difficulty: Option<&crate::difficulty::Difficulty>,
) -> KeptPlanLine {
    #[cfg(any(test, feature = "test-support"))]
    PLAN_FITS.with(|fits| fits.set(fits.get() + 1));
    #[cfg(any(test, feature = "test-support"))]
    design::text_measured();
    // G118: the roster's typed parts; its words only for the layout and the name.
    let words = typed.text();
    let roster = words.as_str();
    let with_day = match day {
        Some(day) if !roster.trim().is_empty() => format!("{roster} · {day}"),
        Some(day) => day.to_owned(),
        None => roster.to_owned(),
    };
    let summary = with_day.as_str();
    let c = colors();
    let font = Type::Small.font(ui);
    let laid = |text: String| without_loose_separators(ui, text, &font, width);
    let rows = |text: &str| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout(text.to_owned(), font.clone(), c.muted, width)
                .rows
                .len()
        })
    };
    let alone = laid(fight_summary_text(summary));
    let Some(parts) = difficulty.and_then(crate::difficulty::Difficulty::parts) else {
        // The breaks only steer the wrap; the name keeps the summary's words.
        return KeptPlanLine {
            text: PlanText::Alone(alone),
            name: summary.to_owned(),
            hover: PlanHover::None,
        };
    };
    // The form keeps its words together and wraps whole. Where the row has no room for it,
    // the word's limit gives way first, then the summary from its end.
    let nb = |parts: Vec<(String, bool)>| -> Vec<(String, bool)> {
        parts
            .into_iter()
            .map(|(words, word)| (words.replace(' ', "\u{a0}"), word))
            .collect()
    };
    let joined = |parts: &[(String, bool)]| -> String {
        parts.iter().map(|(words, _)| words.as_str()).collect()
    };
    let with_form = |facts: &str, form: &str| {
        laid(format!(
            "{}{FACT_SEPARATOR}{form}",
            fight_summary_text(facts)
        ))
    };
    // The day never adds a line (review of PR 284): the room is the roster's own lines, and
    // the day gives way last within it (round 28 FE6).
    let lines = match day {
        Some(_) if !roster.trim().is_empty() => rows(&laid(fight_summary_text(roster))),
        _ => rows(&alone),
    };
    let missing = difficulty
        .filter(|difficulty| difficulty.rating.is_none())
        .and_then(|difficulty| difficulty.missing.as_ref());
    // The summary's facts and members, each with the separator before it and its turn to give
    // way: the party's size first (`5 party`, the first fact), then the roster's members from
    // the end, then the journey's day. A cut member leaves `…` in its place:
    // `2 × Ember Cultist, … · Day 2 of Travel`; the party's size goes without one.
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Turn {
        Party,
        Member,
        Day,
    }
    // G118: the units from the typed parts, never from the words. A member unit keeps its
    // member, which R31 asks whether its XP is known.
    type Units<'u> = Vec<(&'u str, String, Turn, Option<&'u crate::difficulty::Member>)>;
    let mut units: Units<'_> = Vec::new();
    let first = |units: &Units<'_>| units.is_empty();
    if typed.party > 0 {
        units.push((
            "",
            design::count(typed.party, "party", "party"),
            Turn::Party,
            None,
        ));
    }
    for (at, member) in typed.members.iter().enumerate() {
        let before = match (first(&units), at) {
            (true, _) => "",
            (false, 0) => " · ",
            _ => ", ",
        };
        units.push((before, member.words(), Turn::Member, Some(member)));
    }
    for fact in &typed.facts {
        let before = if first(&units) { "" } else { " · " };
        units.push((before, fact.words().to_owned(), Turn::Member, None));
    }
    if let Some(day) = day {
        let before = if units.is_empty() { "" } else { " · " };
        units.push((before, day.to_owned(), Turn::Day, None));
    }
    // The units in the order they give way.
    let mut giving_way: Vec<usize> = (0..units.len()).collect();
    giving_way.sort_by_key(|&at| (units[at].2, std::cmp::Reverse(at)));
    // The summary with only `count` units kept: the last `count` to give way.
    let cut_at = |count: usize| {
        let gone = &giving_way[..units.len() - count];
        let mut pieces: Vec<(&str, &str)> = Vec::new();
        let mut cut_before = false;
        for (at, (before, unit, turn, _)) in units.iter().enumerate() {
            if !gone.contains(&at) {
                pieces.push((before, unit.as_str()));
                cut_before = false;
            } else if *turn != Turn::Party && !cut_before {
                pieces.push((before, "…"));
                cut_before = true;
            }
        }
        // A cut party leaves the next piece first on the line, without its separator.
        let mut text = String::new();
        for (index, (before, unit)) in pieces.iter().enumerate() {
            if index > 0 {
                text.push_str(before);
            }
            text.push_str(unit);
        }
        text
    };
    let (shown, parts, cut) = match missing {
        // W1-D2, X1, R31: whose input is not known, in the `XP not known` place. A candidate is
        // a set of units and the reason for EXACTLY those units (`Missing::unsaid`), and only
        // whole candidates are compared: most units first, the names whole, else the first
        // name shortened from its end. Where even its first letter has no room, the summary
        // gives way first; the reason alone only where nothing else fits. Each step shows
        // fewer units, so the search ends.
        Some(missing) => {
            let short = missing.short();
            let unsaid = |count: usize| {
                let gone = &giving_way[..units.len() - count];
                missing.unsaid(
                    units
                        .iter()
                        .enumerate()
                        .filter_map(|(at, unit)| Some((unit.3?, !gone.contains(&at)))),
                )
            };
            let candidate = |count: usize, words: &str| {
                let form = words.replace(' ', "\u{a0}");
                if count == units.len() {
                    with_form(summary, &form)
                } else if count == 0 {
                    laid(form)
                } else {
                    with_form(&cut_at(count), &form)
                }
            };
            let fits = |count: usize, words: &str| rows(&candidate(count, words)) <= lines;
            let chosen = (1..=units.len()).rev().find_map(|count| {
                let reason = unsaid(count);
                let words = if reason.names.is_empty() {
                    short.to_owned()
                } else {
                    reason
                        .fitted(|words| fits(count, words))
                        .filter(|words| words != short)?
                };
                fits(count, &words).then_some((count, words))
            });
            let (count, words) = chosen.unwrap_or_else(|| {
                // No fact or member keeps its place: the named reason alone, in the room the
                // summary leaves (review of PR 299).
                let words = unsaid(0)
                    .fitted(|words| fits(0, words))
                    .unwrap_or_else(|| short.to_owned());
                (0, words)
            });
            let form = nb(vec![(words.clone(), false)]);
            (candidate(count, &words), form, count < units.len())
        }
        None => {
            // Q-s, R39: a known word. `5 party` gives way first (it is the same on every row),
            // then the word's limit, then the XP number, then the members from their end, and
            // the journey's day last; the word never gives way. One closed order of whole
            // candidates (the units kept and the form), the first that keeps the lines
            // (`design::fitting::first_fit`).
            #[derive(Clone, Copy)]
            enum Form {
                /// `XP 1,050 · Moderate (up to 1,125)`
                Full,
                /// `XP 1,050 · Moderate`
                NoLimit,
                /// `Moderate`
                Word,
            }
            let rating = difficulty
                .and_then(|difficulty| difficulty.rating.as_ref())
                .expect("a known word has its rating");
            let form_parts = |form| match form {
                Form::Full => nb(parts.clone()),
                Form::NoLimit => nb(rating.parts(false)),
                Form::Word => nb(rating
                    .parts(false)
                    .into_iter()
                    .filter(|(_, word)| *word)
                    .collect()),
            };
            let all = units.len();
            let party = usize::from(typed.party > 0);
            // Without a party's size there is no candidate that drops it.
            let order = [(all, Form::Full)]
                .into_iter()
                .chain((party == 1).then_some((all - 1, Form::Full)))
                .chain([(all - party, Form::NoLimit)])
                .chain((0..=all - party).rev().map(|count| (count, Form::Word)));
            let shown_with = |count: usize, form: &str| {
                if count == all {
                    with_form(summary, form)
                } else if count == 0 {
                    form.to_owned()
                } else {
                    with_form(&cut_at(count), form)
                }
            };
            let chosen = design::fitting::first_fit(
                order,
                |_| true,
                |(count, form)| {
                    let parts = form_parts(form);
                    let shown = shown_with(count, &joined(&parts));
                    (rows(&shown) <= lines).then_some((shown, parts))
                },
            );
            let ((count, _), (shown, parts)) = chosen.unwrap_or_else(|| {
                // Not even the word alone keeps the lines: the word alone, wrapped.
                let parts = form_parts(Form::Word);
                ((0, Form::Word), (joined(&parts), parts))
            });
            (shown, parts, count < all)
        }
    };
    let form = joined(&parts);
    let facts = shown.strip_suffix(form.as_str()).unwrap_or("");
    let mut job = egui::text::LayoutJob::default();
    crate::difficulty::append(&mut job, ui, Type::Small, facts, c.muted);
    crate::difficulty::append_form(&mut job, ui, Type::Small, &parts);
    job.wrap.max_width = width;
    let job = even_separators(ui, job);
    // The accessible name keeps the whole summary and the whole form.
    let plain_form = missing.map_or_else(
        || {
            difficulty
                .and_then(crate::difficulty::Difficulty::form)
                .unwrap_or_default()
        },
        |missing| {
            missing
                .words()
                .unwrap_or_else(|| missing.short().to_owned())
        },
    );
    let name = format!("{summary} · {plain_form}");
    let difficulty = difficulty.expect("the form came from it");
    let hover = match (&difficulty.rating, &difficulty.missing) {
        (Some(rating), _) => PlanHover::Rating(rating.clone(), cut.then(|| summary.to_owned())),
        // W1-D2: whose input is not known, on hover.
        (None, Some(missing)) => {
            let names = missing.words().unwrap_or_default();
            let hover = match (cut, names.is_empty()) {
                (true, true) => summary.to_owned(),
                (true, false) => format!("{summary}\n{names}"),
                (false, _) => names,
            };
            if hover.is_empty() {
                PlanHover::None
            } else {
                PlanHover::Text(hover)
            }
        }
        (None, None) => PlanHover::None,
    };
    KeptPlanLine {
        text: PlanText::Form(job),
        name,
        hover,
    }
}

/// The opened planned fight's link to its encounter. FH6: the row above it already says the
/// difficulty, so this line does not say it again.
fn read_encounter(ui: &mut egui::Ui) -> egui::Response {
    design::link(ui, "Read encounter")
}

/// A planned fight's menu line that opens its encounter in the editor, with the way back to this
/// session (Sol's W1-D1 after app round 2). An unsaved encounter, group or creature in the editor
/// blocks it, so the editor never loses typed words.
fn edit_encounter_line(ui: &mut egui::Ui, plan: &Plan<'_>) -> bool {
    Action::quiet("Edit encounter")
        .blocked(plan.group_blocked)
        .show(ui)
        .clicked()
}

/// One scroll, on the frame the DM opened a played fight's row (review round 1): the row with its
/// fields and the next fight's primary action both in view when they fit together, with as little
/// scroll as that needs; else that action in view at the pane's bottom, under as much of the row
/// as fits. With no fight to come, the row at the top, as for any other row.
fn reveal_opened_played(
    ui: &egui::Ui,
    opened: Option<egui::Rect>,
    next_action: Option<egui::Rect>,
) {
    let Some(opened) = opened else {
        return;
    };
    match next_action.filter(|action| action.top() > opened.top()) {
        Some(action) if opened.union(action).height() <= ui.clip_rect().height() => {
            ui.scroll_to_rect(opened.union(action), None);
        }
        Some(action) => ui.scroll_to_rect(action, Some(egui::Align::BOTTOM)),
        None => ui.scroll_to_rect(opened, Some(egui::Align::TOP)),
    }
}

/// The rule below a row. Returns its height, which the scenes' fold counts with the row's.
fn plan_rule(ui: &mut egui::Ui) -> f32 {
    let top = ui.cursor().top();
    design::gap(ui, design::Gap::S);
    design::rule(ui);
    ui.cursor().top() - top
}

/// The words of one of the plan's add menus.
struct PlanMenu<'a> {
    word: &'a str,
    icon: &'a str,
    name: &'a str,
    new: &'a str,
    blocked: Option<&'a str>,
    find: &'a str,
    field: design::FieldId,
    hint: &'a str,
}

enum MenuChoice {
    New,
    Named(String),
}

/// A button whose menu offers a new thing, then the campaign's, with a search when they are many.
fn plan_menu(
    ui: &mut egui::Ui,
    menu: &PlanMenu<'_>,
    names: &[&str],
    search: &mut String,
    focus: &mut bool,
) -> Option<MenuChoice> {
    let response = Action::quiet(menu.word)
        .icon(menu.icon)
        .name(menu.name)
        .density(Density::Read)
        .show(ui);
    if response.clicked() {
        *focus = true;
    }
    let mut choice = None;
    design::menu_popup(ui, &response)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            // A fixed place, whatever the list's length.
            if Action::quiet(menu.new)
                .icon(icons::ADD)
                .density(Density::Read)
                .blocked(menu.blocked)
                .show(ui)
                .clicked()
            {
                choice = Some(MenuChoice::New);
                ui.close();
            }
            // A note under its choice, at its words after the icon (M-P0.27).
            design::choice_note(ui, true, menu.blocked);
            if names.len() > FIGHT_SEARCH_FROM {
                let field = menu
                    .field
                    .search(ui, menu.find, search, menu.hint, FIGHT_MENU_WIDTH)
                    .field;
                if std::mem::take(focus) {
                    field.request_focus();
                }
            }
            design::rule(ui);
            let query = search.trim().to_lowercase();
            let mut chosen = None;
            design::ScrollColumn::new(menu.word)
                .max_height(FIGHT_MENU_HEIGHT)
                .fit_height()
                .show(ui, |ui| {
                    for name in names {
                        if name.to_lowercase().contains(&query) {
                            // Round 26 FL4: one left edge. Each name is a quiet action with the
                            // new action's padding and an empty icon slot, so the names start
                            // at the x of the word after `+`.
                            let row = Action::quiet(name)
                                .plain()
                                .icon("")
                                .name(&format!("Plan {name}"))
                                .density(Density::Read)
                                .show(ui);
                            if row.clicked() {
                                chosen = Some((*name).to_owned());
                            }
                        }
                    }
                });
            if let Some(name) = chosen {
                choice = Some(MenuChoice::Named(name));
                search.clear();
                ui.close();
            }
        });
    choice
}

/// The fields of one event, in place in a session's plan. Returns whether any changed. Each is
/// keyed by its event (`TextKey::of`), so a cursor or text Undo never crosses to another row's
/// event (review of PR 312).
fn event_fields(ui: &mut egui::Ui, event: &mut timeline::Event) -> bool {
    let key = design::TextKey::of(&*event);
    let title = design::labelled_field(
        ui,
        EVENT_TITLE.of(key.clone()),
        "Title",
        &mut event.title,
        None,
        None,
    );
    let mut changed = title.changed();
    design::field(ui, "Notes", None, None, |ui| {
        let width = ui.available_width() - 2.0 * design::S;
        let response = EVENT_NOTES
            .of(key)
            .multi_line(ui, &mut event.notes, |edit| {
                edit.desired_width(width)
                    .desired_rows(2)
                    .margin(egui::vec2(design::S, design::S))
            });
        design::named(ui, &response, "Notes on this event");
        changed |= response.changed();
    });
    changed
}

/// A played fight's `Where`, in place under its opened row's links (W2-D3): the Timeline detail's
/// own field on the same event, so the typed place has one owner, the Timeline, and saves as
/// any edit of an event does. Returns whether it changed.
fn place_field(ui: &mut egui::Ui, event: &mut timeline::Event) -> bool {
    design::gap(ui, design::Gap::XS);
    let key = design::TextKey::of(&*event);
    design::labelled_field(
        ui,
        EVENT_PLACE.of(key),
        "Where",
        &mut event.place,
        None,
        None,
    )
    .changed()
}

const EVENT_TITLE: design::FieldName = crate::field_id!("plan_event_title");
const EVENT_NOTES: design::FieldName = crate::field_id!("plan_event_notes");
const EVENT_PLACE: design::FieldName = crate::field_id!("plan_event_place");

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
    /// Whether `Add to Log` fits on the row, for the last few rooms it was measured in. The
    /// words and the done mark are this model's, so only the room can change it
    /// (`Scene::log_keeps_rows`).
    pub(super) fit: std::sync::Mutex<Vec<(f32, f32, bool)>>,
    /// Its words' lines as last measured, by the room, the font generation and the scale: a
    /// quiet frame measures none (layout 11c A2c).
    words: Box<std::sync::Mutex<Option<(SceneRoom, SceneLines)>>>,
}

/// The room a scene's words were measured in: its width, the font generation and the scale; and
/// whether they were measured with `Not found` after them.
type SceneRoom = (u32, u64, u32, bool);

/// A scene's words as measured: how many lines they take alone; for a scene whose entry is gone,
/// also the words that fit with `Not found` in those lines.
#[derive(Clone)]
struct SceneLines {
    alone: usize,
    gone: Option<String>,
}

impl Scene {
    /// The scene's words measured in `width`, or as last measured there.
    fn lines(
        &self,
        ui: &egui::Ui,
        (width, gone): (f32, bool),
        measure: impl FnOnce() -> SceneLines,
    ) -> SceneLines {
        let room = (
            width.to_bits(),
            design::theme::font_generation(ui.ctx()),
            ui.ctx().pixels_per_point().to_bits(),
            gone,
        );
        let mut kept = self
            .words
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match &*kept {
            Some((at, lines)) if *at == room => lines.clone(),
            _ => {
                let lines = measure();
                *kept = Some((room, lines.clone()));
                lines
            }
        }
    }
}

impl Scene {
    /// The fit rule for this scene's `Add to Log`, measured again only when the room or the
    /// action's width changes; an unchanged frame lays out no words.
    fn log_keeps_rows(&self, ui: &egui::Ui, plain: &str, menu: f32, with_log: f32) -> bool {
        let room = ui.available_width();
        #[cfg(any(test, feature = "test-support"))]
        FIT_COUNTS.with(|counts| counts.set((counts.get().0 + 1, counts.get().1)));
        let mut fit = self
            .fit
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(&(_, _, keeps)) = fit
            .iter()
            .find(|(at, wide, _)| *at == room && *wide == with_log)
        {
            return keeps;
        }
        let keeps = log_keeps_rows(ui, plain, menu, with_log);
        // A few rooms: the window's sizes, with Lookup open or closed.
        if fit.len() == FIT_ROOMS {
            fit.remove(0);
        }
        fit.push((room, with_log, keeps));
        keeps
    }
}

/// How many rooms a done scene's fit remembers.
const FIT_ROOMS: usize = 4;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How often frames reached a done scene's fit, and how often it laid out the words.
    static FIT_COUNTS: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
}

/// A done scene's fit on this (the UI) thread: frames that reached it, and layouts of its words.
#[cfg(any(test, feature = "test-support"))]
pub fn scene_fit_counts() -> (usize, usize) {
    FIT_COUNTS.with(std::cell::Cell::get)
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
                            fit: std::sync::Mutex::new(Vec::new()),
                            words: Box::default(),
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

impl ProjectionCache {
    /// The revision of the projection last asked for: it changes whenever the projection's
    /// inputs do (`ProjectionInputs`).
    fn revision(&self) -> Option<design::memo::Revision> {
        self.inputs.as_ref().map(design::memo::Revised::revision)
    }
}

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
pub(super) enum Place {
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

/// The played fight's Log as laid out for one width: its newest entries first (the run's summary
/// holds the log newest first), each on one line, cut by whole words with `…` and whole on hover,
/// and, when an entry is left out or cut, a last line that says what Whole run has. Every row is
/// one `Small` line, the notice included, so the block is never taller than a third of the
/// reader's measure, whatever a DM's note holds.
struct LogView {
    rows: Vec<(Arc<egui::Galley>, String)>,
    notice: Option<Arc<egui::Galley>>,
}

/// What a `LogView` was laid out from: the log's revision as read, the width, the text size and
/// the scale.
type LogKey = (LogRevision, u32, u32, u32);

/// The played fight's Log, laid out again only when its log as read (`revision`, taken once per
/// read from disk), the width or the text size changed: a quiet frame lays out and hashes
/// nothing, however long the log or a note in it.
fn log_view(ui: &egui::Ui, run: Uuid, revision: LogRevision, log: &[String]) -> Arc<LogView> {
    let width = ui.available_width();
    let font = Type::Small.font(ui);
    let key: LogKey = (
        revision,
        width.to_bits(),
        font.size.to_bits(),
        ui.ctx().pixels_per_point().to_bits(),
    );
    let id = egui::Id::new(("plan_fight_log", run));
    if let Some((seen, view)) = ui.data(|data| data.get_temp::<(LogKey, Arc<LogView>)>(id))
        && seen == key
    {
        return view;
    }
    // One row: a `Small` line and the gap after it; the block holds as many as a third of the
    // reader's measure does, and at least the newest entry and the notice.
    let gap = ui.spacing().item_spacing.y;
    let room = ((design::READER_MEASURE / 3.0 + gap) / (Type::Small.line() + gap))
        .floor()
        .max(2.0) as usize;
    let line = |words: &str| {
        let mut job = egui::text::LayoutJob::simple_singleline(
            words.replace(['\n', '\r'], " "),
            font.clone(),
            colors().muted,
        );
        job.wrap.max_width = width.max(1.0);
        job.wrap.max_rows = 1;
        job.wrap.break_anywhere = false;
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    };
    let mut rows: Vec<_> = log
        .iter()
        .take(room)
        .map(|entry| (line(entry), entry.clone()))
        .collect();
    let notice = (log.len() > rows.len() || rows.iter().any(|(row, _)| row.elided)).then(|| {
        rows.truncate(room - 1);
        let words = match log.len() - rows.len() {
            0 => "Whole run has every line whole.".to_owned(),
            1 => "Whole run has 1 more line.".to_owned(),
            more => format!("Whole run has {more} more lines."),
        };
        line(&words)
    });
    let view = Arc::new(LogView { rows, notice });
    ui.data_mut(|data| data.insert_temp(id, (key, view.clone())));
    view
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
pub(super) enum SceneAct {
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

/// One scene: its box where a fight has its caret, its words at a plan row title's size, its
/// entry's kind, status and place, and its menu. Returns the row's height, as the fold
/// measures it.
fn scene_row(
    ui: &mut egui::Ui,
    (width, columns): (f32, design::PlanColumns),
    scene: &Scene,
    (slot, line): (Slot, usize),
    plan: &Plan<'_>,
    act: &mut Option<SceneAct>,
) -> f32 {
    let c = colors();
    let task = &scene.task;
    let plain = plain_links(&task.words);
    // A done scene that links one World entry offers its Log in the row's action place.
    let log = scene
        .entry
        .as_ref()
        .and_then(|entry| entry.log.as_ref())
        .filter(|_| task.done);
    let log_name = format!("Add to Log: {plain}");
    // On the row it has no icon, as the row's other actions (X29); its menu line keeps it.
    let row_log = || {
        Action::quiet("Add to Log")
            .density(Density::Read)
            .name(&log_name)
    };
    let log_action = || row_log().icon(icons::ADD);
    // On the row while it fits, else a line of the scene's own menu.
    let menu = Density::Read.control();
    let with_log = log
        .map(|_| menu + row_log().natural_width(ui) + design::S)
        .filter(|&with_log| scene.log_keeps_rows(ui, &plain, menu, with_log));
    let on_row = with_log.is_some();
    let action_width = with_log.unwrap_or(menu);
    let widths = [width - action_width - design::S, action_width];
    let mut tick = false;
    let mut remove = false;
    let mut start = None;
    let mut add_log = false;
    let top = ui.cursor().top();
    ui.push_id(("scene", task.at, &task.line), |ui| {
        plan_row_with(
            ui,
            (&widths, columns),
            |ui| {
                let mut done = task.done;
                let response = scene_box(ui, &mut done);
                let state = if task.done { "Done" } else { "Open" };
                design::named(ui, &response, &format!("{state}: {plain}"));
                tick = response.clicked();
            },
            |ui| {
                if scene.not_found && !scene.second_line {
                    gone_scene_words(ui, scene, &plain);
                } else {
                    scene_words(ui, scene, &plain);
                }
            },
            |ui| {
                if let Some(detail) = scene
                    .entry
                    .as_ref()
                    .and_then(|entry| entry.detail.as_deref())
                {
                    wrapped_words(ui, Type::Small, detail, c.muted);
                } else if scene.not_found && scene.second_line {
                    // As Lookup and World say it of a link whose entry is gone (X14).
                    wrapped_words(ui, Type::Small, "Not found", c.result.warning);
                }
            },
            |ui| {
                design::row_menu(ui, &plain, |ui| {
                    // The act that starts something comes first; the removal is last.
                    // While a run is open, the reader's Add to fight is the way in.
                    if plan.open_run.is_none()
                        && let Some(fight) =
                            scene.entry.as_ref().and_then(|entry| entry.start.as_ref())
                    {
                        if Action::quiet("Start fight")
                            .icon(icons::RUN)
                            .name(&format!("Start a fight with {}", fight.name))
                            .blocked(plan.blocked)
                            .show(ui)
                            .clicked()
                        {
                            start = Some(fight.clone());
                            ui.close();
                        }
                        // A note under its choice, at its words after the icon (M-P0.27).
                        design::choice_note(ui, true, plan.blocked);
                    }
                    if log.is_some() && !on_row && log_action().show(ui).clicked() {
                        add_log = true;
                        ui.close();
                    }
                    if Action::quiet("Remove from tonight")
                        .icon(icons::REMOVE)
                        .show(ui)
                        .clicked()
                    {
                        remove = true;
                        ui.close();
                    }
                });
                if on_row {
                    add_log = row_log().show(ui).clicked();
                }
            },
        );
        plan_rule(ui);
    });
    if tick {
        *act = Some(SceneAct::Tick(slot, task.clone()));
    } else if remove {
        *act = Some(SceneAct::Remove(slot, line, task.clone()));
    } else if let Some(fight) = start {
        *act = Some(SceneAct::StartFight(fight));
    } else if let Some((id, name)) = log.filter(|_| add_log) {
        *act = Some(SceneAct::AddToLog(
            *id,
            name.clone(),
            words_outside_links(&task.words),
        ));
    }
    ui.cursor().top() - top
}

/// A scene's words at a plan row title's size. A one-line title is centred in its link's `XL`
/// target (`design::wrapped_link`), and a row's words start at the top of their row, so a
/// one-line scene starts that much lower: its line is the fights' title line, and its second
/// line keeps a fight row's pitch (app round 2, X12).
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn scene_words(ui: &mut egui::Ui, scene: &Scene, plain: &str) {
    let task = &scene.task;
    let font = Type::Body.font(ui);
    let width = ui.available_width();
    let lines = scene.lines(ui, (width, false), || {
        #[cfg(any(test, feature = "test-support"))]
        design::text_measured();
        SceneLines {
            alone: ui.fonts_mut(|fonts| {
                fonts
                    .layout(plain.to_owned(), font.clone(), colors().text, width)
                    .rows
                    .len()
            }),
            gone: None,
        }
    });
    if lines.alone == 1 {
        let row_height = ui.fonts_mut(|fonts| fonts.row_height(&font));
        ui.add_space(((design::XL - row_height) / 2.0).max(0.0));
    }
    crate::markdown_view::show_row_words(ui, &task.words, task.done);
}

/// The words of a scene whose entry is gone and that had no second line: the words, then the
/// warning mark and `Not found` at the end of their line, in the lines the words take alone, so
/// the row keeps its height. Where the mark has no room, the words give up whole words from
/// their end first; hover keeps them whole (review round 1).
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn gone_scene_words(ui: &mut egui::Ui, scene: &Scene, plain: &str) {
    let task = &scene.task;
    let c = colors();
    let width = ui.available_width();
    let words_color = if task.done { c.muted } else { c.text };
    let (body, small) = (Type::Body.font(ui), Type::Small.font(ui));
    let job = |words: &str| {
        let mut job = egui::text::LayoutJob::default();
        let format = |font_id: &egui::FontId, color| egui::TextFormat {
            font_id: font_id.clone(),
            color,
            ..Default::default()
        };
        job.append(words, 0.0, format(&body, words_color));
        job.append(
            &format!("  {} Not found", icons::WARN),
            0.0,
            format(&small, c.result.warning),
        );
        job.wrap.max_width = width;
        job
    };
    let all: Vec<&str> = plain.split_whitespace().collect();
    let whole = all.join(" ");
    let lines = scene.lines(ui, (width, true), || {
        #[cfg(any(test, feature = "test-support"))]
        design::text_measured();
        let rows =
            |job: egui::text::LayoutJob| ui.fonts_mut(|fonts| fonts.layout_job(job).rows.len());
        let alone = ui.fonts_mut(|fonts| {
            fonts
                .layout(plain.to_owned(), Type::Body.font(ui), words_color, width)
                .rows
                .len()
        });
        let fitted = (1..=all.len())
            .rev()
            .map(|kept| {
                let words = all[..kept].join(" ");
                if kept == all.len() {
                    words
                } else {
                    format!("{words}…")
                }
            })
            .find(|words| rows(job(words)) <= alone)
            .unwrap_or_else(|| format!("{}…", all.first().copied().unwrap_or_default()));
        SceneLines {
            alone,
            gone: Some(fitted),
        }
    });
    let alone = lines.alone;
    let fitted = lines.gone.unwrap_or_else(|| whole.clone());
    let shown = job(&fitted);
    if alone == 1 {
        let row_height = ui.fonts_mut(|fonts| fonts.row_height(&body));
        ui.add_space(((design::XL - row_height) / 2.0).max(0.0));
    }
    let response = ui.add(egui::Label::new(shown).wrap());
    design::named(ui, &response, &format!("{plain} · Not found"));
    if fitted != whole {
        response.on_hover_text(plain);
    }
}

/// A scene's box, the theme's checkbox with a 24 px target, in the leading slot that a fight's
/// caret has. Its target ends where the words start, so the words take the fights' title column;
/// it reaches `box_hang` into the margin before the rows.
fn scene_box(ui: &mut egui::Ui, done: &mut bool) -> egui::Response {
    let slot = ui.max_rect();
    let rect = egui::Rect::from_min_size(
        egui::pos2(slot.right() + design::S - design::XL, slot.top()),
        egui::vec2(design::XL, design::XL),
    );
    let mut cell = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    cell.spacing_mut().interact_size.y = design::XL;
    cell.add(egui::Checkbox::without_text(done))
}

/// How far a scene's box reaches before the rows: its target, less the leading slot and the gap
/// after it.
fn box_hang() -> f32 {
    (design::XL - lead_width() - design::S).max(0.0)
}

/// Every plan row's leading slot: a fight's caret, a group's icon, a scene's box.
fn lead_width() -> f32 {
    Type::Body.size()
}

/// The fit rule for a done scene's `Add to Log` on its row: the scene's words keep their rows
/// beside an action place `with_log` wide, each with the same words as beside the menu alone
/// (`menu` wide), so no word moves.
fn log_keeps_rows(ui: &egui::Ui, plain: &str, menu: f32, with_log: f32) -> bool {
    #[cfg(any(test, feature = "test-support"))]
    design::text_measured();
    #[cfg(any(test, feature = "test-support"))]
    FIT_COUNTS.with(|counts| counts.set((counts.get().0, counts.get().1 + 1)));
    // The words' room beside the action place, as `plan_row_with` gives it after the leading
    // slot.
    let room = ui.available_width() - lead_width() - 2.0 * design::S;
    // A plan row title's type, as `show_row_words` draws the words, and each row's words as it
    // breaks them there.
    let rows = |action: f32| {
        let width = room - action;
        let font = Type::Body.font(ui);
        ui.fonts_mut(|fonts| {
            let galley = fonts.layout(plain.to_owned(), font, colors().text, width);
            galley
                .rows
                .iter()
                .map(|row| row.glyphs.len())
                .collect::<Vec<_>>()
        })
    };
    rows(with_log) == rows(menu)
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

/// Each fight still to come in the plan, in order, by its name, with the day it is played on
/// when its line came from a journey's check.
fn road_days(
    body: &str,
    events: &[timeline::Event],
    session: Option<Uuid>,
    fights: &[PlanFight],
) -> RoadDays {
    let starts = timeline::clock(events);
    plan_links(body, events, session, fights)
        .into_iter()
        .filter(|(_, _, waiting)| *waiting)
        .map(|(at, fight, _)| {
            let line = body[at..].lines().next().unwrap_or_default();
            let day = checks::marked(line)
                .mark
                .and_then(|mark| mark.day_words(events, &starts, session));
            (fight.name.to_lowercase(), day)
        })
        .collect()
}

/// The day a planned fight is played on, in the form that fits after its summary: `Day 2 of
/// Road to the ford`, else `Day 2`. As the row's rating word does, the words keep the row's
/// lines (rule P): first the journey's name goes; the day itself gives way last, in the row's
/// cut order (`planned_summary_on_road`).
/// `planned_summary_on_road` draws it after the summary.
#[cfg(test)]
fn road_day(ui: &egui::Ui, summary: &str, day: Option<&str>) -> Option<String> {
    road_day_in(ui, (summary, ui.available_width()), day)
}

/// `road_day` for a summary laid out in `width` (a plan row's summary room).
fn road_day_in(ui: &egui::Ui, (summary, width): (&str, f32), day: Option<&str>) -> Option<String> {
    let day = day?;
    let font = Type::Small.font(ui);
    let rows = |text: &str| {
        let laid = without_loose_separators(ui, fight_summary_text(text), &font, width);
        ui.fonts_mut(|fonts| {
            fonts
                .layout(laid, font.clone(), colors().muted, width)
                .rows
                .len()
        })
    };
    let with = |words: &str| {
        if summary.trim().is_empty() {
            words.to_owned()
        } else {
            format!("{summary} · {words}")
        }
    };
    let alone = rows(summary).max(1);
    let short = day.split_once(" of ").map_or(day, |(head, _)| head);
    // Where neither form keeps the lines, the short one goes to the row, whose cut order lets
    // the day give way last (round 28 FE6, review of PR 324).
    Some(
        [day, short]
            .into_iter()
            .find(|words| rows(&with(words)) <= alone)
            .unwrap_or(short)
            .to_owned(),
    )
}

/// The help line of **How long** when the clock cannot place the journey.
const LENGTH_HELP: &str = timeline::LENGTH_HELP;

/// Which result's Undo the DM pressed in a journey's checks.
#[derive(Clone, Copy, PartialEq)]
enum TravelUndo {
    None,
    Checks,
}

/// A journey's encounter checks in the plan: what the DM typed, each journey's last result with
/// its Undo, and the plan line's Undo. The length being typed is the Timeline book's.
#[derive(Default)]
pub(super) struct Travel {
    block: checks::State,
    /// The open journey's start on the clock, while the Timeline is as it was: its count of
    /// changes, the event, its place in the list, and the minute.
    start: Option<(u64, String, Option<usize>, Option<u64>)>,
    focus_length: bool,
    /// Each journey's last Timeline act, by its event. Leaving the page ends them.
    done: checks::Results,
    /// `Plan “<name>”`: the journey, the name and the check it was clicked on, and the note's
    /// Undo, with its order and the refusal of its last Undo. It lasts while the note's words are
    /// what the act left, as every note act's Undo does; a page change keeps it. The one store
    /// both readers read.
    planned: design::Results<PlanLineOwner, PlannedLine>,
    /// The fights still to come with their days on the road, while the note, the Timeline and
    /// the campaign's fights are as they were.
    days: Option<(RoadKey, Arc<PlannedDays>)>,
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

/// What the plan's days on the road are read from: the session, the note's revision, the
/// Timeline's count of changes, and the revision of the campaign's fights (a fight renamed on
/// disk changes which rows are planned, though their count stays; review of PR 717).
type RoadKey = (Option<Uuid>, u64, u64, design::memo::Revision);

/// Each fight still to come, by its name, with its day on the road when it has one.
type RoadDays = Vec<(String, Option<String>)>;

/// The planned fights' days, worked out once for each key of the road's days (`planned_days`):
/// the day of each planned fight by its place in the plan (`None` with no day).
#[derive(Default)]
struct PlannedDays {
    at: Vec<Option<String>>,
}

#[cfg(test)]
thread_local! {
    /// How often the planned fights' days were worked out (`days_by_place`).
    static DAYS_BY_PLACE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The day of each planned fight, by its place: that fight's own turn among the rows to come.
/// One pass: each name's days are taken in order as its rows come.
fn days_by_place(days: &[(String, Option<String>)], planned: &[&PlanFight]) -> Vec<Option<String>> {
    #[cfg(test)]
    DAYS_BY_PLACE.with(|count| count.set(count.get() + 1));
    let mut by_name: std::collections::HashMap<&str, std::collections::VecDeque<&Option<String>>> =
        std::collections::HashMap::new();
    for (name, day) in days {
        by_name.entry(name.as_str()).or_default().push_back(day);
    }
    planned
        .iter()
        .map(|fight| {
            by_name
                .get_mut(fight.name.to_lowercase().as_str())
                .and_then(std::collections::VecDeque::pop_front)
                .and_then(Clone::clone)
        })
        .collect()
}

/// F7's action and its words.
const ADD_REWARDS: &str = "Add rewards";
const REWARDS_BLOCKED: &str = "Splat cannot read this fight’s run, so its rewards are not known.";

/// Where a played row has `Add rewards`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RewardsAt {
    Row,
    Link,
    Menu,
}

/// What the DM pressed for a played fight: `Add rewards`, or `Undo rewards`, by its event.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum RewardsAct {
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
    /// Whether `Add rewards` fits on a played row: by the title, its lead icon, the row's width
    /// and the action's. Worked out when one of them changes, not each frame.
    fit: BTreeMap<(String, &'static str, i64, i64), RewardsAt>,
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
    pub(super) fn see(&mut self, draft: Option<&crate::sessions::Draft>, session: Option<Uuid>, book: &Book) {
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

    /// Where a played row has `Add rewards` (rule 2, the fit rule): in its action band while the
    /// title keeps the width and the line breaks it has beside the menu alone, at the action's
    /// own width; else as the last link of the opened row while that line has room for it; else
    /// as a line of the row's menu. No line is added. `row` is the row's width with its icon.
    fn fits(
        &mut self,
        ui: &egui::Ui,
        title: &str,
        icon: &'static str,
        row: f32,
        action: f32,
    ) -> RewardsAt {
        #[cfg(any(test, feature = "test-support"))]
        design::text_measured();
        let key = (
            title.to_owned(),
            icon,
            (row * 10.0).round() as i64,
            (action * 10.0).round() as i64,
        );
        if let Some(fits) = self.fit.get(&key) {
            return *fits;
        }
        let word = |words: &str, kind: Type| {
            ui.painter()
                .layout_no_wrap(words.to_owned(), kind.font(ui), colors().text)
                .size()
                .x
        };
        let galley = |text: egui::RichText, room: f32| {
            egui::WidgetText::from(text).into_galley(
                ui,
                Some(egui::TextWrapMode::Wrap),
                room,
                egui::TextStyle::Body,
            )
        };
        // As `plan_row_with` lays it out: the icon, a gap, and the title beside the band.
        let column = row - galley(Type::Body.rich(ui, icon), f32::INFINITY).size().x - design::S;
        // The title's width and where each of its lines breaks, beside a band.
        let layout = |band: f32| {
            let title = galley(
                Type::Body.rich(ui, title).underline(),
                (column - band - design::S).max(design::XL).max(1.0),
            );
            let breaks: Vec<usize> = title
                .rows
                .iter()
                .map(|row| row.char_count_excluding_newline())
                .collect();
            (breaks, title.size().x)
        };
        // As `fight_block` lays out its links, beside the opened row's indent: four words and
        // three `·`, with one blank (`link_gap`) on each side of each `·`.
        let spacing = link_gap(ui);
        let links = ["Log", "Whole run", "Timeline", ADD_REWARDS]
            .map(|link| word(link, Type::Body))
            .iter()
            .sum::<f32>()
            + 3.0 * word("·", Type::Small)
            + 6.0 * spacing;
        let (with, alone) = (layout(action), layout(Density::Read.control()));
        let fits = if with.0 == alone.0 && (with.1 - alone.1).abs() < 0.5 {
            RewardsAt::Row
        } else if links <= row - Type::Body.size() - design::S {
            RewardsAt::Link
        } else {
            RewardsAt::Menu
        };
        if self.fit.len() > 256 {
            self.fit.clear();
        }
        self.fit.insert(key, fits);
        fits
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

    /// The open Travel row's checks: the table, **How long**, the lines and the last results.
    /// Returns the act the DM asked for, whether the length changed, and which Undo was pressed.
    fn travel_block(
        &mut self,
        ui: &mut egui::Ui,
        event: &mut timeline::Event,
        start: Option<u64>,
        plan: &Plan<'_>,
        planned: &dyn Fn(&str) -> Option<bool>,
        length_draft: &mut checks::LengthDraft,
    ) -> (Option<checks::Act>, checks::Taken, TravelUndo) {
        let travel = &mut self.travel;
        let focus = std::mem::take(&mut travel.focus_length);
        let mut undo = TravelUndo::None;
        design::gap(ui, design::Gap::XS);
        // Round 27 FE7: **How long** and its range come first, so the checks' results follow
        // their table directly, as on Timeline. The plan line's result is in the plan's footer.
        let typed = length_draft.words(event);
        let help = checks::span(start, &typed).or_else(|| {
            timeline::read_duration(&typed)
                .is_none()
                .then(|| LENGTH_HELP.to_owned())
        });
        let (length, changed) = design::field(ui, "How long", help.as_deref(), None, |ui| {
            checks::length_input(ui, length_draft, event, "", focus)
        });
        let view = checks::View {
            event,
            start,
            length: &length,
            tables: plan.tables,
            planned,
            reveal: None,
        };
        let mut act = checks::header(ui, &view, &mut travel.block);
        // M0515 (M-P0.27): the journey's check result keeps its line, as the Timeline's does;
        // the two share `checks::Done`, now typed.
        let result = travel
            .done
            .get(&event.id)
            .map(|done| PlacedResult::new(done.words.clone(), done.undo.clone().pressed_as(())));
        if PlacedResult::show_line(result, ui).is_some() {
            undo = TravelUndo::Checks;
        }
        act = checks::lines(ui, &view, &mut travel.block).or(act);
        (act, changed, undo)
    }

    /// The open journey's start on the clock, worked out again only when the Timeline changed
    /// or another journey opened.
    /// Also the open event's place in the list, for `checks::settle_length`.
    fn journey_start(&mut self, book: &Book, open: Option<&str>) -> (Option<usize>, Option<u64>) {
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

    /// The days on the road of the fights still to come, read again only when what they are
    /// read from changed.
    /// The planned fights' days, worked out again only when the session, its draft, the
    /// Timeline or the campaign's fights change; each new answer also ends the plan lines of
    /// fights no longer planned and gives the kept lines a new days' revision
    /// (`forget_plan_lines`). An unchanged frame does neither.
    fn planned_days(
        &mut self,
        book: &Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
        planned: &[usize],
    ) -> Arc<PlannedDays> {
        let Some(draft) = &self.draft else {
            return Arc::default();
        };
        let key = (
            session,
            draft.revision(),
            book.changes(),
            plan.fights_revision,
        );
        if let Some((known, days)) = &self.travel.days
            && *known == key
        {
            return Arc::clone(days);
        }
        let road = road_days(&draft.body, &book.events, session, plan.fights);
        let planned: Vec<&PlanFight> = planned.iter().map(|&at| &plan.fights[at]).collect();
        let days = Arc::new(PlannedDays {
            at: days_by_place(&road, &planned),
        });
        self.travel.days = Some((key, Arc::clone(&days)));
        let seen: Vec<_> = planned
            .iter()
            .zip(&days.at)
            .enumerate()
            .map(|(at, (fight, day))| ((session, at, fight.id), day.clone()))
            .collect();
        if seen != *self.plan_lines_seen {
            self.forget_plan_lines(seen);
        }
        days
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

    /// Tests: no disk check runs until `release_disk_check`, however long the frames take
    /// (the last check is set an hour ahead; `poll` reads the time since it as zero), so a
    /// scenario's outside change meets the DM's next press before `poll` reads it.
    #[cfg(any(test, feature = "test-support"))]
    pub fn hold_disk_check(&mut self) {
        self.last_check = Some(Instant::now() + Duration::from_secs(3600));
    }

    /// Tests: the disk checks run again, from now.
    #[cfg(any(test, feature = "test-support"))]
    pub fn release_disk_check(&mut self) {
        self.last_check = Some(Instant::now());
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

    /// G118: the one end of the plan's kept lines. Only the lines of the planned fights in
    /// `ids` stay (none at a campaign switch); a fight removed or replaced, another session or
    /// another campaign ends its line.
    pub(super) fn forget_plan_lines(&mut self, seen: Vec<(PlanLineId, Option<String>)>) {
        self.plan_lines
            .retain(|id| seen.iter().any(|(planned, _)| planned == id));
        self.plan_lines_seen.set(seen);
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

    /// The session as a list, top to bottom: what happened, then what is still to come. It
    /// replaces a walk between Encounters, Timeline and the note.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls, and a by-owner Undo (design::results::STAGED_ALLOWS)"
    )]
    pub(super) fn plan_pane(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        plan: &Plan<'_>,
        page: Option<(
            &crate::processing::settings::ProcessingSettings,
            &crate::sessions::Note,
        )>,
    ) {
        let session = self.selected;
        let here = self.group_session == session;
        // The group being assembled, or the file's problem, takes the pane: on the session
        // page, the page's messages keep their footer under it.
        let takes_pane = (plan.group_open && here) || book.conflict;
        if takes_pane {
            let group = plan.group_open && here;
            let draw = |ui: &mut egui::Ui, book: &mut Book| {
                if group {
                    (plan.group_editor.borrow_mut())(ui);
                } else {
                    let _ = problems(ui, book);
                }
            };
            match page {
                Some((settings, note)) => {
                    let frame = PaneFrame::new(ui);
                    let messages = self.session_messages(
                        &frame,
                        settings,
                        book,
                        plan,
                        note,
                        ui.available_width(),
                    );
                    let paned = messages.pane(
                        ui,
                        super::fills_body(design::pane::FillsReason::Columns),
                        |ui| draw(ui, book),
                    );
                    if let Some(press) = paned.pressed {
                        self.session_press(press, book, plan);
                    }
                }
                None => draw(ui, book),
            }
            return;
        }
        // A file that cannot be read says so above the plan; the plan stays.
        let _ = problems(ui, book);
        let c = colors();
        self.end_plan_results_after_edits(book);
        self.scenes.end_what_changed(self.draft.as_ref());
        if self
            .removed_group
            .as_ref()
            .is_some_and(|(id, _, _, after, _, _)| {
                *id == session
                    && self
                        .draft
                        .as_ref()
                        .is_none_or(|draft| !draft.body.same_words(after))
            })
        {
            self.removed_group = None;
        }
        // Layout 11c A2a (the v2 review's item 5): what the closed rows read from the note, the
        // Timeline, the fights and groups, the saved runs and the content index, worked out once
        // per change of them (`PlanProjection`), never on a quiet frame.
        let index = ui
            .ctx()
            .data(|data| data.get_temp::<Arc<ContentIndex>>(egui::Id::new("reader_link_index")));
        let projection = self.plan_projection(book, session, plan, index.as_deref());
        // Each scene's line among the plan's lines, at each place.
        let scenes = &projection.scenes;
        // Where a removed scene's row stood: before the line that now follows it.
        let removed_slot = self
            .scenes
            .removed
            .as_ref()
            .and_then(|(line, ..)| projection.line_place.get(*line).copied());
        let scene_count = projection.scene_count;
        let planned_fights = || projection.planned.iter().map(|&at| &plan.fights[at]);
        let planned_groups = || projection.planned_groups.iter().map(|&at| &plan.groups[at]);
        // While the DM types a scene, the fold holds.
        let hold =
            self.scenes.typing || self.scenes.field.as_ref().is_some_and(|field| field.focus);
        let (folded, fold_line) = self.scenes.fold.state(scene_count, hold);
        // Each scene result's place in this frame: its own while that is drawn, else the footer.
        let rows = (!folded).then_some(scenes.as_slice());
        let shown = Shown {
            added: self
                .scenes
                .added
                .as_ref()
                .map(|_| self.shown_at(Place::Field, rows)),
            removed: self
                .scenes
                .removed
                .as_ref()
                .map(|_| self.shown_at(removed_slot.map_or(Place::Footer, Place::Row), rows)),
            fault: self
                .scenes
                .fault
                .as_ref()
                .map(|fault| self.shown_at(fault.place, rows)),
        };
        let mut scene_act = None;
        // The closed rows' heights, without and with the scenes, for the next frame's fold.
        let mut closed = 0.0;
        let mut closed_scenes = 0.0;
        // The next fight's place, and the opened rows' details above it, for the fold.
        let mut details_above = 0.0;
        let mut next = None;
        // B4b N2: a refusal ends when its group is no longer among this plan's groups (removed,
        // renamed away, or its file now holds another group), read here with the groups.
        if let Some((refused_in, id)) = self.plan_group_refused
            && refused_in == session
            && !planned_groups().any(|group| group.id == id)
        {
            self.plan_group_refused = None;
        }
        // Round 28 FD3: the heading counts the rows it stands above, as every group heading does:
        // the events, the planned fights and groups, and every scene, done or not.
        let heading = design::group_heading_text(
            ui,
            if session == self.current() {
                "Tonight"
            } else {
                "This session"
            },
            projection.events_here
                + projection.planned.len()
                + projection.planned_groups.len()
                + scene_count,
        );
        let heading = ui.label(heading);
        if fold_line {
            self.fold_line(ui, heading.rect, scene_count, folded);
        }
        let scene = Action::quiet("Scene")
            .icon(icons::CHECK_SQUARE)
            .name("Add Scene")
            .density(Density::Read);
        let added = match quick_add_with(ui, book, session, &PLAN_KINDS, vec![scene], |ui| {
            self.fight_menu(ui, plan);
            self.group_menu(ui, plan);
        }) {
            Some(Ok(id)) => Some(id),
            Some(Err(_)) => {
                let mut field = design::AddField::default();
                field.focus = true;
                self.scenes.field = Some(field);
                None
            }
            None => None,
        };
        // Planned fights not yet played, and whether each has an ended run: a removal notice
        // says when one with the same name stays.
        let planned_results = &projection.planned_results;
        // Round 26 FL1, round 28: the plan's results, reasons and notices sit in its pane's
        // message footer (`plan_footer_messages`).
        let frame = PaneFrame::new(ui);
        let width = ui.available_width();
        let messages =
            self.plan_footer_messages(&frame, book, plan, page, (shown, planned_results), width);
        // The fold measures the room the rows have with an empty footer: the pane's whole room
        // (below), so that a result coming or going never folds or unfolds the scenes.
        if let Some(id) = &added {
            self.row_reveal = Some((id.clone(), 0, ui.ctx().cumulative_pass_nr()));
            self.record_plan_add(id, book, ui.ctx());
        }
        // An Add in this frame changed the Timeline after the projection read it: its days are
        // worked out again for this frame only.
        let fresh_days;
        let days: &[u32] = if added.is_some() {
            fresh_days = timeline::day_numbers(&book.events);
            &fresh_days
        } else {
            &projection.days
        };
        // Layout 11c A2c-3: the plan's body is its measured region when every row it draws is a
        // closed row laid by its recipe: no event or planned fight open, no scene row drawn,
        // every group laid by its recipe with no reason or refusal under its row, and no removal
        // message among the rows. Else its rows draw in place. Committed here, before any row.
        let removal_shows = self.removed_fight.as_ref().is_some_and(|(id, _, _, undo)| {
            *id == session && self.draft.as_ref().is_some_and(|draft| undo.live(draft))
        });
        // A scene act's words at a row's place (a removal, an Undo's refusal) show among the rows
        // (`scene_rows`), also where no scene is left (review of PR 869, 869-a): they keep the
        // rows in place, so they and their Undo show where they always do.
        let scene_words_among_rows = matches!(shown.removed, Some(Place::Row(_)))
            || matches!(shown.fault, Some(Place::Row(_)));
        let measured = self.open_event.is_none()
            && self.open_planned.is_none()
            && (folded || scenes.iter().all(|scenes| scenes.is_empty()))
            && !removal_shows
            && !scene_words_among_rows
            && planned_groups()
                .all(|group| group.warnings.is_empty() && group.own_problems().is_empty())
            && !(plan.group_blocked.is_some()
                && self.plan_group_refused.is_some_and(|(at, _)| at == session));
        let mut acts = RowActs {
            insert: None,
            remove: None,
            remove_group: None,
            remove_fight: None,
            delete_run: None,
            travel_act: None,
            rewards_act: None,
            primary_available: plan.open_run.is_none() && !self.confirm_record,
            next_action: None,
        };
        let mut toggle = None;
        let mut changed = false;
        let open_event = self.open_event.clone();
        let mut ruling = None;
        // A journey's checks: the clock at the open journey's start, and the results that hold.
        let (open_at, _) = self.journey_start(book, open_event.as_deref());
        checks::settle_length(book, open_at);
        // A length that settled changed the Timeline, so the start is read again.
        let (_, journey_start) = self.journey_start(book, open_event.as_deref());
        self.travel.done.follow(book);
        // The plan line's result follows its note.
        self.follow_plan_line();
        let mut length_taken = checks::Taken::Nothing;
        self.rewards.see(self.draft.as_ref(), session, book);
        // A footer Undo of rewards reaches only the result its line showed.
        let mut rewards_ticket = None;
        let mut travel_undo = (TravelUndo::None, String::new());
        // A name a result links: an encounter or a group, and whether a row for it is to come.
        let in_plan = |name: &str| {
            offer_state(
                name,
                plan.fights
                    .iter()
                    .map(|fight| fight.name.as_str())
                    .chain(plan.groups.iter().map(|group| group.name.as_str())),
                planned_fights()
                    .map(|fight| fight.name.as_str())
                    .chain(planned_groups().map(|group| group.name.as_str())),
            )
        };
        // Review of PR 274: a draw the plan missed (the DM left the view) ends a pending reveal.
        // The DM's own input to the plan's column ends it too, judged by the one cancel rule in
        // the pane (`design::reveal::dm_scrolls`; review of #852): input to another column (a
        // scroll in Lookup) never does.
        let pass = ui.ctx().cumulative_pass_nr();
        if self
            .row_reveal
            .as_ref()
            .is_some_and(|(_, _, last)| pass > last + 1)
        {
            self.row_reveal = None;
        }
        let mut reveal = Reveal::new(self.row_reveal.as_ref().map(|(id, _, _)| id.clone()));
        let mut reveal_cancelled = false;
        // A scene's box reaches `box_hang` before the rows, so that every row's words start at
        // one column (`scene_box`). The pane starts that much before the rows, so the box lies
        // whole inside its scroll column, and the rows keep their place.
        let hang = box_hang();
        let full = ui.available_rect_before_wrap();
        // The file's flags and their file line are footer messages (`file_messages`).
        let wide = full;
        // The fold's room: the pane's, with an empty footer.
        let room = wide.height();
        let wide = wide.with_min_x(wide.left() - hang);
        let mut pane_ui = ui.new_child(egui::UiBuilder::new().max_rect(wide).layout(*ui.layout()));
        pane_ui.set_clip_rect(ui.clip_rect().with_min_x(wide.left()));
        let paned = messages.pane(&mut pane_ui, design::Body::Content, |outer| {
            if reveal.target.is_some() && design::reveal::dm_scrolls(outer) {
                reveal = Reveal::new(None);
                reveal_cancelled = true;
            }
            // The pane is the rows' scroll column; its lane is kept, so a row's actions keep
            // their x when the plan grows long enough to scroll.
            let lane_width = outer.available_width();
            {
                let mut rows = outer.new_child(egui::UiBuilder::new()
                    .max_rect(outer.max_rect().with_min_x(outer.max_rect().left() + hang))
                    .layout(*outer.layout()));
                let ui = &mut rows;
                ui.set_max_width(lane_width - hang);
                let width = lane_width.min(ui.available_width()) - design::M;
                // Layout 11c A2c: every closed row is laid out before any row draws, so the
                // plan's shared column is known first, with no measure kept from the last pass
                // (the scenes and a group with warnings keep theirs until scope B). The laid rows
                // are kept by their inputs: a quiet frame lays out nothing.
                let row_width = ui.available_width();
                let rows = self.plan_rows(
                    ui,
                    book,
                    (session, plan, &projection),
                    (row_width, measured),
                    days,
                );
                let columns = rows.columns;
                let mut any = false;
                // The opened played row, on the frame the DM opened it, and the next fight's action.
                let mut opened_played = None;
                let laid_rows = !(rows.events.is_empty() && rows.fights.is_empty() && rows.groups.is_empty());
                self.plan_measured = measured && laid_rows;
                if measured && laid_rows {
                    any = true;
                    let (rows_closed, rows_next) = self.plan_region_rows(
                        ui,
                        book,
                        (plan, &projection, &rows, days),
                        (&mut acts, &mut toggle, &mut reveal),
                    );
                    closed += rows_closed;
                    next = rows_next;
                } else {
                let mut event_rows = rows.events.iter();
                for (at, event) in book.events.iter_mut().enumerate() {
                    if !event.is_in(session) { continue; }
                    let Some((row, place, _)) = event_rows.next() else { break };
                    let place = *place;
                    any = true;
                    let day = days.get(at).copied().unwrap_or(0);
                    let label = row.words.title.as_str();
                    let is_open = open_event.as_ref() == Some(&event.id);
                    let running = event.run.is_some() && event.run == plan.open_run;
                    let saved = planned_event_run(event, plan, running);
                    let ended = event.run.filter(|_| !running && saved.is_none());
                    let linked = projection.linked.contains(&event.id);
                    let row = ui.push_id(event.id.clone(), |ui| {
                        let listed = self.rewards.listed(&event.id);
                        let drawn = draw_plan_row(ui, row, Some(is_open), |ui| {
                            self.event_actions(ui, &mut acts, (plan, linked), (event, day, label, place));
                        });
                        if drawn.title.clicked() { toggle = Some(event.id.clone()); }
                        if running && drawn.summary.as_ref().is_some_and(egui::Response::clicked) {
                            self.wants_back_to_run = true;
                        }
                        closed += drawn.row;
                        let details_top = ui.cursor().top();
                        if is_open {
                            plan_details(ui, |ui| {
                            if running {
                                self.fight = None;
                                wrapped_words(ui, Type::Small, "This fight is running. Its final HP shows here when it ends.", c.muted);
                            } else if let Some(run) = event.run {
                                let link = ended.filter(|_| place == RewardsAt::Link).map(|_| listed);
                                if self.fight_block(ui, run, &event.id, plan, link) {
                                    acts.rewards_act = Some(RewardsAct::Add(event.id.clone(), run));
                                }
                                changed |= place_field(ui, event);
                            } else {
                                // A journey's checks come first, under its row; Title and Notes follow them.
                                if checks::shown(event) {
                                    let (act, taken, undo) = self.travel_block(ui, event, journey_start, plan, &in_plan, &mut book.length);
                                    if taken != checks::Taken::Nothing { length_taken = taken; }
                                    if let Some(act) = act { acts.travel_act = Some((event.id.clone(), act)); }
                                    if undo != TravelUndo::None { travel_undo = (undo, event.id.clone()); }
                                    design::gap(ui, design::Gap::XS);
                                }
                                changed |= event_fields(ui, event);
                                if let Some(action) = rest_ruling(ui, event) {
                                    ruling = Some((event.id.clone(), action));
                                }
                                if design::link(ui, "Timeline").clicked() { self.wants_event = Some(event.id.clone()); }
                            }
                            });
                        }
                        details_above += ui.cursor().top() - details_top;
                        closed += if is_open { plan_rule(ui) } else { drawn.rule };
                    });
                    // The opened row and the next row stay in reach.
                    if is_open && std::mem::take(&mut self.reveal_event) {
                        // A played fight's row waits for the next fight's action (review round 1).
                        if event.run.is_some() { opened_played = Some(row.response.rect); } else { row.response.scroll_to_me(Some(egui::Align::TOP)); }
                    }
                    reveal.after(ui, Some(&event.id), row.response.rect);
                }
                let mut fight_rows = rows.fights.iter();
                for (position, fight) in planned_fights().enumerate() {
                    let Some((row, occurrence)) = fight_rows.next() else { break };
                    self.removed_fight_message(ui, session, position);
                    if !folded {
                        closed_scenes += self.scene_rows(ui, (width, columns), (&scenes[position], &projection.model), position, shown, plan, &mut scene_act);
                    }
                    any = true;
                    let running = plan.open_run.is_some() && plan.open_encounter == Some(fight.name.as_str());
                    let open = self.open_planned.as_ref() == Some(&fight.path);
                    let fight_row = ui.push_id((&fight.path, *occurrence), |ui| {
                        let drawn = draw_plan_row(ui, row, Some(open), |ui| {
                            self.fight_actions(ui, &mut acts, plan, (position, fight));
                        });
                        if drawn.title.clicked() {
                            self.open_planned = (!open).then(|| fight.path.clone());
                        }
                        if running && drawn.summary.as_ref().is_some_and(egui::Response::clicked) {
                            self.wants_back_to_run = true;
                        }
                        closed += drawn.row;
                        let details_top = ui.cursor().top();
                        if open {
                            plan_details(ui, |ui| {
                                if read_encounter(ui).clicked() { self.wants_lookup = Some(fight.name.clone()); }
                            });
                        }
                        // The next fight's row, to the end of its own details, and what is above it.
                        if position == 0 {
                            next = Some(fold::Next {
                                bottom: closed + ui.cursor().top() - details_top,
                                details: details_above,
                                scenes: closed_scenes,
                            });
                        }
                        closed += if open { plan_rule(ui) } else { drawn.rule };
                    });
                    reveal.after(ui, None, fight_row.response.rect);
                }
                self.removed_fight_message(ui, session, projection.planned.len());
                if !folded {
                    closed_scenes += self.scene_rows(ui, (width, columns), (&scenes[projection.planned.len()], &projection.model), projection.planned.len(), shown, plan, &mut scene_act);
                }
                any |= scenes.iter().any(|scenes| !scenes.is_empty());
                let mut group_rows = rows.groups.iter();
                for (position, group) in planned_groups().enumerate() {
                    let Some(group_row) = group_rows.next() else { break };
                    let (action_width, edit_word, joins_fight) = (group_row.actions, group_row.edit_word, group_row.joins_fight);
                    any = true;
                    let row = ui.push_id(group.id, |ui| {
                        let mut read_group = false;
                        let actions = |ui: &mut egui::Ui| {
                            self.group_actions(ui, &mut acts, plan, (position, group, edit_word, joins_fight));
                        };
                        let top = ui.cursor().top();
                        // The row and the rule its block holds, as the fold counts them; `None` when
                        // the rule follows the row.
                        let mut in_block = None;
                        if let Some(row) = &group_row.row {
                            let drawn = draw_plan_row(ui, row, None, actions);
                            read_group = drawn.title.clicked();
                            in_block = (drawn.rule > 0.0).then_some(drawn.row + drawn.rule);
                            // Its own reasons (`No members can join.`), as warnings under its
                            // row, before its rule (review of PR 841).
                            let problems = group.own_problems();
                            if !problems.is_empty() {
                                plan_details(ui, |ui| {
                                    for reason in problems {
                                        design::gap(ui, design::Gap::XS);
                                        let fault = PlanFault::Check(as_sentence(reason));
                                        let _ = design::outcome::BodyMessage::<()>::from(FaultMessage::new(&fault)).compact().show(ui);
                                    }
                                });
                            }
                        } else {
                            let group_widths = [width - action_width - design::S, action_width];
                            plan_row(ui, (&group_widths, columns), icons::GROUP, |ui| {
                                if design::wrapped_link(ui, &group.name).clicked() { read_group = true; }
                            }, |ui| {
                                let summary = plan_summary_text(ui, group, ui.available_width());
                                // A flagged group shows every fact.
                                plan_group_summary(ui, &summary, usize::MAX);
                                // Name each member left out, its base and its line, beside Edit group, each
                                // in its own fault's tone; then the group's own reason, as a warning.
                                for warning in &group.warnings {
                                    design::gap(ui, design::Gap::XS);
                                    let _ = design::outcome::BodyMessage::<()>::from(FaultMessage::new(warning)).compact().show(ui);
                                }
                                for reason in group.own_problems() {
                                    design::gap(ui, design::Gap::XS);
                                    let fault = PlanFault::Check(as_sentence(reason));
                                    let _ = design::outcome::BodyMessage::<()>::from(FaultMessage::new(&fault)).compact().show(ui);
                                }
                            }, actions);
                        }
                        // B4 review F1: an unsaved draft refuses the open (the same value as the
                        // owner's guard reads); the press paints why under this line and leaves
                        // the draft's host where it is.
                        if read_group {
                            if plan.group_blocked.is_some() {
                                self.plan_group_refused = Some((self.selected, group.id));
                                if in_block.is_some() {
                                    // The row's block holds its rule; the reason goes before it.
                                    ui.ctx().request_discard("the refusal of a plan group shows before its rule");
                                }
                            } else {
                                self.plan_group_refused = None;
                                self.wants_group = Some(group.path.clone()); self.group_session = self.selected;
                            }
                        }
                        if let Some(reason) = plan.group_blocked
                            && self.plan_group_refused == Some((self.selected, group.id))
                        {
                            design::blocked_reason(ui, Some(reason));
                        }
                        closed += match in_block {
                            Some(height) => height,
                            None => ui.cursor().top() - top + plan_rule(ui),
                        };
                    });
                    if let Some((at, _, height)) = &mut acts.remove_group && *at == position { *height = row.response.rect.height(); }
                    reveal.after(ui, None, row.response.rect);
                }
                }
                if !any {
                    design::empty_state(ui, icons::SESSIONS, "Nothing planned yet",
                        "Use Add to plan a fight, a group, or an event.", |_| {});
                }
                reveal_opened_played(ui, opened_played, acts.next_action);
                reveal.finish(ui);
                self.scene_field(ui, index.as_deref(), shown, &mut scene_act);
                let drawn = ui.min_rect();
                outer.advance_cursor_after_rect(drawn);
            }
        });
        ui.advance_cursor_after_rect(pane_ui.min_rect().union(full));
        let RowActs {
            insert,
            remove,
            remove_group,
            remove_fight,
            delete_run,
            travel_act,
            mut rewards_act,
            ..
        } = acts;
        let pane = Some(room);
        if let Some(press) = paned.pressed {
            let (scene, rewards) = self.plan_footer_press(press, book, plan);
            if scene.is_some() {
                scene_act = scene;
            }
            if let Some((act, ticket)) = rewards {
                rewards_act = Some(act);
                rewards_ticket = ticket;
            }
        }
        if reveal_cancelled {
            self.row_reveal = None;
        }
        // The reveal ends when the row after the target is in view, or after at most
        // `REVEAL_DRAWS` tries (a draw that the footer discards to measure itself does not show).
        if let Some((_, draws, last)) = &mut self.row_reveal {
            *draws += 1;
            *last = pass;
            if reveal.shown || *draws >= REVEAL_DRAWS {
                self.row_reveal = None;
            }
        }
        if let Some(viewport) = pane {
            let measure = fold::Measure {
                closed,
                scenes: closed_scenes,
                details: details_above,
                next,
            };
            self.measure_fold(ui.ctx(), folded, viewport, measure, hold);
        }
        if let Some(target) = crate::markdown_view::take_note_link(ui.ctx()) {
            self.open_note_link(target);
        }
        if let Some(act) = scene_act {
            self.scene_act(act, session);
        }
        if let Some(act) = rewards_act {
            self.do_rewards_act((act, rewards_ticket), book, plan);
        }
        if let Some(remove) = remove_fight {
            self.remove_planned_fight(remove, book, plan, &projection.planned);
        }
        if let Some(remove) = remove_group {
            self.remove_planned_group(remove, book);
        }
        if changed {
            book.touch();
        }
        checks::note_taken(book, length_taken);
        if let Some((id, act)) = travel_act {
            self.row_reveal = Some((id.clone(), 0, ui.ctx().cumulative_pass_nr()));
            self.travel_act(book, session, &id, act, plan);
        }
        match travel_undo {
            (TravelUndo::Checks, id) => self.travel.done.undo(&id, book),
            (TravelUndo::None, _) => {}
        }
        if let Some((id, action)) = ruling {
            action.apply(book, &id);
            self.end_plan_add();
        }
        if let Some(saved) = delete_run {
            self.delete_plan_run(&saved, book);
        }
        if let Some(id) = toggle {
            self.toggle_plan_event(id);
        }
        if let Some(id) = remove {
            self.remove_plan_event(&id, book);
        }
        if let (Some(link), Some(session)) = (insert, session)
            && self.add_to_note(ui.ctx(), &link)
        {
            let body = self.draft.as_ref().map_or("", |draft| draft.body.as_str());
            self.mentions
                .insert(session, timeline::referenced_ids(body));
            self.end_plan_add();
        }
    }

    /// The scenes at one place among the plan's rows, with a removal's Undo at its row's place.
    /// Returns the rows' height when closed, without the messages, for the fold.
    #[allow(clippy::too_many_arguments)]
    fn scene_rows(
        &mut self,
        ui: &mut egui::Ui,
        width: (f32, design::PlanColumns),
        (scenes, model): (&[usize], &PlanModel),
        position: usize,
        shown: Shown,
        plan: &Plan<'_>,
        act: &mut Option<SceneAct>,
    ) -> f32 {
        let mut height = 0.0;
        for (at, line) in scenes.iter().enumerate() {
            let scene = model.scene(*line);
            let place = Place::Row((position, at));
            if shown.removed == Some(place) {
                self.removed_scene_message(ui, place, act);
            }
            height += scene_row(ui, width, scene, ((position, at), *line), plan, act);
            self.row_fault(ui, place, shown);
        }
        let end = Place::Row((position, scenes.len()));
        if shown.removed == Some(end) {
            self.removed_scene_message(ui, end, act);
        }
        // An Undo pressed at the end of these scenes says here what it could not do.
        self.row_fault(ui, end, shown);
        height
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

    /// The scene act's refusal under the place where the DM acted, while it shows there.
    #[allow(
        clippy::disallowed_methods,
        reason = "a refused scene act, said where the DM acted (Ban::Body)"
    )]
    fn row_fault(&self, ui: &mut egui::Ui, place: Place, shown: Shown) {
        if let Some(fault) = &self.scenes.fault
            && shown.fault == Some(place)
        {
            design::gap(ui, design::Gap::XS);
            let _ = design::outcome::row_fault(ui, &fault.what);
        }
    }

    /// `N scenes` on the heading's line, in room that is spare there, so it moves no row: a link
    /// with the expanded state, after the section caret that a folded section has, which opens
    /// the folded scenes in place and closes them again.
    fn fold_line(&mut self, ui: &mut egui::Ui, heading: egui::Rect, count: usize, folded: bool) {
        let rect = egui::Rect::from_min_max(
            egui::pos2(
                heading.right() + design::L,
                heading.center().y - design::XL / 2.0,
            ),
            egui::pos2(ui.max_rect().right(), heading.center().y + design::XL / 2.0),
        );
        let mut line = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        let words = design::count(count, "scene", "scenes");
        let caret = if folded { icons::NEXT } else { icons::DOWN };
        let link = glyph_link(&mut line, caret, colors().muted, Type::Body, &words);
        line.ctx()
            .accesskit_node_builder(link.id, |node| node.set_expanded(!folded));
        if link.clicked() {
            self.scenes.fold.open = Some(folded);
        }
    }

    /// What the pane and the rows measured in this frame, for the next frame's fold. Folded
    /// scenes keep the height they had when last shown.
    fn measure_fold(
        &mut self,
        ctx: &egui::Context,
        folded: bool,
        viewport: f32,
        measure: fold::Measure,
        hold: bool,
    ) {
        let fold = &mut self.scenes.fold;
        fold.viewport = Some(viewport);
        fold.measure = if folded {
            fold.measure.folded(measure)
        } else {
            measure
        };
        // The fold settles on the next frame without waiting for input.
        if !hold && fold.measured() != fold.overflow {
            ctx.request_repaint();
        }
    }

    /// The plan model for the selected note, built again only when what it reads changed.
    /// The plan's projection (`PlanProjection`), kept while its inputs' revisions stay.
    /// An event row's actions in its action place (layout 11c A2c-3: the same in place and in
    /// the plan's region): a paused run's menu and Resume, else More, Add rewards and Add to
    /// notes. Its acts go to `acts`.
    fn event_actions(
        &mut self,
        ui: &mut egui::Ui,
        acts: &mut RowActs,
        (plan, linked): (&Plan<'_>, bool),
        (event, day, label, place): (&crate::timeline::Event, u32, &str, RewardsAt),
    ) {
        let c = colors();
        let running = event.run.is_some() && event.run == plan.open_run;
        let saved = planned_event_run(event, plan, running);
        let ended = event.run.filter(|_| !running && saved.is_none());
        let saved_fight = plan.fights.iter().find(|fight| {
            fight
                .saved
                .as_ref()
                .is_some_and(|saved| saved.id == event.run && event.run.is_some())
        });
        let listed = self.rewards.listed(&event.id);
        let on_row = ended.is_some() && place == RewardsAt::Row;

        if let Some(saved) = saved {
            design::row_menu(ui, label, |ui| {
                if let Some(fight) = saved_fight
                    && Action::quiet("Start a new run")
                        .blocked(plan.blocked)
                        .show(ui)
                        .clicked()
                {
                    self.wants_fight = Some(fight.path.clone());
                    ui.close();
                }
                if Action::quiet("Delete the saved run")
                    .blocked(plan.blocked)
                    .show(ui)
                    .clicked()
                {
                    acts.delete_run = Some(saved.clone());
                    ui.close();
                }
                if Action::quiet("Remove")
                    .name("Remove this event")
                    .show(ui)
                    .clicked()
                {
                    acts.remove = Some(event.id.clone());
                    ui.close();
                }
            });
            let action = if std::mem::take(&mut acts.primary_available) {
                Action::primary("Resume")
            } else {
                Action::secondary("Resume")
            };
            if action
                .icon(icons::RUN)
                .name(&format!("Resume: {label}"))
                .blocked(plan.blocked)
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                self.wants_saved_run = Some(saved.path.clone());
            }
        } else {
            // Overflow is always at the row's right edge.
            let more = design::icon_button(ui, icons::EXPAND, &format!("More actions for {label}"));
            design::menu_popup(ui, &more).show(|ui| {
                if checks::offered(event, plan.tables)
                    && Action::quiet("Roll for encounters").show(ui).clicked()
                {
                    acts.travel_act = Some((event.id.clone(), checks::Act::Start));
                    ui.close();
                }
                if Action::quiet("Remove")
                    .name("Remove this event")
                    .blocked_if(running, "This run is open. Close it first.")
                    .show(ui)
                    .clicked()
                {
                    acts.remove = Some(event.id.clone());
                    ui.close();
                }
                // A note under its choice, at its words (M-P0.27, review of PR 780).
                design::choice_note(
                    ui,
                    false,
                    running.then_some("This run is open. Close it first."),
                );
                // Last, so no line of today's menu moves.
                if let Some(run) = ended.filter(|_| place == RewardsAt::Menu) {
                    if listed {
                        design::status(
                            ui,
                            Status {
                                icon: icons::OK,
                                word: "In notes",
                                color: c.document.saved,
                            },
                        );
                    } else if Action::quiet(ADD_REWARDS).show(ui).clicked() {
                        acts.rewards_act = Some(RewardsAct::Add(event.id.clone(), run));
                        ui.close();
                    }
                }
            });
            if let Some(run) = ended.filter(|_| on_row) {
                if listed {
                    design::gap(ui, design::Gap::S);
                    design::status(
                        ui,
                        Status {
                            icon: icons::OK,
                            word: "In notes",
                            color: c.document.saved,
                        },
                    );
                } else if Action::quiet(ADD_REWARDS)
                    .name(&format!("Add rewards: {label}"))
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    acts.rewards_act = Some(RewardsAct::Add(event.id.clone(), run));
                }
            }
            if !running && event.run.is_none() {
                if linked {
                    design::gap(ui, design::Gap::S);
                    design::status(
                        ui,
                        Status {
                            icon: icons::OK,
                            word: "In notes",
                            color: c.document.saved,
                        },
                    );
                } else if Action::quiet("Add to notes")
                    .name(&format!("Add to notes: {label}"))
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    acts.insert = Some(timeline::reference(event, day));
                }
            }
        }
    }

    /// A planned fight's actions (layout 11c A2c-3: the same in place and in the plan's
    /// region): its menu, then Resume or Start fight. The first fight's action is the next one.
    fn fight_actions(
        &mut self,
        ui: &mut egui::Ui,
        acts: &mut RowActs,
        plan: &Plan<'_>,
        (position, fight): (usize, &PlanFight),
    ) {
        let running = plan.open_run.is_some() && plan.open_encounter == Some(fight.name.as_str());
        let saved = planned_saved_run(fight, plan.fights, plan.saved_runs);

        design::row_menu(ui, &fight.name, |ui| {
            if Action::quiet("Remove from tonight").show(ui).clicked() {
                acts.remove_fight = Some((position, fight.name.clone()));
                ui.close();
            }
            if saved.is_some() {
                if Action::quiet("Start a new run")
                    .blocked(plan.blocked)
                    .show(ui)
                    .clicked()
                {
                    self.wants_fight = Some(fight.path.clone());
                    ui.close();
                }
                if Action::quiet("Delete the saved run")
                    .blocked(plan.blocked)
                    .show(ui)
                    .clicked()
                {
                    acts.delete_run = saved.cloned();
                    ui.close();
                }
            }
            // Last, so no line of today's menu moves (W1-D1).
            if edit_encounter_line(ui, plan) {
                self.wants_edit_encounter = Some(fight.id);
                ui.close();
            }
        });
        if running {
            // The run strip owns Back to run.
        } else if let Some(saved) = saved {
            let action = if std::mem::take(&mut acts.primary_available) {
                Action::primary("Resume")
            } else {
                Action::secondary("Resume")
            };
            let shown = action
                .icon(icons::RUN)
                .name(&format!("Resume: {}", fight.name))
                .density(Density::Read)
                .blocked(plan.blocked)
                .show(ui);
            if position == 0 {
                acts.next_action = Some(shown.rect);
            }
            if shown.clicked() {
                self.wants_saved_run = Some(saved.path.clone());
            }
        } else {
            let action = if std::mem::take(&mut acts.primary_available) {
                Action::primary("Start fight")
            } else {
                Action::secondary("Start fight")
            };
            let shown = action
                .icon(icons::RUN)
                .name(&format!("Start fight: {}", fight.name))
                .density(Density::Read)
                .blocked(plan.blocked)
                .show(ui);
            if position == 0 {
                acts.next_action = Some(shown.rect);
            }
            if shown.clicked() {
                self.wants_fight = Some(fight.path.clone());
            }
        }
    }

    /// A planned group's actions (layout 11c A2c-3: the same in place and in the plan's region):
    /// its menu, then Add to fight while a fight is open, else Edit.
    fn group_actions(
        &mut self,
        ui: &mut egui::Ui,
        acts: &mut RowActs,
        plan: &Plan<'_>,
        (position, group, edit_word, joins_fight): (
            usize,
            &crate::encounter_run::RunGroup,
            &str,
            bool,
        ),
    ) {
        design::row_menu(ui, &group.name, |ui| {
            if Action::quiet("Remove from tonight")
                .name(&format!("Remove group {} from the plan", group.name))
                .blocked(plan.group_blocked)
                .show(ui)
                .clicked()
            {
                acts.remove_group = Some((position, group.name.clone(), 0.0));
                ui.close();
            }
            // Add to fight takes the action place; the repair its faults name stays one step away.
            if joins_fight
                && !group.warnings.is_empty()
                && Action::quiet("Edit group")
                    .name(&format!("Edit group {}", group.name))
                    .blocked(plan.group_blocked)
                    .show(ui)
                    .clicked()
            {
                self.wants_group = Some(group.path.clone());
                self.group_session = self.selected;
                ui.close();
            }
        });
        if joins_fight {
            if Action::quiet(ADD_TO_FIGHT)
                .icon(icons::ADD)
                .name(&format!("Add {} to fight", group.name))
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                self.wants_arrivals = true;
            }
        } else if Action::quiet(edit_word)
            .name(&format!("Edit group {}", group.name))
            .density(Density::Read)
            .blocked(plan.group_blocked)
            .show(ui)
            .clicked()
        {
            self.wants_group = Some(group.path.clone());
            self.group_session = self.selected;
        }
    }

    /// The plan's closed rows in its measured region (layout 11c A2c-3): each laid row's block,
    /// drawn from the region's mask before paint, with the same parts and actions as in place.
    /// An added row's reveal is the region's `Navigation::Show` of the row after it, or of the
    /// row itself when it is the last (M0500). Returns the fold's sums: the closed rows' height,
    /// and the next fight's row with what is above it.
    fn plan_region_rows(
        &mut self,
        ui: &mut egui::Ui,
        book: &Book,
        (plan, projection, rows, days): (&Plan<'_>, &PlanProjection, &KeptRows, &[u32]),
        (acts, toggle, reveal): (&mut RowActs, &mut Option<String>, &mut Reveal),
    ) -> (f32, Option<fold::Next>) {
        let (events, fights) = (rows.events.len(), rows.fights.len());
        let row_at = |at: usize| -> &KeptRow {
            if at < events {
                &rows.events[at].0
            } else if at < events + fights {
                &rows.fights[at - events].0
            } else {
                rows.groups[at - events - fights]
                    .row
                    .as_ref()
                    .expect("a measured body lays every group by its recipe")
            }
        };
        let blocks = rows.blocks.len();
        let mut region = self
            .plan_region
            .take()
            .unwrap_or_else(|| design::RegionState::new("session_plan"));
        // The DM's press is input to the region's column, which cancels its older requests: the
        // reveal is asked in a pass with no press, release or held button (the in-place reveal
        // waits as long, up to `REVEAL_DRAWS` draws).
        let pointer_still = ui.input(|input| {
            !input.pointer.any_down()
                && !input.pointer.any_pressed()
                && !input.pointer.any_released()
        });
        if let Some(target) = reveal.target.as_deref()
            && pointer_still
            && let Some(at) = rows
                .events
                .iter()
                .position(|(_, _, index)| book.events[*index].id == target)
        {
            region.navigate(design::Navigation::Show(
                row_at((at + 1).min(blocks - 1)).key,
            ));
            reveal.shown = true;
        }
        let mut read_group = None;
        let _ = design::BlockRegion::new("session_plan", &mut region, &rows.blocks).show_laid(
            ui,
            |at| &row_at(at).laid,
            |l, at| {
                let row = row_at(at);
                if at < events {
                    let (_, place, index) = &rows.events[at];
                    let event = &book.events[*index];
                    let day = days.get(*index).copied().unwrap_or(0);
                    let running = event.run.is_some() && event.run == plan.open_run;
                    let linked = projection.linked.contains(&event.id);
                    let drawn = draw_row_parts(l, row, Some(false), |ui| {
                        self.event_actions(
                            ui,
                            acts,
                            (plan, linked),
                            (event, day, &row.words.title, *place),
                        );
                    });
                    if drawn.title.clicked() {
                        *toggle = Some(event.id.clone());
                    }
                    if running && drawn.summary.as_ref().is_some_and(egui::Response::clicked) {
                        self.wants_back_to_run = true;
                    }
                } else if at < events + fights {
                    let position = at - events;
                    let fight = &plan.fights[projection.planned[position]];
                    let running =
                        plan.open_run.is_some() && plan.open_encounter == Some(fight.name.as_str());
                    let drawn = draw_row_parts(l, row, Some(false), |ui| {
                        self.fight_actions(ui, acts, plan, (position, fight));
                    });
                    if drawn.title.clicked() {
                        self.open_planned = Some(fight.path.clone());
                    }
                    if running && drawn.summary.as_ref().is_some_and(egui::Response::clicked) {
                        self.wants_back_to_run = true;
                    }
                } else {
                    let position = at - events - fights;
                    let group_row = &rows.groups[position];
                    let group = &plan.groups[projection.planned_groups[position]];
                    let drawn = draw_row_parts(l, row, None, |ui| {
                        self.group_actions(
                            ui,
                            acts,
                            plan,
                            (position, group, group_row.edit_word, group_row.joins_fight),
                        );
                    });
                    if drawn.title.clicked() {
                        read_group = Some((group.id, group.path.clone()));
                    }
                }
            },
        );
        self.plan_region = Some(region);
        // B4 review F1: an unsaved draft refuses the open; the refusal shows under the row, so
        // the next pass draws the rows in place.
        if let Some((id, path)) = read_group {
            if plan.group_blocked.is_some() {
                self.plan_group_refused = Some((self.selected, id));
                ui.ctx()
                    .request_discard("the refusal of a plan group shows under its row");
            } else {
                self.plan_group_refused = None;
                self.wants_group = Some(path);
                self.group_session = self.selected;
            }
        }
        if let Some((position, _, height)) = &mut acts.remove_group
            && let Some(row) = rows
                .groups
                .get(*position)
                .and_then(|group| group.row.as_ref())
        {
            *height = row.laid.height();
        }
        // The fold's sums, kept with the rows.
        let (closed, next) = rows.fold;
        (
            closed,
            next.map(|bottom| fold::Next {
                bottom,
                details: 0.0,
                scenes: 0.0,
            }),
        )
    }

    /// The plan's closed rows, laid out by their recipe before any row draws, with the plan's
    /// shared column (layout 11c A2c): built again only when their inputs change (`RowsKey`),
    /// so a quiet frame measures and lays out nothing (review of PR 841).
    #[allow(clippy::too_many_arguments)]
    fn plan_rows(
        &mut self,
        ui: &egui::Ui,
        book: &Book,
        (session, plan, projection): (Option<Uuid>, &Plan<'_>, &PlanProjection),
        (row_width, measured): (f32, bool),
        days: &[u32],
    ) -> Arc<KeptRows> {
        let fight_days = self.planned_days(book, session, plan, &projection.planned);
        let plan_days = self.plan_lines_seen.revision();
        let key = RowsKey {
            projection: self.plan_projection.revision(),
            session,
            draft: self
                .draft
                .as_ref()
                .map_or(0, crate::sessions::Draft::revision),
            book: book.revision(),
            fights: plan.fights_revision,
            saved: plan.saved_runs_revision,
            days: plan_days,
            open_event: self.open_event.clone(),
            open_planned: self.open_planned.clone(),
            open_run: plan.open_run.map(|run| (run, plan.open_state.clone())),
            arrivals_open: plan.arrivals_open,
            refused: self
                .plan_group_refused
                .filter(|(at, _)| plan.group_blocked.is_some() && *at == session),
            env: (
                row_width,
                design::theme::font_generation(ui.ctx()),
                ui.ctx().pixels_per_point(),
                design::theme::atlas_epoch(ui.ctx()),
            ),
            legacy: design::action_rows::MeasuredColumn::kept(ui, PLAN_COLUMNS),
            measured,
        };
        self.plan_rows.reached += 1;
        if let Some((kept, rows)) = &self.plan_rows.kept
            && *kept == key
        {
            return Arc::clone(rows);
        }
        self.plan_rows.builds += 1;
        // The action places' widths: the widest of a row's actions with their alternates, and
        // Add rewards (measured here, behind the key, so a quiet frame measures none).
        let plain_actions = [
            Action::primary("Start fight").icon(icons::RUN),
            Action::primary("Resume").icon(icons::RUN),
            Action::quiet("Add to notes"),
            Action::secondary("Arrivals").icon(icons::GROUP),
        ]
        .into_iter()
        .map(|action| action.density(Density::Read).natural_width(ui))
        .fold(0.0, f32::max)
            + Density::Read.control()
            + design::S;
        let rewards_width = Action::quiet(ADD_REWARDS)
            .density(Density::Read)
            .natural_width(ui)
            + Density::Read.control()
            + design::S;
        let room = design::PlanRow::summary_room(row_width);
        let tail = if measured {
            design::theme::LINE_GAP
        } else {
            0.0
        };
        let mut events = Vec::new();
        for (at, event) in book.events.iter().enumerate() {
            if !event.is_in(session) {
                continue;
            }
            let label = event.label(days.get(at).copied().unwrap_or(0));
            let is_open = self.open_event.as_ref() == Some(&event.id);
            let running = event.run.is_some() && event.run == plan.open_run;
            let saved = planned_event_run(event, plan, running);
            // A fight that ended: its rewards (F7), on the row when the title keeps its lines.
            let ended = event.run.filter(|_| !running && saved.is_none());
            let lead = if is_open { icons::DOWN } else { icons::NEXT };
            let place = self
                .rewards
                .fits(ui, &label, lead, row_width, rewards_width);
            let actions = if ended.is_some() && place == RewardsAt::Row {
                rewards_width
            } else if event.run.is_some() && (running || saved.is_none()) {
                Density::Read.control()
            } else {
                plain_actions
            };
            let summary = if running {
                Some(RowPart::run_link(ui, &plan.open_state))
            } else if let Some(saved) = saved {
                Some(RowPart::saved(ui, saved))
            } else if !(is_open && event.kind == Kind::LongRest) {
                let summary = if event.run.is_some() {
                    timeline::played_fight_summary(event, running)
                } else {
                    event.summary(None)
                };
                let warning = event.time_warning().filter(|_| event.run.is_none());
                plan_event_summary_job(
                    ui,
                    &summary,
                    (event.kind == Kind::LongRest).then_some(event.restores),
                    warning.as_deref(),
                )
                .map(|job| {
                    RowPart::words(
                        job,
                        match warning {
                            Some(_) => PlanHover::Text(timeline::TIME_REPAIR.to_owned()),
                            None => PlanHover::None,
                        },
                    )
                })
            } else {
                None
            };
            let words = RowWords {
                lead,
                title: label,
                summary,
                actions,
            };
            events.push((
                egui::Id::new(("plan_event", &event.id)),
                !is_open,
                words,
                (place, at),
            ));
        }
        let mut fights = Vec::new();
        for (position, fight) in projection
            .planned
            .iter()
            .map(|&at| &plan.fights[at])
            .enumerate()
        {
            let running =
                plan.open_run.is_some() && plan.open_encounter == Some(fight.name.as_str());
            let saved = planned_saved_run(fight, plan.fights, plan.saved_runs);
            let open = self.open_planned.as_ref() == Some(&fight.path);
            let summary = if running {
                RowPart::run_link(ui, &plan.open_state)
            } else if let Some(saved) = saved {
                RowPart::saved(ui, saved)
            } else {
                let day = fight_days.at.get(position).cloned().flatten();
                kept_plan_line(
                    ui,
                    (&mut self.plan_lines, room),
                    (session, position, fight.id),
                    fight,
                    day.as_deref(),
                    plan_days,
                )
                .part(ui, room)
            };
            let words = RowWords {
                lead: if open { icons::DOWN } else { icons::NEXT },
                title: fight.name.clone(),
                summary: Some(summary),
                actions: if running {
                    Density::Read.control()
                } else {
                    plain_actions
                },
            };
            let occurrence = projection.planned[..position]
                .iter()
                .filter(|&&at| plan.fights[at].path == fight.path)
                .count();
            fights.push((
                egui::Id::new(("plan_fight", &fight.path, occurrence)),
                !open,
                words,
                occurrence,
            ));
        }
        let mut groups = Vec::new();
        for group in projection.planned_groups.iter().map(|&at| &plan.groups[at]) {
            let edit_word = if group.warnings.is_empty() {
                "Edit"
            } else {
                "Edit group"
            };
            // With a fight open, the run's words and plus icon: the group joins it through
            // Arrivals. A group none of whose members can join has only its repair to offer.
            let joins_fight = plan.arrivals_open && !group.members.is_empty();
            let group_action = if joins_fight {
                Action::quiet(ADD_TO_FIGHT).icon(icons::ADD)
            } else {
                Action::quiet(edit_word)
            };
            let actions = group_action.density(Density::Read).natural_width(ui)
                + Density::Read.control()
                + design::S;
            // A group with member faults names them in messages measured by their own part, so
            // it keeps today's row until scope B. Its own reasons follow its row (review of PR
            // 841), so the rule follows them.
            let words = group.warnings.is_empty().then(|| RowWords {
                lead: icons::GROUP,
                title: group.name.clone(),
                summary: Some(plan_group_part(
                    ui,
                    &plan_summary_text(ui, group, room),
                    3,
                    room,
                )),
                actions,
            });
            let refused = key.refused.is_some_and(|(_, id)| id == group.id);
            let rule = !refused && group.own_problems().is_empty();
            groups.push((group.id, rule, words, actions, edit_word, joins_fight));
        }
        let columns = design::PlanColumns::of(
            events
                .iter()
                .map(|row| &row.2)
                .chain(fights.iter().map(|row| &row.2))
                .chain(groups.iter().filter_map(|row| row.2.as_ref()))
                .map(|words| words.facts(ui, row_width))
                .chain([key.legacy]),
        );
        let lay = |key: egui::Id, rule: bool, words: RowWords| {
            let part = words.part();
            let row = words.row(key, part.as_slice(), (rule, tail));
            let laid = row.lay(ui, row_width, columns);
            KeptRow {
                key,
                #[cfg(any(test, feature = "test-support"))]
                facts: row.facts(ui, row_width).0,
                #[cfg(any(test, feature = "test-support"))]
                in_column: row.in_column(ui, row_width, columns),
                laid,
                words,
            }
        };
        let mut kept = KeptRows {
            events: events
                .into_iter()
                .map(|(key, rule, words, (place, at))| (lay(key, rule, words), place, at))
                .collect(),
            fights: fights
                .into_iter()
                .map(|(key, rule, words, occurrence)| (lay(key, rule, words), occurrence))
                .collect(),
            groups: groups
                .into_iter()
                .map(
                    |(id, rule, words, actions, edit_word, joins_fight)| GroupRow {
                        row: words.map(|words| lay(egui::Id::new(("plan_group", id)), rule, words)),
                        actions,
                        edit_word,
                        joins_fight,
                    },
                )
                .collect(),
            columns,
            blocks: design::memo::Revised::new(Vec::new()),
            fold: (0.0, None),
        };
        let blocks = kept
            .events
            .iter()
            .map(|(row, ..)| row)
            .chain(kept.fights.iter().map(|(row, _)| row))
            .chain(kept.groups.iter().filter_map(|group| group.row.as_ref()))
            .map(|row| row.laid.block())
            .collect();
        kept.blocks.set(blocks);
        // Each block is its row with the column's gap, then its rule with the gap after it.
        let rule = design::S + 1.0 + design::theme::LINE_GAP;
        let mut closed = 0.0;
        let mut next = None;
        for (at, row) in kept
            .events
            .iter()
            .map(|(row, ..)| row)
            .chain(kept.fights.iter().map(|(row, _)| row))
            .chain(kept.groups.iter().filter_map(|group| group.row.as_ref()))
            .enumerate()
        {
            let height = row.laid.height();
            if at == kept.events.len() && !kept.fights.is_empty() {
                next = Some(closed + height - rule);
            }
            closed += height;
        }
        kept.fold = (closed, next);
        let rows = Arc::new(kept);
        self.plan_rows.kept = Some((key, Arc::clone(&rows)));
        rows
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

    /// `Removed “…” from tonight.` with `Undo remove …`, at `place`, while the note is as the
    /// removal left it. A finished change: the success tone. Its Undo is the note's, held with
    /// the scene's data (`Scenes::removed`), so it is a placed result, not a kept line.
    #[allow(
        clippy::disallowed_methods,
        reason = "a removed scene: its result and its Undo where its row was (Ban::Body)"
    )]
    fn removed_scene_message(&self, ui: &mut egui::Ui, place: Place, act: &mut Option<SceneAct>) {
        let Some((_, words, undo)) = &self.scenes.removed else {
            return;
        };
        if self.draft.as_ref().is_none_or(|draft| !undo.live(draft)) {
            return;
        }
        design::gap(ui, design::Gap::S);
        if let Some(pressed) = PlacedResult::new(
            crate::sentence!("Removed “{}” from tonight.", words),
            UndoStep::of(Verb::Remove, words.as_str()).pressed_as(SceneAct::UndoRemove(place)),
        )
        .show(ui)
        {
            *act = Some(pressed);
        }
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

    /// `Scene`'s field at the end of the plan. Enter adds a scene and keeps the field open for
    /// the next; Escape clears and closes it. What the field did shows directly under it while
    /// it is open, so the field never moves for its own result.
    fn scene_field(
        &mut self,
        ui: &mut egui::Ui,
        index: Option<&ContentIndex>,
        shown: Shown,
        act: &mut Option<SceneAct>,
    ) {
        // The field's state leaves `scenes` while the form draws, so the result's room can draw
        // the plan's result (R30); it goes back unless the field closed.
        let Some(mut state) = self.scenes.field.take() else {
            self.scenes.typing = false;
            return;
        };
        design::gap(ui, design::Gap::S);
        let id = crate::field_id!("plan_scene").once(crate::layout::Once::CreateForm);
        let mut undo = false;
        let result = design::add_field(
            id,
            "Plan a scene",
            "Plan a scene: words, a person or a place",
        )
        .actions(crate::word!("Add"), crate::word!("Cancel"))
        .suggest(|words| {
            let Some(index) = index else {
                return Vec::new();
            };
            let rows = index
                .search(&words.trim().to_lowercase())
                .into_iter()
                .filter(|at| is_world(index, *at))
                .take(4)
                .map(|at| {
                    (
                        index.items[at].entry.name.clone(),
                        world_detail(index, at),
                        index.link_to(at),
                    )
                })
                .collect();
            vec![design::Suggestions {
                label: "World",
                rows,
            }]
        })
        .result_room(|ui| self.field_result(ui, shown, &mut undo))
        .show(ui, &mut state);
        let typed = state.words.trim().to_owned();
        let emptied = state.words.is_empty();
        self.scenes.typing = result.field.has_focus();
        if undo {
            *act = Some(SceneAct::UndoAdd);
        }
        if let Some(link) = result.chosen {
            *act = Some(SceneAct::Add(link));
        } else if result.enter && !typed.is_empty() {
            *act = Some(SceneAct::Add(typed));
        }
        if !(result.cancel && emptied) {
            self.scenes.field = Some(state);
        }
    }

    /// The Scene field's fault, and `Scene added to tonight.` with `Undo add scene`, in the room
    /// the open field keeps for its result (R30), so neither the field nor `Add` moves for it.
    /// `undo`: Undo was pressed. Its Undo is the note's (`Scenes::added`), as the footer's
    /// line of the same result says it (`scene_messages`).
    #[allow(
        clippy::disallowed_methods,
        reason = "an added scene: its result and its Undo under the open Scene field (Ban::Body)"
    )]
    fn field_result(&self, ui: &mut egui::Ui, shown: Shown, undo: &mut bool) {
        self.row_fault(ui, Place::Field, shown);
        if shown.added == Some(Place::Field)
            && let Some(added) = &self.scenes.added
            && self.draft.as_ref().is_some_and(|draft| added.live(draft))
        {
            design::gap(ui, design::Gap::XS);
            *undo |= PlacedResult::new(
                crate::sentence!("Scene added to tonight."),
                UndoStep::of(Verb::Add, "scene").pressed_as(()),
            )
            .show(ui)
            .is_some();
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

    /// `Removed “…” from tonight.` with `Undo remove …`, at the removed fight's place, while
    /// the note is as the removal left it. Its Undo is the note's, held with the fight's data
    /// (`removed_fight`), so it is a placed result, not a kept line.
    #[allow(
        clippy::disallowed_methods,
        reason = "a removed fight: its result and its Undo where its row was (Ban::Body)"
    )]
    fn removed_fight_message(&mut self, ui: &mut egui::Ui, session: Option<Uuid>, position: usize) {
        let Some((id, at, name, undo)) = &self.removed_fight else {
            return;
        };
        if *id != session
            || *at != position
            || self.draft.as_ref().is_none_or(|draft| !undo.live(draft))
        {
            return;
        }
        design::gap(ui, design::Gap::S);
        if PlacedResult::new(
            crate::sentence!("Removed “{}” from tonight.", name),
            UndoStep::of(Verb::Remove, name.as_str()).pressed_as(()),
        )
        .show(ui)
        .is_some()
        {
            if let Some(draft) = &mut self.draft {
                let _ = undo.undo(draft);
            }
            self.removed_fight = None;
            self.plan_notice_state = None;
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

    /// Add a fight to the plan: one the campaign has, or a new one.
    fn fight_menu(&mut self, ui: &mut egui::Ui, plan: &Plan<'_>) {
        let names: Vec<&str> = plan
            .fights
            .iter()
            .map(|fight| fight.name.as_str())
            .collect();
        let menu = PlanMenu {
            word: "Fight",
            icon: icons::ENCOUNTERS,
            name: "Add a fight",
            new: "New encounter",
            blocked: plan.blocked,
            find: "Find a fight",
            field: crate::field_id!("find_a_fight").once(design::Once::PageSearch),
            hint: "Find a fight…",
        };
        match plan_menu(
            ui,
            &menu,
            &names,
            &mut self.fight_search,
            &mut self.focus_fight_search,
        ) {
            Some(MenuChoice::New) => self.wants_new_fight = true,
            Some(MenuChoice::Named(name)) => self.add_to_plan(&name),
            None => {}
        }
    }

    /// Add a group to the plan: one the campaign has, or a new one assembled here.
    fn group_menu(&mut self, ui: &mut egui::Ui, plan: &Plan<'_>) {
        let names: Vec<&str> = plan
            .groups
            .iter()
            .map(|group| group.name.as_str())
            .collect();
        let menu = PlanMenu {
            word: "Group",
            icon: icons::GROUP,
            name: "Add a group",
            new: "New group",
            blocked: plan.group_blocked,
            find: "Find a group",
            field: crate::field_id!("find_a_group").once(design::Once::PageSearch),
            hint: "Find a group…",
        };
        match plan_menu(
            ui,
            &menu,
            &names,
            &mut self.group_search,
            &mut self.focus_group_search,
        ) {
            Some(MenuChoice::New) => {
                self.wants_new_group = true;
                self.group_session = self.selected;
            }
            Some(MenuChoice::Named(name)) => self.add_to_plan(&name),
            None => {}
        }
    }

    /// How a fight ended, in place: final HP and the log, with no run to reopen. `rewards` is
    /// whether its rewards are in the note, when its row has no room for them (F7): the last
    /// link is then `Add rewards`, or the words `In notes`. Returns whether the DM pressed it.
    #[allow(
        clippy::disallowed_methods,
        reason = "the open played fight’s fault in its details (Ban::Body)"
    )]
    fn fight_block(
        &mut self,
        ui: &mut egui::Ui,
        run: Uuid,
        event_id: &str,
        plan: &Plan<'_>,
        rewards: Option<bool>,
    ) -> bool {
        let mut add_rewards = false;
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
        let Some((_, _, result, revision)) = &self.fight else {
            return false;
        };
        match result {
            // R41: the folder's cause in Encounters' words and tone.
            Err(FightFault::Folder(folder)) => {
                let fault = PlanFault::Toned(as_sentence(&folder.message()), folder.severity());
                let _ =
                    design::outcome::BodyMessage::<()>::from(FaultMessage::new(&fault)).show(ui);
            }
            Err(FightFault::Other(error)) => {
                let words = as_sentence(&format!("This fight’s run cannot be read. {error}"));
                let _ = design::outcome::BodyMessage::<()>::from(Notice::new(words)).show(ui);
            }
            Ok(summary) => {
                let known = |n: Option<i64>| n.map_or_else(|| "?".to_owned(), |n| n.to_string());
                let mut cells = Vec::new();
                let mut widest = 0.0_f32;
                for row in in_party_order(&summary.rows, plan.party) {
                    let first = row.name.split_whitespace().next().unwrap_or(&row.name);
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
                    let mut words = egui::text::LayoutJob::default();
                    let format = |color| egui::TextFormat {
                        font_id: Type::Small.font(ui),
                        color,
                        ..Default::default()
                    };
                    words.append(first, 0.0, format(colors().muted));
                    words.append(
                        &format!(" {hp}"),
                        0.0,
                        format(design::hp_color(design::hp_state(row.hp, row.max_hp))),
                    );
                    if row.temporary_hp > 0 {
                        words.append(
                            &format!(" {}", design::temp_hp_text(row.temporary_hp)),
                            0.0,
                            format(colors().hp.temp),
                        );
                    }
                    widest = widest.max(ui.painter().layout_job(words.clone()).size().x);
                    cells.push((words, detail));
                }
                let across = if 3.0 * widest + 2.0 * design::S <= ui.available_width() {
                    3
                } else {
                    2
                };
                let cell_width =
                    (ui.available_width() - (across - 1) as f32 * design::S) / across as f32;
                egui::Grid::new(("played_party_hp", run))
                    .num_columns(across)
                    .min_col_width(cell_width)
                    .max_col_width(cell_width)
                    .min_row_height(Type::Small.line())
                    .spacing(egui::vec2(design::S, design::XS))
                    .show(ui, |ui| {
                        for (at, (words, detail)) in cells.into_iter().enumerate() {
                            ui.add(egui::Label::new(words).wrap()).on_hover_text(detail);
                            if (at + 1) % across == 0 {
                                ui.end_row();
                            }
                        }
                    });
                ui.horizontal(|ui| {
                    // Each word starts where the text does; a short word's target is wider
                    // than the word, never padded with spaces. One blank on each side of `·`,
                    // at every size, also where the page's columns give the plan a wider gap (X48).
                    ui.spacing_mut().item_spacing.x = link_gap(ui);
                    if design::link(ui, "Log").clicked() {
                        self.show_fight_log = !self.show_fight_log;
                    }
                    ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                    if design::link(ui, "Whole run").clicked() {
                        self.wants_saved_run = path;
                    }
                    ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                    if design::link(ui, "Timeline").clicked() {
                        self.wants_event = Some(event_id.to_owned());
                    }
                    if let Some(listed) = rewards {
                        ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                        if listed {
                            ui.label(Type::Body.rich(ui, "In notes").color(colors().muted));
                        } else if design::link(ui, ADD_REWARDS).clicked() {
                            add_rewards = true;
                        }
                    }
                });
                if self.show_fight_log {
                    // A capped block, not a scroll column of its own: the plan's pane is the
                    // scroll column (`Body::Content`).
                    let view = log_view(ui, run, *revision, &summary.log);
                    for (row, whole) in &view.rows {
                        let shown = ui.label(row.clone());
                        if row.elided {
                            shown.on_hover_text(whole);
                        }
                    }
                    if let Some(notice) = &view.notice {
                        ui.label(notice.clone());
                    }
                }
            }
        }
        add_rewards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::Draft;
    use std::{fs, path::PathBuf};

    /// iter-26-B FI7, FL2: at every width, no summary line starts or ends with `·`, and no
    /// member breaks inside its `N × name`.
    #[test]
    fn plan_summaries_never_start_a_line_with_a_separator() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = Type::Small.font(ui);
                let summary = "5 party · 2 × Cinder Wolf, 1 × Ember Cultist · Changed";
                let mut cuts = 0;
                for width in [90.0, 120.0, 140.0, 180.0, 220.0, 260.0, 330.0, 600.0] {
                    let text = summary
                        .split(" · ")
                        .map(|part| {
                            part.split(", ")
                                .map(|member| member.replace(' ', "\u{a0}"))
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .collect::<Vec<_>>()
                        .join(FACT_SEPARATOR);
                    let kept = without_loose_separators(ui, text.clone(), &font, width);
                    cuts += usize::from(kept != text);
                    let galley = ui
                        .painter()
                        .layout(kept, font.clone(), colors().muted, width);
                    let rows: Vec<String> = galley
                        .rows
                        .iter()
                        .map(|row| row.glyphs.iter().map(|glyph| glyph.chr).collect())
                        .collect();
                    for row in &rows {
                        assert!(!row.trim().starts_with('·'), "{width}: {rows:?}");
                        assert!(!row.trim_end().ends_with('·'), "{width}: {rows:?}");
                    }
                    for member in [
                        "2\u{a0}×\u{a0}Cinder\u{a0}Wolf",
                        "1\u{a0}×\u{a0}Ember\u{a0}Cultist",
                    ] {
                        assert!(
                            width < 120.0 || rows.iter().any(|row| row.contains(member)),
                            "{width}: {member} is split in {rows:?}"
                        );
                    }
                }
                // The check can fail: some width wraps at a separator, which the cut removed.
                assert!(cuts > 0, "no width wrapped at a separator");
            });
        });
    }

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

    /// Review of PR 284: the day never changes a planned row's height, at any width.
    #[test]
    fn pr284_review_row_height_and_long_journey_name() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let rating = rated(crate::difficulty::above_high());
        let summary = &roster(5, &[(2, "Ember Cultist")]);
        let words_of = summary.text();
        let day = "Day 2 of The road to the ford beyond the old northern quarry";
        let (mut full, mut short) = (0, 0);
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for width in (120..=600).step_by(5) {
                    let plain = ui
                        .scope(|ui| {
                            ui.set_width(width as f32);
                            planned_summary(ui, summary, Some(&rating));
                        })
                        .response
                        .rect
                        .height();
                    let road = ui
                        .scope(|ui| {
                            ui.set_width(width as f32);
                            let words = road_day(ui, &words_of, Some(day));
                            match words.as_deref() {
                                Some(words) if words.contains("of The road") => full += 1,
                                Some(_) => short += 1,
                                None => panic!("a day given is always offered to the row"),
                            }
                            planned_summary_on_road(ui, summary, words.as_deref(), Some(&rating));
                        })
                        .response
                        .rect
                        .height();
                    assert_eq!(road, plain, "the day adds no height at {width} px");
                }
            });
        });
        assert!(full > 0 && short > 0, "{full} {short}");
        // Round 28 FE6: where the row is cut, the party's size gives way first, then the
        // roster; the day, which ties the fight to its journey, goes last.
        let mut day_kept_in_a_cut = 0;
        let mut seen = Vec::new();
        let day = "Day 2 of Travel";
        for width in (120..=600).step_by(10) {
            let mut words = None;
            let mut painted = None;
            for _ in 0..2 {
                let output = ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.scope(|ui| {
                            ui.set_width(width as f32);
                            words = road_day(ui, &words_of, Some(day));
                            planned_summary_on_road(ui, summary, words.as_deref(), Some(&rating));
                        });
                    });
                });
                painted = output.shapes.iter().find_map(|shape| match &shape.shape {
                    // Q-s (R39) may drop the XP number, and at the narrowest the day: the
                    // summary is the line that ends with the word, which never gives way.
                    egui::Shape::Text(text) if text.galley.job.text.contains("Above") => {
                        Some(text.galley.job.text.replace(['\u{a0}', '\n'], " "))
                    }
                    _ => None,
                });
            }
            let painted = painted.unwrap_or_else(|| panic!("at {width} px: no summary"));
            seen.push(format!("{width}: {words:?} {painted:?}"));
            if painted.contains("Ember Cultist") {
                assert!(
                    painted.contains("Day 2"),
                    "at {width} px the day gave way before the roster: {painted:?}"
                );
            }
            if painted.contains("5 party") {
                assert!(
                    painted.contains("Ember Cultist"),
                    "at {width} px the roster gave way before the party's size: {painted:?}"
                );
            }
            if !painted.contains("5 party") && painted.contains("Day 2") {
                day_kept_in_a_cut += 1;
            }
        }
        assert!(
            day_kept_in_a_cut > 0,
            "no width kept the day in a cut row: {seen:#?}"
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
        // journey and the check, its row says the day, and Remove from tonight takes the line.
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
        assert_eq!(
            road_days(&marked, &book.events, Some(session), &fights),
            [
                (
                    "wolf pack".to_owned(),
                    Some("Day 2 of Road to the ford".to_owned())
                ),
                ("bridge ambush".to_owned(), None)
            ]
        );
        assert_eq!(unlink_planned_fight(&marked, "Wolf Pack", 1), body);
        // Review of PR 717 (P1): the planned fights' days are kept by the fights' revision, not
        // their count. Wolf Pack renamed on disk leaves the note and the count as they were;
        // Bridge Ambush, now the first planned fight, shows no day of Wolf Pack's.
        {
            use crate::sessions::{Draft, NewNote, create_note};
            let dir = tempfile::tempdir().unwrap();
            let note = create_note(
                dir.path(),
                &NewNote {
                    number: 1,
                    title: "The Crossing",
                    date: "2026-11-12",
                    campaign: "Marches",
                    template: &marked[marked.find("## Plan").unwrap()..],
                },
            )
            .unwrap();
            let mut sessions = Sessions {
                draft: Some(Draft::open(&note.path).unwrap()),
                ..Sessions::default()
            };
            let renamed = [
                planned_fight("Grey Wolves", "Encounters/Grey Wolves.md"),
                planned_fight("Bridge Ambush", "Encounters/Bridge Ambush.md"),
            ];
            let mut days_of = |fights: &[PlanFight], fights_revision| {
                let body = sessions.draft.as_ref().unwrap().body.as_str().to_owned();
                let planned: Vec<&PlanFight> =
                    plan_links(&body, &book.events, Some(session), fights)
                        .into_iter()
                        .filter(|(_, _, waiting)| *waiting)
                        .map(|(_, fight, _)| fight)
                        .collect();
                let mut group = |_: &mut egui::Ui| {};
                let plan = Plan {
                    fights,
                    fights_revision,
                    saved_runs: &[],
                    saved_runs_revision: fights_revision,
                    groups: &[],
                    group_blocked: None,
                    group_open: false,
                    group_discard: &|_| None,
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
                let names: Vec<String> = planned.iter().map(|fight| fight.name.clone()).collect();
                let places: Vec<usize> = planned
                    .iter()
                    .map(|fight| {
                        fights
                            .iter()
                            .position(|kept| std::ptr::eq(kept, *fight))
                            .expect("a planned fight is one of the fights")
                    })
                    .collect();
                let days = sessions.planned_days(&book, Some(session), &plan, &places);
                names
                    .into_iter()
                    .zip(days.at.iter().cloned())
                    .collect::<Vec<_>>()
            };
            let read = crate::design::memo::Revised::new(()).revision();
            assert_eq!(
                days_of(&fights, read),
                [
                    (
                        "Wolf Pack".to_owned(),
                        Some("Day 2 of Road to the ford".to_owned())
                    ),
                    ("Bridge Ambush".to_owned(), None)
                ]
            );
            let read_again = crate::design::memo::Revised::new(()).revision();
            assert_eq!(
                days_of(&renamed, read_again),
                [("Bridge Ambush".to_owned(), None)],
                "a renamed fight's day stays with it, never on the row after it"
            );
        }
        // A fight planned the ordinary way carries nothing.
        assert_eq!(
            road_days(body, &book.events, Some(session), &fights)[0].1,
            None
        );
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
        // R45 (e): both readers' lines, made in whole egui frames on unchanged frames, walk no
        // entry either; the refused line offers no Undo, the other line does.
        let ctx = egui::Context::default();
        let draw = |sessions: &mut Sessions| {
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let frame = PaneFrame::new(ui);
                    let timeline = sessions.plan_line_kept(&frame, None, ());
                    let plan = sessions.plan_line_kept(&frame, Some(Some(b)), ());
                    assert_eq!((timeline.len(), plan.len()), (2, 1));
                });
            });
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
        let ctx = egui::Context::default();
        let mut painted = Vec::new();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let campaign = sessions.campaign.as_ref().map(|campaign| campaign.id);
                painted = sessions
                    .travel
                    .planned
                    .newest_first(&campaign)
                    .filter(|shown| [b, twin.id].contains(&shown.value().note))
                    .map(|shown| plan_line_words(shown.value(), true).1.fit(ui, 220.0))
                    .collect();
            });
        });
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
    fn iter5_c_timeline_fight_resumes_by_run_id_without_a_planned_source() {
        use crate::design::test_harness::Themed as _;
        use crate::encounter_run::{SavedRunInfo, Standing};
        use egui_kittest::kittest::Queryable;
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
        let page = Sessions {
            selected: Some(session),
            ..Default::default()
        };
        // The saved runs' facts, which these frames never change.
        let no_runs = crate::design::memo::Revised::new(()).revision();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 640.0))
            .themed_state(
                |ctx,
                 (page, book, runs, open): &mut (
                    Sessions,
                    Book,
                    Vec<SavedRunInfo>,
                    Option<Uuid>,
                )| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &[],
                        saved_runs: runs,
                        saved_runs_revision: no_runs,
                        fights_revision: no_runs,
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discard: &|_| None,
                        arrivals_open: false,
                        group_editor: std::cell::RefCell::new(&mut group),
                        blocked: None,
                        run_directory: None,
                        open_run: *open,
                        open_state: "Round 2".into(),
                        open_encounter: None,
                        party: &[],
                        tables: &[],
                    };
                    egui::CentralPanel::default()
                        .show(ctx, |ui| page.plan_pane(ui, book, &plan, None));
                },
                (page, book, vec![other, saved.clone()], None),
            );
        harness.run_steps(2);
        harness.get_by_label("Resume: Bridge Ambush").click();
        harness.run_steps(2);
        assert_eq!(
            harness.state().0.wants_saved_run.as_ref(),
            Some(&saved.path)
        );
        for state in [Standing::Ended, Standing::Unreadable] {
            harness.state_mut().2[1].standing = state;
            harness.run_steps(2);
            assert!(harness.query_by_label("Resume: Bridge Ambush").is_none());
        }
        harness.state_mut().2[1].standing = Standing::Unfinished;
        harness.state_mut().3 = Some(run);
        harness.run_steps(2);
        assert!(harness.query_by_label("Resume: Bridge Ambush").is_none());
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
    fn iter3_c_closed_plan_rest_shows_not_at_full() {
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::Queryable;
        let mut harness = egui_kittest::Harness::builder().themed(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                plan_event_summary(ui, "Old mill", Some(false), None);
            });
        });
        harness.run_steps(2);
        harness.get_by_label(&format!(
            "Old\u{a0}mill ·\u{a0}{}\u{a0}Not\u{a0}at\u{a0}full",
            icons::LONG_REST
        ));
    }

    #[test]
    fn a_rest_ruling_uses_one_line_when_the_summary_fits() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(design::FORM_WIDTH);
                let plain = ui
                    .scope(|ui| plan_event_summary(ui, "Long rest · Old mill", None, None))
                    .response
                    .rect
                    .height();
                let restored = ui
                    .scope(|ui| plan_event_summary(ui, "Long rest · Old mill", Some(true), None))
                    .response
                    .rect
                    .height();
                assert_eq!(plain, restored);
            });
        });
    }

    /// A planned fight's difficulty with this word.
    fn rated(rating: crate::difficulty::Rating) -> crate::difficulty::Difficulty {
        crate::difficulty::Difficulty {
            xp: rating.xp,
            rating: Some(rating),
            unknown: 0,
            missing: None,
        }
    }

    /// A planned fight's roster as typed parts (G118): the party count and each member.
    fn roster(party: usize, members: &[(usize, &str)]) -> crate::difficulty::Summary {
        crate::difficulty::Summary {
            party,
            members: members
                .iter()
                // The fixtures' creatures with no XP.
                .map(|&(count, name)| {
                    crate::difficulty::Member::new(
                        count,
                        name,
                        ["Nameless Thing", "Wisp"].contains(&name),
                    )
                })
                .collect(),
            facts: Vec::new(),
        }
    }

    /// V9 and W1-D2: the planned fight's row has the same blank on each side of every
    /// separator, with the form (FB6) and with `XP not known` when no word shows (FB4). FH6:
    /// the opened line under it says the difficulty no more.
    #[test]
    fn read_encounter_has_one_blank_each_side_of_each_separator() {
        use crate::difficulty::{Difficulty, Missing, Unknown};
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let rated = Difficulty {
            xp: 6300,
            rating: Some(crate::difficulty::above_high()),
            unknown: 0,
            missing: None,
        };
        let unknown = Difficulty {
            xp: 0,
            rating: None,
            unknown: 2,
            missing: Some(Missing {
                what: Unknown::Xp,
                at: vec![0, 1],
                names: vec!["Nameless Thing".into(), "Wisp".into()],
            }),
        };
        let mut low = crate::difficulty::above_high();
        low.word = crate::difficulty::Word::Low;
        low.xp = 400;
        // A roster member whose name cannot be read: the reason in its short form.
        let nameless = Difficulty {
            xp: 0,
            rating: None,
            unknown: 1,
            missing: Some(Missing {
                what: Unknown::Level,
                at: vec![0],
                names: Vec::new(),
            }),
        };
        for (difficulty, words) in [
            (rated, "5 party · 2 × Ember Cultist · XP 6,300 · Above High"),
            (
                self::rated(low),
                "5 party · 2 × Ember Cultist · XP 400 · Low (up to 750)",
            ),
            (
                unknown,
                "5 party · 2 × Ember Cultist · XP not known: Nameless Thing and 1 more",
            ),
            (nameless, "5 party · 2 × Ember Cultist · Level not known"),
        ] {
            // The second frame has the app's fonts.
            let frame = || {
                ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.set_width(design::LIST_MEASURE);
                        planned_summary(ui, &roster(5, &[(2, "Ember Cultist")]), Some(&difficulty));
                    });
                })
            };
            frame();
            let output = frame();
            // Each glyph on the line by its advance, in order from the left: the blank is the
            // space between two glyphs' boxes, as the font sets it. Glyphs sit on whole pixels,
            // so the blanks agree to 1 px.
            let mut glyphs = Vec::new();
            for shape in &output.shapes {
                if let egui::Shape::Text(text) = &shape.shape {
                    for row in &text.galley.rows {
                        for glyph in &row.glyphs {
                            let left = text.pos.x + glyph.pos.x;
                            glyphs.push((glyph.chr, left, left + glyph.advance_width));
                        }
                    }
                }
            }
            glyphs.sort_by(|a, b| a.1.total_cmp(&b.1));
            let line: String = glyphs.iter().map(|glyph| glyph.0).collect();
            assert_eq!(line.replace('\u{a0}', " "), words);
            let ink: Vec<_> = glyphs
                .iter()
                .filter(|glyph| !glyph.0.is_whitespace())
                .collect();
            // Every separator on the row: the summary's, the join before the difficulty form,
            // and the form's own.
            let blanks: Vec<(f32, f32)> = (1..ink.len() - 1)
                .filter(|at| ink[*at].0 == '·')
                .map(|at| (ink[at].1 - ink[at - 1].2, ink[at + 1].1 - ink[at].2))
                .collect();
            assert!(
                !blanks.is_empty()
                    && blanks
                        .iter()
                        .all(|(before, after)| (before - after).abs() <= 1.0 && *before >= 2.0),
                "{words}: the blank before and after each separator: {blanks:?}"
            );
        }
    }

    #[test]
    fn the_difficulty_form_ends_a_summary_and_the_summary_gives_way() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let rating = rated(crate::difficulty::above_high());
        let typed = roster(
            5,
            &[(1, "Cinder Tyrant"), (2, "Ember Cultist"), (1, "Wolf")],
        );
        let words = typed.text();
        let summary = words.as_str();
        assert_eq!(
            summary,
            "5 party · 1 × Cinder Tyrant, 2 × Ember Cultist, 1 × Wolf"
        );
        let (mut whole, mut cut, mut xp_gone) = (0, 0, 0);
        for width in (120..=640).step_by(4).map(|width| width as f32) {
            let mut heights = (0.0, 0.0);
            // The second frame has the app's fonts.
            let mut output = None;
            for _ in 0..2 {
                output = Some(ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let height = |ui: &mut egui::Ui, rating| {
                            ui.scope(|ui| {
                                ui.set_width(width);
                                planned_summary(ui, &typed, rating);
                            })
                            .response
                            .rect
                            .height()
                        };
                        heights = (height(ui, None), height(ui, Some(&rating)));
                    });
                }));
            }
            let (plain, with) = heights;
            assert!(with <= plain + 0.5, "at {width} px: {with} > {plain}");
            let texts: Vec<String> = output
                .unwrap()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    // The protected word, not the XP number: under Q-s XP can give way.
                    egui::Shape::Text(text) if text.galley.job.text.contains("Above") => Some(
                        text.galley
                            .job
                            .text
                            .replace('\u{a0}', " ")
                            .replace('\n', " · ")
                            .replace(" · · ", " · "),
                    ),
                    _ => None,
                })
                .collect();
            // FB2: the form ends the row at every width.
            let [line] = texts.as_slice() else {
                panic!("at {width} px: {texts:?}");
            };
            // FB2, Q-s: the word ends the row at every width; the XP number goes only after
            // the party's size and before any member.
            let (facts, xp) = match line.strip_suffix("XP 6,300 · Above High") {
                Some(facts) => (facts, true),
                None => (
                    line.strip_suffix("Above High")
                        .unwrap_or_else(|| panic!("at {width} px: {line:?}")),
                    false,
                ),
            };
            let facts = facts.trim_end_matches([' ', '·']);
            let members = "1 × Cinder Tyrant, 2 × Ember Cultist, 1 × Wolf";
            if facts.replace(" · ", " ") == summary.replace(" · ", " ") {
                assert!(
                    xp,
                    "at {width} px the XP gave way before the party: {line:?}"
                );
                whole += 1;
            } else {
                // The summary gives way by whole facts and members, never inside one. Round 28
                // FE6: the party's size gives way first (with no `…`), then (Q-s) the XP
                // number, then the members from the end.
                assert!(
                    !xp || facts == members,
                    "at {width} px a member gave way before the XP number: {line:?}"
                );
                xp_gone += usize::from(!xp && facts == members);
                cut += 1;
                let kept = facts.strip_suffix('…').unwrap_or(facts);
                let kept = kept.trim_end_matches([' ', '·', ',']);
                assert!(
                    kept.is_empty()
                        || [
                            "1 × Cinder Tyrant, 2 × Ember Cultist, 1 × Wolf",
                            "1 × Cinder Tyrant, 2 × Ember Cultist",
                            "1 × Cinder Tyrant",
                        ]
                        .iter()
                        .any(|unit| kept.replace(" · ", " ") == unit.replace(" · ", " ")),
                    "at {width} px a member was cut inside, or before the party's size: {line:?}"
                );
            }
        }
        assert!(
            whole > 0 && cut > 0 && xp_gone > 0,
            "whole {whole}, cut {cut}, every member beside the word alone {xp_gone}"
        );
        // Review of PR 299: one input that is not known, at a width where the named reason fits
        // alone but not after the summary: the summary gives way, the name shows. Round 28 FB6:
        // that holds for a name the summary does not hold (a character's level); a creature the
        // summary already names stays whole in it, with the short reason after it.
        use crate::difficulty::{Difficulty, Missing, Unknown};
        let unknown = |what| Difficulty {
            xp: 0,
            rating: None,
            unknown: 1,
            missing: Some(Missing {
                what,
                at: vec![0],
                names: vec!["Nameless Thing".into()],
            }),
        };
        let one = &roster(0, &[(1, "Nameless Thing")]);
        let wolf = &roster(0, &[(1, "Ember Wolf")]);
        let case = |typed: &crate::difficulty::Summary, difficulty: &Difficulty, reason: &str| {
            let summary = &typed.text();
            let mut painted = None;
            let mut widths = None;
            for _ in 0..3 {
                let output = ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let font = Type::Small.font(ui);
                        let wide = |text: &str| {
                            design::number_width(ui, &text.replace(' ', "\u{a0}"), &font)
                        };
                        let alone = wide(&format!("{reason}: Nameless Thing"));
                        let after = wide(&format!("{summary} · {reason}: N…"));
                        let width = (alone + after) / 2.0;
                        widths = Some((alone, after, width, wide(summary)));
                        ui.scope(|ui| {
                            ui.set_width(width);
                            planned_summary(ui, typed, Some(difficulty));
                        });
                    });
                });
                painted = output.shapes.iter().find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.replace('\u{a0}', " "))
                        .filter(|words| words.contains("not known")),
                    _ => None,
                });
            }
            let (alone, after, width, summary_width) = widths.unwrap();
            assert!(alone < width && width < after && summary_width < width);
            (painted, width)
        };
        let (painted, width) = case(wolf, &unknown(Unknown::Level), "Level not known");
        assert!(
            painted
                .as_deref()
                .is_some_and(|line| line.contains("Level not known: Nameless")),
            "at {width} px: {painted:?}"
        );
        let (painted, width) = case(one, &unknown(Unknown::Xp), "XP not known");
        assert_eq!(
            painted.as_deref(),
            Some("1 × Nameless Thing · XP not known"),
            "FB6: the member stays whole and is not named twice, at {width} px"
        );
        // R31 (review of PR 324, kept): with a creature whose XP is known beside it, the
        // summary does not say whose XP is missing, so the reason keeps the name. Where every
        // creature shown lacks its XP, the reason is short. Members are matched by the name
        // each is called by, never by parsed words; `Changed` is not a creature (G118).
        let wide = |typed: &crate::difficulty::Summary, difficulty: &Difficulty| {
            let mut painted = None;
            for _ in 0..2 {
                let output = ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.scope(|ui| {
                            ui.set_width(600.0);
                            planned_summary(ui, typed, Some(difficulty));
                        });
                    });
                });
                painted = output.shapes.iter().find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.replace('\u{a0}', " "))
                        .filter(|words| words.contains("not known")),
                    _ => None,
                });
            }
            painted
        };
        let missing = |names: &[&str]| Difficulty {
            unknown: names.len() as u64,
            missing: Some(Missing {
                what: Unknown::Xp,
                at: (0..names.len()).collect(),
                names: names.iter().map(|name| (*name).to_owned()).collect(),
            }),
            ..unknown(Unknown::Xp)
        };
        let mixed = roster(0, &[(1, "Wolf"), (1, "Nameless Thing")]);
        assert_eq!(
            wide(&mixed, &unknown(Unknown::Xp)).as_deref(),
            Some("1 × Wolf, 1 × Nameless Thing · XP not known: Nameless Thing"),
            "a mixed roster names whose XP is missing"
        );
        let two = roster(0, &[(1, "Nameless Thing"), (1, "Wisp")]);
        assert_eq!(
            wide(&two, &missing(&["Nameless Thing", "Wisp"])).as_deref(),
            Some("1 × Nameless Thing, 1 × Wisp · XP not known"),
            "two creatures that both lack their XP: the summary says whose"
        );
        let shade = |beside: bool| crate::difficulty::Summary {
            members: beside
                .then(|| crate::difficulty::Member::new(1, "Wolf", false))
                .into_iter()
                .chain([crate::difficulty::Member::renamed(
                    1,
                    "Shade",
                    "Nameless Thing",
                    true,
                )])
                .collect(),
            facts: vec![crate::difficulty::Fact::Changed],
            ..Default::default()
        };
        assert_eq!(
            wide(&shade(false), &missing(&["Shade"])).as_deref(),
            Some("1 × Shade (as Nameless Thing) · Changed · XP not known"),
            "a renamed creature alone is matched by the name it is called by"
        );
        assert_eq!(
            wide(&shade(true), &missing(&["Shade"])).as_deref(),
            Some("1 × Wolf, 1 × Shade (as Nameless Thing) · Changed · XP not known: Shade"),
            "beside a known creature, the reason keeps the called name"
        );
        assert_eq!(
            wide(&roster(0, &[(1, "Wolf")]), &unknown(Unknown::Xp)).as_deref(),
            Some("1 × Wolf · XP not known: Nameless Thing"),
            "a creature the row does not show is named"
        );
        // R31 by the member, never by a name (final review of PR 390): Nameless Thing renamed
        // `Wolf` beside a known Wolf. Both are called `Wolf`; a known creature is shown, so the
        // reason keeps the name.
        let twins = crate::difficulty::Summary {
            members: vec![
                crate::difficulty::Member::new(1, "Wolf", false),
                crate::difficulty::Member::renamed(1, "Wolf", "Nameless Thing", true),
            ],
            ..Default::default()
        };
        assert_eq!(
            wide(&twins, &missing(&["Wolf"])).as_deref(),
            Some("1 × Wolf, 1 × Wolf (as Nameless Thing) · XP not known: Wolf"),
            "a known creature is shown, so the reason keeps the name"
        );
        // A stage and its reason are chosen together (final review of PR 390): at every width
        // the line obeys R31 for the units it shows. Nameless Thing (no XP) and then Wolf
        // (known): Wolf gives way first; then the line says whose, and the reason is short.
        let sweep = |typed: &crate::difficulty::Summary| {
            let mut seen = Vec::new();
            for width in (100..=600).step_by(2).map(|width| width as f32) {
                let mut painted = None;
                for _ in 0..2 {
                    let output = ctx.run(Default::default(), |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            ui.scope(|ui| {
                                ui.set_width(width);
                                planned_summary(ui, typed, Some(&unknown(Unknown::Xp)));
                            });
                        });
                    });
                    painted = output.shapes.iter().find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => {
                            Some(text.galley.job.text.replace(['\u{a0}', '\n'], " "))
                                .filter(|words| words.contains("not known"))
                        }
                        _ => None,
                    });
                }
                seen.push((width, painted.unwrap_or_default()));
            }
            seen
        };
        let lines = sweep(&roster(0, &[(1, "Nameless Thing"), (1, "Wolf")]));
        let mut says_whose = 0;
        for (width, line) in &lines {
            if line.contains("1 × Wolf") {
                assert!(line.contains("XP not known: N"), "at {width} px: {line:?}");
            } else if line.contains("1 × Nameless Thing") {
                assert!(line.ends_with("XP not known"), "at {width} px: {line:?}");
                says_whose += 1;
            } else {
                assert!(line.contains("XP not known"), "at {width} px: {line:?}");
            }
        }
        assert!(says_whose > 0, "no width cut Wolf alone: {lines:?}");
        // G118 (re-check of PR 390): the plan keeps each fitted line. An unchanged redraw fits
        // none; a new width or a new roster fits again; a removed fight's line goes.
        let mut sessions = super::super::Sessions::default();
        let fight = |summary: &crate::difficulty::Summary| PlanFight {
            saved: None,
            id: Uuid::from_u128(1),
            name: "Mill Road".into(),
            path: "Encounters/Mill Road.md".into(),
            summary: summary.clone(),
            revision: Some(crate::design::memo::Revised::new(()).revision()),
            difficulty: unknown(Unknown::Xp),
        };
        let id = (None, 0, Uuid::from_u128(1));
        let fitted = || PLAN_FITS.with(std::cell::Cell::get);
        let draw = |sessions: &mut super::super::Sessions, fight: &PlanFight, width: f32| {
            let _ = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.scope(|ui| {
                        ui.set_width(width);
                        let days = sessions.plan_lines_seen.revision();
                        let room = ui.available_width();
                        let _ = kept_plan_line(
                            ui,
                            (&mut sessions.plan_lines, room),
                            id,
                            fight,
                            None,
                            days,
                        );
                    });
                });
            });
        };
        let first = fight(&mixed);
        draw(&mut sessions, &first, 400.0);
        let before = fitted();
        draw(&mut sessions, &first, 400.0);
        draw(&mut sessions, &first, 400.0);
        assert_eq!(fitted(), before, "an unchanged redraw fits no line");
        draw(&mut sessions, &first, 300.0);
        assert_eq!(fitted(), before + 1, "a new width fits again");
        let second = fight(&two);
        draw(&mut sessions, &second, 300.0);
        assert_eq!(fitted(), before + 2, "a new roster fits again");
        // A new day for a planned fight fits its line again (ruling M6: the plan's revision).
        sessions.forget_plan_lines(vec![(id, Some("Day 3".into()))]);
        draw(&mut sessions, &second, 300.0);
        assert_eq!(fitted(), before + 3, "a new day fits again");
        draw(&mut sessions, &second, 300.0);
        assert_eq!(
            fitted(),
            before + 3,
            "an unchanged redraw after it fits none"
        );
        assert!(sessions.plan_lines.holds(&id));
        sessions.forget_plan_lines(vec![((None, 0, Uuid::from_u128(2)), None)]);
        assert!(
            !sessions.plan_lines.holds(&id),
            "a removed fight's line goes"
        );
        // Layout 11c A2c: a plan row lays the line out in its summary's room, before the row
        // draws; the line is fitted in that room, not in the width of the `Ui` it is asked in.
        let line_in = |width: f32, room: f32| {
            let mut lines = super::super::Sessions::default().plan_lines;
            let mut text = String::new();
            let _ = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(width);
                    let line = kept_plan_line(
                        ui,
                        (&mut lines, room),
                        id,
                        &first,
                        Some("Day 2 of Road to the ford"),
                        crate::design::memo::Revised::new(()).revision(),
                    );
                    text = match line.text {
                        PlanText::Alone(alone) => alone,
                        PlanText::Form(job) => job.text,
                    };
                });
            });
            text
        };
        let (narrow, in_wide) = (line_in(160.0, 160.0), line_in(400.0, 160.0));
        assert!(
            narrow == in_wide && narrow != line_in(400.0, 400.0),
            "the line fits its room: {narrow:?} in its own Ui, {in_wide:?} in a wider one"
        );
    }

    #[test]
    fn long_rest_added_on_plan_restores_and_undo_removes_its_ruling() {
        use crate::design::test_harness::Themed as _;
        timeline::page::check_long_rest_add(&PLAN_KINDS);
        // Early review F1 of PR 444 (ruling L9): the Add result of a long rest names the fights
        // that keep their saved HP. Its kept line follows the saved runs' facts by their own
        // revision: the DM deletes the saved run (the book does not change), and the clause
        // goes in the next frame while the rest's result and its Undo stay; Undo of the
        // deletion brings the clause back.
        use crate::design::memo::{Changed, Revised};
        use crate::encounter_run::{SavedRunInfo, Standing};
        use egui_kittest::kittest::Queryable;
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
        let page = Sessions {
            selected: Some(session),
            ..Default::default()
        };
        type State = (Sessions, Book, Revised<Vec<SavedRunInfo>>);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 640.0))
            .themed_state(
                |ctx, (page, book, runs): &mut State| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &[],
                        saved_runs: runs,
                        saved_runs_revision: runs.revision(),
                        fights_revision: runs.revision(),
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discard: &|_| None,
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
                    page.settle_timed(ctx, book);
                    egui::CentralPanel::default()
                        .show(ctx, |ui| page.plan_pane(ui, book, &plan, None));
                },
                (page, book, Revised::new(vec![saved.clone()])),
            );
        harness.run_steps(2);
        harness.get_by_label("Add Long rest").click();
        harness.run_steps(3);
        let keeps = |harness: &egui_kittest::Harness<'_, State>| {
            harness
                .query_all_by_label_contains("Bridge keeps its saved HP.")
                .next()
                .is_some()
        };
        let rest_stands = |harness: &egui_kittest::Harness<'_, State>| {
            harness.query_by_label("Undo add").is_some()
                && harness
                    .query_all_by_label_contains("Long rest added.")
                    .next()
                    .is_some()
        };
        assert!(
            rest_stands(&harness) && keeps(&harness),
            "the rest's result names the fight that keeps its HP"
        );
        let book_revision = harness.state().1.revision();
        harness.state_mut().2.revise(|runs| {
            runs.clear();
            Changed::Yes
        });
        harness.step();
        assert_eq!(
            harness.state().1.revision(),
            book_revision,
            "deleting a saved run leaves the book as it was"
        );
        assert!(
            !keeps(&harness),
            "a deleted saved run keeps no HP: the clause goes"
        );
        assert!(
            rest_stands(&harness),
            "the unrelated deletion leaves the rest's result and its Undo"
        );
        harness.state_mut().2.revise(|runs| {
            runs.push(saved.clone());
            Changed::Yes
        });
        harness.step();
        assert_eq!(harness.state().1.revision(), book_revision);
        assert!(
            keeps(&harness) && rest_stands(&harness),
            "Undo of the deletion brings the clause back"
        );
    }

    #[test]
    fn plan_summary_follows_the_title_without_an_extra_gap() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(design::LIST_MEASURE / 2.0);
                for name in [
                    "Bridge Ambush".to_owned(),
                    "The very long battle at the bridge ".repeat(5),
                ] {
                    let mut title = egui::Rect::NOTHING;
                    let mut action = egui::Rect::NOTHING;
                    let mut summary = egui::Rect::NOTHING;
                    plan_row(
                        ui,
                        (
                            &[design::FORM_WIDTH, design::FORM_WIDTH / 3.0],
                            design::PlanColumns::default(),
                        ),
                        icons::RUN,
                        |ui| {
                            title = ui
                                .add(egui::Label::new(Type::Body.rich(ui, &name)).wrap())
                                .rect;
                        },
                        |ui| {
                            summary = ui
                                .label(Type::Small.rich(ui, "5 party · 2 × Cinder Wolf"))
                                .rect;
                        },
                        |ui| {
                            action = Action::secondary("Start fight")
                                .density(Density::Read)
                                .show(ui)
                                .rect;
                        },
                    );
                    assert!(
                        summary.top() >= title.bottom(),
                        "{name}: {title:?} {summary:?}"
                    );
                    assert!(
                        (summary.top() - title.bottom().max(action.bottom())).abs() < 1.0,
                        "No extra item gap: {title:?} {action:?} {summary:?}"
                    );
                }
            });
        });
        // PR 605 review (P1): a short name over a summary of three lines. The plan's column is
        // measured from every line its rows draw, so it never starts inside the summary: the
        // row keeps the far edge here, where the column does not fit beside the summary.
        let long = "Rivals · 3 members · XP 600 · Changed · 1 × Ember Cultist · 2 × Goblin \
                    Warrior · 4 × Wolf · 1 × Cinder Tyrant · 3 × Bandit · 2 × Scout · 1 × \
                    Veteran · 5 × Guard · 2 × Thug · 1 × Spy";
        // A narrow window, so the summary takes three lines.
        let narrow = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(480.0, 400.0),
            )),
            ..Default::default()
        };
        for _ in 0..3 {
            let _ = ctx.run(narrow(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    plan_row(
                        ui,
                        (
                            &[design::FORM_WIDTH, design::FORM_WIDTH / 3.0],
                            design::PlanColumns::default(),
                        ),
                        icons::GROUP,
                        |ui| {
                            ui.label(Type::Body.rich(ui, "Ox"));
                        },
                        |ui| {
                            ui.add(egui::Label::new(Type::Small.rich(ui, long)).wrap());
                        },
                        |ui| {
                            let _ = Action::quiet("Edit").density(Density::Read).show(ui);
                        },
                    );
                });
            });
        }
        let records: Vec<_> = design::action_rows::row_records(&ctx)
            .into_iter()
            .filter(|record| record.owner == "plan")
            .collect();
        // Layout PR 5a-2 (I6), PR 772 review (P3): the plan's mixed rows (a fight, a running
        // fight, a group's Edit, a scene's Add to Log), each with its own recipe (its action's
        // width), at 960, 1200 and 1920. The column fits at each width: one column for all the
        // rows, exactly `L` after the widest facts line, and each row's actions start at it.
        let rows: [(&str, &str, &str, bool); 4] = [
            (
                "Bridge Ambush",
                "5 party · 2 × Cinder Wolf",
                "Start fight",
                true,
            ),
            ("Mill Fight", "Round 2 · 4 left", "Resume", true),
            ("Rivals", "3 members · XP 600", "Edit", false),
            ("The miller talks", "", "Add to Log", false),
        ];
        for width in [960.0, 1200.0, 1920.0] {
            let input = || egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 600.0),
                )),
                ..Default::default()
            };
            for _ in 0..3 {
                let _ = ctx.run(input(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        for (title, summary, act, secondary) in rows {
                            let action = || {
                                if secondary {
                                    Action::secondary(act)
                                } else {
                                    Action::quiet(act)
                                }
                                .density(Density::Read)
                            };
                            let own = action().natural_width(ui);
                            plan_row(
                                ui,
                                (&[design::FORM_WIDTH, own], design::PlanColumns::default()),
                                icons::RUN,
                                |ui| {
                                    ui.label(Type::Body.rich(ui, title));
                                },
                                |ui| {
                                    if !summary.is_empty() {
                                        ui.label(Type::Small.rich(ui, summary));
                                    }
                                },
                                |ui| {
                                    let _ = action().show(ui);
                                },
                            );
                        }
                    });
                });
            }
            let fitting: Vec<_> = design::action_rows::row_records(&ctx)
                .into_iter()
                .filter(|record| record.owner == "plan")
                .collect();
            let widest = fitting
                .iter()
                .map(|record| record.facts.right())
                .fold(f32::MIN, f32::max);
            assert!(
                fitting.len() == rows.len()
                    && fitting.iter().all(|record| {
                        !record.wrapped
                            && (record.column - (widest + design::L)).abs() <= 0.5
                            && (record.action.left() - record.column).abs() <= 0.5
                    }),
                "at {width}, one column for the mixed rows, L after the widest facts \
                 {widest}: {fitting:?}"
            );
        }
        assert!(
            records.len() == 1
                && records.iter().all(|record| {
                    record.facts.height() > 3.0 * Type::Small.line() - 0.5
                        && (record.wrapped
                            || record.column >= record.facts.right() + design::L - 0.5)
                }),
            "the column starts after every line of the facts, or keeps the far edge: {records:?}"
        );
    }

    #[test]
    fn i19_p5_group_summary_wraps_before_it_shortens() {
        use crate::design::test_harness::Themed as _;
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH / 2.0, 200.0))
            .themed_state(|ctx, rect| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    *rect = ui.scope(|ui| plan_group_summary(ui, "Rivals · 3 members · XP 600 · Changed · 1 × Ember Cultist · 2 × Goblin Warrior", 3)).response.rect;
                });
            }, egui::Rect::NOTHING);
        harness.run_steps(2);
        let rect = harness.state();
        assert!(rect.height() > Type::Small.line(), "{rect:?}");
        assert!(rect.height() <= Type::Small.line() * 2.0);
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

    /// T12.10 and section 4.1 of the session-cycle design: a scene's box, its removal and the
    /// Scene field are note acts. Each follows the disk, writes the note at once, and refuses with
    /// words when unsaved typed words meet a changed file; a failed save says so, and the file
    /// holds what an act wrote after a restart. An Undo survives an act that leaves the note's
    /// words alone, and ends at one that changes them.
    #[test]
    fn scenes_are_note_acts_with_their_own_undo() {
        use crate::design::test_harness::Themed as _;
        use crate::sessions::{NewNote, create_note, plan_lines};
        use egui_kittest::kittest::Queryable;
        // Review round 1 (REVIEW-291-scene-state-height): a scene that links a rule note has no
        // second line; when the note goes, `Not found` ends its line and the row keeps its
        // height.
        // The saved runs' facts, which these frames never change.
        let no_runs = crate::design::memo::Revised::new(()).revision();
        let mut row = egui_kittest::Harness::builder()
            .with_size(egui::vec2(400.0, 800.0))
            .themed_state(
                |ctx, state: &mut (bool, f32)| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &[],
                        saved_runs: &[],
                        saved_runs_revision: no_runs,
                        fights_revision: no_runs,
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discard: &|_| None,
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
                    let scene = Scene {
                        task: TaskLine {
                            at: 0,
                            line: "- [ ] Read [[Emberfall Rumours]]".into(),
                            done: false,
                            words: "Read [[Emberfall Rumours]]".into(),
                        },
                        // A rule note that is found has no World detail, no fight and no Log.
                        entry: (!state.0).then_some(SceneEntry {
                            detail: None,
                            start: None,
                            log: None,
                        }),
                        not_found: state.0,
                        second_line: false,
                        fit: std::sync::Mutex::new(Vec::new()),
                        words: Box::default(),
                    };
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.set_width(330.0);
                        state.1 = scene_row(
                            ui,
                            (330.0, design::PlanColumns::default()),
                            &scene,
                            ((0, 0), 0),
                            &plan,
                            &mut None,
                        );
                    });
                },
                (false, 0.0),
            );
        row.run_steps(3);
        let found = row.state().1;
        row.state_mut().0 = true;
        row.run_steps(3);
        let missing = row.state().1;
        assert!(
            (found - missing).abs() < 0.5,
            "REVIEW-291-scene-state-height: a rule-note scene changes height when its entry goes: {found} -> {missing}"
        );
        assert!(row.query_by_label_contains("Not found").is_some());
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 2,
                title: "The Crossing",
                date: "2026-11-01",
                campaign: "Marches",
                template: "## Plan for tonight\n\n- [[Bridge]]\n- [ ] [[Captain Varra]]\n- [x] The miller is the informer\n- Hobb owes money\n\n## What happened\n\n",
            },
        )
        .unwrap();
        let read = || fs::read_to_string(&note.path).unwrap();
        let page = Sessions {
            selected: Some(note.id),
            notes: vec![note.clone()],
            folder: Some(dir.path().to_owned()),
            draft: Some(Draft::open(&note.path).unwrap()),
            ..Sessions::default()
        };
        let fights = [planned_fight("Bridge", "Encounters/Bridge.md")];
        // The saved runs' facts, which these frames never change.
        let no_runs = crate::design::memo::Revised::new(()).revision();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 800.0))
            .themed_state(
                |ctx, (page, book): &mut (Sessions, Book)| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &fights,
                        saved_runs: &[],
                        saved_runs_revision: no_runs,
                        fights_revision: no_runs,
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discard: &|_| None,
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
                    // The app's frame: the owner update (`Sessions::poll` ends with
                    // `settle_timed`), then the page.
                    page.settle_timed(ctx, book);
                    egui::CentralPanel::default()
                        .show(ctx, |ui| page.plan_pane(ui, book, &plan, None));
                },
                (page, Book::default()),
            );
        // The plan model is built when the note changes, never on a quiet frame.
        let model = crate::test_support::CachedWork::start(
            "Scene model",
            &mut harness,
            |harness: &mut egui_kittest::Harness<'_, (Sessions, Book)>| {
                let cache = &harness.state().0.plan_model;
                crate::test_support::Counts {
                    reached: cache.reached,
                    work: cache.builds,
                }
            },
        );
        harness.run_steps(2);
        let shown = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, name: &str| {
            harness.query_by_label(name).is_some()
        };
        let warm = shown(&harness, "Open: Captain Varra");
        model
            // The harness drew its first frame, and built the model, as it was made.
            .warm(&mut harness, warm, 0)
            .expect("scene-model-warm");
        model
            .steady(&mut harness, |harness| {
                harness.run_steps(3);
                harness.set_size(egui::vec2(design::FORM_WIDTH + 200.0, 800.0));
                harness.run_steps(3);
                harness.set_size(egui::vec2(design::FORM_WIDTH, 800.0));
                harness.run_steps(3);
                shown(harness, "Open: Captain Varra")
            })
            .expect("scene-model-steady");
        // Layout 11c A2a: the plan's projection (what its closed rows read from the note, the
        // Timeline, the fights and groups, the saved runs and the index) is built when one of
        // them changes, never on a quiet frame or a resize, counted from the plan's entry.
        let projection = crate::test_support::CachedWork::start(
            "Plan projection",
            &mut harness,
            |harness: &mut egui_kittest::Harness<'_, (Sessions, Book)>| {
                let cache = &harness.state().0.plan_projection;
                crate::test_support::Counts {
                    reached: cache.reached,
                    work: cache.builds,
                }
            },
        );
        harness.run_steps(2);
        let warm = shown(&harness, "Open: Captain Varra");
        projection
            .warm(&mut harness, warm, 0)
            .expect("plan-projection-warm");
        projection
            .steady(&mut harness, |harness| {
                harness.run_steps(3);
                harness.set_size(egui::vec2(design::FORM_WIDTH + 200.0, 800.0));
                harness.run_steps(3);
                harness.set_size(egui::vec2(design::FORM_WIDTH, 800.0));
                harness.run_steps(3);
                shown(harness, "Open: Captain Varra")
            })
            .expect("plan-projection-steady");
        // Layout 11c A2c (review of PR 841): the plan's closed rows are measured and laid out by
        // their recipe when one of their inputs changes, never on a quiet frame, counted from
        // the plan's entry through paint.
        let rows = crate::test_support::CachedWork::start(
            "Plan rows",
            &mut harness,
            |harness: &mut egui_kittest::Harness<'_, (Sessions, Book)>| {
                let cache = &harness.state().0.plan_rows;
                crate::test_support::Counts {
                    reached: cache.reached,
                    work: cache.builds,
                }
            },
        );
        harness.run_steps(2);
        let warm = shown(&harness, "Bridge");
        rows.warm(&mut harness, warm, 0).expect("plan-rows-warm");
        rows.steady(&mut harness, |harness| {
            harness.run_steps(3);
            shown(harness, "Bridge")
        })
        .expect("plan-rows-steady");
        // Review of PR 841 (P3): from the plan's entry through its paint, a quiet frame measures
        // no text at all: no action's natural width, no recipe's facts or layout, no fit, no
        // scene's lines (`design::TEXT_MEASURES`). Red: the rows' action widths measured outside
        // the guard.
        let measures = crate::test_support::CachedWork::start(
            "Plan text measures",
            &mut harness,
            |harness: &mut egui_kittest::Harness<'_, (Sessions, Book)>| {
                crate::test_support::Counts {
                    reached: harness.state().0.plan_rows.reached,
                    work: design::TEXT_MEASURES.with(std::cell::Cell::get),
                }
            },
        );
        harness.run_steps(2);
        let warm = shown(&harness, "Bridge");
        measures
            .warm(&mut harness, warm, 0)
            .expect("plan-measures-warm");
        measures
            .steady(&mut harness, |harness| {
                harness.run_steps(3);
                shown(harness, "Bridge")
            })
            .expect("plan-measures-steady");
        // Review of PR 841 (P2): a new font atlas at the same width, scale and theme lays the
        // rows out again, so no kept galley draws from the old atlas. egui makes its atlas again
        // for a new text alpha mode, and again when it comes back. Red: the key without it.
        rows.refresh(&mut harness, 1, |harness| {
            let alpha = harness.ctx.style().visuals.text_alpha_from_coverage;
            harness.ctx.all_styles_mut(|style| {
                style.visuals.text_alpha_from_coverage = egui::epaint::AlphaFromCoverage::Linear;
            });
            harness.step();
            harness
                .ctx
                .all_styles_mut(|style| style.visuals.text_alpha_from_coverage = alpha);
            harness.run_steps(2);
            shown(harness, "Bridge")
        })
        .expect("plan-rows-atlas");
        // Review of PR 703: the planned fights' days are worked out once for each change of
        // the note, the Timeline or the fights, never on a quiet frame or a resize.
        let days = || DAYS_BY_PLACE.with(std::cell::Cell::get);
        let before = days();
        harness.run_steps(3);
        harness.set_size(egui::vec2(design::FORM_WIDTH + 200.0, 800.0));
        harness.run_steps(3);
        assert_eq!(days(), before, "quiet frames work out no planned day");
        // Two scenes, each with its box; the plain list line has no row. Round 28 FD3: the
        // heading counts every row under it, a done scene too (it said 2 for these 3 rows).
        assert!(harness.query_by_label("Open: Captain Varra").is_some());
        assert!(
            harness
                .query_by_label("Done: The miller is the informer")
                .is_some()
        );
        assert!(harness.query_by_label_contains("Hobb").is_none());
        assert!(harness.query_by_label("TONIGHT 3").is_some());

        // An edit on disk a moment before is in the words that a tick changes. One character.
        let outside = read().replace(
            "- Hobb owes money\n",
            "- Hobb owes money\n- [ ] Pay the toll\n",
        );
        fs::write(&note.path, &outside).unwrap();
        model
            .refresh(&mut harness, 1, |harness| {
                harness.get_by_label("Open: Captain Varra").click();
                harness.run_steps(2);
                shown(harness, "Done: Captain Varra")
            })
            .expect("scene-model-refresh");
        let ticked = read();
        assert_eq!(ticked.len(), outside.len());
        assert_eq!(
            ticked
                .chars()
                .zip(outside.chars())
                .filter(|(a, b)| a != b)
                .count(),
            1
        );
        assert!(ticked.contains("- [x] [[Captain Varra]]\n- [x] The miller"));
        assert!(harness.query_by_label("Done: Captain Varra").is_some());
        assert!(harness.query_by_label("Open: Pay the toll").is_some());
        // A tick is its own Undo.
        harness.get_by_label("Done: Captain Varra").click();
        harness.run_steps(2);
        assert_eq!(read(), outside);

        // Remove from tonight takes the line out at once, with Undo remove in its place. An act
        // that leaves the note's words alone keeps that Undo, and it puts the line back.
        harness
            .get_by_label("More actions for Captain Varra")
            .click();
        harness.run_steps(2);
        harness.get_by_label("Remove from tonight").click();
        harness.run_steps(2);
        assert!(!read().contains("Captain Varra"));
        assert!(
            harness
                .query_by_label("Removed “Captain Varra” from tonight.")
                .is_some()
        );
        harness.state_mut().1.add(Kind::Day, Some(note.id));
        harness.run_steps(2);
        // The removal's result is a body message (outside a footer), on the same draw path as
        // a footer's line: its Undo's words and width are fitted once per change of the body
        // (`Body::body_kept`). From the page's entry through paint, an unchanged frame with it
        // measures no text. (Laid words are not counted here: the page's own Add actions lay
        // theirs every frame.) Red: a body message that fits its Undo every frame.
        assert!(
            harness
                .query_by_label("Undo remove Captain Varra")
                .is_some(),
            "body-reached: the result draws its Undo"
        );
        let measures = || design::TEXT_MEASURES.with(std::cell::Cell::get);
        let before = measures();
        harness.run_steps(5);
        assert_eq!(
            measures(),
            before,
            "body-quiet: an unchanged frame with a body message's Undo measures no text"
        );
        // M-N1.1: the Undo names the scene (`UndoStep::of`), as every place showing it does.
        harness.get_by_label("Undo remove Captain Varra").click();
        harness.run_steps(2);
        assert_eq!(read(), outside);
        assert_eq!(harness.state().1.events.len(), 1, "the other act stays");

        // App round 1. A scene's words are drawn at a plan row title's size; a done scene's are
        // muted, an open scene's are not (V1, V8).
        let painted = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, words: &str| {
            fn walk(shape: &egui::Shape, out: &mut Vec<(egui::Rect, Arc<egui::Galley>)>) {
                match shape {
                    egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
                    egui::Shape::Text(text) => {
                        out.push((
                            text.galley.rect.translate(text.pos.to_vec2()),
                            text.galley.clone(),
                        ));
                    }
                    _ => {}
                }
            }
            let mut all = Vec::new();
            for clipped in &harness.output().shapes {
                walk(&clipped.shape, &mut all);
            }
            all.into_iter()
                .filter(|(_, galley)| galley.job.text == words)
                .collect::<Vec<_>>()
        };
        let format = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, words: &str| {
            let found = painted(harness, words);
            assert_eq!(found.len(), 1, "{words} is painted once");
            found[0].1.job.sections[0].format.clone()
        };
        let c = colors();
        let done = format(&harness, "The miller is the informer");
        let open = format(&harness, "Pay the toll");
        assert_eq!(
            (done.color, open.color),
            (c.muted, c.text),
            "a done scene's words are muted, an open scene's are not"
        );
        assert_eq!(
            (done.font_id.size, open.font_id.size),
            (Type::Body.size(), Type::Body.size()),
            "a scene's words are drawn at a plan row title's size"
        );

        // W6-D1: the removal's Undo is the note's. A, the scene after the planned fight goes;
        // B, that fight is begun, so its row and the scene's place among the fights to come go;
        // the DM comes back to the session. Undo remove is in reach and puts back only the scene.
        harness
            .get_by_label("More actions for Captain Varra")
            .click();
        harness.run_steps(2);
        harness.get_by_label("Remove from tonight").click();
        harness.run_steps(2);
        let removed = "Removed “Captain Varra” from tonight.";
        // V12: a finished change has the success tone.
        let (words, _) = painted(&harness, removed)
            .into_iter()
            .next()
            .expect("the removal says so");
        assert!(
            painted(&harness, icons::SUCCESS)
                .iter()
                .any(|(icon, _)| icon.y_range().intersects(words.y_range())),
            "the removal's words have the success tone's icon"
        );
        let fight = harness.state_mut().1.add(Kind::Encounter, Some(note.id));
        {
            let book = &mut harness.state_mut().1;
            let event = book.events.iter_mut().find(|e| e.id == fight).unwrap();
            event.title = "Bridge".into();
            event.run = Some(Uuid::from_u128(7));
        }
        harness.state_mut().0.left_page();
        harness.run_steps(3);
        assert!(
            harness.query_by_label("Start fight: Bridge").is_none(),
            "the fight is played"
        );
        assert!(
            harness.query_all_by_label(removed).next().is_some(),
            "the removal's Undo stays in reach after the fight before it began"
        );
        // Ruling M9: the only Undo at its place shows its whole form where it fits.
        assert!(
            !painted(&harness, "Undo remove Captain Varra").is_empty(),
            "the removal's Undo shows the scene it puts back"
        );
        harness.get_by_label("Undo remove Captain Varra").click();
        harness.run_steps(2);
        assert_eq!(read(), outside, "Undo remove puts back only the scene");
        assert!(
            harness.state().1.events.iter().any(|e| e.id == fight),
            "the fight stays"
        );
        harness.state_mut().1.events.retain(|e| e.id != fight);
        harness.run_steps(2);
        // The same Undo while the scenes fold shows in the plan's footer.
        harness
            .get_by_label("More actions for Captain Varra")
            .click();
        harness.run_steps(2);
        harness.get_by_label("Remove from tonight").click();
        harness.run_steps(2);
        harness.set_size(egui::vec2(design::FORM_WIDTH, 200.0));
        harness.run_steps(4);
        assert!(
            harness.query_by_label("Open: Pay the toll").is_none()
                && harness.query_by_label("2 scenes").is_some(),
            "the scenes fold"
        );
        assert!(
            harness.query_all_by_label(removed).next().is_some(),
            "the removal's Undo stays in reach while the scenes fold"
        );
        // The footer's line of the same result names the scene too.
        harness.get_by_label("Undo remove Captain Varra").click();
        harness.run_steps(2);
        assert_eq!(read(), outside);
        // Review of PR 280: the closed rows decide the fold, and a result in the footer never
        // does. At the least height that shows the scenes, the day's `Undo add` comes into the
        // footer and no row changes: the scenes stay.
        let mut height = 200.0;
        while harness.query_by_label("Open: Pay the toll").is_none() {
            height += 8.0;
            assert!(height < 800.0, "the scenes show at some height");
            harness.set_size(egui::vec2(design::FORM_WIDTH, height));
            harness.run_steps(4);
        }
        let (id, label, count) = {
            let book = &harness.state().1;
            let event = book.events.last().unwrap();
            let day = *timeline::day_numbers(&book.events).last().unwrap();
            (event.id.clone(), event.label(day), book.events.len())
        };
        let added_at = 1000.0;
        crate::frame_clock::set(added_at);
        {
            let ctx = harness.ctx.clone();
            let page = &mut harness.state_mut().0;
            let added = kept::PlanAdded {
                owner: page.of_session(page.selected),
                id,
                label: label.clone(),
                to: None,
                count,
            };
            page.added.record_timed(added, &ctx);
        }
        harness.run_steps(4);
        assert!(
            harness.query_by_label("Undo add").is_some(),
            "the footer holds a result"
        );
        assert!(
            harness.query_by_label("Open: Pay the toll").is_some(),
            "a result in the footer does not fold the scenes"
        );
        // Round 30 B7d, ruling L9: the Add result and its Undo end by the frame clock, in the
        // owner update (`settle_timed`), not in the footer, which only presents the kept line.
        // The pane is drawn unchanged across the end, with no input: until it, each frame asks
        // for a repaint at the time left; at it, one frame drops the words, `Undo add` and the
        // footer's room together, and builds the line no more.
        let words = format!("{label} added.");
        let footer = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>| {
            let session = harness.state().0.selected;
            design::pane::footer_rect(
                &harness.ctx,
                egui::Id::new(("session_plan_footer", session)),
            )
        };
        let repaint = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>| {
            harness.output().viewport_output[&egui::ViewportId::ROOT].repaint_delay
        };
        let start = crate::design::results::projection_work();
        let stands = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>| {
            (
                harness.query_by_label("Undo add").is_some(),
                harness.query_all_by_label(&words).next().is_some(),
            )
        };
        // egui asks for the repaint one predicted frame early, so it lands at the end.
        let dt = f64::from(harness.ctx.input(|input| input.predicted_dt));
        for (at, left) in [(29.0, 1.0), (29.5, 0.5)] {
            crate::frame_clock::set(added_at + at);
            harness.step();
            assert_eq!(
                stands(&harness),
                (true, true),
                "{at} s of frames later the words and the Undo stand"
            );
            let delay = repaint(&harness).as_secs_f64();
            assert!(
                (delay - (left - dt)).abs() < 1e-3,
                "{at} s: the owner asks for a repaint at the end, {left} s on, not {delay}"
            );
        }
        let held = footer(&harness).expect("the result's footer is drawn");
        let quiet = crate::design::results::projection_work();
        assert_eq!(
            (quiet.built - start.built, quiet.bytes - start.bytes),
            (0, 0),
            "frames with no change build the kept line no more"
        );
        // The exact end: `frame_clock::passed` holds at 30 s.
        crate::frame_clock::set(added_at + 30.0);
        harness.step();
        assert_eq!(
            stands(&harness),
            (false, false),
            "30 s of frames later the words and the Undo are gone in one frame"
        );
        assert!(
            footer(&harness).is_none_or(|now| (now.height() - held.height()).abs() > 1.0),
            "in that frame the footer gives its room back: {held:?} -> {:?}",
            footer(&harness)
        );
        assert!(harness.state().0.added.is_empty(), "the owner ended it");
        harness.set_size(egui::vec2(design::FORM_WIDTH, 800.0));
        harness.run_steps(4);
        // An act that changes the note's words ends that Undo.
        harness
            .get_by_label("More actions for Captain Varra")
            .click();
        harness.run_steps(2);
        harness.get_by_label("Remove from tonight").click();
        harness.run_steps(2);
        harness.get_by_label("Open: Pay the toll").click();
        harness.run_steps(2);
        assert!(
            harness
                .query_by_label("Undo remove Captain Varra")
                .is_none()
        );
        assert!(read().contains("- [x] Pay the toll") && !read().contains("Captain Varra"));

        // Scene, in the strip's More list: Enter adds a task line at the end of the plan, keeps
        // the field open, and Undo scene takes the line back out.
        harness.get_by_label("More Add actions").click();
        harness.run_steps(2);
        harness.get_by_label("Add Scene").click();
        harness.run_steps(3);
        let before = read();
        harness
            .get_by_label("Plan a scene")
            .type_text("Meet the ferryman");
        harness.run_steps(1);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        assert!(read().contains("- [x] Pay the toll\n- [ ] Meet the ferryman\n\n## What happened"));
        // The footer's group carries the same words as the message in it.
        assert!(
            harness
                .query_all_by_label("Scene added to tonight.")
                .next()
                .is_some()
        );
        assert!(
            harness.query_by_label("Plan a scene").is_some(),
            "the field stays open"
        );
        harness.get_by_label("Undo add scene").click();
        harness.run_steps(2);
        assert_eq!(read(), before);
        // Another change of the words ends that Undo for good, also when the words come back.
        harness.get_by_label("Plan a scene").focus();
        harness.run_steps(1);
        harness
            .get_by_label("Plan a scene")
            .type_text("Cross at dawn");
        harness.run_steps(1);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        assert!(harness.query_by_label("Undo add scene").is_some());
        for state in ["Done", "Open"] {
            harness
                .get_by_label(&format!("{state}: Pay the toll"))
                .click();
            harness.run_steps(2);
        }
        assert!(harness.query_by_label("Undo add scene").is_none());
        // So does another session.
        harness.get_by_label("Plan a scene").focus();
        harness.run_steps(1);
        harness
            .get_by_label("Plan a scene")
            .type_text("Rest at the ford");
        harness.run_steps(1);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        assert!(harness.query_by_label("Undo add scene").is_some());
        harness.state_mut().0.select(None);
        harness.run_steps(2);
        harness.state_mut().0.select(Some(note.id));
        harness.run_steps(2);
        assert!(harness.query_by_label("Undo add scene").is_none());
        assert!(read().contains("- [ ] Cross at dawn\n- [ ] Rest at the ford\n"));
        // Review of PR 280: a refused Undo says why where the DM sees it. With the field closed,
        // `Undo scene` is in the footer; Obsidian changes the note just before the DM presses
        // it. The outside words stay, and the reason shows in the footer.
        let kept = read();
        assert!(
            harness.state().0.scenes.field.is_none(),
            "the field is closed"
        );
        harness
            .state_mut()
            .0
            .scene_act(SceneAct::Add("Wait for the tide".into()), Some(note.id));
        harness.run_steps(3);
        let obsidian = format!("{}\nObsidian added this just before Undo.\n", read());
        fs::write(&note.path, &obsidian).unwrap();
        harness.get_by_label("Undo add scene").click();
        harness.run_steps(3);
        assert_eq!(read(), obsidian, "the outside edit stays");
        assert!(
            harness
                .query_all_by_label(
                    "The scene “Wait for the tide” was not removed from Session 2: The Crossing. \
                     The session note changed after the act. Check the session note first."
                )
                .next()
                .is_some(),
            "a refused Undo scene says why after the field closed"
        );
        fs::write(&note.path, &kept).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.run_steps(2);

        // Unsaved typed words and a changed file: the act changes nothing and says why.
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            draft.body.push_str("Words being typed.\n");
            draft.edited();
        }
        let changed = read().replace("## What happened", "## What happened\n\nObsidian");
        fs::write(&note.path, &changed).unwrap();
        harness.get_by_label("Done: Pay the toll").click();
        harness.run_steps(2);
        assert_eq!(read(), changed);
        assert!(
            harness
                .query_by_label(
                    "The scene “Pay the toll” was not unticked in Session 2: The Crossing. The \
                     session note has unsaved words, and its file changed on disk. Choose which \
                     words to keep in the session note first."
                )
                .is_some()
        );

        // A failed save says so, and the tick waits in the draft.
        {
            let page = &mut harness.state_mut().0;
            page.draft = Some(Draft::open(&note.path).unwrap());
        }
        harness.run_steps(2);
        let mut permissions = fs::metadata(dir.path()).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o555);
        fs::set_permissions(dir.path(), permissions.clone()).unwrap();
        harness.get_by_label("Done: Pay the toll").click();
        harness.run_steps(2);
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
        fs::set_permissions(dir.path(), permissions).unwrap();
        assert_eq!(read(), changed);
        assert!(harness.query_by_label("Notes not saved.").is_some());
        assert!(
            harness
                .state()
                .0
                .draft
                .as_ref()
                .unwrap()
                .body
                .contains("- [ ] Pay the toll")
        );

        // After a restart, the note holds what the acts wrote.
        let reopened = Draft::open(&note.path).unwrap();
        let tasks: Vec<_> = plan_lines(&reopened.body)
            .into_iter()
            .filter_map(|line| match line {
                crate::sessions::PlanLine::Task(task) => Some((task.done, task.words)),
                crate::sessions::PlanLine::Links(_) => None,
            })
            .collect();
        assert_eq!(
            tasks,
            [
                (true, "The miller is the informer".to_owned()),
                (true, "Pay the toll".to_owned()),
                (false, "Cross at dawn".to_owned()),
                (false, "Rest at the ford".to_owned())
            ]
        );

        // Review of PR 261: a stale click names the line it was drawn from. Two scenes read the
        // same; Obsidian ticks the first; the DM clicks the first's old box. Nothing is written,
        // the second stays open, and the words show where the DM acted.
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 1,
                title: "Review",
                date: "2026-11-11",
                campaign: "Review",
                template: "## Plan for tonight\n\n- [ ] Talk\n- [ ] Talk\n",
            },
        )
        .unwrap();
        let mut page = Sessions {
            selected: Some(note.id),
            notes: vec![note.clone()],
            draft: Some(Draft::open(&note.path).unwrap()),
            ..Sessions::default()
        };
        let task = plan_lines(&page.draft.as_ref().unwrap().body)
            .into_iter()
            .find_map(|line| match line {
                PlanLine::Task(task) => Some(task),
                PlanLine::Links(_) => None,
            })
            .unwrap();
        let changed =
            fs::read_to_string(&note.path)
                .unwrap()
                .replacen("- [ ] Talk", "- [x] Talk", 1);
        fs::write(&note.path, &changed).unwrap();
        page.scene_act(SceneAct::Tick((0, 0), task), Some(note.id));
        assert_eq!(
            fs::read_to_string(&note.path).unwrap(),
            changed,
            "the first scene changed on disk; clicking it must not tick the second scene"
        );
        let fault = page.scenes.fault.as_ref().expect("the act says why");
        // M-N1.4: a refused scene act is a bounded refusal (ruling M7): the act, the scene and
        // the session, the cause and the next step.
        assert_eq!(
            (fault.what.cause().as_str(), fault.place),
            (
                "The scene “Talk” was not ticked in Session 1: Review. That line of the session \
                 note changed. Check the session note first.",
                Place::Row((0, 0))
            )
        );
        assert_eq!(
            page.draft.as_ref().unwrap().body.as_str(),
            crate::sessions::body_of(&changed)
        );

        // Review of PR 261: a scene added while the save is blocked keeps its Undo through the
        // retry that writes it, and that Undo takes the line back out of the file.
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 2,
                title: "Retry",
                date: "2026-11-11",
                campaign: "Review",
                template: "## Plan for tonight\n\n- [ ] First\n",
            },
        )
        .unwrap();
        let mut page = Sessions {
            selected: Some(note.id),
            notes: vec![note.clone()],
            draft: Some(Draft::open(&note.path).unwrap()),
            ..Sessions::default()
        };
        let before = fs::read_to_string(&note.path).unwrap();
        let mut permissions = fs::metadata(dir.path()).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o555);
        fs::set_permissions(dir.path(), permissions.clone()).unwrap();
        page.scene_act(SceneAct::Add("Second".into()), Some(note.id));
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
        fs::set_permissions(dir.path(), permissions).unwrap();
        assert_eq!(
            page.scenes.fault.as_ref().unwrap().what.cause().as_str(),
            "Notes not saved."
        );
        assert!(page.draft.as_ref().unwrap().body.contains("- [ ] Second"));
        page.draft.as_mut().unwrap().save();
        assert!(
            fs::read_to_string(&note.path)
                .unwrap()
                .contains("- [ ] Second")
        );
        assert!(
            page.scenes
                .added
                .as_ref()
                .is_some_and(|undo| undo.live(page.draft.as_ref().unwrap())),
            "a failed scene save followed by retry must keep Undo scene"
        );
        page.scenes.end_what_changed(page.draft.as_ref());
        assert!(
            page.scenes.fault.is_none(),
            "the save ends `Notes not saved`"
        );
        page.scene_act(SceneAct::UndoAdd, Some(note.id));
        assert_eq!(fs::read_to_string(&note.path).unwrap(), before);
        projection_follows_each_input();
    }

    /// The plan's inputs, which the projection cases below change one at a time.
    struct PlanInputs {
        fights: Arc<[PlanFight]>,
        groups: Vec<Arc<crate::encounter_run::RunGroup>>,
        fights_revision: crate::design::memo::Revision,
        saved: Vec<crate::encounter_run::SavedRunInfo>,
        saved_revision: crate::design::memo::Revision,
        index: Option<Arc<ContentIndex>>,
    }

    /// Review of PR 804 (a cache proof changes content, not shape): each input of the plan's
    /// projection, changed alone with the others' revisions kept, shows its new words on the next
    /// frame: a fight renamed with the same count, an event retitled and an event's day changed,
    /// a saved run that ends, a content index rebuild, and a switch to another session with the
    /// same revisions. Red runs: each input left out of `ProjectionInputs` once.
    fn projection_follows_each_input() {
        use crate::design::test_harness::Themed;
        use crate::sessions::{NewNote, create_note};
        use egui_kittest::kittest::Queryable;
        let root = tempfile::tempdir().unwrap();
        let note = |number, title| {
            create_note(
                root.path(),
                &NewNote {
                    number,
                    title,
                    date: "2026-11-01",
                    campaign: "Marches",
                    template: "## Plan for tonight\n\n- [[Bridge]]\n- [[Bridge]]\n- [[Mill]]\n- [ ] [[Captain Varra]]\n\n## What happened\n\n",
                },
            )
            .unwrap()
        };
        let (here, there) = (note(2, "The Crossing"), note(3, "The Ford"));
        let page = Sessions {
            selected: Some(here.id),
            notes: vec![here.clone(), there.clone()],
            folder: Some(root.path().to_owned()),
            draft: Some(Draft::open(&here.path).unwrap()),
            ..Sessions::default()
        };
        let still = crate::design::memo::Revised::new(()).revision();
        let inputs = std::rc::Rc::new(std::cell::RefCell::new(PlanInputs {
            fights: vec![
                planned_fight("Bridge", "Encounters/Bridge.md"),
                planned_fight("Ford", "Encounters/Ford.md"),
            ]
            .into(),
            groups: Vec::new(),
            fights_revision: still,
            saved: Vec::new(),
            saved_revision: still,
            index: None,
        }));
        let shared = std::rc::Rc::clone(&inputs);
        let mut book = Book::default();
        // A played fight in this session, with a title no planned fight has yet.
        let played = book.add(Kind::Encounter, Some(here.id));
        {
            let event = book.event_mut(&played).unwrap();
            event.title = "Scouts".into();
            event.run = Some(Uuid::from_u128(77));
        }
        let day = book.add(Kind::Day, Some(here.id));
        book.touch();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 900.0))
            .themed_state(
                move |ctx, (page, book): &mut (Sessions, Book)| {
                    let inputs = shared.borrow();
                    if let Some(index) = &inputs.index {
                        let index = Arc::clone(index);
                        ctx.data_mut(|data| {
                            data.insert_temp(egui::Id::new("reader_link_index"), index);
                        });
                    }
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &inputs.fights,
                        saved_runs: &inputs.saved,
                        saved_runs_revision: inputs.saved_revision,
                        fights_revision: inputs.fights_revision,
                        groups: &inputs.groups,
                        group_blocked: None,
                        group_open: false,
                        group_discard: &|_| None,
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
                    page.settle_timed(ctx, book);
                    egui::CentralPanel::default()
                        .show(ctx, |ui| page.plan_pane(ui, book, &plan, None));
                },
                (page, book),
            );
        harness.run_steps(3);
        let starts = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, name: &str| {
            harness
                .query_all_by_label(&format!("Start fight: {name}"))
                .count()
        };
        let has = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, words: &str| {
            harness.query_all_by_label_contains(words).next().is_some()
        };
        assert_eq!(
            starts(&harness, "Bridge"),
            2,
            "both Bridge lines are to come"
        );
        assert_eq!(starts(&harness, "Mill"), 0, "no fight is called Mill yet");
        // The fights and groups: a fight renamed (Ford to Mill, in the app and on disk), with the
        // same count and the note unchanged: the note's Mill line is a planned fight now.
        {
            let mut inputs = inputs.borrow_mut();
            inputs.fights = vec![
                planned_fight("Bridge", "Encounters/Bridge.md"),
                planned_fight("Mill", "Encounters/Mill.md"),
            ]
            .into();
            inputs.fights_revision = crate::design::memo::Revised::new(()).revision();
        }
        harness.step();
        assert_eq!(
            starts(&harness, "Mill"),
            1,
            "a renamed fight is planned on the next frame"
        );
        // The Timeline: the played fight retitled Bridge plays one Bridge line.
        {
            let book = &mut harness.state_mut().1;
            book.event_mut(&played).unwrap().title = "Bridge".into();
            book.touch();
        }
        harness.step();
        assert_eq!(
            starts(&harness, "Bridge"),
            1,
            "a retitled event plays its line"
        );
        // The Timeline: a day before this one makes it Day 2.
        assert!(has(&harness, "Day 1"));
        {
            let book = &mut harness.state_mut().1;
            let first = book.add(Kind::Day, Some(here.id));
            let at = book
                .events
                .iter()
                .position(|event| event.id == first)
                .unwrap();
            let moved = book.events.remove(at);
            book.events.insert(0, moved);
            book.touch();
        }
        harness.step();
        assert!(
            has(&harness, "Day 2") && book_has(&harness, &day),
            "a day before it makes this day Day 2 on the next frame"
        );
        // The played Bridge goes; the removal says the other Bridge lines stay.
        {
            let book = &mut harness.state_mut().1;
            book.remove(&played);
        }
        harness.run_steps(2);
        assert!(
            has(&harness, "other “Bridge”"),
            "the removal names the other Bridge"
        );
        // The saved runs, by the real path (reviews of PR 820 and PR 840): Bridge and Mill are
        // encounters of the campaign, and Bridge's run is saved unfinished in its run folder.
        // `Encounters` reads the folder and `plan_kept` lists Bridge with its run; the plan
        // offers Resume. Then the run ends (the same run, saved as ended): read again, Bridge
        // has no unfinished run, and the plan offers Start fight again on the next frame.
        let campaign = crate::persistence::Campaign {
            id: Uuid::new_v4(),
            name: "Marches".into(),
            path: root.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        crate::content_workspace::CampaignDocument::new(&campaign)
            .unwrap()
            .save()
            .unwrap();
        let bridge = Uuid::new_v4();
        for (id, name) in [(bridge, "Bridge"), (Uuid::new_v4(), "Mill")] {
            fs::write(
                root.path().join(format!("{name}.md")),
                splat::content::formats::fixture(
                    "Fixture.md",
                    &format!("splat_id: {id}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter: {{roster: []}}\n"),
                    "",
                )
                .unwrap(),
            )
            .unwrap();
        }
        let wolf = || {
            let fields = format!(
                "splat_id: {}\nsplat_kind: creature\nsplat_name: Wolf\nsplat_creature: {{type: beast, cr_text: \"1/4\"}}\nsplat_stats:\n  max_hp: {{value: 11}}\n  armor_class: {{value: 13}}\n",
                Uuid::new_v4()
            );
            let text =
                splat::content::formats::fixture("Fixture.md", &fields, "\n## Actions\nBite.\n");
            let entry = splat::content::Entry::parse("creature.md", &text.unwrap()).unwrap();
            splat_campaign::encounter_run::Participant::new(
                "wolf".into(),
                "enemies".into(),
                entry,
                &serde_yaml_ng::Mapping::new(),
                true,
            )
            .unwrap()
        };
        let runs = root.path().join("Encounter Runs");
        fs::create_dir_all(&runs).unwrap();
        splat_encounters_ui::encounter_run::write_in_progress_run(
            campaign.id,
            &runs,
            "Bridge",
            vec![(wolf(), 12), (wolf(), 8)],
        )
        .unwrap();
        // The run came from Bridge: its file names the encounter, as a run started from it does.
        let file = fs::read_dir(&runs).unwrap().next().unwrap().unwrap().path();
        let mut saved: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&file).unwrap()).unwrap();
        saved["source"] = bridge.to_string().into();
        fs::write(&file, saved.to_string()).unwrap();
        let mut encounters = splat_encounters_ui::encounters::Encounters::default();
        // The test reads the run folder itself; `poll` notes the run facts, as each frame.
        encounters.hold_reads = true;
        encounters.refresh_now(&campaign, &[]);
        let campaigns = splat_content_ui::campaigns::Campaigns::default();
        let ctx = harness.ctx.clone();
        let read = |encounters: &mut splat_encounters_ui::encounters::Encounters| {
            encounters.poll(&ctx, &campaigns);
            let kept = encounters.plan_kept();
            let mut inputs = inputs.borrow_mut();
            inputs.fights = kept.fights;
            inputs.fights_revision = encounters.plan_fights_revision();
            inputs.saved_revision = encounters.saved_runs_revision();
        };
        read(&mut encounters);
        harness.step();
        assert!(
            harness.query_all_by_label("Resume: Bridge").count() > 0,
            "a fight with its unfinished run offers Resume"
        );
        let fights_revision = inputs.borrow().fights_revision;
        fs::remove_file(&file).unwrap();
        splat_encounters_ui::encounter_run::write_ended_run_with(
            campaign.id,
            &runs,
            "Bridge",
            Some(bridge),
            "Won",
            vec![wolf()],
            "# Bridge",
        )
        .unwrap();
        encounters.read_runs_again(&campaign);
        read(&mut encounters);
        assert_eq!(
            inputs.borrow().fights_revision,
            fights_revision,
            "only the saved runs changed"
        );
        harness.step();
        assert!(
            harness.query_all_by_label("Resume: Bridge").count() == 0
                && starts(&harness, "Bridge") > 0,
            "a run that ended: the plan offers Start fight again on the next frame"
        );
        // The content index, one built index for another (review of PR 820): with a Captain
        // Varra note the scene finds her; the note deleted and the index built again (another
        // generation, an index all the same), the scene says `Not found`.
        let varra = root.path().join("Captain Varra.md");
        fs::write(
            &varra,
            format!(
                "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: rule\nsplat_name: Captain Varra\nsplat_rule: {{}}\n---\n\nThe captain of the ford.\n",
                Uuid::new_v4()
            ),
        )
        .unwrap();
        inputs.borrow_mut().index = Some(Arc::new(
            crate::lookup::ContentIndex::load(&[], Some(&campaign)).unwrap(),
        ));
        harness.step();
        assert!(
            !has(&harness, "Not found"),
            "the first index finds Captain Varra"
        );
        fs::remove_file(&varra).unwrap();
        inputs.borrow_mut().index = Some(Arc::new(
            crate::lookup::ContentIndex::load(&[], Some(&campaign)).unwrap(),
        ));
        harness.step();
        assert!(
            has(&harness, "Not found"),
            "an index built again shows on the next frame"
        );
        // The session: another session with the same draft and revisions has none of this
        // session's events: its heading counts its rows (its 3 planned fights and its scene),
        // not this one's (its 2 days too).
        let heading = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, count: usize| {
            has(harness, &format!("TONIGHT {count}"))
                || has(harness, &format!("THIS SESSION {count}"))
        };
        assert!(
            heading(&harness, 6),
            "this session's heading counts its 6 rows"
        );
        harness.state_mut().0.selected = Some(there.id);
        harness.step();
        assert!(
            heading(&harness, 4),
            "another session's heading counts its own rows on the next frame"
        );
        // Layout 11c A2c: the plan's shared column comes from every closed row, laid out before
        // any row draws. In one pass, with no second pass to fix a column kept from the last
        // one, a longer title moves every row's action: all stand in one column, `L` after the
        // widest facts. Red: the column from the last pass.
        harness.state_mut().0.selected = Some(here.id);
        harness.set_size(egui::vec2(1200.0, 900.0));
        harness.run_steps(3);
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        let column = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>| {
            let records: Vec<_> = design::action_rows::row_records(&harness.ctx)
                .into_iter()
                .filter(|record| record.owner == "plan")
                .collect();
            let widest = records
                .iter()
                .map(|record| record.facts.right())
                .fold(f32::MIN, f32::max);
            let holds = records.len() >= 4
                && records.iter().all(|record| {
                    !record.wrapped
                        && (record.column - (widest + design::L)).abs() <= 0.5
                        && record.action.left() >= record.column - 0.5
                });
            (holds, widest, records)
        };
        let (holds, before, records) = column(&harness);
        assert!(holds, "one column before: {records:?}");
        {
            let book = &mut harness.state_mut().1;
            let long = book.add(Kind::Encounter, Some(here.id));
            book.event_mut(&long).unwrap().title =
                "The long road along the river to the old mill below the burning shrine".into();
            book.touch();
        }
        harness.step();
        let (holds, after, records) = column(&harness);
        assert!(
            holds && after > before + design::L,
            "in one pass every row's action moves to the new column, L after the widest facts \
             ({before} to {after}): {records:?}"
        );
        // The groups (review of PR 841): a group with no members and so no member fault, linked
        // in the note, is a row laid by its recipe on the next frame, and keeps its own reason,
        // `No members can join.`, as a warning under its row. Red: the recipe row with no reason.
        {
            let mut inputs = inputs.borrow_mut();
            inputs.groups = vec![Arc::new(crate::encounter_run::RunGroup {
                id: Uuid::new_v4(),
                path: "Encounter Groups/Empty Watch.md".into(),
                name: "Empty Watch".into(),
                side: "enemies".into(),
                encounter: None,
                encounter_name: None,
                members: vec![],
                notes: String::new(),
                xp: 0,
                xp_unknown: vec![],
                unavailable: 0,
                unavailable_members: vec![],
                problem: Some("No members can join.".into()),
                warnings: vec![],
            })];
            inputs.fights_revision = crate::design::memo::Revised::new(()).revision();
        }
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let body = draft
                .body
                .as_str()
                .replace("- [[Mill]]\n", "- [[Mill]]\n- [[Empty Watch]]\n");
            let _ = draft.body.replace(body);
            draft.edited();
        }
        harness.step();
        assert!(
            harness.query_by_label("Empty Watch").is_some() && has(&harness, "No members can join"),
            "an empty group's row says why it cannot join"
        );
        // Review of PR 841 (P3): a planned fight's row and its title are keyed by the fight's
        // file and its occurrence, never by its place: a fight planned above Mill keeps Mill's
        // title focused. Red: keyed by place.
        harness.get_by_label("Mill").focus();
        harness.run_steps(2);
        assert!(harness.get_by_label("Mill").is_focused());
        let bridges = harness.query_all_by_label("Bridge").count();
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let body =
                draft
                    .body
                    .as_str()
                    .replacen("- [[Mill]]\n", "- [[Bridge]]\n- [[Mill]]\n", 1);
            let _ = draft.body.replace(body);
            draft.edited();
        }
        harness.run_steps(2);
        assert!(
            harness.query_all_by_label("Bridge").count() == bridges + 1
                && harness.get_by_label("Mill").is_focused(),
            "a fight planned above Mill keeps Mill's title focused"
        );
        // Review of PR 841 (P2): the kept rows are keyed by the column the scenes measured too.
        // A scene wider than every row moves every kept row's action in the frame it widens (its
        // redraw pass): all stand in one column, `L` after the scene's facts. Red: the key
        // without the scenes' column.
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(2).unwrap());
        let (_, before, _) = column(&harness);
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let body = draft.body.as_str().replacen(
                "- [ ] [[Captain Varra]]",
                "- [ ] [[Captain Varra]] waits at the ford with the toll collectors, the miller's ledger and the ferryman's boat",
                1,
            );
            let _ = draft.body.replace(body);
            draft.edited();
        }
        harness.step();
        let (holds, after, records) = column(&harness);
        assert!(
            holds && after > before + design::L,
            "a wider scene moves every row's action in the frame it widens ({before} to \
             {after}): {records:?}"
        );
        // The same for a group with a member fault, drawn by `plan_row_with` until scope B: its
        // wider title moves every row's action in the frame it joins the plan.
        let before = after;
        {
            let mut inputs = inputs.borrow_mut();
            inputs.groups.push(Arc::new(crate::encounter_run::RunGroup {
                id: Uuid::new_v4(),
                path: "Encounter Groups/Watch.md".into(),
                name: "The watch at the ford, with the toll collectors, the ferryman, the miller's two sons, their dogs and the old bridge keeper".into(),
                side: "enemies".into(),
                encounter: None,
                encounter_name: None,
                members: vec![],
                notes: String::new(),
                xp: 0,
                xp_unknown: vec![],
                unavailable: 1,
                unavailable_members: vec![],
                problem: Some("Wolf: this campaign's packages have no such creature.".into()),
                warnings: vec![splat_content_ui::roster::MemberFault::new(
                    splat_content_ui::roster::FaultKind::NotInPackages,
                    "Wolf: this campaign's packages have no such creature.".into(),
                )],
            }));
            inputs.fights_revision = crate::design::memo::Revised::new(()).revision();
        }
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let body = draft.body.as_str().replacen(
                "- [[Empty Watch]]\n",
                "- [[Empty Watch]]\n- [[The watch at the ford, with the toll collectors, the ferryman, the miller's two sons, their dogs and the old bridge keeper]]\n",
                1,
            );
            let _ = draft.body.replace(body);
            draft.edited();
        }
        harness.step();
        let (holds, after, records) = column(&harness);
        assert!(
            holds && after > before + design::L,
            "a wider group with a member fault moves every row's action in the frame it joins \
             ({before} to {after}): {records:?}"
        );
    }

    /// Layout 11c A2c-3: while every row the plan draws is a closed row, its body is its
    /// measured region, drawn from the region's mask; an opened row draws the rows in place. A
    /// row opened, closed or deleted, and a session switch (in one pass and in two), move no row
    /// that stays, and each fight has one accessible node. Red: the region's blocks without the
    /// column's gap (`PlanRow::tail`).
    #[test]
    fn the_plan_body_is_its_region_while_every_row_is_closed() {
        use crate::design::test_harness::Themed as _;
        use crate::sessions::{NewNote, create_note};
        use egui_kittest::kittest::Queryable;
        let root = tempfile::tempdir().unwrap();
        let note = |number, title, template| {
            create_note(
                root.path(),
                &NewNote {
                    number,
                    title,
                    date: "2026-11-01",
                    campaign: "Marches",
                    template,
                },
            )
            .unwrap()
        };
        let here = note(
            2,
            "The Crossing",
            "## Plan for tonight\n\n- [[Bridge]]\n- [[Mill]]\n- [[Ford]]\n\n## What happened\n\n",
        );
        let there = note(
            3,
            "The Ford",
            "## Plan for tonight\n\n- [[Ford]]\n\n## What happened\n\n",
        );
        let page = Sessions {
            selected: Some(here.id),
            notes: vec![here.clone(), there.clone()],
            folder: Some(root.path().to_owned()),
            draft: Some(Draft::open(&here.path).unwrap()),
            ..Sessions::default()
        };
        let fights = [
            planned_fight("Bridge", "Encounters/Bridge.md"),
            planned_fight("Mill", "Encounters/Mill.md"),
            planned_fight("Ford", "Encounters/Ford.md"),
        ];
        let still = crate::design::memo::Revised::new(()).revision();
        let mut book = Book::default();
        book.add(Kind::Day, Some(here.id));
        book.touch();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 700.0))
            .themed_state(
                move |ctx, (page, book): &mut (Sessions, Book)| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &fights,
                        saved_runs: &[],
                        saved_runs_revision: still,
                        fights_revision: still,
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discard: &|_| None,
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
                    page.settle_timed(ctx, book);
                    egui::CentralPanel::default()
                        .show(ctx, |ui| page.plan_pane(ui, book, &plan, None));
                },
                (page, book),
            );
        harness.run_steps(3);
        let top = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, name: &str| {
            harness.get_by_label(name).rect().top()
        };
        let measured =
            |harness: &egui_kittest::Harness<'_, (Sessions, Book)>| harness.state().0.plan_measured;
        assert!(measured(&harness), "closed rows: the body is its region");
        for name in ["Bridge", "Mill", "Ford"] {
            assert_eq!(
                harness.query_all_by_label(name).count(),
                1,
                "one accessible node for {name}"
            );
        }
        // Each row the region draws whole is registered as a row (the pane's whole-row cut, and
        // the gate's row checks). Red: a measured body that registers none.
        let rows: Vec<egui::Rect> = harness
            .ctx
            .data(|data| data.get_temp::<(u64, Vec<egui::Rect>)>(egui::Id::new(design::ROW_RECTS)))
            .map(|(_, rows)| rows)
            .unwrap_or_default();
        for name in ["Bridge", "Mill", "Ford"] {
            let title = harness.get_by_label(name).rect();
            assert!(
                rows.iter().any(|row| row.contains_rect(title)),
                "{name}'s row is registered: {rows:?}"
            );
        }
        // Review of PR 869 (P2): a row keeps its controls' ids in both modes, so a focused
        // control keeps its focus while the body switches to the rows in place and back. Red:
        // the region's own block scope.
        harness.get_by_label("Start fight: Ford").focus();
        harness.run_steps(2);
        assert!(harness.get_by_label("Start fight: Ford").is_focused());
        harness.state_mut().0.open_planned = Some("Encounters/Mill.md".into());
        harness.run_steps(2);
        assert!(
            !measured(&harness) && harness.get_by_label("Start fight: Ford").is_focused(),
            "a focused control keeps its focus when the rows draw in place"
        );
        harness.state_mut().0.open_planned = None;
        harness.run_steps(2);
        assert!(
            measured(&harness) && harness.get_by_label("Start fight: Ford").is_focused(),
            "and when they are the region again"
        );
        let before = ["Bridge", "Mill", "Ford"].map(|name| top(&harness, name));
        // The first open: the rows draw in place, and the rows above the opened one and the
        // opened one keep their place.
        harness.get_by_label("Mill").click();
        harness.run_steps(2);
        assert!(!measured(&harness), "an opened row: the rows draw in place");
        assert!(
            (top(&harness, "Bridge") - before[0]).abs() < 0.5
                && (top(&harness, "Mill") - before[1]).abs() < 0.5,
            "an open moves no row above it: {before:?} -> {:?}",
            ["Bridge", "Mill"].map(|name| top(&harness, name))
        );
        // The last close: the region again, every row where it was.
        harness.get_by_label("Mill").click();
        harness.run_steps(2);
        let closed = ["Bridge", "Mill", "Ford"].map(|name| top(&harness, name));
        assert!(
            measured(&harness)
                && closed
                    .iter()
                    .zip(before)
                    .all(|(now, then)| (now - then).abs() < 0.5),
            "the last close: the region, every row where it was: {before:?} -> {closed:?}"
        );
        // A deletion: Mill leaves the plan, and Ford takes its place.
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let body = draft.body.as_str().replacen("- [[Mill]]\n", "", 1);
            let _ = draft.body.replace(body);
            draft.edited();
        }
        harness.run_steps(2);
        assert!(
            measured(&harness)
                && harness.query_by_label("Mill").is_none()
                && (top(&harness, "Ford") - before[1]).abs() < 0.5,
            "a deleted row: the next row takes its place"
        );
        // A session switch, in one pass and in two: the other session's rows, at once.
        for passes in [1, 2] {
            harness.ctx.options_mut(|options| {
                options.max_passes = std::num::NonZeroUsize::new(passes).unwrap();
            });
            harness.state_mut().0.selected = Some(there.id);
            harness.state_mut().0.draft = Some(Draft::open(&there.path).unwrap());
            harness.step();
            assert!(
                measured(&harness)
                    && harness.query_by_label("Bridge").is_none()
                    && harness.query_all_by_label("Ford").count() == 1,
                "{passes} pass(es): the other session's rows"
            );
            harness.state_mut().0.selected = Some(here.id);
            harness.state_mut().0.draft = Some(Draft::open(&here.path).unwrap());
            harness.step();
            assert!(
                measured(&harness) && harness.query_all_by_label("Bridge").count() == 1,
                "{passes} pass(es): this session's rows again"
            );
        }
        // An added row below the view (M0500): the region shows it whole (`Navigation::Show` of
        // the row after it), and only rows whole in view have an accessible node.
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(2).unwrap());
        {
            let book = &mut harness.state_mut().1;
            for _ in 0..8 {
                book.add(Kind::Day, Some(here.id));
            }
            book.touch();
        }
        harness.set_size(egui::vec2(design::FORM_WIDTH, 420.0));
        harness.run_steps(3);
        assert!(
            measured(&harness) && harness.query_by_label("Long rest").is_none(),
            "no long rest yet"
        );
        harness.get_by_label("Add Long rest").click();
        harness.run_steps(4);
        assert!(
            measured(&harness) && harness.query_by_label("Long rest").is_some(),
            "the added row is brought into view in the region"
        );
        // Review of PR 869 (869-a), a row of this proof: the last scene removed while a closed
        // fight remains, after the switches above. Its removal result and its Undo are reachable
        // where they always show (among the rows); the DM presses Undo by its label, and the
        // scene comes back beside the same fight. Red: the measured mode, which left the result
        // out.
        harness.set_size(egui::vec2(design::FORM_WIDTH, 1400.0));
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let body =
                draft
                    .body
                    .as_str()
                    .replacen("- [[Ford]]\n", "- [[Ford]]\n- [ ] Pay the toll\n", 1);
            let _ = draft.body.replace(body);
            draft.edited();
        }
        harness.run_steps(2);
        assert!(
            !measured(&harness) && harness.query_by_label("Open: Pay the toll").is_some(),
            "a scene row draws the rows in place"
        );
        let ford = top(&harness, "Ford");
        let scene = top(&harness, "Open: Pay the toll");
        harness
            .get_by_label("More actions for Pay the toll")
            .click();
        harness.run_steps(2);
        harness.get_by_label("Remove from tonight").click();
        harness.run_steps(3);
        assert!(
            harness.query_by_label("Open: Pay the toll").is_none()
                && harness
                    .query_by_label_contains("Removed “Pay the toll” from tonight.")
                    .is_some(),
            "the last scene's removal says so"
        );
        harness.get_by_label("Undo remove Pay the toll").click();
        harness.run_steps(3);
        assert!(
            harness.query_by_label("Open: Pay the toll").is_some()
                && (top(&harness, "Ford") - ford).abs() < 0.5
                && (top(&harness, "Open: Pay the toll") - scene).abs() < 0.5,
            "Undo brings the scene back beside the same fight"
        );
    }

    fn book_has(harness: &egui_kittest::Harness<'_, (Sessions, Book)>, id: &str) -> bool {
        harness.state().1.events.iter().any(|event| event.id == id)
    }

    /// F7, rule 2: `Add rewards` goes on a played row only when the title keeps its width and
    /// its line breaks, laid out as `wrapped_link` lays it out. Two lines are not enough.
    #[test]
    fn the_rewards_fit_keeps_each_title_as_it_is() {
        use crate::design::test_harness::Themed as _;
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(500.0, 800.0))
            .themed_state(
                |ctx, faults: &mut Vec<String>| {
                    faults.clear();
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.set_width(330.0);
                        // X48: beside the note the page gives the plan its column gap, `L`; the
                        // links keep `S`, so a title with no room on its row has `Add rewards` as
                        // the opened row's last link at the 330 px plan, not in the menu.
                        ui.spacing_mut().item_spacing.x = design::L;
                        let mut rewards = Rewards::default();
                        let long = "Wolves at the Ford Below the Burning Shrine Road";
                        let place = rewards.fits(ui, long, icons::NEXT, 330.0, Action::quiet(ADD_REWARDS)
                            .density(Density::Read)
                            .natural_width(ui)
                            + Density::Read.control()
                            + design::S);
                        if place != RewardsAt::Link {
                            faults.push(format!("{long:?} has Add rewards at {place:?}"));
                        }
                        let band = Action::quiet(ADD_REWARDS)
                            .density(Density::Read)
                            .natural_width(ui)
                            + Density::Read.control()
                            + design::S;
                        let words: Vec<_> = "Ambush of the caravan at the abandoned bridge over the river beside the mill at dawn"
                            .split_whitespace()
                            .collect();
                        let titles = (3..=words.len())
                            .map(|n| words[..n].join(" "))
                            .chain((3..18).map(|n| format!("{}at the bridge", "Wolf ".repeat(n))));
                        // How many titles take the action on the row, and how many do not.
                        let mut placed = [0, 0];
                        for title in titles {
                            let on_row =
                                rewards.fits(ui, &title, icons::NEXT, 330.0, band) == RewardsAt::Row;
                            placed[usize::from(on_row)] += 1;
                            if !on_row {
                                continue;
                            }
                            let galley = |text: egui::RichText, room| {
                                egui::WidgetText::from(text).into_galley(
                                    ui,
                                    Some(egui::TextWrapMode::Wrap),
                                    room,
                                    egui::TextStyle::Body,
                                )
                            };
                            let column = 330.0
                                - galley(Type::Body.rich(ui, icons::NEXT), f32::INFINITY).size().x
                                - design::S;
                            let measure = |action: f32| {
                                galley(
                                    Type::Body.rich(ui, &title).underline(),
                                    (column - action - design::S).max(design::XL).max(1.0),
                                )
                                .size()
                            };
                            let (before, after) = (measure(Density::Read.control()), measure(band));
                            if (before - after).length() > 0.5 {
                                faults.push(format!("{title:?}: {before:?} -> {after:?}"));
                            }
                        }
                        if placed.contains(&0) {
                            faults.push(format!("off the row and on it: {placed:?}"));
                        }
                    });
                },
                Vec::<String>::new(),
            );
        harness.run_steps(2);
        assert!(
            harness.state().is_empty(),
            "a title of today changed its size: {:?}",
            harness.state()
        );
    }

    /// F7, T18.1: `Add rewards` writes one block in the session note through the note's one act
    /// path, with its own Undo kept by the note and the fight.
    #[test]
    fn rewards_are_a_note_act_with_their_own_undo() {
        use crate::design::test_harness::Themed as _;
        use crate::sessions::{NewNote, create_note};
        use egui_kittest::kittest::Queryable;
        use splat_campaign::encounter_run::Participant;
        use splat_encounters_ui::encounter_run::write_ended_run_with;
        let root = tempfile::tempdir().unwrap();
        let (sessions, runs) = (root.path().join("Sessions"), root.path().join("Runs"));
        fs::create_dir_all(&sessions).unwrap();
        let campaign = Uuid::new_v4();
        let creature = |name: &str, cr: &str| {
            let fields = format!(
                "splat_id: {}\nsplat_kind: creature\nsplat_name: {name}\nsplat_creature: {{type: beast, cr_text: \"{cr}\"}}\nsplat_stats:\n  max_hp: {{value: 11}}\n  armor_class: {{value: 13}}\n",
                Uuid::new_v4()
            );
            let text =
                splat::content::formats::fixture("Fixture.md", &fields, "\n## Actions\nBite.\n");
            splat::content::Entry::parse("creature.md", &text.unwrap()).unwrap()
        };
        let joined = |key: &str, entry| {
            Participant::new(
                key.into(),
                "enemies".into(),
                entry,
                &serde_yaml_ng::Mapping::new(),
                true,
            )
            .unwrap()
        };
        // Each fight ends with its first creature at 0 HP, as the run file holds it.
        let ended = |name: &str, details: &str, participants| {
            let fact =
                write_ended_run_with(campaign, &runs, name, None, "Won", participants, details)
                    .unwrap();
            let path = crate::encounter_run::find_run(&runs, campaign, fact.id).unwrap();
            let mut saved: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            saved["state"]["participants"][0]["hp"] = 0.into();
            fs::write(&path, saved.to_string()).unwrap();
            fact
        };
        let bridge = ended(
            "Bridge",
            "# Bridge\n\n## Loot\n\n- 30 gp\n",
            vec![
                joined("wolf", creature("Wolf", "1 (XP 200; PB +2)")),
                joined("wolf2", creature("Wolf", "1 (XP 200; PB +2)")),
            ],
        );
        // Review of PR 645: Bridge's log is long, and its newest entry is a long DM note. The run
        // file holds the log oldest first.
        {
            let path = crate::encounter_run::find_run(&runs, campaign, bridge.id).unwrap();
            let mut saved: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            let mut log: Vec<String> = (1..=30)
                .map(|round| format!("Round {round}: the Wolf bites for {round}."))
                .collect();
            log.push(format!(
                "Note: {}",
                "The bridge creaks under the weight of the whole party. ".repeat(12)
            ));
            saved["state"]["log"] = log.into();
            fs::write(&path, saved.to_string()).unwrap();
        }
        let shrine = ended(
            "Shrine",
            "# Shrine",
            vec![joined("varra", creature("Captain Varra", ""))],
        );
        let note = create_note(
            &sessions,
            &NewNote {
                number: 2,
                title: "The Crossing",
                date: "2026-11-01",
                campaign: "Marches",
                template: "## What happened\n\n## Loot and rewards\n\n## Open threads\n\n",
            },
        )
        .unwrap();
        let mut book = Book::default();
        book.open(Some(&sessions));
        book.record_run(&bridge, Some(note.id), false);
        book.record_run(&shrine, Some(note.id), false);
        let id = |name: &str| {
            book.events
                .iter()
                .find(|event| event.title == name)
                .unwrap()
                .id
                .clone()
        };
        let (a, b) = (id("Bridge"), id("Shrine"));
        let read = || fs::read_to_string(&note.path).unwrap();
        let page = Sessions {
            selected: Some(note.id),
            notes: vec![note.clone()],
            folder: Some(sessions.clone()),
            draft: Some(Draft::open(&note.path).unwrap()),
            campaign: Some(crate::persistence::Campaign {
                id: campaign,
                name: "Marches".into(),
                path: root.path().to_owned(),
                dndbeyond_campaign_id: None,
                dndbeyond_campaign_name: None,
            }),
            ..Sessions::default()
        };
        // The saved runs' facts, which these frames never change.
        let no_runs = crate::design::memo::Revised::new(()).revision();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 800.0))
            .themed_state(
                |ctx, (page, book): &mut (Sessions, Book)| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &[],
                        saved_runs: &[],
                        saved_runs_revision: no_runs,
                        fights_revision: no_runs,
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discard: &|_| None,
                        arrivals_open: false,
                        group_editor: std::cell::RefCell::new(&mut group),
                        blocked: None,
                        run_directory: Some(page.folder.clone().unwrap().with_file_name("Runs")),
                        open_run: None,
                        open_state: String::new(),
                        open_encounter: None,
                        party: &[],
                        tables: &[],
                    };
                    egui::CentralPanel::default()
                        .show(ctx, |ui| page.plan_pane(ui, book, &plan, None));
                },
                (page, book),
            );
        harness.run_steps(2);
        // The footer's scroll part is named by its messages too, so a message can match twice.
        let shown = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, words: &str| {
            harness.query_all_by_label_contains(words).next().is_some()
        };
        let block_a = format!(
            "- [[Timeline#^{a}|Bridge]]\n  - XP 200 for 1 at 0 HP · 1 still stood (XP 200)\n  - 30 gp\n"
        );
        let block_b = format!(
            "- [[Timeline#^{b}|Shrine]]\n  - XP 0. XP not known for Captain Varra: enter an award here.\n"
        );

        // An edit on disk a moment before is in the words that the act changes. With no block
        // in any note, the block has XP and the result says how to stop it. The row then says
        // `In notes`, and the note beside the plan is asked to show the block.
        let outside = read().replace("## What happened\n", "## What happened\n\nThe ford held.\n");
        fs::write(&note.path, &outside).unwrap();
        let next_row = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>| {
            harness.get_by_label("Add rewards: Shrine").rect()
        };
        let shrine_before = next_row(&harness);
        harness.get_by_label("Add rewards: Bridge").click();
        harness.run_steps(2);
        // Round 27 FN1, FK3, SK2: the result is in the plan's footer, so the next row's action
        // keeps its place.
        assert_eq!(
            next_row(&harness),
            shrine_before,
            "the rewards result moved the next row"
        );
        let with_a = outside.replace(
            "## Loot and rewards\n\n",
            &format!("## Loot and rewards\n\n{block_a}\n"),
        );
        assert_eq!(read(), with_a);
        assert!(shown(
            &harness,
            "Rewards for “Bridge” added under Loot and rewards. XP line added. Delete it in the note to leave XP out from now on. XP tonight: 200 for 1 fight."
        ));
        assert!(harness.query_by_label("Add rewards: Bridge").is_none());
        assert!(shown(&harness, "In notes"));
        assert!(harness.state().0.caret_to_show.is_some());
        // Review of PR 296, item 1: the footer shows the result by the name the act read; an
        // unchanged render reads no fight's name from the Timeline again.
        let reads = || FIGHT_NAME_READS.with(std::cell::Cell::get);
        let after_act = reads();
        harness.run_steps(4);
        assert!(shown(&harness, "Rewards for “Bridge”"));
        assert_eq!(
            reads(),
            after_act,
            "a render searched the Timeline for a fight's name"
        );

        // A: Bridge. B: Shrine, another fight. It follows the newest block, which has XP, and
        // names the award that is not known, which the sum leaves out. A's Undo stays.
        harness.state_mut().0.caret_to_show = None;
        harness.get_by_label("Add rewards: Shrine").click();
        harness.run_steps(2);
        let with_both = with_a.replace(&block_a, &format!("{block_a}{block_b}"));
        assert_eq!(read(), with_both);
        assert!(shown(
            &harness,
            "XP 0, as in your last rewards. XP tonight: 200 for 2 fights. 1 award is not known."
        ));
        assert_eq!(harness.query_all_by_label("Undo add rewards").count(), 2);
        // The footer lists the newest result first: B, then A.
        let top = |words: &str| {
            harness
                .query_all_by_label_contains(words)
                .map(|node| node.rect().top())
                .reduce(f32::max)
        };
        let (newest, oldest) = (top("Rewards for “Shrine”"), top("Rewards for “Bridge”"));
        assert!(
            newest
                .zip(oldest)
                .is_some_and(|(newest, oldest)| newest < oldest),
            "newest first: {newest:?} {oldest:?}"
        );
        // Undo of A (the footer's second) takes back A's block alone; B's stays.
        harness
            .query_all_by_label("Undo add rewards")
            .last()
            .unwrap()
            .click();
        harness.run_steps(2);
        assert_eq!(
            read(),
            outside.replace(
                "## Loot and rewards\n\n",
                &format!("## Loot and rewards\n\n{block_b}\n")
            )
        );
        assert!(harness.query_by_label("Add rewards: Bridge").is_some());
        assert_eq!(harness.query_all_by_label("Undo add rewards").count(), 1);

        // Another session and back keeps an Undo: it is the note's. Undo B gives the words
        // back exactly.
        harness.state_mut().0.select(None);
        harness.run_steps(2);
        harness.state_mut().0.select(Some(note.id));
        harness.run_steps(2);
        harness.get_by_label("Undo add rewards").click();
        harness.run_steps(2);
        assert_eq!(read(), outside);

        // The DM deletes the XP line: the next block has none, as the newest block says.
        let no_xp = with_a.replace("  - XP 200 for 1 at 0 HP · 1 still stood (XP 200)\n", "");
        harness.get_by_label("Add rewards: Bridge").click();
        harness.run_steps(2);
        // An item typed under the block in Obsidian, after a blank line, is the block's: Undo
        // takes back nothing and says so under the row.
        let typed = read().replace(&block_a, &format!("{block_a}\n  - Varra's signet ring\n"));
        fs::write(&note.path, &typed).unwrap();
        harness.get_by_label("Undo add rewards").click();
        harness.run_steps(2);
        assert_eq!(read(), typed);
        assert!(shown(&harness, "That line changed on disk."));
        assert!(harness.query_by_label("Undo add rewards").is_none());
        fs::write(&note.path, &no_xp).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.run_steps(2);
        assert!(
            harness.query_by_label("Undo add rewards").is_none(),
            "the block is no longer the act's"
        );
        harness.get_by_label("Add rewards: Shrine").click();
        harness.run_steps(2);
        assert!(read().contains("- [[Timeline#^"));
        assert!(!read().contains("XP not known"));
        assert!(shown(&harness, "No XP, as in your last rewards."));
        assert!(!shown(&harness, "XP tonight"));
        fs::write(&note.path, &outside).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.run_steps(2);

        // Unsaved typed words and a changed file: the act writes nothing and says why.
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            draft.body.push_str("Words being typed.\n");
            draft.edited();
        }
        let changed = outside.replace("The ford held.", "The ford held. Obsidian");
        fs::write(&note.path, &changed).unwrap();
        harness.get_by_label("Add rewards: Bridge").click();
        harness.run_steps(2);
        assert_eq!(read(), changed);
        assert!(shown(
            &harness,
            "The note changed on disk. Choose in the note first."
        ));

        // A failed save says so; the block waits in the draft, with its Undo.
        harness.state_mut().0.draft = Some(Draft::open(&note.path).unwrap());
        harness.run_steps(2);
        let mut permissions = fs::metadata(&sessions).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o555);
        fs::set_permissions(&sessions, permissions.clone()).unwrap();
        harness.get_by_label("Add rewards: Bridge").click();
        harness.run_steps(2);
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
        fs::set_permissions(&sessions, permissions).unwrap();
        assert_eq!(read(), changed);
        assert!(shown(&harness, "Notes not saved"));
        assert!(
            harness
                .state()
                .0
                .draft
                .as_ref()
                .unwrap()
                .body
                .contains(&block_a)
        );
        assert!(harness.query_by_label("Undo add rewards").is_some());
        harness.state_mut().0.draft.as_mut().unwrap().save();
        assert!(read().contains(&block_a));

        // After a restart the note holds the block, and the row says `In notes`, with no Undo.
        harness.state_mut().0 = Sessions {
            selected: Some(note.id),
            notes: vec![note.clone()],
            folder: Some(sessions.clone()),
            draft: Some(Draft::open(&note.path).unwrap()),
            campaign: harness.state().0.campaign.clone(),
            ..Sessions::default()
        };
        harness.run_steps(2);
        assert!(shown(&harness, "In notes"));
        assert!(harness.query_by_label("Undo add rewards").is_none());

        // The act reads its new words again before it writes. A block that would be code in a
        // code block that is not closed writes nothing, says so, and a second press neither.
        let restarted = read();
        let prefix = &restarted[..restarted.len() - crate::sessions::body_of(&restarted).len()];
        let open_fence = format!("{prefix}# Session\n\n```\nunfinished example\n");
        fs::write(&note.path, &open_fence).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.run_steps(2);
        for _ in 0..2 {
            harness.get_by_label("Add rewards: Bridge").click();
            harness.run_steps(2);
            assert_eq!(read(), open_fence);
            assert!(shown(&harness, crate::sessions::REWARDS_IN_CODE));
        }
        // Undo compares whole lines: a one-line block moved into a quote just before a heading
        // is not the act's any more. The file keeps its bytes, and the row says so.
        fs::write(
            &note.path,
            format!("{prefix}# Session\n\n- [[Timeline#^{a}|Bridge]]\n  - 30 gp\n"),
        )
        .unwrap();
        harness.get_by_label("Add rewards: Shrine").click();
        harness.run_steps(2);
        let single = format!("- [[Timeline#^{b}|Shrine]]\n");
        assert!(read().ends_with(&single));
        let quoted = format!("{prefix}# Session\n\n> {single}## Keep this heading\nKeep this.\n");
        fs::write(&note.path, &quoted).unwrap();
        harness.get_by_label("Undo add rewards").click();
        harness.run_steps(2);
        assert_eq!(read(), quoted);
        assert!(shown(&harness, "That line changed on disk."));
        fs::write(&note.path, &restarted).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.run_steps(2);

        // R41 (round 30): a run folder that is gone says so in its own words, not that the
        // run cannot be read; nothing is written.
        let before = read();
        let aside = runs.with_file_name("Runs set aside");
        fs::rename(&runs, &aside).unwrap();
        harness.get_by_label("Add rewards: Shrine").click();
        harness.run_steps(2);
        assert_eq!(read(), before);
        assert!(shown(&harness, "Splat cannot find the run folder "));
        assert!(!shown(&harness, REWARDS_BLOCKED));
        let tone = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>| {
            harness
                .state()
                .0
                .rewards
                .fault
                .as_ref()
                .map(|fault| fault.4)
        };
        assert_eq!(
            tone(&harness),
            Some(Severity::NeedsFixing),
            "missing: a warning"
        );
        // R41 (round 30 review F4): a folder that cannot be read (here a file stands where it
        // should be) says so, as an error in the footer too.
        fs::write(&runs, "not a folder").unwrap();
        harness.get_by_label("Add rewards: Shrine").click();
        harness.run_steps(2);
        assert_eq!(read(), before);
        assert!(shown(&harness, "Splat cannot read the run folder "));
        assert_eq!(
            tone(&harness),
            Some(Severity::CannotRead),
            "unreadable: an error"
        );
        fs::remove_file(&runs).unwrap();
        fs::rename(&aside, &runs).unwrap();

        // A run that cannot be read: nothing is written, and the row says why.
        fs::remove_file(crate::encounter_run::find_run(&runs, campaign, shrine.id).unwrap())
            .unwrap();
        harness.get_by_label("Add rewards: Shrine").click();
        harness.run_steps(2);
        assert_eq!(read(), before);
        assert!(shown(&harness, REWARDS_BLOCKED));

        // A title that fills the row: the action is the last link of the opened row.
        let long = "Wolves at the Ford Below the Burning Shrine Road and the Old Toll Bridge";
        harness
            .state_mut()
            .1
            .events
            .iter_mut()
            .find(|event| event.id == a)
            .unwrap()
            .title = long.into();
        fs::write(&note.path, &outside).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.run_steps(2);
        assert!(
            harness
                .query_by_label(&format!("Add rewards: {long}"))
                .is_none()
        );
        harness.get_by_label(long).click();
        harness.run_steps(2);
        harness.get_by_label(ADD_REWARDS).click();
        harness.run_steps(2);
        assert!(read().contains(&format!("- [[Timeline#^{a}|{long}]]\n  - XP 200")));
        assert!(shown(&harness, "In notes"));
        // Review of PR 645 (P1s): Log opens the played fight's log in place, in the plan's own
        // pane, which scrolls it: no scroll column of its own nests there (a debug build refuses
        // one). Its newest entries show first (the summary holds the log newest first), the
        // newest a long note cut to one line, and the oldest is left to Whole run, which a last
        // line says. However long a note, the block is no taller than a third of the reader's
        // measure.
        harness.get_by_label("Log").click();
        harness.run_steps(2);
        let path = crate::encounter_run::find_run(&runs, campaign, bridge.id).unwrap();
        let log = crate::encounter_run::fight_summary(&path, campaign)
            .unwrap()
            .log;
        assert!(log[0].starts_with("Note: "), "the newest entry is the note");
        let newest = harness
            .query_by_label(&log[0])
            .expect("the newest entry shows, whole in its name")
            .rect();
        assert!(
            harness.query_by_label(log.last().unwrap()).is_none(),
            "the oldest entry is left to Whole run"
        );
        let notice = harness
            .query_by_label_contains("Whole run has ")
            .expect("a last line says what Whole run has")
            .rect();
        assert!(
            notice.bottom() - newest.top() <= design::READER_MEASURE / 3.0 + 0.5,
            "the log block is {} px, over a third of the reader's measure",
            notice.bottom() - newest.top()
        );
        // Review of PR 645 (re-checks): the Log is laid out from the log as read from disk. A
        // note rewritten on disk and read again shows its new words, also at the same length;
        // and at 960 and 1920 px a note that grows moves no later plan row.
        let write_newest = |note: &str| {
            let mut saved: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
            let entries = saved["state"]["log"].as_array_mut().unwrap();
            *entries.last_mut().unwrap() = note.into();
            fs::write(&path, saved.to_string()).unwrap();
        };
        // Close the fight and open it again (its summary is read again), then its Log.
        let reopen = |harness: &mut egui_kittest::Harness<'_, (Sessions, Book)>| {
            harness.get_by_label(long).click();
            harness.run_steps(2);
            harness.get_by_label(long).click();
            harness.run_steps(2);
            harness.get_by_label("Log").click();
            harness.run_steps(2);
        };
        let same_length = log[0].replacen("creaks", "groans", 1);
        assert_eq!(same_length.len(), log[0].len());
        write_newest(&same_length);
        reopen(&mut harness);
        assert!(
            harness.query_by_label(&same_length).is_some()
                && harness.query_by_label(&log[0]).is_none(),
            "a note rewritten to the same length shows its new words"
        );
        let later = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>| {
            harness.get_by_label("Shrine").rect().top()
        };
        for size in [egui::vec2(960.0, 800.0), egui::vec2(1920.0, 1080.0)] {
            harness.set_size(size);
            write_newest(&same_length);
            reopen(&mut harness);
            let before = later(&harness);
            let grown = format!(
                "Note: {}",
                "The gate holds, and the wolves wait outside. ".repeat(40)
            );
            write_newest(&grown);
            reopen(&mut harness);
            let newest = harness
                .query_by_label(&grown)
                .expect("the grown note shows, whole in its name")
                .rect();
            let notice = harness
                .query_by_label_contains("Whole run has ")
                .expect("a last line says what Whole run has")
                .rect();
            assert!(
                notice.bottom() - newest.top() <= design::READER_MEASURE / 3.0 + 0.5,
                "at {size:?} the log block is {} px",
                notice.bottom() - newest.top()
            );
            assert!(
                (later(&harness) - before).abs() < 0.5,
                "at {size:?} the grown note moved the next plan row from {before} to {}",
                later(&harness)
            );
        }
        harness.set_size(egui::vec2(design::FORM_WIDTH, 800.0));
        write_newest(&log[0]);
        reopen(&mut harness);
        harness.get_by_label("Log").click();
        harness.run_steps(2);

        // Too narrow for the link after Timeline: no line is added, and the action is the last
        // line of the row's menu. With one blank at each `·` (round 27 FH4), the links fit at
        // 300 px, so this is a narrower window.
        fs::write(&note.path, &outside).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.set_size(egui::vec2(260.0, 800.0));
        harness.run_steps(2);
        assert!(harness.query_by_label(ADD_REWARDS).is_none());
        harness
            .get_by_label(&format!("More actions for {long}"))
            .click();
        harness.run_steps(2);
        harness.get_by_label(ADD_REWARDS).click();
        harness.run_steps(2);
        assert!(read().contains(&format!("- [[Timeline#^{a}|{long}]]\n  - XP 200")));
        // The menu then says `In notes` in the action's place.
        assert!(!shown(&harness, "In notes"));
        harness
            .get_by_label(&format!("More actions for {long}"))
            .click();
        harness.run_steps(2);
        assert!(shown(&harness, "In notes"));
        assert!(harness.query_by_label(ADD_REWARDS).is_none());
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
}
