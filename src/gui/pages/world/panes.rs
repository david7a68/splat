//! The page's larger part (egui: `WorldPage::show`'s central panel): the held NPC's row, then
//! the repair, an unreadable note, a creation form or the held NPC's card, a checks view, the
//! open note, or no note. Each is a pane whose messages sit in its footer.
use super::{SplatRoot, act, footer, on_key, on_words};
use crate::design::DiskConflictChoice;
use crate::gui::{icons, kit, theme};
use crate::lookup::{ContentIndex, Item};
use crate::persistence::Campaign;
use crate::world::edit::DraftValue;
use crate::world::page::{CardAct, Pane, View};
use gpui::{
    AnyElement, Context, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*, px,
};

/// The pane the page shows now.
pub(super) fn render(
    root: &mut SplatRoot,
    campaign: &Campaign,
    index: &ContentIndex,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let held = root.app.world_page.held_name().map(str::to_owned);
    let pane = match root.app.world_page.pane(index) {
        Pane::Repair => repair(root, campaign, cx),
        Pane::Unread => unread(root, campaign, index, cx),
        Pane::Creation => creation(root, campaign, index, cx),
        Pane::Checks(view) => checks(root, campaign, index, view, cx),
        Pane::Note(at) => super::reader::render(root, campaign, index, at, window, cx),
        Pane::Empty => empty(root, campaign, index, cx),
    };
    div()
        .flex()
        .flex_col()
        .size_full()
        .when_some(held, |column, name| column.child(held_row(&name, cx)))
        .child(div().flex_1().min_h_0().child(pane))
        .into_any_element()
}

/// The one row above another view while an unsaved generated NPC is held: `<name>`, the draft
/// status `Unsaved NPC`, then `Show`.
fn held_row(name: &str, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = theme::colors();
    let row = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(theme::S)
        .pb(theme::S)
        .child(kit::text(theme::Type::Body, c.text, name.to_owned()))
        .child(kit::status(&kit::Status::new(
            icons::DRAFT,
            crate::world::UNSAVED_NPC,
            c.document.unsaved,
        )))
        .child(kit::text(theme::Type::Body, c.muted, "·"))
        .child(
            kit::Link::new("world-held-show", "Show").on_click(cx.listener(
                |root: &mut SplatRoot, _, _, cx| {
                    root.app.world_page.press_show_held();
                    cx.notify();
                },
            )),
        );
    kit::target(row, format!("{}: {name}", crate::world::UNSAVED_NPC)).into_any_element()
}

/// A pane with its footer lines.
fn pane(id: impl Into<gpui::ElementId>, footer: Vec<AnyElement>, body: impl IntoElement) -> AnyElement {
    let mut pane = kit::Pane::new(id).flush();
    for line in footer {
        pane = pane.footer(line);
    }
    pane.child(body).into_any_element()
}

/// No note open: the advice, with the page's result and lost Log lines in the footer.
fn empty(
    root: &mut SplatRoot,
    campaign: &Campaign,
    index: &ContentIndex,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let lines = root.app.world_page.footer_plain(None, index, campaign, |_, _| {});
    let footer = footer::render(root, lines, cx);
    pane(
        "world-empty",
        footer,
        kit::empty_state(
            icons::WORLD,
            "No note open",
            "Choose one from the list, or make a new one.",
            None::<AnyElement>,
        ),
    )
}

/// A creation form, the held NPC's card, or the page's error with its Cancel.
fn creation(
    root: &mut SplatRoot,
    campaign: &Campaign,
    index: &ContentIndex,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let lines = root.app.world_page.creation_footer();
    let footer = footer::render(root, lines, cx);
    let mut column = div().flex().flex_col().max_w(theme::FORM_WIDTH + px(160.0));
    if let Some(kind) = root.app.world_page.new_kind() {
        column = column.child(new_form(root, kind, index, cx));
    }
    let run_open = root.app.world_page.run_open();
    if let Some(card) = root.app.world_page.card(index) {
        column = column.child(card_view(card, run_open, cx));
    }
    let page = &root.app.world_page;
    if let Some((words, false)) = page.error_words() {
        // M0607: the error in place of the page's body, with its Cancel at the message's words.
        column = column.child(
            kit::Message::new(kit::Tone::Error, words.to_owned()).action(
                kit::Button::secondary("world-error-cancel", "Cancel")
                    .density(theme::Density::Read)
                    .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                        root.app.world_page.cancel_error();
                        cx.notify();
                    })),
            ),
        );
    }
    let _ = campaign;
    pane("world-creation", footer, column)
}

