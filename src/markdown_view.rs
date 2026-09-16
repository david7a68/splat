//! Read-only Markdown presentation. Content cannot execute UI actions.
use eframe::egui::{self, text::LayoutJob};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

pub fn show(ui: &mut egui::Ui, markdown: &str) {
    let mut job = LayoutJob::default();
    let mut heading = None;
    let mut inventory_table = false;
    let mut target: Option<String> = None;
    let mut in_link = false;
    let mut code = false;
    let mut strong = false;
    let mut italic = false;
    let mut lists: Vec<Option<u64>> = Vec::new();
    let mut table: Option<Vec<Vec<String>>> = None;
    for event in Parser::new_ext(
        markdown,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    ) {
        match event {
            Event::Html(ref text) if text.trim().starts_with("<!-- splat:inventory") => {
                inventory_table = true;
            }
            Event::Start(Tag::Link { dest_url, .. }) if table.is_none() => {
                if target.is_some() {
                    flush(ui, &mut job, &mut target);
                }
                if dest_url.starts_with("https://") || dest_url.starts_with("http://") {
                    target = Some(dest_url.into_string());
                    in_link = true;
                }
            }
            Event::End(TagEnd::Link) => in_link = false,
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
                    let column_width = ((ui.available_width() - 16.0 * columns.len() as f32)
                        / columns.len().max(1) as f32)
                        .clamp(90.0, 180.0);
                    ui.push_id(ui.next_auto_id(), |ui| {
                        egui::Grid::new("markdown_table")
                            .num_columns(columns.len())
                            .min_col_width(column_width)
                            .max_col_width(column_width)
                            .spacing([16.0, 7.0])
                            .striped(true)
                            .show(ui, |ui| {
                                for (i, row) in rows.iter().enumerate() {
                                    for &column in &columns {
                                        let cell =
                                            row.get(column).map(String::as_str).unwrap_or("");
                                        let cell = if compact {
                                            match cell {
                                                "Link" => "Item",
                                                "true" => "Yes",
                                                "false" => "—",
                                                other => other,
                                            }
                                        } else {
                                            cell
                                        };
                                        let text = egui::RichText::new(cell);
                                        ui.add(
                                            egui::Label::new(if i == 0 {
                                                text.strong()
                                            } else {
                                                text
                                            })
                                            .wrap(),
                                        );
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                    ui.add_space(8.0);
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
                ui.add_space(8.0);
                heading = Some(level);
            }
            Event::End(TagEnd::Heading(_)) => {
                flush(ui, &mut job, &mut target);
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
            Event::Start(Tag::Strong) => strong = true,
            Event::End(TagEnd::Strong) => strong = false,
            Event::Start(Tag::Emphasis) => italic = true,
            Event::End(TagEnd::Emphasis) => italic = false,
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
                        color: ui.visuals().text_color(),
                        ..Default::default()
                    },
                );
            }
            Event::End(TagEnd::Paragraph | TagEnd::Item) => flush(ui, &mut job, &mut target),
            Event::HardBreak if table.is_none() => {
                job.append("\n", 0.0, egui::TextFormat::default())
            }
            Event::SoftBreak if table.is_none() => job.append(
                if code { "\n" } else { " " },
                0.0,
                egui::TextFormat::default(),
            ),
            Event::Text(text) | Event::Code(text) => {
                let font_id = if code {
                    egui::TextStyle::Monospace
                } else if heading.is_some() {
                    egui::TextStyle::Heading
                } else if strong {
                    egui::TextStyle::Button
                } else {
                    egui::TextStyle::Body
                }
                .resolve(ui.style());
                let font_id = if let Some(level) = heading {
                    egui::FontId::proportional(match level {
                        pulldown_cmark::HeadingLevel::H1 => 26.0,
                        pulldown_cmark::HeadingLevel::H2 => 21.0,
                        _ => 17.0,
                    })
                } else {
                    font_id
                };
                job.append(
                    &text,
                    0.0,
                    egui::TextFormat {
                        font_id,
                        color: if in_link {
                            ui.visuals().hyperlink_color
                        } else if strong {
                            ui.visuals().strong_text_color()
                        } else {
                            ui.visuals().text_color()
                        },
                        italics: italic,
                        ..Default::default()
                    },
                );
            }
            Event::Rule => {
                flush(ui, &mut job, &mut target);
                ui.separator();
            }
            _ => {}
        }
    }
    flush(ui, &mut job, &mut target);
}

fn flush(ui: &mut egui::Ui, job: &mut LayoutJob, target: &mut Option<String>) {
    if job.is_empty() {
        return;
    }
    let label = egui::Label::new(std::mem::take(job)).wrap();
    if let Some(url) = target.take() {
        if ui
            .add(label.sense(egui::Sense::click()))
            .on_hover_text(&url)
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
        {
            ui.ctx().open_url(egui::OpenUrl::new_tab(url));
        }
    } else {
        ui.add(label);
    }
    ui.add_space(5.0);
}

#[cfg(test)]
mod tests {
    use super::*;
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
