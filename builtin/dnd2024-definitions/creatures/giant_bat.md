---
splat_schema: 1
splat_id: 3e305169-1950-5fa9-b74f-21fa4f6bc77c
splat_kind: creature
splat_key: dnd2024:creatures/giant_bat
splat_name: Giant Bat
splat_origin:
  key: dnd2024:reference/creature/giant_bat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 349
  url: https://www.dndbeyond.com/srd
---

# Giant Bat

## Source text

<!-- SRD page 349 -->

Large Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 22 (4d10)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 16 +3 +3 Con 11 +0 +0\
Int 2 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
Senses Blindsight 120 ft.; Passive Perception 11\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/349|page 349]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 10 ft., Fly 60 ft. ` |
| ` senses_text ` | ` Blindsight 120 ft.; Passive Perception 11 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 22 (4d10) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 15 ` | ` 2 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 11 ` | ` 0 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 22 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_bat ` |
| ` implementation ` | ` dnd2024:creature_giant_bat ` |

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
  supplied_attack_total: 5
  reach_ft: 5
hit:
  damage:
  - average: 6
    roll:
      count: 1
      sides: 6
      modifier: 3
    type: piercing
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 120 ` |
| ` passive_perception ` | ` 11 ` |

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
