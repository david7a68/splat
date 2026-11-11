//! Support shared by unit tests and visual scenarios.
//!
//! [`CachedWork`] proves that a cache keeps input-sized work (a parse, a lookup, a read of the
//! whole draft) off frames that change nothing. It only keeps counts and gives verdicts. The
//! work count lives inside the function or access that does the work, for example
//! `library::repair_row` and `library::draft_view::DraftView`. So a second, uncounted call
//! cannot hide behind a counted decision to refresh.
//!
//! Each step gives its own verdict, for its own assertion:
//!
//! 1. [`CachedWork::warm`]: the view shows the state under test, and since
//!    [`CachedWork::start`] frames reached the seam and did the work at least `min_work` times.
//! 2. [`CachedWork::steady`]: unchanged frames and a resize reach the seam again, but do no new
//!    work.
//! 3. [`CachedWork::refresh`]: an edit to the input gives the right new result, with at least
//!    `min_work` new work.
//!
//! The fourth step is a mutation, done by hand and recorded in the PR: move the work outside
//! its guard, change neither the count nor these checks, and `steady` must fail.

use std::marker::PhantomData;

/// A seam's counts at one moment, or the change between two moments.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    /// How often frames reached the seam: its guard's own count, or the number of frames when
    /// the guard has none. `steady` requires it to grow, so frames that never reach the guard
    /// cannot pass as cached.
    pub reached: usize,
    /// How often the input-sized work ran, counted inside the work.
    pub work: usize,
}

impl Counts {
    fn since(self, earlier: Counts) -> Counts {
        Counts {
            reached: self.reached.saturating_sub(earlier.reached),
            work: self.work.saturating_sub(earlier.work),
        }
    }
}

/// The proof for one counted seam. See the module documentation.
pub struct CachedWork<S: ?Sized, F> {
    name: &'static str,
    read: F,
    start: Counts,
    subject: PhantomData<fn(&mut S)>,
}

impl<S: ?Sized, F: Fn(&mut S) -> Counts> CachedWork<S, F> {
    /// Takes the counts before the view opens. Counts from earlier views, or from earlier
    /// scenarios on the same thread, then do not count.
    pub fn start(name: &'static str, subject: &mut S, read: F) -> Self {
        let start = read(subject);
        Self {
            name,
            read,
            start,
            subject: PhantomData,
        }
    }

    /// Step 1. `shown` says whether the view shows the state under test.
    pub fn warm(&self, subject: &mut S, shown: bool, min_work: usize) -> Result<Counts, String> {
        let seen = (self.read)(subject).since(self.start);
        let name = self.name;
        if !shown {
            Err(format!(
                "{name}: the view does not show the state under test ({seen:?})"
            ))
        } else if seen.reached == 0 {
            Err(format!("{name}: no frame reached the seam ({seen:?})"))
        } else if seen.work < min_work {
            Err(format!(
                "{name}: the work ran {} times, at least {min_work} expected",
                seen.work
            ))
        } else {
            Ok(seen)
        }
    }

    /// Step 2. `frames` draws unchanged frames and a resize, and says whether the view still
    /// shows the state under test. Returns what those frames did.
    pub fn steady(
        &self,
        subject: &mut S,
        frames: impl FnOnce(&mut S) -> bool,
    ) -> Result<Counts, String> {
        let before = (self.read)(subject);
        let shown = frames(subject);
        let seen = (self.read)(subject).since(before);
        let name = self.name;
        if !shown {
            Err(format!(
                "{name}: the unchanged frames lost the state under test ({seen:?})"
            ))
        } else if seen.reached == 0 {
            Err(format!(
                "{name}: the unchanged frames did not reach the seam, so they prove nothing ({seen:?})"
            ))
        } else if seen.work > 0 {
            Err(format!(
                "{name}: unchanged frames did the work {} times; it belongs behind the cache",
                seen.work
            ))
        } else {
            Ok(seen)
        }
    }

    /// Step 3. `edit` changes the input and says whether the view now shows the right result.
    pub fn refresh(
        &self,
        subject: &mut S,
        min_work: usize,
        edit: impl FnOnce(&mut S) -> bool,
    ) -> Result<Counts, String> {
        let before = (self.read)(subject);
        let right = edit(subject);
        let seen = (self.read)(subject).since(before);
        let name = self.name;
        if !right {
            Err(format!(
                "{name}: after the edit the view does not show the new result ({seen:?})"
            ))
        } else if seen.work < min_work {
            Err(format!(
                "{name}: the edit did not refresh: the work ran {} times, at least {min_work} expected",
                seen.work
            ))
        } else {
            Ok(seen)
        }
    }
}

mod tests {
    use super::*;

    /// A seam with a guard: `frame` does the work only when the input changed, unless `unguarded`.
    #[derive(Default)]
    struct Seam {
        input: u32,
        seen: Option<u32>,
        unguarded: bool,
        counts: Counts,
    }

    impl Seam {
        fn frame(&mut self) {
            self.counts.reached += 1;
            if self.unguarded || self.seen != Some(self.input) {
                self.counts.work += 1;
                self.seen = Some(self.input);
            }
        }
    }

    fn proof(seam: &mut Seam) -> CachedWork<Seam, fn(&mut Seam) -> Counts> {
        CachedWork::start("Seam", seam, |seam| seam.counts)
    }

    /// Each verdict passes on a guarded seam and fails on the fault it names.
    #[test]
    fn cached_work_verdicts_fail_on_their_faults() {
        let mut seam = Seam::default();
        let check = proof(&mut seam);
        // Warm: nothing drawn yet.
        assert!(check.warm(&mut seam, true, 1).is_err(), "warm-unreached");
        seam.frame();
        assert!(check.warm(&mut seam, false, 1).is_err(), "warm-not-shown");
        assert!(
            check.warm(&mut seam, true, 2).is_err(),
            "warm-too-little-work"
        );
        assert_eq!(
            check.warm(&mut seam, true, 1),
            Ok(Counts {
                reached: 1,
                work: 1
            })
        );
        // Steady: guarded frames pass; no frame, a lost state or an unguarded frame fail.
        let three = |seam: &mut Seam| {
            (0..3).for_each(|_| seam.frame());
            true
        };
        assert_eq!(
            check.steady(&mut seam, three),
            Ok(Counts {
                reached: 3,
                work: 0
            })
        );
        assert!(
            check.steady(&mut seam, |_| true).is_err(),
            "steady-unreached"
        );
        assert!(
            check
                .steady(&mut seam, |seam| {
                    seam.frame();
                    false
                })
                .is_err(),
            "steady-lost"
        );
        seam.unguarded = true;
        assert!(check.steady(&mut seam, three).is_err(), "steady-unguarded");
        seam.unguarded = false;
        // Refresh: an edit must redo the work and show the right result.
        let edit = |seam: &mut Seam| {
            seam.input += 1;
            seam.frame();
            true
        };
        assert_eq!(
            check.refresh(&mut seam, 1, edit),
            Ok(Counts {
                reached: 1,
                work: 1
            })
        );
        assert!(
            check
                .refresh(&mut seam, 1, |seam| {
                    seam.frame();
                    true
                })
                .is_err(),
            "refresh-no-work"
        );
        assert!(
            check
                .refresh(&mut seam, 1, |seam| {
                    seam.input += 1;
                    seam.frame();
                    false
                })
                .is_err(),
            "refresh-wrong-result"
        );
        // Start counts only what came after it.
        let late = proof(&mut seam);
        assert!(
            late.warm(&mut seam, true, 1).is_err(),
            "warm-counts-since-start"
        );
    }
}