/// Inline creation: the label title and the fields, `Create …` on the last field's line, then the
/// kept line: `Cancel`, then the reason or the file the name makes.
fn new_form(
    root: &mut SplatRoot,
    kind: &'static str,
    index: &ContentIndex,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let page = &root.app.world_page;
    let (name, note_type) = page.new_fields();
    let (name, note_type) = (name.to_owned(), note_type.to_owned());
    let title = match kind {
        "npc" => "New NPC",
        "location" => "New location",
        _ => "New note",
    };
    let label = match kind {
        "npc" => "Create NPC",
        "location" => "Create location",
        _ => "Create note",
    };
    let reason = name.trim().is_empty().then_some("Type a name first.");
    let create = if root.app.world_page.run_open() {
        kit::Button::secondary("world-create", label)
    } else {
        kit::Button::primary("world-create", label)
    };
    let create = create
        .blocked(reason)
        .on_click(act(cx, |root, campaign, _, _, _| {
            root.app.world_page.create(campaign);
        }));
    let enter = on_key(cx, |root, campaign, _, _, _| root.app.world_page.create(campaign));
    let name_field = kit::TextField::new(
        gpui::ElementId::Name(format!("world-new-name-{kind}").into()),
        name.clone(),
    )
        .name("Name")
        .on_change(on_words(cx, |root, words| root.app.world_page.set_new_name(words)))
        .on_enter(enter);
    let field_line = |field: AnyElement, create: Option<kit::Button>| {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::S)
            .child(div().w(theme::FORM_WIDTH).flex_shrink(1.0).child(field))
            .when_some(create, |line, create| line.child(create))
    };
    let mut form = div()
        .flex()
        .flex_col()
        .child(kit::heading(
            kit::Heading::Group {
                title,
                count: 0,
            },
            true,
        ));
    let named = kit::field("Name", None, None, name_field).into_any_element();
    if kind == "note" {
        form = form.child(field_line(named, None));
        let type_field = kit::field(
            "Type (optional)",
            Some("Leave empty for a plain note. For example: faction or quest.".into()),
            None,
            kit::TextField::new("world-new-type", note_type)
                .name("Type (optional)")
                .on_change(on_words(cx, |root, words| root.app.world_page.set_note_type(words))),
        )
        .into_any_element();
        form = form.child(field_line(type_field, Some(create)));
    } else {
        form = form.child(field_line(named, Some(create)));
    }
    let detail = match (reason, kind) {
        (Some(reason), _) => Some(reason.to_owned()),
        (None, "npc") => Some(format!(
            "Will create {}",
            crate::world::new_entry_path(Some(index), &name, "npc")
        )),
        _ => None,
    };
    form.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::L)
            .min_h(theme::Density::Read.control())
            .child(
                kit::Button::quiet("world-new-cancel", "Cancel")
                    .density(theme::Density::Read)
                    .line_start()
                    .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                        root.app.world_page.cancel_new();
                        cx.notify();
                    })),
            )
            .when_some(detail, |line, detail| {
                line.child(kit::text(theme::Type::Small, c.muted, detail))
            }),
    )
    .into_any_element()
}

