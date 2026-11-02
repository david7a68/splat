---
splat_schema: 1
splat_id: a7024fb9-92e0-5b68-87a8-2ff015f95749
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_armor
splat_name: Armor
splat_origin:
  key: dnd2024:reference/pages/204
splat_sources:
- title: SRD 5.2.1
  locator: pp. 204
  url: https://www.dndbeyond.com/srd
---

# Armor

## Source text

<!-- SRD page 204 -->

An item in the Armor category is typically a magical version of armor from “Equipment.” Unless an armor’s description notes otherwise, the armor must be worn for its magic to function.

Some suits of magic armor specify the type of armor they are, such as Chain Mail or Plate Armor. If no type is specified, choose the type or determine it randomly.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_categories_armor ` |
| ` implementation ` | ` dnd2024:rules_magic_item_categories_armor ` |

### Parameters

#### Magic armor

<!-- splat:yaml path=splat_rule/parameters/magic_armor -->

```yaml
worn_required_for_magic: true
description_can_override: true
unspecified_armor_type:
  choice_authority: gm
  methods:
  - choose
  - random
  options: armor_table_types
```
