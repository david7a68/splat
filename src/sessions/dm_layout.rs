//! The DM screen's layout: a split tree. A node is one tile, or a split of its room into parts
//! side by side (`Across`) or one above the other (`Down`), each part a node with its share of
//! the room. Sizes are shares, so a layout made on one screen fits another, and no tile is
//! drawn under its minimum. A tile that is in no node is hidden.
//!
//! The tree is kept in one form (`Node::tidy`): every split has two parts or more, no split has
//! a part split the same way (that part's parts join it), and each split's shares sum to 1.
use super::Tile;
use crate::design::DockSide;
use eframe::egui::{Rect, Vec2, pos2, vec2};
use serde::{Deserialize, Serialize};

/// Every tile, in the order the tabs and `Show …` list them.
pub(super) const TILES: [Tile; 4] = [Tile::Plan, Tile::Happened, Tile::Threads, Tile::Rest];

/// The least height of a tile: its title and a few lines.
pub(super) const TILE_MIN_HEIGHT: f32 = 120.0;
/// The least width of a tile other than the plan.
pub(super) const TILE_MIN_WIDTH: f32 = 200.0;
/// What a tile put along an edge of the whole screen takes of it.
pub(super) const EDGE_SHARE: f32 = 1.0 / 3.0;

/// How a split lays out its parts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Axis {
    /// Side by side, from the left.
    Across,
    /// One above the other, from the top.
    Down,
}

impl Axis {
    fn of(side: DockSide) -> Self {
        match side {
            DockSide::Left | DockSide::Right => Self::Across,
            DockSide::Top | DockSide::Bottom => Self::Down,
        }
    }

    fn along(self, size: Vec2) -> f32 {
        match self {
            Self::Across => size.x,
            Self::Down => size.y,
        }
    }
}

/// Where a tile goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Place {
    /// On one side of another tile: the two share that tile's room.
    Beside(Tile, DockSide),
    /// Along one edge of the whole screen, over all the other tiles' width or height.
    Edge(DockSide),
}

/// The room between tiles: beside each other, and one above the other.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Gaps {
    pub(super) across: f32,
    pub(super) down: f32,
}

impl Gaps {
    fn of(self, axis: Axis) -> f32 {
        match axis {
            Axis::Across => self.across,
            Axis::Down => self.down,
        }
    }
}

/// The plan's least width, and the gaps, for one page.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Room {
    pub(super) plan_min: f32,
    pub(super) gaps: Gaps,
}

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Tile(Tile),
    Split { axis: Axis, parts: Vec<(Node, f32)> },
}

impl Node {
    fn tiles(&self, out: &mut Vec<Tile>) {
        match self {
            Self::Tile(tile) => out.push(*tile),
            Self::Split { parts, .. } => {
                for (part, _) in parts {
                    part.tiles(out);
                }
            }
        }
    }

    fn min_size(&self, room: Room) -> Vec2 {
        match self {
            Self::Tile(Tile::Plan) => vec2(room.plan_min, TILE_MIN_HEIGHT),
            Self::Tile(_) => vec2(TILE_MIN_WIDTH, TILE_MIN_HEIGHT),
            Self::Split { axis, parts } => {
                let gaps = room.gaps.of(*axis) * (parts.len() - 1) as f32;
                let (along, across) = parts.iter().map(|(part, _)| part.min_size(room)).fold(
                    (0.0_f32, 0.0_f32),
                    |(along, across), size| match axis {
                        Axis::Across => (along + size.x, across.max(size.y)),
                        Axis::Down => (along + size.y, across.max(size.x)),
                    },
                );
                match axis {
                    Axis::Across => vec2(along + gaps, across),
                    Axis::Down => vec2(across, along + gaps),
                }
            }
        }
    }

    /// The node at `path`, part by part from this one.
    fn at_mut(&mut self, path: &[usize]) -> Option<&mut Self> {
        match path.split_first() {
            None => Some(self),
            Some((&first, rest)) => match self {
                Self::Split { parts, .. } => parts.get_mut(first)?.0.at_mut(rest),
                Self::Tile(_) => None,
            },
        }
    }

