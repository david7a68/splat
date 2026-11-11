//! TEMPORARY: package A's `design::help` and `design::message_space`, copied from its work in
//! progress so this branch builds before A lands. Removed at the merge with origin/main.
#![allow(dead_code)]
use crate::design::{self, Type, colors};
use eframe::egui::{self, Align, Id, Layout, Rect, Response, Sense, Ui, pos2, vec2};

pub fn help(ui: &mut Ui, text: &str) -> Response {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
        ui.add(egui::Label::new(Type::Small.rich(ui, text).color(colors().muted)).wrap())
    })
    .inner
}

pub fn message_space<R>(
    ui: &mut Ui,
    id: &str,
    width: f32,
    message: Option<&str>,
    add: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let (rect, _) = ui.allocate_exact_size(vec2(width, 2.0 * Type::Body.line()), Sense::hover());
    message.map(|name| fixed_part(ui, rect, id, name, add))
}

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
        design::focus_ring(
            ui,
            Rect::from_min_max(rect.min, pos2(rect.right() - lane, rect.bottom())),
        );
    }
    inner
}
