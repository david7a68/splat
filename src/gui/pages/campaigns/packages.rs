//! Campaign → Packages (egui: `CampaignContent::show`, `selection_bar`, `packages_content`): the
//! save row of the package choices, the issues, then the package list beside the chosen
//! package's entries (or `Needed by your entries`), and the page's footer.
use crate::campaign_content::view::{Ready, SaveBar};
use crate::gui::kit::{self, Button, Checkbox, Message, Pane, Row, Section, TextField, Tone};
use crate::gui::pages::settings;
use crate::gui::theme::{Density, L, LIST_MEASURE, M, S, Type, XS, colors};
use crate::gui::{SplatRoot, icons};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window, div,
    px,
};

fn id(prefix: &str, uuid: uuid::Uuid) -> ElementId {
    ElementId::Name(SharedString::from(format!("{prefix}-{uuid}")))
}

pub(super) fn render(
    root: &mut SplatRoot,
    _: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let ready = {
        let crate::gui::SplatRoot { app, bridge, .. } = root;
        let ready = app.campaign_content.prepare(bridge.ctx(), &app.campaigns);
        if std::mem::take(&mut app.campaign_content.open_run_settings) {
            app.campaign_tab = crate::CampaignTab::RunSettings;
        }
        ready
    };
    let active = match ready {
        Ready::NoCampaign => {
            return kit::text(Type::Body, c.text, "Select a campaign first.").into_any_element();
        }
        Ready::PreviousUnsaved => {
            return div()
                .flex()
                .flex_col()
                .gap(XS)
                .child(kit::heading(kit::Heading::Section("Unsaved campaign content"), true))
                .child(kit::text(
                    Type::Body,
                    c.text,
                    "The previous campaign still has pending changes. Select it again to save them, or discard and switch.",
                ))
                .child(
                    Button::destructive("content-discard-switch", "Discard and switch campaign")
                        .on_click(cx.listener(|root, _, _, cx| {
                            let crate::gui::SplatRoot { app, bridge, .. } = root;
                            app.campaign_content
                                .discard_and_switch(bridge.ctx(), &app.campaigns);
                            cx.notify();
                        })),
                )
                .into_any_element();
        }
        Ready::Page(active) => active,
    };
    let open = root.pages.campaigns.repairs_open.clone();
    let lines = root
        .app
        .campaign_content
        .footer_lines(&root.app.campaigns, &active, &|path| open.contains(path));
    let footer_active = active.clone();
    let footer = kit::messages::lines(
        "campaign-content-footer",
        lines,
        settings::on_press(cx, move |root, press, cx| {
            let crate::gui::SplatRoot { app, bridge, .. } = root;
            app.campaign_content
                .footer_press(bridge.ctx(), &app.campaigns, &footer_active, press);
            cx.notify();
        }),
    );
    let mut column = div().flex().flex_col().w_full().max_w(LIST_MEASURE).pb(L);
    if let Some(bar) = root.app.campaign_content.save_bar() {
        column = column.child(save_bar(bar, &active, cx));
    }
    if root.app.campaign_content.errors_are_content() {
        let (error, reconcile) = root
            .app
            .campaign_content
            .content_errors(&root.app.campaigns, &active);
        if let Some((words, action)) = error {
            let mut message = Message::new(Tone::Error, words);
            if let Some(action) = action {
                let write = active.clone();
                message = message.action(
                    Button::secondary("content-write-campaign", action).on_click(cx.listener(
                        move |root, _, _, cx| {
                            let app = &mut root.app;
                            app.campaign_content
                                .write_campaign_now(&app.campaigns, &write);
                            cx.notify();
                        },
                    )),
                );
            }
            column = column.child(message);
        }
        if let Some(words) = reconcile {
            column = column.child(Message::new(Tone::Error, words));
        }
    }
    for path in root.app.campaign_content.repair_paths() {
        column = column.child(Message::new(
            Tone::Warning,
            format!("{path} needs a repair before the packages can change."),
        ));
    }
    if !active.path.exists() {
        let folder = active.clone();
        column = column.child(
            Message::new(Tone::Error, "The campaign folder is missing.").action(
                Button::secondary("content-create-folder", "Create campaign folder").on_click(
                    cx.listener(move |root, _, _, cx| {
                        let crate::gui::SplatRoot { app, bridge, .. } = root;
                        app.campaign_content
                            .create_folder(bridge.ctx(), &app.campaigns, &folder);
                        cx.notify();
                    }),
                ),
            ),
        );
    }
    if root.app.campaign_content.loading() {
        column = column.child(
            div()
                .flex()
                .flex_row()
                .gap(S)
                .child(kit::icon(icons::BUSY, Type::Body, c.connection.busy))
                .child(kit::text(Type::Body, c.text, "Loading campaign content…")),
        );
    } else {
        column = column.children(issues(root, &active, cx));
        if root.app.campaign_content.save_bar().is_some() {
            column = column.child(kit::rule()).child(lists(root, &active, cx));
        }
    }
    let mut pane = Pane::new("campaign-content").flush().child(column);
    for line in footer {
        pane = pane.footer(line);
    }
    pane.into_any_element()
}

