#!/usr/bin/env python3
"""Ask a Codex model to check, creature by creature, what Splat makes of each stat block.

Splat first writes one JSON file per creature: its note, its action tiles, its Spellcasting
lists with each spell's tile, roll and rules text, and the order a run arms them in
(`library::spells::tests::dump_creatures`, an ignored test). Then this script gives each file
to `codex exec` in a read-only sandbox and asks for a verdict in a fixed JSON shape. At the
end it writes `report.md` and `verdicts.jsonl` beside the verdicts.

Run from the repository root:

    scripts/check_creatures.py --packages "<PHB folder>:<MM folder>" --out artifacts/screenshots/creature-check

Book packages are copyrighted: keep `--out` under `artifacts/screenshots/`, which git ignores.
A second run skips creatures that already have a verdict, so an interrupted run resumes.
Standard library only.
"""

from __future__ import annotations

import argparse
import concurrent.futures
import json
import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path

MODEL = "gpt-6-luna"

SCHEMA = {
    "type": "object",
    "additionalProperties": False,
    "required": ["creature", "verdict", "problems"],
    "properties": {
        "creature": {"type": "string"},
        "verdict": {"type": "string", "enum": ["ok", "problems"]},
        "problems": {
            "type": "array",
            "items": {
                "type": "object",
                "additionalProperties": False,
                "required": ["area", "item", "expected", "got", "kind"],
                "properties": {
                    "area": {
                        "type": "string",
                        "enum": [
                            "action",
                            "spell list",
                            "spell roll",
                            "spell facts",
                            "run order",
                            "other",
                        ],
                    },
                    "item": {"type": "string"},
                    "expected": {"type": "string"},
                    "got": {"type": "string"},
                    "kind": {
                        "type": "string",
                        "enum": ["wrong", "missing", "extra", "doubtful"],
                    },
                },
            },
        },
    },
}

PROMPT = """\
You check how Splat, a D&D 2024 app for game masters, reads one creature's stat block.
The JSON at the end is the creature's note (`body`) and what Splat made of it. Compare
Splat's results with the printed stat block in `body` and with each spell's own rules text
(`spell_text`). Report only real faults. Do not report style or wording.

What Splat shows:
- `actions`: one tile for each paragraph of the stat block that has an attack roll, a saving
  throw DC or damage, in printed order. `label` is the name with the bonus or DC; `numbers`
  is the bonus or `DC n Ability`, then the damage dice and types; `facts` is the reach,
  range or area, and `half on a success`. A Spellcasting paragraph is not an action tile.
  An action that uses Spellcasting to cast a spell, with no numbers of its own, needs no tile.
- `spellcasting`: each Spellcasting paragraph with the caster's `save_dc` and
  `spell_attack`: the `+n to hit with spell attacks` it prints, or else its save DC less 8,
  which the 2024 rules always make the spell attack bonus (null with neither), and
  each list line (`printed`) with its `label` and its `spells`. Every spell the line names
  must be in `spells` or in `missing`. `missing` is right only for a spell that is not in the
  D&D 2024 SRD 5.2.1 or the 2024 Player's Handbook.
- Each spell: `name` (without words in brackets), `opens` (the spell entry it opens),
  `facts` (level cast or `Cantrip`, casting time, range, `Concentration`, and
  `half on a success`), and `rolls` (its tile, or null). `spell_record` is the spell
  entry's own level, casting time, range and concentration: check `facts` against it, not
  against what you remember of the spell.
- `rolls` must exist exactly when the spell's text has a spell attack, or a saving throw
  made by the creatures it targets. A cantrip whose damage grows with the caster's level
  must not roll. When it rolls: the save ability, DC = `save_dc`, or the attack bonus =
  `spell_attack`; the damage dice and types of a hit or a failed save at the level cast. A
  list item `(level N version)` is cast at level N, and the spell's `Using a Higher-Level
  Spell Slot` paragraph adds its dice; otherwise it is cast at its own level. Damage that
  comes later (at the end of a turn), only on a condition, or as another choice
  (`... or 3d8 Necrotic damage if ...`) is not part of the roll. `half on a success` only
  when the text says a success takes half.
- A paragraph of its own that casts spells by name (`Dread Shout. The wight casts Fear,
  using ...`) also gets spell tiles, from its own spell save DC or else Spellcasting's.
  They are not in `spellcasting`, but their rolls are in `run_order`.
- `run_order`: all the action labels, then the labels of the spells that roll (from the
  lists and from such paragraphs), in printed order.

Accepted limits, do not report: Ice Knife rolls only its hit; Spirit Guardians rolls only
its first damage choice; a spell whose save is made by someone other than its targets
(Sanctuary) still rolls a save; a spell of several rays or bolts rolls one of them.

Use `doubtful`, not `wrong`, when the rules leave room for Splat's reading. Before you
report a problem, read the stat block or the spell's text again and quote what it says in
`expected`. Never report a problem whose `expected` and `got` say the same thing.

Return JSON in the given schema. `creature` is the creature's `name`. `verdict` is `ok`
exactly when `problems` is empty. For each problem: `area`, `item` (the action or spell),
`expected` (what the stat block or spell text says), `got` (what Splat shows), `kind`
(`wrong`, `missing`, `extra`, or `doubtful` when the text is unclear).

Creature JSON:
"""


