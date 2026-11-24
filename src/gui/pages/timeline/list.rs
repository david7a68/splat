//! The Timeline's list of events (egui: `TimelinePage::list`, `row`): one row for each event that
//! passes the filters, in story order, in a virtual list, so a long timeline costs a frame no more
//! than a short one. Each row has the spine and the kind's icon, the title (a day's in capitals),
//! and its second line in `Small` with each status part in its own colour; an open fight's result
//! is a link back to its run.
use crate::gui::theme::{HAIRLINE_WIDTH, S, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use crate::timeline::page::{RowTint, RowWords, long_rest_status, short_session};
use crate::timeline::{Kind, kind_icon};
use gpui::{
    AnyElement, Context, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    ScrollStrategy, SharedString, StatefulInteractiveElement, Styled, StyledText, TextRun, Window,
    div, prelude::*, px,
};

/// A row's height (egui's `ROW`), and the gap under it.
const ROW: gpui::Pixels = px(44.0);
const ROW_GAP: gpui::Pixels = px(6.0);
/// The row's icon column: the spine sits at its centre, `SPINE` from the row's left.
const ICON_COLUMN: gpui::Pixels = px(16.0);

/// The list, or `No matching events` when the filters hide every event.
pub(crate) fn render(
    root: &mut SplatRoot,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let view = root.app.timeline_page.view(&root.app.timeline);
    if view.rows.is_empty() {
        // An unreadable file has its own message, and no filter caused it.
        if root.app.timeline.unreadable.is_some() {
            return div().into_any_element();
        }
        return kit::empty_state(
            icons::TIMELINE,
            "No matching events",
            "Clear the search and filters to see the whole timeline.",
            Some(
                kit::Link::new("timeline-show-all", "Show all events").on_click(cx.listener(
                    |root, _, _, cx| {
                        root.app.timeline_page.show_all();
                        cx.notify();
                    },
                )),
            ),
        )
        .into_any_element();
    }
    // The row before the target comes to the top, as the list opened before.
    if let Some(target) = root.app.timeline_page.scroll_to.take()
        && let Some(row) = view.rows.iter().position(|&index| index == target)
    {
        root.pages
            .timeline
            .list
            .scroll_to_item(row.saturating_sub(1), ScrollStrategy::Top);
    }
    let count = view.rows.len();
    kit::uniform_column(
        "timeline-list",
        &root.pages.timeline.list,
        count,
        cx.processor(|root, range: std::ops::Range<usize>, _window, cx| {
            let view = root.app.timeline_page.view(&root.app.timeline);
            let sessions = root.app.sessions.links();
            range
                .filter_map(|row| {
                    let index = *view.rows.get(row)?;
                    let event = root.app.timeline.events.get(index)?;
                    let session = event
                        .session
                        .and_then(|id| sessions.iter().find(|session| session.id == id))
                        .map(|session| short_session(&session.label));
                    let words = root.app.timeline_page.row_words(
                        event,
                        view.days[index],
                        session,
                        view.flagged.get(&event.id).copied(),
                        view.road.get(&event.id).map(String::as_str),
                    );
                    let selected =
                        root.app.timeline_page.selected.as_deref() == Some(event.id.as_str());
                    Some(row_element(
                        event.id.clone(),
                        event.kind,
                        words,
                        selected,
                        cx,
                    ))
                })
                .collect::<Vec<_>>()
        }),
    )
    .into_any_element()
}

/// The second line's words from byte `from`, muted, with each status part in its own colour.
fn second_line(words: &RowWords, from: usize) -> StyledText {
    let c = colors();
    let detail = words.detail.replace(['\n', '\r'], " ");
    let mut runs = Vec::new();
    let mut at = from.min(detail.len());
    let run = |len: usize, color: Hsla| TextRun {
        len,
        font: gpui::Font {
            family: SharedString::new_static(Type::Small.family()),
            features: Default::default(),
            fallbacks: None,
            weight: Type::Small.weight(),
            style: Type::Small.style(),
        },
        color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    for (range, tint) in &words.spans {
        let (start, end) = (
            range.start.max(at).min(detail.len()),
            range.end.min(detail.len()),
        );
        if start >= end {
            continue;
        }
        if start > at {
            runs.push(run(start - at, c.muted));
        }
        let color = match tint {
            RowTint::Warning => c.result.warning,
            RowTint::Ruling(restores) => {
                crate::gui::bridge::color(long_rest_status(*restores).color)
            }
        };
        runs.push(run(end - start, color));
        at = end;
    }
    if detail.len() > at {
        runs.push(run(detail.len() - at, c.muted));
    }
    StyledText::new(SharedString::from(
        detail[from.min(detail.len())..].to_owned(),
    ))
    .with_runs(runs)
}

/// One row: the spine and its kind's icon, the title (and a day heading's flag), the second
/// line; a press opens the event.
fn row_element(
    id: String,
    kind: Kind,
    words: RowWords,
    selected: bool,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let day = kind == Kind::Day;
    let spine = |grow: bool| {
        div()
            .w(HAIRLINE_WIDTH)
            .when(grow, |line| line.flex_1())
            .when(!day, |line| line.bg(c.rule))
    };
    let icon = div()
        .flex()
        .flex_col()
        .items_center()
        .w(ICON_COLUMN)
        .h_full()
        .flex_none()
        .child(spine(true))
        .child(kit::icon(
            kind_icon(kind),
            Type::Name,
            if day { c.text } else { c.muted },
        ))
        .child(spine(true));
    let title = if day {
        kit::label(words.title.clone(), 0)
    } else {
        div().child(kit::text::one_line(
            if selected { Type::Strong } else { Type::Body },
            c.text,
            words.title.clone(),
        ))
    };
    let mut first = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_w_0()
        .child(title.min_w_0());
    if let Some(flag) = &words.flag {
        first = first.child(
            div()
                .flex()
                .flex_row()
                .flex_none()
                .gap(XS)
                .child(kit::icon(icons::WARN, Type::Small, c.result.warning))
                .child(kit::text(Type::Small, c.muted, flag.clone())),
        );
    }
    let mut text = div().flex().flex_col().flex_1().min_w_0().child(first);
    if !words.detail.is_empty() {
        if words.open && !words.summary.is_empty() {
            // The open fight's result is a link back to its run; the words around it stay.
            let (before, _) = words.detail.split_once(&words.summary).unwrap_or(("", ""));
            let from = before.len() + words.summary.len();
            text = text.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .min_w_0()
                    .ty(Type::Small)
                    .whitespace_nowrap()
                    .overflow_hidden()
                    .child(kit::text(Type::Small, c.muted, before.to_owned()))
                    .child(
                        kit::status_link(
                            ElementId::Name(format!("timeline-run-{id}").into()),
                            kit::Status::new(icons::RUN, words.summary.clone(), c.text),
                        )
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.timeline_page.wants_back_to_run = true;
                            cx.notify();
                        })),
                    )
                    .child(second_line(&words, from)),
            );
        } else {
            text = text.child(
                div()
                    .ty(Type::Small)
                    .whitespace_nowrap()
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(second_line(&words, 0)),
            );
        }
    }
    let open = id.clone();
    let row = kit::Row::new(
        ElementId::Name(format!("timeline-row-{id}").into()),
        words.name.clone(),
    )
    .height(ROW)
    .selected(selected)
    .on_click(cx.listener(move |root, _, _, cx| {
        let page = &mut root.app.timeline_page;
        if page.selected.as_ref() != Some(&open) {
            page.reset_detail = true;
            root.pages.timeline.form_top = true;
        }
        page.select(Some(open.clone()));
        page.focus_title = false;
        cx.notify();
    }))
    .child(icon)
    .child(text);
    let row = div().h(ROW + ROW_GAP).pb(ROW_GAP).child(row);
    // A length the clock cannot count says why on hover, and how to write one (egui `row`).
    if words.warning {
        row.id(ElementId::Name(format!("timeline-row-hover-{id}").into()))
            .tooltip(kit::tooltip::tip(
                format!(
                    "{}. {}",
                    crate::timeline::TIME_NOT_COUNTED,
                    crate::timeline::TIME_REPAIR
                )
                .into(),
            ))
            .into_any_element()
    } else {
        row.into_any_element()
    }
}
