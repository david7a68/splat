---
splat_schema: 1
splat_id: 73df5740-aae0-5764-9992-52fca234c83a
splat_kind: creature
splat_key: dnd2024:creatures/hippopotamus
splat_name: Hippopotamus
splat_origin:
  key: dnd2024:reference/creature/hippopotamus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 355, 356
  url: https://www.dndbeyond.com/srd
---

# Hippopotamus

## Source text

<!-- SRD page 355 -->

Large Beast, Unaligned\
AC 14  Initiative −2 (8)\
HP 82 (11d10 + 22)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +7 Dex 7 −2 −2 Con 15 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 4 −3 −3\
Skills Perception +3\
Senses Passive Perception 13\
Languages None\
CR 4 (XP 1,100; PB +2)

### Traits

Hold Breath. The hippopotamus can hold its breath for 10 minutes.

<!-- SRD page 356 -->

### Actions

Multiattack. The hippopotamus makes two Bite attacks.

Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 16 (2d10 + 5) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/355|page 355]], [[../../srd-5.2.1/pages/356|page 356]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Swim 30 ft. ` |
| ` skills_text ` | ` Perception +3 ` |
| ` senses_text ` | ` Passive Perception 13 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 82 (11d10 + 22) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 21 ` | ` 7 ` |
| ` Dexterity ` | ` 7 ` | ` -2 ` |
| ` Constitution ` | ` 15 ` | ` 2 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 4 ` | ` -3 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 82 ` |  |
| ` initiative ` | ` -2 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` swim_speed ` | ` 30 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/hippopotamus ` |
| ` implementation ` | ` dnd2024:creature_hippopotamus ` |

### Parameters

#### Features

##### Hold breath

<!-- splat:values path=splat_rule/parameters/features/hold_breath -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` hold_breath_seconds ` | ` 600 ` |

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: bite
  count: 2
```

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
  supplied_attack_total: 7
  reach_ft: 5
hit:
  damage:
  - average: 16
    roll:
      count: 2
      sides: 10
      modifier: 5
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
