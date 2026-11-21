//! A fixed DM screen using the open session's plan and draft.
use super::{Book, Draft, Note, PLAN_WIDTH, Plan, Sessions, compact_plan_for_reading, page_column};
use crate::{
    design::{
        self, Action, Density, Heading, Type, colors,
        layout::ScrollColumn,
        memo::{Memo, Revision},
    },
    markdown_view,
    sessions::{outline, without_empty_sections, without_own_heading},
};
use eframe::egui::{self, Key, Modifiers};
use pulldown_cmark::HeadingLevel::H2;
use std::{borrow::Cow, ops::Range, sync::Arc};

const HAPPENED: &str = "What happened";
const THREADS: &str = "Open threads";
const LIVE_NOTES: design::FieldName = crate::field_id!("dm_screen_what_happened");
const PLAY_PLAN_WIDTH: f32 = PLAN_WIDTH + 4.0 * design::L;
const LIVE_NOTES_WIDTH: f32 = 360.0;
const REFERENCE_WIDTH: f32 = 300.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Play,
    Notes,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Widget {
    #[default]
    Plan,
    Happened,
    Threads,
    Note,
}

impl Widget {
    fn title(self) -> &'static str {
        match self {
            Self::Plan => "Plan for tonight",
            Self::Happened => HAPPENED,
            Self::Threads => THREADS,
            Self::Note => "Note",
        }
    }
}

#[derive(Default)]
pub(super) struct Preview {
    screen: Option<Screen>,
    widget: Widget,
    projection: Memo<Revision, Arc<Sections>>,
}

impl Preview {
    pub(super) fn is_open(&self) -> bool {
        self.screen.is_some()
    }

    pub(super) fn is_play(&self) -> bool {
        self.screen == Some(Screen::Play)
    }

    pub(super) fn show_notes(&mut self) {
        if self.is_open() {
            self.screen = Some(Screen::Notes);
        }
    }

    pub(super) fn toggle_button(&mut self, ui: &mut egui::Ui) {
        let word = if self.is_open() {
            "Close DM screen"
        } else {
            "DM screen"
        };
        if Action::quiet(word).show(ui).clicked() {
            if self.is_open() {
                *self = Self::default();
            } else {
                self.screen = Some(Screen::Play);
            }
        }
    }

    pub(super) fn page_keys(&mut self, ui: &egui::Ui) {
        if ui.input_mut(|input| input.consume_key(Modifiers::CTRL, Key::PageUp)) {
            self.screen = Some(Screen::Play);
        }
        if ui.input_mut(|input| input.consume_key(Modifiers::CTRL, Key::PageDown)) {
            self.screen = Some(Screen::Notes);
        }
    }

    pub(super) fn pager(&mut self, ui: &mut egui::Ui) {
        let pages = [(Some(Screen::Play), "Play"), (Some(Screen::Notes), "Notes")];
        let width = pages
            .iter()
            .map(|&(_, word)| Action::quiet(word).natural_width(ui))
            .sum();
        ui.vertical_centered(|ui| {
            ui.allocate_ui(egui::vec2(width, Density::Read.control()), |ui| {
                design::segmented(ui, &mut self.screen, &pages);
            });
        });
    }
}

pub(super) fn pager_height(ui: &egui::Ui) -> f32 {
    Density::Read.control() + ui.spacing().item_spacing.y
}

/// Positions in the draft, and the projected words for its read-only widgets.
struct Sections {
    happened: Option<Range<usize>>,
    threads: String,
    remainder: String,
}

impl Sections {
    fn read(body: &str, note: &Note, model: &super::plan::PlanModel, plan: &Plan<'_>) -> Self {
        let outline = outline(body);
        let mut happened = None;
        let mut threads = None;
        let mut remainder = body.to_owned();
        // Work backwards so each removal keeps the preceding ranges valid. A repeated heading
        // stays in the Note widget while the first occurrence supplies its own tile.
        let mut removed = Vec::new();
        for (level, heading) in &outline.headings {
            if *level != H2 {
                continue;
            }
            let target = match body[heading.clone()].trim_end().strip_prefix("## ") {
                Some(HAPPENED) => &mut happened,
                Some(THREADS) => &mut threads,
                _ => continue,
            };
            if target.is_none() {
                let end = outline.section_end(heading.end, body);
                let section = &body[heading.end..end];
                // Leave the line break before the next heading outside the editor. The rest
                // stays editable, including blank lines the DM has just typed.
                let words = if end < body.len() {
                    section
                        .strip_suffix("\r\n")
                        .or_else(|| section.strip_suffix('\n'))
                        .unwrap_or(section)
                } else {
                    section
                };
                *target = Some(heading.end..heading.end + words.len());
                removed.push(heading.start..end);
            }
        }
        for section in removed.into_iter().rev() {
            remainder.replace_range(section, "");
        }
        let remainder = compact_plan_for_reading(
            &without_own_heading(&remainder, note).replace("\r\n", "\n"),
            plan.fights,
            plan.groups,
            &model.scene_lines(),
            true,
        );
        Self {
            happened,
            threads: threads.map_or_else(String::new, |range| body[range].trim().to_owned()),
            remainder: without_empty_sections(&remainder),
        }
    }
}

