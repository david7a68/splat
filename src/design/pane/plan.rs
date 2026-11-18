//! The whole-block plan of a pane's body (migration-32, layout step 3a): a closed list of blocks,
//! their extents measured once per change of the content or of what it is measured in, an index
//! of their tops, and the eligibility mask (which blocks are whole in the native view at an
//! offset). Pure geometry: no `Ui`, no viewport, no focus, no caller; layout step 3b draws from
//! it.
//!
//! The rules it keeps (layout design §3):
//!
//! - **I1, whole blocks.** A block that fits the view is either wholly eligible (it paints, takes
//!   a press and shows to a screen reader) or not eligible at all: never a fragment.
//! - **I2, the native range.** The mask never shortens the range: the content's extent is every
//!   block's whole height, and at the largest offset the last block that fits is whole.
//! - **I12, quiet work.** An unchanged plan is not measured again: a frame with the same content
//!   and the same measuring environment builds nothing, and a mask reads only `O(log n)` index
//!   entries, every one counted ([`Work`] counts both, owned by the plan's owner, never shared
//!   between threads).
//! - **Ruling L6.** A block taller than the view splits at its semantic boundary (a section's
//!   heading over its first row) into units, each checked alone; a unit that still does not fit
//!   and meets the view is [`TooTall`], counted, never a silent pass.
//! - **Ruling L7.** At an offset where no unit is whole, the DM's offset stays; the mask says so
//!   ([`NoWholeBlock`], counted) with the units at the view's top and after it, and the offset
//!   that shows each.
//! - **Design §3, the pixel grid.** Whether a unit starts or ends inside the view is decided on
//!   the physical pixel grid of the scale it was measured at: at most one physical pixel after
//!   rounding, at every scale.
#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "layout step 3b's first host draws from the plan; 3a is its pure half"
    )
)]

use crate::design::{
    extras::Heading,
    memo::{Changed, Memo, Revised, Revision},
    tokens::{Density, S, Type, XXL},
};

/// A list row of the first vocabulary. Its height is the part's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Row {
    /// The `None yet.` row of an empty list.
    Empty,
    /// A row of one line.
    OneLine,
    /// A title over a second line (`two_line_row`).
    TwoLine,
    /// A Library entry: its name over its detail line (layout PR 3b's first host).
    Entry,
    /// A fact/action row (`action_rows`): its facts, with a status line when the list has one,
    /// and an action line when its actions wrap under its facts (layout PR 4).
    Action { status: bool, wrapped: bool },
}

/// The height of a fact/action row's facts (`Row::Action`): the name's link target (`XL`) and
/// a line of facts, which egui lays out at its control height.
pub const ACTION_ROW: f32 = super::super::tokens::XL + super::super::theme::INTERACT_HEIGHT;

/// The height of a Library entry row (`Row::Entry`).
pub const ENTRY_ROW_HEIGHT: f32 = 44.0;

impl Row {
    fn height(self) -> f32 {
        match self {
            Self::Empty => Type::Body.line(),
            Self::OneLine => Density::Scan.row(),
            Self::TwoLine => XXL + S,
            Self::Entry => ENTRY_ROW_HEIGHT,
            Self::Action { status, wrapped } => {
                ACTION_ROW
                    + if status { Type::Small.line() } else { 0.0 }
                    + if wrapped {
                        Density::Read.control()
                    } else {
                        0.0
                    }
            }
        }
    }
}

/// One block of a body, in content order: a row, or a list group's heading bound to its first
/// row (or its `None yet.` row), which are one block while they fit the view together.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    Row(Row),
    Section(Row),
}

/// A list group's heading: the space above it, its line and the space under it.
fn heading() -> f32 {
    let group = Heading::Group {
        title: "",
        count: 0,
    };
    group.above() + Type::Label.line() + group.below()
}

/// What a plan is measured in: the body's width, the theme's font generation and the scale. A
/// change of any of them measures the plan again.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::design) struct MeasureEnv {
    pub width: f32,
    pub fonts: u64,
    pub scale: f32,
}

/// The measured blocks: where each one starts, where the content ends, where a section may
/// split, and the scale it was measured at.
#[derive(Debug, PartialEq)]
pub(in crate::design) struct BlockPlan {
    index: Index,
    /// The tallest block: the least room in which every block can be whole (ruling L3).
    tallest: f32,
    /// Physical pixels per point: its comparisons are made on that grid.
    scale: f32,
}

