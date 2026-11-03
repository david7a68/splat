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
    let button = Action::quiet(icons::EXPAND)
        .density(Density::Read)
        .name(&name)
        .hint(&name)
        .show(ui);
    egui::Popup::menu(&button)
        .align(egui::RectAlign::BOTTOM_END)
        .show(add);
}

pub(super) fn menu_action(ui: &mut egui::Ui, word: &str, blocked: Option<&str>) -> bool {
    let action = match word {
        "Move up" => Action::quiet(word).icon(icons::MOVE_UP),
        "Move down" => Action::quiet(word).icon(icons::MOVE_DOWN),
        "Remove" | "Delete the saved run" => Action::quiet(word).icon(icons::REMOVE),
        _ => Action::quiet(word),
    };
    let pressed = action
        .density(Density::Read)
        .blocked(blocked)
        .show(ui)
        .clicked();
    blocked_reason(ui, blocked);
    if pressed {
        ui.close();
    }
    pressed
}

/// The same named side picker serves roster slots and groups. Other opens a text field.
pub(super) fn side_control(ui: &mut egui::Ui, name: &str, side: &mut String, choices: &[String]) {
    let id = ui.make_persistent_id((name, "other_side"));
    let mut other = ui.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
    let custom = !matches!(side.as_str(), "party" | "enemies");
    ui.allocate_ui_with_layout(
        egui::vec2(design::S * 13.0, Density::Read.control()),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            let combo = egui::ComboBox::from_id_salt(name)
                .icon(design::combo_icon)
                .width(design::S * 13.0)
                .selected_text(if other {
                    "Other…".to_owned()
                } else {
                    design::reader::human_label(side)
                })
                .show_ui(ui, |ui| {
                    for choice in ["party", "enemies"].into_iter().chain(
                        choices
                            .iter()
                            .map(String::as_str)
                            .filter(|c| !matches!(*c, "party" | "enemies")),
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
            if other || custom {
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
                    design::S * 13.0,
                )
                .changed()
                    && !text.trim().is_empty()
                {
                    *side = text.trim().to_lowercase();
                }
            }
        },
    );
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
    egui::CollapsingHeader::new(design::group_heading_text(ui, word, count))
        .id_salt(id)
        .icon(design::section_icon)
        .open(open)
        .show(ui, add);
}

fn list_row(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    name: &str,
    detail: &str,
    status: bool,
    actions: impl FnOnce(&mut egui::Ui),
) {
    ui.push_id(id, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Scan.row() + design::S);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_width((ui.available_width() - design::S * 40.0).max(design::S * 12.0));
                ui.add(
                    egui::Label::new(Type::Strong.rich(ui, name).color(colors().text)).truncate(),
                )
                .on_hover_text(name);
                let label = if detail.contains("Changed elsewhere") {
                    format!("{} {detail}", icons::WARN)
                } else if status {
                    format!("{} {detail}", icons::RUN)
                } else {
                    detail.to_owned()
                };
                let response = ui
                    .add(
                        egui::Label::new(Type::Small.rich(ui, label).color(
                            if detail.contains("Changed elsewhere") {
                                colors().result.warning
                            } else {
                                colors().muted
                            },
                        ))
                        .wrap(),
                    )
                    .on_hover_text(detail);
                design::named(ui, &response, &format!("{name} · {detail}"));
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), actions);
        });
    });
}

fn round(run: &SavedRunInfo) -> &str {
    run.label
        .split(" · ")
        .find(|part| part.starts_with("Round "))
        .unwrap_or("Setup")
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

pub(super) fn in_progress_list(
    ui: &mut egui::Ui,
    page: &history::Arranged<'_>,
    templates: &[Template],
    live: Option<&SavedRunInfo>,
    blocked: Option<&'static str>,
    loading: bool,
    action: &mut Option<ListAction>,
) {
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
        let detail = run.label.clone();
        list_row(ui, &run.path, &run.encounter, &detail, true, |ui| {
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
                let word = format!("Running · {}", round(run));
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
                    .density(Density::Read)
                    .blocked(blocked)
                    .show(ui)
                    .clicked()
                {
                    *action = Some(ListAction::Resume(run.path.clone()));
                }
            }
        });
        if !open {
            blocked_reason(ui, blocked);
        }
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
    if page.ready.is_empty() {
        blocked_reason(ui, Some("None ready. Select New encounter."));
    }
    for (index, row) in page.ready.iter().enumerate() {
        let template = row.template;
        let start_blocked = if row.unfinished_run {
            Some("This encounter has an unfinished run. Resume it in In progress.")
        } else {
            blocked
        };
        list_row(
            ui,
            &template.path,
            &template.name,
            template.summary.as_deref().unwrap_or_default(),
            false,
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
            },
        );
        blocked_reason(ui, start_blocked);
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
                if let Some(path) = groups::list(ui, snapshot, blocked) {
                    *action = Some(ListAction::OpenGroup(path));
                }
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
                        .map(crate::encounter_run::calendar_time)
                        .unwrap_or_else(|| "Date unknown".to_owned());
                    let detail = format!(
                        "{} · {date}{}",
                        run.label,
                        if past.template.is_none() {
                            " · Encounter deleted"
                        } else {
                            ""
                        }
                    );
                    list_row(ui, &run.path, &run.encounter, &detail, false, |ui| {
                        row_menu(ui, &run.encounter, |ui| {
                            if menu_action(
                                ui,
                                "Delete the saved run",
                                live.filter(|live| live.id == run.id)
                                    .map(|_| "Close this run before deleting its save."),
                            ) {
                                *action = Some(ListAction::Delete(run.path.clone()));
                            }
                        });
                        if let Some(template) = past.template {
                            if Action::quiet("Edit")
                                .density(Density::Read)
                                .name(&format!("Edit {}", run.encounter))
                                .show(ui)
                                .clicked()
                            {
                                *action = Some(ListAction::Edit(template.path.clone()));
                            }
                            if Action::secondary(START_FIGHT)
                                .density(Density::Read)
                                .name(&format!("Start fight: {}", run.encounter))
                                .blocked(blocked)
                                .show(ui)
                                .clicked()
                            {
                                *action = Some(ListAction::Run(template.path.clone()));
                            }
                        }
                        let opened = read.is_some_and(|(path, _)| *path == run.path);
                        if Action::quiet(if opened { "Close history" } else { "Open" })
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
                        history_summary(ui, summary);
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

fn history_summary(ui: &mut egui::Ui, summary: &FightSummary) {
    ui.indent("history_summary", |ui| {
        design::status(
            ui,
            Status {
                icon: icons::READ_ONLY,
                word: "Read-only",
                color: colors().document.read_only,
            },
        );
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
}
