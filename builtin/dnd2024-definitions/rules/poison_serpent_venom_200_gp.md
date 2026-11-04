---
splat_schema: 2
splat_id: 8ed010ee-3518-578a-a264-830b49c0f933
splat_kind: rule
splat_key: dnd2024:rules/poison_serpent_venom_200_gp
splat_name: Serpent Venom (200 GP)
splat_origin: dnd2024:reference/pages/198
splat_sources:
- SRD 5.2.1, pp. 198, https://www.dndbeyond.com/srd
---

# Serpent Venom (200 GP)

## Source text

<!-- SRD page 198 -->

Injury Poison

A creature subjected to Serpent Venom must succeed on a DC 11 Constitution saving throw, taking 10 (3d6) Poison damage on a failed save or half as much damage on a successful one.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_serpent_venom_200_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_serpent_venom_200_gp ` |

### Parameters

#### Poison

<!-- splat:values path=splat_rule/parameters/poison -->

| Field | Value |
| --- | --- |
| ` type ` | ` injury ` |
| ` cost_gp ` | ` 200 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 11 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
damage:
  dice:
    count: 3
    sides: 6
    add: 0
  average: 10
  type: poison
```

#### Success

<!-- splat:values path=splat_rule/parameters/success -->

| Field | Value |
| --- | --- |
| ` damage_multiplier ` | ` 0.5 ` |
| ` round ` | ` down ` |
