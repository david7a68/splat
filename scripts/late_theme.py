#!/usr/bin/env python3
"""Issue #810: move late theme installs onto `design::test_harness`, by script.

Input: the design lint's site list (each `late-theme` line of src/design/lint_allowlist.txt), or
the files named. For each `theme::install(&VAR.ctx);` line, it finds the statement above that
binds VAR with an egui_kittest builder (`.build_state(`, `.build_ui_state(`, `.build_ui(`,
`.build(`, or `Harness::new*`), switches it to the themed builder (`themed_state`, `themed_ui_state`, `themed_ui`,
`themed`), drops the install line, and imports the trait in that test. A site that does not
match this standard setup is printed as MANUAL and left for a hand edit.

Usage (from the repository root):
  scripts/late_theme.py                       dry run over the lint's site list (a diff)
  scripts/late_theme.py --limit 40            dry run over whole files, up to about 40 sites
  scripts/late_theme.py --write --limit 40    the same, written
  scripts/late_theme.py --write src/a.rs ...  these files, written
  --skip-open MINE    leave out each file that an open, non-draft PR changes (`gh pr list`),
                      except PRs whose head branch matches the regular expression MINE (your
                      own); the skipped files are printed as SKIP, to retry after those land
After a write, lower the files' `late-theme` counts in the allowlist (the lint test says how).
"""
import difflib
import re
import sys

ALLOWLIST = "src/design/lint_allowlist.txt"
INSTALL = re.compile(r"^\s*(?:crate::)?(?:design::)?theme::install\(&(\w+)\.ctx\);\s*$")
# `Harness::new*` is `Harness::builder()` with its defaults and the same first draw.
SWAP = [("Harness::new_ui_state(", "Harness::builder().themed_ui_state("),
        ("Harness::new_state(", "Harness::builder().themed_state("),
        ("Harness::new_ui(", "Harness::builder().themed_ui("),
        ("Harness::new(", "Harness::builder().themed("),
        (".build_ui_state(", ".themed_ui_state("), (".build_state(", ".themed_state("),
        (".build_ui(", ".themed_ui("), (".build(", ".themed(")]
def use_of(new):
    return "Themed" if new.endswith("_state(") else "ThemedPage"

def rewrite(path, write):
    original = open(path).read()
    lines = original.split("\n")
    done, skipped = 0, []
    i = 0
    while i < len(lines):
        m = INSTALL.match(lines[i])
        if not m:
            i += 1
            continue
        var = m.group(1)
        # The statement that binds VAR, above.
        start = None
        for j in range(i - 1, max(-1, i - 120), -1):
            if re.search(rf"\blet\s+(mut\s+)?{var}\b\s*(:[^=]*)?=", lines[j]):
                start = j
                break
        if start is None:
            skipped.append((i + 1, "no binding"))
            i += 1
            continue
        swapped = None
        for k in range(start, i):
            for old, new in SWAP:
                if old in lines[k]:
                    lines[k] = lines[k].replace(old, new, 1)
                    swapped = new
                    break
            if swapped:
                break
        if not swapped:
            skipped.append((i + 1, "no builder"))
            i += 1
            continue
        del lines[i]
        # A helper that binds the harness only to return it returns the builder directly.
        if lines[i].strip() == var and lines[i - 1].rstrip().endswith(";"):
            del lines[i]
            lines[i - 1] = lines[i - 1].rstrip()[:-1]
            lines[start] = re.sub(rf"let\s+(mut\s+)?{var}\b\s*(:[^=]*)?=\s*", "", lines[start], count=1)
        # The trait import, once in the enclosing function.
        for f in range(start, -1, -1):
            if re.match(r"^\s*(pub(\([^)]*\))?\s+)?fn \w+", lines[f]):
                # The body starts at the first line ending with `{` from here.
                b = f
                while not lines[b].rstrip().endswith("{"):
                    b += 1
                indent = re.match(r"^(\s*)", lines[f]).group(1) + "    "
                use = f"use crate::design::test_harness::{use_of(swapped)} as _;"
                if use not in lines[b + 1] and use not in lines[b + 2]:
                    lines.insert(b + 1, indent + use)
                    i += 1
                break
        done += 1
    text = "\n".join(lines)
    if write:
        open(path, "w").write(text)
    else:
        sys.stdout.writelines(difflib.unified_diff(
            original.splitlines(True), text.splitlines(True), path, path))
    for line, why in skipped:
        print(f"MANUAL {path}:{line}: {why}")
    print(f"{'MOVED' if write else 'WOULD MOVE'} {path}: {done}")
    return done, len(skipped)


def sites():
    """The lint's site list: each `late-theme` line of the allowlist, as (file, count)."""
    for line in open(ALLOWLIST):
        parts = line.split()
        if len(parts) == 3 and parts[1] == "late-theme":
            yield parts[0], int(parts[2])


def open_pr_files(mine):
    """Each file an open, non-draft PR changes, by PR number, leaving out branches matching `mine`."""
    import json
    import subprocess
    listed = subprocess.run(
        ["gh", "pr", "list", "--state", "open", "--limit", "200",
         "--json", "number,isDraft,headRefName,files"],
        check=True, capture_output=True, text=True).stdout
    files = {}
    for pr in json.loads(listed):
        if pr["isDraft"] or re.search(mine, pr["headRefName"]):
            continue
        for changed in pr["files"]:
            files.setdefault(changed["path"], []).append(pr["number"])
    return files


def main():
    args = sys.argv[1:]
    write = "--write" in args
    busy = {}
    if "--skip-open" in args:
        mine = args[args.index("--skip-open") + 1]
        args.remove(mine)
        busy = open_pr_files(mine)
    limit = None
    if "--limit" in args:
        limit = int(args[args.index("--limit") + 1])
    files = [a for a in args if a.endswith(".rs")]
    if not files:
        total = 0
        for path, count in sites():
            if path in busy or (limit is not None and total + count > limit):
                continue
            files.append(path)
            total += count
    for path in [path for path in files if path in busy]:
        print(f"SKIP {path}: open PR {', '.join(map(str, busy[path]))}")
        files.remove(path)
    moved = manual = 0
    for path in files:
        done, skipped = rewrite(path, write)
        moved += done
        manual += skipped
    print(f"TOTAL {'moved' if write else 'would move'} {moved}, manual {manual}")


if __name__ == "__main__":
    main()
