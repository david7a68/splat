---
splat_schema: 1
splat_id: 1d55496c-347e-5683-9e18-a8d1419b207f
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_weapons
splat_name: Weapons
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
---

# Weapons

## Source text

<!-- SRD page 205 -->

A magic weapon is typically a magical version of a weapon from “Equipment.” Some magic weapons specify the type of weapon they are in their descriptions, such as a Longsword or Longbow. If no weapon type is specified, you may choose the type or determine it randomly.

Ammunition. If a magic weapon has the Ammunition property, ammunition fired from it is considered magical for the purpose of any rule that cares whether a weapon is magical or not.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_categories_weapons ` |
| ` implementation ` | ` dnd2024:rules_magic_item_categories_weapons ` |

### Parameters

#### Weapon type

<!-- splat:values path=splat_rule/parameters/weapon_type -->

| Field | Value |
| --- | --- |
| ` if_specified ` | ` item_definition ` |
| ` otherwise ` | ` choose_or_randomize_equipment_weapon_type ` |

#### Magic weapon with ammunition property

<!-- splat:values path=splat_rule/parameters/magic_weapon_with_ammunition_property -->

| Field | Value |
| --- | --- |
| ` fired_ammunition_counts_as_magical_weapon ` | ` true ` |