/// The held NPC's card (egui: `lookup::generator_card`): its name and attributes, the names
/// help with `Copy the marker`, the tables it used and their problems, the file it will save,
/// then Save NPC, Reroll and Discard.
fn card_view(
    card: crate::world::page::CardView,
    run_open: bool,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let act_on = |cx: &mut Context<SplatRoot>, card_act: CardAct| {
        act::<gpui::ClickEvent>(cx, move |root, campaign, index, _, cx| {
            if card_act == CardAct::CopiedMarker {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                    crate::world::model::NAMES_MARKER.to_owned(),
                ));
            }
            root.app.world_page.card_act(card_act, campaign, index);
        })
    };
    let mut column = div()
        .flex()
        .flex_col()
        .gap(theme::XS)
        .max_w(theme::READER_MEASURE)
        .child(kit::heading(
            kit::Heading::Group {
                title: "Generated NPC",
                count: 0,
            },
            true,
        ))
        .child(kit::text(theme::Type::Strong, c.text, card.npc.name.clone()));
    for (label, words) in splat_content_ui::npc_generator::ATTRIBUTES.iter().zip(&card.npc.attributes) {
        column = column.child(kit::text(theme::Type::Body, c.text, format!("{label}: {words}")));
    }
    if card.sources.is_empty() && card.problems.is_empty() {
        column = column.child(kit::help(crate::lookup::OWN_NAMES_HELP)).child(
            kit::Button::quiet("world-copy-marker", crate::lookup::COPY_MARKER)
                .density(theme::Density::Read)
                .line_start()
                .hint(crate::world::model::NAMES_MARKER)
                .on_click(act_on(cx, CardAct::CopiedMarker)),
        );
    }
    for (at, source) in card.sources.iter().enumerate() {
        let (kind, path) = source.split_once(": ").unwrap_or(("npc-name", source));
        let label = if kind == "npc-name" {
            "Names".to_owned()
        } else {
            crate::design::reader::human_label(kind)
        };
        let path = path.to_owned();
        column = column.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(theme::XS)
                .child(kit::text(theme::Type::Small, c.muted, format!("{label} from")))
                .child(kit::Link::new(("world-card-source", at), path.clone()).on_click(
                    cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                        root.app.world_page.ask_lookup = Some(path.clone());
                        cx.notify();
                    }),
                )),
        );
    }
    for (at, problem) in card.problems.iter().enumerate() {
        let path = problem.path.clone();
        column = column.child(
            kit::Message::new(kit::Tone::Warning, problem.message.clone()).action(
                kit::Button::secondary(("world-card-problem", at), format!("Open {}", problem.path))
                    .density(theme::Density::Read)
                    .hint(problem.marker.clone())
                    .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                        root.app.world_page.ask_lookup = Some(path.clone());
                        cx.notify();
                    })),
            ),
        );
    }
    let save = if run_open {
        kit::Button::secondary("world-card-save", "Save NPC")
    } else {
        kit::Button::primary("world-card-save", "Save NPC")
    };
    column
        .child(kit::text(
            theme::Type::Small,
            c.muted,
            format!("Will save {}", card.save_path),
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(theme::S)
                .child(
                    save.density(theme::Density::Read)
                        .on_click(act_on(cx, CardAct::Save)),
                )
                .child(
                    kit::Button::secondary("world-card-again", "Reroll")
                        .icon(icons::DICE)
                        .density(theme::Density::Read)
                        .on_click(act_on(cx, CardAct::Again)),
                )
                .child(
                    kit::Button::quiet("world-card-discard", "Discard")
                        .density(theme::Density::Read)
                        .on_click(act_on(cx, CardAct::Discard)),
                ),
        )
        .into_any_element()
}

