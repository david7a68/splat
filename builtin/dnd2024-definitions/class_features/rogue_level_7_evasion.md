---
splat_schema: 1
splat_id: 8d2df17b-dfa9-5c3e-9f69-1436ccfb79a0
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_7_evasion
splat_name: 'Level 7: Evasion'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
---

# Level 7: Evasion

## Source text

<!-- SRD page 63 -->

You can nimbly dodge out of the way of certain dangers. When you’re subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw and only half damage if you fail. You can’t use this feature if you have the Incapacitated condition.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_7_evasion ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_7_evasion ` |

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
