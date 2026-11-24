//! List rows (`design::row`, `two_line_row`): a fixed-height row with a hover fill, the selected
//! fill when chosen, and a focus ring. Use `Row` for any list whose rows are chosen by a press
//! (Library entries, sessions, events); put the row's facts and actions in its `child`ren. A row's
//! actions go in one column `L` after its widest facts (`docs/design-system.md`, Action rows).
use super::{OnClick, button::element_name, target};
use crate::gui::theme::{Density, FOCUS_WIDTH, RADIUS_CONTROL, S, Type, TypeExt, colors};
use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*,
};
use std::rc::Rc;

/// A row of a list. Its name is its accessible name and the harness's name for it.
#[derive(IntoElement)]
pub struct Row {
    id: ElementId,
    name: SharedString,
    height: Pixels,
    selected: bool,
    grows: bool,
    children: Vec<AnyElement>,
    on_click: Option<OnClick>,
}

impl Row {
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            height: Density::Scan.row(),
            selected: false,
            grows: false,
            children: Vec::new(),
            on_click: None,
        }
    }

    /// Its height (default a `Scan` row). Use `Density::row()` or a run scale's row.
    pub fn height(mut self, height: Pixels) -> Self {
        self.height = height;
        self
    }

    /// Its height is the least it takes: words that wrap make it taller (a path and a cause
    /// that must stay whole).
    pub fn grows(mut self) -> Self {
        self.grows = true;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Row {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Row {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let mut row = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .when(self.grows, |row| row.min_h(self.height))
            .when(!self.grows, |row| row.h(self.height))
            .flex_none()
            .px(S)
            .gap(S)
            .rounded(RADIUS_CONTROL)
            .when(self.selected, |row| row.bg(c.selected))
            .when(!self.selected, |row| row.hover(|s| s.bg(c.hover)))
            .focusable()
            .tab_stop(true)
            .role(gpui::Role::Button)
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.name.clone())
            .aria_selected(self.selected)
            .children(self.children);
        if let Some(on_click) = self.on_click {
            row = row
                .cursor_pointer()
                .on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let row = target(row, self.name.clone());
        target(row, element_name(&self.id))
    }
}

/// A row of a title (`Strong` when chosen) over a muted `Small` detail, both cut to one line.
pub fn two_line_row(
    id: impl Into<ElementId>,
    title: impl Into<SharedString>,
    detail: impl Into<SharedString>,
    selected: bool,
) -> Row {
    let c = colors();
    let (title, detail) = (title.into(), detail.into());
    Row::new(id, format!("{title} · {detail}"))
        .height(Density::Glance.row())
        .selected(selected)
        .child(
            div()
                .flex()
                .flex_col()
                .min_w_0()
                .flex_1()
                .child(
                    div()
                        .ty(if selected { Type::Strong } else { Type::Body })
                        .text_color(c.text)
                        .truncate()
                        .child(title),
                )
                .child(
                    div()
                        .ty(Type::Small)
                        .text_color(c.muted)
                        .truncate()
                        .child(detail),
                ),
        )
}
