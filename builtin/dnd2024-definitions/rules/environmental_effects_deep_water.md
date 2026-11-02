---
splat_schema: 1
splat_id: 7da65499-488c-5f4f-942a-103c2a1e1dcf
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_deep_water
splat_name: Deep Water
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
---

# Deep Water

## Source text

<!-- SRD page 195 -->

Swimming through deep water (more than 100 feet deep) presents additional challenges because of the water’s pressure and cold temperature. After each hour of swimming in deep water, a creature that lacks a Swim Speed must succeed on a DC 10 Constitution saving throw or gain 1 Exhaustion level.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_deep_water ` |
| ` implementation ` | ` dnd2024:rules_environmental_effects_deep_water ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` water_depth_ft_greater_than ` | ` 100 ` |
| ` subject_lacks_swim_speed ` | ` true ` |
| ` trigger ` | ` each_swimming_hour_end ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 10 ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` exhaustion_gain ` | ` 1 ` |
