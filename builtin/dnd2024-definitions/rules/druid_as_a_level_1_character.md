---
splat_schema: 1
splat_id: 6621b501-072a-50cb-bc24-e3d31afd6ee9
splat_kind: rule
splat_key: dnd2024:rules/druid_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41
  url: https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 41 -->

•  Gain all the traits in the Core Druid Traits table.

•  Gain the Druid’s level 1 features, which are listed in the Druid Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/druid_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rules_druid_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_core_traits ` | ` all ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/druid ` |

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` class_level ` | ` 1 ` |
| ` from ` | ` class_definition ` |
