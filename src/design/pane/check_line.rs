//! A checkbox whose words wrap, as a recipe (layout 11c, note C0-parts Part C v2): a list of such
//! lines (the Campaign page's changes to review) holds each one as a measured block, whole at an
//! edge.
//!
//! - **The line** is egui's check box with its words wrapped in the row's width, measured by the
//!   one rule a notice row's checks use (`check_height`): its icon and the gap after it, then its
//!   words wrapped in the rest, at least an interact height. The host's check box is egui's own,
//!   drawn in the line's slot, so its press target (the box and every row of its words), its role
//!   (CheckBox) and its name (its words) are egui's.
//! - **A line that cannot be chosen** (`blocked`) lays its words alone, muted, in Small type,
//!   with no box: the DM reads why, nothing invites a press.
//! - **The tail**: the gap a region's column leaves after an item, inside the block.
//!
//! The layout's inputs are the words, `blocked`, the width and the fonts (their generation, the
//! scale, the theme): a host keeps a line's `Laid` until one of them changes.
use egui::{Id, Rect, Response, Ui, pos2, vec2};

use super::laid::{Laid, LaidText, LaidUi};
use super::notice_row::check_height;
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
    /// The check box, as tall as its words (empty for a blocked line).
    pub const LINE: usize = 0;
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
            return Laid::new(
                self.key,
                vec2(width, height + self.tail),
                None,
                vec![LaidText {
                    at: pos2(0.0, 0.0),
                    galley,
                }],
                vec![Rect::from_min_size(pos2(0.0, 0.0), vec2(0.0, 0.0))],
            );
        }
        let height = check_height(ui, self.words, width);
        Laid::new(
            self.key,
            vec2(width, height + self.tail),
            None,
            Vec::new(),
            vec![Rect::from_min_size(pos2(0.0, 0.0), vec2(width, height))],
        )
    }

    /// Draw egui's check box in the line's slot, its words wrapping there; a press on the box or
    /// on any row of its words toggles `checked`. A blocked line draws only its laid words.
    pub fn show(&self, ui: &mut LaidUi<'_>, checked: &mut bool) -> Option<Response> {
        if self.blocked {
            return None;
        }
        let words = self.words;
        Some(ui.in_slot(slot::LINE, |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            ui.checkbox(checked, words)
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

    /// At each width and scale: the laid line is exactly as tall as the check box egui draws in
    /// it (the reference: the drawn box's own rect), its words wrap at the narrow width, and a
    /// press on the last row of its words toggles it. A blocked line has no check box node.
    #[test]
    fn a_check_line_is_as_tall_as_its_check_box_and_takes_a_press_on_any_row() {
        for scale in [1.0, 1.5] {
            for width in [340.0, 560.0, 960.0] {
                let checked = std::rc::Rc::new(std::cell::Cell::new(false));
                let laid_at = std::rc::Rc::new(std::cell::Cell::new(None::<(Rect, f32, Rect)>));
                let (checked_in, laid_in) = (checked.clone(), laid_at.clone());
                let mut harness = Harness::builder()
                    .with_size(vec2(width + 40.0, 600.0))
                    .with_pixels_per_point(scale)
                    .themed_ui(move |ui| {
                        let line = CheckLine {
                            key: Id::new("long"),
                            words: LONG,
                            blocked: false,
                            tail: 0.0,
                        };
                        let laid = line.lay(ui, width);
                        let mut drawn = Rect::NOTHING;
                        let rect = draw_laid(ui, &laid, |ui| {
                            let mut state = checked_in.get();
                            if let Some(response) = line.show(ui, &mut state) {
                                drawn = response.rect;
                            }
                            checked_in.set(state);
                        });
                        laid_in.set(Some((rect, laid.height(), drawn)));
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
                let (rect, height, drawn) = laid_at.get().expect("the line drew");
                assert!(
                    (height - drawn.height()).abs() <= 0.5 && rect.contains_rect(drawn.shrink(0.5)),
                    "{case}: the laid line ({height} tall, at {rect:?}) is the check box egui drew in it ({drawn:?})"
                );
                let boxes: Vec<_> = harness
                    .get_all_by_role(egui::accesskit::Role::CheckBox)
                    .map(|node| node.accesskit_node().label().unwrap_or_default())
                    .collect();
                assert!(
                    boxes == [LONG],
                    "{case}: one check box, named by its words; the blocked line has none: {boxes:?}"
                );
                if width < 400.0 {
                    assert!(
                        height > crate::design::theme::INTERACT_HEIGHT * 1.5,
                        "{case}: at this width the words wrap ({height})"
                    );
                }
                // A press on the last row of the words, far from the box.
                let at = pos2(drawn.center().x, drawn.bottom() - 4.0);
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
                    "{case}: a press on the words' last row ({at:?} in {drawn:?}) ticks the line"
                );
            }
        }
    }
}
