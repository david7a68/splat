---
splat_schema: 2
splat_id: b96d0eea-daf0-56bb-a9e5-324f249d7afe
splat_kind: rule
splat_key: dnd2024:rules/barbarian_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin: dnd2024:reference/pages/028
splat_sources:
- SRD 5.2.1, pp. 28, https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 28 -->

•  Gain all the traits in the Core Barbarian Traits table.

•  Gain the Barbarian’s level 1 features, which are listed in the Barbarian Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/barbarian_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rule_barbarian_as_a_level_1_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry_mode ` | ` level_one_character ` |
| ` grant_all_core_traits ` | ` true ` |
| ` grant_features_at_class_level ` | ` 1 ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/barbarian ` |

#### Embedded feature progression source

<!-- splat:values path=splat_rule/parameters/embedded_feature_progression_source -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/barbarian_progression ` |

#### Level one features

<!-- splat:rows path=splat_rule/parameters/level_one_features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/barbarian_level_1_rage ` |
| ` 1 ` | ` dnd2024:class_features/barbarian_level_1_unarmored_defense ` |
| ` 2 ` | ` dnd2024:class_features/barbarian_level_1_weapon_mastery ` |
