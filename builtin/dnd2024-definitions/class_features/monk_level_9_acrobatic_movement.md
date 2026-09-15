---
splat_schema: 1
splat_id: 8eba4d5b-02d3-5f10-bfd3-b5201f5d3732
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_9_acrobatic_movement
splat_name: 'Level 9: Acrobatic Movement'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
---

# Level 9: Acrobatic Movement

## Source text

<!-- SRD page 51 -->

While you aren’t wearing armor or wielding a\
Shield, you gain the ability to move along vertical\
surfaces and across liquids on your turn without\
falling during the movement.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_9_acrobatic_movement ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_9_acrobatic_movement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movement_surfaces ` | ` ["vertical", "liquid"] ` |
| ` falling_during_movement ` | ` false ` |

#### Requires

<!-- splat:values path=splat_rule/parameters/requires -->

| Field | Value |
| --- | --- |
| ` armor_worn ` | ` false ` |
| ` shield_wielded ` | ` false ` |
| ` turn ` | ` actor ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 9 ` |
