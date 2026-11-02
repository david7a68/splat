---
splat_schema: 1
splat_id: e50cfa8d-b6fd-542b-bd8b-e14e0929fa19
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_5_extra_attack
splat_name: 'Level 5: Extra Attack'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 5: Extra Attack

## Source text

<!-- SRD page 51 -->

You can attack twice instead of once whenever you take the Attack action on your turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_5_extra_attack ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_5_extra_attack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` attack ` |
| ` turn ` | ` actor ` |
| ` attack_count ` | ` 2 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 5 ` |
