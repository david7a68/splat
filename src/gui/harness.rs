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
use crate::visual_tests::fixture::{Seed, World};
use gpui::{
    AnyWindowHandle, AppContext, Bounds, Context, Entity, HeadlessAppContext, Keystroke, Modifiers,
    MouseButton, MouseDownEvent, MouseUpEvent, Pixels, PlatformInput, Size, Window, px, size,
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

/// The character cache of this test process: one folder for every shell, as the egui suite has
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
        // One device pixel per pixel, as the egui references (`egui-reference/*__1200x800.png`).
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
    /// what a frame's logic changed is drawn too.
    pub(crate) fn settle(&mut self) {
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

    /// Press (down and up) the middle of the target `name`, then settle.
    pub(crate) fn click(&mut self, name: &str) {
        let bounds = self.bounds(name).unwrap_or_else(|| {
            panic!(
                "No target named {name:?}. Targets: {:?}",
                kit::probe::names()
            )
        });
        self.click_at(bounds.center());
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

    /// Press keys, as GPUI writes them: `"ctrl-k"`, `"escape"`, `"tab shift-tab"`.
    pub(crate) fn press(&mut self, keystrokes: &str) {
        for text in keystrokes.split_whitespace() {
            let os = self.cx.update(|cx| cx.platform_info().os);
            let keystroke = Keystroke::parse_with_os(text, os)
                .unwrap_or_else(|_| panic!("{text:?} is not a keystroke"));
            self.cx
                .update_window(self.window, |_, window, cx| {
                    window.dispatch_keystroke(keystroke, cx);
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

    /// Render the window and write it to `artifacts/screenshots/gpui/<name>.png`.
    pub(crate) fn capture(&mut self, name: &str) -> PathBuf {
        self.settle();
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

    /// The window's size now.
    pub(crate) fn size(&mut self) -> Size<Pixels> {
        self.cx
            .update_window(self.window, |_, window, _| window.viewport_size())
            .expect("the window")
    }
}
