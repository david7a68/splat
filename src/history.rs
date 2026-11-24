//! Back and Forward through what a reader opened. Library and Lookup each keep their own
//! trail.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Back,
    Forward,
}

/// One reader's history: the places Back and Forward go to, oldest first. The open entry
/// is not in it. It sits between `steps[at - 1]`, where Back goes, and `steps[at]`, where
/// Forward goes. A step swaps the open entry with the one it goes to.
#[derive(Debug)]
pub struct Trail<T> {
    steps: Vec<T>,
    at: usize,
}

impl<T> Default for Trail<T> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            at: 0,
        }
    }
}

impl<T> Trail<T> {
    /// Leave the open entry for a new one: Back returns to it, and nothing is ahead.
    pub fn leave(&mut self, here: Option<T>) {
        self.steps.truncate(self.at);
        self.steps.extend(here);
        self.at = self.steps.len();
    }

    /// Where a step would go.
    pub fn peek(&self, step: Step) -> Option<&T> {
        match step {
            Step::Back => self.steps.get(self.at.checked_sub(1)?),
            Step::Forward => self.steps.get(self.at),
        }
    }

    /// Take a step, if `find` finds where it goes. The open entry, `here`, takes its place.
    /// A place that is gone, or a step with no open entry to leave, drops that place.
    pub fn go<R>(
        &mut self,
        step: Step,
        here: Option<T>,
        find: impl FnOnce(&T) -> Option<R>,
    ) -> Option<(T, R)> {
        let slot = match step {
            Step::Back => self.at.checked_sub(1)?,
            Step::Forward => self.at,
        };
        let Some(found) = find(self.steps.get(slot)?) else {
            self.steps.remove(slot);
            self.at = slot;
            return None;
        };
        let target = match here {
            Some(here) => {
                self.at = slot + usize::from(step == Step::Forward);
                std::mem::replace(&mut self.steps[slot], here)
            }
            None => {
                self.at = slot;
                self.steps.remove(slot)
            }
        };
        Some((target, found))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Go with `here` open, where every place but "gone" is found.
    fn go(trail: &mut Trail<&'static str>, step: Step, here: &'static str) -> Option<&'static str> {
        trail
            .go(step, Some(here), |place| (*place != "gone").then_some(()))
            .map(|(place, ())| place)
    }

    #[test]
    fn a_step_swaps_the_open_entry_with_where_it_goes() {
        let mut trail = Trail::default();
        trail.leave(Some("a"));
        trail.leave(Some("b"));
        assert_eq!(go(&mut trail, Step::Back, "c"), Some("b"));
        assert_eq!(go(&mut trail, Step::Back, "b"), Some("a"));
        assert_eq!(go(&mut trail, Step::Back, "a"), None);
        assert_eq!(go(&mut trail, Step::Forward, "a"), Some("b"));
        assert_eq!(go(&mut trail, Step::Forward, "b"), Some("c"));
        assert_eq!(go(&mut trail, Step::Forward, "c"), None);
        assert_eq!(trail.peek(Step::Back), Some(&"b"));
    }

    #[test]
    fn leaving_for_a_new_entry_drops_what_was_ahead() {
        let mut trail = Trail::default();
        trail.leave(Some("a"));
        trail.leave(Some("b"));
        go(&mut trail, Step::Back, "c");
        trail.leave(Some("b"));
        assert_eq!(trail.peek(Step::Forward), None);
        assert_eq!(go(&mut trail, Step::Back, "d"), Some("b"));
        assert_eq!(go(&mut trail, Step::Back, "b"), Some("a"));
    }

    #[test]
    fn a_place_that_is_gone_is_dropped() {
        let mut trail = Trail::default();
        trail.leave(Some("a"));
        trail.leave(Some("gone"));
        assert_eq!(go(&mut trail, Step::Back, "b"), None);
        assert_eq!(go(&mut trail, Step::Back, "b"), Some("a"));
        assert_eq!(trail.peek(Step::Forward), Some(&"b"));
        go(&mut trail, Step::Forward, "a");
        assert_eq!(trail.peek(Step::Back), Some(&"a"));
        assert_eq!(trail.peek(Step::Forward), None);
        // With nothing open, a step drops the place it went to; nothing is left to return to.
        let found = trail.go(Step::Back, None, |_| Some(()));
        assert_eq!(found.map(|(place, ())| place), Some("a"));
        assert_eq!(
            (trail.peek(Step::Back), trail.peek(Step::Forward)),
            (None, None)
        );
    }
}
