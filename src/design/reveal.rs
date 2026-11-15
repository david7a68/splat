//! One reveal part (mechanism M6, ruling R34): after an act or an open, the least scroll that
//! brings its target (an opened form, an act's result and its next action) whole into view.
//!
//! - **Asked at the act.** [`Reveal::ask`] takes an [`Act`], which only a press or a key of this
//!   pass gives ([`Act::clicked`]); a new act replaces the reveal before it. A draw path cannot renew a reveal: it has no
//!   act, and asking again with the same act is no new request.
//! - **Runs in the scroll area that owns the target.** The target says where it is with
//!   [`Reveal::mark`] after it is drawn. The part finds the innermost scroll area of the target's
//!   `Ui` (egui's `UiStack`) and resolves against that area's own view, recorded by
//!   `ScrollColumn` when its content starts ([`region`]); never against a clip that a collapsing
//!   body shortens for a while. The scroll is the area's offset for its next pass, written by
//!   that `ScrollColumn` only ([`take_offset`]): no egui scroll request, so a nested area cannot
//!   take it, and nothing is animated.
//! - **Ends for good** ([`End`]): the target whole in view; a frame with no progress; a target
//!   taller than the view after one edge scroll (an opened form shows its top, an act's result
//!   its end, where its action is: Q2.1); any input of the DM's (a wheel, a press or drag, a key,
//!   typed or pasted text, IME), observed at every mark even after it ended; the owner's end
//!   ([`Reveal::end`], a frame that did not draw the target, another visit); and a bound of [`PASSES`] scroll frames inside an absolute deadline.
//! - **A resize** brings back only a reveal that ended `Whole`, while the DM has given no input
//!   since its act: a window resize is not the DM's act on the content (the import form at 960,
//!   1200 and 1920 px). It never brings back one that ended for any other reason.
//!
//! One owner holds a [`Reveal`] (the open form, the plan act's journey); its end event is the
//! owner's. Nothing repaints for a reveal once it has ended, and an ended reveal costs one
//! comparison per mark.
use std::hash::Hash;

use eframe::egui::{self, Id, Rect, Response, Ui, Vec2};

/// A reveal's bound: frames in which it scrolls.
pub const PASSES: u8 = 4;

/// The frames, from its first mark, after which a running reveal ends whatever happened.
const DEADLINE: u64 = 2 * PASSES as u64;

/// The act that may ask for a reveal: a press or a key of this pass. Its frame and widget are
/// the reveal's generation, so a pass drawn again in the same frame asks for nothing new.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Act {
    /// The frame of the act; `None` only for a test's act made outside a frame.
    frame: Option<u64>,
    id: Id,
}

impl Act {
    /// The act of a control the DM pressed in this pass.
    pub fn clicked(response: &Response) -> Option<Self> {
        response.clicked().then(|| Self {
            frame: Some(response.ctx.cumulative_frame_nr()),
            id: response.id,
        })
    }

    /// An act outside any frame, for a test that opens a form before its first frame.
    #[cfg(test)]
    pub(crate) fn test(salt: impl Hash) -> Self {
        Self {
            frame: None,
            id: Id::new(("reveal_test_act", salt)),
        }
    }

    /// An act in frame `frame`, for a test.
    #[cfg(test)]
    pub(crate) fn test_at(salt: impl Hash, frame: u64) -> Self {
        Self {
            frame: Some(frame),
            id: Id::new(("reveal_test_act", salt)),
        }
    }
}

/// What the target is, which decides the edge that shows when it is taller than the view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Want {
    /// An opened form: its top shows.
    Open,
    /// An act's result and the next action it makes possible: its end shows.
    #[allow(dead_code, reason = "step 2's callers (lane B6) ask for a result")]
    Result,
}

/// Why a reveal ended. Every end is for good, except `Whole` after a resize (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    Whole,
    NoProgress,
    TooTall,
    ByTheDm,
    OwnerGone,
    Bound,
}

