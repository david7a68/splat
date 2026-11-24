//! The open event's form (egui: `TimelinePage::detail`): its title line (the kind, the title, Copy
//! link, the save state and the `⋯` menu with `Roll for encounters` and `Remove`), its Title, then
//! the form that scrolls: Kind, How long (with a journey's encounter checks under it), the rest
//! ruling, Starts and Ends, Where (with the locations it could be), Session, From the run,
//! Notes, the file's other fields, Link text, the notes that mention it, and Place after.
use super::checks;
use crate::gui::theme::{Density, L, M, S, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use crate::timeline::page::{
    DetailActs, PlaceHelp, event_labels, saving_blocked, saving_now, saving_states,
};
use crate::timeline::{EventDisplay as _, Kind, clock, clock_words, day_numbers, reference};
use gpui::{
    AnyElement, ClipboardItem, Context, ElementId, IntoElement, ParentElement, SharedString,
    Styled, Window, div, prelude::*, px,
};
use std::time::{Duration, Instant};

/// How long `Copied` shows after Copy link.
const COPIED_FOR: Duration = Duration::from_secs(2);

/// An element id of a part of the form of the event `id`.
fn part(id: &str, name: &str) -> ElementId {
    ElementId::Name(format!("timeline-event-{id}-{name}").into())
}

/// Apply what the form did (`TimelinePage::apply_detail`), then draw again.
pub(crate) fn apply(root: &mut SplatRoot, id: &str, acts: DetailActs, cx: &mut Context<SplatRoot>) {
    let sessions = root.app.sessions.links();
    let app = &mut root.app;
    app.timeline_page
        .apply_detail(&mut app.timeline, &mut app.sessions, &sessions, id, acts);
    cx.notify();
}

/// Change the open event `id` with `edit`, as a field's edit: the book saves after its pause.
fn edit(root: &mut SplatRoot, id: &str, edit: impl FnOnce(&mut crate::timeline::Event)) {
    let book = &mut root.app.timeline;
    if let Some(event) = book.event_mut(id) {
        edit(event);
        book.touch();
    }
}

/// The form of the open event `id`.
pub(crate) fn render(
    root: &mut SplatRoot,
    id: &str,
    width: gpui::Pixels,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let Some(index) = root.app.timeline.events.iter().position(|e| e.id == id) else {
        return div().into_any_element();
    };
    crate::timeline::checks::settle_length(&mut root.app.timeline, Some(index));
    let page = &mut root.app.timeline_page;
    page.checks_done.follow(&root.app.timeline);
    page.copied = page
        .copied
        .take()
        .filter(|(_, at)| at.elapsed() < COPIED_FOR);
    let copied = page.copied.as_ref().is_some_and(|(copied, _)| copied == id);
    if std::mem::take(&mut page.reset_detail) || std::mem::take(&mut root.pages.timeline.form_top) {
        root.pages
            .timeline
            .form
            .set_offset(gpui::point(px(0.0), px(0.0)));
    }
    let focus_title = std::mem::take(&mut root.app.timeline_page.focus_title);
    let focus_length = std::mem::take(&mut root.app.timeline_page.focus_length);
    let book = &root.app.timeline;
    let days = day_numbers(&book.events);
    let event = &book.events[index];
    let title = event.label(days[index]);
    let link = reference(event, days[index]);
    let open_run = event.run.is_some() && event.run == root.app.timeline_page.open_run;
    let tables = root
        .app
        .timeline_page
        .tables
        .clone()
        .map(|sources| sources.tables.clone())
        .unwrap_or_default();
    // The title line: the kind before the title, Copy link, the save state, the `⋯` menu.
    let saving = saving_states();
    let now = kit::Status::of(&saving[saving_now(book, id)]);
    let copy = {
        let link = link.clone();
        let owner = id.to_owned();
        kit::Button::secondary(
            part(id, "copy"),
            if copied { "Copied" } else { "Copy link" },
        )
        .icon(if copied { icons::OK } else { icons::COPY })
        .density(Density::Read)
        .on_click(cx.listener(move |root, _, _, cx| {
            cx.write_to_clipboard(ClipboardItem::new_string(link.clone()));
            root.app.timeline_page.copied = Some((owner.clone(), Instant::now()));
            root.ctx.request_repaint_after(COPIED_FOR);
            cx.notify();
        }))
    };
    let mut menu = kit::MenuButton::new(
        part(id, "menu"),
        kit::Button::quiet(part(id, "more"), "")
            .icon(icons::EXPAND)
            .name(format!("More actions for {title}"))
            .density(Density::Read),
    );
    if crate::timeline::checks::offered(event, &tables) {
        let owner = id.to_owned();
        menu = menu.item(
            kit::MenuItem::new("Roll for encounters").on_click(cx.listener(
                move |root, _, _, cx| {
                    let acts = DetailActs {
                        checks: Some(crate::timeline::checks::Act::Start),
                        ..DetailActs::default()
                    };
                    apply(root, &owner, acts, cx);
                },
            )),
        );
    }
    let owner = id.to_owned();
    menu = menu.item(
        kit::MenuItem::new("Remove")
            .blocked(open_run.then_some("This run is open. Close it first."))
            .on_click(cx.listener(move |root, _, _, cx| {
                let acts = DetailActs {
                    removed: true,
                    ..DetailActs::default()
                };
                apply(root, &owner, acts, cx);
            })),
    );
    // The kind before the title: its icon alone for a day or an untitled event.
    let kind_word = crate::timeline::page::header_kind(event);
    let kind_icon = crate::timeline::kind_icon(event.kind);
    // The title line keeps its title, its save state and its menu on one line; Copy link takes
    // a line of its own under them when the column has no room for it there.
    let measure = crate::gui::pages::trackers::measure::width;
    let status_room = saving
        .iter()
        .map(|state| measure(window, Type::Small, state.word))
        .fold(px(0.0), gpui::Pixels::max)
        + crate::gui::theme::ICON_SLOT;
    let need = measure(window, Type::Small, kind_word)
        + crate::gui::theme::ICON_SLOT
        + measure(window, Type::Heading, &title)
        + crate::gui::pages::trackers::measure::button_width(
            window,
            if copied { "Copied" } else { "Copy link" },
            true,
        )
        + status_room
        + Density::Read.control()
        + S * 5.0;
    let (copy_inline, copy_below) = if need > width - M {
        (None, Some(copy))
    } else {
        (Some(copy), None)
    };
    let heading = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_h(Density::Read.control())
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(XS)
                .min_w_0()
                .flex_1()
                .child(kit::icon(kind_icon, Type::Small, c.muted))
                .when(!kind_word.is_empty(), |line| {
                    line.child(kit::text(Type::Small, c.muted, kind_word).flex_none())
                })
                .child(kit::text::one_line(Type::Heading, c.text, title.clone()).min_w_0()),
        )
        .children(copy_inline)
        .child(div().flex_none().w(status_room).child(kit::status(&now)))
        .child(menu);
    let mut header = div().flex().flex_col().gap(XS).pr(M).child(heading);
    if let Some(copy) = copy_below {
        header = header.child(div().flex().flex_row().child(copy));
    }
    if let Some(reason) = saving_blocked(book) {
        header = header.child(kit::text(Type::Small, c.muted, reason));
    }
    let owner = id.to_owned();
    let title_field = kit::TextField::new(part(id, "title"), event.title.clone())
        .name("Title")
        .focus(focus_title)
        .on_change(cx.listener(move |root, words: &str, _, cx| {
            let words = words.to_owned();
            edit(root, &owner, |event| event.title = words);
            cx.notify();
        }));
    header = header
        .child(kit::rule())
        .child(kit::gap(XS))
        .child(kit::field(
            "Title",
            (event.kind == Kind::Day)
                .then(|| "Optional. Days number themselves, so a name is extra.".into()),
            None,
            title_field,
        ))
        .child(kit::rule());
    let form = form(root, id, index, focus_length, window, cx);
    // Review of #817 (P3): the title line stays above the form only while it leaves the form at
    // least one control row; below that it scrolls with the form, so the form always has room.
    // Its height and the column's are the last frame's.
    let room = root.pages.timeline.detail_room.clone();
    let (header_height, column_height) = room.get();
    let scrolls =
        column_height > px(0.0) && column_height - header_height < Density::Read.control();
    let header = {
        let room = room.clone();
        div().relative().child(header).child(
            gpui::canvas(
                move |bounds, window, _| {
                    let (old, column) = room.get();
                    if old != bounds.size.height {
                        room.set((bounds.size.height, column));
                        window.refresh();
                    }
                },
                |_, _, _, _| {},
            )
            .absolute()
            .top_0()
            .left_0()
            .size_full(),
        )
    };
    let column_probe = gpui::canvas(
        move |bounds, window, _| {
            let (header, old) = room.get();
            if old != bounds.size.height {
                room.set((header, bounds.size.height));
                window.refresh();
            }
        },
        |_, _, _, _| {},
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full();
    let column = div().relative().flex().flex_col().size_full();
    let column =
        if scrolls {
            column.child(kit::scroll::scroll_column_with(
                part(id, "form"),
                &root.pages.timeline.form,
                div().child(header).child(div().pr(M).pt(M).child(form)),
            ))
        } else {
            column.child(header).child(div().flex_1().min_h_0().child(
                kit::scroll::scroll_column_with(
                    part(id, "form"),
                    &root.pages.timeline.form,
                    div().pr(M).pt(M).child(form),
                ),
            ))
        };
    column.child(column_probe).into_any_element()
}

/// The form under the title line, which scrolls.
fn form(
    root: &mut SplatRoot,
    id: &str,
    index: usize,
    focus_length: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let sessions = root.app.sessions.links();
    let book = &root.app.timeline;
    let event = &book.events[index];
    let starts = clock(&book.events)[index];
    let mut form = div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(crate::gui::theme::FORM_WIDTH);
    // Kind.
    let day_start = starts
        .filter(|_| event.kind == Kind::Day)
        .map(|start| SharedString::from(format!("Starts: {}", clock_words(start))));
    let kinds = Kind::ALL;
    let owner = id.to_owned();
    form = form.child(kit::field(
        "Kind",
        day_start,
        None,
        kit::Dropdown::new(
            part(id, "kind"),
            "Kind of event",
            kinds.iter().map(|kind| kind.word()),
            kinds.iter().position(|kind| *kind == event.kind),
        )
        .on_select(cx.listener(move |root, at: &usize, _, cx| {
            if let Some(kind) = Kind::ALL.get(*at).copied() {
                edit(root, &owner, |event| event.kind = kind);
            }
            cx.notify();
        })),
    ));
    // How long, with the checks to follow it or not.
    let length = book.length.words(event);
    if event.kind != Kind::Day {
        let hint = match event.kind {
            Kind::LongRest => "8 hours",
            Kind::ShortRest => "1 hour",
            _ => "",
        };
        let owner = id.to_owned();
        let field = if crate::timeline::checks::shown(event) {
            let (enter, blur) = (owner.clone(), owner.clone());
            kit::TextField::new(part(id, "length"), length.clone())
                .placeholder(hint)
                .name("How long")
                .focus(focus_length)
                .on_change(cx.listener(move |root, words: &str, _, cx| {
                    let book = &mut root.app.timeline;
                    if let Some(event) = book.events.iter().find(|e| e.id == owner).cloned() {
                        book.length.type_words(&event, words);
                    }
                    cx.notify();
                }))
                .on_enter(crate::gui::pages::trackers::act(cx, move |root| {
                    crate::timeline::checks::commit_length(&mut root.app.timeline, &enter);
                }))
                .on_blur(crate::gui::pages::trackers::act(cx, move |root| {
                    crate::timeline::checks::commit_length(&mut root.app.timeline, &blur);
                }))
        } else {
            // With no checks to follow it, the event changes as the DM types.
            kit::TextField::new(part(id, "length"), event.duration.clone())
                .placeholder(hint)
                .name("How long")
                .focus(focus_length)
                .on_change(cx.listener(move |root, words: &str, _, cx| {
                    let words = words.to_owned();
                    edit(root, &owner, |event| event.duration = words);
                    cx.notify();
                }))
        };
        form = form.child(kit::field(
            "How long",
            Some(crate::timeline::LENGTH_HELP.into()),
            None,
            field,
        ));
    }
    // The rest ruling: the status now in force and the action that changes it.
    if event.kind == Kind::LongRest {
        let status = kit::Status::of(&crate::timeline::page::long_rest_status(event.restores));
        let restores = event.restores;
        let owner = id.to_owned();
        form = form.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(L)
                .pb(S)
                .child(kit::status(&status))
                .child(
                    kit::Button::quiet(
                        part(id, "ruling"),
                        if restores {
                            "Mark not at full"
                        } else {
                            "Mark everyone at full"
                        },
                    )
                    .density(Density::Read)
                    .on_click(cx.listener(move |root, _, _, cx| {
                        let acts = DetailActs {
                            ruling: Some(crate::timeline::page::RestRulingAction::Set(!restores)),
                            ..DetailActs::default()
                        };
                        apply(root, &owner, acts, cx);
                    })),
                ),
        );
    }
    // Starts and Ends; a length the clock cannot count takes Ends' place with its warning.
    if let Some(start) = starts.filter(|_| event.kind != Kind::Day) {
        let mut line = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(S)
            .pb(M)
            .child(kit::text(
                Type::Small,
                c.muted,
                format!("Starts: {}", clock_words(start)),
            ));
        line = match event.time_warning() {
            Some(warning) => line.child(
                div()
                    .id(part(id, "time-warning"))
                    .child(kit::text(Type::Small, c.result.warning, warning))
                    .tooltip(kit::tooltip::tip(crate::timeline::TIME_REPAIR.into())),
            ),
            None => line.child(kit::text(
                Type::Small,
                c.muted,
                format!(
                    "Ends: {}",
                    clock_words(start.saturating_add(event.minutes()))
                ),
            )),
        };
        form = form.child(line);
    }
    // A journey's encounter checks, under its own How long.
    if crate::timeline::checks::shown(event) {
        form = form.child(checks::render(root, id, index, &length, starts, window, cx));
    }
    let book = &root.app.timeline;
    let event = &book.events[index];
    // Where, and the locations it could be.
    let owner = id.to_owned();
    form = form.child(
        kit::field(
            "Where",
            None,
            None,
            kit::TextField::new(part(id, "place"), event.place.clone())
                .name("Where")
                .on_change(cx.listener(move |root, words: &str, _, cx| {
                    let words = words.to_owned();
                    edit(root, &owner, |event| event.place = words);
                    cx.notify();
                })),
        )
        .pb(px(0.0)),
    );
    let mut help = div().flex().flex_row().flex_wrap().gap(S).pb(L);
    match root.app.timeline_page.place_help_of(&event.place) {
        PlaceHelp::None => {}
        PlaceHelp::Exact(place, name) => {
            help = help.child(
                kit::Link::new(part(id, "read-place"), format!("Read {name}")).on_click(
                    cx.listener(move |root, _, _, cx| {
                        root.app.timeline_page.open_place = Some(place);
                        cx.notify();
                    }),
                ),
            );
        }
        PlaceHelp::Close(names) => {
            for (at, name) in names.into_iter().enumerate() {
                let owner = id.to_owned();
                help = help.child(
                    kit::Link::new(part(id, &format!("place-{at}")), name.clone()).on_click(
                        cx.listener(move |root, _, _, cx| {
                            let name = name.clone();
                            edit(root, &owner, |event| event.place = name);
                            cx.notify();
                        }),
                    ),
                );
            }
        }
    }
    form = form.child(help);
    // Session, and a way to its page while its note is there.
    let mut options = vec!["No session".to_owned()];
    options.extend(sessions.iter().map(|session| session.label.clone()));
    let ids: Vec<uuid::Uuid> = sessions.iter().map(|session| session.id).collect();
    let chosen = event
        .session
        .and_then(|session| ids.iter().position(|known| *known == session))
        .map(|at| at + 1)
        .or(event.session.is_none().then_some(0));
    let owner = id.to_owned();
    let choice_ids = ids.clone();
    form = form.child(
        kit::field(
            "Session",
            None,
            None,
            kit::Dropdown::new(
                part(id, "session"),
                "Session of this event",
                options,
                chosen,
            )
            .on_select(cx.listener(move |root, at: &usize, _, cx| {
                let session = at.checked_sub(1).and_then(|at| choice_ids.get(at).copied());
                edit(root, &owner, |event| event.session = session);
                cx.notify();
            })),
        )
        .pb(XS),
    );
    if let Some(session) = event.session.filter(|session| ids.contains(session)) {
        form = form.child(
            kit::Link::new(part(id, "open-session"), "Open this session").on_click(cx.listener(
                move |root, _, _, cx| {
                    root.app.timeline_page.open_session = Some(session);
                    cx.notify();
                },
            )),
        );
    }
    form = form.child(kit::gap(L));
    if !event.result.is_empty() {
        let open = event.run.is_some() && event.run == root.app.timeline_page.open_run;
        form = form.child(kit::field(
            "From the run",
            None,
            None,
            kit::text(
                Type::Body,
                c.text,
                crate::timeline::played_fight_summary(event, open),
            ),
        ));
    }
    let owner = id.to_owned();
    form = form.child(kit::field(
        "Notes",
        None,
        None,
        kit::TextEditor::new(part(id, "notes"), event.notes.clone())
            .name("Notes on this event")
            .rows(3)
            .on_change(cx.listener(move |root, words: &str, _, cx| {
                let words = words.to_owned();
                edit(root, &owner, |event| event.notes = words);
                cx.notify();
            })),
    ));
    for (label, value) in &event.extra {
        if let (Some(label), Some(value)) = (label.as_str(), value.as_str()) {
            form = form.child(kit::text(Type::Body, c.muted, format!("{label}: {value}")));
        }
    }
    // Link text, folded.
    let link = reference(event, day_numbers(&book.events)[index]);
    let link_open = root.pages.timeline.link_open;
    form = form.child(crate::gui::pages::trackers::section_heading(
        part(id, "link-text"),
        "Link text",
        0,
        link_open,
        cx.listener(|root, _, _, cx| {
            root.pages.timeline.link_open = !root.pages.timeline.link_open;
            cx.notify();
        }),
    ));
    if link_open {
        form = form.child(div().ty(Type::Mono).text_color(c.muted).pb(S).child(link));
    }
    // The session notes that mention it.
    let mentions: Vec<_> = sessions
        .iter()
        .filter(|session| session.mentions.iter().any(|m| m == id))
        .collect();
    if !mentions.is_empty() {
        let mut links = div().flex().flex_col().items_start();
        for session in mentions {
            let target = session.id;
            links = links.child(
                kit::Link::new(
                    part(id, &format!("mention-{target}")),
                    session.label.clone(),
                )
                .on_click(cx.listener(move |root, _, _, cx| {
                    root.app.timeline_page.open_session = Some(target);
                    cx.notify();
                })),
            );
        }
        form = form
            .child(kit::gap(M))
            .child(kit::field("Mentioned in", None, None, links));
    }
    // Place after: the event before this one, in a menu of every other event.
    let labels = event_labels(book, &day_numbers(&book.events), &sessions);
    let event_ids: Vec<String> = book.events.iter().map(|event| event.id.clone()).collect();
    let others: Vec<usize> = (0..book.events.len()).filter(|at| *at != index).collect();
    let mut options = vec!["Start of the timeline".to_owned()];
    options.extend(others.iter().map(|at| labels[*at].clone()));
    let chosen = match index.checked_sub(1) {
        None => 0,
        Some(before) => others
            .iter()
            .position(|at| *at == before)
            .map_or(0, |at| at + 1),
    };
    let owner = id.to_owned();
    form = form.child(kit::gap(M)).child(kit::field(
        "Place after",
        None,
        None,
        kit::Dropdown::new(
            part(id, "place-after"),
            "Place this event after",
            options,
            Some(chosen),
        )
        .on_select(cx.listener(move |root, at: &usize, _, cx| {
            if *at == chosen {
                return;
            }
            let after = at
                .checked_sub(1)
                .and_then(|at| others.get(at))
                .map(|at| event_ids[*at].clone());
            let acts = DetailActs {
                placed: Some(after),
                ..DetailActs::default()
            };
            apply(root, &owner, acts, cx);
        })),
    ));
    form.into_any_element()
}
