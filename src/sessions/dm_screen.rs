//! A fixed DM screen using the open session's plan and draft.
use super::{Book, Draft, Note, PLAN_WIDTH, Plan, Sessions, compact_plan_for_reading, page_column};
use crate::{
    design::{
        self, Action, Density, Heading, Type, colors,
        layout::ScrollColumn,
        memo::{Memo, Revision},
    },
    markdown_view,
    sessions::{WHAT_HAPPENED, section, without_empty_sections, without_own_heading},
};
use eframe::egui::{self, Key, Modifiers};
use std::{borrow::Cow, ops::Range, sync::Arc};

const THREADS: &str = "Open threads";
const REST: &str = "Rest of note";
const LIVE_NOTES: design::FieldName = crate::field_id!("dm_screen_what_happened");
const PLAY_PLAN_WIDTH: f32 = PLAN_WIDTH + 4.0 * design::L;
const LIVE_NOTES_WIDTH: f32 = 360.0;
const REFERENCE_WIDTH: f32 = 300.0;
/// A column that holds a full line of reading in its tile.
const NOTE_MEASURE: f32 = design::READER_MEASURE + super::NOTE_FRAME_EDGES;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Play,
    Notes,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Tile {
    #[default]
    Plan,
    Happened,
    Threads,
    Rest,
}

impl Tile {
    fn title(self) -> &'static str {
        match self {
            Self::Plan => "Plan for tonight",
            Self::Happened => WHAT_HAPPENED,
            Self::Threads => THREADS,
            Self::Rest => REST,
        }
    }
}

#[derive(Default)]
pub(super) struct Preview {
    screen: Option<Screen>,
    tile: Tile,
    projection: Memo<Revision, Arc<Sections>>,
    /// True only while Play draws its tiles, so that the plan's rows take the tile's layout
    /// there and nowhere else (Play gives way to the full view for a conflict or a group).
    in_tiles: bool,
}

impl Preview {
    pub(super) fn is_open(&self) -> bool {
        self.screen.is_some()
    }

    pub(super) fn is_play(&self) -> bool {
        self.screen == Some(Screen::Play)
    }

    pub(super) fn in_tiles(&self) -> bool {
        self.in_tiles
    }

