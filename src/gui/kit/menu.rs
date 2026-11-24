//! Menus, drop-downs and popovers (`design::menu_popup`, `Dropdown`, `row_menu`).
//!
//! - `MenuButton`: a button that opens a menu of `MenuItem`s under it (a row's `⋯`, `New ▾`).
//! - `Dropdown`: a choice of one value from a list, shown in a field-like frame with a caret (the
//!   campaign switcher, a kind).
//! - `Popover`: any content in a raised layer under a trigger.
//!
//! One layer is open at a time. The kit keeps which one in a GPUI global (`OpenPopup`), so a
//! caller keeps no open flag: a press on the trigger opens or closes it, a press outside closes
//! it, a choice closes it, and the root's Escape closes it (`close_open_popup`). The layer opens
//! under its trigger, or above it when there is more room there, inside the pane or column that
//! holds the trigger (`kit::layer`); a long one scrolls.
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::icons;
use crate::gui::theme::{
    Density, FOCUS_WIDTH, HAIRLINE_WIDTH, ICON_SLOT, RADIUS_CONTROL, RADIUS_LAYER, S, Type,
    TypeExt, XS, colors,
};
use gpui::{
    AnyElement, App, ClickEvent, Div, ElementId, Global, InteractiveElement, IntoElement,
    ParentElement, Pixels, Point, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, div, prelude::*, px,
};
use std::rc::Rc;

/// The drop-down frame's padding above and below its words (a two-line name fills 39 px).
const DROPDOWN_PADDING: Pixels = px(2.5);

/// Which layer is open, and the layer that the last press outside closed with that press's
/// place (so that press, when it lands on the same layer's trigger, does not open it again; on
/// another trigger it opens that one).
#[derive(Default)]
pub struct OpenPopup {
    open: Option<ElementId>,
    closed: Option<(ElementId, Point<Pixels>)>,
    /// The open layer was drawn from its trigger, in view, in the last frame.
    drawn: bool,
    /// The open layer opened since the last frame began, so it has not been drawn yet.
    fresh: bool,
}

impl Global for OpenPopup {}

/// As each frame starts (the root calls it): a layer that stayed open but was not drawn in the
/// last frame closes. Its trigger left the window (a row a reload removed, a form that closed)
/// or was scrolled out of its room, and an egui popup closed when its button was not drawn. So
/// no invisible menu keeps the global keys refused or eats the next press.
pub fn begin_frame(window: &mut Window, cx: &mut App) {
    let popup = cx.default_global::<OpenPopup>();
    if popup.open.is_some() && !popup.fresh && !popup.drawn {
        popup.open = None;
        window.refresh();
    }
    popup.drawn = false;
    popup.fresh = false;
}

/// The open layer `id` was placed in view in this frame (`kit::layer`).
pub(super) fn mark_drawn(id: &ElementId, cx: &mut App) {
    let popup = cx.default_global::<OpenPopup>();
    if popup.open.as_ref() == Some(id) {
        popup.drawn = true;
    }
}

/// Whether the layer of `id` is open.
pub fn is_open(id: &ElementId, cx: &App) -> bool {
    cx.try_global::<OpenPopup>()
        .is_some_and(|popup| popup.open.as_ref() == Some(id))
}

/// Whether any kit layer is open (a modal test).
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
    let popup = cx.default_global::<OpenPopup>();
    popup.open = Some(id.into());
    popup.fresh = true;
    window.refresh();
}

