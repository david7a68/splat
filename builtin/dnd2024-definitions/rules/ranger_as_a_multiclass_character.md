---
splat_schema: 1
splat_id: 6592c6b4-936e-5dd1-9a7d-69a9e85939bb
splat_kind: rule
splat_key: dnd2024:rules/ranger_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 57 -->

•  Gain the following traits from the Core Ranger\
Traits table: Hit Point Die, proficiency with Mar-\
tial weapons, proficiency in one skill of your\
choice from the Ranger’s skill list, and training\
with Light and Medium armor and Shields.\
•  Gain the Ranger’s level 1 features, which are\
listed in the Ranger Features table. See the mul-\
ticlassing rules in “Character Creation” to deter-\
mine your available spell slots.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/ranger_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rule_ranger_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry_mode ` | ` multiclass ` |
| ` hit_die_sides ` | ` 10 ` |
| ` armor_training ` | ` ["light", "medium", "shield"] ` |
| ` musical_instrument_choice_count ` | ` 0 ` |
| ` weapon_proficiencies ` | ` ["martial"] ` |
| ` grant_features_at_class_level ` | ` 1 ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/ranger ` |

#### Skill choice

<!-- splat:values path=splat_rule/parameters/skill_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` options ` | ` ["animal_handling", "athletics", "insight", "investigation", "nature", "perception", "stealth", "survival"] ` |

#### Spell slots

<!-- splat:values path=splat_rule/parameters/spell_slots -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/multiclassing_spellcasting ` |

#### Level one features

<!-- splat:rows path=splat_rule/parameters/level_one_features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/ranger_level_1_favored_enemy ` |
| ` 1 ` | ` dnd2024:class_features/ranger_level_1_spellcasting ` |
| ` 2 ` | ` dnd2024:class_features/ranger_level_1_weapon_mastery ` |
