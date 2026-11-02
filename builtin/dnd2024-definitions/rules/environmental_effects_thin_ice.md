---
splat_schema: 1
splat_id: 47868ad9-a302-5fb2-aacc-8d29832f211d
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_thin_ice
splat_name: Thin Ice
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
---

# Thin Ice

## Source text

<!-- SRD page 195 -->

Thin ice has a weight tolerance of 3d10 × 10 pounds per 10-foot-square area. Whenever the total weight on an area of thin ice exceeds its tolerance, the ice in that area breaks. All creatures on broken ice fall through. Below the ice is frigid water (see “Frigid Water” above).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_thin_ice ` |
| ` implementation ` | ` dnd2024:rules_environmental_effects_thin_ice ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` break_when_weight ` | ` greater_than_tolerance ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` square ` |
| ` side_feet ` | ` 10 ` |

#### Weight tolerance lb

<!-- splat:yaml path=splat_rule/parameters/weight_tolerance_lb -->

```yaml
roll:
  count: 3
  sides: 10
  add: 0
multiplier: 10
```

#### Break effect

<!-- splat:yaml path=splat_rule/parameters/break_effect -->

```yaml
creatures: all_on_broken_ice
fall_into: frigid_water
water_rules:
  key: dnd2024:rules/environmental_effects_frigid_water
```
