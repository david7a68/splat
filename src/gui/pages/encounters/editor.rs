//! The encounter editor (egui: `Encounters::editor_view`, `editor_panes`, `roster`,
//! `add_pane`, `creature_search`): the back line, the name and its actions, the save row, the
//! roster column, Add participants, and the footer.
use super::common::{egui_color, footer, on_page, reason, row_menu, small_icon, text_width};
use crate::gui::theme::{
    Density, HAIRLINE_WIDTH, L, LIST_MEASURE, M, PANEL_MIN_WIDTH, S, Type,
    TypeExt, XS, colors,
};
use crate::gui::{SplatRoot, icons, kit};
use eframe::egui;
use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, Context, Div, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    StatefulInteractiveElement, Styled, Window, div, px, uniform_list,
};
use splat_encounters_ui::encounters::model::{
    AddView, EditorView, HeadAct, PartyButton, ResultRow, RosterAct, RosterRow, RosterView,
};
use std::sync::Arc;

/// The id of the page's creature filter, shared by the roster and a group (as in egui).
pub(super) const FILTER_ID: &str = "creature_search_filter";

/// The encounter editor.
pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let Some(active) = root.app.campaigns.active_campaign() else {
        return div().into_any_element();
    };
    let Some(view) = root.app.encounters.editor_frame(&active) else {
        return div().into_any_element();
    };
    let ctx = root.bridge.ctx().clone();
    let linked = active.dndbeyond_campaign_id.is_some();
    let roster = root.app.encounters.roster_view(&ctx, &root.app.party, linked);
    let filter = filter(&ctx);
    let add = root.app.encounters.add_view(filter);
    let lines = root.app.encounters.editor_footer(&ctx, &active.path);
    let choices = kit::menu::is_open(&super::filter::menu_id(), cx)
        .then(|| root.app.encounters.filter_choices());
    let focus_name = root.app.encounters.take_focus_name();
    let focus_search = root.app.encounters.take_focus_search();

    let content = window.viewport_size().width - crate::gui::theme::RAIL_WIDTH - M * 2.0;
    let add_width = px(crate::design::side_panel_width(f32::from(content)))
        .max(content - LIST_MEASURE)
        .min(px(640.0));
    let roster_width = (content - add_width).max(PANEL_MIN_WIDTH);
    let head = head(&view, roster_width, focus_name, cx);
    let body: AnyElement = match (roster, add, view.reading) {
        (Some(roster), Some(add), false) => {
            let roster_pane = kit::Pane::new(("encounter-roster", view.draft.as_u128() as u64))
                .flush()
                .child(roster_column(root, &view, &roster, window, cx));
            kit::Columns::new("encounter-editor-columns")
                .column(
                    roster_width,
                    PANEL_MIN_WIDTH..LIST_MEASURE,
                    div().size_full().pr(S).child(roster_pane),
                )
                .rest(add_column(&add, choices.as_ref(), focus_search, cx))
                .into_any_element()
        }
        _ => div()
            .flex()
            .flex_row()
            .gap(S)
            .child(small_icon(icons::BUSY, colors().muted))
            .child(kit::text(Type::Body, colors().muted, "Reading this campaign…"))
            .into_any_element(),
    };
    let footer_lines = footer("encounter-editor-footer", lines, cx);
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(head)
        .child(kit::rule())
        .child(div().flex_1().min_h_0().pt(S).child(body))
        .when(!footer_lines.is_empty(), |editor| {
            editor.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .pt(S)
                    .border_t(HAIRLINE_WIDTH)
                    .border_color(colors().hairline)
                    .children(footer_lines),
            )
        })
        .into_any_element()
}

/// The page's creature filter, kept in the bridge's egui context as the egui page keeps it.
pub(super) fn filter(ctx: &egui::Context) -> splat_content::content::creature_filter::Filter {
    ctx.data(|data| data.get_temp(egui::Id::new(FILTER_ID)))
        .unwrap_or_default()
}

