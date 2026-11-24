//! The run strip above every other page while a run is open (egui `Splat::run_strip`,
//! `design::TurnStrip`): the round, whose turn it is and who is next; Next turn and Back to run;
//! and what needs the DM: Not saved, a stopped sync, a sync error or a conflict.
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::{Page, party};
use gpui::{
    AnyElement, ClickEvent, Context, InteractiveElement, IntoElement, ParentElement, Styled,
    Window, div,
};
use theme::{Density, Type, TypeExt, colors};

/// The strip.
pub(crate) fn render_strip(
    root: &mut SplatRoot,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let Some(run) = root.app.encounters.run() else {
        return div().into_any_element();
    };
    let strip = run.strip();
    let title = if strip.running {
        format!("Round {}", strip.round)
    } else {
        format!("{} · {}", run.state_words(false), strip.name)
    };
    let mut line = div()
        .id("run-strip")
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::S)
        .w_full()
        .h(Density::Glance.control() + theme::S)
        .flex_none()
        .px(theme::M)
        .py(theme::XS)
        .bg(c.current_row)
        .child(div().flex_none().ty(Type::Heading).text_color(c.text).child(title));
    if strip.running {
        let turn = strip.current.clone().unwrap_or_else(|| "?".to_owned());
        line = line.child(
            div()
                .min_w_0()
                .ty(Type::Name)
                .text_color(c.turn.current)
                .truncate()
                .child(format!("{turn}\u{2019}s turn")),
        );
        if let Some(next) = &strip.next {
            line = line.child(
                div()
                    .min_w_0()
                    .ty(Type::Small)
                    .text_color(c.turn.next)
                    .truncate()
                    .child(format!("Next: {next}")),
            );
        }
    }
    line = line.child(div().flex_1());
    if strip.running {
        line = line.child(
            kit::Button::primary("run-strip-next-turn", "Next turn")
                .icon(icons::NEXT)
                .density(Density::Read)
                .on_click(cx.listener(|root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut() {
                        run.next_turn();
                    }
                    root.app
                        .sessions
                        .after_a_step_back_to_note(root.app.notes_gave_way);
                    cx.notify();
                })),
        );
    }
    line = line.child(
        kit::Button::secondary("run-strip-back", "Back to run")
            .density(Density::Read)
            .on_click(cx.listener(|root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                root.show_page(Page::Run, cx);
            })),
    );
    if strip.not_saved {
        line = line.child(kit::status(&kit::Status::new(
            icons::ERROR,
            "Not saved",
            c.document.invalid,
        )));
    }
    // The rail says `Sync stopped` on every page, as a link to Party; the strip does not say
    // it again.
    if strip.sync_stopped && !root.app.party.sync_stopped {
        line = line.child(kit::status(&super::status(&party::SyncStopped::rail())));
    }
    if let Some(name) = strip
        .sync_error
        .as_ref()
        .filter(|_| !strip.sync_stopped && strip.conflict.is_none())
    {
        line = line.child(kit::status(&kit::Status::new(
            icons::ERROR,
            format!("Sync error: {name}"),
            c.result.error,
        )));
    }
    if let Some(name) = &strip.conflict {
        let full = format!("Sync conflict: {name}");
        line = line.child(
            kit::status_link(
                "run-strip-conflict",
                kit::Status::new(icons::WARN, full.clone(), c.sync.conflict),
            )
            .hint(full)
            .on_click(cx.listener(|root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                if let Some(run) = root.app.encounters.run_mut() {
                    run.review_conflict();
                }
                root.show_page(Page::Run, cx);
            })),
        );
    }
    line.into_any_element()
}
