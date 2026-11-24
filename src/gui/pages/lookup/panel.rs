//! Lookup's panel (egui: `Lookup::panel`, `written_reader`, `generator_card`): its title line
//! with Back, the open action and Close; the search field; the result list; New NPC, New location
//! and Generate NPC; the held NPC's card; then what the reader shows.
use crate::gui::kit::{self, Button, IconButton, Link, Message, Row, TextField, Tone};
use crate::gui::theme::{Density, L, M, PANEL_MIN_WIDTH, S, Type, XS, XXL, colors};
use crate::gui::{SplatRoot, icons};
use crate::lookup::panel_view::{Showing, search_hint};
use gpui::{
    AnyElement, Context, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*,
};

/// A result row's height (`RESULT_ROW`).
const RESULT_ROW: gpui::Pixels = XXL;

pub(super) fn render(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let lookup = &mut root.app.lookup;
    let query = lookup.refresh_matches();
    let focus = lookup.take_focus_search();
    // The title line: Lookup, Back while there is somewhere to go back to, the open action
    // with its words, and Close.
    let back = lookup.can_go_back().then(|| {
        IconButton::new("lookup-back", icons::HISTORY_BACK, "Back")
            .hint("Back (Alt+←)")
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.lookup.go(crate::history::Step::Back);
                cx.notify();
            }))
    });
    let open = lookup.open_action_label().map(|words| {
        Button::quiet("lookup-open", words)
            .icon(icons::OPEN_OUT)
            .density(Density::Read)
            .blocked(lookup.open_reason())
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.lookup.press_open();
                cx.notify();
            }))
    });
    let title = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .min_h(Density::Read.control())
        .child(kit::text(Type::Heading, c.text, "Lookup"))
        .children(back)
        .child(div().flex_1())
        .children(open)
        .child(
            IconButton::new("lookup-close", icons::CLOSE, "Close Lookup").on_click(cx.listener(
                |root, _, _, cx| {
                    root.app.lookup.close();
                    cx.notify();
                },
            )),
        );
    let width = PANEL_MIN_WIDTH - M * 2.0;
    let field = TextField::new("lookup_search", lookup.query().to_owned())
        .placeholder(search_hint(f32::from(width)))
        .name(crate::lookup::SEARCH_NAME)
        .focus(focus)
        .on_change(cx.listener(|root, words: &str, _, cx| {
            if words.is_empty() && !root.app.lookup.query().is_empty() {
                root.app.lookup.clear_search();
            } else {
                root.app.lookup.set_query(words);
            }
            cx.notify();
        }))
        .on_enter(super::super::settings::act(cx, |root, _, cx| {
            root.app.lookup.close_results();
            cx.notify();
        }));
    // Up and Down move the highlight from the field, as egui's field does.
    let search = div()
        .capture_key_down(cx.listener(|root, event: &gpui::KeyDownEvent, _, cx| {
            let key = event.keystroke.key.as_str();
            // Escape in the field closes Lookup, as everywhere else in the panel.
            if key == "escape" && !event.keystroke.modifiers.modified() {
                let app = &root.app;
                let held = app.lookup.reader.takes_escape()
                    || (app.page == crate::Page::Library && app.library.form_open());
                if !crate::gui::kit::menu::any_open(cx) && !held {
                    root.app.lookup.close();
                    cx.stop_propagation();
                    cx.notify();
                }
                return;
            }
            if event.keystroke.modifiers.modified() || !matches!(key, "up" | "down") {
                return;
            }
            if root.app.lookup.results_shown() {
                root.app.lookup.move_highlight(key == "down");
                cx.stop_propagation();
                cx.notify();
            }
        }))
        .child(field);
    let mut column = div()
        .flex()
        .flex_col()
        .size_full()
        .p(M)
        .gap(XS)
        .bg(c.page)
        .child(title)
        .child(search);
    let lookup = &root.app.lookup;
    if let Some(error) = lookup.error() {
        column = column.child(Message::new(Tone::Error, error.to_owned()));
    }
    if lookup.reading() {
        return column
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(S)
                    .child(kit::icon(icons::BUSY, Type::Body, c.connection.busy))
                    .child(kit::text(Type::Body, c.muted, "Reading content…")),
            )
            .into_any_element();
    }
    if let Some((count, _)) = lookup.broken() {
        column = column.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .child(kit::status(&kit::Status::new(
                    icons::ERROR,
                    format!(
                        "{} cannot be read",
                        crate::design::count(count, "note", "notes")
                    ),
                    c.result.error,
                )))
                .child(Link::new("lookup-broken-show", "Show").on_click(cx.listener(
                    |root, _, _, cx| {
                        root.app.lookup.show_broken();
                        cx.notify();
                    },
                ))),
        );
    }
    if let Some(error) = lookup.create_error() {
        column = column.child(Message::new(Tone::Error, error.to_owned()));
    }
    if lookup.offers_results() {
        column = column.child(
            Button::quiet(
                "lookup-show-results",
                format!(
                    "Show {}",
                    crate::design::count(lookup.match_count(), "result", "results")
                ),
            )
            .name("Show the results")
            .density(Density::Read)
            .line_start()
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.lookup.open_results();
                cx.notify();
            })),
        );
    }
    if lookup.results_shown() {
        column = column.child(results(root, cx));
    }
    column = column.children(creation(root, cx));
    let lookup = &root.app.lookup;
    if lookup.generated_on_screen() {
        return column.child(card(root, cx)).into_any_element();
    }
    column = column.child(kit::rule());
    if let Some(npc) = lookup.generated() {
        column = column.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .pb(S)
                .child(kit::text(Type::Body, c.text, npc.name.clone()))
                .child(kit::status(&kit::Status::new(
                    icons::DRAFT,
                    "Unsaved NPC",
                    c.document.unsaved,
                )))
                .child(kit::text(Type::Body, c.muted, "·"))
                .child(Link::new("lookup-unsaved-show", "Show").on_click(cx.listener(
                    |root, _, _, cx| {
                        root.app.lookup.show_card();
                        cx.notify();
                    },
                ))),
        );
    }
    let body = match lookup.showing() {
        Showing::Written(at) => written(root, at, &query, cx),
        Showing::Entry(position) => super::entry::render(root, position, window, cx),
        Showing::Generated | Showing::Nothing => {
            if query.is_empty() {
                kit::help(crate::lookup::SEARCH_HELP).pt(S).into_any_element()
            } else {
                div().into_any_element()
            }
        }
    };
    column
        .child(div().flex_1().min_h_0().child(body))
        .into_any_element()
}

