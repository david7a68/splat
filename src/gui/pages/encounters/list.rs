//! The Encounters list (egui: `Encounters::ledger_view`, `encounters/list.rs`): the header, In
//! progress, Ready to run, Groups and History, one facts column across them, and the footer.
use super::common::{
    detail_line, egui_color, footer, on_page, reason, row_menu, small_icon, text_width,
};
use crate::gui::theme::{Density, ICON_SLOT, L, M, S, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use eframe::egui;
use gpui::{
    AnyElement, Context, Div, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    StatefulInteractiveElement, Styled, Window, div, px,
};
use splat_encounters_ui::encounters::model::{
    Body, GroupRow, History, Ledger, PastRow, ReadRun, ReadyRow, RunRow,
};
use splat_encounters_ui::encounters::{Encounters, ListAction, Story};

/// The list page.
pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let header = root.app.encounters.list_header();
    let sessions = root.app.sessions.links();
    let story = Story {
        events: &root.app.timeline.events,
        sessions: &sessions,
        revision: Some((
            root.app.timeline.revision(),
            root.app.sessions.notes_revision(),
        )),
    };
    let body = root.app.encounters.ledger_data(&story);
    let lines = root.app.encounters.list_footer();
    let empty = matches!(body, Body::Empty { .. });
    let mut actions = Vec::new();
    if !empty {
        actions.push(
            kit::Button::secondary("encounters-new", "New encounter")
                .icon(icons::ADD)
                .blocked(header.new_blocked)
                .on_click(on_page(cx, |page, _, _, _| page.new_encounter()))
                .into_any_element(),
        );
        actions.push(
            kit::Button::quiet("encounters-new-group", "New group")
                .icon(icons::ADD)
                .blocked(header.group_blocked)
                .hint(Encounters::new_group_hint())
                .on_click(on_page(cx, |page, _, _, _| page.new_group()))
                .into_any_element(),
        );
    }
    actions.push(
        kit::Button::quiet("encounters-reload", "Reload from disk")
            .icon(icons::REFRESH)
            .blocked(header.reload_blocked)
            .hint("Read encounters and runs from disk again")
            .on_click(on_page(cx, |page, ctx, active, campaigns| {
                page.reload_from_disk(ctx, active, campaigns)
            }))
            .into_any_element(),
    );
    let content: AnyElement = match body {
        Body::Reading => div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(small_icon(icons::BUSY, colors().muted))
            .child(kit::text(Type::Body, colors().muted, "Reading this campaign…"))
            .into_any_element(),
        Body::Nothing => div().into_any_element(),
        Body::Empty { issues } => div()
            .flex()
            .flex_col()
            .children(issues_block(&issues))
            .child(kit::empty_state(
                icons::ENCOUNTERS,
                "No encounters yet",
                "Build a roster from your party and the creatures this campaign can use. Then run it at the table.",
                Some(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(L)
                        .child(
                            kit::Button::primary("encounters-first", "New encounter")
                                .icon(icons::ADD)
                                .blocked(header.new_blocked)
                                .name("Create a first encounter")
                                .on_click(on_page(cx, |page, _, _, _| page.new_encounter())),
                        )
                        .child(
                            kit::Button::quiet("encounters-first-group", "New group")
                                .icon(icons::ADD)
                                .blocked(header.group_blocked)
                                .hint(Encounters::new_group_hint())
                                .on_click(on_page(cx, |page, _, _, _| page.new_group())),
                        )
                        .into_any_element(),
                ),
            ))
            .into_any_element(),
        Body::Ledger(ledger) => ledger_body(root, &ledger, window, cx),
    };
    let mut pane = kit::Pane::new("encounters-list").flush().child(content);
    for line in footer("encounters-list-footer", lines, cx) {
        pane = pane.footer(line);
    }
    div()
        .flex()
        .flex_col()
        .size_full()
        .max_w(super::common::measure())
        .child(kit::page_header(
            "Encounters",
            header.context.map(Into::into),
            actions,
        ))
        .child(div().flex_1().min_h_0().child(pane))
        .into_any_element()
}

fn issues_block(issues: &[String]) -> Option<Div> {
    (!issues.is_empty()).then(|| {
        let count = issues.len();
        div()
            .flex()
            .flex_col()
            .child(kit::heading(
                kit::Heading::Group {
                    title: if count == 1 {
                        "Content issue"
                    } else {
                        "Content issues"
                    },
                    count,
                },
                true,
            ))
            .children(issues.iter().map(|issue| reason(issue.clone())))
            .child(kit::gap(L))
    })
}

