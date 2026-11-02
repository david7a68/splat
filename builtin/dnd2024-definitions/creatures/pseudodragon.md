---
splat_schema: 1
splat_id: 19456839-4a0c-5fe3-9215-4852c1a02028
splat_kind: creature
splat_key: dnd2024:creatures/pseudodragon
splat_name: Pseudodragon
splat_origin:
  key: dnd2024:reference/creature/pseudodragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 316
  url: https://www.dndbeyond.com/srd
---

# Pseudodragon

## Source text

<!-- SRD page 316 -->

Tiny Dragon, Neutral Good\
AC 14  Initiative +2 (12)\
HP 10 (3d4 + 3)\
Speed 15 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 15 +2 +2 Con 13 +1 +1\
Int 10 +0 +0 Wis 12 +1 +1 Cha 10 +0 +0\
Skills Perception +5, Stealth +4\
Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 15\
Languages Understands Common and Draconic but can’t speak\
CR 1/4 (XP 50; PB +2)

### Traits

Magic Resistance. The pseudodragon has Advantage on saving throws against spells and other magical effects.

### Actions

Multiattack. The pseudodragon makes two Bite attacks.

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage.

Sting. Constitution Saving Throw: DC 12, one creature the pseudodragon can see within 5 feet. Failure: 5 (2d4) Poison damage, and the target has the Poisoned condition for 1 hour. Failure by 5 or More: While Poisoned, the target also has the Unconscious condition, which ends early if the target takes damage or a creature within 5 feet of it takes an action to wake it.

Original layout: [[../../srd-5.2.1/pages/316|page 316]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` tiny ` |
| ` type ` | ` dragon ` |
| ` alignment ` | ` Neutral Good ` |
| ` movement_text ` | ` 15 ft., Fly 60 ft. ` |
| ` skills_text ` | ` Perception +5, Stealth +4 ` |
| ` senses_text ` | ` Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 15 ` |
| ` languages_text ` | ` Understands Common and Draconic but can’t speak ` |
| ` cr_text ` | ` 1/4 (XP 50; PB +2) ` |
| ` hit_points_text ` | ` 10 (3d4 + 3) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 6 ` | ` -2 ` |
| ` Dexterity ` | ` 15 ` | ` 2 ` |
| ` Constitution ` | ` 13 ` | ` 1 ` |
| ` Intelligence ` | ` 10 ` | ` 0 ` |
| ` Wisdom ` | ` 12 ` | ` 1 ` |
| ` Charisma ` | ` 10 ` | ` 0 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 14 ` |  |
| ` max_hp ` | ` 10 ` |  |
| ` initiative ` | ` 2 ` |  |
| ` speed ` | ` 15 ` | ` ft ` |
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
| ` mechanic ` | ` dnd2024:creatures/pseudodragon ` |
| ` implementation ` | ` dnd2024:creature_pseudodragon ` |

### Parameters

#### Features

##### Magic resistance

<!-- splat:values path=splat_rule/parameters/features/magic_resistance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` trait ` |
| ` operation ` | ` passive_rule ` |
| ` saving_throw_advantage_against ` | ` ["spells", "magical_effects"] ` |

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
  supplied_attack_total: 4
  reach_ft: 5
hit:
  damage:
  - average: 4
    roll:
      count: 1
      sides: 4
      modifier: 2
    type: piercing
```

##### Sting

<!-- splat:yaml path=splat_rule/parameters/features/sting -->

```yaml
economy: action
operation: saving_throw
ability: constitution
dc: 12
targets:
  kind: creature
  count: 1
  range_ft: 5
  visible: true
failure:
  damage:
  - roll:
      count: 2
      sides: 4
      modifier: 0
    type: poison
  conditions:
  - condition: poisoned
    duration_seconds: 3600
failure_by_5_or_more:
  additional_while_poisoned:
    condition: unconscious
    ends_on:
    - target.damage_taken
    - event: creature.action
      operation: wake_target
      within_ft: 5
success:
  effects: []
```

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 4 ` |

#### Senses

<!-- splat:values path=splat_rule/parameters/senses -->

| Field | Value |
| --- | --- |
| ` blindsight ` | ` 10 ` |
| ` darkvision ` | ` 60 ` |
| ` passive_perception ` | ` 15 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speaks: []
understands:
- common
- draconic
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
