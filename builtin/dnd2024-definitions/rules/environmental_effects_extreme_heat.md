---
splat_schema: 1
splat_id: b6abf51d-2706-5b2d-a211-7ce1d3b86d16
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_extreme_heat
splat_name: Extreme Heat
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
---

# Extreme Heat

## Source text

<!-- SRD page 195 -->

When the temperature is 100 degrees Fahrenheit or higher, a creature exposed to the extreme heat and without access to drinkable water must succeed on a Constitution saving throw at the end of each hour or gain 1 Exhaustion level. The DC is 5 for the first hour and increases by 1 for each additional hour. Creatures wearing Medium or Heavy armor have Disadvantage on the save. Creatures that have Resistance or Immunity to Fire damage automatically succeed on the save.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_extreme_heat ` |
| ` implementation ` | ` dnd2024:rules_environmental_effects_extreme_heat ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` temperature_f_minimum ` | ` 100 ` |
| ` requires_no_drinkable_water ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
event: exposure_hour.ended
dc:
  initial: 5
  increment_per_additional_hour: 1
disadvantage_if_armor:
- medium
- heavy
automatic_success_if_any:
- fire_resistance
- fire_immunity
```

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` exhaustion_levels ` | ` 1 ` |
