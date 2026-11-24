#!/usr/bin/env python3
"""Compare the findings of two visual captures (Sol 2-020). Read-only.

Builders compare two runs of `devenv shell -- visual-test <label>` often (a head and its base, a
fix and its red run). This command does it the same way each time:

1. It checks that the two captures can be compared (the lead's compare preflight): the same
   state images, by name, and the same recorded renderer. Else it stops with PRECHECK_FAILED.
2. It reads each capture's finding rows: `ergonomics.json` (`violations`) and `stillness.json`.
3. It prints the rows only in A (REMOVED) and only in B (ADDED), by scenario, check, state
   (frame) and window width, with the control's name and, for an overlap, the other control's
   name, and the bounds. Each line ends with the JSON file it came from.

A row is matched by its scenario, check, state, window, control and words; numbers in the words
(an overlap's size, a moved rect) are left out of the match, so a sub-pixel change is not a new
row. Equal rows are counted, so a second copy of a row shows as added.

Usage (from the repository root):
  scripts/compare_captures.py artifacts/screenshots/<A> artifacts/screenshots/<B>
  scripts/compare_captures.py A B --scenario encounter-groups,run-add-creature --check G1,no-overlap
Options:
  --scenario LIST   only these scenarios (comma-separated names)
  --check LIST      only checks that start with one of these (comma-separated)
  --skip-preflight  compare rows even when the captures differ (says so on its first line)
Exit status: 0 when no row differs, 1 when rows differ, 2 when the preflight fails.
"""
import argparse
import collections
import json
import re
import sys
from pathlib import Path

NUMBER = re.compile(r"-?\d+(\.\d+)?")


def preflight(a, b):
    """The lead's compare preflight: the same state images and the same recorded renderer."""
    inventories, renderers = [], []
    for directory in (a, b):
        names = {p.name for p in directory.glob("*.png") if "__" in p.name}
        if not names:
            raise ValueError(f"{directory}: no state PNGs")
        renderer_file = directory / "renderer.txt"
        renderer = renderer_file.read_text().strip() if renderer_file.is_file() else ""
        if not renderer or renderer.lower() == "not recorded":
            raise ValueError(f"{directory}: renderer not recorded")
        inventories.append(names)
        renderers.append(renderer)
    scenarios = [sorted({name.split("__", 1)[0] for name in names}) for names in inventories]
    if scenarios[0] != scenarios[1]:
        raise ValueError(
            f"scenario set differs: only_A={sorted(set(scenarios[0]) - set(scenarios[1]))} "
            f"only_B={sorted(set(scenarios[1]) - set(scenarios[0]))}")
    if inventories[0] != inventories[1]:
        raise ValueError(
            f"capture names differ: only_A={sorted(inventories[0] - inventories[1])[:8]} "
            f"only_B={sorted(inventories[1] - inventories[0])[:8]}")
    if renderers[0] != renderers[1]:
        raise ValueError("renderers differ; capture the pair on the same renderer")
    renderer = renderers[0].splitlines()[0]
    return f"{len(inventories[0])} images, {len(scenarios[0])} scenarios, renderer {renderer}"


def bounds(rect):
    if not rect or len(rect) != 4:
        return ""
    if any(not isinstance(value, (int, float)) for value in rect):
        return "[no bounds]"
    x, y, w, h = rect
    return f"[{x:.0f},{y:.0f} {w:.0f}x{h:.0f}]"


def rows(directory):
    """Each finding row of one capture: (match key, printed line, source file)."""
    out = []
    ergonomics = directory / "ergonomics.json"
    if ergonomics.is_file():
        for row in json.loads(ergonomics.read_text()).get("violations", []):
            detail = row.get("detail", "")
            control = row.get("node", "")
            other = re.match(r"Overlaps (.+?) by ", detail)
            name = f"{control} <> {other.group(1)}" if other else f"{control}: {detail}"
            key = (row.get("scenario"), row.get("check"), row.get("state"), row.get("window"),
                   control, NUMBER.sub("#", detail))
            out.append((key, f"{name} {bounds(row.get('rect'))}", ergonomics))
    stillness = directory / "stillness.json"
    if stillness.is_file():
        for row in json.loads(stillness.read_text()):
            control = row.get("control", "")
            trigger = row.get("trigger", "")
            key = (row.get("scenario"), row.get("check"), row.get("state"), row.get("window"),
                   control, NUMBER.sub("#", trigger))
            line = f"{control} {bounds(row.get('old'))} -> {bounds(row.get('new'))} ({trigger})"
            out.append((key, line, stillness))
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("a", type=Path)
    parser.add_argument("b", type=Path)
    parser.add_argument("--scenario", default="")
    parser.add_argument("--check", default="")
    parser.add_argument("--skip-preflight", action="store_true")
    args = parser.parse_args()
    try:
        print(f"PREFLIGHT compatible: {preflight(args.a, args.b)}")
    except (ValueError, OSError) as error:
        if not args.skip_preflight:
            print(f"PRECHECK_FAILED: {error}", file=sys.stderr)
            return 2
        print(f"PREFLIGHT skipped: {error}")
    scenarios = {s for s in args.scenario.split(",") if s}
    checks = [c for c in args.check.split(",") if c]

    def wanted(key):
        return (not scenarios or key[0] in scenarios) and (
            not checks or any((key[1] or "").startswith(c) for c in checks))

    found = []
    for directory in (args.a, args.b):
        found.append([row for row in rows(directory) if wanted(row[0])])
    counts = [collections.Counter(key for key, _, _ in side) for side in found]
    differs = False
    for title, side, here, there in (("REMOVED (only in A)", found[0], counts[0], counts[1]),
                                     ("ADDED (only in B)", found[1], counts[1], counts[0])):
        extra = here - there
        lines = collections.defaultdict(list)
        for key, line, source in side:
            if extra[key] > 0:
                extra[key] -= 1
                scenario, check, state, window = key[:4]
                width = (window or "").split("x")[0]
                lines[scenario].append(f"  {check} / {state} / {width}: {line}  [{source}]")
        total = sum(len(v) for v in lines.values())
        print(f"{title}: {total}")
        for scenario in sorted(lines, key=str):
            print(f"{scenario}")
            print("\n".join(sorted(lines[scenario])))
        differs |= total > 0
    return 1 if differs else 0


if __name__ == "__main__":
    sys.exit(main())
