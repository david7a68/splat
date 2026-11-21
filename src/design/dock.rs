//! Docking a tile: a grip that carries it (`drag_grip` with `drag_source`), the side of a place
//! the pointer is nearest (`DockSide::nearest`), and where the tile will go (`dock_preview`).
//! The caller owns the layout and decides what a drop does.
use super::tokens::*;
use egui::{self, CursorIcon, Id, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2};

/// A side of a place where a tile can go.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DockSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl DockSide {
    pub const ALL: [Self; 4] = [Self::Left, Self::Right, Self::Top, Self::Bottom];

    /// The side of `rect` that `pointer` is nearest, as a part of its width or height, so a
    /// wide place and a tall one split the same way: by its diagonals.
    pub fn nearest(rect: Rect, pointer: Pos2) -> Self {
        let x = (pointer.x - rect.left()) / rect.width().max(1.0);
        let y = (pointer.y - rect.top()) / rect.height().max(1.0);
        [
            (x, Self::Left),
            (1.0 - x, Self::Right),
            (y, Self::Top),
            (1.0 - y, Self::Bottom),
        ]
        .into_iter()
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map_or(Self::Left, |(_, side)| side)
    }

    /// The part of `rect` along this side: `share` of its width or height.
    pub fn part(self, rect: Rect, share: f32) -> Rect {
        let (width, height) = (rect.width() * share, rect.height() * share);
        match self {
            Self::Left => Rect::from_min_size(rect.min, Vec2::new(width, rect.height())),
            Self::Right => {
                Rect::from_min_max(egui::pos2(rect.right() - width, rect.top()), rect.max)
            }
            Self::Top => Rect::from_min_size(rect.min, Vec2::new(rect.width(), height)),
            Self::Bottom => {
                Rect::from_min_max(egui::pos2(rect.left(), rect.bottom() - height), rect.max)
            }
        }
    }

    /// The band of `rect` within `depth` of this side.
    pub fn band(self, rect: Rect, depth: f32) -> Rect {
        let share = |length: f32| (depth / length.max(1.0)).min(1.0);
        match self {
            Self::Left | Self::Right => self.part(rect, share(rect.width())),
            Self::Top | Self::Bottom => self.part(rect, share(rect.height())),
        }
    }
}

/// Where a dragged tile will go if it is let go now: `rect`, tinted with an accent outline,
/// over what is drawn there.
pub fn dock_preview(ui: &Ui, rect: Rect) {
    let c = super::colors();
    let painter = ui.painter();
    painter.rect_filled(rect, RADIUS_CONTROL, c.accent.gamma_multiply(0.12));
    painter.rect_stroke(
        rect,
        RADIUS_CONTROL,
        Stroke::new(FOCUS_WIDTH, c.accent),
        StrokeKind::Inside,
    );
}

/// The grip that drags a tile: the drag icon in an `XL` square, muted until hovered, with the
/// grab cursor. It takes no keyboard focus: the tile's menu does the same with the keys. Give
/// it to `drag_source` with the tile.
pub fn drag_grip(ui: &mut Ui, id: Id, name: &str) -> Response {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(XL), Sense::hover());
    let response = ui.interact(rect, id, Sense::DRAG);
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, name));
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_description("Drag to place the tile beside another, or along an edge.");
    });
    if response.hovered() && !response.dragged() {
        ui.ctx().set_cursor_icon(CursorIcon::Grab);
    }
    if ui.is_rect_visible(rect) {
        let c = super::colors();
        let colour = if response.hovered() || response.dragged() {
            c.text
        } else {
            c.muted
        };
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            super::icons::DRAG,
            super::Type::Body.font(ui),
            colour,
        );
    }
    response
}
