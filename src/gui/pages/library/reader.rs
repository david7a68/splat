//! The Library's reader pane (`Library::editor` and `editor_pane`): a form in its place, the empty
//! reader, or the open entry: its toolbar, its owner and file line, its save row, then Read (the
//! title, the body's notices, the body) or Edit Markdown (the editor), or a changed file's
//! choices and comparison; the pane's footer under it all.
use super::{footer, forms};
use crate::gui::kit::WithTooltip as _;
use crate::gui::pages::reader::entry::{EntryView, Source};
use crate::gui::theme::{
    Density, HAIRLINE_WIDTH, L, M, READER_MEASURE, S, Type, TypeExt, XS, colors,
};
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::library::model::ReaderView;
use crate::library::view::NoLinks;
use crate::lookup::ContentIndex;
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*, px,
};
use std::rc::Rc;
use std::sync::Arc;

/// The width the reader's lines keep (`view::WIDE_READER`).
const EDITOR_MAX_WIDTH: f32 = crate::library::EDITOR_MAX_WIDTH;

pub(super) fn pane(
    root: &mut SplatRoot,
    index: &Option<Arc<ContentIndex>>,
    width: f32,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let (package_form, new_form) = root.app.library.forms();
    if package_form || new_form {
        // A form has no reader: the page's results sit in the pane's message footer.
        let body = if package_form {
            forms::package_form(root, window, cx)
        } else {
            forms::new_entry(root, window, cx)
        };
        let lines = root.app.library.form_footer(&root.app.campaigns);
        let mut pane = kit::Pane::new("library-form").flush().child(body);
        for line in footer::render(lines, cx) {
            pane = pane.footer(line);
        }
        return pane.into_any_element();
    }
    let subtitle = root.app.library.open_subtitle();
    let view = {
        let app = &mut root.app;
        app.library
            .reader_view(&app.campaigns, index.as_ref(), &subtitle)
    };
    let Some((view, lines)) = view else {
        return empty_reader(root, cx);
    };
    let footer = footer::render(lines, cx);
    entry(root, view, width, footer, index, window, cx)
}

/// No entry open: the empty reader, with the page's results in its footer.
fn empty_reader(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let campaigns = &root.app.campaigns;
    let body = if campaigns.active_campaign().is_some() || !campaigns.modules().is_empty() {
        "Choose one from the list, or make a new one."
    } else {
        "Choose one from the list."
    };
    let lines = root.app.library.empty_reader_footer(&root.app.campaigns);
    let mut pane = kit::Pane::new("library-empty-reader")
        .flush()
        .child(kit::empty_state(
            icons::LIBRARY,
            "No entry open",
            body,
            None::<AnyElement>,
        ));
    for line in footer::render(lines, cx) {
        pane = pane.footer(line);
    }
    pane.into_any_element()
}

/// The toolbar: the mode switch, Back, the extras, and the draft's reason on its line.
fn toolbar(view: &ReaderView, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let bar = &view.toolbar;
    let mode = kit::Segmented::new(
        "library-mode",
        ["Read", bar.source_label],
        usize::from(bar.source),
    )
    .on_select(cx.listener(|root, at: &usize, _, cx| {
        root.app.library.set_source(*at == 1);
        cx.notify();
    }));
    let mode = match &bar.mode_blocked {
        Some(reason) => div()
            .id("library-mode-lock")
            .opacity(0.6)
            .with_tooltip(reason.clone())
            .child(mode)
            .into_any_element(),
        None => mode.into_any_element(),
    };
    let mut line = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(S)
        .min_h(Density::Read.control())
        .child(mode);
    if bar.back {
        line = line.child(
            kit::Button::quiet("library-back", "Back")
                .icon(icons::HISTORY_BACK)
                .key("Alt+Left")
                .density(Density::Read)
                .hint("Return to the entry before this link. During a draft, read it in Lookup.")
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.library.back();
                    cx.notify();
                })),
        );
    }
    for (at, (label, act, blocked)) in bar.actions.iter().enumerate() {
        let act = *act;
        line = line.child(
            kit::Button::quiet(ElementId::Name(format!("library-act-{at}").into()), *label)
                .density(Density::Read)
                .blocked(blocked.clone())
                .on_click(cx.listener(move |root, _, _, cx| {
                    let app = &mut root.app;
                    app.library.reader_act(act, &app.campaigns);
                    cx.notify();
                })),
        );
    }
    if let Some(reason) = &bar.draft_reason {
        line = line.child(kit::text(Type::Small, c.muted, reason.clone()));
    }
    div()
        .flex()
        .flex_col()
        .child(line)
        .children(
            bar.reasons
                .iter()
                .map(|reason| kit::text(Type::Small, c.muted, reason.clone())),
        )
        .into_any_element()
}

