---
splat_schema: 1
splat_id: 3f186276-b389-504d-ac9e-2de7ad63233f
splat_kind: rule
splat_key: dnd2024:glossary/dehydration_hazard
splat_name: Dehydration [Hazard]
splat_origin:
  key: dnd2024:reference/glossary/dehydration_hazard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
---

# Dehydration [Hazard]

## Source text

<!-- SRD page 181 -->

A creature requires an amount of water per day\
based on its size, as shown in the Water Needs per\
Day table. A creature that drinks less than half the\
required water for a day gains 1 Exhaustion level\
at the day’s end. Exhaustion caused by dehydra-\
tion can’t be removed until the creature drinks the\
full amount of water required for a day. See also\
“Exhaustion.”\
Water Needs per Day\
Size Water Size Water\
Tiny 1/4 gallon Large 4 gallons\
Small 1 gallon Huge 16 gallons\
Medium 1 gallon Gargantuan 64 gallons\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/dehydration_hazard ` |
| ` implementation ` | ` dnd2024:glossary_dehydration_hazard ` |

### Parameters

#### Daily water gallons

<!-- splat:values path=splat_rule/parameters/daily_water_gallons -->

| Field | Value |
| --- | --- |
| ` tiny ` | ` 0.25 ` |
| ` small ` | ` 1 ` |
| ` medium ` | ` 1 ` |
| ` large ` | ` 4 ` |
| ` huge ` | ` 16 ` |
| ` gargantuan ` | ` 64 ` |

#### Exhaustion

<!-- splat:yaml path=splat_rule/parameters/exhaustion -->

```yaml
trigger: day_end
predicate:
  op: less_than
  args:
  - var: water_consumed
  - op: multiply
    args:
    - 0.5
    - var: daily_requirement
levels: 1
cause: dehydration
```

#### Cause exhaustion removal locked until

<!-- splat:values path=splat_rule/parameters/cause_exhaustion_removal_locked_until -->

| Field | Value |
| --- | --- |
| ` water_consumed_gallons_at_least ` | ` full_daily_requirement ` |
