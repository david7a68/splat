//! The creature filter: a Filter menu with a submenu for
//! each fact, a removable chip for each chosen value, and `Clear filters` when two or more are
//! chosen. The Library and the encounter's creature search share it.
//!
//! The caller keeps the filter (the Encounters page: in the state context) and gives the changed
//! filter back in
//! `on_change`:
//!
//! ```ignore
//! kit::FilterBar::new("library-filter", library.filter(), || library.filter_choices(), cx)
//!     .on_change(cx.listener(|root, filter: &Filter, _, cx| { root.app.library.set_filter(filter.clone()); cx.notify() }))
//! ```
use super::button::element_name;
use super::{Button, Checkbox, Popover, target};
use crate::gui::icons;
use crate::gui::theme::{
    Density, FOCUS_WIDTH, RADIUS_CONTROL, S, SCROLL_BAR_WIDTH, Type, TypeExt, XS, colors,
};
use gpui::{
    AnyElement, App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, prelude::*, px,
};
use gpui_component::scroll::ScrollableElement as _;
use splat_content::content::creature_filter::{Facet, Filter, challenge_value};
use splat_content_ui::creature_filter::{
    chips, choose_bound, facet_word, filter_word, range_words,
};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

/// Each fact's values among the creatures the filter counts, with how many have each
/// (`creature_filter::choices`).
pub type Choices = BTreeMap<Facet, Vec<(String, usize)>>;

type OnChange = Rc<dyn Fn(&Filter, &mut Window, &mut App)>;

/// The menu's width: room for a value and its count, and for the challenge ratings, which wrap.
const MENU_WIDTH: gpui::Pixels = px(280.0);

#[derive(IntoElement)]
pub struct FilterBar {
    id: ElementId,
    filter: Filter,
    choices: Option<Choices>,
    on_change: Option<OnChange>,
}

impl FilterBar {
    /// The bar of `filter`. `choices` gives the facts the menu offers; it runs only while the
    /// menu is open.
    pub fn new(
        id: impl Into<ElementId>,
        filter: &Filter,
        choices: impl FnOnce() -> Choices,
        cx: &App,
    ) -> Self {
        let id = id.into();
        let choices = super::menu::is_open(&Self::menu_id(&id), cx).then(choices);
        Self {
            id,
            filter: filter.clone(),
            choices,
            on_change: None,
        }
    }

    /// The id of the Filter menu of the bar `id` (for `kit::menu::is_open`).
    pub fn menu_id(id: &ElementId) -> ElementId {
        child(id, "menu")
    }

