//! The touchpad's scrolling speed (Settings → Touchpad, `touchpad::TouchpadSpeed`) in the GPUI
//! app. GPUI has no hook to scale scroll events, so the root takes each touchpad scroll before any
//! element sees it (the capture phase, first listener), drops it, and sends it again with its
//! distance multiplied by the chosen speed, once the current event is done. Every scroll area in
//! the window (kit scroll columns, virtual lists, text editors, gpui-component's own) then moves
//! by the same factor.
//!
//! Only a touchpad's scroll moves further: a precise scroll in pixels
//! (`ScrollDelta::Pixels`) from a pointing device. A mouse wheel's lines, a touchscreen's pan and
//! a scroll with Ctrl held (a zoom) keep their distance.
use crate::touchpad::TouchpadSpeed;
use gpui::{
    App, DispatchPhase, IntoElement, PlatformInput, ScrollDelta, ScrollEvent, ScrollSource, Styled,
    Window, canvas,
};
use std::cell::Cell;

thread_local! {
    /// The scroll being sent again, already at speed: the listener lets it through.
    static RESENT: Cell<bool> = const { Cell::new(false) };
}

/// Whether `event` is a touchpad scroll that `speed` moves further (a scroll in pixels without
/// the command key).
fn scales(event: &ScrollEvent, speed: TouchpadSpeed) -> bool {
    speed.factor() != 1.0
        && event.source == ScrollSource::Pointer
        && matches!(event.delta, ScrollDelta::Pixels(_))
        && !event.modifiers.control
        && !event.modifiers.platform
}

/// `event` at `speed`.
fn faster(event: &ScrollEvent, speed: TouchpadSpeed) -> ScrollEvent {
    let mut event = event.clone();
    if let ScrollDelta::Pixels(delta) = event.delta {
        event.delta = ScrollDelta::Pixels(delta * speed.factor());
    }
    event
}

/// An element of no size that, as it paints, puts the root's touchpad listener first in the
/// window's scroll listeners. Draw it as the root's first child.
pub(super) fn listener(speed: TouchpadSpeed) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |_, _, window, _| {
            window.on_mouse_event(move |event: &ScrollEvent, phase, window, cx| {
                if phase != DispatchPhase::Capture
                    || RESENT.with(Cell::get)
                    || !scales(event, speed)
                {
                    return;
                }
                cx.stop_propagation();
                let scaled = faster(event, speed);
                window.defer(cx, move |window, cx: &mut App| resend(scaled, window, cx));
            });
        },
    )
    .absolute()
    .size_0()
}

/// Send `event` to the window as if the platform sent it, marked as already at speed.
fn resend(event: ScrollEvent, window: &mut Window, cx: &mut App) {
    RESENT.with(|resent| resent.set(true));
    window.dispatch_event(PlatformInput::Scroll(event), cx);
    RESENT.with(|resent| resent.set(false));
}
