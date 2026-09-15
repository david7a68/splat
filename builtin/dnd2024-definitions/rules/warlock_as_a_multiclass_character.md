---
splat_schema: 1
splat_id: 6e0d3e4e-ef3d-50f9-a877-dd7560d45c5a
splat_kind: rule
splat_key: dnd2024:rules/warlock_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
---

# As a Multiclass Character

## Source text

<!-- SRD page 70 -->

•  Gain the following traits from the Core Warlock\
Traits table: Hit Point Die and training with Light\
armor.\
•  Gain the Warlock’s level 1 features, which are\
listed in the Warlock Features table. See the mul-\
ticlassing rules in “Character Creation” to deter-\
mine your available spell slots.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_as_a_multiclass_character ` |
| ` implementation ` | ` dnd2024:rules_warlock_as_a_multiclass_character ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` grant_core_fields ` | ` ["hit_point_die"] ` |
| ` armor_training ` | ` ["light"] ` |
| ` spell_slots ` | ` multiclass_spellcasting_rules ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/warlock ` |

#### Weapon proficiencies

<!-- splat:yaml path=splat_rule/parameters/weapon_proficiencies -->

```yaml
[]
```

#### Grant features

<!-- splat:values path=splat_rule/parameters/grant_features -->

| Field | Value |
| --- | --- |
| ` class_level ` | ` 1 ` |
| ` from ` | ` class_definition ` |