/// The widths the ledger's rows share: one facts column, and each list's actions column.
struct Columns {
    facts: Pixels,
    in_progress: Pixels,
    ready: Pixels,
    groups: Pixels,
    history: Pixels,
}

/// The width of a kit button with these words (`kit::Button`'s own padding, icon and edge).
fn button_width(
    window: &mut Window,
    words: &str,
    icon: bool,
    quiet: bool,
    primary: bool,
) -> Pixels {
    let ty = if primary { Type::Heading } else { Type::Strong };
    let padding = if quiet { S } else { M };
    let edge = if quiet { px(0.0) } else { px(2.0) };
    text_width(window, ty, words) + padding * 2.0 + if icon { ICON_SLOT } else { px(0.0) } + edge
}

fn menu_width() -> Pixels {
    Density::Read.control()
}

fn spaced(widths: impl IntoIterator<Item = Pixels>) -> Pixels {
    let mut sum = px(0.0);
    let mut count = 0;
    for width in widths {
        sum += width;
        count += 1;
    }
    sum + S * (count.max(1) - 1) as f32
}

/// The facts and actions columns of the ledger (`list::ledger`): the widest facts of every list
/// (Groups and History one indent further in), and each list's widest actions.
fn columns(window: &mut Window, ledger: &Ledger) -> Columns {
    let facts = |window: &mut Window, name: &str, detail: &str, icons: usize| {
        text_width(window, Type::Strong, name)
            .max(text_width(window, Type::Small, detail) + (L * icons as f32))
    };
    let mut widest = px(0.0);
    let mut in_progress = px(0.0);
    for row in &ledger.in_progress {
        let icons =
            usize::from(row.open.is_none()) + usize::from(row.detail.contains("Changed on disk"));
        widest = widest.max(facts(window, &row.name, &row.detail, icons));
        let last = match &row.open {
            Some(word) => text_width(window, Type::Small, word) + L,
            None => button_width(window, "Resume", true, false, true),
        };
        in_progress = in_progress.max(spaced([
            last,
            button_width(window, "Edit", false, true, false),
            menu_width(),
        ]));
    }
    let mut ready = px(0.0);
    for row in &ledger.ready {
        let form: String = row.form.iter().map(|(words, _)| words.as_str()).collect();
        let line = if form.is_empty() {
            row.detail.clone()
        } else {
            format!("{} · {form}", row.detail)
        };
        widest = widest.max(facts(window, &row.name, &line, 0));
        let mut parts = vec![
            button_width(window, "Start fight", true, false, true),
            button_width(window, "Edit", false, true, false),
            menu_width(),
        ];
        if let Some(short) = row.short_reason {
            parts.insert(0, text_width(window, Type::Small, short) + L);
            if row.go_to_run {
                parts.insert(0, text_width(window, Type::Small, "Go to the run"));
            }
        }
        ready = ready.max(spaced(parts));
    }
    let mut groups = px(0.0);
    for row in ledger.groups.iter().flat_map(|set| &set.rows) {
        widest = widest.max(L + facts(window, &row.name, &row.detail, 0));
        groups = spaced([
            button_width(window, "Edit", false, true, false),
            menu_width(),
        ]);
    }
    let mut history = px(0.0);
    for row in ledger.history.sessions.iter().flat_map(|(_, runs)| runs) {
        widest = widest.max(L + facts(window, &row.name, &row.detail, 0));
        history = spaced([
            button_width(window, "Close", false, true, false)
                .max(button_width(window, "Open", false, true, false)),
            menu_width(),
        ]);
    }
    Columns {
        // A pixel of slack per side, so rounding never cuts a last letter.
        facts: widest.ceil() + px(2.0),
        in_progress: in_progress.ceil(),
        ready: ready.ceil(),
        groups: groups.ceil(),
        history: history.ceil(),
    }
}

