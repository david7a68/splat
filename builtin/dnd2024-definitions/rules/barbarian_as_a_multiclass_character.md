---
splat_schema: 2
splat_id: f1eb1136-7b9b-5b53-9dfb-0e404ed5029f
splat_kind: rule
splat_key: dnd2024:rules/barbarian_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin: dnd2024:reference/pages/028
splat_sources:
- SRD 5.2.1, pp. 28, https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 28 -->

•  Gain the following traits from the Core Barbarian Traits table: Hit Point Die, proficiency with Martial weapons, and training with Shields.

•  Gain the Barbarian’s level 1 features, which are listed in the Barbarian Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/barbarian_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rules_barbarian_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_core_fields ` | ` ["hit_point_die"] ` |
| ` weapon_proficiencies ` | ` ["martial"] ` |
| ` armor_training ` | ` ["shield"] ` |
| ` spell_slots ` | ` not_applicable ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/barbarian ` |

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` class_level ` | ` 1 ` |
| ` from ` | ` class_definition ` |
