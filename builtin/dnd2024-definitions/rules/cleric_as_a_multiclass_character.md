---
splat_schema: 1
splat_id: e8199b30-6022-583c-9e63-a7d4cc96c741
splat_kind: rule
splat_key: dnd2024:rules/cleric_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 36 -->

•  Gain the following traits from the Core Cleric Traits table: Hit Point Die and training with Light and Medium armor and Shields.

•  Gain the Cleric’s level 1 features, which are listed in the Cleric Features table. See the multiclassing rules in “Character Creation” to determine your available spell slots.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/cleric_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rule_cleric_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry_mode ` | ` multiclass ` |
| ` hit_die_sides ` | ` 8 ` |
| ` armor_training ` | ` ["light", "medium", "shield"] ` |
| ` musical_instrument_choice_count ` | ` 0 ` |
| ` grant_features_at_class_level ` | ` 1 ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/cleric ` |

#### Skill choice

<!-- splat:values path=splat_rule/parameters/skill_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 0 ` |
| ` options ` | ` null ` |

#### Weapon proficiencies

<!-- splat:yaml path=splat_rule/parameters/weapon_proficiencies -->

```yaml
[]
```

#### Spell slots

<!-- splat:values path=splat_rule/parameters/spell_slots -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/multiclassing_spellcasting ` |

#### Level one features

<!-- splat:rows path=splat_rule/parameters/level_one_features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_1_divine_order ` |
| ` 1 ` | ` dnd2024:class_features/cleric_level_1_spellcasting ` |
