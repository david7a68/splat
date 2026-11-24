//! The run's own panel beside the roster: the selected participant's name or `This fight · …`, the tabs, the death
//! saves and a sync conflict, then the tab's body: the stat block or sheet with its action
//! tiles, Conditions, Correct, Log, Prep, Keys, or Arrivals.
use super::{on_command, on_run};
use crate::design::outcome::plain::PlainLine;
use crate::encounter_run::{Command, MAX_FROM_SHEET, Panel, Participant, Phase, Run, view};
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::rolls::{self, ArmedAction, Ask, Mode};
use gpui::{
    AnyElement, ClickEvent, Context, ElementId, InteractiveElement, IntoElement, ParentElement,
    Pixels, SharedString, Styled, Window, div, prelude::*, px,
};
use splat_party::combat_sync::SyncState;
use std::rc::Rc;
use theme::{Density, Type, TypeExt, colors};

/// The panel, `width` wide, with its gutter and hairline on the left.
pub(super) fn render(
    root: &SplatRoot,
    run: &Run,
    width: Pixels,
    arrival_lines: Vec<PlainLine<view::ArrivalPress>>,
    window: &Window,
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
    // The panel's own room: its width less the gutter before its words.
    let inner = width - theme::L - theme::HAIRLINE_WIDTH;
    // A long name gives up whole words, keeping its copy number.
    let shown = super::measure::fit_name(window, Type::Title, &title, inner);
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
                .child(shown)
                .with_tooltip_words(title.clone()),
        )
        .child(tabs(
            run,
            if is_sheet { "Sheet" } else { "Stat block" },
            inner,
            window,
            cx,
        ))
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
    // The stat block's notice is the panel's footer, so the stat block keeps its top.
    let stat_block = run.panel == Panel::Reference
        && !(run.sheet.is_some() && run.sheet_for == Some(index))
        && p.definition.path != "Made up in this fight";
    let footer = if stat_block {
        super::stat_block::footer(run, index, cx)
    } else {
        Vec::new()
    };
    body = match run.panel {
        Panel::Reference => body.child(reference(root, run, index, cx)),
        Panel::Conditions => body.child(conditions(run, index, cx)),
        Panel::Record => body
            .children(changes_block(p))
            .child(record(run, index, window, cx)),
        Panel::Log => body.child(log(run)),
        Panel::Encounter => body.child(prep(run, cx)),
        Panel::Keys => body.child(keys(run)),
        Panel::Groups => body.child(arrivals(run, inner, window, cx)),
    };
    let footer = if run.panel == Panel::Groups {
        arrivals_footer(arrival_lines, cx)
    } else {
        footer
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
                .child(kit::scroll_column(
                    ElementId::Name(format!("run-panel-body-{:?}-{}", run.panel, p.key).into()),
                    body,
                )),
        )
        .when(!footer.is_empty(), |panel| {
            panel.child(
                div()
                    .flex()
                    .flex_col()
                    .flex_none()
                    .gap(theme::S)
                    .pt(theme::S)
                    .pb(theme::S)
                    .border_t(theme::HAIRLINE_WIDTH)
                    .border_color(c.hairline)
                    .children(footer),
            )
        })
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

