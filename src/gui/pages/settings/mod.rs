//! Settings → Account and campaigns, and Recording and processing (state: `src/settings.rs`,
//! `src/campaigns.rs`). The shell draws the header, the tabs
//! and Touchpad (`SplatRoot::settings_page`).
//!
//! Wide (1000 px and more), the tab has two panes, each with its own footer: the campaigns
//! (their results, the list's and D&D Beyond's errors) and the account (its errors). Narrow, the
//! two stack in one pane with one footer, in the page's order.
pub(crate) mod account;
pub(crate) mod campaign_list;
mod recording;
#[cfg(test)]
mod tests;

use crate::campaigns::CampaignAct;
use crate::design::outcome::{Messages, PaneFrame, plain::PlainLine};
use crate::gui::SplatRoot;
use crate::gui::kit::{self, Pane};
use crate::gui::theme::{L, LIST_MEASURE, XL};
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, px};

/// This area's GPUI-only state, kept in `root.pages.settings`.
#[derive(Default)]
pub struct State {
    /// The narrow account group shows its whole form (`Connect…`, or a request for the cookie).
    pub(crate) compact_open: bool,
    /// The narrow account's `Sign-in details` section is open.
    pub(crate) sign_in_details_open: bool,
    /// `How to get your sign-in cookie` is open.
    pub(crate) cookie_help_open: bool,
    /// The cookie field takes the keyboard in the next frame.
    pub(crate) focus_cookie: bool,
    /// `Add campaign` is open.
    pub(crate) add_open: bool,
    /// The add form's name field takes the keyboard in the next frame.
    pub(crate) focus_name: bool,
    /// `Add from D&D Beyond` is open (`None`: open while there is no campaign).
    pub(crate) import_open: Option<bool>,
    /// Recording and processing's own state.
    pub(crate) recording: recording::State,
}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {
    // The account column is the one place that tells the account form whether a campaign form is
    // open (its Connect stays secondary then).
    root.app.settings.campaign_form_open = root.app.campaigns.form_open();
}

/// The campaign messages of a footer (`Campaigns::footer_messages`), then the account's
/// (`with_account`, the narrow Settings tab's one footer), then D&D Beyond's, as plain lines in the
/// footer's order. `list`: Settings, where the add results belong. `link`: the open campaign on
/// Link and folder, whose link result shows.
pub(crate) fn campaign_lines(
    root: &SplatRoot,
    list: bool,
    with_account: bool,
    link: Option<uuid::Uuid>,
    blocked: Option<&str>,
) -> Vec<PlainLine<CampaignAct>> {
    let frame = PaneFrame::new();
    let mut messages = Messages::<CampaignAct>::new();
    root.app
        .campaigns
        .footer_messages(&frame, &mut messages, list, blocked);
    if with_account {
        root.app.settings.footer_messages(&mut messages);
    }
    root.app
        .dndbeyond
        .footer_messages(&frame, &mut messages, link);
    messages.into_plain()
}

/// The account's footer lines (`Settings::footer_messages`).
fn account_lines(root: &SplatRoot) -> Vec<PlainLine<CampaignAct>> {
    let mut messages = Messages::<CampaignAct>::new();
    root.app.settings.footer_messages(&mut messages);
    messages.into_plain()
}

/// A root handler for a kit part that gives no event (Enter in a field, Escape in a dialog).
pub(crate) fn act(
    cx: &mut Context<SplatRoot>,
    handler: impl Fn(&mut SplatRoot, &mut Window, &mut Context<SplatRoot>) + 'static,
) -> impl Fn(&mut Window, &mut gpui::App) + 'static {
    let listener = cx.listener(move |root, _: &(), window, cx| handler(root, window, cx));
    move |window, cx| listener(&(), window, cx)
}

/// A root handler for a footer's press (`kit::messages::lines`): the press, taken once.
pub(crate) fn on_press<A: 'static>(
    cx: &mut Context<SplatRoot>,
    handler: impl Fn(&mut SplatRoot, crate::design::outcome::Press<A>, &mut Context<SplatRoot>)
    + 'static,
) -> impl Fn(&kit::messages::Pressed<A>, &mut Window, &mut gpui::App) + 'static {
    cx.listener(move |root, pressed: &kit::messages::Pressed<A>, _, cx| {
        if let Some(press) = pressed.take() {
            handler(root, press, cx);
        }
    })
}

/// The lines as footer messages; a press goes to `Campaigns::footer_act`.
pub(crate) fn footer(
    id: &'static str,
    lines: Vec<PlainLine<CampaignAct>>,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    kit::messages::lines(
        id,
        lines,
        on_press(cx, |root, press, cx| {
            root.app.campaigns.footer_act(press);
            cx.notify();
        }),
    )
}

/// Settings → Account and campaigns.
pub(crate) fn render_account(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let blocked = root.app.blocked_switch();
    let (_, session) = root.app.settings.campaign_session();
    let dialog = campaign_list::remove_dialog(root, cx);
    let wide = window.viewport_size().width >= px(1000.0);
    let body = if wide {
        let campaign_lines = campaign_lines(root, true, false, None, blocked);
        let account_lines = account_lines(root);
        let list = campaign_list::list(root, session, blocked, window, cx);
        let account = account::wide(root, window, cx);
        let mut campaigns = Pane::new("settings-campaigns").flush().child(list);
        for line in footer("settings-campaign-footer", campaign_lines, cx) {
            campaigns = campaigns.footer(line);
        }
        let mut account_pane = Pane::new("settings-account").flush().child(account);
        for line in footer("settings-account-footer", account_lines, cx) {
            account_pane = account_pane.footer(line);
        }
        div()
            .flex()
            .flex_row()
            .size_full()
            .pt(L)
            .gap(XL)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .max_w(LIST_MEASURE)
                    .h_full()
                    .child(campaigns),
            )
            .child(div().flex_1().min_w_0().h_full().child(account_pane))
            .into_any_element()
    } else {
        let lines = campaign_lines(root, true, true, None, blocked);
        let has_run = root.app.encounters.has_run();
        let mut column = div().flex().flex_col().w_full();
        if has_run {
            column = column
                .child(account::compact(root, true, window, cx))
                .child(kit::gap(L))
                .child(campaign_list::list(root, session, blocked, window, cx));
        } else {
            column = column
                .child(kit::gap(L))
                .child(campaign_list::list(root, session, blocked, window, cx))
                .child(account::compact(root, false, window, cx));
        }
        let mut pane = Pane::new("settings-narrow")
            .flush()
            .child(column.max_w(LIST_MEASURE));
        for line in footer("settings-footer", lines, cx) {
            pane = pane.footer(line);
        }
        pane.into_any_element()
    };
    div()
        .size_full()
        .child(body)
        .children(dialog)
        .into_any_element()
}

/// Settings → Recording and processing (`ProcessingSettings::pane`).
pub(crate) fn render_recording(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    recording::render(root, window, cx)
}

/// The add-campaign form that the Campaign page shows when there is no campaign
/// (`Campaigns::show_add_form`, `form_error_inline`).
pub(crate) fn render_add_campaign_form(
    root: &mut SplatRoot,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    root.app.campaigns.set_form_open(true);
    let locked = root.app.campaigns.list_lock(root.app.blocked_switch());
    let form = campaign_list::add_form(root, true, locked, cx);
    let error = campaign_list::form_error_inline(root, cx);
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(form)
        .children(error)
        .into_any_element()
}