/// The back line: the destination's link and, while a draft holds it, that draft's reason.
pub(super) fn back_line(
    id: &'static str,
    word: &'static str,
    name: &'static str,
    held: Option<(&'static str, &'static str)>,
    on_back: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> Div {
    let link = kit::Button::quiet(id, format!("{word} {}", icons::HISTORY_BACK))
        .line_start()
        .name(name)
        .blocked(held.map(|(long, _)| long))
        .on_click(on_back);
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .min_h(Density::Read.control())
        .child(link)
        .children(held.map(|(long, _)| reason(long).whitespace_nowrap()))
}

/// The head: the back line, the name with Save and Start, the save row, the run's note.
fn head(view: &EditorView, roster_width: Pixels, focus: bool, cx: &mut Context<SplatRoot>) -> Div {
    let c = colors();
    let back = back_line(
        "encounter-back",
        view.back.word,
        view.back.name,
        view.back.held,
        on_page(cx, |page, ctx, active, campaigns| {
            page.editor_head_act(ctx, active, campaigns.modules(), HeadAct::Back);
        }),
    );
    let name = kit::TextField::new(
        ElementId::Name(format!("encounter-name:{}", view.draft).into()),
        view.name.clone(),
    )
    .name("Encounter name")
    .placeholder("Encounter name")
    .focus(focus)
    .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
        root.app.encounters.edit_draft(|draft| draft.name(words));
        cx.notify();
    }))
    .on_enter(on_key(cx, |root: &mut SplatRoot| {
        root.app.encounters.ask_focus_search();
    }));
    let save = if view.save_primary {
        kit::Button::primary("encounter-save", "Save encounter")
    } else {
        kit::Button::secondary("encounter-save", "Save encounter")
    }
    .blocked(view.save_blocked.clone())
    .key("Ctrl+S")
    .on_click(on_page(cx, |page, ctx, active, campaigns| {
        page.editor_head_act(ctx, active, campaigns.modules(), HeadAct::Save);
    }));
    let start = if view.start_primary {
        kit::Button::primary("encounter-start", view.start_word)
    } else {
        kit::Button::secondary("encounter-start", view.start_word)
    }
    .blocked(view.start_blocked.clone())
    .hint(view.start_hint)
    .on_click(on_page(cx, |page, ctx, active, campaigns| {
        let save = page.encounter_dirty();
        page.editor_head_act(ctx, active, campaigns.modules(), HeadAct::Start { save });
    }));
    let mut actions = div()
        .flex()
        .flex_row()
        .flex_none()
        .items_center()
        .gap(S)
        .child(save);
    if view.run_open {
        actions = actions.child(reason("Close the open run first.").whitespace_nowrap().mr(S));
    }
    actions = actions.child(start);
    if let Some(blocked) = view.start_another.clone() {
        actions = actions.child(row_menu(
            "encounter-start-another",
            &view.name,
            vec![
                kit::MenuItem::new("Start another fight")
                    .blocked(blocked)
                    .on_click(on_page(cx, |page, ctx, active, campaigns| {
                        let save = page.encounter_dirty();
                        page.editor_head_act(
                            ctx,
                            active,
                            campaigns.modules(),
                            HeadAct::StartAnother { save },
                        );
                    })),
            ],
        ));
    }
    let name_row = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(L)
        .child(div().w(roster_width - L).min_w(px(200.0)).ty(Type::Heading).child(name))
        .child(actions);
    let (icon, word, color) = &view.state;
    let mut row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_h(Density::Read.control())
        .child(kit::status(&kit::Status::new(
            icon.clone(),
            word.clone(),
            egui_color(*color),
        )));
    if let Some(blocked) = view.discard {
        row = row.child(
            kit::Button::quiet("encounter-discard", "Discard")
                .blocked(blocked)
                .on_click(on_page(cx, |page, ctx, active, campaigns| {
                    page.editor_head_act(ctx, active, campaigns.modules(), HeadAct::Discard);
                })),
        );
    }
    for words in [view.lock_reason, view.other_reason].into_iter().flatten() {
        row = row.child(reason(words).whitespace_nowrap().ml(S));
    }
    if let Some(state) = &view.run_state {
        row = row.child(div().flex_1()).child(kit::status(&kit::Status::new(
            icons::RUN,
            state.clone(),
            c.muted,
        )));
    }
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(S)
        .child(back)
        .child(name_row)
        .child(row)
        .children(view.unfinished_note.map(reason))
}

