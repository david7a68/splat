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
//!   entries ([`Work`] counts both, owned by the plan's owner, never shared between threads).
//! - **Ruling L7.** At an offset where no block is whole, the DM's offset stays; the mask says so
//!   ([`NoWholeBlock`], counted) with the neighbouring blocks and the offset that shows each.
//! - **Ruling L6.** A block taller than the view splits at its semantic boundary (a section's
//!   heading over its first row); a row that still does not fit is [`TooTall`], counted, never a
//!   silent pass.
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
pub(in crate::design) enum Row {
    /// The `None yet.` row of an empty list.
    Empty,
    /// A row of one line.
    OneLine,
    /// A title over a second line (`two_line_row`).
    TwoLine,
}

impl Row {
    fn height(self) -> f32 {
        match self {
            Self::Empty => Type::Body.line(),
            Self::OneLine => Density::Scan.row(),
            Self::TwoLine => XXL + S,
        }
    }
}

/// One block of a body, in content order: a row, or a list group's heading bound to its first
/// row (or its `None yet.` row), which are one block while they fit the view together.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::design) enum Block {
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

/// The measured blocks: where each one starts, where the content ends, and where a section may
/// split.
#[derive(Debug, PartialEq)]
pub(in crate::design) struct BlockPlan {
    /// `tops[i]..tops[i + 1]` is block `i`; the last entry is the content's whole extent.
    tops: Vec<f32>,
    /// A section's heading height (its semantic boundary, from its top); `None` for a row.
    heads: Vec<Option<f32>>,
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

/// An offset where no block is whole although content is in the view (ruling L7). The offset
/// stays; the blocks at the view's top and bottom edge, their heights, and the least move that
/// shows each whole are the route to a whole block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::design) struct NoWholeBlock {
    pub owner: &'static str,
    pub offset: f32,
    pub view: f32,
    /// (block, height, the offset that shows it whole), when it fits the view.
    pub above: (usize, f32, Option<f32>),
    pub below: Option<(usize, f32, Option<f32>)>,
}

/// A block in the view that is taller than the view and has no semantic boundary that fits
/// (ruling L6): a counted exception of its owner, never a fragment that passes as whole.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::design) struct TooTall {
    pub owner: &'static str,
    pub block: usize,
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
    pub too_tall: Option<TooTall>,
    /// The index entries this mask read.
    pub read: usize,
}

/// Points of slack in the comparisons: half a point, as the pane's own checks use.
const SLACK: f32 = 0.5;

impl BlockPlan {
    fn measure(blocks: &[Block], _env: &MeasureEnv, work: &mut Work) -> Self {
        // The first vocabulary's heights do not depend on the width, fonts or scale; the plan is
        // still measured again when they change, so a wrapping block added later is too.
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
        work.builds += 1;
        work.blocks_measured += blocks.len();
        work.bytes_indexed +=
            std::mem::size_of_val(tops.as_slice()) + std::mem::size_of_val(heads.as_slice());
        Self { tops, heads }
    }

    /// A plan of blocks of these heights, each with its semantic boundary, for the part's checks.
    #[cfg(test)]
    pub(in crate::design) fn of_heights(blocks: &[(f32, Option<f32>)]) -> Self {
        let mut tops = vec![0.0];
        for (height, _) in blocks {
            tops.push(tops.last().copied().unwrap_or(0.0) + height);
        }
        Self {
            tops,
            heads: blocks.iter().map(|(_, head)| *head).collect(),
        }
    }

    pub(in crate::design) fn len(&self) -> usize {
        self.heads.len()
    }

    /// Block `at`'s top and end, in the content.
    pub(in crate::design) fn span(&self, at: usize) -> (f32, f32) {
        (self.tops[at], self.tops[at + 1])
    }

    /// The content's whole extent: the native range never comes from the mask (I2).
    pub(in crate::design) fn extent(&self) -> f32 {
        self.tops.last().copied().unwrap_or(0.0)
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
        let (top, end) = self.span(at);
        (end - top <= view + SLACK).then(|| {
            // Any offset from the one that puts its end at the view's bottom to its top shows it.
            let low = (end - view).max(0.0).min(top);
            from.clamp(low, top).min(self.max_offset(view))
        })
    }