/// The result rows, `Nothing found`, and `Show all N`.
fn results(root: &SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let lookup = &root.app.lookup;
    let rows = lookup.result_rows();
    let mut list = div().flex().flex_col();
    if rows.is_empty() {
        list = list.child(kit::text(
            Type::Body,
            c.muted,
            "Nothing found. Check the spelling, or try fewer words.",
        ));
    }
    let mut column = div().flex().flex_col();
    for (at, row) in rows.iter().enumerate() {
        let full = format!("{} · {}", row.title, row.detail);
        column = column.child(
            Row::new(("lookup-result", at), full.clone())
                .height(RESULT_ROW)
                .selected(row.selected)
                .on_click(cx.listener(move |root, _, _, cx| {
                    root.app.lookup.choose_result(at, false);
                    cx.notify();
                }))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_baseline()
                        .min_w_0()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .child(kit::text(Type::Body, c.text, row.title.clone()).flex_none())
                        .when(!row.detail.is_empty(), |line| {
                            line.child(
                                kit::text::one_line(
                                    Type::Small,
                                    c.muted,
                                    format!(" · {}", row.detail),
                                )
                                .min_w_0(),
                            )
                        }),
                ),
        );
    }
    let listed = rows.len().min(crate::lookup::Lookup::rows_listed());
    if !rows.is_empty() {
        list = list.child(
            div()
                .h(RESULT_ROW * listed as f32)
                .child(kit::scroll_column("lookup_results", column)),
        );
    }
    if lookup.more_results() {
        list = list.child(
            Button::quiet(
                "lookup-show-all",
                format!("Show all {}", lookup.match_count()),
            )
            .hint("List every match, without more typing")
            .density(Density::Read)
            .line_start()
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.lookup.show_all_results();
                cx.notify();
            })),
        );
    }
    list.into_any_element()
}

/// `New NPC “…”` and `New location “…”` for a name nothing has, or `Generate NPC`.
fn creation(root: &SplatRoot, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let lookup = &root.app.lookup;
    let name = lookup.new_name();
    let generate = lookup.offers_generate();
    if name.is_none() && !generate {
        return None;
    }
    let mut row = div().flex().flex_row().flex_wrap().items_center().gap(S);
    if let Some(name) = name {
        for (at, (word, kind)) in [("New NPC", "npc"), ("New location", "location")]
            .into_iter()
            .enumerate()
        {
            let mut action = Button::quiet(("lookup-new", at), format!("{word} “{name}”"))
                .density(Density::Read)
                .on_click(cx.listener(move |root, _, _, cx| {
                    root.app.lookup.create_named(kind);
                    cx.notify();
                }));
            if at == 0 {
                action = action.line_start();
            }
            row = row.child(action);
        }
    }
    if generate {
        row = row.child(
            Button::quiet("lookup-generate", "Generate NPC")
                .density(Density::Read)
                .line_start()
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.lookup.generate();
                    cx.notify();
                })),
        );
    }
    Some(row.into_any_element())
}

