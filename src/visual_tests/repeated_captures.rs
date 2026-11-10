use std::collections::BTreeMap;

// Run lengths keep earlier frames without retaining a full RGBA buffer per step.
type Runs = Vec<([u8; 4], usize)>;

#[derive(Default)]
pub(super) struct RepeatedCaptures {
    previous: BTreeMap<String, Vec<(String, Runs)>>,
}

fn runs(pixels: &[u8]) -> Runs {
    let mut runs: Runs = Vec::new();
    for pixel in pixels.chunks_exact(4) {
        let pixel: [u8; 4] = pixel.try_into().unwrap();
        if let Some((last, count)) = runs.last_mut()
            && *last == pixel
        {
            *count += 1;
        } else {
            runs.push((pixel, 1));
        }
    }
    runs
}

fn nearly_same(a: &Runs, b: &Runs, pixels: usize) -> bool {
    let (mut left, mut right) = (a.iter(), b.iter());
    let (mut a, mut b) = (left.next().copied(), right.next().copied());
    let mut changed = 0;
    while let (Some((ap, an)), Some((bp, bn))) = (a, b) {
        let count = an.min(bn);
        if ap != bp {
            changed += count;
            if changed * 1000 >= pixels {
                return false;
            }
        }
        a = if an == count {
            left.next().copied()
        } else {
            Some((ap, an - count))
        };
        b = if bn == count {
            right.next().copied()
        } else {
            Some((bp, bn - count))
        };
    }
    a.is_none() && b.is_none()
}

// Compare compressed runs directly, without another full RGBA buffer or PNG decode.
fn within_tolerance(a: &Runs, b: &Runs) -> bool {
    let (mut left, mut right) = (a.iter(), b.iter());
    let (mut a, mut b) = (left.next().copied(), right.next().copied());
    while let (Some((ap, an)), Some((bp, bn))) = (a, b) {
        if ap
            .iter()
            .zip(bp)
            .any(|(a, b)| a.abs_diff(b) > super::compare::DEFAULT_TOLERANCE)
        {
            return false;
        }
        let count = an.min(bn);
        a = if an == count {
            left.next().copied()
        } else {
            Some((ap, an - count))
        };
        b = if bn == count {
            right.next().copied()
        } else {
            Some((bp, bn - count))
        };
    }
    a.is_none() && b.is_none()
}

impl RepeatedCaptures {
    /// A named return must match that frame at this window, within the suite's
    /// comparison tolerance. Retain it so subsequent repetition checks still run.
    pub fn check_named(
        &mut self,
        window: &str,
        state: &str,
        pixels: &[u8],
        earlier: &str,
    ) -> Option<String> {
        let current = runs(pixels);
        let previous = self.previous.entry(window.into()).or_default();
        let failure = match previous.iter().find(|(name, _)| name == earlier) {
            None => Some(format!("No earlier frame {earlier:?} at {window}")),
            Some((_, before)) if !within_tolerance(before, &current) => Some(format!(
                "{state} differs from {earlier} at {window} above the comparison channel tolerance of {} (or has different dimensions)",
                super::compare::DEFAULT_TOLERANCE,
            )),
            Some(_) => None,
        };
        previous.push((state.into(), current));
        failure
    }

    pub fn check(
        &mut self,
        _step: usize,
        window: &str,
        state: &str,
        pixels: &[u8],
        expect_same: Option<&str>,
    ) -> Option<String> {
        let current = runs(pixels);
        let previous = self.previous.entry(window.into()).or_default();
        let failure = expect_same.is_none().then(|| {
            previous.iter().find(|(_, before)| nearly_same(before, &current, pixels.len() / 4))
                .map(|(name, _)| format!("{name} and {state} at {window} differ in fewer than 0.1% of pixels. Use assert_state_matches for a return, or expect_same with a reason only for an intended small visual change."))
        }).flatten();
        previous.push((state.into(), current));
        failure
    }
}

#[test]
fn earlier_frames_and_five_changed_pixels_fail() {
    let mut check = RepeatedCaptures::default();
    let first = vec![0; 40_000];
    let other = vec![255; 40_000];
    assert!(check.check(1, "100x100", "first", &first, None).is_none());
    assert!(check.check(2, "100x100", "other", &other, None).is_none());
    assert!(
        check
            .check(3, "100x100", "two steps back", &first, None)
            .is_some()
    );
    let mut five = first.clone();
    five[..20].fill(255);
    assert!(
        check
            .check(4, "100x100", "five pixels", &five, None)
            .is_some()
    );
    assert!(
        check
            .check(
                5,
                "100x100",
                "intended",
                &first,
                Some("Returns to the first view")
            )
            .is_none()
    );
    assert!(
        check
            .check(6, "other size", "separate", &first, None)
            .is_none()
    );
    assert!(!nearly_same(
        &runs(&first),
        &runs(&{
            let mut ten = first;
            ten[..40].fill(255);
            ten
        }),
        10_000
    ));
}

#[test]
fn named_frames_use_the_reference_window_and_channel_tolerance() {
    let mut check = RepeatedCaptures::default();
    let first = vec![100; 16];
    let other = vec![200; 16];
    check.check(1, "2x2@1", "first", &first, None);
    check.check(2, "2x2@1", "other", &other, None);
    assert!(
        check
            .check_named("2x2@1", "exact", &first, "first")
            .is_none()
    );
    assert!(
        check
            .check_named("2x2@1", "noise", &[102; 16], "first")
            .is_none()
    );
    // One channel above tolerance fails; matching another earlier frame is not enough.
    let mut changed = first.clone();
    changed[7] += 3;
    assert!(
        check
            .check_named("2x2@1", "changed", &changed, "first")
            .is_some()
    );
    assert!(
        check
            .check_named("2x2@1", "wrong reference", &other, "first")
            .is_some()
    );
    assert!(
        check
            .check_named("2x2@1", "missing", &first, "unknown")
            .is_some()
    );
    assert!(
        check
            .check_named("2x2@2", "wrong window", &first, "first")
            .is_some()
    );
    assert!(
        check
            .check_named("2x2@1", "short", &[100; 12], "first")
            .is_some()
    );
    assert!(
        check
            .check_named("2x2@1", "long", &[100; 20], "first")
            .is_some()
    );
    // A compared return does not disable the guard on later captures.
    assert!(
        check
            .check(3, "2x2@1", "accidental duplicate", &first, None)
            .is_some()
    );
}

#[test]
fn named_frames_compare_split_runs_and_every_colour_channel() {
    let first = [100, 100, 100, 255].repeat(4);
    let mut noise = first.clone();
    noise[4] += 2;
    assert!(within_tolerance(&runs(&first), &runs(&noise)));
    for channel in 0..4 {
        let mut changed = first.clone();
        changed[channel] = changed[channel].saturating_sub(3);
        assert!(!within_tolerance(&runs(&first), &runs(&changed)));
    }
    // Compression boundaries can differ while all channels remain within tolerance.
    assert!(within_tolerance(&runs(&noise), &runs(&first)));
}