/// The save row: the choices' state and Discard, Reload from disk, why Save waits, and
/// `Save selections`.
fn save_bar(
    bar: SaveBar,
    active: &crate::persistence::Campaign,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let mut row = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(S)
        .py(XS);
    if bar.changed {
        row = row
            .child(kit::status(&kit::Status::new(
                icons::DRAFT,
                "Unsaved",
                c.document.unsaved,
            )))
            .child(
                Button::quiet("content-discard", "Discard")
                    .density(Density::Read)
                    .on_click(cx.listener(|root, _, _, cx| {
                        root.app.campaign_content.discard_choices();
                        cx.notify();
                    })),
            );
    } else {
        row = row.child(kit::status(&kit::Status::new(
            icons::OK,
            "No changes",
            c.muted,
        )));
    }
    if !bar.dirty {
        let reload = active.clone();
        row = row.child(
            Button::quiet("content-reload", "Reload from disk")
                .icon(icons::REFRESH)
                .density(Density::Read)
                .blocked(
                    bar.reload_blocked
                        .then_some("Save or discard your changes first."),
                )
                .on_click(cx.listener(move |root, _, _, cx| {
                    let crate::gui::SplatRoot { app, bridge, .. } = root;
                    app.campaign_content
                        .reload_now(bridge.ctx(), &app.campaigns, &reload);
                    cx.notify();
                })),
        );
    }
    if let Some(reason) = bar.switch_reason {
        row = row.child(kit::text(Type::Small, c.muted, reason));
    }
    let blocked = bar
        .repair
        .clone()
        .or((!bar.dirty).then(|| "No changes to save.".to_owned()));
    row = row.child(div().flex_1()).child(
        Button::primary("content-save", "Save selections")
            .density(Density::Read)
            .key("Ctrl+S")
            .blocked(blocked)
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.campaign_content.save_now();
                cx.notify();
            })),
    );
    let mut column = div().flex().flex_col().child(row);
    if let Some(reason) = bar.repair {
        column = column.child(div().flex().justify_end().child(kit::text(
            Type::Small,
            c.muted,
            reason,
        )));
    }
    column.into_any_element()
}

/// The Campaign.md flags, the errors and missing dependencies, under one caret heading.
fn issues(
    root: &mut SplatRoot,
    active: &crate::persistence::Campaign,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let issues = root.app.campaign_content.issues(active)?;
    let flag = |(words, choose): (String, bool), at: usize, cx: &mut Context<SplatRoot>| {
        let mut message = Message::new(Tone::Warning, words);
        if choose {
            message = message.action(
                Button::secondary(("content-choose-rules", at), "Choose in Rules and runs")
                    .on_click(cx.listener(|root, _, _, cx| {
                        root.app.campaign_tab = crate::CampaignTab::RunSettings;
                        cx.notify();
                    })),
            );
        }
        message.into_any_element()
    };
    if !issues.shown {
        if issues.flags.is_empty() {
            return None;
        }
        let flags: Vec<AnyElement> = issues
            .flags
            .into_iter()
            .enumerate()
            .map(|(at, f)| flag(f, at, cx))
            .collect();
        return Some(
            div()
                .flex()
                .flex_col()
                .gap(XS)
                .children(flags)
                .into_any_element(),
        );
    }
    if issues.errors.is_empty() && issues.flags.is_empty() {
        return None;
    }
    let open = root.pages.campaigns.issues_open;
    let mut section = Section::new("content-issues", issues.heading.clone(), open).on_toggle(
        cx.listener(|root, open: &bool, _, cx| {
            root.pages.campaigns.issues_open = *open;
            cx.notify();
        }),
    );
    if open {
        for (at, f) in issues.flags.into_iter().enumerate() {
            section = section.child(flag(f, at, cx));
        }
        for error in issues.errors {
            section = section.child(kit::text(Type::Small, colors().muted, error));
        }
        for (at, (words, repairs)) in issues.missing.into_iter().enumerate() {
            let mut message = Message::new(Tone::Warning, words);
            for (index, (label, path)) in repairs.into_iter().enumerate() {
                message = message.action(
                    Button::secondary(
                        ElementId::Name(SharedString::from(format!("content-repair-{at}-{index}"))),
                        label,
                    )
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.bridge.ctx().data_mut(|data| {
                            data.insert_temp(
                                eframe::egui::Id::new("open_library_repair"),
                                path.clone(),
                            )
                        });
                        cx.notify();
                    })),
                );
            }
            section = section.child(message);
        }
        for (entry, name) in issues.affected {
            section = section.child(
                kit::Link::new(id("content-open-affected", entry), format!("Open {name}"))
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.campaign_content.open_in_lookup(entry);
                        cx.notify();
                    })),
            );
        }
    }
    Some(section.into_any_element())
}