/// The owner and file line: the owner's status, then the path and its links.
fn file_line(view: &ReaderView) -> AnyElement {
    let c = colors();
    let line = &view.file_line;
    let (icon, words, color) = &line.status;
    let mut row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_w_0()
        .min_h(Density::Read.control())
        .child(kit::status(&kit::Status::new(
            *icon,
            words.clone(),
            theme::color(*color),
        )));
    if let Some((folder, relative)) = &line.file {
        let path = folder.join(relative);
        if line.folder_only {
            let folder = path
                .ancestors()
                .skip(1)
                .find(|folder| folder.is_dir())
                .unwrap_or(folder)
                .to_path_buf();
            if !relative.is_empty() && !line.status.1.contains("deleted") {
                row = row.child(kit::text(Type::Small, c.muted, relative.clone()));
            }
            let url = footer::file_url(&folder);
            row = row.child(
                kit::link("library-show-folder", "Show folder")
                    .on_click(move |_, _, cx| cx.open_url(&url)),
            );
        } else {
            row = row.child(
                div()
                    .id("library-path")
                    .min_w_0()
                    .flex_shrink_1()
                    .with_tooltip(path.display().to_string())
                    .child(kit::text::one_line(Type::Small, c.muted, relative.clone())),
            );
            if let Some(from) = &line.moved_from {
                row = row.child(kit::text(Type::Small, c.muted, from.clone()));
            }
            if line.obsidian {
                let url = footer::obsidian_url(&path);
                row = row.child(
                    kit::link("library-obsidian", "Edit in Obsidian")
                        .on_click(move |_, _, cx| cx.open_url(&url)),
                );
            }
            let url = footer::file_url(&path);
            row = row.child(
                kit::link("library-show-file", "Show file")
                    .on_click(move |_, _, cx| cx.open_url(&url)),
            );
        }
    }
    row.into_any_element()
}

/// The save row: the draft's word, Discard and the reason at the left, Save at the right.
fn save_row(
    view: &ReaderView,
    width: f32,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let row = view.save_row.as_ref()?;
    let c = colors();
    let reason = row
        .reason
        .as_ref()
        .and_then(|(long, short)| row_reason(window, width, long, short))
        .or(row.lock.clone());
    let (icon, word, color) = &row.word;
    let save = if row.primary {
        kit::Button::primary("library-save", "Save")
    } else {
        kit::Button::secondary("library-save", "Save")
    }
    .density(Density::Read)
    .key("Ctrl+S")
    .blocked(row.save_blocked.clone())
    .on_click(cx.listener(|root, _, _, cx| {
        let app = &mut root.app;
        app.library.save(&app.campaigns);
        cx.notify();
    }));
    Some(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .min_h(Density::Scan.control())
            .max_w(px(EDITOR_MAX_WIDTH))
            .child(kit::status(&kit::Status::new(
                SharedString::from(icon.clone()),
                word.clone(),
                theme::color(*color),
            )))
            .when(row.discard, |line| {
                line.child(
                    kit::Button::quiet("library-discard", "Discard").on_click(cx.listener(
                        |root, _, _, cx| {
                            root.app.library.discard();
                            cx.notify();
                        },
                    )),
                )
            })
            .children(reason.map(|reason| kit::text(Type::Small, c.muted, reason).truncate()))
            .child(div().flex_1())
            .child(save)
            .into_any_element(),
    )
}

