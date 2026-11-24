//! The live run's header (egui `notice::live_header`, `design::TurnHeading`): the round and whose
//! turn it is, `Add to fight` and `Next turn`; under them the notice line (what sync needs, the
//! next turn, the turn's reminders, a sheet's changes) and Undo.
use super::{on_command, on_run};
use crate::encounter_run::{Command, NOBODY_PRESENT, Panel, Run};
use crate::gui::{SplatRoot, bridge, icons, kit, theme};
use super::measure;
use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Pixels, Styled, Window, div, px,
};
use theme::{Density, Type, TypeExt, colors};

/// What sync needs from the DM, on the notice line: the icon, the words and their colour.
struct SyncAlert {
    icon: &'static str,
    words: String,
    color: gpui::Hsla,
}

/// The sync alert the notice line shows, if any, in the egui order.
fn sync_alert(run: &Run) -> Option<SyncAlert> {
    let c = colors();
    let strip = run.strip();
    let party_sync_off = run.party_sync_off();
    let offline = run
        .account_status
        .map_or(c.connection.offline, |status| bridge::color(status.color));
    let alert = |icon, words: String, color| Some(SyncAlert { icon, words, color });
    if let Some(name) = &strip.conflict {
        alert(
            icons::WARN,
            format!("Sync conflict: {name}"),
            c.sync.conflict,
        )
    } else if strip.sync_stopped {
        alert(icons::OFFLINE, "Signed out of D&D Beyond".into(), offline)
    } else if run.sync_needs_sign_in() && !party_sync_off {
        alert(icons::SYNC_OFF, "Not connected".into(), offline)
    } else if !strip.unsent.is_empty() {
        alert(
            icons::WARN,
            "Changes kept in Splat, not sent".into(),
            c.result.warning,
        )
    } else if let Some(name) = &strip.sync_error {
        alert(icons::ERROR, format!("Sync error: {name}"), c.result.error)
    } else if party_sync_off {
        alert(icons::SYNC_OFF, "Sync off for the party".into(), c.muted)
    } else {
        None
    }
}

/// The turn heading's one fitted candidate (egui `turn_heading::fit`, ruling R22): the name's
/// words and size, and whether `Next turn` shows its key letter.
struct Fitted {
    name: String,
    size: Pixels,
    letter: bool,
}

/// Which words of the name a candidate shows.
#[derive(Clone, Copy, PartialEq)]
enum Words {
    /// `Iselle Marrowwind’s turn`, from the largest size down to the least.
    Turn,
    /// `Iselle Marrowwind`, from the largest size down to the least.
    Name,
    /// Whole words at the least size, then `…` (`Ember Cultist… 12`).
    Short,
    /// The first word's letters that fit, then `…` and the copy mark.
    Letters,
}

/// The gap between the heading's parts (egui `turn_heading::GAP`, `2 × S`).
const GAP: Pixels = px(16.0);

