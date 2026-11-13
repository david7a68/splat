//! Ergonomics checks on the AccessKit tree, and the baseline that enforces them.
use crate::Splat;
use eframe::egui::{
    self,
    accesskit::{Action, Role},
};
use egui_kittest::{
    Harness, Node,
    kittest::{AccessKitNode, NodeT},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

/// WCAG 2.5.8 (Target Size, Minimum), in logical pixels.
pub const MIN_TARGET: f32 = 24.0;
/// A control can exceed a neighbour by rounding without a person noticing.
const OVERLAP_TOLERANCE: f32 = 1.0;
/// Panel and window resize handles, and scroll bars, are at most this thick. The tree
/// reports pixels, and a column's 10 point handle is 20 at twice the density.
const CHROME_THICKNESS: f32 = 20.0;
/// A window's resize corners are small in both directions.
const CHROME_CORNER: f32 = 24.0;

/// Violations on egui's own widgets use the check's name with this suffix. The
/// counts stay visible, but apart from what app code and the design system control.
pub const CHROME_SUFFIX: &str = "/egui-chrome";

/// An app target or text that overlaps egui chrome where egui's hit test can give the
/// press to the chrome: a scroll bar or a column splitter. It is an app fault.
pub const APP_VS_CHROME: &str = "no-overlap/app-vs-chrome";

/// No baseline may allow these. Loading or writing a positive allowance fails.
pub const NEVER_ALLOWED: [&str; 1] = [APP_VS_CHROME];

/// A scenario, a state in it, and a window size, as the report names them.
pub type StateKey<'a> = (&'a str, &'a str, &'a str);

