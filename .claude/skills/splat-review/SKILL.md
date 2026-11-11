---
name: splat-review
description: Splat code review of a PR, branch, commit or diff. Use it whenever someone asks to review, check, approve or merge a Splat change.
---

# Splat review

Review the change as the lead who must merge it. Another agent wrote it and may be wrong. Report only what
you can show. Read source at the reviewed commit, list the author's claims, and test each against the code.

## What this repository's reviews most often missed

- **The baseline hides faults.** `src/visual_tests/baseline.json` may hold only egui chrome counts, and a green
  gate means only "no worse than the baseline". Trace every changed count to its cause. An app control or text
  over a splitter or scroll bar is an app fault whatever bucket it is filed in. An increase you cannot trace is
  a required repair.
- **A proof must be able to fail.** For each new check, ask whether it would also pass on the old code. Look for
  filtered assertions that pass on an empty match, `expect_same` with no asserted change before it, and a
  fixture changed so that a check passes.
- **A gate gets weaker only on purpose.** For each deleted or loosened assertion, new `#[ignore]`, or check
  moved to report-only, name the fault it caught and what catches it now. If the task asked for the change (a
  test simplification, a removed feature), confirm that the remaining check still fails on that fault, and list
  any coverage the task chose to drop as a known limit. A weakening that the task did not ask for, or that the
  PR does not state, is a BLOCK.
- **State belongs to one object.** When a change clears or replaces a result, an Undo, a draft or cached
  state, ask whose it was: another note's, another reader's, an older action's. Walk each entry point (keyboard,
  search, drag, repair, Undo, a background timer) against each risk state (unsent text, an outside edit, a
  moved or unreadable file, two faults at once).
- **A rule fixed for one caller is not fixed.** Grep the shared helper and local copies of it.
- **Layout rules reviewers keep finding broken:** every target at least 24 px and clear of splitters; a quiet
  action that starts a line uses `line_start`; choices after a message start at its text, safe first, with
  exactly 16 px (`L`) before a destructive one; tokens only; nothing that grows with the content runs every
  frame.

## Report

```
MERGE | BLOCK — one sentence with the reason.
Evidence: what you read (diff range, gate log, report) and what was missing.
Findings (at most five, most severe first), each:
  file:line — what a DM loses or sees. CONFIRMED | PLAUSIBLE, traced from source | seen in capture.
  Walk-through: start, action, result. Smallest fix. The check that goes red.
Required repairs: numbered, only what must change before merge.
Known limits: non-blocking notes, one line each.
```

BLOCK for data loss, wrong game state, an undeclared weaker gate, a false claim on a save or state path, or a
proof that cannot fail. Otherwise MERGE with required repairs.