/// The roster column: its heading, rows, options, loot and notes.
fn roster_column(
    root: &mut SplatRoot,
    view: &EditorView,
    roster: &RosterView,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let mut heading = kit::label("Roster", roster.count);
    if !roster.difficulty.is_empty() {
        heading = heading.child(kit::text(Type::Label, c.muted, "·"));
        for (words, word) in &roster.difficulty {
            heading = heading.child(
                div()
                    .ty(Type::Label)
                    .text_color(if *word { c.text } else { c.muted })
                    .child(words.clone()),
            );
        }
    }
    let heading = div()
        .id("encounter-roster-heading")
        .pb(XS)
        .child(heading)
        .when_some(roster.budgets.clone(), |heading, budgets| {
            kit::WithTooltip::with_tooltip(heading, budgets)
        });
    let mut column = div().flex().flex_col().w_full().child(heading);
    if roster.rows.is_empty() {
        column = column.child(kit::gap(S)).child(reason(
            "No participants yet. Add the party and creatures from Add participants.",
        ));
    }
    let facts = roster
        .rows
        .iter()
        .map(|row| text_width(window, Type::Strong, &format!("{}. {}", row.index + 1, row.name)))
        .fold(px(0.0), Pixels::max);
    let _ = facts;
    for row in &roster.rows {
        column = column.child(roster_row(root, view, row, cx));
    }
    for fault in &roster.faults_after {
        column = column.child(kit::Message::new(kit::Tone::Warning, fault.clone()).small());
    }
    column = column.child(kit::gap(M));
    if let Some(sync) = &roster.sync {
        column = column.child(
            kit::Checkbox::new("encounter-sync", "Sync party with D&D Beyond", sync.on).on_click(
                cx.listener(|root: &mut SplatRoot, _, _, cx| {
                    root.app.encounters.edit_draft(|draft| draft.toggle_sync());
                    cx.notify();
                }),
            ),
        );
        if let Some(line) = &sync.line {
            let mut line_row = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_baseline()
                .gap(S)
                .child(kit::text(Type::Small, c.text, line.clone()));
            if sync.connect {
                line_row = line_row.child(
                    kit::Button::quiet("encounter-connect", "Connect to sync")
                        .plain()
                        .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                            root.bridge.ctx().data_mut(|data| {
                                data.insert_temp(egui::Id::new("open_account_settings"), true)
                            });
                            cx.notify();
                        })),
                );
            }
            column = column.child(line_row);
        }
        column = column.child(kit::help(sync.help));
    }
    column = column
        .child(
            kit::Checkbox::new(
                "encounter-repeatable",
                "Keep in Ready during a run",
                roster.repeatable,
            )
            .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                root.app.encounters.edit_draft(|draft| draft.toggle_repeatable());
                cx.notify();
            })),
        )
        .child(kit::help(
            "Lists this encounter under Ready to run while a fight from it is unfinished. Start fight waits until no fight is open.",
        ))
        .child(loot(root, view, roster, cx))
        .child(kit::gap(M))
        .child(kit::field(
            "Read-aloud and notes",
            None,
            None,
            kit::TextEditor::new(
                ElementId::Name(format!("encounter-notes:{}", view.draft).into()),
                roster.notes.clone(),
            )
            .name("Read-aloud and notes")
            .placeholder(
                "What you read to the table, then your tactics. The run shows this before the first turn.",
            )
            .rows(5)
            .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                root.app.encounters.edit_draft(|draft| draft.notes(words));
                cx.notify();
            })),
        ));
    column.max_w(LIST_MEASURE)
}

