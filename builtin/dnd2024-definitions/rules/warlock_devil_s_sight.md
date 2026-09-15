---
splat_schema: 1
splat_id: 412dad3e-1aae-5ba0-8a41-89a807f38d4f
splat_kind: rule
splat_key: dnd2024:rules/warlock_devil_s_sight
splat_name: Devil’s Sight
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
---

# Devil’s Sight

## Source text

<!-- SRD page 72 -->

Prerequisite: Level 2+ Warlock\
You can see normally in Dim Light and Darkness—\
both magical and nonmagical—within 120 feet of\
yourself.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_devil_s_sight ` |
| ` implementation ` | ` dnd2024:rules_warlock_devil_s_sight ` |

### Parameters

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 2 ` |

#### Vision

<!-- splat:values path=splat_rule/parameters/vision -->

| Field | Value |
| --- | --- |
| ` range_feet ` | ` 120 ` |
| ` normal_in ` | ` ["dim_light", "darkness"] ` |
| ` magical ` | ` true ` |
| ` nonmagical ` | ` true ` |
