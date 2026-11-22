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
    /// Words drawn as links (`LaidUi::link`), each at its rect: its galley, and the press
    /// target's size (at least 24 px each way, the galley centred in its height).
    links: Vec<(LaidText, Vec2)>,
    /// The laid words a slot draws (`LaidUi::part`), by the slot's place; `None` for a slot the
    /// block fills itself.
    slot_texts: Vec<Option<Arc<Galley>>>,
    /// Each slot's own key, by the slot's place (`Laid::with_slot_keys`): a keyed slot's `Ui`
    /// (and so its controls' ids and their focus) follows its key, not its place.
    slot_keys: Vec<Option<Id>>,
}

impl Laid {
    /// This layout with the words its slots draw (`LaidUi::part`), by the slots' places.
    pub(in crate::design) fn with_slot_texts(mut self, texts: Vec<Option<Arc<Galley>>>) -> Self {
        self.slot_texts = texts;
        self
    }

    /// This layout with a key for each slot that has one, by the slots' places: a control drawn
    /// in a keyed slot keeps its id (and its focus) when an earlier slot comes or goes (an action
    /// hidden before it).
    pub(in crate::design) fn with_slot_keys(mut self, keys: Vec<Option<Id>>) -> Self {
        self.slot_keys = keys;
        self
    }

    /// This layout with `links`, drawn by the block through `LaidUi::link`. Each must lie in the
    /// block.
    pub(in crate::design) fn with_links(mut self, links: Vec<(LaidText, Vec2)>) -> Self {
        debug_assert!(
            links.iter().all(|(text, size)| {
                Rect::from_min_size(egui::Pos2::ZERO, self.size)
                    .expand(0.5)
                    .contains_rect(Rect::from_min_size(text.at, *size))
            }),
            "a recipe lays out every link inside its block"
        );
        self.links = links;
        self
    }