    /// The path to `tile`'s node.
    fn find(&self, tile: Tile) -> Option<Vec<usize>> {
        match self {
            Self::Tile(shown) => (*shown == tile).then(Vec::new),
            Self::Split { parts, .. } => parts.iter().enumerate().find_map(|(at, (part, _))| {
                part.find(tile).map(|mut path| {
                    path.insert(0, at);
                    path
                })
            }),
        }
    }

    /// Take `tile` out of the tree under this split. A split left with one part becomes that
    /// part in `tidy`.
    fn remove(&mut self, tile: Tile) -> bool {
        let Self::Split { parts, .. } = self else {
            return false;
        };
        if let Some(at) = parts.iter().position(|(part, _)| *part == Self::Tile(tile)) {
            parts.remove(at);
            return true;
        }
        parts.iter_mut().any(|(part, _)| part.remove(tile))
    }

    /// Put `tile` on `side` of the tile `target`. In a split the same way, the two share
    /// `target`'s part; otherwise `target`'s node becomes such a split.
    fn put_beside(&mut self, tile: Tile, target: Tile, side: DockSide) -> bool {
        let axis = Axis::of(side);
        let before = matches!(side, DockSide::Left | DockSide::Top);
        match self {
            Self::Tile(shown) if *shown == target => {
                let mut parts = vec![(Self::Tile(target), 0.5), (Self::Tile(tile), 0.5)];
                if before {
                    parts.reverse();
                }
                *self = Self::Split { axis, parts };
                true
            }
            Self::Tile(_) => false,
            Self::Split {
                axis: split_axis,
                parts,
            } => {
                if *split_axis == axis
                    && let Some(at) = parts
                        .iter()
                        .position(|(part, _)| *part == Self::Tile(target))
                {
                    parts[at].1 /= 2.0;
                    let share = parts[at].1;
                    parts.insert(if before { at } else { at + 1 }, (Self::Tile(tile), share));
                    return true;
                }
                parts
                    .iter_mut()
                    .any(|(part, _)| part.put_beside(tile, target, side))
            }
        }
    }

    /// Keep the tree in its one form: no split of one part, no part split the way its split
    /// is, shares that are positive numbers and sum to 1.
    fn tidy(&mut self) {
        let Self::Split { axis, parts } = self else {
            return;
        };
        for (part, _) in parts.iter_mut() {
            part.tidy();
        }
        normalise(parts.iter_mut().map(|(_, share)| share));
        let mut flat = Vec::new();
        for (part, share) in std::mem::take(parts) {
            match part {
                Self::Split {
                    axis: inner,
                    parts: inner_parts,
                } if inner == *axis => {
                    flat.extend(
                        inner_parts
                            .into_iter()
                            .map(|(part, inner_share)| (part, inner_share * share)),
                    );
                }
                part => flat.push((part, share)),
            }
        }
        *parts = flat;
        if parts.len() == 1 {
            *self = parts.remove(0).0;
        }
    }

    fn place(&self, rect: Rect, room: Room, path: &mut Vec<usize>, out: &mut Placed) {
        let Self::Split { axis, parts } = self else {
            if let Self::Tile(tile) = self {
                out.tiles.push((*tile, rect));
            }
            return;
        };
        let gap = room.gaps.of(*axis);
        let length = axis.along(rect.size()) - gap * (parts.len() - 1) as f32;
        let sizes = self.sizes(length, room);
        // An edge's handle reaches past its gap into the parts' margins. A handle of a split
        // inside stops short of it, by as much as it reaches past the other way's gap.
        let reach = |other: f32| ((crate::design::XL - other) / 2.0).max(0.0);
        let mut start = match axis {
            Axis::Across => rect.left(),
            Axis::Down => rect.top(),
        };
        for (at, ((part, _), size)) in parts.iter().zip(&sizes).enumerate() {
            let part_rect = match axis {
                Axis::Across => {
                    Rect::from_min_size(pos2(start, rect.top()), vec2(*size, rect.height()))
                }
                Axis::Down => {
                    Rect::from_min_size(pos2(rect.left(), start), vec2(rect.width(), *size))
                }
            };
            path.push(at);
            part.place(part_rect, room, path, out);
            path.pop();
            start += size;
            if at + 1 < parts.len() {
                let gap_rect = match axis {
                    Axis::Across => {
                        let inset = reach(room.gaps.down);
                        Rect::from_x_y_ranges(
                            start..=start + gap,
                            rect.top() + inset..=rect.bottom() - inset,
                        )
                    }
                    Axis::Down => {
                        let inset = reach(room.gaps.across);
                        Rect::from_x_y_ranges(
                            rect.left() + inset..=rect.right() - inset,
                            start..=start + gap,
                        )
                    }
                };
                let mut before = Vec::new();
                part.tiles(&mut before);
                out.edges.push(Edge {
                    split: path.clone(),
                    after: at,
                    axis: *axis,
                    gap: gap_rect,
                    length,
                    before,
                });
            }
            start += gap;
        }
    }

