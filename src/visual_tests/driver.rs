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
    cell::RefCell,
    collections::BTreeMap,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

pub const SMALL: (f32, f32) = (960.0, 640.0);
pub const DEFAULT: (f32, f32) = (1200.0, 800.0);
pub const LARGE: (f32, f32) = (1920.0, 1080.0);
const ASYNC_TIMEOUT: Duration = Duration::from_secs(180);
// Diagnostic feedback may stop early, but must still fail on missing text.
// The full gate retains the original allowance for slow background work.
const DIAGNOSTIC_TEXT_TIMEOUT: Duration = Duration::from_secs(5);
const TAB_LIMIT: usize = 80;

pub(super) fn check_settle_deadline(scenario: &str, started: Instant, timeout: Duration) {
    if started.elapsed() < timeout {
        return;
    }
    let message = format!("{scenario}: the app did not settle within {timeout:?}");
    #[cfg(target_os = "linux")]
    if super::isolation::is_child() {
        // Unwinding can block in driver cleanup or a scoped sibling's join.
        // The parent owns admission and reports this unsuccessful child status.
        eprintln!("{message}");
        std::process::exit(101);
    }
    panic!("{message}");
}

/// The window sizes a state is captured and checked at.
#[derive(Clone, Copy, PartialEq)]
pub enum Sizes {
    /// Capture both sizes; run ergonomics at the minimum window.
    Default,
    /// Capture and check the minimum and default sizes.
    Standard,
    /// Both sizes checked, plus the default size at twice the pixel density.
    StandardAndHiDpi,
    /// Only the minimum size, for a change specific to the narrow layout.
    Small,
    /// The same two sizes, plus the default size at twice the pixel density.
    DefaultAndHiDpi,
    /// The minimum, default and large sizes.
    All,
    /// All three, plus the default size at twice the pixel density.
    AllAndHiDpi,
}

impl Sizes {
    fn plan(self) -> &'static [((f32, f32), f32)] {
        match self {
            Self::Small => &[(SMALL, 1.0)],
            Self::Default | Self::Standard => &[(SMALL, 1.0), (DEFAULT, 1.0)],
            Self::DefaultAndHiDpi | Self::StandardAndHiDpi => {
                &[(SMALL, 1.0), (DEFAULT, 1.0), (DEFAULT, 2.0)]
            }
            Self::All => &[(SMALL, 1.0), (DEFAULT, 1.0), (LARGE, 1.0)],
            Self::AllAndHiDpi => &[(SMALL, 1.0), (DEFAULT, 1.0), (LARGE, 1.0), (DEFAULT, 2.0)],
        }
    }
}

#[derive(Clone, Copy)]
enum Capture<'s> {
    Save,
    SaveAt(&'s [((f32, f32), f32)]),
    Compare {
        earlier: &'s str,
        changed: &'s [((f32, f32), f32)],
    },
    ChecksOnly,
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
    pub failed_expectations: Vec<String>,
}

pub struct Driver<'a> {
    harness: Harness<'a, Splat>,
    world: World,
    scenario: &'static str,
    /// This scenario's temporary folder.
    pub root: PathBuf,
    output: Option<PathBuf>,
    states: usize,
    named_states: BTreeMap<String, (String, Sizes)>,
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
    /// Each top-level page the app showed, with the interaction count when it appeared.
    pub page_trail: Vec<(u32, String)>,
    pub outcome: Outcome,
    failed_expectations: RefCell<Vec<String>>,
    diagnose: bool,
    targeted: bool,
    timing: CaptureTiming,
    repeated_captures: super::repeated_captures::RepeatedCaptures,
    expect_same: Option<&'static str>,
    /// A probe can expire the real settle loop without waiting three minutes.
    pub(super) settle_timeout: Duration,
    /// The last rendered frame, keyed by the harness and the app pass it shows. Video holds,
    /// aims and captures often show a pass that is already rendered.
    last_render: Option<((usize, u64), std::rc::Rc<image::RgbaImage>)>,
    /// Counts restarts: a new harness counts its passes from zero again.
    harness_generation: usize,
}

