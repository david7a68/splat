---
splat_schema: 1
splat_id: 9ba2827f-99b6-59da-a8c3-d724f26f2d8b
splat_kind: creature
splat_key: dnd2024:creatures/nightmare
splat_name: Nightmare
splat_origin:
  key: dnd2024:reference/creature/nightmare
splat_sources:
- title: SRD 5.2.1
  locator: pp. 311
  url: https://www.dndbeyond.com/srd
---

# Nightmare

## Source text

<!-- SRD page 311 -->

Large Fiend, Neutral Evil\
AC 13 Initiative +2 (12)\
HP 68 (8d10 + 24)\
Speed 60 ft., Fly 90 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 10 +0 +0 Wis 13 +1 +1 Cha 15 +2 +2\
Immunities Fire\
Senses Passive Perception 11\
Languages Understands Abyssal, Common, and Infernal\
but can’t speak\
CR 3 (XP 700; PB +2)\

### Traits

Confer Fire Resistance. The nightmare can grant Re-\
sistance to Fire damage to a rider while it is on the\
nightmare.\
Illumination. The nightmare sheds Bright Light in a 10-\
foot radius and Dim Light for an additional 10 feet.\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8\
+ 4) Bludgeoning damage plus 10 (3d6) Fire damage.\
Ethereal Stride. The nightmare and up to three willing\
creatures within 5 feet of it teleport to the Ethereal\
Plane from the Material Plane or vice versa.\

Original layout: [[../../srd-5.2.1/pages/311|page 311]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` fiend ` |
| ` alignment ` | ` Neutral Evil ` |
| ` movement_text ` | ` 60 ft., Fly 90 ft. (hover) ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` Understands Abyssal, Common, and Infernal but can’t speak ` |
| ` cr_text ` | ` 3 (XP 700; PB +2) ` |
| ` hit_points_text ` | ` 68 (8d10 + 24) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 15 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 68 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/nightmare ` |
| ` implementation ` | ` dnd2024:creature_nightmare ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |

#### Features

##### Confer fire resistance

<!-- splat:yaml path=splat_rule/parameters/features/confer_fire_resistance -->

```yaml
economy: trait
operation: passive_rule
optional: true
grant:
  damage_resistances:
  - fire
  target: rider
  while: mounted_on_self
```

##### Illumination

<!-- splat:values path=splat_rule/parameters/features/illumination -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` bright_radius_ft ` | ` 10 ` |
| ` additional_dim_radius_ft ` | ` 10 ` |

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
  - average: 13
    roll:
      count: 2
      sides: 8
      modifier: 4
    type: bludgeoning
  - average: 10
    roll:
      count: 3
      sides: 6
      modifier: 0
    type: fire
```

##### Ethereal stride

<!-- splat:yaml path=splat_rule/parameters/features/ethereal_stride -->

```yaml
economy: action
operation: planar_teleport
targets:
  self: true
  additional:
    count_maximum: 3
    willing: true
    range_ft: 5
plane_pairs:
- - material
  - ethereal
- - ethereal
  - material
```

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- abyssal
- common
- infernal
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