/// Choose the heading's candidate in R22's order: the suffix gives way first, then the key
/// letter, then the name's size, then its words. The letter may show only while every roster
/// name fits whole beside it at the least size, so it never comes and goes with the turn.
fn fit_turn(
    run: &Run,
    name: &str,
    width: Pixels,
    scale: theme::RunScale,
    add_words: bool,
    window: &Window,
) -> Fitted {
    let turn = px(scale.hp);
    let least = px((scale.name + f32::from(theme::HAIRLINE_WIDTH)).min(scale.hp));
    let keys_work = !run.lookup_open;
    let add_width = measure::button_width(
        window,
        if add_words { "Add to fight" } else { "Add" },
        true,
        None,
    );
    let next_width = |letter: bool| {
        measure::width(window, Type::Heading, "Next turn")
            + theme::M * 2.0
            + theme::ICON_SLOT
            + if letter {
                theme::S + measure::width(window, Type::Small, "N")
            } else {
                px(0.0)
            }
    };
    let round_width = measure::width_at(
        window,
        Type::Round,
        Some(turn),
        &format!("Round {}", run.state.round.max(1)),
    );
    let room = |letter: bool| {
        width - next_width(letter) - GAP - add_width - GAP - round_width - GAP
            - theme::HAIRLINE_WIDTH
    };
    let at = |size: Pixels, words: &str| measure::width_at(window, Type::Name, Some(size), words);
    // The largest size from `turn` down to `least` at which the whole text is inside `room`.
    let scaled = |text: &str, room: Pixels| -> Option<(String, Pixels)> {
        let mut size = turn;
        while size > least {
            if at(size, text) <= room {
                return Some((text.to_owned(), size));
            }
            size -= px(0.25);
        }
        (at(least, text) <= room).then(|| (text.to_owned(), least))
    };
    // The letter shows only while every roster name fits at the least size beside it; the
    // round counts as three of its widest digits, so 9 → 10 changes nothing.
    let roster_fits = {
        let widest = ('0'..='9')
            .map(|digit| {
                measure::width_at(window, Type::Round, Some(turn), &digit.to_string())
            })
            .fold(px(0.0), Pixels::max);
        let allowance =
            measure::width_at(window, Type::Round, Some(turn), "Round ") + widest * 3.0;
        let room = room(true) + round_width - round_width.max(allowance);
        run.state
            .participants
            .iter()
            .all(|p| at(least, &p.name) <= room)
    };
    let full = format!("{name}\u{2019}s turn");
    let order: [(Words, bool, Option<bool>); 6] = [
        (Words::Turn, true, Some(true)),
        (Words::Name, true, Some(true)),
        (Words::Turn, false, Some(false)),
        (Words::Name, false, None),
        (Words::Short, false, None),
        (Words::Letters, false, None),
    ];
    for (words, letter, keys) in order {
        if keys.is_some_and(|keys| keys != keys_work) || (letter && !roster_fits) {
            continue;
        }
        let room = room(letter);
        let chosen = match words {
            Words::Turn => scaled(&full, room),
            Words::Name => scaled(name, room),
            Words::Short => Some(crate::design::turn_heading::whole_words_name_by(
                name,
                f32::from(room),
                |words| f32::from(at(least, words)),
            ))
            .filter(|short| !short.starts_with('…') && at(least, short) <= room)
            .map(|short| (short, least)),
            Words::Letters => letters(name, |cut| at(least, cut) <= room).map(|cut| (cut, least)),
        };
        if let Some((name, size)) = chosen {
            return Fitted { name, size, letter };
        }
    }
    // No candidate fits: one letter and the copy mark, clipped to its room.
    Fitted {
        name: letters(name, |cut| cut.chars().take_while(|c| *c != '…').count() <= 1)
            .unwrap_or_default(),
        size: least,
        letter: false,
    }
}

/// The most letters of the name's first word that `fit` allows, then `…`, then the copy mark of
/// a numbered name (`Seraph… 12`). At least one letter (egui `turn_heading::letters`).
fn letters(name: &str, fit: impl Fn(&str) -> bool) -> Option<String> {
    let words: Vec<&str> = name.split_whitespace().collect();
    let first: Vec<char> = words.first().copied().unwrap_or(name).chars().collect();
    let mark = words
        .last()
        .filter(|last| {
            words.len() > 1
                && (last.chars().all(|c| c.is_ascii_digit()) || last.chars().count() == 1)
        })
        .map_or_else(String::new, |last| format!(" {last}"));
    (1..=first.len())
        .rev()
        .map(|count| format!("{}…{mark}", first[..count].iter().collect::<String>()))
        .find(|cut| fit(cut))
}