/// The reason on the save row (`save_row_with`): its long form when it fits whole beside the
/// widest state word, Discard and Save, else its short form, else none. Measured in the type
/// each part is set in, as egui measures them.
fn row_reason(window: &mut Window, width: f32, long: &str, short: &str) -> Option<String> {
    use crate::design::save_row::{InvalidCause, SaveWord};
    let gap = f32::from(S);
    let measure =
        |window: &mut Window, ty: Type, words: &str| f32::from(kit::text::width(window, ty, words));
    let button = |window: &mut Window, words: &str, key: Option<&str>| {
        measure(window, Type::Strong, words)
            + key.map_or(0.0, |key| f32::from(S) + measure(window, Type::Small, key))
            + 2.0 * f32::from(M)
    };
    let word = [
        SaveWord::Invalid(InvalidCause::Incomplete),
        SaveWord::Invalid(InvalidCause::Refused),
        SaveWord::Invalid(InvalidCause::Unreadable),
        SaveWord::New,
        SaveWord::ChangedOnDisk,
        SaveWord::NotSaved,
        SaveWord::Unsaved,
        SaveWord::Saved,
        SaveWord::NoChanges,
    ]
    .into_iter()
    .map(|word| {
        let status = word.status();
        measure(
            window,
            Type::Small,
            &format!("{} {}", status.icon, status.word),
        )
    })
    .fold(0.0, f32::max);
    let room = width.min(EDITOR_MAX_WIDTH)
        - button(window, "Save", Some("Ctrl+S"))
        - word
        - button(window, "Discard", None)
        - 4.0 * gap;
    let fits = |window: &mut Window, words: &str| measure(window, Type::Small, words) <= room;
    if fits(window, long) {
        Some(long.to_owned())
    } else if fits(window, short) {
        Some(short.to_owned())
    } else {
        None
    }
}

/// A notice in the body with its repair.
fn notice(
    at: usize,
    notice: &crate::library::model::BodyNotice,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let mut message = kit::Message::new(footer::tone(notice.tone), notice.words.clone());
    let mut help = None;
    if let Some((words, blocked, words_help)) = &notice.repair {
        let again = words == "Try again";
        message = message.action(
            kit::Button::secondary(
                ElementId::Name(format!("library-repair-{at}").into()),
                words.clone(),
            )
            .density(Density::Read)
            .blocked(blocked.clone())
            .on_click(cx.listener(move |root, _, _, cx| {
                if again {
                    let ctx = root.ctx.clone();
                    root.app.library.read_again(&ctx, &root.app.campaigns);
                } else {
                    root.app.library.repair();
                }
                cx.notify();
            })),
        );
        help = words_help.clone();
    }
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(L)
        .child(message)
        .children(help.map(kit::help))
        .children(notice.details.clone().map(technical_details))
        .into_any_element()
}

/// The system's own words under `Technical details`.
fn technical_details(words: String) -> impl IntoElement {
    let c = colors();
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(kit::text(Type::Small, c.muted, "Technical details"))
        .child(kit::text(Type::Small, c.faint, words))
}

/// The open entry.
fn entry(
    root: &mut SplatRoot,
    view: ReaderView,
    width: f32,
    footer: Vec<AnyElement>,
    index: &Option<Arc<ContentIndex>>,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let mut head = div()
        .flex()
        .flex_col()
        .w_full()
        .flex_none()
        .child(toolbar(&view, cx))
        .child(file_line(&view));
    if let Some(row) = save_row(&view, width, window, cx) {
        head = head.child(row);
    }
    let body: AnyElement = if let Some(disk) = &view.disk {
        disk_body(root, &view, disk, cx)
    } else if view.toolbar.source {
        // One editor state per file, kept by the page, so a repair or a new creature can put
        // the caret where `editor_pane` puts it.
        let key = format!("{:?}-{}", view.key.0, view.key.1);
        let handle = match &root.pages.library.editor {
            Some((kept, handle)) if *kept == key => handle.clone(),
            _ => {
                let handle = kit::EditorHandle::new("", window, cx);
                root.pages.library.editor = Some((key.clone(), handle.clone()));
                handle
            }
        };
        if let Some(focus) = root.app.library.take_editor_focus(view.repair_line) {
            use crate::library::model::EditorFocus;
            handle.show_words(view.draft.clone(), window, cx);
            handle.focus(window, cx);
            match focus {
                EditorFocus::Keep => {}
                EditorFocus::Caret(at) => handle.set_caret(at, cx),
                EditorFocus::Select(range) => handle.select(range, cx),
            }
        }
        let editable = view.editable;
        let editor = kit::TextEditor::new(
            ElementId::Name(format!("library-editor-{key}").into()),
            view.draft.clone(),
        )
        .handle(&handle)
        .name("Markdown editor")
        .mono()
        .fill()
        .on_change(cx.listener(move |root, words: &str, _, cx| {
            if editable {
                root.app.library.edit_draft(words);
                cx.notify();
            }
        }));
        div()
            .size_full()
            .max_w(px(EDITOR_MAX_WIDTH))
            .pt(XS)
            .child(editor)
            .into_any_element()
    } else {
        read_body(root, &view, index, window, cx)
    };
    let choices = (!view.link_choices.is_empty()).then(|| {
        let mut line = div().flex().flex_row().flex_wrap().gap(S);
        for (at, words) in &view.link_choices {
            let at = *at;
            line = line.child(
                kit::link(
                    ElementId::Name(format!("library-link-choice-{at}").into()),
                    words.clone(),
                )
                .on_click(cx.listener(move |root, _, _, cx| {
                    root.app.library.open_link(at);
                    cx.notify();
                })),
            );
        }
        line
    });
    let mut pane = kit::Pane::new(ElementId::Name(
        format!("library-reader-{:?}-{}", view.key.0, view.key.1).into(),
    ))
    .flush()
    .scroll(&root.pages.library.reader_scroll)
    .header(head)
    .children(choices)
    .child(body);
    if view.toolbar.source && view.disk.is_none() {
        pane = pane.fixed();
    }
    for line in footer {
        pane = pane.footer(line);
    }
    pane.into_any_element()
}

