//! A modal dialog (`design::dialog`, `dialog_actions`), for the brief's modal tasks only (closing
//! with unsaved work, the copy dialog, a confirm that loses data).
//!
//! Its look: a raised layer `XXL` from the window's top, `FORM_WIDTH` wide at most, a dim backdrop
//! that takes every press, the title in `Section` type, the body, then one action line: the
//! destructive action at the left, then at least `L`, then the safe action and the commit at the
//! right, commit last. The dialog takes the keyboard when it opens; Escape is the safe action
//! (`on_escape`). Draw it as the last child of the page or the root while it is open; it covers
//! the window wherever it is drawn.
use crate::gui::theme::{
    DIALOG_INSET, FORM_WIDTH, HAIRLINE_WIDTH, L, RADIUS_LAYER, S, Type, XS, colors,
};
use gpui::{
    AnyElement, App, ElementId, FocusHandle, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, anchored, deferred, div,
    point, prelude::*, px,
};
use std::rc::Rc;

/// An action on a dialog's line (a kit button).
pub type DialogAction = super::Button;

/// `Dialog::new("unsaved-on-close", "Unsaved changes").child(...).destructive(...).action(keep)
///     .action(commit).on_escape(...)`.
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    title: SharedString,
    body: Vec<AnyElement>,
    destructive: Option<DialogAction>,
    actions: Vec<DialogAction>,
    on_escape: Option<Rc<dyn Fn(&mut Window, &mut App)>>,
}

impl Dialog {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            body: Vec::new(),
            destructive: None,
            actions: Vec::new(),
            on_escape: None,
        }
    }

    /// The action that loses data, at the line's left.
    pub fn destructive(mut self, action: DialogAction) -> Self {
        self.destructive = Some(action);
        self
    }

    /// An action at the line's right, in reading order: the safe action first, the commit last.
    pub fn action(mut self, action: DialogAction) -> Self {
        self.actions.push(action);
        self
    }

    /// What Escape does: the safe action's handler.
    pub fn on_escape(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_escape = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}

/// The dialog's kept keyboard focus, and whether it has taken the keyboard yet.
struct Keyboard {
    focus: FocusHandle,
    taken: bool,
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = colors();
        let keyboard = window.use_keyed_state(self.id.clone(), cx, |_, cx| Keyboard {
            focus: cx.focus_handle(),
            taken: false,
        });
        let focus = keyboard.read(cx).focus.clone();
        if !keyboard.read(cx).taken {
            keyboard.update(cx, |keyboard, _| keyboard.taken = true);
            window.focus(&focus, cx);
        }
        let viewport = window.viewport_size();
        let width = (viewport.width - DIALOG_INSET * 2.0).min(FORM_WIDTH);
        let escape = self.on_escape.clone();
        let line = div()
            .flex()
            .flex_row()
            .items_center()
            .pt(S)
            .when_some(self.destructive, |line, action| {
                line.child(action).child(div().w(L).flex_none())
            })
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .justify_end()
                    .items_center()
                    .gap(S)
                    .children(self.actions),
            );
        let frame = div()
            .id(self.id.clone())
            .track_focus(&focus)
            .flex()
            .flex_col()
            .gap(XS)
            .w(width)
            .max_h(viewport.height - DIALOG_INSET * 2.0)
            .p(L)
            .bg(c.raised)
            .border(HAIRLINE_WIDTH)
            .border_color(c.rule)
            .rounded(RADIUS_LAYER)
            .shadow_lg()
            .on_key_down(move |event, window, cx| {
                if event.keystroke.key == "escape"
                    && let Some(escape) = &escape
                {
                    escape(window, cx);
                    cx.stop_propagation();
                }
            })
            .child(super::text::text(Type::Section, c.text, self.title.clone()))
            .child(
                div()
                    .id(ElementId::NamedChild(
                        std::sync::Arc::new(self.id.clone()),
                        SharedString::new_static("body"),
                    ))
                    .flex()
                    .flex_col()
                    .gap(XS)
                    .overflow_y_scroll()
                    .children(self.body),
            )
            .child(line);
        deferred(
            anchored().position(point(px(0.0), px(0.0))).child(
                div()
                    .id(ElementId::NamedChild(
                        std::sync::Arc::new(self.id),
                        SharedString::new_static("backdrop"),
                    ))
                    .w(viewport.width)
                    .h(viewport.height)
                    .bg(c.backdrop)
                    .occlude()
                    .flex()
                    .flex_col()
                    .items_center()
                    .pt(DIALOG_INSET)
                    .child(frame),
            ),
        )
        .with_priority(2)
    }
}
