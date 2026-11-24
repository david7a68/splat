//! The loot list (egui: `loot_editor::show`, `show_cell`): each line a row (a roll on a table with
//! its count and table menu, a Library item that opens, or words in a field), each with its `⋯`
//! and Remove; the Undo of the last removal; the add field with the campaign's tables and items
//! offered as the DM types, and `Roll on a table` beside it. For an encounter the lines are its
//! `## Loot` list; for one creature's Changes they are one cell whose parts `;` separates.
use super::common::reason;
use crate::gui::theme::{Density, L, S, Type, XL, XS, XXL, colors};
use crate::gui::{SplatRoot, icons, kit};
use gpui::{
    AnyElement, Context, Div, ElementId, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, div, px,
};
use splat_content::content::loot::{Loot, MAX_ROLLS, SEPARATOR, parts};
use splat_encounters_ui::encounters::model::{ChangeEdit, ChangesOf};
use splat_encounters_ui::loot_editor::{self, ROLL_MENU, Sources};
use std::sync::Arc;

/// What one list keeps between frames, by its key (`loot_editor::State`).
#[derive(Clone, Default)]
pub struct LootState {
    /// The add field's words.
    add: String,
    /// The line whose field the DM is typing in: it stays a field until it loses focus.
    editing: Option<usize>,
    /// A count being typed, by line.
    count: Option<(usize, String)>,
    /// The last removed line, for its Undo.
    removed: Option<(usize, String)>,
    reason: Option<String>,
    /// A creature's lines as typed (spaces and empty rows kept until they change the loot).
    cell: Option<Vec<String>>,
}

/// Whose loot a list edits.
#[derive(Clone)]
pub(super) enum LootOwner {
    /// The open encounter's `## Loot` list.
    Encounter,
    /// One creature's Changes: one cell.
    Changes(ChangesOf),
}

impl LootOwner {
    fn one_cell(&self) -> bool {
        matches!(self, Self::Changes(_))
    }
}

/// The list's lines now: the encounter's, or a creature's cell as the DM typed it.
fn shown_lines(state: Option<&LootState>, owner: &LootOwner, stored: &[String]) -> Vec<String> {
    match owner {
        LootOwner::Encounter => stored.to_vec(),
        LootOwner::Changes(_) => {
            let now = parts(&stored.join(";"));
            state
                .and_then(|state| state.cell.clone())
                .filter(|kept| parts(&kept.join(";")) == now)
                .unwrap_or(now)
        }
    }
}

/// Write the lines to their owner.
fn write(root: &mut SplatRoot, key: &str, owner: &LootOwner, lines: Vec<String>) {
    match owner {
        LootOwner::Encounter => root.app.encounters.edit_draft(|draft| draft.loot(lines)),
        LootOwner::Changes(of) => {
            let joined = parts(&lines.join(";")).join("; ");
            root.pages
                .encounters
                .loot
                .entry(key.to_owned())
                .or_default()
                .cell = Some(lines);
            root.app
                .encounters
                .edit_changes(of, ChangeEdit::Loot((!joined.is_empty()).then_some(joined)));
        }
    }
}

/// Change the lines from a handler: read them as shown, change them, write them back.
fn change(
    root: &mut SplatRoot,
    key: &str,
    owner: &LootOwner,
    stored: &[String],
    act: impl FnOnce(&mut Vec<String>, &mut LootState),
) {
    let mut state = root
        .pages
        .encounters
        .loot
        .get(key)
        .cloned()
        .unwrap_or_default();
    let mut lines = shown_lines(Some(&state), owner, stored);
    let before = lines.clone();
    act(&mut lines, &mut state);
    root.pages.encounters.loot.insert(key.to_owned(), state);
    if lines != before {
        write(root, key, owner, lines);
    }
}

/// The campaign's tables and items, from the index every page shares.
fn sources(root: &SplatRoot) -> Option<Arc<Sources>> {
    loot_editor::sources(&root.ctx)
}

