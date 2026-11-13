//! The Session page's plan: its rows (planned, open and played fights, groups and events), the
//! Add strip, a row's menu and its opened detail.
use super::{Added, Plan, Sessions, plan_summary, wrapped_words};
use crate::{
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    encounter_run::fight_summary,
    encounters::PlanFight,
    lookup::ContentIndex,
    sessions::{
        ActFault, Apply, NoteUndo, PlanLine, TaskLine, add_scene, change_line, each_linked_name,
        insert_in_note, insert_in_plan, linked_names, plain_links, plan_lines, ticked,
    },
    timeline::{
        self, Book, Kind, checks,
        page::{
            LiveUndos, added_message, file_act, file_notices_in_footer, long_rest_status, problems,
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

const UNDO_ADD_FOR: Duration = Duration::from_secs(30);

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

/// The DM scrolled or moved through the page this draw: the wheel, a drag, or a navigation key.
fn user_moved_the_view(ui: &egui::Ui) -> bool {
    use egui::Key;
    ui.input(|input| {
        input.raw_scroll_delta != egui::Vec2::ZERO
            || input.smooth_scroll_delta != egui::Vec2::ZERO
            || input.pointer.is_decidedly_dragging()
            || input.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::Key {
                        key: Key::ArrowUp
                            | Key::ArrowDown
                            | Key::PageUp
                            | Key::PageDown
                            | Key::Home
                            | Key::End
                            | Key::Tab
                            | Key::Space,
                        pressed: true,
                        ..
                    }
                )
            })
    })
}

/// How many draws a reveal may take before it gives up.
const REVEAL_DRAWS: u8 = 3;

/// What the DM chose in the plan's message footer. It is done after the footer is drawn, so
/// the footer's messages borrow nothing that the plan's rows change.
#[derive(Clone, Debug, PartialEq)]
enum FooterAct {
    GoToGroup,
    UndoGroupDiscard,
    UndoGroupRemove,
    UndoDeletedRun,
    UndoAdd(String),
    UndoRemove,
    Scene(SceneAct),
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
fn planned_saved_run<'a>(
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
fn plan_event_summary(
    ui: &mut egui::Ui,
    summary: &str,
    restores: Option<bool>,
    warning: Option<&str>,
) {
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
    if !job.text.is_empty() {
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
fn plan_summary_text(ui: &egui::Ui, group: &crate::encounter_run::RunGroup) -> String {
    let summary = group.summary();
    let members = group.member_summary();
    match summary.strip_suffix(&members) {
        Some(head) if !members.is_empty() => format!(
            "{head}{}",
            group.member_summary_wrapped(ui, &Type::Small.font(ui), ui.available_width())
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
    let font = Type::Small.font(ui);
    let width = ui.available_width();
    let text = without_loose_separators(ui, summary.replace(" · ", FACT_SEPARATOR), &font, width);
    let mut job = egui::text::LayoutJob::simple(text, font, colors().muted, width);
    job.wrap.max_rows = max_rows;
    let plain = summary.replace('\u{a0}', " ");
    let response = ui.add(egui::Label::new(job).wrap());
    // The no-break spaces only steer the wrap; the name keeps plain spaces.
    design::named(ui, &response, &plain);
    response.on_hover_text(plain);
}

/// A body link whose word starts at the cursor, as the text above it does, and whose underline
/// covers only the word. The target is the link's own minimum size: at least `XL` square. A
/// word narrower than that gets the extra width evenly on both sides, inside the gaps, so the
/// word stays at the text edge and nothing beside it moves (the `line_start` pattern).
fn word_link(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let c = colors();
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), Type::Body.font(ui), c.accent);
    let (id, word) =
        ui.allocate_space(egui::vec2(galley.size().x, galley.size().y.max(design::XL)));
    let target = word.expand2(egui::vec2(
        ((design::XL - word.width()) / 2.0).max(0.0),
        0.0,
    ));
    let response = ui.interact(target, id, egui::Sense::click());
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, ui.is_enabled(), label));
    if ui.is_rect_visible(target) {
        let top = word.center().y - galley.size().y / 2.0;
        let color = if response.hovered() {
            c.focus
        } else {
            c.accent
        };
        ui.painter().hline(
            word.x_range(),
            top + galley.size().y - 1.0,
            egui::Stroke::new(design::HAIRLINE_WIDTH, color.gamma_multiply(0.6)),
        );
        ui.painter()
            .galley(egui::pos2(word.left(), top), galley, color);
        if response.has_focus() {
            design::focus_ring(ui, target);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
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
    widths: &[f32],
    icon: &str,
    words: impl FnMut(&mut egui::Ui),
    summary: impl FnMut(&mut egui::Ui),
    action: impl FnMut(&mut egui::Ui),
) {
    plan_row_with(
        ui,
        widths,
        |ui| {
            ui.label(Type::Body.rich(ui, icon).color(colors().muted));
        },
        words,
        summary,
        action,
    );
}

/// A plan row whose start is `lead`: a fight's caret, or a scene's box. Every row has the one
/// leading slot, so every row's words start at one column.
fn plan_row_with(
    ui: &mut egui::Ui,
    widths: &[f32],
    lead: impl FnOnce(&mut egui::Ui),
    mut words: impl FnMut(&mut egui::Ui),
    mut summary: impl FnMut(&mut egui::Ui),
    mut action: impl FnMut(&mut egui::Ui),
) {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = design::S;
        let (_, slot) = ui.allocate_space(egui::vec2(lead_width(), design::XL));
        lead(
            &mut ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(slot)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            ),
        );
        ui.vertical(|ui| {
            let left = ui.cursor().min;
            let title_width = (ui.available_width() - widths[1] - design::S).max(design::XL);
            let title = ui.allocate_ui_with_layout(
                egui::vec2(title_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(title_width);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    words(ui);
                },
            );
            let action_rect = egui::Rect::from_min_size(
                egui::pos2(ui.max_rect().right() - widths[1], left.y),
                egui::vec2(widths[1], Density::Read.control()),
            );
            let mut action_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(action_rect)
                    .layout(egui::Layout::right_to_left(egui::Align::Center)),
            );
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
            ui.allocate_ui_with_layout(
                egui::vec2(
                    (ui.available_width() - Density::Read.control() - design::S).max(design::XL),
                    0.0,
                ),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    summary(ui);
                },
            );
            ui.expand_to_include_rect(action_rect.union(action_ui.min_rect()));
        });
    });
}

/// A paused run needs its state here; the whole saved-run record stays in Encounters.
fn plan_saved_status(saved: &crate::encounter_run::SavedRunInfo) -> String {
    crate::encounters::run_state_word(saved, true)
}

fn plan_link(ui: &mut egui::Ui, label: &str, open: bool) -> egui::Response {
    let response = design::wrapped_link(ui, label);
    design::named(ui, &response, label);
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_expanded(open));
    response
}