def dump(packages: str, dump_dir: Path) -> None:
    """Write Splat's reading of every creature, one JSON file each."""
    env = dict(os.environ)
    env["SPLAT_CREATURE_PACKAGES"] = packages
    env["SPLAT_CREATURE_DUMP"] = str(dump_dir.resolve())
    subprocess.run(
        [
            "devenv",
            "shell",
            "--",
            "cargo",
            "test",
            "--lib",
            "-p",
            "splat",
            "--",
            "--ignored",
            "--exact",
            "library::spells::tests::dump_creatures",
        ],
        env=env,
        check=True,
    )


def check(
    record: Path, verdicts: Path, schema: Path, model: str, effort: str | None, timeout: int
) -> str:
    """Ask the model about one creature. Returns a short status."""
    verdict = verdicts / record.name
    if verdict.exists():
        return "kept"
    prompt = PROMPT + record.read_text(encoding="utf-8")
    with tempfile.TemporaryDirectory() as empty:
        for attempt in range(2):
            partial = verdict.with_suffix(".part")
            result = subprocess.run(
                [
                    "codex",
                    "exec",
                    "-m",
                    model,
                    *(["-c", f"model_reasoning_effort={effort}"] if effort else []),
                    "-s",
                    "read-only",
                    "--skip-git-repo-check",
                    "--ephemeral",
                    "-C",
                    empty,
                    "--output-schema",
                    str(schema),
                    "-o",
                    str(partial),
                    "-",
                ],
                input=prompt,
                capture_output=True,
                text=True,
                timeout=timeout,
                check=False,
            )
            try:
                json.loads(partial.read_text(encoding="utf-8"))
            except (OSError, ValueError):
                if attempt == 0:
                    time.sleep(10)
                    continue
                (verdicts / f"{record.stem}.error.txt").write_text(
                    result.stdout[-4000:] + "\n" + result.stderr[-4000:], encoding="utf-8"
                )
                partial.unlink(missing_ok=True)
                return "failed"
            partial.rename(verdict)
            (verdicts / f"{record.stem}.error.txt").unlink(missing_ok=True)
            return "checked"
    return "failed"


