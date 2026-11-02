---
splat_schema: 1
splat_id: 650e5703-4479-5566-947b-0c3fb2b83fbf
splat_kind: rule
splat_key: dnd2024:rules/poison_midnight_tears_1_500_gp
splat_name: Midnight Tears (1,500 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197, 198
  url: https://www.dndbeyond.com/srd
---

# Midnight Tears (1,500 GP)

## Source text

<!-- SRD page 197 -->

Ingested Poison

A creature that ingests Midnight Tears suffers no effect until the stroke of midnight. Any effect that ends the Poisoned condition neutralizes this poison. If the poison hasn’t been neutralized before midnight, the creature makes a DC 17 Constitution saving throw, taking 31 (9d6) Poison damage on a failed save or half as much damage on a successful one.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_midnight_tears_1_500_gp ` |
| ` implementation ` | ` dnd2024:rule_poison_midnight_tears_1_500_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` price_gp_per_dose ` | ` 1500 ` |
| ` poison_type ` | ` ingested ` |
| ` activation_timing ` | ` next_midnight ` |
| ` before_activation_effect ` | ` none ` |
| ` neutralized_by ` | ` any_effect_ending_poisoned_condition ` |

#### Activation save

<!-- splat:yaml path=splat_rule/parameters/activation_save -->

```yaml
ability: constitution
dc: 17
failure:
  damage:
    dice:
      count: 9
      sides: 6
  average: 31
  type: poison
success:
  damage_fraction: 0.5
  rounding: floor
```
