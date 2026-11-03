//! Read-only Markdown presentation. Content cannot execute UI actions, but it can say which note
//! a link names, and a roll table can ask for a roll. Whoever drew it decides what happens next.
use crate::design::{self, Type, colors};
use eframe::egui::{self, text::LayoutJob};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::borrow::Cow;

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

/// Plain bold words pressed in the Reader, for Lookup to open or search.
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
    let mut quotes = 0_usize;
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
    let from = ui
        .ctx()
        .data(|data| data.get_temp::<uuid::Uuid>(egui::Id::new("reader_document")));
    let linked = index.map_or_else(
        || markdown.to_owned(),
        |index| index.reader_markdown(markdown, from),
    );
    let source = local_figure_links(&linked, |url| crate::art::link_art(&ctx, url).is_some());
    for (event, range) in Parser::new_ext(
        &source,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
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
                    link = Some((job.text.len(), LinkTarget::Missing));
                    continue;
                }
                if let Some(note) = dest_url.strip_prefix(NOTE_LINK) {
                    link = Some((job.text.len(), LinkTarget::Note(note.to_owned())));
                    continue;
                }
                if dest_url.starts_with("https://") || dest_url.starts_with("http://") {
                    link = Some((job.text.len(), LinkTarget::Url(dest_url.into_string())));
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
                    if !words.is_empty()
                        && !words.ends_with(['.', ':'])
                        && !(job.text[..start].trim().is_empty()
                            && source[range.end..].trim_start().starts_with('|'))
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
            Event::Start(Tag::BlockQuote(_)) => quotes += 1,
            Event::End(TagEnd::BlockQuote(_)) => quotes = quotes.saturating_sub(1),
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
                } else if (italic || quotes > 0) && !dense {
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
    let top = ui.cursor().top();
    let left = ui.cursor().left();
    let outer_clip = ui.clip_rect();
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
                table_part(ui, &cells, widths, &heights, 0..1, matched);
            },
        );
        if widths.len() > 1 {
            let moving_width = width - widths[0] - TABLE_GAP;
            let mut show = |ui: &mut egui::Ui| {
                ui.set_width(moving_width);
                moving_origin = ui.cursor().min;
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    table_part(ui, &cells, widths, &heights, 1..widths.len(), matched);
                });
            };
            if scrolls {
                ui.style_mut().always_scroll_the_only_direction = false;
                let bars = if top < outer_clip.bottom() {
                    egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
                } else {
                    egui::scroll_area::ScrollBarVisibility::AlwaysHidden
                };
                let document = ui
                    .ctx()
                    .data(|data| data.get_temp::<uuid::Uuid>(egui::Id::new("reader_document")));
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
    painter.vline(
        left + widths[0],
        top..=top + height,
        egui::Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
    );
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
) {
    ui.spacing_mut().item_spacing.y = 0.0;
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
            ui.painter()
                .rect_filled(fill, egui::CornerRadius::ZERO, colors().selected);
        }
        let mut x = rect.left();
        for column in columns.clone() {
            let cell = egui::Rect::from_min_size(
                egui::pos2(x, rect.top() + design::XS),
                cells[column].size(),
            );
            ui.place(cell, egui::Label::new(cells[column].clone()));
            x += widths[column] + TABLE_GAP;
        }
        ui.painter().hline(
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

/// What the words waiting to be drawn link to.
#[derive(Default)]
struct Targets {
    links: Vec<(std::ops::Range<usize>, LinkTarget)>,
}

enum LinkTarget {
    Missing,
    Note(String),
    Lookup(String),
    Url(String),
}

/// A paragraph stays one paragraph, and only its links can be pressed.
fn draw_with_links(
    ui: &mut egui::Ui,
    mut job: LayoutJob,
    links: Vec<(std::ops::Range<usize>, LinkTarget)>,
) {
    // A one-line linked paragraph still needs a full control target.
    job.first_row_min_height = job
        .first_row_min_height
        .max(design::Density::Read.control());
    let text = job.text.clone();
    let (position, galley, response) = egui::Label::new(job)
        .wrap()
        .sense(egui::Sense::click())
        .layout_in_ui(ui);
    ui.painter().galley(position, galley.clone(), colors().text);
    for (range, destination) in &links {
        if matches!(destination, LinkTarget::Missing) {
            let mut byte_at = 0;
            for row in &galley.rows {
                let mut x = position.x + row.pos.x;
                for glyph in &row.glyphs {
                    let length = glyph.chr.len_utf8();
                    if range.contains(&byte_at) {
                        let y = position.y + row.pos.y + row.size.y;
                        let mut dot = x;
                        while dot < x + glyph.advance_width {
                            ui.painter().line_segment(
                                [
                                    egui::pos2(dot, y),
                                    egui::pos2(
                                        (dot + design::HAIRLINE_WIDTH).min(x + glyph.advance_width),
                                        y,
                                    ),
                                ],
                                egui::Stroke::new(design::HAIRLINE_WIDTH, colors().muted),
                            );
                            dot += design::XS;
                        }
                    }
                    byte_at += length;
                    x += glyph.advance_width;
                }
                if row.ends_with_newline {
                    byte_at += 1;
                }
            }
        }
    }
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, true, text.as_str()));
    let Some(pointer) = response.hover_pos() else {
        return;
    };
    let at = galley.cursor_from_pos(pointer - position).index;
    let byte = text
        .char_indices()
        .nth(at)
        .map_or(text.len(), |(byte, _)| byte);
    if let Some((_, destination)) = links.iter().find(|(range, _)| range.contains(&byte)) {
        if matches!(destination, LinkTarget::Missing) {
            response.on_hover_text("Not found in this campaign");
            return;
        }
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        if let LinkTarget::Url(url) = destination {
            response.clone().on_hover_text(url);
        }
        if response.clicked() {
            match destination {
                LinkTarget::Missing => {}
                LinkTarget::Note(note) => ui.ctx().data_mut(|data| {
                    let from = data.get_temp::<uuid::Uuid>(egui::Id::new("reader_document"));
                    data.insert_temp(egui::Id::new("clicked_reader_document"), from);
                    data.insert_temp(clicked_id(), note.clone());
                }),
                LinkTarget::Lookup(words) => ui.ctx().data_mut(|data| {
                    data.insert_temp(egui::Id::new("markdown_clicked_lookup"), words.clone());
                }),
                LinkTarget::Url(url) => ui.ctx().open_url(egui::OpenUrl::new_tab(url)),
            }
        }
    }
}

