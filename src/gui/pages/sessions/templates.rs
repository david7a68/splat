//! The templates view (`Sessions::templates_view`): the list of templates beside the shown one
//! (a built-in template read-only with `Customize`; a campaign template in an editor that saves
//! after a pause, with its menu's `Delete`), and the New template form.
use super::footer;
use crate::gui::{SplatRoot, bridge, icons, kit, theme};
use gpui::{
    AnyElement, App, Context, ElementId, InteractiveElement, IntoElement, KeyBinding,
    ParentElement, SharedString, Styled, Window, actions, div, prelude::*, px,
};
use std::rc::Rc;
use theme::{Density, L, S, Type, colors};

actions!(session_template, [SaveTemplate]);

pub(super) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("ctrl-s", SaveTemplate, Some("SessionTemplate"))]);
}

fn id(words: impl std::fmt::Display) -> ElementId {
    ElementId::Name(SharedString::from(words.to_string()))
}

pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let width = super::page_width(root, window);
    let narrow = width < super::TWO_COLUMNS_FROM;
    let sessions = &mut root.app.sessions;
    let template = sessions.gv_template();
    if narrow && sessions.gv_templates_list_only() {
        return list(root, cx);
    }
    let editor = editor(root, template, cx);
    if narrow {
        let back = kit::Button::quiet("templates-all", "All templates")
            .icon(icons::HISTORY_BACK)
            .line_start()
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.sessions.gv_set_templates_list_only(true);
                cx.notify();
            }));
        return div()
            .flex()
            .flex_col()
            .size_full()
            .child(back)
            .child(div().flex_1().min_h_0().child(editor))
            .into_any_element();
    }
    let list = list(root, cx);
    kit::Columns::new("template-columns")
        .column(super::LIST_WIDTH, super::LIST_MIN..super::LIST_MAX, div().size_full().pr(S).child(list))
        .rest(div().size_full().pl(S).child(editor))
        .into_any_element()
}

/// `Sessions` back to the notes, and the templates' rows.
fn list(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let rows = root.app.sessions.gv_template_rows();
    let rows: Vec<_> = rows
        .into_iter()
        .map(|row| {
            let name = row.name.clone();
            kit::two_line_row(
                id(format!("template-row-{}", row.name)),
                row.name,
                row.kind,
                row.selected,
            )
            .on_click(cx.listener(move |root, _, _, cx| {
                root.app.sessions.gv_choose_template(&name);
                cx.notify();
            }))
        })
        .collect();
    let lines = root.app.sessions.gv_template_list_footer();
    let footer = footer::lines(
        "template-list-footer",
        lines,
        Rc::new(|root, press, _, _| root.app.sessions.gv_draft_press(press, true)),
        cx,
    );
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(
            kit::Button::quiet("templates-back", "Sessions")
                .icon(icons::HISTORY_BACK)
                .line_start()
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.gv_show_notes();
                    cx.notify();
                })),
        )
        .child(kit::gap(S))
        .child(
            div()
                .flex_1()
                .min_h_0()
                .child(kit::scroll_column("template-list", div().flex().flex_col().children(rows))),
        )
        .children(footer)
        .into_any_element()
}

