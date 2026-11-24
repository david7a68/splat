//! Buttons: `Button` (primary, secondary, quiet, destructive) and `IconButton`.
//!
//! The rules (`docs/design-system.md`, Widgets, Buttons):
//! - One primary action per view, filled with the accent. Secondary has a field-border edge, quiet
//!   has no edge, destructive has a red edge and red words.
//! - A blocked action is disabled and says why on hover (`.blocked(Some(reason))`); a page that
//!   blocks one control also shows the reason beside it in `Small` words.
//! - A quiet action that starts a line of text uses `.line_start()`: its words stay at the text
//!   edge and its padding (and icon) hang in the gutter.
//! - Every target is at least 24 px; the default height is a `Scan` control (30 px).
use super::{OnClick, target, tooltip::tip};
use crate::gui::theme::{
    Density, FOCUS_WIDTH, HAIRLINE_WIDTH, ICON_SLOT, M, RADIUS_CONTROL, S, Type, TypeExt, XS,
    colors,
};
use gpui::{
    App, ClickEvent, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement, Pixels,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Primary,
    Secondary,
    Quiet,
    Destructive,
}

/// A button with words, and an optional icon before them and key after them.
///
/// ```ignore
/// kit::Button::primary("add-campaign", "Add campaign")
///     .on_click(cx.listener(|root, _, _, cx| root.add_campaign(cx)))
/// ```
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    name: Option<SharedString>,
    kind: Kind,
    icon: Option<&'static str>,
    icon_after: Option<&'static str>,
    blocked: Option<SharedString>,
    hint: Option<SharedString>,
    height: Pixels,
    key: Option<SharedString>,
    selected: bool,
    plain: bool,
    line_start: bool,
    full_width: bool,
    on_click: Option<OnClick>,
}

impl Button {
    fn new(kind: Kind, id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            name: None,
            kind,
            icon: None,
            icon_after: None,
            blocked: None,
            hint: None,
            height: Density::Scan.control(),
            key: None,
            selected: false,
            plain: false,
            line_start: false,
            full_width: false,
            on_click: None,
        }
    }

    /// The one main action of a view: accent fill, `Heading` words.
    pub fn primary(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self::new(Kind::Primary, id, label)
    }

    /// Any other action: a field-border edge, `Strong` words.
    pub fn secondary(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self::new(Kind::Secondary, id, label)
    }

    /// A light action among words or in a row: no edge, `S` padding.
    pub fn quiet(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self::new(Kind::Quiet, id, label)
    }

    /// An action that loses data: a red edge and red words. In a dialog it goes at the left, `L`
    /// before the others (`kit::Dialog`).
    pub fn destructive(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self::new(Kind::Destructive, id, label)
    }

    /// An icon after the words: a menu button's caret.
    pub fn icon_after(mut self, icon: &'static str) -> Self {
        self.icon_after = Some(icon);
        self
    }

    /// An icon before the words (`gui::icons`).
    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    /// The accessible name, when it says more than the words.
    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Disabled with a reason, said on hover. `None` leaves it enabled.
    pub fn blocked(mut self, reason: Option<impl Into<SharedString>>) -> Self {
        self.blocked = reason.map(Into::into);
        self
    }

    /// Words on hover that say more than the label.
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// The control's height for its density (default `Scan`).
    pub fn density(mut self, density: Density) -> Self {
        self.height = density.control();
        self
    }

    /// The key that also presses it, shown after the words (`Ctrl+S`).
    pub fn key(mut self, key: impl Into<SharedString>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Shown as chosen (a toggle that is on): the selected fill and a field edge.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// `Body` words instead of `Strong`.
    pub fn plain(mut self) -> Self {
        self.plain = true;
        self
    }

    /// A quiet action that starts a line: its words at the text edge, its padding and icon in the
    /// gutter.
    pub fn line_start(mut self) -> Self {
        self.line_start = true;
        self
    }

    /// As wide as its parent.
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }

    /// What a press does.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    fn font(&self) -> Type {
        if self.kind == Kind::Primary {
            Type::Heading
        } else if self.plain {
            Type::Body
        } else {
            Type::Strong
        }
    }

    fn padding(&self) -> Pixels {
        if self.kind == Kind::Quiet { S } else { M }
    }
}

/// The fill, edge and word colours of a kind, at rest and on hover and press.
struct Look {
    fill: Hsla,
    hover: Hsla,
    pressed: Hsla,
    edge: Option<Hsla>,
    words: Hsla,
}

fn look(kind: Kind, selected: bool, enabled: bool) -> Look {
    let c = colors();
    let clear = gpui::transparent_black();
    let plain = Look {
        fill: clear,
        hover: c.hover,
        pressed: c.selected,
        edge: None,
        words: c.text,
    };
    if selected {
        return Look {
            fill: c.selected,
            hover: c.selected,
            edge: Some(c.field_border),
            ..plain
        };
    }
    match kind {
        Kind::Primary if !enabled => Look {
            edge: Some(c.field_border),
            words: c.muted,
            ..plain
        },
        Kind::Primary => Look {
            fill: c.accent_fill,
            hover: c.accent_hover,
            pressed: c.accent_hover,
            edge: None,
            words: c.on_accent,
        },
        Kind::Secondary => Look {
            edge: Some(c.field_border),
            ..plain
        },
        Kind::Quiet => plain,
        Kind::Destructive => Look {
            edge: Some(c.destructive),
            words: c.destructive,
            ..plain
        },
    }
}

