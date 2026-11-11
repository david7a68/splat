//! Read-only Markdown presentation. Content cannot execute UI actions, but it can say which note
//! a link names, and a roll table can ask for a roll. Whoever drew it decides what happens next.
use crate::design::{self, Type, colors};
use eframe::egui::util::cache::{ComputerMut, FrameCache};
use eframe::egui::{self, text::LayoutJob};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::{borrow::Cow, sync::Arc};

/// A link to a note, as `world::with_links` writes one.
pub const NOTE_LINK: &str = "splat-note:";
/// An embedded image, as `world::with_links` writes one.
pub const FILE_LINK: &str = "splat-file:";

fn clicked_id() -> egui::Id {
    egui::Id::new("markdown_clicked_note")
}

fn heading_id() -> egui::Id {
    egui::Id::new("markdown_wanted_heading")
}

/// The note link the DM pressed, for whoever just drew this Markdown.
pub fn take_note_link(ctx: &egui::Context) -> Option<String> {
    ctx.data_mut(|data| data.remove_temp::<String>(clicked_id()))
}

/// A resolved bold reference pressed in the Reader, for Lookup to open.
pub fn take_lookup_link(ctx: &egui::Context) -> Option<String> {
    ctx.data_mut(|data| data.remove_temp::<String>(egui::Id::new("markdown_clicked_lookup")))
}

/// Ask the next draw to scroll to this heading. `None` ends the request.
pub fn want_heading(ctx: &egui::Context, heading: Option<String>) {
    ctx.data_mut(|data| match heading {
        Some(heading) => data.insert_temp(heading_id(), heading),
        None => data.remove::<String>(heading_id()),
    });
}

/// True once, for the heading a draw was asked to show.
pub fn wanted_heading(ctx: &egui::Context, heading: &str) -> bool {
    let wanted = ctx.data(|data| data.get_temp::<String>(heading_id()));
    if wanted.is_some_and(|wanted| wanted.eq_ignore_ascii_case(heading.trim())) {
        want_heading(ctx, None);
        return true;
    }
    false
}

#[cfg(test)]
pub fn show(ui: &mut egui::Ui, markdown: &str) {
    show_with_density(ui, markdown, false);
}

/// The Reader's Markdown.
pub fn show_reader(ui: &mut egui::Ui, markdown: &str) {
    ui.scope(|ui| show_with_density(ui, markdown, false));
}

/// The Reader's Markdown, as compact field rows.
pub fn show_compact(ui: &mut egui::Ui, markdown: &str) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 3.0;
        show_with_density(ui, markdown, true);
    });
}

/// A note with its links resolved, which `linked_markdown` keeps from one frame to the next.
#[derive(Clone, Copy)]
struct Linking<'a> {
    index: &'a crate::lookup::ContentIndex,
    markdown: &'a str,
    from: Option<uuid::Uuid>,
}

impl std::hash::Hash for Linking<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.generation().hash(state);
        self.markdown.hash(state);
        self.from.hash(state);
    }
}

#[derive(Default)]
struct Linker;

impl ComputerMut<Linking<'_>, Arc<str>> for Linker {
    fn compute(&mut self, linking: Linking<'_>) -> Arc<str> {
        linking
            .index
            .reader_markdown(linking.markdown, linking.from)
            .into()
    }
}

/// Resolving each link searches the index. A reader repaints on each frame of a scroll, so
/// it resolves a note's links again only when the note, the index, or the reader changes.
fn linked_markdown(
    ctx: &egui::Context,
    index: &crate::lookup::ContentIndex,
    markdown: &str,
    from: Option<uuid::Uuid>,
) -> Arc<str> {
    ctx.memory_mut(|memory| {
        memory
            .caches
            .cache::<FrameCache<Arc<str>, Linker>>()
            .get(Linking {
                index,
                markdown,
                from,
            })
    })
}

fn show_with_density(ui: &mut egui::Ui, markdown: &str, dense: bool) {
    let prose = if dense {
        Type::Stat
    } else {
        design::reader::prose_type(ui)
    };
    let mut job = LayoutJob::default();
    let mut heading = None;
    let mut inventory_table = false;
    let mut target = Targets::default();
    let mut link = None;
    let mut strong_link = None;
    let mut rolls = None;
    let mut in_link = false;
    let mut code = false;
    let mut strong = 0_usize;
    let mut italic = false;
    let mut struck = false;
    let mut lists: Vec<Option<u64>> = Vec::new();
    let mut table: Option<Vec<Vec<String>>> = None;
    let mut figures: Vec<std::path::PathBuf> = Vec::new();
    let mut in_figure = false;
    let ctx = ui.ctx().clone();
    let index = ui.ctx().data(|data| {
        data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
            "reader_link_index",
        ))
    });
    let from = reader_document(ui);
    // Each reader's links have their own ids, so another reader with the same words cannot
    // take a link's focus.
    target.reader = from;
    let linked: Arc<str> = index.as_ref().map_or_else(
        || markdown.into(),
        |index| linked_markdown(&ctx, index, markdown, from),
    );
    let source = local_figure_links(&linked, |url| crate::art::link_art(&ctx, url).is_some());
    for (event, range) in Parser::new_ext(
        &source,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS,
    )
    .into_offset_iter()
    {
        // A roll marker belongs to the table right after it, not to a later one.
        if matches!(
            event,
            Event::Start(
                Tag::Paragraph
                    | Tag::Heading { .. }
                    | Tag::List(_)
                    | Tag::CodeBlock(_)
                    | Tag::BlockQuote(_)
            )
        ) {
            rolls = None;
        }
        if matches!(event, Event::Code(_) | Event::Start(Tag::Image { .. })) {
            strong_link = None;
        }
        match event {
            Event::Html(ref text) if text.trim().starts_with("<!-- splat:inventory") => {
                inventory_table = true;
            }
            Event::Html(ref text) if crate::roll_table::rolls_marker(text).is_some() => {
                rolls = crate::roll_table::rolls_marker(text);
            }
            Event::Text(ref text) if in_figure && &**text == "image" => {}
            Event::Start(Tag::Link { dest_url, .. }) if table.is_none() => {
                // An explicit destination wins over the bold words around it.
                strong_link = None;
                in_link = true;
                // A link to an image with a local copy shows the image instead of opening a page.
                let local = crate::art::link_art(&ctx, &dest_url)
                    .or_else(|| dest_url.strip_prefix(FILE_LINK).map(Into::into));
                if let Some(path) = local {
                    figures.push(path);
                    in_figure = true;
                    continue;
                }
                if dest_url.starts_with("splat-missing:") {
                    job.append(
                        &format!("{} ", design::icons::WARN),
                        0.0,
                        egui::TextFormat {
                            font_id: prose.font(ui),
                            color: colors().result.warning,
                            ..Default::default()
                        },
                    );
                    link = Some((job.text.len(), LinkTarget::Missing));
                    continue;
                }
                if let Some(destination) = LinkTarget::from_destination(&dest_url) {
                    link = Some((job.text.len(), destination));
                }
            }
            Event::End(TagEnd::Link) => {
                in_link = false;
                in_figure = false;
                if let Some((start, destination)) = link.take() {
                    target.links.push((start..job.text.len(), destination));
                }
            }
            Event::Start(Tag::Table(_)) => {
                flush(ui, &mut job, &mut target);
                table = Some(Vec::new());
            }
            Event::Start(Tag::TableHead | Tag::TableRow) => {
                if let Some(rows) = &mut table {
                    rows.push(Vec::new());
                }
            }
            Event::Start(Tag::TableCell) => {
                if let Some(row) = table.as_mut().and_then(|rows| rows.last_mut()) {
                    row.push(String::new());
                }
            }
            Event::End(TagEnd::Table) => {
                if let Some(mut rows) = table.take() {
                    if dense && rows.first().is_some_and(|row| row == &["Field", "Value"]) {
                        show_fields(ui, &rows[1..]);
                        inventory_table = false;
                        continue;
                    }
                    let compact = inventory_table
                        && rows
                            .first()
                            .is_some_and(|row| row.iter().any(|cell| cell == "Link"))
                        && rows.iter().skip(1).all(|row| {
                            rows[0]
                                .iter()
                                .position(|cell| cell == "Link")
                                .and_then(|i| row.get(i))
                                .is_some_and(|name| !name.is_empty())
                        });
                    let columns: Vec<_> = if compact {
                        ["Link", "Quantity", "Equipment", "Attuned"]
                            .iter()
                            .filter_map(|name| rows.first()?.iter().position(|cell| cell == name))
                            .collect()
                    } else {
                        (0..rows.first().map_or(0, Vec::len)).collect()
                    };
                    let display = |cell: &'_ str| -> String {
                        if compact {
                            match cell {
                                "Link" => "Item",
                                "true" => "Yes",
                                "false" => "—",
                                other => other,
                            }
                            .to_owned()
                        } else {
                            cell.to_owned()
                        }
                    };
                    if let Some(Ok((_, dice))) = &rolls
                        && let Some(header) = rows.first_mut().and_then(|row| row.first_mut())
                        && header.trim().is_empty()
                    {
                        *header = dice
                            .strip_prefix('1')
                            .filter(|value| value.starts_with('d'))
                            .unwrap_or(dice)
                            .to_owned();
                    }
                    let mut widths = column_widths(ui, &rows, &columns, &display);
                    if rolls.is_some() && !widths.is_empty() {
                        // Reserve the roll mark so it never wraps above the row label.
                        for row in rows.iter().skip(1) {
                            let label =
                                display(row.get(columns[0]).map(String::as_str).unwrap_or(""));
                            let marked = format!("{} {label}", design::icons::DICE);
                            let width = ui
                                .painter()
                                .layout_no_wrap(marked, Type::Stat.font(ui), colors().text)
                                .size()
                                .x
                                .ceil()
                                + design::XS;
                            widths[0] = widths[0].max(width);
                        }
                    }
                    let matched = rolls
                        .as_ref()
                        .and_then(|marker| crate::roll_table::matched_row(ui, marker, &rows));
                    reader_table(ui, &rows, &columns, &widths, &display, matched);
                    if let Some(marker) = rolls.take() {
                        crate::roll_table::roll_control(ui, &marker, &rows);
                    }
                    ui.add_space(design::S);
                }
                inventory_table = false;
            }
            Event::Text(text) | Event::Code(text) if table.is_some() => {
                if let Some(cell) = table
                    .as_mut()
                    .and_then(|rows| rows.last_mut())
                    .and_then(|row| row.last_mut())
                {
                    cell.push_str(&text);
                }
            }
            Event::Start(Tag::Heading { level, .. }) => {
                flush(ui, &mut job, &mut target);
                ui.add_space(if dense {
                    design::S
                } else {
                    match level {
                        pulldown_cmark::HeadingLevel::H1 => design::S,
                        pulldown_cmark::HeadingLevel::H2 => design::L,
                        _ => design::M,
                    }
                });
                heading = Some(level);
            }
            Event::End(TagEnd::Heading(level)) => {
                if wanted_heading(ui.ctx(), &job.text) {
                    ui.scroll_to_cursor(Some(egui::Align::TOP));
                }
                draw_figures(ui, &mut figures, &job);
                flush(ui, &mut job, &mut target);
                if level == pulldown_cmark::HeadingLevel::H2
                    || (!dense && level == pulldown_cmark::HeadingLevel::H3)
                {
                    design::rule(ui);
                    ui.add_space(design::XS);
                }
                heading = None;
            }
            Event::Start(Tag::CodeBlock(_)) => {
                flush(ui, &mut job, &mut target);
                code = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                flush(ui, &mut job, &mut target);
                code = false;
            }
            Event::Start(Tag::Strong) => {
                strong += 1;
                if strong == 1 {
                    strong_link = (!in_link && heading.is_none() && table.is_none())
                        .then_some(job.text.len());
                }
            }
            Event::End(TagEnd::Strong) => {
                strong = strong.saturating_sub(1);
                if strong == 0
                    && let Some(start) = strong_link.take()
                {
                    let words = job.text[start..].trim();
                    let table_caption = job.text[..start].trim().is_empty()
                        && source[range.end..].trim_start().starts_with('|');
                    if !(words.is_empty() || words.ends_with(['.', ':']) || table_caption)
                        && index.as_ref().is_some_and(|index| {
                            matches!(
                                index.resolve_link(words, from),
                                crate::world::Go::Item(..) | crate::world::Go::Many(_)
                            )
                        })
                    {
                        for section in &mut job.sections {
                            if section.byte_range.start >= start {
                                section.format.color = colors().accent;
                                section.format.underline =
                                    egui::Stroke::new(design::HAIRLINE_WIDTH, colors().accent);
                                if section.format.strikethrough != egui::Stroke::NONE {
                                    section.format.strikethrough.color = colors().accent;
                                }
                            }
                        }
                        target
                            .links
                            .push((start..job.text.len(), LinkTarget::Lookup(words.to_owned())));
                    }
                }
            }
            Event::Start(Tag::Emphasis) => italic = true,
            Event::End(TagEnd::Emphasis) => italic = false,
            Event::Start(Tag::Strikethrough) => struck = true,
            Event::End(TagEnd::Strikethrough) => struck = false,
            Event::Start(Tag::BlockQuote(_)) => {
                flush(ui, &mut job, &mut target);
                target.quotes.push(ui.cursor().top());
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                draw_figures(ui, &mut figures, &job);
                flush(ui, &mut job, &mut target);
                if let Some(top) = target.quotes.pop() {
                    let x = ui.max_rect().left()
                        + QUOTE_INDENT * target.quotes.len() as f32
                        + QUOTE_BAR_WIDTH / 2.0;
                    ui.painter().vline(
                        x,
                        top..=target.quote_bottom.max(top),
                        egui::Stroke::new(QUOTE_BAR_WIDTH, colors().rule),
                    );
                }
            }
            Event::Start(Tag::List(start)) => lists.push(start),
            Event::End(TagEnd::List(_)) => {
                lists.pop();
            }
            Event::Start(Tag::Item) => {
                flush(ui, &mut job, &mut target);
                let indent = "  ".repeat(lists.len().saturating_sub(1));
                let marker = match lists.last_mut() {
                    Some(Some(number)) => {
                        let marker = format!("{number}. ");
                        *number += 1;
                        marker
                    }
                    _ => "• ".into(),
                };
                job.append(
                    &format!("{indent}{marker}"),
                    0.0,
                    egui::TextFormat {
                        font_id: prose.font(ui),
                        line_height: Some(prose.line()),
                        color: colors().muted,
                        ..Default::default()
                    },
                );
            }
            Event::TaskListMarker(done) => {
                // The marker follows Item's indentation and bullet, before any prose or links.
                let indent = "  ".repeat(lists.len().saturating_sub(1));
                // A bulleted item trades its bullet for the box; a numbered item keeps its number
                // and the box follows it.
                let bulleted = matches!(lists.last(), Some(None));
                let lead = if bulleted {
                    job = LayoutJob::default();
                    indent.clone()
                } else {
                    std::mem::take(&mut job.text).clone()
                };
                if !bulleted {
                    job = LayoutJob::default();
                    job.append(
                        &lead,
                        0.0,
                        egui::TextFormat {
                            font_id: prose.font(ui),
                            line_height: Some(prose.line()),
                            color: colors().muted,
                            ..Default::default()
                        },
                    );
                }
                let icon = if done {
                    egui_phosphor::regular::CHECK_SQUARE
                } else {
                    egui_phosphor::regular::SQUARE
                };
                job.append(
                    &if bulleted {
                        format!("{indent}{icon} ")
                    } else {
                        format!("{icon} ")
                    },
                    0.0,
                    egui::TextFormat {
                        font_id: prose.font(ui),
                        line_height: Some(prose.line()),
                        color: colors().muted,
                        ..Default::default()
                    },
                );
                target.task = Some((
                    done,
                    design::number_width(ui, &lead, &prose.font(ui)),
                    prose,
                ));
            }
            Event::End(TagEnd::Paragraph | TagEnd::Item) => {
                draw_figures(ui, &mut figures, &job);
                flush(ui, &mut job, &mut target);
            }
            Event::HardBreak if table.is_none() => job.append(
                "\n",
                0.0,
                egui::TextFormat {
                    font_id: prose.font(ui),
                    line_height: Some(prose.line()),
                    ..Default::default()
                },
            ),
            Event::SoftBreak if table.is_none() => job.append(
                if code { "\n" } else { " " },
                0.0,
                egui::TextFormat {
                    font_id: prose.font(ui),
                    line_height: Some(prose.line()),
                    ..Default::default()
                },
            ),
            Event::Code(text) if !dense && heading.is_none() => job.append(
                &text,
                0.0,
                egui::TextFormat {
                    font_id: Type::Mono.font(ui),
                    // Mono's own line puts code on the baseline of the serif beside it.
                    line_height: Some(Type::Mono.line()),
                    color: colors().text,
                    background: design::reader::inline_code_fill(),
                    ..Default::default()
                },
            ),
            Event::Text(text) | Event::Code(text) => {
                let role = if code {
                    Type::Mono
                } else if let Some(level) = heading {
                    match level {
                        _ if dense => Type::StatLabel,
                        pulldown_cmark::HeadingLevel::H1 => Type::Title,
                        pulldown_cmark::HeadingLevel::H2 | pulldown_cmark::HeadingLevel::H3 => {
                            Type::Section
                        }
                        _ => Type::ProseStrong,
                    }
                } else if strong > 0 {
                    if dense {
                        Type::StatLabel
                    } else {
                        Type::ProseStrong
                    }
                } else if (italic || !target.quotes.is_empty()) && !dense {
                    Type::ProseItalic
                } else {
                    prose
                };
                // Compact prose keeps the size it was given, whatever its weight.
                let mut font_id = role.font(ui);
                if matches!(role, Type::ProseStrong | Type::ProseItalic) && heading.is_none() {
                    font_id.size = prose.size();
                }
                let color = if matches!(link, Some((_, LinkTarget::Missing))) {
                    colors().muted
                } else if link.is_some() {
                    colors().accent
                } else {
                    colors().text
                };
                job.append(
                    &text,
                    0.0,
                    egui::TextFormat {
                        font_id,
                        line_height: Some(if heading.is_some() || code {
                            role.line()
                        } else {
                            prose.line()
                        }),
                        color,
                        underline: if link.is_some()
                            && !matches!(link, Some((_, LinkTarget::Missing)))
                        {
                            egui::Stroke::new(design::HAIRLINE_WIDTH, color)
                        } else {
                            egui::Stroke::NONE
                        },
                        strikethrough: if struck {
                            egui::Stroke::new(1.0, color)
                        } else {
                            egui::Stroke::NONE
                        },
                        ..Default::default()
                    },
                );
            }
            Event::Rule => {
                flush(ui, &mut job, &mut target);
                design::rule(ui);
            }
            _ => {}
        }
    }
    draw_figures(ui, &mut figures, &job);
    flush(ui, &mut job, &mut target);
}