    /// The DM changed the filter: the handler gets the whole new filter.
    pub fn on_change(mut self, handler: impl Fn(&Filter, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

fn child(id: &ElementId, name: impl Into<SharedString>) -> ElementId {
    ElementId::NamedChild(Arc::new(id.clone()), name.into())
}

/// A press that changes the filter by `change` and hands the result to `on_change`.
fn press(
    filter: &Filter,
    on_change: &Option<OnChange>,
    change: impl Fn(&mut Filter) + 'static,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static {
    let (filter, on_change) = (filter.clone(), on_change.clone());
    move |_, window, cx| {
        let mut next = filter.clone();
        change(&mut next);
        if let Some(on_change) = &on_change {
            on_change(&next, window, cx);
        }
    }
}

/// A chosen value's chip: its words after a close icon; a press removes it.
fn chip(id: ElementId, label: &str) -> Button {
    Button::quiet(id, label.to_owned())
        .icon(icons::CLOSE)
        .density(Density::Scan)
        .selected(true)
        .name(format!("Remove filter {label}"))
}

impl RenderOnce for FilterBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let filter = &self.filter;
        let count = filter.count();
        let trigger = Button::quiet(
            child(&self.id, "button"),
            format!("{} {}", filter_word(filter), icons::DOWN),
        )
        // Not `line_start`: the bar starts at a pane's edge beside a splitter, and a hung frame
        // would touch it (every target clear of a splitter).
        .selected(count > 0)
        .name("Filter creatures");
        let menu = match &self.choices {
            Some(choices) => menu(&self.id, filter, choices, &self.on_change, window, cx),
            None => div().into_any_element(),
        };
        let mut bar = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(S)
            .child(Popover::new(Self::menu_id(&self.id), trigger, menu));
        for (facet, value, label) in chips(filter) {
            let id = child(&self.id, format!("chip:{}:{value}", facet.label()));
            bar = bar.child(chip(id, &label).on_click(press(
                filter,
                &self.on_change,
                move |filter| filter.toggle(facet, &value),
            )));
        }
        if filter.challenge != (None, None) {
            bar = bar.child(
                chip(
                    child(&self.id, "chip:challenge"),
                    &range_words(filter.challenge),
                )
                .on_click(press(filter, &self.on_change, |filter| {
                    filter.challenge = (None, None);
                })),
            );
        }
        if count > 1 {
            bar = bar.child(
                super::Link::new(child(&self.id, "clear"), "Clear filters")
                    .small()
                    .on_click(press(filter, &self.on_change, |filter| {
                        *filter = Filter::default();
                    })),
            );
        }
        bar
    }
}

/// The menu: a row for each fact the creatures have (`design::Submenus`), whose values open
/// under it, each on its own; the challenge opens two rows of ratings, Lowest and Highest.
fn menu(
    id: &ElementId,
    filter: &Filter,
    choices: &Choices,
    on_change: &Option<OnChange>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let c = colors();
    if choices.is_empty() {
        return div()
            .p(S)
            .child(super::text(Type::Small, c.muted, "No creatures to filter."))
            .into_any_element();
    }
    // The open facts, in the order they opened.
    let open = window.use_keyed_state(child(id, "open"), cx, |_, _| Vec::<Facet>::new());
    let opened = open.read(cx).clone();
    let mut rows = div().flex().flex_col().w(MENU_WIDTH);
    for facet in Facet::ALL {
        let Some(values) = choices.get(&facet) else {
            continue;
        };
        let here = opened.contains(&facet);
        let toggle = open.clone();
        let word: SharedString = facet_word(filter, facet).into();
        let row =
            div()
                .id(child(id, format!("fact:{}", facet.label())))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(Density::Read.control())
                .px(S)
                .rounded(RADIUS_CONTROL)
                .when(here, |row| row.bg(c.selected))
                .hover(|row| row.bg(c.hover))
                .cursor_pointer()
                .focusable()
                .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
                .aria_label(word.clone())
                .aria_expanded(here)
                .child(
                    div()
                        .ty(Type::Strong)
                        .text_color(c.text)
                        .child(word.clone()),
                )
                .child(div().icon_font(Type::Body).text_color(c.muted).child(
                    SharedString::new_static(if here { icons::DOWN } else { icons::NEXT }),
                ))
                .on_click(move |_, window, cx| {
                    toggle.update(cx, |open, _| {
                        if let Some(at) = open.iter().position(|f| *f == facet) {
                            open.remove(at);
                        } else {
                            open.push(facet);
                        }
                    });
                    window.refresh();
                });
        let row = target(row, word);
        rows = rows.child(target(
            row,
            element_name(&child(id, format!("fact:{}", facet.label()))),
        ));
        if here {
            let block = if facet == Facet::Challenge {
                challenge(id, filter, values, on_change)
            } else {
                values_block(id, facet, filter, values, on_change)
            };
            rows = rows.child(div().pl(S).pb(XS).child(block));
        }
    }
    // The menu scrolls its facts when the open ones are tall, with its bar in its own lane.
    let room = window.viewport_size().height * 0.6;
    div()
        .id(child(id, "facts"))
        .max_h(room)
        .overflow_y_scrollbar()
        .child(div().pr(SCROLL_BAR_WIDTH).child(rows))
        .into_any_element()
}

/// A fact's values as check boxes: `Underdark (12)`.
fn values_block(
    id: &ElementId,
    facet: Facet,
    filter: &Filter,
    values: &[(String, usize)],
    on_change: &Option<OnChange>,
) -> gpui::Div {
    let mut block = div().flex().flex_col();
    for (value, count) in values {
        let label = facet.label();
        let chosen = value.clone();
        block = block.child(
            Checkbox::new(
                child(id, format!("value:{label}:{value}")),
                format!("{value} ({count})"),
                filter.is_chosen(facet, value),
            )
            .name(format!("Filter {label}: {value}"))
            .on_click(press(filter, on_change, move |filter| {
                filter.toggle(facet, &chosen)
            })),
        );
    }
    block
}

/// Two rows of challenge ratings, the lowest and the highest. A second press on a bound removes
/// it.
fn challenge(
    id: &ElementId,
    filter: &Filter,
    values: &[(String, usize)],
    on_change: &Option<OnChange>,
) -> gpui::Div {
    let c = colors();
    let mut block = div().flex().flex_col().gap(XS);
    for (title, high) in [("Lowest", false), ("Highest", true)] {
        let bound = if high {
            filter.challenge.1
        } else {
            filter.challenge.0
        };
        let mut row = div().flex().flex_row().flex_wrap().gap(XS);
        for (value, _) in values {
            let Some(number) = challenge_value(value) else {
                continue;
            };
            row = row.child(
                Button::quiet(child(id, format!("cr:{title}:{value}")), value.clone())
                    .density(Density::Scan)
                    .selected(bound == Some(number))
                    .name(format!("{title} CR {value}"))
                    .on_click(press(filter, on_change, move |filter| {
                        choose_bound(filter, high, number)
                    })),
            );
        }
        block = block
            .child(super::text(Type::Small, c.muted, title))
            .child(row);
    }
    block
}
