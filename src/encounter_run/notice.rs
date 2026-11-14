//! The run's header. Its second line is the notice line: the notice, the Undo label beside it,
//! and the rule for what shortens first.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{
    Command, NOBODY_PRESENT, Panel, Phase, Run, SHEET_SOURCE,
    view::{DOCK_ALL_WORDS_FROM, dock_name, header_height, log_link, undo_forms},
};
use crate::design::{self, Action, Density, RunScale, Status, Tone, Type, colors, icons};
use eframe::egui::{self, Align, Layout, vec2};

/// What sync needs from the DM, on the line above the roster.
struct SyncAlert {
    text: String,
    /// For when `text` does not fit beside a long Undo.
    short: String,
    color: egui::Color32,
}
impl SyncAlert {
    fn new(icon: &str, text: &str, short: &str, color: egui::Color32) -> Self {
        Self {
            text: format!("{icon} {text}"),
            short: format!("{icon} {short}"),
            color,
        }
    }
}

impl Run {
    /// The current turn's reminders, from the effects with a time: all of them whole, their
    /// count, and the whole words for the hover and the accessible name.
    pub(super) fn turn_reminders(&self) -> Option<(String, String, String)> {
        if self.state.phase != Phase::Running {
            return None;
        }
        let p = self.state.participants.get(self.current()?)?;
        let reminders: Vec<String> = p
            .conditions
            .iter()
            .filter_map(super::Condition::reminder)
            .collect();
        if reminders.is_empty() {
            return None;
        }
        let whole = reminders.join(" · ");
        let full = format!("Reminders for {}: {whole}", p.name);
        Some((whole, reminders.len().to_string(), full))
    }

    /// X49: whether Next turn keeps its `N` key letter. When the current turn's name, at its
    /// least size, is cut beside it and would show more words in its room, the letter gives
    /// way first. The Keys list still has N.
    fn turn_name_keeps_room(
        &self,
        ui: &egui::Ui,
        scale: RunScale,
        name: &str,
        words: bool,
    ) -> bool {
        let turn_size = scale.hp;
        let least = (scale.name + design::HAIRLINE_WIDTH).min(turn_size);
        let gap = ui.spacing().item_spacing.x;
        let next = Action::primary("Next turn")
            .icon(icons::NEXT)
            .density(Density::Glance)
            .key("N")
            .natural_width(ui);
        let add = self.add_to_fight(words, Density::Glance).natural_width(ui);
        let round = design::number_width(
            ui,
            &self.state_words(false),
            &Type::Round.sized(ui, turn_size),
        );
        let room = ui.available_width()
            - (next + add + round + 3.0 * (gap + design::S))
            - design::HAIRLINE_WIDTH;
        let font = Type::Name.sized(ui, least);
        let keyless = Action::primary("Next turn")
            .icon(icons::NEXT)
            .density(Density::Glance)
            .natural_width(ui);
        design::number_width(ui, name, &font) <= room
            || dock_name(ui, name, &font, room) == dock_name(ui, name, &font, room + next - keyless)
    }

