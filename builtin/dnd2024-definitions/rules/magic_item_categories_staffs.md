---
splat_schema: 1
splat_id: 3bb900e6-1df4-5633-b918-8ccf28dfdbea
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_staffs
splat_name: Staffs
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
---

# Staffs

## Source text

<!-- SRD page 205 -->

Items in the Staff category vary widely in appearance: some are of nearly equal diameter throughout and smooth, others are gnarled and twisted, some are made of wood, and others are composed of polished metal or crystal. A staff weighs between 2 and 7 pounds and serves well as a walking stick or cane.

Unless its description notes otherwise, a staff can be used as a nonmagical Quarterstaff and an Arcane Focus.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_categories_staffs ` |
| ` implementation ` | ` dnd2024:rule_magic_item_categories_staffs ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` category ` | ` staff ` |
| ` weight_pounds_range ` | ` [2, 7] ` |
| ` description_exception_overrides ` | ` true ` |

#### Default uses

<!-- splat:table
path: splat_rule/parameters/default_uses
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Weapon:
    path: weapon
    type: reference
    optional: true
  Magical weapon:
    path: magical_weapon
    type: boolean
    optional: true
  Focus:
    path: focus
    type: symbol
    optional: true
-->

| Key | Weapon | Magical weapon | Focus |
| --- | --- | --- | --- |
| ` row_1 ` | [Quarterstaff](<../equipment/quarterstaff.md> "dnd2024:equipment/quarterstaff") | false |  |
| ` row_2 ` |  |  | ` Arcane ` |