    /// The first index in `lo..hi` where `past` holds (it holds from some index on), and how many
    /// entries it read.
    fn first(lo: usize, hi: usize, past: impl Fn(usize) -> bool) -> (usize, usize) {
        let (mut lo, mut hi, mut read) = (lo, hi, 0);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            read += 1;
            if past(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        (lo, read)
    }

    /// Which blocks are whole in a view `view` tall at native `offset` (I1, rulings L6 and L7).
    /// Pure: it reads `O(log n)` index entries and changes nothing, not the offset nor the view.
    pub(in crate::design) fn mask(&self, owner: &'static str, offset: f32, view: f32) -> Mask {
        let n = self.len();
        let (top, bottom) = (offset, offset + view);
        // The first block that starts in the view, then the first from it that ends below it.
        let (start, read_start) = Self::first(0, n, |at| self.tops[at] >= top - SLACK);
        let (end, read_end) = Self::first(start, n, |at| self.tops[at + 1] > bottom + SLACK);
        let mut mask = Mask {
            whole: start..end.max(start),
            eligible: (start < end).then(|| (self.tops[start], self.tops[end])),
            no_whole: None,
            too_tall: None,
            read: read_start + read_end,
        };
        // The blocks cut by the view's top and bottom edges. One taller than the view splits at
        // its semantic boundary (L6): a part that lies whole in the view is eligible.
        let cut_above = start
            .checked_sub(1)
            .filter(|at| self.tops[at + 1] > top + SLACK);
        let cut_below = (end < n && self.tops[end] < bottom - SLACK).then_some(end);
        for at in [cut_above, cut_below].into_iter().flatten() {
            let (from, to) = self.span(at);
            if to - from <= view + SLACK {
                continue;
            }
            let parts = match self.heads[at] {
                Some(head) => [(from, from + head), (from + head, to)],
                None => [(from, to), (to, to)],
            };
            let mut split = false;
            for (from, to) in parts.into_iter().filter(|(from, to)| to > from) {
                if from >= top - SLACK && to <= bottom + SLACK {
                    mask.eligible = Some(match mask.eligible {
                        Some((low, high)) => (low.min(from), high.max(to)),
                        None => (from, to),
                    });
                    split = true;
                }
            }
            if !split && mask.too_tall.is_none() {
                mask.too_tall = Some(TooTall {
                    owner,
                    block: at,
                    height: to - from,
                    view,
                });
            }
        }
        if mask.eligible.is_none() && n > 0 && top < self.extent() - SLACK {
            // L7: no unit is whole. The block under the view's top, and the one after it.
            let (under, read) = Self::first(0, n, |at| self.tops[at + 1] > top + SLACK);
            mask.read += read;
            let under = under.min(n - 1);
            let route = |at: usize| {
                let (from, to) = self.span(at);
                (at, to - from, self.offset_showing(at, view, offset))
            };
            mask.no_whole = Some(NoWholeBlock {
                owner,
                offset,
                view,
                above: route(under),
                below: (under + 1 < n).then(|| route(under + 1)),
            });
        }
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
    key: Option<(Revision, Revision)>,
    work: Work,
}

impl PlanState {
    pub(in crate::design) fn new(owner: &'static str) -> Self {
        Self {
            owner,
            env: None,
            plan: Memo::new(),
            key: None,
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
        self.key = Some(key);
        let work = &mut self.work;
        self.plan
            .get_or(key, || BlockPlan::measure(content, kept, work))
    }

    /// The mask of the last plan at `offset` in a view `view` tall; its diagnostics are counted.
    pub(in crate::design) fn mask(&mut self, offset: f32, view: f32) -> Option<Mask> {
        let mask = self.plan.get(self.key?)?.mask(self.owner, offset, view);
        self.work.entries_read += mask.read;
        self.work.no_whole += usize::from(mask.no_whole.is_some());
        self.work.too_tall += usize::from(mask.too_tall.is_some());
        Some(mask)
    }

    pub(in crate::design) fn work(&self) -> Work {
        self.work
    }
}