/// The checks views: Open questions (with a box to tick each) and Things to check.
fn checks(
    root: &mut SplatRoot,
    campaign: &Campaign,
    index: &ContentIndex,
    view: View,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let lines = root.app.world_page.footer_plain(None, index, campaign, |_, _| {});
    let footer = footer::render(root, lines, cx);
    let c = theme::colors();
    let Some(checks) = root.app.world_page.checks() else {
        return pane("world-checks", footer, div());
    };
    const ROWS: usize = 40;
    let note = |words: String| kit::text(theme::Type::Small, c.muted, words);
    let open = |cx: &mut Context<SplatRoot>, page: usize, heading: Option<String>| {
        act::<gpui::ClickEvent>(cx, move |root, _, index, _, _| {
            root.app.world_page.press_open(index, page, heading.clone());
        })
    };
    let mut column = div().flex().flex_col().gap(theme::XS).max_w(theme::READER_MEASURE);
    // One page's lines, then its name.
    let by_page = |column: gpui::Div,
                   found: &[crate::world::checks::Found],
                   questions: bool,
                   cx: &mut Context<SplatRoot>| {
        let mut column = column;
        let mut place = 0;
        let groups: Vec<_> = found.chunk_by(|a, b| a.page == b.page).collect();
        for group in groups.iter().take(ROWS) {
            for line in *group {
                let link = kit::Link::new(("world-check-line", place), line.words.clone())
                    .on_click(open(cx, line.page, line.heading.clone()));
                let row = if questions {
                    let editable = line.open_box.is_some()
                        && crate::world::edit::editable(&index.items[line.page], index);
                    let at = place;
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .child(div().w(theme::XXL).flex_none().when(editable, |slot| {
                            slot.child(kit::target(
                                div().child(
                                    kit::Checkbox::new(("world-tick", at), "", false).on_click(
                                        act(cx, move |root, campaign, index, _, _| {
                                            root.app.world_page.press_tick(index, campaign, at);
                                        }),
                                    ),
                                ),
                                format!("Open: {}", line.words),
                            ))
                        }))
                        .child(link)
                        .into_any_element()
                } else {
                    link.into_any_element()
                };
                column = column.child(row);
                place += 1;
            }
            let page = group[0].page;
            column = column.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .when(questions, |line| line.pl(theme::XXL))
                    .child(note("In ".to_owned()))
                    .child(
                        kit::Link::new(("world-check-in", page), index.items[page].entry.name.clone())
                            .small()
                            .on_click(open(cx, page, None)),
                    )
                    .pb(theme::S),
            );
        }
        if groups.len() > ROWS {
            column = column.child(note(crate::design::count(groups.len() - ROWS, "more", "more")));
        }
        column
    };
    if view == View::Questions {
        column = column
            .child(kit::text(theme::Type::Display, c.text, "Open questions"))
            .child(note(
                "Each unticked task line in the world’s notes. Tick a question when you decide."
                    .to_owned(),
            ))
            .child(kit::gap(theme::S));
        if checks.questions.is_empty() {
            column = column.child(note("No open questions.".to_owned()));
        }
        column = by_page(column, &checks.questions, true, cx);
        return pane("world-questions", footer, column);
    }
    column = column
        .child(kit::text(
            theme::Type::Display,
            c.text,
            crate::world::page::PROBLEMS_TITLE,
        ))
        .child(note("These lists show notes with a date outside an event, a link to no note, no summary, no links, or a name or sentence on two notes. Splat only lists them. You change the notes.".to_owned()));
    let pages = |cx: &mut Context<SplatRoot>, pages: &[usize]| {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap_x(theme::S)
            .children(pages.iter().map(|&page| {
                kit::Link::new(("world-check-page", page), index.items[page].entry.name.clone())
                    .on_click(open(cx, page, None))
                    .into_any_element()
            }))
    };
    if !checks.loose_dates.is_empty() {
        column = column
            .child(kit::heading(kit::Heading::Section("Dates outside events"), false))
            .child(note("A date in words belongs on an event’s note. An event has a date that Splat can put in order.".to_owned()));
        column = by_page(column, &checks.loose_dates, false, cx);
    }
    if !checks.wanted.is_empty() {
        column = column
            .child(kit::heading(kit::Heading::Section("Pages to write"), false))
            .child(note("A link names each of these, and no note has the name. The note that most lines want is first.".to_owned()));
        for (name, lines, from) in checks.wanted.iter().take(ROWS) {
            column = column.child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap_x(theme::S)
                    .child(kit::text(theme::Type::Body, c.text, name.clone()))
                    .child(note(format!("{} in", crate::design::count(*lines, "line", "lines"))))
                    .child(pages(cx, from)),
            );
        }
        if checks.wanted.len() > ROWS {
            column = column.child(note(crate::design::count(checks.wanted.len() - ROWS, "more", "more")));
        }
    }
    for (title, words, found) in [
        (
            "No summary",
            "Start each of these with one paragraph that says what the subject is. Lists and Lookup show it.",
            &checks.no_summary,
        ),
        (
            "Nothing points here",
            "These notes show on no other note. No line names them, they name no other note, and they are in no place and part of no group.",
            &checks.alone,
        ),
    ] {
        if !found.is_empty() {
            column = column
                .child(kit::heading(kit::Heading::Section(title), false))
                .child(note(words.to_owned()))
                .child(pages(cx, &found[..found.len().min(ROWS)]));
            if found.len() > ROWS {
                column = column.child(note(crate::design::count(found.len() - ROWS, "more", "more")));
            }
        }
    }
    for (title, words, found) in [
        (
            "One name on two notes",
            "A line that uses the name shows on each of these notes.",
            &checks.shared_names,
        ),
        (
            "Said twice",
            "The same sentence is on each of these notes. Keep it on the note of its subject.",
            &checks.repeated,
        ),
    ] {
        if !found.is_empty() {
            column = column
                .child(kit::heading(kit::Heading::Section(title), false))
                .child(note(words.to_owned()));
            for (said, on) in found.iter().take(ROWS) {
                column = column.child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .gap_x(theme::S)
                        .child(kit::text(theme::Type::Small, c.text, said.clone()))
                        .child(pages(cx, on)),
                );
            }
            if found.len() > ROWS {
                column = column.child(note(crate::design::count(found.len() - ROWS, "more", "more")));
            }
        }
    }
    pane("world-problems", footer, column)
}