/// Images come before the words of their block, which are their caption.
fn draw_figures(ui: &mut egui::Ui, figures: &mut Vec<std::path::PathBuf>, caption: &LayoutJob) {
    for figure in figures.drain(..) {
        crate::art::figure(ui, &figure, &caption.text);
    }
}

/// Notes write a captioned image as `*caption* ([image](url))`. Where the image will show in
/// place, the brackets around the link have nothing left to enclose.
fn local_figure_links(markdown: &str, local: impl Fn(&str) -> bool) -> Cow<'_, str> {
    const OPEN: &str = "([image](";
    if !markdown.contains(OPEN) {
        return Cow::Borrowed(markdown);
    }
    let mut output = String::with_capacity(markdown.len());
    let mut rest = markdown;
    while let Some(start) = rest.find(OPEN) {
        let after = &rest[start + OPEN.len()..];
        match after.find("))") {
            Some(end) if local(&after[..end]) => {
                output.push_str(&rest[..start]);
                output.push_str("[image](");
                output.push_str(&after[..=end]);
                rest = &after[end + 2..];
            }
            _ => {
                output.push_str(&rest[..start + OPEN.len()]);
                rest = after;
            }
        }
    }
    output.push_str(rest);
    Cow::Owned(output)
}

const TABLE_GAP: f32 = design::M;

/// The row label has its own column. Only the remaining cells belong to the
/// horizontal scroll area, so its real viewport also supplies the cell padding.
fn reader_table(
    ui: &mut egui::Ui,
    rows: &[Vec<String>],
    columns: &[usize],
    widths: &[f32],
    display: &dyn Fn(&str) -> String,
    matched: Option<usize>,
) {
    if widths.is_empty() {
        return;
    }
    let cells: Vec<Vec<_>> = rows
        .iter()
        .enumerate()
        .map(|(row, cells)| {
            columns
                .iter()
                .enumerate()
                .map(|(position, column)| {
                    let mut text = display(cells.get(*column).map(String::as_str).unwrap_or(""));
                    if matched == Some(row) && position == 0 {
                        text = format!("{} {text}", design::icons::DICE);
                    }
                    let role = if row == 0 {
                        Type::StatLabel
                    } else {
                        Type::Stat
                    };
                    ui.painter()
                        .layout(text, role.font(ui), colors().text, widths[position])
                })
                .collect()
        })
        .collect();
    let heights: Vec<_> = cells
        .iter()
        .map(|row| {
            row.iter().map(|cell| cell.size().y).fold(0.0, f32::max)
                + design::XS * 2.0
                + design::HAIRLINE_WIDTH
        })
        .collect();
    let height: f32 = heights.iter().sum();
    let width = widths.iter().sum::<f32>() + TABLE_GAP * widths.len().saturating_sub(1) as f32;
    let scrolls = width > ui.available_width();
    if scrolls {
        ui.label(
            Type::Small
                .rich(ui, "More columns: scroll sideways or use Shift+wheel.")
                .color(colors().muted),
        );
    }
    // The sideways scroll bar has its own lane below the last row, so it never covers cell text.
    let lane = if scrolls {
        let scroll = ui.spacing().scroll;
        scroll.bar_width + scroll.bar_outer_margin
    } else {
        0.0
    };
    let top = ui.cursor().top();
    let left = ui.cursor().left();
    let outer_clip = ui.clip_rect();
    // While the table's own lane is below view, egui floats the bar up to the reader's bottom
    // edge. There the lane stays at that edge, as the header stays at the top: rows pass under
    // its hairline, so the bar never covers cell text.
    let cut = (scrolls && top < outer_clip.bottom() && top + height + lane > outer_clip.bottom())
        .then(|| (outer_clip.bottom() - lane, outer_clip.bottom()));
    let mut moving_origin = egui::pos2(left + widths[0] + TABLE_GAP, top);
    let mut moving_clip = outer_clip.intersect(egui::Rect::from_min_max(
        egui::pos2(left + widths[0] + TABLE_GAP, outer_clip.top()),
        outer_clip.max,
    ));
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = TABLE_GAP;
        ui.allocate_ui_with_layout(
            egui::vec2(widths[0], height),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                table_part(ui, &cells, widths, &heights, 0..1, matched, cut);
            },
        );
        if widths.len() > 1 {
            let moving_width = width - widths[0] - TABLE_GAP;
            let mut show = |ui: &mut egui::Ui| {
                ui.set_width(moving_width);
                moving_origin = ui.cursor().min;
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    table_part(ui, &cells, widths, &heights, 1..widths.len(), matched, cut);
                    if lane > 0.0 {
                        ui.add_space(lane);
                    }
                });
            };
            if scrolls {
                ui.style_mut().always_scroll_the_only_direction = false;
                let bars = if top < outer_clip.bottom() {
                    egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
                } else {
                    egui::scroll_area::ScrollBarVisibility::AlwaysHidden
                };
                let document = reader_document(ui);
                let surface = ui
                    .ctx()
                    .data(|data| data.get_temp::<egui::Id>(egui::Id::new("reader_surface")));
                let offset_id = egui::Id::new(("reader_table_offset", surface, document, rows));
                let offset = ui
                    .ctx()
                    .data(|data| data.get_temp::<f32>(offset_id))
                    .unwrap_or_default();
                let output = egui::ScrollArea::horizontal()
                    .id_salt(("reader_table", rows))
                    .horizontal_scroll_offset(offset)
                    .max_width(ui.available_width())
                    .scroll_bar_visibility(bars)
                    .show(ui, &mut show);
                ui.ctx()
                    .data_mut(|data| data.insert_temp(offset_id, output.state.offset.x));
                moving_clip = outer_clip.intersect(output.inner_rect);
            } else {
                ui.allocate_ui_with_layout(
                    egui::vec2(moving_width, height),
                    egui::Layout::top_down(egui::Align::Min),
                    &mut show,
                );
            }
        }
    });
    let painter = ui.painter().with_clip_rect(outer_clip);
    let rows_end = cut.map_or(top + height, |(lane_top, _)| (top + height).min(lane_top));
    painter.vline(
        left + widths[0],
        top..=rows_end,
        egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
    );
    if let Some((lane_top, _)) = cut {
        painter.hline(
            left..=(left + width).min(moving_clip.right()),
            lane_top,
            egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
        );
    }
    // The header follows the top edge until the final row leaves the reader.
    if let Some(header) = cells.first() {
        let header_top = top.max(outer_clip.top()).min(top + height - heights[0]);
        let header_rect = egui::Rect::from_min_max(
            egui::pos2(left, header_top),
            egui::pos2(
                (left + width).min(if widths.len() > 1 {
                    moving_clip.right()
                } else {
                    outer_clip.right()
                }),
                header_top + heights[0],
            ),
        );
        painter.rect_filled(header_rect, egui::CornerRadius::ZERO, colors().page);
        let fixed = egui::Rect::from_min_size(
            egui::pos2(left, header_top),
            egui::vec2(widths[0], heights[0]),
        );
        painter.rect_filled(fixed, egui::CornerRadius::ZERO, colors().page);
        painter.galley(
            fixed.min + egui::vec2(0.0, design::XS),
            header[0].clone(),
            colors().text,
        );
        if widths.len() > 1 {
            let painter = painter.with_clip_rect(moving_clip);
            let strip = egui::Rect::from_min_size(
                egui::pos2(moving_clip.left(), header_top),
                egui::vec2(moving_clip.width(), heights[0]),
            );
            painter.rect_filled(strip, egui::CornerRadius::ZERO, colors().page);
            let mut x = moving_origin.x;
            for column in 1..widths.len() {
                painter.galley(
                    egui::pos2(x, header_top + design::XS),
                    header[column].clone(),
                    colors().text,
                );
                x += widths[column] + TABLE_GAP;
            }
        }
        painter.hline(
            header_rect.x_range(),
            header_rect.bottom(),
            egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
        );
    }
}

