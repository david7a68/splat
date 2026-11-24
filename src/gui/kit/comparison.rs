//! A draft beside its file (`design::draft_comparison_styled`): how many lines differ and under
//! which heading, `Next difference`, then the two texts side by side, aligned line by line
//! (`design::aligned`), each line that differs on the selected colour with the accent at its
//! left edge, and a hatched cell where one side has no line. It scrolls in its own column.
//!
//! ```ignore
//! kit::comparison::DraftComparison::new("encounter-disk", &disk.draft, &disk_text)
//! ```
use super::{Button, label};
use crate::gui::theme::{
    Density, HAIRLINE_WIDTH, L, S, SCROLL_BAR_WIDTH, Type, TypeExt, XS, colors,
};
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, ScrollHandle,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder as _,
};
use gpui_component::scroll::ScrollableElement as _;
use std::sync::Arc;

/// What the comparison keeps between frames: its texts' alignment (made again only when a text
/// changes), the difference it shows, and its scroll.
struct Kept {
    texts: u64,
    aligned: Arc<crate::design::Aligned>,
    current: usize,
    scroll: ScrollHandle,
}

/// The draft (left) beside the file now (right).
#[derive(IntoElement)]
pub struct DraftComparison {
    id: ElementId,
    draft: SharedString,
    disk: SharedString,
    headings: [&'static str; 2],
}

impl DraftComparison {
    pub fn new(
        id: impl Into<ElementId>,
        draft: impl Into<SharedString>,
        disk: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            draft: draft.into(),
            disk: disk.into(),
            headings: ["Your draft", "In the file now"],
        }
    }
}

fn texts_key(draft: &str, disk: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (draft, disk).hash(&mut hasher);
    hasher.finish()
}

impl RenderOnce for DraftComparison {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = colors();
        let key = texts_key(&self.draft, &self.disk);
        let (draft, disk) = (self.draft.clone(), self.disk.clone());
        let state = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), "comparison".into()),
            cx,
            move |_, _| {
                let aligned = Arc::new(crate::design::aligned([&draft, &disk]));
                Kept {
                    texts: key,
                    current: aligned.groups.first().copied().unwrap_or(0),
                    aligned,
                    scroll: ScrollHandle::new(),
                }
            },
        );
        let (draft, disk) = (self.draft.clone(), self.disk.clone());
        state.update(cx, |kept, _| {
            if kept.texts != key {
                kept.aligned = Arc::new(crate::design::aligned([&draft, &disk]));
                kept.texts = key;
                kept.current = kept.aligned.groups.first().copied().unwrap_or(0);
            }
        });
        let (aligned, current, scroll_handle) = {
            let kept = state.read(cx);
            (kept.aligned.clone(), kept.current, kept.scroll.clone())
        };
        let count = if aligned.groups.is_empty() {
            "No lines differ".to_owned()
        } else {
            crate::design::count(aligned.changed, "line differs", "lines differ")
        };
        // The heading the shown difference sits under, as the egui summary says it.
        let heading = aligned
            .rows
            .iter()
            .take(current + 1)
            .rev()
            .filter_map(|row| row[0].as_deref())
            .find(|line| line.trim_start().starts_with('#'))
            .map(|line| line.trim_start_matches('#').trim().to_owned());
        let summary = match heading {
            Some(heading) => format!("{count} · Under the heading “{heading}”"),
            None => count,
        };
        let mut line = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(S)
            .child(super::text(Type::Small, c.muted, summary));
        if aligned.groups.len() > 1 {
            let next_state = state.clone();
            line = line.child(
                Button::quiet(
                    ElementId::NamedChild(Arc::new(self.id.clone()), "next".into()),
                    "Next difference",
                )
                .density(Density::Read)
                .on_click(move |_, _, cx| {
                    next_state.update(cx, |kept, cx| {
                        let next = kept
                            .aligned
                            .groups
                            .iter()
                            .copied()
                            .find(|&row| row > kept.current)
                            .or_else(|| kept.aligned.groups.first().copied())
                            .unwrap_or(0);
                        kept.current = next;
                        // Two rows of context above the difference, as egui keeps.
                        kept.scroll.scroll_to_item(next.saturating_sub(2));
                        cx.notify();
                    });
                }),
            );
        }
        let rows = aligned.rows.iter().enumerate().map(|(at, row)| {
            let differs = row[0] != row[1];
            let cell = |side: usize| {
                let words = row[side].clone();
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_row()
                    .when(differs, |cell| cell.bg(c.selected))
                    .child(div().flex_none().w(XS).when(differs, |mark| {
                        mark.child(div().h_full().w(HAIRLINE_WIDTH).ml(XS / 2.0).bg(c.accent))
                    }))
                    .child(match words {
                        Some(words) => div()
                            .flex_1()
                            .min_w_0()
                            .pl(S)
                            .ty(Type::Mono)
                            .text_color(c.text)
                            .child(if words.is_empty() {
                                " ".to_owned()
                            } else {
                                words
                            }),
                        // The side has no line here: a quiet hatch, named for a reader.
                        None => super::target(
                            div()
                                .flex_1()
                                .min_w_0()
                                .ml(S)
                                .ty(Type::Mono)
                                .text_color(c.rule)
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .child("╱".repeat(80)),
                            "No line",
                        ),
                    })
            };
            div()
                .id(("comparison-row", at))
                .flex()
                .flex_row()
                .w_full()
                .gap(S)
                .child(cell(0))
                .child(cell(1))
        });
        div()
            .flex()
            .flex_col()
            .size_full()
            .min_h_0()
            .gap(S)
            .child(line)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .gap(S)
                    .child(div().flex_1().child(label(self.headings[0], 0)))
                    .child(div().flex_1().child(label(self.headings[1], 0))),
            )
            .child(
                // The rows are the scrolled element's own children, so `Next difference` scrolls
                // to one by its place; the column keeps the bar's lane.
                div().flex_1().min_h(gpui::px(120.0)).pb(L).child(
                    div()
                        .id(ElementId::NamedChild(Arc::new(self.id), "rows".into()))
                        .size_full()
                        .min_h_0()
                        .flex()
                        .flex_col()
                        .pr(SCROLL_BAR_WIDTH)
                        .overflow_y_scroll()
                        .track_scroll(&scroll_handle)
                        .children(rows)
                        .vertical_scrollbar(&scroll_handle),
                ),
            )
    }
}