/// The header of the live view.
pub(super) fn live(
    run: &Run,
    width: gpui::Pixels,
    scale: theme::RunScale,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let nobody_present = !run.state.participants.iter().any(|p| p.present);
    let current = run
        .current()
        .and_then(|at| run.state.participants.get(at))
        .map(|p| p.name.clone());
    let turn_size = px(scale.hp);
    let round = div()
        .flex_none()
        .ty(Type::Round)
        .text_size(turn_size)
        .line_height(turn_size * 1.25)
        .text_color(c.text)
        .child(format!("Round {}", run.state.round));
    let add_words = f32::from(width) >= design_add_words_from();
    let fitted = current
        .as_ref()
        .map(|name| fit_turn(run, name, width, scale, add_words, window));
    let letter = fitted.as_ref().is_none_or(|fitted| fitted.letter) && !run.lookup_open;
    let name = current.zip(fitted).map(|(name, fitted)| {
        let whole = format!("{name}\u{2019}s turn");
        div()
            .id("run-turn-name")
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .whitespace_nowrap()
            .ty(Type::Name)
            .text_size(fitted.size)
            .text_color(c.turn.current)
            .child(fitted.name)
            .with_tooltip_words(whole)
    });
    let add = kit::Button::secondary(
        "run-add-to-fight",
        if add_words { "Add to fight" } else { "Add" },
    )
    .icon(icons::ADD)
    .name("Add to fight")
    .hint("Add an encounter group or a creature (Ctrl+G)")
    .density(Density::Read)
    .selected(run.panel == Panel::Groups && !run.hide_panel)
    .on_click(on_run(cx, |run| run.show_arrivals()));
    let next = kit::Button::primary("run-next-turn", "Next turn")
        .icon(icons::NEXT)
        .density(Density::Glance)
        .blocked(nobody_present.then_some(NOBODY_PRESENT))
        .on_click(on_command(cx, Command::Next));
    let next = if letter { next.key("N") } else { next };
    let first = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(GAP)
        .min_h(px((scale.hp * 1.6).max(36.0)))
        .child(round)
        .child(name.map_or_else(|| div().flex_1().into_any_element(), IntoElement::into_any_element))
        .child(add)
        .child(next);
    div()
        .flex()
        .flex_col()
        .flex_none()
        .child(first)
        .child(kit::gap(px(2.0)))
        .child(notice_line(run, width, window, cx))
        .child(kit::gap(px(2.0)))
        .child(kit::rule())
}

/// `ADD_WORDS_FROM`: under this width `Add to fight` says `Add`.
fn design_add_words_from() -> f32 {
    crate::design::turn_heading::ADD_WORDS_FROM
}

/// Hover words on an element with an id.
trait TooltipWords: kit::WithTooltip {
    fn with_tooltip_words(self, words: String) -> Self {
        self.with_tooltip(words)
    }
}
impl<T: kit::WithTooltip> TooltipWords for T {}