impl Sessions {
    pub(super) fn dm_play(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        plan: &Plan<'_>,
        note: &Note,
    ) {
        let model = self.plan_model(super::reader_index(ui).as_deref(), plan);
        let Some(draft) = &self.draft else { return };
        let sections = Arc::clone(
            self.dm_screen
                .projection
                .get_or(self.plan_model.revision(), || {
                    Arc::new(Sections::read(&draft.body, note, &model, plan))
                }),
        );
        let save = ui.input_mut(|input| input.consume_key(Modifiers::COMMAND, Key::S));
        let width = ui.available_width();
        let mut widget = self.dm_screen.widget;
        let mut draw = |ui: &mut egui::Ui, widget, heading| {
            tile(ui, widget, heading, |ui| {
                self.dm_widget(ui, widget, book, plan, note, &sections)
            });
        };
        if width >= PLAY_PLAN_WIDTH + LIVE_NOTES_WIDTH + REFERENCE_WIDTH + 2.0 * design::S {
            let height = ui.available_height();
            let tile_width = width - 2.0 * design::S;
            let notes_width = (tile_width - PLAY_PLAN_WIDTH - REFERENCE_WIDTH)
                .min(design::READER_MEASURE + super::NOTE_FRAME_EDGES);
            let reference_width = tile_width - PLAY_PLAN_WIDTH - notes_width;
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = design::S;
                for (width, widgets) in [
                    (PLAY_PLAN_WIDTH, &[Widget::Plan][..]),
                    (notes_width, &[Widget::Happened]),
                    (reference_width, &[Widget::Threads, Widget::Note]),
                ] {
                    page_column(ui, width, height, |ui| {
                        for &widget in widgets {
                            draw(ui, widget, true);
                        }
                    });
                }
            });
        } else {
            design::segmented(
                ui,
                &mut widget,
                &[
                    (Widget::Plan, "Plan"),
                    (Widget::Happened, HAPPENED),
                    (Widget::Threads, THREADS),
                    (Widget::Note, "Note"),
                ],
            );
            draw(ui, widget, false);
            self.dm_screen.widget = widget;
        }
        if save && let Some(draft) = &mut self.draft {
            draft.save();
        }
        if let Some(target) = markdown_view::take_note_link(ui.ctx()) {
            self.open_note_link(target);
        }
    }

    fn dm_widget(
        &mut self,
        ui: &mut egui::Ui,
        widget: Widget,
        book: &mut Book,
        plan: &Plan<'_>,
        note: &Note,
        sections: &Sections,
    ) {
        match widget {
            Widget::Plan => self.plan_pane(ui, book, plan, None),
            Widget::Happened => {
                if let Some(range) = &sections.happened {
                    live_notes(
                        ui,
                        note,
                        self.draft.as_mut().expect("the open draft"),
                        range,
                    );
                }
            }
            Widget::Threads | Widget::Note => {
                let text = if widget == Widget::Threads {
                    &sections.threads
                } else {
                    &sections.remainder
                };
                if !text.trim().is_empty() {
                    let mut scroll = ScrollColumn::new((note.id, widget));
                    if widget == Widget::Threads {
                        scroll = scroll.max_height(ui.available_height() / 3.0).fit_height();
                    }
                    scroll.show(ui, |ui| {
                        ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
                        markdown_view::show_reader(ui, text);
                    });
                }
            }
        }
    }
}

fn tile(ui: &mut egui::Ui, widget: Widget, heading: bool, body: impl FnOnce(&mut egui::Ui)) {
    ui.scope_builder(
        egui::UiBuilder::new().id(("session_dm_widget", widget)),
        |ui| {
            egui::Frame::new()
                .stroke(egui::Stroke::new(
                    design::HAIRLINE_WIDTH,
                    colors().field_border,
                ))
                .corner_radius(design::RADIUS_CONTROL)
                .inner_margin(design::S)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    if heading {
                        design::heading(ui, Heading::Section(widget.title()));
                    }
                    body(ui);
                });
        },
    );
}

fn live_notes(ui: &mut egui::Ui, note: &Note, draft: &mut Draft, range: &Range<usize>) {
    let mut words = Cow::Borrowed(&draft.body[range.clone()]);
    ScrollColumn::new((note.id, Widget::Happened)).show(ui, |ui| {
        let font = Type::Body.font(ui);
        let width = ui.available_width().min(design::READER_MEASURE);
        let output = LIVE_NOTES.of(note.id).editor(ui, &mut words, |edit| {
            edit.hint_text("Capture what happens at the table…")
                .font(font)
                .desired_width(width)
                .desired_rows(8)
        });
        design::named(ui, &output.response, HAPPENED);
    });
    if words.as_ref() != &draft.body[range.clone()] {
        let replacement = words.into_owned();
        // For an empty heading, keep what is typed on its own line, also when the next
        // heading immediately follows it or the file ends at the heading.
        let before = &draft.body[..range.start];
        let after = &draft.body[range.end..];
        let newline = if before.ends_with("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        let leading = if before.ends_with('\n') { "" } else { newline };
        let trailing = if after.is_empty() || after.starts_with(['\r', '\n']) {
            ""
        } else {
            newline
        };
        draft
            .body
            .replace_range(range.clone(), &format!("{leading}{replacement}{trailing}"));
        draft.edited();
    }
}
