---
splat_schema: 1
splat_id: f4ff983a-33bb-5a7b-9161-a0a881dee6b3
splat_kind: rule
splat_key: dnd2024:rules/paladin_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 53 -->

•  Gain the following traits from the Core Paladin\
Traits table: Hit Point Die, proficiency with Mar-\
tial weapons, and training with Light and Medium\
armor and Shields.\
•  Gain the Paladin’s level 1 features, which are\
listed in the Paladin Features table. See the multi-\
classing rules in “Character Creation” to deter-\
mine your available spell slots.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/paladin_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rules_paladin_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_core_fields ` | ` ["hit_point_die"] ` |
| ` weapon_proficiencies ` | ` ["martial"] ` |
| ` armor_training ` | ` ["light", "medium", "shield"] ` |
| ` spell_slots ` | ` multiclass_spellcasting_rules ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/paladin ` |

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` class_level ` | ` 1 ` |
| ` from ` | ` class_definition ` |
