//! The Library's reader pane (`Library::editor` and `editor_pane`): a form in its place, the empty
//! reader, or the open entry: its toolbar, its owner and file line, its save row, then Read (the
//! title, the body's notices, the body) or Edit Markdown (the editor), or a changed file's
//! choices and comparison; the pane's footer under it all.
use super::{footer, forms};
use crate::gui::theme::{
    Density, HAIRLINE_WIDTH, L, M, READER_MEASURE, S, Type, TypeExt, XS, colors,
};
use crate::gui::{SplatRoot, bridge, icons, kit};
use crate::library::model::ReaderView;
use crate::lookup::ContentIndex;
use crate::gui::kit::WithTooltip as _;
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*, px,
};
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
    let mut pane = kit::Pane::new("library-empty-reader").flush().child(kit::empty_state(
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
            kit::Button::quiet(
                ElementId::Name(format!("library-act-{at}").into()),
                *label,
            )
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
            bridge::color(*color),
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
fn save_row(view: &ReaderView, width: f32, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let row = view.save_row.as_ref()?;
    let c = colors();
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
                bridge::color(*color),
            )))
            .when(row.discard, |line| {
                line.child(kit::Button::quiet("library-discard", "Discard").on_click(
                    cx.listener(|root, _, _, cx| {
                        root.app.library.discard();
                        cx.notify();
                    }),
                ))
            })
            .children(
                row.reason
                    .as_ref()
                    .map(|(long, short)| {
                        // The long form when it fits whole beside the row's parts, else the
                        // short one (`save_row_with`, measured there; estimated here from the
                        // `Small` type's average letter).
                        let room = width.min(EDITOR_MAX_WIDTH) - 300.0;
                        let fits = long.chars().count() as f32 * 6.2 <= room;
                        if fits { long.clone() } else { short.clone() }
                    })
                    .or(row.lock.clone())
                    .map(|reason| kit::text(Type::Small, c.muted, reason).truncate()),
            )
            .child(div().flex_1())
            .child(save)
            .into_any_element(),
    )
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
                    let ctx = root.bridge.ctx().clone();
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
    let c = colors();
    let mut head = div()
        .flex()
        .flex_col()
        .w_full()
        .flex_none()
        .child(toolbar(&view, cx))
        .child(file_line(&view));
    if let Some(row) = save_row(&view, width, cx) {
        head = head.child(row);
    }
    let body: AnyElement = if let Some(disk) = &view.disk {
        disk_body(&view, disk, cx)
    } else if view.toolbar.source {
        let focus = root.app.library.take_focus_editor();
        let editable = view.editable;
        let editor = kit::TextEditor::new(
            ElementId::Name(
                format!("library-editor-{:?}-{}", view.key.0, view.key.1).into(),
            ),
            view.draft.clone(),
        )
        .name("Markdown editor")
        .mono()
        .fill()
        .on_change(cx.listener(move |root, words: &str, _, cx| {
            if editable {
                root.app.library.edit_draft(words);
                cx.notify();
            }
        }));
        let _ = (focus, window);
        div()
            .size_full()
            .max_w(px(EDITOR_MAX_WIDTH))
            .pt(XS)
            .child(editor)
            .into_any_element()
    } else {
        let index = index.clone();
        let mut reader = div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(READER_MEASURE)
            .pt(XS)
            .child(kit::text(Type::Title, c.text, view.title.clone()))
            .when(!view.subtitle.is_empty(), |reader| {
                reader.child(kit::text(Type::Small, c.muted, view.subtitle.clone()))
            });
        if !view.notices.is_empty() {
            reader = reader.child(kit::gap(L));
        }
        for (at, item) in view.notices.iter().enumerate() {
            reader = reader.child(notice(at, item, cx));
        }
        div()
            .flex()
            .flex_col()
            .w_full()
            .child(reader)
            .child(kit::gap(M))
            .child(
                kit::markdown::Markdown::new(
                    ElementId::Name(format!("library-body-{}", view.note).into()),
                    view.body.clone(),
                )
                .on_link(cx.listener(move |root, target: &str, _, cx| {
                    root.app.library.follow_link(target, index.as_deref());
                    cx.notify();
                })),
            )
            .into_any_element()
    };
    let choices = (!view.link_choices.is_empty()).then(|| {
        let mut line = div().flex().flex_row().flex_wrap().gap(S);
        for (at, words) in &view.link_choices {
            let at = *at;
            line = line.child(
                kit::link(
                    ElementId::Name(format!("library-link-choice-{at}").into()),
                    words.clone()
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

/// A changed file: its warning with the two choices, then the comparison (or the draft alone
/// when the file has gone, or the system's words when it cannot be read).
fn disk_body(
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
                        let ctx = root.bridge.ctx().clone();
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
        Some(text) => body.child(comparison(&view.draft, text)).into_any_element(),
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

/// The draft beside the file now, line by line, with the lines that differ marked.
fn comparison(draft: &str, disk: &str) -> impl IntoElement {
    let c = colors();
    let left: Vec<&str> = draft.lines().collect();
    let right: Vec<&str> = disk.lines().collect();
    let differ = (0..left.len().max(right.len()))
        .filter(|&at| left.get(at) != right.get(at))
        .count();
    let side = |title: &'static str, lines: &[&str], other: &[&str]| {
        let mut column = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .child(kit::label(title, 0))
            .child(kit::gap(XS));
        for (at, line) in lines.iter().enumerate() {
            let changed = other.get(at) != Some(line);
            column = column.child(
                div()
                    .ty(Type::Mono)
                    .text_color(c.text)
                    .pl(S)
                    .border_l(crate::gui::theme::HAIRLINE_WIDTH)
                    .border_color(if changed { c.accent } else { gpui::transparent_black() })
                    .when(changed, |line| line.bg(c.current_row))
                    .child(if line.is_empty() {
                        " ".to_owned()
                    } else {
                        (*line).to_owned()
                    }),
            );
        }
        column
    };
    div()
        .flex()
        .flex_col()
        .w_full()
        .gap(S)
        .child(kit::text(
            Type::Small,
            c.muted,
            crate::design::count(differ, "line differs", "lines differ"),
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .w_full()
                .gap(L)
                .child(side("Your draft", &left, &right))
                .child(side("In the file now", &right, &left)),
        )
}
