---
splat_schema: 1
splat_id: 13727e86-eb5f-51e5-8cb5-68e0d06f0bbf
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_rods
splat_name: Rods
splat_origin:
  key: dnd2024:reference/pages/204
splat_sources:
- title: SRD 5.2.1
  locator: pp. 204
  url: https://www.dndbeyond.com/srd
---

# Rods

## Source text

<!-- SRD page 204 -->

An item in the Rod category is a scepter usually made of metal, wood, or bone. A typical rod weighs 2 to 5 pounds.

Unless its description notes otherwise, a rod can be used as an Arcane Focus.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_item_categories_rods ` |
| ` implementation ` | ` dnd2024:rules_magic_item_categories_rods ` |

### Parameters

#### Rod

<!-- splat:yaml path=splat_rule/parameters/rod -->

```yaml
typical_weight_lb:
  minimum: 2
  maximum: 5
arcane_focus: true
description_can_override: true
```