/// The list and its add field. `key` keys its fields and its kept state (an encounter draft's id,
/// a Changes form's key); `name` names its fields (`the encounter`, a creature's name); `stored`
/// is its loot now (the encounter's lines, or a creature's one line).
pub(super) fn list(
    root: &SplatRoot,
    key: String,
    name: String,
    stored: Vec<String>,
    owner: LootOwner,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let state = root
        .pages
        .encounters
        .loot
        .get(&key)
        .cloned()
        .unwrap_or_default();
    let sources = sources(root);
    let lines = shown_lines(Some(&state), &owner, &stored);
    let stored = Arc::new(stored);
    let mut list = div().flex().flex_col().gap(XS);
    for at in 0..lines.len() {
        list = list.child(line_row(
            &key,
            &name,
            &lines,
            at,
            &state,
            sources.as_deref(),
            (&owner, &stored),
            cx,
        ));
    }
    if let Some((at, line)) = state.removed.clone() {
        let words = format!("Removed {}.", loot_editor::plain(&line));
        let (key, owner, stored) = (key.clone(), owner.clone(), stored.clone());
        list = list.child(
            // A removal is a finished change: the success tone, as every finished change has.
            kit::Message::new(kit::Tone::Success, words).small().action(
                kit::Button::quiet(
                    ElementId::Name(format!("loot-undo:{key}").into()),
                    "Undo remove loot",
                )
                .icon(icons::UNDO)
                .density(Density::Read)
                .line_start()
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    change(root, &key, &owner, &stored, |lines, state| {
                        lines.insert(at.min(lines.len()), line.clone());
                        state.removed = None;
                    });
                    cx.notify();
                })),
            ),
        );
    }
    list.child(add_row(
        &key,
        &name,
        &state,
        sources.as_deref(),
        (&owner, &stored),
        cx,
    ))
}

/// One line: a roll, a Library item, or words in a field; then its `⋯` with Remove.
#[allow(clippy::too_many_arguments)]
fn line_row(
    key: &str,
    name: &str,
    lines: &[String],
    at: usize,
    state: &LootState,
    sources: Option<&Sources>,
    (owner, stored): (&LootOwner, &Arc<Vec<String>>),
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let line = lines[at].clone();
    let editing = state.editing == Some(at);
    let row_name = format!("{name} loot line {}", at + 1);
    let mut row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_h(Density::Read.control());
    let mut below = None;
    // A text line's field takes the row; the others leave the room after their words.
    let mut grows = false;
    match Loot::parse(&line) {
        Loot::Roll { times, key: table } if !editing => {
            let count = match &state.count {
                Some((place, typed)) if *place == at => typed.clone(),
                _ => times.to_string(),
            };
            let (k, o, s) = (key.to_owned(), owner.clone(), stored.clone());
            let count_table = table.clone();
            row = row
                .child(kit::icon(icons::DICE, Type::Body, c.muted))
                .child(kit::text(Type::Body, c.text, "Roll"))
                .child(
                    kit::TextField::new(
                        ElementId::Name(format!("loot-count:{key}:{at}").into()),
                        count,
                    )
                    .name(format!("Rolls on {table}, {row_name}"))
                    .placeholder("1")
                    .width(XXL + S)
                    .on_change(
                        cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                            let words = words.to_owned();
                            let table = count_table.clone();
                            change(root, &k, &o, &s, |lines, state| {
                                if let Ok(times) = words.trim().parse::<u32>()
                                    && (1..=MAX_ROLLS).contains(&times)
                                    && let Some(line) = lines.get_mut(at)
                                {
                                    *line = Loot::Roll { times, key: table }.line();
                                }
                                state.count = Some((at, words));
                            });
                            cx.notify();
                        }),
                    )
                    .on_blur({
                        let k = key.to_owned();
                        super::editor::on_key(cx, move |root: &mut SplatRoot| {
                            if let Some(state) = root.pages.encounters.loot.get_mut(&k)
                                && state.count.as_ref().is_some_and(|(place, _)| *place == at)
                            {
                                state.count = None;
                            }
                        })
                    }),
                )
                .child(kit::text(Type::Body, c.muted, "×"));
            let picked = {
                let (k, o, s) = (key.to_owned(), owner.clone(), stored.clone());
                move |chosen: String| {
                    let (k, o, s) = (k.clone(), o.clone(), s.clone());
                    move |root: &mut SplatRoot| {
                        change(root, &k, &o, &s, |lines, _| {
                            if let Some(line) = lines.get_mut(at) {
                                *line = Loot::Roll {
                                    times,
                                    key: chosen.clone(),
                                }
                                .line();
                            }
                        });
                    }
                }
            };
            row = row.child(table_menu(
                ElementId::Name(format!("loot-table:{key}:{at}").into()),
                &table,
                format!("Table for {row_name}"),
                sources,
                picked,
                cx,
            ));
            match sources.map(|sources| (sources.problem(&table), sources.table(&table))) {
                Some((Some(problem), _)) => {
                    row = row.child(kit::status(&kit::Status::new(
                        icons::WARN,
                        "Cannot roll",
                        c.result.warning,
                    )));
                    below = Some(problem);
                }
                Some((None, Some(table))) => {
                    row = row.child(kit::text(
                        Type::Small,
                        c.muted,
                        format!("{} · {}", table.dice, table.path),
                    ));
                }
                _ => {}
            }
        }
        Loot::Text(text) if !editing && loot_editor::shown(&text).is_some() => {
            let shown = loot_editor::shown(&text).unwrap_or(&text).to_owned();
            row = row.child(kit::icon(icons::LIBRARY, Type::Body, c.muted));
            row = match loot_editor::link_target(&text) {
                // As a creature's name does, the item's name opens it.
                Some(target) => {
                    let target = target.to_owned();
                    row.child(
                        kit::Link::new(
                            ElementId::Name(format!("loot-item:{key}:{at}").into()),
                            shown,
                        )
                        .on_click(cx.listener(
                            move |root: &mut SplatRoot, _, _, cx| {
                                crate::markdown_view::send_note_link(
                                    &root.ctx,
                                    target.clone(),
                                    None,
                                );
                                cx.notify();
                            },
                        )),
                    )
                }
                None => row.child(kit::text(Type::Body, c.text, shown)),
            };
            row = row.child(kit::text(Type::Small, c.muted, "Library item"));
        }
        _ => {
            let (k, o, s) = (key.to_owned(), owner.clone(), stored.clone());
            let one_cell = owner.one_cell();
            grows = true;
            row = row.child(
                div().flex_1().min_w(px(120.0)).child(
                    kit::TextField::new(
                        ElementId::Name(format!("loot-line:{key}:{at}").into()),
                        line.clone(),
                    )
                    .name(row_name.clone())
                    .placeholder("What the party finds")
                    .on_change(
                        cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                            let words = words.to_owned();
                            change(root, &k, &o, &s, |lines, state| {
                                state.editing = Some(at);
                                if one_cell && words.contains(SEPARATOR) {
                                    state.reason = Some(loot_editor::NO_SEPARATOR.into());
                                } else if let Some(line) = lines.get_mut(at) {
                                    *line = words;
                                    state.reason = None;
                                }
                            });
                            cx.notify();
                        }),
                    )
                    .on_blur({
                        let (k, o, s) = (key.to_owned(), owner.clone(), stored.clone());
                        let root = cx.entity().downgrade();
                        move |_, cx| {
                            let (k, o, s) = (k.clone(), o.clone(), s.clone());
                            let _ = root.update(cx, |root, cx| {
                                // A row emptied by hand is gone, with nothing to bring back.
                                change(root, &k, &o, &s, |lines, state| {
                                    if state.editing == Some(at) {
                                        state.editing = None;
                                    }
                                    if lines.get(at).is_some_and(|line| line.trim().is_empty()) {
                                        lines.remove(at);
                                    }
                                });
                                cx.notify();
                            });
                        }
                    }),
                ),
            );
        }
    }
    let (k, o, s) = (key.to_owned(), owner.clone(), stored.clone());
    let menu = super::common::row_menu(
        ElementId::Name(format!("loot-menu:{key}:{at}").into()),
        &row_name,
        vec![kit::MenuItem::new("Remove").on_click(cx.listener(
            move |root: &mut SplatRoot, _, _, cx| {
                change(root, &k, &o, &s, |lines, state| {
                    if at < lines.len() {
                        let line = lines.remove(at);
                        state.removed = (!line.trim().is_empty()).then_some((at, line));
                        state.editing = None;
                        state.count = None;
                    }
                });
                cx.notify();
            },
        ))],
    );
    let row = if grows {
        row
    } else {
        row.child(div().flex_1())
    }
    .child(menu);
    let mut block = div().flex().flex_col().child(row);
    if let Some(words) = below {
        // Under the key it is about, so the row stays one line.
        block = block.child(div().pl(XL + XXL + L).child(reason(words)));
    }
    block
}

