---
splat_schema: 1
splat_id: 8d9274a1-66f0-5917-9aab-95806a159fe3
splat_kind: rule
splat_key: dnd2024:rules/weapons_versatile
splat_name: Versatile
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
---

# Versatile

## Source text

<!-- SRD page 90 -->

A Versatile weapon can be used with one or two hands. A damage value in parentheses appears with the property. The weapon deals that damage when used with two hands to make a melee attack.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_versatile ` |
| ` implementation ` | ` dnd2024:rule_weapons_versatile ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` hands_options ` | ` [1, 2] ` |
| ` two_handed_melee_damage ` | ` weapon_versatile_damage ` |
| ` other_damage ` | ` weapon_base_damage ` |