/// Read: the title with the entry's picture, the body's notices, then the entry as the reader
/// shows it (egui `view::title_with_art` and `view::show_with_related`): a stat block, a spell's
/// or an item's facts, class features, related entries and the closed end sections. Its related,
/// feature and spell links open their entries here; a note link in its words opens its entry, or
/// offers its choices.
fn read_body(
    root: &mut SplatRoot,
    view: &ReaderView,
    index: &Option<Arc<ContentIndex>>,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let mut head = div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(READER_MEASURE)
        .pt(XS)
        .child(crate::gui::pages::reader::entry::title_with_art(
            &view.title,
            &view.subtitle,
            view.entry.as_ref(),
            cx,
        ));
    if !view.notices.is_empty() {
        head = head.child(kit::gap(L));
    }
    for (at, item) in view.notices.iter().enumerate() {
        head = head.child(notice(at, item, cx));
    }
    let open = cx.listener(|root, at: &usize, _, cx| {
        root.app.library.open_link(*at);
        cx.notify();
    });
    let link_index = index.clone();
    let from = view.entry.as_ref().map(|entry| entry.id);
    let follow = cx.listener(move |root, note: &str, _, cx| {
        if let Some(index) = &link_index
            && let crate::world::Go::Item(_, heading) = index.resolve_link(note, from)
        {
            crate::markdown_view::want_heading(&root.ctx, heading);
        }
        root.app.library.follow_link(note, link_index.as_deref());
        cx.notify();
    });
    let reader = EntryView::new(
        ElementId::Name(format!("library-body-{}", view.note).into()),
        view.name.clone(),
        view.entry.as_ref(),
        view.entry
            .as_ref()
            .map_or_else(String::new, |entry| entry.body.clone()),
        Source {
            path: view.origin.path.clone(),
            owner: view.origin.owner.clone(),
            builtin: view.origin.builtin,
        },
        Rc::new(NoLinks),
    )
    .wide_tables()
    // The Library draws no value block above the entry.
    .empty_value_block(false)
    .on_open(move |at, window, cx| open(&at, window, cx))
    .on_note(follow)
    .jump_in(&root.pages.library.reader_scroll);
    let reader = match root.app.library.links() {
        Some(links) => reader.render_with(links, window, cx),
        None => reader.render_with(&NoLinks, window, cx),
    };
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(head)
        .child(reader)
        .into_any_element()
}

