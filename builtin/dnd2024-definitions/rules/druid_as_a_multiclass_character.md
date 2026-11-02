---
splat_schema: 1
splat_id: dff89ced-250c-5c88-a4d6-74a0e18c4ef9
splat_kind: rule
splat_key: dnd2024:rules/druid_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 41 -->

•  Gain the following traits from the Core Druid Traits table: Hit Point Die and training with Light armor and Shields.

•  Gain the Druid’s level 1 features, which are listed in the Druid Features table. See the multiclassing rules in “Character Creation” to determine your available spell slots.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/druid_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rules_druid_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry ` | ` multiclass ` |
| ` hit_die_sides ` | ` 8 ` |
| ` armor_training ` | ` ["light", "shield"] ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/druid ` |

#### Features

<!-- splat:table
path: splat_rule/parameters/features
key: Feature
order:
- druid_level_1_druidic
- druid_level_1_primal_order
- druid_level_1_spellcasting
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
| [Level 1: Druidic](<../class_features/druid_level_1_druidic.md> "dnd2024:class_features/druid_level_1_druidic") | ` 1 ` |
| [Level 1: Primal Order](<../class_features/druid_level_1_primal_order.md> "dnd2024:class_features/druid_level_1_primal_order") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/druid_level_1_spellcasting.md> "dnd2024:class_features/druid_level_1_spellcasting") | ` 1 ` |

#### Spell slots rule

<!-- splat:values path=splat_rule/parameters/spell_slots_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/multiclassing_spellcasting ` |
