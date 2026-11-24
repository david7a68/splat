//! The run's own panel beside the roster (egui `view::side_panel`, `setup_panel`, `panel_tabs`,
//! `panel_body`): the selected participant's name or `This fight · …`, the tabs, the death
//! saves and a sync conflict, then the tab's body: the stat block or sheet with its action
//! tiles, Conditions, Correct, Log, Prep, Keys, or Arrivals.
use super::{on_command, on_run};
use crate::encounter_run::{Command, MAX_FROM_SHEET, Panel, Participant, Phase, Run, view};
use splat_party::combat_sync::SyncState;
use crate::gui::{SplatRoot, bridge, icons, kit, theme};
use crate::rolls::{self, ArmedAction, Ask, Mode};
use gpui::{
    AnyElement, ClickEvent, Context, ElementId, InteractiveElement, IntoElement, ParentElement,
    Pixels, SharedString, Styled, div, prelude::*, px,
};
use theme::{Density, Type, TypeExt, colors};

/// The panel, `width` wide, with its gutter and hairline on the left.
pub(super) fn render(
    root: &SplatRoot,
    run: &Run,
    width: Pixels,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let fight = matches!(
        run.panel,
        Panel::Log | Panel::Encounter | Panel::Groups | Panel::Keys
    );
    let index = if run.panel == Panel::Reference {
        run.panel_row()
    } else {
        run.selected
    };
    let Some(p) = run.state.participants.get(index) else {
        return div().into_any_element();
    };
    let title = if fight {
        fight_title(run.panel).to_owned()
    } else {
        p.name.clone()
    };
    let is_sheet = p.definition.fields.contains_key("splat_external");
    let mut top = div()
        .flex()
        .flex_col()
        .flex_none()
        .child(
            div()
                .id("run-panel-title")
                .ty(Type::Title)
                .text_color(c.text)
                .truncate()
                .child(title.clone())
                .with_tooltip_words(title),
        )
        .child(tabs(run, if is_sheet { "Sheet" } else { "Stat block" }, cx))
        .child(kit::rule());
    if run.state.phase == Phase::Setup {
        top = top.children(sync_block(run, cx));
    }
    let mut body = div().flex().flex_col().w_full().gap(theme::XS);
    if !fight && p.shows_death_saves() {
        body = body.child(death_saves(run, index, cx));
    }
    if !fight
        && let Some(sync) = run.syncs.get(&index)
        && sync.state() == SyncState::Conflict
    {
        body = body.child(conflicts(index, None, sync, cx));
    }
    body = match run.panel {
        Panel::Reference => body.child(reference(root, run, index, cx)),
        Panel::Conditions => body.child(conditions(run, index, cx)),
        Panel::Record => body
            .children(changes_block(p))
            .child(record(run, index, cx)),
        Panel::Log => body.child(log(run)),
        Panel::Encounter => body.child(prep(run, cx)),
        Panel::Keys => body.child(keys(run)),
        Panel::Groups => body.child(arrivals(run, cx)),
    };
    div()
        .flex()
        .flex_col()
        .h_full()
        .w(width)
        .flex_none()
        .pl(theme::L)
        .border_l(theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .child(top)
        .child(
            div()
                .flex_1()
                .min_h_0()
                .pt(theme::S)
                .child(kit::scroll_column("run-panel-body", body)),
        )
        .into_any_element()
}

/// `This fight · Log` and the other fight tabs' titles.
pub(super) fn fight_title(panel: Panel) -> &'static str {
    match panel {
        Panel::Log => "This fight · Log",
        Panel::Encounter => "This fight · Prep",
        Panel::Groups => "This fight · Arrivals",
        Panel::Keys => "This fight · Keys",
        _ => "This fight",
    }
}

/// The participant's tabs, then the fight's (`panel_tabs`).
fn tabs(run: &Run, reference: &'static str, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    const PARTICIPANT: [Panel; 3] = [Panel::Reference, Panel::Conditions, Panel::Record];
    const FIGHT: [Panel; 2] = [Panel::Log, Panel::Encounter];
    let chosen = |list: &[Panel]| {
        list.iter()
            .position(|panel| *panel == run.panel)
            .unwrap_or(usize::MAX)
    };
    let participant = kit::Segmented::new(
        "run-panel-tabs",
        [reference, "Conditions", "Correct"],
        chosen(&PARTICIPANT),
    )
    .on_select(cx.listener(|root: &mut SplatRoot, at: &usize, _, cx| {
        if let Some(run) = root.app.encounters.run_mut() {
            run.panel = PARTICIPANT[*at];
        }
        cx.notify();
    }));
    let fight = kit::Segmented::new("run-panel-fight-tabs", ["Log", "Prep"], chosen(&FIGHT))
        .on_select(cx.listener(|root: &mut SplatRoot, at: &usize, _, cx| {
            if let Some(run) = root.app.encounters.run_mut() {
                run.panel = FIGHT[*at];
            }
            cx.notify();
        }));
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::L)
        .child(participant)
        .child(fight)
}

/// Death saves: the counts, `+ Success`, `+ Failure` and a roll (`death_saves`).
fn death_saves(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let p = &run.state.participants[index];
    let (successes, failures) = (p.successes, p.failures);
    let name = p.name.clone();
    div()
        .flex()
        .flex_col()
        .child(kit::heading(
            kit::Heading::Group {
                title: "Death saves",
                count: 0,
            },
            true,
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::XS)
                .min_h(Density::Read.control())
                .child(
                    div()
                        .w(px(150.0))
                        .flex_none()
                        .child(kit::text(Type::Small, c.text, view::death_save_words(successes, failures))),
                )
                .child(
                    kit::Button::quiet("run-death-success", "+ Success")
                        .name("Add a death save success")
                        .density(Density::Read)
                        .blocked((successes >= 3).then_some("Three successes are recorded."))
                        .on_click(on_command(cx, Command::DeathSaves(index, successes + 1, failures))),
                )
                .child(
                    kit::Button::quiet("run-death-failure", "+ Failure")
                        .name("Add a death save failure")
                        .density(Density::Read)
                        .blocked((failures >= 3).then_some("Three failures are recorded."))
                        .on_click(on_command(cx, Command::DeathSaves(index, successes, failures + 1))),
                )
                .child(
                    kit::Button::quiet("run-death-roll", "Roll")
                        .icon(icons::DICE)
                        .name("Roll a death save")
                        .density(Density::Read)
                        .on_click(on_run(cx, move |run| {
                            run.roll_link(Ask {
                                link: splat::content::dice_text::RollLink::d20(
                                    splat::content::dice_text::D20::DeathSave,
                                    0,
                                    "",
                                ),
                                occurrence: None,
                                mode: Mode::Normal,
                                roller: Some(index),
                                entry: name.clone(),
                                inspired: false,
                            });
                        })),
                ),
        )
}

