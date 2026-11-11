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
  app fault whatever bucket it is filed in; fix the clearance.
- **A proof must be able to fail.** Break the code once and see each new check go red; say so in the PR. A check
  that also passes on the old code proves nothing. Watch for a filtered assertion that passes on an empty
  match, `expect_same` with no asserted change before it, and a fixture changed so that a check passes.
- **A gate gets weaker only on purpose.** Delete or loosen an assertion, add `#[ignore]`, or move a check to
  report-only only when the task asks for it. Then say in the PR which fault the old check caught and what
  catches it now, or that the task chose to drop it.
- **State belongs to one object.** When a change clears or replaces a result, an Undo, a draft or cached state,
  ask whose it was: another note's, another reader's, an older action's. Walk each entry point (keyboard,
  search, drag, repair, Undo, a background timer) against each risk state (unsent text, an outside edit, a
  moved or unreadable file, two faults at once).
- **A rule fixed for one caller is not fixed.** Grep the shared helper and local copies of it, and fix every
  caller in the same change.
- **Layout rules that keep getting broken:** every target at least 24 px and clear of splitters; a quiet action
  that starts a line uses `line_start`; choices after a message start at its text, safe first, with exactly
  16 px (`L`) before a destructive one; nothing that grows with the content runs every frame.
- **Run what you touched.** A focused run covers the scenarios of every file you changed, also files outside
  your task. The full `audit-gate` runs on merged source before merge.