/// The notice line: on the left the most pressing notice, the turn's reminders and a sheet's
/// changes; on the right Undo.
fn notice_line(
    run: &Run,
    width: Pixels,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let strip = run.strip();
    let nobody_present = !run.state.participants.iter().any(|p| p.present);
    let alert = sync_alert(run);
    let needs_connection =
        strip.sync_stopped || (run.sync_needs_sign_in() && !run.party_sync_off());
    let mut left = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .flex_1()
        .min_w_0()
        .overflow_hidden();
    if let Some(notice) = run.recording_notice.filter(|_| strip.conflict.is_none()) {
        left = left.child(
            div()
                .id("run-recording-silence")
                .child(kit::status(&kit::Status::new(
                    icons::WARN,
                    "Recording silence",
                    c.result.warning,
                )))
                .with_tooltip_words(notice.to_owned()),
        );
        if alert.is_some() {
            left = left.child(kit::text(Type::Small, c.muted, "·"));
        }
    }
    let mut next_shown = false;
    if nobody_present && run.recording_notice.is_none() && strip.conflict.is_none() {
        left = left.child(
            div()
                .id("run-nobody-present")
                .min_w_0()
                .child(kit::status(&kit::Status::new(
                    icons::WARN,
                    "No participants are present",
                    c.result.warning,
                )))
                .with_tooltip_words(NOBODY_PRESENT.to_owned()),
        );
    } else if let Some(alert) = &alert {
        if strip.conflict.is_some() {
            let word = alert.words.clone();
            left = left.child(
                kit::status_link(
                    "run-review-conflict",
                    kit::Status::new(alert.icon, word.clone(), alert.color),
                )
                .hint(word)
                .on_click(on_run(cx, |run| run.review_conflict())),
            );
        } else {
            left = left.child(
                div()
                    .id("run-sync-alert")
                    .min_w_0()
                    .child(kit::status(&kit::Status::new(
                        alert.icon,
                        alert.words.clone(),
                        alert.color,
                    )))
                    .with_tooltip_words(alert.words.clone()),
            );
        }
        if needs_connection {
            left = left.child(kit::gap(px(0.0)).w(theme::XS)).child(
                kit::Button::quiet("run-connect-to-sync", "Connect to sync")
                    .plain()
                    .density(Density::Read)
                    .on_click(on_run(cx, |run| run.wants_sign_in = true)),
            );
        }
    } else if run.recording_notice.is_none()
        && let Some(next) = &strip.next
    {
        next_shown = true;
        left = left.child(kit::text::one_line(
            Type::Small,
            c.turn.next,
            format!("Next: {next}"),
        ));
    }
    let _ = next_shown;
    // A sheet's changes: a warning with `Log` and its close, on this line in the first of its
    // forms that fits; with no room, under the roster (`live::live_notice`).
    let sheet = sheet_in_header(run, width, window);
    let sheet_room = sheet
        .as_ref()
        .map_or(px(0.0), |form| sheet_width(window, form));
    if let Some(form) = sheet {
        let detail = run.notice.clone().unwrap_or_else(|| form.clone());
        left = left
            .child(kit::gap(px(0.0)).w(theme::S))
            .child(sheet_notice(form, detail, cx));
    }
    if let Some((words, _, full)) = run.turn_reminders() {
        left = left.child(kit::gap(px(0.0)).w(theme::S)).child(
            div()
                .id("run-turn-reminders")
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::XS)
                .min_w_0()
                .child(kit::icon(icons::PIN, Type::Small, c.result.info))
                .child(kit::text::one_line(Type::Small, c.text, words))
                .with_tooltip_words(full),
        );
    }
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::L)
        .h(Density::Read.control())
        .child(left)
        .child(undo(run, undo_room(run, width - sheet_room, window), window, cx))
}

/// The sheet-change notice's width on the notice line: its mark and words, `Log`, its close
/// and their gaps.
fn sheet_width(window: &Window, form: &str) -> Pixels {
    measure::width(window, Type::Small, &format!("{} {form}", icons::WARN))
        + measure::width(window, Type::Body, "Log")
        + Density::Read.control()
        + theme::L
        + theme::S * 3.0
}

/// The form of a sheet's changes that the notice line has room for, beside its notice and
/// Undo's shortest form (egui `live_header`'s `header_sheet_forms`); none when it shows under
/// the roster, or when there is no such notice.
pub(super) fn sheet_in_header(run: &Run, width: Pixels, window: &Window) -> Option<String> {
    let summary = run
        .notice_summary
        .as_ref()
        .filter(|_| run.notice.is_some() && run.recording_notice.is_none())?;
    let undo = undo_room(run, px(0.0), window).max(px(0.0));
    let label = run.undo_label();
    let shortest = run
        .undo_word_forms(&label)
        .into_iter()
        .next_back()
        .unwrap_or(label);
    let reserve = undo.max(undo_width(window, &shortest, false));
    let notice = width - undo_room(run, width, window) - theme::L;
    let room = width - notice - reserve - theme::L;
    crate::encounter_run::notice::header_sheet_forms(summary)
        .into_iter()
        .find(|form| sheet_width(window, form) <= room)
}

