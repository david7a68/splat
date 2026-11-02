---
splat_schema: 1
splat_id: 13adc30e-253e-5245-b9b9-33c8e2f929a4
splat_kind: rule
splat_key: dnd2024:rules/combat_range
splat_name: Range
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
---

# Range

## Source text

<!-- SRD page 15 -->

You can make ranged attacks only against targets within a specified range. If a ranged attack, such as one made with a spell, has a single range, you can’t attack a target beyond this range.

Some ranged attacks, such as those made with a Longbow, have two ranges. The smaller number is the normal range, and the larger number is the long range. Your attack roll has Disadvantage when your target is beyond normal range, and you can’t attack a target beyond long range.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_range ` |
| ` implementation ` | ` dnd2024:rules_combat_range ` |

### Parameters

#### Single range

<!-- splat:values path=splat_rule/parameters/single_range -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` listed_range ` |

#### Two ranges

<!-- splat:values path=splat_rule/parameters/two_ranges -->

| Field | Value |
| --- | --- |
| ` within_normal ` | ` normal_roll ` |
| ` beyond_normal_through_long ` | ` disadvantage ` |
| ` beyond_long ` | ` prohibited ` |
