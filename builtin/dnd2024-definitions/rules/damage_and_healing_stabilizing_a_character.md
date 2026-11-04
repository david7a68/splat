---
splat_schema: 2
splat_id: e82ba8ae-7528-581a-8985-ece751b33ea3
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_stabilizing_a_character
splat_name: Stabilizing a Character
splat_origin: dnd2024:reference/pages/018
splat_sources:
- SRD 5.2.1, pp. 18, https://www.dndbeyond.com/srd
---

# Stabilizing a Character

## Source text

<!-- SRD page 18 -->

You can take the Help action to try to stabilize a creature with 0 Hit Points, which requires a successful DC 10 Wisdom (Medicine) check.

A Stable creature doesn’t make Death Saving Throws even though it has 0 Hit Points, but it still has the Unconscious condition. If the creature takes damage, it stops being Stable and starts making Death Saving Throws again. A Stable creature that isn’t healed regains 1 Hit Point after 1d4 hours.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_stabilizing_a_character ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_stabilizing_a_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` help ` |
| ` requires_target_hp ` | ` 0 ` |

#### Check

<!-- splat:values path=splat_rule/parameters/check -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` medicine ` |
| ` dc ` | ` 10 ` |

#### Success

<!-- splat:values path=splat_rule/parameters/success -->

| Field | Value |
| --- | --- |
| ` stable ` | ` true ` |
| ` death_saves ` | ` false ` |
| ` unconscious ` | ` true ` |
| ` hp ` | ` 0 ` |

#### Stable damage

<!-- splat:values path=splat_rule/parameters/stable_damage -->

| Field | Value |
| --- | --- |
| ` stable ` | ` false ` |
| ` resume_death_saves ` | ` true ` |

#### Stable without healing

<!-- splat:yaml path=splat_rule/parameters/stable_without_healing -->

```yaml
after:
  dice:
    count: 1
    sides: 4
  unit: hour
regain_hp: 1
```
