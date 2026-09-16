# Imported character content

Imported characters are ordinary schema-1 creature entries. Their items and
spells are campaign-owned entries with normal UUID references. The catalogue,
content editor, dependency traversal, and encounter roster use the same reader
as authored content. Structural validity does not grant simulation support.

The private `EntitySnapshot` JSON remains a lossless provider cache, not a second
simulation format. Both fresh responses and old version-1 caches go through
`content_notes()` when published. Campaign Markdown never embeds that snapshot.
The provider cache retains choices, custom fields, and source data for future
conversion; incomplete semantics are reported rather than inferred from prose.

## Character fields

Existing fields retain their contracts: `splat_stats` contains supplied totals,
`splat_health` selects character zero-HP policy, `splat_resources` contains limits
and known rest recovery, and `splat_initial` holds fetched current amounts.
`splat_proficiencies` contains training and skill proficiency; languages use
`splat_languages`. Multiclass hit dice use `splat_hit_dice.classes`, mapping a
local class key to `{count, die}`. Starting `hit_dice_spent` uses the same keys.
The original single-pool `{count, die}` shape remains valid for authored creatures.

The typed `splat_character` extension contains:

- `level`, `species`, and `classes` (`name`, `level`, optional `subclass`).
- `checks`: names, existing stat keys, proficiency/expertise/half-proficiency,
  and supplied advantage/disadvantage flags.
- `senses`: names and supplied distance text; `defenses`: resistance, immunity,
  and vulnerability names.
- `casting`: supplied ability modifiers, save DCs and attack bonuses, with the
  source class/feature names associated with each total.
- `attacks`: stable local keys for supplied attack profiles, including attack
  bonus, dice/fixed damage, damage type, range and unresolved-save indication.
  Duplicate source rows remain separate. Profile data is not an ability use.
- `features`: named descriptions, known activation, optional resource-pool key,
  and source identity. Descriptions do not implement rules.
- `spells`: normal spell entry references, preparation flags, slot requirement,
  casting ability, supplied override DC, cast level, and restriction text.

A weapon attack with sufficient data also produces `splat_actions` with normal
activation, targets, rolls and attack steps. Its conversion gaps still cover
equipment and feature interactions. Spell attack totals never create free
weapon actions: casting costs and effects require separate rule conversion.

`content::character::Sheet` is the strict serialized contract. The entry reader
checks its shape, local keys, stat/resource references and positive levels.
Normal reference validation resolves all spell and inventory dependencies.

## Inventory and dependencies

One marked Markdown inventory table is authoritative for starting placements.
Contained items use `<!-- splat:inventory container=item_SOURCE_ID -->` tables.
Each row retains quantity and attunement. `splat_inventory_details` maps row keys
to stable `instance_id`, provider `source_id`, `source_equipped`, and nullable
`charges_used`. An unknown charge maximum does not become an invented pool.
Beyond's equipped flag does not establish hand allocation; ambiguous items are
carried with an explicit gap. Known equipped body armor is worn.

Item entries contain category, known mass, equipment facts, described properties,
known armor/weapon values and containment facts. The import extends `splat_item`
with descriptive `properties` (`name`, `description`), `armor` (`value`,
`contribution: base|bonus`, optional strength requirement), `weapon` (dice,
modifier/fixed damage, type and ranges), and containment `capacity_weight_lb`
and `description`. These are supplied facts, not granted effects or rebuild rules.

Spell entries contain level, school, components/material text, concentration,
ritual, duration, casting-time amount/unit, range and readable description. A
referenced spell does not imply that its action is implemented.

Generated dependency identities are scoped to the character and source definition.
This avoids conflicting versions of a shared provider item across independently
refreshed characters. Repeated instances of the same definition within a sheet
share a definition but retain separate inventory row/instance identities.
Definitions live under `Characters/D&D Beyond/Definitions/<character UUID>/`.

## Coverage and publication

`splat_import: {version: 1, gaps: [{path, reason}]}` records incomplete conversion
at specific fields. It is valid on creatures, spells and items, and is separate
from the SRD catalogue's source-specific `splat_conversion` contract.
Unknown import versions or malformed gap records fail parsing. Import metadata
cannot enable execution; the current proof compiler rejects it.

All generated entries are parsed and their references validated together before
publication. Every existing file's generated checksum is checked before any
replacement. Dependencies publish before the creature, with atomic replacement
per file. Personal notes survive refresh, and edited generated content blocks the
whole character's publication. A disk failure or an editor racing the writes can
still leave some updated dependencies; this is not a multi-file transaction.
Old unreferenced generated definitions remain as historical content.

Loading an old private cache automatically creates the new content on its next
publication. The offline export example also performs this migration without
fetching source data. Existing creature UUIDs and note paths stay unchanged.
