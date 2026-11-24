//! The group editor (egui: `Encounters::group_editor_view`, `group_fields`, `members`): the back
//! line, the name with Save group, the save row, Default side, Belongs to, the members with their
//! counts, the notes, Add creatures, and the footer.
use super::common::{egui_color, footer, on_page, reason, row_menu, small_icon};
use super::editor::{add_column, back_line, filter};
use crate::gui::theme::{Density, HAIRLINE_WIDTH, L, LIST_MEASURE, M, PANEL_MIN_WIDTH, S, Type, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use gpui::prelude::FluentBuilder as _;
use gpui::{AnyElement, Context, Div, ElementId, IntoElement, ParentElement, Styled, Window, div, px};
use splat_encounters_ui::encounters::model::{GroupView, HeadAct, MemberAct, MemberRow};

/// The group editor.
pub(super) fn render(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    let Some(active) = root.app.campaigns.active_campaign() else {
        return div().into_any_element();
    };
    let Some(view) = root.app.encounters.group_frame() else {
        return div().into_any_element();
    };
    let ctx = root.bridge.ctx().clone();
    let add = root.app.encounters.add_view(filter(&ctx));
    let lines = root.app.encounters.group_footer(&active.path);
    let choices = kit::menu::is_open(&super::filter::menu_id(), cx)
        .then(|| root.app.encounters.filter_choices());
    let focus_name = root.app.encounters.take_focus_name();
    let focus_search = root.app.encounters.take_focus_search();
    let content = window.viewport_size().width - crate::gui::theme::RAIL_WIDTH - M * 2.0;
    let add_width = px(crate::design::side_panel_width(f32::from(content)))
        .max(content - LIST_MEASURE)
        .min(px(640.0));
    let members_width = (content - add_width).max(PANEL_MIN_WIDTH);
    let head = head(&view, members_width, focus_name, cx);
    let body: AnyElement = match add {
        Some(add) if !view.reading => kit::Columns::new("group-editor-columns")
            .column(
                members_width,
                PANEL_MIN_WIDTH..LIST_MEASURE,
                div().size_full().pr(S).child(
                    kit::Pane::new(("group-members", view.id.as_u128() as u64))
                        .flush()
                        .child(fields(&view, cx)),
                ),
            )
            .rest(add_column(&add, choices.as_ref(), focus_search, cx))
            .into_any_element(),
        _ => div()
            .flex()
            .flex_row()
            .gap(S)
            .child(small_icon(icons::BUSY, colors().muted))
            .child(kit::text(Type::Body, colors().muted, "Reading this campaign…"))
            .into_any_element(),
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

fn head(view: &GroupView, width: gpui::Pixels, focus: bool, cx: &mut Context<SplatRoot>) -> Div {
    let back = back_line(
        "group-back",
        view.back.word,
        view.back.name,
        view.back.held,
        on_page(cx, |page, ctx, active, campaigns| {
            page.group_head_act(ctx, active, campaigns.modules(), HeadAct::Back);
        }),
    );
    let name = kit::TextField::new(
        ElementId::Name(format!("group-name:{}", view.id).into()),
        view.name.clone(),
    )
    .name("Group name")
    .placeholder("Group name")
    .focus_selected(focus)
    .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
        root.app.encounters.edit_group(|group| group.name(words));
        cx.notify();
    }));
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
    let (icon, word, color) = &view.state;
    let mut row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_h(Density::Read.control())
        .child(kit::status(&kit::Status::new(icon.clone(), word.clone(), egui_color(*color))));
    if view.discard {
        row = row.child(kit::Button::quiet("group-discard", "Discard").on_click(on_page(
            cx,
            |page, ctx, active, campaigns| {
                page.group_head_act(ctx, active, campaigns.modules(), HeadAct::Discard);
            },
        )));
    }
    for words in [view.elsewhere.or(view.lock_reason), view.empty_reason].into_iter().flatten() {
        row = row.child(reason(words).whitespace_nowrap().ml(S));
    }
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(S)
        .child(back)
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(L)
                .child(div().w(width - L).min_w(px(200.0)).child(name))
                .child(save),
        )
        .child(row)
}

