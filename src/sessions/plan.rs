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
            added_message, file_notices, long_rest_status, problems, quick_add_with, rest_ruling,
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
fn plan_event_summary(ui: &mut egui::Ui, summary: &str, restores: Option<bool>) {
    let mut job = egui::text::LayoutJob::default();
    let format = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: colors().muted,
        ..Default::default()
    };
    job.append(&plan_summary(summary), 0.0, format.clone());
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
        ui.add(egui::Label::new(job).wrap());
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

/// A plan group's summary. Each separator wraps with the words after it, so no line ends with
/// `·`. A flagged group keeps every fact (`max_rows` of `usize::MAX`).
fn plan_group_summary(ui: &mut egui::Ui, summary: &str, max_rows: usize) {
    let mut job = egui::text::LayoutJob::simple(
        summary.replace(" · ", " ·\u{a0}"),
        Type::Small.font(ui),
        colors().muted,
        ui.available_width(),
    );
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
                        ) && line[end..].trim().is_empty()
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

/// A plan row whose start is `lead`: a fight's caret, or a scene's box.
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
        lead(ui);
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
    let c = colors();
    let plain = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: c.accent,
        ..Default::default()
    };
    let mut job = egui::text::LayoutJob::default();
    job.append(icons::RUN, 0.0, plain.clone());
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

/// A planned fight's summary. Its difficulty word (F1) ends it only when the summary keeps its
/// lines with the word.
fn planned_summary(ui: &mut egui::Ui, summary: &str, rating: Option<&crate::difficulty::Rating>) {
    let c = colors();
    let summary = plan_summary(summary);
    let job = |word: Option<&str>| {
        let mut job = egui::text::LayoutJob::default();
        crate::difficulty::append(&mut job, ui, Type::Small, &summary, c.muted);
        if let Some(word) = word {
            crate::difficulty::append(&mut job, ui, Type::Small, " ·\u{a0}", c.muted);
            crate::difficulty::append(
                &mut job,
                ui,
                Type::Small,
                &word.replace(' ', "\u{a0}"),
                c.text,
            );
        }
        job.wrap.max_width = ui.available_width();
        job
    };
    let rows = |job: egui::text::LayoutJob| ui.fonts_mut(|fonts| fonts.layout_job(job).rows.len());
    match rating.filter(|rating| rows(job(Some(rating.word.text()))) == rows(job(None))) {
        Some(rating) => {
            let response = ui.add(egui::Label::new(job(Some(rating.word.text()))).wrap());
            crate::difficulty::describe(ui, response, rating);
        }
        None => wrapped_words(ui, Type::Small, summary, c.muted),
    }
}

/// The opened planned fight's link to its encounter. With its difficulty known, the line ends
/// with ` · Low · XP 550 of 750` (F1), when the line has room for the whole. With no word
/// because an input is not known, it says whose: ` · XP not known: Nameless Thing` (W1-D2).
fn read_encounter(
    ui: &mut egui::Ui,
    difficulty: Option<&crate::difficulty::Difficulty>,
) -> egui::Response {
    // The link keeps its place; the words take the room to its right on the same line.
    let link = design::link(ui, "Read encounter");
    let Some(difficulty) = difficulty else {
        return link;
    };
    let c = colors();
    let room = ui.max_rect().right() - link.rect.right();
    let muted = |words: &str| {
        let mut job = egui::text::LayoutJob::default();
        crate::difficulty::append(&mut job, ui, Type::Body, &format!(" · {words}"), c.muted);
        job
    };
    // The reason whole, else without its names, which hover and the accessible name keep.
    let mut whole = None;
    let job = match (&difficulty.rating, &difficulty.missing) {
        (Some(rating), _) => {
            let mut job = egui::text::LayoutJob::default();
            crate::difficulty::append(&mut job, ui, Type::Body, " · ", c.muted);
            crate::difficulty::append(&mut job, ui, Type::Body, rating.word.text(), c.text);
            crate::difficulty::append(
                &mut job,
                ui,
                Type::Body,
                &format!(" · {}", rating.numbers()),
                c.muted,
            );
            job
        }
        (None, Some(missing)) => {
            let Some(words) = missing.words() else {
                return link;
            };
            let job = muted(&words);
            if crate::difficulty::width(ui, &job) <= room {
                job
            } else {
                whole = Some(words);
                muted(missing.short())
            }
        }
        (None, None) => return link,
    };
    let width = crate::difficulty::width(ui, &job);
    if width <= room {
        // The words start at the link's right edge, so the blank before the first separator
        // is the blank of the job's own leading space (V9). `ui.put` would centre them.
        let rect =
            egui::Rect::from_min_size(link.rect.right_top(), egui::vec2(width, link.rect.height()));
        let response = ui
            .scope_builder(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                |ui| ui.add(egui::Label::new(job).extend()),
            )
            .inner;
        if let Some(rating) = &difficulty.rating {
            crate::difficulty::describe(ui, response, rating);
        } else if let Some(whole) = whole {
            design::named(ui, &response, &whole);
            response.on_hover_text(whole);
        }
    }
    link
}