/// The encounter's loot: one field per line, a remove for each, and a field that adds a line.
fn loot(root: &mut SplatRoot, view: &EditorView, roster: &RosterView, cx: &mut Context<SplatRoot>) -> Div {
    let mut block = div().flex().flex_col().child(kit::heading(
        kit::Heading::Group {
            title: "Loot",
            count: roster.loot.lines.len(),
        },
        false,
    ));
    if !roster.loot.owned {
        return block.child(reason(
            "The Loot section of this note has more than a list. Edit it in Read-aloud and notes.",
        ));
    }
    block = block.child(kit::help(
        "What the party finds when this fight ends. A creature’s own loot is in its Changes.",
    ));
    for (at, line) in roster.loot.lines.iter().enumerate() {
        let lines = roster.loot.lines.clone();
        let removed = lines.clone();
        block = block.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(
                    div().flex_1().child(
                        kit::TextField::new(
                            ElementId::Name(format!("encounter-loot:{}:{at}", view.draft).into()),
                            line.clone(),
                        )
                        .name(format!("the encounter loot line {}", at + 1))
                        .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                            let mut lines = lines.clone();
                            if let Some(line) = lines.get_mut(at) {
                                *line = words.to_owned();
                            }
                            root.app.encounters.edit_draft(|draft| draft.loot(lines));
                            cx.notify();
                        })),
                    ),
                )
                .child(
                    kit::IconButton::new(
                        ElementId::Name(format!("encounter-loot-remove:{at}").into()),
                        icons::CLOSE,
                        format!("Remove loot line {}", at + 1),
                    )
                    .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                        let mut lines = removed.clone();
                        if at < lines.len() {
                            lines.remove(at);
                        }
                        root.app.encounters.edit_draft(|draft| draft.loot(lines));
                        cx.notify();
                    })),
                ),
        );
    }
    let typed = root.pages.encounters.loot_typed.clone();
    let lines = roster.loot.lines.clone();
    block.child(
        div().max_w(px(460.0)).child(
            kit::TextField::new(
                ElementId::Name(format!("encounter-loot-add:{}", view.draft).into()),
                typed,
            )
            .name("Add loot")
            .placeholder("Type loot, or find an item or table")
            .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                root.pages.encounters.loot_typed = words.to_owned();
                cx.notify();
            }))
            .on_enter(on_key(cx, move |root: &mut SplatRoot| {
                let words = std::mem::take(&mut root.pages.encounters.loot_typed);
                if !words.trim().is_empty() {
                    let mut lines = lines.clone();
                    lines.push(words.trim().to_owned());
                    root.app.encounters.edit_draft(|draft| draft.loot(lines));
                }
            })),
        ),
    )
}

/// One roster row: its name and second line, Edit and its menu, and its fields while open.
fn roster_row(root: &mut SplatRoot, view: &EditorView, row: &RosterRow, cx: &mut Context<SplatRoot>) -> Div {
    let c = colors();
    let key = row.key.clone();
    let mut block = div().flex().flex_col();
    for fault in &row.faults_before {
        block = block.child(kit::Message::new(kit::Tone::Warning, fault.clone()).small());
    }
    let mut line = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_baseline()
        .min_w_0()
        .ty(Type::Small)
        .text_color(c.muted);
    for (at, (words, color, bloodied)) in row.parts.iter().enumerate() {
        if at > 0 {
            line = line.child(div().child(" · "));
        }
        if *bloodied {
            line = line.child(kit::status(&kit::Status::new(
                icons::BLOODIED,
                words.clone(),
                c.hp.bloodied,
            )));
        } else {
            line = line.child(div().text_color(egui_color(*color)).child(words.clone()));
        }
    }
    if let Some((icon, color, _)) = &row.missing {
        line = line.child(div().child(" · ")).child(small_icon(icon, egui_color(*color)));
    }
    if let Some(word) = &row.changed {
        let open = key.clone();
        line = line.child(div().child(" · ")).child(
            kit::Button::quiet(
                ElementId::Name(format!("roster-change:{key}").into()),
                format!("{} {word}", icons::EDIT),
            )
            .plain()
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                root.app.encounters.toggle_row(&open);
                cx.notify();
            })),
        );
    }
    let facts = div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w_0()
        .child(kit::text::one_line(
            Type::Strong,
            c.text,
            format!("{}. {}", row.index + 1, row.name),
        ))
        .child(line);
    let index = row.index;
    let mut items = vec![
        kit::MenuItem::new("Edit").on_click({
            let key = key.clone();
            cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                root.app.encounters.toggle_row(&key);
                cx.notify();
            })
        }),
        kit::MenuItem::new("Move up")
            .icon(icons::MOVE_UP)
            .blocked(row.first.then_some("Already first."))
            .on_click(roster_act(cx, RosterAct::Up(index))),
        kit::MenuItem::new("Move down")
            .icon(icons::MOVE_DOWN)
            .blocked(row.last.then_some("Already last."))
            .on_click(roster_act(cx, RosterAct::Down(index))),
        kit::MenuItem::new("Duplicate").on_click(roster_act(cx, RosterAct::Duplicate(index))),
    ];
    if row.make_yours {
        items.push(kit::MenuItem::new("Make it yours").on_click(on_page(
            cx,
            move |page, _, active, campaigns| page.make_yours(index, active, campaigns.modules()),
        )));
    }
    items.push(
        kit::MenuItem::new("Remove")
            .icon(icons::REMOVE)
            .on_click(roster_act(cx, RosterAct::Remove(index))),
    );
    let action: AnyElement = match &row.repair {
        Some((label, path)) => {
            let path = path.clone();
            kit::Button::quiet(ElementId::Name(format!("roster-repair:{key}").into()), *label)
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    root.bridge.ctx().data_mut(|data| {
                        data.insert_temp(egui::Id::new("open_library_repair"), path.clone())
                    });
                    cx.notify();
                }))
                .into_any_element()
        }
        None => {
            let toggle = key.clone();
            kit::Button::quiet(ElementId::Name(format!("roster-edit:{key}").into()), row.edit_word)
                .name(format!("{} {}", row.edit_word, row.name))
                .selected(row.open)
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    root.app.encounters.toggle_row(&toggle);
                    cx.notify();
                }))
                .into_any_element()
        }
    };
    block = block.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .min_h(Density::Glance.row())
            .py(XS)
            .child(facts)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_none()
                    .items_center()
                    .gap(S)
                    .child(action)
                    .child(row_menu(ElementId::Name(format!("roster-menu:{key}").into()), &row.name, items)),
            ),
    );
    if let Some(form) = &row.form {
        block = block.child(row_form(root, view, row, form, cx));
    } else if row.open && row.missing.is_some() && row.repair.is_none() {
        block = block.child(reason(
            "To replace this creature, remove the row and add another from Add participants.",
        ));
    }
    block.child(kit::rule())
}

