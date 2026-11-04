---
splat_schema: 2
splat_id: e2fe0d85-6fe9-5261-952f-2805d876fb5e
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin: dnd2024:reference/pages/064
splat_sources:
- SRD 5.2.1, pp. 64, https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 64 -->

•  Gain the Hit Point Die from the Core Sorcerer Traits table.

•  Gain the Sorcerer’s level 1 features, which are listed in the Sorcerer Features table. See the multiclassing rules in “Character Creation” to determine your available spell slots.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sorcerer_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rule_sorcerer_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry_mode ` | ` multiclass ` |
| ` hit_die_sides ` | ` 6 ` |
| ` musical_instrument_choice_count ` | ` 0 ` |
| ` grant_features_at_class_level ` | ` 1 ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/sorcerer ` |

#### Armor training

<!-- splat:yaml path=splat_rule/parameters/armor_training -->

```yaml
[]
```

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
| ` 0 ` | ` dnd2024:class_features/sorcerer_level_1_innate_sorcery ` |
| ` 1 ` | ` dnd2024:class_features/sorcerer_level_1_spellcasting ` |
