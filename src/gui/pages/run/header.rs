//! The live run's header (egui `notice::live_header`, `design::TurnHeading`): the round and whose
//! turn it is, `Add to fight` and `Next turn`; under them the notice line (what sync needs, the
//! next turn, the turn's reminders, a sheet's changes) and Undo.
use super::{on_command, on_run};
use crate::encounter_run::{Command, NOBODY_PRESENT, Panel, Run};
use crate::gui::{SplatRoot, bridge, icons, kit, theme};
use gpui::{Context, InteractiveElement, IntoElement, ParentElement, Styled, div, px};
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

/// The header of the live view.
pub(super) fn live(
    run: &Run,
    width: gpui::Pixels,
    scale: theme::RunScale,
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
    let name = current.map(|name| {
        let whole = format!("{name}\u{2019}s turn");
        div()
            .id("run-turn-name")
            .min_w_0()
            .ty(Type::Name)
            .text_size(px(scale.name))
            .text_color(c.turn.current)
            .truncate()
            .child(whole.clone())
            .with_tooltip_words(whole)
    });
    let add = kit::Button::secondary(
        "run-add-to-fight",
        if f32::from(width) < design_add_words_from() {
            "Add"
        } else {
            "Add to fight"
        },
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
    let next = if run.lookup_open { next } else { next.key("N") };
    let first = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::S)
        .min_h(px((scale.hp * 1.6).max(36.0)))
        .child(round)
        .children(name)
        .child(div().flex_1())
        .child(add)
        .child(next);
    div()
        .flex()
        .flex_col()
        .flex_none()
        .child(first)
        .child(kit::gap(px(2.0)))
        .child(notice_line(run, cx))
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
fn notice_line(run: &Run, cx: &mut Context<SplatRoot>) -> impl IntoElement {
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
    // A sheet's changes: a warning with `Log` and its close, on this line.
    if let Some(summary) = run
        .notice_summary
        .clone()
        .filter(|_| run.notice.is_some() && run.recording_notice.is_none())
    {
        let detail = run.notice.clone().unwrap_or_else(|| summary.clone());
        left = left
            .child(kit::gap(px(0.0)).w(theme::S))
            .child(sheet_notice(summary, detail, cx));
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
        .child(undo(run, cx))
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

/// Undo with the newest step's words, or `Nothing to undo` beside the blocked Undo.
fn undo(run: &Run, cx: &mut Context<SplatRoot>) -> impl IntoElement {
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
    row.child(
        super::undo_button(run, "run-undo", label, Density::Read).on_click(on_run(cx, |run| {
            run.undo();
            run.follow_with_keyboard();
        })),
    )
}
