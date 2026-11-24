//! The encounter editor: the back line, the name and its actions, the save row, the
//! roster column (its rows with their Changes and replacement, options, loot, notes and the
//! encounter's own groups), Add participants or the creature made yours, and the footer. Below
//! 760 px the roster and Add participants take turns under tabs. While the encounter's file (or
//! one of its groups') changed on disk, the conflict takes the editor's place.
use super::changes;
use super::common::{footer, on_page, reason, row_menu, small_icon, with_page};
use super::loot::{self, LootOwner};
use crate::gui::theme::{
    Density, HAIRLINE_WIDTH, L, LIST_MEASURE, M, PANEL_MIN_WIDTH, S, Type, TypeExt, XS, colors,
};
use crate::gui::{SplatRoot, icons, kit};
use crate::state_context::{Slot, StateContext};
use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, Context, Div, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    StatefulInteractiveElement, Styled, Window, div, px,
};
use splat_encounters_ui::encounters::model::{
    AddView, ChangesOf, EditorView, GroupAt, HeadAct, NarrowPane, OwnedAct, PartyButton, ResultRow,
    RosterAct, RosterRow, RosterView, UnusedCreature,
};
use std::sync::Arc;

/// The page's creature filter, shared by the roster and a group.
pub(super) const FILTER: Slot<splat_content::content::creature_filter::Filter> =
    Slot::new("creature_search_filter");
/// Add participants' column, by the id its dragged width is saved under (`ADD_PANE`).
pub(super) const ADD_PANE: &str = "encounter_add";
/// The widest Add participants takes by itself (`ADD_PANE_MAX_WIDTH`).
pub(super) const ADD_PANE_MAX_WIDTH: Pixels = px(640.0);
/// Below this the roster and Add participants take turns under tabs.
pub(super) const NARROW: Pixels = px(760.0);

/// Add participants' width: the DM's dragged width, else its own (`editor_columns`).
pub(super) fn add_width(content: Pixels, cx: &gpui::App) -> Pixels {
    match kit::edge::column_width(ADD_PANE, cx) {
        Some(width) => px(width).clamp(PANEL_MIN_WIDTH, ADD_PANE_MAX_WIDTH),
        None => px(crate::design::side_panel_width(f32::from(content)))
            .max(content - LIST_MEASURE)
            .min(ADD_PANE_MAX_WIDTH),
    }
}

