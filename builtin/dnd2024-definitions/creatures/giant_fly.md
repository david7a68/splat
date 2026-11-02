---
splat_schema: 1
splat_id: 1cb10e40-4724-5948-8930-5e7b5876854a
splat_kind: creature
splat_key: dnd2024:creatures/giant_fly
splat_name: Giant Fly
splat_origin:
  key: dnd2024:reference/creature/giant_fly
splat_sources:
- title: SRD 5.2.1
  locator: pp. 222
  url: https://www.dndbeyond.com/srd
---

# Giant Fly

## Source text

<!-- SRD page 222 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 13 +1 +1 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
Senses Darkvision 60 ft., Passive Perception 10\
Languages None\
CR 0 (XP 0; PB +2)

Original layout: [[../../srd-5.2.1/pages/222|page 222]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` large ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 30 ft., Fly 60 ft. ` |
| ` senses_text ` | ` Darkvision 60 ft., Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 0; PB +2) ` |
| ` hit_points_text ` | ` 19 (3d10 + 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 14 ` | ` 2 ` |
| ` Dexterity ` | ` 13 ` | ` 1 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 2 ` | ` -4 ` |
| ` Wisdom ` | ` 10 ` | ` 0 ` |
| ` Charisma ` | ` 3 ` | ` -4 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 11 ` |  |
| ` max_hp ` | ` 19 ` |  |
| ` initiative ` | ` 1 ` |  |
| ` speed ` | ` 30 ` | ` ft ` |
| ` fly_speed ` | ` 60 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_fly ` |
| ` implementation ` | ` dnd2024:creature_giant_fly ` |

### Parameters

#### Features

<!-- splat:yaml path=splat_rule/parameters/features -->

```yaml
{}
```

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` darkvision ` | ` 60 ` |
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
