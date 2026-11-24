//! The entry Lookup shows (egui: the reader part of `Lookup::panel`): Lookup's own `world::Reader`,
//! compact, drawn by the same view as World's reader (`world::reader::render_reader` with
//! `Host::Lookup`): the title line with Edit, Start fight or Add to fight and Add to Log, the Log
//! field, the faults and their repairs, the value rows with Change, the NPC's edit form, the
//! entry as the reader shows every entry (stat block, facts, sections, related entries, the end
//! sections), what other notes say, and a house rule note's tracker history. Its results and
//! their Undo, and the history's, sit in the pane's footer, so a result never moves the text.
use crate::design::outcome::{Messages, PaneFrame, Press};
use crate::gui::pages::world::reader::{Host, render_reader};
use crate::gui::theme::{S, Type, colors};
use crate::gui::{SplatRoot, kit};
use crate::world::FooterAct;
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div};

/// The entry at `position`, in a pane whose footer holds the reader's messages.
pub(super) fn render(
    root: &mut SplatRoot,
    position: usize,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let Some(index) = root.app.lookup.index() else {
        return div().into_any_element();
    };
    let Some(item) = index.items.get(position) else {
        return div().into_any_element();
    };
    let Some(campaign) = root.app.campaigns.active_campaign() else {
        return without_campaign(root, position, cx);
    };
    let id = item.entry.id;
    // Lookup's reader is the compact one: no file line, no `Give this note a type`.
    let app = &mut root.app;
    app.lookup.reader.compact = true;
    app.lookup.reader.open_item(item);
    app.lookup.reader.check_disk(&index, Some(&campaign));
    let lines = {
        let frame = PaneFrame::new();
        let mut messages = Messages::new();
        app.lookup
            .reader
            .footer(&frame, &mut messages, item, &index, Some(&campaign));
        crate::world::TrackerUi::history_footer(
            &crate::trackers::Reader(&mut app.trackers),
            id,
            &frame,
            &mut messages,
        );
        messages.into_plain()
    };
    let footer = kit::messages::lines(
        ("lookup-reader-footer", id.as_u128() as u64),
        lines,
        cx.listener(
            move |root: &mut SplatRoot, pressed: &kit::messages::Pressed<FooterAct>, _, cx| {
                if let Some(press) = pressed.take() {
                    footer_press(root, press, Some(id), cx);
                }
            },
        ),
    );
    // The open action's lock, once above the entry, from the text edge (ruling R35).
    let lock = root
        .app
        .lookup
        .open_reason()
        .map(|reason| kit::text(Type::Small, colors().muted, reason).pb(S));
    let body = render_reader(root, Host::Lookup, item, &index, &campaign, window, cx);
    let mut pane = kit::Pane::new(("lookup-entry", id.as_u128() as u64))
        .flush()
        .scroll(&root.pages.lookup.jump.scroll)
        .children(lock)
        .child(body);
    for line in footer {
        pane = pane.footer(line);
    }
    pane.into_any_element()
}

/// An entry read with no campaign open (a built-in spell, a package's creature): its title and
/// kind line, then the entry as the reader shows it. Nothing can be edited or added to a fight.
fn without_campaign(
    root: &mut SplatRoot,
    position: usize,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let Some(index) = root.app.lookup.index() else {
        return div().into_any_element();
    };
    let item = &index.items[position];
    let id = item.entry.id;
    let open = cx.listener(|root: &mut SplatRoot, at: &usize, _, cx| {
        if let Some(nav) = root.app.lookup.reader.go_to(*at) {
            root.app.lookup.follow_nav(nav);
        }
        cx.notify();
    });
    let follow = cx.listener(move |root: &mut SplatRoot, note: &str, _, cx| {
        let SplatRoot { app, ctx, .. } = root;
        app.lookup.open_note_link(note, Some(id), ctx);
        cx.notify();
    });
    let reader = crate::gui::pages::reader::entry::EntryView::new(
        ("lookup-entry-view", id.as_u128() as u64),
        item.entry.name.clone(),
        Some(&item.entry),
        item.entry.body.clone(),
        crate::gui::pages::reader::entry::Source {
            path: item.entry.path.clone(),
            owner: item.owner.clone(),
            builtin: item.is_builtin(),
        },
        crate::gui::pages::reader::entry::index_links(index.clone()),
    )
    .on_open(move |at, window, cx| open(&at, window, cx))
    .on_note(follow)
    .jump_in(&root.pages.lookup.jump.scroll);
    kit::Pane::new(("lookup-entry", id.as_u128() as u64))
        .flush()
        .scroll(&root.pages.lookup.jump.scroll)
        .child(crate::gui::pages::reader::entry::title_with_art(
            &item.entry.name,
            &index.result_detail(position),
            Some(&item.entry),
            cx,
        ))
        .child(reader)
        .into_any_element()
}

/// With no entry on show, the words of a Log line whose entry is gone still sit in Lookup's
/// footer with `Copy text`, until the DM copies or dismisses them.
pub(super) fn gone_lines(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> Vec<AnyElement> {
    let campaign = root.app.lookup.campaign().map(|campaign| campaign.id);
    let lines = {
        let frame = PaneFrame::new();
        let mut messages = Messages::new();
        root.app
            .lookup
            .reader
            .gone_log_messages(&frame, &mut messages, campaign);
        messages.into_plain()
    };
    kit::messages::lines(
        "lookup-footer",
        lines,
        cx.listener(
            |root: &mut SplatRoot, pressed: &kit::messages::Pressed<FooterAct>, _, cx| {
                if let Some(press) = pressed.take() {
                    footer_press(root, press, None, cx);
                }
            },
        ),
    )
}

/// A press in the reader's footer: the trackers take their Undo, the reader every other press
/// (egui: `Lookup::panel` after its pane); what it opens next goes to Lookup.
fn footer_press(
    root: &mut SplatRoot,
    press: Press<FooterAct>,
    open: Option<uuid::Uuid>,
    cx: &mut Context<SplatRoot>,
) {
    if press.id == FooterAct::Trackers {
        root.app.trackers.undo_remove(press.ticket);
        cx.notify();
        return;
    }
    let Some(campaign) = root.app.lookup.campaign().cloned() else {
        return;
    };
    let (nav, effect) = root.app.lookup.reader.footer_press(press, open, &campaign);
    if let Some(nav) = nav {
        root.app.lookup.follow_nav(nav);
    }
    match effect {
        Some(crate::world::model::Effect::Copy(words)) => {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(words));
        }
        Some(crate::world::model::Effect::Open(url)) => cx.open_url(&url),
        None => {}
    }
    cx.notify();
}