/// Where each block starts and where a section may split. Read one entry at a time through
/// [`Reader`], which counts each read for its mask (I12); a test build also counts each read
/// here, so a read that goes round the count shows.
#[derive(Debug, PartialEq)]
struct Index {
    /// `tops[i]..tops[i + 1]` is block `i`; the last entry is the content's whole extent.
    tops: Vec<f32>,
    /// A section's heading height (its semantic boundary, from its top); `None` for a row.
    heads: Vec<Option<f32>>,
}

#[cfg(test)]
thread_local! {
    static INDEX_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The index entries read on this thread so far, for the check that a mask counts each one.
#[cfg(test)]
pub(in crate::design) fn index_reads() -> usize {
    INDEX_READS.with(std::cell::Cell::get)
}

impl Index {
    fn read(&self) {
        #[cfg(test)]
        INDEX_READS.with(|reads| reads.set(reads.get() + 1));
    }

    fn top(&self, at: usize) -> f32 {
        self.read();
        self.tops[at]
    }

    fn head(&self, at: usize) -> Option<f32> {
        self.read();
        self.heads[at]
    }
}

/// A mask's reads of the plan's index, each one counted (I12).
struct Reader<'p> {
    index: &'p Index,
    read: std::cell::Cell<usize>,
}

impl<'p> Reader<'p> {
    fn new(plan: &'p BlockPlan) -> Self {
        Self {
            index: &plan.index,
            read: std::cell::Cell::new(0),
        }
    }

    fn top(&self, at: usize) -> f32 {
        self.read.set(self.read.get() + 1);
        self.index.top(at)
    }

    fn head(&self, at: usize) -> Option<f32> {
        self.read.set(self.read.get() + 1);
        self.index.head(at)
    }

    fn span(&self, at: usize) -> (f32, f32) {
        (self.top(at), self.top(at + 1))
    }

    fn extent(&self) -> f32 {
        self.top(self.index.heads.len())
    }

    /// The first index in `lo..hi` where `past` holds (it holds from some index on).
    fn first(lo: usize, hi: usize, past: impl Fn(usize) -> bool) -> usize {
        let (mut lo, mut hi) = (lo, hi);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if past(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

/// Which part of a block a unit is: the whole block, or the heading or the row of a section
/// taller than the view (ruling L6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::design) enum Part {
    Block,
    Heading,
    Row,
}

/// One unit of the mask: a block, or one part of a split section, and its span in the content.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Unit {
    block: usize,
    part: Part,
    from: f32,
    to: f32,
}

/// A unit at the view's top or after it, its top in the content and its height, and the offset
/// nearest the DM's that shows it whole by the mask's own edge rule (`None` when no offset in
/// the native range does: it is taller than the view).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::design) struct Route {
    pub block: usize,
    pub part: Part,
    pub top: f32,
    pub height: f32,
    pub offset: Option<f32>,
}

/// What a plan's owner did, for I12: plans measured, the blocks and index bytes they wrote, the
/// index entries the masks read, and the diagnostics the masks reported. Owned by the plan's
/// owner, so visual scenarios on parallel threads never share a count.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(in crate::design) struct Work {
    pub builds: usize,
    pub blocks_measured: usize,
    pub bytes_indexed: usize,
    pub entries_read: usize,
    pub no_whole: usize,
    pub too_tall: usize,
}

/// An offset where no unit is whole although content is in the view (ruling L7). The offset
/// stays; the unit under the view's top and the one after it (a block, or a split section's
/// heading or row), their heights, and the least move that shows each whole are the route to a
/// whole unit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::design) struct NoWholeBlock {
    pub owner: &'static str,
    pub offset: f32,
    pub view: f32,
    pub above: Route,
    pub below: Option<Route>,
}

/// A unit that meets the view and is taller than it (ruling L6): a row, or a block with no
/// semantic boundary, or one part of a split section. A counted exception of its owner, never a
/// fragment that passes as whole.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::design) struct TooTall {
    pub owner: &'static str,
    pub block: usize,
    pub part: Part,
    pub height: f32,
    pub view: f32,
}