/// A sync conflict's choices for one participant (`CombatSync::conflicts`).
fn conflicts(
    index: usize,
    name: Option<&str>,
    sync: &splat_party::combat_sync::CombatSync,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let mut block = div().flex().flex_col().gap(theme::XS);
    for row in sync.conflict_rows() {
        let title = match name {
            Some(name) => format!("{name} · {}", row.title),
            None => row.title.clone(),
        };
        let (use_key, keep_key) = (row.key.clone(), row.key.clone());
        block = block
            .child(kit::status(&kit::Status::new(icons::WARN, title, c.sync.conflict)))
            .when(name.is_none(), |block| {
                block.child(kit::text(Type::Small, c.text, row.message.clone()))
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(value_line("Splat", row.splat.clone()))
                    .child(value_line("D&D Beyond", row.beyond.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(theme::XS)
                    .child(
                        kit::Button::secondary(
                            ElementId::Name(format!("run-conflict-use-{index}-{}", row.key).into()),
                            row.use_label.clone(),
                        )
                        .name("Use D&D Beyond values")
                        .hint(row.use_label.clone())
                        .density(Density::Read)
                        .blocked(row.busy)
                        .on_click(on_run(cx, move |run| {
                            if let Some(sync) = run.syncs.get_mut(&index) {
                                sync.use_remote(&use_key);
                            }
                        })),
                    )
                    .child(
                        kit::Button::secondary(
                            ElementId::Name(format!("run-conflict-keep-{index}-{}", row.key).into()),
                            row.keep_label.clone(),
                        )
                        .name("Keep Splat values and retry")
                        .hint(row.keep_label.clone())
                        .density(Density::Read)
                        .blocked(row.keep_blocked)
                        .on_click(on_run(cx, move |run| {
                            if let Some(sync) = run.syncs.get_mut(&index) {
                                sync.keep_local(&keep_key);
                            }
                        })),
                    ),
            );
    }
    block.child(kit::gap(theme::S))
}

fn value_line(label: &'static str, value: String) -> impl IntoElement {
    let c = colors();
    div()
        .flex()
        .flex_row()
        .gap(theme::L)
        .child(div().w(px(80.0)).child(kit::text(Type::Small, c.muted, label)))
        .child(kit::text(Type::Strong, c.text, value))
}

/// `D&D Beyond sync` in setup: each character whose sync has not settled (`sync_block`).
fn sync_block(run: &Run, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let c = colors();
    let names: Vec<(usize, String)> = run
        .syncs
        .iter()
        .filter(|(_, sync)| sync.enabled && sync.unconfirmed())
        .filter_map(|(index, _)| Some((*index, run.state.participants.get(*index)?.name.clone())))
        .collect();
    if names.is_empty() {
        return None;
    }
    let any_conflict = names
        .iter()
        .any(|(index, _)| run.syncs.get(index).is_some_and(|sync| sync.state() == SyncState::Conflict));
    let mut block = div()
        .flex()
        .flex_col()
        .gap(theme::XS)
        .child(kit::heading(
            kit::Heading::Group {
                title: "D&D Beyond sync",
                count: 0,
            },
            false,
        ))
        .child(kit::text(
            Type::Small,
            c.muted,
            if any_conflict {
                "Splat and D&D Beyond differ. Choose which values to keep."
            } else {
                "Splat and D&D Beyond differ."
            },
        ));
    for (index, name) in names {
        let Some(sync) = run.syncs.get(&index) else {
            continue;
        };
        block = block.child(kit::gap(theme::S));
        if sync.state() == SyncState::Conflict {
            block = block.child(conflicts(index, Some(&name), sync, cx));
        } else {
            let status = super::roster::sync_status(sync.state());
            block = block.child(kit::status(&kit::Status::new(
                status.icon.clone(),
                format!("{name} · {}", status.word),
                status.color,
            )));
            if sync.state() == SyncState::Pending {
                block = block.child(kit::help(view::pending_words(sync)));
            }
        }
        block = block.child(
            kit::Button::quiet(
                ElementId::Name(format!("run-stop-sync-{index}").into()),
                format!("Stop sync for {name}"),
            )
            .density(Density::Read)
            .line_start()
            .hint("Sync stops, and changes not yet confirmed are not sent")
            .on_click(on_run(cx, move |run| {
                if let Some(sync) = run.syncs.get_mut(&index) {
                    sync.stop();
                    run.dirty = true;
                }
            })),
        );
    }
    Some(block.child(kit::rule()).into_any_element())
}

/// What the encounter or group changed on this creature (`changes_block`): its origin line.
fn changes_block(p: &Participant) -> Option<AnyElement> {
    let c = colors();
    let origin = view::change_origin(p)?;
    Some(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::XS)
            .pb(theme::S)
            .child(kit::icon(icons::CHANGED, Type::Small, c.participant.changed))
            .child(kit::text(Type::Small, c.participant.changed, origin))
            .into_any_element(),
    )
}

/// AC, HP and initiative in columns (`vital_columns`).
fn vitals(columns: &[(&'static str, String)]) -> impl IntoElement {
    let c = colors();
    div()
        .flex()
        .flex_row()
        .gap(theme::XL)
        .py(theme::S)
        .children(columns.iter().map(|(label, value)| {
            div()
                .flex()
                .flex_col()
                .child(kit::text(Type::Label, c.text, *label))
                .child(kit::text(Type::Stat, c.text, value.clone()))
        }))
}

fn display(value: Option<i64>) -> String {
    value.map_or_else(|| "—".to_owned(), |n| n.to_string())
}

/// The stat block or sheet tab (`reference_tab`): the loot on the card, the starting snapshot's
/// notice, the values, the action tiles and the rules text.
fn reference(root: &SplatRoot, run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let definition = &p.definition;
    let mut column = div().flex().flex_col().gap(theme::XS).w_full();
    column = column.children(super::ended::card_loot(run, index, cx));
    if definition.path == "Made up in this fight" {
        let unknown = |value: Option<i64>| value.map_or_else(|| "Unknown".to_owned(), |n| n.to_string());
        return column
            .child(kit::text(Type::Body, c.muted, "Made up in this fight. It has no stat block."))
            .child(vitals(&[
                ("ARMOR CLASS", unknown(p.armor_class)),
                (
                    "HIT POINTS",
                    splat_party::combat_sync::fraction(unknown(p.hp), unknown(p.max_hp)),
                ),
                ("INITIATIVE", unknown(p.initiative)),
            ]))
            .into_any_element();
    }
    if let Some(record) = run.sheet.clone().filter(|_| run.sheet_for == Some(index)) {
        let freshness = crate::party::freshness(&record, run.connected);
        let ready = run.syncs.get(&index).is_some_and(|sync| sync.ready());
        let (word, age) = if ready {
            ("Connected".to_owned(), String::new())
        } else {
            let (word, age) = freshness
                .word
                .split_once(" · ")
                .unwrap_or((&freshness.word, ""));
            (word.to_owned(), age.to_owned())
        };
        let facts = if ready {
            " · Sync reads this sheet · HP from this run".to_owned()
        } else {
            format!(" · {age} · HP from this run")
        };
        column = column
            .children(changes_block(p))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(theme::XS)
                    .child(kit::status(&kit::Status::new(
                        freshness.icon,
                        word,
                        bridge::color(freshness.color),
                    )))
                    .child(kit::text(Type::Small, c.muted, facts)),
            )
            .child(vitals(&[
                ("ARMOR CLASS", display(p.armor_class)),
                (
                    "HIT POINTS",
                    splat_party::combat_sync::fraction(display(p.hp), display(p.max_hp)),
                ),
                ("INITIATIVE", display(p.initiative)),
            ]))
            .child(tiles(run, index, cx));
        return column.into_any_element();
    }
    // A stat block whose file changed, went or cannot be read since the run copied it.
    let mut notice = None;
    if let Some(content) = &run.content {
        let at = content.position_of(&splat::content::Reference::Id(definition.id));
        let current = at.map(|at| &content.items[at].entry);
        let unreadable = content
            .errors
            .iter()
            .any(|error| error.contains(&definition.path));
        notice = view::snapshot_notice(definition, current, unreadable)
            .map(|words| (words.to_string(), unreadable, at));
    }
    if definition.fields.contains_key("splat_external") {
        column = column.child(kit::text(
            Type::Small,
            c.document.read_only,
            view::snapshot_values_line(notice.is_some()),
        ));
    }
    let speed = p.changes.as_ref().and_then(|changes| {
        changes
            .overrides
            .rows(&changes.base)
            .into_iter()
            .find(|row| row.label == "Speed")
            .map(|row| row.changed)
    });
    column = column
        .children(changes_block(p))
        .child(vitals(&[
            ("ARMOR CLASS", display(p.armor_class)),
            (
                "HIT POINTS",
                splat_party::combat_sync::fraction(display(p.hp), display(p.max_hp)),
            ),
            (
                "INITIATIVE",
                p.initiative_bonus
                    .map_or_else(|| "—".to_owned(), crate::design::signed),
            ),
        ]))
        .children(speed.map(|speed| kit::text(Type::Small, c.participant.changed, format!("Speed {speed}"))))
        .child(tiles(run, index, cx))
        .child(
            kit::markdown::Markdown::new(
                ElementId::Name(format!("run-stat-block-{}", p.key).into()),
                definition.body.clone(),
            )
            .compact(),
        );
    if let Some((words, unreadable, at)) = notice {
        let tone = if unreadable {
            kit::Tone::Error
        } else {
            kit::Tone::Warning
        };
        let mut message = kit::Message::new(tone, words).small();
        if let Some(at) = at {
            message = message.action(
                kit::Button::secondary("run-read-new-version", "Read the new version")
                    .density(Density::Read)
                    .on_click(on_run(cx, move |run| run.lookup_request = Some(at))),
            );
        }
        column = column.child(kit::gap(theme::S)).child(message);
    }
    let _ = root;
    column.into_any_element()
}

/// The participant's actions as tiles: a press arms an attack, a save or damage, or rolls a
/// check at once; Ctrl+1 to 9 arm them too. Under the armed tile: what to do next and its
/// actions (`rolls::tiles`, `armed_view`).
fn tiles(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let chips = run.chips(index);
    if chips.is_empty() {
        return div().into_any_element();
    }
    let armed = run.armed_view().filter(|_| run.panel_row() == index);
    let name = run.state.participants[index].name.clone();
    let mut column = div().flex().flex_col().gap(theme::XS).pt(theme::S).child(kit::heading(
        kit::Heading::Section("Actions"),
        true,
    ));
    for (at, chip) in chips.into_iter().enumerate() {
        let (detail, facts) = rolls::tile_words(&chip);
        let marked = armed.as_ref().is_some_and(|view| {
            view.link == chip.link
                && view
                    .occurrence
                    .is_none_or(|armed| chip.spell_occurrence == Some(armed))
        });
        let (link, occurrence, entry) = (chip.link.clone(), chip.spell_occurrence, name.clone());
        let tile = div()
            .id(ElementId::Name(format!("run-tile-{index}-{at}").into()))
            .flex()
            .flex_col()
            .gap(px(2.0))
            .p(theme::S)
            .rounded(theme::RADIUS_CONTROL)
            .border(if marked { theme::FOCUS_WIDTH } else { theme::HAIRLINE_WIDTH })
            .border_color(if marked { c.accent } else { c.field_border })
            .cursor_pointer()
            .hover(|s| s.bg(c.hover))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(theme::XS)
                    .child(kit::icon(tile_icon(&chip.link), Type::Body, c.text))
                    .child(div().flex_1().min_w_0().child(kit::text::one_line(Type::Strong, c.text, tile_title(&chip))))
                    .when(at < 9, |line| {
                        line.child(kit::text(Type::Small, c.faint, format!("Ctrl+{}", at + 1)))
                    }),
            )
            .child(kit::text(Type::Small, c.text, detail))
            .children(facts.map(|facts| kit::text(Type::Small, c.muted, facts)))
            .on_click(cx.listener(move |root: &mut SplatRoot, event: &ClickEvent, _, cx| {
                if let Some(run) = root.app.encounters.run_mut() {
                    let modifiers = event.modifiers();
                    let d20 = matches!(link, splat::content::dice_text::RollLink::D20 { .. });
                    let mode = if d20 && modifiers.shift {
                        Mode::Advantage
                    } else if d20 && modifiers.secondary(cx) {
                        Mode::Disadvantage
                    } else {
                        Mode::Normal
                    };
                    run.roll_link(Ask {
                        link: link.clone(),
                        occurrence,
                        mode,
                        roller: Some(index),
                        entry: entry.clone(),
                        inspired: false,
                    });
                }
                cx.notify();
            }));
        let tile = kit::WithTooltip::with_tooltip(tile, chip.text.clone());
        column = column.child(kit::target(tile, chip.label.clone()));
        if marked && let Some(view) = &armed {
            column = column.child(armed_line(view, cx));
        }
    }
    column.into_any_element()
}

/// The tile's title: the action's name, without its numbers.
fn tile_title(chip: &rolls::Chip) -> String {
    rolls::link_label(&chip.link).to_owned()
}

fn tile_icon(link: &splat::content::dice_text::RollLink) -> &'static str {
    use splat::content::dice_text::RollLink;
    match link {
        RollLink::D20 { .. } => icons::ATTACK,
        RollLink::SaveDc { .. } => icons::SAVE_DC,
        RollLink::Damage { .. } | RollLink::Dice { .. } => icons::AREA,
    }
}

/// Under the armed tile: what a click on a row does now, and the armed action's choices.
fn armed_line(view: &rolls::ArmedView, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let mut actions = div().flex().flex_row().flex_wrap().gap(theme::XS);
    for (at, (words, action)) in view.actions.iter().enumerate() {
        let action: ArmedAction = *action;
        actions = actions.child(
            kit::Button::quiet(ElementId::Name(format!("run-armed-{at}").into()), *words)
                .density(Density::Read)
                .on_click(on_run(cx, move |run| run.armed_action(action))),
        );
    }
    div()
        .flex()
        .flex_col()
        .gap(theme::XS)
        .pl(theme::S)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::XS)
                .child(kit::icon(icons::ARMED, Type::Small, c.accent))
                .child(kit::text(Type::Small, c.text, view.status.clone())),
        )
        .child(actions)
}

