---
splat_schema: 2
splat_id: 00c4d51c-6a19-583b-a094-926faacd71a7
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_dropping_to_0_hit_points
splat_name: Dropping to 0 Hit Points
splat_origin: dnd2024:reference/pages/017
splat_sources:
- SRD 5.2.1, pp. 17, https://www.dndbeyond.com/srd
---

# Dropping to 0 Hit Points

## Source text

<!-- SRD page 17 -->

When a creature drops to 0 Hit Points, it either dies outright or falls unconscious, as explained below.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_dropping_to_0_hit_points ` |
| ` implementation ` | ` dnd2024:rule_damage_and_healing_dropping_to_0_hit_points ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` outcomes ` | ` ["instant_death", "unconscious"] ` |

#### Apply definitions

<!-- splat:rows path=splat_rule/parameters/apply_definitions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/dead ` |
| ` 1 ` | ` dnd2024:glossary/unconscious_condition ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` hit_points ` | ` 0 ` |
