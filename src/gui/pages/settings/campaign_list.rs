//! The campaign list, the campaign forms and the remove dialog (egui: `Campaigns::show`, `form`,
//! `remove_dialog`, `form_error_inline`, and `DndBeyond::show_import`). Campaign → Link and folder
//! draws the details form from here too.
use crate::auth::Session;
use crate::gui::kit::{
    self, Button, Dropdown, MenuButton, MenuItem, Message, Section, Status, TextField, Tone,
    WithTooltip as _,
};
use crate::gui::theme::{Density, FORM_WIDTH, L, S, Type, XS, colors};
use crate::gui::{SplatRoot, icons};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*,
};
use uuid::Uuid;

fn id(prefix: &str, uuid: Uuid) -> ElementId {
    ElementId::Name(SharedString::from(format!("{prefix}-{uuid}")))
}

/// A blocked reason as a line of its own: `Small` muted words.
pub(crate) fn blocked_reason(reason: Option<&str>) -> Option<AnyElement> {
    reason.map(|words| kit::text(Type::Small, colors().muted, words.to_owned()).into_any_element())
}

/// The folder's state, said only when it matters (a missing folder), or on Link and folder.
pub(crate) fn folder_status(path: &std::path::Path) -> Status {
    let c = colors();
    if path.is_dir() {
        Status::new(icons::OK, "Folder available", c.result.success)
    } else {
        Status::new(icons::WARN, "Folder unavailable", c.result.warning)
    }
}

/// The list: its heading and Reload, each campaign's row (its details form under it while
/// open), the import from D&D Beyond, and the add form.
pub(super) fn list(
    root: &mut SplatRoot,
    session: Option<Session>,
    blocked: Option<&'static str>,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let campaigns = &root.app.campaigns;
    let all = campaigns.all().to_vec();
    let unwritable = campaigns.list_lock(blocked);
    let reload = (!all.is_empty() || !campaigns.writable()).then(|| {
        Button::quiet("campaigns-reload", "Reload from disk")
            .icon(icons::REFRESH)
            .density(Density::Read)
            .hint("Reload campaign settings from disk")
            .blocked(blocked)
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.campaigns.reload_from_disk();
                cx.notify();
            }))
    });
    let mut column = div().flex().flex_col().w_full().child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(kit::heading(
                kit::Heading::Group {
                    title: "Campaigns",
                    count: all.len(),
                },
                true,
            ))
            .children(reload),
    );
    column = column.children(blocked_reason(blocked));
    if all.is_empty() {
        column = column.child(kit::gap(XS)).child(kit::text(
            Type::Body,
            c.muted,
            "No campaign yet. Add one below.",
        ));
    }
    let active = campaigns.active_id();
    let editing = campaigns.editing();
    for campaign in &all {
        let is_active = active == Some(campaign.id);
        let mut block = div().flex().flex_col().flex_1().min_w_0().child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .child(kit::text(Type::Name, c.text, campaign.name.clone()))
                .when(is_active, |line| {
                    line.child(kit::status(&Status::new(icons::OK, "Active", c.text)))
                }),
        );
        if !campaign.path.is_dir() {
            block = block.child(
                div()
                    .id(id("campaign-folder", campaign.id))
                    .child(kit::status(&folder_status(&campaign.path)))
                    .with_tooltip(campaign.path.display().to_string()),
            );
        }
        if let Some(error) = root.app.campaigns.manifest_error(campaign.id) {
            block = block.child(kit::text(Type::Small, c.result.error, error.to_owned()));
        }
        let link = campaign
            .dndbeyond_campaign_name
            .as_deref()
            .map(|name| format!("Linked to {name}"))
            .or(campaign
                .dndbeyond_campaign_id
                .as_deref()
                .map(|id| format!("Linked to campaign {id}")))
            .unwrap_or_else(|| "Not linked".to_owned());
        block = block.child(kit::status(&Status::new(
            if campaign.dndbeyond_campaign_id.is_some() {
                icons::LINK
            } else {
                icons::SYNC_OFF
            },
            link,
            c.muted,
        )));
        let campaign_id = campaign.id;
        let mut actions = div().flex().flex_row().items_center().gap(XS).flex_none();
        if !is_active {
            actions = actions.child(
                Button::secondary(id("campaign-open", campaign_id), "Open")
                    .density(Density::Read)
                    .name(format!("Make {} the active campaign", campaign.name))
                    .blocked(unwritable)
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.campaigns.open_campaign(campaign_id);
                        cx.notify();
                    })),
            );
        }
        if editing != Some(campaign_id) {
            let edited = campaign.clone();
            actions = actions.child(
                Button::quiet(id("campaign-edit", campaign_id), "Edit details")
                    .density(Density::Read)
                    .blocked(unwritable)
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.campaigns.start_edit(&edited, false);
                        cx.notify();
                    })),
            );
        }
        let more = Button::quiet(id("campaign-more", campaign_id), "")
            .icon(icons::EXPAND)
            .name(format!("More actions for {}", campaign.name))
            .blocked(unwritable);
        actions = actions.child(
            MenuButton::new(id("campaign-menu", campaign_id), more).item(
                MenuItem::new("Remove…").destructive().on_click(cx.listener(
                    move |root, _, _, cx| {
                        root.app.campaigns.ask_remove(campaign_id);
                        cx.notify();
                    },
                )),
            ),
        );
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(L)
                .min_h(Density::Glance.row() + XS)
                .py(XS)
                .child(block)
                .child(actions),
        );
        if editing == Some(campaign_id) {
            column = column.child(details_form(root, true, cx));
        }
        column = column.child(kit::rule());
    }
    let campaigns = &root.app.campaigns;
    let forms_closed = campaigns.editing().is_none() && !campaigns.import_open();
    if session.is_some() || forms_closed {
        column = column.child(kit::gap(S));
    }
    if let Some(session) = session.clone() {
        column = column.child(import(root, session, unwritable, window, cx));
    }
    let campaigns = &root.app.campaigns;
    if campaigns.editing().is_none() && !campaigns.import_open() {
        column = column.child(add_section(root, unwritable, window, cx));
        if session.is_none()
            && root.app.campaigns.form_open()
            && window.viewport_size().width < gpui::px(1000.0)
        {
            column = column.child(import_hint(cx));
        }
    }
    column.into_any_element()
}

