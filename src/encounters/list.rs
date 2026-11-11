//! The encounter ledger and its local editor controls.
use super::*;
use crate::encounter_run::{FightSummary, SavedRunInfo};
use std::path::PathBuf;

pub(super) fn blocked_reason(ui: &mut egui::Ui, reason: Option<&str>) {
    if let Some(reason) = reason {
        ui.add(egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted)).wrap());
    }
}

pub(super) fn row_menu(ui: &mut egui::Ui, name: &str, add: impl FnOnce(&mut egui::Ui)) {
    let name = format!("More actions for {name}");
    // The pane the menu opens from: the menu stays inside it, as its submenus do.
    let inside = ui.clip_rect();
    let button = Action::quiet(icons::EXPAND)
        .density(Density::Read)
        .name(&name)
        .hint(&name)
        .show(ui);
    let popup = egui::Popup::menu(&button);
    // When the pane changes size under an open menu (the window shrinks), the pane brings the
    // menu's button back into view, so the menu does not hang from a button the DM cannot see.
    // Only a resize does this: the DM's own scrolling stays where the DM leaves it.
    let pane = popup.get_id().with("pane size");
    if egui::Popup::is_id_open(ui.ctx(), popup.get_id()) {
        let last = ui.data_mut(|data| data.get_temp::<egui::Vec2>(pane));
        let resized = last.is_some_and(|last| last != inside.size());
        let pending = pane.with("bring back");
        let wheel = ui.input(|input| input.raw_scroll_delta != egui::Vec2::ZERO);
        let bring_back = !wheel
            && (resized
                || ui
                    .data_mut(|data| data.get_temp::<bool>(pending))
                    .unwrap_or(false));
        if bring_back && !inside.contains_rect(button.rect) {
            // The layout can settle a frame after the resize: keep asking until the button shows.
            ui.data_mut(|data| data.insert_temp(pending, true));
            ui.scroll_to_rect(button.rect, None);
        } else {
            ui.data_mut(|data| data.remove::<bool>(pending));
        }
        ui.data_mut(|data| data.insert_temp(pane, inside.size()));
    } else {
        ui.data_mut(|data| data.remove::<egui::Vec2>(pane));
    }
    let size = egui::AreaState::load(ui.ctx(), popup.get_id()).and_then(|state| state.size);
    let aligns = menu_aligns(button.interact_rect, size, inside);
    popup
        .align(aligns[0])
        .align_alternatives(&aligns[1..])
        .show(add);
}

/// Where a row menu may open: below or above its button, ending at the button's right edge, or
/// starting at its left edge when only that keeps the menu `S` inside the pane. egui picks the
/// first of these that fits the window, and the first one when none does, so the menu never
/// opens beside its button, over a splitter or scroll bar.
fn menu_aligns(
    button: egui::Rect,
    size: Option<egui::Vec2>,
    inside: egui::Rect,
) -> [egui::RectAlign; 2] {
    use egui::RectAlign;
    let start_only = size.is_some_and(|size| {
        button.right() - size.x < inside.left() + design::S
            && button.left() + size.x <= inside.right() - design::S
    });
    if start_only {
        [RectAlign::BOTTOM_START, RectAlign::TOP_START]
    } else {
        [RectAlign::BOTTOM_END, RectAlign::TOP_END]
    }
}

fn menu_icon(word: &str) -> Option<&'static str> {
    match word {
        "Move up" => Some(icons::MOVE_UP),
        "Move down" => Some(icons::MOVE_DOWN),
        "Remove" | "Delete the saved run" => Some(icons::REMOVE),
        _ => None,
    }
}