/// The action tiles as one line above the dock while Lookup or Notes hide the panel
/// (`compact_actions`): `Death saves` first where the panel has them.
pub(super) fn compact_actions(run: &Run, cx: &mut Context<SplatRoot>) -> AnyElement {
    let row = run.panel_row();
    let chips = run.chips(row);
    let death_saves = run.state.participants[run.selected].shows_death_saves();
    let name = run.state.participants[row].name.clone();
    let armed = run.armed_view();
    let mut line = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .h(px(rolls::BAR_BUTTONS + 4.0))
        .flex_none()
        .overflow_hidden();
    if death_saves {
        line = line.child(
            kit::Button::quiet("run-compact-death-saves", "Death saves")
                .density(Density::Read)
                .on_click(on_run(cx, |run| run.close_lookup = true)),
        );
    }
    for (at, chip) in chips.into_iter().enumerate() {
        let marked = armed.as_ref().is_some_and(|view| view.link == chip.link);
        let (link, occurrence, entry) = (chip.link.clone(), chip.spell_occurrence, name.clone());
        line = line.child(
            kit::Button::quiet(ElementId::Name(format!("run-compact-{at}").into()), chip.label.clone())
                .icon(tile_icon(&chip.link))
                .density(Density::Read)
                .selected(marked)
                .hint(chip.text.clone())
                .on_click(on_run(cx, move |run| {
                    run.roll_link(Ask {
                        link: link.clone(),
                        occurrence,
                        mode: Mode::Normal,
                        roller: Some(row),
                        entry: entry.clone(),
                        inspired: false,
                    });
                })),
        );
    }
    line.into_any_element()
}