fn toggle(id: &ElementId, event: &ClickEvent, window: &mut Window, cx: &mut App) {
    let popup = cx.default_global::<OpenPopup>();
    let this_press = event.down_position();
    if popup
        .closed
        .take()
        .is_some_and(|(closed, at)| closed == *id && Some(at) == this_press)
    {
        // The press that closed this layer: it stays closed.
        window.refresh();
        return;
    }
    popup.open = if popup.open.as_ref() == Some(id) {
        None
    } else {
        popup.fresh = true;
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

/// The raised layer's frame around `content`: at most as wide and tall as its room allows, and
/// it scrolls past that. A press outside it closes it.
fn frame(id: ElementId, content: AnyElement, widest: Pixels, tallest: Pixels) -> AnyElement {
    let c = colors();
    let layer = id.clone();
    div()
        .id(ElementId::NamedChild(
            std::sync::Arc::new(id),
            SharedString::new_static("layer"),
        ))
        .max_w(widest)
        .max_h(tallest)
        .overflow_y_scroll()
        .p(XS)
        .bg(c.raised)
        .border(HAIRLINE_WIDTH)
        .border_color(c.rule)
        .rounded(RADIUS_LAYER)
        .shadow_lg()
        .occlude()
        .role(gpui::Role::Menu)
        .on_mouse_down_out(move |event, window, cx| {
            let popup = cx.default_global::<OpenPopup>();
            popup.open = None;
            popup.closed = Some((layer.clone(), event.position));
            window.refresh();
        })
        .child(content)
        .into_any_element()
}

/// `container` (the trigger's box) with the layer of `id` open from it (`kit::layer::attach`).
/// `match_width`: the layer is at least as wide as the trigger (a drop-down's list).
fn open_from(container: Div, id: &ElementId, match_width: bool, content: impl IntoElement) -> Div {
    let layer = id.clone();
    let content = content.into_any_element();
    super::layer::attach(container, id, match_width, move |widest, tallest| {
        frame(layer, content, widest, tallest)
    })
}

/// The raised layer of `id` for a menu that opens from code, such as a row's right-click menu
/// (`open_popup`): draw it after the element it belongs to while `is_open(id)`. It opens under the
/// place it is drawn, or above it when there is more room there.
pub fn layer_for(id: &ElementId, content: impl IntoElement) -> impl IntoElement {
    open_from(div().w_full().h(px(0.0)), id, false, content)
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
    rule_before: bool,
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
            rule_before: false,
            on_click: None,
        }
    }

    /// A rule above this item: it starts a new group.
    pub fn rule_before(mut self) -> Self {
        self.rule_before = true;
        self
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
            .tab_stop(enabled)
            .role(gpui::Role::MenuItem)
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
            // The words keep one line while the layer's room allows; a narrow room wraps them
            //.
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .ty(Type::Body)
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
            .when(self.rule_before, |item| {
                item.child(div().py(XS).child(super::rule()))
            })
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
    heading: Option<SharedString>,
    items: Vec<MenuItem>,
}

impl MenuButton {
    /// `trigger` is a kit button; its own `on_click` is replaced by the toggle.
    pub fn new(id: impl Into<ElementId>, trigger: super::Button) -> Self {
        Self {
            id: id.into(),
            trigger,
            heading: None,
            items: Vec::new(),
        }
    }

    /// Muted words above the items that say what they apply to (`This roll only`).
    pub fn heading(mut self, words: impl Into<SharedString>) -> Self {
        self.heading = Some(words.into());
        self
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
        let heading = self.heading.map(|words| {
            div()
                .px(S)
                .py(XS)
                .ty(Type::Small)
                .text_color(colors().muted)
                .child(words)
        });
        let menu = div().flex().flex_col().child(trigger);
        if open {
            open_from(
                menu,
                &self.id,
                false,
                div()
                    .flex()
                    .flex_col()
                    .children(heading)
                    .child(items(&self.id, self.items)),
            )
        } else {
            menu
        }
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
    groups: Vec<usize>,
    locks: Vec<(usize, SharedString)>,
    locked_look: bool,
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
            groups: Vec::new(),
            locks: Vec::new(),
            locked_look: false,
            on_select: None,
        }
    }

    /// The choices from `index` on start a new group, under a rule (`Everything` apart from
    /// `D&D 2024` and the packages).
    pub fn group_at(mut self, index: usize) -> Self {
        self.groups.push(index);
        self
    }

    /// The choice at `index` cannot be chosen now, with its reason on its row (`Already linked
    /// to another campaign.`).
    pub fn blocked_choice(mut self, index: usize, reason: impl Into<SharedString>) -> Self {
        self.locks.push((index, reason.into()));
        self
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

    /// While blocked, it shows a lock in place of its caret and keeps its words in the text
    /// colour, so the DM reads the value and sees that it cannot change now (the campaign
    /// switcher during a run or a draft). The reason stays on hover.
    pub fn lock_when_blocked(mut self) -> Self {
        self.locked_look = true;
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
            .tab_stop(enabled)
            .role(gpui::Role::ComboBox)
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
            .child({
                let locked = !enabled && self.locked_look;
                div()
                    .flex_none()
                    .icon_font(Type::Body)
                    .text_color(if locked { c.muted } else { c.text })
                    .child(SharedString::new_static(if locked {
                        icons::READ_ONLY
                    } else {
                        icons::DOWN
                    }))
            });
        trigger = match self.width {
            Some(width) => trigger.w(width),
            None => trigger.w_full(),
        };
        if let Some(reason) = self.blocked.clone() {
            if !self.locked_look {
                trigger = trigger.opacity(super::button::DISABLED_OPACITY);
            }
            trigger = trigger.tooltip(tip(reason));
        } else {
            trigger = trigger
                .cursor_pointer()
                .hover(|s| s.bg(c.hover))
                .on_click(move |event, window, cx| toggle(&toggle_id, event, window, cx));
        }
        let trigger = target(trigger, self.name.clone());
        let trigger = target(trigger, element_name(&self.id));
        // The harness finds a locked drop-down by its name and `(locked)`.
        let trigger = if !enabled && self.locked_look {
            target(trigger, format!("{} (locked)", self.name))
        } else {
            trigger
        };
        let mut choices: Vec<MenuItem> = self
            .options
            .iter()
            .enumerate()
            .map(|(index, option)| {
                let on_select = self.on_select.clone();
                let mut item = MenuItem::new(option.clone()).checked(self.chosen == Some(index));
                item.id = format!("choice-{index}").into();
                item.rule_before = index > 0 && self.groups.contains(&index);
                item.blocked = self
                    .locks
                    .iter()
                    .find(|(at, _)| *at == index)
                    .map(|(_, reason)| reason.clone());
                item.on_click(move |_, window, cx| {
                    if let Some(on_select) = &on_select {
                        on_select(&index, window, cx);
                    }
                })
            })
            .collect();
        choices.extend(self.extra);
        let column = div()
            .flex()
            .flex_col()
            .when_some(self.width, |column, width| column.w(width))
            .when(self.width.is_none(), |column| column.w_full())
            .child(trigger);
        if open && enabled {
            open_from(column, &self.id, true, items(&self.id, choices))
        } else {
            column
        }
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
        let popover = div().flex().flex_col().child(trigger);
        if open {
            open_from(popover, &self.id, false, self.content)
        } else {
            popover
        }
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
        let menu = div().flex().flex_col().child(trigger);
        if open {
            open_from(menu, &self.id, false, items(&self.id, self.items))
        } else {
            menu
        }
    }
}