def report(dump_dir: Path, verdicts: Path, out: Path) -> tuple[int, int, int]:
    """Merge the verdicts into `verdicts.jsonl` and `report.md`."""
    rows = []
    for path in sorted(verdicts.glob("*.json")):
        verdict = json.loads(path.read_text(encoding="utf-8"))
        record = json.loads((dump_dir / path.name).read_text(encoding="utf-8"))
        verdict["owner"] = record.get("owner")
        verdict["path"] = record.get("path")
        rows.append(verdict)
    failed = sorted(p.stem.removesuffix(".error") for p in verdicts.glob("*.error.txt"))
    with (out / "verdicts.jsonl").open("w", encoding="utf-8") as lines:
        for row in rows:
            lines.write(json.dumps(row, ensure_ascii=False) + "\n")
    faulty = [row for row in rows if row["problems"]]
    by_area: dict[str, int] = {}
    for row in faulty:
        for problem in row["problems"]:
            by_area[problem["area"]] = by_area.get(problem["area"], 0) + 1
    text = [
        "# Creature check",
        "",
        f"{len(rows)} creatures checked, {len(faulty)} with problems, {len(failed)} not checked.",
        "",
        "| Area | Problems |",
        "| --- | ---: |",
        *(f"| {area} | {count} |" for area, count in sorted(by_area.items())),
        "",
    ]
    for row in faulty:
        text += [f"## {row['creature']}", "", f"`{row['path']}` ({row['owner']})", ""]
        for problem in row["problems"]:
            text.append(
                f"- **{problem['area']}, {problem['kind']}**: {problem['item']}. "
                f"Expected: {problem['expected']}. Got: {problem['got']}."
            )
        text.append("")
    if failed:
        text += ["## Not checked", "", *(f"- {name}" for name in failed), ""]
    (out / "report.md").write_text("\n".join(text), encoding="utf-8")
    return len(rows), len(faulty), len(failed)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument(
        "--packages",
        default=os.environ.get("SPLAT_CREATURE_PACKAGES", ""),
        help="package folders to load beside the built-in content, separated by ':'",
    )
    parser.add_argument("--out", type=Path, required=True, help="folder for the dump and verdicts")
    parser.add_argument("--model", default=MODEL)
    parser.add_argument("--effort", help="reasoning effort: low, medium, high (model default)")
    parser.add_argument("--jobs", type=int, default=6, help="Codex calls at once")
    parser.add_argument("--timeout", type=int, default=900, help="seconds for one creature")
    parser.add_argument("--owner", action="append", help="only creatures of this package name")
    parser.add_argument("--only", help="only creatures whose name contains these words")
    parser.add_argument("--limit", type=int, help="check at most this many creatures")
    parser.add_argument("--no-dump", action="store_true", help="reuse the dump in --out")
    parser.add_argument("--report-only", action="store_true", help="only merge the verdicts")
    args = parser.parse_args()

    dump_dir = args.out / "dump"
    verdicts = args.out / "verdicts"
    verdicts.mkdir(parents=True, exist_ok=True)
    if not (args.no_dump or args.report_only):
        dump_dir.mkdir(parents=True, exist_ok=True)
        dump(args.packages, dump_dir)
    if not args.report_only:
        schema = args.out / "schema.json"
        schema.write_text(json.dumps(SCHEMA, indent=2), encoding="utf-8")
        records = []
        for path in sorted(dump_dir.glob("*.json")):
            record = json.loads(path.read_text(encoding="utf-8"))
            if args.owner and record.get("owner") not in args.owner:
                continue
            if args.only and args.only.lower() not in record["name"].lower():
                continue
            records.append(path)
        records = records[: args.limit] if args.limit else records
        print(f"Checking {len(records)} creatures with {args.model}, {args.jobs} at once.")
        done = 0
        with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
            futures = {
                pool.submit(
                    check, path, verdicts, schema, args.model, args.effort, args.timeout
                ): path
                for path in records
            }
            for future in concurrent.futures.as_completed(futures):
                done += 1
                try:
                    status = future.result()
                except subprocess.TimeoutExpired:
                    status = "timed out"
                    (verdicts / f"{futures[future].stem}.error.txt").write_text(
                        f"No verdict within {args.timeout} s.\n", encoding="utf-8"
                    )
                print(f"[{done}/{len(records)}] {futures[future].stem}: {status}", flush=True)
    checked, faulty, failed = report(dump_dir, verdicts, args.out)
    print(f"{checked} checked, {faulty} with problems, {failed} not checked: {args.out / 'report.md'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