/// A note that the system will not read: its file, how to recover it, `Try again` and
/// `Close note`, and the system's words in Technical details. Nothing here writes the file.
fn unread(
    root: &mut SplatRoot,
    campaign: &Campaign,
    index: &ContentIndex,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let Some(view) = root.app.world_page.unread_view(index, campaign) else {
        return div().into_any_element();
    };
    let path = campaign.path.join(&view.path);
    let show = crate::world::model::file_link(&path);
    let folder = view
        .path
        .rsplit_once('/')
        .map(|(folder, _)| format!("{folder}/"));
    let column = div()
        .flex()
        .flex_col()
        .gap(theme::S)
        .child(kit::text(theme::Type::Display, c.text, view.file))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::S)
                .when_some(folder, |line, folder| {
                    line.child(kit::text(theme::Type::Small, c.muted, folder))
                })
                .child(
                    kit::Link::new("world-unread-show", "Show file")
                        .hint(path.display().to_string())
                        .on_click(move |_, _, cx| cx.open_url(&show)),
                ),
        )
        .child(
            div().max_w(theme::READER_MEASURE + px(40.0)).child(
                kit::Message::new(kit::Tone::Error, view.words)
                    .action(
                        kit::Button::secondary("world-unread-again", "Try again")
                            .density(theme::Density::Read)
                            .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                                root.app.world_page.unread_again();
                                cx.notify();
                            })),
                    )
                    .action(
                        kit::Button::secondary("world-unread-close", "Close note")
                            .density(theme::Density::Read)
                            .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                                root.app.world_page.cancel_error();
                                cx.notify();
                            })),
                    ),
            ),
        )
        .child(
            kit::QuietSection::new("world-unread-details", "Technical details")
                .child(kit::text(theme::Type::Small, c.muted, view.system)),
        );
    pane("world-unread", Vec::new(), column)
}

