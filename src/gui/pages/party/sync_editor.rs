//! The combat sync sheet editor (egui `CombatSync::show`), drawn once for both places that show
//! it: the Party page's sheet (`Combat sync · On`) and the run's Correct tab (`D&D Beyond combat
//! sync`). Its switch, its state words and Check now, each conflict with its two choices, then,
//! after a read, every value Splat can send with − and +: HP, temporary HP, the maximum HP
//! modifier, death saves and inspiration; rests and recovery; the sheet's conditions; spell
//! slots and limited uses spent; the purse; the inventory; companions and summons.
//!
//! The values and their acts are the sync's own (`combat_sync::view`): a change goes half a
//! second after the DM stops. In the run, the run's frame logic takes each change into the
//! participant with a named Undo step, as egui's does.
use crate::gui::kit::{self, Button, Checkbox, Link, Section, Stepper, stepper};
use crate::gui::theme::{Density, L, READER_MEASURE, S, Type, XS, colors};
use crate::gui::{SplatRoot, icons};
use gpui::{
    AnyElement, Context, Div, ElementId, IntoElement, ParentElement, Pixels, SharedString, Styled,
    Window, div, prelude::*, px,
};
use splat_party::combat_sync::CombatSync;
use splat_party::combat_sync::view::{Editor, Field, Named, Rest, Row, Step};

/// Where the editor is, and so which sync its controls change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Host {
    /// The Party page's selected character (`Party::combat`).
    Party,
    /// The run's participant at this row (`Run::syncs`).
    Run(usize),
}

impl Host {
    /// Change this host's sync, when it has one.
    fn with(self, root: &mut SplatRoot, act: impl FnOnce(&mut CombatSync)) {
        match self {
            Host::Party => act(&mut root.app.party.combat),
            Host::Run(index) => {
                if let Some(run) = root.app.encounters.run_mut()
                    && let Some(sync) = run.syncs.get_mut(&index)
                {
                    act(sync);
                }
            }
        }
    }

    /// A control's id, apart from the other host's.
    fn id(self, part: impl std::fmt::Display) -> ElementId {
        ElementId::Name(SharedString::from(match self {
            Host::Party => format!("party-sync-{part}"),
            Host::Run(index) => format!("run-sync-{index}-{part}"),
        }))
    }

    /// `Sync this sheet with D&D Beyond`, ticked or cleared.
    fn set_enabled(self, root: &mut SplatRoot, enabled: bool) {
        match self {
            Host::Party => root.app.party.set_combat_sync(enabled),
            Host::Run(index) => {
                if let Some(run) = root.app.encounters.run_mut() {
                    // Viewing Correct is not a request to start this character's sync: a
                    // sync is kept only when the DM turns it on.
                    if enabled || run.syncs.contains_key(&index) {
                        run.syncs.entry(index).or_default().set_enabled(enabled);
                        run.dirty = true;
                    }
                }
            }
        }
    }
}

