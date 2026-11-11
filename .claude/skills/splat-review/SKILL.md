---
name: splat-review
description: Splat code review of a PR, branch, commit or diff (data safety, test proof, the visual gate and its baseline, design-system consistency). Use it whenever someone asks to review, check, approve or merge a Splat change, even a small one, and when writing the brief for a review agent.
---

# Splat review

You review a change to Splat, the Rust and egui desktop app for D&D DMs. Another agent wrote it and may be
wrong. Report only what you can show, and rate honestly: do not lower a finding to pass the change, and do not
raise one to have something to report.

## Steps

1. **Fix the target.** For a PR: `gh pr view <n>` (the body holds the claims, proof table, judgement calls and
   known limits) and `gh pr diff <n>`. Read source at the reviewed head with `git show <sha>:<path>`, not the
   live tree. Done when you have the diff, the head SHA and a numbered list of the PR's claims.
2. **Read the evidence.** In the gate log, `grep -n '^error'` must print nothing but `error: test failed` from a
   run whose only failures you can explain; one `FAILED` line from the settle probe's inner child is expected
   when its parent passes. Read exit codes, never a `tail` summary. Diff `src/visual_tests/baseline.json`
   against `origin/main` and trace every changed count to its rows in the gate's
   `artifacts/screenshots/<label>/ergonomics.json` (both controls of each overlap). With no gate log or
   report, say so in the report: an increase you cannot trace is a required repair, not a pass. Done when
   every baseline change has a named cause or is listed as untraced.
3. **Walk the checklist below**, then each disclosed list the diff reaches:
   - reads, writes, converts, moves, deletes, syncs or undoes DM data → [data.md](data.md)
   - adds or changes tests, scenarios, fixtures or the baseline → [proof.md](proof.md)
   - changes layout, wording, controls or `docs/design-system.md` → [ui.md](ui.md)
   - contains a merge commit, or you will land it → [landing.md](landing.md)
4. **Prove each finding** with a state walk-through: starting state, defaults, action, result (saved bytes,
   HP values, focused control, message shown). Name the check that would go red on it. If that check would
   also pass before the change, mark the finding PLAUSIBLE, not CONFIRMED. For each new proof check in the
   PR, name the assertion that fails when its property breaks.
5. **Report** in this shape, and keep it short enough to act on (about a screen and a half):

   ```
   MERGE | BLOCK — one sentence with the reason.
   Evidence: what you read (diff range, gate log, report) and what was missing.
   Findings (at most five, most severe first), each:
     file:line — what a DM loses or sees. CONFIRMED | PLAUSIBLE, traced from source | seen in capture.
     Walk-through: start, action, result. Smallest fix. The check that goes red.
   Required repairs: numbered, only what must change before merge.
   Known limits: non-blocking notes, one line each.
   Checked and fine: up to five lines on the risky paths you cleared.
   ```

   The verdict word comes first because the lead acts on it; the split between required repairs and known
   limits tells the author what to do now and the next reviewer what is still open.

## Checklist for every review

- **Weaker gate.** A deleted or loosened assertion, a new `#[ignore]`, a looser substring, a check moved to
  report-only or `not_measured`, or a baseline allowance other than egui chrome counts. A change that makes
  any gate weaker than main's is a BLOCK, however good the rest is (PR 205 first let every new
  app-vs-chrome overlap pass).
- **Claims match code.** Check each PR claim against the source. PR 197 claimed a sync refusal the code never
  ran; PR 203 claimed an alignment fix whose geometry equalled the old code's.
- **Whole family.** A rule fixed for one caller is not fixed. Grep the shared helper and any local renderer
  of the same thing; a caller that is only listed in the PR is still open (round 15 message choices, round 17
  `line_start`).
- **Data decides.** Branches use a state, a fault kind or one shared constant, never copied UI words,
  fixture names or scenario names.
- **Per-frame cost.** Nothing rebuilds or clones the catalogue each frame; no allocation or relayout that
  grows with the content runs every frame without a cache (PR 200 built a `String` per link per frame).
- **Hygiene.** Tokens only (`src/design/lint.rs`); no `unwrap` or panic on user data; no near-copy of a design
  helper (a local `warning_link` beside `design::wrapped_link`); no negative spacing that cancels another
  module's gap without a guard test.
- **Scope.** Edits outside the files the task owns, and restyling nobody asked for. Late regressions in
  earlier loops came mostly from the round before's small fixes.

## Verdict

BLOCK for data loss, wrong game state, a weaker gate, a false claim on a save or state path, or a proof that
cannot fail. Otherwise MERGE with required repairs. An optional finding is a required repair only on a data or
save path, or when a DM is likely to notice it; else it is a known limit.

## Briefing a review agent

Give it the PR number, a read-only worktree, the head SHA, the gate log path and the task's scope or decision
text, and tell it to follow this skill. Cap screenshot reading at 60 images, each read once. One reviewer per
PR, no subagents, about ten minutes. After a repair, brief a fresh reviewer on the repair commits only.
