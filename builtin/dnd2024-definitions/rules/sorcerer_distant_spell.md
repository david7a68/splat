---
splat_schema: 1
splat_id: 72467b86-44e5-57b5-9aba-c403dfeab9b7
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_distant_spell
splat_name: Distant Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
---

# Distant Spell

## Source text

<!-- SRD page 67 -->

Cost: 1 Sorcery Point

When you cast a spell that has a range of at least 5 feet, you can spend 1 Sorcery Point to double the spell’s range. Or when you cast a spell that has a range of Touch, you can spend 1 Sorcery Point to make the spell’s range 30 feet.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_distant_spell ` |
| ` implementation ` | ` dnd2024:rules_sorcerer_distant_spell ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` spell.cast_pending ` |

#### Cost

<!-- splat:values path=splat_rule/parameters/cost -->

| Field | Value |
| --- | --- |
| ` resource ` | ` sorcery_points ` |
| ` amount ` | ` 1 ` |

#### Range branches

<!-- splat:table
path: splat_rule/parameters/range_branches
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Range feet minimum:
    path: range_feet_minimum
    type: integer
    optional: true
  Range multiplier:
    path: range_multiplier
    type: integer
    optional: true
  Range kind:
    path: range_kind
    type: symbol
    optional: true
  Replacement range feet:
    path: replacement_range_feet
    type: integer
    optional: true
-->

| Key | Range feet minimum | Range multiplier | Range kind | Replacement range feet |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 2 ` |  |  |
| ` row_2 ` |  |  | ` Touch ` | ` 30 ` |
