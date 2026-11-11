//! TEMPORARY: package A's `design::help` and `design::MessageSpace`, copied from its PR (252)
//! so this branch builds before A lands. Removed at the merge with origin/main.
#![allow(dead_code)]
use crate::design::{READER_MEASURE, Type, colors, focus_ring};
use eframe::egui::{self, Align, Id, Layout, Rect, Response, Sense, Ui, pos2, vec2};

pub fn help(ui: &mut Ui, text: &str) -> Response {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(READER_MEASURE));
        ui.add(egui::Label::new(Type::Small.rich(ui, text).color(colors().muted)).wrap())
    })
    .inner
}

/// How many text lines the message space below a save row holds.
pub const MESSAGE_SPACE_LINES: f32 = 2.0;

/// The height of the message space below a save row: `MESSAGE_SPACE_LINES` body lines.
pub fn message_space_height() -> f32 {
    MESSAGE_SPACE_LINES * Type::Body.line()
}

/// One message in a `MessageSpace`: its whole words, the part's accessible name, and how to
/// draw it.
type SpaceMessage<'a, R> = (&'a str, Box<dyn FnOnce(&mut Ui) -> R + 'a>);

/// The one message space below a save row: a `width` by `MESSAGE_SPACE_LINES` part at the
/// cursor, whatever it holds, so no message moves what is below it. It holds Save's reason and
/// the latest result in one part, in one order: a result that carries a live Undo comes first,
/// then the reason; otherwise the reason comes first, then the result. With neither, the space
/// stays empty. Longer words wrap and scroll inside the part (see `fixed_part`), whose accessible
/// name is every message's words in that order.
///
/// ```ignore
/// let pressed = design::MessageSpace::new("library_message", width)
///     .reason(problem, |ui| { design::message(ui, Tone::Error, problem); })
///     .result(&words, true, |ui| design::draft_discarded(ui, name))
///     .show(ui);
/// ```
// Library, World repair and Campaign repair adopt it this round (iter-25 packages L, D and C).
#[allow(dead_code)]
pub struct MessageSpace<'a, R> {
    id: &'a str,
    width: f32,
    reason: Option<SpaceMessage<'a, ()>>,
    result: Option<(SpaceMessage<'a, R>, bool)>,
}

#[allow(dead_code)]
impl<'a, R> MessageSpace<'a, R> {
    /// An empty space `width` wide. `id` keeps the part's scroll and focus.
    pub fn new(id: &'a str, width: f32) -> Self {
        Self {
            id,
            width,
            reason: None,
            result: None,
        }
    }

    /// Why Save or the draft waits: a refusal, a parse error, a failed write or guidance.
    pub fn reason(mut self, words: &'a str, add: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.reason = Some((words, Box::new(add)));
        self
    }

    /// The latest result. `has_undo` is true while its Undo is live; then it comes before the
    /// reason, so a reason that does not clear never hides the way back.
    pub fn result(
        mut self,
        words: &'a str,
        has_undo: bool,
        add: impl FnOnce(&mut Ui) -> R + 'a,
    ) -> Self {
        self.result = Some(((words, Box::new(add)), has_undo));
        self
    }

    /// Reserve the space and draw what it holds. Returns what the result's `add` returned, when
    /// there is a result.
    pub fn show(self, ui: &mut Ui) -> Option<R> {
        let (rect, _) =
            ui.allocate_exact_size(vec2(self.width, message_space_height()), Sense::hover());
        let undo_first = self.result.as_ref().is_some_and(|(_, has_undo)| *has_undo);
        let (reason, result) = (self.reason, self.result.map(|(result, _)| result));
        if reason.is_none() && result.is_none() {
            return None;
        }
        let reason_words = reason.as_ref().map(|(words, _)| *words);
        let result_words = result.as_ref().map(|(words, _)| *words);
        let name = if undo_first {
            [result_words, reason_words]
        } else {
            [reason_words, result_words]
        }
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
        fixed_part(ui, rect, self.id, &name, |ui| {
            if undo_first {
                let out = result.map(|(_, add)| add(ui));
                if let Some((_, add)) = reason {
                    add(ui);
                }
                out
            } else {
                if let Some((_, add)) = reason {
                    add(ui);
                }
                result.map(|(_, add)| add(ui))
            }
        })
    }
}

/// One fixed part of a reserved space: a fixed rect whatever it holds. Longer words wrap and
/// scroll inside it, beside a lane kept for the scroll bar, so nothing is cut and nothing below
/// moves. The part is one focusable widget named with its whole message, so Tab reaches it and a
/// screen reader reads all of it. While it has focus, the up and down arrows scroll it by a line
/// and Page Up and Page Down by the part; the wheel scrolls it too.
pub fn fixed_part<R>(
    ui: &mut Ui,
    rect: Rect,
    id: &str,
    name: &str,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    let focus = Id::new((id, "focus"));
    let focused = ui.ctx().memory(|memory| memory.has_focus(focus));
    let step = if focused {
        // The arrows scroll the part instead of moving focus.
        ui.ctx().memory_mut(|memory| {
            memory.set_focus_lock_filter(
                focus,
                egui::EventFilter {
                    vertical_arrows: true,
                    ..Default::default()
                },
            );
        });
        let line = Type::Body.line();
        ui.input_mut(|input| {
            let mut step = 0.0;
            for (key, by) in [
                (egui::Key::ArrowDown, line),
                (egui::Key::ArrowUp, -line),
                (egui::Key::PageDown, rect.height()),
                (egui::Key::PageUp, -rect.height()),
            ] {
                while input.consume_key(egui::Modifiers::NONE, key) {
                    step += by;
                }
            }
            step
        })
    } else {
        0.0
    };
    let mut part = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::top_down(Align::Min)),
    );
    let lane = part.spacing().scroll.bar_width + part.spacing().scroll.bar_outer_margin;
    let inner = egui::ScrollArea::vertical()
        .id_salt(id)
        .max_height(rect.height())
        // egui's default minimum would let a short overflow grow past the part.
        .min_scrolled_height(rect.height())
        .auto_shrink([false, true])
        .show(&mut part, |ui| {
            ui.set_max_width(rect.width() - lane);
            if step != 0.0 {
                ui.scroll_with_delta(vec2(0.0, -step));
            }
            add(ui)
        })
        .inner;
    let response = ui.interact(rect, focus, Sense::focusable_noninteractive());
    // A named group that holds the message and its actions: a screen reader reads the whole
    // message, and the group is not a second target over its own Undo.
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::GenericContainer);
        node.set_label(name);
    });
    if response.has_focus() {
        // Inside the content area, clear of the part's own scroll bar lane.
        focus_ring(
            ui,
            Rect::from_min_max(rect.min, pos2(rect.right() - lane, rect.bottom())),
        );
    }
    inner
}
