//! The D&D Beyond link of the open campaign (`dndbeyond::DndBeyond::show`): the block's heading
//! with the link's state, the picker of the account's campaigns, Refresh list, and the linked
//! campaign's details. The Campaign page's Link and folder tab draws it
//! (`library::render_dndbeyond_link`); its errors and its `Campaign link saved.` result go in
//! that pane's footer, from `DndBeyond::footer_messages` (typed lines, `design::outcome::plain`).
use crate::gui::theme::{Density, FORM_WIDTH, S, Type, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div};

pub(crate) fn render_link(
    root: &mut SplatRoot,
    blocked: Option<&str>,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let (_, session) = root.app.settings.campaign_session();
    let (refreshing, none_returned) = root.app.dndbeyond.list_state();
    let note = if refreshing {
        kit::text(Type::Small, c.muted, "Refreshing…").into_any_element()
    } else if session.is_none() {
        kit::text(
            Type::Small,
            c.muted,
            "Not connected · showing the name saved on this computer",
        )
        .into_any_element()
    } else {
        kit::status(&kit::Status::new(
            icons::ONLINE,
            "Connected",
            c.connection.connected,
        ))
        .into_any_element()
    };
    let heading = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(kit::label("D&D Beyond", 0))
        .child(note);
    let mut block = div().flex().flex_col().gap(XS).child(heading);
    if root.app.campaigns.active_campaign().is_none() {
        return block
            .child(kit::text(
                Type::Body,
                c.text,
                "Select a local campaign to configure its D&D Beyond link.",
            ))
            .into_any_element();
    }
    // The picker is in view: a link not yet checked against the account is checked once.
    {
        let ctx = root.bridge.ctx().clone();
        let app = &mut root.app;
        app.dndbeyond.link_seen(session.clone(), &app.campaigns, &ctx);
    }
    if session.is_none() {
        block = block.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .child(kit::text(Type::Small, c.muted, "Connect your account in "))
                .child(kit::link("ddb-settings", "Settings").small().on_click(
                    cx.listener(|root, _, _, cx| {
                        root.bridge.ctx().data_mut(|data| {
                            data.insert_temp(
                                eframe::egui::Id::new("open_account_settings"),
                                true,
                            )
                        });
                        cx.notify();
                    }),
                ))
                .child(kit::text(Type::Small, c.muted, " to update this link.")),
        );
    }
    let refresh_blocked = root
        .app
        .dndbeyond
        .refresh_blocked(session.as_ref(), blocked)
        .map(str::to_owned);
    block = block.child(
        kit::Button::secondary("ddb-refresh", "Refresh list")
            .density(Density::Read)
            .blocked(refresh_blocked)
            .on_click(cx.listener(|root, _, _, cx| {
                let ctx = root.bridge.ctx().clone();
                let (_, session) = root.app.settings.campaign_session();
                let app = &mut root.app;
                app.dndbeyond.refresh_link(session, &app.campaigns, &ctx);
                cx.notify();
            })),
    );
    let (choices, chosen) = root.app.dndbeyond.link_choices(&root.app.campaigns);
    let ids: Vec<_> = choices
        .iter()
        .map(|(id, _, locked)| (id.clone(), locked.is_some()))
        .collect();
    block = block.child(
        kit::Dropdown::new(
            "ddb-link",
            "D&D Beyond campaign link",
            choices.iter().map(|(_, name, locked)| match locked {
                Some(reason) => format!("{name} · {reason}"),
                None => name.clone(),
            }),
            Some(chosen),
        )
        .width(FORM_WIDTH)
        .blocked(blocked.map(str::to_owned))
        .on_select(cx.listener(move |root, at: &usize, _, cx| {
            // A campaign linked to another local campaign cannot be chosen here.
            if let Some((id, false)) = ids.get(*at).cloned() {
                let app = &mut root.app;
                app.dndbeyond.choose_link(id, &mut app.campaigns);
                cx.notify();
            }
        })),
    );
    if let Some((words, url)) = root.app.dndbeyond.link_details(&root.app.campaigns) {
        block = block.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .children(words.into_iter().map(|words| kit::text(Type::Body, c.text, words)))
                .child(
                    kit::link("ddb-open", "Open D&D Beyond campaign")
                        .on_click(move |_, _, cx| cx.open_url(&url)),
                ),
        );
    }
    if none_returned {
        block = block.child(kit::text(
            Type::Body,
            c.text,
            "No campaigns returned for this account.",
        ));
    }
    block.into_any_element()
}
