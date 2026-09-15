---
splat_schema: 1
splat_id: 70523a2d-8d24-57a8-bb2b-89db5b614d56
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_17_precise_hunter
splat_name: 'Level 17: Precise Hunter'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
---

# Level 17: Precise Hunter

## Source text

<!-- SRD page 59 -->

You have Advantage on attack rolls against the crea-\
ture currently marked by your Hunter’s Mark.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_17_precise_hunter ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_17_precise_hunter ` |

### Parameters

#### Advantage

<!-- splat:values path=splat_rule/parameters/advantage -->

| Field | Value |
| --- | --- |
| ` attack_target ` | ` actor.current_hunters_mark_target ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_level ` | ` 17 ` |
