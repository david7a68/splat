//! Splat's shared GPUI parts. Pages build only from these and from `gpui` layout (`div`, flex,
//! gaps from `theme`), never from `gpui_component` directly: each part here wraps it in Splat's
//! tokens and rules (`docs/design-system.md`).
//!
//! One file per part. A page agent that needs a new part adds a new file here and a line below.
//! See `docs/gpui-port.md`, Kit guide, for each part's signature and when to use it.
//!
//! Every interactive part takes a stable element id (`impl Into<ElementId>`) and its words; its
//! click handler is `Fn(&ClickEvent, &mut Window, &mut App)`, which `cx.listener` makes from a
//! root method. Parts that keep GPUI state of their own (a text field's input, an open menu) keep
//! it with `window.use_keyed_state` under their id, so the caller holds only its own data.

pub mod button;
pub mod checkbox;
pub mod dialog;
pub mod empty;
pub mod field;
pub mod list;
pub mod markdown;
pub mod menu;
pub mod nav;
pub mod outcome;
pub mod panes;
pub mod scroll;
pub mod status;
pub mod tabs;
pub mod text;
pub mod title;
pub mod tooltip;

pub use button::{Button, IconButton};
pub use checkbox::{Checkbox, Switch};
pub use dialog::{Dialog, DialogAction};
pub use empty::empty_state;
pub use field::{TextEditor, TextField, field, help};
pub use list::{Row, two_line_row};
pub use markdown::markdown;
pub use menu::{Dropdown, MenuButton, MenuItem, Popover};
pub use nav::{NavItem, NavToggle};
pub use outcome::{Message, Tone};
pub use panes::{Columns, Pane};
pub use scroll::scroll_column;
pub use status::{Status, status, status_link};
pub use tabs::Segmented;
pub use text::{Heading, gap, heading, icon, label, rule, text};
pub use title::page_header;
pub use tooltip::WithTooltip;

use gpui::{IntoElement, ParentElement, SharedString};

/// A click handler as the kit takes it.
pub type OnClick = std::rc::Rc<dyn Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App)>;

/// Mark an element as a named target, so the test harness can find it by `name`
/// (`harness::Shell::click`). Kit parts call it with their words and their id; a page calls it
/// only for a target the kit does not draw. In a normal build it does nothing.
#[allow(unused_variables)]
pub fn target<E: ParentElement>(element: E, name: impl Into<SharedString>) -> E {
    #[cfg(test)]
    {
        let name = name.into();
        let mut element = element;
        element.extend(std::iter::once(probe::probe(name).into_any_element()));
        element
    }
    #[cfg(not(test))]
    element
}

/// The harness's record of named targets and where they were last laid out.
#[cfg(test)]
pub(crate) mod probe {
    use gpui::{Bounds, IntoElement, Pixels, SharedString, Styled, canvas};
    use std::cell::RefCell;

    thread_local! {
        /// The targets of the last frame, in paint order. One thread per test (each test's app
        /// runs on its own thread), so no test sees another's targets.
        static TARGETS: RefCell<Vec<(SharedString, Bounds<Pixels>)>> = const { RefCell::new(Vec::new()) };
    }

    /// An invisible child as large as its parent, which records the parent's bounds.
    pub(super) fn probe(name: SharedString) -> impl IntoElement {
        canvas(
            move |bounds, _, _| {
                TARGETS.with(|targets| targets.borrow_mut().push((name.clone(), bounds)));
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full()
    }

    /// Forget the last frame's targets (the root does this as each frame starts).
    pub(crate) fn clear() {
        TARGETS.with(|targets| targets.borrow_mut().clear());
    }

    /// The last target laid out under `name`.
    pub(crate) fn find(name: &str) -> Option<Bounds<Pixels>> {
        TARGETS.with(|targets| {
            targets
                .borrow()
                .iter()
                .rev()
                .find(|(n, _)| n.as_ref() == name)
                .map(|(_, bounds)| *bounds)
        })
    }

    /// Every name laid out in the last frame (for a test's failure message).
    pub(crate) fn names() -> Vec<String> {
        TARGETS.with(|targets| {
            targets
                .borrow()
                .iter()
                .map(|(name, _)| name.to_string())
                .collect()
        })
    }
}
