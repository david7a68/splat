---
splat_schema: 1
splat_id: 5333b55b-f367-5498-87ce-4043b633e717
splat_kind: creature
splat_key: dnd2024:creatures/mammoth
splat_name: Mammoth
splat_origin:
  key: dnd2024:reference/creature/mammoth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 357
  url: https://www.dndbeyond.com/srd
---

# Mammoth

## Source text

<!-- SRD page 357 -->

Huge Beast, Unaligned\
AC 13  Initiative +2 (12)\
HP 126 (11d12 + 55)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  24 +7 +10 Dex 9 −1 −1 Con 21 +5 +8\
Int 3 −4 −4 Wis 11 +0 +0 Cha 6 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 6 (XP 2,300; PB +3)

### Actions

Multiattack. The mammoth makes two Gore attacks.

Gore. Melee Attack Roll: +10, reach 10 ft. Hit: 18 (2d10 + 7) Piercing damage. If the target is a Huge or smaller creature and the mammoth moved 20+ feet straight toward it immediately before the hit, the target has the Prone condition.

### Bonus Actions

Trample. Dexterity Saving Throw: DC 18, one creature within 5 feet that has the Prone condition. Failure: 29 (4d10 + 7) Bludgeoning damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/357|page 357]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 50 ft. ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 6 (XP 2,300; PB +3) ` |
| ` hit_points_text ` | ` 126 (11d12 + 55) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 24 ` | ` 10 ` |
| ` Dexterity ` | ` 9 ` | ` -1 ` |
| ` Constitution ` | ` 21 ` | ` 8 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 13 ` |  |
| ` max_hp ` | ` 126 ` |  |
| ` initiative ` | ` 2 ` |  |
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
| ` mechanic ` | ` dnd2024:creatures/mammoth ` |
| ` implementation ` | ` dnd2024:creature_mammoth ` |

### Parameters

#### Features

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
operation: multiattack
sequence:
- operation: attack_sequence
  feature: gore
  count: 2
```

##### Gore

<!-- splat:yaml path=splat_rule/parameters/features/gore -->

```yaml
economy: action
operation: attack
attack:
  mode: melee
  targets:
    count: 1
    kind: creature_or_object
  supplied_attack_total: 10
  reach_ft: 10
hit:
  damage:
  - average: 18
    roll:
      count: 2
      sides: 10
      modifier: 7
    type: piercing
  prone:
    condition: prone
    maximum_size: huge
    requires:
      immediately_preceding_straight_movement_toward_target_ft: 20
```

##### Trample

<!-- splat:yaml path=splat_rule/parameters/features/trample -->

```yaml
economy: bonus_action
operation: saving_throw
ability: dexterity
dc: 18
targets:
  kind: creature
  count: 1
  range_ft: 5
  required_conditions:
  - prone
failure:
  damage:
  - average: 29
    roll:
      count: 4
      sides: 10
      modifier: 7
    type: bludgeoning
success:
  damage_multiplier: 0.5
  other_effects: false
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
