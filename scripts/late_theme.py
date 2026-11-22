#!/usr/bin/env python3
"""Issue #810: move late theme installs onto `design::test_harness`, by script.

Input: the design lint's site list (each `late-theme` line of src/design/lint_allowlist.txt), or
the files named. For each `theme::install(&VAR.ctx);` line, it finds the statement above that
binds VAR with an egui_kittest builder (`.build_state(`, `.build_ui_state(`, `.build_ui(`,
`.build(`), switches it to the themed builder (`themed_state`, `themed_ui_state`, `themed_ui`,
`themed`), drops the install line, and imports the trait in that test. A site that does not
match this standard setup is printed as MANUAL and left for a hand edit.

Usage (from the repository root):
  scripts/late_theme.py                       dry run over the lint's site list (a diff)
  scripts/late_theme.py --limit 40            dry run over whole files, up to about 40 sites
  scripts/late_theme.py --write --limit 40    the same, written
  scripts/late_theme.py --write src/a.rs ...  these files, written
After a write, lower the files' `late-theme` counts in the allowlist (the lint test says how).
"""
import difflib
import re
import sys

ALLOWLIST = "src/design/lint_allowlist.txt"
INSTALL = re.compile(r"^\s*(?:crate::)?(?:design::)?theme::install\(&(\w+)\.ctx\);\s*$")
SWAP = [(".build_ui_state(", ".themed_ui_state("), (".build_state(", ".themed_state("),
        (".build_ui(", ".themed_ui("), (".build(", ".themed(")]
USES = {".themed_ui_state(": "Themed", ".themed_state(": "Themed", ".themed_ui(": "ThemedPage", ".themed(": "ThemedPage"}

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
        # The trait import, once in the enclosing function.
        for f in range(start, -1, -1):
            if re.match(r"^\s*(pub(\([^)]*\))?\s+)?fn \w+", lines[f]):
                # The body starts at the first line ending with `{` from here.
                b = f
                while not lines[b].rstrip().endswith("{"):
                    b += 1
                indent = re.match(r"^(\s*)", lines[f]).group(1) + "    "
                use = f"use crate::design::test_harness::{USES[swapped]} as _;"
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


def main():
    args = sys.argv[1:]
    write = "--write" in args
    limit = None
    if "--limit" in args:
        limit = int(args[args.index("--limit") + 1])
    files = [a for a in args if a.endswith(".rs")]
    if not files:
        total = 0
        for path, count in sites():
            if limit is not None and total + count > limit:
                continue
            files.append(path)
            total += count
    moved = manual = 0
    for path in files:
        done, skipped = rewrite(path, write)
        moved += done
        manual += skipped
    print(f"TOTAL {'moved' if write else 'would move'} {moved}, manual {manual}")


if __name__ == "__main__":
    main()
