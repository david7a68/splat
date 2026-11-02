---
splat_schema: 1
splat_id: 0bef5350-ad14-5b5c-95a7-be01b33718aa
splat_kind: creature
splat_key: dnd2024:creatures/giant_eagle
splat_name: Giant Eagle
splat_origin:
  key: dnd2024:reference/creature/giant_eagle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 350, 351
  url: https://www.dndbeyond.com/srd
---

# Giant Eagle

## Source text

<!-- SRD page 350 -->

Large Celestial, Neutral Good\
AC 13 Initiative +3 (13)\
HP 26 (4d10 + 4)\
Speed 10 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 17 +3 +3 Con 13 +1 +1\
Int 8 −1 −1 Wis 14 +2 +2 Cha 10 +0 +0\
Skills Perception +6\
Resistances Necrotic, Radiant\
Senses Passive Perception 16\
Languages Celestial; understands Common and Primordial (Auran) but can’t speak them\
CR 1 (XP 200; PB +2)

### Actions

Multiattack. The eagle makes two Rend attacks.

<!-- SRD page 351 -->

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Slashing damage plus 3 (1d6) Radiant damage.

Original layout: [[../../srd-5.2.1/pages/350|page 350]], [[../../srd-5.2.1/pages/351|page 351]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` celestial ` |
| ` alignment ` | ` Neutral Good ` |
| ` movement_text ` | ` 10 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +6 ` |
| ` resistances_text ` | ` Necrotic, Radiant ` |
| ` senses_text ` | ` Passive Perception 16 ` |
| ` languages_text ` | ` Celestial; understands Common and Primordial (Auran) but can’t speak them ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 26 (4d10 + 4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 16 ` | ` 3 ` |
| ` Dexterity ` | ` 17 ` | ` 3 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 8 ` | ` -1 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 26 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 80 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_eagle ` |
| ` implementation ` | ` dnd2024:creature_giant_eagle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["necrotic", "radiant"] ` |

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: rend
  count: 2
```

##### Rend

<!-- splat:values path=splat_rule/parameters/features/rend -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/rend/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 5
reach_ft: 5
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/rend/hit/damage
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Average:
    path: average
    type: integer
  Roll count:
    path: roll/count
    type: integer
  Roll sides:
    path: roll/sides
    type: integer
  Roll modifier:
    path: roll/modifier
    type: integer
  Type:
    path: type
    type: symbol
-->

| Key | Average | Roll count | Roll sides | Roll modifier | Type |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 1 ` | ` 4 ` | ` 3 ` | ` Slashing ` |
| ` row_2 ` | ` 3 ` | ` 1 ` | ` 6 ` | ` 0 ` | ` Radiant ` |

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
speaks:
- celestial
understands:
- common
- language: primordial
  dialects:
  - auran
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
