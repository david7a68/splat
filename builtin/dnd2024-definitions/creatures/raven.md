---
splat_schema: 1
splat_id: f7c24eb6-393e-592b-9df3-307b98d00712
splat_kind: creature
splat_key: dnd2024:creatures/raven
splat_name: Raven
splat_origin:
  key: dnd2024:reference/creature/raven
splat_sources:
- title: SRD 5.2.1
  locator: pp. 359
  url: https://www.dndbeyond.com/srd
---

# Raven

## Source text

<!-- SRD page 359 -->

Tiny Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 2 (1d4)\
Speed 10 ft., Fly 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 14 +2 +2 Con 10 +0 +0\
Int  5 −3 −3 Wis 13 +1 +1 Cha 6 −2 −2\
Skills Perception +3\
Senses Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)

### Traits

Mimicry. The raven can mimic simple sounds it has heard, such as a whisper or chitter. A hearer can discern the sounds are imitations with a successful DC 10 Wisdom (Insight) check.

### Actions

Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/359|page 359]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 10 ft., Fly 50 ft. ` |
| ` skills_text ` | ` Perception +3 ` |
| ` senses_text ` | ` Passive Perception 13 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 0 (XP 10; PB +2) ` |
| ` hit_points_text ` | ` 2 (1d4) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 2 ` | ` -4 ` |
| ` Dexterity ` | ` 14 ` | ` 2 ` |
| ` Constitution ` | ` 10 ` | ` 0 ` |
| ` Intelligence ` | ` 5 ` | ` -3 ` |
| ` Wisdom ` | ` 13 ` | ` 1 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 2 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 10 ` | ` ft ` |
| ` fly_speed ` | ` 50 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/raven ` |
| ` implementation ` | ` dnd2024:creature_raven ` |

### Parameters

#### Features

##### Mimicry

<!-- splat:yaml path=splat_rule/parameters/features/mimicry -->

```yaml
economy: trait
operation: passive_rule
sounds:
  complexity: simple
  previously_heard: true
detect_imitation:
  check: wisdom_insight
  dc: 10
```

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