    /// The parts' sizes along the split in `length`: by their shares, none under its minimum.
    /// When the minimums do not fit, by their shares alone.
    fn sizes(&self, length: f32, room: Room) -> Vec<f32> {
        let Self::Split { axis, parts } = self else {
            return vec![length];
        };
        let shares: Vec<f32> = parts.iter().map(|(_, share)| *share).collect();
        let mins: Vec<f32> = parts
            .iter()
            .map(|(part, _)| axis.along(part.min_size(room)))
            .collect();
        fit(&shares, &mins, length)
            .unwrap_or_else(|| shares.iter().map(|share| share * length).collect())
    }
}

/// Scale positive, finite shares to the sum 1; any other share counts as an even one.
fn normalise<'a>(shares: impl Iterator<Item = &'a mut f32>) {
    let mut shares: Vec<&mut f32> = shares.collect();
    let count = shares.len() as f32;
    for share in &mut shares {
        if !share.is_finite() || **share <= 0.0 {
            **share = 1.0 / count;
        }
    }
    let sum: f32 = shares.iter().map(|share| **share).sum();
    // Shares near `f32::MAX` add up past it: they all go even.
    let even = !sum.is_finite();
    for share in shares {
        *share = if even { 1.0 / count } else { *share / sum };
    }
}

/// Sizes for `shares` of `room`, none under its minimum. The sizes under their minimum take
/// it, and the others share what is left in their proportions. `None` when the minimums do
/// not fit.
fn fit(shares: &[f32], mins: &[f32], room: f32) -> Option<Vec<f32>> {
    if mins.iter().sum::<f32>() > room + 0.5 {
        return None;
    }
    let mut at_min = vec![false; shares.len()];
    loop {
        let left = room
            - mins
                .iter()
                .zip(&at_min)
                .filter(|(_, at)| **at)
                .map(|(min, _)| min)
                .sum::<f32>();
        let free: f32 = shares
            .iter()
            .zip(&at_min)
            .filter(|(_, at)| !**at)
            .map(|(share, _)| share)
            .sum();
        let mut changed = false;
        for (index, share) in shares.iter().enumerate() {
            if !at_min[index] && share / free * left < mins[index] {
                at_min[index] = true;
                changed = true;
            }
        }
        if !changed || at_min.iter().all(|at| *at) {
            return Some(
                shares
                    .iter()
                    .enumerate()
                    .map(|(index, share)| {
                        if at_min[index] {
                            mins[index]
                        } else {
                            share / free * left
                        }
                    })
                    .collect(),
            );
        }
    }
}

/// Move the edge after part `edge` by `by` (`None`: even out the two parts), inside both
/// parts' minimums where they fit, and give back the shares.
fn moved(mut sizes: Vec<f32>, mins: &[f32], edge: usize, by: Option<f32>) -> Vec<f32> {
    let (a, b) = (sizes[edge], sizes[edge + 1]);
    let target = match by {
        Some(by) => a + by,
        None => (a + b) / 2.0,
    };
    let lowest = mins[edge].min((a + b) / 2.0);
    let highest = (a + b - mins[edge + 1]).max(lowest);
    sizes[edge] = target.clamp(lowest, highest);
    sizes[edge + 1] = a + b - sizes[edge];
    let sum: f32 = sizes.iter().sum();
    sizes.iter().map(|size| size / sum).collect()
}