/// Which blocks are whole in the view at an offset.
#[derive(Clone, Debug, PartialEq)]
pub(in crate::design) struct Mask {
    /// The blocks wholly in the view, in order: only they are drawn whole.
    pub whole: std::ops::Range<usize>,
    /// The content's y span of every whole unit in the view (the whole blocks, and a section's
    /// heading or row when the section is taller than the view): only this paints, takes a press
    /// or shows to a screen reader. `None`: no unit is whole.
    pub eligible: Option<(f32, f32)>,
    pub no_whole: Option<NoWholeBlock>,
    /// Every unit taller than the view that meets it (two at most: one at each edge).
    pub too_tall: Vec<TooTall>,
    /// The index entries this mask read: every one (its searches, the eligible span, the cut
    /// blocks' spans and boundaries, and the routes).
    pub read: usize,
}

impl BlockPlan {
    fn measure(blocks: &[Block], env: &MeasureEnv, work: &mut Work) -> Self {
        // The first vocabulary's heights do not depend on the width or fonts; the plan is still
        // measured again when they change, so a wrapping block added later is too.
        let mut tops = Vec::with_capacity(blocks.len() + 1);
        let mut heads = Vec::with_capacity(blocks.len());
        let mut top = 0.0;
        for block in blocks {
            tops.push(top);
            let (height, head) = match *block {
                Block::Row(row) => (row.height(), None),
                Block::Section(row) => (heading() + row.height(), Some(heading())),
            };
            heads.push(head);
            top += height;
        }
        tops.push(top);
        let tallest = tops
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold(0.0, f32::max);
        work.builds += 1;
        work.blocks_measured += blocks.len();
        work.bytes_indexed +=
            std::mem::size_of_val(tops.as_slice()) + std::mem::size_of_val(heads.as_slice());
        Self {
            index: Index { tops, heads },
            tallest,
            scale: env.scale,
        }
    }

    /// A plan of blocks of these heights, each with its semantic boundary, measured at `scale`,
    /// for the part's checks.
    #[cfg(test)]
    pub(in crate::design) fn of_heights(blocks: &[(f32, Option<f32>)], scale: f32) -> Self {
        let mut tops = vec![0.0];
        for (height, _) in blocks {
            tops.push(tops.last().copied().unwrap_or(0.0) + height);
        }
        Self {
            tallest: blocks.iter().map(|(height, _)| *height).fold(0.0, f32::max),
            index: Index {
                tops,
                heads: blocks.iter().map(|(_, head)| *head).collect(),
            },
            scale,
        }
    }

    pub(in crate::design) fn len(&self) -> usize {
        self.index.heads.len()
    }

    /// The tallest block: the least room in which every block can be whole (ruling L3).
    pub(in crate::design) fn tallest(&self) -> f32 {
        self.tallest
    }

    /// The scale the plan was measured at: its comparisons are on that physical pixel grid.
    pub(in crate::design) fn scale(&self) -> f32 {
        self.scale
    }

    /// Block `at`'s top and end, in the content.
    pub(in crate::design) fn span(&self, at: usize) -> (f32, f32) {
        Reader::new(self).span(at)
    }

    /// The content's whole extent: the native range never comes from the mask (I2).
    pub(in crate::design) fn extent(&self) -> f32 {
        Reader::new(self).extent()
    }

    /// The largest native offset in a view this tall.
    pub(in crate::design) fn max_offset(&self, view: f32) -> f32 {
        (self.extent() - view).max(0.0)
    }

    /// An offset kept in the native range (a content that shrank while scrolled): the DM's
    /// offset, or the largest one; never snapped to a block.
    pub(in crate::design) fn clamp(&self, offset: f32, view: f32) -> f32 {
        offset.clamp(0.0, self.max_offset(view))
    }

    /// The offset nearest `from` that shows block `at` whole in a view this tall, in `O(1)`;
    /// `None` when the block is taller than the view.
    pub(in crate::design) fn offset_showing(&self, at: usize, view: f32, from: f32) -> Option<f32> {
        let read = Reader::new(self);
        let (top, end) = read.span(at);
        let block = Unit {
            block: at,
            part: Part::Block,
            from: top,
            to: end,
        };
        self.showing(&read, block, view, from)
    }

    /// `y` on the physical pixel grid of the plan's scale.
    fn grid(&self, y: f32) -> f32 {
        (y * self.scale).round()
    }

    /// `y` is at or after `edge` on the pixel grid.
    fn at_or_after(&self, y: f32, edge: f32) -> bool {
        self.grid(y) >= self.grid(edge)
    }

