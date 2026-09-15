---
splat_schema: 1
splat_id: 5a3c78f9-2858-53fa-970d-95bf9c8d5bcb
splat_kind: creature
splat_key: dnd2024:creatures/pirate
splat_name: Pirate
splat_origin:
  key: dnd2024:reference/creature/pirate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 314
  url: https://www.dndbeyond.com/srd
---

# Pirate

## Source text

<!-- SRD page 314 -->

Medium or Small Humanoid, Neutral\
AC 14  Initiative +5 (15)\
HP 33 (6d8 + 6)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 16 +3 +5 Con 12 +1 +1\
Int 8 −1 −1 Wis 12 +1 +1 Cha 14 +2 +4\
Gear Daggers (6), Leather Armor\
Senses Passive Perception 11\
Languages Common plus one other language\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The pirate makes two Dagger attacks.\
It can replace one attack with a use of Enthrall-\
ing Panache.\
Dagger. Melee or Ranged Attack Roll: +5, reach 5 ft. or\
range 20/60 ft. Hit: 5 (1d4 + 3) Piercing damage.\
Enthralling Panache. Wisdom Saving Throw: DC 12,\
one creature the pirate can see within 30 feet. Failure:\
The target has the Charmed condition until the start of\
the pirate’s next turn.\

Original layout: [[../../srd-5.2.1/pages/314|page 314]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` type ` | ` humanoid ` |
| ` alignment ` | ` Neutral ` |
| ` size_options_text ` | ` Medium or Small ` |
| ` movement_text ` | ` 30 ft. ` |
| ` gear_text ` | ` Daggers (6), Leather Armor ` |
| ` senses_text ` | ` Passive Perception 11 ` |
| ` languages_text ` | ` Common plus one other language ` |
| ` cr_text ` | ` 1 (XP 200; PB +2) ` |
| ` hit_points_text ` | ` 33 (6d8 + 6) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 16 ` | ` 5 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 8 ` | ` -1 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 14 ` | ` 4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 33 ` |  |
| ` initiative ` | ` 5 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/pirate ` |
| ` implementation ` | ` dnd2024:creature_pirate ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: dagger
  count: 2
replacements:
- replace_count: 1
  maximum_uses: 1
  operation:
    operation: use_feature
    feature: enthralling_panache
```

##### Dagger

<!-- splat:yaml path=splat_rule/parameters/features/dagger -->

```yaml
economy: action
operation: attack
attack:
  mode: melee_or_ranged
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 5
  reach_ft: 5
  range_ft: 20
  long_range_ft: 60
hit:
  damage:
  - average: 5
    roll:
      count: 1
      sides: 4
      modifier: 3
    type: piercing
```

##### Enthralling panache

<!-- splat:yaml path=splat_rule/parameters/features/enthralling_panache -->

```yaml
economy: action
operation: saving_throw
ability: wisdom
dc: 12
targets:
  kind: creature
  count: 1
  range_ft: 30
  visible: true
failure:
  conditions:
  - condition: charmed
    until: self.next_turn_start
success:
  effects: []
```

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Item:
    path: item
    type: symbol
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Daggers ` | ` 6 ` |
| ` row_2 ` | ` Leather armor ` | ` 1 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

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