fn ledger_body(
    root: &mut SplatRoot,
    ledger: &Ledger,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let columns = columns(window, ledger);
    let state = &root.pages.encounters;
    let (groups_open, history_open) = (!state.groups_closed, !state.history_closed);
    let c = colors();
    let mut body = div()
        .flex()
        .flex_col()
        .w_full()
        .children(issues_block(&ledger.issues));

    // In progress.
    let mut heading = kit::label("In progress", ledger.in_progress.len());
    if let Some(order) = ledger.in_progress_order {
        heading = heading.child(kit::text(Type::Label, c.muted, format!("· {order}")));
    }
    body = body.child(div().pb(XS).child(heading));
    if let Some(words) = &ledger.in_progress_reason {
        body = body.child(reason(words.clone()));
    }
    for row in &ledger.in_progress {
        body = body.child(run_row(row, &columns, cx));
    }
    body = body.child(kit::gap(L));

    // Ready to run.
    body = body.child(
        div()
            .pb(XS)
            .child(kit::label("Ready to run", ledger.ready.len())),
    );
    if let Some(blocked) = ledger.ready_blocked {
        body = body.child(reason(blocked));
    }
    if ledger.ready.is_empty() {
        body = body.child(reason("None ready. Select New encounter."));
    }
    for row in &ledger.ready {
        body = body.child(ready_row(row, &columns, cx));
    }
    body = body.child(kit::gap(L));

    // Groups.
    if ledger.groups.is_empty() {
        body = body
            .child(div().pb(XS).child(kit::label("Groups", 0)))
            .child(reason("No groups yet. Select New group."));
    } else {
        body = body.child(section_heading(
            "encounters-groups-section",
            "Groups",
            ledger.group_count,
            groups_open,
            cx.listener(|root: &mut SplatRoot, _, _, cx| {
                root.pages.encounters.groups_closed ^= true;
                cx.notify();
            }),
        ));
        if groups_open {
            let mut sets = div().flex().flex_col().pl(L);
            for (index, set) in ledger.groups.iter().enumerate() {
                sets = sets.child(kit::heading(
                    kit::Heading::Group {
                        title: &set.title,
                        count: set.rows.len(),
                    },
                    index == 0,
                ));
                for row in &set.rows {
                    sets = sets.child(group_row(row, &columns, cx));
                }
            }
            body = body.child(sets);
        }
    }
    body = body.child(kit::gap(L));

    // History.
    let names = ledger
        .history
        .sessions
        .iter()
        .flat_map(|(_, runs)| runs)
        .filter_map(|run| run.read.as_ref())
        .flat_map(|read| &read.rows)
        .map(|row| text_width(window, Type::Strong, &row.name))
        .fold(px(0.0), Pixels::max)
        .ceil();
    let names = names.min((columns.facts - L * 2.0).max(px(80.0)));
    body = body.child(history_block(
        &ledger.history,
        &columns,
        history_open,
        names,
        cx,
    ));
    kit::scroll_column("encounters-ledger", body).into_any_element()
}

/// A section heading that opens and closes (`design::section_opened`): a caret, then the words.
fn section_heading(
    id: &'static str,
    words: &str,
    count: usize,
    open: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    let c = colors();
    let toggle = div()
        .id(id)
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .min_h(crate::gui::theme::MIN_TARGET)
        .cursor_pointer()
        .child(small_icon(
            if open { icons::DOWN } else { icons::NEXT },
            c.text,
        ))
        .child(kit::label(words.to_owned(), count))
        .on_click(on_click);
    kit::target(toggle, format!("{words} section"))
}

/// One ledger row: its facts in the shared facts column, `L`, then its actions at the end of its
/// list's actions column.
fn ledger_row(
    id: ElementId,
    indent: Pixels,
    columns: &Columns,
    actions_width: Pixels,
    facts: Div,
    actions: Vec<AnyElement>,
) -> Div {
    let _ = id;
    div()
        .flex()
        .flex_row()
        .items_center()
        .min_h(Density::Scan.row() + S)
        .child(
            div()
                .flex_shrink(1.0)
                .min_w_0()
                .w((columns.facts - indent).max(px(0.0)))
                .child(facts),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .flex_none()
                .items_center()
                .justify_end()
                .ml(L)
                .gap(S)
                .min_w(actions_width)
                .children(actions),
        )
}

fn facts(name: &str, line: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .min_w_0()
        .child(kit::text::one_line(
            Type::Strong,
            colors().text,
            name.to_owned(),
        ))
        .child(line)
}

