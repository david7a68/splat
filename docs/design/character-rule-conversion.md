# Character rule conversion

The importer now converts the 119 source definitions present in the three cached
party sheets. This covers features, actions, spells, and items. Conversion means
that the source behavior has typed data and references. It does not mean that an
engine adapter exists or that a participant has supplied every required choice.

## Source evidence and matching

`builtin/dndbeyond/conversions.json` maps provider definition identities to either
an existing converted SRD rule or authored typed parameters for that exact source.
The mapping is explicit. Names are not used to choose a rule or an edition.

Each mapping pins SHA-256 of canonical JSON containing these provider fields:
`id`, `entityTypeId`, `definitionKey`, `name`, `description`, `snippet`, `sources`,
`isLegacy`, `level`, `damage`, `damageType`, `properties`, `range`, `longRange`,
`armorClass`, `armorTypeId`, `components`, `duration`, and `activation`.
Absent values are null; object keys are sorted. In particular, `snippet` is
source evidence, not just display text: Circle Magic puts its general casting
and contribution rules there. Character names, accounts, inventory instance
identities, and full provider responses are not in this registry.

A new definition or changed source keeps its conversion gap. The previous mapping
is not applied on ID or name alone. Tests cover both source drift and changes to
the converted mechanics. To extend coverage, inspect both description and snippet,
author or select the exact rule, and update the source fingerprint. Do not update
a fingerprint merely to make a mismatch pass.

## Coverage

The source-matched 2024 spells, equipment, Wizard features, Barbarian features,
and weapon mastery rules reference the existing typed SRD catalogue directly,
using its canonical UUIDs. Their transitive dependencies remain in that shared
catalogue. No copied SRD mechanics, spell headers, item definitions, or private
SRD UUIDs are emitted. Repeated grants and different characters therefore share
one definition. The campaign dependency resolver brings in required shared
entries without requiring selection of the whole package. Non-SRD conversions
continue to use character-scoped local entries.

Additional typed contracts cover:

- Witch Bolt and Mind Sliver (2024), and Cause Fear (2017).
- The 2014 Sorcerer and Draconic Bloodline features, with source class progression.
- Goblin features, species choices, lineage grants, rest behavior, and Tough.
- Circle Magic initiation and all six enhancement options. The contract includes
  primary/secondary caster requirements, contribution timing and range, ongoing
  Concentration, slot costs, failure outcomes, and option-specific limits.
- Legacy equipment rules that differ from 2024 equipment. For example, the old
  Tinderbox takes an Action to light exposed fuel; the revised version takes a
  Bonus Action. The old Rope's burst check is Strength DC 17 and its HP is 2;
  the revised Rope has different checks and binding behavior.
- Character-creation grants and optional campaign-system prompts. Their 95
  selectable variants now have a separate source-pinned option catalogue.
  See [optional character systems](../research/character-campaign-options.md)
  for the primary sources and the state contract below.

Rage recovery overrides the provider's coarse reset category with the actual
2024 contract: restore one use on a Short Rest and all uses on a Long Rest.
Resource bindings identify the actor's actual pool for each mapped feature.
Spell bindings identify the source class/ability, supplied DC/attack values,
available slot pools, and at-will/ritual status. Class spells use that class's
computed totals, including supplied bonuses; species spells keep their separate
casting ability. An attack profile never bypasses spell casting costs.

## Data and engine boundaries

`splat_character.rules` maps source feature/action keys to normal entry references.
`resource_bindings` associates these rule entries with imported pool keys.
`choices` preserves selected option IDs, labels, and source/component identity. Unset choices
and choices whose labels are unavailable remain explicit. Future class choices
are retained but do not create a current missing-input warning.
`class_progression` preserves the provider's level-indexed spell tables and
casting ability. `contributions` records source modifier facts and restrictions.

`splat_attack_bindings` links supplied attack profiles to their item, Unarmed
Strike, or spell rules. Duplicate carried weapons remain separate instances;
using an item requires selecting an instance and establishing equipment state.
The provider's equipped flag does not establish which hands hold a weapon.

`splat_stat_policy` defines the supplied-baseline contract. Recorded source
contributions must not be added again. If an included source changes, a fresh
provider total or a separately validated rebuild is required. Dynamic effects
may apply only changes not already included in the baseline. This closes the
snapshot representation contract without inventing an unknown base calculation.

Import metadata version 2 separates:

- `gaps`: missing or unmatched rule conversion.
- `engine_gaps`: required engine adapter names. `content::adapters` says which are implemented.
- `required_inputs`: unset source selections or encounter-specific facts.
- `rule_fingerprint`: a fingerprint of the typed rule/action/trait/effect fields,
  parameters, contracts, character bindings, stats, resources, spell/item rules,
  attack bindings, stat policy and required mechanics. Changing those fields
  invalidates the old coverage declaration.

Version-1 metadata remains readable. Neither version grants execution. Native
source-specific parameters remain data; they do not register scripts or engine
implementations. The proof executor continues to reject imported records.

## Optional character systems

