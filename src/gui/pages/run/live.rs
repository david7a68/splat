//! The Running phase (egui `view::live_view`): the header, the roster with its notices and
//! message footer, the action line while the panel is hidden, the dock and the footer, with the
//! run's panel beside them.
use super::{Side, dock, footer, header, on_run, panel, roster};
use crate::encounter_run::{Panel, view};
use crate::gui::{SplatRoot, icons, kit, theme};
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, ParentElement, Styled, Window, div,
    prelude::*, px,
};
use theme::{Density, RunScale, Type, colors};

/// The footer's height (egui `FOOTER_HEIGHT`).
pub(super) const FOOTER_HEIGHT: f32 = 28.0 + 7.0;

/// The Running phase.
pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let room = root
        .pages
        .run
        .width
        .unwrap_or_else(|| super::page_width(root, window));
    let window_width = window.viewport_size().width;
    let height = window.viewport_size().height - theme::S * 2.0;
    let side = super::side_width(root, Side::Live, room, window_width);
    let Some(run) = root.app.encounters.run() else {
        return div().into_any_element();
    };
    let hide_panel = run.hide_panel;
    let roster_width = if hide_panel {
        (room - theme::M).min(px(theme::RUN_LARGE.max_width))
    } else {
        room - side - theme::M
    };
    let roster_room = f32::from(height)
        - (theme::RUN_REGULAR.hp * 1.6).max(36.0)
        - 32.0
        - view::DOCK_HEIGHT
        - FOOTER_HEIGHT;
    let scale = RunScale::fit(roster_room, f32::from(roster_width), run.state.order.len());
    let current = run.current();
    let next_up = run.next_up();
    let rows: Vec<AnyElement> = run
        .state
        .order
        .clone()
        .into_iter()
        .map(|index| {
            roster::row(
                root,
                run,
                index,
                roster::TurnMark::of(index, current, next_up),
                scale,
                roster::Use::Live,
                cx,
            )
        })
        .collect();
    let notice = live_notice(run, cx);
    let messages = footer::messages(run, false, cx);
    let compact = hide_panel.then(|| panel::compact_actions(run, cx));
    let main = div()
        .flex()
        .flex_col()
        .h_full()
        .w(roster_width)
        .flex_none()
        .child(header::live(run, roster_width, scale, window, cx))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_h_0()
                .child(kit::scroll_column(
                    "run-roster",
                    div().flex().flex_col().w_full().children(rows),
                ))
                .children(notice)
                .when(!messages.is_empty(), |body| {
                    body.child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_none()
                            .gap(theme::S)
                            .pt(theme::S)
                            .children(messages),
                    )
                }),
        )
        .children(compact)
        .child(dock::render(run, roster_width, cx))
        .child(footer::live(run, cx));
    let mut page = div()
        .flex()
        .flex_row()
        .size_full()
        .on_drag_move(super::on_panel_drag(cx))
        .child(main);
    if !hide_panel {
        page = page
            .child(super::splitter(Side::Live, cx))
            .child(panel::render(root, run, side, window, cx));
    }
    page.into_any_element()
}

/// A notice under the roster that sync or the run left (the header shows a sheet's summary):
/// its words, `Log` and its close (`live_notice`).
fn live_notice(run: &crate::encounter_run::Run, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let c = colors();
    let notice = run.notice.clone()?;
    if run.notice_summary.is_some() && run.recording_notice.is_none() {
        return None;
    }
    Some(
        div()
            .id("run-live-notice")
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::XS)
            .flex_none()
            .min_h(Density::Read.control())
            .mt(theme::S)
            .pl(theme::S)
            .border_l(theme::MARK_WIDTH)
            .border_color(c.result.warning)
            .child(kit::icon(icons::WARN, Type::Small, c.result.warning))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(kit::text(Type::Small, c.text, notice)),
            )
            .child(
                kit::Button::quiet("run-live-notice-log", "Log")
                    .plain()
                    .density(Density::Read)
                    .on_click(on_run(cx, |run| {
                        run.panel = Panel::Log;
                        run.close_lookup = true;
                    })),
            )
            .child(
                kit::IconButton::new("run-live-notice-dismiss", icons::CLOSE, "Dismiss").on_click(
                    on_run(cx, |run| {
                        run.notice = None;
                        run.return_focus = run.lookup_open;
                    }),
                ),
            )
            .into_any_element(),
    )
}
