//! The D&D Beyond account (egui: `Settings::show`, `show_compact`, `show_account`): its status,
//! its help, the sign-in form, and Disconnect. Wide, it is the Settings tab's right column;
//! narrow, a compact group under (or, during a run, above) the campaigns.
use crate::gui::kit::{self, Button, Checkbox, Link, Message, Section, TextField, Tone};
use crate::gui::theme::{Density, Type, XS, colors};
use crate::gui::{SplatRoot, bridge};
use crate::settings::Account;
use gpui::{
    AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, prelude::*, px,
};

/// The sign-in page of D&D Beyond.
const SIGN_IN_URL: &str = "https://www.dndbeyond.com/sign-in";

/// The account in the wide tab: its heading, its status and everything under it.
pub(super) fn wide(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .flex()
                .items_center()
                .h(Density::Read.control())
                .child(kit::heading(
                    kit::Heading::Group {
                        title: "D&D Beyond account",
                        count: 0,
                    },
                    true,
                )),
        )
        .child(body(root, true, window, cx))
        .into_any_element()
}

/// The account in the narrow tab (`show_compact`): a status line with `Connect…`, or a caret
/// section with the sign-in details while connected; the whole form once the DM asks for it.
pub(super) fn compact(
    root: &mut SplatRoot,
    first: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let settings = &root.app.settings;
    let account = settings.account_now();
    let open = root.pages.settings.compact_open
        || settings.signing_in_again()
        || root.pages.settings.focus_cookie;
    let heading = kit::heading(
        kit::Heading::Group {
            title: "D&D Beyond account",
            count: 0,
        },
        first,
    );
    if open {
        root.pages.settings.compact_open = true;
        return div()
            .flex()
            .flex_col()
            .child(heading)
            .child(body(root, true, window, cx))
            .into_any_element();
    }
    let details = !matches!(
        account,
        Account::NotConnected | Account::Failed | Account::RefreshFailing
    );
    let c = colors();
    let status = bridge::status(&settings.connection_status());
    let line = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(crate::gui::theme::S)
        .child(kit::status(&status))
        .when(account == Account::RefreshFailing, |line| {
            line.child(kit::text(
                Type::Small,
                c.muted,
                "Sync works for about one more minute.",
            ))
        })
        .when(!details, |line| {
            line.child(
                Button::quiet(
                    "account-connect-open",
                    if matches!(account, Account::NotConnected | Account::Failed) {
                        "Connect…"
                    } else {
                        "Connect again…"
                    },
                )
                .on_click(cx.listener(|root, _, _, cx| {
                    root.pages.settings.compact_open = true;
                    root.pages.settings.focus_cookie = true;
                    cx.notify();
                })),
            )
        });
    let mut column = div().flex().flex_col().child(heading).child(line);
    if details {
        let open = root.pages.settings.sign_in_details_open;
        let mut section = Section::new("compact-sign-in-details", "Sign-in details", open)
            .on_toggle(cx.listener(|root, open: &bool, _, cx| {
                root.pages.settings.sign_in_details_open = *open;
                cx.notify();
            }));
        if open {
            section = section.child(body(root, false, window, cx));
        }
        column = column.child(section);
    }
    column.into_any_element()
}