/// The encounter editor.
pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let Some(active) = root.app.campaigns.active_campaign() else {
        return div().into_any_element();
    };
    // The file (or a group's) changed on disk: the conflict in the editor's place.
    if let Some(disk) = root.app.encounters.encounter_disk() {
        return super::disk::view(
            "encounter-disk",
            &disk,
            |root: &mut SplatRoot, keep| {
                with_page(root, |page, ctx, active, campaigns| {
                    page.encounter_disk_choice(ctx, active, campaigns.modules(), keep)
                });
            },
            cx,
        );
    }
    let Some(view) = root.app.encounters.editor_frame(&active) else {
        return div().into_any_element();
    };
    let ctx = root.ctx.clone();
    let linked = active.dndbeyond_campaign_id.is_some();
    let roster = root
        .app
        .encounters
        .roster_view(&ctx, &root.app.party, linked);
    let filter = filter(&ctx);
    let add = root.app.encounters.add_view(filter);
    let unused = root.app.encounters.unused_creatures(&ctx);
    let creature = root.app.encounters.creature_view();
    let owned = root.app.encounters.owned_groups(&ctx);
    let lines = root.app.encounters.editor_footer(&active.path);
    let choices = kit::menu::is_open(&super::filter::menu_id(), cx)
        .then(|| root.app.encounters.filter_choices());
    let focus_name = root.app.encounters.take_focus_name();
    let focus_search = root.app.encounters.take_focus_search();

    // The room the editor is laid out in (beside Lookup, in the session plan), never more than
    // the window gives the page: content wider than the room must not widen it.
    let page = root.page_width(window);
    let content = root
        .pages
        .encounters
        .room
        .get()
        .map_or(page, |room| room.min(page));
    let narrow = content < NARROW;
    let add_width = add_width(content, cx);
    let roster_width = (content - add_width).clamp(PANEL_MIN_WIDTH, LIST_MEASURE);
    let head = head(&view, (roster_width, content, narrow), focus_name, cx);
    let body: AnyElement = match (roster, add, view.reading) {
        (Some(roster), Some(add), false) => {
            let roster_column = roster_column(root, &view, &roster, owned, !narrow, window, cx);
            let roster_pane = kit::Pane::new(("encounter-roster", view.draft.as_u128() as u64))
                .flush()
                .child(roster_column);
            let side: AnyElement = match &creature {
                Some(creature) => super::creature::pane(creature, cx),
                None => {
                    let room = if narrow { content } else { add_width - M };
                    let facts = results_facts(root, &add, room, window);
                    add_column(
                        &add,
                        &unused,
                        choices.as_ref(),
                        (focus_search, !narrow),
                        (&root.pages.encounters.results_scroll.clone(), facts),
                        cx,
                    )
                    .into_any_element()
                }
            };
            if narrow {
                narrow_body(root, &roster, creature.is_some(), roster_pane, side, cx)
            } else {
                // The two panes stay together at the left: the roster at most the list
                // measure, then Add participants at its own or dragged width.
                div()
                    .size_full()
                    .max_w(LIST_MEASURE + add_width + S)
                    .child(
                        kit::Columns::new("encounter-editor-columns")
                            .rest(
                                div()
                                    .size_full()
                                    .max_w(LIST_MEASURE)
                                    .pr(S)
                                    .child(roster_pane),
                            )
                            .column(
                                add_width,
                                PANEL_MIN_WIDTH..ADD_PANE_MAX_WIDTH,
                                div().size_full().pl(M).child(side),
                            )
                            .saved(ADD_PANE),
                    )
                    .into_any_element()
            }
        }
        _ => reading(),
    };
    let footer_lines = footer("encounter-editor-footer", lines, cx);
    div()
        .relative()
        .flex()
        .flex_col()
        .size_full()
        .min_w_0()
        .overflow_hidden()
        .child(super::common::room_probe(
            root.pages.encounters.room.clone(),
        ))
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

/// `Reading this campaign…` while the campaign is read.
pub(super) fn reading() -> AnyElement {
    div()
        .flex()
        .flex_row()
        .gap(S)
        .child(small_icon(icons::BUSY, colors().muted))
        .child(kit::text(
            Type::Body,
            colors().muted,
            "Reading this campaign…",
        ))
        .into_any_element()
}

/// The narrow editor: the roster's tab with its count, then Add participants' (or the open
/// creature's), the roster's difficulty at the row's end, and the chosen pane under them.
fn narrow_body(
    root: &SplatRoot,
    roster: &RosterView,
    creature: bool,
    roster_pane: kit::Pane,
    side: AnyElement,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let pane = root.app.encounters.narrow_pane();
    let count = if roster.count == 0 {
        "Roster".to_owned()
    } else {
        format!("Roster {}", crate::design::count(roster.count, "", ""))
    };
    let other = if creature {
        "This creature"
    } else {
        "Add participants"
    };
    let tabs = kit::Segmented::new(
        "encounter-narrow-tabs",
        [count, other.to_owned()],
        usize::from(pane == NarrowPane::Add),
    )
    .on_select(cx.listener(|root: &mut SplatRoot, at: &usize, _, cx| {
        root.app.encounters.show_narrow_pane(if *at == 0 {
            NarrowPane::Roster
        } else {
            NarrowPane::Add
        });
        cx.notify();
    }));
    let mut difficulty = div().flex().flex_row().items_baseline().gap(XS);
    for (words, word) in &roster.difficulty {
        difficulty = difficulty.child(
            div()
                .ty(Type::Small)
                .text_color(if *word { c.text } else { c.muted })
                .child(words.clone()),
        );
    }
    div()
        .flex()
        .flex_col()
        .size_full()
        .gap(S)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(L)
                .child(tabs)
                .child(div().flex_1())
                .child(difficulty),
        )
        .child(div().flex_1().min_h_0().child(match pane {
            NarrowPane::Roster => roster_pane.into_any_element(),
            NarrowPane::Add => side,
        }))
        .into_any_element()
}