/// The repair of a note that cannot be read: its file, then the source editor and its save row,
/// or, when the file changed under the draft, the two versions and the choice.
fn repair(root: &mut SplatRoot, campaign: &Campaign, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = theme::colors();
    root.pages.world.repair_presses.clear();
    let run_open = root.app.world_page.run_open();
    let Some(repair) = root.app.world_page.repair_mut() else {
        return div().into_any_element();
    };
    let view = repair.view(campaign);
    let lines = if view.disk.is_none() {
        repair.footer_plain(campaign)
    } else {
        Vec::new()
    };
    let mut footer = Vec::new();
    for (at, line) in lines.into_iter().enumerate() {
        let mut message = kit::Message::new(tone_of(line.tone), line.words);
        if let Some(undo) = line.undo {
            let press = root.pages.world.repair_presses.keep(undo.press);
            message = message.action(
                kit::Button::quiet(("world-repair-undo", at), undo.label)
                    .icon(icons::UNDO)
                    .density(theme::Density::Read)
                    .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                        if let Some(press) = root.pages.world.repair_presses.take(press)
                            && let Some(repair) = root.app.world_page.repair_mut()
                        {
                            repair.press_footer(press);
                        }
                        cx.notify();
                    })),
            );
        }
        footer.push(message.into_any_element());
    }
    let path = campaign.path.join(&view.path);
    let (edit, show) = (
        crate::world::model::obsidian_link(&path),
        crate::world::model::file_link(&path),
    );
    let folder = view
        .path
        .rsplit_once('/')
        .map(|(folder, _)| format!("{folder}/"));
    let mut column = div()
        .flex()
        .flex_col()
        .gap(theme::S)
        .child(kit::text(theme::Type::Display, c.text, view.file.clone()))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::S)
                .when_some(folder, |line, folder| {
                    line.child(kit::text(theme::Type::Small, c.muted, folder))
                })
                .child(
                    kit::Link::new("world-repair-edit", "Edit in Obsidian")
                        .hint(path.display().to_string())
                        .on_click(move |_, _, cx| cx.open_url(&edit)),
                )
                .child(
                    kit::Link::new("world-repair-show", "Show file")
                        .hint(path.display().to_string())
                        .on_click(move |_, _, cx| cx.open_url(&show)),
                ),
        );
    let choose = |cx: &mut Context<SplatRoot>, choice: DiskConflictChoice| {
        act::<gpui::ClickEvent>(cx, move |root, campaign, _, _, _| {
            if let Some(repair) = root.app.world_page.repair_mut()
                && repair.press_choice(campaign, choice)
            {
                root.app.world_page.repair_saved(campaign);
            }
        })
    };
    if let Some(disk) = view.disk.clone() {
        column = column
            .child(conflict_message(
                "This note changed on disk. Your draft is safe until you choose.",
                choose(cx, DiskConflictChoice::Overwrite),
                choose(cx, DiskConflictChoice::Reload),
            ))
            .child(compare(&view.text, &disk));
        return pane("world-repair", footer, column);
    }
    let save = act::<gpui::ClickEvent>(cx, |root, campaign, _, _, _| {
        if let Some(repair) = root.app.world_page.repair_mut()
            && repair.press_save(campaign)
        {
            root.app.world_page.repair_saved(campaign);
        }
    });
    let reason = view
        .problem
        .clone()
        .or_else(|| (!view.dirty).then(|| crate::design::save_row::NO_CHANGES.to_owned()));
    column = column
        .child(
            div().max_w(theme::LIST_MEASURE).h(px(360.0)).child(
                kit::TextEditor::new(
                    gpui::ElementId::Name(format!("world-repair-text-{}", view.path).into()),
                    view.text,
                )
                    .name("Repair text")
                    .rows(14)
                    .mono()
                    .fill()
                    .on_change(on_words(cx, |root, words| {
                        if let Some(repair) = root.app.world_page.repair_mut() {
                            repair.set_text(words);
                        }
                    })),
            ),
        )
        .child(save_row(
            SaveRow {
                dirty: view.dirty,
                failed: view.failed,
                primary: !run_open,
                save: "Save repaired note",
                blocked: reason,
                lock: None,
                cancel: None,
            },
            save,
            cx.listener(|root: &mut SplatRoot, _, _, cx| {
                if let Some(repair) = root.app.world_page.repair_mut() {
                    repair.press_discard();
                }
                cx.notify();
            }),
        ));
    pane("world-repair", footer, column.max_w(theme::LIST_MEASURE))
}

/// A design tone as the kit's.
fn tone_of(tone: crate::design::Tone) -> kit::Tone {
    match tone {
        crate::design::Tone::Info => kit::Tone::Info,
        crate::design::Tone::Success => kit::Tone::Success,
        crate::design::Tone::Warning => kit::Tone::Warning,
        crate::design::Tone::Error => kit::Tone::Error,
    }
}

/// A disk conflict's message and its two choices: keep the draft, or reload and lose it.
fn conflict_message(
    words: impl Into<SharedString>,
    overwrite: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
    reload: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> AnyElement {
    kit::Message::new(kit::Tone::Warning, words)
        .action(
            kit::Button::secondary("world-conflict-overwrite", "Overwrite with my draft")
                .density(theme::Density::Read)
                .on_click(overwrite),
        )
        .destructive(
            kit::Button::destructive("world-conflict-reload", "Reload and lose my draft")
                .density(theme::Density::Read)
                .on_click(reload),
        )
        .into_any_element()
}

/// The draft beside the file on disk.
fn compare(mine: &str, theirs: &str) -> AnyElement {
    let c = theme::colors();
    let side = |title: &'static str, words: &str| {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap(theme::XS)
            .child(kit::label(title, 0))
            .child(
                div()
                    .p(theme::S)
                    .bg(c.well)
                    .rounded(theme::RADIUS_CONTROL)
                    .child(kit::text(theme::Type::Mono, c.text, words.to_owned())),
            )
    };
    div()
        .flex()
        .flex_row()
        .gap(theme::M)
        .child(side("Your draft", mine))
        .child(side("On disk", theirs))
        .into_any_element()
}

