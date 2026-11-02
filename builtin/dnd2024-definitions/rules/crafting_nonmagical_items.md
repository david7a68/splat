---
splat_schema: 1
splat_id: 10605318-c041-5207-be01-b99126d74e7c
splat_kind: rule
splat_key: dnd2024:rules/crafting_nonmagical_items
splat_name: Crafting Nonmagical Items
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
---

# Crafting Nonmagical Items

## Source text

<!-- SRD page 103 -->

To craft a nonmagical item, you need tools, raw materials, and time, each of which is detailed below. If you meet the requirements, you make the item, and you can use it or sell it at its normal price.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/crafting_nonmagical_items ` |
| ` implementation ` | ` dnd2024:rules_crafting_nonmagical_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requirements ` | ` ["tools", "raw_materials", "time"] ` |

#### Requirements satisfied

<!-- splat:values path=splat_rule/parameters/requirements_satisfied -->

| Field | Value |
| --- | --- |
| ` create_item ` | ` true ` |
| ` allow_use ` | ` true ` |
| ` sale_price ` | ` normal_item_price ` |

#### Requirement rules

<!-- splat:rows path=splat_rule/parameters/requirement_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/crafting_nonmagical_items_tools ` |
| ` 1 ` | ` dnd2024:rules/crafting_nonmagical_items_raw_materials ` |
| ` 2 ` | ` dnd2024:rules/crafting_nonmagical_items_time ` |