/// `To import from D&D Beyond, connect your account first.`, under the open add form in the
/// narrow tab (`DndBeyond::import_hint`).
fn import_hint(cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(XS)
        .child(kit::text(Type::Body, c.text, "To import from D&D Beyond,"))
        .child(
            kit::Link::new("campaigns-connect-hint", "connect your account").on_click(cx.listener(
                |root, _, _, cx| {
                    root.pages.settings.focus_cookie = true;
                    root.pages.settings.compact_open = true;
                    cx.notify();
                },
            )),
        )
        .child(kit::text(Type::Body, c.text, "first."))
        .into_any_element()
}

/// `Add campaign`: a caret section while there are campaigns, else the form itself.
fn add_section(
    root: &mut SplatRoot,
    unwritable: Option<&'static str>,
    _: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    if root.app.campaigns.all().is_empty() {
        root.app.campaigns.set_form_open(true);
        return add_form(root, true, cx);
    }
    // The name field's request for the keyboard opens the section.
    if root.app.campaigns.take_focus_name() {
        root.pages.settings.add_open = true;
        root.pages.settings.focus_name = true;
    }
    let open = root.pages.settings.add_open;
    root.app.campaigns.set_form_open(open);
    let mut section = Section::new("campaign-add-section", "Add campaign", open)
        .blocked(unwritable)
        .on_toggle(cx.listener(|root, open: &bool, _, cx| {
            root.pages.settings.add_open = *open;
            if *open {
                root.pages.settings.focus_name = true;
            }
            cx.notify();
        }));
    if open {
        section = section.child(add_form(root, true, cx));
    }
    section.into_any_element()
}

