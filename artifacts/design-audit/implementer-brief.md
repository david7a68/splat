# Implementer brief

You implement one package of a design audit of Splat, a Rust and egui desktop app. The
design decisions are already made. Your job is code, tests and evidence, not taste.

## Read first

1. `artifacts/design-audit/README.md`: the metrics and severity scale.
2. `artifacts/design-audit/<iter>/decisions.md`: the shared rules and **your package**.
3. The finding files in `artifacts/design-audit/<iter>/` that your package names. Each
   finding's "Proposed fix" is accepted as written unless `decisions.md` changes it.
4. `docs/design-system.md`, `README.md` (Development and Visual tests) and `CONTEXT.md`.

## Rules

- Do exactly what your package says. Where it is unclear, choose what is closest to
  `docs/design-system.md`, and list the choice in the PR under "Judgement calls". Do not
  restyle anything outside your package.
- Package A owns `src/design/`. Other packages may only **add** a new function to
  `src/design/components.rs` when their package says so, and must not change existing ones.
- Colours, sizes and type come from tokens only (`src/design/lint.rs` enforces this).
- Keep the brief's workflow goals (`docs/ux-intent.md` section 8). Update a visual scenario
  when a word or a step changes, and add a scenario step where a new state needs a capture.
  A cost goal may only get better. Never add an allowance to
  `src/visual_tests/baseline.json` for anything except egui chrome counts.
- Remove any test that your change makes wrong; do not weaken a test to pass.
- Write plain English in the UI and docs: short sentences, active voice (ASD-STE100 style).
- Update `docs/design-system.md` and `README.md` where your change alters what they say.

## Build, test and timing

Work in your own worktree. Run commands with `devenv shell -- <command>` from the
worktree root. Use the same target directory for every build in your worktree.

- `devenv shell -- cargo test --locked` runs every unit test and the visual suite.
- `devenv shell -- check` runs formatting and Clippy with warnings denied.
- `SPLAT_VISUAL_VIDEO=0 devenv shell -- visual-test <label>` writes captures to
  `artifacts/screenshots/<label>/`. Use `--only a,b` while you iterate. Before the PR, run
  the full suite once with the label `<iter>-<package letter>` (for example `iter-1-B`).

Record the wall time of every build and test command you run (use `date +%s` before and
after). Put a table of them in the PR body. If a single edit-build-test loop takes more than
3 minutes, say so in the PR and say what took the time.

## Finish

1. Merge `origin/main` into your branch (merge, do not rebase) and fix conflicts. Run the
   full test suite and `check` again after the merge.
2. Commit with clear messages. End each commit message with
   `Co-Authored-By: Codex (gpt-6.1-sol) <noreply@openai.com>`.
3. Push and open a PR against `main` with `gh pr create`. Title: a plain sentence of what
   changed. Body: the findings fixed (by ID), the findings not fixed and why, judgement
   calls, the capture label and the 6 to 10 capture file names that best show the change,
   test results, and the timing table. Do not merge it.
4. Reply with: the PR URL, the head SHA, the capture label, and any finding you could not
   fix.
