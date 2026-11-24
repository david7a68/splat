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
pub mod comparison;
pub mod dialog;
pub mod edge;
pub mod empty;
pub mod fact;
pub mod field;
pub mod hp;
pub mod layer;
pub mod link;
pub mod list;
pub mod markdown;
pub mod markdown_source;
pub mod measure;
pub mod menu;
pub mod messages;
pub mod nav;
pub mod outcome;
pub mod panes;
pub mod portrait;
pub mod quiet_section;
pub mod reveal;
pub mod roll_link;
pub mod scroll;
pub mod section;
pub mod status;
pub mod stepper;
pub mod tabs;
pub mod text;
pub mod title;
pub mod tooltip;
pub mod value_block;
pub mod virtual_list;

pub use button::{Button, IconButton};
pub use checkbox::{Checkbox, Switch};
pub use comparison::DraftComparison;
pub use dialog::Dialog;
pub use edge::EdgeColumn;
pub use empty::empty_state;
pub use field::{EditorHandle, TextEditor, TextField, field, help};
pub use link::{Link, link};
pub use list::{Row, two_line_row};
pub use markdown::{Markdown, markdown};
pub use menu::{Dropdown, MenuButton, MenuItem, Popover, StatusMenu};
pub use nav::{NavItem, NavToggle};
pub use outcome::{Message, Tone};
pub use panes::{Columns, Pane};
pub use portrait::portrait;
pub use quiet_section::QuietSection;
pub use roll_link::RollNumber;
pub use scroll::scroll_column;
pub use section::Section;
pub use status::{Status, status, status_link};
pub use stepper::Stepper;
pub use tabs::Segmented;
pub use text::{Heading, gap, heading, icon, label, rule, text};
pub use title::{page_header, page_header_with};
pub use tooltip::WithTooltip;
pub use value_block::{HeadingNote, HeadingRow, ValueBlock};
pub use virtual_list::{measured_column, uniform_column};

use gpui::{IntoElement, ParentElement, SharedString};

/// A click handler as the kit takes it.
pub type OnClick = std::rc::Rc<dyn Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App)>;

/// A handler with no event (Escape in a dialog, Enter in a field).
pub type OnAct = std::rc::Rc<dyn Fn(&mut gpui::Window, &mut gpui::App)>;

/// A handler of a chosen index (a segment, a drop-down's choice).
pub type OnIndex = std::rc::Rc<dyn Fn(&usize, &mut gpui::Window, &mut gpui::App)>;

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