fn plan_rule(ui: &mut egui::Ui) {
    ui.add_space(design::S);
    design::rule(ui);
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
                            let row = ui.selectable_label(false, *name);
                            design::named(ui, &row, &format!("Plan {name}"));
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
}

struct SceneEntry {
    /// Its kind, status and place, for a World entry.
    detail: Option<String>,
    /// The fight that its `Start fight` starts, for an NPC or a creature.
    start: Option<crate::world::StartFight>,
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
    })
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
                        ModelLine::Scene(Scene { task, entry })
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
}

/// Where a scene act's words show: under the scene's row, under the Scene field, or below the
/// Add strip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    Row(Slot),
    Field,
    Strip,
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

/// What the scenes' fold measured on the last frame. Scenes fold first when the plan's rows do
/// not fit its pane, so that nothing else in the plan moves or narrows because scenes exist.
#[derive(Default)]
struct Fold {
    /// The pane's height, once a frame measured it.
    viewport: Option<f32>,
    /// The rows' height without the scenes, and the scenes' own height when last shown.
    without: f32,
    scenes: f32,
    /// The DM opened (`true`) or closed (`false`) the folded scenes in this session.
    open: Option<bool>,
}

impl Fold {
    /// Whether the scenes fold now, and whether the heading offers `N scenes`.
    fn state(&self, count: usize) -> (bool, bool) {
        if count == 0 {
            return (false, false);
        }
        let overflow = self
            .viewport
            .is_some_and(|viewport| self.without + self.scenes > viewport + 0.5);
        let folded = self.open.map_or(overflow, |open| !open);
        (folded, overflow || self.open.is_some())
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
    /// A scene taken out of tonight: its place among the rows, its words, and its Undo.
    removed: Option<(Slot, String, NoteUndo)>,
    pub(super) fault: Option<Fault>,
    fold: Fold,
    /// The scenes' height drawn in this frame.
    drawn: f32,
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
enum SceneAct {
    Add(String),
    Tick(Slot, TaskLine),
    Remove(Slot, TaskLine),
    UndoAdd,
    UndoRemove,
    StartFight(crate::world::StartFight),
}

/// One scene: its box where a fight has its caret, its words as the Reader draws them, its
/// entry's kind, status and place, and its menu.
fn scene_row(
    ui: &mut egui::Ui,
    width: f32,
    scene: &Scene,
    slot: Slot,
    plan: &Plan<'_>,
    act: &mut Option<SceneAct>,
) {
    let c = colors();
    let task = &scene.task;
    let plain = plain_links(&task.words);
    let widths = [
        width - Density::Read.control() - design::S,
        Density::Read.control(),
    ];
    let mut tick = false;
    let mut remove = false;
    let mut start = None;
    ui.push_id(("scene", task.at, &task.line), |ui| {
        plan_row_with(
            ui,
            &widths,
            |ui| {
                let mut done = task.done;
                let size = egui::vec2(design::XL, design::XL);
                let response = ui.add_sized(size, egui::Checkbox::without_text(&mut done));
                let state = if task.done { "Done" } else { "Open" };
                design::named(ui, &response, &format!("{state}: {plain}"));
                tick = response.clicked();
            },
            |ui| {
                ui.scope(|ui| {
                    if task.done {
                        ui.visuals_mut().override_text_color = Some(c.muted);
                    }
                    crate::markdown_view::show_reader(ui, &task.words);
                });
            },
            |ui| {
                if let Some(detail) = scene
                    .entry
                    .as_ref()
                    .and_then(|entry| entry.detail.as_deref())
                {
                    wrapped_words(ui, Type::Small, detail, c.muted);
                }
            },
            |ui| {
                design::row_menu(ui, &plain, |ui| {
                    if Action::quiet("Remove from tonight").show(ui).clicked() {
                        remove = true;
                        ui.close();
                    }
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
                });
            },
        );
        plan_rule(ui);
    });
    if tick {
        *act = Some(SceneAct::Tick(slot, task.clone()));
    } else if remove {
        *act = Some(SceneAct::Remove(slot, task.clone()));
    } else if let Some(fight) = start {
        *act = Some(SceneAct::StartFight(fight));
    }
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

/// The help line of **How long** when the clock cannot place the journey.
const LENGTH_HELP: &str = "A number and a unit move the clock: 3 days, 8 hours, an hour.";

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
    /// changes, the event, and the minute.
    start: Option<(u64, String, Option<u64>)>,
    focus_length: bool,
    /// Each journey's last Timeline act, by its event. Leaving the page ends them.
    done: checks::Results,
    /// `Plan “<name>”`: the journey, the name, and the note's Undo. It lasts while the note's
    /// words are what the act left, as every note act's Undo does; a page change keeps it.
    planned: Option<(String, String, NoteUndo)>,
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
        let header = checks::View {
            event,
            start,
            length: &event.duration,
            tables: plan.tables,
            planned,
        };
        let mut act = checks::header(ui, &header, &mut travel.block);
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
        };
        act = checks::lines(ui, &view, &mut travel.block).or(act);
        let done = travel.done.get(&event.id);
        let plan_line = travel
            .planned
            .as_ref()
            .filter(|(id, _, _)| *id == event.id)
            .map(|(_, name, _)| name.as_str());
        if let Some(done) = done {
            ui.add_space(design::S);
            if design::undo_message(ui, Tone::Success, &done.words, done.undo) {
                undo = TravelUndo::Checks;
            }
        }
        if let Some(name) = plan_line {
            ui.add_space(design::S);
            if design::undo_message(
                ui,
                Tone::Success,
                &format!("“{name}” is in tonight’s plan."),
                "Undo plan line",
            ) {
                undo = TravelUndo::PlanLine;
            }
        }
        (act, changed, undo)
    }

