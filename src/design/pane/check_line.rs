//! A checkbox whose words wrap, as a recipe (layout 11c, note C0-parts Part C): its box and its
//! words are measured with the fonts before anything paints, so a list of such lines (the
//! Campaign page's changes to review) holds each one as a measured block, whole at an edge.
//!
//! The geometry is egui's `Checkbox` with wrapping words, decided before paint:
//!
//! - **The box**, `icon_width` square at the line's start, centred on the words' first row: a
//!   long line's box stays beside its first words.
//! - **The words**, `icon_spacing` after the box, wrapped in the rest of the width, Body type.
//! - **The press target** is the whole line, the box and every row of its words, as egui's
//!   checkbox takes a press on its words. Its role is CheckBox and its words are its name.
//! - **A line that cannot be chosen** (`blocked`) lays its words alone, muted, in Small type,
//!   with no box: the DM reads why, nothing invites a press.
//!
//! The line is at least one interact height tall (egui's checkbox's least size).
use egui::{Id, Rect, Response, Sense, Ui, WidgetInfo, WidgetType, pos2, vec2};

use super::laid::{Laid, LaidText, LaidUi};
use crate::design::tokens::{Type, colors};

/// A checkbox line whose words wrap (note C0-parts Part C).
#[derive(Clone, Copy)]
pub struct CheckLine<'a> {
    /// The line's stable key (the choice it toggles).
    pub key: Id,
    /// The words: the checkbox's name, wrapped in the line.
    pub words: &'a str,
    /// The line cannot be chosen: its words alone, muted, with no box.
    pub blocked: bool,
    /// The gap the column leaves after the block (in a region); zero in place.
    pub tail: f32,
}

/// The slots of a laid check line.
pub mod slot {
    /// The whole line: the press target.
    pub const LINE: usize = 0;
    /// The box (empty for a blocked line).
    pub const BOX: usize = 1;
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many check lines were laid out (a host keeps its layouts on quiet frames).
    pub static CHECK_LINES_LAID: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl CheckLine<'_> {
    /// The line laid out in a row `width` wide.
    pub fn lay(&self, ui: &Ui, width: f32) -> Laid {
        #[cfg(any(test, feature = "test-support"))]
        CHECK_LINES_LAID.with(|laid| laid.set(laid.get() + 1));
        let spacing = ui.spacing();
        let least = spacing.interact_size.y.max(spacing.icon_width);
        if self.blocked {
            let galley = ui.fonts_mut(|fonts| {
                fonts.layout(
                    self.words.to_owned(),
                    Type::Small.font(ui),
                    colors().muted,
                    width,
                )
            });
            let height = galley.size().y;
            let line = Rect::from_min_size(pos2(0.0, 0.0), vec2(width, height));
            return Laid::new(
                self.key,
                vec2(width, height + self.tail),
                None,
                vec![LaidText {
                    at: pos2(0.0, 0.0),
                    galley,
                }],
                vec![line, Rect::from_min_size(pos2(0.0, 0.0), vec2(0.0, 0.0))],
            );
        }
        let (icon, gap) = (spacing.icon_width, spacing.icon_spacing);
        let left = icon + gap;
        let galley = ui.fonts_mut(|fonts| {
            fonts.layout(
                self.words.to_owned(),
                Type::Body.font(ui),
                colors().text,
                (width - left).max(1.0),
            )
        });
        let height = galley.size().y.max(least);
        // One row of words is centred in the line, as egui centres a checkbox's words; more
        // rows start at the top. The box is centred on the first row.
        let top = (height - galley.size().y) / 2.0;
        let first = galley
            .rows
            .first()
            .map_or(galley.size().y, |row| row.rect().height());
        let centre = if galley.rows.len() > 1 {
            first / 2.0
        } else {
            height / 2.0
        };
        let centre = centre.clamp(icon / 2.0, height - icon / 2.0);
        let box_rect = Rect::from_center_size(pos2(icon / 2.0, centre), vec2(icon, icon));
        let line = Rect::from_min_size(pos2(0.0, 0.0), vec2(width, height));
        Laid::new(
            self.key,
            vec2(width, height + self.tail),
            None,
            vec![LaidText {
                at: pos2(left, top),
                galley,
            }],
            vec![line, box_rect],
        )
    }