fn flush(ui: &mut egui::Ui, job: &mut LayoutJob, target: &mut Targets) {
    if job.is_empty() {
        return;
    }
    let links = std::mem::take(&mut target.links);
    if !links.is_empty() {
        draw_with_links(ui, std::mem::take(job), links);
        ui.add_space(design::S);
        return;
    }
    ui.add(egui::Label::new(std::mem::take(job)).wrap());
    ui.add_space(design::S);
}

#[cfg(test)]
mod tests {
    use super::*;

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

    /// Click the first letter of these words in the actual laid-out paragraph.
    fn click_words(markdown: &str, words: &str) -> (Option<String>, Option<String>, Vec<String>) {
        let context = egui::Context::default();
        design::theme::install(&context);
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
        (take_note_link(&context), take_lookup_link(&context), urls)
    }

    #[test]
    fn plain_bold_names_open_lookup_and_plain_words_do_not() {
        let markdown = "- **Docket** speaks to **Advocate Ulul**.\n";
        assert_eq!(
            click_words(markdown, "Docket"),
            (None, Some("Docket".into()), vec![])
        );
        assert_eq!(
            click_words(markdown, "Advocate Ulul"),
            (None, Some("Advocate Ulul".into()), vec![])
        );
        assert_eq!(click_words(markdown, "speaks"), (None, None, vec![]));
        assert_eq!(
            click_words("__Véldo & Sons__ arrive.", "Sons"),
            (None, Some("Véldo & Sons".into()), vec![])
        );
        assert_eq!(
            click_words("**Docket __the clerk__ arrives**.", "arrives"),
            (None, Some("Docket the clerk arrives".into()), vec![])
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
        assert_eq!(
            click_words(markdown, "Docket"),
            (None, Some("Docket".into()), vec![])
        );
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
    fn bold_lookup_links_keep_their_weight_and_only_links_get_link_color() {
        let galleys = painted("**Docket** speaks. **Read [the rules](https://example.com)**.");
        let name = format_of(&galleys, "Docket");
        assert_eq!(
            name.font_id.family,
            egui::FontFamily::Name(design::FAMILY_SERIF_SEMIBOLD.into())
        );
        assert_eq!(name.color, colors().accent);
        assert_eq!(format_of(&galleys, "Read ").color, colors().text);
        assert_eq!(format_of(&galleys, "the rules").color, colors().accent);
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
}
