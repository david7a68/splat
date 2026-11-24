//! The rail (`Splat::rail`): the app's name, the campaign switcher, the Run slot, the pages, then
//! at the bottom edge the status slot (it grows upward), Lookup, Notes, Settings and the account
//! line. No rail target moves when a state comes or goes: the Run slot keeps one height in every
//! state, and the bottom group keeps its place whatever the statuses say
//! (`docs/design-system.md`, Space, shape and density).
use super::{SplatRoot, bridge, icons, kit, theme};
use crate::{NO_FIGHT_OPEN, Page, SettingsTab, draft_page, encounters, world};
use gpui::{
    Context, Div, IntoElement, ParentElement, SharedString, Styled, Window, div, prelude::*, px,
};
use theme::{RAIL_ITEM_GAP, S, Type, XS, colors};

/// The campaign switcher's slot: two `Small` lines of a long name and its frame.
const SELECTOR_HEIGHT: f32 = 44.0;

/// The rail's column (`Splat::rail`, `Column::left("navigation", …)`): wider in a wide window,
/// except at the table (the run page); the DM can drag it wider, up to `RAIL_DRAG_MAX_WIDTH`,
/// but not at the table, where the roster needs the room.
pub fn column(root: &SplatRoot, window: &Window) -> kit::EdgeColumn {
    let at_table = root.app.visible_page() == Some(Page::Run);
    let natural = if !at_table && window.viewport_size().width >= theme::RAIL_WIDE_FROM {
        theme::RAIL_WIDE_WIDTH
    } else {
        theme::RAIL_WIDTH
    };
    let widest = if at_table {
        natural
    } else {
        theme::RAIL_DRAG_MAX_WIDTH
    };
    kit::EdgeColumn::new(
        "navigation",
        kit::edge::Side::Left,
        natural,
        theme::RAIL_WIDTH..=widest,
    )
    .page_keeps(theme::RAIL_LEAVES_WINDOW)
}

/// The rail's width now: its natural width, or the one the DM dragged it to.
pub fn width(root: &SplatRoot, window: &Window, cx: &gpui::App) -> gpui::Pixels {
    column(root, window).width(window.viewport_size().width, cx)
}