/// A save row's facts (`design::save_row`).
struct SaveRow {
    dirty: bool,
    failed: bool,
    primary: bool,
    save: &'static str,
    /// Why Save waits.
    blocked: Option<String>,
    /// The lock's sentence while the draft has changes (`Save or discard to switch campaign.`).
    lock: Option<&'static str>,
    /// Cancel's reason while the form is clean, when Cancel waits.
    cancel: Option<Option<&'static str>>,
}

/// The save row: the draft's state word, Discard (or a clean form's Cancel), the reason, and
/// Save at the row's end.
fn save_row(
    row: SaveRow,
    save: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
    discard: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> AnyElement {
    let c = theme::colors();
    let (icon, word, color) = if row.failed {
        (icons::ERROR, "Not saved", c.result.error)
    } else if row.dirty {
        (icons::DRAFT, "Unsaved", c.document.unsaved)
    } else {
        (icons::OK, "No changes", c.muted)
    };
    let save = if row.primary && !row.failed {
        kit::Button::primary("world-save", row.save)
    } else {
        kit::Button::secondary("world-save", row.save)
    }
    .key("Ctrl+S")
    .blocked(row.blocked.clone())
    .on_click(save);
    let mut line = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::S)
        .min_h(theme::Density::Scan.control())
        .child(kit::status(&kit::Status::new(icon, word, color)));
    match (row.dirty, row.cancel) {
        (false, Some(reason)) => {
            line = line.child(
                kit::Button::quiet("world-cancel", "Cancel")
                    .blocked(reason)
                    .on_click(discard),
            );
        }
        (true, _) => {
            line = line.child(kit::Button::quiet("world-discard", "Discard").on_click(discard));
        }
        (false, None) => {}
    }
    let reason = if row.dirty {
        row.lock.map(str::to_owned).or(row.blocked)
    } else {
        None
    };
    line.when_some(reason, |line, reason| {
        line.child(kit::text(theme::Type::Small, c.muted, reason))
    })
    .child(div().flex_1())
    .child(save)
    .into_any_element()
}