/// The participant's tabs, then the fight's (`panel_tabs`). When both do not fit in `room`, the
/// fight's tabs become a `Fight` menu, underlined in the accent while one of them shows.
fn tabs(
    run: &Run,
    reference: &'static str,
    room: Pixels,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    const PARTICIPANT: [Panel; 3] = [Panel::Reference, Panel::Conditions, Panel::Record];
    const FIGHT: [(Panel, &str); 2] = [(Panel::Log, "Log"), (Panel::Encounter, "Prep")];
    let chosen = |list: &[Panel]| {
        list.iter()
            .position(|panel| *panel == run.panel)
            .unwrap_or(usize::MAX)
    };
    let participant_words = [reference, "Conditions", "Correct"];
    let width = |words: &mut dyn Iterator<Item = &str>| {
        words
            .map(|label| super::measure::width(window, Type::Strong, label) + theme::L)
            .fold(px(0.0), |sum, one| sum + one)
    };
    let fits = width(&mut participant_words.iter().copied())
        + width(&mut FIGHT.iter().map(|(_, label)| *label))
        + theme::L
        <= room;
    let participant =
        kit::Segmented::new("run-panel-tabs", participant_words, chosen(&PARTICIPANT)).on_select(
            cx.listener(|root: &mut SplatRoot, at: &usize, _, cx| {
                if let Some(run) = root.app.encounters.run_mut() {
                    run.panel = PARTICIPANT[*at];
                }
                cx.notify();
            }),
        );
    let fight: AnyElement = if fits {
        let panels: Vec<Panel> = FIGHT.iter().map(|(panel, _)| *panel).collect();
        kit::Segmented::new(
            "run-panel-fight-tabs",
            FIGHT.map(|(_, label)| label),
            chosen(&panels),
        )
        .on_select(cx.listener(|root: &mut SplatRoot, at: &usize, _, cx| {
            if let Some(run) = root.app.encounters.run_mut() {
                run.panel = FIGHT[*at].0;
            }
            cx.notify();
        }))
        .into_any_element()
    } else {
        let current = FIGHT.iter().any(|(panel, _)| *panel == run.panel);
        let mut menu = kit::MenuButton::new(
            "run-panel-fight-menu",
            kit::Button::quiet("run-panel-fight", "Fight")
                .icon(icons::DOWN)
                .name("Fight menu")
                .density(Density::Read),
        );
        for (panel, label) in FIGHT {
            menu = menu.item(
                kit::MenuItem::new(label)
                    .checked(run.panel == panel)
                    .on_click(on_run(cx, move |run| run.panel = panel)),
            );
        }
        div()
            .flex()
            .flex_col()
            .child(menu)
            .when(current, |menu| {
                menu.child(div().h(theme::HAIRLINE_WIDTH).w_full().bg(colors().accent))
            })
            .into_any_element()
    };
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::L)
        .child(div().flex_none().child(participant))
        .child(div().flex_none().child(fight))
}

