---
splat_schema: 1
splat_id: 657f9051-fe6c-5ebf-895a-fbd2eff8ebc7
splat_kind: rule
splat_key: dnd2024:rules/bard_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 31 -->

•  Gain the following traits from the Core Bard Traits table: Hit Point Die, proficiency in one skill of your choice, proficiency with one Musical Instrument of your choice, and training with Light armor.

•  Gain the Bard’s level 1 features, which are listed in the Bard Features table. See the multiclassing rules in “Character Creation” to determine your available spell slots.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/bard_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rule_bard_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry_mode ` | ` multiclass ` |
| ` hit_die_sides ` | ` 8 ` |
| ` armor_training ` | ` ["light"] ` |
| ` musical_instrument_choice_count ` | ` 1 ` |
| ` grant_features_at_class_level ` | ` 1 ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/bard ` |

#### Skill choice

<!-- splat:values path=splat_rule/parameters/skill_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` options ` | ` all ` |

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
| ` 0 ` | ` dnd2024:class_features/bard_level_1_bardic_inspiration ` |
| ` 1 ` | ` dnd2024:class_features/bard_level_1_spellcasting ` |
