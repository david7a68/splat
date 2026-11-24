//! The Session page's pane footers: the egui page's own footer lines (`Messages::into_plain`)
//! drawn by `kit::messages`, each press handed to the page's own press method
//! (`session_press`, `draft_press`, …) with its ticket.
use crate::gui::{SplatRoot, kit};
use crate::sessions::page::gpui_facade::{FooterLine, FooterPress};
use gpui::{AnyElement, Context, Window};
use std::rc::Rc;

/// What a press in this footer does: the page's own `session_press`, `draft_press`, …
pub(super) type OnPress =
    Rc<dyn Fn(&mut SplatRoot, FooterPress, &mut Window, &mut Context<SplatRoot>)>;

pub(super) use kit::messages::tone;

/// Each line as a kit message, in the footer's order. `id` keeps the controls' ids apart from
/// another footer's.
pub(super) fn lines(
    id: &str,
    lines: Vec<FooterLine>,
    on_press: OnPress,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    kit::messages::lines(
        gpui::ElementId::Name(id.to_owned().into()),
        lines,
        cx.listener(
            move |root, pressed: &kit::messages::Pressed<crate::sessions::page::SessionAct>, window, cx| {
                if let Some(press) = pressed.take() {
                    on_press(root, press, window, cx);
                    cx.notify();
                }
            },
        ),
    )
}