    /// The open journey's start on the clock, worked out again only when the Timeline changed
    /// or another journey opened.
    fn journey_start(&mut self, book: &Book, open: Option<&str>) -> Option<u64> {
        let open = open?;
        if let Some((changes, id, start)) = &self.travel.start
            && *changes == book.changes()
            && id == open
        {
            return *start;
        }
        let start = book
            .events
            .iter()
            .position(|event| event.id == open && checks::shown(event))
            .and_then(|at| timeline::clock(&book.events)[at]);
        self.travel.start = Some((book.changes(), open.to_owned(), start));
        start
    }

    /// Take back the plan line of `Plan “…”`, while the note is as that act left it.
    fn undo_plan_line(&mut self) {
        let (Some((_, name, undo)), Some(draft)) = (self.travel.planned.take(), &mut self.draft)
        else {
            return;
        };
        // A note that changed since says so where the DM acted.
        if let Err(fault) = undo.undo(draft) {
            self.travel
                .block
                .failed(&checks::Act::Plan(name), fault.words().to_owned());
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
        if let checks::Act::Plan(name) = &act {
            let (Some(draft), Some(note)) = (&mut self.draft, session) else {
                return;
            };
            let line = format!("- [[{name}]]");
            let written = draft.apply(note, Apply::Now, |words| {
                Some(plan_first(words, &line, &book.events, session, plan.fights))
            });
            match written {
                Ok(undo) => self.travel.planned = Some((id.to_owned(), name.clone(), undo)),
                Err(fault) => {
                    let words = fault.words().to_owned();
                    if let crate::sessions::ActFault::NotSaved(_, undo) = fault {
                        self.travel.planned = Some((id.to_owned(), name.clone(), undo));
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

    fn removed_group_message(
        &self,
        ui: &mut egui::Ui,
        session: Option<Uuid>,
        position: usize,
    ) -> bool {
        let Some((id, at, _, _, height, name)) = &self.removed_group else {
            return false;
        };
        if *id != session || *at != position {
            return false;
        }
        ui.allocate_ui(egui::vec2(ui.available_width(), *height), |ui| {
            ui.set_min_height(*height);
            design::undo_message(
                ui,
                Tone::Success,
                &format!("Removed “{name}” from tonight."),
                "Undo remove",
            )
        })
        .inner
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
        file_notices(ui, book);
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
        let mut scenes: Vec<Vec<&Scene>> = vec![Vec::new()];
        for line in &model.lines {
            match line {
                ModelLine::Scene(scene) => scenes
                    .last_mut()
                    .expect("one list of scenes for each place")
                    .push(scene),
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
        let open_scenes = scenes
            .iter()
            .flatten()
            .filter(|scene| !scene.task.done)
            .count();
        let scene_count = scenes.iter().map(Vec::len).sum::<usize>();
        let (folded, fold_line) = self.scenes.fold.state(scene_count);
        self.scenes.drawn = 0.0;
        let mut scene_act = None;
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
        if plan.group_open && self.not_being_played().is_none() {
            let label = self
                .notes
                .iter()
                .find(|note| Some(note.id) == self.group_session)
                .map_or_else(
                    || "another session".to_owned(),
                    |note| format!("“{}”", note.label()),
                );
            if design::message_with_action(
                ui,
                Tone::Info,
                &format!("A group is being assembled in the plan of {label}."),
                "Go to it",
            ) {
                self.select(self.group_session);
            }
        }
        if plan.group_discarded && here && design::draft_discarded(ui, &plan.group_discarded_name) {
            self.wants_group_undo = true;
        }
        if let Some(problem) = &self.rename_problem {
            design::message(ui, Tone::Warning, problem);
        }
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
        self.plan_messages(ui, book, session, plan, &planned_results);
        self.scene_messages(ui, folded, &mut scene_act);
        if let Some(id) = &added {
            if let Some(event) = book.events.iter_mut().find(|event| event.id == *id) {
                event.title = event.kind.word().to_owned();
            }
            let days = timeline::day_numbers(&book.events);
            let index = book.events.iter().position(|event| event.id == *id);
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
        let mut undo_group_remove = false;
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
        checks::settle_length(book, open_event.as_deref());
        let journey_start = self.journey_start(book, open_event.as_deref());
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
        egui::CentralPanel::default().frame(egui::Frame::new()).show_inside(ui, |ui| {
            // The rows leave the scroll bar's lane free whether or not the bar shows, so a row's
            // actions keep their x when the plan grows long enough to scroll.
            let lane_width = ui.available_width() - ui.spacing().scroll.allocated_width();
            let output = egui::ScrollArea::vertical().id_salt(("session_plan", session))
                .auto_shrink([false, false]).show(ui, |ui| {
                ui.set_max_width(lane_width);
                let width = lane_width.min(ui.available_width()) - design::M;
                let action_width = [Action::primary("Start fight").icon(icons::RUN), Action::primary("Resume").icon(icons::RUN), Action::quiet("Add to notes"), Action::secondary("Arrivals").icon(icons::GROUP)]
                    .into_iter().map(|action| action.density(Density::Read).natural_width(ui)).fold(0.0, f32::max)
                    + Density::Read.control() + design::S;
                let widths = [width - action_width - design::S, action_width];
                let mut any = false;
                let mut primary_available = plan.open_run.is_none() && !self.confirm_record;
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
                        let row_widths = if event.run.is_some() && (running || saved.is_none()) {
                            [width - Density::Read.control() - design::S, Density::Read.control()]
                        } else { widths };
                        plan_row(ui, &row_widths, if is_open { icons::DOWN } else { icons::NEXT }, |ui| {
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
                                plan_event_summary(ui, &summary, (event.kind == Kind::LongRest).then_some(event.restores));
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
                                });
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
                        if is_open {
                            plan_details(ui, |ui| {
                            if running {
                                self.fight = None;
                                wrapped_words(ui, Type::Small, "This fight is running. Its final HP shows here when it ends.", c.muted);
                            } else if let Some(run) = event.run {
                                self.fight_block(ui, run, &event.id, plan);
                            } else {
                                changed |= event_fields(ui, event);
                                if checks::shown(event) {
                                    let (act, taken, undo) = self.travel_block(ui, event, journey_start, plan, &in_plan, &mut book.length);
                                    if taken != checks::Taken::Nothing { length_taken = taken; }
                                    if let Some(act) = act { travel_act = Some((event.id.clone(), act)); }
                                    if undo != TravelUndo::None { travel_undo = (undo, event.id.clone()); }
                                }
                                if let Some(action) = rest_ruling(ui, event, ruling_undo.as_ref() == Some(&event.id)) {
                                    ruling = Some((event.id.clone(), action));
                                }
                                if design::link(ui, "Timeline").clicked() { self.wants_event = Some(event.id.clone()); }
                            }
                            });
                        }
                        if self.row_notice == Some((session, at)) && self.deleted_run.is_some() {
                            ui.add_space(design::S);
                            self.deleted_run_message(ui);
                        }
                        plan_rule(ui);
                    });
                    // The opened row and the next row stay in reach.
                    if is_open && std::mem::take(&mut self.reveal_event) { row.response.scroll_to_me(Some(egui::Align::TOP)); }
                }
                if let Some((at, words)) = &removed_row && *at >= event_count {
                    ui.add_space(design::S);
                    undo_row_remove |= design::undo_message(ui, Tone::Success, words, "Undo remove");
                }
                for (position, fight) in planned.iter().enumerate() {
                    self.removed_fight_message(ui, session, position);
                    if !folded {
                        self.scene_rows(ui, width, &scenes[position], position, plan, &mut scene_act);
                    }
                    any = true;
                    let running = plan.open_run.is_some() && plan.open_encounter == Some(fight.name.as_str());
                    let saved = planned_saved_run(fight, plan.fights, plan.saved_runs);
                    ui.push_id((&fight.path, position), |ui| {
                        let row_widths = if running {
                            [width - Density::Read.control() - design::S, Density::Read.control()]
                        } else { widths };
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
                            } else { planned_summary(ui, &fight.summary, fight.difficulty.rating.as_ref()); }
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
                            });
                            if running {
                                // The run strip owns Back to run.
                            } else if let Some(saved) = saved {
                                let action = if std::mem::take(&mut primary_available) { Action::primary("Resume") } else { Action::secondary("Resume") };
                                if action.icon(icons::RUN).name(&format!("Resume: {}", fight.name)).density(Density::Read).blocked(plan.blocked).show(ui).clicked() {
                                    self.wants_saved_run = Some(saved.path.clone());
                                }
                            } else {
                                let action = if std::mem::take(&mut primary_available) {
                                    Action::primary("Start fight")
                                } else { Action::secondary("Start fight") };
                                if action.icon(icons::RUN).name(&format!("Start fight: {}", fight.name))
                                    .density(Density::Read).blocked(plan.blocked).show(ui).clicked() {
                                    self.wants_fight = Some(fight.path.clone());
                                }
                            }
                        });
                        if self.row_notice == Some((session, event_count + position)) && self.deleted_run.is_some() {
                            ui.add_space(design::S);
                            self.deleted_run_message(ui);
                        }
                        if self.open_planned.as_ref() == Some(&fight.path) {
                            plan_details(ui, |ui| {
                                if read_encounter(ui, Some(&fight.difficulty).filter(|_| !running && saved.is_none())).clicked() { self.wants_lookup = Some(fight.name.clone()); }
                            });
                        }
                        plan_rule(ui);
                    });
                }
                self.removed_fight_message(ui, session, planned.len());
                if !folded {
                    self.scene_rows(ui, width, &scenes[planned.len()], planned.len(), plan, &mut scene_act);
                }
                any |= scenes.iter().any(|scenes| !scenes.is_empty());
                for (position, group) in planned_groups.iter().enumerate() {
                    undo_group_remove |= self.removed_group_message(ui, session, position);
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
                        if read_group { self.wants_group = Some(group.path.clone()); self.group_session = self.selected; }
                        plan_rule(ui);
                    });
                    if let Some((at, _, height)) = &mut remove_group && *at == position { *height = row.response.rect.height(); }
                }
                undo_group_remove |= self.removed_group_message(ui, session, planned_groups.len());
                if !any && self.removed_group.is_none() {
                    design::empty_state(ui, icons::SESSIONS, "Nothing planned yet",
                        "Use Add to plan a fight, a group, or an event.", |_| {});
                }
                self.scene_field(ui, index.as_deref(), &mut scene_act);
            });
            pane = Some((output.inner_rect.height(), output.content_size.y));
        });
        if let Some((viewport, content)) = pane {
            self.measure_fold(ui.ctx(), scene_count, folded, viewport, content);
        }
        if let Some(target) = crate::markdown_view::take_note_link(ui.ctx()) {
            self.open_note_link(target);
        }
        if let Some(act) = scene_act {
            self.scene_act(act, session);
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
            self.removed_group =
                Some((session, position, before, draft.body.clone(), height, name));
            self.plan_notice_state = Some((draft.body.clone(), book.events.clone()));
        }
        if changed {
            book.touch();
        }
        checks::note_taken(book, length_taken);
        if let Some((id, act)) = travel_act {
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

    /// The scenes at one place among the plan's rows, each with its removal's Undo in its place.
    fn scene_rows(
        &mut self,
        ui: &mut egui::Ui,
        width: f32,
        scenes: &[&Scene],
        position: usize,
        plan: &Plan<'_>,
        act: &mut Option<SceneAct>,
    ) {
        let top = ui.cursor().top();
        for (at, scene) in scenes.iter().enumerate() {
            self.removed_scene_message(ui, (position, at), act);
            scene_row(ui, width, scene, (position, at), plan, act);
            self.row_fault(ui, Place::Row((position, at)));
        }
        self.removed_scene_message(ui, (position, scenes.len()), act);
        self.scenes.drawn += ui.cursor().top() - top;
    }

    /// The scene act's words under the place where the DM acted.
    fn row_fault(&self, ui: &mut egui::Ui, place: Place) {
        if let Some(fault) = &self.scenes.fault
            && fault.place == place
        {
            ui.add_space(design::XS);
            design::message(ui, Tone::Warning, &fault.words);
        }
    }

    /// `N scenes` on the heading's line, in room that is spare there, so it moves no row: a link
    /// with the expanded state, which opens the folded scenes in place and closes them again.
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
        if plan_link(&mut line, &words, !folded).clicked() {
            self.scenes.fold.open = Some(folded);
        }
    }

