---
splat_schema: 1
splat_id: b630923a-c365-5ee2-9aee-68e460bdffaf
splat_kind: creature
splat_key: dnd2024:creatures/elephant
splat_name: Elephant
splat_origin:
  key: dnd2024:reference/creature/elephant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 348
  url: https://www.dndbeyond.com/srd
---

# Elephant

## Source text

<!-- SRD page 348 -->

Huge Beast, Unaligned\
AC 12  Initiative −1 (9)\
HP 76 (8d12 + 24)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 9 −1 −1 Con 17 +3 +3\
Int 3 −4 −4 Wis 11 +0 +0 Cha 6 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 4 (XP 1,100; PB +2)

### Actions

Multiattack. The elephant makes two Gore attacks.

Gore. Melee Attack Roll: +8, reach 5 ft. Hit: 15 (2d8 + 6) Piercing damage. If the target is a Huge or smaller creature and the elephant moved 20+ feet straight toward it immediately before the hit, the target has the Prone condition.

### Bonus Actions

Trample. Dexterity Saving Throw: DC 16, one creature within 5 feet that has the Prone condition. Failure: 17 (2d10 + 6) Bludgeoning damage. Success: Half damage.

Original layout: [[../../srd-5.2.1/pages/348|page 348]].

## Creature

<!-- splat:values path=splat_creature -->

| Field | Value |
| --- | --- |
| ` size ` | ` huge ` |
| ` type ` | ` beast ` |
| ` alignment ` | ` Unaligned ` |
| ` movement_text ` | ` 40 ft. ` |
| ` senses_text ` | ` Passive Perception 10 ` |
| ` languages_text ` | ` None ` |
| ` cr_text ` | ` 4 (XP 1,100; PB +2) ` |
| ` hit_points_text ` | ` 76 (8d12 + 24) ` |

## Abilities

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| ` Strength ` | ` 22 ` | ` 6 ` |
| ` Dexterity ` | ` 9 ` | ` -1 ` |
| ` Constitution ` | ` 17 ` | ` 3 ` |
| ` Intelligence ` | ` 3 ` | ` -4 ` |
| ` Wisdom ` | ` 11 ` | ` 0 ` |
| ` Charisma ` | ` 6 ` | ` -2 ` |

## Stats

<!-- splat:map path=splat_stats -->

| Key | value | unit |
| --- | --- | --- |
| ` armor_class ` | ` 12 ` |  |
| ` max_hp ` | ` 76 ` |  |
| ` initiative ` | ` -1 ` |  |
| ` speed ` | ` 40 ` | ` ft ` |

## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/elephant ` |
| ` implementation ` | ` dnd2024:creature_elephant ` |

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
  supplied_attack_total: 8
  reach_ft: 5
hit:
  damage:
  - average: 15
    roll:
      count: 2
      sides: 8
      modifier: 6
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
dc: 16
targets:
  kind: creature
  count: 1
  range_ft: 5
  required_conditions:
  - prone
failure:
  damage:
  - average: 17
    roll:
      count: 2
      sides: 10
      modifier: 6
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
