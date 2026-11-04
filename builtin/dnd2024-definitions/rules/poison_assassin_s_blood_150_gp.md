---
splat_schema: 2
splat_id: c338b33d-31da-5fb5-ac44-6a7c66b3718e
splat_kind: rule
splat_key: dnd2024:rules/poison_assassin_s_blood_150_gp
splat_name: Assassin’s Blood (150 GP)
splat_origin: dnd2024:reference/pages/197
splat_sources:
- SRD 5.2.1, pp. 197, https://www.dndbeyond.com/srd
---

# Assassin’s Blood (150 GP)

## Source text

<!-- SRD page 197 -->

Ingested Poison

A creature subjected to Assassin’s Blood makes a DC 10 Constitution saving throw. On a failed save, the creature takes 6 (1d12) Poison damage and has the Poisoned condition for 24 hours. On a successful save, the creature takes half as much damage only.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_assassin_s_blood_150_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_assassin_s_blood_150_gp ` |

### Parameters

#### Poison

<!-- splat:values path=splat_rule/parameters/poison -->

| Field | Value |
| --- | --- |
| ` type ` | ` ingested ` |
| ` cost_gp ` | ` 150 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 10 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
damage:
  dice:
    count: 1
    sides: 12
    add: 0
  average: 6
  type: poison
condition:
  id: poisoned
  duration:
    value: 24
    unit: hours
```

#### Success

<!-- splat:values path=splat_rule/parameters/success -->

| Field | Value |
| --- | --- |
| ` damage_multiplier ` | ` 0.5 ` |
| ` round ` | ` down ` |
| ` condition ` | ` null ` |