/// The Conditions tab (`conditions_tab`): the official conditions as toggles, the Exhaustion
/// level, the other effects with their Remove, the `Other effect…` field and its help.
fn conditions(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let mut grid = div().flex().flex_row().flex_wrap().gap(theme::XS).max_w(theme::FORM_WIDTH);
    for (_, name) in splat_party::combat_sync::CONDITIONS.iter() {
        let label = view::condition_label(p, name);
        let selected = label.is_some();
        let shown = label.unwrap_or_else(|| (*name).to_owned());
        let command = view::condition_press(p, index, name);
        grid = grid.child(
            div().w(px(170.0)).flex_none().child(
                kit::Button::quiet(ElementId::Name(format!("run-condition-{name}").into()), shown)
                    .icon(if selected { icons::OK } else { icons::ADD })
                    .name(*name)
                    .density(Density::Read)
                    .selected(selected)
                    .on_click(on_command(cx, command)),
            ),
        );
    }
    let mut column = div().flex().flex_col().gap(theme::XS).child(grid);
    // One Exhaustion level, a stepper: each step replaces the last.
    if let Some(at) = p
        .conditions
        .iter()
        .position(|c| c.name().starts_with("Exhaustion"))
    {
        let level = i64::from(view::exhaustion_level(p.conditions[at].name()));
        let down = if level <= 1 {
            Command::RemoveCondition(index, at)
        } else {
            view::set_exhaustion(index, (level - 1) as u8)
        };
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::S)
                .child(kit::text(Type::Body, c.text, "Exhaustion level"))
                .child(
                    kit::IconButton::new("run-exhaustion-down", icons::SUBTRACT, "Lower exhaustion")
                        .on_click(on_command(cx, down)),
                )
                .child(kit::text(Type::Strong, c.text, level.to_string()))
                .child(
                    kit::IconButton::new("run-exhaustion-up", icons::ADD, "Raise exhaustion")
                        .on_click(on_run(cx, move |run| {
                            if level >= view::EXHAUSTION_MAX {
                                run.error = Some("Exhaustion stops at 6.".into());
                            } else {
                                run.run_command(view::set_exhaustion(index, (level + 1) as u8));
                            }
                        })),
                ),
        );
    }
    let counts = true;
    let custom = view::other_effects(p, counts);
    if !custom.is_empty() {
        column = column.child(kit::heading(
            kit::Heading::Group {
                title: "Other effects",
                count: 0,
            },
            false,
        ));
        for (at, words) in custom {
            column = column.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(theme::S)
                    .min_h(Density::Read.control())
                    .child(div().flex_1().min_w_0().child(kit::text(Type::Body, c.text, words)))
                    .child(
                        kit::Button::quiet(ElementId::Name(format!("run-effect-remove-{at}").into()), "Remove")
                            .density(Density::Read)
                            .on_click(on_command(cx, Command::RemoveCondition(index, at))),
                    ),
            );
        }
    } else {
        column = column.child(kit::gap(theme::S));
    }
    let key = p.key.clone();
    let draft = run
        .effect_fields
        .get(&key)
        .map(|field| field.words.clone())
        .unwrap_or_default();
    let placeholder = if run.state.phase == Phase::Running && !run.effect_in_dock {
        view::OTHER_EFFECT_TIMED
    } else {
        "Other effect…"
    };
    let add = {
        let key = key.clone();
        move |root: &mut SplatRoot, cx: &mut Context<SplatRoot>| {
            if let Some(run) = root.app.encounters.run_mut()
                && let Some(index) = run.state.participants.iter().position(|p| p.key == key)
            {
                let typed = run
                    .effect_fields
                    .get(&key)
                    .map(|field| field.words.trim().to_owned())
                    .unwrap_or_default();
                if !typed.is_empty() {
                    let command = view::other_effect_command(&run.state.participants[index], index, typed);
                    run.run_command(command);
                    if run.error.is_none() {
                        run.effect_fields.remove(&key);
                        run.focus_amount = run.state.phase == Phase::Running;
                    }
                }
            }
            cx.notify();
        }
    };
    let add_click = add.clone();
    let typing = key.clone();
    column = column
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::XS)
                .max_w(theme::FORM_WIDTH)
                .child(
                    div().key_context("RunField").flex_1().child(
                        kit::TextField::new(ElementId::Name(format!("run-other-effect-{key}").into()), draft.clone())
                            .placeholder(placeholder)
                            .name("Other effect")
                            .focus(run.focus_other_effect)
                            .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                                if let Some(run) = root.app.encounters.run_mut() {
                                    run.effect_fields.entry(typing.clone()).or_default().words =
                                        words.to_owned();
                                    run.focus_other_effect = false;
                                }
                                cx.notify();
                            }))
                            .on_enter(super::plain(cx, move |root, _, cx| add(root, cx))),
                    ),
                )
                .child(
                    kit::Button::secondary("run-other-effect-add", "Add")
                        .density(Density::Read)
                        .blocked(draft.trim().is_empty().then_some("Type an effect first."))
                        .on_click(cx.listener(move |root: &mut SplatRoot, _: &ClickEvent, _, cx| add_click(root, cx))),
                ),
        )
        .child(kit::help(view::CONDITIONS_HELP));
    column.into_any_element()
}

