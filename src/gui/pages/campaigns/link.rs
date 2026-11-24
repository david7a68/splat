//! Campaign → Link and folder (egui: the Link branch of `Splat::page`, `Sessions::folder_block`,
//! `Campaigns::show_active`, `DndBeyond::show`): the page's lock said once, then one block per
//! value (the Session folder, the campaign's folder, its D&D Beyond link), the details form
//! while it is open, and the tab's footer.
use crate::gui::kit::{self, Button, Link, Pane, Section, TextField};
use crate::gui::pages::settings::{self, campaign_list};
use crate::gui::theme::{Density, FORM_WIDTH, L, LIST_MEASURE, S, Type, XS, colors};
use crate::gui::{SplatRoot, bridge, icons};
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, prelude::*};

pub(super) fn render(root: &mut SplatRoot, _: &mut Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let blocked = root.app.blocked_switch();
    let lock = crate::campaigns::Campaigns::link_lock(blocked);
    let Some(campaign) = root.app.campaigns.active_campaign() else {
        return kit::text(
            Type::Body,
            c.muted,
            "Select a campaign, or add one in Settings.",
        )
        .into_any_element();
    };
    let lines = settings::campaign_lines(root, false, false, Some(campaign.id), blocked);
    let mut column = div().flex().flex_col().w_full().max_w(LIST_MEASURE).pb(L);
    column = column.children(campaign_list::blocked_reason(lock));
    column = column.child(session_folder(root, lock, cx));
    column = column.child(kit::rule());
    // The campaign's folder: its block's heading row with Edit details.
    let edited = campaign.clone();
    column = column
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w_full()
                .max_w(FORM_WIDTH)
                .child(kit::heading(
                    kit::Heading::Group {
                        title: "Folder",
                        count: 0,
                    },
                    false,
                ))
                .child(
                    Button::quiet("link-edit-details", "Edit details")
                        .blocked(blocked)
                        .on_click(cx.listener(move |root, _, _, cx| {
                            root.app.campaigns.start_edit(&edited, true);
                            cx.notify();
                        })),
                ),
        )
        .child(kit::text(
            Type::Body,
            c.text,
            campaign.path.display().to_string(),
        ))
        .child(kit::status(&campaign_list::folder_status(&campaign.path)))
        .child(kit::gap(S))
        .child(kit::rule());
    column = column.child(dndbeyond(root, lock, cx));
    column = column.child(kit::gap(S));
    let open = root.pages.campaigns.details_open;
    let mut details = Section::new("link-technical", "Technical details", open)
        .quiet()
        .on_toggle(cx.listener(|root, open: &bool, _, cx| {
            root.pages.campaigns.details_open = *open;
            cx.notify();
        }));
    if open {
        details = details
            .child(kit::text(Type::Body, c.text, format!("ID: {}", campaign.id)))
            .child(kit::help("Settings are stored in the campaign manifest."));
    }
    column = column.child(details);
    if root.app.campaigns.editing().is_some() {
        column = column.child(kit::gap(S));
        column = column.child(campaign_list::details_form(root, true, cx));
    }
    let mut pane = Pane::new("campaign-link").flush().child(column);
    for line in settings::footer("campaign-link-footer", lines, cx) {
        pane = pane.footer(line);
    }
    pane.into_any_element()
}

/// The Session folder's block: its field and help, its save row, and its own messages under it.
fn session_folder(root: &mut SplatRoot, lock: Option<&'static str>, cx: &mut Context<SplatRoot>) -> AnyElement {
    let crate::Splat {
        sessions, timeline, ..
    } = &mut root.app;
    let Some((state, changed)) = sessions.folder_row(timeline, lock) else {
        return div().into_any_element();
    };
    let lines = sessions.folder_lines(timeline, lock.is_some());
    let field = TextField::new("session_folder", sessions.folder_text().to_owned())
        .name("Folder")
        .disabled(lock.is_some())
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.sessions.set_folder_text(words);
            cx.notify();
        }));
    let mut block = div()
        .flex()
        .flex_col()
        .gap(XS)
        .max_w(FORM_WIDTH)
        .child(kit::heading(
            kit::Heading::Group {
                title: "Session folder",
                count: 0,
            },
            false,
        ))
        .child(field)
        .child(kit::help(
            "Inside the campaign folder, unless the path is absolute. A name such as 30 Sessions is fine.",
        ));
    if let Some(folder) = sessions.folder_now() {
        block = block
            .child(kit::help(format!("Now: {}", folder.display())))
            .child(kit::help(
                "Notes, Timeline.md, Templates and Recordings are all here.",
            ));
    }
    let primary = state.save_weight() == crate::design::save_row::SaveWeight::Primary;
    let reason = state.save_blocked().map(str::to_owned);
    let save = if primary {
        Button::primary("session-folder-save", "Save folder")
    } else {
        Button::secondary("session-folder-save", "Save folder")
    }
    .key("Ctrl+S")
    .blocked(reason.clone())
    .on_click(cx.listener(|root, _, _, cx| {
        let crate::Splat {
            sessions, timeline, ..
        } = &mut root.app;
        sessions.folder_act(timeline, crate::sessions::page::FolderAct::Save);
        cx.notify();
    }));
    let row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .py(XS)
        .child(kit::status(&bridge::status(&state.status())))
        .when(changed && lock.is_none(), |row| {
            row.child(Button::quiet("session-folder-discard", "Discard").on_click(
                cx.listener(|root, _, _, cx| {
                    root.app.sessions.discard_folder();
                    cx.notify();
                }),
            ))
        })
        .child(div().flex_1())
        .child(save);
    block = block.child(row);
    if lock.is_none() && changed {
        block = block.children(campaign_list::blocked_reason(reason.as_deref()));
    }
    let messages = kit::footer::lines(
        "session-folder-messages",
        lines,
        settings::on_press(cx, |root, press, cx| {
            let crate::Splat {
                sessions, timeline, ..
            } = &mut root.app;
            sessions.folder_act(timeline, press.id);
            cx.notify();
        }),
    );
    block
        .children(messages)
        .child(kit::gap(S))
        .into_any_element()
}

