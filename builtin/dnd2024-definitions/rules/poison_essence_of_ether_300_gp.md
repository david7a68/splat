---
splat_schema: 2
splat_id: ef56cdab-838b-584b-99e9-ef935b1d8021
splat_kind: rule
splat_key: dnd2024:rules/poison_essence_of_ether_300_gp
splat_name: Essence of Ether (300 GP)
splat_origin: dnd2024:reference/pages/197
splat_sources:
- SRD 5.2.1, pp. 197, https://www.dndbeyond.com/srd
---

# Essence of Ether (300 GP)

## Source text

<!-- SRD page 197 -->

Inhaled Poison

A creature subjected to Essence of Ether must succeed on a DC 15 Constitution saving throw or have the Poisoned condition for 8 hours. The creature also has the Unconscious condition while Poisoned in this way. The creature wakes up if it takes damage or if another creature takes an action to shake it awake.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_essence_of_ether_300_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_essence_of_ether_300_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` wake_triggers ` | ` ["damage_taken", "other_creature_action_shake_awake"] ` |
| ` wake_ends_poisoned ` | ` false ` |

#### Poison

<!-- splat:values path=splat_rule/parameters/poison -->

| Field | Value |
| --- | --- |
| ` type ` | ` inhaled ` |
| ` cost_gp ` | ` 300 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 15 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
condition:
  id: poisoned
  duration:
    value: 8
    unit: hours
unconscious_while_poisoned: true
```