/// The shown template, or the New template form; the template's results in its footer.
fn editor(
    root: &mut SplatRoot,
    template: Option<crate::sessions::page::gpui_facade::TemplateView>,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let new_form = root.app.sessions.gv_new_template_name().map(str::to_owned);
    let body: AnyElement = if let Some(name) = new_form {
        let focus = std::mem::take(&mut root.pages.sessions.focus_template_name);
        new_template(name, root.app.sessions.gv_open_run(), focus, cx)
    } else if let Some(template) = template {
        let mut line = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(L)
            .min_h(Density::Scan.control())
            .child(kit::text(Type::Section, c.text, template.name.clone()).flex_1());
        let words: AnyElement = if let Some((path, words, state, word)) = template.file {
            let status = crate::sessions::page::Sessions::gv_template_status(&state);
            line = line
                .child(kit::status(&bridge::status(&status)))
                .child(
                    kit::MenuButton::new(
                        "template-more",
                        kit::Button::quiet("template-more-button", "More")
                            .name(format!("More actions for {}", template.name)),
                    )
                    .item(
                        kit::MenuItem::new(word).on_click(cx.listener(|root, _, _, cx| {
                            root.app.sessions.gv_delete_template();
                            cx.notify();
                        })),
                    ),
                );
            div()
                .key_context("SessionTemplate")
                .on_action(cx.listener(|root, _: &SaveTemplate, _, cx| {
                    root.app.sessions.gv_save_template();
                    cx.notify();
                }))
                .size_full()
                .max_w(super::note::frame_width())
                .child(
                    kit::TextEditor::new(id(format!("template-text-{}", path.display())), words)
                        .name("Template text")
                        .text_type(Type::Prose)
                        .fill()
                        .on_change(cx.listener(|root, words: &str, _, cx| {
                            root.app.sessions.gv_template_edited(words);
                            cx.notify();
                        })),
                )
                .into_any_element()
        } else {
            let read_only = kit::Status::new(icons::READ_ONLY, "Read-only", c.document.read_only);
            line = line.child(kit::status(&read_only)).child(
                kit::Button::secondary("template-customize", "Customize")
                    .name("Customize this template")
                    .on_click(cx.listener(|root, _, _, cx| {
                        root.app.sessions.gv_customize_template();
                        cx.notify();
                    })),
            );
            kit::scroll_column(
                "template-built-in",
                div().max_w(theme::READER_MEASURE).child(kit::text(
                    Type::Prose,
                    c.text,
                    template.built_in.unwrap_or_default(),
                )),
            )
            .into_any_element()
        };
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap(S)
            .child(kit::help(
                "Splat fills {{number}}, {{title}}, {{date}} and {{campaign}}. Obsidian’s core Templates plugin also fills {{date}} and {{title}}.",
            ))
            .child(line)
            .child(div().flex_1().min_h_0().child(words))
            .into_any_element()
    } else {
        div().into_any_element()
    };
    let lines = root.app.sessions.gv_draft_footer(true);
    let footer = footer::lines(
        "template-footer",
        lines,
        Rc::new(|root, press, _, _| root.app.sessions.gv_draft_press(press, true)),
        cx,
    );
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(div().flex_1().min_h_0().child(body))
        .when(!footer.is_empty(), |pane| {
            pane.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .pt(S)
                    .border_t(theme::HAIRLINE_WIDTH)
                    .border_color(c.hairline)
                    .children(footer),
            )
        })
        .into_any_element()
}

/// `New template`: its name, Create template and Cancel.
fn new_template(
    name: String,
    run_open: bool,
    focus: bool,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let entity = cx.entity().downgrade();
    let blocked = name.trim().is_empty().then_some("Type a name first.");
    let field = kit::TextField::new("template-name", name)
        .name("Template name")
        .focus(focus)
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.sessions.gv_set_new_template_name(words);
            cx.notify();
        }))
        .on_enter(move |_, cx| {
            let _ = entity.update(cx, |root, cx| {
                root.app.sessions.gv_create_template();
                cx.notify();
            });
        });
    let create = if run_open {
        kit::Button::secondary("template-create", "Create template")
    } else {
        kit::Button::primary("template-create", "Create template")
    }
    .blocked(blocked)
    .on_click(cx.listener(|root, _, _, cx| {
        root.app.sessions.gv_create_template();
        cx.notify();
    }));
    div()
        .flex()
        .flex_col()
        .pl(L - S)
        .w(theme::FORM_WIDTH)
        .max_w_full()
        .child(kit::label("New template", 0))
        .child(kit::field(
            "Template name",
            Some("Starts with the title line only.".into()),
            None,
            field,
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(create)
                .child(
                    kit::Button::quiet("template-cancel", "Cancel").on_click(cx.listener(
                        |root, _, _, cx| {
                            root.app.sessions.gv_cancel_new_template();
                            cx.notify();
                        },
                    )),
                ),
        )
        .children(blocked.map(|reason| kit::text(Type::Small, colors().muted, reason).pt(px(4.0))))
        .into_any_element()
}