/// The page's creature filter, kept in the state context so the roster and a group share it.
pub(super) fn filter(ctx: &StateContext) -> splat_content::content::creature_filter::Filter {
    ctx.get(FILTER).unwrap_or_default()
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

/// The name field's place in the head (`save_row::head_plan`): it ends at the roster column's
/// edge, and gives way to its actions down to `NAME_LEAST`.
pub(super) fn head_name(column: Pixels) -> Div {
    div()
        .flex_1()
        .min_w(px(crate::design::save_row::NAME_LEAST))
        .max_w((column - S * 2.0).max(Density::Scan.control()))
}

/// The name and its actions: the actions stand `L` after the name while the name keeps at least
/// `NAME_LEAST`, else they take a line of their own at the text edge. Widths alone decide it, so
/// nothing moves while the DM types.
pub(super) fn head_row(name: Div, actions: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap_x(L)
        .gap_y(XS)
        .child(name)
        .child(actions)
}

/// A button's width in its wider form, as the head measures it.
pub(super) fn button_width(window: &mut Window, words: &str) -> Pixels {
    kit::measure::width(window, Type::Strong, words) + M * 2.0 + px(2.0)
}

/// The head: the back line, the name with Save and Start, the save row, the run's note.
fn head(
    view: &EditorView,
    (roster_width, content, narrow): (Pixels, Pixels, bool),
    focus: bool,
    cx: &mut Context<SplatRoot>,
) -> Div {
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
        actions = actions.child(
            reason("Close the open run first.")
                .whitespace_nowrap()
                .mr(S),
        );
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
    let column = if narrow { content } else { roster_width };
    let name = head_name(column).child(
        kit::TextField::new(
            ElementId::Name(format!("encounter-name:{}", view.draft).into()),
            view.name.clone(),
        )
        .name("Encounter name")
        .placeholder("Encounter name")
        .text_type(Type::Heading)
        .focus_selected(focus)
        .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
            root.app.encounters.edit_draft(|draft| draft.name(words));
            cx.notify();
        }))
        .on_enter(on_key(cx, |root: &mut SplatRoot| {
            root.app.encounters.ask_focus_search();
        })),
    );
    let name_row = head_row(name, actions);
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
            crate::gui::theme::color(*color),
        )));
    if let Some(blocked) = view.discard {
        row = row.child(
            kit::Button::quiet("encounter-discard", "Discard")
                .density(Density::Read)
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
        row = row
            .child(div().flex_1())
            .child(kit::status(&kit::Status::new(
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

/// The roster column: its heading, rows, options, loot, notes and the encounter's own groups.
fn roster_column(
    root: &mut SplatRoot,
    view: &EditorView,
    roster: &RosterView,
    owned: Option<splat_encounters_ui::encounters::model::OwnedGroups>,
    heading_shown: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let mut column = div().flex().flex_col().w_full();
    if heading_shown {
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
        column = column.child(
            div()
                .id("encounter-roster-heading")
                .pb(XS)
                .child(heading)
                .when_some(roster.budgets.clone(), |heading, budgets| {
                    kit::WithTooltip::with_tooltip(heading, budgets)
                }),
        );
    }
    if roster.rows.is_empty() {
        column = column.child(kit::gap(S)).child(reason(
            "No participants yet. Add the party and creatures from Add participants.",
        ));
    }
    let _ = window;
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
                    kit::Link::new("encounter-connect", "Connect to sync")
                        .small()
                        .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                            root.ctx.set(crate::requests::OPEN_ACCOUNT_SETTINGS, true);
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
        .child(loot_block(root, view, roster, cx))
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
    if let Some(owned) = owned {
        column = column.child(owned_groups(root, owned, cx));
    }
    kit::scroll_column(
        ElementId::Name(format!("encounter-roster-scroll:{}", view.draft).into()),
        column.max_w(LIST_MEASURE),
    )
}

/// The encounter's loot (`loot_field`): its heading, then the loot list, or why the note's
/// Loot section stays in the notes.
fn loot_block(
    root: &SplatRoot,
    view: &EditorView,
    roster: &RosterView,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let block = div().flex().flex_col().child(kit::heading(
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
    block
        .child(kit::help(
            "What the party finds when this fight ends. A creature’s own loot is in its Changes.",
        ))
        .child(loot::list(
            root,
            format!("encounter:{}", view.draft),
            "the encounter".to_owned(),
            roster.loot.lines.clone(),
            LootOwner::Encounter,
            cx,
        ))
}

/// Groups for this encounter (`groups::of_encounter`): each in a card of its own with its name,
/// `Add here` and its `⋯` (Move to, Make available to any encounter, Delete group), and its
/// fields; then `New group for this encounter`.
fn owned_groups(
    root: &SplatRoot,
    owned: splat_encounters_ui::encounters::model::OwnedGroups,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let mut block = div()
        .flex()
        .flex_col()
        .child(kit::heading(
            kit::Heading::Group {
                title: "Groups for this encounter",
                count: owned.count,
            },
            false,
        ))
        .child(kit::help(
            "They arrive when you add them to a run of this encounter. Save encounter saves them too.",
        ));
    if !owned.cards.is_empty() {
        block = block.child(kit::gap(XS));
    }
    for card in owned.cards {
        let index = card.index;
        let at = GroupAt::Owned(index);
        let mut move_to = Vec::new();
        for (id, name) in &card.move_to {
            move_to.push(
                kit::MenuItem::new(format!("Move to {name}")).on_click(owned_act(
                    cx,
                    index,
                    OwnedAct::MoveTo(*id),
                )),
            );
        }
        let mut items = move_to;
        items.push(
            kit::MenuItem::new("Make available to any encounter").on_click(owned_act(
                cx,
                index,
                OwnedAct::AnyEncounter,
            )),
        );
        items.push(
            kit::MenuItem::new("Delete group")
                .icon(icons::REMOVE)
                .on_click(owned_act(cx, index, OwnedAct::Delete)),
        );
        let adding = card.adding;
        let name = kit::TextField::new(
            ElementId::Name(format!("owned-group-name:{}", card.fields.id).into()),
            card.name.clone(),
        )
        .name(format!("Group name: {}", card.name))
        .text_type(Type::Heading)
        .focus_selected(card.focus)
        .on_change(
            cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                root.app
                    .encounters
                    .owned_act(index, OwnedAct::Name(words.to_owned()));
                cx.notify();
            }),
        );
        let top = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .max_w(crate::gui::theme::FORM_WIDTH)
                    .child(name),
            )
            .child(
                kit::Button::quiet(
                    ElementId::Name(format!("owned-group-add:{}", card.fields.id).into()),
                    if adding { "Adding here" } else { "Add here" },
                )
                .icon(if adding { icons::OK } else { icons::ADD })
                .density(Density::Read)
                .selected(adding)
                .name(format!("Add creatures to {}", card.name))
                .on_click(owned_act(cx, index, OwnedAct::AddHere)),
            )
            .child(row_menu(
                ElementId::Name(format!("owned-group-menu:{}", card.fields.id).into()),
                &card.name,
                items,
            ));
        let fields = super::group::fields(root, at, &card.fields, cx);
        block = block
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(XS)
                    .p(S + crate::gui::theme::FOCUS_WIDTH
                        - if adding {
                            crate::gui::theme::FOCUS_WIDTH
                        } else {
                            HAIRLINE_WIDTH
                        })
                    .rounded(crate::gui::theme::RADIUS_CONTROL)
                    .border(if adding {
                        crate::gui::theme::FOCUS_WIDTH
                    } else {
                        HAIRLINE_WIDTH
                    })
                    .border_color(if adding { c.accent } else { c.rule })
                    .child(top)
                    .child(fields),
            )
            .child(kit::gap(S));
    }
    block.child(
        div().flex().child(
            kit::Button::secondary("owned-group-new", "New group for this encounter")
                .icon(icons::ADD)
                .density(Density::Read)
                .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                    root.app.encounters.new_owned_group();
                    cx.notify();
                })),
        ),
    )
}