/// Draw the rail.
pub fn render(
    root: &mut SplatRoot,
    connection: crate::Connection,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> gpui::Stateful<Div> {
    let c = colors();
    let page = root.app.visible_page();
    let mut top = div()
        .flex()
        .flex_col()
        .gap(RAIL_ITEM_GAP)
        .child(kit::text(Type::Title, c.text, "Splat"))
        .child(kit::gap(XS))
        .child(switcher(root, cx))
        .child(kit::gap(S))
        .child(run_slot(root, page, cx));
    let review = root.app.campaign_content.review_detail();
    for (item, icon) in [
        (Page::Encounters, icons::ENCOUNTERS),
        (Page::Sessions, icons::SESSIONS),
        (Page::Timeline, icons::TIMELINE),
        (Page::World, icons::WORLD),
        (Page::Party, icons::PARTY),
        (Page::Library, icons::LIBRARY),
        (Page::Campaign, icons::CAMPAIGN),
    ] {
        let detail = match item {
            Page::Campaign => review.clone(),
            _ => None,
        };
        top = top.child(
            kit::NavItem::new(
                gpui::ElementId::Name(format!("nav-{}", item.name()).into()),
                icon,
                item.name(),
            )
            .detail(detail)
            .selected(page == Some(item))
            .on_click(cx.listener(move |root, _, _, cx| root.show_page(item, cx))),
        );
    }
    let mut statuses = root.app.statuses(connection).into_iter();
    // The account line sits last, below Settings.
    let account = statuses.next();
    let slot = status_slot(root, statuses.collect(), cx);
    let lower = lower_group(root, page, account, cx);
    div()
        .id("rail")
        .flex()
        .flex_col()
        .size_full()
        .p(S)
        .bg(c.rail)
        .border_r(theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .child(top)
        .child(div().flex_1().min_h_0())
        .child(slot)
        .child(lower)
}

/// The campaign switcher (`Campaigns::selector`): the active campaign's name, a lock while the
/// campaign cannot change, and `Add campaign` when there is none.
fn switcher(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> Div {
    let campaigns = &root.app.campaigns;
    let blocked = root.app.blocked_switch();
    let lock = campaigns.switcher_lock(blocked).map(str::to_owned);
    let label = campaigns.switcher_label().to_owned();
    let all: Vec<_> = campaigns
        .all()
        .iter()
        .map(|campaign| (campaign.id, campaign.name.clone()))
        .collect();
    let active = campaigns.active_id();
    let mut options: Vec<SharedString> = Vec::new();
    let mut ids = Vec::new();
    if !all.is_empty() {
        options.push("No campaign".into());
        ids.push(None);
    }
    for (id, name) in &all {
        options.push(name.clone().into());
        ids.push(Some(*id));
    }
    let chosen = ids
        .iter()
        .position(|id| *id == active)
        .filter(|_| active.is_some());
    let mut dropdown = kit::Dropdown::new(
        "active-campaign",
        format!("Active campaign: {label}"),
        options,
        chosen,
    )
    .placeholder(label)
    .blocked(lock)
    .on_select(cx.listener(move |root, index: &usize, _, cx| {
        let Some(id) = ids.get(*index).copied() else {
            return;
        };
        if id != root.app.campaigns.active_id() {
            root.app.campaigns.choose(id);
        }
        cx.notify();
    }));
    if all.is_empty() {
        dropdown = dropdown.command(kit::MenuItem::new("Add campaign").on_click(cx.listener(
            |root, _, _, cx| {
                root.app.campaigns.focus_add();
                root.app.settings_tab = SettingsTab::Account;
                root.show_page(Page::Settings, cx);
            },
        )));
    }
    div()
        .h(px(SELECTOR_HEIGHT))
        .flex_none()
        .child(dropdown.density(theme::Density::Read))
}

/// The Run slot: the open run, the paused run, or a quiet `Run` with `No fight open`.
fn run_slot(root: &mut SplatRoot, page: Option<Page>, cx: &mut Context<SplatRoot>) -> Div {
    let app = &root.app;
    let item = if app.encounters.has_run() {
        let detail = app
            .encounters
            .run()
            .map(|run| run.state_words(false))
            .unwrap_or_else(|| "Setup".to_owned());
        let copy = app.encounters.run_copy_label();
        let hint = format!(
            "{detail}{}",
            copy.map_or(String::new(), |copy| format!(" · {copy}"))
        );
        kit::NavItem::new("nav-run", icons::RUN, "Run")
            .detail(Some(detail.clone()))
            .name(format!("Run · {detail}"))
            .selected(page == Some(Page::Run))
            .hint(hint)
            .on_click(cx.listener(|root, _, _, cx| root.show_page(Page::Run, cx)))
    } else if let Some(paused) = app.encounters.paused_run() {
        let fights = app.encounters.paused_run_count();
        let (path, name) = (paused.path.clone(), paused.encounter.clone());
        let state = encounters::run_state_word(paused, false);
        let label = if fights == 1 {
            "Paused run"
        } else {
            "Paused runs"
        };
        let detail = if fights == 1 {
            name.clone()
        } else {
            crate::design::count(fights, "fight", "fights")
        };
        let full = if fights == 1 {
            format!("{name} · {state}")
        } else {
            detail.clone()
        };
        kit::NavItem::new("nav-run", icons::RUN, label)
            .detail(Some(detail))
            .name(format!("{label} · {full}"))
            .hint(full)
            .on_click(cx.listener(move |root, _, _, cx| {
                if fights > 1 {
                    root.show_page(Page::Encounters, cx);
                } else if let Some(active) = root.app.campaigns.active_campaign() {
                    root.app.encounters.open_saved_run(&path, &active);
                    cx.notify();
                }
            }))
    } else {
        kit::NavItem::new("nav-run", icons::RUN, "Run")
            .detail(Some(NO_FIGHT_OPEN))
            .name(crate::no_run_name())
            .hint(format!("{NO_FIGHT_OPEN}. Encounters lists the fights."))
            .on_click(cx.listener(|root, _, _, cx| root.show_page(Page::Encounters, cx)))
    };
    div().child(item)
}

/// The status slot above Lookup: a recording, the rail's statuses, the trackers' sign, and words
/// that could not be saved, each leading to where the DM can act.
fn status_slot(
    root: &mut SplatRoot,
    statuses: Vec<crate::design::Status<'static>>,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let app = &root.app;
    let mut slot = div().flex().flex_col().gap(RAIL_ITEM_GAP);
    if let (Some(status), Some(words)) = (
        app.sessions.rail_recording_status(),
        app.sessions.recording_detail(),
    ) {
        let state = status.word;
        let full = if state == "Processed" && app.sessions.job_warned() {
            crate::sessions::page::processed_warning().word.to_owned()
        } else {
            app.sessions
                .recording_hover()
                .unwrap_or_else(|| state.to_owned())
        };
        let shown = kit::Status::new(status.icon, words, bridge::color(status.word_color()))
            .icon_color(bridge::color(status.icon_color()));
        slot = slot.child(
            kit::status_link("rail-recording", shown)
                .hint(full)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.open_live_recording();
                    root.show_page(Page::Sessions, cx);
                })),
        );
    }
    for (index, state) in statuses.into_iter().enumerate() {
        slot = slot.child(rail_status(root, ("rail-status", index), state, false, cx));
    }
    if let Some(trackers) = trackers_status(root, cx) {
        slot = slot.child(trackers);
    }
    let app = &root.app;
    for (lost, word, target) in [
        (
            app.sessions.busy_with_edit(),
            if app.sessions.notes_changed_on_disk() {
                "Notes changed"
            } else {
                "Notes not saved"
            },
            Page::Sessions,
        ),
        (
            app.timeline.not_saved(),
            app.timeline
                .attention_status()
                .map_or("Timeline not saved", |state| state.word),
            Page::Timeline,
        ),
    ] {
        if !lost {
            continue;
        }
        let attention = match target {
            Page::Sessions => app.sessions.attention_status(),
            _ => app.timeline.attention_status(),
        };
        let status = kit::Status::new(
            attention.map_or(icons::ERROR, |state| state.icon),
            word,
            attention.map_or(colors().document.invalid, |state| {
                bridge::color(state.color)
            }),
        );
        let hint = if target == Page::Sessions && app.sessions.notes_changed_on_disk() {
            "Notes changed on disk"
        } else {
            word
        };
        slot = slot.child(
            kit::status_link(
                gpui::ElementId::Name(format!("rail-lost-{}", target.name()).into()),
                status,
            )
            .hint(hint)
            .on_click(cx.listener(move |root, _, _, cx| root.show_page(target, cx))),
        );
    }
    slot
}

