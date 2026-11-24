//! The Session folder block (`Sessions::folder_block`), for the Campaign page's Link and folder
//! tab: the folder's name, where the notes are now, Save folder with its discard, and the
//! offer to move the files there.
use crate::gui::{SplatRoot, kit, theme};
use crate::sessions::page::FolderAct;
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div};
use theme::{FORM_WIDTH, S, Type, XS, colors};

/// The block. `locked`: why the campaign's link waits, and with it this block.
pub(crate) fn render_folder_block(
    root: &mut SplatRoot,
    locked: Option<&str>,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let app = &mut root.app;
    let Some(view) = app.sessions.gv_folder_block(&app.timeline, locked) else {
        return div().into_any_element();
    };
    let field = kit::TextField::new("session-folder", view.text.clone())
        .name("Folder")
        .disabled(locked.is_some())
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.sessions.gv_set_folder_text(words);
            cx.notify();
        }));
    let now = view.now.as_ref().map(|folder| {
        div()
            .flex()
            .flex_col()
            .gap(XS)
            .child(kit::help(format!("Now: {}", folder.display())))
            .child(kit::help(
                "Notes, Timeline.md, Templates and Recordings are all here.",
            ))
    });
    let save = kit::Button::secondary("session-folder-save", "Save folder")
        .blocked(view.waits.clone().or(view
            .elsewhere
            .then(|| "Choose below to move the files or not.".to_owned())))
        .on_click(cx.listener(|root, _, _, cx| {
            let app = &mut root.app;
            app.sessions.gv_folder_act(&app.timeline, FolderAct::Save);
            cx.notify();
        }));
    let discard = view.changed.then(|| {
        kit::Button::quiet("session-folder-discard", "Discard").on_click(cx.listener(
            |root, _, _, cx| {
                root.app.sessions.gv_discard_folder_text();
                cx.notify();
            },
        ))
    });
    let messages = view.messages.into_iter().enumerate().map(|(at, message)| {
        let mut shown = kit::Message::new(super::footer::tone(message.tone), message.words);
        for (words, act, blocked) in message.choices {
            shown = shown.action(
                kit::Button::secondary(
                    gpui::ElementId::Name(format!("session-folder-{at}-{words}").into()),
                    words,
                )
                .density(theme::Density::Read)
                .blocked(blocked)
                .on_click(cx.listener(move |root, _, _, cx| {
                    let app = &mut root.app;
                    app.sessions.gv_folder_act(&app.timeline, act);
                    cx.notify();
                })),
            );
        }
        shown
    });
    div()
        .flex()
        .flex_col()
        .gap(S)
        .w(FORM_WIDTH)
        .max_w_full()
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
        ))
        .children(now)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(save)
                .children(discard),
        )
        .children(
            (view.changed && locked.is_none())
                .then_some(view.waits)
                .flatten()
                .map(|why| kit::text(Type::Small, c.muted, why)),
        )
        .children(messages)
        .into_any_element()
}
