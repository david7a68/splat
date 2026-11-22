//! A block laid out before paint (migration-32, layout 11c, scope A; design note v2): a recipe
//! of `design` measures every part of a block with the fonts alone, before anything paints, so
//! the whole-block plan knows its height and the pane knows which blocks are whole before it
//! draws any (I1).
//!
//! - **Only a recipe makes one.** [`Laid::new`] is the design system's; a caller gets a `Laid`
//!   from a recipe and gives its [`Laid::block`] to the plan.
//! - **Bound to its layout.** Each `Laid` has its own layout revision. The plan's block names
//!   it, so a block drawn from another layout (a stale one) is refused: a panic in debug and
//!   test builds; in a release build a counted skip that draws nothing.
//! - **Exact rects.** The texts are the recipe's galleys, painted at their rects with no new
//!   wrap; each control goes in its slot's rect. No spacing is read from the `Ui`: the block's
//!   `Ui` has no item spacing.
//! - **Checked by real bounds.** In debug and test builds, after a block draws, every shape it
//!   painted (its visual bounds, not its layout allocation) and every widget it added lie in the
//!   block's rect, to one physical pixel.
use std::sync::Arc;

use egui::{Galley, Id, Rect, Response, Ui, UiBuilder, Vec2, Widget};

use super::plan::{Block, Measured};
use crate::design::memo::Revision;

/// A text part of a laid block: its galley (with its colours) and where it starts in the block.
#[derive(Clone, Debug)]
pub(in crate::design) struct LaidText {
    pub at: egui::Pos2,
    pub galley: Arc<Galley>,
}

/// A block laid out by a recipe: its size, its head (its semantic boundary from its top, ruling
/// L6), its texts and its control slots, each in the block's own coordinates.
#[derive(Clone, Debug)]
pub struct Laid {
    key: Id,
    size: Vec2,
    head: Option<f32>,
    revision: Revision,
    texts: Vec<LaidText>,
    slots: Vec<Rect>,
}

impl Laid {
    /// A recipe's layout of the block `key`. Each text and slot must lie in the block.
    pub(in crate::design) fn new(
        key: Id,
        size: Vec2,
        head: Option<f32>,
        texts: Vec<LaidText>,
        slots: Vec<Rect>,
    ) -> Self {
        let block = Rect::from_min_size(egui::Pos2::ZERO, size).expand(0.5);
        debug_assert!(
            texts
                .iter()
                .all(|text| block.contains_rect(Rect::from_min_size(text.at, text.galley.size())))
                && slots.iter().all(|slot| block.contains_rect(*slot)),
            "a recipe lays out every part inside its block"
        );
        Self {
            key,
            size,
            head,
            revision: Revision::next(),
            texts,
            slots,
        }
    }

    /// The block the plan measures: this layout's height, head, key and revision.
    pub fn block(&self) -> Block {
        Block::Measured(Measured::new(
            self.size.y,
            self.head,
            self.key,
            self.revision,
        ))
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn revision(&self) -> Revision {
        self.revision
    }
}

/// The `Ui` a laid block draws its controls in: its slots, at their rects.
pub struct LaidUi<'u> {
    ui: &'u mut Ui,
    laid: &'u Laid,
}

impl LaidUi<'_> {
    /// Slot `at`'s rect, on the screen.
    pub fn slot(&self, at: usize) -> Rect {
        self.laid.slots[at].translate(self.ui.max_rect().min.to_vec2())
    }

    /// `widget` in slot `at`, at its rect.
    pub fn put(&mut self, at: usize, widget: impl Widget) -> Response {
        let rect = self.slot(at);
        self.ui.put(rect, widget)
    }

    /// `add` in a `Ui` that is slot `at`'s rect, with no item spacing.
    pub fn in_slot<R>(&mut self, at: usize, add: impl FnOnce(&mut Ui) -> R) -> R {
        let rect = self.slot(at);
        let mut child = self
            .ui
            .new_child(UiBuilder::new().id_salt(("slot", at)).max_rect(rect));
        child.spacing_mut().item_spacing = Vec2::ZERO;
        add(&mut child)
    }
}

