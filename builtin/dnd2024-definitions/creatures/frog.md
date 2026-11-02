---
splat_schema: 1
splat_id: d9dba9af-35dc-5cab-b2a5-997303b74c58
splat_kind: creature
splat_key: dnd2024:creatures/frog
splat_name: Frog
splat_origin:
  key: dnd2024:reference/creature/frog
splat_sources:
- title: SRD 5.2.1
  locator: pp. 348, 349
  url: https://www.dndbeyond.com/srd
---

# Frog

## Source text

<!-- SRD page 348 -->

Tiny Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 1 (1d4 − 1)\
Speed 20 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 1 −5 −5 Dex 13 +1 +1 Con 8 −1 −1\
Int 1 −5 −5 Wis 8 −1 −1 Cha 3 −4 −4\
Skills Perception +1, Stealth +3\
Senses Darkvision 30 ft.; Passive Perception 11\
Languages None\
CR 0 (XP 10; PB +2)

<!-- SRD page 349 -->

### Traits

Amphibious. The frog can breathe air and water.

Standing Leap. The frog’s Long Jump is up to 10 feet and its High Jump is up to 5 feet with or without a running start.

### Actions

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/348|page 348]], [[../../srd-5.2.1/pages/349|page 349]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 20 ft., Swim 20 ft. ` |
| ` skills_text ` | ` Perception +1, Stealth +3 ` |
| ` senses_text ` | ` Darkvision 30 ft.; Passive Perception 11 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 1 (1d4 - 1) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 1 ` | ` -5 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 8 ` | ` -1 ` |
| ` Intelligence ` | ` 1 ` | ` -5 ` |
| ` Wisdom ` | ` 8 ` | ` -1 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 1 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` swim_speed ` | ` 20 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/frog ` |
| ` implementation ` | ` dnd2024:creature_frog ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

##### Standing leap

<!-- splat:values path=splat_rule/parameters/features/standing_leap -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` long_jump_ft ` | ` 10 ` |
| ` high_jump_ft ` | ` 5 ` |
| ` running_start_required ` | ` false ` |

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
  - average: 1
    roll:
      constant: 1
    type: piercing
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 1 ` |
| ` stealth ` | ` 3 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 30 ` |
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