fn owned_act(
    cx: &mut Context<SplatRoot>,
    index: usize,
    act: OwnedAct,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static {
    cx.listener(move |root: &mut SplatRoot, _, _, cx| {
        root.app.encounters.owned_act(index, act.clone());
        cx.notify();
    })
}

/// One roster row: its name and second line, Edit and its menu, and while open its fields and
/// its Changes, or a missing creature's replacement picker.
fn roster_row(
    root: &mut SplatRoot,
    view: &EditorView,
    row: &RosterRow,
    cx: &mut Context<SplatRoot>,
) -> Div {
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
            line = line.child(
                div()
                    .text_color(crate::gui::theme::color(*color))
                    .child(words.clone()),
            );
        }
    }
    if let Some((icon, color, _)) = &row.missing {
        line = line
            .child(div().child(" · "))
            .child(small_icon(icon, crate::gui::theme::color(*color)));
    }
    if let Some(word) = &row.changed {
        let open = key.clone();
        line = line
            .child(div().child(" · "))
            .child(small_icon(icons::CHANGED, c.muted).mr(XS))
            .child(
                kit::Link::new(
                    ElementId::Name(format!("roster-change:{key}").into()),
                    word.clone(),
                )
                .small()
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
        items.push(
            kit::MenuItem::new("Make it yours").on_click(on_page(
                cx,
                move |page, _, active, campaigns| {
                    page.make_yours(index, active, campaigns.modules())
                },
            )),
        );
    }
    items.push(
        kit::MenuItem::new("Remove")
            .icon(icons::REMOVE)
            .on_click(roster_act(cx, RosterAct::Remove(index))),
    );
    let action: AnyElement = match &row.repair {
        Some((label, path)) => {
            let path = path.clone();
            kit::Button::quiet(
                ElementId::Name(format!("roster-repair:{key}").into()),
                *label,
            )
            .density(Density::Read)
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                root.ctx
                    .set(crate::requests::OPEN_LIBRARY_REPAIR, path.clone());
                cx.notify();
            }))
            .into_any_element()
        }
        None => {
            let toggle = key.clone();
            kit::Button::quiet(
                ElementId::Name(format!("roster-edit:{key}").into()),
                row.edit_word,
            )
            .name(format!("{} {}", row.edit_word, row.name))
            .density(Density::Read)
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
                    .child(row_menu(
                        ElementId::Name(format!("roster-menu:{key}").into()),
                        &row.name,
                        items,
                    )),
            ),
    );
    if let Some(form) = &row.form {
        block = block.child(row_form(root, view, row, form, cx));
        if let Some(changes) = root
            .app
            .encounters
            .changes_form(&ChangesOf::Row(key.clone()))
        {
            block = block.child(changes::form(
                root,
                ChangesOf::Row(key.clone()),
                changes,
                cx,
            ));
        }
    } else if row.open && row.missing.is_some() && row.repair.is_none() {
        let count = root.app.encounters.missing_rows(&key);
        let replaced = key.clone();
        block = block.child(changes::replacement(
            root,
            &key,
            count,
            &row.name,
            move |root: &mut SplatRoot, creature, all| {
                root.app.encounters.replace_row(&replaced, creature, all);
                root.pages.encounters.replacing.remove(&replaced);
            },
            cx,
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
    let side_key = key.clone();
    let side_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(reason("Side"))
        .child(super::common::side_choice(
            root,
            (format!("roster-side:{key}"), "Side"),
            &form.side,
            &form.sides,
            move |root: &mut SplatRoot, side: &str| {
                root.app
                    .encounters
                    .edit_draft(|draft| draft.side(&side_key, side));
            },
            cx,
        ));
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
            .on_change(
                cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                    root.app
                        .encounters
                        .edit_draft(|draft| draft.initiative_group(&group_key, words));
                    cx.notify();
                }),
            ),
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
                .on_change(cx.listener(
                    move |root: &mut SplatRoot, words: &str, _, cx| {
                        if let Ok(number) = words.trim().parse::<i64>() {
                            root.app
                                .encounters
                                .edit_draft(|draft| draft.value(&value_key, field, Some(number)));
                        }
                        cx.notify();
                    },
                )),
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
    unused: &[UnusedCreature],
    choices: Option<&super::filter::Choices>,
    focus: bool,
    heading: bool,
    summary: Option<&str>,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let group = matches!(add.party, PartyButton::None);
    let mut head = div().flex().flex_col().gap(XS);
    if heading {
        head = head.child(div().pb(XS).child(kit::label(
            if group {
                "Add creatures"
            } else {
                "Add participants"
            },
            0,
        )));
    }
    head = head.child(kit::help(if group {
        "From this campaign and the packages it uses. Party members go on an encounter’s roster."
    } else {
        splat_encounters_ui::encounters::PARTY_SOURCE
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
    head = head.child(search(add, choices, focus, name, cx));
    if let Some(summary) = summary.filter(|summary| !summary.is_empty()) {
        head = head.child(kit::WithTooltip::with_tooltip(
            kit::text::one_line(Type::Small, c.muted, summary.to_owned())
                .id("group-member-summary"),
            summary.to_owned(),
        ));
    }
    if add.close_names {
        head = head.child(kit::text(
            Type::Small,
            c.muted,
            "No exact match. These names are close.",
        ));
    }
    for (at, unused) in unused.iter().enumerate() {
        let (package, entry) = (unused.package, unused.entry);
        head = head.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_baseline()
                .gap(S)
                .child(kit::text(Type::Small, c.muted, unused.words.clone()))
                .child(
                    kit::Link::new(ElementId::Name(format!("use-it:{at}").into()), "Use it")
                        .small()
                        .name(format!("Use it: {}", unused.words))
                        .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                            root.ctx
                                .set(crate::requests::USE_PACKAGE_ENTRY, (package, entry));
                            cx.notify();
                        })),
                ),
        );
    }
    head
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
                .focus_selected(focus)
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
            kit::Button::quiet("encounter-search-clear", "Clear")
                .density(Density::Read)
                .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                    root.app.encounters.set_search("");
                    cx.notify();
                })),
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
    unused: &[UnusedCreature],
    choices: Option<&super::filter::Choices>,
    (focus, heading): (bool, bool),
    (handle, facts): (&gpui::UniformListScrollHandle, Pixels),
    cx: &mut Context<SplatRoot>,
) -> Div {
    add_column_with(
        add,
        unused,
        choices,
        (focus, heading, None),
        (handle, facts),
        cx,
    )
}

