---
splat_schema: 1
splat_id: f3b29089-44cc-5d81-b254-7e8304b98d9e
splat_kind: creature
splat_key: dnd2024:creatures/griffon
splat_name: Griffon
splat_origin:
  key: dnd2024:reference/creature/griffon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 295
  url: https://www.dndbeyond.com/srd
---

# Griffon

## Source text

<!-- SRD page 295 -->

Large Monstrosity, Unaligned\
AC 12 Initiative +2 (12)\
HP 59 (7d10 + 21)\
Speed 30 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 2 −4 −4 Wis 13 +1 +1 Cha 8 −1 −1\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 2 (XP 450; PB +2)

### Actions

Multiattack. The griffon makes two Rend attacks.

Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8 + 4) Piercing damage. If the target is a Medium or smaller creature, it has the Grappled condition (escape DC 14) from both of the griffon’s front claws.

Original layout: [[../../srd-5.2.1/pages/295|page 295]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` monstrosity ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Fly 80 ft. ` |
| ` skills_text ` | ` Perception +5 ` |
| ` senses_text ` | ` Darkvision 60 ft.; Passive Perception 15 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 2 (XP 450; PB +2) ` |
| ` hit_points_text ` | ` 59 (7d10 + 21) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 16 ` | ` 3 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 8 ` | ` -1 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 59 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/griffon ` |
| ` implementation ` | ` dnd2024:creature_griffon ` |

### Parameters

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

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

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
    type: piercing
  grapple:
    condition: grappled
    source: self
    escape_dc: 14
    maximum_size: medium
    appendage_pool:
      count: 2
      type: claws
      cost: 2
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
| ` darkvision ` | ` 60 ` |
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
