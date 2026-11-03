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

impl RepeatedCaptures {
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
                .map(|(name, _)| format!("{name} and {state} at {window} differ in fewer than 0.1% of pixels. Mark the later step expect_same with a reason only when this is intended."))
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