/// Where the results' action column starts (layout PR 4): `L` after the widest facts of every
/// result, so each `Add` keeps its x as the DM scrolls. Measured once per ranking (kept in the
/// page's state by the results' list), at most the column's room less the action.
pub(super) fn results_facts(
    root: &mut SplatRoot,
    add: &AddView,
    room: Pixels,
    window: &mut Window,
) -> Pixels {
    // The ranking's list, while the page keeps it (`add_view`), with its length.
    let key = (Arc::as_ptr(&add.results) as usize, add.results.len());
    let widest = match root.pages.encounters.results_facts {
        Some((kept, width)) if kept == key => width,
        _ => {
            let stats = px(48.0 + 52.0 + 40.0);
            let width = add
                .results
                .iter()
                .map(|row| {
                    let name = kit::measure::width(window, Type::Body, &row.name);
                    let detail = match &row.unavailable {
                        Some(why) => {
                            kit::measure::width(window, Type::Small, why) + Type::Small.size() + XS
                        }
                        None => {
                            stats
                                + row.source.as_deref().map_or(px(0.0), |source| {
                                    XS + kit::measure::width(window, Type::Small, source)
                                })
                        }
                    };
                    name.max(detail)
                })
                .fold(px(0.0), Pixels::max);
            root.pages.encounters.results_facts = Some((key, width));
            width
        }
    };
    // The action: `Add` and, beside a blocked one, `Open in World`.
    let action = button_width(window, "Add") + S + button_width(window, "Open in World");
    widest.min((room - L - action - crate::gui::theme::SCROLL_BAR_WIDTH).max(px(120.0)))
}