fn run_row(row: &RunRow, columns: &Columns, cx: &mut Context<SplatRoot>) -> Div {
    let key = row.path.to_string_lossy().to_string();
    let mut items = Vec::new();
    if let Some((path, blocked)) = &row.start_another {
        let path = path.clone();
        items.push(
            kit::MenuItem::new("Start another fight")
                .blocked(*blocked)
                .on_click(act(cx, ListAction::Run(path))),
        );
    }
    if let Some(blocked) = row.delete {
        items.push(
            kit::MenuItem::new("Delete the saved run")
                .icon(icons::REMOVE)
                .blocked(blocked)
                .on_click(act(cx, ListAction::Delete(row.path.clone()))),
        );
    }
    let mut actions: Vec<AnyElement> = Vec::new();
    match &row.open {
        Some(word) => actions.push(
            kit::status_link(
                ElementId::Name(format!("run-open:{key}").into()),
                kit::Status::new(icons::RUN, word.clone(), colors().muted),
            )
            .on_click(act(cx, ListAction::BackToRun))
            .into_any_element(),
        ),
        None => {
            let id = ElementId::Name(format!("resume:{key}").into());
            let button = if row.resume_primary {
                kit::Button::primary(id, "Resume")
            } else {
                kit::Button::secondary(id, "Resume")
            };
            actions.push(
                button
                    .icon(icons::RUN)
                    .blocked(row.blocked)
                    .on_click(act(cx, ListAction::Resume(row.path.clone())))
                    .into_any_element(),
            );
        }
    }
    let edit_slot = button_slot();
    actions.push(match &row.edit {
        Some(path) => edit_slot
            .child(
                kit::Button::quiet(ElementId::Name(format!("edit:{key}").into()), "Edit")
                    .on_click(act(cx, ListAction::Edit(path.clone()))),
            )
            .into_any_element(),
        None => edit_slot.into_any_element(),
    });
    actions.push(
        row_menu(
            ElementId::Name(format!("menu:{key}").into()),
            &row.name,
            items,
        )
        .into_any_element(),
    );
    ledger_row(
        ElementId::Name(format!("run-row:{key}").into()),
        px(0.0),
        columns,
        columns.in_progress,
        facts(&row.name, detail_line(&row.detail, row.open.is_none())),
        actions,
    )
}

/// The In progress row's Edit slot: as wide where the row has no Edit, so Resume keeps its place.
fn button_slot() -> Div {
    div().flex().flex_row().justify_end().min_w(px(44.0))
}

fn ready_row(row: &ReadyRow, columns: &Columns, cx: &mut Context<SplatRoot>) -> Div {
    let c = colors();
    let key = row.path.clone();
    let mut line = div()
        .id(ElementId::Name(format!("ready-line:{key}").into()))
        .flex()
        .flex_row()
        .items_baseline()
        .min_w_0()
        .overflow_hidden()
        .whitespace_nowrap()
        .ty(Type::Small)
        .text_color(c.muted)
        .child(row.detail.clone());
    if !row.form.is_empty() {
        line = line.child(" · ");
        for (words, word) in &row.form {
            line = line.child(
                div()
                    .text_color(if *word { c.text } else { c.muted })
                    .child(words.clone()),
            );
        }
    }
    if let Some(budgets) = row.budgets.clone() {
        line = kit::WithTooltip::with_tooltip(line, budgets);
    }
    let mut items = vec![
        kit::MenuItem::new("Edit encounter").on_click(act(cx, ListAction::Edit(row.path.clone()))),
    ];
    if row.removable {
        items.push(
            kit::MenuItem::new("Remove")
                .icon(icons::REMOVE)
                .on_click(act(cx, ListAction::RemoveEncounter(row.path.clone()))),
        );
    }
    let mut actions: Vec<AnyElement> = Vec::new();
    if let Some(short) = row.short_reason {
        if row.go_to_run {
            actions.push(
                kit::Button::quiet(
                    ElementId::Name(format!("go-to-run:{key}").into()),
                    "Go to the run",
                )
                .plain()
                .on_click(act(cx, ListAction::BackToRun))
                .into_any_element(),
            );
        }
        actions.push(reason(short).whitespace_nowrap().mr(S).into_any_element());
    }
    let id = ElementId::Name(format!("start:{key}").into());
    let start = if row.start_primary {
        kit::Button::primary(id, "Start fight")
    } else {
        kit::Button::secondary(id, "Start fight")
    };
    actions.push(
        start
            .icon(icons::RUN)
            .blocked(row.start_blocked)
            .on_click(act(cx, ListAction::Run(row.path.clone())))
            .into_any_element(),
    );
    actions.push(
        kit::Button::quiet(ElementId::Name(format!("edit:{key}").into()), "Edit")
            .on_click(act(cx, ListAction::Edit(row.path.clone())))
            .into_any_element(),
    );
    actions.push(
        row_menu(
            ElementId::Name(format!("menu:{key}").into()),
            &row.name,
            items,
        )
        .into_any_element(),
    );
    ledger_row(
        ElementId::Name(format!("ready-row:{key}").into()),
        px(0.0),
        columns,
        columns.ready,
        facts(&row.name, line),
        actions,
    )
}

