# Tests, fixtures and the visual gate

For any change to tests, visual scenarios, fixtures or `src/visual_tests/baseline.json`.

## Proof that can fail

Every new check goes red on the fault it claims to catch. Ask the PR for its fault runs, and test one
yourself by reading: would the check pass on the old code? If yes, it proves nothing (PR 203's alignment
check passed before the fix; PR 202's first repair check passed without the fix because the reader re-read
the file anyway).

- Assertions are exact: `assert_eq!`, fixed counts, `collect::<Option<Vec<_>>>()` or `zip` forms that fail
  when a control is missing.
- A filtered assertion (`.filter(..).all(..)`) first requires the expected items, then asserts a fact the
  filter did not use; otherwise it passes on an empty match.
- `expect_same` follows an assertion of the state change it expects.
- A gesture check shows a changed value partway; a navigation check asserts a changed position or a newly
  visible destination; a geometry check runs at the captured size; a bound injects a value just outside it.
- A check for a new result uses an id, value or count that did not exist before the action.
- A test name claims no more than the test checks, and no test encodes a defect as expected behaviour.

## Changed and removed checks

- A removed or changed assertion is either a wording update to the new text or replaced by a stronger
  check. A renamed accessible name changes the selector and keeps the destination or result assertion.
- Two capture steps that look the same mean one state does nothing; the step asserts the difference first.
- Test seams are `#[cfg(test)]` or identical to production; test-only hooks say so in their name
  (`simulate_folder_scan`).

## Fixtures

- A fixture is never renamed, lengthened or removed to make a check pass; realistic content that fails a check
  means the product is wrong. Adding a fixture for a new case is fine.
- Flag a fixture that creates a false state, a step that proves nothing (Undo already empty), and a scenario
  that edits fixture files on disk without isolating them.

## The baseline and the gate

- `src/visual_tests/baseline.json` allows only egui chrome counts. "Green" means no worse than the baseline,
  so the baseline can hide faults: trace each increase in the gate's `ergonomics.json`.
- An app control or text over a splitter or scroll bar is an app fault (`no-overlap/app-vs-chrome`), whatever
  bucket the report used. `KNOWN_APP_VS_CHROME` in `src/visual_tests/ergonomics.rs` may only shrink.
- A count said to come from main is checked with `git show origin/main:src/visual_tests/baseline.json`.
- Task-cost goals only get better.
- Mass "the app did not settle within 180s" under load means contention; the scenarios pass when rerun
  alone. Never raise the timeout.
- A focused repair run covers every scenario of every file the repair touched, inside or outside the task
  (PR 202 missed `session-recording` after a Sessions edit). The full `audit-gate` runs on merged source
  before merge.