/// The trackers' sign (`Splat::trackers_status`): `Due N` while notices are due, or `Trackers not
/// read`; nothing with no house rule note in play.
fn trackers_status(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> Option<impl IntoElement> {
    let trackers = &root.app.trackers;
    if !trackers.in_play() {
        return None;
    }
    let due = trackers.due();
    let not_read = if trackers.unreadable.is_some() {
        Some(crate::trackers::ledger::FILE)
    } else if trackers.rules.unreadable.is_some() {
        Some("A house rule file")
    } else {
        None
    };
    let (status, hint) = if let Some(file) = not_read {
        (
            kit::Status::new(icons::ERROR, "Trackers not read", colors().result.error),
            format!("{file} cannot be read. Open Trackers (Ctrl+T)."),
        )
    } else if due > 0 {
        (
            kit::Status::new(icons::BELL, format!("Due {due}"), colors().text),
            format!(
                "{} due. Open Trackers (Ctrl+T).",
                crate::design::count(due, "notice is", "notices are")
            ),
        )
    } else {
        return None;
    };
    Some(
        kit::status_link("rail-trackers", status)
            .hint(hint)
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.notes_give_way();
                root.app.lookup.show_trackers();
                cx.notify();
            })),
    )
}

/// One status line in the rail (`Splat::rail_status`). Some lead to where the DM can act on them;
/// the account line also says while the party is being read.
fn rail_status(
    root: &mut SplatRoot,
    id: impl Into<gpui::ElementId>,
    state: crate::design::Status<'static>,
    account: bool,
    cx: &mut Context<SplatRoot>,
) -> gpui::AnyElement {
    let app = &root.app;
    let held = match app.world_page.held_npc() {
        Some(name) => Some(format!(
            "{}: {name}. Show its card on World.",
            world::page::UNSAVED_NPC
        )),
        None => app
            .lookup
            .generated_name()
            .map(|name| format!("{}: {name}. Show it in Lookup.", world::page::UNSAVED_NPC)),
    };
    let word = state.word;
    let full = match word {
        world::page::UNSAVED_NPC => held.unwrap_or_else(|| word.to_owned()),
        "Signed out" => "Signed out of D&D Beyond".to_owned(),
        "Sync stopped" => {
            "Party sync stopped when D&D Beyond disconnected. Connect, then turn sync on in Party."
                .to_owned()
        }
        _ => word.to_owned(),
    };
    let updating = account && app.party.connected() && app.party.busy();
    let full = if updating {
        format!("{full} · Updating the party")
    } else {
        full
    };
    let links = word == "Sync stopped"
        || word == "Session folder not found"
        || word == world::page::UNSAVED_NPC
        || draft_page(word).is_some();
    if word == "Connected · not remembered" {
        // Two lines: the connection, then `Not remembered` under its word.
        let status = bridge::status(&crate::design::Status {
            word: "Connected",
            ..state
        });
        return div()
            .flex()
            .flex_col()
            .min_h(theme::Type::Small.line() * 2.0)
            .child(kit::status(&status))
            .child(div().pl(theme::ICON_SLOT - XS).child(kit::text(
                Type::Small,
                colors().muted,
                "Not remembered",
            )))
            .into_any_element();
    }
    let status = bridge::status(&state);
    if !links {
        let line = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(XS)
            .child(kit::status(&status))
            .when(updating, |line| {
                line.child(kit::icon(
                    icons::BUSY,
                    Type::Small,
                    colors().connection.busy,
                ))
            });
        return if account {
            div()
                .min_h(theme::Type::Small.line() * 2.0)
                .child(line)
                .into_any_element()
        } else {
            line.into_any_element()
        };
    }
    kit::status_link(id, status)
        .hint(full)
        .on_click(cx.listener(move |root, _, _, cx| {
            let app = &mut root.app;
            if word == "Sync stopped" {
                app.party.wants_combat_sync = true;
                app.page = Page::Party;
            } else if word == "Session folder not found" {
                app.page = Page::Sessions;
            } else if word == world::page::UNSAVED_NPC {
                if app.world_page.has_generated() {
                    app.world_page.open_held();
                    app.page = Page::World;
                } else {
                    app.notes_give_way();
                    app.lookup.show_generated();
                }
            } else if let Some((page, tab)) = draft_page(word) {
                if word == encounters::GROUP_DRAFT && app.encounters.group_open_in_plan() {
                    app.page = Page::Sessions;
                    app.sessions.show_group_plan();
                } else {
                    app.page = page;
                    if let Some(tab) = tab {
                        app.campaign_tab = tab;
                    }
                }
            }
            cx.notify();
        }))
        .into_any_element()
}