fn group_row(row: &GroupRow, columns: &Columns, cx: &mut Context<SplatRoot>) -> Div {
    let key = row.path.clone();
    let items = vec![
        kit::MenuItem::new("Remove")
            .icon(icons::REMOVE)
            .blocked(row.blocked)
            .on_click(act(cx, ListAction::RemoveGroup(row.path.clone()))),
    ];
    let actions = vec![
        kit::Button::quiet(ElementId::Name(format!("edit-group:{key}").into()), "Edit")
            .name(format!("Edit group {}", row.name))
            .blocked(row.blocked)
            .on_click(act(cx, ListAction::OpenGroup(row.path.clone())))
            .into_any_element(),
        row_menu(
            ElementId::Name(format!("menu:{key}").into()),
            &row.name,
            items,
        )
        .into_any_element(),
    ];
    let mut block = div().flex().flex_col().child(ledger_row(
        ElementId::Name(format!("group-row:{key}").into()),
        L,
        columns,
        columns.groups,
        facts(&row.name, detail_line(&row.detail, false)),
        actions,
    ));
    if !row.faults.is_empty() {
        block = block.child(reason("Cannot join"));
    }
    for (index, (fault, packages)) in row.faults.iter().enumerate() {
        let (label, repair): (&str, Repair) = if *packages {
            (
                "Campaign → Packages",
                Box::new(|root: &mut SplatRoot| {
                    root.bridge.ctx().data_mut(|data| {
                        data.insert_temp(egui::Id::new("open_campaign_packages"), true)
                    });
                }),
            )
        } else {
            let path = row.path.clone();
            (
                "Edit group",
                Box::new(move |root: &mut SplatRoot| {
                    super::common::with_page(root, |page, _, active, campaigns| {
                        page.list_act(ListAction::OpenGroup(path.clone()), active, campaigns)
                    });
                }),
            )
        };
        block = block.child(
            kit::Message::new(kit::messages::tone(fault.tone), fault.words.clone())
                .small()
                .action(
                    kit::Button::quiet(
                        ElementId::Name(format!("group-fault:{key}:{index}").into()),
                        label,
                    )
                    .plain()
                    .on_click(cx.listener(move |root, _, _, cx| {
                        repair(root);
                        cx.notify();
                    })),
                ),
        );
    }
    for problem in &row.problems {
        block = block.child(kit::Message::new(kit::Tone::Warning, problem.clone()).small());
    }
    block
}

/// A group fault's repair: Campaign → Packages, or the group's editor.
type Repair = Box<dyn Fn(&mut SplatRoot)>;

fn history_block(
    history: &History,
    columns: &Columns,
    open: bool,
    names: Pixels,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let mut block = div().flex().flex_col();
    if let Some(words) = &history.reason {
        return block
            .child(div().pb(XS).child(kit::label("History", 0)))
            .child(reason(words.clone()));
    }
    let forced = history
        .sessions
        .iter()
        .flat_map(|(_, runs)| runs)
        .any(|run| run.read.is_some());
    let open = open || forced;
    block = block.child(section_heading(
        "encounters-history-section",
        "History",
        history.count,
        open,
        cx.listener(|root: &mut SplatRoot, _, _, cx| {
            root.pages.encounters.history_closed ^= true;
            cx.notify();
        }),
    ));
    if !open {
        return block;
    }
    let mut sessions = div().flex().flex_col().pl(L);
    for (index, (label, runs)) in history.sessions.iter().enumerate() {
        sessions = sessions.child(kit::heading(
            kit::Heading::Group {
                title: label,
                count: runs.len(),
            },
            index == 0,
        ));
        for run in runs {
            sessions = sessions.child(past_row(run, columns, cx));
            if let Some(read) = &run.read {
                sessions = sessions
                    .child(read_run(read, run, names, cx))
                    .child(kit::gap(L));
            }
        }
    }
    if !history.unreadable.is_empty() {
        sessions = sessions.child(kit::heading(
            kit::Heading::Group {
                title: &format!("Unreadable {}", history.unreadable.len()),
                count: 0,
            },
            false,
        ));
        for label in &history.unreadable {
            sessions = sessions.child(reason(label.clone()));
        }
    }
    block.child(sessions)
}

