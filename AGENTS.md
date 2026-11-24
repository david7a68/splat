# Splat: notes for agents

Splat is a Rust and GPUI desktop app for D&D DMs. These notes hold only what you cannot find by looking.

## Where things are

- Code checks and commands: [Check a change](docs/development.md#check-a-change). Run Rust and app checks
  through `devenv shell -- <command>`.
- UI rules: `docs/design-system.md` is the source of truth. When your change alters what it says, change it in
  the same PR. Colours, sizes and type come from tokens (`src/design/lint.rs`). Pages build from the kit
  (`src/gui/kit/`); `docs/design-system.md` (Widgets) says which part to use.
- Domain words: `CONTEXT.md`. UI text is plain English in short, active sentences.

## Choose code checks

1. Start with the diff that will merge. Code checks apply to changes that can affect a build, program,
   generated output or test. Include the config, lockfiles, assets, content and fixtures those paths
   read. Review prose and comments on their own merits when they have no such effect.
2. Trace each change through callers and dependencies wherever inputs, outputs or effects can change.
   Follow imports, calls and build rules. Also follow shared files, generated output, schemas, saved
   data, environment settings and commands that call a tool. Check both paths for a move and the
   old path for a deletion.
3. Stop at an interface only when you can prove the change has no effect on the other side. Use
   source evidence or a comparison that covers the supported contract, including errors and side effects
   where relevant. A matching signature or one passing case is insufficient. If proof is missing, keep tracing.
4. Run the checks for every domain the trace reaches on the proposed merge source. In the PR, record
   what you traced, why you stopped at each boundary and which checks ran.

## Before you ask for review

Apply the rules below to the code and checks your trace reaches.

- **Captures are not a gate.** `devenv shell -- gui-test` writes the GPUI pages' captures to
  `artifacts/screenshots/gpui/`, and no test compares them. Look at every capture your change can affect, at
  each size it is taken. An app control or text over a splitter or scroll bar is an app fault; fix the
  clearance. Look at a new or wider menu at the narrowest size yourself.
- **A proof must be able to fail.** Break the code once and see each new check go red; say so in the PR. Do the
  same for an existing check when you edit its setup. A check that also passes on the old code proves nothing.
  Watch for a filtered assertion that passes on an empty match, an assertion that reads state before
  `settle()` lets the change land, and a fixture changed so that a check passes.
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
- **A ban is a list, not a proof.** `clippy.toml` holds the paths that a design part replaces (kept results,
  the save row, checked words). A path that goes around a part and is not listed is found by review: add it in
  the same PR.
- **Layout rules that keep getting broken:** every target at least 24 px and clear of splitters; a quiet action
  that starts a line uses `line_start`; choices after a message start at its text, safe first, with exactly
  16 px (`L`) before a destructive one; nothing that grows with the content runs every frame.
- **A raised lint count is a decision.** `src/design/lint_allowlist.txt` counts can only shrink on their own, but a
  change that raises a count and its allowance together passes the test. Say in the PR why each raised line is
  needed (a straight apostrophe in file content, a phrase outside its owner), or remove the use.
- **GUI tests.** A focused run (`devenv shell -- gui-test gui::pages::<page>`) covers every page the change can
  affect. A shell's ids and dice come from a hash of its name (`first_uuid` in `src/gui/harness.rs`). Renaming
  it gives it new ids and dice.
