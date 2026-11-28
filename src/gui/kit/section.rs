//! A caret section (`design::section`, `design::Section`; the quiet one is `QuietSection`): a `Strong` title after a caret that
//! points right while closed and down while open; the body shows under the title while open.
//! The caller keeps whether it is open when it gives `on_toggle` (its state, so a reopened page
//! keeps it); without `on_toggle` the section keeps it under its id, starting at `open`.
//!
//! ```ignore
//! kit::Section::new("cookie-help", "How to get your sign-in cookie", state.cookie_help)
//!     .on_toggle(cx.listener(|root, open: &bool, _, cx| { root.pages.settings.cookie_help = *open; cx.notify() }))
//!     .child(kit::text(Type::Body, c.text, "1. Sign in…"))
//! ```
use super::{button::element_name, target};
use crate::gui::icons;
use crate::gui::theme::{
    Density, FOCUS_WIDTH, ICON_SLOT, RADIUS_CONTROL, S, Type, TypeExt, XS, colors,
};
use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, prelude::*,
};
use std::rc::Rc;

type OnToggle = Rc<dyn Fn(&bool, &mut Window, &mut App)>;

#[derive(IntoElement)]
pub struct Section {
    id: ElementId,
    title: SharedString,
    open: bool,
    blocked: Option<SharedString>,
    on_toggle: Option<OnToggle>,
    warning: bool,
    body: Vec<AnyElement>,
}

impl Section {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>, open: bool) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            open,
            blocked: None,
            on_toggle: None,
            warning: false,
            body: Vec::new(),
        }
    }

    /// The title cannot open or close now, with the reason on hover.
    pub fn blocked(mut self, reason: Option<impl Into<SharedString>>) -> Self {
        self.blocked = reason.map(Into::into);
        self
    }

    /// A collapsing content issue count (`issue_count_title`): the warning icon and colour
    /// before the title, so the count reads as a status on every page.
    pub fn warning(mut self) -> Self {
        self.warning = true;
        self
    }

    /// The DM opened (`true`) or closed it.
    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Section {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}

impl RenderOnce for Section {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = colors();
        let first = self.open;
        let kept = self.on_toggle.is_none().then(|| {
            window.use_keyed_state(
                ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "open".into()),
                cx,
                move |_, _| first,
            )
        });
        let open = kept.as_ref().map_or(self.open, |kept| *kept.read(cx));
        let enabled = self.blocked.is_none();
        let (ty, color) = if self.warning {
            (Type::Strong, c.result.warning)
        } else {
            (Type::Strong, c.text)
        };
        let mut header = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .h(Density::Read.control())
            .pr(S)
            .rounded(RADIUS_CONTROL)
            .text_color(color)
            .focusable()
            .tab_stop(enabled)
            .role(gpui::Role::Button)
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.title.clone())
            .aria_expanded(open)
            .child(div().icon_font(Type::Small).w(ICON_SLOT).flex_none().child(
                SharedString::new_static(if open { icons::DOWN } else { icons::NEXT }),
            ))
            .when(self.warning, |header| {
                header.child(
                    div()
                        .icon_font(ty)
                        .pr(XS)
                        .flex_none()
                        .child(SharedString::new_static(icons::WARN)),
                )
            })
            .child(div().ty(ty).child(self.title.clone()));
        if let Some(reason) = self.blocked.clone() {
            header = header
                .opacity(super::button::DISABLED_OPACITY)
                .tooltip(super::tooltip::tip(reason));
        } else {
            let on_toggle = self.on_toggle;
            header = header
                .cursor_pointer()
                .hover(|s| s.text_color(c.text))
                .on_click(move |_, window, cx| match (&kept, &on_toggle) {
                    (Some(kept), _) => {
                        kept.update(cx, |open, _| *open = !*open);
                        window.refresh();
                    }
                    (None, Some(on_toggle)) => on_toggle(&!open, window, cx),
                    (None, None) => {}
                });
        }
        let header = target(header, self.title.clone());
        let header = target(header, element_name(&self.id));
        let _ = enabled;
        div().flex().flex_col().child(header).when(open, |section| {
            section.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(XS)
                    .pl(ICON_SLOT)
                    .children(self.body),
            )
        })
    }
}
