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
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "layout 11c A2's `PlanRow` recipe is the first to lay out a block"
        )
    )]
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
    /// How many control slots the layout has.
    pub fn slots(&self) -> usize {
        self.laid.slots.len()
    }

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

/// Where a block's paint and widgets start on every layer it may paint to, to check them after
/// it draws (debug and test builds).
struct Bounds {
    starts: Vec<(egui::LayerId, usize, usize)>,
}

impl Bounds {
    /// The layers a block's renderer may paint to: its own, and every layer egui knows in this
    /// pass (each `Area`; outside `src/design/` a layer comes only from a design part, as Clippy
    /// bans `Ui::with_layer_id` and `Painter::new`), except a popup's (`Foreground`, `Tooltip`,
    /// `Debug`: a menu, a tooltip or a drag ghost is a design part that leaves its block by rule).
    fn layers(ui: &Ui) -> Vec<egui::LayerId> {
        let mut layers: Vec<egui::LayerId> = ui.ctx().memory(|memory| memory.layer_ids().collect());
        if !layers.contains(&ui.layer_id()) {
            layers.push(ui.layer_id());
        }
        layers.retain(|layer| {
            !matches!(
                layer.order,
                egui::Order::Foreground | egui::Order::Tooltip | egui::Order::Debug
            )
        });
        layers
    }

    fn counts(ui: &Ui, layer: egui::LayerId) -> (usize, usize) {
        (
            ui.ctx()
                .graphics(|graphics| graphics.get(layer).map_or(0, |list| list.next_idx().0)),
            ui.ctx()
                .viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count()),
        )
    }

    fn start(ui: &Ui) -> Self {
        if !cfg!(debug_assertions) {
            return Self { starts: Vec::new() };
        }
        Self {
            starts: Self::layers(ui)
                .into_iter()
                .map(|layer| {
                    let (shapes, widgets) = Self::counts(ui, layer);
                    (layer, shapes, widgets)
                })
                .collect(),
        }
    }

    /// Debug and test builds: every shape painted since `start` on any of those layers (its
    /// visual bounds) and every widget added lies in `rect`, to one physical pixel.
    fn check(self, ui: &Ui, rect: Rect) {
        if !cfg!(debug_assertions) {
            return;
        }
        let room = rect.expand(1.0 / ui.ctx().pixels_per_point() + 0.01);
        for layer in Self::layers(ui) {
            let (shapes, widgets) = self
                .starts
                .iter()
                .find(|(kept, ..)| *kept == layer)
                .map_or((0, 0), |(_, shapes, widgets)| (*shapes, *widgets));
            let painted: Vec<Rect> = ui.ctx().graphics(|graphics| {
                graphics.get(layer).map_or(Vec::new(), |list| {
                    list.all_entries()
                        .skip(shapes)
                        .map(|clipped| clipped.shape.visual_bounding_rect())
                        .filter(|bounds| bounds.is_positive())
                        .collect()
                })
            });
            let placed: Vec<Rect> = ui.ctx().viewport(|viewport| {
                viewport
                    .this_pass
                    .widgets
                    .get_layer(layer)
                    .skip(widgets)
                    .map(|widget| widget.rect)
                    .collect()
            });
            for bounds in painted.iter().chain(&placed) {
                assert!(
                    room.contains_rect(*bounds),
                    "a laid block paints or places {bounds:?} on {layer:?} outside its rect {rect:?}"
                );
            }
        }
    }
}