/// Draw `laid` in `ui` (the block's own `Ui`, whose rect the plan placed) when its layout is the
/// plan's (`layout`): its texts, then `draw`'s controls in its slots. `false`: a stale layout,
/// refused (a panic in debug and test builds), nothing drawn.
pub(in crate::design) fn render(
    ui: &mut Ui,
    laid: &Laid,
    layout: Revision,
    draw: impl FnOnce(&mut LaidUi<'_>),
) -> bool {
    if laid.revision != layout {
        debug_assert!(
            false,
            "a block drawn from a stale layout: the plan measured another revision"
        );
        return false;
    }
    let rect = ui.max_rect();
    ui.spacing_mut().item_spacing = Vec2::ZERO;
    let bounds = Bounds::start(ui);
    for text in &laid.texts {
        let at = Rect::from_min_size(rect.min + text.at.to_vec2(), text.galley.size());
        let _ = ui.put(
            at,
            egui::Label::new(egui::WidgetText::Galley(text.galley.clone()))
                .selectable(false)
                .wrap_mode(egui::TextWrapMode::Extend),
        );
    }
    draw(&mut LaidUi { ui, laid });
    bounds.check(ui, rect);
    true
}

/// Where a block's paint and widgets start on its layer, to check them after it draws.
struct Bounds {
    layer: egui::LayerId,
    shapes: usize,
    widgets: usize,
}

impl Bounds {
    fn start(ui: &Ui) -> Self {
        let layer = ui.layer_id();
        Self {
            layer,
            shapes: ui
                .ctx()
                .graphics(|graphics| graphics.get(layer).map_or(0, |list| list.next_idx().0)),
            widgets: ui
                .ctx()
                .viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count()),
        }
    }

    /// Debug and test builds: every shape painted since `start` (its visual bounds) and every
    /// widget added lies in `rect`, to one physical pixel.
    fn check(self, ui: &Ui, rect: Rect) {
        if !cfg!(debug_assertions) {
            return;
        }
        let room = rect.expand(1.0 / ui.ctx().pixels_per_point() + 0.01);
        let painted = ui.ctx().graphics(|graphics| {
            graphics.get(self.layer).map_or(Vec::new(), |list| {
                list.all_entries()
                    .skip(self.shapes)
                    .map(|clipped| clipped.shape.visual_bounding_rect())
                    .filter(|bounds| bounds.is_positive())
                    .collect()
            })
        });
        let widgets: Vec<Rect> = ui.ctx().viewport(|viewport| {
            viewport
                .this_pass
                .widgets
                .get_layer(self.layer)
                .skip(self.widgets)
                .map(|widget| widget.rect)
                .collect()
        });
        for bounds in painted.iter().chain(&widgets) {
            assert!(
                room.contains_rect(*bounds),
                "a laid block paints or places {bounds:?} outside its rect {rect:?}"
            );
        }
    }
}