    /// What the plan's pane and rows measured in this frame, for the next frame's fold.
    fn measure_fold(
        &mut self,
        ctx: &egui::Context,
        count: usize,
        folded: bool,
        viewport: f32,
        content: f32,
    ) {
        let fold = &mut self.scenes.fold;
        fold.viewport = Some(viewport);
        if folded {
            fold.without = content;
        } else {
            fold.scenes = self.scenes.drawn;
            fold.without = content - self.scenes.drawn;
        }
        // The fold settles on the next frame without waiting for input.
        if fold.state(count).0 != folded {
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
            cache.model = Some(Arc::new(PlanModel::build(
                body,
                plan.fights,
                plan.groups,
                index,
            )));
            cache.key = Some(key);
        }
        Arc::clone(cache.model.as_ref().expect("the model was just built"))
    }

    fn removed_scene_message(&self, ui: &mut egui::Ui, slot: Slot, act: &mut Option<SceneAct>) {
        let Some((at, words, undo)) = &self.scenes.removed else {
            return;
        };
        if *at != slot || self.draft.as_ref().is_none_or(|draft| !undo.live(draft)) {
            return;
        }
        ui.add_space(design::S);
        if design::undo_message(
            ui,
            Tone::Info,
            &format!("Removed “{words}” from tonight."),
            "Undo remove",
        ) {
            *act = Some(SceneAct::UndoRemove);
        }
    }

