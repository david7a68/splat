---
splat_schema: 1
splat_id: 9e1ea0e7-23c3-5691-a57c-b9970b82c0bb
splat_kind: creature
splat_key: dnd2024:creatures/pegasus
splat_name: Pegasus
splat_origin:
  key: dnd2024:reference/creature/pegasus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313
  url: https://www.dndbeyond.com/srd
---

# Pegasus

## Source text

<!-- SRD page 313 -->

Large Celestial, Chaotic Good\
AC 12 Initiative +2 (12)\
HP 59 (7d10 + 21)\
Speed 60 ft., Fly 90 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +4 Con 16 +3 +5\
Int 10 +0 +0 Wis 15 +2 +4 Cha 13 +1 +3\
Skills Perception +6\
Senses Passive Perception 16\
Languages Understands Celestial, Common, Elvish, and\
Sylvan but can’t speak\
CR 2 (XP 450; PB +2)\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 7 (1d6\
+ 4) Bludgeoning damage plus 5 (2d4) Radiant damage.\

Original layout: [[../../srd-5.2.1/pages/313|page 313]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Chaotic Good ` |
| ` movement_text ` | ` 60 ft., Fly 90 ft. ` |
| ` skills_text ` | ` Perception +6 ` |
| ` senses_text ` | ` Passive Perception 16 ` |
| ` languages_text ` | ` Understands Celestial, Common, Elvish, and Sylvan but can’t speak ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 59 (7d10 + 21) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 15 ` | ` 4 ` |
| ` Constitution ` | ` 16 ` | ` 5 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 15 ` | ` 4 ` |
| ` Charisma ` | ` 13 ` | ` 3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 59 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 60 ` | ` ft ` |
| ` fly_speed ` | ` 90 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/pegasus ` |
| ` implementation ` | ` dnd2024:creature_pegasus ` |

### Parameters

#### Features

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

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
  - average: 7
    roll:
      count: 1
      sides: 6
      modifier: 4
    type: bludgeoning
  - average: 5
    roll:
      count: 2
      sides: 4
      modifier: 0
    type: radiant
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 6 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- celestial
- common
- elvish
- sylvan
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