fn table_part(
    ui: &mut egui::Ui,
    cells: &[Vec<std::sync::Arc<egui::Galley>>],
    widths: &[f32],
    heights: &[f32],
    columns: std::ops::Range<usize>,
    matched: Option<usize>,
    cut: Option<(f32, f32)>,
) {
    ui.spacing_mut().item_spacing.y = 0.0;
    // Under the scroll bar's lane (`cut`: its top, and the reader's visible bottom) a row is out
    // of view: it is drawn clipped at the lane, and its cells are not controls.
    let painter = match cut {
        Some((lane_top, _)) => ui.painter().with_clip_rect(
            ui.clip_rect()
                .intersect(egui::Rect::everything_above(lane_top)),
        ),
        None => ui.painter().clone(),
    };
    let width = widths[columns.clone()].iter().sum::<f32>()
        + TABLE_GAP * columns.len().saturating_sub(1) as f32;
    for (row, cells) in cells.iter().enumerate() {
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(width, heights[row]), egui::Sense::hover());
        if matched == Some(row) {
            let mut fill = rect;
            if columns.start == 0 && columns.end == 1 && widths.len() > 1 {
                fill.max.x += TABLE_GAP;
            }
            painter.rect_filled(fill, egui::CornerRadius::ZERO, colors().selected);
        }
        let mut x = rect.left();
        for column in columns.clone() {
            let cell = egui::Rect::from_min_size(
                egui::pos2(x, rect.top() + design::XS),
                cells[column].size(),
            );
            if cut.is_some_and(|(lane_top, bottom)| cell.bottom() > lane_top && cell.top() < bottom)
            {
                painter.galley(cell.min, cells[column].clone(), colors().text);
            } else {
                ui.place(cell, egui::Label::new(cells[column].clone()));
            }
            x += widths[column] + TABLE_GAP;
        }
        painter.hline(
            rect.x_range(),
            rect.bottom(),
            egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
        );
    }
}

/// Fit columns to their text without breaking words. A table that cannot fit
/// those widths scrolls within its reader.
fn column_widths(
    ui: &egui::Ui,
    rows: &[Vec<String>],
    columns: &[usize],
    display: &dyn Fn(&str) -> String,
) -> Vec<f32> {
    let font = Type::StatLabel.font(ui);
    let measure = |text: &str| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(text.to_owned(), font.clone(), colors().text)
                .size()
                .x
                .ceil()
                + design::XS
        })
    };
    let mut natural = vec![1.0_f32; columns.len()];
    let mut floor = vec![1.0_f32; columns.len()];
    for (row_index, row) in rows.iter().enumerate() {
        for (position, &column) in columns.iter().enumerate() {
            let cell = display(row.get(column).map(String::as_str).unwrap_or(""));
            natural[position] = natural[position].max(measure(&cell));
            if row_index == 0 {
                floor[position] = floor[position].max(measure(&cell));
            }
            for word in cell.split_whitespace() {
                floor[position] = floor[position].max(measure(word));
            }
        }
    }
    let gaps = TABLE_GAP * columns.len().saturating_sub(1) as f32;
    let available = (ui.available_width() - gaps - design::XS).max(1.0);
    let total: f32 = natural.iter().sum();
    if total <= available {
        return natural;
    }
    let floors: f32 = floor.iter().sum();
    if floors >= available {
        return floor;
    }
    let share = (available - floors) / (total - floors).max(1.0);
    natural
        .iter()
        .zip(&floor)
        .map(|(natural, floor)| floor + (natural - floor) * share)
        .collect()
}

fn show_fields(ui: &mut egui::Ui, rows: &[Vec<String>]) {
    let available = ui.available_width();
    let label_width = (available * 0.25).clamp(100.0, 145.0);
    let value_width = (available - label_width - 16.0).max(70.0);
    ui.push_id(ui.next_auto_id(), |ui| {
        egui::Grid::new("library_fields")
            .num_columns(2)
            .spacing([design::M, 3.0])
            .min_col_width(0.0)
            .max_col_width(value_width.max(label_width))
            .show(ui, |ui| {
                for row in rows {
                    let field = row
                        .first()
                        .map_or("", String::as_str)
                        .trim()
                        .replace('_', " ");
                    let value = row.get(1).map_or("", String::as_str).trim();
                    let value = display_field_value(value);
                    field_cell(
                        ui,
                        label_width,
                        Type::StatLabel.rich(ui, field).color(colors().text),
                    );
                    field_cell(
                        ui,
                        value_width,
                        Type::Stat.rich(ui, value).color(colors().text),
                    );
                    ui.end_row();
                }
            });
    });
}

fn field_cell(ui: &mut egui::Ui, width: f32, text: egui::RichText) {
    ui.allocate_ui_with_layout(
        egui::vec2(width, 0.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_width(width);
            ui.add(egui::Label::new(text).wrap());
        },
    );
}

fn display_field_value(value: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(value) {
        Ok(serde_json::Value::Array(values)) if values.iter().all(|v| v.is_string()) => values
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        Ok(serde_json::Value::Bool(value)) => if value { "Yes" } else { "No" }.into(),
        _ => value.to_owned(),
    }
}

/// Each level of quote moves its words this far right. Its bar is in that gap.
const QUOTE_INDENT: f32 = design::L;
const QUOTE_BAR_WIDTH: f32 = 2.0;

/// What the words waiting to be drawn link to, and the quotes they are in.
#[derive(Default)]
struct Targets {
    /// The document the reader shows, when it shows one.
    reader: Option<uuid::Uuid>,
    links: Vec<(std::ops::Range<usize>, LinkTarget)>,
    task: Option<(bool, f32, Type)>,
    /// The top of each open quote, outermost first.
    quotes: Vec<f32>,
    /// The bottom of the last words drawn in a quote, where its bar stops.
    quote_bottom: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LinkTarget {
    Missing,
    Note(String),
    Lookup(String),
    Url(String),
    /// A dice link in a stat block or a sheet. The run or the preview stream rolls it.
    Roll(splat::content::dice_text::RollLink),
}

impl LinkTarget {
    /// Where a Markdown link's destination goes, if it can be followed.
    pub(crate) fn from_destination(destination: &str) -> Option<Self> {
        if let Some(note) = destination.strip_prefix(NOTE_LINK) {
            Some(Self::Note(note.to_owned()))
        } else if let Some(link) =
            splat::content::dice_text::RollLink::from_destination(destination)
        {
            Some(Self::Roll(link))
        } else if destination.starts_with("https://") || destination.starts_with("http://") {
            Some(Self::Url(destination.to_owned()))
        } else {
            None
        }
    }
}

/// A link's underline, or a missing link's dots, sit this far below the words' baseline.
const UNDERLINE_BELOW_BASELINE: f32 = design::HAIRLINE_WIDTH * 2.0;

/// A paragraph stays one paragraph. Each link in it is its own control: Tab reaches the
/// links in reading order, Enter or Space follows the focused one, and the pointer presses
/// only a link's own target.
fn draw_with_links(
    ui: &mut egui::Ui,
    mut job: LayoutJob,
    mut links: Vec<(std::ops::Range<usize>, LinkTarget)>,
    reader: egui::Id,
) -> egui::Rect {
    // A one-line linked paragraph still needs a full target. Every row of a linked paragraph is
    // `XL` (see `lift_link_rows`), so the first row is no taller than the rest.
    job.first_row_min_height = job.first_row_min_height.max(design::XL);
    // The paragraph's own words, before any display space is added.
    let text = job.text.clone();
    reserve_short_links(ui, &mut job, &mut links);
    lift_link_rows(ui, &mut job);
    // The paragraph draws each link's underline itself, 2 px below the words' baseline, so a
    // taller row never moves it down to the bottom of the target, near the next line.
    let mut underlines: Vec<egui::Stroke> = vec![egui::Stroke::NONE; links.len()];
    for section in &mut job.sections {
        if section.format.underline == egui::Stroke::NONE {
            continue;
        }
        let range = &section.byte_range;
        if let Some(at) = links
            .iter()
            .position(|(link, _)| link.start <= range.start && range.end <= link.end)
        {
            underlines[at] = section.format.underline;
            section.format.underline = egui::Stroke::NONE;
        }
    }
    let (position, galley, response) = egui::Label::new(job)
        .wrap()
        .sense(egui::Sense::hover())
        .layout_in_ui(ui);
    ui.painter().galley(position, galley.clone(), colors().text);
    // The paragraph reads as plain text, as an unlinked one does. Its links follow as controls.
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &text));
    egui::text_selection::accesskit_text::update_accesskit_for_text_widget(
        ui.ctx(),
        response.id,
        None,
        egui::accesskit::Role::Label,
        egui::emath::TSTransform::from_translation(position.to_vec2()),
        &galley,
    );
    let words = link_words(&galley, position, &links);
    for (stroke, parts) in underlines.iter().zip(&words) {
        if *stroke == egui::Stroke::NONE {
            continue;
        }
        for part in parts {
            let y = ui
                .painter()
                .round_to_pixel_center(part.y + UNDERLINE_BELOW_BASELINE);
            ui.painter().hline(part.left..=part.right, y, *stroke);
        }
    }
    for ((_, destination), parts) in links.iter().zip(&words) {
        if matches!(destination, LinkTarget::Missing) {
            for part in parts {
                let y = part.y + UNDERLINE_BELOW_BASELINE;
                let mut dot = part.left;
                while dot < part.right {
                    ui.painter().circle_filled(
                        egui::pos2(dot, y),
                        design::HAIRLINE_WIDTH / 2.0,
                        colors().muted,
                    );
                    dot += design::XS;
                }
            }
        }
    }
    let targets = link_targets(&words, response.rect);
    let links_id = paragraph_id(ui.ctx(), reader, &text);
    for (index, ((range, destination), rects)) in links.iter().zip(&targets).enumerate() {
        let words_text = &galley.job.text[range.clone()];
        let id = links_id.with(index);
        // Only the first part of a wrapped link takes focus, so Tab stops once per link.
        let parts: Vec<egui::Response> = rects
            .iter()
            .enumerate()
            .map(|(part, rect)| {
                let first = part == 0 && !matches!(destination, LinkTarget::Missing);
                let sense = if first {
                    egui::Sense::click()
                } else {
                    egui::Sense::CLICK
                };
                let part_response = ui.interact(*rect, id.with(part), sense);
                let role = if first {
                    egui::WidgetType::Link
                } else {
                    egui::WidgetType::Label
                };
                // The name is built only when an accessibility reader or a test asks for it.
                part_response.widget_info(|| {
                    egui::WidgetInfo::labeled(role, true, link_name(words_text, destination))
                });
                part_response
            })
            .collect();
        if parts.first().is_some_and(egui::Response::has_focus) {
            // The ring sits `XS / 2` outside the words, so it never covers a letter, even for a
            // link at the start of a line. It stays inside the reader's visible area, which
            // keeps a gutter beside the text for it.
            for part in &words[index] {
                let ring = part.rect.expand2(egui::vec2(design::XS / 2.0, 0.0));
                design::focus_ring(ui, ring.intersect(ui.clip_rect()));
            }
        }
        // A click by key or pointer wins over the pointer resting on another row of the link.
        let Some(pressed) = parts
            .iter()
            .find(|part| part.clicked())
            .or_else(|| parts.iter().find(|part| part.hovered()))
        else {
            continue;
        };
        if let LinkTarget::Roll(link) = destination {
            crate::rolls::interact(ui, pressed, link);
            continue;
        }
        match press(ui, pressed, destination) {
            Some(LinkTarget::Note(note)) => {
                let from = reader_document(ui);
                send_note_link(ui.ctx(), note.clone(), from);
            }
            Some(LinkTarget::Lookup(words)) => ui.ctx().data_mut(|data| {
                data.insert_temp(egui::Id::new("markdown_clicked_lookup"), words.clone());
            }),
            _ => {}
        }
    }
    response.rect
}

/// A link whose words are narrower than `XL` gets `XL` of layout width. A link inside a line has
/// its words in the middle of that space, so the spare width is split before and after them. A
/// link that starts a line keeps its words at the text edge, with all the space after them. The
/// text after a link starts after its space, so the line wraps if it must. The space is display
/// only: the paragraph's words do not change. Links `XL` or wider keep their layout.
fn reserve_short_links(
    ui: &egui::Ui,
    job: &mut LayoutJob,
    links: &mut [(std::ops::Range<usize>, LinkTarget)],
) {
    let mut measure = job.clone();
    measure.wrap.max_width = f32::INFINITY;
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(measure));
    // Each short link, by its index in `links`, and the space it still needs.
    let short: Vec<(usize, f32)> = link_words(&galley, egui::Pos2::ZERO, links)
        .iter()
        .enumerate()
        .filter_map(|(index, parts)| {
            let width: f32 = parts.iter().map(|part| part.right - part.left).sum();
            // Glyphs snap to whole pixels, which can take up to a pixel from the space.
            (!parts.is_empty() && width < design::XL).then_some((index, design::XL - width + 1.0))
        })
        .collect();
    if short.is_empty() {
        return;
    }
    // A link at the start of the paragraph or after a line break starts a line. One that wraps to
    // the start of a row is found by laying the paragraph out at the width its label wraps at.
    // Moving a link's space after it can only change the rows after that link, so each pass
    // keeps the links it found.
    let mut starts: Vec<bool> = short
        .iter()
        .map(|(index, _)| {
            let start = links[*index].0.start;
            start == 0 || job.text[..start].ends_with('\n')
        })
        .collect();
    let original = (job.clone(), links.to_vec());
    for _ in 0..4 {
        *job = original.0.clone();
        links.clone_from_slice(&original.1);
        place_short_links(ui, job, links, &short, &starts);
        let mut measure = job.clone();
        measure.wrap.max_width = ui.available_width();
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(measure));
        let words = link_words(&galley, egui::Pos2::ZERO, links);
        let mut changed = false;
        for ((index, _), starts) in short.iter().zip(&mut starts) {
            if !*starts && words[*index].first().is_some_and(|part| part.row_start) {
                *starts = true;
                changed = true;
            }
        }
        if !changed {
            return;
        }
    }
    *job = original.0;
    links.clone_from_slice(&original.1);
    place_short_links(ui, job, links, &short, &starts);
}

