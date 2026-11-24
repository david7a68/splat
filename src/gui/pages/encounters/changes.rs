//! A creature's Changes, for a roster row and a group's member:
//! its heading with Done, the display name, each number with how it changes and its preview, the
//! side, the starting conditions, the note and the creature's own loot. Every edit goes to
//! `Encounters::edit_changes`, which keeps the block one the form can read.
use super::loot::{self, LootOwner};
use crate::gui::theme::{Density, FORM_WIDTH, L, S, Type, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use gpui::{Context, Div, ElementId, ParentElement, Pixels, Styled, div, px};
use splat_encounters_ui::encounters::model::{
    ChangeEdit, ChangeHow, ChangeRowView, ChangesForm, ChangesOf, SideView,
};

/// A label's width before its field (`LABEL_WIDTH`).
const LABEL_WIDTH: Pixels = px(120.0);

/// A number's well: a signed amount of a few digits.
const NUMBER_WELL: Pixels = px(56.0);

fn label(words: &'static str) -> Div {
    div()
        .w(LABEL_WIDTH)
        .flex_none()
        .child(kit::text(Type::Small, colors().muted, words))
}

/// A handler that sends one edit of this form.
fn edit(
    cx: &mut Context<SplatRoot>,
    of: &ChangesOf,
    make: impl Fn() -> ChangeEdit + 'static,
) -> impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static {
    let of = of.clone();
    cx.listener(move |root: &mut SplatRoot, _, _, cx| {
        root.app.encounters.edit_changes(&of, make());
        cx.notify();
    })
}

/// The form, under its row.
pub(super) fn form(
    root: &SplatRoot,
    of: ChangesOf,
    form: ChangesForm,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let key = form.key.clone();
    let heading = div()
        .flex()
        .flex_row()
        .items_center()
        .min_h(Density::Read.control())
        .child(div().flex_1().child(kit::label(form.title.clone(), 0)))
        .child(
            kit::Button::quiet(
                ElementId::Name(format!("changes-done:{key}").into()),
                "Done",
            )
            .name("Done with changes")
            .density(Density::Read)
            .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                root.app.encounters.close_form();
                cx.notify();
            })),
        );
    let mut column = div().flex().flex_col().gap(XS).pt(XS).pb(S).child(heading);
    if let Some(words) = form.unreadable {
        return column.child(kit::Message::new(kit::Tone::Error, words).small());
    }
    if let Some(words) = form.top_fault.clone() {
        column = column.child(kit::Message::new(kit::Tone::Error, words).small());
    }
    column = column.child(text_row(
        &of,
        (
            &key,
            "Display name",
            form.name.clone(),
            form.name_hint.clone(),
        ),
        ChangeEdit::Name,
        cx,
    ));
    for row in &form.rows {
        column = column.child(change_row(&of, &key, row, cx));
    }
    match &form.side {
        SideView::Hidden => {}
        SideView::Row { side, sides } => {
            column = column.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(S)
                    .child(label("Side"))
                    .child(super::common::side_choice(
                        root,
                        (format!("changes-side:{key}"), "Changed side"),
                        side,
                        sides,
                        {
                            let of = of.clone();
                            move |root: &mut SplatRoot, side: &str| {
                                root.app
                                    .encounters
                                    .edit_changes(&of, ChangeEdit::Side(Some(side.to_owned())));
                            }
                        },
                        cx,
                    ))
                    .child(
                        kit::Button::quiet(
                            ElementId::Name(format!("changes-same-row:{key}").into()),
                            "Same as row",
                        )
                        .density(Density::Read)
                        .on_click(edit(cx, &of, || ChangeEdit::Side(None))),
                    ),
            );
        }
        SideView::Group { side, changed } => {
            let mut row = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(label("Side"))
                .child(super::common::side_choice(
                    root,
                    (format!("changes-side:{key}"), "Changed side"),
                    side,
                    &[],
                    {
                        let of = of.clone();
                        move |root: &mut SplatRoot, side: &str| {
                            root.app
                                .encounters
                                .edit_changes(&of, ChangeEdit::Side(Some(side.to_owned())));
                        }
                    },
                    cx,
                ));
            if *changed {
                row = row.child(
                    kit::Button::quiet(
                        ElementId::Name(format!("changes-same-group:{key}").into()),
                        "Same as group",
                    )
                    .density(Density::Read)
                    .on_click(edit(cx, &of, || ChangeEdit::Side(None))),
                );
            }
            column = column.child(row);
        }
    }
    if let Some(conditions) = &form.conditions {
        column = column
            .child(kit::gap(S))
            .child(kit::label("Starting conditions", 0))
            .child(conditions_block(root, &of, &key, conditions, cx));
    }
    column = column.child(text_row(
        &of,
        (
            &key,
            "Note",
            form.note.clone(),
            "Why this creature differs".to_owned(),
        ),
        ChangeEdit::Note,
        cx,
    ));
    column = column
        .child(kit::gap(S))
        .child(kit::label("Loot", 0))
        .child(div().max_w(FORM_WIDTH + px(160.0)).child(loot::list(
            root,
            format!("changes:{key}"),
            form.loot_name.clone(),
            form.loot.clone().into_iter().collect(),
            LootOwner::Changes(of),
            cx,
        )));
    let _ = c;
    column
}

