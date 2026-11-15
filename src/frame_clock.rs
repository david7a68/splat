//! The app's one clock for words that end by themselves (round 30 B7d): `Recording saved` on the
//! rail, `Hearing sound`, and the Add results of the plan and the Timeline with their Undo. It is
//! egui's frame time (`egui::InputState::time`), read once at the start of each frame (`tick`).
//! In the app it moves with real seconds, so every timing stays as it was. Under the test
//! harness each frame moves it by the harness's fixed step, so the frames a test runs, not the
//! machine's speed, decide when such a word ends, and two runs of one scenario show the same
//! words. A timer that reads `Instant` instead shows different words in different captures.
use eframe::egui;
use std::{cell::Cell, time::Duration};

thread_local! {
    static NOW: Cell<f64> = const { Cell::new(0.0) };
}

/// Read the frame clock: at the start of each frame, before anything reads `now`.
pub fn tick(ctx: &egui::Context) {
    let time = ctx.input(|input| input.time);
    NOW.with(|now| now.set(time));
}

/// The frame clock, in seconds, as the frame's `tick` read it.
pub fn now() -> f64 {
    NOW.with(Cell::get)
}

/// Has `duration` passed on the frame clock since `at` (a value of `now`)?
pub fn passed(at: f64, duration: Duration) -> bool {
    now() - at >= duration.as_secs_f64()
}

/// Set the frame clock, for a unit test that steps it past a timer's end.
#[cfg(test)]
pub fn set(seconds: f64) {
    NOW.with(|now| now.set(seconds));
}