/// A line under a menu choice: why it is blocked, or what it does. It starts at the choice's
/// words, after the quiet action's padding and any icon, and it ends the quiet action's padding
/// before the menu's edge. The menu asks for its width with both, so the note keeps the menu's
/// right margin and wraps before it.
pub(super) fn choice_note(ui: &mut egui::Ui, word: &str, note: Option<&str>) {
    if note.is_none() {
        return;
    }
    let inset = design::S + menu_icon(word).map_or(0.0, |_| design::ICON_SLOT);
    ui.horizontal(|ui| {
        ui.add_space(inset);
        let gap = ui.spacing().item_spacing.x;
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width() - design::S);
            blocked_reason(ui, note);
        });
        // The layout adds its item gap after the note; together they make the padding `S`.
        ui.add_space(design::S - gap);
    });
}

pub(super) fn menu_action(ui: &mut egui::Ui, word: &str, blocked: Option<&str>) -> bool {
    let action = match menu_icon(word) {
        Some(icon) => Action::quiet(word).icon(icon),
        None => Action::quiet(word),
    };
    let pressed = action
        .density(Density::Read)
        .blocked(blocked)
        .show(ui)
        .clicked();
    choice_note(ui, word, blocked);
    if pressed {
        ui.close();
    }
    pressed
}

/// The same named side picker serves roster slots and groups. Other opens a text field.
pub(super) fn side_control(ui: &mut egui::Ui, name: &str, side: &mut String, choices: &[String]) {
    let id = ui.make_persistent_id((name, "other_side"));
    let custom = !matches!(side.as_str(), "party" | "enemies")
        && !choices.iter().any(|choice| choice == side);
    let mut other = ui.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
    ui.horizontal(|ui| {
        if other {
            let mut text = if custom {
                design::reader::human_label(side)
            } else {
                String::new()
            };
            if design::text_input(
                ui,
                &format!("Other side for {name}"),
                &mut text,
                "Side name",
                design::S * 10.0,
            )
            .changed()
                && !text.trim().is_empty()
            {
                *side = text.trim().to_lowercase();
            }
        }
        let combo = egui::ComboBox::from_id_salt(name)
            .icon(design::combo_icon)
            .width(if other {
                Density::Read.control()
            } else {
                design::S * 13.0
            })
            .selected_text(if other {
                String::new()
            } else {
                design::reader::human_label(side)
            })
            .show_ui(ui, |ui| {
                for choice in ["party", "enemies"].into_iter().chain(
                    choices
                        .iter()
                        .map(String::as_str)
                        .filter(|choice| !matches!(*choice, "party" | "enemies")),
                ) {
                    if ui
                        .selectable_label(
                            side == choice && !other,
                            design::reader::human_label(choice),
                        )
                        .clicked()
                    {
                        *side = choice.to_owned();
                        other = false;
                    }
                }
                if ui.selectable_label(other, "Other…").clicked() {
                    other = true;
                }
            });
        design::named(ui, &combo.response, name);
    });
    ui.data_mut(|data| data.insert_temp(id, other));
}

fn heading(ui: &mut egui::Ui, word: &str, count: usize) {
    design::group_heading(ui, word, count);
}

fn section(
    ui: &mut egui::Ui,
    id: &str,
    word: &str,
    count: usize,
    open: Option<bool>,
    add: impl FnOnce(&mut egui::Ui),
) {
    if open.is_none() {
        design::section_with_id(ui, id, design::group_heading_text(ui, word, count), add);
    } else {
        egui::CollapsingHeader::new(design::group_heading_text(ui, word, count))
            .id_salt(id)
            .icon(design::section_icon)
            .open(open)
            .show(ui, add);
    }
}

