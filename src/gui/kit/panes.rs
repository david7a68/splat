//! Panes and the columns between them (`design::pane`, `design::Column`).
//!
//! - `Pane`: one task's column: an optional header that stays, a body that scrolls (with the bar's
//!   lane kept), and an optional footer under the scroll for the pane's results, reasons and
//!   notices (`kit::Message`s), so a message never moves the form the DM works in.
//! - `Columns`: panes side by side with a drag handle (splitter) between them. Each fixed column
//!   has a default width and a range; the last column takes the rest. The widths live in
//!   gpui-component's `ResizableState`, kept under the columns' id. A column `saved` under its
//!   id keeps the DM's width after a restart (`kit::edge::ColumnWidths`, `ui_state`).
//!
//! Keep targets clear of a splitter: a pane's content has `M` padding at its sides.
use crate::gui::theme::{HAIRLINE_WIDTH, M, S, colors};
use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, ScrollHandle,
    SharedString, Styled, Window, div, prelude::*,
};
use gpui_component::{ResizableState, h_resizable, resizable_panel};
use std::ops::Range;

/// One pane: a header, a scrolling body and a footer.
#[derive(IntoElement)]
pub struct Pane {
    id: ElementId,
    header: Option<AnyElement>,
    body: Vec<AnyElement>,
    footer: Vec<AnyElement>,
    scrolls: bool,
    padded: bool,
    handle: Option<ScrollHandle>,
}

impl Pane {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            header: None,
            body: Vec::new(),
            footer: Vec::new(),
            scrolls: true,
            padded: true,
            handle: None,
        }
    }

    /// The body scrolls by a handle the page keeps: a reader's typed jump to a heading
    /// (`kit::Markdown::jump_in`) moves it.
    pub fn scroll(mut self, handle: &ScrollHandle) -> Self {
        self.handle = Some(handle.clone());
        self
    }

    /// What stays above the scroll (a title line, a search field).
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// The pane's messages, under its scroll (`kit::Message`).
    pub fn footer(mut self, message: impl IntoElement) -> Self {
        self.footer.push(message.into_any_element());
        self
    }

    /// A body that does not scroll (it fills the pane and scrolls inside itself).
    pub fn fixed(mut self) -> Self {
        self.scrolls = false;
        self
    }

    /// No `M` padding at the sides (a pane whose rows reach its edges).
    pub fn flush(mut self) -> Self {
        self.padded = false;
        self
    }
}

impl ParentElement for Pane {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}

impl RenderOnce for Pane {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let body = div().flex().flex_col().w_full().children(self.body);
        let body_id = ElementId::NamedChild(
            std::sync::Arc::new(self.id.clone()),
            SharedString::new_static("body"),
        );
        // The room a menu in the body keeps inside: the body above the footer, clear of the
        // scroll bar's lane (`kit::layer`).
        let lane = if self.scrolls {
            crate::gui::theme::SCROLL_BAR_WIDTH
        } else {
            gpui::px(0.0)
        };
        let body: AnyElement = if self.scrolls {
            match &self.handle {
                Some(handle) => {
                    super::scroll::scroll_column_with(body_id, handle, body).into_any_element()
                }
                None => super::scroll::scroll_column(body_id, body).into_any_element(),
            }
        } else {
            body.size_full().into_any_element()
        };
        div()
            .flex()
            .flex_col()
            .size_full()
            .min_h_0()
            .when(self.padded, |pane| pane.px(M))
            .when_some(self.header, |pane, header| pane.child(header))
            .child(
                div()
                    .relative()
                    .flex_1()
                    .min_h_0()
                    .child(super::layer::room_start(lane))
                    .child(body)
                    .child(super::layer::room_end()),
            )
            .when(!self.footer.is_empty(), |pane| {
                pane.child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(S)
                        .pt(S)
                        .pb(S)
                        .border_t(HAIRLINE_WIDTH)
                        .border_color(colors().hairline)
                        .children(self.footer),
                )
            })
    }
}

/// One column of `Columns`.
struct Column {
    key: Option<&'static str>,
    width: Option<Pixels>,
    range: Option<Range<Pixels>>,
    content: AnyElement,
}

/// Columns side by side with splitters (`design::PaneColumns`, `Column::left`).
///
/// ```ignore
/// kit::Columns::new("library-columns")
///     .column(px(320.0), px(240.0)..px(480.0), list_pane)
///     .rest(reader_pane)
/// ```
#[derive(IntoElement)]
pub struct Columns {
    id: ElementId,
    columns: Vec<Column>,
}

impl Columns {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            columns: Vec::new(),
        }
    }

    /// A column of a default width that the DM can drag within `range`.
    pub fn column(
        mut self,
        width: Pixels,
        range: Range<Pixels>,
        content: impl IntoElement,
    ) -> Self {
        self.columns.push(Column {
            key: None,
            width: Some(width),
            range: Some(range),
            content: content.into_any_element(),
        });
        self
    }

    /// Keep the last column's width, as the DM drags it, under `key` (the column id:
    /// `library_sections`, `world_list_panel`, …), here and after a restart.
    pub fn saved(mut self, key: &'static str) -> Self {
        if let Some(column) = self.columns.last_mut() {
            column.key = Some(key);
        }
        self
    }

    /// The column that takes the room the others leave.
    pub fn rest(mut self, content: impl IntoElement) -> Self {
        self.columns.push(Column {
            key: None,
            width: None,
            range: None,
            content: content.into_any_element(),
        });
        self
    }
}

impl RenderOnce for Columns {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(
            ElementId::NamedChild(
                std::sync::Arc::new(self.id.clone()),
                SharedString::new_static("widths"),
            ),
            cx,
            |_, _| ResizableState::default(),
        );
        let keys: Vec<Option<&'static str>> = self.columns.iter().map(|c| c.key).collect();
        let mut group = h_resizable(self.id)
            .with_state(&state)
            .on_resize(move |state, _, cx| {
                let sizes = state.read(cx).sizes().clone();
                for (key, size) in keys.iter().zip(sizes) {
                    if let Some(key) = key {
                        super::edge::set_column_width(key, Some(f32::from(size).round()), cx);
                    }
                }
            });
        for column in self.columns {
            // A saved column is a harness target by its key (`column world_list_panel`).
            let content = match column.key {
                Some(key) => super::target(
                    div().size_full().child(column.content),
                    format!("column {key}"),
                )
                .into_any_element(),
                None => column.content,
            };
            let mut panel = resizable_panel().child(content);
            let saved = column
                .key
                .and_then(|key| super::edge::column_width(key, cx))
                .map(gpui::px);
            if let Some(width) = saved.or(column.width) {
                let width = match &column.range {
                    Some(range) => width.clamp(range.start, range.end),
                    None => width,
                };
                panel = panel.size(width);
            }
            if let Some(range) = column.range {
                panel = panel.size_range(range);
            }
            group = group.child(panel);
        }
        group
    }
}
