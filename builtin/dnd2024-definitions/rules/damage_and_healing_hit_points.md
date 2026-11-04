---
splat_schema: 2
splat_id: 7b749381-128f-5563-8b8c-e68513aaa2f3
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_hit_points
splat_name: Hit Points
splat_origin: dnd2024:reference/pages/016
splat_sources:
- SRD 5.2.1, pp. 16, https://www.dndbeyond.com/srd
---

# Hit Points

## Source text

<!-- SRD page 16 -->

Hit Points represent durability and the will to live. Creatures with more Hit Points are more difficult to kill. Your Hit Point maximum is the number of Hit Points you have when uninjured. Your current Hit Points can be any number from that maximum down to 0, which is the lowest Hit Points can go.

### Resting

Adventurers can’t spend every hour adventuring. They need rest. Any creature can take hour-long Short Rests in the midst of a day and an 8-hour Long Rest to end it. Regaining Hit Points is one of the main benefits of a rest. “Rules Glossary” provides the rules for Short and Long Rests.

Whenever you take damage, subtract it from your Hit Points. Hit Point loss has no effect on your capabilities until you reach 0 Hit Points.

If you have half your Hit Points or fewer, you’re Bloodied, which has no game effect on its own but which might trigger other game effects.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_hit_points ` |
| ` implementation ` | ` dnd2024:rule_damage_and_healing_hit_points ` |

### Parameters

#### Apply definitions

<!-- splat:rows path=splat_rule/parameters/apply_definitions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/short_rest ` |
| ` 1 ` | ` dnd2024:glossary/long_rest ` |

#### Hit points

<!-- splat:values path=splat_rule/parameters/hit_points -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 0 ` |
| ` maximum ` | ` hit_point_maximum ` |
| ` damage_operation ` | ` subtract ` |
| ` loss_above_zero_impairs_capabilities ` | ` false ` |

#### Bloodied

<!-- splat:values path=splat_rule/parameters/bloodied -->

| Field | Value |
| --- | --- |
| ` current_hit_points_lte_fraction_maximum ` | ` 0.5 ` |
| ` inherent_effect ` | ` false ` |

#### Rest duration hours

<!-- splat:values path=splat_rule/parameters/rest_duration_hours -->

| Field | Value |
| --- | --- |
| ` short ` | ` 1 ` |
| ` long ` | ` 8 ` |