pub(super) fn list_row(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    name: &str,
    detail: &str,
    status: bool,
    rating: Option<&crate::difficulty::Rating>,
    actions: impl FnOnce(&mut egui::Ui),
) {
    ui.push_id(id, |ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), Density::Scan.row() + design::S),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                ui.set_min_height(Density::Scan.row() + design::S);
                ui.scope(actions);
                ui.add_space(design::L);
                let room = ui.available_width().max(0.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(room, Type::Strong.line() + Type::Small.line()),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.set_width(room);
                        ui.add(
                            egui::Label::new(Type::Strong.rich(ui, name).color(colors().text))
                                .truncate(),
                        )
                        .on_hover_text(name);
                        let room = ui.available_width() - if status { design::L } else { 0.0 };
                        let visible_detail = fit_detail(ui, detail, &Type::Small.font(ui), room);
                        // F1: the word ends the line only when the detail stays whole with it.
                        let rating = rating.filter(|rating| {
                            visible_detail == detail
                                && design::number_width(
                                    ui,
                                    &format!("{detail} · {}", rating.word.text()),
                                    &Type::Small.font(ui),
                                ) <= room
                        });
                        let warning = visible_detail.split(" · ").position(|part| {
                            matches!(part, "Changed on disk" | "Offered in no fight")
                        });
                        let mut job = egui::text::LayoutJob::default();
                        for (index, part) in visible_detail.split(" · ").enumerate() {
                            if index > 0 {
                                job.append(
                                    " · ",
                                    0.0,
                                    egui::TextFormat {
                                        font_id: Type::Small.font(ui),
                                        color: colors().muted,
                                        ..Default::default()
                                    },
                                );
                            }
                            let words = if warning == Some(index) {
                                format!("{} {part}", icons::WARN)
                            } else if index == 0 && status {
                                format!("{} {part}", icons::RUN)
                            } else {
                                part.to_owned()
                            };
                            job.append(
                                &words,
                                0.0,
                                egui::TextFormat {
                                    font_id: Type::Small.font(ui),
                                    color: if warning == Some(index) {
                                        colors().result.warning
                                    } else {
                                        colors().muted
                                    },
                                    ..Default::default()
                                },
                            );
                        }
                        let Some(rating) = rating else {
                            let response =
                                ui.add(egui::Label::new(job).extend()).on_hover_text(detail);
                            design::named(ui, &response, &format!("{name} · {detail}"));
                            return;
                        };
                        let word = rating.word.text();
                        crate::difficulty::append(&mut job, ui, Type::Small, " · ", colors().muted);
                        crate::difficulty::append(&mut job, ui, Type::Small, word, colors().text);
                        let response = ui.add(egui::Label::new(job).extend());
                        design::named(ui, &response, &format!("{name} · {detail} · {word}"));
                        crate::difficulty::describe(ui, response, rating);
                    },
                );
            },
        );
    });
}

/// A row's second line in `width`. A paused run's last change gives up its words first, by whole
/// words, so that the save time after it stays whole. With no room for one of its words, the last
/// change goes. Hover keeps the full line.
fn fit_detail(ui: &egui::Ui, detail: &str, font: &egui::FontId, width: f32) -> String {
    let parts = detail.split(" · ").collect::<Vec<_>>();
    let last = parts
        .iter()
        .position(|part| part.starts_with(crate::encounter_run::LAST_CHANGE));
    let Some(at) = last.filter(|_| design::number_width(ui, detail, font) > width) else {
        return design::words_that_fit(ui, detail, font, width);
    };
    let mut others = parts.clone();
    others.remove(at);
    let room = width - design::number_width(ui, &format!("{} · ", others.join(" · ")), font);
    let change = design::words_that_fit(ui, parts[at], font, room);
    let has_words = change.trim_end_matches('…').trim() != crate::encounter_run::LAST_CHANGE.trim();
    if has_words && design::number_width(ui, &change, font) <= room {
        others.insert(at, &change);
    }
    design::words_that_fit(ui, &others.join(" · "), font, width)
}

