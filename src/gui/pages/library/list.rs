//! The Library's list: the scope drop-down, the sections column, the search and the entry rows
//! (a virtual list: only the rows in view draw), and the empty campaign scope.
use super::{footer, forms};
use crate::gui::theme::{Density, L, S, SCROLL_BAR_WIDTH, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use crate::library::model::{EmptyList, ListView};
use crate::lookup::ContentIndex;
use crate::gui::kit::WithTooltip as _;
use gpui::{
    AnyElement, Context, ElementId, InteractiveElement, IntoElement, ParentElement, ScrollStrategy,
    SharedString, Styled, Window, div, prelude::*, px,
};
use std::sync::Arc;

/// The scope drop-down (`Library::scope_menu`): what the catalogue shows. Choosing here never
/// changes the active campaign.
fn scope_menu(root: &SplatRoot, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let (choices, chosen, _group) = root.app.library.scope_choices(&root.app.campaigns);
    let scopes: Vec<_> = choices.iter().map(|(scope, _)| *scope).collect();
    kit::Dropdown::new(
        "library-scope",
        "What the Library shows",
        choices.into_iter().map(|(_, words)| words),
        Some(chosen),
    )
    .on_select(cx.listener(move |root, at: &usize, _, cx| {
        if let Some(scope) = scopes.get(*at) {
            root.app.library.choose_scope(*scope);
            cx.notify();
        }
    }))
}

/// A section row: its label, and its count at the right.
fn section_row(
    id: ElementId,
    label: &'static str,
    count: Option<usize>,
    selected: bool,
) -> kit::Row {
    let c = colors();
    kit::Row::new(id, label)
        .height(Density::Read.row())
        .selected(selected)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .ty(if selected { Type::Strong } else { Type::Body })
                .text_color(if selected { c.text } else { c.muted })
                .truncate()
                .child(label),
        )
        .children(count.map(|count| {
            kit::text(Type::Small, c.muted, count.to_string()).flex_none()
        }))
}

/// The package's About row.
fn about_row(index: usize, label: String, selected: bool, cx: &mut Context<SplatRoot>) -> kit::Row {
    let c = colors();
    kit::Row::new("library-about", label.clone())
        .height(Density::Read.row())
        .selected(selected)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .ty(if selected { Type::Strong } else { Type::Body })
                .text_color(if selected { c.text } else { c.muted })
                .truncate()
                .child(label),
        )
        .on_click(cx.listener(move |root, _, _, cx| {
            root.app.library.open_about(index);
            cx.notify();
        }))
}

/// The sections column of a wide page.
pub(super) fn sections_column(
    root: &mut SplatRoot,
    list: &ListView,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let mut rows = div().flex().flex_col().w_full();
    for (group_at, group) in list.groups.iter().enumerate() {
        if !group.title.is_empty() {
            rows = rows.child(kit::heading(
                kit::Heading::Group {
                    title: group.title,
                    count: group.count,
                },
                false,
            ));
        }
        for (row_at, row) in group.rows.iter().enumerate() {
            let (section, kind) = (row.section, row.kind);
            rows = rows.child(
                section_row(
                    ElementId::Name(format!("library-section-{group_at}-{row_at}").into()),
                    row.label,
                    row.painted.then_some(row.count),
                    row.selected,
                )
                .on_click(cx.listener(move |root, _, _, cx| {
                    root.app.library.choose_section(section, kind);
                    cx.notify();
                })),
            );
        }
    }
    div()
        .flex()
        .flex_col()
        .size_full()
        .gap(XS)
        .child(div().pr(SCROLL_BAR_WIDTH).child(scope_menu(root, cx)))
        .children(list.about.as_ref().map(|(index, label, selected)| {
            div()
                .pr(SCROLL_BAR_WIDTH)
                .child(about_row(*index, label.clone(), *selected, cx))
        }))
        .child(
            div()
                .flex_1()
                .min_h_0()
                .child(kit::scroll_column("library-sections", rows)),
        )
        .into_any_element()
}