/// The Arrivals footer: the newest arrival's result with a tie's `Move`, and each group's
/// reason its Add waits for (`groups_tab`'s footer, `Run::arrival_lines`). A press goes to
/// `Run::arrival_press` with its ticket: a Move whose step is no longer the newest does nothing.
fn arrivals_footer(
    lines: Vec<PlainLine<view::ArrivalPress>>,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    kit::messages::lines(
        "run-arrivals-footer",
        lines,
        cx.listener(
            |root: &mut SplatRoot, pressed: &kit::messages::Pressed<view::ArrivalPress>, _, cx| {
                if let Some(press) = pressed.take()
                    && let Some(run) = root.app.encounters.run_mut()
                {
                    run.arrival_press(press);
                }
                cx.notify();
            },
        ),
    )
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
                .child(div().w(px(150.0)).flex_none().child(kit::text(
                    Type::Small,
                    c.text,
                    view::death_save_words(successes, failures),
                )))
                .child(
                    kit::Button::quiet("run-death-success", "+ Success")
                        .name("Add a death save success")
                        .density(Density::Read)
                        .blocked((successes >= 3).then_some("Three successes are recorded."))
                        .on_click(on_command(
                            cx,
                            Command::DeathSaves(index, successes + 1, failures),
                        )),
                )
                .child(
                    kit::Button::quiet("run-death-failure", "+ Failure")
                        .name("Add a death save failure")
                        .density(Density::Read)
                        .blocked((failures >= 3).then_some("Three failures are recorded."))
                        .on_click(on_command(
                            cx,
                            Command::DeathSaves(index, successes, failures + 1),
                        )),
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
            .child(kit::status(&kit::Status::new(
                icons::WARN,
                title,
                c.sync.conflict,
            )))
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
                            ElementId::Name(
                                format!("run-conflict-keep-{index}-{}", row.key).into(),
                            ),
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
        .child(
            div()
                .w(px(80.0))
                .child(kit::text(Type::Small, c.muted, label)),
        )
        .child(kit::text(Type::Strong, c.text, value))
}

/// `D&D Beyond sync` in setup: each character whose sync has not settled (`sync_block`).
pub(super) fn sync_block(run: &Run, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
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
    let any_conflict = names.iter().any(|(index, _)| {
        run.syncs
            .get(index)
            .is_some_and(|sync| sync.state() == SyncState::Conflict)
    });
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

/// What the encounter or group changed on this creature (`changes_block`).
fn changes_block(p: &Participant) -> Option<AnyElement> {
    super::stat_block::changes(p, false)
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

/// The stat block or sheet tab (`reference_tab`): the loot on the card, the starting snapshot's
/// notice, the values, the action tiles and the rules text.
fn reference(root: &SplatRoot, run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let definition = &p.definition;
    let mut column = div().flex().flex_col().gap(theme::XS).w_full();
    column = column.children(super::ended::card_loot(run, index, cx));
    if definition.path == "Made up in this fight" {
        let unknown =
            |value: Option<i64>| value.map_or_else(|| "Unknown".to_owned(), |n| n.to_string());
        return column
            .child(kit::text(
                Type::Body,
                c.muted,
                "Made up in this fight. It has no stat block.",
            ))
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
    // A linked character shows the same sheet as the Party page, with its age.
    if let Some(record) = run.sheet.clone().filter(|_| run.sheet_for == Some(index)) {
        return column
            .child(sheet(root, run, index, &record, cx))
            .into_any_element();
    }
    let _ = root;
    super::stat_block::render(run, index, cx)
}

/// A linked character's Sheet tab (`reference_tab`): what the run changed, the sheet's age and
/// where its values come from beside its portrait, then the Party page's sheet pages
/// (`party::sheet::page_body`), whose numbers roll for this row and whose attacks are the run's
/// own tiles.
fn sheet(
    root: &SplatRoot,
    run: &Run,
    index: usize,
    record: &splat::character::EntitySnapshot,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    use crate::gui::pages::party::sheet::{Reader, page_body};
    let c = colors();
    let p = &run.state.participants[index];
    let live = splat_party::party::LiveValues {
        hp: p.hp,
        max_hp: p.max_hp,
        temporary_hp: p.temporary_hp,
        conditions: p.conditions.clone(),
        death_saves: splat::character::DeathSaves {
            successes: u64::from(p.successes),
            failures: u64::from(p.failures),
        },
        death_saves_shown: p.shows_death_saves(),
    };
    let mut column = div().flex().flex_col().gap(theme::XS).w_full();
    // The death saves are above, below the tabs; the status follows.
    if p.shows_death_saves() {
        column = column.child(kit::gap(theme::L));
    }
    column = column.children(changes_block(p));
    let freshness = crate::party::freshness(record, run.connected);
    // The sheet is older than what the DM just set here, so say which is which. While sync
    // reads this sheet, the copy's age says nothing about these values.
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
    // Beside an open conflict, green words would say the opposite of what matters.
    if !run
        .syncs
        .get(&index)
        .is_some_and(|sync| sync.state() == SyncState::Conflict)
    {
        let art = run.row_art(&root.ctx, p);
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(theme::S)
                .children(art.map(|path| {
                    kit::portrait(
                        Some(path.as_path()),
                        &record.definition.splat_name,
                        theme::XXL,
                    )
                }))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .flex_1()
                        .min_w_0()
                        .items_center()
                        .child(kit::status(&kit::Status::new(
                            freshness.icon,
                            word,
                            theme::color(freshness.color),
                        )))
                        .child(kit::text(Type::Small, c.muted, facts)),
                ),
        );
    }
    // A sheet with no spells has no Spells page.
    let spells = crate::party::has_spells(record);
    let pages: Vec<(usize, &'static str)> = crate::party::SHEET_PAGES
        .iter()
        .copied()
        .filter(|(page, _)| *page != 2 || spells)
        .collect();
    let page = if !spells && run.sheet_page == 2 {
        0
    } else {
        run.sheet_page
    };
    let chosen = pages.iter().position(|(at, _)| *at == page).unwrap_or(0);
    let numbers: Vec<usize> = pages.iter().map(|(at, _)| *at).collect();
    column = column
        .child(
            kit::Segmented::new(
                "run-sheet-pages",
                pages.iter().map(|(_, words)| *words).collect::<Vec<_>>(),
                chosen,
            )
            .on_select(cx.listener(move |root: &mut SplatRoot, at: &usize, _, cx| {
                if let Some(run) = root.app.encounters.run_mut()
                    && let Some(page) = numbers.get(*at)
                {
                    run.sheet_page = *page;
                }
                cx.notify();
            })),
        )
        .child(kit::rule());
    let reader = Reader {
        key: "run",
        search: String::new(),
        roller: Some(index),
        actions: (page == 1).then(|| tile_list(run, index, cx)),
    };
    column
        .child(page_body(root, record, page, Some(&live), reader, cx))
        .into_any_element()
}

/// A linked sheet's actions as tiles, under `Attacks` on its Actions page: a press arms an
/// attack, a save or damage, or rolls a check at once; Ctrl+1 to 9 arm them too. Under the armed
/// tile: what to do next and its actions (`rolls::tiles`, `armed_view`). A stat block's tiles are
/// the entry view's (`stat_block`).
fn tile_list(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let chips = run.chips(index);
    if chips.is_empty() {
        return div().into_any_element();
    }
    let armed = run.armed_view().filter(|_| run.panel_row() == index);
    let name = run.state.participants[index].name.clone();
    let mut column = div().flex().flex_col().gap(theme::XS);
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
            .border(if marked {
                theme::FOCUS_WIDTH
            } else {
                theme::HAIRLINE_WIDTH
            })
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
                    .child(div().flex_1().min_w_0().child(kit::text::one_line(
                        Type::Strong,
                        c.text,
                        tile_title(&chip),
                    )))
                    .when(at < 9, |line| {
                        line.child(kit::text(Type::Small, c.faint, format!("Ctrl+{}", at + 1)))
                    }),
            )
            .child(kit::text(Type::Small, c.text, detail))
            .children(facts.map(|facts| kit::text(Type::Small, c.muted, facts)))
            .on_click(
                cx.listener(move |root: &mut SplatRoot, event: &ClickEvent, _, cx| {
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
                }),
            );
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
            kit::Button::quiet(
                ElementId::Name(format!("run-compact-{at}").into()),
                chip.label.clone(),
            )
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
    let mut grid = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap(theme::XS)
        .max_w(theme::FORM_WIDTH);
    for (_, name) in splat_party::combat_sync::CONDITIONS.iter() {
        let label = view::condition_label(p, name);
        let selected = label.is_some();
        let shown = label.unwrap_or_else(|| (*name).to_owned());
        let command = view::condition_press(p, index, name);
        grid = grid.child(
            div().w(px(170.0)).flex_none().child(
                kit::Button::quiet(
                    ElementId::Name(format!("run-condition-{name}").into()),
                    shown,
                )
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
                    kit::IconButton::new(
                        "run-exhaustion-down",
                        icons::SUBTRACT,
                        "Lower exhaustion",
                    )
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
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(kit::text(Type::Body, c.text, words)),
                    )
                    .child(
                        kit::Button::quiet(
                            ElementId::Name(format!("run-effect-remove-{at}").into()),
                            "Remove",
                        )
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
                    let command =
                        view::other_effect_command(&run.state.participants[index], index, typed);
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
    let escaping = key.clone();
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
                        kit::TextField::new(
                            ElementId::Name(format!("run-other-effect-{key}").into()),
                            draft.clone(),
                        )
                        .placeholder(placeholder)
                        .name("Other effect")
                        .focus(run.focus_other_effect)
                        .on_change(
                            cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                                if let Some(run) = root.app.encounters.run_mut() {
                                    run.effect_fields.entry(typing.clone()).or_default().words =
                                        words.to_owned();
                                    run.focus_other_effect = false;
                                }
                                cx.notify();
                            }),
                        )
                        .on_enter(super::plain(cx, move |root, _, cx| add(root, cx)))
                        // Escape clears the participant's draft, as in the dock's field.
                        .on_escape(super::plain(cx, move |root, _, cx| {
                            if let Some(run) = root.app.encounters.run_mut() {
                                if let Some(field) = run.effect_fields.get_mut(&escaping) {
                                    field.clear();
                                }
                                run.effect_fields.retain(|_, field| !field.words.is_empty());
                                run.focus_amount = run.state.phase == Phase::Running;
                            }
                            cx.notify();
                        })),
                    ),
                )
                .child(
                    kit::Button::secondary("run-other-effect-add", "Add")
                        .density(Density::Read)
                        .blocked(draft.trim().is_empty().then_some("Type an effect first."))
                        .on_click(cx.listener(
                            move |root: &mut SplatRoot, _: &ClickEvent, _, cx| add_click(root, cx),
                        )),
                ),
        )
        .child(kit::help(view::CONDITIONS_HELP));
    column.into_any_element()
}

