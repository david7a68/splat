#!/usr/bin/env python3
"""One red run, the same way in every lane (Sol 2-006).

A red run proves a check can fail: a mutation patch breaks the code under test, the check runs,
and it must fail at the assertion it was written for. This command:

1. makes an isolated copy of the proved head (a scratch git worktree under
   `artifacts/screenshots/red/<label>`, which git ignores, seeded with a reflinked copy of this
   checkout's `target/`, so only the crates the patch touches compile again);
2. applies the mutation patch there;
3. runs the focused command there (a test filter, `gui-test <filter>`, any command);
4. REJECTS the run when the patch does not apply, the mutant does not compile, the command
   passes, or it fails without the expected assertion text or check id in a failure message;
5. keeps the log in `artifacts/screenshots/red-<label>.log`, removes the copy, and prints one
   PR-ready evidence row: `| patch summary | command | failed assertion | log path |`.

Usage (from the repository root):
  scripts/red_run.py --patch mutant.diff --expect "a day notice is worked out" \\
      -- devenv shell -- cargo test --locked --lib -p splat-trackers -- day_marks_follow_the_campaigns_day_count
  scripts/red_run.py --patch mutant.diff --expect "the tracker's row moves down" -- devenv shell -- gui-test gui::pages::trackers

Options:
  --patch FILE     the mutation, as `git diff` writes it (required)
  --expect TEXT    the assertion's message or its words (required)
  --head REF       the proved head (default: HEAD)
  --label NAME     names the copy and the log (default: from the patch's file name)
  --summary TEXT   the row's patch summary (default: the patch's first changed file and line)
  --keep           keep the copy for a look
Exit status: 0 for a good red run; 1 for a rejected one; 2 for a usage fault.
"""
import argparse
import os
import re
import shutil
import subprocess
import sys

COMPILE = re.compile(r"^error(\[E\d+\])?: |^error: could not compile ", re.M)


def run(cmd, cwd, **kw):
    return subprocess.run(cmd, cwd=cwd, text=True, capture_output=True, **kw)


def failures(log):
    """Each failure message in the log: a test's panic (its lines up to a blank line)."""
    blocks = []
    lines = log.splitlines()
    for at, line in enumerate(lines):
        if "panicked at" in line:
            block = [line]
            for follow in lines[at + 1:]:
                if not follow.strip() or follow.startswith("note: run with"):
                    break
                block.append(follow)
            blocks.append("\n".join(block))
    return blocks


def summary_of(patch_text):
    """The patch's first changed file and line, and its first removed and added lines."""
    path, line, removed, added = "?", "?", None, None
    new_line = None
    for raw in patch_text.splitlines():
        if raw.startswith("+++ "):
            path = raw[4:].removeprefix("b/")
        elif raw.startswith("@@"):
            match = re.search(r"\+(\d+)", raw)
            new_line = int(match.group(1)) if match else None
        elif new_line is None or raw.startswith("---"):
            continue
        elif raw.startswith("-"):
            if removed is None:
                removed = raw[1:].strip()
                line = str(new_line) if line == "?" else line
        elif raw.startswith("+"):
            if added is None:
                added = raw[1:].strip()
                line = str(new_line) if line == "?" else line
            new_line += 1
        else:
            new_line += 1
    words = f"{path}:{line}"
    if removed is not None:
        words += f": `{removed[:60]}`"
    if added is not None:
        words += f" → `{added[:60]}`"
    return words


def cell(text):
    return text.replace("|", "\\|").replace("\n", " ")


def main():
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--patch", required=True)
    parser.add_argument("--expect", required=True)
    parser.add_argument("--head", default="HEAD")
    parser.add_argument("--label")
    parser.add_argument("--summary")
    parser.add_argument("--keep", action="store_true")
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    if not command:
        print("Give the focused command after `--`.", file=sys.stderr)
        return 2
    root = run(["git", "rev-parse", "--show-toplevel"], ".").stdout.strip()
    head = run(["git", "rev-parse", args.head], root).stdout.strip()
    if not head:
        print(f"No such head: {args.head}", file=sys.stderr)
        return 2
    patch = os.path.abspath(args.patch)
    patch_text = open(patch).read()
    label = args.label or re.sub(r"[^A-Za-z0-9_-]", "-", os.path.splitext(os.path.basename(patch))[0])
    copy = os.path.join(root, "artifacts", "screenshots", "red", label)
    log_path = os.path.join(root, "artifacts", "screenshots", f"red-{label}.log")
    os.makedirs(os.path.dirname(log_path), exist_ok=True)
    if os.path.exists(copy):
        print(f"REJECTED: {copy} is in use; give another --label", file=sys.stderr)
        return 1
    made = run(["git", "worktree", "add", "--detach", copy, head], root)
    if made.returncode:
        print(f"REJECTED: no copy of {head}: {made.stderr.strip()}", file=sys.stderr)
        return 1
    verdict, failed_at = None, ""
    try:
        target = os.path.join(root, "target")
        if os.path.isdir(target):
            subprocess.run(["cp", "-a", "--reflink=auto", target, os.path.join(copy, "target")], check=False)
        applied = run(["git", "apply", patch], copy)
        if applied.returncode:
            verdict = f"the patch does not apply to {head[:9]}: {applied.stderr.strip()}"
        else:
            with open(log_path, "w") as log:
                log.write(f"RED_RUN head {head} patch {patch}\nCOMMAND {' '.join(command)}\n")
                log.flush()
                done = subprocess.run(command, cwd=copy, stdout=log, stderr=subprocess.STDOUT, text=True)
            text = open(log_path).read()
            blocks = failures(text)
            matched = [block for block in blocks if args.expect in block]
            if done.returncode == 0:
                verdict = "the mutant passed: this check cannot fail on it"
            elif COMPILE.search(text) and not blocks:
                verdict = "the mutant does not compile; a red run must fail at its assertion"
            elif not matched:
                first = blocks[0].splitlines()[:3] if blocks else ["no failure message in the log"]
                verdict = "it failed elsewhere, not at the expected check: " + " / ".join(first)
            else:
                lines = matched[0].splitlines()
                failed_at = next((l.strip() for l in lines if args.expect in l), lines[0].strip())
    finally:
        if not args.keep:
            run(["git", "worktree", "remove", "--force", copy], root)
            shutil.rmtree(copy, ignore_errors=True)
    if verdict:
        print(f"REJECTED red run ({label}): {verdict}\nLog: {log_path}")
        return 1
    summary = args.summary or summary_of(patch_text)
    print(f"| {cell(summary)} | `{cell(' '.join(command))}` | {cell(failed_at)} | `{log_path}` |")
    return 0


if __name__ == "__main__":
    sys.exit(main())
