---
splat_schema: 1
splat_id: dd57d273-fc5e-53a3-8a5e-f5a6158cb96c
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_13_relentless_hunter
splat_name: 'Level 13: Relentless Hunter'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
---

# Level 13: Relentless Hunter

## Source text

<!-- SRD page 59 -->

Taking damage can’t break your Concentration on Hunter’s Mark.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_13_relentless_hunter ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_13_relentless_hunter ` |

### Parameters

#### Spell

<!-- splat:values path=splat_rule/parameters/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hunter_s_mark ` |

#### Concentration end prevention

<!-- splat:values path=splat_rule/parameters/concentration_end_prevention -->

| Field | Value |
| --- | --- |
| ` cause ` | ` damage_taken ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_level ` | ` 13 ` |