/// Where each tile and each edge between parts is in one frame.
#[derive(Debug, Default)]
pub(super) struct Placed {
    pub(super) tiles: Vec<(Tile, Rect)>,
    pub(super) edges: Vec<Edge>,
}

/// An edge between two parts of a split, as `Layout::place` placed it.
#[derive(Clone, Debug)]
pub(super) struct Edge {
    /// The split's path from the root.
    pub(super) split: Vec<usize>,
    /// The part before the edge.
    pub(super) after: usize,
    pub(super) axis: Axis,
    /// The room between the two parts, less what the handles of the split around it take.
    pub(super) gap: Rect,
    /// The split's length for its parts, less its gaps.
    pub(super) length: f32,
    /// The tiles in the part before the edge, for the handle's name.
    pub(super) before: Vec<Tile>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Layout {
    root: Node,
}

impl Layout {
    /// One tile.
    pub(super) fn tile(tile: Tile) -> Self {
        Self {
            root: Node::Tile(tile),
        }
    }

    /// `parts` side by side, or one above the other, with these shares.
    pub(super) fn split(axis: Axis, parts: Vec<(Self, f32)>) -> Self {
        let mut root = Node::Split {
            axis,
            parts: parts
                .into_iter()
                .map(|(part, share)| (part.root, share))
                .collect(),
        };
        root.tidy();
        Self { root }
    }

    /// The tiles shown, in reading order, as a narrow page's tabs list them.
    pub(super) fn tiles(&self) -> Vec<Tile> {
        let mut out = Vec::new();
        self.root.tiles(&mut out);
        out
    }

    /// The tiles the DM hid.
    pub(super) fn hidden(&self) -> Vec<Tile> {
        let shown = self.tiles();
        TILES
            .into_iter()
            .filter(|tile| !shown.contains(tile))
            .collect()
    }

    /// The least size the layout needs.
    pub(super) fn min_size(&self, room: Room) -> Vec2 {
        self.root.min_size(room)
    }

    /// Each tile's and each edge's place in `rect`, or `None` when the tiles' least widths do
    /// not fit (the page shows tabs). A height under the tiles' least heights shares the room
    /// by the shares alone.
    pub(super) fn place(&self, rect: Rect, room: Room) -> Option<Placed> {
        if self.min_size(room).x > rect.width() + 0.5 {
            return None;
        }
        let mut out = Placed::default();
        self.root.place(rect, room, &mut Vec::new(), &mut out);
        Some(out)
    }

    /// Move `edge` by `by` points (`None`: give its two parts the same room).
    pub(super) fn resize(&mut self, edge: &Edge, by: Option<f32>, room: Room) {
        let Some(split) = self.root.at_mut(&edge.split) else {
            return;
        };
        let sizes = split.sizes(edge.length, room);
        let Node::Split { axis, parts } = split else {
            return;
        };
        if edge.after + 1 >= parts.len() {
            return;
        }
        let mins: Vec<f32> = parts
            .iter()
            .map(|(part, _)| axis.along(part.min_size(room)))
            .collect();
        for ((_, share), new) in parts.iter_mut().zip(moved(sizes, &mins, edge.after, by)) {
            *share = new;
        }
    }

    /// Whether `tile` can go to `place`: the place is another tile that shows, or an edge with
    /// another tile to stand beside.
    pub(super) fn can_place(&self, tile: Tile, place: Place) -> bool {
        let shown = self.tiles();
        match place {
            Place::Beside(target, _) => target != tile && shown.contains(&target),
            Place::Edge(_) => shown.iter().any(|other| *other != tile),
        }
    }

    pub(super) fn place_tile(&mut self, tile: Tile, place: Place) {
        if !self.can_place(tile, place) {
            return;
        }
        self.root.remove(tile);
        self.root.tidy();
        match place {
            Place::Beside(target, side) => {
                self.root.put_beside(tile, target, side);
            }
            Place::Edge(side) => {
                let rest = std::mem::replace(&mut self.root, Node::Tile(tile));
                let mut parts = vec![(Node::Tile(tile), EDGE_SHARE), (rest, 1.0 - EDGE_SHARE)];
                if matches!(side, DockSide::Right | DockSide::Bottom) {
                    parts.reverse();
                }
                self.root = Node::Split {
                    axis: Axis::of(side),
                    parts,
                };
            }
        }
        self.root.tidy();
    }