fn past_row(row: &PastRow, columns: &Columns, cx: &mut Context<SplatRoot>) -> Div {
    let key = row.path.to_string_lossy().to_string();
    let mut items = vec![
        kit::MenuItem::new("Delete the saved run")
            .icon(icons::REMOVE)
            .blocked(row.delete_blocked)
            .on_click(act(cx, ListAction::Delete(row.path.clone()))),
    ];
    if let Some((path, blocked)) = &row.template {
        items.push(
            kit::MenuItem::new("Start fight")
                .blocked(*blocked)
                .on_click(act(cx, ListAction::Run(path.clone()))),
        );
        items.push(kit::MenuItem::new("Edit").on_click(act(cx, ListAction::Edit(path.clone()))));
    }
    let opened = row.read.is_some();
    let word = if opened { "Close" } else { "Open" };
    let actions = vec![
        kit::Button::quiet(ElementId::Name(format!("history-open:{key}").into()), word)
            .name(format!("{word} the run of {}, {}", row.name, row.label))
            .on_click(act(cx, ListAction::ReadHistory(row.path.clone())))
            .into_any_element(),
        row_menu(
            ElementId::Name(format!("menu:{key}").into()),
            &row.name,
            items,
        )
        .into_any_element(),
    ];
    ledger_row(
        ElementId::Name(format!("past-row:{key}").into()),
        L,
        columns,
        columns.history,
        facts(&row.name, detail_line(&row.detail, false)),
        actions,
    )
}

/// An ended fight read in place (`list::history_summary`).
fn read_run(read: &ReadRun, row: &PastRow, names: Pixels, cx: &mut Context<SplatRoot>) -> Div {
    let c = colors();
    let key = row.path.to_string_lossy().to_string();
    let mut block = div()
        .flex()
        .flex_col()
        .pl(L)
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_baseline()
                .gap(S)
                .child(kit::status(&kit::Status::new(
                    icons::READ_ONLY,
                    "Read-only",
                    c.document.read_only,
                )))
                .child(kit::text(Type::Small, c.muted, read.end_reason.clone())),
        )
        .child(kit::gap(S));
    block = match &read.not_last {
        Some(words) => block.child(reason(words.clone())),
        None => block.child(
            div().flex().child(
                kit::Button::secondary(
                    ElementId::Name(format!("continue:{key}").into()),
                    "Continue fight",
                )
                .icon(icons::RUN)
                .blocked(read.continue_blocked)
                .on_click(act(cx, ListAction::ContinueFight(row.path.clone()))),
            ),
        ),
    };
    block = block.child(kit::heading(
        kit::Heading::Group {
            title: "Final state",
            count: 0,
        },
        false,
    ));
    for final_row in &read.rows {
        let mut values = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_baseline()
            .gap(M)
            .ty(Type::Small)
            .child(div().text_color(c.text).child(final_row.hp.clone()));
        if let Some(temp) = &final_row.temp {
            values = values.child(div().text_color(c.hp.temp).child(temp.clone()));
        }
        if !final_row.rest.is_empty() {
            let mut rest = div().flex().flex_row().flex_wrap().items_baseline();
            for (index, (words, color)) in final_row.rest.iter().enumerate() {
                if index > 0 {
                    rest = rest.child(div().text_color(c.muted).child(" · "));
                }
                rest = rest.child(div().text_color(egui_color(*color)).child(words.clone()));
            }
            values = values.child(rest);
        }
        block = block.child(
            div()
                .flex()
                .flex_row()
                .items_baseline()
                .gap(M)
                .child(div().w(names).flex_none().child(kit::text(
                    Type::Strong,
                    c.text,
                    final_row.name.clone(),
                )))
                .child(values),
        );
    }
    block = block.child(kit::heading(
        kit::Heading::Group {
            title: "Combat log",
            count: 0,
        },
        false,
    ));
    for line in &read.log {
        block = block.child(reason(line.clone()));
    }
    block
}

/// A click that does one of the list's row actions (`Encounters::list_act`).
fn act(
    cx: &mut Context<SplatRoot>,
    action: ListAction,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static {
    let action = std::rc::Rc::new(action);
    on_page(cx, move |page, _, active, campaigns| {
        page.list_act(action.as_ref().clone(), active, campaigns)
    })
}