/// The quiet menu of the campaign's tables, with the caret after its word.
fn table_menu<F, A>(
    id: ElementId,
    word: &str,
    name: String,
    sources: Option<&Sources>,
    picked: F,
    cx: &mut Context<SplatRoot>,
) -> AnyElement
where
    F: Fn(String) -> A,
    A: Fn(&mut SplatRoot) + 'static,
{
    let trigger_id = ElementId::NamedChild(Arc::new(id.clone()), "button".into());
    let mut menu = kit::MenuButton::new(
        id,
        kit::Button::quiet(trigger_id, format!("{word} {}", icons::DOWN))
            .density(Density::Read)
            .name(name),
    );
    let tables = sources.map_or(&[][..], |sources| &sources.tables[..]);
    if tables.is_empty() {
        menu = menu.item(
            kit::MenuItem::new(loot_editor::NO_TABLES)
                .blocked(Some("Mark a table in a World note to roll on it.")),
        );
    }
    for table in tables {
        let act = picked(table.key.clone());
        menu = menu.item(
            kit::MenuItem::new(format!("{} · {} · {}", table.key, table.dice, table.path))
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    act(root);
                    cx.notify();
                })),
        );
    }
    menu.into_any_element()
}

/// The field that adds a line, `Roll on a table` beside it, the tables and items that match the
/// words under it, and the reason a line was not added.
fn add_row(
    key: &str,
    name: &str,
    state: &LootState,
    sources: Option<&Sources>,
    (owner, stored): (&LootOwner, &Arc<Vec<String>>),
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let one_cell = owner.one_cell();
    let typed = state.add.trim().to_owned();
    // A line added from the field, a suggestion or the menu.
    let adder = {
        let (k, o, s) = (key.to_owned(), owner.clone(), stored.clone());
        move |line: Option<String>, enter: bool| {
            let (k, o, s) = (k.clone(), o.clone(), s.clone());
            move |root: &mut SplatRoot| {
                let line = line.clone();
                change(root, &k, &o, &s, |lines, state| {
                    let typed = state.add.trim().to_owned();
                    let line = match line {
                        Some(line) => Some(line),
                        None if enter && loot_editor::too_many(&typed) => {
                            state.reason =
                                Some(format!("One line rolls at most {MAX_ROLLS} times."));
                            None
                        }
                        None if enter && !typed.is_empty() => Some(typed),
                        None => None,
                    };
                    if let Some(line) = line {
                        if one_cell && line.contains(SEPARATOR) {
                            state.reason = Some(loot_editor::NO_SEPARATOR.into());
                        } else {
                            lines.push(line);
                            state.add.clear();
                            state.reason = None;
                            state.removed = None;
                        }
                    }
                });
            }
        }
    };
    let times = loot_editor::typed_times(&typed);
    let roll = |table: String| Loot::Roll { times, key: table }.line();
    let field_key = key.to_owned();
    let field = kit::TextField::new(
        ElementId::Name(format!("loot-add:{key}").into()),
        state.add.clone(),
    )
    .name(format!("Add loot to {name}"))
    .placeholder("Type loot, or find an item or table")
    .on_change(
        cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
            let state = root
                .pages
                .encounters
                .loot
                .entry(field_key.clone())
                .or_default();
            state.add = words.to_owned();
            state.reason = None;
            cx.notify();
        }),
    )
    .on_enter(super::editor::on_key(cx, adder(None, true)));
    let menu = table_menu(
        ElementId::Name(format!("loot-roll:{key}").into()),
        ROLL_MENU,
        format!("{ROLL_MENU} for {name}"),
        sources,
        |table| adder(Some(roll(table)), false),
        cx,
    );
    let mut block = div().flex().flex_col().gap(XS).child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(div().flex_1().min_w(px(120.0)).child(field))
            .child(menu),
    );
    if let Some(sources) = sources.filter(|_| !typed.is_empty()) {
        let (tables, items) = loot_editor::matching(sources, &typed);
        let mut found = div().flex().flex_col();
        if !tables.is_empty() {
            found = found.child(kit::label("Roll tables", 0));
            for table in tables {
                let title = if times == 1 {
                    table.key.clone()
                } else {
                    format!("{times} × {}", table.key)
                };
                let detail = table
                    .fault()
                    .unwrap_or_else(|| format!("{} · {}", table.dice, table.path));
                found = found.child(suggestion(
                    format!("loot-suggest-table:{key}:{}", table.key),
                    title,
                    detail,
                    adder(Some(roll(table.key.clone())), false),
                    cx,
                ));
            }
        }
        if !items.is_empty() {
            found = found.child(kit::label("Items", 0));
            for item in items {
                found = found.child(suggestion(
                    format!("loot-suggest-item:{key}:{}", item.link),
                    item.name.clone(),
                    item.detail.clone(),
                    adder(Some(item.link.clone()), false),
                    cx,
                ));
            }
        }
        block = block.child(
            found
                .p(XS)
                .border_1()
                .border_color(c.hairline)
                .rounded(crate::gui::theme::RADIUS_CONTROL),
        );
    }
    if let Some(words) = &state.reason {
        block = block.child(reason(words.clone()));
    }
    block
}

/// One suggestion under the add field: its title and detail; a press adds it.
fn suggestion(
    id: String,
    title: String,
    detail: String,
    act: impl Fn(&mut SplatRoot) + 'static,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let name = title.clone();
    kit::target(
        div()
            .id(ElementId::Name(id.into()))
            .flex()
            .flex_row()
            .items_baseline()
            .gap(S)
            .px(S)
            .min_h(Density::Read.control())
            .rounded(crate::gui::theme::RADIUS_CONTROL)
            .cursor_pointer()
            .hover(|row| row.bg(c.hover))
            .child(kit::text(Type::Body, c.text, title))
            .child(kit::text(Type::Small, c.muted, detail))
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                act(root);
                cx.notify();
            })),
        name,
    )
}