    /// `y` is at or before `edge` on the pixel grid.
    fn at_or_before(&self, y: f32, edge: f32) -> bool {
        self.grid(y) <= self.grid(edge)
    }

    /// Whether `from..to` is whole in a view `view` tall at `offset`: eligibility's edge rule.
    fn whole_at(&self, from: f32, to: f32, offset: f32, view: f32) -> bool {
        self.at_or_after(from, offset) && self.at_or_before(to, offset + view)
    }

    /// The offset nearest `near` within `range` at which `from..to` is whole in a view this tall,
    /// by eligibility's own edge rule (`whole_at`), or `None`. Its edges' pixels bound a window
    /// of offsets (the view's last pixel at or after its end's, its first at or before its
    /// top's). The pixel offset nearest `near` in it, when it shows the span; else the nearest of
    /// `near` held inside it, its ends and its middle; each checked by that rule.
    fn whole_offset(
        &self,
        from: f32,
        to: f32,
        view: f32,
        near: f32,
        range: (f32, f32),
    ) -> Option<f32> {
        let scale = self.scale;
        let low = ((self.grid(to) - 0.5) / scale - view).max(range.0);
        let high = ((self.grid(from) + 0.5) / scale).min(range.1);
        if low > high {
            return None;
        }
        let shows =
            |at: &f32| *at >= range.0 && *at <= range.1 && self.whole_at(from, to, *at, view);
        // An offset on the pixel grid when one shows it; else (the native range ends between two
        // pixels) the nearest of the window's own points.
        let (first, last) = ((low * scale).ceil(), (high * scale).floor());
        let pixel = (first <= last).then(|| (near * scale).round().clamp(first, last) / scale);
        pixel.filter(shows).or_else(|| {
            [near.clamp(low, high), low, high, (low + high) / 2.0]
                .into_iter()
                .filter(shows)
                .min_by(|a, b| (a - near).abs().total_cmp(&(b - near).abs()))
        })
    }

    /// A span fits a view this tall: some offset shows it whole (`whole_offset`).
    fn fits(&self, from: f32, to: f32, view: f32) -> bool {
        self.whole_offset(from, to, view, from, (f32::MIN, f32::MAX))
            .is_some()
    }

    /// Block `at`'s units in a view this tall: the block while it fits; else a section's
    /// heading and row (its semantic boundary, L6), or the block alone when it has none.
    fn units(&self, read: &Reader<'_>, at: usize, view: f32) -> [Option<Unit>; 2] {
        let (from, to) = read.span(at);
        let unit = |part, from, to| {
            Some(Unit {
                block: at,
                part,
                from,
                to,
            })
        };
        if self.fits(from, to, view) {
            return [unit(Part::Block, from, to), None];
        }
        match read.head(at) {
            Some(head) => [
                unit(Part::Heading, from, from + head),
                unit(Part::Row, from + head, to),
            ],
            None => [unit(Part::Block, from, to), None],
        }
    }

    /// The offset nearest `from` in the native range that shows `unit` whole in a view this
    /// tall, by the mask's own edge rule.
    fn showing(&self, read: &Reader<'_>, unit: Unit, view: f32, from: f32) -> Option<f32> {
        let most = (read.extent() - view).max(0.0);
        self.whole_offset(unit.from, unit.to, view, from, (0.0, most))
    }