/// A test recipe: a block of `lines` lines of `text` at `width`, with one control slot of
/// `control` px at its right edge on its first line.
#[cfg(test)]
pub(in crate::design) fn test_recipe(
    ui: &Ui,
    key: impl std::hash::Hash,
    text: &str,
    width: f32,
    control: f32,
) -> Laid {
    let font = crate::design::tokens::Type::Body.font(ui);
    let galley = ui.fonts_mut(|fonts| {
        fonts.layout(
            text.to_owned(),
            font,
            egui::Color32::GRAY,
            width - control - crate::design::tokens::S,
        )
    });
    let control_height = crate::design::theme::INTERACT_HEIGHT.max(crate::design::tokens::XL);
    let height = galley.size().y.max(control_height);
    Laid::new(
        Id::new(key),
        egui::vec2(width, height),
        None,
        vec![LaidText {
            at: egui::Pos2::ZERO,
            galley,
        }],
        vec![Rect::from_min_size(
            egui::pos2(width - control, 0.0),
            egui::vec2(control, control_height),
        )],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::{
        memo::Revised,
        outcome::{Messages, NoPress, PaneFrame},
        pane::{BlockRegion, Body, RegionState},
    };
    use egui_kittest::{Harness, kittest::Queryable};

    /// A Content pane whose body is a region of measured blocks: `Row k` and an `Act k` control.
    struct Page {
        texts: Vec<String>,
        laid: Vec<Laid>,
        rows: Revised<Vec<Block>>,
        region: RegionState,
        /// The pane's visible room and the region's top, in this pass.
        clip: Rect,
        top: f32,
        clicked: Vec<usize>,
        /// A control paints a mark outside its block.
        overflow: Option<usize>,
        /// Lay out the blocks again, without giving the plan the new layouts.
        stale: bool,
    }

    impl Page {
        fn new(texts: Vec<String>) -> Self {
            Self {
                texts,
                laid: Vec::new(),
                rows: Revised::new(Vec::new()),
                region: RegionState::new("laid_test"),
                clip: Rect::NOTHING,
                top: 0.0,
                clicked: Vec::new(),
                overflow: None,
                stale: false,
            }
        }
    }

    fn harness(page: Page, size: egui::Vec2) -> Harness<'static, Page> {
        let mut harness = Harness::builder().with_size(size).build_ui_state(
            |ui, page: &mut Page| {
                let frame = PaneFrame::new(ui);
                let messages = Messages::<NoPress>::new(&frame, "laid_test", ui.available_width());
                let _ = messages.pane(ui, Body::Content, |ui| {
                    let width = ui.available_width();
                    if page.laid.is_empty() || std::mem::take(&mut page.stale) {
                        page.laid = (0..page.texts.len())
                            .map(|at| test_recipe(ui, at, &page.texts[at], width, 80.0))
                            .collect();
                        if page.rows.is_empty() {
                            page.rows.set(page.laid.iter().map(Laid::block).collect());
                        }
                    }
                    page.top = ui.cursor().top();
                    page.clip = ui.clip_rect();
                    let Page {
                        laid,
                        rows,
                        region,
                        clicked,
                        overflow,
                        ..
                    } = page;
                    BlockRegion::new("laid_test", region, rows).show_laid(
                        ui,
                        |at| &laid[at],
                        |ui, at| {
                            if ui.put(0, egui::Button::new(format!("Act {at}"))).clicked() {
                                clicked.push(at);
                            }
                            if *overflow == Some(at) {
                                let slot = ui.slot(0);
                                ui.in_slot(0, |ui| {
                                    ui.painter().rect_filled(
                                        slot.translate(egui::vec2(0.0, slot.height() + 40.0)),
                                        0.0,
                                        egui::Color32::RED,
                                    );
                                });
                            }
                        },
                    );
                });
            },
            page,
        );
        crate::design::theme::install(&harness.ctx);
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        harness.run_steps(3);
        harness
    }

    /// The texts painted in the last frame.
    fn painted(harness: &Harness<'_, Page>) -> Vec<String> {
        fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
                egui::Shape::Text(text) => out.push(text.galley.text().to_owned()),
                _ => {}
            }
        }
        let mut out = Vec::new();
        for shape in &harness.output().shapes {
            walk(&shape.shape, &mut out);
        }
        out
    }

    fn wheel(harness: &mut Harness<'_, Page>, by: f32) {
        let at = harness.state().clip.center();
        harness
            .input_mut()
            .events
            .push(egui::Event::PointerMoved(at));
        harness.input_mut().events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -by),
            modifiers: egui::Modifiers::NONE,
        });
        harness.run_steps(4);
    }

    /// Block `at`'s rect on the screen, from its layout.
    fn block_rect(page: &Page, at: usize) -> Rect {
        let top = page.top + page.laid[..at].iter().map(Laid::height).sum::<f32>();
        Rect::from_min_size(
            egui::pos2(page.clip.left(), top),
            egui::vec2(page.clip.width(), page.laid[at].height()),
        )
    }

    fn texts(count: usize) -> Vec<String> {
        (0..count)
            .map(|at| {
                let words = "long words for a wrapped line ".repeat(at % 4 * 3);
                format!("Row {at} {words}")
            })
            .collect()
    }

    /// Layout 11c, scope A (I1): a measured body in a Content pane is drawn from its mask,
    /// decided before any block draws. At offsets that cut a block at each edge, at three
    /// widths, the cut blocks have no accessible node, no paint and no press area; every block
    /// drawn lies whole in the pane's room. Red: draw every visible block (the legacy path).
    #[test]
    fn a_measured_body_draws_only_its_whole_blocks() {
        for width in [960.0, 1200.0, 1920.0] {
            let mut harness = harness(Page::new(texts(40)), egui::vec2(width, 420.0));
            let mut cut_seen = [false, false];
            for step in 0..12 {
                wheel(&mut harness, 37.0 + step as f32);
                let page = harness.state();
                let texts = painted(&harness);
                for at in 0..page.laid.len() {
                    let rect = block_rect(page, at);
                    let label = format!("Act {at}");
                    let node = harness.query_by_label(&label);
                    let cut_top =
                        rect.top() < page.clip.top() - 0.5 && rect.bottom() > page.clip.top() + 0.5;
                    let cut_bottom = rect.top() < page.clip.bottom() - 0.5
                        && rect.bottom() > page.clip.bottom() + 0.5;
                    let whole = rect.top() >= page.clip.top() - 0.5
                        && rect.bottom() <= page.clip.bottom() + 0.5;
                    let drawn = texts
                        .iter()
                        .any(|text| text.starts_with(&format!("Row {at} ")));
                    if cut_top || cut_bottom {
                        cut_seen[usize::from(cut_bottom)] = true;
                        assert!(
                            node.is_none() && !drawn,
                            "{width}: block {at}, cut by the room's edge, is not drawn"
                        );
                    } else {
                        assert_eq!(
                            node.is_some(),
                            whole,
                            "{width}: block {at} is drawn exactly when it is whole in the room"
                        );
                    }
                }
                // A press on a cut block's visible part reaches nothing.
                let cut = (0..page.laid.len()).find(|&at| {
                    let rect = block_rect(page, at);
                    rect.top() < page.clip.bottom() - 0.5
                        && rect.bottom() > page.clip.bottom() + 0.5
                });
                if let Some(at) = cut {
                    let rect = block_rect(page, at).intersect(page.clip);
                    let press = egui::pos2(rect.right() - 40.0, rect.center().y);
                    harness
                        .input_mut()
                        .events
                        .push(egui::Event::PointerMoved(press));
                    for pressed in [true, false] {
                        harness.input_mut().events.push(egui::Event::PointerButton {
                            pos: press,
                            button: egui::PointerButton::Primary,
                            pressed,
                            modifiers: egui::Modifiers::NONE,
                        });
                        harness.step();
                    }
                    assert!(
                        harness.state().clicked.is_empty(),
                        "{width}: a press on the cut block {at} reaches nothing"
                    );
                }
            }
            assert_eq!(cut_seen, [true, true], "{width}: both edges cut a block");
        }
    }

    /// Ruling L6, item 6 of the v2 review: a closed block taller than the pane keeps its clipped
    /// paint, counted, so its control stays reachable; it is never masked away.
    #[test]
    fn a_block_taller_than_the_pane_stays_reachable_and_counted() {
        let mut tall = texts(8);
        tall[3] = format!("Row 3 {}", "a played summary that goes on ".repeat(200));
        let mut harness = harness(Page::new(tall), egui::vec2(960.0, 300.0));
        let mut reached = false;
        for _ in 0..30 {
            wheel(&mut harness, 60.0);
            let page = harness.state();
            let rect = block_rect(page, 3);
            if rect.top() < page.clip.top() && rect.bottom() > page.clip.bottom() {
                reached = true;
                assert!(
                    painted(&harness)
                        .iter()
                        .any(|text| text.starts_with("Row 3 ")),
                    "the tall block keeps its clipped paint"
                );
                assert!(page.region.work().too_tall > 0, "the tall block is counted");
            }
        }
        assert!(reached, "the view lay inside the tall block");
        let mut harness = self::harness(
            Page::new({
                let mut texts = texts(8);
                texts[0] = format!("Row 0 {}", "a played summary that goes on ".repeat(200));
                texts
            }),
            egui::vec2(960.0, 300.0),
        );
        harness.run_steps(2);
        assert!(
            harness.query_by_label("Act 0").is_some(),
            "the control at the tall block's top is reachable"
        );
    }

    /// Item 4 of the v2 review: the bounds check reads the painted shapes, not the layout
    /// allocation: a mark painted outside the block fails it.
    #[test]
    #[should_panic(expected = "outside its rect")]
    fn paint_outside_a_block_fails_the_bounds_check() {
        let mut page = Page::new(texts(6));
        page.overflow = Some(1);
        let _ = harness(page, egui::vec2(960.0, 420.0));
    }

    /// A block drawn from a layout the plan did not measure is refused.
    #[test]
    #[should_panic(expected = "stale layout")]
    fn a_stale_layout_is_refused() {
        let mut harness = harness(Page::new(texts(6)), egui::vec2(960.0, 420.0));
        harness.state_mut().stale = true;
        harness.step();
    }

    /// The block list's revision changes with every geometry input: a reorder of two blocks of
    /// the same height (the same total height and tops) plans again, and an unchanged list does
    /// not.
    #[test]
    fn a_reorder_plans_again() {
        let mut harness = harness(Page::new(texts(6)), egui::vec2(960.0, 420.0));
        let builds = harness.state().region.work().builds;
        harness.run_steps(2);
        assert_eq!(
            harness.state().region.work().builds,
            builds,
            "an unchanged list is not planned again"
        );
        let page = harness.state_mut();
        let mut swapped: Vec<Block> = page.rows.to_vec();
        // Rows 0 and 4 have the same text length: the same heights, so the same total height.
        swapped.swap(0, 4);
        page.laid.swap(0, 4);
        page.rows.set(swapped);
        harness.run_steps(2);
        assert_eq!(
            harness.state().region.work().builds,
            builds + 1,
            "a reorder plans again"
        );
    }
}
