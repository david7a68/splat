# Splat data design

> Character storage update: the [implemented character cache](character-cache.md)
> supersedes earlier memory-only restrictions for D&D Beyond character records.
> Saved manual DM runs supersede the earlier in-memory-only run boundary.
> Automatic combat resolution remains future work.

The candidate schema for the agreed scope is complete and ready for review.
The design is committed. A [read-only verifier and SRD source import](../srd-import.md)
now exercise part of the format. A [bounded execution proof](../execution-proof.md) now runs converted SRD mechanics.
The [whole-SRD catalogue](../srd-definitions.md) now contains typed rule specifications with explicit engine gaps.
The content editor, campaign content loader, and basic encounter roster editor are implemented.
The encounter editor saves local templates with direct creature references, starting HP and overrides, and a DM can prepare encounter groups to add during a run;
manual DM runs now save independent state, initiative, HP, conditions, and a combat log
in the configurable campaign run folder. Campaign-selected NPC initiative is implemented.
Bidirectional combat sync runs in the background from the D&D Beyond connection, in Party and for
linked run participants. An encounter can turn it off with `combat_sync: false`.
Shared party bindings, automatic combat resolution, and the full simulator remain later work.

Read these current contracts in order:

1. [Schema overview and first worked slice](schema-draft.md).
2. [Mechanics](mechanics-schema.md): typed expressions, operations, events,
   budgets, effects, casting, mastery and source-specific outcomes.
3. [State](state-schema.md): health, items, relationships, forms and lifecycle.
4. [Campaign and storage](campaign-schema.md): owners, party bindings, phases,
   strategies, sequences, fingerprints and edit recovery.
5. [Official action-resolution rules](action-resolution.md): rule authority,
   timing, expenditure and the user's two scope/ruling decisions.

Review [the complete notes](examples/README.md), [the encounter trace](encounter-trace.md),
[the schema checks](schema-checks.md), and [coverage of all 34 families](schema-coverage.md).

## Field index

All notes require `splat_schema`, `splat_id`, `splat_kind`, `splat_name`; optional
common fields are `splat_key`, `splat_sources`, `splat_origin`, `splat_requires`,
`splat_optional`, and `splat_requires_mechanics`. Required references must resolve
to enabled content in the permitted ownership scope. Unknown engine contracts
block the affected use even if the note parses.

| Kind | Required kind-specific data | Optional supported groups |
| --- | --- | --- |
| package | `splat_namespace` | `splat_placement` |
| campaign | `splat_namespace` | `splat_packages`, `splat_rule_choices`, `splat_party`, `splat_placement` |
| creature | `splat_creature`, stats required by its selected mechanics | `splat_stats`, `splat_proficiencies`, `splat_hit_dice`, `splat_senses`, `splat_languages`, `splat_health`, `splat_initial`, `splat_resources`, `splat_actions`, `splat_effects`, `splat_budgets`, `splat_limits`, `splat_traits`, `splat_forms`, `splat_relationships`, `splat_attack_options`, `splat_inventory_details`, marked inventory tables |
| item | `splat_item` definition | Common mechanics groups above where relevant, plus `splat_parameters` |
| spell | `splat_spell`, `splat_actions` | `splat_parameters`, local effects/rolls/resources as declared |
| effect | `splat_effect` | `splat_parameters` |
| container | `splat_item` EntryRef | `splat_initial`, `splat_inventory_details`, marked inventory tables |
| encounter | `splat_encounter` | All encounter fields defined in Campaign schema |
| encounter_group | `splat_encounter_group` | `side`, ordered `members`, with their counts and overrides |
| sequence | `splat_sequence` | Explicit encounter bindings, elapsed/rest steps |
| rule | `splat_rule` | Source attribution and typed implementation parameters |
| rule_group | `splat_rule_group` | Required/optional dependency metadata |
| strategy | `splat_strategy` | Registered policy parameters and declared priorities |
| seed | `splat_seed` | Common metadata; no runtime state or log |
| reference | `splat_reference`, with `executable: false` | Source excerpts; see [import contract](../srd-import.md) |

Optional does not mean unnecessary to execute: a creature without a required
save stat is an editable draft but cannot resolve that save. Draft validity,
reference validity and simulation capability are separate checks.

## Earlier decision records

[Packages](packages.md), [combat](combat.md), [storage](storage.md),
[containers](containers.md), and [the original creature example](creature-example.md)
retain the interview decisions and their earlier illustrative fields. Their
old statements that a schema detail is still open are superseded by the current
contracts above. Agreed scope and user preferences remain in force.

## What is deliberately outside this schema version

Maps/geometry/pathfinding; persisted runs/results/logs; package publishing,
distribution or updates; arbitrary package scripts; automatic interpretation
of readied spells; and a complete encoded catalogue of every published option.
The schema retains unsupported-rule diagnostics rather than silently omitting
mechanics. The Hydra capacity ruling and manual readied-spell tracking remain.

Next work after review is implementation and source-content validation. This
document does not authorize that work. D&D Beyond character access still needs
a verified live adapter and remains subject to the already agreed live-only
data boundary.

The [conversion status contract](conversion-status.md) and
[whole-SRD catalogue report](../srd-definitions.md) record the later authorized
conversion work. They distinguish incomplete conversion from unavailable engine
support; neither is treated as an executable rule.

[Markdown layout](markdown-layout.md) defines editable tables and local YAML blocks,
with identity in frontmatter and generated verification records in package reports.