impl RenderOnce for Button {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let enabled = self.blocked.is_none();
        let look = look(self.kind, self.selected, enabled);
        let padding = self.padding();
        let name = self.name.clone().unwrap_or_else(|| self.label.clone());
        let hover_words = self.blocked.clone().or(self.hint.clone());
        // A `line_start` action hangs its padding, and its icon, left of the text edge.
        let hang = if self.line_start {
            padding
                + if self.icon.is_some() {
                    ICON_SLOT
                } else {
                    px(0.0)
                }
        } else {
            px(0.0)
        };
        let mut button = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .flex_none()
            .items_center()
            .h(self.height)
            .min_w(self.height)
            .px(padding)
            .ml(-hang)
            .rounded(RADIUS_CONTROL)
            .bg(look.fill)
            .ty(self.font())
            .text_color(look.words)
            .whitespace_nowrap()
            .overflow_hidden()
            .focusable()
            .tab_stop(enabled)
            .role(gpui::Role::Button)
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(name.clone())
            .when_some(hover_words.clone(), |button, words| {
                button.aria_description(words)
            });
        if self.full_width {
            button = button.w_full().ml(px(0.0));
        }
        if let Some(edge) = look.edge {
            button = button.border(HAIRLINE_WIDTH).border_color(edge);
        }
        if let Some(icon) = self.icon {
            button = button.child(
                div()
                    .icon_font(Type::Body)
                    .w(ICON_SLOT)
                    .flex_none()
                    .child(SharedString::new_static(icon)),
            );
        }
        button = button.child(self.label.clone());
        if let Some(icon) = self.icon_after {
            button = button.child(
                div()
                    .icon_font(Type::Body)
                    .pl(XS)
                    .flex_none()
                    .child(SharedString::new_static(icon)),
            );
        }
        if let Some(key) = self.key.clone() {
            button = button.child(div().pl(S).ty(Type::Small).text_color(c.faint).child(key));
        }
        if let Some(words) = hover_words {
            button = button.tooltip(tip(words));
        }
        if enabled {
            let (hover, pressed) = (look.hover, look.pressed);
            button = button
                .cursor_pointer()
                .hover(move |s| s.bg(hover))
                .active(move |s| s.bg(pressed));
            if let Some(on_click) = self.on_click {
                button = button.on_click(move |event, window, cx| on_click(event, window, cx));
            }
        } else {
            button = button.opacity(DISABLED_OPACITY);
        }
        let button = target(button, self.label.clone());
        // A button whose accessible name says more than its words (`Add Long rest`, `Change
        // Place`) is found by that name too.
        let button = match self.name.clone().filter(|name| *name != self.label) {
            Some(name) => target(button, name),
            None => button,
        };
        target(button, element_name(&self.id))
    }
}

/// A disabled control is drawn at this opacity.
pub const DISABLED_OPACITY: f32 = 0.6;

/// The harness name of an element id (`id:<id>`).
pub fn element_name(id: &ElementId) -> SharedString {
    SharedString::from(format!("id:{id}"))
}

/// An icon target (a row's `⋯`, a close `×`), one `Read` control square. Its name is its
/// accessible name and its hover words; an icon button never stands for an action that needs
/// words (`design::icon_button`).
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: &'static str,
    name: SharedString,
    hint: Option<SharedString>,
    on: Option<bool>,
    disabled: bool,
    on_click: Option<OnClick>,
}

impl IconButton {
    pub fn new(
        id: impl Into<ElementId>,
        icon: &'static str,
        name: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            name: name.into(),
            hint: None,
            on: None,
            disabled: false,
            on_click: None,
        }
    }

    /// Shown but not pressable (a stepper's − at its lowest value): drawn at the disabled
    /// opacity, its press ignored. Its hover words stay.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Hover words that say more than the name (its key).
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// A view toggle: shown on (selected fill) or off.
    pub fn toggled(mut self, on: bool) -> Self {
        self.on = Some(on);
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let size = Density::Read.control();
        let on = self.on == Some(true);
        let hint = self.hint.clone().unwrap_or_else(|| self.name.clone());
        let mut button = div()
            .id(self.id.clone())
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(size)
            .rounded(RADIUS_CONTROL)
            .icon_font(Type::Heading)
            .text_color(if on { c.text } else { c.muted })
            .when(on, |s| s.bg(c.selected))
            .focusable()
            .tab_stop(true)
            .role(gpui::Role::Button)
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.name.clone())
            .tooltip(tip(hint))
            .child(SharedString::new_static(self.icon));
        if self.disabled {
            button = button.opacity(DISABLED_OPACITY);
        } else {
            button = button
                .cursor_pointer()
                .hover(|s| s.bg(c.hover).text_color(c.text));
            if let Some(on_click) = self.on_click {
                button = button.on_click(move |event, window, cx| on_click(event, window, cx));
            }
        }
        let button = target(button, self.name.clone());
        target(button, element_name(&self.id))
    }
}
