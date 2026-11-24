//! Menus, drop-downs and popovers (`design::menu_popup`, `Dropdown`, `row_menu`).
//!
//! - `MenuButton`: a button that opens a menu of `MenuItem`s under it (a row's `⋯`, `New ▾`).
//! - `Dropdown`: a choice of one value from a list, shown in a field-like frame with a caret (the
//!   campaign switcher, a kind).
//! - `Popover`: any content in a raised layer under a trigger.
//!
//! One layer is open at a time. The kit keeps which one in a GPUI global (`OpenPopup`), so a
//! caller keeps no open flag: a press on the trigger opens or closes it, a press outside closes
//! it, a choice closes it, and the root's Escape closes it (`close_open_popup`). The layer keeps
//! inside the window, one `S` clear of its edges.
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::icons;
use crate::gui::theme::{
    Density, FOCUS_WIDTH, HAIRLINE_WIDTH, ICON_SLOT, RADIUS_CONTROL, RADIUS_LAYER, S, Type,
    TypeExt, XS, colors,
};
use gpui::{
    AnyElement, App, ClickEvent, Div, ElementId, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Point, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, anchored, deferred, div, prelude::*, px,
};
use std::rc::Rc;

/// The drop-down frame's padding above and below its words (egui's combo box: a two-line name
/// fills 39 px).
const DROPDOWN_PADDING: Pixels = px(2.5);

/// Which layer is open, and where the press that last closed one was (so that press, when it
/// lands on the same trigger, does not open it again).
#[derive(Default)]
pub struct OpenPopup {
    open: Option<ElementId>,
    closed_at: Option<Point<Pixels>>,
}

impl Global for OpenPopup {}

/// Whether the layer of `id` is open.
pub fn is_open(id: &ElementId, cx: &App) -> bool {
    cx.try_global::<OpenPopup>()
        .is_some_and(|popup| popup.open.as_ref() == Some(id))
}

/// Whether any kit layer is open (a modal test, as egui's `Popup::is_any_open`).
pub fn any_open(cx: &App) -> bool {
    cx.try_global::<OpenPopup>()
        .is_some_and(|popup| popup.open.is_some())
}

/// Close the open layer, if any; true when one closed. The root calls it on Escape first.
pub fn close_open_popup(window: &mut Window, cx: &mut App) -> bool {
    let popup = cx.default_global::<OpenPopup>();
    let closed = popup.open.take().is_some();
    if closed {
        window.refresh();
    }
    closed
}

/// Open the layer of `id` from code (a key that opens a menu).
pub fn open_popup(id: impl Into<ElementId>, window: &mut Window, cx: &mut App) {
    cx.default_global::<OpenPopup>().open = Some(id.into());
    window.refresh();
}

fn toggle(id: &ElementId, event: &ClickEvent, window: &mut Window, cx: &mut App) {
    let popup = cx.default_global::<OpenPopup>();
    let this_press = event.down_position();
    if popup
        .closed_at
        .take()
        .is_some_and(|at| Some(at) == this_press)
    {
        // The press that closed this layer: it stays closed.
        window.refresh();
        return;
    }
    popup.open = if popup.open.as_ref() == Some(id) {
        None
    } else {
        Some(id.clone())
    };
    window.refresh();
}

fn close(window: &mut Window, cx: &mut App) {
    cx.default_global::<OpenPopup>().open = None;
    window.refresh();
}

trait DownPosition {
    fn down_position(&self) -> Option<Point<Pixels>>;
}

impl DownPosition for ClickEvent {
    fn down_position(&self) -> Option<Point<Pixels>> {
        match self {
            ClickEvent::Mouse(mouse) => Some(mouse.down.position),
            _ => None,
        }
    }
}