/// The whole panel. `sync` is the host's sync, or `None` for a run row that has none yet.
pub(crate) fn panel(
    host: Host,
    sync: Option<&CombatSync>,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let unstarted = CombatSync::default();
    let sync = sync.unwrap_or(&unstarted);
    let enabled = sync.enabled;
    let mut column = div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(
            Checkbox::new(
                host.id("enabled"),
                "Sync this sheet with D&D Beyond",
                enabled,
            )
            .on_click(cx.listener(move |root, _, _, cx| {
                host.set_enabled(root, !enabled);
                cx.notify();
            })),
        )
        .child(kit::help(
            "During a fight, Sync party in the run turns sync on for the whole party.",
        ));
    if !enabled {
        if let Some(error) = &sync.error {
            column = column.child(kit::text(Type::Body, c.result.error, error.clone()));
        }
        return column
            .child(kit::help(
                "Enable to read the live sheet. Edits made here will then update D&D Beyond.",
            ))
            .into_any_element();
    }
    column = column.child(kit::help("Splat checks the sheet every 15 seconds."));
    if sync.busy() {
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(kit::icon(icons::BUSY, Type::Body, c.connection.busy))
                .child(kit::text(Type::Body, c.text, "Checking character…")),
        );
    }
    if let Some(error) = &sync.error {
        column = column.child(kit::text(Type::Body, c.result.error, error.clone()));
    }
    let state = if sync.dirty() {
        Some("Local changes pending")
    } else if sync.has_remote() {
        Some("Values match the last read")
    } else {
        None
    };
    column = column.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(
                Button::secondary(host.id("check-now"), "Check now")
                    .density(Density::Read)
                    .blocked(sync.busy().then_some("Wait for the current sheet read."))
                    .on_click(cx.listener(move |root, _, _, cx| {
                        host.with(root, CombatSync::check_now);
                        cx.notify();
                    })),
            )
            .children(state.map(|words| kit::text(Type::Body, c.text, words))),
    );
    column = column.child(conflicts(host, sync, None, true, cx));
    let Some(editor) = sync.editor() else {
        // The heading says the state (`Waiting for the first sheet read`); this says the next
        // step.
        return column
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .max_w(READER_MEASURE)
                    .child(kit::text(
                        Type::Small,
                        c.muted,
                        "If the first sheet read does not come, check the connection in ",
                    ))
                    .child(Link::new(host.id("settings"), "Settings").small().on_click(
                        cx.listener(|root, _, _, cx| {
                            root.app.settings.focus_sign_in();
                            root.app.settings_tab = crate::SettingsTab::Account;
                            root.show_page(crate::Page::Settings, cx);
                        }),
                    ))
                    .child(kit::text(Type::Small, c.muted, ".")),
            )
            .into_any_element();
    };
    // Say once, at the top, which values can change and what a change does.
    let (icon, color, words) = if editor.writable {
        (
            icons::EDIT,
            c.accent,
            "Values with − and + are editable. Splat sends a change to D&D Beyond half a second after you stop, then reads the sheet to confirm it.",
        )
    } else {
        (
            icons::READ_ONLY,
            c.sync.read_only,
            "Read only: D&D Beyond reports that this sign-in cannot edit this character. Nothing here can change.",
        )
    };
    column = column.child(
        div()
            .flex()
            .flex_row()
            .items_start()
            .gap(XS)
            .child(kit::icon(icon, Type::Small, color))
            .child(div().flex_1().min_w_0().child(kit::help(words))),
    );
    let max_hp = sync.max_hp().unwrap_or(0);
    column
        .child(life(host, &editor, max_hp, window, cx))
        .child(rests(host, sync, editor.writable, window, cx))
        .child(sections(host, &editor, window, cx))
        .child(kit::help(
            "Concentration, custom effects, initiative and DM notes stay in Splat. D&D Beyond has no place for them.",
        ))
        .into_any_element()
}

