//! A Library pane's footer: the typed lines (`design::outcome::port`) as kit messages, in the
//! footer's one order, each control a kit button. A press reaches the line it was drawn from
//! (by its place in the lines kept from the last frame) and goes to `Library::press_footer`.
use crate::gui::theme::{S, Type, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use crate::library::LibraryPress;
use crate::library::model::{ControlKind, FooterLine};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, div,
};
use std::path::Path;

/// The kit's tone of a design tone.
pub(super) fn tone(tone: crate::design::Tone) -> kit::Tone {
    match tone {
        crate::design::Tone::Info => kit::Tone::Info,
        crate::design::Tone::Success => kit::Tone::Success,
        crate::design::Tone::Warning => kit::Tone::Warning,
        crate::design::Tone::Error => kit::Tone::Error,
    }
}

/// A file's `file://` address, as egui's file links open it.
pub(super) fn file_url(path: &Path) -> String {
    format!("file://{}", encode(path))
}

/// Obsidian's address of a file (`Edit in Obsidian`).
pub(super) fn obsidian_url(path: &Path) -> String {
    format!("obsidian://open?path={}", encode(path))
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

/// The footer of this frame: `lines` are kept for their presses and drawn. Empty with no line.
pub(super) fn render(
    root: &mut SplatRoot,
    lines: Vec<FooterLine<LibraryPress>>,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    let elements = lines
        .iter()
        .enumerate()
        .map(|(at, line)| self::line(at, line, cx))
        .collect();
    root.pages.library.footer = lines;
    elements
}

/// One line as a kit message.
fn line(at: usize, line: &FooterLine<LibraryPress>, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let mut message = kit::Message::new(tone(line.tone), line.words.clone());
    let mut helps = Vec::new();
    for (control, part) in line.controls.iter().enumerate() {
        let id = ElementId::Name(format!("library-footer-{at}-{control}").into());
        let press = cx.listener(move |root: &mut SplatRoot, _, _, cx| {
            let Some(press) = root
                .pages
                .library
                .footer
                .get_mut(at)
                .and_then(|line| line.press(control))
            else {
                return;
            };
            let ctx = root.bridge.ctx().clone();
            let app = &mut root.app;
            app.library.press_footer(press, &ctx, &mut app.campaigns);
            cx.notify();
        });
        if part.kind == ControlKind::Dismiss {
            message = message.action(
                kit::IconButton::new(id, icons::CLOSE, part.words.clone()).on_click(press),
            );
            continue;
        }
        let button = match part.kind {
            ControlKind::Undo | ControlKind::Link => kit::Button::quiet(id, part.words.clone()),
            ControlKind::Primary => kit::Button::primary(id, part.words.clone()),
            ControlKind::Lose => kit::Button::destructive(id, part.words.clone()),
            _ => kit::Button::secondary(id, part.words.clone()),
        }
        .density(crate::gui::theme::Density::Read)
        .blocked(part.blocked.clone());
        let button = match &part.hover {
            Some(hover) => button.hint(hover.clone()),
            None => button,
        }
        .on_click(press);
        if let Some(help) = &part.help {
            helps.push(help.clone());
        }
        message = if part.kind == ControlKind::Lose {
            message.destructive(button)
        } else {
            message.action(button)
        };
    }
    let file = line.file.as_ref().map(|(folder, relative)| {
        let path = folder.join(relative);
        let shown: SharedString = relative
            .components()
            .map(|part| part.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/")
            .into();
        let url = file_url(&path);
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(kit::text(Type::Small, c.muted, shown))
            .child(
                kit::link(
                    ElementId::Name(format!("library-footer-{at}-file").into()),
                    "Show file"
                )
                .on_click(move |_, _, cx| cx.open_url(&url)),
            )
    });
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(message)
        .children(file)
        .children(line.help.iter().chain(&helps).map(|help| kit::help(help.clone())))
        .into_any_element()
}