/// The edit form, in place of the note's rows and text: Stat block, Place and Part of with
/// their Change, Status, Text, then the save row.
pub(super) fn edit_form(
    root: &mut SplatRoot,
    item: &Item,
    index: &ContentIndex,
    campaign: &Campaign,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let id = item.entry.id;
    let Some(view) = root.app.world_page.reader.draft_view(item, index, campaign) else {
        return div().into_any_element();
    };
    let on_note = move |cx: &mut Context<SplatRoot>,
                        f: fn(&mut crate::world::page::WorldPage, &Item, &ContentIndex, &Campaign)| {
        act::<gpui::ClickEvent>(cx, move |root, campaign, index, _, _| {
            if let Some(at) = index.position(id) {
                f(&mut root.app.world_page, &index.items[at], index, campaign);
            }
        })
    };
    let mut form = div().flex().flex_col().pt(theme::M).max_w(theme::FORM_WIDTH);
    if let Some(conflict) = &view.conflict {
        let choose = |cx: &mut Context<SplatRoot>, choice: DiskConflictChoice| {
            act::<gpui::ClickEvent>(cx, move |root, campaign, index, _, _| {
                if let Some(at) = index.position(id) {
                    let page = &mut root.app.world_page;
                    if let Some(nav) =
                        page.reader
                            .resolve_conflict(&index.items[at], index, campaign, choice)
                    {
                        page.take_nav(nav, index);
                    }
                }
            })
        };
        form = form.child(conflict_message(
            conflict.words.clone(),
            choose(cx, DiskConflictChoice::Overwrite),
            choose(cx, DiskConflictChoice::Reload),
        ));
        for (label, words) in &conflict.effects {
            form = form.child(
                div()
                    .flex()
                    .flex_row()
                    .gap(theme::S)
                    .child(
                        div()
                            .w(px(104.0))
                            .flex_none()
                            .child(kit::text(theme::Type::Small, c.muted, label.clone())),
                    )
                    .child(kit::text(theme::Type::Small, c.text, words.clone())),
            );
        }
        if let Some((mine, theirs)) = &conflict.texts {
            form = form.child(div().pt(theme::S).child(compare(mine, theirs)));
        }
        return form.into_any_element();
    }
    let value = |value: &DraftValue| match value {
        DraftValue::Set(words) => kit::text(theme::Type::Body, c.text, words.clone()),
        DraftValue::NotSet => kit::text(theme::Type::Small, c.muted, "not set"),
        DraftValue::NotRead => kit::text(theme::Type::Small, c.result.warning, "not read"),
    };
    let change = |cx: &mut Context<SplatRoot>, key: &'static str, label: &str, blocked: Option<String>| {
        kit::Button::quiet(
            gpui::ElementId::Name(format!("world-draft-change-{key}").into()),
            "Change",
        )
            .density(theme::Density::Read)
            .name(format!("Change {label}"))
            .blocked(blocked)
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                root.app.world_page.reader.press_draft_change(key);
                cx.notify();
            }))
    };
    let picker_key = root.app.world_page.reader.picker_view().map(|view| view.key);
    let mut rows: Vec<(&'static str, &'static str, &DraftValue, Option<String>)> = Vec::new();
    if let Some(creature) = &view.creature {
        rows.push(("creature", "Stat block", creature, None));
    }
    rows.push(("in", "Place", &view.place, None));
    rows.push((
        "part_of",
        "Part of",
        &view.groups,
        view.groups_error.as_ref().map(|(reason, _)| reason.clone()),
    ));
    for (key, label, shown, blocked) in rows {
        form = form.child(kit::field(
            label,
            None,
            None,
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::L)
                .child(value(shown))
                .child(change(cx, key, label, blocked)),
        ));
        if key == "part_of"
            && let Some((reason, whole)) = &view.groups_error
        {
            let url = crate::world::model::obsidian_link(&campaign.path.join(&view.path));
            form = form.child(
                div().pb(theme::S).child(
                    kit::Message::new(kit::Tone::Warning, reason.clone())
                        .action(
                            kit::Button::secondary("world-draft-unread-edit", "Edit in Obsidian")
                                .density(theme::Density::Read)
                                .hint(whole.clone())
                                .on_click(move |_, _, cx| cx.open_url(&url)),
                        ),
                ),
            );
        }
        if picker_key == Some(key) {
            form = form.child(super::reader::picker(root, item, index, true, cx));
        }
    }
    form = form
        .child(kit::field(
            "Status",
            None,
            None,
            kit::TextField::new(("world-draft-status", id.as_u128() as u64), view.status.clone())
                .placeholder(crate::world::edit::STATUS_HINT)
                .name("Status")
                .on_change(on_words(cx, move |root, words| {
                    root.app.world_page.reader.set_draft_status(id, words);
                })),
        ))
        .child(kit::field(
            "Text",
            None,
            None,
            div().h(px(194.0)).child(
                kit::TextEditor::new(("world-draft-text", id.as_u128() as u64), view.text.clone())
                    .name("Text")
                    .rows(8)
                    .fill()
                    .on_change(on_words(cx, move |root, words| {
                        root.app.world_page.reader.set_draft_text(id, words);
                    })),
            ),
        ));
    let cancel_reason = view
        .log_waits
        .then_some(crate::world::model::LOG_WAITS_FOR_CANCEL_WORDS);
    form.child(save_row(
        SaveRow {
            dirty: view.dirty,
            failed: false,
            primary: !view.live,
            save: "Save changes",
            blocked: if view.log_waits {
                Some(crate::world::LOG_WAITS_FOR_SAVE.to_owned())
            } else if !view.dirty {
                Some(crate::design::save_row::NO_CHANGES.to_owned())
            } else {
                None
            },
            lock: Some("Save or discard to switch campaign."),
            cancel: Some(cancel_reason),
        },
        on_note(cx, |page, item, index, campaign| {
            if let Some(nav) = page.reader.save_draft(item, index, campaign) {
                page.take_nav(nav, index);
            }
        }),
        cx.listener(move |root: &mut SplatRoot, _, _, cx| {
            root.app.world_page.reader.discard_draft(id);
            cx.notify();
        }),
    ))
    .into_any_element()
}
