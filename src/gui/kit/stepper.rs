//! A whole number with − and + (egui `design::stepper`): `Decrease <name>`, the number in a well
//! the DM can type in, then `Increase <name>`. At a limit the − or + is disabled. A typed number
//! keeps to the range; a value already outside it only steps back towards it, never further, as
//! the egui stepper does.
//!
//! ```ignore
//! kit::Stepper::new(("hp", id), "Current HP", hp, 0..=max)
//!     .well(well)
//!     .on_change(cx.listener(move |root, value: &i64, _, cx| { …; cx.notify() }))
//! ```
use super::{IconButton, TextField};
use crate::gui::icons;
use crate::gui::theme::{Density, Type, XS};
use gpui::{
    App, ElementId, IntoElement, ParentElement, Pixels, RenderOnce, SharedString, Styled, Window,
    div, px,
};
use std::ops::RangeInclusive;
use std::rc::Rc;

/// The narrowest well, egui's `interact_size.x`.
pub const MIN_WELL: Pixels = px(40.0);

type OnValue = Rc<dyn Fn(&i64, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Stepper {
    id: ElementId,
    name: SharedString,
    value: i64,
    range: RangeInclusive<i64>,
    well: Pixels,
    disabled: bool,
    on_change: Option<OnValue>,
}

impl Stepper {
    /// `name` names the value: its − is `Decrease <name>`, its + `Increase <name>`.
    pub fn new(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        value: i64,
        range: RangeInclusive<i64>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            value,
            range,
            well: MIN_WELL,
            disabled: false,
            on_change: None,
        }
    }

    /// The well's width. A block of steppers gives each the same one ([`well_for`]), so every −
    /// and every + of the block share an x.
    pub fn well(mut self, well: Pixels) -> Self {
        self.well = well;
        self
    }

    /// Shown, not changeable (a sheet this sign-in cannot edit).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The new value, after a − or + or a typed number.
    pub fn on_change(mut self, handler: impl Fn(&i64, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

/// What a text field takes around its words: the input's own padding (10 px a side), the
/// frame's `XS` a side, a focus border a side, and room for the caret.
const FIELD_CHROME: Pixels = px(36.0);

/// A well that holds `digits` characters of the stepper's number, never below [`MIN_WELL`].
pub fn well_for(window: &Window, digits: usize) -> Pixels {
    (super::measure::digit_width(window, Type::Body) * digits as f32 + FIELD_CHROME).max(MIN_WELL)
}

/// The characters a whole number shows: its digits, and a minus sign when it is negative.
pub fn digits(value: i64) -> usize {
    value
        .unsigned_abs()
        .checked_ilog10()
        .map_or(1, |log| log as usize + 1)
        + usize::from(value < 0)
}

/// A typed number, with a true minus or a hyphen.
fn parse(words: &str) -> Option<i64> {
    words.trim().replace('\u{2212}', "-").parse().ok()
}

impl RenderOnce for Stepper {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let value = self.value;
        let (low, high) = (
            (*self.range.start()).min(value),
            (*self.range.end()).max(value),
        );
        let child = |part: &'static str| {
            ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), part.into())
        };
        let send = |to: i64, on_change: Option<OnValue>| {
            move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                if let Some(on_change) = &on_change {
                    on_change(&to, window, cx);
                }
            }
        };
        let typed = self.on_change.clone();
        div()
            .flex()
            .flex_row()
            .flex_none()
            .items_center()
            .gap(XS / 2.0)
            .h(Density::Read.control())
            .child(
                IconButton::new(
                    child("less"),
                    icons::SUBTRACT,
                    format!("Decrease {}", self.name),
                )
                .disabled(self.disabled || value <= low)
                .on_click(send(value - 1, self.on_change.clone())),
            )
            .child(
                TextField::new(child("well"), value.to_string())
                    .name(self.name.clone())
                    .width(self.well)
                    .disabled(self.disabled)
                    .on_change(move |words: &str, window, cx| {
                        let Some(number) = parse(words) else {
                            return;
                        };
                        let number = number.clamp(low, high);
                        if number != value
                            && let Some(on_change) = &typed
                        {
                            on_change(&number, window, cx);
                        }
                    }),
            )
            .child(
                IconButton::new(child("more"), icons::ADD, format!("Increase {}", self.name))
                    .disabled(self.disabled || value >= high)
                    .on_click(send(value + 1, self.on_change)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_and_typed_numbers() {
        assert_eq!(digits(0), 1);
        assert_eq!(digits(145), 3);
        assert_eq!(digits(-12), 3);
        assert_eq!(parse(" −3 "), Some(-3));
        assert_eq!(parse("12"), Some(12));
        assert_eq!(parse("x"), None);
    }
}
