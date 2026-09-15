---
splat_schema: 1
splat_id: 1a191a34-0ce9-5f59-8e13-07ecfba84ef5
splat_kind: rule
splat_key: dnd2024:rules/monk_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 49 -->

•  Gain all the traits in the Core Monk Traits table.\
•  Gain the Monk’s level 1 features, which are listed\
in the Monk Features table.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/monk_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rules_monk_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_total_character_level ` | ` 1 ` |
| ` grant_all_core_traits ` | ` true ` |

#### Core traits

<!-- splat:values path=splat_rule/parameters/core_traits -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/monk ` |

#### Features

<!-- splat:table
path: splat_rule/parameters/features
key: Feature
order:
- monk_level_1_martial_arts
- monk_level_1_unarmored_defense
columns:
  Feature:
    path: feature
    type: reference
  Level:
    path: level
    type: integer
-->

| Feature | Level |
| --- | --- |
| [Level 1: Martial Arts](<../class_features/monk_level_1_martial_arts.md> "dnd2024:class_features/monk_level_1_martial_arts") | ` 1 ` |
| [Level 1: Unarmored Defense](<../class_features/monk_level_1_unarmored_defense.md> "dnd2024:class_features/monk_level_1_unarmored_defense") | ` 1 ` |