#[derive(Clone, Copy, Debug)]
enum Phase {
    Running {
        /// Scroll frames left.
        left: u8,
        /// The absolute frame after which it ends: `DEADLINE` frames after the act (a target
        /// that never shows ages out), or after the first mark for a test's act or a resize.
        /// Never reset in draw.
        deadline: Option<u64>,
        /// The target's top and the frame of the last scroll asked for.
        asked: Option<(f32, u64)>,
        /// The target is taller than the view: one edge scroll, then `TooTall`.
        edge: bool,
    },
    Ended(End),
}

impl Phase {
    /// A running reveal whose deadline is `deadline`, or `DEADLINE` frames after its first mark.
    fn fresh(deadline: Option<u64>) -> Self {
        Self::Running {
            left: PASSES,
            deadline,
            asked: None,
            edge: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Run {
    act: Act,
    visit: Id,
    want: Want,
    phase: Phase,
    /// The DM gave input after the act.
    touched: bool,
    /// The last frame that marked the target.
    frame: Option<u64>,
    /// The view's size and whether the target was whole in it, at the last mark.
    seen: Option<(Vec2, bool)>,
}

/// One owner's reveal. The owner holds it, asks at its act and marks the target while drawing;
/// its own end event calls [`Reveal::end`].
#[derive(Debug, Default)]
pub struct Reveal {
    run: Option<Run>,
}

impl Reveal {
    /// Ask at `act` for the target of `visit` (the owner's object and what it shows now, such as
    /// the remote campaign and the campaign open) to come into view. The same act again is no
    /// new request; another act replaces the reveal before it.
    pub fn ask(&mut self, act: Act, visit: impl Hash, want: Want) {
        if self.run.is_some_and(|run| run.act == act) {
            return;
        }
        self.run = Some(Run {
            act,
            visit: Id::new(visit),
            want,
            phase: Phase::fresh(act.frame.map(|frame| frame + DEADLINE)),
            touched: false,
            frame: None,
            seen: None,
        });
    }

    /// The owner ended (its form closed, its object went away): the reveal ends for good.
    pub fn end(&mut self) {
        if let Some(run) = &mut self.run {
            run.phase = Phase::Ended(End::OwnerGone);
        }
    }

    /// Whether it runs.
    #[cfg(test)]
    pub(crate) fn running(&self) -> bool {
        self.run
            .is_some_and(|run| matches!(run.phase, Phase::Running { .. }))
    }

    /// Why it ended; `None` while it runs or before any act.
    #[cfg(test)]
    pub(crate) fn ended(&self) -> Option<End> {
        match self.run?.phase {
            Phase::Ended(end) => Some(end),
            Phase::Running { .. } => None,
        }
    }

    /// The target of `visit` is drawn at `target` (screen rect) in `ui`. One step: it observes
    /// the DM's input and the owner's visit, then scrolls the owning area the least, or ends.
    pub fn mark(&mut self, ui: &Ui, visit: impl Hash, target: Rect) {
        let Some(run) = &mut self.run else {
            return;
        };
        // Ended for a reason a resize cannot undo: nothing to observe.
        if matches!(run.phase, Phase::Ended(end) if end != End::Whole) {
            return;
        }
        let frame = ui.ctx().cumulative_frame_nr();
        // A frame that did not draw the target (the page left), or another visit (another
        // campaign open): the owner's view of the target ended.
        if run.frame.is_some_and(|last| frame > last + 1) || Id::new(visit) != run.visit {
            run.phase = Phase::Ended(End::OwnerGone);
            return;
        }
        run.frame = Some(frame);
        // The act's own frame holds the act's own input (its press and release).
        if run.act.frame != Some(frame) && dm_input(ui) {
            run.touched = true;
            run.phase = Phase::Ended(End::ByTheDm);
            return;
        }
        let area = owning_area(ui);
        let view = area.view;
        let whole = target.top() >= view.top() - 0.5 && target.bottom() <= view.bottom() + 0.5;
        if let Some((size, was_whole)) = run.seen
            && (size - view.size()).length() > 0.5
            && was_whole
            && matches!(run.phase, Phase::Ended(End::Whole))
            && !run.touched
        {
            run.phase = Phase::fresh(None);
        }
        run.seen = Some((view.size(), whole));
        let Phase::Running {
            left,
            deadline,
            asked,
            edge,
        } = &mut run.phase
        else {
            return;
        };
        let deadline = *deadline.get_or_insert(frame + DEADLINE);
        let end = if whole {
            Some(End::Whole)
        } else if frame > deadline {
            Some(End::Bound)
        } else {
            match *asked {
                // A pass drawn again in the same frame asks again (a pass drawn again may drop
                // the offset); a new frame judges the last scroll.
                Some((_, at)) if at != frame && *edge => Some(End::TooTall),
                Some((top, at)) if at != frame && (top - target.top()).abs() < 0.5 => {
                    Some(End::NoProgress)
                }
                Some((_, at)) if at == frame => None,
                _ => {
                    *left = left.saturating_sub(1);
                    None
                }
            }
        };
        if let Some(end) = end {
            run.phase = Phase::Ended(end);
            return;
        }
        let tall = target.height() > view.height();
        *edge |= tall;
        let delta = if tall {
            match run.want {
                Want::Open => target.top() - view.top(),
                Want::Result => target.bottom() - view.bottom(),
            }
        } else if target.bottom() > view.bottom() {
            target.bottom() - view.bottom()
        } else {
            target.top() - view.top()
        };
        *asked = Some((target.top(), frame));
        let last = *left == 0 && !*edge;
        area.scroll(ui, delta, target, tall.then_some(run.want));
        if last {
            run.phase = Phase::Ended(End::Bound);
        } else {
            ui.ctx().request_repaint();
        }
    }
}

/// Any input of the DM's in this pass: a wheel, a press or drag, a key, typed, pasted or composed
/// text. Read before a child can consume a key (`mark` runs at the target, but egui keeps typed
/// text and wheel deltas in the pass's input).
fn dm_input(ui: &Ui) -> bool {
    ui.input(|input| {
        input.raw_scroll_delta != Vec2::ZERO
            || input.smooth_scroll_delta != Vec2::ZERO
            || input.pointer.any_pressed()
            || input.pointer.is_decidedly_dragging()
            || input.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::Text(_)
                        | egui::Event::Paste(_)
                        | egui::Event::Ime(_)
                        | egui::Event::MouseWheel { .. }
                        | egui::Event::Key { pressed: true, .. }
                )
            })
    })
}

