---
splat_schema: 1
splat_id: 6a1bb24a-f0ac-53c9-bcc2-e8639b61111b
splat_kind: creature
splat_key: dnd2024:creatures/archelon
splat_name: Archelon
splat_origin:
  key: dnd2024:reference/creature/archelon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344, 345
  url: https://www.dndbeyond.com/srd
---

# Archelon

## Source text

<!-- SRD page 344 -->

Huge Beast (Dinosaur), Unaligned\
AC 17  Initiative +3 (13)\
HP 90 (12d12 + 12)\
Speed 20 ft., Swim 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 16 +3 +3 Con 13 +1 +1\
Int 4 −3 −3 Wis 14 +2 +2 Cha 6 −2 −2\
Skills Stealth +5\
Senses Passive Perception 12\
Languages None\
CR 4 (XP 1,100; PB +2)

<!-- SRD page 345 -->

### Traits

Amphibious. The archelon can breathe air and water.

### Actions

Multiattack. The archelon makes two Bite attacks.

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 14 (3d6 + 4) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/344|page 344]], [[../../srd-5.2.1/pages/345|page 345]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` tags_text ` | ` (Dinosaur) ` |
| ` movement_text ` | ` 20 ft., Swim 80 ft. ` |
| ` skills_text ` | ` Stealth +5 ` |
| ` senses_text ` | ` Passive Perception 12 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 90 (12d12 + 12) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 18 ` | ` 4 ` |
| ` Dexterity ` | ` 16 ` | ` 3 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 4 ` | ` -3 ` |
| ` Wisdom ` | ` 14 ` | ` 2 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 17 ` |  |
| ` max_hp ` | ` 90 ` |  |
| ` initiative ` | ` 3 ` |  |
| ` speed ` | ` 20 ` | ` ft ` |
| ` swim_speed ` | ` 80 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/archelon ` |
| ` implementation ` | ` dnd2024:creature_archelon ` |

### Parameters

#### Features

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` breathing ` | ` ["air", "water"] ` |

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
  supplied_attack_total: 6
  reach_ft: 5
hit:
  damage:
  - average: 14
    roll:
      count: 3
      sides: 6
      modifier: 4
    type: piercing
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 5 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

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
