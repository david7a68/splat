//! The GPUI test harness: the whole app in a headless window with Splat's own fonts, real text
//! shaping and the Vulkan renderer, built over a fixture's state, with helpers to press, type and
//! capture.
//!
//! ```ignore
//! let mut shell = Shell::new("library-browse", Seed::Campaign);   // 1200×800
//! shell.click("Library");                                          // a kit target's words
//! shell.capture("library-browse__01-list");                        // artifacts/screenshots/gpui/
//! assert_eq!(shell.read(|root| root.app.page), Page::Library);
//! ```
//!
//! Targets are found by the names the kit gives them (`kit::target`): a button's, nav item's or
//! row's words, a field's name, and `id:<element id>` for any kit part with an id. A page that
//! draws its own pressable element marks it with `kit::target(element, name)`.
use super::{SplatRoot, kit};
use crate::Splat;
use crate::fixtures::{Seed, World};
use gpui::{
    AnyWindowHandle, AppContext, Bounds, Context, Entity, HeadlessAppContext, Keystroke, Modifiers,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, PlatformInput, Point,
    ScrollDelta, ScrollEvent, Size, Window, point, px, size,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

/// The app in a headless window.
pub(crate) struct Shell {
    cx: HeadlessAppContext,
    window: AnyWindowHandle,
    root: Entity<SplatRoot>,
    _home: tempfile::TempDir,
}

/// Where captures go: `artifacts/screenshots/gpui/` under the checkout (ignored by git).
pub(crate) fn capture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("artifacts/screenshots/gpui")
}

