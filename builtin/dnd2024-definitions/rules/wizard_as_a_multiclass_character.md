---
splat_schema: 1
splat_id: dc625776-3147-5cd3-98e7-20bf3e5cde4e
splat_kind: rule
splat_key: dnd2024:rules/wizard_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/077
splat_sources:
- title: SRD 5.2.1
  locator: pp. 77
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 77 -->

•  Gain the Hit Point Die from the Core Wizard Traits\
table.\
•  Gain the Wizard’s level 1 features, which are\
listed in the Wizard Features table. See the mul-\
ticlassing rules in “Character Creation” to deter-\
mine your available spell slots.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/wizard_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rules_wizard_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry ` | ` multiclass ` |
| ` hit_die_sides ` | ` 6 ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/wizard ` |

#### Armor training

<!-- splat:yaml path=splat_rule/parameters/armor_training -->

```yaml
[]
```

#### Features

<!-- splat:table
path: splat_rule/parameters/features
key: Feature
order:
- wizard_level_1_arcane_recovery
- wizard_level_1_ritual_adept
- wizard_level_1_spellcasting
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
| [Level 1: Arcane Recovery](<../class_features/wizard_level_1_arcane_recovery.md> "dnd2024:class_features/wizard_level_1_arcane_recovery") | ` 1 ` |
| [Level 1: Ritual Adept](<../class_features/wizard_level_1_ritual_adept.md> "dnd2024:class_features/wizard_level_1_ritual_adept") | ` 1 ` |
| [Level 1: Spellcasting](<../class_features/wizard_level_1_spellcasting.md> "dnd2024:class_features/wizard_level_1_spellcasting") | ` 1 ` |

#### Spell slots rule

<!-- splat:values path=splat_rule/parameters/spell_slots_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/multiclassing_spellcasting ` |