/// Read the state from old and current run labels without exposing round zero.
pub(crate) fn run_state_word(run: &SavedRunInfo, paused: bool) -> String {
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

fn row_menu_for_run(
    ui: &mut egui::Ui,
    run: &SavedRunInfo,
    template: Option<&Template>,
    open: bool,
    blocked: Option<&str>,
    loading: bool,
    action: &mut Option<ListAction>,
) {
    row_menu(ui, &run.encounter, |ui| {
        if let Some(template) = template
            && menu_action(ui, "Start another fight", blocked)
        {
            *action = Some(ListAction::Run(template.path.clone()));
        }
        if !open
            && menu_action(
                ui,
                "Delete the saved run",
                loading.then_some("Still reading this campaign."),
            )
        {
            *action = Some(ListAction::Delete(run.path.clone()));
        }
    });
}

/// A duplicate name uses its session first, and has only one save time.
fn paused_run_detail(
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
                    crate::encounter_run::calendar_time(at)
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

pub(super) fn in_progress_list(
    ui: &mut egui::Ui,
    page: &history::Arranged<'_>,
    templates: &[Template],
    live: Option<&SavedRunInfo>,
    availability: (Option<&'static str>, bool),
    story: &Story<'_>,
    action: &mut Option<ListAction>,
) {
    let (blocked, loading) = availability;
    heading(ui, "IN PROGRESS", page.in_progress.len());
    if page.in_progress.is_empty() {
        blocked_reason(
            ui,
            Some("No unfinished runs. Start a fight from Ready to run."),
        );
    }
    for (index, run) in page.in_progress.iter().enumerate() {
        let open = live.is_some_and(|live| live.id == run.id);
        let template = history::template_of(run, templates);
        let detail = paused_run_detail(run, &page.in_progress, open, story);
        list_row(ui, &run.path, &run.encounter, &detail, !open, None, |ui| {
            row_menu_for_run(ui, run, template, open, blocked, loading, action);
            let edit_width = design::number_width(ui, "Edit", &Type::Strong.font(ui)) + design::L;
            ui.allocate_ui_with_layout(
                egui::vec2(edit_width, Density::Read.control()),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    ui.set_min_width(edit_width);
                    if let Some(template) = template
                        && Action::quiet("Edit")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                    {
                        *action = Some(ListAction::Edit(template.path.clone()));
                    }
                },
            );
            if open {
                let word = run_state_word(run, false);
                if design::status_link(
                    ui,
                    Status {
                        icon: icons::RUN,
                        word: &word,
                        color: colors().muted,
                    },
                )
                .clicked()
                {
                    *action = Some(ListAction::BackToRun);
                }
            } else {
                let button = if index == 0 && live.is_none() {
                    Action::primary("Resume")
                } else {
                    Action::secondary("Resume")
                };
                if button
                    .icon(icons::RUN)
                    .density(Density::Read)
                    .blocked(blocked)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(ListAction::Resume(run.path.clone()));
                }
            }
        });
    }
    ui.add_space(design::L);
}

pub(super) fn ready_list(
    ui: &mut egui::Ui,
    page: &history::Arranged<'_>,
    live: Option<&SavedRunInfo>,
    blocked: Option<&'static str>,
    action: &mut Option<ListAction>,
) {
    heading(ui, "READY TO RUN", page.ready.len());
    blocked_reason(ui, blocked);
    if page.ready.is_empty() {
        blocked_reason(ui, Some("None ready. Select New encounter."));
    }
    for (index, row) in page.ready.iter().enumerate() {
        let template = row.template;
        let unfinished = page.in_progress.iter().find(|run| {
            run.source == Some(template.id)
                || (run.source.is_none() && run.encounter == template.name)
        });
        let open = unfinished.is_some_and(|run| live.is_some_and(|live| live.id == run.id));
        let short_reason = if open {
            Some("Go back to the run".to_owned())
        } else if row.unfinished_run {
            Some("Resume it in In progress".to_owned())
        } else if template.summary.as_deref().is_none_or(str::is_empty) {
            Some("Add participants first".to_owned())
        } else {
            None
        };
        let start_blocked = if row.unfinished_run {
            Some(if open {
                "This encounter has an open run. Go back to the run."
            } else {
                "This encounter has an unfinished run. Resume it in In progress."
            })
        } else if template.summary.as_deref().is_none_or(str::is_empty) {
            Some("Add participants first")
        } else {
            blocked
        };
        list_row(
            ui,
            &template.path,
            &template.name,
            template
                .summary
                .as_deref()
                .filter(|s| !s.is_empty())
                .unwrap_or("No participants yet"),
            false,
            // A fight with an unfinished run is not ready, so it has no word (F1).
            template
                .difficulty
                .rating
                .as_ref()
                .filter(|_| !row.unfinished_run),
            |ui| {
                row_menu(ui, &template.name, |ui| {
                    if menu_action(ui, "Edit encounter", None) {
                        *action = Some(ListAction::Edit(template.path.clone()));
                    }
                });
                if Action::quiet("Edit")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(ListAction::Edit(template.path.clone()));
                }
                let button = if index == 0 && page.in_progress.is_empty() && live.is_none() {
                    Action::primary(START_FIGHT)
                } else {
                    Action::secondary(START_FIGHT)
                };
                if button
                    .icon(icons::RUN)
                    .density(Density::Read)
                    .blocked(start_blocked)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(ListAction::Run(template.path.clone()));
                }
                if let Some(reason) = &short_reason {
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted))
                            .extend(),
                    );
                    ui.add_space(design::L);
                }
            },
        );
    }
}