/// The Correct tab (`record_tab`): HP, initiative, the DM's notes and the sheet's sync.
fn record(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let sheet_decides = run.syncs.get(&index).is_some_and(|sync| sync.ready());
    let mut column = div().flex().flex_col().gap(theme::S).max_w(theme::FORM_WIDTH);
    column = column
        .child(kit::heading(
            kit::Heading::Group {
                title: "Correct HP",
                count: 0,
            },
            true,
        ))
        .child(number_row(
            "Current HP",
            ElementId::Name(format!("run-correct-hp-{}", p.key).into()),
            p.hp,
            false,
            cx,
            move |run, value| {
                let max = run.state.participants[index].max_hp;
                Command::Health(index, value, max)
            },
        ))
        .child(number_row(
            "Maximum HP",
            ElementId::Name(format!("run-correct-max-{}", p.key).into()),
            p.max_hp,
            sheet_decides,
            cx,
            move |run, value| {
                let hp = run.state.participants[index].hp;
                Command::Health(index, hp, value)
            },
        ))
        .when(sheet_decides, |column| column.child(kit::help(MAX_FROM_SHEET)))
        .child(kit::help(
            "You resolve conditions, Concentration, death saves and recovery. Zero HP does not remove a turn.",
        ))
        .child(kit::heading(
            kit::Heading::Group {
                title: "Correct initiative",
                count: 0,
            },
            false,
        ));
    if run.state.phase == Phase::Setup {
        column = column.child(number_row(
            "Initiative bonus",
            ElementId::Name(format!("run-correct-bonus-{}", p.key).into()),
            p.initiative_bonus,
            false,
            cx,
            move |_, value| Command::Bonus(index, value),
        ));
    } else {
        let (words, color) = if p.initiative.is_none() {
            (
                "No initiative yet: last in the order until you set it. The current turn stays.",
                c.result.warning,
            )
        } else if run.current() == Some(index) {
            (
                "The turn stays. If the new total moves this row, it moves when its turn ends.",
                c.muted,
            )
        } else {
            ("This row moves. The current turn stays.", c.muted)
        };
        let text = run.setup_inputs.get(&index).cloned().unwrap_or_default();
        let total = crate::design::parse_signed(&text);
        let set = move |root: &mut SplatRoot, cx: &mut Context<SplatRoot>| {
            if let Some(run) = root.app.encounters.run_mut()
                && let Some(total) = run.setup_inputs.get(&index).and_then(|text| crate::design::parse_signed(text))
            {
                if run.state.participants[index].initiative != Some(total) {
                    run.run_command(Command::Initiative(index, Some(total)));
                }
                run.setup_inputs.remove(&index);
                run.return_focus = run.lookup_open;
            }
            cx.notify();
        };
        column = column
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(theme::XS)
                    .child(div().w(px(110.0)).child(kit::text(Type::Small, c.muted, "Initiative")))
                    .child(
                        div().key_context("RunField").w(px(72.0)).child(
                            kit::TextField::new(ElementId::Name(format!("run-correct-initiative-{}", p.key).into()), text)
                                .placeholder("Total")
                                .name(format!("Initiative for {}", p.name))
                                .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                                    if let Some(run) = root.app.encounters.run_mut() {
                                        run.setup_inputs.insert(index, super::number_text(words));
                                    }
                                    cx.notify();
                                }))
                                .on_enter(super::plain(cx, move |root, _, cx| set(root, cx))),
                        ),
                    )
                    .child(
                        kit::Button::secondary("run-set-initiative", "Set initiative")
                            .density(Density::Read)
                            .blocked(total.is_none().then_some("Type a number first."))
                            .on_click(cx.listener(move |root: &mut SplatRoot, _: &ClickEvent, _, cx| set(root, cx))),
                    ),
            )
            .child(kit::text(Type::Small, color, words));
    }
    // The DM's notes on this participant.
    let empty = run.note.trim().is_empty();
    let note_text = run.note.clone();
    column = column
        .child(kit::heading(
            kit::Heading::Group {
                title: "DM notes",
                count: 0,
            },
            false,
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::XS)
                .child(div().w(px(110.0)).child(kit::text(Type::Small, c.muted, "Note")))
                .child(
                    div().key_context("RunField").flex_1().child(
                        kit::TextField::new(ElementId::Name(format!("run-note-{}", p.key).into()), note_text.clone())
                            .placeholder("What happened")
                            .name("DM note")
                            .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                                if let Some(run) = root.app.encounters.run_mut() {
                                    run.note = words.to_owned();
                                }
                                cx.notify();
                            })),
                    ),
                ),
        )
        .child(
            div().pl(px(110.0) + theme::XS).child(
                kit::Button::secondary("run-add-note", "Add note")
                    .density(Density::Read)
                    .blocked(empty.then_some("Type a note first."))
                    .on_click(on_run(cx, move |run| {
                        let note = run.note.clone();
                        run.run_command(Command::Note(index, note));
                    })),
            ),
        );
    if !p.notes.is_empty() {
        column = column
            .child(kit::text(Type::Small, c.muted, "Notes so far"))
            .child(kit::text(Type::Body, c.text, p.notes.clone()));
    }
    if p.definition.fields.contains_key("splat_external") {
        column = column
            .child(kit::heading(
                kit::Heading::Group {
                    title: "D&D Beyond combat sync",
                    count: 0,
                },
                false,
            ))
            .child(sync_settings(run, index, cx));
    }
    column.into_any_element()
}