/// Adds each short link's space: half before and half after its words, or all after them when
/// it starts a line.
fn place_short_links(
    ui: &egui::Ui,
    job: &mut LayoutJob,
    links: &mut [(std::ops::Range<usize>, LinkTarget)],
    short: &[(usize, f32)],
    starts: &[bool],
) {
    // Where each short link ends, and the space that still goes there, in text order.
    let mut spaces: Vec<(usize, f32)> = Vec::new();
    // Where the last short link ended, and the space after it.
    let mut previous: Option<(usize, f32)> = None;
    for (&(index, space), &starts) in short.iter().zip(starts) {
        let range = links[index].0.clone();
        let own = job.sections.iter_mut().find(|section| {
            section.byte_range.start == range.start && !section.byte_range.is_empty()
        });
        // A row drops the space before its first glyph, so a link that starts a line keeps its
        // words at the text edge.
        let before = match own {
            Some(own) if !starts => {
                own.leading_space += space / 2.0;
                space / 2.0
            }
            _ => 0.0,
        };
        let mut space = space - before;
        let at_line_end = job.text[range.end..].starts_with('\n') || range.end == job.text.len();
        let link_follows = links.iter().any(|(other, _)| other.start == range.end);
        // A link right after another short link has the first one's space before it as well as
        // its own. It gets as much space after it, so the pair reads evenly; a blank that
        // follows in the text counts toward it.
        if let Some((_, after)) = previous.filter(|(end, _)| *end == range.start)
            && !at_line_end
            && !link_follows
        {
            let next = job.text[range.end..].chars().next();
            let blank = next
                .filter(|next| next.is_whitespace())
                .map_or(0.0, |next| {
                    let font = job
                        .sections
                        .iter()
                        .find(|section| section.byte_range.start == range.end)
                        .map(|section| section.format.font_id.clone());
                    font.map_or(0.0, |font| {
                        ui.fonts_mut(|fonts| fonts.glyph_width(&font, next))
                    })
                });
            space += (after - blank).max(0.0);
        }
        previous = Some((range.end, space));
        match job
            .sections
            .iter_mut()
            .find(|section| section.byte_range.start == range.end && !section.byte_range.is_empty())
        {
            // The text after the link starts after the reserved space.
            Some(next) if !at_line_end && !link_follows => next.leading_space += space,
            // A blank holds the space: where nothing follows on the line, so the paragraph's own
            // width includes it, and before another link, so one underline does not join both.
            _ => spaces.push((range.end, space)),
        }
    }
    spaces.sort_by_key(|(at, _)| *at);
    if spaces.is_empty() {
        return;
    }
    let mut text = String::with_capacity(job.text.len() + spaces.len());
    let mut sections = Vec::with_capacity(job.sections.len() + spaces.len());
    let mut pending = spaces.iter().peekable();
    for section in &job.sections {
        let start = text.len();
        text.push_str(&job.text[section.byte_range.clone()]);
        let mut moved = section.clone();
        moved.byte_range = start..text.len();
        sections.push(moved);
        while let Some(&&(at, space)) = pending.peek() {
            if at > section.byte_range.end {
                break;
            }
            pending.next();
            if at < section.byte_range.end {
                continue;
            }
            let mut format = section.format.clone();
            format.underline = egui::Stroke::NONE;
            format.strikethrough = egui::Stroke::NONE;
            format.background = egui::Color32::TRANSPARENT;
            let blank = ui.fonts_mut(|fonts| fonts.glyph_width(&format.font_id, ' '));
            let start = text.len();
            text.push(' ');
            sections.push(egui::text::LayoutSection {
                leading_space: (space - blank).max(0.0),
                byte_range: start..text.len(),
                format,
            });
        }
    }
    let added = |at: usize, inclusive: bool| {
        spaces
            .iter()
            .filter(|(space, _)| *space < at || (inclusive && *space == at))
            .count()
    };
    for (range, _) in links.iter_mut() {
        *range = range.start + added(range.start, true)..range.end + added(range.end, false);
    }
    job.text = text;
    job.sections = sections;
}

/// Where the body line is shorter than `XL`, every row of a paragraph that holds a link grows to
/// `XL`, so each link's target is `XL` tall and stays on its own row, and the paragraph keeps one
/// line pitch. Every part of a row grows by the same amount, so the words keep their baselines.
/// A paragraph without a link keeps its line height.
fn lift_link_rows(ui: &egui::Ui, job: &mut LayoutJob) {
    // Reader prose is already `XL` tall, so it needs no second layout.
    if job.sections.iter().all(|section| {
        section
            .format
            .line_height
            .is_some_and(|line| line >= design::XL)
    }) {
        return;
    }
    let mut measure = job.clone();
    // The width the paragraph's label wraps at.
    measure.wrap.max_width = ui.available_width();
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(measure));
    let mut lifts: Vec<(std::ops::Range<usize>, f32)> = Vec::new();
    let mut byte_at = 0;
    for row in &galley.rows {
        let start = byte_at;
        byte_at += row
            .glyphs
            .iter()
            .map(|glyph| glyph.chr.len_utf8())
            .sum::<usize>();
        let end = byte_at;
        if row.ends_with_newline {
            byte_at += 1;
        }
        if row.height() < design::XL {
            lifts.push((start..end, design::XL - row.height()));
        }
    }
    if lifts.is_empty() {
        return;
    }
    // Cut the sections at the lifted rows' edges, then lift the parts inside those rows.
    let mut sections = Vec::with_capacity(job.sections.len() + 2 * lifts.len());
    for section in &job.sections {
        let range = section.byte_range.clone();
        let mut cuts: Vec<usize> = lifts
            .iter()
            .flat_map(|(row, _)| [row.start, row.end])
            .filter(|cut| range.start < *cut && *cut < range.end)
            .collect();
        cuts.sort_unstable();
        cuts.dedup();
        cuts.push(range.end);
        let mut from = range.start;
        for cut in cuts {
            let mut part = section.clone();
            if from != range.start {
                part.leading_space = 0.0;
            }
            part.byte_range = from..cut;
            if let Some((_, extra)) = lifts
                .iter()
                .find(|(row, _)| from < cut && row.start <= from && cut <= row.end)
            {
                let line = part.format.line_height.unwrap_or_else(|| {
                    ui.fonts_mut(|fonts| fonts.row_height(&part.format.font_id))
                });
                part.format.line_height = Some(line + extra);
            }
            sections.push(part);
            from = cut;
        }
    }
    job.sections = sections;
}

/// The document the reader being drawn shows, if it shows one. A page with a document sets
/// `reader_document` just before its reader. The first reader to read it takes it, with the pass
/// and the panel it was taken in, so the rest of that page's reader shares it. A reader in
/// another panel or a later pass, with no new value, has no document: it never takes the one
/// that another page left behind (Sessions notes and Lookup's written-note card have none).
fn reader_document(ui: &egui::Ui) -> Option<uuid::Uuid> {
    #[derive(Clone, Copy)]
    struct Taken {
        pass: u64,
        panel: egui::Id,
        document: uuid::Uuid,
    }
    let pass = ui.ctx().cumulative_pass_nr();
    let panel = ui
        .stack()
        .iter()
        .find(|frame| frame.is_panel_ui() || frame.is_area_ui())
        .map_or(egui::Id::NULL, |frame| frame.id);
    let taken_id = egui::Id::new("markdown_reader_document_taken");
    ui.ctx().data_mut(|data| {
        if let Some(document) = data.remove_temp::<uuid::Uuid>(egui::Id::new("reader_document")) {
            data.insert_temp(
                taken_id,
                Taken {
                    pass,
                    panel,
                    document,
                },
            );
            return Some(document);
        }
        data.get_temp::<Taken>(taken_id)
            .filter(|taken| taken.pass == pass && taken.panel == panel)
            .map(|taken| taken.document)
    })
}

/// The paragraph's links keep their ids, and so keyboard focus, when the layout around them
/// changes: at a breakpoint, or when a pane opens beside the reader. The id comes from the
/// reader and the paragraph's words, so another reader with the same words, drawn earlier,
/// does not move it. The same words twice in one reader count up in drawing order.
fn paragraph_id(ctx: &egui::Context, reader: egui::Id, text: &str) -> egui::Id {
    #[derive(Clone, Default)]
    struct Seen {
        pass: u64,
        count: std::collections::HashMap<egui::Id, u32>,
    }
    let words = reader.with(("markdown_paragraph_links", text));
    let pass = ctx.cumulative_pass_nr();
    let nth = ctx.data_mut(|data| {
        let seen = data.get_temp_mut_or_default::<Seen>(egui::Id::new("markdown_paragraph_seen"));
        if seen.pass != pass {
            *seen = Seen {
                pass,
                ..Default::default()
            };
        }
        let nth = seen.count.entry(words).or_default();
        *nth += 1;
        *nth
    });
    words.with(nth)
}

/// One row's part of a link's words, on the screen.
#[derive(Clone, Copy, Debug)]
struct LinkPart {
    left: f32,
    right: f32,
    /// The words' baseline area, where a missing link's dots go.
    y: f32,
    /// The whole row's height across the words.
    rect: egui::Rect,
    /// The words are the first on their row, at the text edge.
    row_start: bool,
}

/// Where each link's words are drawn, one part for each row it is on.
fn link_words(
    galley: &egui::Galley,
    position: egui::Pos2,
    links: &[(std::ops::Range<usize>, LinkTarget)],
) -> Vec<Vec<LinkPart>> {
    links
        .iter()
        .map(|(range, _)| {
            let mut parts = Vec::new();
            let mut byte_at = 0;
            for row in &galley.rows {
                let mut run: Option<(f32, f32, f32)> = None;
                let mut row_start = false;
                for (at, glyph) in row.glyphs.iter().enumerate() {
                    if range.contains(&byte_at) {
                        row_start |= at == 0;
                        let x = position.x + row.pos.x + glyph.pos.x;
                        let y = position.y + row.pos.y + glyph.pos.y;
                        let start = run.map_or(x, |(start, _, _)| start);
                        run = Some((start, x + glyph.advance_width, y));
                    }
                    byte_at += glyph.chr.len_utf8();
                }
                if let Some((left, right, y)) = run {
                    let row_rect = row.rect().translate(position.to_vec2());
                    parts.push(LinkPart {
                        left,
                        right,
                        y,
                        rect: egui::Rect::from_x_y_ranges(left..=right, row_rect.y_range()),
                        row_start,
                    });
                }
                if row.ends_with_newline {
                    byte_at += 1;
                }
            }
            parts
        })
        .collect()
}

/// Each link part's press target: at least `XL` square where the paragraph has room. Inside a
/// line it is centred on the words, as `reserve_short_links` centres them in their space; at the
/// start of a row it grows right from the text edge. Two targets never share a point: where
/// grown targets meet, they split halfway between the words. The paragraph's own layout stays.
fn link_targets(words: &[Vec<LinkPart>], bounds: egui::Rect) -> Vec<Vec<egui::Rect>> {
    let grow = |part: &LinkPart| {
        let words = part.rect;
        let width = words.width().max(design::XL);
        let left = if part.row_start {
            words.left()
        } else {
            (words.center().x - width / 2.0).max(bounds.left())
        };
        let mut x = left..=left + width;
        if *x.end() > bounds.right() {
            let left = (bounds.right() - width).max(bounds.left());
            x = left..=bounds.right().max(words.right());
        }
        let height = words.height().max(design::XL);
        let mut top = words.center().y - height / 2.0;
        top = top.min(bounds.bottom() - height).max(bounds.top());
        // Inside the paragraph, so a target never reaches past its own text.
        egui::Rect::from_x_y_ranges(x, top..=top + height).intersect(bounds)
    };
    let mut targets: Vec<Vec<egui::Rect>> = words
        .iter()
        .map(|parts| parts.iter().map(grow).collect())
        .collect();
    let all: Vec<(usize, usize)> = words
        .iter()
        .enumerate()
        .flat_map(|(link, parts)| (0..parts.len()).map(move |part| (link, part)))
        .collect();
    for (at, &(a_link, a_part)) in all.iter().enumerate() {
        for &(b_link, b_part) in &all[at + 1..] {
            let shared = targets[a_link][a_part].intersect(targets[b_link][b_part]);
            if shared.width() <= 0.0 || shared.height() <= 0.0 {
                continue;
            }
            let (a_words, b_words) = (words[a_link][a_part].rect, words[b_link][b_part].rect);
            let same_row = a_words.y_range().intersection(b_words.y_range()).span() > 0.0;
            let (first, second, split) = if same_row {
                let (left, right) = if a_words.center().x <= b_words.center().x {
                    ((a_link, a_part), (b_link, b_part))
                } else {
                    ((b_link, b_part), (a_link, a_part))
                };
                let mid = (words[left.0][left.1].rect.right()
                    + words[right.0][right.1].rect.left())
                    / 2.0;
                (left, right, (mid, true))
            } else {
                let (upper, lower) = if a_words.center().y <= b_words.center().y {
                    ((a_link, a_part), (b_link, b_part))
                } else {
                    ((b_link, b_part), (a_link, a_part))
                };
                let mid = (words[upper.0][upper.1].rect.bottom()
                    + words[lower.0][lower.1].rect.top())
                    / 2.0;
                (upper, lower, (mid, false))
            };
            let (mid, sideways) = split;
            if sideways {
                targets[first.0][first.1].max.x = targets[first.0][first.1].max.x.min(mid);
                targets[second.0][second.1].min.x = targets[second.0][second.1].min.x.max(mid);
            } else {
                targets[first.0][first.1].max.y = targets[first.0][first.1].max.y.min(mid);
                targets[second.0][second.1].min.y = targets[second.0][second.1].min.y.max(mid);
            }
        }
    }
    targets
}