/// The package list, left; the chosen package's entries or `Needed by your entries`, right.
fn lists(
    root: &mut SplatRoot,
    active: &crate::persistence::Campaign,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let campaign = active.id;
    let rows = root.app.campaign_content.package_rows(&root.app.campaigns);
    let mut left = div().flex().flex_col();
    for row in rows {
        let module = row.id;
        let checked = row.checked;
        let detail = match row.mark {
            Some((icon, _)) => div()
                .flex()
                .flex_row()
                .gap(XS)
                .child(kit::icon(icon, Type::Small, c.result.warning))
                .child(kit::text::one_line(
                    Type::Small,
                    c.muted,
                    row.detail.clone(),
                )),
            None => kit::text::one_line(Type::Small, c.muted, row.detail.clone()),
        };
        let spoken = match row.mark {
            Some((_, words)) => format!("{} · {words} · {}", row.name, row.detail),
            None => format!("{} · {}", row.name, row.detail),
        };
        let mixed = row.indeterminate;
        left = left.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(XS)
                .child(
                    Checkbox::new(id("content-use", module), "", checked)
                        .mixed(mixed)
                        .on_click(cx.listener(move |root, _, _, cx| {
                            root.app
                                .campaign_content
                                .toggle_package(campaign, module, !checked);
                            cx.notify();
                        })),
                )
                .child(
                    div().flex_1().min_w_0().child(
                        Row::new(id("content-package", module), spoken)
                            .height(Density::Glance.row() + XS)
                            .selected(row.selected)
                            .on_click(cx.listener(move |root, _, _, cx| {
                                root.app.campaign_content.select_package(module);
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .min_w_0()
                                    .child(kit::text::one_line(
                                        if row.selected {
                                            Type::Strong
                                        } else {
                                            Type::Body
                                        },
                                        c.text,
                                        row.name.clone(),
                                    ))
                                    .child(detail),
                            ),
                    ),
                ),
        );
        if let Some(note) = row.note {
            left = left.child(
                div()
                    .pl(px(48.0))
                    .child(kit::text(Type::Small, c.muted, note)),
            );
        }
    }
    let needed = root.app.campaign_content.needed_count();
    left = left.child(kit::rule()).child(
        kit::two_line_row(
            "content-needed",
            "Needed by your entries",
            if needed == 0 {
                String::new()
            } else {
                crate::design::count(needed, "entry", "entries")
            },
            root.app.campaign_content.needed_shown(),
        )
        .height(Density::Glance.row() + XS)
        .on_click(cx.listener(|root, _, _, cx| {
            root.app.campaign_content.select_needed();
            cx.notify();
        })),
    );
    let right = if root.app.campaign_content.needed_shown() {
        needed_column(root, active, cx)
    } else {
        package_column(root, active, cx)
    };
    div()
        .flex()
        .flex_row()
        .w_full()
        .gap(M)
        .pt(XS)
        .child(div().w(px(236.0)).flex_none().child(left))
        .child(div().flex_1().min_w_0().child(right))
        .into_any_element()
}

fn search(
    root: &SplatRoot,
    field: ElementId,
    name: &'static str,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    TextField::new(field, root.app.campaign_content.search_words().to_owned())
        .placeholder("Find an entry…")
        .name(name)
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.campaign_content.set_search(words);
            cx.notify();
        }))
        .into_any_element()
}