/// An open row's fields: its side, its initiative group, its starting values.
fn row_form(
    root: &mut SplatRoot,
    view: &EditorView,
    row: &RosterRow,
    form: &splat_encounters_ui::encounters::model::RowForm,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let key = row.key.clone();
    let mut named: Vec<String> = vec!["party".to_owned(), "enemies".to_owned()];
    for side in &form.sides {
        if !named.contains(side) {
            named.push(side.clone());
        }
    }
    if !named.contains(&form.side) && !form.side.trim().is_empty() {
        named.push(form.side.clone());
    }
    let other_open = root.pages.encounters.other_side.as_deref() == Some(key.as_str());
    let chosen = if other_open {
        Some(named.len())
    } else {
        named.iter().position(|side| *side == form.side)
    };
    let mut options: Vec<String> = named
        .iter()
        .map(|side| crate::design::reader::human_label(side))
        .collect();
    options.push("Other…".to_owned());
    let picked = named.clone();
    let side_key = key.clone();
    let mut side_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(reason("Side"))
        .child(
            kit::Dropdown::new(
                ElementId::Name(format!("roster-side:{key}").into()),
                "Side",
                options,
                chosen,
            )
            .width(px(104.0))
            .on_select(cx.listener(move |root: &mut SplatRoot, index: &usize, _, cx| {
                match picked.get(*index) {
                    Some(side) => {
                        root.pages.encounters.other_side = None;
                        root.app.encounters.edit_draft(|draft| draft.side(&side_key, side));
                    }
                    None => root.pages.encounters.other_side = Some(side_key.clone()),
                }
                cx.notify();
            })),
        );
    if other_open {
        let other_key = key.clone();
        side_row = side_row.child(
            kit::TextField::new(ElementId::Name(format!("roster-side-other:{key}").into()), "")
                .name("Other side for Side")
                .placeholder("Side name")
                .width(px(80.0))
                .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                    root.app.encounters.edit_draft(|draft| draft.side(&other_key, words));
                    cx.notify();
                })),
        );
    }
    let group_key = key.clone();
    let mut values = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(S)
        .child(
            kit::TextField::new(
                ElementId::Name(format!("roster-group:{}:{key}", view.draft).into()),
                form.group.clone(),
            )
            .name("Initiative group (optional)")
            .placeholder("Initiative group")
            .width(px(120.0))
            .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                root.app
                    .encounters
                    .edit_draft(|draft| draft.initiative_group(&group_key, words));
                cx.notify();
            })),
        );
    for (field, label, value) in form.values.iter().copied() {
        let toggle_key = key.clone();
        values = values.child(
            kit::Checkbox::new(
                ElementId::Name(format!("roster-value:{field}:{key}").into()),
                label,
                value.is_some(),
            )
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                let next = if value.is_some() { None } else { Some(0) };
                root.app
                    .encounters
                    .edit_draft(|draft| draft.value(&toggle_key, field, next));
                cx.notify();
            })),
        );
        if let Some(value) = value {
            let value_key = key.clone();
            values = values.child(
                kit::TextField::new(
                    ElementId::Name(format!("roster-number:{field}:{key}").into()),
                    value.to_string(),
                )
                .name(label)
                .width(px(64.0))
                .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                    if let Ok(number) = words.trim().parse::<i64>() {
                        root.app
                            .encounters
                            .edit_draft(|draft| draft.value(&value_key, field, Some(number)));
                    }
                    cx.notify();
                })),
            );
        }
    }
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(S)
        .child(side_row)
        .child(values)
        .child(kit::help(form.help))
}

