---
splat_schema: 1
splat_id: 9c2f102d-04c3-55ac-a045-96c02796bf7e
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_extended_travel
splat_name: Extended Travel
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
---

# Extended Travel

## Source text

<!-- SRD page 192 -->

Characters can push themselves to travel for more\
than 8 hours per day, at the risk of tiring. At the end\
of each additional hour of travel beyond 8 hours,\
each character must succeed on a Constitution sav-\
ing throw or gain 1 Exhaustion level. The DC is 10\
plus 1 for each hour past 8 hours.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/travel_pace_extended_travel ` |
| ` implementation ` | ` dnd2024:rules_travel_pace_extended_travel ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` safe_daily_travel_hours ` | ` 8 ` |
| ` trigger ` | ` each_additional_hour_end ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc:
  op: add
  args:
  - 10
  - op: subtract
    args:
    - var: hours_traveled_today
    - 8
failure:
  exhaustion_gain: 1
```
