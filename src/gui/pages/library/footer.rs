//! A Library pane's footer: the typed lines (`design::outcome::plain`) drawn by `kit::messages`,
//! in the footer's one order. A press carries its line's ticket, so it reaches only the entry its
//! line showed, and goes to `Library::press_footer`.
use crate::gui::{SplatRoot, kit};
use crate::library::LibraryPress;
use crate::library::model::FooterLine;
use gpui::{AnyElement, Context};
use std::path::Path;

pub(super) use crate::gui::kit::messages::tone;

/// A file's `file://` address, as file links open it (`design::outcome::file_url`).
pub(super) fn file_url(path: &Path) -> String {
    format!("file://{}", encode(path))
}

/// Obsidian's address of a file (`Edit in Obsidian`).
pub(super) fn obsidian_url(path: &Path) -> String {
    crate::design::outcome::obsidian_url(path)
}

fn encode(path: &Path) -> String {
    path.to_string_lossy()
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

/// The footer's lines as kit messages. Empty with no line.
pub(super) fn render(
    lines: Vec<FooterLine<LibraryPress>>,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    kit::messages::lines(
        "library-footer",
        lines,
        cx.listener(
            |root: &mut SplatRoot, pressed: &kit::messages::Pressed<LibraryPress>, _, cx| {
                let Some(press) = pressed.take() else {
                    return;
                };
                let ctx = root.ctx.clone();
                let app = &mut root.app;
                app.library.press_footer(press, &ctx, &mut app.campaigns);
                cx.notify();
            },
        ),
    )
}
