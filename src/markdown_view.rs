//! Read-only Markdown presentation. Content cannot execute UI actions.
use eframe::egui::{self, text::LayoutJob};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

pub fn show(ui: &mut egui::Ui, markdown: &str) {
    let mut job = LayoutJob::default();
    let mut heading = false;
    let mut code = false;
    let mut strong = false;
    let mut italic = false;
    let mut lists: Vec<Option<u64>> = Vec::new();
    let mut table: Option<Vec<Vec<String>>> = None;
    let flush = |ui: &mut egui::Ui, job: &mut LayoutJob| {
        if !job.is_empty() {
            ui.add(egui::Label::new(std::mem::take(job)).wrap());
            ui.add_space(5.0);
        }
    };
    for event in Parser::new_ext(
        markdown,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    ) {
        match event {
            Event::Start(Tag::Table(_)) => {
                flush(ui, &mut job);
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
                    ui.push_id(ui.next_auto_id(), |ui| {
                        egui::Grid::new("markdown_table")
                            .striped(true)
                            .show(ui, |ui| {
                                for (i, row) in rows.iter().enumerate() {
                                    for cell in row {
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
            Event::Start(Tag::Heading { .. }) => {
                flush(ui, &mut job);
                heading = true;
            }
            Event::End(TagEnd::Heading(_)) => {
                flush(ui, &mut job);
                heading = false;
            }
            Event::Start(Tag::CodeBlock(_)) => {
                flush(ui, &mut job);
                code = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                flush(ui, &mut job);
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
                flush(ui, &mut job);
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
            Event::End(TagEnd::Paragraph | TagEnd::Item) => flush(ui, &mut job),
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
                } else if heading {
                    egui::TextStyle::Heading
                } else if strong {
                    egui::TextStyle::Button
                } else {
                    egui::TextStyle::Body
                }
                .resolve(ui.style());
                job.append(
                    &text,
                    0.0,
                    egui::TextFormat {
                        font_id,
                        color: if strong {
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
                flush(ui, &mut job);
                ui.separator();
            }
            _ => {}
        }
    }
    flush(ui, &mut job);
}