`builtin/dndbeyond/campaign-options.json` contains 19 Dark Bargain variants,
28 Character Thread milestone variants, and 48 Runestone power variants.
Each maps the exact option ID and SHA-256 of canonical JSON over `id`, `label`,
`description`, and `sourceId` to typed rule parameters. A choice must advertise
the selected ID in `optionIds`, and the option must belong to the right system.
The shared provider choice catalogue is scoped by `{componentTypeId}-{type}`.

`splat_character.campaign_options` separates four states:

- `inactive`: the sheet does not grant this optional system. No effect is imported.
- `unselected`: the system exists, but its source choices are explicitly null.
  No effect is imported. This does not assert that the GM disabled the system.
- `active`: named selected options, source choice/option IDs, and normal rule
  references. Both the benefit and cost belong to this selected rule.
- `unknown`: the parent rule or source selection data cannot be established.
  A required input explains what is missing; no optional effect is inferred.

Selections follow the D&D Beyond sheet on refresh. Clearing a selection removes
its grant reference. Old unreferenced generated definitions may remain on disk,
as with other imports, but are not active character grants. Unknown or changed
selected options retain their identity and a conversion gap, with no verified
rule reference. Referenced SRD spell and creature rules stay in the shared catalogue, including
their transitive dependencies. No random Runestone draw occurs during import.

Thread stages require earned milestones and any reward choices. They do not
advance automatically with character level. Cursemarked replaces earlier
milestone benefits while retaining access to the specified earlier sacrifice
alternatives. Rune overrides replace earlier values: for example Thurisaz uses
one damage die at its selected tier, not all earlier dice. Once per day stays
distinct from rest recovery. Patron powers retain their proximity restrictions.

Source contradictions and omissions remain explicit inputs for active options:
the Collector of Stories (Charisma) casting ability, Bloodsworn Wrath's version and slot
cost, the Cursemarked trigger result, Laukaz's unspecified action costs, and
Sowilo's unspecified patron beam range. Required secondary selections are not
invented. Unselected catalogue options do not impose these requirements on a
character. Supplied stats remain baselines; optional grants never add their
recorded modifiers to those totals a second time.

The source names Wrathful Smite without identifying its edition, and the spell
is outside the SRD catalogue. Bloodsworn variants containing the optional Wrath
reward keep a conditional conversion gap until its intended version and rule
are established. A placeholder spell name never becomes a free or verified cast.

## Character check

D&D Beyond stays the source of truth for an imported character. The check compares a
sheet with what the SRD rules give for its classes, levels and choices. It reports
differences and changes nothing. It reads the provider cache and the published sheet,
so it adds no saved fields. It appears in Party under **Source & parity**, and this
command runs it on a cache folder without writing. It fails on a difference or a choice the
rules do not allow. An unset choice is reported, but it is an incomplete sheet, not a failure:

```sh
devenv shell -- cargo run --locked --bin verify-content -- --check-characters CACHE_DIRECTORY
```

[`character::build`](../../src/character/build.rs) knows only the catalogue. It gives
the features, capacities, class values, spell slots and choices for a class, a level
and a subclass. Multiclass characters use their total level for the proficiency bonus
and the Multiclass Spellcaster table for slots. Pact Magic stays separate.

[`character::check`](../../src/character/check.rs) compares that result with the sheet:

| Finding | Meaning |
| --- | --- |
| Differs | D&D Beyond and the SRD rules give different results |
| Missing choice | The rules ask for a decision the sheet does not have |
| Choice not allowed | The sheet's decision is one the rules do not allow |
| Not compared | Splat has no reviewed link, so it cannot say |

`builtin/dndbeyond/class-links.json` holds the class links. Each class, feature, option and
subclass pins a SHA-256 of its provider source. If that source changes, the link stops applying
and the item shows as not compared, and no feature, saving throw or choice proved by that source
counts as a match. Numbers from a linked class's tables, such as spell slots and cantrips, are
compared directly, so a changed feature description does not stop those comparisons.

The links now cover the 2024 Barbarian and Wizard, because those are the SRD classes the
cached party has. The other ten classes work in `character::build`, but their D&D Beyond IDs
are not known yet, so those characters show as not compared. No cached character has passed
level 1, so subclass and feat option IDs are also unknown, and the shape of D&D Beyond's feat
rows at higher levels has not been seen. Spell slots are not compared for a multiclass
character, because the importer does not create pools for one.

To link another class, cache a character of that class, compare its source with the SRD,
add its D&D Beyond ID to `CLASSES` in `scripts/dndbeyond_class_links.py`, and run the script
on the cache folder. That allowlist is the review of the edition. Within a listed class the
script proposes each feature's rule from a source-matched conversion, or else from its name
and level, and prints a similarity score for each linked feature. Read the low scores before
you commit. Wizard Subclass scores low because the SRD text also has the spellbook sidebar.
Subclass and feat links are added by hand, and a rerun keeps them. Weapons that are not in the
SRD, such as firearms, stay unlinked on purpose. Species, background and class tool choices are
not checked yet.
