//! A side column the DM can drag wider or narrower (`design::Column`): the rail, and the panel
//! beside a page (Lookup, the session's notes). Until the DM drags it, a column has its natural
//! width, which can follow the window. After that it has the DM's width, here and after a
//! restart, and the page beside it takes what is left.
//!
//! The DM's widths, by egui's column id (`navigation`, `lookup`, `library_sections`, …), live in
//! one GPUI global (`ColumnWidths`), as egui keeps them in its memory, so a column deep in a page
//! needs no path back to the root. The root restores them at the start and saves them with
//! `ui_state` when they change. `kit::Columns` keeps its splitters' widths there too
//! (`Columns::saved`).
//!
//! The splitter on the column's inner edge drags it; with the keyboard on it, Left and Right move
//! the edge by `L` and Home gives back the natural width, as egui's handle does.
use crate::gui::theme::{FOCUS_WIDTH, L, SPLITTER_WIDTH, colors};
use gpui::{
    AnyElement, App, CursorStyle, ElementId, FocusHandle, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Role, SharedString, StatefulInteractiveElement, Styled, Window, div,
    prelude::*, px,
};
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

/// What a drag leaves for the page beside a column, unless the column says more
/// (`design::column::PAGE_KEEPS`).
const PAGE_KEEPS: Pixels = px(320.0);

/// The widths the DM dragged columns to, by column id.
#[derive(Clone, Default, PartialEq, Debug)]
pub struct ColumnWidths(pub BTreeMap<String, f32>);

impl Global for ColumnWidths {}

/// Restore the widths this computer remembers (the root, once at the start).
pub fn restore_column_widths(widths: BTreeMap<String, f32>, cx: &mut App) {
    cx.set_global(ColumnWidths(widths));
}

/// Every width the DM chose, for `ui_state`.
pub fn column_widths(cx: &App) -> BTreeMap<String, f32> {
    cx.try_global::<ColumnWidths>()
        .map(|widths| widths.0.clone())
        .unwrap_or_default()
}

/// The width the DM dragged this column to, if they did (`design::column_width`).
pub fn column_width(key: &str, cx: &App) -> Option<f32> {
    cx.try_global::<ColumnWidths>()
        .and_then(|widths| widths.0.get(key).copied())
}

/// Keep the DM's width of a column (`None` forgets it: the column goes back to its natural
/// width).
pub fn set_column_width(key: &str, width: Option<f32>, cx: &mut App) {
    let widths = &mut cx.default_global::<ColumnWidths>().0;
    match width {
        Some(width) => {
            widths.insert(key.to_owned(), width);
        }
        None => {
            widths.remove(key);
        }
    }
}

/// Which edge of the window or page the column stands at.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    /// At the left; the splitter is on its right edge (the rail).
    Left,
    /// At the right; the splitter is on its left edge (Lookup, the notes panel).
    Right,
}

