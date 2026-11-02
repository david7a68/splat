//! Drives the real app as a user would, through the AccessKit tree and raw input.
use super::{
    ergonomics::{self, TaskCost, Violation},
    fixture::World,
    video::{Overlay, Recorder},
};
use crate::Splat;
use eframe::egui::{
    self, ViewportId,
    accesskit::{self, Role},
};
use egui_kittest::{Harness, Node, kittest::NodeT};
use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

pub const SMALL: (f32, f32) = (960.0, 640.0);
pub const DEFAULT: (f32, f32) = (1200.0, 800.0);
pub const LARGE: (f32, f32) = (1920.0, 1080.0);
const ASYNC_TIMEOUT: Duration = Duration::from_secs(180);
const TAB_LIMIT: usize = 80;

/// The window sizes a state is captured and checked at.
#[derive(Clone, Copy, PartialEq)]
pub enum Sizes {
    Default,
    /// The same two sizes, plus the default size at twice the pixel density.
    DefaultAndHiDpi,
    /// The minimum, default and large sizes.
    All,
    /// All three, plus the default size at twice the pixel density.
    AllAndHiDpi,
}

#[derive(Default)]
pub struct Outcome {
    pub violations: Vec<Violation>,
    pub passes: Vec<ergonomics::Pass>,
    pub task_costs: Vec<TaskCost>,
    pub screenshots: Vec<String>,
    pub unreached: Vec<String>,
    pub not_measured: Vec<String>,
    pub look_review: Vec<String>,
    pub video: Option<String>,
}

pub struct Driver<'a> {
    harness: Harness<'a, Splat>,
    world: World,
    scenario: &'static str,
    /// This scenario's temporary folder.
    pub root: PathBuf,
    output: Option<PathBuf>,
    states: usize,
    checks_run: (usize, usize),
    interactions: u32,
    last_goal: u32,
    window: (f32, f32),
    density: f32,
    /// The size a scenario works at. The brief's tests are at the minimum window.
    base: (f32, f32),
    capturing: bool,
    video: Option<Recorder>,
    pointer: Option<egui::Pos2>,
    /// The current state has already been on screen long enough to read.
    held: bool,
    pub outcome: Outcome,
}

impl Outcome {
    /// A key that did its job is a pass. One that did not is a violation.
    pub(super) fn keyboard_result(
        &mut self,
        scenario: &str,
        window: String,
        (kind, view): (&str, &str),
        passed: bool,
        detail: String,
    ) {
        if passed {
            self.passes.push(ergonomics::Pass {
                check: "keyboard".into(),
                scenario: scenario.to_owned(),
                state: view.to_owned(),
                detail: format!("{kind}: {detail}"),
            });
        } else {
            self.violations.push(Violation {
                check: "keyboard".into(),
                scenario: scenario.to_owned(),
                state: view.to_owned(),
                window,
                node: kind.to_owned(),
                role: "keyboard".into(),
                rect: [0.0; 4],
                detail,
            });
        }
    }
}

/// One node's facts, copied out so the tree is free to change.
#[derive(Clone)]
pub struct Found {
    pub id: accesskit::NodeId,
    pub name: String,
    pub role: Role,
    /// The whole node, which can extend past its scroll area.
    pub rect: egui::Rect,
    /// The part of the node a person can see. Not positive when scrolled away.
    pub shown: egui::Rect,
    pub disabled: bool,
}

impl Found {
    pub fn is_link(&self) -> bool {
        self.role == Role::Link
    }

    pub fn in_view(&self) -> bool {
        self.shown.is_positive()
    }

    pub fn fully_in_view(&self) -> bool {
        self.in_view()
            && (self.shown.width() - self.rect.width()).abs() < 1.0
            && (self.shown.height() - self.rect.height()).abs() < 1.0
    }
}

fn node_name(node: &Node<'_>) -> String {
    let access = node.accesskit_node();
    access
        .label()
        .filter(|label| !label.trim().is_empty())
        .or_else(|| access.placeholder().map(str::to_owned))
        .or_else(|| access.value())
        .unwrap_or_default()
}

fn found(node: &Node<'_>, clip: Option<egui::Rect>, viewport: egui::Rect) -> Found {
    let access = node.accesskit_node();
    let rect = access
        .bounding_box()
        .map_or(egui::Rect::NOTHING, |rect| egui::Rect {
            min: egui::pos2(rect.x0 as f32, rect.y0 as f32),
            max: egui::pos2(rect.x1 as f32, rect.y1 as f32),
        });
    let shown = clip
        .map_or(rect, |clip| rect.intersect(clip))
        .intersect(viewport);
    Found {
        id: access.id(),
        name: node_name(node),
        role: access.role(),
        rect,
        shown,
        disabled: access.is_disabled(),
    }
}

impl<'a> Driver<'a> {
    pub fn new(
        world: &World,
        scenario: &'static str,
        root: &Path,
        output: Option<&Path>,
        record: bool,
    ) -> Self {
        let video = output.filter(|_| record).map(|directory| {
            Recorder::start(&directory.join("video").join(format!("{scenario}.mp4")))
                .unwrap_or_else(|error| panic!("{scenario}: {error}"))
        });
        let harness = Self::build_harness(world, output.is_some());
        let mut driver = Self {
            harness,
            world: world.clone(),
            scenario,
            root: root.to_path_buf(),
            output: output.map(Path::to_path_buf),
            states: 0,
            checks_run: (0, 0),
            interactions: 0,
            last_goal: 0,
            window: SMALL,
            density: 1.0,
            base: SMALL,
            capturing: output.is_some(),
            video,
            pointer: None,
            held: false,
            outcome: Outcome {
                violations: Vec::new(),
                passes: Vec::new(),
                task_costs: Vec::new(),
                screenshots: Vec::new(),
                unreached: Vec::new(),
                not_measured: Vec::new(),
                look_review: Vec::new(),
                video: None,
            },
        };
        driver.settle();
        driver.hold();
        driver
    }

