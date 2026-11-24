//! The Library's header (`show`'s title line): the title and its count, `New entry`, the
//! `Packages` menu and the issues count with its menu; then the line a page with no campaign
//! shows under it.
use crate::gui::theme::{Density, L, LINE_GAP, READER_MEASURE, S, Type, XL, XS, colors};
use crate::gui::{SplatRoot, bridge, icons, kit};
use crate::lookup::ContentIndex;
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, Styled, Window, div, prelude::*, px,
};
use std::sync::Arc;

pub(super) fn render(
    root: &mut SplatRoot,
    index: &Option<Arc<ContentIndex>>,
    width: f32,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let header = root.app.library.header(
        &root.app.campaigns,
        index.as_ref(),
        width >= f32::from(READER_MEASURE),
    );
    let mut actions = Vec::new();
    if let Some(blocked) = header.new_entry {
        actions.push(
            kit::Button::secondary("library-new", "New entry")
                .icon(icons::ADD)
                .blocked(blocked)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.library.open_new_entry();
                    cx.notify();
                }))
                .into_any_element(),
        );
    }
    let packages = kit::MenuButton::new(
        "library-packages",
        kit::Button::quiet("library-packages-button", "Packages")
            .icon_after(icons::DOWN)
            .blocked(header.packages_blocked.clone()),
    )
    .item(
        kit::MenuItem::new("Reload from disk")
            .icon(icons::REFRESH)
            .on_click(cx.listener(|root, _, _, cx| {
                let ctx = root.bridge.ctx().clone();
                root.app.library.reload(&ctx, &root.app.campaigns);
                cx.notify();
            })),
    )
    .item(
        kit::MenuItem::new("Manage packages…")
            .icon(icons::SETTINGS)
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.library.toggle_package_form();
                cx.notify();
            })),
    );
    actions.push(packages.into_any_element());
    if let Some(issues) = header.issues {
        let (icon, color) = if issues.unreadable {
            (icons::ERROR, c.result.error)
        } else {
            (icons::WARN, c.result.warning)
        };
        let mut content = div()
            .id("library-issues-list")
            .flex()
            .flex_col()
            .gap(XS)
            .max_w(READER_MEASURE)
            .max_h(XL * 6.0 + px(32.0))
            .overflow_y_scroll();
        for line in &issues.lines {
            content = content.child(kit::text(Type::Small, c.muted, line.clone()));
        }
        if !issues.links.is_empty()
            && let Some(lock) = &issues.open_lock
        {
            content = content.child(kit::text(Type::Small, c.muted, lock.clone()));
        }
        for (at, words) in &issues.links {
            let at = *at;
            content = if issues.open_lock.is_some() {
                content.child(kit::text(Type::Small, c.muted, words.clone()))
            } else {
                content.child(
                    kit::link(
                        ElementId::Name(format!("library-issue-{at}").into()),
                        words.clone(),
                    )
                    .small()
                    .on_click(cx.listener(move |root, _, window, cx| {
                        root.app.library.open_broken(at);
                        kit::menu::close_open_popup(window, cx);
                        cx.notify();
                    })),
                )
            };
        }
        actions.push(
            kit::Popover::on_status(
                "library-issues",
                kit::status_link(
                    "library-issues-link",
                    kit::Status::new(icon, issues.word.clone(), color),
                ),
                content,
            )
            .into_any_element(),
        );
    }
    // The shared title line (M0327-M0329): the title and its count, then the actions after
    // them, as egui's `TitleLine::page` places them.
    let mut title = div()
        .flex()
        .flex_row()
        .items_baseline()
        .gap(S + XS)
        .flex_none()
        .child(kit::text(Type::Title, c.text, "Library").whitespace_nowrap());
    if let Some(context) = header.context {
        title = title.child(kit::text(Type::Small, c.muted, context).whitespace_nowrap());
    }
    let mut column = div()
        .flex()
        .flex_col()
        .w_full()
        .flex_none()
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .min_h(Density::Scan.control())
                .gap(S)
                .child(title)
                .children(actions),
        )
        .child(kit::rule().mt(LINE_GAP))
        .child(kit::gap(L));
    if let Some((words, action)) = header.no_campaign {
        column = column.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .child(kit::text(Type::Small, c.muted, words))
                .child(
                    kit::Button::primary("library-no-campaign", action)
                        .density(Density::Read)
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.library.ask_for_settings();
                            cx.notify();
                        })),
                ),
        );
    }
    let _ = bridge::color;
    column.child(kit::gap(XS)).into_any_element()
}
