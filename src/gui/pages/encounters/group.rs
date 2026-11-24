//! The group editor (egui: `Encounters::group_editor_view`, `group_fields`, `members`): the back
//! line, the name with Save group, the save row, Default side, Belongs to, the members with their
//! counts, Changes and replacements, the notes, Add creatures, and the footer. Below 760 px the
//! members and Add creatures take turns under tabs. While its file changed on disk, the conflict
//! takes the editor's place. An encounter's own groups draw their fields here too (`fields`).
use super::changes;
use super::common::{egui_color, footer, on_page, reason, row_menu, with_page};
use super::editor::{NARROW, add_column_with, back_line, button_width, content_width, filter};
use crate::gui::theme::{
    Density, HAIRLINE_WIDTH, L, LIST_MEASURE, M, PANEL_MIN_WIDTH, S, Type, XS, colors,
};
use crate::gui::{SplatRoot, icons, kit};
use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, Context, Div, ElementId, IntoElement, ParentElement, Styled, Window, div, px,
};
use splat_encounters_ui::encounters::model::{
    ChangesOf, GroupAt, GroupFields, GroupView, HeadAct, MemberAct, MemberRow, NarrowPane,
};

/// The group editor.
pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let Some(active) = root.app.campaigns.active_campaign() else {
        return div().into_any_element();
    };
    if let Some(disk) = root.app.encounters.group_disk() {
        return super::disk::view(
            "group-disk",
            &disk,
            |root: &mut SplatRoot, keep| {
                with_page(root, |page, ctx, active, campaigns| {
                    page.group_disk_choice(ctx, active, campaigns.modules(), keep)
                });
            },
            cx,
        );
    }
    let ctx = root.bridge.ctx().clone();
    let sources = splat_encounters_ui::loot_editor::sources(&ctx);
    let Some(view) = root.app.encounters.group_frame(sources.as_deref()) else {
        return div().into_any_element();
    };
    let add = root.app.encounters.add_view(filter(&ctx));
    let lines = root.app.encounters.group_footer(&active.path);
    let choices = kit::menu::is_open(&super::filter::menu_id(), cx)
        .then(|| root.app.encounters.filter_choices());
    let focus_name = root.app.encounters.take_focus_name();
    let focus_search = root.app.encounters.take_focus_search();
    let content = content_width(window);
    let narrow = content < NARROW;
    // The Add creatures column keeps its own width here (egui's group editor does not drag).
    let add_width = px(crate::design::side_panel_width(f32::from(content)))
        .max(content - LIST_MEASURE)
        .min(super::editor::ADD_PANE_MAX_WIDTH);
    let members_width = (content - add_width).clamp(PANEL_MIN_WIDTH, LIST_MEASURE);
    let head = head(
        &view,
        if narrow { content } else { members_width },
        content,
        focus_name,
        window,
        cx,
    );
    let handle = root.pages.encounters.results_scroll.clone();
    let body: AnyElement = match add {
        Some(add) if !view.reading => {
            let members = kit::Pane::new(("group-members", view.id.as_u128() as u64))
                .flush()
                .child(kit::scroll_column(
                    ("group-members-scroll", view.id.as_u128() as u64),
                    fields(root, GroupAt::Open, &view.fields, cx).max_w(LIST_MEASURE),
                ));
            if narrow {
                let summary = view.summary.clone();
                let side =
                    add_column_with(&add, &[], choices.as_ref(), (focus_search, false, Some(&summary)), &handle, cx);
                narrow_body(root, &view, members, side.into_any_element(), cx)
            } else {
                div()
                    .flex()
                    .flex_row()
                    .size_full()
                    .child(div().flex_1().min_w_0().max_w(LIST_MEASURE).pr(S).child(members))
                    .child(
                        div().flex_none().w(add_width).h_full().pl(M).child(add_column_with(
                            &add,
                            &[],
                            choices.as_ref(),
                            (focus_search, true, None),
                            &handle,
                            cx,
                        )),
                    )
                    .into_any_element()
            }
        }
        _ => super::editor::reading(),
    };
    let footer_lines = footer("group-editor-footer", lines, cx);
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