/// A changed file: its warning with the two choices, then the comparison (or the draft alone
/// when the file has gone, or the system's words when it cannot be read).
fn disk_body(
    root: &mut SplatRoot,
    view: &ReaderView,
    disk: &crate::library::model::DiskView,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let unread = disk.unread;
    let message = kit::Message::new(footer::tone(disk.tone), disk.words.clone())
        .action(
            kit::Button::secondary("library-disk-keep", disk.keep.clone())
                .density(Density::Read)
                .on_click(cx.listener(move |root, _, _, cx| {
                    if unread {
                        let ctx = root.ctx.clone();
                        root.app.library.read_again(&ctx, &root.app.campaigns);
                    } else {
                        root.app.library.overwrite_with_draft();
                    }
                    cx.notify();
                })),
        )
        .destructive(
            kit::Button::destructive("library-disk-lose", disk.lose.clone())
                .density(Density::Read)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.library.lose_draft();
                    cx.notify();
                })),
        );
    let mut body = div()
        .flex()
        .flex_col()
        .w_full()
        .gap(S)
        .child(div().max_w(READER_MEASURE).child(message));
    if let Some(details) = &disk.details {
        body = body.child(technical_details(details.clone()));
        return body.into_any_element();
    }
    match &disk.disk_text {
        Some(text) => body
            .child(comparison(root, &view.draft, text, cx))
            .into_any_element(),
        None => body
            .child(kit::heading(
                kit::Heading::Group {
                    title: "Your draft",
                    count: 0,
                },
                false,
            ))
            .child(
                div()
                    .max_w(px(EDITOR_MAX_WIDTH))
                    .p(S)
                    .bg(c.well)
                    .border(HAIRLINE_WIDTH)
                    .border_color(c.field_border)
                    .ty(Type::Mono)
                    .text_color(c.text)
                    .child(view.draft.clone()),
            )
            .into_any_element(),
    }
}

/// The draft beside the file now (`grouped_draft_comparison`): equal lines aligned (a line
/// only one side has stands beside an empty, hatched row), the lines that differ marked, and a
/// summary of the count and the heading the change is under; `Next difference` steps through
/// the changes when there are more than one.
fn comparison(
    root: &mut SplatRoot,
    draft: &str,
    disk: &str,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    use std::hash::{Hash, Hasher};
    let c = colors();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (draft, disk).hash(&mut hasher);
    let texts = hasher.finish();
    let state = &mut root.pages.library;
    let lines = match &state.comparison {
        Some((seen, lines)) if *seen == texts => lines.clone(),
        _ => {
            let lines = Arc::new(crate::design::ComparedLines::of(draft, disk));
            state.comparison = Some((texts, lines.clone()));
            state.difference = None;
            lines
        }
    };
    let group = state.difference.filter(|at| *at < lines.groups.len());
    let current = group.and_then(|at| lines.groups.get(at)).copied();
    let summary = lines.summary(group);
    let next = (lines.groups.len() > 1).then(|| {
        let count = lines.groups.len();
        kit::Button::quiet("library-next-difference", "Next difference")
            .density(Density::Read)
            .on_click(cx.listener(move |root, _, _, cx| {
                let library = &mut root.pages.library;
                library.difference = Some(library.difference.map_or(1, |at| (at + 1) % count));
                cx.notify();
            }))
    });
    let side = |side: usize, title: &'static str| {
        let mut column = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .child(kit::label(title, 0))
            .child(kit::gap(XS));
        for (at, row) in lines.rows.iter().enumerate() {
            let changed = row[0] != row[1];
            let here = current == Some(at);
            column = column.child(match &row[side] {
                Some(line) => div()
                    .ty(Type::Mono)
                    .text_color(c.text)
                    .pl(S)
                    .border_l(crate::gui::theme::HAIRLINE_WIDTH)
                    .border_color(if changed {
                        c.accent
                    } else {
                        gpui::transparent_black()
                    })
                    .when(changed, |line| {
                        line.bg(if here { c.selected } else { c.current_row })
                    })
                    .child(if line.is_empty() {
                        " ".to_owned()
                    } else {
                        line.clone()
                    })
                    .into_any_element(),
                // No line on this side: the row keeps its height, faint, named for a reader.
                None => div()
                    .id(ElementId::Name(
                        format!("library-no-line-{side}-{at}").into(),
                    ))
                    .ty(Type::Mono)
                    .pl(S)
                    .bg(c.well)
                    .text_color(c.faint)
                    .with_tooltip("No line")
                    .child(" ")
                    .into_any_element(),
            });
        }
        column
    };
    div()
        .flex()
        .flex_col()
        .w_full()
        .gap(S)
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .child(kit::target(
                    kit::text(Type::Small, c.muted, summary.clone()),
                    summary,
                ))
                .children(next),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .w_full()
                .gap(L)
                .child(side(0, "Your draft"))
                .child(side(1, "In the file now")),
        )
}