/// The Correct tab (`record_tab`): HP, initiative, the DM's notes and the sheet's sync.
fn record(
    run: &Run,
    index: usize,
    window: &gpui::Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let sheet_decides = run.syncs.get(&index).is_some_and(|sync| sync.ready());
    let mut column = div()
        .flex()
        .flex_col()
        .gap(theme::S)
        .max_w(theme::FORM_WIDTH);
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
                && let Some(total) = run
                    .setup_inputs
                    .get(&index)
                    .and_then(|text| crate::design::parse_signed(text))
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
                    .child(
                        div()
                            .w(px(110.0))
                            .child(kit::text(Type::Small, c.muted, "Initiative")),
                    )
                    .child(
                        div().key_context("RunField").w(px(72.0)).child(
                            kit::TextField::new(
                                ElementId::Name(format!("run-correct-initiative-{}", p.key).into()),
                                text,
                            )
                            .placeholder("Total")
                            .name(format!("Initiative for {}", p.name))
                            .on_change(cx.listener(
                                move |root: &mut SplatRoot, words: &str, _, cx| {
                                    if let Some(run) = root.app.encounters.run_mut() {
                                        run.setup_inputs.insert(index, super::number_text(words));
                                    }
                                    cx.notify();
                                },
                            ))
                            .on_enter(super::plain(cx, move |root, _, cx| set(root, cx))),
                        ),
                    )
                    .child(
                        kit::Button::secondary("run-set-initiative", "Set initiative")
                            .density(Density::Read)
                            .blocked(total.is_none().then_some("Type a number first."))
                            .on_click(cx.listener(
                                move |root: &mut SplatRoot, _: &ClickEvent, _, cx| set(root, cx),
                            )),
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
                .child(
                    div()
                        .w(px(110.0))
                        .child(kit::text(Type::Small, c.muted, "Note")),
                )
                .child(
                    div().key_context("RunField").flex_1().child(
                        kit::TextField::new(
                            ElementId::Name(format!("run-note-{}", p.key).into()),
                            note_text.clone(),
                        )
                        .placeholder("What happened")
                        .name("DM note")
                        .on_change(cx.listener(
                            |root: &mut SplatRoot, words: &str, _, cx| {
                                if let Some(run) = root.app.encounters.run_mut() {
                                    run.note = words.to_owned();
                                }
                                cx.notify();
                            },
                        )),
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
            .child(sync_settings(run, index, window, cx));
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
        .child(
            div()
                .w(px(110.0))
                .child(kit::text(Type::Small, c.muted, label)),
        )
        .child(
            div().key_context("RunField").w(px(96.0)).child(
                kit::TextField::new(id, value.map_or_else(String::new, |n| n.to_string()))
                    .name(label)
                    .disabled(locked)
                    .on_change(
                        cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                            if let Some(run) = root.app.encounters.run_mut() {
                                let typed = super::number_text(words);
                                let parsed = crate::design::parse_signed(&typed);
                                if typed.is_empty() || parsed.is_some() {
                                    let command = make(run, parsed);
                                    run.run_command(command);
                                }
                            }
                            cx.notify();
                        }),
                    ),
            ),
        )
}

/// A sheet's sync in Correct (`CombatSync::show`): the same sync editor as the Party page's
/// sheet (`party::sync_editor`), for this row's sync. Without a connection, the box waits for
/// one and `Connect to sync` leads to it.
fn sync_settings(
    run: &Run,
    index: usize,
    window: &gpui::Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    use crate::gui::pages::party::sync_editor;
    let connected = run.connected || run.simulate_connection;
    let sync = run.syncs.get(&index);
    if !connected {
        let enabled = sync.is_some_and(|sync| sync.enabled);
        return div()
            .flex()
            .flex_col()
            .gap(theme::XS)
            .child(
                kit::Checkbox::new(
                    "run-sync-sheet",
                    "Sync combat stats with D&D Beyond",
                    enabled,
                )
                .blocked(Some("Connect to sync.")),
            )
            .child(
                kit::Link::new("run-sync-connect", "Connect to sync")
                    .on_click(on_run(cx, |run| run.wants_sign_in = true)),
            )
            .into_any_element();
    }
    sync_editor::panel(sync_editor::Host::Run(index), sync, window, cx)
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
                            .on_click(cx.listener(
                                move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                                    root.ctx.set(crate::requests::EDIT_RUN_ENCOUNTER, id);
                                    if let Some(run) = root.app.encounters.run_mut() {
                                        run.prep_refusal = None;
                                    }
                                    cx.notify();
                                },
                            )),
                    )
                    .children(run.prep_refusal.as_ref().map(|refusal| {
                        kit::text(Type::Small, c.muted, refusal.reason().to_owned())
                    }));
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
        column = column.child(kit::gap(theme::M)).child(
            kit::QuietSection::new("run-prep-technical", "Technical details").child(
                kit::markdown::Markdown::new("run-prep-technical-body", technical).compact(),
            ),
        );
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
                .child(
                    div()
                        .w(px(96.0))
                        .flex_none()
                        .child(kit::text(Type::Heading, c.text, *key)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .child(kit::text(Type::Body, c.muted, *what)),
                )
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
fn arrivals(run: &Run, room: Pixels, window: &Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    // One block for every group, no wider than a list, so the controls keep one column.
    let block = theme::LIST_MEASURE.min(room - theme::SCROLL_BAR_WIDTH);
    let mut column = div()
        .flex()
        .flex_col()
        .gap(theme::XS)
        .max_w(theme::LIST_MEASURE);
    if run.groups.as_ref().is_some_and(|groups| !groups.is_empty()) {
        column = column.child(kit::help("Drag a group name to the list, or use Add."));
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
                    .child(kit::text(
                        Type::Small,
                        c.muted,
                        "No groups yet. Make one on Encounters with",
                    ))
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
            column = column
                .child(group_row(run, group, block, window, cx))
                .child(kit::rule());
        }
    }
    column.into_any_element()
}

