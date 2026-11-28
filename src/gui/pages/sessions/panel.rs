//! The Notes panel beside another page (`Sessions::notes_panel`, Ctrl+J): the note of the session
//! being played, from its `## What happened` on, with its title, date, state and close icon.
use super::footer;
use crate::gui::{SplatRoot, kit, theme};
use crate::sessions::page::gpui_facade::PanelState;
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, prelude::*};
use std::rc::Rc;
use theme::{M, S, Type, colors};

pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let (title, date) = root.app.sessions.gv_panel_title();
    let status = root
        .app
        .sessions
        .gv_note_status()
        .map(|status| kit::status(&kit::Status::of(&status)));
    let head = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_baseline()
                .gap_x(S)
                .flex_1()
                .min_w_0()
                .child(kit::text(Type::Section, c.text, title))
                .children(date.map(|date| kit::text(Type::Small, c.muted, date))),
        )
        .child(
            kit::IconButton::new("notes-panel-close", crate::gui::icons::CLOSE, "Close notes")
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.notes_open = false;
                    cx.notify();
                })),
        );
    let body: AnyElement = match root.app.sessions.gv_panel() {
        PanelState::NoSession => kit::Message::new(kit::Tone::Info, "No session yet.")
            .action(
                kit::Button::primary("notes-panel-new", "New session").on_click(cx.listener(
                    |root, _, _, cx| {
                        root.app.sessions.gv_start_new_session();
                        cx.notify();
                    },
                )),
            )
            .into_any_element(),
        PanelState::NoCurrent => kit::Message::new(
            kit::Tone::Info,
            "New events go to no session, so no note is open here. Choose a session on the Timeline page.",
        )
        .into_any_element(),
        // While the file changed under the draft, the panel shows the draft beside the file
        // (egui: the shared note editor's comparison), so nothing is typed into a draft that
        // cannot save; the banner holds the choice.
        PanelState::Note { .. } if root.app.sessions.gv_conflict() => {
            super::note::conflict(root, window, cx)
        }
        PanelState::Note { .. } => super::note::editor(root, true, window, cx),
    };
    let lines = root.app.sessions.gv_panel_footer();
    let footer = footer::lines(
        "notes-panel-footer",
        lines,
        Rc::new(|root, press, _, _| root.app.sessions.gv_panel_press(press)),
        cx,
    );
    div()
        .flex()
        .flex_col()
        .size_full()
        .p(M)
        .bg(c.page)
        .child(head)
        .children(status)
        .child(kit::rule().my(S))
        .child(div().flex_1().min_h_0().child(body))
        .when(!footer.is_empty(), |panel| {
            panel.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .pt(S)
                    .border_t(theme::HAIRLINE_WIDTH)
                    .border_color(c.hairline)
                    .children(footer),
            )
        })
        .into_any_element()
}