/// The drag of one column's splitter.
struct EdgeDrag(&'static str);

/// What a drag shows under the pointer: nothing (the column itself follows it).
struct NoGhost;

impl gpui::Render for NoGhost {
    fn render(&mut self, _: &mut Window, _: &mut gpui::Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

/// A side column with a splitter on its inner edge.
///
/// ```ignore
/// let column = kit::EdgeColumn::new("lookup", Side::Right, natural, PANEL_MIN_WIDTH..=PANEL_DRAG_MAX_WIDTH)
///     .page_keeps(PANEL_LEAVES_PAGE)
///     .name("Lookup");
/// let width = column.width(room, cx);
/// column.render(room, panel, window, cx)
/// ```
#[derive(Clone)]
pub struct EdgeColumn {
    key: &'static str,
    side: Side,
    natural: Pixels,
    range: RangeInclusive<Pixels>,
    page_keeps: Pixels,
    name: SharedString,
}

impl EdgeColumn {
    /// The column `key` (egui's id, which `ui_state` keeps), its natural width and the range a drag
    /// keeps it in.
    pub fn new(
        key: &'static str,
        side: Side,
        natural: Pixels,
        range: RangeInclusive<Pixels>,
    ) -> Self {
        Self {
            key,
            side,
            natural,
            range,
            page_keeps: PAGE_KEEPS,
            name: SharedString::new_static(key),
        }
    }

    /// For a page that needs more than the usual room beside this column.
    pub fn page_keeps(mut self, width: Pixels) -> Self {
        self.page_keeps = width;
        self
    }

    /// The column's name in its splitter's accessible name (`Resize Lookup column`).
    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = name.into();
        self
    }

    /// The narrowest and widest the column can be in `room` (`Column::bounds`).
    fn bounds(&self, room: Pixels) -> (Pixels, Pixels) {
        let min = (*self.range.start()).min(self.natural);
        let max = (*self.range.end())
            .max(self.natural)
            .min((room - self.page_keeps).max(*self.range.start()));
        (min, max)
    }

    /// The width this column draws at in `room` (the room of its side of the window, the column
    /// included): the DM's width when they dragged it, else its natural width, inside its range.
    pub fn width(&self, room: Pixels, cx: &App) -> Pixels {
        let (min, max) = self.bounds(room);
        let chosen = column_width(self.key, cx).map_or(self.natural, px);
        chosen.clamp(min, max.max(min)).min(room).round()
    }

    /// The column at its width in `room`, holding `content`, with its splitter on the inner edge.
    pub fn render(
        self,
        room: Pixels,
        content: impl IntoElement,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let width = self.width(room, cx);
        let id = ElementId::Name(format!("column-{}", self.key).into());
        let focus = window
            .use_keyed_state(
                ElementId::NamedChild(std::sync::Arc::new(id.clone()), "splitter-focus".into()),
                cx,
                |_, cx| cx.focus_handle().tab_stop(true),
            )
            .read(cx)
            .clone();
        let handle = splitter(&self, id.clone(), &focus, room, window);
        let column = self.clone();
        div()
            .id(id)
            .relative()
            .flex_none()
            .h_full()
            .w(width)
            .on_drag_move::<EdgeDrag>(move |event, window, cx| {
                if event.drag(cx).0 != column.key {
                    return;
                }
                let pointer = event.event.position.x;
                let wanted = match column.side {
                    Side::Left => pointer - event.bounds.left(),
                    Side::Right => event.bounds.right() - pointer,
                };
                let (min, max) = column.bounds(room);
                let width = wanted.clamp(min, max.max(min)).min(room).round();
                set_column_width(column.key, Some(f32::from(width)), cx);
                window.refresh();
            })
            .child(super::layer::room_start(px(0.0)))
            .child(content)
            .child(handle)
            .child(super::layer::room_end())
            .into_any_element()
    }
}

/// The splitter: a band `SPLITTER_WIDTH` wide over the column's inner edge. It shows the focus
/// colour while the keyboard is on it.
fn splitter(
    column: &EdgeColumn,
    id: ElementId,
    focus: &FocusHandle,
    room: Pixels,
    window: &Window,
) -> impl IntoElement {
    let c = colors();
    let focused = focus.is_focused(window);
    let key = column.key;
    let keys = column.clone();
    let band = div()
        .id(ElementId::NamedChild(
            std::sync::Arc::new(id),
            "splitter".into(),
        ))
        .absolute()
        .top_0()
        .h_full()
        .w(SPLITTER_WIDTH)
        .when(column.side == Side::Left, |band| {
            band.right(-(SPLITTER_WIDTH / 2.0))
        })
        .when(column.side == Side::Right, |band| {
            band.left(-(SPLITTER_WIDTH / 2.0))
        })
        .flex()
        .justify_center()
        .cursor(CursorStyle::ResizeLeftRight)
        .track_focus(focus)
        .role(Role::Splitter)
        .aria_label(SharedString::from(format!("Resize {} column", column.name)))
        .on_drag(EdgeDrag(key), |_, _, _, cx| cx.new(|_| NoGhost))
        .on_key_down(move |event, window, cx| {
            let pressed = event.keystroke.key.as_str();
            if event.keystroke.modifiers.modified() {
                return;
            }
            let step = match (pressed, keys.side) {
                ("right", Side::Left) | ("left", Side::Right) => L,
                ("left", Side::Left) | ("right", Side::Right) => -L,
                ("home", _) => {
                    set_column_width(keys.key, None, cx);
                    cx.stop_propagation();
                    window.refresh();
                    return;
                }
                _ => return,
            };
            let (min, max) = keys.bounds(room);
            let now = keys.width(room, cx);
            let next = (now + step).clamp(min, max.max(min)).min(room);
            set_column_width(keys.key, Some(f32::from(next)), cx);
            cx.stop_propagation();
            window.refresh();
        })
        .child(
            div()
                .h_full()
                .w(FOCUS_WIDTH)
                .when(focused, |line| line.bg(c.focus))
                .hover(|line| line.bg(c.rule)),
        );
    super::target(band, format!("Resize {} column", column.name))
}
