---
splat_schema: 2
splat_id: f2c0f678-f96e-57f5-be17-644b3b43e49d
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_wands
splat_name: Wands
splat_origin: dnd2024:reference/pages/205
splat_sources:
- SRD 5.2.1, pp. 205, https://www.dndbeyond.com/srd
---

# Wands

## Source text

<!-- SRD page 205 -->

An item in the Wand category is typically 12 to 15 inches long and crafted of metal, bone, or wood. It is tipped with metal, crystal, stone, or some other material.

Unless its description notes otherwise, a wand can be used as an Arcane Focus.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_categories_wands ` |
| ` implementation ` | ` dnd2024:rules_magic_item_categories_wands ` |

### Parameters

#### Wand

<!-- splat:yaml path=splat_rule/parameters/wand -->

```yaml
typical_length_inches:
  minimum: 12
  maximum: 15
arcane_focus: true
description_can_override: true
```
