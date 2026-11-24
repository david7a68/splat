//! The session banner (`Sessions::banner`): what cannot wait, on every page. The shell places it
//! above the page; it holds the recording alerts first, then another session's live recording,
//! the notices that arrived by themselves, and the note's warning with its choices. At most a
//! third of the window; it scrolls past that.
use super::footer::tone;
use crate::design::outcome::plain::Kind;
use crate::gui::{SplatRoot, bridge, kit, theme};
use crate::sessions::page::gpui_facade::{BannerAct, BannerChoice, BannerMessage};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window,
    div, prelude::*,
};
use theme::{M, S, Type, XS, colors};

pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let run_open = root.app.encounters.has_run();
    let banner = root
        .app
        .sessions
        .gv_banner(&root.app.processing, run_open)?;
    let c = colors();
    let other = banner.other.map(|other| {
        div()
            .flex()
            .flex_row()
            .items_baseline()
            .gap(XS)
            .child(kit::icon(
                other.status.icon,
                Type::Small,
                bridge::color(other.status.icon_color()),
            ))
            .child(kit::text(
                Type::Small,
                bridge::color(other.status.word_color()),
                other.words,
            ))
            .child(kit::text(Type::Small, c.muted, other.detail))
    });
    let messages = banner
        .messages
        .into_iter()
        .enumerate()
        .map(|(at, message)| self::message(at, message, cx))
        .collect::<Vec<_>>();
    if other.is_none() && messages.is_empty() {
        return None;
    }
    let most = window.viewport_size().height / 3.0;
    Some(
        div()
            .w_full()
            .flex_none()
            .bg(c.raised)
            .border_t(theme::HAIRLINE_WIDTH)
            .border_color(c.hairline)
            .px(M)
            .py(XS)
            .max_h(most)
            .child(kit::scroll_column(
                "session-banner",
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .py(XS)
                    .children(other)
                    .children(messages),
            ))
            .into_any_element(),
    )
}

fn message(at: usize, message: BannerMessage, cx: &mut Context<SplatRoot>) -> AnyElement {
    let BannerMessage {
        tone: words_tone,
        words,
        hover,
        choices,
    } = message;
    let mut shown = kit::Message::new(tone(words_tone), words.clone());
    let mut dismiss = None;
    for choice in choices {
        let BannerChoice {
            words: label,
            kind,
            blocked,
            act,
        } = choice;
        let id = ElementId::Name(SharedString::from(format!("banner-{at}-{label}")));
        let click = cx.listener(move |root, _, _, cx| {
            press(root, act.clone());
            cx.notify();
        });
        match kind {
            Kind::Dismiss => {
                dismiss = Some(kit::IconButton::new(id, crate::gui::icons::CLOSE, label).on_click(click));
            }
            Kind::Lose => {
                shown = shown.destructive(
                    kit::Button::destructive(id, label)
                        .density(theme::Density::Read)
                        .blocked(blocked)
                        .on_click(click),
                );
            }
            _ => {
                let button = if kind == Kind::Primary {
                    kit::Button::primary(id, label)
                } else {
                    kit::Button::secondary(id, label)
                };
                shown = shown.action(
                    button
                        .density(theme::Density::Read)
                        .blocked(blocked)
                        .on_click(click),
                );
            }
        }
    }
    let words = div().flex_1().min_w_0().child(shown);
    let words = match hover {
        Some(hover) => div()
            .id(ElementId::Name(SharedString::from(format!("banner-words-{at}"))))
            .flex_1()
            .min_w_0()
            .child(words)
            .with_tooltip(hover)
            .into_any_element(),
        None => words.into_any_element(),
    };
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(S)
        .child(words)
        .children(dismiss)
        .into_any_element()
}

fn press(root: &mut SplatRoot, act: BannerAct) {
    let ctx = root.bridge.ctx().clone();
    let app = &mut root.app;
    app.sessions
        .gv_banner_act(act, &ctx, &app.processing, &app.timeline);
}

use kit::WithTooltip as _;
