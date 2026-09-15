# SRD executable conversion: first proof

The converted Markdown notes in `builtin/dnd2024-mechanics` now supply executable
attack, save, healing, resource and consumable definitions. The Rust interpreter
reads the notes through the existing reader, compiles supported fields into typed
structures, and resolves actions against independent runtime actors. It does not
interpret Markdown prose or load code supplied by a package.

This first reviewed set contains Guard, Skeleton, Black Bear, Acid Splash,
Inflict Wounds, Cure Wounds, Healing Word, and four strengths of Potion of Healing.
These notes extend the existing `dnd2024` package namespace; they do not register
another owner with the same namespace. Their source references resolve into the
unchanged complete SRD package. Source attribution in that package applies here.

## Run it

```sh
devenv shell -- cargo run --locked --bin verify-content -- --executable
devenv shell -- cargo run --locked --bin execution-proof -- 42
devenv shell -- cargo test --locked execution
```

The demonstration runs a Black Bear against two Skeletons. It loads all attack
bonuses, AC, HP, damage dice and Multiattack steps from Markdown. It prints the
seed, engine/RNG versions, initiative, outcome and event trace as JSON. The data
and live encounter state remain separate; nothing is saved to authored notes.
The engine can also take controlled die faces for source-rule regression tests.

Identical Skeletons share an initiative roll; they take separate turns. A fixed
controller order resolves ties. The demonstration explicitly supplies melee
reach, clear paths and absence of other Advantage/Disadvantage sources. It has
no map. Its simple strategy may forgo a remaining Multiattack after its fixed
target dies. A 20-round cap is reported as a limit, never a win.
Monster ranged attacks use the source rule that the stat block carries the
necessary ammunition (SRD p255); equipment loss/transfer is outside this scene.

## What the proof checks

- Supplied attack bonuses and AC are not rebuilt or counted twice. Natural 1
  misses, natural 20 hits, and criticals double damage dice but not flat bonuses.
- Ranged attacks in the long range band have Disadvantage. Presence of both
  Advantage and Disadvantage cancels them under the D&D rule.
- Multiattack uses distinct attack and damage rolls with one action debit.
- Area membership is an explicit complete target set. One use-scoped damage
  roll is shared while each creature makes its own saving throw.
- Saving throws have no general natural-1 failure or natural-20 success rule.
  Successful saves use the spell's declared zero/half damage outcome.
- Immunity, Resistance and Vulnerability apply by damage type, without repeated
  halving from duplicate sources. Temporary HP absorbs damage before HP.
- The 2024 Cure Wounds uses 2d8 per slot level. Inflict Wounds uses a Constitution
  save and half damage on success. Healing Word uses a Bonus Action.
- Spell grants supply the save DC, ability modifier and cantrip tier. A caller
  cannot replace those with an incompatible value. Slot level must match the
  declared casting level; only one slot can be expended in a turn.
- Potion use consumes one selected item instance and a Bonus Action. Healing
  stops at maximum HP. Separate placements keep independent quantities.
- Missing target facts, components, resources or unsupported mechanics block
  resolution without changing state, event history or the random stream.

Source-driven tests check these outcomes with known dice. A successful compile
alone does not count as an engine outcome test.

## Current execution boundary

This is a bounded interpreter, not the full simulator. It resolves instantaneous
creature attacks, save damage, healing and potion use in an explicitly supplied
encounter state. It implements monster death at zero HP. Character death saves,
knockout, ongoing effects, Concentration, reactions/Counterspell, resting,
equipment changes, actor movement and campaign rule selection remain outside
this interpreter. Creature gear/senses are not instantiated as a complete actor
sheet by this proof. It assumes the stat-block attack and defense configuration
and needs controller facts for relevant spatial and roll conditions.

The controlled caster used in tests is an explicit fixture with spell grants;
it is not a claim that an SRD Guard can cast spells. The interpreter rejects
unsupported fields and required mechanics instead of dropping them. Required
components remain explicit, and spell permission is checked separately from
slot availability. Future reaction support must replace the uninterrupted
casting path with the specified rule-owned interruption stages.

The existing schema is sufficient for this slice: rolls, parameters, costs,
source references and nested repeat steps are all declared in the notes.
The interpreter's strict typed subset is intentionally smaller than the complete
candidate schema. Unknown required nested fields fail compilation. Wider
conversion must retain all source clauses and disclose any required mechanics
that cannot yet compile or execute.

## Expansion and coverage

After the first set passed, `scripts/convert_srd.py` added 21 more creature attack
profiles directly from the official source. The total is **32 executable notes**:
24 creature profiles, four spells and four potion strengths. The converter requires
complete consumption of every action clause. Recharge, knockdown riders,
conditional damage and unsupported passive traits stop that conversion.

The conversion scripts require Python 3 with PyYAML. They read the pinned
local SRD; they do not fetch or trust replacement source text.

```sh
python3 scripts/convert_srd.py --check
python3 scripts/test_convert_srd.py
```

The generated `builtin/dnd2024-mechanics/generated/coverage.json` accounts for all
1,452 source notes: 8 named source entries have the reviewed 11-definition slice,
21 have generated attack profiles, and 1,423 are outside this bounded execution profile. That last
count includes 364 full-page notes that cover classes, feats, equipment and other
rules beyond the named-entry index. It is a coverage ledger, not a claim that the
entire SRD is executable. No incomplete creature is accepted by dropping its
unconverted traits or action riders.

The [whole-SRD definition conversion](srd-definitions.md) is now complete and tracks engine gaps separately. Foundry's structured 2024 SRD was
examined as a secondary transcription aid: 338/339 spells, 333/336 creatures,
and 256/258 magic-item names have unique matching CC-BY records. Its activities
still leave some mechanics in prose and include source/version differences;
those records have not been copied into the built-in package or accepted as
verified execution definitions.

A targeting refinement from this proof is `area: {shape: sphere, radius_feet: 5}`
on the Acid Splash target selector. For such selectors, `range` applies to the
chosen origin. The caller supplies separate origin-range/clear-path facts and
complete area membership. A creature in the sphere can be beyond 60 feet from
the caster; it is not incorrectly discarded by a per-creature range test.

Observed seed-42 demonstration: the bear wins in round three at 3 HP; both
Skeletons reach zero. Two independent runs produce the same 37-event JSON trace.
This confirms repeatability for the pinned engine, content and fixed strategy;
it does not implement the designed general replay fingerprint format.
