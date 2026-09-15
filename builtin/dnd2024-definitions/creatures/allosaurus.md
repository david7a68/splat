---
splat_schema: 1
splat_id: 6ff31d77-aeac-5666-8066-a74ed07ab885
splat_kind: creature
splat_key: dnd2024:creatures/allosaurus
splat_name: Allosaurus
splat_origin:
  key: dnd2024:reference/creature/allosaurus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344
  url: https://www.dndbeyond.com/srd
---

# Allosaurus

## Source text

<!-- SRD page 344 -->

Large Beast (Dinosaur), Unaligned\
AC 13  Initiative +1 (11)\
HP 51 (6d10 + 18)\
Speed 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 13 +1 +1 Con 17 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +5\
Senses Passive Perception 15\
Languages None\
CR 2 (XP 450; PB +2)\

### Actions

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 15 (2d10 +\
4) Piercing damage.\
Claws. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8\
+4) Slashing damage. If the target is a Large or smaller\
creature and the allosaurus moved 30+ feet straight\
toward it immediately before the hit, the target has the\
Prone condition, and the allosaurus can make one Bite\
attack against it.\

Original layout: [[../../srd-5.2.1/pages/344|page 344]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` tags_text ` | ` (Dinosaur) ` |
| ` movement_text ` | ` 60 ft. ` |
| ` skills_text ` | ` Perception +5 ` |
| ` senses_text ` | ` Passive Perception 15 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 51 (6d10 + 18) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 19 ` | ` 4 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 17 ` | ` 3 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 51 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/allosaurus ` |
| ` implementation ` | ` dnd2024:creature_allosaurus ` |

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
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 15
    roll:
      count: 2
      sides: 10
      modifier: 4
    type: piercing
```

##### Claws

<!-- splat:yaml path=splat_rule/parameters/features/claws -->

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
  - average: 8
    roll:
      count: 1
      sides: 8
      modifier: 4
    type: slashing
  prone:
    condition: prone
    maximum_size: large
    requires:
      immediately_preceding_straight_movement_toward_target_ft: 30
  additional_attack:
    feature: bite
    target: same
    requires:
      maximum_target_size: large
      immediately_preceding_straight_movement_toward_target_ft: 30
    optional: true
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