/// The D&D Beyond block: its heading row with the link's state, the way to connect, Refresh
/// list, the picker, the linked campaign's facts and its page.
fn dndbeyond(root: &mut SplatRoot, lock: Option<&'static str>, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let (_, session) = root.app.settings.campaign_session();
    {
        let SplatRoot { app, bridge, .. } = root;
        app.dndbeyond
            .link_in_view(session.clone(), &app.campaigns, bridge.ctx());
    }
    let ddb = &root.app.dndbeyond;
    let note: AnyElement = if ddb.refreshing() {
        kit::status(&kit::Status::new(icons::BUSY, "Refreshing…", c.connection.busy))
            .into_any_element()
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
    let mut block = div()
        .flex()
        .flex_col()
        .items_start()
        .gap(XS)
        .max_w(FORM_WIDTH)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(kit::heading(
                    kit::Heading::Group {
                        title: "D&D Beyond",
                        count: 0,
                    },
                    false,
                ))
                .child(note),
        );
    if session.is_none() {
        block = block.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(XS)
                .child(kit::text(Type::Small, c.muted, "Connect your account in"))
                .child(Link::new("link-open-settings", "Settings").small().on_click(
                    cx.listener(|root, _, _, cx| {
                        root.app.settings_tab = crate::SettingsTab::default();
                        root.app.settings.focus_sign_in();
                        root.show_page(crate::Page::Settings, cx);
                    }),
                ))
                .child(kit::text(Type::Small, c.muted, "to update this link.")),
        );
    }
    let refresh_lock = ddb.link_refresh_lock(lock, session.is_some());
    let refresh_session = session.clone();
    block = block.child(
        Button::secondary("ddb-refresh-list", "Refresh list")
            .density(Density::Read)
            .blocked(refresh_lock)
            .on_click(cx.listener(move |root, _, _, cx| {
                if let Some(session) = refresh_session.clone() {
                    let SplatRoot { app, bridge, .. } = root;
                    app.dndbeyond
                        .refresh_link(session, &app.campaigns, bridge.ctx());
                }
                cx.notify();
            })),
    );
    block = block.children(campaign_list::link_dropdown(root, lock, cx));
    let campaign = root.app.campaigns.active_campaign();
    if let Some(id) = campaign.as_ref().and_then(|c| c.dndbeyond_campaign_id.clone()) {
        let mut facts = div().flex().flex_row().flex_wrap().items_center().gap(S);
        if let Some(remote) = root.app.dndbeyond.remotes().find(|remote| remote.id == id) {
            if let Some(dm) = remote.dm {
                facts = facts.child(kit::text(Type::Body, c.text, format!("DM: {dm}")));
            }
            if let Some(count) = remote.players {
                facts = facts.child(kit::text(Type::Body, c.text, format!("Players: {count}")));
            }
        }
        let url = format!("https://www.dndbeyond.com/campaigns/{id}");
        facts = facts.child(
            Link::new("ddb-open-campaign", "Open D&D Beyond campaign")
                .on_click(move |_, _, cx| cx.open_url(&url)),
        );
        block = block.child(facts);
    }
    if root.app.dndbeyond.loaded() && root.app.dndbeyond.remotes().next().is_none() {
        block = block.child(kit::text(
            Type::Body,
            c.text,
            "No campaigns returned for this account.",
        ));
    }
    block.into_any_element()
}
