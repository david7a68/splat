---
splat_schema: 2
splat_id: 15acfb33-b514-5fd2-8f24-45652cf96fe8
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_good_roads
splat_name: Good Roads
splat_origin: dnd2024:reference/pages/192
splat_sources:
- SRD 5.2.1, pp. 192, https://www.dndbeyond.com/srd
---

# Good Roads

## Source text

<!-- SRD page 192 -->

The presence of a good road increases the group’s maximum pace by one step (from Slow to Normal or from Normal to Fast).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/travel_pace_good_roads ` |
| ` implementation ` | ` dnd2024:rule_travel_pace_good_roads ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` good_road_maximum_pace_step_increase ` | ` 1 ` |
| ` pace_order ` | ` ["slow", "normal", "fast"] ` |
| ` cap ` | ` fast ` |