/// What a drag of an Arrivals group carries: the group, and its name for the ghost. A group
/// whose Add waits carries none; its ghost still follows the pointer.
#[derive(Clone)]
pub(super) struct GroupDrag {
    pub(super) id: Option<uuid::Uuid>,
    name: SharedString,
}

impl gpui::Render for GroupDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::XS)
            .px(theme::S)
            .h(Density::Read.control())
            .rounded(theme::RADIUS_CONTROL)
            .bg(c.raised)
            .border(theme::HAIRLINE_WIDTH)
            .border_color(c.rule)
            .child(kit::icon(icons::ADD, Type::Body, c.text))
            .child(kit::text::one_line(Type::Strong, c.text, self.name.clone()))
    }
}

/// The initiative list as the place a group from Arrivals is dropped (`group_drop_zone`): while
/// a group that can join is held over it, it says so with the accent outline; a drop adds the
/// group as its Add does. A group whose Add came to wait while it was held does not join.
pub(super) fn group_drop_zone(
    zone: gpui::Div,
    id: &'static str,
    cx: &mut Context<SplatRoot>,
) -> gpui::Stateful<gpui::Div> {
    let c = colors();
    zone.id(id)
        .rounded(theme::RADIUS_CONTROL)
        .drag_over::<GroupDrag>(move |style, drag, _, _| {
            if drag.id.is_some() {
                style.border(theme::FOCUS_WIDTH).border_color(c.accent)
            } else {
                style
            }
        })
        .on_drop(
            cx.listener(|root: &mut SplatRoot, drag: &GroupDrag, _, cx| {
                if let (Some(id), Some(run)) = (drag.id, root.app.encounters.run_mut()) {
                    let takes = run
                        .groups
                        .iter()
                        .flat_map(|groups| groups.iter())
                        .find(|group| group.id == id)
                        .is_some_and(|group| run.add_waits_words(group).is_none());
                    if takes {
                        run.add_group_from_panel(id);
                    }
                }
                cx.notify();
            }),
        )
}