pub(super) fn group_list(
    ui: &mut egui::Ui,
    snapshot: &Snapshot,
    blocked: Option<&'static str>,
    action: &mut Option<ListAction>,
) {
    if snapshot.groups.is_empty() {
        heading(ui, "GROUPS", 0);
        blocked_reason(ui, Some("No groups yet. Select New group."));
    } else {
        section(
            ui,
            "encounter_groups",
            "GROUPS",
            snapshot.groups.len(),
            None,
            |ui| {
                groups::list(ui, snapshot, blocked, action);
            },
        );
    }
}

pub(super) fn history_list(
    ui: &mut egui::Ui,
    page: &history::Arranged<'_>,
    blocked: Option<&'static str>,
    live: Option<&SavedRunInfo>,
    read: Option<&(PathBuf, FightSummary)>,
    not_last: Option<&'static str>,
    action: &mut Option<ListAction>,
) {
    ui.add_space(design::L);
    if page.ended_count() == 0 && page.unreadable.is_empty() {
        heading(ui, "HISTORY", 0);
        blocked_reason(ui, Some("No ended fights yet."));
        return;
    }
    section(
        ui,
        "encounter_history",
        "HISTORY",
        page.ended_count(),
        read.map(|_| true),
        |ui| {
            for session in &page.history {
                design::group_heading(ui, session.label, session.runs.len());
                for past in &session.runs {
                    let run = past.run;
                    let date = run
                        .saved_at
                        .map(|at| crate::encounter_run::saved_time(at, crate::hermetic::now()))
                        .unwrap_or_else(|| "Date unknown".to_owned());
                    let state = run_state_word(run, false);
                    let outcome = run
                        .label
                        .split(" · Ended")
                        .next()
                        .filter(|_| !run.label.starts_with("Ended"));
                    let detail = format!(
                        "{}{} · {date}{}",
                        outcome.map_or_else(String::new, |outcome| format!("{outcome} · ")),
                        state,
                        if past.template.is_none() {
                            " · Encounter deleted"
                        } else {
                            ""
                        }
                    );
                    list_row(ui, &run.path, &run.encounter, &detail, false, None, |ui| {
                        row_menu(ui, &run.encounter, |ui| {
                            if menu_action(
                                ui,
                                "Delete the saved run",
                                live.filter(|live| live.id == run.id)
                                    .map(|_| "Close this run before deleting its save."),
                            ) {
                                *action = Some(ListAction::Delete(run.path.clone()));
                            }
                            if let Some(template) = past.template {
                                let start_blocked = blocked.or(template
                                    .summary
                                    .as_deref()
                                    .is_none_or(str::is_empty)
                                    .then_some("Add participants first"));
                                if menu_action(ui, START_FIGHT, start_blocked) {
                                    *action = Some(ListAction::Run(template.path.clone()));
                                }
                                if menu_action(ui, "Edit", None) {
                                    *action = Some(ListAction::Edit(template.path.clone()));
                                }
                            }
                        });
                        let opened = read.is_some_and(|(path, _)| *path == run.path);
                        if Action::quiet(if opened { "Close" } else { "Open" })
                            .density(Density::Read)
                            .name(&format!(
                                "{} the run of {}, {}",
                                if opened { "Close" } else { "Open" },
                                run.encounter,
                                run.label
                            ))
                            .show(ui)
                            .clicked()
                        {
                            *action = Some(ListAction::ReadHistory(run.path.clone()));
                        }
                    });
                    if let Some((path, summary)) = read
                        && *path == run.path
                    {
                        // The open run is this fight, so nothing stands in its way.
                        let here = live.is_some_and(|live| live.id == run.id);
                        // Continue fight is for the fight that was played last.
                        if history_summary(ui, summary, blocked.filter(|_| !here), not_last) {
                            *action = Some(ListAction::ContinueFight(run.path.clone()));
                        }
                    }
                }
            }
            if !page.unreadable.is_empty() {
                design::group_label(ui, &format!("Unreadable {}", page.unreadable.len()));
                for run in &page.unreadable {
                    blocked_reason(ui, Some(&run.label));
                }
            }
        },
    );
}

