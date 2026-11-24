//! Campaign → Rules and runs (egui: `Encounters::run_settings`): the run folder, how NPCs roll
//! initiative, ties in roster order, the inspiration rule when there are several, the house
//! rules in play, then the save row; the tab's footer holds the discard's Undo, the unread rule
//! choices, the runs a folder change leaves with their two choices, and the errors.
use crate::gui::kit::{self, Button, Checkbox, Dropdown, Pane, TextField};
use crate::gui::pages::settings;
use crate::gui::theme::{FORM_WIDTH, L, S, Type, XS, colors};
use crate::gui::{SplatRoot, bridge};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*,
};

/// `Roll NPC initiative`'s help (`INITIATIVE_HELP`).
const INITIATIVE_HELP: &str = "Run setup starts with this choice. You can change it there for one roll. Groups that arrive later use it too.";
/// The house rules' help (`HOUSE_RULES_HELP`).
const HOUSE_RULES_HELP: &str = "Trackers (Ctrl+T) shows the trackers of each note in play.";

pub(super) fn render(
    root: &mut SplatRoot,
    _: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    {
        let SplatRoot { app, bridge, .. } = root;
        app.encounters
            .run_settings_prepare(bridge.ctx(), &app.campaigns);
    }
    let Some(active) = root.app.campaigns.active_campaign() else {
        return div().into_any_element();
    };
    let Some(form) = root.app.encounters.run_form() else {
        return kit::icon(crate::gui::icons::BUSY, Type::Body, c.connection.busy)
            .into_any_element();
    };
    let closed = root
        .app
        .encounters
        .draft_dirty()
        .then_some("Save or discard the open encounter first.");
    let lines = {
        let SplatRoot { app, bridge, .. } = root;
        app.encounters
            .run_settings_lines(bridge.ctx(), &active, &app.campaigns)
    };
    let row = root.app.encounters.run_row(&active);
    let mut column = div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(FORM_WIDTH)
        .pt(S)
        .pb(L);
    let id = |name: &str| ElementId::Name(SharedString::from(format!("{name}-{}", active.id)));
    column = column.child(kit::field(
        "Run folder",
        Some("Relative to the campaign folder, or an absolute path.".into()),
        None,
        TextField::new(id("run_folder"), form.folder.clone())
            .name("Run folder")
            .disabled(closed.is_some())
            .on_change(cx.listener(|root, words: &str, _, cx| {
                root.app.encounters.set_run_folder(words);
                cx.notify();
            })),
    ));
    column = column.child(kit::field(
        "Roll NPC initiative",
        Some(INITIATIVE_HELP.into()),
        None,
        Dropdown::new(
            id("run_initiative"),
            "Roll NPC initiative",
            form.groupings.clone(),
            Some(form.grouping),
        )
        .width(FORM_WIDTH)
        .blocked(closed)
        .on_select(cx.listener(|root, at: &usize, _, cx| {
            root.app.encounters.set_grouping(*at);
            cx.notify();
        })),
    ));
    let ties = form.roster_ties;
    column = column.child(
        div().pb(L).child(
            Checkbox::new(
                id("run_ties"),
                "Break initiative ties in roster order",
                ties,
            )
            .blocked(closed)
            .on_click(cx.listener(move |root, _, _, cx| {
                root.app.encounters.set_roster_ties(!ties);
                cx.notify();
            })),
        ),
    );
    if let Some((lines, chosen)) = form.inspiration.clone() {
        column = column.child(kit::field(
            "Inspiration rule",
            None,
            None,
            Dropdown::new(id("run_inspiration"), "Inspiration rule", lines, chosen)
                .placeholder("Inspiration rule")
                .width(FORM_WIDTH)
                .blocked(closed)
                .on_select(cx.listener(|root, at: &usize, _, cx| {
                    root.app.encounters.set_inspiration(*at);
                    cx.notify();
                })),
        ));
        if form.inspiration_unset {
            column = column.child(kit::help(
                "This campaign’s packages have more than one inspiration rule. Runs offer inspiration once you choose one.",
            ));
        }
    }
    if !form.house_rules.is_empty() {
        let mut switches = div().flex().flex_col().gap(XS);
        for (at, (note, name, on)) in form.house_rules.into_iter().enumerate() {
            switches = switches.child(
                Checkbox::new(("run-house-rule", at), name, on)
                    .blocked(closed)
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.encounters.set_in_play(note, !on);
                        cx.notify();
                    })),
            );
        }
        column = column.child(kit::field(
            "House rules in play",
            Some(HOUSE_RULES_HELP.into()),
            None,
            switches,
        ));
    }
    if let Some(row) = row {
        let primary = row.state.save_weight() == crate::design::save_row::SaveWeight::Primary;
        let reason = row.state.save_blocked().map(str::to_owned);
        let save_active = active.clone();
        let save = if primary {
            Button::primary("run-settings-save", "Save run settings")
        } else {
            Button::secondary("run-settings-save", "Save run settings")
        }
        .key("Ctrl+S")
        .blocked(reason.clone())
        .on_click(cx.listener(move |root, _, _, cx| {
            let SplatRoot { app, bridge, .. } = root;
            app.encounters
                .save_run_settings(bridge.ctx(), &save_active, &app.campaigns);
            cx.notify();
        }));
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .py(XS)
                .child(kit::status(&bridge::status(&row.state.status())))
                .when(row.changed && closed.is_none(), |line| {
                    line.child(Button::quiet("run-settings-discard", "Discard").on_click(
                        cx.listener(|root, _, _, cx| {
                            root.app.encounters.discard_run_settings();
                            cx.notify();
                        }),
                    ))
                })
                .child(div().flex_1())
                .child(save),
        );
        if row.changed {
            column = column.children(super::super::settings::campaign_list::blocked_reason(
                reason.as_deref(),
            ));
        }
    }
    let press_active = active.clone();
    let footer = kit::messages::lines(
        "run-settings-footer",
        lines,
        settings::on_press(cx, move |root, press, cx| {
            let SplatRoot { app, bridge, .. } = root;
            app.encounters
                .run_settings_press(bridge.ctx(), &press_active, &app.campaigns, press);
            cx.notify();
        }),
    );
    let mut pane = Pane::new("run-settings").flush().child(column);
    for line in footer {
        pane = pane.footer(line);
    }
    pane.into_any_element()
}
