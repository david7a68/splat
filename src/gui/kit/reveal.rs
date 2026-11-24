//! A reveal (`design::reveal`, egui's `scroll_to_rect`): an invisible box over its parent that,
//! in the frame it is laid out, scrolls a column the page keeps (`scroll::scroll_column_with`) so
//! the parent comes into view. Give it for one frame: the frame after an add or an act whose
//! result the DM must see (a plan row after an add, a check's result after a roll).
//!
//! GPUI lays a frame out before it knows where the parent lands, so the scroll lands in the next
//! frame, which the reveal asks for at once. No scroll the app makes is animated (R43).
use gpui::{
    Bounds, IntoElement, ParentElement, Pixels, ScrollHandle, Styled, Window, canvas, div, point,
};

/// How a reveal places its parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    /// The least scroll that brings it whole into view (or its top, when it is taller than the
    /// column).
    Least,
    /// Its top at the column's top.
    Top,
}

/// The scroll offset that shows `target` in a column of `view` scrolled to `offset` (negative
/// down, as GPUI keeps it), at most `max` down.
pub fn offset_for(
    target: Bounds<Pixels>,
    view: Bounds<Pixels>,
    offset: Pixels,
    max: Pixels,
    align: Align,
) -> Pixels {
    let wanted = match align {
        Align::Top => offset - (target.top() - view.top()),
        Align::Least if target.top() < view.top() || target.size.height > view.size.height => {
            offset + (view.top() - target.top())
        }
        Align::Least if target.bottom() > view.bottom() => {
            offset - (target.bottom() - view.bottom())
        }
        Align::Least => offset,
    };
    wanted.clamp(-max, gpui::px(0.0))
}

/// Put in a `.relative()` parent: scrolls `handle`'s column to show the parent, once per frame
/// it is drawn.
pub fn reveal(handle: &ScrollHandle, align: Align) -> impl IntoElement {
    let handle = handle.clone();
    canvas(
        move |bounds, window: &mut Window, _| {
            let view = handle.bounds();
            let now = handle.offset();
            let y = offset_for(bounds, view, now.y, handle.max_offset().y, align);
            if y != now.y {
                handle.set_offset(point(now.x, y));
                window.refresh();
            }
        },
        |_, _, _, _| {},
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// `element` with a reveal over it while `align` is given.
pub fn revealed(
    element: impl IntoElement,
    handle: &ScrollHandle,
    align: Option<Align>,
) -> gpui::AnyElement {
    match align {
        Some(align) => div()
            .relative()
            .w_full()
            .child(element)
            .child(reveal(handle, align))
            .into_any_element(),
        None => element.into_any_element(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{px, size};

    fn at(top: f32, height: f32) -> Bounds<Pixels> {
        Bounds::new(point(px(0.0), px(top)), size(px(100.0), px(height)))
    }

    #[test]
    fn a_reveal_scrolls_least_or_to_the_top() {
        let view = at(100.0, 200.0);
        // In view: no scroll.
        assert_eq!(
            offset_for(at(150.0, 20.0), view, px(0.0), px(500.0), Align::Least),
            px(0.0)
        );
        // Below: down by what is hidden.
        assert_eq!(
            offset_for(at(290.0, 20.0), view, px(0.0), px(500.0), Align::Least),
            px(-10.0)
        );
        // Above: up by what is hidden, never past the top.
        assert_eq!(
            offset_for(at(90.0, 20.0), view, px(-40.0), px(500.0), Align::Least),
            px(-30.0)
        );
        assert_eq!(
            offset_for(at(0.0, 20.0), view, px(-40.0), px(500.0), Align::Least),
            px(0.0)
        );
        // Top: its top at the column's top, as far as the column scrolls.
        assert_eq!(
            offset_for(at(250.0, 20.0), view, px(0.0), px(500.0), Align::Top),
            px(-150.0)
        );
        assert_eq!(
            offset_for(at(250.0, 20.0), view, px(0.0), px(60.0), Align::Top),
            px(-60.0)
        );
    }
}