/// Add participants (or Add creatures, with the members' summary in the narrow group editor).
pub(super) fn add_column_with(
    add: &AddView,
    unused: &[UnusedCreature],
    choices: Option<&super::filter::Choices>,
    (focus, heading, summary): (bool, bool, Option<&str>),
    (handle, facts): (&gpui::UniformListScrollHandle, Pixels),
    cx: &mut Context<SplatRoot>,
) -> Div {
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex_none()
                .child(add_head(add, unused, choices, focus, heading, summary, cx)),
        )
        .child(
            div()
                .flex_1()
                .min_h_0()
                .pt(S)
                .child(results(add, (handle, facts), cx)),
        )
}

/// The search's results: a virtual list of rows with its scroll bar, or the empty state.
pub(super) fn results(
    add: &AddView,
    (handle, facts): (&gpui::UniformListScrollHandle, Pixels),
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    if add.results.is_empty() {
        let words = if add.filter.is_empty() {
            "Check the spelling, or check"
        } else {
            "Remove a filter, or check"
        };
        // One sentence in one size: the link is a Body link inside it, and the full stop
        // follows its last word.
        let body = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .justify_center()
            .items_baseline()
            .gap(XS)
            .child(kit::text(Type::Body, c.muted, words))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_baseline()
                    .child(
                        kit::Link::new("encounter-packages", "Campaign → Packages").on_click(
                            cx.listener(|root: &mut SplatRoot, _, _, cx| {
                                root.ctx.set(crate::requests::OPEN_CAMPAIGN_PACKAGES, true);
                                cx.notify();
                            }),
                        ),
                    )
                    .child(kit::text(Type::Body, c.muted, ".")),
            );
        return kit::empty_state(icons::SEARCH, "No creatures match", "", Some(body))
            .into_any_element();
    }
    let rows: Arc<Vec<ResultRow>> = add.results.clone();
    let status = rows.iter().any(|row| row.unavailable.is_some());
    let list = kit::virtual_list::uniform_column(
        "encounter-results",
        handle,
        rows.len(),
        cx.processor(
            move |_root: &mut SplatRoot, range: std::ops::Range<usize>, _window, cx| {
                range
                    .filter_map(|at| rows.get(at).map(|row| result_row(row, (status, facts), cx)))
                    .collect::<Vec<_>>()
            },
        ),
    );
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(div().flex_1().min_h_0().child(list))
        .into_any_element()
}