    /// The scenes' results in the plan's message place, below the Add strip.
    fn scene_messages(&mut self, ui: &mut egui::Ui, folded: bool, act: &mut Option<SceneAct>) {
        // A row's words show under the row; a folded row's show here.
        if let Some(fault) = &self.scenes.fault
            && (fault.place == Place::Strip || folded && matches!(fault.place, Place::Row(_)))
        {
            design::message(ui, Tone::Warning, &fault.words);
        }
        if let Some(undo) = &self.scenes.added
            && self.draft.as_ref().is_some_and(|draft| undo.live(draft))
            && design::undo_message(ui, Tone::Success, "Scene added to tonight.", "Undo scene")
        {
            *act = Some(SceneAct::UndoAdd);
        }
    }

    /// `Scene`'s field at the end of the plan. Enter adds a scene and keeps the field open for
    /// the next; Escape clears and closes it.
    fn scene_field(
        &mut self,
        ui: &mut egui::Ui,
        index: Option<&ContentIndex>,
        act: &mut Option<SceneAct>,
    ) {
        let Some(state) = &mut self.scenes.field else {
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
        if reveal {
            result.field.scroll_to_me(None);
        }
        let typed = state.words.trim().to_owned();
        let emptied = state.words.is_empty();
        self.row_fault(ui, Place::Field);
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

    /// One scene act, through the note's one act path: it follows the disk, changes the one line
    /// that the row was drawn from and saves at once. What it could not do shows where the DM
    /// acted. An act whose save failed keeps its Undo: the fragment waits in the draft.
    fn scene_act(&mut self, act: SceneAct, session: Option<Uuid>) {
        if let SceneAct::StartFight(fight) = act {
            self.wants_start_fight = Some(fight);
            return;
        }
        let (Some(draft), Some(note)) = (&mut self.draft, session) else {
            return;
        };
        let scenes = &mut self.scenes;
        scenes.fault = None;
        let (result, place) = match act {
            SceneAct::StartFight(_) => return,
            SceneAct::Add(words) => {
                let result = draft.apply(note, Apply::Now, |text| Some(add_scene(text, &words)));
                if let Ok(undo) | Err(ActFault::NotSaved(_, undo)) = &result {
                    scenes.added = Some(undo.clone());
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
            SceneAct::Remove(slot, task) => {
                let result = draft.apply(note, Apply::Now, |text| {
                    change_line(text, task.at, &task.line, None)
                });
                if let Ok(undo) | Err(ActFault::NotSaved(_, undo)) = &result {
                    scenes.removed = Some((slot, plain_links(&task.words), undo.clone()));
                }
                (result.map(drop), Place::Row(slot))
            }
            SceneAct::UndoAdd => (
                scenes.added.take().map_or(Ok(()), |undo| undo.undo(draft)),
                Place::Strip,
            ),
            SceneAct::UndoRemove => match scenes.removed.take() {
                Some((slot, _, undo)) => (undo.undo(draft), Place::Row(slot)),
                None => (Ok(()), Place::Strip),
            },
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

    fn deleted_run_message(&mut self, ui: &mut egui::Ui) {
        if let Some((deleted, _)) = &self.deleted_run {
            let words = deleted.message();
            if design::undo_message(
                ui,
                if deleted.unsent.is_empty() {
                    Tone::Success
                } else {
                    Tone::Warning
                },
                &words,
                crate::encounter_run::DeletedRun::UNDO_LABEL,
            ) {
                self.undo_deleted_run();
            }
        }
    }

    /// What can be undone in the plan: the event just added or just removed.
    fn plan_messages(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        session: Option<Uuid>,
        plan: &Plan<'_>,
        planned: &[(String, bool)],
    ) {
        if self.row_notice.is_none() {
            self.deleted_run_message(ui);
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
            if design::undo_message(ui, Tone::Success, &words, "Undo add") {
                book.undo_add(&id);
                self.added = None;
            }
        } else if !self.removed_event_in_plan
            && !self.removal_superseded(book)
            && let Some(label) = book.removed().map(str::to_owned)
        {
            let words = removed_event_words(book, session, &days, &label, planned);
            if design::undo_message(ui, Tone::Success, &words, "Undo remove") {
                book.undo_remove();
            }
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

    /// How a fight ended, in place: final HP and the log, with no run to reopen.
    fn fight_block(&mut self, ui: &mut egui::Ui, run: Uuid, event_id: &str, plan: &Plan<'_>) {
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
            return;
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
                        detail.push_str(&format!(" · {condition}"));
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
                    // than the word, never padded with spaces.
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::Draft;
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
            name: name.into(),
            path: path.into(),
            summary: String::new(),
            difficulty: Default::default(),
        }
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
                plan_event_summary(ui, "Old mill", Some(false));
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
                    .scope(|ui| plan_event_summary(ui, "Long rest · Old mill", None))
                    .response
                    .rect
                    .height();
                let restored = ui
                    .scope(|ui| plan_event_summary(ui, "Long rest · Old mill", Some(true)))
                    .response
                    .rect
                    .height();
                assert_eq!(plain, restored);
            });
        });
    }

    /// V9 and W1-D2: the opened planned fight's line has the same blank on each side of every
    /// separator, with the word and its numbers, and with the reason when no word shows.
    #[test]
    fn read_encounter_has_one_blank_each_side_of_each_separator() {
        use crate::difficulty::{Difficulty, Missing, Unknown};
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let rated = Difficulty {
            xp: 6300,
            rating: Some(crate::difficulty::above_high()),
            missing: None,
        };
        let unknown = Difficulty {
            xp: 0,
            rating: None,
            missing: Some(Missing {
                what: Unknown::Xp,
                at: vec![0, 1],
                names: vec!["Nameless Thing".into(), "Wisp".into()],
            }),
        };
        for (difficulty, words) in [
            (rated, "Read encounter · Above High · XP 6,300"),
            (
                unknown,
                "Read encounter · XP not known: Nameless Thing and 1 more",
            ),
        ] {
            // The second frame has the app's fonts.
            let frame = || {
                ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.set_width(design::LIST_MEASURE);
                        plan_details(ui, |ui| {
                            read_encounter(ui, Some(&difficulty));
                        });
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
    fn the_difficulty_word_joins_a_summary_only_when_the_summary_keeps_its_lines() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let rating = crate::difficulty::above_high();
        let summary = "5 party · Cinder Tyrant · 2 × Ember Cultist";
        let widths: Vec<f32> = (120..=360).map(|width| width as f32).collect();
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for width in &widths {
                    let height = |ui: &mut egui::Ui, rating| {
                        ui.scope(|ui| {
                            ui.set_width(*width);
                            planned_summary(ui, summary, rating);
                        })
                        .response
                        .rect
                        .height()
                    };
                    let plain = height(ui, None);
                    assert_eq!(height(ui, Some(&rating)), plain, "at {width} px");
                }
            });
        });
        let shown = output
            .shapes
            .iter()
            .filter(|shape| match &shape.shape {
                egui::Shape::Text(text) => text.galley.job.text.ends_with("Above\u{a0}High"),
                _ => false,
            })
            .count();
        // The word shows where it fits, and gives way where it would add a line.
        assert!(
            shown > 0 && shown < widths.len(),
            "{shown} of {}",
            widths.len()
        );
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
        assert!(harness.query_by_label("Scene added to tonight.").is_some());
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