    /// Which units are whole in a view `view` tall at native `offset` (I1, rulings L6 and L7,
    /// on the pixel grid). Pure: it reads `O(log n)` index entries, counts each one, and changes
    /// nothing, not the offset nor the view.
    pub(in crate::design) fn mask(&self, owner: &'static str, offset: f32, view: f32) -> Mask {
        let read = Reader::new(self);
        let n = self.len();
        let (top, bottom) = (offset, offset + view);
        // The first block that starts in the view, then the first from it that ends below it.
        let start = Reader::first(0, n, |at| self.at_or_after(read.top(at), top));
        let end = Reader::first(start, n, |at| !self.at_or_before(read.top(at + 1), bottom));
        let mut mask = Mask {
            whole: start..end.max(start),
            eligible: (start < end).then(|| (read.top(start), read.top(end))),
            no_whole: None,
            too_tall: Vec::new(),
            read: 0,
        };
        // The blocks cut by the view's top and bottom edges. Each of their units is checked
        // alone: a unit that fits and lies whole in the view is eligible; a unit taller than the
        // view that meets it is `TooTall` (L6), also beside an eligible heading.
        let cut_above = start
            .checked_sub(1)
            .filter(|&at| !self.at_or_before(read.top(at + 1), top));
        let cut_below = (end < n && !self.at_or_after(read.top(end), bottom)).then_some(end);
        for at in [cut_above, cut_below].into_iter().flatten() {
            for unit in self.units(&read, at, view).into_iter().flatten() {
                if self.fits(unit.from, unit.to, view) {
                    if self.at_or_after(unit.from, top) && self.at_or_before(unit.to, bottom) {
                        mask.eligible = Some(match mask.eligible {
                            Some((low, high)) => (low.min(unit.from), high.max(unit.to)),
                            None => (unit.from, unit.to),
                        });
                    }
                } else if !self.at_or_before(unit.to, top) && !self.at_or_after(unit.from, bottom) {
                    mask.too_tall.push(TooTall {
                        owner,
                        block: at,
                        part: unit.part,
                        height: unit.to - unit.from,
                        view,
                    });
                }
            }
        }
        if mask.eligible.is_none() && n > 0 && !self.at_or_after(top, read.extent()) {
            // L7: no unit is whole. The unit under the view's top, and the one after it: the next
            // part of its section, or the next block's first unit.
            let under =
                Reader::first(0, n, |at| !self.at_or_before(read.top(at + 1), top)).min(n - 1);
            let units = self.units(&read, under, view);
            let here = match units {
                [Some(heading), Some(row)] if self.at_or_before(heading.to, top) => {
                    [Some(row), None]
                }
                units => units,
            };
            let above = here[0].expect("a block has a unit");
            let below = here[1].or_else(|| {
                (under + 1 < n)
                    .then(|| self.units(&read, under + 1, view)[0])
                    .flatten()
            });
            let route = |unit: Unit| Route {
                block: unit.block,
                part: unit.part,
                top: unit.from,
                height: unit.to - unit.from,
                offset: self.showing(&read, unit, view, offset),
            };
            mask.no_whole = Some(NoWholeBlock {
                owner,
                offset,
                view,
                above: route(above),
                below: below.map(route),
            });
        }
        mask.read = read.read.get();
        mask
    }
}

/// A body's plan, kept by its owner (one per pane instance and view): measured once per change
/// of the content's revision or of the measuring environment's, and never on an unchanged frame
/// (ruling M6: a sealed `Memo` key of two revisions, never a width in the key).
pub(in crate::design) struct PlanState {
    owner: &'static str,
    env: Option<Revised<MeasureEnv>>,
    plan: Memo<(Revision, Revision), BlockPlan>,
    work: Work,
}

impl PlanState {
    pub(in crate::design) fn new(owner: &'static str) -> Self {
        Self {
            owner,
            env: None,
            plan: Memo::new(),
            work: Work::default(),
        }
    }

    /// The plan of `content` measured in `env`: the kept one while neither changed.
    pub(in crate::design) fn plan(
        &mut self,
        content: &Revised<Vec<Block>>,
        env: MeasureEnv,
    ) -> &BlockPlan {
        let kept = match &mut self.env {
            Some(kept) => {
                kept.revise(|kept| {
                    let changed = *kept != env;
                    *kept = env;
                    Changed::from(changed)
                });
                kept
            }
            None => self.env.insert(Revised::new(env)),
        };
        let key = (content.revision(), kept.revision());
        let work = &mut self.work;
        self.plan
            .get_or(key, || BlockPlan::measure(content, kept, work))
    }

    /// The mask of `content`'s plan in `env` (the kept plan while neither changed) at `offset` in
    /// a view `view` tall; its diagnostics are counted. It plans first, so it never reads a plan
    /// of older content.
    pub(in crate::design) fn mask(
        &mut self,
        content: &Revised<Vec<Block>>,
        env: MeasureEnv,
        offset: f32,
        view: f32,
    ) -> Mask {
        let owner = self.owner;
        let mask = self.plan(content, env).mask(owner, offset, view);
        self.work.entries_read += mask.read;
        self.work.no_whole += usize::from(mask.no_whole.is_some());
        self.work.too_tall += mask.too_tall.len();
        mask
    }

    pub(in crate::design) fn work(&self) -> Work {
        self.work
    }
}
