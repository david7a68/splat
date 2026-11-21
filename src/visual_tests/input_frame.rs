//! Checked batches of raw events for proofs about input in the same frame.
//!
//! `Harness::step` runs a frame PER queued helper event. Do not drain that queue here:
//! doing so would apply part of the action before the frame the proof observes. Instead,
//! verify the actual first input batch and the frame count, including implicit key repeats.
//! One input frame can contain several egui layout passes; keep those real semantics.
use crate::egui::{self, Event};
use egui_kittest::Harness;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Observed {
    frames: usize,
    events: Vec<Event>,
    modifiers: egui::Modifiers,
}

type Observation = Arc<Mutex<Option<Observed>>>;

/// The batch ran in one input frame, without changing egui's layout-pass limit.
#[derive(Debug)]
pub(crate) struct FrameReceipt {
    pub layout_passes: u64,
}

fn observer(ctx: &egui::Context) -> Observation {
    let id = egui::Id::new("splat.test.input_frame.observer");
    let (shared, install) = ctx.data_mut(|data| {
        if let Some(shared) = data.get_temp::<Observation>(id) {
            (shared, false)
        } else {
            let shared: Observation = Arc::new(Mutex::new(None));
            data.insert_temp(id, shared.clone());
            (shared, true)
        }
    });
    if install {
        let weak = Arc::downgrade(&shared);
        ctx.on_begin_pass(
            "splat checked input frame",
            Arc::new(move |ctx| {
                let Some(shared) = weak.upgrade() else { return };
                let mut slot = shared.lock().expect("input-frame observer lock");
                if let Some(seen) = slot.as_mut()
                    && ctx.current_pass_index() == 0
                {
                    seen.frames += 1;
                    if seen.frames == 1 {
                        // Read before a widget can consume an event; later layout passes
                        // have empty input and must not replace this first batch.
                        ctx.input(|input| {
                            seen.events = input.raw.events.clone();
                            seen.modifiers = input.raw.modifiers;
                        });
                    }
                }
            }),
        );
    }
    shared
}

/// Deliver exactly this batch in one frame. Queueing Harness actions beforehand is an error.
/// Do not run settling steps before asserting the resulting first displayed frame.
/// A new physical key press needs a release after its previous press; an intentional repeat
/// must say `repeat: true`. The check compares with egui's normalized input, not just the fixture.
#[track_caller]
pub(crate) fn one_pass<State>(
    harness: &mut Harness<'_, State>,
    events: impl IntoIterator<Item = Event>,
) -> FrameReceipt {
    assert!(
        harness.input().events.is_empty(),
        "one_pass: pending raw input"
    );
    let expected: Vec<_> = events.into_iter().collect();
    let modifiers = harness.input().modifiers;
    let ctx = harness.ctx.clone();
    let before_frame = ctx.cumulative_frame_nr();
    let before_pass = ctx.cumulative_pass_nr();
    let shared = observer(&ctx);
    {
        let mut slot = shared.lock().expect("input-frame observer lock");
        assert!(
            slot.is_none(),
            "one_pass: prior input frame did not complete"
        );
        *slot = Some(Observed::default());
    }
    harness.input_mut().events.extend(expected.clone());
    harness.step();
    let seen = shared
        .lock()
        .expect("input-frame observer lock")
        .take()
        .expect("input frame armed");
    assert_eq!(
        ctx.cumulative_frame_nr() - before_frame,
        1,
        "one_pass: queued Harness actions ran extra input frames"
    );
    assert_eq!(seen.frames, 1, "one_pass: expected one first input pass");
    assert_eq!(
        seen.events, expected,
        "one_pass: input batch changed (queued action or implicit key repeat)"
    );
    assert_eq!(
        seen.modifiers, modifiers,
        "one_pass: queued modifiers changed input"
    );
    FrameReceipt {
        layout_passes: ctx.cumulative_pass_nr() - before_pass,
    }
}

/// The next explicitly named input frame. Does not release held keys or settle the app.
#[track_caller]
pub(crate) fn next_pass<State>(
    harness: &mut Harness<'_, State>,
    events: impl IntoIterator<Item = Event>,
) -> FrameReceipt {
    one_pass(harness, events)
}

/// The painted text shapes of a harness: each galley's words and where they are drawn.
pub(crate) fn painted_texts(harness: &egui_kittest::Harness<'_>) -> Vec<(String, egui::Rect)> {
    fn walk(shape: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            egui::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for shape in &harness.output().shapes {
        walk(&shape.shape, &mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(pressed: bool) -> Event {
        Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn harness() -> Harness<'static, Vec<Vec<Event>>> {
        Harness::new_state(
            |ctx, seen| {
                seen.push(ctx.input(|input| input.events.clone()));
            },
            Vec::new(),
        )
    }

    #[test]
    fn raw_events_meet_in_one_frame() {
        let mut h = harness();
        let before = h.state().len();
        let events = [Event::Text("typed".into()), key(true)];
        let receipt = one_pass(&mut h, events.clone());
        assert_eq!(h.state().len(), before + 1);
        assert_eq!(h.state().last().unwrap(), &events);
        assert_eq!(receipt.layout_passes, 1);
    }

    #[test]
    #[should_panic(expected = "queued Harness actions ran extra input frames")]
    fn queued_actions_are_rejected_not_drained() {
        let mut h = harness();
        h.event(Event::Text("one".into()));
        h.event(Event::Text("two".into()));
        one_pass(&mut h, [Event::Text("raw".into())]);
    }

    #[test]
    #[should_panic(expected = "input batch changed")]
    fn even_one_hidden_queued_event_is_rejected() {
        let mut h = harness();
        h.event(Event::Text("queued".into()));
        one_pass(&mut h, [Event::Text("raw".into())]);
    }

    #[test]
    fn release_makes_the_later_enter_a_fresh_press() {
        let mut h = harness();
        one_pass(&mut h, [key(true)]);
        next_pass(&mut h, [key(false)]);
        next_pass(&mut h, [key(true)]);
        assert_eq!(h.state().last().unwrap(), &[key(true)]);
    }

    #[test]
    #[should_panic(expected = "implicit key repeat")]
    fn missing_release_cannot_claim_a_fresh_press() {
        let mut h = harness();
        one_pass(&mut h, [key(true)]);
        next_pass(&mut h, [key(true)]);
    }

    #[test]
    fn layout_discard_keeps_real_multiple_passes_in_one_frame() {
        let mut h = Harness::new_state(
            |ctx, armed: &mut bool| {
                if *armed && ctx.current_pass_index() == 0 {
                    ctx.request_discard("unit probe");
                }
            },
            false,
        );
        *h.state_mut() = true;
        let receipt = one_pass(&mut h, [Event::Text("one batch".into())]);
        assert_eq!(receipt.layout_passes, 2);
    }
}