/// A link's accessible name: its words, then where it goes.
fn link_name(words: &str, destination: &LinkTarget) -> String {
    match destination {
        LinkTarget::Missing => format!("{words} (not found in this campaign)"),
        LinkTarget::Note(note) => format!("{words} (opens {note})"),
        LinkTarget::Lookup(_) => format!("{words} (opens in Lookup)"),
        LinkTarget::Url(url) => format!("{words} (opens {url})"),
        LinkTarget::Roll(link) => crate::rolls::hint(link),
    }
}

/// Show that the pointer is on a link that can be pressed. A pressed web link opens here.
/// Another pressed link goes back to the caller, which knows where it opens.
pub(crate) fn press<'a>(
    ui: &egui::Ui,
    response: &egui::Response,
    destination: &'a LinkTarget,
) -> Option<&'a LinkTarget> {
    if matches!(destination, LinkTarget::Missing) {
        response.clone().on_hover_text("Not found in this campaign");
        return None;
    }
    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    if let LinkTarget::Url(url) = destination {
        response.clone().on_hover_text(url);
    }
    // A linked world page says what it is before the DM opens it.
    if let LinkTarget::Note(note) = destination
        && response.hovered()
        && let Some(index) = ui.ctx().data(|data| {
            data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))
        })
    {
        let from = reader_document(ui);
        if let crate::world::Go::Item(at, _) = index.resolve_link(note, from)
            && !index.items[at].summary().is_empty()
        {
            response.clone().on_hover_text(index.items[at].summary());
        }
    }
    if !response.clicked() {
        return None;
    }
    if let LinkTarget::Url(url) = destination {
        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
        return None;
    }
    Some(destination)
}

/// Give a pressed note link to whichever page takes it next. `from` is the note it is in.
pub fn send_note_link(ctx: &egui::Context, note: String, from: Option<uuid::Uuid>) {
    ctx.data_mut(|data| {
        data.insert_temp(egui::Id::new("clicked_reader_document"), from);
        data.insert_temp(clicked_id(), note);
    });
}

