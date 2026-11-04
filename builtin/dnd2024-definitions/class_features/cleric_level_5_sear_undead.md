---
splat_schema: 2
splat_id: 9bb79a15-f2ae-5e02-a7b6-b0c01116f75e
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_5_sear_undead
splat_name: 'Level 5: Sear Undead'
splat_origin: dnd2024:reference/pages/037
splat_sources:
- SRD 5.2.1, pp. 37, https://www.dndbeyond.com/srd
---

# Level 5: Sear Undead

## Source text

<!-- SRD page 37 -->

Whenever you use Turn Undead, you can roll a number of d8s equal to your Wisdom modifier (minimum of 1d8) and add the rolls together. Each Undead that fails its saving throw against that use of Turn Undead takes Radiant damage equal to the roll’s total. This damage doesn’t end the turn effect.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_5_sear_undead ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_5_sear_undead ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` turn_undead.used ` |
| ` optional ` | ` true ` |
| ` damage_ends_turn_undead ` | ` false ` |

#### Damage roll

<!-- splat:yaml path=splat_rule/parameters/damage_roll -->

```yaml
shared_for_all_targets: true
roll:
  dice:
    count:
      op: max
      args:
      - 1
      - stat: abilities.wisdom.modifier
    sides: 8
type: radiant
```

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` undead ` |
| ` failed_triggering_save ` | ` true ` |