/// A row's second line: a leading warning mark in its tone, then the words.
fn subtitle(words: &str) -> impl IntoElement {
    let c = colors();
    let (mark, rest) = match words.strip_prefix(icons::WARN) {
        Some(rest) => (true, rest.trim_start().to_owned()),
        None => (false, words.to_owned()),
    };
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .min_w_0()
        .when(mark, |line| {
            line.child(kit::icon(icons::WARN, Type::Small, c.result.warning))
        })
        .child(
            div()
                .ty(Type::Small)
                .text_color(c.muted)
                .truncate()
                .child(rest),
        )
}

/// The entries column: a compact page's drop-downs, the search, the heading and the rows.
pub(super) fn entries_column(
    root: &mut SplatRoot,
    list: &ListView,
    index: &Option<Arc<ContentIndex>>,
    compact: bool,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let mut column = div().flex().flex_col().size_full();
    if compact {
        let sections: Vec<_> = list.section_choices.iter().map(|(s, _)| *s).collect();
        let chosen = list
            .chosen_section
            .and_then(|chosen| sections.iter().position(|s| *s == chosen));
        let labels = list.section_choices.iter().map(|(section, label)| match section {
            crate::library::model::Section::All => "All",
            crate::library::model::Section::Characters => "Characters",
            _ => label,
        });
        column = column.child(
            div()
                .flex()
                .flex_row()
                .gap(S)
                .pb(XS)
                .child(div().flex_1().min_w_0().child(scope_menu(root, cx)))
                .when(!sections.is_empty(), |line| {
                    line.child(
                        div().flex_1().min_w_0().child(
                            kit::Dropdown::new("library-section", "Section", labels, chosen)
                                .on_select(cx.listener(move |root, at: &usize, _, cx| {
                                    if let Some(section) = sections.get(*at) {
                                        root.app.library.choose_section(*section, None);
                                        cx.notify();
                                    }
                                })),
                        ),
                    )
                }),
        );
    }
    let search = root.app.library.search_text().to_owned();
    column = column.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(
                div().flex_1().min_w_0().child(
                    kit::TextField::new("library-search", search.clone())
                        .placeholder("Search by name or detail…")
                        .name("Search the Library")
                        .on_change(cx.listener(|root, words: &str, _, cx| {
                            root.app.library.set_search(words);
                            cx.notify();
                        })),
                ),
            )
            .when(!search.is_empty(), |line| {
                line.child(
                    kit::Button::quiet("library-search-clear", "Clear").on_click(cx.listener(
                        |root, _, _, cx| {
                            root.app.library.set_search("");
                            cx.notify();
                        },
                    )),
                )
            }),
    );
    if compact && let Some((at, label, selected)) = &list.about {
        column = column.child(about_row(*at, label.clone(), *selected, cx));
    }
    match list.heading {
        Some((title, count)) => {
            column = column.child(kit::heading(kit::Heading::Group { title, count }, false));
        }
        None => column = column.child(kit::gap(XS)),
    }
    if list.close_names {
        column = column.child(kit::text(
            Type::Body,
            c.muted,
            crate::library::model::CLOSE_NAMES,
        ));
    }
    if let Some(empty) = &list.empty {
        if list.heading.is_none() {
            column = column.child(kit::gap(S));
        }
        column = column.child(match empty {
            EmptyList::Package { name, form: true } => div()
                .flex()
                .flex_col()
                .gap(S)
                .child(kit::icon(icons::LIBRARY, Type::Title, c.muted))
                .child(kit::text(
                    Type::Title,
                    c.text,
                    format!("No entries in {name} yet"),
                ))
                .into_any_element(),
            EmptyList::Package { name, form: false } => kit::empty_state(
                icons::LIBRARY,
                format!("No entries in {name} yet"),
                "Create the first entry.",
                Some(
                    kit::Button::secondary("library-empty-new", "New entry")
                        .icon(icons::ADD)
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.library.start_package_entry();
                            cx.notify();
                        })),
                ),
            )
            .into_any_element(),
            EmptyList::Words { words, wider } => div()
                .flex()
                .flex_col()
                .items_start()
                .pl(S)
                .child(kit::text(Type::Body, c.muted, words.clone()))
                .when(*wider, |block| {
                    block.child(
                        kit::Button::quiet(
                            "library-search-everything",
                            crate::library::SEARCH_EVERYTHING,
                        )
                        .density(Density::Read)
                        .line_start()
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.library.search_everything();
                            cx.notify();
                        })),
                    )
                })
                .into_any_element(),
        });
    }
    let handle = root.pages.library.list_scroll.clone();
    if list.reveal
        && let Some(row) = list.selected_row
    {
        handle.scroll_to_item(row, ScrollStrategy::Center);
    }
    let index = index.clone();
    let rows = kit::virtual_column(
        "library-entries",
        &handle,
        list.rows,
        cx.processor(move |root: &mut SplatRoot, range: std::ops::Range<usize>, _, cx| {
            range
                .filter_map(|row| {
                    let entry = root.app.library.entry_row(row, index.as_deref())?;
                    let note = entry.note;
                    let c = colors();
                    let name: SharedString = entry.name.clone().into();
                    Some(
                        div()
                            .id(ElementId::Name(format!("library-entry-{note}").into()))
                            .w_full()
                            .with_tooltip(entry.hover.clone())
                            .child(
                                kit::Row::new(
                                    ElementId::Name(format!("library-row-{note}").into()),
                                    format!("{} · {}", entry.name, entry.subtitle),
                                )
                                .height(px(crate::design::ENTRY_ROW_HEIGHT))
                                .selected(entry.selected)
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .flex_1()
                                        .min_w_0()
                                        .child(
                                            div()
                                                .ty(if entry.selected {
                                                    Type::Strong
                                                } else {
                                                    Type::Body
                                                })
                                                .text_color(c.text)
                                                .truncate()
                                                .child(name),
                                        )
                                        .child(subtitle(&entry.subtitle)),
                                )
                                .on_click(cx.listener(move |root, _, _, cx| {
                                    root.app.library.press_entry(note);
                                    cx.notify();
                                })),
                            ),
                    )
                })
                .collect()
        }),
    );
    column
        .child(div().flex_1().min_h_0().pt(XS).child(rows))
        .into_any_element()
}