    /// The block's size.
    pub fn size(&self) -> Vec2 {
        self.size
    }

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
            links: Vec::new(),
            slot_texts: Vec::new(),
            slot_keys: Vec::new(),
        }
    }

    /// Its head (its semantic boundary from its top), for a recipe's proofs.
    #[cfg(test)]
    pub(in crate::design) fn head(&self) -> Option<f32> {
        self.head
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

    /// Slot `at`'s size as laid (no width: a part the recipe gave no place). A host lays an
    /// action's words in it before paint (`Action::laid_words`).
    pub fn slot_size(&self, at: usize) -> Vec2 {
        self.slots.get(at).map_or(Vec2::ZERO, Rect::size)
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

    /// Link `at` (`Laid::with_links`): its laid words at their rect, a press target as tall as the
    /// recipe made it, named `name` to a screen reader, as `design::wrapped_link` draws one.
    pub fn link(&mut self, at: usize, name: &str) -> Response {
        // An empty title shows `Untitled`, and says it (review of PR 820).
        let name = if name.trim().is_empty() {
            "Untitled"
        } else {
            name
        };
        let (text, size) = &self.laid.links[at];
        let origin = self.ui.max_rect().min;
        let rect = Rect::from_min_size(origin + text.at.to_vec2(), *size);
        let response = self.ui.interact(
            rect,
            self.ui.id().with(("laid_link", at)),
            egui::Sense::click(),
        );
        let enabled = self.ui.is_enabled();
        response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, enabled, name));
        if self.ui.is_rect_visible(rect) {
            let top = rect.center().y - text.galley.size().y / 2.0;
            self.ui.painter().galley(
                egui::pos2(rect.left(), top),
                text.galley.clone(),
                crate::design::tokens::colors().accent,
            );
            if response.has_focus() {
                crate::design::components::focus_ring(self.ui, rect);
            }
        }
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    }

    /// Slot `at`'s laid words (`Laid::with_slot_texts`) at its rect: a label named `name` that
    /// takes a hover, or a link (a press target at least `XL` each way, the words centred in its
    /// height). The caller adds its hover text.
    pub fn part(&mut self, at: usize, name: &str, link: bool) -> Response {
        let rect = self.slot(at);
        let galley = self.laid.slot_texts[at]
            .clone()
            .expect("a slot with laid words");
        let sense = if link {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        };
        let response = self
            .ui
            .interact(rect, self.ui.id().with(("laid_part", at)), sense);
        let enabled = self.ui.is_enabled();
        let kind = if link {
            egui::WidgetType::Link
        } else {
            egui::WidgetType::Label
        };
        response.widget_info(|| egui::WidgetInfo::labeled(kind, enabled, name));
        if self.ui.is_rect_visible(rect) {
            let top = rect.center().y - galley.size().y / 2.0;
            self.ui.painter().galley(
                egui::pos2(rect.left(), top),
                galley,
                crate::design::tokens::colors().text,
            );
            if link && response.has_focus() {
                crate::design::components::focus_ring(self.ui, rect);
            }
        }
        if link {
            response.on_hover_cursor(egui::CursorIcon::PointingHand)
        } else {
            response
        }
    }

    /// `widget` in slot `at`, at its rect.
    pub fn put(&mut self, at: usize, widget: impl Widget) -> Response {
        let rect = self.slot(at);
        self.ui.put(rect, widget)
    }

    /// `add`'s actions in slot `at`: right to left and centred on their line, `S` apart, as a
    /// list's action place draws them (`action_rows::ActionSlot::child`).
    pub fn actions<R>(&mut self, at: usize, add: impl FnOnce(&mut Ui) -> R) -> R {
        let rect = self.slot(at);
        let mut child = self.ui.new_child(
            UiBuilder::new()
                .id_salt(("slot", at))
                .max_rect(rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        child.spacing_mut().item_spacing.x = crate::design::tokens::S;
        add(&mut child)
    }

    /// `add` in a `Ui` that is slot `at`'s rect, with no item spacing.
    pub fn in_slot<R>(&mut self, at: usize, add: impl FnOnce(&mut Ui) -> R) -> R {
        let rect = self.slot(at);
        // A keyed slot's `Ui` is its key's own within its block, whatever slots come before it;
        // another follows its place.
        let builder = match self.laid.slot_keys.get(at).copied().flatten() {
            Some(key) => UiBuilder::new()
                .id_salt(self.ui.id().with(("slot", key)))
                .global_scope(true),
            None => UiBuilder::new().id_salt(("slot", at)),
        };
        let mut child = self.ui.new_child(builder.max_rect(rect));
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

/// The `Ui` id of the block keyed `key`, in a region or in place (`draw_laid`): its key's own,
/// so its controls keep their ids, and their focus, whichever draws it and whatever draws before
/// it.
pub(in crate::design) fn block_scope(key: Id) -> Id {
    Id::new(("laid_block", key))
}

/// Draw `laid` in place in `ui`, outside a region: its laid size at the cursor, its texts, and
/// `draw`'s controls in its slots (layout 11c A2c). A host whose rows are not yet a region (the
/// Session plan while a row is open) draws its closed rows through the same recipe, so they keep
/// their height and place in both of its modes. The block registers its whole rect as a row
/// (`register_row`: a footer never cuts through it) and returns it.
pub fn draw_laid(ui: &mut Ui, laid: &Laid, draw: impl FnOnce(&mut LaidUi<'_>)) -> Rect {
    // The block's own gaps are laid out inside it (`theme::LINE_GAP`): the caller's scope leaves
    // the column's gap after it.
    let gap = ui.spacing().item_spacing.y;
    ui.spacing_mut().item_spacing.y = 0.0;
    let (rect, _) = ui.allocate_exact_size(laid.size, egui::Sense::hover());
    ui.spacing_mut().item_spacing.y = gap;
    // The block's `Ui` (and so its controls' ids and their focus) is its own, by its key, as a
    // region's block is: two rows under one parent never share an id, and a row keeps its ids
    // whatever rows draw before it (review of PR 840).
    let mut child = ui.new_child(
        UiBuilder::new()
            .id_salt(block_scope(laid.key))
            .global_scope(true)
            .max_rect(rect),
    );
    let _ = render(&mut child, laid, laid.revision, draw);
    crate::design::register_row(ui, rect);
    rect
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
        pane::{BlockRegion, Navigation, RegionState},
        reveal::JumpTo,
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
        /// A typed jump to a block's top, asked once in the pane's column before the region
        /// draws, and one after it.
        jumps: (Option<usize>, Option<usize>),
        /// The least scroll that shows a block (`bring_into_view`), asked once before the
        /// region draws.
        bring: Option<usize>,
        /// What `dm_scrolls` (a retry's rule, PR 780) said in the pane's column in the last pass.
        scrolls: bool,
        /// A block whose control is a multi-line text field that indents (it keeps Tab), its
        /// text and its id.
        field: Option<usize>,
        field_text: String,
        field_id: Option<Id>,
        /// The pane's column hosts reveals (`ContentScroll::hosts_reveals`).
        hosts: bool,
        /// A reveal of a block (`reveal`'s target), asked once in the pane's column with a test
        /// act: `Some(false)` asks (`Reveal::ask`), `Some(true)` asks on a result
        /// (`Reveal::ask_on_result`).
        reveal: crate::design::reveal::Reveal,
        ask: Option<bool>,
        target: usize,
        /// The awaited result comes in the next pass (`Reveal::result_came`).
        came: bool,
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
                jumps: (None, None),
                bring: None,
                scrolls: false,
                field: None,
                field_text: String::new(),
                field_id: None,
                hosts: false,
                reveal: crate::design::reveal::Reveal::default(),
                ask: None,
                target: 0,
                came: false,
            }
        }
    }

    fn harness(page: Page, size: egui::Vec2) -> Harness<'static, Page> {
        use crate::design::test_harness::Themed as _;
        let mut harness = Harness::builder().with_size(size).themed_ui_state(
            |ui, page: &mut Page| {
                let full = ui.available_rect_before_wrap();
                if page.beside.is_some() {
                    let (_, right) = full.split_left_right_at_fraction(0.5);
                    page.beside = Some(right);
                    ui.scope_builder(egui::UiBuilder::new().max_rect(right), |ui| {
                        crate::design::layout::ScrollColumn::new("laid_test_beside").show(
                            ui,
                            |ui| {
                                let _ = ui.button("Beside button");
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
                let scroll = if page.hosts {
                    crate::design::pane::ContentScroll::default().hosts_reveals()
                } else {
                    crate::design::pane::ContentScroll::default()
                };
                let _ = messages.pane_content(ui, scroll, |ui| {
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
                    page.scrolls = crate::design::reveal::dm_scrolls(ui);
                    let (top, pull) = (page.top, page.pull);
                    // A jump asked in this pass before the region draws, or after it.
                    let jumps = std::mem::take(&mut page.jumps);
                    let bring = page.bring.take();
                    let (ask, came, target) =
                        (page.ask.take(), std::mem::take(&mut page.came), page.target);
                    let Page {
                        laid,
                        rows,
                        region,
                        clicked,
                        overflow,
                        stray,
                        field,
                        field_text,
                        field_id,
                        reveal,
                        ..
                    } = page;
                    let top_of = |at: usize| top + laid[..at].iter().map(Laid::height).sum::<f32>();
                    if let Some(at) = jumps.0 {
                        crate::design::reveal::jump(ui, top_of(at), JumpTo::LinkedHeading);
                    }
                    if let Some(at) = bring {
                        let block = Rect::from_min_size(
                            egui::pos2(ui.max_rect().left(), top_of(at)),
                            egui::vec2(ui.max_rect().width(), laid[at].height()),
                        );
                        let _ = crate::design::reveal::bring_into_view(
                            ui,
                            block,
                            JumpTo::OpenMenuControl,
                        );
                    }
                    BlockRegion::new("laid_test", region, rows).show_laid(
                        ui,
                        |at| &laid[at],
                        |ui, at| {
                            if *field == Some(at) {
                                let size = ui.slot(0).size();
                                let edit = ui.in_slot(0, |ui| {
                                    ui.set_min_size(size);
                                    crate::field_id!("laid_test_field")
                                        .once(crate::design::layout::Once::Example)
                                        .multi_line(ui, field_text, |edit| {
                                            edit.desired_rows(1)
                                                .desired_width(size.x)
                                                .lock_focus(true)
                                        })
                                });
                                *field_id = Some(edit.id);
                            } else if ui.put(0, egui::Button::new(format!("Act {at}"))).clicked() {
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
                    if let Some(at) = jumps.1 {
                        crate::design::reveal::jump(ui, top_of(at), JumpTo::LinkedHeading);
                    }
                    // A reveal of the target block, asked with a test act in the pane's column,
                    // and its mark in every pass.
                    if let Some(waits) = ask {
                        let act = crate::design::reveal::Act::test(ui, 0);
                        if waits {
                            reveal.ask_on_result(act, 0, "laid_target");
                        } else {
                            reveal.ask(act, 0, "laid_target");
                        }
                    }
                    if came {
                        reveal.result_came(ui.ctx());
                    }
                    if !laid.is_empty() {
                        let block = Rect::from_min_size(
                            egui::pos2(ui.max_rect().left(), top_of(target)),
                            egui::vec2(ui.max_rect().width(), laid[target].height()),
                        );
                        reveal.mark(ui, 0, "laid_target", block);
                    }
                });
            },
            page,
        );
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
        // Shift+Tab goes back through it, each control whole in view again.
        for slot in (0..7).rev() {
            key_press(&mut harness, egui::Key::Tab, egui::Modifiers::SHIFT);
            // egui moves the focus backwards a frame later; the pane's scroll follows it.
            harness.run_steps(2);
            let label = if slot == 0 {
                "Act 2".to_owned()
            } else {
                format!("Act 2.{slot}")
            };
            let node = harness.get_by_label(&label);
            assert!(node.is_focused(), "Shift+Tab focuses {label}");
            let clip = harness.state().clip;
            assert!(
                clip.expand(0.5).contains_rect(node.rect()),
                "{label} {:?} is whole in the room {clip:?}",
                node.rect()
            );
        }
        // The arrows go down through it the same way.
        for slot in 1..8 {
            key_press(&mut harness, egui::Key::ArrowDown, egui::Modifiers::NONE);
            // egui moves the focus by an arrow at the end of its frame; the scroll follows it.
            harness.run_steps(2);
            let label = format!("Act 2.{slot}");
            let node = harness.get_by_label(&label);
            assert!(node.is_focused(), "ArrowDown focuses {label}");
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
        // The keyboard. Focus in the other column: Page Down there, in the frame that asks or the
        // next, does not end this column's request.
        let key = |harness: &mut Harness<'_, Page>, key, modifiers| {
            for pressed in [true, false] {
                harness.input_mut().events.push(egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed,
                    repeat: false,
                    modifiers,
                });
            }
        };
        harness.get_by_label("Beside button").focus();
        harness.run_steps(2);
        show(&mut harness, 20);
        key(&mut harness, egui::Key::PageDown, egui::Modifiers::NONE);
        harness.run_steps(3);
        assert!(
            shown(&harness, 20),
            "Page Down in another column leaves the request"
        );
        show(&mut harness, 8);
        harness.step();
        key(&mut harness, egui::Key::PageDown, egui::Modifiers::NONE);
        harness.run_steps(3);
        assert!(
            shown(&harness, 8),
            "Page Down in another column leaves a queued request"
        );
        // Focus in this column: a shortcut (Ctrl+S, Ctrl+Page Down) moves nothing and leaves a
        // queued request;
        // Page Down ends a request.
        harness.get_by_label("Act 8").focus();
        harness.run_steps(2);
        show(&mut harness, 30);
        harness.step();
        key(&mut harness, egui::Key::S, egui::Modifiers::CTRL);
        harness.run_steps(3);
        assert!(shown(&harness, 30), "a shortcut leaves a queued request");
        // A key that moves nothing (Escape) leaves it too.
        show(&mut harness, 16);
        harness.step();
        key(&mut harness, egui::Key::Escape, egui::Modifiers::NONE);
        harness.run_steps(3);
        assert!(shown(&harness, 16), "Escape leaves a queued request");
        // A shortcut on a key that would move the view (Ctrl+Page Down: the next tab) too.
        show(&mut harness, 12);
        harness.step();
        key(&mut harness, egui::Key::PageDown, egui::Modifiers::CTRL);
        harness.run_steps(3);
        assert!(
            shown(&harness, 12),
            "a shortcut on a view key leaves a queued request"
        );
        harness.get_by_label("Act 12").focus();
        harness.run_steps(2);
        show(&mut harness, 2);
        key(&mut harness, egui::Key::PageDown, egui::Modifiers::NONE);
        harness.run_steps(3);
        assert!(
            !shown(&harness, 2),
            "Page Down in this column ends the request"
        );
        // Review of PR 787 (P2): the focus moves from the other column into this one in a pass
        // where the other column recorded first: Page Down in the next frame still ends this
        // column's queued request.
        harness.get_by_label("Beside button").focus();
        harness.run_steps(2);
        let here = (0..40)
            .find(|at| shown(&harness, *at))
            .expect("a whole block");
        let far = if here > 20 { 2 } else { 36 };
        show(&mut harness, far);
        harness.get_by_label(&format!("Act {here}")).focus();
        harness.step();
        key(&mut harness, egui::Key::PageDown, egui::Modifiers::NONE);
        harness.run_steps(3);
        assert!(
            !shown(&harness, far),
            "Page Down ends the request of the column the focus moved into"
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

    /// Review of PR 770: a column has one queued request. A typed jump and a measured region's
    /// request asked for one column in one pass: only the newest applies, in either order, and the
    /// same for `bring_into_view`. A wheel over another column keeps a jump and a
    /// `bring_into_view`. Red runs: two queues (the jump outranks the
    /// region's request); the jump's cancel read window-wide.
    #[test]
    fn a_column_has_one_request_whoever_asks_it() {
        let mut page = Page::new(texts(40));
        page.beside = Some(Rect::NOTHING);
        let mut harness = harness(page, egui::vec2(1200.0, 420.0));
        // The jump first, then the region's request: the request applies.
        harness.state_mut().jumps = (Some(10), None);
        harness
            .state_mut()
            .region
            .navigate(Navigation::Show(Id::new(30usize)));
        harness.run_steps(3);
        assert!(
            shown(&harness, 30) && !shown(&harness, 10),
            "the newer region request applies, not the older jump"
        );
        // The region's request first, then the jump: the jump applies (its block at the top).
        harness
            .state_mut()
            .region
            .navigate(Navigation::Show(Id::new(5usize)));
        harness.state_mut().jumps = (None, Some(20));
        harness.run_steps(3);
        assert!(
            shown(&harness, 20) && !shown(&harness, 5),
            "the newer jump applies, not the older request"
        );
        // A jump queued, and a wheel over the other column in the next frame: it applies.
        harness.state_mut().jumps = (Some(2), None);
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
            shown(&harness, 2),
            "a wheel over another column leaves this column's jump"
        );
        // The same for the least scroll that shows a block (`bring_into_view`).
        harness.state_mut().bring = Some(28);
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
            shown(&harness, 28),
            "a wheel over another column leaves this column's bring_into_view"
        );
        // A region's request after it, in one pass, replaces it.
        harness.state_mut().bring = Some(9);
        harness
            .state_mut()
            .region
            .navigate(Navigation::Show(Id::new(36usize)));
        harness.run_steps(3);
        assert!(
            shown(&harness, 36) && !shown(&harness, 9),
            "the newer region request replaces bring_into_view"
        );
    }

    /// One input of the resolver's case table (Sol sweep 2-001).
    #[derive(Clone, Copy, Debug)]
    enum Input {
        Wheel,
        ContentDrag,
        BarDrag,
        Press,
        Key(egui::Key, egui::Modifiers),
        Text,
        Ime,
        /// A key pressed while a press is held in the column (review of PR 757).
        HeldKey(egui::Key),
    }

    /// The resolver's case table (Sol sweep 2-001; reviews of PRs 757 and 770): two columns, a
    /// request queued for column A (the pane; for block 20, away from the blocks Home and End
    /// route to), and each input the cancel rule handles, aimed at
    /// column B (the pointer over it, the focus in it) and then at A. A's request survives every
    /// input at B; every input at A ends it, except a shortcut that moves nothing (Ctrl+S). Each
    /// cancel branch of `take_queued` has a row. A retry's rule (`dm_scrolls`, the row menu's
    /// bring-back of PR 780) is the same rule: in A it says the DM scrolls for exactly the inputs
    /// that end A's request. (A press on A's scroll bar sets A's offset in
    /// egui's own pass, after a queued one, so the bar always wins; its row checks that.)
    ///
    /// Sol 2-011: every kind of pending request runs the same rows (`Kind`): the region's
    /// request, a jump, `bring_into_view`, an asked reveal and one that waits for a result. They
    /// all answer to the one cancel entry point (`reveal::cancel`), so a new kind gets a row here
    /// and must pass every input at A and at B.
    #[test]
    fn the_resolver_cancels_only_on_its_own_columns_input() {
        use egui::{Key, Modifiers};
        let rows = [
            ("wheel", Input::Wheel, true),
            ("content drag", Input::ContentDrag, true),
            ("bar drag", Input::BarDrag, true),
            ("press", Input::Press, true),
            ("Page Up", Input::Key(Key::PageUp, Modifiers::NONE), true),
            (
                "Page Down",
                Input::Key(Key::PageDown, Modifiers::NONE),
                true,
            ),
            ("Home", Input::Key(Key::Home, Modifiers::NONE), true),
            ("End", Input::Key(Key::End, Modifiers::NONE), true),
            ("ArrowUp", Input::Key(Key::ArrowUp, Modifiers::NONE), true),
            (
                "ArrowDown",
                Input::Key(Key::ArrowDown, Modifiers::NONE),
                true,
            ),
            (
                "ArrowLeft",
                Input::Key(Key::ArrowLeft, Modifiers::NONE),
                true,
            ),
            (
                "ArrowRight",
                Input::Key(Key::ArrowRight, Modifiers::NONE),
                true,
            ),
            ("Tab", Input::Key(Key::Tab, Modifiers::NONE), true),
            ("Shift+Tab", Input::Key(Key::Tab, Modifiers::SHIFT), true),
            ("IME", Input::Ime, true),
            ("Ctrl+Home", Input::Key(Key::Home, Modifiers::CTRL), true),
            ("Ctrl+End", Input::Key(Key::End, Modifiers::CTRL), true),
            (
                "Ctrl+ArrowLeft",
                Input::Key(Key::ArrowLeft, Modifiers::CTRL),
                true,
            ),
            (
                "Ctrl+ArrowDown",
                Input::Key(Key::ArrowDown, Modifiers::CTRL),
                true,
            ),
            ("Page Down while held", Input::HeldKey(Key::PageDown), true),
            ("text", Input::Text, true),
            // A key that activates the focused control is a press of it (Sol 2-011).
            ("Enter", Input::Key(Key::Enter, Modifiers::NONE), true),
            ("Space", Input::Key(Key::Space, Modifiers::NONE), true),
            ("Ctrl+S", Input::Key(Key::S, Modifiers::CTRL), false),
        ];
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Kind {
            Region,
            Jump,
            Bring,
            Reveal,
            Waiting,
        }
        let mut wrong = Vec::new();
        for kind in [
            Kind::Region,
            Kind::Jump,
            Kind::Bring,
            Kind::Reveal,
            Kind::Waiting,
        ] {
            let mut page = Page::new(texts(40));
            page.beside = Some(Rect::NOTHING);
            page.hosts = matches!(kind, Kind::Reveal | Kind::Waiting);
            page.target = 20;
            let mut harness = harness(page, egui::vec2(1200.0, 420.0));
            let button = |at: egui::Pos2, pressed| egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Modifiers::NONE,
            };
            // Whether the view is near block 35 now (the request applied, even for one frame).
            let near = |harness: &Harness<'_, Page>| {
                let page = harness.state();
                let target = block_rect(page, 20);
                target.top() < page.clip.bottom() + 100.0
                    && target.bottom() > page.clip.top() - 100.0
            };
            for (name, input, cancels) in rows {
                for in_a in [false, true] {
                    // From the top, nothing pending, the focus and the pointer where the input aims.
                    harness.state_mut().region.navigate(Navigation::Reset);
                    harness.state_mut().reveal = crate::design::reveal::Reveal::default();
                    harness.run_steps(10);
                    let beside = harness.state().beside.expect("the other column");
                    let clip = harness.state().clip;
                    let (point, bar, focus) = if in_a {
                        (
                            clip.left_center() + egui::vec2(100.0, 0.0),
                            egui::pos2(beside.left() - 5.0, clip.top() + 12.0),
                            "Act 0",
                        )
                    } else {
                        (
                            beside.center(),
                            egui::pos2(beside.right() - 5.0, beside.top() + 12.0),
                            "Beside button",
                        )
                    };
                    harness.get_by_label(focus).focus();
                    harness
                        .input_mut()
                        .events
                        .push(egui::Event::PointerMoved(point));
                    harness.run_steps(2);
                    // A's request, queued or asked (one pass a frame: it applies in the next frame).
                    match kind {
                        Kind::Region => harness
                            .state_mut()
                            .region
                            .navigate(Navigation::Show(Id::new(20usize))),
                        Kind::Jump => harness.state_mut().jumps.0 = Some(20),
                        Kind::Bring => harness.state_mut().bring = Some(20),
                        Kind::Reveal => harness.state_mut().ask = Some(false),
                        Kind::Waiting => harness.state_mut().ask = Some(true),
                    }
                    harness.step();
                    // Whether A's request applied in any frame of the input, and whether a retry's rule
                    // (`dm_scrolls`) said the DM scrolls A in any frame.
                    let (mut seen, mut scrolls) = (false, false);
                    let mut observe = |harness: &Harness<'_, Page>| {
                        seen |= near(harness);
                        scrolls |= harness.state().scrolls;
                    };
                    let events = &mut harness.input_mut().events;
                    match input {
                        Input::Wheel => events.push(egui::Event::MouseWheel {
                            unit: egui::MouseWheelUnit::Point,
                            delta: egui::vec2(0.0, -10.0),
                            modifiers: Modifiers::NONE,
                        }),
                        Input::ContentDrag | Input::BarDrag => {
                            let from = if matches!(input, Input::BarDrag) {
                                bar
                            } else {
                                point
                            };
                            events.push(egui::Event::PointerMoved(from));
                            events.push(button(from, true));
                            harness.step();
                            observe(&harness);
                            for step in 1..4 {
                                let to = from + egui::vec2(0.0, 12.0 * step as f32);
                                harness
                                    .input_mut()
                                    .events
                                    .push(egui::Event::PointerMoved(to));
                                harness.step();
                                observe(&harness);
                            }
                            harness
                                .input_mut()
                                .events
                                .push(button(from + egui::vec2(0.0, 36.0), false));
                        }
                        Input::Press => {
                            events.push(button(point, true));
                            events.push(button(point, false));
                        }
                        Input::Key(key, modifiers) => {
                            for pressed in [true, false] {
                                events.push(egui::Event::Key {
                                    key,
                                    physical_key: None,
                                    pressed,
                                    repeat: false,
                                    modifiers,
                                });
                            }
                        }
                        Input::Text => events.push(egui::Event::Text("x".to_owned())),
                        Input::HeldKey(key) => {
                            events.push(button(point, true));
                            harness.step();
                            observe(&harness);
                            for pressed in [true, false] {
                                harness.input_mut().events.push(egui::Event::Key {
                                    key,
                                    physical_key: None,
                                    pressed,
                                    repeat: false,
                                    modifiers: Modifiers::NONE,
                                });
                            }
                            harness.step();
                            observe(&harness);
                            harness.input_mut().events.push(button(point, false));
                        }
                        Input::Ime => {
                            events.push(egui::Event::Ime(egui::ImeEvent::Commit("x".to_owned())));
                        }
                    }
                    for _ in 0..3 {
                        harness.step();
                        observe(&harness);
                    }
                    // A waiting reveal's result comes after the input.
                    if kind == Kind::Waiting {
                        harness.state_mut().came = true;
                        for _ in 0..8 {
                            harness.step();
                            observe(&harness);
                        }
                    }
                    // Survived: its block is whole in view. Ended: the view came nowhere near its block
                    // in any frame (a request that applied for one frame, then was scrolled past by the
                    // input, was not ended).
                    let expected = !(in_a && cancels);
                    let survived = if expected { shown(&harness, 20) } else { seen };
                    if kind == Kind::Region && scrolls == expected {
                        wrong.push(format!(
                            "{name} at {}: dm_scrolls {scrolls}, expected {}",
                            if in_a { "A" } else { "B" },
                            !expected
                        ));
                    }
                    if survived != expected {
                        wrong.push(format!(
                            "{kind:?}: {name} at {}: survived {survived}, expected {expected}",
                            if in_a { "A" } else { "B" }
                        ));
                    }
                }
            }
        }
        assert!(wrong.is_empty(), "the case table: {wrong:#?}");
    }

    /// Two columns, one inside the other: a jump for the outer one, asked once.
    struct Nested {
        jump: bool,
        /// A jump to the outer column's top, asked once.
        top: bool,
        /// The outer column's view and the inner column's, in the last pass.
        outer: Rect,
        inner: Rect,
    }

    /// Review of PR 757 (P2, P3): a key or a wheel belongs to the innermost column, not to every
    /// column whose rect holds its control or the pointer. A jump queued for the outer column
    /// survives each input aimed at the inner column (the focus on its control, the pointer over
    /// it) and ends on the same input aimed at the outer column. Red: ownership by rect.
    #[test]
    fn a_nested_column_owns_its_own_keys() {
        use crate::design::test_harness::Themed as _;
        use egui::{Key, Modifiers};
        use egui_kittest::kittest::Queryable;
        let mut harness = Harness::builder()
            .with_size(egui::vec2(600.0, 340.0))
            .themed_ui_state(
                |ui, nested: &mut Nested| {
                    crate::design::layout::ScrollColumn::new("laid_test_outer")
                        .max_height(300.0)
                        .show(ui, |ui| {
                            nested.outer = ui.clip_rect();
                            if std::mem::take(&mut nested.top) {
                                crate::design::reveal::jump(
                                    ui,
                                    ui.cursor().top(),
                                    JumpTo::LinkedHeading,
                                );
                            }
                            let _ = ui.button("Outer button");
                            for line in 0..80 {
                                if line == 4 {
                                    crate::design::layout::ScrollColumn::new("laid_test_inner")
                                        .max_height(100.0)
                                        .show(ui, |ui| {
                                            nested.inner = ui.clip_rect();
                                            let _ = ui.button("Inner button");
                                            for inner in 0..20 {
                                                let _ = ui.label(format!("Inner {inner}"));
                                            }
                                        });
                                }
                                if line == 60 && std::mem::take(&mut nested.jump) {
                                    crate::design::reveal::jump(
                                        ui,
                                        ui.cursor().top(),
                                        JumpTo::LinkedHeading,
                                    );
                                }
                                let _ = ui.label(format!("Outer {line}"));
                            }
                        });
                },
                Nested {
                    jump: false,
                    top: false,
                    outer: Rect::NOTHING,
                    inner: Rect::NOTHING,
                },
            );
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        harness.run_steps(3);
        let jumped = |harness: &Harness<'_, Nested>| {
            harness
                .query_by_label("Outer 60")
                .is_some_and(|node| harness.state().outer.contains_rect(node.rect()))
        };
        let rows = [
            ("wheel", None, true),
            // Review of PR 787 (P2): the pointer moves from the inner column to the outer one in
            // the frame of the wheel.
            ("wheel, moved in from the inner column", None, false),
            ("Page Down", Some((Key::PageDown, Modifiers::NONE)), true),
            ("ArrowDown", Some((Key::ArrowDown, Modifiers::NONE)), true),
            ("Ctrl+Home", Some((Key::Home, Modifiers::CTRL)), true),
            ("Tab", Some((Key::Tab, Modifiers::NONE)), true),
        ];
        let mut wrong = Vec::new();
        for (name, key, plain) in rows {
            for at_inner in [true, false] {
                if !plain && at_inner {
                    continue;
                }
                // From the outer column's top, the focus and the pointer where the input aims.
                harness.run_steps(10);
                harness.state_mut().top = true;
                harness.run_steps(4);
                let (focus, point) = if at_inner {
                    ("Inner button", harness.state().inner.center())
                } else {
                    (
                        "Outer button",
                        harness.state().outer.left_bottom() + egui::vec2(40.0, -20.0),
                    )
                };
                harness.get_by_label(focus).focus();
                // The moved row starts over the inner column.
                let start = if plain {
                    point
                } else {
                    harness.state().inner.center()
                };
                harness
                    .input_mut()
                    .events
                    .push(egui::Event::PointerMoved(start));
                harness.run_steps(2);
                harness.state_mut().jump = true;
                harness.step();
                if !plain {
                    harness
                        .input_mut()
                        .events
                        .push(egui::Event::PointerMoved(point));
                }
                match key {
                    None => harness.input_mut().events.push(egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, -10.0),
                        modifiers: Modifiers::NONE,
                    }),
                    Some((key, modifiers)) => {
                        for pressed in [true, false] {
                            harness.input_mut().events.push(egui::Event::Key {
                                key,
                                physical_key: None,
                                pressed,
                                repeat: false,
                                modifiers,
                            });
                        }
                    }
                }
                // Whether the jump applied in any frame (the input itself may move the column after
                // it, as a wheel does once the jump moved the inner column from under the pointer).
                let mut survived = false;
                for _ in 0..3 {
                    harness.step();
                    survived |= jumped(&harness);
                }
                if survived != at_inner {
                    wrong.push(format!(
                        "{name} at the {} column: the outer jump survived {survived}",
                        if at_inner { "inner" } else { "outer" }
                    ));
                }
            }
        }
        assert!(wrong.is_empty(), "nested columns: {wrong:#?}");
    }

    /// Layout 11c, A1c (the v2 review's item 3): keyboard routing across masked blocks. From the
    /// last whole block, Tab and ArrowDown go to the next block, which is not drawn: it comes
    /// whole into view and its control takes the focus. From the first whole block, Shift+Tab
    /// and ArrowUp go to the block before it. End and Home go to the last and the first block.
    /// An older request and a Tab in one pass: the Tab cancels the older one, and its own lands.
    /// Reds: no routing; the key's own request cancelled with the older ones.
    #[test]
    fn keys_go_across_masked_blocks() {
        use egui::{Key, Modifiers};
        let mut harness = harness(Page::new(texts(40)), egui::vec2(960.0, 420.0));
        let edge = |harness: &Harness<'_, Page>| {
            let whole: Vec<usize> = (0..40).filter(|at| shown(harness, *at)).collect();
            (whole[0], whole[whole.len() - 1])
        };
        let focused = |harness: &Harness<'_, Page>, at: usize| {
            harness
                .query_by_label(&format!("Act {at}"))
                .is_some_and(|node| node.is_focused())
        };
        let press = |harness: &mut Harness<'_, Page>, key, modifiers| {
            for pressed in [true, false] {
                harness.input_mut().events.push(egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed,
                    repeat: false,
                    modifiers,
                });
            }
            harness.run_steps(6);
        };
        let mut wrong = Vec::new();
        let mut check = |harness: &Harness<'_, Page>, name: &str, at: usize| {
            if !(focused(harness, at) && shown(harness, at)) {
                wrong.push(format!(
                    "{name}: Act {at} focused {} and whole {}",
                    focused(harness, at),
                    shown(harness, at)
                ));
            }
        };
        // Down from the last whole block, by Tab and by ArrowDown.
        for (name, key) in [("Tab", Key::Tab), ("ArrowDown", Key::ArrowDown)] {
            let (_, last) = edge(&harness);
            harness.get_by_label(&format!("Act {last}")).focus();
            harness.run_steps(2);
            press(&mut harness, key, Modifiers::NONE);
            check(&harness, name, last + 1);
        }
        // Up from the first whole block, by Shift+Tab and by ArrowUp.
        for (name, key, modifiers) in [
            ("Shift+Tab", Key::Tab, Modifiers::SHIFT),
            ("ArrowUp", Key::ArrowUp, Modifiers::NONE),
        ] {
            let (first, _) = edge(&harness);
            harness.get_by_label(&format!("Act {first}")).focus();
            harness.run_steps(2);
            press(&mut harness, key, modifiers);
            check(&harness, name, first - 1);
        }
        // End and Home.
        press(&mut harness, Key::End, Modifiers::NONE);
        check(&harness, "End", 39);
        press(&mut harness, Key::Home, Modifiers::NONE);
        check(&harness, "Home", 0);
        // An older request and a Tab in one pass.
        let (_, last) = edge(&harness);
        harness.get_by_label(&format!("Act {last}")).focus();
        harness.run_steps(2);
        harness
            .state_mut()
            .region
            .navigate(Navigation::Show(Id::new(30usize)));
        press(&mut harness, Key::Tab, Modifiers::NONE);
        check(&harness, "Tab over an older request", last + 1);
        if shown(&harness, 30) {
            wrong.push("the older request survived the Tab".to_owned());
        }
        // Review of PR 787 (P2): the DM's wheel cancels a routed Tab's request, and its focus
        // with it: the target that the wheel brings into view does not take the focus.
        let (_, last) = edge(&harness);
        harness.get_by_label(&format!("Act {last}")).focus();
        harness.run_steps(2);
        for pressed in [true, false] {
            harness.input_mut().events.push(egui::Event::Key {
                key: Key::Tab,
                physical_key: None,
                pressed,
                repeat: false,
                modifiers: Modifiers::NONE,
            });
        }
        harness.step();
        let at = harness.state().clip.center();
        harness
            .input_mut()
            .events
            .push(egui::Event::PointerMoved(at));
        harness.input_mut().events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -120.0),
            modifiers: Modifiers::NONE,
        });
        harness.run_steps(8);
        if focused(&harness, last + 1) {
            wrong.push(format!(
                "a cancelled Tab's target Act {} took the focus (whole {})",
                last + 1,
                shown(&harness, last + 1)
            ));
        }
        assert!(wrong.is_empty(), "keyboard routing: {wrong:#?}");
        // Review of PR 787 (P2): a focused text field keeps its keys. In a field that indents, at
        // the drawn edge, Home, End, the arrows and Tab move its caret or indent: the focus stays
        // in it and the view stays.
        let mut harness = self::harness(Page::new(texts(40)), egui::vec2(960.0, 420.0));
        let (_, last) = edge(&harness);
        harness.state_mut().field = Some(last);
        harness.run_steps(2);
        let field = harness.state().field_id.expect("the field");
        harness.ctx.memory_mut(|memory| memory.request_focus(field));
        harness.run_steps(2);
        for (name, key, modifiers) in [
            ("Home", Key::Home, Modifiers::NONE),
            ("End", Key::End, Modifiers::NONE),
            ("ArrowDown", Key::ArrowDown, Modifiers::NONE),
            ("ArrowUp", Key::ArrowUp, Modifiers::NONE),
            ("Tab", Key::Tab, Modifiers::NONE),
        ] {
            press(&mut harness, key, modifiers);
            let kept = harness.ctx.memory(|memory| memory.has_focus(field));
            let page = harness.state();
            let whole = page.clip.expand(0.5).contains_rect(block_rect(page, last));
            if !kept || !whole {
                wrong.push(format!(
                    "{name} in a text field: the field kept the focus {kept}, its block whole {whole}"
                ));
            }
        }
        assert!(
            wrong.is_empty(),
            "keyboard routing in a text field: {wrong:#?}"
        );
    }
}
