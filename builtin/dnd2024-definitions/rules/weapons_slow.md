---
splat_schema: 2
splat_id: b328bae9-3a97-5fa7-8ea8-29dc2f473890
splat_kind: rule
splat_key: dnd2024:rules/weapons_slow
splat_name: Slow
splat_origin: dnd2024:reference/pages/090
splat_sources:
- SRD 5.2.1, pp. 90, https://www.dndbeyond.com/srd
---

# Slow

## Source text

<!-- SRD page 90 -->

If you hit a creature with this weapon and deal damage to it, you can reduce its Speed by 10 feet until the start of your next turn. If the creature is hit more than once by weapons that have this property, the Speed reduction doesn’t exceed 10 feet.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_slow ` |
| ` implementation ` | ` dnd2024:rules_weapons_slow ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` speed_reduction_feet ` | ` 10 ` |
| ` maximum_combined_slow_reduction_feet ` | ` 10 ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` hit_creature ` | ` true ` |
| ` damage_dealt ` | ` true ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` until ` | ` attacker_next_turn_start ` |
