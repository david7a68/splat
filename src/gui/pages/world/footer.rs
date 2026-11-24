//! A pane's footer messages (egui: `design::outcome::Messages` in a `PaneFrame`): every result
//! with its Undo, then the reasons, then the notices, from the stores' own lines
//! (`design::outcome::plain`). Each control's press carries its store's ticket, so an Undo acts
//! only on the result its line showed; a replaced result's press does nothing.
use super::{SplatRoot, world};
use crate::design::outcome::{Press, plain};
use crate::gui::{kit, theme};
use crate::world::FooterAct;
use gpui::{AnyElement, ClickEvent, Context, IntoElement, ParentElement, Styled, div};

/// The presses of the footer lines of this frame, by their place. A button takes its own once.
pub(crate) struct Presses<A>(Vec<Option<Press<A>>>);

impl<A> Default for Presses<A> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<A> Presses<A> {
    pub(super) fn clear(&mut self) {
        self.0.clear();
    }

    pub(super) fn keep(&mut self, press: Press<A>) -> usize {
        self.0.push(Some(press));
        self.0.len() - 1
    }

    pub(super) fn take(&mut self, at: usize) -> Option<Press<A>> {
        self.0.get_mut(at).and_then(Option::take)
    }
}

/// A design tone as the kit's.
fn tone(tone: crate::design::Tone) -> kit::Tone {
    match tone {
        crate::design::Tone::Info => kit::Tone::Info,
        crate::design::Tone::Success => kit::Tone::Success,
        crate::design::Tone::Warning => kit::Tone::Warning,
        crate::design::Tone::Error => kit::Tone::Error,
    }
}

/// The press of the footer control at `at`, as the page takes it: its own result and ticks, the
/// trackers' Undo, the reader's results; then what the view does beside (copy, open).
fn pressed(root: &mut SplatRoot, at: usize, cx: &mut Context<SplatRoot>) {
    let Some(press) = root.pages.world.presses.take(at) else {
        return;
    };
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

/// The lines as kit messages, each control with its press kept for this frame.
pub(super) fn render(
    root: &mut SplatRoot,
    lines: Vec<plain::Line<FooterAct>>,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    let mut shown = Vec::with_capacity(lines.len());
    for (line_at, line) in lines.into_iter().enumerate() {
        let mut message = kit::Message::new(tone(line.tone), line.words);
        if let Some(undo) = line.undo {
            let label = undo.label.clone();
            let blocked = undo.blocked.clone();
            let at = root.pages.world.presses.keep(undo.press);
            message = message.action(
                kit::Button::quiet(("world-footer-undo", line_at), label)
                    .icon(crate::gui::icons::UNDO)
                    .density(theme::Density::Read)
                    .blocked(blocked)
                    .on_click(cx.listener(move |root, _: &ClickEvent, _, cx| {
                        pressed(root, at, cx);
                    })),
            );
        }
        if let Some((path, words, edit, show)) = line.file {
            message = message.action(kit::text(theme::Type::Small, theme::colors().muted, words));
            if let Some(path) = path {
                if edit {
                    let url = crate::world::model::obsidian_link(&path);
                    message = message.action(
                        kit::Link::new(("world-footer-edit", line_at), "Edit in Obsidian")
                            .small()
                            .on_click(move |_, _, cx| cx.open_url(&url)),
                    );
                }
                if show {
                    let url = crate::world::model::file_link(&path);
                    message = message.action(
                        kit::Link::new(("world-footer-show", line_at), "Show file")
                            .small()
                            .on_click(move |_, _, cx| cx.open_url(&url)),
                    );
                }
            }
        }
        for (choice_at, choice) in line.choices.into_iter().enumerate() {
            let at = root.pages.world.presses.keep(choice.press);
            let id = ("world-footer-choice", line_at * 16 + choice_at);
            let handler = cx.listener(move |root, _: &ClickEvent, _, cx| pressed(root, at, cx));
            if choice.link {
                let mut link = kit::Link::new(id, choice.label).small().on_click(handler);
                if let Some(hover) = choice.hover {
                    link = link.hint(hover);
                }
                message = message.action(link);
            } else if choice.dismiss {
                message = message.action(
                    kit::IconButton::new(id, crate::gui::icons::CLOSE, choice.label)
                        .on_click(handler),
                );
            } else if choice.lose {
                message = message.destructive(
                    kit::Button::destructive(id, choice.label)
                        .density(theme::Density::Read)
                        .blocked(choice.blocked)
                        .on_click(handler),
                );
            } else {
                message = message.action(
                    kit::Button::secondary(id, choice.label)
                        .density(theme::Density::Read)
                        .blocked(choice.blocked)
                        .on_click(handler),
                );
            }
        }
        let mut block = div().flex().flex_col().gap(theme::XS).child(message);
        for help in line.help {
            block = block.child(kit::help(help));
        }
        shown.push(block.into_any_element());
    }
    shown
}
