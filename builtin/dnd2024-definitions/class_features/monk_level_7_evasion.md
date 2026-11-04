---
splat_schema: 2
splat_id: ca23af47-f359-56e7-99e4-6be91a14275f
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_7_evasion
splat_name: 'Level 7: Evasion'
splat_origin: dnd2024:reference/pages/051
splat_sources:
- SRD 5.2.1, pp. 51, https://www.dndbeyond.com/srd
---

# Level 7: Evasion

## Source text

<!-- SRD page 51 -->

When you’re subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw and only half damage if you fail.

You don’t benefit from this feature if you have the Incapacitated condition.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_7_evasion ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_7_evasion ` |

### Parameters

#### When

<!-- splat:values path=splat_rule/parameters/when -->

| Field | Value |
| --- | --- |
| ` saving_throw_ability ` | ` dexterity ` |
| ` normal_success_damage_multiplier ` | ` 0.5 ` |
| ` incapacitated ` | ` false ` |

#### Damage multipliers

<!-- splat:values path=splat_rule/parameters/damage_multipliers -->

| Field | Value |
| --- | --- |
| ` success ` | ` 0 ` |
| ` failure ` | ` 0.5 ` |