/// One encounter group in Arrivals (`group_row`): its name, which drags to the initiative list,
/// with the fight after it joins (`Fight after: Moderate`) on the heading or under it; its join
/// status and members; an initiative total and Add, which Enter in the total presses too; then
/// each fault with its repair.
fn group_row(
    run: &Run,
    group: &crate::encounter_run::RunGroup,
    block: Pixels,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let id = group.id;
    let name = group.name.clone();
    let status = run.group_status(group, run.group_joined(group));
    let blocked = run.add_waits_words(group);
    let total = run.group_totals.get(&id).cloned().unwrap_or_default();
    let small = |words: &str| super::measure::width(window, Type::Small, words);
    let strong = |words: &str| super::measure::width(window, Type::Strong, words);
    // The fight after the group joins, by the run's kept words (`arrival_words`).
    let heading_fit = run.arrival_heading(
        group,
        f32::from(block),
        |words| f32::from(small(words)),
        |words| f32::from(strong(words)),
    );
    let split = &heading_fit.split;
    let title = format!("{} {name}", icons::DRAG);
    // The line's words as one, the name a test finds it by.
    let line_words = heading_fit.line.as_ref().map(|fitted| {
        let form: String = fitted
            .form
            .iter()
            .map(|(words, _)| words.as_str())
            .collect();
        match (fitted.lead.is_empty(), form.is_empty()) {
            (true, _) => form,
            (false, true) => fitted.lead.clone(),
            (false, false) => format!("{} · {form}", fitted.lead),
        }
    });
    let answer = heading_fit.line.as_ref().map(|fitted| {
        let mut words = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .when(!fitted.wrap, |line| line.whitespace_nowrap());
        if !fitted.lead.is_empty() {
            let lead = if fitted.form.is_empty() {
                fitted.lead.clone()
            } else {
                format!("{} · ", fitted.lead)
            };
            words = words.child(kit::text(Type::Small, c.muted, lead));
        }
        for (part, word) in &fitted.form {
            words = words.child(kit::text(
                Type::Small,
                if *word { c.text } else { c.muted },
                part.clone(),
            ));
        }
        words
    });
    let totals_text = heading_fit.totals.clone();
    let described = heading_fit.described.clone();
    let hover = heading_fit.hover.clone();
    let changed = view::changed_mark(group).is_some();
    let movable = blocked.is_none();
    let drag = GroupDrag {
        id: movable.then_some(id),
        name: name.clone().into(),
    };
    let grip = div()
        .id(ElementId::Name(format!("run-arrival-grip-{id}").into()))
        .flex_none()
        .w(if split.below {
            px(split.title)
        } else {
            px(split.title).min(block - px(split.room) - theme::L)
        })
        .min_w_0()
        .h(Density::Read.control())
        .flex()
        .items_center()
        .cursor_grab()
        .child(
            div()
                .min_w_0()
                .ty(Type::Strong)
                .text_color(c.text)
                .truncate()
                .child(title),
        )
        .on_drag(drag, |drag, _, _, cx| cx.new(|_| drag.clone()))
        .with_tooltip_words(hover);
    let grip = kit::target(
        grip,
        format!(
            "Drag {name} to the initiative list{}",
            if changed { ". Changed." } else { "" }
        ),
    );
    let mut heading = div().flex().flex_row().items_center().w(block).child(grip);
    let mut below = None;
    match answer {
        Some(answer) if split.below => {
            below = Some(kit::target(answer, line_words.clone().unwrap_or_default()));
        }
        Some(answer) => {
            heading = heading.child(div().flex_1()).child(kit::target(
                div()
                    .id(ElementId::Name(format!("run-arrival-fight-{id}").into()))
                    .flex_none()
                    .child(answer)
                    .when_some(described.clone(), |line, words| {
                        line.with_tooltip_words(words)
                    }),
                line_words.clone().unwrap_or_default(),
            ));
        }
        None => {}
    }
    let members = match view::changed_mark(group) {
        Some(mark) => format!("{} ·\u{a0}{mark}", group.member_summary()),
        None => group.member_summary(),
    };
    let summary = if status.is_empty() {
        members
    } else {
        format!("{status} · {members}")
    };
    let add_word = view::arrival_add_word(group);
    let on_enter = {
        let blocked = blocked.is_some();
        super::plain(cx, move |root, _, cx| {
            if !blocked && let Some(run) = root.app.encounters.run_mut() {
                run.add_group_from_panel(id);
            }
            cx.notify();
        })
    };
    let controls = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .w(block)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .child(kit::text(Type::Small, c.muted, summary)),
        )
        .child(kit::text(Type::Small, c.muted, "Initiative"))
        .child(
            div()
                .id(ElementId::Name(
                    format!("run-arrival-total-box-{id}").into(),
                ))
                .key_context("RunField")
                .w(px(72.0))
                .flex_none()
                .child(
                    kit::TextField::new(
                        ElementId::Name(format!("run-arrival-total-{id}").into()),
                        total,
                    )
                    .placeholder("Roll")
                    .name(format!("Initiative total for {name}"))
                    .disabled(group.problem.is_some())
                    .on_change(
                        cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                            if let Some(run) = root.app.encounters.run_mut() {
                                run.group_totals.insert(id, words.to_owned());
                                run.totals_revision += 1;
                            }
                            cx.notify();
                        }),
                    )
                    .on_enter(on_enter),
                )
                .with_tooltip_words("Blank rolls initiative"),
        )
        .child(
            kit::Button::secondary(
                ElementId::Name(format!("run-arrival-add-{id}").into()),
                add_word,
            )
            .icon(icons::ADD)
            .name(format!("Add {name} to the initiative list"))
            .density(Density::Read)
            .blocked(blocked.clone())
            .on_click(on_run(cx, move |run| run.add_group_from_panel(id))),
        );
    let mut row = div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .py(theme::XS)
        .child(heading)
        .children(below)
        .when(heading_fit.line.is_none(), |row| {
            row.child(kit::text(Type::Small, c.muted, totals_text.clone()))
        })
        .child(controls);
    // The group's faults, each with its repair; `Cannot join` before them (`member_fault_row`).
    let faults = run.arrival_faults(group);
    if !faults.is_empty() {
        row = row.child(kit::text(Type::Small, c.muted, "Cannot join"));
        let on_press: kit::messages::OnPress<view::GroupRepair> = Rc::new(cx.listener(
            move |root: &mut SplatRoot,
                  pressed: &kit::messages::Pressed<view::GroupRepair>,
                  _,
                  cx| {
                if let Some(press) = pressed.take()
                    && let Some(run) = root.app.encounters.run_mut()
                {
                    run.group_repair(id, press.id);
                }
                cx.notify();
            },
        ));
        for (at, line) in faults.into_iter().enumerate() {
            row = row.child(kit::messages::message(
                ElementId::Name(format!("run-arrival-fault-{id}-{at}").into()),
                line,
                on_press.clone(),
                true,
            ));
        }
    }
    row
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
                    .child(div().flex_1().min_w_0().child(kit::text::one_line(
                        Type::Strong,
                        c.text,
                        picked.entry.name.clone(),
                    )))
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
                .on_click(on_run(cx, move |run| {
                    run.ad_hoc.picked = Some(picked.clone())
                })),
            );
        }
        if shown < matches.len() {
            let all = typed.clone();
            column = column.child(
                kit::Button::quiet(
                    "run-creature-show-all",
                    format!("Show all {}", matches.len()),
                )
                .density(Density::Read)
                .hint("List every creature that matches, without more typing")
                .on_click(on_run(cx, move |run| {
                    run.ad_hoc.show_all = Some(all.clone())
                })),
            );
        }
        if typed.is_empty() {
            return column.into_any_element();
        }
    }
    let add = run.creature_add();
    if add.made_up {
        column = column.child(kit::text(
            Type::Small,
            c.muted,
            "Made up here. It stays in this fight.",
        ));
    }
    let field = |id: &'static str,
                 label: &'static str,
                 name: &'static str,
                 value: String,
                 hint: String,
                 width: f32,
                 focus: bool,
                 set: fn(&mut crate::encounter_run::ad_hoc::AdHoc, String),
                 cx: &mut Context<SplatRoot>| {
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
                        .on_change(
                            cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                                if let Some(run) = root.app.encounters.run_mut() {
                                    set(&mut run.ad_hoc, words.to_owned());
                                    run.ad_hoc.focus_hp = false;
                                }
                                cx.notify();
                            }),
                        )
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
                .child(field(
                    "run-creature-count",
                    "Count",
                    "New creature count",
                    form.count.clone(),
                    "1".into(),
                    52.0,
                    false,
                    |form, words| form.count = words,
                    cx,
                ))
                .child(field(
                    "run-creature-hp",
                    "HP",
                    "New creature HP",
                    form.hp.clone(),
                    add.hp_hint.clone(),
                    64.0,
                    form.focus_hp,
                    |form, words| form.hp = words,
                    cx,
                ))
                .child(field(
                    "run-creature-ac",
                    "AC",
                    "New creature AC",
                    form.ac.clone(),
                    add.ac_hint.clone(),
                    52.0,
                    false,
                    |form, words| form.ac = words,
                    cx,
                )),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(theme::M)
                .child(field(
                    "run-creature-total",
                    "Initiative",
                    "New creature initiative total",
                    form.total.clone(),
                    "Roll".into(),
                    52.0,
                    false,
                    |form, words| form.total = words,
                    cx,
                ))
                .child(side_choice(run, cx)),
        )
        .children(
            add.blocked
                .clone()
                .map(|why| kit::text(Type::Small, c.muted, why)),
        )
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
            kit::Dropdown::new(
                "run-creature-side",
                "Side of the new creature",
                words,
                chosen,
            )
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