/// A `ScrollColumn`'s facts in one pass, recorded where its content starts.
#[derive(Clone, Copy, Debug)]
struct Region {
    pass: u64,
    /// The column's persistent key, which [`take_offset`] reads.
    key: Id,
    /// The visible view, in screen points: the area's viewport inside its clip.
    view: Rect,
    /// The vertical offset this pass draws with.
    offset: f32,
}

fn region_id(content: Id) -> Id {
    content.with("design_reveal_region")
}

fn offset_id(key: Id) -> Id {
    key.with("design_reveal_offset")
}

/// Run `add` as the content of the scroll column `key`, whose content `Ui` is `ui` and whose
/// viewport (egui's, in content points) is `viewport`. `ScrollColumn` calls it; only it may.
pub(super) fn region<R>(ui: &mut Ui, key: Id, viewport: Rect, add: impl FnOnce(&mut Ui) -> R) -> R {
    let origin = ui.max_rect().min.to_vec2();
    let view = viewport.translate(origin).intersect(ui.clip_rect());
    let region = Region {
        pass: ui.ctx().cumulative_pass_nr(),
        key,
        view,
        offset: viewport.top(),
    };
    let id = region_id(ui.id());
    ui.ctx().data_mut(|data| data.insert_temp(id, region));
    add(ui)
}

/// The offset a reveal wrote for the column `key` in the last frame, taken once. A column that
/// was not drawn in the frame after it was written never takes it.
pub(super) fn take_offset(ui: &Ui, key: Id) -> Option<f32> {
    let frame = ui.ctx().cumulative_frame_nr();
    ui.ctx()
        .data_mut(|data| data.remove_temp::<(f32, u64)>(offset_id(key)))
        .filter(|(_, written)| frame <= written + 1)
        .map(|(offset, _)| offset)
}

/// The scroll area that owns a target, and how it scrolls.
struct Area {
    view: Rect,
    /// The owning `ScrollColumn`, with its offset this pass; `None`: an area the part does not
    /// own (a raw `egui::ScrollArea`, `ScrollColumn::show_rows`) or none.
    column: Option<(Id, f32)>,
}