/// The character cache of this test process: one folder for every shell
/// (`hermetic::closed::use_cache_root` takes one root per process). Each campaign's characters
/// are kept apart inside it by the campaign's id.
pub(crate) fn character_cache() -> PathBuf {
    static ROOT: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
    ROOT.get_or_init(|| {
        let root = std::env::temp_dir().join(format!("splat-gpui-cache-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("the character cache");
        crate::hermetic::closed::use_cache_root(root.clone());
        root
    })
    .clone()
}

/// A scenario's first id, from its name, so ids and dice repeat from run to run.
fn first_uuid(name: &str) -> u128 {
    // FNV-1a, 128 bits.
    let mut hash: u128 = 0x6c62272e07bb014262b821756295c58d;
    for byte in name.bytes() {
        hash ^= u128::from(byte);
        hash = hash.wrapping_mul(0x0000000001000000000000000000013B);
    }
    hash.max(1)
}

impl Shell {
    /// The app at 1200×800 over `seed`'s fixture.
    pub(crate) fn new(name: &str, seed: Seed) -> Self {
        Self::with_size(name, seed, size(px(1200.0), px(800.0)))
    }

    /// The app at `window` size over `seed`'s fixture.
    pub(crate) fn with_size(name: &str, seed: Seed, window: Size<Pixels>) -> Self {
        crate::hermetic::closed::repeatable_uuids(first_uuid(name));
        crate::hermetic::closed::clear_next_folder();
        let home = tempfile::tempdir().expect("a test folder");
        let world = World::build(&home.path().join(name), &character_cache(), name, seed)
            .unwrap_or_else(|error| panic!("{name}: could not build its world: {error}"));
        Self::over(world.app(), home, window)
    }

    /// The app over a state the test built itself.
    pub(crate) fn over(app: Splat, home: tempfile::TempDir, window: Size<Pixels>) -> Self {
        let text_system = Arc::new(gpui_wgpu::CosmicTextSystem::new_without_system_fonts(
            super::theme::family::UI,
        ));
        let mut cx = HeadlessAppContext::with_platform(text_system, Arc::new(()), || {
            gpui_platform::current_headless_renderer()
        });
        cx.update(super::init);
        let mut root = None;
        let handle = cx
            .open_window(window, |window, cx| {
                let view = cx.new(|cx| SplatRoot::new(app, window, cx));
                root = Some(view.clone());
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            })
            .expect("a headless window");
        // One device pixel per pixel, as the reference captures (`*__1200x800.png`).
        cx.simulate_window_scale_factor_change(handle.into(), 1.0);
        let mut shell = Self {
            cx,
            window: handle.into(),
            root: root.expect("the root view"),
            _home: home,
        };
        shell.settle();
        shell
    }

    /// Run frames until nothing is left to do: pending tasks, then a draw, a few times over so
    /// what a frame's logic changed is drawn too. A read of the party's sheets in flight is
    /// waited for on its own result first, as the app shows it within a frame or two: the party
    /// bar, the run's Sheet tab and the synced roster rows read those sheets.
    pub(crate) fn settle(&mut self) {
        self.draw_frames();
        for _ in 0..10 {
            if !self.read(|root| root.app.party.reader.loading()) {
                return;
            }
            let root = self.root.clone();
            self.cx
                .update_window(self.window, |_, _, cx| {
                    root.update(cx, |root, _| root.app.party.reader.wait_for_load());
                })
                .expect("the window");
            self.draw_frames();
        }
    }

    fn draw_frames(&mut self) {
        for _ in 0..4 {
            self.cx.run_until_parked();
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.refresh();
                    window.draw(cx).clear(cx);
                })
                .expect("the window");
        }
        self.cx.run_until_parked();
    }

    /// Let `time` pass (timers, the background poll), then settle.
    pub(crate) fn wait(&mut self, time: Duration) {
        self.cx.advance_clock(time);
        self.settle();
    }

    /// Another window takes the focus, and the app's window gets it back: the DM came back from
    /// Obsidian or a file manager (egui's `focus_window(false)`, then `focus_window(true)`).
    pub(crate) fn focus_window_again(&mut self) {
        struct Away;
        impl gpui::Render for Away {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl gpui::IntoElement {
                gpui::Empty
            }
        }
        let away = self
            .cx
            .open_window(size(px(10.0), px(10.0)), |_, cx| cx.new(|_| Away))
            .expect("another window");
        let activate = |cx: &mut HeadlessAppContext, window: AnyWindowHandle| {
            cx.update_window(window, |_, window, _| window.activate_window())
                .expect("the window");
            cx.run_until_parked();
        };
        activate(&mut self.cx, away.into());
        activate(&mut self.cx, self.window);
        self.cx
            .update_window(away.into(), |_, window, _| window.remove_window())
            .expect("the other window");
        self.settle();
    }

    /// Read the root's state.
    pub(crate) fn read<R>(&mut self, read: impl FnOnce(&SplatRoot) -> R) -> R {
        self.cx.read_entity(&self.root, |root, _| read(root))
    }

    /// Change the root's state (a test's setup), then settle.
    pub(crate) fn update<R>(
        &mut self,
        update: impl FnOnce(&mut SplatRoot, &mut Window, &mut Context<SplatRoot>) -> R,
    ) -> R {
        let root = self.root.clone();
        let result = self
            .cx
            .update_window(self.window, |_, window, cx| {
                root.update(cx, |root, cx| update(root, window, cx))
            })
            .expect("the window");
        self.settle();
        result
    }

    /// Where the target `name` was last laid out (see the module's doc for names).
    pub(crate) fn bounds(&mut self, name: &str) -> Option<Bounds<Pixels>> {
        kit::probe::find(name)
    }

    /// Press (down and up) the middle of the target `name`, then settle. A target whose middle
    /// its scroll view hides is first scrolled into view with the wheel, as the DM would; one
    /// that no scroll brings into view fails the test, so no test presses what the DM cannot see.
    pub(crate) fn click(&mut self, name: &str) {
        let at = self.into_view(name);
        self.click_at(at);
    }

    /// The middle of the target `name`, scrolled into its view first (see `click`).
    pub(crate) fn into_view(&mut self, name: &str) -> Point<Pixels> {
        let find = |name: &str| {
            kit::probe::find_in_view(name).unwrap_or_else(|| {
                panic!(
                    "No target named {name:?}. Targets: {:?}",
                    kit::probe::names()
                )
            })
        };
        let window = self.size();
        let (mut bounds, mut view) = find(name);
        'scrolls: for _ in 0..40 {
            let middle = bounds.center();
            if view.contains(&middle) {
                return middle;
            }
            // A touchpad's scroll by the distance to the target, with the pointer where the DM
            // would put it: in the seen part nearest to the target, one pixel inside. A target
            // clipped away whole has no seen part to point at: then the places up its column,
            // inside the window, are tried until one scrolls it.
            let near = |at: Pixels, from: Pixels, to: Pixels| {
                if at >= to {
                    to - px(1.0)
                } else if at <= from {
                    from + px(1.0)
                } else {
                    at
                }
            };
            let seen = view.size.width > px(1.0)
                && view.size.height > px(1.0)
                && view.intersects(&Bounds::new(point(px(0.0), px(0.0)), window));
            let places: Vec<Point<Pixels>> = if seen {
                vec![point(
                    near(middle.x, view.left(), view.right()),
                    near(middle.y, view.top(), view.bottom()),
                )]
            } else {
                let x = near(middle.x, px(0.0), window.width);
                let from = near(middle.y, px(0.0), window.height);
                (0..)
                    .map(|step| from - px(24.0 * step as f32))
                    .take_while(|y| *y > px(0.0))
                    .map(|y| point(x, y))
                    .collect()
            };
            for at in places {
                let by = point(at.x - middle.x, at.y - middle.y);
                self.scroll(at, ScrollDelta::Pixels(by), Modifiers::default());
                let (moved, now) = find(name);
                if moved != bounds {
                    (bounds, view) = (moved, now);
                    continue 'scrolls;
                }
            }
            break;
        }
        panic!(
            "The target {name:?} at {bounds:?} is outside the part of the window it can be seen \
             in ({view:?}), and scrolling does not bring it in: the DM could not press it"
        )
    }

    /// Press the target `name` that closes the window (the close prompt's `Save and close` or
    /// `Discard and close`): true when the window is gone after it. No settle follows, as there
    /// is no window left to draw.
    pub(crate) fn click_to_close(&mut self, name: &str) -> bool {
        let at = self.into_view(name);
        for input in [
            PlatformInput::MouseDown(MouseDownEvent {
                button: MouseButton::Left,
                position: at,
                modifiers: Modifiers::default(),
                click_count: 1,
                first_mouse: false,
            }),
            PlatformInput::MouseUp(MouseUpEvent {
                button: MouseButton::Left,
                position: at,
                modifiers: Modifiers::default(),
                click_count: 1,
            }),
        ] {
            let _ = self.cx.update_window(self.window, |_, window, cx| {
                window.dispatch_event(input, cx);
            });
            self.cx.run_until_parked();
        }
        let id = self.window.window_id();
        !self
            .cx
            .update(|cx| cx.windows().iter().any(|window| window.window_id() == id))
    }

    /// The test's own folder: the campaigns, the Library's packages and the settings live in it.
    pub(crate) fn home(&self) -> &std::path::Path {
        self._home.path()
    }

    /// Press at a place in the window, then settle.
    pub(crate) fn click_at(&mut self, position: gpui::Point<Pixels>) {
        for input in [
            PlatformInput::MouseDown(MouseDownEvent {
                button: MouseButton::Left,
                position,
                modifiers: Modifiers::default(),
                click_count: 1,
                first_mouse: false,
            }),
            PlatformInput::MouseUp(MouseUpEvent {
                button: MouseButton::Left,
                position,
                modifiers: Modifiers::default(),
                click_count: 1,
            }),
        ] {
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.dispatch_event(input, cx);
                })
                .expect("the window");
            self.cx.run_until_parked();
        }
        self.settle();
    }

    /// Press at `from`, move to `to` in a few steps with the button held, and let go (a splitter's
    /// drag), then settle.
    pub(crate) fn drag(&mut self, from: Point<Pixels>, to: Point<Pixels>) {
        let steps = 4;
        let mut inputs = vec![PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position: from,
            modifiers: Modifiers::default(),
            click_count: 1,
            first_mouse: false,
        })];
        for step in 1..=steps {
            let along = step as f32 / steps as f32;
            inputs.push(PlatformInput::MouseMove(MouseMoveEvent {
                position: point(
                    from.x + (to.x - from.x) * along,
                    from.y + (to.y - from.y) * along,
                ),
                pressed_button: Some(MouseButton::Left),
                modifiers: Modifiers::default(),
            }));
        }
        inputs.push(PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position: to,
            modifiers: Modifiers::default(),
            click_count: 1,
        }));
        for input in inputs {
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.dispatch_event(input, cx);
                })
                .expect("the window");
            self.settle();
        }
    }

    /// Press at `from` and move to `to` with the button held, without letting go (a drag in
    /// progress), then settle. `release` ends it.
    pub(crate) fn hold_and_move(&mut self, from: Point<Pixels>, to: Point<Pixels>) {
        let mut inputs = vec![PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position: from,
            modifiers: Modifiers::default(),
            click_count: 1,
            first_mouse: false,
        })];
        for step in 1..=4 {
            let along = step as f32 / 4.0;
            inputs.push(PlatformInput::MouseMove(MouseMoveEvent {
                position: point(
                    from.x + (to.x - from.x) * along,
                    from.y + (to.y - from.y) * along,
                ),
                pressed_button: Some(MouseButton::Left),
                modifiers: Modifiers::default(),
            }));
        }
        for input in inputs {
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.dispatch_event(input, cx);
                })
                .expect("the window");
            self.settle();
        }
    }

    /// Let go of the left button at `at` (the end of `hold_and_move`), then settle.
    pub(crate) fn release(&mut self, at: Point<Pixels>) {
        let input = PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position: at,
            modifiers: Modifiers::default(),
            click_count: 1,
        });
        self.cx
            .update_window(self.window, |_, window, cx| {
                window.dispatch_event(input, cx);
            })
            .expect("the window");
        self.settle();
    }

    /// Drag the target `from` to the middle of the target `to` with the left button (a row or a
    /// group dropped on a list), then settle.
    pub(crate) fn drag_target(&mut self, from: &str, to: &str) {
        let start = self
            .bounds(from)
            .unwrap_or_else(|| panic!("No target named {from:?}"))
            .center();
        let end = self
            .bounds(to)
            .unwrap_or_else(|| panic!("No target named {to:?}"))
            .center();
        self.drag(start, end);
    }

    /// Scroll at `at` by `delta`, as a touchpad (`precise`, in pixels) or a mouse wheel (lines)
    /// reports it, with `modifiers` held, then settle.
    pub(crate) fn scroll(&mut self, at: Point<Pixels>, delta: ScrollDelta, modifiers: Modifiers) {
        let input = PlatformInput::Scroll(ScrollEvent {
            position: at,
            delta,
            modifiers,
            ..Default::default()
        });
        self.cx
            .update_window(self.window, |_, window, cx| {
                window.dispatch_event(input, cx);
            })
            .expect("the window");
        self.settle();
    }

    /// Press keys, as GPUI writes them: `"ctrl-k"`, `"escape"`, `"tab shift-tab"`. Each key goes
    /// down and then up, as a keyboard sends it: Enter or Space on a focused button presses it
    /// as the key comes up.
    pub(crate) fn press(&mut self, keystrokes: &str) {
        for text in keystrokes.split_whitespace() {
            let os = self.cx.update(|cx| cx.platform_info().os);
            let keystroke = Keystroke::parse_with_os(text, os)
                .unwrap_or_else(|_| panic!("{text:?} is not a keystroke"));
            let up = PlatformInput::KeyUp(gpui::KeyUpEvent {
                keystroke: keystroke.clone(),
            });
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.dispatch_keystroke(keystroke, cx);
                })
                .expect("the window");
            self.cx.run_until_parked();
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.dispatch_event(up, cx);
                })
                .expect("the window");
            self.settle();
        }
    }

    /// Type words into the focused field, one character at a time.
    pub(crate) fn type_text(&mut self, words: &str) {
        for character in words.chars() {
            let key = character.to_string();
            let keystroke = Keystroke {
                modifiers: Modifiers::default(),
                key: key.clone(),
                key_char: Some(key),
            };
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.dispatch_keystroke(keystroke, cx);
                })
                .expect("the window");
            self.cx.run_until_parked();
        }
        self.settle();
    }

    /// Put words into the focused field in one input, as a paste or an input method gives them:
    /// the field changes once, not once per character.
    pub(crate) fn insert_text(&mut self, words: &str) {
        let keystroke = Keystroke {
            modifiers: Modifiers::default(),
            key: words.to_owned(),
            key_char: Some(words.to_owned()),
        };
        self.cx
            .update_window(self.window, |_, window, cx| {
                window.dispatch_keystroke(keystroke, cx);
            })
            .expect("the window");
        self.settle();
    }

    /// Render the window and write it to `artifacts/screenshots/gpui/<name>.png`. The session
    /// note, the template and the Timeline are saved first, as egui's driver did before every
    /// capture (`finish_capture_saves`): test builds save only when told to, so the capture shows
    /// what the DM sees once the save timer has run, never a held "Saving…".
    pub(crate) fn capture(&mut self, name: &str) -> PathBuf {
        self.update(|root, _, cx| {
            root.app.sessions.flush();
            root.app.timeline.save();
            cx.notify();
        });
        let image = self
            .cx
            .capture_screenshot(self.window)
            .unwrap_or_else(|error| panic!("{name}: no screenshot: {error:#}"));
        let dir = capture_dir();
        std::fs::create_dir_all(&dir).expect("the capture folder");
        let path = dir.join(format!("{name}.png"));
        image
            .save(&path)
            .unwrap_or_else(|error| panic!("{name}: could not save {path:?}: {error}"));
        path
    }

    /// The mean time of `frames` whole frames with nothing changed (the root's render, layout,
    /// prepaint and paint): what a frame costs while the DM only looks.
    pub(crate) fn frame_cost(&mut self, frames: u32) -> Duration {
        self.settle();
        let started = std::time::Instant::now();
        for _ in 0..frames {
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.refresh();
                    window.draw(cx).clear(cx);
                })
                .expect("the window");
        }
        started.elapsed() / frames.max(1)
    }

    /// The window's size now.
    pub(crate) fn size(&mut self) -> Size<Pixels> {
        self.cx
            .update_window(self.window, |_, window, _| window.viewport_size())
            .expect("the window")
    }
}