/// One result: its name (it opens the entry in Lookup) and stats, or why it cannot be added,
/// then Add (with `Open in World` when it cannot), or `In roster`.
fn result_row(
    row: &ResultRow,
    (status, facts): (bool, Pixels),
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let detail: AnyElement = match &row.unavailable {
        // The warning wraps in the two lines its row keeps; it is never cut short in one.
        Some(why) => div()
            .flex()
            .flex_row()
            .items_start()
            .gap(XS)
            .max_h(Type::Small.line() * 2.0)
            .overflow_hidden()
            .child(small_icon(icons::WARN, c.result.warning))
            .child(div().flex_1().min_w_0().child(kit::text(
                Type::Small,
                c.result.warning,
                why.clone(),
            )))
            .into_any_element(),
        None => div()
            .flex()
            .flex_row()
            .items_baseline()
            .ty(Type::Small)
            .text_color(c.muted)
            .whitespace_nowrap()
            .min_w_0()
            .overflow_hidden()
            .children(stat_cells(&row.detail, row.hp_missing))
            .children(row.source.clone().map(|source| div().ml(XS).child(source)))
            .into_any_element(),
    };
    let action: AnyElement = if row.in_roster {
        kit::status(&kit::Status::new(icons::OK, "In roster", c.muted)).into_any_element()
    } else {
        let (id, party) = (row.id, row.party);
        let add = kit::Button::secondary(ElementId::Name(format!("add:{}", row.id).into()), "Add")
            .name(format!("Add {}", row.name))
            .density(Density::Read)
            .blocked(row.unavailable.clone())
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                root.app.encounters.add_result(id, party);
                cx.notify();
            }));
        let mut action = div().flex().flex_row().items_center().gap(S).child(add);
        if row.unavailable.is_some() {
            let creature = row.id;
            action = action.child(
                kit::Button::quiet(
                    ElementId::Name(format!("open-in-world:{}", row.id).into()),
                    "Open in World",
                )
                .density(Density::Read)
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    root.ctx
                        .set(crate::requests::REVEAL_ENCOUNTER_NPC, creature);
                    cx.notify();
                })),
            );
        }
        action.into_any_element()
    };
    div()
        .id(ElementId::Name(format!("result:{}", row.id).into()))
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .h(result_height(status))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_none()
                .w(facts)
                .min_w_0()
                .child(div().flex().child(name_link(row, cx)))
                .child(detail),
        )
        .child(div().flex_none().child(action))
        .into_any_element()
}

