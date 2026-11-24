//! The Stat block tab (egui `reference_tab`'s stat block, `view::show_run`): the Reader's entry
//! view in run mode, so the run's creature reads as it does in the Library and Lookup, with the
//! run's live AC, HP and initiative over the printed ones, and its actions as tiles that arm
//! (Ctrl+1 to 9) and roll for its row. Above it: the loot on the card, a sheet's read-only line
//! and what the encounter or group changed. Under it, pinned at the panel's bottom: a snapshot
//! whose file changed, went or cannot be read, with `Read the new version`.
use crate::encounter_run::{Participant, Run, view};
use crate::gui::pages::reader::entry::{self, EntryView, Source};
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::library::view::{NoLinks, RunValues};
use crate::rolls::{self, ArmedView};
use eframe::egui;
use gpui::{
    AnyElement, App, Context, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Styled, Window, div,
};
use std::rc::Rc;
use theme::{Type, colors};

/// Where the run's copy of a stat block comes from (egui's `Origin` in `reference_tab`).
const OWNER: &str = "Starting snapshot, saved with this run";

/// The tab's body for a stat block (not a sheet, and not a creature made up in the fight).
pub(super) fn render(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let definition = &p.definition;
    let mut column = div().flex().flex_col().gap(theme::XS).w_full();
    column = column.children(super::ended::card_loot(run, index, cx));
    if definition.fields.contains_key("splat_external") {
        let frame = crate::design::outcome::PaneFrame::plain();
        let (_, notice) = run.reference_messages(&frame, index, 0.0);
        column = column.child(kit::text(
            Type::Small,
            c.document.read_only,
            view::snapshot_values_line(notice),
        ));
    }
    column = column.children(changes(p, true));
    let links = run.content.clone().map_or_else(
        || Rc::new(NoLinks) as Rc<dyn crate::library::view::Links>,
        entry::index_links,
    );
    let speed = p.changes.as_ref().and_then(|changes| {
        changes
            .overrides
            .rows(&changes.base)
            .into_iter()
            .find(|row| row.label == "Speed")
            .map(|row| row.changed)
    });
    let values = RunValues {
        armor_class: p.armor_class,
        hp: p.max_hp,
        current_hp: p.hp,
        initiative: p.initiative_bonus,
        speed,
    };
    let view = EntryView::new(
        ElementId::Name(format!("run-stat-block-{}", p.key).into()),
        definition.name.clone(),
        Some(definition),
        definition.body.clone(),
        Source {
            path: definition.path.clone(),
            owner: OWNER.to_owned(),
            builtin: false,
        },
        links,
    )
    .run(values)
    .on_open({
        let open = cx.listener(|root: &mut SplatRoot, at: &usize, _, cx| {
            // A followed related, feature or spell link opens in Lookup beside the run.
            if let Some(run) = root.app.encounters.run_mut() {
                run.lookup_request = Some(*at);
            }
            cx.notify();
        });
        move |at, window, cx| open(&at, window, cx)
    });
    column
        .child(RunReader {
            view,
            row: index,
            armed: run.armed_view(),
            inspired: run.can_buy_advantage(run.panel_row()),
            changes: run.changes(index),
        })
        .into_any_element()
}