/// A text row: its label, then its field (`text_row`).
fn text_row(
    of: &ChangesOf,
    (key, label_words, value, hint): (&str, &'static str, String, String),
    make: fn(String) -> ChangeEdit,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let of = of.clone();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(label(label_words))
        .child(
            div().w(px(320.0)).min_w(px(120.0)).child(
                kit::TextField::new(
                    ElementId::Name(format!("changes-{label_words}:{key}").into()),
                    value,
                )
                .name(label_words)
                .placeholder(hint)
                .on_change(cx.listener(
                    move |root: &mut SplatRoot, words: &str, _, cx| {
                        root.app
                            .encounters
                            .edit_changes(&of, make(words.to_owned()));
                        cx.notify();
                    },
                )),
            ),
        )
}

/// One number: its label, how it changes, its value or amount, its preview, and its fault
/// under its own field (`change_row`).
fn change_row(of: &ChangesOf, key: &str, row: &ChangeRowView, cx: &mut Context<SplatRoot>) -> Div {
    let c = colors();
    let label_words = row.label;
    let mut hows = vec![(ChangeHow::Same, "Same")];
    if row.can_set {
        hows.push((ChangeHow::Set, "Set to"));
    }
    hows.push((ChangeHow::Add, "Add"));
    let chosen = hows.iter().position(|(how, _)| *how == row.how);
    let picked: Vec<ChangeHow> = hows.iter().map(|(how, _)| *how).collect();
    let how_of = of.clone();
    let mut line = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(S)
        .child(label(label_words))
        .child(
            kit::Dropdown::new(
                ElementId::Name(format!("changes-how:{label_words}:{key}").into()),
                format!("{label_words}: how it changes"),
                hows.iter().map(|(_, words)| *words).collect::<Vec<_>>(),
                chosen,
            )
            .width(px(86.0))
            .density(Density::Read)
            .on_select(cx.listener(move |root: &mut SplatRoot, at: &usize, _, cx| {
                if let Some(how) = picked.get(*at) {
                    root.app
                        .encounters
                        .edit_changes(&how_of, ChangeEdit::How(label_words, *how));
                }
                cx.notify();
            })),
        );
    if row.how != ChangeHow::Same {
        let word = if row.how == ChangeHow::Set {
            "value"
        } else {
            "amount"
        };
        let number_of = of.clone();
        let mut stepper = kit::Stepper::new(
            ElementId::Name(format!("changes-number:{label_words}:{key}").into()),
            format!("{label_words}: {word}"),
            row.number,
            row.range.clone(),
        )
        .well(NUMBER_WELL)
        .on_change(
            cx.listener(move |root: &mut SplatRoot, number: &i64, _, cx| {
                root.app
                    .encounters
                    .edit_changes(&number_of, ChangeEdit::Number(label_words, *number));
                cx.notify();
            }),
        );
        if row.how == ChangeHow::Add {
            // Zero changes nothing: an amount's steps go over it (`change_stepper`).
            stepper = stepper.change();
        }
        line = line.child(stepper);
    }
    if let Some(preview) = &row.preview {
        line = line.child(match row.fault {
            // An invalid value marks the place where the DM typed it.
            Some(_) => kit::text(
                Type::Small,
                c.result.error,
                format!("{} {preview}", icons::ERROR),
            ),
            None => kit::text(Type::Small, c.muted, preview.clone()),
        });
    }
    let mut block = div().flex().flex_col().gap(XS).child(line);
    if let Some(fault) = &row.fault {
        // The reason sits under its own field, at the field's left edge.
        block = block.child(
            div()
                .pl(LABEL_WIDTH + S)
                .child(kit::Message::new(kit::Tone::Error, fault.clone()).small()),
        );
    }
    block
}

/// The run's condition choices, the Other effects with Remove, and the field that adds one
/// (`conditions_row`).
fn conditions_block(
    root: &SplatRoot,
    of: &ChangesOf,
    key: &str,
    conditions: &[String],
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let same = splat_content::content::overrides::same_condition;
    let mut grid = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap(XS)
        .max_w(FORM_WIDTH);
    for (_, name) in splat_party::combat_sync::CONDITIONS.iter() {
        let selected = conditions.iter().any(|c| same(c, name));
        let name = *name;
        grid = grid.child(
            div().w(px(170.0)).flex_none().child(
                kit::Button::quiet(
                    ElementId::Name(format!("changes-condition:{name}:{key}").into()),
                    name,
                )
                .icon(if selected { icons::OK } else { icons::ADD })
                .density(Density::Read)
                .selected(selected)
                .on_click(edit(cx, of, move || ChangeEdit::Condition(name.to_owned()))),
            ),
        );
    }
    let official = |c: &str| {
        splat_party::combat_sync::CONDITIONS
            .iter()
            .any(|(_, name)| same(c, name))
    };
    let mut column = div().flex().flex_col().gap(XS).child(grid);
    let customs: Vec<&String> = conditions.iter().filter(|c| !official(c)).collect();
    if !customs.is_empty() {
        column = column
            .child(kit::gap(S))
            .child(kit::label("Other effects", 0));
    }
    for custom in customs {
        let gone = custom.clone();
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .max_w(FORM_WIDTH)
                .min_h(Density::Read.control())
                .child(div().flex_1().min_w_0().child(kit::text(
                    Type::Body,
                    c.text,
                    custom.clone(),
                )))
                .child(super::common::row_menu(
                    ElementId::Name(format!("changes-effect:{custom}:{key}").into()),
                    custom,
                    vec![
                        kit::MenuItem::new("Remove")
                            .icon(icons::REMOVE)
                            .on_click(edit(cx, of, move || {
                                ChangeEdit::RemoveCondition(gone.clone())
                            })),
                    ],
                )),
        );
    }
    let typed = root
        .pages
        .encounters
        .other_effects
        .get(key)
        .cloned()
        .unwrap_or_default();
    let add = {
        let (of, key) = (of.clone(), key.to_owned());
        move |root: &mut SplatRoot| {
            let words = root
                .pages
                .encounters
                .other_effects
                .remove(&key)
                .unwrap_or_default();
            if !words.trim().is_empty() {
                root.app
                    .encounters
                    .edit_changes(&of, ChangeEdit::AddCondition(words));
            }
        }
    };
    let typing = key.to_owned();
    let click = add.clone();
    column.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(XS)
            .max_w(FORM_WIDTH)
            .child(
                div().flex_1().child(
                    kit::TextField::new(
                        ElementId::Name(format!("changes-other-effect:{key}").into()),
                        typed.clone(),
                    )
                    .name("Other effect")
                    .placeholder("Other effect…")
                    .on_change(
                        cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                            root.pages
                                .encounters
                                .other_effects
                                .insert(typing.clone(), words.to_owned());
                            cx.notify();
                        }),
                    )
                    .on_enter(super::editor::on_key(cx, add)),
                ),
            )
            .child(
                kit::Button::secondary(
                    ElementId::Name(format!("changes-other-effect-add:{key}").into()),
                    "Add",
                )
                .density(Density::Read)
                .blocked(typed.trim().is_empty().then_some("Type an effect first."))
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    click(root);
                    cx.notify();
                })),
            ),
    )
}