/// The account's status (when `status`), help, warning, sign-in form and Disconnect.
fn body(root: &mut SplatRoot, status: bool, _: &mut Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let focus_cookie = std::mem::take(&mut root.pages.settings.focus_cookie)
        | root.app.settings.take_focus_cookie();
    let campaign_form_open = root.app.campaigns.form_open();
    let settings = &root.app.settings;
    let account = settings.account_now();
    let mut column = div().flex().flex_col().gap(XS);
    if status {
        column = column.child(kit::status(&bridge::status(&settings.connection_status())));
    }
    match account {
        Account::Connected if settings.remembered() => {
            column = column.child(kit::help("Splat signs in again when it starts."));
        }
        Account::Connected => {
            column = column.child(kit::help("Splat stays signed in while it is open."));
        }
        Account::RefreshFailing => {
            column = column.child(kit::help(
                "Splat keeps trying. Sync works for about one more minute.",
            ));
        }
        _ => {}
    }
    let ended = settings.session_ended();
    if let Some(words) = ended {
        column = column.child(kit::help(words));
    }
    if let Some(warning) = settings.warning() {
        let message = if account == Account::KeyringWarning {
            Some(
                Message::new(
                    Tone::Warning,
                    "This computer did not save your sign-in. Unlock your password manager, then connect again.",
                )
                .action(Button::secondary("account-keyring-again", "Connect again").on_click(
                    cx.listener(|root, _, _, cx| {
                        root.app.settings.focus_sign_in();
                        cx.notify();
                    }),
                )),
            )
        } else if warning.trim().is_empty() {
            None
        } else {
            Some(Message::new(Tone::Warning, warning.to_owned()))
        };
        column = column.children(message);
    }
    if account == Account::RefreshFailing {
        column = column.child(
            Button::secondary("account-connect-again", "Connect again").on_click(cx.listener(
                |root, _, _, cx| {
                    root.app.settings.focus_sign_in();
                    cx.notify();
                },
            )),
        );
    }
    if settings.sign_in_open() {
        if ended.is_none() {
            column = column.child(kit::help(
                "Connect your account to import campaigns and read characters.",
            ));
        }
        column = column.child(
            Link::new("account-sign-in-link", "Open D&D Beyond sign-in")
                .on_click(|_, _, cx| cx.open_url(SIGN_IN_URL)),
        );
        let help_open = root.pages.settings.cookie_help_open;
        let mut help = Section::new(
            "account-cookie-help",
            "How to get your sign-in cookie",
            help_open,
        )
        .on_toggle(cx.listener(|root, open: &bool, _, cx| {
            root.pages.settings.cookie_help_open = *open;
            cx.notify();
        }));
        if help_open {
            help = help
                .child(kit::text(Type::Body, c.text, "1. Sign in to D&D Beyond in your browser."))
                .child(kit::text(
                    Type::Body,
                    c.text,
                    "2. Open developer tools (F12), then Application or Storage → Cookies.",
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_baseline()
                        .child(kit::text(
                            Type::Body,
                            c.text,
                            "3. Select dndbeyond.com and copy the value of ",
                        ))
                        .child(kit::text(Type::Mono, c.text, "CobaltSession"))
                        .child(kit::text(Type::Body, c.text, ".")),
                )
                .child(kit::text(
                    Type::Body,
                    c.text,
                    "4. Paste only the value below and select Connect.",
                ));
        }
        column = column.child(help);
        let cookie_width = px(420.0);
        let field = TextField::new("session_cookie", settings.cookie().to_owned())
            .placeholder("Paste sign-in cookie")
            .name("Paste sign-in cookie")
            .masked()
            .width(cookie_width)
            .focus(focus_cookie)
            .on_change(cx.listener(|root, words: &str, _, cx| {
                root.app.settings.set_cookie(words);
                cx.notify();
            }));
        column = column.child(kit::field("Sign-in cookie", None, None, field).pb(px(0.0)));
        column = column.child(
            Checkbox::new(
                "account-remember",
                "Remember my D&D Beyond sign-in on this computer",
                settings.remember(),
            )
            .on_click(cx.listener(|root, _, _, cx| {
                let remember = !root.app.settings.remember();
                root.app.settings.set_remember(remember);
                cx.notify();
            })),
        );
        let reason = settings.connect_reason();
        let failure = settings.connect_failure().filter(|_| reason.is_none());
        let shown = reason.map(str::to_owned).or(failure.map(str::to_owned));
        let connect = if reason.is_none() && account != Account::Connected && !campaign_form_open
        {
            Button::primary("account-connect", "Connect")
        } else {
            Button::secondary("account-connect", "Connect")
        }
        .blocked(reason)
        .on_click(cx.listener(|root, _, _, cx| {
            let SplatRoot { app, bridge, .. } = root;
            app.settings.connect(bridge.ctx());
            cx.notify();
        }));
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_end()
                .gap(crate::gui::theme::S)
                .w(cookie_width)
                .children(shown.map(|words| {
                    kit::text(
                        Type::Small,
                        if failure.is_some() && reason.is_none() {
                            c.result.error
                        } else {
                            c.muted
                        },
                        words,
                    )
                    .flex_1()
                    .min_w_0()
                    .text_right()
                }))
                .child(connect),
        );
        column = column.child(kit::help(
            "Splat sends your sign-in cookie only to D&D Beyond. With the box ticked, Splat signs in again when it starts.",
        ));
    }
    if matches!(
        account,
        Account::Connected | Account::KeyringWarning | Account::RefreshFailing
    ) {
        column = column.child(kit::help(if settings.remembered() {
            "Disconnect removes Splat’s saved D&D Beyond sign-in. It does not sign your browser out of D&D Beyond."
        } else {
            "Disconnect signs Splat out. It does not sign your browser out of D&D Beyond."
        }));
    }
    if settings.can_forget() {
        column = column.child(
            Button::quiet(
                "account-disconnect",
                if account == Account::Busy {
                    "Cancel"
                } else {
                    "Disconnect"
                },
            )
            .line_start()
            .density(Density::Read)
            .on_click(cx.listener(|root, _, _, cx| {
                let SplatRoot { app, bridge, .. } = root;
                app.settings.forget(bridge.ctx());
                root.pages.settings.compact_open = false;
                cx.notify();
            })),
        );
    }
    column.into_any_element()
}
