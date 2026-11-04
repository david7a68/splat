---
splat_schema: 2
splat_id: 96a6cef6-b93e-5400-99a8-9947e4688b71
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_11_superior_hunter_s_prey
splat_name: 'Level 11: Superior Hunter’s Prey'
splat_origin: dnd2024:reference/pages/061
splat_sources:
- SRD 5.2.1, pp. 61, https://www.dndbeyond.com/srd
---

# Level 11: Superior Hunter’s Prey

## Source text

<!-- SRD page 61 -->

Once per turn when you deal damage to a creature marked by your Hunter’s Mark, you can also deal that spell’s extra damage to a different creature that you can see within 30 feet of the first creature.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/hunter_level_11_superior_hunter_s_prey ` |
| ` implementation ` | ` dnd2024:class_features_hunter_level_11_superior_hunter_s_prey ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.damages_own_hunters_mark_target ` |
| ` optional ` | ` true ` |
| ` damage ` | ` triggering_hunters_mark.extra_damage ` |

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` period ` | ` turn ` |

#### Additional target

<!-- splat:values path=splat_rule/parameters/additional_target -->

| Field | Value |
| --- | --- |
| ` type ` | ` creature ` |
| ` different_from_original ` | ` true ` |
| ` visible_to_actor ` | ` true ` |
| ` within_original_target_feet ` | ` 30 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_level ` | ` 11 ` |
