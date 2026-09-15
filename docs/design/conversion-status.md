# Definition conversion status fields

These fields extend schema 1 for the read-only SRD conversion catalogue. They
record unfinished authoring work; they are not runtime mechanics or scripts.
See [the current conversion report](../srd-definitions.md) for scope and counts.

`splat_conversion` is a closed mapping with:

| Field | Contract |
| --- | --- |
| `status` | `partial` or `specified`; neither permits execution |
| `category` | Source definition category |
| `source_path`, `source_sha256` | Pinned source file and SHA256 |
| `pages` | Nonempty SRD page list, 1–364 |
| `conversion_gaps` | Unresolved clause keys or explicit extraction diagnostics |
| `engine_gaps` | Required, unavailable registered mechanic names |
| `execution_profiles` | Keys of separately tested, bounded projections |
| `parent_name` | Optional source heading context, not an entry reference |
| `line_span` | Optional extraction inventory address: `first`, `count` |

`partial` requires at least one conversion gap. `specified` requires no conversion
gaps and at least one engine gap. The engine-gap set equals
`splat_requires_mechanics`. A future implemented definition must pass the real
compiler; it cannot obtain execution by changing this status.

`splat_clauses` is a nonempty list of `{key, text, sha256, status}`. Clause keys
are local and unique. Status is `needs_semantic_conversion` or
`contract_specified`. Each unresolved clause must occur in `conversion_gaps`.
The hash covers the UTF-8 text exactly. A specified note cannot contain an
unresolved clause. Clause prose is never evaluated, and a recorded contract
specification does not supply its implementation.

The initial converter retains one conservative `source_rules` obligation per
note. It does not claim to have resolved every sentence after extracting a spell
header, table column or attack prefix. More precise clause splitting can refine
these obligations without changing that boundary.

## Descriptive fields on drafts

Drafts retain the existing `splat_creature`, `splat_spell`, `splat_item`,
`splat_stats`, `splat_rule` and `splat_rule_group` shapes. Descriptive extensions
below are readable data; the proof compiler does not accept them as mechanics.

- Spell: `classes`, `casting_time`, `ritual`, `range`, and `material` retain the
  official header. Material has `description`, `consumed`, optional
  `minimum_value: {value, unit}`. Range is `{feet: integer}` or `{mode: string}`.
- Creature: `alignment`, `size_options_text`, `tags_text`, `movement_text`,
  `hit_points_text`, `skills_text`, `resistances_text`, `vulnerabilities_text`,
  `immunities_text`, `gear_text`, `senses_text`, `languages_text`, `cr_text`.
  Optional `ambiguous_saves` lists abilities with unresolved printed save signs.
- Item: `rarity`, `price`, `properties_text`, `mastery`, `weapon_category`,
  `damage: {expression, type}` and `armor` retain table values. Armor records
  `category`, `base_ac` or `ac_bonus`, `add_dexterity_modifier`, optional
  `dexterity_modifier_max`, `strength_minimum`, and `stealth_disadvantage`.
  Missing mass stays absent. It is not zero.
- Class progression: `splat_rule.parameters` contains `class` and 20 `rows`, each
  with `level`, `proficiency_bonus`, `features_text` and class-specific numeric or
  dice columns. `features_text` is not a resolved grant. `pact_slots` and
  `pact_slot_level` are separate from `spell_slots_1` through `spell_slots_9`.

The five reviewed specifications use named operations or traits with closed
parameter sets recorded in `scripts/srd_inputs/contracts.yaml`. They define
future adapters, including their source-specific timing and outcomes. General
source rules, such as spellcasting legality, still apply. No arbitrary formula,
foreign-system object path, macro or code is executed.
