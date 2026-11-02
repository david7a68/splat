---
splat_schema: 1
splat_id: 43db1d09-eb5f-5099-8b19-ac6e53fc99c1
splat_kind: rule
splat_key: dnd2024:rules/paladin_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53
  url: https://www.dndbeyond.com/srd
---

# As a Level 1 Character

## Source text

<!-- SRD page 53 -->

•  Gain all the traits in the Core Paladin Traits table.

•  Gain the Paladin’s level 1 features, which are listed in the Paladin Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/paladin_as_a_level_1_character ` |
| ` implementation ` | ` dnd2024:rule_paladin_as_a_level_1_character ` |

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
| ` key ` | ` dnd2024:classes/paladin ` |

#### Embedded feature progression source

<!-- splat:values path=splat_rule/parameters/embedded_feature_progression_source -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/paladin_progression ` |

#### Level one features

<!-- splat:rows path=splat_rule/parameters/level_one_features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/paladin_level_1_lay_on_hands ` |
| ` 1 ` | ` dnd2024:class_features/paladin_level_1_spellcasting ` |
| ` 2 ` | ` dnd2024:class_features/paladin_level_1_weapon_mastery ` |
