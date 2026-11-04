---
splat_schema: 2
splat_id: ec4d2bf7-8884-53b5-82af-780a5e103ebf
splat_kind: rule
splat_key: dnd2024:rules/monk_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin: dnd2024:reference/pages/049
splat_sources:
- SRD 5.2.1, pp. 49, https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 49 -->

•  Gain the Hit Point Die trait from the Core Monk Traits table.

•  Gain the Monk’s level 1 features, which are listed in the Monk Features table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/monk_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rule_monk_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` entry_mode ` | ` multiclass ` |
| ` hit_die_sides ` | ` 8 ` |
| ` musical_instrument_choice_count ` | ` 0 ` |
| ` grant_features_at_class_level ` | ` 1 ` |
| ` spell_slots ` | ` not_applicable ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/monk ` |

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

#### Level one features

<!-- splat:rows path=splat_rule/parameters/level_one_features -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/monk_level_1_martial_arts ` |
| ` 1 ` | ` dnd2024:class_features/monk_level_1_unarmored_defense ` |