    /// Exchange two tiles' places.
    pub(super) fn swap(&mut self, a: Tile, b: Tile) {
        let (Some(at_a), Some(at_b)) = (self.root.find(a), self.root.find(b)) else {
            return;
        };
        if let Some(node) = self.root.at_mut(&at_a) {
            *node = Node::Tile(b);
        }
        if let Some(node) = self.root.at_mut(&at_b) {
            *node = Node::Tile(a);
        }
    }

    /// Whether `tile` can be hidden: one tile always shows.
    pub(super) fn can_hide(&self, tile: Tile) -> bool {
        let shown = self.tiles();
        shown.contains(&tile) && shown.len() > 1
    }

    pub(super) fn hide(&mut self, tile: Tile) {
        if self.can_hide(tile) {
            self.root.remove(tile);
            self.root.tidy();
        }
    }

    /// Show a hidden tile again, below the last tile in reading order.
    pub(super) fn show(&mut self, tile: Tile) {
        let shown = self.tiles();
        if let Some(&last) = shown.last()
            && !shown.contains(&tile)
        {
            self.root.put_beside(tile, last, DockSide::Bottom);
            self.root.tidy();
        }
    }

    pub(super) fn to_saved(&self) -> DmScreenLayout {
        fn saved(node: &Node) -> SavedNode {
            match node {
                Node::Tile(tile) => SavedNode {
                    tile: Some(tile.key().to_owned()),
                    ..SavedNode::default()
                },
                Node::Split { axis, parts } => SavedNode {
                    split: Some(
                        match axis {
                            Axis::Across => "across",
                            Axis::Down => "down",
                        }
                        .to_owned(),
                    ),
                    parts: parts
                        .iter()
                        .map(|(part, share)| SavedPart {
                            share: *share,
                            node: saved(part),
                        })
                        .collect(),
                    ..SavedNode::default()
                },
            }
        }
        DmScreenLayout {
            root: Some(saved(&self.root)),
        }
    }

    /// The layout a saved one names. Unknown and repeated tiles, unknown splits, splits with
    /// nothing in them and sizes that are no number are left out or evened out; with no tile
    /// left, there is none.
    pub(super) fn from_saved(saved: &DmScreenLayout) -> Option<Self> {
        fn read(saved: &SavedNode, seen: &mut Vec<Tile>) -> Option<Node> {
            if let Some(name) = &saved.tile {
                let tile = TILES.into_iter().find(|tile| tile.key() == name)?;
                if seen.contains(&tile) {
                    return None;
                }
                seen.push(tile);
                return Some(Node::Tile(tile));
            }
            let axis = match saved.split.as_deref()? {
                "across" => Axis::Across,
                "down" => Axis::Down,
                _ => return None,
            };
            let parts: Vec<(Node, f32)> = saved
                .parts
                .iter()
                .filter_map(|part| Some((read(&part.node, seen)?, part.share)))
                .collect();
            (!parts.is_empty()).then_some(Node::Split { axis, parts })
        }
        let mut root = read(saved.root.as_ref()?, &mut Vec::new())?;
        root.tidy();
        Some(Self { root })
    }
}

/// The DM screen's layout as the app's window state saves it. Tiles go by name, so a file from
/// another version still opens: an unknown name is left out (`Layout::from_saved`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DmScreenLayout {
    #[serde(default)]
    pub root: Option<SavedNode>,
}

