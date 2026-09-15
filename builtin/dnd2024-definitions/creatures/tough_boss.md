---
splat_schema: 1
splat_id: e046c72b-0474-5e7d-abb7-02703b4ac38c
splat_kind: creature
splat_key: dnd2024:creatures/tough_boss
splat_name: Tough Boss
splat_origin:
  key: dnd2024:reference/creature/tough_boss
splat_sources:
- title: SRD 5.2.1
  locator: pp. 332
  url: https://www.dndbeyond.com/srd
---

# Tough Boss

## Source text

<!-- SRD page 332 -->

Medium or Small Humanoid, Neutral\
AC 16  Initiative +2 (12)\
HP 82 (11d8 + 33)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +5 Dex 14 +2 +2 Con16 +3 +5\
Int 11 +0 +0 Wis 10 +0 +0 Cha 11 +0 +2\
Gear Chain Mail, Heavy Crossbow, Warhammer\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 4 (XP 1,100; PB +2)\

### Traits

Pack Tactics. The tough has Advantage on an attack\
roll against a creature if at least one of the tough’s allies\
is within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Multiattack. The tough makes two attacks, using War-\
hammer or Heavy Crossbow in any combination.\
Warhammer. Melee Attack Roll: +5, reach 5 ft. Hit: 12\
(2d8 + 3) Bludgeoning damage. If the target is a Large\
or smaller creature, the tough pushes the target up to 10\
feet straight away from itself.\
Heavy Crossbow. Ranged Attack Roll: +4, range\
100/400 ft. Hit: 13 (2d10 + 2) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/332|page 332]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Chain Mail, Heavy Crossbow, Warhammer ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 82 (11d8 + 33) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 17 ` | ` 5 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 16 ` | ` 5 ` |
| ` Intelligence ` | ` 11 ` | ` 0 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 11 ` | ` 2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 16 ` |  |
| ` max_hp ` | ` 82 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/tough_boss ` |
| ` implementation ` | ` dnd2024:creature_tough_boss ` |

### Parameters

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: trait
operation: passive_rule
attack_advantage:
  target_kind: creature
  ally_within_target_ft: 5
  ally_excludes_conditions:
  - incapacitated
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_choice
  count: 2
  options:
  - warhammer
  - heavy_crossbow
```

##### Warhammer

<!-- splat:yaml path=splat_rule/parameters/features/warhammer -->

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
  - average: 12
    roll:
      count: 2
      sides: 8
      modifier: 3
    type: bludgeoning
  push:
    maximum_ft: 10
    direction: straight_away_from_self
    maximum_size: large
```

##### Heavy crossbow

<!-- splat:yaml path=splat_rule/parameters/features/heavy_crossbow -->

```yaml
economy: action
operation: attack
attack:
  mode: ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  range_ft: 100
  long_range_ft: 400
hit:
  damage:
  - average: 13
    roll:
      count: 2
      sides: 10
      modifier: 2
    type: piercing
```

#### Gear

<!-- splat:rows path=splat_rule/parameters/gear -->

| Row | item | quantity |
| --- | --- | --- |
| ` 0 ` | ` chain_mail ` | ` 1 ` |
| ` 1 ` | ` heavy_crossbow ` | ` 1 ` |
| ` 2 ` | ` warhammer ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks:
- common
understands: []
additional_languages:
  count: 1
  chooser: creator
  can_speak: true
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