impl Area {
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn scroll(&self, ui: &Ui, delta: f32, target: Rect, edge: Option<Want>) {
        match self.column {
            Some((key, offset)) => {
                let frame = ui.ctx().cumulative_frame_nr();
                let to = (offset + delta).max(0.0);
                ui.ctx()
                    .data_mut(|data| data.insert_temp(offset_id(key), (to, frame)));
            }
            // An area the part does not own takes egui's request at its end: the innermost
            // area, which is this one. Never animated.
            None => {
                let (rect, align) = match edge {
                    Some(Want::Open) => (
                        Rect::from_min_size(target.min, egui::vec2(target.width(), 1.0)),
                        Some(egui::Align::TOP),
                    ),
                    Some(Want::Result) => (
                        Rect::from_min_max(
                            egui::pos2(target.left(), target.bottom() - 1.0),
                            target.max,
                        ),
                        Some(egui::Align::BOTTOM),
                    ),
                    None => (target, None),
                };
                ui.scroll_to_rect_animation(rect, align, egui::style::ScrollAnimation::none());
            }
        }
    }
}

/// The innermost scroll area of `ui`: its view, and the column if a `ScrollColumn` drew it in
/// this pass. Outside any scroll area, the clip.
fn owning_area(ui: &Ui) -> Area {
    let pass = ui.ctx().cumulative_pass_nr();
    let innermost = ui
        .stack()
        .iter()
        .find(|frame| frame.kind() == Some(egui::UiKind::ScrollArea))
        .map(|frame| frame.id);
    let region = innermost.and_then(|id| {
        ui.ctx()
            .data(|data| data.get_temp::<Region>(region_id(id)))
            .filter(|region| region.pass == pass)
    });
    match region {
        Some(region) => Area {
            view: region.view,
            column: Some((region.key, region.offset)),
        },
        None => Area {
            view: ui.clip_rect(),
            column: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable;

    const WIDTH: f32 = 400.0;
    const HEIGHT: f32 = 300.0;

    /// What a test page draws: a column with `above` px of room, the target (`Target top`, a
    /// block of `tall` px, `Target end`), then 900 px; whether the target is drawn; and a draw
    /// path that asks again on every frame with the act it kept.
    struct Page {
        reveal: Reveal,
        act: Act,
        want: Want,
        above: f32,
        tall: f32,
        drawn: bool,
        renew: bool,
        /// The target inside a nested column 120 px high, at the outer column's top.
        nested: bool,
    }

    impl Page {
        fn new() -> Self {
            Self {
                reveal: Reveal::default(),
                act: Act::test(1),
                want: Want::Open,
                above: 900.0,
                tall: 40.0,
                drawn: true,
                renew: false,
                nested: false,
            }
        }
    }

    fn room(ui: &mut Ui, height: f32) {
        ui.allocate_exact_size(egui::vec2(1.0, height), egui::Sense::hover());
    }

    fn target(ui: &mut Ui, page: &mut Page) {
        if page.renew {
            page.reveal.ask(page.act, 0, page.want);
        }
        if page.drawn {
            let tall = page.tall;
            let rect = ui
                .vertical(|ui| {
                    ui.label("Target top");
                    room(ui, tall);
                    ui.label("Target end");
                })
                .response
                .rect;
            page.reveal.mark(ui, 0, rect);
        }
    }

    fn harness(mut page: Page) -> egui_kittest::Harness<'static, Page> {
        page.reveal.ask(page.act, 0, page.want);
        egui_kittest::Harness::builder()
            .with_size(egui::vec2(WIDTH, HEIGHT))
            .build_ui_state(
                |ui, page: &mut Page| {
                    super::super::ScrollColumn::new("outer").show(ui, |ui| {
                        ui.label("Outer top");
                        if page.nested {
                            super::super::ScrollColumn::new("inner")
                                .max_height(120.0)
                                .min_scrolled_height(120.0)
                                .show(ui, |ui| {
                                    room(ui, page.above);
                                    target(ui, page);
                                    room(ui, 900.0);
                                });
                        } else {
                            room(ui, page.above);
                            target(ui, page);
                        }
                        room(ui, 900.0);
                    });
                },
                page,
            )
    }

    fn top(h: &egui_kittest::Harness<'_, Page>, name: &str) -> f32 {
        h.get_by_label(name).rect().top()
    }

    fn wheel(h: &mut egui_kittest::Harness<'_, Page>, by: f32) {
        h.event(egui::Event::PointerMoved(egui::pos2(WIDTH / 2.0, 100.0)));
        h.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, by),
            modifiers: egui::Modifiers::NONE,
        });
    }

    /// A draw path that asks on every frame with the act it kept asks for nothing new: an
    /// ended reveal stays ended after the DM scrolls the target away.
    #[test]
    fn asking_again_with_the_same_act_is_no_new_request() {
        let mut page = Page::new();
        page.renew = true;
        let mut h = harness(page);
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Whole));
        wheel(&mut h, 600.0);
        h.run_steps(8);
        let away = top(&h, "Target top");
        h.run_steps(8);
        assert!(
            away > HEIGHT && (top(&h, "Target top") - away).abs() < 0.5,
            "a renewed ask brought the target back: {away} then {}",
            top(&h, "Target top")
        );
        assert_eq!(h.state().reveal.ended(), Some(End::ByTheDm));
        // Another act is a new request, and it replaces the old one.
        h.state_mut().act = Act::test(2);
        h.run_steps(8);
        assert!(
            (0.0..HEIGHT).contains(&top(&h, "Target top")),
            "a new act did not reveal the target"
        );
    }

    /// A frame that does not draw the target ends the reveal: when the target comes back, it
    /// is not scrolled to again. A target that never shows ages out from its act.
    #[test]
    fn a_missing_target_ends_the_reveal() {
        let mut h = harness(Page::new());
        h.step();
        h.state_mut().drawn = false;
        h.step();
        // It comes back further down, out of view.
        h.state_mut().drawn = true;
        h.state_mut().above += 1000.0;
        h.step();
        assert!(
            top(&h, "Target top") > HEIGHT,
            "the target came back in view"
        );
        let back = top(&h, "Target top");
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::OwnerGone));
        assert!(
            (top(&h, "Target top") - back).abs() < 0.5,
            "a target that left a frame was scrolled to again: {back} then {}",
            top(&h, "Target top")
        );
        // Asked in frame 0, first drawn 20 frames later: it ended with its deadline.
        let mut page = Page::new();
        page.act = Act::test_at(3, 0);
        page.drawn = false;
        let mut h = harness(page);
        h.run_steps(20);
        h.state_mut().drawn = true;
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Bound));
        assert!(
            top(&h, "Target top") > HEIGHT,
            "a late target was scrolled to"
        );
    }

    /// The target in a nested column scrolls that column only: the outer column stays.
    #[test]
    fn a_nested_column_owns_its_target() {
        let mut page = Page::new();
        page.nested = true;
        page.above = 400.0;
        let mut h = harness(page);
        let outer = top(&h, "Outer top");
        h.run_steps(8);
        assert_eq!(h.state().reveal.ended(), Some(End::Whole));
        let (start, end) = (top(&h, "Target top"), h.get_by_label("Target end").rect());
        assert!(
            start >= outer && end.bottom() <= outer + 140.0,
            "the target is not whole in its column: {start} to {end:?}"
        );
        assert!(
            (top(&h, "Outer top") - outer).abs() < 0.5,
            "the outer column moved: {outer} then {}",
            top(&h, "Outer top")
        );
    }

    /// Taller than the view: an opened form shows its top, an act's result its end, once.
    #[test]
    fn a_tall_target_shows_the_edge_its_want_names() {
        for (want, name) in [(Want::Open, "Target top"), (Want::Result, "Target end")] {
            let mut page = Page::new();
            page.want = want;
            page.tall = 600.0;
            let mut h = harness(page);
            h.run_steps(8);
            assert_eq!(h.state().reveal.ended(), Some(End::TooTall), "{want:?}");
            let at = h.get_by_label(name).rect();
            assert!(
                at.top() >= -0.5 && at.bottom() <= HEIGHT + 0.5,
                "{want:?}: {name} at {at:?}"
            );
        }
    }
}