/// The add form (and the import form, for the remote campaign it adds): the name, the folder
/// with Browse… and Add campaign on its line, the folder's help, then a kept line with Cancel,
/// the folder Splat will create and the reason Add waits.
pub(crate) fn add_form(
    root: &mut SplatRoot,
    primary: bool,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let focus =
        root.app.campaigns.take_focus_name() | std::mem::take(&mut root.pages.settings.focus_name);
    let campaigns = &root.app.campaigns;
    let (name, path) = campaigns.form_words();
    let reason = campaigns.form_reason_words();
    let cancel = reason.is_none() || campaigns.import_open();
    let preview = campaigns.form_preview();
    let name_field = TextField::new("campaign_name-add", name.to_owned())
        .name("Campaign name")
        .focus(focus)
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.campaigns.set_form_name(words);
            cx.notify();
        }))
        .on_enter(super::act(cx, |root, _, cx| {
            commit(root);
            cx.notify();
        }));
    let folder_field = TextField::new("campaign_folder-add", path.to_owned())
        .name("Folder")
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.campaigns.set_form_path(words);
            cx.notify();
        }));
    let create = if primary {
        Button::primary("campaign-add", "Add campaign")
    } else {
        Button::secondary("campaign-add", "Add campaign")
    }
    .blocked(reason)
    .on_click(cx.listener(|root, _, _, cx| {
        commit(root);
        cx.notify();
    }));
    let browse = Button::secondary("campaign-browse-add", "Browse…")
        .density(Density::Read)
        .on_click(cx.listener(|root, _, _, cx| {
            root.app.campaigns.browse_folder();
            cx.notify();
        }));
    let mut kept = div().flex().flex_row().flex_wrap().items_center().gap(S);
    if cancel {
        kept = kept.child(
            Button::quiet("campaign-add-cancel", "Cancel")
                .line_start()
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.campaigns.cancel_form();
                    root.pages.settings.add_open = false;
                    cx.notify();
                })),
        );
    }
    if let Some(preview) = preview {
        kept = kept.child(kit::text(
            Type::Small,
            c.muted,
            format!("Will create {preview}"),
        ));
    }
    if let Some(reason) = reason {
        kept = kept.child(kit::text(Type::Small, c.muted, reason));
    }
    div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(FORM_WIDTH)
        .pt(XS)
        .child(kit::field("Campaign name", None, None, name_field).pb(S))
        .child(
            kit::field(
                "Folder",
                None,
                None,
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(S)
                    .child(div().flex_1().min_w_0().child(folder_field))
                    .child(browse)
                    .child(create),
            )
            .pb(XS),
        )
        .child(kit::help(crate::campaigns::FOLDER_HELP))
        .child(kit::gap(XS))
        .child(kept)
        .into_any_element()
}

/// Add campaign or Save details: a new campaign's form closes.
fn commit(root: &mut SplatRoot) {
    if root.app.campaigns.form_reason_words().is_some() {
        return;
    }
    if root.app.campaigns.commit_form() {
        root.pages.settings.add_open = false;
    }
}

/// The details form of the campaign being edited: its name, its folder with Browse…, the
/// folder's help, then a save row (its state and Discard or Cancel; Save details at the right).
pub(crate) fn details_form(
    root: &mut SplatRoot,
    primary: bool,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let focus = root.app.campaigns.take_focus_name();
    let campaigns = &root.app.campaigns;
    let editing = campaigns.editing().unwrap_or_default();
    let (name, path) = campaigns.form_words();
    let unchanged = campaigns.form_unchanged();
    let reason = campaigns.form_reason_words();
    let name_field = TextField::new(id("campaign_name", editing), name.to_owned())
        .name("Campaign name")
        .focus(focus)
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.campaigns.set_form_name(words);
            root.app.campaigns.details_edited();
            cx.notify();
        }));
    let folder_field = TextField::new(id("campaign_folder", editing), path.to_owned())
        .name("Folder")
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.campaigns.set_form_path(words);
            root.app.campaigns.details_edited();
            cx.notify();
        }));
    let state = if unchanged {
        Status::new(icons::OK, "No changes", c.muted)
    } else {
        Status::new(icons::DRAFT, "Unsaved", c.document.unsaved)
    };
    let undo = if unchanged {
        Button::quiet("campaign-details-cancel", "Cancel")
            .line_start()
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.campaigns.cancel_form();
                cx.notify();
            }))
    } else {
        Button::quiet("campaign-details-discard", "Discard").on_click(cx.listener(
            |root, _, _, cx| {
                root.app.campaigns.discard_details();
                cx.notify();
            },
        ))
    };
    let save = if primary {
        Button::primary("campaign-details-save", "Save details")
    } else {
        Button::secondary("campaign-details-save", "Save details")
    }
    .blocked(reason)
    .on_click(cx.listener(|root, _, _, cx| {
        commit(root);
        cx.notify();
    }));
    div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(FORM_WIDTH)
        .pt(XS)
        .child(kit::field("Campaign name", None, None, name_field).pb(S))
        .child(
            kit::field(
                "Folder",
                None,
                None,
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(S)
                    .child(div().flex_1().min_w_0().child(folder_field))
                    .child(
                        Button::secondary(id("campaign-browse", editing), "Browse…")
                            .density(Density::Read)
                            .on_click(cx.listener(|root, _, _, cx| {
                                root.app.campaigns.browse_folder();
                                root.app.campaigns.details_edited();
                                cx.notify();
                            })),
                    ),
            )
            .pb(XS),
        )
        .child(kit::help(crate::campaigns::FOLDER_HELP))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .py(XS)
                .child(kit::status(&state))
                .child(undo)
                .child(div().flex_1())
                .children(
                    reason
                        .filter(|_| !unchanged)
                        .map(|words| kit::text(Type::Small, c.muted, words)),
                )
                .child(save),
        )
        .into_any_element()
}

