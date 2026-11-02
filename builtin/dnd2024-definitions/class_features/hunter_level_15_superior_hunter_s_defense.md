---
splat_schema: 1
splat_id: ca92ed48-3abe-527e-b1b8-40a16e208e09
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_15_superior_hunter_s_defense
splat_name: 'Level 15: Superior Hunter’s Defense'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
---

# Level 15: Superior Hunter’s Defense

## Source text

<!-- SRD page 61 -->

When you take damage, you can take a Reaction to give yourself Resistance to that damage and any other damage of the same type until the end of the current turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/hunter_level_15_superior_hunter_s_defense ` |
| ` implementation ` | ` dnd2024:class_features_hunter_level_15_superior_hunter_s_defense ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` damage.taken ` |
| ` action ` | ` reaction ` |

#### Resistance

<!-- splat:values path=splat_rule/parameters/resistance -->

| Field | Value |
| --- | --- |
| ` type ` | ` triggering_damage_type ` |
| ` applies_to_triggering_damage ` | ` true ` |
| ` until ` | ` current_turn_end ` |
