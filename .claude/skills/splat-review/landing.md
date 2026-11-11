# Merges and landing

For a PR that contains a merge commit, and for the lead who lands it.

## Reviewing a merge

- Resolve each hunk. `git checkout --ours` or `--theirs` on a whole conflicted file drops the other side's
  hunks (PR 106 lost another package's captures and wording); use `git merge-file`.
- After a merge, confirm that each feature main added is still reached, and read the combined diff for
  dropped lines.
- Where both sides changed a shared test helper, the result keeps one helper that meets both contracts and
  every caller.
- Merged chrome counts in `baseline.json` are provisional: recapture the affected scenarios on merged source
  and apply them with `visual-baseline-chrome`.
- A branch that removes a feature main has, on purpose, needs a decision from the user, not a reviewer.

## Landing

- Merge `origin/main` into the branch; do not rebase a pushed branch. If main rewrote the code under the fix,
  merge first and fix on top.
- Run the full `audit-gate` on the merged source and read its exit code.
- Confirm `git merge-base --is-ancestor origin/main <head>`, and that GitHub shows the same head you tested
  (it can lag after a push), then `gh pr merge --squash --match-head-commit <sha>`.
- Commits made after a review get their own check: a fresh focused review for a product or data repair; a
  simplify pass gives pixel-identical captures.