/// The tab's footer: the snapshot's notice and `Read the new version`, as kit messages.
pub(super) fn footer(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> Vec<AnyElement> {
    let frame = crate::design::outcome::PaneFrame::plain();
    let (messages, _) = run.reference_messages(&frame, index, 0.0);
    kit::messages::lines(
        ElementId::Name(format!("run-reference-footer-{index}").into()),
        messages.into_plain(),
        cx.listener(
            |root: &mut SplatRoot,
             pressed: &kit::messages::Pressed<view::ReferencePress>,
             _,
             cx| {
                if let Some(press) = pressed.take()
                    && let Some(run) = root.app.encounters.run_mut()
                {
                    run.reference_press(press);
                }
                cx.notify();
            },
        ),
    )
}

/// What the encounter or group changed on this creature (`changes_block`): what to add by hand
/// above a stat block, the origin, `Changed`, each change with the value it replaces, and the
/// by-hand note.
pub(super) fn changes(p: &Participant, above_stat_block: bool) -> Option<AnyElement> {
    let c = colors();
    let lines = view::change_lines(p, above_stat_block);
    let mut block = div().flex().flex_col().gap(theme::XS);
    let mut any = false;
    if let Some(words) = lines.add_by_hand {
        any = true;
        block = block.child(kit::target(
            kit::text(Type::Body, c.participant.changed, words.clone()),
            words,
        ));
    }
    if let Some(origin) = lines.origin {
        any = true;
        block = block.child(kit::text(Type::Small, c.muted, origin));
    }
    if let Some(word) = lines.status {
        any = true;
        block = block.child(kit::status(&kit::Status::new(
            icons::CHANGED,
            word,
            c.participant.changed,
        )));
    }
    if lines.gap {
        any = true;
        // What the DM must do comes first, in larger type; the rest the run has done.
        let (by_hand, applied): (Vec<_>, Vec<_>) = lines.rows.iter().partition(|row| row.by_hand);
        let separate = !by_hand.is_empty() && !applied.is_empty();
        for (at, row) in by_hand.iter().chain(&applied).enumerate() {
            if separate && at == by_hand.len() {
                block = block.child(kit::gap(theme::S));
            }
            let ty = if row.by_hand { Type::Body } else { Type::Small };
            let preview = view::change_preview(row);
            block = block.child(kit::target(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .child(kit::text(ty, c.muted, format!("{}: ", row.label)))
                    .child(kit::text(ty, c.participant.changed, preview.clone())),
                format!("{}: {preview}", row.label),
            ));
        }
        if let Some(note) = lines.note {
            block = block.child(kit::text(Type::Small, c.muted, note));
        }
        block = block.child(kit::gap(theme::S));
    }
    any.then(|| block.into_any_element())
}

/// The Reader's entry view as the run's row reads it (egui: `set_roller`, `armed_tiles_begin`
/// around `show_run`). While it builds, the row is the roller in the app's egui context: its
/// tiles show their `Ctrl+n`, the armed action and the numbers the run rolls. A press inside it
/// asks with the row as its roller, so the run rolls it (`Splat::route_rolls`); the same press in
/// Lookup stays a preview.
#[derive(IntoElement)]
struct RunReader {
    view: EntryView,
    row: usize,
    armed: Option<ArmedView>,
    inspired: bool,
    changes: (i64, i64),
}

impl RenderOnce for RunReader {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let ctx = host(cx);
        let row = self.row;
        if let Some(ctx) = &ctx {
            rolls::set_roller(ctx, Some(row));
            rolls::set_armed(ctx, self.armed);
            rolls::set_inspired(ctx, self.inspired);
            rolls::set_changes(ctx, row, self.changes);
        }
        // The entry view builds its tiles now, while the row is the roller.
        let view = self.view.render(window, cx).into_any_element();
        if let Some(ctx) = &ctx {
            rolls::set_roller(ctx, None);
            rolls::set_inspired(ctx, false);
        }
        div()
            .id("run-reader")
            .w_full()
            // Before the tile or dice link under the pointer asks (its click is a bubbling mouse
            // up), the row becomes the roller again; the next frame's poll clears it.
            .capture_any_mouse_up(move |_, _, cx| {
                if let Some(ctx) = host(cx) {
                    rolls::set_roller(&ctx, Some(row));
                }
            })
            .child(view)
    }
}

/// The app's egui context, where rolls wait for the frame logic.
fn host(cx: &App) -> Option<egui::Context> {
    cx.try_global::<kit::markdown::Host>()
        .map(|host| host.0.clone())
}

/// Each frame on the Run page, before anything draws: what was pressed under an armed tile
/// (egui `armed_tiles_end`), and no roller left from a press.
pub(super) fn poll(root: &mut SplatRoot, cx: &App) {
    let Some(ctx) = host(cx) else {
        return;
    };
    rolls::set_roller(&ctx, None);
    if let Some(action) = rolls::take_armed_action(&ctx)
        && let Some(run) = root.app.encounters.run_mut()
    {
        run.armed_action(action);
    }
}
