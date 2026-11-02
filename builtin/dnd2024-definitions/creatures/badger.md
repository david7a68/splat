---
splat_schema: 1
splat_id: 883a37d4-0528-5c57-a94c-cf5f8e56eb9d
splat_kind: creature
splat_key: dnd2024:creatures/badger
splat_name: Badger
splat_origin:
  key: dnd2024:reference/creature/badger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
---

# Badger

## Source text

<!-- SRD page 345 -->

Tiny Beast, Unaligned\
AC 11  Initiative +0 (10)\
HP 5 (1d4 + 3)\
Speed 20 ft., Burrow 5 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 11 +0 +0 Con 16 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +3\
Resistances Poison\
Senses Darkvision 30 ft.; Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/345|page 345]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 20 ft., Burrow 5 ft. ` |
| ` skills_text ` | ` Perception +3 ` |
| ` resistances_text ` | ` Poison ` |
| ` senses_text ` | ` Darkvision 30 ft.; Passive Perception 13 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 5 (1d4 + 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 10 ` | ` 0 ` |
| ` Dexterity ` | ` 11 ` | ` 0 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 5 ` |  |
| ` initiative ` | ` 0 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` burrow_speed ` | ` 5 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/badger ` |
| ` implementation ` | ` dnd2024:creature_badger ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["poison"] ` |

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
  supplied_attack_total: 2
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
| ` perception ` | ` 3 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 30 ` |
| ` passive_perception ` | ` 13 ` |

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