/// An ended fight, read in place. Returns true when the DM chooses to go on with it.
/// `not_last` says why this fight cannot go on, when it is not the fight that was played last:
/// then its sentence stands where the control would be.
fn history_summary(
    ui: &mut egui::Ui,
    summary: &FightSummary,
    blocked: Option<&str>,
    not_last: Option<&str>,
) -> bool {
    let mut go_on = false;
    ui.indent("history_summary", |ui| {
        ui.horizontal_wrapped(|ui| {
            design::status(
                ui,
                Status {
                    icon: icons::READ_ONLY,
                    word: "Read-only",
                    color: colors().document.read_only,
                },
            );
            ui.label(
                Type::Small
                    .rich(ui, &summary.end_reason)
                    .color(colors().muted),
            );
        });
        // The one way on from a fight that ended by mistake. Undo puts the ending back.
        match not_last {
            Some(reason) => blocked_reason(ui, Some(reason)),
            None => {
                go_on = crate::encounter_run::view::continue_action()
                    .density(Density::Read)
                    .blocked(blocked)
                    .show(ui)
                    .clicked();
            }
        }
        design::group_label(ui, "Final state");
        for row in &summary.rows {
            ui.horizontal_wrapped(|ui| {
                ui.label(Type::Strong.rich(ui, &row.name).color(colors().text));
                let hp = row.hp.map_or_else(|| "?".to_owned(), |hp| hp.to_string());
                let max = row
                    .max_hp
                    .map_or_else(|| "?".to_owned(), |hp| hp.to_string());
                ui.label(
                    Type::Small
                        .rich(ui, format!("HP {hp}/{max}"))
                        .color(colors().text),
                );
                if row.temporary_hp > 0 {
                    ui.label(
                        Type::Small
                            .rich(ui, format!("{} +{}", icons::TEMP_HP, row.temporary_hp))
                            .color(colors().hp.temp),
                    );
                }
                if !row.present {
                    blocked_reason(ui, Some("Absent"));
                }
                if !row.conditions.is_empty() {
                    blocked_reason(ui, Some(&row.conditions.join(", ")));
                }
            });
        }
        design::group_label(ui, "Combat log");
        for line in &summary.log {
            blocked_reason(ui, Some(line));
        }
    });
    go_on
}

#[cfg(test)]
mod iter_2_c_tests {
    use super::*;
    use crate::timeline::{Book, Kind, page::SessionLink};