/// The first campaign's form error, where its page has no footer (`form_error_inline`): a
/// fault with `Register as a new campaign` when the folder is a copy.
pub(crate) fn form_error_inline(
    root: &SplatRoot,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let error = root.app.campaigns.error()?.to_owned();
    let mut message = Message::new(Tone::Error, error.clone());
    if error.contains("registered at another folder") {
        let (name, _) = root.app.campaigns.form_words();
        message = message.action(
            Button::secondary("campaign-register-new", "Register as a new campaign")
                .blocked(name.trim().is_empty().then_some("Type a name first."))
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.campaigns.register_copy_now();
                    cx.notify();
                })),
        );
    }
    Some(message.into_any_element())
}

/// The remove dialog, while it is open (`Campaigns::remove_dialog`).
pub(crate) fn remove_dialog(root: &SplatRoot, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let campaign = root.app.campaigns.removing()?;
    let c = colors();
    Some(
        kit::Dialog::new("remove_campaign", format!("Remove {}?", campaign.name))
            .child(kit::text(
                Type::Body,
                c.text,
                "Splat forgets this campaign. Its encounters, runs and D&D Beyond link stay in its folder. To use it again, add the folder back.",
            ))
            .child(kit::text(
                Type::Small,
                c.muted,
                campaign.path.display().to_string(),
            ))
            .destructive(
                Button::destructive("remove-campaign-confirm", "Remove from Splat").on_click(
                    cx.listener(|root, _, _, cx| {
                        root.app.campaigns.remove_confirmed();
                        cx.notify();
                    }),
                ),
            )
            .action(
                Button::secondary("remove-campaign-keep", "Keep the campaign").on_click(
                    cx.listener(|root, _, _, cx| {
                        root.app.campaigns.keep_campaign();
                        cx.notify();
                    }),
                ),
            )
            .on_escape(super::act(cx, |root, _, cx| {
                root.app.campaigns.keep_campaign();
                cx.notify();
            }))
            .into_any_element(),
    )
}