    /// Draw the line's box and its press target in its laid slots; a press on the box or on any
    /// row of its words toggles `checked`. A blocked line draws nothing more than its words.
    pub fn show(&self, ui: &mut LaidUi<'_>, checked: &mut bool) -> Option<Response> {
        if self.blocked {
            return None;
        }
        let box_rect = ui.slot(slot::BOX);
        let words = self.words;
        Some(ui.in_slot(slot::LINE, |ui| {
            let rect = ui.max_rect();
            let mut response = ui.allocate_rect(rect, Sense::click());
            if response.clicked() {
                *checked = !*checked;
                response.mark_changed();
            }
            let enabled = ui.is_enabled();
            let state = *checked;
            response
                .widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, enabled, state, words));
            if ui.is_rect_visible(rect) {
                let visuals = *ui.style().interact(&response);
                let small = Rect::from_center_size(
                    box_rect.center(),
                    egui::Vec2::splat(ui.spacing().icon_width_inner),
                );
                ui.painter().add(egui::epaint::RectShape::new(
                    box_rect.expand(visuals.expansion),
                    visuals.corner_radius,
                    visuals.bg_fill,
                    visuals.bg_stroke,
                    egui::StrokeKind::Inside,
                ));
                if state {
                    ui.painter().add(egui::Shape::line(
                        vec![
                            pos2(small.left(), small.center().y),
                            pos2(small.center().x, small.bottom()),
                            pos2(small.right(), small.top()),
                        ],
                        visuals.fg_stroke,
                    ));
                }
                if response.has_focus() {
                    crate::design::components::focus_ring(ui, box_rect);
                }
            }
            response
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::pane::draw_laid;
    use crate::design::test_harness::ThemedPage as _;
    use egui_kittest::Harness;
    use egui_kittest::kittest::{NodeT, Queryable};

    const LONG: &str = "Ash Wraith (creature): move to Ashen Marches, the package that now holds it and the two \
                        encounters that use it";

    /// At each width and scale: the line is as tall as egui's own checkbox with the same words
    /// wrapped in the same width (the reference), its box sits on the first row of its words,
    /// and a press on the last row of its words toggles it. A blocked line has no box and no
    /// check box node.
    #[test]
    fn a_check_line_is_as_tall_as_its_words_and_takes_a_press_on_any_row() {
        for scale in [1.0, 1.5] {
            for width in [340.0, 560.0, 960.0] {
                let checked = std::rc::Rc::new(std::cell::Cell::new(false));
                let laid_at = std::rc::Rc::new(std::cell::Cell::new(None::<(Rect, f32)>));
                let reference = std::rc::Rc::new(std::cell::Cell::new(0.0_f32));
                let (checked_in, laid_in, reference_in) =
                    (checked.clone(), laid_at.clone(), reference.clone());
                let mut harness = Harness::builder()
                    .with_size(vec2(width + 40.0, 600.0))
                    .with_pixels_per_point(scale)
                    .themed_ui(move |ui| {
                        ui.set_width(width);
                        let line = CheckLine {
                            key: Id::new("long"),
                            words: LONG,
                            blocked: false,
                            tail: 0.0,
                        };
                        let laid = line.lay(ui, width);
                        let rect = draw_laid(ui, &laid, |ui| {
                            let mut state = checked_in.get();
                            let _ = line.show(ui, &mut state);
                            checked_in.set(state);
                        });
                        laid_in.set(Some((rect, laid.height())));
                        // egui's own checkbox, its words wrapping in the same width.
                        ui.scope(|ui| {
                            ui.set_max_width(width);
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                            let mut state = false;
                            let words = egui::RichText::new(LONG).font(Type::Body.font(ui));
                            let response = ui.checkbox(&mut state, words);
                            reference_in.set(response.rect.height());
                        });
                        let blocked = CheckLine {
                            key: Id::new("blocked"),
                            words: "Ember Cultist (creature): remove. Not available: it is in use.",
                            blocked: true,
                            tail: 0.0,
                        };
                        let laid = blocked.lay(ui, width);
                        draw_laid(ui, &laid, |ui| {
                            let mut state = false;
                            assert!(blocked.show(ui, &mut state).is_none());
                        });
                    });
                harness.run_steps(2);
                let case = format!("width {width}, scale {scale}");
                let (rect, height) = laid_at.get().expect("the line drew");
                assert!(
                    (height - reference.get()).abs() <= 1.0,
                    "{case}: the laid line is {height} tall, egui's checkbox with the same words {}",
                    reference.get()
                );
                // Two check boxes named by the words: the laid line and egui's reference. The
                // blocked line adds none.
                let boxes: Vec<_> = harness
                    .get_all_by_role(egui::accesskit::Role::CheckBox)
                    .map(|node| node.accesskit_node().label().unwrap_or_default())
                    .collect();
                assert!(
                    boxes.len() == 2 && boxes.iter().all(|label| label == LONG),
                    "{case}: the line is a check box named by its words, the blocked line none: {boxes:?}"
                );
                // A press on the last row of the words, far from the box.
                let at = pos2(rect.center().x, rect.bottom() - 4.0);
                harness.event(egui::Event::PointerMoved(at));
                for pressed in [true, false] {
                    harness.event(egui::Event::PointerButton {
                        pos: at,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    });
                    harness.step();
                }
                harness.run_steps(1);
                assert!(
                    checked.get(),
                    "{case}: a press on the words' last row ({at:?} in {rect:?}) ticks the line"
                );
                if width < 400.0 {
                    assert!(
                        height > crate::design::theme::INTERACT_HEIGHT * 1.5,
                        "{case}: at this width the words wrap ({height})"
                    );
                }
            }
        }
    }

    /// The box is centred on the words' first row, however many rows they take: read from the
    /// paint (the words' galley as painted) and the box's slot as drawn.
    #[test]
    fn the_box_stands_beside_the_first_row() {
        let drawn = std::rc::Rc::new(std::cell::Cell::new(None::<Rect>));
        let drawn_in = drawn.clone();
        let mut harness = Harness::builder()
            .with_size(vec2(400.0, 300.0))
            .themed_ui(move |ui| {
                let line = CheckLine {
                    key: Id::new("long"),
                    words: LONG,
                    blocked: false,
                    tail: 0.0,
                };
                let laid = line.lay(ui, 300.0);
                draw_laid(ui, &laid, |ui| {
                    drawn_in.set(Some(ui.slot(slot::BOX)));
                    let mut state = true;
                    let _ = line.show(ui, &mut state);
                });
            });
        harness.run_steps(2);
        let box_rect = drawn.get().expect("the box drew");
        let (rows, first) = harness
            .output()
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text == LONG => Some((
                    text.galley.rows.len(),
                    text.pos.y + text.galley.rows[0].rect().center().y,
                )),
                _ => None,
            })
            .expect("the words are painted");
        assert!(rows > 1, "the words wrap at 300 px ({rows} rows)");
        assert!(
            (box_rect.center().y - first).abs() <= 0.5,
            "the box {box_rect:?} is centred on the first row's centre {first}"
        );
    }
}
