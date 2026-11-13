# Splat: notes for agents

Splat is a Rust and egui desktop app for D&D DMs. These notes hold only what you cannot find by looking.

## Where things are

- Commands and the visual gate: `README.md`, sections Development and Visual tests. Run them through
  `devenv shell -- <command>`. `check` is formatting and Clippy; `visual-test <label> --only a,b` is fast
  feedback; `audit-gate <label>` is the full gate.
- UI rules: `docs/design-system.md` is the source of truth. When your change alters what it says, change it in
  the same PR. Colours, sizes and type come from tokens (`src/design/lint.rs`).
- Domain words: `CONTEXT.md`. UI text is plain English in short, active sentences.

## Before you ask for review

Past reviews found the same kinds of fault again and again. Check your own change against these first.

- **The baseline hides faults.** `src/visual_tests/baseline.json` may hold only egui chrome counts, and a green
  gate means only "no worse than the baseline". Know the cause of every count you change: read both controls of
  each new overlap in the gate's `ergonomics.json`. An app control or text over a splitter or scroll bar is an
  app fault whatever bucket it is filed in; fix the clearance. The gate compares a menu's or popup's content
  with a splitter or scroll bar under it, but only at the sizes its state checks (`Sizes::Default` checks only
  960 px), so look at a new or wider menu at the other sizes yourself.
- **A proof must be able to fail.** Break the code once and see each new check go red; say so in the PR. Do the
  same for an existing check when you edit its setup. A check that also passes on the old code proves nothing.
  Watch for a filtered assertion that passes on an empty match, `expect_same` with no asserted change before
  it, and a fixture changed so that a check passes.
- **Extend a test before you add one.** When an existing test has the same setup, add the case or the
  assertion there.
- **A gate gets weaker only on purpose.** Delete or loosen an assertion, add `#[ignore]`, or move a check to
  report-only only when the task asks for it. Then say in the PR which fault the old check caught and what
  catches it now, or that the task chose to drop it.
- **State belongs to one object.** When a change clears or replaces a result, an Undo, a draft or cached state,
  ask whose it was: another note's, another reader's, an older action's. Walk each entry point (keyboard,
  search, drag, repair, Undo, a background timer) against each risk state (unsent text, an outside edit, a
  moved or unreadable file, two faults at once). For a result that stays on screen and for an Undo, name the
  owner and the event that ends it. Then test A, an unrelated B, and Undo of A: B's change stays, and A's words
  on screen go back together with A's data.
- **A rule fixed for one caller is not fixed.** Grep the shared helper and local copies of it, and fix every
  caller in the same change. Another screen that shows the same thing is also a caller: a list row and its
  pane, a preview and the live view.
- **Layout rules that keep getting broken:** every target at least 24 px and clear of splitters; a quiet action
  that starts a line uses `line_start`; choices after a message start at its text, safe first, with exactly
  16 px (`L`) before a destructive one; nothing that grows with the content runs every frame.
- **A raised lint count is a decision.** `src/design/lint_allowlist.txt` counts can only shrink on their own, but a
  change that raises a count and its allowance together passes the test. Say in the PR why each raised line is
  needed (a new `allow(deprecated)`, a `Sentence::outside`, a body message place), or remove the use.
- **The audit's counts (G6) run in every gate.** When your change makes a task of `artifacts/dm-audit/` cost
  more or fewer acts on purpose, run `devenv shell -- audit-freeze counts` and say in the pull request why; add
  a new task with `devenv shell -- audit-freeze add`.
- **Pull requests of the session-cycle build plan also run G7:** `SPLAT_FROZEN=1 devenv shell -- audit-gate
  <label>`. The plain reference changes only in one commit made by `devenv shell -- audit-freeze reference` on
  an unchanged commit of `main`, which it records; make that commit again after each merge of `main`. A
  feature adds its views with `devenv shell -- audit-freeze add` and its changes as rows in `allowed.json`.
- **Run what you touched.** A focused run covers the scenarios of every file you changed, also files outside
  your task. The full `audit-gate` runs on merged source before merge. A visual scenario's ids and dice come
  from a hash of its name (`first_uuid` in `src/visual_tests/mod.rs`), not from its place in `SCENARIOS`. Add
  or move a scenario anywhere; renaming one gives it new ids and new dice.
