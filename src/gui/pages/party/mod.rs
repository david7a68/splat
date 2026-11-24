//! The Party page and the party bar on the Session page (egui: `src/party.rs` (`PartyPage`),
//! `src/party_support/sheets.rs`, `src/party_bar.rs`, `Splat::party_bar`).
mod bar;
mod list;
pub(crate) mod sheet;
#[cfg(test)]
mod tests;

use crate::gui::kit::{self, Button, Link};
use crate::gui::theme::{Type, colors};
use crate::gui::{SplatRoot, icons};
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, px};
use splat_party_page::PartyHeader;

/// This area's GPUI-only state, kept in `root.pages.party`.
#[derive(Default)]
pub struct State {
    /// The sheet's page (`SHEET_PAGES`).
    pub(crate) page: usize,
    /// `Filter this page`.
    pub(crate) search: String,
    /// The Combat sync section is open.
    pub(crate) sync_open: bool,
    /// The open caret sections and entries of the sheet, by key.
    pub(crate) open: std::collections::HashSet<String>,
}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {
    root.app.party_page.run_open = root.app.run_fact.is_some();
}

/// The page.
pub(crate) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let (_, session) = root.app.settings.campaign_session();
    let connected = session.is_some();
    let linked = root
        .app
        .campaigns
        .active_campaign()
        .is_some_and(|campaign| campaign.dndbeyond_campaign_id.is_some());
    let header = PartyHeader::of(&root.app.party, connected, linked);
    let words = header
        .context
        .split_once(' ')
        .map_or(header.context.as_str(), |(_, words)| words)
        .to_owned();
    let context = kit::status(&kit::Status::new(header.icon, words, c.muted)).into_any_element();
    let mut actions = Vec::new();
    if header.refresh {
        if header.connect {
            actions.push(
                Link::new("party-connect-to-refresh", "Connect to refresh")
                    .on_click(cx.listener(|root, _, _, cx| {
                        root.app.party_page.wants_settings = true;
                        cx.notify();
                    }))
                    .into_any_element(),
            );
        }
        let refresh_session = session.clone();
        actions.push(
            Button::quiet("party-refresh", "Refresh party")
                .icon(icons::REFRESH)
                .blocked(header.refresh_reason.clone())
                .on_click(cx.listener(move |root, _, _, cx| {
                    if let Some(session) = refresh_session.clone() {
                        let SplatRoot { app, bridge, .. } = root;
                        app.party.reader.refresh_by_hand(session, bridge.ctx());
                    }
                    cx.notify();
                }))
                .into_any_element(),
        );
    }
    let mut page = div()
        .flex()
        .flex_col()
        .size_full()
        .child(kit::page_header_with("Party", Some(context), actions));
    if linked
        && connected
        && let Some(reason) = header.refresh_reason.clone()
    {
        page = page.child(kit::text(Type::Small, c.muted, reason));
    }
    let lines = {
        let crate::Splat {
            party_page, party, ..
        } = &mut root.app;
        party_page.footer_lines(party, connected, linked)
    };
    let footer = kit::messages::lines(
        "party-footer",
        lines,
        super::settings::on_press(cx, |root, press, cx| {
            let crate::Splat {
                party_page, party, ..
            } = &mut root.app;
            if let Some(url) = party_page.footer_press(party, press) {
                cx.open_url(&url);
            }
            cx.notify();
        }),
    );
    let body = body(root, session.is_some(), linked, window, cx);
    page.child(
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .child(body)
            .children(footer),
    )
    .into_any_element()
}

/// The empty states, or the list and the sheet.
fn body(
    root: &mut SplatRoot,
    connected: bool,
    linked: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let run_open = root.app.party_page.run_open;
    let button = |id: &'static str, words: &'static str| {
        if run_open {
            Button::secondary(id, words)
        } else {
            Button::primary(id, words)
        }
    };
    if !linked {
        let action = if !connected {
            button("party-connect", "Connect D&D Beyond").on_click(cx.listener(|root, _, _, cx| {
                root.app.party_page.wants_settings = true;
                cx.notify();
            }))
        } else {
            button("party-link", "Link this campaign").on_click(cx.listener(|root, _, _, cx| {
                root.app.party_page.wants_link_page = true;
                cx.notify();
            }))
        };
        return kit::empty_state(
            icons::PARTY,
            if connected {
                "Link this campaign to D&D Beyond to see its party"
            } else {
                "Connect D&D Beyond to see the party"
            },
            if connected {
                "Choose its D&D Beyond campaign under Campaign → Link and folder."
            } else {
                "First connect your D&D Beyond account. Then choose its campaign under Campaign → Link and folder."
            },
            Some(action.into_any_element()),
        )
        .into_any_element();
    }
    let reader = &root.app.party.reader;
    if reader.members.is_empty() {
        if reader.loaded {
            return kit::empty_state(
                icons::PARTY,
                "This campaign has no party members",
                "Add characters in D&D Beyond, then refresh the party.",
                Some(
                    Link::new("party-empty-connect", "Connect D&D Beyond")
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.party_page.wants_settings = true;
                            cx.notify();
                        }))
                        .into_any_element(),
                ),
            )
            .into_any_element();
        }
        if !reader.loading() {
            return kit::empty_state(
                icons::PARTY,
                "No characters yet",
                "Connect to D&D Beyond in Settings to load this party.",
                Some(
                    button("party-none-connect", "Connect D&D Beyond")
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.party_page.wants_settings = true;
                            cx.notify();
                        }))
                        .into_any_element(),
                ),
            )
            .into_any_element();
        }
        return div().into_any_element();
    }
    let tonight = splat_party_page::tonight_words(&root.app.party).map(kit::help);
    // The list takes about half the page beside its portraits, as the egui column does.
    let room = window.viewport_size().width - crate::gui::theme::RAIL_WIDTH;
    let list_width = (room * 0.46 + px(54.0)).clamp(px(384.0), px(624.0));
    let list = list::list(root, list_width, cx);
    let sheet = sheet::view(root, connected, window, cx);
    div()
        .flex()
        .flex_col()
        .size_full()
        .children(tonight)
        .child(
            kit::Columns::new("party-columns")
                .column(list_width, px(384.0)..px(694.0), list)
                .rest(div().size_full().pl(crate::gui::theme::M).child(sheet)),
        )
        .into_any_element()
}

/// The party bar on the Session page while a session is played (`Splat::party_bar`), or `None`.
/// It also sets `sessions.party_hidden` as the egui bar does.
pub(crate) fn render_bar(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let app = &mut root.app;
    let here = app.page == crate::Page::Sessions
        && app.campaigns.active_campaign().is_some()
        && app.party.has_members();
    let shows = here && app.sessions.party_bar_shows();
    app.sessions.party_hidden = here && !shows;
    if !shows {
        return None;
    }
    Some(bar::render(root, window, cx))
}
