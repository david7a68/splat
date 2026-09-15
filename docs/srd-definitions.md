# Whole-SRD definition conversion

The catalogue contains **2,065 definition drafts**, derived from the bundled
SRD 5.2.1. This pass has complete named-entry and page coverage, but **semantic
conversion is not complete**. It must not be described as an executable SRD.

| Content | Definitions |
| --- | ---: |
| Creatures | 336 |
| Spells | 339 |
| Magic items | 258 |
| Glossary entries | 155 |
| Classes / subclasses | 12 / 12 |
| Class feature entries | 232 |
| Backgrounds / species / feats | 4 / 9 / 17 |
| Ordinary equipment, including weapons and armor | 157 |
| Class progression tables | 12 |
| Other rule sections | 522 |

All 240 class progression rows have structured numeric columns, including
Warlock Pact Magic slots. All 38 weapons and 13 armor/shield rows have extracted
values. All spell headers have level, school, class list, casting time, range,
components, material text and duration. Creature drafts retain scores, signed
save totals, AC, fixed HP and movement values where stated. Variable HP is not
replaced by the first printed alternative. Defenses, senses, gear, skill totals
and languages are retained as labelled source text pending further conversion.

These counts describe an extraction inventory. They do not count all option
variants as separate, fully resolved templates. For example, the source heading
“Potions of Healing” still covers four strengths; the four existing execution
profiles remain separate. General tables other than the weapon, armor and class
progression tables still need table-specific decoding. Section extraction can
include table fragments within feature prose; the page reference retains the
original layout. Page coverage alone does not prove semantic coverage.

## Status and execution

The machine-readable [coverage inventory](../builtin/dnd2024-definitions/coverage.json)
records two separate kinds of gaps:

- **Conversion gap:** a source clause still needs typed mechanics, an explicit
  interpretation, or a resolved reference. There are **2,060 partial drafts**.
- **Engine gap:** a specified rule contract has no engine adapter. Five definitions
  have source-specific contracts: Hydra, Counterspell, Polymorph, Bag of Holding,
  and Wand of Fireballs. They require six unavailable contracts. Their referenced
  definitions can have further conversion gaps; specifying a wand does not make
  its Fireball dependency executable.

There are **zero executable full catalogue definitions**. The existing
[32 execution profiles](execution-proof.md) remain available for the bounded
proof. They are alternate views of the same source identities, not extra library
copies. The catalogue preserves UUIDs for keys shared with those profiles.
Load the catalogue with the source package, or the proof profiles with the source
package. Loading both views together correctly reports duplicate identities.
No second package manifest claims the `dnd2024` namespace.

The compiler rejects a note with conversion metadata before compiling actions.
Changing a status string cannot grant execution. The reader rejects concealed
conversion gaps, changed clause hashes, mismatched required-mechanic lists and
unknown status fields. Built-in verification checks the inventory, source hashes,
identities, references, all named sources, every rules page, and the rejection of
every catalogue draft by the execution compiler. This verifies structure and the
execution boundary, not the correctness of an unavailable adapter.

## Source and known ambiguity

Only the bundled official SRD contributes shipped data. A third-party structural
snapshot was investigated but is not included or used by this converter. The
source PDF, source text and reference notes remain unchanged. Attribution is
included in the catalogue's `LICENSE.txt`.

The Young White Dragon's Intelligence saving throw is printed as an unsigned
`2` on page 340. This was checked in the rendered PDF. The draft keeps its other
stats and records this save as unresolved. It does not silently choose `+2` or
`-2`. That ambiguity is a conversion gap, not an engine gap.

The Hydra contract retains the selected ruling: recompute extra reaction capacity
at the start of its next turn. It heals 20 HP per regrowth event, not per head.
Counterspell requires a Constitution save and does not expend the interrupted
spell's slot on failure. Polymorph retains HP and grants temporary HP. Bag of
Holding tracks both mass and volume limits. Wand of Fireballs permits 1–3 charges,
levels 3–5, dawn recovery and the last-charge destruction roll.

## Reproduce and verify

Run from the repository root, with Python, PyYAML and Poppler available:

```sh
python scripts/convert_srd_definitions.py --check
python -m unittest discover -s scripts -p 'test_*.py'
devenv shell -- cargo run --locked --bin verify-content -- --definitions
devenv shell -- cargo run --locked --bin verify-content -- --definitions --require-converted
devenv shell -- cargo run --locked --bin verify-content -- --executable
devenv shell -- cargo run --locked --bin execution-proof -- 42
```

`--definitions` succeeds for a valid catalogue with disclosed gaps.
`--require-converted` deliberately fails while partial definitions remain. Engine
gaps alone do not fail that flag; they still prevent execution. Both commands
print the separate gap lists. The proof command exercises only its stated subset.

To regenerate, omit `--check`. Source hashes pin the input. Reviewed contracts in
`scripts/srd_inputs/contracts.yaml` also pin their source note hash, so a changed
source cannot silently retain reviewed status. Regeneration refuses unknown
stale files and edited generated notes. It does not write campaign data.

## Remaining conversion work

The inventory is a starting point for the remaining semantic pass, not a
substitute for it. Work still includes:

- Encode remaining action riders, spell effects, resource recovery, traits,
  conditions, scaling, grants and equipment behavior.
- Resolve class/subclass membership and feature grants into entry references;
  split option families into selectable variants where the source requires it.
- Decode remaining tables, such as equipment subtypes, random tables and
  feature choices; connect table rows to the rules that use them.
- Replace each `source_rules` conversion gap only after all its clauses are
  accounted for by typed data or a specified source-specific contract.

Engine implementation remains outside this pass, as requested. These conversion
gaps must not be renamed to engine gaps merely to make the report pass.