    /// Astra review of PR 231: a row menu opens below or above its button, never beside it, and
    /// starts at the button's left edge only when ending at its right edge would cross the pane.
    #[test]
    fn iter22_b_row_menu_opens_below_or_above_inside_its_pane() {
        use egui::{RectAlign, pos2, vec2};
        let pane = egui::Rect::from_min_max(pos2(150.0, 90.0), pos2(590.0, 632.0));
        let button = egui::Rect::from_min_max(pos2(561.0, 600.0), pos2(585.0, 628.0));
        let menu = Some(vec2(236.0, 110.0));
        let end = [RectAlign::BOTTOM_END, RectAlign::TOP_END];
        // Unknown size (the first frame) and the usual case: end at the button's right edge.
        assert_eq!(menu_aligns(button, None, pane), end);
        assert_eq!(menu_aligns(button, menu, pane), end);
        // A button near the pane's left edge: ending at it would cross the pane, so start at it.
        let near_left = button.translate(vec2(160.0 - button.left(), 0.0));
        assert_eq!(
            menu_aligns(near_left, menu, pane),
            [RectAlign::BOTTOM_START, RectAlign::TOP_START]
        );
        // A pane too narrow for either: still end at the button, never beside it.
        let narrow = egui::Rect::from_min_max(pos2(150.0, 90.0), pos2(330.0, 632.0));
        assert_eq!(menu_aligns(near_left, menu, narrow), end);
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
            },
        );
        assert_eq!(detail.matches("Saved ").count(), 1);
        assert!(!detail.contains("2026-01-01"));
    }

    /// FT2: a paused copy's line gives up its last change before its save time.
    #[test]
    fn a_paused_copy_shortens_its_last_change_and_keeps_its_save_time() {
        let context = egui::Context::default();
        design::theme::install(&context);
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| check(ui));
        });
        fn check(ui: &egui::Ui) {
            let font = Type::Small.font(ui);
            let width = |text: &str| design::number_width(ui, text, &font);
            assert!(width("Last: 1") > 0.0, "the test measures text");
            let detail = "Copy 2 · Closed last · Round 2 · Paused · Last: 1 damage to Kestrel Dunmoor (6 HP left) · 2026-01-01 03:00";
            // It fits: nothing changes.
            assert_eq!(fit_detail(ui, detail, &font, width(detail)), detail);
            // Room for some of the last change: it loses whole words, and the time stays whole.
            let shortened = "Copy 2 · Closed last · Round 2 · Paused · Last: 1 damage to Kestrel… · 2026-01-01 03:00";
            let fitted = fit_detail(ui, detail, &font, width(shortened));
            assert!(fitted.ends_with(" · 2026-01-01 03:00"), "{fitted}");
            assert!(fitted.contains(" · Last: 1 damage to Kestrel…"), "{fitted}");
            assert!(width(&fitted) <= width(shortened));
            // Room for no word of it: the last change goes, and the time stays whole.
            let without = "Copy 2 · Closed last · Round 2 · Paused · 2026-01-01 03:00";
            let fitted = fit_detail(
                ui,
                detail,
                &font,
                width(without) + width(" · Last: …") / 2.0,
            );
            assert_eq!(fitted, without);
            // A line with a warning keeps that part whole too.
            let changed = "Changed on disk · Round 2 · Paused · Last: 3 damage to Kestrel Dunmoor (8 HP left) · 2026-01-01 03:00";
            let fitted = fit_detail(ui, changed, &font, width(changed) - 1.0);
            assert!(
                fitted.starts_with("Changed on disk · Round 2 · Paused · Last: 3"),
                "{fitted}"
            );
            assert!(fitted.ends_with(" · 2026-01-01 03:00"), "{fitted}");
            // A line without a last change still shortens from its end.
            let history = "Victory · Round 4 · 2026-01-01 03:00";
            let fitted = fit_detail(ui, history, &font, width("Victory · Round 4 · 2026-01-01"));
            assert_eq!(
                fitted,
                design::words_that_fit(ui, history, &font, width("Victory · Round 4 · 2026-01-01"))
            );
            assert!(fitted.ends_with('…'));
        }
    }
}
