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
use theme::{Density, Type, colors};

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
    let arrivals = super::arrivals_prepare(root);
    // A changed window size must not leave the selected row out of view (egui `roster`).
    let scroll = root
        .app
        .encounters
        .run_mut()
        .is_some_and(|run| std::mem::take(&mut run.scroll_to_selected));
    let resized = root.pages.run.roster_room != Some(height);
    root.pages.run.roster_room = Some(height);
    let Some(run) = root.app.encounters.run() else {
        return div().into_any_element();
    };
    let hide_panel = run.hide_panel;
    let roster_width = if hide_panel {
        (room - theme::M).min(px(theme::RUN_LARGE.max_width))
    } else {
        room - side - theme::M
    };
    // The roster's box as it was laid out last frame; before the first, an estimate.
    let measured = root.pages.run.roster_box.get();
    let roster_room = if measured > 1.0 {
        measured
    } else {
        f32::from(height)
            - (theme::RUN_REGULAR.hp * 1.6).max(36.0)
            - 32.0
            - view::DOCK_HEIGHT
            - FOOTER_HEIGHT
            - if hide_panel {
                crate::rolls::BAR_BUTTONS + 4.0
            } else {
                0.0
            }
    };
    let roster_box = root.pages.run.roster_box.clone();
    let current = run.current();
    let next_up = run.next_up();
    let order = run.state.order.clone();
    let plan = roster::plan(
        run,
        &order,
        f32::from(roster_width - theme::SCROLL_BAR_WIDTH),
        roster_room,
        window,
    );
    let scale = plan.scale;
    let rows: Vec<AnyElement> = order
        .iter()
        .enumerate()
        .map(|(at, &index)| {
            roster::row(
                root,
                run,
                index,
                roster::TurnMark::of(index, current, next_up),
                &plan,
                at,
                roster::Use::Live,
                cx,
            )
        })
        .collect();
    let handle = root.pages.run.roster_scroll.clone();
    let spans = plan.spans(&order);
    let list_height = if plan.overflow {
        plan.room - view::OVERFLOW_LINE
    } else {
        plan.room
    };
    if (scroll || resized)
        && let Some(&(top, bottom, _)) = spans.iter().find(|(.., index)| *index == run.selected)
    {
        // The next row too when it follows, so the DM sees who is coming; past a margin at
        // each end, so neither row is cut.
        let shown = f32::from(handle.bounds().size.height);
        let shown = if shown > 1.0 { shown } else { list_height };
        let bottom = next_up
            .and_then(|next| spans.iter().find(|(.., index)| *index == next))
            .filter(|(next_top, next_bottom, _)| {
                *next_top >= top && next_bottom - top + 8.0 < shown
            })
            .map_or(bottom, |(_, next_bottom, _)| *next_bottom);
        let offset = -f32::from(handle.offset().y);
        let wanted = if top - 4.0 < offset {
            (top - 4.0).max(0.0)
        } else if bottom + 4.0 > offset + shown {
            bottom + 4.0 - shown
        } else {
            offset
        };
        if (wanted - offset).abs() > 0.5 {
            handle.set_offset(gpui::point(px(0.0), px(-wanted)));
        }
    }
    let overflow_line = overflow_note(run, &handle, &spans, list_height);
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
                .child(
                    // The rows' box: its height is the room the next frame fits them to.
                    div()
                        .relative()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_h_0()
                        .child(
                            gpui::canvas(
                                move |bounds, window, _| {
                                    let height = f32::from(bounds.size.height);
                                    if (roster_box.get() - height).abs() > 0.5 {
                                        roster_box.set(height);
                                        window.on_next_frame(|window, _| window.refresh());
                                    }
                                },
                                |_, _, _, _| {},
                            )
                            .absolute()
                            .size_full(),
                        )
                        .child(
                            panel::group_drop_zone(
                                div().flex().flex_col().flex_1().min_h_0(),
                                "run-roster-drop",
                                cx,
                            )
                            .child(kit::scroll::scroll_column_with(
                                "run-roster",
                                &handle,
                                div().flex().flex_col().w_full().children(rows),
                            )),
                        )
                        .children(overflow_line),
                )
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
            .child(panel::render(root, run, side, arrivals, window, cx));
    }
    page.into_any_element()
}

/// In a long roster, what is out of view, on which side, and what needs attention there
/// (`overflow_note`): a row cut by the edge counts as out of view.
fn overflow_note(
    run: &crate::encounter_run::Run,
    handle: &gpui::ScrollHandle,
    spans: &[(f32, f32, usize)],
    list_height: f32,
) -> Option<AnyElement> {
    let c = colors();
    let shown = f32::from(handle.bounds().size.height);
    let top = -f32::from(handle.offset().y);
    let bottom = top + if shown > 1.0 { shown } else { list_height };
    let above: Vec<usize> = spans
        .iter()
        .filter(|(start, ..)| *start < top - 1.0)
        .map(|(.., index)| *index)
        .collect();
    let below: Vec<usize> = spans
        .iter()
        .filter(|(_, end, _)| *end > bottom + 1.0)
        .map(|(.., index)| *index)
        .collect();
    let words = run.overflow_words(&above, &below)?;
    Some(
        div()
            .id("run-roster-overflow")
            .flex()
            .items_center()
            .flex_none()
            .h(px(view::OVERFLOW_LINE))
            .w_full()
            .bg(c.page)
            .border_t(theme::HAIRLINE_WIDTH)
            .border_color(c.hairline)
            .child(kit::text::one_line(Type::Small, c.muted, words))
            .into_any_element(),
    )
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