/// A row's or member's missing-creature picker (`replace_missing_creature`): a search, `Replace
/// all` when several rows name it, and five creatures, each with `Use`.
pub(super) fn replacement(
    root: &SplatRoot,
    key: &str,
    count: usize,
    name: &str,
    on_use: impl Fn(&mut SplatRoot, uuid::Uuid, bool) + Clone + 'static,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let (query, all) = root
        .pages
        .encounters
        .replacing
        .get(key)
        .cloned()
        .unwrap_or_else(|| (String::new(), true));
    let rows = root.app.encounters.replacement_rows(&query);
    let typing = key.to_owned();
    let mut block = div().flex().flex_col().gap(XS).pb(S).child(
        kit::TextField::new(
            ElementId::Name(format!("replacement-search:{key}").into()),
            query,
        )
        .name("Replacement creature")
        .placeholder("Choose another creature…")
        .on_change(
            cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                root.pages
                    .encounters
                    .replacing
                    .entry(typing.clone())
                    .or_insert_with(|| (String::new(), true))
                    .0 = words.to_owned();
                cx.notify();
            }),
        ),
    );
    if count > 1 {
        let toggling = key.to_owned();
        block = block.child(
            kit::Checkbox::new(
                ElementId::Name(format!("replacement-all:{key}").into()),
                format!("Replace all {count} {name} rows"),
                all,
            )
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                let entry = root
                    .pages
                    .encounters
                    .replacing
                    .entry(toggling.clone())
                    .or_insert_with(|| (String::new(), true));
                entry.1 ^= true;
                cx.notify();
            })),
        );
    }
    for row in rows {
        let id = row.id;
        let use_it = on_use.clone();
        let detail = super::editor::stat_cells(&row.detail, row.hp_missing);
        block = block.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(L)
                .min_h(Density::Scan.row())
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w_0()
                        .child(kit::text(Type::Body, c.text, row.name.clone()))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .ty_small_muted()
                                .children(detail)
                                .children(row.source.clone().map(|s| div().ml(XS).child(s))),
                        ),
                )
                .child(
                    kit::Button::secondary(
                        ElementId::Name(format!("replacement-use:{key}:{id}").into()),
                        "Use",
                    )
                    .name(format!("Use {}", row.name))
                    .density(Density::Read)
                    .blocked(row.unavailable.clone())
                    .on_click(cx.listener(
                        move |root: &mut SplatRoot, _, _, cx| {
                            use_it(root, id, all);
                            cx.notify();
                        },
                    )),
                ),
        );
    }
    block
}

trait SmallMuted {
    fn ty_small_muted(self) -> Self;
}

impl SmallMuted for Div {
    fn ty_small_muted(self) -> Self {
        use crate::gui::theme::TypeExt as _;
        self.ty(Type::Small)
            .text_color(colors().muted)
            .whitespace_nowrap()
            .overflow_hidden()
    }
}
