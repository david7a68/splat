---
splat_schema: 1
splat_id: 488c2274-8013-558e-ba89-c7f90a86abda
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_18_indomitable_might
splat_name: 'Level 18: Indomitable Might'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
---

# Level 18: Indomitable Might

## Source text

<!-- SRD page 30 -->

If your total for a Strength check or Strength saving\
throw is less than your Strength score, you can use\
that score in place of the total.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_18_indomitable_might ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_18_indomitable_might ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` rolls ` | ` ["ability_check", "saving_throw"] ` |
| ` ability ` | ` strength ` |

#### Optional total floor

<!-- splat:values path=splat_rule/parameters/optional_total_floor -->

| Field | Value |
| --- | --- |
| ` stat ` | ` abilities.strength.score ` |