fn needed_column(
    root: &mut SplatRoot,
    active: &crate::persistence::Campaign,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let mut column = div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(kit::heading(
            kit::Heading::Group {
                title: "Needed by your entries",
                count: root.app.campaign_content.needed_count(),
            },
            true,
        ))
        .child(search(
            root,
            id("needed_entry_search", active.id),
            "Find a needed entry",
            cx,
        ));
    for row in root.app.campaign_content.needed_rows() {
        let entry = row.id;
        column = column.child(
            Row::new(
                id("content-needed-row", entry),
                format!("{} · {}", row.name, row.detail),
            )
            .height(Density::Glance.row() + XS)
            .on_click(cx.listener(move |root, _, _, cx| {
                root.app.campaign_content.open_in_lookup(entry);
                cx.notify();
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .min_w_0()
                    .child(kit::text::one_line(Type::Body, c.text, row.name))
                    .child(kit::text::one_line(Type::Small, c.muted, row.detail)),
            ),
        );
    }
    column.into_any_element()
}

fn package_column(
    root: &mut SplatRoot,
    _: &crate::persistence::Campaign,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let Some(view) = root.app.campaign_content.package_view() else {
        return div().into_any_element();
    };
    let module = view.id;
    let mut head = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(S)
        .min_h(Density::Read.control())
        .child(kit::label(view.name.clone(), view.count))
        .child(kit::text(
            Type::Small,
            c.muted,
            if view.all {
                "Whole package, with future additions"
            } else {
                "Chosen entries only"
            },
        ));
    if !view.all {
        head = head.child(
            Button::secondary("content-choose-all", view.choose_all)
                .density(Density::Read)
                .hint("Every entry, including future additions")
                .on_click(cx.listener(move |root, _, _, cx| {
                    root.app.campaign_content.choose_all(module);
                    cx.notify();
                })),
        );
    }
    head = head.child(
        Button::quiet("content-choose-none", "Choose none")
            .density(Density::Read)
            .blocked((!view.chosen).then_some("Nothing here is chosen."))
            .on_click(cx.listener(move |root, _, _, cx| {
                root.app.campaign_content.choose_none(module);
                cx.notify();
            })),
    );
    let rows = root.app.campaign_content.entry_rows();
    let count = rows.len();
    let rows = std::rc::Rc::new(rows);
    let list = gpui::uniform_list(
        ElementId::Name(SharedString::from(format!("package_entries-{module}"))),
        count,
        cx.processor(move |_root, range: std::ops::Range<usize>, _, cx| {
            let c = colors();
            range
                .map(|at| {
                    let row = &rows[at];
                    let (entry, checked) = (row.id, row.checked);
                    let mut check = Checkbox::new(id("content-entry", entry), "", checked);
                    if let Some(reason) = &row.locked {
                        check = check.blocked(Some(reason.clone()));
                    } else {
                        check = check.on_click(cx.listener(move |root, _, _, cx| {
                            root.app
                                .campaign_content
                                .toggle_entry(module, entry, !checked);
                            cx.notify();
                        }));
                    }
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(S)
                        .h(px(30.0))
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .child(check)
                        .child(
                            kit::Link::new(id("content-entry-name", entry), row.name.clone())
                                .on_click(cx.listener(move |root, _, _, cx| {
                                    root.app.campaign_content.open_in_lookup(entry);
                                    cx.notify();
                                })),
                        )
                        .child(kit::text(Type::Small, c.muted, row.subtitle.clone()))
                        .children(row.needed.clone().map(|needed| {
                            kit::text(
                                Type::Small,
                                c.document.read_only,
                                if row.subtitle.is_empty() {
                                    needed
                                } else {
                                    format!("· {needed}")
                                },
                            )
                        }))
                        .into_any_element()
                })
                .collect()
        }),
    )
    .h(px(30.0) * count.clamp(4, 14) as f32);
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(head)
        .child(search(
            root,
            id("package_entry_search", module),
            "Find an entry in this package",
            cx,
        ))
        .child(list)
        .into_any_element()
}