/// Lookup, Notes, Settings and the account line, at the rail's bottom edge.
fn lower_group(
    root: &mut SplatRoot,
    page: Option<Page>,
    account: Option<crate::design::Status<'static>>,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let app = &root.app;
    let mut lower = div()
        .flex()
        .flex_col()
        .gap(RAIL_ITEM_GAP)
        .pt(XS)
        .child(
            kit::NavToggle::new(
                "nav-lookup",
                icons::SEARCH,
                "Lookup",
                "Ctrl+K",
                app.lookup.open,
            )
            .hint("Search everything, beside this page (Ctrl+K)")
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.toggle_lookup();
                cx.notify();
            })),
        )
        .child(
            kit::NavToggle::new(
                "nav-notes",
                icons::DRAFT,
                "Notes",
                "Ctrl+J",
                app.sessions.notes_open && app.page != Page::Sessions,
            )
            .hint("This session’s notes, beside this page (Ctrl+J)")
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.toggle_notes();
                cx.notify();
            })),
        )
        .child(
            kit::NavItem::new("nav-settings", icons::SETTINGS, Page::Settings.name())
                .selected(page == Some(Page::Settings))
                .on_click(cx.listener(|root, _, _, cx| root.show_page(Page::Settings, cx))),
        );
    // The account slot holds two `Small` lines, so a status that wraps never moves the others.
    let account = account.map(|account| rail_status(root, "rail-account", account, true, cx));
    lower = lower.child(
        div()
            .pt(XS)
            .min_h(theme::Type::Small.line() * 2.0)
            .children(account),
    );
    lower
}