/// `Add from D&D Beyond` (`DndBeyond::show_import`): the account's campaigns, each with Add… or
/// Added, and the import form under the one being added.
fn import(
    root: &mut SplatRoot,
    session: Session,
    unwritable: Option<&'static str>,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let open = root
        .pages
        .settings
        .import_open
        .unwrap_or_else(|| root.app.campaigns.all().is_empty());
    let mut section = Section::new("campaign-import-section", "Add from D&D Beyond", open)
        .blocked(unwritable)
        .on_toggle(cx.listener(|root, open: &bool, _, cx| {
            root.pages.settings.import_open = Some(*open);
            cx.notify();
        }));
    if !open {
        return section.into_any_element();
    }
    {
        let SplatRoot { app, ctx, .. } = root;
        if app.dndbeyond.wants_first_list() {
            app.dndbeyond.refresh_list(session.clone(), ctx);
        }
    }
    let refreshing = root.app.dndbeyond.refreshing();
    let refresh_session = session.clone();
    section = section.child(
        Button::secondary("ddb-refresh", "Refresh from D&D Beyond")
            .density(Density::Read)
            .blocked(refreshing.then_some("Already refreshing."))
            .on_click(cx.listener(move |root, _, _, cx| {
                let SplatRoot { app, ctx, .. } = root;
                app.dndbeyond.refresh_list(refresh_session.clone(), ctx);
                cx.notify();
            })),
    );
    if refreshing {
        section = section.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(kit::icon(icons::BUSY, Type::Small, c.connection.busy))
                .children(blocked_reason(Some("Already refreshing."))),
        );
    }
    let remotes: Vec<(String, String)> = root
        .app
        .dndbeyond
        .remotes()
        .map(|remote| (remote.id.to_owned(), remote.name.to_owned()))
        .collect();
    if root.app.dndbeyond.loaded() && remotes.is_empty() {
        section = section.child(kit::text(
            Type::Body,
            c.text,
            "No campaigns returned for this account.",
        ));
    }
    // Round 27 AN2: one action column, `L` after the widest name, so each Add or Added sits
    // near its name; a name too long for the room left ends in `…`, whole on hover.
    let column = remotes
        .iter()
        .map(|(_, name)| f32::from(kit::text::width(window, Type::Strong, name)))
        .fold(0.0, f32::max);
    for (at, (remote_id, name)) in remotes.into_iter().enumerate() {
        let added = root.app.campaigns.is_added(&remote_id);
        let importing = root.app.campaigns.importing(&remote_id);
        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(L)
            .min_h(Density::Read.control())
            .child(
                div()
                    .id(("ddb-remote", at))
                    .w(gpui::px(column))
                    .flex_shrink_1()
                    .min_w_0()
                    .child(kit::text::one_line(Type::Strong, c.text, name.clone()))
                    .with_tooltip(name.clone()),
            );
        if added {
            row = row.child(kit::status(&Status::new(icons::OK, "Added", c.muted)));
        } else if !importing {
            let (remote, remote_name) = (remote_id.clone(), name.clone());
            row = row.child(
                Button::secondary(("ddb-add", at), "Add…")
                    .name(format!("Add {name} from D&D Beyond"))
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.campaigns.begin_import(&remote, &remote_name);
                        cx.notify();
                    })),
            );
        }
        section = section.child(row);
        if importing {
            section = section.child(add_form(root, true, cx));
        }
    }
    section.into_any_element()
}

/// The D&D Beyond link's picker (`DndBeyond::show_link`), for Link and folder.
pub(crate) fn link_dropdown(
    root: &SplatRoot,
    blocked: Option<&'static str>,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let campaign = root.app.campaigns.active_campaign()?;
    let choices = root.app.dndbeyond.link_choices(&root.app.campaigns);
    let current = campaign.dndbeyond_campaign_id.clone();
    let chosen = choices.iter().position(|(id, _, _)| *id == current);
    let names: Vec<SharedString> = choices
        .iter()
        .map(|(_, name, _)| SharedString::from(name.clone()))
        .collect();
    let locks: Vec<(usize, &'static str)> = choices
        .iter()
        .enumerate()
        .filter_map(|(at, (_, _, lock))| lock.map(|lock| (at, lock)))
        .collect();
    let ids: Vec<(Option<String>, bool)> = choices
        .into_iter()
        .map(|(id, _, lock)| (id, lock.is_some()))
        .collect();
    // A campaign already linked elsewhere is a blocked choice with its reason on its row, as
    // egui's `blocked_choice` draws it.
    let mut dropdown = Dropdown::new(
        id("ddb_link", campaign.id),
        "D&D Beyond campaign link",
        names,
        chosen,
    );
    for (at, lock) in locks {
        dropdown = dropdown.blocked_choice(at, lock);
    }
    Some(
        dropdown
            .width(FORM_WIDTH)
            .blocked(blocked)
            .on_select(cx.listener(move |root, index: &usize, _, cx| {
                if let Some((id, locked)) = ids.get(*index)
                    && !locked
                {
                    let SplatRoot { app, .. } = root;
                    app.dndbeyond.choose_link(&mut app.campaigns, id.clone());
                    cx.notify();
                }
            }))
            .into_any_element(),
    )
}
