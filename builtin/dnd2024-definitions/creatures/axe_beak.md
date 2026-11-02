---
splat_schema: 1
splat_id: 3fbef6af-bb6e-51d8-8bf2-9bd632abf451
splat_kind: creature
splat_key: dnd2024:creatures/axe_beak
splat_name: Axe Beak
splat_origin:
  key: dnd2024:reference/creature/axe_beak
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
---

# Axe Beak

## Source text

<!-- SRD page 260 -->

Large Monstrosity, Unaligned\
AC 11 Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 12 +1 +1 Con 12 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Senses Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/260|page 260]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 50 ft. ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 19 (3d10 + 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 12 ` | ` 1 ` |
| ` Constitution ` | ` 12 ` | ` 1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 5 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 19 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/axe_beak ` |
| ` implementation ` | ` dnd2024:creature_axe_beak ` |

### Parameters

#### Features

##### Beak

<!-- splat:yaml path=splat_rule/parameters/features/beak -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 6
    roll:
      count: 1
      sides: 8
      modifier: 2
    type: slashing
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

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