/// The raised layer under a trigger.
fn layer(id: &ElementId, min_width: Option<Pixels>, content: impl IntoElement) -> impl IntoElement {
    let c = colors();
    let closing = id.clone();
    // `anchored` starts at its parent's origin: a box of no height under the trigger puts the
    // layer's top at the trigger's bottom edge.
    div().w_full().h(px(0.0)).child(
        deferred(
            anchored().snap_to_window_with_margin(S).child(
                div()
                    .id(ElementId::NamedChild(
                        std::sync::Arc::new(closing),
                        SharedString::new_static("layer"),
                    ))
                    .mt(XS)
                    .when_some(min_width, |layer, width| layer.min_w(width))
                    .p(XS)
                    .bg(c.raised)
                    .border(HAIRLINE_WIDTH)
                    .border_color(c.rule)
                    .rounded(RADIUS_LAYER)
                    .shadow_lg()
                    .occlude()
                    .on_mouse_down_out(|event, window, cx| {
                        let popup = cx.default_global::<OpenPopup>();
                        popup.open = None;
                        popup.closed_at = Some(event.position);
                        window.refresh();
                    })
                    .child(content),
            ),
        )
        .with_priority(1),
    )
}

/// One choice in a menu: its words, an optional icon, and a reason when it is blocked (shown as
/// a note under its words, as `choice_note` does).
pub struct MenuItem {
    id: SharedString,
    label: SharedString,
    icon: Option<&'static str>,
    checked: Option<bool>,
    blocked: Option<SharedString>,
    destructive: bool,
    on_click: Option<OnClick>,
}

impl MenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            id: label.clone(),
            label,
            icon: None,
            checked: None,
            blocked: None,
            destructive: false,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    /// A choice that is on or off (a filter): a check before its words when on.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Disabled, with its reason under its words.
    pub fn blocked(mut self, reason: Option<impl Into<SharedString>>) -> Self {
        self.blocked = reason.map(Into::into);
        self
    }

    /// Red words (Remove, Delete).
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    fn render(self, menu: &ElementId, icon_slot: bool) -> AnyElement {
        let c = colors();
        let enabled = self.blocked.is_none();
        let id = ElementId::NamedChild(std::sync::Arc::new(menu.clone()), self.id.clone());
        let words_color = if !enabled {
            c.disabled
        } else if self.destructive {
            c.destructive
        } else {
            c.text
        };
        let mark = self
            .icon
            .or(self.checked.filter(|on| *on).map(|_| icons::OK));
        let mut row = div()
            .id(id.clone())
            .flex()
            .flex_row()
            .items_center()
            .min_h(Density::Read.control())
            .px(S)
            .rounded(RADIUS_CONTROL)
            .text_color(words_color)
            .focusable()
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.label.clone())
            .when(icon_slot, |row| {
                row.child(
                    div()
                        .w(ICON_SLOT)
                        .flex_none()
                        .icon_font(Type::Body)
                        .when_some(mark, |slot, mark| {
                            slot.child(SharedString::new_static(mark))
                        }),
                )
            })
            .child(
                div()
                    .ty(Type::Body)
                    .whitespace_nowrap()
                    .child(self.label.clone()),
            );
        if enabled {
            row = row.cursor_pointer().hover(|s| s.bg(c.hover));
            if let Some(on_click) = self.on_click {
                row = row.on_click(move |event, window, cx| {
                    close(window, cx);
                    on_click(event, window, cx);
                });
            }
        }
        let mut item = div()
            .flex()
            .flex_col()
            .child(target(row, self.label.clone()));
        if let Some(reason) = self.blocked {
            item = item.child(
                div()
                    .pl(S + if icon_slot { ICON_SLOT } else { px(0.0) })
                    .pr(S)
                    .pb(XS)
                    .ty(Type::Small)
                    .text_color(c.muted)
                    .child(reason),
            );
        }
        target(item, element_name(&id)).into_any_element()
    }
}

/// The menu's column of items.
fn items(menu: &ElementId, items: Vec<MenuItem>) -> Div {
    let icon_slot = items
        .iter()
        .any(|item| item.icon.is_some() || item.checked.is_some());
    div()
        .flex()
        .flex_col()
        .children(items.into_iter().map(|item| item.render(menu, icon_slot)))
}