/// The held NPC's card (`generator_card`, Lookup's form: a link to use your own names).
fn card(root: &SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let lookup = &root.app.lookup;
    let Some(npc) = lookup.generated() else {
        return div().into_any_element();
    };
    let (sources, problems) = lookup.generator_facts();
    let mut column = div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(kit::rule())
        .child(kit::heading(kit::Heading::Group { title: "Generated NPC", count: 0 }, false))
        .child(kit::text(Type::Strong, c.text, npc.name.clone()));
    for (label, words) in splat_content_ui::npc_generator::ATTRIBUTES.iter().zip(&npc.attributes) {
        column = column.child(kit::text(Type::Body, c.text, format!("{label}: {words}")));
    }
    if sources.is_empty() && problems.is_empty() {
        column = column.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .child(kit::text(Type::Small, c.muted, crate::lookup::BUILT_IN_NAMES))
                .child(
                    Link::new("lookup-own-names", crate::lookup::USE_OWN_NAMES)
                        .small()
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.lookup.generated_to_world();
                            cx.notify();
                        })),
                ),
        );
    }
    for source in sources {
        let (kind, path) = source.split_once(": ").unwrap_or(("npc-name", source));
        let label = if kind == "npc-name" {
            "Names".to_owned()
        } else {
            crate::design::reader::human_label(kind)
        };
        column = column.child(kit::text(Type::Small, c.muted, format!("{label} from {path}")));
    }
    for problem in problems {
        column = column.child(Message::new(Tone::Warning, problem.message.clone()));
    }
    if let Some(path) = lookup.generated_path() {
        column = column.child(kit::text(Type::Small, c.muted, format!("Will save {path}")));
    }
    let save = if lookup.run_open {
        Button::secondary("lookup-save-npc", "Save NPC")
    } else {
        Button::primary("lookup-save-npc", "Save NPC")
    };
    column
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .child(save.density(Density::Read).on_click(cx.listener(|root, _, _, cx| {
                    root.app.lookup.save_generated();
                    cx.notify();
                })))
                .child(
                    Button::secondary("lookup-reroll", "Reroll")
                        .icon(icons::DICE)
                        .density(Density::Read)
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.lookup.generate();
                            cx.notify();
                        })),
                )
                .child(
                    Button::quiet("lookup-discard-npc", "Discard")
                        .density(Density::Read)
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.lookup.discard_generated();
                            cx.notify();
                        })),
                ),
        )
        .into_any_element()
}

/// A found note or event (`written_reader`): its title, where it lives with its open action,
/// the lines that have the words, then its words.
fn written(root: &SplatRoot, at: usize, query: &str, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let Some(note) = root.app.lookup.written_at(at) else {
        return div().into_any_element();
    };
    let place = match note.place {
        crate::lookup::WrittenIn::Session(_) => "Open this session",
        crate::lookup::WrittenIn::Event(_) => "Open on the timeline",
    };
    let mut column = div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(kit::text(Type::Display, c.text, note.title.clone()))
        .child(kit::text(Type::Small, c.muted, note.detail.clone()))
        .child(
            Button::quiet("lookup-written-open", place)
                .icon(icons::OPEN_OUT)
                .density(Density::Read)
                .on_click(cx.listener(move |root, _, _, cx| {
                    root.app.lookup.open_written_place(at);
                    cx.notify();
                })),
        )
        .child(kit::gap(L));
    let found: Vec<&str> = note
        .text
        .lines()
        .filter(|line| !query.is_empty() && line.to_lowercase().contains(query))
        .take(3)
        .collect();
    if !found.is_empty() && note.text.lines().count() > 12 {
        column = column.child(kit::label("Found here", 0));
        for line in found {
            column = column.child(kit::text(
                Type::Body,
                c.text,
                line.trim_start_matches(['#', '-', '*', ' ']).to_owned(),
            ));
        }
        column = column.child(kit::rule());
    }
    let text = SharedString::from(note.text.clone());
    column = column.child(kit::markdown::Markdown::new(("lookup-written-text", at), text).compact());
    kit::scroll_column(("lookup-written", at), column.pr(S)).into_any_element()
}
