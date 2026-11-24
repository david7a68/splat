//! A member's history and the problems list in the reader of a house rule note in play (egui:
//! `src/trackers/history.rs`). World's reader calls `render` under the note and `footer` in its
//! pane's footer. Each tracker and each day mark of the note has a folded section with its entries
//! in the ledger's order (a tracker's says its start value first); a row with another session's
//! stamp has `Remove` in its menu, whose result and one Undo stand in the footer.
#![allow(
    dead_code,
    reason = "World’s reader of a house rule note calls these (docs/gpui-port.md)"
)]
use super::{TrackerFault, fault_message, measure, section_heading};
use crate::design::outcome::{Messages, PaneFrame};
use crate::gui::theme::{Density, S, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, kit};
use crate::trackers::history::{REMOVE_PLAYED, in_play, start_words};
use gpui::{
    AnyElement, Context, ElementId, InteractiveElement, IntoElement, ParentElement, Styled, Window,
    div, prelude::*,
};
use splat_content::content::Entry;

/// What a press in the history's footer asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HistoryPress {
    /// `Undo remove …`: the last Remove, by its line's ticket.
    UndoRemove,
}

/// The problems list and each member's folded history, under the note in its reader, or `None`
/// when the note is no house rule note in play. A file that cannot be read shows its fault and
/// no history or value.
pub(crate) fn render(
    root: &mut SplatRoot,
    note: &Entry,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let trackers = &root.app.trackers;
    if !crate::trackers::is_house_rule_note(note) || !trackers.in_play() {
        return None;
    }
    let id = note.id;
    if let Some(error) = &trackers.rules.unreadable {
        let fault = TrackerFault::NotesUnreadable(error.clone());
        return Some(div().pt(S).child(fault_message(&fault, false)).into_any_element());
    }
    if !in_play(&trackers.rules, id) {
        return None;
    }
    if let Some(error) = &trackers.unreadable {
        let fault = TrackerFault::LedgerUnreadable(error.clone());
        return Some(div().pt(S).child(fault_message(&fault, false)).into_any_element());
    }
    let c = colors();
    let mut column = div().flex().flex_col().pt(S);
    for (_, problem) in trackers
        .problems()
        .iter()
        .filter(|(owner, _)| *owner == Some(id))
    {
        column = column.child(
            div()
                .pt(XS)
                .child(fault_message(&TrackerFault::Problem(problem.clone()), true)),
        );
    }
    let rules = &trackers.rules;
    let sections: Vec<(String, String, Option<String>)> = rules
        .trackers
        .iter()
        .filter(|t| t.note == id)
        .filter_map(|t| {
            let def = t.def.as_ref().ok()?;
            Some((t.key.clone(), def.name.clone(), Some(start_words(def))))
        })
        .chain(rules.day_marks.iter().filter(|m| m.note == id).filter_map(|m| {
            Some((m.key.clone(), m.def.as_ref().ok()?.title().to_owned(), None))
        }))
        .collect();
    let played = trackers.played;
    for (key, name, start) in sections {
        let rows = trackers.history_rows(&key);
        let open = root
            .pages
            .trackers
            .history_open
            .contains(&(id, key.clone()));
        let toggle = (id, key.clone());
        column = column.child(section_heading(
            ElementId::Name(format!("tracker-history-{id}-{key}").into()),
            &format!("{name} history"),
            rows.len(),
            open,
            cx.listener(move |root, _, _, cx| {
                let open = &mut root.pages.trackers.history_open;
                if !open.remove(&toggle) {
                    open.insert(toggle.clone());
                }
                cx.notify();
            }),
        ));
        if !open {
            continue;
        }
        if let Some(start) = start {
            column = column.child(kit::text(Type::Small, c.muted, start));
        }
        for row in rows {
            let removable = row.entry.session != played;
            let entry = row.entry.clone();
            let row_id = |part: &str| {
                ElementId::Name(format!("tracker-history-{}-{part}", row.entry.id).into())
            };
            let menu = kit::MenuButton::new(
                row_id("menu"),
                kit::Button::quiet(row_id("more"), "")
                    .icon(crate::gui::icons::EXPAND)
                    .name(format!("More actions for {}", row.words))
                    .density(Density::Read),
            )
            .item(
                kit::MenuItem::new("Remove")
                    .blocked((!removable).then_some(REMOVE_PLAYED))
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.trackers.remove(id, &entry);
                        cx.notify();
                    })),
            );
            let words = div()
                .id(row_id("words"))
                .flex_1()
                .min_w_0()
                .ty(Type::Body)
                .text_color(c.text)
                .truncate()
                .child(row.words.clone())
                .tooltip(kit::tooltip::tip(row.words.clone().into()));
            column = column.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(S)
                    .h(Density::Read.control())
                    .child(words)
                    .children(row.value.map(|value| {
                        let value = value.to_string();
                        div()
                            .w(measure::width(window, Type::Numeral, &value))
                            .flex_none()
                            .ty(Type::Numeral)
                            .text_color(c.text)
                            .child(value)
                    }))
                    .child(menu),
            );
        }
    }
    Some(column.into_any_element())
}

/// The reader's footer messages of the history of `note` (`history::footer`): why the last
/// Remove or its Undo wrote nothing, and the last Remove with `Undo remove`.
pub(crate) fn footer(
    root: &mut SplatRoot,
    note: uuid::Uuid,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    let frame = PaneFrame::plain();
    let mut messages = Messages::new(&frame, ("tracker_history_footer", note), 0.0);
    crate::trackers::history::footer(
        &root.app.trackers,
        note,
        &frame,
        &mut messages,
        crate::world::said,
        HistoryPress::UndoRemove,
    );
    kit::messages::lines(
        ElementId::Name(format!("tracker-history-footer-{note}").into()),
        messages.into_plain(),
        cx.listener(|root, pressed: &kit::messages::Pressed<HistoryPress>, _, cx| {
            if let Some(press) = pressed.take() {
                match press.id {
                    HistoryPress::UndoRemove => root.app.trackers.undo_remove(press.ticket),
                }
                cx.notify();
            }
        }),
    )
}
