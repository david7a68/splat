//! A whole number with − and + (egui `design::stepper`): `Decrease <name>`, the number in a well
//! the DM can type in, then `Increase <name>`. At a limit the − or + is disabled. A typed number
//! keeps to the range; a value already outside it only steps back towards it, never further, as
//! the egui stepper does. `.change()` shows a change (`+3`, `−1`) whose steps skip 0 (egui
//! `change_stepper`); `.grouped()` a count with its digits grouped (`1,000`, egui
//! `grouped_stepper`).
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

/// How the well shows its number (egui `Shown`).
#[derive(Clone, Copy, Default, PartialEq)]
enum Shown {
    /// `3`
    #[default]
    Plain,
    /// `+3`, `−1`: a change, whose steps skip 0.
    Change,
    /// `1,000`: a count with its digits grouped.
    Grouped,
}

#[derive(IntoElement)]
pub struct Stepper {
    id: ElementId,
    name: SharedString,
    value: i64,
    range: RangeInclusive<i64>,
    well: Pixels,
    disabled: bool,
    shown: Shown,
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
            shown: Shown::Plain,
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

    /// A change (`+3`, `−1`): its − and + step over 0, which changes nothing (egui
    /// `change_stepper`).
    pub fn change(mut self) -> Self {
        self.shown = Shown::Change;
        self
    }

    /// A count that can reach the thousands (a group's member count): `1,000`, typed with or
    /// without the commas (egui `grouped_stepper`).
    pub fn grouped(mut self) -> Self {
        self.shown = Shown::Grouped;
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

/// A typed number, with a true minus or a hyphen, a `+`, and commas or spaces between digits.
fn parse(words: &str) -> Option<i64> {
    crate::design::parse_signed(
        &words
            .chars()
            .filter(|c| *c != ',' && !c.is_whitespace())
            .collect::<String>(),
    )
}

impl Shown {
    /// The words the well shows for `value`.
    fn words(self, value: i64) -> String {
        match self {
            Self::Plain => value.to_string(),
            Self::Change => crate::design::signed(value),
            Self::Grouped => {
                let digits = crate::design::grouped_number(value.unsigned_abs() as usize);
                if value < 0 {
                    format!("\u{2212}{digits}")
                } else {
                    digits
                }
            }
        }
    }

    /// One step from `value` by `by` (±1): a change steps over 0.
    fn step(self, value: i64, by: i64) -> i64 {
        let next = value + by;
        if self == Self::Change && next == 0 {
            next + by
        } else {
            next
        }
    }
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
        let shown = self.shown;
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
                .on_click(send(
                    shown.step(value, -1).clamp(low, high),
                    self.on_change.clone(),
                )),
            )
            .child(
                TextField::new(child("well"), shown.words(value))
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
                    .on_click(send(shown.step(value, 1).clamp(low, high), self.on_change)),
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
        assert_eq!(parse("+2"), Some(2));
        assert_eq!(parse("1,000"), Some(1000));
        assert_eq!(Shown::Change.words(-1), "\u{2212}1");
        assert_eq!(Shown::Change.words(3), "+3");
        assert_eq!(Shown::Grouped.words(12_345), "12,345");
        assert_eq!(Shown::Change.step(1, -1), -1);
        assert_eq!(Shown::Change.step(-1, 1), 1);
        assert_eq!(Shown::Plain.step(1, -1), 0);
    }
}
