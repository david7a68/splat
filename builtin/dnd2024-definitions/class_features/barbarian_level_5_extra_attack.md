---
splat_schema: 1
splat_id: 962c9d58-958e-5761-87d7-0a9a3d229f60
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_5_extra_attack
splat_name: 'Level 5: Extra Attack'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
---

# Level 5: Extra Attack

## Source text

<!-- SRD page 29 -->

You can attack twice instead of once whenever you\
take the Attack action on your turn.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_5_extra_attack ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_5_extra_attack ` |

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
| ` class ` | ` barbarian ` |
| ` minimum_level ` | ` 5 ` |
