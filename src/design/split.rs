//! A drag handle on the edge between two tiles of one page: between two columns of tiles, or
//! between two tiles stacked in a column (the DM screen). It works as a column's handle does
//! (`column.rs`), but it is the app's own control, so it is a whole target: `XL` across, over
//! the gap and the tiles' frame margins, along the whole edge. Tab gives it focus, shown by an
//! accent line. The arrow keys along its axis move
//! the edge by `L`, and Home asks for the two sides to share their room evenly. The caller owns
//! the sizes and their limits: the handle only says what the DM asked for.
use super::tokens::*;
use egui::{self, CursorIcon, Id, Key, Modifiers, Rect, Sense, Stroke, Ui};

/// Which edge a handle is on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitAxis {
    /// The edge between two columns: it moves left and right.
    Columns,
    /// The edge between two tiles in a column: it moves up and down.
    Rows,
}

/// What the DM asked of a handle in this frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SplitMove {
    /// Move the edge by this many points: right or down when positive.
    By(f32),
    /// Give the two sides the same room.
    Even,
}

/// The handle on the edge in `gap`, the room between two tiles, named for the DM (`Resize …`).
/// A column edge's grab band is kept clear of the `line_start` hangs of the column after it
/// (`pane::reserve`), so draw the handle before that column.
pub fn split_handle(
    ui: &mut Ui,
    id: Id,
    axis: SplitAxis,
    gap: Rect,
    name: &str,
) -> Option<SplitMove> {
    let grab = XL / 2.0;
    let centre = gap.center();
    let band = match axis {
        SplitAxis::Columns => {
            Rect::from_x_y_ranges((centre.x - grab)..=(centre.x + grab), gap.y_range())
        }
        SplitAxis::Rows => {
            Rect::from_x_y_ranges(gap.x_range(), (centre.y - grab)..=(centre.y + grab))
        }
    };
    if axis == SplitAxis::Columns {
        super::pane::reserve(ui, band);
    }
    let response = ui.interact(band, id, Sense::DRAG | Sense::FOCUSABLE);
    let (back, forward, keys) = match axis {
        SplitAxis::Columns => (
            Key::ArrowLeft,
            Key::ArrowRight,
            "Left and Right move the edge. Home gives both sides the same width.",
        ),
        SplitAxis::Rows => (
            Key::ArrowUp,
            Key::ArrowDown,
            "Up and Down move the edge. Home gives both tiles the same height.",
        ),
    };
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::Splitter);
        node.set_label(name);
        node.set_description(keys);
    });
    if response.hovered() || response.dragged() {
        ui.ctx().set_cursor_icon(match axis {
            SplitAxis::Columns => CursorIcon::ResizeHorizontal,
            SplitAxis::Rows => CursorIcon::ResizeVertical,
        });
    }
    let line = |stroke: Stroke| match axis {
        SplitAxis::Columns => ui.painter().vline(centre.x, band.y_range(), stroke),
        SplitAxis::Rows => ui.painter().hline(band.x_range(), centre.y, stroke),
    };
    if response.has_focus() {
        line(Stroke::new(FOCUS_WIDTH, super::colors().focus));
    } else if response.hovered() || response.dragged() {
        line(Stroke::new(HAIRLINE_WIDTH, super::colors().muted));
    }
    let mut asked = None;
    if response.has_focus() {
        // Egui moves focus with the arrow keys; a focused handle keeps them, as a column's does.
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                id,
                egui::EventFilter {
                    horizontal_arrows: axis == SplitAxis::Columns,
                    vertical_arrows: axis == SplitAxis::Rows,
                    ..Default::default()
                },
            );
        });
        let (back, forward, even) = ui.input_mut(|input| {
            (
                input.consume_key(Modifiers::NONE, back),
                input.consume_key(Modifiers::NONE, forward),
                input.consume_key(Modifiers::NONE, Key::Home),
            )
        });
        if even {
            asked = Some(SplitMove::Even);
        } else if back || forward {
            asked = Some(SplitMove::By(
                (i32::from(forward) - i32::from(back)) as f32 * L,
            ));
        }
    }
    // The edge follows the pointer: the caller keeps it inside the two sides' limits.
    if response.dragged()
        && let Some(pointer) = response.interact_pointer_pos()
    {
        let by = match axis {
            SplitAxis::Columns => pointer.x - centre.x,
            SplitAxis::Rows => pointer.y - centre.y,
        };
        if by.abs() >= 0.5 {
            asked = Some(SplitMove::By(by));
        }
    }
    if asked.is_some() {
        ui.ctx().request_repaint();
    }
    asked
}
