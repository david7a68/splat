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
    let source = local_figure_links(markdown, |url| crate::art::link_art(&ctx, url).is_some());
    for event in Parser::new_ext(
        &source,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    ) {
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
                if let Some(rows) = table.take() {
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
                    let widths = column_widths(ui, &rows, &columns, &display);
                    // The rules end with the table, not at the measure.
                    let table_width = widths.iter().sum::<f32>()
                        + TABLE_GAP * widths.len().saturating_sub(1) as f32;
                    ui.scope(|ui| {
                        ui.set_max_width(ui.available_width().min(table_width));
                        for (i, row) in rows.iter().enumerate() {
                            design::table_row(ui, &widths, TABLE_GAP, |ui, position| {
                                let cell = display(
                                    row.get(columns[position]).map(String::as_str).unwrap_or(""),
                                );
                                let role = if i == 0 { Type::StatLabel } else { Type::Stat };
                                ui.add(
                                    egui::Label::new(role.rich(ui, cell).color(colors().text))
                                        .wrap(),
                                );
                            });
                        }
                    });
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
                    if !words.is_empty() {
                        for section in &mut job.sections {
                            if section.byte_range.start >= start {
                                section.format.color = colors().accent;
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
                let color = if link.is_some() {
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

/// Fit columns to their text. Never narrower than a column's longest word, while
/// there is room, so "Constitution" does not break in a side panel.
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
        })
    };
    let mut natural = vec![1.0_f32; columns.len()];
    let mut floor = vec![1.0_f32; columns.len()];
    for row in rows {
        for (position, &column) in columns.iter().enumerate() {
            let cell = display(row.get(column).map(String::as_str).unwrap_or(""));
            natural[position] = natural[position].max(measure(&cell));
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
        return floor.iter().map(|w| w * available / floors).collect();
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
    Note(String),
    Lookup(String),
    Url(String),
}

/// A paragraph stays one paragraph, and only its links can be pressed.
fn draw_with_links(
    ui: &mut egui::Ui,
    job: LayoutJob,
    links: Vec<(std::ops::Range<usize>, LinkTarget)>,
) {
    let text = job.text.clone();
    let (position, galley, response) = egui::Label::new(job)
        .wrap()
        .sense(egui::Sense::click())
        .layout_in_ui(ui);
    ui.painter().galley(position, galley.clone(), colors().text);
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
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        if let LinkTarget::Url(url) = destination {
            response.clone().on_hover_text(url);
        }
        if response.clicked() {
            match destination {
                LinkTarget::Note(note) => ui
                    .ctx()
                    .data_mut(|data| data.insert_temp(clicked_id(), note.clone())),
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