    /// The round and the turn are the strongest things on the page, above any row, and
    /// they grow with the rows. Next up is the only name that may be off the roster's view.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    pub(super) fn live_header(&mut self, ui: &mut egui::Ui, scale: RunScale) -> bool {
        let c = colors();
        let strip = self.strip();
        let name = strip.current.as_deref().unwrap_or_default();
        let mut next_turn = false;
        let nobody_present = !self.state.participants.iter().any(|p| p.present);
        // As large as the largest HP, in the row's own face: nothing outweighs the header.
        let turn_size = scale.hp;
        let height = header_height(scale);
        // A narrow roster keeps its room for the turn's name.
        let words = ui.available_width() >= DOCK_ALL_WORDS_FROM;
        let keeps_room = self.turn_name_keeps_room(ui, scale, name, words);
        let next_key = !self.lookup_open && keeps_room;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), height),
            Layout::right_to_left(Align::Center),
            |ui| {
                ui.set_min_height(height);
                let next = || {
                    Action::primary("Next turn")
                        .icon(icons::NEXT)
                        .density(Density::Glance)
                };
                // FK8: with Lookup open the key letter hides (the keys do not work then), but its
                // room stays, so `Add to fight` keeps its x when Lookup opens and closes.
                // Only on a roster wide enough for every word, so the room does not come and go
                // with the turn's name (a pressed Next turn would move).
                let kept = (self.lookup_open && words).then(|| next().key("N").natural_width(ui));
                let action = next().key(if next_key { "N" } else { "" });
                let action = match kept {
                    Some(width) => action.min_width(width),
                    None => action,
                };
                next_turn = action
                    .blocked(nobody_present.then_some(NOBODY_PRESENT))
                    .show(ui)
                    .clicked();
                // As the strip pairs Back to run with Next turn: one height, one small gap.
                design::gap(ui, design::Gap::S);
                if self.add_to_fight(words, Density::Glance).show(ui).clicked() {
                    self.show_arrivals();
                }
                design::gap(ui, design::Gap::S);
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let response = ui.label(
                        Type::Round
                            .rich(ui, self.state_words(false))
                            .font(Type::Round.sized(ui, turn_size))
                            .color(c.text),
                    );
                    design::named(
                        ui,
                        &response,
                        &format!("Run · {} · Round {}", self.name, self.state.round),
                    );
                    design::gap(ui, design::Gap::S);
                    // Drop the turn suffix before shortening the identifying name.
                    let room = ui.available_width() - design::HAIRLINE_WIDTH;
                    let least = (scale.name + design::HAIRLINE_WIDTH).min(turn_size);
                    let full = format!("{name}’s turn");
                    let font = Type::Name.sized(ui, turn_size);
                    let fit = |text: &str| {
                        (turn_size * room / design::number_width(ui, text, &font)).min(turn_size)
                    };
                    let (text, size) = if fit(&full) >= least {
                        (full.clone(), fit(&full))
                    } else if fit(name) >= least {
                        (name.to_owned(), fit(name))
                    } else {
                        let font = Type::Name.sized(ui, least);
                        (dock_name(ui, name, &font, room), least)
                    };
                    let response = ui.add(
                        egui::Label::new(
                            Type::Name
                                .rich(ui, text)
                                .font(Type::Name.sized(ui, size))
                                .color(c.turn.current),
                        )
                        .truncate(),
                    );
                    design::named(ui, &response, &full);
                    response.on_hover_text(&full);
                });
            },
        );
        ui.add_space(6.0);
        // Measure the notice first; Undo takes the remaining room on line two.
        let undo_label = self.undo_label();
        let mut undo = false;
        let mut review_conflict = false;
        let party_sync_off = self.party_sync_off();
        let needs_connection = strip.sync_stopped || (self.sync_needs_sign_in() && !party_sync_off);
        let offline_color = self
            .account_status
            .map_or(c.connection.offline, |status| status.color);
        let sync_alert = if let Some(name) = &strip.conflict {
            Some(SyncAlert::new(
                icons::WARN,
                &format!("Sync conflict: {name}"),
                &format!("Conflict: {name}"),
                c.sync.conflict,
            ))
        } else if strip.sync_stopped {
            Some(SyncAlert::new(
                icons::OFFLINE,
                "Signed out of D&D Beyond",
                "Signed out",
                offline_color,
            ))
        } else if self.sync_needs_sign_in() && !party_sync_off {
            Some(SyncAlert::new(
                icons::SYNC_OFF,
                "Not connected",
                "Not connected",
                offline_color,
            ))
        } else if !strip.unsent.is_empty() {
            Some(SyncAlert::new(
                icons::WARN,
                "Changes kept in Splat, not sent",
                "Not sent",
                c.result.warning,
            ))
        } else if let Some(name) = &strip.sync_error {
            Some(SyncAlert::new(
                icons::ERROR,
                &format!("Sync error: {name}"),
                "Sync error",
                c.result.error,
            ))
        } else if party_sync_off {
            Some(SyncAlert::new(
                icons::SYNC_OFF,
                "Sync off for the party",
                "Sync off for the party",
                c.muted,
            ))
        } else {
            None
        };
        let mut sheet_notice_in_header = false;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), Density::Read.control()),
            Layout::right_to_left(Align::Center),
            |ui| {
                let font = Type::Small.font(ui);
                let notice = if self.recording_notice.is_some() && strip.conflict.is_none() {
                    format!(
                        "{} Recording silence · {}",
                        icons::WARN,
                        sync_alert.as_ref().map_or("", |alert| alert.short.as_str())
                    )
                } else if let Some(alert) = &sync_alert {
                    // Measure only what is painted: no space after a notice with no link.
                    let link = if needs_connection {
                        "Connect to sync"
                    } else if self.hide_panel && strip.conflict.is_some() {
                        "Review"
                    } else {
                        ""
                    };
                    if link.is_empty() {
                        alert.text.clone()
                    } else {
                        format!("{} {link}", alert.text)
                    }
                } else if nobody_present {
                    format!("{} No participants are present", icons::WARN)
                } else {
                    strip
                        .next
                        .as_ref()
                        .map_or(String::new(), |next| format!("Next: {next}"))
                };
                let action_font = Type::Strong.font(ui);
                let from_sheet = self.undo_from_sheet();
                // The forms of the one rule, once a frame, from the step's own target.
                let target = self.undo_target();
                let forms = undo_forms(&undo_label, target, from_sheet, &self.state.participants);
                let fitted = forms.target.first().unwrap_or(&undo_label).clone();
                // The reserve: the target cut to `5 × L`, at its own place in the words.
                let mut minimum = fitted.clone();
                if let Some(name) = target {
                    let short = dock_name(ui, name, &action_font, design::L * 5.0);
                    if let Some(before) = minimum.strip_suffix(name) {
                        minimum = format!("{before}{short}");
                    } else if let Some(after) = minimum.strip_prefix(&format!("Undo: {name}")) {
                        minimum = format!("Undo: {short}{after}");
                    }
                }
                // The shortest target Undo keeps: a sheet's step also gives up its source.
                let shortest = minimum
                    .strip_prefix("Undo: ")
                    .and_then(|action| action.strip_prefix(SHEET_SOURCE))
                    .filter(|_| from_sheet)
                    .map_or_else(|| minimum.clone(), |action| format!("Undo: {action}"));
                let target_reserve = self.undo_action(&shortest).natural_width(ui) + design::XS;
                let minimum = self.undo_action(&minimum).natural_width(ui)
                    + design::XS
                    + if self.history.is_empty() {
                        design::number_width(ui, "Nothing to undo", &font) + design::S
                    } else {
                        0.0
                    };
                // A state notice stays whole; Undo then shortens its target, and shows the word
                // Undo alone only when no target fits. Only the Next notice gives way to Undo's
                // shortened target. Undo keeps its full accessible name and hover text.
                let state_notice =
                    sync_alert.is_some() || self.recording_notice.is_some() || nobody_present;
                let silence_beside_sync_off = party_sync_off && self.recording_notice.is_some();
                let beside_state = state_notice && !self.history.is_empty();
                let minimum = if silence_beside_sync_off || beside_state {
                    self.undo_action("Undo").natural_width(ui) + design::XS
                } else {
                    minimum
                };
                // A shortened Undo target comes before the sheet-change notice, which can go
                // below the roster.
                let sheet_reserve = if beside_state && !silence_beside_sync_off {
                    target_reserve
                } else {
                    minimum
                };
                let notice_width = design::number_width(ui, &notice, &font) + design::S;
                let sheet_room =
                    (ui.available_width() - sheet_reserve - notice_width - design::L).max(0.0);
                let sheet_width = |summary: &str| {
                    design::number_width(ui, &format!("{} {summary}", icons::WARN), &font)
                        + design::number_width(ui, "Log", &Type::Small.font(ui))
                        + Density::Read.control()
                        + design::L
                        + design::S * 3.0
                };
                let sheet = self
                    .notice_summary
                    .as_ref()
                    .filter(|_| self.notice.is_some() && self.recording_notice.is_none())
                    .and_then(|summary| {
                        header_sheet_forms(summary)
                            .into_iter()
                            .find(|form| sheet_width(form) <= sheet_room)
                    });
                let sheet_width = sheet.as_ref().map_or(0.0, |summary| sheet_width(summary));
                sheet_notice_in_header = sheet.is_some();
                let notice_width = (notice_width + if sheet.is_some() { sheet_width } else { 0.0 })
                    .min((ui.available_width() - minimum - design::L).max(0.0));
                let mut slot = ui.available_width() - notice_width - design::L;
                let mut keyless = false;
                let shown = if party_sync_off
                    && self.recording_notice.is_some()
                    && self.undo_action(&fitted).natural_width(ui) > slot
                {
                    "Undo".to_owned()
                } else {
                    // The key letter goes before the target does; the Keys list still has U.
                    // No target fits: the step's words without it, `Undo: 8 damage × 3`,
                    // before `Undo` alone. Hover keeps the whole step.
                    let strong = Type::Strong.font(ui);
                    let width_of = |words: &str, drop_key: bool| {
                        let action = self.undo_action(words);
                        let action = if drop_key { action.key("") } else { action };
                        action.natural_width(ui)
                    };
                    let target = |words: &str, drop_key: bool| {
                        design::number_width(ui, words, &strong)
                            <= slot - width_of("", drop_key) - design::XS
                    };
                    let mut shown = "Undo".to_owned();
                    // iter-27-R SJ1: the key letter gives way before any word does: only the
                    // whole label keeps it, and every shorter form is tried without it.
                    'fit: for (tier, measured) in [(&forms.target, true), (&forms.bare, false)] {
                        for (at, words) in tier.iter().enumerate() {
                            let whole = measured && at == 0;
                            for drop_key in [!whole, true] {
                                if width_of(words, drop_key) <= slot
                                    && (!measured || target(words, drop_key))
                                {
                                    shown.clone_from(words);
                                    keyless = drop_key;
                                    break 'fit;
                                }
                            }
                        }
                    }
                    shown
                };
                // Part 3, the current turn's reminders, takes only the room that the notice,
                // the sheet summary and today's Undo label leave. `Next:` gives way to it.
                let next_shown = sync_alert.is_none()
                    && self.recording_notice.is_none()
                    && !nobody_present
                    && strip.next.is_some();
                let undo_width = if self.history.is_empty() {
                    self.undo_action("Undo").natural_width(ui)
                        + design::S
                        + design::number_width(ui, self.empty_undo_words(), &font)
                } else {
                    let action = self.undo_action(&shown);
                    let action = if keyless { action.key("") } else { action };
                    action.natural_width(ui)
                };
                let next_width = if next_shown {
                    design::number_width(ui, &notice, &font) + design::S
                } else {
                    0.0
                };
                let reminder = self.turn_reminders().and_then(|(whole, count, full)| {
                    let room = slot - undo_width - design::L + next_width;
                    [whole, count]
                        .into_iter()
                        .map(|words| {
                            let width =
                                design::number_width(ui, &format!("{} {words}", icons::PIN), &font)
                                    + design::XS;
                            (words, width)
                        })
                        .find(|(_, width)| *width <= room)
                        .map(|(words, width)| (words, width, full))
                });
                if let Some((_, width, _)) = &reminder {
                    slot -= width + design::L - next_width;
                }
                ui.allocate_ui_with_layout(
                    vec2(slot, Density::Read.control()),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        undo = if self.history.is_empty() {
                            self.empty_undo(ui)
                        } else {
                            let action = self.undo_action(&shown);
                            let action = if keyless { action.key("") } else { action };
                            action
                                .name(&undo_label)
                                .hint(&undo_label)
                                .show(ui)
                                .clicked()
                        };
                    },
                );
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = design::XS;
                    let font = Type::Small.font(ui);
                    if let Some(notice) = self.recording_notice.filter(|_| strip.conflict.is_none())
                    {
                        design::status(
                            ui,
                            Status {
                                icon: icons::WARN,
                                word: "Recording silence",
                                color: c.result.warning,
                            },
                        )
                        .on_hover_text(notice);
                        if sync_alert.is_some() {
                            ui.label(Type::Small.rich(ui, "·").color(c.muted));
                        }
                    }
                    if nobody_present && self.recording_notice.is_none() && strip.conflict.is_none()
                    {
                        // A warning has its icon, as `2 tied` does.
                        ui.label(
                            Type::Small
                                .rich(ui, format!("{} No participants are present", icons::WARN))
                                .color(c.result.warning),
                        )
                        .on_hover_text(NOBODY_PRESENT);
                    } else if let Some(alert) = &sync_alert {
                        let link = if strip.conflict.is_some() && self.hide_panel {
                            "Review"
                        } else if needs_connection {
                            "Connect to sync"
                        } else {
                            ""
                        };
                        let full = &alert.text;
                        let link_width = design::number_width(ui, link, &font) + design::S;
                        // A conflict's name wider than the line beside the bare Undo shortens
                        // as the dock's name does; the icon, `Conflict` and `Review` stay.
                        let short = match &strip.conflict {
                            Some(name)
                                if design::number_width(ui, &alert.short, &font)
                                    + link_width
                                    + design::XS
                                    > ui.available_width() =>
                            {
                                let prefix = format!("{} Conflict: ", icons::WARN);
                                let room = ui.available_width()
                                    - link_width
                                    - design::XS
                                    - design::number_width(ui, &prefix, &font);
                                format!("{prefix}{}", dock_name(ui, name, &font, room))
                            }
                            _ => alert.short.clone(),
                        };
                        let fits_link = !link.is_empty()
                            && design::number_width(ui, &short, &font) + link_width + design::XS
                                <= ui.available_width();
                        let text_width = ui.available_width()
                            - if sheet.is_some() { sheet_width } else { 0.0 }
                            - if fits_link {
                                link_width + design::XS
                            } else {
                                0.0
                            };
                        let full_fits = design::number_width(ui, full, &font) <= text_width;
                        let words = if full_fits { full } else { &short };
                        ui.allocate_ui_with_layout(
                            vec2(text_width, Density::Read.control()),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                if strip.conflict.is_some() {
                                    let word = words
                                        .split_once(' ')
                                        .map_or(words.as_str(), |(_, word)| word);
                                    if design::status_link(
                                        ui,
                                        Status {
                                            icon: icons::WARN,
                                            word,
                                            color: alert.color,
                                        },
                                    )
                                    .on_hover_text(full)
                                    .clicked()
                                    {
                                        review_conflict = true;
                                    }
                                } else {
                                    ui.add(
                                        egui::Label::new(
                                            Type::Small.rich(ui, words).color(alert.color),
                                        )
                                        .truncate(),
                                    )
                                    .on_hover_text(full);
                                }
                            },
                        );
                        // X69: `S` before the link, as `link_width` counts it, so a long
                        // name never runs into `Review`.
                        if fits_link {
                            design::gap(ui, design::Gap::XS);
                        }
                        if !link.is_empty() && fits_link && design::small_link(ui, link).clicked() {
                            if link == "Review" {
                                review_conflict = true;
                            } else {
                                self.wants_sign_in = true;
                            }
                        }
                    } else if self.recording_notice.is_none() {
                        let text = if nobody_present {
                            NOBODY_PRESENT.to_owned()
                        } else {
                            strip
                                .next
                                .as_ref()
                                .map_or(String::new(), |next| format!("Next: {next}"))
                        };
                        // Undo is shortened first. Then the next name goes whole, because
                        // the roster marks NEXT on its row. It gives way to the reminders.
                        let shown = if nobody_present {
                            design::words_that_fit(ui, &text, &font, ui.available_width())
                        } else if reminder.is_some() {
                            String::new()
                        } else if design::number_width(ui, &text, &font) <= ui.available_width() {
                            text
                        } else {
                            String::new()
                        };
                        ui.label(Type::Small.rich(ui, shown).color(if nobody_present {
                            c.result.warning
                        } else {
                            c.turn.next
                        }));
                    }
                    if let Some(summary) = &sheet {
                        design::gap(ui, design::Gap::S);
                        // The compact message's own top alignment is useful in stacked
                        // notices. Here its text-height box is centred by the control row.
                        let width =
                            design::number_width(ui, &format!("{} {summary}", icons::WARN), &font)
                                + design::S
                                + design::HAIRLINE_WIDTH
                                + design::XS;
                        let response = ui
                            .allocate_ui_with_layout(
                                vec2(width, Type::Small.line()),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ui.spacing_mut().item_spacing.x = design::S;
                                    design::small_message(ui, Tone::Warning, summary)
                                },
                            )
                            .inner;
                        let detail = self.notice.as_deref().unwrap_or(summary);
                        design::named(ui, &response, detail);
                        response.on_hover_text(detail);
                        let log = log_link(ui);
                        design::named(ui, &log, "Log");
                        if log.clicked() {
                            self.panel = Panel::Log;
                            self.close_lookup = true;
                        }
                        if design::icon_button(ui, icons::CLOSE, "Dismiss this message").clicked() {
                            self.notice = None;
                            self.notice_summary = None;
                        }
                    }
                    if let Some((words, _, full)) = &reminder {
                        if sheet.is_some() || !next_shown {
                            design::gap(ui, design::Gap::S);
                        }
                        let response = ui
                            .horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = design::XS;
                                ui.label(Type::Small.rich(ui, icons::PIN).color(c.result.info));
                                ui.label(Type::Small.rich(ui, words.as_str()).color(c.text));
                            })
                            .response;
                        design::named(ui, &response, full);
                        response.on_hover_text(full);
                    }
                });
            },
        );
        ui.add_space(2.0);
        design::rule(ui);
        if review_conflict {
            self.review_conflict();
        }
        if undo {
            self.undo();
            self.follow_with_keyboard();
        }
        if next_turn {
            self.run_command(Command::Next);
        }
        sheet_notice_in_header
    }
}

