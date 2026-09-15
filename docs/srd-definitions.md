# SRD 5.2.1 definition conversion

The whole catalogue is converted: **1,970 typed rule specifications**, plus
**95 informational entries**. All **2,065** inventoried entries are accounted
for, and no `source_rules` conversion gaps remain. Engine implementation remains
outside this pass. The existing 32 bounded execution profiles still run.

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

The [coverage inventory](../builtin/dnd2024-definitions/coverage.json) records
source hashes, definition identities, status, execution profiles and source
adjudications. The [contract registry](../builtin/dnd2024-definitions/contracts.json)
records typed interfaces, closed parameter shapes, specification fingerprints,
references, citations and missing engine capabilities. The specifications live
once in the note bodies.

## What was converted

Each rule-bearing note now contains a source-specific registered rule contract
with explicit parameters in [marked tables and local YAML blocks](design/markdown-layout.md).
Frontmatter holds identity and source citations. These parameters include targets, save and attack outcomes,
scaling, choices, traits, conditions, durations, resources, recovery, forms,
spell grants, equipment effects, inventory capacities, restrictions and exceptions.
Keeping the source prose alone no longer qualifies a note as specified.

The source research is kept as [Markdown with YAML frontmatter](research/srd-conversion/).
The converter merges these specifications with the original stat/header/table
extraction to produce the built-in Markdown library. Full original prose remains
with each definition for reading and comparison. The input documents cite the
bundled official CC-BY SRD; no third-party mechanical database is shipped.

All 240 class progression rows retain their numeric columns and now have typed
feature grants. Warlock Pact Magic remains distinct from ordinary spell slots.
All 38 weapons and 13 armor/shield rows retain their values and have typed usage
rules. Item and spell variants, random tables, species legacy tables, pack
contents, spell lists, traps, hazards and character options are encoded within
the definitions that own them. Variants can be parameters/options within one
entry; the count does not imply each variant has a separate UUID.

Several source extractions mixed table columns into neighboring notes. Research
used the original page layout to place those mechanics under the right owner,
including the Apparatus of the Crab controls, species tables, and Giant Fly's
neighboring item prose. Source references and the PDF remain unchanged.

## Conversion is distinct from engine support

A **conversion gap** means a rule lacks a complete typed specification. There
are none in this generated catalogue. An **engine gap** means the required
registered adapter does not exist. The verifier lists these separately and
also groups missing adapters by capability.

The 1,970 source-specific contracts are interface and behavior specifications;
they are not 1,970 executable plugins. Their parameters are data, never scripts.
Some operations can later share engine adapters. This pass does not claim that
those adapters, or a full encounter simulator, have been implemented.

There are **zero executable full catalogue definitions**. The existing
[32 execution profiles](execution-proof.md) are alternate bounded views of the
same source identities. The catalogue preserves their shared UUIDs and keys.
Load either the catalogue or those profiles with the source package. Loading
both as independent library copies correctly reports duplicate identities.
There is one `dnd2024` package owner; no second namespace manifest was added.

The compiler rejects unavailable catalogue contracts before action execution.
Catalogue loading checks report declaration types, specification fingerprints
and conversion status. Standalone note reading checks the resolved body data. Built-in verification also checks the registry's closed parameter
shapes, implementation names, provenance, all references and source coverage.
Changing a content declaration cannot register engine code or grant execution.

## Source rulings and required inputs

The Young White Dragon's unsigned Intelligence SAVE `2` is now a supplied
base total of **+2**, under the user's ruling. Its Intelligence score remains
6. Roll `d20 + 2`, then apply additional modifiers; do not add the ability
modifier or proficiency again. [Online research](research/young-white-dragon-save.md)
found another project explicitly using +2, while Open5e and Foundry use −2.
The raw publisher value remains in the source record.

Narrow source ambiguities are explicit case inputs or rule selections, not
invented defaults or generic manual-resolution fallbacks:

- Creation: duration for minerals absent from its material-duration table.
- Flesh to Stone: whether its initial failed save counts toward the three failures.
- Control Weather: time to restore normal weather gradually after the spell ends.
- Ioun Stone: absorption of a spell larger than the remaining lifetime budget.
- Land vehicles: differing travel-pace statements on pages 12 and 192.

These cases appear in the coverage report's `adjudications` list. They must be
resolved when the affected case is used. Normal source-authorized GM choices,
such as Wish's Reshape Reality mode, remain explicit choices. Geometry facts
remain supplied assumptions; this work does not introduce maps or pathfinding.
The truncated Telekinesis example adds no guessed rule.

The [independent source audit](research/srd-contract-audit.md) checked nine complex
contracts. Its two findings were fixed: separate Apparatus claw damage rolls,
and Wand of Wonder's requirement that its own restraint still apply before the
petrification repeat save. Regression tests cover both.

## Reproduce and verify

From the repository root, with Python, PyYAML and Poppler available:

```sh
python scripts/convert_srd_definitions.py --check
python -m unittest discover -s scripts -p 'test_*.py'
devenv shell -- cargo run --locked --bin verify-content -- --definitions --require-converted
devenv shell -- cargo run --locked --bin verify-content -- --executable
devenv shell -- cargo run --locked --bin execution-proof -- 42
```

The conversion command now requires a research specification for every entry.
`--allow-partial` is a development option that cannot pass the final conversion
gate. Omit `--check` to regenerate. Regeneration refuses to overwrite edited
notes or remove unknown files. It does not write campaign data.

`--require-converted` succeeds with zero conversion gaps. Missing engine adapters
are reported and still block execution. The proof command tests only its stated
supported subset. Structural validation and a source audit do not replace
future adapter tests for all possible rule interactions.

Mapped tables now keep variants, pack contents, spell references and their usage
limits, and simple roll results together. Stable row keys attach complex details.
The reader reconstructs the same typed specifications; this is a presentation
change and does not add engine implementations. Checked duplicate source tables
are replaced by the authoritative table while citations and source files remain.