/// The open fight's state on its plan row. The run icon is its own glyph before the link words,
/// so the underline covers only the words, as in `status_link`. The press target covers both, at
/// least 24 px, and the accessible name is the words.
fn plan_run_link(ui: &mut egui::Ui, word: &str) -> egui::Response {
    glyph_link(ui, icons::RUN, colors().accent, Type::Small, word)
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
    job.wrap.max_width = ui.available_width().max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
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
/// and no line starts or ends with `·`.
fn planned_summary(
    ui: &mut egui::Ui,
    summary: &str,
    difficulty: Option<&crate::difficulty::Difficulty>,
) {
    let c = colors();
    let font = Type::Small.font(ui);
    let width = ui.available_width();
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
        let response = ui.add(egui::Label::new(Type::Small.rich(ui, alone).color(c.muted)).wrap());
        // The breaks only steer the wrap; the name keeps the summary's words.
        design::named(ui, &response, summary);
        return;
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
    let lines = rows(&alone);
    let missing = difficulty
        .filter(|difficulty| difficulty.rating.is_none())
        .and_then(|difficulty| difficulty.missing.as_ref());
    let (full, short) = match missing {
        // W1-D2, X1: whose input is not known, in the `XP not known` place: whole, else the
        // first name shortened from its end, else the reason alone.
        Some(missing) => {
            let fitted = missing
                .fitted(|words| rows(&with_form(summary, &words.replace(' ', "\u{a0}"))) <= lines)
                .unwrap_or_else(|| missing.short().to_owned());
            let form = nb(vec![(fitted, false)]);
            (form.clone(), form)
        }
        None => {
            let full = nb(parts);
            let short = difficulty
                .and_then(|difficulty| difficulty.rating.as_ref())
                .map_or_else(|| full.clone(), |rating| nb(rating.parts(false)));
            (full, short)
        }
    };
    // The summary's facts and members, each with the separator before it. It gives way by
    // whole ones, from its end: `5 party · 1 × Cinder Tyrant, …`.
    let mut units: Vec<(&str, &str)> = Vec::new();
    for (index, fact) in summary.split(" · ").enumerate() {
        for (at, member) in fact.split(", ").enumerate() {
            let before = match (index, at) {
                (0, 0) => "",
                (_, 0) => " · ",
                _ => ", ",
            };
            units.push((before, member));
        }
    }
    let cut_at = |count: usize| {
        let kept: String = units[..count]
            .iter()
            .map(|(before, unit)| format!("{before}{unit}"))
            .collect();
        format!("{kept}{}…", units[count].0)
    };
    let whole = [&full, &short]
        .into_iter()
        .map(|parts| (with_form(summary, &joined(parts)), parts))
        .find(|(shown, _)| rows(shown) <= lines);
    let (shown, parts, cut) = if let Some((shown, parts)) = whole {
        (shown, parts.clone(), false)
    } else {
        // The most whole facts and members that keep the lines, by halves; with none, the
        // form alone.
        let form = joined(&short);
        let (mut fits, mut over) = (0, units.len());
        while over - fits > 1 {
            let middle = (fits + over) / 2;
            if rows(&with_form(&cut_at(middle), &form)) <= lines {
                fits = middle;
            } else {
                over = middle;
            }
        }
        let shown = if fits == 0 {
            form
        } else {
            with_form(&cut_at(fits), &form)
        };
        (shown, short.clone(), true)
    };
    let form = joined(&parts);
    let facts = shown.strip_suffix(form.as_str()).unwrap_or("");
    let mut job = egui::text::LayoutJob::default();
    crate::difficulty::append(&mut job, ui, Type::Small, facts, c.muted);
    crate::difficulty::append_form(&mut job, ui, Type::Small, &parts);
    job.wrap.max_width = width;
    let response = ui.add(egui::Label::new(job).wrap());
    // The accessible name keeps the whole summary and the whole form.
    let plain_form = missing.map_or_else(
        || joined(&full).replace('\u{a0}', " "),
        |missing| {
            missing
                .words()
                .unwrap_or_else(|| missing.short().to_owned())
        },
    );
    design::named(ui, &response, &format!("{summary} · {plain_form}"));
    let difficulty = difficulty.expect("the form came from it");
    match (&difficulty.rating, &difficulty.missing) {
        (Some(rating), _) => {
            let response = if cut {
                response.on_hover_text(summary)
            } else {
                response
            };
            crate::difficulty::describe(ui, response, rating);
        }
        // W1-D2: whose input is not known, on hover.
        (None, Some(missing)) => {
            let names = missing.words().unwrap_or_default();
            let hover = match (cut, names.is_empty()) {
                (true, true) => summary.to_owned(),
                (true, false) => format!("{summary}\n{names}"),
                (false, _) => names,
            };
            if !hover.is_empty() {
                response.on_hover_text(hover);
            }
        }
        (None, None) => {}
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
    ui.add_space(design::S);
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
    egui::Popup::menu(&response)
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
            if let Some(reason) = menu.blocked {
                wrapped_words(ui, Type::Small, reason, colors().muted);
            }
            if names.len() > FIGHT_SEARCH_FROM {
                let field =
                    design::search_field(ui, menu.find, search, menu.hint, FIGHT_MENU_WIDTH);
                if std::mem::take(focus) {
                    field.request_focus();
                }
            }
            ui.separator();
            let query = search.trim().to_lowercase();
            let mut chosen = None;
            egui::ScrollArea::vertical()
                .id_salt(menu.word)
                .max_height(FIGHT_MENU_HEIGHT)
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

/// The fields of one event, in place in a session's plan. Returns whether any changed.
fn event_fields(ui: &mut egui::Ui, event: &mut timeline::Event) -> bool {
    let title = design::text_field(ui, "Title", &mut event.title, None, None);
    let mut changed = title.changed();
    design::field(ui, "Notes", None, None, |ui| {
        let response = ui.add(
            egui::TextEdit::multiline(&mut event.notes)
                .desired_width(ui.available_width() - 2.0 * design::S)
                .desired_rows(2)
                .margin(egui::vec2(design::S, design::S)),
        );
        design::named(ui, &response, "Notes on this event");
        changed |= response.changed();
    });
    changed
}

/// A played fight's `Where`, in place under its opened row's links (W2-D3): the Timeline detail's
/// own field on the same event, so the typed place has one owner, the Timeline, and saves as
/// any edit of an event does. Returns whether it changed.
fn place_field(ui: &mut egui::Ui, event: &mut timeline::Event) -> bool {
    ui.add_space(design::XS);
    design::text_field(ui, "Where", &mut event.place, None, None).changed()
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
struct Scene {
    task: TaskLine,
    /// The one entry that its words link, when they link exactly one.
    entry: Option<SceneEntry>,
    /// Its words link a name and no entry has it (X14): it says `Not found`.
    not_found: bool,
    /// Its entry had a second line when it was last found, so `Not found` takes that line; else
    /// it ends the words' own line, so the row keeps its height (review round 1).
    second_line: bool,
    /// Whether `Add to Log` fits on the row, for the last few rooms it was measured in. The
    /// words and the done mark are this model's, so only the room can change it
    /// (`Scene::log_keeps_rows`).
    fit: std::sync::Mutex<Vec<(f32, f32, bool)>>,
}

impl Scene {
    /// The fit rule for this scene's `Add to Log`, measured again only when the room or the
    /// action's width changes; an unchanged frame lays out no words.
    fn log_keeps_rows(&self, ui: &egui::Ui, plain: &str, menu: f32, with_log: f32) -> bool {
        let room = ui.available_width();
        #[cfg(test)]
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

#[cfg(test)]
thread_local! {
    /// How often frames reached a done scene's fit, and how often it laid out the words.
    static FIT_COUNTS: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
}

/// A done scene's fit on this (the UI) thread: frames that reached it, and layouts of its words.
#[cfg(test)]
pub(crate) fn scene_fit_counts() -> (usize, usize) {
    FIT_COUNTS.with(std::cell::Cell::get)
}

struct SceneEntry {
    /// Its kind, status and place, for a World entry.
    detail: Option<String>,
    /// The fight that its `Start fight` starts, for an NPC or a creature.
    start: Option<crate::world::StartFight>,
    /// The entry's ID, when it is a campaign World entry whose Log takes a line (F6).
    log: Option<(Uuid, String)>,
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
fn words_outside_links(words: &str) -> String {
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
    key: Option<(Option<Uuid>, u64, Option<u64>, u64)>,
    model: Option<Arc<PlanModel>>,
    /// How often a frame asked for the model, and how often it was built.
    pub(super) reached: usize,
    pub(super) builds: usize,
    /// Per linked name, whether its entry had a second line when last found
    /// (`PlanModel::place_not_found`).
    seen: std::collections::HashMap<String, bool>,
}

/// Where a scene act's words show: under the scene's row, under the Scene field, or in the
/// plan's message footer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    Row(Slot),
    Field,
    Footer,
}

/// Where each scene result shows in this frame, decided once before anything is drawn
/// (`Sessions::shown_at`).
#[derive(Clone, Copy, Default)]
struct Shown {
    added: Option<Place>,
    removed: Option<Place>,
    fault: Option<Place>,
}

/// What a scene act could not do, while the note's words are the words it met.
#[derive(Debug)]
pub(super) struct Fault {
    pub(super) words: String,
    met: String,
    place: Place,
    /// The act waits in the draft: its words end when the note is saved.
    waits: bool,
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
    field: Option<design::AddField>,
    /// `Scene added to tonight.`
    pub(super) added: Option<NoteUndo>,
    /// A scene taken out of tonight: its line's place among the plan's lines, its words, and
    /// its Undo. The place is the line that now follows it, which stays while the note is as the
    /// act left it, also when a fight before it is played.
    removed: Option<(usize, String, NoteUndo)>,
    pub(super) fault: Option<Fault>,
    fold: Fold,
    /// The Scene field had focus on the last frame.
    typing: bool,
}

impl Scenes {
    /// An Undo ends for good at another change of the note's words or in another note, also
    /// when the words come back to what the act left.
    fn end_what_changed(&mut self, draft: Option<&crate::sessions::Draft>) {
        if self.fault.as_ref().is_some_and(|fault| {
            draft.is_none_or(|draft| draft.body != fault.met || (fault.waits && !draft.unsaved()))
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
enum SceneAct {
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
    width: f32,
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
            &widths,
            |ui| {
                let mut done = task.done;
                let response = scene_box(ui, &mut done);
                let state = if task.done { "Done" } else { "Open" };
                design::named(ui, &response, &format!("{state}: {plain}"));
                tick = response.clicked();
            },
            |ui| {
                if scene.not_found && !scene.second_line {
                    gone_scene_words(ui, task, &plain);
                } else {
                    scene_words(ui, task, &plain);
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
                        if let Some(reason) = plan.blocked {
                            wrapped_words(ui, Type::Small, reason, c.muted);
                        }
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
fn scene_words(ui: &mut egui::Ui, task: &TaskLine, plain: &str) {
    let font = Type::Body.font(ui);
    let width = ui.available_width();
    let (rows, row_height) = ui.fonts_mut(|fonts| {
        let rows = fonts
            .layout(plain.to_owned(), font.clone(), colors().text, width)
            .rows
            .len();
        (rows, fonts.row_height(&font))
    });
    if rows == 1 {
        ui.add_space(((design::XL - row_height) / 2.0).max(0.0));
    }
    crate::markdown_view::show_row_words(ui, &task.words, task.done);
}

/// The words of a scene whose entry is gone and that had no second line: the words, then the
/// warning mark and `Not found` at the end of their line, in the lines the words take alone, so
/// the row keeps its height. Where the mark has no room, the words give up whole words from
/// their end first; hover keeps them whole (review round 1).
fn gone_scene_words(ui: &mut egui::Ui, task: &TaskLine, plain: &str) {
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
    let rows = |job: egui::text::LayoutJob| ui.fonts_mut(|fonts| fonts.layout_job(job).rows.len());
    let alone = ui.fonts_mut(|fonts| {
        fonts
            .layout(plain.to_owned(), Type::Body.font(ui), words_color, width)
            .rows
            .len()
    });
    let all: Vec<&str> = plain.split_whitespace().collect();
    let whole = all.join(" ");
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
    #[cfg(test)]
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

/// A planned fight's summary with the day it is played on: `… · Day 2 of Road to the ford`.
/// As the row's rating word does, the words show only while the line keeps its rows (rule P):
/// first the journey's name goes, then the day.
fn on_road(ui: &egui::Ui, summary: &str, day: Option<&str>) -> String {
    let Some(day) = day else {
        return summary.to_owned();
    };
    let font = Type::Small.font(ui);
    let width = ui.available_width();
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
    [with(day), with(short)]
        .into_iter()
        .find(|words| rows(words) <= alone)
        .unwrap_or_else(|| summary.to_owned())
}

/// The help line of **How long** when the clock cannot place the journey.
const LENGTH_HELP: &str = timeline::LENGTH_HELP;

/// Which result's Undo the DM pressed in a journey's checks.
#[derive(Clone, Copy, PartialEq)]
enum TravelUndo {
    None,
    Checks,
    PlanLine,
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
    /// Undo. It lasts while the note's words are what the act left, as every note act's Undo
    /// does; a page change keeps it.
    planned: Option<(String, checks::Planned, NoteUndo)>,
    /// The fights still to come with their days on the road, while the note, the Timeline and
    /// the campaign's fights are as they were.
    days: Option<(RoadKey, Arc<RoadDays>)>,
}

/// What the plan's days on the road are read from: the session, the note's revision, the
/// Timeline's count of changes, and the count of the campaign's fights.
type RoadKey = (Option<Uuid>, u64, u64, usize);

/// Each fight still to come, by its name, with its day on the road when it has one.
type RoadDays = Vec<(String, Option<String>)>;

/// The day of the planned fight at this place: that fight's own turn among the rows to come.
fn planned_day(
    days: &[(String, Option<String>)],
    planned: &[&PlanFight],
    position: usize,
) -> Option<String> {
    let name = planned[position].name.to_lowercase();
    let turn = planned[..position]
        .iter()
        .filter(|fight| fight.name.eq_ignore_ascii_case(&name))
        .count();
    days.iter()
        .filter(|(known, _)| *known == name)
        .nth(turn)
        .and_then(|(_, day)| day.clone())
}

/// F7's action and its words.
const ADD_REWARDS: &str = "Add rewards";
/// The gap between the opened played row's links and their `·`, at every size (X48).
const LINK_GAP: f32 = design::S;
const REWARDS_BLOCKED: &str = "Splat cannot read this fight’s run, so its rewards are not known.";

/// Where a played row has `Add rewards`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RewardsAt {
    Row,
    Link,
    Menu,
}

/// What the DM pressed for a played fight: `Add rewards`, or `Undo rewards`, by its event.
enum RewardsAct {
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
/// fight, and shows under that fight's row while its block stands as the act left it. Another
/// act, another session or another page leaves it.
#[derive(Default)]
pub(super) struct Rewards {
    done: BTreeMap<(Uuid, String), RewardsDone>,
    /// What an act could not do, where the DM acted: the note, the fight, the words, and the
    /// draft's revision it met. A change of the note ends it.
    fault: Option<(Uuid, String, String, u64)>,
    seen: Option<RewardsSeen>,
    /// Whether `Add rewards` fits on a played row: by the title, its lead icon, the row's width
    /// and the action's. Worked out when one of them changes, not each frame.
    fit: BTreeMap<(String, &'static str, i64, i64), RewardsAt>,
}

impl Rewards {
    /// Read the selected note again when its draft changed; a result whose block no longer
    /// stands as the act left it ends.
    fn see(&mut self, draft: Option<&crate::sessions::Draft>, session: Option<Uuid>, book: &Book) {
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
        self.done.retain(|(id, _), done| {
            *id != note || crate::sessions::remove_rewards(&draft.body, &done.written).is_some()
        });
        self.seen = Some(RewardsSeen {
            revision: draft.revision(),
            listed: crate::sessions::listed_references(&draft.body),
            tonight: crate::sessions::xp_in_note(&draft.body, &|id| fights.contains(id)),
        });
    }

    fn listed(&self, event: &str) -> bool {
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
        // three `·`, with a `LINK_GAP` between each two of them.
        let links = ["Log", "Whole run", "Timeline", ADD_REWARDS]
            .map(|link| word(link, Type::Body))
            .iter()
            .sum::<f32>()
            + 3.0 * word("·", Type::Small)
            + 6.0 * LINK_GAP;
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

    /// A fight's result and its fault, under its row: a result that belongs to one row stays
    /// under that row (the plan's footer rule), and a fight's row never folds.
    fn message(
        &self,
        ui: &mut egui::Ui,
        note: Uuid,
        event: &str,
        revision: u64,
    ) -> Option<RewardsAct> {
        let mut act = None;
        if let Some((id, at, words, met)) = &self.fault
            && (*id, at.as_str(), *met) == (note, event, revision)
        {
            ui.add_space(design::S);
            design::message(ui, Tone::Warning, words);
        }
        if let Some(done) = self.done.get(&(note, event.to_owned())) {
            let mut words = format!("Rewards added under Loot and rewards. {}", done.choice);
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
            ui.add_space(design::S);
            if design::undo_message(ui, Tone::Success, &words, "Undo rewards") {
                act = Some(RewardsAct::Undo(event.to_owned()));
            }
        }
        act
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
                let read = match (&plan.run_directory, &self.campaign) {
                    (Some(directory), Some(campaign)) => {
                        crate::encounter_run::find_run(directory, campaign.id, run)
                            .and_then(|path| crate::encounter_run::rewards(&path, campaign.id))
                    }
                    _ => Err(String::new()),
                };
                let Ok(found) = read else {
                    rewards.fault =
                        Some((note, event, REWARDS_BLOCKED.to_owned(), draft.revision()));
                    return;
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
                        },
                    );
                    self.caret_to_show = Some(shown);
                }
                if let Some(why) = refused {
                    rewards.fault = Some((note, event, why.to_owned(), draft.revision()));
                    return;
                }
                (event, result.map(drop))
            }
        };
        if let Err(fault) = result {
            rewards.fault = Some((note, event, fault.words().to_owned(), draft.revision()));
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
        ui.add_space(design::XS);
        // The plan line's message shows on the line of the check it came from. With that check
        // gone, it takes the kept line, unless a roll's result holds it: then it ends the block.
        let plan_line = travel
            .planned
            .as_ref()
            .filter(|(id, _, _)| *id == event.id)
            .map(|(_, line, _)| line);
        let on_check = plan_line.filter(|line| event.checks.iter().any(|check| line.on(check)));
        let elsewhere = plan_line.filter(|_| on_check.is_none());
        let done = travel.done.get(&event.id);
        let plan_words = elsewhere.map(checks::Planned::words);
        let (kept, last) = match (done, &plan_words) {
            (Some(done), _) => (
                Some((done.words.as_str(), done.undo, TravelUndo::Checks)),
                plan_words.as_deref(),
            ),
            (None, Some(words)) => (
                Some((words.as_str(), "Undo plan line", TravelUndo::PlanLine)),
                None,
            ),
            (None, None) => (None, None),
        };
        let header = checks::View {
            event,
            start,
            length: &event.duration,
            tables: plan.tables,
            planned,
            plan_line: on_check,
        };
        let mut act = checks::header(ui, &header, &mut travel.block);
        if design::undo_line(
            ui,
            Tone::Success,
            kept.map(|(words, undo, _)| (words, undo)),
        ) {
            undo = kept.map_or(TravelUndo::None, |(_, _, which)| which);
        }
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
            plan_line: on_check,
        };
        act = checks::lines(ui, &view, &mut travel.block).or(act);
        if act == Some(checks::Act::UndoPlan) {
            act = None;
            undo = TravelUndo::PlanLine;
        }
        if let Some(words) = last {
            ui.add_space(design::S);
            if design::undo_message(ui, Tone::Success, words, "Undo plan line") {
                undo = TravelUndo::PlanLine;
            }
        }
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
        let fight = [PlanFight {
            saved: None,
            id: Uuid::nil(),
            name: name.to_owned(),
            path: String::new(),
            summary: String::new(),
            difficulty: Default::default(),
        }];
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
    fn planned_days(
        &mut self,
        book: &Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
    ) -> Arc<RoadDays> {
        let Some(draft) = &self.draft else {
            return Arc::default();
        };
        let key = (session, draft.revision(), book.changes(), plan.fights.len());
        if let Some((known, days)) = &self.travel.days
            && *known == key
        {
            return Arc::clone(days);
        }
        let days = Arc::new(road_days(&draft.body, &book.events, session, plan.fights));
        self.travel.days = Some((key, Arc::clone(&days)));
        days
    }

    /// Take back the plan line of `Plan “…”`, while the note is as that act left it.
    fn undo_plan_line(&mut self) {
        let (Some((_, line, undo)), Some(draft)) = (self.travel.planned.take(), &mut self.draft)
        else {
            return;
        };
        // A note that changed since says so where the DM acted.
        if let Err(fault) = undo.undo(draft) {
            self.travel
                .block
                .failed(&checks::Act::Plan(line), fault.words().to_owned());
        }
    }

    /// Carry out an act on a journey's checks. `Plan` writes the session note; the others write
    /// `Timeline.md`.
    fn travel_act(
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
            let (Some(draft), Some(note)) = (&mut self.draft, session) else {
                return;
            };
            let line = road_plan_line(book, id, chosen);
            let written = draft.apply(note, Apply::Now, |words| {
                Some(plan_first(words, &line, &book.events, session, plan.fights))
            });
            match written {
                Ok(undo) => self.travel.planned = Some((id.to_owned(), chosen.clone(), undo)),
                Err(fault) => {
                    let words = fault.words().to_owned();
                    if let crate::sessions::ActFault::NotSaved(_, undo) = fault {
                        self.travel.planned = Some((id.to_owned(), chosen.clone(), undo));
                    }
                    self.travel.block.failed(&act, words);
                }
            }
            self.mentions
                .insert(note, timeline::referenced_ids(&draft.body));
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
        self.notice = Some((Tone::Warning, why));
        self.notice_session = None;
    }

    /// Lead ruling, round 26 (one pane, one footer): a plan group's removal result and its Undo
    /// sit in the plan's message footer, not in the removed row's place.
    fn removed_group_message<'a>(
        &self,
        undos: &mut LiveUndos<'a>,
        act: &'a std::cell::Cell<Option<FooterAct>>,
        session: Option<Uuid>,
    ) {
        let Some((id, _, _, after, _, name)) = &self.removed_group else {
            return;
        };
        if *id != session {
            return;
        }
        let words = format!("Removed “{name}” from tonight.");
        let order = self
            .result_orders
            .of("group remove", (session, name, after.len()));
        undos.push(order, words.clone(), move |ui| {
            if design::undo_message(ui, Tone::Success, &words, "Undo remove") {
                act.set(Some(FooterAct::UndoGroupRemove));
            }
        });
    }

    /// The session as a list, top to bottom: what happened, then what is still to come. It
    /// replaces a walk between Encounters, Timeline and the note.
    pub(super) fn plan_pane(&mut self, ui: &mut egui::Ui, book: &mut Book, plan: &Plan<'_>) {
        let session = self.selected;
        let here = self.group_session == session;
        if plan.group_open && here {
            (plan.group_editor.borrow_mut())(ui);
            return;
        }
        let c = colors();
        self.end_plan_results_after_edits(book);
        self.scenes.end_what_changed(self.draft.as_ref());
        if self
            .removed_group
            .as_ref()
            .is_some_and(|(id, _, _, after, _, _)| {
                *id == session && self.draft.as_ref().is_none_or(|draft| draft.body != *after)
            })
        {
            self.removed_group = None;
        }
        // The comparison or the unread file takes the pane: the DM must choose first.
        if problems(ui, book) {
            return;
        }
        let body = self
            .draft
            .as_ref()
            .map_or_else(String::new, |draft| draft.body.clone());
        let mut played: BTreeMap<String, usize> = BTreeMap::new();
        for event in &book.events {
            if event.is_in(session) && event.run.is_some() {
                *played.entry(event.title.to_lowercase()).or_default() += 1;
            }
        }
        // The plan's lines in the note's order: the fights still to come, and the scenes
        // before each of them and after the last.
        let index = ui
            .ctx()
            .data(|data| data.get_temp::<Arc<ContentIndex>>(egui::Id::new("reader_link_index")));
        let model = self.plan_model(index.as_deref(), plan);
        let mut planned: Vec<&PlanFight> = Vec::new();
        // Each scene with its line's place among the plan's lines.
        let mut scenes: Vec<Vec<(usize, &Scene)>> = vec![Vec::new()];
        // Where a removed scene's row stood: before the line that now follows it.
        let removed_line = self.scenes.removed.as_ref().map(|(line, ..)| *line);
        let mut removed_slot = None;
        let place_now = |scenes: &Vec<Vec<(usize, &Scene)>>| {
            (scenes.len() - 1, scenes.last().map_or(0, Vec::len))
        };
        for (at, line) in model.lines.iter().enumerate() {
            if removed_line == Some(at) {
                removed_slot = Some(place_now(&scenes));
            }
            match line {
                ModelLine::Scene(scene) => scenes
                    .last_mut()
                    .expect("one list of scenes for each place")
                    .push((at, scene)),
                ModelLine::Fights(names) => {
                    for name in names {
                        let Some(fight) = plan.fights.iter().find(|fight| fight.name == *name)
                        else {
                            continue;
                        };
                        let done = played.entry(fight.name.to_lowercase()).or_default();
                        let waiting = *done == 0;
                        *done = done.saturating_sub(1);
                        if waiting {
                            planned.push(fight);
                            scenes.push(Vec::new());
                        }
                    }
                }
            }
        }
        if removed_line == Some(model.lines.len()) {
            removed_slot = Some(place_now(&scenes));
        }
        let open_scenes = scenes
            .iter()
            .flatten()
            .filter(|(_, scene)| !scene.task.done)
            .count();
        let scene_count = scenes.iter().map(Vec::len).sum::<usize>();
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
        let planned_groups: Vec<_> = linked_names(&body)
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
                    .find(|group| group.name.eq_ignore_ascii_case(name))
                    .cloned()
            })
            .collect();
        let heading = design::group_heading_text(
            ui,
            if session == self.current() {
                "Tonight"
            } else {
                "This session"
            },
            book.events
                .iter()
                .filter(|event| event.is_in(session))
                .count()
                + planned.len()
                + planned_groups.len()
                + open_scenes,
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
        let planned_results: Vec<(String, bool)> = planned
            .iter()
            .map(|fight| {
                (
                    fight.name.clone(),
                    planned_saved_run(fight, plan.fights, plan.saved_runs).is_some_and(|saved| {
                        saved.standing == crate::encounter_run::Standing::Ended
                    }),
                )
            })
            .collect();
        // Round 26 FL1: the plan's results, reasons and notices sit in its message footer, pinned
        // below the rows, so a result never moves a row or the Add strip.
        let footer_act = std::cell::Cell::new(None);
        let mut group_undo_in_footer = false;
        let mut footer =
            design::Footer::new(("session_plan_footer", session), ui.available_width());
        if plan.group_open && self.not_being_played().is_none() {
            let label = self
                .notes
                .iter()
                .find(|note| Some(note.id) == self.group_session)
                .map_or_else(
                    || "another session".to_owned(),
                    |note| format!("“{}”", note.label()),
                );
            let words = format!("A group is being assembled in the plan of {label}.");
            let act = &footer_act;
            footer.notice(words.clone(), move |ui| {
                if design::message_with_action(ui, Tone::Info, &words, "Go to it") {
                    act.set(Some(FooterAct::GoToGroup));
                }
            });
        }
        let mut undos = LiveUndos::new();
        if plan.group_discarded && here {
            let name = plan.group_discarded_name.clone();
            let act = &footer_act;
            let order = plan.group_discarded_order;
            undos.push(order, format!("{name} draft discarded."), move |ui| {
                if design::draft_discarded(ui, &name) {
                    act.set(Some(FooterAct::UndoGroupDiscard));
                }
            });
        }
        if let Some(problem) = self.rename_problem.clone() {
            footer.reason(problem.clone(), move |ui| {
                design::message(ui, Tone::Warning, &problem);
            });
        }
        let file = std::cell::Cell::new(None);
        file_notices_in_footer(&mut footer, &mut undos, book, &file);
        self.plan_messages(
            ui,
            &mut undos,
            &footer_act,
            book,
            session,
            plan,
            &planned_results,
        );
        self.scene_messages(&mut footer, &mut undos, &footer_act, shown);
        self.removed_group_message(&mut undos, &footer_act, session);
        undos.into_footer(&mut footer);
        // The fold measures the room the rows have with an empty footer, so that a result coming
        // or going never folds or unfolds the scenes.
        let pane_bottom = ui.available_rect_before_wrap().bottom();
        let body_bottom = footer.show(ui);
        let footer_height = pane_bottom - body_bottom;
        match footer_act.take() {
            Some(FooterAct::GoToGroup) => {
                self.select(self.group_session);
            }
            Some(FooterAct::UndoGroupDiscard) => self.wants_group_undo = true,
            Some(FooterAct::UndoGroupRemove) => group_undo_in_footer = true,
            Some(FooterAct::UndoDeletedRun) => self.undo_deleted_run(),
            Some(FooterAct::UndoAdd(id)) => {
                book.undo_add(&id);
                self.added = None;
            }
            Some(FooterAct::UndoRemove) => {
                book.undo_remove();
            }
            Some(FooterAct::Scene(act)) => scene_act = Some(act),
            None => {}
        }
        file_act(book, file.take());
        if let Some(id) = &added {
            if let Some(event) = book.events.iter_mut().find(|event| event.id == *id) {
                event.title = event.kind.word().to_owned();
            }
            let days = timeline::day_numbers(&book.events);
            let index = book.events.iter().position(|event| event.id == *id);
            self.row_reveal = Some((id.clone(), 0, ui.ctx().cumulative_pass_nr()));
            self.result_orders.renew("add");
            self.added = Some(Added {
                id: id.clone(),
                label: index.map_or_else(String::new, |at| book.events[at].label(days[at])),
                at: Instant::now(),
                count: book.events.len(),
            });
            self.open_event = None;
        }
        let days = timeline::day_numbers(&book.events);
        let mut insert = None;
        let mut remove = None;
        let mut remove_group = None;
        let undo_group_remove = group_undo_in_footer;
        let mut remove_fight = None;
        let mut delete_run = None;
        let mut toggle = None;
        let mut changed = false;
        let superseded = self.removal_superseded(book);
        let removed_row = book
            .removed_row()
            .filter(|_| self.removed_event_in_plan && !superseded)
            .map(|(at, label)| {
                let words = removed_event_words(book, session, &days, &label, &planned_results);
                (at, words)
            });
        let mut undo_row_remove = false;
        let event_count = book.events.len();
        let open_event = self.open_event.clone();
        let ruling_undo = book.rest_undo_id().map(str::to_owned);
        let mut ruling = None;
        let mut pane = None;
        // A journey's checks: the clock at the open journey's start, and the results that hold.
        let (open_at, _) = self.journey_start(book, open_event.as_deref());
        checks::settle_length(book, open_at);
        // A length that settled changed the Timeline, so the start is read again.
        let (_, journey_start) = self.journey_start(book, open_event.as_deref());
        self.travel.done.follow(book);
        if self
            .travel
            .planned
            .as_ref()
            .is_some_and(|(_, _, undo)| self.draft.as_ref().is_none_or(|draft| !undo.live(draft)))
        {
            self.travel.planned = None;
        }
        let mut length_taken = checks::Taken::Nothing;
        let mut travel_act = None;
        self.rewards.see(self.draft.as_ref(), session, book);
        let revision = self.draft.as_ref().map(crate::sessions::Draft::revision);
        let mut rewards_act = None;
        let mut travel_undo = (TravelUndo::None, String::new());
        // A name a result links: an encounter or a group, and whether a row for it is to come.
        let in_plan = |name: &str| {
            let fight = plan
                .fights
                .iter()
                .any(|fight| fight.name.eq_ignore_ascii_case(name));
            let group = plan
                .groups
                .iter()
                .any(|group| group.name.eq_ignore_ascii_case(name));
            (fight || group).then(|| {
                planned
                    .iter()
                    .any(|fight| fight.name.eq_ignore_ascii_case(name))
                    || planned_groups
                        .iter()
                        .any(|group| group.name.eq_ignore_ascii_case(name))
            })
        };
        // Review of PR 274: the DM's own scroll or navigation, or a draw the plan missed (the
        // DM left the view), ends a pending reveal, so it never pulls the plan back.
        let pass = ui.ctx().cumulative_pass_nr();
        if self
            .row_reveal
            .as_ref()
            .is_some_and(|(_, _, last)| pass > last + 1 || user_moved_the_view(ui))
        {
            self.row_reveal = None;
        }
        let mut reveal = Reveal::new(self.row_reveal.as_ref().map(|(id, _, _)| id.clone()));
        egui::CentralPanel::default().frame(egui::Frame::new()).show_inside(ui, |ui| {
            // The rows leave the scroll bar's lane free whether or not the bar shows, so a row's
            // actions keep their x when the plan grows long enough to scroll.
            let lane_width = ui.available_width() - ui.spacing().scroll.allocated_width();
            // The rows end above the message footer, whatever the panel's own bottom.
            let rows_height = (body_bottom - ui.cursor().top()).max(0.0);
            // A scene's box reaches `box_hang` before the rows, so that every row's words start
            // at one column (`scene_box`). The scroll area starts that much before the rows, so
            // the box lies whole inside it, and the rows keep their place.
            let hang = box_hang();
            let area = ui.max_rect().with_min_x(ui.max_rect().left() - hang);
            let clip = ui.clip_rect().with_min_x(area.left());
            let mut area = ui.new_child(egui::UiBuilder::new().max_rect(area).layout(*ui.layout()));
            area.set_clip_rect(clip);
            let output = egui::ScrollArea::vertical().id_salt(("session_plan", session))
                .max_height(rows_height)
                .auto_shrink([false, false]).show(&mut area, |outer| {
                let mut rows = outer.new_child(egui::UiBuilder::new()
                    .max_rect(outer.max_rect().with_min_x(outer.max_rect().left() + hang))
                    .layout(*outer.layout()));
                let ui = &mut rows;
                ui.set_max_width(lane_width);
                let width = lane_width.min(ui.available_width()) - design::M;
                let action_width = [Action::primary("Start fight").icon(icons::RUN), Action::primary("Resume").icon(icons::RUN), Action::quiet("Add to notes"), Action::secondary("Arrivals").icon(icons::GROUP)]
                    .into_iter().map(|action| action.density(Density::Read).natural_width(ui)).fold(0.0, f32::max)
                    + Density::Read.control() + design::S;
                let widths = [width - action_width - design::S, action_width];
                let rewards_width = Action::quiet(ADD_REWARDS).density(Density::Read).natural_width(ui)
                    + Density::Read.control() + design::S;
                let mut any = false;
                let mut primary_available = plan.open_run.is_none() && !self.confirm_record;
                // The opened played row, on the frame the DM opened it, and the next fight's action.
                let mut opened_played = None;
                let mut next_action = None;
                for (at, event) in book.events.iter_mut().enumerate() {
                    if let Some((removed_at, words)) = &removed_row && *removed_at == at {
                        ui.add_space(design::S);
                        undo_row_remove |= design::undo_message(ui, Tone::Success, words, "Undo remove");
                    }
                    if !event.is_in(session) { continue; }
                    any = true;
                    let day = days.get(at).copied().unwrap_or(0);
                    let label = event.label(day);
                    let is_open = open_event.as_ref() == Some(&event.id);
                    let running = event.run.is_some() && event.run == plan.open_run;
                    let saved_fight = plan.fights.iter().find(|fight| fight.saved.as_ref()
                        .is_some_and(|saved| saved.id == event.run && event.run.is_some()));
                    let saved = plan.saved_runs.iter().find(|saved| {
                        saved.standing == crate::encounter_run::Standing::Unfinished
                            && saved.id.is_some()
                            && saved.id == event.run
                            && !running
                    });
                    let summary = if event.run.is_some() {
                        timeline::played_fight_summary(event, running)
                    } else {
                        event.summary(None)
                    };
                    let row = ui.push_id(event.id.clone(), |ui| {
                        // A fight that ended: its rewards (F7), on the row when the title keeps its lines.
                        let ended = event.run.filter(|_| !running && saved.is_none());
                        let lead = if is_open { icons::DOWN } else { icons::NEXT };
                        let listed = self.rewards.listed(&event.id);
                        let place = self.rewards.fits(ui, &label, lead, ui.available_width(), rewards_width);
                        let on_row = ended.is_some() && place == RewardsAt::Row;
                        let row_widths = if on_row {
                            [width - rewards_width - design::S, rewards_width]
                        } else if event.run.is_some() && (running || saved.is_none()) {
                            [width - Density::Read.control() - design::S, Density::Read.control()]
                        } else { widths };
                        let top = ui.cursor().top();
                        plan_row(ui, &row_widths, lead, |ui| {
                            if plan_link(ui, &label, is_open).clicked() { toggle = Some(event.id.clone()); }
                        }, |ui| {
                            if running {
                                if plan_run_link(ui, &plan.open_state).clicked() {
                                    self.wants_back_to_run = true;
                                }
                            } else if let Some(saved) = saved {
                                design::status(ui, Status { icon: icons::RUN,
                                    word: &plan_saved_status(saved), color: c.muted });
                            } else if !(is_open && event.kind == Kind::LongRest) {
                                plan_event_summary(ui, &summary, (event.kind == Kind::LongRest).then_some(event.restores),
                                    event.time_warning().filter(|_| event.run.is_none()).as_deref());
                            }
                        }, |ui| {
                            if let Some(saved) = saved {
                                design::row_menu(ui, &label, |ui| {
                                    if let Some(fight) = saved_fight
                                        && Action::quiet("Start a new run").blocked(plan.blocked).show(ui).clicked() {
                                        self.wants_fight = Some(fight.path.clone()); ui.close();
                                    }
                                    if Action::quiet("Delete the saved run").blocked(plan.blocked).show(ui).clicked() {
                                        delete_run = Some(saved.clone()); ui.close();
                                    }
                                    if Action::quiet("Remove").name("Remove this event").show(ui).clicked() {
                                        remove = Some(event.id.clone()); ui.close();
                                    }
                                });
                                let action = if std::mem::take(&mut primary_available) { Action::primary("Resume") } else { Action::secondary("Resume") };
                                if action.icon(icons::RUN).name(&format!("Resume: {label}")).blocked(plan.blocked).density(Density::Read).show(ui).clicked() {
                                    self.wants_saved_run = Some(saved.path.clone());
                                }
                            } else {
                                // Overflow is always at the row's right edge.
                                let more = design::icon_button(ui, icons::EXPAND, &format!("More actions for {label}"));
                                egui::Popup::menu(&more).show(|ui| {
                                    if checks::offered(event, plan.tables) && Action::quiet("Roll for encounters").show(ui).clicked() {
                                        travel_act = Some((event.id.clone(), checks::Act::Start)); ui.close();
                                    }
                                    if Action::quiet("Remove").name("Remove this event")
                                        .blocked_if(running, "This run is open. Close it first.").show(ui).clicked() {
                                        remove = Some(event.id.clone()); ui.close();
                                    }
                                    if running { wrapped_words(ui, Type::Small, "This run is open. Close it first.", c.muted); }
                                    // Last, so no line of today's menu moves.
                                    if let Some(run) = ended.filter(|_| place == RewardsAt::Menu) {
                                        if listed {
                                            design::status(ui, Status { icon: icons::OK, word: "In notes", color: c.document.saved });
                                        } else if Action::quiet(ADD_REWARDS).show(ui).clicked() {
                                            rewards_act = Some(RewardsAct::Add(event.id.clone(), run)); ui.close();
                                        }
                                    }
                                });
                                if let Some(run) = ended.filter(|_| on_row) {
                                    if listed {
                                        ui.add_space(design::S);
                                        design::status(ui, Status { icon: icons::OK, word: "In notes", color: c.document.saved });
                                    } else if Action::quiet(ADD_REWARDS).name(&format!("Add rewards: {label}"))
                                        .density(Density::Read).show(ui).clicked() {
                                        rewards_act = Some(RewardsAct::Add(event.id.clone(), run));
                                    }
                                }
                                if !running && event.run.is_none() {
                                    let linked = body.contains(&format!("^{}", event.id));
                                    if linked {
                                        ui.add_space(design::S);
                                        design::status(ui, Status { icon: icons::OK, word: "In notes", color: c.document.saved });
                                    }
                                    else if Action::quiet("Add to notes").name(&format!("Add to notes: {label}"))
                                        .density(Density::Read).show(ui).clicked() { insert = Some(timeline::reference(event, day)); }
                                }
                            }
                        });
                        closed += ui.cursor().top() - top;
                        let details_top = ui.cursor().top();
                        if is_open {
                            plan_details(ui, |ui| {
                            if running {
                                self.fight = None;
                                wrapped_words(ui, Type::Small, "This fight is running. Its final HP shows here when it ends.", c.muted);
                            } else if let Some(run) = event.run {
                                let link = ended.filter(|_| place == RewardsAt::Link).map(|_| listed);
                                if self.fight_block(ui, run, &event.id, plan, link) {
                                    rewards_act = Some(RewardsAct::Add(event.id.clone(), run));
                                }
                                changed |= place_field(ui, event);
                            } else {
                                // A journey's checks come first, under its row; Title and Notes follow them.
                                if checks::shown(event) {
                                    let (act, taken, undo) = self.travel_block(ui, event, journey_start, plan, &in_plan, &mut book.length);
                                    if taken != checks::Taken::Nothing { length_taken = taken; }
                                    if let Some(act) = act { travel_act = Some((event.id.clone(), act)); }
                                    if undo != TravelUndo::None { travel_undo = (undo, event.id.clone()); }
                                    ui.add_space(design::XS);
                                }
                                changed |= event_fields(ui, event);
                                if let Some(action) = rest_ruling(ui, event, ruling_undo.as_ref() == Some(&event.id)) {
                                    ruling = Some((event.id.clone(), action));
                                }
                                if design::link(ui, "Timeline").clicked() { self.wants_event = Some(event.id.clone()); }
                            }
                            });
                        }
                        details_above += ui.cursor().top() - details_top;
                        if let (Some(note), Some(revision)) = (session, revision)
                            && let Some(act) = self.rewards.message(ui, note, &event.id, revision) {
                            rewards_act = Some(act);
                        }
                        if self.row_notice == Some((session, at)) && self.deleted_run.is_some() {
                            ui.add_space(design::S);
                            self.deleted_run_in_row(ui);
                        }
                        closed += plan_rule(ui);
                    });
                    // The opened row and the next row stay in reach.
                    if is_open && std::mem::take(&mut self.reveal_event) {
                        // A played fight's row waits for the next fight's action (review round 1).
                        if event.run.is_some() { opened_played = Some(row.response.rect); } else { row.response.scroll_to_me(Some(egui::Align::TOP)); }
                    }
                    reveal.after(ui, Some(&event.id), row.response.rect);
                }
                if let Some((at, words)) = &removed_row && *at >= event_count {
                    ui.add_space(design::S);
                    undo_row_remove |= design::undo_message(ui, Tone::Success, words, "Undo remove");
                }
                let days = self.planned_days(book, session, plan);
                for (position, fight) in planned.iter().enumerate() {
                    let day = planned_day(&days, &planned, position);
                    self.removed_fight_message(ui, session, position);
                    if !folded {
                        closed_scenes += self.scene_rows(ui, width, &scenes[position], position, shown, plan, &mut scene_act);
                    }
                    any = true;
                    let running = plan.open_run.is_some() && plan.open_encounter == Some(fight.name.as_str());
                    let saved = planned_saved_run(fight, plan.fights, plan.saved_runs);
                    let fight_row = ui.push_id((&fight.path, position), |ui| {
                        let row_widths = if running {
                            [width - Density::Read.control() - design::S, Density::Read.control()]
                        } else { widths };
                        let top = ui.cursor().top();
                        plan_row(ui, &row_widths, if self.open_planned.as_ref() == Some(&fight.path) { icons::DOWN } else { icons::NEXT }, |ui| {
                            if plan_link(ui, &fight.name, self.open_planned.as_ref() == Some(&fight.path)).clicked() {
                                self.open_planned = (self.open_planned.as_ref() != Some(&fight.path)).then(|| fight.path.clone());
                            }
                        }, |ui| {
                            if running {
                                if plan_run_link(ui, &plan.open_state).clicked() {
                                    self.wants_back_to_run = true;
                                }
                            } else if let Some(saved) = saved {
                                design::status(ui, Status { icon: icons::RUN, word: &plan_saved_status(saved), color: c.muted });
                            } else { planned_summary(ui, &on_road(ui, &fight.summary, day.as_deref()), Some(&fight.difficulty)); }
                        }, |ui| {
                            design::row_menu(ui, &fight.name, |ui| {
                                if Action::quiet("Remove from tonight").show(ui).clicked() {
                                    remove_fight = Some((position, fight.name.clone())); ui.close();
                                }
                                if saved.is_some() {
                                    if Action::quiet("Start a new run").blocked(plan.blocked).show(ui).clicked() {
                                        self.wants_fight = Some(fight.path.clone()); ui.close();
                                    }
                                    if Action::quiet("Delete the saved run").blocked(plan.blocked).show(ui).clicked() {
                                        delete_run = saved.cloned(); ui.close();
                                    }
                                }
                                // Last, so no line of today's menu moves (W1-D1).
                                if edit_encounter_line(ui, plan) { self.wants_edit_encounter = Some(fight.id); ui.close(); }
                            });
                            if running {
                                // The run strip owns Back to run.
                            } else if let Some(saved) = saved {
                                let action = if std::mem::take(&mut primary_available) { Action::primary("Resume") } else { Action::secondary("Resume") };
                                let shown = action.icon(icons::RUN).name(&format!("Resume: {}", fight.name)).density(Density::Read).blocked(plan.blocked).show(ui);
                                if position == 0 { next_action = Some(shown.rect); }
                                if shown.clicked() {
                                    self.wants_saved_run = Some(saved.path.clone());
                                }
                            } else {
                                let action = if std::mem::take(&mut primary_available) {
                                    Action::primary("Start fight")
                                } else { Action::secondary("Start fight") };
                                let shown = action.icon(icons::RUN).name(&format!("Start fight: {}", fight.name))
                                    .density(Density::Read).blocked(plan.blocked).show(ui);
                                if position == 0 { next_action = Some(shown.rect); }
                                if shown.clicked() {
                                    self.wants_fight = Some(fight.path.clone());
                                }
                            }
                        });
                        closed += ui.cursor().top() - top;
                        if self.row_notice == Some((session, event_count + position)) && self.deleted_run.is_some() {
                            ui.add_space(design::S);
                            self.deleted_run_in_row(ui);
                        }
                        let details_top = ui.cursor().top();
                        if self.open_planned.as_ref() == Some(&fight.path) {
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
                        closed += plan_rule(ui);
                    });
                    reveal.after(ui, None, fight_row.response.rect);
                }
                self.removed_fight_message(ui, session, planned.len());
                if !folded {
                    closed_scenes += self.scene_rows(ui, width, &scenes[planned.len()], planned.len(), shown, plan, &mut scene_act);
                }
                any |= scenes.iter().any(|scenes| !scenes.is_empty());
                for (position, group) in planned_groups.iter().enumerate() {
                    any = true;
                    let row = ui.push_id(group.id, |ui| {
                        let mut read_group = false;
                        let edit_word = if group.warnings.is_empty() { "Edit" } else { "Edit group" };
                        // With a fight open, the run's words and plus icon: the group joins it through Arrivals.
                        // A group none of whose members can join has only its repair to offer.
                        let joins_fight = plan.arrivals_open && !group.members.is_empty();
                        let group_action = if joins_fight { Action::quiet(ADD_TO_FIGHT).icon(icons::ADD) } else { Action::quiet(edit_word) };
                        let action_width = group_action.density(Density::Read).natural_width(ui) + Density::Read.control() + design::S;
                        let group_widths = [width - action_width - design::S, action_width];
                        let top = ui.cursor().top();
                        plan_row(ui, &group_widths, icons::GROUP, |ui| {
                            if design::wrapped_link(ui, &group.name).clicked() { read_group = true; }
                        }, |ui| {
                            let summary = plan_summary_text(ui, group);
                            // Members stay whole, so a long member list can take a third line
                            // before it shortens. A flagged group shows every fact.
                            plan_group_summary(ui, &summary, if group.warnings.is_empty() { 3 } else { usize::MAX });
                            // Name each member left out, its base and its line, beside Edit group, each
                            // in its own fault's tone; then the group's own reason, as a warning.
                            for warning in &group.warnings {
                                ui.add_space(design::XS);
                                design::small_message(ui, warning.tone(), warning);
                            }
                            for reason in group.own_problems() {
                                ui.add_space(design::XS);
                                design::small_message(ui, Tone::Warning, reason);
                            }
                        }, |ui| {
                            design::row_menu(ui, &group.name, |ui| {
                                if Action::quiet("Remove from tonight").name(&format!("Remove group {} from the plan", group.name))
                                    .blocked(plan.group_blocked).show(ui).clicked() {
                                    remove_group = Some((position, group.name.clone(), 0.0)); ui.close();
                                }
                                // Add to fight takes the action place; the repair its faults name stays one step away.
                                if joins_fight && !group.warnings.is_empty()
                                    && Action::quiet("Edit group").name(&format!("Edit group {}", group.name))
                                        .blocked(plan.group_blocked).show(ui).clicked() {
                                    self.wants_group = Some(group.path.clone()); self.group_session = self.selected; ui.close();
                                }
                            });
                            if joins_fight {
                                if Action::quiet(ADD_TO_FIGHT).icon(icons::ADD)
                                    .name(&format!("Add {} to fight", group.name)).density(Density::Read).show(ui).clicked() {
                                    self.wants_arrivals = true;
                                }
                            } else if Action::quiet(edit_word).name(&format!("Edit group {}", group.name))
                                .density(Density::Read).blocked(plan.group_blocked).show(ui).clicked() {
                                self.wants_group = Some(group.path.clone()); self.group_session = self.selected;
                            }
                        });
                        closed += ui.cursor().top() - top;
                        if read_group { self.wants_group = Some(group.path.clone()); self.group_session = self.selected; }
                        closed += plan_rule(ui);
                    });
                    if let Some((at, _, height)) = &mut remove_group && *at == position { *height = row.response.rect.height(); }
                    reveal.after(ui, None, row.response.rect);
                }
                if !any {
                    design::empty_state(ui, icons::SESSIONS, "Nothing planned yet",
                        "Use Add to plan a fight, a group, or an event.", |_| {});
                }
                reveal_opened_played(ui, opened_played, next_action);
                reveal.finish(ui);
                self.scene_field(ui, index.as_deref(), shown, &mut scene_act);
                let drawn = ui.min_rect();
                outer.advance_cursor_after_rect(drawn);
            });
            ui.advance_cursor_after_rect(area.min_rect());
            pane = Some(output.inner_rect.height() + footer_height);
        });
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
            self.rewards_act(act, book, session, plan);
        }
        // A draft is open only for a selected session, so the act has its session.
        if let Some((position, name)) = remove_fight
            && let Some(draft) = &mut self.draft
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
                    .filter(|fight| fight.name.eq_ignore_ascii_case(&name))
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
        if undo_group_remove
            && let Some((_, _, before, _, _, _)) = self.removed_group.take()
            && let Some(draft) = &mut self.draft
        {
            draft.body = before;
            draft.edited();
        }
        if let Some((position, name, height)) = remove_group
            && let Some(draft) = &mut self.draft
        {
            let before = draft.body.clone();
            unlink_group(&mut draft.body, &name);
            draft.edited();
            self.result_orders.renew("group remove");
            self.removed_group =
                Some((session, position, before, draft.body.clone(), height, name));
            self.plan_notice_state = Some((draft.body.clone(), book.events.clone()));
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
            (TravelUndo::PlanLine, _) => self.undo_plan_line(),
            (TravelUndo::None, _) => {}
        }
        if let Some((id, action)) = ruling {
            action.apply(book, &id);
            self.added = None;
        }
        if let Some(saved) = delete_run {
            match crate::encounter_run::DeletedRun::delete(&saved) {
                Ok(deleted) => {
                    self.row_notice = book
                        .events
                        .iter()
                        .position(|event| event.run == saved.id)
                        .or_else(|| {
                            planned
                                .iter()
                                .position(|fight| {
                                    planned_saved_run(fight, plan.fights, plan.saved_runs)
                                        .is_some_and(|run| run.id == saved.id)
                                })
                                .map(|at| event_count + at)
                        })
                        .map(|at| (session, at));
                    self.plan_notice_state = Some((
                        self.draft
                            .as_ref()
                            .map_or_else(String::new, |draft| draft.body.clone()),
                        book.events.clone(),
                    ));
                    self.record_deleted_run(book, deleted, saved.id);
                }
                Err(error) => self.rename_problem = Some(error),
            }
        }
        if let Some(id) = toggle {
            self.fight = None;
            self.show_fight_log = false;
            self.reveal_event = true;
            self.open_event = (self.open_event.as_ref() != Some(&id)).then_some(id);
        }
        if let Some(id) = remove {
            book.remove(&id);
            self.removed_event_in_plan = true;
            self.plan_notice_state = Some((
                self.draft
                    .as_ref()
                    .map_or_else(String::new, |draft| draft.body.clone()),
                book.events.clone(),
            ));
            self.open_event = None;
        }
        if undo_row_remove {
            book.undo_remove();
            self.removed_event_in_plan = false;
            self.plan_notice_state = None;
        }
        if let (Some(link), Some(draft), Some(session)) = (insert, &mut self.draft, session) {
            insert_in_note(&mut draft.body, &link);
            draft.edited();
            self.added = None;
            self.mentions
                .insert(session, timeline::referenced_ids(&draft.body));
        }
    }

    /// The scenes at one place among the plan's rows, with a removal's Undo at its row's place.
    /// Returns the rows' height when closed, without the messages, for the fold.
    #[allow(clippy::too_many_arguments)]
    fn scene_rows(
        &mut self,
        ui: &mut egui::Ui,
        width: f32,
        scenes: &[(usize, &Scene)],
        position: usize,
        shown: Shown,
        plan: &Plan<'_>,
        act: &mut Option<SceneAct>,
    ) -> f32 {
        let mut height = 0.0;
        for (at, (line, scene)) in scenes.iter().enumerate() {
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
    fn shown_at(&self, place: Place, rows: Option<&[Vec<(usize, &Scene)>]>) -> Place {
        let drawn = match place {
            Place::Field => self.scenes.field.is_some(),
            Place::Row((position, at)) => {
                rows.is_some_and(|rows| rows.get(position).is_some_and(|row| at <= row.len()))
            }
            Place::Footer => true,
        };
        if drawn { place } else { Place::Footer }
    }

    /// The scene act's words under the place where the DM acted, while they show there.
    fn row_fault(&self, ui: &mut egui::Ui, place: Place, shown: Shown) {
        if let Some(fault) = &self.scenes.fault
            && shown.fault == Some(place)
        {
            ui.add_space(design::XS);
            design::message(ui, Tone::Warning, &fault.words);
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
    pub(super) fn plan_model(
        &mut self,
        index: Option<&ContentIndex>,
        plan: &Plan<'_>,
    ) -> Arc<PlanModel> {
        use std::hash::{Hash, Hasher};
        let mut names = std::collections::hash_map::DefaultHasher::new();
        for fight in plan.fights {
            fight.name.hash(&mut names);
        }
        0u8.hash(&mut names);
        for group in plan.groups {
            group.name.hash(&mut names);
        }
        let key = (
            self.selected,
            self.draft
                .as_ref()
                .map_or(0, crate::sessions::Draft::revision),
            index.map(ContentIndex::generation),
            names.finish(),
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

    /// `Removed “…” from tonight.` with `Undo remove`, at `place`, while the note is as the
    /// removal left it. A finished change: the success tone.
    fn removed_scene_message(&self, ui: &mut egui::Ui, place: Place, act: &mut Option<SceneAct>) {
        let Some((_, words, undo)) = &self.scenes.removed else {
            return;
        };
        if self.draft.as_ref().is_none_or(|draft| !undo.live(draft)) {
            return;
        }
        ui.add_space(design::S);
        if design::undo_message(
            ui,
            Tone::Success,
            &format!("Removed “{words}” from tonight."),
            "Undo remove",
        ) {
            *act = Some(SceneAct::UndoRemove(place));
        }
    }

    /// The scenes' results whose own place is not drawn, in the plan's message footer
    /// (`shown_at`): the Scene field's once it closes, a row's while the scenes fold, and a
    /// removal whose row's place is gone. The Undo stays the note's, so it stays in reach.
    fn scene_messages<'a>(
        &self,
        footer: &mut design::Footer<'a>,
        undos: &mut LiveUndos<'a>,
        act: &'a std::cell::Cell<Option<FooterAct>>,
        shown: Shown,
    ) {
        if let Some(fault) = &self.scenes.fault
            && shown.fault == Some(Place::Footer)
        {
            let words = fault.words.clone();
            footer.reason(words.clone(), move |ui| {
                design::message(ui, Tone::Warning, &words);
            });
        }
        if shown.added == Some(Place::Footer)
            && let Some(undo) = &self.scenes.added
            && self.draft.as_ref().is_some_and(|draft| undo.live(draft))
        {
            const ADDED: &str = "Scene added to tonight.";
            // A cheap identity: the session and the length the act left, never the words.
            let order = self
                .result_orders
                .of("scene", (self.selected, undo.after().len()));
            undos.push(order, ADDED.to_owned(), move |ui| {
                if design::undo_message(ui, Tone::Success, ADDED, "Undo scene") {
                    act.set(Some(FooterAct::Scene(SceneAct::UndoAdd)));
                }
            });
        }
        if shown.removed == Some(Place::Footer)
            && let Some((_, words, undo)) = &self.scenes.removed
            && self.draft.as_ref().is_some_and(|draft| undo.live(draft))
        {
            let words = format!("Removed “{words}” from tonight.");
            // A cheap identity: the session and the length the act left.
            let order = self
                .result_orders
                .of("scene-removed", (self.selected, undo.after().len()));
            undos.push(order, words.clone(), move |ui| {
                if design::undo_message(ui, Tone::Success, &words, "Undo remove") {
                    act.set(Some(FooterAct::Scene(SceneAct::UndoRemove(Place::Footer))));
                }
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
        let Some(state) = &mut self.scenes.field else {
            self.scenes.typing = false;
            return;
        };
        let reveal = state.focus;
        ui.add_space(design::S);
        let result = design::add_field(
            ui,
            state,
            "Plan a scene",
            "Plan a scene: words, a person or a place",
            0.0,
            |_| None,
            |words| {
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
            },
        );
        let typed = state.words.trim().to_owned();
        let emptied = state.words.is_empty();
        self.scenes.typing = result.field.has_focus();
        self.field_result(ui, reveal.then_some(result.field.rect), shown, act);
        if let Some(link) = result.chosen {
            *act = Some(SceneAct::Add(link));
        } else if result.enter && !typed.is_empty() {
            *act = Some(SceneAct::Add(typed));
        }
        if ui.input(|input| input.key_pressed(egui::Key::Escape))
            && (result.field.has_focus() || result.field.lost_focus())
            && emptied
        {
            self.scenes.field = None;
        }
    }

    /// The Scene field's fault, and `Scene added to tonight.` with `Undo scene`, under the open
    /// field, so the field never moves for its own result. With `reveal`, the field's rectangle,
    /// the field and its words scroll into view together.
    fn field_result(
        &self,
        ui: &mut egui::Ui,
        reveal: Option<egui::Rect>,
        shown: Shown,
        act: &mut Option<SceneAct>,
    ) {
        self.row_fault(ui, Place::Field, shown);
        if shown.added == Some(Place::Field)
            && let Some(undo) = &self.scenes.added
            && self.draft.as_ref().is_some_and(|draft| undo.live(draft))
        {
            ui.add_space(design::XS);
            if design::undo_message(ui, Tone::Success, "Scene added to tonight.", "Undo scene") {
                *act = Some(SceneAct::UndoAdd);
            }
        }
        if let Some(field) = reveal {
            let bottom = ui.cursor().top().max(field.bottom());
            ui.scroll_to_rect(field.with_max_y(bottom), None);
        }
    }

    /// One scene act, through the note's one act path: it follows the disk, changes the one line
    /// that the row was drawn from and saves at once. What it could not do shows where the DM
    /// acted. An act whose save failed keeps its Undo: the fragment waits in the draft.
    fn scene_act(&mut self, act: SceneAct, session: Option<Uuid>) {
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
        let scenes = &mut self.scenes;
        scenes.fault = None;
        let (result, place) = match act {
            SceneAct::StartFight(_) | SceneAct::AddToLog(..) => return,
            SceneAct::Add(words) => {
                let result = draft.apply(note, Apply::Now, |text| Some(add_scene(text, &words)));
                if let Ok(undo) | Err(ActFault::NotSaved(_, undo)) = &result {
                    scenes.added = Some(undo.clone());
                    self.result_orders.renew("scene");
                    if let Some(field) = &mut scenes.field {
                        field.clear();
                        field.focus = true;
                    }
                }
                (result.map(drop), Place::Field)
            }
            SceneAct::Tick(slot, task) => (
                draft
                    .apply(note, Apply::Now, |text| {
                        change_line(
                            text,
                            task.at,
                            &task.line,
                            Some(&ticked(&task.line, !task.done)),
                        )
                    })
                    .map(drop),
                Place::Row(slot),
            ),
            SceneAct::Remove(slot, line, task) => {
                let result = draft.apply(note, Apply::Now, |text| {
                    change_line(text, task.at, &task.line, None)
                });
                if let Ok(undo) | Err(ActFault::NotSaved(_, undo)) = &result {
                    scenes.removed = Some((line, plain_links(&task.words), undo.clone()));
                }
                (result.map(drop), Place::Row(slot))
            }
            SceneAct::UndoAdd => (
                scenes.added.take().map_or(Ok(()), |undo| undo.undo(draft)),
                Place::Field,
            ),
            SceneAct::UndoRemove(place) => (
                scenes
                    .removed
                    .take()
                    .map_or(Ok(()), |(_, _, undo)| undo.undo(draft)),
                place,
            ),
        };
        if let Err(fault) = result {
            scenes.fault = Some(Fault {
                words: fault.words().to_owned(),
                met: draft.body.clone(),
                place,
                waits: matches!(fault, ActFault::NotSaved(..)),
            });
        }
    }

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
        ui.add_space(design::S);
        if design::undo_message(
            ui,
            Tone::Success,
            &format!("Removed “{name}” from tonight."),
            "Undo remove",
        ) {
            if let Some(draft) = &mut self.draft {
                let _ = undo.undo(draft);
            }
            self.removed_fight = None;
            self.plan_notice_state = None;
        }
    }

    /// The plan's results end at the next edit of the note or the timeline events.
    fn end_plan_results_after_edits(&mut self, book: &Book) {
        if let Some((body, events)) = &self.plan_notice_state
            && (self.draft.as_ref().is_none_or(|draft| draft.body != *body)
                || book.events != *events)
        {
            self.deleted_run = None;
            self.row_notice = None;
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
        self.result_orders.renew("deleted run");
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

    /// The deleted run's result at the row it was deleted from: the row owns it.
    fn deleted_run_in_row(&mut self, ui: &mut egui::Ui) {
        if let Some((tone, words)) = self.deleted_run_words()
            && design::undo_message(
                ui,
                tone,
                &words,
                crate::encounter_run::DeletedRun::UNDO_LABEL,
            )
        {
            self.undo_deleted_run();
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

    fn deleted_run_message<'a>(
        &self,
        undos: &mut LiveUndos<'a>,
        act: &'a std::cell::Cell<Option<FooterAct>>,
    ) {
        if let Some((tone, words)) = self.deleted_run_words() {
            let order = self.result_orders.of("deleted run", &words);
            undos.push(order, words.clone(), move |ui| {
                if design::undo_message(
                    ui,
                    tone,
                    &words,
                    crate::encounter_run::DeletedRun::UNDO_LABEL,
                ) {
                    act.set(Some(FooterAct::UndoDeletedRun));
                }
            });
        }
    }

    /// What can be undone in the plan: the event just added or just removed.
    #[expect(
        clippy::too_many_arguments,
        reason = "the plan’s parts that its results name"
    )]
    fn plan_messages<'a>(
        &mut self,
        ui: &egui::Ui,
        undos: &mut LiveUndos<'a>,
        act: &'a std::cell::Cell<Option<FooterAct>>,
        book: &Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
        planned: &[(String, bool)],
    ) {
        if self.row_notice.is_none() {
            self.deleted_run_message(undos, act);
        }
        let days = timeline::day_numbers(&book.events);
        // It goes at the DM's next edit, or soon, so that it cannot undo the wrong thing.
        if let Some(just) = &self.added {
            let current = book
                .events
                .iter()
                .position(|event| event.id == just.id)
                .map(|at| book.events[at].label(days[at]));
            if just.at.elapsed() >= UNDO_ADD_FOR
                || just.count != book.events.len()
                || current.as_ref() != Some(&just.label)
            {
                self.added = None;
            } else {
                ui.ctx().request_repaint_after(UNDO_ADD_FOR);
            }
        }
        if let Some(just) = &self.added {
            let to = self.session_name(session);
            let restores = book
                .events
                .iter()
                .any(|event| event.id == just.id && event.restores)
                .then(|| timeline::fights_that_keep_hp(&book.events, session, plan.saved_runs));
            let (id, words) = (
                just.id.clone(),
                added_message(&just.label, to.as_deref(), restores),
            );
            let order = self.result_orders.of("add", &id);
            undos.push(order, words.clone(), move |ui| {
                if design::undo_message(ui, Tone::Success, &words, "Undo add") {
                    act.set(Some(FooterAct::UndoAdd(id)));
                }
            });
        } else if !self.removed_event_in_plan
            && !self.removal_superseded(book)
            && let Some(label) = book.removed().map(str::to_owned)
        {
            let words = removed_event_words(book, session, &days, &label, planned);
            let order = self.result_orders.of("remove", book.removal_action());
            undos.push(order, words.clone(), move |ui| {
                if design::undo_message(ui, Tone::Success, &words, "Undo remove") {
                    act.set(Some(FooterAct::UndoRemove));
                }
            });
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
    fn fight_block(
        &mut self,
        ui: &mut egui::Ui,
        run: Uuid,
        event_id: &str,
        plan: &Plan<'_>,
        rewards: Option<bool>,
    ) -> bool {
        let mut add_rewards = false;
        let path = plan.run_directory.as_ref().and_then(|directory| {
            crate::encounter_run::find_run(directory, self.campaign.as_ref()?.id, run).ok()
        });
        if self.fight.as_ref().map(|(id, _)| *id) != Some(run) {
            let result = match (&path, &self.campaign) {
                (Some(path), Some(campaign)) => fight_summary(path, campaign.id),
                _ => Err("The run folder is not available.".to_owned()),
            };
            self.fight = Some((run, result));
        }
        let Some((_, result)) = &self.fight else {
            return false;
        };
        match result {
            Err(error) => {
                design::message(
                    ui,
                    Tone::Info,
                    &format!("This fight’s run cannot be read. {error}"),
                );
            }
            Ok(summary) => {
                let known = |n: Option<i64>| n.map_or_else(|| "?".to_owned(), |n| n.to_string());
                let mut cells = Vec::new();
                let mut widest = 0.0_f32;
                for row in in_party_order(&summary.rows, plan.party) {
                    let first = row.name.split_whitespace().next().unwrap_or(&row.name);
                    let hp = format!("{}/{}", known(row.hp), known(row.max_hp));
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
                    // than the word, never padded with spaces. One gap, `S`, at every size,
                    // also where the page's columns give the plan a wider one (X48).
                    ui.spacing_mut().item_spacing.x = LINK_GAP;
                    if word_link(ui, "Log").clicked() {
                        self.show_fight_log = !self.show_fight_log;
                    }
                    ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                    if word_link(ui, "Whole run").clicked() {
                        self.wants_saved_run = path;
                    }
                    ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                    if word_link(ui, "Timeline").clicked() {
                        self.wants_event = Some(event_id.to_owned());
                    }
                    if let Some(listed) = rewards {
                        ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                        if listed {
                            ui.label(Type::Body.rich(ui, "In notes").color(colors().muted));
                        } else if word_link(ui, ADD_REWARDS).clicked() {
                            add_rewards = true;
                        }
                    }
                });
                if self.show_fight_log {
                    egui::ScrollArea::vertical()
                        .id_salt(("fight_log", run))
                        .max_height(design::READER_MEASURE / 3.0)
                        .show(ui, |ui| {
                            for line in &summary.log {
                                wrapped_words(ui, Type::Small, line, colors().muted);
                            }
                        });
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
            summary: String::new(),
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
        let summary = "5 party · 2 × Ember Cultist";
        let day = "Day 2 of The road to the ford beyond the old northern quarry";
        let (mut full, mut short, mut hidden) = (0, 0, 0);
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
                            let words = on_road(ui, summary, Some(day));
                            if words.contains("of The road") {
                                full += 1;
                            } else if words.contains("Day 2") {
                                short += 1;
                            } else {
                                hidden += 1;
                            }
                            planned_summary(ui, &words, Some(&rating));
                        })
                        .response
                        .rect
                        .height();
                    assert_eq!(road, plain, "the day adds no height at {width} px");
                }
            });
        });
        assert!(
            full > 0 && short > 0 && hidden > 0,
            "{full} {short} {hidden}"
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
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 640.0))
            .build_state(
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
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discarded: false,
                        group_discarded_name: String::new(),
                        group_discarded_order: 0,
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
                    egui::CentralPanel::default().show(ctx, |ui| page.plan_pane(ui, book, &plan));
                },
                (page, book, vec![other, saved.clone()], None),
            );
        design::theme::install(&harness.ctx);
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
        use egui_kittest::kittest::Queryable;
        let mut harness = egui_kittest::Harness::builder().build(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                plan_event_summary(ui, "Old mill", Some(false), None);
            });
        });
        design::theme::install(&harness.ctx);
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
                        planned_summary(ui, "5 party · 2 × Ember Cultist", Some(&difficulty));
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
        let summary = "5 party · Cinder Tyrant · 2 × Ember Cultist, 1 × Wolf";
        let (mut whole, mut cut) = (0, 0);
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
                                planned_summary(ui, summary, rating);
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
                    egui::Shape::Text(text) if text.galley.job.text.contains("XP") => Some(
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
            let facts = line
                .strip_suffix("XP 6,300 · Above High")
                .unwrap_or_else(|| panic!("at {width} px: {line:?}"));
            let facts = facts.trim_end_matches([' ', '·']);
            if facts.replace(" · ", " ") == summary.replace(" · ", " ") {
                whole += 1;
            } else {
                // The summary gives way by whole facts and members, never inside one.
                cut += 1;
                let kept = facts
                    .strip_suffix('…')
                    .unwrap_or_else(|| panic!("{line:?}"));
                let kept = kept.trim_end_matches([' ', '·', ',']);
                assert!(
                    kept.is_empty()
                        || [
                            "5 party",
                            "5 party · Cinder Tyrant",
                            "5 party · Cinder Tyrant · 2 × Ember Cultist"
                        ]
                        .iter()
                        .any(|unit| kept.replace(" · ", " ") == unit.replace(" · ", " ")),
                    "at {width} px a member was cut inside: {line:?}"
                );
            }
        }
        assert!(whole > 0 && cut > 0, "whole {whole}, cut {cut}");
    }

    #[test]
    fn long_rest_added_on_plan_restores_and_undo_removes_its_ruling() {
        timeline::page::check_long_rest_add(&PLAN_KINDS);
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
                        &[design::FORM_WIDTH, design::FORM_WIDTH / 3.0],
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
    }

    #[test]
    fn i19_p5_group_summary_wraps_before_it_shortens() {
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH / 2.0, 200.0))
            .build_state(|ctx, rect| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    *rect = ui.scope(|ui| plan_group_summary(ui, "Rivals · 3 members · XP 600 · Changed · 1 × Ember Cultist · 2 × Goblin Warrior", 3)).response.rect;
                });
            }, egui::Rect::NOTHING);
        design::theme::install(&harness.ctx);
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
        use crate::sessions::{NewNote, create_note, plan_lines};
        use egui_kittest::kittest::Queryable;
        // Review round 1 (REVIEW-291-scene-state-height): a scene that links a rule note has no
        // second line; when the note goes, `Not found` ends its line and the row keeps its
        // height.
        let mut row = egui_kittest::Harness::builder()
            .with_size(egui::vec2(400.0, 800.0))
            .build_state(
                |ctx, state: &mut (bool, f32)| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &[],
                        saved_runs: &[],
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discarded: false,
                        group_discarded_name: String::new(),
                        group_discarded_order: 0,
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
                    };
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.set_width(330.0);
                        state.1 = scene_row(ui, 330.0, &scene, ((0, 0), 0), &plan, &mut None);
                    });
                },
                (false, 0.0),
            );
        design::theme::install(&row.ctx);
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
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 800.0))
            .build_state(
                |ctx, (page, book): &mut (Sessions, Book)| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &fights,
                        saved_runs: &[],
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discarded: false,
                        group_discarded_name: String::new(),
                        group_discarded_order: 0,
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
                    egui::CentralPanel::default().show(ctx, |ui| page.plan_pane(ui, book, &plan));
                },
                (page, Book::default()),
            );
        design::theme::install(&harness.ctx);
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
        // Two scenes, each with its box; the plain list line has no row. A done scene does not
        // count among the rows still to come.
        assert!(harness.query_by_label("Open: Captain Varra").is_some());
        assert!(
            harness
                .query_by_label("Done: The miller is the informer")
                .is_some()
        );
        assert!(harness.query_by_label_contains("Hobb").is_none());
        assert!(harness.query_by_label("TONIGHT 2").is_some());

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
        harness.get_by_label("Undo remove").click();
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
        harness.get_by_label("Undo remove").click();
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
        harness.get_by_label("Undo remove").click();
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
        harness.state_mut().0.added = Some(Added {
            id,
            label,
            at: Instant::now(),
            count,
        });
        harness.run_steps(4);
        assert!(
            harness.query_by_label("Undo add").is_some(),
            "the footer holds a result"
        );
        assert!(
            harness.query_by_label("Open: Pay the toll").is_some(),
            "a result in the footer does not fold the scenes"
        );
        harness.state_mut().0.added = None;
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
        assert!(harness.query_by_label("Undo remove").is_none());
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
        harness.get_by_label("Undo scene").click();
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
        assert!(harness.query_by_label("Undo scene").is_some());
        for state in ["Done", "Open"] {
            harness
                .get_by_label(&format!("{state}: Pay the toll"))
                .click();
            harness.run_steps(2);
        }
        assert!(harness.query_by_label("Undo scene").is_none());
        // So does another session.
        harness.get_by_label("Plan a scene").focus();
        harness.run_steps(1);
        harness
            .get_by_label("Plan a scene")
            .type_text("Rest at the ford");
        harness.run_steps(1);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        assert!(harness.query_by_label("Undo scene").is_some());
        harness.state_mut().0.select(None);
        harness.run_steps(2);
        harness.state_mut().0.select(Some(note.id));
        harness.run_steps(2);
        assert!(harness.query_by_label("Undo scene").is_none());
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
        harness.get_by_label("Undo scene").click();
        harness.run_steps(3);
        assert_eq!(read(), obsidian, "the outside edit stays");
        assert!(
            harness
                .query_all_by_label("That line changed on disk.")
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
                .query_by_label("The note changed on disk. Choose in the note first.")
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
        assert!(harness.query_by_label("Notes not saved").is_some());
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
        assert_eq!(
            (fault.words.as_str(), fault.place),
            ("That line changed on disk.", Place::Row((0, 0)))
        );
        assert_eq!(
            page.draft.as_ref().unwrap().body,
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
        assert_eq!(page.scenes.fault.as_ref().unwrap().words, "Notes not saved");
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
    }

    /// F7, rule 2: `Add rewards` goes on a played row only when the title keeps its width and
    /// its line breaks, laid out as `wrapped_link` lays it out. Two lines are not enough.
    #[test]
    fn the_rewards_fit_keeps_each_title_as_it_is() {
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(500.0, 800.0))
            .build_state(
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
        design::theme::install(&harness.ctx);
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
        use crate::encounter_run::{Participant, write_ended_run_with};
        use crate::sessions::{NewNote, create_note};
        use egui_kittest::kittest::Queryable;
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
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(design::FORM_WIDTH, 800.0))
            .build_state(
                |ctx, (page, book): &mut (Sessions, Book)| {
                    let mut group = |_: &mut egui::Ui| {};
                    let plan = Plan {
                        fights: &[],
                        saved_runs: &[],
                        groups: &[],
                        group_blocked: None,
                        group_open: false,
                        group_discarded: false,
                        group_discarded_name: String::new(),
                        group_discarded_order: 0,
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
                    egui::CentralPanel::default().show(ctx, |ui| page.plan_pane(ui, book, &plan));
                },
                (page, book),
            );
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let shown = |harness: &egui_kittest::Harness<'_, (Sessions, Book)>, words: &str| {
            harness.query_by_label_contains(words).is_some()
        };
        let block_a = format!(
            "- [[Timeline#^{a}|Bridge]]\n  - XP 200 for 1 at 0 HP · 1 still stood (XP 200)\n  - 30 gp\n"
        );
        let block_b = format!(
            "- [[Timeline#^{b}|Shrine]]\n  - XP 0 for 0 at 0 HP. XP not known for Captain Varra: enter an award here.\n"
        );

        // An edit on disk a moment before is in the words that the act changes. With no block
        // in any note, the block has XP and the result says how to stop it. The row then says
        // `In notes`, and the note beside the plan is asked to show the block.
        let outside = read().replace("## What happened\n", "## What happened\n\nThe ford held.\n");
        fs::write(&note.path, &outside).unwrap();
        harness.get_by_label("Add rewards: Bridge").click();
        harness.run_steps(2);
        let with_a = outside.replace(
            "## Loot and rewards\n\n",
            &format!("## Loot and rewards\n\n{block_a}\n"),
        );
        assert_eq!(read(), with_a);
        assert!(shown(
            &harness,
            "Rewards added under Loot and rewards. XP line added. Delete it in the note to leave XP out from now on. XP tonight: 200 for 1 fight."
        ));
        assert!(harness.query_by_label("Add rewards: Bridge").is_none());
        assert!(shown(&harness, "In notes"));
        assert!(harness.state().0.caret_to_show.is_some());

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
        assert_eq!(harness.query_all_by_label("Undo rewards").count(), 2);
        // Undo of A takes back A's block alone; B's stays.
        harness
            .query_all_by_label("Undo rewards")
            .next()
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
        assert_eq!(harness.query_all_by_label("Undo rewards").count(), 1);

        // Another session and back keeps an Undo: it is the note's. Undo B gives the words
        // back exactly.
        harness.state_mut().0.select(None);
        harness.run_steps(2);
        harness.state_mut().0.select(Some(note.id));
        harness.run_steps(2);
        harness.get_by_label("Undo rewards").click();
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
        harness.get_by_label("Undo rewards").click();
        harness.run_steps(2);
        assert_eq!(read(), typed);
        assert!(shown(&harness, "That line changed on disk."));
        assert!(harness.query_by_label("Undo rewards").is_none());
        fs::write(&note.path, &no_xp).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.run_steps(2);
        assert!(
            harness.query_by_label("Undo rewards").is_none(),
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
        assert!(harness.query_by_label("Undo rewards").is_some());
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
        assert!(harness.query_by_label("Undo rewards").is_none());

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
        harness.get_by_label("Undo rewards").click();
        harness.run_steps(2);
        assert_eq!(read(), quoted);
        assert!(shown(&harness, "That line changed on disk."));
        fs::write(&note.path, &restarted).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.run_steps(2);

        // A run that cannot be read: nothing is written, and the row says why.
        let before = read();
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

        // Too narrow for the link after Timeline: no line is added, and the action is the last
        // line of the row's menu. With one `S` gap at each `·` (X48), the links fit at 330 px
        // (`the_rewards_fit_keeps_each_title_as_it_is`), so this is a narrower window.
        fs::write(&note.path, &outside).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        harness.set_size(egui::vec2(300.0, 800.0));
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
                saved_at: None,
                unsent: Vec::new(),
            };
            let deleted = crate::encounter_run::DeletedRun::delete(&info).unwrap();
            sessions.record_deleted_run(book, deleted, id);
            let body = sessions
                .draft
                .as_ref()
                .map_or_else(String::new, |draft| draft.body.clone());
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
        book.undo_remove();
        sessions.end_plan_results_after_edits(&book);
        assert!(!sessions.removal_superseded(&book));
        // Removing the same event again is a new step: its result shows (path 1).
        book.remove(&first);
        assert!(
            !sessions.removal_superseded(&book),
            "a new removal of the same event keeps its own result"
        );
        book.undo_remove();
        book.remove(&second);
        assert!(!sessions.removal_superseded(&book));
        // Re-check of PR 245: with a note open, Timeline's Undo remove and a new removal of the
        // same event both happen before the plan draws again. The note and the events then
        // equal the plan's witness, so its cleanup keeps the deletion result; the new removal
        // still shows its message and Undo remove, because it is a different action.
        book.undo_remove();
        let note = folder.path().join("note.md");
        fs::write(&note, "## What happened\n\nThe bridge held.\n").unwrap();
        sessions.draft = Some(Draft::open(&note).unwrap());
        book.remove(&first);
        delete(&mut sessions, &book, Some(run));
        assert!(sessions.removal_superseded(&book));
        book.undo_remove();
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
    }
}
