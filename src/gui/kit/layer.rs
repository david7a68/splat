//! Where a raised layer opens (egui `design::popups`, `Dropdown::list`): under its trigger, or
//! above it when it does not fit below and there is more room above, `XS` from the trigger, and
//! always inside the room that owns the trigger: a pane's body above its footer and clear of its
//! scroll bar's lane (`kit::Pane`), a side column (`kit::EdgeColumn`: the rail, Lookup, the notes
//! panel), else the window one `S` inside its edges. A layer taller than its room scrolls; one
//! wider than its room is as wide as the room. While the trigger is scrolled out of its room, the
//! layer is not drawn.
//!
//! The room: each owner pushes its bounds as it lays out its children and pops them after
//! (`room_start`, `room_end`), so a trigger finds the innermost owner's room. The layer is drawn
//! after the whole window (`deferred`) and placed from what its trigger recorded in this frame.
use crate::gui::theme::{S, XS};
use gpui::{
    AlignItems, AnyElement, App, Bounds, Display, Div, Element, ElementId, FlexDirection,
    GlobalElementId, InspectorElementId, IntoElement, LayoutId, ParentElement, Pixels, Position,
    Style, Styled, Window, canvas, deferred, point, px,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Where a layer's trigger was, by the layer's id, kept from frame to frame.
type Anchors = std::collections::HashMap<ElementId, Rc<Cell<Option<Anchor>>>>;

thread_local! {
    /// The rooms of the owners being laid out now, innermost last.
    static ROOMS: RefCell<Vec<Bounds<Pixels>>> = const { RefCell::new(Vec::new()) };
    /// Each open layer's anchor (one per layer id the DM opened, so a handful).
    static ANCHORS: RefCell<Anchors> = RefCell::new(Anchors::new());
}

/// Starts an owner's room: an invisible first child as large as the owner (which must be
/// `relative`), less `right` at its right edge (a scroll bar's lane).
pub fn room_start(right: Pixels) -> impl IntoElement {
    canvas(
        move |bounds, _, _| {
            let mut room = bounds;
            room.size.width = (room.size.width - right).max(px(0.0));
            ROOMS.with(|rooms| rooms.borrow_mut().push(room));
        },
        |_, _, _, _| {},
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// Ends the owner's room: its last child.
pub fn room_end() -> impl IntoElement {
    canvas(
        |_, _, _| {
            ROOMS.with(|rooms| rooms.borrow_mut().pop());
        },
        |_, _, _, _| {},
    )
    .absolute()
    .size_0()
}

/// What the trigger recorded in this frame: its bounds and its room.
#[derive(Clone, Copy)]
struct Anchor {
    trigger: Bounds<Pixels>,
    room: Bounds<Pixels>,
}

/// Builds the layer once its room is known, with the widest and tallest it may be.
type Build = Box<dyn FnOnce(Pixels, Pixels) -> AnyElement>;

/// Open `layer` from `container`, the trigger's own box (made `relative`): the layer's room and
/// place come from the container's bounds. `match_width` makes the layer at least as wide as the
/// trigger (a drop-down's list). `layer(max_width, max_height)` builds the layer's element.
///
/// A deferred layer is laid out before its trigger is placed, so the layer is sized from the room
/// its trigger had in the last frame (kept under `id`) and placed from this frame's; when the two
/// differ (the frame it opens, a resize), it asks for one more frame.
pub fn attach(
    container: Div,
    id: &ElementId,
    match_width: bool,
    layer: impl FnOnce(Pixels, Pixels) -> AnyElement + 'static,
) -> Div {
    let anchor = ANCHORS.with(|anchors| {
        anchors
            .borrow_mut()
            .entry(id.clone())
            .or_insert_with(|| Rc::new(Cell::new(None)))
            .clone()
    });
    let record = anchor.clone();
    container
        .relative()
        .child(
            canvas(
                move |trigger, window, _| {
                    let room = ROOMS
                        .with(|rooms| rooms.borrow().last().copied())
                        .unwrap_or_else(|| window_room(window));
                    let room = room.intersect(&window_room(window));
                    record.set(Some(Anchor { trigger, room }));
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        )
        .child(
            deferred(Placed {
                anchor,
                match_width,
                build: Some(Box::new(layer)),
                child: None,
                shown: false,
            })
            .with_priority(1),
        )
}

/// The window, one `S` inside its edges.
fn window_room(window: &Window) -> Bounds<Pixels> {
    let size = window.viewport_size();
    Bounds::new(
        point(S, S),
        gpui::size(
            (size.width - S * 2.0).max(px(0.0)),
            (size.height - S * 2.0).max(px(0.0)),
        ),
    )
}

/// The room below and above the trigger, less the gap.
fn spaces(anchor: &Anchor) -> (Pixels, Pixels) {
    let below = anchor.room.bottom() - anchor.trigger.bottom() - XS;
    let above = anchor.trigger.top() - anchor.room.top() - XS;
    (below.max(px(0.0)), above.max(px(0.0)))
}

/// The widest, tallest and narrowest the layer may be from `anchor`.
fn limits(anchor: &Anchor, match_width: bool) -> (Pixels, Pixels, Pixels) {
    let (below, above) = spaces(anchor);
    let narrowest = if match_width {
        anchor.trigger.size.width.min(anchor.room.size.width)
    } else {
        px(0.0)
    };
    (anchor.room.size.width, below.max(above), narrowest)
}

/// Whether the trigger is out of its room's view (scrolled away).
fn out_of_view(anchor: &Anchor) -> bool {
    anchor.trigger.top() >= anchor.room.bottom() || anchor.trigger.bottom() <= anchor.room.top()
}

/// The layer, sized in layout from the last frame's anchor and placed in prepaint from this
/// frame's.
struct Placed {
    anchor: Rc<Cell<Option<Anchor>>>,
    match_width: bool,
    build: Option<Build>,
    child: Option<AnyElement>,
    /// Placed in this frame (its trigger is in view).
    shown: bool,
}

/// The child's layout and the limits it was laid out with.
pub struct PlacedLayout(Option<(LayoutId, (Pixels, Pixels, Pixels))>);

impl Element for Placed {
    type RequestLayoutState = PlacedLayout;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, PlacedLayout) {
        let Some(build) = self.build.take() else {
            let style = Style {
                position: Position::Absolute,
                ..Style::default()
            };
            return (window.request_layout(style, None, cx), PlacedLayout(None));
        };
        let sized = match self.anchor.get() {
            Some(anchor) => limits(&anchor, self.match_width),
            None => {
                let room = window_room(window);
                (room.size.width, room.size.height, px(0.0))
            }
        };
        let (widest, tallest, narrowest) = sized;
        // The layer is laid out inside its trigger's box, which would wrap its words to the
        // trigger's width: its own node is as wide as the room, a row in which the layer takes its
        // natural width, at most the room's.
        let style = Style {
            position: Position::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: Some(AlignItems::FlexStart),
            size: gpui::Size {
                width: widest.into(),
                height: gpui::Length::Auto,
            },
            ..Style::default()
        };
        let mut child = gpui::div()
            .min_w(narrowest)
            .child(build(widest, tallest))
            .into_any_element();
        let id = child.request_layout(window, cx);
        self.child = Some(child);
        (
            window.request_layout(style, Some(id), cx),
            PlacedLayout(Some((id, sized))),
        )
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut PlacedLayout,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (Some((id, sized)), Some(anchor), Some(child)) =
            (layout.0, self.anchor.get(), self.child.as_mut())
        else {
            return;
        };
        if limits(&anchor, self.match_width) != sized {
            // Laid out for another room: lay it out again in the next frame.
            window.refresh();
        }
        if out_of_view(&anchor) {
            return;
        }
        let size = window.layout_bounds(id).size;
        let (below, above) = spaces(&anchor);
        let room = anchor.room;
        // Under the trigger when it fits there, or when there is no more room above (egui:
        // `below >= whole_row && (natural <= below || below >= above)`).
        let y = if size.height <= below || below >= above {
            anchor.trigger.bottom() + XS
        } else {
            anchor.trigger.top() - XS - size.height
        };
        let x = anchor
            .trigger
            .left()
            .min(room.right() - size.width)
            .max(room.left());
        let offset = point(x, y) - bounds.origin;
        window.with_element_offset(point(offset.x.round(), offset.y.round()), |window| {
            child.prepaint(window, cx)
        });
        self.shown = true;
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut PlacedLayout,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.shown
            && let Some(child) = self.child.as_mut()
        {
            child.paint(window, cx);
        }
    }
}
