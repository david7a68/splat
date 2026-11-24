//! The egui bridge: a headless `egui::Context` that the app's state methods still take while the
//! port runs (`settings.poll(ctx)`, `lookup.poll(ctx, …)`, `encounters.refresh_content(ctx, …)`,
//! the cross-page requests that egui code leaves in `ctx.data`).
//!
//! It draws nothing. Each frame runs the frame logic inside one egui pass on it
//! (`Bridge::pass`), so `ctx.input` has the time, `ctx.data` works and egui's per-pass state
//! advances. Background work asks this context for a repaint when it has news; the bridge turns
//! that into its signal (`Signal::take_due`), which the root's timer task reads between frames.
//!
//! The cleanup wave moves these methods off egui and deletes the bridge. A page port that finds a
//! state method taking `&egui::Context` or `&mut egui::Ui` only for data calls it with
//! `root.bridge.ctx()`; one that draws needs its own GPUI view.
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// When background work next wants a frame.
#[derive(Clone, Default)]
pub struct Signal(Arc<Mutex<Option<Instant>>>);

impl Signal {
    /// Ask for a frame after `delay`.
    fn ask(&self, delay: Duration) {
        let at = Instant::now() + delay.min(Duration::from_secs(3600));
        let mut due = self.0.lock().unwrap_or_else(|poison| poison.into_inner());
        *due = Some(due.map_or(at, |old| old.min(at)));
    }

    /// Whether a frame is due now; a due frame is taken.
    pub fn take_due(&self) -> bool {
        let mut due = self.0.lock().unwrap_or_else(|poison| poison.into_inner());
        match *due {
            Some(at) if at <= Instant::now() => {
                *due = None;
                true
            }
            _ => false,
        }
    }

    /// Whether any frame is asked for (now or later).
    pub fn pending(&self) -> bool {
        self.0
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .is_some()
    }
}

/// The headless egui context and its signal.
pub struct Bridge {
    ctx: egui::Context,
    signal: Signal,
    started: Instant,
}

impl Default for Bridge {
    fn default() -> Self {
        Self::new()
    }
}

impl Bridge {
    pub fn new() -> Self {
        let ctx = egui::Context::default();
        // One pass per frame: the frame logic runs once, never again for a discarded pass.
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        crate::design::theme::install(&ctx);
        crate::art::install(&ctx);
        let signal = Signal::default();
        let asked = signal.clone();
        ctx.set_request_repaint_callback(move |info| asked.ask(info.delay));
        Self {
            ctx,
            signal,
            started: Instant::now(),
        }
    }

    /// The egui context, for a state method that takes one outside a pass.
    pub fn ctx(&self) -> &egui::Context {
        &self.ctx
    }

    /// The signal the root's timer task reads.
    pub fn signal(&self) -> Signal {
        self.signal.clone()
    }

    /// Run `logic` inside one egui pass, with the time since the app started.
    pub fn pass(&mut self, mut logic: impl FnMut(&egui::Context)) {
        let input = egui::RawInput {
            time: Some(self.started.elapsed().as_secs_f64()),
            ..Default::default()
        };
        let _ = self.ctx.run(input, |ctx| logic(ctx));
    }
}

/// An egui colour token as a GPUI colour, for a state method that returns one (a `Status`'s
/// colour). Pages use `theme::colors()` for their own colours.
pub fn color(color: egui::Color32) -> gpui::Hsla {
    gpui::Rgba {
        r: f32::from(color.r()) / 255.0,
        g: f32::from(color.g()) / 255.0,
        b: f32::from(color.b()) / 255.0,
        a: f32::from(color.a()) / 255.0,
    }
    .into()
}

/// An egui `design::Status` (from a state method) as the kit's. The icon glyphs are the same
/// Phosphor code points in both UIs.
pub fn status(status: &crate::design::Status<'_>) -> crate::gui::kit::Status {
    crate::gui::kit::Status {
        icon: status.icon.to_owned().into(),
        word: status.word.to_owned().into(),
        color: color(status.color),
        icon_color: None,
    }
}
