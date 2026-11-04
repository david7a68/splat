---
splat_schema: 2
splat_id: af2e9ac8-fac6-5292-b5fe-0c96b1b9bb51
splat_kind: creature
splat_key: dnd2024:creatures/mastiff
splat_name: Mastiff
splat_origin: dnd2024:reference/creature/mastiff
splat_sources:
- SRD 5.2.1, pp. 357, https://www.dndbeyond.com/srd
---

# Mastiff


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 5 (1d8 + 1) |
| Initiative | +2 (12) |
| Speed | 40 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Skills | Perception +5 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 13 | +1 | +1 |
| Dex | 14 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +3 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 357 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 5 (1d8 + 1)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 13 +1 +1 Dex 14 +2 +2 Con 12 +1 +1\
> Int 3 −4 −4 Wis 12 +1 +3 Cha 7 −2 −2\
> Skills Perception +5\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages None\
> CR 1/8 (XP 25; PB +2)


### Actions

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Piercing damage. If the target is a Medium or smaller creature, it has the Prone condition.

Original layout: [[../../srd-5.2.1/pages/357|page 357]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/mastiff ` |
| ` implementation ` | ` dnd2024:creature_mastiff ` |

### Parameters

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 3
  reach_ft: 5
hit:
  damage:
  - average: 4
    roll:
      count: 1
      sides: 6
      modifier: 1
    type: piercing
  prone:
    condition: prone
    maximum_size: medium
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 15 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands: []
```

#### Resolution conventions

<!-- splat:yaml path=splat_rule/parameters/resolution_conventions -->

```yaml
distance_unit: foot
duration_unit: second
damage_roll_mode:
  choice:
  - listed_average
  - roll_dice
  chooser: gm
supplied_totals: before_additional_effect_modifiers
multiattack_nested_economy_cost: 0
nested_feature_resource_cost: normal
spell_level_unless_specified: lowest_possible
spell_upcast_without_permission: false
spell_casting_time_at_least_one_minute: normal_casting_procedure
geometry:
  input: adjudication
  assumptions:
  - range_and_visibility_verified
  - area_membership_verified
  - path_and_destination_legal
unlisted_success_branch: no_effect
unlisted_duration: until_removed_by_applicable_rule
```