/// The campaign scope with no entries of its own: the scope drop-down, then a pane with the New
/// entry form or the empty state, and the page's footer.
pub(super) fn empty_scope(
    root: &mut SplatRoot,
    _width: f32,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let (_, new_form) = root.app.library.forms();
    let body = if new_form {
        forms::new_entry(root, window, cx)
    } else {
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap(S)
            .child(kit::icon(icons::LIBRARY, Type::Title, c.muted))
            .child(kit::text(Type::Title, c.text, "No entries of your own yet"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .child(kit::text(Type::Body, c.muted, "Copy an "))
                    .child(
                        kit::link("library-can-use", "entry you can use")
                            .on_click(cx.listener(|root, _, _, cx| {
                                root.app.library.show_usable();
                                cx.notify();
                            })),
                    )
                    .child(kit::text(
                        Type::Body,
                        c.muted,
                        ", or make a new one here.",
                    )),
            )
            .child(
                kit::Button::primary("library-empty-scope-new", "New entry")
                    .icon(icons::ADD)
                    .on_click(cx.listener(|root, _, _, cx| {
                        root.app.library.start_entry();
                        cx.notify();
                    })),
            )
            .into_any_element()
    };
    let lines = root.app.library.form_footer(&root.app.campaigns);
    let mut pane = kit::Pane::new("library-empty-scope").child(body);
    for line in footer::render(root, lines, cx) {
        pane = pane.footer(line);
    }
    div()
        .flex()
        .flex_col()
        .size_full()
        .gap(L)
        .child(div().w(px(166.0)).child(scope_menu(root, cx)))
        .child(div().flex_1().min_h_0().child(pane))
        .into_any_element()
}
