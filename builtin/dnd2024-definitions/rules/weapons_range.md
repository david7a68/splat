---
splat_schema: 2
splat_id: ae8358e1-ebac-5cbf-8b1a-65380c39bfe5
splat_kind: rule
splat_key: dnd2024:rules/weapons_range
splat_name: Range
splat_origin: dnd2024:reference/pages/090
splat_sources:
- SRD 5.2.1, pp. 90, https://www.dndbeyond.com/srd
---

# Range

## Source text

<!-- SRD page 90 -->

A Range weapon has a range in parentheses after the Ammunition or Thrown property. The range lists two numbers. The first is the weapon’s normal range in feet, and the second is the weapon’s long range. When attacking a target beyond normal range, you have Disadvantage on the attack roll. You can’t attack a target beyond the long range.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_range ` |
| ` implementation ` | ` dnd2024:rules_weapons_range ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` range_unit ` | ` feet ` |
| ` first_number ` | ` normal_range ` |
| ` second_number ` | ` long_range ` |
| ` range_follows_property ` | ` ["ammunition", "thrown"] ` |
| ` beyond_normal ` | ` disadvantage ` |
| ` beyond_long ` | ` prohibited ` |
