use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct RepeatedCaptures {
    previous: BTreeMap<String, (usize, String, [u8; 32])>,
}

impl RepeatedCaptures {
    pub fn check(
        &mut self,
        step: usize,
        window: &str,
        state: &str,
        pixels: &[u8],
        expect_same: Option<&str>,
    ) -> Option<String> {
        let digest = Sha256::digest(pixels).into();
        let before = self
            .previous
            .insert(window.into(), (step, state.into(), digest));
        let (previous_step, previous_state, previous_digest) = before?;
        (previous_step + 1 == step && previous_digest == digest && expect_same.is_none())
            .then(|| format!("{previous_state} and {state} at {window} are pixel-identical. Mark the second step expect_same with a reason only when this is intended."))
    }
}

#[test]
fn repeated_capture_fails_unless_the_second_step_expects_the_same_pixels() {
    let mut check = RepeatedCaptures::default();
    assert!(check.check(1, "960x640", "first", &[1, 2], None).is_none());
    assert!(check.check(2, "960x640", "repeat", &[1, 2], None).is_some());
    assert!(
        check
            .check(
                3,
                "960x640",
                "restored",
                &[1, 2],
                Some("Navigation returns to the saved view")
            )
            .is_none()
    );
    assert!(
        check
            .check(4, "960x640", "changed", &[3, 4], None)
            .is_none()
    );
    assert!(
        check
            .check(6, "960x640", "not adjacent", &[3, 4], None)
            .is_none()
    );
    assert!(
        check
            .check(7, "1200x800", "another size", &[3, 4], None)
            .is_none()
    );
}