    fn build_harness(world: &World, capturing: bool) -> Harness<'a, Splat> {
        let app = world.app();
        let builder = Harness::<Splat>::builder()
            .with_size(egui::vec2(SMALL.0, SMALL.1))
            .with_pixels_per_point(1.0)
            .with_max_steps(64);
        let builder = if capturing {
            builder.renderer(software_renderer())
        } else {
            builder
        };
        builder.build_eframe(move |cc| {
            crate::design::theme::install(&cc.egui_ctx);
            crate::art::install(&cc.egui_ctx);
            app
        })
    }

    /// Quit and start the app again on the same files, as a restart does.
    pub fn restart(&mut self) {
        // The old app writes its state as it runs. A new one reads that.
        self.harness = Self::build_harness(&self.world, self.capturing);
        self.window = self.base;
        self.resize(self.base, 1.0);
        self.settle();
        self.held = false;
        self.hold();
    }

    /// Work at another size until the next call.
    pub fn set_base(&mut self, size: (f32, f32)) {
        self.base = size;
        self.resize(size, 1.0);
    }

    /// The text on screen in full: painted, not cut to an ellipsis, inside the window and
    /// the pane that clips it. The accessible name can be whole when the visible text is not.
    pub fn visible_text(&self, text: &str) -> bool {
        self.painted_in(self.harness.ctx.content_rect())
            .contains(text)
    }

    /// What is painted from this label down, in its column and to its right.
    pub fn painted_from_label(&self, name: &str) -> String {
        self.nodes()
            .into_iter()
            .find(|node| node.name == name && node.role == Role::Label)
            .map_or_else(String::new, |label| {
                let window = self.harness.ctx.content_rect();
                self.painted_in(egui::Rect::from_min_max(label.rect.min, window.max))
            })
    }

    /// What the run's header paints: between the rail and `Next turn`, on its line.
    pub fn header_rect(&self) -> egui::Rect {
        let next = self
            .nodes()
            .into_iter()
            .find(|node| node.name == "Next turn")
            .expect("the run's header");
        egui::Rect::from_min_max(
            egui::pos2(self.harness.ctx.content_rect().left() + 136.0, 0.0),
            egui::pos2(next.rect.min.x, next.rect.max.y),
        )
    }

    /// The size of the painted text in this rect that holds these words.
    pub fn painted_size(&self, rect: egui::Rect, words: &str) -> Option<f32> {
        let mut found = None;
        self.each_painted_text(|text, _| {
            if rect.contains(text.visual_bounding_rect().center())
                && text.galley.job.text.contains(words)
            {
                found = text
                    .galley
                    .job
                    .sections
                    .first()
                    .map(|s| s.format.font_id.size);
            }
        });
        found
    }

    /// Each text painted now, with the clip of the pane it is in.
    fn each_painted_text(&self, mut visit: impl FnMut(&egui::epaint::TextShape, egui::Rect)) {
        fn walk(
            shape: &egui::Shape,
            clip: egui::Rect,
            visit: &mut impl FnMut(&egui::epaint::TextShape, egui::Rect),
        ) {
            match shape {
                egui::Shape::Vec(shapes) => {
                    shapes.iter().for_each(|shape| walk(shape, clip, visit));
                }
                egui::Shape::Text(text) => visit(text, clip),
                _ => {}
            }
        }
        for clipped in &self.harness.output().shapes {
            walk(&clipped.shape, clipped.clip_rect, &mut visit);
        }
    }

    /// Where a control is, by its exact name.
    pub fn place_of(&self, name: &str) -> Option<egui::Rect> {
        self.nodes()
            .into_iter()
            .find(|node| node.name == name)
            .map(|node| node.rect)
    }

    /// What the rail paints. It has its own words, whatever the page says.
    pub fn rail_words(&self) -> String {
        let window = self.harness.ctx.content_rect();
        self.painted_in(egui::Rect::from_min_max(
            window.min,
            egui::pos2(window.left() + 136.0, window.bottom()),
        ))
    }

    /// The node of a run row, by its participant's name.
    pub fn row_node(&self, name: &str) -> Option<Found> {
        let prefix = format!("{name},");
        self.nodes()
            .into_iter()
            .find(|node| node.name.starts_with(&prefix) && node.role == Role::Button)
    }

    /// Check a condition at the working size, then again at the default size.
    pub fn observe_both(&mut self, id: &str, detail: &str, check: impl Fn(&Self) -> bool) {
        let small = check(self);
        let other = if self.base == DEFAULT { SMALL } else { DEFAULT };
        let default = self.at_size(other, |d| check(d));
        self.observe(
            id,
            small && default,
            format!(
                "{detail}. {}x{}: {small}. {}x{}: {default}",
                self.base.0, self.base.1, other.0, other.1
            ),
        );
    }

    /// Run frames until no async load is pending and egui asks for no more repaints.
    pub fn settle(&mut self) {
        let started = Instant::now();
        let mut quiet = 0;
        let mut repainting = 0;
        loop {
            self.harness.step();
            let busy = self.harness.state().busy();
            let repaint = self
                .harness
                .output()
                .viewport_output
                .get(&ViewportId::ROOT)
                .is_some_and(|viewport| viewport.repaint_delay.is_zero());
            repainting = if repaint { repainting + 1 } else { 0 };
            quiet = if busy || (repaint && repainting < 32) {
                0
            } else {
                quiet + 1
            };
            if quiet >= 3 {
                return;
            }
            assert!(
                started.elapsed() < ASYNC_TIMEOUT,
                "{}: the app did not settle within {ASYNC_TIMEOUT:?}",
                self.scenario,
            );
            if busy {
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    }

    /// Every node in the tree, with the part of it that can be seen.
    pub fn nodes(&self) -> Vec<Found> {
        let viewport = self.harness.ctx.content_rect();
        let mut nodes = Vec::new();
        ergonomics::visit_nodes(&self.harness, |node, clip| {
            nodes.push(found(node, clip, viewport));
        });
        nodes
    }

    /// Lines for a failure message: everything on screen with a role or name.
    pub fn describe(&self) -> String {
        self.nodes()
            .iter()
            .filter(|node| {
                !matches!(
                    node.role,
                    Role::GenericContainer | Role::TextRun | Role::Unknown
                ) || !node.name.is_empty()
            })
            .map(|node| {
                format!(
                    "  {:?} {:?} {:?}{}",
                    node.role,
                    node.name,
                    node.shown,
                    if node.disabled { " (disabled)" } else { "" }
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The part of a visible node to aim at. A node that is in the tree but out of
    /// view is first scrolled in, which costs the user an interaction too.
    fn aim(&mut self, what: &str, test: impl Fn(&Found) -> bool) -> egui::Rect {
        for attempt in 0..2 {
            let matches: Vec<_> = self.nodes().into_iter().filter(|n| test(n)).collect();
            if let Some(node) = matches.iter().find(|node| node.in_view()) {
                return node.shown;
            }
            let Some(hidden) = matches.first().filter(|_| attempt == 0) else {
                break;
            };
            self.reveal(hidden.id);
        }
        panic!(
            "{}: no visible node for {what}.\n{}",
            self.scenario,
            self.describe()
        );
    }

    fn reveal(&mut self, id: accesskit::NodeId) {
        self.harness.event(egui::Event::AccessKitActionRequest(
            accesskit::ActionRequest {
                target: id,
                action: accesskit::Action::ScrollIntoView,
                data: None,
            },
        ));
        self.interactions += 1;
        self.harness.step();
        self.settle();
        self.film_result("Scroll into view");
    }

    /// Does any node carry exactly this name? Text in a label counts.
    pub fn has(&self, name: &str) -> bool {
        self.nodes().iter().any(|node| node.name == name)
    }

    pub fn has_containing(&self, name: &str) -> bool {
        self.nodes().iter().any(|node| node.name.contains(name))
    }

    /// Can a person see this text or control right now?
    pub fn in_view(&self, name: &str) -> bool {
        self.nodes()
            .iter()
            .any(|node| node.name == name && node.in_view())
    }

    /// Is this exact text on screen as reading text, not as a control or field?
    pub fn has_label(&self, name: &str) -> bool {
        self.nodes()
            .iter()
            .any(|node| node.role == Role::Label && node.name == name)
    }

    pub fn in_view_containing(&self, name: &str) -> bool {
        self.nodes()
            .iter()
            .any(|node| node.name.contains(name) && node.in_view())
    }

    /// Is a control with this text there, and disabled?
    pub fn is_disabled(&self, name: &str) -> bool {
        self.nodes()
            .iter()
            .any(|node| node.name.contains(name) && node.disabled && node.role != Role::Label)
    }

    /// Fail the scenario unless this text is on screen. Scripts call this so a
    /// screenshot can never silently show the wrong state.
    pub fn expect(&self, name: &str) {
        assert!(
            self.has_containing(name),
            "{}: expected {name:?} on screen.\n{}",
            self.scenario,
            self.describe()
        );
    }

    fn click_rect(&mut self, rect: egui::Rect, caption: &str) {
        let pos = rect.center();
        self.film_aim(rect, caption);
        self.harness.hover_at(pos);
        self.harness.step();
        for pressed in [true, false] {
            self.harness.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
        }
        self.harness.step();
        // Move the pointer away, so no hover state or tooltip enters the capture.
        self.harness.remove_cursor();
        self.interactions += 1;
        self.settle();
        let pressed = Overlay {
            pressed: true,
            ..Overlay::caption(caption)
        };
        self.film_frames(pressed, super::video::PRESS_FRAMES);
        self.film_result(caption);
    }

    /// Drag a column's edge sideways, halfway down the window.
    pub fn drag_column_edge(&mut self, from: f32, to: f32) {
        let y = self.window.1 / 2.0;
        let (start, end) = (egui::pos2(from, y), egui::pos2(to, y));
        self.harness.hover_at(start);
        self.harness.step();
        for (pos, pressed) in [(start, true), (end, true), (end, false)] {
            self.harness.hover_at(pos);
            self.harness.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            self.harness.step();
        }
        self.harness.remove_cursor();
        self.interactions += 1;
        self.settle();
        self.film_result("Drag a column's edge");
    }

    fn is_control(node: &Found) -> bool {
        !node.disabled && !matches!(node.role, Role::Label | Role::TextRun)
    }

    /// Click a control by its accessible name.
    pub fn click(&mut self, name: &str) {
        let rect = self.aim(&format!("control {name:?}"), |node| {
            node.name == name && Self::is_control(node)
        });
        self.click_rect(rect, &format!("Click: {name}"));
    }

    /// Click the one control whose name has this text. Two matches fail the script: a
    /// script that takes the first one can press Undo instead of a row, and not notice.
    pub fn click_containing(&mut self, name: &str) {
        let matches: Vec<String> = self
            .nodes()
            .into_iter()
            .filter(|node| node.name.contains(name) && Self::is_control(node))
            .map(|node| node.name)
            .collect();
        assert!(
            matches.len() <= 1,
            "{}: {name:?} matches more than one control: {matches:?}",
            self.scenario
        );
        let rect = self.aim(&format!("control containing {name:?}"), |node| {
            node.name.contains(name) && Self::is_control(node)
        });
        self.click_rect(rect, &format!("Click: {name}"));
    }

    /// Click the one control whose name starts with this text.
    pub fn click_starting(&mut self, name: &str) {
        let matches: Vec<String> = self
            .nodes()
            .into_iter()
            .filter(|node| node.name.starts_with(name) && Self::is_control(node))
            .map(|node| node.name)
            .collect();
        assert!(
            matches.len() <= 1,
            "{}: {name:?} starts more than one control: {matches:?}",
            self.scenario
        );
        let rect = self.aim(&format!("control starting {name:?}"), |node| {
            node.name.starts_with(name) && Self::is_control(node)
        });
        self.click_rect(rect, &format!("Click: {name}"));
    }

    /// Click a run row, by its participant's name.
    pub fn click_row(&mut self, name: &str) {
        let prefix = format!("{name},");
        let rect = self.aim(&format!("the row of {name:?}"), |node| {
            node.name.starts_with(&prefix) && Self::is_control(node)
        });
        self.click_rect(rect, &format!("Click: {name}"));
    }

    /// Every text painted inside this rect and whole: not cut to an ellipsis, and not
    /// outside the clip of the pane it is in. This is what a person can read there.
    pub fn painted_in(&self, rect: egui::Rect) -> String {
        let mut out = Vec::new();
        self.each_painted_text(|text, clip| {
            let bounds = text.visual_bounding_rect();
            if rect.contains(bounds.center())
                && !text.galley.elided
                && clip.expand(1.0).contains_rect(bounds)
            {
                out.push(text.galley.job.text.clone());
            }
        });
        out.join(" ")
    }

    /// Where the reader's title is, which moves when the reader scrolls.
    pub fn scroll_position_of_reader(&self) -> Option<f32> {
        self.nodes()
            .into_iter()
            .find(|node| node.role == Role::Label && node.name.starts_with("Library / "))
            .map(|node| node.rect.min.y)
    }

    /// Drop keyboard focus, as a click on plain text does.
    pub fn harness_focus_nothing(&mut self) {
        self.clear_focus();
    }

    /// The buttons in the rail that open a page.
    pub fn rail_page_buttons(&self) -> Vec<String> {
        self.nodes()
            .into_iter()
            .filter(|node| {
                node.role == Role::Button
                    && !node.disabled
                    && node.rect.max.x < 140.0
                    && !node.name.is_empty()
                    && node.name != "Lookup"
                    && node.name != "Notes"
            })
            .map(|node| node.name)
            .collect()
    }

    /// The page's title: the first label at the top of the page, right of the rail.
    pub fn page_title(&self) -> Option<String> {
        self.nodes()
            .into_iter()
            .filter(|node| {
                node.role == Role::Label && node.rect.min.x > 140.0 && node.rect.min.y < 40.0
            })
            .min_by(|a, b| a.rect.min.x.total_cmp(&b.rect.min.x))
            .map(|node| node.name)
    }

    /// What a run row really paints, which its accessible name need not match.
    pub fn row_words(&self, name: &str) -> String {
        self.row_node(name)
            .map_or_else(String::new, |row| self.painted_in(row.shown))
    }

    /// The name in the dock, which is the selected participant's.
    pub fn focused_row_name(&self) -> Option<String> {
        self.nodes()
            .into_iter()
            .find(|node| node.name.starts_with("Selected: "))
            .map(|node| node.name)
    }

    /// A run row's current HP, from its accessible name.
    pub fn row_hp(&self, name: &str) -> Option<i64> {
        let row = self.row_node(name)?;
        let (_, rest) = row.name.split_once("HP ")?;
        rest.split('/').next()?.parse().ok()
    }

    /// The text in the Markdown editor, or in another multiline field.
    pub fn editor_text(&self) -> Option<String> {
        self.harness
            .root()
            .children_recursive()
            .find(|node| node.accesskit_node().role() == Role::MultilineTextInput)
            .and_then(|node| node.accesskit_node().value())
    }

    /// The text in a single-line field, by its accessible name.
    pub fn field_text(&self, name: &str) -> Option<String> {
        self.harness
            .root()
            .children_recursive()
            .find(|node| {
                let access = node.accesskit_node();
                access.role() == Role::TextInput && access.label().as_deref() == Some(name)
            })
            .and_then(|node| node.accesskit_node().value())
    }

    /// Every disabled control here that gives no reason when the pointer rests on it.
    pub fn disabled_without_reason(&mut self) -> Vec<String> {
        self.disabled_without_reason_except(&[])
    }

    /// A dialog turns off what is behind it, and that says nothing about what is in it.
    pub fn disabled_without_reason_except(&mut self, behind: &[String]) -> Vec<String> {
        let names: Vec<(String, egui::Rect)> = self
            .nodes()
            .into_iter()
            .filter(|node| {
                node.disabled
                    && node.fully_in_view()
                    && !node.name.is_empty()
                    && !behind.contains(&node.name)
                    && matches!(
                        node.role,
                        Role::Button | Role::ComboBox | Role::TextInput | Role::CheckBox
                    )
            })
            .map(|node| (node.name, node.shown))
            .collect();
        let mut silent = Vec::new();
        for (name, rect) in names {
            let window = self.harness.ctx.content_rect();
            let before = self.painted_in(window);
            self.harness.hover_at(rect.center());
            self.harness.run_steps(6);
            // A reason is words, not any new thing on screen. Three is the shortest.
            let after = self.painted_in(window);
            let added = after
                .split_whitespace()
                .count()
                .saturating_sub(before.split_whitespace().count());
            if added < 3 {
                silent.push(name);
            }
            self.harness.remove_cursor();
            self.settle();
        }
        silent
    }

    /// Click the first visible control with this name, top to bottom.
    pub fn click_first(&mut self, name: &str) {
        let first = self
            .nodes()
            .into_iter()
            .find(|node| node.name == name && Self::is_control(node) && node.in_view());
        let Some(node) = first else {
            panic!(
                "{}: no control named {name:?}.\n{}",
                self.scenario,
                self.describe()
            );
        };
        let rect = node.shown;
        self.click_rect(rect, &format!("Click: {name}"));
    }

    /// Click a control on the same row as some text, such as a saved run's Resume.
    pub fn click_beside(&mut self, text: &str, name: &str) {
        let row = self.row_of(text);
        let rect = self.aim(&format!("{name:?} beside {text:?}"), |node| {
            node.name == name
                && Self::is_control(node)
                && node.rect.center().y >= row.min.y - 12.0
                && node.rect.center().y <= row.max.y + 12.0
        });
        self.click_rect(rect, &format!("Click: {name} beside {text}"));
    }

    /// Like `click_beside`, for a row whose text is exactly this, among longer texts.
    pub fn click_beside_exact(&mut self, text: &str, name: &str) {
        let row = self.aim(&format!("row with exactly {text:?}"), |node| {
            node.name == text && !matches!(node.role, Role::TextRun)
        });
        let rect = self.aim(&format!("{name:?} beside {text:?}"), |node| {
            node.name == name
                && Self::is_control(node)
                && node.rect.center().y >= row.min.y - 16.0
                && node.rect.center().y <= row.max.y + 16.0
        });
        self.click_rect(rect, &format!("Click: {name} beside {text}"));
    }

    /// Click the first control with this name below some text, such as a row's field.
    pub fn click_below(&mut self, text: &str, name: &str) {
        let row = self.row_of(text);
        let mut below: Vec<_> = self
            .nodes()
            .into_iter()
            .filter(|node| {
                // A row's own controls can sit a little above its text.
                node.name == name && Self::is_control(node) && node.rect.min.y >= row.min.y - 12.0
            })
            .collect();
        below.sort_by(|a, b| a.rect.min.y.total_cmp(&b.rect.min.y));
        let Some(node) = below.first() else {
            panic!(
                "{}: no {name:?} below {text:?}.\n{}",
                self.scenario,
                self.describe()
            );
        };
        let id = node.id;
        let rect = self.aim(&format!("{name:?} below {text:?}"), |node| node.id == id);
        self.click_rect(rect, &format!("Click: {name} for {text}"));
    }

    /// Where the text is, scrolled into view if the row is out of view.
    pub fn row_of(&mut self, text: &str) -> egui::Rect {
        self.aim(&format!("row with {text:?}"), |node| {
            node.name.contains(text) && !matches!(node.role, Role::TextRun)
        })
    }

    /// Open the `index`th dropdown on screen, top to bottom in tree order.
    pub fn open_combo_at(&mut self, index: usize) {
        let matches: Vec<_> = self
            .nodes()
            .into_iter()
            .filter(|node| node.role == Role::ComboBox && node.in_view())
            .collect();
        let rect = matches
            .get(index)
            .unwrap_or_else(|| {
                panic!(
                    "{}: no dropdown {index}.\n{}",
                    self.scenario,
                    self.describe()
                )
            })
            .shown;
        self.click_rect(rect, "Click: dropdown");
    }

    fn is_text_input(role: Role) -> bool {
        matches!(
            role,
            Role::TextInput | Role::MultilineTextInput | Role::PasswordInput
        )
    }

    fn click_field(&mut self, field: &str) {
        let rect = self.aim(&format!("text field {field:?}"), |node| {
            Self::is_text_input(node.role)
                && !node.disabled
                && (node.name == field || node.name.starts_with(&format!("{field}: ")))
        });
        self.click_rect(rect, &format!("Click: {field}"));
    }

    /// Focus a text field by its hint or current text, then type into it.
    pub fn type_into(&mut self, field: &str, text: &str) {
        self.click_field(field);
        self.type_text(text);
    }

    /// Focus a field by its hint or text, replace what it holds, then type.
    pub fn replace_text_in(&mut self, field: &str, text: &str) {
        self.click_field(field);
        self.press_ctrl(egui::Key::A);
        self.type_text(text);
    }

    /// Focus a field by its text, then empty it.
    pub fn clear_text_in(&mut self, field: &str) {
        self.click_field(field);
        self.press_ctrl(egui::Key::A);
        self.press(egui::Key::Backspace);
    }

    /// Focus the visible Markdown editor, go to its end, then type.
    pub fn type_into_editor(&mut self, text: &str) {
        let rect = self.aim("the Markdown editor", |node| {
            node.role == Role::MultilineTextInput
        });
        self.click_rect(rect, "Click: Markdown editor");
        self.press_ctrl(egui::Key::End);
        self.type_text(text);
    }

    /// Replace everything in the visible Markdown editor.
    pub fn replace_editor_text(&mut self, text: &str) {
        let rect = self.aim("the Markdown editor", |node| {
            node.role == Role::MultilineTextInput
        });
        self.click_rect(rect, "Click: Markdown editor");
        self.press_ctrl(egui::Key::A);
        self.type_text(text);
    }

    /// Type into the field that has focus.
    pub fn type_text(&mut self, text: &str) {
        let caption = format!("Type: {text}");
        self.film_caption(&caption);
        self.harness.event(egui::Event::Text(text.to_owned()));
        self.interactions += 1;
        self.settle();
        self.film_result(&caption);
    }

    pub fn press(&mut self, key: egui::Key) {
        let caption = format!("Press: {}", key.name());
        self.film_caption(&caption);
        self.harness.key_press(key);
        self.interactions += 1;
        self.settle();
        self.film_result(&caption);
    }

    /// Shift+key.
    pub fn press_shift(&mut self, key: egui::Key) {
        let caption = format!("Press: Shift+{}", key.name());
        self.film_caption(&caption);
        self.harness.key_press_modifiers(
            egui::Modifiers {
                shift: true,
                ..Default::default()
            },
            key,
        );
        self.interactions += 1;
        self.settle();
        self.film_result(&caption);
    }

    /// Ctrl+key, with the modifiers a Linux desktop reports.
    pub fn press_ctrl(&mut self, key: egui::Key) {
        let caption = format!("Press: Ctrl+{}", key.name());
        self.film_caption(&caption);
        self.harness.key_press_modifiers(
            egui::Modifiers {
                ctrl: true,
                command: true,
                ..Default::default()
            },
            key,
        );
        self.interactions += 1;
        self.settle();
        self.film_result(&caption);
    }

    /// Press Tab until a control has focus. A person working by keyboard alone does
    /// the same. Returns false if Tab never reached it.
    pub fn tab_to(&mut self, name: &str) -> bool {
        let caption = format!("Keyboard: Tab to {name}");
        self.film_caption(&caption);
        for _ in 0..TAB_LIMIT {
            self.harness.key_press(egui::Key::Tab);
            self.harness.run_steps(4);
            self.interactions += 1;
            let focused = self
                .harness
                .root()
                .children_recursive()
                .find(|node| node.accesskit_node().is_focused())
                .map(|node| node_name(&node));
            if focused.as_deref() == Some(name) {
                self.film_result(&caption);
                return true;
            }
        }
        self.clear_focus();
        false
    }

    /// Tab to a control and press Enter.
    pub fn activate_by_keyboard(&mut self, name: &str) -> bool {
        if !self.tab_to(name) {
            return false;
        }
        self.press(egui::Key::Enter);
        self.clear_focus();
        true
    }

    /// The name of each enabled button in the page, right of the navigation rail.
    pub fn enabled_page_buttons(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .nodes()
            .into_iter()
            .filter(|node| {
                node.role == Role::Button
                    && !node.disabled
                    && node.in_view()
                    && !node.name.is_empty()
                    && node.rect.min.x > 140.0
            })
            .map(|node| node.name)
            .collect();
        names.dedup();
        names
    }

    /// Rest the pointer on a control until any tooltip shows, and read the screen.
    pub fn tooltip_contains(&mut self, name: &str, text: &str) -> bool {
        let rect = self.aim(&format!("hover target {name:?}"), |node| {
            node.name == name && !matches!(node.role, Role::Label | Role::TextRun)
        });
        self.film_aim(rect, &format!("Hover: {name}"));
        self.harness.hover_at(rect.center());
        // A tooltip waits half a second, and one frame here is a quarter second.
        self.harness.run_steps(6);
        self.film_result(&format!("Hover: {name}"));
        let shown = self.has_containing(text);
        self.harness.remove_cursor();
        self.settle();
        shown
    }

    /// Wheel-scroll down over this point. One notch is 100 points.
    pub fn scroll_at(&mut self, pos: egui::Pos2, notches: u32) {
        self.film_aim(
            egui::Rect::from_center_size(pos, egui::Vec2::ZERO),
            "Scroll",
        );
        self.harness.hover_at(pos);
        self.harness.step();
        for _ in 0..notches {
            self.harness.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, -100.0),
                modifiers: egui::Modifiers::NONE,
            });
            self.harness.step();
            self.interactions += 1;
            self.film_frames(Overlay::caption("Scroll"), 2);
        }
        self.harness.remove_cursor();
        self.settle();
        self.film_result("Scroll");
    }

    /// Wheel-scroll over this point until a virtual list shows the text.
    pub fn scroll_until(&mut self, pos: egui::Pos2, text: &str, limit: u32) {
        for _ in 0..limit {
            if self.in_view_containing(text) {
                return;
            }
            self.scroll_at(pos, 1);
        }
        panic!(
            "{}: {text:?} did not appear after {limit} scroll notches.\n{}",
            self.scenario,
            self.describe()
        );
    }

    /// Ask the window to close, as the window manager's close button does.
    pub fn request_close(&mut self) {
        self.harness
            .input_mut()
            .viewports
            .get_mut(&ViewportId::ROOT)
            .expect("root viewport")
            .events
            .push(egui::ViewportEvent::Close);
        self.interactions += 1;
        self.settle();
    }

    pub fn interaction_count(&self) -> u32 {
        self.interactions
    }

    /// Start counting a new task here, without recording the steps before it.
    pub fn begin_task(&mut self) {
        self.last_goal = self.interactions;
    }

    /// Record how many interactions the user needed to reach a goal, counted
    /// from the previous goal in this scenario.
    pub fn goal(&mut self, goal: &str) -> u32 {
        let cost = self.interactions - self.last_goal;
        self.outcome.task_costs.push(TaskCost {
            scenario: self.scenario.to_owned(),
            goal: goal.to_owned(),
            interactions: cost,
            total: self.interactions,
        });
        self.last_goal = self.interactions;
        cost
    }

    /// A goal with a target from `docs/ux-intent.md` section 8.
    pub fn goal_within(&mut self, id: &str, goal: &str, target: u32) {
        let cost = self.goal(&format!("{id}: {goal}"));
        self.observe(
            id,
            cost <= target,
            format!("{goal}: {cost} interactions, target {target}"),
        );
    }

    /// The result of an observable test from `docs/ux-intent.md` section 8.
    pub fn observe(&mut self, id: &str, passed: bool, detail: impl Into<String>) {
        let detail = detail.into();
        if passed {
            self.outcome.passes.push(ergonomics::Pass {
                check: "workflow".into(),
                scenario: self.scenario.to_owned(),
                state: id.to_owned(),
                detail: format!("{id}: {detail}"),
            });
        } else {
            self.outcome.violations.push(Violation {
                check: "workflow".into(),
                scenario: self.scenario.to_owned(),
                state: id.to_owned(),
                window: format!("{}x{}", self.window.0, self.window.1),
                node: id.to_owned(),
                role: "workflow".into(),
                rect: [0.0; 4],
                detail,
            });
        }
    }

    /// Say plainly that the script could not reach a state the brief asks for.
    pub fn unreached(&mut self, state: &str, nearest: &str) {
        self.outcome
            .unreached
            .push(format!("{}: {state}. Nearest: {nearest}", self.scenario));
    }

    /// A workflow test this harness cannot measure honestly.
    pub fn not_measured(&mut self, id: &str, why: &str) {
        self.outcome
            .not_measured
            .push(format!("{id} ({}): {why}", self.scenario));
    }

    /// A workflow test a person must judge from the named capture.
    pub fn review(&mut self, id: &str, what: &str) {
        self.outcome.look_review.push(format!(
            "{id} ({}): {what}, state {:02}",
            self.scenario, self.states
        ));
    }

    /// Test-only access to app state that no UI route can create.
    pub fn app_mut(&mut self) -> &mut Splat {
        self.held = false;
        self.harness.state_mut()
    }

    /// The longest line of visible text, to check reading width.
    pub fn longest_visible_line(&self) -> usize {
        self.nodes()
            .iter()
            .filter(|node| node.role == Role::TextRun && node.in_view())
            .map(|node| node.name.trim().chars().count())
            .max()
            .unwrap_or(0)
    }

    /// Look at the app at another window size, off camera, then come back.
    pub fn at_size<T>(&mut self, size: (f32, f32), look: impl FnOnce(&Self) -> T) -> T {
        self.resize(size, 1.0);
        let result = look(self);
        self.resize(self.base, 1.0);
        result
    }

    fn recording(&self) -> bool {
        self.video.is_some() && self.window == SMALL && self.density == 1.0
    }

    fn film_frames(&mut self, overlay: Overlay, frames: usize) {
        if !self.recording() {
            return;
        }
        let overlay = Overlay {
            pointer: overlay.pointer.or(self.pointer),
            ..overlay
        };
        let image = self
            .harness
            .render()
            .unwrap_or_else(|error| panic!("{}: render failed: {error}", self.scenario));
        if let Some(video) = &mut self.video {
            video.frames(&image, &overlay, frames);
        }
    }

    /// Move the pointer to a target and mark it, before the app reacts.
    fn film_aim(&mut self, target: egui::Rect, caption: &str) {
        if !self.recording() {
            self.pointer = Some(target.center());
            return;
        }
        let image = self
            .harness
            .render()
            .unwrap_or_else(|error| panic!("{}: render failed: {error}", self.scenario));
        let end = target.center();
        let start = self.pointer.unwrap_or(end);
        if let Some(video) = &mut self.video {
            video.aim(&image, start, target, caption);
        }
        self.pointer = Some(end);
    }

    fn film_caption(&mut self, caption: &str) {
        self.film_frames(Overlay::caption(caption), super::video::AIM_FRAMES);
    }

    /// Show the settled result of a step for long enough to read.
    fn film_result(&mut self, caption: &str) {
        self.film_frames(Overlay::caption(caption), super::video::HOLD_FRAMES);
        self.held = true;
    }

    fn hold(&mut self) {
        if !self.held {
            self.film_frames(Overlay::default(), super::video::HOLD_FRAMES);
            self.held = true;
        }
    }

    fn resize(&mut self, size: (f32, f32), density: f32) {
        self.window = size;
        self.density = density;
        self.harness.set_size(egui::vec2(size.0, size.1));
        self.harness
            .input_mut()
            .viewports
            .get_mut(&ViewportId::ROOT)
            .expect("root viewport")
            .native_pixels_per_point = Some(density);
        self.settle();
    }

    /// Capture and check the app in its current state.
    pub fn state(&mut self, name: &str, sizes: Sizes) {
        self.states += 1;
        let label = format!("{:02}-{name}", self.states);
        let plan: &[((f32, f32), f32)] = match sizes {
            Sizes::Default => &[(SMALL, 1.0), (DEFAULT, 1.0)],
            Sizes::DefaultAndHiDpi => &[(SMALL, 1.0), (DEFAULT, 1.0), (DEFAULT, 2.0)],
            Sizes::All => &[(SMALL, 1.0), (DEFAULT, 1.0), (LARGE, 1.0)],
            Sizes::AllAndHiDpi => &[(SMALL, 1.0), (DEFAULT, 1.0), (LARGE, 1.0), (DEFAULT, 2.0)],
        };
        for &(size, density) in plan {
            self.resize(size, density);
            // Logical layout at twice the density matches the 1x capture. The minimum
            // window is the harder test, so a state checked once is checked there.
            let one_size = matches!(sizes, Sizes::Default | Sizes::DefaultAndHiDpi);
            if density == 1.0 && (!one_size || size == SMALL) {
                self.check(&label, size);
            }
            // G4's one high density. Egui's chrome is counted once, at 1x.
            if density != 1.0 {
                let before = self.outcome.violations.len();
                self.check(&format!("{label}@2x"), size);
                let added = self.outcome.violations.split_off(before);
                self.outcome.violations.extend(
                    added
                        .into_iter()
                        .filter(|violation| !violation.check.ends_with(ergonomics::CHROME_SUFFIX)),
                );
            }
            self.capture(&label, size, density);
        }
        self.resize(self.base, 1.0);
        self.hold();
    }

    /// How many times the checks ran at the minimum window, and at high density.
    pub fn checks_run(&self) -> (usize, usize) {
        self.checks_run
    }

    pub fn states_shown(&self) -> usize {
        self.states
    }

    fn check(&mut self, state: &str, size: (f32, f32)) {
        if self.harness.ctx.pixels_per_point() > 1.0 {
            self.checks_run.1 += 1;
        } else if size == SMALL {
            self.checks_run.0 += 1;
        }
        // AccessKit reports pixels, and the checks work in points.
        let mut controls = ergonomics::controls(&self.harness);
        let scale = 1.0 / self.harness.ctx.pixels_per_point();
        for control in &mut controls {
            control.rect = control.rect * scale;
            control.clip = control.clip.map(|clip| clip * scale);
        }
        let viewport = self.harness.ctx.content_rect();
        let window = format!("{}x{}", size.0, size.1);
        self.outcome.violations.extend(ergonomics::inspect(
            &controls,
            viewport,
            self.scenario,
            state,
            &window,
        ));
        self.outcome.violations.extend(ergonomics::cut_text(
            &self.harness.output().shapes,
            self.scenario,
            state,
            &window,
        ));
    }

    fn capture(&mut self, state: &str, size: (f32, f32), density: f32) {
        let Some(directory) = &self.output else {
            return;
        };
        let file = format!(
            "{}__{state}__{}x{}{}.png",
            self.scenario,
            size.0,
            size.1,
            if density == 1.0 { "" } else { "@2x" }
        );
        let path = directory.join(&file);
        let image = self
            .harness
            .render()
            .unwrap_or_else(|error| panic!("{}: render failed: {error}", self.scenario));
        image
            .save(&path)
            .unwrap_or_else(|error| panic!("Could not write {}: {error}", path.display()));
        self.outcome.screenshots.push(file);
    }

    /// Capture the current state with the pointer resting on a control.
    pub fn state_hovering(&mut self, name: &str, target: &str) {
        self.states += 1;
        let label = format!("{:02}-{name}", self.states);
        let rect = self.aim(&format!("hover target {target:?}"), |node| {
            node.name == target && !matches!(node.role, Role::Label | Role::TextRun)
        });
        self.harness.hover_at(rect.center());
        self.harness.run_steps(2);
        self.capture(&label, self.base, 1.0);
        self.harness.remove_cursor();
        self.settle();
    }

    /// Press Tab through the current view and report how far focus travels.
    /// Input focus is cleared afterwards, so the script continues as it was.
    /// The accessible name of the control with keyboard focus.
    pub fn focused_name(&self) -> Option<String> {
        self.harness
            .root()
            .children_recursive()
            .find(|node| node.accesskit_node().is_focused())
            .map(|node| node_name(&node))
    }

    pub fn check_tab_order(&mut self, view: &str) {
        let controls = ergonomics::controls(&self.harness);
        let expected: Vec<u64> = controls
            .iter()
            .filter(|control| !control.disabled && !control.chrome && !control.is_label())
            .map(|control| control.id)
            .collect();
        let total = controls
            .iter()
            .filter(|control| !control.disabled && !control.is_label())
            .count();
        let mut reached: Vec<u64> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        let mut chrome_stops = 0;
        let mut stops: Vec<u64> = Vec::new();
        // Pressing past every stop shows the order wraps, rather than stalls.
        for _ in 0..(total + 2).min(TAB_LIMIT) {
            self.harness.key_press(egui::Key::Tab);
            // The app steps over an unnamed handle, which takes frames a person never sees.
            self.harness.run_steps(4);
            let focused = self
                .harness
                .root()
                .children_recursive()
                .find(|node| node.accesskit_node().is_focused())
                .map(|node| (node.accesskit_node().id().0, node_name(&node)));
            let Some((id, name)) = focused else {
                continue;
            };
            if stops.contains(&id) {
                continue;
            }
            stops.push(id);
            if expected.contains(&id) {
                reached.push(id);
                names.push(name);
            } else {
                chrome_stops += 1;
            }
        }
        self.clear_focus();
        let target = expected
            .len()
            .min(TAB_LIMIT - chrome_stops.min(TAB_LIMIT - 1));
        if chrome_stops > 0 {
            self.keyboard_result(
                "tab-stops-on-handles",
                view,
                false,
                format!(
                    "Tab stops on {chrome_stops} egui handles that have no name or visible focus"
                ),
            );
        }
        let shown = names
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        let detail = format!(
            "Tab reached {} of {} enabled controls, with {chrome_stops} extra stops on egui handles. First stops: {shown}",
            reached.len(),
            expected.len(),
        );
        self.keyboard_result(
            "tab-order",
            view,
            !expected.is_empty() && reached.len() >= target,
            detail,
        );
    }

    /// Keyboard results use the same pass and violation records as other checks.
    pub fn keyboard_result(&mut self, kind: &str, view: &str, passed: bool, detail: String) {
        self.outcome.keyboard_result(
            self.scenario,
            format!("{}x{}", self.base.0, self.base.1),
            (kind, view),
            passed,
            detail,
        );
    }

    fn clear_focus(&mut self) {
        self.harness.ctx.memory_mut(|memory| {
            if let Some(id) = memory.focused() {
                memory.surrender_focus(id);
            }
        });
        self.settle();
    }

    /// End the video here, with a final hold on the current state.
    pub fn stop_filming(&mut self) {
        self.held = false;
        self.hold();
        if let Some(video) = self.video.take() {
            match video.finish() {
                Ok(name) => self.outcome.video = Some(name),
                Err(error) => panic!("{}: {error}", self.scenario),
            }
        }
    }

    pub fn finish(mut self) -> Outcome {
        self.stop_filming();
        self.outcome
    }
}

fn software_renderer() -> egui_kittest::wgpu::WgpuTestRenderer {
    use egui_kittest::wgpu::{WgpuTestRenderer, create_render_state, default_wgpu_setup};
    let state = create_render_state(default_wgpu_setup());
    let adapter = state.adapter.get_info();
    assert_eq!(
        adapter.device_type,
        eframe::wgpu::DeviceType::Cpu,
        "Captures must use a software rasterizer so they repeat between machines. Found {} ({:?}). Use `devenv shell -- visual-test`.",
        adapter.name,
        adapter.backend,
    );
    WgpuTestRenderer::from_render_state(state)
}