    pub(super) fn open_play(&mut self) {
        self.screen = Some(Screen::Play);
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
                self.open_play();
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

/// What the What happened tile edits: the section's words, without the line break that ends
/// the note. `None` when the note has no `What happened` section. One reading
/// (`sessions::section`) with the app's other writers of the section.
fn happened_words(body: &str) -> Option<Range<usize>> {
    let found = section(body, WHAT_HAPPENED)?;
    let mut end = found.end;
    if end == body.len() && end > found.words {
        let words = &body[found.words..end];
        end -= words
            .strip_suffix("\r\n")
            .or_else(|| words.strip_suffix('\n'))
            .map_or(0, |kept| words.len() - kept.len());
    }
    Some(found.words..end)
}

/// Positions in the draft, and the projected words for the read-only tiles.
struct Sections {
    /// The draft's revision that these were read from.
    revision: u64,
    happened: Option<Range<usize>>,
    threads: String,
    remainder: String,
}

impl Sections {
    fn read(
        body: &str,
        revision: u64,
        note: &Note,
        model: &super::plan::PlanModel,
        plan: &Plan<'_>,
    ) -> Self {
        let happened = section(body, WHAT_HAPPENED);
        let threads = section(body, THREADS);
        // The sections with a tile of their own leave the rest of the note, from the end so
        // that each cut keeps the earlier offsets.
        let mut cuts: Vec<Range<usize>> = [happened, threads]
            .into_iter()
            .flatten()
            .map(|found| found.heading..found.next)
            .collect();
        cuts.sort_by_key(|cut| cut.start);
        let mut remainder = body.to_owned();
        for cut in cuts.into_iter().rev() {
            remainder.replace_range(cut, "");
        }
        let remainder = compact_plan_for_reading(
            &without_own_heading(&remainder, note).replace("\r\n", "\n"),
            plan.fights,
            plan.groups,
            &model.scene_lines(),
            true,
        );
        Self {
            revision,
            happened: happened_words(body),
            threads: threads.map_or_else(String::new, |found| {
                body[found.words..found.end].trim().to_owned()
            }),
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
        let revision = draft.revision();
        let sections = Arc::clone(
            self.dm_screen
                .projection
                .get_or(self.plan_model.revision(), || {
                    Arc::new(Sections::read(&draft.body, revision, note, &model, plan))
                }),
        );
        let save = ui.input_mut(|input| input.consume_key(Modifiers::COMMAND, Key::S));
        let width = ui.available_width();
        let mut shown = self.dm_screen.tile;
        self.dm_screen.in_tiles = true;
        let mut draw = |ui: &mut egui::Ui, tile, heading, stacked| {
            self::tile(ui, tile, heading, |ui| {
                self.dm_tile(ui, tile, book, plan, note, &sections, stacked);
            });
        };
        if width >= PLAY_PLAN_WIDTH + LIVE_NOTES_WIDTH + REFERENCE_WIDTH + 2.0 * design::S {
            let height = ui.available_height();
            // A wide screen (1440p, or 4K at 150 %) gives Open threads and the rest of the note
            // a column each, once both hold a full line of reading.
            let four =
                width - 3.0 * design::S - PLAY_PLAN_WIDTH - NOTE_MEASURE >= 2.0 * NOTE_MEASURE;
            let gaps = if four { 3.0 } else { 2.0 } * design::S;
            let tile_width = width - gaps;
            let notes_width = (tile_width - PLAY_PLAN_WIDTH - REFERENCE_WIDTH).min(NOTE_MEASURE);
            let reference_width = tile_width - PLAY_PLAN_WIDTH - notes_width;
            let columns: &[(f32, &[Tile])] = if four {
                &[
                    (PLAY_PLAN_WIDTH, &[Tile::Plan][..]),
                    (notes_width, &[Tile::Happened][..]),
                    (reference_width / 2.0, &[Tile::Threads][..]),
                    (reference_width / 2.0, &[Tile::Rest][..]),
                ]
            } else {
                &[
                    (PLAY_PLAN_WIDTH, &[Tile::Plan][..]),
                    (notes_width, &[Tile::Happened][..]),
                    (reference_width, &[Tile::Threads, Tile::Rest][..]),
                ]
            };
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = design::S;
                for &(width, tiles) in columns {
                    page_column(ui, width, height, |ui| {
                        for &tile in tiles {
                            draw(ui, tile, true, tiles.len() > 1);
                        }
                    });
                }
            });
        } else {
            design::segmented(
                ui,
                &mut shown,
                &[
                    (Tile::Plan, "Plan"),
                    (Tile::Happened, WHAT_HAPPENED),
                    (Tile::Threads, THREADS),
                    (Tile::Rest, REST),
                ],
            );
            draw(ui, shown, false, false);
            self.dm_screen.tile = shown;
        }
        self.dm_screen.in_tiles = false;
        if save && let Some(draft) = &mut self.draft {
            draft.save();
        }
        if let Some(target) = markdown_view::take_note_link(ui.ctx()) {
            self.open_note_link(target);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn dm_tile(
        &mut self,
        ui: &mut egui::Ui,
        tile: Tile,
        book: &mut Book,
        plan: &Plan<'_>,
        note: &Note,
        sections: &Sections,
        stacked: bool,
    ) {
        match tile {
            Tile::Plan => self.plan_pane(ui, book, plan, None),
            Tile::Happened => {
                let draft = self.draft.as_mut().expect("the open draft");
                // The Plan tile draws first, and its acts (Remove, Add, Undo) change the note in
                // that frame. The words' place is then read again from the changed note.
                let range = if draft.revision() == sections.revision {
                    sections.happened.clone()
                } else {
                    happened_words(&draft.body)
                };
                live_notes(ui, note, draft, range);
            }
            Tile::Threads | Tile::Rest => {
                let text = if tile == Tile::Threads {
                    &sections.threads
                } else {
                    &sections.remainder
                };
                if !text.trim().is_empty() {
                    let mut scroll = ScrollColumn::new((note.id, tile));
                    // Above the rest of the note, the threads keep it in view.
                    if stacked && tile == Tile::Threads {
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

fn tile(ui: &mut egui::Ui, tile: Tile, heading: bool, body: impl FnOnce(&mut egui::Ui)) {
    ui.scope_builder(egui::UiBuilder::new().id(("session_dm_tile", tile)), |ui| {
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
                    design::heading(ui, Heading::Section(tile.title()));
                }
                body(ui);
            });
    });
}

/// The What happened editor, over `range` of the draft. With no such section (`None`), the
/// first words typed start one at the end of the note.
fn live_notes(ui: &mut egui::Ui, note: &Note, draft: &mut Draft, range: Option<Range<usize>>) {
    let shown = range
        .as_ref()
        .map_or("", |range| &draft.body[range.clone()]);
    let mut words = Cow::Borrowed(shown);
    // The editor fills its tile: it is where the DM writes during play.
    let height = ui.available_height();
    ScrollColumn::new((note.id, Tile::Happened)).show(ui, |ui| {
        let font = Type::Body.font(ui);
        let width = ui.available_width().min(design::READER_MEASURE);
        let hint = if range.is_some() {
            "Capture what happens at the table…"
        } else {
            "Type what happens at the table. Splat adds a What happened section to the end of the note."
        };
        let output = LIVE_NOTES.of(note.id).editor(ui, &mut words, |edit| {
            edit.hint_text(hint)
                .font(font)
                .desired_width(width)
                .min_size(egui::vec2(0.0, height))
        });
        design::named(ui, &output.response, WHAT_HAPPENED);
    });
    if words.as_ref() == shown {
        return;
    }
    let replacement = words.into_owned();
    let body = &mut draft.body;
    let newline = if body.contains("\r\n") { "\r\n" } else { "\n" };
    match range {
        Some(range) => {
            // For an empty heading, keep what is typed on its own line, also when the next
            // heading immediately follows it or the file ends at the heading.
            let before = &body[..range.start];
            let after = &body[range.end..];
            let leading = if before.ends_with('\n') { "" } else { newline };
            let trailing = if after.is_empty() || after.starts_with(['\r', '\n']) {
                ""
            } else {
                newline
            };
            body.replace_range(range, &format!("{leading}{replacement}{trailing}"));
        }
        None => {
            // As `insert_in_note` adds words to a note without the section: at its end. The
            // editor's words stay exactly as typed, so the caret keeps its place.
            body.truncate(body.trim_end().len());
            if !body.is_empty() {
                body.push_str(newline);
                body.push_str(newline);
            }
            body.push_str(&format!(
                "## {WHAT_HAPPENED}{newline}{replacement}{newline}"
            ));
        }
    }
    draft.edited();
}