/// A creature's name that opens its entry in Lookup (`creature_name_link`): its words in the
/// text colour, underlined while the pointer is on it.
fn name_link(row: &ResultRow, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let peek = row.id;
    kit::target(
        div()
            .id(ElementId::Name(format!("result-read:{}", row.id).into()))
            .flex_none()
            .min_h(Density::Read.control() - XS)
            .flex()
            .items_center()
            .cursor_pointer()
            .ty(Type::Body)
            .text_color(c.text)
            .whitespace_nowrap()
            .hover(|style| style.underline())
            .child(row.name.clone())
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                root.app.encounters.peek = Some(peek);
                cx.notify();
            })),
        row.name.clone(),
    )
}

/// A result row's height: its two lines and its Add, with room between rows; a third line while
/// a result's warning can take two (`Row::Action { status }`).
fn result_height(status: bool) -> Pixels {
    let base = Density::Scan.row() + S;
    if status {
        base + Type::Small.line()
    } else {
        base
    }
}

/// A result's stats, each in its own cell (`detail_columns`): CR, HP and AC.
pub(super) fn stat_cells(detail: &str, hp_missing: bool) -> Vec<Div> {
    let c = colors();
    let parts: Vec<&str> = detail.split(" · ").collect();
    let other = parts
        .iter()
        .find(|part| {
            !["CR ", "HP ", "AC "]
                .iter()
                .any(|prefix| part.starts_with(prefix))
        })
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