/// Each conflict: its title as a warning status (after `name` in the run's form), the sheet's
/// words when `explanation`, both values, and Use and Keep (`CombatSync::conflicts`).
pub(crate) fn conflicts(
    host: Host,
    sync: &CombatSync,
    name: Option<&str>,
    explanation: bool,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let mut block = div().flex().flex_col().gap(XS);
    for row in sync.conflict_rows() {
        let title = match name {
            Some(name) => format!("{name} · {}", row.title),
            None => row.title.clone(),
        };
        let (use_key, keep_key) = (row.key.clone(), row.key.clone());
        block = block
            .child(kit::status(&kit::Status::new(
                icons::WARN,
                title,
                c.sync.conflict,
            )))
            .when(explanation, |block| {
                block.child(
                    kit::text(Type::Small, c.text, row.message.clone()).max_w(READER_MEASURE),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(value_line("Splat", row.splat.clone()))
                    .child(value_line("D&D Beyond", row.beyond.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(XS)
                    .child(
                        Button::secondary(
                            host.id(format!("conflict-use-{}", row.key)),
                            row.use_label.clone(),
                        )
                        .name("Use D&D Beyond values")
                        .hint(row.use_label.clone())
                        .density(Density::Read)
                        .blocked(row.busy)
                        .on_click(cx.listener(move |root, _, _, cx| {
                            host.with(root, |sync| sync.use_remote(&use_key));
                            cx.notify();
                        })),
                    )
                    .child(
                        Button::secondary(
                            host.id(format!("conflict-keep-{}", row.key)),
                            row.keep_label.clone(),
                        )
                        .name("Keep Splat values and retry")
                        .hint(row.keep_label.clone())
                        .density(Density::Read)
                        .blocked(row.keep_blocked)
                        .on_click(cx.listener(move |root, _, _, cx| {
                            host.with(root, |sync| sync.keep_local(&keep_key));
                            cx.notify();
                        })),
                    ),
            );
    }
    block.into_any_element()
}

/// `Splat  58/62 HP, 0 temporary HP`: a muted label and its value.
fn value_line(label: &'static str, value: String) -> Div {
    let c = colors();
    div()
        .flex()
        .flex_row()
        .items_baseline()
        .gap(L)
        .child(
            div()
                .w(px(80.0))
                .flex_none()
                .child(kit::text(Type::Small, c.muted, label)),
        )
        .child(kit::text(Type::Strong, c.text, value))
}

/// A muted capital label over a block (egui `Heading::Label`).
fn block_label(words: &'static str) -> Div {
    div()
        .pt(L)
        .pb(XS)
        .child(kit::text(Type::Label, colors().muted, words))
}

/// The well that holds the widest of `values`.
fn well(window: &Window, values: impl IntoIterator<Item = i64>) -> Pixels {
    stepper::well_for(
        window,
        values.into_iter().map(stepper::digits).max().unwrap_or(1),
    )
}

/// The label column of a grid: its widest label in `ty`, then `L` before the value.
fn label_column(window: &Window, ty: Type, labels: impl IntoIterator<Item = String>) -> Pixels {
    labels
        .into_iter()
        .map(|label| kit::measure::width(window, ty, &label))
        .fold(px(0.0), Pixels::max)
        + L
}

/// One value's stepper, its change sent to the host's sync, then its small note.
fn stepper_and_note(
    host: Host,
    step: &Step,
    well: Pixels,
    writable: bool,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let changed = step.clone();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(
            Stepper::new(
                host.id(format!("{}-{}", step.key, field_word(step.field))),
                step.name.clone(),
                step.value,
                step.range.clone(),
            )
            .well(well)
            .disabled(!writable)
            .on_change(cx.listener(move |root, value: &i64, _, cx| {
                host.with(root, |sync| sync.step(&changed, *value));
                cx.notify();
            })),
        )
        .children(step.note.clone().map(kit::help))
}

/// A field's word in a control's id.
fn field_word(field: Field) -> &'static str {
    match field {
        Field::Value(field) => field,
        Field::CurrentHp => "current",
        Field::Exhaustion => "exhaustion",
    }
}

/// One grid row: its label in the label column, then the value.
fn grid_row(label_width: Pixels, label: Option<Div>, value: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_row()
        .items_center()
        .min_h(Density::Read.control())
        .child(
            div()
                .w(label_width)
                .flex_none()
                .children(label.map(IntoElement::into_any_element)),
        )
        .child(div().flex_1().min_w_0().child(value))
}

/// A grid of steps: a `Body` label, then the stepper, every well one width.
fn step_grid(
    host: Host,
    steps: &[Step],
    writable: bool,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let well = well(window, steps.iter().map(|step| step.value));
    let labels = label_column(window, Type::Body, steps.iter().map(|s| s.label.clone()));
    let mut grid = div().flex().flex_col().gap(XS);
    for step in steps {
        grid = grid.child(grid_row(
            labels,
            Some(kit::text(Type::Body, c.text, step.label.clone())),
            stepper_and_note(host, step, well, writable, cx),
        ));
    }
    grid
}

/// HP, temporary HP, the maximum HP modifier, death saves and inspiration.
fn life(
    host: Host,
    editor: &Editor,
    max_hp: i64,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let steps = editor.life.iter().filter_map(|row| match row {
        Row::Step(step) => Some(step),
        _ => None,
    });
    let well = well(
        window,
        steps
            .clone()
            .filter(|step| !matches!(step.field, Field::Value("successCount" | "failCount")))
            .map(|step| step.value)
            .chain([max_hp]),
    );
    let labels = label_column(
        window,
        Type::Body,
        editor.life.iter().filter_map(|row| match row {
            Row::Step(step) => Some(step.label.clone()),
            Row::Inspiration(_) => Some("Inspiration".to_owned()),
            Row::Help(_) => None,
        }),
    );
    let writable = editor.writable;
    let mut grid = div().flex().flex_col().gap(XS);
    for row in &editor.life {
        grid = grid.child(match row {
            Row::Step(step) => grid_row(
                labels,
                Some(kit::text(Type::Body, c.text, step.label.clone())),
                stepper_and_note(host, step, well, writable, cx),
            ),
            Row::Help(words) => grid_row(labels, None, kit::help(*words)),
            Row::Inspiration(on) => {
                let on = *on;
                let mut check = Checkbox::new(host.id("inspiration"), "Has inspiration", on);
                if writable {
                    check = check.on_click(cx.listener(move |root, _, _, cx| {
                        host.with(root, |sync| sync.set_inspiration(!on));
                        cx.notify();
                    }));
                } else {
                    check = check.blocked(Some(READ_ONLY));
                }
                grid_row(
                    labels,
                    Some(kit::text(Type::Body, c.text, "Inspiration")),
                    check,
                )
            }
        });
    }
    div()
        .flex()
        .flex_col()
        .child(block_label("Hit points"))
        .child(grid)
}

/// Why a control of a read-only sheet does not change.
const READ_ONLY: &str = "This sign-in cannot edit this character on D&D Beyond.";

/// Short rest, long rest and restore to life. D&D Beyond applies each one itself.
fn rests(
    host: Host,
    sync: &CombatSync,
    writable: bool,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let mut block = div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(block_label("Rests and recovery"));
    let (take_label, note, reset, long) = match sync.rest() {
        Rest::Choose { blocker, restore } => {
            let mut buttons = div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(S)
                .child(
                    Button::secondary(host.id("short-rest"), "Short rest…")
                        .icon(icons::SHORT_REST)
                        .density(Density::Read)
                        .blocked(blocker)
                        .on_click(cx.listener(move |root, _, _, cx| {
                            host.with(root, |sync| sync.open_rest(false));
                            cx.notify();
                        })),
                )
                .child(
                    Button::secondary(host.id("long-rest"), "Long rest…")
                        .icon(icons::LONG_REST)
                        .density(Density::Read)
                        .blocked(blocker)
                        .on_click(cx.listener(move |root, _, _, cx| {
                            host.with(root, |sync| sync.open_rest(true));
                            cx.notify();
                        })),
                );
            // Restore to life is for a character at 0 HP, as on the sheet.
            if restore {
                for (full, label) in [
                    (true, "Restore to life with full HP"),
                    (false, "Restore to life with 1 HP"),
                ] {
                    buttons = buttons.child(
                        Button::secondary(host.id(format!("restore-{full}")), label)
                            .density(Density::Read)
                            .blocked(blocker)
                            .on_click(cx.listener(move |root, _, _, cx| {
                                host.with(root, |sync| {
                                    let _ = sync.restore_life(full);
                                });
                                cx.notify();
                            })),
                    );
                }
            }
            return block.child(buttons).child(kit::help(
                "A rest goes to D&D Beyond once, when you take it. D&D Beyond resets the resources by its own rules.",
            ));
        }
        Rest::Short {
            dice,
            reset_max_hp_bonus,
            blocker,
        } => {
            let well = stepper::well_for(window, 2);
            let labels = label_column(
                window,
                Type::Body,
                dice.iter().map(|(_, s)| s.label.clone()),
            );
            let mut grid = div().flex().flex_col().gap(XS);
            for (hit, step) in dice {
                let class = hit.class;
                grid = grid.child(grid_row(
                    labels,
                    Some(kit::text(Type::Body, c.text, step.label.clone())),
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(S)
                        .child(
                            Stepper::new(
                                host.id(format!("rest-dice-{class}")),
                                step.name.clone(),
                                step.value,
                                step.range.clone(),
                            )
                            .well(well)
                            .disabled(!writable)
                            .on_change(cx.listener(
                                move |root, value: &i64, _, cx| {
                                    host.with(root, |sync| sync.set_rest_dice(class, *value));
                                    cx.notify();
                                },
                            )),
                        )
                        .children(step.note.clone().map(kit::help)),
                ));
            }
            block = block
                .child(kit::heading(kit::Heading::Panel("Short rest"), true))
                .child(kit::text(Type::Small, c.muted, "Hit dice to spend"))
                .child(grid);
            (
                "Take short rest",
                "This marks the hit dice as spent. Add the HP that they give in Current HP.",
                (reset_max_hp_bonus, blocker),
                None,
            )
        }
        Rest::Long {
            reset_max_hp_bonus,
            reduce_exhaustion,
            blocker,
        } => {
            block = block.child(kit::heading(kit::Heading::Panel("Long rest"), true));
            (
                "Take long rest",
                "D&D Beyond restores HP, hit dice, spell slots and resources by its own rules.",
                (reset_max_hp_bonus, blocker),
                Some(reduce_exhaustion),
            )
        }
    };
    let (reset, blocker) = reset;
    block = block.child(
        Checkbox::new(
            host.id("rest-reset"),
            "Reset the maximum HP modifier to 0",
            reset,
        )
        .on_click(cx.listener(move |root, _, _, cx| {
            host.with(root, |sync| sync.set_rest_reset(!reset));
            cx.notify();
        })),
    );
    if let Some(reduce) = long {
        block = block.child(
            Checkbox::new(
                host.id("rest-exhaustion"),
                "Reduce Exhaustion by one level",
                reduce,
            )
            .on_click(cx.listener(move |root, _, _, cx| {
                host.with(root, |sync| sync.set_rest_exhaustion(!reduce));
                cx.notify();
            })),
        );
    }
    block.child(kit::help(note)).child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(
                Button::primary(host.id("take-rest"), take_label)
                    .density(Density::Read)
                    .blocked(blocker)
                    .on_click(cx.listener(move |root, _, _, cx| {
                        host.with(root, |sync| {
                            let _ = sync.take_rest();
                        });
                        cx.notify();
                    })),
            )
            .child(
                Button::quiet(host.id("cancel-rest"), "Cancel")
                    .density(Density::Read)
                    .on_click(cx.listener(move |root, _, _, cx| {
                        host.with(root, CombatSync::cancel_rest);
                        cx.notify();
                    })),
            ),
    )
}

/// The caret sections: conditions, slots, limited uses, purse, inventory, companions. Each keeps
/// whether it is open under its id (closed at first), as egui's `design::section` does.
fn sections(host: Host, editor: &Editor, window: &Window, cx: &mut Context<SplatRoot>) -> Div {
    let writable = editor.writable;
    let mut column = div().flex().flex_col().gap(XS).pt(S);
    // Sheet conditions: a box for each official condition, then the Exhaustion level on its own
    // line, so a stepper never runs off a narrow column.
    let mut conditions = Section::new(host.id("section-conditions"), "Sheet conditions", false);
    if let Some(present) = &editor.conditions {
        let mut boxes = div().flex().flex_row().flex_wrap().gap_x(L).gap_y(XS);
        for (id, name, on) in present.present.iter().copied() {
            let mut check = Checkbox::new(host.id(format!("condition-{id}")), name, on);
            if writable {
                check = check.on_click(cx.listener(move |root, _, _, cx| {
                    host.with(root, |sync| sync.set_condition(id, !on));
                    cx.notify();
                }));
            } else {
                check = check.blocked(Some(READ_ONLY));
            }
            boxes = boxes.child(check);
        }
        conditions = conditions.child(boxes);
        if let Some(step) = &present.exhaustion {
            let well = well(window, [step.value]);
            conditions = conditions.child(labelled(
                host,
                step,
                label_column(window, Type::Small, [step.label.clone()]),
                well,
                writable,
                cx,
            ));
        }
    }
    column = column.child(conditions);
    column = column.child(
        Section::new(host.id("section-slots"), "Spell slots spent", false).child(step_grid(
            host,
            &editor.slots,
            writable,
            window,
            cx,
        )),
    );
    column = column.child(
        Section::new(host.id("section-limited"), "Limited uses spent", false).child(step_grid(
            host,
            &editor.limited,
            writable,
            window,
            cx,
        )),
    );
    if let Some(coins) = &editor.currency {
        column = column.child(
            Section::new(host.id("section-currency"), "Currency", false)
                .child(step_grid(host, coins, writable, window, cx)),
        );
    }
    if editor.inventory_items > 0 {
        // The title holds the count, so the section keeps its own id as items come and go.
        column = column.child(
            Section::new(
                host.id("section-inventory"),
                format!("Inventory · {} items", editor.inventory_items),
                false,
            )
            .child(named_block(host, &editor.inventory, writable, window, cx))
            .child(kit::help(
                "Party inventory and the coins in containers stay on D&D Beyond.",
            )),
        );
    }
    if !editor.companions.is_empty() {
        column = column.child(
            Section::new(
                host.id("section-companions"),
                "Companions and summons",
                false,
            )
            .child(named_block(host, &editor.companions, writable, window, cx)),
        );
    }
    column
}

/// A small label in the block's label column, then its stepper and note, on one line.
fn labelled(
    host: Host,
    step: &Step,
    label_width: Pixels,
    well: Pixels,
    writable: bool,
    cx: &mut Context<SplatRoot>,
) -> Div {
    grid_row(
        label_width,
        Some(kit::text(Type::Small, colors().muted, step.label.clone())),
        stepper_and_note(host, step, well, writable, cx),
    )
}

/// Items or companions: each one's name on its own line, and each value on a line under it,
/// with one label column and one well for the whole block, so every − and + share an x.
fn named_block(
    host: Host,
    named: &[Named],
    writable: bool,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let c = colors();
    let steps = named.iter().flat_map(|n| n.steps.iter());
    let well = well(window, steps.clone().map(|step| step.value));
    let labels = label_column(window, Type::Small, steps.map(|step| step.label.clone()));
    let mut block = div().flex().flex_col();
    for thing in named {
        let mut values = div().flex().flex_col().gap(XS).pl(L);
        for step in &thing.steps {
            values = values.child(labelled(host, step, labels, well, writable, cx));
        }
        block = block
            .child(kit::text(Type::Body, c.text, thing.name.clone()))
            .child(values)
            .child(kit::gap(XS));
    }
    block
}