fn flush(ui: &mut egui::Ui, job: &mut LayoutJob, target: &mut Targets) {
    if job.is_empty() {
        return;
    }
    let links = std::mem::take(&mut target.links);
    let task = target.task.take();
    let reader = egui::Id::new(("markdown_reader", target.reader));
    let job = std::mem::take(job);
    let draw = |ui: &mut egui::Ui| {
        let rect = if links.is_empty() {
            ui.add(egui::Label::new(job).wrap()).rect
        } else {
            draw_with_links(ui, job, links, reader)
        };
        if let Some((done, indent, role)) = task {
            let icon_rect = egui::Rect::from_min_size(
                rect.min + egui::vec2(indent, 0.0),
                egui::vec2(role.size(), role.line()),
            );
            ui.interact(
                icon_rect,
                ui.next_auto_id().with("task-marker"),
                egui::Sense::hover(),
            )
            .widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Label,
                    true,
                    if done { "Done" } else { "Open" },
                )
            });
        }
    };
    if target.quotes.is_empty() {
        draw(ui);
    } else {
        let rect = ui
            .horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.add_space(QUOTE_INDENT * target.quotes.len() as f32);
                ui.vertical(draw);
            })
            .response
            .rect;
        target.quote_bottom = rect.bottom();
    }
    ui.add_space(design::S);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_markers_replace_only_leading_list_syntax() {
        let markdown = "- [ ] Can a ring be sold?\n- [x] Decided.\n- Keep [ ] and [x] in this line.\n\n[ ] A paragraph, not a task.\n\n`[x] Code stays text.`\n";
        let text = painted(markdown)
            .iter()
            .map(|galley| galley.job.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            text.contains(&format!(
                "{} Can a ring be sold?",
                egui_phosphor::regular::SQUARE
            )),
            "FD-2-open: {text}"
        );
        assert!(
            text.contains(&format!(
                "{} Decided.",
                egui_phosphor::regular::CHECK_SQUARE
            )),
            "FD-2-answered: {text}"
        );
        for literal in [
            "Keep [ ] and [x] in this line.",
            "[ ] A paragraph, not a task.",
            "[x] Code stays text.",
        ] {
            assert!(
                text.contains(literal),
                "FD-2-literal: missing {literal:?} in {text}"
            );
        }
        // A numbered task keeps its number; the box follows it.
        let numbered = painted("1. [ ] Ask the duke\n2. [x] Pay the toll\n")
            .iter()
            .map(|galley| galley.job.text.clone())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            numbered.contains(&format!(
                "1. {} Ask the duke",
                egui_phosphor::regular::SQUARE
            )) && numbered.contains(&format!(
                "2. {} Pay the toll",
                egui_phosphor::regular::CHECK_SQUARE
            )),
            "FD-2-numbered: {numbered}"
        );
        assert_eq!(
            click_words("- [ ] [Read the note](splat-note:Rook)", "Read the note")
                .0
                .as_deref(),
            Some("Rook")
        );
    }

    #[test]
    fn wide_tables_keep_whole_words_and_can_scroll_to_the_last_column() {
        let markdown = "| Level | Proficiency Bonus | Features | Cantrips | Prepared Spells | 1st | 2nd | 3rd | 4th | 5th | 6th | 7th | 8th | 9th |\n\
                        | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n\
                        | 5 | +3 | Memorize Spell | 4 | 9 | 4 | 3 | 2 | 0 | 0 | 0 | 0 | 0 | Ninth-slot |\n";
        for width in [280.0, 360.0, 488.0] {
            let context = egui::Context::default();
            design::theme::install(&context);
            let mut output = None;
            for frame in 0..30 {
                let events = if frame == 2 {
                    vec![
                        egui::Event::PointerMoved(egui::pos2(100.0, 80.0)),
                        egui::Event::MouseWheel {
                            unit: egui::MouseWheelUnit::Point,
                            delta: egui::vec2(-2000.0, 0.0),
                            modifiers: egui::Modifiers::NONE,
                        },
                    ]
                } else {
                    vec![]
                };
                let rendered = context.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 500.0),
                        )),
                        time: Some(f64::from(frame) / 60.0),
                        events,
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| show(ui, markdown));
                    },
                );
                if frame == 1 {
                    let words = rendered.shapes.iter().find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) if text.galley.job.text == "Memorize Spell" => {
                            Some(
                                text.galley
                                    .rows
                                    .iter()
                                    .map(|row| row.text())
                                    .collect::<Vec<_>>(),
                            )
                        }
                        _ => None,
                    });
                    let words = words.expect("the feature column is painted");
                    assert!(
                        words.iter().any(|row| row.contains("Memorize"))
                            && words.iter().any(|row| row.contains("Spell")),
                        "table at {width}px breaks words: {words:?}"
                    );
                }
                output = Some(rendered);
            }
            let output = output.unwrap();
            let last_column: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == "Ninth-slot" => {
                        Some((shape.clip_rect, text.visual_bounding_rect()))
                    }
                    _ => None,
                })
                .collect();
            assert!(
                output.shapes.iter().any(|shape| {
                    matches!(&shape.shape, egui::Shape::Text(text)
                    if text.galley.job.text == "Ninth-slot"
                        && shape.clip_rect.contains_rect(text.visual_bounding_rect()))
                }),
                "the last column must be readable after horizontal scrolling at {width}px: {last_column:?}"
            );
        }
    }

    #[test]
    fn missing_links_have_even_dots_below_the_baseline() {
        let context = egui::Context::default();
        design::theme::install(&context);
        let output = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, "[Missing place](splat-missing:Gone)");
            });
        });
        let dots: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match shape.shape {
                egui::Shape::Circle(ref circle) => Some(circle.center),
                _ => None,
            })
            .collect();
        assert!(dots.len() > 10, "missing underline: {dots:?}");
        for pair in dots.windows(2) {
            assert_eq!(pair[1].x - pair[0].x, design::XS);
        }
        let text = output
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text)
                    if text.galley.job.text == format!("{} Missing place", design::icons::WARN) =>
                {
                    Some(text)
                }
                _ => None,
            })
            .unwrap();
        let row = &text.galley.rows[0];
        let word = row.glyphs.iter().find(|glyph| glyph.chr == 'M').unwrap();
        let baseline = text.pos.y + row.pos.y + word.pos.y;
        assert_eq!(dots[0].y, baseline + design::HAIRLINE_WIDTH * 2.0);
    }

    /// Click the first letter of these words in the actual laid-out paragraph.
    fn click_words(markdown: &str, words: &str) -> (Option<String>, Option<String>, Vec<String>) {
        let context = egui::Context::default();
        design::theme::install(&context);
        click_words_in(&context, markdown, words)
    }

    fn click_words_in(
        context: &egui::Context,
        markdown: &str,
        words: &str,
    ) -> (Option<String>, Option<String>, Vec<String>) {
        let draw = |ctx: &egui::Context| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, markdown));
        };
        let output = context.run(egui::RawInput::default(), draw);
        let pointer = output
            .shapes
            .iter()
            .find_map(|shape| {
                let egui::Shape::Text(text) = &shape.shape else {
                    return None;
                };
                let byte = text.galley.job.text.find(words)?;
                let at = text.galley.job.text[..byte].chars().count();
                let cursor = |at| {
                    text.galley
                        .pos_from_cursor(egui::text::CCursor::new(at))
                        .center()
                };
                Some(text.pos + ((cursor(at).to_vec2() + cursor(at + 1).to_vec2()) / 2.0))
            })
            .unwrap_or_else(|| panic!("no words {words:?}"));
        let mut urls = Vec::new();
        for pressed in [true, false] {
            let output = context.run(
                egui::RawInput {
                    events: vec![
                        egui::Event::PointerMoved(pointer),
                        egui::Event::PointerButton {
                            pos: pointer,
                            button: egui::PointerButton::Primary,
                            pressed,
                            modifiers: egui::Modifiers::NONE,
                        },
                    ],
                    ..Default::default()
                },
                draw,
            );
            for command in output.platform_output.commands {
                if let egui::OutputCommand::OpenUrl(url) = command {
                    urls.push(url.url);
                }
            }
        }
        (take_note_link(context), take_lookup_link(context), urls)
    }

    #[test]
    fn unresolved_bold_and_bold_italic_stay_emphasis() {
        for (markdown, words) in [
            ("**Oren** speaks.", "Oren"),
            ("***nobody*** answers.", "nobody"),
            ("__Véldo & Sons__ arrive.", "Sons"),
            ("**Docket __the clerk__ arrives**.", "arrives"),
        ] {
            assert_eq!(click_words(markdown, words), (None, None, vec![]));
            let galleys = painted(markdown);
            for galley in galleys {
                for section in &galley.job.sections {
                    assert_eq!(section.format.color, colors().text);
                    assert_eq!(section.format.underline, egui::Stroke::NONE);
                }
            }
        }
    }

    #[test]
    fn resolved_bold_spell_keeps_one_click_lookup() {
        let context = egui::Context::default();
        design::theme::install(&context);
        let index = Arc::new(crate::lookup::ContentIndex::load(&[], None).unwrap());
        assert!(matches!(
            index.resolve_link("Fireball", None),
            crate::world::Go::Item(..)
        ));
        context.data_mut(|data| data.insert_temp(egui::Id::new("reader_link_index"), index));
        for markdown in ["Cast **Fireball**.", "Cast ***Fireball***."] {
            assert_eq!(
                click_words_in(&context, markdown, "Fireball"),
                (None, Some("Fireball".into()), vec![])
            );
        }
        assert_eq!(
            click_words_in(&context, "Cast [[Fireball]].", "Fireball"),
            (Some("Fireball".into()), None, vec![])
        );
        assert_eq!(
            click_words_in(&context, "***nobody*** answers.", "nobody"),
            (None, None, vec![])
        );
    }

    #[test]
    fn explicit_links_keep_their_targets_inside_bold_text() {
        for markdown in [
            "**[the clerk](splat-note:Docket)**",
            "[**the clerk**](splat-note:Docket)",
        ] {
            assert_eq!(
                click_words(markdown, "the clerk"),
                (Some("Docket".into()), None, vec![])
            );
        }
        let markdown = "**Docket** reads [**the rules**](https://example.com).";
        assert_eq!(
            click_words(markdown, "the rules"),
            (None, None, vec!["https://example.com".into()])
        );
        assert_eq!(click_words(markdown, "reads"), (None, None, vec![]));
        assert_eq!(click_words(markdown, "Docket"), (None, None, vec![]));
    }

    #[test]
    fn headings_and_code_do_not_become_lookup_links() {
        for markdown in [
            "## **Docket**\n",
            "`**Docket**`",
            "```\n**Docket**\n```",
            "**`Docket`**",
        ] {
            assert_eq!(click_words(markdown, "Docket"), (None, None, vec![]));
        }
    }

    #[test]
    fn brackets_go_only_from_links_with_a_local_image() {
        let text =
            "*A map* ([image](https://a/map.png)) and *a lost one* ([image](https://a/lost.png)).";
        let result = local_figure_links(text, |url| url.ends_with("map.png"));
        assert_eq!(
            result,
            "*A map* [image](https://a/map.png) and *a lost one* ([image](https://a/lost.png))."
        );
        assert_eq!(local_figure_links(text, |_| false), text);
    }

    /// The Reader's parts of a note, painted.
    fn painted(markdown: &str) -> Vec<std::sync::Arc<egui::Galley>> {
        let context = egui::Context::default();
        design::theme::install(&context);
        let output = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, markdown));
        });
        output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.clone()),
                _ => None,
            })
            .collect()
    }

    fn format_of(galleys: &[std::sync::Arc<egui::Galley>], words: &str) -> egui::TextFormat {
        galleys
            .iter()
            .flat_map(|galley| {
                galley
                    .job
                    .sections
                    .iter()
                    .filter(|part| &galley.job.text[part.byte_range.clone()] == words)
                    .map(|part| part.format.clone())
            })
            .next()
            .unwrap_or_else(|| panic!("no words {words:?}"))
    }

    #[test]
    fn bold_keeps_its_weight_and_only_explicit_links_get_link_color() {
        let galleys = painted("**Docket** speaks. **Read [the rules](https://example.com)**.");
        let name = format_of(&galleys, "Docket");
        assert_eq!(
            name.font_id.family,
            egui::FontFamily::Name(design::FAMILY_SERIF_SEMIBOLD.into())
        );
        assert_eq!(name.color, colors().text);
        assert_eq!(name.underline, egui::Stroke::NONE);
        assert_eq!(format_of(&galleys, "Read ").color, colors().text);
        assert_eq!(format_of(&galleys, "the rules").color, colors().accent);
    }

    #[test]
    fn quotes_are_indented_beside_a_bar() {
        let context = egui::Context::default();
        design::theme::install(&context);
        let output = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, "Before\n\n> Outer\n>\n> > Inner\n\nAfter\n");
            });
        });
        let text = |words: &str| {
            output
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == words => {
                        Some(text.galley.rect.translate(text.pos.to_vec2()))
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("{words} was not drawn"))
        };
        let (before, outer, inner, after) =
            (text("Before"), text("Outer"), text("Inner"), text("After"));
        assert!((outer.left() - before.left() - QUOTE_INDENT).abs() < 1.0);
        assert!((inner.left() - before.left() - 2.0 * QUOTE_INDENT).abs() < 1.0);
        assert!((after.left() - before.left()).abs() < 1.0);
        let bars: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::LineSegment { points, stroke }
                    if stroke.color == colors().rule && points[0].x == points[1].x =>
                {
                    Some(egui::Rect::from_two_pos(points[0], points[1]))
                }
                _ => None,
            })
            .collect();
        assert_eq!(bars.len(), 2, "one bar for each quote");
        for (bar, words) in bars.iter().zip([inner, outer]) {
            assert!(bar.left() < words.left());
            assert!(bar.top() <= words.top() && bar.bottom() >= words.bottom());
            assert!(bar.bottom() < after.top());
        }
        assert!(bars[1].top() <= outer.top() && bars[1].bottom() >= inner.bottom());
    }

    #[test]
    fn code_and_struck_words_show_in_the_reader() {
        let galleys = painted("Roll `1d20` ~~old~~ now\n");
        let code = format_of(&galleys, "1d20");
        assert_eq!(code.font_id.family, egui::FontFamily::Monospace);
        assert_ne!(code.background, egui::Color32::TRANSPARENT);
        assert_ne!(format_of(&galleys, "old").strikethrough, egui::Stroke::NONE);
        assert_eq!(
            format_of(&galleys, " now").strikethrough,
            egui::Stroke::NONE
        );
    }
    #[test]
    fn inventory_reader_shows_names_without_exposing_storage_ids() {
        let context = egui::Context::default();
        let output = context.run(egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(600.0, 500.0))),
            ..Default::default()
        }, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, "<!-- splat:inventory -->\n\n| Key | Item | Quantity | Equipment | Attuned | Link |\n| --- | --- | --- | --- | --- | --- |\n| item_123 | `3302150c-7356-5633-9215-20d96c893348` | 2 | carried | false | Dagger |\n\n## Abilities\n\n| Ability | Score |\n| --- | --- |\n| Strength | 17 |\n");
            });
        });
        let text: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text),
                _ => None,
            })
            .collect();
        assert!(text.iter().any(|text| text.galley.job.text == "Dagger"));
        assert!(
            !text
                .iter()
                .any(|text| text.galley.job.text.contains("item_123")
                    || text.galley.job.text.contains("3302150c"))
        );
        assert!(
            text.iter()
                .any(|text| text.galley.job.text == "Strength" && text.galley.rows.len() == 1)
        );
    }

    /// A scroll repaints the reader on each frame. Its links must not be resolved again until
    /// the note or the index changes.
    #[test]
    fn a_note_keeps_its_resolved_links_while_it_is_unchanged() {
        let ctx = egui::Context::default();
        let index = crate::lookup::ContentIndex::load(&[], None).unwrap();
        let reloaded = crate::lookup::ContentIndex::load(&[], None).unwrap();
        let note = "[[Wolf]] and [[Gone]]";
        let mut seen = Vec::new();
        for (index, note) in [
            (&index, note),
            (&index, note),
            (&index, "[[Wolf]]"),
            (&reloaded, "[[Wolf]]"),
        ] {
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                seen.push(linked_markdown(ctx, index, note, None));
            });
        }
        assert!(
            Arc::ptr_eq(&seen[0], &seen[1]),
            "the same note, frame after frame"
        );
        assert!(!Arc::ptr_eq(&seen[1], &seen[2]), "a changed note");
        assert!(!Arc::ptr_eq(&seen[2], &seen[3]), "a reloaded index");
        assert_eq!(&*seen[2], &*seen[3]);
        assert_eq!(*seen[0], *index.reader_markdown(note, None));
    }

    /// The Reader with a paragraph, in a test harness with the app's theme.
    fn reader(markdown: &'static str) -> egui_kittest::Harness<'static> {
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(480.0, 320.0))
            .build_ui(move |ui| show(ui, markdown));
        design::theme::install(&harness.ctx);
        harness.run_ok();
        harness
    }

    fn focused<S>(harness: &egui_kittest::Harness<'_, S>) -> Option<String> {
        use egui_kittest::kittest::NodeT;
        harness
            .root()
            .children_recursive()
            .find(|node| node.is_focused())
            .and_then(|node| node.accesskit_node().label())
    }

    fn link_names(harness: &egui_kittest::Harness<'_>) -> Vec<String> {
        use egui_kittest::kittest::NodeT;
        harness
            .root()
            .children_recursive()
            .filter(|node| node.accesskit_node().role() == egui::accesskit::Role::Link)
            .filter_map(|node| node.accesskit_node().label())
            .collect()
    }

    #[test]
    fn iter16_a_ad1_tab_reaches_each_link_and_enter_follows_the_focused_one() {
        let mut harness =
            reader("Ask [Wren](splat-note:Wren) or [Oskar](splat-note:Oskar) about the bells.");
        // Each link is its own control with its own name: its words and where it goes.
        assert_eq!(
            link_names(&harness),
            ["Wren (opens Wren)", "Oskar (opens Oskar)"],
            "AD1-names"
        );
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Wren (opens Wren)"),
            "AD1-tab-1"
        );
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Oskar (opens Oskar)"),
            "AD1-tab-2"
        );
        assert_eq!(take_note_link(&harness.ctx), None, "AD1-no-early-follow");
        harness.key_press(egui::Key::Enter);
        harness.step();
        assert_eq!(
            take_note_link(&harness.ctx).as_deref(),
            Some("Oskar"),
            "AD1-enter"
        );
        // Space follows the focused link as well, without the pointer.
        harness.key_press(egui::Key::Space);
        harness.step();
        assert_eq!(
            take_note_link(&harness.ctx).as_deref(),
            Some("Oskar"),
            "AD1-space"
        );
    }

    #[test]
    fn iter16_a_ad1_the_focus_ring_is_on_the_focused_link_only() {
        let mut harness =
            reader("Ask [Wren](splat-note:Wren) or [Oskar](splat-note:Oskar) about the bells.");
        let ring = |harness: &egui_kittest::Harness<'_>| -> Vec<egui::Rect> {
            harness
                .output()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Rect(rect) if rect.stroke.color == colors().focus => {
                        Some(rect.rect)
                    }
                    _ => None,
                })
                .collect()
        };
        let word = |harness: &egui_kittest::Harness<'_>, words: &str| {
            use egui_kittest::kittest::NodeT;
            harness
                .root()
                .children_recursive()
                .find(|node| {
                    node.accesskit_node()
                        .label()
                        .is_some_and(|name| name.starts_with(words))
                })
                .map(|node| node.rect())
                .expect("the link is in the tree")
        };
        assert!(ring(&harness).is_empty(), "AD1-ring-none");
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        let rings = ring(&harness);
        let oskar = word(&harness, "Oskar");
        let wren = word(&harness, "Wren");
        assert!(
            rings.len() == 1 && rings[0].intersects(oskar) && !rings[0].intersects(wren),
            "AD1-ring-one: rings {rings:?}, Oskar {oskar:?}, Wren {wren:?}"
        );
    }

    #[test]
    fn iter16_a_ad1_edge_paragraphs() {
        // Empty: a paragraph with no link has no link control.
        assert!(
            link_names(&reader("The bells ring at dusk.")).is_empty(),
            "AD1-empty"
        );
        // A missing link is not a stop: Enter cannot follow it.
        let mut missing = reader("Ask [Nobody](splat-missing:Nobody) or [Wren](splat-note:Wren).");
        assert_eq!(
            link_names(&missing),
            ["Wren (opens Wren)"],
            "AD1-missing-names"
        );
        // Long: a link that wraps is one Tab stop and keeps its whole name.
        let mut long = reader(
            "Read [the long account of the night the bells of the old harbour chapel rang without a ringer](splat-note:Bells) and then [Wren](splat-note:Wren).",
        );
        let names = link_names(&long);
        assert_eq!(names.len(), 2, "AD1-long-one-link: {names:?}");
        long.key_press(egui::Key::Tab);
        long.run_ok();
        long.key_press(egui::Key::Tab);
        long.run_ok();
        assert_eq!(
            focused(&long).as_deref(),
            Some("Wren (opens Wren)"),
            "AD1-long-tab"
        );
        missing.key_press(egui::Key::Tab);
        missing.run_ok();
        assert_eq!(
            focused(&missing).as_deref(),
            Some("Wren (opens Wren)"),
            "AD1-missing-skip"
        );
    }

    #[test]
    fn iter16_a_ad1_crowded_short_links_never_share_a_target() {
        // Two short links with no space between them, and one on the next row.
        let part = |left: f32, right: f32, top: f32| LinkPart {
            left,
            right,
            y: top + 16.0,
            rect: egui::Rect::from_min_max(egui::pos2(left, top), egui::pos2(right, top + 22.0)),
            row_start: false,
        };
        let words = vec![
            vec![part(10.0, 24.0, 0.0)],
            vec![part(24.0, 38.0, 0.0)],
            vec![part(12.0, 26.0, 22.0)],
        ];
        let bounds = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(200.0, 44.0));
        let targets = link_targets(&words, bounds);
        let all: Vec<egui::Rect> = targets.iter().flatten().copied().collect();
        for (at, a) in all.iter().enumerate() {
            assert!(bounds.contains_rect(*a), "AD1-inside: {a:?}");
            for b in &all[at + 1..] {
                let shared = a.intersect(*b);
                assert!(
                    shared.width() <= 0.0 || shared.height() <= 0.0,
                    "AD1-crowded: {a:?} and {b:?}"
                );
            }
        }
        // Each target still covers its own words.
        for (target, parts) in all.iter().zip(words.iter().flatten()) {
            assert!(
                target.contains_rect(parts.rect.shrink2(egui::vec2(0.0, 1.0))),
                "AD1-covers"
            );
        }
    }

    #[test]
    fn iter16_a_ad1_focus_survives_a_layout_change() {
        // A pane or a breakpoint can add a widget above the reader. That moves every automatic
        // id below it, but not the link the DM has focused.
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(480.0, 320.0))
            .build_ui_state(
                |ui, wide: &mut bool| {
                    if *wide {
                        ui.label("A pane beside the reader");
                    }
                    show(
                        ui,
                        "Ask [Wren](splat-note:Wren) or [Oskar](splat-note:Oskar) about the bells.",
                    );
                },
                false,
            );
        design::theme::install(&harness.ctx);
        harness.run_ok();
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        *harness.state_mut() = true;
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Oskar (opens Oskar)"),
            "AD1-layout-focus"
        );
        harness.key_press(egui::Key::Enter);
        harness.step();
        assert_eq!(
            take_note_link(&harness.ctx).as_deref(),
            Some("Oskar"),
            "AD1-layout-enter"
        );
    }

    #[test]
    fn iter16_a_ad1_the_same_paragraph_twice_has_two_sets_of_links() {
        let mut harness = reader(
            "Ask [Wren](splat-note:Wren) about the bells.\n\nAsk [Wren](splat-note:Wren) about the bells.",
        );
        assert_eq!(link_names(&harness).len(), 2, "AD1-twice-names");
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        // The second copy has focus, so only one ring shows, on the lower line.
        let rings: Vec<egui::Rect> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Rect(rect) if rect.stroke.color == colors().focus => Some(rect.rect),
                _ => None,
            })
            .collect();
        let links: Vec<egui::Rect> = {
            use egui_kittest::kittest::NodeT;
            harness
                .root()
                .children_recursive()
                .filter(|node| node.accesskit_node().role() == egui::accesskit::Role::Link)
                .map(|node| node.rect())
                .collect()
        };
        assert!(
            rings.len() == 1 && links.len() == 2 && rings[0].center().y > links[0].bottom(),
            "AD1-twice-ring: rings {rings:?}, links {links:?}"
        );
    }

    /// A reader of the given width, with the Reader's prose, or the compact rows when `dense`.
    fn reader_at(
        markdown: &'static str,
        width: f32,
        dense: bool,
    ) -> egui_kittest::Harness<'static> {
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(width, 480.0))
            .build_ui(move |ui| {
                if dense {
                    show_compact(ui, markdown);
                } else {
                    show(ui, markdown);
                }
            });
        design::theme::install(&harness.ctx);
        harness.run_ok();
        harness
    }

    /// Every press target of a link, with its name: a wrapped link has one on each row.
    fn targets_in(harness: &egui_kittest::Harness<'_>) -> Vec<(String, egui::Rect)> {
        use egui_kittest::kittest::NodeT;
        harness
            .root()
            .children_recursive()
            .filter_map(|node| {
                let name = node.accesskit_node().label()?;
                (name.contains(" (opens ") || name.ends_with(" (not found in this campaign)"))
                    .then(|| (name, node.rect()))
            })
            .collect()
    }

    /// The painted paragraphs, each with where it is drawn.
    fn galleys_in(harness: &egui_kittest::Harness<'_>) -> Vec<(egui::Pos2, Arc<egui::Galley>)> {
        harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some((text.pos, text.galley.clone())),
                _ => None,
            })
            .collect()
    }

    /// The painted rows of every paragraph, on the screen.
    fn rows_in(harness: &egui_kittest::Harness<'_>) -> Vec<egui::Rect> {
        galleys_in(harness)
            .iter()
            .flat_map(|(pos, galley)| {
                galley
                    .rows
                    .iter()
                    .map(|row| row.rect().translate(pos.to_vec2()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Where the first glyph of `words` is drawn, and its baseline.
    fn glyph_at(harness: &egui_kittest::Harness<'_>, words: &str) -> Option<(f32, f32)> {
        for (pos, galley) in galleys_in(harness) {
            let Some(start) = galley.job.text.find(words) else {
                continue;
            };
            let mut byte_at = 0;
            for row in &galley.rows {
                for glyph in &row.glyphs {
                    if byte_at == start {
                        return Some((
                            pos.x + row.pos.x + glyph.pos.x,
                            pos.y + row.pos.y + glyph.pos.y,
                        ));
                    }
                    byte_at += glyph.chr.len_utf8();
                }
                if row.ends_with_newline {
                    byte_at += 1;
                }
            }
        }
        None
    }

    const SHORT_LINK_CASES: [&str; 8] = [
        // Side by side, with no space between them.
        "[Jo](splat-note:Jo)[Li](splat-note:Li)",
        "Ask [Jo](splat-note:Jo) or [Li](splat-note:Li) about the ferry.",
        // Last in the paragraph.
        "The ferry hands are [Jo](splat-note:Jo)",
        // Each on its own line, one above the other.
        "[Jo](splat-note:Jo)  \n[Li](splat-note:Li)  \n[Wren](splat-note:Wren)",
        // Crowded: more short links than fit on one line.
        "[A](splat-note:A) [B](splat-note:B) [C](splat-note:C) [D](splat-note:D) [E](splat-note:E) [F](splat-note:F) [G](splat-note:G) [H](splat-note:H) [I](splat-note:I) [J](splat-note:J) [K](splat-note:K) [L](splat-note:L) [M](splat-note:M) [N](splat-note:N) [O](splat-note:O) [P](splat-note:P) [Q](splat-note:Q) [R](splat-note:R) [S](splat-note:S) [T](splat-note:T) [U](splat-note:U) [V](splat-note:V) [W](splat-note:W) [X](splat-note:X) [Y](splat-note:Y) [Z](splat-note:Z)",
        // A missing short link beside a found one.
        "[Jo](splat-missing:Jo)[Li](splat-note:Li) and [Oskar](splat-note:Oskar).",
        // The longest realistic words around short ones, so the paragraph wraps.
        "On the second crossing [Jo](splat-note:Jo) hands the rope to [the ferry master of the inner sea and the outer islands](splat-note:Wren), then [Li](splat-note:Li) ties it off.",
        // Short links in bold.
        "**[Jo](splat-note:Jo)** rows and **[Li](splat-note:Li)** steers.",
    ];

    /// The links each case holds, by their first part.
    const SHORT_LINK_COUNTS: [usize; 8] = [2, 2, 1, 3, 26, 3, 3, 2];

    /// Every case, with a short `[[Name|Li]]` alias as World notes write it.
    fn short_link_cases() -> Vec<(&'static str, usize)> {
        let alias: &'static str = Box::leak(
            crate::world::with_links(
                "[[Lianne Marsh|Li]] and [[Jo]] row the ferry.",
                &std::collections::HashMap::new(),
            )
            .into_boxed_str(),
        );
        assert!(alias.contains("splat-note:"), "AD1-alias-case: {alias}");
        SHORT_LINK_CASES
            .into_iter()
            .zip(SHORT_LINK_COUNTS)
            .chain([(alias, 2)])
            .collect()
    }

    #[test]
    fn iter17_a_ad1_every_link_target_is_24_square_on_its_own_row() {
        for (case, (markdown, links)) in short_link_cases().into_iter().enumerate() {
            // Reader prose (24 px lines), compact prose (22 px) and compact rows (18 px).
            for (width, dense) in [(480.0, false), (360.0, false), (480.0, true)] {
                let harness = reader_at(markdown, width, dense);
                let at = format!("case {case} at {width} (dense {dense})");
                let targets = targets_in(&harness);
                let rows = rows_in(&harness);
                let firsts = {
                    use egui_kittest::kittest::NodeT;
                    let found = harness
                        .root()
                        .children_recursive()
                        .filter(|node| node.accesskit_node().role() == egui::accesskit::Role::Link)
                        .count();
                    found + markdown.matches("splat-missing:").count()
                };
                assert_eq!(firsts, links, "AD1-count: {at}: {targets:?}");
                // A missing link has a hover target but is not a control in the tree.
                let missing = markdown.matches("splat-missing:").count();
                assert!(targets.len() >= links - missing, "AD1-count-parts: {at}");
                if missing > 0 {
                    // Its words still have their own XL, centred on them, before the next
                    // link's target.
                    let (jo_left, jo_right, _) = words_span(&harness, "Jo").expect("Jo is drawn");
                    let jo = (jo_left + jo_right) / 2.0 - design::XL / 2.0;
                    let li = targets
                        .iter()
                        .find(|(name, _)| name == "Li (opens Li)")
                        .map(|(_, rect)| *rect)
                        .expect("Li is a link");
                    assert!(
                        li.left() >= jo + design::XL - 0.5,
                        "AD1-missing-space: {at}: Jo at {jo}, Li target {li:?}, targets {targets:?}"
                    );
                }
                for (name, target) in &targets {
                    assert!(
                        target.width() >= design::XL - 0.01 && target.height() >= design::XL - 0.01,
                        "AD1-size: {at}: {name} {target:?}"
                    );
                    let touched: Vec<&egui::Rect> = rows
                        .iter()
                        .filter(|row| row.y_range().intersection(target.y_range()).span() > 0.5)
                        .collect();
                    assert!(
                        touched.len() == 1
                            && target.top() >= touched[0].top() - 0.5
                            && target.bottom() <= touched[0].bottom() + 0.5,
                        "AD1-own-row: {at}: {name} {target:?} touches {touched:?}"
                    );
                }
                for (index, (a_name, a)) in targets.iter().enumerate() {
                    for (b_name, b) in &targets[index + 1..] {
                        let shared = a.intersect(*b);
                        assert!(
                            shared.width() <= 0.01 || shared.height() <= 0.01,
                            "AD1-overlap: {at}: {a_name} {a:?} and {b_name} {b:?}"
                        );
                    }
                }
            }
        }
    }

    /// Where `words` are drawn on one row: left, right and baseline.
    fn words_span(harness: &egui_kittest::Harness<'_>, words: &str) -> Option<(f32, f32, f32)> {
        for (pos, galley) in galleys_in(harness) {
            let Some(start) = galley.job.text.find(words) else {
                continue;
            };
            let end = start + words.len();
            let mut byte_at = 0;
            let mut span: Option<(f32, f32, f32)> = None;
            for row in &galley.rows {
                for glyph in &row.glyphs {
                    if (start..end).contains(&byte_at) {
                        let x = pos.x + row.pos.x + glyph.pos.x;
                        let y = pos.y + row.pos.y + glyph.pos.y;
                        let right = x + glyph.advance_width;
                        span = Some(span.map_or((x, right, y), |(left, _, y)| (left, right, y)));
                    }
                    byte_at += glyph.chr.len_utf8();
                }
                if row.ends_with_newline {
                    byte_at += 1;
                }
            }
            return span;
        }
        None
    }

    fn target_of(harness: &egui_kittest::Harness<'_>, name: &str) -> egui::Rect {
        targets_in(harness)
            .into_iter()
            .find(|(found, _)| found == name)
            .map(|(_, rect)| rect)
            .unwrap_or_else(|| panic!("{name} is a link"))
    }

    /// The horizontal lines a reader paints: its links' underlines.
    fn underlines_in(harness: &egui_kittest::Harness<'_>) -> Vec<(f32, f32, f32)> {
        harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::LineSegment { points, .. } if points[0].y == points[1].y => Some((
                    points[0].x.min(points[1].x),
                    points[0].x.max(points[1].x),
                    points[0].y,
                )),
                _ => None,
            })
            .collect()
    }

    /// FD5: a short link inside a line has its words in the middle of its `XL` target, and the
    /// words before and after it stay outside that target.
    #[test]
    fn iter18_a_fd5_a_short_link_inside_a_line_centres_its_words() {
        for (width, dense) in [(480.0, false), (360.0, false), (480.0, true)] {
            let harness = reader_at(
                "Ask [Jo](splat-note:Jo) or [Li](splat-note:Li) about the ferry.",
                width,
                dense,
            );
            let at = format!("at {width} (dense {dense})");
            for (words, name, before, after) in [
                ("Jo", "Jo (opens Jo)", "Ask", "or"),
                ("Li", "Li (opens Li)", "or", "about"),
            ] {
                let target = target_of(&harness, name);
                let (left, right, _) = words_span(&harness, words).expect("words are drawn");
                let (_, before, _) = words_span(&harness, before).expect("word before");
                let (after, _, _) = words_span(&harness, after).expect("word after");
                let (space_before, space_after) = (left - target.left(), target.right() - right);
                assert!(
                    target.width() >= design::XL - 0.01
                        && (space_before - space_after).abs() <= 1.0
                        && space_before > 2.0,
                    "FD5-centred: {at}: {words} at {left}..{right}, target {target:?}"
                );
                assert!(
                    before <= target.left() + 0.5 && after >= target.right() - 0.5,
                    "FD5-apart: {at}: {words} target {target:?}, before ends {before}, after starts {after}"
                );
            }
        }
    }

    /// FD5 witness: `[[Jo]][[Li]]` inside a line has even space before and after `Li`, and both
    /// targets are `XL` wide and apart.
    #[test]
    fn iter18_a_fd5_a_side_by_side_pair_inside_a_line_is_even() {
        for (width, dense) in [(480.0, false), (360.0, false), (480.0, true)] {
            let harness = reader_at(
                "Ask [Jo](splat-note:Jo)[Li](splat-note:Li) work the inner ferry.",
                width,
                dense,
            );
            let at = format!("at {width} (dense {dense})");
            let (_, jo_right, _) = words_span(&harness, "Jo").expect("Jo is drawn");
            let (li_left, li_right, _) = words_span(&harness, "Li").expect("Li is drawn");
            let (work, _, _) = words_span(&harness, "work").expect("work is drawn");
            let (before, after) = (li_left - jo_right, work - li_right);
            assert!(
                (before - after).abs() <= 2.0,
                "FD5-even: {at}: before Li {before}, after Li {after}"
            );
            let jo = target_of(&harness, "Jo (opens Jo)");
            let li = target_of(&harness, "Li (opens Li)");
            assert!(
                jo.width() >= design::XL - 0.01
                    && li.width() >= design::XL - 0.01
                    && jo.right() <= li.left() + 0.01,
                "FD5-pair-targets: {at}: {jo:?} {li:?}"
            );
        }
    }

    /// FD5: a short link that starts a line keeps its words at the text edge, with its target
    /// growing right and the next target or word after it. This holds at the start of the
    /// paragraph and where the link wraps to the start of a row.
    #[test]
    fn iter18_a_fd5_a_link_that_starts_a_line_keeps_its_words_at_the_edge() {
        for (width, dense) in [(480.0, false), (360.0, false), (480.0, true)] {
            let harness = reader_at(
                "[Jo](splat-note:Jo)[Li](splat-note:Li) work the inner ferry.",
                width,
                dense,
            );
            let edge = galleys_in(&harness)[0].0.x;
            let (left, _, _) = words_span(&harness, "Jo").expect("Jo is drawn");
            let target = target_of(&harness, "Jo (opens Jo)");
            let li = target_of(&harness, "Li (opens Li)");
            assert!(
                (left - edge).abs() < 0.5
                    && (target.left() - left).abs() < 0.5
                    && li.left() >= target.right() - 0.01,
                "FD5-line-start: at {width} (dense {dense}): edge {edge}, Jo at {left}, target {target:?}, Li {li:?}"
            );
        }
        // A link that wraps to the start of a row: every width that puts `Jo` first on a later row.
        let markdown = "The ferry hands wait on the pier for [Jo](splat-note:Jo) tonight.";
        let mut wrapped = 0;
        for width in (160..=420).step_by(4) {
            let harness = reader_at(markdown, width as f32, false);
            let (pos, galley) = galleys_in(&harness)[0].clone();
            let Some(row) = galley
                .rows
                .iter()
                .skip(1)
                .find(|row| row.glyphs.first().is_some_and(|glyph| glyph.chr == 'J'))
            else {
                continue;
            };
            wrapped += 1;
            let edge = pos.x + row.pos.x;
            let (left, _, _) = words_span(&harness, "Jo").expect("Jo is drawn");
            let (tonight, _, _) = words_span(&harness, "tonight").expect("tonight is drawn");
            let target = target_of(&harness, "Jo (opens Jo)");
            assert!(
                (left - edge).abs() < 0.5
                    && (target.left() - left).abs() < 0.5
                    && tonight >= target.right() - 0.5,
                "FD5-wrapped-start: at {width}: edge {edge}, Jo at {left}, target {target:?}, tonight at {tonight}"
            );
        }
        assert!(
            wrapped > 0,
            "FD5-wrapped-reached: no width put Jo first on a row"
        );
    }

    /// FT1: in compact prose, every row of a paragraph that holds a link has the link pitch, so
    /// the lines are evenly spaced. A compact paragraph without a link keeps its own pitch.
    #[test]
    fn iter18_a_ft1_a_linked_compact_paragraph_has_one_pitch() {
        let linked = "The ferry crosses the inner sea twice each day, and the hands rest on the far pier. On the way back [ask Wren](splat-note:Wren) about the rope, then about the bells and the tide tables.";
        let plain = "The ferry crosses the inner sea twice each day, and the hands rest on the far pier. On the way back ask Wren about the rope, then about the bells and the tide tables.";
        for (width, dense) in [(360.0, false), (480.0, true), (300.0, true)] {
            let at = format!("at {width} (dense {dense})");
            let pitches = |markdown: &'static str| {
                let harness = reader_at(markdown, width, dense);
                let tops: Vec<f32> = rows_in(&harness).iter().map(|row| row.top()).collect();
                tops.windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<f32>>()
            };
            let linked_pitch = pitches(linked);
            assert!(linked_pitch.len() >= 2, "FT1-rows: {at}: {linked_pitch:?}");
            assert!(
                linked_pitch
                    .iter()
                    .all(|pitch| (pitch - design::XL).abs() < 0.5),
                "FT1-even: {at}: {linked_pitch:?}"
            );
            let harness = reader_at(linked, width, dense);
            let (_, _, ask) = words_span(&harness, "ask Wren").expect("the link is drawn");
            assert!(
                !rows_in(&harness)[0].y_range().contains(ask),
                "FT1-link-on-a-later-line: {at}"
            );
            let plain_pitch = pitches(plain);
            assert!(
                plain_pitch.len() >= 2 && plain_pitch.iter().all(|pitch| *pitch < design::XL - 0.5),
                "FT1-plain-keeps-pitch: {at}: {plain_pitch:?}"
            );
        }
    }

    /// FT1: a link's underline is under its words, 2 px below their baseline, in reader and
    /// compact prose, never at the bottom of a taller row. Side-by-side links keep separate
    /// underlines.
    #[test]
    fn iter18_a_ft1_the_underline_sits_under_its_words() {
        for (width, dense) in [(480.0, false), (360.0, false), (480.0, true)] {
            let harness = reader_at(
                "On the way back [ask Wren](splat-note:Wren) and [Jo](splat-note:Jo)[Li](splat-note:Li) about the rope.",
                width,
                dense,
            );
            let at = format!("at {width} (dense {dense})");
            let lines = underlines_in(&harness);
            for words in ["ask Wren", "Jo", "Li"] {
                let (left, right, baseline) = words_span(&harness, words).expect("words are drawn");
                let under: Vec<_> = lines
                    .iter()
                    .filter(|(l, r, _)| *l < right && left < *r)
                    .collect();
                assert!(
                    under.len() == 1
                        && (under[0].0 - left).abs() < 0.5
                        && (under[0].1 - right).abs() < 0.5
                        && (under[0].2 - (baseline + 2.0)).abs() <= 1.0,
                    "FT1-underline: {at}: {words} at {left}..{right}, baseline {baseline}, lines {under:?}"
                );
            }
            // The text itself draws no underline, which would sit at the bottom of the row.
            assert!(
                galleys_in(&harness).iter().all(|(_, galley)| galley
                    .job
                    .sections
                    .iter()
                    .all(|section| section.format.underline == egui::Stroke::NONE)),
                "FT1-one-underline: {at}"
            );
        }
    }

    #[test]
    fn iter17_a_ad1_links_24_px_or_wider_keep_their_layout() {
        let markdown = "Ask [the ferry master](splat-note:Wren) or [the bell keeper](splat-note:Oskar) about the bells, and then ask them again about the rope.";
        for (width, dense) in [(480.0, false), (360.0, false), (480.0, true)] {
            let harness = reader_at(markdown, width, dense);
            let galleys = galleys_in(&harness);
            let job = &galleys[0].1.job;
            assert_eq!(
                job.text,
                "Ask the ferry master or the bell keeper about the bells, and then ask them again about the rope.",
                "AD1-wide-words: at {width}"
            );
            assert!(
                job.sections
                    .iter()
                    .all(|section| section.leading_space == 0.0),
                "AD1-wide-space: at {width}: {:?}",
                job.sections
            );
            if !dense && width > 400.0 {
                // Reader prose rows are already 24 px, so nothing about the layout changes.
                assert!(
                    job.sections
                        .iter()
                        .all(|section| section.format.line_height == Some(Type::Prose.line())),
                    "AD1-wide-line: at {width}"
                );
            }
        }
    }

    #[test]
    fn iter17_a_ad1_a_lifted_row_keeps_its_baselines() {
        // Compact prose is 22 px. The second row holds the link and the code, so it grows.
        let linked = "The ferry crosses the inner sea twice each day. On the way back [ask the ferry master](splat-note:Wren) about `rope` and then the bells.";
        let plain = "The ferry crosses the inner sea twice each day. On the way back ask the ferry master about `rope` and then the bells.";
        let lifted = reader_at(linked, 360.0, false);
        let unlifted = reader_at(plain, 360.0, false);
        let offsets = |harness: &egui_kittest::Harness<'_>| {
            let (_, back) = glyph_at(harness, "back").expect("back is drawn");
            let (_, ask) = glyph_at(harness, "ask the").expect("ask is drawn");
            let (_, rope) = glyph_at(harness, "rope").expect("rope is drawn");
            (back, ask, rope)
        };
        let (back, ask, rope) = offsets(&lifted);
        let (plain_back, _, plain_rope) = offsets(&unlifted);
        let row = rows_in(&lifted)
            .into_iter()
            .find(|row| row.y_range().contains(ask))
            .expect("the link's row");
        assert!(row.height() >= design::XL - 0.01, "AD1-lift: {row:?}");
        assert!(
            (back - ask).abs() < 0.5,
            "AD1-baseline: back {back}, link {ask}"
        );
        assert!(
            ((rope - back) - (plain_rope - plain_back)).abs() < 0.5,
            "AD1-code-baseline: lifted {} plain {}",
            rope - back,
            plain_rope - plain_back
        );
    }

    #[test]
    fn iter17_a_limit1_an_earlier_reader_with_the_same_words_does_not_take_focus() {
        const WORDS: &str =
            "Ask [Wren](splat-note:Wren) or [Oskar](splat-note:Oskar) about the bells.";
        let (a, b) = (uuid::Uuid::from_u128(1), uuid::Uuid::from_u128(2));
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(480.0, 480.0))
            .build_ui_state(
                move |ui, earlier: &mut bool| {
                    if *earlier {
                        ui.ctx()
                            .data_mut(|data| data.insert_temp(egui::Id::new("reader_document"), a));
                        show_reader(ui, WORDS);
                    }
                    ui.ctx()
                        .data_mut(|data| data.insert_temp(egui::Id::new("reader_document"), b));
                    show_reader(ui, WORDS);
                },
                false,
            );
        design::theme::install(&harness.ctx);
        harness.run_ok();
        let focused_rect = |harness: &egui_kittest::Harness<'_, bool>| {
            use egui_kittest::kittest::NodeT;
            harness
                .root()
                .children_recursive()
                .find(|node| node.is_focused())
                .map(|node| node.rect())
        };
        let lowest_oskar = |harness: &egui_kittest::Harness<'_, bool>| {
            use egui_kittest::kittest::NodeT;
            harness
                .root()
                .children_recursive()
                .filter(|node| {
                    node.accesskit_node().role() == egui::accesskit::Role::Link
                        && node.accesskit_node().label().as_deref() == Some("Oskar (opens Oskar)")
                })
                .map(|node| node.rect())
                .max_by(|x, y| x.top().total_cmp(&y.top()))
        };
        let from = |harness: &egui_kittest::Harness<'_, bool>| {
            harness.ctx.data(|data| {
                data.get_temp::<Option<uuid::Uuid>>(egui::Id::new("clicked_reader_document"))
            })
        };
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Oskar (opens Oskar)"),
            "L1-start"
        );
        // Reader A, with the same words, appears above B.
        *harness.state_mut() = true;
        harness.run_ok();
        assert_eq!(
            focused_rect(&harness),
            lowest_oskar(&harness),
            "L1-focus-stays-in-B"
        );
        harness.key_press(egui::Key::Enter);
        harness.step();
        assert_eq!(
            take_note_link(&harness.ctx).as_deref(),
            Some("Oskar"),
            "L1-enter-with-A"
        );
        assert_eq!(from(&harness), Some(Some(b)), "L1-from-B-with-A");
        // Reader A goes away again.
        *harness.state_mut() = false;
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Oskar (opens Oskar)"),
            "L1-after"
        );
        harness.key_press(egui::Key::Enter);
        harness.step();
        assert_eq!(
            take_note_link(&harness.ctx).as_deref(),
            Some("Oskar"),
            "L1-enter"
        );
        assert_eq!(from(&harness), Some(Some(b)), "L1-from-B");
    }

    #[test]
    fn iter17_a_limit3_the_ring_never_covers_a_letter_of_a_line_start_link() {
        let mut harness = reader("[Wren](splat-note:Wren) runs the ferry at dusk.");
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Wren (opens Wren)"),
            "L3-focus"
        );
        let rings: Vec<(egui::Rect, egui::Rect)> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Rect(rect) if rect.stroke.color == colors().focus => {
                    Some((rect.rect, shape.clip_rect))
                }
                _ => None,
            })
            .collect();
        // Every glyph of the link's words, on the screen.
        let (pos, galley) = galleys_in(&harness)
            .into_iter()
            .find(|(_, galley)| galley.job.text == "Wren runs the ferry at dusk.")
            .expect("the paragraph is drawn");
        let row = &galley.rows[0];
        let glyphs: Vec<egui::Rect> = row.glyphs[..4]
            .iter()
            .map(|glyph| {
                egui::Rect::from_x_y_ranges(
                    pos.x + row.pos.x + glyph.pos.x
                        ..=pos.x + row.pos.x + glyph.pos.x + glyph.advance_width,
                    pos.y + row.pos.y..=pos.y + row.pos.y + row.size.y,
                )
            })
            .collect();
        assert_eq!(rings.len(), 1, "L3-one-ring: {rings:?}");
        let (ring, clip) = rings[0];
        let inner = ring.shrink(design::FOCUS_WIDTH);
        assert!(
            glyphs.iter().all(|glyph| {
                inner.left() >= ring.left()
                    && glyph.left() >= inner.left() - 0.01
                    && glyph.right() <= inner.right() + 0.01
            }),
            "L3-ring-off-letters: ring {ring:?}, inner {inner:?}, glyphs {glyphs:?}"
        );
        assert!(
            clip.contains_rect(ring),
            "L3-ring-in-clip: ring {ring:?}, clip {clip:?}"
        );
    }

    #[test]
    fn iter17_a_limit1_a_reader_without_a_document_keeps_its_ids_beside_one_with_a_document() {
        const SESSION: &str =
            "Ask [Wren](splat-note:Wren) or [Oskar](splat-note:Oskar) about the bells.";
        let document = uuid::Uuid::from_u128(7);
        // Lookup, a side panel, draws before the page. Its reader shows a document.
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(900.0, 480.0))
            .build_state(
                move |ctx, lookup: &mut bool| {
                    if *lookup {
                        egui::SidePanel::right("lookup").show(ctx, |ui| {
                            ctx.data_mut(|data| {
                                data.insert_temp(egui::Id::new("reader_document"), document)
                            });
                            show_reader(ui, "Wren runs the ferry at dusk.");
                        });
                    }
                    egui::CentralPanel::default().show(ctx, |ui| show_reader(ui, SESSION));
                },
                false,
            );
        design::theme::install(&harness.ctx);
        harness.run_ok();
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        harness.key_press(egui::Key::Tab);
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Oskar (opens Oskar)"),
            "L1-panel-start"
        );
        *harness.state_mut() = true;
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Oskar (opens Oskar)"),
            "L1-panel-focus-stays"
        );
        harness.key_press(egui::Key::Enter);
        harness.step();
        assert_eq!(
            take_note_link(&harness.ctx).as_deref(),
            Some("Oskar"),
            "L1-panel-enter"
        );
        let from = harness.ctx.data(|data| {
            data.get_temp::<Option<uuid::Uuid>>(egui::Id::new("clicked_reader_document"))
        });
        assert_eq!(from, Some(None), "L1-panel-from-none");
        // A document set once and then not again does not stay with a later pass.
        *harness.state_mut() = false;
        harness.run_ok();
        assert_eq!(
            focused(&harness).as_deref(),
            Some("Oskar (opens Oskar)"),
            "L1-panel-after"
        );
    }
}