/// A test recipe of fixed geometry: `texts` (each wrapped beside the control column) at their
/// tops, a block of `size` with its `head`, and a control slot of `control` px at the right edge
/// at each of `slots`' tops.
#[cfg(test)]
pub(in crate::design) fn test_block(
    ui: &Ui,
    key: impl std::hash::Hash,
    texts: &[(&str, f32)],
    (size, head): (Vec2, Option<f32>),
    slots: &[f32],
    control: f32,
) -> Laid {
    let font = crate::design::tokens::Type::Body.font(ui);
    let wrap = size.x - control - crate::design::tokens::S;
    let texts = texts
        .iter()
        .map(|(text, top)| LaidText {
            at: egui::pos2(0.0, *top),
            galley: ui.fonts_mut(|fonts| {
                fonts.layout((*text).to_owned(), font.clone(), egui::Color32::GRAY, wrap)
            }),
        })
        .collect();
    let height = crate::design::theme::INTERACT_HEIGHT;
    Laid::new(
        Id::new(key),
        size,
        head,
        texts,
        slots
            .iter()
            .map(|top| {
                Rect::from_min_size(
                    egui::pos2(size.x - control, *top),
                    egui::vec2(control, height),
                )
            })
            .collect(),
    )
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
        pane::{BlockRegion, Body, Navigation, RegionState},
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
        /// A section 300 px tall with a 150 px head (`Head k` over `Body k`, a control on each).
        split: Option<usize>,
        /// A block of this many controls, 80 px apart.
        controls: Option<(usize, usize)>,
        /// A control paints outside its block on a layer of its own.
        stray: Option<usize>,
        /// Every block drawn asks egui to scroll to the region's top (a raw `scroll_to_rect`).
        pull: bool,
        /// Another scroll column beside the pane (its rect in this pass).
        beside: Option<Rect>,
        /// Block 1 takes block 0's key.
        duplicate: bool,
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
                split: None,
                controls: None,
                stray: None,
                pull: false,
                beside: None,
                duplicate: false,
            }
        }
    }

    fn harness(page: Page, size: egui::Vec2) -> Harness<'static, Page> {
        let mut harness = Harness::builder().with_size(size).build_ui_state(
            |ui, page: &mut Page| {
                let full = ui.available_rect_before_wrap();
                if page.beside.is_some() {
                    let (_, right) = full.split_left_right_at_fraction(0.5);
                    page.beside = Some(right);
                    ui.scope_builder(egui::UiBuilder::new().max_rect(right), |ui| {
                        crate::design::layout::ScrollColumn::new("laid_test_beside").show(
                            ui,
                            |ui| {
                                for line in 0..80 {
                                    let _ = ui.label(format!("Beside {line}"));
                                }
                            },
                        );
                    });
                }
                let left = page
                    .beside
                    .map_or(full, |_| full.split_left_right_at_fraction(0.5).0);
                let mut ui = ui.new_child(egui::UiBuilder::new().max_rect(left));
                let ui = &mut ui;
                let frame = PaneFrame::new(ui);
                let messages = Messages::<NoPress>::new(&frame, "laid_test", ui.available_width());
                let _ = messages.pane(ui, Body::Content, |ui| {
                    let width = ui.available_width();
                    if page.laid.is_empty() || std::mem::take(&mut page.stale) {
                        let (split, controls, duplicate) =
                            (page.split, page.controls, page.duplicate);
                        page.laid = (0..page.texts.len())
                            .map(|at| match (split, controls) {
                                (Some(split), _) if split == at => test_block(
                                    ui,
                                    at,
                                    &[
                                        (&format!("Head {at} "), 0.0),
                                        (&format!("Body {at} "), 150.0),
                                    ],
                                    (egui::vec2(width, 300.0), Some(150.0)),
                                    &[0.0, 150.0],
                                    80.0,
                                ),
                                (_, Some((block, count))) if block == at => test_block(
                                    ui,
                                    at,
                                    &[(&format!("Row {at} "), 0.0)],
                                    (egui::vec2(width, 80.0 * count as f32), None),
                                    &(0..count).map(|i| 80.0 * i as f32).collect::<Vec<_>>(),
                                    80.0,
                                ),
                                _ if duplicate && at == 1 => {
                                    test_recipe(ui, 0usize, &page.texts[at], width, 80.0)
                                }
                                _ => test_recipe(ui, at, &page.texts[at], width, 80.0),
                            })
                            .collect();
                        if page.rows.is_empty() {
                            page.rows.set(page.laid.iter().map(Laid::block).collect());
                        }
                    }
                    page.top = ui.cursor().top();
                    page.clip = ui.clip_rect();
                    let (top, pull) = (page.top, page.pull);
                    let Page {
                        laid,
                        rows,
                        region,
                        clicked,
                        overflow,
                        stray,
                        ..
                    } = page;
                    BlockRegion::new("laid_test", region, rows).show_laid(
                        ui,
                        |at| &laid[at],
                        |ui, at| {
                            if ui.put(0, egui::Button::new(format!("Act {at}"))).clicked() {
                                clicked.push(at);
                            }
                            for slot in 1..ui.slots() {
                                let _ = ui.put(slot, egui::Button::new(format!("Act {at}.{slot}")));
                            }
                            if *stray == Some(at) {
                                let slot = ui.slot(0);
                                ui.in_slot(0, |ui| stray_mark(ui, slot));
                            }
                            if pull {
                                ui.in_slot(0, |ui| {
                                    ui.scroll_to_rect(
                                        Rect::from_min_size(
                                            egui::pos2(slot_left(ui), top),
                                            egui::vec2(1.0, 1.0),
                                        ),
                                        None,
                                    );
                                });
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

    fn slot_left(ui: &Ui) -> f32 {
        ui.max_rect().left()
    }

    /// A mark painted 300 px below `slot`, on an `Area` of its own at the middle order (a layer
    /// other than the block's).
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn stray_mark(ui: &Ui, slot: Rect) {
        egui::Area::new(Id::new("laid_test_stray"))
            .order(egui::Order::Middle)
            .fixed_pos(slot.min + egui::vec2(0.0, 300.0))
            .show(ui.ctx(), |ui| {
                ui.painter().rect_filled(
                    Rect::from_min_size(ui.max_rect().min, slot.size()),
                    0.0,
                    egui::Color32::RED,
                );
            });
    }

    /// The texts painted in the last frame where their clip shows them (a shape that lies wholly
    /// outside its clip shows nothing).
    fn painted(harness: &Harness<'_, Page>) -> Vec<String> {
        fn walk(shape: &egui::Shape, clip: Rect, out: &mut Vec<String>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, clip, out)),
                egui::Shape::Text(text) => {
                    let shown = clip.intersect(shape.visual_bounding_rect());
                    if shown.height() > 0.5 && shown.width() > 0.5 {
                        out.push(text.galley.text().to_owned());
                    }
                }
                _ => {}
            }
        }
        let mut out = Vec::new();
        for shape in &harness.output().shapes {
            walk(&shape.shape, shape.clip_rect, &mut out);
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

    /// Whether block `at` is drawn whole in the pane's room.
    fn shown(harness: &Harness<'_, Page>, at: usize) -> bool {
        let page = harness.state();
        harness.query_by_label(&format!("Act {at}")).is_some()
            && page.clip.expand(0.5).contains_rect(block_rect(page, at))
    }

    fn key_press(harness: &mut Harness<'_, Page>, key: egui::Key, modifiers: egui::Modifiers) {
        for pressed in [true, false] {
            harness.input_mut().events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed,
                repeat: false,
                modifiers,
            });
        }
        harness.run_steps(3);
    }

    /// Review of PR 743 (P2): a section taller than the pane splits at its head (L6). A 300 px
    /// block with a 150 px head, shown from its top in a room between the two: its head is whole
    /// and draws (its words and its control); its cut body does not paint. Red: draw only the
    /// whole and the `TooTall` blocks.
    #[test]
    fn a_split_section_draws_its_whole_head() {
        let mut page = Page::new(texts(8));
        page.split = Some(3);
        let mut harness = harness(page, egui::vec2(960.0, 260.0));
        harness
            .state_mut()
            .region
            .navigate(Navigation::Show(Id::new(3usize)));
        harness.run_steps(3);
        let page = harness.state();
        let rect = block_rect(page, 3);
        assert!(
            page.clip.height() > 150.0 && page.clip.height() < 300.0,
            "the room {:?} is between the head and the section",
            page.clip
        );
        assert!(
            (rect.top() - page.clip.top()).abs() < 1.0,
            "a section taller than the room shows its head at the room's top: {rect:?} in {:?}",
            page.clip
        );
        let texts = painted(&harness);
        assert!(
            texts.iter().any(|text| text.starts_with("Head 3 "))
                && harness.query_by_label("Act 3").is_some(),
            "the whole head draws, with its control"
        );
        assert!(
            !texts.iter().any(|text| text.starts_with("Body 3 ")),
            "the cut body does not paint"
        );
    }

    /// Review of PR 743 (P2): a renderer that paints outside its block on a layer of its own
    /// (an `Area`) fails the bounds check too.
    #[test]
    #[should_panic(expected = "outside its rect")]
    fn paint_on_another_layer_fails_the_bounds_check() {
        let mut page = Page::new(texts(6));
        page.stray = Some(1);
        let _ = harness(page, egui::vec2(960.0, 420.0));
    }

    /// Review of PR 743 (P3): Tab goes into and through a block taller than the pane, and focus
    /// clearance brings each of its controls whole into view.
    #[test]
    fn tab_reaches_each_control_of_a_block_taller_than_the_pane() {
        let mut page = Page::new(texts(6));
        page.controls = Some((2, 8));
        let mut harness = harness(page, egui::vec2(960.0, 300.0));
        harness.get_by_label("Act 1").focus();
        harness.run_steps(3);
        for slot in 0..8 {
            key_press(&mut harness, egui::Key::Tab, egui::Modifiers::NONE);
            let label = if slot == 0 {
                "Act 2".to_owned()
            } else {
                format!("Act 2.{slot}")
            };
            let node = harness.get_by_label(&label);
            assert!(node.is_focused(), "Tab {slot} focuses {label}");
            let clip = harness.state().clip;
            assert!(
                clip.expand(0.5).contains_rect(node.rect()),
                "{label} {:?} is whole in the room {clip:?}",
                node.rect()
            );
        }
        assert!(
            harness.state().region.work().too_tall > 0,
            "the block is counted"
        );
    }

    /// Layout 11c, A1b (the v2 review's item 2): the resolver of a measured body in a Content
    /// pane. A request names its block by key and shows it whole, in the next frame with one pass
    /// a frame and in the asking frame with two; a key no longer in the plan ends its request;
    /// the DM's wheel in the same pass cancels it; a held press makes it wait, and it lands after
    /// the release; and a raw `scroll_to_rect` in the passes that queue and apply it does not
    /// move the view afterwards.
    #[test]
    fn a_request_shows_its_block_by_key_in_the_order_of_the_resolver() {
        let mut harness = harness(Page::new(texts(40)), egui::vec2(960.0, 420.0));
        let show = |harness: &mut Harness<'_, Page>, at: usize| {
            harness
                .state_mut()
                .region
                .navigate(Navigation::Show(Id::new(at)));
        };
        show(&mut harness, 30);
        harness.step();
        assert!(!shown(&harness, 30), "one pass a frame: not yet");
        harness.step();
        assert!(shown(&harness, 30), "one pass a frame: the next frame");
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(2).unwrap());
        show(&mut harness, 5);
        harness.step();
        assert!(shown(&harness, 5), "two passes: the asking frame");
        let top = block_rect(harness.state(), 5).top();
        harness
            .state_mut()
            .region
            .navigate(Navigation::Show(Id::new("gone")));
        harness.run_steps(3);
        assert_eq!(
            block_rect(harness.state(), 5).top(),
            top,
            "a key not in the plan moves nothing"
        );
        // The DM's wheel in the same pass cancels it.
        show(&mut harness, 35);
        let at = harness.state().clip.center();
        harness
            .input_mut()
            .events
            .push(egui::Event::PointerMoved(at));
        harness.input_mut().events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -10.0),
            modifiers: egui::Modifiers::NONE,
        });
        harness.run_steps(4);
        assert!(!shown(&harness, 35), "the wheel cancels the request");
        // A held press makes it wait; it lands after the release.
        let at = harness.state().clip.center();
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        harness
            .input_mut()
            .events
            .push(egui::Event::PointerMoved(at));
        harness.input_mut().events.push(button(true));
        harness.step();
        show(&mut harness, 25);
        harness.run_steps(2);
        assert!(
            !shown(&harness, 25),
            "the request waits while the press is held"
        );
        harness.input_mut().events.push(button(false));
        harness.run_steps(3);
        assert!(shown(&harness, 25), "it lands after the release");
        // A wheel while the press is held cancels the waiting request.
        harness
            .input_mut()
            .events
            .push(egui::Event::PointerMoved(at));
        harness.input_mut().events.push(button(true));
        harness.step();
        show(&mut harness, 38);
        harness.step();
        harness.input_mut().events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -10.0),
            modifiers: egui::Modifiers::NONE,
        });
        harness.run_steps(2);
        harness.input_mut().events.push(button(false));
        harness.run_steps(4);
        assert!(
            !shown(&harness, 38),
            "a wheel while held cancels the request"
        );
        // egui's own target, asked in the passes that queue and apply the request, does not
        // write the offset after it.
        harness.state_mut().pull = true;
        show(&mut harness, 12);
        harness.step();
        harness.state_mut().pull = false;
        harness.run_steps(3);
        assert!(
            shown(&harness, 12),
            "a raw scroll_to_rect does not move the view after it"
        );
        // One pass a frame: a request queued in one frame and a wheel in the next: the column
        // cancels it.
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        show(&mut harness, 33);
        harness.step();
        let at = harness.state().clip.center();
        harness
            .input_mut()
            .events
            .push(egui::Event::PointerMoved(at));
        harness.input_mut().events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -10.0),
            modifiers: egui::Modifiers::NONE,
        });
        harness.run_steps(4);
        assert!(!shown(&harness, 33), "the wheel cancels a queued request");
    }

    fn button_at(at: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    /// Review of PR 757 (P1): in a section drawn as its head (the 300 px section in a 244 px
    /// room), Tab from the head's control reaches the body's control, which the head's clip hides:
    /// the body comes whole into view, so the control shows, keeps the focus and paints. Red: no
    /// clearance for a clipped part's control.
    #[test]
    fn tab_into_a_clipped_part_brings_its_unit_into_view() {
        let mut page = Page::new(texts(8));
        page.split = Some(3);
        let mut harness = harness(page, egui::vec2(960.0, 260.0));
        harness
            .state_mut()
            .region
            .navigate(Navigation::Show(Id::new(3usize)));
        harness.run_steps(3);
        harness.get_by_label("Act 3").focus();
        harness.run_steps(3);
        key_press(&mut harness, egui::Key::Tab, egui::Modifiers::NONE);
        let node = harness.get_by_label("Act 3.1");
        let clip = harness.state().clip;
        assert!(node.is_focused(), "Tab reaches the body's control");
        assert!(
            clip.expand(0.5).contains_rect(node.rect()),
            "the body's control {:?} is whole in the room {clip:?}",
            node.rect()
        );
        assert!(
            painted(&harness)
                .iter()
                .any(|text| text.starts_with("Body 3 ")),
            "the body's unit draws"
        );
    }

    /// Review of PR 757 (P1, P2): a queued request belongs to its column. With one pass a frame:
    /// a wheel over another column does not end it; a press held still in its own column makes
    /// it wait (queued first, then held) and it lands after the release; a drag begun in its
    /// column ends it, also while the press is held. Red runs: input read window-wide; no wait for
    /// a held press in the column; a drag that only defers it.
    #[test]
    fn a_queued_request_answers_only_its_own_column() {
        let mut page = Page::new(texts(40));
        page.beside = Some(Rect::NOTHING);
        let mut harness = harness(page, egui::vec2(1200.0, 420.0));
        let show = |harness: &mut Harness<'_, Page>, at: usize| {
            harness
                .state_mut()
                .region
                .navigate(Navigation::Show(Id::new(at)));
        };
        // A wheel over the other column.
        show(&mut harness, 30);
        harness.step();
        let beside = harness.state().beside.expect("the other column").center();
        harness
            .input_mut()
            .events
            .push(egui::Event::PointerMoved(beside));
        harness.input_mut().events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -10.0),
            modifiers: egui::Modifiers::NONE,
        });
        harness.run_steps(3);
        assert!(
            shown(&harness, 30),
            "a wheel over another column leaves the request"
        );
        // Queued, then a press held still in its column: it waits, and lands after the release.
        // A point of the pane's own column (the clip spans both columns).
        let at = harness.state().clip.left_center() + egui::vec2(100.0, 0.0);
        harness
            .input_mut()
            .events
            .push(egui::Event::PointerMoved(at));
        harness.step();
        show(&mut harness, 5);
        harness.step();
        harness.input_mut().events.push(button_at(at, true));
        harness.run_steps(2);
        assert!(
            !shown(&harness, 5),
            "the queued request waits while the press is held"
        );
        harness.input_mut().events.push(button_at(at, false));
        harness.run_steps(3);
        assert!(shown(&harness, 5), "it lands after the release");
        // Queued, then a drag begun in its column: it ends.
        show(&mut harness, 35);
        harness.step();
        harness.input_mut().events.push(button_at(at, true));
        harness.step();
        for step in 1..4 {
            harness.input_mut().events.push(egui::Event::PointerMoved(
                at + egui::vec2(0.0, 12.0 * step as f32),
            ));
            harness.step();
        }
        let end = at + egui::vec2(0.0, 36.0);
        harness.input_mut().events.push(button_at(end, false));
        harness.run_steps(3);
        assert!(
            !shown(&harness, 35),
            "a drag in its column ends the request"
        );
    }

    /// Review of PR 757 (P2): two measured blocks of one region may not share a key.
    #[test]
    #[should_panic(expected = "share a key")]
    fn two_blocks_of_one_key_are_refused() {
        let mut page = Page::new(texts(6));
        page.duplicate = true;
        let _ = harness(page, egui::vec2(960.0, 420.0));
    }
}