/// The room Undo has on the notice line (egui `live_header`'s slot): the line less its notice's
/// words and `L`. A state notice stays whole; only `Next: …` gives way, down to Undo's shortest
/// form with its target.
fn undo_room(run: &Run, width: Pixels, window: &Window) -> Pixels {
    let strip = run.strip();
    let nobody_present = !run.state.participants.iter().any(|p| p.present);
    let alert = sync_alert(run);
    let needs_connection =
        strip.sync_stopped || (run.sync_needs_sign_in() && !run.party_sync_off());
    let small = |words: &str| measure::width(window, Type::Small, words);
    let state = alert.is_some() || run.recording_notice.is_some() || nobody_present;
    let notice = if run.recording_notice.is_some() && strip.conflict.is_none() {
        small("Recording silence") + theme::ICON_SLOT
            + alert
                .as_ref()
                .map_or(px(0.0), |alert| small(&alert.words) + theme::L)
    } else if let Some(alert) = &alert {
        small(&alert.words)
            + theme::ICON_SLOT
            + if needs_connection {
                small("Connect to sync") + theme::XS + theme::S * 2.0
            } else {
                px(0.0)
            }
    } else if nobody_present {
        small("No participants are present") + theme::ICON_SLOT
    } else {
        strip
            .next
            .as_ref()
            .map_or(px(0.0), |next| small(&format!("Next: {next}")))
    };
    let notice = notice + theme::S;
    let room = width - notice - theme::L;
    if state {
        return room;
    }
    // `Next: …` gives way to Undo's shortest form that still names its target.
    let label = run.undo_label();
    let shortest = run
        .undo_word_forms(&label)
        .into_iter()
        .filter(|words| words.len() < label.len())
        .next_back()
        .unwrap_or(label);
    room.max(undo_width(window, &shortest, false))
}

/// The width of the header's Undo with these words (an icon, `U` while the keys work).
fn undo_width(window: &Window, words: &str, key: bool) -> Pixels {
    measure::button_width(window, words, true, key.then_some("U"))
}

/// The header's sheet notice: its words, `Log`, and its close (`header_notice`).
fn sheet_notice(summary: String, detail: String, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    div()
        .id("run-sheet-notice")
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .min_w_0()
        .pl(theme::S)
        .border_l(theme::MARK_WIDTH)
        .border_color(c.result.warning)
        .child(kit::icon(icons::WARN, Type::Small, c.result.warning))
        .child(kit::text::one_line(Type::Small, c.text, summary))
        .child(
            kit::Button::quiet("run-sheet-notice-log", "Log")
                .plain()
                .density(Density::Read)
                .on_click(on_run(cx, |run| {
                    run.panel = Panel::Log;
                    run.close_lookup = true;
                })),
        )
        .child(
            kit::IconButton::new("run-sheet-notice-dismiss", icons::CLOSE, "Dismiss").on_click(
                on_run(cx, |run| {
                    run.notice = None;
                    run.notice_summary = None;
                }),
            ),
        )
        .with_tooltip_words(detail)
}

/// Undo with the newest step's words, or `Nothing to undo` beside the blocked Undo. Words too
/// long for `room` give up the key letter first, then shorten by the one rule (`undo_forms`),
/// then say `Undo` alone; the whole label stays its name and hover.
fn undo(run: &Run, room: Pixels, window: &Window, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::S)
        .flex_none();
    if run.history.is_empty() {
        let full = run.empty_undo_reason().to_owned();
        return row
            .child(
                div()
                    .id("run-nothing-to-undo")
                    .child(kit::text::one_line(
                        Type::Small,
                        c.muted,
                        run.empty_undo_words().to_owned(),
                    ))
                    .with_tooltip_words(full),
            )
            .child(super::undo_button(
                run,
                "run-undo",
                "Undo".into(),
                Density::Read,
            ));
    }
    let label = run.undo_label();
    let keys = !run.lookup_open;
    let (words, key) = if undo_width(window, &label, keys) <= room {
        (label.clone(), keys)
    } else if undo_width(window, &label, false) <= room {
        (label.clone(), false)
    } else {
        (
            run.undo_word_forms(&label)
                .into_iter()
                .find(|words| undo_width(window, words, false) <= room)
                .unwrap_or_else(|| "Undo".to_owned()),
            false,
        )
    };
    let button = super::undo_button_keyed(run, "run-undo", words, Density::Read, key);
    row.child(button.on_click(on_run(cx, |run| {
        run.undo();
        run.follow_with_keyboard();
    })))
}
