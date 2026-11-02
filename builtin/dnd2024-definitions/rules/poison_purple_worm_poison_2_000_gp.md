---
splat_schema: 1
splat_id: 372f2450-098a-5935-9361-2d3cfa8f2f80
splat_kind: rule
splat_key: dnd2024:rules/poison_purple_worm_poison_2_000_gp
splat_name: Purple Worm Poison (2,000 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
---

# Purple Worm Poison (2,000 GP)

## Source text

<!-- SRD page 198 -->

Injury Poison

A creature subjected to Purple Worm Poison makes a DC 21 Constitution saving throw, taking 35 (10d6) Poison damage on a failed save or half as much damage on a successful one.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_purple_worm_poison_2_000_gp ` |
| ` implementation ` | ` dnd2024:rule_poison_purple_worm_poison_2_000_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` price_gp_per_dose ` | ` 2000 ` |
| ` poison_type ` | ` injury ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc: 21
failure:
  damage:
    dice:
      count: 10
      sides: 6
  average: 35
  type: poison
success:
  damage_fraction: 0.5
  rounding: floor
```