/// The narrow group editor: `Members N` and `Add creatures` as tabs, the chosen pane under them.
fn narrow_body(
    root: &SplatRoot,
    view: &GroupView,
    members: kit::Pane,
    side: AnyElement,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let pane = root.app.encounters.narrow_pane();
    let count = if view.fields.count == 0 {
        "Members".to_owned()
    } else {
        format!("Members {}", crate::design::count(view.fields.count, "", ""))
    };
    let tabs = kit::Segmented::new(
        "group-narrow-tabs",
        [count, "Add creatures".to_owned()],
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
    div()
        .flex()
        .flex_col()
        .size_full()
        .gap(S)
        .child(div().flex().child(tabs))
        .child(div().flex_1().min_h_0().child(match pane {
            NarrowPane::Roster => members.into_any_element(),
            NarrowPane::Add => side,
        }))
        .into_any_element()
}

fn head(
    view: &GroupView,
    column: gpui::Pixels,
    line: gpui::Pixels,
    focus: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let back = back_line(
        "group-back",
        view.back.word,
        view.back.name,
        view.back.held,
        on_page(cx, |page, ctx, active, campaigns| {
            page.group_head_act(ctx, active, campaigns.modules(), HeadAct::Back);
        }),
    );
    let save = if view.save_primary {
        kit::Button::primary("group-save", "Save group")
    } else {
        kit::Button::secondary("group-save", "Save group")
    }
    .blocked(view.save_blocked.clone())
    .key("Ctrl+S")
    .on_click(on_page(cx, |page, ctx, active, campaigns| {
        page.group_head_act(ctx, active, campaigns.modules(), HeadAct::Save);
    }));
    let (name_width, beside) =
        super::editor::head_plan(column, line, button_width(window, "Save group"));
    let name = div().w(name_width).flex_none().child(
        kit::TextField::new(
            ElementId::Name(format!("group-name:{}", view.id).into()),
            view.name.clone(),
        )
        .name("Group name")
        .placeholder("Group name")
        .text_type(Type::Heading)
        .focus_selected(focus)
        .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
            root.app.encounters.edit_group(|group| group.name(words));
            cx.notify();
        })),
    );
    let names: Vec<AnyElement> = if beside {
        vec![
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(L)
                .child(name)
                .child(save)
                .into_any_element(),
        ]
    } else {
        vec![
            name.into_any_element(),
            div().flex().child(save).into_any_element(),
        ]
    };
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
    if view.discard {
        row = row.child(
            kit::Button::quiet("group-discard", "Discard")
                .density(Density::Read)
                .on_click(on_page(cx, |page, ctx, active, campaigns| {
                    page.group_head_act(ctx, active, campaigns.modules(), HeadAct::Discard);
                })),
        );
    }
    for words in [view.elsewhere.or(view.lock_reason), view.empty_reason]
        .into_iter()
        .flatten()
    {
        row = row.child(reason(words).whitespace_nowrap().ml(S));
    }
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(S)
        .child(back)
        .children(names)
        .child(row)
}