/// A handler with no event (Enter in a field) that changes the root and draws again.
pub(super) fn on_key(
    cx: &mut Context<SplatRoot>,
    act: impl Fn(&mut SplatRoot) + 'static,
) -> impl Fn(&mut Window, &mut gpui::App) + 'static {
    let root = cx.entity().downgrade();
    move |_, cx| {
        let _ = root.update(cx, |root, cx| {
            act(root);
            cx.notify();
        });
    }
}

fn roster_act(
    cx: &mut Context<SplatRoot>,
    act: RosterAct,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static {
    cx.listener(move |root: &mut SplatRoot, _, _, cx| {
        root.app.encounters.roster_act(act);
        cx.notify();
    })
}

/// Add participants' head: its heading, the party's line, the target and the search.
pub(super) fn add_head(
    add: &AddView,
    choices: Option<&super::filter::Choices>,
    focus: bool,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let group = matches!(add.party, PartyButton::None);
    let mut head = div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(div().pb(XS).child(kit::label(
            if group { "Add creatures" } else { "Add participants" },
            0,
        )))
        .child(kit::help(if group {
            "From this campaign and the packages it uses. Party members go on an encounter’s roster."
        } else {
            "Party members come from the sheets Splat saved last."
        }));
    head = match add.party {
        PartyButton::None => head,
        PartyButton::InRoster => head.child(kit::status(&kit::Status::new(
            icons::OK,
            "Party in roster",
            c.muted,
        ))),
        PartyButton::Add { blocked } => head
            .child(
                div().flex().child(
                    kit::Button::secondary("encounter-add-party", "Add party")
                        .icon(icons::PARTY)
                        .blocked(blocked)
                        .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                            root.app.encounters.add_party();
                            cx.notify();
                        })),
                ),
            )
            .children(blocked.map(reason)),
    };
    if let Some(target) = &add.adding_to {
        head = head.child(reason(format!("Adding to {target}")));
    }
    let name = if group {
        "Find a creature"
    } else {
        "Find a creature or party member"
    };
    head.child(search(add, choices, focus, name, cx))
}

/// The search field and the filter's words.
pub(super) fn search(
    add: &AddView,
    choices: Option<&super::filter::Choices>,
    focus: bool,
    name: &'static str,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let results = add.results.clone();
    let close_names = add.close_names;
    let mut field = div().flex().flex_row().items_center().gap(S).child(
        div().flex_1().child(
            kit::TextField::new("encounter-search", add.search.clone())
                .name(name)
                .placeholder("Find a creature by name or detail…")
                .focus(focus)
                .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                    root.app.encounters.set_search(words);
                    cx.notify();
                }))
                .on_enter(on_key(cx, move |root: &mut SplatRoot| {
                    root.app.encounters.add_best_match(&results, close_names);
                })),
        ),
    );
    if !add.search.is_empty() {
        field = field.child(
            kit::Button::quiet("encounter-search-clear", "Clear").on_click(cx.listener(
                |root: &mut SplatRoot, _, _, cx| {
                    root.app.encounters.set_search("");
                    cx.notify();
                },
            )),
        );
    }
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(field)
        .child(super::filter::bar(&add.filter, choices, cx))
}