/// A button that opens a menu under it.
///
/// ```ignore
/// kit::MenuButton::new(("row-menu", id), kit::Button::quiet(("row-more", id), "⋯").name("More actions for X"))
///     .item(kit::MenuItem::new("Rename").on_click(...))
/// ```
#[derive(IntoElement)]
pub struct MenuButton {
    id: ElementId,
    trigger: super::Button,
    items: Vec<MenuItem>,
}

impl MenuButton {
    /// `trigger` is a kit button; its own `on_click` is replaced by the toggle.
    pub fn new(id: impl Into<ElementId>, trigger: super::Button) -> Self {
        Self {
            id: id.into(),
            trigger,
            items: Vec::new(),
        }
    }

    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for MenuButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let open = is_open(&self.id, cx);
        let id = self.id.clone();
        let trigger = self
            .trigger
            .on_click(move |event, window, cx| toggle(&id, event, window, cx));
        div().flex().flex_col().child(trigger).when(open, |menu| {
            menu.child(layer(&self.id, None, items(&self.id, self.items)))
        })
    }
}

/// A choice of one value from a list (`design::Dropdown`): the chosen words (or the placeholder in
/// faint) and a caret in a field-like frame; its menu lists every choice with a check on the
/// chosen one.
#[derive(IntoElement)]
pub struct Dropdown {
    id: ElementId,
    options: Vec<SharedString>,
    chosen: Option<usize>,
    placeholder: SharedString,
    name: SharedString,
    blocked: Option<SharedString>,
    width: Option<Pixels>,
    density: Density,
    extra: Vec<MenuItem>,
    on_select: Option<super::OnIndex>,
}

impl Dropdown {
    pub fn new(
        id: impl Into<ElementId>,
        name: impl Into<SharedString>,
        options: impl IntoIterator<Item = impl Into<SharedString>>,
        chosen: Option<usize>,
    ) -> Self {
        Self {
            id: id.into(),
            options: options.into_iter().map(Into::into).collect(),
            chosen,
            placeholder: SharedString::default(),
            name: name.into(),
            blocked: None,
            width: None,
            density: Density::Read,
            extra: Vec::new(),
            on_select: None,
        }
    }

    /// Words shown while nothing is chosen.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Disabled with a reason on hover.
    pub fn blocked(mut self, reason: Option<impl Into<SharedString>>) -> Self {
        self.blocked = reason.map(Into::into);
        self
    }

    /// A fixed width (default: its parent's).
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    pub fn density(mut self, density: Density) -> Self {
        self.density = density;
        self
    }

    /// A command after the choices (`Add campaign…`).
    pub fn command(mut self, item: MenuItem) -> Self {
        self.extra.push(item);
        self
    }