/// The sheet-change notice's forms on the notice line, longest first: the whole summary,
/// then its first changed value with the name (`Brannoch Vale: HP 58 → 54`). A count of
/// removed Undo steps stays in both (FJ1, X47): with no room for it, the notice goes below
/// the roster, where it keeps the count too.
fn header_sheet_forms(summary: &str) -> [String; 2] {
    let (changes, removed) = summary
        .rsplit_once(" · ")
        .filter(|(_, removed)| removed.ends_with(" removed"))
        .map_or((summary, String::new()), |(changes, removed)| {
            (changes, format!(" · {removed}"))
        });
    let compact = changes
        .split(", ")
        .next()
        .unwrap_or(changes)
        .replace(&format!("{} ", super::SHEET_CHANGED), ": ");
    [summary.to_owned(), format!("{compact}{removed}")]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encounter_run::Phase;
    use eframe::egui::Rect;

    #[test]
    fn iter8_b_sheet_notice_centres_with_next_and_log_and_has_a_gap() {
        use egui_kittest::kittest::Queryable;
        for width in [1200.0, 1920.0] {
            let mut run = super::super::tests::run();
            for (index, total) in [20, 15, 10].into_iter().enumerate() {
                run.apply(Command::Initiative(index, Some(total))).unwrap();
            }
            run.apply(Command::Begin).unwrap();
            run.notice = Some("Guard 1’s sheet changed: HP 17 → 15. Undo stops here.".into());
            run.notice_summary = Some("Guard 1: HP 17 → 15".into());
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 800.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        run.live_header(ui, design::RUN_REGULAR);
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let log = harness.get_by_label("Log").rect();
            let text = |prefix: &str| {
                harness
                    .output()
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) if text.galley.text().starts_with(prefix) => {
                            Some(text)
                        }
                        _ => None,
                    })
                    .unwrap_or_else(|| panic!("Required text: {prefix}"))
            };
            let next = text("Next:");
            let notice = text("Guard 1: HP");
            let next_rect = Rect::from_min_size(next.pos, next.galley.size());
            let notice_rect = Rect::from_min_size(notice.pos, notice.galley.size());
            let icon = text(icons::WARN);
            let icon_rect = Rect::from_min_size(icon.pos, icon.galley.size());
            assert!(
                (notice_rect.left() - icon_rect.right() - design::XS).abs() <= 1.0,
                "iter9-B-FT7-icon-gap: {icon_rect:?}, {notice_rect:?}"
            );
            let bar = harness
                .output()
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Rect(rect) if rect.fill == colors().result.warning => {
                        Some(rect.rect)
                    }
                    _ => None,
                })
                .expect("Required warning bar");
            assert!(
                (notice_rect.center().y - next_rect.center().y).abs() <= 1.0
                    && (notice_rect.center().y - log.center().y).abs() <= 1.0,
                "iter8-B-AT2-centre: {notice_rect:?}, {next_rect:?}, {log:?}"
            );
            assert!(
                bar.left() - next_rect.right() >= design::S,
                "iter8-B-AT2-gap: {bar:?}, {next_rect:?}"
            );
            assert!(
                (bar.center().y - notice_rect.center().y).abs() <= 1.0,
                "iter8-B-AT2-bar: {bar:?}, {notice_rect:?}"
            );
        }
    }

    /// F2, section 1.4: the current turn's reminders take only the room that the notice,
    /// the sheet summary and today's Undo label leave: whole, else their count, else nothing.
    /// No reminder makes Undo shorter. Row 20: a conflict's long name shortens, and `Review`
    /// stays.
    #[test]
    fn reminders_take_only_spare_room_and_a_long_conflict_name_shortens() {
        use crate::encounter_run::Condition;
        use egui_kittest::kittest::Queryable;
        let header = |mut run: crate::encounter_run::Run, width: f32| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 640.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        run.live_header(ui, design::RUN_REGULAR);
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            harness
        };
        let painted = |harness: &egui_kittest::Harness<'_>, words: &str| {
            harness.output().shapes.iter().any(|shape| {
                matches!(&shape.shape, egui::Shape::Text(text) if text.galley.text() == words)
            })
        };
        let undo = |harness: &egui_kittest::Harness<'_>| {
            harness
                .query_by_label_contains("Undo:")
                .map(|node| node.rect())
        };
        let begun = || {
            let mut run = super::super::tests::run();
            for (index, total) in [20, 15, 10].into_iter().enumerate() {
                run.apply(Command::Initiative(index, Some(total))).unwrap();
            }
            run.apply(Command::Begin).unwrap();
            run
        };
        let name = begun().state.participants[0].name.clone();
        let next = begun().state.participants[1].name.clone();
        let mut long = begun();
        let mut three = begun();
        let plain = header(begun(), 442.0);
        assert!(painted(&plain, &format!("Next: {next}")));
        let before = undo(&plain).expect("Undo with its target");
        long.state.participants[0].conditions =
            vec![Condition::read("Wis save DC 15 (Hold Person) on its turn")];
        let harness = header(long, 442.0);
        assert!(painted(&harness, "Wis save DC 15 (Hold Person)"));
        assert!(
            !painted(&harness, &format!("Next: {next}")),
            "Next gives way"
        );
        assert!(
            harness
                .query_by_label(&format!(
                    "Reminders for {name}: Wis save DC 15 (Hold Person)"
                ))
                .is_some()
        );
        assert_eq!(
            undo(&harness),
            Some(before),
            "no reminder makes Undo shorter"
        );
        let reminder = harness
            .output()
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.text() == "Wis save DC 15 (Hold Person)" => {
                    Some(text.visual_bounding_rect())
                }
                _ => None,
            })
            .expect("the reminder's words");
        assert!(
            reminder.right() < before.left(),
            "{reminder:?} clear of {before:?}"
        );
        // Narrower, the whole words would need Undo's room: they are the count, and Undo and
        // its target keep their place.
        let narrow = header(begun(), 360.0);
        let before_narrow = undo(&narrow).expect("Undo with its target");
        let mut tight = begun();
        tight.state.participants[0].conditions =
            vec![Condition::read("Wis save DC 15 (Hold Person) on its turn")];
        let harness = header(tight, 360.0);
        assert!(!painted(&harness, "Wis save DC 15 (Hold Person)"));
        assert!(painted(&harness, "1"), "the count");
        assert_eq!(undo(&harness), Some(before_narrow));
        three.state.participants[0].conditions = vec![
            Condition::read("Wis save DC 15 (Hold Person) on its turn"),
            Condition::read("Wis save DC 15 (Hold Person) ends 10 rounds"),
            Condition::read("Wis save DC 15 (Hold Monster) on its turn"),
        ];
        let harness = header(three, 442.0);
        assert!(
            painted(&harness, "3"),
            "three long reminders are their count"
        );
        assert_eq!(undo(&harness), Some(before));
        // A conflict comes first: a long name shortens beside the bare Undo, `Review` stays,
        // and the reminders have no room.
        let mut conflict = super::super::tests::conflicted();
        conflict.state.participants[0].name =
            "Oren Thistlewick, Keeper of the Southern Road".into();
        conflict.state.participants[0].conditions =
            vec![Condition::read("Wis save DC 15 (Hold Person) on its turn")];
        conflict.hide_panel = true;
        let harness = header(conflict, 442.0);
        assert!(
            harness.query_by_label("Review").is_some(),
            "{:?}",
            harness.root()
        );
        let shown = harness
            .output()
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.text().contains("Conflict: ") => {
                    Some(text.galley.text().to_owned())
                }
                _ => None,
            })
            .expect("the conflict's words");
        assert!(shown.ends_with('…'), "{shown}");
        assert!(!painted(&harness, "Wis save DC 15 (Hold Person)"));
    }

    #[test]
    fn iter6_b_turn_heading_keeps_each_of_five_copy_numbers() {
        for number in 1..=5 {
            for width in [442.0, 760.0] {
                let mut run = super::super::tests::run();
                let base = run.state.participants[0].clone();
                run.state.participants = (1..=5)
                    .map(|n| {
                        let mut p = base.clone();
                        p.name = format!("Cinder Wolf {n}");
                        p.key = format!("wolf-{n}");
                        p.initiative = Some(20 - n);
                        p
                    })
                    .collect();
                run.state.order = (0..5).collect();
                run.state.cursor = number - 1;
                run.state.phase = Phase::Running;
                run.state.round = 1;
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(vec2(width, 640.0))
                    .build(move |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            run.live_header(ui, design::RUN_REGULAR);
                        });
                    });
                design::theme::install(&harness.ctx);
                harness.run_steps(2);
                let texts: Vec<_> = harness
                    .output()
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text),
                        _ => None,
                    })
                    .collect();
                let heading = texts
                    .iter()
                    .find(|text| {
                        text.galley
                            .job
                            .sections
                            .iter()
                            .any(|section| section.format.color == colors().turn.current)
                    })
                    .expect("turn heading");
                assert!(
                    heading.galley.text().contains(&number.to_string()),
                    "iter6-B-FT1: {}",
                    heading.galley.text()
                );
                assert!(
                    !heading.galley.elided,
                    "iter6-B-FT1: the fitted heading stays whole"
                );
            }
        }
    }

    #[test]
    fn iter6_b_narrow_header_keeps_add_word() {
        let mut run = super::super::tests::run();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(442.0, 640.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    run.live_header(ui, design::RUN_REGULAR);
                });
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        assert!(
            harness
                .output()
                .shapes
                .iter()
                .any(|shape| matches!(&shape.shape,
            egui::Shape::Text(text) if text.galley.text().contains("Add") && !text.galley.elided)),
            "iter6-B-AT2: Add keeps a whole word"
        );
    }

    #[test]
    fn iter_2_b_sheet_notice_uses_header_room_and_keeps_undo() {
        use egui_kittest::kittest::Queryable;
        for (width, in_header) in [(1920.0, true), (442.0, false)] {
            let mut run = super::super::tests::run();
            for (index, total) in [20, 15, 10].into_iter().enumerate() {
                run.apply(Command::Initiative(index, Some(total))).unwrap();
            }
            run.apply(Command::Begin).unwrap();
            run.notice =
                Some("Guard 1’s sheet changed: HP 17 → 15. Undo for Guard 1 stops here.".into());
            run.notice_summary = Some("Guard 1’s sheet changed: HP 17 → 15".into());
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 640.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        run.live_header(ui, design::RUN_REGULAR);
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            assert_eq!(
                harness
                    .query_all_by_label_contains("Guard 1’s sheet changed: HP 17 → 15")
                    .count()
                    > 0,
                in_header,
                "{width}: {:?}",
                harness.root()
            );
            assert!(harness.query_by_label("Undo: begin combat").is_some());
        }
    }
}