/// One number of Correct: its label and a field that applies each change as a step.
fn number_row(
    label: &'static str,
    id: ElementId,
    value: Option<i64>,
    locked: bool,
    cx: &mut Context<SplatRoot>,
    make: impl Fn(&Run, Option<i64>) -> Command + 'static,
) -> impl IntoElement {
    let c = colors();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .child(div().w(px(110.0)).child(kit::text(Type::Small, c.muted, label)))
        .child(
            div().key_context("RunField").w(px(96.0)).child(
                kit::TextField::new(id, value.map_or_else(String::new, |n| n.to_string()))
                    .name(label)
                    .disabled(locked)
                    .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                        if let Some(run) = root.app.encounters.run_mut() {
                            let typed = super::number_text(words);
                            let parsed = crate::design::parse_signed(&typed);
                            if typed.is_empty() || parsed.is_some() {
                                let command = make(run, parsed);
                                run.run_command(command);
                            }
                        }
                        cx.notify();
                    })),
            ),
        )
}

/// A sheet's sync in Correct (`CombatSync::show`): its switch, Check now and its state. The
/// sheet's own values (HP, rests, slots, purse) are edited on the Party page.
fn sync_settings(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let connected = run.connected || run.simulate_connection;
    let sync = run.syncs.get(&index);
    let enabled = sync.is_some_and(|sync| sync.enabled);
    let mut column = div().flex().flex_col().gap(theme::XS);
    if !connected {
        return column
            .child(
                kit::Checkbox::new("run-sync-sheet", "Sync combat stats with D&D Beyond", enabled)
                    .blocked(Some("Connect to sync")),
            )
            .child(
                kit::Button::quiet("run-sync-connect", "Connect to sync")
                    .plain()
                    .line_start()
                    .density(Density::Read)
                    .on_click(on_run(cx, |run| run.wants_sign_in = true)),
            )
            .into_any_element();
    }
    column = column
        .child(
            kit::Checkbox::new("run-sync-sheet", "Sync this sheet with D&D Beyond", enabled).on_click(
                on_run(cx, move |run| {
                    let sync = run.syncs.entry(index).or_default();
                    if sync.enabled {
                        sync.stop();
                    } else {
                        sync.start();
                    }
                    run.dirty = true;
                }),
            ),
        )
        .child(kit::help(
            "During a fight, Sync party in the run turns sync on for the whole party.",
        ));
    let Some(sync) = sync.filter(|sync| sync.enabled) else {
        if let Some(error) = sync.and_then(|sync| sync.error.clone()) {
            column = column.child(kit::text(Type::Small, c.result.error, error));
        }
        return column
            .child(kit::help(
                "Enable to read the live sheet. Edits made here will then update D&D Beyond.",
            ))
            .into_any_element();
    };
    column = column.child(kit::help("Splat checks the sheet every 15 seconds."));
    if sync.busy() {
        column = column.child(kit::text(Type::Small, c.muted, "Checking character…"));
    }
    if let Some(error) = &sync.error {
        column = column.child(kit::text(Type::Small, c.result.error, error.clone()));
    }
    column = column.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::S)
            .child(
                kit::Button::secondary("run-sync-check-now", "Check now")
                    .density(Density::Read)
                    .blocked(sync.busy().then_some("Wait for the current sheet read."))
                    .on_click(on_run(cx, move |run| {
                        if let Some(sync) = run.syncs.get_mut(&index) {
                            sync.check_now();
                        }
                    })),
            )
            .child(kit::text(
                Type::Small,
                c.muted,
                if sync.dirty() {
                    "Local changes pending"
                } else if sync.has_read() {
                    "Values match the last read"
                } else {
                    ""
                },
            )),
    );
    if sync.state() == SyncState::Conflict {
        column = column.child(conflicts(index, None, sync, cx));
    }
    if !sync.has_read() {
        column = column.child(kit::text(
            Type::Small,
            c.muted,
            "If the first sheet read does not come, check the connection in Settings.",
        ));
    } else if !sync.can_edit() {
        column = column.child(kit::text(
            Type::Small,
            c.sync.read_only,
            "Read only: D&D Beyond reports that this sign-in cannot edit this character. Nothing here can change.",
        ));
    }
    column
        .child(kit::help(
            "Concentration, custom effects, initiative and DM notes stay in Splat. D&D Beyond has no place for them.",
        ))
        .into_any_element()
}

/// The Log tab, newest first (`log_lines`).
pub(super) fn log(run: &Run) -> AnyElement {
    let c = colors();
    if run.state.log.is_empty() {
        return kit::empty_state(
            icons::SESSIONS,
            "No log entries yet",
            "Begin combat, or add a note on the Correct tab, to record what happens.",
            None::<AnyElement>,
        )
        .into_any_element();
    }
    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .children(
            run.state
                .log
                .iter()
                .rev()
                .map(|line| kit::text(Type::Small, c.text, line.clone())),
        )
        .into_any_element()
}

