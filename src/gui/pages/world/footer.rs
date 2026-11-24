//! A pane's footer messages (`design::outcome::Messages` in a `PaneFrame`): every result
//! with its Undo, then the reasons, then the notices, from the stores' own lines
//! (`design::outcome::plain`), drawn by `kit::messages`. Each control's press carries its store's
//! ticket, so an Undo acts only on the result its line showed; a replaced result's press does
//! nothing.
use super::{SplatRoot, world};
use crate::design::outcome::Press;
use crate::design::outcome::plain::PlainLine;
use crate::gui::kit;
use crate::world::FooterAct;
use gpui::{AnyElement, Context};

/// A footer press, as the page takes it: its own result and ticks, the trackers' Undo, the
/// reader's results; then what the view does beside (copy, open).
fn pressed(root: &mut SplatRoot, press: Press<FooterAct>, cx: &mut Context<SplatRoot>) {
    let Some((campaign, index)) = world(root) else {
        return;
    };
    let app = &mut root.app;
    let effect = app.world_page.footer_press_plain(
        press,
        &index,
        &campaign,
        &mut crate::trackers::Reader(&mut app.trackers),
    );
    match effect {
        Some(crate::world::model::Effect::Copy(words)) => {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(words));
        }
        Some(crate::world::model::Effect::Open(url)) => cx.open_url(&url),
        None => {}
    }
    cx.notify();
}

/// The lines as kit messages.
pub(super) fn render(
    lines: Vec<PlainLine<FooterAct>>,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    kit::messages::lines(
        "world-footer",
        lines,
        cx.listener(
            |root: &mut SplatRoot, pressed: &kit::messages::Pressed<FooterAct>, _, cx| {
                if let Some(press) = pressed.take() {
                    self::pressed(root, press, cx);
                }
            },
        ),
    )
}