/// Default side, Belongs to, the members and the notes (`group_fields`).
fn fields(view: &GroupView, cx: &mut Context<SplatRoot>) -> Div {
    let c = colors();
    let label = |words: &'static str| div().w(px(80.0)).flex_none().child(reason(words));
    let mut sides = vec!["party".to_owned(), "enemies".to_owned()];
    if !sides.contains(&view.side) && !view.side.trim().is_empty() {
        sides.push(view.side.clone());
    }
    let chosen = sides.iter().position(|side| *side == view.side);
    let words: Vec<String> = sides
        .iter()
        .map(|side| crate::design::reader::human_label(side))
        .collect();
    let picked = sides.clone();
    let side = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(label("Default side"))
        .child(
            kit::Dropdown::new("group-side", "Default side", words, chosen)
                .width(px(104.0))
                .on_select(cx.listener(move |root: &mut SplatRoot, index: &usize, _, cx| {
                    if let Some(side) = picked.get(*index) {
                        root.app.encounters.edit_group(|group| group.side(side));
                    }
                    cx.notify();
                })),
        )
        .child(
            kit::TextField::new(("group-side-other", view.id.as_u128() as u64), "")
                .name("Other side for Default side")
                .placeholder("Other side")
                .width(px(120.0))
                .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                    root.app.encounters.edit_group(|group| group.side(words));
                    cx.notify();
                })),
        );
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
                    "group-belongs",
                    "Belongs to",
                    view.belongs.iter().map(|(_, name)| name.clone()).collect::<Vec<_>>(),
                    view.belongs_chosen,
                )
                .on_select(cx.listener(move |root: &mut SplatRoot, index: &usize, _, cx| {
                    if let Some(id) = ids.get(*index) {
                        root.app.encounters.edit_group(|group| group.belongs(*id));
                    }
                    cx.notify();
                })),
            ),
        );
    let mut heading = kit::label("Members", view.count);
    if !view.heading.is_empty() {
        heading = heading.child(kit::text(Type::Small, c.muted, format!("· {}", view.heading)));
    }
    let mut column = div()
        .flex()
        .flex_col()
        .gap(XS)
        .w_full()
        .max_w(LIST_MEASURE)
        .child(side)
        .child(kit::help("Party, enemies, or any other name, such as rivals."))
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
        column = column.child(member_row(member, cx));
    }
    column
        .child(kit::gap(M))
        .child(kit::field(
            "Group notes",
            None,
            None,
            kit::TextEditor::new(("group-notes", view.id.as_u128() as u64), view.notes.clone())
                .name("Group notes")
                .placeholder("When they arrive, what they want. Markdown is fine.")
                .rows(4)
                .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                    root.app.encounters.edit_group(|group| group.notes(words));
                    cx.notify();
                })),
        ))
}

fn act(
    cx: &mut Context<SplatRoot>,
    act: MemberAct,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static {
    cx.listener(move |root: &mut SplatRoot, _, _, cx| {
        root.app.encounters.member_act(act);
        cx.notify();
    })
}

/// One member: its name and stats, its count with − and +, and its menu.
fn member_row(member: &MemberRow, cx: &mut Context<SplatRoot>) -> Div {
    let c = colors();
    let index = member.index;
    let count = member.count;
    let detail = if member.missing {
        div().child(kit::status(&kit::Status::new(
            icons::WARN,
            "Not found. Its package entry was deleted.",
            c.result.warning,
        )))
    } else {
        div()
            .ty_small()
            .child(kit::text(Type::Small, c.muted, member.detail.clone()))
            .when(member.hp_missing, |line| {
                line.child(kit::status(&kit::Status::new(icons::WARN, "HP not set", c.result.warning)))
            })
    };
    let stepper = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .child(reason("Count"))
        .child(
            kit::IconButton::new(("member-less", index), icons::SUBTRACT, format!("Fewer {}", member.name))
                .on_click(act(cx, MemberAct::Count(index, count.saturating_sub(1)))),
        )
        .child(
            kit::TextField::new(("member-count", index), count.to_string())
                .name(format!("Count of {}", member.name))
                .width(px(40.0))
                .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                    if let Ok(number) = words.trim().parse::<u64>() {
                        root.app.encounters.member_act(MemberAct::Count(index, number));
                    }
                    cx.notify();
                })),
        )
        .child(
            kit::IconButton::new(("member-more", index), icons::ADD, format!("More {}", member.name))
                .on_click(act(cx, MemberAct::Count(index, count + 1))),
        );
    let items = vec![
        kit::MenuItem::new("Move up")
            .icon(icons::MOVE_UP)
            .blocked(member.first.then_some("Already first."))
            .on_click(act(cx, MemberAct::Up(index))),
        kit::MenuItem::new("Move down")
            .icon(icons::MOVE_DOWN)
            .blocked(member.last.then_some("Already last."))
            .on_click(act(cx, MemberAct::Down(index))),
        kit::MenuItem::new("Remove")
            .icon(icons::REMOVE)
            .on_click(act(cx, MemberAct::Remove(index))),
    ];
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .py(XS)
        .child(kit::text::one_line(
            Type::Strong,
            c.text,
            format!("{}. {}", index + 1, member.name),
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
                .child(row_menu(("member-menu", index), &member.name, items)),
        )
        .child(kit::rule())
}

trait SmallLine {
    fn ty_small(self) -> Self;
}

impl SmallLine for Div {
    fn ty_small(self) -> Self {
        self.flex().flex_row().flex_wrap().items_baseline().gap(S)
    }
}
