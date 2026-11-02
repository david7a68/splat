---
splat_schema: 1
splat_id: 19858d37-f205-5771-83ad-26c554f3dd29
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_extreme_cold
splat_name: Extreme Cold
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
---

# Extreme Cold

## Source text

<!-- SRD page 195 -->

When the temperature is 0 degrees Fahrenheit or lower, a creature exposed to the extreme cold must succeed on a DC 10 Constitution saving throw at the end of each hour or gain 1 Exhaustion level. Creatures that have Resistance or Immunity to Cold damage automatically succeed on the save.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_extreme_cold ` |
| ` implementation ` | ` dnd2024:rule_environmental_effects_extreme_cold ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` temperature_fahrenheit_maximum ` | ` 0 ` |

#### Exposure save

<!-- splat:yaml path=splat_rule/parameters/exposure_save -->

```yaml
timing: end_of_each_hour
ability: constitution
dc: 10
failure:
  exhaustion_levels_add: 1
automatic_success_if_any:
- cold_resistance
- cold_immunity
```
