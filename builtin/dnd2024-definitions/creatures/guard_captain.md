---
splat_schema: 1
splat_id: 9eb70a3b-655f-5d61-9db6-9da7198c1f94
splat_kind: creature
splat_key: dnd2024:creatures/guard_captain
splat_name: Guard Captain
splat_origin:
  key: dnd2024:reference/creature/guard_captain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 296
  url: https://www.dndbeyond.com/srd
---

# Guard Captain

## Source text

<!-- SRD page 296 -->

Medium or Small Humanoid, Neutral\
AC 18  Initiative +4 (14)\
HP 75 (10d8 + 30)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 14 +2 +2 Con 16 +3 +3\
Int 12 +1 +1 Wis 14 +2 +2 Cha 13 +1 +1\
Skills Athletics +6, Perception +4\
Gear Breastplate, Javelins (6), Longsword, Shield\
Senses Passive Perception 14\
Languages Common\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The guard makes two attacks, using Javelin\
or Longsword in any combination.\
Javelin. Melee or Ranged Attack Roll: +6, reach 5 ft. or\
range 30/120 ft. Hit: 14 (3d6 + 4) Piercing damage.\
Longsword. Melee Attack Roll: +6, reach 5 ft. Hit: 15\
(2d10 + 4) Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/296|page 296]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Athletics +6, Perception +4 ` |
| ` gear_text ` | ` Breastplate, Javelins (6), Longsword, Shield ` |
| ` senses_text ` | ` Passive Perception 14 ` |
| ` languages_text ` | ` Common ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 75 (10d8 + 30) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 12 ` | ` 1 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 13 ` | ` 1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 75 ` |  |
| ` initiative ` | ` 4 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/guard_captain ` |
| ` implementation ` | ` dnd2024:creature_guard_captain ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - javelin
  - longsword
```

##### Javelin

<!-- splat:yaml path=splat_rule/parameters/features/javelin -->

```yaml
economy: action
operation: attack
attack:
  mode: melee_or_ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
  range_ft: 30
  long_range_ft: 120
hit:
  damage:
  - average: 14
    roll:
      count: 3
      sides: 6
      modifier: 4
    type: piercing
```

##### Longsword

<!-- splat:yaml path=splat_rule/parameters/features/longsword -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 15
    roll:
      count: 2
      sides: 10
      modifier: 4
    type: slashing
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 6 ` |
| ` perception ` | ` 4 ` |

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | quantity |
| --- | --- | --- |
| ` 0 ` | ` breastplate ` | ` 1 ` |
| ` 1 ` | ` javelins ` | ` 6 ` |
| ` 2 ` | ` longsword ` | ` 1 ` |
| ` 3 ` | ` shield ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
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