/// One node: a tile by name, or a split (`across` or `down`) with its parts.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SavedNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<SavedPart>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SavedPart {
    #[serde(default)]
    pub share: f32,
    #[serde(default)]
    pub node: SavedNode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use Tile::*;

    const ROOM: Room = Room {
        plan_min: 394.0,
        gaps: Gaps {
            across: 16.0,
            down: 14.0,
        },
    };

    /// The default's three columns: the plan, What happened, Open threads over the rest.
    fn three() -> Layout {
        Layout::split(
            Axis::Across,
            vec![
                (Layout::tile(Plan), 0.3),
                (Layout::tile(Happened), 0.3),
                (
                    Layout::split(
                        Axis::Down,
                        vec![(Layout::tile(Threads), 1.0), (Layout::tile(Rest), 1.0)],
                    ),
                    0.4,
                ),
            ],
        )
    }

    /// The tree's form and its shares to a thousandth: `down[plan 0.333, …]`.
    fn shape(layout: &Layout) -> String {
        fn node(this: &Node) -> String {
            match this {
                Node::Tile(tile) => tile.key().to_owned(),
                Node::Split { axis, parts } => format!(
                    "{}[{}]",
                    match axis {
                        Axis::Across => "across",
                        Axis::Down => "down",
                    },
                    parts
                        .iter()
                        .map(|(part, share)| format!("{} {share:.3}", node(part)))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            }
        }
        node(&layout.root)
    }

    fn placed(layout: &Layout, width: f32, height: f32) -> Option<Placed> {
        layout.place(
            Rect::from_min_size(pos2(0.0, 0.0), vec2(width, height)),
            ROOM,
        )
    }

    fn rect(layout: &Layout, tile: Tile) -> Rect {
        placed(layout, 1532.0, 1000.0)
            .expect("the tiles fit")
            .tiles
            .into_iter()
            .find(|(shown, _)| *shown == tile)
            .unwrap()
            .1
    }

    #[track_caller]
    fn near(a: f32, b: f32) {
        assert!((a - b).abs() < 0.01, "{a} is not {b}");
    }

    #[test]
    fn sizes_follow_their_shares_and_never_go_under_a_minimum() {
        let layout = three();
        // 1532 less two gaps of 16 is 1500: 450, 450 and 600.
        near(rect(&layout, Plan).width(), 450.0);
        near(rect(&layout, Happened).left(), 466.0);
        near(rect(&layout, Threads).width(), 600.0);
        // 1000 less one gap of 14, in halves.
        near(rect(&layout, Threads).height(), 493.0);
        near(rect(&layout, Rest).top(), 507.0);
        // Narrower: the plan keeps its minimum, and the others share what is left.
        let narrow = placed(&layout, 1032.0, 1000.0).unwrap().tiles;
        near(narrow[0].1.width(), 394.0);
        near(narrow[1].1.width() / narrow[2].1.width(), 0.75);
        // Too narrow for the minimums: the page shows tabs.
        let least = 394.0 + 2.0 * TILE_MIN_WIDTH + 32.0;
        near(layout.min_size(ROOM).x, least);
        assert!(placed(&layout, least - 1.0, 900.0).is_none());
    }

    #[test]
    fn an_edge_moves_between_its_two_parts_inside_their_minimums() {
        let mut layout = three();
        let edges = placed(&layout, 1532.0, 1000.0).unwrap().edges;
        let edge = |axis: Axis, after| {
            edges
                .iter()
                .find(|edge| edge.axis == axis && edge.after == after)
                .unwrap()
                .clone()
        };
        layout.resize(&edge(Axis::Across, 0), Some(100.0), ROOM);
        near(rect(&layout, Plan).width(), 550.0);
        near(rect(&layout, Happened).width(), 350.0);
        // As far as the part after the edge allows.
        layout.resize(&edge(Axis::Across, 0), Some(1000.0), ROOM);
        near(rect(&layout, Happened).width(), TILE_MIN_WIDTH);
        layout.resize(&edge(Axis::Across, 0), None, ROOM);
        near(rect(&layout, Plan).width(), 450.0);
        layout.resize(&edge(Axis::Down, 0), Some(-1000.0), ROOM);
        near(rect(&layout, Threads).height(), TILE_MIN_HEIGHT);
        // The handle of the split inside stops short of the columns' handles.
        let inner = edge(Axis::Down, 0);
        near(inner.gap.left(), rect(&layout, Threads).left() + 4.0);
        near(inner.gap.height(), 14.0);
        assert_eq!(inner.before, [Threads]);
    }

    #[test]
    fn a_tile_goes_beside_another_or_along_an_edge_of_the_screen() {
        let mut layout = three();
        // Across the top: the plan over the other three.
        layout.place_tile(Plan, Place::Edge(DockSide::Top));
        assert_eq!(
            shape(&layout),
            "down[plan 0.333, across[what_happened 0.429, \
             down[open_threads 0.500, rest_of_note 0.500] 0.571] 0.667]"
        );
        near(rect(&layout, Plan).width(), 1532.0);
        // Beside a tile in a split the same way: the two share that tile's part. The split it
        // left had one part, so that part took its place.
        layout.place_tile(Rest, Place::Beside(Happened, DockSide::Left));
        assert_eq!(
            shape(&layout),
            "down[plan 0.333, across[rest_of_note 0.214, what_happened 0.214, \
             open_threads 0.571] 0.667]"
        );
        // Below a tile in a split the other way: that tile becomes a split.
        layout.place_tile(Threads, Place::Beside(Rest, DockSide::Bottom));
        assert_eq!(
            shape(&layout),
            "down[plan 0.333, across[down[rest_of_note 0.500, open_threads 0.500] 0.500, \
             what_happened 0.500] 0.667]"
        );
        assert!(!layout.can_place(Plan, Place::Beside(Plan, DockSide::Left)));
        // Along the left edge of a row of columns: one more column, not a row inside a row.
        let mut layout = three();
        layout.place_tile(Rest, Place::Edge(DockSide::Left));
        assert_eq!(
            shape(&layout),
            "across[rest_of_note 0.333, plan 0.200, what_happened 0.200, open_threads 0.267]"
        );
    }

    #[test]
    fn tiles_swap_hide_and_show_and_one_tile_always_shows() {
        let mut layout = three();
        layout.swap(Plan, Rest);
        assert_eq!(layout.tiles(), [Rest, Happened, Threads, Plan]);
        layout.hide(Happened);
        assert_eq!(layout.hidden(), [Happened]);
        layout.show(Happened);
        assert_eq!(layout.tiles(), [Rest, Threads, Plan, Happened]);
        for tile in [Plan, Threads, Rest] {
            layout.hide(tile);
        }
        assert!(!layout.can_hide(Happened));
        layout.hide(Happened);
        assert_eq!(layout, Layout::tile(Happened));
        assert!(!layout.can_place(Happened, Place::Edge(DockSide::Left)));
    }

    #[test]
    fn a_saved_layout_opens_with_what_it_can_read() {
        let mut layout = three();
        layout.place_tile(Plan, Place::Edge(DockSide::Top));
        let text = serde_json::to_string(&layout.to_saved()).unwrap();
        let read: DmScreenLayout = serde_json::from_str(&text).unwrap();
        assert_eq!(shape(&Layout::from_saved(&read).unwrap()), shape(&layout));
        let tile = |name: &str, share| SavedPart {
            share,
            node: SavedNode {
                tile: Some(name.into()),
                ..SavedNode::default()
            },
        };
        let odd = DmScreenLayout {
            root: Some(SavedNode {
                split: Some("across".into()),
                parts: vec![
                    tile("dice", 1.0),
                    tile(Plan.key(), f32::NAN),
                    SavedPart {
                        share: 2.0,
                        node: SavedNode {
                            split: Some("diagonal".into()),
                            parts: vec![tile(Rest.key(), 1.0)],
                            ..SavedNode::default()
                        },
                    },
                    tile(Plan.key(), 1.0),
                ],
                ..SavedNode::default()
            }),
        };
        assert_eq!(Layout::from_saved(&odd), Some(Layout::tile(Plan)));
        assert_eq!(Layout::from_saved(&DmScreenLayout::default()), None);
        // Shares too big to add up go even, never to NaN, which would not save.
        let huge = DmScreenLayout {
            root: Some(SavedNode {
                split: Some("across".into()),
                parts: vec![tile(Plan.key(), f32::MAX), tile(Happened.key(), f32::MAX)],
                ..SavedNode::default()
            }),
        };
        assert_eq!(
            shape(&Layout::from_saved(&huge).unwrap()),
            "across[plan 0.500, what_happened 0.500]"
        );
    }
}