    /// The index the DM chose. Fits `cx.listener(|root, index: &usize, window, cx| …)`.
    pub fn on_select(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Dropdown {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = colors();
        let open = is_open(&self.id, cx);
        let enabled = self.blocked.is_none();
        let shown = self
            .chosen
            .and_then(|index| self.options.get(index).cloned());
        let (words, words_color) = match shown {
            Some(words) => (words, c.text),
            None => (self.placeholder.clone(), c.faint),
        };
        let toggle_id = self.id.clone();
        let mut trigger = div()
            .id(ElementId::NamedChild(
                std::sync::Arc::new(self.id.clone()),
                SharedString::new_static("trigger"),
            ))
            .flex()
            .flex_row()
            .items_center()
            .gap(XS)
            .min_h(self.density.control())
            .px(S)
            .py(DROPDOWN_PADDING)
            .rounded(RADIUS_CONTROL)
            .border(HAIRLINE_WIDTH)
            .border_color(c.field_border)
            .focusable()
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.name.clone())
            .aria_expanded(open)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .ty(Type::Small)
                    .text_color(words_color)
                    .child(words.clone()),
            )
            .child(
                div()
                    .flex_none()
                    .icon_font(Type::Body)
                    .text_color(c.text)
                    .child(SharedString::new_static(icons::DOWN)),
            );
        trigger = match self.width {
            Some(width) => trigger.w(width),
            None => trigger.w_full(),
        };
        if let Some(reason) = self.blocked.clone() {
            trigger = trigger
                .opacity(super::button::DISABLED_OPACITY)
                .tooltip(tip(reason));
        } else {
            trigger = trigger
                .cursor_pointer()
                .hover(|s| s.bg(c.hover))
                .on_click(move |event, window, cx| toggle(&toggle_id, event, window, cx));
        }
        let trigger = target(trigger, self.name.clone());
        let trigger = target(trigger, element_name(&self.id));
        let mut choices: Vec<MenuItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(index, option)| {
                let on_select = self.on_select.clone();
                let mut item = MenuItem::new(option.clone()).checked(self.chosen == Some(index));
                item.id = format!("choice-{index}").into();
                item.on_click(move |_, window, cx| {
                    if let Some(on_select) = &on_select {
                        on_select(&index, window, cx);
                    }
                })
            })
            .collect();
        choices.extend(self.extra);
        div()
            .flex()
            .flex_col()
            .when_some(self.width, |column, width| column.w(width))
            .when(self.width.is_none(), |column| column.w_full())
            .child(trigger)
            .when(open && enabled, |column| {
                column.child(layer(&self.id, self.width, items(&self.id, choices)))
            })
    }
}

/// Any content in a raised layer under a trigger (a form that opens from a button, a picker).
/// `Popover::on_status` opens it from a status link instead (a count of faults whose list opens
/// under it).
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    trigger: Trigger,
    content: AnyElement,
}

/// What a popover opens from.
enum Trigger {
    Button(super::Button),
    Status(super::status::StatusLink),
}

impl Popover {
    pub fn new(
        id: impl Into<ElementId>,
        trigger: super::Button,
        content: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            trigger: Trigger::Button(trigger),
            content: content.into_any_element(),
        }
    }

    /// A popover under a status link (`kit::status_link`); its own `on_click` is replaced by the
    /// toggle.
    pub fn on_status(
        id: impl Into<ElementId>,
        trigger: super::status::StatusLink,
        content: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            trigger: Trigger::Status(trigger),
            content: content.into_any_element(),
        }
    }
}

impl RenderOnce for Popover {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let open = is_open(&self.id, cx);
        let id = self.id.clone();
        let toggle = move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
            toggle(&id, event, window, cx)
        };
        let trigger = match self.trigger {
            Trigger::Button(button) => button.on_click(toggle).into_any_element(),
            Trigger::Status(link) => link.on_click(toggle).into_any_element(),
        };
        div()
            .flex()
            .flex_col()
            .child(trigger)
            .when(open, |popover| {
                popover.child(layer(&self.id, None, self.content))
            })
    }
}

/// A status that opens a menu under it (`TitleLine::status`): the page header's
/// `2 notes need fixing`, whose menu lists each note. The status keeps its icon, word and colour;
/// the press opens or closes the menu as a `MenuButton`'s does.
#[derive(IntoElement)]
pub struct StatusMenu {
    id: ElementId,
    status: super::Status,
    items: Vec<MenuItem>,
}

impl StatusMenu {
    pub fn new(id: impl Into<ElementId>, status: super::Status) -> Self {
        Self {
            id: id.into(),
            status,
            items: Vec::new(),
        }
    }

    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for StatusMenu {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let open = is_open(&self.id, cx);
        let id = self.id.clone();
        let trigger = super::status_link(
            ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "trigger".into()),
            self.status,
        )
        .on_click(move |event, window, cx| toggle(&id, event, window, cx));
        div().flex().flex_col().child(trigger).when(open, |menu| {
            menu.child(layer(&self.id, None, items(&self.id, self.items)))
        })
    }
}