/// Add participants: its head, then the results in a virtual list that fills the column.
pub(super) fn add_column(
    add: &AddView,
    choices: Option<&super::filter::Choices>,
    focus: bool,
    cx: &mut Context<SplatRoot>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .size_full()
        .pl(M)
        .child(div().flex_none().child(add_head(add, choices, focus, cx)))
        .child(div().flex_1().min_h_0().pt(S).child(results(add, cx)))
}

/// The search's results: a virtual list of rows, or the empty state.
pub(super) fn results(add: &AddView, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    if add.results.is_empty() {
        let words = if add.filter.is_empty() {
            "Check the spelling, or check Campaign → Packages."
        } else {
            "Remove a filter, or check Campaign → Packages."
        };
        return kit::empty_state(icons::SEARCH, "No creatures match", words, None::<AnyElement>).into_any_element();
    }
    let rows: Arc<Vec<ResultRow>> = add.results.clone();
    let close = add.close_names;
    let list = uniform_list(
        "encounter-results",
        rows.len(),
        cx.processor(move |_root: &mut SplatRoot, range: std::ops::Range<usize>, _window, cx| {
            range
                .filter_map(|at| rows.get(at).map(|row| result_row(row, cx)))
                .collect::<Vec<_>>()
        }),
    )
    .size_full();
    div()
        .flex()
        .flex_col()
        .size_full()
        .when(close, |block| {
            block.child(kit::text(
                Type::Small,
                c.muted,
                "No exact match. These names are close.",
            ))
        })
        .child(div().flex_1().min_h_0().child(list))
        .into_any_element()
}

/// One result: its name and stats, then Add (or `In roster`).
fn result_row(row: &ResultRow, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let detail = div()
        .flex()
        .flex_row()
        .items_baseline()
        .ty(Type::Small)
        .text_color(c.muted)
        .whitespace_nowrap()
        .min_w_0()
        .overflow_hidden()
        .children(stat_cells(&row.detail, row.hp_missing))
        .children(row.source.clone().map(|source| div().ml(XS).child(source)));
    let action: AnyElement = if row.in_roster {
        kit::status(&kit::Status::new(icons::OK, "In roster", c.muted)).into_any_element()
    } else {
        let (id, party) = (row.id, row.party);
        kit::Button::secondary(ElementId::Name(format!("add:{}", row.id).into()), "Add")
            .name(format!("Add {}", row.name))
            .blocked(row.unavailable.clone())
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                root.app.encounters.add_result(id, party);
                cx.notify();
            }))
            .into_any_element()
    };
    let peek = row.id;
    div()
        .id(ElementId::Name(format!("result:{}", row.id).into()))
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .h(result_height())
        .pr(S)
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .cursor_pointer()
                .child(kit::text::one_line(Type::Strong, c.text, row.name.clone()))
                .child(detail)
                .id(ElementId::Name(format!("result-read:{}", row.id).into()))
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    root.app.encounters.peek = Some(peek);
                    cx.notify();
                })),
        )
        .child(div().flex_none().w(px(96.0)).child(action))
        .into_any_element()
}

/// A result row's height: its two lines and its Add, with room between rows.
fn result_height() -> Pixels {
    Density::Scan.row() + S
}

/// A result's stats, each in its own cell (`detail_columns`): CR, HP and AC.
fn stat_cells(detail: &str, hp_missing: bool) -> Vec<Div> {
    let c = colors();
    let parts: Vec<&str> = detail.split(" · ").collect();
    let other = parts
        .iter()
        .find(|part| !["CR ", "HP ", "AC "].iter().any(|prefix| part.starts_with(prefix)))
        .copied();
    let mut cells = Vec::new();
    for (prefix, width) in [("CR ", px(48.0)), ("HP ", px(52.0)), ("AC ", px(40.0))] {
        let part = parts
            .iter()
            .find(|part| part.starts_with(prefix))
            .copied()
            .or(other.filter(|_| prefix == "CR "));
        let cell = div().w(width).flex_none();
        cells.push(match part {
            Some(part) if hp_missing && part.contains("HP not set") => cell.child(kit::status(
                &kit::Status::new(icons::WARN, "HP not set", c.result.warning),
            )),
            Some(part) => cell.child(part.to_owned()),
            None => cell,
        });
    }
    if hp_missing && !detail.contains("HP not set") {
        cells.push(div().child(kit::status(&kit::Status::new(
            icons::WARN,
            "HP not set",
            c.result.warning,
        ))));
    }
    cells
}
