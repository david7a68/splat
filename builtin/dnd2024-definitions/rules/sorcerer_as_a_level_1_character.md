---
splat_schema: 2
splat_id: ac56109a-cc47-5b67-a56c-99fc5d3a86fc
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin: dnd2024:reference/pages/064
splat_sources:
- SRD 5.2.1, pp. 64, https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 64 -->

•  Gain all the traits in the Core Sorcerer Traits table.

•  Gain the Sorcerer’s level 1 features, which are listed in the Sorcerer Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rules_sorcerer_as_a_level_1_character ` |

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
| ` key ` | ` dnd2024:classes/sorcerer ` |

#### Features

<!-- splat:table
path: splat_rule/parameters/features
key: Feature
order:
- sorcerer_level_1_innate_sorcery
- sorcerer_level_1_spellcasting
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
| [Level 1: Innate Sorcery](<../class_features/sorcerer_level_1_innate_sorcery.md> "dnd2024:class_features/sorcerer_level_1_innate_sorcery") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/sorcerer_level_1_spellcasting.md> "dnd2024:class_features/sorcerer_level_1_spellcasting") | ` 1 ` |
