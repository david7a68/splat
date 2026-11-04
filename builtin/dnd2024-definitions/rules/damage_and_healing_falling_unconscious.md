---
splat_schema: 2
splat_id: 79005fc2-99e2-5b05-8fcb-233c8f38db21
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_falling_unconscious
splat_name: Falling Unconscious
splat_origin: dnd2024:reference/pages/017
splat_sources:
- SRD 5.2.1, pp. 17, https://www.dndbeyond.com/srd
---

# Falling Unconscious

## Source text

<!-- SRD page 17 -->

If you reach 0 Hit Points and don’t die instantly, you have the Unconscious condition (see “Rules Glossary”) until you regain any Hit Points, and you now face making Death Saving Throws (see below).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_falling_unconscious ` |
| ` implementation ` | ` dnd2024:rule_damage_and_healing_falling_unconscious ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` unconscious_until ` | ` any_hit_points_regained ` |

#### Apply definitions

<!-- splat:rows path=splat_rule/parameters/apply_definitions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/unconscious_condition ` |
| ` 1 ` | ` dnd2024:glossary/death_saving_throw ` |

#### When

<!-- splat:values path=splat_rule/parameters/when -->

| Field | Value |
| --- | --- |
| ` hit_points ` | ` 0 ` |
| ` instant_death ` | ` false ` |