/// The Prep tab (`prep_notes`): the encounter's read-aloud text and notes, or how to add them.
pub(super) fn prep(run: &Run, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let (prep, technical) = run.prep();
    let mut column = div().flex().flex_col().gap(theme::XS);
    if prep.is_empty() {
        column = match run.source {
            Some(id) => {
                let action = div()
                    .flex()
                    .flex_col()
                    .child(
                        kit::Button::quiet("run-edit-encounter", "Edit encounter")
                            .line_start()
                            .on_click(cx.listener(move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                                root.bridge.ctx().data_mut(|data| {
                                    data.insert_temp(eframe::egui::Id::new("edit_run_encounter"), id);
                                });
                                if let Some(run) = root.app.encounters.run_mut() {
                                    run.prep_refusal = None;
                                }
                                cx.notify();
                            })),
                    )
                    .children(
                        run.prep_refusal
                            .as_ref()
                            .map(|refusal| kit::text(Type::Small, c.muted, refusal.reason().to_owned())),
                    );
                column.child(kit::empty_state(
                    icons::SESSIONS,
                    "No prep notes yet",
                    "Add read-aloud text or notes when you edit the encounter.",
                    Some(action.into_any_element()),
                ))
            }
            None => column.child(kit::empty_state(
                icons::SESSIONS,
                "No prep notes",
                "This fight did not start from a saved encounter, so it has no prep notes.",
                None::<AnyElement>,
            )),
        };
    } else {
        column = column
            .child(kit::label(view::READ_ALOUD, 0))
            .child(kit::markdown::Markdown::new("run-prep", prep.to_owned()).compact());
    }
    if let Some(technical) = technical {
        let technical = technical.to_owned();
        column = column.child(kit::gap(theme::M)).child(super::ended::section(
            "run-prep-technical",
            "Technical details",
            move || kit::markdown::Markdown::new("run-prep-technical-body", technical.clone()).compact().into_any_element(),
            cx,
        ));
    }
    column.into_any_element()
}

/// The Keys tab: the run's keyboard commands.
fn keys(run: &Run) -> AnyElement {
    let c = colors();
    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .pt(theme::XS)
        .children(view::KEYS.iter().map(|(key, what)| {
            div()
                .flex()
                .flex_row()
                .items_start()
                .child(div().w(px(96.0)).flex_none().child(kit::text(Type::Heading, c.text, *key)))
                .child(div().flex_1().min_w_0().child(kit::text(Type::Body, c.muted, *what)))
        }))
        .child(kit::gap(theme::XS))
        .child(kit::text(
            Type::Small,
            c.muted,
            if run.lookup_open {
                "Letters do nothing while Notes or Lookup is open. Use the buttons."
            } else {
                "Letters work when the amount has focus, or nothing has."
            },
        ))
        .into_any_element()
}

/// Arrivals (`groups_tab`): a creature to find or make up, then the campaign's encounter groups
/// with their Add.
fn arrivals(run: &Run, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let mut column = div().flex().flex_col().gap(theme::XS).max_w(theme::LIST_MEASURE);
    if run.groups.as_ref().is_some_and(|groups| !groups.is_empty()) {
        column = column.child(kit::help("Use Add to bring a group into the fight."));
    }
    column = column.child(creature_form(run, cx)).child(kit::rule());
    let Some(groups) = run.groups.clone() else {
        return column
            .child(kit::label("Encounter groups", 0))
            .child(kit::text(
                Type::Body,
                c.muted,
                "This campaign’s groups have not loaded yet. If this stays, the Encounters page says why.",
            ))
            .into_any_element();
    };
    if groups.is_empty() {
        return column
            .child(kit::label("Encounter groups", 0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(theme::XS)
                    .child(kit::text(Type::Small, c.muted, "No groups yet. Make one on Encounters with"))
                    .child(
                        kit::Button::quiet("run-new-group", "New group")
                            .plain()
                            .density(Density::Read)
                            .on_click(on_run(cx, |run| run.wants_new_group = true)),
                    ),
            )
            .into_any_element();
    }
    let offered = crate::encounter_run::OfferedGroups::of(&groups, run.source);
    let in_plan = |group: &std::sync::Arc<crate::encounter_run::RunGroup>| {
        run.planned
            .iter()
            .any(|name| name.eq_ignore_ascii_case(&group.name))
    };
    let (planned, any): (Vec<_>, Vec<_>) = offered.any.iter().cloned().partition(in_plan);
    let labelled = !offered.own.is_empty() || !planned.is_empty() || !offered.other.is_empty();
    if !labelled {
        column = column.child(kit::label("Encounter groups", offered.any.len()));
    }
    for (heading, list) in [
        ("For this encounter", offered.own.as_slice()),
        ("In tonight’s plan", planned.as_slice()),
        ("For any encounter", any.as_slice()),
        ("For one encounter", offered.other.as_slice()),
    ] {
        if labelled && !list.is_empty() {
            column = column.child(kit::heading(
                kit::Heading::Group {
                    title: heading,
                    count: list.len(),
                },
                false,
            ));
        }
        if heading == "For one encounter" && !list.is_empty() {
            column = column.child(kit::help(
                "This run does not say which encounter it came from, so the groups of every encounter show.",
            ));
        }
        for group in list {
            column = column.child(group_row(run, group, cx)).child(kit::rule());
        }
    }
    column.into_any_element()
}

/// One encounter group in Arrivals: its name and numbers, its status and members, an
/// initiative total and Add.
fn group_row(run: &Run, group: &crate::encounter_run::RunGroup, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let id = group.id;
    let status = run.group_status(group, run.group_joined(group));
    let members: Vec<String> = group
        .members
        .iter()
        .map(|member| {
            if member.count > 1 {
                format!("{} × {}", member.count, member.entry.name)
            } else {
                member.entry.name.clone()
            }
        })
        .collect();
    let blocked = run.add_waits_words(group);
    let total = run.group_totals.get(&id).cloned().unwrap_or_default();
    let numbers = format!(
        "{} · {} · {}",
        crate::design::reader::human_label(&group.side),
        group.count_label(),
        group.xp_label()
    );
    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .py(theme::XS)
        .child(
            div()
                .flex()
                .flex_row()
                .items_baseline()
                .gap(theme::S)
                .child(div().min_w_0().child(kit::text::one_line(Type::Strong, c.text, group.name.clone())))
                .child(kit::text::one_line(Type::Small, c.muted, numbers)),
        )
        .when(!status.is_empty(), |row| row.child(kit::text(Type::Small, c.muted, status.clone())))
        .child(kit::text(Type::Small, c.muted, members.join(", ")))
        .children(group.problem.clone().map(|problem| kit::text(Type::Small, c.result.warning, problem)))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::XS)
                .child(kit::text(Type::Small, c.muted, "Initiative"))
                .child(
                    div().key_context("RunField").w(px(72.0)).child(
                        kit::TextField::new(ElementId::Name(format!("run-arrival-total-{id}").into()), total)
                            .placeholder("Roll")
                            .name(format!("Initiative total for {}", group.name))
                            .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                                if let Some(run) = root.app.encounters.run_mut() {
                                    run.group_totals.insert(id, words.to_owned());
                                    run.totals_revision += 1;
                                }
                                cx.notify();
                            })),
                    ),
                )
                .child(
                    kit::Button::secondary(ElementId::Name(format!("run-arrival-add-{id}").into()), "Add")
                        .icon(icons::ADD)
                        .name(format!("Add {}", group.name))
                        .density(Density::Read)
                        .blocked(blocked.clone())
                        .on_click(on_run(cx, move |run| run.add_group_from_panel(id))),
                ),
        )
        .children(blocked.map(|why| kit::text(Type::Small, c.muted, why)))
}

