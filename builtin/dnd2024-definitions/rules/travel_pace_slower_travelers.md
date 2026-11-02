---
splat_schema: 1
splat_id: 423fdb62-5c87-56e0-82c1-77de312beed8
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_slower_travelers
splat_name: Slower Travelers
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
---

# Slower Travelers

## Source text

<!-- SRD page 192 -->

The group must move at a Slow pace if any group member’s Speed is reduced to half or less of normal.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/travel_pace_slower_travelers ` |
| ` implementation ` | ` dnd2024:rules_travel_pace_slower_travelers ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` group_pace ` | ` slow ` |

#### Predicate

<!-- splat:values path=splat_rule/parameters/predicate -->

| Field | Value |
| --- | --- |
| ` any_member_speed_ratio_lte ` | ` 0.5 ` |
