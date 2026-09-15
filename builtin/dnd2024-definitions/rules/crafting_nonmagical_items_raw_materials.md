---
splat_schema: 1
splat_id: e04d38f4-48df-5dcf-85b2-3f43dec3a84f
splat_kind: rule
splat_key: dnd2024:rules/crafting_nonmagical_items_raw_materials
splat_name: Raw Materials
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
---

# Raw Materials

## Source text

<!-- SRD page 103 -->

To make an item, you need raw materials worth\
half its purchase cost (round down). For example,\
you need 750 GP of raw materials to make Plate\
Armor, which sells for 1,500 GP. The GM determines\
whether appropriate raw materials are available.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/crafting_nonmagical_items_raw_materials ` |
| ` implementation ` | ` dnd2024:rules_crafting_nonmagical_items_raw_materials ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` appropriate_material_availability_authority ` | ` gm ` |

#### Raw material cost

<!-- splat:yaml path=splat_rule/parameters/raw_material_cost -->

```yaml
op: floor
value:
  op: multiply
  left: purchase_cost
  right: 0.5
```