/// Find or make up a creature (`add_creature_form`).
fn creature_form(run: &Run, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let mut column = div().flex().flex_col().gap(theme::XS);
    if let Some(picked) = run.ad_hoc.picked.clone() {
        column = column
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(div().flex_1().min_w_0().child(kit::text::one_line(Type::Strong, c.text, picked.entry.name.clone())))
                    .child(
                        kit::Button::quiet("run-creature-change", "Change")
                            .name("Choose another creature")
                            .density(Density::Read)
                            .on_click(on_run(cx, |run| run.ad_hoc = Default::default())),
                    ),
            )
            .child(kit::text(Type::Small, c.muted, picked.detail.clone()));
    } else {
        column = column.child(
            div().key_context("RunField").child(
                kit::TextField::new("run-creature-search", run.ad_hoc.query.clone())
                    .placeholder("Find or name a creature…")
                    .name("Find or name a creature")
                    .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                        if let Some(run) = root.app.encounters.run_mut() {
                            run.ad_hoc.query = words.to_owned();
                            let typed = words.trim().to_lowercase();
                            if run.ad_hoc.show_all.as_deref() != Some(typed.as_str()) {
                                run.ad_hoc.show_all = None;
                            }
                        }
                        cx.notify();
                    })),
            ),
        );
        let (matches, shown) = run.creature_matches();
        let typed = run.ad_hoc.query.trim().to_lowercase();
        for creature in matches.iter().take(shown) {
            let picked = creature.clone();
            column = column.child(
                kit::two_line_row(
                    ElementId::Name(format!("run-creature-{}", creature.entry.name).into()),
                    creature.entry.name.clone(),
                    creature.detail.clone(),
                    creature.lower == typed,
                )
                .on_click(on_run(cx, move |run| run.ad_hoc.picked = Some(picked.clone()))),
            );
        }
        if shown < matches.len() {
            let all = typed.clone();
            column = column.child(
                kit::Button::quiet("run-creature-show-all", format!("Show all {}", matches.len()))
                    .density(Density::Read)
                    .hint("List every creature that matches, without more typing")
                    .on_click(on_run(cx, move |run| run.ad_hoc.show_all = Some(all.clone()))),
            );
        }
        if typed.is_empty() {
            return column.into_any_element();
        }
    }
    let add = run.creature_add();
    if add.made_up {
        column = column.child(kit::text(Type::Small, c.muted, "Made up here. It stays in this fight."));
    }
    let field = |id: &'static str, label: &'static str, name: &'static str, value: String, hint: String, width: f32, focus: bool, set: fn(&mut crate::encounter_run::ad_hoc::AdHoc, String), cx: &mut Context<SplatRoot>| {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::XS)
            .child(kit::text(Type::Small, c.muted, label))
            .child(
                div().key_context("RunField").w(px(width)).child(
                    kit::TextField::new(id, value)
                        .placeholder(hint)
                        .name(name)
                        .focus(focus)
                        .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                            if let Some(run) = root.app.encounters.run_mut() {
                                set(&mut run.ad_hoc, words.to_owned());
                                run.ad_hoc.focus_hp = false;
                            }
                            cx.notify();
                        }))
                        .on_enter(super::plain(cx, |root, _, cx| {
                            if let Some(run) = root.app.encounters.run_mut() {
                                run.add_creature_from_form();
                            }
                            cx.notify();
                        })),
                ),
            )
    };
    let form = &run.ad_hoc;
    column = column
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(theme::M)
                .child(field("run-creature-count", "Count", "New creature count", form.count.clone(), "1".into(), 52.0, false, |form, words| form.count = words, cx))
                .child(field("run-creature-hp", "HP", "New creature HP", form.hp.clone(), add.hp_hint.clone(), 64.0, form.focus_hp, |form, words| form.hp = words, cx))
                .child(field("run-creature-ac", "AC", "New creature AC", form.ac.clone(), add.ac_hint.clone(), 52.0, false, |form, words| form.ac = words, cx)),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(theme::M)
                .child(field("run-creature-total", "Initiative", "New creature initiative total", form.total.clone(), "Roll".into(), 52.0, false, |form, words| form.total = words, cx))
                .child(side_choice(run, cx)),
        )
        .children(add.blocked.clone().map(|why| kit::text(Type::Small, c.muted, why)))
        .child(
            kit::Button::secondary("run-creature-add", add.label.clone())
                .icon(icons::ADD)
                .name(format!("Add {} to the initiative list", add.name))
                .density(Density::Read)
                .blocked(add.blocked.clone())
                .on_click(on_run(cx, |run| run.add_creature_from_form())),
        );
    column.into_any_element()
}

/// The made-up creature's side.
fn side_choice(run: &Run, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let sides = run.sides_to_join();
    let chosen = sides.iter().position(|side| side == run.ad_hoc.side());
    let words: Vec<SharedString> = sides
        .iter()
        .map(|side| crate::design::reader::human_label(side).into())
        .collect();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .child(kit::text(Type::Small, c.muted, "Side"))
        .child(
            kit::Dropdown::new("run-creature-side", "Side of the new creature", words, chosen)
                .width(px(110.0))
                .density(Density::Read)
                .on_select(cx.listener(move |root: &mut SplatRoot, at: &usize, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut()
                        && let Some(side) = run.sides_to_join().get(*at).cloned()
                    {
                        run.ad_hoc.side = Some(side);
                    }
                    cx.notify();
                })),
        )
}

/// Hover words on an element with an id.
trait TooltipWords: kit::WithTooltip {
    fn with_tooltip_words(self, words: impl Into<SharedString>) -> Self {
        self.with_tooltip(words)
    }
}
impl<T: kit::WithTooltip> TooltipWords for T {}
