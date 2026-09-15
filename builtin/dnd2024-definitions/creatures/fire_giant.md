---
splat_schema: 1
splat_id: da6a0d9a-9e54-5118-a7c0-0508b28c00fe
splat_kind: creature
splat_key: dnd2024:creatures/fire_giant
splat_name: Fire Giant
splat_origin:
  key: dnd2024:reference/creature/fire_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 285
  url: https://www.dndbeyond.com/srd
---

# Fire Giant

## Source text

<!-- SRD page 285 -->

Huge Giant, Lawful Evil\
AC 18 Initiative +3 (13)\
HP 162 (13d12 + 78)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 25 +7 +7 Dex 9 −1 +3 Con 23 +6 +10\
Int 10 +0 +0 Wis 14 +2 +2 Cha 13 +1 +5\
Skills Athletics +11, Perception +6\
Immunities Fire\
Senses Passive Perception 16\
Languages Giant\
CR 9 (XP 5,000; PB +4)\

### Actions

Multiattack. The giant makes two attacks, using Flame\
Sword or Hammer Throw in any combination.\
Flame Sword. Melee Attack Roll: +11, reach 10 ft.\
Hit: 21 (4d6 + 7) Slashing damage plus 10 (3d6)\
Fire damage.\
Hammer Throw. Ranged Attack Roll: +11, range 60/240\
ft. Hit: 23 (3d10 + 7) Bludgeoning damage plus 4 (1d8)\
Fire damage, and the target is pushed up to 15 feet\
straight away from the giant and has Disadvantage\
on the next attack roll it makes before the end of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/285|page 285]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` giant ` |
| ` alignment ` | ` Lawful Evil ` |
| ` movement_text ` | ` 30 ft. ` |
| ` skills_text ` | ` Athletics +11, Perception +6 ` |
| ` immunities_text ` | ` Fire ` |
| ` senses_text ` | ` Passive Perception 16 ` |
| ` languages_text ` | ` Giant ` |
| ` cr_text ` | ` 9 (XP 5,000; PB +4) ` |
| ` hit_points_text ` | ` 162 (13d12 + 78) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 25 ` | ` 7 ` |
| ` Dexterity ` | ` 9 ` | ` 3 ` |
| ` Constitution ` | ` 23 ` | ` 10 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 13 ` | ` 5 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 18 ` |  |
| ` max_hp ` | ` 162 ` |  |
| ` initiative ` | ` 3 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/fire_giant ` |
| ` implementation ` | ` dnd2024:creature_fire_giant ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["fire"] ` |

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
  - flame_sword
  - hammer_throw
```

##### Flame sword

<!-- splat:values path=splat_rule/parameters/features/flame_sword -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/flame_sword/attack -->

```yaml
mode: melee
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 11
reach_ft: 10
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/flame_sword/hit/damage
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
| ` row_1 ` | ` 21 ` | ` 4 ` | ` 6 ` | ` 7 ` | ` Slashing ` |
| ` row_2 ` | ` 10 ` | ` 3 ` | ` 6 ` | ` 0 ` | ` Fire ` |

##### Hammer throw

<!-- splat:values path=splat_rule/parameters/features/hammer_throw -->

| Field | Value |
| --- | --- |
| ` economy ` | ` action ` |
| ` operation ` | ` attack ` |

###### Attack

<!-- splat:yaml path=splat_rule/parameters/features/hammer_throw/attack -->

```yaml
mode: ranged
targets:
  count: 1
  kind: creature_or_object
supplied_attack_total: 11
range_ft: 60
long_range_ft: 240
```

###### Hit

###### Damage

<!-- splat:table
path: splat_rule/parameters/features/hammer_throw/hit/damage
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
| ` row_1 ` | ` 23 ` | ` 3 ` | ` 10 ` | ` 7 ` | ` Bludgeoning ` |
| ` row_2 ` | ` 4 ` | ` 1 ` | ` 8 ` | ` 0 ` | ` Fire ` |

###### Push

<!-- splat:values path=splat_rule/parameters/features/hammer_throw/hit/push -->

| Field | Value |
| --- | --- |
| ` maximum_ft ` | ` 15 ` |
| ` direction ` | ` straight_away_from_self ` |

###### Next attack modifier

<!-- splat:values path=splat_rule/parameters/features/hammer_throw/hit/next_attack_modifier -->

| Field | Value |
| --- | --- |
| ` disadvantage ` | ` true ` |
| ` expires ` | ` target.next_turn_end ` |
| ` uses ` | ` 1 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 11 ` |
| ` perception ` | ` 6 ` |

#### Condition immunities

<!-- splat:yaml path=splat_rule/parameters/condition_immunities -->

```yaml
[]
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 16 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- giant
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