#[derive(Default)]
struct CaptureTiming {
    render: Duration,
    png: Duration,
    settle: Duration,
    renders: usize,
    saves: usize,
    settle_frames: usize,
    settles: usize,
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
        let started = Instant::now();
        let harness = Self::build_harness(world, output.is_some());
        if std::env::var_os("SPLAT_VISUAL_TIMINGS").is_some() {
            println!(
                "Timing {scenario}: app + fonts + renderer = {:?}",
                started.elapsed()
            );
        }
        let mut driver = Self {
            harness,
            world: world.clone(),
            scenario,
            root: root.to_path_buf(),
            output: output.map(Path::to_path_buf),
            states: 0,
            named_states: BTreeMap::new(),
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
            page_trail: Vec::new(),
            failed_expectations: RefCell::default(),
            diagnose: std::env::var_os("SPLAT_VISUAL_DIAGNOSE").is_some(),
            targeted: std::env::var_os("SPLAT_VISUAL_SCENARIOS").is_some(),
            timing: CaptureTiming::default(),
            repeated_captures: Default::default(),
            expect_same: None,
            settle_timeout: ASYNC_TIMEOUT,
            last_render: None,
            harness_generation: 0,
            outcome: Outcome {
                violations: Vec::new(),
                passes: Vec::new(),
                task_costs: Vec::new(),
                screenshots: Vec::new(),
                unreached: Vec::new(),
                not_measured: Vec::new(),
                look_review: Vec::new(),
                video: None,
                failed_expectations: Vec::new(),
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
            builder.renderer(super::render::CaptureRenderer::new())
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
        self.harness_generation += 1;
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

    pub fn window_size(&self) -> (f32, f32) {
        self.window
    }

    /// The text on screen in full: painted, not cut to an ellipsis, inside the window and
    /// the pane that clips it. The accessible name can be whole when the visible text is not.
    pub fn visible_text(&self, text: &str) -> bool {
        self.painted_in(self.harness.ctx.content_rect())
            .contains(text)
    }

    /// A passage inside a scrolling source editor can be visible while its whole galley is not.
    pub fn visible_passage(&self, words: &str) -> bool {
        let mut visible = false;
        self.each_painted_text(|text, clip| {
            if let Some(at) = text.galley.job.text.find(words) {
                let start = text.galley.job.text[..at].chars().count();
                let end = start + words.chars().count();
                let bounds = text
                    .galley
                    .pos_from_cursor(egui::text::CCursor::new(start))
                    .union(text.galley.pos_from_cursor(egui::text::CCursor::new(end)))
                    .translate(text.pos.to_vec2());
                visible |= clip.expand(1.0).contains_rect(bounds)
                    && self.harness.ctx.content_rect().contains_rect(bounds);
            }
        });
        visible
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

    /// The dock must identify exactly one row, including when Lookup hides the sheet.
    pub fn dock_names_one_participant(&self) -> bool {
        let nodes = self.nodes();
        let Some(label) = nodes
            .iter()
            .find(|n| n.name.starts_with("Selected: ") && n.role == Role::Label)
        else {
            return false;
        };
        let shown = self.painted_in(label.rect);
        let words: Vec<_> = shown
            .split([' ', '…', '\n'])
            .filter(|w| !w.is_empty())
            .collect();
        if words.is_empty() {
            return false;
        }
        let candidates = nodes
            .iter()
            .filter(|n| n.name.contains(", HP ") || n.name.contains(", initiative "))
            .filter(|n| {
                let name = n.name.split(',').next().unwrap_or("");
                let parts: Vec<_> = name.split_whitespace().collect();
                words.iter().all(|word| parts.contains(word))
            })
            .count();
        candidates == 1
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

    pub fn painted_left(&self, words: &str) -> Option<f32> {
        let mut left = None;
        self.each_painted_text(|text, clip| {
            let bounds = text.visual_bounding_rect();
            if text.galley.job.text.contains(words) && clip.intersects(bounds) {
                left = Some(text.pos.x);
            }
        });
        left
    }

    /// Every horizontal hairline painted inside an area, as a rect one pixel high, such as the
    /// rule under a section heading, which spans its column.
    pub fn hairlines_in(&self, area: egui::Rect) -> Vec<egui::Rect> {
        fn visit(shape: &egui::Shape, area: egui::Rect, found: &mut Vec<egui::Rect>) {
            match shape {
                egui::Shape::Vec(shapes) => {
                    shapes.iter().for_each(|shape| visit(shape, area, found));
                }
                egui::Shape::LineSegment { points, stroke }
                    if stroke.color == crate::design::colors().hairline
                        && (points[0].y - points[1].y).abs() < 0.5 =>
                {
                    let rect = egui::Rect::from_two_pos(points[0], points[1])
                        .expand2(egui::vec2(0.0, 0.5));
                    if area.contains(rect.center()) {
                        found.push(rect);
                    }
                }
                _ => {}
            }
        }
        let mut found = Vec::new();
        for shape in &self.harness.output().shapes {
            visit(&shape.shape, area, &mut found);
        }
        found
    }

    /// A tone bar beside a linked message, rather than a hand-built warning row.
    pub fn has_message_bar(&self, first: egui::Rect, color: egui::Color32) -> bool {
        fn contains(shape: &egui::Shape, first: egui::Rect, color: egui::Color32) -> bool {
            match shape {
                egui::Shape::Vec(shapes) => {
                    shapes.iter().any(|shape| contains(shape, first, color))
                }
                egui::Shape::Rect(bar) => {
                    bar.fill == color
                        && bar.rect.width() <= crate::design::XS
                        && bar.rect.right() < first.left()
                        && first.left() - bar.rect.right() <= crate::design::XXL
                        && (bar.rect.top() - first.top()).abs() <= crate::design::S
                        && bar.rect.height() >= crate::design::Type::Body.line()
                }
                _ => false,
            }
        }
        self.harness
            .output()
            .shapes
            .iter()
            .any(|shape| contains(&shape.shape, first, color))
    }

    /// A message's tone bar beside one line of its painted words: the bar is as tall as the
    /// words, ends before them, and starts on their line.
    pub fn has_tone_bar(&self, words: egui::Rect, color: egui::Color32) -> bool {
        fn contains(shape: &egui::Shape, words: egui::Rect, color: egui::Color32) -> bool {
            match shape {
                egui::Shape::Vec(shapes) => {
                    shapes.iter().any(|shape| contains(shape, words, color))
                }
                egui::Shape::Rect(bar) => {
                    bar.fill == color
                        && bar.rect.width() <= crate::design::XS
                        && bar.rect.right() < words.left()
                        && words.left() - bar.rect.right() <= crate::design::XXL
                        && (bar.rect.top() - words.top()).abs() <= 1.0
                        && bar.rect.bottom() >= words.bottom() - 1.0
                }
                _ => false,
            }
        }
        self.harness
            .output()
            .shapes
            .iter()
            .any(|shape| contains(&shape.shape, words, color))
    }

    pub fn painted_right(&self, words: &str) -> Option<f32> {
        let mut right = None;
        self.each_painted_text(|text, clip| {
            let bounds = text.visual_bounding_rect();
            if text.galley.job.text.contains(words) && clip.intersects(bounds) {
                right = Some(bounds.right());
            }
        });
        right
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

    /// Colour of the first text section for this visible label.
    pub fn painted_color(&self, words: &str) -> Option<egui::Color32> {
        let mut found = None;
        self.each_painted_text(|text, clip| {
            if clip.intersects(text.visual_bounding_rect()) && text.galley.job.text == words {
                found = text
                    .galley
                    .job
                    .sections
                    .first()
                    .map(|section| section.format.color);
            }
        });
        found
    }

    /// Each text painted now, with the clip of the pane it is in.
    pub(super) fn each_painted_text(
        &self,
        mut visit: impl FnMut(&egui::epaint::TextShape, egui::Rect),
    ) {
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

    /// The painted field frame around the visible part of a text input.
    pub fn input_frame(&self, name: &str) -> Option<(egui::Rect, egui::Rect)> {
        let shown = self
            .nodes()
            .into_iter()
            .find(|node| node.name == name)?
            .shown;
        fn visit(
            shape: &egui::Shape,
            clip: egui::Rect,
            shown: egui::Rect,
            found: &mut Vec<(egui::Rect, egui::Rect)>,
        ) {
            match shape {
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        visit(shape, clip, shown, found);
                    }
                }
                egui::Shape::Rect(rect)
                    if rect.stroke.color == crate::design::colors().field_border
                        && rect.rect.contains_rect(shown) =>
                {
                    found.push((rect.rect, clip))
                }
                _ => {}
            }
        }
        let mut found = Vec::new();
        for shape in &self.harness.output().shapes {
            visit(&shape.shape, shape.clip_rect, shown, &mut found);
        }
        found
            .into_iter()
            .min_by(|a, b| a.0.area().total_cmp(&b.0.area()))
    }

    /// The painted frame of a named control, for fill and border checks.
    pub fn control_frame(&self, name: &str) -> Option<egui::epaint::RectShape> {
        self.control_frame_at(self.place_of(name)?)
    }

    /// Inspect this control's frame when several controls have the same name.
    pub fn control_frame_at(&self, bounds: egui::Rect) -> Option<egui::epaint::RectShape> {
        // AccessKit reports pixels; painted shapes use logical points.
        let bounds = bounds * (1.0 / self.harness.ctx.pixels_per_point());
        fn find(shape: &egui::Shape, bounds: egui::Rect) -> Option<egui::epaint::RectShape> {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().find_map(|shape| find(shape, bounds)),
                egui::Shape::Rect(rect)
                    if (rect.rect.min - bounds.min).length() < 1.0
                        && (rect.rect.max - bounds.max).length() < 1.0 =>
                {
                    Some(rect.clone())
                }
                _ => None,
            }
        }
        self.harness
            .output()
            .shapes
            .iter()
            .find_map(|shape| find(&shape.shape, bounds))
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
            self.timing.settle_frames += 1;
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
                let page = self.current_page();
                if self.page_trail.last().is_none_or(|(_, last)| *last != page) {
                    self.page_trail.push((self.interactions, page));
                }
                self.timing.settle += started.elapsed();
                self.timing.settles += 1;
                return;
            }
            check_settle_deadline(self.scenario, started, self.settle_timeout);
            if busy {
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    }

    pub fn wait_for_text(&mut self, text: &str) {
        let started = Instant::now();
        let timeout = if self.diagnose {
            DIAGNOSTIC_TEXT_TIMEOUT
        } else {
            ASYNC_TIMEOUT
        };
        while !self.visible_text(text) {
            assert!(
                started.elapsed() < timeout,
                "{}: waiting for painted {text:?} after {timeout:?}.\n{}",
                self.scenario,
                self.describe()
            );
            self.harness.step();
        }
    }

    /// The top-level page the app is showing now.
    pub fn current_page(&self) -> String {
        self.harness
            .state()
            .visible_page()
            .map_or_else(String::new, |page| page.name().to_owned())
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
    /// Targeted runs collect every reached mismatch before failing the suite.
    pub fn expect(&self, name: &str) {
        if self.has_containing(name) {
            return;
        }
        let message = format!(
            "{}: expected {name:?} on screen.\n{}",
            self.scenario,
            self.describe()
        );
        if self.diagnose || self.targeted {
            eprintln!("{message}");
            self.failed_expectations.borrow_mut().push(message);
        } else {
            panic!("{message}");
        }
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

    /// Press on a control by its accessible name, and carry it to a place. It stays held.
    pub fn drag_hold(&mut self, name: &str, target: egui::Rect) {
        let rect = self.aim(&format!("drag handle {name:?}"), |node| {
            node.name == name && Self::is_control(node)
        });
        let (from, to) = (rect.center(), target.center());
        self.film_aim(rect, &format!("Drag: {name}"));
        self.harness.hover_at(from);
        self.harness.step();
        self.harness.event(egui::Event::PointerButton {
            pos: from,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        });
        self.harness.step();
        for step in 1..=4 {
            self.harness
                .hover_at(from + (to - from) * (step as f32 / 4.0));
            self.harness.step();
        }
    }

    /// Let go of what `drag_hold` holds, over this place.
    pub fn drag_release(&mut self, target: egui::Rect) {
        self.harness.event(egui::Event::PointerButton {
            pos: target.center(),
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        });
        self.harness.step();
        self.harness.remove_cursor();
        self.interactions += 1;
        self.settle();
        self.film_result("Dropped");
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

    /// Add an event from the shared strip, opening More only when the strip needs it.
    pub fn click_add_event(&mut self, name: &str) {
        if !self.has(name) {
            self.click("More Add actions");
        }
        self.click(name);
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
        // On the name, where a person aims. A row's second line has links of its own.
        let name_at = egui::pos2(rect.left() + 80.0, rect.top() + 12.0);
        let target = egui::Rect::from_center_size(name_at, egui::Vec2::splat(4.0));
        self.click_rect(target, &format!("Click: {name}"));
    }

    /// Ctrl+click a run row, to add it to the targets of one amount.
    pub fn ctrl_click_row(&mut self, name: &str) {
        let prefix = format!("{name},");
        let rect = self.aim(&format!("the row of {name:?}"), |node| {
            node.name.starts_with(&prefix) && Self::is_control(node)
        });
        let pos = egui::pos2(rect.left() + 80.0, rect.top() + 12.0);
        let held = egui::Modifiers {
            ctrl: true,
            command: true,
            ..Default::default()
        };
        self.harness.hover_at(pos);
        self.harness.step();
        self.harness.input_mut().modifiers = held;
        for pressed in [true, false] {
            self.harness.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: held,
            });
        }
        self.harness.step();
        self.harness.input_mut().modifiers = egui::Modifiers::NONE;
        self.harness.remove_cursor();
        self.interactions += 1;
        self.settle();
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

    /// Visible laid-out lines inside an AccessKit control's pixel bounds.
    pub fn painted_lines_in(&self, bounds: egui::Rect) -> Vec<String> {
        let bounds = bounds * (1.0 / self.harness.ctx.pixels_per_point());
        let mut lines = Vec::new();
        self.each_painted_text(|text, clip| {
            for row in &text.galley.rows {
                let rect = row.rect().translate(text.pos.to_vec2());
                if bounds.contains(rect.center()) && clip.intersects(rect) {
                    lines.push(row.glyphs.iter().map(|glyph| glyph.chr).collect());
                }
            }
        });
        lines
    }

    /// The first source line must fit inside the editor's visible clip.
    pub fn first_source_line_visible(&self, prefix: &str) -> bool {
        let mut visible = false;
        self.each_painted_text(|text, clip| {
            if text.galley.job.text.starts_with(prefix)
                && let Some(row) = text.galley.rows.first()
            {
                let rect = row.rect().translate(text.pos.to_vec2());
                visible |= clip.expand(1.0).contains_rect(rect);
            }
        });
        visible
    }

    /// The heights of painted horizontal rules (hairlines and separators) that cross `x` between
    /// `top` and `bottom`. All values are AccessKit pixels, like `place_of`.
    pub fn horizontal_rules(&self, x: f32, top: f32, bottom: f32) -> Vec<f32> {
        let scale = self.harness.ctx.pixels_per_point();
        fn walk(shape: &egui::Shape, clip: egui::Rect, out: &mut Vec<(egui::Pos2, egui::Pos2)>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, clip, out)),
                egui::Shape::LineSegment { points, .. }
                    if (points[0].y - points[1].y).abs() < 0.01 && clip.contains(points[0]) =>
                {
                    out.push((points[0], points[1]));
                }
                _ => {}
            }
        }
        let mut lines = Vec::new();
        for clipped in &self.harness.output().shapes {
            walk(&clipped.shape, clipped.clip_rect, &mut lines);
        }
        let mut rules: Vec<f32> = lines
            .into_iter()
            .map(|(a, b)| (a.x.min(b.x) * scale, a.x.max(b.x) * scale, a.y * scale))
            .filter(|&(left, right, y)| left <= x && x <= right && top <= y && y <= bottom)
            .map(|(_, _, y)| y)
            .collect();
        rules.sort_by(f32::total_cmp);
        rules
    }

    pub fn painted_rect_in(&self, words: &str, area: egui::Rect) -> Option<egui::Rect> {
        let mut found = None;
        self.each_painted_text(|text, clip| {
            let rect = egui::Rect::from_min_size(text.pos, text.galley.size());
            if text.galley.job.text.contains(words)
                && area.contains(rect.center())
                && clip.intersects(rect)
            {
                found = Some(rect);
            }
        });
        found
    }

    /// Actual row bounds exclude a wrapping label's indentation before its first word.
    pub fn painted_word_rect(&self, words: &str, area: egui::Rect) -> Option<egui::Rect> {
        let mut found = None;
        self.each_painted_text(|text, clip| {
            if text.galley.job.text == words {
                for row in &text.galley.rows {
                    let rect = row
                        .rect_without_leading_space()
                        .translate(text.pos.to_vec2());
                    if area.contains(rect.center()) && clip.intersects(rect) {
                        found = Some(rect);
                    }
                }
            }
        });
        found
    }

    /// Where the reader's title is, which moves when the reader scrolls.
    pub fn scroll_position_of_reader(&self, title: &str) -> Option<f32> {
        self.nodes()
            .into_iter()
            .find(|node| node.role == Role::Label && node.name == title)
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
        let nodes = self.nodes();
        // The party bar stands above the Session page's title, so the title starts below it.
        let top = nodes
            .iter()
            .find(|node| node.name == "Hide the party")
            .map_or(0.0, |node| node.rect.max.y + crate::design::XS);
        nodes
            .into_iter()
            .filter(|node| {
                node.role == Role::Label
                    && node.rect.min.x > 140.0
                    && node.rect.min.y >= top
                    && node.rect.min.y < top + 40.0
            })
            .min_by(|a, b| a.rect.min.x.total_cmp(&b.rect.min.x))
            .map(|node| node.name)
    }

    /// What a run row really paints, which its accessible name need not match.
    pub fn row_words(&self, name: &str) -> String {
        self.row_node(name)
            .map_or_else(String::new, |row| self.painted_in(row.shown))
            .replace('\u{a0}', " ")
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
        let is_row = |node: &Found| node.name == text && !matches!(node.role, Role::TextRun);
        self.aim(&format!("row with exactly {text:?}"), is_row);
        // One name can head several rows, and only one of them has this control.
        let rows: Vec<_> = self
            .nodes()
            .into_iter()
            .filter(|node| is_row(node))
            .map(|node| node.rect)
            .collect();
        let rect = self.aim(&format!("{name:?} beside {text:?}"), |node| {
            node.name == name
                && Self::is_control(node)
                && rows.iter().any(|row| {
                    node.rect.center().y >= row.min.y - 16.0
                        && node.rect.center().y <= row.max.y + 16.0
                })
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

    /// Alt+key, for reader history.
    pub fn press_alt(&mut self, key: egui::Key) {
        let caption = format!("Press: Alt+{}", key.name());
        self.film_caption(&caption);
        self.harness.key_press_modifiers(egui::Modifiers::ALT, key);
        self.interactions += 1;
        self.settle();
        self.film_result(&caption);
    }

    /// Press a mouse side button over a visible node: Extra1 is Back and Extra2 is Forward.
    pub fn press_side_button(&mut self, button: egui::PointerButton, over: &str) {
        let rect = self.aim(&format!("{button:?} over {over:?}"), |node| {
            node.name == over
        });
        let caption = format!("Press: mouse {button:?} over {over}");
        self.film_aim(rect, &caption);
        let mut raw = egui::RawInput {
            events: vec![egui::Event::PointerButton {
                pos: rect.center(),
                button,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        // Kittest does not call eframe's raw input hook, which sets the side buttons aside.
        let ctx = self.harness.ctx.clone();
        eframe::App::raw_input_hook(self.harness.state_mut(), &ctx, &mut raw);
        self.harness.step();
        self.harness.remove_cursor();
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

    /// Rest the pointer on painted words with no control of their own, until any tooltip shows.
    /// The tooltip's text must be painted once more than before.
    pub fn painted_tooltip_shows(&mut self, words: &str, text: &str) -> bool {
        let mut at = None;
        self.each_painted_text(|shape, _| {
            if at.is_none() && shape.galley.job.text.contains(words) {
                at = Some(shape.visual_bounding_rect().center());
            }
        });
        let Some(at) = at else {
            return false;
        };
        let painted = |d: &Self| {
            d.painted_in(d.harness.ctx.content_rect())
                .matches(text)
                .count()
        };
        let before = painted(self);
        self.harness.hover_at(at);
        // A tooltip waits half a second, and one frame here is a quarter second.
        self.harness.run_steps(6);
        let shown = painted(self);
        self.harness.remove_cursor();
        self.settle();
        shown > before
    }

    /// Wheel-scroll down over this point. One notch is 100 points.
    pub fn scroll_at(&mut self, pos: egui::Pos2, notches: u32) {
        self.scroll_by(pos, notches, egui::vec2(0.0, -100.0));
    }

    /// Return to the start of a reader after editing its source.
    pub fn scroll_up_at(&mut self, pos: egui::Pos2, notches: u32) {
        self.scroll_by(pos, notches, egui::vec2(0.0, 100.0));
    }

    /// Scroll across a table to read columns outside the pane.
    pub fn scroll_sideways_at(&mut self, pos: egui::Pos2, notches: u32) {
        self.scroll_by(pos, notches, egui::vec2(-100.0, 0.0));
    }

    fn scroll_by(&mut self, pos: egui::Pos2, notches: u32, delta: egui::Vec2) {
        self.film_aim(
            egui::Rect::from_center_size(pos, egui::Vec2::ZERO),
            "Scroll",
        );
        self.harness.hover_at(pos);
        self.harness.step();
        for _ in 0..notches {
            self.harness.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta,
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

    /// Steps that prove a result and come back to the same place. The task being counted
    /// keeps its own cost: these steps are not part of it. Only proof steps that end in the
    /// state they started from go inside; no step of the DM's task may, or its cost hides.
    pub fn aside<T>(&mut self, steps: impl FnOnce(&mut Self) -> T) -> T {
        let before = self.interactions;
        let result = steps(self);
        self.last_goal += self.interactions - before;
        result
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

    /// A new audit workflow has an explicit cost target, without adding a baseline allowance.
    pub fn audit_goal_within(&mut self, id: &str, goal: &str, target: u32) {
        let cost = self.interactions - self.last_goal;
        self.last_goal = self.interactions;
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

    /// Obsidian edits happen outside the app; exercise the real focus-return reload.
    pub fn focus_window(&mut self, focused: bool) {
        self.harness.input_mut().focused = focused;
        self.harness
            .input_mut()
            .events
            .push(egui::Event::WindowFocused(focused));
        self.settle();
    }

    /// Test-only access to app state that no UI route can create.
    pub fn app_mut(&mut self) -> &mut Splat {
        self.held = false;
        self.harness.state_mut()
    }

    /// The actual source editor's focus and scroll, for a layout transition check.
    pub fn note_editor_view(&self) -> (bool, f32) {
        let (id, offset) = self.harness.ctx.data(|data| {
            data.get_temp::<(egui::Id, egui::Vec2)>(egui::Id::new("session_editor_proof"))
                .expect("The note editor was painted")
        });
        (
            self.harness.ctx.memory(|memory| memory.has_focus(id)),
            offset.y,
        )
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
        let image = self.render();
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
        let image = self.render();
        let end = target.center();
        let start = self.pointer.unwrap_or(end);
        if let Some(video) = &mut self.video {
            video.aim(&image, start, target, caption);
        }
        self.pointer = Some(end);
    }

    /// The app's last pass as an image. A pass that is already rendered is not rendered again.
    fn render(&mut self) -> std::rc::Rc<image::RgbaImage> {
        let key = (
            self.harness_generation,
            self.harness.ctx.cumulative_pass_nr(),
        );
        if let Some((rendered, image)) = &self.last_render
            && *rendered == key
        {
            return image.clone();
        }
        let started = Instant::now();
        let image = std::rc::Rc::new(
            self.harness
                .render()
                .unwrap_or_else(|error| panic!("{}: render failed: {error}", self.scenario)),
        );
        self.timing.render += started.elapsed();
        self.timing.renders += 1;
        self.last_render = Some((key, image.clone()));
        image
    }

    fn film_caption(&mut self, caption: &str) {
        self.film_frames(Overlay::caption(caption), super::video::AIM_FRAMES);
    }

    pub fn column_edge(&self, name: &str) -> f32 {
        self.harness
            .ctx
            .read_response(egui::Id::new(name).with("__resize"))
            .expect("column resize handle")
            .rect
            .center()
            .x
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
        self.state_prepared(name, sizes, |_| {});
    }

    /// Check a return against a named earlier state, at every size it captured.
    /// Rendering and ergonomics still run; no duplicate PNG is written.
    pub fn assert_state_matches(&mut self, name: &str, earlier: &str) {
        self.state_matching(name, earlier, None, |_| {});
    }

    /// Save only the sizes where this step changes the view. Compare the others.
    pub fn state_changed_at(&mut self, name: &str, earlier: &str, changed: Sizes) {
        self.state_matching(name, earlier, Some(changed), |_| {});
    }

    /// Retain per-size preparation and observations when comparing shared views.
    pub fn state_changed_at_prepared(
        &mut self,
        name: &str,
        earlier: &str,
        changed: Sizes,
        prepare: impl FnMut(&mut Self),
    ) {
        self.state_matching(name, earlier, Some(changed), prepare);
    }

    fn state_matching(
        &mut self,
        name: &str,
        earlier: &str,
        changed: Option<Sizes>,
        prepare: impl FnMut(&mut Self),
    ) {
        let (label, sizes) = self
            .named_states
            .get(earlier)
            .cloned()
            .unwrap_or_else(|| panic!("{}: no earlier state named {earlier:?}", self.scenario));
        let changed = changed.map_or(&[][..], Sizes::plan);
        assert!(changed.iter().all(|size| sizes.plan().contains(size)));
        self.state_prepared_inner(
            name,
            sizes,
            Capture::Compare {
                earlier: &label,
                changed,
            },
            prepare,
        );
    }

    /// Retain all size checks when another scenario already supplies the image.
    pub fn check_state(&mut self, name: &str, sizes: Sizes) {
        self.state_prepared_inner(name, sizes, Capture::ChecksOnly, |_| {});
    }

    /// Check every size, saving only views not already supplied by another scenario.
    pub fn state_captured_at(&mut self, name: &str, sizes: Sizes, captures: Sizes) {
        assert!(
            captures
                .plan()
                .iter()
                .all(|size| sizes.plan().contains(size))
        );
        self.state_prepared_inner(name, sizes, Capture::SaveAt(captures.plan()), |_| {});
    }

    /// The next capture intentionally changes very little. Assert its effect first.
    pub fn expect_same(&mut self, reason: &'static str) {
        assert!(!reason.trim().is_empty(), "expect_same needs a reason");
        self.expect_same = Some(reason);
    }

    /// Restore transient UI after each resize, then verify the named state before capture.
    /// Repeating setup for another capture size is not another action in the DM's task.
    pub fn state_prepared(&mut self, name: &str, sizes: Sizes, prepare: impl FnMut(&mut Self)) {
        self.state_prepared_inner(name, sizes, Capture::Save, prepare);
    }

    fn state_prepared_inner(
        &mut self,
        name: &str,
        sizes: Sizes,
        capture: Capture<'_>,
        mut prepare: impl FnMut(&mut Self),
    ) {
        let task_interactions = self.interactions;
        self.finish_capture_saves();
        let expect_same = self.expect_same.take();
        self.states += 1;
        let label = format!("{:02}-{name}", self.states);
        self.named_states
            .insert(name.into(), (label.clone(), sizes));
        for &(size, density) in sizes.plan() {
            self.resize(size, density);
            prepare(self);
            self.finish_capture_saves();
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
            self.capture(&label, size, density, expect_same, capture);
        }
        self.resize(self.base, 1.0);
        prepare(self);
        self.settle();
        self.interactions = task_interactions;
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
        let horizontal_regions: Vec<_> = ergonomics::horizontal_scroll_regions(&self.harness)
            .into_iter()
            .map(|rect| rect * scale)
            .collect();
        self.outcome.violations.extend(ergonomics::inspect(
            &controls,
            &horizontal_regions,
            viewport,
            self.scenario,
            state,
            &window,
        ));
        self.outcome.violations.extend(ergonomics::missing_glyphs(
            &self.harness.ctx,
            &self.harness.output().shapes,
            self.scenario,
            state,
            &window,
        ));
        self.outcome.violations.extend(ergonomics::cut_text(
            &self.harness.output().shapes,
            &horizontal_regions,
            &ergonomics::lookup_results(&self.harness)
                .into_iter()
                .map(|(full, rect)| (full, rect * scale))
                .collect::<Vec<_>>(),
            self.scenario,
            state,
            &window,
        ));
    }

    fn capture(
        &mut self,
        state: &str,
        size: (f32, f32),
        density: f32,
        expect_same: Option<&str>,
        capture: Capture<'_>,
    ) {
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
        let image = self.render();
        let window = format!("{}x{}@{density}", size.0, size.1);
        let earlier = match capture {
            Capture::Compare { earlier, changed } if !changed.contains(&(size, density)) => {
                Some(earlier)
            }
            _ => None,
        };
        let failure = if let Some(earlier) = earlier {
            let failure =
                self.repeated_captures
                    .check_named(&window, state, image.as_raw(), earlier);
            if failure.is_none() {
                self.outcome.passes.push(ergonomics::Pass {
                    check: "frame-match".into(),
                    scenario: self.scenario.into(),
                    state: state.into(),
                    detail: format!("{window}: matches {earlier} within the comparison tolerance"),
                });
            }
            failure
        } else {
            self.repeated_captures
                .check(self.states, &window, state, image.as_raw(), expect_same)
        };
        let matched = earlier.is_some() && failure.is_none();
        if let Some(failure) = failure {
            self.failed_expectations
                .borrow_mut()
                .push(format!("{}: {failure}", self.scenario));
        }
        if let Some(reason) = expect_same {
            self.outcome.passes.push(ergonomics::Pass {
                check: "expect_same".into(),
                scenario: self.scenario.into(),
                state: state.into(),
                detail: format!("{window}: {reason}"),
            });
        }
        // Shared views still enter the repetition history. Save no second PNG unless
        // a named comparison fails and needs a diagnostic image.
        if matched
            || matches!(capture, Capture::ChecksOnly)
            || matches!(capture, Capture::SaveAt(plan) if !plan.contains(&(size, density)))
        {
            return;
        }
        let started = Instant::now();
        save_png(&image, &path)
            .unwrap_or_else(|error| panic!("Could not write {}: {error}", path.display()));
        self.timing.png += started.elapsed();
        self.timing.saves += 1;
        self.outcome.screenshots.push(file);
    }

    /// Capture the current state with the pointer resting on a control.
    pub fn state_hovering(&mut self, name: &str, target: &str) {
        self.capture_hovering(name, target, 2);
    }

    /// Capture the current state with the pointer resting on a control until its tooltip shows.
    pub fn state_with_tooltip(&mut self, name: &str, target: &str) {
        // A tooltip waits half a second, and one frame here is a quarter second.
        self.capture_hovering(name, target, 6);
    }

    fn capture_hovering(&mut self, name: &str, target: &str, steps: usize) {
        self.finish_capture_saves();
        self.states += 1;
        let label = format!("{:02}-{name}", self.states);
        let rect = self.aim(&format!("hover target {target:?}"), |node| {
            node.name == target && !matches!(node.role, Role::Label | Role::TextRun)
        });
        self.harness.hover_at(rect.center());
        self.harness.run_steps(steps);
        let expect_same = self.expect_same.take();
        self.capture(&label, self.base, 1.0, expect_same, Capture::Save);
        self.harness.remove_cursor();
        self.settle();
    }

    fn finish_capture_saves(&mut self) {
        self.app_mut().sessions.flush();
        self.app_mut().timeline.save();
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
            .filter(|control| {
                !control.disabled
                    && (!control.chrome || control.role == Role::Splitter)
                    && !control.is_label()
            })
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
        self.outcome.failed_expectations = self.failed_expectations.into_inner();
        if std::env::var_os("SPLAT_VISUAL_TIMINGS").is_some() {
            println!(
                "Timing {}: settle = {:?}, render = {:?}, PNG encode/write = {:?}",
                self.scenario, self.timing.settle, self.timing.render, self.timing.png
            );
            println!(
                "Counts {}: settles = {}, settle frames = {}, renders = {}, PNGs = {}",
                self.scenario,
                self.timing.settles,
                self.timing.settle_frames,
                self.timing.renders,
                self.timing.saves
            );
        }
        self.outcome
    }
}

#[test]
fn diagnostic_and_targeted_text_failures_remain_failures_and_full_mode_still_panics() {
    let root = tempfile::tempdir().unwrap();
    let world = super::fixture::World::build(
        root.path(),
        root.path(),
        "text-check",
        super::fixture::Seed::Empty,
    )
    .unwrap();
    let mut driver = Driver::new(&world, "text-check", root.path(), None, false);
    driver.diagnose = true;
    driver.expect("missing diagnostic phrase one");
    driver.expect("missing diagnostic phrase two");
    assert_eq!(driver.failed_expectations.borrow().len(), 2);
    driver.diagnose = false;
    driver.targeted = true;
    driver.expect("missing targeted phrase one");
    driver.expect("missing targeted phrase two");
    assert_eq!(driver.failed_expectations.borrow().len(), 4);
    driver.targeted = false;
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(
            || driver.expect("missing normal phrase")
        ))
        .is_err()
    );
    let outcome = driver.finish();
    assert_eq!(outcome.failed_expectations.len(), 4);
    assert!(outcome.failed_expectations[0].contains("phrase one"));
    assert!(outcome.failed_expectations[1].contains("phrase two"));
    assert!(outcome.failed_expectations[2].contains("targeted phrase one"));
    assert!(outcome.failed_expectations[3].contains("targeted phrase two"));
}

#[test]
fn diagnostic_painted_text_wait_still_fails_for_absent_text() {
    let root = tempfile::tempdir().unwrap();
    let world = World::build(
        root.path(),
        root.path(),
        "painted-text-check",
        super::fixture::Seed::Empty,
    )
    .unwrap();
    let mut driver = Driver::new(&world, "painted-text-check", root.path(), None, false);
    driver.diagnose = true;
    let started = Instant::now();
    let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        driver.wait_for_text("this phrase is deliberately absent");
    }))
    .expect_err("a missing painted phrase must still fail");
    let message = failure.downcast_ref::<String>().unwrap();
    assert!(message.contains("this phrase is deliberately absent"));
    assert!(started.elapsed() < Duration::from_secs(15));
}

/// Write a capture with one filter for every row. The default tries five on each row: it
/// took four times as long, for files 4% smaller. Sub and Up made files 40% larger.
fn save_png(image: &image::RgbaImage, path: &Path) -> image::ImageResult<()> {
    use image::codecs::png::{CompressionType, PngEncoder};
    let file = std::io::BufWriter::new(std::fs::File::create(path)?);
    image.write_with_encoder(PngEncoder::new_with_quality(
        file,
        CompressionType::Fast,
        FILTER,
    ))
}

const FILTER: image::codecs::png::FilterType = image::codecs::png::FilterType::Paeth;
