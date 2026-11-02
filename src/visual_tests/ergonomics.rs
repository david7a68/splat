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
use std::collections::{BTreeMap, HashMap};

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

pub const CHECKS: [&str; 10] = [
    "target-size",
    "accessible-name",
    "in-viewport",
    "no-overlap",
    "clipped",
    "keyboard",
    "target-size/egui-chrome",
    "accessible-name/egui-chrome",
    "no-overlap/egui-chrome",
    "workflow",
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
    pub rect: egui::Rect,
    pub disabled: bool,
    /// The top-level pane or window that holds this control.
    pub scope: u64,
    /// The visible part of the scroll area around this control, if any.
    pub clip: Option<egui::Rect>,
    /// A scroll bar, resize handle or window title bar that egui adds by itself.
    pub chrome: bool,
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

/// egui's own scroll bar: unnamed, with no role, thin, and lying wholly outside a scroll
/// surface, along the full length of that surface's edge. Scroll content lies inside the
/// surface, and a control the app draws has a role and a name, so neither can match.
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
                && rect.top() >= surface.bottom()
                && rect.top() <= surface.bottom() + CHROME_THICKNESS;
            tall || wide
        })
}

/// A column's drag handle, which egui adds to the panel: unnamed, with no role, thin, as
/// tall as a column, and only draggable. A control the app draws has a role and a name.
fn column_handle(node: &AccessKitNode<'_>, rect: egui::Rect) -> bool {
    const AS_TALL_AS_A_COLUMN: f32 = 100.0;
    node.role() == Role::Unknown
        && node.label().is_none()
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
    for child in node.children() {
        let scope = if depth == 0 {
            child.accesskit_node().id().0
        } else {
            scope
        };
        let inner = if surfaces.is_empty() {
            clip
        } else {
            // Content is a sibling of the surface, and overlaps it.
            let bounds = extent(&child);
            surfaces
                .iter()
                .find(|surface| surface.intersects(bounds))
                .map_or(clip, |surface| {
                    Some(clip.map_or(*surface, |outer| outer.intersect(*surface)))
                })
        };
        visit(&child, scope, inner, parent_is_window, &surfaces);
        walk(&child, scope, inner, depth + 1, visit);
    }
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

/// Controls, and labels for the overlap and clipping checks. See `Control::is_label`.
pub fn controls(harness: &Harness<'_, Splat>) -> Vec<Control> {
    let mut out = Vec::new();
    let root = harness.root();
    let id = root.accesskit_node().id().0;
    walk(
        &root,
        id,
        None,
        0,
        &mut |node, scope, clip, parent_is_window, surfaces| {
            let access = node.accesskit_node();
            if access.role() == Role::Label
                && let Some(rect) = rect_of(&access)
            {
                out.push(Control {
                    id: access.id().0,
                    name: access.label().unwrap_or_default(),
                    placeholder: None,
                    role: Role::Label,
                    rect,
                    disabled: false,
                    scope,
                    clip,
                    chrome: false,
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
                rect,
                disabled: access.is_disabled(),
                scope,
                clip,
                chrome: scroll_bar(&access, rect, surfaces)
                    || column_handle(&access, rect)
                    || popup_body(node, rect, parent_is_window),
            });
        },
    );
    out
}

/// Text that may lose its end, each with the reason.
const ELISION_ALLOWED: &[(&str, &str)] = &[(
    // The fixture's folder. A path in a one-line field scrolls inside the field.
    "/tmp/splat-visual-world",
    "a folder path in a text field",
)];

/// Painted text that lost its end to an ellipsis, or runs past the edge of its pane. A
/// tooltip does not help a DM reading across a table, so all of it is reported, whatever
/// painted it, unless it is in the named list above.
pub fn cut_text(
    shapes: &[egui::epaint::ClippedShape],
    scenario: &str,
    state: &str,
    window: &str,
) -> Vec<Violation> {
    fn search(
        shape: &egui::Shape,
        clip: egui::Rect,
        found: &mut Vec<(String, egui::Rect, &'static str)>,
    ) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| search(shape, clip, found)),
            egui::Shape::Text(text) => {
                let bounds = text.visual_bounding_rect();
                let words = text.galley.job.text.clone();
                if text.galley.elided {
                    found.push((words, bounds, "Cut to an ellipsis"));
                } else if bounds.intersects(clip)
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
    for clipped in shapes {
        search(&clipped.shape, clipped.clip_rect, &mut found);
    }
    found
        .into_iter()
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

/// Run the per-state checks over the controls in the tree.
pub fn inspect(
    controls: &[Control],
    viewport: egui::Rect,
    scenario: &str,
    state: &str,
    window: &str,
) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut report = |check: &str, control: &Control, other_chrome: bool, detail: String| {
        violations.push(Violation {
            check: if control.chrome || other_chrome {
                format!("{check}{CHROME_SUFFIX}")
            } else {
                check.to_owned()
            },
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
            && (control.rect.left() < shown(control).left() - OVERLAP_TOLERANCE
                || control.rect.right() > shown(control).right() + OVERLAP_TOLERANCE)
        {
            report(
                "clipped",
                control,
                false,
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
                "target-size",
                control,
                false,
                format!(
                    "{:.1}x{:.1}, needs {MIN_TARGET}x{MIN_TARGET}{enabled}",
                    control.rect.width(),
                    control.rect.height()
                ),
            );
        }
        if visible && control.name.trim().is_empty() {
            report(
                "accessible-name",
                control,
                false,
                format!("No label{enabled}"),
            );
        }
        // A disabled control behind a dialog is not a target, and it has no scroll node.
        if !control.disabled && control.clip.is_none() && !viewport.contains_rect(control.rect) {
            report(
                "in-viewport",
                control,
                false,
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
            if first.scope != second.scope || (first.is_label() && second.is_label()) {
                continue;
            }
            let overlap = first_shown.intersect(shown(second));
            if overlap.width() > OVERLAP_TOLERANCE && overlap.height() > OVERLAP_TOLERANCE {
                report(
                    "no-overlap",
                    first,
                    second.chrome,
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

    /// Lines where `current` is worse than this baseline, then lines where it is better.
    /// A scenario that did not run is neither.
    pub fn compare(&self, current: &Self, scenarios: &[&str]) -> (Vec<String>, Vec<String>) {
        let mut worse = Vec::new();
        let mut better = Vec::new();
        let empty = BTreeMap::new();
        for check in CHECKS {
            let known = self.violations.get(check).unwrap_or(&empty);
            let found = current.violations.get(check).unwrap_or(&empty);
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
    use egui_kittest::Harness;

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
    }

    /// A real control can never count as egui chrome, however small it is.
    #[test]
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
            rect,
            disabled: false,
            scope: 1,
            clip: None,
            chrome: false,
        }
    }

    fn checks_for(controls: &[Control]) -> Vec<String> {
        let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 300.0));
        inspect(controls, viewport, "seeded", "state", "400x300")
            .into_iter()
            .map(|violation| violation.check)
            .collect()
    }

    fn at(x: f32, y: f32, width: f32, height: f32) -> egui::Rect {
        egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, height))
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

    #[test]
    fn clipped_fails_on_a_control_cut_at_the_side_of_its_pane() {
        let mut cut = control("Save", at(10.0, 10.0, 80.0, 30.0));
        cut.clip = Some(at(0.0, 0.0, 50.0, 300.0));
        assert!(checks_for(&[cut]).contains(&"clipped".to_owned()));
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
        let details: Vec<String> = cut_text(&harness.output().shapes, "seeded", "state", "200x100")
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