/// Default side, Belongs to, the members and the notes (`group_fields`), for the group editor
/// and for each of an encounter's own groups.
pub(super) fn fields(
    root: &SplatRoot,
    at: GroupAt,
    view: &GroupFields,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let key = format!("{at:?}:{}", view.id);
    let label = |words: &'static str| div().w(px(80.0)).flex_none().child(reason(words));
    let side = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(label("Default side"))
        .child(super::common::side_choice(
            root,
            (format!("group-side:{key}"), "Default side"),
            &view.side,
            &[],
            move |root: &mut SplatRoot, side: &str| {
                root.app
                    .encounters
                    .edit_group_at(at, |group| group.side(side));
            },
            cx,
        ));
    let ids: Vec<_> = view.belongs.iter().map(|(id, _)| *id).collect();
    let belongs = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(label("Belongs to"))
        .child(
            div().flex_1().child(
                kit::Dropdown::new(
                    ElementId::Name(format!("group-belongs:{key}").into()),
                    "Belongs to",
                    view.belongs
                        .iter()
                        .map(|(_, name)| name.clone())
                        .collect::<Vec<_>>(),
                    view.belongs_chosen,
                )
                .density(Density::Read)
                .on_select(cx.listener(
                    move |root: &mut SplatRoot, index: &usize, _, cx| {
                        if let Some(id) = ids.get(*index) {
                            root.app
                                .encounters
                                .edit_group_at(at, |group| group.belongs(*id));
                        }
                        cx.notify();
                    },
                )),
            ),
        );
    let mut heading = kit::label("Members", view.count);
    if !view.heading.is_empty() {
        heading = heading.child(kit::text(
            Type::Small,
            c.muted,
            format!("· {}", view.heading),
        ));
    }
    let mut column = div()
        .flex()
        .flex_col()
        .gap(XS)
        .w_full()
        .child(side)
        .child(kit::help(
            "Party, enemies, or any other name, such as rivals.",
        ))
        .child(kit::gap(S))
        .child(belongs)
        .children(view.belongs_help.map(kit::help))
        .children(
            view.belongs_fault
                .map(|words| kit::Message::new(kit::Tone::Warning, words).small()),
        )
        .child(kit::gap(S))
        .child(heading);
    if view.members.is_empty() && view.unread.is_empty() {
        column = column.child(reason("No members yet. Add creatures from the search."));
    }
    for words in &view.unread {
        column = column.child(kit::Message::new(kit::Tone::Warning, words.clone()).small());
    }
    for member in &view.members {
        column = column.child(member_row(root, at, member, cx));
    }
    column.child(kit::gap(M)).child(kit::field(
        "Group notes",
        None,
        None,
        kit::TextEditor::new(
            ElementId::Name(format!("group-notes:{key}").into()),
            view.notes.clone(),
        )
        .name("Group notes")
        .placeholder("When they arrive, what they want. Markdown is fine.")
        .rows(4)
        .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
            root.app
                .encounters
                .edit_group_at(at, |group| group.notes(words));
            cx.notify();
        })),
    ))
}