/// The only app-vs-chrome pairs a run may have, with the exact count for each state. It is
/// empty: every app target and text clears every scroll bar and splitter. This list is code, not
/// `baseline.json`, so no baseline refresh can add to it, and it may only shrink: a state with
/// fewer pairs than listed fails the run until its line is removed or lowered.
pub const KNOWN_APP_VS_CHROME: &[(StateKey<'static>, usize)] = &[];

/// App-vs-chrome lines by state: a state that is not in `known` or has more pairs than
/// listed, then a listed state with fewer pairs (the list is stale). Both fail a run. A
/// scenario that did not run gives no line.
pub fn compare_app_vs_chrome(
    violations: &[Violation],
    known: &[(StateKey<'_>, usize)],
    scenarios: &[&str],
) -> (Vec<String>, Vec<String>) {
    let mut found: BTreeMap<StateKey<'_>, usize> = BTreeMap::new();
    for violation in violations.iter().filter(|v| v.check == APP_VS_CHROME) {
        let key = (
            violation.scenario.as_str(),
            violation.state.as_str(),
            violation.window.as_str(),
        );
        *found.entry(key).or_default() += 1;
    }
    let listed = |key: StateKey<'_>| {
        known
            .iter()
            .filter(|(listed, _)| *listed == key)
            .map(|(_, count)| count)
            .sum::<usize>()
    };
    let line = |(scenario, state, window): StateKey<'_>, count: usize, limit: usize| {
        format!("{APP_VS_CHROME} / {scenario} / {state} / {window}: {count} (known {limit})")
    };
    let worse = found
        .iter()
        .filter(|(key, count)| **count > listed(**key))
        .map(|(key, count)| line(*key, *count, listed(*key)))
        .collect();
    let stale = known
        .iter()
        .filter(|((scenario, ..), _)| scenarios.contains(scenario))
        .filter_map(|(key, limit)| {
            let count = found.get(key).copied().unwrap_or(0);
            (count < *limit).then(|| {
                format!(
                    "{}. Fixed: lower or remove its line in KNOWN_APP_VS_CHROME",
                    line(*key, count, *limit)
                )
            })
        })
        .collect();
    (worse, stale)
}

/// Help is `design::help`: `Small` muted words that wrap at `READER_MEASURE`. A muted `Small`
/// text wider than that measure is help that skipped it.
pub const HELP_MEASURE: &str = "help-measure";

pub const CHECKS: [&str; 13] = [
    "target-size",
    "accessible-name",
    "in-viewport",
    "no-overlap",
    "clipped",
    "keyboard",
    "target-size/egui-chrome",
    "accessible-name/egui-chrome",
    "no-overlap/egui-chrome",
    APP_VS_CHROME,
    "workflow",
    "missing-glyph",
    HELP_MEASURE,
];

#[derive(Clone, Debug, Serialize)]
pub struct Violation {
    pub check: String,
    pub scenario: String,
    pub state: String,
    pub window: String,
    pub node: String,
    pub role: String,
    /// x, y, width, height in logical pixels.
    pub rect: [f32; 4],
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Pass {
    pub check: String,
    pub scenario: String,
    pub state: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct TaskCost {
    pub scenario: String,
    pub goal: String,
    /// Interactions since the previous goal in the scenario.
    pub interactions: u32,
    /// Interactions since the scenario started.
    pub total: u32,
}

/// An interactive node, flattened for checking.
#[derive(Clone, Debug)]
pub struct Control {
    pub id: u64,
    /// The accessible name: the node's label, not a placeholder hint.
    pub name: String,
    pub placeholder: Option<String>,
    pub role: Role,
    /// An interactive ancestor that owns a label painted inside its bounds.
    pub label_owner: Option<u64>,
    pub rect: egui::Rect,
    pub disabled: bool,
    /// The top-level pane or window that holds this control.
    pub scope: u64,
    /// The visible part of the scroll area around this control, if any.
    pub clip: Option<egui::Rect>,
    /// A scroll bar, resize handle or window title bar that egui adds by itself.
    pub chrome: bool,
    /// The popup background (egui chrome) that this control or text is drawn in.
    pub popup: Option<u64>,
    /// An open modal dialog lies over this control's layer and takes its presses.
    pub blocked: bool,
}

fn rect_of(node: &AccessKitNode<'_>) -> Option<egui::Rect> {
    node.bounding_box().map(|rect| egui::Rect {
        min: egui::pos2(rect.x0 as f32, rect.y0 as f32),
        max: egui::pos2(rect.x1 as f32, rect.y1 as f32),
    })
}

/// Roles that accept input, even when disabled (egui then drops their actions).
fn control_role(role: Role) -> bool {
    matches!(
        role,
        Role::Button
            | Role::Link
            | Role::CheckBox
            | Role::RadioButton
            | Role::ComboBox
            | Role::TextInput
            | Role::MultilineTextInput
            | Role::PasswordInput
            | Role::Slider
            | Role::SpinButton
            | Role::ColorWell
            | Role::Tab
    )
}

fn supports(node: &AccessKitNode<'_>, action: Action) -> bool {
    node.data().supports_action(action)
}

fn thin(rect: egui::Rect) -> bool {
    rect.width().min(rect.height()) <= CHROME_THICKNESS
        || rect.width().max(rect.height()) <= CHROME_CORNER
}

/// A scroll area's drag surface fills its viewport and sits behind the content.
/// It is how the check finds scroll areas. It is not a target of its own.
fn scroll_surface(node: &AccessKitNode<'_>) -> Option<egui::Rect> {
    let rect = rect_of(node)?;
    (node.role() == Role::Unknown
        && !supports(node, Action::Click)
        && (supports(node, Action::Focus) || node.is_disabled())
        && !thin(rect))
    .then_some(rect)
}

/// Selectable text responds to the pointer, but it is reading content rather
/// than a control: its size follows the font, and its name comes from its text.
fn interactive(node: &AccessKitNode<'_>) -> bool {
    match node.role() {
        Role::Label | Role::TextRun | Role::GenericContainer => false,
        role => {
            control_role(role)
                || (scroll_surface(node).is_none()
                    && (supports(node, Action::Click) || supports(node, Action::Focus)))
        }
    }
}

/// Egui's own scroll bar: unnamed, with no role, thin, and following its surface
/// along the full length of an edge. A nested horizontal bar can float upward.
fn scroll_bar(node: &AccessKitNode<'_>, rect: egui::Rect, surfaces: &[egui::Rect]) -> bool {
    const SAME_EDGE: f32 = 1.0;
    node.role() == Role::Unknown
        && node.label().is_none()
        && rect.width().min(rect.height()) <= CHROME_THICKNESS
        && surfaces.iter().any(|surface| {
            let tall = (rect.top() - surface.top()).abs() <= SAME_EDGE
                && (rect.bottom() - surface.bottom()).abs() <= SAME_EDGE
                && rect.left() >= surface.right()
                && rect.left() <= surface.right() + CHROME_THICKNESS;
            let wide = (rect.left() - surface.left()).abs() <= SAME_EDGE
                && (rect.right() - surface.right()).abs() <= SAME_EDGE
                // Egui moves a nested bar up into the visible part of its parent.
                && rect.top() >= surface.top()
                && rect.top() <= surface.bottom() + CHROME_THICKNESS;
            tall || wide
        })
}

/// Egui column handles remain thin chrome. Splat names them as splitters and adds
/// keyboard control and visible focus. Ordinary controls cannot use this exception.
fn column_handle(node: &AccessKitNode<'_>, rect: egui::Rect) -> bool {
    const AS_TALL_AS_A_COLUMN: f32 = 100.0;
    node.role() == Role::Splitter
        && node
            .label()
            .is_some_and(|label| label.starts_with("Resize ") && label.ends_with(" column"))
        && supports(node, Action::Focus)
        && !supports(node, Action::Click)
        && rect.width() <= CHROME_THICKNESS
        && rect.height() >= AS_TALL_AS_A_COLUMN
}

/// A popup's own background: unnamed, with no role, directly under the window, and with
/// all the popup's contents inside it. A control is a leaf, so it cannot match.
fn popup_body(node: &Node<'_>, rect: egui::Rect, parent_is_window: bool) -> bool {
    let access = node.accesskit_node();
    access.role() == Role::Unknown
        && access.label().is_none()
        && parent_is_window
        && !thin(rect)
        && node.children().next().is_some()
        && rect.expand(1.0).contains_rect(extent(node))
}

fn extent(node: &Node<'_>) -> egui::Rect {
    let own = rect_of(&node.accesskit_node()).unwrap_or(egui::Rect::NOTHING);
    node.children()
        .fold(own, |rect, child| rect.union(extent(&child)))
}

/// Receives a node, its top-level scope, the scroll area clip around it, whether its
/// parent is a window, and the scroll surfaces beside it.
type Visit<'a, 't> = dyn FnMut(&Node<'t>, u64, Option<egui::Rect>, bool, &[egui::Rect]) + 'a;

/// Visit every node with its top-level scope and the scroll area clip around it.
fn walk<'t>(
    node: &Node<'t>,
    scope: u64,
    clip: Option<egui::Rect>,
    depth: usize,
    visit: &mut Visit<'_, 't>,
) {
    let surfaces: Vec<egui::Rect> = node
        .children()
        .filter_map(|child| scroll_surface(&child.accesskit_node()))
        .collect();
    let parent_is_window = node.accesskit_node().role() == Role::Window;
    // The surfaces already behind us. A surface comes after its own content.
    let mut passed = 0;
    for child in node.children() {
        let scope = if depth == 0 {
            child.accesskit_node().id().0
        } else {
            scope
        };
        let inner = if surfaces.is_empty() {
            clip
        } else {
            // Content is a sibling of the surface, and overlaps it. Scrolled content also
            // passes behind other areas on the page, which do not clip it.
            let bounds = extent(&child);
            surfaces[passed..]
                .iter()
                .chain(&surfaces[..passed])
                .find(|surface| surface.intersects(bounds))
                .map_or(clip, |surface| {
                    Some(clip.map_or(*surface, |outer| outer.intersect(*surface)))
                })
        };
        if scroll_surface(&child.accesskit_node()).is_some() {
            passed += 1;
        }
        // Content comes before its own surface; egui's bars come after it. Only
        // earlier surfaces can identify a bar, including one floated over content.
        visit(&child, scope, inner, parent_is_window, &surfaces[..passed]);
        walk(&child, scope, inner, depth + 1, visit);
    }
}

/// What the last pass said about rows: the labels `design::row_detail` marked, and the rects
/// that row components gave `design::register_row`.
pub struct RowDetails {
    marked: HashSet<egui::accesskit::NodeId>,
    rows: Vec<egui::Rect>,
}

impl RowDetails {
    pub fn of(ctx: &egui::Context) -> Self {
        let marked: Vec<egui::Id> = ctx
            .data(|data| {
                data.get_temp::<(u64, Vec<egui::Id>)>(egui::Id::new(crate::design::ROW_DETAILS))
            })
            .map(|(_, items)| items)
            .unwrap_or_default();
        let rows: Vec<egui::Rect> = ctx
            .data(|data| {
                data.get_temp::<(u64, Vec<egui::Rect>)>(egui::Id::new(crate::design::ROW_RECTS))
            })
            .map(|(_, items)| items)
            .unwrap_or_default();
        Self {
            marked: marked
                .iter()
                .map(|id| egui::accesskit::NodeId(id.value()))
                .collect(),
            rows,
        }
    }

    /// A row's own detail line: marked, and inside a registered row. `rect` is in points.
    pub fn owns(&self, id: egui::accesskit::NodeId, rect: egui::Rect) -> bool {
        self.marked.contains(&id)
            && self
                .rows
                .iter()
                .any(|row| row.expand(OVERLAP_TOLERANCE).contains_rect(rect))
    }
}

/// The rects that row components gave `design::register_row` in the last pass, in points.
pub fn row_rects(ctx: &egui::Context) -> Vec<egui::Rect> {
    RowDetails::of(ctx).rows
}

/// A `Label` node's words: egui puts them in its value.
pub fn label_words(node: &AccessKitNode<'_>) -> Option<String> {
    (node.role() == Role::Label)
        .then(|| node.value().or_else(|| node.label()))
        .flatten()
}

/// The words of every `Label` node that is not a row's, for `wide_help`. A label inside a control
/// is that control's content, and a label marked with `design::row_detail` that lies inside a
/// registered row is that row's own second line; neither is help.
pub fn label_texts(harness: &Harness<'_, Splat>) -> Vec<String> {
    let rows = RowDetails::of(&harness.ctx);
    let scale = 1.0 / harness.ctx.pixels_per_point();
    let mut found = Vec::new();
    visit_nodes(harness, |node, _| {
        let access = node.accesskit_node();
        if rect_of(&access).is_some_and(|rect| rows.owns(access.id(), rect * scale)) {
            return;
        }
        let mut parent = node.parent();
        while let Some(ancestor) = parent {
            if control_role(ancestor.accesskit_node().role()) {
                return;
            }
            parent = ancestor.parent();
        }
        found.extend(label_words(&node.accesskit_node()));
    });
    found
}

/// Every node with the scroll area clip around it.
pub fn visit_nodes<'t>(
    harness: &'t Harness<'_, Splat>,
    mut visit: impl FnMut(&Node<'t>, Option<egui::Rect>),
) {
    let root = harness.root();
    let id = root.accesskit_node().id().0;
    walk(&root, id, None, 0, &mut |node, _, clip, _, _| {
        visit(node, clip)
    });
}

/// A real horizontal bar proves that content cut at this surface's sides can be
/// revealed. Do not treat clipping in ordinary vertical readers this way.
pub fn horizontal_scroll_regions(harness: &Harness<'_, Splat>) -> Vec<egui::Rect> {
    let root = harness.root();
    let mut regions = Vec::new();
    walk(
        &root,
        root.accesskit_node().id().0,
        None,
        0,
        &mut |node, _, _, _, surfaces| {
            let access = node.accesskit_node();
            if let Some(rect) = rect_of(&access)
                && rect.width() > rect.height()
            {
                for surface in surfaces {
                    if scroll_bar(&access, rect, &[*surface]) {
                        regions.push(*surface);
                    }
                }
            }
        },
    );
    regions
}

fn can_scroll_sideways(rect: egui::Rect, clip: egui::Rect, regions: &[egui::Rect]) -> bool {
    regions.iter().any(|region| {
        region.intersects(rect)
            && (region.left() - clip.left()).abs() <= crate::design::XS
            && (region.right() - clip.right()).abs() <= crate::design::XS
    })
}

/// Lookup may shorten only a result's muted detail. The row must retain its
/// complete text in accessibility, and its name must remain fully visible.
pub fn lookup_results(harness: &Harness<'_, Splat>) -> Vec<(String, egui::Rect)> {
    let mut rows = Vec::new();
    visit_nodes(harness, |node, _| {
        let access = node.accesskit_node();
        if access.data().role_description() == Some("Lookup result")
            && let Some(rect) = rect_of(&access)
            && let Some(label) = access.label()
        {
            rows.push((label, rect));
        }
    });
    rows
}

/// Controls, and labels for the overlap and clipping checks. See `Control::is_label`.
pub fn controls(harness: &Harness<'_, Splat>) -> Vec<Control> {
    let mut out = Vec::new();
    let root = harness.root();
    let id = root.accesskit_node().id().0;
    // Popup backgrounds first, so each control can name the one it is drawn in.
    let mut popups = HashSet::new();
    walk(
        &root,
        id,
        None,
        0,
        &mut |node, _, _, parent_is_window, _| {
            if let Some(rect) = rect_of(&node.accesskit_node())
                && popup_body(node, rect, parent_is_window)
            {
                popups.insert(node.accesskit_node().id().0);
            }
        },
    );
    let popup_of = |node: &Node<'_>| {
        let mut parent = node.parent();
        while let Some(ancestor) = parent {
            let id = ancestor.accesskit_node().id().0;
            if popups.contains(&id) {
                return Some(id);
            }
            parent = ancestor.parent();
        }
        None
    };
    // A modal dialog blocks every layer below its own. AccessKit ids are egui's ids.
    let layers: HashMap<u64, egui::LayerId> = harness.ctx.viewport(|viewport| {
        [&viewport.prev_pass, &viewport.this_pass]
            .into_iter()
            .flat_map(|pass| pass.widgets.layers())
            .flat_map(|(layer, widgets)| widgets.iter().map(|widget| (widget.id.value(), *layer)))
            .collect()
    });
    let blocked = |node: &AccessKitNode<'_>| {
        layers.get(&node.id().0).is_some_and(|layer| {
            !harness
                .ctx
                .memory(|memory| memory.allows_interaction(*layer))
        })
    };
    walk(&root, id, None, 0, &mut |node, scope, clip, _, surfaces| {
        let access = node.accesskit_node();
        if access.role() == Role::Label
            && let Some(rect) = rect_of(&access)
        {
            let mut parent = node.parent();
            let mut label_owner = None;
            while let Some(ancestor) = parent {
                let access = ancestor.accesskit_node();
                if control_role(access.role()) {
                    label_owner = Some(access.id().0);
                    break;
                }
                parent = ancestor.parent();
            }
            out.push(Control {
                id: access.id().0,
                name: access.label().unwrap_or_default(),
                placeholder: None,
                role: Role::Label,
                label_owner,
                rect,
                disabled: false,
                scope,
                clip,
                chrome: false,
                popup: popup_of(node),
                blocked: false,
            });
            return;
        }
        if !interactive(&access) {
            return;
        }
        let Some(rect) = rect_of(&access) else {
            return;
        };
        out.push(Control {
            id: access.id().0,
            name: access
                .label()
                .filter(|label| !label.trim().is_empty())
                .unwrap_or_default(),
            placeholder: access.placeholder().map(str::to_owned),
            role: access.role(),
            label_owner: None,
            rect,
            disabled: access.is_disabled(),
            scope,
            clip,
            chrome: scroll_bar(&access, rect, surfaces)
                || column_handle(&access, rect)
                || popups.contains(&access.id().0),
            popup: popup_of(node),
            blocked: blocked(&access),
        });
    });
    out
}

/// Text that may lose its end, each with the reason.
const ELISION_ALLOWED: &[(&str, &str)] = &[(
    // The fixture's folder. A path in a one-line field scrolls inside the field.
    "/tmp/splat-visual-world",
    "a folder path in a text field",
)];

fn opaque_panes(shape: &egui::Shape, clip: egui::Rect, panes: &mut Vec<egui::Rect>) {
    match shape {
        egui::Shape::Vec(items) => {
            for item in items {
                opaque_panes(item, clip, panes);
            }
        }
        egui::Shape::Rect(pane) if pane.fill.a() == 255 => {
            let rect = pane.rect.intersect(clip);
            if rect.width() >= 100.0
                && rect.height() >= 100.0
                && (pane.rect.left() <= clip.left() + OVERLAP_TOLERANCE
                    || pane.rect.right() >= clip.right() - OVERLAP_TOLERANCE)
                && (pane.rect.top() <= clip.top() + OVERLAP_TOLERANCE
                    || pane.rect.bottom() >= clip.bottom() - OVERLAP_TOLERANCE)
            {
                panes.push(rect);
            }
        }
        _ => {}
    }
}

fn pane_covers_edge(rect: egui::Rect, text: egui::Rect) -> bool {
    // Whole windows and popups intentionally hide their background. A sibling pane
    // clips one edge of text whose other edge is still visible.
    rect.top() <= text.top()
        && rect.bottom() >= text.bottom()
        && rect.intersect(text).width() > OVERLAP_TOLERANCE
        && !rect.contains_rect(text)
}

/// Painted text that lost its end to an ellipsis, or runs past the edge of its pane. A
/// tooltip does not help a DM reading across a table, so all of it is reported, whatever
/// painted it, except the named field above and the specified Lookup detail suffix.
pub fn cut_text(
    shapes: &[egui::epaint::ClippedShape],
    horizontal_regions: &[egui::Rect],
    lookup_results: &[(String, egui::Rect)],
    scenario: &str,
    state: &str,
    window: &str,
) -> Vec<Violation> {
    fn search(
        shape: &egui::Shape,
        clip: egui::Rect,
        horizontal_regions: &[egui::Rect],
        found: &mut Vec<(String, egui::Rect, &'static str)>,
    ) {
        match shape {
            egui::Shape::Vec(shapes) => shapes
                .iter()
                .for_each(|shape| search(shape, clip, horizontal_regions, found)),
            egui::Shape::Text(text) => {
                let bounds = text.visual_bounding_rect();
                let words = text.galley.job.text.clone();
                if text.galley.elided {
                    found.push((words, bounds, "Cut to an ellipsis"));
                } else if bounds.intersects(clip)
                    && !can_scroll_sideways(bounds, clip, horizontal_regions)
                    && (bounds.right() > clip.right() + OVERLAP_TOLERANCE
                        || bounds.left() < clip.left() - OVERLAP_TOLERANCE)
                {
                    found.push((words, bounds, "Runs past the edge of its pane"));
                }
            }
            _ => {}
        }
    }
    let mut found = Vec::new();
    let panes: Vec<_> = shapes
        .iter()
        .enumerate()
        .flat_map(|(index, clipped)| {
            let mut panes = Vec::new();
            opaque_panes(&clipped.shape, clipped.clip_rect, &mut panes);
            panes.into_iter().map(move |rect| (index, rect))
        })
        .collect();
    for (index, clipped) in shapes.iter().enumerate() {
        search(
            &clipped.shape,
            clipped.clip_rect,
            horizontal_regions,
            &mut found,
        );
        // A later opaque pane can hide an edge even when the text's own clip is wide.
        fn covered(
            shape: &egui::Shape,
            clip: egui::Rect,
            later: &[(usize, egui::Rect)],
            index: usize,
            found: &mut Vec<(String, egui::Rect, &'static str)>,
        ) {
            match shape {
                egui::Shape::Vec(items) => {
                    for item in items {
                        covered(item, clip, later, index, found);
                    }
                }
                egui::Shape::Text(text) => {
                    let bounds = text.visual_bounding_rect().intersect(clip);
                    if bounds.is_positive()
                        && later
                            .iter()
                            .any(|(at, pane)| *at > index && pane_covers_edge(*pane, bounds))
                    {
                        found.push((
                            text.galley.job.text.clone(),
                            bounds,
                            "Covered by a later pane",
                        ));
                    }
                }
                _ => {}
            }
        }
        covered(&clipped.shape, clipped.clip_rect, &panes, index, &mut found);
    }
    found
        .into_iter()
        .filter(|(text, rect, how)| {
            !(*how == "Cut to an ellipsis"
                && text.starts_with(" · ")
                && lookup_results
                    .iter()
                    .any(|(full, row)| full.ends_with(text) && row.contains_rect(*rect)))
        })
        .filter(|(text, ..)| {
            !ELISION_ALLOWED
                .iter()
                .any(|(start, _)| text.starts_with(start))
        })
        .map(|(text, rect, how)| Violation {
            check: "clipped".into(),
            scenario: scenario.to_owned(),
            state: state.to_owned(),
            window: window.to_owned(),
            node: text.chars().take(40).collect(),
            role: "Text".into(),
            rect: [rect.left(), rect.top(), rect.width(), rect.height()],
            detail: format!("{how}: {text}"),
        })
        .collect()
}

/// Every painted character must exist in its font family, including its fallbacks.
pub fn missing_glyphs(
    ctx: &egui::Context,
    shapes: &[egui::epaint::ClippedShape],
    scenario: &str,
    state: &str,
    window: &str,
) -> Vec<Violation> {
    fn visit(
        ctx: &egui::Context,
        shape: &egui::Shape,
        clip: egui::Rect,
        found: &mut Vec<(char, egui::Rect, String)>,
    ) {
        match shape {
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    visit(ctx, shape, clip, found);
                }
            }
            egui::Shape::Text(text) if text.visual_bounding_rect().intersects(clip) => {
                for section in &text.galley.job.sections {
                    let words = &text.galley.job.text[section.byte_range.clone()];
                    for character in words
                        .chars()
                        .filter(|c| !c.is_control() && !c.is_whitespace())
                    {
                        if !ctx
                            .fonts_mut(|fonts| fonts.has_glyph(&section.format.font_id, character))
                        {
                            found.push((
                                character,
                                text.visual_bounding_rect(),
                                text.galley.job.text.clone(),
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    let mut found = Vec::new();
    for shape in shapes {
        visit(ctx, &shape.shape, shape.clip_rect, &mut found);
    }
    found
        .into_iter()
        .map(|(character, rect, words)| Violation {
            check: "missing-glyph".into(),
            scenario: scenario.into(),
            state: state.into(),
            window: window.into(),
            node: words.chars().take(40).collect(),
            role: "Text".into(),
            rect: [rect.left(), rect.top(), rect.width(), rect.height()],
            detail: format!("No glyph for U+{:04X} in {words:?}", character as u32),
        })
        .collect()
}

/// Every painted label (its words are those of a `Label` node in `labels`: not a row's) that is
/// all muted `Small` words and wider than `READER_MEASURE`: help that does not wrap at the
/// reader's measure (`design::help`). A text with any other size, family or colour in it (a
/// status word, a link, a value) is not help. A label that does not wrap (`truncate`, `extend`)
/// and stands alone is help too: only a row's own detail line, proved by the control around it or
/// marked with `design::row_detail`, passes.
pub fn wide_help(
    ctx: &egui::Context,
    shapes: &[egui::epaint::ClippedShape],
    labels: &HashSet<&str>,
    scenario: &str,
    state: &str,
    window: &str,
) -> Vec<Violation> {
    use crate::design::{READER_MEASURE, Type, colors};
    let small = ctx.style().text_styles.get(&Type::Small.style()).cloned();
    let muted = colors().muted;
    let help = |text: &egui::epaint::TextShape| {
        let sections = &text.galley.job.sections;
        labels.contains(text.galley.job.text.as_str())
            && !sections.is_empty()
            && sections.iter().all(|section| {
                let color = if section.format.color == egui::Color32::PLACEHOLDER {
                    text.fallback_color
                } else {
                    section.format.color
                };
                Some(&section.format.font_id) == small.as_ref() && color == muted
            })
    };
    fn visit(
        shape: &egui::Shape,
        clip: egui::Rect,
        help: &dyn Fn(&egui::epaint::TextShape) -> bool,
        found: &mut Vec<(String, egui::Rect)>,
    ) {
        match shape {
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    visit(shape, clip, help, found);
                }
            }
            egui::Shape::Text(text)
                if text.visual_bounding_rect().intersects(clip)
                    && text.visual_bounding_rect().width() > READER_MEASURE + OVERLAP_TOLERANCE
                    && help(text) =>
            {
                found.push((text.galley.job.text.clone(), text.visual_bounding_rect()));
            }
            _ => {}
        }
    }
    let mut found = Vec::new();
    for shape in shapes {
        visit(&shape.shape, shape.clip_rect, &help, &mut found);
    }
    found
        .into_iter()
        .map(|(words, rect)| Violation {
            check: HELP_MEASURE.into(),
            scenario: scenario.into(),
            state: state.into(),
            window: window.into(),
            node: words.chars().take(40).collect(),
            role: "Text".into(),
            rect: [rect.left(), rect.top(), rect.width(), rect.height()],
            detail: format!(
                "Muted Small words {:.0} wide, past the {READER_MEASURE:.0} reader measure. Use design::help: {words}",
                rect.width()
            ),
        })
        .collect()
}

impl Control {
    /// Labels are not targets. They are here so a control lying on text is found.
    pub fn is_label(&self) -> bool {
        self.role == Role::Label
    }
}

fn display_name(control: &Control) -> String {
    if !control.name.is_empty() {
        control.name.clone()
    } else if let Some(hint) = &control.placeholder {
        format!("(unnamed {:?}, hint: {hint})", control.role)
    } else if control.chrome {
        format!(
            "(egui {} at edge)",
            if control.role == Role::Unknown {
                "handle"
            } else {
                "control"
            }
        )
    } else {
        format!("(unnamed {:?})", control.role)
    }
}

/// A check on one control: egui's own widgets count apart from the app's.
fn own_check(check: &str, control: &Control) -> String {
    if control.chrome {
        format!("{check}{CHROME_SUFFIX}")
    } else {
        check.to_owned()
    }
}

/// The check that an overlap of two controls (or a control and text) belongs to.
///
/// Two app controls are an app fault, and two pieces of egui chrome are egui's own count.
/// For a mixed pair, egui's hit test decides. A popup's background is registered before
/// the items in it, so in its own popup an enabled item is on top and takes every press.
/// A disabled item takes no press, so the press reaches the background, which has no
/// action, as it does in the popup's margin. That pair stays egui's count. Any other
/// chrome (a scroll bar or a column splitter) can take the press from the app control or
/// cover its text: that is an app fault, whatever its size.
fn overlap_check(first: &Control, second: &Control) -> String {
    let (app, chrome) = match (first.chrome, second.chrome) {
        (false, false) => return "no-overlap".to_owned(),
        (true, true) => return format!("no-overlap{CHROME_SUFFIX}"),
        (false, true) => (first, second),
        (true, false) => (second, first),
    };
    if app.popup == Some(chrome.id) {
        format!("no-overlap{CHROME_SUFFIX}")
    } else {
        APP_VS_CHROME.to_owned()
    }
}

/// A popup is a top-level scope of its own, drawn over the panes below it, so the pair
/// check does not usually compare it with them. Its content (a choice or its text) over
/// a column splitter or a scroll bar of another scope is still an app fault: the content
/// hides the handle, or the handle shows at the edge of a choice. A handle that an open
/// modal dialog blocks is not a handle there: the dialog lies over it, and it takes no
/// press. Other pairs across scopes, the popup's own background among them, stay out of
/// the check.
fn popup_content_over_handle(first: &Control, second: &Control) -> bool {
    let content = |control: &Control| !control.chrome && control.popup.is_some();
    // Chrome is a scroll bar (thin), a column splitter or a popup background (never thin).
    let handle = |control: &Control| {
        control.chrome && !control.blocked && (control.role == Role::Splitter || thin(control.rect))
    };
    (content(first) && handle(second)) || (content(second) && handle(first))
}

/// Run the per-state checks over the controls in the tree.
pub fn inspect(
    controls: &[Control],
    horizontal_regions: &[egui::Rect],
    viewport: egui::Rect,
    scenario: &str,
    state: &str,
    window: &str,
) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut report = |check: String, control: &Control, detail: String| {
        violations.push(Violation {
            check,
            scenario: scenario.to_owned(),
            state: state.to_owned(),
            window: window.to_owned(),
            node: display_name(control),
            role: format!("{:?}", control.role),
            rect: [
                control.rect.min.x,
                control.rect.min.y,
                control.rect.width(),
                control.rect.height(),
            ],
            detail,
        });
    };
    // What a person can see of a control, after scrolling and the window edge.
    let shown = |control: &Control| {
        let rect = control
            .clip
            .map_or(control.rect, |clip| control.rect.intersect(clip));
        rect.intersect(viewport)
    };
    let on_screen = |rect: egui::Rect| rect.is_positive();
    for control in controls {
        let visible = on_screen(shown(control));
        // Text cut off at the side of its pane cannot be read, and no scroll reveals it.
        if visible
            && !control.chrome
            && !control
                .clip
                .is_some_and(|clip| can_scroll_sideways(control.rect, clip, horizontal_regions))
            && (control.rect.left() < shown(control).left() - OVERLAP_TOLERANCE
                || control.rect.right() > shown(control).right() + OVERLAP_TOLERANCE)
        {
            report(
                own_check("clipped", control),
                control,
                format!(
                    "{:.0} wide, {:.0} shown. It is cut off at the side",
                    control.rect.width(),
                    shown(control).width()
                ),
            );
        }
        if control.is_label() {
            continue;
        }
        let enabled = if control.disabled { " (disabled)" } else { "" };
        if visible && (control.rect.width() < MIN_TARGET || control.rect.height() < MIN_TARGET) {
            report(
                own_check("target-size", control),
                control,
                format!(
                    "{:.1}x{:.1}, needs {MIN_TARGET}x{MIN_TARGET}{enabled}",
                    control.rect.width(),
                    control.rect.height()
                ),
            );
        }
        if visible && control.name.trim().is_empty() {
            report(
                own_check("accessible-name", control),
                control,
                format!("No label{enabled}"),
            );
        }
        // A disabled control behind a dialog is not a target, and it has no scroll node.
        if !control.disabled && control.clip.is_none() && !viewport.contains_rect(control.rect) {
            report(
                own_check("in-viewport", control),
                control,
                format!("Outside the {window} window, and not in a scroll area{enabled}"),
            );
        }
    }
    for (index, first) in controls.iter().enumerate() {
        let first_shown = shown(first);
        if !on_screen(first_shown) {
            continue;
        }
        for second in &controls[index + 1..] {
            if (first.scope != second.scope && !popup_content_over_handle(first, second))
                || (first.is_label() && second.is_label())
            {
                continue;
            }
            let overlap = first_shown.intersect(shown(second));
            // A link or a mark inside a row is the row's second target, meant to be on top.
            let nested = |inner: &Control, outer: &Control| {
                matches!(inner.role, Role::Link | Role::CheckBox)
                    && outer.rect.contains_rect(inner.rect)
            };
            // A row can own real status widgets. Its own child labels are its content;
            // labels beside it still participate in the overlap check.
            let owned_label = |inner: &Control, outer: &Control| {
                inner.is_label()
                    && inner.label_owner == Some(outer.id)
                    && outer.rect.contains_rect(inner.rect)
            };
            if nested(first, second)
                || nested(second, first)
                || owned_label(first, second)
                || owned_label(second, first)
            {
                continue;
            }
            if overlap.width() > OVERLAP_TOLERANCE && overlap.height() > OVERLAP_TOLERANCE {
                report(
                    overlap_check(first, second),
                    first,
                    format!(
                        "Overlaps {} by {:.1}x{:.1}",
                        display_name(second),
                        overlap.width(),
                        overlap.height()
                    ),
                );
            }
        }
    }
    violations
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Baseline {
    /// Check, then scenario, then the number of known violations.
    pub violations: BTreeMap<String, BTreeMap<String, usize>>,
    /// Scenario, then goal, then the interactions it may take.
    pub task_cost: BTreeMap<String, BTreeMap<String, u32>>,
}

impl Baseline {
    pub fn measure(violations: &[Violation], costs: &[TaskCost]) -> Self {
        let mut baseline = Self::default();
        for violation in violations {
            *baseline
                .violations
                .entry(violation.check.clone())
                .or_default()
                .entry(violation.scenario.clone())
                .or_default() += 1;
        }
        for cost in costs {
            baseline
                .task_cost
                .entry(cost.scenario.clone())
                .or_default()
                .insert(cost.goal.clone(), cost.interactions);
        }
        baseline
    }

    /// Replace only selected scenarios' chrome counts, including counts now zero.
    /// App violations, task costs and unselected scenarios retain their allowances.
    pub fn with_chrome_from(mut self, current: &Self, scenarios: &[&str]) -> Self {
        for check in CHECKS.iter().filter(|check| check.ends_with(CHROME_SUFFIX)) {
            let counts = self.violations.entry((*check).to_owned()).or_default();
            for &scenario in scenarios {
                counts.remove(scenario);
                if let Some(&count) = current
                    .violations
                    .get(*check)
                    .and_then(|counts| counts.get(scenario))
                    && count != 0
                {
                    counts.insert(scenario.to_owned(), count);
                }
            }
        }
        self.violations
            .retain(|check, counts| !check.ends_with(CHROME_SUFFIX) || !counts.is_empty());
        self
    }

    /// Read a baseline and refuse any allowance for a check in `NEVER_ALLOWED`.
    pub fn parse(text: &[u8]) -> Result<Self, String> {
        let baseline: Self =
            serde_json::from_slice(text).map_err(|error| format!("invalid baseline: {error}"))?;
        baseline.refuse_never_allowed()?;
        Ok(baseline)
    }

    /// An error that names each positive count for a check that no baseline may allow.
    pub fn refuse_never_allowed(&self) -> Result<(), String> {
        let found: Vec<String> = NEVER_ALLOWED
            .iter()
            .filter_map(|check| Some((check, self.violations.get(*check)?)))
            .flat_map(|(check, counts)| {
                counts
                    .iter()
                    .filter(|(_, count)| **count > 0)
                    .map(move |(scenario, count)| format!("{check} / {scenario}: {count}"))
            })
            .collect();
        if found.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "No baseline may allow an overlap where egui chrome can take the press. Fix \
                 the clearance instead:\n{}",
                found.join("\n")
            ))
        }
    }

    /// Lines where `current` is worse than this baseline, then lines where it is better.
    /// A scenario that did not run is neither. A check in `NEVER_ALLOWED` is not compared
    /// here, whatever this baseline says: `compare_app_vs_chrome` compares it by state.
    pub fn compare(&self, current: &Self, scenarios: &[&str]) -> (Vec<String>, Vec<String>) {
        let mut worse = Vec::new();
        let mut better = Vec::new();
        let empty = BTreeMap::new();
        for check in CHECKS.iter().filter(|check| !NEVER_ALLOWED.contains(check)) {
            let known = self.violations.get(*check).unwrap_or(&empty);
            let found = current.violations.get(*check).unwrap_or(&empty);
            for scenario in scenarios {
                let known = known.get(*scenario).copied().unwrap_or(0);
                let found = found.get(*scenario).copied().unwrap_or(0);
                if found > known {
                    worse.push(format!("{check} / {scenario}: {found} (baseline {known})"));
                } else if found < known {
                    better.push(format!("{check} / {scenario}: {found} (baseline {known})"));
                }
            }
        }
        for scenario in scenarios {
            let no_goals = BTreeMap::new();
            let known = self.task_cost.get(*scenario).unwrap_or(&no_goals);
            let found = current.task_cost.get(*scenario).unwrap_or(&no_goals);
            for (goal, &cost) in found {
                match known.get(goal) {
                    None => worse.push(format!(
                        "task-cost / {scenario} / {goal}: {cost} (no baseline)"
                    )),
                    Some(&limit) if cost > limit => worse.push(format!(
                        "task-cost / {scenario} / {goal}: {cost} (baseline {limit})"
                    )),
                    Some(&limit) if cost < limit => better.push(format!(
                        "task-cost / {scenario} / {goal}: {cost} (baseline {limit})"
                    )),
                    Some(_) => {}
                }
            }
            for goal in known.keys().filter(|goal| !found.contains_key(*goal)) {
                worse.push(format!(
                    "task-cost / {scenario} / {goal}: goal is no longer reached"
                ));
            }
        }
        (worse, better)
    }
}

#[derive(Serialize)]
pub struct Report<'a> {
    pub videos: &'a [String],
    pub not_measured: &'a [String],
    pub look_review: &'a [String],
    pub totals: BTreeMap<String, usize>,
    pub by_check_and_scenario: &'a BTreeMap<String, BTreeMap<String, usize>>,
    pub task_cost: &'a [TaskCost],
    pub passes: &'a [Pass],
    pub unreached: &'a [String],
    pub failed_scenarios: &'a [String],
    pub regressions: &'a [String],
    pub improvements: &'a [String],
    /// The listed app-vs-chrome pairs in the scenarios that ran. See `KNOWN_APP_VS_CHROME`.
    pub known_open: &'a [String],
    pub violations: &'a [Violation],
}

pub fn write_markdown(report: &Report<'_>) -> String {
    let mut text = String::from("# Ergonomics report\n\n");
    text.push_str("Generated by the visual suite. See `src/visual_tests/baseline.json`.\n\n");
    text.push_str("## Totals\n\n| Check | Violations |\n|---|---|\n");
    for check in CHECKS {
        text.push_str(&format!(
            "| {check} | {} |\n",
            report.totals.get(check).copied().unwrap_or(0)
        ));
    }
    text.push('\n');
    let mut list = |title: &str, lines: &[String]| {
        if !lines.is_empty() {
            text.push_str(&format!("## {title}\n\n"));
            for line in lines {
                text.push_str(&format!("- {line}\n"));
            }
            text.push('\n');
        }
    };
    list("Failed scenarios", report.failed_scenarios);
    list("Regressions against the baseline", report.regressions);
    list("Improvements against the baseline", report.improvements);
    list(
        "Known app-vs-chrome pairs (the list must end empty)",
        report.known_open,
    );
    list(
        "States and journeys the suite could not reach",
        report.unreached,
    );
    list("Workflow tests not measured", report.not_measured);
    list("Workflow tests for human review", report.look_review);
    list("Videos", report.videos);
    text.push_str(
        "## Violations by check and scenario\n\n| Check | Scenario | Count |\n|---|---|---|\n",
    );
    for (check, scenarios) in report.by_check_and_scenario {
        for (scenario, count) in scenarios {
            text.push_str(&format!("| {check} | {scenario} | {count} |\n"));
        }
    }
    text.push_str("\n## Task cost\n\n| Scenario | Goal | Interactions | Scenario total |\n|---|---|---|---|\n");
    for cost in report.task_cost {
        text.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            cost.scenario, cost.goal, cost.interactions, cost.total
        ));
    }
    text.push_str("\n## Checks that passed (keyboard and workflow)\n\n");
    for pass in report.passes {
        text.push_str(&format!(
            "- {} / {}: {}\n",
            pass.scenario, pass.state, pass.detail
        ));
    }
    text.push_str("\n## Violations\n\n");
    // The same control fails at each size, so group the rows to keep them readable.
    let mut groups: Vec<(String, Vec<&Violation>)> = Vec::new();
    let mut index = HashMap::new();
    for violation in report.violations {
        let key = format!(
            "{} / {} / {}",
            violation.check, violation.scenario, violation.state
        );
        let slot = *index.entry(key.clone()).or_insert_with(|| {
            groups.push((key, Vec::new()));
            groups.len() - 1
        });
        groups[slot].1.push(violation);
    }
    for (title, violations) in groups {
        text.push_str(&format!(
            "### {title}\n\n| Window | Node | Role | Rect | Detail |\n|---|---|---|---|---|\n"
        ));
        for v in violations {
            text.push_str(&format!(
                "| {} | {} | {} | {:.0},{:.0} {:.0}x{:.0} | {} |\n",
                v.window,
                v.node.replace('|', "\\|").replace('\n', " "),
                v.role,
                v.rect[0],
                v.rect[1],
                v.rect[2],
                v.rect[3],
                v.detail.replace('|', "\\|").replace('\n', " "),
            ));
        }
        text.push('\n');
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::{Harness, kittest::Queryable};

    #[test]
    fn chrome_refresh_preserves_unselected_and_app_allowances_and_clears_zeroes() {
        let known: Baseline = serde_json::from_value(serde_json::json!({
            "violations": {
                "target-size/egui-chrome": {"party": 9, "settings": 4},
                "accessible-name/egui-chrome": {"party": 3},
                "target-size": {"party": 2},
                "workflow": {"party": 1}
            },
            "task_cost": {"party": {"goal": 7}, "settings": {"goal": 3}}
        }))
        .unwrap();
        let current: Baseline = serde_json::from_value(serde_json::json!({
            "violations": {
                "target-size/egui-chrome": {"party": 12, "settings": 100},
                "target-size": {"party": 200},
                "no-overlap/egui-chrome": {"party": 1}
            },
            "task_cost": {"party": {"goal": 100}}
        }))
        .unwrap();
        let updated = known.with_chrome_from(&current, &["party"]);
        assert_eq!(updated.violations["target-size/egui-chrome"]["party"], 12);
        assert_eq!(updated.violations["target-size/egui-chrome"]["settings"], 4);
        assert!(
            !updated
                .violations
                .contains_key("accessible-name/egui-chrome")
        );
        assert_eq!(updated.violations["no-overlap/egui-chrome"]["party"], 1);
        assert_eq!(updated.violations["target-size"]["party"], 2);
        assert_eq!(updated.violations["workflow"]["party"], 1);
        assert_eq!(updated.task_cost["party"]["goal"], 7);
        assert_eq!(updated.task_cost["settings"]["goal"], 3);
    }

    #[test]
    fn a_column_handle_is_chrome() {
        let mut harness = Harness::builder().build_state(
            |ctx, _: &mut crate::Splat| {
                crate::design::Column::left("c", 200.0, 150.0..=300.0).show(ctx, |_| {});
                egui::CentralPanel::default().show(ctx, |_| {});
            },
            crate::Splat::default(),
        );
        harness.run_steps(2);
        let found = controls(&harness);
        assert_eq!(
            found.iter().filter(|control| control.chrome).count(),
            1,
            "{found:?}"
        );
        assert_eq!(found[0].role, Role::Splitter);
        assert_eq!(found[0].name, "Resize side panel column");
    }

    /// A real control can never count as egui chrome, however small it is.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn a_small_control_without_a_name_is_not_chrome() {
        let app = crate::Splat {
            campaigns: crate::campaigns::Campaigns::default(),
            ..Default::default()
        };
        let harness = Harness::builder().build_state(
            |ctx, _: &mut crate::Splat| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.set_min_height(4000.0);
                        // Thin, unnamed, and tall, right on the scroll area's edge.
                        let (_, response) =
                            ui.allocate_exact_size(egui::vec2(10.0, 300.0), egui::Sense::click());
                        response.widget_info(|| {
                            egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "")
                        });
                    });
                });
            },
            app,
        );
        let found = controls(&harness);
        let small: Vec<_> = found
            .iter()
            .filter(|control| control.role == Role::Button && control.rect.width() < 24.0)
            .collect();
        assert!(!small.is_empty(), "the test control should be in the tree");
        assert!(small.iter().all(|control| !control.chrome));
    }

    /// Even with no role and no name, a thin control in scroll content is not a scroll
    /// bar, wherever it lies. The real bar beside it still is.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn only_the_real_scroll_bar_is_chrome() {
        let harness = Harness::builder().build_state(
            |ctx, _: &mut crate::Splat| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The bar the app uses, which has its own room beside the content.
                    ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.set_min_height(4000.0);
                        let room = ui.available_width() - 10.0;
                        ui.horizontal(|ui| {
                            ui.add_space(room);
                            ui.allocate_exact_size(egui::vec2(10.0, 300.0), egui::Sense::click());
                        });
                    });
                });
            },
            crate::Splat::default(),
        );
        // A scroll area only knows it overflows after its first frame.
        let mut harness = harness;
        harness.run_steps(3);
        let found = controls(&harness);
        let thin_controls: Vec<_> = found
            .iter()
            .filter(|control| !control.is_label() && control.rect.width() <= CHROME_THICKNESS)
            .collect();
        assert_eq!(
            thin_controls.len(),
            2,
            "the bar and the test control: {thin_controls:?}"
        );
        let chrome = thin_controls
            .iter()
            .filter(|control| control.chrome)
            .count();
        assert_eq!(chrome, 1, "only the bar is chrome: {thin_controls:?}");
        assert!(
            thin_controls
                .iter()
                .any(|control| !control.chrome && (control.rect.height() - 300.0).abs() < 1.0),
            "{thin_controls:?}"
        );
    }

    /// An unnamed control in a popup is a control, not the popup's background.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn a_control_in_a_popup_is_not_its_background() {
        let mut harness = Harness::builder().build_state(
            |ctx, _: &mut crate::Splat| {
                egui::Window::new("Popup").show(ctx, |ui| {
                    ui.allocate_exact_size(egui::vec2(100.0, 40.0), egui::Sense::click());
                    ui.label("Text under it");
                });
            },
            crate::Splat::default(),
        );
        harness.run_steps(3);
        let found = controls(&harness);
        let control = found
            .iter()
            .find(|control| {
                (control.rect.width() - 100.0).abs() < 1.0
                    && (control.rect.height() - 40.0).abs() < 1.0
            })
            .expect("the test control should be in the tree");
        assert!(!control.chrome, "{control:?}");
    }

    fn control(name: &str, rect: egui::Rect) -> Control {
        Control {
            id: rect.left() as u64 + 1,
            name: name.to_owned(),
            placeholder: None,
            role: Role::Button,
            label_owner: None,
            rect,
            disabled: false,
            scope: 1,
            clip: None,
            chrome: false,
            popup: None,
            blocked: false,
        }
    }

    fn checks_for(controls: &[Control]) -> Vec<String> {
        let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 300.0));
        inspect(controls, &[], viewport, "seeded", "state", "400x300")
            .into_iter()
            .map(|violation| violation.check)
            .collect()
    }

    fn at(x: f32, y: f32, width: f32, height: f32) -> egui::Rect {
        egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, height))
    }

    #[test]
    fn missing_glyph_fails_for_an_unassigned_character_and_accepts_app_icons() {
        let mut harness = Harness::builder().build_ui(|ui| {
            ui.label(format!("Known {} × →", crate::design::icons::EXPAND));
            ui.label("Missing \u{0378}");
        });
        crate::design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let found = missing_glyphs(
            &harness.ctx,
            &harness.output().shapes,
            "seeded",
            "state",
            "400x300",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].check, "missing-glyph");
        assert!(found[0].detail.contains("U+0378"));
    }

    /// Muted `Small` help wider than the reader measure fails whether it wraps (in a 560 px scope),
    /// is cut to fit (`truncate`) or keeps one line (`extend`). The same words through
    /// `design::help` pass. Muted `Body` words, a `Small` status colour, words a row paints
    /// itself, and a row's own detail line (marked with `design::row_detail` and inside a row given
    /// to `design::register_row`, in either mode) are not help. A mark on a label outside every
    /// row changes nothing.
    #[test]
    fn help_measure_fails_on_a_wide_muted_small_label() {
        use crate::design::{Type, colors};
        let words = "Splat fills the template with the session’s name, its date and its number. ";
        let words = words.repeat(4);
        let mut harness = Harness::builder()
            .with_size(egui::vec2(1200.0, 600.0))
            .build_ui(move |ui| {
                let muted = |ui: &egui::Ui, lead: &str| {
                    Type::Small
                        .rich(ui, format!("{lead} {words}"))
                        .color(colors().muted)
                };
                // The faults: help in a 560 px scope, help cut to fit, help on one line.
                ui.scope(|ui| {
                    ui.set_max_width(560.0);
                    ui.add(egui::Label::new(muted(ui, "Wraps.")).wrap());
                });
                ui.scope(|ui| {
                    ui.set_max_width(700.0);
                    ui.add(egui::Label::new(muted(ui, "Cut.")).truncate());
                });
                ui.add(egui::Label::new(muted(ui, "One line.")).extend());
                crate::design::help(ui, &format!("Help. {words}"));
                ui.add(egui::Label::new(
                    Type::Body
                        .rich(ui, format!("Body. {words}"))
                        .color(colors().muted),
                ));
                ui.add(egui::Label::new(
                    Type::Small
                        .rich(ui, format!("Status. {words}"))
                        .color(colors().result.warning),
                ));
                // Words a row paints itself, not a label, are not help.
                let galley = ui.painter().layout(
                    format!("A row paints this. {words}"),
                    Type::Small.font(ui),
                    colors().muted,
                    700.0,
                );
                ui.painter().galley(ui.cursor().min, galley, colors().muted);
                ui.add_space(Type::Small.line() * 2.0);
                // A mark alone owns nothing: a marked label outside every row is still help.
                let alone = ui.add(egui::Label::new(muted(ui, "Marked alone.")).extend());
                crate::design::row_detail(ui, &alone);
                // A row's own detail line, in either mode, inside its registered row, is not help.
                let row = ui.scope(|ui| {
                    ui.label("Row name");
                    let detail = ui.add(egui::Label::new(muted(ui, "Row on one line.")).extend());
                    crate::design::row_detail(ui, &detail);
                });
                crate::design::register_row(ui, row.response.rect);
                let row = ui.scope(|ui| {
                    ui.set_max_width(700.0);
                    ui.label("Row name");
                    let detail = ui.add(egui::Label::new(muted(ui, "Row cut.")).truncate());
                    crate::design::row_detail(ui, &detail);
                });
                crate::design::register_row(ui, row.response.rect);
            });
        crate::design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let rows = RowDetails::of(&harness.ctx);
        let names: Vec<String> = harness
            .query_all_by_role(Role::Label)
            .filter(|node| !rows.owns(node.accesskit_node().id(), node.rect()))
            .filter_map(|node| label_words(&node.accesskit_node()))
            .collect();
        let labels = names.iter().map(String::as_str).collect();
        let found = wide_help(
            &harness.ctx,
            &harness.output().shapes,
            &labels,
            "seeded",
            "state",
            "1200x600",
        );
        let leads: Vec<_> = found
            .iter()
            .map(|violation| violation.node.split('.').next().unwrap_or_default())
            .collect();
        assert_eq!(
            leads,
            ["Wraps", "Cut", "One line", "Marked alone"],
            "{found:?}"
        );
        assert_eq!(
            (rows.marked.len(), rows.rows.len()),
            (3, 2),
            "three marks, two registered rows"
        );
        assert!(
            found
                .iter()
                .all(|violation| violation.check == HELP_MEASURE)
        );
        assert!(
            found[0].rect[2] > 488.0 + OVERLAP_TOLERANCE && found[0].rect[2] <= 560.0,
            "the 560 px label is found at its width: {found:?}"
        );
    }

    /// Every check can fail. A check that is 0 everywhere looks the same as a broken one.
    #[test]
    fn a_clean_control_breaks_no_check() {
        let clean = control("Save", at(10.0, 10.0, 80.0, 30.0));
        assert_eq!(checks_for(&[clean]), Vec::<String>::new());
    }

    #[test]
    fn keyboard_fails_when_a_key_does_nothing() {
        let mut outcome = crate::visual_tests::driver::Outcome::default();
        let key = ("ctrl-s", "library, dirty note");
        outcome.keyboard_result("seeded", "960x640".into(), key, true, "worked".into());
        assert!(outcome.violations.is_empty());
        assert_eq!(outcome.passes.len(), 1);
        outcome.keyboard_result("seeded", "960x640".into(), key, false, "nothing".into());
        assert_eq!(outcome.passes.len(), 1);
        assert_eq!(outcome.violations.len(), 1);
        assert_eq!(outcome.violations[0].check, "keyboard");
    }

    #[test]
    fn target_size_fails_on_a_small_control() {
        let small = control("Save", at(10.0, 10.0, 16.0, 16.0));
        assert!(checks_for(&[small]).contains(&"target-size".to_owned()));
    }

    #[test]
    fn accessible_name_fails_on_an_unnamed_control() {
        let unnamed = control("", at(10.0, 10.0, 80.0, 30.0));
        assert!(checks_for(&[unnamed]).contains(&"accessible-name".to_owned()));
    }

    #[test]
    fn in_viewport_fails_outside_the_window() {
        let outside = control("Save", at(500.0, 10.0, 80.0, 30.0));
        assert!(checks_for(&[outside]).contains(&"in-viewport".to_owned()));
    }

    #[test]
    fn no_overlap_fails_on_two_controls_in_one_place() {
        let a = control("Save", at(10.0, 10.0, 80.0, 30.0));
        let b = control("Send", at(20.0, 10.0, 80.0, 30.0));
        assert!(checks_for(&[a, b]).contains(&"no-overlap".to_owned()));
    }

    fn overlap_checks(controls: &[Control]) -> Vec<String> {
        checks_for(controls)
            .into_iter()
            .filter(|check| check.starts_with("no-overlap"))
            .collect()
    }

    /// Each order of each kind of pair: the first control is the one reported.
    #[test]
    fn an_overlap_is_filed_by_whether_egui_chrome_can_take_the_press() {
        let app = control("Save", at(10.0, 10.0, 80.0, 30.0));
        let other_app = control("Send", at(20.0, 10.0, 80.0, 30.0));
        let mut splitter = control("Resize side panel column", at(30.0, 0.0, 10.0, 200.0));
        splitter.role = Role::Splitter;
        splitter.chrome = true;
        let mut bar = control("", at(35.0, 30.0, 200.0, 8.0));
        bar.role = Role::Unknown;
        bar.chrome = true;
        assert_eq!(overlap_checks(&[app.clone(), other_app]), ["no-overlap"]);
        assert_eq!(
            overlap_checks(&[splitter.clone(), bar.clone()]),
            ["no-overlap/egui-chrome"]
        );
        assert_eq!(
            overlap_checks(&[app.clone(), splitter.clone()]),
            [APP_VS_CHROME]
        );
        assert_eq!(overlap_checks(&[splitter, app.clone()]), [APP_VS_CHROME]);
        let mut text = control("Spellcasting", at(50.0, 20.0, 80.0, 18.0));
        text.role = Role::Label;
        assert_eq!(
            overlap_checks(&[bar.clone(), text.clone()]),
            [APP_VS_CHROME]
        );
        assert_eq!(overlap_checks(&[text, bar]), [APP_VS_CHROME]);
    }

    /// A popup's own background is behind its items, so they stay egui's count. Another
    /// popup's background is not. The popup is a scope of its own, over the page's scope:
    /// its content over a splitter or a scroll bar of the page is an app fault.
    #[test]
    fn an_item_over_its_own_popup_background_stays_egui_chrome() {
        const POPUP: u64 = 2;
        let mut background = control("", at(0.0, 0.0, 200.0, 120.0));
        background.role = Role::Unknown;
        background.chrome = true;
        background.scope = POPUP;
        let mut item = control("Delete", at(7.0, 7.0, 186.0, 28.0));
        item.popup = Some(background.id);
        item.scope = POPUP;
        let mut text = control("Danger", at(9.0, 40.0, 186.0, 16.0));
        text.role = Role::Label;
        text.popup = Some(background.id);
        text.scope = POPUP;
        for pair in [
            [background.clone(), item.clone()],
            [item.clone(), background.clone()],
            [background.clone(), text.clone()],
            [text.clone(), background.clone()],
        ] {
            assert_eq!(
                overlap_checks(&pair),
                ["no-overlap/egui-chrome"],
                "{pair:?}"
            );
        }
        // The page's splitter and scroll bar, under the popup.
        let mut splitter = control("Resize creature search column", at(150.0, 0.0, 10.0, 300.0));
        splitter.role = Role::Splitter;
        splitter.chrome = true;
        let mut bar = control("", at(0.0, 44.0, 300.0, 8.0));
        bar.id = 99;
        bar.role = Role::Unknown;
        bar.chrome = true;
        assert_ne!(splitter.scope, POPUP);
        for (content, handle) in [(&item, &splitter), (&text, &splitter), (&text, &bar)] {
            for pair in [
                [content.clone(), handle.clone()],
                [handle.clone(), content.clone()],
            ] {
                assert_eq!(overlap_checks(&pair), [APP_VS_CHROME], "{pair:?}");
            }
        }
        // Clear of the handles, nothing; the background over them stays out of the check,
        // and so does page content in another scope.
        let mut clear = item.clone();
        clear.rect = at(7.0, 7.0, 130.0, 28.0);
        let mut page = item.clone();
        page.popup = None;
        // A modal dialog over the page blocks its splitter.
        let mut blocked = splitter.clone();
        blocked.blocked = true;
        // A submenu is a popup of its own, over its parent menu's background.
        let mut submenu = item.clone();
        submenu.scope = POPUP + 1;
        submenu.popup = Some(background.id + 1);
        for pair in [
            [clear, splitter.clone()],
            [background.clone(), splitter.clone()],
            [background.clone(), bar],
            [page, splitter],
            [item.clone(), blocked.clone()],
            [blocked, item.clone()],
            [submenu.clone(), background.clone()],
            [background.clone(), submenu],
        ] {
            assert!(overlap_checks(&pair).is_empty(), "{pair:?}");
        }
        item.popup = Some(background.id + 1);
        assert_eq!(overlap_checks(&[background, item]), [APP_VS_CHROME]);
    }

    /// The real tree: a menu opened in a column, wider than the column, lies over the
    /// column's splitter in another scope. Each choice and text over the splitter is an
    /// app fault; the menu's own background stays egui's count; a narrow menu is clear.
    /// The same menu in a modal dialog over the column is clear too: the dialog blocks
    /// the splitter.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn a_menu_over_a_column_splitter_is_an_app_fault() {
        // In a modal dialog over the column, the menu crosses a splitter the dialog blocks.
        let checks = |choice: &'static str, note: &'static str, modal: bool| {
            let mut harness =
                Harness::builder()
                    .with_size(egui::vec2(600.0, 400.0))
                    .build_state(
                        move |ctx, _: &mut crate::Splat| {
                            let menu = |ui: &mut egui::Ui| {
                                ui.menu_button("Open", |ui| {
                                    let _ = ui.button(choice);
                                    ui.label(note);
                                });
                            };
                            crate::design::Column::left("encounter_add", 200.0, 150.0..=300.0)
                                .show(ctx, |ui| {
                                    if !modal {
                                        menu(ui);
                                    }
                                });
                            egui::CentralPanel::default().show(ctx, |_| {});
                            if modal {
                                egui::Modal::new(egui::Id::new("dialog")).show(ctx, |ui| {
                                    ui.set_min_width(300.0);
                                    menu(ui);
                                });
                            }
                        },
                        crate::Splat::default(),
                    );
            harness.run();
            harness.get_by_label("Open").click();
            harness.run();
            let found = controls(&harness);
            let splitter = found
                .iter()
                .find(|control| control.name == "Resize creature search column")
                .expect("the column's splitter is in the tree");
            assert!(splitter.chrome && splitter.role == Role::Splitter);
            let item = found
                .iter()
                .find(|control| control.name == choice)
                .expect("the menu choice is in the tree");
            assert!(item.popup.is_some() && item.scope != splitter.scope);
            // The menu's text, which has no accessible name of its own here.
            let text = found
                .iter()
                .find(|control| control.is_label() && control.popup == item.popup)
                .expect("the menu's text is in the tree");
            assert_eq!(text.scope, item.scope);
            let overlaps = [
                item.rect.intersects(splitter.rect),
                text.rect.intersects(splitter.rect),
            ];
            let checks: Vec<_> = inspect(
                &found,
                &[],
                harness.ctx.content_rect(),
                "seeded",
                "menu",
                "600x400",
            )
            .into_iter()
            .filter(|violation| violation.check.starts_with("no-overlap"))
            .map(|violation| (violation.check, violation.node, violation.detail))
            .collect();
            (overlaps, checks, splitter.blocked)
        };
        let wide = "Move this group to Arena Final";
        let note = "Danger: its creatures move with it";
        let (overlaps, found, blocked) = checks(wide, note, false);
        assert_eq!(overlaps, [true; 2], "the wide menu crosses the splitter");
        assert!(!blocked);
        let against_splitter: Vec<_> = found
            .iter()
            .filter(|(.., detail)| detail.contains("Resize creature search column"))
            .chain(
                found
                    .iter()
                    .filter(|(_, node, _)| node == "Resize creature search column"),
            )
            .collect();
        let names: Vec<_> = against_splitter
            .iter()
            .map(|(check, node, detail)| {
                assert_eq!(check, APP_VS_CHROME, "{node}: {detail}");
                if node.starts_with("Resize") {
                    detail.clone()
                } else {
                    node.clone()
                }
            })
            .collect();
        assert!(
            names.iter().any(|name| name.contains(wide)),
            "the choice: {found:?}"
        );
        assert!(
            names.iter().any(|name| name.contains("(unnamed Label)")),
            "its text: {found:?}"
        );
        let background: Vec<_> = found
            .iter()
            .filter(|(_, node, _)| node == "(egui handle at edge)")
            .collect();
        assert_eq!(background.len(), 2, "the menu's own background: {found:?}");
        assert!(
            background
                .iter()
                .all(|(check, ..)| check == "no-overlap/egui-chrome"),
            "the menu's own background: {found:?}"
        );
        let (overlaps, found, blocked) = checks(wide, note, true);
        assert_eq!(
            overlaps, [true; 2],
            "the menu in the dialog crosses the splitter"
        );
        assert!(blocked, "the dialog blocks the splitter");
        assert!(
            found.iter().all(|(check, ..)| check != APP_VS_CHROME),
            "{found:?}"
        );
        let (overlaps, found, _) = checks("Move", "Danger", false);
        assert_eq!(
            overlaps, [false; 2],
            "the narrow menu is clear of the splitter"
        );
        assert!(
            found.iter().all(|(check, ..)| check != APP_VS_CHROME),
            "{found:?}"
        );
    }

    /// A click as the visual driver makes one: hover, then press and release.
    fn press_at(harness: &mut Harness<'_, crate::Splat>, pos: egui::Pos2) {
        harness.hover_at(pos);
        harness.step();
        for pressed in [true, false] {
            harness.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
        }
        harness.step();
        harness.remove_cursor();
        harness.run();
    }

    /// The five points a press is tried at: the middle and each corner of the overlap.
    fn press_points(overlap: egui::Rect) -> [egui::Pos2; 5] {
        let inner = overlap.shrink(1.0);
        [
            inner.center(),
            inner.left_top(),
            inner.right_top(),
            inner.left_bottom(),
            inner.right_bottom(),
        ]
    }

    /// Egui's own hit test: a press anywhere on a menu item reaches the item, not the
    /// popup's background behind it, so the pair stays egui's count.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn a_press_on_a_menu_item_reaches_the_item_and_the_pair_stays_chrome() {
        let clicks = std::rc::Rc::new(std::cell::Cell::new(0));
        let counted = clicks.clone();
        let mut harness = Harness::builder()
            .with_size(egui::vec2(400.0, 300.0))
            .build_state(
                move |ctx, _: &mut crate::Splat| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.menu_button("Open", |ui| {
                            if ui.button("Delete group").clicked() {
                                counted.set(counted.get() + 1);
                            }
                            ui.label("Danger");
                        });
                    });
                },
                crate::Splat::default(),
            );
        let open = |harness: &mut Harness<'_, crate::Splat>| {
            harness.get_by_label("Open").click();
            harness.run();
        };
        open(&mut harness);
        let found = controls(&harness);
        let item = found
            .iter()
            .find(|control| control.name == "Delete group")
            .expect("the menu item is in the tree");
        let background = found
            .iter()
            .find(|control| control.chrome && Some(control.id) == item.popup)
            .expect("the item names its popup background");
        let overlap = item.rect.intersect(background.rect);
        assert!(overlap.width() > 1.0 && overlap.height() > 1.0);
        let pair: Vec<_> = inspect(
            &found,
            &[],
            harness.ctx.content_rect(),
            "seeded",
            "menu",
            "400x300",
        )
        .into_iter()
        .filter(|violation| {
            violation.check.starts_with("no-overlap")
                && (violation.node == "Delete group" || violation.detail.contains("Delete group"))
        })
        .map(|violation| violation.check)
        .collect();
        assert_eq!(pair, ["no-overlap/egui-chrome"]);
        for (index, pos) in press_points(overlap).into_iter().enumerate() {
            if index > 0 {
                open(&mut harness);
            }
            press_at(&mut harness, pos);
            assert_eq!(
                clicks.get(),
                index + 1,
                "a press at {pos:?} reaches the item"
            );
        }
    }

    /// Egui's own hit test: a nested horizontal bar that floats over a row takes the press
    /// there, so the row's overlap with it is an app fault. Away from the bar the row works.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn a_press_where_a_scroll_bar_lies_on_a_row_reaches_the_bar_and_is_an_app_fault() {
        let clicks = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let counted = clicks.clone();
        let mut harness = Harness::builder()
            .with_size(egui::vec2(300.0, 240.0))
            .build_state(
                move |ctx, _: &mut crate::Splat| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::ScrollArea::horizontal()
                                .id_salt("wide-table")
                                .show(ui, |ui| {
                                    for row in 0..30 {
                                        let button = egui::Button::new(format!("Row {row}"))
                                            .min_size(egui::vec2(600.0, 30.0));
                                        if ui.add(button).clicked() {
                                            counted.borrow_mut().push(row);
                                        }
                                    }
                                });
                        });
                    });
                },
                crate::Splat::default(),
            );
        harness.run_steps(3);
        let found = controls(&harness);
        let bar = found
            .iter()
            .find(|control| control.chrome && control.rect.width() > control.rect.height())
            .expect("the floated bar is chrome");
        let shown = |control: &Control| {
            control
                .clip
                .map_or(control.rect, |clip| control.rect.intersect(clip))
        };
        let (row, under) = found
            .iter()
            .filter(|control| control.name.starts_with("Row "))
            .map(|control| (control, shown(control).intersect(bar.rect)))
            .find(|(_, overlap)| overlap.width() > 1.0 && overlap.height() > 1.0)
            .expect("a row lies under the bar");
        let checks: Vec<_> = inspect(
            &found,
            &[],
            harness.ctx.content_rect(),
            "seeded",
            "table",
            "300x240",
        )
        .into_iter()
        .filter(|violation| {
            violation.check.starts_with("no-overlap")
                && (violation.node == row.name || violation.detail.contains(&row.name))
        })
        .map(|violation| violation.check)
        .collect();
        assert_eq!(checks, [APP_VS_CHROME]);
        for pos in press_points(under) {
            press_at(&mut harness, pos);
        }
        assert_eq!(
            *clicks.borrow(),
            Vec::<usize>::new(),
            "the bar takes every press"
        );
        // The same row still takes a press beside the bar.
        let visible = shown(row);
        let beside = if under.top() - visible.top() > visible.bottom() - under.bottom() {
            egui::pos2(visible.center().x, (visible.top() + under.top()) / 2.0)
        } else {
            egui::pos2(
                visible.center().x,
                (under.bottom() + visible.bottom()) / 2.0,
            )
        };
        press_at(&mut harness, beside);
        let number: usize = row.name["Row ".len()..].parse().unwrap();
        assert_eq!(*clicks.borrow(), vec![number]);
    }

    /// A hand-written allowance for a mixed overlap does not load, and does not apply.
    #[test]
    fn a_mixed_overlap_fails_against_a_hand_written_allowance() {
        let text =
            br#"{"violations": {"no-overlap/app-vs-chrome": {"party": 3}}, "task_cost": {}}"#;
        let error = Baseline::parse(text).expect_err("a positive allowance does not load");
        assert!(
            error.contains("no-overlap/app-vs-chrome / party: 3"),
            "{error}"
        );
        let zero =
            br#"{"violations": {"no-overlap/app-vs-chrome": {"party": 0}}, "task_cost": {}}"#;
        assert!(Baseline::parse(zero).is_ok());
        // Built in memory, past the loader: the allowance has no route to apply. The
        // per-scenario comparison skips the check, and the state comparison fails the pair.
        let known: Baseline = serde_json::from_slice(text).unwrap();
        let violations = [mixed("party", "01-start", "960x640")];
        let current = Baseline::measure(&violations, &[]);
        assert_eq!(
            current.violations[APP_VS_CHROME]["party"], 1,
            "the run counts the pair"
        );
        assert_eq!(known.compare(&current, &["party"]), (vec![], vec![]));
        let (worse, stale) = compare_app_vs_chrome(&violations, &[], &["party"]);
        assert_eq!(
            worse,
            ["no-overlap/app-vs-chrome / party / 01-start / 960x640: 1 (known 0)"]
        );
        assert!(stale.is_empty());
    }

    fn mixed(scenario: &str, state: &str, window: &str) -> Violation {
        Violation {
            check: APP_VS_CHROME.to_owned(),
            scenario: scenario.to_owned(),
            state: state.to_owned(),
            window: window.to_owned(),
            node: "(unnamed Label)".to_owned(),
            role: "Label".to_owned(),
            rect: [0.0; 4],
            detail: "Overlaps (egui handle at edge) by 80.0x5.0".to_owned(),
        }
    }

    /// The pairs this round lists, in the shape of `KNOWN_APP_VS_CHROME`. The tests use
    /// their own copy, so they keep working as the real list shrinks.
    const LISTED: &[(StateKey<'static>, usize)] = &[
        (
            ("library-browse", "05-class-traits-and-features", "1200x800"),
            1,
        ),
        (
            (
                "library-browse",
                "05-class-traits-and-features",
                "1920x1080",
            ),
            5,
        ),
    ];

    fn listed_run() -> Vec<Violation> {
        let mut run = vec![mixed(
            "library-browse",
            "05-class-traits-and-features",
            "1200x800",
        )];
        for _ in 0..5 {
            run.push(mixed(
                "library-browse",
                "05-class-traits-and-features",
                "1920x1080",
            ));
        }
        run
    }

    #[test]
    fn exactly_the_listed_app_vs_chrome_pairs_pass() {
        let run = listed_run();
        assert_eq!(run.len(), 6);
        let ran = ["library-browse", "party"];
        assert_eq!(compare_app_vs_chrome(&run, LISTED, &ran), (vec![], vec![]));
        // A scenario that did not run is neither worse nor stale.
        assert_eq!(
            compare_app_vs_chrome(&[], LISTED, &["party"]),
            (vec![], vec![])
        );
    }

    #[test]
    fn an_app_vs_chrome_pair_in_another_scenario_fails() {
        let mut run = listed_run();
        run.push(mixed("npc-in-encounter", "03-npc-in-setup", "960x640"));
        let (worse, stale) =
            compare_app_vs_chrome(&run, LISTED, &["library-browse", "npc-in-encounter"]);
        assert_eq!(
            worse,
            [
                "no-overlap/app-vs-chrome / npc-in-encounter / 03-npc-in-setup / 960x640: 1 (known 0)"
            ]
        );
        assert!(stale.is_empty());
    }

    #[test]
    fn an_app_vs_chrome_pair_in_another_state_of_a_listed_scenario_fails() {
        for extra in [
            mixed("library-browse", "06-spell-table", "1920x1080"),
            mixed("library-browse", "05-class-traits-and-features", "960x640"),
            mixed("library-browse", "05-class-traits-and-features", "1200x800"),
        ] {
            let mut run = listed_run();
            run.push(extra.clone());
            let (worse, stale) = compare_app_vs_chrome(&run, LISTED, &["library-browse"]);
            assert_eq!(worse.len(), 1, "{extra:?}: {worse:?}");
            assert!(
                worse[0].contains(&format!("{} / {}", extra.state, extra.window)),
                "{worse:?}"
            );
            assert!(stale.is_empty());
        }
    }

    #[test]
    fn a_fixed_app_vs_chrome_pair_fails_as_stale_until_the_list_shrinks() {
        let mut run = listed_run();
        run.pop();
        assert_eq!(run.len(), 5);
        let (worse, stale) = compare_app_vs_chrome(&run, LISTED, &["library-browse"]);
        assert!(worse.is_empty());
        assert_eq!(
            stale,
            [
                "no-overlap/app-vs-chrome / library-browse / 05-class-traits-and-features / 1920x1080: 4 (known 5). Fixed: lower or remove its line in KNOWN_APP_VS_CHROME"
            ]
        );
        let (worse, stale) = compare_app_vs_chrome(&[], LISTED, &["library-browse"]);
        assert!(worse.is_empty());
        assert_eq!(stale.len(), 2, "{stale:?}");
    }

    /// The real list: unique states, positive counts, and only this round's open pairs.
    #[test]
    fn the_known_app_vs_chrome_list_only_shrinks() {
        let mut keys: Vec<_> = KNOWN_APP_VS_CHROME.iter().map(|(key, _)| *key).collect();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), KNOWN_APP_VS_CHROME.len());
        assert!(KNOWN_APP_VS_CHROME.iter().all(|(_, count)| *count > 0));
        for (key, count) in KNOWN_APP_VS_CHROME {
            assert!(
                LISTED
                    .iter()
                    .any(|(listed, limit)| listed == key && count <= limit),
                "{key:?} is not one of the open pairs, or its count grew"
            );
        }
    }

    #[test]
    fn a_label_inside_its_own_interactive_ancestor_does_not_overlap_it() {
        let row = control("Kestrel", at(10.0, 10.0, 200.0, 50.0));
        let mut label = control("Bloodied", at(20.0, 40.0, 70.0, 16.0));
        label.role = Role::Label;
        label.label_owner = Some(row.id);
        assert!(!checks_for(&[row, label]).contains(&"no-overlap".to_owned()));
    }

    #[test]
    fn a_sibling_control_still_overlaps_an_owned_label() {
        let row = control("Kestrel", at(10.0, 10.0, 200.0, 50.0));
        let mut label = control("Bloodied", at(20.0, 40.0, 70.0, 16.0));
        label.role = Role::Label;
        label.label_owner = Some(row.id);
        let sibling = control("Refresh", at(30.0, 38.0, 80.0, 24.0));
        let viewport = at(0.0, 0.0, 400.0, 300.0);
        let found = inspect(
            &[row, sibling, label],
            &[],
            viewport,
            "seeded",
            "owned-label-with-sibling",
            "400x300",
        );
        assert!(found.iter().any(|violation| {
            violation.check == "no-overlap"
                && violation.node == "Refresh"
                && violation.detail.starts_with("Overlaps Bloodied")
        }));
    }

    #[test]
    fn clipped_fails_on_a_control_cut_at_the_side_of_its_pane() {
        let mut cut = control("Save", at(10.0, 10.0, 80.0, 30.0));
        cut.clip = Some(at(0.0, 0.0, 50.0, 300.0));
        assert!(checks_for(&[cut]).contains(&"clipped".to_owned()));
    }

    #[test]
    fn clipped_fails_when_a_later_pane_covers_the_end_of_text() {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(400.0, 300.0))
            .build(|ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.painter().text(
                        egui::pos2(100.0, 50.0),
                        egui::Align2::LEFT_TOP,
                        "Saved beside the roster",
                        egui::FontId::proportional(18.0),
                        egui::Color32::BLACK,
                    );
                    ui.painter().rect_filled(
                        at(180.0, 0.0, 220.0, 300.0),
                        0.0,
                        egui::Color32::WHITE,
                    );
                });
            });
        harness.run_steps(2);
        let found = cut_text(
            &harness.output().shapes,
            &[],
            &[],
            "seeded",
            "covered",
            "400x300",
        );
        assert!(
            found
                .iter()
                .any(|v| v.detail.starts_with("Covered by a later pane: Saved")),
            "{found:?}"
        );
    }

    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn horizontal_clipping_is_allowed_only_where_a_real_scroll_bar_can_reveal_it() {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(300.0, 240.0))
            .build_state(
                |ctx, _: &mut crate::Splat| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::ScrollArea::horizontal()
                                .id_salt("wide-table")
                                .show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(800.0, 800.0));
                                    ui.horizontal(|ui| {
                                        ui.add_space(200.0);
                                        ui.label("A column beyond the visible edge");
                                    });
                                });
                        });
                    });
                },
                crate::Splat::default(),
            );
        harness.run_steps(3);
        let regions = horizontal_scroll_regions(&harness);
        assert_eq!(regions.len(), 1);
        let found = controls(&harness);
        let check = |regions: &[egui::Rect]| {
            inspect(
                &found,
                regions,
                harness.ctx.content_rect(),
                "seeded",
                "state",
                "300x240",
            )
            .into_iter()
            .filter(|violation| violation.check == "clipped")
            .count()
        };
        assert!(check(&[]) > 0, "the column starts partly outside the pane");
        assert_eq!(check(&regions), 0);
        assert!(
            !cut_text(
                &harness.output().shapes,
                &[],
                &[],
                "seeded",
                "state",
                "300x240"
            )
            .is_empty()
        );
        assert!(
            cut_text(
                &harness.output().shapes,
                &regions,
                &[],
                "seeded",
                "state",
                "300x240"
            )
            .is_empty()
        );
        assert!(
            found
                .iter()
                .any(|control| { control.chrome && control.rect.width() > control.rect.height() }),
            "the bar floated up by the outer reader still counts as chrome"
        );
    }

    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn a_thin_unnamed_control_inside_horizontal_content_is_not_chrome() {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(300.0, 240.0))
            .build_state(
                |ctx, _: &mut crate::Splat| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
                        let width = ui.available_width();
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            ui.allocate_exact_size(egui::vec2(width, 8.0), egui::Sense::click());
                            ui.set_min_size(egui::vec2(800.0, 80.0));
                        });
                    });
                },
                crate::Splat::default(),
            );
        harness.run_steps(3);
        let found: Vec<_> = controls(&harness)
            .into_iter()
            .filter(|control| !control.is_label() && control.rect.height() <= CHROME_THICKNESS)
            .collect();
        assert_eq!(found.len(), 2, "the control and the real bar: {found:?}");
        assert_eq!(found.iter().filter(|control| control.chrome).count(), 1);
        assert!(checks_for(&found).contains(&"target-size".to_owned()));
    }

    /// A long scrolled note passes behind a smaller scroll area above it. Its own area clips
    /// it, so what shows of it does not touch the other area's content.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn scrolled_content_is_clipped_by_its_own_area_and_not_one_it_passes_behind() {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(400.0, 400.0))
            .build_state(
                |ctx, _: &mut crate::Splat| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("above")
                            .max_height(100.0)
                            .max_width(200.0)
                            .show(ui, |ui| {
                                ui.add_sized(egui::vec2(200.0, 40.0), egui::Button::new("Above"));
                                ui.set_min_height(600.0);
                            });
                        egui::ScrollArea::vertical()
                            .id_salt("below")
                            .vertical_scroll_offset(200.0)
                            .show(ui, |ui| {
                                ui.add_sized(egui::vec2(360.0, 900.0), egui::Button::new("Below"));
                            });
                    });
                },
                crate::Splat::default(),
            );
        harness.run_steps(3);
        let found = controls(&harness);
        let below = found
            .iter()
            .find(|control| control.name == "Below")
            .unwrap();
        let area = below.clip.expect("in a scroll area");
        assert!(
            below.rect.top() < area.top(),
            "the test needs it scrolled up: {below:?}"
        );
        assert!(
            area.width() > 300.0 && area.top() > 100.0,
            "its own area, not the narrow one above: {area:?}"
        );
        assert!(!checks_for(&found).contains(&"clipped".to_owned()));
        let above = found
            .iter()
            .find(|control| control.name == "Above")
            .unwrap();
        let above_shown = above.rect.intersect(above.clip.expect("in a scroll area"));
        assert!(
            !below.rect.intersect(area).intersects(above_shown),
            "what shows of them does not touch"
        );
    }

    /// Painted text has no node for the other checks to find, so this reads the shapes.
    #[test]
    fn lookup_elision_requires_the_full_accessible_detail_and_keeps_checking_names() {
        const DETAIL: &str = " · Heading in a long campaign note · Campaign";
        let mut harness = Harness::builder()
            .with_size(egui::vec2(200.0, 100.0))
            .build_state(
                |ctx, _: &mut crate::Splat| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add(
                            egui::Label::new("A long name that must not lose its end").truncate(),
                        );
                        let response = crate::design::row(
                            ui,
                            28.0,
                            false,
                            &format!("Entry{DETAIL}"),
                            |ui, rect| {
                                let text = crate::design::one_line(
                                    ui,
                                    DETAIL,
                                    egui::FontId::proportional(14.0),
                                    egui::Color32::WHITE,
                                    rect.width(),
                                );
                                ui.painter().galley(rect.min, text, egui::Color32::WHITE);
                            },
                        );
                        ui.ctx().accesskit_node_builder(response.id, |node| {
                            node.set_role_description("Lookup result");
                        });
                    });
                },
                crate::Splat::default(),
            );
        harness.run_steps(2);
        let check = |rows: &[(String, egui::Rect)]| {
            cut_text(
                &harness.output().shapes,
                &[],
                rows,
                "seeded",
                "state",
                "200x100",
            )
        };
        assert_eq!(check(&[]).len(), 2, "both painted strings are shortened");
        let mut rows = lookup_results(&harness);
        assert_eq!(rows.len(), 1);
        let found = check(&rows);
        assert_eq!(found.len(), 1);
        assert!(found[0].detail.contains("A long name"), "names still fail");
        rows[0].0 = "Entry".into();
        assert_eq!(
            check(&rows).len(),
            2,
            "missing accessible detail still fails"
        );
    }

    /// Painted text has no node for the other checks to find, so this reads the shapes.
    #[test]
    fn cut_text_fails_on_a_truncated_label_and_past_a_pane_edge() {
        let mut harness = Harness::builder()
            .with_size(egui::vec2(200.0, 100.0))
            .build_state(
                |ctx, _: &mut crate::Splat| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add(
                            egui::Label::new("A long label that cannot fit in this little window")
                                .truncate(),
                        );
                        ui.scope(|ui| {
                            ui.set_clip_rect(at(0.0, 0.0, 30.0, 100.0));
                            ui.painter().text(
                                egui::pos2(10.0, 60.0),
                                egui::Align2::LEFT_CENTER,
                                "Past the edge",
                                egui::FontId::proportional(14.0),
                                egui::Color32::WHITE,
                            );
                        });
                    });
                },
                crate::Splat::default(),
            );
        harness.run_steps(2);
        let details: Vec<String> = cut_text(
            &harness.output().shapes,
            &[],
            &[],
            "seeded",
            "state",
            "200x100",
        )
        .into_iter()
        .map(|violation| violation.detail)
        .collect();
        assert!(
            details.iter().any(|d| d.starts_with("Cut to an ellipsis")),
            "{details:?}"
        );
        assert!(
            details.iter().any(|d| d.starts_with("Runs past")),
            "{details:?}"
        );
    }
}
