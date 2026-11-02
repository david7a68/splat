---
splat_schema: 1
splat_id: f1f9e408-ca42-5199-9419-f34758028cc9
splat_kind: rule
splat_key: dnd2024:rules/poison_torpor_600_gp
splat_name: Torpor (600 GP)
splat_origin:
  key: dnd2024:reference/pages/198
splat_sources:
- title: SRD 5.2.1
  locator: pp. 198
  url: https://www.dndbeyond.com/srd
---

# Torpor (600 GP)

## Source text

<!-- SRD page 198 -->

Ingested Poison

A creature subjected to Torpor poison must succeed on a DC 15 Constitution saving throw or have the Poisoned condition for 4d6 hours. The creature’s Speed is halved while the creature is Poisoned in this way.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_torpor_600_gp ` |
| ` implementation ` | ` dnd2024:rule_poison_torpor_600_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` price_gp_per_dose ` | ` 600 ` |
| ` poison_type ` | ` ingested ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc: 15
failure:
  condition: poisoned
  duration:
    unit: hours
    amount:
      dice:
        count: 4
        sides: 6
```

#### While poisoned by this effect

<!-- splat:values path=splat_rule/parameters/while_poisoned_by_this_effect -->

| Field | Value |
| --- | --- |
| ` speed_multiplier ` | ` 0.5 ` |