fn act(
    cx: &mut Context<SplatRoot>,
    at: GroupAt,
    act: MemberAct,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static {
    cx.listener(move |root: &mut SplatRoot, _, _, cx| {
        root.app.encounters.member_act_at(at, act);
        cx.notify();
    })
}

/// The member's element ids: the open group's keep egui's plain names (`member-more-0`), an
/// encounter's own group's add its place.
fn member_id(at: GroupAt, words: &str, index: usize) -> ElementId {
    match at {
        GroupAt::Open => ElementId::Name(format!("{words}-{index}").into()),
        GroupAt::Owned(group) => ElementId::Name(format!("{words}-{group}-{index}").into()),
    }
}

/// One member: its name and stats with its change words, its fault, its count with − and +,
/// Edit (or `Replace…`) and its menu; while open its Changes, or the replacement picker.
fn member_row(
    root: &SplatRoot,
    at: GroupAt,
    member: &MemberRow,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let index = member.index;
    let count = member.count;
    let mut detail = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_baseline()
        .gap(XS);
    if !member.detail.is_empty() {
        detail = detail.child(kit::text(Type::Small, c.muted, member.detail.clone()));
    }
    if member.hp_missing && !member.detail.contains("HP not set") {
        detail = detail.child(kit::status(&kit::Status::new(
            icons::WARN,
            "HP not set",
            c.result.warning,
        )));
    }
    if let Some(word) = &member.change_word {
        if !member.detail.is_empty() {
            detail = detail.child(kit::text(Type::Small, c.muted, "·"));
        }
        detail = detail
            .child(kit::icon(icons::CHANGED, Type::Small, c.muted))
            .child(
                kit::Link::new(member_id(at, "member-change", index), word.clone())
                    .small()
                    .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                        root.app.encounters.toggle_member(at, index);
                        cx.notify();
                    })),
            );
    }
    let stepper = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .child(reason("Count"))
        .child(
            kit::IconButton::new(
                member_id(at, "member-less", index),
                icons::SUBTRACT,
                format!("Fewer {}", member.name),
            )
            .on_click(act(cx, at, MemberAct::Count(index, count.saturating_sub(1)))),
        )
        .child(
            kit::TextField::new(member_id(at, "member-count", index), count.to_string())
                .name(format!("How many of {}", member.name))
                .width(px(40.0))
                .on_change(
                    cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                        if let Ok(number) = words.trim().parse::<u64>() {
                            root.app
                                .encounters
                                .member_act_at(at, MemberAct::Count(index, number));
                        }
                        cx.notify();
                    }),
                ),
        )
        .child(
            kit::IconButton::new(
                member_id(at, "member-more", index),
                icons::ADD,
                format!("More {}", member.name),
            )
            .on_click(act(cx, at, MemberAct::Count(index, count + 1))),
        );
    let items = vec![
        kit::MenuItem::new("Move up")
            .icon(icons::MOVE_UP)
            .blocked(member.first.then_some("Already first."))
            .on_click(act(cx, at, MemberAct::Up(index))),
        kit::MenuItem::new("Move down")
            .icon(icons::MOVE_DOWN)
            .blocked(member.last.then_some("Already last."))
            .on_click(act(cx, at, MemberAct::Down(index))),
        kit::MenuItem::new("Remove")
            .icon(icons::REMOVE)
            .on_click(act(cx, at, MemberAct::Remove(index))),
    ];
    let edit = kit::Button::quiet(member_id(at, "member-edit", index), member.edit_word)
        .name(member.edit_name.clone())
        .density(Density::Read)
        .selected(member.open)
        .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
            root.app.encounters.toggle_member(at, index);
            cx.notify();
        }));
    let mut block = div()
        .flex()
        .flex_col()
        .gap(XS)
        .py(XS)
        .child(kit::text::one_line(
            Type::Strong,
            c.text,
            format!("{}. {}", member.number, member.name),
        ))
        .child(detail)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(stepper)
                .child(div().flex_1())
                .child(edit)
                .child(row_menu(member_id(at, "member-menu", index), &member.name, items)),
        );
    if let Some((words, error)) = &member.fault {
        // The fault, its help under its words, then its repair at them.
        block = block.child(
            kit::Message::new(
                if *error {
                    kit::Tone::Error
                } else {
                    kit::Tone::Warning
                },
                words.clone(),
            )
            .small()
            .detail(kit::help(
                "Choose a replacement, check the packages it uses, or remove this member.",
            ))
            .action(
                kit::Link::new(member_id(at, "member-packages", index), "Campaign → Packages")
                    .small()
                    .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                        root.bridge.ctx().data_mut(|data| {
                            data.insert_temp(
                                eframe::egui::Id::new("open_campaign_packages"),
                                true,
                            )
                        });
                        cx.notify();
                    })),
            ),
        );
        if member.open {
            let key = member.key.clone();
            let gone = key.clone();
            block = block.child(changes::replacement(
                root,
                &key,
                1,
                "",
                move |root: &mut SplatRoot, creature, _| {
                    root.app.encounters.replace_member(at, index, creature);
                    root.pages.encounters.replacing.remove(&gone);
                },
                cx,
            ));
        }
    }
    if let Some(words) = &member.change_fault {
        block = block.child(kit::Message::new(kit::Tone::Error, words.clone()).small());
    }
    if member.open
        && let Some(form) = root
            .app
            .encounters
            .changes_form(&ChangesOf::Member(at, index))
    {
        block = block.child(changes::form(root, ChangesOf::Member(at, index), form, cx));
    }
    block.child(kit::rule())
}
